# Task Completion Checklist

After completing a coding task in this project, run the following:

1. **Build**: `npm run build` — TypeScript must compile cleanly
2. **Test**: `npm test` — full suite (does `build:test` first, then `node scripts/run-tests.js`)
3. **Verify specific changes**: `node --test dist/tests/<module>.test.js` for affected test files
4. **MCP / Serena tools**: prefer `mcp__serena__*` for navigation and symbolic edits during the work

## When modifying tool definitions or handlers
- Add the tool to `toolDispatchMap` in `src/mcp-server.ts` if it's a specialized handler
- Add the command name to `COMMON_COMMANDS` in `src/tool-constants.ts` if it's a generic-command tool
- Update `COMMAND_GUIDANCE` in `src/command-guidance.ts` (`whenToUse`, `commonPattern`, optional `errorPrevention`)
- Public symbols intended for consumers should be re-exported from `src/mcp-tools.ts` (the barrel) and `src/index.ts`

## When modifying skill JSONs
- DO NOT edit `qsv/qsv-*.json` manually — they are auto-generated from qsv USAGE text
- Run `qsv --update-mcp-skills` from the qsv repo root, then `npm run build` here
- The Rust generator lives at `src/mcp_skills_gen.rs` in the qsv repo

## Versioning / release sync
- `package.json` and `manifest.json` versions MUST match (`src/version.ts` asserts this)
- Update both before packaging

## Packaging (only when releasing)
- `npm run mcpb:package` — MCP bundle (`.mcpb`)
- `npm run plugin:package` — Cowork plugin (`.plugin`)
- `npm run clean:bundles` to remove stale artifacts first if needed

## Style
- Project rustfmt note (for qsv Rust repo, not this dir): always `cargo +nightly fmt`
- TS code: ESM imports use `.js` extensions; kebab-case filenames; classes PascalCase; functions camelCase; constants UPPER_SNAKE_CASE
