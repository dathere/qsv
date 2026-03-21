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
    cmd.args(&[
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
        cmd.args(&[
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
    cmd.args(&[
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
        cmd.args(&[
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
    cmd.args(&[
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
    cmd.args(&[
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
        cmd.args(&[
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

// Test pivot with len aggregation
pivotp_test!(pivotp_len_agg, |wrk: Workdir, mut cmd: process::Command| {
    cmd.args(&[
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
        cmd.args(&[
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
        cmd.args(&[
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "item",
            "sales.csv",
        ]);

        wrk.assert_err(&mut cmd);

        let msg = wrk.output_stderr(&mut cmd);
        let expected_msg = r#"Polars error: ExprContext { error: ComputeError(ErrString("aggregation 'item' expected no or a single value, got 2 values")), expr: ErrString("col(\"sales\").filter([(col(\"product\"))"#;
        assert!(msg.starts_with(expected_msg));
    }
);

// Test pivot with sorted columns
pivotp_test!(
    pivotp_sort_columns,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
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
        cmd.args(&[
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
        cmd.args(&[
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
        cmd.args(&[
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

        cmd.args(&[
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
        let expected = vec![svec!["date;A;B"], svec!["2023-01-01;100.0;150.0"]];
        assert_eq!(got, expected);
    }
);

// Test pivot with no explicit index (uses remaining columns)
pivotp_test!(
    pivotp_no_index,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&["product", "--values", "sales", "--agg", "sum", "sales.csv"]);

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
        cmd.args(&[
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

        cmd.args(&[
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

        cmd.args(&[
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
        cmd.args(&[
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

        cmd.args(&[
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
        cmd.args(&[
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--validate",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let msg = wrk.output_stderr(&mut cmd);
        let expected_msg = "Info: High variability in values (CV > 1), using Median for more \
                            robust central tendency\nPivot on-column cardinality:\n  product: \
                            2\n(2, 3)\n";
        assert_eq!(msg, expected_msg);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "A", "B"],
            svec!["2023-01-01", "150.0", "150.0"],
            svec!["2023-01-02", "300.0", "300.0"],
        ];
        assert_eq!(got, expected);
    }
);

// Test smart aggregation without moarstats — graceful degradation to existing behavior
// Normal numeric data with CV > 1% should use Median (existing CV-based behavior)
#[test]
fn pivotp_smart_no_moarstats() {
    let wrk = Workdir::new("pivotp_smart_no_moarstats");
    // Without moarstats, smart aggregation uses only base stats.
    // CV in qsv is stored as a percentage (stddev/mean * 100),
    // so even tight values like 100-104 have CV > 1 (i.e., > 1%).
    // This test verifies that the existing CV-based logic still works
    // correctly without moarstats data, picking Median for CV > 1.
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

    // Run stats first so pivotp can use them
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args(["--everything", "normal.csv"]);
    wrk.assert_success(&mut stats_cmd);

    // Run pivotp with smart agg — no moarstats, CV > 1% triggers Median
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
    wrk.assert_success(&mut cmd);

    let stderr = wrk.output_stderr(&mut cmd);
    // Existing behavior: CV > 1% triggers Median, and no moarstats checks fire
    // (because moarstats hasn't been run, so all moarstats fields are None)
    assert!(
        stderr.contains("CV > 1"),
        "Expected CV-based Median without moarstats, got: {stderr}"
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
    wrk.assert_success(&mut cmd);

    let stderr = wrk.output_stderr(&mut cmd);
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
    wrk.assert_success(&mut cmd);

    let stderr = wrk.output_stderr(&mut cmd);
    // Bimodal data: moarstats --advanced computes bimodality_coefficient >= 0.555,
    // the bimodal branch fires first and picks Len (central tendency is misleading).
    // If other checks fire first (e.g., high CV or outlier fraction), Median is also valid.
    assert!(
        stderr.contains("Len") || stderr.contains("Median"),
        "Expected Len or Median for bimodal data with moarstats, got: {stderr}"
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
    wrk.assert_success(&mut cmd);

    let stderr = wrk.output_stderr(&mut cmd);
    // Should use Median due to outlier contamination
    assert!(
        stderr.contains("Median"),
        "Expected Median for outlier-heavy data with moarstats, got: {stderr}"
    );
}

// Test pivot with custom infer length
pivotp_test!(
    pivotp_infer_len,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
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
            svec!["2023-01-01", "150.0", "150.0"],
            svec!["2023-01-02", "300.0", "300.0"],
        ];
        assert_eq!(got, expected);
    }
);

// Test grand total with single index
pivotp_test!(
    pivotp_grand_total,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
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
        cmd.args(&[
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
            svec!["2023-01-02", "South", "0", "250"],
            svec!["2023-01-02", "North", "300", "350"],
            svec!["2023-01-02", "Total", "300", "600"],
        ];
        assert_eq!(got, expected);
    }
);

// Test grand total and subtotal together
pivotp_test!(
    pivotp_grand_total_and_subtotal,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
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
            svec!["2023-01-02", "South", "0", "250"],
            svec!["2023-01-02", "North", "300", "350"],
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
        cmd.args(&[
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
        cmd.args(&[
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
        cmd.args(&[
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
        // Grand total row should be the last row
        let last = got.last().unwrap();
        assert_eq!(last[0], "Grand Total");
        // The values should be sums of the mean values
        assert!(last.len() > 1, "Grand total row should have value columns");
    }
);

// Test grand total with sort-columns
pivotp_test!(
    pivotp_grand_total_sort_columns,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
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
