static USAGE: &str = r#"
Remove duplicate rows from an arbitrarily large CSV/text file using a memory-mapped,
on-disk hash table.

Unlike the 'dedup' command, this command does not load the entire file into memory
to sort the CSV first before deduping it. 

This allows it to run in constant memory and the output will retain the input sort order.

This command has TWO modes of operation.

 * CSV MODE
   when --select is set, it dedupes based on the given column/s. See `qsv select --help`
   for select syntax details.
 * LINE MODE
   when --select is NOT set, it deduplicates any input text file (not just CSVs) on a
   line-by-line basis.

A duplicate count will be sent to <stderr>.

Usage:
    qsv extdedup [options] [<input>] [<output>]
    qsv extdedup --help

extdedup options:
    -s, --select <arg>         Select a subset of columns to dedup.
                               Note that the outputs will remain at the full width of the CSV.
                               If --select is NOT set, extdedup will work in LINE MODE, deduping
                               the input as a text file on a line-by-line basis.
    --no-output                Do not write deduplicated output to <output>.
                               Use this if you only want to know the duplicate count.
                               Applies to both CSV MODE and LINE MODE.
    -D, --dupes-output <file>  Write duplicates to <file>.
                               In CSV MODE, <file> is a valid CSV with the same columns as the
                               input plus a leading "dupe_rowno" column (1-based data row number).
                               In LINE MODE, <file> is NOT a valid CSV — each duplicate line is
                               prefixed by its 0-based file line index and a tab character.
    -H, --human-readable       Comma separate duplicate count.
    --memory-limit <arg>       The maximum amount of memory to buffer the on-disk hash table.
                               If less than 50, this is a percentage of total memory.
                               If more than 50, this is the memory in MB to allocate, capped
                               at 90 percent of total memory.
                               [default: 10]
    --temp-dir <arg>           Directory to store temporary hash table file.
                               If not specified, defaults to operating system temp directory.

Common options:
                               CSV MODE ONLY:
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. That is, it will be deduped with the rest
                               of the rows. Otherwise, the first row will always
                               appear as the header row in the output.
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character. (default: ,)

    -h, --help                 Display this message
    -q, --quiet                Do not print duplicate count to stderr.
"#;

use std::{
    fs,
    io::{self, BufRead, Write, stdin, stdout},
    path::PathBuf,
};

use indicatif::HumanCount;
use serde::Deserialize;
use sysinfo::System;

use crate::{
    CliResult, config,
    config::{Config, Delimiter},
    odhtcache,
    select::SelectColumns,
    util,
};

#[derive(Deserialize)]
struct Args {
    arg_input:           Option<String>,
    flag_select:         Option<SelectColumns>,
    arg_output:          Option<String>,
    flag_no_headers:     bool,
    flag_delimiter:      Option<Delimiter>,
    flag_no_output:      bool,
    flag_dupes_output:   Option<String>,
    flag_human_readable: bool,
    flag_memory_limit:   Option<u64>,
    flag_temp_dir:       Option<String>,
    flag_quiet:          bool,
}

const MEMORY_LIMITED_BUFFER: u64 = 100 * 1_000_000; // 100 MB

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Set the memory buffer size for the on-disk hash table based on --memory-limit
    // and system capabilities.
    let mem_limited_buffer_bytes = calculate_memory_limit(args.flag_memory_limit);
    log::info!("{mem_limited_buffer_bytes} bytes used for memory buffer for on-disk hash table...");

    let quiet = args.flag_quiet;
    let human_readable = args.flag_human_readable;

    let dupes_count = if args.flag_select.is_some() {
        dedup_csv(args, mem_limited_buffer_bytes)?
    } else {
        dedup_lines(args, mem_limited_buffer_bytes)?
    };

    if quiet {
        return Ok(());
    }

    eprintln!(
        "{}",
        if human_readable {
            HumanCount(dupes_count).to_string()
        } else {
            dupes_count.to_string()
        }
    );

    Ok(())
}

fn dedup_csv(args: Args, mem_limited_buffer: u64) -> Result<u64, crate::clitypes::CliError> {
    // run() only routes here when flag_select is Some; this destructure
    // documents that invariant without an unwrap.
    let Some(select) = args.flag_select else {
        unreachable!("dedup_csv called without --select");
    };
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers)
        .select(select);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(args.arg_output.as_ref()).writer()?;

    let headers = rdr.byte_headers()?.clone();

    // Only construct the dupes writer when an output path was given;
    // Config::new(None).writer() would otherwise target stdout and contend
    // with `wtr` on flush.
    let mut dupewtr = if args.flag_dupes_output.is_some() {
        let mut w = Config::new(args.flag_dupes_output.as_ref()).writer()?;
        let mut dupe_headers = csv::ByteRecord::new();
        dupe_headers.push_field(b"dupe_rowno");
        dupe_headers.extend(headers.iter());
        w.write_byte_record(&dupe_headers)?;
        Some(w)
    } else {
        None
    };

    let temp_dir = args.flag_temp_dir.map(PathBuf::from);
    let mut dedup_cache = odhtcache::ExtDedupCache::new(mem_limited_buffer, temp_dir);
    let mut dupes_count = 0_u64;
    let sel = rconfig.selection(&headers)?;

    let no_output = args.flag_no_output;
    if !no_output {
        rconfig.write_headers(&mut rdr, &mut wtr)?;
    }

    // Pre-allocate and reuse the key buffer. A US (Unit Separator, \x1F)
    // byte separates selected fields so rows ("a","bc") and ("ab","c")
    // produce distinct keys ("a\x1Fbc" vs "ab\x1Fc"); without it both rows
    // collapse to "abc" and the second is silently treated as a duplicate.
    let mut key = String::with_capacity(256);
    let mut dupe_row = csv::ByteRecord::new();

    for (row_idx, row) in rdr.byte_records().enumerate() {
        let curr_row = row?;
        key.clear();
        let mut first = true;
        for field in sel.select(&curr_row) {
            if first {
                first = false;
            } else {
                key.push('\x1F');
            }
            match simdutf8::basic::from_utf8(field) {
                Ok(s) => key.push_str(s),
                Err(_) => key.push_str(&String::from_utf8_lossy(field)),
            }
        }

        // Single hash-table touch: insert returns true when the key is new.
        if dedup_cache.insert(&key) {
            if !no_output {
                wtr.write_byte_record(&curr_row)?;
            }
        } else {
            dupes_count += 1;
            if let Some(ref mut w) = dupewtr {
                dupe_row.clear();
                // 1-based data-row index (matches the existing fixture format).
                dupe_row.push_field(itoa::Buffer::new().format(row_idx + 1).as_bytes());
                dupe_row.extend(curr_row.iter());
                w.write_byte_record(&dupe_row)?;
            }
        }
    }

    if let Some(mut w) = dupewtr {
        w.flush()?;
    }
    wtr.flush()?;

    Ok(dupes_count)
}

fn dedup_lines(args: Args, mem_limited_buffer: u64) -> Result<u64, crate::clitypes::CliError> {
    let input_reader: Box<dyn BufRead> = match &args.arg_input {
        Some(input_path) => {
            if input_path.to_lowercase().ends_with(".sz") {
                return fail_clierror!(
                    "Input file cannot be a .sz file. Use 'qsv snappy decompress' first."
                );
            }
            let file = fs::File::open(input_path)?;
            Box::new(io::BufReader::with_capacity(
                config::DEFAULT_RDR_BUFFER_CAPACITY,
                file,
            ))
        },
        None => Box::new(io::BufReader::new(stdin().lock())),
    };
    let mut output_writer: Box<dyn Write> = match &args.arg_output {
        Some(output_path) => Box::new(io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            fs::File::create(output_path)?,
        )),
        None => Box::new(io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            stdout().lock(),
        )),
    };

    // Only open the dupes file when --dupes-output was given. The previous
    // implementation opened /dev/null (or "nul" on Windows) as a sink,
    // which wasted a real file handle and forced platform-specific code.
    let mut dupes_writer = match args.flag_dupes_output {
        Some(path) => Some(io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            fs::File::create(path)?,
        )),
        None => None,
    };
    let temp_dir = args.flag_temp_dir.map(PathBuf::from);
    let mut dedup_cache = odhtcache::ExtDedupCache::new(mem_limited_buffer, temp_dir);
    let mut dupes_count = 0_u64;
    let no_output = args.flag_no_output;
    for (row_idx, line) in input_reader.lines().enumerate() {
        let line = line?;
        // Single hash-table touch: insert returns true when the line is new.
        if dedup_cache.insert(&line) {
            if !no_output {
                writeln!(output_writer, "{line}")?;
            }
        } else {
            dupes_count += 1;
            if let Some(ref mut dw) = dupes_writer {
                writeln!(dw, "{row_idx}\t{line}")?;
            }
        }
    }
    if let Some(mut dw) = dupes_writer {
        dw.flush()?;
    }
    output_writer.flush()?;

    Ok(dupes_count)
}

/// Determines the memory buffer size to use for on-disk hash table based on
/// the provided flag and the system's total memory.
///
/// # Arguments
///
/// * `flag_memory_limit` - An optional u64 value representing the user-specified memory limit.
///
/// # Returns
///
/// A u64 value representing the calculated memory limit in bytes.
///
/// # Behavior
///
/// - If the system is not supported, it returns a predefined `MEMORY_LIMITED_BUFFER` value.
/// - If `flag_memory_limit` is None, it returns the `MEMORY_LIMITED_BUFFER`.
/// - If `flag_memory_limit` is Some(limit):
///   - For limit <= 50, it's treated as a percentage of total system memory.
///   - For limit > 50, it's treated as megabytes, but capped at 90% of total system memory.
pub fn calculate_memory_limit(flag_memory_limit: Option<u64>) -> u64 {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        return MEMORY_LIMITED_BUFFER;
    }

    let mut sys = System::new();
    sys.refresh_memory();
    let total_memory = sys.total_memory();

    #[allow(clippy::cast_precision_loss)]
    match flag_memory_limit {
        Some(limit) if limit <= 50 => ((total_memory as f64 * limit as f64) / 100.0) as u64,
        Some(limit) => {
            let limit_bytes = limit.saturating_mul(1_000_000); // Convert MB to bytes
            let ninety_percent_total = (total_memory as f64 * 0.9) as u64;
            std::cmp::min(limit_bytes, ninety_percent_total)
        },
        None => MEMORY_LIMITED_BUFFER,
    }
}

#[test]
fn test_extdedup_mem_check() {
    // check to see if sysinfo return meminfo without segfaulting
    let mut sys = System::new();
    sys.refresh_memory();
    let mem10percent = (sys.total_memory() * 1000) / 10; // 10 percent of total memory
    assert!(mem10percent > 0);
}
