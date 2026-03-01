static USAGE: &str = r#"
Logs an MCP tool invocation entry to qsvmcp.log.

This command is used internally by the MCP server to create an audit trail
of tool invocations. Each entry includes the tool name, a prefixed invocation
UUID (s- for start, e- for end), and context (agent's reason or result).

The log file (qsvmcp.log) is written in the current working directory.

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

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = crate::util::get_args(USAGE, argv)?;

    let message = args.arg_message.join(" ");

    let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    let line = format!(
        "[{timestamp}] {} {}: {message}\n",
        args.arg_invocation_id, args.arg_tool_name,
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("qsvmcp.log")?;

    file.write_all(line.as_bytes())?;
    file.flush()?;

    Ok(())
}
