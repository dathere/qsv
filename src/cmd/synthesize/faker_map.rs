//! Maps a dictionary `content_type` semantic token to a `fake-rs` faker that
//! produces a realistic fake value.
//!
//! The token vocabulary is `crate::cmd::describegpt::dictionary::CONTENT_TYPE_VOCAB`
//! (the 40 tokens emitted by `describegpt --infer-content-type`). Every token
//! except `category` and `unknown` maps to a faker; those two (and any token not
//! in the vocabulary) return `None` so the caller falls back to
//! enumeration/frequency- or type-based generation.
//!
//! v1 is English-only (`EN` locale). The `--locale` flag is validated in
//! `super::run`; multi-locale faker dispatch is future work (each fake-rs locale
//! is a distinct type, so it needs macro-based expansion).

use fake::{
    Fake,
    faker::{
        address::en as addr, barcode::en as barcode, chrono::en as fchrono, color::en as fcolor,
        company::en as company, creditcard::en as creditcard, currency::en as currency,
        filesystem::en as fs, internet::en as net, job::en as job, lorem::en as lorem,
        name::en as name, phone_number::en as phone,
    },
    uuid::UUIDv4,
};
use rand::rngs::StdRng;

/// Generate one fake value for the given `content_type` token, or `None` if the
/// token has no faker mapping (`category`, `unknown`, or anything outside the
/// vocabulary) — the caller then falls back to enumeration/type-based generation.
pub(crate) fn content_type_to_value(content_type: &str, rng: &mut StdRng) -> Option<String> {
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

/// Tokens that are deliberately *not* fakers — they fall through to
/// enumeration/frequency- or type-based generation.
const NON_FAKER_TOKENS: &[&str] = &["category", "unknown"];

/// Whether `content_type` maps to a faker (i.e. `content_type_to_value` would
/// return `Some`). Used at generator-construction time to avoid a wasted RNG draw.
pub(crate) fn is_faker_token(content_type: &str) -> bool {
    use crate::cmd::describegpt::dictionary::CONTENT_TYPE_VOCAB;

    CONTENT_TYPE_VOCAB.contains(&content_type) && !NON_FAKER_TOKENS.contains(&content_type)
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use super::*;
    use crate::cmd::describegpt::dictionary::CONTENT_TYPE_VOCAB;

    #[test]
    fn every_vocab_token_maps_or_is_known_non_faker() {
        let mut rng = StdRng::seed_from_u64(42);
        for &token in CONTENT_TYPE_VOCAB {
            let value = content_type_to_value(token, &mut rng);
            if NON_FAKER_TOKENS.contains(&token) {
                assert!(value.is_none(), "{token} should have no faker");
                assert!(
                    !is_faker_token(token),
                    "{token} should not be a faker token"
                );
            } else {
                assert!(
                    value.is_some(),
                    "{token} is in the vocab but has no faker mapping"
                );
                assert!(is_faker_token(token), "{token} should be a faker token");
            }
        }
    }

    #[test]
    fn unrecognized_token_returns_none() {
        let mut rng = StdRng::seed_from_u64(1);
        assert!(content_type_to_value("definitely_not_a_token", &mut rng).is_none());
        assert!(!is_faker_token("definitely_not_a_token"));
    }

    #[test]
    fn faker_is_deterministic_with_seed() {
        let mut rng1 = StdRng::seed_from_u64(7);
        let mut rng2 = StdRng::seed_from_u64(7);
        for &token in CONTENT_TYPE_VOCAB {
            assert_eq!(
                content_type_to_value(token, &mut rng1),
                content_type_to_value(token, &mut rng2),
                "{token} not deterministic for a fixed seed"
            );
        }
    }
}
