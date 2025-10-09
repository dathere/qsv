use std::process;

use crate::workdir::Workdir;

fn setup(name: &str) -> (Workdir, process::Command) {
    let rows1 = vec![svec!["h1", "h2"], svec!["a", "b"]];
    let rows2 = vec![svec!["h2", "h3"], svec!["y", "z"]];

    let wrk = Workdir::new(name);
    wrk.create("in1.csv", rows1);
    wrk.create("in2.csv", rows2);

    let mut cmd = wrk.command("headers");
    cmd.arg("in1.csv");

    (wrk, cmd)
}

#[test]
fn headers_basic() {
    let (wrk, mut cmd) = setup("headers_basic");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
1   h1
2   h2";
    assert_eq!(got, expected.to_string());
}

#[test]
fn headers_just_names() {
    let (wrk, mut cmd) = setup("headers_just_names");
    cmd.arg("--just-names");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
h1
h2";
    assert_eq!(got, expected.to_string());
}

#[test]
fn headers_just_count() {
    let (wrk, mut cmd) = setup("headers_just_count");
    cmd.arg("--just-count");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "2";
    assert_eq!(got, expected.to_string());
}

#[test]
fn headers_notrim() {
    let wrk = Workdir::new("headers_notrim");

    // headers taken from malformed CSV example - cities.csv at
    // https://people.sc.fsu.edu/~jburkardt/data/csv/csv.html
    wrk.create(
        "data.csv",
        vec![
            svec![
                "\"LatD\"",
                "\"LatM\"",
                "\"LatS\"",
                "\"NS\"",
                "\"LonD\"",
                "\"LonM\"",
                "\"LonS\"",
                "\"EW\"",
                "\"City\"",
                "\"State\""
            ],
            svec![
                "41",
                "5",
                "59",
                "N",
                "80",
                "39",
                "0",
                "W",
                "Youngstown",
                "OH"
            ],
        ],
    );

    let mut cmd = wrk.command("headers");
    cmd.arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"1   "LatD"
2   "LatM"
3   "LatS"
4   "NS"
5   "LonD"
6   "LonM"
7   "LonS"
8   "EW"
9   "City"
10  "State""#;
    assert_eq!(got, expected);
}

#[test]
fn headers_trim() {
    let wrk = Workdir::new("headers_trim");

    // headers taken from malformed CSV example - cities.csv at
    // https://people.sc.fsu.edu/~jburkardt/data/csv/csv.html
    wrk.create(
        "data.csv",
        vec![
            svec![
                "\"LatD\"",
                "\"LatM\"",
                "\"LatS\"",
                "\"NS\"",
                "\"LonD\"",
                "\"LonM\"",
                "\"LonS\"",
                "\"EW\"",
                "\"City\"",
                "\"State\""
            ],
            svec![
                "41",
                "5",
                "59",
                "N",
                "80",
                "39",
                "0",
                "W",
                "Youngstown",
                "OH"
            ],
        ],
    );

    let mut cmd = wrk.command("headers");
    cmd.arg("--trim").arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"1   LatD
2   LatM
3   LatS
4   NS
5   LonD
6   LonM
7   LonS
8   EW
9   City
10  State"#;
    assert_eq!(got, expected);
}

#[test]
fn headers_multiple() {
    let (wrk, mut cmd) = setup("headers_multiple");
    cmd.arg("in2.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
h1
h2
h2
h3";
    assert_eq!(got, expected.to_string());
}

#[test]
fn headers_multiple_just_count() {
    let (wrk, mut cmd) = setup("headers_multiple_just_count");
    cmd.arg("in2.csv").arg("--just-count");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4";
    assert_eq!(got, expected.to_string());
}

#[test]
fn headers_intersect() {
    let (wrk, mut cmd) = setup("headers_intersect");
    cmd.arg("in2.csv").arg("--intersect");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
h1
h2
h3";
    assert_eq!(got, expected.to_string());
}

#[test]
fn headers_infile() {
    let wrk = Workdir::new("headers_infile");
    wrk.create("in1.csv", vec![svec!["a", "b", "c"], svec!["1", "2", "3"]]);

    wrk.create("in2.csv", vec![svec!["c", "d", "e"], svec!["3", "1", "2"]]);

    wrk.create(
        "in3.csv",
        vec![svec!["a", "b", "f", "g"], svec!["1", "2", "4", "3"]],
    );

    wrk.create_from_string("testdata.infile-list", "in1.csv\nin2.csv\nin3.csv\n");

    let mut cmd: process::Command = wrk.command("headers");
    cmd.arg("testdata.infile-list");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        ["a"],
        ["b"],
        ["c"],
        ["c"],
        ["d"],
        ["e"],
        ["a"],
        ["b"],
        ["f"],
        ["g"],
    ];
    assert_eq!(got, expected);

    let mut cmd: process::Command = wrk.command("headers");
    cmd.arg("testdata.infile-list").arg("--just-count");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "10";
    assert_eq!(got, expected.to_string());
}

#[test]
fn headers_intersect_infile() {
    let wrk = Workdir::new("headers_intersect_infile");
    wrk.create("in1.csv", vec![svec!["a", "b", "c"], svec!["1", "2", "3"]]);

    wrk.create("in2.csv", vec![svec!["c", "d", "e"], svec!["3", "1", "2"]]);

    wrk.create(
        "in3.csv",
        vec![svec!["a", "b", "f", "g"], svec!["1", "2", "4", "3"]],
    );

    wrk.create_from_string("testdata.infile-list", "in1.csv\nin2.csv\nin3.csv\n");

    let mut cmd = wrk.command("headers");
    cmd.arg("--intersect").arg("testdata.infile-list");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![["a"], ["b"], ["c"], ["d"], ["e"], ["f"], ["g"]];
    assert_eq!(got, expected);
}

#[test]
fn headers_stdin_explicit() {
    use std::io::Write;

    let wrk = Workdir::new("headers_stdin_explicit");

    // Create test data
    let rows = vec![svec!["h1", "h2", "h3"], svec!["a", "b", "c"]];
    wrk.create("test_data.csv", rows);

    // Set up command to read from stdin
    let mut cmd = wrk.command("headers");
    cmd.arg("-"); // Explicitly use stdin

    // Set up stdin and stdout for the command
    let stdin_data = wrk.read_to_string("test_data.csv").unwrap();
    cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());

    // Run the command
    let mut child = cmd.spawn().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin.write_all(stdin_data.as_bytes()).unwrap();
    });

    // Wait for the command to complete and capture output
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let got = String::from_utf8_lossy(&output.stdout);
    let expected = "1   h1\n2   h2\n3   h3";
    assert_eq!(got.trim(), expected);
}

#[test]
fn headers_stdin_implicit() {
    use std::io::Write;

    let wrk = Workdir::new("headers_stdin_implicit");

    // Create test data
    let rows = vec![svec!["col1", "col2"], svec!["x", "y"]];
    wrk.create("test_data.csv", rows);

    // Set up command WITHOUT explicit "-" argument (implicit stdin)
    let mut cmd = wrk.command("headers");

    // Set up stdin and stdout for the command
    let stdin_data = wrk.read_to_string("test_data.csv").unwrap();
    cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());

    // Run the command
    let mut child = cmd.spawn().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin.write_all(stdin_data.as_bytes()).unwrap();
    });

    // Wait for the command to complete and capture output
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let got = String::from_utf8_lossy(&output.stdout);
    let expected = "1   col1\n2   col2";
    assert_eq!(got.trim(), expected);
}

#[test]
fn headers_stdin_with_just_names() {
    use std::io::Write;

    let wrk = Workdir::new("headers_stdin_with_just_names");

    // Create test data
    let rows = vec![svec!["name", "age", "city"], svec!["Alice", "30", "NYC"]];
    wrk.create("test_data.csv", rows);

    // Set up command to read from stdin with --just-names flag
    let mut cmd = wrk.command("headers");
    cmd.arg("--just-names");

    // Set up stdin and stdout for the command
    let stdin_data = wrk.read_to_string("test_data.csv").unwrap();
    cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());

    // Run the command
    let mut child = cmd.spawn().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin.write_all(stdin_data.as_bytes()).unwrap();
    });

    // Wait for the command to complete and capture output
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let got = String::from_utf8_lossy(&output.stdout);
    let expected = "name\nage\ncity";
    assert_eq!(got.trim(), expected);
}
