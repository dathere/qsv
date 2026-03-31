use std::fs;

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

    wrk.assert_success(&mut cmd);
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

    wrk.assert_success(&mut cmd);
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

    wrk.assert_success(&mut cmd);
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

    wrk.assert_success(&mut cmd);
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

    wrk.assert_success(&mut cmd);
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

    wrk.assert_success(&mut cmd);
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

    wrk.assert_success(&mut cmd);
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
    let mut cmd2 = wrk.command("blake3");
    cmd2.arg("--no-names")
        .arg("--no-mmap")
        .arg(wrk.path("hello.txt"));
    let hash_no_mmap: String = wrk.stdout(&mut cmd2);

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
    let mut cmd2 = wrk.command("blake3");
    cmd2.arg("--no-names").arg(wrk.path("hello.txt"));
    let hash_default: String = wrk.stdout(&mut cmd2);

    // They should be different
    assert_ne!(
        hash_derived.trim(),
        hash_default.trim(),
        "derive-key should produce different hash"
    );

    wrk.assert_success(&mut cmd);
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

    wrk.assert_success(&mut cmd);
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

    wrk.assert_success(&mut cmd);
}
