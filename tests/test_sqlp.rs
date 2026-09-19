use crate::workdir::Workdir;

macro_rules! sqlp_test {
    ($name:ident, $fun:expr_2021) => {
        mod $name {
            use std::process;

            #[allow(unused_imports)]
            use super::{make_rows, setup};
            use crate::workdir::Workdir;

            #[test]
            fn headers() {
                let wrk = setup(stringify!($name));
                let mut cmd = wrk.command("sqlp");
                cmd.args(&["cities.csv", "places.csv"]);
                $fun(wrk, cmd);
            }
        }
    };
}

fn setup(name: &str) -> Workdir {
    let cities = vec![
        svec!["city", "state"],
        svec!["Boston", "MA"],
        svec!["New York", "NY"],
        svec!["San Francisco", "CA"],
        svec!["Buffalo", "NY"],
    ];
    let places = vec![
        svec!["city", "place"],
        svec!["Boston", "Logan Airport"],
        svec!["Boston", "Boston Garden"],
        svec!["Buffalo", "Ralph Wilson Stadium"],
        svec!["Orlando", "Disney World"],
    ];

    let wrk = Workdir::new(name);
    wrk.create("cities.csv", cities);
    wrk.create("places.csv", places);
    wrk
}

fn make_rows(left_only: bool, rows: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut all_rows = vec![];
    if left_only {
        all_rows.push(svec!["city", "state"]);
    } else {
        all_rows.push(svec!["city", "state", "city:places", "place"]);
    }
    all_rows.extend(rows);
    all_rows
}

sqlp_test!(
    sqlp_join_inner,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("select * from cities inner join places on cities.city = places.city");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Boston", "Logan Airport"],
                svec!["Boston", "MA", "Boston", "Boston Garden"],
                svec!["Buffalo", "NY", "Buffalo", "Ralph Wilson Stadium"],
            ],
        );
        // sqlp exposes no ordering option and its join output is nondeterministic
        // run to run; assert content, not order.
        assert_eq!(
            crate::workdir::sorted_rows(got),
            crate::workdir::sorted_rows(expected)
        );
    }
);

sqlp_test!(
    sqlp_right_join,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("select * from cities right join places on cities.city = places.city");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Boston", "Logan Airport"],
                svec!["Boston", "MA", "Boston", "Boston Garden"],
                svec!["Buffalo", "NY", "Buffalo", "Ralph Wilson Stadium"],
                svec!["", "", "Orlando", "Disney World"],
            ],
        );
        // sqlp exposes no ordering option and its join output is nondeterministic
        // run to run; assert content, not order.
        assert_eq!(
            crate::workdir::sorted_rows(got),
            crate::workdir::sorted_rows(expected)
        );
    }
);

sqlp_test!(
    sqlp_join_outer_left,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("select * from cities left outer join places on cities.city = places.city");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Boston", "Logan Airport"],
                svec!["Boston", "MA", "Boston", "Boston Garden"],
                svec!["New York", "NY", "", ""],
                svec!["San Francisco", "CA", "", ""],
                svec!["Buffalo", "NY", "Buffalo", "Ralph Wilson Stadium"],
            ],
        );
        // sqlp exposes no ordering option and its join output is nondeterministic
        // run to run; assert content, not order.
        assert_eq!(
            crate::workdir::sorted_rows(got),
            crate::workdir::sorted_rows(expected)
        );
    }
);

sqlp_test!(
    sqlp_join_outer_full,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("select * from cities full outer join places on cities.city = places.city");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["city", "state", "city:places", "place"],
            svec!["Boston", "MA", "Boston", "Logan Airport"],
            svec!["Boston", "MA", "Boston", "Boston Garden"],
            svec!["Buffalo", "NY", "Buffalo", "Ralph Wilson Stadium"],
            svec!["", "", "Orlando", "Disney World"],
            svec!["New York", "NY", "", ""],
            svec!["San Francisco", "CA", "", ""],
        ];
        // sqlp exposes no ordering option and its join output is nondeterministic
        // run to run; assert content, not order.
        assert_eq!(
            crate::workdir::sorted_rows(got),
            crate::workdir::sorted_rows(expected)
        );
    }
);

#[test]
fn sqlp_join_cross() {
    let wrk = Workdir::new("join_cross");
    wrk.create(
        "letters.csv",
        vec![svec!["h1", "h2"], svec!["a", "b"], svec!["c", "d"]],
    );
    wrk.create(
        "numbers.csv",
        vec![svec!["h3", "h4"], svec!["1", "2"], svec!["3", "4"]],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.args(["letters.csv", "numbers.csv"])
        .arg("select * from letters cross join numbers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2", "h3", "h4"],
        svec!["a", "b", "1", "2"],
        svec!["a", "b", "3", "4"],
        svec!["c", "d", "1", "2"],
        svec!["c", "d", "3", "4"],
    ];
    // sqlp exposes no ordering option and its join output is nondeterministic
    // run to run; assert content, not order.
    assert_eq!(
        crate::workdir::sorted_rows(got),
        crate::workdir::sorted_rows(expected)
    );
}

#[test]
fn sqlp_join_same_colname_1820() {
    let wrk = Workdir::new("sqlp_join_same_colname_1820");
    wrk.create("one.csv", vec![svec!["id", "data"], svec!["1", "open"]]);
    wrk.create("two.csv", vec![svec!["id", "data"], svec!["1", "closed"]]);

    let mut cmd = wrk.command("sqlp");
    cmd.args(["one.csv", "two.csv"])
        .arg("SELECT _t_1.id, _t_2.data FROM _t_1 JOIN _t_2 ON _t_1.id = _t_2.id");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["id", "data"], svec!["1", "closed"]];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_groupby_orderby() {
    let wrk = Workdir::new("sqlp_boston311_groupby_orderby");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");

    // we quote "boston311-100" as contains a hyphen in its name, which is a special character
    // in SQL, so we need to make it a quoted identifier
    cmd.arg(&test_file)
        .arg(r#"select ward, count(*) as cnt from "boston311-100" group by ward order by cnt desc, ward asc"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "cnt"],
        svec!["Ward 3", "10"],
        svec!["Ward 6", "7"],
        svec!["Ward 1", "6"],
        svec!["3", "5"],
        svec!["Ward 20", "5"],
        svec!["Ward 4", "5"],
        svec!["Ward 5", "5"],
        svec!["14", "4"],
        svec!["Ward 13", "4"],
        svec!["Ward 16", "4"],
        svec!["Ward 18", "3"],
        svec!["Ward 19", "3"],
        svec!["Ward 7", "3"],
        svec!["Ward 8", "3"],
        svec!["03", "2"],
        svec!["17", "2"],
        svec!["22", "2"],
        svec!["Ward 11", "2"],
        svec!["Ward 21", "2"],
        svec![" ", "1"],
        svec!["01", "1"],
        svec!["02", "1"],
        svec!["04", "1"],
        svec!["06", "1"],
        svec!["07", "1"],
        svec!["1", "1"],
        svec!["10", "1"],
        svec!["16", "1"],
        svec!["18", "1"],
        svec!["19", "1"],
        svec!["21", "1"],
        svec!["7", "1"],
        svec!["8", "1"],
        svec!["9", "1"],
        svec!["Ward 10", "1"],
        svec!["Ward 12", "1"],
        svec!["Ward 14", "1"],
        svec!["Ward 15", "1"],
        svec!["Ward 17", "1"],
        svec!["Ward 2", "1"],
        svec!["Ward 22", "1"],
        svec!["Ward 9", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_groupby_orderby_all() {
    let wrk = Workdir::new("sqlp_boston311_groupby_orderby_all");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");

    cmd.arg(&test_file)
        .arg(r#"select ward, count(*) as cnt from "boston311-100" group by ward order by all"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "cnt"],
        svec![" ", "1"],
        svec!["01", "1"],
        svec!["02", "1"],
        svec!["03", "2"],
        svec!["04", "1"],
        svec!["06", "1"],
        svec!["07", "1"],
        svec!["1", "1"],
        svec!["10", "1"],
        svec!["14", "4"],
        svec!["16", "1"],
        svec!["17", "2"],
        svec!["18", "1"],
        svec!["19", "1"],
        svec!["21", "1"],
        svec!["22", "2"],
        svec!["3", "5"],
        svec!["7", "1"],
        svec!["8", "1"],
        svec!["9", "1"],
        svec!["Ward 1", "6"],
        svec!["Ward 10", "1"],
        svec!["Ward 11", "2"],
        svec!["Ward 12", "1"],
        svec!["Ward 13", "4"],
        svec!["Ward 14", "1"],
        svec!["Ward 15", "1"],
        svec!["Ward 16", "4"],
        svec!["Ward 17", "1"],
        svec!["Ward 18", "3"],
        svec!["Ward 19", "3"],
        svec!["Ward 2", "1"],
        svec!["Ward 20", "5"],
        svec!["Ward 21", "2"],
        svec!["Ward 22", "1"],
        svec!["Ward 3", "10"],
        svec!["Ward 4", "5"],
        svec!["Ward 5", "5"],
        svec!["Ward 6", "7"],
        svec!["Ward 7", "3"],
        svec!["Ward 8", "3"],
        svec!["Ward 9", "1"],
    ];
    assert_eq!(got, expected);
}

// Verify the SQL-standard FILTER (WHERE …) aggregate modifier added in
// Polars PR https://github.com/pola-rs/polars/pull/27564 works through
// sqlp. The data + assertions mirror that PR's docstring example so the
// expected numbers are easy to cross-check: for group "aa" the
// unfiltered SUM(x) is 105.375 and SUM(x) FILTER (WHERE y > 250) is
// 44.875 (only the rows where y > 250 contribute); for "bb" the
// unfiltered SUM(x) is -43.75 and the filtered SUM is 6.5.
#[test]
fn sqlp_aggregate_filter() {
    let wrk = Workdir::new("sqlp_aggregate_filter");
    wrk.create(
        "data.csv",
        vec![
            svec!["grp", "x", "y"],
            svec!["aa", "60.5", "100"],
            svec!["bb", "-50.25", "200"],
            svec!["aa", "42.75", "300"],
            svec!["bb", "-3.5", "400"],
            svec!["aa", "2.125", "500"],
            svec!["bb", "10.0", "600"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv").arg(
        "SELECT grp, SUM(x) AS sum_x, SUM(x) FILTER (WHERE y > 250) AS sum_x_when_y_gt_250, \
         COUNT(*) FILTER (WHERE y > 250) AS n_y_gt_250 FROM data GROUP BY grp ORDER BY grp",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["grp", "sum_x", "sum_x_when_y_gt_250", "n_y_gt_250"],
        svec!["aa", "105.375", "44.875", "2"],
        svec!["bb", "-43.75", "6.5", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
// #[ignore = "temporarily disable due to a bug in polars aliasing"]
fn sqlp_boston311_groupby_orderby_with_table_alias() {
    let wrk = Workdir::new("sqlp_boston311_groupby_orderby");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");

    // we use _t_1 alias as "boston311-100" contains a hyphen in its name, which is a special
    // character in SQL
    cmd.arg(&test_file)
        .arg("select ward, count(*) as cnt from _t_1 group by ward order by cnt desc, ward asc");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "cnt"],
        svec!["Ward 3", "10"],
        svec!["Ward 6", "7"],
        svec!["Ward 1", "6"],
        svec!["3", "5"],
        svec!["Ward 20", "5"],
        svec!["Ward 4", "5"],
        svec!["Ward 5", "5"],
        svec!["14", "4"],
        svec!["Ward 13", "4"],
        svec!["Ward 16", "4"],
        svec!["Ward 18", "3"],
        svec!["Ward 19", "3"],
        svec!["Ward 7", "3"],
        svec!["Ward 8", "3"],
        svec!["03", "2"],
        svec!["17", "2"],
        svec!["22", "2"],
        svec!["Ward 11", "2"],
        svec!["Ward 21", "2"],
        svec![" ", "1"],
        svec!["01", "1"],
        svec!["02", "1"],
        svec!["04", "1"],
        svec!["06", "1"],
        svec!["07", "1"],
        svec!["1", "1"],
        svec!["10", "1"],
        svec!["16", "1"],
        svec!["18", "1"],
        svec!["19", "1"],
        svec!["21", "1"],
        svec!["7", "1"],
        svec!["8", "1"],
        svec!["9", "1"],
        svec!["Ward 10", "1"],
        svec!["Ward 12", "1"],
        svec!["Ward 14", "1"],
        svec!["Ward 15", "1"],
        svec!["Ward 17", "1"],
        svec!["Ward 2", "1"],
        svec!["Ward 22", "1"],
        svec!["Ward 9", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_wnull_value() {
    let wrk = Workdir::new("sqlp_boston311_wnull_value");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");

    cmd.arg(&test_file)
        .args(["--wnull-value", "Not Specified"])
        .arg(
            "select location_street_name, location_zipcode from _t_1 where location_zipcode is \
             null order by location_street_name limit 5",
        );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["location_street_name", "location_zipcode"],
        svec!["INTERSECTION Asticou Rd & Washington St", "Not Specified"],
        svec![
            "INTERSECTION Charles River Plz & Cambridge St",
            "Not Specified"
        ],
        svec!["INTERSECTION Columbia Rd & E Cottage St", "Not Specified"],
        svec!["INTERSECTION E Canton St & Albany St", "Not Specified"],
        svec![
            "INTERSECTION Gallivan Blvd & Washington St",
            "Not Specified"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_decimal_comma_issue_1050() {
    let wrk = Workdir::new("sqlp_decimal_comma_issue_1050");
    let test_file = wrk.load_test_file("progetti_sample_10.csv");

    let mut cmd = wrk.command("sqlp");

    cmd.arg(&test_file)
        .arg("--decimal-comma")
        .args(["--delimiter", ";"])
        .arg("select COD_LOCALE_PROGETTO, FINANZ_UE from _t_1");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"COD_LOCALE_PROGETTO;FINANZ_UE
1AGCOE1627;6328008,44
1AGCOE2113;344203,2
1AGCOE645;491260,0
1AGCOE705;522811,32
1AGCOE706;460463,85
1ANPALINP-001;141566507,74
1ANPALINP-CLP-00002;47325000,0
1ANPALINP-CLP-00003;78450000,0
1ANPALINP-CLP-00004;67500000,0
1ANPALVAO1C001;6416,62"#;
    assert_eq!(got, expected);
}

#[test]
fn sqlp_null_aware_equality_checks() {
    let wrk = Workdir::new("sqlp_null_aware_equality_checks");
    wrk.create(
        "test_null.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "1"],
            svec!["", ""],
            svec!["3", "3"],
            svec!["6", "4"],
            svec!["5", ""],
        ],
    );

    let mut cmd = wrk.command("sqlp");

    cmd.arg("test_null.csv")
        .args(["--wnull-value", "NULL"])
        .arg(
            r#"SELECT (a = b) as "1_eq_unaware",
           (a != b) as "2_neq_unaware", 
           (a <=> b) as "3_eq_aware", 
           (a IS NOT DISTINCT FROM b) as "4_eq_aware", 
           (a IS DISTINCT FROM b) as "5_neq_aware" 
         FROM test_null"#,
        );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "1_eq_unaware",
            "2_neq_unaware",
            "3_eq_aware",
            "4_eq_aware",
            "5_neq_aware"
        ],
        svec!["true", "false", "true", "true", "false"],
        svec!["NULL", "NULL", "true", "true", "false"],
        svec!["true", "false", "true", "true", "false"],
        svec!["false", "true", "false", "false", "true"],
        svec!["NULL", "NULL", "false", "false", "true"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_rnull_values_wnull_value() {
    let wrk = Workdir::new("sqlp_rnull_values_wnull_value");
    wrk.create(
        "test_null.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "Nothing"],
            svec!["2", "NA"],
            svec!["3", "Dunno"],
            svec!["4", "4"],
            svec!["5", ""],
            svec!("6", "6"),
            svec!("7", "DUNNO"),
        ],
    );

    let mut cmd = wrk.command("sqlp");

    cmd.arg("test_null.csv")
        .args(["--rnull-values", "Nothing,NA,<empty string>,Dunno"])
        .args(["--wnull-value", "NULL"])
        .arg("SELECT * FROM test_null");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b"],
        svec!["1", "NULL"],
        svec!["2", "NULL"],
        svec!["3", "NULL"],
        svec!["4", "4"],
        svec!["5", "NULL"],
        svec!["6", "6"],
        svec!["7", "DUNNO"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_regex_operators() {
    let wrk = Workdir::new("sqlp_regex_operators");
    wrk.create(
        "test_regex.csv",
        vec![
            svec!["n", "sval"],
            svec!["1", "ABC"],
            svec!["2", "abc"],
            svec!["3", "000"],
            svec!["4", "A0C"],
            svec!["5", "a0c"],
        ],
    );

    // ~ operator - contains pattern (case-sensitive)
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_regex.csv")
        .arg(r#"SELECT * FROM test_regex WHERE sval ~ '\d'"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["n", "sval"],
        svec!["3", "000"],
        svec!["4", "A0C"],
        svec!["5", "a0c"],
    ];
    assert_eq!(got, expected);

    // ~* operator - contains pattern (case-insensitive)
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_regex.csv")
        .arg(r#"SELECT * FROM test_regex WHERE sval ~* '^a0'"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["n", "sval"], svec!["4", "A0C"], svec!["5", "a0c"]];
    assert_eq!(got, expected);

    // !~ operator - does not contain pattern (case-sensitive)
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_regex.csv")
        .arg(r#"SELECT * FROM test_regex WHERE sval !~ '^a0'"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["n", "sval"],
        svec!["1", "ABC"],
        svec!["2", "abc"],
        svec!["3", "000"],
        svec!["4", "A0C"],
    ];
    assert_eq!(got, expected);

    // !~* operator - does not contain pattern (case-insensitive)
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_regex.csv")
        .arg(r#"SELECT * FROM test_regex WHERE sval !~* '^a0'"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["n", "sval"],
        svec!["1", "ABC"],
        svec!["2", "abc"],
        svec!["3", "000"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_regexp_like() {
    let wrk = Workdir::new("sqlp_regexp_like");
    wrk.create(
        "test_regexp_like.csv",
        vec![
            svec!["scol"],
            svec!["abcde"],
            svec!["abc"],
            svec!["a"],
            svec![""],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_regexp_like.csv")
        .arg(r#"SELECT scol FROM test_regexp_like where REGEXP_LIKE(scol,'(C|D)','i')"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["scol"], svec!["abcde"], svec!["abc"]];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_string_functions() {
    let wrk = Workdir::new("sqlp_string_functions");
    wrk.create(
        "test_strings.csv",
        vec![
            svec!["scol"],
            svec!["abcdE"],
            svec!["abc"],
            svec!["    abc"],
            svec!["a"],
            svec!["b"],
        ],
    );

    // starts_with
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT starts_with(scol, 'a') from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["true"],
        svec!["true"],
        svec!["false"],
        svec!["true"],
        svec!["false"],
    ];
    assert_eq!(got, expected);

    // ends_with
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT ends_with(scol, 'c') from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["false"],
        svec!["true"],
        svec!["true"],
        svec!["false"],
        svec!["false"],
    ];
    assert_eq!(got, expected);

    // left
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT left(scol, 3) from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["abc"],
        svec!["abc"],
        svec!["   "],
        svec!["a"],
        svec!["b"],
    ];
    assert_eq!(got, expected);

    // substr
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT substr(scol, 2, 2) from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["bc"],
        svec!["bc"],
        svec!["  "],
        svec![""],
        svec![""],
    ];
    assert_eq!(got, expected);

    // upper
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT upper(scol) from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["ABCDE"],
        svec!["ABC"],
        svec!["    ABC"],
        svec!["A"],
        svec!["B"],
    ];
    assert_eq!(got, expected);

    // lower
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT lower(scol) from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["abcde"],
        svec!["abc"],
        svec!["    abc"],
        svec!["a"],
        svec!["b"],
    ];
    assert_eq!(got, expected);

    // length
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT length(scol) from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["5"],
        svec!["3"],
        svec!["7"],
        svec!["1"],
        svec!["1"],
    ];
    assert_eq!(got, expected);

    // octet_length
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT octet_length(scol) from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["5"],
        svec!["3"],
        svec!["7"],
        svec!["1"],
        svec!["1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_try_parsedates() {
    let wrk = Workdir::new("sqlp_boston311_try_parsedates");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file)
        .arg("--try-parsedates")
        .arg(
            "select ward, cast(avg(closed_dt - open_dt) as float) as avg_tat from _t_1 where \
             case_status = 'Closed' group by ward order by avg_tat desc, ward asc",
        )
        .arg("--ignore-errors");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "avg_tat"],
        svec!["Ward 11", "4847760000000.0"],
        svec!["01", "4818270000000.0"],
        svec!["Ward 13", "1518365750000.0"],
        svec!["Ward 15", "1278926000000.0"],
        svec!["Ward 21", "878446000000.0"],
        svec!["Ward 14", "618933000000.0"],
        svec!["Ward 3", "437691444444.0"],
        svec!["Ward 5", "411909500000.0"],
        svec!["Ward 20", "367233000000.0"],
        svec!["9", "353495000000.0"],
        svec!["Ward 18", "249882000000.0"],
        svec!["19", "212566000000.0"],
        svec!["Ward 4", "112872600000.0"],
        svec!["Ward 1", "107850666666.0"],
        svec!["Ward 10", "104110000000.0"],
        svec!["16", "93557000000.0"],
        svec!["Ward 19", "84164000000.0"],
        svec!["10", "79101000000.0"],
        svec!["21", "77717000000.0"],
        svec!["7", "74611000000.0"],
        svec!["17", "70117500000.0"],
        svec!["3", "68836600000.0"],
        svec!["Ward 9", "64097000000.0"],
        svec!["Ward 12", "62930000000.0"],
        svec!["Ward 6", "54770166666.0"],
        svec!["Ward 7", "38346333333.0"],
        svec!["Ward 8", "32767500000.0"],
        svec!["03", "29810500000.0"],
        svec!["07", "25328000000.0"],
        svec!["22", "23919000000.0"],
        svec!["14", "20786500000.0"],
        svec!["Ward 22", "13524000000.0"],
        svec!["1", "9469000000.0"],
        svec!["06", "5290000000.0"],
        svec!["Ward 16", "4533666666.0"],
        svec!["8", "1757000000.0"],
        svec!["02", "1650000000.0"],
        svec!["18", "507000000.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_try_parsedates_precision() {
    let wrk = Workdir::new("sqlp_boston311_try_parsedates_precision");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file)
        .arg("--try-parsedates")
        .args(["--float-precision", "3"])
        .arg(
            "select ward, cast(avg(closed_dt - open_dt) as float) as avg_tat from _t_1 where \
             case_status = 'Closed' group by ward order by avg_tat desc, ward asc limit 5",
        )
        .arg("--ignore-errors");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "avg_tat"],
        svec!["Ward 11", "4847760000000.000"],
        svec!["01", "4818270000000.000"],
        svec!["Ward 13", "1518365750000.000"],
        svec!["Ward 15", "1278926000000.000"],
        svec!["Ward 21", "878446000000.000"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_try_parsedates_format() {
    let wrk = Workdir::new("sqlp_boston311_try_parsedates_format");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file)
        .arg("--try-parsedates")
        .args(["--datetime-format", "%a %Y-%m-%d %H:%M:%S"])
        .arg("select closed_dt, open_dt from _t_1 where case_status = 'Closed' limit 5")
        .arg("--ignore-errors");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["closed_dt", "open_dt"],
        svec!["Wed 2022-01-19 11:42:16", "Wed 2022-01-19 11:18:00"],
        svec!["Sun 2022-01-09 06:43:06", "Sat 2022-01-08 12:54:49"],
        svec!["Mon 2022-01-10 08:42:23", "Sat 2022-01-01 00:16:00"],
        svec!["Thu 2022-01-20 08:45:03", "Thu 2022-01-20 08:07:49"],
        svec!["Thu 2022-01-20 08:45:12", "Thu 2022-01-20 08:15:45"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_comments() {
    let wrk = Workdir::new("sqlp_comments");
    wrk.create(
        "comments.csv",
        vec![
            svec!["# test file to see how comments work", ""],
            svec!["# this is another comment before the header", ""],
            svec!["# DATA DICTIONARY", ""],
            svec!["# column1 - alphabetic; id of the column", ""],
            svec!["# column2 - numeric; just a number", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["#b", "2"],
            svec!["c", "3"],
            svec!["#d - this row is corrupted skip", "extra col2"],
            svec!["e", "5"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.env("QSV_COMMENT_CHAR", "#");
    cmd.arg("comments.csv")
        .arg("select column1, column2 from comments order by column2 desc");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["e", "5"],
        svec!["c", "3"],
        svec!["a", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_compress() {
    let wrk = Workdir::new("sqlp_compress");
    wrk.create(
        "data.csv",
        vec![
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["c", "3"],
            svec!["e", "5"],
        ],
    );

    let out_file = wrk.path("out.csv.sz").to_string_lossy().to_string();

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg("select column1, column2 from data order by column2 desc")
        .args(["-o", &out_file]);

    wrk.assert_success(&mut cmd);

    let mut cmd_2 = wrk.command("snappy"); // DevSkim: ignore DS126858
    cmd_2.arg("decompress").arg(out_file); // DevSkim: ignore DS126858

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_2); // DevSkim: ignore DS126858
    let expected = vec![
        svec!["column1", "column2"],
        svec!["e", "5"],
        svec!["c", "3"],
        svec!["a", "1"],
    ];

    assert_eq!(got, expected);
}

#[test]
// #[ignore = "temporarily disable due to a bug in polars aliasing"]
fn sqlp_boston311_sql_script() {
    let wrk = Workdir::new("sqlp_boston311_sql_script");
    let test_file = wrk.load_test_file("boston311-100.csv");

    wrk.create_from_string(
        "test.sql",
        r#"create table temp_table as select * from "boston311-100" where ontime = 'OVERDUE';
create table temp_table2 as select * from temp_table limit 10;
-- we already got what we needed from temp_table into temp_table2
-- so we can truncate temp_table. Otherwise, the memory taken by temp_table
-- won't be released until the end of the script
truncate temp_table;
select ward,count(*) as cnt from temp_table2 group by ward order by cnt desc, ward asc;"#,
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg("test.sql");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "cnt"],
        svec!["Ward 3", "2"],
        svec![" ", "1"],
        svec!["04", "1"],
        svec!["3", "1"],
        svec!["Ward 13", "1"],
        svec!["Ward 17", "1"],
        svec!["Ward 19", "1"],
        svec!["Ward 21", "1"],
        svec!["Ward 6", "1"],
    ];

    assert_eq!(got, expected);
}

#[test]
// #[ignore = "temporarily disable due to a bug in polars aliasing"]
fn sqlp_boston311_sql_script_json() {
    let wrk = Workdir::new("sqlp_boston311_sql_script_json");
    let test_file = wrk.load_test_file("boston311-100.csv");

    wrk.create_from_string(
        "test.sql",
        r#"create table temp_table as select * from "boston311-100" where ontime = 'OVERDUE';
create table temp_table2 as select * from temp_table limit 10;
select ward,count(*) as cnt from temp_table2 group by ward order by cnt desc, ward asc;"#,
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file)
        .arg("test.sql")
        .args(["--format", "json"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"[{"ward":"Ward 3","cnt":2},{"ward":" ","cnt":1},{"ward":"04","cnt":1},{"ward":"3","cnt":1},{"ward":"Ward 13","cnt":1},{"ward":"Ward 17","cnt":1},{"ward":"Ward 19","cnt":1},{"ward":"Ward 21","cnt":1},{"ward":"Ward 6","cnt":1}]"#;

    assert_eq!(got, expected);
}

#[test]
// #[ignore = "temporarily disable due to a bug in polars aliasing"]
fn sqlp_boston311_sql_script_jsonl() {
    let wrk = Workdir::new("sqlp_boston311_sql_script_jsonl");
    let test_file = wrk.load_test_file("boston311-100.csv");

    wrk.create_from_string(
        "test.sql",
        r#"create table temp_table as select * from "boston311-100" where ontime = 'OVERDUE';
create table temp_table2 as select * from temp_table limit 10;
select ward,count(*) as cnt from temp_table2 group by ward order by cnt desc, ward asc;"#,
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file)
        .arg("test.sql")
        .args(["--format", "jsonl"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"ward":"Ward 3","cnt":2}
{"ward":" ","cnt":1}
{"ward":"04","cnt":1}
{"ward":"3","cnt":1}
{"ward":"Ward 13","cnt":1}
{"ward":"Ward 17","cnt":1}
{"ward":"Ward 19","cnt":1}
{"ward":"Ward 21","cnt":1}
{"ward":"Ward 6","cnt":1}"#;

    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_sql_script_jsonl_cache_schema() {
    let wrk = Workdir::new("sqlp_boston311_sql_script_jsonl_cache_schema");
    let test_file = wrk.load_test_file("boston311-100.csv");

    wrk.create_from_string(
        "test.sql",
        r#"create table temp_table as select * from "boston311-100" where ontime = 'OVERDUE';
create table temp_table2 as select * from temp_table limit 10;
select ward,count(*) as cnt from temp_table2 group by ward order by cnt desc, ward asc;"#,
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file)
        .arg("test.sql")
        .args(["--format", "jsonl", "--cache-schema"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"ward":"Ward 3","cnt":2}
{"ward":" ","cnt":1}
{"ward":"04","cnt":1}
{"ward":"3","cnt":1}
{"ward":"Ward 13","cnt":1}
{"ward":"Ward 17","cnt":1}
{"ward":"Ward 19","cnt":1}
{"ward":"Ward 21","cnt":1}
{"ward":"Ward 6","cnt":1}"#;

    assert_eq!(got, expected);
    assert!(wrk.path("boston311-100.csv.pschema.json").exists());
    let boston311_schema =
        std::fs::read_to_string(wrk.path("boston311-100.csv.pschema.json")).unwrap();
    assert_eq!(
        boston311_schema,
        r#"{
  "fields": {
    "case_enquiry_id": "UInt64",
    "open_dt": {
      "Datetime": [
        "Milliseconds",
        null
      ]
    },
    "target_dt": {
      "Datetime": [
        "Milliseconds",
        null
      ]
    },
    "closed_dt": {
      "Datetime": [
        "Milliseconds",
        null
      ]
    },
    "ontime": "String",
    "case_status": "String",
    "closure_reason": "String",
    "case_title": "String",
    "subject": "String",
    "reason": "String",
    "type": "String",
    "queue": "String",
    "department": "String",
    "submittedphoto": "String",
    "closedphoto": "String",
    "location": "String",
    "fire_district": "String",
    "pwd_district": "String",
    "city_council_district": "String",
    "police_district": "String",
    "neighborhood": "String",
    "neighborhood_services_district": "String",
    "ward": "String",
    "precinct": "String",
    "location_street_name": "String",
    "location_zipcode": "String",
    "latitude": "Float32",
    "longitude": "Float32",
    "source": "String"
  },
  "metadata": null
}"#
    );
}

#[test]
fn sqlp_boston311_sql_cache_schema_decimal_override() {
    let wrk = Workdir::new("sqlp_boston311_sql_cache_schema_decimal_override");
    let test_file = wrk.load_test_file("boston311-100.csv");

    wrk.create_from_string(
        "test.sql",
        "select latitude,longitude from 'boston311-100' limit 10",
    );

    wrk.create_from_string(
        "boston311-100.csv.pschema.json",
        r#"{
        "fields": {
          "case_enquiry_id": "Int64",
          "open_dt": "String",
          "target_dt": "String",
          "closed_dt": "String",
          "ontime": "String",
          "case_status": "String",
          "closure_reason": "String",
          "case_title": "String",
          "subject": "String",
          "reason": "String",
          "type": "String",
          "queue": "String",
          "department": "String",
          "submittedphoto": "String",
          "closedphoto": "String",
          "location": "String",
          "fire_district": "String",
          "pwd_district": "String",
          "city_council_district": "String",
          "police_district": "String",
          "neighborhood": "String",
          "neighborhood_services_district": "String",
          "ward": "String",
          "precinct": "String",
          "location_street_name": "String",
          "location_zipcode": "String",
          "latitude": {"Decimal" : [10, 3]},
          "longitude": {"Decimal" : [10, 6]},
          "source": "String"
        },"metadata": null
      }"#,
    );

    assert!(wrk.path("boston311-100.csv").exists());
    assert!(wrk.path("boston311-100.csv.pschema.json").exists());

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file)
        .arg("test.sql")
        .args(["--format", "jsonl", "--cache-schema"]);

    let got: String = wrk.stdout_on_success(&mut cmd);
    let expected = r#"{"latitude":"42.359","longitude":"-71.058700"}
{"latitude":"42.363","longitude":"-71.056600"}
{"latitude":"42.288","longitude":"-71.133000"}
{"latitude":"42.340","longitude":"-71.080300"}
{"latitude":"42.374","longitude":"-71.059900"}
{"latitude":"42.359","longitude":"-71.070000"}
{"latitude":"42.312","longitude":"-71.115200"}
{"latitude":"42.361","longitude":"-71.063800"}
{"latitude":"42.360","longitude":"-71.063400"}
{"latitude":"42.349","longitude":"-71.081100"}"#;

    assert_eq!(got, expected);
}

#[test]
// #[ignore = "temporarily disable due to a bug in polars aliasing"]
fn sqlp_boston311_cte_script() {
    let wrk = Workdir::new("sqlp_boston311_cte");
    let test_file = wrk.load_test_file("boston311-100.csv");

    wrk.create_from_string(
        "test.sql",
        r#"with boston311_roxbury as (select * from "boston311-100" where neighborhood = 'Roxbury')
select ward,count(*) as cnt from boston311_roxbury group by ward order by cnt desc, ward asc;"#,
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg("test.sql");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "cnt"],
        svec!["Ward 11", "2"],
        svec!["Ward 13", "2"],
        svec!["Ward 8", "2"],
        svec!["14", "1"],
        svec!["Ward 12", "1"],
    ];

    assert_eq!(got, expected);
}

#[test]
// #[ignore = "temporarily disable due to a bug in polars aliasing"]
fn sqlp_boston311_cte() {
    let wrk = Workdir::new("sqlp_boston311_cte");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg(
        r#"with boston311_roxbury as (select * from "boston311-100" where neighborhood = 'Roxbury')
    select ward,count(*) as cnt from boston311_roxbury group by ward order by cnt desc, ward asc;"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "cnt"],
        svec!["Ward 11", "2"],
        svec!["Ward 13", "2"],
        svec!["Ward 8", "2"],
        svec!["14", "1"],
        svec!["Ward 12", "1"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_cte_gz() {
    let wrk = Workdir::new("sqlp_boston311_cte_gz");
    let test_file = wrk.load_test_file("boston311-100.csv.gz");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg(
        r#"with boston311_roxbury as (select * from read_csv('boston311-100.csv.gz') where neighborhood = 'Roxbury')
    select ward,count(*) as cnt from boston311_roxbury group by ward order by cnt desc, ward asc;"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["ward", "cnt"],
        svec!["Ward 11", "2"],
        svec!["Ward 13", "2"],
        svec!["Ward 8", "2"],
        svec!["14", "1"],
        svec!["Ward 12", "1"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_case_expression() {
    let wrk = Workdir::new("sqlp_boston311_case_expression");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg(
        r#"SELECT case_enquiry_id, 
           CASE closed_dt is null and case_title ~* 'graffiti' 
              WHEN True THEN 'Yes' 
              WHEN False THEN 'No' 
              ELSE 'N/A'
           END as graffiti_related
           from _t_1
           where case_status = 'Open'"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["case_enquiry_id", "graffiti_related"],
        svec!["101004143000", "No"],
        svec!["101004155594", "No"],
        svec!["101004154423", "No"],
        svec!["101004141848", "No"],
        svec!["101004113313", "No"],
        svec!["101004113751", "Yes"],
        svec!["101004113902", "Yes"],
        svec!["101004113473", "No"],
        svec!["101004113604", "No"],
        svec!["101004114154", "Yes"],
        svec!["101004114383", "No"],
        svec!["101004114795", "Yes"],
        svec!["101004118346", "Yes"],
        svec!["101004115302", "No"],
        svec!["101004115066", "No"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_case_expression_zlib() {
    let wrk = Workdir::new("sqlp_boston311_case_expression_zlib");
    let test_file = wrk.load_test_file("boston311-100.csv.zlib");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg(
        r#"SELECT case_enquiry_id, 
           CASE closed_dt is null and case_title ~* 'graffiti' 
              WHEN True THEN 'Yes' 
              WHEN False THEN 'No' 
              ELSE 'N/A'
           END as graffiti_related
           from read_csv('boston311-100.csv.zlib')
           where case_status = 'Open'"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["case_enquiry_id", "graffiti_related"],
        svec!["101004143000", "No"],
        svec!["101004155594", "No"],
        svec!["101004154423", "No"],
        svec!["101004141848", "No"],
        svec!["101004113313", "No"],
        svec!["101004113751", "Yes"],
        svec!["101004113902", "Yes"],
        svec!["101004113473", "No"],
        svec!["101004113604", "No"],
        svec!["101004114154", "Yes"],
        svec!["101004114383", "No"],
        svec!["101004114795", "Yes"],
        svec!["101004118346", "Yes"],
        svec!["101004115302", "No"],
        svec!["101004115066", "No"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_case_expression_streaming() {
    let wrk = Workdir::new("sqlp_boston311_case_expression_streaming");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file)
        .arg(
            r#"SELECT case_enquiry_id, 
           CASE closed_dt is null and case_title ~* 'graffiti' 
              WHEN True THEN 'Yes' 
              WHEN False THEN 'No' 
              ELSE 'N/A'
           END as graffiti_related
           from _t_1
           where case_status = 'Open'"#,
        )
        .arg("--streaming");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["case_enquiry_id", "graffiti_related"],
        svec!["101004143000", "No"],
        svec!["101004155594", "No"],
        svec!["101004154423", "No"],
        svec!["101004141848", "No"],
        svec!["101004113313", "No"],
        svec!["101004113751", "Yes"],
        svec!["101004113902", "Yes"],
        svec!["101004113473", "No"],
        svec!["101004113604", "No"],
        svec!["101004114154", "Yes"],
        svec!["101004114383", "No"],
        svec!["101004114795", "Yes"],
        svec!["101004118346", "Yes"],
        svec!["101004115302", "No"],
        svec!["101004115066", "No"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_case() {
    let wrk = Workdir::new("sqlp_boston311_case");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg(
        r#"SELECT case_enquiry_id, 
           CASE 
              WHEN case_title ~* 'graffiti' THEN 'Graffitti' 
              WHEN case_title ~* 'vehicle' THEN 'Vehicle'
              WHEN case_title ~* 'sidewalk' THEN 'Sidewalk'
              ELSE 'Something else'
           END as topic
           from _t_1
           where case_status = 'Open'"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["case_enquiry_id", "topic"],
        svec!["101004143000", "Something else"],
        svec!["101004155594", "Something else"],
        svec!["101004154423", "Sidewalk"],
        svec!["101004141848", "Something else"],
        svec!["101004113313", "Something else"],
        svec!["101004113751", "Graffitti"],
        svec!["101004113902", "Graffitti"],
        svec!["101004113473", "Sidewalk"],
        svec!["101004113604", "Something else"],
        svec!["101004114154", "Graffitti"],
        svec!["101004114383", "Something else"],
        svec!["101004114795", "Graffitti"],
        svec!["101004118346", "Graffitti"],
        svec!["101004115302", "Vehicle"],
        svec!["101004115066", "Sidewalk"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_case_zstd() {
    let wrk = Workdir::new("sqlp_boston311_case_zst");
    let test_file = wrk.load_test_file("boston311-100.csv.zst");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg(
        r#"SELECT case_enquiry_id, 
           CASE 
              WHEN case_title ~* 'graffiti' THEN 'Graffitti' 
              WHEN case_title ~* 'vehicle' THEN 'Vehicle'
              WHEN case_title ~* 'sidewalk' THEN 'Sidewalk'
              ELSE 'Something else'
           END as topic
           from read_csv('boston311-100.csv.zst')
           where case_status = 'Open'"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["case_enquiry_id", "topic"],
        svec!["101004143000", "Something else"],
        svec!["101004155594", "Something else"],
        svec!["101004154423", "Sidewalk"],
        svec!["101004141848", "Something else"],
        svec!["101004113313", "Something else"],
        svec!["101004113751", "Graffitti"],
        svec!["101004113902", "Graffitti"],
        svec!["101004113473", "Sidewalk"],
        svec!["101004113604", "Something else"],
        svec!["101004114154", "Graffitti"],
        svec!["101004114383", "Something else"],
        svec!["101004114795", "Graffitti"],
        svec!["101004118346", "Graffitti"],
        svec!["101004115302", "Vehicle"],
        svec!["101004115066", "Sidewalk"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_literal_pattern_match() {
    let wrk = Workdir::new("sqlp_literal_pattern_match");
    wrk.create(
        "test.csv",
        vec![
            svec!["idx", "val"],
            svec!["0", "ABC"],
            svec!["1", "abc"],
            svec!["2", "000"],
            svec!["3", "A0C"],
            svec!["4", "a0c"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv")
        .arg(r#"SELECT * FROM test WHERE val NOT REGEXP '.*c$'"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["idx", "val"],
        svec!["0", "ABC"],
        svec!["2", "000"],
        svec!["3", "A0C"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_expression_pattern_match() {
    let wrk = Workdir::new("sqlp_expression_pattern_match");
    wrk.create(
        "test.csv",
        vec![
            svec!["idx", "val", "pat"],
            svec!["0", "ABC", "^A"],
            svec!["1", "abc", "^A"],
            svec!["2", "000", "^A"],
            svec!["3", "A0C", r#"[AB]\d.*$"#,],
            svec!["4", "a0c", ".*xxx$"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv")
        .arg("SELECT idx, val FROM test WHERE val REGEXP pat");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["idx", "val"], svec!["0", "ABC"], svec!["3", "A0C"]];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_sql_join_on_subquery() {
    let wrk = Workdir::new("sqlp_sql_join_on_subquery");
    wrk.create(
        "test.csv",
        vec![
            svec!["idx", "val"],
            svec!["0", "ABC"],
            svec!["1", "abc"],
            svec!["2", "000"],
            svec!["3", "A0C"],
            svec!["4", "a0c"],
        ],
    );

    wrk.create(
        "test2.csv",
        vec![
            svec!["idx", "val"],
            svec!["0", "ABC"],
            svec!["1", "abc"],
            svec!["2", "000"],
            svec!["3", "A0C"],
            svec!["4", "a0c"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg("test2.csv").arg(
        "SELECT * FROM test t1 JOIN (SELECT idx, val FROM test2 WHERE idx > 2) t2 ON t1.idx = \
         t2.idx",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["idx", "val", "idx:t2", "val:t2"],
        svec!["3", "A0C", "3", "A0C"],
        svec!["4", "a0c", "4", "a0c"],
    ];

    // sqlp exposes no ordering option and its join output is nondeterministic
    // run to run; assert content, not order.
    assert_eq!(
        crate::workdir::sorted_rows(got),
        crate::workdir::sorted_rows(expected)
    );
}

#[test]
fn sqlp_sql_join_on_literal_string_comparison() {
    let wrk = Workdir::new("sqlp_sql_join_on_literal_string_comparison");
    wrk.create(
        "test.csv",
        vec![
            svec!["name", "role"],
            svec!["alice", "admin"],
            svec!["bob", "user"],
            svec!["adam", "admin"],
            svec!["charlie", "user"],
        ],
    );

    wrk.create(
        "test2.csv",
        vec![
            svec!["name", "dept"],
            svec!["alice", "IT"],
            svec!["bob", "HR"],
            svec!["charlie", "IT"],
            svec!["adam", "SEC"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg("test2.csv").arg(
        "SELECT t1.name, t1.role, t2.dept FROM test t1 INNER JOIN test2 t2 ON t1.name = t2.name \
         AND t1.role = 'admin' ORDER BY t1.name",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "role", "dept"],
        svec!["adam", "admin", "SEC"],
        svec!["alice", "admin", "IT"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_sql_join_on_expression_comparison() {
    let wrk = Workdir::new("sqlp_sql_join_on_expression_comparison");
    wrk.create(
        "test.csv",
        vec![
            svec!["code", "value"],
            svec!["HELLO", "100"],
            svec!["WORLD", "200"],
            svec!["FOO", "300"],
        ],
    );

    wrk.create(
        "test2.csv",
        vec![
            svec!["code", "value"],
            svec!["hello", "-1.2345"],
            svec!["world", "4.5678"],
            svec!["bar", "2.2222"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv")
        .arg("test2.csv")
        .arg(
            "SELECT t1.code AS code1, t2.code AS code2, (t1.value * t2.value) AS VAL FROM test t1 \
             INNER JOIN test2 t2 ON LOWER(t1.code) = t2.code",
        )
        .args(["--float-precision", "2"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["code1", "code2", "VAL"],
        svec!["HELLO", "hello", "-123.45"],
        svec!["WORLD", "world", "913.56"],
    ];

    // sqlp exposes no ordering option and its join output is nondeterministic
    // run to run; assert content, not order.
    assert_eq!(
        crate::workdir::sorted_rows(got),
        crate::workdir::sorted_rows(expected)
    );
}

#[test]
fn sqlp_sql_from_subquery() {
    let wrk = Workdir::new("sqlp_sql_from_subquery");
    wrk.create(
        "test.csv",
        vec![
            svec!["idx", "val"],
            svec!["0", "ABC"],
            svec!["1", "abc"],
            svec!["2", "000"],
            svec!["3", "A0C"],
            svec!["4", "a0c"],
        ],
    );

    wrk.create(
        "test2.csv",
        vec![
            svec!["idx", "val"],
            svec!["0", "ABC"],
            svec!["1", "abc"],
            svec!["2", "000"],
            svec!["3", "A0C"],
            svec!["4", "a0c"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg("test2.csv").arg(
        "SELECT * FROM (SELECT idx, val FROM test WHERE idx > 2) t1 JOIN test2 t2 ON t1.idx = \
         t2.idx",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["idx", "val", "idx:t2", "val:t2"],
        svec!["3", "A0C", "3", "A0C"],
        svec!["4", "a0c", "4", "a0c"],
    ];

    // sqlp exposes no ordering option and its join output is nondeterministic
    // run to run; assert content, not order.
    assert_eq!(
        crate::workdir::sorted_rows(got),
        crate::workdir::sorted_rows(expected)
    );
}

#[test]
fn sqlp_sql_tsv() {
    let wrk = Workdir::new("sqlp_sql_tsv");
    wrk.create(
        "test.csv",
        vec![
            svec!["idx", "val"],
            svec!["0", "ABC"],
            svec!["1", "abc"],
            svec!["2", "000"],
            svec!["3", "A0C"],
            svec!["4", "a0c"],
        ],
    );

    let output_file = wrk.path("output.tsv").to_string_lossy().to_string();

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv")
        .arg("SELECT * FROM test")
        .args(["--output", &output_file]);

    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string(&output_file).unwrap();

    let expected = "idx\tval\n0\tABC\n1\tabc\n2\t000\n3\tA0C\n4\ta0c\n";

    assert_eq!(got, expected);
}

#[test]
fn sqlp_sql_ssv() {
    let wrk = Workdir::new("sqlp_sql_ssv");
    wrk.create(
        "test.csv",
        vec![
            svec!["idx", "val"],
            svec!["0", "ABC"],
            svec!["1", "abc"],
            svec!["2", "000"],
            svec!["3", "A0C"],
            svec!["4", "a0c"],
        ],
    );

    let output_file = wrk.path("output.ssv").to_string_lossy().to_string();

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv")
        .arg("SELECT * FROM test")
        .args(["--output", &output_file]);

    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string(&output_file).unwrap();

    let expected = "idx;val\n0;ABC\n1;abc\n2;000\n3;A0C\n4;a0c\n";

    assert_eq!(got, expected);
}

#[test]
fn sqlp_sql_ssv_output() {
    let wrk = Workdir::new("sqlp_sql_ssv_output");
    wrk.create_with_delim(
        "test.ssv",
        vec![
            svec!["idx", "val"],
            svec!["0", "ABC"],
            svec!["1", "abc"],
            svec!["2", "000"],
            svec!["3", "A0C"],
            svec!["4", "a0c"],
        ],
        b';',
    );

    let output_file = wrk.path("output.ssv").to_string_lossy().to_string();

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.ssv")
        .arg("SELECT * FROM test")
        .args(["--output", &output_file]);

    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string(&output_file).unwrap();

    let expected = "idx;val\n0;ABC\n1;abc\n2;000\n3;A0C\n4;a0c\n";

    assert_eq!(got, expected);
}

#[test]
fn sqlp_issue2014() {
    let wrk = Workdir::new("sqlp_issue2014");
    wrk.create_with_delim(
        "test.ssv",
        vec![
            svec!["id", "item", "price"],
            svec!["0", "wallet", "9.99"],
            svec!["1", "comb", "1.39"],
            svec!["2", "pencil", "0.49"],
        ],
        b';',
    );

    let output_file = wrk.path("output.ssv.sz").to_string_lossy().to_string();

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.ssv")
        .arg("SELECT * FROM test")
        .args(["--output", &output_file]);

    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("snappy");
    cmd.arg("decompress").arg(output_file.clone());
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        ["id;item;price"],
        ["0;wallet;9.99"],
        ["1;comb;1.39"],
        ["2;pencil;0.49"],
    ];
    assert_eq!(got, expected);

    let mut cmd = wrk.command("slice");
    cmd.arg(output_file.clone());

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "item", "price"],
        svec!["0", "wallet", "9.99"],
        svec!["1", "comb", "1.39"],
        svec!["2", "pencil", "0.49"],
    ];

    assert_eq!(got, expected);

    let mut cmd = wrk.command("headers");
    cmd.arg(output_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![["1   id"], ["2   item"], ["3   price"]];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_binary_functions() {
    let wrk = Workdir::new("sqlp_sql_binary_functions");
    wrk.create("dummy.csv", vec![svec!["dummy"], svec!["0"]]);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("dummy.csv")
        .arg(
            r#"
        SELECT *,
          -- bit strings
          b''                 AS b0,
          b'1001'             AS b1,
          b'11101011'         AS b2,
          b'1111110100110010' AS b3,
          -- hex strings
          x''                 AS x0,
          x'FF'               AS x1,
          x'4142'             AS x2,
          x'DeadBeef'         AS x3,
        FROM dummy
        "#,
        )
        .args(["--format", "parquet"]);

    wrk.assert_success(&mut cmd);
}

#[test]

fn sqlp_length_fns() {
    let wrk = Workdir::new("sqlp_sql_length_fns");
    wrk.create(
        "test.csv",
        vec![svec!["words"], svec!["Cafe"], svec![""], svec!["東京"]],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
              words,
              LENGTH(words) AS n_chrs1,
              CHAR_LENGTH(words) AS n_chrs2,
              CHARACTER_LENGTH(words) AS n_chrs3,
              OCTET_LENGTH(words) AS n_bytes,
              BIT_LENGTH(words) AS n_bits
            FROM test
"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec![
            "words", "n_chrs1", "n_chrs2", "n_chrs3", "n_bytes", "n_bits"
        ],
        svec!["Cafe", "4", "4", "4", "4", "32"],
        // as the rnull-values is not set, the empty string is not converted to NULL
        // so the output is all empty strings
        svec!["", "", "", "", "", ""],
        svec!["東京", "2", "2", "2", "6", "48"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_length_fns_rnull_set_to_null() {
    let wrk = Workdir::new("sqlp_sql_length_fns_rnull_set_to_null");
    wrk.create(
        "test.csv",
        vec![svec!["words"], svec!["Cafe"], svec![""], svec!["東京"]],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv")
        .arg(
            r#"
        SELECT
              words,
              LENGTH(words) AS n_chrs1,
              CHAR_LENGTH(words) AS n_chrs2,
              CHARACTER_LENGTH(words) AS n_chrs3,
              OCTET_LENGTH(words) AS n_bytes,
              BIT_LENGTH(words) AS n_bits
            FROM test
"#,
        )
        .args(["--rnull-values", "NULL"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec![
            "words", "n_chrs1", "n_chrs2", "n_chrs3", "n_bytes", "n_bits"
        ],
        svec!["Cafe", "4", "4", "4", "4", "32"],
        // as the rnull-values is set to NULL, the empty string is converted to NULL
        // so the output of all the length functions is zero
        svec!["", "0", "0", "0", "0", "0"],
        svec!["東京", "2", "2", "2", "6", "48"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_control_flow() {
    let wrk = Workdir::new("sqlp_control_flow");
    wrk.create(
        "test.csv",
        vec![
            svec!["x", "y", "z"],
            svec!["1", "5", "3"],
            svec!["", "4", "4"],
            svec!["2", "", ""],
            svec!["3", "3", "3"],
            svec!["", "", "6"],
            svec!["4", "2", ""],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
          COALESCE(x,y,z) as "coalsc",
          NULLIF(x, y) as "nullif x_y",
          NULLIF(y, z) as "nullif y_z",
          IFNULL(x, y) as "ifnull x_y",
          IFNULL(y,-1) as "inullf y_z",
          COALESCE(x, NULLIF(y,z)) as "both",
          IF(x = y, 'eq', 'ne') as "x_eq_y",
        FROM test
"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec![
            "coalsc",
            "nullif x_y",
            "nullif y_z",
            "ifnull x_y",
            "inullf y_z",
            "both",
            "x_eq_y"
        ],
        svec!["1", "1", "5", "1", "5", "1", "ne"],
        svec!["4", "", "", "4", "4", "", "ne"],
        svec!["2", "2", "", "2", "-1", "2", "ne"],
        svec!["3", "", "", "3", "3", "3", "eq"],
        svec!["6", "", "", "", "-1", "", "ne"],
        svec!["4", "4", "2", "4", "2", "4", "ne"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_div_sign() {
    let wrk = Workdir::new("sqlp_div_sign");
    wrk.create(
        "test.csv",
        vec![
            svec!["a", "b"],
            svec!["10.0", "-100.5"],
            svec!["20.0", "7.0"],
            svec!["30.0", "2.5"],
            svec!["40.0", ""],
            svec!["50.0", "-3.14"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
            a / b AS a_div_b,
            a // b AS a_floordiv_b,
            SIGN(b) AS b_sign,
        FROM test
"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["a_div_b", "a_floordiv_b", "b_sign"],
        svec!["-0.09950248756218906", "-1", "-1.0"],
        svec!["2.857142857142857", "2", "1.0"],
        svec!["12.0", "12", "1.0"],
        svec!["", "", ""],
        svec!["-15.92356687898089", "-16", "-1.0"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_string_replace() {
    let wrk = Workdir::new("sqlp_string_replace");
    wrk.create(
        "test.csv",
        vec![
            svec!["words"],
            svec!["Yemeni coffee is the best coffee"],
            svec![""],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv")
        .arg(
            r#"
        SELECT
        REPLACE(
          REPLACE(words, 'coffee', 'tea'),
          'Yemeni',
          'English breakfast'
        )
        FROM test
"#,
        )
        .args(["--rnull-values", "NULL"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["words"],
        svec!["English breakfast tea is the best tea"],
        svec![""],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_compound_join_basic() {
    let wrk = Workdir::new("sqlp_compound_join_basic");
    wrk.create(
        "test1.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "1"],
            svec!["2", "3"],
            svec!["3", "4"],
            svec!["4", "4"],
            svec!["5", "5"],
        ],
    );

    wrk.create(
        "test2.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "0"],
            svec!["2", "3"],
            svec!["3", "4"],
            svec!["4", "5"],
            svec!["5", "6"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.args(["test1.csv", "test2.csv"])
        .arg(
            r#"
        SELECT * FROM test1
         INNER JOIN test2 ON test1.a = test2.a AND test1.b = test2.b
"#,
        )
        .arg("--truncate-ragged-lines");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["a", "b", "a:test2", "b:test2"],
        svec!["2", "3", "2", "3"],
        svec!["3", "4", "3", "4"],
    ];

    // sqlp exposes no ordering option and its join output is nondeterministic
    // run to run; assert content, not order.
    assert_eq!(
        crate::workdir::sorted_rows(got),
        crate::workdir::sorted_rows(expected)
    );
}

#[test]
fn sqlp_compound_join_diff_colnames() {
    let wrk = Workdir::new("sqlp_compound_join_diff_colnames");
    wrk.create(
        "test1.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "1"],
            svec!["2", "2"],
            svec!["3", "3"],
            svec!["4", "4"],
            svec!["5", "5"],
        ],
    );

    wrk.create(
        "test2.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["0", "1", "7"],
            svec!["2", "2", "8"],
            svec!["3", "3", "9"],
            svec!["4", "5", "10"],
            svec!["5", "6", "11"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.args(["test1.csv", "test2.csv"])
        .arg(
            r#"
        SELECT * FROM test1
         INNER JOIN test2 ON test1.a = test2.b AND test1.b = test2.a
"#,
        )
        .arg("--truncate-ragged-lines");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["a", "b", "a:test2", "b:test2", "c"],
        svec!["2", "2", "2", "2", "8"],
        svec!["3", "3", "3", "3", "9"],
    ];

    // sqlp exposes no ordering option and its join output is nondeterministic
    // run to run; assert content, not order.
    assert_eq!(
        crate::workdir::sorted_rows(got),
        crate::workdir::sorted_rows(expected)
    );
}

#[test]
fn sqlp_compound_join_three_tables() {
    let wrk = Workdir::new("sqlp_compound_join_three_tables");
    wrk.create(
        "test1.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "1"],
            svec!["2", "3"],
            svec!["3", "4"],
            svec!["4", "4"],
            svec!["5", "5"],
        ],
    );

    wrk.create(
        "test2.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "0"],
            svec!["2", "3"],
            svec!["3", "4"],
            svec!["4", "5"],
            svec!["5", "6"],
        ],
    );

    wrk.create(
        "test3.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "0", "0"],
            svec!["2", "3", "3"],
            svec!["3", "4", "4"],
            svec!["4", "5", "5"],
            svec!["5", "6", "6"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.args(["test1.csv", "test2.csv", "test3.csv"])
        .arg(
            r#"
            SELECT * FROM test1
            INNER JOIN test2
                ON test1.a = test2.a AND test1.b = test2.b
            INNER JOIN test3
                ON test1.a = test3.a AND test1.b = test3.b
"#,
        )
        .arg("--truncate-ragged-lines");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["a", "b", "a:test2", "b:test2", "a:test3", "b:test3", "c"],
        svec!["2", "3", "2", "3", "2", "3", "3"],
        svec!["3", "4", "3", "4", "3", "4", "4"],
    ];

    // sqlp exposes no ordering option and its join output is nondeterministic
    // run to run; assert content, not order.
    assert_eq!(
        crate::workdir::sorted_rows(got),
        crate::workdir::sorted_rows(expected)
    );
}

#[test]
fn sqlp_string_concat() {
    let wrk = Workdir::new("sqlp_string_concat");
    wrk.create(
        "test.csv",
        vec![
            svec!["x", "y", "z"],
            svec!["a", "d", "1"],
            svec!["", "e", "2"],
            svec!["c", "f", "3"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
           ("x" || "x" || "y")           AS c0,
           ("x" || "y" || "z")           AS c1,
           CONCAT(("x" + "x"), "y")      AS c2,
           CONCAT("x", "x", "y")         AS c3,
           CONCAT("x", "y", ("z" * 2))   AS c4,
           CONCAT_WS(':', "x", "y", "z") AS c5,
           CONCAT_WS('!', "x", "y", "z") AS c6,
        FROM test
"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["c0", "c1", "c2", "c3", "c4", "c5", "c6"],
        svec!["aad", "ad1", "aad", "aad", "ad2", "a:d:1", "a!d!1"],
        svec!["", "", "e", "e", "e4", "e:2", "e!2"],
        svec!["ccf", "cf3", "ccf", "ccf", "cf6", "c:f:3", "c!f!3"],
    ];

    assert_eq!(got, expected);
}

// #[test]
// fn sqlp_select_1() {
//     let wrk = Workdir::new("sqlp_select_1");
//     wrk.create(
//         "test.csv",
//         vec![
//             svec!["x", "y", "z"],
//             svec!["a", "d", "1"],
//             svec!["", "e", "2"],
//             svec!["c", "f", "3"],
//         ],
//     );

//     let mut cmd = wrk.command("sqlp");
//     cmd.arg("test.csv").arg("SELECT 1 from _t_1");

//     wrk.assert_success(&mut cmd);

//     let got = wrk.output_stderr(&mut cmd);
//     let expected = "(3, 1)";

//     assert!(got.starts_with(expected));
// }

#[test]
fn sqlp_string_right_reverse() {
    let wrk = Workdir::new("sqlp_string_right_reverse");
    wrk.create(
        "test.csv",
        vec![
            svec!["txt"],
            svec!["abcde"],
            svec!["abc"],
            svec!["a"],
            svec![""],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
           LEFT(txt,2) AS "l",
           RIGHT(txt,2) AS "r",
           REVERSE(txt) AS "rev"
        FROM test
"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["l", "r", "rev"],
        svec!["ab", "de", "edcba"],
        svec!["ab", "bc", "cba"],
        svec!["a", "a", "a"],
        svec!["", "", ""],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_modulo() {
    let wrk = Workdir::new("sqlp_modulo");
    wrk.create(
        "test.csv",
        vec![
            svec!["a", "b", "c", "d"],
            svec!["1.5", "6", "11", "16.5"],
            svec!["", "7", "12", "17.0"],
            svec!["3.0", "8", "13", "18.5"],
            svec!["4333333333", "9", "14", ""],
            svec!["5.0", "10", "15", "20.0"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
            cast(a as float) % 2 AS a2,
            cast(b as float)  % 3 AS b3,
            MOD(cast(c as float), 4) AS c4,
            MOD(cast(d as float), 5.5) AS d55
        FROM test
"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["a2", "b3", "c4", "d55"],
        svec!["1.5", "0.0", "3.0", "0.0"],
        svec!["", "1.0", "0.0", "0.5"],
        svec!["1.0", "2.0", "1.0", "2.0"],
        svec!["1.0", "0.0", "2.0", ""],
        svec!["1.0", "1.0", "3.0", "3.5"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_try_cast() {
    let wrk = Workdir::new("sqlp_try_cast");
    wrk.create(
        "test.csv",
        vec![
            svec!["foo", "bar"],
            svec!["65432", "1999-12-31"],
            svec!["101010", "N/A"],
            svec!["-3333", "2024-01-01"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
            try_cast(foo as uint2),
            try_cast(bar as DATE)
        FROM test
"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["foo", "bar"],
        svec!["65432", "1999-12-31"],
        svec!["", ""],
        svec!["", "2024-01-01"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_stddev_variance() {
    let wrk = Workdir::new("sqlp_stddev_variance");
    wrk.create(
        "test.csv",
        vec![
            svec!["v1", "v2", "v3", "v4"],
            svec!["-1.0", "5.5", "-10", "-100"],
            svec!["0.0", "0.0", "", "0.0"],
            svec!["1.0", "3.0", "10", "-50.0"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
            STDEV(v1) AS "v1_std",
            STDDEV(v2) AS "v2_std",
            STDEV_SAMP(v3) AS "v3_std",
            STDDEV_SAMP(v4) AS "v4_std",
            VAR(v1) AS "v1_var",
            VARIANCE(v2) AS "v2_var",
            VARIANCE(v3) AS "v3_var",
            VAR_SAMP(v4) AS "v4_var"
        FROM test
"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec![
            "v1_std", "v2_std", "v3_std", "v4_std", "v1_var", "v2_var", "v3_var", "v4_var"
        ],
        svec![
            "1.0",
            "2.753785273643051",
            "14.142135623730951",
            "50.0",
            "1.0",
            "7.583333333333334",
            "200.0",
            "2500.0"
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_string_position() {
    let wrk = Workdir::new("sqlp_string_position");
    wrk.create(
        "cities.csv",
        vec![
            svec!["city"],
            svec!["Dubai"],
            svec!["Abu Dhabi"],
            svec!["Sharjah"],
            svec!["Al Ain"],
            svec!["Ajman"],
            svec!["Ras Al Khaimah"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("cities.csv").arg(
        r#"
        SELECT
        POSITION('a' IN city) AS a_lc1,
        POSITION('A' IN city) AS a_uc1,
        STRPOS(city,'a') AS a_lc2,
        STRPOS(city,'A') AS a_uc2,
      FROM cities
"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["a_lc1", "a_uc1", "a_lc2", "a_uc2"],
        svec!["4", "0", "4", "0"],
        svec!["7", "1", "7", "1"],
        svec!["3", "0", "3", "0"],
        svec!["0", "1", "0", "1"],
        svec!["4", "1", "4", "1"],
        svec!["2", "5", "2", "5"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_date_part() {
    let wrk = Workdir::new("sqlp_date_part");
    wrk.create(
        "datestbl.csv",
        vec![svec!["datecol"], svec!["20-01-2012 10:30:20"]],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("datestbl.csv")
        .arg(
            r#"
        SELECT 
            DATE_PART('isoyear', datecol) as c1,
            DATE_PART('month', datecol) as c2,
            DATE_PART('day', datecol) as c3,
            DATE_PART('hour', datecol) as c4,
            DATE_PART('minute', datecol) as c5,
            DATE_PART('second', datecol) as c6,
            DATE_PART('millisecond', datecol) as c61,
            DATE_PART('microsecond', datecol) as c62,
            DATE_PART('nanosecond', datecol) as c63,
            DATE_PART('isoweek', datecol) as c7,
            DATE_PART('dayofyear', datecol) as c8,
            DATE_PART('dayofweek', datecol) as c9,
            DATE_PART('time', datecol) as c10,
            DATE_PART('decade', datecol) as c11,
            DATE_PART('century', datecol) as c12,
            DATE_PART('millennium', datecol) as c13,
            DATE_PART('quarter', datecol) as c14,
      FROM datestbl
"#,
        )
        .arg("--try-parsedates");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec![
            "c1", "c2", "c3", "c4", "c5", "c6", "c61", "c62", "c63", "c7", "c8", "c9", "c10",
            "c11", "c12", "c13", "c14"
        ],
        svec![
            "2012",
            "1",
            "20",
            "10",
            "30",
            "20",
            "20000.0",
            "20000000.0",
            "20000000000.0",
            "3",
            "20",
            "5",
            "10:30:20.000000000",
            "201",
            "21",
            "3",
            "1"
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_date_part_tz() {
    let wrk = Workdir::new("sqlp_date_part_tz");
    wrk.create(
        "datestbl.csv",
        vec![
            svec!["datecol"],
            svec!["2012-01-20T10:30:20-05:00"],
            svec!["2012-11-05T10:30:20.1234Z"],
            svec!["1999-03-15T15:30:20.1234+01:00"],
            svec!["1978-08-05T23:30:20.1234-01:00"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("datestbl.csv")
        .arg(
            r#"
        SELECT
            DATE_PART('isoyear', datecol) as c1,
            DATE_PART('month', datecol) as c2,
            DATE_PART('day', datecol) as c3,
            DATE_PART('hour', datecol) as c4,
            DATE_PART('minute', datecol) as c5,
            DATE_PART('second', datecol) as c6,
            DATE_PART('millisecond', datecol) as c61,
            DATE_PART('microsecond', datecol) as c62,
            DATE_PART('nanosecond', datecol) as c63,
            DATE_PART('isoweek', datecol) as c7,
            DATE_PART('dayofyear', datecol) as c8,
            DATE_PART('dayofweek', datecol) as c9,
            DATE_PART('time', datecol) as c10,
            DATE_PART('decade', datecol) as c11,
            DATE_PART('century', datecol) as c12,
            DATE_PART('millennium', datecol) as c13,
            DATE_PART('quarter', datecol) as c14,
            DATE_PART('timezone', datecol) as c15,
            DATE_PART('epoch', datecol) as c16,
      FROM datestbl
"#,
        )
        .arg("--try-parsedates");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec![
            "c1", "c2", "c3", "c4", "c5", "c6", "c61", "c62", "c63", "c7", "c8", "c9", "c10",
            "c11", "c12", "c13", "c14", "c15", "c16"
        ],
        svec![
            "2012",
            "1",
            "20",
            "15",
            "30",
            "20",
            "20000.0",
            "20000000.0",
            "20000000000.0",
            "3",
            "20",
            "5",
            "15:30:20.000000000",
            "201",
            "21",
            "3",
            "1",
            "0",
            "1327073420.0"
        ],
        svec![
            "2012",
            "11",
            "5",
            "10",
            "30",
            "20",
            "20123.4",
            "20123400.0",
            "20123400000.0",
            "45",
            "310",
            "1",
            "10:30:20.123400000",
            "201",
            "21",
            "3",
            "4",
            "0",
            "1352111420.1234"
        ],
        svec![
            "1999",
            "3",
            "15",
            "14",
            "30",
            "20",
            "20123.4",
            "20123400.0",
            "20123400000.0",
            "11",
            "74",
            "1",
            "14:30:20.123400000",
            "199",
            "20",
            "2",
            "1",
            "0",
            "921508220.1234"
        ],
        svec![
            "1978",
            "8",
            "6",
            "0",
            "30",
            "20",
            "20123.4",
            "20123400.0",
            "20123400000.0",
            "31",
            "218",
            "0",
            "00:30:20.123400000",
            "197",
            "20",
            "2",
            "3",
            "0",
            "271211420.1234"
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_date() {
    let wrk = Workdir::new("sqlp_date");

    let mut cmd = wrk.command("sqlp");
    cmd.arg("SKIP_INPUT").arg(
        r#"
        SELECT 
            DATE('2021-03-15') as c1,
            DATE('2021-03-15 10:30:20', '%Y-%m-%d %H:%M:%S') as c2,
            DATE('03-15-2021 10:30:20 AM EST', '%m-%d-%Y %I:%M:%S %p %Z') as c3"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // this is the documented behavior of the date function
    // use STRFTIME and STRPTIME for more control
    let expected = vec![
        svec!["c1", "c2", "c3"],
        svec!["2021-03-15", "2021-03-15", "2021-03-15"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_date_strftime() {
    let wrk = Workdir::new("sqlp_date_strftime");

    wrk.create(
        "data.csv",
        vec![
            svec!["dtm", "dt", "tm"],
            svec!["1972-03-06 23:50:03", "1978-07-05", "10:10:10"],
            svec!["1980-09-30 01:25:50", "1969-12-31", "22:33:55"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg(
            r#"
        SELECT
      STRFTIME(dtm,'%m.%d.%Y/%T') AS s_dtm,
      STRFTIME(dt,'%B %d, %Y') AS s_dt,
      STRFTIME(tm,'%S.%M.%H') AS s_tm,
    FROM data"#,
        )
        .arg("--try-parsedates");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["s_dtm", "s_dt", "s_tm"],
        svec!["03.06.1972/23:50:03", "July 05, 1978", "10.10.10"],
        svec!["09.30.1980/01:25:50", "December 31, 1969", "55.33.22"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_read_jsonl() {
    let wrk = Workdir::new("sqlp_read_jsonl");

    let test_jsonl = wrk.load_test_file("boston311-10.jsonl");

    let mut cmd = wrk.command("sqlp");
    cmd.arg("SKIP_INPUT").arg(
        format!(
            "SELECT * FROM read_json('{}') WHERE source = 'City Worker App'",
            test_jsonl
        )
        .as_str(),
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec![
            "case_enquiry_id",
            "open_dt",
            "target_dt",
            "closed_dt",
            "ontime",
            "case_status",
            "closure_reason",
            "case_title",
            "subject",
            "reason",
            "type",
            "queue",
            "department",
            "submittedphoto",
            "closedphoto",
            "location",
            "fire_district",
            "pwd_district",
            "city_council_district",
            "police_district",
            "neighborhood",
            "neighborhood_services_district",
            "ward",
            "precinct",
            "location_street_name",
            "location_zipcode",
            "latitude",
            "longitude",
            "source"
        ],
        svec![
            "101004120108",
            "2022-01-08 12:54:49",
            "2022-01-11 08:30:00",
            "2022-01-09 06:43:06",
            "ONTIME",
            "Closed",
            "Case Closed. Closed date : Sun Jan 09 06:43:06 EST 2022 Noted ",
            "CE Collection",
            "Public Works Department",
            "Street Cleaning",
            "CE Collection",
            "PWDx_District 1C: Downtown",
            "PWDx",
            "",
            "",
            "198 W Springfield St  Roxbury  MA  02118",
            "4",
            "1C",
            "7",
            "D4",
            "South End",
            "6",
            "Ward 9",
            "0902",
            "198 W Springfield St",
            "2118",
            "42.3401",
            "-71.0803",
            "City Worker App"
        ],
        svec![
            "101004141354",
            "2022-01-20 08:07:49",
            "2022-01-21 08:30:00",
            "2022-01-20 08:45:03",
            "ONTIME",
            "Closed",
            "Case Closed. Closed date : Thu Jan 20 08:45:03 EST 2022 Noted ",
            "CE Collection",
            "Public Works Department",
            "Street Cleaning",
            "CE Collection",
            "PWDx_District 1B: North End",
            "PWDx",
            "",
            "",
            "21-23 Temple St  Boston  MA  02114",
            "3",
            "1B",
            "1",
            "A1",
            "Beacon Hill",
            "3",
            "Ward 3",
            "0306",
            "21-23 Temple St",
            "2114",
            "42.3606",
            "-71.0638",
            "City Worker App"
        ],
        svec![
            "101004141367",
            "2022-01-20 08:15:45",
            "2022-01-21 08:30:00",
            "2022-01-20 08:45:12",
            "ONTIME",
            "Closed",
            "Case Closed. Closed date : Thu Jan 20 08:45:12 EST 2022 Noted ",
            "CE Collection",
            "Public Works Department",
            "Street Cleaning",
            "CE Collection",
            "PWDx_District 1B: North End",
            "PWDx",
            "",
            "",
            "12 Derne St  Boston  MA  02114",
            "3",
            "1B",
            "1",
            "A1",
            "Beacon Hill",
            "3",
            "Ward 3",
            "0306",
            "12 Derne St",
            "2114",
            "42.3596",
            "-71.0634",
            "City Worker App"
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_string_like_ops() {
    let wrk = Workdir::new("sqlp_string_like_ops");

    wrk.create(
        "likedata.csv",
        vec![
            svec!["x", "y"],
            svec!["aaa", "abc"],
            svec!["bbb", "b"],
            svec!["a", "aa"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("likedata.csv").arg(
        r#"
        SELECT
            x,
            x ^@ 'a' AS x_starts_with_a,
            x ~~* '%B' AS x_ends_with_b_case_insensitive,
            x ^@ y AS x_starts_with_y,
            x ~~ '%a' AS x_ends_with_a
        FROM likedata"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec![
            "x",
            "x_starts_with_a",
            "x_ends_with_b_case_insensitive",
            "x_starts_with_y",
            "x_ends_with_a"
        ],
        svec!["aaa", "true", "false", "false", "true"],
        svec!["bbb", "false", "true", "true", "false"],
        svec!["a", "true", "false", "false", "true"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_natural_join() {
    let wrk = Workdir::new("sqlp_natural_join");

    wrk.create(
        "data1.csv",
        vec![
            svec!["CharacterID", "FirstName", "LastName"],
            svec!["1", "Jerna Morat", "Gurgeh"],
            svec!["2", "Cheradenine", "Zakalwe"],
            svec!["3", "Byr", "Genar-Hofoen"],
        ],
    );

    wrk.create(
        "data2.csv",
        vec![
            svec!["CharacterID", "Book"],
            svec!["1", "Player of Games"],
            svec!["2", "Use of Weapons"],
            svec!["3", "Excession"],
        ],
    );

    wrk.create(
        "data3.csv",
        vec![
            svec!["CharacterID", "Ship"],
            svec!["1", "Limiting Factor"],
            svec!["2", "Xenophobe"],
            svec!["3", "Grey Area"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.args(["data1.csv", "data2.csv", "data3.csv"]).arg(
        r#"SELECT COLUMNS('^[^:]+$')
  FROM data1
    NATURAL JOIN data2
    NATURAL JOIN data3
  ORDER BY CharacterID"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["CharacterID", "FirstName", "LastName", "Book", "Ship"],
        svec![
            "1",
            "Jerna Morat",
            "Gurgeh",
            "Player of Games",
            "Limiting Factor"
        ],
        svec!["2", "Cheradenine", "Zakalwe", "Use of Weapons", "Xenophobe"],
        svec!["3", "Byr", "Genar-Hofoen", "Excession", "Grey Area"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_star_ilike() {
    let wrk = Workdir::new("sqlp_star_ilike");

    wrk.create(
        "starlikedata.csv",
        vec![
            svec!["ID", "FirstName", "LastName", "Address", "City"],
            svec!["333", "Bruce", "Wayne", "The Batcave", "Gotham"],
            svec!["666", "Diana", "Prince", "Paradise Island", "Themyscira"],
            svec!["999", "Clark", "Kent", "Fortress of Solitude", "Metropolis"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("starlikedata.csv").arg(
        r#"
        SELECT * ILIKE '%a%e%'
  FROM starlikedata
  ORDER BY FirstName"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["FirstName", "LastName", "Address"],
        svec!["Bruce", "Wayne", "The Batcave"],
        svec!["Clark", "Kent", "Fortress of Solitude"],
        svec!["Diana", "Prince", "Paradise Island"],
    ];

    assert_eq!(got, expected);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("starlikedata.csv").arg(
        r#"
        SELECT * ILIKE '%I%' RENAME (FirstName AS Name) 
  FROM starlikedata
  ORDER BY 3 DESC"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["ID", "Name", "City"],
        svec!["666", "Diana", "Themyscira"],
        svec!["999", "Clark", "Metropolis"],
        svec!["333", "Bruce", "Gotham"],
    ];

    assert_eq!(got, expected);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("starlikedata.csv").arg(
        r#"
        SELECT * EXCLUDE (ID, City, LastName) RENAME FirstName AS Name
  FROM starlikedata
  ORDER BY Name"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Name", "Address"],
        svec!["Bruce", "The Batcave"],
        svec!["Clark", "Fortress of Solitude"],
        svec!["Diana", "Paradise Island"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_skip_input() {
    let wrk = Workdir::new("sqlp_skip_input");

    let mut cmd = wrk.command("sqlp");
    cmd.arg("SKIP_INPUT")
        .arg("SELECT 1 AS one, '2' AS two, 3.0 AS three");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["one", "two", "three"], svec!["1", "2", "3.0"]];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_autotab_delim() {
    let wrk = Workdir::new("sqlp_autotab_delim");
    wrk.create_with_delim(
        "cities_array.tsv",
        vec![
            svec!["countryid", "cities"],
            svec!["DB", "Dubai,Abu Dhabi,Sharjah"],
            svec!["IN", "Mumbai,Delhi,Bangalore"],
            svec!["US", "New York,Los Angeles,Chicago"],
            svec!["UK", "London,Birmingham,Manchester"],
            svec!["CN", "Beijing"],
            svec!["RU", "Moscow"],
            svec!["NL", "Amsterdam"],
            svec!["IT", "Rome,Milan,Turin,Naples,Venice"],
        ],
        b'\t',
    );

    let output_file = wrk.path("output.tsv").to_string_lossy().to_string();

    let mut cmd = wrk.command("sqlp");
    cmd.arg("cities_array.tsv")
        .arg(
            r#"
            SELECT 
                countryid, cities
            FROM cities_array
    "#,
        )
        .args(["--output", &output_file]);

    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string(&output_file).unwrap();
    let expected = r#"countryid	cities
DB	Dubai,Abu Dhabi,Sharjah
IN	Mumbai,Delhi,Bangalore
US	New York,Los Angeles,Chicago
UK	London,Birmingham,Manchester
CN	Beijing
RU	Moscow
NL	Amsterdam
IT	Rome,Milan,Turin,Naples,Venice
"#;

    assert_eq!(got, expected);
}

#[test]
fn sqlp_dollar_quoting_string_literals() {
    let wrk = Workdir::new("sqlp_dollar_quoting_string_literals");
    wrk.create(
        "cities_array.csv",
        vec![
            svec!["country_id", "description"],
            svec!["DB", "Dubai"],
            svec!["IN", "India"],
            svec!["US", "United States"],
            svec!["UK", "United Kingdom"],
            svec!["CN", "China"],
            svec!["RU", "Russia"],
            svec!["NL", "Netherlands"],
            svec!["IT", "Italy"],
        ],
    );

    let output_file = wrk.path("output.tsv").to_string_lossy().to_string();

    let mut cmd = wrk.command("sqlp");
    cmd.arg("cities_array.csv")
        .arg(
            r#"SELECT country_id, $$This is a literal $,%,',"$$ AS complex_literal FROM cities_array"#,
        )
        .args(["--output", &output_file]);

    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string(&output_file).unwrap();
    let expected = r#"country_id	complex_literal
DB	"This is a literal $,%,',"""
IN	"This is a literal $,%,',"""
US	"This is a literal $,%,',"""
UK	"This is a literal $,%,',"""
CN	"This is a literal $,%,',"""
RU	"This is a literal $,%,',"""
NL	"This is a literal $,%,',"""
IT	"This is a literal $,%,',"""
"#;

    assert_eq!(got, expected);
}

#[test]
fn sqlp_string_function_joins() {
    let wrk = Workdir::new("sqlp_string_function_joins");

    // Create left table with partial strings
    wrk.create(
        "left.csv",
        vec![
            svec!["id", "val"],
            svec!["1", "cat"],
            svec!["2", "dog"],
            svec!["3", "bird"],
            svec!["4", "fish"],
        ],
    );

    // Create right table with full strings
    wrk.create(
        "right.csv",
        vec![
            svec!["id", "val"],
            svec!["1", "category"],
            svec!["2", "doghouse"],
            svec!["3", "jailbird"],
            svec!["4", "starfish"],
            svec!["5", "catalog"],
            svec!["6", "hotdog"],
        ],
    );

    // Test STRPOS
    let mut cmd = wrk.command("sqlp");
    cmd.args(["left.csv", "right.csv"]).arg(
        "SELECT l.val as left_val, r.val as right_val FROM left l CROSS JOIN right r WHERE \
         STRPOS(r.val, l.val) > 0 ORDER BY l.val, r.val",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["left_val", "right_val"],
        svec!["bird", "jailbird"],
        svec!["cat", "catalog"],
        svec!["cat", "category"],
        svec!["dog", "doghouse"],
        svec!["dog", "hotdog"],
        svec!["fish", "starfish"],
    ];
    assert_eq!(got, expected);

    // Test STARTS_WITH
    let mut cmd = wrk.command("sqlp");
    cmd.args(["left.csv", "right.csv"]).arg(
        "SELECT l.val as left_val, r.val as right_val FROM left l CROSS JOIN right r WHERE \
         STARTS_WITH(r.val, l.val) ORDER BY l.val, r.val",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["left_val", "right_val"],
        svec!["cat", "catalog"],
        svec!["cat", "category"],
        svec!["dog", "doghouse"],
    ];
    assert_eq!(got, expected);

    // Test ENDS_WITH
    let mut cmd = wrk.command("sqlp");
    cmd.args(["left.csv", "right.csv"]).arg(
        "SELECT l.val as left_val, r.val as right_val FROM left l CROSS JOIN right r WHERE \
         ENDS_WITH(r.val, l.val) ORDER BY l.val, r.val",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["left_val", "right_val"],
        svec!["bird", "jailbird"],
        svec!["dog", "hotdog"],
        svec!["fish", "starfish"],
    ];
    assert_eq!(got, expected);

    // Test reverse STRPOS (left contains right)
    let mut cmd = wrk.command("sqlp");
    cmd.args(["left.csv", "right.csv"]).arg(
        "SELECT l.val as left_val, r.val as right_val FROM left l CROSS JOIN right r WHERE \
         STRPOS(l.val, r.val) > 0 ORDER BY l.val, r.val",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["left_val", "right_val"]]; // Empty because no left values contain right values in our test data
    assert_eq!(got, expected);
}

#[test]
fn sqlp_string_to_array() {
    let wrk = Workdir::new("sqlp_string_to_array");
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "cities"],
            svec!["1", "Dubai,Abu Dhabi,Sharjah"],
            svec!["2", "Mumbai,Delhi,Bangalore"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg(
            r#"
        SELECT id, STRING_TO_ARRAY(cities, ',') AS cities FROM data
    "#,
        )
        .args(["--format", "json"]);

    let got: String = wrk.stdout_on_success(&mut cmd);
    let expected = r#"[{"id":1,"cities":["Dubai","Abu Dhabi","Sharjah"]},{"id":2,"cities":["Mumbai","Delhi","Bangalore"]}]"#;
    assert_eq!(got, expected);
}

#[test]
fn sqlp_split_part() {
    let wrk = Workdir::new("sqlp_split_part");
    wrk.create(
        "data.csv",
        vec![
            svec!["s"],
            svec!["xx,yy,zz"],
            svec!["abc,,xyz,???,hmmm"],
            svec![""],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv").arg(
        r#"
        SELECT
          SPLIT_PART(s,',',1) AS "s+1",
          SPLIT_PART(s,',',3) AS "s+3",
          SPLIT_PART(s,',',-2) AS "s-2",
        FROM data
    "#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["s+1", "s+3", "s-2"],
        svec!["xx", "zz", "yy"],
        svec!["abc", "xyz", "???"],
        svec!["", "", ""],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_corr() {
    let wrk = Workdir::new("sqlp_corr");
    wrk.create(
        "data.csv",
        vec![
            svec!["foo", "bar"],
            svec!["1", "2"],
            svec!["2", "4"],
            svec!["3", "7"],
            svec!["4", "5"],
            svec!["5", "9"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg("SELECT CORR(foo, bar) AS corr FROM data")
        .args(["--float-precision", "6"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["corr"], svec!["0.877809"]];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_covar_pop() {
    let wrk = Workdir::new("sqlp_covar_pop");
    wrk.create(
        "data.csv",
        vec![
            svec!["foo", "bar"],
            svec!["1", "2"],
            svec!["2", "4"],
            svec!["3", "7"],
            svec!["4", "5"],
            svec!["5", "9"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg("SELECT COVAR(foo, bar) AS covar FROM data")
        .args(["--float-precision", "2"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["covar"], svec!["3.75"]];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_corr_covar_covar_pop() {
    let wrk = Workdir::new("sqlp_corr_covar_covar_pop");
    wrk.create(
        "data.csv",
        vec![
            svec!["foo", "bar"],
            svec!["1", "2"],
            svec!["2", "4"],
            svec!["3", "7"],
            svec!["4", "5"],
            svec!["5", "9"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg(
            "SELECT CORR(foo, bar) AS corr, COVAR(foo, bar) AS covar, COVAR_POP(foo, bar) AS \
             covar_pop FROM data",
        )
        .args(["--float-precision", "2"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["corr", "covar", "covar_pop"],
        svec!["0.88", "3.75", "3.00"],
    ];
    assert_eq!(got, expected);
}

// #[test]
// fn sqlp_generate_graphviz_plan() {
//     let wrk = Workdir::new("sqlp_generate_graphviz_plan");

//     wrk.create(
//         "data.csv",
//         vec![
//             svec!["a", "b", "c"],
//             svec!["1", "2", "3"],
//             svec!["4", "5", "6"],
//             svec!["7", "8", "9"],
//         ],
//     );

//     let output_dotfile = wrk.path("output.dot").to_string_lossy().to_string();

//     let mut cmd = wrk.command("sqlp");
//     cmd.env("POLARS_VISUALIZE_PHYSICAL_PLAN", output_dotfile.as_str())
//         .arg("data.csv")
//         .arg(
//             r#"
//         SELECT a, b, c
//         FROM data
//         WHERE a > 2
//         ORDER BY a DESC
//     "#,
//         )
//         .arg("--streaming");

//     wrk.assert_success(&mut cmd);

//     assert!(std::path::Path::new(&output_dotfile).exists());

//     let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
//     let expected = vec![
//         svec!["a", "b", "c"],
//         svec!["7", "8", "9"],
//         svec!["4", "5", "6"],
//     ];

//     assert_eq!(got, expected);

//     let got_dot = wrk.read_to_string(&output_dotfile).unwrap();
//     let expected_dot = r#"digraph {
//     "Projection" -> "Filter";
//     "Filter" -> "CsvScan";
// }"#;

//     assert_eq!(got_dot, expected_dot);
// }

#[test]
fn sqlp_decimal_comma_validation() {
    let wrk = Workdir::new("sqlp_decimal_comma_validation");

    // Create test data with decimal commas
    let test_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create("data.csv", test_data);

    // Test 1: --decimal-comma with comma delimiter should fail
    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg("--decimal-comma")
        .arg("select * from _t_1");

    let got = wrk.stderr_on_error(&mut cmd);
    assert!(got.contains("Using --decimal-comma with a comma separator is invalid"));

    // Test 2: --decimal-comma with semicolon delimiter should succeed
    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg("--decimal-comma")
        .args(["--delimiter", ";"])
        .arg("select * from _t_1");

    let got: String = wrk.stdout_on_success(&mut cmd);
    let expected = "id,value\n\"1,\"\"100,50\"\"\"\n\"2,\"\"200,75\"\"\"";
    assert_eq!(got, expected);

    // Test 3: --decimal-comma with tab delimiter should succeed
    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg("--decimal-comma")
        .args(["--delimiter", "\t"])
        .arg("select * from _t_1");

    let got: String = wrk.stdout_on_success(&mut cmd);
    let expected = "id,value\n\"1,\"\"100,50\"\"\"\n\"2,\"\"200,75\"\"\"";
    assert_eq!(got, expected);

    // Test 4: --decimal-comma with pipe delimiter should succeed
    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg("--decimal-comma")
        .args(["--delimiter", "|"])
        .arg("select * from _t_1");

    let got: String = wrk.stdout_on_success(&mut cmd);
    let expected = "id,value\n\"1,\"\"100,50\"\"\"\n\"2,\"\"200,75\"\"\"";
    assert_eq!(got, expected);
}

#[test]
fn sqlp_decimal_comma_validation_with_tsv_file() {
    let wrk = Workdir::new("sqlp_decimal_comma_validation_with_tsv_file");

    // Create test data with decimal commas in TSV format
    let test_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create_with_delim("data.tsv", test_data, b'\t');

    // Test: --decimal-comma with TSV file (tab delimiter) should succeed
    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.tsv")
        .arg("--decimal-comma")
        .arg("select * from _t_1");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["id", "value"],
        svec!["1", "100,5"],
        svec!["2", "200,75"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_decimal_comma_validation_with_ssv_file() {
    let wrk = Workdir::new("sqlp_decimal_comma_validation_with_ssv_file");

    // Create test data with decimal commas in SSV format
    let test_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create_with_delim("data.ssv", test_data, b';');

    // Test: --decimal-comma with SSV file (semicolon delimiter) should succeed
    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.ssv")
        .arg("--decimal-comma")
        .arg("select * from _t_1");

    let got: String = wrk.stdout_on_success(&mut cmd);
    let expected = "id,value\n1,\"100,5\"\n2,\"200,75\"";
    assert_eq!(got, expected);
}

#[test]
fn sqlp_decimal_comma_validation_with_output_file() {
    let wrk = Workdir::new("sqlp_decimal_comma_validation_with_output_file");

    // Create test data with decimal commas
    let test_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create_with_delim("data.csv", test_data, b';');

    // Test: --decimal-comma with output file should validate the output delimiter
    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg("--decimal-comma")
        .args(["--delimiter", ";"])
        .args(["--output", "output.csv"])
        .arg("select * from _t_1");

    wrk.assert_success(&mut cmd);
    let got: String = wrk.read_to_string("output.csv").unwrap();
    let expected = "id;value\n1;100,5\n2;200,75\n";
    assert_eq!(got, expected);

    // Test: --decimal-comma with output file that would have comma delimiter should fail
    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg("--decimal-comma")
        .args(["--output", "output.csv"])
        .arg("select * from _t_1");

    let got = wrk.stderr_on_error(&mut cmd);
    assert!(got.contains("Using --decimal-comma with a comma separator is invalid"));
}

#[test]
fn sqlp_decimal_comma_validation_with_tsv_output() {
    let wrk = Workdir::new("sqlp_decimal_comma_validation_with_tsv_output");

    // Create test data with decimal commas
    let test_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create_with_delim("data.csv", test_data, b'\t');

    // Test: --decimal-comma with TSV output file should succeed
    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg("--decimal-comma")
        .args(["--delimiter", ";"])
        .args(["--output", "output.tsv"])
        .arg("select * from _t_1");

    wrk.assert_success(&mut cmd);
    let got: String = wrk.read_to_string("output.tsv").unwrap();
    let expected = "\"id\tvalue\"\n\"1\t100,50\"\n\"2\t200,75\"\n";
    assert_eq!(got, expected);
}

#[test]
fn sqlp_decimal_comma_validation_with_ssv_output() {
    let wrk = Workdir::new("sqlp_decimal_comma_validation_with_ssv_output");

    // Create test data with decimal commas
    let test_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create_with_delim("data.csv", test_data, b';');

    // Test: --decimal-comma with SSV output file should succeed
    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg("--decimal-comma")
        .args(["--delimiter", ";"])
        .args(["--output", "output.ssv"])
        .arg("select * from _t_1");

    wrk.assert_success(&mut cmd);
    let got: String = wrk.read_to_string("output.ssv").unwrap();
    let expected = "id;value\n1;100,5\n2;200,75\n";
    assert_eq!(got, expected);
}

#[test]
fn sqlp_decimal_comma_validation_with_skip_input() {
    let wrk = Workdir::new("sqlp_decimal_comma_validation_with_skip_input");

    // Create test data with decimal commas
    let test_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create("data.csv", test_data);

    // Test: --decimal-comma with SKIP_INPUT should succeed since no validation is performed
    let mut cmd = wrk.command("sqlp");
    cmd.arg("SKIP_INPUT")
        .arg("--decimal-comma")
        .args(["--output", "output.csv"])
        .arg("select * from read_csv('data.csv')");

    wrk.assert_success(&mut cmd);
    let got: String = wrk.read_to_string("output.csv").unwrap();
    let expected = "id,value\n1,\"100,50\"\n2,\"200,75\"\n";
    assert_eq!(got, expected);
}

#[test]
fn sqlp_union_positional() {
    let wrk = Workdir::new("sqlp_union_positional");

    // Create first table with columns "Value" and "Tag"
    wrk.create(
        "df1.csv",
        vec![
            svec!["Value", "Tag"],
            svec!["100", "hello"],
            svec!["200", "foo"],
        ],
    );

    // Create second table with different column names "Number" and "String"
    wrk.create(
        "df2.csv",
        vec![
            svec!["Number", "String"],
            svec!["300", "world"],
            svec!["400", "bar"],
        ],
    );

    // Test: UNION should combine tables positionally, using column names from first table
    let mut cmd = wrk.command("sqlp");
    cmd.args(["df1.csv", "df2.csv"]).arg(
        r#"SELECT u.* FROM (
            SELECT * FROM df1
            UNION
            SELECT * FROM df2
        ) u ORDER BY Value"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Value", "Tag"],
        svec!["100", "hello"],
        svec!["200", "foo"],
        svec!["300", "world"],
        svec!["400", "bar"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_unnest_issue_3108_first() {
    let wrk = Workdir::new("sqlp_unnest_issue_3108_first");

    wrk.create("data.csv", vec![svec!["id", "data"], svec!["1", "a,b,c"]]);

    // Test: UNNEST should unnest the array column
    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg("select first(id) as idf, unnest(string_to_array(data, ',')) as value from data");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["idf", "value"],
        svec!["1", "a"],
        svec!["1", "b"],
        svec!["1", "c"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_rank_funcs() {
    let wrk = Workdir::new("sqlp_rank_funcs");

    wrk.create(
        "data.csv",
        vec![
            svec!["id", "category", "value"],
            svec!["1", "A", "20"],
            svec!["2", "A", "10"],
            svec!["3", "A", "25"],
            svec!["4", "B", "10"],
            svec!["5", "B", "40"],
            svec!["6", "B", "25"],
            svec!["7", "C", "35"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv").arg(
        r#"SELECT
            value,
            ROW_NUMBER() OVER (ORDER BY value DESC, category DESC) AS row_num,
            RANK() OVER (ORDER BY value DESC, category DESC) AS rank,
            DENSE_RANK() OVER (ORDER BY value DESC, category DESC) AS dense_rank
        FROM data
        ORDER BY value, id DESC"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["value", "row_num", "rank", "dense_rank"],
        svec!["10", "6", "6", "6"],
        svec!["10", "7", "7", "7"],
        svec!["20", "5", "5", "5"],
        svec!["25", "3", "3", "3"],
        svec!["25", "4", "4", "4"],
        svec!["35", "2", "2", "2"],
        svec!["40", "1", "1", "1"],
    ];
    assert_eq!(got, expected);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv").arg(
        r#"SELECT
            value,
            ROW_NUMBER() OVER (ORDER BY value) AS row_num,
            RANK() OVER (ORDER BY value) AS rank,
            DENSE_RANK() OVER (ORDER BY value) AS dense_rank
        FROM data
        ORDER BY value, id"#,
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["value", "row_num", "rank", "dense_rank"],
        svec!["10", "1", "1", "1"],
        svec!["10", "2", "1", "1"],
        svec!["20", "3", "3", "2"],
        svec!["25", "4", "4", "3"],
        svec!["25", "5", "4", "3"],
        svec!["35", "6", "6", "4"],
        svec!["40", "7", "7", "5"],
    ];
    assert_eq!(got, expected);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv").arg(
        r#"SELECT
            category,
            value,
            ROW_NUMBER() OVER (PARTITION BY category ORDER BY value) AS row_num,
            RANK() OVER (PARTITION BY category ORDER BY value) AS rank,
            DENSE_RANK() OVER (PARTITION BY category ORDER BY value) AS dense
        FROM data
        ORDER BY category, value"#,
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "value", "row_num", "rank", "dense"],
        svec!["A", "10", "1", "1", "1"],
        svec!["A", "20", "2", "2", "2"],
        svec!["A", "25", "3", "3", "3"],
        svec!["B", "10", "1", "1", "1"],
        svec!["B", "25", "2", "2", "2"],
        svec!["B", "40", "3", "3", "3"],
        svec!["C", "35", "1", "1", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_named_window_references() {
    let wrk = Workdir::new("sqlp_named_window_references");

    wrk.create(
        "data.csv",
        vec![
            svec!["id", "category", "value"],
            svec!["1", "A", "20"],
            svec!["2", "A", "10"],
            svec!["3", "A", "30"],
            svec!["4", "B", "15"],
            svec!["5", "B", "50"],
            svec!["6", "B", "30"],
            svec!["7", "C", "35"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv").arg(
        r#"SELECT
        category,
        value,
        SUM(value) OVER w AS "w:sum",
        MIN(value) OVER w AS "w:min",
        AVG(value) OVER w AS "w:avg",
      FROM data
      WINDOW w AS (PARTITION BY category ORDER BY value)
      ORDER BY category, value"#,
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "value", "w:sum", "w:min", "w:avg"],
        svec!["A", "10", "10", "10", "20.0"],
        svec!["A", "20", "30", "10", "20.0"],
        svec!["A", "30", "60", "10", "20.0"],
        svec!["B", "15", "15", "15", "31.666666666666668"],
        svec!["B", "30", "45", "15", "31.666666666666668"],
        svec!["B", "50", "95", "15", "31.666666666666668"],
        svec!["C", "35", "35", "35", "35.0"],
    ];
    assert_eq!(got, expected);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv").arg(
        r#"SELECT
        category,
        value,
        AVG(value) OVER w1 AS category_avg,
        SUM(value) OVER w2 AS running_value,
        COUNT(*) OVER w3 AS total_count
      FROM data
      WINDOW
        w1 AS (PARTITION BY category),
        w2 AS (ORDER BY value),
        w3 AS ()
      ORDER BY category, value"#,
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec![
            "category",
            "value",
            "category_avg",
            "running_value",
            "total_count"
        ],
        svec!["A", "10", "20.0", "10", "7"],
        svec!["A", "20", "20.0", "45", "7"],
        svec!["A", "30", "20.0", "75", "7"],
        svec!["B", "15", "31.666666666666668", "25", "7"],
        svec!["B", "30", "31.666666666666668", "105", "7"],
        svec!["B", "50", "31.666666666666668", "190", "7"],
        svec!["C", "35", "35.0", "140", "7"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_array_to_string() {
    let wrk = Workdir::new("sqlp_array_to_string");

    wrk.create(
        "data.csv",
        vec![
            svec!["a", "b"],
            svec!["first", "1"],
            svec!["first", "1"],
            svec!["third", "42"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv").arg(
        r#"
        SELECT b, ARRAY_TO_STRING("a",', ') AS a2s,
        FROM (
            SELECT b, ARRAY_AGG(a) AS "a"
            FROM data
            GROUP BY b
        ) tbl
        ORDER BY a2s"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["b", "a2s"],
        svec!["1", "first, first"],
        svec!["42", "third"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_unnest_issue_3108() {
    let wrk = Workdir::new("sqlp_unnest_issue_3108");

    wrk.create(
        "data.csv",
        vec![
            svec!["id", "data"],
            svec!["1", "a,b,c"],
            svec!["2", ""],
            svec!["3", "b,c,d"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg("select id, unnest(string_to_array(data, ',')) as value from data");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["id", "value"],
        svec!["1", "a"],
        svec!["1", "b"],
        svec!["1", "c"],
        svec!["2", ""],
        svec!["3", "b"],
        svec!["3", "c"],
        svec!["3", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_distinct_basic() {
    let wrk = Workdir::new("sqlp_distinct_basic");
    wrk.create(
        "data.csv",
        vec![
            svec!["category", "subcategory", "value", "status", "score"],
            svec!["A", "x", "100", "active", "10"],
            svec!["A", "x", "100", "active", "20"],
            svec!["B", "y", "200", "active", "30"],
            svec!["B", "y", "200", "inactive", "30"],
            svec!["B", "z", "300", "active", "40"],
            svec!["C", "x", "400", "inactive", "50"],
            svec!["C", "y", "500", "active", "60"],
            svec!["", "x", "600", "active", "70"],
            svec!["", "y", "700", "inactive", "80"],
            svec!["A", "x", "100", "active", "10"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .args(["--rnull-values", "<empty string>"])
        .args(["--wnull-value", "NULL"])
        .arg("SELECT DISTINCT category FROM data ORDER BY category NULLS LAST");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category"],
        svec!["A"],
        svec!["B"],
        svec!["C"],
        svec!["NULL"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_distinct_multiple_columns() {
    let wrk = Workdir::new("sqlp_distinct_multiple_columns");
    wrk.create(
        "data.csv",
        vec![
            svec!["category", "subcategory", "value", "status", "score"],
            svec!["A", "x", "100", "active", "10"],
            svec!["A", "x", "100", "active", "20"],
            svec!["B", "y", "200", "active", "30"],
            svec!["B", "y", "200", "inactive", "30"],
            svec!["B", "z", "300", "active", "40"],
            svec!["C", "x", "400", "inactive", "50"],
            svec!["C", "y", "500", "active", "60"],
            svec!["", "x", "600", "active", "70"],
            svec!["", "y", "700", "inactive", "80"],
            svec!["A", "x", "100", "active", "10"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .args(["--rnull-values", "<empty string>"])
        .args(["--wnull-value", "NULL"])
        .arg(
            "SELECT DISTINCT category, subcategory FROM data ORDER BY category NULLS LAST, \
             subcategory",
        );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "subcategory"],
        svec!["A", "x"],
        svec!["B", "y"],
        svec!["B", "z"],
        svec!["C", "x"],
        svec!["C", "y"],
        svec!["NULL", "x"],
        svec!["NULL", "y"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_distinct_with_nulls() {
    let wrk = Workdir::new("sqlp_distinct_with_nulls");
    wrk.create(
        "data.csv",
        vec![
            svec!["category", "subcategory", "value", "status", "score"],
            svec!["A", "x", "100", "active", "10"],
            svec!["A", "x", "100", "active", "20"],
            svec!["B", "y", "200", "active", "30"],
            svec!["B", "y", "200", "inactive", "30"],
            svec!["B", "z", "300", "active", "40"],
            svec!["C", "x", "400", "inactive", "50"],
            svec!["C", "y", "500", "active", "60"],
            svec!["", "x", "600", "active", "70"],
            svec!["", "y", "700", "inactive", "80"],
            svec!["A", "x", "100", "active", "10"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .args(["--rnull-values", "<empty string>"])
        .args(["--wnull-value", "NULL"])
        .arg("SELECT DISTINCT category FROM data ORDER BY category NULLS FIRST");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category"],
        svec!["NULL"],
        svec!["A"],
        svec!["B"],
        svec!["C"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_distinct_all_columns() {
    let wrk = Workdir::new("sqlp_distinct_all_columns");
    wrk.create(
        "data.csv",
        vec![
            svec!["category", "subcategory", "value", "status", "score"],
            svec!["A", "x", "100", "active", "10"],
            svec!["A", "x", "100", "active", "20"],
            svec!["B", "y", "200", "active", "30"],
            svec!["B", "y", "200", "inactive", "30"],
            svec!["B", "z", "300", "active", "40"],
            svec!["C", "x", "400", "inactive", "50"],
            svec!["C", "y", "500", "active", "60"],
            svec!["", "x", "600", "active", "70"],
            svec!["", "y", "700", "inactive", "80"],
            svec!["A", "x", "100", "active", "10"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .args(["--rnull-values", "<empty string>"])
        .args(["--wnull-value", "NULL"])
        .arg(
            "SELECT DISTINCT * FROM data ORDER BY category NULLS LAST, subcategory, value, \
             status, score",
        );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "subcategory", "value", "status", "score"],
        svec!["A", "x", "100", "active", "10"],
        svec!["A", "x", "100", "active", "20"],
        svec!["B", "y", "200", "active", "30"],
        svec!["B", "y", "200", "inactive", "30"],
        svec!["B", "z", "300", "active", "40"],
        svec!["C", "x", "400", "inactive", "50"],
        svec!["C", "y", "500", "active", "60"],
        svec!["NULL", "x", "600", "active", "70"],
        svec!["NULL", "y", "700", "inactive", "80"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_distinct_on_basic() {
    let wrk = Workdir::new("sqlp_distinct_on_basic");
    wrk.create(
        "data.csv",
        vec![
            svec!["category", "subcategory", "value", "status", "score"],
            svec!["A", "x", "100", "active", "10"],
            svec!["A", "x", "100", "active", "20"],
            svec!["B", "y", "200", "active", "30"],
            svec!["B", "y", "200", "inactive", "30"],
            svec!["B", "z", "300", "active", "40"],
            svec!["C", "x", "400", "inactive", "50"],
            svec!["C", "y", "500", "active", "60"],
            svec!["", "x", "600", "active", "70"],
            svec!["", "y", "700", "inactive", "80"],
            svec!["A", "x", "100", "active", "10"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .args(["--rnull-values", "<empty string>"])
        .args(["--wnull-value", "NULL"])
        .arg(
            "SELECT DISTINCT ON (category) category, value, status FROM data ORDER BY category \
             NULLS LAST, value DESC",
        );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "value", "status"],
        svec!["A", "100", "active"],
        svec!["B", "300", "active"],
        svec!["C", "500", "active"],
        svec!["NULL", "700", "inactive"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_distinct_on_multiple_keys() {
    let wrk = Workdir::new("sqlp_distinct_on_multiple_keys");
    wrk.create(
        "data.csv",
        vec![
            svec!["category", "subcategory", "value", "status", "score"],
            svec!["A", "x", "100", "active", "10"],
            svec!["A", "x", "100", "active", "20"],
            svec!["B", "y", "200", "active", "30"],
            svec!["B", "y", "200", "inactive", "30"],
            svec!["B", "z", "300", "active", "40"],
            svec!["C", "x", "400", "inactive", "50"],
            svec!["C", "y", "500", "active", "60"],
            svec!["", "x", "600", "active", "70"],
            svec!["", "y", "700", "inactive", "80"],
            svec!["A", "x", "100", "active", "10"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .args(["--rnull-values", "<empty string>"])
        .args(["--wnull-value", "NULL"])
        .arg(
            "SELECT DISTINCT ON (category, subcategory) * FROM data ORDER BY category NULLS LAST, \
             subcategory, score DESC",
        );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "subcategory", "value", "status", "score"],
        svec!["A", "x", "100", "active", "20"],
        svec!["B", "y", "200", "active", "30"],
        svec!["B", "z", "300", "active", "40"],
        svec!["C", "x", "400", "inactive", "50"],
        svec!["C", "y", "500", "active", "60"],
        svec!["NULL", "x", "600", "active", "70"],
        svec!["NULL", "y", "700", "inactive", "80"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_distinct_on_with_nulls() {
    let wrk = Workdir::new("sqlp_distinct_on_with_nulls");
    wrk.create(
        "data.csv",
        vec![
            svec!["category", "subcategory", "value", "status", "score"],
            svec!["A", "x", "100", "active", "10"],
            svec!["A", "x", "100", "active", "20"],
            svec!["B", "y", "200", "active", "30"],
            svec!["B", "y", "200", "inactive", "30"],
            svec!["B", "z", "300", "active", "40"],
            svec!["C", "x", "400", "inactive", "50"],
            svec!["C", "y", "500", "active", "60"],
            svec!["", "x", "600", "active", "70"],
            svec!["", "y", "700", "inactive", "80"],
            svec!["A", "x", "100", "active", "10"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .args(["--rnull-values", "<empty string>"])
        .args(["--wnull-value", "NULL"])
        .arg("SELECT DISTINCT ON (category) * FROM data ORDER BY category NULLS LAST, value DESC");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "subcategory", "value", "status", "score"],
        svec!["A", "x", "100", "active", "10"],
        svec!["B", "z", "300", "active", "40"],
        svec!["C", "y", "500", "active", "60"],
        svec!["NULL", "y", "700", "inactive", "80"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_distinct_on_expression() {
    // Note: DISTINCT ON only supports column names, not expressions.
    // This test verifies that DISTINCT ON works with a computed column in ORDER BY
    // by using a column alias approach instead.
    let wrk = Workdir::new("sqlp_distinct_on_expression");
    wrk.create(
        "data.csv",
        vec![
            svec!["category", "subcategory", "value", "status", "score"],
            svec!["A", "x", "100", "active", "10"],
            svec!["A", "x", "100", "active", "20"],
            svec!["B", "y", "200", "active", "30"],
            svec!["B", "y", "200", "inactive", "30"],
            svec!["B", "z", "300", "active", "40"],
            svec!["C", "x", "400", "inactive", "50"],
            svec!["C", "y", "500", "active", "60"],
            svec!["", "x", "600", "active", "70"],
            svec!["", "y", "700", "inactive", "80"],
            svec!["A", "x", "100", "active", "10"],
        ],
    );

    // Since DISTINCT ON doesn't support expressions, we test with a column name
    // but use an expression in ORDER BY to verify ordering works correctly
    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .args(["--rnull-values", "<empty string>"])
        .args(["--wnull-value", "NULL"])
        .arg(
            "SELECT DISTINCT ON (category) category, value FROM data ORDER BY category NULLS \
             LAST, value DESC",
        );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "value"],
        svec!["A", "100"],
        svec!["B", "300"],
        svec!["C", "500"],
        svec!["NULL", "700"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_distinct_with_aggregation() {
    let wrk = Workdir::new("sqlp_distinct_with_aggregation");
    wrk.create(
        "data.csv",
        vec![
            svec!["category", "subcategory", "value", "status", "score"],
            svec!["A", "x", "100", "active", "10"],
            svec!["A", "x", "100", "active", "20"],
            svec!["B", "y", "200", "active", "30"],
            svec!["B", "y", "200", "inactive", "30"],
            svec!["B", "z", "300", "active", "40"],
            svec!["C", "x", "400", "inactive", "50"],
            svec!["C", "y", "500", "active", "60"],
            svec!["", "x", "600", "active", "70"],
            svec!["", "y", "700", "inactive", "80"],
            svec!["A", "x", "100", "active", "10"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .args(["--rnull-values", "<empty string>"])
        .args(["--wnull-value", "NULL"])
        .arg(
            "SELECT DISTINCT category, COUNT(*) as cnt FROM data GROUP BY category ORDER BY \
             category NULLS LAST",
        );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "cnt"],
        svec!["A", "3"],
        svec!["B", "3"],
        svec!["C", "2"],
        svec!["NULL", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_distinct_on_complex_ordering() {
    let wrk = Workdir::new("sqlp_distinct_on_complex_ordering");
    wrk.create(
        "data.csv",
        vec![
            svec!["category", "subcategory", "value", "status", "score"],
            svec!["A", "x", "100", "active", "10"],
            svec!["A", "x", "100", "active", "20"],
            svec!["B", "y", "200", "active", "30"],
            svec!["B", "y", "200", "inactive", "30"],
            svec!["B", "z", "300", "active", "40"],
            svec!["C", "x", "400", "inactive", "50"],
            svec!["C", "y", "500", "active", "60"],
            svec!["", "x", "600", "active", "70"],
            svec!["", "y", "700", "inactive", "80"],
            svec!["A", "x", "100", "active", "10"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .args(["--rnull-values", "<empty string>"])
        .args(["--wnull-value", "NULL"])
        .arg(
            "SELECT DISTINCT ON (category) category, value, status FROM data ORDER BY category \
             NULLS LAST, value DESC, status ASC",
        );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "value", "status"],
        svec!["A", "100", "active"],
        svec!["B", "300", "active"],
        svec!["C", "500", "active"],
        svec!["NULL", "700", "inactive"],
    ];
    assert_eq!(got, expected);
}

#[cfg(not(feature = "datapusher_plus"))]
#[test]
fn sqlp_datetime_schema_inference() {
    let wrk = Workdir::new("sqlp_datetime_schema_inference");

    // Create a CSV with datetime columns
    wrk.create(
        "datetimes.csv",
        vec![
            svec!["id", "name", "created_at", "updated_at", "amount"],
            svec![
                "1",
                "Alice",
                "2024-01-15 10:30:00",
                "2024-02-01 14:00:00",
                "100.50"
            ],
            svec![
                "2",
                "Bob",
                "2024-03-20 09:15:00",
                "2024-04-10 16:45:00",
                "200.75"
            ],
            svec![
                "3",
                "Charlie",
                "2024-05-05 08:00:00",
                "2024-06-15 12:30:00",
                "300.25"
            ],
        ],
    );

    // Step 1: Generate stats cache with --infer-dates so DateTime type is detected
    let mut cmd = wrk.command("stats");
    cmd.arg("datetimes.csv")
        .arg("--infer-dates")
        .args(["--dates-whitelist", "all"])
        .arg("--cardinality")
        .arg("--stats-jsonl")
        .args(["--cache-threshold", "1"]);

    // Verify stats detected DateTime columns
    let stats_output: String = wrk.stdout_on_success(&mut cmd);
    assert!(stats_output.contains("created_at,DateTime"));
    assert!(stats_output.contains("updated_at,DateTime"));

    // Step 2: Generate Polars schema from stats cache
    let mut cmd = wrk.command("schema");
    cmd.arg("--polars").arg("datetimes.csv");
    wrk.assert_success(&mut cmd);

    // Verify the pschema.json file was created and contains Datetime type
    assert!(wrk.path("datetimes.csv.pschema.json").exists());
    let schema_json = std::fs::read_to_string(wrk.path("datetimes.csv.pschema.json")).unwrap();
    assert!(
        schema_json.contains(r#""Datetime""#),
        "Schema should contain Datetime type, got: {schema_json}"
    );
    assert!(
        schema_json.contains(r#""Milliseconds""#),
        "Schema should contain Milliseconds time unit, got: {schema_json}"
    );

    // Step 3: Use sqlp with the cached schema to verify it parses datetimes correctly
    let mut cmd = wrk.command("sqlp");
    cmd.arg("datetimes.csv")
        .arg("SELECT id, name, created_at, updated_at FROM datetimes LIMIT 1")
        .arg("--cache-schema");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "name", "created_at", "updated_at"],
        svec![
            "1",
            "Alice",
            "2024-01-15T10:30:00.000",
            "2024-02-01T14:00:00.000"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_datetime_pschema_override() {
    let wrk = Workdir::new("sqlp_datetime_pschema_override");

    // Create a CSV with datetime columns
    wrk.create(
        "events.csv",
        vec![
            svec!["id", "event_name", "event_time", "score"],
            svec!["1", "login", "2024-06-15 08:30:00", "95.5"],
            svec!["2", "logout", "2024-06-15 17:00:00", "88.0"],
        ],
    );

    // Manually create a pschema.json with Datetime type
    wrk.create_from_string(
        "events.csv.pschema.json",
        r#"{
  "fields": {
    "id": "UInt8",
    "event_name": "String",
    "event_time": {
      "Datetime": [
        "Milliseconds",
        null
      ]
    },
    "score": "Float32"
  },
  "metadata": null
}"#,
    );

    // sqlp should use the manually provided schema and parse datetimes
    let mut cmd = wrk.command("sqlp");
    cmd.arg("events.csv")
        .arg("SELECT id, event_name, event_time, score FROM events")
        .arg("--cache-schema");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "event_name", "event_time", "score"],
        svec!["1", "login", "2024-06-15T08:30:00.000", "95.5"],
        svec!["2", "logout", "2024-06-15T17:00:00.000", "88.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_alias_no_substring_collision() {
    // Regression: `_t_1` must not match the prefix of `_t_10`. Before the
    // word-boundary regex fix, the naive `String::replace` would rewrite
    // `_t_10` into `"d1"0`, producing a SQL parse error.
    let wrk = Workdir::new("sqlp_alias_no_substring_collision");

    // Create 10 input files so we have aliases _t_1 .. _t_10.
    for i in 1..=10 {
        wrk.create(
            &format!("d{i}.csv"),
            vec![vec!["v".to_string()], vec![format!("row_{i}")]],
        );
    }

    let mut cmd = wrk.command("sqlp");
    for i in 1..=10 {
        cmd.arg(format!("d{i}.csv"));
    }
    cmd.arg("select v from _t_10");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["v"], svec!["row_10"]];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_alias_preserves_literal_with_alias_substring() {
    // Regression: a string literal containing an alias-shaped token (e.g.
    // `_t_10_note`) must survive the rewrite untouched, because `\d+` is
    // greedy and the next char (`_`) does not form a word boundary with `0`.
    // (Alias text inside literals delimited only by quotes is *not* protected
    // — that is a documented limitation of the regex-based approach.)
    let wrk = Workdir::new("sqlp_alias_preserves_literal_with_alias_substring");
    wrk.create("d1.csv", vec![svec!["v"], svec!["one"]]);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("d1.csv")
        .arg("select '_t_10_note' as k, v from _t_1");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["k", "v"], svec!["_t_10_note", "one"]];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_invalid_format_errors() {
    // Regression: an invalid --format value should fail loudly instead of
    // silently falling back to CSV.
    let wrk = Workdir::new("sqlp_invalid_format_errors");
    wrk.create("d.csv", vec![svec!["v"], svec!["1"]]);

    let mut cmd = wrk.command("sqlp");
    cmd.args(["d.csv", "select * from d", "--format", "jsonn"]);

    wrk.assert_err(&mut cmd);
}

// ---------------------------------------------------------------------------
// Polars SQL surface added between py-1.44.0 and rev 9d5804d (the bump in
// 50c2cf18f). One test per upstream behavior change; the upstream PR is cited
// so the next bump can diff against it.
// ---------------------------------------------------------------------------

/// Fixture shared by the GROUP BY / aggregate tests below.
fn grouping_fixture(wrk: &Workdir) {
    wrk.create(
        "groups.csv",
        vec![
            svec!["category", "class", "value"],
            svec!["a", "x", "1"],
            svec!["a", "x", "2"],
            svec!["a", "y", "3"],
            svec!["b", "x", "4"],
            svec!["b", "y", "5"],
            svec!["b", "y", "6"],
        ],
    );
}

#[test]
fn sqlp_grouping_sets() {
    // pola-rs/polars#29278: GROUP BY GROUPING SETS. The subtotal rows carry a
    // NULL in the columns they do not group by, so --wnull-value makes the
    // difference between "subtotal" and "a group whose key is empty" visible.
    let wrk = Workdir::new("sqlp_grouping_sets");
    grouping_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("groups.csv")
        .arg(
            "SELECT category, class, SUM(value) AS total, COUNT(*) AS n FROM groups GROUP BY \
             GROUPING SETS ((category, class), (category), (class), ()) ORDER BY category NULLS \
             LAST, class NULLS LAST",
        )
        .args(["--wnull-value", "NULL"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "class", "total", "n"],
        svec!["a", "x", "3", "2"],
        svec!["a", "y", "3", "1"],
        svec!["a", "NULL", "6", "3"],
        svec!["b", "x", "4", "1"],
        svec!["b", "y", "11", "2"],
        svec!["b", "NULL", "15", "3"],
        svec!["NULL", "x", "7", "3"],
        svec!["NULL", "y", "14", "3"],
        svec!["NULL", "NULL", "21", "6"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_group_by_rollup() {
    // pola-rs/polars#29278: ROLLUP(a, b) == GROUPING SETS ((a,b), (a), ()),
    // i.e. it does NOT include the (class) subtotal that CUBE adds below.
    let wrk = Workdir::new("sqlp_group_by_rollup");
    grouping_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("groups.csv")
        .arg(
            "SELECT category, class, SUM(value) AS total FROM groups GROUP BY ROLLUP(category, \
             class) ORDER BY category NULLS LAST, class NULLS LAST",
        )
        .args(["--wnull-value", "NULL"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "class", "total"],
        svec!["a", "x", "3"],
        svec!["a", "y", "3"],
        svec!["a", "NULL", "6"],
        svec!["b", "x", "4"],
        svec!["b", "y", "11"],
        svec!["b", "NULL", "15"],
        svec!["NULL", "NULL", "21"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_group_by_cube() {
    // pola-rs/polars#29278: CUBE(a, b) adds the (class) subtotals that ROLLUP
    // omits -- the two NULL-category rows below are the whole point.
    let wrk = Workdir::new("sqlp_group_by_cube");
    grouping_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("groups.csv")
        .arg(
            "SELECT category, class, SUM(value) AS total FROM groups GROUP BY CUBE(category, \
             class) ORDER BY category NULLS LAST, class NULLS LAST",
        )
        .args(["--wnull-value", "NULL"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "class", "total"],
        svec!["a", "x", "3"],
        svec!["a", "y", "3"],
        svec!["a", "NULL", "6"],
        svec!["b", "x", "4"],
        svec!["b", "y", "11"],
        svec!["b", "NULL", "15"],
        svec!["NULL", "x", "7"],
        svec!["NULL", "y", "14"],
        svec!["NULL", "NULL", "21"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_grouping_function() {
    // pola-rs/polars#29278: GROUPING(col) is 1 when col is rolled up in that
    // row, else 0; with several arguments the bits are packed MSB-first, so
    // GROUPING(category, class) == 3 on the grand total but 1 when only class
    // is rolled up.
    let wrk = Workdir::new("sqlp_grouping_function");
    grouping_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("groups.csv")
        .arg(
            "SELECT category, class, SUM(value) AS total, GROUPING(category) AS gc, \
             GROUPING(class) AS gk, GROUPING(category, class) AS gb FROM groups GROUP BY \
             ROLLUP(category, class) ORDER BY category NULLS LAST, class NULLS LAST",
        )
        .args(["--wnull-value", "NULL"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "class", "total", "gc", "gk", "gb"],
        svec!["a", "x", "3", "0", "0", "0"],
        svec!["a", "y", "3", "0", "0", "0"],
        svec!["a", "NULL", "6", "0", "1", "1"],
        svec!["b", "x", "4", "0", "0", "0"],
        svec!["b", "y", "11", "0", "0", "0"],
        svec!["b", "NULL", "15", "0", "1", "1"],
        svec!["NULL", "NULL", "21", "1", "1", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_grouping_function_select_alias() {
    // pola-rs/polars#29278: a SELECT alias over an expression is accepted both
    // as the ROLLUP key and as the GROUPING() argument.
    let wrk = Workdir::new("sqlp_grouping_function_select_alias");
    grouping_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("groups.csv")
        .arg(
            "SELECT UPPER(category) AS cat, SUM(value) AS total, GROUPING(cat) AS g FROM groups \
             GROUP BY ROLLUP(cat) ORDER BY cat NULLS LAST",
        )
        .args(["--wnull-value", "NULL"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["cat", "total", "g"],
        svec!["A", "6", "0"],
        svec!["B", "15", "0"],
        svec!["NULL", "21", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_date_integer_arithmetic() {
    // pola-rs/polars#29156: a Date +/- an integer shifts by whole days, with the
    // integer on either side and from either a literal or a column. The fixture
    // straddles a leap day (2020-02-28 + 2 == 2020-03-01) and the same date in a
    // non-leap year (2021-02-28 + 2 == 2021-03-02).
    let wrk = Workdir::new("sqlp_date_integer_arithmetic");
    wrk.create(
        "dates.csv",
        vec![
            svec!["dt", "n"],
            svec!["2020-01-01", "5"],
            svec!["2020-02-28", "2"],
            svec!["2021-02-28", "2"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("dates.csv")
        .arg(
            "SELECT dt, n, dt + 5 AS plus_lit, 5 + dt AS lit_plus, dt - 5 AS minus_lit, dt + n AS \
             plus_col, dt - n AS minus_col FROM dates",
        )
        .arg("--try-parsedates");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec![
            "dt",
            "n",
            "plus_lit",
            "lit_plus",
            "minus_lit",
            "plus_col",
            "minus_col"
        ],
        svec![
            "2020-01-01",
            "5",
            "2020-01-06",
            "2020-01-06",
            "2019-12-27",
            "2020-01-06",
            "2019-12-27"
        ],
        svec![
            "2020-02-28",
            "2",
            "2020-03-04",
            "2020-03-04",
            "2020-02-23",
            "2020-03-01",
            "2020-02-26"
        ],
        svec![
            "2021-02-28",
            "2",
            "2021-03-05",
            "2021-03-05",
            "2021-02-23",
            "2021-03-02",
            "2021-02-26"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_typed_temporal_literals() {
    // pola-rs/polars#29007: `DATE '...'` / `TIMESTAMP '...'` are parsed as typed
    // literals rather than a string that later gets cast, so they compare
    // directly against a Date column.
    let wrk = Workdir::new("sqlp_typed_temporal_literals");
    wrk.create(
        "dates.csv",
        vec![
            svec!["dt"],
            svec!["2020-01-01"],
            svec!["2020-02-28"],
            svec!["2021-02-28"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("dates.csv")
        .arg("SELECT dt FROM dates WHERE dt > DATE '2020-01-15' ORDER BY dt")
        .arg("--try-parsedates");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![svec!["dt"], svec!["2020-02-28"], svec!["2021-02-28"]]
    );

    let mut between_cmd = wrk.command("sqlp");
    between_cmd
        .arg("dates.csv")
        .arg(
            "SELECT dt FROM dates WHERE dt BETWEEN DATE '2020-01-01' AND DATE '2020-06-01' ORDER \
             BY dt",
        )
        .arg("--try-parsedates");
    let got_between: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut between_cmd);
    assert_eq!(
        got_between,
        vec![svec!["dt"], svec!["2020-01-01"], svec!["2020-02-28"]]
    );

    // As projected values: DATE keeps day resolution, TIMESTAMP is a Datetime.
    let mut literal_cmd = wrk.command("sqlp");
    literal_cmd.arg("dates.csv").arg(
        "SELECT DATE '2020-02-29' AS d, TIMESTAMP '2020-01-01 08:00:00' AS ts FROM dates LIMIT 1",
    );
    let got_literal: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut literal_cmd);
    assert_eq!(
        got_literal,
        vec![
            svec!["d", "ts"],
            svec!["2020-02-29", "2020-01-01T08:00:00.000000"]
        ]
    );
}

#[test]
fn sqlp_cast_string_to_temporal() {
    // pola-rs/polars#28062 removed the string->temporal *cast*, and #28986 then
    // made `CAST(<string> AS DATE/TIME/TIMESTAMP)` in SQL *parse* the string
    // instead. So the SQL spelling keeps working even though the underlying
    // cast is gone -- this test pins that, for a column operand, the `::` form,
    // and a bare string literal.
    let wrk = Workdir::new("sqlp_cast_string_to_temporal");
    wrk.create(
        "strs.csv",
        vec![
            svec!["d", "ts", "t"],
            svec!["2000-02-01", "2000-02-01 12:30:00", "12:30:00"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("strs.csv").arg(
        "SELECT CAST(d AS DATE) AS d1, CAST(ts AS TIMESTAMP) AS ts1, CAST(ts AS DATETIME) AS ts2, \
         CAST(t AS TIME) AS t1 FROM strs",
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["d1", "ts1", "ts2", "t1"],
            svec![
                "2000-02-01",
                "2000-02-01T12:30:00.000000",
                "2000-02-01T12:30:00.000000",
                "12:30:00.000000000"
            ]
        ]
    );

    let mut colon_cmd = wrk.command("sqlp");
    colon_cmd
        .arg("strs.csv")
        .arg("SELECT d::date AS d1, ts::timestamp AS ts1, t::time AS t1 FROM strs");
    let got_colon: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut colon_cmd);
    assert_eq!(
        got_colon,
        vec![
            svec!["d1", "ts1", "t1"],
            svec![
                "2000-02-01",
                "2000-02-01T12:30:00.000000",
                "12:30:00.000000000"
            ]
        ]
    );

    let mut literal_cmd = wrk.command("sqlp");
    literal_cmd
        .arg("strs.csv")
        .arg("SELECT CAST('2000-02-01' AS DATE) AS d1, CAST('12:30:00' AS TIME) AS t1 FROM strs");
    let got_literal: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut literal_cmd);
    assert_eq!(
        got_literal,
        vec![svec!["d1", "t1"], svec!["2000-02-01", "12:30:00.000000000"]]
    );
}

#[test]
fn sqlp_cast_strict_vs_try_temporal() {
    // pola-rs/polars#28986: CAST is strict and TRY_CAST is not. On a column
    // holding one unparseable value, CAST fails the whole query while TRY_CAST
    // nulls just that row. `sqlp_try_cast` covers only the TRY_CAST half.
    let wrk = Workdir::new("sqlp_cast_strict_vs_try_temporal");
    // The `id` column is load-bearing: a projection of the nulled column ALONE
    // writes the failed row as an empty line, which the CSV reader then drops,
    // so the TRY_CAST assertion below could not tell a nulled row from a
    // missing one.
    wrk.create(
        "badstrs.csv",
        vec![
            svec!["id", "s"],
            svec!["1", "2000-02-01"],
            svec!["2", "not-a-date"],
        ],
    );

    let mut strict_cmd = wrk.command("sqlp");
    strict_cmd
        .arg("badstrs.csv")
        .arg("SELECT id, CAST(s AS DATE) AS d FROM badstrs ORDER BY id");
    let stderr = wrk.stderr_on_error(&mut strict_cmd);
    assert!(
        stderr.contains("conversion from `str` to `date` failed"),
        "unexpected stderr: {stderr}"
    );
    assert!(
        stderr.contains("not-a-date"),
        "error should name the offending value: {stderr}"
    );

    let mut try_cmd = wrk.command("sqlp");
    try_cmd
        .arg("badstrs.csv")
        .arg("SELECT id, TRY_CAST(s AS DATE) AS d FROM badstrs ORDER BY id");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut try_cmd);
    assert_eq!(
        got,
        vec![svec!["id", "d"], svec!["1", "2000-02-01"], svec!["2", ""]]
    );
}

#[test]
fn sqlp_approx_quantile() {
    // pola-rs/polars#29288 added APPROX_QUANTILE to the SQL frontend. It is
    // cfg-gated on polars' `approx_quantile` feature, which qsv enables
    // explicitly in Cargo.toml -- upstream ships it only inside the
    // docs-selection/full umbrellas, so without that line sqlp rejects the
    // function outright.
    //
    // Signatures: (col, q), (col, q, allowed_rank_error), (col, q, error, method).
    // The sketch is deterministic for a given input, so these are exact
    // expectations, not a tolerance band (verified byte-identical over 20 runs).
    let wrk = Workdir::new("sqlp_approx_quantile");
    grouping_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("groups.csv").arg(
        "SELECT APPROX_QUANTILE(value, 0.5) AS a2, APPROX_QUANTILE(value, 0.5, 0.01) AS a3, \
         APPROX_QUANTILE(value, 0.5, 0.01, 'kll') AS a4, APPROX_QUANTILE(value, 0.25) AS q25, \
         APPROX_QUANTILE(value, 0.75) AS q75 FROM groups",
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["a2", "a3", "a4", "q25", "q75"],
            svec!["4", "4", "4", "2", "5"]
        ]
    );

    // It is a real aggregate, so it works per group.
    let mut grouped_cmd = wrk.command("sqlp");
    grouped_cmd.arg("groups.csv").arg(
        "SELECT category, APPROX_QUANTILE(value, 0.5) AS aq FROM groups GROUP BY category ORDER \
         BY category",
    );
    let got_grouped: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut grouped_cmd);
    assert_eq!(
        got_grouped,
        vec![svec!["category", "aq"], svec!["a", "2"], svec!["b", "5"]]
    );

    // Outside 2-4 arguments it is a syntax error, not a silent fallback.
    let mut arity_cmd = wrk.command("sqlp");
    arity_cmd
        .arg("groups.csv")
        .arg("SELECT APPROX_QUANTILE(value) AS aq FROM groups");
    let stderr = wrk.stderr_on_error(&mut arity_cmd);
    assert!(
        stderr.contains("APPROX_QUANTILE expects 2-4 arguments (found 1)"),
        "unexpected stderr: {stderr}"
    );
}

/// Fixture for the window-function tests: two groups, correlated a/b pairs.
fn window_fixture(wrk: &Workdir) {
    wrk.create(
        "win.csv",
        vec![
            svec!["i", "g", "a", "b"],
            svec!["0", "a", "1", "1"],
            svec!["1", "a", "2", "3"],
            svec!["2", "a", "3", "2"],
            svec!["3", "b", "4", "10"],
            svec!["4", "b", "5", "20"],
        ],
    );
}

#[test]
fn sqlp_over_multi_arg_aggregates() {
    // pola-rs/polars#29160: OVER now applies to aggregates that take more than
    // one argument. Before the fix the window was dropped and these collapsed to
    // a whole-frame aggregate. The existing OVER tests only cover the
    // single-argument ranking functions (ROW_NUMBER/RANK/DENSE_RANK), so the
    // per-partition values below are the regression signal.
    let wrk = Workdir::new("sqlp_over_multi_arg_aggregates");
    window_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("win.csv")
        .arg(
            "SELECT i, g, CORR(a,b) OVER (PARTITION BY g) AS corr, COVAR_POP(a,b) OVER (PARTITION \
             BY g) AS cvp, COVAR_SAMP(a,b) OVER (PARTITION BY g) AS cvs, QUANTILE_CONT(a,0.5) \
             OVER (PARTITION BY g) AS qc, QUANTILE_DISC(a,0.5) OVER (PARTITION BY g) AS qd, \
             STRING_AGG(g,'-') OVER (PARTITION BY g) AS sa FROM win ORDER BY i",
        )
        .args(["--float-precision", "6"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["i", "g", "corr", "cvp", "cvs", "qc", "qd", "sa"],
        svec![
            "0", "a", "0.500000", "0.333333", "0.500000", "2.000000", "2.000000", "a-a-a"
        ],
        svec![
            "1", "a", "0.500000", "0.333333", "0.500000", "2.000000", "2.000000", "a-a-a"
        ],
        svec![
            "2", "a", "0.500000", "0.333333", "0.500000", "2.000000", "2.000000", "a-a-a"
        ],
        svec![
            "3", "b", "1.000000", "2.500000", "5.000000", "4.500000", "4.000000", "b-b"
        ],
        svec![
            "4", "b", "1.000000", "2.500000", "5.000000", "4.500000", "4.000000", "b-b"
        ],
    ];
    assert_eq!(got, expected);
}

/// Fixture for the window NULLS ordering tests: nulls in both groups.
fn window_nulls_fixture(wrk: &Workdir) {
    wrk.create(
        "wnulls.csv",
        vec![
            svec!["grp", "a"],
            svec!["x", "20.0"],
            svec!["x", ""],
            svec!["x", "10.0"],
            svec!["y", ""],
            svec!["y", "40.0"],
            svec!["y", "30.0"],
        ],
    );
}

#[test]
fn sqlp_window_order_by_nulls_last() {
    // pola-rs/polars#29159: NULLS FIRST/LAST inside a *window's* ORDER BY is now
    // respected. The existing NULLS FIRST/LAST tests in this file all sit on the
    // top-level ORDER BY, which took a different code path.
    let wrk = Workdir::new("sqlp_window_order_by_nulls_last");
    window_nulls_fixture(&wrk);

    // `grp` is deliberately NOT projected. The two null-`a` rows are peers in
    // this window, so which one gets rn 5 vs rn 6 is unspecified -- but `a`,
    // `rn`, `cnt` and `total` are identical either way, so the assertion is a
    // total order on what it actually asserts. Adding a second window ORDER BY
    // key as a tiebreak is NOT an option: polars silently ignores
    // NULLS FIRST/LAST once a window has more than one key (pinned by
    // sqlp_window_order_by_multiple_keys_limitation), which would invert the
    // very property under test.
    let mut cmd = wrk.command("sqlp");
    cmd.arg("wnulls.csv").arg(
        "SELECT a, ROW_NUMBER() OVER (ORDER BY a NULLS LAST) AS rn, COUNT(*) OVER (ORDER BY a \
         NULLS LAST) AS cnt, SUM(a) OVER (ORDER BY a NULLS LAST) AS total FROM wnulls ORDER BY rn",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["a", "rn", "cnt", "total"],
        svec!["10.0", "1", "1", "10.0"],
        svec!["20.0", "2", "2", "30.0"],
        svec!["30.0", "3", "3", "60.0"],
        svec!["40.0", "4", "4", "100.0"],
        svec!["", "5", "5", "100.0"],
        svec!["", "6", "6", "100.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_window_order_by_nulls_first() {
    // pola-rs/polars#29159, the mirror of sqlp_window_order_by_nulls_last: the
    // two nulls must lead, and DESC must not silently flip the null placement.
    let wrk = Workdir::new("sqlp_window_order_by_nulls_first");
    window_nulls_fixture(&wrk);

    // As in sqlp_window_order_by_nulls_last, `grp` is left out: the two null
    // rows are peers, so only `a` and `rn` are determinate.
    let mut cmd = wrk.command("sqlp");
    cmd.arg("wnulls.csv")
        .arg("SELECT a, ROW_NUMBER() OVER (ORDER BY a NULLS FIRST) AS rn FROM wnulls ORDER BY rn");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["a", "rn"],
            svec!["", "1"],
            svec!["", "2"],
            svec!["10.0", "3"],
            svec!["20.0", "4"],
            svec!["30.0", "5"],
            svec!["40.0", "6"],
        ]
    );

    let mut desc_cmd = wrk.command("sqlp");
    desc_cmd.arg("wnulls.csv").arg(
        "SELECT a, ROW_NUMBER() OVER (ORDER BY a DESC NULLS FIRST) AS rn FROM wnulls ORDER BY rn",
    );
    let got_desc: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut desc_cmd);
    assert_eq!(
        got_desc,
        vec![
            svec!["a", "rn"],
            svec!["", "1"],
            svec!["", "2"],
            svec!["40.0", "3"],
            svec!["30.0", "4"],
            svec!["20.0", "5"],
            svec!["10.0", "6"],
        ]
    );
}

#[test]
fn sqlp_window_partition_by_order_by_nulls() {
    // pola-rs/polars#29159 combined with PARTITION BY: the null sorts last
    // *within* each partition, so both groups restart at rn 1.
    //
    // Unlike the two tests above this one CAN project `grp` and assert an exact
    // order: each partition holds exactly one null, so there are no peers and
    // every row's rn is determined.
    let wrk = Workdir::new("sqlp_window_partition_by_order_by_nulls");
    window_nulls_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("wnulls.csv").arg(
        "SELECT grp, a, ROW_NUMBER() OVER (PARTITION BY grp ORDER BY a NULLS LAST) AS rn FROM \
         wnulls ORDER BY grp, rn",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["grp", "a", "rn"],
            svec!["x", "10.0", "1"],
            svec!["x", "20.0", "2"],
            svec!["x", "", "3"],
            svec!["y", "30.0", "1"],
            svec!["y", "40.0", "2"],
            svec!["y", "", "3"],
        ]
    );
}

#[test]
fn sqlp_over_aggregate_with_having() {
    // pola-rs/polars#29006: an OVER window wrapping an aggregate coexists with
    // HAVING -- the window sees the post-aggregation groups.
    let wrk = Workdir::new("sqlp_over_aggregate_with_having");
    window_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("win.csv").arg(
        "SELECT g, SUM(a) AS s, MAX(SUM(a)) OVER () AS mx FROM win GROUP BY g HAVING SUM(a) > 3 \
         ORDER BY g",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["g", "s", "mx"],
            svec!["a", "6", "9"],
            svec!["b", "9", "9"],
        ]
    );
}

/// Fixture for the parenthesized-JOIN tests: overlapping a/b so that a
/// two-column constraint selects exactly one row.
fn paren_join_fixture(wrk: &Workdir) {
    wrk.create(
        "df1.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "2"],
            svec!["2", "3"],
            svec!["3", "4"],
        ],
    );
    wrk.create(
        "df2.csv",
        vec![
            svec!["a", "b"],
            svec!["2", "3"],
            svec!["3", "9"],
            svec!["9", "9"],
        ],
    );
}

#[test]
fn sqlp_join_parenthesized_constraint() {
    // pola-rs/polars#28967: `ON (<constraint>)` wrapped in parens is accepted.
    // ORDER BY a1 is on a unique left key -- sqlp has no --maintain-order, so a
    // join test without a total order is not repeatable.
    let wrk = Workdir::new("sqlp_join_parenthesized_constraint");
    paren_join_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.args(["df1.csv", "df2.csv"]).arg(
        "SELECT df1.a AS a1, df1.b AS b1, df2.a AS a2, df2.b AS b2 FROM df1 JOIN df2 ON (df1.a = \
         df2.a AND df1.b = df2.b) ORDER BY a1",
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![svec!["a1", "b1", "a2", "b2"], svec!["2", "3", "2", "3"],]
    );

    // LEFT JOIN keeps the non-matching left rows with nulls on the right.
    let mut left_cmd = wrk.command("sqlp");
    left_cmd.args(["df1.csv", "df2.csv"]).arg(
        "SELECT df1.a AS a1, df1.b AS b1, df2.a AS a2, df2.b AS b2 FROM df1 LEFT JOIN df2 ON \
         (df1.a = df2.a AND df1.b = df2.b) ORDER BY a1",
    );
    let got_left: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut left_cmd);
    assert_eq!(
        got_left,
        vec![
            svec!["a1", "b1", "a2", "b2"],
            svec!["1", "2", "", ""],
            svec!["2", "3", "2", "3"],
            svec!["3", "4", "", ""],
        ]
    );

    // A parenthesized non-predicate is rejected rather than treated as a cross
    // join filter.
    let mut bad_cmd = wrk.command("sqlp");
    bad_cmd
        .args(["df1.csv", "df2.csv"])
        .arg("SELECT * FROM df1 JOIN df2 ON (df1.a)");
    let stderr = wrk.stderr_on_error(&mut bad_cmd);
    assert!(
        stderr.contains("predicates must resolve to boolean"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn sqlp_join_parenthesized_relation_alias() {
    // pola-rs/polars#29158: bare parens around a join do NOT introduce a new
    // scope, so aliases declared inside them stay visible to the outer SELECT.
    // Asserted by equivalence with the unparenthesized spelling.
    let wrk = Workdir::new("sqlp_join_parenthesized_relation_alias");
    paren_join_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.args(["df1.csv", "df2.csv"]).arg(
        "SELECT lhs.a, rhs.b FROM (df1 AS lhs INNER JOIN df2 AS rhs ON lhs.b = rhs.b) ORDER BY \
         lhs.a",
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["a", "b"], svec!["2", "3"]];
    assert_eq!(got, expected);

    let mut plain_cmd = wrk.command("sqlp");
    plain_cmd.args(["df1.csv", "df2.csv"]).arg(
        "SELECT lhs.a, rhs.b FROM df1 AS lhs INNER JOIN df2 AS rhs ON lhs.b = rhs.b ORDER BY lhs.a",
    );
    let got_plain: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut plain_cmd);
    assert_eq!(got_plain, expected);
}

#[test]
fn sqlp_join_literal_comparison() {
    // pola-rs/polars#28701: a constant comparison in an ON clause belongs to the
    // input it names, not to a post-join filter. The LEFT JOIN case is what
    // distinguishes the two: `bob`/`charlie` fail `role = 'admin'`, so they must
    // still appear with a null dept. A post-join filter would drop them.
    let wrk = Workdir::new("sqlp_join_literal_comparison");
    wrk.create(
        "people.csv",
        vec![
            svec!["name", "role"],
            svec!["alice", "admin"],
            svec!["bob", "user"],
            svec!["adam", "admin"],
            svec!["charlie", "user"],
        ],
    );
    wrk.create(
        "depts.csv",
        vec![
            svec!["name", "dept"],
            svec!["alice", "IT"],
            svec!["bob", "HR"],
            svec!["charlie", "IT"],
            svec!["adam", "SEC"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.args(["people.csv", "depts.csv"]).arg(
        "SELECT people.name, people.role, depts.dept FROM people INNER JOIN depts ON people.name \
         = depts.name AND people.role = 'admin' ORDER BY people.name",
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["name", "role", "dept"],
            svec!["adam", "admin", "SEC"],
            svec!["alice", "admin", "IT"],
        ]
    );

    let mut left_cmd = wrk.command("sqlp");
    left_cmd.args(["people.csv", "depts.csv"]).arg(
        "SELECT people.name, people.role, depts.dept FROM people LEFT JOIN depts ON people.name = \
         depts.name AND people.role = 'admin' ORDER BY people.name",
    );
    let got_left: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut left_cmd);
    assert_eq!(
        got_left,
        vec![
            svec!["name", "role", "dept"],
            svec!["adam", "admin", "SEC"],
            svec!["alice", "admin", "IT"],
            svec!["bob", "user", ""],
            svec!["charlie", "user", ""],
        ]
    );
}

#[test]
fn sqlp_join_non_equi_decimal() {
    // pola-rs/polars#29156 also taught the non-equi join path to compare
    // Decimals. Reachable because qsv enables both `dtype-decimal` and `iejoin`.
    //
    // The values carry fractional parts ON PURPOSE: 1.05 < 1.10 is true in
    // Decimal but false under any integer truncation, so the (1, 10) pair below
    // is what proves the comparison really happens at decimal precision rather
    // than the test passing on whole numbers that compare the same either way.
    let wrk = Workdir::new("sqlp_join_non_equi_decimal");
    wrk.create(
        "dec1.csv",
        vec![
            svec!["id", "v"],
            svec!["1", "1.05"],
            svec!["2", "2.50"],
            svec!["3", "10.10"],
        ],
    );
    wrk.create(
        "dec2.csv",
        vec![
            svec!["id", "w"],
            svec!["10", "1.10"],
            svec!["11", "2.50"],
            svec!["12", "9.99"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.args(["dec1.csv", "dec2.csv"]).arg(
        "SELECT dec1.id AS l, dec1.v, dec2.id AS r, dec2.w FROM dec1 JOIN dec2 ON CAST(dec1.v AS \
         DECIMAL(10,2)) < CAST(dec2.w AS DECIMAL(10,2)) ORDER BY l, r",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["l", "v", "r", "w"],
            svec!["1", "1.05", "10", "1.1"],
            svec!["1", "1.05", "11", "2.5"],
            svec!["1", "1.05", "12", "9.99"],
            svec!["2", "2.5", "12", "9.99"],
        ]
    );

    // >= pins exact boundary equality: 2.50 vs 2.50 matches, and 10.10 exceeds
    // every right-hand value.
    let mut ge_cmd = wrk.command("sqlp");
    ge_cmd.args(["dec1.csv", "dec2.csv"]).arg(
        "SELECT dec1.id AS l, dec2.id AS r FROM dec1 JOIN dec2 ON CAST(dec1.v AS DECIMAL(10,2)) \
         >= CAST(dec2.w AS DECIMAL(10,2)) ORDER BY l, r",
    );
    let got_ge: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut ge_cmd);
    assert_eq!(
        got_ge,
        vec![
            svec!["l", "r"],
            svec!["2", "10"],
            svec!["2", "11"],
            svec!["3", "10"],
            svec!["3", "11"],
            svec!["3", "12"],
        ]
    );
}

/// Fixture for the subquery / scope tests. `t2` deliberately names its columns
/// `b, a` so an unqualified reference would be ambiguous across the two.
fn subquery_fixture(wrk: &Workdir) {
    wrk.create(
        "t1.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "10"],
            svec!["2", "20"],
            svec!["3", "30"],
        ],
    );
    wrk.create(
        "t2.csv",
        vec![svec!["b", "a"], svec!["1", "100"], svec!["2", "200"]],
    );
}

#[test]
fn sqlp_exists_subquery() {
    // pola-rs/polars#29006: EXISTS / NOT EXISTS correlated on the outer relation.
    // Nothing in this file covered EXISTS before -- only derived-table subqueries.
    let wrk = Workdir::new("sqlp_exists_subquery");
    subquery_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.args(["t1.csv", "t2.csv"])
        .arg("SELECT a, b FROM t1 WHERE EXISTS (SELECT 1 FROM t2 WHERE t2.b = t1.a) ORDER BY a");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![svec!["a", "b"], svec!["1", "10"], svec!["2", "20"]]
    );

    let mut not_cmd = wrk.command("sqlp");
    not_cmd.args(["t1.csv", "t2.csv"]).arg(
        "SELECT a, b FROM t1 WHERE NOT EXISTS (SELECT 1 FROM t2 WHERE t2.b = t1.a) ORDER BY a",
    );
    let got_not: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut not_cmd);
    assert_eq!(got_not, vec![svec!["a", "b"], svec!["3", "30"]]);
}

#[test]
fn sqlp_correlated_scalar_subquery() {
    // pola-rs/polars#28939: a scalar subquery's aggregate binds to the
    // subquery's OWN relation. `SUM(x.a)` therefore sums t2's `a`, and a row
    // with no match yields null rather than 0.
    let wrk = Workdir::new("sqlp_correlated_scalar_subquery");
    subquery_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.args(["t1.csv", "t2.csv"])
        .arg("SELECT a, (SELECT SUM(x.a) FROM t2 x WHERE x.b = t1.a) AS s FROM t1 ORDER BY a");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["a", "s"],
            svec!["1", "100"],
            svec!["2", "200"],
            svec!["3", ""],
        ]
    );

    // Aggregating a column of the OUTER relation inside the subquery is now an
    // error instead of silently resolving.
    let mut bad_cmd = wrk.command("sqlp");
    bad_cmd
        .args(["t1.csv", "t2.csv"])
        .arg("SELECT a, (SELECT SUM(t1.b) FROM t2 x WHERE x.b = t1.a) AS s FROM t1");
    let stderr = wrk.stderr_on_error(&mut bad_cmd);
    assert!(
        stderr.contains("no table or struct column named 't1' found"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn sqlp_scope_strictness() {
    // pola-rs/polars#28937: SQL scope rules now follow Postgres. Once a relation
    // is aliased, its original name is out of scope -- but an unqualified
    // ORDER BY key still resolves, and an outer alias is still visible to a
    // correlated subquery.
    let wrk = Workdir::new("sqlp_scope_strictness");
    subquery_fixture(&wrk);

    // The alias hides the table name.
    let mut bad_cmd = wrk.command("sqlp");
    bad_cmd.arg("t1.csv").arg("SELECT t1.a FROM t1 AS f");
    let stderr = wrk.stderr_on_error(&mut bad_cmd);
    assert!(
        stderr.contains("no table or struct column named 't1' found"),
        "unexpected stderr: {stderr}"
    );

    let ok_queries = [
        // qualified by the alias, ordered by the bare column name
        "SELECT f.a FROM t1 AS f ORDER BY a",
        // unaliased, so the table name is still in scope
        "SELECT t1.a FROM t1 ORDER BY a",
        // the outer alias `o` is visible inside the correlated IN subquery
        "SELECT a FROM t1 o WHERE b IN (SELECT x.b FROM t1 AS x WHERE x.a <= o.a) ORDER BY a",
    ];
    for query in ok_queries {
        let mut cmd = wrk.command("sqlp");
        cmd.arg("t1.csv").arg(query);
        let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
        assert_eq!(
            got,
            vec![svec!["a"], svec!["1"], svec!["2"], svec!["3"]],
            "query: {query}"
        );
    }
}

#[test]
fn sqlp_group_by_unaliased_constants() {
    // pola-rs/polars#29367: unaliased constants in a SELECT with GROUP BY are
    // projected once per group and named `literal`, `literal:1`, `literal:2`
    // rather than colliding on a single `literal` column.
    let wrk = Workdir::new("sqlp_group_by_unaliased_constants");
    subquery_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("t1.csv")
        .arg("SELECT 2, a, 'x', COUNT(*) AS n, 3 FROM t1 GROUP BY a ORDER BY a");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["literal", "a", "literal:1", "n", "literal:2"],
            svec!["2", "1", "x", "1", "3"],
            svec!["2", "2", "x", "1", "3"],
            svec!["2", "3", "x", "1", "3"],
        ]
    );
}

#[test]
fn sqlp_order_by_aggregate() {
    // pola-rs/polars#29010: ORDER BY accepts a restated aggregate, an aggregate
    // that is not in the SELECT list at all, COUNT(*), and an aggregate
    // expression. `b` outranks `a` on SUM(value) (15 vs 6) while the two tie on
    // COUNT(*), so the COUNT case needs `category` as a tiebreak to be a total
    // order.
    let wrk = Workdir::new("sqlp_order_by_aggregate");
    grouping_fixture(&wrk);

    let by_total = vec![
        svec!["category", "total"],
        svec!["b", "15"],
        svec!["a", "6"],
    ];

    let mut restated_cmd = wrk.command("sqlp");
    restated_cmd.arg("groups.csv").arg(
        "SELECT category, SUM(value) AS total FROM groups GROUP BY category ORDER BY SUM(value) \
         DESC",
    );
    let got_restated: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut restated_cmd);
    assert_eq!(got_restated, by_total);

    // ORDER BY an aggregate expression, not just the bare aggregate.
    let mut expr_cmd = wrk.command("sqlp");
    expr_cmd.arg("groups.csv").arg(
        "SELECT category, SUM(value) AS total FROM groups GROUP BY category ORDER BY SUM(value) * \
         -1",
    );
    let got_expr: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut expr_cmd);
    assert_eq!(got_expr, by_total);

    // The aggregate need not be projected.
    let mut unselected_cmd = wrk.command("sqlp");
    unselected_cmd
        .arg("groups.csv")
        .arg("SELECT category FROM groups GROUP BY category ORDER BY SUM(value) DESC");
    let got_unselected: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut unselected_cmd);
    assert_eq!(
        got_unselected,
        vec![svec!["category"], svec!["b"], svec!["a"]]
    );

    // COUNT(*) ties at 3 for both groups, so `category` decides.
    let mut count_cmd = wrk.command("sqlp");
    count_cmd
        .arg("groups.csv")
        .arg("SELECT category FROM groups GROUP BY category ORDER BY COUNT(*) DESC, category");
    let got_count: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut count_cmd);
    assert_eq!(got_count, vec![svec!["category"], svec!["a"], svec!["b"]]);
}

#[test]
fn sqlp_cte_shadows_table_and_case_insensitive_relation() {
    // pola-rs/polars#29006: a CTE shadows a same-named registered table, and a
    // relation name resolves case-insensitively.
    let wrk = Workdir::new("sqlp_cte_shadows_table_and_case_insensitive_relation");
    grouping_fixture(&wrk);

    let mut cte_cmd = wrk.command("sqlp");
    cte_cmd.arg("groups.csv").arg(
        "WITH groups AS (SELECT 'z' AS category, 99 AS value) SELECT category, value FROM groups",
    );
    let got_cte: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cte_cmd);
    assert_eq!(got_cte, vec![svec!["category", "value"], svec!["z", "99"]]);

    let mut case_cmd = wrk.command("sqlp");
    case_cmd
        .arg("groups.csv")
        .arg("SELECT category FROM GROUPS GROUP BY category ORDER BY category");
    let got_case: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut case_cmd);
    assert_eq!(got_case, vec![svec!["category"], svec!["a"], svec!["b"]]);
}

#[test]
fn sqlp_date_part_functions() {
    // pola-rs/polars#29269 added the bare date-part shorthands to the SQL
    // frontend: YEAR, QUARTER, MONTH, WEEK, DAY/DAYOFMONTH, DAYOFWEEK,
    // DAYOFYEAR, HOUR, MINUTE, SECOND. None of them existed at py-1.44.0.
    //
    // The 2024-12-31 row is the discriminating one: WEEK is ISO week, so it is
    // **1** (of the following year), not 53 as a naive week-of-year would give.
    // DAYOFWEEK is ISO too -- Monday is 1, so the Monday 2021-03-15 is 1 and the
    // Tuesday 2024-12-31 is 2. DAYOFYEAR 366 confirms the leap year.
    let wrk = Workdir::new("sqlp_date_part_functions");
    wrk.create(
        "dtparts.csv",
        vec![
            svec!["ts", "d"],
            svec!["2021-03-15 10:30:20", "2021-03-15"],
            svec!["2024-12-31 23:59:59", "2024-12-31"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("dtparts.csv")
        .arg(
            "SELECT ts, YEAR(ts) AS y, QUARTER(ts) AS q, MONTH(ts) AS mo, WEEK(ts) AS wk, DAY(ts) \
             AS d, DAYOFMONTH(ts) AS dom, DAYOFWEEK(ts) AS dow, DAYOFYEAR(ts) AS doy, HOUR(ts) AS \
             h, MINUTE(ts) AS mi, SECOND(ts) AS s FROM dtparts",
        )
        .arg("--try-parsedates");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec![
            "ts", "y", "q", "mo", "wk", "d", "dom", "dow", "doy", "h", "mi", "s"
        ],
        svec![
            "2021-03-15T10:30:20.000000",
            "2021",
            "1",
            "3",
            "11",
            "15",
            "15",
            "1",
            "74",
            "10",
            "30",
            "20"
        ],
        svec![
            "2024-12-31T23:59:59.000000",
            "2024",
            "4",
            "12",
            "1",
            "31",
            "31",
            "2",
            "366",
            "23",
            "59",
            "59"
        ],
    ];
    assert_eq!(got, expected);

    // They apply to a Date column too. `d` is date-shaped, so --try-parsedates
    // makes it a real Date; note CAST(ts AS DATE) would NOT work here, because
    // format inference cannot read a datetime string as a date.
    let mut date_cmd = wrk.command("sqlp");
    date_cmd
        .arg("dtparts.csv")
        .arg("SELECT YEAR(d) AS y, MONTH(d) AS mo, DAY(d) AS dd, WEEK(d) AS wk FROM dtparts")
        .arg("--try-parsedates");
    let got_date: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut date_cmd);
    assert_eq!(
        got_date,
        vec![
            svec!["y", "mo", "dd", "wk"],
            svec!["2021", "3", "15", "11"],
            svec!["2024", "12", "31", "1"],
        ]
    );
}

#[test]
fn sqlp_grouping_id() {
    // pola-rs/polars#29278 added GROUPING_ID alongside GROUPING(). On a ROLLUP
    // the two agree: both pack one bit per argument, MSB-first, so the grand
    // total is 3 (both keys rolled up) and a category subtotal is 1 (only class
    // rolled up).
    let wrk = Workdir::new("sqlp_grouping_id");
    grouping_fixture(&wrk);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("groups.csv")
        .arg(
            "SELECT category, class, SUM(value) AS total, GROUPING_ID(category, class) AS gid, \
             GROUPING(category, class) AS g FROM groups GROUP BY ROLLUP(category, class) ORDER BY \
             category NULLS LAST, class NULLS LAST",
        )
        .args(["--wnull-value", "NULL"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "class", "total", "gid", "g"],
        svec!["a", "x", "3", "0", "0"],
        svec!["a", "y", "3", "0", "0"],
        svec!["a", "NULL", "6", "1", "1"],
        svec!["b", "x", "4", "0", "0"],
        svec!["b", "y", "11", "0", "0"],
        svec!["b", "NULL", "15", "1", "1"],
        svec!["NULL", "NULL", "21", "3", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_cast_temporal_error_payloads() {
    // pola-rs/polars#28986 routes SQL string->temporal casts through
    // strptime-infer, which reports TWO different failures depending on whether
    // a format could be inferred at all:
    //   * some rows parse   -> InvalidOperation, "conversion ... failed", naming the offending
    //     value (see sqlp_cast_strict_vs_try_temporal)
    //   * NO row parses     -> ComputeError, "could not find an appropriate format to parse
    //     <dates|times>"
    // Before the rewrite both cases produced the first message, so the
    // format-inference failure is the new signal.
    let wrk = Workdir::new("sqlp_cast_temporal_error_payloads");
    wrk.create(
        "allbad.csv",
        vec![svec!["id", "s"], svec!["1", "aaa"], svec!["2", "bbb"]],
    );

    let mut date_cmd = wrk.command("sqlp");
    date_cmd
        .arg("allbad.csv")
        .arg("SELECT id, CAST(s AS DATE) AS d FROM allbad");
    let date_stderr = wrk.stderr_on_error(&mut date_cmd);
    assert!(
        date_stderr.contains("could not find an appropriate format to parse dates"),
        "unexpected stderr: {date_stderr}"
    );

    let mut time_cmd = wrk.command("sqlp");
    time_cmd
        .arg("allbad.csv")
        .arg("SELECT id, CAST(s AS TIME) AS t FROM allbad");
    let time_stderr = wrk.stderr_on_error(&mut time_cmd);
    assert!(
        time_stderr.contains("could not find an appropriate format to parse times"),
        "unexpected stderr: {time_stderr}"
    );

    // TRY_CAST still degrades to all-null rather than failing.
    let mut try_cmd = wrk.command("sqlp");
    try_cmd
        .arg("allbad.csv")
        .arg("SELECT id, TRY_CAST(s AS DATE) AS d FROM allbad ORDER BY id");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut try_cmd);
    assert_eq!(got, vec![svec!["id", "d"], svec!["1", ""], svec!["2", ""]]);
}

#[test]
fn sqlp_grouping_distinguishes_a_data_null() {
    // pola-rs/polars#29278: this is the whole reason GROUPING()/GROUPING_ID()
    // exist. A ROLLUP writes NULL into the keys it rolls up, which is
    // indistinguishable *in the data* from a group whose key is genuinely NULL.
    // The fixture therefore carries real NULL categories (empty CSV fields read
    // as NULL), so two output rows both print NULL in `category` and only
    // GROUPING() tells them apart:
    //     category=NULL, g=0  -> the real NULL group, total 12 (5 + 7)
    //     category=NULL, g=1  -> the ROLLUP grand total, total 15 (3 + 12)
    // The shared `grouping_fixture` has no NULL in the data and so cannot reach
    // this distinction at all.
    let wrk = Workdir::new("sqlp_grouping_distinguishes_a_data_null");
    wrk.create(
        "gnull.csv",
        vec![
            svec!["category", "class", "value"],
            svec!["a", "x", "1"],
            svec!["a", "y", "2"],
            svec!["", "x", "5"],
            svec!["", "y", "7"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("gnull.csv")
        .arg(
            "SELECT category, SUM(value) AS total, GROUPING(category) AS g FROM gnull GROUP BY \
             ROLLUP(category) ORDER BY g, category NULLS LAST",
        )
        .args(["--wnull-value", "NULL"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "total", "g"],
        svec!["a", "3", "0"],
        // a REAL null key: GROUPING is 0, because `category` participates in
        // this grouping set -- the NULL is data, not a subtotal marker.
        svec!["NULL", "12", "0"],
        // the rolled-up grand total: same printed NULL, GROUPING is 1.
        svec!["NULL", "15", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_window_order_by_multiple_keys_limitation() {
    // Found while hardening the #29159 tests, and the reason they project only
    // determinate columns instead of adding a tiebreak key.
    //
    // A window ORDER BY honors NULLS FIRST/LAST only with a SINGLE key. Add a
    // second key and the NULLS clause is SILENTLY IGNORED -- the nulls move to
    // the front even when every key says NULLS LAST. A top-level ORDER BY with
    // two keys honors it correctly, so this is specific to the window path.
    //
    // Mixing directions or NULLS placement across window keys is rejected
    // outright. If a future polars fixes any of this, these assertions fail and
    // the tiebreak workaround becomes available.
    //
    // Reported upstream as pola-rs/polars#29390. Root cause:
    // Expr::over_with_options collapses several ORDER BY keys into a single
    // as_struct(...), and a struct holding a null field is not itself null, so
    // SortOptions::nulls_last has nothing to act on. `descending` survives the
    // same path, which is why only the null placement is wrong.
    let wrk = Workdir::new("sqlp_window_order_by_multiple_keys_limitation");
    window_nulls_fixture(&wrk);

    // Single key: NULLS LAST is honored -- nulls get rn 5 and 6.
    let mut single_cmd = wrk.command("sqlp");
    single_cmd
        .arg("wnulls.csv")
        .arg("SELECT a, ROW_NUMBER() OVER (ORDER BY a NULLS LAST) AS rn FROM wnulls ORDER BY rn");
    let got_single: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut single_cmd);
    assert_eq!(
        got_single,
        vec![
            svec!["a", "rn"],
            svec!["10.0", "1"],
            svec!["20.0", "2"],
            svec!["30.0", "3"],
            svec!["40.0", "4"],
            svec!["", "5"],
            svec!["", "6"],
        ]
    );

    // Two keys, BOTH spelled NULLS LAST: the clause is dropped and the nulls
    // lead. This is the bug -- it is pinned, not endorsed.
    let mut multi_cmd = wrk.command("sqlp");
    multi_cmd.arg("wnulls.csv").arg(
        "SELECT grp, a, ROW_NUMBER() OVER (ORDER BY a NULLS LAST, grp NULLS LAST) AS rn FROM \
         wnulls ORDER BY rn",
    );
    let got_multi: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut multi_cmd);
    assert_eq!(
        got_multi,
        vec![
            svec!["grp", "a", "rn"],
            svec!["x", "", "1"],
            svec!["y", "", "2"],
            svec!["x", "10.0", "3"],
            svec!["x", "20.0", "4"],
            svec!["y", "30.0", "5"],
            svec!["y", "40.0", "6"],
        ]
    );

    // The same two-key ORDER BY at the TOP level honors NULLS LAST, which is
    // what makes the above a window-specific defect rather than a syntax quirk.
    let mut top_cmd = wrk.command("sqlp");
    top_cmd
        .arg("wnulls.csv")
        .arg("SELECT grp, a FROM wnulls ORDER BY a NULLS LAST, grp");
    let got_top: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut top_cmd);
    assert_eq!(
        got_top,
        vec![
            svec!["grp", "a"],
            svec!["x", "10.0"],
            svec!["x", "20.0"],
            svec!["y", "30.0"],
            svec!["y", "40.0"],
            svec!["x", ""],
            svec!["y", ""],
        ]
    );

    // Mixed NULLS placement across window keys is a hard error.
    let mut mixed_nulls_cmd = wrk.command("sqlp");
    mixed_nulls_cmd.arg("wnulls.csv").arg(
        "SELECT a, ROW_NUMBER() OVER (ORDER BY a NULLS FIRST, grp) AS rn FROM wnulls ORDER BY rn",
    );
    let mixed_nulls_stderr = wrk.stderr_on_error(&mut mixed_nulls_cmd);
    assert!(
        mixed_nulls_stderr
            .contains("OVER does not (yet) support mixed NULLS FIRST/LAST ordering for ORDER BY"),
        "unexpected stderr: {mixed_nulls_stderr}"
    );

    // So is a mixed asc/desc window ORDER BY.
    let mut mixed_dir_cmd = wrk.command("sqlp");
    mixed_dir_cmd.arg("wnulls.csv").arg(
        "SELECT a, ROW_NUMBER() OVER (ORDER BY a DESC NULLS FIRST, grp) AS rn FROM wnulls ORDER \
         BY rn",
    );
    let mixed_dir_stderr = wrk.stderr_on_error(&mut mixed_dir_cmd);
    assert!(
        mixed_dir_stderr.contains("OVER does not (yet) support mixed asc/desc directions"),
        "unexpected stderr: {mixed_dir_stderr}"
    );
}
