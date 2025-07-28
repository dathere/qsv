use crate::workdir::Workdir;

macro_rules! joinp_test {
    ($name:ident, $fun:expr_2021) => {
        mod $name {
            use std::process;

            #[allow(unused_imports)]
            use super::{make_rows, setup};
            use crate::workdir::Workdir;

            #[test]
            fn headers() {
                let wrk = setup(stringify!($name));
                let mut cmd = wrk.command("joinp");
                cmd.args(&["city", "cities.csv", "city", "places.csv"]);
                $fun(wrk, cmd);
            }
        }
    };
}

macro_rules! joinp_test_cache_schema {
    ($name:ident, $fun:expr_2021) => {
        mod $name {
            use std::process;

            #[allow(unused_imports)]
            use super::{make_rows, setup};
            use crate::workdir::Workdir;

            #[test]
            fn headers() {
                let wrk = setup(stringify!($name));
                let mut cmd = wrk.command("joinp");
                cmd.args(&[
                    "city",
                    "cities.csv",
                    "city",
                    "places.csv",
                    "--cache-schema",
                    "1",
                ]);
                $fun(wrk, cmd);
            }
        }
    };
}

macro_rules! joinp_test_tab {
    ($name0:ident, $fun:expr_2021) => {
        mod $name0 {
            use std::process;

            #[allow(unused_imports)]
            use super::{make_rows, setup};
            use crate::workdir::Workdir;

            #[test]
            fn headers() {
                let wrk = setup(stringify!($name0));
                let mut cmd = wrk.command("joinp");
                cmd.args(&["city", "cities.tsv", "city", "places.ssv"]);
                $fun(wrk, cmd);
            }
        }
    };
}

macro_rules! joinp_test_comments {
    ($name2:ident, $fun:expr_2021) => {
        mod $name2 {
            use std::process;

            #[allow(unused_imports)]
            use super::{make_rows, setup};
            use crate::workdir::Workdir;

            #[test]
            fn headers() {
                let wrk = setup(stringify!($name2));
                let mut cmd = wrk.command("joinp");
                cmd.env("QSV_COMMENT_CHAR", "#");
                cmd.args(&["city", "cities_comments.csv", "city", "places.ssv"]);
                $fun(wrk, cmd);
            }
        }
    };
}

macro_rules! joinp_test_compressed {
    ($name3:ident, $fun:expr_2021) => {
        mod $name3 {
            use std::process;

            #[allow(unused_imports)]
            use super::{make_rows, setup};
            use crate::workdir::Workdir;

            #[test]
            fn headers() {
                let wrk = setup(stringify!($name3));
                let mut cmd = wrk.command("joinp");
                cmd.args(&["city", "cities.csv.sz", "city", "places.ssv.sz"]);
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
    let cities_comments = vec![
        svec!["#this is a comment", ""],
        svec!["city", "state"],
        svec!["Boston", "MA"],
        svec!["New York", "NY"],
        svec!["#Washington", "DC"],
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
    wrk.create("cities.csv", cities.clone());
    wrk.create("cities_comments.csv", cities_comments);
    wrk.create("places.csv", places.clone());

    // create TSV/SSV versions
    wrk.create_with_delim("cities.tsv", cities, b'\t');
    wrk.create_with_delim("places.ssv", places.clone(), b';');

    // create snappy compressed versions
    let out_file = wrk.path("cities.csv.sz").to_string_lossy().to_string();
    let mut cmd = wrk.command("snappy");
    cmd.arg("compress")
        .arg("cities.csv")
        .args(["--output", &out_file]);
    wrk.assert_success(&mut cmd);
    drop(cmd);

    let out_file2 = wrk.path("places.csv.sz").to_string_lossy().to_string();
    let mut cmd = wrk.command("snappy");
    cmd.arg("compress")
        .arg("places.csv")
        .args(["--output", &out_file2]);
    wrk.assert_success(&mut cmd);

    let out_file3 = wrk.path("places.ssv.sz").to_string_lossy().to_string();
    let mut cmd = wrk.command("snappy");
    cmd.arg("compress")
        .arg("places.ssv")
        .args(["--output", &out_file3]);
    wrk.assert_success(&mut cmd);

    wrk
}

fn make_rows(left_only: bool, rows: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut all_rows = vec![];
    if left_only {
        all_rows.push(svec!["city", "state"]);
    } else {
        all_rows.push(svec!["city", "state", "place"]);
    }
    all_rows.extend(rows.into_iter());
    all_rows
}

joinp_test!(joinp_inner, |wrk: Workdir, mut cmd: process::Command| {
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = make_rows(
        false,
        vec![
            svec!["Boston", "MA", "Logan Airport"],
            svec!["Boston", "MA", "Boston Garden"],
            svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
        ],
    );
    assert_eq!(got, expected);
});

joinp_test_cache_schema!(
    joinp_inner_cache_schema,
    |wrk: Workdir, mut cmd: process::Command| {
        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
            ],
        );
        assert_eq!(got, expected);
        assert!(wrk.path("cities.pschema.json").exists());
        let cities_schema = std::fs::read_to_string(wrk.path("cities.pschema.json")).unwrap();
        assert_eq!(
            cities_schema,
            r#"{
  "fields": {
    "city": "String",
    "state": "String"
  }
}"#
        );
        assert!(wrk.path("places.pschema.json").exists());
        let places_schema = std::fs::read_to_string(wrk.path("places.pschema.json")).unwrap();
        assert_eq!(
            places_schema,
            r#"{
  "fields": {
    "city": "String",
    "place": "String"
  }
}"#
        );
    }
);

joinp_test_tab!(
    joinp_inner_tab,
    |wrk: Workdir, mut cmd: process::Command| {
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
            ],
        );
        assert_eq!(got, expected);
    }
);

joinp_test_comments!(
    joinp_inner_comments,
    |wrk: Workdir, mut cmd: process::Command| {
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
            ],
        );
        assert_eq!(got, expected);
    }
);

joinp_test_compressed!(
    joinp_inner_compressed,
    |wrk: Workdir, mut cmd: process::Command| {
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
            ],
        );
        assert_eq!(got, expected);
    }
);

joinp_test!(
    joinp_outer_left,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["New York", "NY", ""],
                svec!["San Francisco", "CA", ""],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
            ],
        );
        assert_eq!(got, expected);
    }
);

joinp_test_tab!(
    joinp_outer_left_tab,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["New York", "NY", ""],
                svec!["San Francisco", "CA", ""],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
            ],
        );
        assert_eq!(got, expected);
    }
);

joinp_test_comments!(
    joinp_outer_left_comments,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["New York", "NY", ""],
                svec!["San Francisco", "CA", ""],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
            ],
        );
        assert_eq!(got, expected);
    }
);

joinp_test!(
    joinp_outer_left_filter_left,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left").args(["--filter-left", "city = 'Boston'"]);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
            ],
        );
        assert_eq!(got, expected);
    }
);

joinp_test_tab!(
    joinp_outer_left_filter_left_tab,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left").args(["--filter-left", "city = 'Boston'"]);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
            ],
        );
        assert_eq!(got, expected);
    }
);

joinp_test_comments!(
    joinp_outer_left_filter_left_comments,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left").args(["--filter-left", "city = 'Boston'"]);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
            ],
        );
        assert_eq!(got, expected);
    }
);

joinp_test!(
    joinp_inner_filter_right,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(["--filter-right", "place ~* 'w'"]);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(false, vec![svec!["Buffalo", "NY", "Ralph Wilson Stadium"]]);
        assert_eq!(got, expected);
    }
);

joinp_test_comments!(
    joinp_inner_filter_right_comments,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(["--filter-right", "place ~* 'w'"]);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(false, vec![svec!["Buffalo", "NY", "Ralph Wilson Stadium"]]);
        assert_eq!(got, expected);
    }
);

joinp_test!(
    joinp_outer_left_validate_none,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left").args(["--validate", "none"]);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["New York", "NY", ""],
                svec!["San Francisco", "CA", ""],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
            ],
        );
        assert_eq!(got, expected);
    }
);

joinp_test!(
    joinp_outer_left_validate_none_streaming,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left")
            .args(["--validate", "none"])
            .arg("--streaming");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["New York", "NY", ""],
                svec!["San Francisco", "CA", ""],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
            ],
        );
        assert_eq!(got, expected);
    }
);

joinp_test!(
    joinp_outer_right_none_streaming,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--right")
            .args(["--validate", "none"])
            .arg("--streaming");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected: Vec<Vec<String>> = vec![
            svec!["state", "city", "place"],
            svec!["MA", "Boston", "Logan Airport"],
            svec!["MA", "Boston", "Boston Garden"],
            svec!["NY", "Buffalo", "Ralph Wilson Stadium"],
            svec!["", "Orlando", "Disney World"],
        ];
        assert_eq!(got, expected);
    }
);

joinp_test_comments!(
    joinp_outer_left_validate_none_comments,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left").args(["--validate", "none"]);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["New York", "NY", ""],
                svec!["San Francisco", "CA", ""],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
            ],
        );
        assert_eq!(got, expected);
    }
);

joinp_test!(
    joinp_outer_left_validate_manytoone,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left").args(["--validate", "manytoone"]);
        let got: String = wrk.output_stderr(&mut cmd);
        assert_eq!(
            got,
            "Polars error: ComputeError(ErrString(\"join keys did not fulfill m:1 validation\"))\n"
        );
        wrk.assert_err(&mut cmd);
    }
);

joinp_test!(
    joinp_outer_invalid_validation,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left").args(["--validate", "manytoeveryone"]);
        let got: String = wrk.output_stderr(&mut cmd);
        assert_eq!(
            got,
            "usage error: Invalid join validation: manytoeveryone\n"
        );
        wrk.assert_err(&mut cmd);
    }
);

joinp_test!(
    joinp_outer_left_validate_onetomany,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left").args(["--validate", "OneToMany"]);
        let got: String = wrk.output_stderr(&mut cmd);
        assert_eq!(got, "(5, 3)\n");
        wrk.assert_success(&mut cmd);
    }
);

joinp_test!(
    joinp_outer_left_validate_onetoone,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left").args(["--validate", "OneToone"]);
        let got: String = wrk.output_stderr(&mut cmd);
        assert_eq!(
            got,
            "Polars error: ComputeError(ErrString(\"join keys did not fulfill 1:1 validation\"))\n"
        );
        wrk.assert_err(&mut cmd);
    }
);

joinp_test!(joinp_full, |wrk: Workdir, mut cmd: process::Command| {
    cmd.arg("--full").arg("--coalesce");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected1 = make_rows(
        false,
        vec![
            svec!["Boston", "MA", "Logan Airport"],
            svec!["Boston", "MA", "Boston Garden"],
            svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
            svec!["Orlando", "", "Disney World"],
            svec!["San Francisco", "CA", ""],
            svec!["New York", "NY", ""],
        ],
    );
    let expected2 = make_rows(
        false,
        vec![
            svec!["Boston", "MA", "Logan Airport"],
            svec!["Boston", "MA", "Boston Garden"],
            svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
            svec!["Orlando", "", "Disney World"],
            svec!["New York", "NY", ""],
            svec!["San Francisco", "CA", ""],
        ],
    );
    assert!(got == expected1 || got == expected2);
});

joinp_test!(
    joinp_full_not_coalesced,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--full");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let mut expected1 = make_rows(
            false,
            vec![
                svec!["city", "state", "city_right", "place"],
                svec!["Boston", "MA", "Boston", "Logan Airport"],
                svec!["Boston", "MA", "Boston", "Boston Garden"],
                svec!["Buffalo", "NY", "Buffalo", "Ralph Wilson Stadium"],
                svec!["", "", "Orlando", "Disney World"],
                svec!["San Francisco", "CA", "", ""],
                svec!["New York", "NY", "", ""],
            ],
        );
        // remove the first old header from expected1
        expected1.remove(0);

        let mut expected2 = make_rows(
            false,
            vec![
                svec!["city", "state", "city_right", "place"],
                svec!["Boston", "MA", "Boston", "Logan Airport"],
                svec!["Boston", "MA", "Boston", "Boston Garden"],
                svec!["Buffalo", "NY", "Buffalo", "Ralph Wilson Stadium"],
                svec!["", "", "Orlando", "Disney World"],
                svec!["New York", "NY", "", ""],
                svec!["San Francisco", "CA", "", ""],
            ],
        );
        expected2.remove(0);

        assert!(got == expected1 || got == expected2);
    }
);

joinp_test_compressed!(
    joinp_full_compressed,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--full").arg("--coalesce");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected1 = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
                svec!["Orlando", "", "Disney World"],
                svec!["San Francisco", "CA", ""],
                svec!["New York", "NY", ""],
            ],
        );
        let expected2 = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
                svec!["Orlando", "", "Disney World"],
                svec!["New York", "NY", ""],
                svec!["San Francisco", "CA", ""],
            ],
        );
        assert!(got == expected1 || got == expected2);
    }
);

joinp_test_comments!(
    joinp_full_comments,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--full").arg("--coalesce");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected1 = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
                svec!["Orlando", "", "Disney World"],
                svec!["San Francisco", "CA", ""],
                svec!["New York", "NY", ""],
            ],
        );
        let expected2 = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
                svec!["Orlando", "", "Disney World"],
                svec!["New York", "NY", ""],
                svec!["San Francisco", "CA", ""],
            ],
        );
        assert!(got == expected1 || got == expected2);
    }
);

joinp_test!(
    joinp_left_semi,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left-semi");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(true, vec![svec!["Boston", "MA"], svec!["Buffalo", "NY"]]);
        assert_eq!(got, expected);
    }
);

joinp_test_tab!(
    joinp_left_semi_tab,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left-semi");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(true, vec![svec!["Boston", "MA"], svec!["Buffalo", "NY"]]);
        assert_eq!(got, expected);
    }
);

joinp_test_comments!(
    joinp_left_semi_comments,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left-semi");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(true, vec![svec!["Boston", "MA"], svec!["Buffalo", "NY"]]);
        assert_eq!(got, expected);
    }
);

joinp_test!(
    joinp_left_anti,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left-anti");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            true,
            vec![svec!["New York", "NY"], svec!["San Francisco", "CA"]],
        );
        assert_eq!(got, expected);
    }
);

joinp_test_comments!(
    joinp_left_anti_comments,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left-anti");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            true,
            vec![svec!["New York", "NY"], svec!["San Francisco", "CA"]],
        );
        assert_eq!(got, expected);
    }
);

#[test]
fn joinp_cross() {
    let wrk = Workdir::new("join_cross");
    wrk.create(
        "letters.csv",
        vec![svec!["h1", "h2"], svec!["a", "b"], svec!["c", "d"]],
    );
    wrk.create(
        "numbers.csv",
        vec![svec!["h3", "h4"], svec!["1", "2"], svec!["3", "4"]],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--cross").args(["letters.csv", "numbers.csv"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2", "h3", "h4"],
        svec!["a", "b", "1", "2"],
        svec!["a", "b", "3", "4"],
        svec!["c", "d", "1", "2"],
        svec!["c", "d", "3", "4"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_cross_compress() {
    let wrk = Workdir::new("join_cross_compress");
    wrk.create(
        "letters.csv",
        vec![svec!["h1", "h2"], svec!["a", "b"], svec!["c", "d"]],
    );
    wrk.create(
        "numbers.csv",
        vec![svec!["h3", "h4"], svec!["1", "2"], svec!["3", "4"]],
    );

    let out_file = wrk.path("out.csv.sz").to_string_lossy().to_string();

    let mut cmd = wrk.command("joinp");
    cmd.arg("--cross")
        .args(["letters.csv", "numbers.csv"])
        .args(["--output", &out_file]);

    wrk.assert_success(&mut cmd);

    let mut cmd2 = wrk.command("snappy"); // DevSkim: ignore DS126858
    cmd2.arg("decompress").arg(&out_file); // DevSkim: ignore DS126858

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2); // DevSkim: ignore DS126858
    let expected = vec![
        svec!["h1", "h2", "h3", "h4"],
        svec!["a", "b", "1", "2"],
        svec!["a", "b", "3", "4"],
        svec!["c", "d", "1", "2"],
        svec!["c", "d", "3", "4"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asof_date() {
    let wrk = Workdir::new("join_asof_date");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["date", "gdp"],
            svec!["2016-01-01", "4164"],
            svec!["2017-01-01", "4411"],
            svec!["2018-01-01", "4566"],
            svec!["2019-01-01", "4696"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["date", "population"],
            svec!["2016-05-12", "82.19"],
            svec!["2017-05-12", "82.66"],
            svec!["2018-05-12", "83.12"],
            svec!["2019-05-12", "83.52"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["date", "population.csv", "date", "gdp.csv"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date", "population", "gdp"],
        svec!["2016-05-12", "82.19", "4164"],
        svec!["2017-05-12", "82.66", "4411"],
        svec!["2018-05-12", "83.12", "4566"],
        svec!["2019-05-12", "83.52", "4696"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asof_date_compress() {
    let wrk = Workdir::new("join_asof_date_compress");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["date", "gdp"],
            svec!["2016-01-01", "4164"],
            svec!["2017-01-01", "4411"],
            svec!["2018-01-01", "4566"],
            svec!["2019-01-01", "4696"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["date", "population"],
            svec!["2016-05-12", "82.19"],
            svec!["2017-05-12", "82.66"],
            svec!["2018-05-12", "83.12"],
            svec!["2019-05-12", "83.52"],
        ],
    );

    let out_file = wrk.path("out.csv.sz").to_string_lossy().to_string();

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["date", "population.csv", "date", "gdp.csv"])
        .args(["--output", &out_file]);

    wrk.assert_success(&mut cmd);

    let mut cmd2 = wrk.command("snappy"); // DevSkim: ignore DS126858
    cmd2.arg("decompress").arg(&out_file); // DevSkim: ignore DS126858

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2); // DevSkim: ignore DS126858
    let expected = vec![
        svec!["date", "population", "gdp"],
        svec!["2016-05-12", "82.19", "4164"],
        svec!["2017-05-12", "82.66", "4411"],
        svec!["2018-05-12", "83.12", "4566"],
        svec!["2019-05-12", "83.52", "4696"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asof_date_comments() {
    let wrk = Workdir::new("join_asof_date_comments");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["#comment", "here"],
            svec!["date", "gdp"],
            svec!["2016-01-01", "4164"],
            svec!["2017-01-01", "4411"],
            svec!["2018-01-01", "4566"],
            svec!["2019-01-01", "4696"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["date", "population"],
            svec!["2016-05-12", "82.19"],
            svec!["2017-05-12", "82.66"],
            svec!["#comment", "in the middle"],
            svec!["2018-05-12", "83.12"],
            svec!["2019-05-12", "83.52"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.env("QSV_COMMENT_CHAR", "#");
    cmd.arg("--asof")
        .args(["date", "population.csv", "date", "gdp.csv"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date", "population", "gdp"],
        svec!["2016-05-12", "82.19", "4164"],
        svec!["2017-05-12", "82.66", "4411"],
        svec!["2018-05-12", "83.12", "4566"],
        svec!["2019-05-12", "83.52", "4696"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asofby_1() {
    let wrk = Workdir::new("join_asofby_timeseries");
    wrk.create(
        "trades.csv",
        vec![
            svec!["time", "ticker", "groups_numeric", "bid"],
            svec!["2016-01-01 12:23:00", "MSFT", "1", "51.95"],
            svec!["2016-01-01 12:38:00", "MSFT", "1", "51.95"],
            svec!["2016-01-01 12:48:00", "GOOG", "2", "720.77"],
            svec!["2016-01-01 12:48:00", "GOOG", "2", "720.92"],
            svec!["2016-01-01 12:48:00", "AAPL", "3", "98.0"],
        ],
    );
    wrk.create(
        "quotes.csv",
        vec![
            svec!["time", "ticker", "groups_numeric", "bid"],
            svec!["2016-01-01 12:23:00", "GOOG", "2", "720.50"],
            svec!["2016-01-01 12:23:00", "MSFT", "1", "51.95"],
            svec!["2016-01-01 12:30:00", "MSFT", "1", "51.97"],
            svec!["2016-01-01 12:41:00", "MSFT", "1", "51.99"],
            svec!["2016-01-01 12:48:00", "GOOG", "2", "720.50"],
            svec!["2016-01-01 12:49:00", "AAPL", "3", "97.99"],
            svec!["2016-01-01 12:52:00", "GOOG", "2", "720.50"],
            svec!["2016-01-01 12:55:00", "MSFT", "1", "52.01"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["time", "trades.csv", "time", "quotes.csv"])
        .args(["--left_by", "ticker"])
        .args(["--right_by", "ticker"])
        .args(["--datetime-format", "%Y-%m-%d %H:%M"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "time",
            "ticker",
            "groups_numeric",
            "bid",
            "ticker_right",
            "groups_numeric_right",
            "bid_right"
        ],
        svec!["2016-01-01 12:23", "MSFT", "1", "51.95", "", "", ""],
        svec![
            "2016-01-01 12:38",
            "MSFT",
            "1",
            "51.95",
            "MSFT",
            "1",
            "51.97"
        ],
        svec![
            "2016-01-01 12:48",
            "GOOG",
            "2",
            "720.77",
            "MSFT",
            "1",
            "51.99"
        ],
        svec![
            "2016-01-01 12:48",
            "GOOG",
            "2",
            "720.92",
            "MSFT",
            "1",
            "51.99"
        ],
        svec![
            "2016-01-01 12:48",
            "AAPL",
            "3",
            "98.0",
            "MSFT",
            "1",
            "51.99"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asofby_left_place_date() {
    let wrk = Workdir::new("join_asofby_left_place_date");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["date", "gdp", "place"],
            svec!["2016-01-01", "4164", "US"],
            svec!["2017-01-01", "4411", "US"],
            svec!["2018-01-01", "4566", "Asia"],
            svec!["2019-01-01", "4696", "EU"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["date", "population", "place"],
            svec!["2016-05-12", "82.19", "US"],
            svec!["2017-05-12", "82.66", "US"],
            svec!["2018-05-12", "83.12", "Asia"],
            svec!["2018-05-12", "84.12", "Asia"],
            svec!["2019-05-12", "83.52", "EU"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["date", "population.csv", "date", "gdp.csv"])
        .args(["--left_by", "place"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date", "population", "place", "gdp", "place_right"],
        svec!["2016-05-12", "82.19", "US", "4164", "US"],
        svec!["2017-05-12", "82.66", "US", "4411", "US"],
        svec!["2018-05-12", "83.12", "Asia", "4566", "Asia"],
        svec!["2018-05-12", "84.12", "Asia", "4566", "Asia"],
        svec!["2019-05-12", "83.52", "EU", "4696", "EU"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asofby_right_place_date() {
    let wrk = Workdir::new("join_asofby_right_place_date");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["date", "gdp", "place"],
            svec!["2016-01-01", "4164", "US"],
            svec!["2017-01-01", "4411", "US"],
            svec!["2018-01-01", "4566", "Asia"],
            svec!["2019-01-01", "4696", "EU"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["date", "population", "place"],
            svec!["2016-05-12", "82.19", "US"],
            svec!["2017-05-12", "82.66", "US"],
            svec!["2018-05-12", "83.12", "Asia"],
            svec!["2018-05-12", "84.12", "Asia"],
            svec!["2019-05-12", "83.52", "EU"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["date", "population.csv", "date", "gdp.csv"])
        .args(["--right_by", "place"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date", "population", "place", "gdp", "place_right"],
        svec!["2016-05-12", "82.19", "US", "4164", "US"],
        svec!["2017-05-12", "82.66", "US", "4411", "US"],
        svec!["2018-05-12", "83.12", "Asia", "4566", "Asia"],
        svec!["2018-05-12", "84.12", "Asia", "4566", "Asia"],
        svec!["2019-05-12", "83.52", "EU", "4696", "EU"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asofby_leftright_place_date() {
    let wrk = Workdir::new("join_asofby_leftright_place_date");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["date", "gdp", "place"],
            svec!["2016-01-01", "4164", "US"],
            svec!["2017-01-01", "4411", "US"],
            svec!["2018-01-01", "4566", "Asia"],
            svec!["2019-01-01", "4696", "EU"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["date", "population", "other_place"],
            svec!["2016-05-12", "82.19", "US"],
            svec!["2017-05-12", "82.66", "US"],
            svec!["2018-05-12", "83.12", "Asia"],
            svec!["2018-05-12", "84.12", "Asia"],
            svec!["2019-05-12", "83.52", "EU"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["date", "population.csv", "date", "gdp.csv"])
        .args(["--left_by", "place"])
        .args(["--right_by", "other_place"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date", "population", "other_place", "gdp", "place"],
        svec!["2016-05-12", "82.19", "US", "4164", "US"],
        svec!["2017-05-12", "82.66", "US", "4411", "US"],
        svec!["2018-05-12", "83.12", "Asia", "4566", "Asia"],
        svec!["2018-05-12", "84.12", "Asia", "4566", "Asia"],
        svec!["2019-05-12", "83.52", "EU", "4696", "EU"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asof_nearest_date() {
    let wrk = Workdir::new("join_asof_nearest_date");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["date", "gdp"],
            svec!["2016-01-01", "4164"],
            svec!["2017-01-22", "4422"],
            svec!["2017-01-10", "4410"],
            svec!["2018-01-01", "4501"],
            svec!["2018-01-05", "4505"],
            svec!["2018-01-14", "4514"],
            svec!["2019-01-01", "4696"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["date", "population"],
            svec!["2016-05-12", "82.19"],
            svec!["2017-05-12", "82.66"],
            svec!["2018-05-12", "83.12"],
            svec!["2019-05-12", "83.52"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof").args(["--strategy", "nearest"]).args([
        "date",
        "population.csv",
        "date",
        "gdp.csv",
    ]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date", "population", "gdp"],
        svec!["2016-05-12", "82.19", "4164"],
        svec!["2017-05-12", "82.66", "4422"],
        svec!["2018-05-12", "83.12", "4514"],
        svec!["2019-05-12", "83.52", "4696"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asof_date_diffcolnames() {
    let wrk = Workdir::new("join_asof_date_diffcolnames");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["gdp_date", "gdp"],
            svec!["2016-01-01", "4164"],
            svec!["2017-01-01", "4411"],
            svec!["2018-01-01", "4566"],
            svec!["2019-01-01", "4696"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["pop_date", "population"],
            svec!["2016-05-12", "82.19"],
            svec!["2017-05-12", "82.66"],
            svec!["2018-05-12", "83.12"],
            svec!["2019-05-12", "83.52"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["pop_date", "population.csv", "gdp_date", "gdp.csv"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["pop_date", "population", "gdp_date", "gdp"],
        svec!["2016-05-12", "82.19", "2016-01-01", "4164"],
        svec!["2017-05-12", "82.66", "2017-01-01", "4411"],
        svec!["2018-05-12", "83.12", "2018-01-01", "4566"],
        svec!["2019-05-12", "83.52", "2019-01-01", "4696"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asof_date_diffcolnames_sqlfilter() {
    let wrk = Workdir::new("join_asof_date_diffcolnames_sqlfilter");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["gdp_date", "gdp"],
            svec!["2016-01-01", "4164"],
            svec!["2017-01-01", "4411"],
            svec!["2018-01-01", "4566"],
            svec!["2019-01-01", "4696"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["pop_date", "population"],
            svec!["2016-05-12", "82.19"],
            svec!["2017-05-12", "82.66"],
            svec!["2018-05-12", "83.12"],
            svec!["2019-05-12", "83.52"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["pop_date", "population.csv", "gdp_date", "gdp.csv"])
        .args([
            "--sql-filter",
            "select pop_date, gdp from join_result where gdp > 4500",
        ]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["pop_date", "gdp"],
        svec!["2018-05-12", "4566"],
        svec!["2019-05-12", "4696"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_ignore_case() {
    let wrk = Workdir::new("joinp_ignore_case");

    // Create test data with mixed case cities
    wrk.create(
        "cities_mixed.csv",
        vec![
            svec!["city", "state"],
            svec!["BOSTON", "MA"],
            svec!["new york", "NY"],
            svec!["San Francisco", "CA"],
            svec!["BUFFALO", "NY"],
        ],
    );

    wrk.create(
        "places_mixed.csv",
        vec![
            svec!["city", "place"],
            svec!["Boston", "Logan Airport"],
            svec!["boston", "Boston Garden"],
            svec!["BUFFALO", "Ralph Wilson Stadium"],
            svec!["orlando", "Disney World"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.args(&["city", "cities_mixed.csv", "city", "places_mixed.csv"])
        .arg("--ignore-case");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["city", "state", "city_right", "place"],
        svec!["BOSTON", "MA", "Boston", "Logan Airport"],
        svec!["BOSTON", "MA", "boston", "Boston Garden"],
        svec!["BUFFALO", "NY", "BUFFALO", "Ralph Wilson Stadium"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_ignore_case_maintain_order_right() {
    let wrk = Workdir::new("joinp_mo_r");

    // Create test data with mixed case cities
    wrk.create(
        "cities_mixed.csv",
        vec![
            svec!["city", "state"],
            svec!["BOSTON", "MA"],
            svec!["new york", "NY"],
            svec!["San Francisco", "CA"],
            svec!["BUFFALO", "NY"],
        ],
    );

    wrk.create(
        "places_mixed.csv",
        vec![
            svec!["city", "place"],
            svec!["Boston", "Logan Airport"],
            svec!["boston", "Boston Garden"],
            svec!["BUFFALO", "Ralph Wilson Stadium"],
            svec!["orlando", "Disney World"],
            svec!["new York", "Madison Square Garden"],
            svec!["san francisco", "Fisherman's Wharf"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.args(&["city", "cities_mixed.csv", "city", "places_mixed.csv"])
        .arg("--ignore-case")
        .args(["--maintain-order", "right"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["city", "state", "city_right", "place"],
        svec!["BOSTON", "MA", "Boston", "Logan Airport"],
        svec!["BOSTON", "MA", "boston", "Boston Garden"],
        svec!["BUFFALO", "NY", "BUFFALO", "Ralph Wilson Stadium"],
        svec!["new york", "NY", "new York", "Madison Square Garden"],
        svec!["San Francisco", "CA", "san francisco", "Fisherman's Wharf"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_ignore_case_maintain_order_left() {
    let wrk = Workdir::new("joinp_mo_l");

    // Create test data with mixed case cities
    wrk.create(
        "cities_mixed.csv",
        vec![
            svec!["city", "state"],
            svec!["BOSTON", "MA"],
            svec!["new york", "NY"],
            svec!["San Francisco", "CA"],
            svec!["BUFFALO", "NY"],
        ],
    );

    wrk.create(
        "places_mixed.csv",
        vec![
            svec!["city", "place"],
            svec!["Boston", "Logan Airport"],
            svec!["boston", "Boston Garden"],
            svec!["BUFFALO", "Ralph Wilson Stadium"],
            svec!["orlando", "Disney World"],
            svec!["new York", "Madison Square Garden"],
            svec!["san francisco", "Fisherman's Wharf"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.args(&["city", "cities_mixed.csv", "city", "places_mixed.csv"])
        .arg("--ignore-case")
        .args(["--maintain-order", "left"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["city", "state", "city_right", "place"],
        svec!["BOSTON", "MA", "Boston", "Logan Airport"],
        svec!["BOSTON", "MA", "boston", "Boston Garden"],
        svec!["new york", "NY", "new York", "Madison Square Garden"],
        svec!["San Francisco", "CA", "san francisco", "Fisherman's Wharf"],
        svec!["BUFFALO", "NY", "BUFFALO", "Ralph Wilson Stadium"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_ignore_case_maintain_order_left_right() {
    let wrk = Workdir::new("joinp_mo_lr");

    // Create test data with mixed case cities
    wrk.create(
        "cities_mixed.csv",
        vec![
            svec!["city", "state"],
            svec!["BOSTON", "MA"],
            svec!["new york", "NY"],
            svec!["San Francisco", "CA"],
            svec!["BUFFALO", "NY"],
        ],
    );

    wrk.create(
        "places_mixed.csv",
        vec![
            svec!["city", "place"],
            svec!["Boston", "Logan Airport"],
            svec!["boston", "Boston Garden"],
            svec!["BUFFALO", "Ralph Wilson Stadium"],
            svec!["orlando", "Disney World"],
            svec!["new York", "Madison Square Garden"],
            svec!["san francisco", "Fisherman's Wharf"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.args(&["city", "cities_mixed.csv", "city", "places_mixed.csv"])
        .arg("--ignore-case")
        .args(["--maintain-order", "left_right"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["city", "state", "city_right", "place"],
        svec!["BOSTON", "MA", "Boston", "Logan Airport"],
        svec!["BOSTON", "MA", "boston", "Boston Garden"],
        svec!["new york", "NY", "new York", "Madison Square Garden"],
        svec!["San Francisco", "CA", "san francisco", "Fisherman's Wharf"],
        svec!["BUFFALO", "NY", "BUFFALO", "Ralph Wilson Stadium"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_ignore_case_maintain_order_right_left() {
    let wrk = Workdir::new("joinp_mo_rl");

    // Create test data with mixed case cities
    wrk.create(
        "cities_mixed.csv",
        vec![
            svec!["city", "state"],
            svec!["BOSTON", "MA"],
            svec!["new york", "NY"],
            svec!["San Francisco", "CA"],
            svec!["BUFFALO", "NY"],
        ],
    );

    wrk.create(
        "places_mixed.csv",
        vec![
            svec!["city", "place"],
            svec!["Boston", "Logan Airport"],
            svec!["boston", "Boston Garden"],
            svec!["BUFFALO", "Ralph Wilson Stadium"],
            svec!["orlando", "Disney World"],
            svec!["new York", "Madison Square Garden"],
            svec!["san francisco", "Fisherman's Wharf"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.args(&["city", "cities_mixed.csv", "city", "places_mixed.csv"])
        .arg("--ignore-case")
        .args(["--maintain-order", "right_left"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["city", "state", "city_right", "place"],
        svec!["BOSTON", "MA", "Boston", "Logan Airport"],
        svec!["BOSTON", "MA", "boston", "Boston Garden"],
        svec!["BUFFALO", "NY", "BUFFALO", "Ralph Wilson Stadium"],
        svec!["new york", "NY", "new York", "Madison Square Garden"],
        svec!["San Francisco", "CA", "san francisco", "Fisherman's Wharf"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_filter_pattern_matching() {
    let wrk = Workdir::new("joinp_filter_pattern_matching");

    // Create test data with pattern matching scenarios
    wrk.create(
        "prefixes.csv",
        vec![
            svec!["code", "description"],
            svec!["ABC", "Alpha Beta Charlie"],
            svec!["XYZ", "X-ray Yankee Zulu"],
            svec!["123", "One Two Three"],
        ],
    );

    wrk.create(
        "values.csv",
        vec![
            svec!["id", "value"],
            svec!["ABC123", "First"],
            svec!["ABCDEF", "Second"],
            svec!["XYZ789", "Third"],
            svec!["123456", "Fourth"],
            svec!["TEST123", "Fifth"],
            svec!["DEF123", "Sixth"],
        ],
    );

    // Test 1: Right starts-with left
    let mut cmd = wrk.command("joinp");
    cmd.args(&["code", "prefixes.csv", "id", "values.csv"])
        .arg("--cross")
        .args([
            "--sql-filter",
            "select * from join_result where STARTS_WITH(id, code)",
        ]);

    wrk.assert_success(&mut *&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["code", "description", "id", "value"],
        svec!["ABC", "Alpha Beta Charlie", "ABC123", "First"],
        svec!["ABC", "Alpha Beta Charlie", "ABCDEF", "Second"],
        svec!["XYZ", "X-ray Yankee Zulu", "XYZ789", "Third"],
        svec!["123", "One Two Three", "123456", "Fourth"],
    ];
    assert_eq!(got, expected);

    // Test 2: Right contains left
    let mut cmd = wrk.command("joinp");
    cmd.args(&["code", "prefixes.csv", "id", "values.csv"])
        .arg("--cross")
        .args([
            "--sql-filter",
            "select * from join_result where STRPOS(id, code) > 0",
        ]);

    wrk.assert_success(&mut *&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["code", "description", "id", "value"],
        svec!["ABC", "Alpha Beta Charlie", "ABC123", "First"],
        svec!["ABC", "Alpha Beta Charlie", "ABCDEF", "Second"],
        svec!["XYZ", "X-ray Yankee Zulu", "XYZ789", "Third"],
        svec!["123", "One Two Three", "ABC123", "First"],
        svec!["123", "One Two Three", "123456", "Fourth"],
        svec!["123", "One Two Three", "TEST123", "Fifth"],
        svec!["123", "One Two Three", "DEF123", "Sixth"],
    ];
    assert_eq!(got, expected);

    // Test 3: Right ends-with left
    let mut cmd = wrk.command("joinp");
    cmd.args(&["code", "prefixes.csv", "id", "values.csv"])
        .arg("--cross")
        .args([
            "--sql-filter",
            "select * from join_result where ENDS_WITH(id, code)",
        ]);

    wrk.assert_success(&mut *&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["code", "description", "id", "value"],
        svec!["123", "One Two Three", "ABC123", "First"],
        svec!["123", "One Two Three", "TEST123", "Fifth"],
        svec!["123", "One Two Three", "DEF123", "Sixth"],
    ];
    assert_eq!(got, expected);

    // Create reversed test data for left-side pattern matching
    wrk.create(
        "full_codes.csv",
        vec![
            svec!["code", "description"],
            svec!["ABC123", "Full Code 1"],
            svec!["ABCDEF", "Full Code 2"],
            svec!["XYZ789", "Full Code 3"],
        ],
    );

    wrk.create(
        "patterns.csv",
        vec![
            svec!["pattern", "meaning"],
            svec!["ABC", "Alpha Beta Charlie"],
            svec!["123", "One Two Three"],
            svec!["XYZ", "X-ray Yankee Zulu"],
        ],
    );

    // Test 4: Left starts-with right
    let mut cmd = wrk.command("joinp");
    cmd.args(&["code", "full_codes.csv", "pattern", "patterns.csv"])
        .arg("--cross")
        .args([
            "--sql-filter",
            "select * from join_result where STARTS_WITH(code, pattern)",
        ]);

    wrk.assert_success(&mut *&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["code", "description", "pattern", "meaning"],
        svec!["ABC123", "Full Code 1", "ABC", "Alpha Beta Charlie"],
        svec!["ABCDEF", "Full Code 2", "ABC", "Alpha Beta Charlie"],
        svec!["XYZ789", "Full Code 3", "XYZ", "X-ray Yankee Zulu"],
    ];
    assert_eq!(got, expected);

    // Test 5: Left contains right
    let mut cmd = wrk.command("joinp");
    cmd.args(&["code", "full_codes.csv", "pattern", "patterns.csv"])
        .arg("--cross")
        .args([
            "--sql-filter",
            "select * from join_result where STRPOS(code, pattern) > 0",
        ]);

    wrk.assert_success(&mut *&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["code", "description", "pattern", "meaning"],
        svec!["ABC123", "Full Code 1", "ABC", "Alpha Beta Charlie"],
        svec!["ABC123", "Full Code 1", "123", "One Two Three"],
        svec!["ABCDEF", "Full Code 2", "ABC", "Alpha Beta Charlie"],
        svec!["XYZ789", "Full Code 3", "XYZ", "X-ray Yankee Zulu"],
    ];
    assert_eq!(got, expected);

    // Test 6: Left ends-with right
    let mut cmd = wrk.command("joinp");
    cmd.args(&["code", "full_codes.csv", "pattern", "patterns.csv"])
        .arg("--cross")
        .args([
            "--sql-filter",
            "select * from join_result where ENDS_WITH(code, pattern)",
        ]);

    wrk.assert_success(&mut *&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["code", "description", "pattern", "meaning"],
        svec!["ABC123", "Full Code 1", "123", "One Two Three"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_joinp_cache_schema() {
    let wrk = Workdir::new("joinp_cache_schema");

    // Create test files based on issue #2369
    wrk.create(
        "left.csv",
        vec![
            svec!["id", "has_text", "col3", "col4", "col5"],
            svec!["1", "1", "a", "b", "c"],
            svec!["2", "0", "d", "e", "f"],
            svec!["3", "1", "g", "h", "i"],
            svec!["4", "0", "j", "k", "l"],
            svec!["5", "1", "m", "n", "o"],
        ],
    );

    wrk.create(
        "right.csv",
        vec![
            svec!["id", "has_text"],
            svec!["1", "1"],
            svec!["2", "0"],
            svec!["4", "0"],
        ],
    );

    // Test 1: No schema caching (default)
    let mut cmd = wrk.command("joinp");
    cmd.args(&["has_text", "left.csv", "has_text", "right.csv"]);
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "has_text", "col3", "col4", "col5", "id_right"],
        svec!["1", "1", "a", "b", "c", "1"],
        svec!["2", "0", "d", "e", "f", "2"],
        svec!["2", "0", "d", "e", "f", "4"],
        svec!["3", "1", "g", "h", "i", "1"],
        svec!["4", "0", "j", "k", "l", "2"],
        svec!["4", "0", "j", "k", "l", "4"],
        svec!["5", "1", "m", "n", "o", "1"],
    ];
    assert_eq!(got, expected);

    // Test 2: Cache inferred schema
    let mut cmd = wrk.command("joinp");
    cmd.args(&["has_text", "left.csv", "has_text", "right.csv"])
        .arg("--cache-schema")
        .arg("1");

    // success is expected as has_text is no longer interpreted as bool
    // confirms bug reported in https://github.com/dathere/qsv/issues/2369 no longer exists
    wrk.assert_success(&mut cmd);

    // Verify schema files were created
    assert!(wrk.path("left.pschema.json").exists());
    assert!(wrk.path("right.pschema.json").exists());

    // Test 3: Use string schema for all columns
    let mut cmd = wrk.command("joinp");
    cmd.args(&["has_text", "left.csv", "has_text", "right.csv"])
        .arg("--cache-schema")
        .arg("-1");
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, expected);

    // Test 4: Use and cache string schema
    let mut cmd = wrk.command("joinp");
    cmd.args(&["has_text", "left.csv", "has_text", "right.csv"])
        .arg("--cache-schema")
        .arg("-2");
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, expected);

    // Test 5: Invalid cache-schema value
    let mut cmd = wrk.command("joinp");
    cmd.args(&["has_text", "left.csv", "has_text", "right.csv"])
        .arg("--cache-schema")
        .arg("2");
    wrk.assert_err(&mut cmd);
}

joinp_test!(
    joinp_right_semi,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--right-semi");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["city", "place"],
            svec!["Boston", "Logan Airport"],
            svec!["Boston", "Boston Garden"],
            svec!["Buffalo", "Ralph Wilson Stadium"],
        ];
        assert_eq!(got, expected);
    }
);

joinp_test_tab!(
    joinp_right_semi_tab,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--right-semi");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["city", "place"],
            svec!["Boston", "Logan Airport"],
            svec!["Boston", "Boston Garden"],
            svec!["Buffalo", "Ralph Wilson Stadium"],
        ];
        assert_eq!(got, expected);
    }
);

joinp_test_comments!(
    joinp_right_semi_comments,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--right-semi");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["city", "place"],
            svec!["Boston", "Logan Airport"],
            svec!["Boston", "Boston Garden"],
            svec!["Buffalo", "Ralph Wilson Stadium"],
        ];
        assert_eq!(got, expected);
    }
);

joinp_test!(
    joinp_right_anti,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--right-anti");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![svec!["city", "place"], svec!["Orlando", "Disney World"]];
        assert_eq!(got, expected);
    }
);

joinp_test_tab!(
    joinp_right_anti_tab,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--right-anti");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![svec!["city", "place"], svec!["Orlando", "Disney World"]];
        assert_eq!(got, expected);
    }
);

joinp_test_comments!(
    joinp_right_anti_comments,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--right-anti");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![svec!["city", "place"], svec!["Orlando", "Disney World"]];
        assert_eq!(got, expected);
    }
);

#[test]
fn joinp_ignore_leading_zero() {
    let wrk = Workdir::new("joinp_ignore_leading_zero");

    wrk.create(
        "left.csv",
        vec![
            svec!["id", "value"],
            svec!["001", "a"],
            svec!["02", "b"],
            svec!["3", "c"],
        ],
    );

    wrk.create(
        "right.csv",
        vec![
            svec!["id", "desc"],
            svec!["1", "one"],
            svec!["02", "two"],
            svec!["003", "three"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.args(&["id", "left.csv", "id", "right.csv"]).arg("-z");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // note that id and id_right have been stripped of leading zeros
    // this is because Polars inferred the schema as all integers
    // and automatically converted the values to integers
    let expected = vec![
        svec!["id", "value", "id_right", "desc"],
        svec!["1", "a", "1", "one"],
        svec!["2", "b", "2", "two"],
        svec!["3", "c", "3", "three"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_ignore_leading_zero_string_schema() {
    let wrk = Workdir::new("joinp_ignore_leading_zero_string_schema");

    wrk.create(
        "left.csv",
        vec![
            svec!["id", "value"],
            svec!["001", "a"],
            svec!["02", "b"],
            svec!["3", "c"],
        ],
    );

    wrk.create(
        "right.csv",
        vec![
            svec!["id", "desc"],
            svec!["1", "one"],
            svec!["02", "two"],
            svec!["003", "three"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.args(&["id", "left.csv", "id", "right.csv"])
        .arg("-z")
        .args(["--cache-schema", "-2"]); // force schema to all String types

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // note that id and id_right have been stripped of leading zeros
    // this is because Polars inferred the schema as all integers
    // and automatically converted the values to integers
    let expected = vec![
        svec!["id", "value", "id_right", "desc"],
        svec!["001", "a", "1", "one"],
        svec!["02", "b", "02", "two"],
        svec!["3", "c", "003", "three"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_ignore_leading_zero_with_non_numeric() {
    let wrk = Workdir::new("joinp_ignore_leading_zero_with_non_numeric");

    wrk.create(
        "left.csv",
        vec![
            svec!["code", "value"],
            svec!["001A", "a"],
            svec!["02B", "b"],
            svec!["ABC", "c"],
        ],
    );

    wrk.create(
        "right.csv",
        vec![
            svec!["code", "desc"],
            svec!["1A", "one"],
            svec!["0002B", "two"],
            svec!["ABC", "three"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.args(&["code", "left.csv", "code", "right.csv"])
        .arg("--ignore-leading-zeros");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["code", "value", "code_right", "desc"],
        svec!["001A", "a", "1A", "one"],
        svec!["02B", "b", "0002B", "two"],
        svec!["ABC", "c", "ABC", "three"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_ignore_leading_zero_multiple_columns() {
    let wrk = Workdir::new("joinp_ignore_leading_zero_multiple_columns");

    wrk.create(
        "left.csv",
        vec![
            svec!["id", "code", "value"],
            svec!["001", "001A", "a"],
            svec!["02", "02B", "b"],
            svec!["3", "ABC", "c"],
        ],
    );

    wrk.create(
        "right.csv",
        vec![
            svec!["id", "code", "desc"],
            svec!["1", "1A", "one"],
            svec!["002", "0002B", "two"],
            svec!["03", "ABC", "three"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.args(&["id,code", "left.csv", "id,code", "right.csv"])
        .arg("--ignore-leading-zeros")
        .args(["--cache-schema", "-2"]); // force schema to all String types

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "code", "value", "id_right", "code_right", "desc"],
        svec!["001", "001A", "a", "1", "1A", "one"],
        svec!["02", "02B", "b", "002", "0002B", "two"],
        svec!["3", "ABC", "c", "03", "ABC", "three"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_ignore_case_and_leading_zeros() {
    let wrk = Workdir::new("joinp_ignore_case_and_leading_zeros");

    wrk.create(
        "left.csv",
        vec![
            svec!["id", "code", "value"],
            svec!["001", "001abc", "a"],
            svec!["02", "02DEF", "b"],
            svec!["3", "XYZ", "c"],
        ],
    );

    wrk.create(
        "right.csv",
        vec![
            svec!["id", "code", "desc"],
            svec!["1", "00001ABC", "one"],
            svec!["002", "0002def", "two"],
            svec!["03", "xyz", "three"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.args(&["id,code", "left.csv", "id,code", "right.csv"])
        .arg("--ignore-leading-zeros")
        .arg("--ignore-case")
        .args(["--cache-schema", "-2"]); // force schema to all String types

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "code", "value", "id_right", "code_right", "desc"],
        svec!["001", "001abc", "a", "1", "00001ABC", "one"],
        svec!["02", "02DEF", "b", "002", "0002def", "two"],
        svec!["3", "XYZ", "c", "03", "xyz", "three"],
    ];
    assert_eq!(got, expected);
}
#[test]
fn joinp_ignore_case_and_leading_zeros_coalesce() {
    let wrk = Workdir::new("joinp_ignore_case_and_leading_zeros_coalesce");

    wrk.create(
        "left.csv",
        vec![
            svec!["id", "code", "value"],
            svec!["001", "001abc", "a"],
            svec!["02", "02DEF", "b"],
            svec!["3", "XYZ", "c"],
        ],
    );

    wrk.create(
        "right.csv",
        vec![
            svec!["id", "code", "desc"],
            svec!["1", "00001ABC", "one"],
            svec!["002", "0002def", "two"],
            svec!["03", "xyz", "three"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.args(&["id,code", "left.csv", "id,code", "right.csv"])
        .arg("--ignore-leading-zeros")
        .arg("--ignore-case")
        .arg("--coalesce")
        .args(["--cache-schema", "-2"]); // force schema to all String types

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "code", "value", "desc"],
        svec!["001", "001abc", "a", "one"],
        svec!["02", "02DEF", "b", "two"],
        svec!["3", "XYZ", "c", "three"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_non_equi_greater_than() {
    let wrk = Workdir::new("joinp_non_equi_greater_than");

    // Create test data with numeric values to compare
    let prices = vec![
        svec!["item", "price"],
        svec!["apple", "1.00"],
        svec!["banana", "2.00"],
        svec!["orange", "3.00"],
    ];
    let budgets = vec![
        svec!["customer", "budget"],
        svec!["Alice", "2.50"],
        svec!["Bob", "1.50"],
        svec!["Carol", "3.50"],
    ];

    wrk.create("prices.csv", prices);
    wrk.create("budgets.csv", budgets);

    let mut cmd = wrk.command("joinp");
    cmd.arg("--non-equi")
        .arg("budget_right > price_left")
        .args(&["prices.csv", "budgets.csv"])
        .args(["--float-precision", "2"]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["item_left", "price_left", "customer_right", "budget_right"],
        svec!["apple", "1.00", "Bob", "1.50"],
        svec!["apple", "1.00", "Alice", "2.50"],
        svec!["apple", "1.00", "Carol", "3.50"],
        svec!["banana", "2.00", "Alice", "2.50"],
        svec!["banana", "2.00", "Carol", "3.50"],
        svec!["orange", "3.00", "Carol", "3.50"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_non_equi_less_than() {
    let wrk = Workdir::new("joinp_non_equi_less_than");

    let dates = vec![
        svec!["event", "date"],
        svec!["meeting", "2024-01-01"],
        svec!["party", "2024-06-15"],
        svec!["conference", "2024-12-31"],
    ];
    let deadlines = vec![
        svec!["task", "deadline"],
        svec!["report", "2024-03-01"],
        svec!["presentation", "2024-07-01"],
        svec!["review", "2024-12-15"],
    ];

    wrk.create("events.csv", dates);
    wrk.create("deadlines.csv", deadlines);

    let mut cmd = wrk.command("joinp");
    cmd.arg("--non-equi")
        .arg("date_left < deadline_right")
        .args(&["events.csv", "deadlines.csv"]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["event_left", "date_left", "task_right", "deadline_right"],
        svec!["meeting", "2024-01-01", "report", "2024-03-01"],
        svec!["meeting", "2024-01-01", "presentation", "2024-07-01"],
        svec!["meeting", "2024-01-01", "review", "2024-12-15"],
        svec!["party", "2024-06-15", "presentation", "2024-07-01"],
        svec!["party", "2024-06-15", "review", "2024-12-15"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_non_equi_less_than_date_arithmetic() {
    let wrk = Workdir::new("joinp_non_equi_less_than_date_arithmetic");

    let dates = vec![
        svec!["event", "date"],
        svec!["meeting", "2024-01-01"],
        svec!["party", "2024-06-15"],
        svec!["conference", "2024-12-31"],
    ];
    let deadlines = vec![
        svec!["task", "deadline"],
        svec!["report", "2024-03-01"],
        svec!["presentation", "2024-07-01"],
        svec!["review", "2024-12-15"],
    ];

    wrk.create("events.csv", dates);
    wrk.create("deadlines.csv", deadlines);

    let mut cmd = wrk.command("joinp");
    cmd.arg("--non-equi")
        .arg("date_left  + interval '4 months' < deadline_right")
        .args(&["events.csv", "deadlines.csv"])
        .arg("--try-parsedates");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["event_left", "date_left", "task_right", "deadline_right"],
        // svec!["meeting", "2024-01-01", "report", "2024-03-01"], this is less than 4 months
        svec!["meeting", "2024-01-01", "presentation", "2024-07-01"],
        svec!["meeting", "2024-01-01", "review", "2024-12-15"],
        // svec!["party", "2024-06-15", "presentation", "2024-07-01"], this is less than 4 months
        svec!["party", "2024-06-15", "review", "2024-12-15"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_non_equi_not_equal() {
    let wrk = Workdir::new("joinp_non_equi_not_equal");

    let teams = vec![
        svec!["player", "team"],
        svec!["Alice", "Red"],
        svec!["Bob", "Blue"],
        svec!["Carol", "Red"],
    ];
    let matches = vec![
        svec!["opponent1", "team"],
        svec!["David", "Green"],
        svec!["Eve", "Blue"],
        svec!["Frank", "Red"],
    ];

    wrk.create("teams.csv", teams);
    wrk.create("matches.csv", matches);

    let mut cmd = wrk.command("joinp");
    cmd.arg("--non-equi")
        .arg("team_left != team_right")
        .args(&["teams.csv", "matches.csv"]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["player_left", "team_left", "opponent1_right", "team_right"],
        svec!["Alice", "Red", "David", "Green"],
        svec!["Alice", "Red", "Eve", "Blue"],
        svec!["Bob", "Blue", "David", "Green"],
        svec!["Bob", "Blue", "Frank", "Red"],
        svec!["Carol", "Red", "David", "Green"],
        svec!["Carol", "Red", "Eve", "Blue"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_non_equi_invalid_operator() {
    let wrk = Workdir::new("joinp_non_equi_invalid_operator");

    let mut cmd = wrk.command("joinp");
    cmd.arg("--non-equi")
        .arg("col1 INVALID col2")
        .args(&["cities.csv", "places.csv"]);

    wrk.assert_err(&mut cmd);
}

#[test]
fn joinp_non_equi_invalid_format() {
    let wrk = Workdir::new("joinp_non_equi_invalid_format");

    let mut cmd = wrk.command("joinp");
    cmd.arg("--non-equi")
        .arg("invalid expression format")
        .args(&["cities.csv", "places.csv"]);

    wrk.assert_err(&mut cmd);
}

#[test]
fn joinp_non_equi_compound() {
    let wrk = Workdir::new("joinp_non_equi_compound");

    // Create test data with employee salaries and job requirements
    let employees = vec![
        svec!["name", "salary", "experience"],
        svec!["Alice", "75000", "5"],
        svec!["Bob", "85000", "3"],
        svec!["Carol", "95000", "8"],
        svec!["David", "65000", "2"],
    ];
    let jobs = vec![
        svec!["position", "min_salary", "min_exp", "max_salary"],
        svec!["Senior Dev", "80000", "5", "100000"],
        svec!["Junior Dev", "60000", "2", "80000"],
        svec!["Tech Lead", "90000", "7", "120000"],
    ];

    wrk.create("employees.csv", employees);
    wrk.create("jobs.csv", jobs);

    let mut cmd = wrk.command("joinp");
    cmd.arg("--non-equi")
        .arg(
            "salary_left >= min_salary_right AND salary_left <= max_salary_right AND \
             experience_left >= min_exp_right",
        )
        .args(&["employees.csv", "jobs.csv"])
        .args(&[
            "--sql-filter",
            "select * from join_result order by name_left, salary_left, experience_left, \
             position_right",
        ]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "name_left",
            "salary_left",
            "experience_left",
            "position_right",
            "min_salary_right",
            "min_exp_right",
            "max_salary_right"
        ],
        svec!["Alice", "75000", "5", "Junior Dev", "60000", "2", "80000"],
        svec!["Carol", "95000", "8", "Senior Dev", "80000", "5", "100000"],
        svec!["Carol", "95000", "8", "Tech Lead", "90000", "7", "120000"],
        svec!["David", "65000", "2", "Junior Dev", "60000", "2", "80000"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_ignore_leading_zeros_issue_2424() {
    let wrk = Workdir::new("joinp_ignore_leading_zeros_issue_2424");

    wrk.create(
        "file1.csv",
        vec![
            svec!["id", "company_id", "art_no"],
            svec!["1", "COM1", "0724"],
            svec!["2", "cm2", "002"],
            svec!["3", "com3", "78"],
            svec!["4", "CM2", "01"],
            svec!["5", "Cp5", "1"],
            svec!["6", "CPA", "000"],
        ],
    );

    wrk.create(
        "file2.csv",
        vec![
            svec!["id", "company_id", "art_no"],
            svec!["1", "com1", "724"],
            svec!["2", "CM2", "02"],
            svec!["3", "Com3", "078"],
            svec!["4", "cm2", "1"],
            svec!["5", "CP5", "01"],
            svec!["6", "cpa", "0"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.args(&[
        "-i",
        "-z",
        "--left-semi",
        "--cache-schema",
        "-1",
        "company_id,art_no",
        "file1.csv",
        "company_id,art_no",
        "file2.csv",
    ]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "company_id", "art_no"],
        svec!["1", "COM1", "0724"],
        svec!["2", "cm2", "002"],
        svec!["3", "com3", "78"],
        svec!["4", "CM2", "01"],
        svec!["5", "Cp5", "1"],
        svec!["6", "CPA", "000"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_unicode_normalization() {
    let wrk = Workdir::new("joinp_unicode_normalization");

    wrk.create(
        "left.csv",
        vec![
            svec!["name", "value"],
            svec!["cafe\u{0301}", "a"], // café decomposed
            svec!["café", "b"],         // café precomposed
        ],
    );

    wrk.create(
        "right.csv",
        vec![
            svec!["name", "desc"],
            svec!["café", "one"],         // café precomposed
            svec!["cafe\u{0301}", "two"], // café decomposed
        ],
    );

    // Test NFC normalization
    let mut cmd = wrk.command("joinp");
    cmd.args(&["name", "left.csv", "name", "right.csv"])
        .args(["--norm-unicode", "nfc"]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "value", "name_right", "desc"],
        svec!["cafe\u{301}", "a", "café", "one"],
        svec!["café", "b", "café", "one"],
        svec!["cafe\u{301}", "a", "cafe\u{301}", "two"],
        svec!["café", "b", "cafe\u{301}", "two"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_unicode_normalization_with_other_options() {
    let wrk = Workdir::new("joinp_unicode_normalization_with_other_options");

    wrk.create(
        "left.csv",
        vec![
            svec!["id", "name", "value"],
            svec!["001", "CAFÉ", "a"],
            svec!["02", "cafe\u{0301}", "b"],
        ],
    );

    wrk.create(
        "right.csv",
        vec![
            svec!["id", "name", "desc"],
            svec!["1", "café", "one"],
            svec!["002", "CAFE\u{0301}", "two"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.args(&["id,name", "left.csv", "id,name", "right.csv"])
        .args(["--norm-unicode", "nfkc"])
        .arg("--ignore-leading-zeros")
        .arg("--ignore-case")
        .args(["--cache-schema", "-2"]); // force schema to all String types

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "name", "value", "id_right", "name_right", "desc"],
        svec!["001", "CAFÉ", "a", "1", "café", "one"],
        svec!["02", "cafe\u{0301}", "b", "002", "CAFE\u{0301}", "two"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_unicode_normalization_ligatures() {
    let wrk = Workdir::new("joinp_unicode_normalization_ligatures");

    wrk.create(
        "left.csv",
        vec![
            svec!["name", "value"],
            // Different ligature representations
            svec!["fi\u{FB01}le", "a"], // "file" with "fi" ligature
            svec!["ﬁle", "b"],          // "file" with precomposed "fi" ligature
            svec!["file", "c"],         // "file" without ligature
            svec!["o\u{FB03}ce", "d"],  // "office" with "ffi" ligature
            svec!["oﬃce", "e"],         // "office" with precomposed "ffi" ligature
            svec!["office", "f"],       // "office" without ligature
        ],
    );

    wrk.create(
        "right.csv",
        vec![
            svec!["name", "desc"],
            svec!["file", "plain"],   // Regular "file"
            svec!["office", "plain"], // Regular "office"
        ],
    );

    // Test NFKC normalization (should decompose ligatures)
    let mut cmd = wrk.command("joinp");
    cmd.args(&["name", "left.csv", "name", "right.csv"])
        .args(["--norm-unicode", "nfkc"])
        .args(["--maintain-order", "left"]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "value", "name_right", "desc"],
        svec!["ﬁle", "b", "file", "plain"],
        svec!["file", "c", "file", "plain"],
        svec!["oﬃce", "d", "office", "plain"],
        svec!["oﬃce", "e", "office", "plain"],
        svec!["office", "f", "office", "plain"],
    ];
    assert_eq!(got, expected);

    // Test NFKD normalization (should also decompose ligatures)
    let mut cmd = wrk.command("joinp");
    cmd.args(&["name", "left.csv", "name", "right.csv"])
        .args(["--norm-unicode", "nfkd"]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, expected);

    // Test NFC normalization (should NOT decompose ligatures)
    let mut cmd = wrk.command("joinp");
    cmd.args(&["name", "left.csv", "name", "right.csv"])
        .args(["--norm-unicode", "nfc"])
        .args(["--maintain-order", "left"]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "value", "name_right", "desc"],
        svec!["file", "c", "file", "plain"],
        svec!["office", "f", "office", "plain"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asof_allow_exact_matches() {
    let wrk = Workdir::new("joinp_asof_allow_exact_matches");

    // Test data includes both exact and inexact matches
    wrk.create(
        "trades.csv",
        vec![
            svec!["time", "price"],
            svec!["2024-01-01 10:00:00", "100.0"], // Exact match
            svec!["2024-01-01 10:00:03", "100.5"], // In between quotes
            svec!["2024-01-01 10:00:05", "101.0"], // Exact match
            svec!["2024-01-01 10:00:08", "101.5"], // In between quotes
            svec!["2024-01-01 10:00:10", "102.0"], // Exact match
            svec!["2024-01-01 10:00:12", "102.5"], // In between quotes
            svec!["2024-01-01 10:00:15", "103.0"], // Exact match
        ],
    );

    wrk.create(
        "quotes.csv",
        vec![
            svec!["time", "bid"],
            svec!["2024-01-01 10:00:00", "99.5"], // Matches trades[0]
            svec!["2024-01-01 10:00:05", "99.5"], // Matches trades[2]
            svec!["2024-01-01 10:00:10", "101.5"], // Matches trades[4]
            svec!["2024-01-01 10:00:15", "102.25"], // Matches trades[6]
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["time", "trades.csv", "time", "quotes.csv"])
        .arg("--allow-exact-matches")
        .arg("--try-parsedates")
        .args(["--datetime-format", "%Y-%m-%d %H:%M:%S"]);

    let expected = vec![
        svec!["time", "price", "bid"],
        svec!["2024-01-01 10:00:00", "100.0", "99.5"], // Exact match
        svec!["2024-01-01 10:00:03", "100.5", "99.5"], // Uses previous quote
        svec!["2024-01-01 10:00:05", "101.0", "99.5"], // Exact match
        svec!["2024-01-01 10:00:08", "101.5", "99.5"], // Uses previous quote
        svec!["2024-01-01 10:00:10", "102.0", "101.5"], // Exact match
        svec!["2024-01-01 10:00:12", "102.5", "101.5"], // Uses previous quote
        svec!["2024-01-01 10:00:15", "103.0", "102.25"], // Exact match
    ];

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, expected);

    // Test without --allow-exact-matches
    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["time", "trades.csv", "time", "quotes.csv"])
        .arg("--try-parsedates")
        .args(["--datetime-format", "%Y-%m-%d %H:%M:%S"]);

    let expected = vec![
        svec!["time", "price", "bid"],
        svec!["2024-01-01 10:00:00", "100.0", ""], // No match since exact matches not allowed
        svec!["2024-01-01 10:00:03", "100.5", "99.5"], // Uses quote from 10:00:00
        svec!["2024-01-01 10:00:05", "101.0", "99.5"], /* Uses quote from 10:00:00 (exact not
                                                    * allowed) */
        svec!["2024-01-01 10:00:08", "101.5", "99.5"], // Uses quote from 10:00:05
        svec!["2024-01-01 10:00:10", "102.0", "99.5"], /* Uses quote from 10:00:05 (exact not
                                                        * allowed) */
        svec!["2024-01-01 10:00:12", "102.5", "101.5"], // Uses quote from 10:00:10
        svec!["2024-01-01 10:00:15", "103.0", "101.5"], /* Uses quote from 10:00:10 (exact not
                                                         * allowed) */
    ];

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn joinp_asof_sortkey_options() {
    let wrk = Workdir::new("joinp_asof_sortkey_options");

    // Exactly the same trades and quotes test data as the joinp_asof_allow_exact_matches test
    // above, but the data are not sorted.
    wrk.create(
        "trades.csv",
        vec![
            svec!["time", "price"],
            svec!["2024-01-01 10:00:10", "102.0"],
            svec!["2024-01-01 10:00:05", "101.0"],
            svec!["2024-01-01 10:00:00", "100.0"],
            svec!["2024-01-01 10:00:15", "103.0"],
            svec!["2024-01-01 10:00:12", "102.5"],
            svec!["2024-01-01 10:00:03", "100.5"],
            svec!["2024-01-01 10:00:08", "101.5"],
        ],
    );

    wrk.create(
        "quotes.csv",
        vec![
            svec!["time", "bid"],
            svec!["2024-01-01 10:00:10", "101.5"],
            svec!["2024-01-01 10:00:05", "99.5"],
            svec!["2024-01-01 10:00:00", "99.5"],
            svec!["2024-01-01 10:00:15", "102.25"],
        ],
    );

    // But we automatically sort by the asof columns by default, so this works
    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["time", "trades.csv", "time", "quotes.csv"])
        .arg("--allow-exact-matches")
        .arg("--try-parsedates")
        .args(["--datetime-format", "%Y-%m-%d %H:%M:%S"]);

    let expected = vec![
        svec!["time", "price", "bid"],
        svec!["2024-01-01 10:00:00", "100.0", "99.5"], // Exact match
        svec!["2024-01-01 10:00:03", "100.5", "99.5"], // Uses previous quote
        svec!["2024-01-01 10:00:05", "101.0", "99.5"], // Exact match
        svec!["2024-01-01 10:00:08", "101.5", "99.5"], // Uses previous quote
        svec!["2024-01-01 10:00:10", "102.0", "101.5"], // Exact match
        svec!["2024-01-01 10:00:12", "102.5", "101.5"], // Uses previous quote
        svec!["2024-01-01 10:00:15", "103.0", "102.25"], // Exact match
    ];

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, expected);

    // Test without --allow-exact-matches
    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["time", "trades.csv", "time", "quotes.csv"])
        .arg("--try-parsedates")
        .args(["--datetime-format", "%Y-%m-%d %H:%M:%S"]);

    let expected = vec![
        svec!["time", "price", "bid"],
        svec!["2024-01-01 10:00:00", "100.0", ""],
        svec!["2024-01-01 10:00:03", "100.5", "99.5"],
        svec!["2024-01-01 10:00:05", "101.0", "99.5"],
        svec!["2024-01-01 10:00:08", "101.5", "99.5"],
        svec!["2024-01-01 10:00:10", "102.0", "99.5"],
        svec!["2024-01-01 10:00:12", "102.5", "101.5"],
        svec!["2024-01-01 10:00:15", "103.0", "101.5"],
    ];

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, expected);

    // Test with --no-sort
    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["time", "trades.csv", "time", "quotes.csv"])
        .arg("--no-sort")
        .arg("--try-parsedates")
        .args(["--datetime-format", "%Y-%m-%d %H:%M:%S"]);

    // and the output is INCORRECT because the data is not sorted
    let expected = vec![
        svec!["time", "price", "bid"],
        svec!["2024-01-01 10:00:10", "102.0", ""],
        svec!["2024-01-01 10:00:05", "101.0", ""],
        svec!["2024-01-01 10:00:00", "100.0", ""],
        svec!["2024-01-01 10:00:15", "103.0", "99.5"],
        svec!["2024-01-01 10:00:12", "102.5", "99.5"],
        svec!["2024-01-01 10:00:03", "100.5", "99.5"],
        svec!["2024-01-01 10:00:08", "101.5", "99.5"],
    ];

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn joinp_decimal_comma_validation() {
    let wrk = Workdir::new("joinp_decimal_comma_validation");

    // Create test data with decimal commas
    let left_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create("left.csv", left_data.clone());

    let right_data = vec![svec!["id", "name"], svec!["1", "Alice"], svec!["2", "Bob"]];
    wrk.create("right.csv", right_data.clone());

    // Test 1: --decimal-comma with comma delimiter should fail
    let mut cmd = wrk.command("joinp");
    cmd.args(["id", "left.csv", "id", "right.csv"])
        .arg("--decimal-comma");

    wrk.assert_err(&mut cmd);
    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Using --decimal-comma with a comma separator is invalid"));

    // Test 2: --decimal-comma with semicolon delimiter should succeed
    // Create semicolon-delimited versions of the data
    wrk.create_with_delim("left_semi.csv", left_data.clone(), b';');
    wrk.create_with_delim("right_semi.csv", right_data.clone(), b';');

    let mut cmd = wrk.command("joinp");
    cmd.args(["id", "left_semi.csv", "id", "right_semi.csv"])
        .arg("--decimal-comma")
        .args(["--delimiter", ";"]);

    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"id;value;name
1;100,5;Alice
2;200,75;Bob"#;
    assert_eq!(got, expected);

    // Test 3: --decimal-comma with tab delimiter should succeed
    // Create tab-delimited versions of the data
    wrk.create_with_delim("left_tab.csv", left_data.clone(), b'\t');
    wrk.create_with_delim("right_tab.csv", right_data.clone(), b'\t');

    let mut cmd = wrk.command("joinp");
    cmd.args(["id", "left_tab.csv", "id", "right_tab.csv"])
        .arg("--decimal-comma")
        .args(["--delimiter", "\t"]);

    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    let expected = "id\tvalue\tname\n1\t100,5\tAlice\n2\t200,75\tBob";
    assert_eq!(got, expected);

    // Test 4: --decimal-comma with pipe delimiter should succeed
    // Create pipe-delimited versions of the data
    wrk.create_with_delim("left_pipe.csv", left_data.clone(), b'|');
    wrk.create_with_delim("right_pipe.csv", right_data.clone(), b'|');
    let mut cmd = wrk.command("joinp");
    cmd.args(["id", "left_pipe.csv", "id", "right_pipe.csv"])
        .arg("--decimal-comma")
        .args(["--delimiter", "|"]);

    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    let expected = "id|value|name\n1|100,5|Alice\n2|200,75|Bob";
    assert_eq!(got, expected);
}

#[test]
fn joinp_decimal_comma_validation_with_tsv_files() {
    let wrk = Workdir::new("joinp_decimal_comma_validation_with_tsv_files");

    // Create test data with decimal commas in TSV format
    let left_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create_with_delim("left.tsv", left_data, b'\t');

    let right_data = vec![svec!["id", "name"], svec!["1", "Alice"], svec!["2", "Bob"]];
    wrk.create_with_delim("right.tsv", right_data, b'\t');

    // Test: --decimal-comma with TSV files should fail because the validation
    // checks the delimiter parameter, not the file extension
    let mut cmd = wrk.command("joinp");
    cmd.args(["id", "left.tsv", "id", "right.tsv"])
        .arg("--decimal-comma");

    wrk.assert_err(&mut cmd);
    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Using --decimal-comma with a comma separator is invalid"));
}

#[test]
fn joinp_decimal_comma_validation_with_ssv_files() {
    let wrk = Workdir::new("joinp_decimal_comma_validation_with_ssv_files");

    // Create test data with decimal commas in SSV format
    let left_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create_with_delim("left.ssv", left_data, b';');

    let right_data = vec![svec!["id", "name"], svec!["1", "Alice"], svec!["2", "Bob"]];
    wrk.create_with_delim("right.ssv", right_data, b';');

    // Test 1: --decimal-comma with SSV files (semicolon delimiter) should fail here
    // because stdin is not a file, so the delimiter is not detected, so it defaults to
    // comma, as --delimiter is not set.
    let mut cmd = wrk.command("joinp");
    cmd.args(["id", "left.ssv", "id", "right.ssv"])
        .arg("--decimal-comma");

    wrk.assert_err(&mut cmd);
    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Using --decimal-comma with a comma separator is invalid"));

    // Test 2: --decimal-comma with SSV files (semicolon delimiter) should succeed
    let mut cmd = wrk.command("joinp");
    cmd.args(["id", "left.ssv", "id", "right.ssv"])
        .arg("--decimal-comma")
        .args(["--delimiter", ";"]);

    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    let expected = "id;value;name\n1;100,5;Alice\n2;200,75;Bob";
    assert_eq!(got, expected);
}

#[test]
fn joinp_decimal_comma_validation_with_output_file() {
    let wrk = Workdir::new("joinp_decimal_comma_validation_with_output_file");

    // Create test data with decimal commas
    let left_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create_with_delim("left.csv", left_data, b';');

    let right_data = vec![svec!["id", "name"], svec!["1", "Alice"], svec!["2", "Bob"]];
    wrk.create_with_delim("right.csv", right_data, b';');

    // Test: --decimal-comma with output file should validate the output delimiter
    let mut cmd = wrk.command("joinp");
    cmd.args(["id", "left.csv", "id", "right.csv"])
        .arg("--decimal-comma")
        .args(["--delimiter", ";"])
        .args(["--output", "output.csv"]);

    wrk.assert_success(&mut cmd);
    // For semicolon-delimited output, we need to read the raw content and parse manually
    let raw_output = wrk.read_to_string("output.csv").unwrap();
    let lines: Vec<&str> = raw_output.lines().collect();
    let got: String = lines.join("\n");
    let expected = "id;value;name\n1;100,5;Alice\n2;200,75;Bob";
    assert_eq!(got, expected);

    // Test: --decimal-comma with output file that would have comma delimiter should fail
    let mut cmd = wrk.command("joinp");
    cmd.args(["id", "left.csv", "id", "right.csv"])
        .arg("--decimal-comma")
        .args(["--output", "output.csv"]);

    wrk.assert_err(&mut cmd);
    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Using --decimal-comma with a comma separator is invalid"));
}

#[test]
fn joinp_decimal_comma_validation_with_tsv_output() {
    let wrk = Workdir::new("joinp_decimal_comma_validation_with_tsv_output");

    // Create test data with decimal commas
    let left_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create_with_delim("left.tsv", left_data, b'\t');

    let right_data = vec![svec!["id", "name"], svec!["1", "Alice"], svec!["2", "Bob"]];
    wrk.create_with_delim("right.tsv", right_data, b'\t');

    // Test: --decimal-comma with TSV output file should succeed
    let mut cmd = wrk.command("joinp");
    cmd.args(["id", "left.tsv", "id", "right.tsv"])
        .arg("--decimal-comma")
        .args(["--delimiter", "\t"])
        .args(["--output", "output.tsv"]);

    wrk.assert_success(&mut cmd);
    let got: String = wrk.read_to_string("output.tsv").unwrap();
    let expected = "id\tvalue\tname\n1\t100,5\tAlice\n2\t200,75\tBob\n";
    assert_eq!(got, expected);
}

#[test]
fn joinp_decimal_comma_validation_with_ssv_output() {
    let wrk = Workdir::new("joinp_decimal_comma_validation_with_ssv_output");

    // Create test data with decimal commas
    let left_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create_with_delim("left.ssv", left_data, b';');

    let right_data = vec![svec!["id", "name"], svec!["1", "Alice"], svec!["2", "Bob"]];
    wrk.create_with_delim("right.ssv", right_data, b';');

    // Test: --decimal-comma with SSV output file should succeed
    let mut cmd = wrk.command("joinp");
    cmd.args(["id", "left.ssv", "id", "right.ssv"])
        .arg("--decimal-comma")
        .args(["--delimiter", ";"])
        .args(["--output", "output.ssv"]);

    wrk.assert_success(&mut cmd);
    // For SSV files, we need to read the raw content and parse manually
    let raw_output = wrk.read_to_string("output.ssv").unwrap();
    let lines: Vec<&str> = raw_output.lines().collect();
    let got: String = lines.join("\n");
    let expected = "id;value;name\n1;100,5;Alice\n2;200,75;Bob";
    assert_eq!(got, expected);
}

#[test]
fn joinp_decimal_comma_validation_with_cross_join() {
    let wrk = Workdir::new("joinp_decimal_comma_validation_with_cross_join");

    // Create test data with decimal commas
    let left_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create_with_delim("left.ssv", left_data, b';');

    let right_data = vec![svec!["name"], svec!["Alice"], svec!["Bob"]];
    wrk.create_with_delim("right.ssv", right_data, b';');

    // Test: --decimal-comma with cross join and comma delimiter should fail
    let mut cmd = wrk.command("joinp");
    cmd.arg("--cross")
        .args(["left.ssv", "right.ssv"])
        .arg("--decimal-comma");

    wrk.assert_err(&mut cmd);
    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Using --decimal-comma with a comma separator is invalid"));

    // Test: --decimal-comma with cross join and semicolon delimiter should succeed
    let mut cmd = wrk.command("joinp");
    cmd.arg("--cross")
        .args(["left.ssv", "right.ssv"])
        .arg("--decimal-comma")
        .args(["--delimiter", ";"]);

    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    let expected = "id;value;name\n1;100,5;Alice\n1;100,5;Bob\n2;200,75;Alice\n2;200,75;Bob";
    assert_eq!(got, expected);
}

#[test]
fn joinp_decimal_comma_validation_with_non_equi_join() {
    let wrk = Workdir::new("joinp_decimal_comma_validation_with_non_equi_join");

    // Create test data with decimal commas
    let left_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create_with_delim("left.csv", left_data, b';');

    let right_data = vec![
        svec!["id", "min_val", "max_val"],
        svec!["1", "50,5", "150,5"],
        svec!["2", "150,0", "250,0"],
    ];
    wrk.create_with_delim("right.csv", right_data, b';');

    // Test: --decimal-comma with non-equi join and comma delimiter should fail
    let mut cmd = wrk.command("joinp");
    cmd.arg("--non-equi")
        .arg("value_left >= min_val_right AND value_left <= max_val_right")
        .args(["left.csv", "right.csv"])
        .arg("--decimal-comma");

    wrk.assert_err(&mut cmd);
    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Using --decimal-comma with a comma separator is invalid"));

    // Test: --decimal-comma with non-equi join and semicolon delimiter should succeed
    let mut cmd = wrk.command("joinp");
    cmd.arg("--non-equi")
        .arg("value_left >= min_val_right AND value_left <= max_val_right")
        .args(["left.csv", "right.csv"])
        .arg("--decimal-comma")
        .args(["--delimiter", ";"]);

    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"id_left;value_left;id_right;min_val_right;max_val_right
1;100,5;1;50,5;150,5
2;200,75;2;150,0;250,0"#;
    let expected2 = r#"id_left;value_left;id_right;min_val_right;max_val_right
2;200,75;2;150,0;250,0
1;100,5;1;50,5;150,5"#;
    assert!(got == expected || got == expected2);
}

#[test]
fn joinp_decimal_comma_validation_with_asof_join() {
    let wrk = Workdir::new("joinp_decimal_comma_validation_with_asof_join");

    // Create test data with decimal commas
    let left_data = vec![
        svec!["id", "value", "date"],
        svec!["1", "100,50", "2023-01-01"],
        svec!["2", "200,75", "2023-01-02"],
    ];
    wrk.create_with_delim("left.csv", left_data, b'|');

    let right_data = vec![
        svec!["id", "price", "date"],
        svec!["1", "50,25", "2023-01-01"],
        svec!["2", "150,50", "2023-01-02"],
    ];
    wrk.create_with_delim("right.csv", right_data, b'|');

    // Test: --decimal-comma with asof join and pipe delimiter should fail
    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["date", "left.csv", "date", "right.csv"])
        .arg("--decimal-comma");

    wrk.assert_err(&mut cmd);
    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Using --decimal-comma with a comma separator is invalid"));

    // Test: --decimal-comma with asof join and pipe delimiter should succeed
    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["date", "left.csv", "date", "right.csv"])
        .arg("--decimal-comma")
        .args(["--delimiter", "|"]);

    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    let expected =
        "id|value|date|id_right|price\n1|100,5|2023-01-01||\n2|200,75|2023-01-02|1|50,25";
    assert_eq!(got, expected);
}

#[test]
fn joinp_decimal_comma_validation_with_sql_filter() {
    let wrk = Workdir::new("joinp_decimal_comma_validation_with_sql_filter");

    // Create test data with decimal commas
    let left_data = vec![
        svec!["id", "value"],
        svec!["1", "100,50"],
        svec!["2", "200,75"],
    ];
    wrk.create_with_delim("left.csv", left_data, b';');

    let right_data = vec![svec!["id", "name"], svec!["1", "Alice"], svec!["2", "Bob"]];
    wrk.create_with_delim("right.csv", right_data, b';');

    // Test: --decimal-comma with SQL filter and comma delimiter should fail
    let mut cmd = wrk.command("joinp");
    cmd.args(["id", "left.csv", "id", "right.csv"])
        .arg("--decimal-comma")
        .arg("--sql-filter")
        .arg("select id, value from join_result where value > 150");

    wrk.assert_err(&mut cmd);
    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Using --decimal-comma with a comma separator is invalid"));

    // Test: --decimal-comma with SQL filter and semicolon delimiter should succeed
    let mut cmd = wrk.command("joinp");
    cmd.args(["id", "left.csv", "id", "right.csv"])
        .arg("--decimal-comma")
        .args(["--delimiter", ";"])
        .arg("--sql-filter")
        .arg("select id, value from join_result where value > 150");

    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    let expected = "id;value\n2;200,75";
    assert_eq!(got, expected);
}
