use std::borrow::ToOwned;

use crate::workdir::Workdir;

macro_rules! split_eq {
    ($wrk:expr_2021, $path:expr_2021, $expected:expr_2021) => {
        // similar_asserts::assert_eq!($wrk.path($path).into_os_string().into_string().unwrap(),
        // $expected.to_owned());
        similar_asserts::assert_eq!(
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
    similar_asserts::assert_eq!(got, expected);
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

    // Create a simple filter command that creates a copy of each chunk with a .bak extension
    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"])
        .arg("--filter")
        .arg("cp $FILE {}.bak")
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
    cmd.args(["--size", "2"])
        .arg("--pad")
        .arg("3")
        .arg("--filter")
        .arg("cp $FILE chunk_{}.bak")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

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
    cmd.args(["--size", "2"])
        .args(["--filename", "prefix-{}.csv"])
        .arg("--filter")
        .arg("cp $FILE prefix-{}.bak")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

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

    // Create a filter command with chunks
    let mut cmd = wrk.command("split");
    cmd.args(["--chunks", "3"])
        .arg("--filter")
        .arg("cp $FILE chunk_{}.bak")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

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
    cmd.args(["--kb-size", "5"])
        .arg("--filter")
        .arg("cp $FILE {}.bak")
        .arg(&wrk.path("."))
        .arg(test_file);
    wrk.run(&mut cmd);

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
    cmd.args(["--no-headers", "--size", "2"])
        .arg("--filter")
        .arg("cp $FILE {}.bak")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

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
    cmd.args(["--size", "2"])
        .arg("--filter")
        .arg("cp $FILE {}.bak")
        .arg("--filter-cleanup")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

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
    cmd.args(["--size", "2"])
        .arg("--filter")
        .arg("cp $FILE {}.bak")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

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
    cmd.args(["--size", "2"])
        .arg("--filter")
        .arg("nonexistent_command $FILE {}.bak")
        .arg("--filter-cleanup")
        .arg(&wrk.path("."))
        .arg("in.csv");

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
    cmd.args(["--size", "2"])
        .arg("--filter")
        .arg("nonexistent_command $FILE {}.bak")
        .arg("--filter-ignore-errors")
        .arg(&wrk.path("."))
        .arg("in.csv");

    // The command should run successfully despite the filter command failing
    wrk.run(&mut cmd);

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
    cmd.args(["--size", "2"])
        .arg("--filter")
        .arg("nonexistent_command $FILE {}.bak")
        .arg(&wrk.path("."))
        .arg("in.csv");

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
