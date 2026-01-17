#![allow(clippy::needless_continue, clippy::ref_as_ptr, clippy::unused_self)]
// qsv MCP Skills Generator - Generate Agent Skills from qsv command USAGE text
//
// This module parses USAGE text from qsv commands and generates Agent Skill
// definitions in JSON format for use with Claude Desktop (MCP) and the Claude Agent SDK.
//
// Uses qsv-docopt Parser for robust USAGE text parsing.

use std::{fs, path::Path};

use qsv_docopt::parse::{Argument as DocoptArgument, Atom, Parser};
use serde::{Deserialize, Serialize};

use crate::{CliResult, regex_oncelock};

const MAX_ITERATIONS: usize = 100; // Prevent infinite loops

#[derive(Debug, Serialize, Deserialize)]
struct SkillDefinition {
    name:        String,
    version:     String,
    /// Concise description from README.md command table
    /// For detailed help, use `qsv <command> --help` via the qsv_help tool
    description: String,
    category:    String,
    command:     CommandSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    hints:       Option<BehavioralHints>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommandSpec {
    binary:     String,
    subcommand: String,
    args:       Vec<Argument>,
    options:    Vec<Option_>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Argument {
    name:        String,
    #[serde(rename = "type")]
    arg_type:    String,
    required:    bool,
    description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    examples:    Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#enum:      Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Option_ {
    flag:        String,
    #[serde(skip_serializing_if = "Option::is_none")]
    short:       Option<String>,
    #[serde(rename = "type")]
    option_type: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    default:     Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BehavioralHints {
    streamable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    indexed:    Option<bool>,
    memory:     String,
}

struct UsageParser {
    usage_text:   String,
    command_name: String,
}

impl UsageParser {
    const fn new(usage_text: String, command_name: String) -> Self {
        Self {
            usage_text,
            command_name,
        }
    }

    fn parse(&self) -> Result<SkillDefinition, String> {
        // Extract concise description from README.md command table
        // Falls back to first sentence of USAGE text if not found in README
        let description = Self::extract_short_description_from_readme(&self.command_name)
            .unwrap_or_else(|| {
                // Fallback: extract first sentence from USAGE text
                self.extract_description().map_or_else(
                    |_| format!("{} command", self.command_name),
                    |d| d.split('.').next().unwrap_or(&d).trim().to_string() + ".",
                )
            });

        // Use qsv-docopt Parser to parse USAGE text
        let (args, options) = self.parse_with_docopt()?;

        let hints = self.extract_hints();
        let category = self.infer_category();

        Ok(SkillDefinition {
            name: format!("qsv-{}", self.command_name),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description,
            category,
            command: CommandSpec {
                binary: "qsv".to_string(),
                subcommand: self.command_name.clone(),
                args,
                options,
            },
            hints,
        })
    }

    /// Extract positional argument names in order from USAGE line
    fn extract_arg_order_from_usage(&self) -> Vec<String> {
        let mut arg_order = Vec::new();

        // Find the main usage line (not --help line)
        if let Some(usage_line) = self
            .usage_text
            .lines()
            .skip_while(|l| !l.contains("Usage:"))
            .skip(1) // Skip "Usage:" line
            .find(|l| !l.trim().ends_with("--help") && l.contains("qsv"))
        {
            // Extract all <arg> and [<arg>] patterns in order
            let re = regex::Regex::new(r"(?:\[)?<([^>]+)>(?:\])?").unwrap();
            for cap in re.captures_iter(usage_line) {
                if let Some(arg_name) = cap.get(1) {
                    arg_order.push(arg_name.as_str().to_string());
                }
            }
        }

        arg_order
    }

    /// Parse USAGE text using qsv-docopt Parser for robust parsing
    fn parse_with_docopt(&self) -> Result<(Vec<Argument>, Vec<Option_>), String> {
        // Parse USAGE text with docopt
        let parser =
            Parser::new(&self.usage_text).map_err(|e| format!("Docopt parsing failed: {e}"))?;

        let mut args_map = std::collections::HashMap::new();
        let mut options = Vec::new();
        let mut subcommands = Vec::new();

        // Also parse manually to get descriptions
        let manual_descriptions = self.extract_descriptions_from_text();

        // Iterate over parsed atoms from docopt
        for (atom, opts) in parser.descs.iter() {
            match atom {
                Atom::Short(c) => {
                    // Short flag like -d
                    let flag_str = format!("-{c}");

                    // Look for corresponding long flag
                    let long_flag = parser
                        .descs
                        .iter()
                        .find(|(a, o)| {
                            // Check if this long flag is a synonym of the short flag
                            // by comparing pointer equality or field values
                            matches!(a, Atom::Long(_))
                                && std::ptr::eq(*o as *const _, opts as *const _)
                        })
                        .and_then(|(a, _)| {
                            if let Atom::Long(s) = a {
                                Some(format!("--{s}"))
                            } else {
                                None
                            }
                        });

                    // Use long flag as primary, or short if no long flag
                    let primary_flag = long_flag.clone().unwrap_or_else(|| flag_str.clone());

                    // Skip if we already added this as a long flag
                    if options.iter().any(|o: &Option_| o.flag == primary_flag) {
                        continue;
                    }

                    // Skip options not relevant for AI agents using MCP
                    // --quiet: suppresses stderr output (not useful for agents)
                    // --help: universally available for all commands (handled by MCP server)
                    if primary_flag == "--quiet" || primary_flag == "--help" {
                        continue;
                    }
                    if (flag_str == "-q" && long_flag.as_deref() == Some("--quiet"))
                        || (flag_str == "-h" && long_flag.as_deref() == Some("--help"))
                    {
                        continue;
                    }

                    let option_type = match &opts.arg {
                        DocoptArgument::Zero => "flag",
                        DocoptArgument::One(_) => {
                            // Check if it's a number type
                            let desc = manual_descriptions
                                .get(&primary_flag)
                                .or_else(|| manual_descriptions.get(&flag_str))
                                .map_or("", std::string::String::as_str);
                            if desc.contains("<number>") || desc.contains("<int>") {
                                "number"
                            } else {
                                "string"
                            }
                        },
                    };

                    let mut description = manual_descriptions
                        .get(&primary_flag)
                        .or_else(|| manual_descriptions.get(&flag_str))
                        .cloned()
                        .unwrap_or_default();

                    let default = match &opts.arg {
                        DocoptArgument::One(Some(d)) => Some(d.clone()),
                        _ => self.extract_default_value(&description),
                    };

                    // Strip redundant [default: ...] from description if we have a default value
                    if default.is_some() {
                        description = Self::strip_default_from_description(&description);
                    }

                    options.push(Option_ {
                        flag: primary_flag,
                        short: long_flag.and(Some(flag_str)),
                        option_type: option_type.to_string(),
                        description,
                        default,
                    });
                },
                Atom::Long(name) => {
                    let flag_str = format!("--{name}");

                    // Skip if already processed
                    if options.iter().any(|o| o.flag == flag_str) {
                        continue;
                    }

                    // Skip options not relevant for AI agents using MCP
                    // --quiet: suppresses stderr output (not useful for agents)
                    // --help: universally available for all commands (handled by MCP server)
                    if flag_str == "--quiet" || flag_str == "--help" {
                        continue;
                    }

                    // Find corresponding short flag if any
                    let short_flag = parser
                        .descs
                        .iter()
                        .find(|(a, o)| {
                            matches!(a, Atom::Short(_))
                                && std::ptr::eq(*o as *const _, opts as *const _)
                        })
                        .and_then(|(a, _)| {
                            if let Atom::Short(c) = a {
                                Some(format!("-{c}"))
                            } else {
                                None
                            }
                        });

                    let option_type = match &opts.arg {
                        DocoptArgument::Zero => "flag",
                        DocoptArgument::One(_) => {
                            let desc = manual_descriptions
                                .get(&flag_str)
                                .map_or("", std::string::String::as_str);
                            if desc.contains("<number>") || desc.contains("<int>") {
                                "number"
                            } else {
                                "string"
                            }
                        },
                    };

                    let mut description = manual_descriptions
                        .get(&flag_str)
                        .cloned()
                        .unwrap_or_default();

                    let default = match &opts.arg {
                        DocoptArgument::One(Some(d)) => Some(d.clone()),
                        _ => self.extract_default_value(&description),
                    };

                    // Strip redundant [default: ...] from description if we have a default value
                    if default.is_some() {
                        description = Self::strip_default_from_description(&description);
                    }

                    options.push(Option_ {
                        flag: flag_str,
                        short: short_flag,
                        option_type: option_type.to_string(),
                        description,
                        default,
                    });
                },
                Atom::Positional(name) => {
                    // Positional argument like <input>
                    let arg_name = name.clone();
                    let description = manual_descriptions
                        .get(&format!("<{name}>"))
                        .cloned()
                        .unwrap_or_default();

                    let arg_type = self.infer_argument_type(&arg_name, &description);

                    args_map.insert(
                        arg_name.clone(),
                        Argument {
                            name: arg_name.clone(),
                            arg_type,
                            required: !opts.arg.has_default(), // If it has a default, it's optional
                            description,
                            examples: Vec::new(),
                            r#enum: None,
                        },
                    );
                },
                Atom::Command(cmd_name) => {
                    // Collect subcommand names (e.g., "rows", "rowskey", "columns" for cat command)
                    // Skip the main command name itself (e.g., skip "cat" when parsing cat command)
                    // Also skip "--" which is just a docopt separator, not a real subcommand
                    if cmd_name != &self.command_name && cmd_name != "--" {
                        subcommands.push(cmd_name.clone());
                    }
                },
            }
        }

        // If subcommands were found, create a special "subcommand" argument
        // This should be the first argument in the list
        if !subcommands.is_empty() {
            // Sort subcommands alphabetically for deterministic output
            subcommands.sort_unstable();

            // Extract description for subcommands from USAGE text
            let subcommand_desc = self.extract_subcommand_description(&subcommands);

            // Check if subcommand is optional by looking for usage patterns without it
            // e.g., "qsv validate [options] [<input>...]" alongside "qsv validate schema ..."
            let subcommand_optional = self.is_subcommand_optional(&subcommands);

            // Create subcommand argument with enum of valid values
            let subcommand_arg = Argument {
                name:        "subcommand".to_string(),
                arg_type:    "string".to_string(),
                required:    !subcommand_optional, // Usually required, but can be optional
                description: subcommand_desc,
                examples:    Vec::new(),
                r#enum:      Some(subcommands),
            };

            // Insert at the beginning of args_map (will be reordered below)
            args_map.insert("subcommand".to_string(), subcommand_arg);
        }

        // Reorder args based on their appearance in the USAGE line
        let arg_order = self.extract_arg_order_from_usage();
        let mut args = Vec::new();

        // If we have subcommands, add the subcommand argument first
        if let Some(subcommand_arg) = args_map.remove("subcommand") {
            args.push(subcommand_arg);
        }

        // Then add other args in their USAGE order
        for arg_name in arg_order {
            if let Some(arg) = args_map.remove(&arg_name) {
                args.push(arg);
            }
        }

        // Sort options for consistent output
        options.sort_by(|a, b| a.flag.cmp(&b.flag));

        Ok((args, options))
    }

    /// Check if subcommand is optional by looking for usage patterns without subcommands
    /// e.g., validate command has both "qsv validate schema" and "qsv validate [<input>]"
    fn is_subcommand_optional(&self, subcommands: &[String]) -> bool {
        // Look for usage lines that don't include any subcommand
        // These indicate the command can run without a subcommand
        let usage_lines: Vec<&str> = self
            .usage_text
            .lines()
            .skip_while(|l| !l.contains("Usage:"))
            .skip(1) // Skip "Usage:" line itself
            .take_while(|l| {
                !l.trim().is_empty() && !l.contains("options:") && !l.contains("arguments:")
            })
            .collect();

        // Check if any usage line has the command name but no subcommands
        for line in usage_lines {
            // Skip the --help line as it's not a real usage pattern
            if line.contains("--help") {
                continue;
            }

            if line.contains(&format!("qsv {}", self.command_name)) {
                // Check if this line contains any of the subcommand names
                let has_subcommand = subcommands.iter().any(|sub| line.contains(sub));

                // If line has the command but no subcommand, then subcommands are optional
                if !has_subcommand {
                    return true;
                }
            }
        }

        false
    }

    /// Extract description for subcommand argument
    /// Creates a helpful description listing available subcommands
    fn extract_subcommand_description(&self, subcommands: &[String]) -> String {
        if subcommands.is_empty() {
            return String::new();
        }

        // Simple and clean: just list the valid subcommand values
        // Detailed help for each subcommand is available via --help
        format!(
            "Subcommand to execute. Valid values: {}",
            subcommands.join(", ")
        )
    }

    /// Extract descriptions from the usage text manually
    /// Returns a map of flag/arg name to description
    fn extract_descriptions_from_text(&self) -> std::collections::HashMap<String, String> {
        let mut descriptions = std::collections::HashMap::new();
        let lines: Vec<&str> = self.usage_text.lines().collect();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            let trimmed = line.trim();

            // Look for option lines: "    -s, --select <arg>    Description"
            if trimmed.starts_with('-') {
                // Extract flag and description
                if let Some((flags_part, desc_part)) = trimmed.split_once("  ") {
                    let mut description = desc_part.trim().to_string();

                    // Collect multi-line description
                    let mut j = i + 1;
                    while j < lines.len() {
                        let next_line = lines[j].trim();
                        if next_line.is_empty() || next_line.starts_with('-') {
                            break;
                        }
                        if !next_line.starts_with("Usage:") {
                            description.push(' ');
                            description.push_str(next_line);
                        }
                        j += 1;
                    }

                    // Parse flags
                    for flag in flags_part.split(',') {
                        let flag = flag.split_whitespace().next().unwrap_or("");
                        if flag.starts_with("--") || flag.starts_with('-') {
                            descriptions.insert(flag.to_string(), description.clone());
                        }
                    }

                    i = j;
                    continue;
                }
            }
            // Look for argument lines: "    <input>    Description"
            else if trimmed.starts_with('<')
                && trimmed.contains('>')
                && let Some(close_bracket) = trimmed.find('>')
            {
                let arg_name = trimmed[..=close_bracket].trim().to_string();
                let desc_part = trimmed[close_bracket + 1..].trim();

                let mut description = desc_part.to_string();

                // Collect multi-line description
                let mut j = i + 1;
                while j < lines.len() {
                    let next_line = lines[j].trim();
                    if next_line.is_empty()
                        || next_line.starts_with('<')
                        || next_line.starts_with('-')
                    {
                        break;
                    }
                    if !next_line.starts_with("Usage:") {
                        description.push(' ');
                        description.push_str(next_line);
                    }
                    j += 1;
                }

                descriptions.insert(arg_name, description);
                i = j;
                continue;
            }

            i += 1;
        }

        descriptions
    }

    fn extract_description(&self) -> Result<String, String> {
        // Extract first paragraph (before "Usage:" section)
        let lines: Vec<&str> = self.usage_text.lines().collect();
        let mut description_lines = Vec::new();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("Usage:") {
                break;
            }
            if trimmed.starts_with("For more examples,") || trimmed.starts_with("Examples:") {
                break;
            }
            if !trimmed.is_empty() && !trimmed.starts_with('$') && !trimmed.starts_with('#') {
                description_lines.push(trimmed);
            }
        }

        if description_lines.is_empty() {
            return Err("No description found".to_string());
        }

        Ok(description_lines.join(" "))
    }

    fn infer_argument_type(&self, name: &str, description: &str) -> String {
        let name_lower = name.to_lowercase();
        let desc_lower = description.to_lowercase();

        if name_lower.contains("input") || name_lower.contains("file") {
            "file".to_string()
        } else if name_lower.contains("number")
            || name_lower.contains("count")
            || desc_lower.contains("number")
        {
            "number".to_string()
        } else if name_lower.contains("regex")
            || name_lower.contains("pattern")
            || desc_lower.contains("regex")
            || desc_lower.contains("regular expression")
        {
            "regex".to_string()
        } else {
            // if name_lower.contains("column") || name_lower.contains("selection")
            // Also, default to string if we can't infer a better type
            "string".to_string()
        }
    }

    fn extract_default_value(&self, description: &str) -> Option<String> {
        // Look for [default: value] pattern
        if let Some(start) = description.find("[default:")
            && let Some(end) = description[start..].find(']')
        {
            let default_str = &description[start + 9..start + end];
            return Some(default_str.trim().to_string());
        }
        None
    }

    /// Remove [default: value] text from description to avoid redundancy
    /// when we have a separate default field
    fn strip_default_from_description(description: &str) -> String {
        if let Some(start) = description.find("[default:")
            && let Some(end) = description[start..].find(']')
        {
            // Remove the [default: ...] part and clean up extra whitespace
            let before = description[..start].trim();
            let after = description[start + end + 1..].trim();

            // Join with a space, but avoid double spaces
            if after.is_empty() {
                before.to_string()
            } else if before.is_empty() {
                after.to_string()
            } else {
                format!("{before} {after}")
            }
        } else {
            description.to_string()
        }
    }

    fn extract_hints(&self) -> Option<BehavioralHints> {
        // First try to look for emoji markers in usage text
        let has_memory_intensive_in_usage = self.usage_text.contains("🤯");
        let has_indexed_in_usage = self.usage_text.contains("📇");
        let has_proportional_memory_in_usage = self.usage_text.contains("😣");

        // If not found in usage text, check README.md command table
        let (readme_indexed, readme_memory_intensive, readme_proportional_memory) =
            Self::extract_hints_from_readme(&self.command_name);

        // Prefer usage text markers, fallback to README markers
        let has_indexed = has_indexed_in_usage || readme_indexed;
        let has_memory_intensive = has_memory_intensive_in_usage || readme_memory_intensive;
        let has_proportional_memory =
            has_proportional_memory_in_usage || readme_proportional_memory;

        let memory = if has_memory_intensive {
            "full"
        } else if has_proportional_memory {
            "proportional"
        } else {
            "constant"
        };

        // Most commands are streamable unless they load everything into memory
        let streamable = memory == "constant";

        Some(BehavioralHints {
            streamable,
            indexed: if has_indexed { Some(true) } else { None },
            memory: memory.to_string(),
        })
    }

    /// Extract hints from README.md command table
    /// Returns (indexed, memory_intensive, proportional_memory)
    fn extract_hints_from_readme(command_name: &str) -> (bool, bool, bool) {
        // Try to find the README.md in the repo root
        let readme_paths = ["README.md", "../README.md", "../../README.md"];

        for readme_path in &readme_paths {
            if let Ok(readme_content) = fs::read_to_string(readme_path) {
                // Find the line for this command in the table
                // Format: | [command](/src/cmd/command.rs#L2)✨<br>📇🚀🧠🤖🔣👆| Description |
                // Note: The #L2 line number varies, so we need to match more flexibly
                let command_pattern = format!("| [{command_name}](/src/cmd/{command_name}.rs#");

                if let Some(line) = readme_content
                    .lines()
                    .find(|l| l.contains(&command_pattern))
                {
                    // Extract only the emoji marker section (between <br> and the next |)
                    // to avoid matching emojis in the description text (e.g., "index" has 📇 in
                    // description)
                    let emoji_section = if let Some(br_pos) = line.find("<br>") {
                        if let Some(pipe_pos) = line[br_pos..].find('|') {
                            &line[br_pos..br_pos + pipe_pos]
                        } else {
                            &line[br_pos..]
                        }
                    } else {
                        // No <br> marker means no emoji markers for this command
                        ""
                    };

                    let indexed = emoji_section.contains("📇");
                    let memory_intensive = emoji_section.contains("🤯");
                    let proportional_memory = emoji_section.contains("😣");

                    return (indexed, memory_intensive, proportional_memory);
                }
            }
        }

        // Fallback: no hints found
        (false, false, false)
    }

    /// Extract short description from README.md command table
    /// Returns concise description suitable for MCP tool listing
    fn extract_short_description_from_readme(command_name: &str) -> Option<String> {
        let readme_paths = ["README.md", "../README.md", "../../README.md"];

        for readme_path in &readme_paths {
            if let Ok(readme_content) = fs::read_to_string(readme_path) {
                // Find the line for this command in the table
                // Format: | [command](/src/cmd/command.rs#L2)✨<br>📇| Description |
                let command_pattern = format!("| [{command_name}](/src/cmd/{command_name}.rs#");

                if let Some(line) = readme_content
                    .lines()
                    .find(|l| l.contains(&command_pattern))
                {
                    // Handle escaped pipes in markdown table (e.g., \| in code examples)
                    // Replace escaped pipes with placeholder before splitting
                    let placeholder = "\x00PIPE\x00";
                    let line_escaped = line.replace(r"\|", placeholder);

                    // Extract description: everything after the second | and before trailing |
                    // The format is: | command_cell | description |
                    let parts: Vec<&str> = line_escaped.split('|').collect();
                    if parts.len() >= 3 {
                        // Restore escaped pipes in description
                        let description = parts[2].trim().replace(placeholder, "|");

                        // Clean up the description:
                        // 1. Remove markdown links: [text](url) -> text
                        // 2. Remove HTML tags like <br>, <a name=...>
                        // 3. Remove deeplink anchors
                        let cleaned = Self::clean_readme_description(&description);

                        if !cleaned.is_empty() {
                            return Some(cleaned);
                        }
                    }
                }
            }
        }

        None
    }

    /// Clean README description by removing markdown links, HTML tags, etc.
    fn clean_readme_description(description: &str) -> String {
        let mut result = description.to_string();

        // Remove <a name="..."></a> anchor tags
        let anchor_re = regex_oncelock!(r#"<a name="[^"]*"></a>"#);
        result = anchor_re.replace_all(&result, "").to_string();

        // Remove <a name=...> anchor tags (without closing tag)
        let anchor_re2 = regex_oncelock!(r#"<a name=[^>]*>"#);
        result = anchor_re2.replace_all(&result, "").to_string();

        // Remove markdown links: [text](url) -> text
        // Handle URLs with nested parentheses (e.g., Wikipedia links like Frequency_(statistics))
        let link_re = regex_oncelock!(r"\[([^\]]+)\]\((?:[^()]+|\([^()]*\))*\)");
        result = link_re.replace_all(&result, "$1").to_string();

        // Remove remaining HTML tags
        let html_re = regex_oncelock!(r"<[^>]+>");
        result = html_re.replace_all(&result, " ").to_string();

        // Remove emoji markers that might be in description
        // (these are the behavioral hint emojis, not content emojis)
        let emojis_to_remove = [
            "📇",
            "🤯",
            "😣",
            "🧠",
            "🗄️",
            "🗃️",
            "🐻‍❄️",
            "🤖",
            "🏎️",
            "🚀",
            "🌐",
            "🔣",
            "👆",
            "🪄",
            "📚",
            "🌎",
            "⛩️",
            "✨",
        ];
        for emoji in emojis_to_remove {
            result = result.replace(emoji, "");
        }

        // Remove only empty parentheses "()" that remain after stripping emoji references
        // Don't remove parentheses with content as they may contain legitimate abbreviations
        // like (SEM), (CV), (XLSX), etc.
        result = result.replace("()", "");

        // Clean up whitespace
        let whitespace_re = regex_oncelock!(r"\s+");
        result = whitespace_re.replace_all(&result, " ").to_string();

        result.trim().to_string()
    }

    fn infer_category(&self) -> String {
        match self.command_name.as_str() {
            "select" | "slice" | "take" | "sample" | "head" | "tail" => "selection",
            "search" | "searchset" | "grep" | "filter" => "filtering",
            "apply" | "applydp" | "rename" | "transpose" | "reverse" | "datefmt" | "replace" => {
                "transformation"
            },
            "stats" | "moarstats" | "frequency" | "count" | "groupby" => "aggregation",
            "join" | "joinp" => "joining",
            "schema" | "validate" | "safenames" => "validation",
            "fmt" | "fixlengths" | "table" | "align" => "formatting",
            "to" | "input" | "excel" | "json" | "jsonl" | "tojsonl" => "conversion",
            "correlation" | "describegpt" => "analysis",
            _ => "utility",
        }
        .to_string()
    }
}

trait HasDefault {
    fn has_default(&self) -> bool;
}

impl HasDefault for DocoptArgument {
    fn has_default(&self) -> bool {
        matches!(self, DocoptArgument::One(Some(_)))
    }
}

fn extract_usage_from_file(file_path: &Path) -> Result<String, String> {
    let content = fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {e}"))?;

    // Find USAGE constant - handle both r#" and r##" delimiters
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

/// Public function to generate MCP skills JSON files
/// Called via `qsv --update-mcp-skills` flag
pub fn generate_mcp_skills() -> CliResult<()> {
    // Get all commands from src/cmd/*.rs (excluding mod.rs and duplicates)
    // Note: "enumerate" command is invoked as "enum" in qsv
    let commands = vec![
        "apply",
        "behead",
        "cat",
        "clipboard",
        "count",
        "datefmt",
        "dedup",
        "describegpt",
        "diff",
        "edit",
        "enumerate",
        "excel",
        "exclude",
        "explode",
        "extdedup",
        "extsort",
        "fetch",
        "fetchpost",
        "fill",
        "fixlengths",
        "flatten",
        "fmt",
        "foreach",
        "frequency",
        "geocode",
        "geoconvert",
        "headers",
        "index",
        "input",
        "join",
        "joinp",
        "json",
        "jsonl",
        "lens",
        "luau",
        "moarstats",
        "partition",
        "pivotp",
        "pro",
        "prompt",
        "pseudo",
        "python",
        "rename",
        "replace",
        "reverse",
        "safenames",
        "sample",
        "schema",
        "search",
        "searchset",
        "select",
        "slice",
        "snappy",
        "sniff",
        "sort",
        "sortcheck",
        "split",
        "sqlp",
        "stats",
        "table",
        "template",
        "to",
        "tojsonl",
        "transpose",
        "validate",
    ];

    // Determine repository root - look for Cargo.toml with src/cmd
    // This command must be run from within the qsv repository directory
    let mut repo_root = std::env::current_dir()?;
    let original_dir = repo_root.clone();

    let mut iterations = 0;

    loop {
        if repo_root.join("Cargo.toml").exists() && repo_root.join("src/cmd").exists() {
            break;
        }

        iterations += 1;
        if iterations >= MAX_ITERATIONS {
            return fail_clierror!(
                "Could not find qsv repository root after checking {} parent directories. \
                 This command must be run from within the qsv repository directory \
                 (where Cargo.toml and src/cmd exist).\n\
                 Original directory: {}\n\
                 \n\
                 If you're using a package-installed qsv binary, you need to:\n\
                 1. Clone the qsv repository: git clone https://github.com/dathere/qsv.git\n\
                 2. cd into the repository: cd qsv\n\
                 3. Run: qsv --update-mcp-skills",
                MAX_ITERATIONS,
                original_dir.display()
            );
        }

        if !repo_root.pop() {
            return fail_clierror!(
                "Could not find qsv repository root. This command must be run from within \
                 the qsv repository directory (where Cargo.toml and src/cmd exist).\n\
                 Original directory: {}\n\
                 \n\
                 If you're using a package-installed qsv binary, you need to:\n\
                 1. Clone the qsv repository: git clone https://github.com/dathere/qsv.git\n\
                 2. cd into the repository: cd qsv\n\
                 3. Run: qsv --update-mcp-skills",
                original_dir.display()
            );
        }
    }

    // Create output directory
    let output_dir = repo_root.join(".claude/skills/qsv");
    fs::create_dir_all(&output_dir)?;

    eprintln!("QSV MCP Skills Generator (via qsv --update-mcp-skills)");
    eprintln!("=======================================================");
    eprintln!("Repository: {}", repo_root.display());
    eprintln!("Output: {}", output_dir.display());
    eprintln!("Generating {} skills...\n", commands.len());

    let mut success_count = 0;
    let mut error_count = 0;

    for cmd_name in &commands {
        eprintln!("Processing: {cmd_name}");

        // Find command file
        // Note: enumerate.rs is invoked as "enum", python.rs as "py"
        let cmd_file = repo_root.join(format!("src/cmd/{cmd_name}.rs"));

        if !cmd_file.exists() {
            eprintln!("  ❌ File not found: {}", cmd_file.display());
            error_count += 1;
            continue;
        }

        // Extract USAGE text
        let usage_text = match extract_usage_from_file(&cmd_file) {
            Ok(text) => text,
            Err(e) => {
                eprintln!("  ❌ Failed to extract usage: {e}");
                error_count += 1;
                continue;
            },
        };

        // Parse into skill definition
        // For commands with aliases, extract the actual invocation name from USAGE
        // - enumerate is invoked as "enum"
        // - python is invoked as "py"
        let invocation_name = if usage_text.contains("qsv enum ") {
            "enum"
        } else if usage_text.contains("qsv py ") {
            "py"
        } else {
            cmd_name
        };

        let parser = UsageParser::new(usage_text, invocation_name.to_string());
        let skill = match parser.parse() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("  ❌ Failed to parse: {e}");
                error_count += 1;
                continue;
            },
        };

        // Write JSON file
        let output_file = output_dir.join(format!("{}.json", skill.name));
        let json = serde_json::to_string_pretty(&skill)?;
        fs::write(&output_file, json)?;

        eprintln!("  ✅ Generated: {}", output_file.display());
        eprintln!("     - {} arguments", skill.command.args.len());
        eprintln!("     - {} options", skill.command.options.len());
        eprintln!();

        success_count += 1;
    }

    eprintln!("\n✨ MCP Skills generation complete!");
    eprintln!("📁 Output directory: {}", output_dir.display());
    eprintln!(
        "📊 Summary: {} succeeded, {} failed out of {} total",
        success_count,
        error_count,
        commands.len()
    );

    if error_count > 0 {
        return fail_clierror!("{} skill(s) failed to generate", error_count);
    }

    eprintln!("\n💡 Restart Claude Desktop to load the updated skills.");

    Ok(())
}
