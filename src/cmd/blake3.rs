static USAGE: &str = r#"
Compute cryptographic hashes of files using blake3.

This command is functionally similar to b3sum, providing fast, parallel blake3 hashing
of one or more files. It supports keyed hashing, key derivation, variable-length output,
and checksum verification. When no file is given, or when "-" is given, reads stdin.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_blake3.rs.

Usage:
    qsv blake3 [options] [<input>...]
    qsv blake3 --help

blake3 options:
    --keyed              Use the keyed mode, reading the 32-byte key from stdin.
                         When using --keyed, file arguments are required (cannot
                         also read data from stdin).
    --derive-key <CTX>   Use the key derivation mode, with the given context string.
                         Cannot be used with --keyed.
    -l, --length <LEN>   The number of output bytes, before hex encoding.
                         [default: 32]
    --num-threads <NUM>  The maximum number of threads to use for hashing.
                         When not set, uses the number of CPUs.
                         Set to 1 to disable multithreading.
    --no-mmap            Disable memory mapping. Also disables multithreading.
    --no-names           Omit filenames in the output.
    --raw                Write raw output bytes to stdout, rather than hex.
                         Only a single input is allowed. --no-names is implied.
    --tag                Output checksums in tagged format.
    -c, --check          Read blake3 sums from the input files and check them.

Common options:
    -h, --help           Display this message
    -o, --output <file>  Write output to <file> instead of stdout.
    -q, --quiet          Skip printing OK for each checked file.
                         Must be used with --check.
"#;

use std::{
    fmt::Write as FmtWrite,
    fs,
    io::{self, Read, Write, stdin},
    path::Path,
};

use serde::Deserialize;

use crate::{CliResult, config, util};

#[derive(Deserialize)]
struct Args {
    arg_input:        Vec<String>,
    flag_keyed:       bool,
    flag_derive_key:  Option<String>,
    flag_length:      usize,
    flag_num_threads: Option<usize>,
    flag_no_mmap:     bool,
    flag_no_names:    bool,
    flag_raw:         bool,
    flag_tag:         bool,
    flag_check:       bool,
    flag_output:      Option<String>,
    flag_quiet:       bool,
}

/// The hashing mode to use.
enum HashMode {
    /// Default BLAKE3 hashing.
    Default,
    /// Keyed hashing with a 32-byte key.
    Keyed([u8; blake3::KEY_LEN]),
    /// Key derivation with a context string.
    DeriveKey(String),
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Validate flag combinations
    if args.flag_keyed && args.flag_derive_key.is_some() {
        return fail_incorrectusage_clierror!("--keyed and --derive-key cannot be used together.");
    }
    if args.flag_raw && args.flag_tag {
        return fail_incorrectusage_clierror!("--raw and --tag cannot be used together.");
    }
    if args.flag_quiet && !args.flag_check {
        return fail_incorrectusage_clierror!("--quiet must be used with --check.");
    }
    if args.flag_length == 0 {
        return fail_incorrectusage_clierror!("--length must be at least 1.");
    }

    // Determine hash mode
    let hash_mode = if args.flag_keyed {
        if args.arg_input.is_empty() {
            return fail_incorrectusage_clierror!(
                "--keyed requires file arguments (stdin is used for the key)."
            );
        }
        let mut key = [0u8; blake3::KEY_LEN];
        stdin().lock().read_exact(&mut key).map_err(|e| {
            CliError::Other(format!(
                "Failed to read {}-byte key from stdin: {e}",
                blake3::KEY_LEN
            ))
        })?;
        HashMode::Keyed(key)
    } else if let Some(ref context) = args.flag_derive_key {
        HashMode::DeriveKey(context.clone())
    } else {
        HashMode::Default
    };

    // Configure rayon thread pool for num-threads
    if let Some(num_threads) = args.flag_num_threads {
        let threads = if num_threads == 0 {
            num_cpus::get()
        } else {
            num_threads
        };
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .ok(); // Ignore error if pool is already initialized
    }

    // Set up output
    let mut output_writer: Box<dyn Write> = match &args.flag_output {
        Some(output_path) => Box::new(io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            fs::File::create(output_path)?,
        )),
        None => Box::new(io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            io::stdout(),
        )),
    };

    if args.flag_check {
        return check_mode(&args, &hash_mode, &mut output_writer);
    }

    // Determine inputs: if no args (or only "-"), use stdin
    let inputs: Vec<String> = if args.arg_input.is_empty() {
        vec!["-".to_string()]
    } else {
        args.arg_input.clone()
    };

    if args.flag_raw && inputs.len() > 1 {
        return fail_incorrectusage_clierror!("--raw only supports a single input.");
    }

    for input in &inputs {
        let (hash_bytes, name) = hash_input(input, &hash_mode, args.flag_no_mmap)?;

        // Produce output based on length
        let output_bytes = finalize_to_bytes(&hash_bytes, args.flag_length);

        if args.flag_raw {
            output_writer.write_all(&output_bytes)?;
        } else {
            let hex = bytes_to_hex(&output_bytes);
            if args.flag_no_names {
                writeln!(output_writer, "{hex}")?;
            } else if args.flag_tag {
                writeln!(output_writer, "BLAKE3 ({name}) = {hex}")?;
            } else {
                writeln!(output_writer, "{hex}  {name}")?;
            }
        }
    }

    output_writer.flush()?;
    Ok(())
}

/// Hash a single input (file path or "-" for stdin).
/// Returns the blake3 OutputReader and the display name.
fn hash_input(
    input: &str,
    mode: &HashMode,
    no_mmap: bool,
) -> CliResult<(blake3::OutputReader, String)> {
    let mut hasher = match mode {
        HashMode::Default => blake3::Hasher::new(),
        HashMode::Keyed(key) => blake3::Hasher::new_keyed(key),
        HashMode::DeriveKey(context) => blake3::Hasher::new_derive_key(context),
    };

    let name = if input == "-" {
        // Read from stdin
        let mut buf = Vec::new();
        stdin().lock().read_to_end(&mut buf)?;
        hasher.update(&buf);
        "-".to_string()
    } else {
        let path = Path::new(input);
        if !path.exists() {
            return fail_clierror!("{input}: No such file or directory");
        }
        if no_mmap {
            // Read file without mmap, single-threaded
            let data = fs::read(path)?;
            hasher.update(&data);
        } else {
            hasher
                .update_mmap_rayon(path)
                .map_err(|e| CliError::Other(format!("Failed to hash {input}: {e}")))?;
        }
        input.to_string()
    };

    Ok((hasher.finalize_xof(), name))
}

/// Read `length` bytes from the blake3 output reader.
fn finalize_to_bytes(output: &blake3::OutputReader, length: usize) -> Vec<u8> {
    let mut buf = vec![0u8; length];
    let mut reader = output.clone();
    reader.fill(&mut buf);
    buf
}

/// Convert bytes to lowercase hex string.
fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(hex, "{b:02x}").unwrap();
    }
    hex
}

/// Check mode: read checksum files and verify them.
fn check_mode(
    args: &Args,
    hash_mode: &HashMode,
    output_writer: &mut Box<dyn Write>,
) -> CliResult<()> {
    if args.arg_input.is_empty() {
        return fail_incorrectusage_clierror!("--check requires file arguments.");
    }

    let mut failures = 0u64;
    let mut total = 0u64;

    for checkfile in &args.arg_input {
        let contents = fs::read_to_string(checkfile)
            .map_err(|e| CliError::Other(format!("{checkfile}: {e}")))?;

        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            total += 1;

            let (expected_hash, filename) = if line.starts_with("BLAKE3 (") {
                // Tag format: BLAKE3 (filename) = hash
                parse_tag_line(line)?
            } else {
                // Standard format: hash  filename
                parse_standard_line(line)?
            };

            let (output_reader, _) = hash_input(&filename, hash_mode, args.flag_no_mmap)?;
            // Determine expected length from the hex string
            let expected_len = expected_hash.len() / 2;
            let actual_bytes = finalize_to_bytes(&output_reader, expected_len);
            let actual_hex = bytes_to_hex(&actual_bytes);

            if actual_hex == expected_hash {
                if !args.flag_quiet {
                    writeln!(output_writer, "{filename}: OK")?;
                }
            } else {
                writeln!(output_writer, "{filename}: FAILED")?;
                failures += 1;
            }
        }
    }

    output_writer.flush()?;

    if failures > 0 {
        werr!(
            "blake3: WARNING: {failures} computed checksum{} did NOT match",
            if failures == 1 { "" } else { "s" }
        );
        return fail!("");
    }
    if total == 0 {
        return fail_clierror!("No checksums found in input files.");
    }

    Ok(())
}

/// Parse a standard checksum line: `hash  filename`
fn parse_standard_line(line: &str) -> CliResult<(String, String)> {
    // Split on two spaces (standard b3sum format)
    if let Some((hash, filename)) = line.split_once("  ") {
        Ok((hash.to_string(), filename.to_string()))
    } else {
        fail_clierror!("Invalid checksum line: {line}")
    }
}

/// Parse a BSD-style tag line: `BLAKE3 (filename) = hash`
fn parse_tag_line(line: &str) -> CliResult<(String, String)> {
    // Format: BLAKE3 (filename) = hash
    let rest = line
        .strip_prefix("BLAKE3 (")
        .ok_or_else(|| CliError::Other(format!("Invalid tag line: {line}")))?;
    if let Some((filename, hash)) = rest.rsplit_once(") = ") {
        Ok((hash.to_string(), filename.to_string()))
    } else {
        fail_clierror!("Invalid tag line: {line}")
    }
}

use crate::CliError;
