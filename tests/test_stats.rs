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
// stats_tests!(stats_min_mix, "min", &["1", "2", "a", "1.1", "b"], "1.1");
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
            "e76ebf94f99791f11c35c429d54958b7317ac89b5511bf47dea80b7244a413da"
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
            "n_negative",
            "n_zero",
            "n_positive",
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
            "0",
            "0",
            "5",
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
            "2",
            "1",
            "2",
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
            "1",
            "2",
            "2",
            "0",
            ""
        ],
        svec![
            "col4", "Integer", "", "935", "-900", "1000", "1900", "Unsorted", "-0.5", "", "", "",
            "", "", "", "", "187", "304.3603", "", "", "680.5703", "463176", "363.9414", "0", "1",
            "1", "3", "", "0", ""
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
            "",
            "",
            "",
            "e710e39fee131dfec40c956e1c7acd9c290fdd1e54538ba7f7cd1a1487604cb7"
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
    let expected = r#"field	type	is_ascii	sum	min	max	range	sort_order	sortiness	min_length	max_length	sum_length	avg_length	stddev_length	variance_length	cv_length	mean	sem	geometric_mean	harmonic_mean	stddev	variance	cv	nullcount	n_negative	n_zero	n_positive	max_precision	sparsity	qsv__value
col1	Integer		15	1	5	4	Ascending	1								3	0.6325	2.6052	2.1898	1.4142	2	47.1405	0	0	0	5		0	
col2	Integer		10644	0	4321	4321	Descending	-1								2128.8	685.6979	0		1533.267	2350907.76	72.0249	0	0	1	4		0	
col3	String	true		01	10		Ascending	1	2	2	10	2	0	0	0								0					0	
qsv__rowcount																													5
qsv__columncount																													3
qsv__filesize_bytes																													62
qsv__fingerprint_hash																													e31c3350e0b0bfed571dd7df649eef11f47cb07d3e3e11890f07da6bfbd6dd62
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
    let expected = r#"field;type;is_ascii;sum;min;max;range;sort_order;sortiness;min_length;max_length;sum_length;avg_length;stddev_length;variance_length;cv_length;mean;sem;geometric_mean;harmonic_mean;stddev;variance;cv;nullcount;n_negative;n_zero;n_positive;max_precision;sparsity;qsv__value
col1;Integer;;15;1;5;4;Ascending;1;;;;;;;;3;0.6325;2.6052;2.1898;1.4142;2;47.1405;0;0;0;5;;0;
col2;Integer;;10644;0;4321;4321;Descending;-1;;;;;;;;2128.8;685.6979;0;;1533.267;2350907.76;72.0249;0;0;1;4;;0;
col3;String;true;;01;10;;Ascending;1;2;2;10;2;0;0;0;;;;;;;;0;;;;;0;
qsv__rowcount;;;;;;;;;;;;;;;;;;;;;;;;;;;;;5
qsv__columncount;;;;;;;;;;;;;;;;;;;;;;;;;;;;;3
qsv__filesize_bytes;;;;;;;;;;;;;;;;;;;;;;;;;;;;;62
qsv__fingerprint_hash;;;;;;;;;;;;;;;;;;;;;;;;;;;;;e31c3350e0b0bfed571dd7df649eef11f47cb07d3e3e11890f07da6bfbd6dd62
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
    let expected = r#"field,type,is_ascii,sum,min,max,range,sort_order,sortiness,min_length,max_length,sum_length,avg_length,stddev_length,variance_length,cv_length,mean,sem,geometric_mean,harmonic_mean,stddev,variance,cv,nullcount,n_negative,n_zero,n_positive,max_precision,sparsity,qsv__value
col1,Integer,,15,1,5,4,Ascending,1,,,,,,,,3,0.6325,2.6052,2.1898,1.4142,2,47.1405,0,0,0,5,,0,
col2,Integer,,10644,0,4321,4321,Descending,-1,,,,,,,,2128.8,685.6979,0,,1533.267,2350907.76,72.0249,0,0,1,4,,0,
col3,String,true,,01,10,,Ascending,1,2,2,10,2,0,0,0,,,,,,,,0,,,,,0,
qsv__rowcount,,,,,,,,,,,,,,,,,,,,,,,,,,,,,5
qsv__columncount,,,,,,,,,,,,,,,,,,,,,,,,,,,,,3
qsv__filesize_bytes,,,,,,,,,,,,,,,,,,,,,,,,,,,,,62
qsv__fingerprint_hash,,,,,,,,,,,,,,,,,,,,,,,,,,,,,e31c3350e0b0bfed571dd7df649eef11f47cb07d3e3e11890f07da6bfbd6dd62"#;
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
        svec!["field", "type", "is_ascii", "sum", "min", "max", "range", "sort_order", "sortiness", "min_length", "max_length", "sum_length", "avg_length", "stddev_length", "variance_length", "cv_length", "mean", "sem", "geometric_mean", "harmonic_mean", "stddev", "variance", "cv", "nullcount", "n_negative", "n_zero", "n_positive", "max_precision", "sparsity", "mad", "lower_outer_fence", "lower_inner_fence", "q1", "q2_median", "q3", "iqr", "upper_inner_fence", "upper_outer_fence", "skewness", "cardinality", "uniqueness_ratio", "mode", "mode_count", "mode_occurrences", "antimode", "antimode_count", "antimode_occurrences", "percentiles", "qsv__value"], 
        svec!["col1", "String", "false", "", "no_whitespace", "z obscure whitespace 《⋮》 《␌》 《→》 《␤》 《␎》 《␏》 《␊》 《␍》 are also visible", "", "Unsorted", "0.3333", "6", "62", "152", "21.7143", "22.883", "523.6327", "1.0538", "", "", "", "", "", "", "", "0", "", "", "", "", "0", "", "", "", "", "", "", "", "", "", "", "6", "0.8571", "value《¶》", "1", "2", "no_whitespace|the spaces in this field are visible as normal spaces|value《→》|value《⏎》|z obscure whitespa...", "5", "1", "", ""], 
        svec!["col2", "String", "true", "", "《→》value", "also_none", "", "Unsorted", "0.2", "0", "9", "37", "5.2857", "2.5475", "6.4898", "0.482", "", "", "", "", "", "", "", "1", "", "", "", "", "0.1429", "", "", "", "", "", "", "", "", "", "", "6", "0.8571", "《¶》value", "1", "2", "NULL|《→》value|《⏎》value|    |also_none", "5", "1", "", ""], 
        svec!["col3", "String", "true", "", "《→》value《→》", "the trailing spaces are left alone   ", "", "Unsorted", "0.3333", "5", "37", "80", "11.4286", "10.5269", "110.8163", "0.9211", "", "", "", "", "", "", "", "0", "", "", "", "", "0", "", "", "", "", "", "", "", "", "", "", "6", "0.8571", "《¶》value《¶》", "1", "2", "《→》value《→》|《⏎》value《⏎》|          |clean|the trailing spaces are left alone   ", "5", "1", "", ""], 
        svec!["qsv__rowcount", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "7"], svec!["qsv__columncount", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "3"], 
        svec!["qsv__filesize_bytes", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "323"], svec!["qsv__fingerprint_hash", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "04b6264caa1763c923b7fd9826a023b29135e96c33921b89a95b554fcbefccfa"]];
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
            "n_negative",
            "n_zero",
            "n_positive",
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
            "0",
            "0",
            "10",
            "",
            "0",
            "10: 1|25: 3|50: 5|75: 8|90: 9",
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
            "n_negative",
            "n_zero",
            "n_positive",
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
            "0",
            "0",
            "15",
            "",
            "0",
            "10.5: 2|25.25: 4|50.75: 8|75.6: 12|90.1: 14"
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
            "n_negative",
            "n_zero",
            "n_positive",
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
            "",
            "",
            "",
            "0",
            "25: 2020-02-01|50: 2020-03-01|75: 2020-04-01",
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
            "n_negative",
            "n_zero",
            "n_positive",
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
            "0",
            "0",
            "3",
            "",
            "0.4",
            "25: 1|50: 3|75: 5",
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
            "n_negative",
            "n_zero",
            "n_positive",
            "max_precision",
            "sparsity",
            "percentiles",
        ],
        svec![
            "mixed", "String", "true", "", "3", "abc", "", "Unsorted", "0", "1", "3", "7", "1.4",
            "0.9428", "0.8889", "0.6734", "", "", "", "", "", "", "", "0", "", "", "", "", "0", "",
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
            "n_negative",
            "n_zero",
            "n_positive",
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
            "1",
            "0",
            "4",
            "4",
            "0",
            "10: -999999|25: 0|50: 0|75: 999999|90: 1000000"
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
            "n_negative",
            "n_zero",
            "n_positive",
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
            "0",
            "0",
            "10",
            "",
            "0",
            "1: 1|5: 1|33.3: 4|66.6: 7|95: 10|99: 10",
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
            "n_negative",
            "n_zero",
            "n_positive",
            "max_precision",
            "sparsity",
            "percentiles",
        ],
        svec![
            "n",
            "Integer",
            "",
            "42",
            "42",
            "42",
            "0",
            "Unsorted",
            "0",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "42",
            "0",
            "42",
            "42",
            "0",
            "0",
            "0",
            "0",
            "0",
            "0",
            "1",
            "",
            "0",
            "25: 42|50: 42|75: 42",
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn stats_percentiles_deciles_lowercase() {
    let wrk = Workdir::new("stats_percentiles_deciles_lowercase");
    wrk.create(
        "data.csv",
        vec![
            svec!["n"],
            svec!["4"],
            svec!["8"],
            svec!["1"],
            svec!["5"],
            svec!["3"],
            svec!["6"],
            svec!["7"],
            svec!["9"],
            svec!["2"],
            svec!["10"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("deciles");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    // Find the percentiles column value
    let headers = &got[0];
    let percentiles_idx = headers
        .iter()
        .position(|h| h == "percentiles")
        .expect("percentiles column should exist");
    let percentiles_value = &got[1][percentiles_idx];

    // Deciles should expand to: 10,20,30,40,50,60,70,80,90 (i.e., the 10th, 20th, ..., 90th
    // percentiles; 9 values) Verify we have 9 percentile values separated by |
    let percentile_count = percentiles_value.matches('|').count() + 1;
    assert_eq!(
        percentile_count, 9,
        "Deciles should produce 9 percentile values"
    );
    // Verify the values are reasonable (should contain values from the dataset)
    assert!(
        percentiles_value.contains("1"),
        "Should contain first value"
    );
    assert!(percentiles_value.contains("9"), "Should contain last value");
}

#[test]
fn stats_percentiles_deciles_uppercase() {
    let wrk = Workdir::new("stats_percentiles_deciles_uppercase");
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
        .arg("DECILES");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let headers = &got[0];
    let percentiles_idx = headers
        .iter()
        .position(|h| h == "percentiles")
        .expect("percentiles column should exist");
    let percentiles_value = &got[1][percentiles_idx];

    // Should work the same as lowercase - verify structure
    let percentile_count = percentiles_value.matches('|').count() + 1;
    assert_eq!(
        percentile_count, 9,
        "Deciles should produce 9 percentile values"
    );
}

#[test]
fn stats_percentiles_deciles_mixed_case() {
    let wrk = Workdir::new("stats_percentiles_deciles_mixed_case");
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
        .arg("DeCiLeS");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let headers = &got[0];
    let percentiles_idx = headers
        .iter()
        .position(|h| h == "percentiles")
        .expect("percentiles column should exist");
    let percentiles_value = &got[1][percentiles_idx];

    // Should work the same as lowercase (case-insensitive) - verify structure
    let percentile_count = percentiles_value.matches('|').count() + 1;
    assert_eq!(
        percentile_count, 9,
        "Deciles should produce 9 percentile values"
    );
}

#[test]
fn stats_percentiles_quintiles_lowercase() {
    let wrk = Workdir::new("stats_percentiles_quintiles_lowercase");
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
        .arg("quintiles");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let headers = &got[0];
    let percentiles_idx = headers
        .iter()
        .position(|h| h == "percentiles")
        .expect("percentiles column should exist");
    let percentiles_value = &got[1][percentiles_idx];

    // Quintiles should expand to: 20,40,60,80 (4 values)
    // Verify we have 4 percentile values separated by |
    let percentile_count = percentiles_value.matches('|').count() + 1;
    assert_eq!(
        percentile_count, 4,
        "Quintiles should produce 4 percentile values"
    );
    // Verify the values are reasonable
    assert!(
        percentiles_value.contains("2") || percentiles_value.contains("4"),
        "Should contain reasonable values"
    );
}

#[test]
fn stats_percentiles_quintiles_uppercase() {
    let wrk = Workdir::new("stats_percentiles_quintiles_uppercase");
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
        .arg("QUINTILES");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let headers = &got[0];
    let percentiles_idx = headers
        .iter()
        .position(|h| h == "percentiles")
        .expect("percentiles column should exist");
    let percentiles_value = &got[1][percentiles_idx];

    // Should work the same as lowercase - verify structure
    let percentile_count = percentiles_value.matches('|').count() + 1;
    assert_eq!(
        percentile_count, 4,
        "Quintiles should produce 4 percentile values"
    );
}

#[test]
fn stats_percentiles_quintiles_mixed_case() {
    let wrk = Workdir::new("stats_percentiles_quintiles_mixed_case");
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
        .arg("QuInTiLeS");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let headers = &got[0];
    let percentiles_idx = headers
        .iter()
        .position(|h| h == "percentiles")
        .expect("percentiles column should exist");
    let percentiles_value = &got[1][percentiles_idx];

    // Should work the same as lowercase (case-insensitive) - verify structure
    let percentile_count = percentiles_value.matches('|').count() + 1;
    assert_eq!(
        percentile_count, 4,
        "Quintiles should produce 4 percentile values"
    );
}

#[test]
fn stats_percentiles_deciles_with_more_values() {
    let wrk = Workdir::new("stats_percentiles_deciles_more_values");
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
            svec!["16"],
            svec!["17"],
            svec!["18"],
            svec!["19"],
            svec!["20"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("deciles");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let headers = &got[0];
    let percentiles_idx = headers
        .iter()
        .position(|h| h == "percentiles")
        .expect("percentiles column should exist");
    let percentiles_value = &got[1][percentiles_idx];

    // Deciles: 10,20,30,40,50,60,70,80,90 (9 values)
    // Verify we have 9 percentile values
    let percentile_count = percentiles_value.matches('|').count() + 1;
    assert_eq!(
        percentile_count, 9,
        "Deciles should produce 9 percentile values"
    );
    // Verify values are from the dataset range
    assert!(
        percentiles_value.contains("2") || percentiles_value.contains("1"),
        "Should contain values from dataset"
    );
    assert!(
        percentiles_value.contains("20")
            || percentiles_value.contains("18")
            || percentiles_value.contains("19"),
        "Should contain values near end of dataset"
    );
}

#[test]
fn stats_percentiles_quintiles_with_more_values() {
    let wrk = Workdir::new("stats_percentiles_quintiles_more_values");
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
            svec!["16"],
            svec!["17"],
            svec!["18"],
            svec!["19"],
            svec!["20"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("quintiles");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let headers = &got[0];
    let percentiles_idx = headers
        .iter()
        .position(|h| h == "percentiles")
        .expect("percentiles column should exist");
    let percentiles_value = &got[1][percentiles_idx];

    // Quintiles: 20,40,60,80 (4 values)
    // Verify we have 4 percentile values
    let percentile_count = percentiles_value.matches('|').count() + 1;
    assert_eq!(
        percentile_count, 4,
        "Quintiles should produce 4 percentile values"
    );
    // Verify values are from the dataset range
    assert!(
        percentiles_value.contains("4") || percentiles_value.contains("8"),
        "Should contain reasonable values"
    );
}

#[test]
fn stats_percentiles_regular_list_still_works() {
    let wrk = Workdir::new("stats_percentiles_regular_list");
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

    // Regular percentile list should still work (not a special value)
    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("25,50,75");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(got.len() > 0);

    let headers = &got[0];
    let percentiles_idx = headers
        .iter()
        .position(|h| h == "percentiles")
        .expect("percentiles column should exist");
    let percentiles_value = &got[1][percentiles_idx];

    // Regular list should work: 25,50,75 -> 3|5|8
    assert_eq!(percentiles_value, "25: 3|50: 5|75: 8");
}

#[test]
fn stats_percentiles_invalid_list() {
    let wrk = Workdir::new("stats_percentiles_invalid_list");
    wrk.create(
        "data.csv",
        vec![svec!["n"], svec!["1"], svec!["2"], svec!["3"]],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("10,twenty,30,40,50,60,70,80,90,100");
    wrk.assert_err(&mut cmd);
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
            "n_negative",
            "n_zero",
            "n_positive",
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
            "",
            "",
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

#[test]
fn stats_memory_aware_chunking_dynamic() {
    let wrk = Workdir::new("stats_memory_aware_chunking_dynamic");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Create index to enable parallel processing
    let mut cmd = wrk.command("index");
    cmd.arg(test_file.clone());
    wrk.assert_success(&mut cmd);

    // Run stats with QSV_STATS_CHUNK_MEMORY_MB=0 (dynamic sizing)
    // and --everything to trigger non-streaming stats
    let mut cmd = wrk.command("stats");
    cmd.arg("--everything")
        .env("QSV_STATS_CHUNK_MEMORY_MB", "0")
        .arg(test_file);

    // Verify stats computation succeeds and output is correct
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty());
    // Verify we have headers
    assert!(got.len() > 1);
}

#[test]
fn stats_memory_aware_chunking_fixed_limit() {
    let wrk = Workdir::new("stats_memory_aware_chunking_fixed_limit");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Create index to enable parallel processing
    let mut cmd = wrk.command("index");
    cmd.arg(test_file.clone());
    wrk.assert_success(&mut cmd);

    // Run stats with QSV_STATS_CHUNK_MEMORY_MB set to 100MB (fixed limit)
    // and --everything to trigger non-streaming stats
    let mut cmd = wrk.command("stats");
    cmd.arg("--everything")
        .env("QSV_STATS_CHUNK_MEMORY_MB", "100")
        .arg(test_file);

    // Verify stats computation succeeds with fixed memory limit
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty());
    assert!(got.len() > 1);
}

#[test]
fn stats_memory_aware_chunking_unset_streaming() {
    let wrk = Workdir::new("stats_memory_aware_chunking_unset_streaming");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Create index to enable parallel processing
    let mut cmd = wrk.command("index");
    cmd.arg(test_file.clone());
    wrk.assert_success(&mut cmd);

    // Run stats without QSV_STATS_CHUNK_MEMORY_MB and without --everything
    // This should use CPU-based chunking (streaming stats only)
    let mut cmd = wrk.command("stats");
    cmd.arg(test_file);

    // Verify stats computation succeeds
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty());
    assert!(got.len() > 1);
}

#[test]
fn stats_memory_aware_chunking_unset_non_streaming() {
    let wrk = Workdir::new("stats_memory_aware_chunking_unset_non_streaming");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Create index to enable parallel processing
    let mut cmd = wrk.command("index");
    cmd.arg(test_file.clone());
    wrk.assert_success(&mut cmd);

    // Run stats without QSV_STATS_CHUNK_MEMORY_MB but with --cardinality
    // This should automatically enable dynamic memory-aware chunking
    let mut cmd = wrk.command("stats");
    cmd.arg("--cardinality").arg(test_file);

    // Verify stats computation succeeds
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty());
    assert!(got.len() > 1);
}

#[test]
#[ignore = "Creates a 20gb file which takes a long time to run"] // TODO: fix this test
fn stats_auto_index_creation_on_oom() {
    use std::path::Path;

    let wrk = Workdir::new("stats_auto_index_creation_on_oom");

    // Create a larger CSV file to increase chance of triggering OOM check
    // We'll create a file with many rows to make it large enough
    // Target size: at least 10MB to trigger OOM check on most systems
    let mut data = vec![svec!["col1", "col2", "col3", "col4", "col5"]];
    // Create ~10 million rows with reasonably sized data (~200 bytes per row * 10 million rows = 20
    // GB)
    for i in 0..10_000_000 {
        data.push(vec![
            format!("value_{}_with_some_padding_to_make_it_larger", i),
            format!("another_value_{}_with_more_data", i),
            format!("data_{}", i),
            format!("field_{}_content", i),
            format!("final_field_{}_with_additional_text", i),
        ]);
    }
    let test_file = wrk.path("large_data.csv");
    wrk.create("large_data.csv", data);

    // Verify index does not exist initially
    // Use the same path construction as util::idx_path (appends .idx to full path)
    let index_path = format!("{}.idx", test_file.display());
    let index_file = Path::new(&index_path);
    if index_file.exists() {
        std::fs::remove_file(&index_file).unwrap();
    }
    assert!(!index_file.exists(), "Index should not exist initially");

    // Simulate OOM by setting QSV_FREEMEMORY_HEADROOM_PCT very high (90%)
    // and using --memcheck to force conservative mode (stricter check)
    // This will make mem_file_check fail for sequential processing
    // Request non-streaming stats to trigger the OOM check
    let mut cmd = wrk.command("stats");
    cmd.arg("--everything")
        .arg("--memcheck")
        .env("QSV_FREEMEMORY_HEADROOM_PCT", "90")
        .arg(test_file.clone());

    // Verify stats computation succeeds (should auto-create index)
    // and verify stats output is correct
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty());
    assert!(got.len() > 1);

    // Verify index file was created
    // Note: This test may be environment-dependent - on systems with very large RAM,
    // the file might still pass the memory check. The important thing is that
    // the auto-index creation logic exists and works when needed.
    assert!(
        index_file.exists(),
        "Index file should be created automatically when mem_file_check fails"
    );
}

#[test]
fn stats_auto_index_creation_skipped_if_indexed() {
    use std::path::Path;

    let wrk = Workdir::new("stats_auto_index_creation_skipped_if_indexed");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Create index first
    let mut cmd = wrk.command("index");
    cmd.arg(test_file.clone());
    wrk.assert_success(&mut cmd);

    let index_file = Path::new(&test_file).with_extension("csv.idx");
    assert!(index_file.exists(), "Index should exist");

    // Simulate OOM condition
    // With an existing index, parallel processing will be used and mem_file_check is skipped
    let mut cmd = wrk.command("stats");
    cmd.arg("--everything")
        .env("QSV_FREEMEMORY_HEADROOM_PCT", "90")
        .arg(test_file);

    // Verify stats computation succeeds (should use existing index, not create new one)
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty());
    assert!(got.len() > 1);
}

#[test]
fn stats_auto_index_creation_skipped_for_stdin() {
    use std::io::Write;

    let wrk = Workdir::new("stats_auto_index_creation_skipped_for_stdin");

    // Create a small CSV file
    wrk.create(
        "data.csv",
        vec![
            svec!["col1", "col2"],
            svec!["1", "a"],
            svec!["2", "b"],
            svec!["3", "c"],
        ],
    );

    // Read the file content
    let file_content = wrk.read_to_string("data.csv").unwrap();

    // Run stats with stdin input and non-streaming stats
    let mut cmd = wrk.command("stats");
    cmd.arg("--everything")
        .env("QSV_FREEMEMORY_HEADROOM_PCT", "90")
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped());

    // Write to stdin
    let mut child = cmd.spawn().unwrap();
    {
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(file_content.as_bytes()).unwrap();
    }

    // Verify stats computation succeeds (should not attempt to create index for stdin)
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "Stats should succeed for stdin input"
    );
}

#[test]
fn stats_memory_aware_chunking_empty_samples() {
    let wrk = Workdir::new("stats_memory_aware_chunking_empty_samples");

    // Create a CSV file with very few records (< 1000, so sampling may be incomplete)
    let mut data = vec![svec!["col1", "col2"]];
    for i in 0..10 {
        data.push(vec![format!("{}", i), format!("value{}", i)]);
    }
    wrk.create("data.csv", data);

    // Create index to enable parallel processing
    let mut cmd = wrk.command("index");
    cmd.arg("data.csv");
    wrk.assert_success(&mut cmd);

    // Run stats with QSV_STATS_CHUNK_MEMORY_MB=0 (dynamic) and non-streaming stats
    let mut cmd = wrk.command("stats");
    cmd.arg("--cardinality")
        .env("QSV_STATS_CHUNK_MEMORY_MB", "0")
        .arg("data.csv");

    // Verify stats computation succeeds (should fallback to CPU-based chunking)
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty());
    assert!(got.len() > 1);
}

#[test]
fn stats_memory_aware_chunking_small_records() {
    let wrk = Workdir::new("stats_memory_aware_chunking_small_records");

    // Create a CSV file with very small records (single character fields)
    let mut data = vec![svec!["a", "b", "c"]];
    for _ in 0..100 {
        data.push(svec!["x", "y", "z"]);
    }
    wrk.create("data.csv", data);

    // Create index to enable parallel processing
    let mut cmd = wrk.command("index");
    cmd.arg("data.csv");
    wrk.assert_success(&mut cmd);

    // Run stats with QSV_STATS_CHUNK_MEMORY_MB set to 1MB (small fixed value)
    // and non-streaming stats
    let mut cmd = wrk.command("stats");
    cmd.arg("--cardinality")
        .env("QSV_STATS_CHUNK_MEMORY_MB", "1")
        .arg("data.csv");

    // Verify stats computation succeeds (chunk size calculation should handle small records)
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty());
    assert!(got.len() > 1);
}

#[test]
fn stats_memory_aware_chunking_no_samples_fallback() {
    let wrk = Workdir::new("stats_memory_aware_chunking_no_samples_fallback");

    // Create a minimal CSV file
    wrk.create(
        "data.csv",
        vec![svec!["col1"], svec!["1"], svec!["2"], svec!["3"]],
    );

    // Create index to enable parallel processing
    let mut cmd = wrk.command("index");
    cmd.arg("data.csv");
    wrk.assert_success(&mut cmd);

    // Run stats with QSV_STATS_CHUNK_MEMORY_MB=0 (dynamic) and non-streaming stats
    // With very few records, sampling may not work, so should fallback to CPU-based chunking
    let mut cmd = wrk.command("stats");
    cmd.arg("--cardinality")
        .env("QSV_STATS_CHUNK_MEMORY_MB", "0")
        .arg("data.csv");

    // Verify stats computation succeeds (should fallback to CPU-based chunking)
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty());
    assert!(got.len() > 1);
}

// ============================================================================
// Weighted Statistics Tests
// ============================================================================

#[test]
fn stats_weighted_mean_simple() {
    let wrk = Workdir::new("stats_weighted_mean_simple");
    // Values: [1, 2, 3], Weights: [1, 2, 3]
    // Weighted mean = (1*1 + 2*2 + 3*3) / (1+2+3) = 14/6 = 2.3333...
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["1", "1"],
            svec!["2", "2"],
            svec!["3", "3"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty());

    let headers = &got[0];
    let value_row = &got[1];

    // Find mean column
    let mean_idx = headers.iter().position(|h| h == "mean").unwrap();
    let mean_val: f64 = value_row[mean_idx].parse().unwrap();

    // Weighted mean should be approximately 2.3333
    assert!(
        (mean_val - 2.3333333333333335).abs() < 0.0001,
        "Expected weighted mean ~2.333, got {}",
        mean_val
    );
}

#[test]
fn stats_weighted_mean_vs_unweighted() {
    let wrk = Workdir::new("stats_weighted_mean_vs_unweighted");
    // Same values, but with different weights
    // Unweighted mean of [1, 2, 3] = 2.0
    // Weighted mean with weights [1, 1, 10] = (1*1 + 2*1 + 3*10) / 12 = 33/12 = 2.75
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["1", "1"],
            svec!["2", "1"],
            svec!["3", "10"],
        ],
    );

    // Test unweighted
    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv");
    let got_unweighted: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let headers = &got_unweighted[0];
    let value_row = &got_unweighted[1];
    let mean_idx = headers.iter().position(|h| h == "mean").unwrap();
    let unweighted_mean: f64 = value_row[mean_idx].parse().unwrap();
    assert!((unweighted_mean - 2.0).abs() < 0.0001);

    // Test weighted
    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");
    let got_weighted: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let value_row = &got_weighted[1];
    let weighted_mean: f64 = value_row[mean_idx].parse().unwrap();
    assert!(
        (weighted_mean - 2.75).abs() < 0.0001,
        "Expected weighted mean ~2.75, got {}",
        weighted_mean
    );
}

#[test]
fn stats_weighted_stddev() {
    let wrk = Workdir::new("stats_weighted_stddev");
    // Values: [1, 2, 3], Weights: [1, 1, 1] (should match unweighted)
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["1", "1"],
            svec!["2", "1"],
            svec!["3", "1"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(got.len() > 1, "Should have at least one data row");

    let headers = &got[0];
    let value_row = &got[1];

    let stddev_idx = headers
        .iter()
        .position(|h| h == "stddev")
        .expect("Should have stddev column");
    let stddev_str = &value_row[stddev_idx];

    // Check if stddev is empty or NaN
    assert!(!stddev_str.is_empty(), "Stddev should not be empty");
    let stddev_val: f64 = stddev_str.parse().expect("Stddev should be a valid number");

    // With equal weights [1, 1, 1], weighted variance uses sample variance (n-1 denominator)
    // Variance = ((1-2)^2 + (2-2)^2 + (3-2)^2) / (3-1) = (1 + 0 + 1) / 2 = 1.0
    // Stddev = sqrt(1.0) = 1.0
    assert!(
        (stddev_val - 1.0).abs() < 0.01,
        "Expected stddev ~1.0 (sample variance with equal weights), got {}",
        stddev_val
    );
}

#[test]
fn stats_weighted_median() {
    let wrk = Workdir::new("stats_weighted_median");
    // Values: [1, 2, 3], Weights: [1, 2, 1]
    // Cumulative weights: [1, 3, 4], median at 50% = 2.0
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["1", "1"],
            svec!["2", "2"],
            svec!["3", "1"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight")
        .arg("weight")
        .arg("--median")
        .arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(
        got.len() > 1,
        "Should have at least one data row, got {} rows",
        got.len()
    );

    let headers = &got[0];
    let value_row = &got[1];

    let median_idx = headers
        .iter()
        .position(|h| h == "median")
        .expect("Should have median column");
    let median_val: f64 = value_row[median_idx]
        .parse()
        .expect("Median should be a valid number");

    // Weighted median should be 2.0
    assert!(
        (median_val - 2.0).abs() < 0.0001,
        "Expected weighted median 2.0, got {}",
        median_val
    );
}

#[test]
fn stats_weighted_quartiles() {
    let wrk = Workdir::new("stats_weighted_quartiles");
    // Values: [1, 2, 3, 4, 5], Weights: [1, 1, 2, 1, 1]
    // Total weight = 6, Q1 at 25% = 1.5, Q2 (median) at 50% = 3.0, Q3 at 75% = 3.5
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["1", "1"],
            svec!["2", "1"],
            svec!["3", "2"],
            svec!["4", "1"],
            svec!["5", "1"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight")
        .arg("weight")
        .arg("--quartiles")
        .arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(
        got.len() > 1,
        "Should have at least one data row, got {} rows",
        got.len()
    );

    let headers = &got[0];
    let value_row = &got[1];

    let q1_idx = headers
        .iter()
        .position(|h| h == "q1")
        .expect("Should have q1 column");
    let q2_idx = headers
        .iter()
        .position(|h| h == "q2_median")
        .expect("Should have q2_median column");
    let q3_idx = headers
        .iter()
        .position(|h| h == "q3")
        .expect("Should have q3 column");

    let q1: f64 = value_row[q1_idx]
        .parse()
        .expect("Q1 should be a valid number");
    let q2: f64 = value_row[q2_idx]
        .parse()
        .expect("Q2 should be a valid number");
    let q3: f64 = value_row[q3_idx]
        .parse()
        .expect("Q3 should be a valid number");

    // Q2 (median) should be 3.0
    assert!((q2 - 3.0).abs() < 0.1, "Expected Q2 ~3.0, got {}", q2);
    // Q1 and Q3 should be reasonable values
    assert!(
        q1 < q2 && q2 < q3,
        "Quartiles should be ordered: Q1={}, Q2={}, Q3={}",
        q1,
        q2,
        q3
    );
}

#[test]
fn stats_weighted_mad() {
    let wrk = Workdir::new("stats_weighted_mad");
    // Values: [1, 2, 3], Weights: [1, 2, 1]
    // Weighted median = 2.0
    // Absolute deviations: [|1-2|, |2-2|, |3-2|] = [1, 0, 1] with weights [1, 2, 1]
    // Weighted MAD = median of [1, 0, 1] with weights [1, 2, 1] = 0.0 (since 0 has weight 2)
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["1", "1"],
            svec!["2", "2"],
            svec!["3", "1"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight")
        .arg("weight")
        .arg("--mad")
        .arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(
        got.len() > 1,
        "Should have at least one data row, got {} rows",
        got.len()
    );

    let headers = &got[0];
    let value_row = &got[1];

    let mad_idx = headers
        .iter()
        .position(|h| h == "mad")
        .expect("Should have mad column");
    let mad_val: f64 = value_row[mad_idx]
        .parse()
        .expect("MAD should be a valid number");

    // Weighted MAD should be 0.0 (median of absolute deviations [1, 0, 1] with weights [1, 2, 1])
    // Cumulative weights for deviations: [1, 3, 4], median at 50% = 2.0, which corresponds to 0
    assert!(
        (mad_val - 0.0).abs() < 0.0001,
        "Expected weighted MAD ~0.0, got {}",
        mad_val
    );
}

#[test]
fn stats_weighted_percentiles() {
    let wrk = Workdir::new("stats_weighted_percentiles");
    // Values: [1, 2, 3, 4, 5], Weights: [1, 1, 2, 1, 1]
    // Total weight = 6
    // 10th percentile: 0.1 * 6 = 0.6 -> value 1 (cumulative weight reaches 1 at value 1)
    // 50th percentile (median): 0.5 * 6 = 3.0 -> value 3 (cumulative weight reaches 4 at value 3)
    // 90th percentile: 0.9 * 6 = 5.4 -> value 5 (cumulative weight reaches 6 at value 5)
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["1", "1"],
            svec!["2", "1"],
            svec!["3", "2"],
            svec!["4", "1"],
            svec!["5", "1"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight")
        .arg("weight")
        .arg("--percentiles")
        .arg("--percentile-list")
        .arg("10,50,90")
        .arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(
        got.len() > 1,
        "Should have at least one data row, got {} rows",
        got.len()
    );

    let headers = &got[0];
    let value_row = &got[1];

    let percentiles_idx = headers
        .iter()
        .position(|h| h == "percentiles")
        .expect("Should have percentiles column");
    let percentiles_str = &value_row[percentiles_idx];

    // Parse percentiles string (format: "10: 1.0|50: 3.0|90: 5.0")
    let percentile_parts: Vec<&str> = percentiles_str.split('|').collect();
    assert_eq!(
        percentile_parts.len(),
        3,
        "Should have 3 percentiles, got {}",
        percentile_parts.len()
    );

    // Extract values from "10: 1.0" format
    let p10_val: f64 = percentile_parts[0]
        .split(':')
        .nth(1)
        .expect("Should have value after colon")
        .trim()
        .parse()
        .expect("P10 should be a valid number");
    let p50_val: f64 = percentile_parts[1]
        .split(':')
        .nth(1)
        .expect("Should have value after colon")
        .trim()
        .parse()
        .expect("P50 should be a valid number");
    let p90_val: f64 = percentile_parts[2]
        .split(':')
        .nth(1)
        .expect("Should have value after colon")
        .trim()
        .parse()
        .expect("P90 should be a valid number");

    // Verify percentile values
    assert!(
        (p10_val - 1.0).abs() < 0.1,
        "Expected 10th percentile ~1.0, got {}",
        p10_val
    );
    assert!(
        (p50_val - 3.0).abs() < 0.1,
        "Expected 50th percentile (median) ~3.0, got {}",
        p50_val
    );
    assert!(
        (p90_val - 5.0).abs() < 0.1,
        "Expected 90th percentile ~5.0, got {}",
        p90_val
    );
    // Verify ordering
    assert!(
        p10_val <= p50_val && p50_val <= p90_val,
        "Percentiles should be ordered: P10={}, P50={}, P90={}",
        p10_val,
        p50_val,
        p90_val
    );
}

#[test]
fn stats_weighted_geometric_mean() {
    let wrk = Workdir::new("stats_weighted_geometric_mean");
    // Values: [2, 8, 32], Weights: [1, 1, 1]
    // Weighted geometric mean = exp((1*ln(2) + 1*ln(8) + 1*ln(32)) / 3)
    // = exp((ln(2) + ln(8) + ln(32)) / 3) = exp(ln(2*8*32) / 3) = exp(ln(512) / 3)
    // = exp(ln(512^(1/3))) = 512^(1/3) = 8.0
    // With equal weights, should match unweighted geometric mean
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["2", "1"],
            svec!["8", "1"],
            svec!["32", "1"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(got.len() > 1, "Should have at least one data row");

    let headers = &got[0];
    let value_row = &got[1];

    let geom_mean_idx = headers
        .iter()
        .position(|h| h == "geometric_mean")
        .expect("Should have geometric_mean column");
    let geom_mean_str = &value_row[geom_mean_idx];

    // Check if geometric_mean is empty or NaN
    assert!(
        !geom_mean_str.is_empty(),
        "Geometric mean should not be empty"
    );
    let geom_mean_val: f64 = geom_mean_str
        .parse()
        .expect("Geometric mean should be a valid number");

    // Expected: (2 * 8 * 32)^(1/3) = 512^(1/3) ≈ 8.0
    assert!(
        (geom_mean_val - 8.0).abs() < 0.01,
        "Expected geometric mean ~8.0, got {}",
        geom_mean_val
    );
}

#[test]
fn stats_weighted_geometric_mean_unequal_weights() {
    let wrk = Workdir::new("stats_weighted_geometric_mean_unequal_weights");
    // Values: [2, 8], Weights: [1, 3]
    // Weighted geometric mean = exp((1*ln(2) + 3*ln(8)) / 4)
    // = exp((ln(2) + 3*ln(8)) / 4) = exp((ln(2) + ln(8^3)) / 4)
    // = exp(ln(2 * 512) / 4) = exp(ln(1024) / 4) = 1024^(1/4) ≈ 5.66
    wrk.create(
        "data.csv",
        vec![svec!["value", "weight"], svec!["2", "1"], svec!["8", "3"]],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(got.len() > 1, "Should have at least one data row");

    let headers = &got[0];
    let value_row = &got[1];

    let geom_mean_idx = headers
        .iter()
        .position(|h| h == "geometric_mean")
        .expect("Should have geometric_mean column");
    let geom_mean_str = &value_row[geom_mean_idx];

    assert!(
        !geom_mean_str.is_empty(),
        "Geometric mean should not be empty"
    );
    let geom_mean_val: f64 = geom_mean_str
        .parse()
        .expect("Geometric mean should be a valid number");

    // Expected: exp((ln(2) + 3*ln(8)) / 4) = exp(ln(2 * 512) / 4) = 1024^(1/4) ≈ 5.66
    let expected = (2.0_f64 * 8.0_f64.powi(3)).powf(1.0 / 4.0);
    assert!(
        (geom_mean_val - expected).abs() < 0.01,
        "Expected geometric mean ~{}, got {}",
        expected,
        geom_mean_val
    );
}

#[test]
fn stats_weighted_harmonic_mean() {
    let wrk = Workdir::new("stats_weighted_harmonic_mean");
    // Values: [2, 4, 8], Weights: [1, 1, 1]
    // Weighted harmonic mean = 3 / (1/2 + 1/4 + 1/8) = 3 / (4/8 + 2/8 + 1/8)
    // = 3 / (7/8) = 3 * 8/7 = 24/7 ≈ 3.4286
    // With equal weights, should match unweighted harmonic mean
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["2", "1"],
            svec!["4", "1"],
            svec!["8", "1"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(got.len() > 1, "Should have at least one data row");

    let headers = &got[0];
    let value_row = &got[1];

    let harm_mean_idx = headers
        .iter()
        .position(|h| h == "harmonic_mean")
        .expect("Should have harmonic_mean column");
    let harm_mean_str = &value_row[harm_mean_idx];

    // Check if harmonic_mean is empty or NaN
    assert!(
        !harm_mean_str.is_empty(),
        "Harmonic mean should not be empty"
    );
    let harm_mean_val: f64 = harm_mean_str
        .parse()
        .expect("Harmonic mean should be a valid number");

    // Expected: 3 / (1/2 + 1/4 + 1/8) = 3 / (7/8) = 24/7 ≈ 3.4286
    let expected = 3.0 / (1.0 / 2.0 + 1.0 / 4.0 + 1.0 / 8.0);
    assert!(
        (harm_mean_val - expected).abs() < 0.01,
        "Expected harmonic mean ~{}, got {}",
        expected,
        harm_mean_val
    );
}

#[test]
fn stats_weighted_harmonic_mean_unequal_weights() {
    let wrk = Workdir::new("stats_weighted_harmonic_mean_unequal_weights");
    // Values: [2, 8], Weights: [1, 3]
    // Weighted harmonic mean = (1 + 3) / (1/2 + 3/8) = 4 / (4/8 + 3/8)
    // = 4 / (7/8) = 4 * 8/7 = 32/7 ≈ 4.5714
    wrk.create(
        "data.csv",
        vec![svec!["value", "weight"], svec!["2", "1"], svec!["8", "3"]],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(got.len() > 1, "Should have at least one data row");

    let headers = &got[0];
    let value_row = &got[1];

    let harm_mean_idx = headers
        .iter()
        .position(|h| h == "harmonic_mean")
        .expect("Should have harmonic_mean column");
    let harm_mean_str = &value_row[harm_mean_idx];

    assert!(
        !harm_mean_str.is_empty(),
        "Harmonic mean should not be empty"
    );
    let harm_mean_val: f64 = harm_mean_str
        .parse()
        .expect("Harmonic mean should be a valid number");

    // Expected: (1 + 3) / (1/2 + 3/8) = 4 / (7/8) = 32/7 ≈ 4.5714
    let expected = (1.0 + 3.0) / (1.0 / 2.0 + 3.0 / 8.0);
    assert!(
        (harm_mean_val - expected).abs() < 0.01,
        "Expected harmonic mean ~{}, got {}",
        expected,
        harm_mean_val
    );
}

#[test]
fn stats_weighted_geometric_mean_zero_or_negative() {
    let wrk = Workdir::new("stats_weighted_geometric_mean_zero_or_negative");
    // Geometric mean requires positive values
    // Test with zero and negative values - should handle gracefully
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["-1", "1"],
            svec!["0", "1"],
            svec!["2", "1"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(got.len() > 1, "Should have at least one data row");

    let headers = &got[0];
    let value_row = &got[1];

    let geom_mean_idx = headers
        .iter()
        .position(|h| h == "geometric_mean")
        .expect("Should have geometric_mean column");
    let geom_mean_str = &value_row[geom_mean_idx];

    // Geometric mean should only consider positive values (2)
    // So it should be exp(ln(2) / 1) = 2.0
    // But if negative/zero values cause issues, it might be NaN or empty
    if !geom_mean_str.is_empty() && geom_mean_str != "NaN" {
        let geom_mean_val: f64 = geom_mean_str.parse().unwrap_or(f64::NAN);
        // Should only consider positive value (2), so result should be 2.0
        assert!(
            (geom_mean_val - 2.0).abs() < 0.01 || geom_mean_val.is_nan(),
            "Geometric mean should be ~2.0 (only positive values) or NaN, got {}",
            geom_mean_val
        );
    }
}

#[test]
fn stats_weighted_harmonic_mean_zero() {
    let wrk = Workdir::new("stats_weighted_harmonic_mean_zero");
    // Harmonic mean requires non-zero values
    // Test with zero values - should handle gracefully
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["0", "1"],
            svec!["2", "1"],
            svec!["4", "1"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(got.len() > 1, "Should have at least one data row");

    let headers = &got[0];
    let value_row = &got[1];

    let harm_mean_idx = headers
        .iter()
        .position(|h| h == "harmonic_mean")
        .expect("Should have harmonic_mean column");
    let harm_mean_str = &value_row[harm_mean_idx];

    // Harmonic mean should only consider non-zero values (2, 4)
    // So it should be 2 / (1/2 + 1/4) = 2 / (3/4) = 8/3 ≈ 2.667
    // But if zero values cause issues, it might be NaN or empty
    if !harm_mean_str.is_empty() && harm_mean_str != "NaN" {
        let harm_mean_val: f64 = harm_mean_str.parse().unwrap_or(f64::NAN);
        let expected = 2.0 / (1.0 / 2.0 + 1.0 / 4.0);
        assert!(
            (harm_mean_val - expected).abs() < 0.01 || harm_mean_val.is_nan(),
            "Harmonic mean should be ~{} (only non-zero values) or NaN, got {}",
            expected,
            harm_mean_val
        );
    }
}

#[test]
fn stats_weighted_modes() {
    let wrk = Workdir::new("stats_weighted_modes");
    // Values: ["a", "b", "a", "c", "b"], Weights: [1, 2, 3, 1, 1]
    // Weighted frequencies:
    //   "a": 1 + 3 = 4 (max weight - mode)
    //   "b": 2 + 1 = 3
    //   "c": 1 (min weight - antimode)
    // Mode should be "a" with weight 4
    // Antimode should be "c" with weight 1
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["a", "3"],
            svec!["c", "1"],
            svec!["b", "1"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight")
        .arg("weight")
        .arg("--mode")
        .arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(
        got.len() > 1,
        "Should have at least one data row, got {} rows",
        got.len()
    );

    let headers = &got[0];
    let value_row = &got[1];

    let mode_idx = headers
        .iter()
        .position(|h| h == "mode")
        .expect("Should have mode column");
    let mode_count_idx = headers
        .iter()
        .position(|h| h == "mode_count")
        .expect("Should have mode_count column");
    let mode_occurrences_idx = headers
        .iter()
        .position(|h| h == "mode_occurrences")
        .expect("Should have mode_occurrences column");
    let antimode_idx = headers
        .iter()
        .position(|h| h == "antimode")
        .expect("Should have antimode column");
    let antimode_count_idx = headers
        .iter()
        .position(|h| h == "antimode_count")
        .expect("Should have antimode_count column");
    let antimode_occurrences_idx = headers
        .iter()
        .position(|h| h == "antimode_occurrences")
        .expect("Should have antimode_occurrences column");

    let mode_str = &value_row[mode_idx];
    let mode_count: usize = value_row[mode_count_idx]
        .parse()
        .expect("mode_count should be a valid number");
    let mode_occurrences: u32 = value_row[mode_occurrences_idx]
        .parse()
        .expect("mode_occurrences should be a valid number");
    let antimode_str = &value_row[antimode_idx];
    let antimode_count: usize = value_row[antimode_count_idx]
        .parse()
        .expect("antimode_count should be a valid number");
    let antimode_occurrences: u32 = value_row[antimode_occurrences_idx]
        .parse()
        .expect("antimode_occurrences should be a valid number");

    // Mode should be "a" with weight 4
    assert_eq!(mode_str, "a", "Expected mode 'a', got '{}'", mode_str);
    assert_eq!(mode_count, 1, "Expected mode_count 1, got {}", mode_count);
    assert_eq!(
        mode_occurrences, 4,
        "Expected mode_occurrences 4 (weight), got {}",
        mode_occurrences
    );

    // Antimode should be "c" with weight 1
    assert_eq!(
        antimode_str, "c",
        "Expected antimode 'c', got '{}'",
        antimode_str
    );
    assert_eq!(
        antimode_count, 1,
        "Expected antimode_count 1, got {}",
        antimode_count
    );
    assert_eq!(
        antimode_occurrences, 1,
        "Expected antimode_occurrences 1 (weight), got {}",
        antimode_occurrences
    );
}

#[test]
fn stats_weighted_modes_multiple() {
    let wrk = Workdir::new("stats_weighted_modes_multiple");
    // Values: ["x", "y", "x", "z", "y"], Weights: [2, 3, 2, 1, 3]
    // Weighted frequencies:
    //   "x": 2 + 2 = 4 (tied for max weight - mode)
    //   "y": 3 + 3 = 6 (max weight - mode)
    //   "z": 1 (min weight - antimode)
    // Modes should be ["x", "y"] or ["y", "x"] with weight 6 (or 4)
    // Antimode should be "z" with weight 1
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["x", "2"],
            svec!["y", "3"],
            svec!["x", "2"],
            svec!["z", "1"],
            svec!["y", "3"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight")
        .arg("weight")
        .arg("--mode")
        .arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(
        got.len() > 1,
        "Should have at least one data row, got {} rows",
        got.len()
    );

    let headers = &got[0];
    let value_row = &got[1];

    let mode_idx = headers
        .iter()
        .position(|h| h == "mode")
        .expect("Should have mode column");
    let mode_count_idx = headers
        .iter()
        .position(|h| h == "mode_count")
        .expect("Should have mode_count column");
    let antimode_idx = headers
        .iter()
        .position(|h| h == "antimode")
        .expect("Should have antimode column");

    let mode_str = &value_row[mode_idx];
    let mode_count: usize = value_row[mode_count_idx]
        .parse()
        .expect("mode_count should be a valid number");
    let antimode_str = &value_row[antimode_idx];

    // Mode should be "y" (or "x" and "y" if both have max weight)
    // Since "y" has weight 6 and "x" has weight 4, "y" should be the mode
    // But if there's a tie, both could be modes
    assert!(
        mode_str.contains("y"),
        "Expected mode to contain 'y', got '{}'",
        mode_str
    );
    // Mode count should be at least 1
    assert!(
        mode_count >= 1,
        "Expected mode_count >= 1, got {}",
        mode_count
    );

    // Antimode should be "z"
    assert_eq!(
        antimode_str, "z",
        "Expected antimode 'z', got '{}'",
        antimode_str
    );
}

#[test]
fn stats_weighted_modes_cardinality() {
    let wrk = Workdir::new("stats_weighted_modes_cardinality");
    // Test that cardinality works with weighted modes
    // Values: ["a", "b", "c"], Weights: [1, 2, 3]
    // Cardinality should be 3 (3 unique values)
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight")
        .arg("weight")
        .arg("--cardinality")
        .arg("--mode")
        .arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(
        got.len() > 1,
        "Should have at least one data row, got {} rows",
        got.len()
    );

    let headers = &got[0];
    let value_row = &got[1];

    let cardinality_idx = headers
        .iter()
        .position(|h| h == "cardinality")
        .expect("Should have cardinality column");
    let cardinality: u64 = value_row[cardinality_idx]
        .parse()
        .expect("cardinality should be a valid number");

    // Cardinality should be 3 (3 unique values)
    assert_eq!(
        cardinality, 3,
        "Expected cardinality 3, got {}",
        cardinality
    );
}

#[test]
fn stats_weighted_missing_weight_column() {
    let wrk = Workdir::new("stats_weighted_missing_weight_column");
    wrk.create("data.csv", vec![svec!["value"], svec!["1"], svec!["2"]]);

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("nonexistent").arg("data.csv");

    let output = cmd.output().unwrap();
    assert!(
        !output.status.success(),
        "Should fail when weight column doesn't exist"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("not found"),
        "Error should mention column not found"
    );
}

#[test]
fn stats_weighted_invalid_weights_default_to_one() {
    let wrk = Workdir::new("stats_weighted_invalid_weights_default_to_one");
    // Invalid/empty weights default to 1.0, but valid weights are used with their actual values.
    // With weights [1.0 (defaulted from "invalid"), 2 (valid), 1.0 (defaulted from "")],
    // weighted mean = (1*1.0 + 2*2 + 3*1.0) / (1.0 + 2 + 1.0) = 8/4 = 2.0
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["1", "invalid"],
            svec!["2", "2"],
            svec!["3", ""], // empty weight
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let headers = &got[0];
    let value_row = &got[1];

    let mean_idx = headers.iter().position(|h| h == "mean").unwrap();
    let mean_val: f64 = value_row[mean_idx].parse().unwrap();

    assert!(
        (mean_val - 2.0).abs() < 0.01,
        "Expected mean ~2.0 with weights [1.0, 2, 1.0], got {}",
        mean_val
    );
}

#[test]
fn stats_weighted_excludes_weight_column() {
    let wrk = Workdir::new("stats_weighted_excludes_weight_column");
    wrk.create(
        "data.csv",
        vec![svec!["value", "weight"], svec!["1", "1"], svec!["2", "2"]],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    // Check command succeeds
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(got.len() > 1, "Should have at least one data row");

    let headers = &got[0];

    // Weight column should not appear in stats output headers (which are stat column names)
    assert!(
        !headers.contains(&"weight".to_string()),
        "Weight column should be excluded from stats"
    );

    // Check the "field" column in data rows to ensure "value" is present and "weight" is not
    let field_idx = headers
        .iter()
        .position(|h| h == "field")
        .expect("Should have 'field' column");
    let mut found_value = false;
    for row in got.iter().skip(1) {
        if let Some(field_val) = row.get(field_idx) {
            assert_ne!(
                field_val, "weight",
                "Weight column should not appear in stats"
            );
            if field_val == "value" {
                found_value = true;
            }
        }
    }
    assert!(found_value, "Value column should be in stats");
}

#[test]
fn stats_weighted_parallel_processing() {
    let wrk = Workdir::new("stats_weighted_parallel_processing");

    // Create a larger dataset to test parallel processing
    let mut data = vec![svec!["value", "weight"]];
    for i in 1..=100 {
        data.push(vec![format!("{}", i), format!("{}", i % 10 + 1)]);
    }
    wrk.create("data.csv", data);

    // Create index to enable parallel processing
    let mut cmd = wrk.command("index");
    cmd.arg("data.csv");
    wrk.assert_success(&mut cmd);

    // Run weighted stats with parallel processing
    let mut cmd = wrk.command("stats");
    cmd.arg("--weight")
        .arg("weight")
        .arg("--everything")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty());

    let headers = &got[0];
    let value_row = &got[1];

    // Verify weighted statistics are computed
    let mean_idx = headers.iter().position(|h| h == "mean").unwrap();
    let mean_val: f64 = value_row[mean_idx].parse().unwrap();
    assert!(mean_val > 0.0 && mean_val < 100.0);
}

#[test]
fn stats_weighted_cache_filename() {
    let wrk = Workdir::new("stats_weighted_cache_filename");
    wrk.create(
        "data.csv",
        vec![svec!["value", "weight"], svec!["1", "1"], svec!["2", "2"]],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");
    wrk.assert_success(&mut cmd);

    // Check that weighted cache file is created
    assert!(
        wrk.path("data.stats.weighted.csv").exists(),
        "Weighted stats cache file should exist"
    );
    assert!(
        !wrk.path("data.stats.csv").exists(),
        "Unweighted stats cache file should not exist when using --weight"
    );
}

#[test]
fn stats_weighted_vs_unweighted_cache_separation() {
    let wrk = Workdir::new("stats_weighted_vs_unweighted_cache_separation");
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["1", "1"],
            svec!["2", "2"],
            svec!["3", "3"],
        ],
    );

    // Run unweighted stats
    let mut cmd = wrk.command("stats");
    cmd.arg("data.csv");
    wrk.assert_success(&mut cmd);
    assert!(wrk.path("data.stats.csv").exists());

    // Run weighted stats
    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");
    wrk.assert_success(&mut cmd);
    assert!(wrk.path("data.stats.weighted.csv").exists());

    // Both cache files should exist
    assert!(wrk.path("data.stats.csv").exists());
    assert!(wrk.path("data.stats.weighted.csv").exists());
}

#[test]
fn stats_weighted_all_zero_weights() {
    let wrk = Workdir::new("stats_weighted_all_zero_weights");
    // All weights are zero - should handle gracefully
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["1", "0"],
            svec!["2", "0"],
            svec!["3", "0"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    // Command should succeed even with all zero weights
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");
    assert!(
        got.len() > 1,
        "Should have at least one data row, got {} rows",
        got.len()
    );

    let headers = &got[0];
    let value_row = &got[1];

    // With all zero weights, mean/stddev should be empty or zero
    let mean_idx = headers.iter().position(|h| h == "mean");
    if let Some(idx) = mean_idx {
        let mean_str = &value_row[idx];
        // Mean should be empty or zero when all weights are zero
        assert!(
            mean_str.is_empty() || mean_str == "0",
            "Mean should be empty or zero with all zero weights, got '{}'",
            mean_str
        );
    }
}

#[test]
fn stats_weighted_negative_weights() {
    let wrk = Workdir::new("stats_weighted_negative_weights");
    // Negative weights should be ignored
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["1", "-1"], // negative weight - should be ignored
            svec!["2", "2"],  // positive weight - should be used
            svec!["3", "-5"], // negative weight - should be ignored
            svec!["4", "1"],  // positive weight - should be used
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let headers = &got[0];
    let value_row = &got[1];

    let mean_idx = headers.iter().position(|h| h == "mean").unwrap();
    let mean_val: f64 = value_row[mean_idx].parse().unwrap();

    // Only values 2 and 4 should contribute (weights 2 and 1)
    // Weighted mean = (2*2 + 4*1) / (2+1) = 8/3 ≈ 2.67
    assert!(
        (mean_val - 2.67).abs() < 0.1,
        "Expected weighted mean ~2.67 (only positive weights), got {}",
        mean_val
    );
}

#[test]
fn stats_weighted_mixed_zero_and_nonzero_weights() {
    let wrk = Workdir::new("stats_weighted_mixed_zero_and_nonzero_weights");
    // Mix of zero and non-zero weights - only non-zero should contribute
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["10", "0"], // zero weight - should be ignored
            svec!["20", "2"], // non-zero weight - should be used
            svec!["30", "0"], // zero weight - should be ignored
            svec!["40", "3"], // non-zero weight - should be used
            svec!["50", "0"], // zero weight - should be ignored
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let headers = &got[0];
    let value_row = &got[1];

    let mean_idx = headers.iter().position(|h| h == "mean").unwrap();
    let mean_val: f64 = value_row[mean_idx].parse().unwrap();

    // Only values 20 and 40 should contribute (weights 2 and 3)
    // Weighted mean = (20*2 + 40*3) / (2+3) = 160/5 = 32.0
    assert!(
        (mean_val - 32.0).abs() < 0.1,
        "Expected weighted mean ~32.0 (only non-zero weights), got {}",
        mean_val
    );
}

#[test]
fn stats_weighted_nan_weights() {
    let wrk = Workdir::new("stats_weighted_nan_weights");
    // NaN weights should be handled (likely defaulted to 1.0 or ignored)
    // Note: CSV parsing might convert "NaN" to a string, so we test with invalid numeric strings
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["1", "invalid"],    // invalid weight - should default to 1.0
            svec!["2", "2"],          // valid weight
            svec!["3", "notanumber"], // invalid weight - should default to 1.0
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    // Command should succeed
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");

    let headers = &got[0];
    let value_row = &got[1];

    let mean_idx = headers.iter().position(|h| h == "mean").unwrap();
    let mean_val: f64 = value_row[mean_idx].parse().unwrap();

    // Invalid weights default to 1.0, so weights are effectively [1, 2, 1]
    // Weighted mean = (1*1 + 2*2 + 3*1) / (1+2+1) = 8/4 = 2.0
    assert!(
        (mean_val - 2.0).abs() < 0.1,
        "Expected weighted mean ~2.0 (invalid weights default to 1.0), got {}",
        mean_val
    );
}

#[test]
fn stats_weighted_infinity_weights() {
    let wrk = Workdir::new("stats_weighted_infinity_weights");
    // Very large weights that might cause overflow
    // Using scientific notation for very large numbers
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["1", "1e100"], // very large weight
            svec!["2", "1e100"], // very large weight
            svec!["3", "1"],     // normal weight
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight").arg("weight").arg("data.csv");

    // Command should succeed without overflow
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");

    let headers = &got[0];
    let value_row = &got[1];

    let mean_idx = headers.iter().position(|h| h == "mean").unwrap();
    let mean_str = &value_row[mean_idx];

    // With very large weights, the mean should be dominated by values 1 and 2
    // Weighted mean ≈ (1*1e100 + 2*1e100 + 3*1) / (1e100 + 1e100 + 1) ≈ 1.5
    // But due to floating point precision, it might be exactly 1.5 or close to it
    if !mean_str.is_empty() {
        let mean_val: f64 = mean_str.parse().unwrap();
        assert!(
            mean_val >= 1.0 && mean_val <= 2.0,
            "Mean should be between 1.0 and 2.0 with very large weights, got {}",
            mean_val
        );
    }
}

#[test]
fn stats_weighted_zero_weights_with_modes() {
    let wrk = Workdir::new("stats_weighted_zero_weights_with_modes");
    // Test modes/antimodes with zero weights
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["a", "0"], // zero weight
            svec!["b", "2"], // non-zero weight
            svec!["a", "0"], // zero weight
            svec!["c", "1"], // non-zero weight
            svec!["b", "0"], // zero weight
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight")
        .arg("weight")
        .arg("--mode")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let headers = &got[0];
    let value_row = &got[1];

    let mode_idx = headers.iter().position(|h| h == "mode").unwrap();
    let mode_str = &value_row[mode_idx];

    // Only "b" (weight 2) and "c" (weight 1) should contribute
    // Mode should be "b" with weight 2
    assert_eq!(
        mode_str, "b",
        "Expected mode 'b' (weight 2), got '{}'",
        mode_str
    );
}

#[test]
fn stats_weighted_negative_weights_with_quartiles() {
    let wrk = Workdir::new("stats_weighted_negative_weights_with_quartiles");
    // Test quartiles with negative weights (should be ignored)
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["1", "-1"], // negative weight - ignored
            svec!["2", "2"],  // positive weight
            svec!["3", "-5"], // negative weight - ignored
            svec!["4", "3"],  // positive weight
            svec!["5", "1"],  // positive weight
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight")
        .arg("weight")
        .arg("--quartiles")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let headers = &got[0];
    let value_row = &got[1];

    let q2_idx = headers.iter().position(|h| h == "q2_median").unwrap();
    let q2_str = &value_row[q2_idx];

    // Quartiles should be computed (negative weights are now filtered out)
    assert!(
        !q2_str.is_empty(),
        "Q2 median should not be empty when valid weights exist"
    );

    let q2_val: f64 = q2_str.parse().expect("Q2 should be a valid number");

    // Only values 2, 4, 5 should contribute (weights 2, 3, 1)
    // Total weight = 6, median at 50% = 3.0
    // Cumulative: 2 (weight 2) = 2, 4 (weight 3) = 5, 5 (weight 1) = 6
    // Median should be 4 (reaches/exceeds 3.0 at value 4)
    assert!(
        (q2_val - 4.0).abs() < 0.1,
        "Expected weighted median ~4.0 (negative weights ignored), got {}",
        q2_val
    );
}

#[test]
fn stats_weighted_mixed_edge_cases() {
    let wrk = Workdir::new("stats_weighted_mixed_edge_cases");
    // Test combination of edge cases: zero, negative, invalid, and valid weights
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "weight"],
            svec!["10", "0"],       // zero weight - ignored
            svec!["20", "-2"],      // negative weight - ignored
            svec!["30", "invalid"], // invalid weight - defaults to 1.0
            svec!["40", "3"],       // valid positive weight
            svec!["50", ""],        // empty weight - defaults to 1.0
            svec!["60", "2"],       // valid positive weight
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--weight")
        .arg("weight")
        .arg("--mad")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty(), "Stats output should not be empty");

    let headers = &got[0];
    let value_row = &got[1];

    let mean_idx = headers.iter().position(|h| h == "mean").unwrap();
    let mean_val: f64 = value_row[mean_idx].parse().unwrap();

    // Only values 30, 40, 50, 60 should contribute (weights 1, 3, 1, 2)
    // Weighted mean = (30*1 + 40*3 + 50*1 + 60*2) / (1+3+1+2) = 320/7 ≈ 45.71
    assert!(
        (mean_val - 45.71).abs() < 0.5,
        "Expected weighted mean ~45.71 (mixed edge cases), got {}",
        mean_val
    );
}

#[test]
fn stats_json_file_level_metadata() {
    use std::path::Path;

    use serde_json::Value;

    let wrk = Workdir::new("stats_json_file_level_metadata");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Run stats with --dataset-stats to populate the new fields
    let mut dataset_stats_cmd = wrk.command("stats");
    dataset_stats_cmd
        .arg("--dataset-stats")
        .args(["--cache-threshold", "1"])
        .arg(&test_file);

    wrk.run(&mut dataset_stats_cmd);

    // Read the JSON metadata file
    let json_path = wrk.path("boston311-100.stats.csv.json");
    assert!(Path::new(&json_path).exists(), "JSON file should exist");

    let json_content = std::fs::read_to_string(&json_path).unwrap();
    let json: Value = serde_json::from_str(&json_content).unwrap();

    // Verify field_count is a positive integer (boston311-100.csv has 29 columns)
    let field_count = json.get("field_count").expect("field_count should exist");
    assert!(field_count.is_u64(), "field_count should be a number");
    assert_eq!(
        field_count.as_u64().unwrap(),
        29,
        "field_count should be 29"
    );

    // Verify filesize_bytes is a positive integer
    let filesize_bytes = json
        .get("filesize_bytes")
        .expect("filesize_bytes should exist");
    assert!(filesize_bytes.is_u64(), "filesize_bytes should be a number");
    assert!(
        filesize_bytes.as_u64().unwrap() > 0,
        "filesize_bytes should be positive"
    );

    // Verify hash object exists and has BLAKE3 key
    let hash = json.get("hash").expect("hash should exist");
    assert!(hash.is_object(), "hash should be an object");
    let blake3 = hash.get("BLAKE3").expect("hash.BLAKE3 should exist");
    assert!(blake3.is_string(), "hash.BLAKE3 should be a string");
    let blake3_str = blake3.as_str().unwrap();
    assert!(!blake3_str.is_empty(), "hash.BLAKE3 should not be empty");
    assert_eq!(
        blake3_str.len(),
        64,
        "BLAKE3 hash should be 64 hex characters"
    );
}

#[test]
fn stats_json_arg_input_format() {
    use std::path::Path;

    use serde_json::Value;

    let wrk = Workdir::new("stats_json_arg_input_format");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("--dataset-stats")
        .args(["--cache-threshold", "1"])
        .arg(&test_file);

    wrk.run(&mut stats_cmd);

    let json_path = wrk.path("boston311-100.stats.csv.json");
    assert!(Path::new(&json_path).exists(), "JSON file should exist");

    let json_content = std::fs::read_to_string(&json_path).unwrap();
    let json: Value = serde_json::from_str(&json_content).unwrap();

    // Verify arg_input doesn't contain "Some(" wrapper
    let arg_input = json.get("arg_input").expect("arg_input should exist");
    assert!(arg_input.is_string(), "arg_input should be a string");
    let arg_input_str = arg_input.as_str().unwrap();
    assert!(
        !arg_input_str.contains("Some("),
        "arg_input should not contain 'Some('"
    );
    assert!(
        arg_input_str.ends_with("boston311-100.csv"),
        "arg_input should be a plain path"
    );

    // Verify flag_delimiter doesn't contain "None"
    let flag_delimiter = json
        .get("flag_delimiter")
        .expect("flag_delimiter should exist");
    assert!(
        flag_delimiter.is_string(),
        "flag_delimiter should be a string"
    );
    let flag_delimiter_str = flag_delimiter.as_str().unwrap();
    assert!(
        !flag_delimiter_str.contains("None"),
        "flag_delimiter should not contain 'None'"
    );
}

#[test]
fn stats_json_backward_compat() {
    use std::path::Path;

    use serde_json::Value;

    let wrk = Workdir::new("stats_json_backward_compat");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // First, create a cache file
    let mut cmd = wrk.command("stats");
    cmd.arg("--dataset-stats")
        .args(["--cache-threshold", "1"])
        .arg(&test_file);
    wrk.run(&mut cmd);

    let json_path = wrk.path("boston311-100.stats.csv.json");
    assert!(Path::new(&json_path).exists(), "JSON file should exist");

    // Read and modify the JSON to simulate an older cache file without new fields
    let json_content = std::fs::read_to_string(&json_path).unwrap();
    let mut json: Value = serde_json::from_str(&json_content).unwrap();

    // Remove the new fields to simulate an older cache file
    if let Value::Object(ref mut map) = json {
        map.remove("field_count");
        map.remove("filesize_bytes");
        map.remove("hash");
        // Change qsv_version to force cache invalidation and recomputation
        map.insert(
            "qsv_version".to_string(),
            Value::String("0.0.0".to_string()),
        );
    }

    // Write back the modified JSON
    std::fs::write(&json_path, serde_json::to_string_pretty(&json).unwrap()).unwrap();

    // Run stats again - the cache comparison will fail due to qsv_version mismatch,
    // triggering recomputation. This tests that older cache files without the new fields
    // don't cause crashes during the comparison logic.
    let mut recompute_stats_cmd = wrk.command("stats");
    recompute_stats_cmd
        .arg("--dataset-stats")
        .args(["--cache-threshold", "1"])
        .arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut recompute_stats_cmd);
    assert!(!got.is_empty(), "Stats should produce output");

    // Verify the JSON file now has the new fields again
    let json_content = std::fs::read_to_string(&json_path).unwrap();
    let json: Value = serde_json::from_str(&json_content).unwrap();
    assert!(
        json.get("field_count").is_some(),
        "field_count should exist after recomputation"
    );
    assert!(
        json.get("filesize_bytes").is_some(),
        "filesize_bytes should exist after recomputation"
    );
    assert!(
        json.get("hash").is_some(),
        "hash should exist after recomputation"
    );
}
