pub(crate) static USAGE: &str = r#"
Generates a synthetic CSV that is statistically faithful to a source CSV.

`synthesize` analyzes <input> with `stats` and `frequency`, then emits N rows of
fake data that reproduce the source's per-column attributes:

  * Categorical / low-cardinality columns are reproduced by frequency-weighted
    sampling of their *real* value set — cardinality, weights and repetition
    structure are preserved, minus values rarer than the --min-count.
  * Numeric and date/datetime columns are reproduced with quantile buckets
    (quartiles by default, see --quantiles), so the shape of the distribution
    (not just its [min,max] range) is preserved.
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
    values, and values inside the range are drawn uniformly per bucket, so
    integers and dates can still land exactly on real values. With
    the --min-count at 1 the range is the real min and max, and an outlier
    can reappear at or near its real value.

Review the output before sharing it, or training on it, as de-identified data.
A model saved with --save-model holds real values from <input>, such as
enumerated values, bucket boundaries and range ends, including some the
output never shows. Review and share it at least as carefully as the output.

With --seed, output is fully reproducible, whatever the --jobs setting.
Rows are generated in parallel, in fixed blocks of 8192 rows. With --seed
and --shard, several machines can each generate one slice of a dataset.
To profile <input> only once, save the fitted model with --save-model and
generate the other slices from it with --model.

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

  # Generate 1B rows as 4 slices, e.g. one per machine, then join them
  # into the same file a single run would write
  $ qsv synthesize data.csv -n 1000000000 --seed 42 --shard 1/4 -o part1.csv
  $ qsv synthesize data.csv -n 1000000000 --seed 42 --shard 2/4 -o part2.csv
  ...
  $ qsv cat rows part1.csv part2.csv part3.csv part4.csv -o synthetic.csv

  # The same, profiling data.csv only once: the first slice saves the
  # fitted model, and the other slices generate from it without data.csv
  $ qsv synthesize data.csv -n 1000000000 --seed 42 --shard 1/4 --save-model model.json -o part1.csv
  $ qsv synthesize --model model.json -n 1000000000 --seed 42 --shard 2/4 -o part2.csv
  ...

  # Typed training data: Parquet keeps integers, floats, dates and nulls
  $ qsv synthesize data.csv -n 1000000 --seed 42 --format parquet -o synthetic.parquet

  # The same rows as 10 Parquet files of 100,000 rows each,
  # synthetic_000000.parquet to synthetic_900000.parquet
  $ qsv synthesize data.csv -n 1000000 --seed 42 --format parquet --rows-per-file 100000 -o synthetic_{}.parquet

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_synthesize.rs.
See also https://github.com/dathere/qsv/wiki/AI-and-Documentation#synthesize

Usage:
    qsv synthesize [options] <input>
    qsv synthesize [options] --model <file>
    qsv synthesize --help

synthesize arguments:
    <input>                The source CSV to profile. Omitted with --model,
                           which generates from a saved model instead.

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
    --seed <n>             RNG seed for fully reproducible output. The same
                           seed gives the same rows at any --jobs, unless
                           stats uses approximate estimators: either asked
                           for via --stats-options (e.g. `--quantile-method
                           approx`), or switched on by stats itself (with a warning)
                           when the input won't fit in memory.
    --format <fmt>         Output format: csv, jsonl or parquet. JSONL
                           and Parquet keep each column's type: integers
                           and floats as such, dates as Parquet dates (JSONL
                           writes them as strings), empty values as null.
                           A true/false column is written as booleans only
                           if stats types it Boolean (pass it the option
                           `--infer-boolean` via --stats-options); else it
                           is text. A column sampled from real values
                           keeps the type stats gave it only if every value
                           converts unchanged, so a zero-padded code stays
                           text. Parquet needs --output and a qsv built with
                           the polars feature. [default: csv]
    --shard <k/N>          Generate only slice k of N (k from 1 to N) of the
                           rows --rows asks for. Requires --seed. Each slice
                           is a run of whole 8192-row blocks, and slices 1 to
                           N joined in order are the same as one unsharded
                           run: CSV slices with `qsv cat rows`, JSONL slices
                           by plain concatenation, while Parquet slices are
                           separate files to read together as one dataset.
                           Every slice must use the
                           same <input>, options, qsv version and
                           dictionary file, if any (--infer-content-type
                           without --dictionary is refused). Each slice still
                           profiles all of <input> itself, so if stats or
                           frequency warns on one machine that it switched to
                           approximate estimators for lack of memory, that
                           slice comes from a different fit and the joined
                           rows won't match one run. Generating the slices
                           from one model saved with --save-model avoids
                           this. Each CSV slice gets a header row (and each
                           Parquet slice its schema) even when it has no rows
                           (when N exceeds the number of blocks). The counts
                           of --report are for this slice only.
    --rows-per-file <n>    Split the output into files of <n> rows each (the
                           last file may hold fewer). The --output name must
                           contain {}, which is replaced by the zero-based
                           row number of the file's first row, as in `qsv
                           split`, zero-padded so the names sort in order.
                           For example, 1000 rows at 250 per file with
                           `-o synthetic_{}.csv` write synthetic_000.csv,
                           synthetic_250.csv, synthetic_500.csv and
                           synthetic_750.csv. The files hold the
                           same rows a single-file run writes, in order, and
                           are joined the same way as --shard slices. Each
                           CSV file gets a header row and each Parquet file
                           its schema. Can't be combined with --shard.
    --save-model <file>    Also save the fitted model to <file>, so later
                           runs can generate from it with --model without
                           reading <input> again. A --model run with the
                           same --rows and --seed writes the same rows as
                           this run (pools of fake values are sized for
                           this run's --rows). The model holds real values
                           from <input>, some of which the output may never
                           show (see the privacy note above), so share it
                           at least as carefully as the output. Its format
                           is internal to qsv and versioned: a qsv that reads
                           another model version refuses it, and it must be
                           fitted again.
    --model <file>         Generate from a model saved with --save-model
                           instead of profiling an input, which must then be
                           omitted. The model fixes everything fitted from
                           the input, so options that shape the fit or read
                           the input are refused when they have no default
                           (for example the --dictionary, --stats-options,
                           the --report and --delimiter options) and ignored
                           when they do (for example the --locale,
                           the --min-count and the frequency limit).
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
    --quantiles <n>        Number of equal-share buckets that reproduce the
                           shape of numeric and date columns: 2, 4, 5, 10,
                           20, 25, 50 or 100. More buckets keep skewed and
                           long-tailed columns closer to the source. 4 (the
                           default) cuts at the source quartiles; other values
                           cut at `stats --percentiles` (real source values).
                           A column with too few non-null values for every
                           bucket to hold more than <k> of them (<k> is set
                           by the --min-count option) gets fewer buckets, or
                           the quartiles.
                           Also applies to the gaps of `ordered` relationships.
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
                           `frequency` runs and for generating rows.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading the input CSV.
                           Must be a single character. (default: ,)
"#;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    path::Path,
};

use rand::{SeedableRng, rngs::StdRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;

use crate::{
    CliError, CliResult,
    cmd::describegpt::dictionary::{StatsRecord, parse_frequency_csv, parse_stats_csv},
    config::{Config, Delimiter},
    util,
};

mod dictionary;
mod faker_map;
mod generator;
mod group_generator;
mod model;
mod output;
mod range_trim;
mod relationships;
mod report;

use faker_map::Locale;
use relationships::{EmitUnit, ScheduleParams, build_emit_schedule};

#[derive(Deserialize)]
struct Args {
    arg_input:                  Option<String>,
    flag_model:                 Option<String>,
    flag_save_model:            Option<String>,
    flag_dictionary:            Option<String>,
    flag_infer_content_type:    bool,
    flag_rows:                  u64,
    flag_seed:                  Option<u64>,
    flag_shard:                 Option<String>,
    flag_rows_per_file:         Option<u64>,
    flag_format:                String,
    flag_locale:                String,
    flag_freq_limit:            usize,
    flag_quantiles:             Option<usize>,
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

    if args.flag_rows == 0 {
        return fail_incorrectusage_clierror!("--rows must be greater than 0.");
    }
    if let Some(q) = args.flag_quantiles
        && !generator::QUANTILE_CHOICES.contains(&q)
    {
        return fail_incorrectusage_clierror!(
            "--quantiles must be one of 2, 4, 5, 10, 20, 25, 50 or 100 (a divisor of 100), got \
             {q}."
        );
    }
    let shard = match args.flag_shard.as_deref() {
        Some(_) if args.flag_seed.is_none() => {
            return fail_incorrectusage_clierror!(
                "--shard requires --seed, so every slice comes from the same dataset."
            );
        },
        // Each slice would make its own, unseeded LLM call, which can type a
        // column differently on each machine.
        Some(_) if args.flag_infer_content_type && args.flag_dictionary.is_none() => {
            return fail_incorrectusage_clierror!(
                "--shard needs a shared --dictionary instead of --infer-content-type, so every \
                 slice uses the same column types."
            );
        },
        Some(spec) => Some(parse_shard(spec)?),
        None => None,
    };
    let format = output::Format::parse(&args.flag_format)?;
    // Config treats `-o -` as stdout too.
    if format == output::Format::Parquet && args.flag_output.as_deref().is_none_or(|o| o == "-") {
        return fail_incorrectusage_clierror!(
            "--format parquet needs --output: Parquet can't be written to stdout."
        );
    }
    let file_names = match (args.flag_rows_per_file, args.flag_output.as_deref()) {
        (None, _) => None,
        (Some(0), _) => {
            return fail_incorrectusage_clierror!("--rows-per-file must be greater than 0.");
        },
        (Some(_), _) if args.flag_shard.is_some() => {
            return fail_incorrectusage_clierror!(
                "--rows-per-file can't be combined with --shard."
            );
        },
        (Some(_), None | Some("-")) => {
            return fail_incorrectusage_clierror!(
                "--rows-per-file needs --output, a file name with {{}} in it."
            );
        },
        (Some(n), Some(template)) => Some((
            n,
            output::FileNames::new(template, (args.flag_rows - 1) / n * n)?,
        )),
    };
    if let Some((n, names)) = &file_names
        && let Some(path) = [
            args.arg_input.as_deref(),
            args.flag_dictionary.as_deref(),
            args.flag_model.as_deref(),
            args.flag_save_model.as_deref(),
            args.flag_report.as_deref(),
        ]
        .into_iter()
        .flatten()
        .find(|p| names.writes(p, *n, args.flag_rows))
    {
        return fail_incorrectusage_clierror!("--rows-per-file would write over '{path}'.");
    }
    if args.flag_model.is_some() {
        // These only shape the fit, which the model already fixed. Options with
        // a default (--locale, --min-count, …) can't be told apart from unset,
        // so they are ignored instead (see USAGE).
        let fit_only = [
            (args.arg_input.is_some(), "<input>"),
            (args.flag_dictionary.is_some(), "--dictionary"),
            (args.flag_infer_content_type, "--infer-content-type"),
            (args.flag_stats_options.is_some(), "--stats-options"),
            (args.flag_consistent_fakes, "--consistent-fakes"),
            (args.flag_no_relationships, "--no-relationships"),
            (args.flag_strict_relationships, "--strict-relationships"),
            (args.flag_save_model.is_some(), "--save-model"),
            (args.flag_report.is_some(), "--report"),
            (args.flag_delimiter.is_some(), "--delimiter"),
            (args.flag_quantiles.is_some(), "--quantiles"),
        ];
        if let Some((_, name)) = fit_only.iter().find(|(set, _)| *set) {
            return fail_incorrectusage_clierror!(
                "--model can't be combined with {name}: the model was already fitted from its \
                 source."
            );
        }
    }
    let clashes_with_inputs = |path: &str| {
        let path = report::resolve_path(path);
        [
            args.arg_input.as_deref(),
            args.flag_dictionary.as_deref(),
            args.flag_output.as_deref(),
        ]
        .into_iter()
        .flatten()
        .any(|other| report::resolve_path(other) == path)
    };
    if args
        .flag_save_model
        .as_deref()
        .is_some_and(clashes_with_inputs)
    {
        return fail_incorrectusage_clierror!(
            "--save-model must not be the input, --dictionary or --output file."
        );
    }
    if let Some(report_path) = args.flag_report.as_deref()
        && (clashes_with_inputs(report_path)
            || args
                .flag_save_model
                .as_deref()
                .is_some_and(|m| report::resolve_path(m) == report::resolve_path(report_path)))
    {
        return fail_incorrectusage_clierror!(
            "--report must not be the input, --dictionary, --output or --save-model file."
        );
    }

    // The fit (pools, fake values, relationship validation) draws from an RNG
    // seeded with the base seed; each output chunk gets its own RNG keyed on
    // (base seed, chunk index), so a given --seed gives the same output at any
    // --jobs, and from a saved model as from the fit that saved it.
    let base_seed = args.flag_seed.unwrap_or_else(rand::random::<u64>);

    let (model, fitted) = match args.flag_model.as_deref() {
        Some(model_path) => (model::Model::load(model_path)?, None),
        None => {
            let (model, fitted) = fit(&args, base_seed)?;
            (model, Some(fitted))
        },
    };
    if let Some(model_path) = args.flag_save_model.as_deref() {
        model.save(model_path)?;
    }

    // Only with --report: hash every source row so emitted rows can be checked
    // for exact copies. Uses no RNG, so output is the same with or without it.
    let report_keys = match (&args.flag_report, &fitted) {
        (Some(_), Some(fitted)) => {
            let keyer = report::RowKeyer::new(
                &fitted.stats_records,
                &fitted.source_cols,
                &model.schedule,
                fitted.prefer_dmy,
            );
            let keys = report::source_row_keys(&keyer, &fitted.input_path, args.flag_delimiter)?;
            Some((keyer, keys))
        },
        _ => None,
    };
    let mut exact_matches = 0_u64;

    // --- 5. Emit -----------------------------------------------------------
    output::check_unique_columns(&model.columns, format)?;
    let first_file = file_names.as_ref().map(|(_, names)| names.name(0));
    let out_path = first_file.as_deref().or(args.flag_output.as_deref());
    let out_conf = Config::new(out_path.map(str::to_string).as_ref());
    let chunk_builder = out_conf.writer_builder();
    let jsonl_keys = if format == output::Format::Jsonl {
        Some(output::jsonl_keys(&model.columns)?)
    } else {
        None
    };
    #[cfg(feature = "polars")]
    let names: std::sync::Arc<[String]> = model.columns.clone().into();
    let open =
        |path: Option<&str>| output::Output::open(format, path, &model.columns, &model.types);
    let mut out = open(out_path)?;
    let mut file_start = 0_u64;
    let new_sink = |rows: u64, size_hint: u64| -> output::Sink {
        match format {
            output::Format::Csv => output::Sink::csv(&chunk_builder, size_hint as usize),
            output::Format::Jsonl => {
                output::Sink::jsonl(jsonl_keys.clone().unwrap_or_default(), size_hint as usize)
            },
            #[cfg(feature = "polars")]
            output::Format::Parquet => output::Sink::Parquet(output::parquet::Columns::new(
                names.clone(),
                &model.types,
                rows as usize,
                model.prefer_dmy,
            )),
            #[cfg(not(feature = "polars"))]
            output::Format::Parquet => {
                let _ = rows;
                unreachable!("Format::parse refuses parquet")
            },
        }
    };

    // Chunks are generated in parallel, a window at a time, and written in
    // chunk order. The window starts at one chunk and is then sized from the
    // largest chunk seen so far, so the bytes held at once stay near
    // WINDOW_BYTES (but never below one chunk) however wide the rows are. The
    // window only affects memory and speed, never the output.
    let ncols = model.columns.len();
    let total_chunks = args.flag_rows.div_ceil(CHUNK_ROWS);
    let (mut first_chunk, end_chunk) =
        shard.map_or((0, total_chunks), |(k, n)| shard_chunks(total_chunks, k, n));
    let synthetic_rows = end_chunk.saturating_mul(CHUNK_ROWS).min(args.flag_rows)
        - first_chunk.saturating_mul(CHUNK_ROWS).min(args.flag_rows);
    let max_window = (util::njobs(args.flag_jobs) as u64 * 2).max(1);
    let mut window = 1_u64;
    let mut largest_chunk = 0_u64;
    while first_chunk < end_chunk {
        let last_chunk = (first_chunk + window).min(end_chunk);
        let chunks: Vec<CliResult<(Vec<output::Encoded>, u64)>> = (first_chunk..last_chunk)
            .into_par_iter()
            .map(|chunk| {
                let rows = CHUNK_ROWS.min(args.flag_rows - chunk * CHUNK_ROWS);
                // Sized from the largest chunk so far, so the buffer doesn't
                // double past it.
                generate_chunk(
                    &|piece_rows| new_sink(piece_rows, largest_chunk * piece_rows / rows),
                    &model.schedule,
                    &model.types,
                    ncols,
                    base_seed,
                    chunk,
                    rows,
                    args.flag_rows_per_file,
                    report_keys.as_ref(),
                )
            })
            .collect();
        for (chunk, encoded) in (first_chunk..).zip(chunks) {
            let (pieces, matches) = encoded?;
            exact_matches += matches;
            largest_chunk = largest_chunk.max(pieces.iter().map(output::Encoded::size).sum());
            for (i, piece) in pieces.into_iter().enumerate() {
                if let Some((n, names)) = &file_names
                    && i > 0
                {
                    file_start += n;
                    // Finish this file before opening the next, so a failure
                    // to open it leaves this one complete.
                    let done = std::mem::replace(
                        &mut out,
                        output::Output::Bytes(Box::new(std::io::sink())),
                    );
                    done.finish()?;
                    out = open(Some(&names.name(file_start)))?;
                }
                out.write(chunk, end_chunk, piece)?;
            }
        }
        first_chunk = last_chunk;
        window = (WINDOW_BYTES / largest_chunk.max(1)).clamp(1, max_window);
    }
    out.finish()?;

    if let (Some(report_path), Some((keyer, keys)), Some(fitted)) =
        (&args.flag_report, &report_keys, &fitted)
    {
        let compared: Vec<usize> = keyer.compared().collect();
        let names = |pred: &dyn Fn(usize) -> bool| -> Vec<&str> {
            model
                .columns
                .iter()
                .enumerate()
                .filter(|(i, _)| pred(*i))
                .map(|(_, name)| name.as_str())
                .collect()
        };
        // With no column compared every row hashes the same, so a match count
        // would be meaningless: report the row metrics as null instead.
        let any_compared = !compared.is_empty();
        #[allow(clippy::cast_precision_loss)]
        let rate = (any_compared && synthetic_rows > 0)
            .then(|| exact_matches as f64 / synthetic_rows as f64);
        let report = serde_json::json!({
            "source_rows": fitted.total_rows,
            "synthetic_rows": synthetic_rows,
            "shard": args.flag_shard,
            "seed": args.flag_seed,
            "min_count": args.flag_min_count,
            "privacy": {
                "exact_matches": any_compared.then_some(exact_matches),
                "exact_match_rate": rate,
                "distinct_source_rows": any_compared.then_some(keys.len()),
                "compared_columns": names(&|i| compared.contains(&i)),
                "excluded_columns": names(&|i| !compared.contains(&i)),
                "suppressed": &fitted.suppressions,
            },
        });
        std::fs::write(report_path, serde_json::to_string_pretty(&report)? + "\n")?;
    }
    Ok(())
}

/// What a fit leaves behind besides the model, for `--report`.
struct Fitted {
    input_path:    String,
    stats_records: Vec<StatsRecord>,
    source_cols:   Vec<Option<usize>>,
    prefer_dmy:    bool,
    total_rows:    u64,
    suppressions:  Vec<report::Suppression>,
}

/// Profiles <input> (stats, frequency, count, optional dictionary) and builds
/// the emit schedule from it.
fn fit(args: &Args, base_seed: u64) -> CliResult<(model::Model, Fitted)> {
    // Parse --locale into the typed dispatch enum. Case-insensitive; unknown
    // tokens surface the user's exact spelling alongside the supported list.
    let locale = Locale::from_token(&args.flag_locale).map_err(|bad| {
        CliError::IncorrectUsage(format!(
            "Unsupported --locale '{bad}'. Supported: {}.",
            Locale::ALL.join(", "),
        ))
    })?;
    if args.flag_min_count == 0 {
        return fail_incorrectusage_clierror!(
            "--min-count must be at least 1 (1 disables suppression)."
        );
    }
    let mut rng = StdRng::seed_from_u64(base_seed); // DevSkim: ignore DS148264

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
    let quantiles = args.flag_quantiles.unwrap_or(4);
    if quantiles != 4
        && let Some(bad) = args.flag_stats_options.as_deref().and_then(|opts| {
            opts.split_whitespace()
                .find(|t| t.starts_with("--percentiles") || t.starts_with("--percentile-list"))
        })
    {
        return fail_incorrectusage_clierror!(
            "--stats-options may not include '{bad}' with --quantiles: synthesize sets the \
             percentiles itself."
        );
    }

    // Docopt requires <input> whenever --model is absent.
    let input_path = args.arg_input.as_deref().unwrap_or_default();
    if !Path::new(input_path).is_file() {
        return fail_clierror!("Input file '{input_path}' does not exist or is not a file.");
    }

    // --- 1. Analysis (stats + frequency + count) ---------------------------
    // Index the input first (best-effort) to speed up stats/frequency.
    // `Config::index_files()` returns `Ok(None)` when no index exists (not an
    // error) and may auto-index large inputs internally — so we explicitly fall
    // back to `qsv index` whenever it doesn't hand us an existing index.
    let config = Config::new(args.arg_input.as_ref());
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
    if quantiles != 4 {
        let list: Vec<String> = (1..quantiles)
            .map(|i| (100 * i / quantiles).to_string())
            .collect();
        stats_args.push("--percentiles".to_string());
        stats_args.push("--percentile-list".to_string());
        stats_args.push(list.join(","));
        // Edges are fitted values, not display output: rounding to the default
        // 4 places collapses small-magnitude floats onto one edge.
        let user_round = args
            .flag_stats_options
            .as_deref()
            .is_some_and(|o| o.split_whitespace().any(|t| t.starts_with("--round")));
        if !user_round {
            stats_args.push("--round".to_string());
            stats_args.push("9999".to_string());
        }
    }
    stats_args.extend(analysis_common_args.iter().cloned());
    if let Some(ref opts) = args.flag_stats_options {
        stats_args.extend(opts.split_whitespace().map(ToString::to_string));
    }
    let stats_args_ref: Vec<&str> = stats_args.iter().map(String::as_str).collect();
    // stats and frequency are silent unless they warn (e.g. that they switched
    // to approximate estimators for lack of memory), so pass that on.
    let (stats_csv, stats_err) = util::run_qsv_cmd(
        "stats",
        &stats_args_ref,
        input_path,
        "  Computed summary statistics",
    )?;
    eprint!("{stats_err}");

    // frequency
    let mut freq_args: Vec<String> = vec!["--limit".to_string(), args.flag_freq_limit.to_string()];
    freq_args.extend(analysis_common_args.iter().cloned());
    let freq_args_ref: Vec<&str> = freq_args.iter().map(String::as_str).collect();
    let (freq_csv, freq_err) = util::run_qsv_cmd(
        "frequency",
        &freq_args_ref,
        input_path,
        "  Computed frequency distribution",
    )?;
    eprint!("{freq_err}");

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
    let source_config = Config::new(args.arg_input.as_ref()).delimiter(args.flag_delimiter);
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
    generator::apply_quantiles(
        &mut stats_records,
        quantiles,
        total_rows,
        args.flag_min_count,
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
        quantiles,
        min_count: args.flag_min_count,
        source_cols: &source_cols,
        correlation_threshold: args.flag_correlation_threshold,
        strict: args.flag_strict_relationships,
        input_path,
        delimiter: args.flag_delimiter,
        suppressions: &suppressions,
    };
    let schedule = build_emit_schedule(&schedule_params, &mut rng)?;

    let columns = stats_records.iter().map(|sr| sr.field.clone()).collect();
    let stats_types: Vec<&str> = stats_records.iter().map(|sr| sr.r#type.as_str()).collect();
    let types = output::resolve_types(&schedule, &stats_types, prefer_dmy);
    let fit_info = model::FitInfo {
        seed:        args.flag_seed,
        rows:        args.flag_rows,
        source_rows: total_rows,
        min_count:   args.flag_min_count,
    };
    let fitted = Fitted {
        input_path: input_path.to_string(),
        stats_records,
        source_cols,
        prefer_dmy,
        total_rows,
        suppressions: suppressions.into_inner(),
    };
    Ok((
        model::Model::new(columns, types, prefer_dmy, schedule, fit_info),
        fitted,
    ))
}

/// Rows per output chunk. Each chunk has its own RNG, so this value is part of
/// what a --seed reproduces: changing it changes every seeded output.
const CHUNK_ROWS: u64 = 8192;

/// Parses a --shard spec `k/N` (1 <= k <= N) into `(k, N)`.
fn parse_shard(spec: &str) -> CliResult<(u64, u64)> {
    let parsed = spec
        .split_once('/')
        .and_then(|(k, n)| Some((k.trim().parse::<u64>().ok()?, n.trim().parse::<u64>().ok()?)));
    match parsed {
        Some((k, n)) if k >= 1 && k <= n => Ok((k, n)),
        _ => fail_incorrectusage_clierror!(
            "--shard must be k/N with 1 <= k <= N (e.g. 2/4), got '{spec}'."
        ),
    }
}

/// The chunk range `[start, end)` of shard `k` of `n`. Shards are contiguous
/// and in order, so shards 1..=n together cover `0..total_chunks` exactly once.
fn shard_chunks(total_chunks: u64, k: u64, n: u64) -> (u64, u64) {
    let bound = |i: u64| (u128::from(total_chunks) * u128::from(i) / u128::from(n)) as u64;
    (bound(k - 1), bound(k))
}

/// Target for the encoded output held in memory at once across a window of chunks.
const WINDOW_BYTES: u64 = 256 * 1024 * 1024;

/// RNG for one output chunk, keyed on the base seed and the chunk index only
/// (never on fit-time RNG state), with fixed little-endian bytes so every
/// platform derives the same stream.
fn chunk_rng(base_seed: u64, chunk: u64) -> StdRng {
    let mut key = [0_u8; 32];
    key[..8].copy_from_slice(&base_seed.to_le_bytes());
    key[8..16].copy_from_slice(&chunk.to_le_bytes());
    key[16..24].copy_from_slice(b"qsvsynth");
    StdRng::from_seed(key)
}

/// Generates one chunk's rows, plus how many of them exactly match a source
/// row when `--report` keys are given. The rows are encoded as one piece per
/// output file they fall in: with `--rows-per-file`, a new piece starts at each
/// row that starts a file, so a chunk that begins a file has an empty first
/// piece. `new_sink` takes the piece's row count.
#[allow(clippy::too_many_arguments)]
fn generate_chunk(
    new_sink: &(dyn Fn(u64) -> output::Sink + Sync),
    schedule: &[EmitUnit],
    types: &[output::OutType],
    ncols: usize,
    base_seed: u64,
    chunk: u64,
    rows: u64,
    rows_per_file: Option<u64>,
    report_keys: Option<&(report::RowKeyer, HashSet<u128>)>,
) -> CliResult<(Vec<output::Encoded>, u64)> {
    let first_row = chunk * CHUNK_ROWS;
    let starts_file = |i: u64| {
        let row = first_row + i;
        rows_per_file.is_some_and(|n| row > 0 && row.is_multiple_of(n))
    };
    let piece_rows = |from: u64| {
        rows_per_file.map_or(rows - from, |n| {
            (n - (first_row + from) % n).min(rows - from)
        })
    };
    let mut rng = chunk_rng(base_seed, chunk);
    // One reusable row buffer: each EmitUnit writes into its own column slot(s)
    // by index, so the schedule can be walked in any fixed order per row.
    let mut row: Vec<String> = vec![String::new(); ncols];
    let mut matches = 0_u64;
    let mut pieces = Vec::with_capacity(1);
    let mut sink = new_sink(if starts_file(0) { 0 } else { piece_rows(0) });
    for i in 0..rows {
        if starts_file(i) {
            let done = std::mem::replace(&mut sink, new_sink(piece_rows(i)));
            pieces.push(done.finish()?);
        }
        for unit in schedule {
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
        if let Some((keyer, keys)) = report_keys
            && keys.contains(&keyer.synthetic_key(&row))
        {
            matches += 1;
        }
        sink.push(&mut row, types)?;
    }
    pieces.push(sink.finish()?);
    Ok((pieces, matches))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shards_cover_every_chunk_once_in_order() {
        for total in 0..40 {
            for n in 1..12 {
                let mut next = 0;
                for k in 1..=n {
                    let (start, end) = shard_chunks(total, k, n);
                    assert_eq!(start, next, "total {total}, shard {k}/{n}");
                    assert!(end >= start);
                    next = end;
                }
                assert_eq!(next, total, "total {total}, n {n}");
            }
        }
    }

    #[test]
    fn parse_shard_accepts_only_k_of_n() {
        assert_eq!(parse_shard("1/1").unwrap(), (1, 1));
        assert_eq!(parse_shard(" 3 / 4 ").unwrap(), (3, 4));
        for bad in ["0/4", "5/4", "1/0", "4", "/4", "1/", "-1/4", "a/4"] {
            assert!(parse_shard(bad).is_err(), "{bad}");
        }
    }
}
