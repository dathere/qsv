//! `--dictionary`: a JSON Schema (draft 2020-12) data dictionary of the CSV
//! written, built from the file's own metadata - no LLM involved.
//!
//! The shape is the one `describegpt --dictionary` writes (see
//! `format_dictionary_jsonschema`), so `viz smart --dictionary` & `validate`
//! read it as they read describegpt's: per-property `title`, `description`,
//! `type` & `enum`, with qsv's annotations under `x-qsv`.

use std::collections::{BTreeSet, HashMap, HashSet};

use polars::prelude::{AnyValue, DataFrame, DataType, PolarsResult, Schema};
use serde_json::{Map, Value, json};

use super::{F64_WHOLE_LIMIT, is_whole};

/// Distinct values tracked per column, to decide `enum`: beyond this many, a
/// column gets none.
const MAX_DISTINCT: usize = 1000;

/// What the batches written held, per column: its nulls, and - for the
/// columns whose `enum` or `null_values` depend on them - its distinct
/// values, as text.
#[derive(Default)]
pub(super) struct Tallies {
    rows:      u64,
    nulls:     HashMap<String, u64>,
    /// `None` once the column held more than [`MAX_DISTINCT`] values.
    distinct:  HashMap<String, Option<BTreeSet<String>>>,
    /// The sentinels seen in each `--sentinels-embedded` column. Kept apart
    /// from `distinct`, as a column of many values still has only a few.
    sentinels: HashMap<String, (SentinelMatcher, BTreeSet<String>)>,
}

impl Tallies {
    pub(super) fn new(
        distinct: impl IntoIterator<Item = String>,
        sentinels: impl IntoIterator<Item = (String, SentinelMatcher)>,
    ) -> Self {
        Self {
            distinct: distinct
                .into_iter()
                .map(|name| (name, Some(BTreeSet::new())))
                .collect(),
            sentinels: sentinels
                .into_iter()
                .map(|(name, matcher)| (name, (matcher, BTreeSet::new())))
                .collect(),
            ..Self::default()
        }
    }

    pub(super) fn update(&mut self, df: &DataFrame) -> PolarsResult<()> {
        self.rows += df.height() as u64;
        for col in df.columns() {
            let name = col.name().as_str();
            // an empty string is an empty cell too, as `validate` reads it back: the
            // .por reader stores missing strings that way
            let empty = match col.dtype() {
                DataType::String => col
                    .as_materialized_series()
                    .str()?
                    .iter()
                    .filter(|v| v.is_some_and(str::is_empty))
                    .count(),
                _ => 0,
            };
            *self.nulls.entry(name.to_string()).or_default() += (col.null_count() + empty) as u64;
            if let Some((matcher, seen)) = self.sentinels.get_mut(name)
                && *col.dtype() == DataType::String
            {
                for text in col.as_materialized_series().str()?.iter().flatten() {
                    if matcher.matches(text) {
                        seen.insert(text.to_string());
                    }
                }
            }
            let Some(slot) = self.distinct.get_mut(name) else {
                continue;
            };
            let Some(set) = slot else {
                continue;
            };
            let series = col.as_materialized_series();
            for value in series.iter() {
                let Some(text) = value_text(&value) else {
                    continue;
                };
                set.insert(text);
                if set.len() > MAX_DISTINCT {
                    *slot = None;
                    break;
                }
            }
        }
        Ok(())
    }

    fn null_count(&self, name: &str) -> u64 {
        self.nulls.get(name).copied().unwrap_or_default()
    }

    fn distinct(&self, name: &str) -> Option<&BTreeSet<String>> {
        self.distinct.get(name).and_then(Option::as_ref)
    }

    fn sentinels(&self, name: &str) -> Option<&BTreeSet<String>> {
        self.sentinels.get(name).map(|(_, seen)| seen)
    }
}

/// Tells the sentinels apart from the values in a `--sentinels-embedded`
/// column, from the variable's metadata rather than from what the text looks
/// like: a decoded value label is not a sentinel, and a sentinel's own label
/// can be any text.
pub(super) struct SentinelMatcher {
    /// Codes & labels written verbatim.
    exact:    HashSet<String>,
    /// SAS & Stata write an unlabeled sentinel as its tag, `.A` or `.a`.
    tags:     bool,
    /// SPSS's declared missing numbers, written as numbers.
    declared: Option<Value>,
}

impl SentinelMatcher {
    fn new(var: &Value, label_sentinels: bool) -> Self {
        let declared = missing_values(var);
        let mut exact = HashSet::new();
        let labels = var["value_labels"].as_object();
        for (key, label) in labels.into_iter().flatten() {
            let label = label.as_str().unwrap_or_default();
            let sentinel = if let Some(code) = sentinel_code(key) {
                exact.insert(code);
                true
            } else {
                key.parse::<f64>()
                    .is_ok_and(|n| declared_missing(declared.as_ref(), n))
            };
            if sentinel && label_sentinels {
                exact.insert(label.to_string());
            }
        }
        for code in var["missing_strings"].as_array().into_iter().flatten() {
            let Some(code) = code.as_str() else {
                continue;
            };
            exact.insert(code.to_string());
            if label_sentinels
                && let Some(label) = labels.and_then(|l| l.get(code)).and_then(Value::as_str)
            {
                exact.insert(label.to_string());
            }
        }
        Self {
            exact,
            tags: is_numeric_variable(var) && var.get("format_type").is_none(),
            declared,
        }
    }

    fn matches(&self, text: &str) -> bool {
        let text = text.trim_end();
        let tag = |t: &str| {
            let mut chars = t.chars();
            chars.next() == Some('.')
                && chars
                    .next()
                    .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
                && chars.next().is_none()
        };
        self.exact.contains(text)
            || (self.tags && tag(text))
            || text
                .parse::<f64>()
                .is_ok_and(|n| declared_missing(self.declared.as_ref(), n))
    }
}

/// A value as compared with metadata codes: numbers in their shortest form,
/// whole ones without a fraction (1.0 -> "1"), so a code read as "1" from the
/// metadata matches it.
fn value_text(value: &AnyValue) -> Option<String> {
    match value {
        AnyValue::Null => None,
        AnyValue::String("") => None,
        AnyValue::String(s) => Some((*s).to_string()),
        AnyValue::StringOwned(s) if s.is_empty() => None,
        AnyValue::StringOwned(s) => Some(s.to_string()),
        v => match v.extract::<f64>() {
            Some(n) if v.is_primitive_numeric() => Some(number_text(n)),
            _ => Some(v.to_string()),
        },
    }
}

fn number_text(n: f64) -> String {
    if is_whole(n, F64_WHOLE_LIMIT) {
        #[allow(clippy::cast_possible_truncation)]
        let whole = n as i64;
        whole.to_string()
    } else {
        n.to_string()
    }
}

fn number_json(n: f64) -> Value {
    if is_whole(n, F64_WHOLE_LIMIT) {
        #[allow(clippy::cast_possible_truncation)]
        let whole = n as i64;
        json!(whole)
    } else {
        json!(n)
    }
}

/// How the CSV was written, as the dictionary records it.
pub(super) struct Written<'a> {
    pub file_name:       &'a str,
    pub source:          &'a str,
    pub value_labels:    bool,
    /// Set when `QSV_POLARS_FLOAT_PRECISION` rounds the floats written.
    pub float_precision: Option<usize>,
    /// The flags in effect, recorded under `x-qsv.readstat_flags`.
    pub flags:           Value,
}

/// The `qsv stats`-style type of a column as written, & its JSON Schema type.
fn types(dtype: &DataType) -> (&'static str, &'static str) {
    match dtype {
        dt if dt.is_integer() => ("Integer", "integer"),
        dt if dt.is_float() => ("Float", "number"),
        DataType::Boolean => ("Boolean", "boolean"),
        DataType::Date => ("Date", "string"),
        DataType::Datetime(..) => ("DateTime", "string"),
        DataType::Null => ("NULL", "null"),
        _ => ("String", "string"),
    }
}

fn is_numeric_variable(var: &Value) -> bool {
    var["type"]
        .as_str()
        .is_some_and(|t| t.starts_with("Numeric"))
}

/// A sentinel's code as the readers write it: Stata `MISSING_a` -> `.a`, SAS
/// `MISSING_A` -> `.A`.
fn sentinel_code(key: &str) -> Option<String> {
    key.strip_prefix("MISSING_").map(|tag| format!(".{tag}"))
}

/// The variable's value labels, as `[{value, label}]`: numeric codes as
/// numbers, in order, then text codes, then sentinels.
fn value_labels(var: &Value) -> Vec<(Value, String)> {
    let Some(labels) = var["value_labels"].as_object() else {
        return Vec::new();
    };
    let numeric = is_numeric_variable(var);
    let mut numbers: Vec<(f64, String)> = Vec::new();
    let mut texts: Vec<(String, String)> = Vec::new();
    let mut sentinels: Vec<(String, String)> = Vec::new();
    for (key, label) in labels {
        let label = label.as_str().unwrap_or_default().to_string();
        if let Some(code) = sentinel_code(key) {
            sentinels.push((code, label));
        } else if numeric && let Ok(n) = key.parse::<f64>() {
            numbers.push((n, label));
        } else {
            texts.push((key.clone(), label));
        }
    }
    numbers.sort_by(|a, b| a.0.total_cmp(&b.0));
    texts.sort();
    sentinels.sort();
    numbers
        .into_iter()
        .map(|(n, l)| (number_json(n), l))
        .chain(texts.into_iter().map(|(c, l)| (Value::String(c), l)))
        .chain(sentinels.into_iter().map(|(c, l)| (Value::String(c), l)))
        .collect()
}

/// The values the column may hold, if its value labels cover every value it
/// does hold: their codes, or their labels when `--value-labels` decoded them.
/// None if any value falls outside them (a partly labeled variable), so
/// `validate` never rejects the data the dictionary describes.
fn labeled_enum(
    labels: &[(Value, String)],
    missing: Option<&Value>,
    decoded: bool,
    dtype: &DataType,
    held: &BTreeSet<String>,
) -> Option<Vec<Value>> {
    let allowed: Vec<(String, Value)> = labels
        .iter()
        // sentinels & declared missing values are written as empty cells (or
        // in a sentinel column), never as a value of the variable
        .filter(|(code, _)| !code.as_str().is_some_and(|c| c.starts_with('.')))
        .filter(|(code, _)| !code.as_f64().is_some_and(|n| declared_missing(missing, n)))
        .map(|(code, label)| {
            if decoded {
                (label.clone(), Value::String(label.clone()))
            } else {
                match code {
                    // a numeric variable written as text (its sentinels embedded
                    // among its numbers): no enum, so None below
                    Value::Number(_) if *dtype == DataType::String => (String::new(), Value::Null),
                    Value::Number(n) => {
                        let n = n.as_f64().unwrap_or_default();
                        let value = if dtype.is_float() {
                            json!(n)
                        } else {
                            number_json(n)
                        };
                        (number_text(n), value)
                    },
                    other => (
                        other.as_str().unwrap_or_default().to_string(),
                        other.clone(),
                    ),
                }
            }
        })
        .collect();
    if allowed.is_empty()
        || allowed.iter().any(|(_, v)| v.is_null())
        || !held.iter().all(|h| allowed.iter().any(|(t, _)| t == h))
    {
        return None;
    }
    let mut seen = BTreeSet::new();
    Some(
        allowed
            .into_iter()
            .filter(|(text, _)| seen.insert(text.clone()))
            .map(|(_, value)| value)
            .collect(),
    )
}

/// SPSS's declared missing values: `{discrete}`, `{range, discrete}` or
/// `{strings}`. SAS & Stata declare none; their sentinels (`.a`, `.A`) are
/// listed among the value labels when labeled.
fn missing_values(var: &Value) -> Option<Value> {
    let strings: Vec<&Value> = var["missing_strings"]
        .as_array()
        .map(|a| a.iter().collect())
        .unwrap_or_default();
    if !strings.is_empty() {
        return Some(json!({ "strings": strings }));
    }
    let doubles: Vec<f64> = var["missing_doubles"]
        .as_array()?
        .iter()
        .filter_map(Value::as_f64)
        .collect();
    if doubles.is_empty() {
        return None;
    }
    let numbers = |v: &[f64]| v.iter().copied().map(number_json).collect::<Vec<_>>();
    if var["missing_range"].as_bool() == Some(true) && doubles.len() >= 2 {
        let mut missing = json!({ "range": numbers(&doubles[..2]) });
        if doubles.len() > 2 {
            missing["discrete"] = Value::Array(numbers(&doubles[2..]));
        }
        Some(missing)
    } else {
        Some(json!({ "discrete": numbers(&doubles) }))
    }
}

/// Whether SPSS declares `n` a missing value, discretely or by range.
fn declared_missing(missing: Option<&Value>, n: f64) -> bool {
    let Some(missing) = missing else {
        return false;
    };
    let discrete = missing["discrete"]
        .as_array()
        .is_some_and(|a| a.iter().any(|v| v.as_f64() == Some(n)));
    let in_range = missing["range"]
        .as_array()
        .and_then(|r| Some((r.first()?.as_f64()?, r.get(1)?.as_f64()?)))
        .is_some_and(|(lo, hi)| (lo..=hi).contains(&n));
    discrete || in_range
}

/// SPSS records a variable's display format as a numeric type code, width &
/// decimals; this names it the way SPSS (and pyreadstat) write it, e.g.
/// `F8.2`, `DTIME23.2`, `A3`.
fn spss_format(var: &Value) -> Option<String> {
    let name = match var["format_type"].as_u64()? {
        1 => "A",
        2 => "AHEX",
        3 => "COMMA",
        4 => "DOLLAR",
        5 => "F",
        6 => "IB",
        7 => "PIBHEX",
        8 => "P",
        9 => "PIB",
        10 => "PK",
        11 => "RB",
        12 => "RBHEX",
        15 => "Z",
        16 => "N",
        17 => "E",
        20 => "DATE",
        21 => "TIME",
        22 => "DATETIME",
        23 => "ADATE",
        24 => "JDATE",
        25 => "DTIME",
        26 => "WKDAY",
        27 => "MONTH",
        28 => "MOYR",
        29 => "QYR",
        30 => "WKYR",
        31 => "PCT",
        32 => "DOT",
        33 => "CCA",
        34 => "CCB",
        35 => "CCC",
        36 => "CCD",
        37 => "CCE",
        38 => "EDATE",
        39 => "SDATE",
        40 => "MTIME",
        41 => "YMDHMS",
        _ => return None,
    };
    let width = var["format_width"].as_u64().unwrap_or_default();
    match var["format_decimals"].as_u64().unwrap_or_default() {
        0 => Some(format!("{name}{width}")),
        decimals => Some(format!("{name}{width}.{decimals}")),
    }
}

fn role(measure: Option<&str>) -> Option<&'static str> {
    match measure?.to_ascii_lowercase().as_str() {
        "scale" => Some("measure"),
        "nominal" | "ordinal" => Some("dimension"),
        _ => None,
    }
}

fn non_empty(value: &Value) -> Option<&str> {
    value.as_str().map(str::trim).filter(|s| !s.is_empty())
}

fn variables(meta: &Value) -> HashMap<&str, &Value> {
    meta.get("variables")
        .or_else(|| meta.get("columns"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|var| Some((var["name"].as_str()?, var)))
        .collect()
}

/// Whether `--sentinels-embedded` can have put sentinels in this column: a
/// numeric variable turned into text, or a text variable with declared SPSS
/// missing strings.
fn holds_sentinel_text(var: &Value, dtype: &DataType) -> bool {
    *dtype == DataType::String
        && (is_numeric_variable(var)
            || var["missing_strings"]
                .as_array()
                .is_some_and(|a| !a.is_empty()))
}

/// The columns whose distinct values the dictionary needs: labeled variables
/// (for `enum`) & sentinel columns (their `enum`).
pub(super) fn tracked(schema: &Schema, meta: &Value) -> Vec<String> {
    let vars = variables(meta);
    schema
        .iter_names()
        .filter(|name| match vars.get(name.as_str()) {
            Some(var) => var["value_labels"]
                .as_object()
                .is_some_and(|l| !l.is_empty()),
            None => name
                .strip_suffix("_null")
                .is_some_and(|base| vars.contains_key(base)),
        })
        .map(ToString::to_string)
        .collect()
}

/// The `embedded` columns that can hold sentinel text, each with what tells
/// its sentinels apart (for `null_values`).
pub(super) fn sentinel_matchers(
    schema: &Schema,
    meta: &Value,
    embedded: &[String],
    label_sentinels: bool,
) -> Vec<(String, SentinelMatcher)> {
    let vars = variables(meta);
    schema
        .iter()
        .filter(|(name, _)| embedded.iter().any(|e| e == name.as_str()))
        .filter_map(|(name, dtype)| {
            let var = vars.get(name.as_str())?;
            holds_sentinel_text(var, dtype)
                .then(|| (name.to_string(), SentinelMatcher::new(var, label_sentinels)))
        })
        .collect()
}

/// Build the dictionary of the CSV written with `schema`.
pub(super) fn build(schema: &Schema, meta: &Value, tallies: &Tallies, written: &Written) -> Value {
    let vars = variables(meta);

    let mut properties = Map::with_capacity(schema.len());
    let mut required = Vec::with_capacity(schema.len());
    for (name, dtype) in schema.iter() {
        let name = name.as_str();
        required.push(json!(name));
        let null_count = tallies.null_count(name);
        let (qsv_type, json_type) = types(dtype);
        let mut prop = Map::new();
        let mut x_qsv = Map::new();
        let mut type_array = vec![json!(json_type)];
        if null_count > 0 && json_type != "null" {
            type_array.push(json!("null"));
        }
        prop.insert("type".into(), Value::Array(type_array));
        x_qsv.insert("qsv_type".into(), json!(qsv_type));

        if let Some(var) = vars.get(name) {
            let label = non_empty(&var["label"]);
            if let Some(label) = label {
                prop.insert("title".into(), json!(label));
            }
            prop.insert(
                "description".into(),
                json!(
                    label.map_or_else(|| format!("{name} variable"), |l| format!("{l} ({name})"))
                ),
            );
            if *dtype == DataType::Date {
                prop.insert("format".into(), json!("date"));
            }

            let labels = value_labels(var);
            let held = tallies.distinct(name);
            if !labels.is_empty() {
                let decoded = written.value_labels && !dtype.is_primitive_numeric();
                // rounded floats may no longer equal their codes
                let rounded = dtype.is_float() && written.float_precision.is_some();
                if let Some(held) = held.filter(|_| !rounded)
                    && let Some(mut values) =
                        labeled_enum(&labels, missing_values(var).as_ref(), decoded, dtype, held)
                {
                    if null_count > 0 {
                        values.push(Value::Null);
                    }
                    prop.insert("enum".into(), Value::Array(values));
                }
                x_qsv.insert(
                    "value_labels".into(),
                    Value::Array(
                        labels
                            .iter()
                            .map(|(value, label)| json!({ "value": value, "label": label }))
                            .collect(),
                    ),
                );
            }
            if let Some(missing) = missing_values(var) {
                x_qsv.insert("missing_values".into(), missing);
            }
            if let Some(seen) = tallies.sentinels(name)
                && !seen.is_empty()
            {
                x_qsv.insert(
                    "null_values".into(),
                    Value::Array(seen.iter().map(|s| Value::String(s.clone())).collect()),
                );
            }
            if let Some(held) = held {
                x_qsv.insert("cardinality".into(), json!(held.len()));
            }
            if let Some(t) = non_empty(&var["type"]) {
                x_qsv.insert("storage_type".into(), json!(t));
            }
            let source_format = non_empty(&var["format"])
                .map(ToString::to_string)
                .or_else(|| spss_format(var));
            if let Some(f) = source_format {
                x_qsv.insert("source_format".into(), json!(f));
            }
            let measure = non_empty(&var["measure"]).filter(|m| !m.eq_ignore_ascii_case("unknown"));
            if let Some(m) = measure {
                x_qsv.insert("measure".into(), json!(m));
            }
            if let Some(r) = role(measure) {
                x_qsv.insert("role".into(), json!(r));
            }
            for key in ["display_width", "alignment"] {
                if let Some(v) = var.get(key).filter(|v| !v.is_null()) {
                    x_qsv.insert(key.into(), v.clone());
                }
            }
        } else if let Some(base) = name.strip_suffix("_null")
            && vars.contains_key(base)
        {
            // a sentinel column: the qsv readstat --sentinels-as companion of `base`
            prop.insert("title".into(), json!(format!("Sentinel of {base}")));
            prop.insert(
                "description".into(),
                json!(format!(
                    "The user-defined missing value (sentinel) of {base} in each row that has \
                     one{}; empty otherwise.",
                    if written.flags["sentinels_as"] == "label" {
                        ", as its label if it has one"
                    } else {
                        ""
                    }
                )),
            );
            if let Some(held) = tallies.distinct(name) {
                let mut values: Vec<Value> =
                    held.iter().map(|v| Value::String(v.clone())).collect();
                if null_count > 0 {
                    values.push(Value::Null);
                }
                prop.insert("enum".into(), Value::Array(values));
                x_qsv.insert("cardinality".into(), json!(held.len()));
            }
            x_qsv.insert("indicator_for".into(), json!(base));
        } else {
            prop.insert("description".into(), json!(format!("{name} column")));
        }
        x_qsv.insert("null_count".into(), json!(null_count));
        prop.insert("x-qsv".into(), Value::Object(x_qsv));
        properties.insert(name.to_string(), Value::Object(prop));
    }

    let file_label = ["file_label", "data_label"]
        .iter()
        .find_map(|key| non_empty(&meta[*key]));
    let mut top = Map::new();
    top.insert(
        "generated_by".into(),
        // worded like describegpt's, which viz smart --dict-info shows as the drawer's footer
        json!(format!(
            "Generated by qsv v{} readstat, from the metadata of {}",
            env!("CARGO_PKG_VERSION"),
            written.file_name
        )),
    );
    top.insert("source_file".into(), json!(written.file_name));
    top.insert("source_format".into(), json!(written.source));
    for (out, key) in [
        ("encoding", "encoding"),
        ("encoding", "file_encoding"),
        ("timestamp", "timestamp"),
        ("table_name", "table_name"),
    ] {
        if let Some(v) = non_empty(&meta[key]) {
            top.entry(out.to_string()).or_insert_with(|| json!(v));
        }
    }
    top.insert("row_count".into(), json!(tallies.rows));
    top.insert("readstat_flags".into(), written.flags.clone());

    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": file_label.map_or_else(
            || format!("Data Dictionary for {}", written.file_name),
            ToString::to_string,
        ),
        "description": format!(
            "JSON Schema (draft 2020-12) Data Dictionary of {}, built from its own metadata by \
             qsv readstat --dictionary.",
            written.file_name
        ),
        "type": "object",
        "properties": Value::Object(properties),
        "required": Value::Array(required),
        "additionalProperties": false,
        "x-qsv": Value::Object(top),
    })
}
