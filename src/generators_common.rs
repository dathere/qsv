//! Shared helpers used by `mcp_skills_gen` and `help_markdown_gen` to analyse
//! docopt-style USAGE strings.
//!
//! The main responsibility is `extract_required_options_from_usage`, which
//! identifies the set of option flags (long + short forms) that are required
//! to invoke the command — i.e. flags appearing in **every** non-`--help`
//! Usage variant, outside any `[...]` (optional) group, and outside any
//! `(A | B)` alternative group.

use std::collections::{HashMap, HashSet};

/// Bidirectional short↔long alias map for option declarations.
#[derive(Debug, Default, Clone)]
pub struct FlagPairs {
    short_to_long: HashMap<String, String>,
    long_to_short: HashMap<String, String>,
}

impl FlagPairs {
    /// Build the pair map by scanning the options-declaration portion of the
    /// USAGE string for lines like `-k, --keys <keys>` or `--keys, -k <keys>`.
    /// The `Usage:` block itself is deliberately skipped so a future
    /// Usage-line formatting quirk can't introduce a bogus pair.
    pub fn from_usage(usage_text: &str) -> Self {
        let options_text = skip_usage_block(usage_text);

        // Short-first: `-k, --keys`
        let short_first =
            regex::Regex::new(r"(?m)^\s+(-[A-Za-z])\s*,\s*(--[A-Za-z][\w-]*)").unwrap();
        // Long-first:  `--keys, -k`
        let long_first =
            regex::Regex::new(r"(?m)^\s+(--[A-Za-z][\w-]*)\s*,\s*(-[A-Za-z])").unwrap();

        let mut short_to_long: HashMap<String, String> = HashMap::new();
        let mut long_to_short: HashMap<String, String> = HashMap::new();

        for cap in short_first.captures_iter(options_text) {
            if let (Some(s), Some(l)) = (cap.get(1), cap.get(2)) {
                short_to_long.insert(s.as_str().to_string(), l.as_str().to_string());
                long_to_short.insert(l.as_str().to_string(), s.as_str().to_string());
            }
        }
        for cap in long_first.captures_iter(options_text) {
            if let (Some(l), Some(s)) = (cap.get(1), cap.get(2)) {
                short_to_long.insert(s.as_str().to_string(), l.as_str().to_string());
                long_to_short.insert(l.as_str().to_string(), s.as_str().to_string());
            }
        }

        Self {
            short_to_long,
            long_to_short,
        }
    }

    /// Return the partner of `tok` (short→long or long→short) if one exists.
    pub fn partner(&self, tok: &str) -> Option<&str> {
        self.short_to_long
            .get(tok)
            .or_else(|| self.long_to_short.get(tok))
            .map(String::as_str)
    }
}

/// Walk past the `Usage:` block (up to the first blank line after the `Usage:`
/// line) so the options-declaration regex only scans option definition lines.
fn skip_usage_block(usage_text: &str) -> &str {
    let Some(usage_idx) = usage_text.find("Usage:") else {
        return usage_text;
    };
    let after_usage = &usage_text[usage_idx..];
    // Find the first blank line (double newline) after the Usage: header.
    after_usage
        .find("\n\n")
        .map_or("", |i| &after_usage[i + 2..])
}

/// The full global-required detection: intersect required-token sets across
/// all Usage variants, skipping `--help` variants entirely.
pub fn extract_required_options_from_usage(usage_text: &str) -> HashSet<String> {
    let usage_lines: Vec<&str> = usage_text
        .lines()
        .skip_while(|l| !l.contains("Usage:"))
        .skip(1)
        .take_while(|l| {
            let t = l.trim();
            !t.is_empty() && (t.contains("qsv") || t.ends_with("--help"))
        })
        .filter(|l| !l.trim().ends_with("--help"))
        .collect();

    let pairs = FlagPairs::from_usage(usage_text);

    let per_variant: Vec<HashSet<String>> = usage_lines
        .iter()
        .map(|l| required_tokens_in_usage_line(l, &pairs))
        .collect();

    let mut iter = per_variant.into_iter();
    iter.next()
        .map(|first| iter.fold(first, |acc, s| acc.intersection(&s).cloned().collect()))
        .unwrap_or_default()
}

/// Return the set of option tokens required on a single USAGE line: tokens
/// outside any `[...]` optional group and outside any `(A | B)` alternative
/// group. For every flag seen, also include its partner form (short↔long) so
/// callers can match either form.
pub fn required_tokens_in_usage_line(line: &str, pairs: &FlagPairs) -> HashSet<String> {
    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();

    // First pass: identify `(...)` ranges that contain a `|` (alternative
    // groups). Tokens inside such ranges are not individually required.
    let mut alt_mask = vec![false; n];
    {
        let mut stack: Vec<(usize, bool)> = Vec::new();
        let mut bracket_depth: u32 = 0;
        for (i, &ch) in chars.iter().enumerate() {
            match ch {
                '[' => bracket_depth = bracket_depth.saturating_add(1),
                ']' => bracket_depth = bracket_depth.saturating_sub(1),
                '(' if bracket_depth == 0 => stack.push((i, false)),
                ')' if bracket_depth == 0 && !stack.is_empty() => {
                    let (start, has_pipe) = stack.pop().unwrap();
                    if has_pipe {
                        for m in alt_mask.iter_mut().take(i + 1).skip(start) {
                            *m = true;
                        }
                    }
                },
                '|' if bracket_depth == 0 => {
                    if let Some(last) = stack.last_mut() {
                        last.1 = true;
                    }
                },
                _ => {},
            }
        }
    }

    // Second pass: project the line to just the required char positions.
    // `(`/`)` themselves are replaced with spaces so the flag regex (which
    // requires whitespace-or-start before the `-`) matches `-k` inside a plain
    // `(-k)` group.
    let mut proj = String::with_capacity(n);
    let mut bracket_depth: u32 = 0;
    for (i, &ch) in chars.iter().enumerate() {
        let is_structural = matches!(ch, '[' | ']' | '(' | ')');
        if ch == '[' {
            bracket_depth = bracket_depth.saturating_add(1);
        }
        let in_optional = bracket_depth > 0 || is_structural;
        let in_alt = alt_mask[i];
        if in_optional || in_alt {
            proj.push(' ');
        } else {
            proj.push(ch);
        }
        if ch == ']' {
            bracket_depth = bracket_depth.saturating_sub(1);
        }
    }

    let flag_re = regex::Regex::new(r"(?:^|\s)(-{1,2}[A-Za-z][\w-]*)").unwrap();
    let mut out = HashSet::new();
    for cap in flag_re.captures_iter(&proj) {
        if let Some(m) = cap.get(1) {
            let tok = m.as_str().to_string();
            if let Some(partner) = pairs.partner(&tok) {
                out.insert(partner.to_string());
            }
            out.insert(tok);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(xs: &[&str]) -> HashSet<String> {
        xs.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn implode_single_variant_marks_short_and_long() {
        let usage = r#"
Usage:
    qsv implode [options] -k <keys> -v <value> <separator> [<input>]
    qsv implode --help

implode options:
    -k, --keys <keys>    Key columns.
    -v, --value <value>  Value column.
    -r, --rename <name>  Rename.
"#;
        let got = extract_required_options_from_usage(usage);
        assert_eq!(got, set(&["-k", "--keys", "-v", "--value"]));
    }

    #[test]
    fn alternative_group_yields_nothing_required() {
        let usage = r#"
Usage:
    qsv split [options] (--size <arg> | --chunks <arg> | --kb-size <arg>) <outdir> [<input>]
    qsv split --help
"#;
        let got = extract_required_options_from_usage(usage);
        assert!(got.is_empty(), "got {got:?}");
    }

    #[test]
    fn multi_variant_option_not_globally_required() {
        let usage = r#"
Usage:
    qsv joinp [options] <columns1> <input1> <columns2> <input2>
    qsv joinp --cross [options] <input1> <input2>
    qsv joinp --non-equi [options] <keys1> <input1> <keys2> <input2>
    qsv joinp --help
"#;
        let got = extract_required_options_from_usage(usage);
        assert!(got.is_empty(), "got {got:?}");
    }

    #[test]
    fn subcommand_scoped_required_option_is_not_global() {
        let usage = r#"
Usage:
    qsv apply operations <operations> [options] <column> [<input>]
    qsv apply emptyreplace --replacement=<string> [options] <column> [<input>]
    qsv apply dynfmt --formatstr=<string> [options] --new-column=<name> [<input>]
    qsv apply --help
"#;
        let got = extract_required_options_from_usage(usage);
        assert!(got.is_empty(), "got {got:?}");
    }

    #[test]
    fn short_role_collision_doesnt_leak_to_long_partner() {
        // luau's `-n <main-script>` is the short of `-n, --no-headers`, but
        // it only appears on one Usage variant. Intersection must exclude it.
        let usage = r#"
Usage:
    qsv luau map [options] -n <main-script> [<input>]
    qsv luau map [options] <new-columns> <main-script> [<input>]
    qsv luau filter [options] <main-script> [<input>]
    qsv luau --help

Common options:
    -n, --no-headers    When set, the first row will not be interpreted as headers.
"#;
        let got = extract_required_options_from_usage(usage);
        assert!(got.is_empty(), "got {got:?}");
    }

    #[test]
    fn paren_without_pipe_does_not_make_content_optional() {
        // docopt `(X)` with no `|` is a plain group — content is still required.
        let usage = r#"
Usage:
    qsv foo [options] (-k <keys>) <input>
"#;
        let pairs = FlagPairs::from_usage(usage);
        let got =
            required_tokens_in_usage_line("    qsv foo [options] (-k <keys>) <input>", &pairs);
        assert!(got.contains("-k"), "got {got:?}");
    }

    #[test]
    fn nested_optional_inside_alt_stays_optional() {
        let usage = r#"
Usage:
    qsv foo [options] ( [-a] | -b ) <input>
"#;
        let got = extract_required_options_from_usage(usage);
        assert!(got.is_empty(), "got {got:?}");
    }

    #[test]
    fn long_first_declaration_pairs_correctly() {
        // If someone writes `--keys, -k <keys>` (long first), we should still
        // pair them so a Usage line mentioning only one form fills in both.
        let usage = r#"
Usage:
    qsv foo [options] --keys <keys> <input>

options:
    --keys, -k <keys>    Key columns.
"#;
        let got = extract_required_options_from_usage(usage);
        assert_eq!(got, set(&["-k", "--keys"]));
    }

    #[test]
    fn long_only_in_usage_still_expands_via_pair() {
        // If only the long form appears in Usage but a short pair is declared,
        // the short form should also be in the required set.
        let usage = r#"
Usage:
    qsv foo [options] --keys <keys> <input>

options:
    -k, --keys <keys>    Key columns.
"#;
        let got = extract_required_options_from_usage(usage);
        assert_eq!(got, set(&["-k", "--keys"]));
    }

    #[test]
    fn no_usage_variants_yields_empty_set() {
        let usage = "This has no Usage: section at all.";
        assert!(extract_required_options_from_usage(usage).is_empty());
    }

    #[test]
    fn unbalanced_brackets_dont_underflow() {
        // An unbalanced `]` shouldn't underflow bracket_depth and silently
        // drop later required tokens.
        let usage = r#"
Usage:
    qsv foo ] [options] -k <keys> <input>

options:
    -k, --keys <keys>    Key.
"#;
        let got = extract_required_options_from_usage(usage);
        assert!(got.contains("-k") && got.contains("--keys"), "got {got:?}");
    }

    #[test]
    fn pair_regex_does_not_scan_usage_block() {
        // If the Usage: block itself contains a `-x, --xxx` textual pattern
        // (e.g., in a comment or wrapped line), we must not build a bogus
        // pair from it. The legitimate pair comes only from the options block.
        let usage = r#"
Usage:
    qsv foo [options] -x, --extra <bogus> <input>

options:
    -k, --keys <keys>    Key.
"#;
        let pairs = FlagPairs::from_usage(usage);
        // -k ↔ --keys is declared in options; --extra has no partner.
        assert_eq!(pairs.partner("-k"), Some("--keys"));
        assert_eq!(pairs.partner("--keys"), Some("-k"));
        assert_eq!(pairs.partner("--extra"), None);
    }
}
