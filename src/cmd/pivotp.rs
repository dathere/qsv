pub(crate) static USAGE: &str = r#"
Pivots or groups CSV data using the Polars engine.

PIVOT MODE (with <on-cols>):
  The pivot operation consists of:
  - One or more index columns (these will be the new rows)
  - A column that will be pivoted (this will create the new columns)
  - A values column that will be aggregated
  - An aggregation function to apply. Features "smart" aggregation auto-selection.

GROUP-BY MODE (without <on-cols>):
  When <on-cols> is omitted, performs a group-by aggregation instead of a pivot.
  This is useful for simple aggregations like counting rows per group.
  In group-by mode, --index is required and --agg smart resolves to len (count).
  The none aggregation is not supported in group-by mode.
  If --values is omitted, a single "count" column is produced.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_pivotp.rs.
See also https://github.com/dathere/qsv/wiki/SQL-and-Polars#pivotp

Usage:
    qsv pivotp [options] <on-cols> <input>
    qsv pivotp [options] <input>
    qsv pivotp --help

pivotp arguments:
    <on-cols>     The column(s) to pivot on (creates new columns).
                  When omitted, pivotp runs in group-by mode.
    <input>       The input CSV file. The file must have headers.
                  If the file has a pschema.json file, it will be used to
                  inform the pivot operation unless --infer-len is explicitly
                  set to a value other than the default of 10,000 rows.
                  Stdin is not supported. A snappy-compressed (.sz) input is
                  decompressed first, and an --output ending in .sz is
                  snappy-compressed. Lines starting with QSV_COMMENT_CHAR are
                  skipped. QSV_POLARS_FLOAT_PRECISION and QSV_OUTPUT_BOM apply
                  to the output, as in sqlp and joinp.

pivotp options:
    -i, --index <cols>      The column(s) to use as the index (row labels).
                            Specify multiple columns by separating them with a comma.
                            The output will have one row for each unique combination of the index's values.
                            In pivot mode, if None, all remaining columns not specified on --on and --values
                            will be used; at least one of --index and --values must be specified.
                            Required in group-by mode.
    -v, --values <cols>     The column(s) containing values to aggregate.
                            If an aggregation is specified, these are the values on which the aggregation
                            will be computed.
                            In pivot mode, if None, all remaining columns not specified on --on and --index
                            will be used; at least one of --index and --values must be specified.
                            In group-by mode, if omitted, a single "count" column is produced.
    -a, --agg <func>        The aggregation function to use:
                              first - First value encountered
                              last - Last value encountered
                              sum - Sum of values
                              min - Minimum value
                              max - Maximum value
                              mean - Average value
                              median - Median value
                              quantile@<p> - Quantile at probability p in [0, 1] using linear interpolation.
                                             Alias: q@<p>. Examples: quantile@0.95, q@0.5
                                             (q@0.5 is equivalent to median for even-length groups).
                              len - Count of values
                              item - Get single value from group. Raises error if there are multiple values.
                              smart - use value column data type & statistics to pick an aggregation.
                                      Always uses type, cardinality, sparsity, CV, sign
                                      distribution (n_negative/n_positive), and sort_order
                                      from streaming stats.
                                      High variability means a CV above 100% (stddev > mean).
                                      When the stats cache includes non-streaming stats (from a
                                      prior `stats --everything` or `stats --mode`), also uses
                                      mode_count.
                                      With a single --values column, smart first runs
                                      moarstats --advanced to enrich the input's stats cache
                                      (writing <stem>.stats.csv, .stats.csv.json and
                                      .stats.csv.data.jsonl beside it) unless the cache already
                                      has those stats. This one-time pass reads the whole file;
                                      skip it with --no-moarstats. It is also skipped with an
                                      explicit --delimiter, stdin and special formats.
                                      The moarstats stats used are outlier profile, Pearson
                                      skewness, MAD/stddev ratio, median/mean ratio and quartile
                                      coefficient of dispersion (the two ratios only for
                                      positive data), moment skewness (|G1| >= 1 picks median;
                                      Pearson skewness is the fallback), kurtosis, bimodality
                                      (bimodality coefficient >= 0.60 with negative excess
                                      kurtosis), entropy and Gini coefficient.
                                      A single-value column uses first.
                                      The choice depends on the data, so pass an explicit --agg
                                      to pin a result across qsv versions.
                                      For Date/DateTime values, checks sparsity and sort order.
                                      Will only work if there is one value column, otherwise
                                      it falls back to `first`
                            [default: smart]
    --sort-columns          Sort the transposed columns by name. (pivot mode only)
    --maintain-order        Accepted for compatibility. Output is always in first-appearance
                            order: rows follow their index values' first appearance and
                            pivoted columns their <on-cols> values' first appearance.
    --col-separator <arg>   The separator in generated column names in case of multiple --values columns.
                            (pivot mode only; ignored in group-by mode) [default: _]
    --validate              Report the pivot column(s)' cardinality before pivoting. Informational
                            only - it does not refuse a pivot; --max-columns does that.
                            (pivot mode only)
    --max-columns <n>       Maximum number of columns the pivot may create before it is refused.
                            Guards against pivoting on a high-cardinality column, which can
                            exhaust memory. Unlike --validate, this ALWAYS runs.
                            Set to 0 for no limit, for a deliberately wide pivot.
                            (pivot mode only; ignored in group-by mode) [default: 100000]
    --try-parsedates        When set, will attempt to parse columns as dates.
    --infer-len <arg>       Number of rows to scan when inferring schema.
                            Set to 0 to scan entire file. [default: 10000]
    --decimal-comma         Use comma as decimal separator when READING the input.
                            Note that you will need to specify an alternate --delimiter.
    --ignore-errors         Skip rows that can't be parsed.
    --grand-total           Append a grand total row summing all numeric non-index columns.
                            The first index column will contain "Grand <total-label>".
    --subtotal              Insert subtotal rows after each group in the first index column.
                            The second index column will contain the total label.
                            Requires 2+ index columns. (pivot mode only)
    --total-label <arg>     Custom label for total rows. [default: Total]
    --no-moarstats          For smart aggregation, do not run moarstats --advanced to
                            enrich the stats cache. Smart aggregation then uses whatever
                            stats are already cached (or basic `stats --cardinality` stats).

Common options:
    -h, --help              Display this message
    -o, --output <file>     Write output to <file> instead of stdout.
    -d, --delimiter <arg>   The field delimiter for reading/writing CSV data.
                            Must be a single character. If not set, QSV_DEFAULT_DELIMITER
                            is used, else ','. A .tsv/.tab (tab) or .ssv (semicolon)
                            extension on the input or --output file sets that file's
                            delimiter. (default: ,)
    -q, --quiet             Do not return smart aggregation chosen nor pivot result shape to stderr.
"#;

use std::{
    fs::File,
    io,
    io::Write,
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
    cmd::{
        joinp::{path_delim, tsvssv_delim},
        sqlp::compress_output_if_needed,
        stats::StatsData,
    },
    config::Delimiter,
    util,
    util::{StatsMode, get_stats_records},
};

static STATS_RECORDS: OnceLock<(ByteRecord, Vec<StatsData>)> = OnceLock::new();

/// Helper function to convert a Vec<String> to a vector of Expr for column selection
fn cols_to_exprs(cols: &[String]) -> Vec<Expr> {
    cols.iter().map(col).collect()
}

/// Parse `quantile@<p>` or `q@<p>` -> Some(p) iff p ∈ [0, 1]; else None.
fn parse_quantile_agg(s: &str) -> Option<f64> {
    let suffix = s
        .strip_prefix("quantile@")
        .or_else(|| s.strip_prefix("q@"))?;
    let p: f64 = suffix.parse().ok()?;
    (p.is_finite() && (0.0..=1.0).contains(&p)).then_some(p)
}

/// An aggregation, applicable to a column (group-by mode, and the pre-aggregation in pivot mode).
#[derive(Clone, Copy, Debug, PartialEq)]
enum AggKind {
    First,
    Last,
    Sum,
    Min,
    Max,
    Mean,
    Median,
    Len,
    Item,
    Quantile(f64),
}

impl AggKind {
    /// Parse an explicit (already lowercased) `--agg` name. `smart` and `none` are resolved by
    /// the caller.
    fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "first" => Self::First,
            "last" => Self::Last,
            "sum" => Self::Sum,
            "min" => Self::Min,
            "max" => Self::Max,
            "mean" => Self::Mean,
            "median" => Self::Median,
            "len" => Self::Len,
            "item" => Self::Item,
            s => Self::Quantile(parse_quantile_agg(s)?),
        })
    }

    /// The aggregation applied to `e`. `Len` counts rows, nulls included.
    fn on(self, e: Expr) -> Expr {
        match self {
            Self::First => e.first(),
            Self::Last => e.last(),
            Self::Sum => e.sum(),
            Self::Min => e.min(),
            Self::Max => e.max(),
            Self::Mean => e.mean(),
            Self::Median => e.median(),
            Self::Len => e.len(),
            Self::Item => e.item(true),
            Self::Quantile(p) => e.quantile(lit(p), QuantileMethod::Linear),
        }
    }

    /// Should pivot mode aggregate the (index, pivot column) groups before pivoting? Median and
    /// quantiles are already cheap to pivot directly (Polars runs them on its in-memory engine,
    /// ~300 MB for a 50 x 2000 pivot of a 65 MB CSV), and pre-aggregating them costs ~90 MB more.
    /// Every other aggregation costs ~1.4 MB per output column when pivoted directly.
    const fn pre_aggregates(self) -> bool {
        !matches!(self, Self::Median | Self::Quantile(_))
    }

    /// Polars' pivot reports an EMPTY cell (no rows for that index/column pair) as 0 for these
    /// aggregations and as null for every other one.
    const fn empty_cell_is_zero(self) -> bool {
        matches!(self, Self::Sum | Self::Len)
    }
}

/// Build the `SchemaArgs` used for stats/frequency lookups.
/// Extracted to avoid duplicated construction at multiple call sites.
fn build_schema_args(args: &Args) -> util::SchemaArgs {
    util::SchemaArgs {
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
        // match the Polars reader: a .tsv/.tab/.ssv extension beats an explicit --delimiter
        // (see `path_delim`), while Config would let the flag win
        flag_delimiter:       match (args.flag_delimiter, args.arg_input.as_deref()) {
            (Some(d), Some(input)) => Some(Delimiter(tsvssv_delim(input, d.as_byte()))),
            (d, _) => d,
        },
        arg_input:            args.arg_input.clone(),
        flag_memcheck:        false,
        flag_output:          None,
    }
}

/// Does this run's smart aggregation benefit from a `moarstats --advanced` stats cache?
///
/// Only pivot mode with `--agg smart` and exactly one `--values` column consults the value
/// column's distribution (multiple value columns use `first`; group-by mode uses `len`).
/// An explicit --delimiter opts out: moarstats has no --delimiter and resolves it from the
/// file extension, so it could parse the file differently than pivotp does.
fn wants_moarstats_enrichment(args: &Args) -> bool {
    !args.flag_no_moarstats
        && args.flag_delimiter.is_none()
        && args.arg_on_cols.is_some()
        && args
            .flag_agg
            .as_deref()
            .is_none_or(|a| a.eq_ignore_ascii_case("smart"))
        && args
            .flag_values
            .as_deref()
            .is_some_and(|v| !v.contains(','))
}

fn get_cached_stats(args: &Args) -> &'static (ByteRecord, Vec<StatsData>) {
    // `quiet` is latched on the first call because STATS_RECORDS is a
    // process-global OnceLock. In practice `Args` is constructed once per
    // `run()` so every call site sees the same flag, but if this ever gets
    // reused across multiple Args, only the first call's --quiet takes effect.
    let quiet = args.flag_quiet;
    STATS_RECORDS.get_or_init(|| {
        // Enrich BEFORE loading: a FrequencyForceStats load regenerates a stale cache as a lean
        // `stats --cardinality` one, which would throw away moarstats statistics.
        if wants_moarstats_enrichment(args)
            && let Some(input) = args.arg_input.as_deref()
        {
            match util::ensure_advanced_stats_cache(input) {
                Ok(true) if !quiet => {
                    eprintln!(
                        "Info: enriched the stats cache with moarstats --advanced for smart \
                         aggregation"
                    );
                },
                Ok(_) => {},
                Err(e) => {
                    if !quiet {
                        eprintln!(
                            "Warning: unable to enrich the stats cache with moarstats --advanced \
                             (smart-agg will use basic stats): {e}"
                        );
                    }
                },
            }
        }

        let schema_args = build_schema_args(args);
        match get_stats_records(&schema_args, StatsMode::FrequencyForceStats) {
            Ok(pair) => pair,
            Err(e) => {
                if !quiet {
                    eprintln!(
                        "Warning: unable to load stats for pivotp (smart-agg and --validate will \
                         be limited): {e}"
                    );
                }
                (ByteRecord::new(), Vec::new())
            },
        }
    })
}

#[derive(Deserialize)]
struct Args {
    arg_on_cols:         Option<String>,
    arg_input:           Option<String>,
    flag_index:          Option<String>,
    flag_values:         Option<String>,
    flag_agg:            Option<String>,
    flag_sort_columns:   bool,
    // Accepted for compatibility: output is always in first-appearance order.
    #[allow(dead_code)]
    flag_maintain_order: bool,
    flag_col_separator:  String,
    flag_validate:       bool,
    flag_max_columns:    u64,
    flag_try_parsedates: bool,
    flag_infer_len:      usize,
    flag_decimal_comma:  bool,
    flag_ignore_errors:  bool,
    flag_grand_total:    bool,
    flag_subtotal:       bool,
    flag_total_label:    String,
    flag_no_moarstats:   bool,
    flag_output:         Option<String>,
    flag_delimiter:      Option<Delimiter>,
    flag_quiet:          bool,
}

// IMPORTANT: This must be kept in sync with the default value
// of the --infer-len option in the USAGE string above.
const DEFAULT_INFER_LEN: usize = 10000;

// smart-agg thresholds
/// `stats` reports `cv` as a percentage.
const HIGH_CV_PERCENT: f64 = 100.0;
/// |G1| at or above this is "highly skewed" (conventional bound; viz's 0.5 cut only labels
/// moderate skew and deliberately does not gate robust-statistic choices).
const HIGH_MOMENT_SKEWNESS: f64 = 1.0;
/// Fallback when G1 is unavailable: Pearson's second skewness coefficient.
const HIGH_PEARSON_SKEWNESS: f64 = 1.0;
/// Bimodality coefficient cut (paired with negative excess kurtosis). Same as viz's
/// `BIMODALITY_COEFFICIENT_THRESHOLD`: a margin above Sarle's 0.555 (the uniform distribution's
/// value), so flat-ish columns are not flagged as two-peaked.
const BIMODALITY_COEFFICIENT_MIN: f64 = 0.60;

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
    let (csv_fields, csv_stats) = get_cached_stats(args);

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

/// Print the pivot's cardinality report. `--validate` only; purely informational.
///
/// `metadata.estimated_columns` is the PRODUCT of the per-column cardinalities, i.e. an upper
/// bound - correlated <on-cols> produce far fewer real columns. That is fine for a warning, but
/// it is why the hard cap lives in `enforce_max_columns` against the exact count instead.
fn report_pivot_cardinality(metadata: &PivotMetadata) {
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
}

/// Refuse a pivot whose output would be wider than `max_columns` (0 = no limit).
///
/// Enforced ALWAYS, not just under `--validate` (#4589): pivoting on a high-cardinality column
/// otherwise grows memory until the OS kills the process, which is strictly worse than the error
/// the command already knew how to emit.
///
/// `on_col_groups` is `unique_df.height()` - the EXACT number of distinct <on-cols> combinations,
/// not the stats-derived product. That matters twice over: the exact count cannot false-positive
/// on correlated <on-cols> (e.g. `state` x `state_abbrev`, where the product is quadratic but the
/// width is linear), and it cannot fail open when the stats cache is unavailable, which the
/// estimate silently does.
fn enforce_max_columns(
    on_col_groups: u64,
    value_col_count: u64,
    max_columns: u64,
) -> CliResult<()> {
    if max_columns == 0 {
        return Ok(());
    }

    let actual = on_col_groups.saturating_mul(value_col_count);
    if actual > max_columns {
        // Name the remedy flag - an escape hatch nobody can discover is the same as none.
        return fail_clierror!(
            "Pivot would create {} columns, exceeding the --max-columns limit of {}. Pivot on a \
             lower-cardinality column, raise --max-columns, or set --max-columns 0 to disable \
             this limit.",
            HumanCount(actual),
            HumanCount(max_columns)
        );
    }

    Ok(())
}

/// Build a single-row `DataFrame` containing a total row.
/// Numeric columns get their sum; index columns get the provided labels.
#[allow(clippy::missing_asserts_for_indexing)]
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
            // Non-numeric, non-index pivoted columns (e.g. a Date/Bool value
            // column produced by `first`/`last`) get a typed NULL so the
            // resulting total row retains the original schema and can be
            // vstack'd onto the pivot result.
            columns.push(Column::full_null(name.into(), 1, col_ref.dtype()));
        }
    }

    Ok(DataFrame::new(1, columns)?)
}

/// Insert subtotal rows after each group in the first index column.
/// Returns a new `DataFrame` with subtotal rows interleaved.
/// The data is sorted by all index columns to ensure contiguous groups and deterministic ordering.
///
/// Precondition: the first index column must already be cast to `String`.
/// The only caller (`run`) casts all index columns to `String` before invoking
/// this, so we rely on that here rather than handling arbitrary dtypes.
fn insert_subtotals(df: &DataFrame, index_cols: &[String], label: &str) -> CliResult<DataFrame> {
    // Sort by all index columns to ensure contiguous groups and
    // deterministic intra-group ordering, since pivot output order is not guaranteed.
    let df = df.sort(index_cols, SortMultipleOptions::default())?;

    let group_col = df.column(&index_cols[0])?;
    debug_assert!(
        group_col.dtype() == &DataType::String,
        "insert_subtotals: first index column must be String (caller should cast before calling), \
         got {:?}",
        group_col.dtype(),
    );
    let Ok(str_col) = group_col.str() else {
        return fail_clierror!(
            "insert_subtotals: first index column must be String (caller should cast before \
             calling)"
        );
    };
    let mut frames: Vec<DataFrame> = Vec::new();
    let mut group_start = 0_usize;

    for i in 1..=df.height() {
        // Detect group boundary: end of data or value change in first index col
        let current_val = if i < df.height() {
            str_col.get(i)
        } else {
            None
        };
        let start_val = str_col.get(group_start);
        let is_boundary = i == df.height() || current_val != start_val;

        if is_boundary {
            let group_len = i - group_start;
            let group_df = df.slice(group_start as i64, group_len);

            let group_value = start_val.unwrap_or_default();

            // Build subtotal row: first index col = group value, second = label
            let subtotal_row = build_total_row(&group_df, index_cols, group_value, label)?;

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

/// Aggregation for a value column the stats say holds a single distinct value.
///
/// Not `item`: one distinct value does not mean one ROW per pivot cell, and `item` errors on a
/// cell with two rows (even identical ones). Every row holds the same value, so `first` returns
/// exactly what `item` would - and it is also safe when the cardinality is a `HyperLogLog`
/// estimate (`stats --cardinality-method approx`).
fn single_value_agg(args: &Args, value_col: &str) -> AggKind {
    if !args.flag_quiet {
        eprintln!("Info: \"{value_col}\" contains only one value, using First");
    }
    AggKind::First
}

/// Suggest an appropriate aggregation function based on column statistics
#[allow(clippy::cast_precision_loss)]
fn suggest_agg_function(
    args: &Args,
    on_cols: &[String],
    index_cols: Option<&[String]>,
    value_cols: &[String],
) -> Option<AggKind> {
    // If multiple value columns, default to First
    if value_cols.len() > 1 {
        return Some(AggKind::First);
    }

    let quiet = args.flag_quiet;

    let (csv_fields, csv_stats) = get_cached_stats(args);

    // Analyze pivot column characteristics.
    // `ordered_pivot` means *all* pivot columns have stats and are sorted.
    // We combine per-column flags with AND so multi-column order is only
    // claimed when every column is ordered (previously the loop overwrote
    // the flag, making the result depend on column order).
    let mut high_cardinality_pivot = false;
    let mut ordered_pivot = !on_cols.is_empty();
    for on_col in on_cols {
        if let Some(pos) = csv_fields
            .iter()
            .position(|f| std::str::from_utf8(f).unwrap_or("") == on_col)
        {
            let stats = &csv_stats[pos];
            let uniqueness_ratio = stats.uniqueness_ratio.unwrap_or(0.0);

            if uniqueness_ratio > 0.5 {
                high_cardinality_pivot = true;
                if !quiet {
                    eprintln!("Info: Pivot column \"{on_col}\" has high cardinality");
                }
            }

            // Unordered (or missing sort_order) → the whole pivot is unordered
            ordered_pivot &= stats.sort_order.as_deref().is_some_and(|s| s != "Unsorted");
        } else {
            // Missing stats → we cannot prove this column is ordered
            ordered_pivot = false;
        }
    }

    // Analyze index column characteristics (same all-columns-ordered semantics)
    let mut high_cardinality_index = false;
    let mut ordered_index = index_cols.is_some_and(|c| !c.is_empty());
    if let Some(idx_cols) = index_cols {
        for idx_col in idx_cols {
            if let Some(pos) = csv_fields
                .iter()
                .position(|f| std::str::from_utf8(f).unwrap_or("") == idx_col)
            {
                let stats = &csv_stats[pos];
                let uniqueness_ratio = stats.uniqueness_ratio.unwrap_or(0.0);

                if uniqueness_ratio > 0.5 {
                    high_cardinality_index = true;
                    if !quiet {
                        eprintln!("Info: Index column \"{idx_col}\" has high uniqueness ratio");
                    }
                }

                ordered_index &= stats.sort_order.as_deref().is_some_and(|s| s != "Unsorted");
            } else {
                ordered_index = false;
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
                AggKind::Len
            },
            "Integer" | "Float" => {
                if stats.cardinality == 1 {
                    single_value_agg(args, value_col)
                } else if stats.sparsity.unwrap_or(0.0) > 0.5 {
                    if !quiet {
                        eprintln!("Info: \"{value_col}\" contains >50% NULL values, using Len");
                    }
                    AggKind::Len
                } else if let Some(mc) = stats.mode_count
                    && mc > 10
                    && mc < stats.cardinality
                {
                    // Complex multimodal — more than 10 tied modes means
                    // central tendency is especially misleading. When EVERY distinct value
                    // ties (mode_count == cardinality) the data is flat, not multimodal.
                    if !quiet {
                        eprintln!(
                            "Info: Complex multimodal distribution ({mc} modes > 10), using Len"
                        );
                    }
                    AggKind::Len
                } else if let Some(bc) = stats.bimodality_coefficient {
                    // Bimodal distribution — central tendency is misleading. Require negative
                    // excess kurtosis too: heavily skewed UNIMODAL data (amounts, counts,
                    // durations) also inflates BC past the cut, but is leptokurtic, whereas a
                    // genuinely two-peaked distribution is flat-topped.
                    if bc >= BIMODALITY_COEFFICIENT_MIN
                        && let Some(kurt) = stats.kurtosis
                        && kurt < 0.0
                    {
                        if !quiet {
                            eprintln!(
                                "Info: Bimodal distribution detected (bimodality coefficient \
                                 {bc:.3} >= 0.60, excess kurtosis {kurt:.3} < 0), using Len"
                            );
                        }
                        AggKind::Len
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
                    single_value_agg(args, value_col)
                } else if stats.sparsity.unwrap_or(0.0) > 0.5 {
                    if !quiet {
                        eprintln!(
                            "Info: \"{value_col}\" is a sparse {} column (>50% NULL), using Len",
                            stats.r#type
                        );
                    }
                    AggKind::Len
                } else if high_cardinality_pivot || high_cardinality_index {
                    if ordered_pivot && ordered_index {
                        if !quiet {
                            eprintln!(
                                "Info: Ordered temporal data with high cardinality, using Last"
                            );
                        }
                        AggKind::Last
                    } else {
                        if !quiet {
                            eprintln!(
                                "Info: High cardinality detected, using First for {} column",
                                stats.r#type
                            );
                        }
                        AggKind::First
                    }
                } else if let Some(sort_order) = &stats.sort_order
                    && sort_order.starts_with("Ascending")
                {
                    // Value column itself is sorted ascending — most recent entry
                    // is typically most useful for temporal data
                    if !quiet {
                        eprintln!(
                            "Info: Ascending {} values detected, using Last for most recent",
                            stats.r#type
                        );
                    }
                    AggKind::Last
                } else {
                    if !quiet {
                        eprintln!("Info: Using Len for {} column", stats.r#type);
                    }
                    AggKind::Len
                }
            },
            _ => {
                let uniqueness_ratio = stats.uniqueness_ratio.unwrap_or(0.0);
                if stats.cardinality == 1 {
                    single_value_agg(args, value_col)
                } else if (uniqueness_ratio - 1.0).abs() < 0.0001 {
                    if !quiet {
                        eprintln!("Info: \"{value_col}\" contains all unique values, using First");
                    }
                    AggKind::First
                } else if stats.sparsity > Some(0.5) {
                    if !quiet {
                        eprintln!("Info: Sparse data detected, using Len");
                    }
                    AggKind::Len
                } else if let Some(ne) = stats.normalized_entropy {
                    if ne > 0.9 {
                        if !quiet {
                            eprintln!(
                                "Info: Near-uniform string distribution (normalized entropy \
                                 {ne:.3} > 0.9), using Len"
                            );
                        }
                        AggKind::Len
                    } else if high_cardinality_pivot || high_cardinality_index {
                        if !quiet {
                            eprintln!("Info: High cardinality detected, using First");
                        }
                        AggKind::First
                    } else {
                        // Low entropy means few dominant values — First is more
                        // informative than a count
                        if !quiet {
                            eprintln!("Info: Low entropy string column, using First");
                        }
                        AggKind::First
                    }
                } else if high_cardinality_pivot || high_cardinality_index {
                    if !quiet {
                        eprintln!("Info: High cardinality detected, using First");
                    }
                    AggKind::First
                } else {
                    if !quiet {
                        eprintln!("Info: Using Len for String column");
                    }
                    AggKind::Len
                }
            },
        };

        Some(suggested_agg)
    } else {
        None
    }
}

/// Helper for numeric aggregation decisions after the bimodality check.
/// Contains the remaining moarstats-aware checks (outliers, kurtosis, Gini,
/// MAD/stddev divergence, mean/median divergence, quartile dispersion)
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
) -> AggKind {
    // moarstats: outlier impact ratio — mean shifted >20% by outliers. The ratio is signed
    // (negative when the outliers sit on the low side), so compare its magnitude.
    if let Some(oir) = stats.outlier_impact_ratio
        && oir.abs() > 0.2
    {
        if !quiet {
            eprintln!(
                "Info: High outlier impact (ratio {oir:.3}, |value| > 0.2) shifts mean, using \
                 Median"
            );
        }
        return AggKind::Median;
    }

    // moarstats: >15% outlier contamination
    if let Some(op) = stats.outliers_percentage
        && op > 15.0
    {
        if !quiet {
            eprintln!("Info: High outlier percentage ({op:.1}% > 15%), using Median");
        }
        return AggKind::Median;
    }

    // moarstats: leptokurtic / heavy-tailed distribution
    if let Some(k) = stats.kurtosis
        && k > 3.0
    {
        if !quiet {
            eprintln!("Info: Heavy-tailed distribution (kurtosis {k:.3} > 3.0), using Median");
        }
        return AggKind::Median;
    }

    // moarstats: high inequality — few values dominate
    if let Some(gc) = stats.gini_coefficient
        && gc > 0.6
    {
        if !quiet {
            eprintln!("Info: High inequality (Gini coefficient {gc:.3} > 0.6), using Median");
        }
        return AggKind::Median;
    }

    // moarstats: robust vs non-robust spread divergence — outliers inflate stddev
    if let Some(msr) = stats.mad_stddev_ratio
        && msr < 0.5
    {
        if !quiet {
            eprintln!(
                "Info: Robust/non-robust spread divergence (MAD/stddev ratio {msr:.3} < 0.5), \
                 using Median"
            );
        }
        return AggKind::Median;
    }

    // median/mean and (q3-q1)/(q3+q1) are scale ratios: with negative values or a mean near zero
    // their denominators cross zero and the ratios blow up or flip sign, so only trust them on
    // positive-valued data.
    let positive_scale = stats.n_negative == Some(0)
        && stats
            .mean
            .as_deref()
            .and_then(|m| m.parse::<f64>().ok())
            .is_some_and(|m| m > 0.0);

    // moarstats: direct mean/median divergence
    if positive_scale
        && let Some(mmr) = stats.median_mean_ratio
        && !(0.7..=1.3).contains(&mmr)
    {
        if !quiet {
            eprintln!(
                "Info: Mean/median divergence (ratio {mmr:.3}, outside 0.7\u{2013}1.3), using \
                 Median"
            );
        }
        return AggKind::Median;
    }

    // moarstats: robust quartile-based variability
    if positive_scale
        && let Some(qcd) = stats.quartile_coefficient_dispersion
        && qcd > 0.5
    {
        if !quiet {
            eprintln!(
                "Info: High robust variability (quartile coefficient of dispersion {qcd:.3} > \
                 0.5), using Median"
            );
        }
        return AggKind::Median;
    }

    // Mixed-sign cancellation guard — Sum cancels out with substantial
    // negative and positive values, Mean is more informative.
    // Uses integer arithmetic (n * 5 > total ≡ n/total > 20%) to avoid
    // f64 casts.
    let is_mixed_sign = if let (Some(n_neg), Some(n_pos)) = (stats.n_negative, stats.n_positive) {
        let total = n_neg + stats.n_zero.unwrap_or(0) + n_pos;
        total > 0 && n_neg.saturating_mul(5) > total && n_pos.saturating_mul(5) > total
    } else {
        false
    };

    // High CV. `stats` reports cv as a PERCENTAGE ((stddev / mean) * 100), so the cut is 100,
    // i.e. stddev larger than the mean.
    // CV is a scale ratio too: substantial negatives drag the mean toward zero and inflate it, so
    // mixed-sign data skips it and reaches the mixed-sign guard below instead.
    if !is_mixed_sign && stats.cv > Some(HIGH_CV_PERCENT) {
        if !quiet {
            eprintln!(
                "Info: High variability in values (CV > 100%), using Median for more robust \
                 central tendency"
            );
        }
        return AggKind::Median;
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
            return AggKind::Len;
        }

        // Mixed-sign guard applies here too — Sum would cancel out
        if is_mixed_sign {
            if !quiet {
                log_mixed_sign(stats);
            }
            return AggKind::Mean;
        }

        if ordered_pivot && ordered_index {
            if !quiet {
                eprintln!("Info: Ordered high cardinality columns detected, using Mean");
            }
            return AggKind::Mean;
        }
        if !quiet {
            eprintln!("Info: High cardinality in pivot and index columns, using Sum");
        }
        return AggKind::Sum;
    }

    // Skewness. `stats.skewness` is Bowley's quartile skewness, bounded to [-1, 1], so it cannot
    // express "highly skewed" on the usual moment scale and is not used here. Prefer moarstats
    // --advanced moment skewness (G1); fall back to Pearson's second coefficient only when G1 is
    // unavailable.
    if let Some(g1) = stats.moment_skewness {
        if g1.abs() >= HIGH_MOMENT_SKEWNESS {
            if !quiet {
                eprintln!(
                    "Info: Highly skewed numeric data (moment skewness {g1:.3}, |value| >= 1.0), \
                     using Median"
                );
            }
            return AggKind::Median;
        }
    } else if let Some(ps) = stats.pearson_skewness
        && ps.abs() > HIGH_PEARSON_SKEWNESS
    {
        if !quiet {
            eprintln!(
                "Info: Pearson skewness ({ps:.3}, |value| > 1.0) indicates asymmetry, using Median"
            );
        }
        return AggKind::Median;
    }

    // Mixed-sign guard for the default path
    if is_mixed_sign {
        if !quiet {
            log_mixed_sign(stats);
        }
        return AggKind::Mean;
    }

    // Default for numeric
    if !quiet {
        eprintln!("Info: Using Sum for \"{value_col}\"");
    }
    AggKind::Sum
}

/// Log message for the mixed-sign cancellation guard.
fn log_mixed_sign(stats: &StatsData) {
    let (n_neg, n_pos) = (stats.n_negative.unwrap_or(0), stats.n_positive.unwrap_or(0));
    // saturating_add for the total too, to match the end-to-end overflow-safe
    // guarantee described below.
    let total = n_neg
        .saturating_add(stats.n_zero.unwrap_or(0))
        .saturating_add(n_pos);
    // Round to nearest percent via (x*100 + total/2) / total.
    // Use saturating_mul/saturating_add end-to-end so pathological row counts
    // (n > u64::MAX / 100 ≈ 1.8e17) saturate cleanly instead of panicking.
    // checked_div handles the (defensively-impossible) total == 0 case —
    // callers only invoke us after is_mixed_sign, which requires total > 0.
    let neg_pct = n_neg
        .saturating_mul(100)
        .saturating_add(total / 2)
        .checked_div(total)
        .unwrap_or(0);
    let pos_pct = n_pos
        .saturating_mul(100)
        .saturating_add(total / 2)
        .checked_div(total)
        .unwrap_or(0);
    eprintln!(
        "Info: Mixed-sign data ({neg_pct}% negative, {pos_pct}% positive), using Mean to avoid \
         Sum cancellation"
    );
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Resolve positional args.
    // With two usage patterns ("... <on-cols> <input>" and "... <input>"),
    // docopt assigns one positional to <input> and two to <on-cols> + <input>.
    let (on_cols, input_path_str) = match (&args.arg_on_cols, &args.arg_input) {
        (Some(on_cols_val), Some(input_val)) => {
            // Both provided: pivot mode
            let oc: Vec<String> = on_cols_val
                .split(',')
                .map(std::string::ToString::to_string)
                .collect();
            (Some(oc), input_val.clone())
        },
        (None, Some(input_val)) => {
            // Matched second pattern: group-by mode
            (None, input_val.clone())
        },
        (Some(on_cols_val), None) => {
            // Fallback: single positional assigned to on-cols, treat as input
            (None, on_cols_val.clone())
        },
        (None, None) => {
            return fail_incorrectusage_clierror!("An input CSV file is required.");
        },
    };

    let is_groupby_mode = on_cols.is_none();

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

    // Validate mode-specific constraints
    if is_groupby_mode {
        if index_cols.is_none() {
            return fail_incorrectusage_clierror!(
                "--index <cols> is required in group-by mode (when <on-cols> is omitted)."
            );
        }
        if args.flag_subtotal {
            return fail_incorrectusage_clierror!("--subtotal is not supported in group-by mode.");
        }
        if args.flag_validate {
            return fail_incorrectusage_clierror!("--validate is not supported in group-by mode.");
        }
        if args.flag_sort_columns {
            return fail_incorrectusage_clierror!(
                "--sort-columns is not supported in group-by mode."
            );
        }
        if let Some(ref agg) = args.flag_agg
            && agg.eq_ignore_ascii_case("item")
        {
            return fail_incorrectusage_clierror!("--agg item is not supported in group-by mode.");
        }
    } else if index_cols.is_none() && value_cols.is_none() {
        return fail_incorrectusage_clierror!(
            "Either --index <cols> or --values <cols> must be specified."
        );
    }

    // Parse the aggregation function name (default: "smart", which resolves to "len" in group-by
    // mode)
    let agg_name = if let Some(ref agg) = args.flag_agg {
        agg.to_lowercase()
    } else {
        "smart".to_string()
    };

    // Reject --agg none in group-by mode AFTER lowercasing so "None"/"NONE" are also caught
    if is_groupby_mode && agg_name == "none" {
        return fail_incorrectusage_clierror!("--agg \"none\" is not supported in group-by mode.");
    }

    // Reject malformed quantile probabilities with a precise error so users see *why*
    // their probability is bad rather than the generic "Invalid pivot aggregation function".
    // Works in both pivot and group-by mode; no mode-conditional check needed.
    if (agg_name.starts_with("quantile@") || agg_name.starts_with("q@"))
        && parse_quantile_agg(&agg_name).is_none()
    {
        return fail_incorrectusage_clierror!(
            "Invalid quantile probability in --agg {agg_name}: must be a float in [0, 1] (e.g. \
             quantile@0.95)"
        );
    }

    // Resolve the aggregation. It is applied to each value column: per group in group-by mode,
    // and per (index, pivot column) group in pivot mode. "none" means first in pivot mode (it
    // was rejected above in group-by mode).
    let agg_kind = if agg_name == "none" {
        AggKind::First
    } else if agg_name == "smart" {
        if is_groupby_mode {
            // In group-by mode, smart defaults to len (count)
            AggKind::Len
        } else if let Some(value_cols) = &value_cols {
            // Try to suggest an appropriate aggregation function
            let on_cols_ref = on_cols.as_ref().unwrap();
            // fallback to first, which always works
            suggest_agg_function(&args, on_cols_ref, index_cols.as_deref(), value_cols)
                .unwrap_or(AggKind::First)
        } else {
            // Default to Len if no value columns specified
            AggKind::Len
        }
    } else if let Some(kind) = AggKind::from_name(&agg_name) {
        kind
    } else {
        let agg_mode = if is_groupby_mode { "group-by" } else { "pivot" };
        return fail_incorrectusage_clierror!(
            "Invalid {agg_mode} aggregation function: {agg_name}"
        );
    };

    // Resolve delimiters the same way sqlp/joinp do: --delimiter, else QSV_DEFAULT_DELIMITER,
    // else ','; then a .tsv/.tab/.ssv extension on the file decides. The stats cache that
    // smart-agg reads is built through Config, which applies the same rules, so the Polars
    // reader and the stats see the same columns.
    let fallback_delim = if let Some(delimiter) = args.flag_delimiter {
        delimiter.as_byte()
    } else if let Ok(delim) = std::env::var("QSV_DEFAULT_DELIMITER") {
        Delimiter::decode_delimiter(&delim)?.as_byte()
    } else {
        b','
    };
    let delim = path_delim(&input_path_str, args.flag_delimiter, fallback_delim);
    let output_delim = args.flag_output.as_ref().map_or(fallback_delim, |out| {
        path_delim(out, args.flag_delimiter, fallback_delim)
    });

    if args.flag_decimal_comma && delim == b',' {
        return fail_incorrectusage_clierror!(
            "You need to specify an alternate --delimiter when using --decimal-comma."
        );
    }

    // A snappy-compressed input is decompressed to a temp file first, as joinp does. The
    // delimiter above was already resolved from the original name (`.tsv.sz` -> tab).
    let snappy_tmpdir;
    let reader_path = if Path::new(&input_path_str)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("sz"))
    {
        snappy_tmpdir = tempfile::tempdir()?;
        util::decompress_snappy_file(&PathBuf::from(&input_path_str), &snappy_tmpdir)?
    } else {
        input_path_str.clone()
    };

    // Create CSV reader config. Like sqlp/joinp: honor QSV_COMMENT_CHAR (Config, which builds the
    // stats cache smart-agg reads, does too) and read empty fields as nulls.
    let comment_prefix = std::env::var("QSV_COMMENT_CHAR")
        .ok()
        .map(PlSmallStr::from_string);
    let mut csv_reader = LazyCsvReader::new(PlRefPath::new(&reader_path))
        .with_has_header(true)
        .with_missing_is_null(true)
        .with_comment_prefix(comment_prefix)
        .with_try_parse_dates(args.flag_try_parsedates)
        .with_decimal_comma(args.flag_decimal_comma)
        .with_separator(delim)
        .with_ignore_errors(args.flag_ignore_errors);

    // Use a pschema.json (from `schema --polars`) that is at least as new as the input, unless
    // the user asked for a custom inference length. Looked up beside the ORIGINAL input - for a
    // .sz input that is `<input>.sz.pschema.json`, not the decompressed temp file's.
    let pschema = if args.flag_infer_len == DEFAULT_INFER_LEN {
        util::load_valid_pschema(Path::new(&input_path_str))?
    } else {
        None
    };
    if let Some(schema) = pschema {
        csv_reader = csv_reader.with_schema(Some(schema));
    } else {
        // Otherwise we infer the schema using inference length (default or user-specified).
        // --infer-len 0 means "scan the entire file" (None); Polars reads Some(0) as
        // "infer nothing", which types every column as String.
        csv_reader = csv_reader.with_infer_schema_length(if args.flag_infer_len == 0 {
            None
        } else {
            Some(args.flag_infer_len)
        });
    }

    // Read the CSV into a LazyFrame
    let mut lf = csv_reader.finish()?;

    // Get all column names from the schema to compute index/values if not specified
    let schema = lf.collect_schema()?;
    let all_cols: Vec<String> = schema
        .iter_names()
        .map(polars::prelude::PlSmallStr::to_string)
        .collect();

    // Compute actual index and value columns if not specified
    let actual_index_cols: Vec<String> = if let Some(idx_cols) = index_cols {
        idx_cols
    } else if let Some(ref oc) = on_cols {
        // If no index specified, use all columns except on and values
        let on_set: HashSet<&str> = oc.iter().map(std::string::String::as_str).collect();
        let value_set: HashSet<&str> = value_cols
            .as_ref()
            .map(|cols| cols.iter().map(std::string::String::as_str).collect())
            .unwrap_or_default();

        all_cols
            .iter()
            .filter(|c| !on_set.contains(c.as_str()) && !value_set.contains(c.as_str()))
            .cloned()
            .collect()
    } else {
        // group-by mode with no index — already validated above
        unreachable!()
    };

    // Determine the actual value columns
    // In group-by mode without --values, we don't need value columns (just count rows)
    let actual_value_cols: Option<Vec<String>> = if let Some(cols) = value_cols {
        Some(cols)
    } else if let Some(ref oc) = on_cols {
        // Pivot mode: default to all columns except on and index
        let on_set: HashSet<&str> = oc.iter().map(std::string::String::as_str).collect();
        let index_set: HashSet<&str> = actual_index_cols
            .iter()
            .map(std::string::String::as_str)
            .collect();

        Some(
            all_cols
                .iter()
                .filter(|c| !on_set.contains(c.as_str()) && !index_set.contains(c.as_str()))
                .cloned()
                .collect(),
        )
    } else {
        // Group-by mode without --values: no value columns, just count rows
        None
    };

    // Branch into group-by or pivot path
    let mut pivot_result = if is_groupby_mode {
        // === GROUP-BY MODE ===
        // Aggregate each value column ("none" and "item" were rejected during validation).
        // AggKind::Len counts all rows of the group, nulls included.
        let agg_exprs: Vec<Expr> = if let Some(ref val_cols) = actual_value_cols {
            val_cols
                .iter()
                .map(|vc| {
                    agg_kind
                        .on(col(PlSmallStr::from_str(vc)))
                        .alias(PlSmallStr::from_str(vc))
                })
                .collect()
        } else {
            // No value columns: just count rows per group
            vec![len().alias(PlSmallStr::from_str("count"))]
        };

        let group_by_exprs: Vec<Expr> = actual_index_cols.iter().map(col).collect();

        // group_by_stable keeps groups in first-appearance order in a single pass. (This used to
        // be a group_by plus a second scan for each group's first row index, joined and sorted.)
        lf.group_by_stable(group_by_exprs)
            .agg(agg_exprs)
            .collect()?
    } else {
        // === PIVOT MODE ===
        // Safety: on_cols is always Some in pivot mode (is_groupby_mode is false iff on_cols is
        // Some). actual_value_cols is always Some because the pivot-mode fallback
        // (lines ~940-950) computes remaining columns when --values is omitted.
        let on_cols = on_cols.unwrap();
        let actual_value_cols = actual_value_cols.unwrap();

        if args.flag_validate
            && let Some(metadata) =
                calculate_pivot_metadata(&args, &on_cols, Some(&actual_value_cols))
        {
            // Validation uses cached stats records, not the DataFrame itself,
            // so we don't need to collect the LazyFrame here.
            report_pivot_cardinality(&metadata);
        }

        // Compute unique values for the pivot columns to create on_columns DataFrame
        // This is required by the new LazyFrame pivot API
        // We need to maintain discovery order, so we add row numbers and sort by them
        let on_columns = {
            let unique_df = lf
                .clone()
                .select(cols_to_exprs(&on_cols))
                .unique_stable(
                    Some(cols(on_cols.iter().map(std::string::String::as_str))),
                    UniqueKeepStrategy::First,
                )
                .collect()?;

            // Hard cap on output width (#4589). This is the earliest point where the EXACT
            // column count is known, and it is still before the pivot itself - which is where
            // the unbounded allocation happened. The `unique` above is bounded by the number of
            // distinct <on-cols> combinations, so it cannot blow up the way the pivot can.
            enforce_max_columns(
                unique_df.height() as u64,
                actual_value_cols.len() as u64,
                args.flag_max_columns,
            )?;

            Arc::new(unique_df)
        };

        // Convert separator to PlSmallStr
        let separator = PlSmallStr::from_str(&args.flag_col_separator);

        let on_selector = cols(on_cols.iter().map(std::string::String::as_str));
        let index_selector = cols(actual_index_cols.iter().map(std::string::String::as_str));
        let values_selector = cols(actual_value_cols.iter().map(std::string::String::as_str));
        let column_naming = PivotColumnNaming::default();

        let mut pivot_result = if agg_kind.pre_aggregates() {
            // Aggregate each (index, pivot column) group FIRST, then pivot the grouped frame,
            // where every cell holds at most one row. Pivoting raw rows costs Polars one filtered
            // aggregation per output column, ~1.4 MB each regardless of the input size
            // (pola-rs/polars#30173): a 50 x 2000 pivot of a 65 MB CSV peaked at 3.9 GB. With
            // `item` and EAGER flags Polars takes a direct path instead (here: 0.4 GB, 13x
            // faster). group_by_stable keeps first-appearance order, so row order is unchanged.
            lf.group_by_stable(
                actual_index_cols
                    .iter()
                    .chain(on_cols.iter())
                    .map(col)
                    .collect::<Vec<_>>(),
            )
            .agg(
                actual_value_cols
                    .iter()
                    .map(|vc| {
                        agg_kind
                            .on(col(PlSmallStr::from_str(vc)))
                            .alias(PlSmallStr::from_str(vc))
                    })
                    .collect::<Vec<_>>(),
            )
            .collect()?
            .lazy()
            .pivot(
                on_selector,
                on_columns,
                index_selector,
                values_selector,
                // at most one row per cell (null when empty)
                Expr::Element.item(true),
                // rows in first-appearance order of their index values
                true,
                separator,
                column_naming,
            )
            .with_optimizations(OptFlags::EAGER)
            .collect()?
        } else {
            lf.pivot(
                on_selector,
                on_columns,
                index_selector,
                values_selector,
                agg_kind.on(Expr::Element),
                true,
                separator,
                column_naming,
            )
            .collect()?
        };

        // A cell with no rows: pivoting the raw rows gives 0 for sum and len, and null for every
        // other aggregation. Pivoting the pre-aggregated groups gives null throughout (the group
        // does not exist), so restore the 0s. A sum or len that exists is never null.
        if agg_kind.pre_aggregates() && agg_kind.empty_cell_is_zero() {
            let index_set: HashSet<&str> = actual_index_cols
                .iter()
                .map(std::string::String::as_str)
                .collect();
            let fills: Vec<Expr> = pivot_result
                .schema()
                .iter()
                .filter(|(name, _)| !index_set.contains(name.as_str()))
                .map(|(name, dtype)| col(name.clone()).fill_null(lit(0).cast(dtype.clone())))
                .collect();
            pivot_result = pivot_result.lazy().with_columns(fills).collect()?;
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

        pivot_result
    };

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
            Box::new(File::create(path)?) as Box<dyn Write>
        },
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };
    let datetime_fmt: PlSmallStr = PlSmallStr::from_str("%Y-%m-%d %H:%M:%S");
    // same env knobs as sqlp/joinp
    let float_precision = std::env::var("QSV_POLARS_FLOAT_PRECISION")
        .ok()
        .and_then(|s| s.parse().ok());
    CsvWriter::new(&mut writer)
        .include_header(true)
        .include_bom(util::get_envvar_flag("QSV_OUTPUT_BOM"))
        .with_float_precision(float_precision)
        .with_datetime_format(Some(datetime_fmt))
        .with_separator(output_delim)
        .finish(&mut pivot_result)?;
    drop(writer);

    // an --output ending in .sz is snappy-compressed in place, as in sqlp/joinp
    compress_output_if_needed(args.flag_output.clone())?;

    // Print shape to stderr
    if !args.flag_quiet {
        eprintln!("{:?}", pivot_result.shape());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The aggregation `suggest_numeric_after_bimodality` picks, as its Debug rendering.
    fn pick(stats: &StatsData) -> String {
        format!(
            "{:?}",
            suggest_numeric_after_bimodality(stats, true, "v", false, false, false, false)
        )
    }

    // median/mean and QCD are ratios of a positive scale; with negatives in the column they say
    // nothing about skew, so they must not pick Median on their own.
    #[test]
    fn scale_ratios_ignored_on_signed_data() {
        let median = pick(&StatsData {
            median_mean_ratio: Some(0.3),
            n_negative: Some(0),
            n_zero: Some(0),
            n_positive: Some(100),
            mean: Some("5".to_string()),
            ..Default::default()
        });
        let signed = |field: fn(&mut StatsData)| {
            let mut s = StatsData {
                n_negative: Some(10),
                n_zero: Some(0),
                n_positive: Some(90),
                mean: Some("5".to_string()),
                ..Default::default()
            };
            field(&mut s);
            pick(&s)
        };
        let sum = pick(&StatsData::default());
        assert_eq!(
            median,
            pick(&StatsData {
                quartile_coefficient_dispersion: Some(0.9),
                n_negative: Some(0),
                mean: Some("5".to_string()),
                ..Default::default()
            })
        );
        assert_ne!(median, sum);
        assert_eq!(signed(|s| s.median_mean_ratio = Some(0.3)), sum);
        assert_eq!(
            signed(|s| s.quartile_coefficient_dispersion = Some(0.9)),
            sum
        );
    }
}
