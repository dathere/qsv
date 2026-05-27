# `qsv profile` — handoff #3 (YAML-driven projection engine)

This handoff documents the in-progress migration from the hardcoded
`dcat.rs` projection engine to a YAML-driven, multi-profile engine.
The plan lives at:

  `/Users/joelnatividad/.claude/plans/looking-at-src-cmd-profile-rs-luminous-moth.md`

Previous handoff (PR #3901-era, hardcoded DCAT-US v3 engine):
`profile2-handoff.md`. The current handoff supersedes it for the
post-PR-#3901 work.

---

## 1. Where we are right now

Branch: `dcat-us-v3-optimization2`

Three staged commits landed in this session (newest last):

| Commit | Stage | Net | What landed |
|---|---|---|---|
| `b7ecfc41d` | 1: Scaffold | +1525 LOC | `profile_spec.rs`, `projection.rs`, `discovery_merge.rs`, 11 new helpers, `--profile` flag, placeholder YAMLs |
| `a01da79aa` | 2: Goldens  | +1379 LOC | 3 fixture CSVs + 6 normalized goldens captured from legacy engine |
| `6732c644d` | 3: DCAT-US v3 YAML (partial) | +546 LOC | Full `resources/profiles/dcat-us-v3.yaml` (23 dataset fields, 22 distribution fields, 4 vocabs, 53 mappings, catalog block, validation block, discovery_merge block) + dry-compile test |

All commits are non-breaking. The shipped binary still uses the legacy
`dcat.rs` engine; the new modules are scaffolded but unused.
`cargo build` and the existing 192 unit + 29 integration tests still
pass.

**User-locked design decisions** (do not revisit, per the plan):
- Full replacement, not coexistence
- Three bundled profile YAMLs in this PR: `dcat-us-v3`, `dcat-ap-v3`,
  `croissant`
- Embedded + external both supported via `--profile <name|path>`
- Per-profile `validation:` block (schema bundle + entry points +
  CURIE strip list)

**Latest specifications to use** (per the user, mid-session):
- DCAT-US v3:    https://github.com/GSA/dcat-us/
- Croissant:     https://github.com/mlcommons/croissant
- DCAT-AP v3:    https://semiceu.github.io/DCAT-AP/releases/3.0.0/

---

## 2. Current file map

### New (Stages 1-3)

| Path | Role |
|---|---|
| `src/cmd/profile/profile_spec.rs` | `ProfileSpec` serde types, `EMBEDDED` table with `include_str!` for the 3 bundled YAMLs, `load()` (embedded-first, file-path fallback), `load_from_str`. 7 unit tests (incl. `embedded_dcat_us_v3_parses_and_dry_compiles`). |
| `src/cmd/profile/projection.rs` | Generic engine. `ProjectionMode { Dataset, Catalog }`, `ProjectionWarning { Severity { Required, Recommended, Optional, Info }, field, message }`, `project(profile, ctx, mode) -> CliResult<(Value, Vec<ProjectionWarning>)>`. `wrap_as_catalog`, `emit_field` (with `emit_when` + `default` + `required_level` semantics), `emit_recordset` (Croissant-style `for_each_column`), `register_profile_helpers` (closures for `lookup` + `field_mapping`), `dry_compile`. 9 unit tests. |
| `src/cmd/profile/discovery_merge.rs` | `merge(profile, inferred, discovered, forced_paths) -> Value`. Strategies: `fill-if-absent`, `overlay-array`, `never`. Honors `profile.discovery_merge.never_overwrite` + `forced_paths`. 5 unit tests. |
| `resources/profiles/dcat-us-v3.yaml` | **AUTHORED** — full DCAT-US v3 projection (Stage 3). |
| `resources/profiles/dcat-ap-v3.yaml` | **PLACEHOLDER** — populated in Stage 6. |
| `resources/profiles/croissant.yaml` | **PLACEHOLDER** — populated in Stage 7. |
| `resources/profiles/README.md` | Authoring guide + per-field directive doc + versioning contract. |
| `tests/resources/profile/golden/*.csv` | 3 fixture CSVs (10 rows each): nyc-311-subset, usda-soil-subset, wprdc-311-subset. |
| `tests/resources/profile/golden/*.expected.json` | 6 normalized goldens (3 fixtures × dataset/catalog modes). Path-dependent `qsv:sourcePath` fields stripped via jq. |

### New helpers added to `formula_helpers.rs` (11 of the planned 13)

| Name | Kind | Status |
|---|---|---|
| `only_if_absolute_iri` | filter | ✅ added |
| `basename` | filter | ✅ added |
| `file_stem` | filter | ✅ added |
| `sanitize_iso_8601_interval` | filter | ✅ added |
| `format_mailto` | filter | ✅ added |
| `sha256_of` | global | ✅ added (streaming SHA-256) |
| `blake3_of` | global | ✅ added (mmap+rayon, qsv-native) |
| `file_size_of` | global | ✅ added |
| `compress_format` | global | ✅ added |
| `package_format` | global | ✅ added |
| `build_csvw_schema` | global | ✅ added (emits `{columns: [...]}` shape) |
| `lookup` | global | ✅ added — lives in `projection.rs::register_profile_helpers` (needs profile state) |
| `field_mapping` | global | ✅ added — same |

All 13 are wired. The first 11 (profile-agnostic) live in
`formula_helpers.rs::register`. The last 2 (profile-aware) are
closures registered in `projection.rs::register_profile_helpers`.

### Legacy modules — still in place, unchanged

| Path | LOC | Status |
|---|---|---|
| `src/cmd/profile/dcat.rs` | 1738 | **Active** — drives the engine today. Delete in Stage 4. |
| `src/cmd/profile/catalog.rs` | 154 | **Active** — delete in Stage 4 (replaced by `projection::wrap_as_catalog`). |
| `src/cmd/profile/ckan_to_dcat.rs` | 271 | **Active** — delete in Stage 4 (replaced by `profile.field_mappings`). |
| `src/cmd/profile/curie.rs` | 225 | **Active** — delete in Stage 4 (logic moves into `dcat_validate.rs` using `profile.validation.strippable_curie_prefixes`). |

Net Rust LOC delta when Stage 4 lands: roughly **−2080 deleted**,
**+0 added** (the additions already happened in Stage 1), so the
overall PR ends with ~−1050 LOC reduction.

---

## 3. What's left (Stages 4-8)

### Stage 4: Orchestrator swap + delete legacy modules (BIG)

The bulk of the remaining work. Touches `src/cmd/profile.rs::run`
substantially. Sub-steps:

**4a. Wire `--profile` into `run()`:**
   - Load the profile at the top of `run()` via
     `profile_spec::load(args.flag_profile.as_deref().unwrap_or("dcat-us-v3"))`.
   - Add an early `projection::dry_compile(&profile)?` so a malformed
     embedded profile fails fast.
   - Stash the parsed `ProfileSpec` so downstream blocks can reach it.

**4b. Replace `dcat::build(...)` call (line ~373):**
   - The call site at `src/cmd/profile.rs:373` produces `(dcat_block,
     dcat_build_warnings)`. Replace with a `projection::project()` call
     that takes the same analysis context.
   - The analysis context fed to `projection::project()` needs to
     expose: `pkg` (= the merged CKAN package), `res` (= the merged
     CKAN resource[0]), `stats` (= `analysis.context.dpps`), `dpp`
     (= `analysis.context.dpp`), `source_label` (= `display_input`),
     `local_path` (= `input_path`). Build this as a single JSON Value
     and pass to `project()`. Reference: the YAML templates use
     `pkg.title`, `res.url`, `source_label`, `local_path`, `stats`,
     `pkg.dpp_suggestions.*`.

**4c. Replace `merge_discovered(...)` (line ~383) with
     `discovery_merge::merge(&profile, dcat_block, disc,
     &analysis.forced_dcat_paths)`.**

**4d. Replace `catalog::wrap_as_catalog(dataset)` (line ~457) with a
     `projection::project(&profile, &ctx, ProjectionMode::Catalog)`
     call, OR keep using the engine output from step 4b but pass
     `ProjectionMode::Catalog` from the start when `args.flag_catalog`.
     The cleaner refactor is to compute the catalog mode upstream.**

**4e. Update `apply_force_overrides` (line ~896) to consume
     `profile.field_mappings` instead of importing
     `ckan_to_dcat::translate_ckan_ptr`. The `field_mappings` table
     in `dcat-us-v3.yaml` mirrors the legacy table verbatim, so this
     is a literal swap.**

**4f. Wire the parity test (Stage 3's deferred test). Test sketch:**
```rust
#[test]
fn dcat_us_v3_golden_parity_dataset() {
    for fix in ["nyc-311-subset", "usda-soil-subset", "wprdc-311-subset"] {
        let wrk = Workdir::new(&format!("parity_{fix}"));
        // Copy fixture in
        let src = format!("tests/resources/profile/golden/{fix}.csv");
        let dst = wrk.path("in.csv");
        std::fs::copy(&src, &dst).unwrap();
        std::fs::copy(
            "tests/resources/profile/dcat-init-context.json",
            wrk.path("ic.json"),
        ).unwrap();

        let mut cmd = wrk.command("profile");
        cmd.args(["in.csv", "--profile", "dcat-us-v3",
                  "--initial-context", "ic.json", "-o", "out.json"]);
        wrk.assert_success(&mut cmd);

        let actual = read_output(&wrk, "out.json")["dcat"].clone();
        let golden = serde_json::from_str(&std::fs::read_to_string(
            &format!("tests/resources/profile/golden/{fix}.dataset.expected.json")
        ).unwrap()).unwrap();
        // Strip qsv:sourcePath the same way the golden was normalized.
        let actual = normalize(actual);
        assert_eq!(actual, golden, "dcat-us-v3 parity drift on {fix}");
    }
}
```
   (Plus a sibling test for catalog mode.)

**4g. Delete `src/cmd/profile/{dcat,catalog,ckan_to_dcat,curie}.rs`
     and remove their `mod` declarations from `profile.rs`. Migrate
     or drop the unit tests inside those modules — most cover behavior
     now exercised by the YAML+goldens parity tests.**

**4h. Stash key rename: `__pending_dcat_warnings` →
     `__pending_projection_warnings`. The warning filter still walks
     the post-override block via `final_dcat_has_field` (no API change
     needed there).**

**4i. `context.rs`: change `collect_forced_paths(raw_doc)` to take
     a `&ProfileSpec` so it can use `profile.field_mappings` rather
     than `use super::ckan_to_dcat`.**

### Stage 5: Refactor `dcat_validate.rs`

- Drop the hardcoded `BUNDLE` const + the four overlay `include_str!`s
  at the top of `dcat_validate.rs`.
- New public API: `validate(profile: &ProfileSpec, block: &Value) ->
  Vec<ProjectionWarning>`. When `profile.validation.enabled == false`,
  return `vec![]`.
- Load schemas from `profile.validation.schema_dir` (resolved relative
  to `CARGO_MANIFEST_DIR` for embedded-bundle profiles). For DCAT-US
  v3, the existing vendored GSA bundle under `resources/dcat-us-v3/`
  stays exactly where it is.
- CURIE strip list pulled from
  `profile.validation.strippable_curie_prefixes` instead of the
  deleted `curie::STRIPPABLE_PREFIXES`.
- The `Validator::options().build(...)` pattern stays.

### Stage 6: Author `dcat-ap-v3.yaml`

**FETCH spec from https://semiceu.github.io/DCAT-AP/releases/3.0.0/**
first to confirm:
- Required vs recommended cardinality
- The `@context` URI
- The EU theme vocabulary entries

Profile contents:
- `validation.enabled: false` (DCAT-AP ships SHACL, not JSON Schema).
- `vocabularies.eu_theme:` mapping CKAN-style tags to
  `http://publications.europa.eu/resource/authority/data-theme/<...>`
  IRIs.
- Smaller `field_mappings:` — drop `dcat-us:bureauCode` and
  `programCode` (DCAT-AP doesn't define them).
- `dataset.context:` pointing at the official DCAT-AP JSON-LD context.
- `required_level: required` only on `dct:title` and `dct:description`
  (DCAT-AP base cardinality).
- New helpers needed: **none** — reuses `lookup("eu_theme", tag)`.

Smoke test: `dcat_ap_v3_emits_eu_theme_iri` asserts `dcat:theme`
values are EU authority IRIs when `pkg.groups` contains slugs that
match the vocab.

### Stage 7: Author `croissant.yaml`

**FETCH spec from https://github.com/mlcommons/croissant** first to
confirm:
- The current `@context` (typically `https://schema.org/`)
- `@type: sc:Dataset`
- The RecordSet / Field shape
- Datatype mappings (`sc:Integer`, `sc:Float`, etc.)

Profile contents:
- `dataset.context: "https://schema.org/"`; field paths use
  schema.org bare keys (`name`, `description`, `url`, `license`,
  `creator`, `distribution`).
- `dataset.type: "sc:Dataset"`,
  `dct:conformsTo: "http://mlcommons.org/croissant/1.0"` (or current).
- `recordsets:` block (Croissant-specific) — one entry with
  `for_each_column: true` emitting one `cr:Field` per stats column.
  Engine already supports this in `projection.rs::emit_recordset`.
- `vocabularies.croissant_datatype:` mapping qsv stats types to
  `sc:Integer`, `sc:Float`, `sc:Date`, etc.
- `validation.enabled: false`.
- Distribution uses `blake3_of(local_path)` (qsv-native, faster than
  SHA-256 on multi-GB ML training data).
- `discovery_merge.enabled: false` (Croissant isn't typically
  discovered from CKAN-shaped publisher metadata).

Smoke tests:
- `croissant_emits_recordset_per_dataset` — asserts top-level
  `@type: sc:Dataset`, exactly one `cr:RecordSet`, one `cr:Field` per
  CSV column.
- `croissant_uses_schema_org_context` — asserts
  `@context: "https://schema.org/"`.

### Stage 8: Final cleanup

- Update `--profile` flag description in USAGE block (it's already
  there but verify it documents the new behavior post-swap).
- `cargo run -F all_features --bin qsv -- --generate-help-md` to
  refresh `docs/help/profile.md`.
- `cargo run -F all_features --bin qsv -- --update-mcp-skills` to
  refresh MCP skill JSONs.
- `python3 scripts/docs-drift-check.py` → must report "no drift
  detected".
- Verify all 4 binaries build clean:
  - `cargo build --bin qsv -F all_features`
  - `cargo build --bin qsvmcp -F qsvmcp`
  - `cargo build --bin qsvlite -F lite`
  - `cargo build --bin qsvdp -F datapusher_plus`
- `cargo +nightly fmt` final pass.
- Update CLAUDE.md memory if profile-engine workflow changed.

---

## 4. Build / test / verify

```bash
# Verify Stages 1-3 are wired correctly:
cargo build --bin qsv -F profile,feature_capable
cargo test  --bin qsv -F profile,feature_capable cmd::profile::

# Expect: 192 unit tests pass. Specifically the new modules:
# * 7 in profile_spec::tests (incl. embedded_dcat_us_v3_parses_and_dry_compiles)
# * 9 in projection::tests
# * 5 in discovery_merge::tests
# Plus 192 - 21 = 171 from the existing modules (all passing).

# Re-capture goldens (if Stage 2 needs refreshing):
for fix in nyc-311-subset usda-soil-subset wprdc-311-subset; do
  for mode in dataset catalog; do
    flag=""
    [[ "$mode" == "catalog" ]] && flag="--catalog"
    ./target/debug/qsv profile "tests/resources/profile/golden/$fix.csv" \
      --initial-context tests/resources/profile/dcat-init-context.json \
      $flag -o "/tmp/$fix.$mode.json"
    jq -f /tmp/normalize-dcat.jq "/tmp/$fix.$mode.json" \
      > "tests/resources/profile/golden/$fix.$mode.expected.json"
  done
done

# The jq normalization recipe:
# .dcat
# | if .["dcat:distribution"] != null then
#     .["dcat:distribution"] |= map(del(."qsv:sourcePath"))
#   else . end
# | if .["dcat:dataset"] != null then
#     .["dcat:dataset"] |= map(
#       if .["dcat:distribution"] != null then
#         .["dcat:distribution"] |= map(del(."qsv:sourcePath"))
#       else . end
#     )
#   else . end
```

---

## 5. Key insights / decisions made this session

- **`lookup` + `field_mapping` helpers MUST return
  `minijinja::Value::UNDEFINED` (not `None`) on miss.** Returning
  `Option<Value>` from the closure causes minijinja to render `none`
  (literal string) instead of triggering `| default(...)`. The
  closures use `unwrap_or(Value::UNDEFINED)` for this reason. See
  `projection.rs::register_profile_helpers`.

- **`coerce_json_or_string` decision rule:** a rendered template
  string starting with `{` or `[` is parsed as JSON; anything else
  stays a literal string. Templates emitting objects use raw curly
  braces; templates emitting strings can use `| tojson` for safe
  quoting.

- **Goldens normalize only `qsv:sourcePath`.** Everything else in
  `.dcat` (including `dcat:byteSize`, `dcat:checksum`,
  `csvw:tableSchema`) is deterministic for fixed input. The jq
  recipe is committed inline in the Stage-2 commit message.

- **Field-mapping count diverges from the plan.** Plan said 47
  entries; the actual legacy `CKAN_TO_DCAT` has 56 and the YAML's
  `field_mappings:` has 53 (a few duplicates collapsed). Not a
  blocker — the dry-compile + golden tests are the source of truth.

- **Catalog envelope produces 5 keys.** Today: `@type`, `dct:title`,
  `dct:conformsTo`, `dcat:dataset`, `dct:publisher` (when present).
  The YAML's `catalog:` block reproduces this verbatim.

- **`required_level: required` warnings** drive the `dcat_warnings`
  array. The Stage-2 goldens were captured against the canonical
  `dcat-init-context.json` template which populates contactPoint,
  bureauCode, etc., so the goldens carry **zero** warnings. New
  fixtures missing these fields will emit warnings; the parity test
  must compare those too.

---

## 6. Reference URLs

- Plan file: `/Users/joelnatividad/.claude/plans/looking-at-src-cmd-profile-rs-luminous-moth.md`
- DCAT-US v3 spec: <https://github.com/GSA/dcat-us/>
- DCAT-AP v3 spec: <https://semiceu.github.io/DCAT-AP/releases/3.0.0/>
- Croissant spec: <https://github.com/mlcommons/croissant>
- Pittsburgh 311 live URL (smoke test): <https://data.wprdc.org/datastore/dump/5202679a-d243-402e-b82a-63189995a942>

---

## 7. Quick resume checklist

```bash
cd /Users/joelnatividad/GitHub/qsv
git status
git log --oneline -5   # confirm b7ecfc41d, a01da79aa, 6732c644d are on the branch

# Pick up Stage 4 by reading:
cat profile3-handoff.md          # this file
cat profile2-handoff.md          # prior PR-#3901-era context

# Run baseline tests:
cargo test --bin qsv -F profile,feature_capable cmd::profile:: 2>&1 | tail -3
# Expect: 192 passed, 0 failed.

# Inspect the YAML profile to confirm it's intact:
head -50 resources/profiles/dcat-us-v3.yaml

# The Stage 4 entry point: profile.rs:373 — the dcat::build call.
# Read it first via:
#   mcp__serena__find_symbol(name_path="run", relative_path="src/cmd/profile.rs", include_body=false)
# Then start the surgical replacement of dcat::build → projection::project.
```
