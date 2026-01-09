# Continuous Integration (CI)

## Overview

The MCP Server has automated CI testing via GitHub Actions that runs on every push and pull request affecting the `.claude/skills/` directory.

**Workflow File**: `.github/workflows/mcp-server-ci.yml`

## Test Matrix

Tests run across multiple environments to ensure broad compatibility:

### Operating Systems
- ✅ Ubuntu (Linux x86_64)
- ✅ macOS 14 (Apple Silicon ARM64)
- ✅ Windows (x86_64)

### Node.js Versions
- ✅ Node.js 18 (minimum supported)
- ✅ Node.js 20 (LTS)
- ✅ Node.js 22 (current)

**Total**: 9 test combinations (3 OS × 3 Node versions)

**Note**: macOS tests run on ARM64 (Apple Silicon) runners to match available qsv builds.

## What Gets Tested

### 1. Test Job (Matrix)
Each combination runs:
1. **qsv Installation**: Downloads and installs latest qsv binary for the platform
2. **Dependency Installation**: `npm ci` for clean install
3. **TypeScript Compilation**: `npm run build` to verify code compiles
4. **Integration Tests**: `npm test` runs all 28 tests including:
   - Unit tests for config, filesystem, pipeline, tools
   - Integration tests for qsv commands (count, headers, select, search, etc.)
   - Metadata caching and deduplication tests
5. **Build Verification**: Tests that `dist/mcp-server.js` is executable

### 2. Lint Job
Separate TypeScript type checking:
1. **Main Code Compilation**: `npm run build`
2. **Test Code Compilation**: `npm run build:test`

## Triggering CI

CI runs automatically on:
- **Push to master**: Any changes to `.claude/skills/**` or the workflow file
- **Pull Requests**: Any PR targeting master with changes to MCP server code
- **Manual Trigger**: Via GitHub Actions UI (`workflow_dispatch`)

## Viewing Results

1. Go to the [Actions tab](https://github.com/dathere/qsv/actions) in the repository
2. Select "MCP Server CI" workflow
3. View individual test runs for each OS/Node combination

## CI Status Badge

Add this badge to README.md to show CI status:

```markdown
[![MCP Server CI](https://github.com/dathere/qsv/actions/workflows/mcp-server-ci.yml/badge.svg)](https://github.com/dathere/qsv/actions/workflows/mcp-server-ci.yml)
```

## Local Testing

To run tests locally matching CI environment:

```bash
# Install dependencies
npm ci

# Run full test suite (what CI runs)
npm test

# Run build check
npm run build

# Check test compilation
npm run build:test
```

## Troubleshooting CI Failures

### qsv Installation Fails
- Check that qsv releases are available on GitHub
- Verify download URLs match current release naming convention: `qsv-{VERSION}-{TARGET}.zip`
- Supported targets:
  - Ubuntu: `x86_64-unknown-linux-gnu`
  - macOS: `aarch64-apple-darwin` (ARM64 only)
  - Windows: `x86_64-pc-windows-msvc`

### Tests Fail on Specific Platform
- Check platform-specific path handling (Windows vs Unix)
- Verify qsv binary is in PATH correctly

### TypeScript Compilation Fails
- Run `npm run build` locally to see detailed errors
- Check for missing type definitions or incompatible Node.js types

### Node.js Version Issues
- Ensure code uses features compatible with Node.js 18+
- Check `package.json` engines field matches minimum supported version

## Performance

Typical CI run times:
- **Test Job (per combination)**: 2-3 minutes
- **Lint Job**: 30-60 seconds
- **Total (all jobs)**: ~3-5 minutes (parallel execution)

## Dependencies

CI automatically uses:
- Latest qsv release from GitHub
- Node.js versions from `actions/setup-node`
- npm dependencies locked in `package-lock.json`
