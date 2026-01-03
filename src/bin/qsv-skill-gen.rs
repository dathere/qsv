// qsv-skill-gen: Generate Agent Skills from qsv command USAGE text
//
// This tool parses USAGE text from qsv commands and generates Agent Skill
// definitions in JSON format for use with the Claude Agent SDK.

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SkillDefinition {
    name:        String,
    version:     String,
    description: String,
    category:    String,
    command:     CommandSpec,
    examples:    Vec<Example>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hints:       Option<BehavioralHints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_file:   Option<String>,
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
struct Example {
    description: String,
    command:     String,
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
    fn new(usage_text: String, command_name: String) -> Self {
        Self {
            usage_text,
            command_name,
        }
    }

    fn parse(&self) -> Result<SkillDefinition, String> {
        let description = self.extract_description()?;
        let examples = self.extract_examples();
        let (args, options) = self.parse_arguments_and_options()?;
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
            examples,
            hints,
            test_file: Some(format!(
                "https://github.com/dathere/qsv/blob/master/tests/test_{}.rs",
                self.command_name
            )),
        })
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
            if !trimmed.is_empty() && !trimmed.starts_with("$") && !trimmed.starts_with("#") {
                description_lines.push(trimmed);
            }
        }

        if description_lines.is_empty() {
            return Err("No description found".to_string());
        }

        Ok(description_lines.join(" "))
    }

    fn extract_examples(&self) -> Vec<Example> {
        let lines: Vec<&str> = self.usage_text.lines().collect();
        let mut examples = Vec::new();
        let mut current_description = String::new();

        for line in lines {
            let trimmed = line.trim();

            // Comments are descriptions for the next example
            if trimmed.starts_with("#") {
                current_description = trimmed.trim_start_matches('#').trim().to_string();
            }
            // Commands start with $
            else if trimmed.starts_with("$") {
                let command = trimmed.trim_start_matches('$').trim().to_string();

                // Use comment as description, or extract from command
                let description = if !current_description.is_empty() {
                    current_description.clone()
                } else {
                    // Try to infer description from command
                    self.infer_example_description(&command)
                };

                examples.push(Example {
                    description,
                    command,
                });

                current_description.clear();
            }
        }

        examples
    }

    fn infer_example_description(&self, command: &str) -> String {
        // Simple heuristic: use the command itself as description
        if let Some(after_subcommand) = command.strip_prefix("qsv ") {
            if let Some(rest) = after_subcommand.strip_prefix(&format!("{} ", self.command_name)) {
                return rest.to_string();
            }
        }
        command.to_string()
    }

    fn parse_arguments_and_options(&self) -> Result<(Vec<Argument>, Vec<Option_>), String> {
        let mut args = Vec::new();
        let mut options = Vec::new();

        let lines: Vec<&str> = self.usage_text.lines().collect();
        let mut in_args_section = false;
        let mut in_options_section = false;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Detect section headers
            if trimmed.ends_with("arguments:") || trimmed.starts_with("Arguments:") {
                in_args_section = true;
                in_options_section = false;
                continue;
            }
            if trimmed.ends_with("options:")
                || trimmed.starts_with("Options:")
                || trimmed.starts_with("Common options:")
            {
                in_options_section = true;
                in_args_section = false;
                continue;
            }

            // Parse argument definitions
            if in_args_section && trimmed.starts_with("<") {
                if let Some(arg) = self.parse_argument_line(line, &lines, i) {
                    args.push(arg);
                }
            }

            // Parse option definitions
            if in_options_section && trimmed.starts_with("-") {
                if let Some(opt) = self.parse_option_line(line, &lines, i) {
                    options.push(opt);
                }
            }
        }

        Ok((args, options))
    }

    fn parse_argument_line(
        &self,
        line: &str,
        all_lines: &[&str],
        index: usize,
    ) -> Option<Argument> {
        // Parse format: "    <name>            Description text"
        let trimmed = line.trim();

        // Extract argument name (between < and >)
        let start = trimmed.find('<')?;
        let end = trimmed.find('>')?;
        let name = trimmed[start + 1..end].to_string();

        // Check if optional (wrapped in [])
        let required = !line.contains(&format!("[<{}>]", name));

        // Extract description (everything after the argument name)
        let after_name = trimmed[end + 1..].trim();
        let mut description = after_name.to_string();

        // Multi-line descriptions: collect continuation lines
        let mut next_idx = index + 1;
        while next_idx < all_lines.len() {
            let next_line = all_lines[next_idx].trim();
            if next_line.is_empty() || next_line.starts_with("<") || next_line.starts_with("-") {
                break;
            }
            if !next_line.starts_with("Usage:") && !next_line.starts_with("For more") {
                description.push(' ');
                description.push_str(next_line);
            }
            next_idx += 1;
        }

        // Infer type from name and description
        let arg_type = self.infer_argument_type(&name, &description);

        Some(Argument {
            name,
            arg_type,
            required,
            description,
            examples: Vec::new(),
        })
    }

    fn parse_option_line(&self, line: &str, all_lines: &[&str], index: usize) -> Option<Option_> {
        // Parse format: "    -s, --select <arg>    Description text"
        let trimmed = line.trim();

        // Extract flags
        let parts: Vec<&str> = trimmed.splitn(2, "  ").collect();
        if parts.len() < 2 {
            return None;
        }

        let flag_part = parts[0].trim();
        let mut description = parts[1].trim().to_string();

        // Parse short and long flags
        let flags: Vec<&str> = flag_part.split(',').map(|s| s.trim()).collect();
        let mut short = None;
        let mut long = None;

        for flag in flags {
            if flag.starts_with("--") {
                long = Some(flag.split_whitespace().next()?.to_string());
            } else if flag.starts_with("-") {
                short = Some(flag.split_whitespace().next()?.to_string());
            }
        }

        let flag = long.or_else(|| short.clone())?;

        // Multi-line descriptions
        let mut next_idx = index + 1;
        while next_idx < all_lines.len() {
            let next_line = all_lines[next_idx].trim();
            if next_line.is_empty() || next_line.starts_with("-") {
                break;
            }
            description.push(' ');
            description.push_str(next_line);
            next_idx += 1;
        }

        // Determine option type
        let option_type = if flag_part.contains("<") {
            // Has argument
            if flag_part.contains("<number>") || flag_part.contains("<int>") {
                "number"
            } else {
                "string"
            }
        } else {
            "flag"
        };

        // Extract default value
        let default = self.extract_default_value(&description);

        Some(Option_ {
            flag: flag.clone(),
            short: if short.is_some() && short != Some(flag) {
                short
            } else {
                None
            },
            option_type: option_type.to_string(),
            description,
            default,
        })
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
        } else if name_lower.contains("column") || name_lower.contains("selection") {
            "string".to_string()
        } else {
            "string".to_string()
        }
    }

    fn extract_default_value(&self, description: &str) -> Option<String> {
        // Look for [default: value] pattern
        if let Some(start) = description.find("[default:") {
            if let Some(end) = description[start..].find(']') {
                let default_str = &description[start + 9..start + end];
                return Some(default_str.trim().to_string());
            }
        }
        None
    }

    fn extract_hints(&self) -> Option<BehavioralHints> {
        // Look for emoji markers in usage text
        let has_memory_intensive = self.usage_text.contains("🤯");
        let has_indexed = self.usage_text.contains("📇");
        let has_proportional_memory = self.usage_text.contains("😣");

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

    fn infer_category(&self) -> String {
        match self.command_name.as_str() {
            "select" | "slice" | "take" | "sample" | "head" | "tail" => "selection",
            "search" | "searchset" | "grep" | "filter" => "filtering",
            "apply" | "rename" | "transpose" | "reverse" | "datefmt" => "transformation",
            "stats" | "moarstats" | "frequency" | "count" | "groupby" => "aggregation",
            "join" | "joinp" => "joining",
            "schema" | "validate" | "safenames" => "validation",
            "fmt" | "fixlengths" | "table" | "align" => "formatting",
            "to" | "input" | "excel" | "lua" | "foreach" | "python" => "conversion",
            "correlation" | "describegpt" => "analysis",
            _ => "utility",
        }
        .to_string()
    }
}

fn extract_usage_from_file(file_path: &Path) -> Result<String, String> {
    let content =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get all commands from src/cmd/*.rs (excluding mod.rs and duplicates)
    let commands = vec![
        "apply",
        "applydp",
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

    // Create output directory
    let output_dir = PathBuf::from(".claude/skills/qsv");
    fs::create_dir_all(&output_dir)?;

    println!("QSV Agent Skill Generator");
    println!("=========================");
    println!("Generating {} skills...\n", commands.len());

    let mut success_count = 0;
    let mut error_count = 0;

    for cmd_name in &commands {
        println!("Processing: {}", cmd_name);

        // Find command file
        let cmd_file = PathBuf::from(format!("src/cmd/{}.rs", cmd_name));

        if !cmd_file.exists() {
            eprintln!("  ❌ File not found: {}", cmd_file.display());
            error_count += 1;
            continue;
        }

        // Extract USAGE text
        let usage_text = match extract_usage_from_file(&cmd_file) {
            Ok(text) => text,
            Err(e) => {
                eprintln!("  ❌ Failed to extract usage: {}", e);
                error_count += 1;
                continue;
            },
        };

        // Parse into skill definition
        let parser = UsageParser::new(usage_text, cmd_name.to_string());
        let skill = match parser.parse() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("  ❌ Failed to parse: {}", e);
                error_count += 1;
                continue;
            },
        };

        // Write JSON file
        let output_file = output_dir.join(format!("{}.json", skill.name));
        let json = serde_json::to_string_pretty(&skill)?;
        fs::write(&output_file, json)?;

        println!("  ✅ Generated: {}", output_file.display());
        println!("     - {} examples", skill.examples.len());
        println!("     - {} arguments", skill.command.args.len());
        println!("     - {} options", skill.command.options.len());
        println!();

        success_count += 1;
    }

    println!("\n✨ Skill generation complete!");
    println!("📁 Output directory: {}", output_dir.display());
    println!(
        "📊 Summary: {} succeeded, {} failed out of {} total",
        success_count,
        error_count,
        commands.len()
    );

    Ok(())
}
