# `qsv profile` — local working notes

**Status as of 2026-05-27.** This file is local-only — `.gitignore`d
so it doesn't ship with the repo. Earlier handoff docs
(`profile-handoff.md`, `profile2-handoff.md`) were also dropped from
git in prior PRs and only the topic name lives on.

---

## What's landed (in chronological order on master)

### PR #3908 — YAML-driven projection engine (squash `511dd6ae9`)

Built the engine from scratch in 8 stacked commits. Deletes legacy
hardcoded modules (`dcat.rs`, `catalog.rs`, `ckan_to_dcat.rs`,
`curie.rs`) totalling ~2388 LOC. Ships three bundled YAMLs at
`resources/profiles/`:

| Profile | Spec | Validation |
|---|---|---|
| `dcat-us-v3` | DCAT-US v3 (GSA) | JSON Schema (vendored GSA bundle) |
| `dcat-ap-v3` | DCAT-AP v3 (semiceu) | disabled — DCAT-AP ships SHACL upstream |
| `croissant` | Croissant 1.0 (mlcommons) | disabled — no published JSON Schema |

Engine entry points:

| Path | Role |
|---|---|
| `src/cmd/profile/profile_spec.rs` | `ProfileSpec` serde types, `EMBEDDED` table, `load()` (embedded-first + file-path fallback), 7 unit tests |
| `src/cmd/profile/projection.rs` | Generic `project()`, `ProjectionMode { Dataset, Catalog }`, `wrap_as_catalog`, `emit_field`, `emit_recordset`, `register_profile_helpers`, `dry_compile` |
| `src/cmd/profile/discovery_merge.rs` | Profile-aware `merge()` with `fill-if-absent` / `overlay-array` / `never` strategies + RFC 6901 forced-path semantics |
| `src/cmd/profile/dcat_validate.rs` | YAML-aware validator: returns empty `Vec<ProjectionWarning>` when `profile.validation.enabled == false`, CURIE strip list pulled from profile |

### PR #3910 — §6 follow-ups (squash `68cea266e`)

Five commits, three §6 items + one Roborev fix:

| Commit | Item |
|---|---|
| (in squash) | CI gate: parametric `cargo test embedded_*` covers every `EMBEDDED` profile for `load + dry_compile` |
| (in squash) | Per-distribution identity-based discovery merge (new `DistributionMerge` config) |
| (in squash) | Template-driven catalog inheritance via `inner` binding on `catalog.fields[]` + `title_template` ctx |
| (in squash) | Roborev #2499 fix: per-dist merge honors `forced_dcat_paths` + `array_key` actually defaults to `dcat:distribution` |
| (in squash) | Copilot review: misleading clone comment reworded, upfront `to_vec` removed from `merge_distribution_array` |

Net change in PR #3910: ~1003 LOC added, ~23 removed across 6 files,
16 new unit tests.

---

## Still queued

Both involve external runtime deps and deserve their own scoped
branches — neither has a clean "all in Rust" path today.

### 1. Croissant `mlcroissant` Python validator integration

Today `resources/profiles/croissant.yaml` has `validation.enabled:
false`. The canonical validator is the `mlcroissant` Python package
(<https://github.com/mlcommons/croissant/tree/main/python/mlcroissant>).

Plausible design:
- Add `validation.external` block in `ProfileSpec::Validation` —
  `{ command: String, args: Vec<String>, accept_stdin: bool, parse_findings_as: "mlcroissant-text" | "json" }`.
- New module `src/cmd/profile/external_validate.rs` that spawns the
  command with the rendered JSON-LD piped on stdin, captures stdout,
  parses into `ProjectionWarning`s.
- Skip gracefully (Severity::Info warning) when the command is not
  on `PATH` or returns a "module not installed" exit code, so the
  profile still works without Python.
- Wire into the orchestrator parallel to `dcat_validate::validate`.

Open questions:
- Should the spawn be opt-in (env var? CLI flag?) or always-on when
  the binary is on PATH? Always-on with graceful-skip is probably
  least surprising.
- `mlcroissant validate` accepts a file path, not stdin — we'd write
  to a tempfile. That's fine but adds I/O.
- How to surface validator findings as `ProjectionWarning` severity?
  mlcroissant's text output isn't structured; would need either a
  parser or to ask for JSON output via a flag if one exists.

### 2. SHACL validation backend for DCAT-AP v3

`resources/profiles/dcat-ap-v3.yaml` has `validation.enabled: false`
because the spec ships SHACL, not JSON Schema. Two paths:

**(a) Shell out to `pyshacl`** (Python). Same pattern as the proposed
Croissant validator — would benefit from the `external_validate.rs`
work above, so doing Croissant first then reusing the pattern is the
clean order. `pyshacl` is mature and supports the full SHACL Core.

**(b) Native Rust SHACL engine.** Ecosystem is sparse —
`oxigraph`-based SHACL crates are experimental and don't cover the
full constraint set DCAT-AP uses. Not viable today; revisit when a
mature engine appears.

The vendored SHACL shapes for DCAT-AP live upstream at
<https://github.com/SEMICeu/DCAT-AP/blob/master/releases/3.0.0/shacl_shapes.ttl>;
we'd vendor them under `resources/dcat-ap-v3/shacl/` similar to the
GSA JSON Schema bundle.

---

## Decisions locked from prior plan-mode (don't relitigate)

* `UndefinedBehavior::Chainable` for minijinja so `pkg.x.y.z |
  default("")` walks gracefully through missing intermediates.
* `lookup` / `field_mapping` helpers return `Value::UNDEFINED` on
  miss (not `Option<Value>`).
* License is a plain string, not `{"@id": ...}`, per GSA
  `Distribution.json`'s `anyOf: [null, string]`.
* Forced paths use `/dcat/<key>` form with RFC 6901 escaping.
* Croissant uses bare `distribution` (not `dcat:distribution`) per
  schema.org `@vocab` semantics. `DistributionBlock.path` is
  `Option<String>`.
* `DatasetBlock.context` is `Option<Value>` (not `Option<String>`) so
  Croissant can inline an `@context` map.
* BLAKE3 default for Croissant (faster than SHA-256, no SPDX-mandated
  algorithm).

---

## Reference URLs

- DCAT-US v3: <https://github.com/GSA/dcat-us/>
- DCAT-AP v3: <https://semiceu.github.io/DCAT-AP/releases/3.0.0/>
- Croissant: <https://github.com/mlcommons/croissant>
- `mlcroissant` Python validator: <https://github.com/mlcommons/croissant/tree/main/python/mlcroissant>
- pyshacl: <https://github.com/RDFLib/pySHACL>
- Authoring guide: `resources/profiles/README.md`
- Vendored GSA bundle: `resources/dcat-us-v3/`
