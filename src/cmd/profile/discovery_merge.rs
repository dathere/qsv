//! Profile-aware merge of publisher-discovered DCAT metadata into the
//! qsv-inferred projection.
//!
//! Replaces the inline `merge_discovered` in `profile.rs` with a
//! configurable engine that consults `profile.discovery_merge`:
//!
//! * `enabled: false` → merge is a no-op for this profile (Croissant).
//! * `never_overwrite: [...]` → those top-level keys are never touched regardless of strategy
//!   (typical: `@context`, `@type`, `dcat:distribution`).
//! * `default_strategy: fill-if-absent | overlay-array | never` → applied to keys not on the
//!   never_overwrite list AND not on the `forced_paths` list (which always wins regardless of
//!   strategy).
//!
//! The `forced_paths` argument is the set of leaf paths the user
//! overrode via `--initial-context`'s `dataset_info` block (or
//! `{value, force: true}` wrappers). Forced paths are protected from
//! discovered overlay so the user's wishes are honored.

use serde_json::{Map, Value};

use super::profile_spec::ProfileSpec;

pub fn merge(
    profile: &ProfileSpec,
    inferred: Value,
    discovered: Option<&Value>,
    forced_dcat_paths: &[String],
) -> Value {
    let Some(discovered) = discovered else {
        return inferred;
    };
    if !profile.discovery_merge.enabled {
        return inferred;
    }
    let Value::Object(mut inferred_obj) = inferred else {
        return inferred;
    };
    let Some(discovered_obj) = discovered.as_object() else {
        return Value::Object(inferred_obj);
    };

    let never = &profile.discovery_merge.never_overwrite;
    let strategy = profile
        .discovery_merge
        .default_strategy
        .as_deref()
        .unwrap_or("fill-if-absent");

    // Resolve the per-distribution merge config (if any). When
    // enabled, the distribution array bypasses the `never_overwrite`
    // early-return below and gets identity-based element merging.
    // `array_key` defaults to `dcat:distribution` to match the
    // documented contract (Roborev #2499 finding #2): enabling
    // `distribution_merge` without spelling out `array_key` must
    // not silently disable per-distribution merging.
    let dist_cfg = profile
        .discovery_merge
        .distribution_merge
        .as_ref()
        .filter(|d| d.enabled);
    let dist_key = dist_cfg.map(|d| d.array_key.as_deref().unwrap_or("dcat:distribution"));

    // Handle the distribution array first so `never_overwrite` doesn't
    // block it when per-distribution merging is enabled.
    if let (Some(cfg), Some(dkey)) = (dist_cfg, dist_key)
        && let Some(disc_arr) = discovered_obj.get(dkey)
    {
        let inferred_arr = inferred_obj
            .remove(dkey)
            .unwrap_or_else(|| Value::Array(Vec::new()));
        let merged = merge_distribution_array(inferred_arr, disc_arr, cfg, forced_dcat_paths, dkey);
        inferred_obj.insert(dkey.to_string(), merged);
    }

    for (key, value) in discovered_obj {
        // Already merged above by per-distribution path — skip.
        if dist_key == Some(key.as_str()) {
            continue;
        }
        if never.iter().any(|k| k == key) {
            continue;
        }
        if strategy != "overlay-array" && inferred_obj.contains_key(key) {
            // fill-if-absent: inferred always wins.
            continue;
        }
        // Skip when the user marked this top-level DCAT key as forced.
        // Discovered key `k` maps to dataset_info pointer
        // `/dcat/<escaped-k>` per the legacy semantics.
        let candidate = format!("/dcat/{}", escape_token(key));
        let is_forced = forced_dcat_paths
            .iter()
            .any(|p| p == &candidate || p.starts_with(&format!("{candidate}/")));
        if is_forced {
            continue;
        }
        merge_one(&mut inferred_obj, key, value.clone(), strategy);
    }
    Value::Object(inferred_obj)
}

/// RFC 6901 §4 token escape. Internal copy so this module doesn't
/// depend on profile.rs.
fn escape_token(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
}

fn merge_one(out: &mut Map<String, Value>, key: &str, value: Value, strategy: &str) {
    match strategy {
        "never" => { /* no-op */ },
        "overlay-array" => {
            // Append discovered array elements to the inferred array.
            // Single-value discovered becomes a one-element array.
            let merged = match (out.remove(key), value) {
                (Some(Value::Array(mut a)), Value::Array(b)) => {
                    a.extend(b);
                    Value::Array(a)
                },
                (Some(Value::Array(mut a)), other) => {
                    a.push(other);
                    Value::Array(a)
                },
                (Some(other), Value::Array(b)) => {
                    let mut a = vec![other];
                    a.extend(b);
                    Value::Array(a)
                },
                (Some(other_a), other_b) => Value::Array(vec![other_a, other_b]),
                (None, v) => v,
            };
            out.insert(key.to_string(), merged);
        },
        _ /* fill-if-absent */ => {
            if !out.contains_key(key) {
                out.insert(key.to_string(), value);
            }
        },
    }
}

/// Per-element merge of the distribution array. For each publisher
/// Distribution, locate a matching inferred Distribution via
/// `identity_keys` (first non-empty match wins), then merge each
/// publisher field into the matched inferred object via the
/// configured `field_strategy`. Publisher distributions that don't
/// match any inferred entry are appended only when
/// `append_unmatched: true`.
///
/// `forced_dcat_paths` honors the same `dataset_info` force-override
/// semantics that the outer `merge` uses for top-level keys: any
/// publisher field whose pointer matches a forced path exactly, or
/// is the parent of one (forced sub-key under a publisher field), is
/// skipped (Roborev #2499 finding #1). Forced paths use the index of
/// the *inferred* distribution as the array index.
///
/// The inferred and discovered values are coerced into arrays
/// uniformly: a single object becomes a one-element slice. The
/// returned value is always a `Value::Array`. Iteration is by
/// reference; per-field clones happen lazily inside the match arms
/// so large publisher catalogs don't pay an upfront `to_vec`
/// allocation and unmatched-drop entries are never cloned.
fn merge_distribution_array(
    inferred: Value,
    discovered: &Value,
    cfg: &super::profile_spec::DistributionMerge,
    forced_dcat_paths: &[String],
    dist_key: &str,
) -> Value {
    let mut inferred_arr = coerce_to_array(inferred);
    // Borrow the publisher array as a slice — `from_ref` lets the
    // scalar case share the same iteration path without allocating
    // a temporary Vec.
    let disc_slice: &[Value] = match discovered {
        Value::Array(a) => a.as_slice(),
        other => std::slice::from_ref(other),
    };

    let field_strategy = cfg.field_strategy.as_deref().unwrap_or("fill-if-absent");
    let dist_prefix = format!("/dcat/{}", escape_token(dist_key));

    for disc in disc_slice {
        let Value::Object(disc_obj) = disc else {
            // Non-object publisher entry: append only if configured
            // (clone only at the moment of insertion).
            if cfg.append_unmatched {
                inferred_arr.push(disc.clone());
            }
            continue;
        };
        match find_distribution_match(&inferred_arr, disc_obj, &cfg.identity_keys) {
            Some(idx) => {
                if let Value::Object(inf_obj) = &mut inferred_arr[idx] {
                    for (k, v) in disc_obj {
                        // Honor dataset_info force overrides: skip
                        // publisher fields whose JSON pointer
                        // (`/dcat/<dist_key>/<idx>/<field>`) is
                        // forced exactly or has a forced sub-key
                        // beneath it. Mirrors the top-level
                        // forced-path semantics in `merge` so users
                        // cannot have their explicit overrides
                        // silently overwritten by per-distribution
                        // merging.
                        let field_path = format!("{dist_prefix}/{idx}/{}", escape_token(k));
                        let is_forced = forced_dcat_paths
                            .iter()
                            .any(|p| p == &field_path || p.starts_with(&format!("{field_path}/")));
                        if is_forced {
                            continue;
                        }
                        // Per-field clone — only the fields that
                        // actually flow into the inferred record
                        // pay an allocation, and forced fields are
                        // already filtered out above.
                        merge_one(inf_obj, k, v.clone(), field_strategy);
                    }
                }
            },
            None => {
                if cfg.append_unmatched {
                    inferred_arr.push(Value::Object(disc_obj.clone()));
                }
            },
        }
    }
    Value::Array(inferred_arr)
}

/// Coerce a JSON value into a `Vec<Value>` so the merge loop can
/// treat single-object and array-of-object inputs uniformly. `Null`
/// becomes an empty Vec; objects/scalars become one-element Vecs.
fn coerce_to_array(v: Value) -> Vec<Value> {
    match v {
        Value::Array(a) => a,
        Value::Null => Vec::new(),
        other => vec![other],
    }
}

/// Find the index of the first inferred distribution that matches
/// `disc` on any identity key. Match is exact-string equality on a
/// non-empty value present on both sides. Returns `None` when no
/// identity key produces a match (e.g. publisher only has a title,
/// no URL).
fn find_distribution_match(
    inferred: &[Value],
    disc: &Map<String, Value>,
    identity_keys: &[String],
) -> Option<usize> {
    for key in identity_keys {
        let Some(disc_val) = disc.get(key).and_then(non_empty_string) else {
            continue;
        };
        for (i, inf) in inferred.iter().enumerate() {
            let Some(inf_val) = inf
                .as_object()
                .and_then(|o| o.get(key))
                .and_then(non_empty_string)
            else {
                continue;
            };
            if disc_val == inf_val {
                return Some(i);
            }
        }
    }
    None
}

/// Treat empty strings as absent so identity matching doesn't pair
/// distributions on a shared `""` value.
fn non_empty_string(v: &Value) -> Option<&str> {
    v.as_str().filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::cmd::profile::profile_spec::load_from_str;

    const PROFILE_WITH_NEVER: &str = r#"
name: test-merge
dataset:
  type: dcat:Dataset
discovery_merge:
  enabled: true
  never_overwrite: ["@context", "@type", "dcat:distribution"]
"#;

    #[test]
    fn fill_if_absent_strategy() {
        let profile = load_from_str(PROFILE_WITH_NEVER, "t").unwrap();
        let inferred = json!({ "@type": "dcat:Dataset", "dct:title": "Hello" });
        let discovered = json!({
            "@type": "dcat:Dataset",
            "dct:title": "OVERRIDE",
            "dct:description": "Filled"
        });
        let merged = merge(&profile, inferred, Some(&discovered), &[]);
        // dct:title stays inferred (fill-if-absent), description is filled.
        assert_eq!(
            merged.get("dct:title").and_then(Value::as_str),
            Some("Hello")
        );
        assert_eq!(
            merged.get("dct:description").and_then(Value::as_str),
            Some("Filled")
        );
    }

    #[test]
    fn never_overwrite_protected_keys() {
        let profile = load_from_str(PROFILE_WITH_NEVER, "t").unwrap();
        let inferred = json!({ "@type": "dcat:Dataset", "@context": "ctx-inferred" });
        let discovered = json!({ "@context": "ctx-discovered" });
        let merged = merge(&profile, inferred, Some(&discovered), &[]);
        assert_eq!(
            merged.get("@context").and_then(Value::as_str),
            Some("ctx-inferred")
        );
    }

    #[test]
    fn forced_paths_block_overlay() {
        let profile = load_from_str(PROFILE_WITH_NEVER, "t").unwrap();
        let inferred = json!({ "@type": "dcat:Dataset" });
        let discovered = json!({ "dct:title": "Discovered Title" });
        // Forced paths use the dataset_info form: /dcat/<key>.
        let merged = merge(
            &profile,
            inferred,
            Some(&discovered),
            &["/dcat/dct:title".to_string()],
        );
        assert!(merged.get("dct:title").is_none());
    }

    #[test]
    fn forced_nested_path_blocks_top_level_merge() {
        // A forced leaf path like /dcat/dcat:contactPoint/vcard:fn should
        // also block the top-level contactPoint key from being copied
        // wholesale from the discovered block.
        let profile = load_from_str(PROFILE_WITH_NEVER, "t").unwrap();
        let inferred = json!({ "@type": "dcat:Dataset" });
        let discovered = json!({
            "dcat:contactPoint": {"@type": "vcard:Individual", "vcard:fn": "DISCOVERED"}
        });
        let merged = merge(
            &profile,
            inferred,
            Some(&discovered),
            &["/dcat/dcat:contactPoint/vcard:fn".to_string()],
        );
        assert!(merged.get("dcat:contactPoint").is_none());
    }

    #[test]
    fn disabled_is_noop() {
        let yaml = r#"
name: croissant-style
dataset:
  type: sc:Dataset
discovery_merge:
  enabled: false
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let inferred = json!({ "@type": "sc:Dataset" });
        let discovered = json!({ "name": "Discovered" });
        let merged = merge(&profile, inferred.clone(), Some(&discovered), &[]);
        assert_eq!(merged, inferred);
    }

    #[test]
    fn overlay_array_strategy_appends() {
        let yaml = r#"
name: overlay-test
dataset:
  type: dcat:Dataset
discovery_merge:
  enabled: true
  default_strategy: overlay-array
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let inferred = json!({ "dcat:keyword": ["a", "b"] });
        let discovered = json!({ "dcat:keyword": ["c", "d"] });
        let merged = merge(&profile, inferred, Some(&discovered), &[]);
        let kw = merged
            .get("dcat:keyword")
            .and_then(Value::as_array)
            .unwrap();
        let strs: Vec<&str> = kw.iter().filter_map(Value::as_str).collect();
        assert_eq!(strs, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn forced_full_iri_key_blocks_matching_discovered_key() {
        // Roborev #2495 finding #2: regression coverage migrated from
        // the deleted merge_discovered tests. A forced path that
        // targets a full-IRI JSON-LD property
        // (`http://purl.org/dc/terms/title`) must use RFC 6901
        // escaping so the candidate path comparison sees the same
        // form. The discovered key carries the unescaped IRI; the
        // forced path uses `~1` for each `/`.
        let profile = load_from_str(PROFILE_WITH_NEVER, "t").unwrap();
        let inferred = json!({ "@type": "dcat:Dataset" });
        let discovered = json!({
            "http://purl.org/dc/terms/title": "Discovered IRI Title"
        });
        // Escaped per RFC 6901: each `/` → `~1`. Block.
        let forced = vec!["/dcat/http:~1~1purl.org~1dc~1terms~1title".to_string()];
        let merged = merge(&profile, inferred, Some(&discovered), &forced);
        assert!(
            merged.get("http://purl.org/dc/terms/title").is_none(),
            "forced full-IRI path must block the matching discovered key"
        );
    }

    #[test]
    fn forced_full_iri_key_does_not_block_unrelated_discovered_key() {
        // Companion: escaping must NOT over-match. A forced path for
        // `terms/title` shouldn't block the unrelated `dct:identifier`
        // discovered key.
        let profile = load_from_str(PROFILE_WITH_NEVER, "t").unwrap();
        let inferred = json!({ "@type": "dcat:Dataset" });
        let discovered = json!({ "dct:identifier": "id-123" });
        let forced = vec!["/dcat/http:~1~1purl.org~1dc~1terms~1title".to_string()];
        let merged = merge(&profile, inferred, Some(&discovered), &forced);
        assert_eq!(
            merged.get("dct:identifier").and_then(Value::as_str),
            Some("id-123"),
            "forced IRI path must not over-match unrelated discovered keys"
        );
    }

    #[test]
    fn escape_token_handles_rfc6901_round_trip() {
        // The internal escape_token must follow RFC 6901 §4: `~` →
        // `~0` first, then `/` → `~1` (order matters to avoid
        // double-escaping the `~` introduced by `/`).
        assert_eq!(escape_token(""), "");
        assert_eq!(escape_token("plain"), "plain");
        assert_eq!(escape_token("a/b"), "a~1b");
        assert_eq!(escape_token("a~b"), "a~0b");
        assert_eq!(escape_token("a~/b"), "a~0~1b");
        assert_eq!(
            escape_token("http://purl.org/dc/terms/title"),
            "http:~1~1purl.org~1dc~1terms~1title",
        );
    }

    const PROFILE_WITH_DIST_MERGE: &str = r#"
name: test-dist-merge
dataset:
  type: dcat:Dataset
discovery_merge:
  enabled: true
  never_overwrite: ["@context", "@type", "dcat:distribution"]
  default_strategy: fill-if-absent
  distribution_merge:
    enabled: true
    array_key: "dcat:distribution"
    identity_keys: ["dcat:downloadURL", "dcat:accessURL"]
    field_strategy: fill-if-absent
    append_unmatched: false
"#;

    #[test]
    fn dist_merge_pairs_on_download_url_and_fills_absent_fields() {
        let profile = load_from_str(PROFILE_WITH_DIST_MERGE, "t").unwrap();
        let inferred = json!({
            "@type": "dcat:Dataset",
            "dcat:distribution": [{
                "dcat:downloadURL": "https://example.org/data.csv",
                "dcat:mediaType": "text/csv",
                "dcat:byteSize": "42",
            }],
        });
        let discovered = json!({
            "dcat:distribution": [{
                "dcat:downloadURL": "https://example.org/data.csv",
                "dct:title": "Publisher Title",
                "dct:license": "https://creativecommons.org/licenses/by/4.0/",
                "dcat:byteSize": "OVERRIDE",
            }],
        });
        let merged = merge(&profile, inferred, Some(&discovered), &[]);
        let dist = merged
            .get("dcat:distribution")
            .and_then(Value::as_array)
            .expect("distribution array");
        assert_eq!(dist.len(), 1, "matched into a single element, no append");
        let entry = &dist[0];
        // Inferred wins on conflicts (fill-if-absent).
        assert_eq!(
            entry.get("dcat:byteSize").and_then(Value::as_str),
            Some("42"),
            "qsv-inferred byteSize must win"
        );
        // Publisher fields fill absent slots.
        assert_eq!(
            entry.get("dct:title").and_then(Value::as_str),
            Some("Publisher Title")
        );
        assert_eq!(
            entry.get("dct:license").and_then(Value::as_str),
            Some("https://creativecommons.org/licenses/by/4.0/")
        );
        // Inferred-only fields are preserved.
        assert_eq!(
            entry.get("dcat:mediaType").and_then(Value::as_str),
            Some("text/csv")
        );
    }

    #[test]
    fn dist_merge_falls_back_to_access_url_when_download_url_missing() {
        let profile = load_from_str(PROFILE_WITH_DIST_MERGE, "t").unwrap();
        let inferred = json!({
            "dcat:distribution": [{
                "dcat:accessURL": "https://catalog.example.org/datasets/abc",
                "qsv:sourcePath": "/local/data.csv",
            }],
        });
        let discovered = json!({
            "dcat:distribution": [{
                "dcat:accessURL": "https://catalog.example.org/datasets/abc",
                "dct:title": "ABC Dataset",
            }],
        });
        let merged = merge(&profile, inferred, Some(&discovered), &[]);
        let dist = merged
            .get("dcat:distribution")
            .and_then(Value::as_array)
            .unwrap();
        assert_eq!(dist.len(), 1);
        assert_eq!(
            dist[0].get("dct:title").and_then(Value::as_str),
            Some("ABC Dataset")
        );
    }

    #[test]
    fn dist_merge_drops_unmatched_when_append_disabled() {
        let profile = load_from_str(PROFILE_WITH_DIST_MERGE, "t").unwrap();
        let inferred = json!({
            "dcat:distribution": [{
                "dcat:downloadURL": "https://example.org/a.csv",
            }],
        });
        let discovered = json!({
            "dcat:distribution": [
                {"dcat:downloadURL": "https://example.org/MIRROR.csv", "dct:title": "Mirror"},
            ],
        });
        let merged = merge(&profile, inferred, Some(&discovered), &[]);
        let dist = merged
            .get("dcat:distribution")
            .and_then(Value::as_array)
            .unwrap();
        assert_eq!(dist.len(), 1, "unmatched publisher entry is dropped");
        assert!(
            dist[0].get("dct:title").is_none(),
            "the mirror dist should not have leaked title onto the inferred entry"
        );
    }

    #[test]
    fn dist_merge_appends_unmatched_when_configured() {
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
discovery_merge:
  enabled: true
  never_overwrite: ["dcat:distribution"]
  distribution_merge:
    enabled: true
    array_key: "dcat:distribution"
    identity_keys: ["dcat:downloadURL"]
    field_strategy: fill-if-absent
    append_unmatched: true
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let inferred = json!({
            "dcat:distribution": [{
                "dcat:downloadURL": "https://example.org/a.csv",
                "dcat:mediaType": "text/csv",
            }],
        });
        let discovered = json!({
            "dcat:distribution": [
                {"dcat:downloadURL": "https://example.org/MIRROR.csv", "dct:title": "Mirror"},
            ],
        });
        let merged = merge(&profile, inferred, Some(&discovered), &[]);
        let dist = merged
            .get("dcat:distribution")
            .and_then(Value::as_array)
            .unwrap();
        assert_eq!(dist.len(), 2, "unmatched publisher entry is appended");
        // Original inferred entry retained verbatim.
        assert_eq!(
            dist[0].get("dcat:mediaType").and_then(Value::as_str),
            Some("text/csv")
        );
        // Publisher-only entry appended.
        assert_eq!(
            dist[1].get("dct:title").and_then(Value::as_str),
            Some("Mirror")
        );
    }

    #[test]
    fn dist_merge_handles_single_object_inputs() {
        // Inputs may be single objects (not arrays); the helper must
        // coerce both sides uniformly.
        let profile = load_from_str(PROFILE_WITH_DIST_MERGE, "t").unwrap();
        let inferred = json!({
            "dcat:distribution": {
                "dcat:downloadURL": "https://example.org/x.csv",
                "dcat:mediaType": "text/csv",
            }
        });
        let discovered = json!({
            "dcat:distribution": {
                "dcat:downloadURL": "https://example.org/x.csv",
                "dct:license": "CC-BY-4.0",
            }
        });
        let merged = merge(&profile, inferred, Some(&discovered), &[]);
        let dist = merged
            .get("dcat:distribution")
            .and_then(Value::as_array)
            .unwrap();
        assert_eq!(dist.len(), 1);
        assert_eq!(
            dist[0].get("dct:license").and_then(Value::as_str),
            Some("CC-BY-4.0")
        );
        assert_eq!(
            dist[0].get("dcat:mediaType").and_then(Value::as_str),
            Some("text/csv")
        );
    }

    #[test]
    fn dist_merge_bypasses_never_overwrite_when_enabled() {
        // The outer `never_overwrite` lists `dcat:distribution` —
        // historically this means "publisher distribution array is
        // dropped wholesale". With distribution_merge enabled, that
        // rule is bypassed for the array key so per-element merging
        // can actually run.
        let profile = load_from_str(PROFILE_WITH_DIST_MERGE, "t").unwrap();
        assert!(
            profile
                .discovery_merge
                .never_overwrite
                .contains(&"dcat:distribution".to_string()),
            "test profile must list dcat:distribution in never_overwrite"
        );
        let inferred = json!({
            "dcat:distribution": [{
                "dcat:downloadURL": "https://example.org/x.csv",
            }],
        });
        let discovered = json!({
            "dcat:distribution": [{
                "dcat:downloadURL": "https://example.org/x.csv",
                "dct:title": "Publisher",
            }],
        });
        let merged = merge(&profile, inferred, Some(&discovered), &[]);
        let dist = merged.get("dcat:distribution").unwrap().as_array().unwrap();
        assert_eq!(
            dist[0].get("dct:title").and_then(Value::as_str),
            Some("Publisher"),
            "distribution_merge.enabled must override never_overwrite"
        );
    }

    #[test]
    fn dist_merge_disabled_preserves_never_overwrite() {
        // When the per-distribution config is absent (or enabled:
        // false), the legacy never_overwrite rule still wins and the
        // publisher distribution array is dropped wholesale.
        let profile = load_from_str(PROFILE_WITH_NEVER, "t").unwrap();
        assert!(profile.discovery_merge.distribution_merge.is_none());
        let inferred = json!({
            "dcat:distribution": [{"qsv:sourcePath": "/local/x.csv"}],
        });
        let discovered = json!({
            "dcat:distribution": [{"dct:title": "Publisher"}],
        });
        let merged = merge(&profile, inferred, Some(&discovered), &[]);
        let dist = merged.get("dcat:distribution").unwrap().as_array().unwrap();
        assert_eq!(dist.len(), 1);
        assert!(
            dist[0].get("dct:title").is_none(),
            "legacy never_overwrite still drops publisher distributions"
        );
    }

    #[test]
    fn dist_merge_empty_identity_keys_treats_all_as_unmatched() {
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
discovery_merge:
  enabled: true
  never_overwrite: ["dcat:distribution"]
  distribution_merge:
    enabled: true
    array_key: "dcat:distribution"
    identity_keys: []
    append_unmatched: true
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let inferred = json!({"dcat:distribution": [{"dcat:downloadURL": "x"}]});
        let discovered =
            json!({"dcat:distribution": [{"dcat:downloadURL": "x", "dct:title": "T"}]});
        let merged = merge(&profile, inferred, Some(&discovered), &[]);
        let dist = merged.get("dcat:distribution").unwrap().as_array().unwrap();
        // With no identity keys, the publisher entry can't pair —
        // so with append_unmatched: true it gets appended, not merged.
        assert_eq!(dist.len(), 2);
        assert!(dist[0].get("dct:title").is_none());
        assert_eq!(dist[1].get("dct:title").and_then(Value::as_str), Some("T"));
    }

    #[test]
    fn dist_merge_array_key_defaults_to_dcat_distribution() {
        // Roborev #2499 finding #2: enabling distribution_merge
        // without spelling out `array_key` must NOT silently disable
        // per-distribution merging. The default is `dcat:distribution`.
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
discovery_merge:
  enabled: true
  never_overwrite: ["dcat:distribution"]
  distribution_merge:
    enabled: true
    identity_keys: ["dcat:downloadURL"]
    field_strategy: fill-if-absent
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        assert!(
            profile
                .discovery_merge
                .distribution_merge
                .as_ref()
                .unwrap()
                .array_key
                .is_none(),
            "array_key intentionally omitted in fixture",
        );
        let inferred = json!({
            "dcat:distribution": [{
                "dcat:downloadURL": "https://example.org/x.csv",
                "dcat:mediaType": "text/csv",
            }],
        });
        let discovered = json!({
            "dcat:distribution": [{
                "dcat:downloadURL": "https://example.org/x.csv",
                "dct:title": "Default Key",
            }],
        });
        let merged = merge(&profile, inferred, Some(&discovered), &[]);
        let dist = merged.get("dcat:distribution").unwrap().as_array().unwrap();
        assert_eq!(
            dist[0].get("dct:title").and_then(Value::as_str),
            Some("Default Key"),
            "omitting array_key must default to dcat:distribution, not disable"
        );
    }

    #[test]
    fn dist_merge_honors_forced_path_on_distribution_field() {
        // Roborev #2499 finding #1: a forced dataset_info path at
        // /dcat/dcat:distribution/0/<field> must block the publisher
        // overlay on that field even when the inferred value is
        // absent. Without the forced-path check, publisher metadata
        // would silently fill the user's intentional override gap.
        let profile = load_from_str(PROFILE_WITH_DIST_MERGE, "t").unwrap();
        let inferred = json!({
            "dcat:distribution": [{
                "dcat:downloadURL": "https://example.org/x.csv",
                // Note: dct:title intentionally absent — the user has
                // a forced override at this path.
            }],
        });
        let discovered = json!({
            "dcat:distribution": [{
                "dcat:downloadURL": "https://example.org/x.csv",
                "dct:title": "Publisher Title",
                "dct:license": "https://creativecommons.org/licenses/by/4.0/",
            }],
        });
        // User forced the title on inferred[0] (the leaf they wanted
        // to override via --initial-context dataset_info).
        let forced = vec!["/dcat/dcat:distribution/0/dct:title".to_string()];
        let merged = merge(&profile, inferred, Some(&discovered), &forced);
        let dist = merged.get("dcat:distribution").unwrap().as_array().unwrap();
        assert!(
            dist[0].get("dct:title").is_none(),
            "forced dist field must block publisher overlay even when inferred is absent"
        );
        // Non-forced sibling still flows through.
        assert_eq!(
            dist[0].get("dct:license").and_then(Value::as_str),
            Some("https://creativecommons.org/licenses/by/4.0/"),
            "forced-path protection is leaf-specific, not whole-element"
        );
    }

    #[test]
    fn dist_merge_honors_forced_nested_path_under_distribution_field() {
        // A forced path INSIDE a publisher field
        // (`/dcat/dcat:distribution/0/dcat-us:accessRestriction/some-key`)
        // must block the publisher from replacing the whole parent
        // field. Mirrors the top-level
        // `forced_nested_path_blocks_top_level_merge` semantics for
        // the per-distribution path.
        let profile = load_from_str(PROFILE_WITH_DIST_MERGE, "t").unwrap();
        let inferred = json!({
            "dcat:distribution": [{
                "dcat:downloadURL": "https://example.org/x.csv",
            }],
        });
        let discovered = json!({
            "dcat:distribution": [{
                "dcat:downloadURL": "https://example.org/x.csv",
                "dcat-us:accessRestriction": {"label": "PUBLISHER"},
            }],
        });
        let forced = vec!["/dcat/dcat:distribution/0/dcat-us:accessRestriction/label".to_string()];
        let merged = merge(&profile, inferred, Some(&discovered), &forced);
        let dist = merged.get("dcat:distribution").unwrap().as_array().unwrap();
        assert!(
            dist[0].get("dcat-us:accessRestriction").is_none(),
            "forced sub-key under publisher field must block parent overlay"
        );
    }

    #[test]
    fn dist_merge_unrelated_forced_path_does_not_block() {
        // Companion: a forced path on a sibling distribution
        // (`/dcat/dcat:distribution/1/...`) must NOT block overlay
        // on index 0. Index-specific guarding is required so a user
        // override on one resource doesn't silently lock down the
        // whole array.
        let profile = load_from_str(PROFILE_WITH_DIST_MERGE, "t").unwrap();
        let inferred = json!({
            "dcat:distribution": [{
                "dcat:downloadURL": "https://example.org/x.csv",
            }],
        });
        let discovered = json!({
            "dcat:distribution": [{
                "dcat:downloadURL": "https://example.org/x.csv",
                "dct:title": "Title for index 0",
            }],
        });
        // Forced path targets index 1, not 0. Has no effect on
        // matched entry at index 0.
        let forced = vec!["/dcat/dcat:distribution/1/dct:title".to_string()];
        let merged = merge(&profile, inferred, Some(&discovered), &forced);
        let dist = merged.get("dcat:distribution").unwrap().as_array().unwrap();
        assert_eq!(
            dist[0].get("dct:title").and_then(Value::as_str),
            Some("Title for index 0"),
            "forced path on a different index must not over-match"
        );
    }
}
