pub(crate) static USAGE: &str = r#"
Generates a synthetic CSV that is statistically faithful to a source CSV.

`synthesize` analyzes <input> with `stats` and `frequency`, then emits N rows of
fake data that reproduce the source's per-column attributes:

  * Categorical / low-cardinality columns are reproduced by frequency-weighted
    sampling of their *real* value set — cardinality, weights and repetition
    structure are preserved, minus values rarer than the --min-count.
  * Numeric and date/datetime columns are reproduced with quartile buckets, so
    the shape of the distribution (not just its [min,max] range) is preserved.
    The range ends are trimmed by the --min-count (see below).
  * Null ratios are reproduced per column.

When a Data Dictionary is supplied (via --dictionary, or generated on the fly
with --infer-content-type), each column's semantic Content Type picks a
realistic faker (names, emails, addresses, UUIDs, etc.) for columns that are
NOT fully enumerated by `frequency`. For bounded-cardinality faker columns
(cardinality < requested rows and below an internal cap of 100,000), a fixed
pool of distinct fake values is pre-generated and sampled from, so the column's
cardinality is preserved. For very high cardinality columns above this cap, a
fresh fake value is generated per row instead — distinct count is approximate
in that case.

When `stats` provides string-length statistics (min_length / max_length /
avg_length / stddev_length) AND the column is routed to an unstructured text
generator (lorem_*, free_text, or the no-faker fallback), synthesized values
are truncated so their character lengths follow Normal(avg_length,
stddev_length) clamped to [min_length, max_length]. This applies to unstructured
pooled values as well — a low-cardinality free-text column still gets its
generated pool entries truncated. Structured semantic fakers (email, name,
uuid, phone, address parts, etc.) ignore these stats — truncating them would
corrupt their format, so their pools are reproduced verbatim. Frequency-
enumerated values are always reproduced verbatim and are never truncated.

When the Data Dictionary declares `relationships`, the named columns are
generated *jointly* so inter-column structure survives into each output row:

  * joint      — categorical / functional-dependency groups (e.g.
                 city/state/zip). Whole value-tuples are sampled from the
                 source by frequency, so only real co-occurring combinations
                 are emitted.
  * ordered    — columns that must keep a monotonic order within a row (e.g.
                 created_date <= closed_date). The anchor column is generated
                 from its own distribution; each later column is the anchor
                 plus a non-negative gap drawn from the gap distribution
                 learned from the source.
  * correlated — numeric columns whose correlation should be preserved. A
                 Gaussian copula couples the columns while leaving each
                 column's own distribution unchanged.

Relationships are read from the dictionary's `relationships` array — inferred
by `describegpt` or hand-authored. Columns not named by any relationship are
still generated independently. Pass --no-relationships to disable relationship
modeling entirely.

Privacy note: synthetic output is not anonymized. It can contain real values
and real value combinations copied from <input>. By default, values and
`joint` tuples seen fewer than 5 times in <input> are dropped from the real
values that columns and joint groups sample from (see the --min-count
option). The count is of rows, not people, so in data with many rows per
person one person's value can still pass it. That limits copying but does
not stop it:

  * A column fully enumerated by `frequency` (cardinality within
    the --freq-limit) is sampled from its real values, so any value at or
    above the --min-count can be emitted verbatim. Setting the --freq-limit
    to 0 removes that cap, so any column that is not all-unique can be
    reproduced from its real values. Setting the --min-count to 1 lets
    values that occur only once through as well.
  * With a Data Dictionary, enumerated name, email, address and other
    structured-faker columns still emit their real values unless you set
    the --consistent-fakes option. That option only covers columns with up
    to 100,000 distinct values; larger enumerated columns stay real.
  * A `joint` relationship emits whole real value-tuples (e.g. a
    city/zip/age combination) seen at least --min-count times.
  * Numeric and date ranges are trimmed so that at least --min-count rows
    lie at or beyond each end (the k-th smallest and k-th largest values),
    so a lone outlier no longer sets the range. The ends are themselves real
    values, and values inside the range are drawn uniformly per quartile, so
    integers and dates can still land exactly on real values. With
    the --min-count at 1 the range is the real min and max, and an outlier
    can reappear at or near its real value.

Review the output before sharing it, or training on it, as de-identified data.

With --seed, output is fully reproducible.

Examples:

  # Pure statistical synthesis — no dictionary needed
  $ qsv synthesize data.csv -n 1000 --seed 42 > synthetic.csv

  # First, generate the Data Dictionary with describegpt
  $ qsv describegpt data.csv --dictionary --infer-content-type --format JSON -o dict.json
  # Then layer in semantic fakers from the dictionary
  $ qsv synthesize data.csv --dictionary dict.json -n 1000 > synthetic.csv

  # Let synthesize build the dictionary itself (needs an LLM API key)
  $ qsv synthesize data.csv --infer-content-type -n 1000 > synthetic.csv

  # Preserve inter-column relationships declared in the dictionary
  # (e.g. city/state/zip, created_date <= closed_date)
  $ qsv synthesize data.csv --dictionary dict.json -n 1000 > synthetic.csv

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_synthesize.rs.
See also https://github.com/dathere/qsv/wiki/AI-and-Documentation#synthesize

Usage:
    qsv synthesize [options] <input>
    qsv synthesize --help

synthesize options:
    --dictionary <file>    Data Dictionary JSON file produced by
                           `describegpt --dictionary --infer-content-type --format JSON`.
                           Layers semantic Content Types onto generation. If
                           omitted, generation is purely type/frequency-based.
    --infer-content-type   Generate the Data Dictionary on the fly by invoking
                           `describegpt --dictionary --infer-content-type` on
                           <input>. Requires an LLM API key (QSV_LLM_APIKEY).
                           Ignored if --dictionary is given.
    -n, --rows <n>         Number of synthetic rows to generate. [default: 100]
    --seed <n>             RNG seed for fully reproducible output.
    --locale <loc>         Locale for faker-backed columns. Case-insensitive.
                           Supported: en, fr_fr, de_de, it_it, pt_br, pt_pt,
                           ja_jp, zh_cn, zh_tw, ar_sa, cy_gb, fa_ir, nl_nl,
                           tr_tr. Sparse locales (those without per-category
                           data in fake-rs) silently fall back to en data for
                           the missing categories — e.g. lorem text under a
                           non-en locale is still English, since only zh_cn
                           has localized lorem data. [default: en]
    --freq-limit <n>       Frequency pool depth passed to the internal `frequency`
                           run as --limit. A column is reproduced via exact
                           frequency-weighted sampling only when its cardinality
                           is fully captured within this limit; higher values
                           reproduce more columns verbatim. 0 means unlimited.
                           [default: 100]
    --min-count <k>        Privacy threshold. A real value of an enumerated
                           column, or a real `joint` value-tuple, that occurs
                           fewer than <k> times in <input> is dropped from the
                           values it samples; its share is redistributed over
                           the kept values. An enumerated column whose values
                           are all below <k> falls back to its faker or
                           type-based generator (numeric and date columns to
                           their trimmed range), and a joint group to
                           independent columns, or aborts under
                           the --strict-relationships option. A warning is printed
                           when either happens. 1 disables suppression.
                           Counts are of rows, not people: where one person
                           has many rows (transactions, visits, logs), one
                           person's value can pass <k> alone. Raise <k>, or
                           dedupe to one row per person first.
                           Numeric and date ranges, and the gaps of `ordered`
                           relationships, are also trimmed to their k-th
                           smallest and k-th largest values (one extra pass
                           over <input>). A numeric or date column with fewer
                           than 2k-1 non-null values is left empty unless its
                           real values are sampled.
                           [default: 5]
    --report <file>        Write a JSON report of what the run reproduced
                           from <input>: source and synthetic row counts,
                           how many synthetic rows exactly match a source
                           row (numbers and dates compared by value, other
                           columns as text), and the values and joint tuples
                           dropped by the --min-count option. Takes one
                           extra pass over <input> and holds a hash per
                           distinct source row in memory. A row with a
                           float, free-text or faker column rarely matches
                           exactly, so a low match rate does not by itself
                           show that the output is private. A joint group
                           that falls back to independent columns is listed,
                           and so is each of its columns that also drops
                           values, so suppressed rows can be counted more
                           than once. Must not be the input, dictionary or
                           output file.
    --stats-options <arg>  Extra options appended to the internal `stats` run.
                           Note: cardinality, quartiles and date inference are
                           always enabled — do not re-specify them here.
                           Column selection (--select/-s) and --weight are
                           rejected; run `qsv select` on <input> first.
    --consistent-fakes     For structured-faker columns with bounded cardinality
                           (cardinality fully captured by `frequency`), build a
                           stable source-value -> fake-value mapping so the same
                           source value always produces the same fake in the
                           output. Preserves the source frequency distribution
                           and overrides the default "emit real values when
                           frequency-enumerated" behavior for structured fakers
                           (names, emails, addresses, etc.). Has no effect on
                           unstructured columns (lorem_*, free_text, unknown),
                           all-unique columns, or non-faker columns. Useful for
                           deidentified synthesis where you want stable joins
                           on the faked columns.
    --no-relationships     Disable inter-column relationship modeling. Every
                           column is generated independently even when the
                           dictionary declares a `relationships` array.
    --joint-cardinality-cap <n>  Maximum number of distinct value-tuples a
                           `joint` relationship may have. A joint group above
                           this cap falls back to independent generation (or
                           aborts under --strict-relationships). 0 means
                           unlimited. [default: 100000]
    --correlation-threshold <f>  Minimum absolute Spearman correlation for a
                           pair of columns to stay in a `correlated`
                           relationship. Weakly-correlated members are dropped.
                           [default: 0.3]
    --strict-relationships  Abort instead of warning-and-degrading when a
                           declared relationship fails validation.
    -j, --jobs <arg>       Number of jobs to use for the internal `stats` and
                           `frequency` runs.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading the input CSV.
                           Must be a single character. (default: ,)
"#;

use std::{cell::RefCell, collections::HashMap, path::Path};

use rand::{SeedableRng, rngs::StdRng};
use serde::Deserialize;

use crate::{
    CliError, CliResult,
    cmd::describegpt::dictionary::{parse_frequency_csv, parse_stats_csv},
    config::{Config, Delimiter},
    util,
};

mod dictionary;
mod faker_map;
mod generator;
mod group_generator;
mod range_trim;
mod relationships;
mod report;

use faker_map::Locale;
use relationships::{EmitUnit, ScheduleParams, build_emit_schedule};

#[derive(Deserialize)]
struct Args {
    arg_input:                  String,
    flag_dictionary:            Option<String>,
    flag_infer_content_type:    bool,
    flag_rows:                  u64,
    flag_seed:                  Option<u64>,
    flag_locale:                String,
    flag_freq_limit:            usize,
    flag_stats_options:         Option<String>,
    flag_consistent_fakes:      bool,
    flag_no_relationships:      bool,
    flag_joint_cardinality_cap: u64,
    flag_min_count:             u64,
    flag_report:                Option<String>,
    flag_correlation_threshold: f64,
    flag_strict_relationships:  bool,
    flag_jobs:                  Option<usize>,
    flag_output:                Option<String>,
    flag_delimiter:             Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Parse --locale into the typed dispatch enum. Case-insensitive; unknown
    // tokens surface the user's exact spelling alongside the supported list.
    let locale = Locale::from_token(&args.flag_locale).map_err(|bad| {
        CliError::IncorrectUsage(format!(
            "Unsupported --locale '{bad}'. Supported: {}.",
            Locale::ALL.join(", "),
        ))
    })?;

    if args.flag_rows == 0 {
        return fail_incorrectusage_clierror!("--rows must be greater than 0.");
    }
    // Column selection and --weight change which columns stats reports and in
    // what order, which synthesize can't see; every pass that reads source
    // cells relies on stats covering every column in order. Select first.
    if let Some(opts) = args.flag_stats_options.as_deref() {
        // `stats_options_select` also catches `s` ending a short-flag cluster
        // (`-Es 2,1`); the token scan covers `--weight` and a bare `--select`.
        let bad = opts
            .split_whitespace()
            .find(|t| {
                *t == "--select"
                    || t.starts_with("--select=")
                    || *t == "--weight"
                    || t.starts_with("--weight=")
                    || (t.starts_with("-s") && !t.starts_with("--"))
            })
            .map(|t| format!("'{t}'"))
            .or_else(|| util::stats_options_select(opts).map(|sel| format!("a selection ({sel})")));
        if let Some(bad) = bad {
            return fail_incorrectusage_clierror!(
                "--stats-options may not include {bad}: synthesize needs stats for every column. \
                 To synthesize a subset of columns, run `qsv select` on the input first."
            );
        }
    }
    if args.flag_min_count == 0 {
        return fail_incorrectusage_clierror!(
            "--min-count must be at least 1 (1 disables suppression)."
        );
    }

    let input_path = args.arg_input.as_str();
    if !Path::new(input_path).is_file() {
        return fail_clierror!("Input file '{input_path}' does not exist or is not a file.");
    }
    if let Some(report_path) = &args.flag_report {
        let report = report::resolve_path(report_path);
        let clashes = std::iter::once(Some(input_path))
            .chain([args.flag_dictionary.as_deref(), args.flag_output.as_deref()])
            .flatten()
            .any(|other| report::resolve_path(other) == report);
        if clashes {
            return fail_incorrectusage_clierror!(
                "--report must not be the input, --dictionary or --output file."
            );
        }
    }

    // Single master RNG, threaded through both the sampling/selection logic and
    // every faker call, so a given --seed produces fully reproducible output.
    let mut rng = match args.flag_seed {
        Some(seed) => StdRng::seed_from_u64(seed), // DevSkim: ignore DS148264
        _ => rand::make_rng::<StdRng>(),
    };

    // --- 1. Analysis (stats + frequency + count) ---------------------------
    // Index the input first (best-effort) to speed up stats/frequency.
    // `Config::index_files()` returns `Ok(None)` when no index exists (not an
    // error) and may auto-index large inputs internally — so we explicitly fall
    // back to `qsv index` whenever it doesn't hand us an existing index.
    let config = Config::new(Some(&args.arg_input));
    if !matches!(config.index_files(), Ok(Some(_))) {
        let _ = util::run_qsv_cmd("index", &[], input_path, "  Indexed input");
    }

    // Delimiter passthrough for the analysis subcommands. `qsv index` accepts
    // neither `-d` nor `--jobs`; `qsv count` accepts `-d` but not `--jobs`;
    // `stats` and `frequency` accept both.
    let delim_arg: Option<String> = args.flag_delimiter.map(|d| {
        let byte = d.as_byte();
        if byte == b'\t' {
            "\\t".to_string()
        } else {
            (byte as char).to_string()
        }
    });
    let mut delim_only_args: Vec<String> = Vec::new();
    if let Some(ref delim) = delim_arg {
        delim_only_args.push("-d".to_string());
        delim_only_args.push(delim.clone());
    }
    let mut analysis_common_args: Vec<String> = delim_only_args.clone();
    if let Some(jobs) = args.flag_jobs {
        analysis_common_args.push("--jobs".to_string());
        analysis_common_args.push(jobs.to_string());
    }

    // stats — --cardinality/--quartiles/--infer-dates are always required.
    let mut stats_args: Vec<String> = vec![
        "--cardinality".to_string(),
        "--quartiles".to_string(),
        "--infer-dates".to_string(),
    ];
    stats_args.extend(analysis_common_args.iter().cloned());
    if let Some(ref opts) = args.flag_stats_options {
        stats_args.extend(opts.split_whitespace().map(ToString::to_string));
    }
    let stats_args_ref: Vec<&str> = stats_args.iter().map(String::as_str).collect();
    let (stats_csv, _) = util::run_qsv_cmd(
        "stats",
        &stats_args_ref,
        input_path,
        "  Computed summary statistics",
    )?;

    // frequency
    let mut freq_args: Vec<String> = vec!["--limit".to_string(), args.flag_freq_limit.to_string()];
    freq_args.extend(analysis_common_args.iter().cloned());
    let freq_args_ref: Vec<&str> = freq_args.iter().map(String::as_str).collect();
    let (freq_csv, _) = util::run_qsv_cmd(
        "frequency",
        &freq_args_ref,
        input_path,
        "  Computed frequency distribution",
    )?;

    // count — exact source row count, needed for null-ratio math. `count`
    // does NOT accept `--jobs`, so use the delimiter-only args here.
    let count_args_ref: Vec<&str> = delim_only_args.iter().map(String::as_str).collect();
    let (count_out, _) = util::run_qsv_cmd("count", &count_args_ref, input_path, "  Counted rows")?;
    let total_rows: u64 = count_out.trim().parse().map_err(|e| {
        CliError::Other(format!(
            "Unable to parse row count '{}': {e}",
            count_out.trim()
        ))
    })?;

    // --- 2. Parse analysis -------------------------------------------------
    let (mut stats_records, _addl_cols) = parse_stats_csv(&stats_csv)?;
    let mut frequency_records = parse_frequency_csv(&freq_csv)?;
    // Without headers stats names columns "0", "1", … but frequency names them
    // "1", "2", …; renumber frequency's so both point at the same column.
    if crate::config::resolved_no_headers(false) {
        for fr in &mut frequency_records {
            if let Some(n) = fr
                .field
                .parse::<usize>()
                .ok()
                .and_then(|n| n.checked_sub(1))
            {
                fr.field = n.to_string();
            }
        }
    }

    if stats_records.is_empty() {
        return fail_clierror!("No columns found in '{input_path}'.");
    }

    // Trim numeric/date ranges so outliers seen fewer than --min-count times
    // can't reappear at their real values. Dates are parsed with the same DMY
    // preference `stats` used to type the column.
    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY")
        || args
            .flag_stats_options
            .as_deref()
            .is_some_and(|o| o.split_whitespace().any(|t| t == "--prefer-dmy"));
    // Stats records are not positional (`--stats-options` can `--select` or
    // reorder), so every pass that reads source cells maps by header name.
    let source_config = Config::new(Some(&args.arg_input)).delimiter(args.flag_delimiter);
    let source_headers = source_config.reader()?.headers()?.clone();
    let source_cols = range_trim::source_column_indices(
        &stats_records,
        &source_headers,
        source_config.no_headers,
    );
    range_trim::trim_range_bounds(
        &mut stats_records,
        &source_cols,
        input_path,
        args.flag_delimiter,
        args.flag_min_count,
        prefer_dmy,
    )?;

    // --- 3. Dictionary (optional) → content types + relationships ----------
    let (content_types, relationships) = if let Some(ref dict_path) = args.flag_dictionary {
        dictionary::load_dictionary(dict_path)?
    } else if args.flag_infer_content_type {
        dictionary::infer_dictionary(input_path)?
    } else {
        (HashMap::new(), Vec::new())
    };
    // --no-relationships forces fully independent generation (legacy behavior).
    let relationships = if args.flag_no_relationships {
        Vec::new()
    } else {
        relationships
    };

    // --- 4. Build the row-emission schedule --------------------------------
    // Ungrouped columns get one ColumnGenerator each; columns named by a valid
    // `relationship` are grouped under a GroupGenerator that emits them jointly.
    let suppressions = RefCell::new(Vec::new());
    let schedule_params = ScheduleParams {
        stats_records: &stats_records,
        frequency_records: &frequency_records,
        content_types: &content_types,
        relationships: &relationships,
        total_rows,
        requested_rows: args.flag_rows,
        locale,
        consistent_fakes: args.flag_consistent_fakes,
        joint_cardinality_cap: args.flag_joint_cardinality_cap,
        min_count: args.flag_min_count,
        source_cols: &source_cols,
        correlation_threshold: args.flag_correlation_threshold,
        strict: args.flag_strict_relationships,
        input_path,
        delimiter: args.flag_delimiter,
        suppressions: &suppressions,
    };
    let schedule = build_emit_schedule(&schedule_params, &mut rng)?;

    // Only with --report: hash every source row so emitted rows can be checked
    // for exact copies. Uses no RNG, so output is the same with or without it.
    let report_keys = match args.flag_report {
        Some(_) => {
            let keyer = report::RowKeyer::new(&stats_records, &source_cols, &schedule, prefer_dmy);
            let keys = report::source_row_keys(&keyer, input_path, args.flag_delimiter)?;
            Some((keyer, keys))
        },
        None => None,
    };
    let mut exact_matches = 0_u64;

    // --- 5. Emit -----------------------------------------------------------
    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;

    let header: csv::StringRecord = stats_records
        .iter()
        .map(|stats_record| stats_record.field.clone())
        .collect();
    wtr.write_record(&header)?;

    // One reusable row buffer: each EmitUnit writes into its own column slot(s)
    // by index, so the schedule can be walked in any fixed order per row.
    let ncols = stats_records.len();
    let mut row: Vec<String> = vec![String::new(); ncols];
    let mut record = csv::StringRecord::new();
    for _ in 0..args.flag_rows {
        for unit in &schedule {
            match unit {
                EmitUnit::Independent { col_idx, generator } => {
                    row[*col_idx] = generator.next(&mut rng);
                },
                EmitUnit::Group(group) => {
                    for (col_idx, value) in group.next(&mut rng) {
                        row[col_idx] = value;
                    }
                },
            }
        }
        if let Some((keyer, keys)) = &report_keys
            && keys.contains(&keyer.synthetic_key(&row))
        {
            exact_matches += 1;
        }
        record.clear();
        for value in &row {
            record.push_field(value);
        }
        wtr.write_record(&record)?;
    }
    wtr.flush()?;

    if let (Some(report_path), Some((keyer, keys))) = (&args.flag_report, &report_keys) {
        let compared: Vec<usize> = keyer.compared().collect();
        let names = |pred: &dyn Fn(usize) -> bool| -> Vec<&str> {
            stats_records
                .iter()
                .enumerate()
                .filter(|(i, _)| pred(*i))
                .map(|(_, sr)| sr.field.as_str())
                .collect()
        };
        // With no column compared every row hashes the same, so a match count
        // would be meaningless: report the row metrics as null instead.
        let any_compared = !compared.is_empty();
        #[allow(clippy::cast_precision_loss)]
        let rate = exact_matches as f64 / args.flag_rows as f64;
        let report = serde_json::json!({
            "source_rows": total_rows,
            "synthetic_rows": args.flag_rows,
            "seed": args.flag_seed,
            "min_count": args.flag_min_count,
            "privacy": {
                "exact_matches": any_compared.then_some(exact_matches),
                "exact_match_rate": any_compared.then_some(rate),
                "distinct_source_rows": any_compared.then_some(keys.len()),
                "compared_columns": names(&|i| compared.contains(&i)),
                "excluded_columns": names(&|i| !compared.contains(&i)),
                "suppressed": &*suppressions.borrow(),
            },
        });
        std::fs::write(report_path, serde_json::to_string_pretty(&report)? + "\n")?;
    }
    Ok(())
}
