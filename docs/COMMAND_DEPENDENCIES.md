# QSV Command Dependencies

This document identifies QSV commands that have dependencies on the outputs of other commands (such as stats, index, frequency, etc.).

## 1. Dependency on `index` (created via `qsv index`)
Many commands in `qsv` are "index-aware." While most can function without an index by performing a sequential scan, they will automatically detect and use a `.idx` file if it exists to provide significant performance improvements (random access, faster counting, etc.).

*   **`reverse`**: Specifically uses the index to reverse rows in a streaming fashion without loading the entire file into memory. Without an index, it must load the entire CSV into RAM.
*   **`count`**: Provides an O(1) row count if an index is present.
*   **`slice`**: Uses the index to jump directly to a specific row offset.
*   **`sample`**: Uses the index to perform efficient random sampling (the "indexed" sampling method).
*   **`split`**: Uses the index to efficiently slice pieces of the file.
*   **`luau`**: Can trigger "Random Access Mode" if a script uses the `_INDEX` variable, which requires an index file. It also provides a `qsv_autoindex()` helper to create one on the fly.
*   **`search` / `searchset`**: Uses the index to speed up searches when combined with specific options.
*   **`replace`**: Uses the index to parallelize replace operations.
*   **`extsort`**: Requires an index when sorting CSV files (as opposed to text mode).
*   **`pragmastat`**: Uses the index for row count estimation.
*   **`stats` / `frequency` / `moarstats`**: Can use the index to parallelize processing or resume/speed up calculations.
*   **`schema`**: Uses the index to parallelize the internal frequency pass.
*   **`sniff`**: Uses the index to sample records without a full scan.
*   **`clean`**: Structurally validates `.idx` files against their source before deleting them.

> [!NOTE]
> Beyond the commands listed above, `util::count_rows()` short-circuits on an index whenever one
> is present, so *any* command that needs a row count (for a progress bar, for chunking, or for
> its own logic) is index-accelerated for free. The list above covers commands with a distinct,
> deliberate index code path.

## 2. Dependency on `stats` (created via `qsv stats`)
The following "smart" commands (🪄) use the stats cache (`stats.csv.data.jsonl`) to optimize processing. They use the `get_stats_records()` utility function or run `stats` as a subprocess.

*   **`schema`**: Reuses the `stats.csv.data.jsonl` cache file if it exists and is current (generated with `--cardinality` and `--infer-dates`). If not present, it internally runs `stats` to generate this data.
*   **`describegpt`**: Uses summary statistics to provide context to the LLM. It can explicitly read an existing stats file via the `--stats-options "file:<path>"` option. Runs `stats` as a subprocess.
*   **`frequency`**: Uses `get_stats_records()` to optimize processing by detecting column cardinality and unique columns.
*   **`sample`**: Uses `get_stats_records()` for smart sampling decisions.
*   **`joinp`**: Uses `get_stats_records()` with `StatsMode::PolarsSchema` for Polars schema inference.
*   **`pivotp`**: Uses `get_stats_records()` with `StatsMode::FrequencyForceStats` for smart aggregation.
*   **`sqlp`**: Indirectly uses the stats cache via `util::infer_polars_schema()` for data type inference.
*   **`tojsonl`**: Uses the stats cache via `infer_schema_from_stats` for JSON data type inference.
*   **`moarstats`**: Reads `.stats.csv` files to add extended statistics.
*   **`scoresql`**: Analyzes SQL queries against stats, moarstats, and frequency caches to produce performance scores with optimization suggestions.
*   **`pragmastat`**: Reads the stats JSONL cache directly to auto-filter non-numeric columns and support Date/DateTime columns.
*   **`synthesize`**: Uses the stats cache to model each column's distribution when generating statistically-faithful synthetic CSVs.
*   **`viz`**: Uses `get_stats_records()` with `StatsMode::ProfileSchema` to drive `viz smart` chart selection, and also reads the stats sidecars directly.
*   **`profile`**: Uses `get_stats_records()` with `StatsMode::ProfileSchema` for metadata extraction.
*   **`clean`**: Reads stats cache metadata (the `.stats.csv.json` sidecar) to verify a cache belongs to its source before deleting it.

The following commands consume an *existing* stats cache opportunistically via
`get_stats_records_readonly()` — unlike the commands above, they never generate one, and simply
proceed without it when absent:

*   **`extsort`**: Reads cached stats to inform its external sort strategy.
*   **`sortcheck`**: Reads cached stats to short-circuit sortedness checks.

## 3. Dependency on `frequency` (created via `qsv frequency`)
*   **`schema`**: Uses frequency distributions internally to identify "low cardinality" columns and automatically build `enum` constraints for the generated JSON Schema.
*   **`describegpt`**: Uses frequency distributions to provide data distribution context to the LLM. It can read an existing frequency file via the `--freq-options "file:<path>"` option.
*   **`synthesize`**: Uses frequency distributions to reproduce realistic value distributions in the generated synthetic CSVs.
*   **`viz`**: Reads the frequency cache (via `frequency::frequency_cache_path`) to drive `viz smart` chart selection.
*   **`scoresql`**: Analyzes queries against the frequency cache as part of its scoring.
*   **`clean`**: Reads frequency cache metadata to verify a cache belongs to its source before deleting it.

## 4. Dependency on `schema` (created via `qsv schema`)
*   **`validate`**: Primarily depends on a `.schema.json` file (produced by `schema`) to validate CSV records.
*   **`sqlp`, `joinp`, `pivotp`, `to`**: These Polars-based commands automatically look for a `.pschema.json` file (created via `qsv schema --polars`). If found, they use it to bypass schema inference, ensuring correct data types (like `Decimal` or `Date`) and optimizing query planning.

## 5. Cross-Command Data Dependencies
*   **`validate` (via `dynamicEnum`)**: Can depend on **any other CSV** to serve as a lookup table for validating values in a specific column.
*   **`join` / `joinp`**: Naturally depend on the output of other commands if you are joining a primary file against a processed "reference" file.
*   **`sqlp`, `schema`, `to`**: All three run **`qsv sniff` as a subprocess** (via `util::infer_polars_schema()`) to infer a Polars schema when no `.pschema.json` is available.
*   **`sqlp` / `luau`**: These are general-purpose "glue" commands that often serve as the end of a pipeline, consuming outputs from `stats`, `schema`, or filtered CSVs created by other commands.
