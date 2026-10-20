// only used by the hash_blake3_file tests, which are gated out of qsvlite
#[cfg(not(feature = "lite"))]
use std::io::Write;

use super::{fmt_duration_compact, is_io_print_panic};

#[test]
fn write_json_record_matching_widths() {
    let headers = csv::ByteRecord::from(vec!["a", "b"]);
    let record = csv::ByteRecord::from(vec!["1", ""]);
    let mut out = Vec::new();
    let mut is_first = true;
    super::write_json_record(&mut out, false, &headers, &record, &mut is_first).unwrap();
    assert_eq!(String::from_utf8(out).unwrap(), r#"{"a":"1","b":null}"#);
}

// the caller's reader decides whether rows can be ragged; a row wider than the headers
// used to index past the end of the header list
#[test]
fn write_json_record_rejects_more_fields_than_headers() {
    let headers = csv::ByteRecord::from(vec!["a"]);
    let record = csv::ByteRecord::from(vec!["1", "2", "3"]);
    let mut out = Vec::new();
    let mut is_first = true;
    let err =
        super::write_json_record(&mut out, false, &headers, &record, &mut is_first).unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    assert!(
        err.to_string().contains("more fields (3) than headers (1)"),
        "{err}"
    );
}

#[cfg(any(feature = "geocode", all(feature = "viz", feature = "feature_capable")))]
#[test]
fn api_key_is_redacted_from_anything_quoting_a_url() {
    use super::redact_api_key;
    // a credential must not survive into stderr, a log, or a pasted bug report. reqwest's own
    // error Display embeds the whole query string, so this is the last line of defence.
    //
    // The fixture is SYNTHETIC on purpose: never paste a captured secret in here. The natural
    // way to write this test is to copy the error you just reproduced, which is how a real key
    // once ended up committed inside the very fix for credential leaking.
    let leaked = concat!(
        "error sending request for url (https://api.census.gov/data/2024/acs/acs5",
        "?get=B01003_001E&for=state%3A01&key=EXAMPLE0000NOTAREALKEY0000EXAMPLE)"
    );
    let safe = redact_api_key(leaked);
    assert!(!safe.contains("EXAMPLE0000"), "{safe}");
    assert!(safe.contains("key=REDACTED"), "{safe}");
    // the rest of the URL survives, because it is what makes the error diagnosable
    assert!(safe.contains("for=state%3A01"), "{safe}");
    // a trailing paren ends the value rather than being swallowed
    assert!(safe.ends_with(')'), "{safe}");
    // multiple occurrences, and a key= that is not a query parameter
    assert_eq!(
        redact_api_key("?key=aaa&x=1 and ?key=bbb"),
        "?key=REDACTED&x=1 and ?key=REDACTED"
    );
    assert_eq!(redact_api_key("the monkey=banana"), "the monkey=banana");
    // a URL that a log wrapped: the key is no less secret for having a space before it
    assert_eq!(redact_api_key("...& key=abc123"), "...& key=REDACTED");
    assert_eq!(redact_api_key("no key here"), "no key here");
    // an OpenCage-shaped URL (key in the middle, more params after it)
    let oc = redact_api_key(
            "error sending request for url \
             (https://api.opencagedata.com/geocode/v1/json?q=Berlin&key=EXAMPLEOCKEY&limit=1)",
        );
    assert!(!oc.contains("EXAMPLEOCKEY"), "{oc}");
    assert!(oc.contains("key=REDACTED&limit=1"), "{oc}");
}

// One unit, the largest that fits, so the unit boundaries are the whole contract. Shared by
// the stale-refresh warning and the `qsv get cache-list` AGE column.
#[test]
fn fmt_duration_compact_picks_the_largest_fitting_unit() {
    assert_eq!(fmt_duration_compact(0), "0s");
    assert_eq!(fmt_duration_compact(59), "59s");
    assert_eq!(fmt_duration_compact(60), "1m");
    assert_eq!(fmt_duration_compact(3_599), "59m");
    assert_eq!(fmt_duration_compact(3_600), "1h");
    assert_eq!(fmt_duration_compact(86_399), "23h");
    assert_eq!(fmt_duration_compact(86_400), "1d");
    // the `qsv get` default TTL — days are the largest unit, so this is not "4w"
    assert_eq!(fmt_duration_compact(2_419_200), "28d");
}

// The panic-hook guard must intercept a failed `println!`/`eprintln!` (an I/O failure)
// while leaving every genuine qsv panic on the human_panic crash-report path. Matching is
// deliberately narrow, so both directions matter — a false positive here would swallow a
// real bug report.
#[test]
fn io_print_panics_are_recognized_and_nothing_else_is() {
    // the exact shapes std emits (issue #2661 reported the first one verbatim)
    assert!(is_io_print_panic(
        "failed printing to stdout: Broken pipe (os error 32)"
    ));
    assert!(is_io_print_panic(
        "failed printing to stderr: No space left on device (os error 28)"
    ));

    // a genuine qsv panic must still reach the crash report
    assert!(!is_io_print_panic(
        "called `Option::unwrap()` on a `None` value"
    ));
    assert!(!is_io_print_panic("index out of bounds: the len is 3"));
    assert!(!is_io_print_panic("attempt to divide by zero"));
    // near-misses: the prefix is anchored, so a message merely MENTIONING it is a bug
    assert!(!is_io_print_panic(
        "assertion failed: failed printing to stdout"
    ));
    assert!(!is_io_print_panic("failed printing to a file"));

    // the separator is matched too, so a hypothetical future panic that merely BEGINS
    // the same way keeps its crash report rather than being mistaken for a write failure
    assert!(!is_io_print_panic(
        "failed printing to stdout buffer invariant violated"
    ));
    assert!(!is_io_print_panic("failed printing to stdout"));
    assert!(!is_io_print_panic("failed printing to stderr, somehow"));
}

/// A non-2xx response must be an ERROR, never a file.
///
/// `download_file` streamed whatever came back, so a "Not Found" page was written to disk as
/// if it were the payload and only failed later, when something tried to parse it - as a
/// Geonames index, a JSON Schema, a CSV. Served from a local socket so this is deterministic
/// and offline.
#[test]
fn download_file_rejects_non_success_status() {
    use std::io::{Read, Write as _};

    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        if let Ok((mut sock, _)) = listener.accept() {
            // drain the request so the client isn't left waiting on us
            let _ = sock.read(&mut [0_u8; 1024]);
            let _ = sock.write_all(
                b"HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\nConnection: close\r\n\r\nNot \
                      Found",
            );
        }
    });

    let dest = std::env::temp_dir().join(format!("qsv_download_404_{}.bin", std::process::id()));
    let _ = std::fs::remove_file(&dest);

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(super::download_file(
        &format!("http://{addr}/missing.sz"),
        dest.clone(),
        false,
        None,
        Some(10),
        None,
    ));
    let _ = server.join();

    assert!(result.is_err(), "a 404 response must not be a success");
    assert!(
        !dest.exists(),
        "the error body must not be left behind as the payload"
    );
    let _ = std::fs::remove_file(&dest);
}

/// A FINAL 3xx must be refused too.
///
/// The guard is `!status.is_success()` rather than `error_for_status`, which only rejects
/// 4xx/5xx - so a 304, or a redirect the client stopped at, would otherwise be streamed to
/// disk as if it were the payload. Same defect, different status code.
#[test]
fn download_file_rejects_final_3xx() {
    use std::io::{Read, Write as _};

    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        if let Ok((mut sock, _)) = listener.accept() {
            let _ = sock.read(&mut [0_u8; 1024]);
            let _ = sock.write_all(
                b"HTTP/1.1 304 Not Modified\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            );
        }
    });

    let dest = std::env::temp_dir().join(format!("qsv_download_304_{}.bin", std::process::id()));
    let _ = std::fs::remove_file(&dest);

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(super::download_file(
        &format!("http://{addr}/unchanged.sz"),
        dest.clone(),
        false,
        None,
        Some(10),
        None,
    ));
    let _ = server.join();

    assert!(result.is_err(), "a 304 response must not be a success");
    assert!(!dest.exists(), "a 304 must not leave a file behind");
    let _ = std::fs::remove_file(&dest);
}

use std::path::Path;

use filetime::FileTime;
#[cfg(not(feature = "lite"))]
use tempfile::NamedTempFile;

use super::*;
use crate::cmd::stats::StatsData;

// A stats cache is located by input path and validated by mtime, NOT by parsing options, so
// these two fields are the only thing standing between a `--no-headers` (or differently
// delimited) cache and a headered consumer silently reading positional field names.
#[test]
fn stats_cache_parsing_opts_match_rejects_mismatched_no_headers() {
    let input = Path::new("data.csv");
    let meta = serde_json::json!({"flag_no_headers": true, "flag_delimiter": ","});
    // cache built headerless, consumer wants headers -> reject
    assert!(!stats_cache_parsing_opts_match(
        &meta, input, false, None, None, false
    ));
    // same setting on both sides -> accept
    assert!(stats_cache_parsing_opts_match(
        &meta, input, true, None, None, false
    ));
}

#[test]
fn stats_cache_parsing_opts_match_rejects_mismatched_delimiter() {
    let input = Path::new("data.csv");
    let meta = serde_json::json!({"flag_no_headers": false, "flag_delimiter": ";"});
    // cache built semicolon-delimited, consumer resolves ',' from the .csv extension
    assert!(!stats_cache_parsing_opts_match(
        &meta, input, false, None, None, false
    ));
    assert!(stats_cache_parsing_opts_match(
        &meta,
        input,
        false,
        Some(Delimiter(b';')),
        None,
        false
    ));
}

#[test]
fn stats_cache_parsing_opts_match_resolves_unset_delimiter_from_extension() {
    // an unset delimiter on BOTH sides resolves from the file extension, so a cache whose
    // recorded delimiter is empty still matches a consumer that passed none
    let meta = serde_json::json!({"flag_no_headers": false, "flag_delimiter": ""});
    assert!(stats_cache_parsing_opts_match(
        &meta,
        Path::new("data.csv"),
        false,
        None,
        None,
        false
    ));
    // ... and a .tsv input resolves to tab on both sides
    assert!(stats_cache_parsing_opts_match(
        &meta,
        Path::new("data.tsv"),
        false,
        None,
        None,
        false
    ));
    // but an explicit comma against a .tsv input is a genuine mismatch
    assert!(!stats_cache_parsing_opts_match(
        &meta,
        Path::new("data.tsv"),
        false,
        Some(Delimiter(b',')),
        None,
        false
    ));
}

#[test]
fn stats_cache_parsing_opts_conflict_only_on_a_readable_mismatch() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("data.csv");
    // No sidecar at all: NOT a conflict. `moarstats` writes the JSONL cache without one, and
    // rejecting those would discard a rich cache and regenerate a leaner one.
    assert!(!stats_cache_parsing_opts_conflict(
        &input, &input, false, None, None, false
    ));
    // Present but unparseable: same — decline to judge rather than discard.
    std::fs::write(dir.path().join("data.stats.csv.json"), b"{not json").unwrap();
    assert!(!stats_cache_parsing_opts_conflict(
        &input, &input, false, None, None, false
    ));
    // Readable and matching: no conflict.
    std::fs::write(
        dir.path().join("data.stats.csv.json"),
        br#"{"flag_no_headers": false, "flag_delimiter": ","}"#,
    )
    .unwrap();
    assert!(!stats_cache_parsing_opts_conflict(
        &input, &input, false, None, None, false
    ));
    // Readable and mismatched: the case this exists to catch.
    std::fs::write(
        dir.path().join("data.stats.csv.json"),
        br#"{"flag_no_headers": true, "flag_delimiter": ","}"#,
    )
    .unwrap();
    assert!(stats_cache_parsing_opts_conflict(
        &input, &input, false, None, None, false
    ));
}

#[test]
fn stats_cache_parsing_opts_conflict_sees_the_sidecar_beside_a_symlink() {
    // For a symlinked input the JSONL cache is looked up beside the canonical TARGET, but the
    // `stats` subprocess writes its metadata beside the SYMLINK. Checking only the canonical
    // location finds nothing and waves a mismatched cache through.
    let dir = tempfile::tempdir().unwrap();
    let link = dir.path().join("link.csv");
    let canonical = dir.path().join("target.csv");
    // metadata exists ONLY beside the "symlink" path, and it disagrees on --no-headers
    std::fs::write(
        dir.path().join("link.stats.csv.json"),
        br#"{"flag_no_headers": true, "flag_delimiter": ","}"#,
    )
    .unwrap();
    assert!(stats_cache_parsing_opts_conflict(
        &link, &canonical, false, None, None, false
    ));
    // and the agreeing case is still not a conflict
    std::fs::write(
        dir.path().join("link.stats.csv.json"),
        br#"{"flag_no_headers": false, "flag_delimiter": ","}"#,
    )
    .unwrap();
    assert!(!stats_cache_parsing_opts_conflict(
        &link, &canonical, false, None, None, false
    ));
}

#[test]
fn stats_cache_parsing_opts_match_rejects_mismatched_prefer_dmy() {
    let input = Path::new("data.csv");
    let meta = serde_json::json!({
        "flag_no_headers": false, "flag_delimiter": ",", "flag_prefer_dmy": true
    });
    // cache built dmy, consumer parses mdy -> reject: 01/02/2020 is a different instant
    assert!(!stats_cache_parsing_opts_match(
        &meta,
        input,
        false,
        None,
        Some(false),
        false
    ));
    // same setting on both sides -> accept
    assert!(stats_cache_parsing_opts_match(
        &meta,
        input,
        false,
        None,
        Some(true),
        false
    ));
    // a consumer that does no date inference (extsort, sortcheck) opts out entirely, so
    // the flag must not force it to regenerate
    assert!(stats_cache_parsing_opts_match(
        &meta, input, false, None, None, false
    ));
}

// --flexible is a parsing option too, but unlike the others it is checked ONE-WAY: a
// flexible cache may describe ragged records a strict consumer must refuse, while a strict
// cache can only exist for a file that had none, so it is always safe for a flexible one.
// The symmetric check would make flexible and strict consumers of the same unchanged file
// regenerate each other's cache on every invocation, forever.
#[test]
fn stats_cache_parsing_opts_match_refuses_a_flexible_cache_to_a_strict_consumer() {
    let input = Path::new("data.csv");
    let flexible_cache = serde_json::json!({
        "flag_no_headers": false, "flag_delimiter": ",", "flag_flexible": true
    });
    assert!(!stats_cache_parsing_opts_match(
        &flexible_cache,
        input,
        false,
        None,
        None,
        false
    ));
    // a flexible consumer may reuse it
    assert!(stats_cache_parsing_opts_match(
        &flexible_cache,
        input,
        false,
        None,
        None,
        true
    ));
}

#[test]
fn stats_cache_parsing_opts_match_accepts_a_strict_cache_for_a_flexible_run() {
    let input = Path::new("data.csv");
    let strict_cache = serde_json::json!({
        "flag_no_headers": false, "flag_delimiter": ",", "flag_flexible": false
    });
    // BOTH directions accept a strict cache - this pins the asymmetry. If someone
    // "restores" a symmetric `==` comparison, the flexible case below fails.
    assert!(stats_cache_parsing_opts_match(
        &strict_cache,
        input,
        false,
        None,
        None,
        true
    ));
    assert!(stats_cache_parsing_opts_match(
        &strict_cache,
        input,
        false,
        None,
        None,
        false
    ));
    // a legacy cache written before the field existed records nothing, and must be read
    // as strict rather than rejected
    let legacy = serde_json::json!({"flag_no_headers": false, "flag_delimiter": ","});
    assert!(stats_cache_parsing_opts_match(
        &legacy, input, false, None, None, false
    ));
}

#[test]
fn stats_jsonl_predates_stats_cache_only_when_the_sidecar_is_newer() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("data.csv");
    let jsonl = dir.path().join("data.stats.csv.data.jsonl");
    let sidecar = dir.path().join("data.stats.csv.json");

    // no JSONL at all: nothing to judge
    assert!(!stats_jsonl_predates_stats_cache(&jsonl, [&input, &input]));

    std::fs::write(&jsonl, b"{}\n").unwrap();
    // JSONL but NO sidecar: deliberately NOT stale. `moarstats` writes the JSONL from its
    // extended stats and writes no sidecar; discarding that would downgrade consumers.
    assert!(!stats_jsonl_predates_stats_cache(&jsonl, [&input, &input]));

    // sidecar OLDER than the JSONL: exactly the `stats` then `moarstats` ordering, where
    // the JSONL is the later and richer artifact. Still not stale.
    std::fs::write(&sidecar, b"{}").unwrap();
    filetime::set_file_mtime(&sidecar, FileTime::from_unix_time(1_000, 0)).unwrap();
    filetime::set_file_mtime(&jsonl, FileTime::from_unix_time(2_000, 0)).unwrap();
    assert!(!stats_jsonl_predates_stats_cache(&jsonl, [&input, &input]));

    // sidecar NEWER than the JSONL: a recompute refreshed the stats and left the JSONL
    // behind. This is the case that silently poisoned consumers.
    filetime::set_file_mtime(&sidecar, FileTime::from_unix_time(3_000, 0)).unwrap();
    assert!(stats_jsonl_predates_stats_cache(&jsonl, [&input, &input]));
}

#[test]
fn statsdata_moment_skewness_accepts_string_caches() {
    // moarstats 23.0.0 wrote `moment_skewness` into `.data.jsonl` as a JSON STRING (it was not
    // yet in STATSDATA_TYPES_MAP), and `get_stats_records` does not version-check the JSONL.
    // A strict f64 would fail the WHOLE record on those files — i.e. break plain `viz smart`
    // on any directory where moarstats ran before the upgrade.
    let de = |v: &str| {
        let line = format!(
            r#"{{"field":"x","type":"Float","nullcount":0,"cardinality":9,"moment_skewness":{v}}}"#
        );
        serde_json::from_str::<crate::cmd::stats::StatsData>(&line)
            .unwrap()
            .moment_skewness
    };
    assert_eq!(de(r#""1.87""#), Some(1.87));
    assert_eq!(de(r#"" -0.5 ""#), Some(-0.5));
    assert_eq!(de("1.87"), Some(1.87));
    assert_eq!(de("3"), Some(3.0));
    assert_eq!(de(r#""""#), None);
    assert_eq!(de(r#""abc""#), None);
    assert_eq!(de(r#""NaN""#), None);
    assert_eq!(de("null"), None);
    let absent: crate::cmd::stats::StatsData =
        serde_json::from_str(r#"{"field":"x","type":"Float","nullcount":0,"cardinality":9}"#)
            .unwrap();
    assert_eq!(absent.moment_skewness, None);
}

#[test]
fn statsdata_lag1_and_benford_accept_string_caches() {
    // same trap as `moment_skewness`: caches written before these joined STATSDATA_TYPES_MAP
    // carry them as JSON STRINGS, which a strict f64 would reject for the WHOLE record.
    let line = r#"{"field":"x","type":"Float","nullcount":0,"cardinality":9,"lag1_autocorrelation":"0.72","benford_mad":"0.0041"}"#;
    let s: crate::cmd::stats::StatsData = serde_json::from_str(line).unwrap();
    assert_eq!(s.lag1_autocorrelation, Some(0.72));
    assert_eq!(s.benford_mad, Some(0.0041));
    let line = r#"{"field":"x","type":"Float","nullcount":0,"cardinality":9,"lag1_autocorrelation":-0.3,"benford_mad":""}"#;
    let s: crate::cmd::stats::StatsData = serde_json::from_str(line).unwrap();
    assert_eq!(s.lag1_autocorrelation, Some(-0.3));
    assert_eq!(s.benford_mad, None);
}

#[test]
fn resolve_stats_columns_exact_selection_wins() {
    let headers = csv::StringRecord::from(vec!["a", "b", "a"]);
    // the baseline ran here with --select 3,3: exact, and names line up
    assert_eq!(
        resolve_stats_columns(&["a", "a"], &headers, Some(&[2, 2])),
        (vec![Some(2), Some(2)], vec![])
    );
    // a selection that doesn't line up with the rows is ignored (falls back below)
    assert_eq!(
        resolve_stats_columns(&["a", "b", "a"], &headers, Some(&[0, 1])).0,
        [Some(0), Some(1), Some(2)]
    );
}

#[test]
fn resolve_stats_columns_reused_cache_rules() {
    let headers = csv::StringRecord::from(vec!["a", "b", "a", "c", "a"]);
    // full coverage of the `a` twins: by occurrence; unknown names -> None
    assert_eq!(
        resolve_stats_columns(&["a", "b", "a", "c", "a", "zz"], &headers, None),
        (
            vec![Some(0), Some(1), Some(2), Some(3), Some(4), None],
            vec![]
        )
    );
    // a unique name maps to its column however many rows carry it (`--select 1,1`)
    assert_eq!(
        resolve_stats_columns(&["b", "b", "c"], &headers, None),
        (vec![Some(1), Some(1), Some(3)], vec![])
    );
    // a partial (or repeated) cover of the twins can't be told apart: None + reported
    assert_eq!(
        resolve_stats_columns(&["b", "a"], &headers, None),
        (vec![Some(1), None], vec!["a".to_string()])
    );
    assert_eq!(
        resolve_stats_columns(&["a", "a", "a", "a"], &headers, None).1,
        ["a"]
    );
}

#[test]
fn stats_options_select_finds_every_spelling() {
    assert_eq!(stats_options_select(""), None);
    assert_eq!(stats_options_select("--everything"), None);
    assert_eq!(stats_options_select("--select 2,3").as_deref(), Some("2,3"));
    assert_eq!(stats_options_select("--select=a,b").as_deref(), Some("a,b"));
    assert_eq!(stats_options_select("-E -s 1,1").as_deref(), Some("1,1"));
    assert_eq!(stats_options_select("-Es 4").as_deref(), Some("4"));
    assert_eq!(stats_options_select("-Es4-6").as_deref(), Some("4-6"));
    assert_eq!(
        stats_options_select("-s1 --everything").as_deref(),
        Some("1")
    );
    // `-c` takes an argument, so the `s` after it is part of that value, not --select
    assert_eq!(stats_options_select("-cs 5"), None);
    // the last one wins
    assert_eq!(
        stats_options_select("-s 1 --select 2").as_deref(),
        Some("2")
    );
}

#[test]
fn disambiguate_names_follows_safenames_and_never_collides() {
    let got = |names: &[&str]| disambiguate_names(names);
    // unique names pass through untouched
    assert_eq!(got(&["a", "b"]), ["a", "b"]);
    // later duplicates get the safenames suffix
    assert_eq!(got(&["a", "a", "b", "a"]), ["a", "a_2", "b", "a_3"]);
    // a suffix that is already a real name is skipped, whichever side of it the name sits
    assert_eq!(got(&["a", "a", "a_2"]), ["a", "a_3", "a_2"]);
    assert_eq!(got(&["a_2", "a", "a"]), ["a_2", "a", "a_3"]);
    // every label is unique
    let labels = got(&["x", "x", "x_2", "x", "x_3"]);
    let set: std::collections::HashSet<&String> = labels.iter().collect();
    assert_eq!(set.len(), labels.len(), "{labels:?}");
}

#[test]
fn statsdata_carries_every_column_stats_emits() {
    // StatsData has no deny_unknown_fields, so a column present in the cache but missing
    // from the struct is SILENTLY DROPPED on deserialize rather than erroring. That is how
    // sortiness, geometric_mean, harmonic_mean and percentiles stayed unreachable to every
    // consumer (schema, frequency, tojsonl, viz smart, describegpt) despite `stats` writing
    // them into every .stats.csv.data.jsonl. Shapes and values below are taken from a real
    // `stats --everything --stats-jsonl` line.
    let line = r#"{"field":"id","type":"Integer","nullcount":0,"cardinality":5,
            "sortiness":1.0,"geometric_mean":2.6052,"harmonic_mean":2.1898,
            "median":"2",
            "percentiles":"5: 1|10: 1|40: 2|60: 3|90: 5|95: 5"}"#;

    let s: crate::cmd::stats::StatsData = serde_json::from_str(line).unwrap();

    assert_eq!(s.sortiness, Some(1.0));
    assert_eq!(s.geometric_mean, Some(2.6052));
    assert_eq!(s.harmonic_mean, Some(2.1898));
    assert_eq!(s.median.as_deref(), Some("2"));
    assert_eq!(
        s.percentiles.as_deref(),
        Some("5: 1|10: 1|40: 2|60: 3|90: 5|95: 5")
    );

    // `median` is a type-dependent RENDERING (like min/max), so a Date column carries an
    // RFC3339 string in the same key. It must survive deserialization verbatim - typing it
    // as a float coerced this to 0.0 and silently corrupted date medians.
    let date_line = r#"{"field":"open_dt","type":"Date","nullcount":0,"cardinality":3,
            "median":"2020-01-15"}"#;
    let s: crate::cmd::stats::StatsData = serde_json::from_str(date_line).unwrap();
    assert_eq!(s.median.as_deref(), Some("2020-01-15"));

    // and a cache written before these columns existed must still deserialize
    let legacy = r#"{"field":"id","type":"Integer","nullcount":0,"cardinality":5}"#;
    let s: crate::cmd::stats::StatsData = serde_json::from_str(legacy).unwrap();
    assert_eq!(s.sortiness, None);
    assert_eq!(s.percentiles, None);
    assert_eq!(s.median, None);
}

#[test]
fn statsdata_date_renderings_survive_and_numeric_caches_still_load() {
    // `mean`, the three quartiles and the four fences are type-dependent RENDERINGS, exactly
    // like min/max/median/percentiles: a numeric column carries a number, a Date/DateTime
    // column carries RFC3339. Typing them Float made csv_to_jsonl coerce every date one to
    // 0.0 - the stats CSV held "2020-06-15" while the cache every consumer reads said 0.0.
    //
    // This assertion has to live IN-CRATE: tests/ is a separate binary and cannot name
    // StatsData, so a test over the jsonl file's bytes proves nothing about what consumers
    // actually deserialize.
    let date_line = r#"{"field":"open_dt","type":"Date","nullcount":0,"cardinality":5,
            "mean":"2010-12-06","q1":"1995-01-08","q2_median":"2020-06-15","q3":"2022-01-30",
            "lower_outer_fence":"1913-11-02","lower_inner_fence":"1954-06-06",
            "upper_inner_fence":"2062-09-03","upper_outer_fence":"2103-04-08",
            "iqr":9884.0,"mad":259.0}"#;
    let s: crate::cmd::stats::StatsData = serde_json::from_str(date_line).unwrap();

    assert_eq!(s.mean.as_deref(), Some("2010-12-06"));
    assert_eq!(s.q1.as_deref(), Some("1995-01-08"));
    assert_eq!(s.q2_median.as_deref(), Some("2020-06-15"));
    assert_eq!(s.q3.as_deref(), Some("2022-01-30"));
    assert_eq!(s.lower_outer_fence.as_deref(), Some("1913-11-02"));
    assert_eq!(s.lower_inner_fence.as_deref(), Some("1954-06-06"));
    assert_eq!(s.upper_inner_fence.as_deref(), Some("2062-09-03"));
    assert_eq!(s.upper_outer_fence.as_deref(), Some("2103-04-08"));

    // a date rendering has no numeric reading - `None`, never a fabricated 0.0
    assert_eq!(s.mean_f64(), None);
    assert_eq!(s.q2_median_f64(), None);
    assert_eq!(s.upper_outer_fence_f64(), None);

    // `iqr`/`mad` are a COUNT OF DAYS for a date column, not a date. They stay f64; retyping
    // them would corrupt them in the opposite direction.
    assert_eq!(s.iqr, Some(9884.0));
    assert_eq!(s.mad, Some(259.0));

    // A NUMERIC column carries these as JSON NUMBERS - and so does every cache written
    // before this fix, which stays on disk and still passes mtime validation after an
    // upgrade. Rejecting a number here would silently discard those caches on first read.
    let numeric_line = r#"{"field":"amt","type":"Float","nullcount":0,"cardinality":9,
            "mean":42.5,"q1":10,"q2_median":40.0,"q3":75.25,
            "lower_outer_fence":-185.0,"lower_inner_fence":-87.5,
            "upper_inner_fence":172.5,"upper_outer_fence":270.0}"#;
    let s: crate::cmd::stats::StatsData = serde_json::from_str(numeric_line).unwrap();

    assert_eq!(s.mean_f64(), Some(42.5));
    assert_eq!(s.q1_f64(), Some(10.0));
    assert_eq!(s.q2_median_f64(), Some(40.0));
    assert_eq!(s.q3_f64(), Some(75.25));
    assert_eq!(s.lower_outer_fence_f64(), Some(-185.0));
    assert_eq!(s.lower_inner_fence_f64(), Some(-87.5));
    assert_eq!(s.upper_inner_fence_f64(), Some(172.5));
    assert_eq!(s.upper_outer_fence_f64(), Some(270.0));

    // the availability probes these back (has_quartiles in stats_satisfy_mode, the box-panel
    // gate in viz) must keep seeing a PRESENT value for a date column. Representing a date
    // quartile as None would make `qsv schema` judge the cache insufficient and regenerate
    // it on every run.
    let s: crate::cmd::stats::StatsData = serde_json::from_str(date_line).unwrap();
    assert!(s.q2_median.is_some(), "a date quartile still EXISTS");
    assert!(s.mean.is_some());

    // absent keys stay absent
    let bare = r#"{"field":"id","type":"Integer","nullcount":0,"cardinality":5}"#;
    let s: crate::cmd::stats::StatsData = serde_json::from_str(bare).unwrap();
    assert_eq!(s.mean, None);
    assert_eq!(s.q2_median, None);
    assert_eq!(s.mean_f64(), None);
}

#[test]
fn statsdata_renderings_serialize_back_as_numbers() {
    // StatsData is NOT deserialize-only: `profile`'s build_dpps serializes it straight back
    // out with serde_json::to_value, and those values reach the published DCAT-US v3 /
    // Croissant / csvw projections. Storing the renderings as Option<String> made the DERIVED
    // Serialize emit a numeric column's mean as "42.5" instead of 42.5 - a silent change to
    // profile output that the whole test suite went green through (roborev 4326).
    let line = r#"{"field":"amt","type":"Float","nullcount":0,"cardinality":9,
            "mean":42.5,"q1":10,"q2_median":40.0,"q3":75.25,
            "lower_outer_fence":-185.0,"lower_inner_fence":-87.5,
            "upper_inner_fence":172.5,"upper_outer_fence":270.0}"#;
    let s: crate::cmd::stats::StatsData = serde_json::from_str(line).unwrap();
    let v = serde_json::to_value(&s).unwrap();

    for key in [
        "mean",
        "q1",
        "q2_median",
        "q3",
        "lower_outer_fence",
        "lower_inner_fence",
        "upper_inner_fence",
        "upper_outer_fence",
    ] {
        assert!(
            v[key].is_number(),
            "a numeric column's `{key}` must serialize back as a JSON NUMBER (profile's dpps \
             feeds it to the published projections), got {}",
            v[key]
        );
    }
    assert_eq!(v["mean"], serde_json::json!(42.5));
    assert_eq!(v["q1"], serde_json::json!(10.0));

    // a Date column's rendering has no numeric reading, so it goes out as a string
    let date_line = r#"{"field":"open_dt","type":"Date","nullcount":0,"cardinality":5,
            "mean":"2010-12-06","q2_median":"2020-06-15"}"#;
    let s: crate::cmd::stats::StatsData = serde_json::from_str(date_line).unwrap();
    let v = serde_json::to_value(&s).unwrap();
    assert_eq!(v["mean"], serde_json::json!("2010-12-06"));
    assert_eq!(v["q2_median"], serde_json::json!("2020-06-15"));
}

#[test]
fn statsdata_stale_precoercion_date_cache_yields_no_fabricated_zero() {
    // A `.data.jsonl` written BEFORE the rendering fix stores the coerced 0.0 for a
    // Date/DateTime column. Those caches stay on disk and still pass mtime validation after
    // an upgrade, so the 0.0 would deserialize as "0" and the accessors would hand consumers
    // the very fabricated zero this fix removes (roborev 4327). The row's `type` is the
    // tell-tale: a date column has no numeric quartile, whatever the cache claims.
    let stale = r#"{"field":"open_dt","type":"Date","nullcount":0,"cardinality":5,
            "mean":0.0,"q1":0.0,"q2_median":0.0,"q3":0.0,
            "lower_outer_fence":0.0,"lower_inner_fence":0.0,
            "upper_inner_fence":0.0,"upper_outer_fence":0.0,"iqr":9884.0}"#;
    let s: crate::cmd::stats::StatsData = serde_json::from_str(stale).unwrap();

    assert_eq!(s.mean_f64(), None, "stale date cache must not yield 0.0");
    assert_eq!(s.q1_f64(), None);
    assert_eq!(s.q2_median_f64(), None);
    assert_eq!(s.q3_f64(), None);
    assert_eq!(s.lower_outer_fence_f64(), None);
    assert_eq!(s.lower_inner_fence_f64(), None);
    assert_eq!(s.upper_inner_fence_f64(), None);
    assert_eq!(s.upper_outer_fence_f64(), None);

    // the availability probes still see a present value, so this does NOT push consumers
    // into regenerating the cache on every run
    assert!(s.q2_median.is_some());
    // and `iqr`, a day-count, is untouched by the type gate
    assert_eq!(s.iqr, Some(9884.0));

    // the gate is keyed on the column TYPE, so numeric columns are unaffected
    let numeric = r#"{"field":"amt","type":"Float","nullcount":0,"cardinality":9,
            "mean":0.0,"q1":0.0}"#;
    let s: crate::cmd::stats::StatsData = serde_json::from_str(numeric).unwrap();
    assert_eq!(s.mean_f64(), Some(0.0), "a numeric 0.0 is a real value");
    assert_eq!(s.q1_f64(), Some(0.0));
}

#[test]
fn normalize_stale_date_renderings_keeps_accessors_and_serialization_agreeing() {
    // The accessors refuse a stale date zero, but `profile`'s build_dpps serializes StatsData
    // straight back out, and ser_stat_rendering sees only the field - not the row's type - so
    // "0" went back out as JSON 0 into published projections (roborev 4328). Normalizing once
    // at load is what makes the two paths agree. NOT a manual Serialize impl: a third
    // hand-maintained field list beside the struct and STATSDATA_TYPES_MAP is exactly the
    // drift that silently dropped sortiness/geometric_mean/harmonic_mean/percentiles/median.
    let stale = r#"{"field":"open_dt","type":"Date","nullcount":0,"cardinality":5,
            "mean":0.0,"q1":0.0,"q2_median":0.0,"q3":0.0,
            "lower_outer_fence":0.0,"lower_inner_fence":0.0,
            "upper_inner_fence":0.0,"upper_outer_fence":0.0,"iqr":9884.0}"#;
    let mut s: crate::cmd::stats::StatsData = serde_json::from_str(stale).unwrap();
    s.normalize_stale_date_renderings();

    let v = serde_json::to_value(&s).unwrap();
    for key in [
        "mean",
        "q1",
        "q2_median",
        "q3",
        "lower_outer_fence",
        "lower_inner_fence",
        "upper_inner_fence",
        "upper_outer_fence",
    ] {
        assert!(
            v[key].is_null(),
            "a stale date {key} must not re-serialize as a number, got {}",
            v[key]
        );
    }
    // day-counts are not renderings and survive
    assert_eq!(v["iqr"], serde_json::json!(9884.0));

    // a POST-fix date cache holds real RFC3339 renderings - untouched, so this is a no-op
    let fresh = r#"{"field":"open_dt","type":"Date","nullcount":0,"cardinality":5,
            "mean":"2010-12-06","q2_median":"2020-06-15"}"#;
    let mut s: crate::cmd::stats::StatsData = serde_json::from_str(fresh).unwrap();
    s.normalize_stale_date_renderings();
    assert_eq!(s.mean.as_deref(), Some("2010-12-06"));
    assert_eq!(s.q2_median.as_deref(), Some("2020-06-15"));

    // and a NUMERIC column is never touched, whatever the values
    let numeric = r#"{"field":"amt","type":"Float","nullcount":0,"cardinality":9,
            "mean":0.0,"q2_median":42.5}"#;
    let mut s: crate::cmd::stats::StatsData = serde_json::from_str(numeric).unwrap();
    s.normalize_stale_date_renderings();
    assert_eq!(s.mean_f64(), Some(0.0));
    assert_eq!(s.q2_median_f64(), Some(42.5));
}

#[cfg(test)]
fn write_test_zip(path: &Path, entries: &[(&str, &[u8])]) {
    use std::io::Write as _;
    let f = std::fs::File::create(path).unwrap();
    let mut zw = zip::ZipWriter::new(f);
    let opts: zip::write::SimpleFileOptions = zip::write::SimpleFileOptions::default();
    for (name, data) in entries {
        zw.start_file(*name, opts).unwrap();
        zw.write_all(data).unwrap();
    }
    zw.finish().unwrap();
}

fn sd(r#type: &str, sort_order: Option<&str>, nullcount: u64, is_ascii: bool) -> StatsData {
    StatsData {
        r#type: r#type.to_string(),
        sort_order: sort_order.map(ToString::to_string),
        nullcount,
        is_ascii,
        ..Default::default()
    }
}

#[cfg(feature = "self_update")]
#[test]
fn update_check_guard_throttles() {
    let dir = tempfile::tempdir().unwrap();
    let stamp = dir.path().join(".last_update_check");
    let guard =
        self_update::check_interval::UpdateCheckGuard::new(&stamp, Duration::from_secs(3600));
    // no stamp yet => a check is due
    assert!(guard.should_check().unwrap());
    // after recording, the interval throttles the next check
    guard.record_check().unwrap();
    assert!(!guard.should_check().unwrap());
}

#[test]
fn proves_ascending_string_lex() {
    let stats = vec![sd("String", Some("Ascending"), 0, true)];
    // default lexicographic, ascending -> hit
    assert!(stats_cache_proves_ascending(&[0], &stats, false, false));
    // extsort variant (ASCII) -> hit
    assert!(stats_cache_proves_ascending(&[0], &stats, false, true));
    // numeric requested on a String column -> miss
    assert!(!stats_cache_proves_ascending(&[0], &stats, true, false));
}

#[test]
fn proves_ascending_descending_never() {
    // a Descending cache is never an ascending proof (--reverse is not
    // short-circuited regardless of direction)
    let stats = vec![sd("String", Some("Descending"), 0, true)];
    assert!(!stats_cache_proves_ascending(&[0], &stats, false, false));
    assert!(!stats_cache_proves_ascending(&[0], &stats, false, true));
}

#[test]
fn proves_ascending_numeric() {
    for typ in ["Integer", "Float"] {
        let stats = vec![sd(typ, Some("Ascending"), 0, true)];
        assert!(stats_cache_proves_ascending(&[0], &stats, true, false));
        // lexicographic requested on a numeric column -> miss
        assert!(!stats_cache_proves_ascending(&[0], &stats, false, false));
    }
}

#[test]
fn proves_ascending_extsort_non_ascii() {
    // ascending String but not ASCII: safe for plain lex, NOT for extsort
    let stats = vec![sd("String", Some("Ascending"), 0, false)];
    assert!(stats_cache_proves_ascending(&[0], &stats, false, false));
    assert!(!stats_cache_proves_ascending(&[0], &stats, false, true));
}

#[test]
fn proves_ascending_guards() {
    let asc = sd("String", Some("Ascending"), 0, true);
    // multi-column selection -> never
    let stats = vec![asc.clone(), asc.clone()];
    assert!(!stats_cache_proves_ascending(&[0, 1], &stats, false, false));
    // out-of-bounds index -> never
    assert!(!stats_cache_proves_ascending(&[5], &stats, false, false));
    // nullcount != 0 -> never
    let nulls = vec![sd("String", Some("Ascending"), 1, true)];
    assert!(!stats_cache_proves_ascending(&[0], &nulls, false, false));
    // Unsorted -> never
    let unsorted = vec![sd("String", Some("Unsorted"), 0, true)];
    assert!(!stats_cache_proves_ascending(&[0], &unsorted, false, false));
    // missing sort_order -> never
    let none = vec![sd("String", None, 0, true)];
    assert!(!stats_cache_proves_ascending(&[0], &none, false, false));
    // Date typed -> never (no matching comparator)
    let date = vec![sd("Date", Some("Ascending"), 0, true)];
    assert!(!stats_cache_proves_ascending(&[0], &date, false, false));
    assert!(!stats_cache_proves_ascending(&[0], &date, true, false));
}

#[test]
fn extract_all_zip_entries_orders_tabular_first_and_filters() {
    let src = tempfile::tempdir().unwrap();
    let zip_path = src.path().join("data.zip");
    // Archive order deliberately puts a system file, an unsupported file, and
    // a non-tabular special-format file BEFORE the tabular entries.
    write_test_zip(
        &zip_path,
        &[
            ("__MACOSX/junk", b"x"),    // system file -> skipped
            ("notes.txt", b"hello"),    // unsupported -> skipped
            ("b.json", b"[{\"a\":1}]"), // special-format supported -> "other"
            ("a.tsv", b"a\tb\n1\t2\n"), // tabular
            ("c.csv", b"x,y\n3,4\n"),   // tabular
        ],
    );

    let work = tempfile::tempdir().unwrap();
    let (_, got) = extract_all_zip_entries(&zip_path, &work).unwrap();
    let names: Vec<String> = got
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    // Tabular entries first (in archive order), then other supported entries;
    // the system file and the unsupported .txt are dropped.
    assert_eq!(names, vec!["a.tsv", "c.csv", "b.json"]);
    // And the first element matches what the reader-level select_zip_entry
    // would pick (the first CSV/TSV/TAB/SSV entry).
    assert!(got[0].exists());
}

#[test]
fn extract_all_zip_entries_errors_when_no_supported_entry() {
    let src = tempfile::tempdir().unwrap();
    let zip_path = src.path().join("nope.zip");
    write_test_zip(&zip_path, &[("readme.txt", b"hi"), ("notes.md", b"yo")]);
    let work = tempfile::tempdir().unwrap();
    let err = extract_all_zip_entries(&zip_path, &work).unwrap_err();
    assert!(
        err.to_string().contains("no supported"),
        "unexpected error: {err}"
    );
}

#[cfg(unix)]
#[test]
fn test_sync_directory_smoke() {
    // sync_directory is best-effort by design: success paths must
    // return normally, and error paths (missing dir, FS that doesn't
    // support directory fsync) must NOT panic or propagate. This
    // smoke test covers both branches.
    let dir = tempfile::TempDir::new().unwrap();

    // Existing directory: should succeed (or silently no-op if the
    // backing FS doesn't support it).
    sync_directory(dir.path());

    // Non-existent path: should still return without panicking;
    // the open() error is logged and swallowed.
    sync_directory(&dir.path().join("definitely_does_not_exist"));
}

#[cfg(not(feature = "lite"))]
#[test]
fn test_hash_blake3_file() {
    // Create a temporary file with known content
    let mut temp_file = NamedTempFile::new().unwrap();
    let test_content = b"Hello, World! This is a test file for BLAKE3 hashing.";
    temp_file.write_all(test_content).unwrap();
    temp_file.flush().unwrap();

    // Calculate expected hash using blake3 directly
    let mut expected_hasher = blake3::Hasher::new();
    expected_hasher.update(test_content);
    let expected_hash = expected_hasher.finalize().to_hex().to_string();

    // Test our function
    let actual_hash = hash_blake3_file(temp_file.path()).unwrap();

    assert_eq!(actual_hash, expected_hash);
}

#[cfg(not(feature = "lite"))]
#[test]
fn test_hash_blake3_file_large() {
    // Create a larger test file (1MB)
    let mut temp_file = NamedTempFile::new().unwrap();
    let test_content = b"Large file test content. ".repeat(40000); // ~1MB
    temp_file.write_all(&test_content).unwrap();
    temp_file.flush().unwrap();

    // Calculate expected hash
    let mut expected_hasher = blake3::Hasher::new();
    expected_hasher.update(&test_content);
    let expected_hash = expected_hasher.finalize().to_hex().to_string();

    // Test our function
    let actual_hash = hash_blake3_file(temp_file.path()).unwrap();

    assert_eq!(actual_hash, expected_hash);
}

#[cfg(not(feature = "lite"))]
#[test]
fn benchmark_hash_blake3_file() {
    // Create a test file for benchmarking
    let mut temp_file = NamedTempFile::new().unwrap();
    let test_content = b"Benchmark test content. ".repeat(100000); // ~2.4MB
    temp_file.write_all(&test_content).unwrap();
    temp_file.flush().unwrap();

    // Benchmark the function
    let start = std::time::Instant::now();
    let hash = hash_blake3_file(temp_file.path()).unwrap();
    let duration = start.elapsed();

    println!("BLAKE3 Hash: {}", hash);
    println!("BLAKE3 Time: {:?}", duration);
    println!("File size: {} bytes", test_content.len());
    println!(
        "BLAKE3 Speed: {:.2} MB/s",
        (test_content.len() as f64 / 1024.0 / 1024.0) / duration.as_secs_f64()
    );
    assert_eq!(
        hash,
        "87b87c9d36ee75bf8cb30940f6bbbeec3e67328190181f8c59c3cbcd6f35228a"
    );
}

#[cfg(not(feature = "lite"))]
#[test]
fn benchmark_hash_blake3_file_large() {
    // Create a larger test file (100MB) to test parallel processing
    let mut temp_file = NamedTempFile::new().unwrap();
    let test_content = b"Large benchmark test content for parallel processing. ".repeat(2000000); // ~100MB
    temp_file.write_all(&test_content).unwrap();
    temp_file.flush().unwrap();

    // Benchmark the function
    let start = std::time::Instant::now();
    let hash = hash_blake3_file(temp_file.path()).unwrap();
    let duration = start.elapsed();

    println!("BLAKE3 Large file hash: {}", hash);
    println!("BLAKE3 Large file time: {:?}", duration);
    println!("Large file size: {} bytes", test_content.len());
    println!(
        "BLAKE3 Large file speed: {:.2} MB/s",
        (test_content.len() as f64 / 1024.0 / 1024.0) / duration.as_secs_f64()
    );
    assert_eq!(
        hash,
        "6cd27b8098295afc42527bcee267ce39e757345f9f82c20b66efc75ffe4c1631"
    );
}

#[test]
fn test_transform_github_url_blob() {
    // Test GitHub blob URL transformation
    let blob_url = "https://github.com/dathere/qsv/blob/master/resources/test/boston311-100.csv";
    let expected =
        "https://raw.githubusercontent.com/dathere/qsv/master/resources/test/boston311-100.csv";
    assert_eq!(transform_github_url(blob_url), expected);
}

#[test]
fn test_transform_github_url_blob_with_branch() {
    // Test with a different branch name
    let blob_url = "https://github.com/user/repo/blob/develop/path/to/file.csv";
    let expected = "https://raw.githubusercontent.com/user/repo/develop/path/to/file.csv";
    assert_eq!(transform_github_url(blob_url), expected);
}

#[test]
fn test_transform_github_url_blob_http() {
    // Test HTTP (not HTTPS) URL transformation
    let blob_url = "http://github.com/user/repo/blob/main/file.csv";
    let expected = "https://raw.githubusercontent.com/user/repo/main/file.csv";
    assert_eq!(transform_github_url(blob_url), expected);
}

#[test]
fn test_transform_github_url_raw() {
    // Test that raw URLs are not modified
    let raw_url =
        "https://raw.githubusercontent.com/dathere/qsv/master/resources/test/boston311-100.csv";
    assert_eq!(transform_github_url(raw_url), raw_url);
}

#[test]
fn test_transform_github_url_non_github() {
    // Test that non-GitHub URLs are not modified
    let other_url = "https://example.com/data/file.csv";
    assert_eq!(transform_github_url(other_url), other_url);
}

#[test]
fn test_transform_github_url_no_blob() {
    // Test that GitHub URLs without /blob/ are not modified
    let repo_url = "https://github.com/dathere/qsv";
    assert_eq!(transform_github_url(repo_url), repo_url);
}

#[test]
fn test_transform_github_url_with_query_params() {
    // Test that query parameters are stripped (they don't apply to raw.githubusercontent.com)
    let blob_url = "https://github.com/user/repo/blob/main/file.csv?ref=v1.0";
    let expected = "https://raw.githubusercontent.com/user/repo/main/file.csv";
    assert_eq!(transform_github_url(blob_url), expected);
}

#[test]
fn test_transform_github_url_with_fragment() {
    // Test that fragments (line highlighting) are stripped
    let blob_url = "https://github.com/user/repo/blob/main/file.csv#L10-L20";
    let expected = "https://raw.githubusercontent.com/user/repo/main/file.csv";
    assert_eq!(transform_github_url(blob_url), expected);
}

#[test]
fn test_transform_github_url_with_query_and_fragment() {
    // Test that both query parameters and fragments are stripped
    let blob_url = "https://github.com/user/repo/blob/main/file.csv?ref=v1.0#L10";
    let expected = "https://raw.githubusercontent.com/user/repo/main/file.csv";
    assert_eq!(transform_github_url(blob_url), expected);
}

/// The system temp root must NEVER be removable. `Config::new` once fell back to
/// `env::temp_dir()` when `TempDir::new()` failed, and `log_end` removes `TEMP_FILE_DIR`
/// wholesale - so without this guard a rare temp-dir creation failure would have wiped
/// every other process's temp files. roborev 4366.
#[test]
fn temp_dir_is_removable_refuses_the_system_temp_root() {
    let system_temp = std::env::temp_dir();
    assert!(
        !temp_dir_is_removable(&system_temp),
        "the system temp root must never be removable"
    );

    // A SYMLINK to the system temp root must not be removable either. This is the case
    // that actually exercises the canonicalize branch: `Path` equality normalizes away
    // `.` and trailing separators, so those variants are already caught by the check
    // above and would leave that branch untested.
    #[cfg(unix)]
    {
        let holder = tempfile::TempDir::new().unwrap();
        let link = holder.path().join("link-to-system-temp");
        std::os::unix::fs::symlink(&system_temp, &link).unwrap();
        assert_ne!(
            link, system_temp,
            "the symlink path must differ textually, or this asserts nothing"
        );
        assert!(
            !temp_dir_is_removable(&link),
            "a symlink resolving to the system temp root must not be removable"
        );
    }

    // a filesystem root is never removable
    assert!(!temp_dir_is_removable(std::path::Path::new("/")));

    // but a qsv-created subdirectory of it IS - this is the normal case, and a guard
    // that refused everything would silently reintroduce the leak this all started from
    let owned = tempfile::TempDir::new().unwrap();
    assert!(
        temp_dir_is_removable(owned.path()),
        "a qsv-created temp directory must remain removable"
    );
}
