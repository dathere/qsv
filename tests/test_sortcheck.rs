use crate::workdir::Workdir;

#[test]
fn sortcheck_select_notsorted() {
    let wrk = Workdir::new("sortcheck_select_notsorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers")
        .args(["--select", "2"])
        .arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_select_sorted() {
    let wrk = Workdir::new("sortcheck_select_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers")
        .args(["--select", "1"])
        .arg("in.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_select_unsorted() {
    let wrk = Workdir::new("sortcheck_select_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers")
        .args(["--select", "2"])
        .arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_sorted() {
    let wrk = Workdir::new("sortcheck_simple_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers").arg("in.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_simple_unsorted() {
    let wrk = Workdir::new("sortcheck_simple_unsorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_all() {
    let wrk = Workdir::new("sortcheck_simple_all");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--all").arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_all_json() {
    let wrk = Workdir::new("sortcheck_simple_all_json");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--all").arg("--json").arg("in.csv");

    let output = cmd.output().unwrap();
    let got_stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

    assert_eq!(
        got_stdout,
        r#"{"sorted":false,"record_count":9,"unsorted_breaks":2,"dupe_count":-1}
"#
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_json() {
    let wrk = Workdir::new("sortcheck_simple_json");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--json").arg("in.csv");

    let output = cmd.output().unwrap();
    let got_stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

    assert_eq!(
        got_stdout,
        r#"{"sorted":false,"record_count":9,"unsorted_breaks":2,"dupe_count":-1}
"#
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_ignore_case_sorted() {
    let wrk = Workdir::new("sortcheck_ignore_case_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["name"],
            svec!["alpha"],
            svec!["Beta"],
            svec!["gamma"],
            svec!["Delta"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--ignore-case").arg("in.csv");

    wrk.assert_err(&mut cmd);

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");
    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_ignore_case_actually_sorted() {
    let wrk = Workdir::new("sortcheck_ignore_case_actually_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["name"],
            svec!["alpha"],
            svec!["Beta"],
            svec!["Charlie"],
            svec!["delta"],
        ],
    );

    // Lex sort sees uppercase < lowercase, so this is NOT sorted lex.
    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");
    wrk.assert_err(&mut cmd);

    // But it IS sorted ignoring case.
    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--ignore-case").arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_empty() {
    let wrk = Workdir::new("sortcheck_empty");
    wrk.create("in.csv", vec![svec!["col1", "col2"]]);

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");
    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--json").arg("in.csv");
    let output = cmd.output().unwrap();
    let got_stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();
    assert_eq!(
        got_stdout,
        "{\"sorted\":true,\"record_count\":1,\"unsorted_breaks\":0,\"dupe_count\":0}\n"
    );
    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_single_row() {
    let wrk = Workdir::new("sortcheck_single_row");
    wrk.create("in.csv", vec![svec!["col1"], svec!["only"]]);

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_numeric_sorted() {
    let wrk = Workdir::new("sortcheck_numeric_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["n"],
            svec!["2"],
            svec!["10"],
            svec!["30"],
            svec!["200"],
        ],
    );

    // Lex sees "10" < "2", so the file is NOT sorted lex.
    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");
    wrk.assert_err(&mut cmd);

    // But it IS sorted numerically.
    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--numeric").arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_numeric_unsorted() {
    let wrk = Workdir::new("sortcheck_numeric_unsorted");
    wrk.create(
        "in.csv",
        vec![svec!["n"], svec!["2"], svec!["30"], svec!["10"]],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--numeric").arg("in.csv");
    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_natural_sorted() {
    let wrk = Workdir::new("sortcheck_natural_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["item"],
            svec!["item1"],
            svec!["item2"],
            svec!["item10"],
            svec!["item20"],
        ],
    );

    // Lex sees "item10" < "item2", so the file is NOT sorted lex.
    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");
    wrk.assert_err(&mut cmd);

    // But it IS sorted naturally.
    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--natural").arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_natural_unsorted() {
    let wrk = Workdir::new("sortcheck_natural_unsorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["item"],
            svec!["item1"],
            svec!["item10"],
            svec!["item2"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--natural").arg("in.csv");
    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_natural_ignore_case() {
    let wrk = Workdir::new("sortcheck_natural_ignore_case");
    wrk.create(
        "in.csv",
        vec![
            svec!["item"],
            svec!["Item1"],
            svec!["item2"],
            svec!["ITEM10"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--natural").arg("--ignore-case").arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_natural_overrides_numeric() {
    // Mirrors sort/dedup precedence: --natural beats --numeric. A file that
    // is sorted naturally but NOT numerically should pass when both flags
    // are set.
    let wrk = Workdir::new("sortcheck_natural_overrides_numeric");
    wrk.create(
        "in.csv",
        vec![
            svec!["item"],
            svec!["item1"],
            svec!["item2"],
            svec!["item10"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--natural").arg("--numeric").arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_simple_all_json_progressbar() {
    let wrk = Workdir::new("sortcheck_simple_all_json_progressbar");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--all")
        .arg("--json")
        .arg("--progressbar")
        .arg("in.csv");

    let output = cmd.output().unwrap();
    let got_stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

    assert_eq!(
        got_stdout,
        r#"{"sorted":false,"record_count":9,"unsorted_breaks":2,"dupe_count":-1}
"#
    );
    wrk.assert_err(&mut cmd);
}
