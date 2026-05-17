//! Maps a dictionary `content_type` semantic token to a `fake-rs` faker that
//! produces a realistic fake value.
//!
//! The token vocabulary is `crate::cmd::describegpt::dictionary::CONTENT_TYPE_VOCAB`
//! (47 tokens total; the full list is rendered into the LLM prompt by
//! `content_type_vocab_list()`, but `parse_llm_dictionary_response()` rejects
//! `unique_id` from LLM output — it's deterministic-only and set by
//! `generate_code_based_dictionary()` from the `<ALL_UNIQUE>` frequency
//! sentinel — so 46 tokens may actually appear in an LLM-produced dictionary).
//! Of the 47 vocab tokens, 44 map to a `fake-rs` faker; the three exceptions
//! (`category`, `unique_id`, `unknown` — see `NON_FAKER_TOKENS` below) and any
//! token not in the vocabulary return `None` so the caller falls back to
//! enumeration / frequency- or type-based generation. `unique_id` lands in
//! this fallback set because there is no general-purpose `fake-rs` faker that
//! guarantees per-row uniqueness across a synthesis pass.
//!
//! `duration` additionally accepts an LLM-inferred upper-bound suffix
//! (`"duration:N"` where N is seconds); see `parse_duration_cap`. Both the
//! bare and suffixed forms share one generation path.
//!
//! ## Multi-locale dispatch
//!
//! Each `fake-rs` locale is a *distinct Rust type* (e.g. `FirstName<EN>` vs
//! `FirstName<FR_FR>`), so runtime locale selection requires a `match` arm per
//! locale. The `gen_faker_for_locale!` macro stamps out one near-identical
//! generator function per locale; the public `content_type_to_value` then
//! dispatches by `Locale` enum. fake-rs declares every locale submodule for
//! every category, so all paths compile — sparse locales (e.g. lorem under
//! `FR_FR`) fall back to the `Data` trait default (EN data) at runtime.

use fake::{Fake, uuid::UUIDv4};
use rand::rngs::StdRng;

/// Stamp out one `content_type_to_value_<locale>` function bound to a specific
/// fake-rs locale module. The macro body is the entire token→faker `match` —
/// keep it in sync with `CONTENT_TYPE_VOCAB`.
macro_rules! gen_faker_for_locale {
    ($fn_name:ident, $loc:ident) => {
        #[allow(clippy::too_many_lines)]
        fn $fn_name(content_type: &str, rng: &mut StdRng) -> Option<String> {
            use fake::faker::{
                address::$loc as addr, barcode::$loc as barcode, color::$loc as fcolor,
                company::$loc as company, creditcard::$loc as creditcard,
                currency::$loc as currency, filesystem::$loc as fs, internet::$loc as net,
                job::$loc as job, lorem::$loc as lorem, name::$loc as name,
                phone_number::$loc as phone, time::$loc as ftime,
            };
            // NOTE: fake-rs's `automotive::LicencePlate` is only implemented for PT_PT,
            // TR_TR and FA_IR locales (5.1.0). To support `license_plate` across all the
            // qsv locales without lots of cfg gating, `synthesize` rolls its own
            // locale-agnostic generator in the match arm below (US-style `AAA-1234`).

            macro_rules! fake_str {
                ($faker:expr) => {
                    Some(($faker).fake_with_rng::<String, _>(rng))
                };
            }

            // `duration` (bare or "duration:N") shares one generation path with
            // a parametric cap, so handle it before the literal match keeps the
            // dispatch table simple. The cap is in seconds — bare "duration"
            // defaults to 86_400 (24 h); "duration:N" carries an LLM-inferred
            // upper bound. fake's Duration spans billions of seconds, so the
            // `% cap` bound is what makes the output realistic.
            //
            // Format is `H+:MM:SS` — minutes and seconds are always two
            // digits, but the hours component is variable-width: for the
            // 24-h default and typical LLM-picked caps (race times, session
            // durations, etc.) it renders as 2 digits, but a cap above
            // 99 h * 3600 s lets hours grow to 3+ digits. This is
            // deliberate: clamping would silently distort the requested
            // range; we keep the value faithful and accept the wider format.
            if let Some(cap) = parse_duration_cap(content_type) {
                let d: ::time::Duration = ftime::Duration().fake_with_rng(rng);
                let total = d.whole_seconds().unsigned_abs() % cap;
                return Some(format!(
                    "{:02}:{:02}:{:02}",
                    total / 3600,
                    (total % 3600) / 60,
                    total % 60
                ));
            }

            match content_type {
                // person / identity
                "first_name" => fake_str!(name::FirstName()),
                "last_name" => fake_str!(name::LastName()),
                "full_name" => fake_str!(name::Name()),
                "username" => fake_str!(net::Username()),
                "password" => fake_str!(net::Password(8..20)),
                "email" => fake_str!(net::SafeEmail()),
                "phone" => fake_str!(phone::PhoneNumber()),

                // address / location
                "street_address" => Some(format!(
                    "{} {} {}",
                    addr::BuildingNumber().fake_with_rng::<String, _>(rng),
                    addr::StreetName().fake_with_rng::<String, _>(rng),
                    addr::StreetSuffix().fake_with_rng::<String, _>(rng),
                )),
                // `street_name` is the same faker the `street_address` composite uses
                // for its middle component — kept as a separate vocab token so the LLM
                // can classify standalone street-name columns (e.g. NYC 311's `Street Name`,
                // which sits alongside but separate from `Incident Address`).
                "street_name" => fake_str!(addr::StreetName()),
                "building_number" => fake_str!(addr::BuildingNumber()),
                "secondary_address" => fake_str!(addr::SecondaryAddress()),
                "city" => fake_str!(addr::CityName()),
                "state" => fake_str!(addr::StateName()),
                "state_abbr" => fake_str!(addr::StateAbbr()),
                "zip_code" => fake_str!(addr::ZipCode()),
                "country" => fake_str!(addr::CountryName()),
                "country_code" => fake_str!(addr::CountryCode()),
                "latitude" => fake_str!(addr::Latitude()),
                "longitude" => fake_str!(addr::Longitude()),
                "time_zone" => fake_str!(addr::TimeZone()),

                // company / job
                "company_name" => fake_str!(company::CompanyName()),
                // `industry` and `profession` live under the `company` module in fake-rs,
                // not `job` (per the upstream README). They generate broader-category
                // strings — `industry` returns things like "Manufacturing" / "Healthcare",
                // `profession` returns role/occupation labels like "Doctor" / "Teacher".
                // `job_title` (fake-rs's `job::Title`) remains the most specific —
                // combinations of seniority + field + position (e.g. "Senior Backend
                // Architect").
                "industry" => fake_str!(company::Industry()),
                "job_title" => fake_str!(job::Title()),
                "profession" => fake_str!(company::Profession()),

                // identifiers / technical
                "uuid" => fake_str!(UUIDv4),
                "credit_card" => fake_str!(creditcard::CreditCardNumber()),
                "currency_code" => fake_str!(currency::CurrencyCode()),
                "isbn" => fake_str!(barcode::Isbn13()),
                // NOTE: fake-rs v5.1.0 has a determinism bug in `IP::Dummy for String`
                // (the impl ignores the passed RNG). Using `IPv4` directly — same
                // result format ("a.b.c.d"), deterministic with a seeded RNG. The
                // dedicated `IPv6` faker is used for `ipv6_address` for the same
                // reason: avoid the `IP::Dummy` randomness path entirely.
                "ip_address" => fake_str!(net::IPv4()),
                "ipv6_address" => fake_str!(net::IPv6()),
                "mac_address" => fake_str!(net::MACAddress()),
                "url" => Some(format!(
                    "https://www.{}.{}",
                    lorem::Word().fake_with_rng::<String, _>(rng),
                    net::DomainSuffix().fake_with_rng::<String, _>(rng),
                )),
                "user_agent" => fake_str!(net::UserAgent()),
                "file_name" => fake_str!(fs::FileName()),
                "file_path" => fake_str!(fs::FilePath()),
                "mime_type" => fake_str!(fs::MimeType()),
                "color_hex" => fake_str!(fcolor::HexColor()),
                // Locale-agnostic US-style plate: 3 uppercase letters + dash + 4 digits
                // (e.g. "ABC-1234"). fake-rs's `automotive::LicencePlate` exists but is
                // only implemented for PT_PT / TR_TR / FA_IR in 5.1.0 — adopting it
                // would either drop the token from EN-and-other locales or require
                // per-locale cfg gating; rolling our own keeps the token uniformly
                // available across every qsv-supported locale. Uses the same seeded
                // `rng` argument so output is deterministic with `--seed`.
                "license_plate" => {
                    // `rand 0.10` exposes `random_range` on the `RngExt` trait (renamed
                    // from `Rng::gen_range` in the 0.9->0.10 migration); the trait must
                    // be in scope at the call site for method resolution to find it.
                    use rand::RngExt;
                    let letters: String = (0..3)
                        .map(|_| (b'A' + rng.random_range(0..26)) as char)
                        .collect();
                    let digits: String = (0..4)
                        .map(|_| char::from_digit(rng.random_range(0..10), 10).unwrap())
                        .collect();
                    Some(format!("{letters}-{digits}"))
                },

                // temporal (time-of-day only; `duration` / `duration:N` is
                // handled by the pre-match check above. Plain date/datetime
                // are handled deterministically by the stats `type` column +
                // min/max in generator::build_date(), so they don't appear
                // here either.)
                "time" => fake_str!(ftime::Time()),

                // generic / fallback text
                "lorem_word" => fake_str!(lorem::Word()),
                "lorem_sentence" => fake_str!(lorem::Sentence(4..10)),
                "lorem_paragraph" => fake_str!(lorem::Paragraph(3..7)),
                "free_text" => fake_str!(lorem::Sentence(6..15)),

                // `category`, `unique_id` & `unknown` (and anything unrecognized) have no
                // faker — the caller handles them via enumeration/frequency or type-based
                // rules. NB: `unique_id` per-row uniqueness is not guaranteed by this
                // fallback (see the NON_FAKER_TOKENS comment); a dedicated unique generator
                // is a follow-up.
                _ => None,
            }
        }
    };
}

/// Single source of truth for locale support. Given a list of
/// `(VariantName, fake_rs_module, generator_fn_name)` triples, this macro
/// stamps out:
///   * the `Locale` enum (one variant per triple)
///   * `Locale::ALL` (uppercase tokens, derived via `stringify!`)
///   * `Locale::from_token` (case-insensitive lookup)
///   * one `gen_faker_for_locale!`-generated function per locale
///   * the `content_type_to_value` dispatch match
///
/// Adding a new locale = adding one row to the `define_locales!` invocation
/// below — no other place to update.
macro_rules! define_locales {
    ($( ($variant:ident, $module:ident, $fn_name:ident) ),* $(,)?) => {
        /// Locale supported by `synthesize`'s faker dispatch. Mirrors fake-rs 5.1.0's
        /// locale modules — every variant maps to one `fake::faker::*::<locale>` path.
        ///
        /// Sparse locales (those without per-category data) silently fall back to EN
        /// trait defaults at runtime; see module-level docs.
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub(crate) enum Locale {
            $($variant,)*
        }

        impl Locale {
            /// All supported locale tokens (uppercase canonical form), for USAGE help
            /// and error messages.
            pub(crate) const ALL: &'static [&'static str] = &[
                $(stringify!($variant),)*
            ];

            /// Case-insensitive parse. Returns `Err(input_as_typed)` for unknown tokens
            /// so the caller can surface the user's exact spelling in the error.
            pub(crate) fn from_token(s: &str) -> Result<Self, String> {
                let upper = s.to_ascii_uppercase();
                $(
                    if upper == stringify!($variant) {
                        return Ok(Self::$variant);
                    }
                )*
                Err(s.to_string())
            }
        }

        $(
            gen_faker_for_locale!($fn_name, $module);
        )*

        /// Generate one fake value for the given `content_type` token under `locale`,
        /// or `None` if the token has no faker mapping (`category`, `unknown`, or
        /// anything outside the vocabulary) — the caller then falls back to
        /// enumeration/type-based generation.
        pub(crate) fn content_type_to_value(
            content_type: &str,
            locale: Locale,
            rng: &mut StdRng,
        ) -> Option<String> {
            match locale {
                $( Locale::$variant => $fn_name(content_type, rng), )*
            }
        }
    };
}

define_locales! {
    (EN,    en,    content_type_to_value_en),
    (FR_FR, fr_fr, content_type_to_value_fr_fr),
    (DE_DE, de_de, content_type_to_value_de_de),
    (IT_IT, it_it, content_type_to_value_it_it),
    (PT_BR, pt_br, content_type_to_value_pt_br),
    (PT_PT, pt_pt, content_type_to_value_pt_pt),
    (JA_JP, ja_jp, content_type_to_value_ja_jp),
    (ZH_CN, zh_cn, content_type_to_value_zh_cn),
    (ZH_TW, zh_tw, content_type_to_value_zh_tw),
    (AR_SA, ar_sa, content_type_to_value_ar_sa),
    (CY_GB, cy_gb, content_type_to_value_cy_gb),
    (FA_IR, fa_ir, content_type_to_value_fa_ir),
    (NL_NL, nl_nl, content_type_to_value_nl_nl),
    (TR_TR, tr_tr, content_type_to_value_tr_tr),
}

/// Tokens that are deliberately *not* fakers — they fall through to
/// enumeration/frequency- or type-based generation.
///
/// `unique_id` is here because there is no general-purpose `fake-rs` faker that
/// guarantees per-row uniqueness across a synthesis pass. Instead, synthesize
/// uses the same fallback path as `unknown`: numeric columns go through
/// `build_numeric` (which samples from min/max or quartile buckets — so it does
/// NOT guarantee uniqueness and can emit duplicates), and string columns get
/// type-based text. A future enhancement could swap in a dedicated unique-value
/// generator (e.g. counter-backed for integers, UUIDv4 for strings) to honor
/// the `unique_id` contract; today the classification round-trips through the
/// dictionary but synthesize's output is not guaranteed unique.
const NON_FAKER_TOKENS: &[&str] = &["category", "unique_id", "unknown"];

/// Parse the duration cap (in seconds) from a `content_type` token.
///
/// Returns `Some(cap)` for:
///   * the bare `"duration"` token (defaults to 86_400 = 24 h)
///   * `"duration:N"` where N is a positive integer (LLM-inferred upper bound; whitespace around N
///     is tolerated)
///   * `"duration:<malformed>"` (non-numeric / zero / negative / empty) — degrades to the 86_400
///     default rather than `None`, so user-supplied dictionary JSON that bypasses
///     `describegpt::normalize_duration_token` still gets duration generation instead of falling
///     through to lorem fallback text
///
/// Returns `None` only when the token is not a duration token at all (no
/// `"duration"` / `"duration:"` prefix), so the caller can fall through to
/// the regular `match` dispatch.
pub(crate) fn parse_duration_cap(content_type: &str) -> Option<u64> {
    if content_type == "duration" {
        return Some(86_400);
    }
    let suffix = content_type.strip_prefix("duration:")?;
    match suffix.trim().parse::<u64>() {
        Ok(n) if n > 0 => Some(n),
        // Malformed / zero / negative suffix → degrade to the default.
        // describegpt's `normalize_duration_token` normalizes most of these
        // before they hit the dictionary, but synthesize also accepts
        // hand-crafted dictionary JSON, so we re-apply the contract here.
        _ => Some(86_400),
    }
}

/// Whether `content_type` maps to a faker (i.e. `content_type_to_value` would
/// return `Some`). Used at generator-construction time to avoid a wasted RNG
/// draw. Locale-agnostic: every faker token resolves under every locale
/// (sparse locales fall back to EN data inside fake-rs).
///
/// Also recognizes every form `parse_duration_cap` accepts — including
/// malformed `duration:N` suffixes that degrade to the default cap — so the
/// faker-vs-fallback decision stays consistent for hand-crafted dictionary
/// JSON that bypasses `describegpt::normalize_duration_token`.
pub(crate) fn is_faker_token(content_type: &str) -> bool {
    use crate::cmd::describegpt::dictionary::CONTENT_TYPE_VOCAB;

    if parse_duration_cap(content_type).is_some() {
        return true;
    }
    CONTENT_TYPE_VOCAB.contains(&content_type) && !NON_FAKER_TOKENS.contains(&content_type)
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use super::*;
    use crate::cmd::describegpt::dictionary::CONTENT_TYPE_VOCAB;

    /// Iterate every supported locale paired with a fresh seed.
    fn all_locales() -> impl Iterator<Item = Locale> {
        [
            Locale::EN,
            Locale::FR_FR,
            Locale::DE_DE,
            Locale::IT_IT,
            Locale::PT_BR,
            Locale::PT_PT,
            Locale::JA_JP,
            Locale::ZH_CN,
            Locale::ZH_TW,
            Locale::AR_SA,
            Locale::CY_GB,
            Locale::FA_IR,
            Locale::NL_NL,
            Locale::TR_TR,
        ]
        .into_iter()
    }

    #[test]
    fn locale_from_token_is_case_insensitive() {
        assert_eq!(Locale::from_token("EN").unwrap(), Locale::EN);
        assert_eq!(Locale::from_token("en").unwrap(), Locale::EN);
        assert_eq!(Locale::from_token("fr_fr").unwrap(), Locale::FR_FR);
        assert_eq!(Locale::from_token("Fr_Fr").unwrap(), Locale::FR_FR);
        assert_eq!(Locale::from_token("JA_JP").unwrap(), Locale::JA_JP);
    }

    #[test]
    fn locale_from_token_rejects_unknown() {
        let err = Locale::from_token("ZZ_ZZ").unwrap_err();
        assert_eq!(err, "ZZ_ZZ", "error should echo the user's input verbatim");
    }

    #[test]
    fn locale_all_covers_every_variant() {
        // Round-trip every token in ALL through from_token to confirm coverage.
        for token in Locale::ALL {
            Locale::from_token(token)
                .unwrap_or_else(|_| panic!("ALL contains '{token}' but from_token rejected it"));
        }
    }

    #[test]
    fn every_vocab_token_maps_or_is_known_non_faker_for_every_locale() {
        for locale in all_locales() {
            let mut rng = StdRng::seed_from_u64(42); // DevSkim: ignore DS148264
            for &token in CONTENT_TYPE_VOCAB {
                let value = content_type_to_value(token, locale, &mut rng);
                if NON_FAKER_TOKENS.contains(&token) {
                    assert!(value.is_none(), "{token}/{locale:?} should have no faker");
                    assert!(
                        !is_faker_token(token),
                        "{token} should not be a faker token"
                    );
                } else {
                    assert!(
                        value.is_some(),
                        "{token}/{locale:?} is in the vocab but has no faker mapping"
                    );
                    assert!(is_faker_token(token), "{token} should be a faker token");
                }
            }
        }
    }

    #[test]
    fn unrecognized_token_returns_none() {
        let mut rng = StdRng::seed_from_u64(1); // DevSkim: ignore DS148264
        for locale in all_locales() {
            assert!(
                content_type_to_value("definitely_not_a_token", locale, &mut rng).is_none(),
                "unknown token should return None for {locale:?}"
            );
        }
        assert!(!is_faker_token("definitely_not_a_token"));
    }

    #[test]
    fn temporal_fakers_emit_well_formed_strings() {
        // `time` should produce a time-of-day (the time crate's Display gives
        // "HH:MM:SS" or "HH:MM:SS.sssssssss"); `duration` is formatted
        // explicitly as HH:MM:SS in the faker arm. Both must be non-empty
        // and contain colon separators for every locale.
        for locale in all_locales() {
            let mut rng = StdRng::seed_from_u64(13); // DevSkim: ignore DS148264

            let time = content_type_to_value("time", locale, &mut rng)
                .unwrap_or_else(|| panic!("time/{locale:?} returned None"));
            assert!(!time.is_empty(), "time/{locale:?} produced empty string");
            assert!(
                time.matches(':').count() >= 2,
                "time/{locale:?} = {time:?} does not look like HH:MM:SS"
            );

            let duration = content_type_to_value("duration", locale, &mut rng)
                .unwrap_or_else(|| panic!("duration/{locale:?} returned None"));
            assert_eq!(
                duration.matches(':').count(),
                2,
                "duration/{locale:?} = {duration:?} must be HH:MM:SS"
            );
            // Each segment must be a 2-digit integer.
            let segments: Vec<&str> = duration.split(':').collect();
            assert_eq!(segments.len(), 3, "duration/{locale:?} = {duration:?}");
            for seg in &segments {
                assert_eq!(
                    seg.len(),
                    2,
                    "duration/{locale:?} segment {seg:?} not 2 digits in {duration:?}"
                );
                assert!(
                    seg.chars().all(|c| c.is_ascii_digit()),
                    "duration/{locale:?} segment {seg:?} not all digits in {duration:?}"
                );
            }
        }
    }

    #[test]
    fn parse_duration_cap_handles_all_forms() {
        // Bare token defaults to 24h.
        assert_eq!(parse_duration_cap("duration"), Some(86_400));
        // Well-formed suffix: any positive integer.
        assert_eq!(parse_duration_cap("duration:1"), Some(1));
        assert_eq!(parse_duration_cap("duration:3600"), Some(3_600));
        assert_eq!(parse_duration_cap("duration:31536000"), Some(31_536_000));
        // Whitespace around the number is tolerated.
        assert_eq!(parse_duration_cap("duration: 18000"), Some(18_000));
        // Malformed / zero / negative / empty suffix degrades to the 86_400
        // default — synthesize also accepts hand-crafted dictionary JSON
        // that may not have been through normalize_duration_token, so this
        // helper enforces the "degrade to bare duration" contract on its own.
        assert_eq!(parse_duration_cap("duration:0"), Some(86_400));
        assert_eq!(parse_duration_cap("duration:-5"), Some(86_400));
        assert_eq!(parse_duration_cap("duration:abc"), Some(86_400));
        assert_eq!(parse_duration_cap("duration:"), Some(86_400));
        // Non-duration tokens: still None so the caller falls through to
        // the regular faker-map dispatch.
        assert_eq!(parse_duration_cap("durationfoo"), None);
        assert_eq!(parse_duration_cap("time"), None);
        assert_eq!(parse_duration_cap("email"), None);
        assert_eq!(parse_duration_cap(""), None);
    }

    #[test]
    fn duration_suffix_caps_generated_value() {
        // With a tight cap, every generated value's whole-seconds total must
        // fit inside [0, cap). Verify across many draws so the modulo bound
        // is actually load-bearing.
        let cap_seconds: u64 = 300; // 5 minutes
        let mut rng = StdRng::seed_from_u64(99); // DevSkim: ignore DS148264
        for _ in 0..200 {
            let out = content_type_to_value("duration:300", Locale::EN, &mut rng)
                .expect("duration:300 must produce a value");
            // Format is HH:MM:SS; parse back to seconds.
            let parts: Vec<u64> = out
                .split(':')
                .map(|s| s.parse::<u64>().expect("HH:MM:SS digits"))
                .collect();
            assert_eq!(parts.len(), 3, "{out:?} not HH:MM:SS");
            let total = parts[0] * 3600 + parts[1] * 60 + parts[2];
            assert!(
                total < cap_seconds,
                "duration:300 generated {out:?} ({total}s) outside [0,{cap_seconds})"
            );
        }
    }

    #[test]
    fn is_faker_token_recognizes_duration_suffix() {
        assert!(is_faker_token("duration"));
        assert!(is_faker_token("duration:1"));
        assert!(is_faker_token("duration:86400"));
        assert!(is_faker_token("duration: 18000"));
        // Malformed suffixes ALSO count as faker tokens — they degrade
        // to the default 86_400 cap inside parse_duration_cap so that
        // hand-crafted dictionary JSON with a typo still produces fake
        // durations instead of falling through to lorem text.
        assert!(is_faker_token("duration:0"));
        assert!(is_faker_token("duration:bogus"));
        assert!(is_faker_token("duration:"));
        // Other vocab tokens still resolve correctly.
        assert!(is_faker_token("time"));
        assert!(is_faker_token("email"));
        assert!(!is_faker_token("category"));
        assert!(!is_faker_token("unknown"));
        assert!(!is_faker_token("not_a_token"));
        // Non-duration tokens that just happen to start with "duration"
        // (no colon, no exact match) are NOT faker tokens.
        assert!(!is_faker_token("durationfoo"));
    }

    #[test]
    fn new_vocab_tokens_are_faker_tokens() {
        // Defense against future drift: the 5 tokens added alongside the two-pass
        // feature MUST map to a faker, not silently fall through to type-based
        // generation (which would defeat the point of adding them).
        for tok in [
            "street_name",
            "license_plate",
            "industry",
            "profession",
            "ipv6_address",
        ] {
            assert!(
                is_faker_token(tok),
                "{tok} must be a faker token; check CONTENT_TYPE_VOCAB and gen_faker_for_locale!"
            );
        }
    }

    #[test]
    fn new_vocab_tokens_produce_non_empty_values_across_locales() {
        // Sanity check: every locale must produce SOMETHING for each new token.
        // Catches regressions where a faker was wired only under EN and silently
        // returned None / empty string under sparse locales.
        for locale in all_locales() {
            for tok in [
                "street_name",
                "license_plate",
                "industry",
                "profession",
                "ipv6_address",
            ] {
                let mut rng = StdRng::seed_from_u64(17); // DevSkim: ignore DS148264
                let value = content_type_to_value(tok, locale, &mut rng);
                let v = value.unwrap_or_else(|| {
                    panic!("{tok}/{locale:?} returned None — must yield a string")
                });
                assert!(!v.is_empty(), "{tok}/{locale:?} produced empty string");
            }
        }
    }

    #[test]
    fn license_plate_has_us_format_aaa_dash_nnnn() {
        // license_plate uses qsv's locale-agnostic generator (fake-rs's
        // automotive::LicencePlate is only implemented for PT_PT / TR_TR / FA_IR).
        // The output MUST match `AAA-NNNN` exactly: 3 uppercase ASCII letters, a dash,
        // 4 decimal digits. A regression here would mean either the generator drifted
        // or the wrong faker is being dispatched.
        for locale in all_locales() {
            let mut rng = StdRng::seed_from_u64(23); // DevSkim: ignore DS148264
            let plate = content_type_to_value("license_plate", locale, &mut rng).unwrap();
            assert_eq!(
                plate.len(),
                8,
                "{locale:?}: expected 8-char plate, got {plate:?}"
            );
            let bytes = plate.as_bytes();
            assert!(
                bytes[0..3]
                    .iter()
                    .all(|b| b.is_ascii_uppercase() && b.is_ascii_alphabetic()),
                "{locale:?}: first 3 chars must be A-Z, got {plate:?}"
            );
            assert_eq!(
                bytes[3], b'-',
                "{locale:?}: 4th char must be '-', got {plate:?}"
            );
            assert!(
                bytes[4..8].iter().all(u8::is_ascii_digit),
                "{locale:?}: last 4 chars must be 0-9, got {plate:?}"
            );
        }
    }

    #[test]
    fn ipv6_address_has_valid_shape() {
        // IPv6 output must parse as a valid std::net::Ipv6Addr — fake-rs's
        // `net::IPv6()` returns the canonical hex-colon form. Guards against a
        // future change that swaps in IPv4 by mistake (which would parse as
        // Ipv4Addr but NOT as Ipv6Addr).
        use std::net::Ipv6Addr;
        let mut rng = StdRng::seed_from_u64(31); // DevSkim: ignore DS148264
        let value = content_type_to_value("ipv6_address", Locale::EN, &mut rng).unwrap();
        value.parse::<Ipv6Addr>().unwrap_or_else(|e| {
            panic!("ipv6_address produced {value:?} which doesn't parse as Ipv6Addr: {e}")
        });
    }

    #[test]
    fn faker_is_deterministic_with_seed_for_every_locale() {
        for locale in all_locales() {
            let mut rng1 = StdRng::seed_from_u64(7); // DevSkim: ignore DS148264
            let mut rng2 = StdRng::seed_from_u64(7); // DevSkim: ignore DS148264
            for &token in CONTENT_TYPE_VOCAB {
                assert_eq!(
                    content_type_to_value(token, locale, &mut rng1),
                    content_type_to_value(token, locale, &mut rng2),
                    "{token}/{locale:?} not deterministic for a fixed seed"
                );
            }
        }
    }
}
