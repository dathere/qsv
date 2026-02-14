# Environment Variables

| Variable | Description |
| --- | --- |
| `QSV_DOTENV_PATH` | The full pathname of the dotenv file to load, OVERRIDING existing environment variables. This takes precedence over any other dotenv files in the filesystem. Set to "<NONE>" to disable dotenv processing. |
| `QSV_DEFAULT_DELIMITER` | single ascii character to use as delimiter.  Overrides `--delimiter` option. Defaults to "," (comma) for CSV files & "\t" (tab) for TSV files when not set. Note that this will also set the delimiter for qsv's output to stdout.<br>However, using the `--output` option, regardless of this environment variable, will automatically change the delimiter used in the generated file based on the file extension - i.e. comma for `.csv`; tab for `.tsv` & `.tab` ; and semicolon for `.ssv` files |
| `QSV_SNIFF_DELIMITER` | if set, the delimiter is automatically detected. Overrides `QSV_DEFAULT_DELIMITER` & `--delimiter` option. Note that this does not work with stdin. |
| `QSV_SNIFF_PREAMBLE` | if set, qsv will attempt to sniff the number of preamble rows. |
| `QSV_NO_HEADERS` | if set, the first row will **NOT** be interpreted as headers. Supersedes `QSV_TOGGLE_HEADERS`. |
| `QSV_TOGGLE_HEADERS` | if set to `1`, toggles header setting - i.e. inverts qsv header behavior, with no headers being the default, & setting `--no-headers` will actually mean headers will not be ignored. |
| `QSV_ANTIMODES_LEN` | set to the maximum number of characters when listing "antimodes" in `stats`. Otherwise, the default is 100. Set to 0 to disable length limiting. |
| `QSV_AUTOINDEX_SIZE` | if set, specifies the minimum file size (in bytes) of a CSV file before an index is automatically created. Note that stale indices are automatically updated regardless of this setting. |
| `QSV_STATSCACHE_MODE` | Specifies how the stats cache is used by "smart" commands. Valid values are:<br />  * auto - use the stats cache if it's valid (the stats-jsonl file exists and is current) - default.<br />  * force - if the cache does not exist, create it by running stats.<br />  * none - do not use the stats cache, even if it exists. |
| `QSV_STATS_STRING_MAX_LENGTH` | Specifies the maximum string length for the "min"/"max" stats column. Some CSVs can have very long string columns that can cause other parsers to fail (e.g. Python's CSV reader can only accommodate 128kb strings by default) and when converting spatial formats like GeoJSON or Shapefile to CSV, the geometry column can easily be larger than this. When set, truncates the "min"/"max" columns of type String at the specified length and then appends an ellipsis (...). |
| `QSV_STATS_CHUNK_MEMORY_MB` | Controls memory-aware chunk sizing for parallel statistics processing. When set to a positive number, limits the maximum memory per chunk (in MB). When set to `0`, dynamically estimates chunk size by sampling records and available system memory. When unset, automatically enables dynamic sizing for non-streaming statistics (median, quartiles, modes, cardinality) and uses CPU-based chunking (dividing work by number of CPU cores) for streaming statistics only. Dynamic sizing means chunk sizes are determined based on available memory and sampled record sizes, allowing efficient processing of large files without exceeding system resources, while CPU-based chunking divides work evenly among CPU cores for streaming statistics. When set to -1, forces CPU-based chunking. This allows processing arbitrarily large files by creating smaller chunks that fit in available memory. |
| `QSV_FREQ_CHUNK_MEMORY_MB` | Controls memory-aware chunk sizing for frequency distribution processing. Set to 0 for dynamic sizing, or a positive number for a fixed memory limit per chunk, or -1 for CPU-based chunking (chunk size = num records / number of CPUs). This allows processing arbitrarily large files by creating smaller chunks that fit in available memory. |
| `QSV_FREQ_HIGH_CARD_THRESHOLD` | Absolute cardinality threshold for HIGH_CARDINALITY classification in the frequency cache (`--frequency-jsonl`). Columns with cardinality exceeding this value (or `QSV_FREQ_HIGH_CARD_PCT` of rowcount, whichever is smaller) get a single HIGH_CARDINALITY sentinel entry instead of full frequency data. Only used as a fallback when the `--high-card-threshold` CLI default (1000) is used. |
| `QSV_FREQ_HIGH_CARD_PCT` | Percentage of rowcount threshold for HIGH_CARDINALITY classification in the frequency cache (`--frequency-jsonl`). Columns with cardinality exceeding this percentage of the total row count (or `QSV_FREQ_HIGH_CARD_THRESHOLD`, whichever is smaller) get a single HIGH_CARDINALITY sentinel entry. Only used as a fallback when the `--high-card-pct` CLI default (90) is used. |
| `QSV_CACHE_DIR` | The directory to use for caching downloaded lookup_table resources using the `luau` qsv_register_lookup() helper function. |
| `QSV_CKAN_API` | The CKAN Action API endpoint to use with the `luau` qsv_register_lookup() helper function when using the "ckan://" scheme. |
| `QSV_CKAN_TOKEN`| The CKAN token to use with the `luau` qsv_register_lookup() helper function when using the "ckan://" scheme. Only required to access private resources. |
| `QSV_COMMENT_CHAR` | set to an ascii character. If set, any lines(including the header) that start with this character are ignored. |
| `QSV_MAX_JOBS` | number of jobs to use for multithreaded commands (currently `apply`, `applydp`, `dedup`, `diff`, `extsort`, `frequency`, `joinp`, `schema`, `snappy`, `sort`, `split`, `stats`, `to`, `tojsonl` & `validate`). If not set, max_jobs is set to the detected number of logical processors.  See [Multithreading](docs/PERFORMANCE.md#multithreading) for more info. |
| `QSV_NO_UPDATE` | if set, prohibit self-update version check for the latest qsv release published on GitHub. |
| `QSV_LLM_BASE_URL` | The LLM API URL to use with the `describegpt` command. |
| `QSV_LLM_APIKEY` | The API key of the supported LLM service to use with the `describegpt` command. |
| `QSV_LLM_MODEL` | The LLM Model to use with the `describegpt` command. |
| `QSV_DESCRIBEGPT_DB_ENGINE` | The database engine to use for SQL query execution in the `describegpt` command. When set to a path containing "duckdb" (case-insensitive), DuckDB will be used instead of the default Polars SQL engine. The environment variable value should be the fully qualified path to the DuckDB binary. Note that all loaded DuckDB extensions will be sent as additional context to the LLM to let it know what functions (even UDFs!) it can use in the SQL queries it generates. |
| `QSV_TEST_DESCRIBEGPT` | If set, enables `describegpt` command tests. Requires LM Studio with openai/gpt-oss-20b model loaded. |
| `QSV_OUTPUT_BOM` | if set, the output will have a Byte Order Mark (BOM) at the beginning. This is used to generate Excel-friendly CSVs on Windows. |
| `QSV_FORCE_COLOR` | if set, forces colorized output even when redirecting or running in CI. Used by the `color` command to override automatic color detection. |
| `QSV_THEME` | sets the color theme for the `color` command. Valid values are DARK or LIGHT (case-insensitive). If not set, the theme is automatically detected based on the terminal background color. |
| `QSV_TERMWIDTH` | overrides the detected terminal width for the `color` command. Must be a value between 1 and 1000. If not set, the terminal width is automatically detected or defaults to 80 when output is redirected. |
| `QSV_POLARS_FLOAT_PRECISION` | The precision to use when converting Polars-enabled formats (Avro,Arrow,Parquet,JSON,JSONL and gz,zlib & zst compressed files) to CSV. If set, this will also override the --float-precision option of the `sqlp` command. |
| `QSV_POLARS_DECIMAL_SCALE`  | The scale to use when using the Polars Decimal type. If not set, this defaults to 5. |
| `QSV_PREFER_DMY` | if set, date parsing will use DMY format. Otherwise, use MDY format (used with `datefmt`, `schema`, `sniff` & `stats` commands). |
| `QSV_REGEX_UNICODE` | if set, makes `search`, `searchset` & `replace` commands unicode-aware. For increased performance, these commands are not unicode-aware by default & will ignore unicode values when matching & will abort when unicode characters are used in the regex. Note that the `apply operations regex_replace` operation is always unicode-aware. |
| `QSV_RDR_BUFFER_CAPACITY` | reader buffer size (default - 128k (bytes): 131072) |
| `QSV_SKIP_FORMAT_CHECK` | if set, skips mime-type checking of input files. Set this when optimizing for performance and when encountering false positives as a format check involves scanning the input file to infer the mime-type/format. |
| `QSV_STATS_SEPARATOR` | the separator to use to delimit multiple MODE/ANTIMODE and PERCENTILE values. |
| `QSV_WTR_BUFFER_CAPACITY` | writer buffer size (default - 512k (bytes): 524288) |
| `QSV_FREEMEMORY_HEADROOM_PCT` | the percentage of free available memory required when running qsv in "non-streaming" mode (i.e. the entire file needs to be loaded into memory). If the incoming file is greater than the available memory after the headroom is subtracted, qsv will not proceed. Set to 0 to skip memory check. See [Memory Management](#memory-management) for more info. (default: (percent) 20 ) |
| `QSV_MEMORY_CHECK` | if set, check if input file size < AVAILABLE memory - HEADROOM (CONSERVATIVE mode) when running in "non-streaming" mode. Otherwise, qsv will only check if the input file size < TOTAL memory - HEADROOM (NORMAL mode). This is done to prevent Out-of-Memory errors. See [Memory Management](#memory-management) for more info. |
| `QSV_LOG_LEVEL` | desired level (default - off; `error`, `warn`, `info`, `trace`, `debug`). |
| `QSV_LOG_DIR` | when logging is enabled, the directory where the log files will be stored. If the specified directory does not exist, qsv will attempt to create it. If not set, the log files are created in the directory where qsv was started. See [Logging](docs/Logging.md#logging) for more info. |
| `QSV_LOG_UNBUFFERED` | if set, log messages are written directly to disk, without buffering. Otherwise, log messages are buffered before being written to the log file (8k buffer, flushing every second). See [flexi_logger](https://docs.rs/flexi_logger/latest/flexi_logger/enum.WriteMode.html) for details. |
| `QSV_PROGRESSBAR` | if set, enable the --progressbar option on the `apply`, `fetch`, `fetchpost`, `foreach`, `luau`, `py`, `replace`, `search`, `searchset`, `sortcheck` & `validate` commands.  |
| `QSV_DISKCACHE_TTL_SECS` | set time-to-live of diskcache cached values (default (seconds): 2419200 (28 days)). |
| `QSV_DISKCACHE_TTL_REFRESH`| if set, enables cache hits to refresh TTL of diskcache cached values. |
| `QSV_REDIS_CONNSTR` | the `fetch` command can use [Redis](https://redis.io/) to cache responses. Set to connect to the desired Redis instance. (default: `redis:127.0.0.1:6379/1`). For more info on valid Redis connection string formats, click [here](https://docs.rs/redis/latest/redis/#connection-parameters). |
| `QSV_FP_REDIS_CONNSTR` | the `fetchpost` command can also use Redis to cache responses (default: `redis:127.0.0.1:6379/2`). |
| `QSV_DG_REDIS_CONNSTR` | the `describegpt` command can also use Redis to cache responses (default: `redis:127.0.0.1:6379/3`). |
| `QSV_REDIS_MAX_POOL_SIZE` | the maximum Redis connection pool size. (default: 20). |
| `QSV_REDIS_TTL_SECS` | set time-to-live of Redis cached values (default (seconds): 2419200 (28 days)). |
| `QSV_REDIS_TTL_REFRESH`| if set, enables cache hits to refresh TTL of Redis cached values. |
| `QSV_TIMEOUT`| for commands with a --timeout option (`fetch`, `fetchpost`, `luau`, `sniff` and `validate`), the number of seconds before a web request times out (default: 30). |
| `QSV_USER_AGENT`| the user-agent to use for web requests. When specifying a custom user agent. It supports the following variables - $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME and $QSV_KIND. Try to conform to the [IETF RFC 72321 standard](https://tools.ietf.org/html/rfc7231#section-5.5.3). See [here](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent) for examples.<br>(default: $QSV_BIN_NAME/$QSV_VERSION ($QSV_TARGET; $QSV_KIND; https://github.com/dathere/qsv) - e.g.<br>`qsv/0.105.0 (x86_64-unknown-linux; prebuilt; https://github.com/dathere/qsv)`).|
| `QSV_GEOIP2_FILENAME` | the filename of the GeoIP2 database to use for the `geocode` command. (default: `GeoLite2-City.mmdb`) |
| `QSV_GEOCODE_INDEX_FILENAME` | The filename of the Geonames index file to use for the `geocode` command. If not set, the default index file for that qsv version is downloaded and saved to `QSV_CACHE_DIR`. Set this only if you have a custom Geonames index file. |

Several dependencies also have environment variables that influence qsv's performance & behavior:

* Memory Allocator   
  When incorporating qsv into a data pipeline that runs in batch mode, particularly with very large CSV files using qsv commands that load entire CSV files into memory, you can fine tune qsv's memory allocator run-time behavior using the environment variables for the allocator you're using:

  * [mimalloc](https://github.com/microsoft/mimalloc#environment-options)
    
* Network Access ([reqwest](https://docs.rs/reqwest/latest/reqwest/))   
  qsv uses reqwest and will honor [proxy settings](https://docs.rs/reqwest/latest/reqwest/index.html#proxies) set through the `HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY` & `NO_PROXY` environment variables.

* Polars   
  qsv uses [polars](https://github.com/pola.rs/polars) for several commands - currently `count`, `joinp`, `pivotp` and `sqlp`. Polars has its own set of environment variables that can be set to influence its behavior (see [here](https://github.com/pola-rs/polars/blob/dd1fc86b65ae39b741f46edc6da01d024bed50b6/crates/polars/src/lib.rs#L366-L408)). The most relevant ones are:

  * `POLARS_VERBOSE` - if set to 1, polars will output logging messages to stderr.
  * `POLARS_PANIC_ON_ERR` - if set to 1, panics on polars-related errors, instead of returning an error.
  * `POLARS_BACKTRACE_IN_ERR` - if set to 1, includes backtrace in polars-related error messages.
  
> ℹ️ **NOTE:** To get a list of all active qsv-relevant environment variables, run `qsv --envlist`.
Relevant env vars are defined as anything that starts with `QSV_`, `MIMALLOC_` & the proxy variables listed above.

## MCP Server Environment Variables

The qsv MCP (Model Context Protocol) Server exposes qsv's capabilities to AI agents like Claude. It can be used as a standalone MCP server or as a Claude Desktop Extension (MCPB).

These environment variables configure the MCP server behavior:

### Core Configuration

| Variable | Description | Default |
| --- | --- | --- |
| `QSV_MCP_BIN_PATH` | Full path to the qsv binary. If not set, auto-detects from PATH and common installation locations (`/usr/local/bin/qsv`, `/opt/homebrew/bin/qsv`, `~/.cargo/bin/qsv`, etc.). | Auto-detect |
| `QSV_MCP_WORKING_DIR` | Default working directory for file operations. Supports template variables like `${HOME}`, `${DOWNLOADS}`, `${DOCUMENTS}`. | `${DOWNLOADS}` |
| `QSV_MCP_ALLOWED_DIRS` | Additional directories where qsv can access files (colon-separated on Unix, semicolon on Windows, or JSON array). File access is restricted to working directory and these directories only. | Empty (working dir only) |

### Performance Tuning

| Variable | Description | Default | Range |
| --- | --- | --- | --- |
| `QSV_MCP_TIMEOUT_MS` | Command timeout in milliseconds. Used by Desktop extension mode. | 300,000 (5 min) | 10,000 - 3,600,000 |
| `QSV_MCP_OPERATION_TIMEOUT_MS` | Operation timeout in milliseconds. Used by legacy MCP mode. | 120,000 (2 min) | 1,000 - 1,800,000 |
| `QSV_MCP_MAX_OUTPUT_SIZE` | Maximum output size in bytes before results are automatically saved to disk. | 52,428,800 (50 MB) | 1,048,576 - 104,857,600 |
| `QSV_MCP_CONVERTED_LIFO_SIZE_GB` | Maximum size for the converted file cache (Excel→CSV, JSONL→CSV) in GB. Uses LIFO eviction. | 1.0 | 0.1 - 100.0 |
| `QSV_MCP_MAX_FILES_PER_LISTING` | Maximum number of files returned in a single directory listing. | 1,000 | 1 - 100,000 |
| `QSV_MCP_MAX_PIPELINE_STEPS` | Maximum number of steps allowed in a single pipeline execution. | 50 | 1 - 1,000 |
| `QSV_MCP_MAX_CONCURRENT_OPERATIONS` | Maximum number of concurrent qsv operations. | 10 | 1 - 100 |
| `QSV_MCP_MAX_EXAMPLES` | Maximum number of examples to include in MCP tool descriptions. Set to 0 to disable examples. | 5 | 0 - 20 |

### Update Checking

| Variable | Description | Default |
| --- | --- | --- |
| `QSV_MCP_CHECK_UPDATES_ON_STARTUP` | Check for new qsv releases on GitHub at startup. | `true` |
| `QSV_MCP_NOTIFY_UPDATES` | Display update notifications in server logs when new versions are available. | `true` |
| `QSV_MCP_AUTO_REGENERATE_SKILLS` | Automatically regenerate skill definitions when qsv version changes. Runs `qsv --update-mcp-skills`. | `false` |
| `QSV_MCP_GITHUB_REPO` | GitHub repository for update checks. | `dathere/qsv` |

### Desktop Extension Mode

| Variable | Description | Default |
| --- | --- | --- |
| `MCPB_EXTENSION_MODE` | Set automatically by Claude Desktop when running as an extension. Enables stricter validation (requires fully qualified qsv path, version >= 13.0.0). | `false` |

### Template Variables

The following template variables can be used in `QSV_MCP_WORKING_DIR` and `QSV_MCP_ALLOWED_DIRS`:

| Variable | Expands To |
| --- | --- |
| `${HOME}` | User's home directory |
| `${USERPROFILE}` | User's home directory (Windows alias) |
| `${DESKTOP}` | User's Desktop folder |
| `${DOCUMENTS}` | User's Documents folder |
| `${DOWNLOADS}` | User's Downloads folder |
| `${TEMP}` / `${TMPDIR}` | System temporary directory |

### Example Configuration

**Legacy MCP mode** (claude_desktop_config.json):
```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": ["/path/to/.claude/skills/dist/mcp-server.js"],
      "env": {
        "QSV_MCP_BIN_PATH": "/usr/local/bin/qsv",
        "QSV_MCP_WORKING_DIR": "${DOWNLOADS}",
        "QSV_MCP_ALLOWED_DIRS": "${DOCUMENTS}:${DESKTOP}",
        "QSV_MCP_TIMEOUT_MS": "300000",
        "QSV_MCP_CHECK_UPDATES_ON_STARTUP": "true"
      }
    }
  }
}
```

**Desktop Extension mode**: Configuration is managed through Claude Desktop Settings → Extensions → qsv, which sets these environment variables automatically.

## .env File Support
qsv supports the use of `.env` files to set environment variables. The `.env` file is a simple text file that contains key-value pairs, one per line. 

It processes `.env` files as follows:

* Upon invocation, qsv will check if the `QSV_DOTENV_PATH` environment variable is set. If it is, it will look for the file specified by the variable. If the file is found, it will be processed.
* If the `QSV_DOTENV_PATH` environment variable is not set, qsv will look for a file named `.env` in the current working directory. If one is found, it will be processed.
* If no `.env` file is not found in the current working directory, qsv will next look for an `.env` file with the same filestem as the binary in the directory where the binary is (e.g. if `qsv`/`qsvlite`/`qsvdp` is in `/usr/local/bin`, it will look for `/usr/local/bin/qsv.env`, `/usr/local/bin/qsvlite.env` or `/usr/local/bin/qsvdp.env` respectively).
* If no `.env` files are found, qsv will proceed with its default settings and the current environment variables, which may include "QSV_" variables.

When processing `.env` files, qsv will:
* overwrite any existing environment variables with the same name
* where multiple declarations of the same variable exist, the last one will be used
* ignore any lines that start with `#` (comments)

To facilitate the use of `.env` files, a [`dotenv.template`](../dotenv.template) file is included in the qsv distribution. This file contains all the environment variables that qsv recognizes, along with their default values. Copy the template to a file named '.env' and modify it to suit your needs.
