//! Tests for the tool definitions embedded in the qsv/qsvmcp binaries (issue #4638):
//! `--tool-definition <command>` and `--export-tool-definitions <dir>`.

use std::{fs, path::Path};

use crate::workdir::Workdir;

/// The committed skill JSONs that build.rs embeds.
fn repo_skills_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(".claude/skills/qsv")
}

fn count_files(dir: &Path, prefix: &str, suffix: &str) -> usize {
    fs::read_dir(dir)
        .unwrap()
        .flatten()
        .filter(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            name.starts_with(prefix) && name.ends_with(suffix)
        })
        .count()
}

#[test]
fn tool_definition_is_the_committed_json() {
    let wrk = Workdir::new("tool_definition_is_the_committed_json");
    let mut cmd = wrk.command("--tool-definition");
    cmd.arg("stats");

    let output = wrk.output(&mut cmd);
    assert!(output.status.success());

    // byte-identical to the committed file proves the embed is that file, not a regeneration
    let expected = fs::read(repo_skills_dir().join("qsv-stats.json")).unwrap();
    assert_eq!(output.stdout, expected);

    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["name"], "qsv-stats");
}

#[test]
fn tool_definition_unknown_command() {
    let wrk = Workdir::new("tool_definition_unknown_command");
    let mut cmd = wrk.command("--tool-definition");
    cmd.arg("nosuchcommand");

    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("is not a command in this qsv binary"),
        "{stderr}"
    );
}

#[test]
fn tool_definition_installed_command_without_definition() {
    // behead is installed but deliberately outside the MCP command set
    let wrk = Workdir::new("tool_definition_installed_command_without_definition");
    let mut cmd = wrk.command("--tool-definition");
    cmd.arg("behead");

    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("has no embedded tool definition"),
        "{stderr}"
    );
}

#[test]
fn export_tool_definitions() {
    let wrk = Workdir::new("export_tool_definitions");
    let mut cmd = wrk.command("--export-tool-definitions");
    cmd.arg("defs");
    wrk.assert_success(&mut cmd);

    let out = wrk.path("defs");
    let skills = out.join("mcp_skills");
    let help = out.join("help");

    // every committed skill is an MCP command, and both qsv and qsvmcp install all of them
    let expected_skills = count_files(&repo_skills_dir(), "qsv-", ".json");
    assert!(expected_skills > 0);
    assert_eq!(count_files(&skills, "qsv-", ".json"), expected_skills);
    assert_eq!(
        fs::read(skills.join("qsv-stats.json")).unwrap(),
        fs::read(repo_skills_dir().join("qsv-stats.json")).unwrap()
    );

    // help covers every skill command (and more), plus the table of contents
    assert!(count_files(&help, "", ".md") > expected_skills);
    assert!(help.join("TableOfContents.md").is_file());
    assert!(help.join("stats.md").is_file());

    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(out.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["qsv_version"], env!("CARGO_PKG_VERSION"));
    let expected_bin = if cfg!(feature = "qsvmcp") {
        "qsvmcp"
    } else {
        "qsv"
    };
    assert_eq!(manifest["binary"], expected_bin);
    assert_eq!(
        manifest["mcp_skills"].as_array().unwrap().len(),
        expected_skills
    );
    assert!(
        manifest["commands"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("stats"))
    );
}
