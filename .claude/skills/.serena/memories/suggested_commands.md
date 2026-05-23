# Suggested Commands

All commands run from `/Users/joelnatividad/GitHub/qsv/.claude/skills/` unless noted.

## Build
- `npm run build` — compile TypeScript to `dist/`
- `npm run build:test` — compile with test config (includes test files)

## Test
- `npm test` — builds with test config, then runs all tests via `scripts/run-tests.js`
- `node --test dist/tests/<module>.test.js` — run a specific test file
- `node --test --test-name-pattern="pattern" dist/tests/` — filter tests by name
- `npm run test:watch` — watch mode for tests
- `npm run test:coverage` — c8 text coverage (`npm run build:test && c8 --reporter=text node scripts/run-tests.js`)
- `npm run test:examples` — runs `examples/basic.js`
- `npm run test-update-checker` — runs `examples/update-checker-demo.js`

## Run
- `npm run mcp:start` — start the MCP server (`node dist/mcp-server.js`)
- `npm run mcp:install` — install MCP server config

## Package & Release
- `npm run mcpb:package` — package as `.mcpb` MCP bundle (`qsv-mcp-server-<v>.mcpb`)
- `npm run plugin:package` — package as `.plugin` Cowork plugin (`qsv-data-wrangling-<v>.plugin`)
- `npm run plugin:install` — install Cowork plugin locally (`bash scripts/install-cowork-plugin.sh`)
- `npm run clean:bundles` — clean built bundle artifacts

## Skill Regeneration (from qsv repo root, NOT from this dir)
- `qsv --update-mcp-skills` — regenerate skill JSONs in `.claude/skills/qsv/`
- After regeneration: `npm run build` here

## System Utilities (macOS / Darwin)
- Standard: `git`, `ls`, `cd`, `grep`, `find`
- Toolchain: `node`, `npm`, `tsc` (via npx or npm scripts)
