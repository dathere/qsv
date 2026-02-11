// Auto-generate clap::Command tree from qsv command USAGE text.
//
// This module reads the `static USAGE` string from each `src/cmd/*.rs` file in
// the qsv repository and builds a complete clap Command tree suitable for shell
// completion generation. It uses the qsv_docopt parser for robust option type
// detection and subcommand discovery, combined with direct text parsing for
// short-flag extraction (since docopt's SynonymMap doesn't expose short-flag
// synonyms through iteration).

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use clap::{Arg, ArgAction, Command};
use qsv_docopt::parse::{Argument as DocoptArgument, Atom, Parser};

/// Safety limit for directory traversal depth when searching for the repo root.
/// Well beyond any realistic nesting depth.
const MAX_DIR_TRAVERSAL_DEPTH: usize = 100;

/// Files to skip when scanning `src/cmd/` (not user-facing commands).
const SKIP_FILES: &[&str] = &[
    "mod",            // module declarations, not a command
    "python.pyo3-23", // internal PyO3 variant, not a separate command
    "applydp",        // DataPusher+-specific variant, excluded from completions
];

/// Walk up from CWD to find the qsv repository root (contains `Cargo.toml` + `src/cmd/`).
pub fn find_repo_root() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    for _ in 0..MAX_DIR_TRAVERSAL_DEPTH {
        if dir.join("Cargo.toml").exists() && dir.join("src/cmd").is_dir() {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
    None
}

/// Extract the `static USAGE` string from a qsv command source file.
/// Handles both `r#"..."#` and `r##"..."##` raw string delimiters.
fn extract_usage_from_file(file_path: &Path) -> Result<String, String> {
    let content =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {e}"))?;

    // Find the first `static USAGE` declaration in the file.
    // Using `find()` returns the first occurrence, which is always the actual
    // USAGE constant (not a comment or string literal containing the pattern).
    let (usage_start, skip_len, end_delimiter) =
        if let Some(pos) = content.find("static USAGE: &str = r##\"") {
            (pos, 26, "\"##;")
        } else if let Some(pos) = content.find("static USAGE: &str = r#\"") {
            (pos, 24, "\"#;")
        } else {
            return Err("USAGE constant not found".to_string());
        };

    let after_start = &content[usage_start + skip_len..];

    let usage_end = after_start
        .find(end_delimiter)
        .ok_or("End of USAGE constant not found")?;

    Ok(after_start[..usage_end].to_string())
}

/// Extract the actual CLI command name from the Usage: section of the USAGE text.
///
/// Most commands match their filename (e.g., `count.rs` → `count`), but some
/// have aliases:
///   - `enumerate.rs` → invoked as `qsv enum`
///   - `python.rs` → invoked as `qsv py`
fn extract_command_name<'a>(file_stem: &'a str, usage_text: &str) -> &'a str {
    if usage_text.contains("qsv enum ") {
        return "enum";
    }
    if usage_text.contains("qsv py ") {
        return "py";
    }
    file_stem
}

/// Check whether a long flag name is valid for clap.
/// Filters out decorative lines of dashes or other non-flag strings
/// that docopt may misparse as long flags.
fn is_valid_long_flag(name: &str) -> bool {
    !name.is_empty() && name.chars().any(|c| c.is_alphanumeric())
}

/// Extract short-to-long flag mappings from USAGE text by parsing lines like:
///   `-d, --delimiter <arg>  Description`
///   `-H, --human-readable   Description`
///
/// Returns a map of long-flag-name → short-char (e.g., "delimiter" → 'd').
fn extract_short_flags(usage_text: &str) -> HashMap<String, char> {
    let mut map = HashMap::new();

    for line in usage_text.lines() {
        let trimmed = line.trim();

        // Match lines starting with a short flag: -X, --long-name
        if trimmed.len() >= 2
            && trimmed.starts_with('-')
            && trimmed.as_bytes()[1] != b'-'
            && trimmed.as_bytes()[1].is_ascii_alphanumeric()
        {
            let short_char = trimmed.as_bytes()[1] as char;

            // Look for the corresponding --long-name
            if let Some(rest) = trimmed.get(2..) {
                // Skip optional comma and whitespace: "-d, --delimiter" or "-d --delimiter"
                let rest = rest.trim_start_matches(',').trim_start();
                if let Some(long_part) = rest.strip_prefix("--") {
                    // Extract the long name (up to whitespace, '=' or '<')
                    let long_name: String = long_part
                        .chars()
                        .take_while(|c| *c != ' ' && *c != '=' && *c != '<' && *c != '\t')
                        .collect();
                    if !long_name.is_empty() && is_valid_long_flag(&long_name) {
                        map.insert(long_name, short_char);
                    }
                }
            }
        }
    }

    map
}

/// Build a single `clap::Command` from parsed USAGE text.
fn build_command_from_usage(command_name: &str, usage_text: &str) -> Result<Command, String> {
    let parser = Parser::new(usage_text)
        .map_err(|e| format!("Docopt parsing failed for {command_name}: {e}"))?;

    // Extract short flag mappings from the raw USAGE text, since docopt's
    // SynonymMap.iter() only returns canonical (long) entries.
    let short_flags = extract_short_flags(usage_text);

    let mut args: Vec<Arg> = Vec::new();
    let mut subcommands: Vec<String> = Vec::new();
    let mut used_shorts: Vec<char> = Vec::new();

    for (atom, opts) in parser.descs.iter() {
        match atom {
            Atom::Short(_) => {
                // Short flags are handled via the short_flags map when
                // processing the corresponding Atom::Long entry.
                // SynonymMap.iter() typically doesn't yield Short atoms,
                // but if it does, we ignore them here to avoid duplicates.
            }
            Atom::Long(name) => {
                // Skip --help (clap adds it automatically) and invalid names
                if name == "help" || !is_valid_long_flag(name) {
                    continue;
                }

                // Skip if already processed
                if args.iter().any(|a| a.get_id().as_str() == name.as_str()) {
                    continue;
                }

                let mut arg = Arg::new(name.clone()).long(name.clone());

                // Add short flag if one was found in the USAGE text,
                // but only if the short char hasn't already been used
                // (some commands reuse short flags across different options).
                if let Some(&short) = short_flags.get(name.as_str()) {
                    if !used_shorts.contains(&short) {
                        arg = arg.short(short);
                        used_shorts.push(short);
                    }
                }

                arg = match &opts.arg {
                    DocoptArgument::Zero => arg.action(ArgAction::SetTrue),
                    DocoptArgument::One(_) => arg.action(ArgAction::Set),
                };

                args.push(arg);
            }
            Atom::Positional(_) => {
                // Shell completions don't define positional args --
                // they default to file-path completion in most shells.
            }
            Atom::Command(cmd_name) => {
                // Collect subcommand names; skip the main command itself and "--"
                if cmd_name != command_name && cmd_name != "--" {
                    subcommands.push(cmd_name.clone());
                }
            }
        }
    }

    let mut cmd = Command::new(command_name.to_string());

    // If the command has subcommands, create child Commands.
    // Each subcommand gets a copy of all the args (slightly more permissive
    // than strictly correct, but acceptable for shell completions and matches
    // the old manually-written behaviour for most commands).
    if !subcommands.is_empty() {
        subcommands.sort_unstable();
        subcommands.dedup();
        for sub_name in &subcommands {
            let sub_cmd = Command::new(sub_name.clone()).args(args.iter().cloned());
            cmd = cmd.subcommand(sub_cmd);
        }
    }

    cmd = cmd.args(args);
    Ok(cmd)
}

/// Build the complete `qsv` CLI `Command` tree by scanning source files.
///
/// Reads every `src/cmd/*.rs` file in the repository, extracts the USAGE text,
/// and builds a clap `Command` for each. Returns the root `Command` with all
/// subcommands and global flags.
pub fn build_cli(repo_root: &Path) -> Command {
    let cmd_dir = repo_root.join("src/cmd");

    let mut entries: Vec<_> = fs::read_dir(&cmd_dir)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", cmd_dir.display()))
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name_str = name.to_string_lossy();
            name_str.ends_with(".rs") && {
                let stem = name_str.trim_end_matches(".rs");
                !SKIP_FILES.contains(&stem)
            }
        })
        .collect();

    entries.sort_by_key(|e| e.file_name());

    let mut commands: Vec<Command> = Vec::new();

    for entry in entries {
        let path = entry.path();
        let file_stem = match path.file_stem() {
            Some(s) => s.to_string_lossy().into_owned(),
            None => {
                eprintln!("Warning: skipping {}: no file stem", path.display());
                continue;
            }
        };

        // Extract USAGE text
        let usage_text = match extract_usage_from_file(&path) {
            Ok(text) => text,
            Err(e) => {
                eprintln!("Warning: skipping {file_stem}: {e}");
                continue;
            }
        };

        // Determine the actual CLI command name
        let command_name = extract_command_name(&file_stem, &usage_text);

        // Build the Command
        match build_command_from_usage(command_name, &usage_text) {
            Ok(cmd) => commands.push(cmd),
            Err(e) => {
                eprintln!("Warning: skipping {command_name}: {e}");
            }
        }
    }

    eprintln!(
        "Auto-generated completions for {} commands from {}",
        commands.len(),
        cmd_dir.display()
    );

    // Root command with global flags
    Command::new("qsv")
        .args([
            Arg::new("list").long("list").action(ArgAction::SetTrue),
            Arg::new("envlist")
                .long("envlist")
                .action(ArgAction::SetTrue),
            Arg::new("update")
                .long("update")
                .action(ArgAction::SetTrue),
            Arg::new("updatenow")
                .long("updatenow")
                .action(ArgAction::SetTrue),
            Arg::new("version")
                .long("version")
                .short('V')
                .action(ArgAction::SetTrue),
        ])
        .subcommands(commands)
}
