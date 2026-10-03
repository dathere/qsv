// `<command> --help --format json|md` and `--export-tool-definitions` (issue #4638) generate
// tool definitions and help Markdown at runtime from the binary's own USAGE text. The core
// guarantee: they are byte-identical to what the in-repo generators commit to
// `.claude/skills/qsv/` and `docs/help/`, which the wiki-lint CI job keeps current.
//
// `TableOfContents.md` is deliberately NOT compared: no build has every command (`py` is not in
// `all_features`), so an exported table of contents is always shorter than the committed one.

use std::{fs, path::PathBuf};

use crate::workdir::Workdir;

fn repo_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// Export into a fresh workdir; returns the workdir and the parsed manifest.
fn export(name: &str) -> (Workdir, serde_json::Value) {
    let wrk = Workdir::new(name);
    let mut cmd = wrk.command("");
    cmd.args(["--export-tool-definitions", "defs"]);
    wrk.assert_success(&mut cmd);
    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(wrk.path("defs/manifest.json")).unwrap()).unwrap();
    (wrk, manifest)
}

fn names(manifest: &serde_json::Value, key: &str) -> Vec<String> {
    manifest[key]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect()
}

#[test]
fn help_format_json_is_the_committed_skill() {
    let wrk = Workdir::new("tool_defs_json_stats");
    let mut cmd = wrk.command("stats");
    cmd.args(["--help", "--format", "json"]);
    let got: String = wrk.stdout(&mut cmd);
    let want = fs::read_to_string(repo_path(".claude/skills/qsv/qsv-stats.json")).unwrap();
    // Workdir::stdout trims leading/trailing newlines
    assert_eq!(got, want.trim_matches(&['\r', '\n'][..]));
}

#[test]
fn help_format_md_is_the_committed_help() {
    let wrk = Workdir::new("tool_defs_md_enum");
    let mut cmd = wrk.command("enum");
    cmd.args(["-h", "--format=md"]);
    let got: String = wrk.stdout(&mut cmd);
    let want = fs::read_to_string(repo_path("docs/help/enum.md")).unwrap();
    assert_eq!(got, want.trim_matches(&['\r', '\n'][..]));
}

#[test]
fn help_format_rejects_unknown_formats() {
    let wrk = Workdir::new("tool_defs_bad_format");
    let mut cmd = wrk.command("stats");
    cmd.args(["--help", "--format", "yaml"]);
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("Valid formats: json, md"), "{stderr}");
}

/// After `--`, `--help --format=json` are operands: a pattern and a file literally named
/// `--format=json`, not a request for a tool definition.
#[test]
fn help_format_ignores_operands_after_double_dash() {
    let wrk = Workdir::new("tool_defs_double_dash");
    fs::write(wrk.path("--format=json"), "h\n--help\nx\n").unwrap();
    let mut cmd = wrk.command("search");
    // the input has no file extension
    cmd.env("QSV_SKIP_FORMAT_CHECK", "1")
        .args(["--", "--help", "--format=json"]);
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "h\n--help");
}

#[test]
fn plain_help_is_unchanged() {
    let wrk = Workdir::new("tool_defs_plain_help");
    let mut cmd = wrk.command("stats");
    cmd.arg("--help");
    let got: String = wrk.stdout(&mut cmd);
    assert!(got.contains("Usage:"), "{got}");
    assert!(!got.trim_start().starts_with('{'), "{got}");
}

#[test]
fn exported_definitions_match_the_committed_files() {
    let (wrk, manifest) = export("tool_defs_export_identity");

    let mut json_checked = 0;
    let mut md_checked = 0;
    for name in names(&manifest, "commands") {
        let skill = repo_path(&format!(".claude/skills/qsv/qsv-{name}.json"));
        if skill.exists() {
            let got =
                fs::read_to_string(wrk.path(&format!("defs/tool-definitions/qsv-{name}.json")))
                    .unwrap();
            assert!(
                got == fs::read_to_string(&skill).unwrap(),
                "exported qsv-{name}.json differs from {}",
                skill.display()
            );
            json_checked += 1;
        }
        let help = repo_path(&format!("docs/help/{name}.md"));
        if help.exists() {
            let got = fs::read_to_string(wrk.path(&format!("defs/help/{name}.md"))).unwrap();
            assert!(
                got == fs::read_to_string(&help).unwrap(),
                "exported help/{name}.md differs from {}",
                help.display()
            );
            md_checked += 1;
        }
    }
    // guard against a vacuous pass (e.g. an empty manifest)
    assert!(
        json_checked >= 40,
        "only {json_checked} JSON definitions compared"
    );
    assert!(md_checked >= 50, "only {md_checked} help files compared");
}

#[test]
fn export_layout_and_manifest() {
    let (wrk, manifest) = export("tool_defs_export_manifest");

    assert_eq!(manifest["qsv_version"], env!("CARGO_PKG_VERSION"));
    let bin = wrk.qsv_bin();
    let bin_stem = bin.file_stem().unwrap().to_str().unwrap();
    assert_eq!(manifest["binary"], bin_stem);

    let commands = names(&manifest, "commands");
    let mcp_skills = names(&manifest, "mcp_skills");
    // `enum` is the invocation of src/cmd/enumerate.rs, which the MCP set lists by source name
    assert!(commands.contains(&"enum".to_string()));
    assert!(mcp_skills.contains(&"enum".to_string()));
    assert!(!commands.contains(&"help".to_string()));
    for skill in &mcp_skills {
        assert!(
            commands.contains(skill),
            "MCP skill {skill} is not an installed command"
        );
    }

    assert!(wrk.path("defs/help/TableOfContents.md").exists());
    for name in &commands {
        let json = wrk.path(&format!("defs/tool-definitions/qsv-{name}.json"));
        let def: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&json).unwrap()).unwrap();
        assert_eq!(def["name"], format!("qsv-{name}"));
        assert!(
            wrk.path(&format!("defs/help/{name}.md")).exists(),
            "no help for {name}"
        );
    }
    let exported = fs::read_dir(wrk.path("defs/tool-definitions"))
        .unwrap()
        .count();
    assert_eq!(exported, commands.len());
}

/// Every installed command's definition - not just the curated MCP set, whose committed
/// files `test_mcp_skills` checks - documents every argument and option.
#[test]
fn every_exported_definition_is_described() {
    let (wrk, manifest) = export("tool_defs_export_described");
    let mut missing = Vec::new();
    for name in names(&manifest, "commands") {
        let path = wrk.path(&format!("defs/tool-definitions/qsv-{name}.json"));
        let def: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        for (kind, key) in [("args", "name"), ("options", "flag")] {
            for item in def["command"][kind].as_array().into_iter().flatten() {
                if item["description"]
                    .as_str()
                    .unwrap_or_default()
                    .trim()
                    .is_empty()
                {
                    missing.push(format!("{name} {}", item[key]));
                }
            }
        }
    }
    assert!(missing.is_empty(), "undescribed: {missing:?}");
}
