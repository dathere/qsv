use std::{borrow::ToOwned, cmp, process};

use newline_converter::dos2unix;

use crate::workdir::Workdir;

macro_rules! stats_tests {
    ($name:ident, $field:expr_2021, $rows:expr_2021, $expect:expr_2021) => {
        stats_tests!($name, $field, $rows, $expect, false, true);
    };
    ($name:ident, $field:expr_2021, $rows:expr_2021, $expect:expr_2021, $nulls:expr_2021, $infer_dates:expr_2021) => {
        mod $name {
            use super::test_stats;

            stats_test_headers!($name, $field, $rows, $expect, $nulls, $infer_dates);
            stats_test_no_headers!($name, $field, $rows, $expect, $nulls, $infer_dates);
        }
    };
}

macro_rules! stats_no_infer_dates_tests {
    ($name:ident, $field:expr_2021, $rows:expr_2021, $expect:expr_2021) => {
        stats_tests!($name, $field, $rows, $expect, false, false);
    };
    ($name:ident, $field:expr_2021, $rows:expr_2021, $expect:expr_2021, $nulls:expr_2021, $infer_dates:expr_2021) => {
        mod $name {
            use super::test_stats;

            stats_test_headers!($name, $field, $rows, $expect, $nulls, $infer_dates);
            stats_test_no_headers!($name, $field, $rows, $expect, $nulls, $infer_dates);
        }
    };
}

macro_rules! stats_test_headers {
    ($name:ident, $field:expr_2021, $rows:expr_2021, $expect:expr_2021) => {
        stats_test_headers!($name, $field, $rows, $expect, false, true);
    };
    ($name:ident, $field:expr_2021, $rows:expr_2021, $expect:expr_2021, $nulls:expr_2021, $infer_dates:expr_2021) => {
        #[test]
        fn headers_no_index() {
            let name = concat!(stringify!($name), "_headers_no_index");
            test_stats(
                name,
                $field,
                $rows,
                $expect,
                true,
                false,
                $nulls,
                $infer_dates,
            );
        }

        #[test]
        fn headers_index() {
            let name = concat!(stringify!($name), "_headers_index");
            test_stats(
                name,
                $field,
                $rows,
                $expect,
                true,
                true,
                $nulls,
                $infer_dates,
            );
        }
    };
}

macro_rules! stats_test_no_headers {
    ($name:ident, $field:expr_2021, $rows:expr_2021, $expect:expr_2021) => {
        stats_test_no_headers!($name, $field, $rows, $expect, false, true);
    };
    ($name:ident, $field:expr_2021, $rows:expr_2021, $expect:expr_2021, $nulls:expr_2021, $infer_dates:expr_2021) => {
        #[test]
        fn no_headers_no_index() {
            let name = concat!(stringify!($name), "_no_headers_no_index");
            test_stats(
                name,
                $field,
                $rows,
                $expect,
                false,
                false,
                $nulls,
                $infer_dates,
            );
        }

        #[test]
        fn no_headers_index() {
            let name = concat!(stringify!($name), "_no_headers_index");
            test_stats(
                name,
                $field,
                $rows,
                $expect,
                false,
                true,
                $nulls,
                $infer_dates,
            );
        }
    };
}

#[allow(clippy::too_many_arguments)]
fn test_stats<S>(
    name: S,
    field: &str,
    rows: &[&str],
    expected: &str,
    headers: bool,
    use_index: bool,
    nulls: bool,
    infer_dates: bool,
) where
    S: ::std::ops::Deref<Target = str>,
{
    let (wrk, mut cmd) = setup(name, rows, headers, use_index, nulls, infer_dates);
    let field_val = get_field_value(&wrk, &mut cmd, field).unwrap_or_default();
    // Only compare the first few bytes since floating point arithmetic
    // can mess with exact comparisons.
    // when field = skewness, we're comparing a long sequence of the quartile columns,
    // that's why we use 40, if not, its a single column, and we need to compare only
    // the first 15 characters just in case its a float
    let len = cmp::min(
        if field == "skewness" { 40 } else { 15 },
        cmp::min(field_val.len(), expected.len()),
    );
    assert_eq!(&field_val[0..len], &expected[0..len]);
}

fn setup<S>(
    name: S,
    rows: &[&str],
    headers: bool,
    use_index: bool,
    nulls: bool,
    infer_dates: bool,
) -> (Workdir, process::Command)
where
    S: ::std::ops::Deref<Target = str>,
{
    let wrk = Workdir::new(&name);
    let mut data: Vec<Vec<String>> = rows.iter().map(|&s| vec![s.to_owned()]).collect();
    if headers {
        data.insert(0, svec!["header"]);
    }
    if use_index {
        wrk.create_indexed("in.csv", data);
    } else {
        wrk.create("in.csv", data);
    }

    let mut cmd = wrk.command("stats");
    cmd.arg("in.csv");
    if !headers {
        cmd.arg("--no-headers");
    }
    if nulls {
        cmd.arg("--nulls");
    }
    if infer_dates {
        cmd.arg("--infer-dates").arg("--dates-whitelist").arg("all");
    }

    (wrk, cmd)
}

fn get_field_value(
    wrk: &Workdir,
    cmd: &mut process::Command,
    field: &str,
) -> Result<String, String> {
    if field == "median" {
        cmd.arg("--median");
    }
    if field == "quartiles" {
        cmd.arg("--quartiles");
    }
    if field == "cardinality" {
        cmd.arg("--cardinality");
    }
    if field == "mode" || field == "antimode" {
        cmd.arg("--mode");
    }
    if field == "infer_dates" {
        cmd.arg("--infer-dates");
    }

    let mut rows: Vec<Vec<String>> = wrk.read_stdout(cmd);
    if rows.is_empty() {
        return Err(format!("Empty stats for command '{cmd:?}'."));
    }
    let headers = rows.remove(0);
    let mut sequence: Vec<&str> = vec![];
    for row in &rows {
        for (h, val) in headers.iter().zip(row.iter()) {
            match field {
                "quartiles" => match &**h {
                    "lower_outer_fence" | "lower_inner_fence" | "q1" | "q2_median" | "q3"
                    | "iqr" | "upper_inner_fence" | "upper_outer_fence" => {
                        sequence.push(val);
                    },
                    "skewness" => {
                        sequence.push(val);
                        return Ok(sequence.join(","));
                    },
                    _ => {},
                },
                _ => {
                    if &**h == field {
                        return Ok(val.clone());
                    }
                },
            }
        }
    }
    Err(format!(
        "Could not find field '{field}' in headers '{headers:?}' for command '{cmd:?}'."
    ))
}

stats_tests!(stats_infer_string, "type", &["a"], "String");
stats_tests!(stats_infer_int, "type", &["1"], "Integer");
stats_tests!(stats_infer_float, "type", &["1.2"], "Float");
stats_tests!(stats_infer_null, "type", &[""], "NULL");
stats_tests!(stats_infer_date, "type", &["1968-06-27"], "Date");
stats_tests!(stats_infer_date_unix_epoch, "type", &["1970-01-01"], "Date");
stats_tests!(
    stats_infer_date_unix_epoch_2,
    "type",
    &["1970-01-01 00:00:00"],
    "Date"
);
stats_tests!(
    stats_infer_date_unix_epoch_3,
    "type",
    &["1970-01-01 00:00:00 UTC"],
    "Date"
);
stats_tests!(
    stats_infer_date_unix_epoch_4,
    "type",
    &["1970-01-01 00:00:00.000 UTC"],
    "Date"
);
stats_tests!(
    stats_infer_date_unix_epoch_5,
    "type",
    &["1970-01-01 00:00:00.000000 UTC"],
    "Date"
);
stats_tests!(
    stats_infer_date_unix_epoch_6,
    "type",
    &["1970-01-01 00:00:00.000000000 UTC"],
    "Date"
);
stats_tests!(
    stats_infer_date_unix_epoch_7,
    "type",
    &["1970-01-01 00:00:01 UTC"],
    "DateTime"
);
stats_no_infer_dates_tests!(stats_infer_nodate, "type", &["1968-06-27"], "String");
stats_tests!(
    stats_infer_datetime,
    "type",
    &["1968-06-27 12:30:01"],
    "DateTime"
);
stats_no_infer_dates_tests!(
    stats_infer_nodatetime,
    "type",
    &["1968-06-27 12:30:01"],
    "String"
);
stats_tests!(stats_infer_string_null, "type", &["a", ""], "String");
stats_tests!(stats_infer_int_null, "type", &["1", ""], "Integer");
stats_tests!(stats_infer_float_null, "type", &["1.2", ""], "Float");
stats_tests!(
    stats_infer_date_null,
    "type",
    &["June 27, 1968", ""],
    "Date"
);
stats_no_infer_dates_tests!(
    stats_infer_no_infer_dates_null,
    "type",
    &["June 27, 1968", ""],
    "String"
);
stats_tests!(
    stats_infer_datetime_null,
    "type",
    &["June 27, 1968 12:30:00 UTC", ""],
    "DateTime"
);
stats_no_infer_dates_tests!(
    stats_infer_nodatetime_null,
    "type",
    &["June 27, 1968 12:30:00 UTC", ""],
    "String"
);
stats_tests!(stats_infer_null_string, "type", &["", "a"], "String");
stats_tests!(stats_infer_null_int, "type", &["", "1"], "Integer");
stats_tests!(stats_infer_null_float, "type", &["", "1.2"], "Float");
stats_tests!(
    stats_infer_null_date,
    "type",
    &["", "September 17, 2012 at 10:09am PST"],
    "Date"
);
stats_no_infer_dates_tests!(
    stats_infer_null_nodate,
    "type",
    &["", "September 17, 2012 at 10:09am PST"],
    "String"
);
stats_tests!(
    stats_infer_date_datetime,
    "type",
    &["September 11, 2001", "September 17, 2012 at 10:09am PST"],
    "DateTime"
);
stats_no_infer_dates_tests!(
    stats_infer_nodate_nodatetime,
    "type",
    &["September 11, 2001", "September 17, 2012 at 10:09am PST"],
    "String"
);
stats_tests!(stats_infer_int_string, "type", &["1", "a"], "String");
stats_tests!(stats_infer_string_int, "type", &["a", "1"], "String");
stats_tests!(stats_infer_int_float, "type", &["1", "1.2"], "Float");
stats_tests!(stats_infer_float_int, "type", &["1.2", "1"], "Float");
stats_tests!(
    stats_infer_null_int_float_string,
    "type",
    &["", "1", "1.2", "a"],
    "String"
);
stats_tests!(
    stats_infer_date_string,
    "type",
    &["1968-06-27", "abcde"],
    "String"
);
stats_tests!(
    stats_infer_string_date,
    "type",
    &["wxyz", "1968-06-27"],
    "String"
);

stats_tests!(stats_no_mean, "mean", &["a"], "");
stats_tests!(stats_no_stddev, "stddev", &["a"], "");
stats_tests!(stats_no_variance, "variance", &["a"], "");
stats_tests!(stats_no_median, "median", &["a"], "");
stats_tests!(stats_no_quartiles, "quartiles", &["a"], ",,,,,");
stats_tests!(stats_no_mode, "mode", &["a", "b"], "N/A");
stats_tests!(
    stats_multiple_modes,
    "mode",
    &["a", "a", "b", "b", "c", "d", "e", "e"],
    "a|b|e|3|1"
);
stats_tests!(
    stats_multiple_modes_num,
    "mode",
    &["5", "5", "33", "33", "42", "17", "99", "99"],
    "33|5|99|3|1"
);
stats_tests!(
    stats_multiple_antimodes,
    "antimode",
    &["a", "a", "b", "b", "c", "d", "e", "e"],
    "c|d|2|1"
);
stats_tests!(
    stats_multiple_antimodes_num,
    "antimode",
    &["5", "5", "33", "33", "42", "17", "98", "99", "99"],
    "17|42|98|3|1"
);
stats_tests!(
    stats_range,
    "range",
    &["a", "a", "b", "b", "c", "d", "e", "e"],
    ""
);
stats_tests!(
    stats_range_num,
    "range",
    &["5", "5", "33", "33", "42", "17", "98", "99", "99"],
    "94"
);
stats_tests!(
    stats_sparsity,
    "sparsity",
    &["5", "5", "33", "33", "42", "17", "98", "99", "99", ""],
    "0.1"
);

stats_tests!(stats_null_mean, "mean", &[""], "");
stats_tests!(stats_null_stddev, "stddev", &[""], "");
stats_tests!(stats_null_variance, "variance", &[""], "");
stats_tests!(stats_null_median, "median", &[""], "");
stats_tests!(stats_null_quartiles, "quartiles", &[""], ",,,,,");
stats_tests!(stats_null_mode, "mode", &[""], "N/A");
stats_tests!(stats_null_antimode, "antimode", &[""], "*ALL");
stats_tests!(stats_null_range, "range", &[""], "N/A");
stats_tests!(stats_null_sparsity, "sparsity", &[""], "1.0");

stats_tests!(stats_includenulls_null_mean, "mean", &[""], "", true, false);
stats_tests!(
    stats_includenulls_null_stddev,
    "stddev",
    &[""],
    "",
    true,
    false
);
stats_tests!(
    stats_includenulls_null_variance,
    "variance",
    &[""],
    "",
    true,
    false
);
stats_tests!(
    stats_includenulls_null_median,
    "median",
    &[""],
    "",
    true,
    false
);
stats_tests!(
    stats_includenulls_null_quartiles,
    "quartiles",
    &[""],
    ",,,,,",
    true,
    false
);
stats_tests!(
    stats_includenulls_null_mode,
    "mode",
    &[""],
    "N/A",
    true,
    false
);
stats_tests!(
    stats_includenulls_null_antimode,
    "antimode",
    &[""],
    "*ALL",
    true,
    false
);
stats_tests!(
    stats_includenulls_null_range,
    "range",
    &[""],
    "N/A",
    true,
    false
);
stats_tests!(
    stats_includenulls_null_sparsity,
    "sparsity",
    &[""],
    "1.0",
    true,
    false
);

stats_tests!(
    stats_includenulls_mean,
    "mean",
    &["5", "", "15", "10"],
    "7.5",
    true,
    false
);

stats_tests!(stats_sum_integers, "sum", &["1", "2"], "3");
stats_tests!(stats_sum_floats, "sum", &["1.5", "2.8"], "4.3");
stats_tests!(stats_sum_mixed1, "sum", &["1.5", "2"], "3.5");
stats_tests!(stats_sum_mixed2, "sum", &["2", "1.5"], "3.5");
stats_tests!(stats_sum_mixed3, "sum", &["1.5", "hi", "2.8"], "4.3");
stats_tests!(stats_sum_nulls1, "sum", &["1", "", "2"], "3");
stats_tests!(stats_sum_nulls2, "sum", &["", "1", "2"], "3");
stats_tests!(
    stats_sum_overflow,
    "sum",
    &[
        "9223372036854775807", // i64::MAX
        "1",
        "2"
    ],
    "*OVERFLOW*"
);
stats_tests!(
    stats_sum_negative_overflow,
    "sum",
    &[
        "-9223372036854775808", // i64::MIN
        "-1",
        "-2"
    ],
    "*UNDERFLOW*"
);

stats_tests!(stats_min, "min", &["2", "1.1"], "1.1");
stats_tests!(stats_max, "max", &["2", "1.1"], "2");
stats_tests!(stats_min_mix, "min", &["2", "a", "1.1"], "1.1");
stats_tests!(stats_max_mix, "max", &["2", "a", "1.1"], "a");
stats_tests!(stats_min_null, "min", &["", "2", "1.1"], "1.1");
stats_tests!(stats_max_null, "max", &["2", "1.1", ""], "2");

stats_tests!(stats_len_min, "min_length", &["aa", "a"], "1");
stats_tests!(stats_len_max, "max_length", &["a", "aa"], "2");
stats_tests!(stats_len_min_null, "min_length", &["", "aa", "a"], "0");
stats_tests!(stats_len_max_null, "max_length", &["a", "aa", ""], "2");

stats_tests!(stats_mean, "mean", &["5", "15", "10"], "10");
stats_tests!(stats_stddev, "stddev", &["1", "2", "3"], "0.8165");
stats_tests!(stats_variance, "variance", &["3", "5", "7", "9", "11"], "8");
stats_tests!(stats_mean_null, "mean", &["", "5", "15", "10"], "10");
stats_tests!(stats_stddev_null, "stddev", &["1", "2", "3", ""], "0.8165");
stats_tests!(
    stats_variance_null,
    "variance",
    &["3", "5", "7", "9", "", "10"],
    "6"
);
stats_tests!(stats_mean_mix, "mean", &["5", "15.1", "9.9"], "10");
stats_tests!(stats_stddev_mix, "stddev", &["1", "2.1", "2.9"], "0.7789");
stats_tests!(
    stats_variance_mix,
    "variance",
    &["1.5", "2", "2.5", "3"],
    "0.3125"
);

stats_tests!(stats_cardinality, "cardinality", &["a", "b", "a"], "2");
stats_tests!(stats_mode, "mode", &["a", "b", "a"], "a,1,2");
stats_tests!(stats_mode_null, "mode", &["", "a", "b", "a"], "a,1,2");
stats_tests!(stats_antimode, "antimode", &["a", "b", "a"], "b,1,1");
stats_tests!(
    stats_antimode_null,
    "antimode",
    &["", "a", "b", "a"],
    "NULL|b|2|1"
);
stats_tests!(stats_median, "median", &["1", "2", "3"], "2");
stats_tests!(stats_median_null, "median", &["", "1", "2", "3"], "2");
stats_tests!(stats_median_even, "median", &["1", "2", "3", "4"], "2.5");
stats_tests!(
    stats_median_even_null,
    "median",
    &["", "1", "2", "3", "4"],
    "2.5"
);
stats_tests!(stats_median_mix, "median", &["1", "2.5", "3"], "2.5");
stats_tests!(
    stats_quartiles,
    "quartiles",
    &["1", "2", "3"],
    "-5,-2,1,2,3,2,6,9"
);
stats_tests!(
    stats_quartiles_null,
    "quartiles",
    &["", "1", "2", "3"],
    "-5,-2,1,2,3,2,6,9"
);
stats_tests!(
    stats_quartiles_even,
    "quartiles",
    &["1", "2", "3", "4"],
    "-4.5,-1.5,1.5,2.5,3.5,2,6.5,9.5"
);
stats_tests!(
    stats_quartiles_even_null,
    "quartiles",
    &["", "1", "2", "3", "4"],
    "-4.5,-1.5,1.5,2.5,3.5,2,6.5,9.5"
);
stats_tests!(
    stats_quartiles_mix,
    "quartiles",
    &["1", "2.0", "3", "4"],
    "-4.5,-1.5,1.5,2.5,3.5,2,6.5,9.5"
);
stats_tests!(stats_quartiles_null_empty, "quartiles", &[""], "");

stats_tests!(stats_nullcount, "nullcount", &["", "1", "2"], "1");
stats_tests!(stats_nullcount_none, "nullcount", &["a", "1", "2"], "0");
stats_tests!(
    stats_nullcount_spacenotnull,
    "nullcount",
    &[" ", "1", "2"],
    "0"
);
stats_tests!(stats_nullcount_all, "nullcount", &["", "", ""], "3");
stats_no_infer_dates_tests!(
    stats_noinfer_null_nodate2,
    "type",
    &["", "September 17, 2012 at 10:09am PST"],
    "String",
    true,
    false
);
stats_no_infer_dates_tests!(
    stats_infer_null_nodate2,
    "type",
    &["", "September 17, 2012 at 10:09am PST"],
    "DateTime",
    true,
    true
);
stats_tests!(
    stats_infer_date_datetime2,
    "type",
    &["September 11, 2001", "September 17, 2012 at 10:09am PST"],
    "DateTime",
    false,
    true
);
stats_no_infer_dates_tests!(
    stats_infer_date_datetime3,
    "type",
    &["9-11", "September 17, 2012 at 10:09am PST"],
    "String",
    false,
    true
);

#[test]
fn stats_prefer_dmy() {
    let wrk = Workdir::new("stats_prefer_dmy");
    let test_file = wrk.load_test_file("boston311-dmy-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-dates")
        .arg("--prefer-dmy")
        .arg("--dataset-stats")
        .args(["--dates-whitelist", "_dT"])
        .arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed stddev_length, variance_length, cv_length, variance, geometric_mean, harmonic_mean,
    // stddev, sem & cv columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("!/stddev_length|variance_length|cv_length|variance|geometric_mean|harmonic_mean|stddev|sem|cv/")
        .arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_prefer_mdy() {
    let wrk = Workdir::new("stats_prefer_mdy");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-dates")
        .arg("--dataset-stats")
        .args(["--dates-whitelist", "_dt"])
        .arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed stddev_length, variance_length, cv_length, variance, geometric_mean, harmonic_mean,
    // stddev, sem & cv columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("!/stddev_length|variance_length|cv_length|variance|geometric_mean|harmonic_mean|stddev|sem|cv/")
        .arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);

    let expected2 = wrk.load_test_resource("boston311-100-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_rounding() {
    let wrk = Workdir::new("stats_rounding");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.args(&["--everything", "--dataset-stats"])
        .args(&["--round", "8"])
        .arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed stddev_length, variance_length, cv_length, variance, geometric_mean, harmonic_mean,
    // stddev, sem & cv columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("!/stddev_length|variance_length|cv_length|variance|geometric_mean|harmonic_mean|stddev|sem|cv/")
        .arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-everything-8places-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_no_rounding() {
    let wrk = Workdir::new("stats_no_rounding");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--everything")
        .args(["--round", "9999"])
        .arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-everything-norounding-stats.csv");

    // this should NOT BE EQUAL as floats are not rounded, and comparing floats is not reliable
    assert_ne!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_no_date_inference() {
    let wrk = Workdir::new("stats_no_date_inference");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.args(&["--everything", "--dataset-stats"])
        .arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed stddev_length, variance_length, cv_length, variance, geometric_mean, harmonic_mean,
    // stddev, sem & cv columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("!/stddev_length|variance_length|cv_length|variance|geometric_mean|harmonic_mean|stddev|sem|cv/")
        .arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-everything-nodate-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_with_date_inference() {
    let wrk = Workdir::new("stats_with_date_inference");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--everything")
        .arg(test_file)
        .arg("--infer-dates")
        .arg("--dataset-stats")
        .args(["--dates-whitelist", "all"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed stddev_length, variance_length, cv_length, variance, geometric_mean, harmonic_mean,
    // stddev, sem & cv columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("!/stddev_length|variance_length|cv_length|variance|geometric_mean|harmonic_mean|stddev|sem|cv/")
        .arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-everything-date-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_with_date_inference_default_whitelist() {
    let wrk = Workdir::new("stats_with_date_inference_default_whitelist");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.args(&["--everything", "--dataset-stats"])
        .arg(test_file)
        .arg("--infer-dates");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed stddev_length, variance_length, cv_length, variance, geometric_mean, harmonic_mean,
    // stddev, sem & cv columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("!/stddev_length|variance_length|cv_length|variance|geometric_mean|harmonic_mean|stddev|sem|cv/")
        .arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 =
        wrk.load_test_resource("boston311-100-everything-inferdates-defaultwhitelist-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_with_date_inference_variance_stddev() {
    let wrk = Workdir::new("stats_with_date_inference_variance_stddev");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--everything")
        .arg(test_file)
        .arg("--infer-dates")
        .args(["--dates-whitelist", "all"])
        .arg("--dataset-stats");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 =
        wrk.load_test_resource("boston311-100-everything-date-stats-variance-stddev.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_with_date_type() {
    let wrk = Workdir::new("stats_with_date_type");
    let test_file = wrk.load_test_file("boston311-100-notime.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--everything")
        .arg(test_file)
        .arg("--infer-dates")
        .args(["--dates-whitelist", "all"])
        .arg("--dataset-stats");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed stddev_length, variance_length, cv_length, variance, geometric_mean, harmonic_mean,
    // stddev, sem & cv columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("!/stddev_length|variance_length|cv_length|variance|geometric_mean|harmonic_mean|stddev|sem|cv/")
        .arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-everything-datenotime-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_typesonly() {
    let wrk = Workdir::new("stats_typesonly");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.args(["--typesonly", "--dataset-stats"]).arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-typesonly-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_typesonly_with_dates() {
    let wrk = Workdir::new("stats_typesonly_with_dates");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.args(["--typesonly", "--dataset-stats"])
        .arg("--infer-dates")
        .args(["--dates-whitelist", "all"])
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-typesonly-withdates-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_typesonly_cache_threshold_zero() {
    use std::path::Path;

    let wrk = Workdir::new("stats_typesonly_cache_threshold_zero");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.args(["--typesonly", "--dataset-stats"])
        .arg("--infer-dates")
        .args(["--dates-whitelist", "all"])
        .args(&["--cache-threshold", "0"])
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-typesonly-withdates-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());

    // check that the stats cache files were NOT created
    assert!(!Path::new(&wrk.path("boston311-100.stats.csv")).exists());
    assert!(!Path::new(&wrk.path("boston311-100.stats.csv.json")).exists());
}

#[test]
fn stats_typesonly_cache() {
    let wrk = Workdir::new("stats_typesonly_cache");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.args(["--typesonly", "--dataset-stats"])
        .arg("--infer-dates")
        .args(["--dates-whitelist", "all"])
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-typesonly-withdates-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_cache() {
    use std::path::Path;

    let wrk = Workdir::new("stats_cache");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-dates")
        .args(["--dates-whitelist", "all"])
        .arg("--dataset-stats")
        // set cache threshold to 1 to force cache creation
        .args(&["--cache-threshold", "1"])
        .arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed variance, stddev, sem & cv columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("!/variance|geometric_mean|harmonic_mean|stddev|sem|cv/")
        .arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());

    // check that the stats cache files were created
    assert!(Path::new(&wrk.path("boston311-100.stats.csv")).exists());
    assert!(Path::new(&wrk.path("boston311-100.stats.csv.json")).exists());
}

#[test]
#[ignore = "temporarily ignore while tblshooting fingerprint hash and cache_threshold"]
fn stats_cache_negative_threshold() {
    use std::path::Path;

    let wrk = Workdir::new("stats_cache_negative_threshold");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-dates")
        .arg("--dates-whitelist")
        .arg("all")
        // set cache threshold to -10240 to set autoindex_size to 10 kb
        // and to force cache creation
        .args(["-c", "-10240"])
        .arg(test_file.clone());

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // the index file SHOULD have been created as the input file size > 10 kb
    assert!(Path::new(&format!("{test_file}.idx")).exists());

    wrk.create("in2.csv", got);

    // removed variance, stddev, sem & cv columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("!/variance|geometric_mean|harmonic_mean|stddev|sem|cv/")
        .arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());

    // check that the stats cache files were created
    assert!(Path::new(&wrk.path("boston311-100.stats.csv")).exists());
    assert!(Path::new(&wrk.path("boston311-100.stats.csv.json")).exists());
}

#[test]
fn stats_cache_negative_threshold_unmet() {
    use std::path::Path;

    let wrk = Workdir::new("stats_cache_negative_threshold_unmet");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-dates")
        .args(["--dates-whitelist", "all"])
        .arg("--dataset-stats")
        // set cache threshold to -51200 to set autoindex_size to 50 kb
        // and to force cache creation
        .args(&["--cache-threshold", "-51200"])
        .arg(test_file.clone());

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // the index file SHOULD NOT have been created as the input file < 50 kb
    assert!(!Path::new(&format!("{test_file}.idx")).exists());

    wrk.create("in2.csv", got);

    // removed variance, stddev, sem & cv columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("!/variance|geometric_mean|harmonic_mean|stddev|sem|cv/")
        .arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());

    // check that the stats cache files were created
    assert!(Path::new(&wrk.path("boston311-100.stats.csv")).exists());
    assert!(Path::new(&wrk.path("boston311-100.stats.csv.json")).exists());
}

#[test]
#[ignore = "temporarily ignore while tblshooting fingerprint hash and cache_threshold"]
fn stats_cache_negative_threshold_five() {
    use std::path::Path;

    let wrk = Workdir::new("stats_cache_negative_threshold_five");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-dates")
        .arg("--dates-whitelist")
        .arg("all")
        // set cache threshold to -10245 to set autoindex_size to 10 kb
        // this creates an index file, and then autodeletes it AND the stats cache files
        .args(["-c", "-10245"])
        .arg(test_file.clone());

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // the index file WAS CREATED as the input file is > 10k
    // but the index file WAS DELETED after stats exits as the threshold was negative
    // and ends with a 5
    assert!(!Path::new(&format!("{test_file}.idx")).exists());

    wrk.create("in2.csv", got);

    // removed variance, stddev, sem & cv columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("!/variance|geometric_mean|harmonic_mean|stddev|sem|cv/")
        .arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());

    // check that the stats cache files were created
    assert!(!Path::new(&wrk.path("boston311-100.stats.csv")).exists());
    assert!(!Path::new(&wrk.path("boston311-100.stats.csv.json")).exists());
}

#[test]
fn stats_antimodes_len_500() {
    let wrk = Workdir::new("stats_antimodes_len_500");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.env("QSV_ANTIMODES_LEN", "500")
        .args(&["--everything", "--dataset-stats"])
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-antimodes-len500-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_infer_boolean_1_0() {
    let wrk = Workdir::new("stats_infer_boolean_1_0");
    let test_file = wrk.load_test_file("boston311-10-boolean-1or0.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--dataset-stats")
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-10-boolean-1or0-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_infer_boolean_t_f() {
    let wrk = Workdir::new("stats_infer_boolean_t_f");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--dataset-stats")
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-10-boolean-tf-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_infer_boolean_true_false() {
    let wrk = Workdir::new("stats_infer_boolean_true_false");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--boolean-patterns")
        .arg("true:false")
        .arg("--dataset-stats")
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-10-boolean-tf-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_infer_boolean_true_false_error() {
    let wrk = Workdir::new("stats_infer_boolean_true_false_error");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--boolean-patterns")
        .arg("true:falsy")
        .arg("--dataset-stats")
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-10-boolean-tf-stats.csv");

    assert_ne!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_infer_boolean_true_false_error_pattern_mismatch() {
    let wrk = Workdir::new("stats_infer_boolean_true_false_error_pattern_mismatch");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--boolean-patterns")
        .arg("true:no")
        .arg("--dataset-stats")
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-10-boolean-tf-stats.csv");

    assert_ne!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_typesonly_infer_boolean_t_f() {
    let wrk = Workdir::new("stats_typesonly_infer_boolean_t_f");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--typesonly")
        .arg("--infer-boolean")
        .arg("--dataset-stats")
        .arg(test_file);

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-10-typesonly-boolean-tf-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_typesonly_infer_boolean_t_f_infer_dates() {
    let wrk = Workdir::new("stats_typesonly_infer_boolean_t_f_infer_dates");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--typesonly")
        .arg("--infer-boolean")
        .arg("--infer-dates")
        .arg("--dataset-stats")
        .arg(test_file);

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-10-typesonly-boolean-tf-inferdates-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_infer_boolean_invalid_pattern() {
    let wrk = Workdir::new("stats_infer_boolean_invalid_pattern");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--boolean-patterns")
        .arg(":false,yep:,1:0")
        .arg("--dataset-stats")
        .arg(test_file);

    wrk.assert_err(&mut cmd);
}

#[test]
fn stats_is_ascii() {
    let wrk = Workdir::new("stats_is_ascii");
    let test_file = wrk.load_test_file("boston311-100-with-nonascii.csv");
    let mut cmd = wrk.command("stats");
    cmd.arg(test_file).arg("--dataset-stats").arg("--force");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed stddev_length, variance_length, cv_length, variance, geometric_mean, harmonic_mean,
    // stddev, sem & cv columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("!/stddev_length|variance_length|cv_length|variance|geometric_mean|harmonic_mean|stddev|sem|cv/")
        .arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-with-nonascii-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_everything_utf8_japanese_issue817() {
    let wrk = Workdir::new("stats_everything_utf8_japanese");
    let test_file = wrk.load_test_file("utf8-japanesedata.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--everything").arg(test_file);

    wrk.assert_success(&mut cmd);
    // TODO: for now, let's just make sure it doesn't crash
    // comparing utf8 output is a bit tricky, with git line endings
    // and other things

    // let got: String = wrk.stdout(&mut cmd);
    // let expected = wrk.load_test_resource("utf8-japanesedata-stats-everything.csv");
    // assert_eq!(dos2unix(&got).trim_end(), dos2unix(&expected).trim_end());
}

#[test]
fn stats_leading_zero_handling() {
    let wrk = Workdir::new("stats_leading_zero_handling");

    wrk.create(
        "data.csv",
        vec![
            svec!["col1", "col2", "col3"],
            svec!["1", "4321", "01"],
            svec!["2", "3210", "02"],
            svec!["3", "2101", "03"],
            svec!["4", "1012", "04"],
            svec!["5", "0", "10"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--typesonly")
        .arg("--dataset-stats")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "type"],
        svec!["col1", "Integer"],
        svec!["col2", "Integer"],
        svec!["col3", "String"],
        svec!["qsv__rowcount", "5"],
        svec!["qsv__columncount", "3"],
        svec!["qsv__filesize_bytes", "62"],
        svec![
            "qsv__fingerprint_hash",
            //DevSkim::ignore DS173237
            "ae045ecc55c3c99d40dd2b7369e55db9d15d1a19988850c496aa3afd456e164e"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn stats_zero_cv() {
    let wrk = Workdir::new("stats_zero_cv");

    wrk.create(
        "data.csv",
        vec![
            svec!["col1", "col2", "col3", "col4"],
            svec!["1", "-10", "-100.0", "1000"],
            svec!["2", "-5", "-20.05", "825"],
            svec!["3", "0", "0.0", "10"],
            svec!["4", "5", "20.05", "-900"],
            svec!["5", "10", "100.0", "0"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv").arg("--dataset-stats");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "sort_order",
            "sortiness",
            "min_length",
            "max_length",
            "sum_length",
            "avg_length",
            "stddev_length",
            "variance_length",
            "cv_length",
            "mean",
            "sem",
            "geometric_mean",
            "harmonic_mean",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "max_precision",
            "sparsity",
            "qsv__value"
        ],
        svec![
            "col1",
            "Integer",
            "",
            "15",
            "1",
            "5",
            "4",
            "Ascending",
            "1",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "3",
            "0.6325",
            "2.6052",
            "2.1898",
            "1.4142",
            "2",
            "47.1405",
            "0",
            "",
            "0",
            ""
        ],
        svec![
            "col2",
            "Integer",
            "",
            "0",
            "-10",
            "10",
            "20",
            "Ascending",
            "1",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "0",
            "3.1623",
            "",
            "",
            "7.0711",
            "50",
            "",
            "0",
            "",
            "0",
            ""
        ],
        svec![
            "col3",
            "Float",
            "",
            "0",
            "-100.0",
            "100.0",
            "200",
            "Ascending",
            "1",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "0",
            "28.8472",
            "",
            "",
            "64.5043",
            "4160.801",
            "",
            "0",
            "2",
            "0",
            ""
        ],
        svec![
            "col4", "Integer", "", "935", "-900", "1000", "1900", "Unsorted", "-0.5", "", "", "",
            "", "", "", "", "187", "304.3603", "", "", "680.5703", "463176", "363.9414", "0", "",
            "0", ""
        ],
        svec![
            "qsv__rowcount",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "5"
        ],
        svec![
            "qsv__columncount",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "4"
        ],
        svec![
            "qsv__filesize_bytes",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "93"
        ],
        svec![
            "qsv__fingerprint_hash",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "9e162b617fe233c126f65b81136a6da6ea6367a63e520ed99f823f3fc1915ded"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn stats_output_tab_delimited() {
    let wrk = Workdir::new("stats_output_tab_delimited");

    wrk.create(
        "data.csv",
        vec![
            svec!["col1", "col2", "col3"],
            svec!["1", "4321", "01"],
            svec!["2", "3210", "02"],
            svec!["3", "2101", "03"],
            svec!["4", "1012", "04"],
            svec!["5", "0", "10"],
        ],
    );

    let out_file = wrk.path("output.tab").to_string_lossy().to_string();

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .args(&["--output", &out_file])
        .arg("--dataset-stats");

    wrk.assert_success(&mut cmd);

    let got = std::fs::read_to_string(out_file).unwrap();
    let expected = r#"field	type	is_ascii	sum	min	max	range	sort_order	sortiness	min_length	max_length	sum_length	avg_length	stddev_length	variance_length	cv_length	mean	sem	geometric_mean	harmonic_mean	stddev	variance	cv	nullcount	max_precision	sparsity	qsv__value
col1	Integer		15	1	5	4	Ascending	1								3	0.6325	2.6052	2.1898	1.4142	2	47.1405	0		0	
col2	Integer		10644	0	4321	4321	Descending	-1								2128.8	685.6979	0		1533.267	2350907.76	72.0249	0		0	
col3	String	true		01	10		Ascending	1	2	2	10	2	0	0	0								0		0	
qsv__rowcount																										5
qsv__columncount																										3
qsv__filesize_bytes																										62
qsv__fingerprint_hash																										cf6ba47ee7251a17b8d7789636b2c5d2302ea6e0fb69572f1eaf25aead083cef
"#;
    assert_eq!(got, expected);
}

#[test]
fn stats_output_ssv_delimited() {
    let wrk = Workdir::new("stats_output_ssv_delimited");

    wrk.create(
        "data.csv",
        vec![
            svec!["col1", "col2", "col3"],
            svec!["1", "4321", "01"],
            svec!["2", "3210", "02"],
            svec!["3", "2101", "03"],
            svec!["4", "1012", "04"],
            svec!["5", "0", "10"],
        ],
    );

    let out_file = wrk.path("output.ssv").to_string_lossy().to_string();

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .args(&["--output", &out_file])
        .arg("--dataset-stats");

    wrk.assert_success(&mut cmd);

    let got = std::fs::read_to_string(out_file).unwrap();
    let expected = r#"field;type;is_ascii;sum;min;max;range;sort_order;sortiness;min_length;max_length;sum_length;avg_length;stddev_length;variance_length;cv_length;mean;sem;geometric_mean;harmonic_mean;stddev;variance;cv;nullcount;max_precision;sparsity;qsv__value
col1;Integer;;15;1;5;4;Ascending;1;;;;;;;;3;0.6325;2.6052;2.1898;1.4142;2;47.1405;0;;0;
col2;Integer;;10644;0;4321;4321;Descending;-1;;;;;;;;2128.8;685.6979;0;;1533.267;2350907.76;72.0249;0;;0;
col3;String;true;;01;10;;Ascending;1;2;2;10;2;0;0;0;;;;;;;;0;;0;
qsv__rowcount;;;;;;;;;;;;;;;;;;;;;;;;;;5
qsv__columncount;;;;;;;;;;;;;;;;;;;;;;;;;;3
qsv__filesize_bytes;;;;;;;;;;;;;;;;;;;;;;;;;;62
qsv__fingerprint_hash;;;;;;;;;;;;;;;;;;;;;;;;;;cf6ba47ee7251a17b8d7789636b2c5d2302ea6e0fb69572f1eaf25aead083cef
"#;
    assert_eq!(got, expected);
}

#[test]
fn stats_output_csvsz_delimited() {
    let wrk = Workdir::new("stats_output_csvsz_delimited");

    wrk.create(
        "data.csv",
        vec![
            svec!["col1", "col2", "col3"],
            svec!["1", "4321", "01"],
            svec!["2", "3210", "02"],
            svec!["3", "2101", "03"],
            svec!["4", "1012", "04"],
            svec!["5", "0", "10"],
        ],
    );

    let out_file = wrk.path("output.csv.sz").to_string_lossy().to_string();

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .args(&["--output", &out_file])
        .arg("--dataset-stats");

    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("snappy");
    cmd.arg("decompress").arg(out_file.clone());

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"field,type,is_ascii,sum,min,max,range,sort_order,sortiness,min_length,max_length,sum_length,avg_length,stddev_length,variance_length,cv_length,mean,sem,geometric_mean,harmonic_mean,stddev,variance,cv,nullcount,max_precision,sparsity,qsv__value
col1,Integer,,15,1,5,4,Ascending,1,,,,,,,,3,0.6325,2.6052,2.1898,1.4142,2,47.1405,0,,0,
col2,Integer,,10644,0,4321,4321,Descending,-1,,,,,,,,2128.8,685.6979,0,,1533.267,2350907.76,72.0249,0,,0,
col3,String,true,,01,10,,Ascending,1,2,2,10,2,0,0,0,,,,,,,,0,,0,
qsv__rowcount,,,,,,,,,,,,,,,,,,,,,,,,,,5
qsv__columncount,,,,,,,,,,,,,,,,,,,,,,,,,,3
qsv__filesize_bytes,,,,,,,,,,,,,,,,,,,,,,,,,,62
qsv__fingerprint_hash,,,,,,,,,,,,,,,,,,,,,,,,,,cf6ba47ee7251a17b8d7789636b2c5d2302ea6e0fb69572f1eaf25aead083cef"#;
    assert_eq!(got, expected);
}

mod stats_infer_nothing {
    // Only test CSV data with headers.
    // Empty CSV data with no headers won't produce any statistical analysis.
    use super::test_stats;
    stats_test_headers!(stats_infer_nothing, "type", &[], "NULL");
}

mod stats_zero_cardinality {
    use super::test_stats;
    stats_test_headers!(stats_zero_cardinality, "cardinality", &[], "0");
}

mod stats_zero_mode {
    use super::test_stats;
    stats_test_headers!(stats_zero_mode, "mode", &[], "N/A");
}

mod stats_zero_mean {
    use super::test_stats;
    stats_test_headers!(stats_zero_mean, "mean", &[], "");
}

mod stats_zero_median {
    use super::test_stats;
    stats_test_headers!(stats_zero_median, "median", &[], "");
}

mod stats_zero_quartiles {
    use super::test_stats;
    stats_test_headers!(stats_zero_quartiles, "quartiles", &[], ",,,,,");
}

mod stats_header_fields {
    use super::test_stats;
    stats_test_headers!(stats_header_field_name, "field", &["a"], "header");
    stats_test_no_headers!(stats_header_no_field_name, "field", &["a"], "0");
}

#[test]
fn stats_vis_whitespace() {
    let wrk = Workdir::new("stats_vis_whitespace");

    // Create test data with various types of whitespace
    wrk.create(
        "data.csv",
        vec![
            svec!["col1", "col2", "col3"],
            svec!["value\t", "\tvalue", "\tvalue\t"], // Tabs
            svec!["value\r", "\rvalue", "\rvalue\r"], // Carriage returns
            svec!["value\n", "\nvalue", "\nvalue\n"], // Line feeds
            svec!["value\n", "\nvalue", "\nvalue\n"], // Line feeds repeat
            svec!["no_whitespace", "also_none", "clean"],
            svec![
                "the spaces in this field are visible as normal spaces",
                "    ",
                "the trailing spaces are left alone   "
            ],
            svec![
                "z obscure whitespace \u{000B} \u{000C} \u{0009} \u{0085} \u{200E} \u{200F} \
                 \u{2028} \u{2029} are also visible",
                "",
                "          "
            ],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--vis-whitespace")
        .arg("--everything")
        .arg("--dataset-stats")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Create expected output with visualized whitespace using the exact markers
    #[rustfmt::skip]
    let expected = vec![
        svec![
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "sort_order",
            "sortiness",
            "min_length",
            "max_length",
            "sum_length",
            "avg_length",
            "stddev_length",
            "variance_length",
            "cv_length",
            "mean",
            "sem",
            "geometric_mean",
            "harmonic_mean",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "max_precision",
            "sparsity",
            "mad",
            "lower_outer_fence",
            "lower_inner_fence",
            "q1",
            "q2_median",
            "q3",
            "iqr",
            "upper_inner_fence",
            "upper_outer_fence",
            "skewness",
            "cardinality",
            "uniqueness_ratio",
            "mode",
            "mode_count",
            "mode_occurrences",
            "antimode",
            "antimode_count",
            "antimode_occurrences",
            "percentiles",
            "qsv__value"
        ],
        svec!["col1", "String", "false", "", "no_whitespace", "z obscure whitespace 《⋮》 《␌》 《→》 《␤》 《␎》 《␏》 《␊》 《␍》 are also visible", "", "Unsorted", "0.3333", "6", "62", "152", "21.7143", "22.883", "523.6327", "1.0538", "", "", "", "", "", "", "", "0", "", "0", "", "", "", "", "", "", "", "", "", "", "6", "0.8571", "value《¶》", "1", "2", "no_whitespace|the spaces in this field are visible as normal spaces|value《→》|value《⏎》|z obscure whitespa...", "5", "1", "", ""],
        svec!["col2", "String", "true", "", "《→》value", "also_none", "", "Unsorted", "0.2", "0", "9", "37", "5.2857", "2.5475", "6.4898", "0.482", "", "", "", "", "", "", "", "1", "", "0.1429", "", "", "", "", "", "", "", "", "", "", "6", "0.8571", "《¶》value", "1", "2", "NULL|《→》value|《⏎》value|    |also_none", "5", "1", "", ""], 
        svec!["col3", "String", "true", "", "《→》value《→》", "the trailing spaces are left alone   ", "", "Unsorted", "0.3333", "5", "37", "80", "11.4286", "10.5269", "110.8163", "0.9211", "", "", "", "", "", "", "", "0", "", "0", "", "", "", "", "", "", "", "", "", "", "6", "0.8571", "《¶》value《¶》", "1", "2", "《→》value《→》|《⏎》value《⏎》|          |clean|the trailing spaces are left alone   ", "5", "1", "", ""], 
        svec!["qsv__rowcount", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "7"], 
        svec!["qsv__columncount", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "3"], 
        svec!["qsv__filesize_bytes", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "323"], 
        svec!["qsv__fingerprint_hash", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "39f84fb60ced1488a91fc3a7a6e22692dafb414e0d43f09ab768c5f9981db630"]
    ];
    //
    assert_eq!(got, expected);
}

#[test]
fn stats_percentiles() {
    let wrk = Workdir::new("stats_percentiles");
    wrk.create(
        "data.csv",
        vec![
            svec!["n"],
            svec!["1"],
            svec!["2"],
            svec!["3"],
            svec!["4"],
            svec!["5"],
            svec!["6"],
            svec!["7"],
            svec!["8"],
            svec!["9"],
            svec!["10"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("10,25,50,75,90");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let expected = vec![
        svec![
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "sort_order",
            "sortiness",
            "min_length",
            "max_length",
            "sum_length",
            "avg_length",
            "stddev_length",
            "variance_length",
            "cv_length",
            "mean",
            "sem",
            "geometric_mean",
            "harmonic_mean",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "max_precision",
            "sparsity",
            "percentiles",
        ],
        svec![
            "n",
            "Integer",
            "",
            "55",
            "1",
            "10",
            "9",
            "Ascending",
            "1",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "5.5",
            "0.9083",
            "4.5287",
            "3.4142",
            "2.8723",
            "8.25",
            "52.2233",
            "0",
            "",
            "0",
            "1|3|5|8|9",
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn stats_percentiles_floats() {
    let wrk = Workdir::new("stats_percentiles");
    wrk.create(
        "data.csv",
        vec![
            svec!["n"],
            svec!["1"],
            svec!["2"],
            svec!["3"],
            svec!["4"],
            svec!["5"],
            svec!["6"],
            svec!["7"],
            svec!["8"],
            svec!["9"],
            svec!["10"],
            svec!["11"],
            svec!["12"],
            svec!["13"],
            svec!["14"],
            svec!["15"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("10.5,25.25,50.75,75.6,90.1");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let expected = vec![
        svec![
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "sort_order",
            "sortiness",
            "min_length",
            "max_length",
            "sum_length",
            "avg_length",
            "stddev_length",
            "variance_length",
            "cv_length",
            "mean",
            "sem",
            "geometric_mean",
            "harmonic_mean",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "max_precision",
            "sparsity",
            "percentiles",
        ],
        svec![
            "n",
            "Integer",
            "",
            "120",
            "1",
            "15",
            "14",
            "Ascending",
            "1",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "8",
            "1.1155",
            "6.4234",
            "4.5205",
            "4.3205",
            "18.6667",
            "54.0062",
            "0",
            "",
            "0",
            "2|4|8|12|14"
        ],
    ];

    //
    assert_eq!(got, expected);
}

#[test]
fn stats_percentiles_with_dates() {
    let wrk = Workdir::new("stats_percentiles_dates");
    wrk.create(
        "data.csv",
        vec![
            svec!["date"],
            svec!["2020-01-01"],
            svec!["2020-02-01"],
            svec!["2020-03-01"],
            svec!["2020-04-01"],
            svec!["2020-05-01"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("25,50,75")
        .arg("--infer-dates");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let expected = vec![
        svec![
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "sort_order",
            "sortiness",
            "min_length",
            "max_length",
            "sum_length",
            "avg_length",
            "stddev_length",
            "variance_length",
            "cv_length",
            "mean",
            "sem",
            "geometric_mean",
            "harmonic_mean",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "max_precision",
            "sparsity",
            "percentiles",
        ],
        svec![
            "date",
            "Date",
            "",
            "",
            "2020-01-01",
            "2020-05-01",
            "121",
            "Ascending",
            "1",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "2020-03-01",
            "19.10099",
            "18322.55022",
            "18322.50044",
            "42.71112",
            "1824.24",
            "0.2331",
            "0",
            "",
            "0",
            "2020-02-01|2020-03-01|2020-04-01",
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn stats_percentiles_with_nulls() {
    let wrk = Workdir::new("stats_percentiles_nulls");
    wrk.create(
        "data.csv",
        vec![
            svec!["n"],
            svec!["1"],
            svec![""],
            svec!["3"],
            svec![""],
            svec!["5"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("25,50,75");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let expected = vec![
        svec![
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "sort_order",
            "sortiness",
            "min_length",
            "max_length",
            "sum_length",
            "avg_length",
            "stddev_length",
            "variance_length",
            "cv_length",
            "mean",
            "sem",
            "geometric_mean",
            "harmonic_mean",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "max_precision",
            "sparsity",
            "percentiles",
        ],
        svec![
            "n",
            "Integer",
            "",
            "9",
            "1",
            "5",
            "4",
            "Ascending",
            "1",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "3",
            "0.9428",
            "2.4662",
            "1.9565",
            "1.633",
            "2.6667",
            "54.4331",
            "2",
            "",
            "0.4",
            "1|3|5",
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn stats_percentiles_mixed_types() {
    let wrk = Workdir::new("stats_percentiles_mixed");
    wrk.create(
        "data.csv",
        vec![
            svec!["mixed"],
            svec!["1"],
            svec!["2.5"],
            svec!["abc"],
            svec!["3"],
            svec!["4.7"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("25,50,75");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let expected = vec![
        svec![
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "sort_order",
            "sortiness",
            "min_length",
            "max_length",
            "sum_length",
            "avg_length",
            "stddev_length",
            "variance_length",
            "cv_length",
            "mean",
            "sem",
            "geometric_mean",
            "harmonic_mean",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "max_precision",
            "sparsity",
            "percentiles",
        ],
        svec![
            "mixed", "String", "true", "", "3", "abc", "", "Unsorted", "0", "1", "3", "7", "1.4",
            "0.9428", "0.8889", "0.6734", "", "", "", "", "", "", "", "0", "", "0", "",
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn stats_percentiles_edge_cases() {
    let wrk = Workdir::new("stats_percentiles_edge");
    wrk.create(
        "data.csv",
        vec![
            svec!["n"],
            svec!["0.0000001"],
            svec!["1000000"],
            svec!["-999999"],
            svec!["0.0000099"],
            svec!["999999"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("10,25,50,75,90");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let expected = vec![
        svec![
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "sort_order",
            "sortiness",
            "min_length",
            "max_length",
            "sum_length",
            "avg_length",
            "stddev_length",
            "variance_length",
            "cv_length",
            "mean",
            "sem",
            "geometric_mean",
            "harmonic_mean",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "max_precision",
            "sparsity",
            "percentiles",
        ],
        svec![
            "n",
            "Float",
            "",
            "1000000",
            "-999999.0",
            "1000000.0",
            "1999999",
            "Unsorted",
            "0.5",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "200000",
            "334663.7716",
            "",
            "",
            "748330.9428",
            "559999199999.6",
            "374.1655",
            "0",
            "4",
            "0",
            "-999999|0|0|999999|1000000"
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn stats_percentiles_custom_list() {
    let wrk = Workdir::new("stats_percentiles_custom");
    wrk.create(
        "data.csv",
        vec![
            svec!["n"],
            svec!["1"],
            svec!["2"],
            svec!["3"],
            svec!["4"],
            svec!["5"],
            svec!["6"],
            svec!["7"],
            svec!["8"],
            svec!["9"],
            svec!["10"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("1,5,33.3,66.6,95,99");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let expected = vec![
        svec![
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "sort_order",
            "sortiness",
            "min_length",
            "max_length",
            "sum_length",
            "avg_length",
            "stddev_length",
            "variance_length",
            "cv_length",
            "mean",
            "sem",
            "geometric_mean",
            "harmonic_mean",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "max_precision",
            "sparsity",
            "percentiles",
        ],
        svec![
            "n",
            "Integer",
            "",
            "55",
            "1",
            "10",
            "9",
            "Ascending",
            "1",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "5.5",
            "0.9083",
            "4.5287",
            "3.4142",
            "2.8723",
            "8.25",
            "52.2233",
            "0",
            "",
            "0",
            "1|1|4|7|10|10",
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn stats_percentiles_single_value() {
    let wrk = Workdir::new("stats_percentiles_single");
    wrk.create("data.csv", vec![svec!["n"], svec!["42"]]);

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("25,50,75");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let expected = vec![
        svec![
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "sort_order",
            "sortiness",
            "min_length",
            "max_length",
            "sum_length",
            "avg_length",
            "stddev_length",
            "variance_length",
            "cv_length",
            "mean",
            "sem",
            "geometric_mean",
            "harmonic_mean",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "max_precision",
            "sparsity",
            "percentiles",
        ],
        svec![
            "n", "Integer", "", "42", "42", "42", "0", "Unsorted", "0", "", "", "", "", "", "", "",
            "42", "0", "42", "42", "0", "0", "0", "0", "", "0", "42|42|42",
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn stats_infer_boolean_prefix_pattern() {
    let wrk = Workdir::new("stats_infer_boolean_prefix_pattern");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--boolean-patterns")
        .arg("t*:f*")
        .arg("--dataset-stats")
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-10-boolean-tf-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_infer_boolean_multiple_patterns() {
    let wrk = Workdir::new("stats_infer_boolean_multiple_patterns");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--boolean-patterns")
        .arg("true:false,t*:f*,y*:n*")
        .arg("--dataset-stats")
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-10-boolean-tf-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_infer_boolean_case_insensitive() {
    let wrk = Workdir::new("stats_infer_boolean_case_insensitive");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--boolean-patterns")
        .arg("TRUE:FALSE")
        .arg("--dataset-stats")
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-10-boolean-tf-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_infer_boolean_long_patterns() {
    let wrk = Workdir::new("stats_infer_boolean_long_patterns");

    // Create test data with values that won't match the truthy:falsy pattern
    wrk.create(
        "data.csv",
        vec![
            svec!["col1"],
            svec!["true"],
            svec!["false"],
            svec!["true"],
            svec!["false"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--boolean-patterns")
        .arg("truthy:falsy")
        .arg("--dataset-stats")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);

    // Should not be inferred as boolean since values don't match pattern
    assert!(!got.contains("Boolean"));
    assert!(got.contains("String"));
}

#[test]
fn stats_infer_boolean_cardinality_three() {
    let wrk = Workdir::new("stats_infer_boolean_cardinality_three");

    // Create a test file with 3 distinct values that would match boolean patterns
    wrk.create(
        "data.csv",
        vec![
            svec!["col1"],
            svec!["true"],
            svec!["truthy"],
            svec!["false"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--boolean-patterns")
        .arg("true*:false*")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);

    // Should not be inferred as boolean because cardinality is 3
    assert!(!got.contains("Boolean"));
    assert!(got.contains("String"));
}

#[test]
fn stats_infer_boolean_empty_pattern() {
    let wrk = Workdir::new("stats_infer_boolean_empty_pattern");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--boolean-patterns")
        .arg("")
        .arg("--dataset-stats")
        .arg(test_file);

    // This should fail with an error
    wrk.assert_err(&mut cmd);
}

#[test]
fn stats_infer_boolean_missing_colon() {
    let wrk = Workdir::new("stats_infer_boolean_missing_colon");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--boolean-patterns")
        .arg("truefalse")
        .arg("--dataset-stats")
        .arg(test_file);

    // This should fail with an error
    wrk.assert_err(&mut cmd);
}

#[test]
fn stats_infer_boolean_missing_true_pattern() {
    let wrk = Workdir::new("stats_infer_boolean_missing_true_pattern");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--boolean-patterns")
        .arg(":false")
        .arg("--dataset-stats")
        .arg(test_file);

    // This should fail with an error
    wrk.assert_err(&mut cmd);
}

#[test]
fn stats_infer_boolean_missing_false_pattern() {
    let wrk = Workdir::new("stats_infer_boolean_missing_false_pattern");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean")
        .arg("--boolean-patterns")
        .arg("true:")
        .arg("--dataset-stats")
        .arg(test_file);

    // This should fail with an error
    wrk.assert_err(&mut cmd);
}

#[test]
fn stats_issue_2668_semicolon_separator() {
    let wrk = Workdir::new("stats_issue_2668_semicolon_separator");
    wrk.create("data.csv", vec![svec!["h1;h2;h3;h4"], svec!["1;2;3;4"]]);

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv");

    // should not fail, and just treat the data as a single column
    // as the default delimiter is comma, and the separator is semicolon
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "sort_order",
            "sortiness",
            "min_length",
            "max_length",
            "sum_length",
            "avg_length",
            "stddev_length",
            "variance_length",
            "cv_length",
            "mean",
            "sem",
            "geometric_mean",
            "harmonic_mean",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "max_precision",
            "sparsity"
        ],
        svec![
            "h1;h2;h3;h4",
            "String",
            "true",
            "",
            "1;2;3;4",
            "1;2;3;4",
            "",
            "Unsorted",
            "0",
            "7",
            "7",
            "7",
            "7",
            "0",
            "0",
            "0",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "0",
            "",
            "0"
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn stats_string_max_length() {
    let wrk = Workdir::new("stats_string_max_length");

    // Create test data with long strings
    wrk.create(
        "data.csv",
        vec![
            svec!["col1", "col2"],
            svec!["short", "very_short"],
            svec!["medium_length_string", "medium_length_string"],
            svec![
                "this_is_a_very_long_string_that_should_be_truncated",
                "another_very_long_string_that_should_be_truncated"
            ],
        ],
    );

    // Run stats with QSV_STATS_STRING_MAX_LENGTH set to 10
    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv").env("QSV_STATS_STRING_MAX_LENGTH", "10");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Print the output for debugging
    // println!("Output with QSV_STATS_STRING_MAX_WIDTH=10:");
    // for row in &got {
    //     println!("{:?}", row);
    // }

    // Find the min and max values in the output
    let mut min_value = String::new();
    let mut max_value = String::new();

    // Find the row for col1
    for row in &got {
        if row.len() > 0 && row[0] == "col1" {
            // The min and max values are in columns 4 and 5 (0-indexed)
            if row.len() > 5 {
                min_value = row[4].clone();
                max_value = row[5].clone();
            }
            break;
        }
    }

    // Check that the long string was truncated
    assert_eq!(min_value, "medium_len...");
    assert_eq!(max_value, "this_is_a_...");

    // Run stats with QSV_STATS_STRING_MAX_LENGTH set to 20
    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv").env("QSV_STATS_STRING_MAX_LENGTH", "20");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Find the min and max values in the output
    let mut min_value = String::new();
    let mut max_value = String::new();

    // Find the row for col1
    for row in &got {
        if row.len() > 0 && row[0] == "col1" {
            // The min and max values are in columns 4 and 5 (0-indexed)
            if row.len() > 5 {
                min_value = row[4].clone();
                max_value = row[5].clone();
            }
            break;
        }
    }

    // Check that the long string was truncated with a longer width
    assert_eq!(min_value, "medium_length_string");
    assert_eq!(max_value, "this_is_a_very_long_...");

    // Run stats without the environment variable
    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Find the min and max values in the output
    let mut min_value = String::new();
    let mut max_value = String::new();

    // Find the row for col1
    for row in &got {
        if row.len() > 0 && row[0] == "col1" {
            // The min and max values are in columns 4 and 5 (0-indexed)
            if row.len() > 5 {
                min_value = row[4].clone();
                max_value = row[5].clone();
            }
            break;
        }
    }

    // Check that the long string was not truncated
    assert_eq!(min_value, "medium_length_string");
    assert_eq!(
        max_value,
        "this_is_a_very_long_string_that_should_be_truncated"
    );
}
