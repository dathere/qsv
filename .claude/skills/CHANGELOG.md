# Changelog

All notable changes to the qsv Agent Skills (MCP Server) project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Executor Timeout Handling** - `runQsv()` now enforces timeout on spawned qsv processes
  - Default timeout: 10 minutes (configurable via `params.timeoutMs`)
  - Graceful termination: sends SIGTERM, then SIGKILL after 1 second
  - Returns exit code 124 (standard timeout code) with descriptive error message
  - Prevents hanging processes from blocking the MCP server

### Removed
- **Removed client-detector.ts** - Eliminated client auto-detection that bypassed deferred loading
  - Previously, Claude clients were auto-detected and got all tools exposed immediately
  - Now deferred loading is consistent for ALL clients (~85% token reduction by default)
  - Users who want all tools immediately can set `QSV_MCP_EXPOSE_ALL_TOOLS=true`
  - Simplifies codebase and matches 15.3.0 documentation

### Changed
- **Reduced MCP Skills** - Excluded `edit`, `flatten`, `pro`, and `snappy` commands from MCP skills generation, reducing from 60 to 56 skills
  - `edit` - interactive command not suitable for AI agent use
  - `flatten` - not suitable for AI agent use
  - `pro` - contains interactive/terminal-dependent subcommands (lens, workflow)
  - `snappy` - compression utility not needed for AI agents

### Removed
- **Removed `qsv_data_profile` tool** - The tool produced ~60KB output for a 40-column file, filling Claude's context window and making it impractical
  - Tool guidance now recommends using `qsv stats --cardinality --stats-jsonl` instead
  - Removed profile cache manager and related configuration options
  - Core tools reduced from 8 to 7

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
