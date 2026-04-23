//! Shared helpers used by `mcp_skills_gen` and `help_markdown_gen` to analyse
//! docopt-style USAGE strings.
//!
//! The main responsibility is `extract_required_options_from_usage`, which
//! identifies the set of option flags (long + short forms) that are required
//! to invoke the command — i.e. flags appearing in **every** non-`--help`
//! Usage variant, outside any `[...]` (optional) group, and outside any
//! `(A | B)` alternative group.

use std::{
    collections::{HashMap, HashSet},
    sync::OnceLock,
};

use regex::Regex;

/// `-k, --keys` (short-first option declaration, anchored to an indented line).
fn short_first_pair_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?m)^\s+(-[A-Za-z])\s*,\s*(--[A-Za-z][\w-]*)").unwrap())
}

/// `--keys, -k` (long-first option declaration).
fn long_first_pair_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?m)^\s+(--[A-Za-z][\w-]*)\s*,\s*(-[A-Za-z])").unwrap())
}

/// An option flag token appearing after a whitespace-or-start boundary.
fn flag_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?:^|\s)(-{1,2}[A-Za-z][\w-]*)").unwrap())
}

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
    /// Usage-line formatting quirk can't introduce a bogus pair. If no options
    /// section can be located (e.g. a minimal fixture), falls back to scanning
    /// the entire text so callers still get usable pairings.
    pub fn from_usage(usage_text: &str) -> Self {
        let narrowed = options_section(usage_text);
        let scan = if narrowed.is_empty() {
            usage_text
        } else {
            narrowed
        };

        let mut short_to_long: HashMap<String, String> = HashMap::new();
        let mut long_to_short: HashMap<String, String> = HashMap::new();

        for cap in short_first_pair_re().captures_iter(scan) {
            if let (Some(s), Some(l)) = (cap.get(1), cap.get(2)) {
                short_to_long.insert(s.as_str().to_string(), l.as_str().to_string());
                long_to_short.insert(l.as_str().to_string(), s.as_str().to_string());
            }
        }
        for cap in long_first_pair_re().captures_iter(scan) {
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

/// Return the slice of USAGE starting at the first `options:` / `Options:`
/// header (case-insensitive, line-anchored) and ending at the end of the
/// string. Returns `""` if no such header exists.
fn options_section(usage_text: &str) -> &str {
    // Line-anchored `options:` header, case-insensitive. Matches
    // `options:`, `Options:`, `Common options:`, `map options:`, etc.
    // Leading class is intentionally `[ \t\w-]` (not `\s`) so it cannot
    // straddle a newline.
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"(?mi)^[ \t\w-]*options:[ \t]*$").unwrap());
    re.find(usage_text).map_or("", |m| &usage_text[m.start()..])
}

/// The full global-required detection: intersect required-token sets across
/// all Usage variants, skipping `--help` variants entirely.
pub fn extract_required_options_from_usage(usage_text: &str) -> HashSet<String> {
    let usage_lines = collect_usage_lines(usage_text);

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

/// Collect the non-`--help` Usage variants from a docopt USAGE string.
///
/// The block is bounded by the `Usage:` header and the first blank line after
/// it. If the header line itself carries an inline variant
/// (`Usage: qsv foo ...`), that variant is retained.
///
/// Continuation detection compares each line's leading whitespace against a
/// *baseline indent* — the indent of the first non-blank line in the block.
/// Lines strictly more indented than the baseline are merged into the
/// previous variant (so a wrapped Usage line can't become its own bogus
/// variant). Lines at the baseline start new variants.
///
/// `--help` stub variants are filtered out.
fn collect_usage_lines(usage_text: &str) -> Vec<String> {
    let mut lines = usage_text.lines();

    // Locate the `Usage:` header and capture any inline variant on the same
    // line (e.g. `Usage: qsv foo [options]`).
    let mut inline_variant: Option<String> = None;
    for line in lines.by_ref() {
        if let Some(after) = line.find("Usage:").map(|i| &line[i + "Usage:".len()..]) {
            let inline = after.trim();
            if !inline.is_empty() {
                inline_variant = Some(inline.to_string());
            }
            break;
        }
    }

    // Remaining block, up to the first blank line.
    let raw: Vec<String> = lines
        .take_while(|l| !l.trim().is_empty())
        .map(str::to_string)
        .collect();

    // Baseline indent: the leading-whitespace count of the first non-blank
    // line in the block. Defaults to 0 if the block is empty.
    let baseline_indent = raw
        .iter()
        .map(|l| l.len() - l.trim_start().len())
        .next()
        .unwrap_or(0);

    let mut variants: Vec<String> = Vec::new();

    // Store the inline variant (if any) synthesized at the baseline indent
    // so its indentation comparison matches the rest of the block.
    if let Some(inline) = inline_variant {
        variants.push(format!("{}{}", " ".repeat(baseline_indent), inline));
    }

    for line in raw {
        let leading_ws = line.len() - line.trim_start().len();
        let is_continuation = leading_ws > baseline_indent;

        if is_continuation && let Some(last) = variants.last_mut() {
            last.push(' ');
            last.push_str(line.trim());
        } else {
            variants.push(line);
        }
    }

    variants
        .into_iter()
        .filter(|l| !l.trim().ends_with("--help"))
        .collect()
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

    let mut out = HashSet::new();
    for cap in flag_re().captures_iter(&proj) {
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

    #[test]
    fn continuation_line_does_not_truncate_usage_block() {
        // A long Usage variant that wraps onto a continuation line without
        // repeating the binary name must not cause later variants to be
        // dropped. Previously the scan terminated on the first line not
        // containing `qsv` or ending in `--help`, which would have silently
        // truncated the intersection here.
        let usage = r#"
Usage:
    qsv foo [options] <very-long-positional> <another-positional>
        --bar <bar>
    qsv foo [options] --bar <bar> <input>
    qsv foo --help

options:
    --bar <bar>    A required option across variants.
"#;
        let got = extract_required_options_from_usage(usage);
        assert!(got.contains("--bar"), "got {got:?}");
    }

    #[test]
    fn pair_regex_scans_only_the_options_section_not_description() {
        // A description paragraph between Usage and the options section must
        // not be scanned for pairs — otherwise a wrapped sentence beginning
        // with `-s, --long` could silently introduce a bogus pair.
        let usage = r#"
Usage:
    qsv foo [options] <input>

This is a description paragraph. On a fresh line, a quirky sentence:
    -x, --bogus <unreal>   not actually an option; just prose.

options:
    -k, --keys <keys>    Key.
"#;
        let pairs = FlagPairs::from_usage(usage);
        assert_eq!(pairs.partner("-k"), Some("--keys"));
        // The bogus description line is *before* the `options:` header, so
        // it must not contribute a pair.
        assert_eq!(pairs.partner("-x"), None);
        assert_eq!(pairs.partner("--bogus"), None);
    }

    #[test]
    fn options_header_with_prefix_word_matches() {
        // `Common options:` and `map options:` must both be recognised as an
        // options-section header so the pair regex scans only those blocks.
        let usage_common = r#"
Usage:
    qsv foo [options] --keys <keys> <input>

Common options:
    -k, --keys <keys>    Key.
"#;
        assert_eq!(
            extract_required_options_from_usage(usage_common),
            set(&["-k", "--keys"])
        );

        let usage_map = r#"
Usage:
    qsv foo map [options] --keys <keys> <input>

map options:
    -k, --keys <keys>    Key.
"#;
        assert_eq!(
            extract_required_options_from_usage(usage_map),
            set(&["-k", "--keys"])
        );
    }

    #[test]
    fn options_header_with_leading_tab_matches() {
        // `\t options:` (tab-indented header) must still match the
        // options-section regex.
        let usage = "\nUsage:\n    qsv foo [options] --keys <keys>\n\n\toptions:\n    -k, --keys \
                     <keys>    Key.\n";
        assert_eq!(
            extract_required_options_from_usage(usage),
            set(&["-k", "--keys"])
        );
    }

    #[test]
    fn indented_wrap_line_merges_into_parent_variant() {
        // A wrapped Usage line at deeper-than-baseline indent must fold into
        // the parent variant, not stand alone. Without this, the intersection
        // would treat `<keys>` as its own variant with no options and wipe
        // out any options required by the true variant.
        let usage = r#"
Usage:
    qsv foo [options] -k <keys> -v <value>
        <more-args>
    qsv foo --help

options:
    -k, --keys <keys>    Key.
    -v, --value <v>      Value.
"#;
        assert_eq!(
            extract_required_options_from_usage(usage),
            set(&["-k", "--keys", "-v", "--value"])
        );
    }

    #[test]
    fn continuation_starting_with_qsv_prefix_is_still_a_continuation() {
        // Per the indentation-outranks-prefix rule, a continuation line whose
        // first bareword begins with `qsv-` (e.g. the hypothetical
        // `qsv-helper` operand) must still fold into its parent variant
        // because it's indented past the baseline.
        let usage = r#"
Usage:
    qsv foo [options] --keys <keys>
        qsv-helper-arg
    qsv foo --help

options:
    -k, --keys <keys>    Key.
"#;
        assert_eq!(
            extract_required_options_from_usage(usage),
            set(&["-k", "--keys"])
        );
    }

    #[test]
    fn inline_usage_plus_indented_second_variant_stays_separate() {
        // Regression for the inline-vs-non-inline storage asymmetry: a
        // `Usage: qsv foo --bar` header followed by a second variant at the
        // standard column must be recognised as two variants (not merged as
        // "continuation" because the stored inline variant had leading_ws=0).
        // `--bar` appears only in the first variant → intersection is empty.
        let usage = r#"
Usage: qsv foo --bar
       qsv foo --baz

options:
    --bar    Only in variant 1.
    --baz    Only in variant 2.
"#;
        assert!(
            extract_required_options_from_usage(usage).is_empty(),
            "got {:?}",
            extract_required_options_from_usage(usage)
        );
    }

    #[test]
    fn inline_usage_header_variant_is_retained() {
        // `Usage: qsv foo [options] -k <keys>` — the variant on the same
        // line as the `Usage:` header must be recognised.
        let usage = r#"
Usage: qsv foo [options] -k <keys> <input>

options:
    -k, --keys <keys>    Key.
"#;
        assert_eq!(
            extract_required_options_from_usage(usage),
            set(&["-k", "--keys"])
        );
    }

    #[test]
    fn fallback_to_whole_text_when_no_options_section() {
        // A minimal fixture without an `options:` header must still pair
        // short/long declarations that happen to appear elsewhere, so tests
        // and small USAGE strings keep working.
        let usage = r#"
Usage:
    qsv foo [options] --keys <keys> <input>

    -k, --keys <keys>    Key columns (no options: header).
"#;
        let got = extract_required_options_from_usage(usage);
        assert_eq!(got, set(&["-k", "--keys"]));
    }
}
