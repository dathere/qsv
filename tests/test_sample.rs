use crate::workdir::Workdir;

#[test]
fn sample_seed() {
    let wrk = Workdir::new("sample_seed");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("5").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["6", "e"],
        svec!["8", "h"],
        svec!["3", "d"],
        svec!["7", "i"],
        svec!["5", "f"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_seed_delimiter() {
    let wrk = Workdir::new("sample_seed_delimiter");
    wrk.create_with_delim(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
        ],
        b'|',
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("5").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R|S"],
        svec!["6|e"],
        svec!["8|h"],
        svec!["3|d"],
        svec!["7|i"],
        svec!["5|f"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_seed_faster() {
    let wrk = Workdir::new("sample_seed_faster");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--rng", "faster"])
        .args(["--seed", "42"])
        .arg("5")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["8", "h"],
        svec!["2", "a"],
        svec!["7", "i"],
        svec!["4", "c"],
        svec!["5", "f"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_seed_secure() {
    let wrk = Workdir::new("sample_seed_secure");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--rng", "cryptosecure"])
        .args(["--seed", "42"])
        .arg("5")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["1", "b"],
        svec!["6", "e"],
        svec!["3", "d"],
        svec!["4", "c"],
        svec!["8", "h"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_seed_url() {
    let wrk = Workdir::new("sample_seed_url");

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"])
        .arg("5")
        .arg("https://github.com/dathere/qsv/raw/master/resources/test/aliases.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        ["position", "title"],
        ["Q107058011", "ambassador to Mauritania"],
        [
            "Q100797227",
            "Minister of Family and Social Services of the Government of the Balearic Islands",
        ],
        [
            "Q106968387",
            "Minister of Research and Universities of the Government of Catalonia",
        ],
        ["Q106918017", "conseller d'Obres Públiques i Urbanisme"],
        [
            "Q106162142",
            "Conseiller aux Infrastructures, au Territoire et à l'Environnement de la Généralité \
             valencienne",
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_percentage_seed_no_index_percentage() {
    let wrk = Workdir::new("sample_percentage_seed_no_index_percentage");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("0.6").arg("in.csv");

    // no error since percentage sampling no longer requires an index
    // though note the results are different even with the same seed and
    // sample size. This is because we use sample_reservoir method, not
    // sample_random_access method
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["6", "e"],
        svec!["8", "h"],
        svec!["3", "d"],
        svec!["7", "i"],
        svec!["8", "h"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_percentage_seed_indexed() {
    let wrk = Workdir::new("sample_indexed");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("0.4").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["4", "c"],
        svec!["5", "f"],
        svec!["6", "e"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_percentage_seed_indexed_faster() {
    let wrk = Workdir::new("sample_indexed_faster");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--rng", "faster"])
        .args(["--seed", "42"])
        .arg("0.4")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["5", "f"],
        svec!["8", "h"],
        svec!["8", "h"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_percentage_seed_indexed_secure() {
    let wrk = Workdir::new("sample_indexed_secure");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--rng", "cryptosecure"])
        .args(["--seed", "42"])
        .arg("0.4")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["1", "b"],
        svec!["3", "d"],
        svec!["8", "h"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_indexed_random_access() {
    let wrk = Workdir::new("sample_indexed_random_access");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["9", "i"],
            svec!["10", "j"],
            svec!["11", "k"],
            svec!["12", "l"],
            svec!["13", "m"],
            svec!["14", "n"],
            svec!["15", "o"],
            svec!["16", "p"],
            svec!["17", "q"],
            svec!["18", "r"],
            svec!["19", "s"],
            svec!["20", "t"],
            svec!["21", "u"],
            svec!["22", "v"],
            svec!["23", "w"],
            svec!["24", "x"],
            svec!["25", "y"],
            svec!["26", "z"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("4").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["7", "i"],
        svec!["19", "s"],
        svec!["22", "v"],
        svec!["24", "x"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_indexed_random_access_faster() {
    let wrk = Workdir::new("sample_indexed_random_access_faster");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["9", "i"],
            svec!["10", "j"],
            svec!["11", "k"],
            svec!["12", "l"],
            svec!["13", "m"],
            svec!["14", "n"],
            svec!["15", "o"],
            svec!["16", "p"],
            svec!["17", "q"],
            svec!["18", "r"],
            svec!["19", "s"],
            svec!["20", "t"],
            svec!["21", "u"],
            svec!["22", "v"],
            svec!["23", "w"],
            svec!["24", "x"],
            svec!["25", "y"],
            svec!["26", "z"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--rng", "faster"])
        .args(["--seed", "42"])
        .arg("4")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["11", "k"],
        svec!["15", "o"],
        svec!["21", "u"],
        svec!["22", "v"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_indexed_random_access_secure() {
    let wrk = Workdir::new("sample_indexed_random_access_secure");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["9", "i"],
            svec!["10", "j"],
            svec!["11", "k"],
            svec!["12", "l"],
            svec!["13", "m"],
            svec!["14", "n"],
            svec!["15", "o"],
            svec!["16", "p"],
            svec!["17", "q"],
            svec!["18", "r"],
            svec!["19", "s"],
            svec!["20", "t"],
            svec!["21", "u"],
            svec!["22", "v"],
            svec!["23", "w"],
            svec!["24", "x"],
            svec!["25", "y"],
            svec!["26", "z"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--rng", "cryptosecure"])
        .args(["--seed", "42"])
        .arg("4")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["1", "b"],
        svec!["3", "d"],
        svec!["7", "i"],
        svec!["10", "j"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_percentage_negative_sample_size_error() {
    let wrk = Workdir::new("sample_negative");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("-5").arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sample_bernoulli_seed() {
    let wrk = Workdir::new("sample_bernoulli_seed");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--bernoulli"])
        .args(["--seed", "42"])
        .arg("0.5")
        .arg("in.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["4", "c"],
        svec!["5", "f"],
        svec!["6", "e"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_bernoulli_seed_faster() {
    let wrk = Workdir::new("sample_bernoulli_seed_faster");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--bernoulli"])
        .args(["--rng", "faster"])
        .args(["--seed", "76"])
        .arg("0.45")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["1", "b"],
        svec!["2", "a"],
        svec!["4", "c"],
        svec!["6", "e"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_bernoulli_seed_secure() {
    let wrk = Workdir::new("sample_bernoulli_seed_secure");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--bernoulli"])
        .args(["--rng", "cryptosecure"])
        .args(["--seed", "42"])
        .arg("0.5")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["R", "S"], svec!["3", "d"]];
    assert_eq!(got, expected);
}

#[test]
fn sample_bernoulli_invalid_probability() {
    let wrk = Workdir::new("sample_bernoulli_invalid");
    wrk.create(
        "in.csv",
        vec![svec!["R", "S"], svec!["1", "b"], svec!["2", "a"]],
    );

    // Test probability > 1.0
    let mut cmd = wrk.command("sample");
    cmd.args(["--bernoulli"]).arg("1.5").arg("in.csv");
    wrk.assert_err(&mut cmd);

    // Test probability <= 0.0
    let mut cmd = wrk.command("sample");
    cmd.args(["--bernoulli"]).arg("0.0").arg("in.csv");
    wrk.assert_err(&mut cmd);

    let mut cmd = wrk.command("sample");
    cmd.args(["--bernoulli"]).arg("-0.5").arg("in.csv");
    wrk.assert_err(&mut cmd);
}

#[test]
fn sample_systematic() {
    let wrk = Workdir::new("sample_systematic");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["9", "g"],
            svec!["10", "j"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--systematic", "first"]).arg("3").arg("in.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["1", "b"],
        svec!["4", "c"],
        svec!["7", "i"],
        svec!["10", "j"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_stratified() {
    let wrk = Workdir::new("sample_stratified");
    wrk.create(
        "in.csv",
        vec![
            svec!["Group", "Value"],
            svec!["A", "1"],
            svec!["A", "2"],
            svec!["A", "3"],
            svec!["B", "4"],
            svec!["B", "5"],
            svec!["B", "6"],
            svec!["C", "7"],
            svec!["C", "8"],
            svec!["C", "9"],
            svec!["C", "10"],
            svec!["C", "11"],
            svec!["D", "12"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--stratified", "Group"])
        .args(["--seed", "42"])
        .arg("2")
        .arg("in.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Group", "Value"],
        svec!["A", "3"],
        svec!["A", "2"],
        svec!["B", "4"],
        svec!["B", "6"],
        svec!["C", "9"],
        svec!["C", "8"],
        svec!["D", "12"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_stratified_large_sample_size() {
    let wrk = Workdir::new("sample_stratified_large_sample_size");
    wrk.create(
        "in.csv",
        vec![
            svec!["Group", "Value"],
            svec!["A", "1"],
            svec!["A", "2"],
            svec!["A", "3"],
            svec!["B", "4"],
            svec!["B", "5"],
            svec!["B", "6"],
            svec!["C", "7"],
            svec!["C", "8"],
            svec!["C", "9"],
            svec!["C", "10"],
            svec!["C", "11"],
            svec!["D", "12"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--stratified", "Group"])
        .args(["--seed", "42"])
        .arg("100")
        .arg("in.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Group", "Value"],
        svec!["A", "1"],
        svec!["A", "2"],
        svec!["A", "3"],
        svec!["B", "4"],
        svec!["B", "5"],
        svec!["B", "6"],
        svec!["C", "7"],
        svec!["C", "8"],
        svec!["C", "9"],
        svec!["C", "10"],
        svec!["C", "11"],
        svec!["D", "12"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_weighted() {
    let wrk = Workdir::new("sample_weighted");
    wrk.create(
        "in.csv",
        vec![
            svec!["ID", "Weight"],
            svec!["1", "10"],
            svec!["2", "20"],
            svec!["3", "30"],
            svec!["4", "40"],
            svec!["5", "50"],
            svec!["6", "60"],
            svec!["7", "70"],
            svec!["8", "80"],
            svec!["9", "90"],
            svec!["10", "100"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--weighted", "ID"])
        .args(["--seed", "42"])
        .arg("4")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ID", "Weight"],
        svec!["5", "50"],
        svec!["6", "60"],
        svec!["9", "90"],
        svec!["10", "100"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_cluster() {
    let wrk = Workdir::new("sample_cluster");
    wrk.create(
        "in.csv",
        vec![
            svec!["Household", "Person", "Age"],
            svec!["H1", "P1", "25"],
            svec!["H1", "P2", "30"],
            svec!["H1", "P3", "35"],
            svec!["H2", "P3", "45"],
            svec!["H2", "P4", "50"],
            svec!["H2", "P5", "55"],
            svec!["H3", "P5", "35"],
            svec!["H3", "P6", "40"],
            svec!["H3", "P7", "45"],
            svec!["H4", "P7", "28"],
            svec!["H4", "P8", "32"],
            svec!["H4", "P9", "36"],
            svec!["H4", "P10", "40"],
            svec!["H5", "P11", "44"],
            svec!["H5", "P12", "48"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--cluster", "Household"])
        .args(["--seed", "42"])
        .arg("2")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Household", "Person", "Age"],
        svec!["H1", "P1", "25"],
        svec!["H1", "P2", "30"],
        svec!["H1", "P3", "35"],
        svec!["H3", "P5", "35"],
        svec!["H3", "P6", "40"],
        svec!["H3", "P7", "45"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_stratified_invalid_column() {
    let wrk = Workdir::new("sample_stratified_invalid");
    wrk.create(
        "in.csv",
        vec![svec!["Group", "Value"], svec!["A", "1"], svec!["B", "2"]],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--stratified", "999"]).arg("1").arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sample_weighted_negative_weights() {
    let wrk = Workdir::new("sample_weighted_negative");
    wrk.create(
        "in.csv",
        vec![
            svec!["ID", "Weight"],
            svec!["1", "-10"],
            svec!["2", "20"],
            svec!["3", "30"],
            svec!["4", "40"],
            svec!["5", "-50"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--weighted", "1"]).arg("1").arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sample_stratified_empty_stratum() {
    let wrk = Workdir::new("sample_stratified_empty");
    wrk.create(
        "in.csv",
        vec![
            svec!["Group", "Value"],
            svec!["A", "1"],
            svec!["", "2"], // empty stratum
            svec!["A", "3"],
            svec!["B", "4"],
            svec!["", "5"], // another empty stratum
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--stratified", "Group"])
        .args(["--seed", "42"])
        .arg("2")
        .arg("in.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Group", "Value"],
        svec!["", "2"],
        svec!["", "5"],
        svec!["A", "1"],
        svec!["A", "3"],
        svec!["B", "4"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_weighted_zero_weights() {
    let wrk = Workdir::new("sample_weighted_zero");
    wrk.create(
        "in.csv",
        vec![
            svec!["ID", "Weight"],
            svec!["1", "0"],
            svec!["2", "0"],
            svec!["3", "30"],
            svec!["4", "0"],
            svec!["5", "50"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--weighted", "Weight"])
        .args(["--seed", "42"])
        .arg("2")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["ID", "Weight"], svec!["3", "30"], svec!["5", "50"]];
    assert_eq!(got, expected);
}

#[test]
fn sample_cluster_single_record() {
    let wrk = Workdir::new("sample_cluster_single");
    wrk.create(
        "in.csv",
        vec![
            svec!["Cluster", "Value"],
            svec!["A", "1"], // single record cluster
            svec!["B", "2"],
            svec!["B", "3"],
            svec!["C", "4"], // single record cluster
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--cluster", "Cluster"])
        .args(["--seed", "42"])
        .arg("2")
        .arg("in.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Cluster", "Value"],
        svec!["A", "1"],
        svec!["B", "2"],
        svec!["B", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_systematic_with_headers() {
    let wrk = Workdir::new("sample_systematic_headers");
    wrk.create(
        "in.csv",
        vec![
            svec!["Header1", "Header2"], // should be preserved
            svec!["1", "a"],
            svec!["2", "b"],
            svec!["3", "c"],
            svec!["4", "d"],
            svec!["5", "e"],
            svec!["6", "f"],
            svec!["7", "g"],
            svec!["8", "h"],
            svec!["9", "i"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--systematic", "first"]).arg("3").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Header1", "Header2"],
        svec!["1", "a"],
        svec!["4", "d"],
        svec!["7", "g"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_systematic_with_headers_random_with_seed() {
    let wrk = Workdir::new("sample_systematic_headers_random_with_seed");
    wrk.create(
        "in.csv",
        vec![
            svec!["Header1", "Header2"], // should be preserved
            svec!["1", "a"],
            svec!["2", "b"],
            svec!["3", "c"],
            svec!["4", "d"],
            svec!["5", "e"],
            svec!["6", "f"],
            svec!["7", "g"],
            svec!["8", "h"],
            svec!["9", "i"],
            svec!["10", "j"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--systematic", "random", "--seed", "65"])
        .arg("4.5")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Header1", "Header2"],
        svec!["5", "e"],
        svec!["9", "i"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_systematic_no_headers() {
    let wrk = Workdir::new("sample_systematic_no_headers");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "a"],
            svec!["2", "b"],
            svec!["3", "c"],
            svec!["4", "d"],
            svec!["5", "e"],
            svec!["6", "f"],
            svec!["7", "g"],
            svec!["8", "h"],
            svec!["9", "i"],
            svec!["10", "j"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--systematic", "first", "--no-headers"])
        .arg("3.3")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["1", "a"], svec!["4", "d"], svec!["7", "g"]];
    assert_eq!(got, expected);
}

#[test]
fn sample_multiple_methods_error() {
    let wrk = Workdir::new("sample_multiple_methods");
    wrk.create(
        "in.csv",
        vec![svec!["ID", "Value"], svec!["1", "a"], svec!["2", "b"]],
    );

    // Test combining bernoulli with systematic
    let mut cmd = wrk.command("sample");
    cmd.args(["--bernoulli", "--systematic", "first"])
        .arg("0.5")
        .arg("in.csv");
    wrk.assert_err(&mut cmd);

    // Test combining weighted with stratified
    let mut cmd = wrk.command("sample");
    cmd.args(["--weighted", "ID", "--stratified", "ID"])
        .arg("1")
        .arg("in.csv");
    wrk.assert_err(&mut cmd);
}

#[test]
fn sample_invalid_rng() {
    let wrk = Workdir::new("sample_invalid_rng");
    wrk.create(
        "in.csv",
        vec![svec!["ID", "Value"], svec!["1", "a"], svec!["2", "b"]],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--rng", "invalid_rng"]).arg("1").arg("in.csv");
    wrk.assert_err(&mut cmd);
}

#[test]
fn sample_systematic_invalid_interval() {
    let wrk = Workdir::new("sample_systematic_invalid_interval");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
        ],
    );

    // Test interval of 0
    let mut cmd = wrk.command("sample");
    cmd.args(["--systematic", "first"]).arg("0").arg("in.csv");
    wrk.assert_err(&mut cmd);

    // Test negative interval
    let mut cmd = wrk.command("sample");
    cmd.args(["--systematic", "first"]).arg("-2").arg("in.csv");
    wrk.assert_err(&mut cmd);
}

#[test]
fn sample_weighted_invalid_weights() {
    let wrk = Workdir::new("sample_weighted_invalid");
    wrk.create(
        "in.csv",
        vec![
            svec!["ID", "Weight"],
            svec!["1", "abc"], // non-numeric weight -> treated as 0
            svec!["2", "20.5"],
            svec!["3", ""], // empty weight -> treated as 0
            svec!["4", "40"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--weighted", "Weight"])
        .args(["--seed", "42"])
        .arg("2")
        .arg("in.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Records with invalid weights are treated as having zero weight
    // we requested two samples but returning only one as we ran out of records
    let expected = vec![svec!["ID", "Weight"], svec!["4", "40"]];
    assert_eq!(got, expected);
}

#[test]
fn sample_cluster_too_many_clusters_with_stats_cache() {
    let wrk = Workdir::new("sample_cluster_too_many_with_stats_cache");
    wrk.create(
        "in.csv",
        vec![
            svec!["Cluster", "Value"],
            svec!["A", "1"],
            svec!["B", "2"],
            svec!["C", "3"],
        ],
    );

    // create stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["in.csv", "-E", "--stats-jsonl"]);

    wrk.assert_success(&mut stats_cmd);

    // Request more clusters than exist, this error only happens with a stats cache
    let mut cmd = wrk.command("sample");
    cmd.args(["--cluster", "Cluster"])
        .args(["--seed", "42"])
        .arg("5") // Only 3 clusters exist
        .arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sample_stratified_with_delimiter() {
    let wrk = Workdir::new("sample_stratified_delimiter");
    wrk.create_with_delim(
        "in.csv",
        vec![
            svec!["Group", "Value"],
            svec!["A", "1"],
            svec!["A", "2"],
            svec!["B", "3"],
            svec!["B", "4"],
        ],
        b'|',
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--stratified", "Group"])
        .args(["--seed", "42"])
        .args(["--delimiter", "|"])
        .arg("1")
        .arg("in.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["Group|Value"], svec!["A|2"], svec!["B|3"]];
    assert_eq!(got, expected);
}

#[test]
fn sample_weighted_all_zero_weights() {
    let wrk = Workdir::new("sample_weighted_all_zero");
    wrk.create(
        "in.csv",
        vec![
            svec!["ID", "Weight"],
            svec!["1", "0"],
            svec!["2", "0"],
            svec!["3", "0"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--weighted", "Weight"])
        .args(["--seed", "42"])
        .arg("2")
        .arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sample_systematic_fractional() {
    let wrk = Workdir::new("sample_systematic_fractional");
    wrk.create(
        "in.csv",
        vec![
            svec!["ID", "Value"],
            svec!["1", "a"],
            svec!["2", "b"],
            svec!["3", "c"],
            svec!["4", "d"],
            svec!["5", "e"],
            svec!["6", "f"],
            svec!["7", "g"],
            svec!["8", "h"],
            svec!["9", "i"],
            svec!["10", "j"],
        ],
    );

    // Test with fractional interval (3.5 means every 3rd record and 50% of population)
    let mut cmd = wrk.command("sample");
    cmd.args(["--systematic", "first"])
        .args(["--seed", "42"])
        .arg("3.5")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ID", "Value"],
        svec!["1", "a"],
        svec!["4", "d"],
        svec!["7", "g"],
        svec!["10", "j"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_multiple_sampling_methods_error() {
    let wrk = Workdir::new("sample_multiple_methods_error");
    wrk.create(
        "in.csv",
        vec![svec!["ID", "Value"], svec!["1", "a"], svec!["2", "b"]],
    );

    // Test combining cluster with weighted
    let mut cmd = wrk.command("sample");
    cmd.args(["--cluster", "ID", "--weighted", "Value"])
        .arg("1")
        .arg("in.csv");
    wrk.assert_err(&mut cmd);

    // Test combining systematic with stratified
    let mut cmd = wrk.command("sample");
    cmd.args(["--systematic", "first", "--stratified", "ID"])
        .arg("1")
        .arg("in.csv");
    wrk.assert_err(&mut cmd);
}

#[test]
fn sample_remote_bernoulli_streaming_standard_rng() {
    let wrk = Workdir::new("sample_remote_bernoulli_streaming_standard_rng");

    // Use a small test file from the qsv repository that we know supports range requests
    let test_url = "https://raw.githubusercontent.com/dathere/qsv/refs/heads/master/resources/test/NYC311-50k.csv";

    let mut cmd = wrk.command("sample");
    cmd.args(["--bernoulli"])
        .args(["--seed", "42"])
        .arg("0.3") // 30% probability
        .arg(test_url);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Verify we got the header
    assert_eq!(
        got[0],
        vec![
            "Unique Key",
            "Created Date",
            "Closed Date",
            "Agency",
            "Agency Name",
            "Complaint Type",
            "Descriptor",
            "Location Type",
            "Incident Zip",
            "Incident Address",
            "Street Name",
            "Cross Street 1",
            "Cross Street 2",
            "Intersection Street 1",
            "Intersection Street 2",
            "Address Type",
            "City",
            "Landmark",
            "Facility Type",
            "Status",
            "Due Date",
            "Resolution Description",
            "Resolution Action Updated Date",
            "Community Board",
            "BBL",
            "Borough",
            "X Coordinate (State Plane)",
            "Y Coordinate (State Plane)",
            "Open Data Channel Type",
            "Park Facility Name",
            "Park Borough",
            "Vehicle Type",
            "Taxi Company Borough",
            "Taxi Pick Up Location",
            "Bridge Highway Name",
            "Bridge Highway Direction",
            "Road Ramp",
            "Bridge Highway Segment",
            "Latitude",
            "Longitude",
            "Location",
        ]
    );

    // Verify we got some records (exact count as we're using a seed)
    assert!(got.len() == 14_938);

    // Verify the structure of sampled records
    for record in got.iter().skip(1) {
        assert_eq!(record.len(), 41); // Each record should have position and title
        assert!(!record[0].is_empty()); // Unique Key should not be empty
        assert!(!record[1].is_empty()); // Created Date should not be empty
    }
}

#[test]
fn sample_remote_bernoulli_streaming_cryptosecure() {
    let wrk = Workdir::new("sample_remote_bernoulli_streaming_cryptosecure");

    // Use a small test file from the qsv repository that we know supports range requests
    let test_url = "https://raw.githubusercontent.com/dathere/qsv/refs/heads/master/resources/test/NYC311-50k.csv";

    let mut cmd = wrk.command("sample");
    cmd.args(["--bernoulli"])
        .args(["--rng", "cryptosecure"])
        .args(["--seed", "42"])
        .arg("0.3") // 30% probability
        .arg(test_url);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Verify we got the header
    assert_eq!(
        got[0],
        vec![
            "Unique Key",
            "Created Date",
            "Closed Date",
            "Agency",
            "Agency Name",
            "Complaint Type",
            "Descriptor",
            "Location Type",
            "Incident Zip",
            "Incident Address",
            "Street Name",
            "Cross Street 1",
            "Cross Street 2",
            "Intersection Street 1",
            "Intersection Street 2",
            "Address Type",
            "City",
            "Landmark",
            "Facility Type",
            "Status",
            "Due Date",
            "Resolution Description",
            "Resolution Action Updated Date",
            "Community Board",
            "BBL",
            "Borough",
            "X Coordinate (State Plane)",
            "Y Coordinate (State Plane)",
            "Open Data Channel Type",
            "Park Facility Name",
            "Park Borough",
            "Vehicle Type",
            "Taxi Company Borough",
            "Taxi Pick Up Location",
            "Bridge Highway Name",
            "Bridge Highway Direction",
            "Road Ramp",
            "Bridge Highway Segment",
            "Latitude",
            "Longitude",
            "Location",
        ]
    );

    // Verify we got some records (exact count as we're using a seed)
    assert!(got.len() == 14_815);

    // Verify the structure of sampled records
    for record in got.iter().skip(1) {
        assert_eq!(record.len(), 41); // Each record should have position and title
        assert!(!record[0].is_empty()); // Unique Key should not be empty
        assert!(!record[1].is_empty()); // Created Date should not be empty
    }
}
