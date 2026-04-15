use crate::workdir::Workdir;

#[test]
fn log_start_entry() {
    let wrk = Workdir::new("log_start_entry");
    let mut cmd = wrk.command("log");
    cmd.args(["qsv_stats", "s-abc123", "Analyzing", "data"]);

    wrk.output(&mut cmd);

    let log_content: String = wrk.from_str(&wrk.path("qsvmcp.log"));
    assert!(log_content.contains("s-abc123 qsv_stats: Analyzing data"));
    // Verify ISO-8601 timestamp format (RFC 3339 with milliseconds)
    // e.g. [2026-03-01T10:30:00.123Z]
    let re = regex::Regex::new(r"^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z\] ").unwrap();
    assert!(
        re.is_match(&log_content),
        "Log entry should start with RFC 3339 millis timestamp, got: {}",
        &log_content[..log_content.len().min(40)]
    );
}

#[test]
fn log_end_entry_success() {
    let wrk = Workdir::new("log_end_entry_success");
    let mut cmd = wrk.command("log");
    cmd.args(["qsv_stats", "e-abc123", "ok(1.23s)"]);

    wrk.output(&mut cmd);

    let log_content: String = wrk.from_str(&wrk.path("qsvmcp.log"));
    assert!(log_content.contains("e-abc123 qsv_stats: ok(1.23s)"));
}

#[test]
fn log_end_entry_error() {
    let wrk = Workdir::new("log_end_entry_error");
    let mut cmd = wrk.command("log");
    cmd.args([
        "qsv_stats",
        "e-abc123",
        "error(0.45s):",
        "file",
        "not",
        "found",
    ]);

    wrk.output(&mut cmd);

    let log_content: String = wrk.from_str(&wrk.path("qsvmcp.log"));
    assert!(log_content.contains("e-abc123 qsv_stats: error(0.45s): file not found"));
}

#[test]
fn log_append_behavior() {
    let wrk = Workdir::new("log_append_behavior");

    // First entry
    let mut cmd1 = wrk.command("log");
    cmd1.args(["qsv_stats", "s-first", "Starting"]);
    wrk.output(&mut cmd1);

    // Second entry
    let mut cmd_2 = wrk.command("log");
    cmd_2.args(["qsv_frequency", "s-second", "Checking", "frequencies"]);
    wrk.output(&mut cmd_2);

    let log_content: String = wrk.from_str(&wrk.path("qsvmcp.log"));
    let lines: Vec<&str> = log_content.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].contains("s-first qsv_stats: Starting"));
    assert!(lines[1].contains("s-second qsv_frequency: Checking frequencies"));
}

#[test]
fn log_empty_message() {
    let wrk = Workdir::new("log_empty_message");
    let mut cmd = wrk.command("log");
    cmd.args(["qsv_count", "s-nomsg"]);

    wrk.output(&mut cmd);

    let log_content: String = wrk.from_str(&wrk.path("qsvmcp.log"));
    assert!(log_content.contains("s-nomsg qsv_count: "));
}

#[test]
fn log_special_chars_sanitized() {
    let wrk = Workdir::new("log_special_chars_sanitized");

    // Message with newlines that could inject fake log entries
    let mut cmd = wrk.command("log");
    cmd.args([
        "qsv_stats",
        "s-inject1",
        "legit\n[2099-01-01T00:00:00.000Z] s-fake injected_tool: fake entry",
    ]);
    wrk.output(&mut cmd);

    let log_content: String = wrk.from_str(&wrk.path("qsvmcp.log"));
    let lines: Vec<&str> = log_content.lines().collect();
    // Should be exactly one log line — the newline must be sanitized
    assert_eq!(
        lines.len(),
        1,
        "Newline in message must not create extra log lines"
    );
    // The sanitized message should still contain all non-control parts on a single line
    assert!(lines[0].contains("s-inject1 qsv_stats:"));
    // The injected fake entry text is present but harmlessly on the same line
    assert!(lines[0].contains("injected_tool"));
}

#[test]
fn log_control_chars_in_tool_name() {
    let wrk = Workdir::new("log_control_chars_in_tool_name");
    let mut cmd = wrk.command("log");
    cmd.args(["tool\r\nfake", "s-ctrl", "test"]);

    wrk.output(&mut cmd);

    let log_content: String = wrk.from_str(&wrk.path("qsvmcp.log"));
    let lines: Vec<&str> = log_content.lines().collect();
    assert_eq!(
        lines.len(),
        1,
        "Control chars in tool name must not create extra log lines"
    );
}

#[test]
fn log_control_chars_in_invocation_id() {
    let wrk = Workdir::new("log_control_chars_in_invocation_id");
    let mut cmd = wrk.command("log");
    cmd.args(["qsv_stats", "s-inject\nfake-id", "test message"]);

    wrk.output(&mut cmd);

    let log_content: String = wrk.from_str(&wrk.path("qsvmcp.log"));
    let lines: Vec<&str> = log_content.lines().collect();
    assert_eq!(
        lines.len(),
        1,
        "Control chars in invocation ID must not create extra log lines"
    );
    assert!(lines[0].contains("s-inject fake-id qsv_stats: test message"));
}
