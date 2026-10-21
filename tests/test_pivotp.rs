use crate::workdir::Workdir;

macro_rules! pivotp_test {
    ($name:ident, $fun:expr_2021) => {
        mod $name {
            use std::process;

            #[allow(unused_imports)]
            use super::setup;
            use crate::workdir::Workdir;

            #[test]
            fn main() {
                let wrk = setup(stringify!($name));
                let cmd = wrk.command("pivotp");
                $fun(wrk, cmd);
            }
        }
    };
}

macro_rules! pivotp_maintain_order_test {
    ($name:ident, $fun:expr_2021) => {
        mod $name {
            use std::process;

            #[allow(unused_imports)]
            use super::setup_maintain_order;
            use crate::workdir::Workdir;

            #[test]
            fn main() {
                let wrk = setup_maintain_order(stringify!($name));
                let cmd = wrk.command("pivotp");
                $fun(wrk, cmd);
            }
        }
    };
}

fn setup(name: &str) -> Workdir {
    // Sample data for testing pivot operations
    let sales = vec![
        svec!["date", "product", "region", "sales"],
        svec!["2023-01-01", "A", "North", "100"],
        svec!["2023-01-01", "B", "North", "150"],
        svec!["2023-01-01", "A", "South", "200"],
        svec!["2023-01-02", "B", "South", "250"],
        svec!["2023-01-02", "A", "North", "300"],
        svec!["2023-01-02", "B", "North", "350"],
    ];

    let wrk = Workdir::new(name);
    wrk.create("sales.csv", sales);
    wrk
}

fn setup_maintain_order(name: &str) -> Workdir {
    // Sample data for testing pivot operations
    let sales = vec![
        svec!["date", "product", "region", "sales"],
        svec!["2023-01-01", "C", "North", "100"],
        svec!["2023-01-01", "D", "South", "200"],
        svec!["2023-01-02", "B", "South", "250"],
        svec!["2023-01-02", "A", "North", "300"],
        svec!["2023-01-01", "A", "North", "100"],
        svec!["2023-01-01", "B", "North", "150"],
        svec!["2023-01-01", "A", "South", "200"],
        svec!["2023-01-02", "B", "North", "350"],
        svec!["2023-01-02", "C", "South", "400"],
        svec!["2023-01-02", "D", "North", "450"],
    ];

    let wrk = Workdir::new(name);
    wrk.create("sales_maintain_order.csv", sales);
    wrk
}

// Test basic pivot with single index
pivotp_test!(pivotp_basic, |wrk: Workdir, mut cmd: process::Command| {
    cmd.args([
        "product",
        "--index",
        "date",
        "--values",
        "sales",
        "--agg",
        "first",
        "sales.csv",
    ]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date", "A", "B"],
        svec!["2023-01-01", "100", "150"],
        svec!["2023-01-02", "300", "250"],
    ];
    assert_eq!(got, expected);
});

// Test pivot with multiple index columns
pivotp_test!(
    pivotp_multi_index,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "date,region",
            "--values",
            "sales",
            "--agg",
            "sum",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "region", "A", "B"],
            svec!["2023-01-01", "North", "100", "150"],
            svec!["2023-01-01", "South", "200", "0"],
            svec!["2023-01-02", "South", "0", "250"],
            svec!["2023-01-02", "North", "300", "350"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with sum aggregation
pivotp_test!(pivotp_sum_agg, |wrk: Workdir, mut cmd: process::Command| {
    cmd.args([
        "product",
        "--index",
        "region",
        "--values",
        "sales",
        "--agg",
        "sum",
        "sales.csv",
    ]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["region", "A", "B"],
        svec!["North", "400", "500"],
        svec!["South", "200", "250"],
    ];
    assert_eq!(got, expected);
});

// Test pivot with mean aggregation
pivotp_test!(
    pivotp_mean_agg,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "mean",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["region", "A", "B"],
            svec!["North", "200.0", "250.0"],
            svec!["South", "200.0", "250.0"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with min aggregation
pivotp_test!(pivotp_min_agg, |wrk: Workdir, mut cmd: process::Command| {
    cmd.args([
        "product",
        "--index",
        "region",
        "--values",
        "sales",
        "--agg",
        "min",
        "sales.csv",
    ]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["region", "A", "B"],
        svec!["North", "100", "150"],
        svec!["South", "200", "250"],
    ];
    assert_eq!(got, expected);
});

// Test pivot with max aggregation
pivotp_test!(pivotp_max_agg, |wrk: Workdir, mut cmd: process::Command| {
    cmd.args([
        "product",
        "--index",
        "region",
        "--values",
        "sales",
        "--agg",
        "max",
        "sales.csv",
    ]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["region", "A", "B"],
        svec!["North", "300", "350"],
        svec!["South", "200", "250"],
    ];
    assert_eq!(got, expected);
});

// Test pivot with median aggregation
pivotp_test!(
    pivotp_median_agg,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "median",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["region", "A", "B"],
            svec!["North", "200.0", "250.0"],
            svec!["South", "200.0", "250.0"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with quantile@0.95 aggregation (linear interpolation)
// Fixture: North/A=[100,300] -> q95 = 100 + 0.95*200 = 290.0
//          North/B=[150,350] -> q95 = 150 + 0.95*200 = 340.0
//          South/A=[200], South/B=[250] -> single-element groups return that value
pivotp_test!(
    pivotp_quantile_p95_agg,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "quantile@0.95",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["region", "A", "B"],
            svec!["North", "290.0", "340.0"],
            svec!["South", "200.0", "250.0"],
        ];
        assert_eq!(got, expected);
    }
);

// Test that q@0.5 alias produces the same output as median for the sales fixture.
// For 2-element sorted groups, Polars' median() == linear quantile(0.5) == midpoint.
pivotp_test!(
    pivotp_q_alias_p50_equals_median,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "q@0.5",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        // Same expected output as pivotp_median_agg above.
        let expected = vec![
            svec!["region", "A", "B"],
            svec!["North", "200.0", "250.0"],
            svec!["South", "200.0", "250.0"],
        ];
        assert_eq!(got, expected);
    }
);

// Test that malformed quantile probabilities are rejected with a precise error.
pivotp_test!(
    pivotp_quantile_invalid_p_rejected,
    |wrk: Workdir, mut cmd: process::Command| {
        // Out-of-range high
        cmd.args([
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "quantile@1.5",
            "sales.csv",
        ]);
        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(
            stderr.contains("Invalid quantile probability"),
            "expected 'Invalid quantile probability' in stderr, got: {stderr}"
        );

        // Out-of-range low (negative)
        let mut cmd_2 = wrk.command("pivotp");
        cmd_2.args([
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "q@-0.1",
            "sales.csv",
        ]);
        let stderr2 = wrk.stderr_on_error(&mut cmd_2);
        assert!(
            stderr2.contains("Invalid quantile probability"),
            "expected 'Invalid quantile probability' in stderr, got: {stderr2}"
        );

        // Non-numeric suffix
        let mut cmd3 = wrk.command("pivotp");
        cmd3.args([
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "quantile@abc",
            "sales.csv",
        ]);
        let stderr3 = wrk.stderr_on_error(&mut cmd3);
        assert!(
            stderr3.contains("Invalid quantile probability"),
            "expected 'Invalid quantile probability' in stderr, got: {stderr3}"
        );
    }
);

// Test quantile in group-by mode (no <on-cols>).
// North sales sorted = [100, 150, 300, 350] -> q25 linear at index 0.75 = 100 + 0.75*50 = 137.5
// South sales sorted = [200, 250]           -> q25 linear at index 0.25 = 200 + 0.25*50 = 212.5
pivotp_test!(
    pivotp_quantile_groupby_mode,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "q@0.25",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["region", "sales"],
            svec!["North", "137.5"],
            svec!["South", "212.5"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with len aggregation
pivotp_test!(pivotp_len_agg, |wrk: Workdir, mut cmd: process::Command| {
    cmd.args([
        "product",
        "--index",
        "region",
        "--values",
        "sales",
        "--agg",
        "len",
        "sales.csv",
    ]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["region", "A", "B"],
        svec!["North", "2", "2"],
        svec!["South", "1", "1"],
    ];
    assert_eq!(got, expected);
});

// Test pivot with last aggregation
pivotp_test!(
    pivotp_last_agg,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "last",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["region", "A", "B"],
            svec!["North", "300", "350"],
            svec!["South", "200", "250"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with item aggregation
pivotp_test!(
    pivotp_item_agg,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "item",
            "sales.csv",
        ]);

        let msg = wrk.stderr_on_error(&mut cmd);
        // Assert the substantive complaint only. Polars used to wrap this in an
        // `ExprContext { .. expr: <rendered expr> }` envelope and no longer does;
        // the envelope and the expression rendering are upstream cosmetics.
        let expected_msg = r#"aggregation 'item' expected no or a single value, got 2 values"#;
        assert!(
            msg.contains(expected_msg),
            "expected {expected_msg:?} in stderr, got: {msg}"
        );
    }
);

// Test pivot with sorted columns
pivotp_test!(
    pivotp_sort_columns,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--sort-columns",
            "--agg",
            "first",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "A", "B"], // Columns will be sorted alphabetically
            svec!["2023-01-01", "100", "150"],
            svec!["2023-01-02", "300", "250"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with maintain-order flag
pivotp_maintain_order_test!(
    pivotp_maintain_order,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--maintain-order",
            "--agg",
            "first",
            "sales_maintain_order.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "C", "D", "B", "A"],
            svec!["2023-01-01", "100", "200", "150", "100"],
            svec!["2023-01-02", "400", "450", "250", "300"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with maintain-order flag
pivotp_maintain_order_test!(
    pivotp_maintain_order_and_sort_columns,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--maintain-order",
            "--sort-columns",
            "--agg",
            "first",
            "sales_maintain_order.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "A", "B", "C", "D"],
            svec!["2023-01-01", "100", "150", "100", "200"],
            svec!["2023-01-02", "300", "250", "400", "450"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with custom column separator
pivotp_test!(
    pivotp_col_separator,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--col-separator",
            "::",
            "--agg",
            "first",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "A", "B"],
            svec!["2023-01-01", "100", "150"],
            svec!["2023-01-02", "300", "250"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with custom delimiter
pivotp_test!(
    pivotp_delimiter,
    |wrk: Workdir, mut cmd: process::Command| {
        // Create data with semicolon delimiter
        let sales = vec![
            svec!["date;product;region;sales"],
            svec!["2023-01-01;A;North;100"],
            svec!["2023-01-01;B;North;150"],
        ];
        wrk.create("sales_semicolon.csv", sales);

        cmd.args([
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--delimiter",
            ";",
            "sales_semicolon.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![svec!["date;A;B"], svec!["2023-01-01;100;150"]];
        assert_eq!(got, expected);
    }
);

// Test pivot with no explicit index (uses remaining columns)
pivotp_test!(
    pivotp_no_index,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(["product", "--values", "sales", "--agg", "sum", "sales.csv"]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "region", "A", "B"],
            svec!["2023-01-01", "North", "100", "150"],
            svec!["2023-01-01", "South", "200", "0"],
            svec!["2023-01-02", "South", "0", "250"],
            svec!["2023-01-02", "North", "300", "350"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with multiple on-cols
pivotp_test!(
    pivotp_multi_on_cols,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product,region", // Multiple on-cols
            "--index",
            "date",
            "--values",
            "sales",
            "--agg",
            "sum",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec![
                "date",
                "{\"A\",\"North\"}",
                "{\"B\",\"North\"}",
                "{\"A\",\"South\"}",
                "{\"B\",\"South\"}"
            ],
            svec!["2023-01-01", "100", "150", "200", "0"],
            svec!["2023-01-02", "300", "350", "0", "250"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with multiple value columns
pivotp_test!(
    pivotp_multi_values,
    |wrk: Workdir, mut cmd: process::Command| {
        // Create test data with multiple value columns
        let sales_multi = vec![
            svec!["date", "product", "region", "sales", "quantity"],
            svec!["2023-01-01", "A", "North", "100", "10"],
            svec!["2023-01-01", "B", "North", "150", "15"],
            svec!["2023-01-01", "A", "South", "200", "20"],
            svec!["2023-01-02", "B", "South", "250", "25"],
            svec!["2023-01-02", "A", "North", "300", "30"],
            svec!["2023-01-02", "B", "North", "350", "35"],
        ];
        wrk.create("sales_multi.csv", sales_multi);

        cmd.args([
            "product",
            "--index",
            "date",
            "--values",
            "sales,quantity", // Multiple value columns
            "--agg",
            "sum",
            "sales_multi.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "sales_A", "sales_B", "quantity_A", "quantity_B"],
            svec!["2023-01-01", "300", "150", "30", "15"],
            svec!["2023-01-02", "300", "600", "30", "60"],
        ];
        assert_eq!(got, expected);
    }
);

pivotp_test!(
    pivotp_multi_values_custom_col_separator,
    |wrk: Workdir, mut cmd: process::Command| {
        let sales_multi = vec![
            svec!["date", "product", "region", "sales", "quantity"],
            svec!["2023-01-01", "A", "North", "100", "10"],
            svec!["2023-01-01", "B", "North", "150", "15"],
            svec!["2023-01-01", "A", "South", "200", "20"],
            svec!["2023-01-02", "B", "South", "250", "25"],
            svec!["2023-01-02", "A", "North", "300", "30"],
            svec!["2023-01-02", "B", "North", "350", "35"],
        ];
        wrk.create("sales_multi.csv", sales_multi);

        cmd.args([
            "product",
            "--index",
            "date",
            "--values",
            "sales,quantity",
            "--agg",
            "sum",
            "--col-separator",
            "<->",
            "sales_multi.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec![
                "date",
                "sales<->A",
                "sales<->B",
                "quantity<->A",
                "quantity<->B"
            ],
            svec!["2023-01-01", "300", "150", "30", "15"],
            svec!["2023-01-02", "300", "600", "30", "60"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with try-parsedates flag
pivotp_test!(
    pivotp_try_parsedates,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--try-parsedates",
            "--agg",
            "sum",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "A", "B"],
            svec!["2023-01-01", "300", "150"],
            svec!["2023-01-02", "300", "600"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with decimal comma
pivotp_test!(
    pivotp_decimal_comma,
    |wrk: Workdir, mut cmd: process::Command| {
        // Create data with decimal commas
        let sales_decimal = vec![
            svec!["date", "product", "region", "sales"],
            svec!["2023-01-01", "A", "North", "100,50"],
            svec!["2023-01-01", "B", "North", "150,75"],
        ];
        wrk.create_with_delim("sales_decimal.csv", sales_decimal, b';');

        cmd.args([
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--agg",
            "mean",
            "--decimal-comma",
            "--delimiter",
            ";",
            "sales_decimal.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![svec!["date;A;B"], svec!["2023-01-01;100.5;150.75"]];
        assert_eq!(got, expected);
    }
);

// Test pivot with validation
pivotp_test!(
    pivotp_validate,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--validate",
            "sales.csv",
        ]);

        let msg = wrk.stderr_on_success(&mut cmd);
        let expected_msg =
            "Info: Using Sum for \"sales\"\nPivot on-column cardinality:\n  product: 2\n(2, 3)\n";
        assert_eq!(msg, expected_msg);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "A", "B"],
            svec!["2023-01-01", "300", "150"],
            svec!["2023-01-02", "300", "600"],
        ];
        assert_eq!(got, expected);
    }
);

// #4589 regression: the column cap must fire WITHOUT --validate. Before the fix the cap was
// gated behind that opt-in flag, so the default path grew memory until the OS killed it.
pivotp_test!(
    pivotp_max_columns_exceeded,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--max-columns",
            "1",
            "sales.csv",
        ]);

        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(
            stderr.contains("exceeding the --max-columns limit"),
            "expected the cap to fire without --validate, got: {stderr}"
        );
    }
);

// The escape hatch: a deliberately wide pivot stays possible. Before #4589 this was expressed
// by simply omitting --validate, so without `0` the fix would remove a capability.
pivotp_test!(
    pivotp_max_columns_zero_unlimited,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--max-columns",
            "0",
            "sales.csv",
        ]);

        let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
        // Values are smart-agg'd (--agg defaults to smart -> Sum here); what this test pins is that
        // --max-columns 0 leaves the result identical to an uncapped pivot.
        let expected = vec![
            svec!["date", "A", "B"],
            svec!["2023-01-01", "300", "150"],
            svec!["2023-01-02", "300", "600"],
        ];
        assert_eq!(got, expected);
    }
);

// --validate still reports, and the cap still aborts, and the report comes first.
pivotp_test!(
    pivotp_max_columns_with_validate,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--validate",
            "--max-columns",
            "1",
            "sales.csv",
        ]);

        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(
            stderr.contains("Pivot on-column cardinality"),
            "--validate report should still print, got: {stderr}"
        );
        assert!(
            stderr.contains("exceeding the --max-columns limit"),
            "cap should still abort under --validate, got: {stderr}"
        );
    }
);

// The cap measures the EXACT number of distinct <on-cols> combinations, not the product of the
// per-column cardinalities. Here `product` and `region` are perfectly correlated: the product
// estimate is 2 x 2 = 4, but only 2 combinations exist. Capping on the estimate would abort a
// pivot that produces 2 columns - and because the cap is now always-on, that would break pivots
// that worked before. Both halves matter: without the --max-columns 1 run this test would pass
// even if the cap were removed entirely.
#[test]
fn pivotp_max_columns_uses_exact_count_not_estimate() {
    let wrk = Workdir::new("pivotp_max_columns_uses_exact_count_not_estimate");
    let data = vec![
        svec!["date", "product", "region", "sales"],
        svec!["2023-01-01", "A", "North", "100"],
        svec!["2023-01-02", "A", "North", "200"],
        svec!["2023-01-01", "B", "South", "150"],
        svec!["2023-01-02", "B", "South", "250"],
    ];
    wrk.create("corr.csv", data);

    let run = |max_columns: &str| {
        let mut cmd = wrk.command("pivotp");
        cmd.args([
            "product,region",
            "--index",
            "date",
            "--values",
            "sales",
            "--max-columns",
            max_columns,
            "corr.csv",
        ]);
        cmd
    };

    // 2 real columns <= 3, even though the cardinality product is 4.
    let mut ok = run("3");
    wrk.assert_success(&mut ok);

    // ...and the cap is genuinely live on the same input.
    let mut err = run("1");
    let stderr = wrk.stderr_on_error(&mut err);
    assert!(
        stderr.contains("exceeding the --max-columns limit"),
        "expected the cap to fire at 1, got: {stderr}"
    );
}

// Test smart aggregation without moarstats — CV above 100% picks Median
#[test]
fn pivotp_smart_no_moarstats() {
    let wrk = Workdir::new("pivotp_smart_no_moarstats");
    // Without moarstats, smart aggregation uses only base stats.
    // `stats` reports CV as a PERCENTAGE ((stddev / mean) * 100), so "high variability" is
    // CV > 100%, i.e. a stddev larger than the mean. These values have CV ~ 200%.
    let data = vec![
        svec!["category", "group", "value"],
        svec!["A", "X", "1"],
        svec!["A", "Y", "2"],
        svec!["B", "X", "1"],
        svec!["B", "Y", "3"],
        svec!["C", "X", "2"],
        svec!["C", "Y", "90"],
    ];
    wrk.create("spread.csv", data);

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", "spread.csv"]);
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group",
        "--index",
        "category",
        "--values",
        "value",
        "--agg",
        "smart",
        "spread.csv",
    ]);

    let stderr = wrk.stderr_on_success(&mut cmd);
    assert!(
        stderr.contains("CV > 100%") && stderr.contains("Median"),
        "Expected CV-based Median without moarstats, got: {stderr}"
    );
}

// Regression: CV is a percentage, so tightly clustered values (CV ~ 1.6%) are NOT "high
// variability". The old `cv > 1.0` cut read that as CV > 1% and sent nearly every numeric
// column to Median.
#[test]
fn pivotp_smart_low_cv_uses_sum() {
    let wrk = Workdir::new("pivotp_smart_low_cv_uses_sum");
    let data = vec![
        svec!["category", "group", "value"],
        svec!["A", "X", "100"],
        svec!["A", "Y", "102"],
        svec!["B", "X", "101"],
        svec!["B", "Y", "103"],
        svec!["C", "X", "100"],
        svec!["C", "Y", "104"],
    ];
    wrk.create("normal.csv", data);

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", "normal.csv"]);
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group",
        "--index",
        "category",
        "--values",
        "value",
        "--agg",
        "smart",
        "normal.csv",
    ]);

    let stderr = wrk.stderr_on_success(&mut cmd);
    assert!(
        stderr.contains("Using Sum for \"value\"") && !stderr.contains("CV >"),
        "Expected Sum for low-CV data, got: {stderr}"
    );
}

// Gamma(k=2) quantiles: moment skewness G1 ~ 1.28, while every earlier moarstats check stays
// quiet (excess kurtosis ~ 2.0, BC ~ 0.52, MAD/stddev ~ 0.59, CV ~ 12%) and Pearson skewness is
// only ~ 0.69 - so only the G1 check can pick Median.
const GAMMA2_VALUES: &str =
    "101.0 101.8 102.4 102.9 103.3 103.8 104.1 104.5 104.8 105.2 105.5 105.8 106.1 106.4 106.7 \
     107.0 107.3 107.5 107.8 108.1 108.4 108.7 108.9 109.2 109.5 109.7 110.0 110.3 110.6 110.8 \
     111.1 111.4 111.7 111.9 112.2 112.5 112.8 113.1 113.3 113.6 113.9 114.2 114.5 114.8 115.1 \
     115.4 115.7 116.0 116.3 116.6 116.9 117.3 117.6 117.9 118.3 118.6 119.0 119.3 119.7 120.0 \
     120.4 120.8 121.2 121.6 122.0 122.4 122.8 123.3 123.7 124.2 124.6 125.1 125.6 126.1 126.7 \
     127.2 127.8 128.4 129.0 129.6 130.3 131.0 131.7 132.5 133.3 134.2 135.1 136.1 137.1 138.3 \
     139.5 140.9 142.5 144.2 146.3 148.7 151.7 155.7 161.7 174.3";

// Gamma(k=9) quantiles: moderate skew, G1 ~ 0.61. Above viz's 0.5 "skewed" label, below the
// 1.0 bound pivotp uses to switch Sum -> Median.
const GAMMA9_VALUES: &str =
    "131.3 137.6 141.2 143.8 146.0 147.9 149.5 151.0 152.4 153.7 154.9 156.1 157.2 158.2 159.2 \
     160.2 161.2 162.1 163.0 163.9 164.7 165.5 166.4 167.2 168.0 168.8 169.5 170.3 171.1 171.8 \
     172.6 173.3 174.0 174.8 175.5 176.2 177.0 177.7 178.4 179.1 179.8 180.5 181.3 182.0 182.7 \
     183.4 184.1 184.9 185.6 186.3 187.1 187.8 188.5 189.3 190.0 190.8 191.6 192.4 193.1 193.9 \
     194.7 195.6 196.4 197.2 198.1 198.9 199.8 200.7 201.6 202.5 203.5 204.4 205.4 206.5 207.5 \
     208.6 209.7 210.8 212.0 213.2 214.4 215.7 217.1 218.5 220.0 221.6 223.2 225.0 226.9 228.9 \
     231.1 233.4 236.1 239.1 242.4 246.4 251.3 257.6 266.9 285.8";

/// Write `values` as a category/group/value CSV, run `stats --everything` +
/// `moarstats --advanced`, then smart-pivot it and return pivotp's stderr.
fn smart_pivot_with_advanced_stats(wrk: &Workdir, file: &str, values: &str) -> String {
    let csv_content = std::iter::once("category,group,value".to_string())
        .chain(values.split_whitespace().enumerate().map(|(i, v)| {
            let cat = if i % 2 == 0 { "A" } else { "B" };
            let grp = if (i / 2) % 2 == 0 { "X" } else { "Y" };
            format!("{cat},{grp},{v}")
        }))
        .collect::<Vec<_>>()
        .join("\n");
    wrk.create_from_string(file, &csv_content);

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", file]);
    wrk.assert_success(&mut stats_cmd);

    let mut moar_cmd = wrk.command("moarstats");
    moar_cmd.args(["--advanced", file]);
    wrk.assert_success(&mut moar_cmd);

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group", "--index", "category", "--values", "value", "--agg", "smart", file,
    ]);
    wrk.stderr_on_success(&mut cmd)
}

// The old check read `stats.skewness`, which is Bowley's quartile skewness (bounded to [-1, 1]),
// against |skew| > 2 - so it could never fire. Moment skewness G1 replaces it.
#[test]
fn pivotp_smart_moarstats_moment_skewness() {
    let wrk = Workdir::new("pivotp_smart_moarstats_moment_skewness");
    let stderr = smart_pivot_with_advanced_stats(&wrk, "skewed.csv", GAMMA2_VALUES);
    assert!(
        stderr.contains("moment skewness") && stderr.contains("Median"),
        "Expected G1-based Median for highly skewed data, got: {stderr}"
    );
}

#[test]
fn pivotp_smart_moarstats_moderate_skew_uses_sum() {
    let wrk = Workdir::new("pivotp_smart_moarstats_moderate_skew_uses_sum");
    let stderr = smart_pivot_with_advanced_stats(&wrk, "moderate.csv", GAMMA9_VALUES);
    assert!(
        stderr.contains("Using Sum for \"value\""),
        "Expected Sum for moderately skewed data (G1 ~ 0.61 < 1.0), got: {stderr}"
    );
}

// A value column holding one distinct value gets First - never Item, which fails on a pivot
// cell with two rows. That also covers a HyperLogLog (approximate) cardinality of 1.
#[test]
fn pivotp_smart_single_value_approx_cardinality_uses_first() {
    let wrk = Workdir::new("pivotp_smart_single_value_approx_cardinality");
    let data = vec![
        svec!["category", "group", "value"],
        svec!["A", "X", "7"],
        svec!["A", "Y", "7"],
        svec!["B", "X", "7"],
        svec!["B", "Y", "7"],
    ];
    wrk.create("constant.csv", data);

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args([
        "--cardinality",
        "--cardinality-method",
        "approx",
        "--stats-jsonl",
        "constant.csv",
    ]);
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group",
        "--index",
        "category",
        "--values",
        "value",
        "--agg",
        "smart",
        "constant.csv",
    ]);
    let stderr = wrk.stderr_on_success(&mut cmd);
    assert!(
        stderr.contains("contains only one value, using First"),
        "Expected First for a single-value column, got: {stderr}"
    );
}

// One distinct value does not mean one row per pivot cell: (A, X) has two identical rows, which
// `item` rejected.
#[test]
fn pivotp_smart_single_value_duplicate_rows() {
    let wrk = Workdir::new("pivotp_smart_single_value_duplicate_rows");
    let data = vec![
        svec!["category", "group", "value"],
        svec!["A", "X", "7"],
        svec!["A", "X", "7"],
        svec!["A", "Y", "7"],
        svec!["B", "X", "7"],
    ];
    wrk.create("constant.csv", data);

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group",
        "--index",
        "category",
        "--values",
        "value",
        "--agg",
        "smart",
        "constant.csv",
    ]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["category", "X", "Y"],
        svec!["A", "7", "7"],
        svec!["B", "7", ""],
    ];
    assert_eq!(got, expected);
}

// An explicit --delimiter on a .tsv: the extension wins for the Polars reader (as in sqlp and
// joinp), so the stats lookup must use the same tab - otherwise smart-agg finds no stats for the
// value column and silently falls back to First, dropping the repeated cell values.
#[test]
fn pivotp_tsv_explicit_comma_delimiter_smart_agg() {
    let wrk = Workdir::new("pivotp_tsv_explicit_comma_delimiter_smart_agg");
    wrk.create_from_string("sales.tsv", &sales_with_delim("\t"));

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "product",
        "--index",
        "date",
        "--values",
        "sales",
        "--delimiter",
        ",",
        "sales.tsv",
    ]);
    let output = cmd.output().unwrap();
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Using Sum for \"sales\""),
        "smart-agg must see the value column's stats, got: {stderr}"
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "date,A,B\n2023-01-01,300,150\n2023-01-02,300,600\n"
    );
}

// --infer-len 0 means "scan the whole file". Polars reads Some(0) as "infer nothing" and types
// every column as String, so `--agg sum` used to fail with "sum not supported for dtype str".
#[test]
fn pivotp_infer_len_zero_keeps_numeric_types() {
    let wrk = setup("pivotp_infer_len_zero_keeps_numeric_types");
    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "product",
        "--index",
        "date",
        "--values",
        "sales",
        "--agg",
        "sum",
        "--infer-len",
        "0",
        "sales.csv",
    ]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["date", "A", "B"],
        svec!["2023-01-01", "300", "150"],
        svec!["2023-01-02", "300", "600"],
    ];
    assert_eq!(got, expected);
}

/// The `setup` sales data, joined with `delim`.
fn sales_with_delim(delim: &str) -> String {
    [
        "date,product,region,sales",
        "2023-01-01,A,North,100",
        "2023-01-01,B,North,150",
        "2023-01-01,A,South,200",
        "2023-01-02,B,South,250",
        "2023-01-02,A,North,300",
        "2023-01-02,B,North,350",
    ]
    .iter()
    .map(|l| l.replace(',', delim))
    .collect::<Vec<_>>()
    .join("\n")
}

// A .tsv input needs no --delimiter: the extension decides, as it does for the stats cache that
// smart-agg reads. Previously the Polars reader used ',' and found one combined column.
#[test]
fn pivotp_tsv_input_without_delimiter() {
    let wrk = Workdir::new("pivotp_tsv_input_without_delimiter");
    wrk.create_from_string("sales.tsv", &sales_with_delim("\t"));

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "product",
        "--index",
        "date",
        "--values",
        "sales",
        "sales.tsv",
    ]);
    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "pivotp on .tsv failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    // stdout keeps the qsv default (','), as in sqlp/joinp: the output delimiter comes from
    // --delimiter or the --output extension, never from the input's extension.
    assert_eq!(stdout, "date,A,B\n2023-01-01,300,150\n2023-01-02,300,600\n");
    assert!(
        stderr.contains("Using Sum for \"sales\""),
        "smart-agg should see the real columns, got: {stderr}"
    );
}

#[test]
fn pivotp_qsv_default_delimiter_env() {
    let wrk = Workdir::new("pivotp_qsv_default_delimiter_env");
    wrk.create_from_string("sales.txt", &sales_with_delim(";"));

    let mut cmd = wrk.command("pivotp");
    cmd.env("QSV_DEFAULT_DELIMITER", ";").args([
        "product",
        "--index",
        "date",
        "--values",
        "sales",
        "--agg",
        "sum",
        "sales.txt",
    ]);
    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "pivotp with QSV_DEFAULT_DELIMITER failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "date;A;B\n2023-01-01;300;150\n2023-01-02;300;600\n"
    );
}

// --decimal-comma needs a non-comma delimiter. A .tsv already has one, so the check must look
// at the RESOLVED delimiter; it used to see the ',' default and refuse.
#[test]
fn pivotp_decimal_comma_tsv_without_delimiter() {
    let wrk = Workdir::new("pivotp_decimal_comma_tsv_without_delimiter");
    wrk.create_from_string(
        "prices.tsv",
        "date\tproduct\tprice\n2023-01-01\tA\t1,50\n2023-01-01\tA\t2,25\n2023-01-01\tB\t3,00\n",
    );

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "product",
        "--index",
        "date",
        "--values",
        "price",
        "--agg",
        "sum",
        "--decimal-comma",
        "prices.tsv",
    ]);
    let output = cmd.output().unwrap();
    assert!(
        output.status.success(),
        "pivotp --decimal-comma on .tsv failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "date,A,B\n2023-01-01,3.75,3.0\n"
    );
}

// The output delimiter follows the --output extension.
#[test]
fn pivotp_output_tsv_extension() {
    let wrk = setup("pivotp_output_tsv_extension");
    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "product",
        "--index",
        "date",
        "--values",
        "sales",
        "--agg",
        "sum",
        "--output",
        "out.tsv",
        "sales.csv",
    ]);
    wrk.assert_success(&mut cmd);
    let got = wrk.read_to_string("out.tsv").unwrap();
    assert_eq!(
        got,
        "date\tA\tB\n2023-01-01\t300\t150\n2023-01-02\t300\t600\n"
    );
}

// Test smart aggregation with moarstats — high kurtosis should pick Median
#[test]
fn pivotp_smart_moarstats_high_kurtosis() {
    let wrk = Workdir::new("pivotp_smart_moarstats_kurtosis");
    // Heavy-tailed data: mostly clustered values with extreme outliers
    // This creates leptokurtic distribution (kurtosis > 3)
    let mut data = vec![svec!["category", "group", "value"]];
    for i in 0..40 {
        let cat = if i % 2 == 0 { "A" } else { "B" };
        let grp = if i % 3 == 0 { "X" } else { "Y" };
        // Most values near 100, but a few extreme outliers
        let val = match i {
            0 => "1000",
            1 => "-500",
            38 => "2000",
            39 => "-800",
            _ => "100",
        };
        data.push(svec![cat, grp, val]);
    }
    wrk.create("kurtosis.csv", data);

    // Run stats --everything first
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", "kurtosis.csv"]);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats --advanced to generate kurtosis and other advanced statistics
    let mut moar_cmd = wrk.command("moarstats");
    moar_cmd.args(["--advanced", "kurtosis.csv"]);
    wrk.assert_success(&mut moar_cmd);

    // Run pivotp with smart agg — kurtosis should trigger Median
    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group",
        "--index",
        "category",
        "--values",
        "value",
        "--agg",
        "smart",
        "kurtosis.csv",
    ]);

    let stderr = wrk.stderr_on_success(&mut cmd);
    // Should use Median due to moarstats detecting heavy tails or outliers
    assert!(
        stderr.contains("Median"),
        "Expected Median for heavy-tailed data with moarstats, got: {stderr}"
    );
}

// Test smart aggregation with moarstats — bimodal data should pick Len
#[test]
fn pivotp_smart_moarstats_bimodal() {
    let wrk = Workdir::new("pivotp_smart_moarstats_bimodal");
    // Bimodal data: two distinct clusters
    let mut data = vec![svec!["category", "group", "value"]];
    for i in 0..60 {
        let cat = if i % 3 == 0 {
            "A"
        } else if i % 3 == 1 {
            "B"
        } else {
            "C"
        };
        let grp = if i % 2 == 0 { "X" } else { "Y" };
        // Two clusters: values near 10 and values near 1000
        let val = if i < 30 { "10" } else { "1000" };
        data.push(svec![cat, grp, val]);
    }
    wrk.create("bimodal.csv", data);

    // Run stats --everything
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", "bimodal.csv"]);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats --advanced so bimodality_coefficient is available
    let mut moar_cmd = wrk.command("moarstats");
    moar_cmd.args(["--advanced", "bimodal.csv"]);
    wrk.assert_success(&mut moar_cmd);

    // Run pivotp with smart agg
    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group",
        "--index",
        "category",
        "--values",
        "value",
        "--agg",
        "smart",
        "bimodal.csv",
    ]);

    let stderr = wrk.stderr_on_success(&mut cmd);
    // Bimodal data: moarstats --advanced computes bimodality_coefficient >= 0.555,
    // the bimodal branch fires first and picks Len (central tendency is misleading).
    // If other checks fire first (e.g., high CV or outlier fraction), Median is also valid.
    assert!(
        stderr.contains("Len") || stderr.contains("Median"),
        "Expected Len or Median for bimodal data with moarstats, got: {stderr}"
    );
}

// Skewed UNIMODAL data can push the bimodality coefficient past 0.555 too, but it is
// leptokurtic; smart agg must not mistake it for bimodal and fall back to Len.
#[test]
fn pivotp_smart_moarstats_skewed_unimodal_not_bimodal() {
    let wrk = Workdir::new("pivotp_smart_moarstats_skewed_unimodal");
    // Exponential-distribution quantiles: moment skewness ~1.9, excess kurtosis ~4.1,
    // bimodality coefficient ~0.60.
    let n = 120;
    let mut data = vec![svec!["category", "group", "value"]];
    for i in 0..n {
        let cat = ["A", "B", "C"][i % 3];
        let grp = if i % 2 == 0 { "X" } else { "Y" };
        #[allow(clippy::cast_precision_loss)]
        let u = (i as f64 + 0.5) / n as f64;
        let val = format!("{:.4}", -100.0 * (1.0 - u).ln());
        data.push(vec![cat.to_string(), grp.to_string(), val]);
    }
    wrk.create("skewed.csv", data);

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", "skewed.csv"]);
    wrk.assert_success(&mut stats_cmd);

    let mut moar_cmd = wrk.command("moarstats");
    moar_cmd.args(["--advanced", "skewed.csv"]);
    wrk.assert_success(&mut moar_cmd);

    // Guard the fixture itself: BC must clear 0.555 with positive excess kurtosis,
    // otherwise this test would pass without exercising the kurtosis guard.
    let stats: Vec<Vec<String>> = wrk.read_stdout(&mut {
        let mut c = wrk.command("select");
        c.args(["field,bimodality_coefficient,kurtosis", "skewed.stats.csv"]);
        c
    });
    let row = stats.iter().find(|r| r[0] == "value").unwrap();
    let bc: f64 = row[1].parse().unwrap();
    let kurt: f64 = row[2].parse().unwrap();
    assert!(
        bc >= 0.555 && kurt > 0.0,
        "fixture drifted: bc={bc}, kurtosis={kurt}"
    );

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group",
        "--index",
        "category",
        "--values",
        "value",
        "--agg",
        "smart",
        "skewed.csv",
    ]);
    let stderr = wrk.stderr_on_success(&mut cmd);
    assert!(
        !stderr.contains("Bimodal") && !stderr.contains("using Len"),
        "skewed unimodal data was treated as bimodal: {stderr}"
    );
}

// Test smart aggregation with moarstats — data with many outliers should pick Median
#[test]
fn pivotp_smart_moarstats_outliers() {
    let wrk = Workdir::new("pivotp_smart_moarstats_outliers");
    // Data with >15% outliers — write CSV directly to avoid svec! lifetime issues
    let csv_content = std::iter::once("category,group,value".to_string())
        .chain((0..40).map(|i| {
            let cat = if i % 2 == 0 { "A" } else { "B" };
            let grp = if i % 2 == 0 { "X" } else { "Y" };
            let val = if i < 8 {
                10000 + i * 1000
            } else {
                50 + (i % 10)
            };
            format!("{cat},{grp},{val}")
        }))
        .collect::<Vec<_>>()
        .join("\n");
    wrk.create_from_string("outliers.csv", &csv_content);

    // Run stats --everything
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", "outliers.csv"]);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats
    let mut moar_cmd = wrk.command("moarstats");
    moar_cmd.args(["outliers.csv"]);
    wrk.assert_success(&mut moar_cmd);

    // Run pivotp with smart agg
    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group",
        "--index",
        "category",
        "--values",
        "value",
        "--agg",
        "smart",
        "outliers.csv",
    ]);

    let stderr = wrk.stderr_on_success(&mut cmd);
    // Should use Median due to outlier contamination
    assert!(
        stderr.contains("Median"),
        "Expected Median for outlier-heavy data with moarstats, got: {stderr}"
    );
}

// Test smart aggregation with moarstats — MAD/stddev ratio < 0.5 should pick Median
#[test]
fn pivotp_smart_moarstats_mad_stddev_ratio() {
    let wrk = Workdir::new("pivotp_smart_mad_stddev");
    // Data where a few large outliers inflate stddev far beyond MAD.
    // Most values cluster near 50, but a few extreme values push stddev up
    // while MAD stays low because the median neighborhood is tight.
    let csv_content = std::iter::once("category,group,value".to_string())
        .chain((0..50).map(|i| {
            let cat = if i % 2 == 0 { "A" } else { "B" };
            let grp = if i % 2 == 0 { "X" } else { "Y" };
            let val = match i {
                0 => 5000,
                1 => 4000,
                2 => 3000,
                _ => 50 + (i % 5),
            };
            format!("{cat},{grp},{val}")
        }))
        .collect::<Vec<_>>()
        .join("\n");
    wrk.create_from_string("mad_stddev.csv", &csv_content);

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", "mad_stddev.csv"]);
    wrk.assert_success(&mut stats_cmd);

    let mut moar_cmd = wrk.command("moarstats");
    moar_cmd.args(["mad_stddev.csv"]);
    wrk.assert_success(&mut moar_cmd);

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group",
        "--index",
        "category",
        "--values",
        "value",
        "--agg",
        "smart",
        "mad_stddev.csv",
    ]);

    let stderr = wrk.stderr_on_success(&mut cmd);
    // Should use Median — outliers inflate stddev but MAD stays robust
    assert!(
        stderr.contains("Median"),
        "Expected Median for data with low MAD/stddev ratio, got: {stderr}"
    );
}

// Test smart aggregation with moarstats — median/mean ratio far from 1.0 should pick Median
#[test]
fn pivotp_smart_moarstats_median_mean_divergence() {
    let wrk = Workdir::new("pivotp_smart_median_mean");
    // Right-skewed data: many small values, a few large ones.
    // Mean gets pulled up by the large values while median stays low,
    // producing a median/mean ratio well below 0.7.
    let csv_content = std::iter::once("category,group,value".to_string())
        .chain((0..60).map(|i| {
            let cat = if i % 3 == 0 {
                "A"
            } else if i % 3 == 1 {
                "B"
            } else {
                "C"
            };
            let grp = if i % 2 == 0 { "X" } else { "Y" };
            // 50 values at 10-15, 10 values at 500-600
            let val = if i < 50 { 10 + (i % 6) } else { 500 + i * 10 };
            format!("{cat},{grp},{val}")
        }))
        .collect::<Vec<_>>()
        .join("\n");
    wrk.create_from_string("median_mean.csv", &csv_content);

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", "median_mean.csv"]);
    wrk.assert_success(&mut stats_cmd);

    let mut moar_cmd = wrk.command("moarstats");
    moar_cmd.args(["median_mean.csv"]);
    wrk.assert_success(&mut moar_cmd);

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group",
        "--index",
        "category",
        "--values",
        "value",
        "--agg",
        "smart",
        "median_mean.csv",
    ]);

    let stderr = wrk.stderr_on_success(&mut cmd);
    // Should use Median — mean and median diverge significantly
    assert!(
        stderr.contains("Median"),
        "Expected Median for data with divergent mean/median, got: {stderr}"
    );
}

// Test smart aggregation with moarstats — high quartile coefficient of dispersion
#[test]
fn pivotp_smart_moarstats_quartile_dispersion() {
    let wrk = Workdir::new("pivotp_smart_qcd");
    // Data with wide IQR relative to the quartile sum.
    // Spread values broadly so Q3-Q1 is large relative to Q3+Q1 (QCD > 0.5).
    let csv_content = std::iter::once("category,group,value".to_string())
        .chain((0..50).map(|i| {
            let cat = if i % 2 == 0 { "A" } else { "B" };
            let grp = if i % 2 == 0 { "X" } else { "Y" };
            // Uniform-ish distribution from 1 to 1000
            let val = 1 + i * 20;
            format!("{cat},{grp},{val}")
        }))
        .collect::<Vec<_>>()
        .join("\n");
    wrk.create_from_string("qcd.csv", &csv_content);

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", "qcd.csv"]);
    wrk.assert_success(&mut stats_cmd);

    let mut moar_cmd = wrk.command("moarstats");
    moar_cmd.args(["qcd.csv"]);
    wrk.assert_success(&mut moar_cmd);

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group", "--index", "category", "--values", "value", "--agg", "smart", "qcd.csv",
    ]);

    let stderr = wrk.stderr_on_success(&mut cmd);
    // Should use Median — high quartile-based dispersion
    assert!(
        stderr.contains("Median"),
        "Expected Median for data with high quartile coefficient of dispersion, got: {stderr}"
    );
}

// Test smart aggregation — mixed-sign data should pick Mean (avoid Sum cancellation)
#[test]
fn pivotp_smart_mixed_sign() {
    let wrk = Workdir::new("pivotp_smart_mixed_sign");
    // Data with substantial negative and positive values (both >20%).
    // Sum would cancel out and be misleading.
    let csv_content = std::iter::once("category,group,value".to_string())
        .chain((0..40).map(|i| {
            let cat = if i % 2 == 0 { "A" } else { "B" };
            let grp = if i % 2 == 0 { "X" } else { "Y" };
            // ~50% negative, ~50% positive; mean ≈ 0 so CV is NaN,
            // which won't preempt the mixed-sign check
            let val = if i < 20 {
                -(10 + (i % 5))
            } else {
                10 + (i % 5)
            };
            format!("{cat},{grp},{val}")
        }))
        .collect::<Vec<_>>()
        .join("\n");
    wrk.create_from_string("mixed_sign.csv", &csv_content);

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", "mixed_sign.csv"]);
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group",
        "--index",
        "category",
        "--values",
        "value",
        "--agg",
        "smart",
        "mixed_sign.csv",
    ]);

    let stderr = wrk.stderr_on_success(&mut cmd);
    // Should use Mean — mixed-sign data makes Sum cancel out
    assert!(
        stderr.contains("Mean") || stderr.contains("Mixed-sign"),
        "Expected Mean for mixed-sign data, got: {stderr}"
    );
}

// Test smart aggregation — complex multimodal (>10 modes) should pick Len
// mode_count comes from `stats --everything` (or `--mode`). We also run
// moarstats so that pivotp reads the existing cache rather than re-running
// stats with FrequencyForceStats (which only produces streaming stats and
// would not include mode_count).
#[test]
fn pivotp_smart_multimodal() {
    let wrk = Workdir::new("pivotp_smart_multimodal");
    // Data where many values appear with equal frequency, creating >10 modes.
    // Each value appears exactly twice so all are tied as modes (mode_count = 20).
    let csv_content = std::iter::once("category,group,value".to_string())
        .chain((0..40).map(|i| {
            let cat = if i % 2 == 0 { "A" } else { "B" };
            let grp = if i % 2 == 0 { "X" } else { "Y" };
            // 20 distinct values, each appearing twice -> mode_count = 20
            let val = (i % 20) * 100;
            format!("{cat},{grp},{val}")
        }))
        .collect::<Vec<_>>()
        .join("\n");
    wrk.create_from_string("multimodal.csv", &csv_content);

    // stats --everything produces mode_count; moarstats prevents pivotp
    // from overwriting the cache with a streaming-only re-run
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", "multimodal.csv"]);
    wrk.assert_success(&mut stats_cmd);

    let mut moar_cmd = wrk.command("moarstats");
    moar_cmd.args(["multimodal.csv"]);
    wrk.assert_success(&mut moar_cmd);

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group",
        "--index",
        "category",
        "--values",
        "value",
        "--agg",
        "smart",
        "multimodal.csv",
    ]);

    let stderr = wrk.stderr_on_success(&mut cmd);
    // Should use Len — too many modes means central tendency is misleading
    assert!(
        stderr.contains("Len") || stderr.contains("modes"),
        "Expected Len for complex multimodal data, got: {stderr}"
    );
}

// Test smart aggregation — Date/DateTime with >50% NULLs should pick Len
// Requires stats --infer-dates + moarstats to preserve Date type in cache
#[test]
fn pivotp_smart_date_sparse() {
    let wrk = Workdir::new("pivotp_smart_date_sparse");
    // Date column with >50% NULL values
    let csv_content = [
        "category,group,date_val",
        "A,X,2024-01-01",
        "A,Y,",
        "B,X,",
        "B,Y,2024-02-15",
        "C,X,",
        "C,Y,",
        "A,X,",
        "B,X,",
        "C,X,",
        "A,Y,2024-03-10",
    ]
    .join("\n");
    wrk.create_from_string("date_sparse.csv", &csv_content);

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", "--infer-dates", "date_sparse.csv"]);
    wrk.assert_success(&mut stats_cmd);

    let mut moar_cmd = wrk.command("moarstats");
    moar_cmd.args(["date_sparse.csv"]);
    wrk.assert_success(&mut moar_cmd);

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group",
        "--index",
        "category",
        "--values",
        "date_val",
        "--agg",
        "smart",
        "--try-parsedates",
        "date_sparse.csv",
    ]);

    let stderr = wrk.stderr_on_success(&mut cmd);
    // Should use Len — >50% NULLs in date column
    assert!(
        stderr.contains("Len") || stderr.contains("NULL"),
        "Expected Len for sparse date column, got: {stderr}"
    );
}

// Test smart aggregation — ascending Date values should pick Last
// Requires stats --infer-dates + moarstats to preserve Date type in cache
#[test]
fn pivotp_smart_date_ascending() {
    let wrk = Workdir::new("pivotp_smart_date_ascending");
    // Ascending date values — Last gives the most recent entry
    let csv_content = [
        "category,group,date_val",
        "A,X,2024-01-01",
        "A,Y,2024-01-15",
        "B,X,2024-02-01",
        "B,Y,2024-02-15",
        "C,X,2024-03-01",
        "C,Y,2024-03-15",
        "A,X,2024-04-01",
        "B,X,2024-04-15",
        "C,X,2024-05-01",
        "A,Y,2024-05-15",
    ]
    .join("\n");
    wrk.create_from_string("date_ascending.csv", &csv_content);

    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", "--infer-dates", "date_ascending.csv"]);
    wrk.assert_success(&mut stats_cmd);

    let mut moar_cmd = wrk.command("moarstats");
    moar_cmd.args(["date_ascending.csv"]);
    wrk.assert_success(&mut moar_cmd);

    let mut cmd = wrk.command("pivotp");
    cmd.args([
        "group",
        "--index",
        "category",
        "--values",
        "date_val",
        "--agg",
        "smart",
        "--try-parsedates",
        "date_ascending.csv",
    ]);

    let stderr = wrk.stderr_on_success(&mut cmd);
    // Should use Last — ascending date values, most recent is most useful
    assert!(
        stderr.contains("Last") || stderr.contains("Ascending"),
        "Expected Last for ascending date column, got: {stderr}"
    );
}

// Test pivot with custom infer length
pivotp_test!(
    pivotp_infer_len,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--infer-len",
            "5",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "A", "B"],
            svec!["2023-01-01", "300", "150"],
            svec!["2023-01-02", "300", "600"],
        ];
        assert_eq!(got, expected);
    }
);

// Test grand total with single index
pivotp_test!(
    pivotp_grand_total,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "sum",
            "--grand-total",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["region", "A", "B"],
            svec!["North", "400", "500"],
            svec!["South", "200", "250"],
            svec!["Grand Total", "600", "750"],
        ];
        assert_eq!(got, expected);
    }
);

// Test subtotal with two index columns
pivotp_test!(
    pivotp_subtotal,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "date,region",
            "--values",
            "sales",
            "--agg",
            "sum",
            "--subtotal",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "region", "A", "B"],
            svec!["2023-01-01", "North", "100", "150"],
            svec!["2023-01-01", "South", "200", "0"],
            svec!["2023-01-01", "Total", "300", "150"],
            svec!["2023-01-02", "North", "300", "350"],
            svec!["2023-01-02", "South", "0", "250"],
            svec!["2023-01-02", "Total", "300", "600"],
        ];
        assert_eq!(got, expected);
    }
);

// Test grand total and subtotal together
pivotp_test!(
    pivotp_grand_total_and_subtotal,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "date,region",
            "--values",
            "sales",
            "--agg",
            "sum",
            "--grand-total",
            "--subtotal",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "region", "A", "B"],
            svec!["2023-01-01", "North", "100", "150"],
            svec!["2023-01-01", "South", "200", "0"],
            svec!["2023-01-01", "Total", "300", "150"],
            svec!["2023-01-02", "North", "300", "350"],
            svec!["2023-01-02", "South", "0", "250"],
            svec!["2023-01-02", "Total", "300", "600"],
            svec!["Grand Total", "", "600", "750"],
        ];
        assert_eq!(got, expected);
    }
);

// Test subtotal with single index column should error
pivotp_test!(
    pivotp_subtotal_single_index_error,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "sum",
            "--subtotal",
            "sales.csv",
        ]);

        wrk.assert_err(&mut cmd);
    }
);

// Test grand total with custom label
pivotp_test!(
    pivotp_grand_total_custom_label,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "sum",
            "--grand-total",
            "--total-label",
            "SUM",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let last = got.last().unwrap();
        assert_eq!(last[0], "Grand SUM");
    }
);

// Test grand total with mean aggregation (totals still sum the means)
pivotp_test!(
    pivotp_grand_total_with_mean_agg,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "mean",
            "--grand-total",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let last = got.last().unwrap();
        assert_eq!(last[0], "Grand Total");
        // Grand total sums the per-region mean values.
        // Verify each value column's grand total equals the sum of the means.
        let header = &got[0];
        let data_rows = &got[1..got.len() - 1];
        for col in 1..header.len() {
            let sum_means: f64 = data_rows
                .iter()
                .map(|row| row[col].parse::<f64>().unwrap())
                .sum();
            let grand_total: f64 = last[col].parse().unwrap();
            assert!(
                (grand_total - sum_means).abs() < 1e-9,
                "Grand total for '{}' ({grand_total}) != sum of means ({sum_means})",
                header[col]
            );
        }
    }
);

// Test grand total with sort-columns
pivotp_test!(
    pivotp_grand_total_sort_columns,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "sum",
            "--grand-total",
            "--sort-columns",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        // Header should have sorted pivot columns
        assert_eq!(got[0], svec!["region", "A", "B"]);
        // Grand total should be last
        let last = got.last().unwrap();
        assert_eq!(last[0], "Grand Total");
    }
);

// Regression test for build_total_row: a non-numeric, non-index pivoted
// column (here a Date from `--try-parsedates` pivoted via `--agg first`)
// must be filled with a typed NULL in the grand-total row so the schema
// matches on vstack. Previously we inserted an empty string, which
// produced a String column that could not vstack onto the Date column.
pivotp_test!(
    pivotp_grand_total_non_numeric_value,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "product",
            "--index",
            "region",
            "--values",
            "date",
            "--agg",
            "first",
            "--try-parsedates",
            "--grand-total",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        // Header: region plus one column per product (A, B)
        assert_eq!(got[0], svec!["region", "A", "B"]);
        // Grand-total row: label in first col, empty (null) for the
        // Date-typed pivot columns.
        let last = got.last().unwrap();
        assert_eq!(last[0], "Grand Total");
        assert_eq!(last[1], "");
        assert_eq!(last[2], "");
    }
);

// Same schema-preservation guarantee for --subtotal as the grand-total
// variant above. To actually exercise build_total_row's non-numeric
// pivoted-value path under --subtotal (not just the index-cast path),
// pivot a Date-typed value column: `sales` as on-col (cardinality is
// small enough for pivot), product+region as the two index columns,
// and `date` + --try-parsedates as the Date value column.
pivotp_test!(
    pivotp_subtotal_non_numeric_value,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "sales",
            "--index",
            "product,region",
            "--values",
            "date",
            "--agg",
            "first",
            "--try-parsedates",
            "--subtotal",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        // A subtotal row should appear, with "Total" in the second index
        // column and empty (null-rendered) Date values in the pivoted
        // columns — previously this would fail with a schema mismatch
        // because we inserted an empty string into a Date column.
        let subtotal_row = got
            .iter()
            .find(|row| row.get(1).map(String::as_str) == Some("Total"))
            .expect("expected at least one subtotal row");
        for cell in &subtotal_row[2..] {
            assert_eq!(
                cell, "",
                "pivoted Date column should be empty in subtotal row"
            );
        }
    }
);

// ==================== GROUP-BY MODE TESTS ====================

macro_rules! pivotp_groupby_test {
    ($name:ident, $fun:expr_2021) => {
        mod $name {
            use std::process;

            #[allow(unused_imports)]
            use super::setup_groupby;
            use crate::workdir::Workdir;

            #[test]
            fn main() {
                let wrk = setup_groupby(stringify!($name));
                let cmd = wrk.command("pivotp");
                $fun(wrk, cmd);
            }
        }
    };
}

fn setup_groupby(name: &str) -> Workdir {
    // Data from issue #3697
    let data = vec![
        svec!["GROUP", "NAME"],
        svec!["G1", "N1"],
        svec!["G2", "N1"],
        svec!["G3", "N2"],
        svec!["G4", "N2"],
        svec!["G5", "N1"],
        svec!["G5", "N2"],
        svec!["G5", "N3"],
    ];

    let wrk = Workdir::new(name);
    wrk.create("test.csv", data);
    wrk
}

// Test basic group-by count with --agg len (the exact use case from issue #3697)
pivotp_groupby_test!(
    pivotp_groupby_basic_count,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(["--index", "GROUP", "--agg", "len", "test.csv"]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["GROUP", "count"],
            svec!["G1", "1"],
            svec!["G2", "1"],
            svec!["G3", "1"],
            svec!["G4", "1"],
            svec!["G5", "3"],
        ];
        assert_eq!(got, expected);
    }
);

// Test group-by with no --values and no explicit --agg (defaults to len/count)
pivotp_groupby_test!(
    pivotp_groupby_no_values_default_agg,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(["--index", "GROUP", "test.csv"]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        // smart agg defaults to len in group-by mode → "count" column
        assert_eq!(got[0], svec!["GROUP", "count"]);
        assert_eq!(got.len(), 6); // 5 groups + header
    }
);

// Test group-by count with grand total
pivotp_groupby_test!(
    pivotp_groupby_grand_total,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "--index",
            "GROUP",
            "--agg",
            "len",
            "--grand-total",
            "test.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["GROUP", "count"],
            svec!["G1", "1"],
            svec!["G2", "1"],
            svec!["G3", "1"],
            svec!["G4", "1"],
            svec!["G5", "3"],
            svec!["Grand Total", "7"],
        ];
        assert_eq!(got, expected);
    }
);

// Test group-by with --values on a specific column
pivotp_groupby_test!(
    pivotp_groupby_with_values,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "--index", "GROUP", "--values", "NAME", "--agg", "len", "test.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["GROUP", "NAME"],
            svec!["G1", "1"],
            svec!["G2", "1"],
            svec!["G3", "1"],
            svec!["G4", "1"],
            svec!["G5", "3"],
        ];
        assert_eq!(got, expected);
    }
);

// Test group-by sum on numeric data using the sales setup
pivotp_test!(
    pivotp_groupby_sum,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "sum",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["region", "sales"],
            svec!["North", "900"],
            svec!["South", "450"],
        ];
        assert_eq!(got, expected);
    }
);

// Test group-by with multiple index columns
pivotp_test!(
    pivotp_groupby_multi_index,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "--index",
            "date,region",
            "--values",
            "sales",
            "--agg",
            "sum",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "region", "sales"],
            svec!["2023-01-01", "North", "250"],
            svec!["2023-01-01", "South", "200"],
            svec!["2023-01-02", "South", "250"],
            svec!["2023-01-02", "North", "650"],
        ];
        assert_eq!(got, expected);
    }
);

// Test group-by with multiple value columns
pivotp_test!(
    pivotp_groupby_multi_values,
    |wrk: Workdir, mut cmd: process::Command| {
        // Create data with two numeric columns
        let data = vec![
            svec!["region", "sales", "qty"],
            svec!["North", "100", "5"],
            svec!["North", "200", "10"],
            svec!["South", "300", "15"],
        ];
        wrk.create("multi.csv", data);

        cmd.args([
            "--index",
            "region",
            "--values",
            "sales,qty",
            "--agg",
            "sum",
            "multi.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["region", "sales", "qty"],
            svec!["North", "300", "15"],
            svec!["South", "300", "15"],
        ];
        assert_eq!(got, expected);
    }
);

// Test group-by with --maintain-order
pivotp_groupby_test!(
    pivotp_groupby_maintain_order,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "--index",
            "GROUP",
            "--agg",
            "len",
            "--maintain-order",
            "test.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        // With maintain-order, groups appear in first-seen order
        assert_eq!(got[0], svec!["GROUP", "count"]);
        assert_eq!(got[1][0], "G1");
        assert_eq!(got[5][0], "G5");
        assert_eq!(got[5][1], "3");
    }
);

// Test group-by with custom total label
pivotp_groupby_test!(
    pivotp_groupby_custom_total_label,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "--index",
            "GROUP",
            "--agg",
            "len",
            "--grand-total",
            "--total-label",
            "Sum",
            "test.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let last = got.last().unwrap();
        assert_eq!(last[0], "Grand Sum");
        assert_eq!(last[1], "7");
    }
);

// Test error: group-by mode without --index
pivotp_groupby_test!(
    pivotp_groupby_no_index_error,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(["--agg", "len", "test.csv"]);

        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(
            stderr.contains("--index <cols> is required in group-by mode"),
            "Expected --index required error, got: {stderr}"
        );
    }
);

// Test error: --subtotal in group-by mode
pivotp_groupby_test!(
    pivotp_groupby_subtotal_error,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(["--index", "GROUP", "--agg", "len", "--subtotal", "test.csv"]);

        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(
            stderr.contains("--subtotal is not supported in group-by mode"),
            "Expected --subtotal error, got: {stderr}"
        );
    }
);

// Test error: --validate in group-by mode
pivotp_groupby_test!(
    pivotp_groupby_validate_error,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(["--index", "GROUP", "--agg", "len", "--validate", "test.csv"]);

        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(
            stderr.contains("--validate is not supported in group-by mode"),
            "Expected --validate error, got: {stderr}"
        );
    }
);

// Test error: --sort-columns in group-by mode
pivotp_groupby_test!(
    pivotp_groupby_sort_columns_error,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "--index",
            "GROUP",
            "--agg",
            "len",
            "--sort-columns",
            "test.csv",
        ]);

        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(
            stderr.contains("--sort-columns is not supported in group-by mode"),
            "Expected --sort-columns error, got: {stderr}"
        );
    }
);

// Test error: --agg none in group-by mode
pivotp_groupby_test!(
    pivotp_groupby_agg_none_error,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(["--index", "GROUP", "--agg", "none", "test.csv"]);

        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(
            stderr.contains("not supported in group-by mode"),
            "Expected --agg none error, got: {stderr}"
        );
    }
);

// Test error: --agg item in group-by mode
pivotp_groupby_test!(
    pivotp_groupby_agg_item_error,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(["--index", "GROUP", "--agg", "item", "test.csv"]);

        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(
            stderr.contains("--agg item is not supported in group-by mode"),
            "Expected --agg item error, got: {stderr}"
        );
    }
);

// The cap is pivot-mode only; group-by mode does not widen output per distinct value, so a
// tiny --max-columns must not leak across and reject a valid group-by.
pivotp_groupby_test!(
    pivotp_groupby_max_columns_ignored,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args([
            "--index",
            "GROUP",
            "--agg",
            "len",
            "--max-columns",
            "1",
            "test.csv",
        ]);
        wrk.assert_success(&mut cmd);
    }
);
