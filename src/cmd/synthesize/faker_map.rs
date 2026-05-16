//! Maps a dictionary `content_type` semantic token to a `fake-rs` faker that
//! produces a realistic fake value.
//!
//! The token vocabulary is `crate::cmd::describegpt::dictionary::CONTENT_TYPE_VOCAB`
//! (the 40 tokens emitted by `describegpt --infer-content-type`). Every token
//! except `category` and `unknown` maps to a faker; those two (and any token not
//! in the vocabulary) return `None` so the caller falls back to
//! enumeration/frequency- or type-based generation.
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

/// Locale supported by `synthesize`'s faker dispatch. Mirrors fake-rs 5.1.0's
/// locale modules — every variant maps to one `fake::faker::*::<locale>` path.
///
/// Sparse locales (those without per-category data) silently fall back to EN
/// trait defaults at runtime; see module-level docs.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Locale {
    EN,
    FR_FR,
    DE_DE,
    IT_IT,
    PT_BR,
    PT_PT,
    JA_JP,
    ZH_CN,
    ZH_TW,
    AR_SA,
    CY_GB,
    FA_IR,
    NL_NL,
    TR_TR,
}

impl Locale {
    /// All supported locale tokens (uppercase canonical form), for USAGE help
    /// and error messages.
    pub(crate) const ALL: &'static [&'static str] = &[
        "EN", "FR_FR", "DE_DE", "IT_IT", "PT_BR", "PT_PT", "JA_JP", "ZH_CN", "ZH_TW", "AR_SA",
        "CY_GB", "FA_IR", "NL_NL", "TR_TR",
    ];

    /// Case-insensitive parse. Returns `Err(input_as_typed)` for unknown tokens
    /// so the caller can surface the user's exact spelling in the error.
    pub(crate) fn from_token(s: &str) -> Result<Self, String> {
        let upper = s.to_ascii_uppercase();
        Ok(match upper.as_str() {
            "EN" => Self::EN,
            "FR_FR" => Self::FR_FR,
            "DE_DE" => Self::DE_DE,
            "IT_IT" => Self::IT_IT,
            "PT_BR" => Self::PT_BR,
            "PT_PT" => Self::PT_PT,
            "JA_JP" => Self::JA_JP,
            "ZH_CN" => Self::ZH_CN,
            "ZH_TW" => Self::ZH_TW,
            "AR_SA" => Self::AR_SA,
            "CY_GB" => Self::CY_GB,
            "FA_IR" => Self::FA_IR,
            "NL_NL" => Self::NL_NL,
            "TR_TR" => Self::TR_TR,
            _ => return Err(s.to_string()),
        })
    }
}

/// Stamp out one `content_type_to_value_<locale>` function bound to a specific
/// fake-rs locale module. The macro body is the entire token→faker `match` —
/// keep it in sync with `CONTENT_TYPE_VOCAB`.
macro_rules! gen_faker_for_locale {
    ($fn_name:ident, $loc:ident) => {
        #[allow(clippy::too_many_lines)]
        fn $fn_name(content_type: &str, rng: &mut StdRng) -> Option<String> {
            use fake::faker::{
                address::$loc as addr, barcode::$loc as barcode, chrono::$loc as fchrono,
                color::$loc as fcolor, company::$loc as company, creditcard::$loc as creditcard,
                currency::$loc as currency, filesystem::$loc as fs, internet::$loc as net,
                job::$loc as job, lorem::$loc as lorem, name::$loc as name,
                phone_number::$loc as phone,
            };

            macro_rules! fake_str {
                ($faker:expr) => {
                    Some(($faker).fake_with_rng::<String, _>(rng))
                };
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
                "job_title" => fake_str!(job::Title()),

                // identifiers / technical
                "uuid" => fake_str!(UUIDv4),
                "credit_card" => fake_str!(creditcard::CreditCardNumber()),
                "currency_code" => fake_str!(currency::CurrencyCode()),
                "isbn" => fake_str!(barcode::Isbn13()),
                // NOTE: fake-rs v5.1.0 has a determinism bug in `IP::Dummy for String`
                // (the impl ignores the passed RNG). Using `IPv4` directly — same
                // result format ("a.b.c.d"), deterministic with a seeded RNG.
                "ip_address" => fake_str!(net::IPv4()),
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

                // temporal
                "time" => fake_str!(fchrono::Time()),

                // generic / fallback text
                "lorem_word" => fake_str!(lorem::Word()),
                "lorem_sentence" => fake_str!(lorem::Sentence(4..10)),
                "lorem_paragraph" => fake_str!(lorem::Paragraph(3..7)),
                "free_text" => fake_str!(lorem::Sentence(6..15)),

                // `category` & `unknown` (and anything unrecognized) have no faker —
                // the caller handles them via enumeration/frequency or type-based rules.
                _ => None,
            }
        }
    };
}

gen_faker_for_locale!(content_type_to_value_en, en);
gen_faker_for_locale!(content_type_to_value_fr_fr, fr_fr);
gen_faker_for_locale!(content_type_to_value_de_de, de_de);
gen_faker_for_locale!(content_type_to_value_it_it, it_it);
gen_faker_for_locale!(content_type_to_value_pt_br, pt_br);
gen_faker_for_locale!(content_type_to_value_pt_pt, pt_pt);
gen_faker_for_locale!(content_type_to_value_ja_jp, ja_jp);
gen_faker_for_locale!(content_type_to_value_zh_cn, zh_cn);
gen_faker_for_locale!(content_type_to_value_zh_tw, zh_tw);
gen_faker_for_locale!(content_type_to_value_ar_sa, ar_sa);
gen_faker_for_locale!(content_type_to_value_cy_gb, cy_gb);
gen_faker_for_locale!(content_type_to_value_fa_ir, fa_ir);
gen_faker_for_locale!(content_type_to_value_nl_nl, nl_nl);
gen_faker_for_locale!(content_type_to_value_tr_tr, tr_tr);

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
        Locale::EN => content_type_to_value_en(content_type, rng),
        Locale::FR_FR => content_type_to_value_fr_fr(content_type, rng),
        Locale::DE_DE => content_type_to_value_de_de(content_type, rng),
        Locale::IT_IT => content_type_to_value_it_it(content_type, rng),
        Locale::PT_BR => content_type_to_value_pt_br(content_type, rng),
        Locale::PT_PT => content_type_to_value_pt_pt(content_type, rng),
        Locale::JA_JP => content_type_to_value_ja_jp(content_type, rng),
        Locale::ZH_CN => content_type_to_value_zh_cn(content_type, rng),
        Locale::ZH_TW => content_type_to_value_zh_tw(content_type, rng),
        Locale::AR_SA => content_type_to_value_ar_sa(content_type, rng),
        Locale::CY_GB => content_type_to_value_cy_gb(content_type, rng),
        Locale::FA_IR => content_type_to_value_fa_ir(content_type, rng),
        Locale::NL_NL => content_type_to_value_nl_nl(content_type, rng),
        Locale::TR_TR => content_type_to_value_tr_tr(content_type, rng),
    }
}

/// Tokens that are deliberately *not* fakers — they fall through to
/// enumeration/frequency- or type-based generation.
const NON_FAKER_TOKENS: &[&str] = &["category", "unknown"];

/// Whether `content_type` maps to a faker (i.e. `content_type_to_value` would
/// return `Some`). Used at generator-construction time to avoid a wasted RNG
/// draw. Locale-agnostic: every faker token resolves under every locale
/// (sparse locales fall back to EN data inside fake-rs).
pub(crate) fn is_faker_token(content_type: &str) -> bool {
    use crate::cmd::describegpt::dictionary::CONTENT_TYPE_VOCAB;

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
