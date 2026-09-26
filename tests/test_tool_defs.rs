//! Tests for the tool definitions embedded in the qsv/qsvmcp binaries (issue #4638):
//! `--tool-definition <command>` and `--export-tool-definitions <dir>`.

use std::{collections::BTreeSet, fs, path::Path};

use crate::workdir::Workdir;

/// The committed tool definitions (one per command) that build.rs embeds.
fn repo_defs_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/tool-definitions")
}

/// The curated MCP skill set, a subset of the tool definitions.
fn repo_skills_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(".claude/skills/qsv")
}

fn repo_help_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/help")
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
    let expected = fs::read(repo_defs_dir().join("qsv-stats.json")).unwrap();
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
fn tool_definition_of_a_non_mcp_command() {
    // behead is deliberately outside the curated MCP skill set, but every command has a
    // tool definition
    let wrk = Workdir::new("tool_definition_of_a_non_mcp_command");
    assert!(!repo_skills_dir().join("qsv-behead.json").exists());
    let mut cmd = wrk.command("--tool-definition");
    cmd.arg("behead");

    let json: String = wrk.stdout_on_success(&mut cmd);
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(json["name"], "qsv-behead");
}

#[test]
fn tool_definition_installed_command_without_definition() {
    // `help` is in `qsv --list` but isn't a README command, so it has no definition
    let wrk = Workdir::new("tool_definition_installed_command_without_definition");
    let mut cmd = wrk.command("--tool-definition");
    cmd.arg("help");

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
    let defs = out.join("tool_definitions");
    let help = out.join("help");

    // exactly the committed definitions whose command is compiled into this binary
    let committed_defs = file_keys(&repo_defs_dir(), "qsv-", ".json");
    let expected_defs: BTreeSet<String> =
        committed_defs.intersection(&installed).cloned().collect();
    assert!(expected_defs.contains("behead"));
    assert_eq!(file_keys(&defs, "qsv-", ".json"), expected_defs);
    assert_eq!(
        fs::read(defs.join("qsv-stats.json")).unwrap(),
        fs::read(repo_defs_dir().join("qsv-stats.json")).unwrap()
    );

    // the manifest marks which of them are in the curated MCP skill set
    let committed_skills = file_keys(&repo_skills_dir(), "qsv-", ".json");
    let expected_skills: BTreeSet<String> =
        committed_skills.intersection(&installed).cloned().collect();
    assert!(!expected_skills.is_empty());

    // likewise for help, plus the table of contents
    let committed_help = file_keys(&repo_help_dir(), "", ".md");
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
    assert_eq!(names("tool_definitions"), expected_defs);
    assert_eq!(names("mcp_skills"), expected_skills);
    assert_eq!(names("help"), expected_help);
}

#[test]
fn export_tool_definitions_removes_stale_files() {
    // a reused export dir must end up matching manifest.json, not keep definitions for
    // commands this binary doesn't have (e.g. exported earlier by a different build)
    let wrk = Workdir::new("export_tool_definitions_removes_stale_files");
    wrk.create_subdir("defs").unwrap();
    wrk.create_subdir("defs/tool_definitions").unwrap();
    wrk.create_subdir("defs/help").unwrap();
    wrk.create_from_string("defs/tool_definitions/qsv-notacommand.json", "{}");
    wrk.create_from_string("defs/help/notacommand.md", "stale");
    // files outside the export's naming pattern are left alone
    wrk.create_from_string("defs/tool_definitions/notes.txt", "keep");
    wrk.create_from_string("defs/help/notes.txt", "keep");

    let mut cmd = wrk.command("--export-tool-definitions");
    cmd.arg("defs");
    wrk.assert_success(&mut cmd);

    let out = wrk.path("defs");
    assert!(!out.join("tool_definitions/qsv-notacommand.json").exists());
    assert!(!out.join("help/notacommand.md").exists());
    assert!(out.join("tool_definitions/notes.txt").is_file());
    assert!(out.join("help/notes.txt").is_file());
    assert!(out.join("tool_definitions/qsv-stats.json").is_file());
    assert!(out.join("help/stats.md").is_file());
}

// The tests below check the committed generator output itself, so they run the same whether
// or not a given command is compiled into the binary under test.

#[test]
fn every_help_command_has_a_tool_definition() {
    let mut help = file_keys(&repo_help_dir(), "", ".md");
    help.remove("TableOfContents");
    assert_eq!(file_keys(&repo_defs_dir(), "qsv-", ".json"), help);
}

#[test]
fn mcp_skills_are_copies_of_their_tool_definitions() {
    // both directories come from one `--update-mcp-skills` run with the same parser
    let skills = file_keys(&repo_skills_dir(), "qsv-", ".json");
    assert!(!skills.is_empty());
    for cmd in &skills {
        let name = format!("qsv-{cmd}.json");
        assert_eq!(
            fs::read(repo_skills_dir().join(&name)).unwrap(),
            fs::read(repo_defs_dir().join(&name)).unwrap(),
            "{name} differs between .claude/skills/qsv and docs/tool-definitions"
        );
    }
}

#[test]
fn tool_definitions_are_keyed_by_invocation_name() {
    // `py` lives in python.rs and `enum` in enumerate.rs; a definition keyed by the source file
    // would never match `qsv --list` and silently drop out of every export
    for (invocation, source) in [("py", "python"), ("enum", "enumerate")] {
        assert!(!repo_defs_dir().join(format!("qsv-{source}.json")).exists());
        let json: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(repo_defs_dir().join(format!("qsv-{invocation}.json"))).unwrap(),
        )
        .unwrap();
        assert_eq!(json["name"], format!("qsv-{invocation}"));
        assert_eq!(json["command"]["subcommand"], invocation);
    }
}
