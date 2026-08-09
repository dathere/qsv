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
5. **`CHANGELOG.md`**: Add new version section at top

If MSRV is changing, also update:
6. **`Cargo.toml`** (line 15): `rust-version = "X.Y"`
7. **`CLAUDE.md`**: `**MSRV**: Rust X.Y` in Project Overview

## Changelog Entry

Add a new section at the top of `CHANGELOG.md` following this format:

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- (new features)

### Changed
- (changes to existing features)

### Fixed
- (bug fixes)

### Removed
- (removed features)
```

Use `git log` from the last release tag to populate the changelog sections.
**Exclude** commits with `(mcp)` or `(plugin)` in the title — those belong in the MCP/Plugin changelog (`.claude/skills/CHANGELOG.md`) and are handled by `/mcp-release-prep`.
Add links to relevant PRs and issues for each changelog entry when possible.

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

## Cross-repo constraints — `dathere/qsv-easy-windows-installer`

The Windows MSI "Easy installer" is a separate repo that consumes qsv releases directly.
Two of its assumptions are things **this** repo controls, so they are release-time checks:

**Installer v1.1.2 (2026-08-09) fixed both of its fragile assumptions** — it now reads
`tag_name` instead of the release title, and extracts `qsv.exe` instead of `qsvp.exe`.
Verified against the live API: `releases/latest` → the tag, the constructed
`.../releases/download/{tag}/qsv-{tag}-x86_64-pc-windows-msvc.zip` returns 200, and
`qsv.exe` is present, for both 21.1.0 and 22.0.1.

What remains:

- **Users on Easy installer ≤ v1.1.1 break when the first stable release without `qsvp.exe`
  ships.** Those versions extract `qsvp.exe` by hardcoded name. They keep working today only
  because `releases/latest` EXCLUDES prereleases, so they still resolve to 21.1.0, which
  ships `qsvp.exe`. The moment 22.0.1 (or any later release) is promoted to stable, they get
  a zip with no `qsvp.exe` and fail. **Worth a line in the release notes telling Windows
  Easy-installer users to upgrade to ≥ v1.1.2.**

- **Release TITLE == tag is no longer load-bearing for v1.1.2+**, but ≤ v1.1.1 still
  interpolates `.name` into the download URL as if it were the tag. Keeping titles as the
  bare version costs nothing and avoids 404ing those users on top of the `qsvp` failure.
