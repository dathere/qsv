static USAGE: &str = r#"
Explore a CSV file interactively using the csvlens (https://github.com/YS-L/csvlens) engine.

If the polars feature is enabled, lens can also browse Arrow, Avro/IPC, Parquet, JSON (JSON Array)
and JSONL files. It also automatically decompresses csv/tsv/tab/ssv files using the gz,zlib & zst
compression formats (e.g. data.csv.gz, data.tsv.zlib, data.tab.gz & data.ssv.zst).

Press 'q' to exit. Press '?' for help.

Usage:
    qsv lens [options] [<input>]
    qsv lens --help

Examples:
Automatically choose delimiter based on the file extension
  $ qsv lens data.csv // comma-separated
  $ qsv lens data.tsv // Tab-separated
  $ qsv lens data.tab // Tab-separated
  $ qsv lens data.ssv // Semicolon-separated
  # custom delimiter
  $ qsv lens --delimiter ';' data.csv

Auto-decompresses several compression formats:
  $ qsv lens data.csv.sz // Snappy-compressed CSV
  $ qsv lens data.csv.gz // Gzipped CSV
  $ qsv lens data.tsv.zlib // Zlib-compressed Tab-separated
  $ qsv lens data.tab.zst // Zstd-compressed Tab-separated
  $ qsv lens data.ssv.zst // Zstd-compressed Semicolon-separated
  
Explore tabular data in other formats
  $ qsv lens data.jsonl // JSON Lines
  $ qsv lens data.json // JSON - will only work with a JSON Array
  $ qsv lens data.parquet // Parquet
  $ qsv lens data.avro // Avro

Prompt the user to select a column to display. Once selected,
exit with the value of the City column for the selected row sent to stdout
  $ qsv lens --prompt 'Select City:' --echo-column 'City' data.csv

Only show rows that contain "NYPD"
  $ qsv lens --filter NYPD data.csv
  # Show rows that contain "nois" case insensitive
  $ qsv lens --filter nois --ignore-case data.csv
 
Find and highlight matches in the data
  $ qsv lens --find 'New York' data.csv

Find and highlight cells that have all numeric values in a column.
Use -m to disable color output so the matches are easier to see.
  $ qsv lens --find '^\d+$' -m data.csv

lens options:
  -d, --delimiter <char>           Delimiter character (comma by default)
                                   "auto" to auto-detect the delimiter
  -t, --tab-separated              Use tab separation. Shortcut for -d '\t'
      --no-headers                 Do not interpret the first row as headers

      --columns <regex>            Use this regex to select columns to display by default.
                                   Example: "col1|col2|col3" to select columns "col1", "col2" and "col3"
                                   and also columns like "col1_1", "col22" and "col3-more".
      --filter <regex>             Use this regex to filter rows to display by default.
                                   The regex is matched against each cell in every column.
                                   Example: "val1|val2" filters rows with any cells containing "val1", "val2"
                                   or text like "my_val1" or "val234".
      --find <regex>               Use this regex to find and highlight matches by default.
                                   The regex is matched against each cell in every column.
                                   Example: "val1|val2" highlights text containing "val1", "val2" or
                                   longer text like "val1_ok" or "val2_error".

  -i, --ignore-case                Searches ignore case. Ignored if any uppercase letters
                                   are present in the search string
  -f, --freeze-columns <num>       Freeze the first N columns
                                   [default: 1]
  -m, --monochrome                 Disable color output
  -W, --wrap-mode <mode>           Set the wrap mode for the output.
                                   Valid modes are:
                                     "words": Wrap at word boundaries
                                     "chars": Wrap at character boundaries
                                     "disabled": No wrapping
                                   For convenience, the first character can be used as a shortcut:
                                     qsv lens -W w data.csv // wrap at word boundaries
                                   [default: disabled]

  -P. --prompt <prompt>            Set a custom prompt in the status bar. Normally paired w/ --echo-column:
                                     qsv lens --prompt 'Select City:' --echo-column 'City'
                                   Supports ANSI escape codes for colored or styled text. When using
                                   escape codes, ensure it's properly escaped. For example, in bash/zsh,
                                   the $'...' syntax is used to do so:
                                     qsv lens --prompt $'\033[1;5;31mBlinking red, bold text\033[0m'
                                   see https://en.wikipedia.org/wiki/ANSI_escape_code#Colors or
                                   https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
                                   for more info on ANSI escape codes.
                                   Typing a complicated prompt on the command line can be tricky.
                                   If the prompt starts with "file:", it's interpreted as a filepath
                                   from which to load the prompt, e.g.
                                     qsv lens --prompt "file:prompt.txt"
      --echo-column <column_name>  Print the value of this column to stdout for the selected row

      --debug                      Show stats for debugging

Common options:
    -h, --help      Display this message
"#;

use std::path::PathBuf;

use csvlens::{CsvlensOptions, WrapMode, run_csvlens_with_options};
use serde::Deserialize;
use tempfile;

use crate::{CliError, CliResult, config::Config, util};

#[derive(Deserialize)]
struct Args {
    arg_input:           Option<String>,
    flag_delimiter:      Option<String>,
    flag_tab_separated:  bool,
    flag_no_headers:     bool,
    flag_columns:        Option<String>,
    flag_filter:         Option<String>,
    flag_find:           Option<String>,
    flag_ignore_case:    bool,
    flag_freeze_columns: Option<u64>,
    flag_monochrome:     bool,
    flag_prompt:         Option<String>,
    flag_echo_column:    Option<String>,
    flag_wrap_mode:      String,
    flag_debug:          bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Process input file
    // support stdin and auto-decompress snappy file
    // stdin/decompressed file is written to a temporary file in tmpdir
    // which is automatically deleted after the command finishes
    let tmpdir = tempfile::tempdir()?;
    let work_input = util::process_input(
        vec![PathBuf::from(
            // if no input file is specified, read from stdin "-"
            args.arg_input.unwrap_or_else(|| "-".to_string()),
        )],
        &tmpdir,
        "",
    )?;
    let input = work_input[0].to_string_lossy().to_string();

    // If the prompt starts with "file:", it's interpreted as a filepath
    // from which to load the prompt, e.g.
    // qsv lens --prompt "file:prompt.txt"
    let prompt = if let Some(prompt) = args.flag_prompt {
        if prompt.starts_with("file:") {
            let prompt_file = PathBuf::from(prompt.trim_start_matches("file:"));
            let prompt = std::fs::read_to_string(prompt_file)?;
            Some(prompt)
        } else {
            Some(prompt)
        }
    } else {
        None
    };

    // Convert the wrap mode to a WrapMode enum value
    // we only check the first character of the wrap mode string for convenience
    let wrap_mode = match args.flag_wrap_mode.to_ascii_lowercase().chars().next() {
        Some('d') => Some(WrapMode::Disabled),
        Some('w') => Some(WrapMode::Words),
        Some('c') => Some(WrapMode::Chars),
        _ => None,
    };

    // Create a Config to:
    // 1. Get the delimiter (from QSV_DEFAULT_DELIMITER env var if set)
    // 2. Check if delimiter sniffing is enabled (via QSV_SNIFF_DELIMITER)
    // 3. Handle special file formats like Parquet/Avro if polars is enabled
    let config: Config = Config::new(Some(input).as_ref());

    let input = config.path.clone().map(|p| p.to_string_lossy().to_string());

    let options = CsvlensOptions {
        filename: input,
        delimiter: if let Some(delimiter) = args.flag_delimiter {
            Some(delimiter)
        } else {
            Some((config.get_delimiter() as char).to_string())
        },
        tab_separated: args.flag_tab_separated,
        no_headers: args.flag_no_headers,
        columns: args.flag_columns,
        filter: args.flag_filter,
        find: args.flag_find,
        ignore_case: args.flag_ignore_case,
        echo_column: args.flag_echo_column,
        debug: args.flag_debug,
        freeze_cols_offset: args.flag_freeze_columns,
        color_columns: !args.flag_monochrome,
        prompt,
        wrap_mode,
    };

    let out = run_csvlens_with_options(options)
        .map_err(|e| CliError::Other(format!("csvlens error: {e}")))?;

    if let Some(selected_cell) = out {
        println!("{selected_cell}");
    }

    Ok(())
}
