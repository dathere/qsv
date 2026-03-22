static USAGE: &str = r#"
Pivots CSV data using the Polars engine.

The pivot operation consists of:
- One or more index columns (these will be the new rows)
- A column that will be pivoted (this will create the new columns)
- A values column that will be aggregated
- An aggregation function to apply. Features "smart" aggregation auto-selection.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_pivotp.rs.

Usage:
    qsv pivotp [options] <on-cols> <input>
    qsv pivotp --help

pivotp arguments:
    <on-cols>     The column(s) to pivot on (creates new columns).
    <input>       is the input CSV file. The file must have headers.
                  If the file has a pschema.json file, it will be used to
                  inform the pivot operation unless --infer-len is explicitly
                  set to a value other than the default of 10,000 rows.
                  Stdin is not supported.

pivotp options:
    -i, --index <cols>      The column(s) to use as the index (row labels).
                            Specify multiple columns by separating them with a comma.
                            The output will have one row for each unique combination of the index's values.
                            If None, all remaining columns not specified on --on and --values will be used.
                            At least one of --index and --values must be specified.
    -v, --values <cols>     The column(s) containing values to aggregate.
                            If an aggregation is specified, these are the values on which the aggregation
                            will be computed. If None, all remaining columns not specified on --on and --index
                            will be used. At least one of --index and --values must be specified.
    -a, --agg <func>        The aggregation function to use:
                              first - First value encountered
                              last - Last value encountered
                              sum - Sum of values
                              min - Minimum value
                              max - Maximum value
                              mean - Average value
                              median - Median value
                              len - Count of values
                              item - Get single value from group. Raises error if there are multiple values.
                              smart - use value column data type & statistics to pick an aggregation.
                                      When moarstats has been run, leverages outlier profile and
                                      Pearson skewness for smarter selection. With moarstats --advanced,
                                      also uses kurtosis, bimodality, entropy and Gini coefficient.
                                      Will only work if there is one value column, otherwise
                                      it falls back to `first`
                            [default: smart]
    --sort-columns          Sort the transposed columns by name.
    --maintain-order        Maintain the order of the input columns.
    --col-separator <arg>   The separator in generated column names in case of multiple --values columns.
                            [default: _]
    --validate              Validate a pivot by checking the pivot column(s)' cardinality.
    --try-parsedates        When set, will attempt to parse columns as dates.
    --infer-len <arg>       Number of rows to scan when inferring schema.
                            Set to 0 to scan entire file. [default: 10000]
    --decimal-comma         Use comma as decimal separator when READING the input.
                            Note that you will need to specify an alternate --delimiter.
    --ignore-errors         Skip rows that can't be parsed.
    --grand-total           Append a grand total row summing all numeric pivot columns.
                            The first index column will contain "Grand <total-label>".
    --subtotal              Insert subtotal rows after each group in the first index column.
                            The second index column will contain the total label.
                            Requires 2+ index columns.
    --total-label <arg>     Custom label for total rows. [default: Total]

Common options:
    -h, --help              Display this message
    -o, --output <file>     Write output to <file> instead of stdout.
    -d, --delimiter <arg>   The field delimiter for reading/writing CSV data.
                            Must be a single character. (default: ,)
    -q, --quiet             Do not return smart aggregation chosen nor pivot result shape to stderr.
"#;

use std::{
    fs::File,
    io,
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
    sync::OnceLock,
};

use csv::ByteRecord;
use foldhash::HashSet;
use indicatif::HumanCount;
use polars::{frame::PivotColumnNaming, prelude::*};
use serde::Deserialize;

use crate::{
    CliResult,
    cmd::stats::StatsData,
    config::Delimiter,
    util,
    util::{StatsMode, get_stats_records},
};

static STATS_RECORDS: OnceLock<(ByteRecord, Vec<StatsData>)> = OnceLock::new();

/// Helper function to convert a Vec<String> to a vector of Expr for column selection
fn cols_to_exprs(cols: &[String]) -> Vec<Expr> {
    cols.iter().map(col).collect()
}

#[derive(Deserialize)]
struct Args {
    arg_on_cols:         String,
    arg_input:           String,
    flag_index:          Option<String>,
    flag_values:         Option<String>,
    flag_agg:            Option<String>,
    flag_sort_columns:   bool,
    flag_maintain_order: bool,
    flag_col_separator:  String,
    flag_validate:       bool,
    flag_try_parsedates: bool,
    flag_infer_len:      usize,
    flag_decimal_comma:  bool,
    flag_ignore_errors:  bool,
    flag_grand_total:    bool,
    flag_subtotal:       bool,
    flag_total_label:    String,
    flag_output:         Option<String>,
    flag_delimiter:      Option<Delimiter>,
    flag_quiet:          bool,
}

// IMPORTANT: This must be kept in sync with the default value
// of the --infer-len option in the USAGE string above.
const DEFAULT_INFER_LEN: usize = 10000;

/// Structure to hold pivot operation metadata
struct PivotMetadata {
    estimated_columns:    u64,
    on_col_cardinalities: Vec<(String, u64)>,
}

/// Calculate pivot operation metadata using stats information
fn calculate_pivot_metadata(
    args: &Args,
    on_cols: &[String],
    value_cols: Option<&Vec<String>>,
) -> Option<PivotMetadata> {
    // Get stats records
    let schema_args = util::SchemaArgs {
        flag_enum_threshold:  0,
        flag_ignore_case:     false,
        flag_strict_dates:    false,
        flag_strict_formats:  false,
        flag_pattern_columns: crate::select::SelectColumns::parse("").unwrap(),
        flag_dates_whitelist: String::new(),
        flag_prefer_dmy:      false,
        flag_force:           false,
        flag_stdout:          false,
        flag_jobs:            None,
        flag_polars:          false,
        flag_no_headers:      false,
        flag_delimiter:       args.flag_delimiter,
        arg_input:            Some(args.arg_input.clone()),
        flag_memcheck:        false,
        flag_output:          None,
    };

    let (csv_fields, csv_stats) = STATS_RECORDS.get_or_init(|| {
        get_stats_records(&schema_args, StatsMode::FrequencyForceStats)
            .unwrap_or_else(|_| (ByteRecord::new(), Vec::new()))
    });

    if csv_stats.is_empty() {
        return None;
    }

    // Get cardinalities for pivot columns
    let mut on_col_cardinalities = Vec::with_capacity(on_cols.len());
    let mut total_new_columns: u64 = 1;

    for on_col in on_cols {
        if let Some(pos) = csv_fields
            .iter()
            .position(|f| std::str::from_utf8(f).unwrap_or("") == on_col)
        {
            let cardinality = csv_stats[pos].cardinality;
            total_new_columns = total_new_columns.saturating_mul(cardinality);
            on_col_cardinalities.push((on_col.clone(), cardinality));
        }
    }

    // Calculate total columns in result
    let value_cols_count = match value_cols {
        Some(cols) => cols.len() as u64,
        None => 1,
    };
    let estimated_columns = total_new_columns.saturating_mul(value_cols_count);

    Some(PivotMetadata {
        estimated_columns,
        on_col_cardinalities,
    })
}

/// Validate pivot operation using metadata
fn validate_pivot_operation(metadata: &PivotMetadata) -> CliResult<()> {
    const COLUMN_WARNING_THRESHOLD: u64 = 1000;

    // Print cardinality information
    if metadata.on_col_cardinalities.len() > 1 {
        eprintln!("Pivot <on-cols> cardinalities:");
    } else {
        eprintln!("Pivot on-column cardinality:");
    }
    for (col, card) in &metadata.on_col_cardinalities {
        eprintln!("  {col}: {}", HumanCount(*card));
    }

    // Warn about large number of columns
    if metadata.estimated_columns > COLUMN_WARNING_THRESHOLD {
        eprintln!(
            "Warning: Pivot will create {} columns. This might impact performance.",
            HumanCount(metadata.estimated_columns)
        );
    }

    // Error if operation would create an unreasonable number of columns
    if metadata.estimated_columns > 100_000 {
        return fail_clierror!(
            "Pivot would create too many columns ({}). Consider reducing the number of pivot \
             columns or using a different approach.",
            HumanCount(metadata.estimated_columns)
        );
    }

    Ok(())
}

/// Build a single-row DataFrame containing a total row.
/// Numeric columns get their sum; index columns get the provided labels.
fn build_total_row(
    df: &DataFrame,
    index_cols: &[String],
    first_index_label: &str,
    second_index_label: &str,
) -> CliResult<DataFrame> {
    let mut columns: Vec<Column> = Vec::with_capacity(df.width());

    for col_ref in df.columns() {
        let name = col_ref.name().to_string();

        if name == index_cols[0] {
            // First index column gets the primary label
            columns.push(Column::new(name.into(), &[first_index_label]));
        } else if index_cols.len() > 1 && name == index_cols[1] {
            // Second index column gets the secondary label
            columns.push(Column::new(name.into(), &[second_index_label]));
        } else if index_cols.iter().any(|ic| ic == &name) {
            // Other index columns get empty string
            columns.push(Column::new(name.into(), &[""]));
        } else if col_ref.dtype().is_numeric() {
            // Sum numeric columns
            let sum_series = col_ref.sum_reduce()?.into_column(name.into());
            columns.push(sum_series);
        } else {
            // Non-numeric pivot columns get empty string
            columns.push(Column::new(name.into(), &[""]));
        }
    }

    Ok(DataFrame::new(1, columns)?)
}

/// Insert subtotal rows after each group in the first index column.
/// Returns a new DataFrame with subtotal rows interleaved.
/// Subtotals are inserted based on the existing row order.
fn insert_subtotals(df: &DataFrame, index_cols: &[String], label: &str) -> CliResult<DataFrame> {
    // Sort by all index columns to ensure contiguous groups and
    // deterministic intra-group ordering, since pivot output order is not guaranteed.
    let df = df.sort(index_cols, SortMultipleOptions::default())?;

    let group_col = df.column(&index_cols[0])?;
    let mut frames: Vec<DataFrame> = Vec::new();
    let mut group_start = 0_usize;

    for i in 1..=df.height() {
        // Detect group boundary: end of data or value change in first index col
        let current_val = if i < df.height() {
            group_col.get(i).map(|v| v.to_string()).ok()
        } else {
            None
        };
        let start_val = group_col
            .get(group_start)
            .map(|v| v.to_string())
            .ok()
            .unwrap_or_default();
        let is_boundary = i == df.height() || current_val.as_deref() != Some(&start_val);

        if is_boundary {
            let group_len = i - group_start;
            let group_df = df.slice(group_start as i64, group_len);

            // Get the group value for the first index column label
            let group_value = if let Ok(str_col) = group_col.str() {
                str_col.get(group_start).unwrap_or_default().to_string()
            } else {
                group_col
                    .get(group_start)
                    .map(|v| v.to_string())
                    .ok()
                    .unwrap_or_default()
            };

            // Build subtotal row: first index col = group value, second = label
            let subtotal_row = build_total_row(&group_df, index_cols, &group_value, label)?;

            frames.push(group_df);
            frames.push(subtotal_row);
            group_start = i;
        }
    }

    // Stack all frames together
    if frames.is_empty() {
        return Ok(df.clone());
    }

    let mut result = frames.remove(0);
    for frame in frames {
        result.vstack_mut(&frame)?;
    }

    Ok(result)
}

/// Suggest an appropriate aggregation function based on column statistics
#[allow(clippy::cast_precision_loss)]
fn suggest_agg_function(
    args: &Args,
    on_cols: &[String],
    index_cols: Option<&[String]>,
    value_cols: &[String],
) -> CliResult<Option<Expr>> {
    // If multiple value columns, default to First
    if value_cols.len() > 1 {
        return Ok(Some(Expr::Element.first()));
    }

    let quiet = args.flag_quiet;

    // Get stats for all columns with enhanced statistics
    let schema_args = util::SchemaArgs {
        flag_enum_threshold:  0,
        flag_ignore_case:     false,
        flag_strict_dates:    false,
        flag_strict_formats:  false,
        flag_pattern_columns: crate::select::SelectColumns::parse("").unwrap(),
        flag_dates_whitelist: String::new(),
        flag_prefer_dmy:      false,
        flag_force:           false,
        flag_stdout:          false,
        flag_jobs:            None,
        flag_polars:          false,
        flag_no_headers:      false,
        flag_delimiter:       args.flag_delimiter,
        arg_input:            Some(args.arg_input.clone()),
        flag_memcheck:        false,
        flag_output:          None,
    };

    let (csv_fields, csv_stats) = STATS_RECORDS.get_or_init(|| {
        get_stats_records(&schema_args, StatsMode::FrequencyForceStats)
            .unwrap_or_else(|_| (ByteRecord::new(), Vec::new()))
    });

    // Analyze pivot column characteristics
    let mut high_cardinality_pivot = false;
    let mut ordered_pivot = false; // Track if pivot columns are ordered
    for on_col in on_cols {
        if let Some(pos) = csv_fields
            .iter()
            .position(|f| std::str::from_utf8(f).unwrap_or("") == on_col)
        {
            let stats = &csv_stats[pos];
            let uniqueness_ratio = stats.uniqueness_ratio.unwrap_or(0.0);

            // Check cardinality ratio

            if uniqueness_ratio > 0.5 {
                high_cardinality_pivot = true;
                if !quiet {
                    eprintln!("Info: Pivot column \"{on_col}\" has high cardinality");
                }
            }

            // Check if column is unordered based on sort_order
            if let Some(sort_order) = &stats.sort_order {
                ordered_pivot = sort_order != "Unsorted";
            }
        }
    }

    // Analyze index column characteristics
    let mut high_cardinality_index = false;
    let mut ordered_index = false;
    if let Some(idx_cols) = index_cols {
        for idx_col in idx_cols {
            if let Some(pos) = csv_fields
                .iter()
                .position(|f| std::str::from_utf8(f).unwrap_or("") == idx_col)
            {
                let stats = &csv_stats[pos];
                let uniqueness_ratio = stats.uniqueness_ratio.unwrap_or(0.0);

                // Check uniqueness ratio
                if uniqueness_ratio > 0.5 {
                    high_cardinality_index = true;
                    if !quiet {
                        eprintln!("Info: Index column \"{idx_col}\" has high uniqueness ratio");
                    }
                }

                // Check if column is unordered
                if let Some(sort_order) = &stats.sort_order {
                    ordered_index = sort_order != "Unsorted";
                }
            }
        }
    }

    // Get stats for the value column
    let value_col = &value_cols[0];
    let field_pos = csv_fields
        .iter()
        .position(|f| std::str::from_utf8(f).unwrap_or("") == value_col);

    if let Some(pos) = field_pos {
        let stats = &csv_stats[pos];

        // Suggest aggregation based on field type and statistics
        let suggested_agg = match stats.r#type.as_str() {
            "NULL" => {
                if !quiet {
                    eprintln!("Info: \"{value_col}\" contains only NULL values");
                }
                Expr::Element.len()
            },
            "Integer" | "Float" => {
                if stats.cardinality == 1 {
                    if !quiet {
                        eprintln!("Info: \"{value_col}\" contains only one value, using Item");
                    }
                    Expr::Element.item(true)
                } else if stats.sparsity.unwrap_or(0.0) > 0.5 {
                    if !quiet {
                        eprintln!("Info: \"{value_col}\" contains >50% NULL values, using Len");
                    }
                    Expr::Element.len()
                } else if let Some(bc) = stats.bimodality_coefficient {
                    if bc >= 0.555 {
                        // Bimodal distribution — central tendency is misleading
                        if !quiet {
                            eprintln!(
                                "Info: Bimodal distribution detected (bimodality coefficient \
                                 {bc:.3} >= 0.555), using Len"
                            );
                        }
                        Expr::Element.len()
                    } else {
                        suggest_numeric_after_bimodality(
                            stats,
                            quiet,
                            value_col,
                            high_cardinality_pivot,
                            high_cardinality_index,
                            ordered_pivot,
                            ordered_index,
                        )
                    }
                } else {
                    suggest_numeric_after_bimodality(
                        stats,
                        quiet,
                        value_col,
                        high_cardinality_pivot,
                        high_cardinality_index,
                        ordered_pivot,
                        ordered_index,
                    )
                }
            },
            "Date" | "DateTime" => {
                if stats.cardinality == 1 {
                    if !quiet {
                        eprintln!("Info: \"{value_col}\" contains only one value, using Item");
                    }
                    Expr::Element.item(true)
                } else if high_cardinality_pivot || high_cardinality_index {
                    if ordered_pivot && ordered_index {
                        if !quiet {
                            eprintln!(
                                "Info: Ordered temporal data with high cardinality, using Last"
                            );
                        }
                        Expr::Element.last()
                    } else {
                        if !quiet {
                            eprintln!(
                                "Info: High cardinality detected, using First for {} column",
                                stats.r#type
                            );
                        }
                        Expr::Element.first()
                    }
                } else {
                    if !quiet {
                        eprintln!("Info: Using Len for {} column", stats.r#type);
                    }
                    Expr::Element.len()
                }
            },
            _ => {
                let uniqueness_ratio = stats.uniqueness_ratio.unwrap_or(0.0);
                if stats.cardinality == 1 {
                    if !quiet {
                        eprintln!("Info: \"{value_col}\" contains only one value, using Item");
                    }
                    Expr::Element.item(true)
                } else if (uniqueness_ratio - 1.0).abs() < 0.0001 {
                    if !quiet {
                        eprintln!("Info: \"{value_col}\" contains all unique values, using First");
                    }
                    Expr::Element.first()
                } else if stats.sparsity > Some(0.5) {
                    if !quiet {
                        eprintln!("Info: Sparse data detected, using Len");
                    }
                    Expr::Element.len()
                } else if let Some(ne) = stats.normalized_entropy {
                    if ne > 0.9 {
                        if !quiet {
                            eprintln!(
                                "Info: Near-uniform string distribution (normalized entropy \
                                 {ne:.3} > 0.9), using Len"
                            );
                        }
                        Expr::Element.len()
                    } else if high_cardinality_pivot || high_cardinality_index {
                        if !quiet {
                            eprintln!("Info: High cardinality detected, using First");
                        }
                        Expr::Element.first()
                    } else {
                        // Low entropy means few dominant values — First is more
                        // informative than a count
                        if !quiet {
                            eprintln!("Info: Low entropy string column, using First");
                        }
                        Expr::Element.first()
                    }
                } else if high_cardinality_pivot || high_cardinality_index {
                    if !quiet {
                        eprintln!("Info: High cardinality detected, using First");
                    }
                    Expr::Element.first()
                } else {
                    if !quiet {
                        eprintln!("Info: Using Len for String column");
                    }
                    Expr::Element.len()
                }
            },
        };

        Ok(Some(suggested_agg))
    } else {
        Ok(None)
    }
}

/// Helper for numeric aggregation decisions after the bimodality check.
/// Contains the remaining moarstats-aware checks (outliers, kurtosis, Gini)
/// followed by the original CV, cardinality, and skewness logic.
#[allow(clippy::fn_params_excessive_bools)]
fn suggest_numeric_after_bimodality(
    stats: &StatsData,
    quiet: bool,
    value_col: &str,
    high_cardinality_pivot: bool,
    high_cardinality_index: bool,
    ordered_pivot: bool,
    ordered_index: bool,
) -> Expr {
    // moarstats: outlier impact ratio — mean shifted >20% by outliers
    if let Some(oir) = stats.outlier_impact_ratio
        && oir > 0.2
    {
        if !quiet {
            eprintln!("Info: High outlier impact (ratio {oir:.3} > 0.2) shifts mean, using Median");
        }
        return Expr::Element.median();
    }

    // moarstats: >15% outlier contamination
    if let Some(op) = stats.outliers_percentage
        && op > 15.0
    {
        if !quiet {
            eprintln!("Info: High outlier percentage ({op:.1}% > 15%), using Median");
        }
        return Expr::Element.median();
    }

    // moarstats: leptokurtic / heavy-tailed distribution
    if let Some(k) = stats.kurtosis
        && k > 3.0
    {
        if !quiet {
            eprintln!("Info: Heavy-tailed distribution (kurtosis {k:.3} > 3.0), using Median");
        }
        return Expr::Element.median();
    }

    // moarstats: high inequality — few values dominate
    if let Some(gc) = stats.gini_coefficient
        && gc > 0.6
    {
        if !quiet {
            eprintln!("Info: High inequality (Gini coefficient {gc:.3} > 0.6), using Median");
        }
        return Expr::Element.median();
    }

    // Original: high CV
    if stats.cv > Some(1.0) {
        if !quiet {
            eprintln!(
                "Info: High variability in values (CV > 1), using Median for more robust central \
                 tendency"
            );
        }
        return Expr::Element.median();
    }

    // Original: high cardinality pivot + index
    if high_cardinality_pivot && high_cardinality_index {
        // moarstats: near-uniform distribution makes aggregation uninformative
        if let Some(ne) = stats.normalized_entropy
            && ne > 0.9
        {
            if !quiet {
                eprintln!(
                    "Info: Near-uniform distribution (normalized entropy {ne:.3} > 0.9), using Len"
                );
            }
            return Expr::Element.len();
        }

        if ordered_pivot && ordered_index {
            if !quiet {
                eprintln!("Info: Ordered high cardinality columns detected, using Mean");
            }
            return Expr::Element.mean();
        }
        if !quiet {
            eprintln!("Info: High cardinality in pivot and index columns, using Sum");
        }
        return Expr::Element.sum();
    }

    // Original: skewness check
    if let Some(skewness) = stats.skewness
        && skewness.abs() > 2.0
    {
        if !quiet {
            eprintln!("Info: Highly skewed numeric data detected, using Median");
        }
        return Expr::Element.median();
    }

    // moarstats: Pearson skewness as complementary measure
    if let Some(ps) = stats.pearson_skewness
        && ps.abs() > 1.0
    {
        if !quiet {
            eprintln!(
                "Info: Pearson skewness ({ps:.3}, |value| > 1.0) indicates asymmetry, using Median"
            );
        }
        return Expr::Element.median();
    }

    // Default for numeric
    if !quiet {
        eprintln!("Info: Using Sum for \"{value_col}\"");
    }
    Expr::Element.sum()
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Parse on column(s)
    let on_cols: Vec<String> = args
        .arg_on_cols
        .as_str()
        .split(',')
        .map(std::string::ToString::to_string)
        .collect();

    // Parse index column(s)
    let index_cols = if let Some(ref flag_index) = args.flag_index {
        let idx_cols: Vec<String> = flag_index
            .as_str()
            .split(',')
            .map(std::string::ToString::to_string)
            .collect();
        Some(idx_cols)
    } else {
        None
    };

    // Parse values column(s)
    let value_cols = if let Some(ref flag_values) = args.flag_values {
        let val_cols: Vec<String> = flag_values
            .as_str()
            .split(',')
            .map(std::string::ToString::to_string)
            .collect();
        Some(val_cols)
    } else {
        None
    };

    if index_cols.is_none() && value_cols.is_none() {
        return fail_incorrectusage_clierror!(
            "Either --index <cols> or --values <cols> must be specified."
        );
    }

    // Get aggregation function - using generic expressions that pivot will apply to value columns
    let agg_expr = if let Some(ref agg) = args.flag_agg {
        let lower_agg = agg.to_lowercase();
        if lower_agg == "none" {
            None
        } else {
            Some(match lower_agg.as_str() {
                "first" => Expr::Element.first(),
                "last" => Expr::Element.last(),
                "sum" => Expr::Element.sum(),
                "min" => Expr::Element.min(),
                "max" => Expr::Element.max(),
                "mean" => Expr::Element.mean(),
                "median" => Expr::Element.median(),
                "len" => Expr::Element.len(),
                "item" => Expr::Element.item(true),
                "smart" => {
                    if let Some(value_cols) = &value_cols {
                        // Try to suggest an appropriate aggregation function
                        match suggest_agg_function(
                            &args,
                            &on_cols,
                            index_cols.as_deref(),
                            value_cols,
                        )? {
                            Some(suggested_agg) => suggested_agg,
                            _ => {
                                // fallback to first, which always works
                                Expr::Element.first()
                            },
                        }
                    } else {
                        // Default to Len if no value columns specified
                        Expr::Element.len()
                    }
                },
                _ => {
                    return fail_incorrectusage_clierror!(
                        "Invalid pivot aggregation function: {agg}"
                    );
                },
            })
        }
    } else {
        None
    };

    // Set delimiter if specified
    let delim = if let Some(delimiter) = args.flag_delimiter {
        delimiter.as_byte()
    } else {
        b','
    };

    if args.flag_decimal_comma && delim == b',' {
        return fail_incorrectusage_clierror!(
            "You need to specify an alternate --delimiter when using --decimal-comma."
        );
    }

    // Create CSV reader config
    let mut csv_reader = LazyCsvReader::new(PlRefPath::new(&args.arg_input))
        .with_has_header(true)
        .with_try_parse_dates(args.flag_try_parsedates)
        .with_decimal_comma(args.flag_decimal_comma)
        .with_separator(delim)
        .with_ignore_errors(args.flag_ignore_errors);

    // check if the pschema.json file exists and is newer or created at the same time
    // as the table file
    let input_path = Path::new(&args.arg_input);
    let schema_file = PathBuf::from(format!(
        "{}.pschema.json",
        input_path.canonicalize()?.display()
    ));
    let valid_schema_exists = schema_file.exists()
        && schema_file.metadata()?.modified()? >= input_path.metadata()?.modified()?;

    if valid_schema_exists && args.flag_infer_len == DEFAULT_INFER_LEN {
        // Use schema from pschema.json file if it exists and is valid
        // and the user did not specify a custom inference length
        let file = File::open(&schema_file)?;
        let mut buf_reader = BufReader::new(file);
        let mut schema_json = String::with_capacity(100);
        buf_reader.read_to_string(&mut schema_json)?;
        let schema: Schema = serde_json::from_str(&schema_json)?;
        csv_reader = csv_reader.with_schema(Some(Arc::new(schema)));
    } else {
        // Otherwise we infer the schema using inference length (default or user-specified)
        csv_reader = csv_reader.with_infer_schema_length(Some(args.flag_infer_len));
    }

    // Read the CSV into a LazyFrame
    let mut lf = csv_reader.finish()?;

    // Add a row index to track discovery order
    let row_order_col = "__qsv_row_order__";
    lf = lf.with_row_index(PlSmallStr::from_str(row_order_col), None);

    // Get all column names from the schema to compute index/values if not specified
    let schema = lf.collect_schema()?;
    let all_cols: Vec<String> = schema
        .iter_names()
        .filter(|name| *name != row_order_col)
        .map(polars::prelude::PlSmallStr::to_string)
        .collect();

    // Compute actual index and value columns if not specified
    let actual_index_cols: Vec<String> = if let Some(idx_cols) = index_cols {
        idx_cols
    } else {
        // If no index specified, use all columns except on and values
        let on_set: HashSet<&str> = on_cols.iter().map(std::string::String::as_str).collect();
        let value_set: HashSet<&str> = value_cols
            .as_ref()
            .map(|cols| cols.iter().map(std::string::String::as_str).collect())
            .unwrap_or_default();

        all_cols
            .iter()
            .filter(|c| !on_set.contains(c.as_str()) && !value_set.contains(c.as_str()))
            .cloned()
            .collect()
    };

    let actual_value_cols: Vec<String> = if let Some(cols) = value_cols {
        cols
    } else {
        // If no values specified, use all columns except on and index
        let on_set: HashSet<&str> = on_cols.iter().map(std::string::String::as_str).collect();
        let index_set: HashSet<&str> = actual_index_cols
            .iter()
            .map(std::string::String::as_str)
            .collect();

        all_cols
            .iter()
            .filter(|c| !on_set.contains(c.as_str()) && !index_set.contains(c.as_str()))
            .cloned()
            .collect()
    };

    if args.flag_validate {
        // Validate the operation - need to collect to get metadata
        let df_for_validation = lf.clone().collect()?;
        if let Some(metadata) = calculate_pivot_metadata(&args, &on_cols, Some(&actual_value_cols))
        {
            validate_pivot_operation(&metadata)?;
        }
        drop(df_for_validation);
    }

    // Compute unique values for the pivot columns to create on_columns DataFrame
    // This is required by the new LazyFrame pivot API
    // We need to maintain discovery order, so we add row numbers and sort by them
    let on_columns = {
        let on_exprs_with_order: Vec<Expr> = cols_to_exprs(&on_cols)
            .into_iter()
            .chain(std::iter::once(col(row_order_col)))
            .collect();

        let unique_df = lf
            .clone()
            .select(on_exprs_with_order)
            .unique(
                Some(cols(on_cols.iter().map(std::string::String::as_str))),
                UniqueKeepStrategy::First,
            )
            .sort([row_order_col], SortMultipleOptions::default())
            .drop(cols([row_order_col]))
            .collect()?;

        Arc::new(unique_df)
    };

    // Create the aggregation expression
    // If agg_expr is None, we need a default
    let agg = agg_expr.unwrap_or_else(|| Expr::Element.first());

    // Convert separator to PlSmallStr
    let separator = PlSmallStr::from_str(&args.flag_col_separator);

    // Compute the minimum row order for each unique index combination
    // This will be used to restore discovery order after pivoting
    // for deterministic output
    let index_order = if actual_index_cols.is_empty() {
        None
    } else {
        let order_df = lf
            .clone()
            .select(
                actual_index_cols
                    .iter()
                    .map(col)
                    .chain(std::iter::once(col(row_order_col)))
                    .collect::<Vec<_>>(),
            )
            .group_by(actual_index_cols.iter().map(col).collect::<Vec<_>>())
            .agg([col(row_order_col).min().alias(row_order_col)])
            .collect()?;
        Some(order_df)
    };

    // Perform pivot operation using the new LazyFrame.pivot API
    // The API expects: on (Selector), on_columns (Arc<DataFrame>), index (Selector),
    // values (Selector), agg (Expr), maintain_order (bool), separator (PlSmallStr)
    let on_selector = cols(on_cols.iter().map(std::string::String::as_str));
    let index_selector = cols(actual_index_cols.iter().map(std::string::String::as_str));
    let values_selector = cols(actual_value_cols.iter().map(std::string::String::as_str));
    let column_naming = PivotColumnNaming::default(); // Use default column naming convention

    let mut pivot_result = lf
        .pivot(
            on_selector,
            on_columns,
            index_selector,
            values_selector,
            agg,
            args.flag_maintain_order,
            separator,
            column_naming,
        )
        .collect()?;

    // Restore discovery order by joining with index_order and sorting
    if let Some(index_order_df) = index_order {
        pivot_result = pivot_result
            .lazy()
            .join(
                index_order_df.lazy(),
                &actual_index_cols.iter().map(col).collect::<Vec<_>>(),
                &actual_index_cols.iter().map(col).collect::<Vec<_>>(),
                JoinArgs::new(JoinType::Left),
            )
            .sort([row_order_col], SortMultipleOptions::default())
            .drop(cols([row_order_col]))
            .collect()?;
    }

    // Sort columns if requested
    if args.flag_sort_columns {
        let columns = pivot_result
            .get_column_names()
            .into_iter()
            .map(polars::prelude::PlSmallStr::to_string);
        let index_cols_set: HashSet<_> = actual_index_cols
            .iter()
            .map(std::string::String::as_str)
            .collect();

        // Separate index and pivoted columns
        let (index_names, mut pivot_names): (Vec<_>, Vec<_>) = columns
            .into_iter()
            .partition(|name| index_cols_set.contains(name.as_str()));

        // Sort only the pivoted columns alphabetically
        pivot_names.sort_unstable();

        // Reconstruct column order: index columns first, then sorted pivot columns
        let mut sorted_columns = index_names;
        sorted_columns.extend(pivot_names);

        pivot_result = pivot_result.select(sorted_columns.as_slice())?;
    }

    // Add subtotals and/or grand total rows
    if args.flag_subtotal || args.flag_grand_total {
        if args.flag_grand_total && actual_index_cols.is_empty() {
            return fail_incorrectusage_clierror!(
                "--grand-total requires at least 1 index column (specified via --index)."
            );
        }
        if args.flag_subtotal && actual_index_cols.len() < 2 {
            return fail_incorrectusage_clierror!(
                "--subtotal requires 2 or more index columns (specified via --index)."
            );
        }

        // Cast index columns to String so we can insert label text
        for idx_col in &actual_index_cols {
            let col_data = pivot_result.column(idx_col)?;
            if col_data.dtype() != &DataType::String {
                let casted = col_data.cast(&DataType::String)?;
                pivot_result.replace(idx_col, casted)?;
            }
        }

        let total_label = &args.flag_total_label;

        // Compute grand total from pivot result (before subtotals are inserted)
        let grand_total_row = if args.flag_grand_total {
            Some(build_total_row(
                &pivot_result,
                &actual_index_cols,
                &format!("Grand {total_label}"),
                "",
            )?)
        } else {
            None
        };

        // Insert subtotals
        if args.flag_subtotal {
            pivot_result = insert_subtotals(&pivot_result, &actual_index_cols, total_label)?;
        }

        // Append grand total
        if let Some(gt_row) = grand_total_row {
            pivot_result = pivot_result.vstack(&gt_row)?;
        }
    }

    // Write output
    let mut writer = match args.flag_output {
        Some(ref output_file) => {
            // no need to use buffered writer here, as CsvWriter already does that
            let path = Path::new(&output_file);
            Box::new(File::create(path).unwrap()) as Box<dyn Write>
        },
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };
    let datetime_fmt: PlSmallStr = PlSmallStr::from_str("%Y-%m-%d %H:%M:%S");
    CsvWriter::new(&mut writer)
        .include_header(true)
        .with_datetime_format(Some(datetime_fmt))
        .with_separator(delim)
        .finish(&mut pivot_result)?;

    // Print shape to stderr
    if !args.flag_quiet {
        eprintln!("{:?}", pivot_result.shape());
    }

    Ok(())
}
