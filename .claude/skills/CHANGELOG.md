# Changelog

All notable changes to the qsv Agent Skills (MCP Server) project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **DuckDB integration** — Automatic DuckDB routing for SQL queries when DuckDB binary is detected
  - New `src/duckdb.ts` module with lazy detection, SQL translation, and query execution
  - Parquet-first strategy: CSV inputs are auto-converted to Parquet before SQL execution
  - DuckDB preferred for PostgreSQL-compatible SQL; falls back to Polars SQL (sqlp) when unavailable
  - Unsupported output formats (Arrow, Avro) automatically fall back to sqlp
  - New env vars: `QSV_MCP_DUCKDB_BIN_PATH`, `QSV_MCP_USE_DUCKDB`
  - DuckDB status shown in `qsv_config` output
  - Engine indicator (`🦆 Engine: DuckDB vX.Y.Z`) prepended to DuckDB query results
  - MCPB Desktop Extension: DuckDB settings exposed in user config UI (`duckdb_path`, `use_duckdb`)

## [16.1.0] - 2026-02-15

### Added
- **pragmastat skill** - New `qsv-pragmastat.json` skill with examples and `--memcheck` flag for conservative memory pre-checks
- **Auto-enable --frequency-jsonl** for frequency command (version guard for qsv >= 16.1.0, skips stdin)
- **Frequency cache options** added to `qsv-frequency.json` skill definition
- **Comprehensive mcp-server test suite** - `mcp-server.test.ts` with 35 tests
- **Google Gemini CLI support** - Extended plugin mode detection + `QSV_MCP_PLUGIN_MODE` override
- **c8 code coverage** - `test:coverage` script
- **Stderr size limit** - Prevents memory issues from excessive error output

### Changed
- **Comprehensive code deduplication** — Refactored MCP server removing ~625 lines across 19 files
  - Consolidated 3 `runQsv` implementations into shared `runQsvSimple` with `onSpawn`/`onExit` callbacks
  - Merged 7 guidance tables into single `COMMAND_GUIDANCE` map
  - Decomposed `handleToolCall` into 4 focused functions
  - Extracted `errorResult`/`successResult` helpers and shared test helpers
  - Optimized SkillLoader with parallel reads and single-pass `getStats`
  - Replaced `Record<string, any>` with `unknown` for type safety
  - Removed dead code (`getFileContent`, `pipelineToShellScript`)
- **Renamed 'analysis' category to 'documentation'** for AI-powered documentation commands
- **Removed unused `binary` field** from CommandSpec type
- **Extracted filesystem tools** into `mcp-filesystem.ts` module
- **Improved COMMAND_GUIDANCE** — Enhanced per-command guidance hints
- **Hardcoded min version** for frequency-jsonl guard (16.1.0)
- **Regenerated skills** with normalized USAGE text, sans unused binary property
- **Minimum qsv version** set to 16.0.0 in manifest metadata

### Fixed
- Skip stats/frequency cache when reading from stdin
- Non-null assertion removal in executeWithFile
- normalizeOptionKey edge case for already-prefixed options
- NaN for invalid versions instead of incorrect comparison
- Stale documentation (core tool counts, pipeline references, artifact counts)
- Timing-dependent test fixes and tightened error assertions
- Windows 8.3 path compatibility in plugin mode tests
- Copilot review fixes for plugin mode validation
- Typo: `diretory` → `directory`

### Dependencies
- Bumped `qs` from 6.14.1 to 6.14.2

### Removed
- `qsv_pipeline` tool — agents should call tools sequentially for better error visibility, or use `qsv_sqlp` for multi-step queries
- `QSV_MCP_MAX_PIPELINE_STEPS` environment variable
- `pipeline.ts` fluent API and `mcp-pipeline.ts` tool handler
- Stale documentation: `PROFILE_CACHE.md`, `COPILOT_REVIEW_FIXES*.md`, `CHANGELOG_FILESYSTEM.md`, `examples/pipeline.js`

## [16.0.0] - 2026-02-08

### Added
- **CSV-to-Parquet Core Tool** - New `qsv_to_parquet` tool converts CSV files to Parquet format for optimized SQL operations
  - Auto-generates stats and Polars schema (`.pschema.json`) for correct data type inference
  - Sniffs CSV for Date/DateTime columns via `--infer-dates --dates-whitelist sniff` for automatic temporal type detection
  - Skips stats/schema regeneration when already up-to-date (checks file modification times)
  - Supports comprehensive CSV dialect coverage (delimiter, quoting, encoding)
  - Outputs Snappy-compressed Parquet with same file stem in working directory
  - Active guidance recommends Parquet for CSV files >10MB needing SQL queries
- **Server Instructions** - MCP Server Instructions sent during initialization for cross-tool workflow guidance
  - Covers workflow ordering, stats cache acceleration, file handling, tool composition, and memory limits
  - Injected into system prompt by compatible MCP clients (Claude Desktop, Claude Code, etc.)
  - Configurable via `QSV_MCP_SERVER_INSTRUCTIONS` env var or MCPB settings; empty = built-in defaults
- **Claude Plugin Layer** - Full Claude Code and Cowork integration via plugin manifest
  - Plugin manifest (`.claude-plugin/plugin.json`) for plugin discovery
  - MCP server config (`.mcp.json`) with server key `"qsv"` and `QSV_MCP_EXPOSE_ALL_TOOLS=true`
  - 5 slash commands: `/data-profile`, `/data-clean`, `/csv-query`, `/data-convert`, `/data-join`
  - 2 subagents: `data-analyst` (read-only analysis), `data-wrangler` (data transformation)
  - 3 domain knowledge skills: `csv-wrangling`, `data-quality`, `qsv-performance`
  - Commands include `allowed-tools` restrictions and `argument-hint` for natural invocation
  - Agents have explicit tool lists and reference domain knowledge skills
  - Skills are concise reference tables optimized for Claude scanning
- **Active Context-Sensitive Guidance** - Beyond passive tool descriptions, added structured guidance dictionaries
  - `WHEN_TO_USE_GUIDANCE`: Per-command hints for when to choose each tool (e.g., sqlp vs joinp, join vs joinp)
  - `COMMON_PATTERNS`: Workflow composition patterns (e.g., "index → stats → frequency", "convert to Parquet → query with DuckDB")
  - `ERROR_PREVENTION_HINTS`: Proactive warnings about common mistakes (OOM on large files, high-cardinality columns, subcommand requirements)
  - `COMPLEMENTARY_SERVERS`: Cross-server integration hints (Census MCP for geocoded US data)
  - Moarstats bivariate stats: guidance clarifies `--bivariate` writes to separate `<FILESTEM>.stats.bivariate.csv` file
- **Promoted `qsv_index` and `qsv_stats` to Core Tools** - Core tools increased from 7 to 10
  - `qsv_index`, `qsv_stats`, and `qsv_to_parquet` join the 7 existing core tools
  - Skill definitions properly loaded for deferred/core-only modes
  - These foundational tools are now always available without search discovery
- **Executor Timeout Handling** - `runQsv()` now enforces timeout on spawned qsv processes
  - Default timeout: 10 minutes via `config.operationTimeoutMs` (per-call override via `params.timeoutMs`)
  - Graceful termination: sends SIGTERM, then SIGKILL after 1 second
  - Returns exit code 124 (standard timeout code) with descriptive error message
  - Prevents hanging processes from blocking the MCP server
- **Gemini CLI Integration** - New documentation and support for Google's Gemini CLI
  - Integration guide at `docs/guides/GEMINI_CLI.md`
  - Template variable expansion support for `${PWD}`

### Changed
- **Reduced MCP Skills** - Excluded `behead`, `edit`, `flatten`, `pro`, and `snappy` commands from MCP skills generation, reducing from 60 to 55 skills
  - `behead` - trivial header removal, not needed for AI agent workflows
  - `edit` - interactive command not suitable for AI agent use
  - `flatten` - not suitable for AI agent use
  - `pro` - contains interactive/terminal-dependent subcommands (lens, workflow)
  - `snappy` - compression utility not needed for AI agents
- **Signal Termination Handling** - Proper signal handling in executor close handler
  - Maps signal termination to conventional exit codes (128 + signal number)
  - SIGTERM→143, SIGKILL→137, SIGINT→130, SIGHUP→129, SIGQUIT→131
  - Signal information added to stderr for debugging
- **Tool Guidance Updates** - Updated command categorization for better accuracy
  - `ALWAYS_FILE_COMMANDS` expanded to 25 commands: added slice, sample, template, geocode, schema, validate, diff, cat, transpose, partition, split, explode
  - `METADATA_COMMANDS` refined to 4 commands: count, headers, index, sniff
  - Enhanced guidance for sqlp (Parquet-first workflow for large CSVs), moarstats (bivariate stats), and geocode (FIPS examples)
- **Reduced Default Concurrent Operations** - `QSV_MCP_MAX_CONCURRENT_OPERATIONS` default changed from 10 to 1
- **Skill Version Sync** - Updated all 55 skill JSON files to version 16.0.0
- **Comprehensive Code Deduplication** - Refactored MCP server codebase removing 625 lines across 19 files
  - Consolidated 3 `runQsv` implementations into shared `runQsvSimple` with `onSpawn`/`onExit` callbacks
  - Merged 7 guidance tables into single `COMMAND_GUIDANCE` map
  - Decomposed `handleToolCall` into 4 focused functions
  - Extracted `errorResult`/`successResult` helpers and shared test helpers
  - Optimized SkillLoader with parallel reads and single-pass `getStats`
  - Replaced `Record<string, any>` with `unknown` for type safety
  - Removed dead code (`getFileContent`, `pipelineToShellScript`)

### Fixed
- **Cowork Compatibility via Plugin Mode** - MCP server now works correctly inside Claude Cowork's VM
  - Cowork mounts workspace at a VM-internal path via symlink, causing `realpathSync()` to resolve
    to a canonical path outside the configured `allowedDirs`, blocking all file operations
  - Added **plugin mode** detection: active when `CLAUDE_PLUGIN_ROOT` env var is set AND
    `MCPB_EXTENSION_MODE` is NOT enabled (i.e., running as a Claude Plugin, not a Desktop Extension)
  - In plugin mode, directories are auto-added to `allowedDirs` instead of being rejected,
    since Cowork/Code already provides filesystem isolation
  - Working directory defaults to `${PWD}` in plugin mode (vs `${DOWNLOADS}` otherwise)
  - `qsv_config` tool now shows three deployment modes: Plugin, Extension, and Legacy
- **isTemporaryFile regex fix** - Corrected regex to match 16-char hex filenames (not 36-char UUID)
- **Process tracking for shutdown** - Restored `Set<ChildProcess>` tracking so graceful shutdown kills active processes
- **Concurrency counter scope** - Moved counter to wrap entire `handleToolCall` body for accurate limiting
- **Version comparison with pre-release metadata** - `compareVersions` now strips pre-release/build metadata
- **Non-blocking path validation** - `isPathAllowed` uses `isDirectory` hint to avoid blocking `statSync`
- **Already-prefixed option keys** - `buildSkillExecParams` handles options that already have `--` prefix
- **Cross-device file moves** - Added EXDEV fallback for file moves across filesystem boundaries
- **Moarstats help guard** - Bivariate stats block now guarded for `--help` requests without `input_file`
- **Search guidance typo** - Fixed "to to" duplicate word in search tool guidance

### Security
- **Windows Cross-Drive Path Traversal Fix** - Security fix for path validation on Windows
  - `path.relative()` returns absolute path when comparing paths on different drives
  - Added `isAbsolute()` validation in `setWorkingDirectory()`, `resolvePath()`, and `validatePath()`
  - Prevents potential path traversal attacks across drive boundaries

### Removed
- **Removed client-detector.ts** - Eliminated client auto-detection that bypassed deferred loading
  - Previously, Claude clients were auto-detected and got all tools exposed immediately
  - Now deferred loading is consistent for ALL clients (~85% token reduction by default)
  - Users who want all tools immediately can set `QSV_MCP_EXPOSE_ALL_TOOLS=true`
  - Simplifies codebase and matches 15.3.0 documentation
- **Removed `qsv_data_profile` tool** - The tool produced ~60KB output for a 40-column file, filling Claude's context window and making it impractical
  - Tool guidance now recommends using `qsv stats --cardinality --stats-jsonl` instead
  - Removed profile cache manager and related configuration options
- **Removed `QSV_MCP_TIMEOUT_MS`** - Dead code with 5-minute default removed
  - Consolidated to single timeout source: `QSV_MCP_OPERATION_TIMEOUT_MS` (10-minute default)
  - Removed profile cache environment variables: `QSV_MCP_PROFILE_CACHE_ENABLED`, `QSV_MCP_PROFILE_CACHE_SIZE_MB`, `QSV_MCP_PROFILE_CACHE_TTL_MS`

### Dependencies
- Bumped `@modelcontextprotocol/sdk` from ^1.25.2 to ^1.26.0

## [15.3.0] - 2026-01-31

### Added
- **BM25 Search Integration** - Upgraded tool search from substring matching to BM25 relevance ranking
  - Uses `wink-bm25-text-search` library for proper probabilistic information retrieval
  - Field-weighted search: name (3x), category (2x), description (1x), examples (0.5x)
  - Text preprocessing with stemming, lowercasing, and negation propagation
  - Falls back to substring search if BM25 index not yet built
- **Deferred Tool Loading** - Implements Anthropic's Tool Search Tool pattern
  - Only 7 core tools loaded initially (reduces token usage ~85%)
  - Tools discovered via `qsv_search_tools` are dynamically added to tool list
  - Core tools (always loaded): `qsv_search_tools`, `qsv_config`, `qsv_set_working_dir`, `qsv_get_working_dir`, `qsv_list_files`, `qsv_pipeline`, `qsv_command`
  - Searched tools are marked as loaded and appear in subsequent ListTools responses

### Changed
- **SkillLoader BM25 Integration** - `search()` method now uses BM25 with limit parameter
  - `isBM25Indexed()` method to check if index is ready
  - BM25 index built automatically after `loadAll()`
- **handleSearchToolsCall** - Now marks found tools as loaded for deferred loading pattern

### Dependencies
- Added `wink-bm25-text-search` ^3.0.0 (MIT licensed) for BM25 search algorithm
- Added `wink-nlp-utils` ^2.1.0 for text preprocessing (stemming, tokenization)

## [15.2.0] - 2026-01-31

### Added
- **Dataset Profiling** - New `qsv_data_profile` tool profiles CSV files to help Claude make informed decisions
  - Uses `qsv frequency --toon` to generate column statistics in TOON format (token-efficient for LLMs)
  - Shows data types, cardinality, uniqueness_ratio, null counts, sparsity, sort_order, and value distributions
  - Helps Claude optimize operations across multiple commands:
    - `sqlp`: Choose optimal JOIN order, GROUP BY columns, WHERE selectivity
    - `joinp`: Determine optimal table order (smaller cardinality on right)
    - `frequency`: Identify high-cardinality columns (`<ALL_UNIQUE>`) to exclude
    - `dedup`: Check if uniqueness_ratio=1 (dedup would be a no-op for that key)
    - `sort`: Check if data is already sorted (sort_order field)
    - `pivotp`: Verify pivot column cardinality to avoid overly wide output
- **Profile Caching** - Configurable cache for TOON profiles to avoid redundant profiling
  - `QSV_MCP_PROFILE_CACHE_ENABLED` - Enable/disable caching (default: true)
  - `QSV_MCP_PROFILE_CACHE_SIZE_MB` - Maximum cache size (default: 10 MB)
  - `QSV_MCP_PROFILE_CACHE_TTL_MS` - Cache TTL (default: 1 hour)
- **Profile-Aware Tool Guidance** - Tool descriptions now include 📊 hints recommending `qsv_data_profile` first

### Changed
- **Documentation Reorganization** - Consolidated markdown files into organized `docs/` subdirectories
  - `docs/guides/` - User guides (QUICK_START, CLAUDE_CODE, DESKTOP_EXTENSION, FILESYSTEM_USAGE)
  - `docs/reference/` - Technical reference (AUTO_UPDATE, CI, SKILLS_API, UPDATE_SYSTEM)
  - `docs/desktop/` - Desktop extension docs (README-MCPB)
  - Root now contains only README.md, README-MCP.md, CLAUDE.md, and CHANGELOG.md

## [15.1.1] - 2026-01-28

### Changed
- **Skill Version Sync** - Updated all 56 skill JSON files to version 15.1.1 to align with MCP server release
- **Documentation Update** - Revised README-MCP.md installation instructions to reference latest MCPB version 15.1.1
- `sniff` is now properly recognized as a METADATA_COMMAND (i.e. uses stdout as they're short outputs)

## [15.1.0] - 2026-01-27

### Changed
- **Simplified Tool Guidance** - Removed redundant feature requirement hints (Polars, Luau) from tool descriptions
- **DuckDB Fallback** - Added guidance to use DuckDB as an alternative when sqlp encounters errors with complex queries
- **Expanded Error Prevention** - Added cat, dedup, sort, and searchset to commands with common mistake warnings
- **Streamlined Descriptions** - Removed verbose optimization hints that are now handled automatically

## [15.0.0] - 2026-01-26

### Added
- **Tool Search Support** - New `qsv_search_tools` for discovering qsv commands by keyword, category, or regex
- **Expose-All-Tools Mode** - Auto-detects Claude clients (Desktop, Code, Cowork) for automatic tool exposure
- **US Census MCP Integration** - Census MCP server awareness with integration guides
- **Agent-Understandable Examples** - Improved usage examples optimized for AI agent comprehension

### Changed
- **Token Optimization** - 66-76% reduction in tool description token usage
- **Streaming Executor** - Uses `spawn` instead of `execFileSync` for better output handling
- **Output Size Limits** - 50MB stdout limit prevents memory issues on large outputs
- **Added `cat` and `geocode` to common tools** - More robust subcommand handling

### Fixed
- Critical MCP server stability errors resolved
- Windows EPERM retry logic with exponential backoff for file locking
- Pass working directory to spawned qsv processes
- Cross-platform test runner improvements

### Removed
- qsv UI commands that are not useful for MCP server (e.g., `lens`, `prompt`)

## [14.1.0] - 2026-01-13

### Added
- Versioned MCPB packaging - `.mcpb` files now include version in filename (e.g., `qsv-mcp-server-14.1.0.mcpb`)
- Version display in packaging script output for better release tracking
- Retry logic with exponential backoff for Windows EPERM errors in cache save operations

### Changed
- Optimized tool definition token consumption by 66-76% through more concise descriptions
- Removed `test_file` fields from skill JSON files to reduce token usage
- MCPB packaging script now reads version from `package.json` automatically

### Fixed
- Resolved critical MCP server errors affecting stability
- Removed redundant semicolons in code for cleaner formatting
- Fixed flaky Windows CI test by adding retry logic for file rename operations (EPERM errors)

## [14.0.0] - 2026-01-12

### Added
- **MCP Desktop Extension (MCPB)** - One-click installation bundle for Claude Desktop
  - Auto-detection of qsv binary path
  - Template variable expansion (`$HOME`, `${HOME}`) in configuration paths
  - Cross-platform support (macOS, Windows, Linux)
  - User-friendly installation workflow
- **Enhanced Tool Descriptions** - Intelligent guidance system for optimal tool selection
  - 💡 USE WHEN - Specific use-case recommendations
  - 📋 COMMON PATTERNS - Workflow patterns and command combinations
  - ⚠️ CAUTION - Memory limits, file size constraints, feature requirements
  - 🚀 PERFORMANCE - Index acceleration tips and cache strategies
- **Stats Cache Auto-Generation** - Automatically enables `--stats-jsonl` when running stats command
- **Production CI/CD** - Comprehensive testing across Node.js 20, 22, 24 on all platforms
- **Update Checker** - Monitors qsv binary versions and notifies of available updates
- **qsv Command Detection** - Automatically detects available qsv commands at runtime
- **Total Memory Context** - Exposes system memory information to help LLM with planning
- **Graceful Shutdown** - Proper cleanup and shutdown handling for MCP server
- **MCP Prompts** - Welcome message and examples prompt for better user onboarding
- Comprehensive documentation in `README-MCPB.md` for Desktop Extension
- Claude Code integration guide in `CLAUDE_CODE.md`

### Changed
- **Token Optimization** - Concise descriptions extracted from README command table instead of verbose USAGE text
- **Security Enhancement** - Replaced `execSync` with `execFileSync` to prevent command injection attacks
- Improved file metadata display when listing supported files
- Enhanced installer experience with better validation and error messages
- Optimized for local file access with improved directory restrictions
- More robust output processing using temp output files and intelligent stdout handling
- Improved error handling and validation throughout codebase
- Updated to `@modelcontextprotocol/sdk` v1.25.2

### Fixed
- Cross-platform test runner compatibility for Node.js 20
- Windows-specific CI test issues
- Promise-based deduplication for metadata cache to prevent race conditions
- Lock file cleanup in converted file cache system
- Template variable expansion in config paths
- Manifest.json compliance with MCP Bundle spec v0.3
- Various issues identified in GitHub Copilot reviews

### Removed
- `applydp` skill (deprecated command)
- Old Phase implementation comments from LIFO converted file cache
- Unnecessary null byte checks
- Unreachable code after cache size cap implementation

## [13.0.0 to 14.0.0] - Development Phase

### Added
- **Initial MCP Server Implementation** - Full Model Context Protocol server for qsv
  - 25 MCP Tools: 20 common commands + 1 generic + 1 pipeline + 3 filesystem tools
  - Natural language interface for all 67 qsv commands
  - Local file access with directory restrictions via `QSV_MCP_ALLOWED_DIRS`
- **Skills Auto-Generation System**
  - Integrated `qsv --update-mcp-skill` command (requires `mcp` feature flag)
  - Automatic parsing of qsv USAGE text using qsv-docopt
  - Performance hint extraction (📇 indexed, 🤯 memory-intensive, 😣 proportional)
  - Concise descriptions from README command table
- **Multi-Format Support**
  - Native: CSV, TSV, SSV, Snappy-compressed variants
  - Auto-converted: Excel (.xls, .xlsx, .xlsm, .xlsb), OpenDocument (.ods), JSONL/NDJSON
  - Transparent file conversion with format auto-detection
- **Pipeline System** - Chain multiple qsv commands into efficient workflows
  - Automatic intermediate file management
  - Automatic indexing between steps
  - Atomic operations with rollback on failure
  - Performance optimization
- **Converted File Manager**
  - LIFO (Last In, First Out) cache for converted files
  - Configurable cache size (default 1GB, range 0.1-100GB)
  - Automatic cleanup of oldest conversions
  - Conversion metadata tracking
- **Filesystem Tools**
  - `qsv_list_files` - List tabular data files in directories
  - `qsv_set_working_dir` - Change working directory
  - `qsv_get_working_dir` - Get current working directory
- **Resource Limits** - DoS prevention and resource management
  - Operation timeout (default 120s)
  - Max file listings (default 1000)
  - Max pipeline steps (default 50)
  - Max concurrent operations (default 10)
- **20 Common Command Tools**
  - `qsv_select`, `qsv_stats`, `qsv_frequency`, `qsv_search`, `qsv_sort`
  - `qsv_dedup`, `qsv_join`, `qsv_count`, `qsv_headers`, `qsv_slice`
  - `qsv_apply`, `qsv_rename`, `qsv_schema`, `qsv_validate`, `qsv_sample`
  - `qsv_moarstats`, `qsv_index`, `qsv_template`, `qsv_diff`, `qsv_cat`
- **Generic Command Tool** - `qsv_command` for remaining 47 qsv commands
- Configuration system with environment variables
- Automated installation script (`scripts/install-mcp.js`)
- MCPB packaging script (`scripts/package-mcpb.js`)
- Comprehensive test suite with cross-platform support
- Example scripts demonstrating usage patterns

### Changed
- Architecture optimized for local file access
- Improved qsv binary discovery with timeout
- Enhanced error messages for better troubleshooting
- More robust parameter handling and validation
- Smart pagination per MCP specification
- Deterministic example generation with IndexMap
- Shell-safe argument formatting for CLI commands

### Security
- Command injection prevention via `execFileSync`
- Directory access control with path validation
- Symlink resolution and `..` traversal prevention
- Resource limits to prevent DoS attacks
- Binary trust verification requirements

### Documentation
- Added comprehensive `README-MCP.md` for MCP Server
- Added `CLAUDE.md` with development guidelines
- Added `FILESYSTEM_USAGE.md` for file operations
- Added `AUTO_UPDATE.md` for update system
- Added design documentation in `docs/design/`
- Added API documentation for Agent Skills

## Version Numbering

The qsv Agent Skills project follows qsv's version numbering scheme. Version numbers match the qsv release they are designed to work with:

- **14.0.x** - Compatible with qsv 14.0.x
- **13.0.x** - Compatible with qsv 13.0.x

Patch versions (x.y.Z) may be released independently for bug fixes and minor improvements to the MCP server without requiring a qsv update.

## Links

- [qsv Repository](https://github.com/dathere/qsv)
- [MCP Specification](https://modelcontextprotocol.io/)
- [Claude Desktop](https://claude.ai/desktop)
- [NPM Package](https://www.npmjs.com/package/@qsv/agent-skills) (future)

---

**Note**: This changelog covers changes to the qsv Agent Skills / MCP Server component specifically. For changes to the main qsv toolkit, see the [qsv CHANGELOG](../../CHANGELOG.md).
