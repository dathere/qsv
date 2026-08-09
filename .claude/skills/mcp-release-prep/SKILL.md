---
name: mcp-release-prep
description: Prepare an MCP server and plugin release by bumping versions across all files and updating changelog
disable-model-invocation: true
---

# MCP Server & Plugin Release Preparation

Prepare an MCP server and plugin release by updating version numbers across all required files and generating a changelog entry. Per the policy adopted in 20.0.0, the MCP server version **tracks the qsv binary version** — 20.0.0, 20.1.0, 21.0.0, 21.1.0 and 22.0.1 all shipped as companion releases to the matching binary. (The version numbers are mechanically independent — nothing enforces the match — but pick the binary's version unless you have a specific reason not to.)

## Arguments

- `version` (required): The new MCP server version number (e.g., "16.2.0")
- `minimum_qsv_version` (optional): New minimum qsv binary version, if changing

All paths below are relative to `.claude/skills/`.

## Version Bump Checklist

### Core (must always update)

1. **`package.json`**: find `"version":` field — source of truth; `version.ts` reads this at runtime
2. **`manifest.json`**: find `"version":` field near top — must match package.json
3. **`.claude-plugin/plugin.json`**: find `"version":` field
4. **`../../.claude-plugin/marketplace.json`** (repo root): find `"version":` in both `metadata` and `plugins[0]` — must match package.json

After bumping `package.json`, run `npm install --package-lock-only` to sync `package-lock.json`.

> **Do NOT look for a `version:` field in `agents/*.md` frontmatter.** Agent frontmatter carried
> one through 19.1.1 but it was removed since; the three agent files now have only `name`,
> `description`, and `allowed-tools`. There is nothing to bump there.

### Conditional (only if minimum qsv binary version changes)

5. **`manifest.json`**: find `"minimum_qsv_version":` in `_meta["com.dathere.qsv"]` — this is the
   **single source of truth** and the only edit point.

   Both consumers read it from `manifest.json` at **runtime**; neither holds a hardcoded version,
   so do not go hunting for a `MINIMUM_QSV_VERSION` constant to edit:
   - `src/config.ts` — `loadMinimumQsvVersion()` → `readMinimumQsvVersionFromManifest()`; falls
     back to `"0.0.0"` (check becomes a no-op) if the manifest can't be read.
   - `scripts/cowork-setup.cjs` — same logic in CJS, same `"0.0.0"` fallback.

   Raise the floor when the regenerated skill JSONs advertise flags that only exist in the new
   binary (the 21.1.0 and 22.0.1 precedent). If the release is purely a docs/packaging change,
   leaving the floor alone is fine.

### Documentation (hardcoded versions to update)

These footers rot easily — they have been missed in past releases, so check all three even if the
release is small. Each is an `**Updated**: YYYY-MM-DD` / `**Version**: X.Y.Z` pair at the bottom
of the file.

6. **`README-MCP.md`**: `**Version**:` footer near the bottom
7. **`docs/desktop/README-MCPB.md`**: `**Version**:` footer near the bottom (this is the only
   version string in the file — there are no `releases/download/<ver>/` URLs to update)
8. **`README.md`**: `**Version**:` footer near the bottom
9. **`docs/guides/START_HERE.md`**: check for hardcoded version references (currently none — it
   uses an unversioned `.mcpb` reference)

A catch-all sweep for anything the list misses — from the repo root, with `<old>` as the version
you are replacing:

```bash
git grep -n '<old>' -- .claude/skills .claude-plugin
```

This must come back clean (modulo historical prose in `docs/audits/`) **before** packaging: the
packaging scripts do no cross-validation, so a partial sweep silently produces a correctly-named
bundle containing a stale manifest.

### Command count verification

Verify the skill-based command count matches actual files:

```bash
ls qsv/qsv-*.json | wc -l
```

Then check descriptions in these files for stale counts:
- `.claude-plugin/plugin.json` description
- `../../.claude-plugin/marketplace.json` — both `metadata.description` and `plugins[0].description`
- `cowork-CLAUDE.md` — Tool Discovery section
- `skills/csv-wrangling/SKILL.md` — Tool Discovery section
- `manifest.json` — `_meta.com.dathere.qsv.features` array

## Changelog Entry

Add a new section at the top of `CHANGELOG.md` (in `.claude/skills/`) following this format:

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- (new features)

### Changed
- (changes to existing features)

### Fixed
- (bug fixes)
```

Find the last qsv release tag and use it to populate the changelog:

```bash
# Find the most recent qsv release tag
LAST_TAG=$(git describe --tags --match '[0-9]*.[0-9]*.[0-9]*' --abbrev=0 2>/dev/null || echo "")

# List relevant commits since that tag (or all commits from repo root if no tag exists)
git log --oneline --no-merges --grep="(mcp)" "${LAST_TAG:-$(git rev-list --max-parents=0 HEAD)}"..HEAD
```

Only commits with `(mcp)` or `(plugin)` in the title are relevant to MCP server releases.

## Verification Steps

After version bumps:

1. `npm run build` — TypeScript compilation succeeds
2. `npm test` — all tests pass. This is the **real gate**: release commits use `[skip ci]`, so
   `mcp-server-ci.yml` will not run on them.
3. `npm run mcpb:package` — generates `qsv-mcp-server-X.Y.Z.mcpb`
4. `npm run plugin:package` — generates `qsv-data-wrangling-X.Y.Z.plugin`

⚠️ **Run steps 3 and 4 back to back.** `clean:bundles` is chained onto *both* and deletes every
`.mcpb`/`.plugin` not matching the current version — so `mcpb:package` removes the previous
release's `.plugin` before `plugin:package` rebuilds it. Stopping in between leaves you with only
one artifact on disk. (Harmless in practice: prior artifacts live on their GitHub release, and
both file types are gitignored, so they are never committed.)

## Publishing

There are **two independent channels**, and only one involves an artifact:

| Channel | Mechanism | Artifact? |
|---|---|---|
| Claude Code marketplace | `marketplace.json` has `"source": "./.claude/skills"` — installs pull live from master | **No — pushing to master *is* the publish** |
| GitHub release assets | `.mcpb` (Claude Desktop) + `.plugin` (Cowork) attached to the qsv release | Yes, uploaded by hand |

Nothing is automated; there is no CI job that packages or publishes either artifact.

```bash
git commit   # subject: chore(mcp): vX.Y.Z release of MCP Server and Claude Cowork plugin
             # body:    [skip ci]
git push
gh release upload <tag> qsv-mcp-server-X.Y.Z.mcpb qsv-data-wrangling-X.Y.Z.plugin
```

⚠️ **Check the binary release is complete before pushing.** Because the marketplace serves live
from master, pushing advertises a plugin whose `minimum_qsv_version` users must be able to satisfy
by download. If `publish.yml` is still in flight — or the release is still marked *Pre-release* —
some platforms have no binary yet, and those users get a version-floor warning from
`cowork-setup.cjs` (a warning, not a refusal). Confirm with `gh release view <tag>` and
`gh run list --workflow=publish.yml` first.

## Cowork Plugin

The Cowork plugin (`.plugin` file) is a separate distribution artifact from the Desktop Extension (`.mcpb`). It provides the workflow layer (skills, agents, hooks) without the MCP server itself.

### Plugin components (all relative to `.claude/skills/`)

- **`.claude-plugin/plugin.json`** — plugin manifest (version already bumped in Core step 3)
- **`scripts/cowork-setup.cjs`** — SessionStart hook that deploys `cowork-CLAUDE.md` to the working directory and validates the qsv binary
- **`cowork-CLAUDE.md`** — workflow template deployed by the hook
- **`skills/`** — 15 SKILL.md files: 9 user-invocable (csv-query, data-clean, data-convert, data-describe, data-join, data-profile, data-validate, data-viz, infer-ontology) + 6 model-invoked (bls-query, csv-wrangling, data-quality, genai-disclaimer, qsv-performance, reproducible-analysis)
- **`agents/`** — subagents (data-analyst, data-wrangler, policy-analyst)

### Plugin-specific review

When preparing a release, also review:

- **`cowork-CLAUDE.md`**: check that tool names, workflow steps, and limits are still accurate
- **`scripts/cowork-setup.cjs`**: if `minimum_qsv_version` changed, update the `MINIMUM_QSV_VERSION` constant (listed in Conditional step 10)
- **`skills/`**, **`agents/`**: check for any hardcoded version references or stale tool names

## Important Notes

- MCP server version advances **independently** of qsv binary version
- `minimum_qsv_version` tracks binary compatibility, not server version — enforced in 3 places: `src/config.ts`, `manifest.json`, and `scripts/cowork-setup.cjs` (all must stay in sync)
- Skill JSON files (`qsv/*.json`) are auto-generated by the **qsv binary** (`qsv --update-mcp-skills`), not by this skill — only bump those via `/release-prep`
- `version.ts` reads version from `package.json` at runtime — no need to edit `version.ts` directly
- The `.plugin` package reads its version from `package.json` (same source of truth as the `.mcpb`)
