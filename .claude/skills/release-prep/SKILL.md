---
name: release-prep
description: Prepare a qsv release by bumping versions across all files and updating changelog
disable-model-invocation: true
---

# Release Preparation

Prepare a qsv release by updating version numbers across all required files and generating a changelog entry.

## Arguments

- `version` (required): The new version number (e.g., "16.2.0")
- `msrv` (optional): New minimum supported Rust version, if changing

## Version Bump Checklist

Update the version string in ALL of these files:

1. **`Cargo.toml`** (line 3): `version = "X.Y.Z"`
2. **`CLAUDE.md`**: `**Current Version**: X.Y.Z` in Project Overview
3. **`.claude/skills/manifest.json`**: `"version"` field (MCP server version -- may differ from binary version)
4. **`.claude/skills/package.json`**: `"version"` field (must match manifest.json)
5. **`docs/CHANGELOG.md`**: Add new version section at top

If MSRV is changing, also update:
6. **`Cargo.toml`** (line 15): `rust-version = "X.Y"`
7. **`CLAUDE.md`**: `**MSRV**: Rust X.Y` in Project Overview

## Changelog Entry

Add a new section at the top of `docs/CHANGELOG.md` following this format:

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- (new features)

### Changed
- (changes to existing features)

### Fixed
- (bug fixes)

### Performance
- (performance improvements)
```

Use `git log` from the last release tag to populate the changelog sections.

## Post-Version-Bump Steps

After version bumps, remind the user to:

1. Run `cargo build --locked --bin qsv -F all_features` to verify the build (omit `--locked` if deps changed)
2. Run `cargo test -F all_features` to verify tests pass
3. Run `qsv --update-mcp-skills` to regenerate skill JSONs with new version
4. Run `bash contrib/completions/generate_examples.bash` to regenerate completions
5. Run `cargo +nightly fmt` to format any changed Rust files
6. Commit all changes together

## Important Notes

- The MCP server version in `manifest.json`/`package.json` can advance independently of the qsv binary version
- The `minimum_qsv_version` field in `manifest.json` tracks the minimum *qsv binary* needed, NOT the MCP server version
- After bumping `Cargo.toml` version, omit `--locked` from cargo commands until `Cargo.lock` is regenerated
