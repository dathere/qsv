use crate::workdir::Workdir;

#[test]
fn enumerate() {
    let wrk = Workdir::new("enum");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "index"],
        svec!["a", "13", "0"],
        svec!["b", "24", "1"],
        svec!["c", "72", "2"],
        svec!["d", "7", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_counter() {
    let wrk = Workdir::new("enumerate_counter");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(&["--start", "10"]).arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "index"],
        svec!["a", "13", "10"],
        svec!["b", "24", "11"],
        svec!["c", "72", "12"],
        svec!["d", "7", "13"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_counter_inc() {
    let wrk = Workdir::new("enumerate_counter_inc");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(&["--start", "10"])
        .args(&["--increment", "3"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "index"],
        svec!["a", "13", "10"],
        svec!["b", "24", "13"],
        svec!["c", "72", "16"],
        svec!["d", "7", "19"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_hash() {
    let wrk = Workdir::new("enumerate_hash");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number", "random_text"],
            svec!["a", "13", "this is a test"],
            svec!["b", "24", "the quick brown fox"],
            svec!["c", "72", "jumps over the lazy dog"],
            svec!["d", "7", "I think, therefore I am"],
            svec!["d", "7", "I think, therefore I am"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(&["--hash", "1-"]).arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "random_text", "hash"],
        svec!["a", "13", "this is a test", "16647336487524601331"],
        svec!["b", "24", "the quick brown fox", "16002172227596323886"],
        svec!["c", "72", "jumps over the lazy dog", "10342577706231926068"],
        svec!["d", "7", "I think, therefore I am", "9529353684586443901"],
        svec!["d", "7", "I think, therefore I am", "9529353684586443901"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_hash_intl() {
    let wrk = Workdir::new("enumerate_hash_intl");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number", "random_text"],
            svec!["a", "13", "これはテストです"],
            svec!["b", "24", "el rápido zorro marrón"],
            svec!["c", "72", "跳过懒狗"],
            svec!["c", "72", "howdy"],
            svec!["d", "7", "I thiñk, therefore I am"],
            svec!["d", "7", "I thiñk, therefore I am"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(&["--hash", "1-"]).arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "random_text", "hash"],
        svec!["a", "13", "これはテストです", "8639687352294778884"],
        svec!["b", "24", "el rápido zorro marrón", "13003362524220562232"],
        svec!["c", "72", "跳过懒狗", "5528893140454782773"],
        svec!["c", "72", "howdy", "2885904032770952417"],
        svec!["d", "7", "I thiñk, therefore I am", "17415763469255125069"],
        svec!["d", "7", "I thiñk, therefore I am", "17415763469255125069"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_hash_replace_old_hash() {
    let wrk = Workdir::new("enumerate_replace_old_hash");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number", "random_text", "hash"],
            svec!["a", "13", "this is a test", "1"],
            svec!["b", "24", "the quick brown fox", "2"],
            svec!["c", "72", "jumps over the lazy dog", "3"],
            svec!["d", "7", "I think, therefore I am", "4"],
            svec!["d", "7", "I think, therefore I am", "5"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(&["--hash", "!/hash/"]).arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "random_text", "hash"],
        svec!["a", "13", "this is a test", "16647336487524601331"],
        svec!["b", "24", "the quick brown fox", "16002172227596323886"],
        svec!["c", "72", "jumps over the lazy dog", "10342577706231926068"],
        svec!["d", "7", "I think, therefore I am", "9529353684586443901"],
        svec!["d", "7", "I think, therefore I am", "9529353684586443901"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_hash_replace_old_hash2() {
    let wrk = Workdir::new("enumerate_replace_old_hash2");
    wrk.create(
        "data.csv",
        vec![
            svec!["hash", "letter", "number", "random_text"],
            svec!["1", "a", "13", "this is a test"],
            svec!["2", "b", "24", "the quick brown fox"],
            svec!["3", "c", "72", "jumps over the lazy dog"],
            svec!["4", "d", "7", "I think, therefore I am"],
            svec!["5", "d", "7", "I think, therefore I am"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(&["--hash", "1-"]).arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "random_text", "hash"],
        svec!["a", "13", "this is a test", "16647336487524601331"],
        svec!["b", "24", "the quick brown fox", "16002172227596323886"],
        svec!["c", "72", "jumps over the lazy dog", "10342577706231926068"],
        svec!["d", "7", "I think, therefore I am", "9529353684586443901"],
        svec!["d", "7", "I think, therefore I am", "9529353684586443901"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_hash_regex() {
    let wrk = Workdir::new("enumerate_replace_regex");
    wrk.create(
        "data.csv",
        vec![
            svec!["hash", "letter", "number", "random_text"],
            svec!["1", "a", "13", "this is a test"],
            svec!["2", "b", "24", "the quick brown fox"],
            svec!["3", "c", "72", "jumps over the lazy dog"],
            svec!["4", "d", "7", "I think, therefore I am"],
            svec!["5", "d", "7", "I think, therefore I am"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(&["--hash", "/letter|number|random_text/"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "random_text", "hash"],
        svec!["a", "13", "this is a test", "16647336487524601331"],
        svec!["b", "24", "the quick brown fox", "16002172227596323886"],
        svec!["c", "72", "jumps over the lazy dog", "10342577706231926068"],
        svec!["d", "7", "I think, therefore I am", "9529353684586443901"],
        svec!["d", "7", "I think, therefore I am", "9529353684586443901"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_hash_subset() {
    let wrk = Workdir::new("enumerate_replace_subset");
    wrk.create(
        "data.csv",
        vec![
            svec!["hash", "letter", "number", "random_text"],
            svec!["1", "a", "13", "this is a test"],
            svec!["2", "b", "24", "the quick brown fox"],
            svec!["3", "c", "72", "jumps over the lazy dog"],
            svec!["4", "d", "7", "I think, therefore I am"],
            svec!["5", "d", "7", "I think, therefore I am"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(&["--hash", "3,4"]).arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "random_text", "hash"],
        svec!["a", "13", "this is a test", "5068084879712623948"],
        svec!["b", "24", "the quick brown fox", "16864506042503095386"],
        svec!["c", "72", "jumps over the lazy dog", "10806014713536340838"],
        svec!["d", "7", "I think, therefore I am", "9766316219039859984"],
        svec!["d", "7", "I think, therefore I am", "9766316219039859984"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_hash_reverse() {
    let wrk = Workdir::new("enumerate_replace_reverse");
    wrk.create(
        "data.csv",
        vec![
            svec!["hash", "letter", "number", "random_text"],
            svec!["1", "a", "13", "this is a test"],
            svec!["2", "b", "24", "the quick brown fox"],
            svec!["3", "c", "72", "jumps over the lazy dog"],
            svec!["4", "d", "7", "I think, therefore I am"],
            svec!["5", "d", "7", "I think, therefore I am"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(&["--hash", "_-1"]).arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "random_text", "hash"],
        svec!["a", "13", "this is a test", "4028571812167573426"],
        svec!["b", "24", "the quick brown fox", "8754606222277392071"],
        svec!["c", "72", "jumps over the lazy dog", "15114562622771216744"],
        svec!["d", "7", "I think, therefore I am", "15631341528458705790"],
        svec!["d", "7", "I think, therefore I am", "15631341528458705790"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_hash_regex_not() {
    let wrk = Workdir::new("enumerate_replace_regex_not");
    wrk.create(
        "data.csv",
        vec![
            svec!["hash", "letter", "number", "random_text"],
            svec!["1", "a", "13", "this is a test"],
            svec!["2", "b", "24", "the quick brown fox"],
            svec!["3", "c", "72", "jumps over the lazy dog"],
            svec!["4", "d", "7", "I think, therefore I am"],
            svec!["5", "d", "7", "I think, therefore I am"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(&["--hash", "!/hash/"]).arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "random_text", "hash"],
        svec!["a", "13", "this is a test", "16647336487524601331"],
        svec!["b", "24", "the quick brown fox", "16002172227596323886"],
        svec!["c", "72", "jumps over the lazy dog", "10342577706231926068"],
        svec!["d", "7", "I think, therefore I am", "9529353684586443901"],
        svec!["d", "7", "I think, therefore I am", "9529353684586443901"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_column_name() {
    let wrk = Workdir::new("enum");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.arg("-c").arg("row").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "row"],
        svec!["a", "13", "0"],
        svec!["b", "24", "1"],
        svec!["c", "72", "2"],
        svec!["d", "7", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_constant() {
    let wrk = Workdir::new("enum");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.arg("--constant").arg("test").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "constant"],
        svec!["a", "13", "test"],
        svec!["b", "24", "test"],
        svec!["c", "72", "test"],
        svec!["d", "7", "test"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_constant_null() {
    let wrk = Workdir::new("enum");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.arg("--constant").arg("<NULL>").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "constant"],
        svec!["a", "13", ""],
        svec!["b", "24", ""],
        svec!["c", "72", ""],
        svec!["d", "7", ""],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_copy() {
    let wrk = Workdir::new("enum");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.arg("--copy").arg("number").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "number_copy"],
        svec!["a", "13", "13"],
        svec!["b", "24", "24"],
        svec!["c", "72", "72"],
        svec!["d", "7", "7"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_copy_long_to_short() {
    let wrk = Workdir::new("enumerate_copy_long_to_short");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13 this is a long string"],
            svec!["b", "24 a shorter one"],
            svec!["c", "72 shorter"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.arg("--copy").arg("number").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "number_copy"],
        svec!["a", "13 this is a long string", "13 this is a long string"],
        svec!["b", "24 a shorter one", "24 a shorter one"],
        svec!["c", "72 shorter", "72 shorter"],
        svec!["d", "7", "7"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_copy_name() {
    let wrk = Workdir::new("enum");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.arg("--copy")
        .arg("number")
        .arg("-c")
        .arg("chiffre")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "chiffre"],
        svec!["a", "13", "13"],
        svec!["b", "24", "24"],
        svec!["c", "72", "72"],
        svec!["d", "7", "7"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_uuid7() {
    let wrk = Workdir::new("enumerate_uuid7");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "93"],
            svec!["z", "24"],
            svec!["x", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.arg("--uuid7").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(5, got.len());
    assert_eq!(3, got[0].len());
    // assert that the uuid7 column is monitonically increasing
    assert!(got[1][2] < got[2][2]);
    assert!(got[2][2] < got[3][2]);
    assert!(got[3][2] < got[4][2]);
}

#[test]
fn enumerate_constant_issue_2172_new_column() {
    let wrk = Workdir::new("enumerate_constant_issue_2172_new_column");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "numcol"],
            svec!["Fred", "0"],
            svec!["Joe", "1"],
            svec!["Mary", "2"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.arg("--constant").arg("test").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "numcol", "constant"],
        svec!["Fred", "0", "test"],
        svec!["Joe", "1", "test"],
        svec!["Mary", "2", "test"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_copy_issue_2172_new_column() {
    let wrk = Workdir::new("enumerate_copy_issue_2172_new_column");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "numcol"],
            svec!["Fred", "0"],
            svec!["Joe", "1"],
            svec!["Mary", "2"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(["--copy", "numcol"])
        .args(["-c", "chiffre"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "numcol", "chiffre"],
        svec!["Fred", "0", "0"],
        svec!["Joe", "1", "1"],
        svec!["Mary", "2", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_hash_issue_2172_new_column() {
    let wrk = Workdir::new("enumerate_hash_issue_2172_new_column");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "hash"],
            svec!["Fred", "0"],
            svec!["Joe", "1"],
            svec!["Mary", "2"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(["--hash", "name"])
        .args(["--new-column", "id"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "id"],
        svec!["Fred", "14076493413524835199"],
        svec!["Joe", "17835011426701871598"],
        svec!["Mary", "11375317742157426807"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_hash_issue_2172() {
    let wrk = Workdir::new("enumerate_hash_issue_2172");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "some_other_column"],
            svec!["Fred", "0"],
            svec!["Joe", "1"],
            svec!["Mary", "2"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(["--hash", "name"])
        .args(["--new-column", "id"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "some_other_column", "id"],
        svec!["Fred", "0", "14076493413524835199"],
        svec!["Joe", "1", "17835011426701871598"],
        svec!["Mary", "2", "11375317742157426807"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_hash_no_concat_collision() {
    // Regression test: distinct multi-column rows must not collide in the hash
    // simply because their concatenated bytes are identical
    // (e.g. ["ab","c"] vs ["a","bc"] both flatten to "abc").
    let wrk = Workdir::new("enumerate_hash_no_concat_collision");
    wrk.create(
        "data.csv",
        vec![svec!["x", "y"], svec!["ab", "c"], svec!["a", "bc"]],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(["--hash", "1-"]).arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 3);
    assert_ne!(
        got[1][2], got[2][2],
        "rows ['ab','c'] and ['a','bc'] must hash differently"
    );
}

#[test]
fn enumerate_hash_only_existing_hash_column_errors() {
    // Regression test: when --hash resolves to ONLY the existing "hash"
    // column, the auto-exclusion would leave nothing to hash. Previously
    // this silently re-included the "hash" column via an empty SelectColumns
    // round-trip; now it must error out cleanly.
    let wrk = Workdir::new("enumerate_hash_only_existing_hash_column_errors");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "hash"],
            svec!["Alice", "1"],
            svec!["Bob", "2"],
        ],
    );
    let mut cmd = wrk.command("enum");
    cmd.args(["--hash", "hash"]).arg("data.csv");

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("nothing left to hash"),
        "expected error mentioning 'nothing left to hash', got: {stderr}"
    );
}
