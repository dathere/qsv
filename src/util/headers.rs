use super::to_lowercase_into;
use crate::select::SelectColumns;

/// The column selection a `--stats-options` string (moarstats, pragmastat) hands to `qsv stats`
/// (`--select <arg>`, `--select=<arg>`, `-s <arg>`, `-s<arg>`, or `s` ending a docopt cluster of
/// no-argument short flags such as `-Es <arg>`), if any. The last one wins. Mirrors
/// moarstats' `stats_options_redirect_output` cluster rules: scanning stops at the first
/// argument-taking short option, whose value is the rest of the token.
pub fn stats_options_select(stats_options: &str) -> Option<String> {
    // no-argument short flags accepted by `qsv stats`
    const STATS_SHORT_FLAGS: [char; 3] = ['E', 'h', 'n'];

    let tokens: Vec<&str> = stats_options.split_whitespace().collect();
    let mut found = None;
    for (i, token) in tokens.iter().enumerate() {
        if let Some(long) = token.strip_prefix("--") {
            if long == "select" {
                found = tokens.get(i + 1).map(ToString::to_string);
            } else if let Some(v) = long.strip_prefix("select=") {
                found = Some(v.to_string());
            }
        } else if let Some(short) = token.strip_prefix('-') {
            for (pos, ch) in short.char_indices() {
                if ch == 's' {
                    let rest = &short[pos + ch.len_utf8()..];
                    found = if rest.is_empty() {
                        tokens.get(i + 1).map(ToString::to_string)
                    } else {
                        Some(rest.to_string())
                    };
                    break;
                }
                if !STATS_SHORT_FLAGS.contains(&ch) {
                    break;
                }
            }
        }
    }
    found
}

/// The exact column of every row that `qsv stats` writes when run with `stats_options`: its
/// `--select` resolved against `headers`, or every column in order. `None` if that selection
/// doesn't resolve. For `resolve_stats_columns`' `selection`.
pub fn stats_options_selection(
    stats_options: &str,
    headers: &csv::ByteRecord,
    use_names: bool,
) -> Option<Vec<usize>> {
    match stats_options_select(stats_options) {
        None => Some((0..headers.len()).collect()),
        Some(spec) => SelectColumns::parse(&spec)
            .and_then(|sc| sc.selection(headers, use_names))
            .ok()
            .map(|sel| sel.to_vec()),
    }
}

/// Map each stats row (`names`: the stats CSV's `field` column, in row order) to the CSV column
/// it describes. A plain name lookup is wrong whenever a header repeats: first-match sent every
/// duplicate to the first column (#4663), last-match to the last. Shared by moarstats and
/// pragmastat, which append columns to a stats cache.
///
/// - `selection`: the exact column of every row, when the baseline `stats` ran in THIS invocation
///   (its `--select`, resolved against the header, or every column in order). It is used only if it
///   lines up with `names` row for row.
/// - Otherwise the stats cache was reused, and which columns it covers is not recorded reliably. A
///   name that occurs once in the header maps to that column, however many rows carry it (`--select
///   1,1`). A repeated name maps by occurrence - the k-th row named N to the k-th column named N -
///   only when the cache has exactly as many rows named N as the header has columns. Any other
///   count means a selection that skipped or repeated a twin, which cannot be told apart without
///   the selection, so those rows map to `None` and their names are returned for a warning. An
///   empty result beats another column's statistics.
///
/// Known limit: equal counts do not prove the order. A reused cache built with a selection that
/// reorders or repeats twins while keeping the count (on `a,b,a`: `--select 3,1` or `3,3`) is
/// mapped by occurrence, so a twin can get the other twin's appended statistics. The stats CSV
/// does not record the selection. The optional `.stats.csv.json` cache-metadata sidecar (not every
/// stats cache has one) keeps it as `flag_select`, but as the selectors' `Debug` text (`Index(3),
/// Index(1)`), not resolved columns, and that format is not a contract, so it is not parsed here.
/// Recomputing the baseline in the same invocation takes the exact `selection` path.
pub fn resolve_stats_columns(
    names: &[&str],
    csv_headers: &csv::StringRecord,
    selection: Option<&[usize]>,
) -> (Vec<Option<usize>>, Vec<String>) {
    if let Some(sel) = selection
        && sel.len() == names.len()
        && sel
            .iter()
            .zip(names)
            .all(|(&col, name)| csv_headers.get(col) == Some(*name))
    {
        return (sel.iter().map(|&col| Some(col)).collect(), Vec::new());
    }

    let mut positions: std::collections::HashMap<&str, Vec<usize>> =
        std::collections::HashMap::new();
    for (idx, h) in csv_headers.iter().enumerate() {
        positions.entry(h).or_default().push(idx);
    }
    let mut rows_named: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for name in names {
        *rows_named.entry(name).or_insert(0) += 1;
    }
    let mut seen: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    let mut ambiguous: Vec<String> = Vec::new();
    let cols = names
        .iter()
        .map(|&name| {
            let k = seen.entry(name).or_insert(0);
            let occurrence = *k;
            *k += 1;
            match positions.get(name) {
                None => None,
                Some(p) if p.len() == 1 => Some(p[0]),
                Some(p) if rows_named.get(name) == Some(&p.len()) => Some(p[occurrence]),
                Some(_) => {
                    if !ambiguous.iter().any(|a| a == name) {
                        ambiguous.push(name.to_string());
                    }
                    None
                },
            }
        })
        .collect();
    (cols, ambiguous)
}

/// Unique display labels for a list of column names that may repeat: the first occurrence keeps
/// its name, later ones get `safenames`' duplicate suffix - `a`, `a_2`, `a_3` - skipping any
/// suffix that is already a real name in the list (so `a,a,a_2` labels as `a`, `a_3`, `a_2`).
///
/// This is the ONE place duplicate labels are minted: `moarstats --bivariate` writes them into
/// its sidecar and `viz smart` rebuilds them from the same stats `field` list to map the sidecar
/// back to columns, so both sides must call this (#4663).
pub fn disambiguate_names<S: AsRef<str>>(names: &[S]) -> Vec<String> {
    let taken: std::collections::HashSet<&str> = names.iter().map(AsRef::as_ref).collect();
    let mut used: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut next_suffix: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    names
        .iter()
        .map(|n| {
            let name = n.as_ref();
            if used.insert(name.to_string()) {
                return name.to_string();
            }
            let k = next_suffix.entry(name).or_insert(2);
            loop {
                let label = format!("{name}_{k}");
                *k += 1;
                if !taken.contains(label.as_str()) && used.insert(label.clone()) {
                    return label;
                }
            }
        })
        .collect()
}

pub fn safe_header_names(
    headers: &csv::StringRecord,
    check_first_char: bool,
    conditional: bool,
    reserved_names: Option<&Vec<String>>,
    unsafe_prefix: &str,
    keep_case: bool,
    collapse: bool,
    unicode: bool,
) -> (Vec<String>, u16) {
    // Create "safe" var/key names - to support dynfmt/url-template, valid python vars & db-safe
    // column names. Fold to lowercase if keep_case is false. Trim leading & trailing whitespace.
    // Replace whitespace/non-alphanumeric) with _. If name starts with a number & check_first_char
    // is true, prepend the unsafe_prefix. If a column with the same name already exists,
    // append a sequence suffix (e.g. _n). Names are limited to 60 BYTES in length (snapped to
    // a UTF-8 char boundary), matching `is_safe_name`'s byte check and staying within
    // Postgres' default NAMEDATALEN of 63 bytes — including any disambiguation suffix.
    // Empty names are replaced with unsafe_prefix as well.
    //
    // If `collapse` is true, runs of non-alphanumeric characters collapse into a
    // single `_` (the regex gains a `+`); since `_` is itself non-alphanumeric,
    // runs of literal underscores collapse too. If `unicode` is true, unicode
    // letters & numbers are preserved instead of being stripped to ASCII.

    // If conditional = true & reserved_names is none, only rename the header if its not safe
    let prefix = if unsafe_prefix.is_empty() {
        "_"
    } else {
        unsafe_prefix
    };
    // Select the sanitization regex based on the (collapse, unicode) axes. Each
    // `regex_oncelock!` literal compiles to its own static OnceLock; we just pick
    // the reference at runtime. Appending `+` collapses runs into one `_`.
    let safename_regex = match (collapse, unicode) {
        (false, false) => regex_oncelock!(r"[^A-Za-z0-9]"),
        (true, false) => regex_oncelock!(r"[^A-Za-z0-9]+"),
        (false, true) => regex_oncelock!(r"[^\p{L}\p{N}]"),
        (true, true) => regex_oncelock!(r"[^\p{L}\p{N}]+"),
    };
    let mut changed_count = 0_u16;
    let mut name_vec: Vec<String> = Vec::with_capacity(headers.len());
    let mut safe_name: String;
    let mut safename_always: String;
    let mut safename_candidate: String;
    let mut final_candidate: String;
    let mut buf_wrk = String::new();

    for header_name in headers {
        let reserved_found = if let Some(reserved_names_vec) = reserved_names {
            if keep_case {
                header_name.clone_into(&mut buf_wrk);
            } else {
                to_lowercase_into(header_name, &mut buf_wrk);
            }
            reserved_names_vec
                .iter()
                .any(|reserved_name| reserved_name == &buf_wrk)
        } else {
            false
        };
        safe_name = if conditional
            && is_conditionally_safe(header_name, collapse, unicode)
            && !reserved_found
        {
            header_name.to_string()
        } else {
            // Trim first so whitespace-only headers are treated as empty —
            // the regex replace below would otherwise yield "" and the
            // as_bytes()[0] access would panic.
            safename_always = if header_name.trim().is_empty() {
                prefix.to_string()
            } else {
                safename_regex
                    .replace_all(header_name.trim(), "_")
                    .to_string()
            };
            // Check the first *scalar value*, not the first byte: in unicode
            // mode a preserved leading Unicode digit (e.g. "２col", "٣col") must
            // also trigger the unsafe prefix, otherwise it bypasses the
            // "starts with a number" safety rule. In ASCII mode the string is
            // pure ASCII, so is_ascii_digit on the first char is equivalent to
            // the previous first-byte check.
            if check_first_char
                && safename_always.chars().next().is_some_and(|c| {
                    if unicode {
                        c.is_numeric()
                    } else {
                        c.is_ascii_digit()
                    }
                })
            {
                safename_always = format!("{prefix}{safename_always}");
            }

            safename_candidate = if reserved_found {
                log::warn!("\"{safename_always}\" is a reserved name: {reserved_names:?}");
                format!("reserved_{safename_always}")
            } else {
                safename_always
            };

            // Apply lowercase and the "_<prefix>" prepend BEFORE truncation —
            // case-folding can change byte length (e.g. some Unicode glyphs
            // expand on lowercase) and prepending the prefix adds bytes too,
            // so doing them post-truncation could push the final name past
            // the 60-byte budget. Order: lowercase → leading-_ prefix → byte
            // truncate at a char boundary.
            final_candidate = if keep_case {
                safename_candidate
            } else {
                safename_candidate.to_lowercase()
            };
            if prefix != "_" && final_candidate.starts_with('_') {
                final_candidate = format!("{prefix}{final_candidate}");
            }
            if final_candidate.len() > 60 {
                final_candidate.truncate(final_candidate.floor_char_boundary(60));
            }
            final_candidate
        };
        let mut sequence_suffix = 2_u16;
        let mut candidate_name = safe_name.clone();
        while name_vec.contains(&candidate_name) {
            // Trim the base so that base + "_<n>" stays within the 60-byte
            // limit; otherwise the pre-truncated `safe_name` plus suffix could
            // overflow (e.g. safe_name=60 bytes + "_2" → 62 bytes), pushing
            // past Postgres' default NAMEDATALEN of 63 bytes and out of sync
            // with `is_safe_name`'s byte-based length check. We measure in
            // bytes (not chars) and snap to a UTF-8 char boundary so non-ASCII
            // headers can't smuggle in a 240-byte name behind a 60-char count.
            // Note: when two distinct long source headers share the same
            // first `base_max` bytes, they'll share the suffix counter — i.e.
            // the second header's first occurrence may be `_3` rather than
            // `_2`. This is intentional: the trimmed prefix is the actual
            // identifier downstream consumers see, so collisions in that
            // namespace must be disambiguated, not the original header's.
            let suffix = format!("_{sequence_suffix}");
            let base_max = 60_usize.saturating_sub(suffix.len());
            let base = if safe_name.len() > base_max {
                &safe_name[..safe_name.floor_char_boundary(base_max)]
            } else {
                safe_name.as_str()
            };
            candidate_name = format!("{base}{suffix}");
            sequence_suffix += 1;
        }
        if candidate_name.ne(header_name) {
            changed_count += 1;
        }
        name_vec.push(candidate_name);
    }
    log::debug!("safe header names: {name_vec:?}");
    (name_vec, changed_count)
}

#[inline]
pub fn is_safe_name(header_name: &str) -> bool {
    if header_name.trim().is_empty()
        || header_name.trim_start_matches('_').is_empty()
        || header_name.len() > 60
    {
        return false;
    }
    let first_character = header_name.trim_start_matches('_').as_bytes()[0];
    if first_character.is_ascii_digit() || first_character.is_ascii_whitespace() {
        return false;
    }
    let safename_re = regex_oncelock!(r"^[\w\-\s]+$");
    safename_re.is_match(header_name)
}

/// Whether `header_name` may be kept as-is in conditional mode given the active
/// `collapse`/`unicode` options. Beyond `is_safe_name`, a name is NOT
/// preservable when:
///  - `collapse` is set and it contains a run of 2+ characters the active rewrite would collapse
///    (e.g. the "__" in `a__b`); or
///  - `unicode` is set and it contains a character the unicode sanitizer (`[^\p{L}\p{N}]`) would
///    replace but `is_safe_name`'s lenient `\w` admits — e.g. combining marks (`\p{M}`) or
///    connector punctuation — apart from the quoted-identifier separators (`_`, `-`, whitespace);
///    or
///  - `unicode` is set and it starts with a Unicode digit (the rewrite prefixes it).
///
/// This keeps conditional mode consistent with always/verify under those flags —
/// e.g. `--mode c --collapse` rewrites `a__b` to `a_b` and `--mode c --unicode`
/// rewrites a decomposed `cafe\u{301}` to `cafe_`, while still preserving genuine
/// quoted identifiers like `Col with Embedded Spaces` and composed unicode names.
#[inline]
fn is_conditionally_safe(header_name: &str, collapse: bool, unicode: bool) -> bool {
    if !is_safe_name(header_name) {
        return false;
    }
    if collapse {
        let run_re = if unicode {
            regex_oncelock!(r"[^\p{L}\p{N}]{2,}")
        } else {
            regex_oncelock!(r"[^A-Za-z0-9]{2,}")
        };
        if run_re.is_match(header_name.trim()) {
            return false;
        }
    }
    if unicode {
        // Reject any character the unicode sanitizer would replace, except the
        // quoted-identifier separators we intentionally preserve (`_`, `-`,
        // whitespace). Without this, `\w`-only chars such as combining marks
        // would be kept here yet rewritten by always/verify.
        let unicode_extra_re = regex_oncelock!(r"[^\p{L}\p{N}_\-\s]");
        if unicode_extra_re.is_match(header_name) {
            return false;
        }
        // A preserved leading Unicode digit must also be rejected so the
        // "starts with a number" rule still applies (e.g. `２col`).
        if header_name
            .trim_start_matches('_')
            .chars()
            .next()
            .is_some_and(char::is_numeric)
        {
            return false;
        }
    }
    true
}

pub struct ColumnNameParser {
    chars: Vec<char>,
    pos:   usize,
}

impl ColumnNameParser {
    pub fn new(s: &str) -> ColumnNameParser {
        ColumnNameParser {
            chars: s.chars().collect(),
            pos:   0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<String>, String> {
        let mut new_cols_name = vec![];
        loop {
            if self.cur().is_none() {
                break;
            }
            if self.cur() == Some('"') {
                self.bump();
                new_cols_name.push(self.parse_quoted_name()?);
            } else {
                new_cols_name.push(self.parse_name());
            }
            self.bump();
        }
        Ok(new_cols_name)
    }

    fn cur(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    const fn bump(&mut self) {
        if self.pos < self.chars.len() {
            self.pos += 1;
        }
    }

    fn is_end_of_field(&self) -> bool {
        self.cur().is_none_or(|c| c == ',')
    }

    fn parse_quoted_name(&mut self) -> Result<String, String> {
        let mut name = String::new();
        loop {
            match self.cur() {
                None => {
                    return fail!("Unclosed quote, missing \".");
                },
                Some('"') => {
                    self.bump();
                    if self.cur() == Some('"') {
                        self.bump();
                        name.push('"');
                        name.push('"');
                        continue;
                    }
                    break;
                },
                Some(c) => {
                    name.push(c);
                    self.bump();
                },
            }
        }
        Ok(name)
    }

    fn parse_name(&mut self) -> String {
        let mut name = String::new();
        loop {
            if self.is_end_of_field() {
                break;
            }
            // safety: we know that the cur() will not be None as we checked above
            name.push(self.cur().unwrap());
            self.bump();
        }
        name
    }
}
