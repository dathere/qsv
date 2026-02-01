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

Common options:
    -h, --help              Display this message
    -o, --output <file>     Write output to <file> instead of stdout.
    -d, --delimiter <arg>   The field delimiter for reading/writing CSV data.
                            Must be a single character. (default: ,)
    -q, --quiet             Do not return smart aggregation chosen nor pivot result shape to stderr.
"#;

use std::{
    collections::HashSet,
    fs::File,
    io,
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
    sync::OnceLock,
};

use csv::ByteRecord;
use indicatif::HumanCount;
use polars::prelude::*;
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

    #[allow(unused_variables)]
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
        let uniqueness_ratio = stats.uniqueness_ratio.unwrap_or(0.0);
        let estimated_row_count = uniqueness_ratio / stats.cardinality as f64;

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
                } else if stats.nullcount as f64 / estimated_row_count > 0.5 {
                    if !quiet {
                        eprintln!("Info: \"{value_col}\" contains >50% NULL values, using Len");
                    }
                    Expr::Element.len()
                } else if stats.cv > Some(1.0) {
                    // High coefficient of variation suggests using median
                    // for better central tendency
                    if !quiet {
                        eprintln!(
                            "Info: High variability in values (CV > 1), using Median for more \
                             robust central tendency"
                        );
                    }
                    Expr::Element.median()
                } else if high_cardinality_pivot && high_cardinality_index {
                    if ordered_pivot && ordered_index {
                        // With ordered high cardinality columns, mean might be more meaningful
                        if !quiet {
                            eprintln!(
                                "Info: Ordered high cardinality columns detected, using Mean"
                            );
                        }
                        Expr::Element.mean()
                    } else {
                        // With unordered high cardinality, sum might be more appropriate
                        if !quiet {
                            eprintln!(
                                "Info: High cardinality in pivot and index columns, using Sum"
                            );
                        }
                        Expr::Element.sum()
                    }
                } else if let Some(skewness) = stats.skewness {
                    if skewness.abs() > 2.0 {
                        // Highly skewed data might benefit from median
                        if !quiet {
                            eprintln!("Info: Highly skewed numeric data detected, using Median");
                        }
                        Expr::Element.median()
                    } else {
                        Expr::Element.sum()
                    }
                } else {
                    Expr::Element.sum()
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
                } else if high_cardinality_pivot || high_cardinality_index {
                    if !quiet {
                        eprintln!("Info: High cardinality detected, using Len");
                    }
                    Expr::Element.len()
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

    let mut pivot_result = lf
        .pivot(
            on_selector,
            on_columns,
            index_selector,
            values_selector,
            agg,
            args.flag_maintain_order,
            separator,
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
