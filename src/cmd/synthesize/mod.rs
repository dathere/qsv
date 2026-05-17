static USAGE: &str = r#"
Generates a synthetic CSV that is statistically faithful to a source CSV.

`synthesize` analyzes <input> with `stats` and `frequency`, then emits N rows of
fake data that reproduce the source's per-column attributes:

  * Categorical / low-cardinality columns are reproduced by frequency-weighted
    sampling of their *real* value set — cardinality, weights and repetition
    structure are preserved exactly.
  * Numeric and date/datetime columns are reproduced with quartile buckets, so
    the shape of the distribution (not just its [min,max] range) is preserved.
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

Columns are generated independently — cross-column correlation is not modeled.

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

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_synthesize.rs.

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
    --stats-options <arg>  Extra options appended to the internal `stats` run.
                           Note: cardinality, quartiles and date inference are
                           always enabled — do not re-specify them here.
    -j, --jobs <arg>       Number of jobs to use for the internal `stats` and
                           `frequency` runs.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading the input CSV.
                           Must be a single character. (default: ,)
"#;

use std::{collections::HashMap, path::Path};

use rand::{SeedableRng, rngs::StdRng};
use serde::Deserialize;

use crate::{
    CliError, CliResult,
    cmd::describegpt::dictionary::{FrequencyRecord, parse_frequency_csv, parse_stats_csv},
    config::{Config, Delimiter},
    util,
};

mod dictionary;
mod faker_map;
mod generator;

use faker_map::Locale;
use generator::ColumnGenerator;

#[derive(Deserialize)]
struct Args {
    arg_input:               String,
    flag_dictionary:         Option<String>,
    flag_infer_content_type: bool,
    flag_rows:               u64,
    flag_seed:               Option<u64>,
    flag_locale:             String,
    flag_freq_limit:         usize,
    flag_stats_options:      Option<String>,
    flag_jobs:               Option<usize>,
    flag_output:             Option<String>,
    flag_delimiter:          Option<Delimiter>,
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

    let input_path = args.arg_input.as_str();
    if !Path::new(input_path).is_file() {
        return fail_clierror!("Input file '{input_path}' does not exist or is not a file.");
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
    let (stats_records, _addl_cols) = parse_stats_csv(&stats_csv)?;
    let frequency_records = parse_frequency_csv(&freq_csv)?;

    if stats_records.is_empty() {
        return fail_clierror!("No columns found in '{input_path}'.");
    }

    // --- 3. Dictionary (optional) → field-name -> content_type map ---------
    let content_types: HashMap<String, String> = if let Some(ref dict_path) = args.flag_dictionary {
        dictionary::load_content_types(dict_path)?
    } else if args.flag_infer_content_type {
        dictionary::infer_content_types(input_path)?
    } else {
        HashMap::new()
    };

    // --- 4. Build one ColumnGenerator per column ---------------------------
    let mut freq_by_field: HashMap<&str, Vec<&FrequencyRecord>> = HashMap::new();
    for freq_record in &frequency_records {
        freq_by_field
            .entry(freq_record.field.as_str())
            .or_default()
            .push(freq_record);
    }

    let generators: Vec<ColumnGenerator> = stats_records
        .iter()
        .map(|stats_record| {
            let freqs = freq_by_field
                .get(stats_record.field.as_str())
                .map(Vec::as_slice)
                .unwrap_or_default();
            let content_type = content_types
                .get(&stats_record.field)
                .map_or("unknown", String::as_str);
            ColumnGenerator::build(
                stats_record,
                freqs,
                content_type,
                total_rows,
                args.flag_rows,
                locale,
                &mut rng,
            )
        })
        .collect();

    // --- 5. Emit -----------------------------------------------------------
    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;

    let header: csv::StringRecord = stats_records
        .iter()
        .map(|stats_record| stats_record.field.clone())
        .collect();
    wtr.write_record(&header)?;

    let mut record = csv::StringRecord::new();
    for _ in 0..args.flag_rows {
        record.clear();
        for generator in &generators {
            record.push_field(&generator.next(&mut rng));
        }
        wtr.write_record(&record)?;
    }

    Ok(wtr.flush()?)
}
