# Suggested Commands

## Build
- `npm run build` — compile TypeScript to `dist/`
- `npm run build:test` — compile with test config (includes test files)

## Test
- `npm test` — builds with test config, then runs all tests
- `node --test dist/tests/<module>.test.js` — run a specific test file
- `node --test --test-name-pattern="pattern" dist/tests/` — filter tests by name
- `npm run test:watch` — watch mode for tests
- `npm run test:coverage` — run tests with c8 coverage

## Run
- `npm run mcp:start` — start the MCP server (`node dist/mcp-server.js`)
- `npm run mcp:install` — install MCP server config

## Package & Release
- `npm run mcpb:package` — package as `.mcpb` MCP bundle
- `npm run plugin:package` — package as `.plugin` Cowork plugin
- `npm run plugin:install` — install Cowork plugin locally

## Skill Regeneration
- `qsv --update-mcp-skills` — regenerate skill JSONs in `qsv/` directory (run from repo root)

## System Utilities (macOS / Darwin)
- `git`, `ls`, `cd`, `grep`, `find` — standard system commands
- `node`, `npm` — Node.js runtime and package manager
- `tsc` — TypeScript compiler (via npx or npm scripts)
