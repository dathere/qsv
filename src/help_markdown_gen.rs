#![allow(clippy::needless_continue, clippy::ref_as_ptr)]
// qsv Help Markdown Generator - Generate Markdown help files from qsv command USAGE text
//
// This module parses USAGE text from all qsv commands and generates readable Markdown
// help files in `docs/help/`, plus a Table of Contents and updated README links.
//
// Uses qsv-docopt Parser for robust structured parsing of options, arguments and defaults,
// combined with manual text parsing for descriptions and prose content.

use std::{fmt::Write, fs, path::Path};

use foldhash::{HashMap, HashMapExt};
use qsv_docopt::parse::{Argument as DocoptArgument, Atom, Parser};

use crate::{CliResult, regex_oncelock};

const MAX_ITERATIONS: usize = 100;
const GITHUB_BASE: &str = "https://github.com/dathere/qsv/blob/master/";

/// Information about a command extracted from README.md
struct CommandInfo {
    /// The invocation name (e.g. "enum", "py")
    invocation_name: String,
    /// The source file stem (e.g. "enumerate", "python")
    source_file:     String,
    /// Short description from README table
    description:     String,
    /// Emoji markers from README table
    emoji_markers:   String,
}

/// Extract all commands from the README.md command table.
/// Returns a Vec of CommandInfo with invocation name, source file, description and emojis.
fn extract_commands_from_readme(repo_root: &Path) -> Result<Vec<CommandInfo>, String> {
    let readme_path = repo_root.join("README.md");
    let readme_content =
        fs::read_to_string(&readme_path).map_err(|e| format!("Failed to read README.md: {e}"))?;

    let mut commands = Vec::new();

    // Match lines like: | [apply](/src/cmd/apply.rs#L2)...|...|
    // or: | [moarstats](/src/cmd/moarstats.rs)<br>...|...|
    let src_link_re = regex_oncelock!(r"\| \[(\w+)\]\(/src/cmd/(\w+)\.rs(?:#L\d+)?\)");

    // Also match already-updated links: | [apply](docs/help/apply.md)...|...|
    let help_link_re = regex_oncelock!(r"\| \[(\w+)\]\(docs/help/\w+\.md\)");

    // Map of invocation names to source file stems for special cases
    let special_mappings: HashMap<&str, &str> =
        HashMap::from_iter([("enum", "enumerate"), ("py", "python")]);

    for line in readme_content.lines() {
        if let Some(caps) = src_link_re.captures(line) {
            let invocation_name = caps[1].to_string();
            let source_file = caps[2].to_string();

            let emoji_markers = extract_emoji_section(line);
            let description = extract_description_from_line(line);

            commands.push(CommandInfo {
                invocation_name,
                source_file,
                description,
                emoji_markers,
            });
        } else if let Some(caps) = help_link_re.captures(line) {
            // Already-updated format: derive source file from invocation name
            let invocation_name = caps[1].to_string();
            let source_file = special_mappings
                .get(invocation_name.as_str())
                .map_or_else(|| invocation_name.clone(), |s| (*s).to_string());

            let emoji_markers = extract_emoji_section(line);
            let description = extract_description_from_line(line);

            commands.push(CommandInfo {
                invocation_name,
                source_file,
                description,
                emoji_markers,
            });
        }
    }

    if commands.is_empty() {
        return Err("No commands found in README.md command table".to_string());
    }

    Ok(commands)
}

/// Extract emoji markers from a README table line
fn extract_emoji_section(line: &str) -> String {
    // Look for content between <br> and the next | in the first cell
    if let Some(br_pos) = line.find("<br>") {
        // Find the next | after the <br>
        if let Some(pipe_pos) = line[br_pos..].find('|') {
            let section = &line[br_pos + 4..br_pos + pipe_pos];
            // Strip HTML tags and clean up
            let html_re = regex_oncelock!(r"<[^>]+>");
            let cleaned = html_re.replace_all(section, "").trim().to_string();
            return cleaned;
        }
    }
    String::new()
}

/// Parse the legend section from README.md into a vec of (emoji_key, description) pairs.
/// Returns pairs sorted by key length descending for longest-match-first replacement.
fn parse_legend(readme_content: &str) -> Vec<(String, String)> {
    let mut legend = Vec::new();
    let Some(start) = readme_content.find("<a name=\"legend_deeplink\">") else {
        return legend;
    };

    let legend_text = &readme_content[start..];
    // Regex to strip markdown links: [text](url) -> text
    let link_re = regex_oncelock!(r"\[([^\]]*)\]\([^)]*\)");
    // Regex to strip HTML tags
    let html_re = regex_oncelock!(r"<[^>]+>");
    // Regex to strip markdown image badges: [![alt](img)](url) -> empty
    let badge_re = regex_oncelock!(r"\[!\[[^\]]*\]\([^)]*\)\]\([^)]*\)");
    // Regex to strip incomplete/partial badge fragments (e.g. from multi-line badges)
    let partial_badge_re = regex_oncelock!(r"\[!\[[^\]]*\]\([^)]*$");

    // First, join continuation lines. A legend entry starts with an emoji or ![
    // at the beginning. Lines that don't start that way are continuations.
    let mut joined_lines: Vec<String> = Vec::new();
    for line in legend_text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        // Check if this line starts a new entry (emoji char, ![, or <a tag)
        let first_char = trimmed.chars().next().unwrap_or(' ');
        let is_new_entry =
            trimmed.starts_with("<a ") || trimmed.starts_with("![") || !first_char.is_ascii();
        if is_new_entry || joined_lines.is_empty() {
            joined_lines.push(trimmed.to_string());
        } else if let Some(last) = joined_lines.last_mut() {
            // Continuation line — append to previous
            last.push(' ');
            last.push_str(trimmed);
        }
    }

    for joined_line in &joined_lines {
        // Strip HTML anchor tags first
        let cleaned = if let Some(close_pos) = joined_line.find("</a>") {
            let before_close = &joined_line[..close_pos];
            let after_close = &joined_line[close_pos + 4..];
            if let Some(open_end) = before_close.rfind('>') {
                let inner = &before_close[open_end + 1..];
                format!("{inner}{after_close}")
            } else {
                after_close.to_string()
            }
        } else {
            joined_line.to_string()
        };

        let cleaned = cleaned.trim().to_string();
        if cleaned.is_empty() {
            continue;
        }

        // Split on first ": " to get key and description
        let (key, desc) = if let Some(pos) = cleaned.find(": ") {
            // Check if there's a space before the colon (image emoji style like `![X](y) :`)
            let before_colon = &cleaned[..pos];
            let after_colon = &cleaned[pos + 2..];
            if before_colon.ends_with(' ') {
                (before_colon.trim_end().to_string(), after_colon.to_string())
            } else {
                (before_colon.to_string(), after_colon.to_string())
            }
        } else {
            continue;
        };

        if key.is_empty() || desc.is_empty() {
            continue;
        }

        // Rewrite image paths in key to match the rewritten paths in markers
        // (markers have docs/images/ -> ../images/ applied before tooltip wrapping)
        let key = key.replace("docs/images/", "../images/");

        // Clean description for tooltip: strip badges, partial badges, markdown links, HTML
        let clean_desc = badge_re.replace_all(&desc, "").to_string();
        let clean_desc = partial_badge_re.replace_all(&clean_desc, "").to_string();
        let clean_desc = link_re.replace_all(&clean_desc, "$1").to_string();
        let clean_desc = html_re.replace_all(&clean_desc, "").to_string();
        // Escape double quotes for HTML title attribute
        let clean_desc = clean_desc.replace('"', "&quot;");
        let clean_desc = clean_desc.trim().to_string();

        if !clean_desc.is_empty() {
            legend.push((key, clean_desc));
        }
    }

    // Sort by key length descending for longest-match-first replacement
    legend.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    legend
}

/// Wrap emoji markers in a string with `<abbr>` tooltip tags using the parsed legend.
/// For unicode emojis: `<abbr title="description">emoji</abbr>`
/// For image markdown `![name](path)`: `![name](path "description")`
///
/// Uses a two-pass approach with placeholders to avoid replacing emojis that appear
/// inside already-inserted tooltip descriptions (e.g. 🏎️'s description mentions 📇).
fn wrap_emojis_with_tooltips(markers: &str, legend: &[(String, String)]) -> String {
    let mut result = markers.to_string();
    // Regex to match image markdown: ![name](path)
    let img_re = regex_oncelock!(r"^!\[([^\]]*)\]\(([^)]*)\)$");

    // Pass 1: Replace each emoji key with a unique placeholder, collecting replacements
    let mut replacements: Vec<String> = Vec::new();

    for (key, desc) in legend {
        if !result.contains(key.as_str()) {
            continue;
        }

        let replacement = if img_re.is_match(key) {
            // Image emoji: add title attribute to markdown image
            // ![name](path) -> ![name](path "description")
            if let Some(caps) = img_re.captures(key) {
                let name = &caps[1];
                let path = &caps[2];
                format!("![{name}]({path} \"{desc}\")")
            } else {
                continue;
            }
        } else {
            // Unicode emoji: wrap with <abbr>
            format!("<abbr title=\"{desc}\">{key}</abbr>")
        };

        // Use a placeholder that won't appear in normal text
        let idx = replacements.len();
        let placeholder = format!("\x00EMOJI{idx}\x00");
        replacements.push(replacement);
        result = result.replace(key.as_str(), &placeholder);
    }

    // Pass 2: Replace all placeholders with their actual values
    for (idx, replacement) in replacements.iter().enumerate() {
        let placeholder = format!("\x00EMOJI{idx}\x00");
        result = result.replace(&placeholder, replacement);
    }

    result
}

/// Extract description from the second column of a README table line
fn extract_description_from_line(line: &str) -> String {
    // Handle escaped pipes
    let placeholder = "\x00PIPE\x00";
    let line_escaped = line.replace(r"\|", placeholder);

    let parts: Vec<&str> = line_escaped.split('|').collect();
    if parts.len() >= 3 {
        let description = parts[2].trim().replace(placeholder, "|");
        clean_readme_description(&description)
    } else {
        String::new()
    }
}

/// Clean README description by removing HTML tags, emojis, etc.
/// Preserves markdown links so they remain clickable in the generated help pages.
/// Rewrites relative URLs to work from the `docs/help/` directory.
fn clean_readme_description(description: &str) -> String {
    let mut result = description.to_string();

    // Remove <a name="..."></a> anchor tags
    let anchor_re = regex_oncelock!(r#"<a name="[^"]*"></a>"#);
    result = anchor_re.replace_all(&result, "").to_string();

    // Remove <a name=...> anchor tags (without closing tag)
    let anchor_re2 = regex_oncelock!(r#"<a name=[^>]*>"#);
    result = anchor_re2.replace_all(&result, "").to_string();

    // Rewrite relative URLs in markdown links to work from docs/help/
    // Uses a regex that handles URLs with balanced parentheses (e.g. Wikipedia links).
    // [text](docs/foo) -> [text](../foo)  (go up from help/ to docs/)
    // [text](/src/foo) -> [text](../../src/foo)  (strip leading / and go up to repo root)
    // [text](resources/foo) -> [text](../../resources/foo)  (go up to repo root)
    // [text](#anchor) -> [text](../../README.md#anchor)  (anchors reference README sections)
    // Absolute URLs (http/https) and mailto links are left unchanged.
    let link_rewrite_re = regex_oncelock!(r"\]\(([^()]*(?:\([^()]*\))*[^()]*)\)");
    result = link_rewrite_re
        .replace_all(&result, |caps: &regex::Captures| {
            let path = &caps[1];
            // Skip absolute URLs and mailto links
            if path.starts_with("http://")
                || path.starts_with("https://")
                || path.starts_with("mailto:")
            {
                caps[0].to_string()
            } else if path.starts_with('#') {
                // Anchor-only links reference README sections, not the current help page
                format!("](../../README.md{path})")
            } else if let Some(rest) = path.strip_prefix("docs/") {
                format!("](../{rest})")
            } else {
                // Strip leading slash to avoid double slashes (e.g. /src/cmd/foo.rs)
                let clean_path = path.strip_prefix('/').unwrap_or(path);
                format!("](../../{clean_path})")
            }
        })
        .to_string();

    // Remove remaining HTML tags (but not markdown links)
    let html_re = regex_oncelock!(r"<[^>]+>");
    result = html_re.replace_all(&result, " ").to_string();

    // Remove emoji markers
    let emojis_to_remove = [
        "📇",
        "🤯",
        "😣",
        "🧠",
        "🗄️",
        "🗃️",
        "🐻\u{200d}❄️",
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
        "🖥️",
    ];
    for emoji in emojis_to_remove {
        result = result.replace(emoji, "");
    }

    // Remove empty parentheses
    result = result.replace("()", "");

    // Clean up whitespace
    let whitespace_re = regex_oncelock!(r"\s+");
    result = whitespace_re.replace_all(&result, " ").to_string();

    result.trim().to_string()
}

/// Extract USAGE text from a command source file
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

/// Parsed option from docopt + manual description extraction
struct ParsedOption {
    flag:        String,
    short:       Option<String>,
    option_type: String,
    description: String,
    default:     Option<String>,
}

/// Parsed positional argument
struct ParsedArgument {
    name:        String,
    description: String,
}

/// Convert a heading string to a GitHub-style markdown anchor slug
fn heading_to_anchor(heading: &str) -> String {
    heading.to_lowercase().replace(' ', "-")
}

/// Parse USAGE text and generate a Markdown help file.
fn generate_command_markdown(
    usage_text: &str,
    cmd_info: &CommandInfo,
    _repo_root: &Path,
    legend: &[(String, String)],
) -> String {
    let mut md = String::with_capacity(4096);

    let source_path = format!("src/cmd/{}.rs", cmd_info.source_file);
    let source_url = format!("{GITHUB_BASE}{source_path}");

    // Title
    let _ = write!(md, "# {}\n\n", cmd_info.invocation_name);

    // Short description from README
    if !cmd_info.description.is_empty() {
        let _ = write!(md, "> {}\n\n", cmd_info.description);
    }

    // Navigation with emoji markers
    let emoji_suffix = if cmd_info.emoji_markers.is_empty() {
        String::new()
    } else {
        // Rewrite image paths for the docs/help/ location
        let markers = cmd_info.emoji_markers.replace("docs/images/", "../images/");
        // Wrap emojis with hover tooltips
        let markers = wrap_emojis_with_tooltips(&markers, legend);
        format!(" | {markers}")
    };
    let _ = write!(
        md,
        "**[Table of Contents](TableOfContents.md)** | **Source: \
         [{source_path}]({source_url})**{emoji_suffix}\n\n"
    );

    // Parse the USAGE text into sections
    let sections = parse_usage_sections(usage_text);

    // Parse arguments and options early so we can collect all heading names
    let parsed_args = parse_arguments_section(&sections.arguments_text);
    let options_sections =
        parse_options_with_docopt(usage_text, &sections, &cmd_info.invocation_name);

    // Collect heading names in appearance order for the heading links bar
    let mut headings: Vec<String> = Vec::new();
    if !sections.description.is_empty() {
        headings.push("Description".to_string());
    }
    if !sections.examples.is_empty() {
        headings.push("Examples".to_string());
    }
    if !sections.usage_patterns.is_empty() {
        headings.push("Usage".to_string());
    }
    if !parsed_args.is_empty() {
        headings.push("Arguments".to_string());
    }
    for (section_title, options) in &options_sections {
        if !options.is_empty() {
            headings.push(section_title.clone());
        }
    }

    // Write heading links bar with anchor for back-navigation
    let has_nav = headings.len() > 1;
    if has_nav {
        let links: Vec<String> = headings
            .iter()
            .map(|h| format!("[{h}](#{})", heading_to_anchor(h)))
            .collect();
        md.push_str("<a name=\"nav\"></a>\n");
        md.push_str(&links.join(" | "));
        md.push_str("\n\n");
    }

    // Helper: write a section heading with an explicit anchor and optional back-link.
    // The explicit <a name="..."> ensures nav bar links resolve correctly even though
    // the back-link text changes the auto-generated heading ID.
    let write_heading = |md: &mut String, title: &str| {
        if has_nav {
            let _ = write!(md, "<a name=\"{}\"></a>\n\n", heading_to_anchor(title));
            let _ = write!(md, "## {title} [↩](#nav)\n\n");
        } else {
            let _ = write!(md, "## {title}\n\n");
        }
    };

    // Description section
    if !sections.description.is_empty() {
        write_heading(&mut md, "Description");
        md.push_str(&format_description(&sections.description));
        md.push('\n');
    }

    // Examples section
    if !sections.examples.is_empty() {
        write_heading(&mut md, "Examples");
        md.push_str(&format_examples(&sections.examples));
        md.push('\n');
    }

    // Usage patterns section
    if !sections.usage_patterns.is_empty() {
        write_heading(&mut md, "Usage");
        md.push_str("```console\n");
        for line in &sections.usage_patterns {
            md.push_str(line);
            md.push('\n');
        }
        md.push_str("```\n\n");
    }

    // Arguments section
    if !parsed_args.is_empty() {
        write_heading(&mut md, "Arguments");
        // Pad header to the longest argument name to prevent word-wrap
        let max_arg_len = parsed_args.iter().map(|a| a.name.len()).max().unwrap_or(0);
        let total_pad = max_arg_len.saturating_sub(6);
        let pad_left = "&nbsp;".repeat(total_pad / 2);
        let pad_right = "&nbsp;".repeat(total_pad - total_pad / 2);
        let _ = writeln!(md, "| {pad_left}Argument{pad_right} | Description |");
        md.push_str("|----------|-------------|\n");
        for arg in &parsed_args {
            let _ = writeln!(
                md,
                "| &nbsp;`{}`&nbsp; | {} |",
                arg.name,
                escape_table_cell(&linkify_bare_urls(&arg.description))
            );
        }
        md.push('\n');
    }

    // Options sections
    for (section_title, options) in &options_sections {
        if options.is_empty() {
            continue;
        }
        write_heading(&mut md, section_title);
        // Pad header to the longest long flag to prevent word-wrap.
        // Minimum width of 14 (length of "--no-headers") ensures even sections
        // with only short flags like --jobs/--batch don't word-wrap on hyphens.
        let max_flag_len = options
            .iter()
            .map(|o| o.flag.len())
            .max()
            .unwrap_or(0)
            .max(14);
        let total_pad = max_flag_len.saturating_sub(4);
        let pad_left = "&nbsp;".repeat(total_pad / 2);
        let pad_right = "&nbsp;".repeat(total_pad - total_pad / 2);
        let _ = writeln!(
            md,
            "| {pad_left}Option{pad_right} | Type | Description | Default |"
        );
        md.push_str("|--------|------|-------------|--------|\n");
        for opt in options {
            let option_display = if let Some(short) = &opt.short {
                format!("&nbsp;`{short},`<br>`{}`&nbsp;", opt.flag)
            } else {
                format!("&nbsp;`{}`&nbsp;", opt.flag)
            };
            let default_str = opt
                .default
                .as_ref()
                .map_or(String::new(), |d| format!("`{d}`"));
            let _ = writeln!(
                md,
                "| {} | {} | {} | {} |",
                option_display,
                opt.option_type,
                escape_table_cell(&linkify_bare_urls(&opt.description)),
                default_str
            );
        }
        md.push('\n');
    }

    // Footer
    md.push_str("---\n");
    let _ = write!(
        md,
        "**Source:** [`{source_path}`]({source_url})\n| **[Table of \
         Contents](TableOfContents.md)** | **[README](../../README.md)**\n"
    );

    md
}

/// Escape pipe characters and newlines for markdown table cells
fn escape_table_cell(text: &str) -> String {
    text.replace('|', "\\|")
        .replace('\n', " ")
        .replace('\r', "")
}

/// Convert bare URLs (https://...) in text to markdown autolinks (<https://...>).
/// Skips URLs that are already inside markdown links `[text](url)` or autolinks `<url>`.
fn linkify_bare_urls(text: &str) -> String {
    // Match URLs not preceded by ]( (markdown link) or < (autolink).
    // URLs may be surrounded by parentheses like (https://example.com) — we handle that.
    let url_re = regex_oncelock!(r"(^|[^<])(https?://[^\s>\]]+)");
    url_re
        .replace_all(text, |caps: &regex::Captures| {
            let prefix = &caps[1];
            // Skip if this is inside a markdown link: ](url)
            if prefix.ends_with("](") {
                return caps[0].to_string();
            }
            let mut url: &str = &caps[2];
            // Strip trailing punctuation that isn't part of the URL
            let mut suffix = String::new();
            while url.ends_with(['.', ',', ';', ':']) {
                suffix.insert(0, url.as_bytes()[url.len() - 1] as char);
                url = &url[..url.len() - 1];
            }
            // Handle trailing ) that closes a surrounding (...) but isn't part of the URL
            // by checking balanced parens
            if url.ends_with(')') && !url_has_balanced_parens(url) {
                suffix.insert(0, ')');
                url = &url[..url.len() - 1];
            }
            format!("{prefix}<{url}>{suffix}")
        })
        .to_string()
}

/// Check if a URL has balanced parentheses (for URLs like Wikipedia that contain parens)
fn url_has_balanced_parens(url: &str) -> bool {
    let mut depth: i32 = 0;
    for c in url.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {},
        }
        if depth < 0 {
            return false;
        }
    }
    depth == 0
}

/// Sections parsed from USAGE text
struct UsageSections {
    description:    Vec<String>,
    examples:       Vec<String>,
    usage_patterns: Vec<String>,
    arguments_text: Vec<String>,
    option_groups:  Vec<(String, Vec<String>)>, // (group_name, lines)
}

/// Parse USAGE text into distinct sections using a state machine
fn parse_usage_sections(usage_text: &str) -> UsageSections {
    #[derive(PartialEq)]
    enum State {
        Description,
        Examples,
        UsagePatterns,
        Arguments,
        Options,
    }

    let lines: Vec<&str> = usage_text.lines().collect();

    let mut description = Vec::new();
    let mut examples = Vec::new();
    let mut usage_patterns = Vec::new();
    let mut arguments_text = Vec::new();
    let mut option_groups: Vec<(String, Vec<String>)> = Vec::new();

    let mut state = State::Description;
    let mut current_option_group_name = String::new();
    let mut current_option_lines: Vec<String> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Detect section transitions
        if trimmed.starts_with("Examples:") || trimmed.starts_with("Examples (") {
            state = State::Examples;
            continue;
        }
        if trimmed.starts_with("Usage:") {
            // Finalize any pending option group
            if !current_option_lines.is_empty() {
                option_groups.push((current_option_group_name.clone(), current_option_lines));
                current_option_lines = Vec::new();
                current_option_group_name.clear();
            }
            state = State::UsagePatterns;
            continue;
        }

        // Detect arguments/options sections that come after Usage: or Examples:
        // Some commands (e.g. transpose) have Examples before options sections,
        // so we need to detect options/arguments headers from the Examples state too.
        if state == State::UsagePatterns
            || state == State::Arguments
            || state == State::Options
            || state == State::Examples
        {
            // Check if this is an arguments header
            if (trimmed.ends_with("arguments:")
                || trimmed.ends_with("argument:")
                || trimmed.ends_with("args:"))
                && !trimmed.starts_with('-')
            {
                state = State::Arguments;
                continue;
            }

            // Check for options group headers - patterns like:
            // "command options:", "Common options:", "WIDTH OPTIONS:", etc.
            if trimmed.ends_with("options:") || trimmed.ends_with("option:") {
                // Finalize previous option group
                if !current_option_lines.is_empty() {
                    option_groups.push((current_option_group_name.clone(), current_option_lines));
                    current_option_lines = Vec::new();
                }
                current_option_group_name = trimmed.trim_end_matches(':').to_string();
                state = State::Options;
                continue;
            }

            // Check for ALL-CAPS header lines within options section (like "WHEN THE POLARS
            // FEATURE IS ENABLED:")
            if state == State::Options
                && trimmed.ends_with(':')
                && trimmed.len() > 3
                && trimmed[..trimmed.len() - 1].chars().all(|c| {
                    c.is_uppercase()
                        || c.is_whitespace()
                        || c == '_'
                        || c == '-'
                        || c == '/'
                        || c == '&'
                })
            {
                // Finalize previous option group
                if !current_option_lines.is_empty() {
                    option_groups.push((current_option_group_name.clone(), current_option_lines));
                    current_option_lines = Vec::new();
                }
                current_option_group_name = trimmed.trim_end_matches(':').to_string();
                continue;
            }
        }

        match state {
            State::Description => {
                description.push(line.to_string());
            },
            State::Examples => {
                // Stop examples if we hit Usage: (already handled above)
                examples.push(line.to_string());
            },
            State::UsagePatterns => {
                if trimmed.is_empty() {
                    // Empty line might indicate end of usage patterns
                    // But only if we already have some patterns
                    if !usage_patterns.is_empty() {
                        // Peek ahead to see if next non-empty line starts a new section
                        let next_nonempty = lines[i + 1..]
                            .iter()
                            .find(|l| !l.trim().is_empty())
                            .map(|l| l.trim());
                        if let Some(next) = next_nonempty
                            && !next.starts_with("qsv ")
                        {
                            // Not more usage patterns, transition to waiting for args/opts
                            continue;
                        }
                    }
                } else if trimmed.starts_with("qsv ") {
                    usage_patterns.push(trimmed.to_string());
                }
            },
            State::Arguments => {
                arguments_text.push(line.to_string());
            },
            State::Options => {
                current_option_lines.push(line.to_string());
            },
        }
    }

    // Finalize last option group
    if !current_option_lines.is_empty() {
        option_groups.push((current_option_group_name, current_option_lines));
    }

    UsageSections {
        description,
        examples,
        usage_patterns,
        arguments_text,
        option_groups,
    }
}

/// Format description lines into markdown
fn format_description(lines: &[String]) -> String {
    let mut md = String::new();
    let mut in_code_block = false;
    let mut prev_empty = false;
    let mut prev_was_heading = false;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Skip empty leading lines
        if trimmed.is_empty() && md.is_empty() {
            continue;
        }

        // Check for === underline (header marker) - skip the underline itself
        if trimmed.chars().all(|c| c == '=') && !trimmed.is_empty() {
            continue;
        }

        // Check for --- separator lines
        if trimmed == "---" || (trimmed.chars().all(|c| c == '-') && trimmed.len() >= 3) {
            if in_code_block {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            md.push_str("---\n\n");
            prev_empty = true;
            continue;
        }

        // Check if next line is === underline (means this line is a heading)
        let next_is_underline = lines.get(i + 1).is_some_and(|next| {
            let nt = next.trim();
            !nt.is_empty() && nt.chars().all(|c| c == '=')
        });

        // ALL-CAPS lines are headings (including those with === underlines)
        if next_is_underline
            || (trimmed.len() > 2
                && trimmed.chars().all(|c| {
                    c.is_uppercase()
                        || c.is_whitespace()
                        || c == '('
                        || c == ')'
                        || c == '-'
                        || c == '_'
                        || c == '/'
                        || c == '&'
                }))
        {
            if in_code_block {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            let _ = write!(md, "### {}\n\n", titlecase_heading(trimmed));
            prev_empty = true;
            prev_was_heading = true;
            continue;
        }

        if prev_was_heading {
            prev_was_heading = false;
        }

        // Handle inline code examples: lines starting with $ qsv or qsv
        if trimmed.starts_with("$ qsv") || (trimmed.starts_with("qsv ") && !trimmed.contains("is "))
        {
            if !in_code_block {
                md.push_str("```console\n");
                in_code_block = true;
            }
            md.push_str(trimmed);
            md.push('\n');
            // Handle continuation lines
            if !trimmed.ends_with('\\') {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            prev_empty = false;
            continue;
        }

        // Handle continuation of code blocks
        if in_code_block {
            md.push_str(trimmed);
            md.push('\n');
            if !trimmed.ends_with('\\') {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            prev_empty = false;
            continue;
        }

        if trimmed.is_empty() {
            if !prev_empty {
                md.push('\n');
                prev_empty = true;
            }
            continue;
        }

        // Bullet list items
        if trimmed.starts_with("* ") || trimmed.starts_with("- ") {
            md.push_str(&linkify_bare_urls(trimmed));
            md.push('\n');
            prev_empty = false;
            continue;
        }

        // Numbered list items
        if trimmed.chars().next().is_some_and(|c| c.is_ascii_digit()) && trimmed.contains(". ") {
            md.push_str(&linkify_bare_urls(trimmed));
            md.push('\n');
            prev_empty = false;
            continue;
        }

        // Regular paragraph text
        md.push_str(&linkify_bare_urls(trimmed));
        md.push('\n');
        prev_empty = false;
    }

    if in_code_block {
        md.push_str("```\n");
    }

    md
}

/// Known acronyms that should be preserved as all-uppercase in title-cased headings
const ACRONYMS: &[&str] = &[
    "API", "CKAN", "CSV", "CV", "HTTP", "HTTPS", "ID", "IP", "IPC", "IQR", "JSON", "JSONL", "LLM",
    "NLP", "ODS", "RAG", "SEM", "SQL", "SSV", "TOML", "TOON", "TSV", "URL", "UUID", "XLSX",
];

/// Title-case a single part (word fragment), preserving known acronyms
fn titlecase_part(part: &str) -> String {
    let upper = part.to_uppercase();
    if ACRONYMS.contains(&upper.as_str()) {
        return upper;
    }
    let lower = part.to_lowercase();
    let mut chars = lower.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Convert ALL-CAPS heading to title case, preserving known acronyms
/// and handling `/`-separated parts (e.g. "ANALYSIS/INFERENCING" → "Analysis/Inferencing")
fn titlecase_heading(s: &str) -> String {
    let s = s.trim();
    let words: Vec<&str> = s.split_whitespace().collect();
    words
        .iter()
        .map(|w| {
            if w.contains('/') {
                w.split('/')
                    .map(titlecase_part)
                    .collect::<Vec<_>>()
                    .join("/")
            } else {
                titlecase_part(w)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Format examples section into markdown
fn format_examples(lines: &[String]) -> String {
    let mut md = String::new();
    let mut in_code_block = false;
    let mut skip_next = false;

    for (idx, line) in lines.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

        let trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            if in_code_block {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            continue;
        }

        // Skip bare === underlines (must come before == HEADING == check since
        // "======" matches both patterns)
        if trimmed.chars().all(|c| c == '=') {
            continue;
        }

        // Check if this line + next line form an underlined heading (HEADING\n======)
        let next_is_underline = lines.get(idx + 1).is_some_and(|next| {
            let nt = next.trim();
            !nt.is_empty() && nt.chars().all(|c| c == '=')
        });

        if next_is_underline {
            if in_code_block {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            let _ = write!(md, "### {}\n\n", titlecase_heading(trimmed));
            skip_next = true; // skip the === underline
            continue;
        }

        // ALL-CAPS lines are headings
        if trimmed.len() > 2
            && trimmed.chars().all(|c| {
                c.is_uppercase()
                    || c.is_whitespace()
                    || c == '('
                    || c == ')'
                    || c == '-'
                    || c == '_'
                    || c == '/'
                    || c == '&'
            })
        {
            if in_code_block {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            let _ = write!(md, "### {}\n\n", titlecase_heading(trimmed));
            continue;
        }

        // Section headers: == SUBCOMMAND ==
        if trimmed.starts_with("==") && trimmed.ends_with("==") {
            if in_code_block {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            let heading = trimmed.trim_start_matches('=').trim_end_matches('=').trim();
            if !heading.is_empty() {
                let _ = write!(md, "### {heading}\n\n");
            }
            continue;
        }

        // "For more examples, see ..." or "For examples, see ..." or "For more extensive
        // examples, see ..."
        if trimmed.starts_with("For more examples, see")
            || trimmed.starts_with("For examples, see")
            || trimmed.starts_with("For more extensive examples, see")
        {
            if in_code_block {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            // Convert URL to markdown link if present
            if let Some(url_start) = trimmed.find("https://") {
                let url = trimmed[url_start..].trim_end_matches('.');
                let _ = write!(md, "For more examples, see [tests]({url}).\n\n");
            } else {
                md.push_str(trimmed);
                md.push_str("\n\n");
            }
            continue;
        }

        // End marker
        if trimmed.starts_with("end Examples") {
            if in_code_block {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            break;
        }

        // Comment lines: # description
        if trimmed.starts_with('#') {
            if in_code_block {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            let comment = trimmed.trim_start_matches('#').trim();
            let _ = write!(md, "> {}\n\n", linkify_bare_urls(comment));
            continue;
        }

        // Command lines: $ qsv ..., qsv ..., or piped commands containing qsv
        // (e.g. "cat in.csv | qsv split ..." or "$ cat in.csv | qsv split ...")
        if trimmed.starts_with("$ qsv")
            || trimmed.starts_with("qsv ")
            || (trimmed.contains("| qsv ") || trimmed.contains("|qsv "))
            || (trimmed.starts_with("$ ") && trimmed.contains("qsv "))
        {
            if !in_code_block {
                md.push_str("```console\n");
                in_code_block = true;
            }
            // Remove leading "$ " if present
            let cmd = trimmed.strip_prefix("$ ").unwrap_or(trimmed);
            md.push_str(cmd);
            md.push('\n');
            // If no continuation, close the code block
            if !trimmed.ends_with('\\') {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            continue;
        }

        // Continuation lines (after a \ line)
        if in_code_block {
            md.push_str(trimmed);
            md.push('\n');
            if !trimmed.ends_with('\\') {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            continue;
        }

        // Any other text (description paragraphs within examples)
        md.push_str(&linkify_bare_urls(trimmed));
        md.push('\n');
    }

    if in_code_block {
        md.push_str("```\n\n");
    }

    md
}

/// Parse the arguments section text into structured arguments
fn parse_arguments_section(lines: &[String]) -> Vec<ParsedArgument> {
    let mut args = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();

        // Skip empty lines and section headers
        if trimmed.is_empty() {
            i += 1;
            continue;
        }

        // Look for argument patterns: <arg>  description
        // or named argument lines with descriptions
        if trimmed.starts_with('<')
            && let Some(close_bracket) = trimmed.find('>')
        {
            let arg_name = &trimmed[..=close_bracket];
            let desc_start = &trimmed[close_bracket + 1..].trim();
            let mut description = desc_start.to_string();

            // Collect multi-line description
            let mut j = i + 1;
            while j < lines.len() {
                let next = lines[j].trim();
                if next.is_empty()
                    || next.starts_with('<')
                    || next.starts_with('-')
                    || next.ends_with(':')
                {
                    break;
                }
                if !description.is_empty() {
                    description.push(' ');
                }
                description.push_str(next);
                j += 1;
            }

            args.push(ParsedArgument {
                name:        arg_name.to_string(),
                description: description.trim().to_string(),
            });
            i = j;
            continue;
        }

        // Look for subcommand description blocks like "OPERATIONS subcommand:"
        // These are informational - include as structured text
        if trimmed.contains("subcommand:") || trimmed.contains("subcommand") {
            // This is descriptive text about arguments, skip for the table
            i += 1;
            continue;
        }

        i += 1;
    }

    args
}

/// Extract flag descriptions from USAGE text manually
fn extract_descriptions_from_text(usage_text: &str) -> HashMap<String, String> {
    let mut descriptions = HashMap::new();
    let lines: Vec<&str> = usage_text.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Look for option lines: "    -s, --select <arg>    Description"
        if trimmed.starts_with('-') {
            if let Some((flags_part, desc_part)) = trimmed.split_once("  ") {
                let mut description = desc_part.trim().to_string();

                // Collect multi-line description
                let mut j = i + 1;
                while j < lines.len() {
                    let next_line = lines[j].trim();
                    if next_line.is_empty() || next_line.starts_with('-') {
                        break;
                    }
                    if !next_line.starts_with("Usage:") && !next_line.ends_with(':') {
                        description.push(' ');
                        description.push_str(next_line);
                    } else {
                        break;
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
                if next_line.is_empty() || next_line.starts_with('<') || next_line.starts_with('-')
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

/// Extract a default value from a description string
fn extract_default_value(description: &str) -> Option<String> {
    if let Some(start) = description.find("[default:")
        && let Some(end) = description[start..].find(']')
    {
        let default_str = &description[start + 9..start + end];
        return Some(default_str.trim().to_string());
    }
    None
}

/// Remove [default: value] text from description
fn strip_default_from_description(description: &str) -> String {
    if let Some(start) = description.find("[default:")
        && let Some(end) = description[start..].find(']')
    {
        let before = description[..start].trim();
        let after = description[start + end + 1..].trim();

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

/// Parse options using docopt for structure and manual text for descriptions.
/// Returns a vec of (section_title, options) pairs.
fn parse_options_with_docopt(
    usage_text: &str,
    sections: &UsageSections,
    command_name: &str,
) -> Vec<(String, Vec<ParsedOption>)> {
    // First, try to get structured info from docopt
    let docopt_info = Parser::new(usage_text).ok();

    // Get manual descriptions
    let _manual_descriptions = extract_descriptions_from_text(usage_text);

    // Build a map of flag -> docopt info (type, default, short/long pairing)
    let mut docopt_map: HashMap<String, (String, Option<String>, Option<String>)> = HashMap::new();
    // (option_type, default, paired_short_or_long)

    if let Some(ref parser) = docopt_info {
        for (atom, opts) in parser.descs.iter() {
            match atom {
                Atom::Short(c) => {
                    let flag_str = format!("-{c}");
                    let long_flag = parser
                        .descs
                        .iter()
                        .find(|(a, o)| {
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

                    let option_type = match &opts.arg {
                        DocoptArgument::Zero => "flag".to_string(),
                        DocoptArgument::One(_) => "string".to_string(),
                    };
                    let default = match &opts.arg {
                        DocoptArgument::One(Some(d)) => Some(d.clone()),
                        _ => None,
                    };

                    docopt_map.insert(flag_str.clone(), (option_type, default, long_flag.clone()));
                    if let Some(ref long) = long_flag {
                        docopt_map.insert(
                            long.clone(),
                            (
                                docopt_map
                                    .get(&flag_str)
                                    .map_or_else(|| "string".to_string(), |v| v.0.clone()),
                                docopt_map.get(&flag_str).and_then(|v| v.1.clone()),
                                Some(flag_str),
                            ),
                        );
                    }
                },
                Atom::Long(name) => {
                    let flag_str = format!("--{name}");
                    if docopt_map.contains_key(&flag_str) {
                        continue;
                    }

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
                        DocoptArgument::Zero => "flag".to_string(),
                        DocoptArgument::One(_) => "string".to_string(),
                    };
                    let default = match &opts.arg {
                        DocoptArgument::One(Some(d)) => Some(d.clone()),
                        _ => None,
                    };

                    docopt_map.insert(flag_str.clone(), (option_type, default, short_flag.clone()));
                    if let Some(ref short) = short_flag
                        && !docopt_map.contains_key(short)
                    {
                        docopt_map.insert(
                            short.clone(),
                            (
                                docopt_map
                                    .get(&flag_str)
                                    .map_or_else(|| "string".to_string(), |v| v.0.clone()),
                                docopt_map.get(&flag_str).and_then(|v| v.1.clone()),
                                Some(flag_str),
                            ),
                        );
                    }
                },
                _ => {},
            }
        }
    }

    // Now process each option group from the sections
    let mut result = Vec::new();

    for (group_name, lines) in &sections.option_groups {
        let section_title = format_option_group_title(group_name, command_name);
        let mut options = Vec::new();
        let mut seen_flags: Vec<String> = Vec::new();

        let mut i = 0;
        while i < lines.len() {
            let trimmed = lines[i].trim();

            if trimmed.is_empty() {
                i += 1;
                continue;
            }

            // ALL-CAPS subsection headers within option groups
            // e.g., "WIDTH OPTIONS:", "WHEN THE POLARS FEATURE IS ENABLED:"
            // Already handled at the section level by parse_usage_sections

            // Option line starts with -
            if trimmed.starts_with('-')
                && let Some(parsed) = parse_option_line(trimmed, &lines[i + 1..], &docopt_map)
            {
                // Skip if we've already seen this flag (from docopt pairing)
                let primary = parsed.flag.clone();
                if !seen_flags.contains(&primary) {
                    if let Some(ref short) = parsed.short {
                        seen_flags.push(short.clone());
                    }
                    seen_flags.push(primary);
                    options.push(parsed);
                }

                // Skip continuation lines
                let mut j = i + 1;
                while j < lines.len() {
                    let next = lines[j].trim();
                    if next.is_empty() || next.starts_with('-') {
                        break;
                    }
                    j += 1;
                }
                i = j;
                continue;
            }

            i += 1;
        }

        if !options.is_empty() {
            result.push((section_title, options));
        }
    }

    result
}

/// Parse a single option line and its continuation lines
fn parse_option_line(
    first_line: &str,
    remaining_lines: &[String],
    docopt_map: &HashMap<String, (String, Option<String>, Option<String>)>,
) -> Option<ParsedOption> {
    let trimmed = first_line.trim();
    if !trimmed.starts_with('-') {
        return None;
    }

    // Split into flags part and description part
    let (flags_part, desc_part) = if let Some((f, d)) = trimmed.split_once("  ") {
        (f.trim(), d.trim())
    } else {
        // Single-word flag with no description on same line
        (trimmed, "")
    };

    // Parse flags: "-s, --select <arg>" or "--flag" or "-f, --flag"
    let mut short = None;
    let mut long = None;

    for part in flags_part.split(',') {
        let part = part.trim();
        let flag_name = part.split_whitespace().next().unwrap_or(part);
        if flag_name.starts_with("--") {
            long = Some(flag_name.to_string());
        } else if flag_name.starts_with('-') {
            short = Some(flag_name.to_string());
        }
    }

    // Primary flag is the long one, or short if no long
    let flag = long.or_else(|| short.clone())?;

    // Collect full description
    let mut description = desc_part.to_string();
    for line in remaining_lines {
        let next = line.trim();
        if next.is_empty() || next.starts_with('-') {
            break;
        }
        if next.ends_with(':')
            && next
                .chars()
                .all(|c| c.is_alphabetic() || c.is_whitespace() || c == ':')
        {
            break;
        }
        description.push(' ');
        description.push_str(next);
    }

    // Get type and default from docopt if available
    let (option_type, docopt_default) = docopt_map.get(&flag).map_or_else(
        || {
            // Fallback: infer from the flags_part
            let has_arg = flags_part.contains('<') || flags_part.contains('=');
            let option_type = if has_arg { "string" } else { "flag" };
            (option_type.to_string(), None)
        },
        |(opt_type, default, _)| (opt_type.clone(), default.clone()),
    );

    // Get default from docopt or from description text
    let default = docopt_default.or_else(|| extract_default_value(&description));

    // Strip default from description if we have it separately
    let description = if default.is_some() {
        strip_default_from_description(&description)
    } else {
        description
    };

    Some(ParsedOption {
        flag,
        short,
        option_type,
        description: description.trim().to_string(),
        default,
    })
}

/// Format option group title
fn format_option_group_title(group_name: &str, _command_name: &str) -> String {
    let lower = group_name.to_lowercase();
    if lower.starts_with("common") {
        "Common Options".to_string()
    } else if lower.contains("option") {
        // Already has "option" in it, just titlecase
        titlecase_heading(group_name)
    } else {
        format!("{} Options", titlecase_heading(group_name))
    }
}

/// Generate the Table of Contents markdown file
fn generate_table_of_contents(
    commands: &[CommandInfo],
    repo_root: &Path,
    legend: &[(String, String)],
) -> String {
    let readme_path = repo_root.join("README.md");
    let readme_content = fs::read_to_string(&readme_path).unwrap_or_default();

    let mut md = String::with_capacity(8192);

    md.push_str("# qsv Command Help\n\n");
    md.push_str(
        "> Auto-generated from qsv command USAGE text. See [README](../../README.md) for full \
         documentation.\n\n",
    );

    md.push_str("| Command | Description |\n");
    md.push_str("| --- | --- |\n");

    for cmd in commands {
        let emoji_str = if cmd.emoji_markers.is_empty() {
            String::new()
        } else {
            // Rewrite image paths from docs/images/ to ../images/ since the ToC
            // lives in docs/help/ and needs to reference docs/images/ as a sibling
            let markers = cmd.emoji_markers.replace("docs/images/", "../images/");
            // Wrap emojis with hover tooltips
            let markers = wrap_emojis_with_tooltips(&markers, legend);
            format!("<br>{markers}")
        };
        let _ = writeln!(
            md,
            "| [{}]({}.md){} | {} |",
            cmd.invocation_name,
            cmd.invocation_name,
            emoji_str,
            escape_table_cell(&cmd.description)
        );
    }

    // Add legend
    md.push_str("\n---\n\n");
    md.push_str("### Legend\n\n");

    // Extract legend from README.md
    let legend_start = readme_content.find("<a name=\"legend_deeplink\">");
    if let Some(start) = legend_start {
        let legend_text = &readme_content[start..];
        // Collect legend lines until we hit an empty line or other section.
        // Each entry must be on its own line — use trailing `  ` (two spaces)
        // for markdown line breaks so they don't run together as a paragraph.
        for line in legend_text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                break;
            }
            // Clean up legend lines - strip HTML anchor, preserving any
            // text content inside the <a> tag (e.g. emoji like ✨).
            // Uses rfind('>') to correctly skip past any attributes in the
            // opening <a ...> tag.
            let cleaned = if let Some(close_pos) = trimmed.find("</a>") {
                let before_close = &trimmed[..close_pos];
                let after_close = &trimmed[close_pos + 4..];
                if let Some(open_end) = before_close.rfind('>') {
                    let inner = &before_close[open_end + 1..];
                    format!("{inner}{after_close}")
                } else {
                    after_close.to_string()
                }
            } else {
                trimmed.to_string()
            };
            // Rewrite image paths for the docs/help/ location
            let cleaned = cleaned.replace("docs/images/", "../images/");
            // Rewrite anchor-only links to point to README (these reference README sections).
            // Uses a regex to only match anchor-only links like [text](#section),
            // not full URLs that happen to contain anchors like [text](https://example.com#frag).
            let anchor_only_re = regex_oncelock!(r"\]\(#");
            let cleaned = anchor_only_re
                .replace_all(&cleaned, "](../../README.md#")
                .to_string();
            md.push_str(&cleaned);
            // Preserve markdown line breaks (two trailing spaces + newline)
            md.push_str("  \n");
        }
    } else {
        // Fallback minimal legend
        md.push_str("See [README](../../README.md) for emoji legend.\n");
    }

    md.push_str("\n---\n**[README](../../README.md)**\n");

    md
}

/// Update README.md command table links to point to help files
fn update_readme_links(repo_root: &Path) -> Result<usize, String> {
    let readme_path = repo_root.join("README.md");
    let content =
        fs::read_to_string(&readme_path).map_err(|e| format!("Failed to read README.md: {e}"))?;

    // Replace source links with help file links
    // Match: [name](/src/cmd/file.rs#Lxx) or [name](/src/cmd/file.rs)
    let link_re = regex_oncelock!(r"\[(\w+)\]\(/src/cmd/\w+\.rs(?:#L\d+)?\)");

    let mut count = 0;
    let new_content = link_re
        .replace_all(&content, |caps: &regex::Captures| {
            let name = &caps[1];
            count += 1;
            format!("[{name}](docs/help/{name}.md)")
        })
        .to_string();

    fs::write(&readme_path, new_content).map_err(|e| format!("Failed to write README.md: {e}"))?;

    Ok(count)
}

/// Public function to generate help markdown files.
/// Called via `qsv --generate-help-md` flag.
pub fn generate_help_markdown() -> CliResult<()> {
    // Determine repository root
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
                "Could not find qsv repository root after checking {} parent directories. This \
                 command must be run from within the qsv repository directory.\nOriginal \
                 directory: {}",
                MAX_ITERATIONS,
                original_dir.display()
            );
        }

        if !repo_root.pop() {
            return fail_clierror!(
                "Could not find qsv repository root.\nOriginal directory: {}",
                original_dir.display()
            );
        }
    }

    // Extract commands from README
    let commands = extract_commands_from_readme(&repo_root)
        .map_err(|e| format!("Failed to extract commands from README: {e}"))?;

    // Parse emoji legend from README for hover tooltips
    let readme_path = repo_root.join("README.md");
    let readme_content = fs::read_to_string(&readme_path).unwrap_or_default();
    let legend = parse_legend(&readme_content);

    // Create output directory
    let output_dir = repo_root.join("docs/help");
    fs::create_dir_all(&output_dir)?;

    eprintln!("QSV Help Markdown Generator (via qsv --generate-help-md)");
    eprintln!("===============================================================");
    eprintln!("Repository: {}", repo_root.display());
    eprintln!("Output: {}", output_dir.display());
    eprintln!("Generating {} help files...\n", commands.len());

    let mut success_count = 0;
    let mut error_count = 0;

    for cmd_info in &commands {
        eprint!("Processing: {}", cmd_info.invocation_name);

        // Find command source file
        let cmd_file = repo_root.join(format!("src/cmd/{}.rs", cmd_info.source_file));
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

        // Generate markdown
        let markdown = generate_command_markdown(&usage_text, cmd_info, &repo_root, &legend);

        // Write help file
        let output_file = output_dir.join(format!("{}.md", cmd_info.invocation_name));
        fs::write(&output_file, &markdown)?;

        eprintln!("  ✅ {}", output_file.display());
        success_count += 1;
    }

    eprintln!(
        "\n📊 Summary: {} succeeded, {} failed out of {} total",
        success_count,
        error_count,
        commands.len()
    );

    if error_count > 0 {
        eprintln!("⚠️  Skipping Table of Contents and README link updates due to errors.");
        return fail_clierror!("{} help file(s) failed to generate", error_count);
    }

    // Generate Table of Contents and update README only when all commands succeeded
    let toc = generate_table_of_contents(&commands, &repo_root, &legend);
    let toc_file = output_dir.join("TableOfContents.md");
    fs::write(&toc_file, &toc)?;
    eprintln!("✅ Generated: {}", toc_file.display());

    match update_readme_links(&repo_root) {
        Ok(count) => {
            eprintln!("✅ Updated {count} links in README.md");
        },
        Err(e) => {
            eprintln!("⚠️  Failed to update README links: {e}");
        },
    }

    eprintln!("\n✨ Help Markdown generation complete!");
    eprintln!("📁 Output directory: {}", output_dir.display());

    Ok(())
}
