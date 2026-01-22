# QSV Command Dependencies

This document identifies QSV commands that have dependencies on the outputs of other commands (such as stats, index, frequency, etc.).

## 1. Dependency on `index` (created via `qsv index`)
Many commands in `qsv` are "index-aware." While most can function without an index by performing a sequential scan, they will automatically detect and use a `.idx` file if it exists to provide significant performance improvements (random access, faster counting, etc.).

*   **`reverse`**: Specifically uses the index to reverse rows in a streaming fashion without loading the entire file into memory. Without an index, it must load the entire CSV into RAM.
*   **`count`**: Provides an O(1) row count if an index is present.
*   **`slice`**: Uses the index to jump directly to a specific row offset.
*   **`sample`**: Uses the index to perform efficient random sampling (the "indexed" sampling method).
*   **`split` / `partition`**: Uses the index to efficiently slice pieces of the file.
*   **`luau`**: Can trigger "Random Access Mode" if a script uses the `_INDEX` variable, which requires an index file. It also provides a `qsv_autoindex()` helper to create one on the fly.
*   **`search` / `searchset`**: Uses the index to speed up searches when combined with specific options.
*   **`stats` / `frequency` / `moarstats`**: Can use the index to parallelize processing or resume/speed up calculations.

## 2. Dependency on `stats` (created via `qsv stats`)
*   **`schema`**: Reuses the `stats.csv.data.jsonl` cache file if it exists and is current (generated with `--cardinality` and `--infer-dates`). If not present, it internally runs `stats` to generate this data.
*   **`describegpt`**: Uses summary statistics to provide context to the LLM. It can explicitly read an existing stats file via the `--stats-options "file:<path>"` option.

## 3. Dependency on `frequency` (created via `qsv frequency`)
*   **`schema`**: Uses frequency distributions internally to identify "low cardinality" columns and automatically build `enum` constraints for the generated JSON Schema.
*   **`describegpt`**: Uses frequency distributions to provide data distribution context to the LLM. It can read an existing frequency file via the `--freq-options "file:<path>"` option.

## 4. Dependency on `schema` (created via `qsv schema`)
*   **`validate`**: Primarily depends on a `.schema.json` file (produced by `schema`) to validate CSV records.
*   **`sqlp`, `joinp`, `pivotp`**: These Polars-based commands automatically look for a `.pschema.json` file (created via `qsv schema --polars`). If found, they use it to bypass schema inference, ensuring correct data types (like `Decimal` or `Date`) and optimizing query planning.

## 5. Cross-Command Data Dependencies
*   **`validate` (via `dynamicEnum`)**: Can depend on **any other CSV** to serve as a lookup table for validating values in a specific column.
*   **`join` / `joinp`**: Naturally depend on the output of other commands if you are joining a primary file against a processed "reference" file.
*   **`sqlp` / `luau`**: These are general-purpose "glue" commands that often serve as the end of a pipeline, consuming outputs from `stats`, `schema`, or filtered CSVs created by other commands.
