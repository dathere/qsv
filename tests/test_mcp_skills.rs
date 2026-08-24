// Guards the invariant that `qsv --update-mcp-skills` (src/mcp_skills_gen.rs)
// emits a real description for every flag and argument. Empty descriptions
// leave MCP agents with a bare name and no idea what it does — see issue #4488.

use std::{fs, path::PathBuf};

fn skills_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".claude/skills/qsv")
}

fn skill_files() -> Vec<PathBuf> {
    let dir = skills_dir();
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.extension().is_some_and(|e| e == "json")
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("qsv-"))
        })
        .collect();
    files.sort_unstable();
    assert!(
        !files.is_empty(),
        "no skill JSONs found in {}",
        dir.display()
    );
    files
}

fn parse(path: &PathBuf) -> serde_json::Value {
    let text = fs::read_to_string(path).unwrap();
    serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("{} is not valid JSON: {e}", path.display()))
}

/// A well-formed flag is one or two dashes, then an alphabetic character,
/// then alphanumerics/dashes/underscores. Anything else means a chunk of
/// USAGE prose (an ASCII separator rule, say) leaked into the option list.
fn is_well_formed_flag(flag: &str) -> bool {
    let Some(name) = flag.strip_prefix("--").or_else(|| flag.strip_prefix('-')) else {
        return false;
    };
    let mut chars = name.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

#[test]
fn every_skill_option_has_a_description() {
    let mut empty = Vec::new();
    for path in skill_files() {
        let json = parse(&path);
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        for opt in json["command"]["options"].as_array().into_iter().flatten() {
            if opt["description"]
                .as_str()
                .unwrap_or_default()
                .trim()
                .is_empty()
            {
                empty.push(format!("{name}: {}", opt["flag"].as_str().unwrap_or("?")));
            }
        }
    }
    assert!(
        empty.is_empty(),
        "skill JSONs with empty option descriptions (regenerate with `qsv \
         --update-mcp-skills`):\n  {}",
        empty.join("\n  ")
    );
}

#[test]
fn every_skill_argument_has_a_description() {
    let mut empty = Vec::new();
    for path in skill_files() {
        let json = parse(&path);
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        for arg in json["command"]["args"].as_array().into_iter().flatten() {
            if arg["description"]
                .as_str()
                .unwrap_or_default()
                .trim()
                .is_empty()
            {
                empty.push(format!("{name}: <{}>", arg["name"].as_str().unwrap_or("?")));
            }
        }
    }
    assert!(
        empty.is_empty(),
        "skill JSONs with empty argument descriptions (add the name to \
         `generic_positional_description` in src/mcp_skills_gen.rs):\n  {}",
        empty.join("\n  ")
    );
}

#[test]
fn every_skill_flag_is_well_formed() {
    let mut bad = Vec::new();
    for path in skill_files() {
        let json = parse(&path);
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        for opt in json["command"]["options"].as_array().into_iter().flatten() {
            let flag = opt["flag"].as_str().unwrap_or_default();
            if !is_well_formed_flag(flag) {
                bad.push(format!("{name}: {flag:?}"));
            }
            if let Some(short) = opt["short"].as_str()
                && !is_well_formed_flag(short)
            {
                bad.push(format!("{name}: short {short:?}"));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "skill JSONs with malformed flags:\n  {}",
        bad.join("\n  ")
    );
}
