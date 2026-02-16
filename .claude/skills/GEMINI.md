# QSV Agent Skills - Project Overview

QSV Agent Skills is a high-performance TypeScript implementation of a Model Context Protocol (MCP) server that exposes the extensive tabular data-wrangling capabilities of the `qsv` toolkit to AI agents like Claude. It enables seamless discovery, invocation, and composition of `qsv` commands for processing CSV, TSV, Excel, JSONL, and other tabular formats directly from local filesystems.

## Key Technologies
- **Language:** TypeScript (Node.js >= 18.0.0)
- **Framework:** @modelcontextprotocol/sdk
- **Core Engine:** `qsv` binary (Rust-based data-wrangling toolkit)
- **Search:** `wink-bm25-text-search` for intelligent tool discovery
- **Packaging:** MCP Bundle (MCPB) for easy distribution and installation

## Architecture & Core Components

- **MCP Server (`src/mcp-server.ts`):** The primary entry point that implements the Model Context Protocol. It manages tool registration, resource handling (filesystem access), and provides intelligent "server instructions" to guide agent workflows.
- **Tool Definition System (`src/mcp-tools.ts`):** Dynamically generates tool definitions for 66+ `qsv` commands. It includes a "Guidance Enhancement" system providing `USE WHEN`, `COMMON PATTERNS`, and `CAUTION` hints to help agents make optimal tool choices.
- **Execution Engine (`src/executor.ts`):** A robust wrapper around `spawn` for streaming `qsv` commands. It handles stdin/stdout/stderr buffering, provides a 50MB output size limit (auto-saving larger results to disk), and implements sophisticated timeout and process management.
- **Skill Loader (`src/loader.ts`):** Dynamically loads skill definitions from auto-generated JSON files in the `qsv/` directory. These JSON files are derived from `qsv`'s usage text using a specialized Rust-based generator.
- **Filesystem & Cache Management:**
  - `src/mcp-filesystem.ts`: Provides secure filesystem access restricted to allowed directories.
  - `src/converted-file-manager.ts`: A LIFO cache for converted files (e.g., Excel to CSV) with automatic cleanup and file locking.
- **Update System (`src/update-checker.ts`):** Monitors `qsv` binary versions and can auto-regenerate skill definitions to ensure synchronization with the underlying toolkit.

## Building and Running

### Development Commands
- **Install Dependencies:** `npm install`
- **Build TypeScript:** `npm run build`
- **Build for Testing:** `npm run build:test`
- **Run Tests:** `npm test` (Uses a cross-platform runner `scripts/run-tests.js`)
- **Watch Mode:** `npm run test:watch`

### MCP Operations
- **Start MCP Server (stdio):** `npm run mcp:start`
- **Install to Claude Desktop:** `npm run mcp:install` (Updates `claude_desktop_config.json`)
- **Package for Distribution:** `npm run mcpb:package` (Generates `.mcpb` bundle)

## Development Conventions

- **Tool Discovery:** Employs "Deferred Tool Loading." Only 9 core tools are loaded initially to reduce token usage; others are discovered via `qsv_search_tools`.
- **Guidance Enhancement:** When adding or modifying tools, include `USE WHEN`, `COMMON PATTERNS`, and `CAUTION` hints in `src/mcp-tools.ts`.
- **Performance Optimization:**
  - **Streaming:** Always use the streaming executor for data processing.
  - **Auto-indexing:** Files > 10MB are automatically indexed for performance.
  - **Stats Cache:** Commands like `stats` auto-generate JSONL caches used by downstream operations.
- **Testing:** Every module must have a corresponding `.test.ts` file in the `tests/` directory. Use the `test-helpers.ts` utilities for consistent setup/cleanup.
- **Type Safety:** Strict TypeScript rules apply. Avoid `any`; use `unknown` with type guards.
- **Version Sync:** The project version in `package.json` and `version.ts` must track the `qsv` binary version.

## Key Configuration (Environment Variables)
- `QSV_MCP_BIN_PATH`: Path to the `qsv` binary.
- `QSV_MCP_WORKING_DIR`: Default directory for file operations.
- `QSV_MCP_ALLOWED_DIRS`: Restricted list of directories accessible by the server.
- `QSV_MCP_EXPOSE_ALL_TOOLS`: Set to `true` to disable deferred loading.
