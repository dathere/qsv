static USAGE: &str = r#"
Get tabular data from various sources into a managed, queryable disk cache.

`get` fetches a resource once, stores it compressed (zstd) and content-addressed
(BLAKE3) in the qsv cache, auto-builds a qsv index for it (for instant random
access & exact record counts), and records rich metadata (ETag, Last-Modified,
sizes, record count, TTL). Subsequent fetches reuse HTTP cache semantics
(ETag/Cache-Control via http-cache) so unchanged resources are not re-downloaded.

Once cached, a resource can be read by ANY qsv command using the `dc:` prefix,
e.g. `qsv stats dc:data.csv`. Stale `dc:` entries are auto-refreshed.

Supported sources:
    local file path
    http:// or https:// URL
    dathere://<path>          datHere qsv-lookup-tables repo
    ckan://<id>               a CKAN resource by id
    ckan://<name>?            a CKAN resource by name (resource_search)
(AWS S3, Azure Blob & Google Cloud Storage, and sftp:// are planned for a
later release.)

Examples:
    Fetch a CSV into the cache and read it back with another command:
        $ qsv get https://example.com/data.csv --name data.csv
        $ qsv stats dc:data.csv

    Seed a CKAN reference table:
        $ qsv get "ckan://covid-vaccinations?" --name vax.csv

    Show what's in the cache, then prune old entries:
        $ qsv get cache-list
        $ qsv get cache-prune --older-than=30d

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_get.rs.

Usage:
    qsv get cache-list [options]
    qsv get cache-info [options]
    qsv get cache-clear [options]
    qsv get cache-prune --older-than=<val> [options]
    qsv get [options] <source>...
    qsv get --help

get arguments:
    <source>...            One or more sources to fetch into the cache.

get options:
    --name <name>          Logical cache name (the `dc:` handle) for the fetched
                           entry. Defaults to the source's terminal path segment.
                           Ignored when multiple sources are given.
    --ttl <secs>           Per-entry time-to-live in seconds. -1 = never expire.
                           [default: 2419200]
    --refresh <policy>     Staleness policy for `dc:` use: on-stale, always or never.
                           [default: on-stale]
    --compress <algo>      Transparent blob compression: zstd or none.
                           [default: zstd]
    --force                Re-fetch even if a fresh cached copy exists.
    --ckan-api <url>       CKAN Action API base URL. Overrides the QSV_CKAN_API
                           env var. [default: https://data.dathere.com/api/3/action]
    --ckan-token <token>   CKAN API token. Overrides the QSV_CKAN_TOKEN env var.
    --timeout <secs>       HTTP request timeout in seconds. [default: 30]
    --older-than <val>     For cache-prune: remove entries older than this age.
                           Accepts seconds, or a value with an s/m/h/d/w suffix
                           (e.g. 3600, 90m, 30d, 2w).
    --json                 For cache-list/cache-info: output JSON instead of a table.

Common options:
    -h, --help             Display this message
    --cache-dir <dir>      The qsv cache directory. Overrides the QSV_CACHE_DIR
                           env var. [default: ~/.qsv-cache]
    -o, --output <file>    For a single <source>, also write the fetched
                           (decompressed) data to <file> (use `-` for stdout).
    -q, --quiet            Do not print progress/summary messages to stderr.
"#;

use serde::Deserialize;

use crate::{
    CliError, CliResult,
    diskcache::{self, DEFAULT_CKAN_API},
    util,
};

#[derive(Deserialize)]
struct Args {
    arg_source:      Vec<String>,
    flag_name:       Option<String>,
    flag_cache_dir:  String,
    flag_ttl:        i64,
    flag_refresh:    String,
    flag_compress:   String,
    flag_force:      bool,
    flag_ckan_api:   Option<String>,
    flag_ckan_token: Option<String>,
    flag_timeout:    u16,
    flag_older_than: Option<String>,
    flag_json:       bool,
    flag_output:     Option<String>,
    flag_quiet:      bool,
    cmd_cache_list:  bool,
    cmd_cache_info:  bool,
    cmd_cache_clear: bool,
    cmd_cache_prune: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let cache_dir = diskcache::set_qsv_cache_dir(&args.flag_cache_dir)?;

    // ---- cache-management subcommands ----
    if args.cmd_cache_list || args.cmd_cache_info {
        return run_cache_list(&cache_dir, args.flag_json, args.cmd_cache_info);
    }
    if args.cmd_cache_clear {
        let removed = diskcache::clear(&cache_dir)?;
        if !args.flag_quiet {
            eprintln!("Cleared {removed} cache entr{}.", plural(removed));
        }
        return Ok(());
    }
    if args.cmd_cache_prune {
        let older_than = parse_age_secs(args.flag_older_than.as_deref())?;
        let removed = diskcache::prune(&cache_dir, older_than)?;
        if !args.flag_quiet {
            eprintln!("Pruned {removed} cache entr{}.", plural(removed));
        }
        return Ok(());
    }

    // ---- main get form ----
    let refresh_policy = diskcache::RefreshPolicy::parse(&args.flag_refresh)?;
    let compression = diskcache::Compression::parse(&args.flag_compress)?;

    let ckan_api_url = args
        .flag_ckan_api
        .clone()
        .or_else(|| std::env::var("QSV_CKAN_API").ok())
        .or_else(|| Some(DEFAULT_CKAN_API.to_string()));
    let ckan_token = args
        .flag_ckan_token
        .clone()
        .or_else(|| std::env::var("QSV_CKAN_TOKEN").ok());

    let multiple = args.arg_source.len() > 1;

    for source in &args.arg_source {
        let opts = diskcache::GetOptions {
            source: source.clone(),
            name: if multiple {
                None
            } else {
                args.flag_name.clone()
            },
            cache_dir: cache_dir.clone(),
            ttl_secs: args.flag_ttl,
            refresh_policy,
            compression,
            force: args.flag_force,
            ckan_api_url: ckan_api_url.clone(),
            ckan_token: ckan_token.clone(),
            timeout_secs: args.flag_timeout,
        };
        let meta = diskcache::get_resource(&opts)?;

        if !args.flag_quiet {
            let records = meta
                .record_count
                .map_or_else(|| "?".to_string(), |c| c.to_string());
            eprintln!(
                "✓ {} → dc:{} ({records} records, {} → {} bytes)",
                meta.source_uri, meta.logical_name, meta.size_uncompressed, meta.size_compressed
            );
        }

        if !multiple && args.flag_output.is_some() {
            diskcache::write_output(&cache_dir, &meta.logical_name, args.flag_output.as_deref())?;
        }
    }

    Ok(())
}

fn run_cache_list(cache_dir: &str, json: bool, info: bool) -> CliResult<()> {
    let entries = diskcache::list_entries(cache_dir)?;

    if json {
        let s = serde_json::to_string_pretty(&entries)
            .map_err(|e| CliError::Other(format!("get: failed to serialize cache list: {e}")))?;
        println!("{s}");
        return Ok(());
    }

    if info {
        // `entries` has one row per name. On-disk totals must de-dupe by the
        // actual file — (blake3, compression) — since the same content stored in
        // different compression variants (.zst vs .raw) occupies distinct files.
        // Content-level totals (uncompressed bytes, records) de-dupe by hash.
        let names = entries.len();
        let mut seen_blobs = std::collections::HashSet::new();
        let mut seen_content = std::collections::HashSet::new();
        let (mut comp, mut uncomp, mut records, mut blobs) = (0u64, 0u64, 0u64, 0u64);
        for e in &entries {
            if seen_blobs.insert((e.blake3.clone(), e.compression)) {
                comp += e.size_compressed;
                blobs += 1;
            }
            if seen_content.insert(e.blake3.clone()) {
                uncomp += e.size_uncompressed;
                records += e.record_count.unwrap_or(0);
            }
        }
        println!("cache directory   : {cache_dir}/get");
        println!("names             : {names}");
        println!("unique blobs      : {blobs}");
        println!("total records     : {records}");
        println!("on disk (comp)    : {comp} bytes");
        println!("original (uncomp) : {uncomp} bytes");
        return Ok(());
    }

    if entries.is_empty() {
        println!("(cache is empty)");
        return Ok(());
    }

    println!(
        "{:<24} {:>10} {:>12} {:>12} {:>4} {:<16} SOURCE",
        "NAME", "RECORDS", "COMP", "UNCOMP", "IDX", "BLAKE3"
    );
    for e in &entries {
        let records = e
            .record_count
            .map_or_else(|| "?".to_string(), |c| c.to_string());
        let b3 = &e.blake3[..e.blake3.len().min(16)];
        println!(
            "{:<24} {:>10} {:>12} {:>12} {:>4} {:<16} {}",
            truncate(&e.logical_name, 24),
            records,
            e.size_compressed,
            e.size_uncompressed,
            if e.indexed { "yes" } else { "no" },
            b3,
            e.source_uri,
        );
    }
    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let kept: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{kept}…")
    }
}

fn plural(n: usize) -> &'static str {
    if n == 1 { "y" } else { "ies" }
}

/// Parse a `--older-than` value into seconds. Accepts a bare integer (seconds)
/// or an integer with an `s`/`m`/`h`/`d`/`w` suffix.
fn parse_age_secs(val: Option<&str>) -> CliResult<i64> {
    let s = val
        .ok_or_else(|| CliError::Other("get: cache-prune requires --older-than".to_string()))?
        .trim();
    let (digits, mult): (&str, i64) = match s.chars().last() {
        Some('s' | 'S') => (&s[..s.len() - 1], 1),
        Some('m' | 'M') => (&s[..s.len() - 1], 60),
        Some('h' | 'H') => (&s[..s.len() - 1], 3600),
        Some('d' | 'D') => (&s[..s.len() - 1], 86_400),
        Some('w' | 'W') => (&s[..s.len() - 1], 604_800),
        _ => (s, 1),
    };
    let base: i64 = digits
        .trim()
        .parse()
        .map_err(|_| CliError::Other(format!("get: invalid --older-than value '{s}'")))?;
    Ok(base.saturating_mul(mult))
}
