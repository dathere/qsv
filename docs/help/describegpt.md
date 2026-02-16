# describegpt

> Infer a ["neuro-symbolic"](https://en.wikipedia.org/wiki/Neuro-symbolic_AI) Data Dictionary, Description & Tags or ask questions about a CSV with a [configurable, Mini Jinja prompt file](../../resources/describegpt_defaults.toml), using any [OpenAI API](https://platform.openai.com/docs/introduction)-compatible LLM, including local LLMs via [Ollama](https://ollama.com), [Jan](https://jan.ai) & [LM Studio](https://lmstudio.ai/). (e.g. [Markdown](../describegpt/nyc311-describegpt.md), [JSON](../describegpt/nyc311-describegpt.json), [TOON](../describegpt/nyc311-describegpt.toon), [Everything](../describegpt/nyc311-describegpt-everything.md), [Spanish](../describegpt/nyc311-describegpt-spanish.md), [Mandarin](../describegpt/nyc311-describegpt-mandarin.md), [Controlled Tags](../describegpt/nyc311-describegpt-tagvocab.md); [--prompt "What are the top 10 complaint types by community board & borough by year?"](../describegpt/nyc311-describegpt-prompt.md) - [deterministic, hallucination-free SQL RAG result](../describegpt/nyc311-describegpt-prompt.csv); [iterative, session-based SQL RAG refinement](../describegpt/allegheny_discussion3.md) - [refined SQL RAG result](../describegpt/mostexpensive6.csv))

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/describegpt.rs](https://github.com/dathere/qsv/blob/master/src/cmd/describegpt.rs)** | [🌐](TableOfContents.md#legend "has web-aware options.")[🤖](TableOfContents.md#legend "command uses Natural Language Processing or Generative AI.")[🪄](TableOfContents.md#legend "\"automagical\" commands that uses stats and/or frequency tables to work \"smarter\" & \"faster\".")[🗃️](TableOfContents.md#legend "Limited Extended input support.")[📚](TableOfContents.md#legend "has lookup table support, enabling runtime \"lookups\" against local or remote reference CSVs.")[⛩️](TableOfContents.md#legend "uses Mini Jinja template engine.") [![CKAN](../images/ckan.png)](TableOfContents.md#legend "has CKAN-aware integration options.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Data Analysis/Inferencing Options](#data-analysis/inferencing-options) | [Dictionary Options](#dictionary-options) | [Tag Options](#tag-options) | [Stats/Frequency Options](#stats/frequency-options) | [Custom Prompt Options](#custom-prompt-options) | [LLM API Options](#llm-api-options) | [Caching Options](#caching-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Create a "neuro-procedural" Data Dictionary and/or infer Description & Tags about a Dataset
using an OpenAI API-compatible Large Language Model (LLM).

It does this by compiling Summary Statistics & a Frequency Distribution of the Dataset,
and then prompting the LLM with detailed, configurable, Mini Jinja-templated prompts with
these extended statistical context.

The Data Dictionary is "neuro-procedural" as it uses a hybrid approach. It's primarily populated
deterministically using Summary Statistics & Frequency Distribution data, and only the human-friendly
Label & Description are populated by the "neural network" LLM using the same statistical context.

CHAT MODE:
You can also use the --prompt option to ask a natural language question about the Dataset.

If the question can be answered by solely using the Dataset's Summary Statistics and
Frequency Distribution data, the LLM will return the answer directly.

CHAT SQL RETRIEVAL-AUGMENTED GENERATION (RAG) SUB-MODE:
If the question cannot be answered using the Dataset's Summary Statistics & Frequency Distribution,
it will first create a Data Dictionary and a small random sample (default: 100 rows) of the Dataset
and provide it to the LLM as additional context to help it generate a SQL query that DETERMINISTICALLY
answers the natural language question.

Two SQL dialects are currently supported - DuckDB (highly recommended) & Polars. If the
QSV_DESCRIBEGPT_DB_ENGINE environment variable is set to the absolute path of the DuckDB binary,
DuckDB will be used to answer the question. Otherwise, if the "polars" feature is enabled,
Polars SQL will be used.

If neither DuckDB nor Polars is available, the SQL query will be returned in a Markdown code block,
along with the reasoning behind the query.

Even in "SQL RAG" mode, though the SQL query is guaranteed to be deterministic, the query itself
may not be correct. In the event of a SQL query execution failure, run the same --prompt with
the --fresh option to request the LLM to generate a new SQL query.

When using DuckDB, all loaded DuckDB extensions will be sent as additional context to the LLM to let
it know what functions (even UDFs!) it can use in the SQL queries it generates. If you want a
specific function or technique to be used in the SQL query, mention it in the prompt.

SUPPORTED MODELS & LLM PROVIDERS:
OpenAI's open-weights gpt-oss model (both 20b and 120b variants) was used during development &
is recommended for most use cases.
It was also tested with OpenAI, TogetherAI, OpenRouter and Google Gemini cloud providers.
For Gemini, use the base URL "<https://generativelanguage.googleapis.com/v1beta/openai">.
Local LLMs tested include Ollama, Jan and LM Studio.

NOTE: LLMs are prone to inaccurate information being produced. Verify output results before using them.

CACHING:
As LLM inferencing takes time and can be expensive, describegpt caches the LLM inferencing results
in a either a disk cache (default) or a Redis cache. It does so by calculating the BLAKE3 hash of the
input file and using it as the primary cache key along with the prompt type, model and other parameters
as required.

The default disk cache is stored in the ~/.qsv/cache/describegpt directory with a default TTL of 28 days
and cache hits NOT refreshing an existing cached value's TTL.
Adjust the QSV_DISKCACHE_TTL_SECS & QSV_DISKCACHE_TTL_REFRESH env vars to change disk cache settings.

Alternatively a Redis cache can be used instead of the disk cache. This is especially useful if you want
to share the cache across the network with other users or computers.
The Redis cache is stored in database 3 by default with a TTL of 28 days and cache hits NOT refreshing
an existing cached value's TTL. Adjust the QSV_DG_REDIS_CONNSTR, QSV_REDIS_MAX_POOL_SIZE,
QSV_REDIS_TTL_SECONDS & QSV_REDIS_TTL_REFRESH env vars to change Redis cache settings.


<a name="examples"></a>

## Examples [↩](#nav)

> Generate a Data Dictionary, Description & Tags of data.csv using default OpenAI gpt-oss-20b model
> (replace <API_KEY> with your OpenAI API key)

```console
qsv describegpt data.csv --api-key <API_KEY> --all
```

> Generate a Data Dictionary of data.csv using the DeepSeek R1:14b model on a local Ollama instance

```console
qsv describegpt data.csv -u http://localhost:11434/v1 --model deepseek-r1:14b --dictionary
```

> Ask questions about the sample NYC 311 dataset using LM Studio with the default gpt-oss-20b model.
> Questions that can be answered using the Summary Statistics & Frequency Distribution of the dataset.

```console
qsv describegpt NYC_311.csv --prompt "What is the most common complaint?"
```

> Ask detailed natural language questions that require SQL queries and auto-invoke SQL RAG mode
> Generate a DuckDB SQL query to answer the question

```console
QSV_DESCRIBEGPT_DB_ENGINE=/path/to/duckdb \
qsv describegpt NYC_311.csv -p "What's the breakdown of complaint types by borough descending order?"
```

> Prompt requires a natural language query. Convert query to SQL using the LLM and save results to
> a file with the --sql-results option.  If generated SQL query runs successfully,
> the file is "results.csv". Otherwise, it is "results.sql".

```console
qsv describegpt NYC_311.csv -p "Aggregate complaint types by community board" --sql-results results
```

> Cache Dictionary, Description & Tags inference results using the Redis cache instead of the disk cache

```console
qsv describegpt data.csv --all --redis-cache
```

> Get fresh Description & Tags inference results from the LLM and refresh disk cache entries for both

```console
qsv describegpt data.csv --description --tags --fresh
```

> Get fresh inference results from the LLM and refresh the Redis cache entries for all three

```console
qsv describegpt data.csv --all --redis-cache --fresh
```

> Forget a cached response for data.csv's data dictionary if it exists and then exit

```console
qsv describegpt data.csv --dictionary --forget
```

> Flush/Remove ALL cached entries in the disk cache

```console
qsv describegpt --flush-cache
```

> Flush/Remove ALL cached entries in the Redis cache

```console
qsv describegpt --redis-cache --flush-cache
```

> Generate Data Dictionary but exclude ID columns from frequency analysis to reduce overhead

```console
qsv describegpt data.csv --dictionary --freq-options "--select '!id,!uuid' --limit 20"
```

> Generate Data Dictionary, Description & Tags but reduce frequency context
> by showing only top 5 values per field

```console
qsv describegpt data.csv --all --freq-options "--limit 5"
```

> Generate Description using weighted frequencies with ascending sort

```console
qsv describegpt data.csv --description --freq-options "--limit 50 --asc --weight count_column"
```

> Generate a Data Dictionary, Description & Tags using a previously compiled stats CSV file and
> frequency CSV file instead of running the stats and frequency commands

```console
qsv describegpt data.csv --all --stats-options "file:my_stats.csv" --freq-options "file:my_freq.csv"
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_describegpt.rs).

For more detailed info on how describegpt works and how to prepare a prompt file,
see <https://github.com/dathere/qsv/blob/master/docs/Describegpt.md>

<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv describegpt [options] [<input>]
qsv describegpt (--redis-cache) (--flush-cache)
qsv describegpt --help
```

<a name="data-analysis/inferencing-options"></a>

## Data Analysis/Inferencing Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--dictionary`&nbsp; | flag | Create a Data Dictionary using a hybrid "neuro-procedural" pipeline - i.e. the Dictionary is populated deterministically using Summary Statistics and Frequency Distribution data, and only the human-friendly Label and Description are populated by the LLM using the same statistical context. |  |
| &nbsp;`--description`&nbsp; | flag | Infer a general Description of the dataset based on detailed statistical context. An Attribution signature is embedded in the Description. |  |
| &nbsp;`--tags`&nbsp; | flag | Infer Tags that categorize the dataset based on detailed statistical context. Useful for grouping datasets and filtering. |  |
| &nbsp;`-A,`<br>`--all`&nbsp; | flag | Shortcut for --dictionary --description --tags. |  |

<a name="dictionary-options"></a>

## Dictionary Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--num-examples`&nbsp; | string | The number of Example values to include in the dictionary. | `5` |
| &nbsp;`--truncate-str`&nbsp; | string | The maximum length of an Example value in the dictionary. An ellipsis is appended to the truncated value. If zero, no truncation is performed. | `25` |
| &nbsp;`--addl-cols`&nbsp; | flag | Add additional columns to the dictionary from the Summary Statistics. |  |
| &nbsp;`--addl-cols-list`&nbsp; | string | A comma-separated list of additional stats columns to add to the dictionary. The columns must be present in the Summary Statistics. If the columns are not present in the Summary Statistics or already in the dictionary, they will be ignored. | `sort_order, sortiness, mean, median, mad, stddev, variance, cv` |

<a name="tag-options"></a>

## Tag Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--num-tags`&nbsp; | string | The maximum number of tags to infer when the --tags option is used. Maximum allowed value is 50. | `10` |
| &nbsp;`--tag-vocab`&nbsp; | string | The CSV file containing the tag vocabulary to use for inferring tags. If no tag vocabulary file is provided, the model will use free-form tags. Supports local files, remote URLs (http/https), CKAN resources (ckan://), and dathere:// scheme. Remote resources are cached locally. The CSV file must have two columns with headers: first column is the tag, second column is the description. Note that qsvlite only supports local files. |  |
| &nbsp;`--cache-dir`&nbsp; | string | The directory to use for caching downloaded tag vocabulary resources. If the directory does not exist, qsv will attempt to create it. If the QSV_CACHE_DIR envvar is set, it will be used instead. | `~/.qsv-cache` |
| &nbsp;`--ckan-api`&nbsp; | string | The URL of the CKAN API to use for downloading tag vocabulary resources with the "ckan://" scheme. If the QSV_CKAN_API envvar is set, it will be used instead. | `https://data.dathere.com/api/3/action` |
| &nbsp;`--ckan-token`&nbsp; | string | The CKAN API token to use. Only required if downloading private resources. If the QSV_CKAN_TOKEN envvar is set, it will be used instead. |  |

<a name="stats/frequency-options"></a>

## Stats/Frequency Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--stats-options`&nbsp; | string | Options for the stats command used to generate summary statistics. If it starts with "file:" prefix, the statistics are read from the specified CSV file instead of running the stats command. e.g. "file:my_custom_stats.csv" | `--infer-dates --infer-boolean --mad --quartiles --percentiles --force --stats-jsonl` |
| &nbsp;`--freq-options`&nbsp; | string | Options for the frequency command used to generate frequency distributions. You can use this to exclude certain variable types from frequency analysis (e.g., --select '!id,!uuid'), limit results differently per use case, or control output format. If --limit is specified here, it takes precedence over --enum-threshold. If it starts with "file:" prefix, the frequency data is read from the specified CSV file instead of running the frequency command. e.g. "file:my_custom_frequency.csv" | `--rank-strategy dense` |
| &nbsp;`--enum-threshold`&nbsp; | string | The threshold for compiling Enumerations with the frequency command before bucketing other unique values into the "Other" category. This is a convenience shortcut for --freq-options --limit <n>. If --freq-options contains --limit, this flag is ignored. | `10` |

<a name="custom-prompt-options"></a>

## Custom Prompt Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-p,`<br>`--prompt`&nbsp; | string | Custom prompt to answer questions about the dataset. The prompt will be answered based on the dataset's Summary Statistics, Frequency data & Data Dictionary. If the prompt CANNOT be answered by looking at these metadata, a SQL query will be generated to answer the question. If the "polars" or the "QSV_DESCRIBEGPT_DB_ENGINE" environment variable is set & the `--sql-results` option is used, the SQL query will be automatically executed and its results returned. Otherwise, the SQL query will be returned along with the reasoning behind it. If it starts with "file:" prefix, the prompt is read from the file specified. e.g. "file:my_long_prompt.txt" |  |
| &nbsp;`--sql-results`&nbsp; | string | The file to save the SQL query results to. Only valid if the --prompt option is used & the "polars" or the "QSV_DESCRIBEGPT_DB_ENGINE" environment variable is set. If the SQL query executes successfully, the results will be saved with a ".csv" extension. Otherwise, it will be saved with a ".sql" extension so the user can inspect why it failed and modify it. |  |
| &nbsp;`--prompt-file`&nbsp; | string | The configurable TOML file containing prompts to use for inferencing. If no file is provided, default prompts will be used. The prompt file uses the Mini Jinja template engine (<https://docs.rs/minijinja>) See <https://github.com/dathere/qsv/blob/master/resources/describegpt_defaults.toml> |  |
| &nbsp;`--sample-size`&nbsp; | string | The number of rows to randomly sample from the input file for the sample data. Uses the INDEXED sampling method with the qsv sample command. | `100` |
| &nbsp;`--fewshot-examples`&nbsp; | flag | By default, few-shot examples are NOT included in the LLM prompt when generating SQL queries. When this option is set, few-shot examples in the default prompt file are included. Though this will increase the quality of the generated SQL, it comes at a cost - increased LLM API call cost in terms of tokens and execution time. See <https://en.wikipedia.org/wiki/Prompt_engineering> for more info. |  |
| &nbsp;`--session`&nbsp; | string | Enable stateful session mode for iterative SQL RAG refinement. The session name is the file path of the markdown file where session messages will be stored. When used with --prompt, subsequent queries in the same session will refine the baseline SQL query. SQL query results (10-row sample) and errors are automatically included in subsequent messages for context. |  |
| &nbsp;`--session-len`&nbsp; | string | Maximum number of recent messages to keep in session context before summarizing older messages. Only used when --session is specified. | `10` |

<a name="llm-api-options"></a>

## LLM API Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-u,`<br>`--base-url`&nbsp; | string | The LLM API URL. Supports APIs & local LLMs compatible with the OpenAI API specification. Some common base URLs: OpenAI: <https://api.openai.com/v1> Gemini: <https://generativelanguage.googleapis.com/v1beta/openai> TogetherAI: <https://api.together.ai/v1> | `http://localhost:1234/v1` |
| &nbsp;`-m,`<br>`--model`&nbsp; | string | The model to use for inferencing. If set, takes precedence over the QSV_LLM_MODEL environment variable. | `openai/gpt-oss-20b` |
| &nbsp;`--language`&nbsp; | string | The output language/dialect to use for the response. (e.g., "Spanish", "French", "Hindi", "Mandarin", "Italian", "Castilian", "Franglais", "Taglish", "Pig Latin", "Valley Girl", "Pirate", "Shakespearean English", "Chavacano", "Gen Z", "Yoda", etc.) |  |
| &nbsp;`--addl-props`&nbsp; | string | Additional model properties to pass to the LLM chat/completion API. Various models support different properties beyond the standard ones. For instance, gpt-oss-20b supports the "reasoning_effort" property. e.g. to set the "reasoning_effort" property to "high" & "temperature" to 0.5, use '{"reasoning_effort": "high", "temperature": 0.5}' |  |
| &nbsp;`-k,`<br>`--api-key`&nbsp; | string | The API key to use. If set, takes precedence over the QSV_LLM_APIKEY envvar. Required when the base URL is not localhost. Set to NONE to suppress sending the API key. |  |
| &nbsp;`-t,`<br>`--max-tokens`&nbsp; | string | Limits the number of generated tokens in the output. Set to 0 to disable token limits. If the --base-url is localhost, indicating a local LLM, the default is automatically set to 0. | `10000` |
| &nbsp;`--timeout`&nbsp; | string | Timeout for completions in seconds. If 0, no timeout is used. | `300` |
| &nbsp;`--user-agent`&nbsp; | string | Specify custom user agent. It supports the following variables - $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME, $QSV_KIND and $QSV_COMMAND. Try to follow the syntax here - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent> |  |
| &nbsp;`--export-prompt`&nbsp; | string | Export the default prompts to the specified file that can be used with the --prompt-file option. The file will be saved with a .toml extension. If the file already exists, it will be overwritten. It will exit after exporting the prompts. |  |

<a name="caching-options"></a>

## Caching Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--no-cache`&nbsp; | flag | Disable default disk cache. |  |
| &nbsp;`--disk-cache-dir`&nbsp; | string | The directory <dir> to store the disk cache. Note that if the directory does not exist, it will be created. If the directory exists, it will be used as is, and will not be flushed. This option allows you to maintain several disk caches for different describegpt jobs (e.g. one for a data portal, another for internal data exchange, etc.) | `~/.qsv/cache/describegpt` |
| &nbsp;`--redis-cache`&nbsp; | flag | Use Redis instead of the default disk cache to cache LLM completions. It connects to "redis://127.0.0.1:6379/3" by default, with a connection pool size of 20, with a TTL of 28 days, and cache hits NOT refreshing an existing cached value's TTL. This option automatically disables the disk cache. |  |
| &nbsp;`--fresh`&nbsp; | flag | Send a fresh request to the LLM API, refreshing a cached response if it exists. When a --prompt SQL query fails, you can also use this option to request the LLM to generate a new SQL query. |  |
| &nbsp;`--forget`&nbsp; | flag | Remove a cached response if it exists and then exit. |  |
| &nbsp;`--flush-cache`&nbsp; | flag | Flush the current cache entries on startup. WARNING: This operation is irreversible. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`--format`&nbsp; | string | Output format: Markdown, TSV, JSON, or TOON. TOON is a compact, human-readable encoding of the JSON data model for LLM prompts. See <https://toonformat.dev/> for more info. | `Markdown` |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. If --format is set to TSV, separate files will be created for each prompt type with the pattern {filestem}.{kind}.tsv (e.g., output.dictionary.tsv, output.tags.tsv). |  |
| &nbsp;`-q,`<br>`--quiet`&nbsp; | flag | Do not print status messages to stderr. |  |

---
**Source:** [`src/cmd/describegpt.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/describegpt.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
