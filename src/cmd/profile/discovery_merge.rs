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

/// Public entry point. Returns the merged Dataset block.
///
/// Today's semantics (minimum-viable Phase 4b): the inferred projection
/// wins; discovered only fills gaps (`fill-if-absent`). Full override
/// semantics is queued as plan §5.4.
pub fn merge(
    profile: &ProfileSpec,
    inferred: Value,
    discovered: Option<Value>,
    forced_paths: &[String],
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
    let Value::Object(discovered_obj) = discovered else {
        return Value::Object(inferred_obj);
    };

    let never = &profile.discovery_merge.never_overwrite;
    let strategy = profile
        .discovery_merge
        .default_strategy
        .as_deref()
        .unwrap_or("fill-if-absent");

    for (key, value) in discovered_obj {
        if never.iter().any(|k| k == &key) {
            continue;
        }
        if is_forced(&key, forced_paths) {
            continue;
        }
        merge_one(&mut inferred_obj, &key, value, strategy);
    }
    Value::Object(inferred_obj)
}

fn is_forced(key: &str, forced_paths: &[String]) -> bool {
    forced_paths
        .iter()
        .any(|p| p == key || p == &format!("/{key}") || p.starts_with(&format!("/{key}/")))
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
        let merged = merge(&profile, inferred, Some(discovered), &[]);
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
        let merged = merge(&profile, inferred, Some(discovered), &[]);
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
        let merged = merge(
            &profile,
            inferred,
            Some(discovered),
            &["/dct:title".to_string()],
        );
        assert!(merged.get("dct:title").is_none());
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
        let merged = merge(&profile, inferred.clone(), Some(discovered), &[]);
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
        let merged = merge(&profile, inferred, Some(discovered), &[]);
        let kw = merged
            .get("dcat:keyword")
            .and_then(Value::as_array)
            .unwrap();
        let strs: Vec<&str> = kw.iter().filter_map(Value::as_str).collect();
        assert_eq!(strs, vec!["a", "b", "c", "d"]);
    }
}
