# `qsv profile` — handoff #3 (YAML-driven projection engine, **COMPLETE**)

The YAML-driven projection engine described in the original plan
**fully landed across 8 staged commits**. The shipped binary always
goes through the YAML engine; the legacy `dcat.rs` / `catalog.rs` /
`ckan_to_dcat.rs` / `curie.rs` modules are deleted.

Previous handoffs:
- `profile-handoff.md` (PR #3898 era, Python-backed) — mostly stale.
- `profile2-handoff.md` (PR #3901 era, hardcoded Rust DCAT-US v3) —
  mostly stale; consult only for context on the pre-YAML engine.

---

## 1. Final state

Branch: `dcat-us-v3-optimization2` (8 new commits since `013be565f`)

| # | Commit | Stage | LOC Δ |
|---|---|---|---|
| 1 | `b7ecfc41d` | Engine scaffold + 11 helpers + `--profile` flag | +1525 |
| 2 | `a01da79aa` | Capture goldens from legacy engine | +1379 |
| 3 | `6732c644d` | Ship dcat-us-v3.yaml (partial) | +546 |
| 4 | `86ecf8c26` | handoff doc | +394 |
| 5 | `551cd443b` | Wire YAML engine into orchestrator + parity tests | +917 / −598 |
| 6 | `965f13a27` | Delete legacy hardcoded engine + refactor validator | +75 / −2441 |
| 7 | `3a025e848` | Ship dcat-ap-v3 profile + 4 smoke tests | +544 |
| 8 | `7936bb6d9` | Ship croissant 1.0 profile + 5 smoke tests | +463 |

**Net Rust LOC delta:** roughly −300 LOC compared to the pre-YAML
engine state, AND the entire DCAT-US v3 / DCAT-AP v3 / Croissant
projection knowledge now lives in three editable YAML files at
`resources/profiles/*.yaml`.

**All tests pass:**
- 127 unit tests (`cargo test cmd::profile::`)
- 40 integration tests (`cargo test --test tests test_profile::`)
- All 4 binaries build clean: `qsv` (-F all_features), `qsvmcp`
  (-F qsvmcp), `qsvlite` (-F lite), `qsvdp` (-F datapusher_plus)
- `python3 scripts/docs-drift-check.py` → "no drift detected"

---

## 2. Final file map

### Profile engine (new)

| Path | Role |
|---|---|
| `src/cmd/profile/profile_spec.rs` | `ProfileSpec` serde types, `EMBEDDED` table with `include_str!` for the 3 bundled YAMLs, `load()` (embedded-first, file-path fallback), `ProfileSpec::translate_ckan_ptr` for the force-override path. 7 unit tests. |
| `src/cmd/profile/projection.rs` | Generic `project(profile, ctx, mode) -> CliResult<(Value, Vec<ProjectionWarning>)>` engine. `ProjectionMode { Dataset, Catalog }`, `Severity { Required, Recommended, Optional, Info }`, `wrap_as_catalog`, `emit_field` (with `emit_when` + `default` + `required_level`), `emit_recordset` (`for_each_column`), `register_profile_helpers` (closures for `lookup` + `field_mapping`), `dry_compile` validator. Uses `UndefinedBehavior::Chainable` so missing intermediate keys gracefully fall through to `\| default(...)`. 9 unit tests. |
| `src/cmd/profile/discovery_merge.rs` | Profile-aware `merge(profile, inferred, discovered, forced_dcat_paths) -> Value`. Strategies: `fill-if-absent`, `overlay-array`, `never`. Honors `profile.discovery_merge.never_overwrite` + `/dcat/<key>` forced-path semantics. 6 unit tests. |

### Bundled profile YAMLs (new)

| Path | Content |
|---|---|
| `resources/profiles/dcat-us-v3.yaml` | Full DCAT-US v3 projection — 4 vocabularies, 53 field_mappings, 23 dataset fields + 22 distribution fields + catalog block, JSON Schema validation against the vendored GSA bundle. Byte-equivalent output to the legacy `dcat.rs` engine — verified by parity tests. |
| `resources/profiles/dcat-ap-v3.yaml` | DCAT-AP v3 (https://semiceu.github.io/DCAT-AP/releases/3.0.0/) — subset of DCAT-US v3 with dcat-us:* extensions dropped, EU theme vocabulary added, validation disabled (DCAT-AP ships SHACL upstream). |
| `resources/profiles/croissant.yaml` | Croissant 1.0 (https://github.com/mlcommons/croissant) — schema.org-rooted JSON-LD with inline `@context` object, `sc:Dataset`/`sc:FileObject` types, per-column `cr:RecordSet`/`cr:Field` expansion, BLAKE3 hash via `cr:fileFingerprint`, validation + discovery disabled. |
| `resources/profiles/README.md` | Authoring guide, per-field directive doc, versioning contract. |

### Helpers added to `formula_helpers.rs`

| Name | Kind | Purpose |
|---|---|---|
| `lookup` | global (profile-aware closure in `projection.rs`) | `lookup("vocab_table", key)` with case-insensitive + RFC 5646 subtag stripping. |
| `field_mapping` | global (profile-aware closure) | Reverse-lookup CKAN → target pointer for templates that walk dynamically. |
| `only_if_absolute_iri` | filter | Pass IRIs through, return UNDEFINED for non-IRIs. |
| `basename` / `file_stem` | filter | Path → final segment / stem. |
| `sanitize_iso_8601_interval` | filter | Reject `R/P1Y`-style intervals. |
| `format_mailto` | filter | Trim + prepend `mailto:`. |
| `sha256_of` / `blake3_of` | global | Streaming hash → lowercase hex. |
| `file_size_of` | global | `fs::metadata.len()` as string. |
| `compress_format` / `package_format` | global | Extension → IANA media type. |
| `build_csvw_schema` | global | Stats map → `{columns: [{name, titles, datatype, qsv:cardinality, qsv:nullcount, qsv:min, qsv:max}, ...]}` (DCAT). |
| `build_croissant_fields` | global | Stats map → flat array of `cr:Field` objects with schema.org dataTypes (Croissant). |
| `bbox_from_dpps` | global | LAT/LON column → `dct:Location` POLYGON-WKT array. |
| `temporal_from_dpps` | global | Date column → `dct:PeriodOfTime` array (one per inferred date column). |
| `csvw_datatype_legacy` | private | qsv stats type → CSVW datatype (Float → double, Integer → integer, etc.) — used by `build_csvw_schema`. |

### Deleted modules (Stage 4b)

- `src/cmd/profile/dcat.rs` (1738 LOC)
- `src/cmd/profile/catalog.rs` (154 LOC)
- `src/cmd/profile/ckan_to_dcat.rs` (271 LOC)
- `src/cmd/profile/curie.rs` (225 LOC)

Total deleted: 2388 LOC. The replacements live in
`resources/profiles/*.yaml` (config) plus a small number of helper
functions in `formula_helpers.rs` + inline CURIE strip in
`dcat_validate.rs`.

### Refactored

| Path | Change |
|---|---|
| `src/cmd/profile.rs::run` | Loads profile at entry, builds `projection_ctx` with `pkg`/`res`/`stats`/`dpp`/`source_label`/`local_path`, calls `projection::project()` instead of `dcat::build()`, calls `discovery_merge::merge()` instead of `merge_discovered()`. `--catalog` mode passes `ProjectionMode::Catalog` upfront. Stash key renamed `__pending_dcat_warnings` → `__pending_projection_warnings`. `run_profile_validation` returns `Vec<ProjectionWarning>` directly. |
| `src/cmd/profile/context.rs` | `ContextArgs` gains `profile: &ProfileSpec`. `load_initial_context` + `collect_forced_paths` take the profile and translate CKAN pointers via `profile.translate_ckan_ptr` instead of importing the deleted `ckan_to_dcat`. |
| `src/cmd/profile/dcat_validate.rs` | New API: `validate(profile: &ProfileSpec, block: &Value) -> Vec<ProjectionWarning>`. Returns empty Vec when `profile.validation.enabled == false`. CURIE strip list pulled from `profile.validation.strippable_curie_prefixes`. `classify_severity` returns `projection::Severity`. The vendored GSA bundle still lives under `resources/dcat-us-v3/` and is loaded at validator-build time. |
| `src/cmd/profile/formula_helpers.rs::register` | Adds the 12 new helpers (11 listed in §4 of the plan + `build_croissant_fields`). |

---

## 3. Verification commands

```bash
# Build matrix — all 4 binaries clean
cargo build --bin qsv     -F all_features
cargo build --bin qsvmcp  -F qsvmcp
cargo build --bin qsvlite -F lite
cargo build --bin qsvdp   -F datapusher_plus

# Tests
cargo test --bin qsv -F profile,feature_capable cmd::profile::     # 127 unit
cargo test --test tests -F profile,feature_capable -- test_profile::  # 40 integration

# Format + clippy (NIGHTLY fmt per CLAUDE.md)
cargo +nightly fmt
cargo clippy --bin qsv -F profile,feature_capable

# Help + MCP regeneration
cargo run -F all_features --bin qsv -- --generate-help-md
cargo run -F all_features --bin qsv -- --update-mcp-skills

# Docs-drift check
python3 scripts/docs-drift-check.py        # no drift detected

# Smoke tests — all three profiles produce distinct, correct output
qsv profile tests/resources/profile/golden/nyc-311-subset.csv \
    --profile dcat-us-v3 \
    --initial-context tests/resources/profile/dcat-init-context.json \
    -o /tmp/dcat-us.json
jq '.dcat["@type"]' /tmp/dcat-us.json     # "dcat:Dataset"

qsv profile tests/resources/profile/golden/nyc-311-subset.csv \
    --profile dcat-ap-v3 \
    -o /tmp/dcat-ap.json
jq '.dcat["dct:conformsTo"][0]["@id"]' /tmp/dcat-ap.json  # "https://semiceu.github.io/DCAT-AP/releases/3.0.0/"

qsv profile tests/resources/profile/golden/nyc-311-subset.csv \
    --profile croissant \
    -o /tmp/croissant.json
jq '.dcat["@type"], .dcat.conformsTo' /tmp/croissant.json
# "sc:Dataset"
# "http://mlcommons.org/croissant/1.0"
```

---

## 4. Key design decisions captured

(Don't relitigate without the user's say-so — these were locked
during the plan-mode session.)

* **`UndefinedBehavior::Chainable` (not Strict)** — lets `pkg.x.y.z |
  default("")` walk gracefully through missing intermediates,
  matching the legacy `dcat.rs` semantics where absent CKAN keys
  silently fell through to fallbacks.
* **`lookup` / `field_mapping` return `Value::UNDEFINED` on miss**
  (NOT `Option<Value>`) so `| default(...)` chains actually
  trigger. Returning `None` from a minijinja closure produces the
  literal string "none".
* **License is a plain string, not `{"@id": ...}`** — GSA's
  `Distribution.json` declares license as
  `anyOf: [null, string]`. The legacy `license_value` produced a
  string; the YAML matches via `{{ lookup("license_iri", raw) |
  default(raw) }}`.
* **Forced paths use `/dcat/<key>` form** — the
  `discovery_merge::merge` skip-check escapes the discovered key via
  RFC 6901 then prefixes with `/dcat/`. Matches the legacy
  `merge_discovered` semantics so `dataset_info` overrides flow
  through unchanged.
* **Croissant uses `distribution` (not `dcat:distribution`)** —
  schema.org's `@vocab` resolves bare names. `DistributionBlock`
  gained an optional `path: String` field for the override; DCAT
  profiles still default to `dcat:distribution`.
* **DatasetBlock.context is `Option<Value>` (not `Option<String>`)** —
  Croissant inlines an `@context` map with `@vocab` + prefix
  shorthands. DCAT profiles still pass a single URI string;
  serde handles both shapes.
* **BLAKE3 default for Croissant** — qsv ships blake3 with
  `mmap + rayon` features unconditionally; it's markedly faster
  than SHA-256 on multi-GB ML training data, and Croissant has no
  SPDX-prescribed algorithm.

---

## 5. Reference URLs

- DCAT-US v3 spec: <https://github.com/GSA/dcat-us/>
- DCAT-AP v3 spec: <https://semiceu.github.io/DCAT-AP/releases/3.0.0/>
- Croissant spec: <https://github.com/mlcommons/croissant>
- Vendored GSA schema bundle: `resources/dcat-us-v3/` (unchanged)
- Authoring guide: `resources/profiles/README.md`

---

## 6. Queued follow-ups (not in scope this PR)

The following items were intentionally NOT done this PR — they're
queued for future work:

* **SHACL validation backend** for DCAT-AP v3. Currently
  `validation.enabled: false`; a sibling
  `dcat_shacl_validate.rs` would consume SHACL constraints
  upstream.
* **Croissant `mlcroissant` Python validator integration**.
  Currently `validation.enabled: false`; could spawn the
  out-of-process validator when available.
* **Per-distribution merging in `discovery_merge`** — today the
  `dcat:distribution` array is always inferred. Per-resource
  identity-based merging would let publisher DCAT contribute
  per-Distribution metadata.
* **Profile dry-run validation gate in CI**. The `dry_compile`
  pass runs at runtime; a `cargo test embedded_*` smoke test
  could exercise it at build time too.
* **Catalog inheritance from arbitrary Dataset keys**. Today
  `catalog.inherit_from_dataset` is a static list per profile;
  could become a template-driven projection.
