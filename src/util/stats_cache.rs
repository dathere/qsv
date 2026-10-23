#[cfg(target_family = "unix")]
use std::os::unix::process::ExitStatusExt;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str,
};

use csv::ByteRecord;
use filetime::FileTime;
use log::info;

use super::{
    DEFAULT_STATSCACHE_MODE, DerivedFile, SchemaArgs, StatsMode, csv_to_jsonl, current_exe,
    get_envvar_flag, njobs,
};
use crate::{
    CliError, CliResult,
    cmd::stats::{STATSDATA_TYPES_MAP, StatsData},
    config::{
        Config, DEFAULT_RDR_BUFFER_CAPACITY, Delimiter, SpecialFormat, get_delim_by_extension,
        get_special_format,
    },
};

/// get stats records from stats.csv.data.jsonl file, or if its invalid, by running the stats
/// command returns tuple (`csv_fields`, `csv_stats`)
/// Does a reused stats cache contain everything `mode` needs?
///
/// The stats cache (`<input>.stats.csv.data.jsonl`) is keyed only by
/// mtime, not by the `StatsMode` that produced it, so a cache built by a
/// leaner mode can be reused by a richer one. Most modes only need the
/// base cardinality/types set any cache carries, so they always pass.
/// `StatsMode::ProfileSchema` additionally needs mode + quartiles; a cache
/// from `qsv schema` (`StatsMode::Schema`) lacks both. Since no
/// producing-mode marker is stored, sufficiency is detected from the data:
///   * `--mode` populates the `mode_count` metadata field for every column (even `Some(0)` for an
///     all-unique column, where the `mode` value itself is `None`); a lean cache leaves
///     `mode_count` `None`. We test `mode_count`, not `mode`, so all-unique datasets aren't
///     mistaken for a lean cache and needlessly regenerated on every run.
///   * `--quartiles` populates `q2_median` for numeric columns, so require it only when the dataset
///     actually has a numeric column.
///   * `--zero-padded-numeric` is NOT detectable from the data (the column is empty for every
///     non-flagged column, so a dataset with no zero-padded codes looks identical with or without
///     the flag) and is therefore not required here: a pre-existing cache without it is still
///     reused, and `viz smart` simply falls back to its statistical routing for such columns until
///     the cache is naturally regenerated.
fn stats_satisfy_mode(stats: &[StatsData], mode: StatsMode) -> bool {
    if mode != StatsMode::ProfileSchema {
        return true;
    }
    let has_mode = stats.iter().any(|s| s.mode_count.is_some());
    let has_numeric = stats
        .iter()
        .any(|s| s.r#type == "Integer" || s.r#type == "Float");
    let has_quartiles = stats.iter().any(|s| s.q2_median.is_some());
    has_mode && (!has_numeric || has_quartiles)
}

/// Do the parsing options recorded in a stats cache's `<input>.stats.csv.json` metadata match the
/// options the consuming command is using?
///
/// The cache is located by input path and validated by mtime, NOT by parsing options — so a cache
/// built under a different `--no-headers` / `--delimiter` can pass every freshness check while
/// describing a DIFFERENT logical CSV: field names become positional (`0`, `1`, …), the header row
/// is counted as data, and the column count can change entirely. Reusing it silently yields wrong
/// stats, or a hard failure downstream when the column count disagrees.
///
/// Shared by `get_stats_records` and `get_stats_records_readonly` deliberately: those two drifted
/// (only the latter validated this), which is precisely how the mismatch became reachable.
///
/// Returns `false` when the metadata is missing or unreadable, so callers regenerate rather than
/// trust a cache they cannot verify.
pub(super) fn stats_cache_parsing_opts_match(
    metadata: &serde_json::Value,
    input_path: &Path,
    no_headers: bool,
    delimiter: Option<Delimiter>,
    prefer_dmy: Option<bool>,
    flexible: bool,
) -> bool {
    // --flexible is a parsing option too: a ragged file can ONLY be summarized under it, since
    // a strict `stats` run errors out before it writes anything. So a cache recording
    // `flag_flexible: true` may describe records a strict consumer would refuse outright.
    //
    // The check is deliberately ONE-WAY - it refuses a flexible cache to a strict consumer, but
    // lets a flexible consumer reuse a strict cache. The converse would make flexible and strict
    // consumers of the same unchanged file invalidate each other's cache on every invocation,
    // forever, exactly like the raw --prefer-dmy comparison did (see `effective_prefer_dmy` in
    // `get_stats_records_flexible`). It also costs nothing in correctness: a strict cache exists
    // only because a strict run succeeded, which means the input had no ragged records, which
    // means --flexible would have computed the very same numbers.
    if !flexible
        && metadata
            .get("flag_flexible")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    {
        return false;
    }

    // --no-headers determines whether the header row is part of the data (and therefore whether
    // field names are real or positional), so it must match the consuming command.
    if metadata
        .get("flag_no_headers")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
        != no_headers
    {
        return false;
    }

    // --prefer-dmy decides how AMBIGUOUS dates are parsed: 01/02/2020 is Jan 2 under mdy and
    // Feb 1 under dmy. A cache built under one must not be served to a command using the
    // other, or its date min/max (and everything derived from them) describes different
    // instants than the consumer believes. stats.rs's own reuse path already compares this;
    // this check exists because get_stats_records short-circuits before that path ever runs.
    //
    // `None` means the consumer does no date inference (extsort, sortcheck), so the flag is
    // irrelevant to it and must not force needless cache regeneration.
    if let Some(prefer_dmy) = prefer_dmy
        && metadata
            .get("flag_prefer_dmy")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
            != prefer_dmy
    {
        return false;
    }

    // The effective delimiter must match. Both the cache and the command resolve an unset
    // delimiter from the same file path, so compare resolved bytes: an explicit recorded/passed
    // delimiter is a single ASCII byte, otherwise the extension default.
    let ext_delim = {
        let (_, d, _) = get_delim_by_extension(input_path, b',');
        d
    };
    let cache_delim = match metadata.get("flag_delimiter").and_then(|v| v.as_str()) {
        Some(s) if !s.is_empty() => s.as_bytes()[0],
        _ => ext_delim,
    };
    let cmd_delim = delimiter.map_or(ext_delim, |d| d.as_byte());
    cache_delim == cmd_delim
}

/// Was the stats cache beside `bases` built WITHOUT `--infer-dates`?
///
/// The `Frequency` and `FrequencyForceStats` modes run `stats --cardinality --stats-jsonl` with
/// no `--infer-dates`, so the cache they leave behind types every date column as `String`. That
/// cache is keyed only by input path and mtime, so a later `qsv schema` / `qsv tojsonl` on the
/// same unchanged file reused it and emitted `string` with no format for genuine date columns -
/// silently disagreeing with what those commands produce on a cold cache, and contradicting
/// tojsonl's own usage text, which promises reuse only of caches built with `--cardinality` AND
/// `--infer-dates`.
///
/// Consistent with `stats_cache_parsing_opts_conflict`: only a positive, readable
/// `"flag_infer_dates": false` counts. A missing or unparseable sidecar means "no opinion", so a
/// sidecar-less `moarstats`-built cache is never discarded on these grounds.
fn stats_cache_lacks_infer_dates(bases: [&Path; 2]) -> bool {
    bases.iter().any(|base| {
        std::fs::read_to_string(base.with_extension("stats.csv.json"))
            .ok()
            .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
            .and_then(|m| {
                m.get("flag_infer_dates")
                    .and_then(serde_json::Value::as_bool)
            })
            == Some(false)
    })
}

/// Does the stats cache's metadata sidecar record an input SIZE that disagrees with the input
/// on disk right now?
///
/// `stats` itself compares this before reusing its CSV cache, because mtime alone is defeated by
/// any mtime-preserving replacement (`cp -p`, `tar -x`, `git checkout`, `rsync -t`). Cache
/// CONSUMERS read the `.stats.csv.data.jsonl` through `get_stats_records`, which judged it by
/// mtime and parsing options only - so the same swap that `stats` now rejects still served
/// consumers the previous file's statistics.
///
/// Same convention as its siblings: only a positive, readable disagreement counts. A missing or
/// unparseable sidecar, or one without `filesize_bytes`, means "no opinion", so the deliberate
/// sidecar-less `moarstats` cache is never discarded on these grounds.
fn stats_cache_filesize_conflict(input_path: &str, bases: [&Path; 2]) -> bool {
    let Ok(input_len) = std::fs::metadata(input_path).map(|m| m.len()) else {
        return false;
    };

    bases.iter().any(|base| {
        std::fs::read_to_string(base.with_extension("stats.csv.json"))
            .ok()
            .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
            .and_then(|m| m.get("filesize_bytes").and_then(serde_json::Value::as_u64))
            .is_some_and(|recorded| recorded != 0 && recorded != input_len)
    })
}

/// Was the stats cache beside `input_path` built with APPROXIMATE cardinality
/// (`--cardinality-method approx`, i.e. `HyperLogLog`)?
///
/// HLL carries roughly +/-1.5% error, so a cached cardinality must not be compared for EQUALITY
/// against an exact row count. `frequency` does exactly that to detect all-unique (ID) columns and
/// collapse them to a single `<ALL_UNIQUE>` sentinel. Against an approximate cardinality that
/// comparison fails both ways: a genuine ID column almost never lands exactly on the row count, so
/// the short-circuit is lost and a full per-column hashmap is built (a performance cliff on the
/// very columns the short-circuit exists for); and a merely near-unique column whose estimate
/// happens to land on the row count is wrongly collapsed, **dropping real frequency rows**.
///
/// Same convention as the sibling sidecar checks: only a positive, readable `"approx"` counts. A
/// missing or unparseable sidecar means "no opinion", so a cache without provenance is treated as
/// exact - which is what it was before `--cardinality-method` existed.
pub fn stats_cache_cardinality_is_approx(input_path: Option<&str>) -> bool {
    let Some(raw) = input_path else {
        // stdin has no stable path to key a cache on
        return false;
    };

    // Resolve a `dc:<name>` disk-cache handle to its materialized CSV path, exactly as
    // get_stats_records does before loading the cache. Without this the caller looks beside the
    // literal "dc:name" string, finds no sidecar, and concludes the cardinality is exact - so a
    // `dc:` input silently kept the invalid shortcut this check exists to suppress.
    #[cfg(feature = "get")]
    let resolved: String = if let Some(dc_name) = raw.strip_prefix("dc:") {
        match crate::diskcache::resolve_dc_path(dc_name) {
            Ok(p) => p.to_string_lossy().into_owned(),
            // unresolvable handle: no cache to judge
            Err(_) => return false,
        }
    } else {
        raw.to_string()
    };
    #[cfg(not(feature = "get"))]
    let resolved: String = raw.to_string();

    let path = Path::new(&resolved);
    let canonical = path.canonicalize().ok();

    [Some(path), canonical.as_deref()]
        .into_iter()
        .flatten()
        .any(|base| {
            std::fs::read_to_string(base.with_extension("stats.csv.json"))
                .ok()
                .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
                .and_then(|m| {
                    m.get("flag_cardinality_method")
                        .and_then(|v| v.as_str().map(str::to_owned))
                })
                .is_some_and(|m| m.eq_ignore_ascii_case("approx"))
        })
}

/// Is a `<FILESTEM>.stats.csv` present AND newer than the input it describes?
///
/// `moarstats` and `pragmastat` locate the baseline stats CSV by path and regenerate it when
/// absent - but they checked EXISTENCE only. A stats CSV left over from an earlier version of the
/// input was therefore accepted as the baseline for every statistic they derive, silently
/// describing data that is no longer there. This is the same first gate `stats` applies to its
/// own cache.
///
/// mtime only: the recorded-size cross-check lives in the `.stats.csv.json` sidecar, which these
/// callers do not require (`moarstats` deliberately writes none).
///
/// A stats CSV we cannot stat is "not current" - regenerate rather than trust it. An INPUT we
/// cannot stat leaves the prior behavior alone, so a transient failure cannot trigger an
/// expensive rebuild.
pub fn stats_csv_is_current(stats_csv_path: &Path, input_path: &Path) -> bool {
    let Ok(stats_modified) = std::fs::metadata(stats_csv_path).and_then(|m| m.modified()) else {
        return false;
    };
    let Ok(input_modified) = std::fs::metadata(input_path).and_then(|m| m.modified()) else {
        return true;
    };
    stats_modified > input_modified
}

/// Was the `<FILESTEM>.stats.csv` beside this path written by a DIFFERENT qsv version?
///
/// `moarstats` and `pragmastat` append derived columns to the stats CSV and skip any column that
/// already exists, so an extended cache built by an older qsv keeps that version's values forever
/// once its mtime is current - including values a later release fixed (e.g. `jarque_bera` and
/// `bimodality_coefficient` before #4651). `stats` itself invalidates on `qsv_version`; this lets
/// those callers apply the same rule, and the `stats` rerun it triggers rewrites the CSV and its
/// sidecar, dropping every appended column.
///
/// Only a readable sidecar counts. No sidecar, or one that does not parse, is "no opinion",
/// matching `stats_cache_parsing_opts_conflict`. A parsed sidecar without a `qsv_version` string IS
/// a mismatch, because `stats` treats it that way too and would recompute regardless.
pub fn stats_csv_version_mismatch(stats_csv_path: &Path) -> bool {
    std::fs::read_to_string(stats_csv_path.with_extension("csv.json"))
        .ok()
        .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
        .is_some_and(|m| {
            m.get("qsv_version").and_then(serde_json::Value::as_str)
                != Some(env!("CARGO_PKG_VERSION"))
        })
}

/// Does the stats cache's metadata sidecar POSTDATE the `.stats.csv.data.jsonl` beside it?
///
/// If it does, the JSONL was produced by an earlier stats run and no longer describes the stats
/// now on disk. A recompute that does not pass `--stats-jsonl` refreshes `<stem>.stats.csv` and
/// `<stem>.stats.csv.json` but leaves the JSONL untouched - and since the JSONL is judged current
/// by mtime against the INPUT, which a recompute never touches, it would otherwise be served
/// indefinitely. Observed: an `--infer-dates` JSONL saying `when=Date` surviving beside a freshly
/// computed `stats.csv` saying `when=String`, and being consumed by `schema`.
///
/// Two cases deliberately do NOT count as stale, both for the same reason `moarstats` writes the
/// JSONL from its EXTENDED stats and writes no sidecar of its own:
///   * **no sidecar at all** - no opinion, never "stale". Treating a sidecar-less JSONL as unusable
///     would discard that richer externally-built cache and regenerate a leaner one.
///   * **JSONL newer than the sidecar** - that is exactly the `stats` then `moarstats` ordering,
///     where the JSONL is the later and richer artifact.
///
/// Both locations are checked for the same reason `stats_cache_parsing_opts_conflict` checks
/// both: for a symlinked input they differ.
pub(super) fn stats_jsonl_predates_stats_cache(statsdata_path: &Path, bases: [&Path; 2]) -> bool {
    let Ok(jsonl_meta) = std::fs::metadata(statsdata_path) else {
        return false;
    };
    let jsonl_mtime = FileTime::from_last_modification_time(&jsonl_meta);

    bases.iter().any(|base| {
        std::fs::metadata(base.with_extension("stats.csv.json"))
            .is_ok_and(|m| FileTime::from_last_modification_time(&m) > jsonl_mtime)
    })
}

/// Does a stats cache's metadata sidecar record parsing options that CONFLICT with the consuming
/// command's? Only a positive, readable mismatch counts as a conflict.
///
/// Absent or unparseable metadata is deliberately NOT a conflict. `moarstats` writes the
/// `.stats.csv.data.jsonl` cache without a companion `.stats.csv.json`, and treating those as
/// unusable would silently discard a rich externally-built cache and regenerate a leaner one —
/// downgrading consumers (e.g. `pivotp --agg smart` loses the `sort_order` it decides on). The
/// mismatch this guards against always writes a sidecar, since it goes through the `stats`
/// subprocess, so declining to judge a sidecar-less cache leaves the pre-existing status quo
/// rather than a new hole.
///
/// (`get_stats_records_readonly` still fails closed on missing metadata: it returns an `Option`
/// its callers treat as a best-effort accelerator, so refusing costs nothing there.)
pub(super) fn stats_cache_parsing_opts_conflict(
    input_path: &Path,
    canonical_input_path: &Path,
    no_headers: bool,
    delimiter: Option<Delimiter>,
    prefer_dmy: Option<bool>,
    flexible: bool,
) -> bool {
    // Both locations must be checked, because for a SYMLINKED input they differ: the JSONL cache
    // is looked up beside the canonicalized target, but the `stats` subprocess is invoked with the
    // original path and writes its `<input>.stats.csv.json` beside the symlink. Looking only
    // beside the canonical target finds no metadata there and waves the cache through — which
    // reopened the exact mismatch this guard exists to catch, via a symlink.
    //
    // Delimiter resolution uses `input_path`, the path actually parsed: `Config` resolves an unset
    // delimiter from that path's extension, so a `link.csv -> target.tsv` symlink must be judged
    // by the extension the reader used, not the target's.
    for meta_base in [input_path, canonical_input_path] {
        let metadata_path = meta_base.with_extension("stats.csv.json");
        let Ok(text) = std::fs::read_to_string(&metadata_path) else {
            continue;
        };
        let Ok(metadata) = serde_json::from_str::<serde_json::Value>(&text) else {
            continue;
        };
        if !stats_cache_parsing_opts_match(
            &metadata, input_path, no_headers, delimiter, prefer_dmy, flexible,
        ) {
            return true;
        }
    }
    false
}

/// Keep the stats cache pair (`.stats.csv` + its `.stats.csv.json` metadata sidecar) beside the
/// CANONICAL target in sync with the pair beside the path as GIVEN.
///
/// Stats-cache readers look up `.stats.csv.data.jsonl` beside the canonicalized input, but `stats`
/// and `moarstats` write their cache files beside the path they were given. Those coincide for an
/// ordinary input and diverge for a symlink — and a divergence is not merely a missing sidecar:
/// an earlier direct run on the target leaves a canonical sidecar that keeps describing itself as
/// current while a run via the link replaces the canonical JSONL underneath it. A later direct run
/// then reads stale-but-matching metadata and reuses a cache built with different parsing options.
///
/// The `.stats.csv` travels WITH its sidecar: replacing only the metadata would let `stats` itself
/// validate the canonical `.stats.csv` from a different run against the copied args and serve it.
/// When there is no given-side `.stats.csv` to copy, the canonical one is removed instead. The
/// canonical JSONL is invalidated FIRST (if that fails, nothing is touched and the old provenance
/// stands), then the canonical sidecar, so a failure part-way leaves no metadata vouching for a
/// mismatched `.stats.csv` and no JSONL whose provenance is gone. Callers write the canonical JSONL
/// afterwards, and must not leave it published when this returns `false`. The `.stats.csv`
/// copy goes through `DerivedFile`, like every other stats cache, so it never grants access the
/// input does not.
///
/// Call this BEFORE writing the canonical JSONL: `stats_jsonl_predates_stats_cache` rejects a JSONL
/// older than either sidecar, and `fs::copy` stamps a fresh mtime on Linux.
///
/// Returns whether the canonical pair now mirrors the given one (trivially `true` when there is
/// nothing to sync). Never errors: a stale sidecar is only a cache-reuse hint, so a failure is
/// logged and reported through the return value.
pub fn sync_stats_cache_to_canonical(input_path: &Path, canonical_input_path: &Path) -> bool {
    let given_metadata = input_path.with_extension("stats.csv.json");
    let canonical_metadata = canonical_input_path.with_extension("stats.csv.json");
    // Compare RESOLVED paths, not the raw ones: for an ordinary input these name the same
    // file via a relative and an absolute path, and `fs::copy` onto itself truncates it —
    // which corrupts the very metadata this is meant to keep trustworthy.
    let same_file = match (
        given_metadata.canonicalize(),
        canonical_metadata.canonicalize(),
    ) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    };
    if same_file || !given_metadata.exists() {
        return true;
    }

    let remove_if_present = |p: &Path| match std::fs::remove_file(p) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(CliError::from(e)),
        _ => Ok(()),
    };
    let given_stats = input_path.with_extension("stats.csv");
    let canonical_stats = canonical_input_path.with_extension("stats.csv");
    let synced = remove_if_present(&canonical_input_path.with_extension("stats.csv.data.jsonl"))
        .and_then(|()| remove_if_present(&canonical_metadata))
        .and_then(|()| {
            if given_stats.exists() {
                let mut installed =
                    DerivedFile::create(canonical_input_path, &canonical_stats, "statscache")?;
                std::io::copy(&mut File::open(&given_stats)?, &mut installed)?;
                installed.install()
            } else {
                remove_if_present(&canonical_stats)
            }
        })
        .and_then(|()| {
            std::fs::copy(&given_metadata, &canonical_metadata)?;
            Ok(())
        });
    if let Err(e) = synced {
        log::warn!(
            "could not sync stats cache beside {}: {e}",
            canonical_input_path.display()
        );
        return false;
    }
    true
}

/// For a symlinked input, also write the `.stats.csv.data.jsonl` cache (and sync the stats cache
/// pair it was built from) beside the CANONICAL target — the location every stats-cache reader
/// looks it up — since `stats`/`moarstats` write their cache files beside the path as given
/// (#4697). Only for the UNWEIGHTED cache: readers never consult `.stats.weighted.csv*`, so callers
/// must not pass a weighted run.
///
/// `jsonl_written` is the JSONL the caller just wrote beside `input_path`, converted from
/// `stats_csv`; `permission_source` is passed through to `csv_to_jsonl`. A no-op when the two
/// locations resolve to the same file. Best-effort: a failure is logged, never returned, since the
/// caller's own cache was written and the reader falls back to regenerating.
pub fn mirror_stats_jsonl_to_canonical(
    stats_csv: &str,
    jsonl_written: &Path,
    input_path: &Path,
    permission_source: &Path,
) {
    let Ok(canonical_input) = input_path.canonicalize() else {
        return;
    };
    let canonical_jsonl = canonical_input.with_extension("stats.csv.data.jsonl");
    if jsonl_written.canonicalize().ok().as_deref() == Some(canonical_jsonl.as_path()) {
        return;
    }
    // sidecars first, so the JSONL written below is the newest of the trio; if they could not be
    // synced, publish nothing beside the target
    if !sync_stats_cache_to_canonical(input_path, &canonical_input) {
        return;
    }
    if let Err(e) = csv_to_jsonl(
        stats_csv,
        &crate::cmd::stats::STATSDATA_TYPES_MAP,
        &canonical_jsonl,
        b',',
        permission_source,
    ) {
        log::warn!(
            "could not write stats JSONL cache beside symlink target {}: {e}",
            canonical_jsonl.display()
        );
    }
}

/// Get the stats records for `args`, reading or regenerating the stats cache as needed.
///
/// `SchemaArgs` is `schema`'s own docopt target, so it cannot carry a `--flexible` field
/// (docopt rejects a struct field with no matching flag in the USAGE). Callers that accept
/// ragged input use `get_stats_records_flexible` instead.
pub fn get_stats_records(
    args: &SchemaArgs,
    requested_mode: StatsMode,
) -> CliResult<(ByteRecord, Vec<StatsData>)> {
    get_stats_records_flexible(args, requested_mode, false)
}

/// `get_stats_records`, with control over whether the `qsv stats` subprocess is told to
/// accept records with a varying number of fields. `frequency --flexible` needs this: without
/// it the child would reject the same ragged file the parent was told to accept.
pub fn get_stats_records_flexible(
    args: &SchemaArgs,
    requested_mode: StatsMode,
    flexible: bool,
) -> CliResult<(ByteRecord, Vec<StatsData>)> {
    let env_mode = env::var("QSV_STATSCACHE_MODE")
        .unwrap_or_else(|_| DEFAULT_STATSCACHE_MODE.to_string())
        .to_ascii_lowercase();

    if !["auto", "force", "none"].contains(&env_mode.as_str()) {
        return fail_incorrectusage_clierror!(
            "Invalid QSV_STATSCACHE_MODE value: {env_mode}. Must be one of: auto, force, none"
        );
    }

    if requested_mode == StatsMode::None
        || env_mode == "none"
        || args.arg_input.is_none()
        || args.arg_input.as_ref() == Some(&"-".to_string())
        // safety: we know that by this point, args.arg_input is not None as
        // the earlier is_none() check would have short-circuited already
        || get_special_format(Path::new(args.arg_input.as_ref().unwrap())) != SpecialFormat::Unknown
    {
        // if stdin or StatsMode::None,
        // we're just doing frequency old school w/o cardinality
        return Ok((ByteRecord::new(), Vec::new()));
    }

    let input_path_raw = args.arg_input.as_ref().ok_or("No input provided")?;
    // Resolve a `dc:<name>` disk-cache handle (the `get` command's cache) to its
    // materialized CSV path: the stats cache is keyed/located by the concrete
    // file path, so `canonicalize`, the `.stats.csv.data.jsonl` sidecar location,
    // and the stats subprocess input all need a real path, not the "dc:" string.
    #[cfg(feature = "get")]
    let input_path_owned: String = if let Some(dc_name) = input_path_raw.strip_prefix("dc:") {
        crate::diskcache::resolve_dc_path(dc_name)?
            .to_string_lossy()
            .into_owned()
    } else {
        input_path_raw.clone()
    };
    #[cfg(not(feature = "get"))]
    let input_path_owned: String = input_path_raw.clone();
    let input_path = &input_path_owned;
    let canonical_input_path = Path::new(input_path).canonicalize()?;
    let statsdata_path = canonical_input_path.with_extension("stats.csv.data.jsonl");

    let stats_data_current = if statsdata_path.exists() {
        let statsdata_metadata = std::fs::metadata(&statsdata_path)?;

        let input_metadata = std::fs::metadata(input_path)?;

        let statsdata_mtime = FileTime::from_last_modification_time(&statsdata_metadata);
        let input_mtime = FileTime::from_last_modification_time(&input_metadata);
        // Does THIS mode actually infer dates? Mirrors the per-mode argv built below: Schema
        // and ProfileSchema always pass --infer-dates, PolarsSchema only when it has a
        // whitelist (i.e. sniff found date columns), and the Frequency modes never do.
        //
        // Used for both date-sensitive checks below. A mode that does not infer dates cannot
        // be affected by --prefer-dmy or by a date-blind cache, so neither may force it to
        // regenerate.
        let mode_infers_dates = match requested_mode {
            StatsMode::Schema | StatsMode::ProfileSchema => true,
            #[cfg(feature = "polars")]
            StatsMode::PolarsSchema => !args.flag_dates_whitelist.is_empty(),
            _ => false,
        };

        // Compare the EFFECTIVE dmy preference. The stats subprocess folds QSV_PREFER_DMY in
        // (stats.rs does `flag_prefer_dmy || get_envvar_flag("QSV_PREFER_DMY")`), so a sidecar
        // written under that env var records `true` -- while consumers that build SchemaArgs
        // literally (frequency, joinp, pivotp, sample) hardcode `flag_prefer_dmy: false`.
        // Comparing the raw flag made every one of those runs mismatch its own cache and
        // regenerate it, on every invocation, forever.
        let effective_prefer_dmy = if mode_infers_dates {
            Some(args.flag_prefer_dmy || get_envvar_flag("QSV_PREFER_DMY"))
        } else {
            None
        };

        if statsdata_mtime > input_mtime {
            // mtime alone is not sufficient: the cache is NOT keyed by parsing options, so a
            // cache written by an earlier run with a different --no-headers/--delimiter would
            // otherwise be reused here and describe a different logical CSV.
            if stats_cache_parsing_opts_conflict(
                Path::new(input_path),
                &canonical_input_path,
                args.flag_no_headers,
                args.flag_delimiter,
                effective_prefer_dmy,
                flexible,
            ) {
                info!(
                    "stats.csv.data.jsonl file was built with different parsing options \
                     (--no-headers/--delimiter/--prefer-dmy/--flexible). Regenerating stats jsonl."
                );
                false
            } else if stats_jsonl_predates_stats_cache(
                &statsdata_path,
                [Path::new(input_path), &canonical_input_path],
            ) {
                info!(
                    "stats.csv.data.jsonl file predates the stats cache beside it. Regenerating \
                     stats jsonl."
                );
                false
            } else if stats_cache_filesize_conflict(
                input_path,
                [Path::new(input_path), &canonical_input_path],
            ) {
                // the input was replaced by content of a DIFFERENT size while keeping its
                // mtime (cp -p, tar -x, git checkout, rsync -t). `qsv stats` itself now
                // rejects its CSV cache on this, but consumers read the jsonl through this
                // path and would otherwise still be served the pre-swap statistics.
                info!(
                    "stats.csv.data.jsonl file describes an input of a different size. \
                     Regenerating stats jsonl."
                );
                false
            } else if mode_infers_dates
                && stats_cache_lacks_infer_dates([Path::new(input_path), &canonical_input_path])
            {
                info!(
                    "stats.csv.data.jsonl file was built without --infer-dates, so date columns \
                     are typed as strings. Regenerating stats jsonl."
                );
                false
            } else {
                info!("Valid stats.csv.data.jsonl file found!");
                true
            }
        } else {
            info!("stats.csv.data.jsonl file is older than input file. Regenerating stats jsonl.");
            false
        }
    } else {
        info!(
            "stats.csv.data.jsonl file does not exist: {}",
            statsdata_path.display()
        );
        false
    };

    // NOTE: Frequency mode used to bail out here with empty stats when the cache was not
    // current and `env_mode != "auto"`. env_mode is validated to auto/force/none and "none"
    // already returned above, so that condition meant exactly "force" - making
    // QSV_STATSCACHE_MODE=force STRICTLY WEAKER than the default: it skipped regeneration,
    // returned no cardinality (losing frequency's <ALL_UNIQUE> short-circuit) and created no
    // cache, while plain auto-mode regenerated and cached. That inverts the documented
    // contract ("force - if the cache does not exist, create it by running stats").
    // Both auto and force now fall through to the regeneration block below, which already
    // adds --force to the stats args for force mode. Only "none" skips the cache.

    // Use qsv's Config system to properly detect delimiter based on file extension
    let rconfig = Config::new(Some(input_path))
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);
    let mut rdr = rconfig.reader()?;
    // get the headers from the input file
    let csv_fields = rdr.byte_headers()?.clone();
    drop(rdr);

    // Update args with the detected delimiter if it wasn't explicitly set
    let detected_delimiter = if args.flag_delimiter.is_none() {
        let path = Path::new(input_path);
        let (_, detected_delim, _) = get_delim_by_extension(path, b',');
        Some(Delimiter(detected_delim))
    } else {
        args.flag_delimiter
    };

    let mut stats_data_loaded = false;
    let mut csv_stats: Vec<StatsData> = Vec::with_capacity(csv_fields.len());

    // if stats_data file exists and is current, use it
    if stats_data_current && !args.flag_force {
        let statsdatajson_rdr =
            BufReader::with_capacity(DEFAULT_RDR_BUFFER_CAPACITY, File::open(statsdata_path)?);

        let mut curr_line: String;
        let mut s_slice: Vec<u8>;

        for line in statsdatajson_rdr.lines() {
            curr_line = line?;
            s_slice = curr_line.as_bytes().to_vec();

            // Parse regular stats record
            let parse_result = cfg_select! {
                target_endian = "little" => simd_json::from_slice::<StatsData>(&mut s_slice),
                _ => serde_json::from_slice::<StatsData>(&s_slice),
            };

            if let Ok(mut stats) = parse_result {
                // a pre-fix cache carries fabricated 0.0 renderings on a date row; drop them
                // once here so accessors and re-serialization (profile's dpps) agree
                stats.normalize_stale_date_renderings();
                csv_stats.push(stats);
            } else {
                // if we encounter a parsing error, clear csv_stats and break
                // so that we regenerate the stats data
                csv_stats.clear();
                break;
            }
        }
        stats_data_loaded = !csv_stats.is_empty();

        // Mode-aware cache reuse: the cache is keyed only by mtime, not by
        // the StatsMode that produced it. A cache built by a leaner mode
        // (e.g. `qsv schema`, StatsMode::Schema — no --quartiles/--mode)
        // doesn't satisfy ProfileSchema's need for quartiles + mode. If the
        // loaded stats lack what the requested mode requires, discard them
        // so the regeneration path below recomputes with the right flags.
        if stats_data_loaded && !stats_satisfy_mode(&csv_stats, requested_mode) {
            csv_stats.clear();
            stats_data_loaded = false;
        }
    }

    // otherwise, run stats command to generate stats.csv.data.jsonl file
    if !stats_data_loaded {
        let stats_args = crate::cmd::stats::Args {
            // the dc:-resolved concrete path, so the stats subprocess reads the
            // real (materialized) file rather than re-resolving the "dc:" handle
            arg_input:                 Some(input_path.clone()),
            flag_select:               crate::select::SelectColumns::parse("").unwrap(),
            flag_everything:           false,
            flag_typesonly:            false,
            flag_infer_boolean:        false,
            flag_boolean_patterns:     String::new(),
            flag_mode:                 false,
            flag_cardinality:          true,
            flag_zero_padded_numeric:  false,
            flag_median:               false,
            flag_quartiles:            false,
            flag_mad:                  false,
            flag_percentiles:          false,
            flag_percentile_list:      "5,10,40,60,90,95".to_string(),
            flag_quantile_method:      "exact".to_string(),
            flag_cardinality_method:   "exact".to_string(),
            flag_mode_cardinality_cap: 0,
            flag_nulls:                false,
            flag_round:                4,
            flag_infer_dates:          true,
            flag_dates_whitelist:      args.flag_dates_whitelist.to_string(),
            flag_prefer_dmy:           args.flag_prefer_dmy,
            flag_force:                args.flag_force,
            flag_jobs:                 Some(njobs(args.flag_jobs)),
            flag_stats_jsonl:          true,
            flag_jsonl:                false,
            flag_pretty_json:          false,
            flag_cache_threshold:      1, // force the creation of stats cache files
            flag_output:               None,
            flag_no_headers:           args.flag_no_headers,
            flag_delimiter:            detected_delimiter,
            flag_flexible:             flexible,
            flag_memcheck:             args.flag_memcheck,
            flag_vis_whitespace:       false,
            flag_weight:               None,
        };

        let tempfile = tempfile::Builder::new().suffix(".stats.csv").tempfile()?;
        // safety: we just created a tempfile, which is guaranteed to have a path
        let tempfile_path = tempfile.path().to_str().unwrap().to_string();

        let statsdatajson_path = &canonical_input_path.with_extension("stats.csv.data.jsonl");

        let input = stats_args.arg_input.unwrap_or_else(|| "-".to_string());

        // for the date-inferring modes (Schema/ProfileSchema), default an unspecified
        // whitelist to "sniff" so date/datetime columns are auto-detected rather than left as
        // strings. An explicit caller value (e.g. `qsv schema --dates-whitelist`, profile's
        // preset, viz's "sniff") still wins.
        let dates_whitelist = if stats_args.flag_dates_whitelist.is_empty() {
            "sniff"
        } else {
            stats_args.flag_dates_whitelist.as_str()
        };

        // we do rustfmt::skip here as it was breaking the stats cmdline along strange
        // boundaries, causing CI errors.
        // This is because we're using tab characters (/t) to separate args to fix #2294,
        #[rustfmt::skip]
        let mut stats_args_str = match requested_mode {
            StatsMode::Schema => {
                // mode is StatsMode::Schema
                // we're generating schema, so we need cardinality and to infer-dates
                format!(
                    "stats\t{input}\t--round\t4\t--cardinality\
                    \t--infer-dates\t--dates-whitelist\t{dates_whitelist}\
                    \t--stats-jsonl\t--force\t--output\t{tempfile_path}"
                )
            },
            StatsMode::ProfileSchema => {
                // mode is StatsMode::ProfileSchema
                // same as Schema, plus quartiles (q1/median/q3) and mode so
                // `qsv profile`'s descriptive-statistics projection has the
                // full extended stat set without a pre-built stats cache, and
                // zero-padded-numeric detection (a streaming, constant-memory
                // stat) so `viz smart` can route zip/FIPS/ICD-style code
                // columns as dimensions instead of measures.
                format!(
                    "stats\t{input}\t--round\t4\t--cardinality\
                    \t--quartiles\t--mode\t--zero-padded-numeric\
                    \t--infer-dates\t--dates-whitelist\t{dates_whitelist}\
                    \t--stats-jsonl\t--force\t--output\t{tempfile_path}"
                )
            },
            StatsMode::Frequency => {
                // StatsMode::Frequency
                // we're doing frequency, so we just need cardinality
                format!("stats\t{input}\t--cardinality\t--stats-jsonl\t--output\t{tempfile_path}")
            },
            StatsMode::FrequencyForceStats => {
                // StatsMode::FrequencyForceStats
                // we're doing frequency, so we need cardinality from a --forced stats run
                format!(
                    "stats\t{input}\t--cardinality\t--stats-jsonl\t--force\t--output\t{tempfile_path}"
                )
            },
            #[cfg(feature = "polars")]
            StatsMode::PolarsSchema => {
                // StatsMode::PolarsSchema
                // we need data types, ranges & cardinality
                // if sniff detected date columns, also infer dates with the sniffed whitelist
                if stats_args.flag_dates_whitelist.is_empty() {
                    format!("stats\t{input}\t--cardinality\t--stats-jsonl\t--output\t{tempfile_path}")
                } else {
                    format!(
                        "stats\t{input}\t--cardinality\
                        \t--infer-dates\t--dates-whitelist\t{dates_whitelist}\
                        \t--stats-jsonl\t--output\t{tempfile_path}",
                        dates_whitelist = stats_args.flag_dates_whitelist
                    )
                }
            },
            StatsMode::Outliers => {
                // StatsMode::Outliers
                // we need data types, ranges, cardinality, quartiles, mad and modes/antimodes
                format!("stats\t{input}\t--cardinality\t--quartiles\t--mad\t--mode\t--stats-jsonl\t--output\t{tempfile_path}")
            },
            StatsMode::None => unreachable!(), // we returned early on None earlier
        };
        if args.flag_prefer_dmy {
            stats_args_str = format!("{stats_args_str}\t--prefer-dmy");
        }
        if args.flag_no_headers {
            stats_args_str = format!("{stats_args_str}\t--no-headers");
        }
        // without this the child would reject the same ragged file the parent was told to
        // accept, so `frequency --flexible` would still fail on the stats-cache path
        if flexible {
            stats_args_str = format!("{stats_args_str}\t--flexible");
        }

        // Use the detected delimiter
        stats_args_str = format!("{stats_args_str}\t--delimiter\t{}", {
            // safety: we know it's Some because we set it above
            let delim_to_use = detected_delimiter.unwrap().as_byte();
            if delim_to_use == b'\t' {
                r"\t".to_string()
            } else {
                (delim_to_use as char).to_string()
            }
        });

        if args.flag_memcheck {
            stats_args_str = format!("{stats_args_str}\t--memcheck");
        }
        if let Some(jobs) = stats_args.flag_jobs {
            stats_args_str = format!("{stats_args_str}\t--jobs\t{jobs}");
        }
        if stats_args.flag_nulls {
            stats_args_str = format!("{stats_args_str}\t--nulls");
        }

        if env_mode == "force" && !stats_args_str.contains("--force") {
            stats_args_str = format!("{stats_args_str}\t--force");
        }

        let stats_args_vec: Vec<&str> = stats_args_str.split('\t').collect();

        let qsv_bin = current_exe()?;
        let mut stats_cmd = std::process::Command::new(qsv_bin);
        if requested_mode == StatsMode::Outliers {
            // set the max length for antimodes
            stats_cmd.env("QSV_ANTIMODES_LEN", "0").args(stats_args_vec);
        } else {
            stats_cmd.args(stats_args_vec);
        }
        let stats_output = stats_cmd.output()?;
        let status = stats_output.status;
        if !status.success() {
            // Relay the child's own diagnostic. Without it the caller (schema, viz smart,
            // frequency's stats cache) only reports an exit code, so a CSV error the child
            // explained perfectly well reaches the user as "exited with code: 1" (#4611).
            let child_stderr = String::from_utf8_lossy(&stats_output.stderr);
            let child_stderr = child_stderr.trim();
            let detail = if child_stderr.is_empty() {
                String::new()
            } else {
                format!("\n{child_stderr}")
            };

            let status_code = status.code();
            if let Some(code) = status_code {
                return Err(CliError::Other(format!(
                    "qsv stats exited with code: {code}{detail}"
                )));
            }
            return Err(CliError::Other(cfg_select! {
                target_family = "unix" => match status.signal() {
                    Some(signal) => {
                        format!("qsv stats terminated with signal: {signal}{detail}")
                    },
                    None => format!("qsv stats terminated by unknown cause{detail}"),
                },
                _ => {
                    format!("qsv stats terminated by unknown cause{detail}")
                },
            }));
        }

        // sync the canonical stats cache pair BEFORE writing the JSONL beside it, so the JSONL is
        // not older than the sidecar `stats_jsonl_predates_stats_cache` compares it against
        let canonical_synced =
            sync_stats_cache_to_canonical(Path::new(input_path), &canonical_input_path);

        // create a stats data jsonl from the output of the stats command
        // stats command always writes comma-delimited CSV to tempfile,
        // so b',' is correct here regardless of the original input delimiter
        csv_to_jsonl(
            &tempfile_path,
            &STATSDATA_TYPES_MAP,
            statsdatajson_path,
            b',',
            &canonical_input_path,
        )?;

        let statsdatajson_rdr =
            BufReader::with_capacity(DEFAULT_RDR_BUFFER_CAPACITY, File::open(statsdatajson_path)?);

        let mut curr_line: String;
        let mut s_slice: Vec<u8>;
        for line in statsdatajson_rdr.lines() {
            curr_line = line?;
            s_slice = curr_line.as_bytes().to_vec();

            // Parse regular stats record
            let parse_result = cfg_select! {
                target_endian = "little" => simd_json::from_slice::<StatsData>(&mut s_slice),
                _ => serde_json::from_slice::<StatsData>(&s_slice),
            };

            match parse_result {
                Ok(mut stats) => {
                    stats.normalize_stale_date_renderings();
                    csv_stats.push(stats);
                },
                Err(e) => return Err(CliError::Other(format!("error parsing stats: {e}"))),
            }
        }

        // a failed sync removed the canonical sidecar: this run has its stats in memory, but a
        // JSONL left beside the target would carry no provenance for the next run to check
        if !canonical_synced {
            let _ = std::fs::remove_file(statsdatajson_path);
        }
    }

    if csv_stats.len() != csv_fields.len() {
        // stats cache is likely corrupted or truncated; fall back to empty stats
        return Ok((ByteRecord::new(), Vec::new()));
    }
    Ok((csv_fields.iter().take(csv_stats.len()).collect(), csv_stats))
}

/// Opportunistically load an EXISTING, CURRENT stats cache sidecar
/// (`<input>.stats.csv.data.jsonl`) WITHOUT ever spawning a `qsv stats`
/// subprocess. Used by stats-cache-aware commands (e.g. `extsort`, `sortcheck`)
/// to short-circuit work when the cache already proves a column's sort order.
///
/// Returns `None` (never an error) when the cache is absent, stale, disabled via
/// `QSV_STATSCACHE_MODE=none`, the input is stdin / a special format, or any
/// IO/parse problem occurs. Unlike [`get_stats_records`], it NEVER regenerates
/// the cache.
#[allow(clippy::missing_panics_doc)]
pub fn get_stats_records_readonly(
    input_path: Option<&str>,
    no_headers: bool,
    delimiter: Option<Delimiter>,
) -> Option<Vec<StatsData>> {
    // honor the stats-cache opt-out (same mechanism as frequency/schema/profile)
    let env_mode = env::var("QSV_STATSCACHE_MODE")
        .unwrap_or_else(|_| DEFAULT_STATSCACHE_MODE.to_string())
        .to_ascii_lowercase();
    if env_mode == "none" {
        return None;
    }

    let input_path_raw = input_path?;
    // reject stdin and special formats (snappy/zip/parquet/etc.) - the stats
    // cache is keyed/located by a concrete CSV/TSV path
    if input_path_raw == "-"
        || get_special_format(Path::new(input_path_raw)) != SpecialFormat::Unknown
    {
        return None;
    }

    // resolve a `dc:<name>` disk-cache handle (the `get` command's cache) to its
    // materialized CSV path, mirroring get_stats_records
    #[cfg(feature = "get")]
    let input_path_owned: String = if let Some(dc_name) = input_path_raw.strip_prefix("dc:") {
        crate::diskcache::resolve_dc_path(dc_name)
            .ok()?
            .to_string_lossy()
            .into_owned()
    } else {
        input_path_raw.to_string()
    };
    #[cfg(not(feature = "get"))]
    let input_path_owned: String = input_path_raw.to_string();

    let canonical_input_path = Path::new(&input_path_owned).canonicalize().ok()?;
    let statsdata_path = canonical_input_path.with_extension("stats.csv.data.jsonl");

    // the cache sidecar must exist AND be newer than the input file
    let statsdata_metadata = std::fs::metadata(&statsdata_path).ok()?;
    let input_metadata = std::fs::metadata(&input_path_owned).ok()?;
    let statsdata_mtime = FileTime::from_last_modification_time(&statsdata_metadata);
    let input_mtime = FileTime::from_last_modification_time(&input_metadata);
    if statsdata_mtime <= input_mtime {
        return None;
    }

    // Validate the companion args metadata (<input>.stats.csv.json). The JSONL
    // sidecar's sort_order / column-mapping is only valid for the parser options it
    // was generated with: a cache built with a different --no-headers, delimiter, or
    // a column --select can pass the mtime/count checks yet describe a different
    // logical CSV. Fail closed if the metadata is missing, unreadable, or mismatched.
    let metadata_path = canonical_input_path.with_extension("stats.csv.json");
    let metadata: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&metadata_path).ok()?).ok()?;

    // only a full-file stats cache can be trusted for positional column mapping
    // (a `--select`ed cache may drop or reorder columns relative to the header)
    if metadata.get("flag_select").and_then(|v| v.as_str()) != Some("<All>") {
        return None;
    }
    // The input must be the same SIZE, not merely older. mtime alone is defeated by any
    // mtime-preserving replacement (cp -p, tar -x, git checkout, rsync -t), and this cache
    // carries sort_order: consuming a stale one lets `extsort` skip a sort it still needs and
    // `sortcheck` report replaced, unsorted input as sorted.
    //
    // FAIL CLOSED, unlike the same check in `get_stats_records`. That asymmetry is deliberate:
    // there, absent metadata must mean "no opinion" so the sidecar-less cache `moarstats`
    // writes from its extended stats is not discarded. No such case reaches here - this path
    // already returns None when the sidecar is missing or unreadable, and requires a full-file
    // (`<All>`) cache - so a sidecar that is present but cannot substantiate the size is simply
    // unverifiable, and an unverifiable cache must not prove a file sorted. This also refuses a
    // recorded 0, which is never written (the real size is stored unconditionally before the
    // sidecar is), and so would only ever be a bypass.
    if !matches!(
        metadata
            .get("filesize_bytes")
            .and_then(serde_json::Value::as_u64),
        Some(recorded) if recorded == input_metadata.len()
    ) {
        return None;
    }
    // --no-headers and the effective delimiter must match the consuming command; shared with
    // `get_stats_records` so the two cannot drift apart again.
    if !stats_cache_parsing_opts_match(
        &metadata,
        Path::new(&input_path_owned),
        no_headers,
        delimiter,
        // extsort/sortcheck do no date inference, so --prefer-dmy cannot affect what they
        // read out of the cache
        None,
        // both read the input strictly, so a cache built from ragged data under --flexible
        // must not prove anything to them
        false,
    ) {
        return None;
    }

    // get the input header count so we can detect a truncated/corrupt cache
    let rconfig = Config::new(Some(&input_path_owned))
        .delimiter(delimiter)
        .no_headers_flag(no_headers);
    let header_count = rconfig.reader().ok()?.byte_headers().ok()?.len();

    // parse the jsonl sidecar - one StatsData record per line, in header order
    let statsdatajson_rdr = BufReader::with_capacity(
        DEFAULT_RDR_BUFFER_CAPACITY,
        File::open(&statsdata_path).ok()?,
    );
    let mut csv_stats: Vec<StatsData> = Vec::with_capacity(header_count);
    for line in statsdatajson_rdr.lines() {
        let curr_line = line.ok()?;
        if curr_line.trim().is_empty() {
            continue;
        }
        // Parse leniently via serde_json::Value rather than strictly into StatsData:
        // an opportunistic cache may have been produced by a lean `stats --stats-jsonl`
        // run that omits non-Option fields like `cardinality`, which would make a strict
        // StatsData deserialize fail. We only need a handful of fields to decide a sort
        // short-circuit, so we extract just those (defaulting the rest).
        let v: serde_json::Value = serde_json::from_slice(curr_line.as_bytes()).ok()?;
        let stats = StatsData {
            field: v
                .get("field")
                .and_then(|x| x.as_str())
                .unwrap_or_default()
                .to_string(),
            r#type: v.get("type").and_then(|x| x.as_str())?.to_string(),
            is_ascii: v
                .get("is_ascii")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false),
            sort_order: v
                .get("sort_order")
                .and_then(|x| x.as_str())
                .map(ToString::to_string),
            // nullcount is a base streaming stat always present; fail closed if absent
            nullcount: v.get("nullcount").and_then(serde_json::Value::as_u64)?,
            ..Default::default()
        };
        csv_stats.push(stats);
    }

    if csv_stats.is_empty() || csv_stats.len() != header_count {
        return None;
    }
    Some(csv_stats)
}

/// Does the stats cache *prove* the single selected column is already in
/// **ascending** order under the requested comparator semantics? Returns `false`
/// when not provably so (the caller must then fall back to the full operation,
/// which is always correct).
///
/// Only the ascending case is decided here. Descending (`--reverse`) is
/// deliberately not short-circuited: `extsort --reverse` reverses the full
/// `sort_key|position` key, so equal selected keys are emitted in reverse input
/// order (an anti-stable tie-break) that a straight passthrough cannot reproduce.
/// Ascending, by contrast, breaks ties by ascending position == input order,
/// which passthrough preserves exactly.
///
/// The cache's `sort_order` is computed using each column's inferred type
/// (String -> byte-lexicographic; Integer/Float -> numeric; Date/DateTime ->
/// chronological). A short-circuit is only safe when the caller's comparator
/// semantics exactly match the cache's per-column semantics, so ALL of the
/// following must hold:
/// - exactly one selected column (per-column order can't prove multi-column order)
/// - `nullcount == 0` (empty cells are excluded from `sort_order` but sort as Less)
/// - `sort_order` is present (all-null columns have none)
/// - type matches the requested comparator (and ASCII when `for_extsort`)
/// - the cached order is `Ascending`
///
/// `requested_numeric`: caller wants numeric comparison (vs. byte-lexicographic).
/// `for_extsort`: additionally require the column to be ASCII - extsort CSV mode
///   concatenates fields via lossy UTF-8, so only ASCII guarantees byte order
///   equals String order.
#[must_use]
pub fn stats_cache_proves_ascending(
    sel: &[usize],
    stats: &[StatsData],
    requested_numeric: bool,
    for_extsort: bool,
) -> bool {
    // a per-column cache can only prove single-column order
    if sel.len() != 1 {
        return false;
    }
    let Some(s) = stats.get(sel[0]) else {
        return false;
    };

    // empty/NULL cells are excluded from the cache's sort_order computation, but
    // the real comparators sort them as Less - so any nulls void the proof
    if s.nullcount != 0 {
        return false;
    }

    // all-null columns have no sort_order
    if s.sort_order.as_deref() != Some("Ascending") {
        return false;
    }

    // the cache's comparison semantics are type-dependent; they must match what
    // the caller will actually do
    if requested_numeric {
        matches!(s.r#type.as_str(), "Integer" | "Float")
    } else {
        s.r#type == "String" && (!for_extsort || s.is_ascii)
    }
}

/// Get the stats CSV file path for a given input CSV path.
/// Returns an absolute path like `/path/to/input.stats.csv`.
pub fn get_stats_csv_path(input_path: &std::path::Path) -> crate::CliResult<std::path::PathBuf> {
    let parent = input_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    let fstem = input_path.file_stem().ok_or_else(|| {
        crate::clitypes::CliError::Other("Invalid input path: no file name".to_string())
    })?;

    let stats_filename = format!("{}.stats.csv", fstem.to_string_lossy());
    let result = parent.join(stats_filename);
    if result.is_absolute() {
        Ok(result)
    } else {
        Ok(std::env::current_dir()?.join(result))
    }
}

/// Base `stats` options for a `moarstats --advanced` enrichment pass (`viz smart --smarter`,
/// `pivotp --agg smart`). Shared so both commands build - and reuse - the same persistent cache.
/// Must stay compatible with `StatsMode::ProfileSchema`, which writes that cache too.
/// `--infer-boolean` is deliberately absent: it would retype 0/1 measures as Boolean.
pub const ADVANCED_STATS_OPTIONS: &str = "--infer-dates --cardinality --mode --mad --quartiles \
                                          --percentiles --zero-padded-numeric --force \
                                          --stats-jsonl --dates-whitelist sniff";

/// `stats` columns an advanced-enriched cache must have (from `ADVANCED_STATS_OPTIONS`), and the
/// column only `moarstats --advanced` adds.
const ADVANCED_BASELINE_COLUMNS: [&str; 3] = ["q1", "mode_count", "mad"];
const ADVANCED_MARKER_COLUMN: &str = "moment_skewness";

/// Make sure `input`'s stats cache carries `moarstats --advanced` statistics, running moarstats
/// as a subprocess only when it does not. Returns whether moarstats ran.
///
/// Callers must use default CSV parsing: moarstats has no `--delimiter`/`--no-headers`, so it
/// resolves the delimiter from the file extension (as `Config` does).
///
/// Skipped (`Ok(false)`) for stdin, special formats (snappy, compressed, parquet, ...), and
/// `QSV_STATSCACHE_MODE=none`.
///
/// The decision is made from the cache's HEADER, not its values: a column can be legitimately
/// empty (e.g. no kurtosis for n < 4), and that must not trigger a rerun on every call.
/// moarstats appends to a current `<stem>.stats.csv` without recomputing it, so when the baseline
/// is lean - e.g. the `stats --cardinality` cache a `FrequencyForceStats` load leaves behind -,
/// stale, from another qsv version, built with approximate cardinality or non-default parsing
/// options, or its JSONL is missing or outdated, it runs with `--force` so the cache is rebuilt
/// with `ADVANCED_STATS_OPTIONS`.
pub fn ensure_advanced_stats_cache(input: &str) -> CliResult<bool> {
    let env_mode = env::var("QSV_STATSCACHE_MODE")
        .unwrap_or_else(|_| DEFAULT_STATSCACHE_MODE.to_string())
        .to_ascii_lowercase();
    if env_mode == "none" || input == "-" {
        return Ok(false);
    }
    let input_path = Path::new(input);
    if !input_path.is_file()
        || get_special_format(input_path) != SpecialFormat::Unknown
        || crate::config::is_snappy_extension(input_path)
    {
        return Ok(false);
    }

    let stats_csv = get_stats_csv_path(input_path)?;
    // A baseline parsed with non-default options (--no-headers, another delimiter) describes a
    // different logical CSV; moarstats would extend it and republish it under default-parsing
    // metadata, slipping past the loader's own parsing guard.
    let canonical = input_path
        .canonicalize()
        .unwrap_or_else(|_| input_path.to_path_buf());
    let current = stats_csv_is_current(&stats_csv, input_path)
        && !stats_csv_version_mismatch(&stats_csv)
        && !stats_cache_cardinality_is_approx(Some(input))
        && !stats_cache_parsing_opts_conflict(input_path, &canonical, false, None, None, false);
    let header: Vec<String> = File::open(&stats_csv)
        .ok()
        .and_then(|f| BufReader::new(f).lines().next())
        .and_then(Result::ok)
        .map(|line| line.split(',').map(str::to_string).collect())
        .unwrap_or_default();
    let has = |col: &str| header.iter().any(|h| h == col);
    let baseline_ok = ADVANCED_BASELINE_COLUMNS.iter().all(|c| has(c));
    // the JSONL is what consumers load; moarstats writes it after the stats CSV
    let jsonl_ok = {
        let jsonl = stats_csv.with_extension("csv.data.jsonl");
        match (
            std::fs::metadata(&jsonl).and_then(|m| m.modified()),
            std::fs::metadata(&stats_csv).and_then(|m| m.modified()),
        ) {
            (Ok(j), Ok(s)) => j >= s,
            _ => false,
        }
    };

    if current && baseline_ok && has(ADVANCED_MARKER_COLUMN) && jsonl_ok {
        return Ok(false);
    }

    let mut cmd = std::process::Command::new(current_exe()?);
    cmd.args([
        "moarstats",
        input,
        "--advanced",
        "--stats-options",
        ADVANCED_STATS_OPTIONS,
    ]);
    // moarstats only rewrites the JSONL when it adds columns, so an enriched CSV with a missing or
    // outdated JSONL needs --force too.
    if !(current && baseline_ok && jsonl_ok) {
        cmd.arg("--force");
    }
    let output = cmd.stdin(std::process::Stdio::null()).output()?;
    if output.status.success() {
        return Ok(true);
    }
    // relay the child's own diagnostic, as the stats subprocess does (#4611)
    let child_stderr = String::from_utf8_lossy(&output.stderr);
    let child_stderr = child_stderr.trim();
    let detail = if child_stderr.is_empty() {
        String::new()
    } else {
        format!("\n{child_stderr}")
    };
    Err(CliError::Other(match output.status.code() {
        Some(code) => format!("qsv moarstats exited with code: {code}{detail}"),
        None => format!("qsv moarstats terminated by a signal{detail}"),
    }))
}
