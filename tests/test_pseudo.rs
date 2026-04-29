use crate::workdir::Workdir;

#[test]
fn pseudo() {
    let wrk = Workdir::new("pseudo");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "colors"],
            svec!["Mary", "yellow"],
            svec!["John", "blue"],
            svec!["Mary", "purple"],
            svec!["Sue", "orange"],
            svec!["John", "magenta"],
            svec!["Mary", "cyan"],
        ],
    );
    let mut cmd = wrk.command("pseudo");
    cmd.arg("name").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "colors"],
        svec!["0", "yellow"],
        svec!["1", "blue"],
        svec!["0", "purple"],
        svec!["2", "orange"],
        svec!["1", "magenta"],
        svec!["0", "cyan"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn pseudo_no_headers() {
    let wrk = Workdir::new("pseudo");
    wrk.create(
        "data.csv",
        vec![
            svec!["Mary", "yellow"],
            svec!["John", "blue"],
            svec!["Mary", "purple"],
            svec!["Sue", "orange"],
            svec!["John", "magenta"],
            svec!["Mary", "cyan"],
        ],
    );
    let mut cmd = wrk.command("pseudo");
    cmd.arg("1").arg("--no-headers").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["0", "yellow"],
        svec!["1", "blue"],
        svec!["0", "purple"],
        svec!["2", "orange"],
        svec!["1", "magenta"],
        svec!["0", "cyan"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn pseudo_formatstr() {
    let wrk = Workdir::new("pseudo_formatstr");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "colors"],
            svec!["Mary", "yellow"],
            svec!["John", "blue"],
            svec!["Mary", "purple"],
            svec!["Sue", "orange"],
            svec!["John", "magenta"],
            svec!["Mary", "cyan"],
        ],
    );
    let mut cmd = wrk.command("pseudo");
    cmd.arg("name")
        .args(["--formatstr", "ID-{}"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "colors"],
        svec!["ID-0", "yellow"],
        svec!["ID-1", "blue"],
        svec!["ID-0", "purple"],
        svec!["ID-2", "orange"],
        svec!["ID-1", "magenta"],
        svec!["ID-0", "cyan"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn pseudo_formatstr_increment() {
    let wrk = Workdir::new("pseudo_formatstr_increment");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "colors"],
            svec!["Mary", "yellow"],
            svec!["John", "blue"],
            svec!["Mary", "purple"],
            svec!["Sue", "orange"],
            svec!["John", "magenta"],
            svec!["Mary", "cyan"],
        ],
    );
    let mut cmd = wrk.command("pseudo");
    cmd.arg("name")
        .args(["--formatstr", "ID-{}"])
        .args(["--increment", "5"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "colors"],
        svec!["ID-0", "yellow"],
        svec!["ID-5", "blue"],
        svec!["ID-0", "purple"],
        svec!["ID-10", "orange"],
        svec!["ID-5", "magenta"],
        svec!["ID-0", "cyan"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn pseudo_formatstr_start_increment() {
    let wrk = Workdir::new("pseudo_formatstr_start_increment");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "colors"],
            svec!["Mary", "yellow"],
            svec!["John", "blue"],
            svec!["Mary", "purple"],
            svec!["Sue", "orange"],
            svec!["John", "magenta"],
            svec!["Mary", "cyan"],
        ],
    );
    let mut cmd = wrk.command("pseudo");
    cmd.arg("name")
        .args(["--start", "1000"])
        .args(["--formatstr", "ID-{}"])
        .args(["--increment", "5"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "colors"],
        svec!["ID-1000", "yellow"],
        svec!["ID-1005", "blue"],
        svec!["ID-1000", "purple"],
        svec!["ID-1010", "orange"],
        svec!["ID-1005", "magenta"],
        svec!["ID-1000", "cyan"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn pseudo_overflow() {
    let wrk = Workdir::new("pseudo_overflow");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "colors"],
            svec!["Mary", "yellow"],
            svec!["John", "blue"],
            svec!["Mary", "purple"],
            svec!["Sue", "orange"],
            svec!["John", "magenta"],
            svec!["Mary", "cyan"],
            svec!["Bob", "teal"],
        ],
    );
    let close_to_max = std::u64::MAX - 10;
    let mut cmd = wrk.command("pseudo");
    cmd.arg("name")
        .args(["--formatstr", "ID-{}"])
        .args(["--start", close_to_max.to_string().as_str()])
        .args(["--increment", "5"])
        .arg("data.csv");

    wrk.assert_err(&mut cmd);
    // Sue gets the last valid counter (u64::MAX) and is written. Repeats of
    // Mary/John continue to resolve from the cache. Bob would need a new
    // pseudonym after the counter overflowed, so the command errors there.
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "colors"],
        svec!["ID-18446744073709551605", "yellow"],
        svec!["ID-18446744073709551610", "blue"],
        svec!["ID-18446744073709551605", "purple"],
        svec!["ID-18446744073709551615", "orange"],
        svec!["ID-18446744073709551610", "magenta"],
        svec!["ID-18446744073709551605", "cyan"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(
        got_err,
        "usage error: Counter overflow: incrementing past u64::MAX (18446744073709551615). Last \
         valid counter: 18446744073709551615.\n"
    );
}

#[test]
fn pseudo_increment_zero_rejected() {
    let wrk = Workdir::new("pseudo_increment_zero_rejected");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["Mary"], svec!["John"]],
    );
    let mut cmd = wrk.command("pseudo");
    cmd.arg("name").args(["--increment", "0"]).arg("data.csv");

    wrk.assert_err(&mut cmd);
    let got_err = wrk.output_stderr(&mut cmd);
    assert!(
        got_err.contains("--increment must be greater than 0"),
        "stderr was: {got_err}"
    );
    // usage-level errors must not write any partial output to stdout
    let got_stdout = wrk.stdout::<String>(&mut cmd);
    assert!(
        got_stdout.is_empty(),
        "expected empty stdout, got: {got_stdout:?}"
    );
}
