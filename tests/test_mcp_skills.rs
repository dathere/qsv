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

/// #4596 - an option's type comes from its declared docopt placeholder, never
/// from prose in its description. Pinned here on the SHIPPED JSONs, in both
/// directions, so neither half of the regression can return silently.
#[test]
fn option_types_follow_placeholders_not_prose() {
    // (skill, flag, expected type)
    const PINNED: &[(&str, &str, &str)] = &[
        // a numeric placeholder types `number` - none of these descriptions
        // says `<number>`, so the old prose heuristic would have said `string`
        ("qsv-sample.json", "--seed", "number"),
        ("qsv-viz.json", "--bins", "number"),
        ("qsv-moarstats.json", "--epsilon", "number"),
        // `<N>`, uppercase
        ("qsv-pragmastat.json", "--subsample", "number"),
        // a non-numeric placeholder stays a `string` even though its
        // description DOES say `<number>` - `Format: <number><unit>`
        ("qsv-sample.json", "--ts-interval", "string"),
    ];

    let find = |skill: &str, flag: &str| -> serde_json::Value {
        let json = parse(&skills_dir().join(skill));
        json["command"]["options"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|o| o["flag"].as_str() == Some(flag))
            .unwrap_or_else(|| panic!("{skill} has no {flag}"))
            .clone()
    };

    let mut wrong = Vec::new();
    let mut vacuous = Vec::new();
    for &(skill, flag, expected) in PINNED {
        let opt = find(skill, flag);
        let actual = opt["type"].as_str().unwrap_or("?");
        if actual != expected {
            wrong.push(format!(
                "{skill}: {flag} is {actual:?}, expected {expected:?}"
            ));
        }

        // Every pin must DISAGREE with the heuristic it exists to keep out:
        // the pre-#4596 generator typed an option `number` iff its description
        // mentioned `<number>` or `<int>`. A pin the old heuristic would have
        // got right proves nothing, and a reword can quietly turn a good pin
        // into one - so the premise is asserted rather than assumed.
        let desc = opt["description"].as_str().unwrap_or_default();
        let prose_heuristic_says = if desc.contains("<number>") || desc.contains("<int>") {
            "number"
        } else {
            "string"
        };
        if prose_heuristic_says == expected {
            vacuous.push(format!("{skill}: {flag} (description: {desc:?})"));
        }
    }

    assert!(
        wrong.is_empty(),
        "option types no longer follow their placeholders (see #4596):\n  {}",
        wrong.join("\n  ")
    );
    assert!(
        vacuous.is_empty(),
        "these pins no longer discriminate against the pre-#4596 prose heuristic, so they no \
         longer test anything - reword the description back, or pin a different option:\n  {}",
        vacuous.join("\n  ")
    );
}
