//! Out-of-process validator integration.
//!
//! Some profiles ship with reference validators that live outside
//! Rust — Croissant uses `mlcroissant` (Python), DCAT-AP uses
//! `pyshacl`, etc. Rather than rebuild each one natively (the Rust
//! ecosystem for SHACL/JSON-LD validators is sparse), profiles can
//! declare an external command in `validation.external` and the
//! engine spawns it with the rendered JSON-LD on disk.
//!
//! Graceful degradation is the design goal:
//!
//! * Missing binary → one `Severity::Info` warning ("`<cmd>` not installed; skipped validation"),
//!   projection still ships.
//! * Spawn error → one `Severity::Recommended` warning with the OS error message.
//! * Non-zero exit → each non-empty stderr line becomes one warning with the configured
//!   `default_severity`. Stdout content is surfaced separately when stderr is empty (some
//!   validators emit findings on stdout).
//! * Exit code zero → empty Vec (success).
//!
//! No timeout is enforced today. `run_profile_validation` (the
//! sibling RFC4180 validator) follows the same convention; adding
//! `wait-timeout` is a queued follow-up if validators start hanging
//! on huge inputs.

use std::{
    ffi::{OsStr, OsString},
    io::Write,
    process::Command,
};

use serde_json::Value;
use tempfile::NamedTempFile;

use super::{
    profile_spec::ProfileSpec,
    projection::{ProjectionWarning, Severity},
};

/// Compile-time-bundled resources resolvable by
/// `ExternalValidatorResource.embedded`. Each entry is an
/// `(<name>, <content>)` pair sourced via `include_str!`. Custom
/// YAML profiles can reference any name listed here from their
/// `validation.external.resources[].embedded` field but cannot
/// register new entries — that's a qsv-release-time decision.
///
/// Adding a new resource: vendor the file under `resources/<slug>/`
/// (with a sibling `README.md` documenting source + re-vendor
/// procedure) and add one tuple here.
/// Embedded resource table. The `geoconnex-shacl-shapes` entry is
/// gated behind the `geoconnex` cargo feature so qsvlite / qsvmcp /
/// default qsvdp builds don't pay the ~10 KB binary cost. The two
/// `cfg`'d definitions differ only by that one tuple.
#[cfg(not(feature = "geoconnex"))]
pub const EMBEDDED_RESOURCES: &[(&str, &str)] = &[(
    "dcat-ap-v3-shacl-shapes",
    include_str!("../../../resources/dcat-ap-v3/shacl/dcat-ap-SHACL.ttl"),
)];

#[cfg(feature = "geoconnex")]
pub const EMBEDDED_RESOURCES: &[(&str, &str)] = &[
    (
        "dcat-ap-v3-shacl-shapes",
        include_str!("../../../resources/dcat-ap-v3/shacl/dcat-ap-SHACL.ttl"),
    ),
    (
        "geoconnex-shacl-shapes",
        include_str!("../../../resources/geoconnex/shacl/geoconnex.ttl"),
    ),
];

/// Resolve an embedded-resource identifier to its content (or
/// `None` when the name isn't bundled).
fn lookup_embedded(name: &str) -> Option<&'static str> {
    EMBEDDED_RESOURCES
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, c)| *c)
}

pub fn validate(profile: &ProfileSpec, block: &Value) -> Vec<ProjectionWarning> {
    let Some(cfg) = profile.validation.external.as_ref() else {
        return Vec::new();
    };

    let severity = parse_severity(cfg.default_severity.as_deref());
    let label = cfg.label.as_deref().unwrap_or(cfg.command.as_str());

    // Validate the resource declarations BEFORE any I/O so a
    // misconfigured profile fails fast with an actionable message.
    // Reserved-name + unknown-embedded checks are Required-severity
    // because they reflect a YAML bug, not a runtime gap.
    for r in &cfg.resources {
        if r.name == "file" {
            return vec![ProjectionWarning {
                field:    "external_validate".to_string(),
                severity: Severity::Required,
                message:  format!(
                    "profile `{}`: external validator resource name `file` is reserved for the \
                     implicit JSON-LD tempfile. Rename the resource.",
                    profile.name
                ),
            }];
        }
        if lookup_embedded(&r.embedded).is_none() {
            let known: Vec<&str> = EMBEDDED_RESOURCES.iter().map(|(n, _)| *n).collect();
            return vec![ProjectionWarning {
                field:    "external_validate".to_string(),
                severity: Severity::Required,
                message:  format!(
                    "profile `{}`: external validator resource `{}` references unknown embedded \
                     `{}`; known: [{}]",
                    profile.name,
                    r.name,
                    r.embedded,
                    known.join(", ")
                ),
            }];
        }
    }

    // Write the rendered JSON-LD to a tempfile. mlcroissant + pyshacl
    // both accept a file path; piping on stdin would need per-tool
    // flags. Keep the contract uniform.
    let mut tmp_jsonld = match write_jsonld_tempfile(block) {
        Ok(t) => t,
        Err(e) => {
            return vec![ProjectionWarning {
                field:    "external_validate".to_string(),
                severity: Severity::Recommended,
                message:  format!("could not write JSON-LD tempfile for `{label}`: {e}"),
            }];
        },
    };

    // Materialize each declared resource tempfile (e.g. SHACL
    // shapes). Tempfiles must outlive the spawn — keep them in a
    // Vec alongside their token bindings.
    let mut resource_tmps: Vec<NamedTempFile> = Vec::with_capacity(cfg.resources.len());
    let mut extras: Vec<(String, OsString)> = Vec::with_capacity(cfg.resources.len());
    for r in &cfg.resources {
        let content =
            lookup_embedded(&r.embedded).expect("resource embedded existence checked above");
        let suffix = r.suffix.as_deref().unwrap_or(".tmp");
        let tmp = match write_resource_tempfile(content, suffix) {
            Ok(t) => t,
            Err(e) => {
                return vec![ProjectionWarning {
                    field:    "external_validate".to_string(),
                    severity: Severity::Recommended,
                    message:  format!(
                        "could not materialize resource `{}` ({}) tempfile for `{label}`: {e}",
                        r.name, r.embedded
                    ),
                }];
            },
        };
        extras.push((r.name.clone(), tmp.path().as_os_str().to_owned()));
        resource_tmps.push(tmp);
    }

    // Keep the JSON-LD tempfile path as `OsString` so non-UTF-8
    // paths (legal on Unix, possible on Windows) pass through to
    // the spawned validator verbatim. `to_string_lossy()` would
    // silently mangle such paths and the validator would then
    // complain that the file doesn't exist.
    let jsonld_path = tmp_jsonld.path().as_os_str().to_owned();
    let resolved_args = resolve_args(&cfg.args, &jsonld_path, &extras);

    let mut cmd = Command::new(&cfg.command);
    cmd.args(&resolved_args);
    let output = match cmd.output() {
        Ok(o) => o,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Graceful skip — the canonical validator simply isn't
            // installed on this machine. The install hint (when
            // provided) gives the user a one-line path to fixing
            // the gap without leaving the terminal.
            let _ = tmp_jsonld.flush();
            let hint = cfg
                .install_hint
                .as_deref()
                .map(|h| format!(" Install: {h}"))
                .unwrap_or_default();
            return vec![ProjectionWarning {
                field:    "external_validate".to_string(),
                severity: Severity::Info,
                message:  format!(
                    "`{label}` not installed; skipped {} validation.{hint}",
                    profile.name
                ),
            }];
        },
        Err(e) => {
            return vec![ProjectionWarning {
                field:    "external_validate".to_string(),
                severity: Severity::Recommended,
                message:  format!("failed to spawn `{label}`: {e}"),
            }];
        },
    };

    // Bind so resource tempfiles outlive the spawn explicitly
    // (the Vec drops here, taking the NamedTempFiles with it).
    drop(resource_tmps);

    if output.status.success() {
        return Vec::new();
    }

    // Non-zero exit: surface findings. Prefer stderr (most validators
    // log there); fall back to stdout when stderr is empty.
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let blob = if stderr.trim().is_empty() {
        stdout.as_ref()
    } else {
        stderr.as_ref()
    };

    let mut warnings = Vec::new();
    for line in blob.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        warnings.push(ProjectionWarning {
            // Keep `field` stable. The rest of the projection
            // pipeline treats it as a JSON-LD key / pointer;
            // encoding a user-configurable label here (which may
            // contain `/` or whitespace) would make it look like
            // a JSON pointer and confuse downstream filtering.
            // The validator label is already carried in the
            // message so users can still trace findings back to
            // their source.
            field: "external_validate".to_string(),
            severity,
            message: format!("{label}: {trimmed}"),
        });
    }

    // Defensive fallback: validator exited non-zero with no output.
    if warnings.is_empty() {
        warnings.push(ProjectionWarning {
            field: "external_validate".to_string(),
            severity,
            message: format!(
                "`{label}` exited with status {} but produced no output",
                output
                    .status
                    .code()
                    .map_or_else(|| "unknown".to_string(), |c| c.to_string())
            ),
        });
    }
    warnings
}

/// Write `block` to a `NamedTempFile` as pretty-printed JSON so the
/// validator's error line numbers line up with something a human can
/// actually grep. The tempfile is returned so its `Drop` only fires
/// after the caller has spawned and waited on the validator.
fn write_jsonld_tempfile(block: &Value) -> Result<NamedTempFile, std::io::Error> {
    let mut tmp = NamedTempFile::with_suffix(".json")?;
    let bytes = serde_json::to_vec_pretty(block).unwrap_or_else(|_| b"{}".to_vec());
    tmp.write_all(&bytes)?;
    tmp.flush()?;
    Ok(tmp)
}

/// Write `content` to a `NamedTempFile` with the given suffix so
/// validators that key off file extension (e.g. `pyshacl` reading
/// `.ttl` as Turtle) can dispatch correctly. The suffix should
/// include the leading dot (e.g. `.ttl`, `.jsonld`).
fn write_resource_tempfile(content: &str, suffix: &str) -> Result<NamedTempFile, std::io::Error> {
    let mut tmp = NamedTempFile::with_suffix(suffix)?;
    tmp.write_all(content.as_bytes())?;
    tmp.flush()?;
    Ok(tmp)
}

/// Substitute the literal `{file}` token in each arg with
/// `file_path`. Named tokens in `extras` (e.g. `{shapes}`) are
/// substituted with the corresponding `OsString` path. If no
/// `{file}` token appears anywhere in `args`, `file_path` is
/// appended as a final argument so validators that take "file as
/// last positional" still work without explicit templating; the
/// same fallback does NOT apply to named extras (their position
/// is always explicit).
///
/// Args themselves come from YAML and are guaranteed UTF-8, but
/// substituted paths are `OsStr`/`OsString` so non-UTF-8 tempfile
/// paths flow through to the spawned process unchanged. The output
/// is `Vec<OsString>`, which `Command::args` accepts directly.
fn resolve_args(
    args: &[String],
    file_path: &OsStr,
    extras: &[(String, OsString)],
) -> Vec<OsString> {
    let has_file_token = args.iter().any(|a| a.contains("{file}"));
    let substituted: Vec<OsString> = args
        .iter()
        .map(|a| substitute_tokens(a, file_path, extras))
        .collect();
    if has_file_token {
        substituted
    } else {
        let mut out = substituted;
        out.push(file_path.to_os_string());
        out
    }
}

/// Walk `arg` looking for `{<name>}` tokens. `{file}` resolves to
/// `file_path`; other names resolve via `extras`. Unknown tokens
/// (and unclosed braces) pass through as literal text so a
/// validator that genuinely wants `{foo}` in its CLI gets it
/// verbatim.
///
/// Stitches the OsStr path segments into an `OsString` so the
/// substituted bytes pass through `Command::args` unchanged on all
/// platforms.
fn substitute_tokens(arg: &str, file_path: &OsStr, extras: &[(String, OsString)]) -> OsString {
    let mut out = OsString::new();
    let mut rest = arg;
    while let Some(open) = rest.find('{') {
        // Push the literal prefix up to and not including `{`.
        out.push(&rest[..open]);
        let after_open = &rest[open + 1..];
        let Some(close) = after_open.find('}') else {
            // No closing brace — push the rest verbatim (including
            // the unmatched `{`) and bail out of the loop.
            out.push(&rest[open..]);
            return out;
        };
        let name = &after_open[..close];
        let resolved: Option<&OsStr> = if name == "file" {
            Some(file_path)
        } else {
            extras
                .iter()
                .find(|(n, _)| n == name)
                .map(|(_, p)| p.as_os_str())
        };
        if let Some(p) = resolved {
            out.push(p)
        } else {
            // Unknown token — pass through as literal so the
            // validator sees exactly what the user wrote.
            out.push("{");
            out.push(name);
            out.push("}");
        }
        rest = &after_open[close + 1..];
    }
    out.push(rest);
    out
}

/// Parse the configured severity string ("required" / "recommended"
/// / "optional" / "info"), case-insensitive. Unknown or absent
/// values default to `Recommended` — non-fatal but visible.
fn parse_severity(raw: Option<&str>) -> Severity {
    match raw.map(str::to_ascii_lowercase).as_deref() {
        Some("required") => Severity::Required,
        Some("optional") => Severity::Optional,
        Some("info") => Severity::Info,
        _ => Severity::Recommended,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::cmd::profile::profile_spec::load_from_str;

    #[test]
    fn no_external_config_returns_empty() {
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
validation:
  enabled: false
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        assert!(validate(&profile, &json!({})).is_empty());
    }

    #[test]
    fn missing_binary_yields_info_warning_not_failure() {
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
validation:
  enabled: false
  external:
    command: "qsv-no-such-validator-binary-12345"
    args: ["validate", "{file}"]
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let warnings = validate(&profile, &json!({"@type": "dcat:Dataset"}));
        assert_eq!(warnings.len(), 1);
        assert!(matches!(warnings[0].severity, Severity::Info));
        assert!(
            warnings[0].message.contains("not installed"),
            "graceful-skip warning must say so explicitly: got `{}`",
            warnings[0].message
        );
    }

    #[test]
    fn missing_binary_warning_includes_install_hint() {
        // The install_hint field surfaces in the missing-binary
        // message so users see the install command at the moment
        // they discover the validator is absent.
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
validation:
  enabled: false
  external:
    command: "qsv-no-such-validator-binary-12345"
    label: "mlcroissant"
    args: ["validate"]
    install_hint: "pip install mlcroissant (https://github.com/mlcommons/croissant)"
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let warnings = validate(&profile, &json!({}));
        assert_eq!(warnings.len(), 1);
        assert!(matches!(warnings[0].severity, Severity::Info));
        assert!(
            warnings[0].message.contains("pip install mlcroissant"),
            "install_hint must appear verbatim in the warning: got `{}`",
            warnings[0].message
        );
        assert!(
            warnings[0]
                .message
                .contains("github.com/mlcommons/croissant"),
            "install_hint URL must reach the user: got `{}`",
            warnings[0].message
        );
        assert!(
            warnings[0].message.contains("Install:"),
            "install hint must be prefixed so the user knows it's an action: got `{}`",
            warnings[0].message
        );
    }

    #[test]
    fn label_overrides_command_in_message() {
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
validation:
  enabled: false
  external:
    command: "qsv-no-such-validator-binary-12345"
    label: "mlcroissant"
    args: ["validate"]
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let warnings = validate(&profile, &json!({}));
        assert!(
            warnings[0].message.starts_with("`mlcroissant`"),
            "label must shadow command in user-facing message"
        );
    }

    #[test]
    fn successful_exit_yields_empty_findings() {
        // `true` is a Unix builtin that exits 0 with no output —
        // perfect stand-in for a validator that passes.
        if !cfg!(unix) {
            return;
        }
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
validation:
  enabled: false
  external:
    command: "true"
    args: []
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        assert!(validate(&profile, &json!({})).is_empty());
    }

    #[test]
    fn non_zero_exit_surfaces_one_warning_per_stderr_line() {
        // `sh -c 'printf "first issue\nsecond issue\n" 1>&2; exit 1'`
        // exits non-zero with two stderr lines, simulating a validator
        // that found two findings.
        if !cfg!(unix) {
            return;
        }
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
validation:
  enabled: false
  external:
    command: "sh"
    args:
      - "-c"
      - 'printf "first issue\nsecond issue\n" 1>&2; exit 1'
    default_severity: "required"
    label: "fake-validator"
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let warnings = validate(&profile, &json!({}));
        assert_eq!(warnings.len(), 2);
        assert!(matches!(warnings[0].severity, Severity::Required));
        // Field stays stable so downstream filters can target a
        // single string; the validator label lives in the message
        // so users can still trace findings back to source.
        assert_eq!(warnings[0].field, "external_validate");
        assert_eq!(warnings[1].field, "external_validate");
        assert_eq!(warnings[0].message, "fake-validator: first issue");
        assert_eq!(warnings[1].message, "fake-validator: second issue");
    }

    #[test]
    fn falls_back_to_stdout_when_stderr_empty() {
        // Some validators log on stdout. When stderr is empty AND
        // exit is non-zero, stdout becomes the finding source.
        if !cfg!(unix) {
            return;
        }
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
validation:
  enabled: false
  external:
    command: "sh"
    args: ["-c", 'echo stdout-finding; exit 2']
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let warnings = validate(&profile, &json!({}));
        assert_eq!(warnings.len(), 1);
        // Default label is the command name when none is configured.
        assert_eq!(warnings[0].message, "sh: stdout-finding");
        assert_eq!(warnings[0].field, "external_validate");
    }

    #[test]
    fn non_zero_exit_with_no_output_yields_one_diagnostic() {
        if !cfg!(unix) {
            return;
        }
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
validation:
  enabled: false
  external:
    command: "sh"
    args: ["-c", "exit 3"]
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let warnings = validate(&profile, &json!({}));
        assert_eq!(warnings.len(), 1);
        assert!(
            warnings[0].message.contains("exited with status 3"),
            "diagnostic must include the exit code"
        );
    }

    #[test]
    fn args_substitute_file_token() {
        // The {file} token must be replaced with the tempfile path.
        // Echo the substituted arg on stdout so we can verify.
        if !cfg!(unix) {
            return;
        }
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
validation:
  enabled: false
  external:
    command: "sh"
    args:
      - "-c"
      - 'echo "got: $1"; test -s "$1" && echo "nonempty"; exit 1'
      - "_"
      - "{file}"
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let warnings = validate(&profile, &json!({"@type": "dcat:Dataset"}));
        // First stdout line carries the substituted path (now
        // prefixed with the label-as-command + ": " per the stable
        // message format).
        assert!(
            warnings.iter().any(|w| w.message.starts_with("sh: got: /")),
            "{{file}} substitution must place the tempfile path in arg slot"
        );
        // Second confirms the file is non-empty (block was serialized).
        assert!(
            warnings.iter().any(|w| w.message == "sh: nonempty"),
            "tempfile must contain the serialized JSON-LD"
        );
    }

    #[test]
    fn args_without_token_get_file_appended() {
        if !cfg!(unix) {
            return;
        }
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
validation:
  enabled: false
  external:
    command: "sh"
    args: ["-c", 'echo "last arg: $1"; exit 1', "_"]
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let warnings = validate(&profile, &json!({}));
        // The tempfile path was appended as the final positional arg
        // after "_", and `$1` (counting from the script's first arg
        // after `-c <script>`) prints it. Message format is
        // "<label>: <line>"; no label set so the command name `sh`
        // is used.
        assert!(
            warnings[0].message.starts_with("sh: last arg: /"),
            "missing {{file}} token must append path as last arg; got `{}`",
            warnings[0].message
        );
    }

    #[test]
    fn parse_severity_handles_known_levels() {
        assert!(matches!(
            parse_severity(Some("required")),
            Severity::Required
        ));
        assert!(matches!(
            parse_severity(Some("RECOMMENDED")),
            Severity::Recommended
        ));
        assert!(matches!(
            parse_severity(Some("Optional")),
            Severity::Optional
        ));
        assert!(matches!(parse_severity(Some("info")), Severity::Info));
        // Unknown / absent default to Recommended.
        assert!(matches!(parse_severity(None), Severity::Recommended));
        assert!(matches!(
            parse_severity(Some("bogus")),
            Severity::Recommended
        ));
    }

    #[test]
    fn resolve_args_substitutes_token_when_present() {
        let args = vec![
            "validate".to_string(),
            "--input".to_string(),
            "{file}".to_string(),
        ];
        let out = resolve_args(&args, OsStr::new("/tmp/x.json"), &[]);
        let expected: Vec<OsString> = ["validate", "--input", "/tmp/x.json"]
            .iter()
            .map(OsString::from)
            .collect();
        assert_eq!(out, expected);
    }

    #[test]
    fn resolve_args_appends_when_token_absent() {
        let args = vec!["validate".to_string(), "--strict".to_string()];
        let out = resolve_args(&args, OsStr::new("/tmp/x.json"), &[]);
        let expected: Vec<OsString> = ["validate", "--strict", "/tmp/x.json"]
            .iter()
            .map(OsString::from)
            .collect();
        assert_eq!(out, expected);
    }

    #[cfg(unix)]
    #[test]
    fn resolve_args_preserves_non_utf8_path_bytes() {
        // Unix paths can contain arbitrary bytes including invalid
        // UTF-8 sequences. The substituted path must reach
        // Command::args verbatim — using to_string_lossy() instead
        // (the pre-fix code path) would replace those bytes with
        // U+FFFD and the spawned validator would then get a path
        // that doesn't exist on disk.
        use std::os::unix::ffi::OsStrExt;
        let raw = b"/tmp/\xFFnon-utf8\xFE.json";
        let path = OsStr::from_bytes(raw);
        let args = vec!["--input".to_string(), "{file}".to_string()];
        let out = resolve_args(&args, path, &[]);
        // The second arg must be exactly the original byte sequence.
        assert_eq!(
            out[1].as_bytes(),
            raw,
            "non-UTF-8 path bytes must survive substitution"
        );
    }

    #[test]
    fn resolve_args_substitutes_named_extras() {
        // {file} and {shapes} both substitute in one pass; unknown
        // tokens pass through verbatim so a validator can still get
        // `{literal-brace}` in its CLI if it expects one.
        let args = vec![
            "-s".to_string(),
            "{shapes}".to_string(),
            "-d".to_string(),
            "{file}".to_string(),
            "--note=keep-{verbatim}".to_string(),
        ];
        let extras = vec![("shapes".to_string(), OsString::from("/tmp/shapes.ttl"))];
        let out = resolve_args(&args, OsStr::new("/tmp/data.json"), &extras);
        let expected: Vec<OsString> = [
            "-s",
            "/tmp/shapes.ttl",
            "-d",
            "/tmp/data.json",
            "--note=keep-{verbatim}",
        ]
        .iter()
        .map(OsString::from)
        .collect();
        assert_eq!(out, expected);
    }

    #[test]
    fn embedded_resources_table_includes_dcat_ap_v3_shapes() {
        // Lock in the embedded resource name DCAT-AP v3 references
        // from its YAML — a typo here would silently break the
        // shipped profile.
        assert!(
            lookup_embedded("dcat-ap-v3-shacl-shapes").is_some(),
            "EMBEDDED_RESOURCES must include the DCAT-AP v3 SHACL shapes by canonical name"
        );
        let shapes = lookup_embedded("dcat-ap-v3-shacl-shapes").unwrap();
        // Sanity check: the bundle is real Turtle, not e.g. an HTML
        // 404 page caught by a bad re-vendor step.
        assert!(
            shapes.contains("@prefix shacl:"),
            "embedded SHACL shapes must declare the SHACL prefix"
        );
        assert!(
            shapes.contains("dcat:Dataset"),
            "embedded SHACL shapes must reference dcat:Dataset"
        );
    }

    #[cfg(feature = "geoconnex")]
    #[test]
    fn embedded_resources_table_includes_geoconnex_shapes() {
        // Same lock-in as the DCAT-AP v3 shapes test — a typo in the
        // EMBEDDED_RESOURCES key would silently break the bundled
        // `geoconnex` profile.
        let shapes = lookup_embedded("geoconnex-shacl-shapes")
            .expect("EMBEDDED_RESOURCES must include the Geoconnex SHACL shapes by canonical name");
        // Sanity check: bundle is real Turtle, not e.g. a corrupted
        // re-vendor.
        assert!(
            shapes.contains("@prefix sh:"),
            "embedded Geoconnex shapes must declare the SHACL prefix"
        );
        assert!(
            shapes.contains("schema:Dataset"),
            "embedded Geoconnex shapes must reference schema:Dataset (target of DatasetShape)"
        );
        assert!(
            shapes.contains("schema:DataDownload"),
            "embedded Geoconnex shapes must reference schema:DataDownload (target of \
             DistributionShape)"
        );
    }

    #[test]
    fn lookup_embedded_returns_none_for_unknown() {
        assert!(lookup_embedded("does-not-exist").is_none());
        assert!(lookup_embedded("").is_none());
    }

    #[test]
    fn resource_with_reserved_name_file_is_rejected() {
        // The implicit `{file}` token always points at the rendered
        // JSON-LD tempfile, so a resource named `file` would shadow
        // it. The framework must reject that loudly.
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
validation:
  enabled: false
  external:
    command: "sh"
    args: ["-c", "exit 0"]
    resources:
      - name: "file"
        embedded: "dcat-ap-v3-shacl-shapes"
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let warnings = validate(&profile, &json!({}));
        assert_eq!(warnings.len(), 1);
        assert!(matches!(warnings[0].severity, Severity::Required));
        assert!(
            warnings[0].message.contains("`file` is reserved"),
            "reserved-name error must call out the conflict; got `{}`",
            warnings[0].message
        );
    }

    #[test]
    fn resource_with_unknown_embedded_is_rejected() {
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
validation:
  enabled: false
  external:
    command: "sh"
    args: ["-c", "exit 0"]
    resources:
      - name: "shapes"
        embedded: "no-such-bundled-resource-12345"
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let warnings = validate(&profile, &json!({}));
        assert_eq!(warnings.len(), 1);
        assert!(matches!(warnings[0].severity, Severity::Required));
        assert!(
            warnings[0].message.contains("unknown embedded"),
            "unknown-embedded error must say so; got `{}`",
            warnings[0].message
        );
        assert!(
            warnings[0].message.contains("dcat-ap-v3-shacl-shapes"),
            "error must list known embedded names so users can spot typos; got `{}`",
            warnings[0].message
        );
    }

    #[cfg(unix)]
    #[test]
    fn resource_tempfile_is_materialized_with_correct_suffix() {
        // The resource tempfile must (a) exist when the validator
        // runs, (b) carry the configured suffix, (c) hold the
        // embedded content. Using `sh` to echo back the metadata
        // proves all three in one spawn.
        let yaml = r#"
name: t
dataset:
  type: dcat:Dataset
validation:
  enabled: false
  external:
    command: "sh"
    args:
      - "-c"
      - 'echo "shapes-path: $1"; head -c 100 "$1"; exit 1'
      - "_"
      - "{shapes}"
    label: "fake-validator"
    resources:
      - name: "shapes"
        embedded: "dcat-ap-v3-shacl-shapes"
        suffix: ".ttl"
"#;
        let profile = load_from_str(yaml, "t").unwrap();
        let warnings = validate(&profile, &json!({}));
        // The shapes tempfile path was substituted and exists.
        let path_line = warnings
            .iter()
            .find_map(|w| w.message.strip_prefix("fake-validator: shapes-path: "))
            .expect("shapes path substituted into args");
        assert!(
            path_line.starts_with('/'),
            "shapes path must be an absolute tempfile path; got `{path_line}`"
        );
        assert!(
            path_line.ends_with(".ttl"),
            "suffix `.ttl` must apply to the materialized tempfile; got `{path_line}`"
        );
        // The content is real Turtle (head of the SHACL bundle).
        assert!(
            warnings.iter().any(|w| w.message.contains("@prefix")),
            "shapes content must be the embedded SHACL Turtle; got warnings: {warnings:#?}"
        );
    }
}
