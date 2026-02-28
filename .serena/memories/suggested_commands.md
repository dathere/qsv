# Suggested Commands

## Rust (main qsv project)

### Building
```bash
# Full-featured variant (use for development)
cargo build --locked --bin qsv -F all_features

# qsvmcp variant
cargo build --locked --bin qsvmcp -F qsvmcp

# qsvlite variant
cargo build --locked --bin qsvlite -F lite
```

### Testing
```bash
# Test all with full features
cargo test --features all_features

# Test specific command
cargo t stats -F all_features

# Test with specific features
cargo t luau -F feature_capable,luau,polars
```

### Formatting & Linting
```bash
cargo +nightly fmt
cargo +nightly clippy -F all_features -- -W clippy::perf
```

### Regenerate MCP Skills
```bash
./target/debug/qsv --update-mcp-skills
```

## TypeScript (MCP Server at .claude/skills/)

### Building
```bash
npm run build          # Production build
npm run build:test     # Test config build
```

### Testing
```bash
npm test               # Build + run all tests
npm run test:watch     # Watch mode
node --test dist/tests/mcp-filesystem.test.js  # Single test file
```

### MCP Server
```bash
npm run mcp:start      # Start server (stdio)
npm run mcp:install    # Install to Claude Desktop
npm run mcpb:package   # Package as MCPB bundle
```

## System (Darwin/macOS)
```bash
git status / git diff / git log
ls / find / grep
```
