use super::{ByteString, current_exe, to_lowercase_into};
use crate::CliResult;

#[inline]
/// Rounds a floating point number to a specified number of decimal places.
///
/// This function takes a 64-bit floating point number and rounds it to the specified number of
/// decimal places using "Bankers Rounding" (Midpoint Nearest Even) strategy. It returns the result
/// as a String.
///
/// # Arguments
///
/// * `dec_f64` - The floating point number to round
/// * `places` - The number of decimal places to round to. If set to 9999, no rounding is performed.
///
/// # Returns
///
/// * A String containing the rounded number with trailing zeros removed and -0.0 normalized to 0.0
pub fn round_num(dec_f64: f64, places: u32) -> String {
    use rust_decimal::{Decimal, RoundingStrategy};

    if dec_f64.is_nan() {
        return String::new();
    }

    // if places is the sentinel value 9999, we don't round, just return the number as is
    if places == 9999 {
        return zmij::Buffer::new().format(dec_f64).to_owned();
    }

    // use from_f64_retain, so we have all the excess bits before rounding with
    // round_dp_with_strategy as from_f64 will prematurely round when it drops the excess bits
    let Some(dec_num) = Decimal::from_f64_retain(dec_f64) else {
        return String::new();
    };

    // round using Midpoint Nearest Even Rounding Strategy AKA "Bankers Rounding."
    // https://docs.rs/rust_decimal/latest/rust_decimal/enum.RoundingStrategy.html#variant.MidpointNearestEven
    // we also normalize to remove trailing zeroes and to change -0.0 to 0.0.
    dec_num
        .round_dp_with_strategy(places, RoundingStrategy::MidpointNearestEven)
        .normalize()
        .to_string()
}

#[inline]
/// Transforms a byte slice into a `ByteString` with optional case-insensitive conversion.
///
/// This function takes a byte slice and attempts to convert it to a UTF-8 string. If successful,
/// it trims whitespace and optionally converts to lowercase. If the input is not valid UTF-8,
/// it returns the original bytes unchanged.
///
/// It's fine-tuned for speed and memory usage, using simdutf8 for UTF-8 validation and
/// `to_lowercase_into` for non-allocating, in-place lowercase conversion.
///
/// # Arguments
///
/// * `bs` - The input byte slice to transform
/// * `casei` - If true, converts the string to lowercase. If false, leaves case unchanged.
///
/// # Returns
///
/// * A `ByteString` (Vec<u8>) containing the transformed bytes
pub fn transform(bs: &[u8], casei: bool) -> ByteString {
    if let Ok(s) = simdutf8::basic::from_utf8(bs) {
        if casei {
            let mut buffer = String::with_capacity(bs.len());
            to_lowercase_into(s.trim(), &mut buffer);
            buffer.into_bytes()
        } else {
            s.trim().as_bytes().to_vec()
        }
    } else {
        bs.to_vec()
    }
}

pub fn load_dotenv() -> CliResult<()> {
    // First, check if there is a QSV_DOTENV_PATH environment variable set
    // if there is, use that as the .env file.
    // Second, use the default .env file in the current directory.
    // If there is no .env file in the current directory, check if there is
    // an .env file with the same filestem as the binary, in the same directory as the binary.
    // If there is, use that. Failing that, qsv proceeds with its default settings and
    // whatever manually set environment variables are present.

    if let Ok(dotenv_path) = std::env::var("QSV_DOTENV_PATH") {
        // <NONE> is a sentinel value to disable dotenv processing
        if dotenv_path == "<NONE>" {
            log::warn!("dotenv processing disabled with QSV_DOTENV_PATH=<NONE>");
            return Ok(());
        }

        let canonical_dotenv_path = std::fs::canonicalize(dotenv_path)?;
        if let Err(e) = dotenvy::from_filename_override(canonical_dotenv_path.clone()) {
            return fail_clierror!(
                "Cannot process .env file set in QSV_DOTENV_PATH - {}: {e}",
                canonical_dotenv_path.display()
            );
        }
        log::info!("Using .env file: {}", canonical_dotenv_path.display());
        return Ok(());
    }

    // check if there is an .env file in the current directory
    if dotenvy::dotenv_override().is_ok() {
        log::info!(
            "Using .env file in current directory: {}",
            std::env::current_dir()?.display()
        );
    } else {
        // no .env file in the current directory or it was invalid
        // now check if there is an .env file with the same name as the executable
        // in the same directory as the executable
        let qsv_binary_path = current_exe()?;

        let qsv_dir = qsv_binary_path.parent().ok_or("No parent directory")?;

        // safety: we know that the file_stem() will not be None as we checked above
        let qsv_binary_filestem = qsv_binary_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let mut qsv_binary_envprofile = qsv_dir.to_path_buf();
        qsv_binary_envprofile.set_file_name(format!("{qsv_binary_filestem}.env"));

        if std::path::Path::new(&qsv_binary_envprofile).exists() {
            log::info!(
                "Using binary .env file: {}",
                qsv_binary_envprofile.display()
            );
            if let Err(e) = dotenvy::from_filename_override(qsv_binary_envprofile.clone()) {
                return fail_clierror!(
                    "Cannot process binary .env file - {}: {e}",
                    qsv_binary_envprofile.display()
                );
            }
        } else {
            // there is no binary .env file, just use the default settings
            // and whatever manually set environment variables are present
            log::info!(
                "No valid .env file found. Proceeding with default settings and current \
                 environment variable settings."
            );
        }
    }

    Ok(())
}

#[inline]
pub fn get_envvar_flag(key: &str) -> bool {
    use std::{
        collections::HashSet,
        sync::{Mutex, OnceLock},
    };

    static UNRECOGNIZED_WARNED: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

    let Ok(raw) = std::env::var(key) else {
        return false;
    };
    // Trim whitespace so values like " true" are recognized as truthy.
    let v = raw.trim().to_ascii_lowercase();
    let truthy = matches!(v.as_str(), "true" | "t" | "1" | "yes" | "y" | "on");
    let falsy = matches!(v.as_str(), "" | "false" | "f" | "0" | "no" | "n" | "off");
    if !truthy && !falsy {
        // Warn at most once per (key, value) pair across the process so callers
        // in hot loops don't spam the log with the same misconfiguration notice.
        // Decide whether to warn while holding the lock, then drop the guard
        // before calling into the logger — log::warn! can take its own locks
        // (flexi_logger), so we don't want to bridge them here.
        let warned = UNRECOGNIZED_WARNED.get_or_init(|| Mutex::new(HashSet::new()));
        let should_warn = match warned.lock() {
            Ok(mut set) => set.insert(format!("{key}={raw}")),
            Err(_) => false,
        };
        if should_warn {
            log::warn!("{key}={raw} is not a recognized boolean; treating as false");
        }
    }
    truthy
}

/// Returns `true` if `flag` appears in `argv` as either a standalone token
/// (`--flag`) or in the `--flag=value` form. Used by the OOM auto-fallback in
/// `stats` and `frequency` to detect whether the user explicitly passed an
/// option, since docopt fills in default values that are indistinguishable
/// from explicit user input on the parsed `Args` struct.
#[inline]
pub fn argv_has_flag(argv: &[&str], flag: &str) -> bool {
    let eq_prefix = format!("{flag}=");
    argv.iter().any(|a| *a == flag || a.starts_with(&eq_prefix))
}
