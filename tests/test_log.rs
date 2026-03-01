use crate::workdir::Workdir;

#[test]
fn log_start_entry() {
    let wrk = Workdir::new("log_start_entry");
    let mut cmd = wrk.command("log");
    cmd.args(["qsv_stats", "s-abc123", "Analyzing", "data"]);

    wrk.output(&mut cmd);

    let log_content: String = wrk.from_str(&wrk.path("qsvmcp.log"));
    assert!(log_content.contains("s-abc123 qsv_stats: Analyzing data"));
    // Verify ISO-8601 timestamp format
    assert!(log_content.starts_with('['));
    assert!(log_content.contains(']'));
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
    let mut cmd2 = wrk.command("log");
    cmd2.args(["qsv_frequency", "s-second", "Checking", "frequencies"]);
    wrk.output(&mut cmd2);

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
