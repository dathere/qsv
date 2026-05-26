//! CKAN-shape → DCAT-US v3 JSON-Pointer translation.
//!
//! `--initial-context` accepts three top-level subtrees:
//! `package` (CKAN dataset metadata), `resource` (the single CKAN
//! resource we ingest), and `dataset_info` (raw DCAT JSON Pointers).
//! Today only `dataset_info` overrides land in the DCAT block —
//! forced fields under `package` / `resource` are silently dropped.
//!
//! This module bridges the gap. For every CKAN slot that the
//! DCAT projection (`dcat.rs::build` and its `add_*` helpers)
//! consumes, this table records the equivalent DCAT JSON Pointer
//! inside the emitted block. `collect_forced_paths` in
//! `context.rs` consults the table to translate `force: true`
//! markings on `package` / `resource` leaves into DCAT pointers
//! that the new `apply_force_overrides` step can apply
//! unconditionally.
//!
//! Translation is intentionally one-way and best-effort:
//! - Composite DCAT fields (e.g. `dcat:contactPoint`, which `dcat::add_contact_point` builds from
//!   BOTH `package.maintainer` and `package.maintainer_email`) only translate when the user forces
//!   the composite `package.contact_point` object, not the individual scalar parts.
//! - CKAN slots that don't surface in the DCAT projection at all (e.g. `package.scheming_version`)
//!   return `None` here so a `force: true` on them is a no-op rather than a translation error.
//! - The resource index is hard-coded to `0` because profile currently emits exactly one
//!   Distribution per run.

/// Static CKAN-pointer → DCAT-pointer table. Keys are RFC 6901
/// pointers under `/package/...` or `/resource/...`; values are
/// RFC 6901 pointers under `/dcat/...`. Entries should appear in
/// the same order as their corresponding `add_*` helpers in
/// `dcat.rs` to make audit-by-grep easier.
const CKAN_TO_DCAT: &[(&str, &str)] = &[
    // ---------------------- package (Dataset-level) -----------------------
    // add_core_identity
    ("/package/title", "/dcat/dct:title"),
    ("/package/notes", "/dcat/dct:description"),
    ("/package/name", "/dcat/dct:identifier"),
    ("/package/metadata_created", "/dcat/dct:issued"),
    ("/package/metadata_modified", "/dcat/dct:modified"),
    ("/package/created", "/dcat/dct:created"),
    // add_provenance
    ("/package/publisher", "/dcat/dct:publisher"),
    ("/package/author", "/dcat/dct:publisher"),
    // add_contact_point (composite only — scalar parts are not
    // independently forceable)
    ("/package/contact_point", "/dcat/dcat:contactPoint"),
    // add_classification
    ("/package/tags", "/dcat/dcat:keyword"),
    ("/package/groups", "/dcat/dcat:theme"),
    // add_governance
    ("/package/language", "/dcat/dct:language"),
    (
        "/package/accrualPeriodicity",
        "/dcat/dct:accrualPeriodicity",
    ),
    ("/package/frequency", "/dcat/dct:accrualPeriodicity"),
    ("/package/update_frequency", "/dcat/dct:accrualPeriodicity"),
    ("/package/dcat-us:accessLevel", "/dcat/dcat-us:accessLevel"),
    ("/package/access_level", "/dcat/dcat-us:accessLevel"),
    // add_us_codes
    ("/package/bureauCode", "/dcat/dcat-us:bureauCode"),
    ("/package/programCode", "/dcat/dcat-us:programCode"),
    // add_extended_metadata
    ("/package/landing_page", "/dcat/dcat:landingPage"),
    ("/package/url", "/dcat/dcat:landingPage"),
    ("/package/data_dictionary", "/dcat/dcat:describedBy"),
    ("/package/describedBy", "/dcat/dcat:describedBy"),
    ("/package/rights", "/dcat/dct:rights"),
    ("/package/accessRights", "/dcat/dct:accessRights"),
    ("/package/access_rights", "/dcat/dct:accessRights"),
    ("/package/purpose", "/dcat/dcat-us:purpose"),
    ("/package/scopeNote", "/dcat/skos:scopeNote"),
    ("/package/scope_note", "/dcat/skos:scopeNote"),
    (
        "/package/liabilityStatement",
        "/dcat/dcat-us:liabilityStatement",
    ),
    (
        "/package/liability_statement",
        "/dcat/dcat-us:liabilityStatement",
    ),
    ("/package/inSeries", "/dcat/dcat:inSeries"),
    ("/package/in_series", "/dcat/dcat:inSeries"),
    (
        "/package/temporalResolution",
        "/dcat/dcat:temporalResolution",
    ),
    ("/package/version", "/dcat/dcat:version"),
    ("/package/versionNotes", "/dcat/dcat:versionNotes"),
    ("/package/version_notes", "/dcat/dcat:versionNotes"),
    // -------------------- resource (Distribution[0]) ----------------------
    // build_distribution
    ("/resource/name", "/dcat/dcat:distribution/0/dct:title"),
    (
        "/resource/description",
        "/dcat/dcat:distribution/0/dct:description",
    ),
    (
        "/resource/url",
        "/dcat/dcat:distribution/0/dcat:downloadURL",
    ),
    (
        "/resource/accessURL",
        "/dcat/dcat:distribution/0/dcat:accessURL",
    ),
    ("/resource/format", "/dcat/dcat:distribution/0/dct:format"),
    (
        "/resource/mediaType",
        "/dcat/dcat:distribution/0/dcat:mediaType",
    ),
    (
        "/resource/license_id",
        "/dcat/dcat:distribution/0/dct:license",
    ),
    ("/resource/license", "/dcat/dcat:distribution/0/dct:license"),
    (
        "/resource/last_modified",
        "/dcat/dcat:distribution/0/dct:modified",
    ),
    (
        "/resource/modified",
        "/dcat/dcat:distribution/0/dct:modified",
    ),
    ("/resource/rights", "/dcat/dcat:distribution/0/dct:rights"),
    (
        "/resource/language",
        "/dcat/dcat:distribution/0/dct:language",
    ),
    (
        "/resource/conformsTo",
        "/dcat/dcat:distribution/0/dct:conformsTo",
    ),
    (
        "/resource/access_restriction",
        "/dcat/dcat:distribution/0/dcat-us:accessRestriction",
    ),
    (
        "/resource/use_restriction",
        "/dcat/dcat:distribution/0/dcat-us:useRestriction",
    ),
    (
        "/resource/cui_restriction",
        "/dcat/dcat:distribution/0/dcat-us:cuiRestriction",
    ),
];

/// Translate a CKAN-shape pointer (e.g. `/package/title`,
/// `/resource/url`) to its equivalent DCAT JSON pointer
/// (e.g. `/dcat/dct:title`, `/dcat/dcat:distribution/0/dcat:downloadURL`).
///
/// Returns `None` for any CKAN pointer that doesn't have a direct
/// DCAT counterpart; `force: true` on such a leaf is a documented
/// no-op rather than a silent corruption.
#[must_use]
pub fn translate_ckan_ptr(ckan_ptr: &str) -> Option<&'static str> {
    CKAN_TO_DCAT
        .iter()
        .find_map(|(k, v)| if *k == ckan_ptr { Some(*v) } else { None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_known_package_keys_returns_expected_dcat_pointers() {
        assert_eq!(
            translate_ckan_ptr("/package/title"),
            Some("/dcat/dct:title")
        );
        assert_eq!(
            translate_ckan_ptr("/package/notes"),
            Some("/dcat/dct:description")
        );
        assert_eq!(
            translate_ckan_ptr("/package/bureauCode"),
            Some("/dcat/dcat-us:bureauCode")
        );
        assert_eq!(
            translate_ckan_ptr("/package/version"),
            Some("/dcat/dcat:version")
        );
    }

    #[test]
    fn translate_known_resource_keys_target_distribution_index_zero() {
        assert_eq!(
            translate_ckan_ptr("/resource/url"),
            Some("/dcat/dcat:distribution/0/dcat:downloadURL")
        );
        assert_eq!(
            translate_ckan_ptr("/resource/format"),
            Some("/dcat/dcat:distribution/0/dct:format")
        );
        assert_eq!(
            translate_ckan_ptr("/resource/language"),
            Some("/dcat/dcat:distribution/0/dct:language")
        );
        assert_eq!(
            translate_ckan_ptr("/resource/conformsTo"),
            Some("/dcat/dcat:distribution/0/dct:conformsTo")
        );
    }

    #[test]
    fn translate_unknown_key_returns_none() {
        // CKAN slots qsv profile does not surface in DCAT (a force
        // on these is intentionally a no-op).
        assert_eq!(translate_ckan_ptr("/package/scheming_version"), None);
        assert_eq!(translate_ckan_ptr("/package/dataset_type"), None);
        // Garbage / non-CKAN-shape pointers also return None.
        assert_eq!(translate_ckan_ptr("/foo/bar"), None);
        assert_eq!(translate_ckan_ptr(""), None);
        assert_eq!(translate_ckan_ptr("not-a-pointer"), None);
    }

    #[test]
    fn aliases_map_to_the_same_dcat_pointer() {
        // Several CKAN keys are documented aliases — verify they
        // all land at the same DCAT pointer so a future user-side
        // refactor doesn't silently split a forced override.
        for alias in &[
            "/package/accrualPeriodicity",
            "/package/frequency",
            "/package/update_frequency",
        ] {
            assert_eq!(
                translate_ckan_ptr(alias),
                Some("/dcat/dct:accrualPeriodicity"),
                "alias {alias} should map to dct:accrualPeriodicity",
            );
        }
        for alias in &["/package/data_dictionary", "/package/describedBy"] {
            assert_eq!(
                translate_ckan_ptr(alias),
                Some("/dcat/dcat:describedBy"),
                "alias {alias} should map to dcat:describedBy",
            );
        }
    }

    #[test]
    fn every_table_entry_has_distinct_ckan_key() {
        // Sanity: a duplicate CKAN key in the table would create
        // ambiguity for translate_ckan_ptr (the first-match-wins
        // shadowing would silently drop the later entry).
        let mut keys: Vec<&str> = CKAN_TO_DCAT.iter().map(|(k, _)| *k).collect();
        keys.sort_unstable();
        let len = keys.len();
        keys.dedup();
        assert_eq!(
            keys.len(),
            len,
            "duplicate CKAN pointers in CKAN_TO_DCAT table"
        );
    }

    #[test]
    fn every_dcat_pointer_starts_with_dcat_prefix() {
        // The apply_force_overrides step assumes every translated
        // pointer lives under /dcat/... so the set_by_pointer
        // machinery (which doesn't auto-create the /dcat parent)
        // doesn't accidentally create a sibling top-level slot.
        for (ckan, dcat) in CKAN_TO_DCAT {
            assert!(
                dcat.starts_with("/dcat/"),
                "{ckan} → {dcat} must target /dcat/...",
            );
        }
    }
}
