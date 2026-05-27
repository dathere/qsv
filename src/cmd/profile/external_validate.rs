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

use std::{io::Write, process::Command};

use serde_json::Value;
use tempfile::NamedTempFile;

use super::{
    profile_spec::ProfileSpec,
    projection::{ProjectionWarning, Severity},
};

/// Run the profile's external validator (if any) against `block`.
///
/// Returns an empty Vec when:
///
/// * `profile.validation.external` is `None` (validator not configured).
/// * The validator command exits with status 0.
///
/// Returns one or more `ProjectionWarning`s when the validator is
/// configured but: (a) the binary isn't on PATH (single `Info`-level
/// notice), (b) spawn fails (single `Recommended`-level OS error), or
/// (c) the command exits non-zero (one warning per non-empty stderr
/// or stdout line).
pub fn validate(profile: &ProfileSpec, block: &Value) -> Vec<ProjectionWarning> {
    let Some(cfg) = profile.validation.external.as_ref() else {
        return Vec::new();
    };

    let severity = parse_severity(cfg.default_severity.as_deref());
    let label = cfg.label.as_deref().unwrap_or(cfg.command.as_str());

    // Write the rendered JSON-LD to a tempfile. mlcroissant + pyshacl
    // both accept a file path; piping on stdin would need per-tool
    // flags. Keep the contract uniform.
    let mut tmp = match write_tempfile(block) {
        Ok(t) => t,
        Err(e) => {
            return vec![ProjectionWarning {
                field:    "external_validate".to_string(),
                severity: Severity::Recommended,
                message:  format!("could not write JSON-LD tempfile for `{label}`: {e}"),
            }];
        },
    };

    let path_str = tmp.path().to_string_lossy().to_string();
    let resolved_args = resolve_args(&cfg.args, &path_str);

    let mut cmd = Command::new(&cfg.command);
    cmd.args(&resolved_args);
    let output = match cmd.output() {
        Ok(o) => o,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Graceful skip — the canonical validator simply isn't
            // installed on this machine. The install hint (when
            // provided) gives the user a one-line path to fixing
            // the gap without leaving the terminal.
            let _ = tmp.flush();
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
            field: format!("external_validate/{label}"),
            severity,
            message: trimmed.to_string(),
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
fn write_tempfile(block: &Value) -> Result<NamedTempFile, std::io::Error> {
    let mut tmp = NamedTempFile::with_suffix(".json")?;
    let bytes = serde_json::to_vec_pretty(block).unwrap_or_else(|_| b"{}".to_vec());
    tmp.write_all(&bytes)?;
    tmp.flush()?;
    Ok(tmp)
}

/// Substitute the literal `{file}` token in each arg with `path`. If
/// no arg contains the token, append `path` as a final argument so
/// validators that take "file as last positional" still work without
/// explicit templating.
fn resolve_args(args: &[String], path: &str) -> Vec<String> {
    let has_token = args.iter().any(|a| a.contains("{file}"));
    if has_token {
        args.iter().map(|a| a.replace("{file}", path)).collect()
    } else {
        let mut out: Vec<String> = args.to_vec();
        out.push(path.to_string());
        out
    }
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
        assert_eq!(warnings[0].message, "first issue");
        assert_eq!(warnings[1].message, "second issue");
        assert_eq!(warnings[0].field, "external_validate/fake-validator");
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
        assert_eq!(warnings[0].message, "stdout-finding");
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
        // First stdout line carries the substituted path.
        assert!(
            warnings.iter().any(|w| w.message.starts_with("got: /")),
            "{{file}} substitution must place the tempfile path in arg slot"
        );
        // Second confirms the file is non-empty (block was serialized).
        assert!(
            warnings.iter().any(|w| w.message == "nonempty"),
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
        // after `-c <script>`) prints it.
        assert!(
            warnings[0].message.starts_with("last arg: /"),
            "missing {{file}} token must append path as last arg"
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
        let out = resolve_args(&args, "/tmp/x.json");
        assert_eq!(out, vec!["validate", "--input", "/tmp/x.json"]);
    }

    #[test]
    fn resolve_args_appends_when_token_absent() {
        let args = vec!["validate".to_string(), "--strict".to_string()];
        let out = resolve_args(&args, "/tmp/x.json");
        assert_eq!(out, vec!["validate", "--strict", "/tmp/x.json"]);
    }
}
