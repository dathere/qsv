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
