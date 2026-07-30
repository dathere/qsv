//! Localization support for `qsv viz` dashboards.
//!
//! rust-i18n owns the message catalog (`src/cmd/locales/*.yml`, initialized by the
//! `i18n!` call in `src/main.rs`). This module owns everything rust-i18n does not:
//!
//! * the [`LOCALES`] table mapping a curated language to its BCP-47 tag and the third-party i18n
//!   assets it needs (DataTables i18n JSON, plotly locale JS),
//! * [`parse_lang`], which accepts the several shapes a language can arrive in,
//! * the resolution order in [`resolve`] (explicit flag > dictionary > English) and the
//!   process-global [`active_locale`] the HTML assemblers read.
//!
//! Adding a language is: one `src/cmd/locales/<tag>.yml` + one [`LOCALES`] row
//! (+ the two vendored asset files once those land).

/// A curated language: one row per language qsv ships translations for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocaleRow {
    /// BCP-47 tag. Doubles as the rust-i18n locale name (so the YAML is
    /// `<bcp47>.yml`) and the value of the `<html lang>` attribute.
    pub bcp47:        &'static str,
    /// English name of the language, e.g. "Spanish". This is the form `describegpt`
    /// documents for its own `--language` flag, so it is what viz forwards when
    /// inferring a dictionary -- a code like "es" would be left to the LLM to guess.
    pub english_name: &'static str,
    /// Alternate spellings accepted on `--language` and in a dictionary's
    /// `x-qsv.detected_language_code`: ISO 639-3 codes (what describegpt's
    /// whatlang detection emits) and the English language name. Must be lowercase
    /// -- `parse_lang` lowercases its input before comparing.
    pub aliases:      &'static [&'static str],
}

/// Curated languages. English is first and is the fallback for every other locale.
///
/// RTL languages (ar, he) are deliberately absent: the dashboard layout has never
/// been audited under `dir="rtl"` and plotly's RTL support is poor. Adding them is
/// a layout project, not a translation one.
pub static LOCALES: &[LocaleRow] = &[
    LocaleRow {
        bcp47:        "en",
        english_name: "English",
        aliases:      &["eng", "english"],
    },
    LocaleRow {
        bcp47:        "es",
        english_name: "Spanish",
        aliases:      &["spa", "spanish", "espanol", "español"],
    },
];

/// Resolve a user- or dictionary-supplied language string to a curated locale.
///
/// Accepts a BCP-47/ISO 639-1 tag (`es`, `pt-BR`), an ISO 639-3 code (`spa` --
/// what describegpt writes to `x-qsv.detected_language_code`), or an English
/// language name (`Spanish`). Matching is case-insensitive, and `_` is normalized
/// to `-` so `pt_BR` works too.
///
/// Returns `None` for anything outside the curated set; callers decide whether
/// that is an error (explicit `--language`) or a fall back to English (an
/// autodetected dictionary language).
pub fn parse_lang(input: &str) -> Option<&'static LocaleRow> {
    let needle = input.trim().to_lowercase().replace('_', "-");
    if needle.is_empty() {
        return None;
    }

    LOCALES
        .iter()
        .find(|row| row.bcp47.to_lowercase() == needle || row.aliases.contains(&needle.as_str()))
        // A regional tag with no curated regional variant falls back to its base
        // language, matching rust-i18n's own territory fallback (zh-CN -> zh).
        .or_else(|| {
            needle.split_once('-').and_then(|(base, _)| {
                LOCALES
                    .iter()
                    .find(|row| row.bcp47.to_lowercase() == base || row.aliases.contains(&base))
            })
        })
}

/// The English locale row -- the default, and the fallback for uncurated languages.
#[inline]
pub fn english() -> &'static LocaleRow {
    &LOCALES[0]
}

/// Comma-separated curated tags, for error messages.
pub fn curated_list() -> String {
    LOCALES
        .iter()
        .map(|row| row.bcp47)
        .collect::<Vec<_>>()
        .join(", ")
}

// The active locale is process-global because rust-i18n's own `set_locale` is:
// threading a locale through ~100 HTML-assembly signatures buys nothing when the
// message lookup consults a global anyway. A plain atomic pointer (rather than a
// `OnceLock`) keeps it *settable more than once*, which matters for the in-file
// unit tests: `viz.rs` has 260+ of them sharing one process, and a set-once cell
// would let whichever test ran first pin the locale for all the others. Unset
// means English, so tests that never touch a locale need no setup at all.
static ACTIVE: std::sync::atomic::AtomicPtr<LocaleRow> =
    std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

/// The locale the current dashboard is being rendered in. English until [`set_active`].
pub fn active_locale() -> &'static LocaleRow {
    let ptr = ACTIVE.load(std::sync::atomic::Ordering::Relaxed);
    if ptr.is_null() {
        english()
    } else {
        // SAFETY: only ever set from `set_active`, which stores the address of a
        // `LOCALES` element -- a `'static` in immutable memory that is never freed.
        unsafe { &*ptr }
    }
}

/// Set the active locale for both this module and rust-i18n's message lookup.
pub fn set_active(row: &'static LocaleRow) {
    rust_i18n::set_locale(row.bcp47);
    ACTIVE.store(
        std::ptr::from_ref(row).cast_mut(),
        std::sync::atomic::Ordering::Relaxed,
    );
}

/// Reset to English. Used by unit tests to isolate themselves from each other.
#[cfg(test)]
pub fn reset_active() {
    set_active(english());
}

/// Serializes every unit test that mutates the active locale.
///
/// Both this module's state and rust-i18n's own `set_locale` are process-global, and cargo runs
/// unit tests on parallel threads within ONE process — so a test that selects Spanish and a test
/// that resets to English will interleave and each observe the other's locale. This lock lives
/// here, not in `viz.rs`, precisely because the racing tests span both modules (a `viz.rs` test
/// asserting Spanish strings was flipped to English by `viz_i18n`'s own reset test: it passed in
/// isolation and failed in the full suite).
///
/// **The rule: any unit test that MUTATES the locale *or* asserts on localized output must take
/// this lock.** Observers matter as much as mutators — the failure that motivated widening it was
/// `render_dict_page_html_renders_a_download_row_per_sidecar`, a pre-existing English-only test
/// that started failing the moment its `>schema<` label became translatable and a concurrent
/// Spanish test flipped it to `>esquema<`.
///
/// Integration tests need none of this — they spawn a fresh process per invocation, which is the
/// better home for locale assertions when there is a choice.
#[cfg(test)]
pub static LOCALE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Take [`LOCALE_LOCK`], ignoring poisoning (a panicking test must not cascade into every other
/// locale test reporting a poisoned lock instead of its own real failure).
#[cfg(test)]
pub fn lock_locale() -> std::sync::MutexGuard<'static, ()> {
    LOCALE_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

/// How the active locale was chosen -- lets the caller decide what to report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolution {
    /// An explicit `--language` (or a dictionary language) matched a curated locale.
    Curated(&'static LocaleRow),
    /// `--language` was given but names no curated locale. Callers MUST treat this
    /// as a hard error: silently anglicizing an explicit request is worse than failing.
    UnknownRequested(String),
    /// A dictionary autodetected a language qsv has no translations for. Render in
    /// English and note it on stderr -- absence of a translation is not a user error.
    UncuratedDetected(String),
}

/// Resolve the dashboard locale.
///
/// `flag` is the raw `--language` value (`"auto"`, or empty, means "not specified");
/// `detected` is `x-qsv.detected_language_code` from a data dictionary, if one was
/// loaded. Precedence: explicit flag > dictionary > English.
pub fn resolve(flag: Option<&str>, detected: Option<&str>) -> Resolution {
    if let Some(requested) = flag
        .map(str::trim)
        .filter(|f| !f.is_empty() && !f.eq_ignore_ascii_case("auto"))
    {
        return parse_lang(requested).map_or_else(
            || Resolution::UnknownRequested(requested.to_string()),
            Resolution::Curated,
        );
    }

    if let Some(detected) = detected.map(str::trim).filter(|d| !d.is_empty()) {
        return parse_lang(detected).map_or_else(
            || Resolution::UncuratedDetected(detected.to_string()),
            Resolution::Curated,
        );
    }

    Resolution::Curated(english())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lang_accepts_bcp47_iso639_3_and_english_names() {
        assert_eq!(parse_lang("es").unwrap().bcp47, "es");
        assert_eq!(parse_lang("spa").unwrap().bcp47, "es");
        assert_eq!(parse_lang("Spanish").unwrap().bcp47, "es");
        assert_eq!(parse_lang("en").unwrap().bcp47, "en");
        assert_eq!(parse_lang("eng").unwrap().bcp47, "en");
    }

    #[test]
    fn parse_lang_is_case_and_separator_insensitive() {
        assert_eq!(parse_lang("  SPA  ").unwrap().bcp47, "es");
        assert_eq!(parse_lang("SPANISH").unwrap().bcp47, "es");
        // A regional variant with no curated regional row falls back to its base.
        assert_eq!(parse_lang("es_MX").unwrap().bcp47, "es");
        assert_eq!(parse_lang("es-419").unwrap().bcp47, "es");
    }

    #[test]
    fn parse_lang_rejects_unknown_and_empty() {
        assert!(parse_lang("vie").is_none());
        assert!(parse_lang("Klingon").is_none());
        assert!(parse_lang("").is_none());
        assert!(parse_lang("   ").is_none());
    }

    #[test]
    fn resolve_prefers_flag_over_dictionary() {
        assert_eq!(
            resolve(Some("en"), Some("spa")),
            Resolution::Curated(english())
        );
        match resolve(Some("es"), Some("eng")) {
            Resolution::Curated(row) => assert_eq!(row.bcp47, "es"),
            other => panic!("expected curated es, got {other:?}"),
        }
    }

    #[test]
    fn resolve_falls_back_to_dictionary_then_english() {
        match resolve(None, Some("spa")) {
            Resolution::Curated(row) => assert_eq!(row.bcp47, "es"),
            other => panic!("expected curated es, got {other:?}"),
        }
        // "auto" and empty both mean "flag not specified".
        match resolve(Some("auto"), Some("spa")) {
            Resolution::Curated(row) => assert_eq!(row.bcp47, "es"),
            other => panic!("expected curated es, got {other:?}"),
        }
        assert_eq!(resolve(None, None), Resolution::Curated(english()));
        assert_eq!(resolve(Some("auto"), None), Resolution::Curated(english()));
    }

    #[test]
    fn resolve_distinguishes_bad_flag_from_uncurated_detection() {
        // An explicit request we cannot honor is an error...
        assert_eq!(
            resolve(Some("xx"), None),
            Resolution::UnknownRequested("xx".to_string())
        );
        // ...but a language the data merely happens to be in is not.
        assert_eq!(
            resolve(None, Some("vie")),
            Resolution::UncuratedDetected("vie".to_string())
        );
    }

    #[test]
    fn every_curated_locale_has_a_yaml_and_unique_aliases() {
        let available = rust_i18n::available_locales!();
        let mut seen: Vec<&str> = Vec::new();
        for row in LOCALES {
            assert!(
                available.iter().any(|loc| loc == row.bcp47),
                "LOCALES row '{}' has no src/cmd/locales/{}.yml (available: {available:?})",
                row.bcp47,
                row.bcp47
            );
            for alias in row.aliases {
                assert!(
                    !alias.chars().any(char::is_uppercase),
                    "alias '{alias}' must be lowercase -- parse_lang lowercases its input"
                );
                assert!(
                    !seen.contains(alias),
                    "alias '{alias}' is claimed by more than one locale"
                );
                seen.push(alias);
            }
        }
    }

    /// Byte offsets of the `{` in every `%{...}` token that `rust_i18n::replace_patterns` would
    /// actually offer for substitution. This deliberately REPLICATES that function's scanner rather
    /// than approximating it: the whole point is to model the quirk, not a tidied-up version of it.
    /// The scanner resets to "seeking `{`" on every `%`, including one INSIDE a token's braces.
    fn substitutable_open_braces(value: &str) -> Vec<usize> {
        let mut positions: Vec<usize> = Vec::new();
        let mut stage = 0u8;
        for (i, &b) in value.as_bytes().iter().enumerate() {
            match (stage, b) {
                (1, b'{') => {
                    stage = 2;
                    positions.push(i);
                },
                (2, b'}') => {
                    stage = 0;
                    positions.push(i);
                },
                (_, b'%') => stage = 1,
                _ => {},
            }
        }
        positions.chunks_exact(2).map(|pair| pair[0]).collect()
    }

    /// The `%{q_*}` argument tokens in `value` that `replace_patterns` would silently skip. Matched
    /// by POSITION, not by name, so a key appearing twice -- once reachable, once stranded -- still
    /// reports the stranded one.
    fn unsubstitutable_arg_tokens(value: &str) -> Vec<&str> {
        let reachable = substitutable_open_braces(value);
        let mut missed = Vec::new();
        let mut cursor = 0usize;
        while let Some(rel) = value[cursor..].find("%{q_") {
            let open = cursor + rel + 1; // index of the `{`
            let Some(close_rel) = value[open..].find('}') else {
                break;
            };
            let close = open + close_rel;
            if !reachable.contains(&open) {
                missed.push(&value[open + 1..close]);
            }
            cursor = close + 1;
        }
        missed
    }

    /// Whether `value` is a double-quoted YAML scalar whose closing quote lands on the same line (a
    /// trailing comment after it is fine). Backslash escapes are honored, so a `\"` inside the
    /// string is not mistaken for the terminator.
    fn is_single_line_double_quoted(value: &str) -> bool {
        let mut chars = value.chars();
        if chars.next() != Some('"') {
            return false;
        }
        let mut escaped = false;
        for c in chars {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                return true;
            }
        }
        false
    }

    /// Guards the one localization failure that is invisible to every other check.
    ///
    /// A plotly token carrying a literal `%` inside its braces (`%{x:.0%}`, the Lorenz axis format)
    /// swallows its own closing brace in rust-i18n's scanner and mis-pairs with the next `{`. Any
    /// `%{q_*}` argument AFTER such a token is then left unsubstituted -- and rust-i18n reports
    /// nothing, so the raw key ships straight into a user's tooltip.
    ///
    /// Key-set parity between locales cannot see this (both locales can be wrong in the same
    /// place), and the English golden fixtures cannot see it either, because the constraint is
    /// about token ORDER within a sentence and every language orders its sentences differently. A
    /// translator moving the measure name to the end of a Lorenz tooltip -- which is exactly what
    /// Spanish word order wants -- reintroduces it with no signal at all.
    ///
    /// Reads the YAML from disk rather than the compiled catalog on purpose: editing only a locale
    /// file may not trigger a rebuild, so a compiled-catalog check could pass against stale bytes.
    #[test]
    fn no_catalog_value_strands_an_argument_behind_a_percent_formatted_token() {
        // the detector must actually detect -- a no-op implementation would sail through the
        // catalog sweep below and look exactly like a clean bill of health
        assert_eq!(
            unsubstitutable_arg_tokens("bottom %{x:.0%} of %{q_label}"),
            vec!["q_label"],
            "detector must flag an argument stranded behind a percent-formatted plotly token"
        );
        assert!(
            unsubstitutable_arg_tokens("%{q_label}: bottom %{x:.0%}").is_empty(),
            "an argument ahead of the percent-formatted token is fine -- ordering is the fix"
        );
        assert!(
            unsubstitutable_arg_tokens("%{q_col}: %{x:.3s}<br>%{z:,} rows").is_empty(),
            "ordinary plotly tokens must not be mistaken for the hazard"
        );

        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/cmd/locales");
        let mut checked = 0usize;
        let mut files = 0usize;
        for entry in std::fs::read_dir(&dir).expect("src/cmd/locales must be readable") {
            let path = entry.expect("readable dir entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("yml") {
                continue;
            }
            files += 1;
            let locale = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let text = std::fs::read_to_string(&path).expect("readable locale file");
            for (idx, line) in text.lines().enumerate() {
                let trimmed = line.trim_start();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }
                let Some((key, rest)) = trimmed.split_once(':') else {
                    continue;
                };
                let value = rest.trim();
                // a nesting parent (`viz:`) has no scalar on this line at all
                if value.is_empty() {
                    continue;
                }
                // the one non-message scalar in these files
                if key == "_version" {
                    continue;
                }
                // Every message value must be a double-quoted scalar that closes on its own line,
                // and that is ENFORCED here rather than assumed.
                //
                // This sweep reads one line at a time, and several YAML scalar styles may legally
                // continue onto the NEXT line: block (`|`, `>`), single-quoted, and even plain. A
                // continuation line carries no `key:`, so it is skipped -- and a `%{q_*}` sitting
                // on it would slip past the hazard check while the `checked` floor below still
                // read as healthy. Recognizing those styles line-wise is not possible without a
                // real YAML parser, and yaml_serde is gated on the `profile` feature, not `viz`.
                //
                // So the sweep demands the one style it can fully see. Both catalogs already use
                // it for every value, so this costs nothing today while making the coverage claim
                // total rather than partial.
                assert!(
                    is_single_line_double_quoted(value),
                    "{locale}.yml:{} key '{key}' is not a double-quoted scalar closed on its own \
                     line. Message catalogs must use that one style: every other YAML scalar \
                     style can continue onto a following line, where this line-wise hazard sweep \
                     cannot see it. Rewrite as: {key}: \"...\"\n  got: {value}",
                    idx + 1
                );
                checked += 1;
                let missed = unsubstitutable_arg_tokens(value);
                assert!(
                    missed.is_empty(),
                    "{locale}.yml:{} key '{key}' strands argument(s) {missed:?} behind a \
                     percent-formatted plotly token -- they will render as literal text. Move \
                     every %{{q_*}} ahead of any %{{..%}} token in this value:\n  {value}",
                    idx + 1
                );
            }
        }
        // a silently-empty sweep is the failure mode this floor exists to catch
        assert!(
            files >= 2,
            "expected at least en.yml and es.yml, saw {files}"
        );
        assert!(
            checked >= 100,
            "only {checked} catalog values were inspected -- the line extractor has probably \
             stopped matching the YAML shape"
        );
    }

    #[test]
    fn english_name_round_trips_through_parse_lang() {
        // viz forwards `english_name` to describegpt's --language. Requiring it to also be
        // an accepted alias keeps that value something a user could have typed themselves,
        // so `--language Spanish` and the forwarded "Spanish" can never diverge.
        for row in LOCALES {
            assert!(!row.english_name.is_empty());
            let parsed = parse_lang(row.english_name)
                .unwrap_or_else(|| panic!("english_name '{}' is not parseable", row.english_name));
            assert_eq!(
                parsed.bcp47, row.bcp47,
                "english_name '{}' should resolve back to its own locale",
                row.english_name
            );
        }
    }

    #[test]
    fn active_locale_defaults_to_english_and_is_resettable() {
        let _guard = lock_locale();
        reset_active();
        assert_eq!(active_locale().bcp47, "en");

        let spanish = parse_lang("es").unwrap();
        set_active(spanish);
        assert_eq!(active_locale().bcp47, "es");

        // Resettable, unlike a OnceLock -- 260+ in-file viz unit tests share this
        // process and must not be able to pin the locale for one another.
        reset_active();
        assert_eq!(active_locale().bcp47, "en");
    }
}
