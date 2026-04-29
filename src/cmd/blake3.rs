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
    --no-mmap            Disable memory mapping. Also disables multithreading.
    --no-names           Omit filenames in the output.
    --raw                Write raw output bytes to stdout, rather than hex.
                         Only a single input is allowed. --no-names is implied.
    --tag                Output checksums in tagged format.
    -c, --check          Read blake3 sums from the input files and check them.
    -j, --jobs <arg>     The number of jobs to run in parallel for hashing.
                         When not set, uses the number of CPUs detected.
                         Set to 1 to disable multithreading.

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

use crate::{CliError, CliResult, config, util};

#[derive(Deserialize)]
struct Args {
    arg_input:       Vec<String>,
    flag_keyed:      bool,
    flag_derive_key: Option<String>,
    flag_length:     usize,
    flag_no_mmap:    bool,
    flag_no_names:   bool,
    flag_raw:        bool,
    flag_tag:        bool,
    flag_check:      bool,
    flag_jobs:       Option<usize>,
    flag_output:     Option<String>,
    flag_quiet:      bool,
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
        if args.arg_input.is_empty() || args.arg_input.iter().any(|i| i == "-") {
            return fail_incorrectusage_clierror!(
                "--keyed requires file arguments and cannot read from stdin (stdin is used for \
                 the key)."
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

    // Configure rayon thread pool
    util::njobs(args.flag_jobs);

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

    let default_length = args.flag_length == 32;

    for input in &inputs {
        let (hasher, name) = hash_input(input, &hash_mode, args.flag_no_mmap)?;

        if args.flag_raw {
            if default_length {
                output_writer.write_all(hasher.finalize().as_bytes())?;
            } else {
                let mut buf = vec![0u8; args.flag_length];
                hasher.finalize_xof().fill(&mut buf);
                output_writer.write_all(&buf)?;
            }
        } else {
            let hex = if default_length {
                // Fast path: stack-allocated hash + hex via blake3's to_hex()
                hasher.finalize().to_hex().to_string()
            } else {
                // Custom length: xof path with manual hex encoding
                let mut buf = vec![0u8; args.flag_length];
                hasher.finalize_xof().fill(&mut buf);
                bytes_to_hex(&buf)
            };
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
/// Returns the populated Hasher and the display name.
fn hash_input(input: &str, mode: &HashMode, no_mmap: bool) -> CliResult<(blake3::Hasher, String)> {
    let mut hasher = match mode {
        HashMode::Default => blake3::Hasher::new(),
        HashMode::Keyed(key) => blake3::Hasher::new_keyed(key),
        HashMode::DeriveKey(context) => blake3::Hasher::new_derive_key(context),
    };

    let name = if input == "-" {
        hasher.update_reader(stdin().lock())?;
        "-".to_string()
    } else {
        let path = Path::new(input);
        if no_mmap {
            let file =
                fs::File::open(path).map_err(|e| CliError::Other(format!("{input}: {e}")))?;
            hasher.update_reader(file)?;
        } else {
            hasher
                .update_mmap_rayon(path)
                .map_err(|e| CliError::Other(format!("{input}: {e}")))?;
        }
        input.to_string()
    };

    Ok((hasher, name))
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

            // Validate hex string
            if expected_hash.len() % 2 != 0 || !expected_hash.chars().all(|c| c.is_ascii_hexdigit())
            {
                return fail_clierror!("Invalid hex checksum in {checkfile}: {expected_hash}");
            }

            let (hasher, _) = hash_input(&filename, hash_mode, args.flag_no_mmap)?;
            let expected_len = expected_hash.len() / 2;

            let actual_hex = if expected_len == 32 {
                hasher.finalize().to_hex().to_string()
            } else {
                let mut buf = vec![0u8; expected_len];
                hasher.finalize_xof().fill(&mut buf);
                bytes_to_hex(&buf)
            };

            if actual_hex.eq_ignore_ascii_case(&expected_hash) {
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
        return fail_clierror!(
            "blake3: WARNING: {failures} computed checksum{} did NOT match",
            if failures == 1 { "" } else { "s" }
        );
    }
    if total == 0 {
        return fail_clierror!("No checksums found in input files.");
    }

    Ok(())
}

/// Parse a standard checksum line: `hash  filename` (text mode)
/// or `hash *filename` (binary mode, single space + asterisk).
fn parse_standard_line(line: &str) -> CliResult<(String, String)> {
    // Text mode: two spaces between hash and filename (standard b3sum format).
    if let Some((hash, filename)) = line.split_once("  ") {
        return Ok((hash.to_string(), filename.to_string()));
    }
    // Binary mode: single space + asterisk-prefixed filename.
    if let Some((hash, filename)) = line.split_once(" *") {
        return Ok((hash.to_string(), filename.to_string()));
    }
    fail_clierror!("Invalid checksum line: {line}")
}

/// Parse a BSD-style tag line: `BLAKE3 (filename) = hash`
fn parse_tag_line(line: &str) -> CliResult<(String, String)> {
    // Caller (check_mode) gates on `line.starts_with("BLAKE3 (")` before
    // dispatching here, so this strip_prefix is infallible by contract.
    let rest = line
        .strip_prefix("BLAKE3 (")
        .expect("parse_tag_line: caller must verify BLAKE3 ( prefix");
    if let Some((filename, hash)) = rest.rsplit_once(") = ") {
        Ok((hash.to_string(), filename.to_string()))
    } else {
        fail_clierror!("Invalid tag line: {line}")
    }
}
