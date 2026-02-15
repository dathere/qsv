# qsv Command Help

> Auto-generated from qsv command USAGE text. See [README](../../README.md) for full documentation.

| Command | Description |
| --- | --- |
| [apply](apply.md)<br>📇🚀🧠🤖🔣👆 | Apply series of string, date, math & currency transformations to given CSV column/s. It also has some basic NLP functions (similarity, sentiment analysis, profanity, eudex, language & name gender) detection. |
| [applydp](applydp.md)<br>📇🚀🔣👆 ![CKAN](../images/ckan.png) | applydp is a slimmed-down version of `apply` with only Datapusher+ relevant subcommands/operations (`qsvdp` binary variant only). |
| [behead](behead.md) | Drop headers from a CSV. |
| [cat](cat.md)<br>🗄️ | Concatenate CSV files by row or by column. |
| [clipboard](clipboard.md)<br>🖥️ | Provide input from the clipboard or save output to the clipboard. |
| [color](color.md)<br>🐻‍❄️🖥️ | Outputs tabular data as a pretty, colorized table that always fits into the terminal. Apart from CSV and its dialects, Arrow, Avro/IPC, Parquet, JSON array & JSONL formats are supported with the "polars" feature. |
| [count](count.md)<br>📇🏎️🐻‍❄️ | Count the rows and optionally compile record width statistics of a CSV file. (11.87 seconds for a 15gb, 27m row NYC 311 dataset without an index. Instantaneous with an index.) If the `polars` feature is enabled, uses Polars' multithreaded, mem-mapped CSV reader for fast counts even without an index |
| [datefmt](datefmt.md)<br>📇🚀👆 | Formats recognized date fields (19 formats recognized) to a specified date format using strftime date format specifiers. |
| [dedup](dedup.md)<br>🤯🚀👆 | Remove duplicate rows (See also `extdedup`, `extsort`, `sort` & `sortcheck` commands). |
| [describegpt](describegpt.md)<br>🌐🤖🪄🗃️📚⛩️ ![CKAN](../images/ckan.png) | Infer a "neuro-symbolic" Data Dictionary, Description & Tags or ask questions about a CSV with a configurable, Mini Jinja prompt file, using any OpenAI API-compatible LLM, including local LLMs via Ollama, Jan & LM Studio. (e.g. Markdown, JSON, TOON, Everything, Spanish, Mandarin, Controlled Tags; --prompt "What are the top 10 complaint types by community board & borough by year?" - deterministic, hallucination-free SQL RAG result; iterative, session-based SQL RAG refinement - refined SQL RAG result) |
| [diff](diff.md)<br>🚀🪄 | Find the difference between two CSVs with ludicrous speed! e.g. _compare two CSVs with 1M rows x 9 columns in under 600ms!_ |
| [edit](edit.md) | Replace the value of a cell specified by its row and column. |
| [enum](enum.md)<br>👆 | Add a new column enumerating rows by adding a column of incremental or uuid identifiers. Can also be used to copy a column or fill a new column with a constant value. |
| [excel](excel.md)<br>🚀 | Exports a specified Excel/ODS sheet to a CSV file. |
| [exclude](exclude.md)<br>📇👆 | Removes a set of CSV data from another set based on the specified columns. |
| [explode](explode.md)<br>🔣👆 | Explode rows into multiple ones by splitting a column value based on the given separator. |
| [extdedup](extdedup.md)<br>👆 | Remove duplicate rows from an arbitrarily large CSV/text file using a memory-mapped, on-disk hash table. Unlike the `dedup` command, this command does not load the entire file into memory nor does it sort the deduped file. |
| [extsort](extsort.md)<br>🚀📇👆 | Sort an arbitrarily large CSV/text file using a multithreaded external merge sort algorithm. |
| [fetch](fetch.md)<br>📇🧠🌐 | Send/Fetch data to/from web services for every row using **HTTP Get**. Comes with HTTP/2 adaptive flow control, jaq JSON query language support, dynamic throttling (RateLimit) & caching with available persistent caching using Redis or a disk-cache. |
| [fetchpost](fetchpost.md)<br>📇🧠🌐⛩️ | Similar to `fetch`, but uses **HTTP Post** (HTTP GET vs POST methods). Supports HTML form (application/x-www-form-urlencoded), JSON (application/json) and custom content types - with the ability to render payloads using CSV data using the Mini Jinja template engine. |
| [fill](fill.md)<br>👆 | Fill empty values. |
| [fixlengths](fixlengths.md) | Force a CSV to have same-length records by either padding or truncating them. |
| [flatten](flatten.md) | A flattened view of CSV records. Useful for viewing one record at a time. e.g. `qsv slice -i 5 data.csv \| qsv flatten`. |
| [fmt](fmt.md) | Reformat a CSV with different delimiters, record terminators or quoting rules. (Supports ASCII delimited data.) |
| [foreach](foreach.md)<br>📇 | Execute a shell command once per record in a given CSV file. |
| [frequency](frequency.md)<br>📇😣🏎️👆🪄![Luau](../images/luau.png) | Build frequency distribution tables of each column. Uses multithreading to go faster if an index is present (Examples: CSV JSON TOON). |
| [geocode](geocode.md)<br>📇🧠🌐🚀🔣👆🌎 | Geocodes a location against an updatable local copy of the Geonames cities & the Maxmind GeoLite2 databases. With caching and multi-threading, it geocodes up to 360,000 records/sec! |
| [geoconvert](geoconvert.md)<br>🌎 | Convert between various spatial formats and CSV/SVG including GeoJSON, SHP, and more. |
| [headers](headers.md)<br>🗄️ | Show the headers of a CSV. Or show the intersection of all headers between many CSV files. |
| [index](index.md) | Create an index for a CSV. This is very quick (even the 15gb, 28m row NYC 311 dataset takes all of 14 seconds to index) & provides constant time indexing/random access into the CSV. With an index, `count`, `sample` & `slice` work instantaneously; random access mode is enabled in `luau`; and multithreading is enabled for the `frequency`, `split`, `stats`, `schema` & `tojsonl` commands. |
| [input](input.md) | Read CSV data with special commenting, quoting, trimming, line-skipping & non-UTF8 encoding handling rules. Typically used to "normalize" a CSV for further processing with other qsv commands. |
| [join](join.md)<br>😣👆 | Inner, outer, right, cross, anti & semi joins. Automatically creates a simple, in-memory hash index to make it fast. |
| [joinp](joinp.md)<br>🚀🐻‍❄️🪄 | Inner, outer, right, cross, anti, semi, non-equi & asof joins using the Pola.rs engine. Unlike the `join` command, `joinp` can process files larger than RAM, is multithreaded, has join key validation, a maintain row order option, pre and post-join filtering, join keys unicode normalization, supports "special" non-equi joins and asof joins (which is particularly useful for time series data) & its output columns can be coalesced. |
| [json](json.md)<br>👆 | Convert JSON array to CSV. |
| [jsonl](jsonl.md)<br>🚀🔣 | Convert newline-delimited JSON (JSONL/NDJSON) to CSV. See `tojsonl` command to convert CSV to JSONL. |
| [lens](lens.md)<br>🐻‍❄️🖥️ | Interactively view, search & filter tabular data files using the csvlens engine. Apart from CSV and its dialects, Arrow, Avro/IPC, Parquet, JSON array & JSONL formats are supported with the "polars" feature. |
| [luau](luau.md)<br>📇🌐🔣📚 ![CKAN](../images/ckan.png) | Create multiple new computed columns, filter rows, compute aggregations and build complex data pipelines by executing a Luau 0.708 expression/script for every row of a CSV file (sequential mode), or using random access with an index (random access mode). Can process a single Luau expression or full-fledged data-wrangling scripts using lookup tables with discrete BEGIN, MAIN and END sections. It is not just another qsv command, it is qsv's Domain-specific Language (DSL) with numerous qsv-specific helper functions to build production data pipelines. |
| [moarstats](moarstats.md)<br>📇🏎️ | Add dozens of additional statistics, including extended outlier, robust & bivariate statistics to an existing stats CSV file. (example). |
| [partition](partition.md)<br>👆 | Partition a CSV based on a column value. |
| [pivotp](pivotp.md)<br>🚀🐻‍❄️🪄 | Pivot CSV data. Features "smart" aggregation auto-selection based on data type & stats. |
| [pragmastat](pragmastat.md)<br>🤯 | Compute pragmatic statistics using the Pragmastat library. |
| [pro](pro.md) | Interact with the qsv pro API. |
| [prompt](prompt.md)<br>🐻‍❄️🖥️ | Open a file dialog to either pick a file as input or save output to a file. |
| [pseudo](pseudo.md)<br>🔣👆 | Pseudonymise the value of the given column by replacing them with an incremental identifier. |
| [py](py.md)<br>📇🔣 | Create a new computed column or filter rows by evaluating a Python expression on every row of a CSV file. Python's f-strings is particularly useful for extended formatting, with the ability to evaluate Python expressions as well. Requires Python 3.8 or greater. |
| [rename](rename.md) | Rename the columns of a CSV efficiently. |
| [replace](replace.md)<br>📇👆🏎️ | Replace CSV data using a regex. Applies the regex to each field individually. |
| [reverse](reverse.md)<br>📇🤯 | Reverse order of rows in a CSV. Unlike the `sort --reverse` command, it preserves the order of rows with the same key. If an index is present, it works with constant memory. Otherwise, it will load all the data into memory. |
| [safenames](safenames.md)<br>![CKAN](../images/ckan.png) | Modify headers of a CSV to only have "safe" names - guaranteed "database-ready"/"CKAN-ready" names. |
| [sample](sample.md)<br>📇🌐🏎️ | Randomly draw rows (with optional seed) from a CSV using seven different sampling methods - reservoir (default), indexed, bernoulli, systematic, stratified, weighted & cluster sampling. Supports sampling from CSVs on remote URLs. |
| [schema](schema.md)<br>📇😣🏎️👆🪄🐻‍❄️ | Infer either a JSON Schema Validation Draft 2020-12 (Example) or Polars Schema (Example) from CSV data. In JSON Schema Validation mode, it produces a `.schema.json` file replete with inferred data type & domain/range validation rules derived from `stats`. Uses multithreading to go faster if an index is present. See `validate` command to use the generated JSON Schema to validate if similar CSVs comply with the schema. With the `--polars` option, it produces a `.pschema.json` file that all polars commands (`sqlp`, `joinp` & `pivotp`) use to determine the data type of each column & to optimize performance. Both schemas are editable and can be fine-tuned. For JSON Schema, to refine the inferred validation rules. For Polars Schema, to change the inferred Polars data types. |
| [search](search.md)<br>📇🏎️👆 | Run a regex over a CSV. Applies the regex to selected fields & shows only matching rows. |
| [searchset](searchset.md)<br>📇🏎️👆 | _Run multiple regexes over a CSV in a single pass._ Applies the regexes to each field individually & shows only matching rows. |
| [select](select.md)<br>👆 | Select, re-order, reverse, duplicate or drop columns. |
| [slice](slice.md)<br>📇🏎️🗃️ | Slice rows from any part of a CSV. When an index is present, this only has to parse the rows in the slice (instead of all rows leading up to the start of the slice). |
| [snappy](snappy.md)<br>🚀🌐 | Does streaming compression/decompression of the input using Google's Snappy framing format (more info). |
| [sniff](sniff.md)<br>📇🌐🤖 ![CKAN](../images/ckan.png) | Quickly sniff & infer CSV metadata (delimiter, header row, preamble rows, quote character, flexible, is_utf8, average record length, number of records, content length & estimated number of records if sniffing a CSV on a URL, number of fields, field names & data types). It is also a general mime type detector. |
| [sort](sort.md)<br>🚀🤯👆 | Sorts CSV data in lexicographical, natural, numerical, reverse, unique or random (with optional seed) order (Also see `extsort` & `sortcheck` commands). |
| [sortcheck](sortcheck.md)<br>📇👆 | Check if a CSV is sorted. With the --json options, also retrieve record count, sort breaks & duplicate count. |
| [split](split.md)<br>📇🏎️ | Split one CSV file into many CSV files. It can split by number of rows, number of chunks or file size. Uses multithreading to go faster if an index is present when splitting by rows or chunks. |
| [sqlp](sqlp.md)<br>📇🚀🐻‍❄️🗄️🪄 | Run Polars SQL (a PostgreSQL dialect) queries against several CSVs, Parquet, JSONL and Arrow files - converting queries to blazing-fast Polars LazyFrame expressions, processing larger than memory CSV files. Query results can be saved in CSV, JSON, JSONL, Parquet, Apache Arrow IPC and Apache Avro formats. |
| [stats](stats.md)<br>📇🤯🏎️👆🪄 | Compute summary statistics (sum, min/max/range, sort order/sortiness, min/max/sum/avg length, mean, standard error of the mean (SEM), geometric/harmonic means, stddev, variance, Coefficient of Variation (CV), nullcount, max precision, sparsity, quartiles, Interquartile Range (IQR), lower/upper fences, skewness, median, mode/s, antimode/s, cardinality & uniqueness ratio) & make GUARANTEED data type inferences (Null, String, Float, Integer, Date, DateTime, Boolean) for each column in a CSV (Example - more info). Uses multithreading to go faster if an index is present (with an index, can compile "streaming" stats on NYC's 311 data (15gb, 28m rows) in less than 7.3 seconds!). |
| [table](table.md)<br>🤯 | Align output of a CSV using elastic tabstops for viewing; or to create an "aligned TSV" file or Fixed Width Format file. To interactively view a CSV, use the `lens` command. |
| [template](template.md)<br>📇🚀🔣📚⛩️![CKAN](../images/ckan.png) | Renders a template using CSV data with the Mini Jinja template engine (Example). |
| [to](to.md)<br>🚀🗄️ | Convert CSV files to PostgreSQL, SQLite, Excel (XLSX), LibreOffice Calc (ODS) and Data Package. |
| [tojsonl](tojsonl.md)<br>📇😣🚀🔣🪄🗃️ | Smartly converts CSV to a newline-delimited JSON (JSONL/NDJSON). By scanning the CSV first, it "smartly" infers the appropriate JSON data type for each column. See `jsonl` command to convert JSONL to CSV. |
| [transpose](transpose.md)<br>🤯👆 | Transpose rows/columns of a CSV. |
| [validate](validate.md)<br>📇🚀🌐📚🗄️![CKAN](../images/ckan.png) | Validate CSV data _blazingly-fast_ using JSON Schema Validation (Draft 2020-12) (e.g. _up to 780,031 rows/second_[^1] using NYC's 311 schema generated by the `schema` command) & put invalid records into a separate file along with a detailed validation error report. Supports several custom JSON Schema formats & keywords: * `currency` custom format with ISO-4217 validation * `dynamicEnum` custom keyword that supports enum validation against a CSV on the filesystem or a URL (http/https/ckan & dathere URL schemes supported) * `uniqueCombinedWith` custom keyword to validate uniqueness across multiple columns for composite key validation. If no JSON schema file is provided, validates if a CSV conforms to the RFC 4180 standard and is UTF-8 encoded. |

---

### Legend

: enabled by a [feature flag](#feature-flags).  
📇: uses an index when available.  
🤯: loads entire CSV into memory, though `dedup`, `stats` & `transpose` have "streaming" modes as well.  
😣: uses additional memory proportional to the cardinality of the columns in the CSV.  
🧠: expensive operations are memoized with available inter-session Redis/Disk caching for fetch commands.  
🗄️: [Extended input support](#extended-input-support).  
🗃️: [Limited Extended input support](#limited-extended-input-support).  
🐻‍❄️: command powered/accelerated by [![polars 0.53.0:c5a142d](https://img.shields.io/badge/polars-0.53.0:c5a142d-blue?logo=polars  
)](https://github.com/pola-rs/polars/releases/tag/rs-0.53.0) vectorized query engine.  
🤖: command uses Natural Language Processing or Generative AI.  
🏎️: multithreaded and/or faster when an index (📇) is available.  
🚀: multithreaded even without an index.  
![CKAN](../images/ckan.png) : has [CKAN](https://ckan.org)-aware integration options.  
🌐: has web-aware options.  
🔣: requires UTF-8 encoded input.  
👆: has powerful column selector support. See [`select`](https://github.com/dathere/qsv/blob/master/src/cmd/select.rs#L2) for syntax.  
🪄: "automagical" commands that uses stats and/or frequency tables to work "smarter" & "faster".  
📚: has lookup table support, enabling runtime "lookups" against local or remote reference CSVs.  
🌎: has geospatial capabilities.  
⛩️: uses [Mini Jinja](https://docs.rs/minijinja/latest/minijinja/) template engine.  
![Luau](../images/luau.png) : uses [Luau](https://luau.org/) [0.708](https://github.com/Roblox/luau/releases/tag/0.708) as an embedded scripting [DSL](https://en.wikipedia.org/wiki/Domain-specific_language).  
🖥️: part of the User Interface (UI) feature group  

---
**[README](../../README.md)**
