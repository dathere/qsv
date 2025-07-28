use crate::workdir::Workdir;

#[test]
fn rename() {
    let wrk = Workdir::new("rename");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
        ],
    );

    let mut cmd = wrk.command("rename");
    cmd.arg("cola,colb").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["cola", "colb"],
        svec!["1", "b"],
        svec!["2", "a"],
        svec!["3", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn rename_generic() {
    let wrk = Workdir::new("rename_generic");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
        ],
    );

    let mut cmd = wrk.command("rename");
    cmd.arg("_all_generic").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["_col_1", "_col_2"],
        svec!["1", "b"],
        svec!["2", "a"],
        svec!["3", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn rename_noheaders() {
    let wrk = Workdir::new("rename_noheaders");
    wrk.create(
        "in.csv",
        vec![svec!["1", "b"], svec!["2", "a"], svec!["3", "d"]],
    );

    let mut cmd = wrk.command("rename");
    cmd.arg("cola,colb").arg("--no-headers").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["cola", "colb"],
        svec!["1", "b"],
        svec!["2", "a"],
        svec!["3", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn rename_noheaders_generic() {
    let wrk = Workdir::new("rename_noheaders_generic");
    wrk.create(
        "in.csv",
        vec![svec!["1", "b"], svec!["2", "a"], svec!["3", "d"]],
    );

    let mut cmd = wrk.command("rename");
    cmd.arg("_ALL_Generic").arg("--no-headers").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["_col_1", "_col_2"],
        svec!["1", "b"],
        svec!["2", "a"],
        svec!["3", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn rename_pairs() {
    let wrk = Workdir::new("rename_pairs");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S", "T"],
            svec!["1", "b", "x"],
            svec!["2", "a", "y"],
            svec!["3", "d", "z"],
        ],
    );

    let mut cmd = wrk.command("rename");
    cmd.arg("R,cola,S,colb").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["cola", "colb", "T"],
        svec!["1", "b", "x"],
        svec!["2", "a", "y"],
        svec!["3", "d", "z"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn rename_pairs_single() {
    let wrk = Workdir::new("rename_pairs_single");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
        ],
    );

    let mut cmd = wrk.command("rename");
    cmd.arg("R,cola").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["cola", "S"],
        svec!["1", "b"],
        svec!["2", "a"],
        svec!["3", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn rename_pairs_multiple() {
    let wrk = Workdir::new("rename_pairs_multiple");
    wrk.create(
        "in.csv",
        vec![
            svec!["A", "B", "C", "D"],
            svec!["1", "2", "3", "4"],
            svec!["5", "6", "7", "8"],
        ],
    );

    let mut cmd = wrk.command("rename");
    cmd.arg("A,alpha,C,gamma").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["alpha", "B", "gamma", "D"],
        svec!["1", "2", "3", "4"],
        svec!["5", "6", "7", "8"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn rename_pairs_no_match() {
    let wrk = Workdir::new("rename_pairs_no_match");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
        ],
    );

    let mut cmd = wrk.command("rename");
    cmd.arg("X,cola,Y,colb").arg("in.csv");

    let got = cmd.output().unwrap();
    assert!(!got.status.success());
    let stderr = String::from_utf8_lossy(&got.stderr);
    assert_eq!(
        stderr,
        "usage error: The length of the CSV headers (2) is different from the provided one (4).\n"
    );
}

#[test]
fn rename_pairs_fallback_to_original() {
    let wrk = Workdir::new("rename_pairs_fallback");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
        ],
    );

    let mut cmd = wrk.command("rename");
    cmd.arg("cola,colb,colc").arg("in.csv");

    let got = cmd.output().unwrap();
    assert!(!got.status.success());
    let stderr = String::from_utf8_lossy(&got.stderr);
    assert_eq!(
        stderr,
        "usage error: The length of the CSV headers (2) is different from the provided one (3).\n"
    );
}
