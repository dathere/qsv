use newline_converter::dos2unix;

use crate::workdir::Workdir;

#[test]
fn snappy_roundtrip() {
    let wrk = Workdir::new("snappy_roundtrip");

    let thedata = vec![
        svec!["Col1", "Description"],
        svec![
            "1",
            "The quick brown fox jumped over the lazy dog by the zigzag quarry site."
        ],
        svec!["2", "メアリーは小さな羊を持っていた"],
        svec![
            "3",
            "I think that I shall never see a poem lovely as a tree."
        ],
        svec!["4", "I think, therefore I am."],
        svec!["5", "मैं हवा पर एक पत्ता हूँ।"],
        svec!["6", "Look at me, I'm the captain now."],
        svec!["7", "终极问题的答案是42。"],
        svec!["8", "I'm Batman."],
    ];
    wrk.create("in.csv", thedata.clone());

    let out_file = wrk.path("out.csv.sz").to_string_lossy().to_string();
    log::info!("out_file: {}", out_file);

    let mut cmd = wrk.command("snappy");
    cmd.arg("compress")
        .arg("in.csv")
        .args(["--output", &out_file]);

    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("snappy"); // DevSkim: ignore DS126858
    cmd.arg("decompress").arg(out_file); // DevSkim: ignore DS126858

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd); // DevSkim: ignore DS126858
    assert_eq!(got, thedata);

    wrk.assert_success(&mut cmd);
}

#[test]
fn snappy_decompress() {
    let wrk = Workdir::new("snappy_decompress");

    let test_file = wrk.load_test_file("boston311-100.csv.sz");

    let mut cmd = wrk.command("snappy");
    cmd.arg("decompress").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());

    wrk.assert_success(&mut cmd);
}

#[test]
fn snappy_decompress_url() {
    let wrk = Workdir::new("snappy_decompress_url");

    let mut cmd = wrk.command("snappy");
    cmd.arg("decompress")
        .arg("https://github.com/dathere/qsv/raw/master/resources/test/boston311-100.csv.sz");

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());

    wrk.assert_success(&mut cmd);
}

#[test]
fn snappy_compress() {
    let wrk = Workdir::new("snappy_compress");

    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("snappy");
    cmd.arg("compress")
        .arg(test_file)
        .args(["--output", "out.csv.sz"]);

    wrk.assert_success(&mut cmd);

    let got_path = wrk.path("out.csv.sz");

    let mut cmd = wrk.command("snappy");
    cmd.arg("decompress")
        .arg(got_path.clone())
        .args(["--output", "out.csv"]);

    wrk.assert_success(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100.csv");
    let got = wrk.read_to_string("out.csv").unwrap();

    assert_eq!(dos2unix(&got).trim_end(), dos2unix(&expected).trim_end());
}

#[test]
fn snappy_check() {
    let wrk = Workdir::new("snappy_check");

    let test_file = wrk.load_test_file("boston311-100.csv.sz");

    let mut cmd = wrk.command("snappy");
    cmd.arg("check").arg(test_file);

    wrk.assert_success(&mut cmd);
}

#[test]
fn snappy_check_invalid() {
    let wrk = Workdir::new("snappy_check_invalid");

    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("snappy");
    cmd.arg("check").arg(test_file);

    wrk.assert_err(&mut cmd);
}

#[test]
fn snappy_validate() {
    let wrk = Workdir::new("snappy_validate");

    let test_file = wrk.load_test_file("boston311-100.csv.sz");

    let mut cmd = wrk.command("snappy");
    cmd.arg("validate").arg(test_file);

    wrk.assert_success(&mut cmd);
}

#[test]
fn snappy_validate_stdin_rejected() {
    // `snappy validate` requires a file/URL <input>; passing stdin must
    // fail with an "incorrect usage" error before any reader setup.
    let wrk = Workdir::new("snappy_validate_stdin_rejected");

    let mut cmd = wrk.command("snappy");
    cmd.arg("validate")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    let mut child = cmd.spawn().unwrap();
    // write nothing; just close stdin so the child sees EOF
    drop(child.stdin.take());

    let output = child.wait_with_output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("stdin is not supported by the snappy validate subcommand"),
        "expected stdin-not-supported error, got stderr: {stderr}"
    );
}

#[test]
fn snappy_decompress_stdin_no_inf_ratio() {
    // When decompressing from stdin, input_bytes is unknown (0). The status
    // message must NOT print an infinite/NaN compression ratio.
    use std::io::Write as _;

    let wrk = Workdir::new("snappy_decompress_stdin_no_inf");

    // Get a real .sz file, then pipe its bytes through stdin.
    let sz_path = wrk.load_test_file("boston311-100.csv.sz");
    let sz_bytes = std::fs::read(&sz_path).unwrap();

    let mut cmd = wrk.command("snappy");
    cmd.arg("decompress")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    let mut child = cmd.spawn().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin.write_all(&sz_bytes).unwrap();
    });

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    // The fallback message must always appear on the stdin path; the test is
    // pointless if we let an empty stderr pass.
    assert!(
        stderr.contains("Decompression successful"),
        "expected fallback decompress-success message, got stderr: {stderr}"
    );
    // The fallback path must skip the "Compression ratio" line entirely —
    // that's exactly how we avoid emitting a non-finite ratio. Asserting the
    // line is absent is precise; substring-matching "inf" is fragile (would
    // collide with words like "info"/"infile") and only worked by accident.
    assert!(
        !stderr.contains("Compression ratio"),
        "stdin path should skip the Compression ratio line, got stderr: {stderr}"
    );
}

#[test]
fn snappy_validate_invalid() {
    let wrk = Workdir::new("snappy_validate_invalid");

    let test_file = wrk.load_test_file("boston311-100-invalidsnappy.csv.sz");

    let mut cmd = wrk.command("snappy");
    cmd.arg("validate").arg(test_file);

    wrk.assert_err(&mut cmd);
}

#[test]
fn snappy_automatic_decompression() {
    let wrk = Workdir::new("snappy_automatic_decompression");

    let test_file = wrk.load_test_file("boston311-100.csv.sz");

    let mut cmd = wrk.command("count");
    cmd.arg(test_file);

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    let expected = "100";
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn snappy_automatic_compression() {
    let wrk = Workdir::new("snappy_automatic_compression");

    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("slice");
    cmd.args(["--len", "50"])
        .arg(test_file)
        .args(["--output", "out.csv.sz"]);

    wrk.assert_success(&mut cmd);

    let got_path = wrk.path("out.csv.sz");

    let mut cmd = wrk.command("count");
    cmd.arg(got_path);

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    let expected = "50";
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn snappy_plain_csv_with_sz_extension_fallback() {
    // Test that a plain CSV file incorrectly named with .sz extension
    // falls back gracefully instead of throwing "corrupt input" error
    let wrk = Workdir::new("snappy_plain_csv_fallback");

    // Create a plain CSV file
    let thedata = vec![
        svec!["Col1", "Col2"],
        svec!["1", "value1"],
        svec!["2", "value2"],
    ];
    wrk.create("plain.csv", thedata.clone());

    // Rename it to have .sz extension (simulating the bug scenario)
    let plain_path = wrk.path("plain.csv");
    let misnamed_path = wrk.path("plain.csv.sz");
    std::fs::copy(&plain_path, &misnamed_path).unwrap();

    // Try to read it - should fall back to reading as plain CSV
    // This tests the fix for issue #2157
    let mut cmd = wrk.command("count");
    cmd.arg("plain.csv.sz");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "2"); // Should count 2 data rows (excluding header)
}

#[test]
fn snappy_case_insensitive_extension() {
    // Test that snappy detection works with case-insensitive extensions
    let wrk = Workdir::new("snappy_case_insensitive");

    let thedata = vec![svec!["Col1", "Col2"], svec!["1", "value1"]];
    wrk.create("test.csv", thedata.clone());

    // Compress to uppercase .SZ
    let out_file = wrk.path("test.csv.SZ").to_string_lossy().to_string();
    let mut cmd = wrk.command("snappy");
    cmd.arg("compress")
        .arg("test.csv")
        .args(["--output", &out_file]);

    wrk.assert_success(&mut cmd);

    // Should be able to read it
    let mut cmd = wrk.command("count");
    cmd.arg("test.csv.SZ");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "1");
}

#[test]
fn snappy_validation_prevents_corrupt_error() {
    // Test that validation prevents "corrupt input" errors
    // when a plain file is incorrectly detected as snappy
    let wrk = Workdir::new("snappy_validation_test");

    // Create a plain CSV file
    let csv_content = "name,age\nAlice,30\nBob,25\n";
    wrk.create_from_string("data.csv", csv_content);

    // Copy it with .sz extension (simulating temp file naming bug)
    let csv_path = wrk.path("data.csv");
    let sz_path = wrk.path("data.csv.sz");
    std::fs::copy(&csv_path, &sz_path).unwrap();

    // Should read successfully without "corrupt input" error
    let mut cmd = wrk.command("slice");
    cmd.args(["--len", "1"]).arg("data.csv.sz");

    wrk.assert_success(&mut cmd);

    // Verify we got the data (should be first row after header)
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["name", "age"], svec!["Alice", "30"]];
    assert_eq!(got, expected);
}
