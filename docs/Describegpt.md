# `describegpt` command

`describegpt` allows users to infer extended metadata about a CSV dataset using large language models, in particular GPT chat completion models from OpenAI's API, Ollama, or an API compatible with the OpenAI API specification such as Jan.

`describegpt` uses `qsv stats` and `qsv frequency` in the background to provide context to the model. It does not send the entire dataset to the model. This "zero-copy" approach is not only ideal for reducing costs (as detailed metadata, even for very large dataset is "tiny" in comparison) and increasing performance, it also protects the privacy of sensitive data.

Note that this command uses LLMs for inferencing and is therefore prone to inaccurate information being produced. Verify output results before using them.

## QSV_OPENAI_KEY

`describegpt` requires an OpenAI API key to use by default. You can set this key using the `QSV_OPENAI_KEY` environment variable. Check [/docs/ENVIRONMENT_VARIABLES.md](/docs/ENVIRONMENT_VARIABLES.md) for more info.

If you're not using the OpenAI API, this environment variable is not necessary so long as you pass a value into `--api-key` (for example when using Ollama, use `--api-key ollama`).

## `--api-key <key>`

You can also specify your API key directly in your CLI using the `--api-key` option.

Note that if you already have `QSV_OPENAI_KEY` set as an environment variable and it is not empty, this environment variable will override your given flag.

If you're using Ollama, use `--api-key ollama`.

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
| `base_url`               | The URL of the LLM API. When it contains "localhost", aumatically sets `tokens` to 0.       |
| `model`                  | The LLM model to use.                                                                       |
| `timeout`                | The timeout in seconds to use when waiting for LLM prompt completions.                      |
| `custom_prompt_guidance` | The guidance used to generate SQL queries in SQL RAG mode.                                  | 

All fields must be present in your prompt file. If you do not want to use a certain prompt, you can set it to an empty string.

Within your prompts, you can use the following variables:

-   `{stats}`
-   `{frequency}`
-   `{json_add}`

These are replaced with the output of `qsv stats`, `qsv frequency` and conditionally ` (in JSON format)`. Note that `{json_add}` adds a space before `(in JSON format)`.

See `resources/describegpt_defaults.json` for the default values.

## Running LLMs locally with Ollama

Since the release of Ollama v0.2.0, Ollama provides the necessary OpenAI compatible endpoints to work with describegpt. You may find the Ollama OpenAI compatibility documentation here: https://github.com/ollama/ollama/blob/main/docs/openai.md.

An example command for getting an inferred description is as follows:

```bash
qsv describegpt <filepath> --base-url http://localhost:11434/v1 --api-key ollama --model <model> --max-tokens <number> --description
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

When DuckDB is used, the SQL query generation guidelines are automatically modified to use DuckDB generation guidelines (see below).

### SQL Query Generation Guidelines

When in SQL RAG mode, the LLM generates SQL queries following these guidelines:

**For PostgreSQL (default):**
- Use PostgreSQL syntax
- Use `INPUT_TABLE_NAME` as the placeholder for the table name
- Column names with spaces and special characters should be enclosed in double quotes
- Avoid certain SQL functions and expressions (see default prompt file for details)
- Add comments with `--` prefix only
- Include a comment with "GENERATED_BY_SIGNATURE" placeholder

The generated SQL query will automatically replace `INPUT_TABLE_NAME` with the fully qualified name of the input CSV file.

**For DuckDB:**
- Use DuckDB syntax
- Use DuckDB's `read_csv` table function to read the input CSV
- Use the placeholder `INPUT_TABLE_NAME` for the input CSV
- Use the Data Dictionary to set the read_csv's `columns` parameter
- Map Data Dictionary data types to proper DuckDB data types in the read_csv's `columns` parameter
- Make sure the generated SQL query is valid and has comments to explain the query
- Add a comment with the placeholder "GENERATED_BY_SIGNATURE" at the top of the query

The generated SQL query will automatically replace `INPUT_TABLE_NAME` with a `read_csv` table function call that reads your CSV file directly.

