// qsv-test-examples-gen: Extract examples from qsv CI test files
//
// This tool parses Rust test files and extracts working examples with
// input data, commands, and expected outputs for load-as-needed documentation.

use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

macro_rules! regex_oncelock {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        #[allow(clippy::regex_creation_in_loops)] // false positive as we use oncelock
        RE.get_or_init(|| regex::Regex::new($re).expect("Invalid regex"))
    }};
}
#[derive(Debug, Serialize, Deserialize)]
struct TestExamples {
    skill:    String,
    version:  String,
    examples: Vec<TestExample>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestExample {
    name:        String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    input:       Option<TestInput>,
    command:     String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    args:        Vec<String>,
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty", default)]
    options:     std::collections::HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected:    Option<TestOutput>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    tags:        Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    data:     Option<Vec<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content:  Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    data:   Option<Vec<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stderr: Option<String>,
}

struct TestParser {
    command_name: String,
    test_content: String,
}

impl TestParser {
    fn new(command_name: String, test_content: String) -> Self {
        Self {
            command_name,
            test_content,
        }
    }

    fn parse(&self) -> Result<Vec<TestExample>, String> {
        let mut examples = Vec::new();
        let mut name_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        // First, try to extract macro-generated tests
        examples.extend(self.parse_macro_tests()?);

        // Find all #[test] function headers
        let test_header_re = regex_oncelock!(r"#\[test\]\s+fn\s+(\w+)\s*\(\s*\)\s*\{");

        for cap in test_header_re.captures_iter(&self.test_content) {
            let test_name = cap.get(1).unwrap().as_str();
            let match_start = cap.get(0).unwrap().start();
            let start_pos = cap.get(0).unwrap().end();

            // Skip if this test is inside a macro definition
            if self.is_inside_macro_definition(match_start) {
                continue;
            }

            // Try to find the module name by looking backwards
            let qualified_name = self.get_qualified_test_name(match_start, test_name);

            // Deduplicate by adding counter if name already exists
            let unique_name = if let Some(count) = name_counts.get_mut(&qualified_name) {
                *count += 1;
                format!("{qualified_name}_{count}")
            } else {
                name_counts.insert(qualified_name.clone(), 1);
                qualified_name.clone()
            };

            // Extract body by counting braces
            if let Some(test_body) = self.extract_function_body(start_pos)
                && let Ok(example) = self.parse_test_function(&unique_name, &test_body)
            {
                examples.push(example);
            }
        }

        Ok(examples)
    }

    /// Parse macro-generated tests (e.g., pivotp_test!(name, |wrk, cmd| { ... }))
    fn parse_macro_tests(&self) -> Result<Vec<TestExample>, String> {
        let mut examples = Vec::new();

        // Match: macro_name!(test_name, |params| { body })
        // Simple pattern to match test macro invocations
        let macro_re = regex_oncelock!(r#"_test!\s*\(\s*(\w+)"#);

        for cap in macro_re.captures_iter(&self.test_content) {
            let test_name = cap.get(1).unwrap().as_str();

            // Find the opening brace of the closure after the test name
            let match_end = cap.get(0).unwrap().end();
            if let Some(brace_pos) = self.test_content[match_end..].find('{') {
                let start_pos = match_end + brace_pos + 1;

                // Extract closure body by counting braces
                if let Some(test_body) = self.extract_function_body(start_pos)
                    && let Ok(example) = self.parse_test_function(test_name, &test_body)
                {
                    examples.push(example);
                }
            }
        }

        Ok(examples)
    }

    /// Check if a test position is inside a macro_rules! definition
    fn is_inside_macro_definition(&self, test_pos: usize) -> bool {
        let before_test = &self.test_content[..test_pos];

        // Look backwards for macro_rules! - if we find it before any closing brace at column 0,
        // then we're inside a macro definition
        let lines: Vec<&str> = before_test.lines().collect();

        for line in lines.iter().rev() {
            // If we hit a closing brace at the start of a line, we've left any macro scope
            if line.trim_start().starts_with('}') && line.starts_with('}') {
                return false;
            }
            // If we find macro_rules!, we're inside a macro definition
            if line.contains("macro_rules!") {
                return true;
            }
        }

        false
    }

    /// Extract module-qualified test name by looking backwards from test position
    fn get_qualified_test_name(&self, test_pos: usize, test_name: &str) -> String {
        let before_test = &self.test_content[..test_pos];

        // First try to find a macro invocation pattern like "macro_name!(test_name,"
        // This handles macro-generated tests
        let macro_re = regex_oncelock!(r"(\w+_test!)\s*\(\s*(\w+)\s*,");

        // Look for the last macro invocation before this test
        if let Some(cap) = macro_re.captures_iter(before_test).last()
            && let Some(module_name) = cap.get(2)
        {
            // Use the macro parameter as the module name (what the macro expands to)
            return module_name.as_str().to_string();
        }

        // Otherwise look for a regular "mod module_name {" before this test
        let mod_re = regex_oncelock!(r"mod\s+(\w+)\s*\{[^}]*$");

        if let Some(cap) = mod_re.captures(before_test)
            && let Some(module_name) = cap.get(1)
        {
            // Return module::function format if within a module
            return format!("{}::{}", module_name.as_str(), test_name);
        }

        // If no module or macro found, just return the test name
        test_name.to_string()
    }

    /// Extract function body by counting braces from start position
    fn extract_function_body(&self, start_pos: usize) -> Option<String> {
        let remaining = &self.test_content[start_pos..];
        let mut brace_count = 1; // We already have the opening brace
        let mut chars_collected = Vec::new();

        for ch in remaining.chars() {
            match ch {
                '{' => {
                    brace_count += 1;
                    chars_collected.push(ch);
                },
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        // Found the matching closing brace
                        return Some(chars_collected.into_iter().collect());
                    }
                    chars_collected.push(ch);
                },
                _ => {
                    chars_collected.push(ch);
                },
            }
        }

        None
    }

    fn parse_test_function(&self, name: &str, body: &str) -> Result<TestExample, String> {
        // Extract input data from wrk.create() calls
        let input = self.extract_input_data(body);

        // Extract command and arguments
        let (command, args, options) = self.extract_command(body)?;

        // Extract expected output
        let expected = self.extract_expected_output(body);

        // Generate description from test name
        let description = self.generate_description(name);

        // Infer tags from test name and body
        let tags = self.infer_tags(name, body);

        Ok(TestExample {
            name: name.to_string(),
            description,
            input,
            command,
            args,
            options,
            expected,
            tags,
        })
    }

    fn extract_input_data(&self, body: &str) -> Option<TestInput> {
        // Match wrk.create("filename", vec![svec![...], ...])
        let create_re = regex_oncelock!(
            r#"wrk\.create\(\s*"([^"]+)"\s*,\s*vec!\s*\[((?:\s*svec!\s*\[[^\]]*\]\s*,?)*)\s*\]"#,
        );

        if let Some(cap) = create_re.captures(body) {
            let filename = cap.get(1)?.as_str().to_string();
            let vec_content = cap.get(2)?.as_str();

            // Parse svec! macro calls
            let svec_re = regex_oncelock!(r#"svec!\s*\[([^\]]*)\]"#);
            let mut data = Vec::new();

            for svec_cap in svec_re.captures_iter(vec_content) {
                let items_str = svec_cap.get(1)?.as_str();
                let row: Vec<String> = items_str
                    .split(',')
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .collect();
                data.push(row);
            }

            if !data.is_empty() {
                return Some(TestInput {
                    data:     Some(data),
                    filename: Some(filename),
                    content:  None,
                });
            }
        }

        None
    }

    fn extract_command(
        &self,
        body: &str,
    ) -> Result<
        (
            String,
            Vec<String>,
            std::collections::HashMap<String, String>,
        ),
        String,
    > {
        // Match wrk.command("subcommand")
        let cmd_re = regex_oncelock!(r#"wrk\.command\("([^"]+)"\)"#);

        let subcommand = cmd_re
            .captures(body)
            .and_then(|c| c.get(1))
            .ok_or("No command found")?
            .as_str();

        // Extract .arg() calls - only match string literals, skip variable references
        // Match both .arg("...") for method chaining and cmd.arg("...") for variable calls
        let arg_re = regex_oncelock!(r#"(?:cmd|wrk)?\.?arg\("([^"]+)"\)"#);

        let mut args = Vec::new();
        let mut options = std::collections::HashMap::new();
        let mut pending_option: Option<String> = None;

        for cap in arg_re.captures_iter(body) {
            let arg_value = cap.get(1).unwrap().as_str().to_string();

            // Skip special arguments
            if arg_value == "--" {
                continue;
            }

            // Check if it's an option (starts with -)
            if arg_value.starts_with('-') {
                // If it's a flag-style option (like -i, --no-headers)
                if arg_value.starts_with("--") && !arg_value.contains('=') {
                    pending_option = Some(arg_value.clone());
                } else if arg_value.starts_with('-') && arg_value.len() == 2 {
                    pending_option = Some(arg_value.clone());
                } else {
                    args.push(arg_value);
                }
            } else if let Some(opt) = pending_option.take() {
                // This is the value for the previous option
                options.insert(opt, arg_value);
            } else {
                // Regular argument
                args.push(arg_value);
            }
        }

        // If there's a pending option without a value, it's a flag
        if let Some(opt) = pending_option {
            options.insert(opt, "true".to_string());
        }

        // Build command with proper shell quoting
        let mut command_parts = vec![subcommand.to_string()];

        // Add positional args
        for arg in &args {
            command_parts.push(Self::shell_quote(arg));
        }

        // Add options
        for (flag, value) in &options {
            command_parts.push(flag.clone());
            if value != "true" {
                // Not a boolean flag, add the value
                command_parts.push(Self::shell_quote(value));
            }
        }

        let full_command = format!("qsv {}", command_parts.join(" "));

        Ok((full_command, args, options))
    }

    /// Quote a string for safe use in shell commands
    ///
    /// This function determines if quoting is needed and applies proper escaping.
    fn shell_quote(arg: &str) -> String {
        // Check if quoting is needed
        let needs_quoting = arg.is_empty()
            || arg.contains(' ')
            || arg.contains('\t')
            || arg.contains('\n')
            || arg.contains('"')
            || arg.contains('\'')
            || arg.contains('\\')
            || arg.contains('$')
            || arg.contains('`')
            || arg.contains('|')
            || arg.contains('&')
            || arg.contains(';')
            || arg.contains('<')
            || arg.contains('>')
            || arg.contains('(')
            || arg.contains(')')
            || arg.contains('{')
            || arg.contains('}')
            || arg.contains('[')
            || arg.contains(']')
            || arg.contains('*')
            || arg.contains('?')
            || arg.contains('!')
            || arg.contains('#');

        if !needs_quoting {
            return arg.to_string();
        }

        // Escape backslashes and double quotes, then wrap in double quotes
        let escaped = arg.replace('\\', "\\\\").replace('"', "\\\"");
        format!("\"{escaped}\"")
    }

    fn extract_expected_output(&self, body: &str) -> Option<TestOutput> {
        // Match let expected = vec![svec![...], ...]
        let expected_re = regex_oncelock!(
            r#"let\s+expected\s*=\s*vec!\s*\[((?:\s*svec!\s*\[[^\]]*\]\s*,?)*)\s*\]"#
        );

        if let Some(cap) = expected_re.captures(body) {
            let vec_content = cap.get(1)?.as_str();

            // Parse svec! macro calls
            let svec_re = regex_oncelock!(r#"svec!\s*\[([^\]]*)\]"#);
            let mut data = Vec::new();

            for svec_cap in svec_re.captures_iter(vec_content) {
                let items_str = svec_cap.get(1)?.as_str();
                let row: Vec<String> = items_str
                    .split(',')
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .collect();
                data.push(row);
            }

            if !data.is_empty() {
                return Some(TestOutput {
                    data:   Some(data),
                    stdout: None,
                    stderr: None,
                });
            }
        }

        None
    }

    fn generate_description(&self, test_name: &str) -> String {
        // Convert snake_case to human readable
        test_name
            .replace('_', " ")
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().chain(chars).collect(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn infer_tags(&self, name: &str, body: &str) -> Vec<String> {
        let mut tags = Vec::new();

        // Tag from test name patterns
        if name.contains("issue_") {
            tags.push("regression".to_string());
        }
        if name.contains("error") || name.contains("err") {
            tags.push("error-handling".to_string());
        }
        if name.contains("normal") || name.contains("basic") {
            tags.push("basic".to_string());
        }
        if name.contains("case") {
            tags.push("case-sensitivity".to_string());
        }
        if name.contains("unicode") {
            tags.push("unicode".to_string());
        }

        // Tag from body content
        if body.contains("--no-headers") {
            tags.push("no-headers".to_string());
        }
        if body.contains("--delimiter") {
            tags.push("custom-delimiter".to_string());
        }

        tags
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let output_dir = PathBuf::from(".claude/skills/examples");
    fs::create_dir_all(&output_dir)?;

    println!("QSV Test Examples Generator");
    println!("===========================");
    println!(
        "Extracting examples from {} test files...\n",
        commands.len()
    );

    let mut success_count = 0;
    let mut error_count = 0;
    let mut total_examples = 0;

    for cmd_name in &commands {
        println!("Processing: test_{cmd_name}.rs");

        let test_file = PathBuf::from(format!("tests/test_{cmd_name}.rs"));

        if !test_file.exists() {
            println!("  ⚠️  Test file not found, skipping");
            continue;
        }

        let test_content = match fs::read_to_string(&test_file) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("  ❌ Failed to read test file: {e}");
                error_count += 1;
                continue;
            },
        };

        let parser = TestParser::new(cmd_name.to_string(), test_content);
        let examples = match parser.parse() {
            Ok(ex) => ex,
            Err(e) => {
                eprintln!("  ❌ Failed to parse tests: {e}");
                error_count += 1;
                continue;
            },
        };

        if examples.is_empty() {
            println!("  ⚠️  No examples extracted");
            continue;
        }

        let test_examples = TestExamples {
            skill:    format!("qsv-{cmd_name}"),
            version:  env!("CARGO_PKG_VERSION").to_string(),
            examples: examples.clone(),
        };

        let output_file = output_dir.join(format!("qsv-{cmd_name}-examples.json"));
        let json = serde_json::to_string_pretty(&test_examples)?;
        fs::write(&output_file, json)?;

        println!("  ✅ Generated: {}", output_file.display());
        println!("     - {} test examples extracted", examples.len());
        println!();

        total_examples += examples.len();
        success_count += 1;
    }

    println!("\n✨ Test example extraction complete!");
    println!("📁 Output directory: {}", output_dir.display());
    println!("📊 Summary: {success_count} succeeded, {error_count} failed",);
    println!("📚 Total examples extracted: {total_examples}");

    Ok(())
}
