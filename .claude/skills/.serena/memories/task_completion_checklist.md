# Task Completion Checklist

After completing a coding task in this project, run the following:

1. **Build**: `npm run build` — ensure TypeScript compiles without errors
2. **Test**: `npm test` — run the full test suite (builds with test config first)
3. **Verify specific changes**: `node --test dist/tests/<module>.test.js` for affected test files

## When modifying tool definitions or handlers
- Ensure the tool appears correctly in `tool-definitions.ts` and has a dispatch entry in `mcp-server.ts`
- If adding guidance, update `COMMAND_GUIDANCE` in `command-guidance.ts`
- For common commands, add to `COMMON_COMMANDS` array in `mcp-tools.ts`

## When modifying skill JSONs
- Do NOT edit `qsv/*.json` manually — they are auto-generated
- Run `qsv --update-mcp-skills` from the repo root, then `npm run build`

## Packaging (only when releasing)
- `npm run mcpb:package` — MCP bundle
- `npm run plugin:package` — Cowork plugin
