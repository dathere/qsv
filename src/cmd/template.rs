static USAGE: &str = r#"
Renders a template using CSV data with the MiniJinja template engine.
https://docs.rs/minijinja/latest/minijinja/

This command processes each row of the CSV file, making the column values available as variables.
Each row is rendered using the template. Column headers become variable names, with non-alphanumeric
characters converted to underscore (_).

Templates use Jinja2 syntax (https://jinja.palletsprojects.com/en/stable/templates/)
and can access an extensive library of built-in filters/functions, with additional ones
from minijinja_contrib https://docs.rs/minijinja-contrib/latest/minijinja_contrib/.
Additional qsv custom filters are also documented at the end of this file.

If the <outdir> argument is specified, it will create a file for each row in <outdir>, with
the filename rendered using --outfilename option.
Otherwise, ALL the rendered rows will be sent to STDOUT or the designated --output.

Example:

data.csv
```csv
"first name","last name",balance,"loyalty points",active,us_state
alice,jones,100.50,1000,true,TX
bob,smith,200.75,2000,false,CA
john,doe,10,1,true,NJ
```

template.tpl
```jinja
{% set us_state_lookup_loaded = register_lookup("us_states", "dathere://us-states-example.csv") -%}
Dear {{ first_name|title }} {{ last_name|title }}!
Your account balance is {{ balance|format_float(2) }}
    with {{ loyalty_points|human_count }} point{{ loyalty_points|int|pluralize }}!
{# This is a comment and will not be rendered. The closing minus sign in this
    block tells MiniJinja to trim whitespaces -#}
{% if us_state_lookup_loaded -%}
    {% if us_state not in ["DE", "CA"] -%}
        {% set tax_rate = us_state|lookup("us_states", "Sales Tax (2023)")|float -%}
        State: {{ us_state|lookup("us_states", "Name") }} {{us_state}} Tax Rate: {{ tax_rate }}%
        {% set loyalty_value = loyalty_points|int / 100 -%}
        {%- set tax_amount = loyalty_value * (tax_rate / 100) -%}
        {%- set loyalty_value = loyalty_value - tax_amount -%}
        Value of Points: {{ loyalty_value }}
    {% else %}
        {% set loyalty_value = 0 -%}
    {% endif %}
    Final Balance: {{ (balance|int - loyalty_value)|format_float(2) }}
{% endif %}
Status: {% if active|to_bool %}Active{% else %}Inactive{% endif %}
```

  $ qsv template --template-file template.tpl data.csv

> [!NOTE]
> All variables are of type String and will need to be cast with the `|float` or `|int`
>  filters for math operations and when a MiniJinja filter/function requires it.
> qsv's custom filters (substr, format_float, human_count, human_float_count, round_banker &
> str_to_bool) do not require casting for convenience.

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_template.rs.
For a relatively complex MiniJinja template, see https://github.com/dathere/qsv/blob/master/scripts/template.tpl

Usage:
    qsv template [options] [--template <str> | --template-file <file>] [<input>] [<outdir> | --output <file>]
    qsv template --help

template arguments:
    <input>                     The CSV file to read. If not given, input is read from STDIN.
    <outdir>                    The directory where the output files will be written.
                                If it does not exist, it will be created.
                                If not set, output will be sent to stdout or the specified --output.
                                When writing to <outdir>, files are organized into subdirectories
                                of --outsubdir-size (default: 1000) files each to avoid filesystem
                                navigation & performance issues.
                                For example, with 3500 records:
                                  * <outdir>/0000/0001.txt through <outdir>/0000/1000.txt
                                  * <outdir>/0001/1001.txt through <outdir>/0001/2000.txt
                                  * <outdir>/0002/2001.txt through <outdir>/0002/3000.txt
                                  * <outdir>/0003/3001.txt through <outdir>/0003/4000.txt
template options:
    --template <str>            MiniJinja template string to use (alternative to --template-file)
    -t, --template-file <file>  MiniJinja template file to use
    -J, --globals-json <file>   A JSON file containing global variables to make available in templates.
                                The JSON properties can be accessed in templates using the "qsv_g"
                                namespace (e.g. {{qsv_g.school_name}}, {{qsv_g.year}}).
                                This allows sharing common values across all template renders.
    --outfilename <str>         MiniJinja template string to use to create the filename of the output
                                files to write to <outdir>. If set to just QSV_ROWNO, the filestem
                                is set to the current rowno of the record, padded with leading
                                zeroes, with the ".txt" extension (e.g. 001.txt, 002.txt, etc.)
                                Note that all the fields, including QSV_ROWNO, are available
                                when defining the filename template.
                                [default: QSV_ROWNO]
    --outsubdir-size <num>      The number of files per subdirectory in <outdir>.
                                [default: 1000]
    --customfilter-error <msg>  The value to return when a custom filter returns an error.
                                Use "<empty string>" to return an empty string.
                                [default: <FILTER_ERROR>]
    -j, --jobs <arg>            The number of jobs to run in parallel.
                                When not set, the number of jobs is set to the number of CPUs detected.
    -b, --batch <size>          The number of rows per batch to load into memory, before running in parallel.
                                Set to 0 to load all rows in one batch.
                                [default: 50000]
    --timeout <seconds>        Timeout for downloading lookups on URLs. [default: 30]
    --cache-dir <dir>          The directory to use for caching downloaded lookup resources.
                               If the directory does not exist, qsv will attempt to create it.
                               If the QSV_CACHE_DIR envvar is set, it will be used instead.
                               [default: ~/.qsv-cache]
    --ckan-api <url>           The URL of the CKAN API to use for downloading lookup resources
                               with the "ckan://" scheme.
                               If the QSV_CKAN_API envvar is set, it will be used instead.
                               [default: https://data.dathere.com/api/3/action]
    --ckan-token <token>       The CKAN API token to use. Only required if downloading private resources.
                               If the QSV_CKAN_TOKEN envvar is set, it will be used instead.

Common options:
    -h, --help                  Display this message
    -o, --output <file>         Write output to <file> instead of stdout
    -n, --no-headers            When set, the first row will not be interpreted
                                as headers. Templates must use numeric 1-based indices
                                with the "_c" prefix. (e.g. col1: {{_c1}} col2: {{_c2}})
    --delimiter <sep>           Field separator for reading CSV [default: ,]
    -p, --progressbar           Show progress bars. Not valid for stdin.
"#;

use std::{
    fmt::Write as _,
    fs,
    io::{BufWriter, Write},
    path::PathBuf,
    sync::{
        OnceLock, RwLock,
        atomic::{AtomicBool, AtomicU16, AtomicU64, Ordering},
    },
};

use foldhash::{HashMap, HashMapExt};
#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{ProgressBar, ProgressDrawTarget};
use minijinja::{Environment, Value, value::ValueKind};
use minijinja_contrib::pycompat::unknown_method_callback;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::IntoParallelRefIterator,
};
use serde::Deserialize;
use simd_json::{BorrowedValue, json};

use crate::{
    CliError, CliResult,
    config::{Config, DEFAULT_WTR_BUFFER_CAPACITY, Delimiter},
    lookup,
    lookup::LookupTableOptions,
    util,
};

const QSV_ROWNO: &str = "QSV_ROWNO";

#[derive(Deserialize)]
struct Args {
    arg_input:               Option<String>,
    arg_outdir:              Option<String>,
    flag_template:           Option<String>,
    flag_template_file:      Option<PathBuf>,
    flag_globals_json:       Option<PathBuf>,
    flag_output:             Option<String>,
    flag_outfilename:        String,
    flag_outsubdir_size:     u16,
    flag_customfilter_error: String,
    flag_jobs:               Option<usize>,
    flag_batch:              usize,
    flag_delimiter:          Option<Delimiter>,
    flag_no_headers:         bool,
    #[allow(dead_code)]
    flag_progressbar:        bool,
    flag_timeout:            u16,
    flag_cache_dir:          String,
    flag_ckan_api:           String,
    flag_ckan_token:         Option<String>,
}

static FILTER_ERROR: OnceLock<String> = OnceLock::new();
static EMPTY_FILTER_ERROR: AtomicBool = AtomicBool::new(false);
// Counts per-row template render failures across the parallel batch loop.
// We still write the error string into each failing row's output so users can
// grep it; this counter exists so the command can surface a single summary
// to stderr at the end instead of failing silently.
static RENDER_ERROR_COUNT: AtomicU64 = AtomicU64::new(0);

impl From<minijinja::Error> for CliError {
    fn from(err: minijinja::Error) -> CliError {
        CliError::Other(err.to_string())
    }
}

// An efficient structure for lookups.
// `rows`: maps the (normalized) key column value -> field name -> field value.
// `lowercased_keys`: maps lowercased key -> original key, populated at registration
// time so case-insensitive lookups stay O(1) instead of O(N) per call.
struct LookupTable {
    rows:            HashMap<String, HashMap<String, String>>,
    lowercased_keys: HashMap<String, String>,
}
type LookupMap = HashMap<String, LookupTable>;

static LOOKUP_MAP: OnceLock<RwLock<LookupMap>> = OnceLock::new();

static QSV_CACHE_DIR: OnceLock<String> = OnceLock::new();
static TIMEOUT_SECS: AtomicU16 = AtomicU16::new(30);
static CKAN_API: OnceLock<String> = OnceLock::new();
static CKAN_TOKEN: OnceLock<Option<String>> = OnceLock::new();
static DELIMITER: OnceLock<Option<Delimiter>> = OnceLock::new();

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Reset the process-wide render-error counter so a second invocation in the
    // same process (tests, embedding, future REPL/MCP path) doesn't carry the
    // previous run's count into the end-of-run summary.
    RENDER_ERROR_COUNT.store(0, Ordering::Relaxed);

    // Get template content
    let template_content = match (args.flag_template_file, args.flag_template) {
        (Some(path), None) => fs::read_to_string(path)?,
        (None, Some(template)) => template,
        _ => {
            return fail_incorrectusage_clierror!(
                "Must provide either --template or --template-file"
            );
        },
    };

    // Initialize FILTER_ERROR from args.flag_customfilter_error
    if FILTER_ERROR
        .set(if args.flag_customfilter_error == "<empty string>" {
            EMPTY_FILTER_ERROR.store(true, Ordering::Relaxed);
            String::new()
        } else {
            args.flag_customfilter_error
        })
        .is_err()
    {
        return fail!("Cannot initialize custom filter error message.");
    }

    TIMEOUT_SECS.store(
        util::timeout_secs(args.flag_timeout)? as u16,
        Ordering::Relaxed,
    );

    // setup globals JSON context if specified
    let mut globals_flag = false;
    let globals_ctx = if let Some(globals_json) = args.flag_globals_json {
        globals_flag = true;
        match std::fs::read(globals_json) {
            Ok(mut bytes) => match simd_json::from_slice(&mut bytes) {
                Ok(json) => json,
                Err(e) => return fail_clierror!("Failed to parse globals JSON file: {e}"),
            },
            Err(e) => return fail_clierror!("Failed to read globals JSON file: {e}"),
        }
    } else {
        json!("")
    };

    let globals_ctx_borrowed: simd_json::BorrowedValue = globals_ctx.into();

    // Set up minijinja environment with qsv's custom functions/filters
    // see https://docs.rs/minijinja-contrib/latest/minijinja_contrib/
    let mut env = Environment::new();
    register_qsv_extensions(&mut env);
    env.add_template("template", &template_content)?;
    let template = env.get_template("template")?;

    // Set up CSV reader
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);
    let mut rdr = rconfig.reader()?;

    // Get width of rowcount for padding leading zeroes and early return
    // For stdin, we can't pre-count rows as it would consume the stream
    let (rowcount, width) = if rconfig.is_stdin() {
        // For stdin, we'll use a reasonable default width and skip early return check
        (0, 6) // Default width of 6 for up to 999,999 rows
    } else {
        let rowcount = util::count_rows(&rconfig)?;
        if rconfig.no_headers && rowcount == 1 {
            return Ok(());
        }
        let width = rowcount.to_string().len();
        (rowcount, width)
    };

    // read headers - the headers are used as MiniJinja variables in the template
    let headers = if args.flag_no_headers {
        csv::StringRecord::new()
    } else {
        let headers = rdr.headers()?.clone();
        let mut sanitized_headers: Vec<String> = headers
            .iter()
            .map(|h| {
                h.chars()
                    .map(|c| if c.is_alphanumeric() { c } else { '_' })
                    .collect()
            })
            .collect();
        // add a column named QSV_ROWNO at the end
        sanitized_headers.push(QSV_ROWNO.to_owned());
        csv::StringRecord::from(sanitized_headers)
    };
    let headers_len = headers.len();

    // Set up output handling
    let output_to_dir = args.arg_outdir.is_some();

    // Reject --outsubdir-size 0 up front. The bucket index calculation downstream
    // is `(global_row - 1) / outsubdir_numfiles`, which would otherwise panic on
    // divide-by-zero the first time we try to place a file in a subdirectory.
    if output_to_dir && args.flag_outsubdir_size == 0 {
        return fail_incorrectusage_clierror!("--outsubdir-size must be greater than 0");
    }
    let mut row_no = 0_u64;

    let use_rowno_filename = args.flag_outfilename == QSV_ROWNO;

    // Create filename template once if needed
    #[allow(unused_assignments)]
    let mut filename_env = Environment::empty();
    let filename_template = if output_to_dir && !use_rowno_filename {
        // actually init the MiniJinja environment with default filters, tests and globals loaded
        filename_env = Environment::new();

        minijinja_contrib::add_to_environment(&mut filename_env);
        filename_env.set_unknown_method_callback(unknown_method_callback);
        filename_env.add_template("filename", &args.flag_outfilename)?;
        filename_env.get_template("filename")?
    } else {
        filename_env.template_from_str("")?
    };

    let mut bulk_wtr = if output_to_dir {
        fs::create_dir_all(args.arg_outdir.as_ref().unwrap())?;
        None
    } else {
        Some(open_bulk_writer(args.flag_output.as_deref())?)
    };

    let num_jobs = util::njobs(args.flag_jobs);
    let batchsize = util::optimal_batch_size(&rconfig, args.flag_batch, num_jobs);

    // prep progress bar
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        util::prep_progress(&progress, rowcount);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    init_lookup_globals(
        args.flag_delimiter,
        &args.flag_cache_dir,
        &args.flag_ckan_api,
        args.flag_ckan_token.as_deref(),
    )?;

    pre_register_lookups(&env, &template_content)?;

    // reuse batch buffers
    #[allow(unused_assignments)]
    let mut batch_record = csv::StringRecord::new();
    let mut batch: Vec<csv::StringRecord> = Vec::with_capacity(batchsize);
    // batch_results stores the results of template rendering for each batch:
    // - First tuple element is the optional output filename (when writing to directory)
    // - Second tuple element is the rendered template content
    let mut batch_results: Vec<(Option<String>, String)> = Vec::with_capacity(batchsize);

    // Track current subdirectory across batches so we don't recreate it at every
    // batch boundary; subdir numbering is global (based on row number), not batch-local.
    let mut outpath = std::path::PathBuf::new();
    let mut current_subdir: Option<usize> = None;
    let outsubdir_numfiles = args.flag_outsubdir_size as usize;

    // Pad subdir names to the width of the highest subdir we'll produce, not to
    // the width of the highest row number — otherwise a 60k-row run with
    // --outsubdir-size 5000 would name subdirs "00000".."00011" (5 digits)
    // when "00".."11" suffices. For stdin we don't know rowcount, so fall back
    // to the rowcount-derived width as before. Guarded by `output_to_dir` so
    // that `--outsubdir-size 0` without an `<outdir>` (where the flag is
    // irrelevant) doesn't trip a divide-by-zero — the validator above only
    // rejects 0 when output_to_dir is true.
    let subdir_width = if output_to_dir && rowcount > 0 {
        let max_subdir = (rowcount - 1) / outsubdir_numfiles as u64;
        max_subdir.to_string().len().max(1)
    } else {
        width
    };

    let no_headers = args.flag_no_headers;

    // main loop to read CSV and construct batches for parallel processing.
    // each batch is processed via Rayon parallel iterator.
    // loop exits when batch is empty.
    'batch_loop: loop {
        for _ in 0..batchsize {
            match rdr.read_record(&mut batch_record) {
                Ok(has_data) => {
                    if has_data {
                        row_no += 1;
                        batch_record.push_field(itoa::Buffer::new().format(row_no));
                        batch.push(std::mem::take(&mut batch_record));
                    } else {
                        // nothing else to add to batch
                        break;
                    }
                },
                Err(e) => {
                    return fail_clierror!("Error reading file: {e}");
                },
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break 'batch_loop;
        }

        // do actual template rendering via Rayon parallel iterator
        batch
            .par_iter()
            .with_min_len(1024)
            .map(|record| {
                let curr_record = record;

                let mut context = simd_json::borrowed::Object::default();
                // Add globals context to the record context
                if globals_flag {
                    context.insert(
                        std::borrow::Cow::Borrowed("qsv_g"),
                        globals_ctx_borrowed.clone(),
                    );
                }
                context.reserve(headers_len);
                let mut row_number = 0_u64;

                // add the fields of the current record to the context
                if no_headers {
                    // Use numeric, column 1-based indices (e.g. _c1, _c2, etc.)
                    let headers_len = curr_record.len();

                    for (i, field) in curr_record.iter().enumerate() {
                        if i == headers_len - 1 {
                            // set the last field to QSV_ROWNO
                            // The QSV_ROWNO field was just appended to the record by the
                            // producer above using itoa, so it is always a valid u64.
                            row_number = atoi_simd::parse::<u64, false, false>(field.as_bytes())
                                .expect("QSV_ROWNO is set by the batch producer");
                            context.insert(
                                std::borrow::Cow::Borrowed(QSV_ROWNO),
                                BorrowedValue::String(std::borrow::Cow::Borrowed(field)),
                            );
                        } else {
                            context.insert(
                                format!("_c{}", i + 1).into(),
                                BorrowedValue::String(std::borrow::Cow::Borrowed(field)),
                            );
                        }
                    }
                } else {
                    // Use header names as template variables
                    for (header, field) in headers.iter().zip(curr_record.iter()) {
                        context.insert(
                            std::borrow::Cow::Borrowed(header),
                            BorrowedValue::String(std::borrow::Cow::Borrowed(field)),
                        );
                        // when headers are defined, the last one is QSV_ROWNO
                        if header == QSV_ROWNO {
                            // The QSV_ROWNO field was just appended to the record by the
                            // producer above using itoa, so it is always a valid u64.
                            row_number = atoi_simd::parse::<u64, false, false>(field.as_bytes())
                                .expect("QSV_ROWNO is set by the batch producer");
                        }
                    }
                }

                let rendered = match template.render(&context) {
                    Ok(s) => s,
                    Err(e) => {
                        RENDER_ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
                        format!("RENDERING ERROR ({row_number}): {e}\n")
                    },
                };

                if output_to_dir {
                    let outfilename = if use_rowno_filename {
                        // Pad row number with required number of leading zeroes
                        format!("{row_number:0width$}.txt")
                    } else {
                        // render filename with record data using context
                        // if the filename cannot be rendered, set the filename so the user
                        // can easily find the record which caused the rendering error
                        // e.g. FILENAME_RENDERING_ERROR-00035.txt is the 35th record in a CSV
                        // with at least 10000 rows (the three leading zeros)
                        filename_template.render(&context).unwrap_or_else(|_| {
                            format!("FILENAME_RENDERING_ERROR-{row_number:0width$}.txt")
                        })
                    };
                    (Some(outfilename), rendered)
                } else {
                    (None, rendered)
                }
            })
            .collect_into_vec(&mut batch_results);

        // First row number in this batch (1-based). Subdir numbering is computed from
        // this global row number so subdirs stay correct across batch boundaries.
        let batch_start_row = row_no - batch.len() as u64 + 1;

        for (idx, result_record) in batch_results.iter().enumerate() {
            if output_to_dir {
                // safety: this is safe as output_to_dir = args.arg_outdir.is_some()
                // and result_record.0 (the filename to use) is_some()
                outpath.push(args.arg_outdir.as_ref().unwrap());

                // Create subdirectory for every outsubdir_size files
                // to make it easier to handle & navigate generated files
                // particularly, if we're using a large input CSV
                let global_row = batch_start_row + idx as u64;
                let subdir_num = ((global_row - 1) / outsubdir_numfiles as u64) as usize;

                if current_subdir == Some(subdir_num) {
                    outpath.push(format!("{subdir_num:0subdir_width$}"));
                } else {
                    // Only create new subdir when the bucket changes
                    let subdir_name = format!("{subdir_num:0subdir_width$}");
                    outpath.push(&subdir_name);

                    // create_dir_all is idempotent and tolerates the dir already
                    // existing (e.g. from a prior batch that ended mid-bucket).
                    fs::create_dir_all(&outpath)?;
                    current_subdir = Some(subdir_num);
                }

                outpath.push(result_record.0.as_deref().unwrap());

                // One file per row in the hot loop. fs::write is a single create+
                // write+close syscall sequence with no intermediate buffering, which
                // matches what the previous BufWriter-sized-to-payload was doing.
                fs::write(&outpath, result_record.1.as_bytes())?;

                outpath.clear();
            } else if let Some(ref mut w) = bulk_wtr {
                w.write_all(result_record.1.as_bytes())?;
            }
        }

        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.inc(batch.len() as u64);
        }

        batch.clear();
    } // end batch loop

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        util::finish_progress(&progress);
    }

    if let Some(mut w) = bulk_wtr {
        w.flush()?;
    }

    // If any rows failed to render, the per-row error message was already written
    // into the output (file or stdout) so the user can grep for it. Surface a
    // single summary line on stderr so the failure isn't invisible to a CI job
    // or cron task that only inspects the exit code (still 0) or the stderr tail.
    let render_errors = RENDER_ERROR_COUNT.load(Ordering::Relaxed);
    if render_errors > 0 {
        eprintln!(
            "qsv template: {render_errors} row(s) failed to render; see \"RENDERING ERROR \
             (rowno): ...\" entries in the output."
        );
    }

    Ok(())
}

// Initialize the file-static OnceLocks that the lookup machinery reads from
// inside the parallel render closure (DELIMITER, QSV_CACHE_DIR, CKAN_API,
// CKAN_TOKEN). The CKAN_* env vars take precedence over the corresponding
// CLI flags. Note: these statics are process-global and only safe to set once
// per process — see RENDER_ERROR_COUNT in run() for the related re-entry caveat.
fn init_lookup_globals(
    flag_delimiter: Option<Delimiter>,
    flag_cache_dir: &str,
    flag_ckan_api: &str,
    flag_ckan_token: Option<&str>,
) -> CliResult<()> {
    // safety: flag_delimiter has a docopt default
    DELIMITER.set(flag_delimiter).unwrap();

    let qsv_cache_dir = lookup::set_qsv_cache_dir(flag_cache_dir)?;
    QSV_CACHE_DIR.set(qsv_cache_dir)?;

    CKAN_API.set(std::env::var("QSV_CKAN_API").unwrap_or_else(|_| flag_ckan_api.to_owned()))?;

    CKAN_TOKEN
        .set(
            std::env::var("QSV_CKAN_TOKEN")
                .ok()
                .or_else(|| flag_ckan_token.map(str::to_owned)),
        )
        .unwrap();

    Ok(())
}

// Pre-register every register_lookup() call we can find in the template body
// before the batch loop starts.
//
// Trade-off: this pre-scan runs every register_lookup() call we can find, even
// if the call is wrapped in a conditional like `{% if cond %}{% set _ =
// register_lookup(...) %}{% endif %}` that the per-row render would skip.
// We accept that — registering an unused lookup is wasted work (and a possible
// CSV/HTTP/CKAN fetch), but doing it up front means a malformed URL or
// unreachable host fails the command at startup instead of on the first row
// that triggers the conditional.
fn pre_register_lookups(env: &Environment, template_content: &str) -> CliResult<()> {
    if !template_content.contains("register_lookup(") {
        return Ok(());
    }

    // NOTE: this regex is a deliberately-loose textual scan, not a parser.
    // It does not understand nested parens, string literals, or {# #} comments.
    // The cost of a false-positive match is a clearer startup-time error;
    // a false-negative just means the registration is deferred to the first
    // row's render. So perfect parsing isn't required.
    let re = regex::Regex::new(r"register_lookup\([^)]+\)")?;

    // Wrap each call in a conditional that emits a single-line error if the
    // call returns false / is missing.
    // safety: write! into a String never fails.
    let temp_template = re
        .find_iter(template_content)
        .fold(String::new(), |mut acc, cap| {
            write!(
                acc,
                r#"{{% if not {cap_str} %}}LOOKUP REGISTRATION ERROR: "{cap_str}"\n{{% endif %}}"#,
                cap_str = cap.as_str(),
            )
            .unwrap();
            acc
        });

    // Render with an empty context against a clone of the real env so the
    // registered functions/filters are visible.
    let temp_env = env.clone();
    match temp_env.render_str(&temp_template, minijinja::context! {}) {
        Ok(s) => {
            if !s.is_empty() {
                return fail_incorrectusage_clierror!("{s}");
            }
            Ok(())
        },
        Err(e) => fail_incorrectusage_clierror!("{e}"),
    }
}

// Open the bulk writer used when output is NOT going to a per-row directory.
// One destination, lots of small writes — use a fat BufWriter so we minimize
// the number of write() syscalls.
fn open_bulk_writer(output_path: Option<&str>) -> CliResult<Box<dyn Write>> {
    Ok(match output_path {
        Some(file) => Box::new(BufWriter::with_capacity(
            DEFAULT_WTR_BUFFER_CAPACITY,
            fs::File::create(file)?,
        )),
        None => Box::new(BufWriter::with_capacity(
            DEFAULT_WTR_BUFFER_CAPACITY,
            std::io::stdout(),
        )),
    })
}

// Configure a MiniJinja environment with the minijinja_contrib batteries
// plus all qsv-specific functions and filters used by `qsv template`. The
// filename-template environment intentionally does NOT call this — filename
// templates are limited to the minijinja_contrib filter set so that a typo
// like `|lookup` in --outfilename surfaces as an "unknown filter" error.
fn register_qsv_extensions(env: &mut Environment) {
    minijinja_contrib::add_to_environment(env);
    env.set_unknown_method_callback(unknown_method_callback);

    env.add_function("register_lookup", register_lookup);

    env.add_filter("substr", substr);
    env.add_filter("format_float", format_float);
    env.add_filter("human_count", human_count);
    env.add_filter("human_float_count", human_float_count);
    env.add_filter("round_banker", round_banker);
    env.add_filter("to_bool", to_bool);
    env.add_filter("lookup", lookup_filter);
}

// Normalize a string into the canonical key form used by both register_lookup
// and the lookup filter, so the two sides always agree on equality.
//
// The CSV-side key is trimmed and, if numeric, re-emitted via itoa (i64) or
// zmij (f64) so that "42", " 42 ", and "42.0" all collapse to the same key.
// The filter-side input goes through this same function so a template author
// can pass `" 42 "`, `"42"`, or the integer `42` and hit the same row.
fn normalize_lookup_key(s: &str) -> std::borrow::Cow<'_, str> {
    use std::borrow::Cow;
    let trimmed = s.trim();
    if let Ok(num) = trimmed.parse::<i64>() {
        Cow::Owned(itoa::Buffer::new().format(num).to_owned())
    } else if let Ok(num) = trimmed.parse::<f64>() {
        // Collapse whole-number floats into the i64 form so "42" and "42.0"
        // (and the integer 42 from a template) all hash to the same key.
        // zmij would otherwise preserve "42.0".
        //
        // Use an exclusive upper bound: i64::MAX (2^63-1) is not exactly
        // representable as f64; the cast rounds up to 2^63, which then
        // saturates back to i64::MAX when re-cast. Excluding the boundary
        // sends 2^63 through zmij instead of producing a misleading key.
        #[allow(clippy::cast_precision_loss)]
        if num.is_finite() && num.fract() == 0.0 && num >= i64::MIN as f64 && num < i64::MAX as f64
        {
            Cow::Owned(itoa::Buffer::new().format(num as i64).to_owned())
        } else {
            Cow::Owned(zmij::Buffer::new().format(num).to_owned())
        }
    } else {
        // Non-numeric: borrow the trimmed slice directly so already-canonical
        // string keys (the common hot-path case) don't allocate.
        Cow::Borrowed(trimmed)
    }
}

// CUSTOM MINIJINJA FILTERS =========================================
// safety: for all FILTER_ERROR.gets, safe to unwrap as FILTER_ERROR
// is initialized on startup

/// Returns a substring of the input string from start index to end index (exclusive).
/// If end is not provided, returns substring from start to end of string.
/// Returns --customfilter-error (default: <FILTER_ERROR>) if indices are invalid.
///
/// NOTE: indices are byte offsets, not character offsets. For ASCII input this is
/// equivalent. For non-ASCII (UTF-8) input, an index that lands inside a multi-byte
/// codepoint is invalid and the filter will return --customfilter-error.
fn substr(value: &str, start: u32, end: Option<u32>) -> String {
    let end = end.unwrap_or(value.len() as _);
    if let Some(s) = value.get(start as usize..end as usize) {
        s.into()
    } else {
        FILTER_ERROR.get().unwrap().clone()
    }
}

/// Formats a float number string with the specified decimal precision.
/// Returns --customfilter-error (default: <FILTER_ERROR>) if input cannot be parsed as float.
fn format_float(value: &Value, precision: u32) -> String {
    // Prevent excessive precision
    let precision = precision.min(16) as usize;
    if value.kind() == ValueKind::String {
        if let Some(s) = value.as_str() {
            s.parse::<f64>().map_or_else(
                |_| FILTER_ERROR.get().unwrap().clone(),
                |num| format!("{num:.precision$}"),
            )
        } else if EMPTY_FILTER_ERROR.load(Ordering::Relaxed) {
            FILTER_ERROR.get().unwrap().clone()
        } else {
            format!(
                r#"{}: "{value}" is not a float."#,
                FILTER_ERROR.get().unwrap()
            )
        }
    } else {
        FILTER_ERROR.get().unwrap().clone()
    }
}

/// Formats an integer with thousands separators (e.g. "1,234,567").
/// Returns --customfilter-error (default: <FILTER_ERROR>) if input cannot be parsed as integer.
fn human_count(value: &Value) -> String {
    if value.kind() == ValueKind::String {
        let s = value.as_str().unwrap().as_bytes();
        atoi_simd::parse::<u64, false, false>(s).map_or_else(
            |_| {
                if EMPTY_FILTER_ERROR.load(Ordering::Relaxed) {
                    FILTER_ERROR.get().unwrap().clone()
                } else {
                    format!(
                        r#"{}: "{value}" is not an integer."#,
                        FILTER_ERROR.get().unwrap()
                    )
                }
            },
            |num| indicatif::HumanCount(num).to_string(),
        )
    } else {
        FILTER_ERROR.get().unwrap().clone()
    }
}

/// Formats a float number with thousands separators (e.g. "1,234,567.89").
/// Returns --customfilter-error (default: <FILTER_ERROR>) if input cannot be parsed as float.
fn human_float_count(value: &Value) -> String {
    if value.kind() == ValueKind::String {
        let s = value.as_str().unwrap();
        s.parse::<f64>().map_or_else(
            |_| {
                if EMPTY_FILTER_ERROR.load(Ordering::Relaxed) {
                    FILTER_ERROR.get().unwrap().clone()
                } else {
                    format!(
                        r#"{}: "{value}" is not a float."#,
                        FILTER_ERROR.get().unwrap()
                    )
                }
            },
            |num| indicatif::HumanFloatCount(num).to_string(),
        )
    } else {
        FILTER_ERROR.get().unwrap().clone()
    }
}

/// Rounds a float number to specified number of decimal places.
/// Round using Midpoint Nearest Even Rounding Strategy AKA "Bankers Rounding."
/// automatically trims trailing zeros
/// https://docs.rs/rust_decimal/latest/rust_decimal/enum.RoundingStrategy.html#variant.MidpointNearestEven
/// Returns --customfilter-error (default: <FILTER_ERROR>) if input cannot be parsed as float.
fn round_banker(value: &Value, places: u32) -> String {
    if value.kind() == ValueKind::String {
        let s = value.as_str().unwrap();
        s.parse::<f64>().map_or_else(
            |_| {
                if EMPTY_FILTER_ERROR.load(Ordering::Relaxed) {
                    FILTER_ERROR.get().unwrap().clone()
                } else {
                    format!(
                        r#"{}: "{value}" is not a float."#,
                        FILTER_ERROR.get().unwrap()
                    )
                }
            },
            |num| util::round_num(num, places),
        )
    } else {
        FILTER_ERROR.get().unwrap().clone()
    }
}

/// Converts boolean-like values to boolean.
/// Returns true for "true", "1", "yes", "t" or "y" (case insensitive).
/// Returns true for any integer value not equal to 0.
/// Returns true for all float values not equal to 0.0.
/// Returns the truthiness of all other values.
fn to_bool(value: &Value) -> bool {
    if value.kind() == ValueKind::String {
        let s = value.as_str().unwrap();
        let truthy = matches!(
            s.to_ascii_lowercase().as_str(),
            "true" | "1" | "yes" | "t" | "y"
        );
        if truthy {
            true
        } else if let Ok(num) = s.parse::<i64>() {
            num != 0
        } else if let Ok(num) = s.parse::<f64>() {
            num.abs() > f64::EPSILON
        } else {
            false
        }
    } else {
        value.is_true()
    }
}

/// Registers a lookup table for use with the lookup filter.
///
/// This function loads a CSV file as a lookup table and registers it in memory for use with
/// the lookup filter in templates. The lookup table is stored as a set of LookupEntry objects
/// containing key-value pairs from the CSV.
///
/// # Arguments
///
/// * `lookup_name` - Name to register the lookup table under
/// * `lookup_table_uri` - Path/URI to the CSV file (supports local files, HTTP(S), CKAN resources)
/// * `cache_age_secs` - Optional cache duration in seconds for remote files. Defaults to 3600 (1
///   hour). Set to 0 to disable caching.
///
/// # Returns
///
/// Returns `Ok(true)` if successful, or a `minijinja::Error` with details if registration fails.
///
/// # Example
///
/// ```text
/// {% set result = register_lookup('products', 'lookup.csv') %}
/// {% if result %}
///   {{ product_id|lookup('products', 'id', 'name') }}
/// {% else %}
///   Error: {{ result.err }}
/// {% endif %}
/// ```
fn register_lookup(
    lookup_name: &str,
    lookup_table_uri: &str,
    cache_age_secs: Option<i64>,
) -> Result<bool, minijinja::Error> {
    // Validate inputs
    if lookup_name.is_empty() {
        return Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "lookup name cannot be empty",
        ));
    }

    let cache_age_secs = cache_age_secs.unwrap_or(3600);

    // Check if lookup_name already exists in LOOKUP_MAP
    if let Some(lock) = LOOKUP_MAP.get()
        && let Ok(map) = lock.read()
        && map.contains_key(lookup_name)
        && cache_age_secs > 0
    {
        // Lookup table already registered
        return Ok(true);
    }

    if lookup_table_uri.is_empty() {
        return Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "lookup table URI cannot be empty",
        ));
    }

    let lookup_opts = LookupTableOptions {
        name: lookup_name.to_string(),
        uri: lookup_table_uri.to_string(),
        cache_dir: QSV_CACHE_DIR
            .get()
            .ok_or_else(|| {
                minijinja::Error::new(
                    minijinja::ErrorKind::InvalidOperation,
                    "cache directory not initialized",
                )
            })?
            .to_string(),
        cache_age_secs,
        delimiter: DELIMITER.get().copied().flatten(),
        ckan_api_url: CKAN_API.get().cloned(),
        ckan_token: CKAN_TOKEN.get().and_then(std::clone::Clone::clone),
        timeout_secs: TIMEOUT_SECS.load(Ordering::Relaxed),
    };

    let lookup_table = lookup::load_lookup_table(&lookup_opts).map_err(|e| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!(r#"failed to load lookup table "{}": {e}"#, lookup_opts.name),
        )
    })?;

    if lookup_table.rowcount == 0 {
        return Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "lookup table is empty",
        ));
    }

    let lookup_config = Config::new(Some(lookup_table.filepath.clone()).as_ref())
        .delimiter(lookup_opts.delimiter)
        .comment(Some(b'#'))
        .no_headers(false);

    let mut rdr = lookup_config.reader().map_err(|e| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!(
                r#"failed to read CSV file "{}": {e}"#,
                lookup_table.filepath
            ),
        )
    })?;

    // Create nested HashMaps for efficient lookups
    let mut lookup_data: HashMap<String, HashMap<String, String>> =
        HashMap::with_capacity(lookup_table.rowcount);
    let mut lowercased_keys: HashMap<String, String> =
        HashMap::with_capacity(lookup_table.rowcount);

    let row_len = lookup_table.headers.len();
    for record in rdr.records().flatten() {
        let mut row_data: HashMap<String, String> =
            HashMap::with_capacity_and_hasher(row_len, foldhash::fast::RandomState::default());

        // Store all fields for this row
        for (header, value) in lookup_table.headers.iter().zip(record.iter()) {
            row_data.insert(header.to_owned(), value.to_owned());
        }

        // Use the first column as the key by default
        if let Some(key_value) = record.get(0) {
            let key = normalize_lookup_key(key_value).into_owned();
            // Pre-build the lowercased -> original key index so case-insensitive
            // lookups can be O(1). On collision (e.g. "Foo" and "foo" are both keys)
            // the first-seen original wins, mirroring HashMap insert semantics.
            lowercased_keys
                .entry(key.to_lowercase())
                .or_insert_with(|| key.clone());
            lookup_data.insert(key, row_data);
        }
    }

    // Initialize LOOKUP_MAP if it's not instantiated
    if LOOKUP_MAP.get().is_none() && LOOKUP_MAP.set(RwLock::new(HashMap::new())).is_err() {
        return Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "failed to initialize lookup map",
        ));
    }

    // Safely get write access to the map
    match LOOKUP_MAP.get().unwrap().write() {
        Ok(mut map) => {
            map.insert(
                lookup_name.to_string(),
                LookupTable {
                    rows: lookup_data,
                    lowercased_keys,
                },
            );
            Ok(true)
        },
        Err(_) => Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "failed to acquire write lock on lookup map",
        )),
    }
}

/// A filter function for looking up values in a registered lookup table.
///
/// This function is used as a template filter to look up values from a previously registered
/// lookup table. It searches for a record in the lookup table where the first column
/// matches the input `value`, and returns the corresponding value from the `field` column.
///
/// # Arguments
///
/// * `value` - The value to look up in the lookup table
/// * `lookup_name` - The name of the registered lookup table to search in
/// * `field` - The column name in the lookup table whose value should be returned
/// * `case_sensitive` - Optional boolean to control case-sensitive matching (defaults to true)
///
/// # Returns
///
/// Returns a `Result` containing either:
/// - `Ok(String)` - The looked up value if found, or the configured error string if not found
/// - `Err(minijinja::Error)` - If any of the required parameters are empty strings
///
/// # Example
///
/// ```text
/// # Case-sensitive lookup (default)
/// {{ product_id|lookup('products', 'name') }}
/// # Case-insensitive lookup (supports Unicode)
/// {{ product_id|lookup('products', 'name', false) }}
/// ```
fn lookup_filter(
    value: &Value,
    lookup_name: &str,
    field: &str,
    case_sensitive: Option<bool>,
) -> Result<String, minijinja::Error> {
    if lookup_name.is_empty() {
        return Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "lookup name not provided",
        ));
    }

    if field.is_empty() {
        return Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "lookup field not provided",
        ));
    }

    let case_sensitive = case_sensitive.unwrap_or(true);

    // Stringify the filter input as a borrow against stack-resident itoa/zmij
    // buffers when possible, so no String is allocated for the common
    // already-canonical case. Numbers go through the same itoa/zmij form as
    // keys, so e.g. integer 42 and float 42.0 both stringify to "42".
    let mut itoa_buf = itoa::Buffer::new();
    let mut zmij_buf = zmij::Buffer::new();
    let raw: &str = match value.kind() {
        ValueKind::String => value.as_str().unwrap(),
        ValueKind::Number => {
            if value.is_integer() {
                itoa_buf.format(value.as_i64().unwrap())
            } else {
                let n: f64 = value
                    .clone()
                    .try_into()
                    .expect("Kind::Number should be integer or float");
                zmij_buf.format(n)
            }
        },
        _ => value.as_str().unwrap_or_default(),
    };
    // Normalize so both sides of the lookup agree (trim + numeric canonicalization).
    // Returns Cow::Borrowed for non-numeric inputs that are already trimmed.
    let normalized = normalize_lookup_key(raw);

    // safety: FILTER_ERROR was initialized in run section
    let filter_error = FILTER_ERROR.get().unwrap();

    Ok(LOOKUP_MAP
        .get()
        .and_then(|lock| lock.read().ok())
        .and_then(|map| {
            let table = map.get(lookup_name)?;
            // Find the matching row. Both branches are O(1): the case-insensitive
            // index is pre-built at register_lookup time.
            let row = if case_sensitive {
                table.rows.get(normalized.as_ref())?
            } else {
                let lowered = normalized.to_lowercase();
                let original = table.lowercased_keys.get(&lowered)?;
                table.rows.get(original)?
            };
            row.get(field).map(String::from)
        })
        .unwrap_or_else(|| {
            if filter_error.is_empty() {
                String::new()
            } else {
                // Report the user-supplied input (pre-normalization) so the
                // diagnostic matches what the template author wrote.
                format!(
                    r#"{filter_error} - lookup: "{lookup_name}-{field}" not found for: "{raw}""#
                )
            }
        }))
}
