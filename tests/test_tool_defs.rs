//! Tests for the tool definitions embedded in the qsv/qsvmcp binaries (issue #4638):
//! `--tool-definition <command>` and `--export-tool-definitions <dir>`.

use std::{collections::BTreeSet, fs, path::Path};

use crate::workdir::Workdir;

/// The committed skill JSONs that build.rs embeds.
fn repo_skills_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(".claude/skills/qsv")
}

/// Keys of the files in `dir` named `prefix<key>suffix`.
fn file_keys(dir: &Path, prefix: &str, suffix: &str) -> BTreeSet<String> {
    fs::read_dir(dir)
        .unwrap()
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            Some(name.strip_prefix(prefix)?.strip_suffix(suffix)?.to_string())
        })
        .collect()
}

/// The commands compiled into the binary under test, from its `--list` output. CI tests
/// partial feature sets (e.g. `polars,feature_capable,readstat` has no viz/geocode), so the
/// expected export has to come from the binary, not from the committed files alone.
fn installed_commands(wrk: &Workdir) -> BTreeSet<String> {
    let mut cmd = wrk.command("--list");
    let list: String = wrk.stdout_on_success(&mut cmd);
    list.lines()
        .filter(|line| line.starts_with("    "))
        .filter_map(|line| line.split_whitespace().next())
        .map(str::to_string)
        .collect()
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
    let installed = installed_commands(&wrk);
    assert!(installed.contains("stats"));

    let mut cmd = wrk.command("--export-tool-definitions");
    cmd.arg("defs");
    wrk.assert_success(&mut cmd);

    let out = wrk.path("defs");
    let skills = out.join("mcp_skills");
    let help = out.join("help");

    // exactly the committed skills whose command is compiled into this binary
    let committed_skills = file_keys(&repo_skills_dir(), "qsv-", ".json");
    let expected_skills: BTreeSet<String> =
        committed_skills.intersection(&installed).cloned().collect();
    assert!(!expected_skills.is_empty());
    assert_eq!(file_keys(&skills, "qsv-", ".json"), expected_skills);
    assert_eq!(
        fs::read(skills.join("qsv-stats.json")).unwrap(),
        fs::read(repo_skills_dir().join("qsv-stats.json")).unwrap()
    );

    // likewise for help, plus the table of contents
    let committed_help = file_keys(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/help"),
        "",
        ".md",
    );
    let mut expected_help: BTreeSet<String> =
        committed_help.intersection(&installed).cloned().collect();
    expected_help.insert("TableOfContents".to_string());
    assert_eq!(file_keys(&help, "", ".md"), expected_help);

    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(out.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["qsv_version"], env!("CARGO_PKG_VERSION"));
    let expected_bin = if cfg!(feature = "qsvmcp") {
        "qsvmcp"
    } else {
        "qsv"
    };
    assert_eq!(manifest["binary"], expected_bin);
    let names = |key: &str| -> BTreeSet<String> {
        manifest[key]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect()
    };
    assert_eq!(names("commands"), installed);
    assert_eq!(names("mcp_skills"), expected_skills);
    assert_eq!(names("help"), expected_help);
}

#[test]
fn export_tool_definitions_removes_stale_files() {
    // a reused export dir must end up matching manifest.json, not keep definitions for
    // commands this binary doesn't have (e.g. exported earlier by a different build)
    let wrk = Workdir::new("export_tool_definitions_removes_stale_files");
    wrk.create_subdir("defs").unwrap();
    wrk.create_subdir("defs/mcp_skills").unwrap();
    wrk.create_subdir("defs/help").unwrap();
    wrk.create_from_string("defs/mcp_skills/qsv-notacommand.json", "{}");
    wrk.create_from_string("defs/help/notacommand.md", "stale");
    // files outside the export's naming pattern are left alone
    wrk.create_from_string("defs/mcp_skills/notes.txt", "keep");
    wrk.create_from_string("defs/help/notes.txt", "keep");

    let mut cmd = wrk.command("--export-tool-definitions");
    cmd.arg("defs");
    wrk.assert_success(&mut cmd);

    let out = wrk.path("defs");
    assert!(!out.join("mcp_skills/qsv-notacommand.json").exists());
    assert!(!out.join("help/notacommand.md").exists());
    assert!(out.join("mcp_skills/notes.txt").is_file());
    assert!(out.join("help/notes.txt").is_file());
    assert!(out.join("mcp_skills/qsv-stats.json").is_file());
    assert!(out.join("help/stats.md").is_file());
}
