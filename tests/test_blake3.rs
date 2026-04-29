use std::{fs, io::Write, process};

use crate::workdir::Workdir;

#[test]
fn blake3_file() {
    let wrk = Workdir::new("blake3_file");
    wrk.create_from_string("hello.txt", "hello world\n");

    let mut cmd = wrk.command("blake3");
    cmd.arg(wrk.path("hello.txt"));

    let got: String = wrk.stdout(&mut cmd);
    // blake3 hash of "hello world\n"
    // Verify: echo -n "hello world\n" | b3sum (but with actual newline)
    assert!(got.contains("hello.txt"), "output should contain filename");
    // Hash should be 64 hex chars
    let hash = got.split("  ").next().unwrap();
    assert_eq!(hash.len(), 64, "hash should be 64 hex chars");
}

#[test]
fn blake3_multiple_files() {
    let wrk = Workdir::new("blake3_multiple_files");
    wrk.create_from_string("a.txt", "aaa");
    wrk.create_from_string("b.txt", "bbb");

    let mut cmd = wrk.command("blake3");
    cmd.arg(wrk.path("a.txt")).arg(wrk.path("b.txt"));

    let got: String = wrk.stdout(&mut cmd);
    let lines: Vec<&str> = got.lines().collect();
    assert_eq!(lines.len(), 2, "should have two output lines");
}

#[test]
fn blake3_no_names() {
    let wrk = Workdir::new("blake3_no_names");
    wrk.create_from_string("hello.txt", "hello");

    let mut cmd = wrk.command("blake3");
    cmd.arg("--no-names").arg(wrk.path("hello.txt"));

    let got: String = wrk.stdout(&mut cmd);
    // Should be just the hash, no filename
    assert!(!got.contains("hello.txt"));
    assert_eq!(got.trim().len(), 64, "hash should be 64 hex chars");
}

#[test]
fn blake3_tag_format() {
    let wrk = Workdir::new("blake3_tag_format");
    wrk.create_from_string("hello.txt", "hello");

    let mut cmd = wrk.command("blake3");
    cmd.arg("--tag").arg(wrk.path("hello.txt"));

    let got: String = wrk.stdout(&mut cmd);
    assert!(
        got.starts_with("BLAKE3 ("),
        "tag format should start with 'BLAKE3 ('"
    );
    assert!(got.contains(") = "), "tag format should contain ') = '");
}

#[test]
fn blake3_custom_length() {
    let wrk = Workdir::new("blake3_custom_length");
    wrk.create_from_string("hello.txt", "hello");

    let mut cmd = wrk.command("blake3");
    cmd.arg("--length")
        .arg("16")
        .arg("--no-names")
        .arg(wrk.path("hello.txt"));

    let got: String = wrk.stdout(&mut cmd);
    // 16 bytes = 32 hex chars
    assert_eq!(got.trim().len(), 32, "16-byte hash should be 32 hex chars");
}

#[test]
fn blake3_check_ok() {
    let wrk = Workdir::new("blake3_check_ok");
    wrk.create_from_string("hello.txt", "hello");

    // First, generate checksum
    let mut cmd = wrk.command("blake3");
    cmd.arg(wrk.path("hello.txt"));
    let checksum_line: String = wrk.stdout(&mut cmd);

    // Write checksum file
    let checksum_path = wrk.path("checksums.txt");
    fs::write(&checksum_path, format!("{checksum_line}\n")).unwrap();

    // Now verify
    let mut cmd = wrk.command("blake3");
    cmd.arg("--check").arg(&checksum_path);

    let got: String = wrk.stdout(&mut cmd);
    assert!(got.contains("OK"), "check should report OK");
}

#[test]
fn blake3_check_failed() {
    let wrk = Workdir::new("blake3_check_failed");
    wrk.create_from_string("hello.txt", "hello");

    // Write a wrong checksum
    let checksum_path = wrk.path("checksums.txt");
    let bad_hash = "0".repeat(64);
    let hello_path = wrk.path("hello.txt").to_string_lossy().to_string();
    fs::write(&checksum_path, format!("{bad_hash}  {hello_path}\n")).unwrap();

    // Verify should fail
    let mut cmd = wrk.command("blake3");
    cmd.arg("--check").arg(&checksum_path);

    wrk.assert_err(&mut cmd);
}

#[test]
fn blake3_check_tag_format() {
    let wrk = Workdir::new("blake3_check_tag_format");
    wrk.create_from_string("hello.txt", "hello");

    // Generate checksum in tag format
    let mut cmd = wrk.command("blake3");
    cmd.arg("--tag").arg(wrk.path("hello.txt"));
    let checksum_line: String = wrk.stdout(&mut cmd);

    // Write checksum file
    let checksum_path = wrk.path("checksums.txt");
    fs::write(&checksum_path, format!("{checksum_line}\n")).unwrap();

    // Verify
    let mut cmd = wrk.command("blake3");
    cmd.arg("--check").arg(&checksum_path);

    let got: String = wrk.stdout(&mut cmd);
    assert!(got.contains("OK"), "check with tag format should report OK");
}

#[test]
fn blake3_no_mmap() {
    let wrk = Workdir::new("blake3_no_mmap");
    wrk.create_from_string("hello.txt", "hello");

    // Hash with mmap
    let mut cmd1 = wrk.command("blake3");
    cmd1.arg("--no-names").arg(wrk.path("hello.txt"));
    let hash_mmap: String = wrk.stdout(&mut cmd1);

    // Hash without mmap
    let mut cmd_2 = wrk.command("blake3");
    cmd_2
        .arg("--no-names")
        .arg("--no-mmap")
        .arg(wrk.path("hello.txt"));
    let hash_no_mmap: String = wrk.stdout(&mut cmd_2);

    assert_eq!(
        hash_mmap.trim(),
        hash_no_mmap.trim(),
        "mmap and no-mmap should produce same hash"
    );
}

#[test]
fn blake3_derive_key() {
    let wrk = Workdir::new("blake3_derive_key");
    wrk.create_from_string("hello.txt", "hello");

    // Hash with derive-key
    let mut cmd = wrk.command("blake3");
    cmd.arg("--derive-key")
        .arg("test context")
        .arg("--no-names")
        .arg(wrk.path("hello.txt"));
    let hash_derived: String = wrk.stdout(&mut cmd);

    // Hash without derive-key
    let mut cmd_2 = wrk.command("blake3");
    cmd_2.arg("--no-names").arg(wrk.path("hello.txt"));
    let hash_default: String = wrk.stdout(&mut cmd_2);

    // They should be different
    assert_ne!(
        hash_derived.trim(),
        hash_default.trim(),
        "derive-key should produce different hash"
    );
}

#[test]
fn blake3_quiet_check() {
    let wrk = Workdir::new("blake3_quiet_check");
    wrk.create_from_string("hello.txt", "hello");

    // Generate checksum
    let mut cmd = wrk.command("blake3");
    cmd.arg(wrk.path("hello.txt"));
    let checksum_line: String = wrk.stdout(&mut cmd);

    let checksum_path = wrk.path("checksums.txt");
    fs::write(&checksum_path, format!("{checksum_line}\n")).unwrap();

    // Verify with --quiet
    let mut cmd = wrk.command("blake3");
    cmd.arg("--check").arg("--quiet").arg(&checksum_path);

    let got: String = wrk.stdout(&mut cmd);
    assert!(
        got.trim().is_empty(),
        "quiet check should produce no output on success"
    );
}

#[test]
fn blake3_nonexistent_file() {
    let wrk = Workdir::new("blake3_nonexistent_file");

    let mut cmd = wrk.command("blake3");
    cmd.arg("nonexistent.txt");

    wrk.assert_err(&mut cmd);
}

#[test]
fn blake3_known_hash() {
    let wrk = Workdir::new("blake3_known_hash");

    // Empty string has a known BLAKE3 hash
    wrk.create_from_string("empty.txt", "");

    let mut cmd = wrk.command("blake3");
    cmd.arg("--no-names").arg(wrk.path("empty.txt"));

    let got: String = wrk.stdout(&mut cmd);
    // BLAKE3 hash of empty input
    assert_eq!(
        got.trim(),
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
    );
}

#[test]
fn blake3_raw_output() {
    let wrk = Workdir::new("blake3_raw_output");
    wrk.create_from_string("hello.txt", "hello");

    let mut cmd = wrk.command("blake3");
    cmd.arg("--raw")
        .arg("--length")
        .arg("16")
        .arg(wrk.path("hello.txt"));

    let output = cmd.output().unwrap();
    assert!(output.status.success());
    // --raw with --length 16 should produce exactly 16 raw bytes
    assert_eq!(
        output.stdout.len(),
        16,
        "raw output should be exactly 16 bytes"
    );
}

#[test]
fn blake3_raw_default_length() {
    let wrk = Workdir::new("blake3_raw_default_length");
    wrk.create_from_string("hello.txt", "hello");

    let mut cmd = wrk.command("blake3");
    cmd.arg("--raw").arg(wrk.path("hello.txt"));

    let output = cmd.output().unwrap();
    assert!(output.status.success());
    // Default length is 32 bytes
    assert_eq!(
        output.stdout.len(),
        32,
        "raw output should be exactly 32 bytes by default"
    );
}

#[test]
fn blake3_stdin_dash() {
    let wrk = Workdir::new("blake3_stdin_dash");

    let mut cmd = wrk.command("blake3");
    cmd.arg("--no-names").arg("-");
    cmd.stdin(process::Stdio::piped());
    cmd.stdout(process::Stdio::piped());
    cmd.stderr(process::Stdio::piped());

    let mut child = cmd.spawn().unwrap();
    {
        let mut stdin_handle = child.stdin.take().unwrap();
        stdin_handle.write_all(b"hello").unwrap();
    }
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should match the known blake3 hash of "hello"
    assert_eq!(
        stdout.trim(),
        "ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f"
    );
}

#[test]
fn blake3_output_file() {
    let wrk = Workdir::new("blake3_output_file");
    wrk.create_from_string("hello.txt", "hello");

    let out_file = wrk.path("output.txt").to_string_lossy().to_string();
    let mut cmd = wrk.command("blake3");
    cmd.arg("--no-names")
        .arg("--output")
        .arg(&out_file)
        .arg(wrk.path("hello.txt"));

    wrk.assert_success(&mut cmd);

    let got = fs::read_to_string(&out_file).unwrap();
    assert_eq!(
        got.trim(),
        "ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f"
    );
}

#[test]
fn blake3_jobs() {
    let wrk = Workdir::new("blake3_jobs");
    wrk.create_from_string("hello.txt", "hello");

    let mut cmd = wrk.command("blake3");
    cmd.arg("--jobs")
        .arg("1")
        .arg("--no-names")
        .arg(wrk.path("hello.txt"));

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(
        got.trim(),
        "ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f"
    );
}

#[test]
fn blake3_keyed() {
    let wrk = Workdir::new("blake3_keyed");
    wrk.create_from_string("hello.txt", "hello");

    // Create a 32-byte key (all zeros for simplicity)
    let key = [0u8; 32];

    let mut cmd = wrk.command("blake3");
    cmd.arg("--keyed")
        .arg("--no-names")
        .arg(wrk.path("hello.txt"));
    cmd.stdin(process::Stdio::piped());
    cmd.stdout(process::Stdio::piped());
    cmd.stderr(process::Stdio::piped());

    let mut child = cmd.spawn().unwrap();
    {
        let mut stdin_handle = child.stdin.take().unwrap();
        stdin_handle.write_all(&key).unwrap();
    }
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "keyed hashing should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let keyed_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    assert_eq!(keyed_hash.len(), 64, "keyed hash should be 64 hex chars");

    // Keyed hash should differ from default hash
    assert_ne!(
        keyed_hash, "ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f",
        "keyed hash should differ from default hash"
    );
}

#[test]
fn blake3_check_invalid_hex() {
    let wrk = Workdir::new("blake3_check_invalid_hex");
    wrk.create_from_string("hello.txt", "hello");

    // Write a checksum with odd-length (invalid) hex
    let checksum_path = wrk.path("checksums.txt");
    let hello_path = wrk.path("hello.txt").to_string_lossy().to_string();
    fs::write(&checksum_path, format!("abc  {hello_path}\n")).unwrap();

    let mut cmd = wrk.command("blake3");
    cmd.arg("--check").arg(&checksum_path);

    wrk.assert_err(&mut cmd);
}

#[test]
fn blake3_check_uppercase_hex() {
    // Checksum file containing uppercase hex must verify OK; the comparison
    // is ASCII-case-insensitive so files produced by tools that emit
    // uppercase hashes still round-trip.
    let wrk = Workdir::new("blake3_check_uppercase_hex");
    wrk.create_from_string("hello.txt", "hello");

    // Generate the (lowercase) checksum first, then uppercase the hash half.
    let mut cmd = wrk.command("blake3");
    cmd.arg(wrk.path("hello.txt"));
    let checksum_line: String = wrk.stdout(&mut cmd);
    let (hash, rest) = checksum_line.split_once("  ").expect("two-space separator");
    let upper_line = format!("{}  {}", hash.to_uppercase(), rest);

    let checksum_path = wrk.path("checksums.txt");
    fs::write(&checksum_path, format!("{upper_line}\n")).unwrap();

    let mut cmd = wrk.command("blake3");
    cmd.arg("--check").arg(&checksum_path);

    let got: String = wrk.stdout(&mut cmd);
    assert!(
        got.contains("OK"),
        "uppercase hex should verify OK, got: {got}"
    );
}

#[test]
fn blake3_check_binary_mode_separator() {
    // `<hash> *<filename>` (single space + asterisk) is the standard *sum
    // binary-mode separator; check_mode must accept it alongside the
    // two-space text-mode separator.
    let wrk = Workdir::new("blake3_check_binary_mode_separator");
    wrk.create_from_string("hello.txt", "hello");

    // Generate the checksum (text-mode default), then rewrite to binary mode.
    let mut cmd = wrk.command("blake3");
    cmd.arg(wrk.path("hello.txt"));
    let checksum_line: String = wrk.stdout(&mut cmd);
    let (hash, filename) = checksum_line.split_once("  ").expect("two-space separator");
    let binary_line = format!("{hash} *{filename}");

    let checksum_path = wrk.path("checksums.txt");
    fs::write(&checksum_path, format!("{binary_line}\n")).unwrap();

    let mut cmd = wrk.command("blake3");
    cmd.arg("--check").arg(&checksum_path);

    let got: String = wrk.stdout(&mut cmd);
    assert!(
        got.contains("OK"),
        "binary-mode separator should verify OK, got: {got}"
    );
}

#[test]
fn blake3_check_malformed_separator() {
    // A line that is neither `<hash>  <filename>` (two spaces) nor
    // `<hash> *<filename>` (single space + asterisk) must error.
    let wrk = Workdir::new("blake3_check_malformed_separator");
    wrk.create_from_string("hello.txt", "hello");

    // 64-char valid hex but with a single plain space (no asterisk) — neither format.
    let checksum_path = wrk.path("checksums.txt");
    let hash = "a".repeat(64);
    let hello_path = wrk.path("hello.txt").to_string_lossy().to_string();
    fs::write(&checksum_path, format!("{hash} {hello_path}\n")).unwrap();

    let mut cmd = wrk.command("blake3");
    cmd.arg("--check").arg(&checksum_path);

    wrk.assert_err(&mut cmd);
}
