static USAGE: &str = r#"
Get tabular data from various sources into a managed, queryable disk cache.

`get` fetches a resource once, stores it compressed (zstd) and content-addressed
(BLAKE3) in the qsv cache, auto-builds a qsv index for it (for instant random
access & exact record counts), and records rich metadata (ETag, Last-Modified,
sizes, record count, TTL). Re-fetches send a conditional request
(ETag/Last-Modified) so unchanged resources are revalidated, not re-downloaded.
Large remote resources stream into the cache as parallel byte-ranges (tune with
the QSV_GET_PART_SIZE and QSV_GET_CONCURRENCY env vars).

Once cached, a resource can be read by ANY qsv command using the `dc:` prefix,
e.g. `qsv stats dc:data.csv`. Stale `dc:` entries are auto-refreshed.

A glob (e.g. data/*.csv) or directory source fetches every matching tabular file
(.csv/.tsv/.tab/.ssv) — supported for local paths and (with the get_cloud feature)
cloud buckets/prefixes. --name is ignored when a source expands to multiple files.

Supported sources:
    local file path, directory, or glob (e.g. /data/*.csv)
    http:// or https:// URL
    dathere://<path>          datHere qsv-lookup-tables repo
    ckan://<id>               a CKAN resource by id
    ckan://<name>?            a CKAN resource by name (resource_search)
    s3://<bucket>/<key>       AWS S3 / S3-compatible       (get_cloud feature)
    gs://<bucket>/<key>       Google Cloud Storage         (get_cloud feature)
    az://<container>/<key>    Azure Blob Storage           (get_cloud feature)
Cloud credentials are read from the standard AWS_*/AZURE_*/GOOGLE_* environment
variables (and IAM roles); use --cloud-opt for one-off overrides such as region
or endpoint. (sftp:// is planned for a later release.)

`--sample` PREVIEW vs the `sample` command: `get --sample N` is a cheap PEEK — it
streams just the first N rows from the head (stopping early, so a huge remote file
is barely touched) and caches nothing. It is NOT a statistical sample. For a random,
representative subset use `qsv sample` instead (which downloads the whole remote
file first, except for its streaming --bernoulli method).

Examples:
    Fetch a CSV into the cache and read it back with another command:
        $ qsv get https://example.com/data.csv --name data.csv
        $ qsv stats dc:data.csv

    Peek at a remote CSV WITHOUT caching it (preview mode, streams to stdout):
        $ qsv get https://example.com/big.csv --sample 10
        $ qsv get https://example.com/big.csv --offset 500 --sample 10
        $ qsv get https://example.com/big.csv --sample 20 --random

    Seed a CKAN reference table:
        $ qsv get "ckan://covid-vaccinations?" --name vax.csv

    Fetch every matching file via a glob or directory (each is cached separately):
        $ qsv get '/data/*.csv'
        $ qsv get /data/

    Fetch from cloud object storage (requires the get_cloud feature):
        $ qsv get s3://my-bucket/data.csv --name data.csv
        $ qsv get gs://my-bucket/data.csv --cloud-opt skip_signature=true
        $ qsv get 's3://my-bucket/exports/*.csv'

    Show what's in the cache, then prune old entries:
        $ qsv get cache-list
        $ qsv get cache-prune --older-than=30d

    Export an already-cached entry to a file or stdout (offline; no re-fetch):
        $ qsv get cache-fetch data.csv --output /tmp/data.csv
        $ qsv get cache-fetch data.csv | qsv stats

    Verify cached blob integrity, then retune an entry's TTL & policy:
        $ qsv get cache-list --verify
        $ qsv get cache-set-ttl data.csv --ttl=86400
        $ qsv get cache-set-policy data.csv --refresh=never

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_get.rs.

Usage:
    qsv get cache-list [--verify] [options]
    qsv get cache-info [options]
    qsv get cache-fetch <name> [options]
    qsv get cache-clear [options]
    qsv get cache-prune --older-than=<val> [options]
    qsv get cache-set-ttl <name> --ttl=<secs> [options]
    qsv get cache-set-policy <name> --refresh=<policy> [options]
    qsv get [--cloud-opt <kv>...] [options] <source>...
    qsv get --help

get arguments:
    <source>...            One or more sources to fetch into the cache.
    <name>                 For cache-fetch / cache-set-ttl / cache-set-policy: the
                           cached logical name (`dc:` handle) to read or modify.
                           A leading `dc:` prefix is accepted and ignored.

cache-fetch writes an ALREADY-cached entry's (decompressed) contents to the --output
file (or stdout if omitted or `-`). It is offline: it reads the cached blob directly and
never re-fetches the source. Errors if <name> is not in the cache.

get options:
    --name <name>          Logical cache name (the `dc:` handle) for the fetched
                           entry. Defaults to the source's terminal path segment.
                           Ignored when multiple sources are given.
    --ttl <secs>           Per-entry time-to-live in seconds. -1 = never expire.
                           Also the value applied by cache-set-ttl. [default: 2419200]
    --refresh <policy>     Staleness policy for `dc:` use: on-stale, always or never.
                           Also the value applied by cache-set-policy. [default: on-stale]
    --compress <algo>      Transparent blob compression: zstd or none.
                           [default: zstd]
    --force                Re-fetch even if a fresh cached copy exists.
    --sample <n>           PREVIEW: stream the first N data records of <source> to
                           stdout (or the --output file) WITHOUT caching. No `dc:`
                           entry is created. The sniffed header row is re-attached.
                           Single <source> only.
    --offset <mb>          PREVIEW: skip ~<mb> megabytes (via an HTTP Range request)
                           before sampling, realigning to the next record boundary.
                           Implies --sample. Requires a Range-capable source.
    --random               PREVIEW: random (reservoir) sampling. Streams the full
                           source and parses it from the start, so quoted multi-line
                           records stay intact. Slower than --sample (which only
                           reads the head); use it when you need a uniform sample.
    --cloud-opt <kv>       Extra cloud object-store config as a `key=value` pair
                           (repeatable), e.g. region=us-east-1 or
                           skip_signature=true. Overrides the
                           AWS_*/AZURE_*/GOOGLE_* environment. (get_cloud only)
    --ckan-api <url>       CKAN Action API base URL. Overrides the QSV_CKAN_API
                           env var. [default: https://data.dathere.com/api/3/action]
    --ckan-token <token>   CKAN API token. Overrides the QSV_CKAN_TOKEN env var.
    --timeout <secs>       HTTP timeout in seconds. For cache downloads this is an INACTIVITY
                           timeout: the transfer aborts only if no data is received from the
                           server for this long, so a slow-but-steady download is NOT cut off.
                           Preview mode (--sample / --offset / --random) instead uses it as a
                           total-request timeout. 0 = no timeout. [default: 60]
    --older-than <val>     For cache-prune: remove entries older than this age.
                           Accepts seconds, or a value with an s/m/h/d/w suffix
                           (e.g. 3600, 90m, 30d, 2w).
    --json                 For cache-list/cache-info: output JSON instead of a table.
    --verify               For cache-list: recompute each cached blob's BLAKE3 and
                           report OK/FAIL per name (exits non-zero on any failure).

Common options:
    -h, --help             Display this message
    --cache-dir <dir>      The qsv cache directory. Overrides the QSV_CACHE_DIR
                           env var. [default: ~/.qsv-cache]
    -o, --output <file>    For a single <source> (or cache-fetch <name>), write the
                           (decompressed) data to <file> (use `-` for stdout).
    -q, --quiet            Do not print progress/summary messages to stderr.
"#;

use serde::Deserialize;

use crate::{
    CliError, CliResult,
    diskcache::{self, DEFAULT_CKAN_API},
    util,
};

/// Records emitted by a `--offset`/`--random` preview when `--sample` is omitted.
const DEFAULT_PREVIEW_SAMPLE: u64 = 10;

#[derive(Deserialize)]
struct Args {
    arg_source:           Vec<String>,
    arg_name:             Option<String>,
    flag_name:            Option<String>,
    flag_cache_dir:       String,
    flag_ttl:             i64,
    flag_refresh:         String,
    flag_compress:        String,
    flag_force:           bool,
    flag_sample:          Option<u64>,
    flag_offset:          Option<u64>,
    flag_random:          bool,
    flag_cloud_opt:       Vec<String>,
    flag_ckan_api:        Option<String>,
    flag_ckan_token:      Option<String>,
    flag_timeout:         u16,
    flag_older_than:      Option<String>,
    flag_json:            bool,
    flag_verify:          bool,
    flag_output:          Option<String>,
    flag_quiet:           bool,
    cmd_cache_list:       bool,
    cmd_cache_info:       bool,
    cmd_cache_fetch:      bool,
    cmd_cache_clear:      bool,
    cmd_cache_prune:      bool,
    cmd_cache_set_ttl:    bool,
    cmd_cache_set_policy: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let cache_dir = diskcache::set_qsv_cache_dir(&args.flag_cache_dir)?;

    // ---- cache-management subcommands ----
    if args.cmd_cache_list || args.cmd_cache_info {
        return run_cache_list(
            &cache_dir,
            args.flag_json,
            args.cmd_cache_info,
            args.flag_verify,
        );
    }
    if args.cmd_cache_fetch {
        let name = args.arg_name.as_deref().unwrap_or_default();
        // convenience: accept (and ignore) a leading `dc:` prefix
        let name = name.strip_prefix("dc:").unwrap_or(name);
        diskcache::write_output(&cache_dir, name, args.flag_output.as_deref())?;
        // stderr confirmation only when writing to a real file (never pollute stdout data)
        if !args.flag_quiet
            && let Some(p) = args.flag_output.as_deref()
            && p != "-"
        {
            eprintln!("✓ wrote dc:{name} to {p}");
        }
        return Ok(());
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
    if args.cmd_cache_set_ttl {
        let name = args.arg_name.as_deref().unwrap_or_default();
        // convenience: accept (and ignore) a leading `dc:` prefix, like cache-fetch
        let name = name.strip_prefix("dc:").unwrap_or(name);
        diskcache::set_ttl(&cache_dir, name, args.flag_ttl)?;
        if !args.flag_quiet {
            eprintln!("Set TTL of '{name}' to {} seconds.", args.flag_ttl);
        }
        return Ok(());
    }
    if args.cmd_cache_set_policy {
        let policy = diskcache::RefreshPolicy::parse(&args.flag_refresh)?;
        let name = args.arg_name.as_deref().unwrap_or_default();
        // convenience: accept (and ignore) a leading `dc:` prefix, like cache-fetch
        let name = name.strip_prefix("dc:").unwrap_or(name);
        diskcache::set_policy(&cache_dir, name, policy)?;
        if !args.flag_quiet {
            eprintln!("Set refresh policy of '{name}' to {}.", args.flag_refresh);
        }
        return Ok(());
    }

    let ckan_api_url = args
        .flag_ckan_api
        .clone()
        .or_else(|| std::env::var("QSV_CKAN_API").ok())
        .or_else(|| Some(DEFAULT_CKAN_API.to_string()));
    let ckan_token = args
        .flag_ckan_token
        .clone()
        .or_else(|| std::env::var("QSV_CKAN_TOKEN").ok());

    // ---- preview mode (--sample/--offset/--random): peek WITHOUT caching ----
    if args.flag_sample.is_some() || args.flag_offset.is_some() || args.flag_random {
        if args.arg_source.len() != 1 {
            return Err(CliError::Other(
                "get: preview mode (--sample/--offset/--random) requires exactly one <source>."
                    .to_string(),
            ));
        }
        let preview = diskcache::PreviewOptions {
            source:       args.arg_source[0].clone(),
            sample:       args.flag_sample.unwrap_or(DEFAULT_PREVIEW_SAMPLE),
            offset_mb:    args.flag_offset,
            random:       args.flag_random,
            ckan_api_url: ckan_api_url.clone(),
            ckan_token:   ckan_token.clone(),
            timeout_secs: args.flag_timeout,
            cloud_opts:   args.flag_cloud_opt.clone(),
        };
        return diskcache::preview_resource(&preview, args.flag_output.as_deref());
    }

    // ---- main get form ----
    let refresh_policy = diskcache::RefreshPolicy::parse(&args.flag_refresh)?;
    let compression = diskcache::Compression::parse(&args.flag_compress)?;

    // Expand any glob/directory sources (local and, with get_cloud, cloud) into
    // concrete sources; non-glob sources pass through unchanged.
    let expand_ctx = diskcache::ExpandCtx {
        cloud_opts: args.flag_cloud_opt.clone(),
    };
    let mut sources = Vec::with_capacity(args.arg_source.len());
    for s in &args.arg_source {
        sources.extend(diskcache::expand_source(s, &expand_ctx)?);
    }

    let multiple = sources.len() > 1;

    for source in &sources {
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
            cloud_opts: args.flag_cloud_opt.clone(),
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

fn run_cache_list(cache_dir: &str, json: bool, info: bool, verify: bool) -> CliResult<()> {
    if verify {
        let results = diskcache::verify(cache_dir)?;
        let failed = results.iter().filter(|(_, ok)| !*ok).count();
        if json {
            let arr: Vec<_> = results
                .iter()
                .map(|(name, ok)| serde_json::json!({ "name": name, "ok": ok }))
                .collect();
            let s = serde_json::to_string_pretty(&arr).map_err(|e| {
                CliError::Other(format!("get: failed to serialize verify results: {e}"))
            })?;
            println!("{s}");
        } else if results.is_empty() {
            println!("(cache is empty)");
        } else {
            println!("{:<32} STATUS", "NAME");
            for (name, ok) in &results {
                println!(
                    "{:<32} {}",
                    truncate(name, 32),
                    if *ok { "OK" } else { "FAIL" }
                );
            }
        }
        if failed > 0 {
            return Err(CliError::Other(format!(
                "get: {failed} cached blob{} failed integrity verification.",
                if failed == 1 { "" } else { "s" }
            )));
        }
        return Ok(());
    }

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
        let with_schema = entries.iter().filter(|e| e.sniffed.is_some()).count();
        println!("cache directory   : {cache_dir}/get");
        println!("names             : {names}");
        println!("with schema       : {with_schema}");
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
        "{:<24} {:>10} {:>12} {:>12} {:>4} {:<5} {:<16} SOURCE",
        "NAME", "RECORDS", "COMP", "UNCOMP", "IDX", "DELIM", "BLAKE3"
    );
    for e in &entries {
        let records = e
            .record_count
            .map_or_else(|| "?".to_string(), |c| c.to_string());
        let b3 = &e.blake3[..e.blake3.len().min(16)];
        println!(
            "{:<24} {:>10} {:>12} {:>12} {:>4} {:<5} {:<16} {}",
            truncate(&e.logical_name, 24),
            records,
            e.size_compressed,
            e.size_uncompressed,
            if e.indexed { "yes" } else { "no" },
            delim_label(e.sniffed.as_ref()),
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

/// Short label for a sniffed delimiter, shown in the `cache-list` DELIM column.
fn delim_label(sniffed: Option<&diskcache::SniffedDialect>) -> &'static str {
    match sniffed.map(|s| s.delimiter) {
        Some(',') => "csv",
        Some('\t') => "tsv",
        Some(';') => "ssv",
        Some('|') => "psv",
        Some(_) => "oth",
        None => "?",
    }
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
