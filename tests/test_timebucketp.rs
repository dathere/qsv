use std::process;

use crate::workdir::Workdir;

/// Readings from two stations, deliberately NOT sorted by `ts`.
fn readings(wrk: &Workdir) {
    wrk.create(
        "r.csv",
        vec![
            svec!["ts", "station", "temp"],
            svec!["2024-01-01 00:05:00", "A", "1"],
            svec!["2024-01-01 00:20:00", "B", "10"],
            svec!["2024-01-01 00:50:00", "A", "3"],
            svec!["2024-01-01 02:10:00", "A", "5"],
            svec!["2024-01-01 01:15:00", "B", "20"],
        ],
    );
}

fn trades(wrk: &Workdir) {
    wrk.create(
        "t.csv",
        vec![
            svec!["ts", "price"],
            svec!["2024-01-01 09:00:00", "10"],
            svec!["2024-01-01 09:30:00", "20"],
            svec!["2024-01-01 10:15:00", "30"],
            svec!["2024-01-01 12:00:00", "40"],
        ],
    );
}

fn tb(wrk: &Workdir, args: &[&str]) -> process::Command {
    let mut cmd = wrk.command("timebucketp");
    cmd.args(args);
    cmd
}

#[test]
fn timebucketp_hourly_sum_and_count_of_unsorted_input() {
    let wrk = Workdir::new("timebucketp_hourly_sum_and_count_of_unsorted_input");
    readings(&wrk);
    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "ts",
            "--every",
            "1h",
            "--agg",
            "sum:temp,len",
            "r.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["ts", "temp_sum", "count"],
        svec!["2024-01-01 00:00:00", "14", "3"],
        svec!["2024-01-01 01:00:00", "20", "1"],
        svec!["2024-01-01 02:00:00", "5", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_by_with_fill_gaps() {
    let wrk = Workdir::new("timebucketp_by_with_fill_gaps");
    readings(&wrk);
    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "ts",
            "--every",
            "1h",
            "--by",
            "station",
            "--agg",
            "sum:temp,mean:temp,len",
            "--fill-gaps",
            "r.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // the empty 01:00 window of station A: the station is kept, sum and count are 0, mean empty
    let expected = vec![
        svec!["station", "ts", "temp_sum", "temp_mean", "count"],
        svec!["A", "2024-01-01 00:00:00", "4", "2.0", "2"],
        svec!["A", "2024-01-01 01:00:00", "0", "", "0"],
        svec!["A", "2024-01-01 02:00:00", "5", "5.0", "1"],
        svec!["B", "2024-01-01 00:00:00", "10", "10.0", "1"],
        svec!["B", "2024-01-01 01:00:00", "20", "20.0", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_sorted_rejects_unsorted_input() {
    // Polars' lazy group_by_dynamic does not check sortedness without `by`, and silently builds
    // wrong windows; timebucketp must refuse instead.
    let wrk = Workdir::new("timebucketp_sorted_rejects_unsorted_input");
    readings(&wrk);
    let mut cmd = tb(&wrk, &["-i", "ts", "--every", "1h", "--sorted", "r.csv"]);
    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("not sorted by --index. Drop --sorted"),
        "stderr: {stderr}"
    );
}

#[test]
fn timebucketp_sorted_rejects_unsorted_group() {
    let wrk = Workdir::new("timebucketp_sorted_rejects_unsorted_group");
    wrk.create(
        "u.csv",
        vec![
            svec!["ts", "station", "temp"],
            svec!["2024-01-01 00:00:00", "B", "7"],
            svec!["2024-01-01 00:50:00", "A", "3"],
            svec!["2024-01-01 00:05:00", "A", "1"],
        ],
    );
    let mut cmd = tb(
        &wrk,
        &[
            "-i", "ts", "--every", "1h", "--by", "station", "--sorted", "u.csv",
        ],
    );
    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("not sorted by --index within each --by group"),
        "stderr: {stderr}"
    );
}

#[test]
fn timebucketp_sorted_accepts_interleaved_groups() {
    let wrk = Workdir::new("timebucketp_sorted_accepts_interleaved_groups");
    wrk.create(
        "i.csv",
        vec![
            svec!["ts", "station", "temp"],
            svec!["2024-01-01 00:20:00", "B", "10"],
            svec!["2024-01-01 00:05:00", "A", "1"],
            svec!["2024-01-01 01:15:00", "B", "20"],
            svec!["2024-01-01 00:50:00", "A", "3"],
            svec!["2024-01-01 02:10:00", "A", "5"],
        ],
    );
    let mut cmd = tb(
        &wrk,
        &[
            "-i", "ts", "--every", "1h", "--by", "station", "--sorted", "--agg", "sum:temp",
            "i.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["station", "ts", "temp_sum"],
        svec!["A", "2024-01-01 00:00:00", "4"],
        svec!["A", "2024-01-01 02:00:00", "5"],
        svec!["B", "2024-01-01 00:00:00", "10"],
        svec!["B", "2024-01-01 01:00:00", "20"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_rolling_is_a_trailing_window_including_the_row() {
    let wrk = Workdir::new("timebucketp_rolling_is_a_trailing_window_including_the_row");
    trades(&wrk);
    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "ts",
            "--rolling",
            "--period",
            "1h",
            "--agg",
            "mean:price,len",
            "t.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // each window is (ts - 1h, ts]
    let expected = vec![
        svec!["ts", "price_mean", "count"],
        svec!["2024-01-01 09:00:00", "10.0", "1"],
        svec!["2024-01-01 09:30:00", "15.0", "2"],
        svec!["2024-01-01 10:15:00", "25.0", "2"],
        svec!["2024-01-01 12:00:00", "40.0", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_overlapping_windows_label_right_boundaries_quantile() {
    let wrk = Workdir::new("timebucketp_overlapping_windows_label_right_boundaries_quantile");
    trades(&wrk);
    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "ts",
            "--every",
            "30m",
            "--period",
            "1h",
            "--label",
            "right",
            "--boundaries",
            "--agg",
            "q@0.5:price,max:price",
            "t.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // the empty 10:30-11:30 window is skipped
    let expected = vec![
        svec![
            "_lower_boundary",
            "_upper_boundary",
            "ts",
            "price_quantile_0.5",
            "price_max"
        ],
        svec![
            "2024-01-01 09:00:00",
            "2024-01-01 10:00:00",
            "2024-01-01 10:00:00",
            "15.0",
            "20"
        ],
        svec![
            "2024-01-01 09:30:00",
            "2024-01-01 10:30:00",
            "2024-01-01 10:30:00",
            "25.0",
            "30"
        ],
        svec![
            "2024-01-01 10:00:00",
            "2024-01-01 11:00:00",
            "2024-01-01 11:00:00",
            "30.0",
            "30"
        ],
        svec![
            "2024-01-01 11:30:00",
            "2024-01-01 12:30:00",
            "2024-01-01 12:30:00",
            "40.0",
            "40"
        ],
        svec![
            "2024-01-01 12:00:00",
            "2024-01-01 13:00:00",
            "2024-01-01 13:00:00",
            "40.0",
            "40"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_negative_offset() {
    let wrk = Workdir::new("timebucketp_negative_offset");
    trades(&wrk);
    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "ts",
            "--every",
            "1h",
            "--offset",
            "-15m",
            "--agg",
            "sum:price",
            "t.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["ts", "price_sum"],
        svec!["2024-01-01 08:45:00", "30"],
        svec!["2024-01-01 09:45:00", "30"],
        svec!["2024-01-01 11:45:00", "40"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_integer_index_fill_gaps() {
    let wrk = Workdir::new("timebucketp_integer_index_fill_gaps");
    wrk.create(
        "n.csv",
        vec![
            svec!["n", "v"],
            svec!["1", "1"],
            svec!["2", "2"],
            svec!["5", "5"],
            svec!["9", "9"],
        ],
    );
    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "n",
            "--every",
            "3i",
            "--agg",
            "sum:v",
            "--fill-gaps",
            "n.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["n", "v_sum"],
        svec!["0", "3"],
        svec!["3", "5"],
        svec!["6", "0"],
        svec!["9", "9"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_integer_index_typed_by_pschema() {
    // `schema --polars` types small non-negative integers as UInt8/UInt16, which Polars windows
    // do not accept, so timebucketp widens the index to Int64.
    let wrk = Workdir::new("timebucketp_integer_index_typed_by_pschema");
    wrk.create(
        "n.csv",
        vec![
            svec!["n", "v"],
            svec!["1", "1"],
            svec!["2", "2"],
            svec!["5", "5"],
        ],
    );
    let mut schema = wrk.command("schema");
    schema.args(["--polars", "n.csv"]);
    wrk.assert_success(&mut schema);
    let pschema = wrk.read_to_string("n.csv.pschema.json").unwrap();
    assert!(pschema.contains("UInt"), "pschema: {pschema}");

    let mut cmd = tb(
        &wrk,
        &["-i", "n", "--every", "3i", "--agg", "sum:v", "n.csv"],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["n", "v_sum"], svec!["0", "3"], svec!["3", "5"]];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_dmy_pschema_outlives_qsv_prefer_dmy() {
    // a pschema made under QSV_PREFER_DMY types the dates as Date, so they stay day-first
    // after the variable is unset (documented in USAGE); --date-format overrides the pschema.
    let wrk = Workdir::new("timebucketp_dmy_pschema_outlives_qsv_prefer_dmy");
    wrk.create(
        "d.csv",
        vec![
            svec!["ts", "v"],
            svec!["01/03/2024", "1"],
            svec!["01/04/2024", "2"],
        ],
    );
    let mut schema = wrk.command("schema");
    schema
        .env("QSV_PREFER_DMY", "1")
        .args(["--polars", "d.csv"]);
    wrk.assert_success(&mut schema);
    let pschema = wrk.read_to_string("d.csv.pschema.json").unwrap();
    assert!(pschema.contains("\"Date\""), "pschema: {pschema}");

    let mut cmd = tb(
        &wrk,
        &["-i", "ts", "--every", "1d", "--agg", "sum:v", "d.csv"],
    );
    cmd.env_remove("QSV_PREFER_DMY");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["ts", "v_sum"],
        svec!["2024-03-01", "1"],
        svec!["2024-04-01", "2"],
    ];
    assert_eq!(got, expected);

    let mut with_format = tb(
        &wrk,
        &[
            "-i",
            "ts",
            "--every",
            "1d",
            "--agg",
            "sum:v",
            "--date-format",
            "%m/%d/%Y",
            "d.csv",
        ],
    );
    with_format.env_remove("QSV_PREFER_DMY");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut with_format);
    let expected = vec![
        svec!["ts", "v_sum"],
        svec!["2024-01-03", "1"],
        svec!["2024-01-04", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_sub_microsecond_windows() {
    // text datetimes parse to microseconds unless a duration needs nanoseconds; at
    // microseconds a 100ns window selects nothing
    let wrk = Workdir::new("timebucketp_sub_microsecond_windows");
    wrk.create(
        "ns.csv",
        vec![
            svec!["ts", "v"],
            svec!["2024-01-01 00:00:00.000000001", "1"],
            svec!["2024-01-01 00:00:00.000000002", "2"],
            svec!["2024-01-01 00:00:00.000000150", "4"],
        ],
    );
    let mut cmd = tb(
        &wrk,
        &["-i", "ts", "--every", "100ns", "--agg", "sum:v", "ns.csv"],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["ts", "v_sum"],
        svec!["2024-01-01 00:00:00", "3"],
        svec!["2024-01-01 00:00:00.000000100", "4"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_ignore_errors_reads_bad_values_as_null() {
    // a value the inferred schema can't hold becomes null; its row still counts
    let wrk = Workdir::new("timebucketp_ignore_errors_reads_bad_values_as_null");
    wrk.create(
        "e.csv",
        vec![
            svec!["ts", "v"],
            svec!["2024-01-01 00:00:00", "1"],
            svec!["2024-01-01 00:10:00", "2"],
            svec!["2024-01-01 00:20:00", "abc"],
        ],
    );
    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "ts",
            "--every",
            "1h",
            "--agg",
            "sum:v,len",
            "--infer-len",
            "2",
            "--ignore-errors",
            "e.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["ts", "v_sum", "count"],
        svec!["2024-01-01 00:00:00", "3", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_weekly_from_monday_fill_gaps() {
    let wrk = Workdir::new("timebucketp_weekly_from_monday_fill_gaps");
    wrk.create(
        "d.csv",
        vec![
            svec!["d", "v"],
            svec!["2024-01-03", "1"],
            svec!["2024-01-09", "2"],
            svec!["2024-01-24", "3"],
        ],
    );
    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "d",
            "--every",
            "1w",
            "--start-by",
            "monday",
            "--agg",
            "sum:v,len",
            "--fill-gaps",
            "d.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["d", "v_sum", "count"],
        svec!["2024-01-01", "1", "1"],
        svec!["2024-01-08", "2", "1"],
        svec!["2024-01-15", "0", "0"],
        svec!["2024-01-22", "3", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_ambiguous_dates_are_month_first() {
    let wrk = Workdir::new("timebucketp_ambiguous_dates_are_month_first");
    wrk.create(
        "us.csv",
        vec![
            svec!["d", "v"],
            svec!["01/03/2024", "1"],
            svec!["01/09/2024", "2"],
        ],
    );
    // like the rest of qsv: January 3 and 9 (Polars alone would guess 1 March and 1 September)
    let mut cmd = tb(&wrk, &["-i", "d", "--every", "1mo", "us.csv"]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(got, vec![svec!["d", "count"], svec!["2024-01-01", "2"]]);

    let mut dmy = tb(&wrk, &["-i", "d", "--every", "1mo", "us.csv"]);
    dmy.env("QSV_PREFER_DMY", "1");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut dmy);
    assert_eq!(
        got,
        vec![
            svec!["d", "count"],
            svec!["2024-03-01", "1"],
            svec!["2024-09-01", "1"]
        ]
    );

    let mut explicit = tb(
        &wrk,
        &[
            "-i",
            "d",
            "--every",
            "1mo",
            "--date-format",
            "%d/%m/%Y",
            "us.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut explicit);
    assert_eq!(
        got,
        vec![
            svec!["d", "count"],
            svec!["2024-03-01", "1"],
            svec!["2024-09-01", "1"]
        ]
    );
}

#[test]
fn timebucketp_day_first_only_first_value_settles_the_order() {
    let wrk = Workdir::new("timebucketp_day_first_only_first_value_settles_the_order");
    wrk.create(
        "d.csv",
        vec![
            svec!["d", "v"],
            svec!["25/12/2024", "1"],
            svec!["01/03/2025", "2"],
        ],
    );
    let mut cmd = tb(&wrk, &["-i", "d", "--every", "1mo", "d.csv"]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["d", "count"],
            svec!["2024-12-01", "1"],
            svec!["2025-03-01", "1"]
        ]
    );
}

#[test]
fn timebucketp_later_day_first_value_is_an_error() {
    let wrk = Workdir::new("timebucketp_later_day_first_value_is_an_error");
    wrk.create(
        "d.csv",
        vec![
            svec!["d", "v"],
            svec!["01/03/2025", "1"],
            svec!["25/12/2024", "2"],
        ],
    );
    let mut cmd = tb(&wrk, &["-i", "d", "--every", "1mo", "d.csv"]);
    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("25/12/2024") && stderr.contains("QSV_PREFER_DMY"),
        "stderr: {stderr}"
    );
}

#[test]
fn timebucketp_zoned_datetimes_are_read_as_utc() {
    let wrk = Workdir::new("timebucketp_zoned_datetimes_are_read_as_utc");
    wrk.create(
        "z.csv",
        vec![
            svec!["ts", "v"],
            svec!["2024-01-01T10:15:00Z", "1"],
            svec!["2024-01-01T12:20:00+02:00", "2"],
        ],
    );
    let mut cmd = tb(
        &wrk,
        &["-i", "ts", "--every", "1h", "--agg", "sum:v", "z.csv"],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![svec!["ts", "v_sum"], svec!["2024-01-01 10:00:00", "3"]]
    );
}

#[test]
fn timebucketp_other_date_columns_are_not_reparsed() {
    let wrk = Workdir::new("timebucketp_other_date_columns_are_not_reparsed");
    wrk.create(
        "o.csv",
        vec![
            svec!["ts", "other"],
            svec!["2024-01-01 10:15:00", "01/03/2024"],
        ],
    );
    let mut cmd = tb(
        &wrk,
        &["-i", "ts", "--every", "1h", "--agg", "first:other", "o.csv"],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["ts", "other_first"],
            svec!["2024-01-01 10:00:00", "01/03/2024"]
        ]
    );
}

#[test]
fn timebucketp_date_format_with_time_fields() {
    let wrk = Workdir::new("timebucketp_date_format_with_time_fields");
    wrk.create(
        "eu.csv",
        vec![
            svec!["ts", "v"],
            svec!["03.01.2024 10:30", "1"],
            svec!["03.01.2024 10:45", "2"],
            svec!["03.01.2024 11:45", "4"],
        ],
    );
    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "ts",
            "--every",
            "1h",
            "--date-format",
            "%d.%m.%Y %H:%M",
            "--agg",
            "sum:v",
            "eu.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["ts", "v_sum"],
        svec!["2024-01-03 10:00:00", "3"],
        svec!["2024-01-03 11:00:00", "4"],
    ];
    assert_eq!(got, expected);
}

/// Run timebucketp and assert it fails with `needle` on stderr.
fn assert_fails_with(name: &str, args: &[&str], needle: &str) {
    let wrk = Workdir::new(name);
    readings(&wrk);
    wrk.create(
        "bad.csv",
        vec![svec!["ts", "v"], svec!["hello", "1"], svec!["world", "2"]],
    );
    wrk.create(
        "nul.csv",
        vec![
            svec!["ts", "v"],
            svec!["2024-01-01 00:00:00", "1"],
            svec!["", "2"],
        ],
    );
    wrk.create_from_string("hdr.csv", "ts,v\n");
    wrk.create(
        "count.csv",
        vec![svec!["ts", "count"], svec!["2024-01-01 00:00:00", "1"]],
    );
    wrk.create(
        "fl.csv",
        vec![svec!["ts", "v"], svec!["1.5", "1"], svec!["2.5", "2"]],
    );
    let mut cmd = tb(&wrk, args);
    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(stderr.contains(needle), "stderr: {stderr}");
}

#[test]
fn timebucketp_errors() {
    let cases: &[(&[&str], &str)] = &[
        (&["--every", "1h", "r.csv"], "--index <col> is required"),
        (&["-i", "ts", "r.csv"], "--every <dur> is required"),
        (
            &[
                "-i",
                "ts",
                "--rolling",
                "--period",
                "1h",
                "--every",
                "1h",
                "r.csv",
            ],
            "--every does not apply to --rolling",
        ),
        (
            &["-i", "ts", "--rolling", "r.csv"],
            "--rolling requires --period",
        ),
        (
            &[
                "-i",
                "ts",
                "--every",
                "1h",
                "--fill-gaps",
                "--label",
                "datapoint",
                "r.csv",
            ],
            "--fill-gaps requires --label left",
        ),
        (
            &[
                "-i",
                "ts",
                "--every",
                "1h",
                "--agg",
                "sum:temp,sum:temp",
                "r.csv",
            ],
            "produces the column \"temp_sum\" more than once",
        ),
        (
            &["-i", "ts", "--every", "1h", "--agg", "len:temp", "r.csv"],
            "--agg len takes no column",
        ),
        (
            &["-i", "ts", "--every", "1h", "--agg", "sum:missing", "r.csv"],
            "--agg column \"missing\" not found",
        ),
        (
            &[
                "-i",
                "ts",
                "--every",
                "1h",
                "--agg",
                "sum:missing",
                "hdr.csv",
            ],
            "--agg column \"missing\" not found",
        ),
        (
            &["-i", "ts", "--every", "1h", "--agg", "sum", "r.csv"],
            "needs a column, as sum:<column>",
        ),
        (
            &["-i", "ts", "--every", "1x", "r.csv"],
            "Invalid --every duration",
        ),
        (
            &["-i", "nope", "--every", "1h", "r.csv"],
            "--index column \"nope\" not found",
        ),
        (
            &["-i", "ts", "--every", "1h", "bad.csv"],
            "Could not parse the --index column as dates",
        ),
        (
            &["-i", "ts", "--every", "1h", "nul.csv"],
            "The --index column has empty values",
        ),
        (
            &["-i", "ts", "--every", "1i", "fl.csv"],
            "must hold dates, datetimes or integers",
        ),
        (
            &[
                "-i",
                "ts",
                "--every",
                "1h",
                "--fill-gaps",
                "--label",
                "right",
                "r.csv",
            ],
            "--fill-gaps requires --label left",
        ),
        (
            &["-i", "ts", "--every", "1h", "--by", "count", "count.csv"],
            "--agg output \"count\" clashes",
        ),
    ];
    for (i, (args, needle)) in cases.iter().enumerate() {
        assert_fails_with(&format!("timebucketp_errors_{i}"), args, needle);
    }
}

#[test]
fn timebucketp_rolling_by_group() {
    let wrk = Workdir::new("timebucketp_rolling_by_group");
    readings(&wrk);
    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "ts",
            "--rolling",
            "--period",
            "1h",
            "--by",
            "station",
            "--agg",
            "sum:temp",
            "r.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // each row's window is (ts - 1h, ts] within its own station
    let expected = vec![
        svec!["station", "ts", "temp_sum"],
        svec!["A", "2024-01-01 00:05:00", "1"],
        svec!["A", "2024-01-01 00:50:00", "4"],
        svec!["A", "2024-01-01 02:10:00", "5"],
        svec!["B", "2024-01-01 00:20:00", "10"],
        svec!["B", "2024-01-01 01:15:00", "30"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_sorted_accepts_sorted_input() {
    let wrk = Workdir::new("timebucketp_sorted_accepts_sorted_input");
    trades(&wrk);
    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "ts",
            "--every",
            "1h",
            "--sorted",
            "--agg",
            "sum:price",
            "t.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["ts", "price_sum"],
        svec!["2024-01-01 09:00:00", "30"],
        svec!["2024-01-01 10:00:00", "30"],
        svec!["2024-01-01 12:00:00", "40"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_header_only_input() {
    let wrk = Workdir::new("timebucketp_header_only_input");
    wrk.create_from_string("h.csv", "ts,g,v\n");
    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "ts",
            "--every",
            "1h",
            "--by",
            "g",
            "--fill-gaps",
            "--agg",
            "sum:v,len",
            "h.csv",
        ],
    );
    let got: String = wrk.stdout_on_success(&mut cmd);
    assert_eq!(got, "g,ts,v_sum,count");
}

#[test]
fn timebucketp_date_format_with_unpadded_time_fields() {
    // %-H and %-M are time fields too: the index must be read as datetimes, not dates
    let wrk = Workdir::new("timebucketp_date_format_with_unpadded_time_fields");
    wrk.create(
        "t.csv",
        vec![
            svec!["ts", "v"],
            svec!["2024-01-03 9:05", "1"],
            svec!["2024-01-03 10:30", "2"],
        ],
    );
    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "ts",
            "--every",
            "1h",
            "--date-format",
            "%Y-%m-%d %-H:%-M",
            "--agg",
            "sum:v",
            "t.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["ts", "v_sum"],
        svec!["2024-01-03 09:00:00", "1"],
        svec!["2024-01-03 10:00:00", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn timebucketp_min_max_of_text_dates_are_month_first() {
    // compared as text, "12/31/2023" would rank above "01/05/2024"; first keeps it as written
    let wrk = Workdir::new("timebucketp_min_max_of_text_dates_are_month_first");
    wrk.create(
        "o.csv",
        vec![
            svec!["ts", "due"],
            svec!["2024-01-01 10:15:00", "12/31/2023"],
            svec!["2024-01-01 10:45:00", "01/05/2024"],
        ],
    );
    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "ts",
            "--every",
            "1h",
            "--agg",
            "min:due,max:due,first:due",
            "o.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["ts", "due_min", "due_max", "due_first"],
            svec![
                "2024-01-01 10:00:00",
                "2023-12-31",
                "2024-01-05",
                "12/31/2023"
            ]
        ]
    );
}

#[test]
fn timebucketp_min_max_of_mixed_text_compare_as_text() {
    // a column that is not all dates keeps text comparison, whatever the row order
    let wrk = Workdir::new("timebucketp_min_max_of_mixed_text_compare_as_text");
    wrk.create(
        "o.csv",
        vec![
            svec!["ts", "status"],
            svec!["2024-01-01 10:15:00", "12/31/2023"],
            svec!["2024-01-01 10:45:00", "pending"],
        ],
    );
    let mut cmd = tb(
        &wrk,
        &["-i", "ts", "--every", "1h", "--agg", "max:status", "o.csv"],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["ts", "status_max"],
            svec!["2024-01-01 10:00:00", "pending"]
        ]
    );
}

#[test]
fn timebucketp_two_digit_years_need_a_date_format() {
    // chrono's %Y would read 10/05/24 as the year 10 (or 24); refuse to guess instead
    let wrk = Workdir::new("timebucketp_two_digit_years_need_a_date_format");
    wrk.create("y.csv", vec![svec!["d", "v"], svec!["10/05/24", "1"]]);
    let mut guessed = tb(&wrk, &["-i", "d", "--every", "1mo", "y.csv"]);
    let stderr = wrk.stderr_on_error(&mut guessed);
    assert!(stderr.contains("--date-format"), "stderr: {stderr}");

    // a later short year, after a 4-digit first value, must not become the year 24 either
    wrk.create(
        "later.csv",
        vec![
            svec!["d", "v"],
            svec!["10/05/2024", "1"],
            svec!["10/06/24", "2"],
        ],
    );
    let mut later = tb(&wrk, &["-i", "d", "--every", "1mo", "later.csv"]);
    let stderr = wrk.stderr_on_error(&mut later);
    assert!(
        stderr.contains(r#"value "10/06/24" has a year before 1000"#),
        "stderr: {stderr}"
    );
    // nor one chrono reads past a space, or one in a format Polars infers
    for (name, first, short) in [
        ("space.csv", "10/05/2024", "10/06/ 24"),
        ("iso.csv", "2024-10-05T10:15:00", "24-10-06T10:15:00"),
    ] {
        wrk.create(
            name,
            vec![svec!["d", "v"], svec![first, "1"], svec![short, "2"]],
        );
        let mut short_cmd = tb(&wrk, &["-i", "d", "--every", "1mo", name]);
        let stderr = wrk.stderr_on_error(&mut short_cmd);
        assert!(
            stderr.contains(&format!("value \"{short}\" has a year before 1000")),
            "{name} stderr: {stderr}"
        );
    }

    let mut cmd = tb(
        &wrk,
        &[
            "-i",
            "d",
            "--every",
            "1mo",
            "--date-format",
            "%m/%d/%y",
            "y.csv",
        ],
    );
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(got, vec![svec!["d", "count"], svec!["2024-10-01", "1"]]);
}
