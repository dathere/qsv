//! Neuro-procedural data dictionary: parsing `stats` / `frequency` CSVs into
//! records, deterministically generating dictionary entries, and merging them
//! with the LLM-produced Label + Description pairs.
//!
//! Fields on these structs are `pub(super)` so sibling submodules (e.g.
//! `formatters`) can read them; all types themselves remain crate-private.

use foldhash::{HashMap, HashMapExt, HashSet};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{CliError, CliResult, extract_json_from_output};

/// Curated, documented vocabulary of semantic Content Type tokens. Each token is
/// intended to map cleanly to a `fake-rs` faker for a future `synthesize` command.
///
/// Primitive numeric/boolean types (`integer`, `decimal`, `boolean`) are
/// deliberately excluded — they are redundant with the dictionary's deterministic
/// `type` column. `synthesize` falls back to `type` + `min`/`max` for plain
/// numeric fields whose `content_type` is `unknown`.
///
/// `time` (time-of-day, e.g. `HH:MM:SS`) and `duration` (elapsed time) ARE
/// included because qsv's stats reports them as `String`, so the deterministic
/// `type` column doesn't cover them; without these tokens `synthesize` would
/// fall through to lorem text for fields that are clearly temporal. They map
/// to `fake::faker::time::en::Time` and `Duration` respectively.
///
/// `date` and `datetime` are stamped DETERMINISTICALLY by
/// `generate_code_based_dictionary` from the stats `Type` column (`Date` /
/// `DateTime`); the LLM never classifies them. They carry an optional `:<fmt>`
/// suffix holding an LLM-inferred chrono strftime format (e.g.
/// `"datetime:%m/%d/%Y %I:%M:%S %p"`) describing the column's on-disk format.
/// The suffix is syntactically validated by `normalize_datetime_token`,
/// semantically validated against real frequency samples by
/// `validate_date_formats`, and consumed by
/// `synthesize::faker_map::parse_date_format`. A bare `date`/`datetime` falls
/// back to `synthesize`'s hardcoded `%Y-%m-%d` / RFC3339 output.
///
/// `downgrade_all_midnight_datetime_columns` reclassifies a `datetime` column
/// as `date` (stripping the time specifiers from `<fmt>`) when every
/// frequency-sampled value falls on midnight — `stats` reports `DateTime`
/// whenever any single value carries a time, which over-reports columns that
/// are plainly dates stored with a zero time-of-day.
///
/// `duration` accepts an optional `:N` suffix carrying an LLM-inferred
/// upper bound in seconds (e.g. `"duration:3600"` for an hour cap). The
/// suffix is normalized by `normalize_duration_token` and consumed by
/// `synthesize::faker_map::parse_duration_cap`. Bare `"duration"` falls
/// back to a 24-hour default at generation time.
///
/// `unique_id` marks fields where every row has a distinct non-null value
/// (i.e. stats `cardinality == rowcount`, no nulls — primary keys, surrogate
/// keys, sequence numbers). It is set DETERMINISTICALLY by
/// `generate_code_based_dictionary` via a structural check on the field's
/// frequency table (single row whose `count == cardinality`), independent of
/// the literal sentinel text, and overrides any token the LLM produced for
/// that field. The vocabulary entry is still exposed to the LLM so its
/// classification stays consistent, but the deterministic check is
/// authoritative — and LLM-supplied `unique_id` is rejected by
/// `parse_llm_dictionary_response` and `combine_dictionary_entries` to keep
/// the contract one-way.
///
/// NOTE on spellings:
/// - `license_plate` uses the AMERICAN spelling in the vocab (the user-facing surface), but maps to
///   fake-rs's `automotive::LicencePlate` (British) at the faker layer in `synthesize::faker_map`.
///   The vocab spelling is what the LLM sees and what users author by hand.
/// - `ipv6_address` mirrors the existing `ip_address` (which is IPv4 today); keeping the explicit
///   version in the token avoids ambiguity for the LLM.
pub(crate) const CONTENT_TYPE_VOCAB: &[&str] = &[
    // person / identity
    "first_name",
    "last_name",
    "full_name",
    "username",
    "password",
    "email",
    "phone",
    // address / location
    "street_address",
    "street_name",
    "building_number",
    "secondary_address",
    "city",
    "state",
    "state_abbr",
    "zip_code",
    "country",
    "country_code",
    "latitude",
    "longitude",
    "time_zone",
    // company / job
    "company_name",
    "industry",
    "job_title",
    "profession",
    // identifiers / technical
    "unique_id",
    "uuid",
    "credit_card",
    "currency_code",
    "isbn",
    "ip_address",
    "ipv6_address",
    "mac_address",
    "url",
    "user_agent",
    "file_name",
    "file_path",
    "mime_type",
    "color_hex",
    "license_plate",
    // temporal (`date`/`datetime` carry an optional LLM-inferred chrono
    // strftime `:<fmt>` suffix and are stamped deterministically from the
    // stats `Type` column - see `normalize_datetime_token`)
    "date",
    "datetime",
    "time",
    "duration",
    // generic / fallback
    "category",
    "lorem_word",
    "lorem_sentence",
    "lorem_paragraph",
    "free_text",
    "unknown",
];

/// Render `CONTENT_TYPE_VOCAB` as a comma-separated string for prompt injection.
pub(super) fn content_type_vocab_list() -> String {
    CONTENT_TYPE_VOCAB.join(", ")
}

/// Closed set of analytical column `role` tokens. Unlike `content_type` (which
/// describes the *kind of value* for synthesis/PII) and `concept` (which gives a
/// *cross-dataset semantic identity* for joining), `role` describes how an agent
/// should USE the column in a query: `dimension` (group/filter by it), `measure`
/// (aggregate it), `identifier` (a key naming an entity), or `timestamp` (a
/// date/time axis). `identifier` and `timestamp` are stamped deterministically
/// (from `is_unique_id` and the stats `Date`/`DateTime` type); `dimension` vs
/// `measure` is the LLM's call, falling back to a type-based default.
pub(crate) const ROLE_VOCAB: &[&str] = &["dimension", "measure", "identifier", "timestamp"];

/// Catalog-wide, namespaced `concept` vocabulary. A concept is the column's
/// real-world semantic identity; two columns in *different* datasets that carry
/// the SAME concept ID denote the same thing and are join-compatible. This is the
/// mechanism by which an agent correlates datasets across a catalog (decision:
/// "shared concept vocabulary only" — no explicit cross-dataset foreign keys).
///
/// Concept is distinct from `content_type`: `content_type` drives `synthesize`
/// (fake-rs fakers) and PII handling; `concept` drives catalog join discovery and
/// is hierarchical/namespaced. Many concepts are SEEDED deterministically from
/// `content_type` (see `concept_from_content_type`) so a `--infer-content-type`
/// run gets concepts for free; the LLM fills/refines the rest from this list.
///
/// Namespaces:
///   * `geo.*`   — spatial identity (join keys for places)
///   * `time.*`  — temporal axes
///   * `id.*`    — entity keys (`id.surrogate_key` is deterministic, tied to `is_unique_id`)
///   * `org.*`   — organizations
///   * `pii.*`   — sensitive personal data (drives the PII quality flag; not a join target)
///   * `measure.*`, `category.*` — quantities / categoricals (not join targets)
///   * domain prefixes (e.g. `nyc.*`) — catalog-specific shared keys
pub(crate) const CONCEPT_VOCAB: &[&str] = &[
    // spatial
    "geo.zip_code",
    "geo.city",
    "geo.state",
    "geo.country",
    "geo.latitude",
    "geo.longitude",
    "geo.coordinate_pair",
    "geo.street_address",
    "geo.census_tract",
    "geo.crs_stateplane_x",
    "geo.crs_stateplane_y",
    // temporal
    "time.event_timestamp",
    "time.created_at",
    "time.closed_at",
    "time.updated_at",
    "time.due_at",
    "time.date",
    "time.duration",
    // identifiers
    "id.surrogate_key",
    "id.natural_key",
    "id.foreign_key",
    "id.uuid",
    // organizations
    "org.agency",
    "org.company",
    "org.industry",
    // sensitive personal data
    "pii.email",
    "pii.phone",
    "pii.full_name",
    "pii.address",
    // quantities / categoricals
    "measure.count",
    "measure.amount",
    "measure.ratio",
    "category.status",
    "category.type",
    "category.channel",
    // NYC-domain extension (illustrative shared catalog keys)
    "nyc.bbl",
    "nyc.borough",
    "nyc.community_board",
    "nyc.complaint_type",
    // fallback
    "unknown",
];

/// Render `CONCEPT_VOCAB` as a comma-separated string for prompt injection.
pub(super) fn concept_vocab_list() -> String {
    CONCEPT_VOCAB.join(", ")
}

/// Render `ROLE_VOCAB` as a comma-separated string for prompt injection.
pub(super) fn role_vocab_list() -> String {
    ROLE_VOCAB.join(", ")
}

/// Concept namespaces that denote a shared real-world entity an agent can join
/// ON across datasets. `pii.*`, `measure.*`, `category.*`, and `unknown` are
/// excluded: they are sensitive, additive, or not stable identities.
const LINKABLE_CONCEPT_PREFIXES: &[&str] = &["geo.", "time.", "id.", "org.", "nyc."];

/// True when `concept` names a cross-dataset-joinable identity (drives the
/// schema-table `Join?` hint and the per-column join line).
pub(super) fn is_linkable_concept(concept: &str) -> bool {
    !concept.is_empty()
        && concept != "unknown"
        && LINKABLE_CONCEPT_PREFIXES
            .iter()
            .any(|p| concept.starts_with(p))
}

/// Deterministic seed mapping from a (base) `content_type` token to a catalog
/// `concept`, so a `--infer-content-type` run gets concepts without extra LLM
/// work. Returns `None` for tokens with no obvious 1:1 concept (the LLM fills
/// those). Caller passes the bare token (no `:<fmt>`/`:N` suffix).
pub(super) fn concept_from_content_type(content_type_base: &str) -> Option<&'static str> {
    Some(match content_type_base {
        "unique_id" => "id.surrogate_key",
        "uuid" => "id.uuid",
        "zip_code" => "geo.zip_code",
        "city" => "geo.city",
        "state" | "state_abbr" => "geo.state",
        "country" | "country_code" => "geo.country",
        "latitude" => "geo.latitude",
        "longitude" => "geo.longitude",
        "street_address" | "street_name" => "geo.street_address",
        "email" => "pii.email",
        "phone" => "pii.phone",
        "first_name" | "last_name" | "full_name" => "pii.full_name",
        "company_name" => "org.company",
        "industry" => "org.industry",
        "date" => "time.date",
        "datetime" => "time.event_timestamp",
        _ => return None,
    })
}

/// Merge an LLM-supplied `concept` onto the (possibly deterministically seeded)
/// `current` value. `id.surrogate_key` is deterministic (tied to `is_unique_id`)
/// and never overridden. Otherwise the LLM fills an empty slot, and on the
/// refine pass (`allow_override`) may replace a non-authoritative seed.
fn merge_concept(current: &str, llm: &str, allow_override: bool) -> String {
    if current == "id.surrogate_key" {
        return current.to_string();
    }
    if !llm.is_empty() && llm != "id.surrogate_key" && (current.is_empty() || allow_override) {
        return llm.to_string();
    }
    current.to_string()
}

/// Merge an LLM-supplied `role` onto the (possibly deterministically seeded)
/// `current` value. `identifier` and `timestamp` are stamped deterministically
/// and never overridden; `dimension`/`measure` come from the LLM (or the
/// type-based fallback applied in `combine_dictionary_entries`).
fn merge_role(current: &str, llm: &str, allow_override: bool) -> String {
    if current == "identifier" || current == "timestamp" {
        return current.to_string();
    }
    if ROLE_VOCAB.contains(&llm) && (current.is_empty() || allow_override) {
        return llm.to_string();
    }
    current.to_string()
}

/// Normalize the `duration` content type, optionally with an LLM-inferred
/// per-field upper-bound suffix.
///
/// Accepts:
///   * `"duration"` → returns `Some("duration")` (synthesize falls back to its default 24-hour cap)
///   * `"duration:N"` where N is a positive integer → returns `Some("duration:N")` (whitespace
///     around N is tolerated)
///   * `"duration:<malformed>"` (non-numeric / zero) → returns `Some("duration")` so a bad suffix
///     degrades gracefully to the unbounded form rather than dropping the classification entirely
///
/// Returns `None` for anything that isn't a duration token, so the caller can
/// fall back to the regular `CONTENT_TYPE_VOCAB` membership check.
///
/// Caller is responsible for lowercasing / outer-trimming the input first.
pub(super) fn normalize_duration_token(raw: &str) -> Option<String> {
    if raw == "duration" {
        return Some("duration".to_string());
    }
    let suffix = raw.strip_prefix("duration:")?;
    match suffix.trim().parse::<u64>() {
        Ok(n) if n > 0 => Some(format!("duration:{n}")),
        _ => Some("duration".to_string()),
    }
}

/// True if `fmt` is a syntactically valid chrono strftime format string, i.e.
/// it contains no unrecognized `%` specifiers. A plain literal with no `%` is
/// considered valid here; semantic mismatches are caught later by
/// `validate_date_formats` parsing a real sample value.
pub(crate) fn is_valid_strftime(fmt: &str) -> bool {
    use chrono::format::{Item, StrftimeItems};
    StrftimeItems::new(fmt).all(|item| !matches!(item, Item::Error))
}

/// Normalize a `date` / `datetime` content type, optionally carrying an
/// LLM-inferred chrono strftime format suffix.
///
/// Accepts:
///   * `"date"` / `"datetime"` → returns the bare token
///   * `"date:<fmt>"` / `"datetime:<fmt>"` → returns the token verbatim when `<fmt>` is a
///     syntactically valid chrono strftime string; an empty or malformed `<fmt>` degrades to the
///     bare token (mirroring how `normalize_duration_token` degrades a bad `:N`)
///
/// Returns `None` for anything that is not a date/datetime token, so the caller
/// can fall back to the regular `CONTENT_TYPE_VOCAB` membership check.
///
/// Unlike `normalize_duration_token`, the input must NOT be fully lowercased by
/// the caller: chrono strftime specifiers are case-sensitive (`%m` month vs
/// `%M` minute, `%p` AM/PM). Only the token prefix (before the first `:`) is
/// case-folded here; the `<fmt>` suffix is preserved verbatim. Splitting on the
/// first `:` keeps colons inside the format (e.g. `%H:%M:%S`) intact.
pub(super) fn normalize_datetime_token(raw: &str) -> Option<String> {
    let raw = raw.trim();
    let (head, fmt) = match raw.split_once(':') {
        Some((h, f)) => (h, Some(f)),
        None => (raw, None),
    };
    let token = match head.to_ascii_lowercase().as_str() {
        "date" => "date",
        "datetime" => "datetime",
        _ => return None,
    };
    match fmt {
        Some(fmt) if !fmt.is_empty() && is_valid_strftime(fmt) => Some(format!("{token}:{fmt}")),
        _ => Some(token.to_string()),
    }
}

/// Returns the bare-token prefix of a content_type, i.e. the part before the
/// first `:` (`"datetime:%F"` → `"datetime"`, `"duration:3600"` → `"duration"`).
/// Non-suffixed tokens are returned unchanged.
pub(super) fn content_type_base(token: &str) -> &str {
    token.split(':').next().unwrap_or(token)
}

/// Extract the strftime format suffix from a `date`/`datetime` `content_type`,
/// when present and syntactically valid (e.g. `"date:%m/%d/%Y"` →
/// `Some("%m/%d/%Y")`). Bare tokens and non-date content types yield `None`.
pub(super) fn content_type_date_format(content_type: &str) -> Option<&str> {
    let fmt = content_type
        .strip_prefix("datetime:")
        .or_else(|| content_type.strip_prefix("date:"))?;
    (!fmt.is_empty() && is_valid_strftime(fmt)).then_some(fmt)
}

/// Reformat an (RFC 3339-normalized) date string `value` using the strftime
/// format carried by `content_type`, when present. Returns `value` unchanged
/// when there is no inferred format, the value is empty, or it cannot be parsed.
///
/// This mirrors the markdown dictionary template's `datefmt` filter so the
/// JSON/TOON/JSONSchema outputs present date Min/Max in the same inferred
/// format as the markdown dictionary, instead of qsv's normalized RFC 3339.
pub(super) fn format_date_value<'a>(
    content_type: &str,
    value: &'a str,
) -> std::borrow::Cow<'a, str> {
    use std::borrow::Cow;
    if value.is_empty() {
        return Cow::Borrowed(value);
    }
    let Some(fmt) = content_type_date_format(content_type) else {
        return Cow::Borrowed(value);
    };
    // Disambiguate the parse using the inferred format's own field order: when
    // `%d` precedes `%m` the column is day-first, so an ambiguous value like
    // "01/02/2020" under `date:%d/%m/%Y` parses as 1 Feb (not 2 Jan) and
    // round-trips unchanged instead of being silently swapped. Min/Max are
    // RFC3339 (preference-invariant), so this only matters for raw Examples.
    match qsv_dateparser::parse_with_preference(value, prefer_dmy(fmt)) {
        Ok(dt) => Cow::Owned(dt.format(fmt).to_string()),
        Err(_) => Cow::Borrowed(value),
    }
}

/// Whether a strftime format is day-first (the day field precedes the month
/// field), used to set `qsv_dateparser`'s DMY-vs-MDY preference when
/// reformatting ambiguous values. Parses the format with chrono's
/// `StrftimeItems` so every day/month padding variant (`%d`, `%e`, `%-d`,
/// `%0d`, `%_d`, `%m`, `%-m`, `%0m`, `%_m`, …) is recognized. Defaults to
/// `false` (month-first) when the order can't be determined.
fn prefer_dmy(fmt: &str) -> bool {
    use chrono::format::{Item, Numeric, StrftimeItems};
    let mut day_idx = None;
    let mut month_idx = None;
    for (i, item) in StrftimeItems::new(fmt).enumerate() {
        match item {
            Item::Numeric(Numeric::Day, _) if day_idx.is_none() => day_idx = Some(i),
            Item::Numeric(Numeric::Month, _) if month_idx.is_none() => month_idx = Some(i),
            _ => {},
        }
    }
    matches!((day_idx, month_idx), (Some(d), Some(m)) if d < m)
}

/// Reformat the value part of each `"value [count]"` example line to the
/// `content_type`'s inferred date format, so the Examples column reads
/// consistently with the date-formatted Min/Max. Passthrough when
/// `content_type` carries no date format and on the `<ALL_UNIQUE>` sentinel.
/// The trailing ` [count]` suffix is preserved verbatim, and values that
/// cannot be parsed (frequency aggregation buckets like `Other…`/`(NULL)…`,
/// truncated `value…` entries) are left unchanged by `format_date_value`.
pub(super) fn format_date_examples(content_type: &str, examples: &str) -> String {
    if examples == "<ALL_UNIQUE>" || content_type_date_format(content_type).is_none() {
        return examples.to_string();
    }
    examples
        .lines()
        .map(|line| {
            // Split off a trailing " [count]" suffix (rfind so values that
            // themselves contain "[" aren't truncated mid-value), reformat the
            // value, then rejoin value + suffix.
            if let Some(idx) = line.rfind(" [")
                && line.ends_with(']')
            {
                let (value, suffix) = line.split_at(idx);
                format!("{}{suffix}", format_date_value(content_type, value))
            } else {
                format_date_value(content_type, line).into_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// LLM-inferred fields for a single dictionary column, keyed by field name in the
/// map returned by `parse_llm_dictionary_response`. `content_type` stays empty
/// unless `--infer-content-type` is set.
#[derive(Debug, Clone, Default)]
pub(super) struct LlmDictField {
    pub(super) label:        String,
    pub(super) description:  String,
    pub(super) content_type: String,
    /// Catalog concept ID (e.g. `geo.zip_code`); empty unless concept inference
    /// is on. Validated against `CONCEPT_VOCAB` in `parse_llm_dictionary_response`.
    pub(super) concept:      String,
    /// Analytical role (`dimension`/`measure`/`identifier`/`timestamp`); empty
    /// unless concept inference is on. Validated against `ROLE_VOCAB`.
    pub(super) role:         String,
}

pub(crate) struct StatsRecord {
    pub(crate) field:       String,
    pub(crate) r#type:      String,
    pub(crate) cardinality: u64,
    pub(crate) nullcount:   u64,
    pub(crate) min:         String, // Empty string if not available
    pub(crate) max:         String, // Empty string if not available
    pub(crate) addl_cols:   IndexMap<String, String>, // Additional columns (preserves CSV order)
}

pub(crate) struct FrequencyRecord {
    pub(crate) field:      String,
    pub(crate) value:      String,
    pub(crate) count:      u64,
    pub(crate) percentage: f64,
    pub(crate) rank:       f64,
}

/// One weighted frequency sample for a field, carrying the qsv-computed
/// `percentage` and `rank` that the flat `examples` string discards. Populated
/// for the same top-N values shown in `examples`; consumed by the SemanticMd
/// formatter to render richer Frequency tables. `value` is already display-
/// formatted (bucket "…" suffix + `truncate_str` truncation) to match `examples`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct FreqDetail {
    pub(super) value:      String,
    pub(super) count:      u64,
    pub(super) percentage: f64,
    /// Dense rank from `frequency`; `0.0` for aggregation buckets (`Other…`/`(NULL)…`).
    pub(super) rank:       f64,
}

/// One row in the generated data dictionary. `label`, `description` and
/// `content_type` start empty and are filled by the LLM pass (`content_type`
/// only when `--infer-content-type` is set); all other fields are populated
/// deterministically from `StatsRecord` + `FrequencyRecord`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DictionaryEntry {
    pub(super) name:         String,
    pub(super) r#type:       String,
    pub(super) label:        String,
    pub(super) description:  String,
    pub(super) content_type: String, // Curated semantic token; empty unless --infer-content-type
    pub(super) min:          String, // Empty string if not available
    pub(super) max:          String, // Empty string if not available
    pub(super) cardinality:  u64,
    pub(super) enumeration:  String, // Empty if not enumerable, otherwise one value per line
    pub(super) null_count:   u64,
    pub(super) addl_cols:    IndexMap<String, String>, // Preserves column order
    pub(super) examples:     String,                   /* Format: "val1 [cnt1]\nval2 [cnt2]…" or
                                                        * "<ALL_UNIQUE>" */
    /// Structured counterpart to `examples`, retaining per-value percentage and rank.
    /// `#[serde(default)]` keeps older cached dictionaries (written before this field
    /// existed) deserializable.
    #[serde(default)]
    pub(super) freq_details: Vec<FreqDetail>,
    /// Structural "every row carries a distinct non-null value" flag (`cardinality ==
    /// rowcount`, no nulls), computed deterministically at generation time. Distinct
    /// from the overloaded `examples == "<ALL_UNIQUE>"` sentinel (which is also set for
    /// constant-value and HIGH_CARDINALITY columns at 100% frequency). Consumed by the
    /// SemanticMd formatter for primary-key inference. `#[serde(default)]` for cache
    /// backward-compatibility.
    #[serde(default)]
    pub(super) is_unique_id: bool,
    /// Catalog-wide semantic identity used for cross-dataset join discovery
    /// (e.g. `geo.zip_code`). Deterministically seeded from `content_type` and
    /// refined by the LLM; empty unless content-type/concept inference is on.
    /// `#[serde(default)]` for cache backward-compatibility.
    #[serde(default)]
    pub(super) concept:      String,
    /// Analytical role: `dimension`, `measure`, `identifier`, or `timestamp`.
    /// `identifier`/`timestamp` are deterministic; the rest are LLM-filled with a
    /// type-based fallback. `#[serde(default)]` for cache backward-compatibility.
    #[serde(default)]
    pub(super) role:         String,
}

/// Parse the `stats` CSV into structured records, returning the records plus
/// the ordered list of additional (non-standard) column names in CSV order.
pub(crate) fn parse_stats_csv(stats_csv: &str) -> CliResult<(Vec<StatsRecord>, Vec<String>)> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_csv.as_bytes());

    let headers = rdr.headers()?.clone();

    let std_cols: HashSet<&str> = ["field", "type", "cardinality", "nullcount", "min", "max"]
        .iter()
        .copied()
        .collect();

    let field_idx = headers
        .iter()
        .position(|h| h == "field")
        .ok_or_else(|| CliError::Other("Stats CSV missing 'field' column".to_string()))?;

    let type_idx = headers
        .iter()
        .position(|h| h == "type")
        .ok_or_else(|| CliError::Other("Stats CSV missing 'type' column".to_string()))?;

    let cardinality_idx = headers.iter().position(|h| h == "cardinality");
    let nullcount_idx = headers
        .iter()
        .position(|h| h == "nullcount")
        .ok_or_else(|| CliError::Other("Stats CSV missing 'nullcount' column".to_string()))?;
    let min_idx = headers.iter().position(|h| h == "min");
    let max_idx = headers.iter().position(|h| h == "max");

    let addl_col_indices: Vec<(usize, String)> = headers
        .iter()
        .enumerate()
        .filter_map(|(idx, header)| {
            if std_cols.contains(header) {
                None
            } else {
                Some((idx, header.to_string()))
            }
        })
        .collect();

    let mut records = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let field = record
            .get(field_idx)
            .ok_or_else(|| CliError::Other("Stats CSV record missing field value".to_string()))?
            .to_string();

        let r#type = record
            .get(type_idx)
            .ok_or_else(|| CliError::Other("Stats CSV record missing type value".to_string()))?
            .to_string();

        let cardinality = cardinality_idx
            .and_then(|idx| record.get(idx))
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let nullcount = record
            .get(nullcount_idx)
            .ok_or_else(|| CliError::Other("Stats CSV record missing nullcount value".to_string()))?
            .parse::<u64>()
            .map_err(|e| CliError::Other(format!("Failed to parse nullcount: {e}")))?;

        let min = min_idx
            .and_then(|idx| record.get(idx))
            .map(std::string::ToString::to_string)
            .unwrap_or_default();

        let max = max_idx
            .and_then(|idx| record.get(idx))
            .map(std::string::ToString::to_string)
            .unwrap_or_default();

        let mut addl_cols = IndexMap::new();
        for (idx, col_name) in &addl_col_indices {
            let value = record
                .get(*idx)
                .map(std::string::ToString::to_string)
                .unwrap_or_default();
            addl_cols.insert(col_name.clone(), value);
        }

        records.push(StatsRecord {
            field,
            r#type,
            cardinality,
            nullcount,
            min,
            max,
            addl_cols,
        });
    }

    let ordered_col_names: Vec<String> =
        addl_col_indices.into_iter().map(|(_, name)| name).collect();

    Ok((records, ordered_col_names))
}

/// Parse the `frequency` CSV into structured records.
pub(crate) fn parse_frequency_csv(frequency_csv: &str) -> CliResult<Vec<FrequencyRecord>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(frequency_csv.as_bytes());

    let headers = rdr.headers()?.clone();

    let field_idx = headers
        .iter()
        .position(|h| h == "field")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'field' column".to_string()))?;

    let value_idx = headers
        .iter()
        .position(|h| h == "value")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'value' column".to_string()))?;

    let count_idx = headers
        .iter()
        .position(|h| h == "count")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'count' column".to_string()))?;

    let percentage_idx = headers
        .iter()
        .position(|h| h == "percentage")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'percentage' column".to_string()))?;

    let rank_idx = headers
        .iter()
        .position(|h| h == "rank")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'rank' column".to_string()))?;

    let mut records = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let field = record
            .get(field_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing field value".to_string()))?
            .to_string();

        let value = record
            .get(value_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing value".to_string()))?
            .to_string();

        let count = record
            .get(count_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing count".to_string()))
            .and_then(|s| {
                if s.is_empty() {
                    Ok(0)
                } else {
                    s.parse::<u64>().map_err(|e| {
                        CliError::Other(format!("Failed to parse count in frequency CSV: {e}"))
                    })
                }
            })?;

        let percentage = record
            .get(percentage_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing percentage".to_string()))
            .and_then(|s| {
                if s.is_empty() {
                    Ok(0.0)
                } else {
                    s.parse::<f64>().map_err(|e| {
                        CliError::Other(format!("Failed to parse percentage in frequency CSV: {e}"))
                    })
                }
            })?;

        let rank = record
            .get(rank_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing rank".to_string()))
            .and_then(|s| {
                if s.is_empty() {
                    Ok(0.0)
                } else {
                    s.parse::<f64>().map_err(|e| {
                        CliError::Other(format!("Failed to parse rank in frequency CSV: {e}"))
                    })
                }
            })?;

        records.push(FrequencyRecord {
            field,
            value,
            count,
            percentage,
            rank,
        });
    }

    Ok(records)
}

/// Generate dictionary entries deterministically from `stats` + `frequency`
/// data. Label and Description are left empty for the LLM pass to fill.
///
/// When `infer_content_type` is set, `content_type` is also deterministically
/// pre-set to `"unique_id"` for fields where `cardinality == rowcount` with no
/// nulls. The detection is structural (single frequency row whose
/// `count == cardinality`), not text-matching, so it works regardless of the
/// `frequency --all-unique-text` setting and won't be confused by fields whose
/// values literally contain the string `<ALL_UNIQUE>`. All other fields'
/// `content_type` stays empty and is filled by the LLM pass.
pub(super) fn generate_code_based_dictionary(
    stats_records: &[StatsRecord],
    frequency_records: &[FrequencyRecord],
    enum_threshold: usize,
    num_examples: u16,
    truncate_str: usize,
    addl_cols: &[String],
    infer_content_type: bool,
) -> Vec<DictionaryEntry> {
    let mut frequency_by_field: HashMap<String, Vec<&FrequencyRecord>> = HashMap::new();
    for freq_record in frequency_records {
        frequency_by_field
            .entry(freq_record.field.clone())
            .or_default()
            .push(freq_record);
    }

    let mut dictionary_entries = Vec::new();

    for stats_record in stats_records {
        let field_name = &stats_record.field;
        let field_frequencies = frequency_by_field
            .get(field_name)
            .cloned()
            .unwrap_or_default();

        let enumeration = if stats_record.cardinality <= enum_threshold as u64 {
            // Check for rank=0 "Other" bucket or <ALL_UNIQUE> sentinel
            let has_other = field_frequencies
                .iter()
                .any(|f| f.rank == 0.0 && !f.value.contains("<ALL_UNIQUE>"));
            if has_other {
                String::new()
            } else {
                let mut enum_values: Vec<String> = field_frequencies
                    .iter()
                    .filter(|f| !f.value.contains("<ALL_UNIQUE>"))
                    .map(|f| f.value.clone())
                    .collect();
                enum_values.sort();
                enum_values.join("\n")
            }
        } else {
            String::new()
        };

        let (examples, freq_details) = if field_frequencies
            .iter()
            .any(|f| (f.percentage - 100.0).abs() < 0.0001)
        {
            ("<ALL_UNIQUE>".to_string(), Vec::new())
        } else {
            let mut sorted_freqs = field_frequencies.clone();
            sorted_freqs.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.value.cmp(&b.value)));

            let mut top_n: Vec<String> = Vec::new();
            let mut details: Vec<FreqDetail> = Vec::new();
            for f in sorted_freqs.iter().take(num_examples as usize) {
                // For frequency bucket entries (rank == 0.0), strip the redundant
                // "(n)" count and append "…" to disambiguate from literal values with
                // the same name (e.g. bucket "Other… [4,091]" vs literal "Other [2,006]")
                let raw_value = if f.rank == 0.0 {
                    let base = if let Some(pos) = f.value.rfind(" (") {
                        &f.value[..pos]
                    } else {
                        &f.value
                    };
                    format!("{base}…")
                } else {
                    f.value.clone()
                };

                let v = if truncate_str > 0 && raw_value.chars().count() > truncate_str {
                    let mut s = raw_value.chars().take(truncate_str).collect::<String>();
                    s.push('…');
                    s
                } else {
                    raw_value
                };
                top_n.push(format!("{} [{}]", v, f.count));
                details.push(FreqDetail {
                    value:      v,
                    count:      f.count,
                    percentage: f.percentage,
                    rank:       f.rank,
                });
            }

            (top_n.join("\n"), details)
        };

        let mut entry_addl_cols = IndexMap::new();
        for col_name in addl_cols {
            if let Some(value) = stats_record.addl_cols.get(col_name) {
                entry_addl_cols.insert(col_name.clone(), value.clone());
            }
        }

        // Deterministically classify fields where every row carries a distinct
        // non-null value as `unique_id`. qsv's frequency command emits an
        // ALL_UNIQUE sentinel row exactly when `cardinality == rowcount` (a
        // single row with `count == rowcount` and `percentage == 100.0`). We
        // detect that structurally — `len() == 1` + `count == cardinality` —
        // rather than matching the literal `<ALL_UNIQUE>` text so that:
        //   - a real field whose values literally contain the string "<ALL_UNIQUE>" isn't
        //     mislabeled (constants have `cardinality == 1`; mixed fields produce more than one
        //     frequency row);
        //   - a custom `frequency --all-unique-text` sentinel is still detected correctly (the text
        //     doesn't matter, only the structural cardinality==count invariant does);
        //   - HIGH_CARDINALITY sentinel rows (also single-row, percentage 100.0, count == rowcount)
        //     are excluded because for them `cardinality < rowcount == count`.
        // Also requires `cardinality > 1` and `nullcount == 0` to enforce
        // the semantic contract (every row has a distinct non-null value), and that the
        // single row covers the whole column (`percentage == 100.0`). The percentage
        // guard rejects truncated/custom frequency data (e.g. `--limit 1 --no-other` or a
        // `file:` frequency CSV) where the lone emitted top row's `count` coincidentally
        // equals the column cardinality even though the column is not unique.
        // Pre-set value takes precedence over whatever the LLM returns (see
        // `combine_dictionary_entries`). Only populate when `--infer-content-type`
        // is on; otherwise the `content_type` column is suppressed entirely.
        let is_all_unique = stats_record.cardinality > 1
            && stats_record.nullcount == 0
            && field_frequencies.len() == 1
            && field_frequencies[0].count == stats_record.cardinality
            && (field_frequencies[0].percentage - 100.0).abs() < 0.0001;
        // Deterministically stamp the bare `content_type` token. `unique_id`
        // keeps priority over `date`/`datetime` (a unique timestamp key is
        // still a key). `date`/`datetime` are derived from the stats `Type`
        // column; the LLM later supplies only the optional strftime `:<fmt>`
        // suffix (merged in `combine_dictionary_entries`).
        let content_type = if !infer_content_type {
            String::new()
        } else if is_all_unique {
            "unique_id".to_string()
        } else {
            match stats_record.r#type.as_str() {
                "Date" => "date".to_string(),
                "DateTime" => "datetime".to_string(),
                _ => String::new(),
            }
        };

        // Deterministically seed `concept` + `role`, gated on the same flag as
        // `content_type`. `concept` seeds from the content_type token (e.g.
        // `zip_code` → `geo.zip_code`); `role` from the structural unique-id flag
        // (`identifier`) and the stats Date/DateTime type (`timestamp`). The LLM
        // fills/refines empty or non-authoritative values in
        // `combine_dictionary_entries`.
        let (concept, role) = if !infer_content_type {
            (String::new(), String::new())
        } else {
            let concept = concept_from_content_type(content_type_base(&content_type))
                .map(ToString::to_string)
                .unwrap_or_default();
            let role = if is_all_unique {
                "identifier".to_string()
            } else {
                match stats_record.r#type.as_str() {
                    "Date" | "DateTime" => "timestamp".to_string(),
                    _ => String::new(),
                }
            };
            (concept, role)
        };

        dictionary_entries.push(DictionaryEntry {
            name: stats_record.field.clone(),
            r#type: stats_record.r#type.clone(),
            label: String::new(),       // Filled by LLM
            description: String::new(), // Filled by LLM
            content_type,               /* Pre-set to "unique_id" for ALL_UNIQUE fields;
                                         * otherwise filled by LLM */
            min: stats_record.min.clone(),
            max: stats_record.max.clone(),
            cardinality: stats_record.cardinality,
            enumeration,
            null_count: stats_record.nullcount,
            addl_cols: entry_addl_cols,
            examples,
            freq_details,
            is_unique_id: is_all_unique,
            concept,
            role,
        });
    }

    dictionary_entries
}

/// Merge a single LLM-supplied `content_type` onto the (possibly
/// deterministically pre-stamped) `current` value, returning the value the
/// entry should hold. Encodes the whole `content_type` merge contract:
///
/// * A code-stamped `date`/`datetime` column (`current` base is `date`/`datetime`) only accepts an
///   LLM value that agrees on the base token AND carries a strftime `:<fmt>` suffix — the LLM may
///   add or correct the format but never reclassify the column or wipe a known format back to bare.
/// * `unique_id` is deterministic — the LLM can never override it.
/// * The LLM may never introduce a `date`/`datetime` token onto a column the stats did not type as
///   such (only the deterministic stamp may), and never supply `unique_id`.
/// * Otherwise an LLM token is written into an empty slot; when `allow_override` is set (the refine
///   pass) it may also replace an existing non-`unique_id`, non-date token.
fn merge_content_type(current: &str, llm: &str, allow_override: bool) -> String {
    let current_base = content_type_base(current);
    if current_base == "date" || current_base == "datetime" {
        if llm.contains(':') && content_type_base(llm) == current_base {
            return llm.to_string();
        }
        return current.to_string();
    }
    if current == "unique_id" {
        return current.to_string();
    }
    if !llm.is_empty()
        && llm != "unique_id"
        && !matches!(content_type_base(llm), "date" | "datetime")
        && (current.is_empty() || allow_override)
    {
        return llm.to_string();
    }
    current.to_string()
}

/// Merge code-generated dictionary entries with the LLM-generated fields (Label,
/// Description and, when `--infer-content-type` is set, Content Type) keyed by
/// field name.
///
/// `content_type` merge contract — see `merge_content_type`:
/// - Code-derived `content_type` always wins. `generate_code_based_dictionary` stamps `"unique_id"`
///   on `<ALL_UNIQUE>` fields and bare `"date"`/`"datetime"` on `Date`/`DateTime`-typed fields.
/// - For a code-stamped `date`/`datetime` field the LLM may only contribute the strftime `:<fmt>`
///   suffix; it can neither reclassify the column nor introduce a `date`/`datetime` token on a
///   non-date column.
/// - LLM-supplied `"unique_id"` is refused (defense in depth — the parser also strips it).
///
/// When `infer_content_type` is set, this is the single point that guarantees
/// every entry has a non-empty `content_type`: any field the LLM classified
/// with an invalid token, omitted the `content_type` key for, or left out of
/// its response entirely falls back to `"unknown"`.
pub(super) fn combine_dictionary_entries(
    mut code_entries: Vec<DictionaryEntry>,
    llm_fields: &HashMap<String, LlmDictField>,
    infer_content_type: bool,
) -> Vec<DictionaryEntry> {
    for entry in &mut code_entries {
        if let Some(llm) = llm_fields.get(&entry.name) {
            entry.label = llm.label.clone();
            entry.description = llm.description.clone();
            entry.content_type = merge_content_type(&entry.content_type, &llm.content_type, false);
            entry.concept = merge_concept(&entry.concept, &llm.concept, false);
            entry.role = merge_role(&entry.role, &llm.role, false);
        }
        if infer_content_type {
            if entry.content_type.is_empty() {
                entry.content_type = "unknown".to_string();
            }
            coerce_role_concept(entry);
        }
    }
    code_entries
}

/// Final fallback coercion for `role`/`concept` once the LLM merge(s) are done,
/// mirroring the `content_type` → `"unknown"` coercion. Applied only when
/// concept inference is on. Empty `concept` becomes `"unknown"` (excluded from
/// the front-matter concept index and from join detection); empty `role`
/// defaults to `measure` for numeric columns and `dimension` otherwise.
fn coerce_role_concept(entry: &mut DictionaryEntry) {
    if entry.concept.is_empty() {
        entry.concept = "unknown".to_string();
    }
    if entry.role.is_empty() {
        entry.role = if entry.r#type == "Integer" || entry.r#type == "Float" {
            "measure".to_string()
        } else {
            "dimension".to_string()
        };
    }
}

/// Two-pass-aware merge: seed `code_entries` with the BASELINE LLM Label / Description /
/// Content Type (from the first pass) and then overlay the REFINE pass's LLM fields on top.
/// If the refine pass omits a field, the baseline Label / Description / Content Type are
/// preserved — this is the critical correctness invariant for `--two-pass`. Without it, a
/// refine response that returns a subset of fields would silently wipe the first-pass
/// human-friendly fields back to code-derived defaults.
///
/// The `content_type` merge contract from `combine_dictionary_entries` is preserved via
/// `merge_content_type`, applied once per pass. The baseline pass may only fill an empty
/// slot (`allow_override = false`); the refine pass may additionally upgrade an existing
/// non-`unique_id`, non-date token (`allow_override = true`). Neither pass can override the
/// deterministic `"unique_id"` stamp, reclassify a `date`/`datetime` column, or introduce a
/// `date`/`datetime` token onto a non-date column — only the strftime `:<fmt>` suffix may
/// be added or corrected on an already-date-typed column.
pub(super) fn combine_dictionary_entries_with_baseline(
    mut code_entries: Vec<DictionaryEntry>,
    baseline_llm_fields: &HashMap<String, LlmDictField>,
    refine_llm_fields: &HashMap<String, LlmDictField>,
    infer_content_type: bool,
) -> Vec<DictionaryEntry> {
    for entry in &mut code_entries {
        // Stage 1: apply baseline (first-pass) LLM values, mirroring
        // `combine_dictionary_entries`'s behavior for the single-pass case.
        if let Some(baseline) = baseline_llm_fields.get(&entry.name) {
            entry.label = baseline.label.clone();
            entry.description = baseline.description.clone();
            entry.content_type =
                merge_content_type(&entry.content_type, &baseline.content_type, false);
            entry.concept = merge_concept(&entry.concept, &baseline.concept, false);
            entry.role = merge_role(&entry.role, &baseline.role, false);
        }
        // Stage 2: overlay refine-pass LLM values where present. Omitted fields keep their
        // baseline values from stage 1 — this is the whole point of the baseline merge.
        if let Some(refine) = refine_llm_fields.get(&entry.name) {
            if !refine.label.is_empty() {
                entry.label = refine.label.clone();
            }
            if !refine.description.is_empty() {
                entry.description = refine.description.clone();
            }
            entry.content_type =
                merge_content_type(&entry.content_type, &refine.content_type, true);
            entry.concept = merge_concept(&entry.concept, &refine.concept, true);
            entry.role = merge_role(&entry.role, &refine.role, true);
        }
        // Stage 3: same final "unknown"/fallback coercion as `combine_dictionary_entries` so
        // the two-pass output matches single-pass invariants.
        if infer_content_type {
            if entry.content_type.is_empty() {
                entry.content_type = "unknown".to_string();
            }
            coerce_role_concept(entry);
        }
    }
    code_entries
}

/// True if `sample` parses cleanly with the chrono strftime `fmt`. For
/// `datetime`, an offset-naive parse is tried first, then an offset-aware one
/// (`%z` / `%:z` formats are rejected by `NaiveDateTime`).
fn sample_parses_with_format(sample: &str, fmt: &str, is_datetime: bool) -> bool {
    use chrono::{DateTime, NaiveDate, NaiveDateTime};
    if is_datetime {
        NaiveDateTime::parse_from_str(sample, fmt).is_ok()
            || DateTime::parse_from_str(sample, fmt).is_ok()
    } else {
        NaiveDate::parse_from_str(sample, fmt).is_ok()
    }
}

/// Group usable raw frequency values by field name — every value except the
/// rank-0 "Other" bucket, the `<ALL_UNIQUE>` sentinel, and the emitted null row.
///
/// The null row is identified by BOTH signals `frequency` guarantees for it,
/// never by either one alone:
///   * its `value` equals the configured `--null-text` (`null_text`, default `(NULL)`), and
///   * its `count` equals the column's null count (carried on `DictionaryEntry.null_count`).
///
/// Value alone is too broad — `frequency`'s null text is configurable, and a
/// real datum that merely reads as the null label would be dropped. Count
/// alone is also too broad — a real value that merely shares the column's null
/// count would be dropped, which can keep a bad date suffix or wrongly
/// downgrade a `datetime` column. Requiring both confines the exclusion to the
/// genuine null sentinel, even when `frequency --pct-nulls` gives that row a
/// real (non-zero) rank that a rank check alone would let through.
///
/// Borrows the sample strings from `frequency_records`.
fn usable_samples_by_field<'a>(
    frequency_records: &'a [FrequencyRecord],
    entries: &[DictionaryEntry],
    null_text: &str,
) -> HashMap<&'a str, Vec<&'a str>> {
    let null_count_by_field: HashMap<&str, u64> = entries
        .iter()
        .filter(|e| e.null_count > 0)
        .map(|e| (e.name.as_str(), e.null_count))
        .collect();

    let mut samples_by_field: HashMap<&str, Vec<&str>> = HashMap::new();
    for rec in frequency_records {
        if rec.rank == 0.0 || rec.value.contains("<ALL_UNIQUE>") {
            continue;
        }
        // the emitted null row: `frequency` writes it with `--null-text` as
        // its value AND a count equal to the column's null count
        if rec.value == null_text && null_count_by_field.get(rec.field.as_str()) == Some(&rec.count)
        {
            continue;
        }
        samples_by_field
            .entry(rec.field.as_str())
            .or_default()
            .push(rec.value.as_str());
    }
    samples_by_field
}

/// Semantically validate the LLM-inferred strftime `:<fmt>` suffix on every
/// `date`/`datetime` entry against the real on-disk sample values.
///
/// `normalize_datetime_token` only checks the suffix is *syntactically* valid
/// chrono strftime; a well-formed format can still mismatch the data (the LLM
/// guesses `%m/%d/%Y` for `%d/%m/%Y` data). For each date/datetime entry that
/// carries a suffix, this attempts to parse EVERY usable raw value from the
/// frequency records — skipping the rank-0 "Other" bucket and the
/// `<ALL_UNIQUE>` sentinel — with the inferred format. An ambiguous format can
/// parse an early sample (`01/02/2020`) yet be disproven by a later one
/// (`13/02/2020`), so all samples must parse; if any fails, the suffix is
/// stripped back to the bare token.
///
/// Entries with no usable sample (an ALL_UNIQUE date column, or one whose only
/// frequency row is the "Other" bucket) keep their suffix unchanged — there is
/// nothing to validate them against.
pub(super) fn validate_date_formats(
    entries: &mut [DictionaryEntry],
    frequency_records: &[FrequencyRecord],
    null_text: &str,
) {
    let samples_by_field = usable_samples_by_field(frequency_records, entries, null_text);

    for entry in entries {
        let base = content_type_base(&entry.content_type);
        if base != "date" && base != "datetime" {
            continue;
        }
        let Some(fmt) = entry
            .content_type
            .strip_prefix("datetime:")
            .or_else(|| entry.content_type.strip_prefix("date:"))
        else {
            continue; // bare token, nothing to validate
        };
        let Some(samples) = samples_by_field.get(entry.name.as_str()) else {
            continue; // no usable sample (e.g. ALL_UNIQUE) - accept as-is
        };
        // Every usable sample must parse: an ambiguous format can match the
        // first sample yet be disproven by a later one. If any fails, the
        // inferred format is wrong for this column - strip back to bare token.
        if !samples
            .iter()
            .all(|s| sample_parses_with_format(s, fmt, base == "datetime"))
        {
            entry.content_type = base.to_string();
        }
    }
}

/// Produce the date-only portion of a chrono strftime format by truncating at
/// the first time-related specifier (`%H`, `%I`, `%M`, `%S`, `%p`, `%z`, …).
///
/// Used when an all-midnight `datetime` column is reclassified as `date`: the
/// on-disk format `%m/%d/%Y %I:%M:%S %p` becomes the date-only `%m/%d/%Y`.
///
/// Returns `None` when there is no clean date-only prefix — a datetime-compound
/// specifier (`%c`, `%+`, `%s`) or a time-first layout — so the caller falls
/// back to a bare `date` token.
fn strip_time_from_format(fmt: &str) -> Option<String> {
    let bytes = fmt.as_bytes();
    let mut i = 0;
    let mut cut: Option<usize> = None;
    while i < bytes.len() {
        if bytes[i] != b'%' {
            i += 1;
            continue;
        }
        let pct = i;
        i += 1; // past '%'
        if i >= bytes.len() {
            break;
        }
        if bytes[i] == b'%' {
            i += 1; // literal "%%"
            continue;
        }
        // skip modifier chars (`-_0.:#` and digits) to reach the specifier letter
        while i < bytes.len() && matches!(bytes[i], b'-' | b'_' | b'0'..=b'9' | b'.' | b':' | b'#')
        {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        let letter = bytes[i] as char;
        i += 1;
        match letter {
            // datetime-compound: no clean date-only prefix
            'c' | '+' | 's' => return None,
            // time-related specifier: the date part ends at this `%`
            'H' | 'k' | 'I' | 'l' | 'P' | 'p' | 'M' | 'S' | 'f' | 'R' | 'T' | 'X' | 'r' | 'Z'
            | 'z' => {
                cut = Some(pct);
                break;
            },
            // date specifier (or a harmless literal like `%n`/`%t`) — keep scanning
            _ => {},
        }
    }
    let date_part = cut.map_or(fmt, |c| &fmt[..c]);
    // trim the trailing date/time separator literal (whitespace, the ISO `T`,
    // or punctuation like `-` `/` `.` `:` `,` `_`) so a format such as
    // `%Y-%m-%d-%H:%M:%S` yields `%Y-%m-%d`, not `%Y-%m-%d-`
    let trimmed = date_part.trim_end_matches(|c: char| {
        c.is_whitespace() || matches!(c, 'T' | '-' | '/' | '.' | ':' | ',' | '_')
    });
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

/// `Some(true)` if `sample` parses with the chrono strftime `fmt` AND its
/// time-of-day is exactly midnight; `Some(false)` if it parses with a
/// non-midnight time; `None` if it does not parse at all (so the caller can
/// ignore it rather than treat it as evidence either way).
fn sample_time_at_midnight(sample: &str, fmt: &str) -> Option<bool> {
    use chrono::{DateTime, NaiveDateTime, Timelike};
    let time = NaiveDateTime::parse_from_str(sample, fmt)
        .map(|dt| dt.time())
        .or_else(|_| DateTime::parse_from_str(sample, fmt).map(|dt| dt.time()))
        .ok()?;
    Some(time.num_seconds_from_midnight() == 0 && time.nanosecond() == 0)
}

/// Reclassify a `datetime` column as `date` when every usable frequency sample
/// parses with the inferred format AND falls exactly on midnight — i.e. the
/// column holds dates stored with a zero time-of-day. `qsv stats` types a
/// column `DateTime` whenever ANY value has a non-midnight time, which
/// over-reports columns whose dominant pattern (per the frequency
/// distribution) is plainly a date.
///
/// Runs after `validate_date_formats`, so a surviving `datetime:<fmt>` suffix
/// is already format-validated. On downgrade the time specifiers are stripped
/// from `<fmt>` via `strip_time_from_format`, yielding `date:<date-fmt>` (or a
/// bare `date` when the format has no clean date-only prefix).
///
/// Bare `datetime` (no inferred format) is left unchanged — there is no format
/// to test the samples against. A sample that does NOT parse with the format
/// blocks the downgrade (the column stays `datetime`): a non-parsing real
/// value could carry a non-midnight time, so ignoring it would risk a wrong
/// downgrade. The emitted null row never reaches here — `usable_samples_by_field`
/// excludes it by `--null-text` value plus null count, even when
/// `frequency --pct-nulls` gives it a real rank.
pub(super) fn downgrade_all_midnight_datetime_columns(
    entries: &mut [DictionaryEntry],
    frequency_records: &[FrequencyRecord],
    null_text: &str,
) {
    let samples_by_field = usable_samples_by_field(frequency_records, entries, null_text);

    for entry in entries {
        let Some(fmt) = entry.content_type.strip_prefix("datetime:") else {
            continue; // bare `datetime`, or not a datetime column
        };
        let Some(samples) = samples_by_field.get(entry.name.as_str()) else {
            continue; // no usable sample to judge by
        };
        // EVERY usable sample must parse with the format: collecting into an
        // `Option<Vec<_>>` yields `None` if any sample fails to parse, which
        // keeps the column `datetime` rather than silently dropping a value
        // that might carry a non-midnight time.
        let Some(midnight_flags) = samples
            .iter()
            .map(|s| sample_time_at_midnight(s, fmt))
            .collect::<Option<Vec<bool>>>()
        else {
            continue;
        };
        if !midnight_flags.is_empty() && midnight_flags.iter().all(|&midnight| midnight) {
            entry.content_type = strip_time_from_format(fmt)
                .map_or_else(|| "date".to_string(), |date_fmt| format!("date:{date_fmt}"));
        }
    }
}

/// Extract the `{field_name: {label, description[, content_type]}}` map from the
/// LLM's JSON response, restricted to the given `field_names`. When
/// `infer_content_type` is set, `content_type` is validated against
/// `CONTENT_TYPE_VOCAB`; a missing, empty, or out-of-vocabulary value is left empty
/// here — `combine_dictionary_entries` is the single point that coerces any
/// still-empty `content_type` to `"unknown"`. When the flag is unset, `content_type`
/// is always empty.
///
/// Most tokens are lowercased before the vocab lookup, but `date`/`datetime` are
/// matched case-preserving via `normalize_datetime_token` because their optional
/// `:<fmt>` chrono strftime suffix is case-sensitive.
///
/// `unique_id` is in the vocab but is REJECTED from LLM input here: it is set
/// deterministically based on the `<ALL_UNIQUE>` frequency sentinel and the LLM
/// has no way to verify that condition, so accepting it would let non-ALL_UNIQUE
/// fields be misclassified.
pub(super) fn parse_llm_dictionary_response(
    llm_response: &str,
    field_names: &[String],
    infer_content_type: bool,
) -> CliResult<HashMap<String, LlmDictField>> {
    let json_value = extract_json_from_output(llm_response)?;

    let mut result = HashMap::new();

    if let Some(obj) = json_value.as_object() {
        for field_name in field_names {
            if let Some(field_obj) = obj.get(field_name)
                && let Some(field_map) = field_obj.as_object()
            {
                let label = field_map
                    .get("label")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let description = field_map
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let content_type = if infer_content_type {
                    // `CONTENT_TYPE_VOCAB` is all lowercase and LLMs don't reliably echo
                    // casing (e.g. "Email", "First_Name"), so most tokens are case-folded
                    // before the vocab lookup. A missing, empty, or out-of-vocabulary value
                    // is left empty here; `combine_dictionary_entries` coerces any
                    // still-empty content_type to "unknown" when the flag is set.
                    //
                    // `date`/`datetime` are handled FIRST and case-preserving: they may
                    // carry a `:<fmt>` chrono strftime suffix (e.g. "datetime:%m/%d/%Y
                    // %I:%M:%S %p") whose specifiers are case-sensitive (%m vs %M, %p), so
                    // `normalize_datetime_token` lowercases only the token prefix.
                    //
                    // `duration` is special: the LLM may append an upper-bound suffix (e.g.
                    // "duration:3600") that isn't in `CONTENT_TYPE_VOCAB` literally, so
                    // route it through `normalize_duration_token`.
                    //
                    // `unique_id` is REJECTED here even though it is in the vocab: it is set
                    // deterministically by `generate_code_based_dictionary` based on the
                    // `<ALL_UNIQUE>` frequency sentinel (`cardinality == rowcount`), and the
                    // LLM has no way to verify that condition. Accepting it from LLM output
                    // would let non-ALL_UNIQUE fields be misclassified as `unique_id`,
                    // breaking the deterministic-only contract documented on
                    // `CONTENT_TYPE_VOCAB`.
                    let raw_trimmed = field_map
                        .get("content_type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .trim();
                    if let Some(normalized) = normalize_datetime_token(raw_trimmed) {
                        normalized
                    } else {
                        let raw = raw_trimmed.to_ascii_lowercase();
                        if let Some(normalized) = normalize_duration_token(&raw) {
                            normalized
                        } else if raw == "unique_id" {
                            String::new()
                        } else if CONTENT_TYPE_VOCAB.contains(&raw.as_str()) {
                            raw
                        } else {
                            String::new()
                        }
                    }
                } else {
                    String::new()
                };

                // `concept` and `role` ride the same `infer_content_type` gate as
                // `content_type`. `concept` is validated against `CONCEPT_VOCAB`
                // (case-folded); `id.surrogate_key` is deterministic and refused
                // from LLM input, mirroring the `unique_id` content_type rejection.
                // `role` is validated against `ROLE_VOCAB`. Out-of-vocab / missing
                // values are left empty; `combine_dictionary_entries` applies the
                // deterministic seed and the final fallback.
                let (concept, role) = if infer_content_type {
                    let concept_raw = field_map
                        .get("concept")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .trim()
                        .to_ascii_lowercase();
                    let concept = if concept_raw == "id.surrogate_key"
                        || !CONCEPT_VOCAB.contains(&concept_raw.as_str())
                    {
                        String::new()
                    } else {
                        concept_raw
                    };
                    let role_raw = field_map
                        .get("role")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .trim()
                        .to_ascii_lowercase();
                    let role = if ROLE_VOCAB.contains(&role_raw.as_str()) {
                        role_raw
                    } else {
                        String::new()
                    };
                    (concept, role)
                } else {
                    (String::new(), String::new())
                };

                result.insert(
                    field_name.clone(),
                    LlmDictField {
                        label,
                        description,
                        content_type,
                        concept,
                        role,
                    },
                );
            }
        }
    }

    Ok(result)
}

/// Extract the optional top-level `relationships` array from the LLM's dictionary
/// response, validated against `field_names`. Each surviving entry is a clean
/// JSON object `{kind, members[, anchor]}` ready to embed in the dictionary
/// output and consume from `synthesize`.
///
/// An entry is dropped when its `kind` is not one of `joint` / `ordered` /
/// `correlated`, when it has fewer than two members, or when any member (or an
/// `ordered` anchor) names a column not in `field_names`. Anything that isn't
/// well-formed JSON yields an empty list — relationship inference is best-effort
/// and never fails the dictionary phase. `synthesize` re-validates every
/// relationship against the real data before using it, so this stage only needs
/// to guarantee structural soundness.
pub(super) fn parse_llm_relationships(
    llm_response: &str,
    field_names: &[String],
) -> Vec<serde_json::Value> {
    let Ok(json_value) = extract_json_from_output(llm_response) else {
        return Vec::new();
    };
    let Some(raw) = json_value.get("relationships").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    let in_fields = |name: &str| field_names.iter().any(|f| f == name);

    let mut result = Vec::new();
    for entry in raw {
        let Some(obj) = entry.as_object() else {
            continue;
        };
        let kind = obj.get("kind").and_then(|v| v.as_str()).unwrap_or_default();
        if !matches!(kind, "joint" | "ordered" | "correlated") {
            continue;
        }
        let members: Vec<String> = obj
            .get("members")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|m| m.as_str())
                    .map(ToString::to_string)
                    .collect()
            })
            .unwrap_or_default();
        if members.len() < 2 || !members.iter().all(|m| in_fields(m)) {
            continue;
        }

        let mut clean = serde_json::Map::new();
        clean.insert("kind".to_string(), serde_json::json!(kind));
        clean.insert("members".to_string(), serde_json::json!(members));
        // `anchor` is meaningful only for `ordered` groups; keep it when the LLM
        // supplied one that names a real column.
        if kind == "ordered"
            && let Some(anchor) = obj.get("anchor").and_then(|v| v.as_str())
            && in_fields(anchor)
        {
            clean.insert("anchor".to_string(), serde_json::json!(anchor));
        }
        result.push(serde_json::Value::Object(clean));
    }
    result
}

/// Extract the optional top-level `grain` string from the LLM's dictionary
/// response — a one-sentence statement of what a single row represents (e.g.
/// "one row = one 311 service request"). Best-effort and dataset-level (it is
/// NOT a per-field value, so it rides alongside `parse_llm_relationships` rather
/// than through `LlmDictField`): malformed JSON or a missing/blank `grain` yields
/// `None`, never failing the dictionary phase.
pub(super) fn parse_llm_grain(llm_response: &str) -> Option<String> {
    let json_value = extract_json_from_output(llm_response).ok()?;
    let grain = json_value.get("grain")?.as_str()?.trim();
    (!grain.is_empty()).then(|| grain.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blank_entry(name: &str) -> DictionaryEntry {
        DictionaryEntry {
            name:         name.to_string(),
            r#type:       "String".to_string(),
            label:        String::new(),
            description:  String::new(),
            content_type: String::new(),
            min:          String::new(),
            max:          String::new(),
            cardinality:  0,
            enumeration:  String::new(),
            null_count:   0,
            addl_cols:    IndexMap::new(),
            examples:     String::new(),
            freq_details: Vec::new(),
            is_unique_id: false,
            concept:      String::new(),
            role:         String::new(),
        }
    }

    #[test]
    fn format_date_examples_reformats_values_keeps_counts() {
        // DateTime values with a time component, inferred date-only format:
        // values are reformatted, " [count]" suffixes preserved verbatim.
        let out = format_date_examples(
            "date:%m/%d/%Y",
            "01/24/2013 12:00:00 AM [5]\n01/07/2014 12:00:00 AM [3]",
        );
        assert_eq!(out, "01/24/2013 [5]\n01/07/2014 [3]");
    }

    #[test]
    fn format_date_examples_idempotent_when_already_formatted() {
        let out = format_date_examples("date:%m/%d/%Y", "01/24/2013 [5]");
        assert_eq!(out, "01/24/2013 [5]");
    }

    #[test]
    fn format_date_examples_respects_day_first_format() {
        // Ambiguous "01/02/2020" under a day-first inferred format must parse as
        // 1 Feb and round-trip unchanged, NOT be swapped to "02/01/2020" by a
        // hardcoded month-first preference.
        assert_eq!(
            format_date_examples("date:%d/%m/%Y", "01/02/2020 [5]"),
            "01/02/2020 [5]"
        );
        // Month-first format keeps month-first interpretation.
        assert_eq!(
            format_date_examples("date:%m/%d/%Y", "01/02/2020 [5]"),
            "01/02/2020 [5]"
        );
        assert!(prefer_dmy("%d/%m/%Y"));
        assert!(!prefer_dmy("%m/%d/%Y"));
        assert!(!prefer_dmy("%Y-%m-%d")); // ISO is month-before-day
        // Padding/variant modifiers must be recognized via StrftimeItems, not
        // a fixed token list: %0d/%0m, %_d/%_m, %-d/%-m, %e all map to the
        // Day/Month numeric fields regardless of padding.
        assert!(prefer_dmy("%0d/%0m/%Y"));
        assert!(prefer_dmy("%_d/%_m/%Y"));
        assert!(prefer_dmy("%-d/%-m/%Y"));
        assert!(prefer_dmy("%e/%m/%Y"));
        assert!(!prefer_dmy("%0m/%0d/%Y"));
        assert!(!prefer_dmy("%H:%M:%S")); // no date fields → default month-first
        // End-to-end through format_date_examples with a padded day-first fmt.
        assert_eq!(
            format_date_examples("date:%0d/%0m/%Y", "01/02/2020 [5]"),
            "01/02/2020 [5]"
        );
    }

    #[test]
    fn format_date_examples_passthrough_cases() {
        // No inferred format → unchanged.
        assert_eq!(
            format_date_examples("date", "01/24/2013 12:00:00 AM [5]"),
            "01/24/2013 12:00:00 AM [5]"
        );
        // Non-date content type → unchanged.
        assert_eq!(
            format_date_examples("category", "alpha [9]\nbeta [3]"),
            "alpha [9]\nbeta [3]"
        );
        // <ALL_UNIQUE> sentinel and empty input → unchanged.
        assert_eq!(
            format_date_examples("date:%m/%d/%Y", "<ALL_UNIQUE>"),
            "<ALL_UNIQUE>"
        );
        assert_eq!(format_date_examples("date:%m/%d/%Y", ""), "");
        // Unparseable frequency-bucket values pass through, real dates still format.
        assert_eq!(
            format_date_examples("date:%m/%d/%Y", "Other… [4091]\n01/24/2013 12:00:00 AM [5]"),
            "Other… [4091]\n01/24/2013 [5]"
        );
    }

    #[test]
    fn parse_llm_response_ignores_content_type_when_flag_off() {
        let json = r#"{
            "name": {"label": "Name", "description": "the name", "content_type": "first_name"}
        }"#;
        let fields = vec!["name".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, false).unwrap();
        let f = parsed.get("name").unwrap();
        assert_eq!(f.label, "Name");
        assert_eq!(f.description, "the name");
        assert!(
            f.content_type.is_empty(),
            "content_type must stay empty when infer_content_type is false"
        );
    }

    #[test]
    fn parse_llm_response_extracts_valid_content_type() {
        let json = r#"{
            "email_addr": {"label": "Email", "description": "an email", "content_type": "email"}
        }"#;
        let fields = vec!["email_addr".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert_eq!(parsed.get("email_addr").unwrap().content_type, "email");
    }

    #[test]
    fn parse_llm_response_normalizes_content_type_casing() {
        // LLMs don't reliably echo casing; a valid-but-differently-cased token must be
        // accepted and stored in its normalized lowercase form, not coerced to "unknown".
        let json = r#"{
            "a": {"label": "A", "description": "d", "content_type": "Email"},
            "b": {"label": "B", "description": "d", "content_type": "FIRST_NAME"},
            "c": {"label": "C", "description": "d", "content_type": "  Mac_Address  "}
        }"#;
        let fields = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert_eq!(parsed.get("a").unwrap().content_type, "email");
        assert_eq!(parsed.get("b").unwrap().content_type, "first_name");
        assert_eq!(parsed.get("c").unwrap().content_type, "mac_address");
    }

    #[test]
    fn parse_llm_response_rejects_llm_supplied_unique_id() {
        // `unique_id` is in CONTENT_TYPE_VOCAB but must be REJECTED from LLM
        // output: it is only valid when stamped deterministically based on the
        // `<ALL_UNIQUE>` sentinel. Accepting it from LLM input would let
        // non-ALL_UNIQUE fields be misclassified. The parser drops it (empties
        // the field); combine_dictionary_entries then coerces to "unknown".
        let json = r#"{
            "a": {"label": "A", "description": "d", "content_type": "unique_id"},
            "b": {"label": "B", "description": "d", "content_type": "UNIQUE_ID"},
            "c": {"label": "C", "description": "d", "content_type": "  Unique_ID  "}
        }"#;
        let fields = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert!(
            parsed.get("a").unwrap().content_type.is_empty(),
            "literal 'unique_id' must be stripped from LLM output"
        );
        assert!(
            parsed.get("b").unwrap().content_type.is_empty(),
            "uppercased 'UNIQUE_ID' must also be stripped (lowercased before check)"
        );
        assert!(
            parsed.get("c").unwrap().content_type.is_empty(),
            "padded/cased 'unique_id' must also be stripped after trim+lowercase"
        );
    }

    #[test]
    fn combine_refuses_llm_supplied_unique_id_for_non_all_unique_field() {
        // Even if a future caller bypasses parse_llm_dictionary_response and
        // hands combine_dictionary_entries an LlmDictField with content_type =
        // "unique_id", combine must refuse to copy it onto a field whose
        // code-derived entry was empty (i.e. not ALL_UNIQUE). Such fields fall
        // through to "unknown" when the flag is on.
        let code_entries = vec![blank_entry("not_unique")];
        let mut llm = HashMap::new();
        llm.insert(
            "not_unique".to_string(),
            LlmDictField {
                label:        "X".to_string(),
                description:  "y".to_string(),
                content_type: "unique_id".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );
        let combined = combine_dictionary_entries(code_entries, &llm, true);
        assert_eq!(
            combined[0].content_type, "unknown",
            "smuggled LLM 'unique_id' on a non-ALL_UNIQUE field must be rejected and fall back to \
             'unknown'"
        );
    }

    #[test]
    fn parse_llm_response_drops_out_of_vocab_content_type() {
        // An out-of-vocabulary token is left empty by parsing; combine_dictionary_entries
        // is what coerces it to "unknown".
        let json = r#"{
            "mystery": {"label": "X", "description": "y", "content_type": "made_up_token"}
        }"#;
        let fields = vec!["mystery".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert!(parsed.get("mystery").unwrap().content_type.is_empty());
    }

    #[test]
    fn normalize_duration_token_handles_all_forms() {
        // Bare token is the trivial accept.
        assert_eq!(
            normalize_duration_token("duration").as_deref(),
            Some("duration")
        );
        // Well-formed positive integer suffix: preserved verbatim.
        assert_eq!(
            normalize_duration_token("duration:3600").as_deref(),
            Some("duration:3600")
        );
        // Whitespace around the number is tolerated.
        assert_eq!(
            normalize_duration_token("duration: 18000").as_deref(),
            Some("duration:18000")
        );
        // Malformed suffixes degrade gracefully to bare "duration" rather
        // than dropping the classification entirely — the LLM picked
        // "duration" correctly, only the cap is bad.
        assert_eq!(
            normalize_duration_token("duration:0").as_deref(),
            Some("duration")
        );
        assert_eq!(
            normalize_duration_token("duration:-5").as_deref(),
            Some("duration")
        );
        assert_eq!(
            normalize_duration_token("duration:abc").as_deref(),
            Some("duration")
        );
        assert_eq!(
            normalize_duration_token("duration:").as_deref(),
            Some("duration")
        );
        // Non-duration tokens return None so the caller falls through to
        // the regular vocab check.
        assert_eq!(normalize_duration_token("time"), None);
        assert_eq!(normalize_duration_token("email"), None);
        assert_eq!(normalize_duration_token(""), None);
    }

    #[test]
    fn parse_llm_response_accepts_duration_suffix() {
        let json = r#"{
            "elapsed":    {"label": "E", "description": "d", "content_type": "duration:3600"},
            "race_time":  {"label": "R", "description": "d", "content_type": "Duration: 18000"},
            "bare":       {"label": "B", "description": "d", "content_type": "duration"},
            "bad_cap":    {"label": "X", "description": "d", "content_type": "duration:0"},
            "bad_suffix": {"label": "Y", "description": "d", "content_type": "duration:abc"}
        }"#;
        let fields = vec![
            "elapsed".to_string(),
            "race_time".to_string(),
            "bare".to_string(),
            "bad_cap".to_string(),
            "bad_suffix".to_string(),
        ];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert_eq!(parsed.get("elapsed").unwrap().content_type, "duration:3600");
        // Casing + inner whitespace normalized.
        assert_eq!(
            parsed.get("race_time").unwrap().content_type,
            "duration:18000"
        );
        assert_eq!(parsed.get("bare").unwrap().content_type, "duration");
        // Malformed suffix collapses to bare "duration" rather than empty.
        assert_eq!(parsed.get("bad_cap").unwrap().content_type, "duration");
        assert_eq!(parsed.get("bad_suffix").unwrap().content_type, "duration");
    }

    #[test]
    fn parse_llm_response_missing_content_type_is_empty() {
        // A missing content_type key is left empty by parsing; combine_dictionary_entries
        // coerces it to "unknown" when the flag is set.
        let json = r#"{ "f": {"label": "F", "description": "d"} }"#;
        let fields = vec!["f".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert!(parsed.get("f").unwrap().content_type.is_empty());
    }

    #[test]
    fn combine_copies_content_type_onto_entry() {
        let code_entries = vec![blank_entry("col_a"), blank_entry("col_b")];
        let mut llm = HashMap::new();
        llm.insert(
            "col_a".to_string(),
            LlmDictField {
                label:        "A".to_string(),
                description:  "desc a".to_string(),
                content_type: "city".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );
        // infer_content_type = false: pure copy, no "unknown" coercion.
        let combined = combine_dictionary_entries(code_entries, &llm, false);
        assert_eq!(combined[0].label, "A");
        assert_eq!(combined[0].description, "desc a");
        assert_eq!(combined[0].content_type, "city");
        // col_b had no LLM entry, so its content_type stays empty when the flag is off.
        assert!(combined[1].content_type.is_empty());
    }

    #[test]
    fn combine_fills_unknown_for_empty_content_type_when_flag_on() {
        // With --infer-content-type set, every entry must end up with a non-empty
        // content_type: a valid token is kept; a field the LLM left empty (e.g. an
        // out-of-vocab token dropped by parsing) or omitted entirely falls back to "unknown".
        let code_entries = vec![
            blank_entry("kept"),
            blank_entry("emptied"),
            blank_entry("omitted"),
        ];
        let mut llm = HashMap::new();
        llm.insert(
            "kept".to_string(),
            LlmDictField {
                label:        "K".to_string(),
                description:  "d".to_string(),
                content_type: "city".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );
        llm.insert(
            "emptied".to_string(),
            LlmDictField {
                label:        "E".to_string(),
                description:  "d".to_string(),
                content_type: String::new(),
                concept:      String::new(),
                role:         String::new(),
            },
        );
        // "omitted" is intentionally absent from the LLM map.
        let combined = combine_dictionary_entries(code_entries, &llm, true);
        assert_eq!(combined[0].content_type, "city");
        assert_eq!(combined[1].content_type, "unknown");
        assert_eq!(combined[2].content_type, "unknown");
    }

    #[test]
    fn combine_preserves_preset_unique_id_over_llm_value() {
        // generate_code_based_dictionary stamps "unique_id" deterministically on
        // ALL_UNIQUE fields; combine_dictionary_entries must keep that value even
        // when the LLM returned a different (in-vocab) token for the same field.
        let mut preset = blank_entry("pk");
        preset.content_type = "unique_id".to_string();
        let code_entries = vec![preset, blank_entry("other")];
        let mut llm = HashMap::new();
        llm.insert(
            "pk".to_string(),
            LlmDictField {
                label:        "Primary Key".to_string(),
                description:  "row identifier".to_string(),
                content_type: "uuid".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );
        llm.insert(
            "other".to_string(),
            LlmDictField {
                label:        "Other".to_string(),
                description:  "city field".to_string(),
                content_type: "city".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );
        let combined = combine_dictionary_entries(code_entries, &llm, true);
        // Deterministic "unique_id" wins over the LLM's "uuid".
        assert_eq!(combined[0].content_type, "unique_id");
        // Label/Description still flow through from the LLM.
        assert_eq!(combined[0].label, "Primary Key");
        assert_eq!(combined[0].description, "row identifier");
        // Non-ALL_UNIQUE field gets the LLM's token unchanged.
        assert_eq!(combined[1].content_type, "city");
    }

    #[test]
    fn generate_marks_all_unique_field_as_unique_id() {
        // Field "id" has the <ALL_UNIQUE> sentinel in its frequency table, so
        // when infer_content_type is on, generate_code_based_dictionary must
        // pre-set its content_type to "unique_id". The peer field "category"
        // has a normal frequency distribution and must stay empty.
        let stats = vec![
            StatsRecord {
                field:       "id".to_string(),
                r#type:      "Integer".to_string(),
                cardinality: 1000,
                nullcount:   0,
                min:         "1".to_string(),
                max:         "1000".to_string(),
                addl_cols:   IndexMap::new(),
            },
            StatsRecord {
                field:       "category".to_string(),
                r#type:      "String".to_string(),
                cardinality: 2,
                nullcount:   0,
                min:         "a".to_string(),
                max:         "b".to_string(),
                addl_cols:   IndexMap::new(),
            },
        ];
        let frequencies = vec![
            FrequencyRecord {
                field:      "id".to_string(),
                value:      "<ALL_UNIQUE>".to_string(),
                count:      1000,
                percentage: 100.0,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "category".to_string(),
                value:      "a".to_string(),
                count:      600,
                percentage: 60.0,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "category".to_string(),
                value:      "b".to_string(),
                count:      400,
                percentage: 40.0,
                rank:       2.0,
            },
        ];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], true);
        assert_eq!(entries[0].name, "id");
        assert_eq!(entries[0].content_type, "unique_id");
        assert_eq!(entries[1].name, "category");
        assert!(
            entries[1].content_type.is_empty(),
            "non-ALL_UNIQUE field must leave content_type empty for LLM fill"
        );
    }

    #[test]
    fn generate_does_not_mark_truncated_frequency_as_unique_id() {
        // Truncated/custom frequency (e.g. `--limit 1 --no-other`) emits a single top
        // row whose `count` happens to equal the column cardinality, but `percentage`
        // is below 100 because the column is NOT unique. The structural detector must
        // require percentage == 100, so neither content_type nor is_unique_id is set.
        let stats = vec![StatsRecord {
            field:       "code".to_string(),
            r#type:      "String".to_string(),
            cardinality: 3,
            nullcount:   0,
            min:         "a".to_string(),
            max:         "c".to_string(),
            addl_cols:   IndexMap::new(),
        }];
        // Single emitted row: count(3) == cardinality(3) but percentage 30 (< 100),
        // i.e. the top value covers 3 of ~10 rows; the rest were truncated.
        let frequencies = vec![FrequencyRecord {
            field:      "code".to_string(),
            value:      "a".to_string(),
            count:      3,
            percentage: 30.0,
            rank:       1.0,
        }];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], true);
        assert_eq!(entries[0].name, "code");
        assert!(
            !entries[0].is_unique_id,
            "truncated single-row frequency with percentage < 100 must not be a unique id"
        );
        assert!(
            entries[0].content_type.is_empty(),
            "truncated single-row frequency must not be stamped unique_id"
        );
    }

    #[test]
    fn generate_skips_unique_id_when_infer_content_type_off() {
        // When --infer-content-type is OFF, the content_type column is suppressed
        // entirely, so we must not pre-set "unique_id" even for ALL_UNIQUE fields.
        let stats = vec![StatsRecord {
            field:       "id".to_string(),
            r#type:      "Integer".to_string(),
            cardinality: 100,
            nullcount:   0,
            min:         "1".to_string(),
            max:         "100".to_string(),
            addl_cols:   IndexMap::new(),
        }];
        let frequencies = vec![FrequencyRecord {
            field:      "id".to_string(),
            value:      "<ALL_UNIQUE>".to_string(),
            count:      100,
            percentage: 100.0,
            rank:       1.0,
        }];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], false);
        assert!(entries[0].content_type.is_empty());
    }

    #[test]
    fn generate_does_not_mislabel_constant_field_with_all_unique_value() {
        // Pathological: a field whose only value is literally the string
        // "<ALL_UNIQUE>". qsv frequency emits a single row with value=
        // "<ALL_UNIQUE>" and count==row_count, but stats.cardinality==1.
        // Structural detection (count == cardinality, cardinality > 1)
        // correctly excludes this.
        let stats = vec![StatsRecord {
            field:       "weird".to_string(),
            r#type:      "String".to_string(),
            cardinality: 1,
            nullcount:   0,
            min:         "<ALL_UNIQUE>".to_string(),
            max:         "<ALL_UNIQUE>".to_string(),
            addl_cols:   IndexMap::new(),
        }];
        let frequencies = vec![FrequencyRecord {
            field:      "weird".to_string(),
            value:      "<ALL_UNIQUE>".to_string(),
            count:      500,
            percentage: 100.0,
            rank:       1.0,
        }];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], true);
        assert!(
            entries[0].content_type.is_empty(),
            "a constant-value field whose value happens to be the string '<ALL_UNIQUE>' must NOT \
             be classified as unique_id"
        );
    }

    #[test]
    fn generate_detects_unique_id_with_custom_all_unique_text() {
        // If qsv frequency was run with `--all-unique-text` set to a custom
        // string (e.g. "<UNIQUE>"), the sentinel row's text differs but the
        // structural invariant (one row, count == cardinality, cardinality > 1,
        // no nulls) still holds. Detection must succeed regardless of the
        // sentinel text.
        let stats = vec![StatsRecord {
            field:       "pk".to_string(),
            r#type:      "Integer".to_string(),
            cardinality: 250,
            nullcount:   0,
            min:         "1".to_string(),
            max:         "250".to_string(),
            addl_cols:   IndexMap::new(),
        }];
        let frequencies = vec![FrequencyRecord {
            field:      "pk".to_string(),
            value:      "<UNIQUE>".to_string(), // user-customized sentinel text
            count:      250,
            percentage: 100.0,
            rank:       1.0,
        }];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], true);
        assert_eq!(
            entries[0].content_type, "unique_id",
            "text-independent detection must classify ALL_UNIQUE even when frequency's sentinel \
             text was customized"
        );
    }

    #[test]
    fn generate_does_not_misclassify_high_cardinality_as_unique_id() {
        // HIGH_CARDINALITY fields also produce a single frequency row with
        // count==row_count and percentage==100.0, but their cardinality is
        // strictly less than row_count (some values repeat). The
        // `count == cardinality` check correctly excludes them.
        let stats = vec![StatsRecord {
            field:       "city".to_string(),
            r#type:      "String".to_string(),
            cardinality: 800, // many distinct values, but with repeats
            nullcount:   0,
            min:         "Aachen".to_string(),
            max:         "Zurich".to_string(),
            addl_cols:   IndexMap::new(),
        }];
        let frequencies = vec![FrequencyRecord {
            field:      "city".to_string(),
            value:      "<HIGH_CARDINALITY>".to_string(),
            count:      10_000, // row_count >> cardinality
            percentage: 100.0,
            rank:       1.0,
        }];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], true);
        assert!(
            entries[0].content_type.is_empty(),
            "HIGH_CARDINALITY field must not be classified as unique_id"
        );
    }

    #[test]
    fn generate_does_not_mislabel_unique_id_when_nulls_present() {
        // Semantic contract: "every row has a distinct non-null value". A
        // field where cardinality == count but nullcount > 0 doesn't qualify
        // (some rows have no value at all). qsv frequency wouldn't emit the
        // ALL_UNIQUE sentinel for this case anyway, but the explicit
        // nullcount==0 check provides defense in depth for hand-crafted or
        // cached frequency input.
        let stats = vec![StatsRecord {
            field:       "maybe_id".to_string(),
            r#type:      "Integer".to_string(),
            cardinality: 95,
            nullcount:   5, // 5 rows are NULL
            min:         "1".to_string(),
            max:         "95".to_string(),
            addl_cols:   IndexMap::new(),
        }];
        let frequencies = vec![FrequencyRecord {
            field:      "maybe_id".to_string(),
            value:      "<ALL_UNIQUE>".to_string(),
            count:      95,
            percentage: 100.0,
            rank:       1.0,
        }];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], true);
        assert!(
            entries[0].content_type.is_empty(),
            "a field with nulls must NOT be classified as unique_id even if non-null cardinality \
             matches the frequency count"
        );
    }

    #[test]
    fn combine_with_baseline_preserves_baseline_when_refine_omits_field() {
        // The single biggest correctness pitfall for --two-pass: if the LLM's refine
        // response leaves out a field, that field's Label/Description must inherit the
        // first-pass values rather than reverting to code-derived defaults.
        let code_entries = vec![blank_entry("kept"), blank_entry("refined")];

        let mut baseline = HashMap::new();
        baseline.insert(
            "kept".to_string(),
            LlmDictField {
                label:        "Baseline Label".to_string(),
                description:  "Baseline description.".to_string(),
                content_type: "email".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );
        baseline.insert(
            "refined".to_string(),
            LlmDictField {
                label:        "Old Label".to_string(),
                description:  "Old description.".to_string(),
                content_type: "free_text".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );

        // Refine pass only returns "refined" — "kept" is intentionally absent.
        let mut refine = HashMap::new();
        refine.insert(
            "refined".to_string(),
            LlmDictField {
                label:        "New Label".to_string(),
                description:  "New description with cross-field context.".to_string(),
                content_type: "address_street".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );

        let combined =
            combine_dictionary_entries_with_baseline(code_entries, &baseline, &refine, true);

        // "kept" inherits baseline values verbatim.
        assert_eq!(combined[0].label, "Baseline Label");
        assert_eq!(combined[0].description, "Baseline description.");
        assert_eq!(combined[0].content_type, "email");
        // "refined" gets refine-pass overrides.
        assert_eq!(combined[1].label, "New Label");
        assert_eq!(
            combined[1].description,
            "New description with cross-field context."
        );
        assert_eq!(combined[1].content_type, "address_street");
    }

    #[test]
    fn combine_with_baseline_rejects_refine_supplied_unique_id() {
        // The deterministic "unique_id" stamp (cardinality == rowcount) must survive
        // the refine pass even if the LLM tries to overwrite it with a different vocab
        // token — and the refine pass also cannot smuggle in a fabricated "unique_id"
        // for a non-ALL_UNIQUE field. Mirrors the single-pass guarantees in
        // combine_dictionary_entries.
        let mut pk = blank_entry("pk");
        pk.content_type = "unique_id".to_string();
        let code_entries = vec![pk, blank_entry("other")];

        // Baseline is empty: this asserts the refine pass is gated independently.
        let baseline = HashMap::new();

        let mut refine = HashMap::new();
        refine.insert(
            "pk".to_string(),
            LlmDictField {
                label:        "Primary Key".to_string(),
                description:  "row id".to_string(),
                // Refine pass tries to overwrite the deterministic stamp with "uuid".
                content_type: "uuid".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );
        refine.insert(
            "other".to_string(),
            LlmDictField {
                label:        "Other".to_string(),
                description:  "not unique".to_string(),
                // Refine pass tries to smuggle "unique_id" onto a non-ALL_UNIQUE field.
                content_type: "unique_id".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );

        let combined =
            combine_dictionary_entries_with_baseline(code_entries, &baseline, &refine, true);

        assert_eq!(
            combined[0].content_type, "unique_id",
            "deterministic 'unique_id' stamp must survive refine-pass overwrite"
        );
        assert_eq!(
            combined[1].content_type, "unknown",
            "refine-supplied 'unique_id' for a non-ALL_UNIQUE field must be rejected and coerced \
             to 'unknown' (the infer_content_type=true default)"
        );
    }

    #[test]
    fn combine_with_baseline_refine_overrides_valid_baseline_content_type() {
        // The whole point of --two-pass: the refine pass can upgrade a baseline content_type
        // to a better vocab token once it sees cross-field context (e.g. "free_text" ->
        // "address_street" after recognizing sibling city / state / zip columns).
        let code_entries = vec![blank_entry("street1")];

        let mut baseline = HashMap::new();
        baseline.insert(
            "street1".to_string(),
            LlmDictField {
                label:        "Street 1".to_string(),
                description:  "First street line".to_string(),
                content_type: "free_text".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );

        let mut refine = HashMap::new();
        refine.insert(
            "street1".to_string(),
            LlmDictField {
                label:        "Street Address".to_string(),
                description:  "Street component of the mailing address.".to_string(),
                content_type: "address_street".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );

        let combined =
            combine_dictionary_entries_with_baseline(code_entries, &baseline, &refine, true);
        assert_eq!(combined[0].content_type, "address_street");
        assert_eq!(combined[0].label, "Street Address");
    }

    #[test]
    fn combine_with_baseline_matches_single_pass_when_refine_empty() {
        // Sanity check: with an empty refine map, the baseline-preserving variant must
        // produce the same result as the single-pass combine. Guards against accidental
        // divergence between the two functions during future refactors.
        let mut pk = blank_entry("pk");
        pk.content_type = "unique_id".to_string();
        let code_entries = vec![pk, blank_entry("city")];

        let mut baseline = HashMap::new();
        baseline.insert(
            "city".to_string(),
            LlmDictField {
                label:        "City".to_string(),
                description:  "City name".to_string(),
                content_type: "city".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );

        let single_pass = combine_dictionary_entries(code_entries.clone(), &baseline, true);
        let two_pass = combine_dictionary_entries_with_baseline(
            code_entries,
            &baseline,
            &HashMap::new(),
            true,
        );

        assert_eq!(single_pass.len(), two_pass.len());
        for (a, b) in single_pass.iter().zip(two_pass.iter()) {
            assert_eq!(a.name, b.name);
            assert_eq!(a.label, b.label);
            assert_eq!(a.description, b.description);
            assert_eq!(a.content_type, b.content_type);
        }
    }

    fn entry_with_content_type(name: &str, content_type: &str) -> DictionaryEntry {
        DictionaryEntry {
            content_type: content_type.to_string(),
            ..blank_entry(name)
        }
    }

    #[test]
    fn normalize_datetime_token_handles_all_forms() {
        // Bare tokens accepted.
        assert_eq!(normalize_datetime_token("date").as_deref(), Some("date"));
        assert_eq!(
            normalize_datetime_token("datetime").as_deref(),
            Some("datetime")
        );
        // The token prefix is case-folded.
        assert_eq!(
            normalize_datetime_token("DateTime").as_deref(),
            Some("datetime")
        );
        // A valid strftime suffix is preserved verbatim, including its case.
        assert_eq!(
            normalize_datetime_token("date:%Y-%m-%d").as_deref(),
            Some("date:%Y-%m-%d")
        );
        assert_eq!(
            normalize_datetime_token("datetime:%m/%d/%Y %I:%M:%S %p").as_deref(),
            Some("datetime:%m/%d/%Y %I:%M:%S %p")
        );
        // Colons inside the format survive — split_once only consumes the first ':'.
        assert_eq!(
            normalize_datetime_token("datetime:%H:%M:%S").as_deref(),
            Some("datetime:%H:%M:%S")
        );
        // A malformed strftime suffix degrades to the bare token.
        assert_eq!(normalize_datetime_token("date:%Q").as_deref(), Some("date"));
        // An empty suffix degrades to the bare token.
        assert_eq!(normalize_datetime_token("date:").as_deref(), Some("date"));
        // Non-date tokens return None so the caller falls through to the vocab check.
        assert_eq!(normalize_datetime_token("time"), None);
        assert_eq!(normalize_datetime_token("duration:3600"), None);
        assert_eq!(normalize_datetime_token("email"), None);
        assert_eq!(normalize_datetime_token(""), None);
    }

    #[test]
    fn parse_llm_response_preserves_date_format_casing() {
        // The date/datetime <fmt> suffix is case-sensitive (%m month vs %M minute,
        // %p AM/PM) and must survive the parser's general lowercasing.
        let json = r#"{
            "created": {"label": "C", "description": "d", "content_type": "datetime:%m/%d/%Y %I:%M:%S %p"},
            "born":    {"label": "B", "description": "d", "content_type": "DATE:%Y-%m-%d"}
        }"#;
        let fields = vec!["created".to_string(), "born".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert_eq!(
            parsed.get("created").unwrap().content_type,
            "datetime:%m/%d/%Y %I:%M:%S %p"
        );
        // token prefix case-folded, format suffix preserved verbatim
        assert_eq!(parsed.get("born").unwrap().content_type, "date:%Y-%m-%d");
    }

    #[test]
    fn parse_llm_relationships_extracts_and_validates() {
        let field_names = vec![
            "created_date".to_string(),
            "closed_date".to_string(),
            "city".to_string(),
            "state".to_string(),
        ];
        let response = r#"{
            "created_date": {"label": "Created", "description": "x"},
            "relationships": [
                {"kind": "ordered", "members": ["created_date", "closed_date"], "anchor": "created_date"},
                {"kind": "joint", "members": ["city", "state"]},
                {"kind": "bogus", "members": ["city", "state"]},
                {"kind": "joint", "members": ["city", "missing_col"]},
                {"kind": "correlated", "members": ["city"]}
            ]
        }"#;
        let rels = parse_llm_relationships(response, &field_names);
        // bogus kind, unknown member, and the 1-member group are all dropped.
        assert_eq!(rels.len(), 2);
        assert_eq!(rels[0]["kind"], "ordered");
        assert_eq!(
            rels[0]["members"],
            serde_json::json!(["created_date", "closed_date"])
        );
        assert_eq!(rels[0]["anchor"], "created_date");
        assert_eq!(rels[1]["kind"], "joint");
        // `anchor` is only kept for `ordered` groups.
        assert!(rels[1].get("anchor").is_none());
    }

    #[test]
    fn parse_llm_relationships_absent_is_empty() {
        let field_names = vec!["a".to_string(), "b".to_string()];
        let response = r#"{"a": {"label": "A", "description": "x"}}"#;
        assert!(parse_llm_relationships(response, &field_names).is_empty());
    }

    #[test]
    fn parse_llm_relationships_drops_invalid_anchor() {
        let field_names = vec!["lo".to_string(), "hi".to_string()];
        let response = r#"{
            "relationships": [
                {"kind": "ordered", "members": ["lo", "hi"], "anchor": "not_a_column"}
            ]
        }"#;
        let rels = parse_llm_relationships(response, &field_names);
        // The relationship survives (members are valid) but the bad anchor is dropped.
        assert_eq!(rels.len(), 1);
        assert!(rels[0].get("anchor").is_none());
    }

    #[test]
    fn generate_stamps_date_datetime_from_stats_type() {
        let stats = vec![
            StatsRecord {
                field:       "created".to_string(),
                r#type:      "DateTime".to_string(),
                cardinality: 50,
                nullcount:   0,
                min:         "2020-01-01T00:00:00+00:00".to_string(),
                max:         "2020-12-31T00:00:00+00:00".to_string(),
                addl_cols:   IndexMap::new(),
            },
            StatsRecord {
                field:       "born".to_string(),
                r#type:      "Date".to_string(),
                cardinality: 50,
                nullcount:   0,
                min:         "1990-01-01".to_string(),
                max:         "1999-12-31".to_string(),
                addl_cols:   IndexMap::new(),
            },
            StatsRecord {
                field:       "name".to_string(),
                r#type:      "String".to_string(),
                cardinality: 50,
                nullcount:   0,
                min:         "a".to_string(),
                max:         "z".to_string(),
                addl_cols:   IndexMap::new(),
            },
        ];
        // non-ALL_UNIQUE frequency rows so nothing is stamped unique_id
        let frequencies = vec![
            FrequencyRecord {
                field:      "created".to_string(),
                value:      "x".to_string(),
                count:      10,
                percentage: 20.0,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "born".to_string(),
                value:      "y".to_string(),
                count:      10,
                percentage: 20.0,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "name".to_string(),
                value:      "z".to_string(),
                count:      10,
                percentage: 20.0,
                rank:       1.0,
            },
        ];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], true);
        assert_eq!(entries[0].content_type, "datetime");
        assert_eq!(entries[1].content_type, "date");
        assert!(entries[2].content_type.is_empty());

        // with infer_content_type off, nothing is stamped
        let off = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], false);
        assert!(off.iter().all(|e| e.content_type.is_empty()));
    }

    #[test]
    fn generate_unique_id_wins_over_date_stamp() {
        // An ALL_UNIQUE DateTime column (a unique timestamp key) is stamped
        // `unique_id`, not `datetime` — the `unique_id` stamp keeps priority.
        let stats = vec![StatsRecord {
            field:       "ts_pk".to_string(),
            r#type:      "DateTime".to_string(),
            cardinality: 1000,
            nullcount:   0,
            min:         "2020-01-01T00:00:00+00:00".to_string(),
            max:         "2020-12-31T00:00:00+00:00".to_string(),
            addl_cols:   IndexMap::new(),
        }];
        let frequencies = vec![FrequencyRecord {
            field:      "ts_pk".to_string(),
            value:      "<ALL_UNIQUE>".to_string(),
            count:      1000,
            percentage: 100.0,
            rank:       1.0,
        }];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], true);
        assert_eq!(entries[0].content_type, "unique_id");
    }

    #[test]
    fn combine_merges_date_format_suffix_onto_stamped_token() {
        // code stamped a bare "datetime"; the LLM contributes the strftime suffix
        let code = vec![entry_with_content_type("created", "datetime")];
        let mut llm = HashMap::new();
        llm.insert(
            "created".to_string(),
            LlmDictField {
                label:        "Created".to_string(),
                description:  "d".to_string(),
                content_type: "datetime:%m/%d/%Y".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );
        let combined = combine_dictionary_entries(code, &llm, true);
        assert_eq!(combined[0].content_type, "datetime:%m/%d/%Y");
    }

    #[test]
    fn combine_keeps_bare_date_token_when_llm_base_mismatches() {
        // code stamped bare "date"; the LLM disagrees on the base token —
        // the deterministic classification wins, the mismatched suffix is dropped
        let code = vec![entry_with_content_type("born", "date")];
        let mut llm = HashMap::new();
        llm.insert(
            "born".to_string(),
            LlmDictField {
                label:        "Born".to_string(),
                description:  "d".to_string(),
                content_type: "datetime:%Y-%m-%dT%H:%M:%S".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );
        let combined = combine_dictionary_entries(code, &llm, true);
        assert_eq!(combined[0].content_type, "date");
    }

    #[test]
    fn combine_drops_llm_date_token_on_non_date_column() {
        // the LLM may not introduce a date/datetime token onto a column the
        // stats did not type as such — only the deterministic stamp may
        let code = vec![entry_with_content_type("note", "")];
        let mut llm = HashMap::new();
        llm.insert(
            "note".to_string(),
            LlmDictField {
                label:        "Note".to_string(),
                description:  "d".to_string(),
                content_type: "date:%Y-%m-%d".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );
        let combined = combine_dictionary_entries(code, &llm, true);
        // dropped → falls back to "unknown"
        assert_eq!(combined[0].content_type, "unknown");
    }

    #[test]
    fn combine_with_baseline_refine_corrects_date_format() {
        // the refine pass may correct the <fmt> suffix on an already-date-typed column
        let code = vec![entry_with_content_type("created", "datetime")];
        let mut baseline = HashMap::new();
        baseline.insert(
            "created".to_string(),
            LlmDictField {
                label:        "C".to_string(),
                description:  "d".to_string(),
                content_type: "datetime:%Y/%m/%d".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );
        let mut refine = HashMap::new();
        refine.insert(
            "created".to_string(),
            LlmDictField {
                label:        "C".to_string(),
                description:  "d".to_string(),
                content_type: "datetime:%m/%d/%Y".to_string(),
                concept:      String::new(),
                role:         String::new(),
            },
        );
        let combined = combine_dictionary_entries_with_baseline(code, &baseline, &refine, true);
        assert_eq!(combined[0].content_type, "datetime:%m/%d/%Y");
    }

    #[test]
    fn validate_date_formats_strips_mismatched_suffix() {
        // the entry claims %m/%d/%Y but the real sample is ISO → suffix stripped
        let mut entries = vec![entry_with_content_type("d", "date:%m/%d/%Y")];
        let freqs = vec![FrequencyRecord {
            field:      "d".to_string(),
            value:      "2020-01-15".to_string(),
            count:      5,
            percentage: 10.0,
            rank:       1.0,
        }];
        validate_date_formats(&mut entries, &freqs, "(NULL)");
        assert_eq!(entries[0].content_type, "date");
    }

    #[test]
    fn validate_date_formats_keeps_matching_suffix() {
        let mut entries = vec![entry_with_content_type(
            "d",
            "datetime:%m/%d/%Y %I:%M:%S %p",
        )];
        let freqs = vec![FrequencyRecord {
            field:      "d".to_string(),
            value:      "01/24/2013 12:00:00 AM".to_string(),
            count:      5,
            percentage: 10.0,
            rank:       1.0,
        }];
        validate_date_formats(&mut entries, &freqs, "(NULL)");
        assert_eq!(entries[0].content_type, "datetime:%m/%d/%Y %I:%M:%S %p");
    }

    #[test]
    fn validate_date_formats_accepts_offset_datetime() {
        // %z offset datetimes: NaiveDateTime rejects them, DateTime::parse_from_str accepts
        let mut entries = vec![entry_with_content_type("d", "datetime:%Y-%m-%dT%H:%M:%S%z")];
        let freqs = vec![FrequencyRecord {
            field:      "d".to_string(),
            value:      "2020-01-15T08:30:00+0000".to_string(),
            count:      5,
            percentage: 10.0,
            rank:       1.0,
        }];
        validate_date_formats(&mut entries, &freqs, "(NULL)");
        assert_eq!(entries[0].content_type, "datetime:%Y-%m-%dT%H:%M:%S%z");
    }

    #[test]
    fn validate_date_formats_keeps_suffix_when_no_sample() {
        // an ALL_UNIQUE column has only the <ALL_UNIQUE> sentinel row — there is
        // nothing to validate against, so the suffix is left unchanged
        let mut entries = vec![entry_with_content_type("d", "date:%d/%m/%Y")];
        let freqs = vec![FrequencyRecord {
            field:      "d".to_string(),
            value:      "<ALL_UNIQUE>".to_string(),
            count:      100,
            percentage: 100.0,
            rank:       1.0,
        }];
        validate_date_formats(&mut entries, &freqs, "(NULL)");
        assert_eq!(entries[0].content_type, "date:%d/%m/%Y");
    }

    #[test]
    fn strip_time_from_format_keeps_date_drops_time() {
        assert_eq!(
            strip_time_from_format("%m/%d/%Y %I:%M:%S %p").as_deref(),
            Some("%m/%d/%Y")
        );
        assert_eq!(
            strip_time_from_format("%Y-%m-%dT%H:%M:%S").as_deref(),
            Some("%Y-%m-%d")
        );
        assert_eq!(
            strip_time_from_format("%d-%b-%Y %H:%M").as_deref(),
            Some("%d-%b-%Y")
        );
        // a timezone-bearing time part still truncates cleanly
        assert_eq!(
            strip_time_from_format("%Y-%m-%dT%H:%M:%S%z").as_deref(),
            Some("%Y-%m-%d")
        );
        // a `-` date/time separator is trimmed, not left dangling
        assert_eq!(
            strip_time_from_format("%Y-%m-%d-%H:%M:%S").as_deref(),
            Some("%Y-%m-%d")
        );
        // a pure-date format is returned unchanged
        assert_eq!(
            strip_time_from_format("%Y-%m-%d").as_deref(),
            Some("%Y-%m-%d")
        );
        // a pure-time format has no date prefix
        assert_eq!(strip_time_from_format("%H:%M:%S"), None);
        // datetime-compound specifiers have no clean date-only prefix
        assert_eq!(strip_time_from_format("%+"), None);
    }

    #[test]
    fn downgrade_reclassifies_all_midnight_datetime_as_date() {
        let mut entries = vec![entry_with_content_type(
            "created",
            "datetime:%m/%d/%Y %I:%M:%S %p",
        )];
        let freqs = vec![
            FrequencyRecord {
                field:      "created".to_string(),
                value:      "01/24/2013 12:00:00 AM".to_string(),
                count:      347,
                percentage: 0.03,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "created".to_string(),
                value:      "01/07/2014 12:00:00 AM".to_string(),
                count:      315,
                percentage: 0.03,
                rank:       2.0,
            },
        ];
        downgrade_all_midnight_datetime_columns(&mut entries, &freqs, "(NULL)");
        // every sample is at midnight → date, with the time stripped from <fmt>
        assert_eq!(entries[0].content_type, "date:%m/%d/%Y");
    }

    #[test]
    fn downgrade_keeps_datetime_when_a_sample_has_a_real_time() {
        let mut entries = vec![entry_with_content_type(
            "due",
            "datetime:%m/%d/%Y %I:%M:%S %p",
        )];
        let freqs = vec![
            FrequencyRecord {
                field:      "due".to_string(),
                value:      "04/08/2015 10:00:58 AM".to_string(),
                count:      214,
                percentage: 0.06,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "due".to_string(),
                value:      "05/02/2014 12:00:00 AM".to_string(),
                count:      183,
                percentage: 0.05,
                rank:       2.0,
            },
        ];
        downgrade_all_midnight_datetime_columns(&mut entries, &freqs, "(NULL)");
        // one sample carries a non-midnight time → stays datetime
        assert_eq!(entries[0].content_type, "datetime:%m/%d/%Y %I:%M:%S %p");
    }

    #[test]
    fn downgrade_keeps_datetime_when_a_sample_is_unparsable() {
        // a usable frequency value that does not parse with the inferred
        // format blocks the downgrade — a non-parsing real value could carry
        // a non-midnight time, so it must not be silently dropped
        let mut entries = vec![entry_with_content_type(
            "closed",
            "datetime:%m/%d/%Y %I:%M:%S %p",
        )];
        let freqs = vec![
            FrequencyRecord {
                field:      "closed".to_string(),
                value:      "11/15/2010 12:00:00 AM".to_string(),
                count:      384,
                percentage: 0.04,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "closed".to_string(),
                value:      "not-a-date".to_string(),
                count:      28619,
                percentage: 2.0,
                rank:       2.0,
            },
        ];
        downgrade_all_midnight_datetime_columns(&mut entries, &freqs, "(NULL)");
        assert_eq!(entries[0].content_type, "datetime:%m/%d/%Y %I:%M:%S %p");
    }

    #[test]
    fn downgrade_leaves_non_datetime_entries_alone() {
        let mut entries = vec![
            entry_with_content_type("bare_dt", "datetime"),
            entry_with_content_type("a_date", "date:%Y-%m-%d"),
            entry_with_content_type("contact", "email"),
        ];
        let freqs = vec![
            FrequencyRecord {
                field:      "bare_dt".to_string(),
                value:      "01/24/2013 12:00:00 AM".to_string(),
                count:      1,
                percentage: 0.1,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "a_date".to_string(),
                value:      "2013-01-24".to_string(),
                count:      1,
                percentage: 0.1,
                rank:       1.0,
            },
        ];
        downgrade_all_midnight_datetime_columns(&mut entries, &freqs, "(NULL)");
        // bare `datetime` has no format to test against → unchanged
        assert_eq!(entries[0].content_type, "datetime");
        // an already-`date` entry → unchanged
        assert_eq!(entries[1].content_type, "date:%Y-%m-%d");
        // a non-date token → unchanged
        assert_eq!(entries[2].content_type, "email");
    }

    #[test]
    fn downgrade_keeps_datetime_when_no_usable_sample() {
        // an ALL_UNIQUE datetime column has no usable frequency sample
        let mut entries = vec![entry_with_content_type(
            "ts",
            "datetime:%m/%d/%Y %I:%M:%S %p",
        )];
        let freqs = vec![FrequencyRecord {
            field:      "ts".to_string(),
            value:      "<ALL_UNIQUE>".to_string(),
            count:      1000,
            percentage: 100.0,
            rank:       1.0,
        }];
        downgrade_all_midnight_datetime_columns(&mut entries, &freqs, "(NULL)");
        assert_eq!(entries[0].content_type, "datetime:%m/%d/%Y %I:%M:%S %p");
    }

    #[test]
    fn validate_date_formats_checks_all_samples_not_just_first() {
        // `%m/%d/%Y` parses the first sample `01/02/2020` but a later sample
        // `13/02/2020` (unambiguously day-first) disproves it — validating
        // against only the first sample would wrongly keep the suffix.
        let mut entries = vec![entry_with_content_type("d", "date:%m/%d/%Y")];
        let freqs = vec![
            FrequencyRecord {
                field:      "d".to_string(),
                value:      "01/02/2020".to_string(),
                count:      5,
                percentage: 10.0,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "d".to_string(),
                value:      "13/02/2020".to_string(),
                count:      4,
                percentage: 8.0,
                rank:       2.0,
            },
        ];
        validate_date_formats(&mut entries, &freqs, "(NULL)");
        assert_eq!(entries[0].content_type, "date");
    }

    #[test]
    fn validate_date_formats_ignores_ranked_null_rows() {
        // `frequency --pct-nulls` emits the null row with a real (non-zero)
        // rank. It is identified by its `--null-text` value AND a count equal
        // to the column's null count, so it does not count as a sample —
        // otherwise it fails to parse and strips an otherwise-valid format.
        // The null label here is custom (`<MISSING>`), threaded in as
        // `null_text`, proving the check honors a configured `--null-text`.
        let mut entries = vec![DictionaryEntry {
            null_count: 5,
            ..entry_with_content_type("d", "date:%Y-%m-%d")
        }];
        let freqs = vec![
            FrequencyRecord {
                field:      "d".to_string(),
                value:      "2020-01-15".to_string(),
                count:      8,
                percentage: 61.5,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "d".to_string(),
                value:      "<MISSING>".to_string(),
                count:      5,
                percentage: 38.5,
                rank:       2.0,
            },
        ];
        validate_date_formats(&mut entries, &freqs, "<MISSING>");
        assert_eq!(entries[0].content_type, "date:%Y-%m-%d");
    }

    #[test]
    fn downgrade_ignores_ranked_null_rows() {
        // a ranked null row — its `--null-text` value plus a count equal to
        // the column's null count — must not count as a sample: it would fail
        // to parse and block the downgrade of an otherwise all-midnight
        // datetime column.
        let mut entries = vec![DictionaryEntry {
            null_count: 5,
            ..entry_with_content_type("closed", "datetime:%m/%d/%Y %I:%M:%S %p")
        }];
        let freqs = vec![
            FrequencyRecord {
                field:      "closed".to_string(),
                value:      "11/15/2010 12:00:00 AM".to_string(),
                count:      8,
                percentage: 61.5,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "closed".to_string(),
                value:      "(NULL)".to_string(),
                count:      5,
                percentage: 38.5,
                rank:       2.0,
            },
        ];
        downgrade_all_midnight_datetime_columns(&mut entries, &freqs, "(NULL)");
        assert_eq!(entries[0].content_type, "date:%m/%d/%Y");
    }

    #[test]
    fn validate_date_formats_keeps_real_sample_sharing_null_count() {
        // a real, non-null value whose count merely equals the column's null
        // count must still be validated — identifying the null row by count
        // alone would drop it. Here that sample (`2020-01-15`, ISO) disproves
        // the inferred `%m/%d/%Y` suffix; only the genuine `(NULL)` row, which
        // also matches `--null-text`, is excluded.
        let mut entries = vec![DictionaryEntry {
            null_count: 5,
            ..entry_with_content_type("d", "date:%m/%d/%Y")
        }];
        let freqs = vec![
            FrequencyRecord {
                field:      "d".to_string(),
                value:      "2020-01-15".to_string(),
                count:      5,
                percentage: 35.7,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "d".to_string(),
                value:      "(NULL)".to_string(),
                count:      5,
                percentage: 35.7,
                rank:       2.0,
            },
        ];
        validate_date_formats(&mut entries, &freqs, "(NULL)");
        assert_eq!(entries[0].content_type, "date");
    }

    #[test]
    fn downgrade_keeps_datetime_when_non_midnight_sample_shares_null_count() {
        // a real non-midnight value whose count merely equals the column's
        // null count must still block the downgrade — identifying the null row
        // by count alone would drop it and wrongly reclassify the column as
        // `date`. Only the genuine `(NULL)` row is excluded.
        let mut entries = vec![DictionaryEntry {
            null_count: 5,
            ..entry_with_content_type("closed", "datetime:%m/%d/%Y %I:%M:%S %p")
        }];
        let freqs = vec![
            FrequencyRecord {
                field:      "closed".to_string(),
                value:      "11/15/2010 12:00:00 AM".to_string(),
                count:      8,
                percentage: 44.4,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "closed".to_string(),
                value:      "04/08/2015 10:00:58 AM".to_string(),
                count:      5,
                percentage: 27.8,
                rank:       2.0,
            },
            FrequencyRecord {
                field:      "closed".to_string(),
                value:      "(NULL)".to_string(),
                count:      5,
                percentage: 27.8,
                rank:       3.0,
            },
        ];
        downgrade_all_midnight_datetime_columns(&mut entries, &freqs, "(NULL)");
        assert_eq!(entries[0].content_type, "datetime:%m/%d/%Y %I:%M:%S %p");
    }

    #[test]
    fn parse_llm_response_extracts_validated_role_and_concept() {
        let json = r#"{
          "zip": {"label":"ZIP","description":"postal code","content_type":"zip_code","role":"dimension","concept":"geo.zip_code"},
          "bad": {"label":"B","description":"d","role":"sideways","concept":"not.a.concept"},
          "sk":  {"label":"K","description":"key","concept":"id.surrogate_key","role":"identifier"}
        }"#;
        let fields = vec!["zip".to_string(), "bad".to_string(), "sk".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert_eq!(parsed["zip"].role, "dimension");
        assert_eq!(parsed["zip"].concept, "geo.zip_code");
        // Invalid role + out-of-vocab concept are dropped to empty (the seed/fallback fills them).
        assert_eq!(parsed["bad"].role, "");
        assert_eq!(parsed["bad"].concept, "");
        // `id.surrogate_key` is reserved/deterministic — rejected from LLM input (like unique_id).
        assert_eq!(parsed["sk"].concept, "");
        assert_eq!(parsed["sk"].role, "identifier");
    }

    #[test]
    fn parse_llm_response_ignores_role_concept_when_flag_off() {
        let json = r#"{"zip":{"label":"ZIP","description":"d","role":"dimension","concept":"geo.zip_code"}}"#;
        let fields = vec!["zip".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, false).unwrap();
        assert_eq!(parsed["zip"].role, "");
        assert_eq!(parsed["zip"].concept, "");
    }

    #[test]
    fn parse_llm_grain_reads_top_level_string() {
        assert_eq!(
            parse_llm_grain(r#"{"grain":"one row = one trip"}"#).as_deref(),
            Some("one row = one trip")
        );
        assert_eq!(parse_llm_grain(r#"{"grain":"   "}"#), None);
        assert_eq!(parse_llm_grain(r#"{"foo":1}"#), None);
        assert_eq!(parse_llm_grain("not json at all"), None);
    }

    #[test]
    fn concept_from_content_type_maps_known_tokens() {
        assert_eq!(concept_from_content_type("zip_code"), Some("geo.zip_code"));
        assert_eq!(
            concept_from_content_type("unique_id"),
            Some("id.surrogate_key")
        );
        assert_eq!(
            concept_from_content_type("datetime"),
            Some("time.event_timestamp")
        );
        assert_eq!(concept_from_content_type("free_text"), None);
    }

    #[test]
    fn is_linkable_concept_excludes_pii_and_unknown() {
        assert!(is_linkable_concept("geo.zip_code"));
        assert!(is_linkable_concept("nyc.bbl"));
        assert!(is_linkable_concept("id.foreign_key"));
        assert!(!is_linkable_concept("pii.email"));
        assert!(!is_linkable_concept("category.status"));
        assert!(!is_linkable_concept("unknown"));
        assert!(!is_linkable_concept(""));
    }

    #[test]
    fn generate_code_based_dictionary_seeds_concept_and_role() {
        let stats = vec![
            StatsRecord {
                field:       "id".to_string(),
                r#type:      "Integer".to_string(),
                cardinality: 3,
                nullcount:   0,
                min:         "1".to_string(),
                max:         "3".to_string(),
                addl_cols:   IndexMap::new(),
            },
            StatsRecord {
                field:       "created".to_string(),
                r#type:      "Date".to_string(),
                cardinality: 2,
                nullcount:   0,
                min:         "2020-01-01".to_string(),
                max:         "2020-12-31".to_string(),
                addl_cols:   IndexMap::new(),
            },
        ];
        let freqs = vec![
            FrequencyRecord {
                field:      "id".to_string(),
                value:      "<ALL_UNIQUE>".to_string(),
                count:      3,
                percentage: 100.0,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "created".to_string(),
                value:      "2020-01-01".to_string(),
                count:      1,
                percentage: 50.0,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "created".to_string(),
                value:      "2020-12-31".to_string(),
                count:      1,
                percentage: 50.0,
                rank:       2.0,
            },
        ];
        let entries = generate_code_based_dictionary(&stats, &freqs, 50, 5, 0, &[], true);

        let id = entries.iter().find(|e| e.name == "id").unwrap();
        assert!(id.is_unique_id);
        assert_eq!(id.concept, "id.surrogate_key");
        assert_eq!(id.role, "identifier");

        let created = entries.iter().find(|e| e.name == "created").unwrap();
        assert_eq!(created.concept, "time.date");
        assert_eq!(created.role, "timestamp");

        // With the flag off, no concept/role is seeded.
        let off = generate_code_based_dictionary(&stats, &freqs, 50, 5, 0, &[], false);
        assert_eq!(off.iter().find(|e| e.name == "id").unwrap().concept, "");
        assert_eq!(off.iter().find(|e| e.name == "id").unwrap().role, "");
    }
}
