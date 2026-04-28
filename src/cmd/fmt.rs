static USAGE: &str = r#"
Formats CSV data with a custom delimiter or CRLF line endings.

Generally, all commands in qsv output CSV data in a default format, which is
the same as the default format for reading CSV data. This makes it easy to
pipe multiple qsv commands together. However, you may want the final result to
have a specific delimiter or record separator, and this is where 'qsv fmt' is
useful.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_fmt.rs.

Usage:
    qsv fmt [options] [<input>]
    qsv fmt --help

fmt options:
    -t, --out-delimiter <arg>  The field delimiter for writing CSV data.
                               Must be a single character.
                               "T" or "\t" can be used as shortcuts for tab.
                               [default: ,]
    --crlf                     Use '\r\n' line endings in the output.
    --ascii                    Use ASCII field/record separators: Unit Separator
                               (U+001F) for fields and Record Separator (U+001E)
                               for records. Substitute (U+001A) is used as the
                               quote character.
    --quote <arg>              The quote character to use. Must be a single
                               character. [default: "]
    --quote-always             Put quotes around every value.
    --quote-never              Never put quotes around any value.
    --escape <arg>             The escape character to use. When not specified,
                               quotes are escaped by doubling them.
    --no-final-newline         Do not write a newline at the end of the output.
                               This makes it easier to paste the output into Excel.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::io::Write;

use serde::Deserialize;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    util,
};

#[derive(Deserialize)]
struct Args {
    arg_input:             Option<String>,
    flag_out_delimiter:    Option<Delimiter>,
    flag_crlf:             bool,
    flag_ascii:            bool,
    flag_output:           Option<String>,
    flag_delimiter:        Option<Delimiter>,
    flag_quote:            Delimiter,
    flag_quote_always:     bool,
    flag_quote_never:      bool,
    flag_escape:           Option<Delimiter>,
    flag_no_final_newline: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    if args.flag_out_delimiter == Some(Delimiter(b'T')) {
        args.flag_out_delimiter = Some(Delimiter(b'\t'));
    }

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(true);
    let mut wconfig = Config::new(args.flag_output.as_ref())
        .delimiter(args.flag_out_delimiter)
        .crlf(args.flag_crlf);

    if args.flag_ascii {
        wconfig = wconfig
            .delimiter(Some(Delimiter(b'\x1f')))
            .terminator(csv::Terminator::Any(b'\x1e'));
        args.flag_quote = Delimiter(b'\x1a');
    }
    if args.flag_quote_always {
        wconfig = wconfig.quote_style(csv::QuoteStyle::Always);
    } else if args.flag_quote_never {
        wconfig = wconfig.quote_style(csv::QuoteStyle::Never);
    }
    if let Some(escape) = args.flag_escape {
        wconfig = wconfig.escape(Some(escape.as_byte())).double_quote(false);
    }
    wconfig = wconfig.quote(args.flag_quote.as_byte());

    let mut rdr = rconfig.reader()?;
    let mut wtr = wconfig.writer()?;

    // Single-record loop with one record held back as `pending` so the final
    // record can be handled separately when `--no-final-newline` is set.
    // mem::swap avoids per-iteration allocations.
    let mut current = csv::ByteRecord::new();
    let mut pending = csv::ByteRecord::new();
    let mut have_pending = false;
    while rdr.read_byte_record(&mut current)? {
        if have_pending {
            wtr.write_record(&pending)?;
        }
        std::mem::swap(&mut current, &mut pending);
        have_pending = true;
    }

    if !have_pending {
        wtr.flush()?;
        return Ok(());
    }

    if !args.flag_no_final_newline {
        wtr.write_record(&pending)?;
        wtr.flush()?;
        return Ok(());
    }

    // --no-final-newline: format the last record into an in-memory buffer using
    // the *exact same* writer settings (via wconfig.from_writer) so the last
    // record can never drift from preceding records when wconfig changes.
    // Strip the trailing terminator, then append the raw bytes to the
    // underlying output.
    let mut buf_wtr = wconfig.from_writer(Vec::<u8>::new());
    buf_wtr.write_record(&pending)?;
    let mut buf = match buf_wtr.into_inner() {
        Ok(b) => b,
        Err(e) => return fail_clierror!("Error buffering final record: {e}"),
    };
    // wconfig.from_writer prepends a UTF-8 BOM when QSV_OUTPUT_BOM is set,
    // but the main writer already emitted one at output start. Gate the
    // strip on the same env var (not on a content match) so a record whose
    // first field happens to begin with U+FEFF is preserved verbatim.
    if util::get_envvar_flag("QSV_OUTPUT_BOM") {
        debug_assert!(buf.starts_with(b"\xEF\xBB\xBF"));
        buf.drain(..3);
    }
    // Strip the trailing terminator. In wconfig, --ascii's
    // `.terminator(Any(b'\x1e'))` override is applied after `.crlf(...)`,
    // so when both flags are set the actual terminator is the 1-byte RS,
    // not CRLF. term_len is therefore 2 only for --crlf without --ascii.
    let term_len = if args.flag_crlf && !args.flag_ascii {
        2
    } else {
        1
    };
    buf.truncate(buf.len().saturating_sub(term_len));

    let mut inner = match wtr.into_inner() {
        Ok(w) => w,
        Err(e) => return fail_clierror!("Error flushing output: {e}"),
    };
    inner.write_all(&buf)?;
    inner.flush()?;
    Ok(())
}
