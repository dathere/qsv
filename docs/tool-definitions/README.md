# qsv tool definitions

One JSON tool definition per qsv command (`qsv-<command>.json`), describing its arguments, options, examples and behavioral hints. Agents and other tools can use them to integrate with qsv.

These files are GENERATED from each command's USAGE text by `qsv --update-mcp-skills`. Never hand-edit them. The curated MCP skill set in [`.claude/skills/qsv/`](../../.claude/skills/qsv) consists of byte-identical copies of a subset of these files.

They are also embedded in the `qsv` and `qsvmcp` binaries, so you can get the definitions matching your installed version without a repo checkout:

```bash
qsv --tool-definition stats          # print one definition to stdout
qsv --export-tool-definitions defs   # write every installed command's definition,
                                     # the help Markdown and a manifest.json to ./defs
```

See [#4638](https://github.com/dathere/qsv/issues/4638).
