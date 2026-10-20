#[cfg(any(feature = "feature_capable", feature = "lite"))]
use std::borrow::Cow;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use csv::ByteRecord;
use docopt::{ArgvMap, Value};
#[cfg(any(feature = "feature_capable", feature = "lite"))]
use serde::{de::Deserialize, de::Deserializer, de::Error};
use sysinfo::System;

use super::{DEFAULT_FREEMEMORY_HEADROOM_PCT, Idx, SAFETY_MARGIN, get_envvar_flag};
use crate::{
    CliResult,
    config::{Config, Delimiter},
};

/// Refuse to run when `--output` names one of the command's own inputs.
///
/// Most commands build the `--output` writer BEFORE opening the input reader, so
/// `File::create` truncates the input to zero length and the command then reads an empty
/// file and exits 0 — silent data loss (issue #4580). Unlike shell redirection
/// (`cmd f > f`), where the shell opens the file before qsv starts, `-o` is opened by qsv
/// itself, so qsv can refuse.
///
/// This works off docopt's PARSED values rather than raw argv. It has to: docopt accepts
/// `--flag value`, `--flag=value` and `-fvalue` interchangeably, so a raw-argv scan sees
/// `--payload-tpl=payload.tpl` as one opaque token and misses that the template is an
/// input — letting `fetchpost --payload-tpl=payload.tpl … -o payload.tpl` zero the
/// template, which is the very bug being guarded against. The parsed map has each value
/// already separated from its flag, in every spelling.
///
/// Identity is tested with `same_file::is_same_file`, which compares inode/dev (file-id on
/// Windows) and therefore catches `./data.csv`, symlinks AND hard links. Canonicalizing
/// cannot: two hard links to one inode have two distinct canonical paths.
///
/// Two known bounds, both accepted deliberately:
///  - Docopt reports values, not their ROLE, so any USER-SUPPLIED argument that happens to name an
///    existing file is treated as an input. `qsv select letter in.csv -o letter`, with a file named
///    `letter` present, is refused even though the selector is not an input. Distinguishing them
///    needs per-command schema knowledge that does not exist at this chokepoint, and the failure is
///    a clear message rather than a destroyed file. Note the scope: docopt-populated DEFAULTS are
///    excluded, because a user can neither see nor avoid a collision with a value they never
///    passed.
///  - A path named inside a query string — `qsv sqlp "select * from read_csv('d.csv')" -o d.csv` —
///    is not a value of its own and is not seen.
pub(super) fn check_output_is_not_input(vals: &ArgvMap, argv: &[&str]) -> CliResult<()> {
    let Some(out_val) = vals.find("--output") else {
        // this command has no --output flag at all
        return Ok(());
    };
    let output = out_val.as_str();
    if output.is_empty() {
        return Ok(());
    }

    let out_path = Path::new(output);
    // An output that does not exist yet cannot be the same file as an existing input, and
    // a directory is never a valid --output (File::create reports that on its own). Both
    // early exits are what keep the guard quiet on ordinary invocations.
    if !out_path.is_file() {
        return Ok(());
    }

    // Compare by pointer, not by key or by string. Docopt stores `-o` and `--output` as
    // synonyms of ONE entry and which key is canonical is an implementation detail; and
    // skipping by string would be outright wrong, since `qsv fmt data.csv -o data.csv` is
    // the flagship case and there the input and output strings are identical.
    let out_ptr = std::ptr::from_ref(out_val);

    // Does `token` supply `value`? Covers docopt's three spellings: `--flag value` (the
    // value is its own token), `--flag=value`, and `-fvalue`. Uses strip_prefix + chars
    // rather than byte slicing, so a non-ASCII token cannot panic on a char boundary.
    fn token_supplies(token: &str, value: &str) -> bool {
        if token == value {
            return true;
        }
        if let Some(long) = token.strip_prefix("--") {
            return long.split_once('=').is_some_and(|(_, v)| v == value);
        }
        let Some(short) = token.strip_prefix('-') else {
            return false;
        };
        // Short options CLUSTER: docopt reads `-ntpayload.tpl` as `-n -t payload.tpl`, so an
        // attached value can begin after any run of single-char flags, not just the first.
        // Consuming only one char missed exactly that, and let `fetchpost -ntpayload.tpl
        // ... -o payload.tpl` zero the template. Flag characters are alphanumeric, so stop
        // at the first character that cannot be one - that bounds the candidate suffixes to
        // plausible split points instead of every suffix of the token.
        let mut rest = short;
        while rest
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphanumeric())
        {
            let mut chars = rest.chars();
            chars.next();
            rest = chars.as_str();
            if rest == value {
                return true;
            }
        }
        false
    }

    let is_input = |candidate: &str| -> bool {
        // Count only what the USER actually typed. The parsed map also carries
        // docopt-populated DEFAULTS, which are not inputs: `frequency`'s `--sketch-method`
        // defaults to `exact`, so an unfiltered scan refuses `qsv frequency in.csv -o exact`
        // over a value nobody passed and nobody can see.
        //
        // Mere presence in argv is not enough, because when the candidate and the output
        // are the same STRING, the --output spelling is itself an occurrence - which is how
        // that same `-o exact` slipped through a presence check. So when the strings
        // coincide, require a SECOND, independent occurrence; that is exactly the flagship
        // case `qsv fmt data.csv -o data.csv`, where `data.csv` is typed twice.
        let typed = argv
            .iter()
            .filter(|token| token_supplies(token, candidate))
            .count();
        if typed < usize::from(candidate == output) + 1 {
            return false;
        }
        let in_path = Path::new(candidate);
        in_path.is_file() && same_file::is_same_file(in_path, out_path).unwrap_or(false)
    };

    // `ArgvMap::map` is `pub` but `#[doc(hidden)]`: docopt exposes no iterator over parsed
    // values, and a fixed list of key names cannot work because the keys differ per
    // command. Revisit if a future qsv_docopt adds a public iterator.
    for (_, value) in vals.map.iter() {
        if std::ptr::from_ref(value) == out_ptr {
            continue;
        }
        // Switch/Counted/Plain(None) carry no path, so they are skipped.
        let candidate = match value {
            Value::Plain(Some(s)) if is_input(s) => s,
            Value::List(list) => match list.iter().find(|s| is_input(s)) {
                Some(s) => s,
                None => continue,
            },
            _ => continue,
        };
        return fail_clierror!(
            "--output ({output}) is the same file as an input ({candidate}). qsv would truncate \
             the input before reading it, losing the data. Pass a different --output."
        );
    }

    Ok(())
}

#[inline]
pub fn many_configs(
    inps: &[PathBuf],
    delim: Option<Delimiter>,
    no_headers: bool,
    flexible: bool,
) -> Result<Vec<Config>, String> {
    let mut inps = inps
        .iter()
        .map(|p| p.to_str().unwrap_or("-").to_owned())
        .collect::<Vec<_>>();
    if inps.is_empty() {
        inps.push("-".to_owned()); // stdin
    }
    let confs = inps
        .into_iter()
        .map(|p| {
            Config::new(Some(p).as_ref())
                .delimiter(delim)
                .no_headers(no_headers)
                .flexible(flexible)
        })
        .collect::<Vec<_>>();
    errif_greater_one_stdin(&confs)?;
    Ok(confs)
}

pub fn errif_greater_one_stdin(inps: &[Config]) -> Result<(), String> {
    let nstd = inps.iter().filter(|inp| inp.is_stdin()).count();
    if nstd > 1 {
        return fail!("At most one <stdin> input is allowed.");
    }
    Ok(())
}

pub const fn chunk_size(nitems: usize, njobs: usize) -> usize {
    if nitems < njobs {
        nitems
    } else {
        nitems / njobs
    }
}

pub const fn num_of_chunks(nitems: usize, chunk_size: usize) -> usize {
    if chunk_size == 0 {
        return nitems;
    }
    let mut n = nitems / chunk_size;
    if !nitems.is_multiple_of(chunk_size) {
        n += 1;
    }
    n
}

pub fn file_metadata(md: &fs::Metadata) -> (u64, u64) {
    use filetime::FileTime;
    let last_modified = FileTime::from_last_modification_time(md).unix_seconds() as u64;
    let fsize = md.len();
    (last_modified, fsize)
}

/// Check if there is enough memory to process the file.
/// Return the maximum file size that can be processed.
/// If the file is larger than the maximum file size, return an error.
/// If memcheck is true, check memory in CONSERVATIVE mode
///   (i.e., Filesize < (AVAIL memory + SWAP) * `platform_factor` - headroom)
/// If memcheck is false, check memory in NORMAL mode
///   (i.e., Filesize < TOTAL memory - headroom)
pub fn mem_file_check(
    path: &Path,
    version_check: bool,
    conservative_memcheck: bool,
) -> CliResult<i64> {
    // if we're NOT calling this from the version() and the file doesn't exist,
    // we don't need to check memory as file existence is checked before this function is called.
    // If we do get here with a non-existent file, that means we're using stdin,
    // so this check doesn't apply, so we return -1
    if !path.exists() && !version_check {
        return Ok(-1_i64);
    }

    let conservative_memcheck_work = get_envvar_flag("QSV_MEMORY_CHECK") || conservative_memcheck;

    let mut mem_pct =
        env::var("QSV_FREEMEMORY_HEADROOM_PCT").map_or(DEFAULT_FREEMEMORY_HEADROOM_PCT, |val| {
            atoi_simd::parse::<u8, false, false>(val.as_bytes())
                .unwrap_or(DEFAULT_FREEMEMORY_HEADROOM_PCT)
        });

    // if QSV_FREEMEMORY_HEADROOM_PCT is 0, we skip the memory check
    if mem_pct == 0 {
        return Ok(i64::MAX);
    }

    let mut sys = sysinfo::System::new();
    sys.refresh_memory();
    let avail_mem = sys.available_memory();
    let free_swap = sys.free_swap();
    let total_mem = sys.total_memory();

    // for safety, we don't want to go below 10% memory headroom
    // nor above 90% memory headroom as its too memory-restrictive
    mem_pct = mem_pct.clamp(10, 90);

    // Platform-specific adjustment factors for conservative mode
    // These account for OS-specific memory management capabilities
    let platform_factor = cfg_select! {
        // macOS has aggressive memory compression & dynamic swap
        target_os = "macos" => 1.3,
        // Linux page cache is reclaimable, but be conservative
        target_os = "linux" => 1.15,
        // Windows memory reporting is already conservative
        target_os = "windows" => 1.0,
        // Other platforms: no adjustment
        _ => 1.0,
    };

    // Calculate maximum available memory based on mode
    #[allow(clippy::cast_precision_loss)]
    let max_avail_mem = if conservative_memcheck_work {
        // CONSERVATIVE: Use available + swap, with platform adjustments
        let base_mem = (avail_mem + free_swap) as f64;
        let adjusted_mem = base_mem * platform_factor;
        (adjusted_mem * ((100 - mem_pct) as f64 / 100.0)) as u64
    } else {
        // NORMAL: Use total memory
        (total_mem as f64 * ((100 - mem_pct) as f64 / 100.0)) as u64
    };

    // if we're calling this from version(), we don't need to check the file size
    if !version_check {
        let file_metadata =
            fs::metadata(path).map_err(|e| format!("Failed to get file size: {e}"))?;
        let fsize = file_metadata.len();
        if fsize > max_avail_mem {
            return fail_oom_clierror!(
                "Not enough memory to process the file. qsv running in non-streaming {mode} mode. \
                 Total memory: {total_mem} Available memory: {avail_mem}. Free swap: {free_swap} \
                 Max Available memory/Max input file size: {max_avail_mem}. \
                 QSV_FREEMEMORY_HEADROOM_PCT: {mem_pct}%. File size: {fsize}.",
                mode = if conservative_memcheck_work {
                    "CONSERVATIVE"
                } else {
                    "NORMAL"
                },
                total_mem = indicatif::HumanBytes(total_mem),
                avail_mem = indicatif::HumanBytes(avail_mem),
                free_swap = indicatif::HumanBytes(free_swap),
                max_avail_mem = indicatif::HumanBytes(max_avail_mem),
                mem_pct = mem_pct,
                fsize = indicatif::HumanBytes(fsize)
            );
        }
    }

    Ok(max_avail_mem as i64)
}

#[cfg(any(feature = "feature_capable", feature = "lite"))]
#[inline]
pub fn condense(val: Cow<[u8]>, n: Option<usize>) -> Cow<[u8]> {
    let Some(n) = n else { return val };

    if let Ok(s) = simdutf8::basic::from_utf8(&val) {
        // Valid UTF-8: count by chars, not bytes, so multi-byte codepoints
        // count once. Truncating by bytes here would risk splitting a char.
        if s.chars().count() <= n {
            return val;
        }
        let mut out: String = s.chars().take(n).collect();
        out.push_str("...");
        return Cow::Owned(out.into_bytes());
    }

    // Not valid UTF-8: truncate by bytes.
    if val.len() <= n {
        return val;
    }
    let mut out = val[..n].to_vec();
    out.extend_from_slice(b"...");
    Cow::Owned(out)
}

/// Samples records from a CSV file for memory estimation.
///
/// Reads the first `sample_size` records for a quick estimate of average record size.
/// First-N sampling is intentionally chosen over reservoir sampling here because this
/// function is called before the main processing loop — a full-file scan would negate
/// the benefit of the estimate.
///
/// # Arguments
///
/// * `rconfig` - CSV reader configuration
/// * `sample_size` - Number of records to sample (typically 1000)
///
/// # Returns
///
/// * `Some(Vec<ByteRecord>)` - Sample records if available
/// * `None` - If no records could be sampled
pub fn sample_records(rconfig: &Config, sample_size: usize) -> Option<Vec<ByteRecord>> {
    let mut samples = Vec::with_capacity(sample_size);
    if let Ok(mut sample_rdr) = rconfig.reader() {
        let _ = sample_rdr.byte_headers(); // consume header row
        for (i, record_result) in sample_rdr.byte_records().enumerate() {
            if i >= sample_size {
                break;
            }
            if let Ok(record) = record_result {
                samples.push(record);
            } else {
                break;
            }
        }
    }
    if samples.is_empty() {
        None
    } else {
        Some(samples)
    }
}

/// Calculates dynamic chunk size based on available memory and record sampling.
///
/// This function estimates an appropriate chunk size by:
/// 1. Calculating average record size from sample records
/// 2. Getting available system memory
/// 3. Calculating memory per chunk (80% of available memory / number of jobs)
/// 4. Dividing memory per chunk by average record size
///
/// # Arguments
///
/// * `idx_count` - Total number of records in the file
/// * `njobs` - Number of parallel jobs
/// * `sample_records` - Optional slice of sample records for memory estimation
/// * `estimate_fn` - Function to estimate memory usage per record
///
/// # Returns
///
/// Calculated chunk size (number of records per chunk)
pub fn calculate_dynamic_chunk_size<F>(
    idx_count: u64,
    njobs: usize,
    sample_records: Option<&[ByteRecord]>,
    estimate_fn: F,
) -> usize
where
    F: Fn(&ByteRecord) -> usize,
{
    if let Some(samples) = sample_records {
        if samples.is_empty() {
            // No samples available, fall back to CPU-based chunking
            log::warn!("No records available for sampling, falling back to CPU-based chunking");
            return chunk_size(idx_count as usize, njobs);
        }

        let total_size: usize = samples.iter().map(&estimate_fn).sum();

        // samples.len() is guaranteed to be positive here as we checked above that it is not empty
        let avg_record_size = (total_size / samples.len()).max(1024);

        // Get available memory from system
        let mut sys = System::new();
        sys.refresh_memory();
        let avail_mem = sys.available_memory();

        // Calculate chunk size based on available memory
        // Use 80% of available memory divided by number of jobs
        #[allow(clippy::cast_precision_loss)]
        let memory_per_chunk = ((avail_mem as f64 * SAFETY_MARGIN) / njobs as f64) as usize;
        debug_assert!(avg_record_size > 0, "avg_record_size must be positive");
        let memory_based_chunk_size = memory_per_chunk / avg_record_size.max(1);

        // Ensure chunk size is reasonable
        let memory_based_chunk_size = memory_based_chunk_size.max(1).min(idx_count as usize);

        // Calculate CPU-based chunk size for optimal parallelization
        let cpu_based_chunk_size = chunk_size(idx_count as usize, njobs);

        // Calculate how many chunks each approach would create
        let memory_based_chunks = num_of_chunks(idx_count as usize, memory_based_chunk_size);
        let cpu_based_chunks = num_of_chunks(idx_count as usize, cpu_based_chunk_size);

        // Prefer CPU-based chunking if:
        // 1. Memory-based chunk size is smaller than CPU-based (degenerate/low memory), OR
        // 2. It creates more chunks (better parallelization)
        if memory_based_chunk_size <= cpu_based_chunk_size || cpu_based_chunks > memory_based_chunks
        {
            cpu_based_chunk_size
        } else {
            memory_based_chunk_size
        }
    } else {
        // No sample records provided, fall back to CPU-based chunking
        chunk_size(idx_count as usize, njobs)
    }
}

pub fn range(start: Idx, end: Idx, len: Idx, index: Idx) -> Result<(usize, usize), String> {
    match (start, end, len, index) {
        (None, None, None, Some(i)) => Ok((i, i + 1)),
        (_, _, _, Some(_)) => fail!("--index cannot be used with --start, --end or --len"),
        (_, Some(_), Some(_), None) => {
            fail!("--end and --len cannot be used at the same time.")
        },
        (_, None, None, None) => Ok((start.unwrap_or(0), usize::MAX)),
        (_, Some(e), None, None) => {
            let s = start.unwrap_or(0);
            if s > e {
                fail_format!(
                    "The end of the range ({e}) must be greater than or\nequal to the start of \
                     the range ({s})."
                )
            } else {
                Ok((s, e))
            }
        },
        (_, None, Some(l), None) => {
            let s = start.unwrap_or(0);
            Ok((s, s + l))
        },
    }
}

/// Represents a filename template of the form `"{}.csv"`, where `"{}"` is
/// the place to insert the part of the filename generated by `qsv`.
#[cfg(any(feature = "feature_capable", feature = "lite"))]
#[derive(Clone)]
pub struct FilenameTemplate {
    prefix: String,
    suffix: String,
}

#[cfg(any(feature = "feature_capable", feature = "lite"))]
impl FilenameTemplate {
    /// Generate a new filename using `unique_value` to replace the `"{}"`
    /// in the template.
    pub fn filename(&self, unique_value: &str) -> String {
        format!("{}{unique_value}{}", self.prefix, self.suffix)
    }

    /// Create a new, writable file in directory `path` with a filename
    /// using `unique_value` to replace the `"{}"` in the template.  Note
    /// that we do not output headers; the caller must do that if
    /// desired.
    pub fn writer<P>(
        &self,
        path: P,
        unique_value: &str,
    ) -> std::io::Result<csv::Writer<Box<dyn std::io::Write + 'static>>>
    where
        P: AsRef<Path>,
    {
        let filename = self.filename(unique_value);
        let full_path = path.as_ref().join(filename);
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let spath = Some(full_path.display().to_string());
        Config::new(spath.as_ref()).writer()
    }
}

#[cfg(any(feature = "feature_capable", feature = "lite"))]
impl<'de> Deserialize<'de> for FilenameTemplate {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<FilenameTemplate, D::Error> {
        let raw = String::deserialize(d)?;
        let chunks = raw.split("{}").collect::<Vec<_>>();
        if chunks.len() == 2 {
            Ok(FilenameTemplate {
                prefix: chunks[0].to_owned(),
                suffix: chunks[1].to_owned(),
            })
        } else {
            Err(D::Error::custom(
                "The --filename argument must contain one '{}'.",
            ))
        }
    }
}
