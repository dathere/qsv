# `describegpt` command

`describegpt` allows users to infer extended metadata about a CSV dataset using large language models, in particular GPT chat completion models from OpenAI's API, Ollama, or an API compatible with the OpenAI API specification such as Jan.

`describegpt` uses `qsv stats` and `qsv frequency` in the background to provide context to the model. It does not send the entire dataset to the model. This "zero-copy" approach is not only ideal for reducing costs (as detailed metadata, even for very large dataset is "tiny" in comparison) and increasing performance, it also protects the privacy of sensitive data.

Note that this command uses LLMs for inferencing and is therefore prone to inaccurate information being produced. Verify output results before using them.

## Basic Inference Options

`describegpt` provides several options to infer different types of metadata about your dataset:

- `--dictionary` - Creates a Data Dictionary using a hybrid "neuro-procedural" pipeline. The Data Dictionary is deterministically populated using Summary Statistics and Frequency Distribution data, and only the human-friendly Label and Description are populated by the LLM using the same statistical context.

- `--description` - Infers a general Description of the dataset based on detailed statistical context.

- `--tags` - Infers Tags that categorize the dataset based on detailed statistical context. Useful for grouping datasets and filtering.

- `-A, --all` - Shortcut for `--dictionary --description --tags`. Generates all three types of metadata in a single run.

## Tag Options

When using the `--tags` option, you can control how tags are inferred:

### `--num-tags <n>`

The maximum number of tags to infer when the `--tags` option is used. The value must be between 1 and 50. The default is 10.

### `--tag-vocab <file>`

The CSV file containing the tag vocabulary to use for inferring tags. If no tag vocabulary file is provided, the model will use free-form tags.

The CSV file must have two columns with headers:
- First column: tag name
- Second column: tag description

Example CSV format:

```csv
tag,description
alphabetical_data,Data containing letters or alphabetical characters
numerical_data,Data containing numbers or numerical values
test_data,Sample or test data used for demonstration
```

The tag vocabulary supports multiple sources:
- **Local files**: Provide a path to a local CSV file
- **Remote URLs**: HTTP/HTTPS URLs pointing to CSV files (e.g., `https://example.com/tags.csv`)
- **CKAN resources**: Use the `ckan://` scheme to reference CKAN dataset resources (e.g., `ckan://dataset-id/resource-id`)
- **dathere:// scheme**: Reference GitHub raw content using the `dathere://` scheme

Remote resources (HTTP/HTTPS, CKAN, and dathere://) are automatically cached locally to avoid repeated downloads. The cache TTL is 1 hour by default.

**Note**: `qsvlite` only supports local files. Remote URLs, CKAN resources, and dathere:// schemes are not available in `qsvlite`.

### `--cache-dir <dir>`

The directory to use for caching downloaded tag vocabulary resources. If the directory does not exist, qsv will attempt to create it. The default is `~/.qsv-cache`.

If the `QSV_CACHE_DIR` environment variable is set, it will be used instead of this option.

### `--ckan-api <url>`

The URL of the CKAN API to use for downloading tag vocabulary resources with the `ckan://` scheme. The default is `https://data.dathere.com/api/3/action`.

If the `QSV_CKAN_API` environment variable is set, it will be used instead of this option.

### `--ckan-token <token>`

The CKAN API token to use. Only required if downloading private CKAN resources. If the `QSV_CKAN_TOKEN` environment variable is set, it will be used instead of this option.

### Examples

Using tag vocabulary with a local CSV file:

```bash
qsv describegpt data.csv --tags --tag-vocab tags.csv
```

Using tag vocabulary with a remote URL:

```bash
qsv describegpt data.csv --tags --tag-vocab https://example.com/tags.csv
```

Using tag vocabulary with CKAN resources:

```bash
qsv describegpt data.csv --tags --tag-vocab ckan://dataset-id/resource-id --ckan-api https://data.example.com/api/3/action
```

Using tag vocabulary with a custom cache directory:

```bash
qsv describegpt data.csv --tags --tag-vocab https://example.com/tags.csv --cache-dir /tmp/qsv-cache
```

Limiting the number of tags inferred:

```bash
qsv describegpt data.csv --tags --tag-vocab tags.csv --num-tags 5
```

## QSV_LLM_APIKEY

When working with a cloud-based LLM, `describegpt` requires an API key. You can set this key using the `QSV_LLM_APIKEY` environment variable. Check [/docs/ENVIRONMENT_VARIABLES.md](/docs/ENVIRONMENT_VARIABLES.md) for more info.

When working with a Local LLM (i.e. if `--base-url` or `QSV_LLM_BASE_URL` contains "localhost"), `describegpt` will NOT require an API key.

## `--api-key <key>`

You can also specify your API key directly in your CLI using the `--api-key` option.

Note that if you already have `QSV_LLM_APIKEY` set as an environment variable and it is not empty, this environment variable will override your given flag.

## `--json`

You can use the `--json` option to expect JSON output. This is useful for piping the output to other commands for example.

Note that **the `--json` option does not indicate to your prompt that you want to generate JSON output based on your dataset**. It instead ensures the command output is in JSON format. You must specify this within your prompts, such as adding the phrase "in JSON format" to your prompt.

If the prompt output is not in valid JSON format but the `--json` option is specified, the command will generate a default error JSON output printed to `stdout`, such as the following:

```json
{
    "option": {
        "error": "Invalid JSON output for option."
    }
}
```

You may often see this error when `--max-tokens` is set too low and therefore the output is incomplete.

The invalid output will be printed in `stderr`.

Note that `--json` may not be used alongside `--jsonl`, nor may they both be set to true in a prompt file at the same time. This will result in an error.

## `--jsonl`

Similar to `--json`, you can use the `--jsonl` option to expect [JSON Lines](https://jsonlines.org/) output.

If you use `--output` with `--jsonl`, the output will be written to a new file if it doesn't exist and any lines after the first will be appended to the file. If the file already exists, the output will be appended to the file. Each inference option (`--dictionary`, `--description`, `--tags`) will be written to a new line in the file.

If you use `--prompt-file` with `--jsonl`, the prompt name and timestamp will also be included in the JSONL output for each inference option.

Note that **the `--jsonl` option does not indicate to your prompt that you want to generate JSONL output based on your dataset**. It instead ensures the command output is in JSONL format. You must specify in your prompt to make a completion in JSON format, such as adding the phrase "in JSON format" to your prompt, and this will then be parsed into JSONL format by `describegpt`.

If the prompt output is not in valid JSON format but the `--jsonl` option is specified, the command will generate a default error JSON output printed to `stdout`, such as the following:

```json
{
    "option": {
        "error": "Invalid JSON output for option."
    }
}
```

You may often see this error when `--max-tokens` is set too low and therefore the output is incomplete.

The invalid output will be printed in `stderr`.

Note that `--jsonl` may not be used alongside `--json`, nor may they both be set to true in a prompt file at the same time. This will result in an error.

## `--max-tokens <value>`

`--max-tokens` is an option that allows you to specify the maximum number of tokens in the completion **output**. This is limited by the maximum number of tokens allowed by the model including the input tokens.

Input tokens may include the output of `qsv stats` and `qsv frequency` from your dataset, which can be large based on your dataset's size. Therefore we use `gpt-oss-20b` as the default model for `describegpt` as it has a maximum token limit of 131,072.

It is highly recommended to set the `--max-tokens` option to set the maximum number of tokens in the completion output. Your output may be truncated if you set this value too low or you may receive errors depending on your options. The default is set to `2000` as a safety measure.

When running a Local LLM (detected if the `base_url` contains localhost), the max token limit is automatically disabled. Your completions are only limited by the LLM model you're using.

## `--prompt-file`

With `describegpt` you can use a prompt file to add your own custom prompts and as an alternative to specifying certain options through the CLI. You can use the `--prompt-file` option to specify a prompt file to use.

If you do not specify a prompt file, default prompts will be used.

| Field                    | Description                                                                                 |
| --------------------     | ------------------------------------------------------------------------------------------- |
| `name`                   | The name of your prompt file.                                                               |
| `description`            | A description of your prompt file.                                                          |
| `author`                 | Your name.                                                                                  |
| `version`                | The version of your prompt file.                                                            |
| `tokens`                 | The maximum number of tokens in the completion output.                                      |
| `system_prompt`          | Overall guidance prompt to the LLM.                                                         |
| `dictionary_prompt`      | The prompt for the `--dictionary` option.                                                   |
| `description_prompt`     | The prompt for the `--description` option.                                                  |
| `tags_prompt`            | The prompt for the `--tags` option.                                                         |
| `json`                   | Whether or not the output should be in JSON format (refer to [`--json`](#json) section).    |
| `jsonl`                  | Whether or not the output should be in JSONL format (refer to [`--jsonl`](#jsonl) section). |
| `base_url`               | The URL of the LLM API. When it contains "localhost", automatically sets `tokens` to 0.     |
| `model`                  | The LLM model to use.                                                                       |
| `timeout`                | The timeout in seconds to use when waiting for LLM prompt completions.                      |
| `custom_prompt_guidance` | The guidance used to generate SQL queries in SQL RAG mode.                                  | 
| `duckdb_sql_guidance`    | DuckDB-specific SQL generation guidelines.                                                  |
| `polars_sql_guidance`    | Polars-specific SQL generation guidelines.                                                  |
| `dd_fewshot_examples`    | DuckDB "few-shot" examples. See https://www.promptingguide.ai/techniques/fewshot            |
| `p_fewshot_examples`     | Polars "few-shot" examples.                                                                 |

All fields must be present in your prompt file. If you do not want to use a certain prompt, you can set it to an empty string.

Within your prompts, you can use the following variables:.
These are replaced in the prompt sent to the LLM with their respective values at run-time:

-   `{STATS}` - summary stats generated by qsv stats
-   `{FREQUENCY}` - frequency distribution generated by qsv frequency
-   `{DICTIONARY}` - replaced by the Data Dictionary that was inferred by the LLM using Stats & Frequency
-   `{JSON_ADD}` - inserts ` (in JSON format)`. Note the leading space.
-   `{INPUT_TABLE_NAME}` - sentinel value that is replaced by the name of the input file
-   `{GENERATED_BY_SIGNATURE}` - replaced with model name and current timestamp
-   `{DUCKDB_VERSION}` - DuckDB version (up to the minor version)


See `resources/describegpt_defaults.toml` for the default values.

## Running LLMs locally with Ollama

Since the release of Ollama v0.2.0, Ollama provides the necessary OpenAI compatible endpoints to work with describegpt. You may find the Ollama OpenAI compatibility documentation here: https://github.com/ollama/ollama/blob/main/docs/openai.md.

An example command for getting an inferred description is as follows:

```bash
qsv describegpt <filepath> --base-url http://localhost:11434/v1 --model <model> --max-tokens <number> --description
```

Remove the arrow brackets `<>` and replace `filepath` with your file's path, `<model>` with the model you want to use, and `number` with the max tokens you want to set based on your model's context size.

## SQL Query Generation and Execution ("SQL RAG" mode)

When using the `--prompt` option, `describegpt` can automatically generate and execute SQL queries to answer questions that cannot be answered using just the summary statistics and frequency distribution data. This is called "SQL RAG" mode.

### Using SQL with Polars (default)

By default, when the `polars` feature is enabled, `describegpt` uses qsv's `sqlp` command to execute SQL queries. You can specify the `--sql-results` option to save the query results to a CSV file:

```bash
qsv describegpt data.csv --prompt "What's the breakdown of complaint types by borough?" --sql-results results.csv
```

### Using SQL with DuckDB

You can also use DuckDB to execute SQL queries by setting the `QSV_DESCRIBEGPT_DB_ENGINE` environment variable to a path containing "duckdb" (case-insensitive). The environment variable value should be the fully qualified path to the DuckDB binary:

```bash
export QSV_DESCRIBEGPT_DB_ENGINE=/usr/local/bin/duckdb
qsv describegpt data.csv --prompt "What's the breakdown of complaint types by borough?" --sql-results results.csv
```

When DuckDB is used, the SQL query generation guidelines are automatically modified to use DuckDB generation guidelines.
See `resources/describegpt_defaults.toml` for the default guidelines for DuckDB and Polars.



