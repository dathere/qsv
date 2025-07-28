use std::{borrow::ToOwned, io::Write};

use crate::workdir::Workdir;

macro_rules! split_eq {
    ($wrk:expr_2021, $path:expr_2021, $expected:expr_2021) => {
        // assert_eq!($wrk.path($path).into_os_string().into_string().unwrap(),
        // $expected.to_owned());
        assert_eq!(
            $wrk.from_str::<String>(&$wrk.path($path)),
            $expected.to_owned()
        );
    };
}

fn data(headers: bool) -> Vec<Vec<String>> {
    let mut rows = vec![
        svec!["a", "b"],
        svec!["c", "d"],
        svec!["e", "f"],
        svec!["g", "h"],
        svec!["i", "j"],
        svec!["k", "l"],
    ];
    if headers {
        rows.insert(0, svec!["h1", "h2"]);
    }
    rows
}

#[test]
fn split_zero() {
    let wrk = Workdir::new("split_zero");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "0"]).arg(&wrk.path(".")).arg("in.csv");
    wrk.assert_err(&mut cmd);
}

#[test]
fn split() {
    let wrk = Workdir::new("split");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"]).arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
k,l
"
    );
    assert!(!wrk.path("6.csv").exists());
}

#[test]
fn split_chunks() {
    let wrk = Workdir::new("split_chunks");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--chunks", "3"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
k,l
"
    );
    assert!(!wrk.path("6.csv").exists());
}

#[test]
fn split_a_lot() {
    let wrk = Workdir::new("split_a_lot");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "1000"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
c,d
e,f
g,h
i,j
k,l
"
    );
    assert!(!wrk.path("1.csv").exists());
}

#[test]
fn split_a_lot_indexed() {
    let wrk = Workdir::new("split_a_lot_indexed");
    wrk.create_indexed("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "1000"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
c,d
e,f
g,h
i,j
k,l
"
    );
    assert!(!wrk.path("1.csv").exists());
}

#[test]
fn split_padding() {
    let wrk = Workdir::new("split");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"])
        .arg("--pad")
        .arg("4")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0000.csv",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "0002.csv",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "0004.csv",
        "\
h1,h2
i,j
k,l
"
    );
    assert!(!wrk.path("0006.csv").exists());
}

#[test]
fn split_chunks_padding() {
    let wrk = Workdir::new("split_chunks_padding");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--chunks", "3"])
        .arg("--pad")
        .arg("4")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0000.csv",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "0002.csv",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "0004.csv",
        "\
h1,h2
i,j
k,l
"
    );
    assert!(!wrk.path("0006.csv").exists());
}

#[test]
fn split_idx() {
    let wrk = Workdir::new("split_idx");
    wrk.create_indexed("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"]).arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
k,l
"
    );
    assert!(!wrk.path("6.csv").exists());
}

#[test]
fn split_chunks_idx() {
    let wrk = Workdir::new("split_chunks_idx");
    wrk.create_indexed("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--chunks", "3"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
k,l
"
    );
    assert!(!wrk.path("6.csv").exists());
}

#[test]
fn split_no_headers() {
    let wrk = Workdir::new("split_no_headers");
    wrk.create("in.csv", data(false));

    let mut cmd = wrk.command("split");
    cmd.args(["--no-headers", "--size", "2"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
i,j
k,l
"
    );
}

#[test]
fn split_chunks_no_headers() {
    let wrk = Workdir::new("split_chunks_no_headers");
    wrk.create("in.csv", data(false));

    let mut cmd = wrk.command("split");
    cmd.args(["--no-headers", "--chunks", "3"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
i,j
k,l
"
    );
}

#[test]
fn split_no_headers_idx() {
    let wrk = Workdir::new("split_no_headers_idx");
    wrk.create_indexed("in.csv", data(false));

    let mut cmd = wrk.command("split");
    cmd.args(["--no-headers", "--size", "2"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
i,j
k,l
"
    );
}

#[test]
fn split_chunks_no_headers_idx() {
    let wrk = Workdir::new("split_chunks_no_headers_idx");
    wrk.create_indexed("in.csv", data(false));

    let mut cmd = wrk.command("split");
    cmd.args(["--no-headers", "--chunks", "3"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
i,j
k,l
"
    );
}

#[test]
fn split_one() {
    let wrk = Workdir::new("split_one");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "1"]).arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
"
    );
    split_eq!(
        wrk,
        "1.csv",
        "\
h1,h2
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
h1,h2
e,f
"
    );
    split_eq!(
        wrk,
        "3.csv",
        "\
h1,h2
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
"
    );
    split_eq!(
        wrk,
        "5.csv",
        "\
h1,h2
k,l
"
    );
}

#[test]
fn split_one_idx() {
    let wrk = Workdir::new("split_one_idx");
    wrk.create_indexed("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "1"]).arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
"
    );
    split_eq!(
        wrk,
        "1.csv",
        "\
h1,h2
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
h1,h2
e,f
"
    );
    split_eq!(
        wrk,
        "3.csv",
        "\
h1,h2
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
"
    );
    split_eq!(
        wrk,
        "5.csv",
        "\
h1,h2
k,l
"
    );
}

#[test]
fn split_uneven() {
    let wrk = Workdir::new("split_uneven");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "4"]).arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
c,d
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
k,l
"
    );
}

#[test]
fn split_chunks_a_lot() {
    let wrk = Workdir::new("split_chunks_a_lot");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--chunks", "10"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
"
    );
    split_eq!(
        wrk,
        "1.csv",
        "\
h1,h2
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
h1,h2
e,f
"
    );
    split_eq!(
        wrk,
        "3.csv",
        "\
h1,h2
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
"
    );
    split_eq!(
        wrk,
        "5.csv",
        "\
h1,h2
k,l
"
    );
    assert!(!wrk.path("6.csv").exists());
}

#[test]
fn split_uneven_idx() {
    let wrk = Workdir::new("split_uneven_idx");
    wrk.create_indexed("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "4"]).arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
c,d
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
k,l
"
    );
}

#[test]
fn split_custom_filename() {
    let wrk = Workdir::new("split");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"])
        .args(["--filename", "prefix-{}.csv"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    assert!(wrk.path("prefix-0.csv").exists());
    assert!(wrk.path("prefix-2.csv").exists());
    assert!(wrk.path("prefix-4.csv").exists());
}

#[test]
fn split_custom_filename_padded() {
    let wrk = Workdir::new("split");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"])
        .arg("--pad")
        .arg("3")
        .args(["--filename", "prefix-{}.csv"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    assert!(wrk.path("prefix-000.csv").exists());
    assert!(wrk.path("prefix-002.csv").exists());
    assert!(wrk.path("prefix-004.csv").exists());
}

#[test]
fn split_nooutdir() {
    let wrk = Workdir::new("split_nooutdir");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"]).arg("in.csv");
    wrk.run(&mut cmd);

    wrk.assert_err(&mut cmd);
    let got = wrk.output_stderr(&mut cmd);
    let expected = "usage error: <outdir> is not specified or is a file.\n";
    assert_eq!(got, expected);
}

#[test]
fn split_kbsize_boston_5k() {
    let wrk = Workdir::new("split_kbsize_boston_5k");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("split");
    cmd.args(["--kb-size", "5"])
        .arg(&wrk.path("."))
        .arg(test_file);
    wrk.run(&mut cmd);

    assert!(wrk.path("0.csv").exists());
    assert!(wrk.path("11.csv").exists());
    assert!(wrk.path("19.csv").exists());
    assert!(wrk.path("27.csv").exists());
    assert!(wrk.path("36.csv").exists());
    assert!(wrk.path("45.csv").exists());
    assert!(wrk.path("52.csv").exists());
    assert!(wrk.path("61.csv").exists());
    assert!(wrk.path("70.csv").exists());
    assert!(wrk.path("78.csv").exists());
    assert!(wrk.path("86.csv").exists());
    assert!(wrk.path("95.csv").exists());
}

#[test]
fn split_kbsize_boston_5k_padded() {
    let wrk = Workdir::new("split_kbsize_boston_5k_padded");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("split");
    cmd.args(["--kb-size", "5"])
        .arg(&wrk.path("."))
        .args(["--filename", "testme-{}.csv"])
        .args(["--pad", "3"])
        .arg(test_file);
    wrk.run(&mut cmd);

    assert!(wrk.path("testme-000.csv").exists());
    assert!(wrk.path("testme-011.csv").exists());
    assert!(wrk.path("testme-019.csv").exists());
    assert!(wrk.path("testme-027.csv").exists());
    assert!(wrk.path("testme-036.csv").exists());
    assert!(wrk.path("testme-045.csv").exists());
    assert!(wrk.path("testme-052.csv").exists());
    assert!(wrk.path("testme-061.csv").exists());
    assert!(wrk.path("testme-070.csv").exists());
    assert!(wrk.path("testme-078.csv").exists());
    assert!(wrk.path("testme-086.csv").exists());
    assert!(wrk.path("testme-095.csv").exists());
}

#[test]
fn split_kbsize_boston_5k_no_headers() {
    let wrk = Workdir::new("split_kbsize_boston_5k_no_headers");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("split");
    cmd.args(["--kb-size", "5"])
        .arg(&wrk.path("."))
        .arg("--no-headers")
        .arg(test_file);
    wrk.run(&mut cmd);

    assert!(wrk.path("0.csv").exists());
    assert!(wrk.path("12.csv").exists());
    assert!(wrk.path("21.csv").exists());
    assert!(wrk.path("29.csv").exists());
    assert!(wrk.path("39.csv").exists());
    assert!(wrk.path("48.csv").exists());
    assert!(wrk.path("56.csv").exists());
    assert!(wrk.path("66.csv").exists());
    assert!(wrk.path("76.csv").exists());
    assert!(wrk.path("84.csv").exists());
    assert!(wrk.path("93.csv").exists());
}

#[test]
fn split_filter_basic() {
    let wrk = Workdir::new("split_filter_basic");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    if cfg!(windows) {
        cmd.args(["--size", "2"])
            .arg("--filter")
            .arg("copy /Y %FILE% {}.bak")
            .arg(&wrk.path("."))
            .arg("in.csv");
    } else {
        cmd.args(["--size", "2"])
            .arg("--filter")
            .arg("cp $FILE {}.bak")
            .arg(&wrk.path("."))
            .arg("in.csv");
    }
    wrk.run(&mut cmd);
    wrk.assert_success(&mut cmd);

    // Check that the original files were created
    assert!(wrk.path("0.csv").exists());
    assert!(wrk.path("2.csv").exists());
    assert!(wrk.path("4.csv").exists());

    // Check that the filtered files were created
    assert!(wrk.path("0.bak").exists());
    assert!(wrk.path("2.bak").exists());
    assert!(wrk.path("4.bak").exists());

    // Verify the content of the filtered files
    split_eq!(
        wrk,
        "0.bak",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.bak",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.bak",
        "\
h1,h2
i,j
k,l
"
    );
}

#[test]
fn split_filter_with_padding() {
    let wrk = Workdir::new("split_filter_with_padding");
    wrk.create("in.csv", data(true));

    // Create a filter command with padding
    let mut cmd = wrk.command("split");
    if cfg!(windows) {
        cmd.args(["--size", "2"])
            .arg("--pad")
            .arg("3")
            .arg("--filter")
            .arg("copy /Y %FILE% chunk_{}.bak")
            .arg(&wrk.path("."))
            .arg("in.csv");
    } else {
        cmd.args(["--size", "2"])
            .arg("--pad")
            .arg("3")
            .arg("--filter")
            .arg("cp $FILE chunk_{}.bak")
            .arg(&wrk.path("."))
            .arg("in.csv");
    }
    wrk.run(&mut cmd);
    wrk.assert_success(&mut cmd);
    // Check that the original files were created
    assert!(wrk.path("000.csv").exists());
    assert!(wrk.path("002.csv").exists());
    assert!(wrk.path("004.csv").exists());

    // Check that the filtered files were created
    assert!(wrk.path("chunk_000.bak").exists());
    assert!(wrk.path("chunk_002.bak").exists());
    assert!(wrk.path("chunk_004.bak").exists());

    // Verify the content of the filtered files
    split_eq!(
        wrk,
        "chunk_000.bak",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "chunk_002.bak",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "chunk_004.bak",
        "\
h1,h2
i,j
k,l
"
    );
}

#[test]
fn split_filter_with_custom_filename() {
    let wrk = Workdir::new("split_filter_with_custom_filename");
    wrk.create("in.csv", data(true));

    // Create a filter command with custom filename
    let mut cmd = wrk.command("split");
    if cfg!(windows) {
        cmd.args(["--size", "2"])
            .args(["--filename", "prefix-{}.csv"])
            .arg("--filter")
            .arg("copy /Y %FILE% prefix-{}.bak")
            .arg(&wrk.path("."))
            .arg("in.csv");
    } else {
        cmd.args(["--size", "2"])
            .args(["--filename", "prefix-{}.csv"])
            .arg("--filter")
            .arg("cp $FILE prefix-{}.bak")
            .arg(&wrk.path("."))
            .arg("in.csv");
    }
    wrk.run(&mut cmd);
    wrk.assert_success(&mut cmd);
    // Check that the original files were created
    assert!(wrk.path("prefix-0.csv").exists());
    assert!(wrk.path("prefix-2.csv").exists());
    assert!(wrk.path("prefix-4.csv").exists());

    // Check that the filtered files were created
    assert!(wrk.path("prefix-0.bak").exists());
    assert!(wrk.path("prefix-2.bak").exists());
    assert!(wrk.path("prefix-4.bak").exists());

    // Verify the content of the filtered files
    split_eq!(
        wrk,
        "prefix-0.bak",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "prefix-2.bak",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "prefix-4.bak",
        "\
h1,h2
i,j
k,l
"
    );
}

#[test]
fn split_filter_with_chunks() {
    let wrk = Workdir::new("split_filter_with_chunks");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    if cfg!(windows) {
        cmd.args(["--chunks", "3"])
            .arg("--filter")
            .arg("copy /Y %FILE% chunk_{}.bak")
            .arg(&wrk.path("."))
            .arg("in.csv");
    } else {
        cmd.args(["--chunks", "3"])
            .arg("--filter")
            .arg("cp $FILE chunk_{}.bak")
            .arg(&wrk.path("."))
            .arg("in.csv");
    }
    wrk.run(&mut cmd);
    wrk.assert_success(&mut cmd);
    // Check that the original files were created
    assert!(wrk.path("0.csv").exists());
    assert!(wrk.path("2.csv").exists());
    assert!(wrk.path("4.csv").exists());

    // Check that the filtered files were created
    assert!(wrk.path("chunk_0.bak").exists());
    assert!(wrk.path("chunk_2.bak").exists());
    assert!(wrk.path("chunk_4.bak").exists());

    // Verify the content of the filtered files
    split_eq!(
        wrk,
        "chunk_0.bak",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "chunk_2.bak",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "chunk_4.bak",
        "\
h1,h2
i,j
k,l
"
    );
}

#[test]
fn split_filter_with_kb_size() {
    let wrk = Workdir::new("split_filter_with_kb_size");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Create a filter command with kb-size
    let mut cmd = wrk.command("split");
    if cfg!(windows) {
        cmd.args(["--kb-size", "5"])
            .arg("--filter")
            .arg("copy /Y %FILE% {}.bak")
            .arg(&wrk.path("."))
            .arg(test_file);
    } else {
        cmd.args(["--kb-size", "5"])
            .arg("--filter")
            .arg("cp $FILE {}.bak")
            .arg(&wrk.path("."))
            .arg(test_file);
    }
    wrk.run(&mut cmd);
    wrk.assert_success(&mut cmd);
    // Check that at least some of the original files were created
    assert!(wrk.path("0.csv").exists());
    assert!(wrk.path("11.csv").exists());

    // Check that at least some of the filtered files were created
    assert!(wrk.path("0.bak").exists());
    assert!(wrk.path("11.bak").exists());

    // Verify the content of the filtered files matches the original files
    split_eq!(wrk, "0.bak", wrk.from_str::<String>(&wrk.path("0.csv")));
    split_eq!(wrk, "11.bak", wrk.from_str::<String>(&wrk.path("11.csv")));
}

#[test]
fn split_filter_with_no_headers() {
    let wrk = Workdir::new("split_filter_with_no_headers");
    wrk.create("in.csv", data(false));

    // Create a filter command with no headers
    let mut cmd = wrk.command("split");
    if cfg!(windows) {
        cmd.args(["--no-headers", "--size", "2"])
            .arg("--filter")
            .arg("copy /Y %FILE% {}.bak")
            .arg(&wrk.path("."))
            .arg("in.csv");
    } else {
        cmd.args(["--no-headers", "--size", "2"])
            .arg("--filter")
            .arg("cp $FILE {}.bak")
            .arg(&wrk.path("."))
            .arg("in.csv");
    }
    wrk.run(&mut cmd);
    wrk.assert_success(&mut cmd);
    // Check that the original files were created
    assert!(wrk.path("0.csv").exists());
    assert!(wrk.path("2.csv").exists());
    assert!(wrk.path("4.csv").exists());

    // Check that the filtered files were created
    assert!(wrk.path("0.bak").exists());
    assert!(wrk.path("2.bak").exists());
    assert!(wrk.path("4.bak").exists());

    // Verify the content of the filtered files
    split_eq!(
        wrk,
        "0.bak",
        "\
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.bak",
        "\
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.bak",
        "\
i,j
k,l
"
    );
}

#[test]
fn split_filter_with_cleanup() {
    let wrk = Workdir::new("split_filter_with_cleanup");
    wrk.create("in.csv", data(true));

    // Create a filter command with cleanup
    let mut cmd = wrk.command("split");
    if cfg!(windows) {
        cmd.args(["--size", "2"])
            .arg("--filter")
            .arg("copy /Y %FILE% {}.bak")
            .arg("--filter-cleanup")
            .arg(&wrk.path("."))
            .arg("in.csv");
    } else {
        cmd.args(["--size", "2"])
            .arg("--filter")
            .arg("cp $FILE {}.bak")
            .arg("--filter-cleanup")
            .arg(&wrk.path("."))
            .arg("in.csv");
    }
    wrk.run(&mut cmd);

    wrk.assert_success(&mut cmd);

    // Check that the original files were removed
    assert!(!wrk.path("0.csv").exists());
    assert!(!wrk.path("2.csv").exists());
    assert!(!wrk.path("4.csv").exists());

    // Check that the filtered files were created
    assert!(wrk.path("0.bak").exists());
    assert!(wrk.path("2.bak").exists());
    assert!(wrk.path("4.bak").exists());

    // Verify the content of the filtered files
    split_eq!(
        wrk,
        "0.bak",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.bak",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.bak",
        "\
h1,h2
i,j
k,l
"
    );
}

#[test]
fn split_filter_without_cleanup() {
    let wrk = Workdir::new("split_filter_without_cleanup");
    wrk.create("in.csv", data(true));

    // Create a filter command without cleanup
    let mut cmd = wrk.command("split");
    if cfg!(windows) {
        cmd.args(["--size", "2"])
            .arg("--filter")
            .arg("copy /Y %FILE% {}.bak")
            .arg(&wrk.path("."))
            .arg("in.csv");
    } else {
        cmd.args(["--size", "2"])
            .arg("--filter")
            .arg("cp $FILE {}.bak")
            .arg(&wrk.path("."))
            .arg("in.csv");
    }
    wrk.run(&mut cmd);
    wrk.assert_success(&mut cmd);

    // Check that the original files were kept
    assert!(wrk.path("0.csv").exists());
    assert!(wrk.path("2.csv").exists());
    assert!(wrk.path("4.csv").exists());

    // Check that the filtered files were created
    assert!(wrk.path("0.bak").exists());
    assert!(wrk.path("2.bak").exists());
    assert!(wrk.path("4.bak").exists());

    // Verify the content of the filtered files
    split_eq!(
        wrk,
        "0.bak",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.bak",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.bak",
        "\
h1,h2
i,j
k,l
"
    );
}

#[test]
fn split_filter_with_cleanup_failed_command() {
    let wrk = Workdir::new("split_filter_with_cleanup_failed_command");
    wrk.create("in.csv", data(true));

    // Create a filter command with cleanup but with a command that will fail
    let mut cmd = wrk.command("split");
    if cfg!(windows) {
        cmd.args(["--size", "2"])
            .arg("--filter")
            .arg("nonexistent_command %FILE% \"{}.bak\"")
            .arg("--filter-cleanup")
            .arg(&wrk.path("."))
            .arg("in.csv");
    } else {
        cmd.args(["--size", "2"])
            .arg("--filter")
            .arg("nonexistent_command $FILE {}.bak")
            .arg("--filter-cleanup")
            .arg(&wrk.path("."))
            .arg("in.csv");
    }
    wrk.run(&mut cmd);
    wrk.assert_err(&mut cmd);

    // The first chunk should still exist because it was created before the filter command failed
    assert!(wrk.path("0.csv").exists());

    // The second and third chunks should not exist because the filter command failed
    assert!(!wrk.path("2.csv").exists());
    assert!(!wrk.path("4.csv").exists());

    // Check that the filtered files were not created
    assert!(!wrk.path("0.bak").exists());
    assert!(!wrk.path("2.bak").exists());
    assert!(!wrk.path("4.bak").exists());
}

#[test]
fn split_filter_with_ignore_errors() {
    let wrk = Workdir::new("split_filter_with_ignore_errors");
    wrk.create("in.csv", data(true));

    // Create a filter command with ignore-errors
    let mut cmd = wrk.command("split");
    if cfg!(windows) {
        cmd.args(["--size", "2"])
            .arg("--filter")
            .arg("nonexistent_command %FILE% \"{}.bak\"")
            .arg("--filter-ignore-errors")
            .arg(&wrk.path("."))
            .arg("in.csv");
    } else {
        cmd.args(["--size", "2"])
            .arg("--filter")
            .arg("nonexistent_command $FILE {}.bak")
            .arg("--filter-ignore-errors")
            .arg(&wrk.path("."))
            .arg("in.csv");
    }
    // The command should run successfully despite the filter command failing
    wrk.run(&mut cmd);
    wrk.assert_success(&mut cmd);
    // Check that the original files were created
    assert!(wrk.path("0.csv").exists());
    assert!(wrk.path("2.csv").exists());
    assert!(wrk.path("4.csv").exists());

    // Check that the filtered files were not created
    assert!(!wrk.path("0.bak").exists());
    assert!(!wrk.path("2.bak").exists());
    assert!(!wrk.path("4.bak").exists());
}

#[test]
fn split_filter_without_ignore_errors() {
    let wrk = Workdir::new("split_filter_without_ignore_errors");
    wrk.create("in.csv", data(true));

    // Create a filter command without ignore-errors
    let mut cmd = wrk.command("split");
    if cfg!(windows) {
        cmd.args(["--size", "2"])
            .arg("--filter")
            .arg("nonexistent_command %FILE% \"{}.bak\"")
            .arg(&wrk.path("."))
            .arg("in.csv");
    } else {
        cmd.args(["--size", "2"])
            .arg("--filter")
            .arg("nonexistent_command $FILE {}.bak")
            .arg(&wrk.path("."))
            .arg("in.csv");
    }
    // The command should fail when the filter command fails
    wrk.assert_err(&mut cmd);

    // The first chunk should still exist because it was created before the filter command failed
    assert!(wrk.path("0.csv").exists());

    // The second and third chunks should not exist because the filter command failed
    assert!(!wrk.path("2.csv").exists());
    assert!(!wrk.path("4.csv").exists());

    // Check that the filtered files were not created
    assert!(!wrk.path("0.bak").exists());
    assert!(!wrk.path("2.bak").exists());
    assert!(!wrk.path("4.bak").exists());
}

#[test]
#[cfg(windows)]
fn split_filter_powershell() {
    let wrk = Workdir::new("split_filter_powershell");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"])
        .arg("--filter")
        .arg(r#"powershell.exe -NoProfile -NonInteractive -Command Copy-Item -Path $env:FILE -Destination "{}.bak""#)
        .arg(&wrk.path("."))
        .arg("in.csv");

    wrk.run(&mut cmd);
    wrk.assert_success(&mut cmd);

    // Check that the original CSV files were created
    assert!(wrk.path("0.csv").exists());
    assert!(wrk.path("2.csv").exists());
    assert!(wrk.path("4.csv").exists());

    // Check that the bak files were created
    assert!(wrk.path("0.bak").exists());
    assert!(wrk.path("2.bak").exists());
    assert!(wrk.path("4.bak").exists());
}

#[test]
#[cfg(windows)]
fn split_filter_powershell_cleanup() {
    let wrk = Workdir::new("split_filter_powershell_cleanup");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"])
        .arg("--filter")
        .arg(r#"powershell.exe -NoProfile -NonInteractive -Command Copy-Item -Path $env:FILE -Destination "{}.bak""#)
        .arg("--filter-cleanup")
        .arg(&wrk.path("."))
        .arg("in.csv");

    wrk.run(&mut cmd);
    wrk.assert_success(&mut cmd);

    // Check that the original CSV files were deleted after compression
    assert!(!wrk.path("0.csv").exists());
    assert!(!wrk.path("2.csv").exists());
    assert!(!wrk.path("4.csv").exists());

    // Check that the bak files were created
    assert!(wrk.path("0.bak").exists());
    assert!(wrk.path("2.bak").exists());
    assert!(wrk.path("4.bak").exists());
}

#[test]
#[cfg(windows)]
fn split_filter_windows_paths() {
    let wrk = Workdir::new("split_filter_windows_paths");
    wrk.create("in.csv", data(true));

    // Test with a path containing spaces and special characters
    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"])
        .arg("--filter")
        .arg("copy /Y %FILE% {}.bak")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);
    wrk.assert_success(&mut cmd);

    // Check that the original files were created
    assert!(wrk.path("0.csv").exists());
    assert!(wrk.path("2.csv").exists());
    assert!(wrk.path("4.csv").exists());

    // Check that the filtered files were created
    assert!(wrk.path("0.bak").exists());
    assert!(wrk.path("2.bak").exists());
    assert!(wrk.path("4.bak").exists());
}

#[test]
#[cfg(windows)]
fn split_filter_windows_long_paths() {
    let wrk = Workdir::new("split_filter_windows_long_paths");
    wrk.create("in.csv", data(true));

    // Create a deeply nested directory structure to test long paths
    let deep_dir = wrk.path("a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z");
    std::fs::create_dir_all(&deep_dir).unwrap();

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"])
        .arg("--filter")
        .arg("copy /Y %FILE% {}.bak")
        .arg(&deep_dir)
        .arg("in.csv");
    wrk.run(&mut cmd);
    wrk.assert_success(&mut cmd);

    // Check that the files were created in the deep directory
    assert!(deep_dir.join("0.csv").exists());
    assert!(deep_dir.join("2.csv").exists());
    assert!(deep_dir.join("4.csv").exists());
    assert!(deep_dir.join("0.bak").exists());
    assert!(deep_dir.join("2.bak").exists());
    assert!(deep_dir.join("4.bak").exists());
}

#[test]
fn split_stdin_100_rows() {
    let wrk = Workdir::new("split_stdin_100_rows");

    // Create a 100 row CSV with headers
    let mut rows = vec![svec!["id", "name", "value"]];

    // Add 100 rows of data
    for i in 0..100 {
        rows.push(vec![
            i.to_string(),
            format!("item_{}", i),
            format!("value_{}", i),
        ]);
    }

    // Create a temporary file with the CSV data
    wrk.create("stdin_data.csv", rows);

    // Run the split command with stdin input
    let mut cmd = wrk.command("split");
    cmd.args(["--size", "20"])
        .arg(&wrk.path("."))
        .arg("--quiet")
        .arg("-"); // Use "-" to indicate stdin

    // Set up stdin for the command
    let stdin_data = wrk.read_to_string("stdin_data.csv").unwrap();
    cmd.stdin(std::process::Stdio::piped());

    // Run the command
    let mut child = cmd.spawn().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin.write_all(stdin_data.as_bytes()).unwrap();
    });

    // Wait for the command to complete
    let status = child.wait().unwrap();
    assert!(status.success());

    // Verify that 5 files were created (100 rows / 20 rows per file = 5 files)
    assert!(wrk.path("0.csv").exists());
    assert!(wrk.path("20.csv").exists());
    assert!(wrk.path("40.csv").exists());
    assert!(wrk.path("60.csv").exists());
    assert!(wrk.path("80.csv").exists());
    assert!(!wrk.path("100.csv").exists());

    // Verify the content of the first file
    split_eq!(
        wrk,
        "0.csv",
        "\
id,name,value
0,item_0,value_0
1,item_1,value_1
2,item_2,value_2
3,item_3,value_3
4,item_4,value_4
5,item_5,value_5
6,item_6,value_6
7,item_7,value_7
8,item_8,value_8
9,item_9,value_9
10,item_10,value_10
11,item_11,value_11
12,item_12,value_12
13,item_13,value_13
14,item_14,value_14
15,item_15,value_15
16,item_16,value_16
17,item_17,value_17
18,item_18,value_18
19,item_19,value_19
"
    );

    // Verify the content of the last file
    split_eq!(
        wrk,
        "80.csv",
        "\
id,name,value
80,item_80,value_80
81,item_81,value_81
82,item_82,value_82
83,item_83,value_83
84,item_84,value_84
85,item_85,value_85
86,item_86,value_86
87,item_87,value_87
88,item_88,value_88
89,item_89,value_89
90,item_90,value_90
91,item_91,value_91
92,item_92,value_92
93,item_93,value_93
94,item_94,value_94
95,item_95,value_95
96,item_96,value_96
97,item_97,value_97
98,item_98,value_98
99,item_99,value_99
"
    );
}
