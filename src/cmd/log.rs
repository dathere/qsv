static USAGE: &str = r#"
Logs an MCP tool invocation entry to qsvmcp.log.

This command is used internally by the MCP server to create an audit trail
of tool invocations. Each entry includes the tool name, a prefixed invocation
UUID (s- for start, e- for end), and context (agent's reason or result).

The log file (qsvmcp.log) is written in the current working directory of the
process. When invoked by the MCP server, this is the server's configured working
directory (set via qsv_set_working_dir). Ensure the CWD is consistent to avoid
logs being scattered across directories.

Usage:
    qsv log <tool-name> <invocation-id> [<message>...]
    qsv log --help

Common options:
    -h, --help     Display this message
"#;

use std::{fs::OpenOptions, io::Write};

use serde::Deserialize;

use crate::clitypes::CliResult;

#[derive(Deserialize)]
struct Args {
    arg_tool_name:     String,
    arg_invocation_id: String,
    arg_message:       Vec<String>,
}

/// Sanitize a string for safe log output by replacing control characters
/// (newlines, carriage returns, tabs, etc.) with spaces, and NUL bytes with
/// the Unicode replacement character (U+FFFD) so they stand out in log output.
fn sanitize_log_field(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c == '\0' {
                '\u{FFFD}'
            } else if c.is_control() {
                ' '
            } else {
                c
            }
        })
        .collect()
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = crate::util::get_args(USAGE, argv)?;

    let message = sanitize_log_field(&args.arg_message.join(" "));
    let tool_name = sanitize_log_field(&args.arg_tool_name);
    let invocation_id = sanitize_log_field(&args.arg_invocation_id);

    let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    let line = format!("[{timestamp}] {invocation_id} {tool_name}: {message}\n");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("qsvmcp.log")?;

    file.write_all(line.as_bytes())?;
    file.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::sanitize_log_field;

    #[test]
    fn nul_bytes_replaced_with_fffd() {
        assert_eq!(sanitize_log_field("id\0test"), "id\u{FFFD}test");
        assert_eq!(sanitize_log_field("hello\0world"), "hello\u{FFFD}world");
        assert_eq!(sanitize_log_field("\0"), "\u{FFFD}");
    }

    #[test]
    fn control_chars_replaced_with_space() {
        assert_eq!(sanitize_log_field("a\nb"), "a b");
        assert_eq!(sanitize_log_field("a\r\nb"), "a  b");
        assert_eq!(sanitize_log_field("a\tb"), "a b");
    }

    #[test]
    fn nul_and_control_chars_mixed() {
        assert_eq!(sanitize_log_field("a\0b\nc"), "a\u{FFFD}b c");
    }

    #[test]
    fn clean_string_unchanged() {
        assert_eq!(sanitize_log_field("hello world"), "hello world");
        assert_eq!(sanitize_log_field(""), "");
    }
}
