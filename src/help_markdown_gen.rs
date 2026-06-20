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
/// Returns a Vec of `CommandInfo` with invocation name, source file, description and emojis.
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

/// Check if a line marks the end of the legend section.
/// Used by both `parse_legend()` and `generate_table_of_contents()` to detect
/// definitive end-of-legend markers (headings, HR rules, footnote references).
#[inline]
fn is_legend_end_marker(line: &str) -> bool {
    line.starts_with('#')
        || line.starts_with("---")
        || line.starts_with("___")
        || line.starts_with("***")
        || line.starts_with("[^")
}

/// Parse the legend section from README.md into a vec of (`emoji_key`, description) pairs.
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
    // We collect lines until a definitive end-of-legend marker (heading, HR, or
    // footnote reference), skipping any blank lines within the section.
    let mut joined_lines: Vec<String> = Vec::new();
    for line in legend_text.lines() {
        let trimmed = line.trim();
        // Skip blank lines within the legend section
        if trimmed.is_empty() {
            continue;
        }
        // Stop at a definitive end-of-legend marker: heading, HR rule, or footnote
        if is_legend_end_marker(trimmed) {
            break;
        }
        // Check if this line starts a new entry: <a tag, ![ image, or emoji character.
        // Emoji detection: check if the first character is in a known emoji Unicode range
        // rather than just checking !is_ascii(), which would also match accented chars.
        // U+2100 avoids false positives from General Punctuation (U+2000–U+206F).
        let first_char = trimmed.chars().next().unwrap_or(' ');
        let is_emoji_start = !first_char.is_ascii()
            && (first_char > '\u{2100}'
                || ('\u{00A9}' == first_char)
                || ('\u{00AE}' == first_char));
        let is_new_entry =
            trimmed.starts_with("<a ") || trimmed.starts_with("![") || is_emoji_start;
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
        // Escape characters for safe use in markdown link title attributes (quoted with ")
        let clean_desc = clean_desc.replace('\\', "\\\\").replace('"', "\\\"");
        let clean_desc = clean_desc.trim().to_string();

        if !clean_desc.is_empty() {
            legend.push((key, clean_desc));
        }
    }

    // Sort by key length descending for longest-match-first replacement
    legend.sort_by_key(|b| std::cmp::Reverse(b.0.len()));
    legend
}

/// Wrap emoji markers in a string with tooltip links using the parsed legend.
/// For unicode emojis: `[emoji](legend_link "description")`
/// For image markdown `![name](path)`: `[![name](path)](legend_link "description")`
///
/// Uses a two-pass approach with placeholders to avoid replacing emojis that appear
/// inside already-inserted tooltip descriptions (e.g. 🏎️'s description mentions 📇).
fn wrap_emojis_with_tooltips(
    markers: &str,
    legend: &[(String, String)],
    legend_link: &str,
) -> String {
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
            // Image emoji: wrap with link for tooltip
            // ![name](path) -> [![name](path)](legend_link "description")
            if let Some(caps) = img_re.captures(key) {
                let name = &caps[1];
                let path = &caps[2];
                format!("[![{name}]({path})]({legend_link} \"{desc}\")")
            } else {
                continue;
            }
        } else {
            // Unicode emoji: wrap with markdown link for tooltip
            format!("[{key}]({legend_link} \"{desc}\")")
        };

        // Use a Private Use Area Unicode placeholder that won't appear in normal text
        let idx = replacements.len();
        let placeholder = format!("\u{E000}EMOJI{idx}\u{E000}");
        replacements.push(replacement);
        result = result.replace(key.as_str(), &placeholder);
    }

    // Pass 2: Replace all placeholders with their actual values
    for (idx, replacement) in replacements.iter().enumerate() {
        let placeholder = format!("\u{E000}EMOJI{idx}\u{E000}");
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

/// Extract a map of long-form flag → display-type label by parsing the `Args`
/// struct in a command's source file.
///
/// Returns an empty map if the file can't be read or no `Args` struct is found.
/// Used to populate the "Type" column in the generated Options table — without
/// this, docopt alone only tells us whether an option has an argument, not
/// whether that argument is an integer, float, etc.
fn extract_arg_types_from_file(file_path: &Path) -> HashMap<String, &'static str> {
    match fs::read_to_string(file_path) {
        Ok(content) => extract_arg_types_from_source(&content),
        Err(_) => HashMap::new(),
    }
}

/// Pure-function variant of `extract_arg_types_from_file` that operates on the
/// source text directly. Public to the module so tests can exercise it.
///
/// Strategy: locate `struct Args { ... }`, walk forward tracking brace depth
/// to find the matching close, then per-line capture `flag_NAME: TYPE,` fields
/// and classify the Rust type into one of the display labels {flag, integer,
/// float, string}. `arg_*` / `cmd_*` fields are ignored — they are
/// positional / subcommand fields and don't appear in the Options table.
///
/// Known limitation: field types are matched on a single line. If a future
/// command splits a field's type across lines (`flag_x:\n    Option<…>,`),
/// the partial type will silently fall back to `"string"`. No current qsv
/// command does this; add a single-line type or extend the line-capture
/// regex if/when it happens.
fn extract_arg_types_from_source(source: &str) -> HashMap<String, &'static str> {
    let mut map = HashMap::new();

    // Match `struct Args` with optional `pub`/`pub(crate)` visibility and an
    // optional generic-parameter list (`<'a>`, `<T>`). Anchored to a word
    // boundary so we don't match `ArgsExtra` and friends. Cheap insurance
    // against future drift even though current commands use plain `Args`.
    let struct_re = regex_oncelock!(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?struct\s+Args\b");
    let Some(struct_match) = struct_re.find(source) else {
        return map;
    };

    let Some(brace_off) = source[struct_match.end()..].find('{') else {
        return map;
    };
    let body_start = struct_match.end() + brace_off + 1;

    // Walk forward tracking brace depth to find the matching `}`. This handles
    // nested generics like `Option<Vec<Foo>>` and any inline blocks robustly.
    let bytes = source.as_bytes();
    let mut depth: usize = 1;
    let mut body_end: Option<usize> = None;
    let mut i = body_start;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    body_end = Some(i);
                    break;
                }
            },
            _ => {},
        }
        i += 1;
    }
    let Some(body_end) = body_end else {
        return map;
    };

    let body = &source[body_start..body_end];

    // Allow an optional visibility modifier (`pub`, `pub(crate)`, etc.) before
    // the field name — stats.rs and a few others mark Args fields `pub`.
    let re = regex_oncelock!(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?flag_([a-z0-9_]+)\s*:\s*(.+)$");

    for caps in re.captures_iter(body) {
        let field = &caps[1];
        let mut raw_type = caps[2].to_string();
        // Strip trailing inline comment, if any.
        if let Some(c_idx) = raw_type.find("//") {
            raw_type.truncate(c_idx);
        }
        let type_str = raw_type.trim_end().trim_end_matches(',').trim();
        if type_str.is_empty() {
            continue;
        }

        let mut flag = String::with_capacity(field.len() + 2);
        flag.push_str("--");
        flag.push_str(&field.replace('_', "-"));

        map.insert(flag, classify_rust_type(type_str));
    }

    map
}

/// Map a Rust type expression (after stripping `Option<>` / `Box<>` wrappers)
/// to one of the minimal display labels: `flag`, `integer`, `float`, or
/// `string`. Anything unrecognized (custom types, `String`, `Vec<…>`, paths,
/// etc.) falls back to `string`. `bool` maps to `flag` because docopt-deserialized
/// `bool` fields are always argument-less switches.
fn classify_rust_type(rust_type: &str) -> &'static str {
    let mut t = rust_type.trim();
    // Repeatedly unwrap `Option<…>` and `Box<…>`.
    loop {
        let unwrapped = if let Some(inner) = t.strip_prefix("Option<") {
            inner.strip_suffix('>').map(str::trim)
        } else if let Some(inner) = t.strip_prefix("Box<") {
            inner.strip_suffix('>').map(str::trim)
        } else {
            None
        };
        match unwrapped {
            Some(next) => t = next,
            None => break,
        }
    }

    match t {
        "bool" => "flag",
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64" | "i128"
        | "isize" => "integer",
        "f32" | "f64" => "float",
        _ => "string",
    }
}

/// Parsed option from docopt + manual description extraction
struct ParsedOption {
    flag:        String,
    short:       Option<String>,
    option_type: String,
    required:    bool,
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
    repo_root: &Path,
    legend: &[(String, String)],
    arg_type_map: &HashMap<String, &'static str>,
) -> String {
    let mut md = String::with_capacity(4096);

    // Support both `src/cmd/<name>.rs` and module-dir `src/cmd/<name>/mod.rs`.
    let flat_path = format!("src/cmd/{}.rs", cmd_info.source_file);
    let source_path = if repo_root.join(&flat_path).exists() {
        flat_path
    } else {
        format!("src/cmd/{}/mod.rs", cmd_info.source_file)
    };
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
        let markers = wrap_emojis_with_tooltips(&markers, legend, "TableOfContents.md#legend");
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
    let options_sections = parse_options_with_docopt(
        usage_text,
        &sections,
        &cmd_info.invocation_name,
        arg_type_map,
    );

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
            // Use non-breaking hyphen (U+2011) so flags don't word-wrap on hyphens
            let nb_flag = opt.flag.replace('-', "\u{2011}");
            let option_display = if let Some(short) = &opt.short {
                let nb_short = short.replace('-', "\u{2011}");
                format!("&nbsp;`{nb_short},`<br>`{nb_flag}`&nbsp;")
            } else {
                format!("&nbsp;`{nb_flag}`&nbsp;")
            };
            let default_str = opt
                .default
                .as_ref()
                .map_or(String::new(), |d| format!("`{d}`"));
            let description = if opt.required {
                let desc = opt.description.trim_end();
                if desc.is_empty() {
                    "**(required)**".to_string()
                } else {
                    format!("{desc} **(required)**")
                }
            } else {
                opt.description.clone()
            };
            let _ = writeln!(
                md,
                "| {} | {} | {} | {} |",
                option_display,
                opt.option_type,
                escape_table_cell(&linkify_bare_urls(&description)),
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

        // Detect section transitions. Only allow entering Examples state from
        // Description/UsagePatterns — once we're inside Arguments/Options, an
        // option's help text containing "Examples:" or "Example:" must not
        // hijack the parser state and pull subsequent option lines into the
        // examples vec. When already in Examples state, swallow further
        // "Examples:"/"Example:" markers so they aren't emitted as literal
        // text (commands like `to` and `fetch` use multiple sub-Examples).
        let is_examples_marker = trimmed.starts_with("Examples:")
            || trimmed.starts_with("Examples (")
            || trimmed.starts_with("Example:")
            || trimmed.starts_with("Example (");
        // In Arguments/Options: leave state alone; line falls through to the
        // per-state arm below and is captured with the rest of the
        // option/argument text.
        if is_examples_marker
            && (state == State::Description
                || state == State::UsagePatterns
                || state == State::Examples)
        {
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

/// Handle explicit triple-backtick fenced-block start/continuation/end. Returns true when
/// the line was consumed (the caller should `continue`). When opening a new
/// fence, closes any active implicit `$ qsv` code block first.
///
/// Shared between `format_description` and `format_examples` so the two stay
/// in sync.
fn handle_fenced_block(
    md: &mut String,
    line: &str,
    trimmed: &str,
    in_code_block: &mut bool,
    in_fenced_block: &mut bool,
) -> bool {
    // Inside an explicit ``` fenced code block — preserve original whitespace.
    if *in_fenced_block {
        md.push_str(line);
        md.push('\n');
        if trimmed.starts_with("```") {
            *in_fenced_block = false;
            md.push('\n');
        }
        return true;
    }

    // Opening of an explicit ``` fenced code block.
    if trimmed.starts_with("```") {
        if *in_code_block {
            md.push_str("```\n\n");
            *in_code_block = false;
        }
        md.push_str(line);
        md.push('\n');
        *in_fenced_block = true;
        return true;
    }

    false
}

/// Append a markdown hard-line-break (two trailing spaces) when the trimmed
/// line ends in `:` (other than `Examples:` / `Example:`), so the following
/// line renders on its own row.
fn maybe_append_colon_break(md: &mut String, trimmed: &str) {
    if trimmed.ends_with(':')
        && !trimmed.starts_with("Examples:")
        && !trimmed.starts_with("Example:")
    {
        md.push_str("  ");
    }
}

/// Leading ASCII-space count of a raw (untrimmed) line. Counts spaces only (not
/// tabs) since docopt USAGE blocks indent with spaces.
fn leading_spaces(line: &str) -> usize {
    line.len() - line.trim_start_matches(' ').len()
}

/// Emit a contiguous run of >=4-space-indented description lines (starting at
/// `start`) as a verbatim ```` ```text ```` fenced code block, preserving column
/// alignment. The run extends while lines are indented >=4 or blank (interior
/// blanks allowed); trailing blanks are trimmed off. Lines are dedented by the
/// run's minimum indent so the block isn't over-indented. Returns the index of
/// the first line AFTER the consumed run (the loop's resume point).
fn emit_indented_block(md: &mut String, lines: &[String], start: usize) -> usize {
    // Find the end of the run. A literal fence marker ends the run so we never
    // nest a ```` ``` ```` inside the ```` ```text ```` block we emit.
    let mut end = start;
    while end < lines.len() {
        let l = &lines[end];
        if l.trim().starts_with("```") {
            break;
        }
        if l.trim().is_empty() || leading_spaces(l) >= 4 {
            end += 1;
        } else {
            break;
        }
    }
    // Trim trailing blank lines off the run.
    let mut run_end = end;
    while run_end > start && lines[run_end - 1].trim().is_empty() {
        run_end -= 1;
    }

    let min_indent = lines[start..run_end]
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| leading_spaces(l))
        .min()
        .unwrap_or(0);

    // If the preceding heading line got a colon hard-break (`:  \n`), drop the two
    // trailing spaces so it sits cleanly above the fence.
    if md.ends_with(":  \n") {
        md.truncate(md.len() - 3);
        md.push('\n');
    }
    if !md.ends_with("\n\n") && !md.is_empty() {
        md.push('\n');
    }

    md.push_str("```text\n");
    for line in &lines[start..run_end] {
        if line.trim().is_empty() {
            md.push('\n');
        } else {
            md.push_str(&line[min_indent..]);
            md.push('\n');
        }
    }
    md.push_str("```\n\n");

    end
}

/// If a trimmed line begins with a `NOTE:`, `WARNING:`, or `IMPORTANT:` marker,
/// return the corresponding GitHub Alert keyword and the remaining text after
/// the marker (trimmed). Used to render these admonitions as GitHub Alerts:
/// <https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#alerts>
fn match_alert_marker(trimmed: &str) -> Option<(&'static str, &str)> {
    const MARKERS: [(&str, &str); 3] = [
        ("NOTE:", "NOTE"),
        ("WARNING:", "WARNING"),
        ("IMPORTANT:", "IMPORTANT"),
    ];
    for (marker, kind) in MARKERS {
        if let Some(rest) = trimmed.strip_prefix(marker) {
            return Some((kind, rest.trim()));
        }
    }
    None
}

/// Format description lines into markdown
fn format_description(lines: &[String]) -> String {
    let mut md = String::new();
    let mut in_code_block = false;
    let mut in_fenced_block = false;
    let mut prev_empty = false;
    let mut prev_was_heading = false;
    // Whether we are currently inside a GitHub Alert blockquote (started by a
    // NOTE:/WARNING:/IMPORTANT: marker). Continuation lines are emitted as
    // blockquote lines until the next blank line closes the alert.
    let mut in_alert = false;
    // Whether we are currently inside a bullet/numbered list. Indented
    // continuation lines of a list item must NOT be captured as a code block.
    let mut in_list = false;
    // When an indented block is emitted as a fenced code block, the outer loop
    // resumes here, skipping the lines already consumed.
    let mut skip_until = 0usize;

    for (i, line) in lines.iter().enumerate() {
        if i < skip_until {
            continue;
        }
        let trimmed = line.trim();

        if handle_fenced_block(
            &mut md,
            line,
            trimmed,
            &mut in_code_block,
            &mut in_fenced_block,
        ) {
            prev_empty = false;
            continue;
        }

        // Continuation of a GitHub Alert: keep absorbing non-empty lines as
        // blockquote lines; a blank line ends the alert. This takes precedence
        // over heading/code/list detection so the whole admonition stays in one
        // blockquote.
        if in_alert {
            if trimmed.is_empty() {
                in_alert = false;
                md.push('\n');
                prev_empty = true;
                continue;
            }
            md.push_str("> ");
            md.push_str(&linkify_bare_urls(trimmed));
            md.push('\n');
            prev_empty = false;
            continue;
        }

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
            // A blank line ends any in-progress list.
            in_list = false;
            if !prev_empty {
                md.push('\n');
                prev_empty = true;
            }
            continue;
        }

        // GitHub Alert markers (NOTE:/WARNING:/IMPORTANT:) at the start of a
        // line are rendered as GitHub Alerts. A blank line must precede the
        // blockquote for it to render, so emit one if the previous output isn't
        // already blank.
        if let Some((kind, rest)) = match_alert_marker(trimmed) {
            if !md.is_empty() && !prev_empty {
                md.push('\n');
            }
            let _ = writeln!(md, "> [!{kind}]");
            if !rest.is_empty() {
                md.push_str("> ");
                md.push_str(&linkify_bare_urls(rest));
                md.push('\n');
            }
            in_alert = true;
            prev_empty = false;
            continue;
        }

        // Bullet list items
        if trimmed.starts_with("* ") || trimmed.starts_with("- ") {
            in_list = true;
            md.push_str(&linkify_bare_urls(trimmed));
            md.push('\n');
            prev_empty = false;
            continue;
        }

        // Numbered list items
        if trimmed.chars().next().is_some_and(|c| c.is_ascii_digit()) && trimmed.contains(". ") {
            in_list = true;
            md.push_str(&linkify_bare_urls(trimmed));
            md.push('\n');
            prev_empty = false;
            continue;
        }

        // Indented (>=4 space) pre-formatted block (e.g. aligned definition lists
        // or ascii tables). Render verbatim in a fenced code block to preserve
        // column alignment. Two guards keep this from mangling ordinary prose:
        //   - the block must be introduced by a colon-terminated heading line directly above it
        //     (the docopt convention for "here comes a pre-formatted listing"); this avoids
        //     capturing hanging-indent continuation lines of flush-left sentences, which merely
        //     wrap.
        //   - it is skipped while inside a list, so indented list-item continuation lines stay part
        //     of the list.
        let colon_introduced = i > 0 && lines[i - 1].trim_end().ends_with(':');
        if !in_list && colon_introduced && leading_spaces(line) >= 4 {
            skip_until = emit_indented_block(&mut md, lines, i);
            prev_empty = true;
            continue;
        }

        // Regular paragraph text
        md.push_str(&linkify_bare_urls(trimmed));
        maybe_append_colon_break(&mut md, trimmed);
        md.push('\n');
        prev_empty = false;
    }

    if in_code_block {
        md.push_str("```\n");
    }
    if in_fenced_block {
        md.push_str("```\n");
    }

    md
}

/// Known acronyms that should be preserved as all-uppercase in title-cased headings
const ACRONYMS: &[&str] = &[
    "API", "CKAN", "CSV", "CV", "HTTP", "HTTPS", "ID", "IP", "IPC", "IQR", "JSON", "JSONL", "LLM",
    "MCP", "NLP", "ODS", "RAG", "SEM", "SQL", "SSV", "TOML", "TOON", "TSV", "URL", "UUID", "XLSX",
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
/// and handling `/`- and `-`-separated parts (e.g. "ANALYSIS/INFERENCING" →
/// "Analysis/Inferencing", "URL-COLUMN" → "URL-Column")
fn titlecase_heading(s: &str) -> String {
    let s = s.trim();
    let words: Vec<&str> = s.split_whitespace().collect();
    words
        .iter()
        .map(|w| {
            if w.contains('/') {
                w.split('/')
                    .map(titlecase_hyphenated)
                    .collect::<Vec<_>>()
                    .join("/")
            } else {
                titlecase_hyphenated(w)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Title-case a hyphenated word, recursively title-casing each `-`-separated
/// part so embedded acronyms (e.g. URL, HTTP) are preserved.
fn titlecase_hyphenated(w: &str) -> String {
    if !w.contains('-') {
        return titlecase_part(w);
    }
    w.split('-')
        .map(titlecase_part)
        .collect::<Vec<_>>()
        .join("-")
}

/// Format examples section into markdown
fn format_examples(lines: &[String]) -> String {
    let mut md = String::new();
    let mut in_code_block = false;
    let mut in_fenced_block = false;
    let mut skip_next = false;

    for (idx, line) in lines.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

        let trimmed = line.trim();

        if handle_fenced_block(
            &mut md,
            line,
            trimmed,
            &mut in_code_block,
            &mut in_fenced_block,
        ) {
            continue;
        }

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

        // ALL-CAPS lines are headings (with optional trailing colon, e.g.
        // "USING THE HTTP-HEADER OPTION:" — must be multi-word to avoid matching
        // single-word capitalized values like "URL:").
        let heading_body = trimmed.strip_suffix(':').unwrap_or(trimmed);
        if heading_body.len() > 2
            && heading_body.contains(' ')
            && heading_body.chars().all(|c| {
                c.is_uppercase()
                    || c.is_whitespace()
                    || c == '('
                    || c == ')'
                    || c == '-'
                    || c == '_'
                    || c == '/'
                    || c == '&'
            })
            && heading_body.chars().any(char::is_alphabetic)
        {
            if in_code_block {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            let _ = write!(md, "### {}\n\n", titlecase_heading(heading_body));
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
        // Consecutive comment lines are merged into a single blockquote
        if trimmed.starts_with('#') {
            if in_code_block {
                md.push_str("```\n\n");
                in_code_block = false;
            }
            let comment = trimmed.trim_start_matches('#').trim();
            let _ = writeln!(md, "> {}", linkify_bare_urls(comment));
            // Check if the next non-empty line is also a comment — if not, end the blockquote
            let next_is_comment = lines
                .get(idx + 1..)
                .and_then(|remaining| remaining.iter().find(|l| !l.trim().is_empty()))
                .is_some_and(|l| l.trim().starts_with('#'));
            if !next_is_comment {
                md.push('\n');
            }
            continue;
        }

        // Lines ending with \ that lead to a qsv command (e.g. env var prefixes)
        if trimmed.ends_with('\\') && !in_code_block {
            let reaches_qsv = lines
                .get(idx + 1..)
                .and_then(|remaining| {
                    remaining
                        .iter()
                        .map(|l| l.trim())
                        .find(|nt| !nt.ends_with('\\'))
                })
                .is_some_and(|nt| {
                    nt.starts_with("qsv ")
                        || nt.starts_with("$ qsv")
                        || nt.contains("| qsv ")
                        || nt.contains("|qsv ")
                        || (nt.starts_with("$ ") && nt.contains("qsv "))
                });
            if reaches_qsv {
                md.push_str("```console\n");
                in_code_block = true;
                md.push_str(trimmed);
                md.push('\n');
                continue;
            }
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

        // Literal blockquote lines (e.g. GitHub `> [!NOTE]` admonitions). Emit
        // verbatim, then close the blockquote with a blank line once the next
        // non-empty line is not itself a blockquote line — otherwise that line
        // is absorbed as a lazy continuation of the blockquote (CommonMark),
        // mis-scoping following text into the note.
        if trimmed.starts_with('>') {
            md.push_str(&linkify_bare_urls(trimmed));
            md.push('\n');
            let next_is_quote = lines
                .get(idx + 1..)
                .and_then(|remaining| remaining.iter().find(|l| !l.trim().is_empty()))
                .is_some_and(|l| l.trim().starts_with('>'));
            if !next_is_quote {
                md.push('\n');
            }
            continue;
        }

        // Any other text (description paragraphs within examples)
        md.push_str(&linkify_bare_urls(trimmed));
        maybe_append_colon_break(&mut md, trimmed);
        md.push('\n');
    }

    if in_code_block {
        md.push_str("```\n\n");
    }
    if in_fenced_block {
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
            let desc_start = trimmed[close_bracket + 1..].trim();
            // Strip leading "..." (docopt repeating indicator) from description
            let desc_start = desc_start
                .strip_prefix("...")
                .map_or(desc_start, str::trim_start);
            let mut description = desc_start.to_string();

            // Collect multi-line description
            let mut j = i + 1;
            let mut in_list = false;
            let mut bullet_indent: usize = 0;
            while j < lines.len() {
                let next = lines[j].trim();
                if next.is_empty()
                    || next.starts_with('<')
                    || (next.starts_with('-') && !next.starts_with("- "))
                    || next.ends_with(':')
                {
                    break;
                }
                if !description.is_empty() {
                    if next.starts_with("* ") || next.starts_with("- ") {
                        if !in_list {
                            description.push_str("<ul>");
                            in_list = true;
                            bullet_indent = lines[j].len() - lines[j].trim_start().len();
                        }
                        let item_text = next[2..].trim_end();
                        description.push_str("<li>");
                        description.push_str(item_text);
                        description.push_str("</li>");
                    } else if in_list {
                        let current_indent = lines[j].len() - lines[j].trim_start().len();
                        if current_indent <= bullet_indent {
                            description.push_str("</ul> ");
                            in_list = false;
                            description.push_str(next);
                        } else if description.ends_with("</li>") {
                            description.truncate(description.len() - 5);
                            description.push(' ');
                            let trimmed_next = next.trim_end();
                            description.push_str(trimmed_next);
                            description.push_str("</li>");
                        }
                    } else if next.starts_with('\u{200E}') {
                        description.push_str("<br>");
                        description.push_str(next.trim_start_matches('\u{200E}').trim_start());
                    } else {
                        description.push(' ');
                        description.push_str(next);
                    }
                }
                j += 1;
            }
            if in_list {
                description.push_str("</ul>");
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
/// Returns a vec of (`section_title`, options) pairs.
fn parse_options_with_docopt(
    usage_text: &str,
    sections: &UsageSections,
    command_name: &str,
    arg_type_map: &HashMap<String, &'static str>,
) -> Vec<(String, Vec<ParsedOption>)> {
    // First, try to get structured info from docopt
    let docopt_info = Parser::new(usage_text).ok();

    // Get manual descriptions
    let _manual_descriptions = extract_descriptions_from_text(usage_text);

    // Detect which options are required (appear outside [options]/[...] in the
    // Usage: section).
    let required_options = extract_required_options_from_usage(usage_text);

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
                        DocoptArgument::One(_) => {
                            // Prefer looking up by the paired long flag — that's
                            // the form that matches the Args struct field name.
                            let lookup_key = long_flag.as_deref().unwrap_or(&flag_str);
                            (*arg_type_map.get(lookup_key).unwrap_or(&"string")).to_string()
                        },
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
                        DocoptArgument::One(_) => {
                            // `flag_str` here is already the `--long` form.
                            (*arg_type_map.get(&flag_str).unwrap_or(&"string")).to_string()
                        },
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
                && let Some(mut parsed) =
                    parse_option_line(trimmed, &lines[i + 1..], &docopt_map, arg_type_map)
            {
                // Skip if we've already seen this flag (from docopt pairing)
                let primary = parsed.flag.clone();
                if !seen_flags.contains(&primary) {
                    if let Some(ref short) = parsed.short {
                        seen_flags.push(short.clone());
                    }
                    seen_flags.push(primary);
                    parsed.required = required_options.contains(&parsed.flag)
                        || parsed
                            .short
                            .as_deref()
                            .is_some_and(|s| required_options.contains(s));
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
    arg_type_map: &HashMap<String, &'static str>,
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
    let mut in_list = false;
    let mut bullet_indent: usize = 0;
    for line in remaining_lines {
        let next = line.trim();
        if next.is_empty() || (next.starts_with('-') && !next.starts_with("- ")) {
            break;
        }
        if next.ends_with(':')
            && next
                .chars()
                .all(|c| c.is_alphabetic() || c.is_whitespace() || c == ':')
        {
            break;
        }
        if next.starts_with("* ") || next.starts_with("- ") {
            if !in_list {
                description.push_str("<ul>");
                in_list = true;
                bullet_indent = line.len() - line.trim_start().len();
            }
            let item_text = next[2..].trim_end();
            description.push_str("<li>");
            description.push_str(item_text);
            description.push_str("</li>");
        } else if in_list {
            let current_indent = line.len() - line.trim_start().len();
            if current_indent <= bullet_indent {
                // Line is at or shallower than bullet indent — close the list
                // and treat as post-list text
                description.push_str("</ul> ");
                in_list = false;
                description.push_str(next);
            } else {
                // Continuation line of the current list item — append to last <li>
                // Remove the closing </li> and append the continuation text
                if description.ends_with("</li>") {
                    description.truncate(description.len() - 5);
                    description.push(' ');
                    let trimmed_next = next.trim_end();
                    description.push_str(trimmed_next);
                    description.push_str("</li>");
                }
            }
        } else if next.starts_with('\u{200E}') {
            // U+200E (Left-to-Right Mark) is used to prevent docopt from
            // parsing negative numbers as option flags — treat as a line break
            description.push_str("<br>");
            description.push_str(next.trim_start_matches('\u{200E}').trim_start());
        } else {
            description.push(' ');
            description.push_str(next);
        }
    }
    if in_list {
        description.push_str("</ul>");
    }

    // Get type and default from docopt if available
    let (option_type, docopt_default) = docopt_map.get(&flag).map_or_else(
        || {
            // Fallback: infer from the flags_part. When the option takes an
            // argument, consult the Args-struct-derived `arg_type_map` so
            // numeric options aren't always labeled "string".
            let has_arg = flags_part.contains('<') || flags_part.contains('=');
            let option_type = if has_arg {
                (*arg_type_map.get(&flag).unwrap_or(&"string")).to_string()
            } else {
                "flag".to_string()
            };
            (option_type, None)
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
        required: false,
        description: description.trim().to_string(),
        default,
    })
}

// `extract_required_options_from_usage` lives in `crate::generators_common`.
// Both generators share that implementation so the detection semantics stay
// consistent.
use crate::generators_common::extract_required_options_from_usage;

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
    readme_content: &str,
    legend: &[(String, String)],
) -> String {
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
            let markers = wrap_emojis_with_tooltips(&markers, legend, "#legend");
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
        // Collect legend lines until we hit a definitive end-of-legend marker.
        // Each entry must be on its own line — use trailing `  ` (two spaces)
        // for markdown line breaks so they don't run together as a paragraph.
        for line in legend_text.lines() {
            let trimmed = line.trim();
            // Skip blank lines within the legend section
            if trimmed.is_empty() {
                continue;
            }
            // Stop at a definitive end-of-legend marker
            if is_legend_end_marker(trimmed) {
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
            // Matches `](#` which only appears in anchor-only links like [text](#section),
            // not in full URLs that happen to contain anchors like [text](https://example.com#frag).
            let cleaned = cleaned.replace("](#", "](../../README.md#");
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

    // Read README once and reuse for legend parsing and table of contents
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

        // Find command source file. Support both layouts: `src/cmd/<name>.rs`
        // and module-dir `src/cmd/<name>/mod.rs`.
        let mut cmd_file = repo_root.join(format!("src/cmd/{}.rs", cmd_info.source_file));
        if !cmd_file.exists() {
            cmd_file = repo_root.join(format!("src/cmd/{}/mod.rs", cmd_info.source_file));
        }
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

        // Extract per-flag Rust types from the command's `Args` struct so the
        // generated Options table's "Type" column reflects the real type
        // (integer/float/string/flag) instead of always "string".
        let arg_type_map = extract_arg_types_from_file(&cmd_file);

        // Generate markdown
        let markdown =
            generate_command_markdown(&usage_text, cmd_info, &repo_root, &legend, &arg_type_map);

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
    let toc = generate_table_of_contents(&commands, &readme_content, &legend);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_unicode_emoji_with_tooltip() {
        let legend = vec![("🔣".to_string(), "Unicode emoji".to_string())];
        let result = wrap_emojis_with_tooltips("Has 🔣 here", &legend, "#legend");
        assert_eq!(result, r#"Has [🔣](#legend "Unicode emoji") here"#);
    }

    #[test]
    fn test_wrap_image_emoji_with_tooltip() {
        let legend = vec![(
            "![img](path/to/img.png)".to_string(),
            "Image description".to_string(),
        )];
        let result =
            wrap_emojis_with_tooltips("Has ![img](path/to/img.png) here", &legend, "#legend");
        assert_eq!(
            result,
            r#"Has [![img](path/to/img.png)](#legend "Image description") here"#
        );
    }

    #[test]
    fn test_wrap_multiple_emojis_no_cross_contamination() {
        // 📇's description should not get tooltipped by 🏎️ or vice versa
        let legend = vec![
            ("🏎️".to_string(), "Fast, uses 📇 for speed".to_string()),
            ("📇".to_string(), "Index-accelerated".to_string()),
        ];
        let result = wrap_emojis_with_tooltips("🏎️ 📇", &legend, "#legend");
        assert!(
            result.contains(r#"[🏎️](#legend "Fast, uses 📇 for speed")"#),
            "result was: {result}"
        );
        assert!(
            result.contains(r#"[📇](#legend "Index-accelerated")"#),
            "result was: {result}"
        );
    }

    #[test]
    fn test_wrap_emoji_not_present() {
        let legend = vec![("🔣".to_string(), "Not here".to_string())];
        let result = wrap_emojis_with_tooltips("No emojis", &legend, "#legend");
        assert_eq!(result, "No emojis");
    }

    #[test]
    fn test_wrap_with_full_legend_link() {
        let legend = vec![("📇".to_string(), "Index".to_string())];
        let result = wrap_emojis_with_tooltips("📇", &legend, "TableOfContents.md#legend");
        assert_eq!(result, r#"[📇](TableOfContents.md#legend "Index")"#);
    }

    #[test]
    fn test_desc_with_escaped_quotes_preserves_link_format() {
        // Verify that a description with pre-escaped double quotes (as produced by
        // parse_legend) doesn't break the markdown link syntax when passed through
        // wrap_emojis_with_tooltips.
        let legend = vec![("🔣".to_string(), r#"Has \"quotes\""#.to_string())];
        let result = wrap_emojis_with_tooltips("🔣", &legend, "#legend");
        // The description should be safely embedded in the markdown title
        assert!(
            result.contains("\"Has"),
            "Double quotes should be escaped: {result}"
        );
        // Should not break the markdown link syntax (no unescaped interior quotes)
        assert!(
            result.starts_with("[🔣](#legend \""),
            "Link format should be intact: {result}"
        );
    }

    #[test]
    fn test_parse_legend_escapes_for_markdown() {
        // Build a minimal README snippet with a legend entry containing special chars
        let readme = r#"<a name="legend_deeplink"></a>
🔣: Description with "quotes" & <angles>

## Next Section
"#;
        let legend = parse_legend(readme);
        assert_eq!(legend.len(), 1);
        let (key, desc) = &legend[0];
        assert_eq!(key, "🔣");
        // Double quotes should be escaped for markdown link titles
        assert!(
            desc.contains("\\\""),
            "Double quotes should be backslash-escaped, got: {desc}"
        );
        // HTML entities should NOT appear (we're generating markdown, not HTML)
        assert!(
            !desc.contains("&amp;"),
            "Should not contain HTML entities, got: {desc}"
        );
        assert!(
            !desc.contains("&lt;"),
            "Should not contain HTML entities, got: {desc}"
        );
    }

    #[test]
    fn test_parse_legend_escapes_backslash() {
        // Verify that literal backslashes in descriptions are escaped for markdown titles
        let readme = r#"<a name="legend_deeplink"></a>
🔣: Path is C:\Users\test

## Next Section
"#;
        let legend = parse_legend(readme);
        assert_eq!(legend.len(), 1);
        let (key, desc) = &legend[0];
        assert_eq!(key, "🔣");
        // Backslashes should be doubled for safe embedding in markdown link titles
        assert!(
            desc.contains(r"C:\\Users\\test"),
            "Backslashes should be escaped, got: {desc}"
        );
    }

    #[test]
    fn test_parse_legend_bare_angle_brackets() {
        // When bare < and > pair up like "< b and c >", the html_re regex
        // treats it as a tag and strips it. This documents that current behavior.
        // In practice, legend descriptions don't contain bare angle brackets.
        let readme = r#"<a name="legend_deeplink"></a>
🔣: Values where a < b and c > d

## Next Section
"#;
        let legend = parse_legend(readme);
        assert_eq!(legend.len(), 1);
        let (_key, desc) = &legend[0];
        // "< b and c >" matches html_re's <[^>]+> pattern, so it gets stripped,
        // leaving "Values where a  d"
        assert!(
            desc.contains("Values where a"),
            "Text before the pseudo-tag should be preserved, got: {desc}"
        );
        assert!(
            !desc.contains("< b"),
            "Pseudo-tag '< b and c >' should be stripped by html_re, got: {desc}"
        );
        assert!(
            desc.contains("d"),
            "Text after the pseudo-tag should be preserved, got: {desc}"
        );
    }

    #[test]
    fn test_parse_legend_unpaired_angle_brackets() {
        // Truly unpaired angle brackets (e.g., "Values < 5" or "a > b") don't
        // match html_re's <[^>]+> pattern because they don't pair up, so they
        // pass through intact. This is distinct from the paired case above.
        let readme = r#"<a name="legend_deeplink"></a>
🔣: Values < 5 are small
📊: Scores > 90 are great

## Next Section
"#;
        let legend = parse_legend(readme);
        assert_eq!(legend.len(), 2);

        let (_key, desc) = &legend[0];
        assert!(
            desc.contains("Values < 5 are small"),
            "Unpaired '<' should pass through, got: {desc}"
        );

        let (_key, desc) = &legend[1];
        assert!(
            desc.contains("Scores > 90 are great"),
            "Unpaired '>' should pass through, got: {desc}"
        );
    }

    // --- format_examples tests ---

    fn lines(input: &[&str]) -> Vec<String> {
        input.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_format_examples_env_var_continuation_reaches_qsv() {
        let input = lines(&[
            "# Set engine and run",
            "QSV_DUCKDB_PATH=/path/to/duckdb \\",
            "qsv describegpt data.csv",
        ]);
        let md = format_examples(&input);
        assert!(
            md.contains("```console\nQSV_DUCKDB_PATH=/path/to/duckdb"),
            "Env-var continuation should be inside code block, got:\n{md}"
        );
        assert!(
            md.contains("qsv describegpt data.csv\n```"),
            "qsv command should close the code block, got:\n{md}"
        );
    }

    #[test]
    fn test_format_examples_continuation_not_reaching_qsv() {
        let input = lines(&["FOO=bar \\", "echo hello"]);
        let md = format_examples(&input);
        assert!(
            !md.contains("```console"),
            "Non-qsv continuation should not open a code block, got:\n{md}"
        );
    }

    #[test]
    fn test_format_examples_consecutive_comments() {
        let input = lines(&["# First comment", "# Second comment", "qsv count data.csv"]);
        let md = format_examples(&input);
        // Both comments in a single blockquote (no blank line between them)
        assert!(
            md.contains("> First comment\n> Second comment\n\n"),
            "Consecutive comments should be in one blockquote, got:\n{md}"
        );
    }

    #[test]
    fn test_format_description_note_marker_becomes_github_alert() {
        // A NOTE: marker at the start of a line becomes a GitHub Alert, with
        // continuation lines absorbed into the blockquote until a blank line.
        let input = lines(&[
            "Find the difference between two CSVs.",
            "",
            "NOTE: diff does not support stdin.",
            "      Keys must be unique within each CSV.",
            "",
            "Following prose.",
        ]);
        let md = format_description(&input);
        assert!(
            md.contains(
                "> [!NOTE]\n> diff does not support stdin.\n> Keys must be unique within each \
                 CSV.\n\nFollowing prose."
            ),
            "NOTE: marker should become a GitHub Alert, got:\n{md}"
        );
    }

    #[test]
    fn test_format_description_marker_only_line_alert() {
        // A bare `IMPORTANT:` line (no trailing text) still opens the alert and
        // absorbs the following lines.
        let input = lines(&["IMPORTANT:", "Column order is preserved.", "", "After."]);
        let md = format_description(&input);
        assert!(
            md.contains("> [!IMPORTANT]\n> Column order is preserved.\n\nAfter."),
            "bare IMPORTANT: line should open an alert, got:\n{md}"
        );
    }

    #[test]
    fn test_format_description_warning_marker_alert() {
        let input = lines(&["WARNING: This command can be dangerous.", "", "More text."]);
        let md = format_description(&input);
        assert!(
            md.contains("> [!WARNING]\n> This command can be dangerous.\n\nMore text."),
            "WARNING: marker should become a GitHub Alert, got:\n{md}"
        );
    }

    #[test]
    fn test_format_examples_blockquote_closed_with_blank_line() {
        // A literal `> [!NOTE]` admonition followed by normal prose must be
        // separated by a blank line, otherwise CommonMark absorbs the prose as
        // a lazy continuation of the blockquote (mis-scoping it into the note).
        let input = lines(&[
            "> [!NOTE]",
            "> All variables are strings.",
            "Additional filters are available:",
        ]);
        let md = format_examples(&input);
        assert!(
            md.contains("> All variables are strings.\n\nAdditional filters are available:"),
            "blockquote must be closed with a blank line before following prose, got:\n{md}"
        );
    }

    #[test]
    fn test_format_examples_consecutive_blockquote_lines_not_split() {
        // Adjacent `>` lines stay in one blockquote (no blank line between them).
        let input = lines(&["> line one", "> line two", "after"]);
        let md = format_examples(&input);
        assert!(
            md.contains("> line one\n> line two\n\nafter"),
            "consecutive blockquote lines should not be split, got:\n{md}"
        );
    }

    #[test]
    fn test_format_examples_comment_as_last_line() {
        let input = lines(&["# Trailing comment"]);
        let md = format_examples(&input);
        assert!(
            md.contains("> Trailing comment\n"),
            "Comment as last line should not panic, got:\n{md}"
        );
    }

    #[test]
    fn test_format_examples_backslash_as_last_line() {
        let input = lines(&["FOO=bar \\"]);
        let md = format_examples(&input);
        // Should not panic; treated as plain text since no following qsv line
        assert!(
            !md.contains("```console"),
            "Dangling backslash should not open code block, got:\n{md}"
        );
    }

    #[test]
    fn test_fenced_block_preserves_indentation_in_description() {
        // Explicit ``` fenced blocks must preserve original indentation;
        // trimming would destroy meaningful whitespace inside e.g. Jinja code.
        let input = lines(&[
            "Some prose.",
            "",
            "```jinja",
            "{% if x -%}",
            "    indented body",
            "        deeper",
            "{% endif %}",
            "```",
        ]);
        let md = format_description(&input);
        assert!(
            md.contains("    indented body"),
            "fenced indentation lost (4 spaces), got:\n{md}"
        );
        assert!(
            md.contains("        deeper"),
            "fenced indentation lost (8 spaces), got:\n{md}"
        );
        assert!(md.contains("```jinja"), "opening fence missing, got:\n{md}");
    }

    #[test]
    fn test_fenced_block_in_examples_preserves_indentation_and_closes() {
        let input = lines(&["```csv", "a,b", "  1,2", "```", "", "$ qsv stats data.csv"]);
        let md = format_examples(&input);
        // Indentation inside fence preserved
        assert!(
            md.contains("  1,2"),
            "fenced indentation not preserved, got:\n{md}"
        );
        // Closing fence emitted
        let closes = md.matches("```").count();
        assert!(
            closes >= 2,
            "closing fence missing (found {closes} backticks blocks), got:\n{md}"
        );
        // qsv line still gets its own ```console fence after the explicit fence
        assert!(
            md.contains("```console\nqsv stats data.csv\n```"),
            "implicit qsv code block did not render after explicit fence, got:\n{md}"
        );
    }

    #[test]
    fn test_colon_introducer_appends_hard_line_break() {
        // A paragraph line ending in `:` (other than Examples:/Example:) should
        // get two trailing spaces so the next line renders on its own row.
        let input = lines(&["The supported formats are:", "csv, tsv, ssv"]);
        let md = format_description(&input);
        assert!(
            md.contains("supported formats are:  \n"),
            "missing hard line break after colon, got:\n{md:?}"
        );
        // A line ending in `Examples:` does NOT get the hard break (handled
        // upstream as a section transition).
        let input2 = lines(&["Examples:", "details"]);
        let md_2 = format_description(&input2);
        assert!(
            !md_2.contains("Examples:  \n"),
            "Examples: should not get hard line break, got:\n{md_2:?}"
        );
    }

    #[test]
    fn test_indented_block_after_colon_becomes_text_fence() {
        // A colon-introduced, >=4-space-indented aligned listing (e.g. viz's
        // "Chart types (subcommands):") is rendered verbatim in a ```text fence
        // with column alignment and continuation indentation preserved.
        let input = lines(&[
            "Chart types (subcommands):",
            "    smart       Auto-dashboard. Picks a chart per column from the",
            "                dataset's stats & frequency distribution.",
            "    bar         Bar chart.        --x = category, --y = value.",
        ]);
        let md = format_description(&input);
        assert!(md.contains("```text\n"), "missing text fence, got:\n{md}");
        // Column alignment preserved (dedented by the common 4-space indent).
        assert!(
            md.contains("smart       Auto-dashboard"),
            "key/value column alignment lost, got:\n{md}"
        );
        // Continuation line keeps its alignment under the description column.
        assert!(
            md.contains("\n            dataset's stats"),
            "continuation indentation lost, got:\n{md}"
        );
        // The introducing heading is plain (no `:  ` hard break before the fence).
        assert!(
            md.contains("Chart types (subcommands):\n\n```text"),
            "heading hard-break not dropped before fence, got:\n{md:?}"
        );
    }

    #[test]
    fn test_hanging_indent_continuation_not_fenced() {
        // A flush-left sentence whose wrapped continuation is merely indented
        // (NOT introduced by a colon heading) must stay prose, not a code block.
        let input = lines(&[
            "center             Robust location; median of pairwise averages.",
            "                   Stable with outliers; tolerates corrupted data.",
        ]);
        let md = format_description(&input);
        assert!(
            !md.contains("```text"),
            "hanging-indent continuation should not be fenced, got:\n{md}"
        );
    }

    #[test]
    fn test_indented_block_skipped_inside_list() {
        // An indented line that is a continuation of a list item (even after a
        // colon-terminated item) stays part of the list, not a code block.
        let input = lines(&[
            "It has subcommands:",
            "1. first item that wraps onto:",
            "    a continuation line of the list item",
        ]);
        let md = format_description(&input);
        assert!(
            !md.contains("```text"),
            "list-item continuation should not be fenced, got:\n{md}"
        );
    }

    #[test]
    fn test_indented_block_dedented_by_min_indent() {
        // Nested indentation (min 4, inner 8) survives: dedent by the run minimum
        // so the relative 4-space step is preserved.
        let input = lines(&["Code:", "    def f():", "        return 1"]);
        let md = format_description(&input);
        assert!(
            md.contains("def f():\n    return 1"),
            "relative nested indent not preserved after dedent, got:\n{md}"
        );
    }

    #[test]
    fn test_indented_block_keeps_interior_blank_line() {
        // An interior blank line does not split the run into two fences.
        let input = lines(&["Listing:", "    one   first", "", "    two   second"]);
        let md = format_description(&input);
        assert_eq!(
            md.matches("```text").count(),
            1,
            "interior blank line should not split the fence, got:\n{md}"
        );
    }

    #[test]
    fn test_parser_does_not_enter_examples_from_options() {
        // An option's help text containing "Examples:" must not flip the
        // parser state and pull subsequent option lines into the examples vec.
        let usage = r#"
Some description.

Usage:
    qsv foo [options]

foo options:
    --interval <s>    Time interval. Examples: "1h", "1d".
    --remote <url>    A remote URL.

Common options:
    -h, --help
"#;
        let sections = parse_usage_sections(usage);
        // Examples vec must remain empty — the source has no real Examples block.
        assert!(
            sections.examples.is_empty(),
            "examples should be empty when only an option's help has \"Examples:\", got: {:?}",
            sections.examples
        );
        // The --remote option must end up in an option group, not in examples.
        let in_options = sections
            .option_groups
            .iter()
            .any(|(_, lines)| lines.iter().any(|l| l.contains("--remote")));
        assert!(
            in_options,
            "--remote option leaked out of options vec; option_groups: {:?}",
            sections.option_groups
        );
    }

    #[test]
    fn test_parser_swallows_repeated_examples_markers_within_examples() {
        // Multi-section commands (e.g. `to`, `fetch`) use repeated
        // "Examples:" markers within the Examples block. Those subsequent
        // markers should be swallowed (skipped), not emitted as literal text.
        let usage = r#"
Top description.

Examples:

$ qsv foo a.csv

Examples:

$ qsv foo b.csv

Usage:
    qsv foo [options]
"#;
        let sections = parse_usage_sections(usage);
        let joined = sections.examples.join("\n");
        // The literal text "Examples:" should not appear inside the examples
        // vec — the marker is a state-transition keyword, not content.
        assert!(
            !joined.contains("Examples:"),
            "literal \"Examples:\" leaked into examples vec, got:\n{joined}"
        );
        // Both qsv invocations should be present.
        assert!(
            joined.contains("qsv foo a.csv") && joined.contains("qsv foo b.csv"),
            "both example commands should be in the examples vec, got:\n{joined}"
        );
    }

    #[test]
    fn test_extract_arg_types_handles_common_shapes() {
        let src = r#"
            #[derive(Deserialize)]
            struct Args {
                flag_jobs:            Option<usize>,
                flag_round:           Option<f64>,
                flag_no_headers:      bool,
                flag_select:          SelectColumns,
                flag_cache_threshold: Option<isize>,
                flag_seed:            Option<u64>,
                flag_dates_whitelist: Option<String>,
                flag_delimiter:       Option<Delimiter>,
                arg_input:            Option<String>,
                cmd_view:             bool,
            }
        "#;
        let m = extract_arg_types_from_source(src);
        assert_eq!(m.get("--jobs").copied(), Some("integer"));
        assert_eq!(m.get("--round").copied(), Some("float"));
        assert_eq!(m.get("--no-headers").copied(), Some("flag"));
        assert_eq!(m.get("--select").copied(), Some("string"));
        assert_eq!(m.get("--cache-threshold").copied(), Some("integer"));
        assert_eq!(m.get("--seed").copied(), Some("integer"));
        assert_eq!(m.get("--dates-whitelist").copied(), Some("string"));
        assert_eq!(m.get("--delimiter").copied(), Some("string"));
        // Positional `arg_*` and subcommand `cmd_*` fields must be skipped.
        assert!(!m.contains_key("--input"));
        assert!(!m.contains_key("--view"));
    }

    #[test]
    fn test_extract_arg_types_unwraps_nested_wrappers() {
        let src = r#"
            struct Args {
                flag_a: Option<Box<u32>>,
                flag_b: Box<Option<f32>>,
                flag_c: Option<Option<i64>>,
            }
        "#;
        let m = extract_arg_types_from_source(src);
        assert_eq!(m.get("--a").copied(), Some("integer"));
        assert_eq!(m.get("--b").copied(), Some("float"));
        assert_eq!(m.get("--c").copied(), Some("integer"));
    }

    #[test]
    fn test_extract_arg_types_missing_struct_returns_empty() {
        let m = extract_arg_types_from_source("// no Args here\nfn main() {}");
        assert!(m.is_empty());
    }

    #[test]
    fn test_extract_arg_types_handles_pub_visibility() {
        // `stats.rs` (and a few others) declare `pub struct Args` with
        // `pub flag_<name>: …,` fields. The extractor must still find them.
        let src = r#"
            pub struct Args {
                pub flag_round:                u32,
                pub flag_jobs:                 Option<usize>,
                pub flag_cache_threshold:      isize,
                pub flag_mode_cardinality_cap: u64,
                pub flag_select:               SelectColumns,
                pub flag_typesonly:            bool,
            }
        "#;
        let m = extract_arg_types_from_source(src);
        assert_eq!(m.get("--round").copied(), Some("integer"));
        assert_eq!(m.get("--jobs").copied(), Some("integer"));
        assert_eq!(m.get("--cache-threshold").copied(), Some("integer"));
        assert_eq!(m.get("--mode-cardinality-cap").copied(), Some("integer"));
        assert_eq!(m.get("--select").copied(), Some("string"));
        assert_eq!(m.get("--typesonly").copied(), Some("flag"));
    }

    #[test]
    fn test_extract_arg_types_handles_generic_args_and_no_space_before_brace() {
        // Future-proof: a command may declare `struct Args<'a>` or
        // `struct Args<T>`, or omit the space before `{`. The regex must
        // still locate the struct body.
        let src_lifetime = r#"
            pub struct Args<'a> {
                flag_count: Option<u32>,
            }
        "#;
        assert_eq!(
            extract_arg_types_from_source(src_lifetime)
                .get("--count")
                .copied(),
            Some("integer"),
        );

        let src_generic = r#"
            struct Args<T> {
                flag_ratio: f64,
            }
        "#;
        assert_eq!(
            extract_arg_types_from_source(src_generic)
                .get("--ratio")
                .copied(),
            Some("float"),
        );

        let src_no_space = r#"
            struct Args{
                flag_force: bool,
            }
        "#;
        assert_eq!(
            extract_arg_types_from_source(src_no_space)
                .get("--force")
                .copied(),
            Some("flag"),
        );
    }
}
