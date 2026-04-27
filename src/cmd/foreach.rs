static USAGE: &str = r#"
Execute a shell command once per record in a given CSV file.

NOTE: Windows users are recommended to use Git Bash as their terminal when
running this command. Download it from https://git-scm.com/downloads. When installing,
be sure to select "Use Git from the Windows Command Prompt" to ensure that the
necessary Unix tools are available in the terminal.

WARNING: This command can be dangerous. Be careful when using it with
untrusted input.

Or per @thadguidry: 😉
Please ensure when using foreach to use trusted arguments, variables, scripts, etc.
If you don't do due diligence and blindly use untrusted parts... foreach can indeed
become a footgun and possibly fry your computer, eat your lunch, and expose an entire
datacenter to a cancerous virus in your unvetted batch file you grabbed from some
stranger on the internet that runs...FOR EACH LINE in your CSV file. GASP!"

Examples:

Delete all files whose filenames are listed in the filename column:

  $ qsv foreach filename 'rm {}' assets.csv

Execute a command that outputs CSV once per record without repeating headers:

  $ qsv foreach query --unify 'search --year 2020 {}' queries.csv > results.csv

Same as above but with an additional column containing the current value:

  $ qsv foreach query -u -c from_query 'search {}' queries.csv > results.csv

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_foreach.rs.

If any child command exits with a non-zero status, foreach finishes processing
all rows but then exits with a non-zero status of its own.

Usage:
    qsv foreach [options] <column> <command> [<input>]
    qsv foreach --help

foreach arguments:
    column      The column whose value is substituted into the command.
                Only a single column is accepted.
    command     The command to execute. Use "{}" to substitute the value
                of the current input file line. The command must be
                non-empty after whitespace trimming.
                If you need to execute multiple commands, use a shell
                script. See foreach_multiple_commands_with_shell_script()
                in tests/test_foreach.rs for an example.
    input       The CSV file to read. If not provided, will read from stdin.

foreach options:
    -u, --unify                If the output of the executed command is a CSV,
                               unify the result by skipping headers on each
                               subsequent command. Does not work when --dry-run is true.
                               The first child's CSV header row becomes canonical;
                               later children are expected to produce the same schema.
    -c, --new-column <name>    If unifying, add a new column with given name
                               and copying the value of the current input file line.
    --dry-run <file|boolean>   If set to true (the default for safety reasons), the commands are
                               sent to stdout instead of executing them.
                               If set to a file, the commands will be written to the specified
                               text file instead of executing them. The file is only created
                               after all flag validation succeeds, so a conflicting flag
                               combination will not truncate an existing file.
                               Only if set to false will the commands be actually executed.
                               [default: true]

Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, the file will be considered to have no
                           headers.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -p, --progressbar      Show progress bars. Not valid for stdin.
"#;

#[cfg(target_family = "windows")]
use std::ffi::OsString;
#[cfg(target_family = "unix")]
use std::{ffi::OsStr, os::unix::ffi::OsStrExt};
use std::{
    io::{self, BufReader, BufWriter, Write},
    process::{Command, Stdio},
};

#[cfg(feature = "feature_capable")]
use indicatif::{ProgressBar, ProgressDrawTarget};
use regex::bytes::Regex;
use serde::Deserialize;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    select::SelectColumns,
    util,
};

#[derive(Deserialize)]
struct Args {
    arg_column:       SelectColumns,
    arg_command:      String,
    arg_input:        Option<String>,
    flag_unify:       bool,
    flag_new_column:  Option<String>,
    flag_dry_run:     String,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
    flag_progressbar: bool,
}

enum DryRun {
    /// dry-run output goes to stdout (the default).
    Stdout,
    /// dry-run output is written to the given file.
    File(String),
    /// not a dry run; child commands are actually executed.
    Disabled,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    if args.arg_command.trim().is_empty() {
        return fail_incorrectusage_clierror!("foreach: <command> cannot be empty");
    }

    let dry_run = match args.flag_dry_run.as_str() {
        s if s.eq_ignore_ascii_case("true") => DryRun::Stdout,
        s if s.eq_ignore_ascii_case("false") => DryRun::Disabled,
        file_str => DryRun::File(file_str.to_string()),
    };
    let is_dry_run = !matches!(dry_run, DryRun::Disabled);

    // Validate flag combinations BEFORE any side effects (file creation, etc.)
    // so a conflicting --dry-run=file --unify never truncates the user's file.
    if is_dry_run && args.flag_unify {
        return fail_incorrectusage_clierror!("Cannot use --unify with --dry-run");
    }
    if args.flag_new_column.is_some() && !args.flag_unify {
        return fail_incorrectusage_clierror!("Cannot use --new-column without --unify");
    }

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers)
        .select(args.arg_column);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(None).writer()?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;
    if sel.len() > 1 {
        return fail_incorrectusage_clierror!(
            "foreach accepts a single column; got {} columns",
            sel.len()
        );
    }
    let Some(&column_index) = sel.iter().next() else {
        return fail_incorrectusage_clierror!("foreach: no input column selected");
    };

    // template_pattern matches `{}` substitution markers in the user's command.
    #[allow(clippy::trivial_regex)]
    let template_pattern = Regex::new(r"\{\}")?;

    // splitter_pattern tokenises the substituted command. It matches either:
    //   - a sequence of word-like characters (a-z, A-Z, 0-9, _, ., +, /, -), or
    //   - a double-quoted, single-quoted, or backtick-quoted string.
    // It does not handle escaped quotes — for anything fancier, users should
    // wrap the command in a shell script.
    let splitter_pattern = Regex::new(r#"(?:[a-zA-Z0-9_.+/-]+|"[^"]*"|'[^']*'|`[^`]*`)"#)?;

    // Open the dry-run sink only AFTER all flag validation has run, so a
    // user-supplied dry-run file is never truncated for a command that was
    // about to error out anyway.
    let mut dry_run_file: Box<dyn Write> = match &dry_run {
        DryRun::Stdout => Box::new(BufWriter::new(io::stdout())),
        DryRun::File(path) => match std::fs::File::create(path) {
            Ok(f) => Box::new(BufWriter::new(f)),
            Err(e) => {
                return fail_incorrectusage_clierror!("Error creating dry-run file '{path}': {e}");
            },
        },
        DryRun::Disabled => Box::new(io::sink()),
    };

    let mut record = csv::ByteRecord::new();
    let mut output_headers_written = false;

    // prep progress bar
    #[cfg(feature = "feature_capable")]
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();
    #[cfg(feature = "feature_capable")]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    #[cfg(feature = "feature_capable")]
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    let mut row_idx: u64 = 0;
    let mut any_child_failed = false;

    while rdr.read_byte_record(&mut record)? {
        row_idx += 1;
        #[cfg(feature = "feature_capable")]
        if show_progress {
            progress.inc(1);
        }
        let current_value = &record[column_index];

        // replace_all returns a Cow<[u8]> that lives only for this iteration —
        // no per-row allocation when there are no `{}` markers, and otherwise a
        // single owned buffer that's dropped at end of iteration.
        let templated_command =
            template_pattern.replace_all(args.arg_command.as_bytes(), current_value);

        let mut command_pieces = splitter_pattern.find_iter(&templated_command);

        let Some(prog_match) = command_pieces.next() else {
            // Empty post-substitution command — treat the same as a non-zero
            // child exit so we honour the "finish all rows, then exit non-zero"
            // contract instead of bailing mid-stream.
            eprintln!("foreach: row {row_idx} command is empty after substitution; skipping");
            any_child_failed = true;
            continue;
        };

        #[cfg(target_family = "unix")]
        let prog = OsStr::from_bytes(prog_match.as_bytes());
        #[cfg(target_family = "windows")]
        let prog = match simdutf8::basic::from_utf8(prog_match.as_bytes()) {
            Ok(s) => OsString::from(s),
            Err(_) => {
                return fail_clierror!("foreach: program path contains invalid UTF-8");
            },
        };

        // Strip outer matching quotes from each subsequent token. The splitter
        // already guarantees that quoted tokens have the same opening and
        // closing quote character, so a one-byte check on each end is enough —
        // no second regex pass needed.
        let cmd_args: Vec<String> = command_pieces
            .map(|piece| {
                let bytes = piece.as_bytes();
                let stripped = if bytes.len() >= 2 {
                    let first = bytes[0];
                    if matches!(first, b'"' | b'\'' | b'`') && bytes[bytes.len() - 1] == first {
                        &bytes[1..bytes.len() - 1]
                    } else {
                        bytes
                    }
                } else {
                    bytes
                };
                simdutf8::basic::from_utf8(stripped)
                    .unwrap_or_default()
                    .to_string()
            })
            .collect();

        if is_dry_run {
            #[cfg(target_family = "unix")]
            let prog_str = simdutf8::basic::from_utf8(prog.as_bytes()).unwrap_or_default();
            #[cfg(target_family = "windows")]
            let prog_str = simdutf8::basic::from_utf8(prog.as_encoded_bytes()).unwrap_or_default();
            let cmd_args_string = cmd_args.join(" ");
            dry_run_file.write_all(format!("{prog_str} {cmd_args_string}\n").as_bytes())?;
            continue;
        }

        let status = if args.flag_unify {
            let mut cmd = Command::new(prog)
                .args(cmd_args)
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .spawn()?;

            {
                let stdout = cmd.stdout.as_mut().unwrap();
                let stdout_reader = BufReader::new(stdout);

                let mut stdout_rdr = csv::ReaderBuilder::new()
                    .delimiter(match &args.flag_delimiter {
                        Some(delimiter) => delimiter.as_byte(),
                        None => b',',
                    })
                    .has_headers(true)
                    .from_reader(stdout_reader);

                let mut output_record = csv::ByteRecord::new();

                if !output_headers_written {
                    // Headers from the first child command's CSV output become
                    // canonical for the unified stream — subsequent commands
                    // are expected to produce CSVs with the same schema.
                    let mut headers = stdout_rdr.byte_headers()?.clone();

                    if let Some(name) = &args.flag_new_column {
                        headers.push_field(name.as_bytes());
                    }

                    wtr.write_byte_record(&headers)?;
                    output_headers_written = true;
                }

                while stdout_rdr.read_byte_record(&mut output_record)? {
                    if args.flag_new_column.is_some() {
                        output_record.push_field(current_value);
                    }

                    wtr.write_byte_record(&output_record)?;
                }
            }

            cmd.wait()?
        } else {
            let mut cmd = Command::new(prog)
                .args(cmd_args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?;

            cmd.wait()?
        };

        if !status.success() {
            eprintln!(
                "foreach: row {row_idx} command failed (exit {})",
                status
                    .code()
                    .map_or_else(|| "signal".to_string(), |c| c.to_string())
            );
            any_child_failed = true;
        }
    }
    #[cfg(feature = "feature_capable")]
    if show_progress {
        util::finish_progress(&progress);
    }
    dry_run_file.flush()?;
    wtr.flush()?;

    if any_child_failed {
        return fail_clierror!("foreach: one or more child commands exited with non-zero status");
    }
    Ok(())
}
