use std::{collections::HashMap, fmt::Write, path::Path};

use chrono::Utc;
use serde_json::Value;
use uuid::Uuid;

use crate::{CliError, CliResult};

const DDI_NS: &str = "http://www.icpsr.umich.edu/DDI";
const DDI_SCHEMA_LOCATION: &str =
	"http://www.icpsr.umich.edu/DDI http://www.icpsr.umich.edu/DDI/Version2-6/XMLSchema/codebook.xsd";
// Keep this aligned with profile/context SchemaArgs::flag_enum_threshold.
const PROFILE_SCHEMA_ENUM_THRESHOLD: u64 = 50;

pub const DEFAULT_DDIC_CATGRY_LIMIT: usize = 25;

#[derive(Debug, Clone)]
pub struct CatgryLimitPolicy {
	default_cap: usize,
	overrides:   HashMap<String, usize>,
}

impl Default for CatgryLimitPolicy {
	fn default() -> Self {
		Self {
			default_cap: DEFAULT_DDIC_CATGRY_LIMIT,
			overrides:   HashMap::new(),
		}
	}
}

impl CatgryLimitPolicy {
	pub fn max_requested_cap(&self) -> usize {
		self.overrides
			.values()
			.copied()
			.max()
			.unwrap_or(self.default_cap)
			.max(self.default_cap)
	}

	pub fn cap_for(&self, field: &str) -> usize {
		self.overrides
			.get(field)
			.copied()
			.unwrap_or(self.default_cap)
	}
}

pub fn parse_limit_policy(raw: Option<&str>) -> CliResult<CatgryLimitPolicy> {
	let Some(raw) = raw else {
		return Ok(CatgryLimitPolicy::default());
	};

	let trimmed = raw.trim();
	if trimmed.is_empty() {
		return Err(CliError::Other(
			"--ddi-catgry-limit cannot be empty".to_string(),
		));
	}

	// parse order: numeric literal -> inline JSON object -> JSON file path
	if let Ok(n) = trimmed.parse::<usize>() {
		return Ok(CatgryLimitPolicy {
			default_cap: n,
			overrides:   HashMap::new(),
		});
	}

	if let Ok(overrides) = parse_override_json_object(trimmed) {
		return Ok(CatgryLimitPolicy {
			default_cap: DEFAULT_DDIC_CATGRY_LIMIT,
			overrides,
		});
	}

	let raw_file = std::fs::read_to_string(trimmed).map_err(|e| {
		CliError::Other(format!(
			"--ddi-catgry-limit value is neither numeric nor a valid JSON object and `{trimmed}` \
			 could not be read as a JSON file: {e}"
		))
	})?;
	let overrides = parse_override_json_object(&raw_file).map_err(|e| {
		CliError::Other(format!(
			"--ddi-catgry-limit file `{trimmed}` does not contain a valid JSON override object: \
			 {e}"
		))
	})?;

	Ok(CatgryLimitPolicy {
		default_cap: DEFAULT_DDIC_CATGRY_LIMIT,
		overrides,
	})
}

pub fn build(
	input_path: &str,
	headers: &[String],
	dpp: &Value,
	stats: &Value,
	frequency: &Value,
	policy: &CatgryLimitPolicy,
) -> CliResult<String> {
	let codebook_id = format!("_{}", Uuid::new_v4());
	let filename = Path::new(input_path)
		.file_name()
		.and_then(|s| s.to_str())
		.unwrap_or(input_path)
		.to_string();

	let row_count = dpp
		.pointer("/dataset_stats/row_count")
		.and_then(Value::as_u64)
		.unwrap_or(0);

	let mut out = String::with_capacity(16_384);
	write!(
		out,
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<codeBook version=\"2.6\" xml:lang=\"en-US\" ID=\"{}\" xmlns=\"{}\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"{}\">\n",
		escape_xml_attr(&codebook_id),
		escape_xml_attr(DDI_NS),
		escape_xml_attr(DDI_SCHEMA_LOCATION)
	)
	.map_err(|e| CliError::Other(format!("could not build DDI XML: {e}")))?;

	let today = Utc::now().format("%Y-%m-%d").to_string();

	write!(
		out,
		"  <docDscr>\n    <citation>\n      <titlStmt>\n        <titl>{}</titl>\n      </titlStmt>\n      <prodStmt>\n        <prodDate date=\"{}\">{}</prodDate>\n        <software version=\"{}\">qsv</software>\n      </prodStmt>\n    </citation>\n  </docDscr>\n",
		escape_xml_text(&format!("qsv profile DDI-Codebook: {filename}")),
		escape_xml_attr(&today),
		escape_xml_text(&today),
		escape_xml_attr(env!("CARGO_PKG_VERSION"))
	)
	.map_err(|e| CliError::Other(format!("could not build DDI XML: {e}")))?;

	// Required study title, sourced from the input filename.
	write!(
		out,
		"  <stdyDscr>\n    <citation>\n      <titlStmt>\n        <titl>{}</titl>\n      \
		 </titlStmt>\n    </citation>\n  </stdyDscr>\n",
		escape_xml_text(&filename)
	)
	.map_err(|e| CliError::Other(format!("could not build DDI XML: {e}")))?;

	write!(
		out,
		"  <fileDscr ID=\"F1\">\n    <fileTxt>\n      <fileName>{}</fileName>\n      <dimensns>\n        <caseQnty>{}</caseQnty>\n        <varQnty>{}</varQnty>\n      </dimensns>\n      <fileType>CSV</fileType>\n    </fileTxt>\n  </fileDscr>\n",
		escape_xml_text(&filename),
		row_count,
		headers.len()
	)
	.map_err(|e| CliError::Other(format!("could not build DDI XML: {e}")))?;

	out.push_str("  <dataDscr>\n");
	for (idx, field) in headers.iter().enumerate() {
		let var_id = format!("V{}", idx + 1);
		let stats_obj = stats_for_field(stats, field);
		let var_type = infer_var_type(stats_obj);
		let has_categories = has_eligible_categories(field, frequency, policy);
		let is_categorical = infer_is_categorical(stats_obj, row_count, has_categories);
		let intrvl = infer_intrvl(var_type, is_categorical);
		let representation_type = infer_representation_type(var_type, is_categorical);
		let dcml_attr = max_precision_attr(stats_obj);
		write!(
			out,
			"    <var ID=\"{}\" name=\"{}\" files=\"F1\" intrvl=\"{}\" representationType=\"{}\"{}>\n      <labl>{}</labl>\n",
			escape_xml_attr(&var_id),
			escape_xml_attr(field),
			escape_xml_attr(intrvl),
			escape_xml_attr(representation_type),
			dcml_attr,
			escape_xml_text(field)
		)
		.map_err(|e| CliError::Other(format!("could not build DDI XML: {e}")))?;

		emit_sumstats(&mut out, stats_obj, row_count)
			.map_err(|e| CliError::Other(format!("could not build DDI XML: {e}")))?;

		emit_categories(&mut out, field, frequency, policy, is_categorical)
			.map_err(|e| CliError::Other(format!("could not build DDI XML: {e}")))?;

		write!(
			out,
			"      <varFormat type=\"{}\" schema=\"other\" otherSchema=\"qsv\" formatname=\"{}\"/>\n",
			escape_xml_attr(var_type),
			escape_xml_attr(infer_source_type(stats_obj).unwrap_or("Unknown"))
		)
		.map_err(|e| CliError::Other(format!("could not build DDI XML: {e}")))?;

		out.push_str("    </var>\n");
	}
	out.push_str("  </dataDscr>\n</codeBook>\n");

	Ok(out)
}

fn parse_override_json_object(raw: &str) -> Result<HashMap<String, usize>, String> {
	let parsed: Value = serde_json::from_str(raw).map_err(|e| e.to_string())?;
	let obj = parsed
		.as_object()
		.ok_or_else(|| "expected a JSON object".to_string())?;

	let mut overrides = HashMap::with_capacity(obj.len());
	for (k, v) in obj {
		let Some(num) = v.as_u64() else {
			return Err(format!("override value for `{k}` must be an unsigned integer"));
		};
		let cap = usize::try_from(num)
			.map_err(|_| format!("override value for `{k}` is too large for this platform"))?;
		overrides.insert(k.to_string(), cap);
	}
	Ok(overrides)
}

fn stats_for_field<'a>(stats: &'a Value, field: &str) -> Option<&'a Value> {
	stats
		.as_object()
		.and_then(|m| m.get(field))
		.and_then(|v| v.get("stats"))
}

fn infer_var_type(stats_obj: Option<&Value>) -> &'static str {
	match infer_source_type(stats_obj) {
		Some("Integer") | Some("Float") => "numeric",
		Some("Date") => "date",
		Some("DateTime") => "datetime",
		_ => "character",
	}
}

fn infer_source_type(stats_obj: Option<&Value>) -> Option<&str> {
	stats_obj
		.and_then(|v| v.get("type"))
		.and_then(Value::as_str)
}

fn infer_intrvl(var_type: &str, is_categorical: bool) -> &'static str {
	if is_categorical {
		return "discrete";
	}

	match var_type {
		"numeric" | "date" | "datetime" => "contin",
		_ => "discrete",
	}
}

fn infer_representation_type(var_type: &str, is_categorical: bool) -> &'static str {
	if is_categorical {
		"coded"
	} else if var_type == "numeric" {
		"numeric"
	} else if var_type == "date" || var_type == "datetime" {
		"datetime"
	} else {
		"text"
	}
}

fn infer_is_categorical(stats_obj: Option<&Value>, row_count: u64, has_categories: bool) -> bool {
	let Some(stats_obj) = stats_obj else {
		return has_categories;
	};

	let source_type = infer_source_type(Some(stats_obj)).unwrap_or("Unknown");
	if has_schema_enum_or_const(stats_obj, source_type) {
		return true;
	}

	if source_type == "Date" || source_type == "DateTime" {
		return false;
	}

	let cardinality = stats_obj
		.get("cardinality")
		.and_then(Value::as_u64)
		.unwrap_or(0);
	let uniqueness_ratio = stats_obj
		.get("uniqueness_ratio")
		.and_then(Value::as_f64)
		.unwrap_or(1.0);
	let nullcount = stats_obj
		.get("nullcount")
		.and_then(Value::as_u64)
		.unwrap_or(0);
	let non_null_count = row_count.saturating_sub(nullcount);

	if non_null_count == 0 {
		return false;
	}

	match source_type {
		"String" | "Integer" => false,
		"Float" => {
			cardinality <= 12 && uniqueness_ratio <= 0.20
		}
		_ => has_categories && cardinality <= 30,
	}
}

fn has_schema_enum_or_const(stats_obj: &Value, source_type: &str) -> bool {
	if source_type != "String" && source_type != "Integer" {
		return false;
	}

	let cardinality = stats_obj
		.get("cardinality")
		.and_then(Value::as_u64)
		.unwrap_or(u64::MAX);

	cardinality <= PROFILE_SCHEMA_ENUM_THRESHOLD
}

fn max_precision_attr(stats_obj: Option<&Value>) -> String {
	let Some(dcml) = stats_obj
		.and_then(|v| v.get("max_precision"))
		.and_then(Value::as_u64)
	else {
		return String::new();
	};

	format!(" dcml=\"{}\"", dcml)
}

fn has_eligible_categories(field: &str, frequency: &Value, policy: &CatgryLimitPolicy) -> bool {
	let cap = policy.cap_for(field);
	if cap == 0 {
		return false;
	}

	let Some(cats) = frequency.as_object().and_then(|m| m.get(field)).and_then(Value::as_array)
	else {
		return false;
	};

	!cats.is_empty() && cats.len() <= cap
}

fn emit_sumstats(
	out: &mut String,
	stats_obj: Option<&Value>,
	row_count: u64,
) -> Result<(), std::fmt::Error> {
	let Some(stats_obj) = stats_obj else {
		return Ok(());
	};

	emit_sumstat_f64(out, "mean", stats_obj.get("mean"));
	emit_sumstat_f64(out, "stdev", stats_obj.get("stddev"));
	emit_sumstat_text(out, "min", stats_obj.get("min"));
	emit_sumstat_text(out, "max", stats_obj.get("max"));

	let nullcount = stats_obj
		.get("nullcount")
		.and_then(Value::as_u64)
		.unwrap_or(0);
	write!(
		out,
		"      <sumStat type=\"invd\">{}</sumStat>\n      <sumStat type=\"vald\">{}</sumStat>\n",
		nullcount,
		row_count.saturating_sub(nullcount)
	)?;

	emit_other_sumstat_f64(out, "sum", stats_obj.get("sum"));
	emit_other_sumstat_f64(out, "range", stats_obj.get("range"));
	emit_other_sumstat_f64(out, "variance", stats_obj.get("variance"));
	emit_other_sumstat_f64(out, "cv", stats_obj.get("cv"));
	emit_other_sumstat_f64(out, "sparsity", stats_obj.get("sparsity"));

	if let Some(cardinality) = stats_obj.get("cardinality").and_then(Value::as_u64) {
		write!(
			out,
			"      <sumStat type=\"other\" otherType=\"cardinality\">{}</sumStat>\n",
			cardinality
		)?;
	}

	Ok(())
}

fn emit_sumstat_f64(out: &mut String, kind: &str, v: Option<&Value>) {
	if let Some(n) = v.and_then(Value::as_f64) {
		let _ = writeln!(
			out,
			"      <sumStat type=\"{}\">{}</sumStat>",
			escape_xml_attr(kind),
			n
		);
	}
}

fn emit_sumstat_text(out: &mut String, kind: &str, v: Option<&Value>) {
	if let Some(s) = v.and_then(Value::as_str) {
		let _ = writeln!(
			out,
			"      <sumStat type=\"{}\">{}</sumStat>",
			escape_xml_attr(kind),
			escape_xml_text(s)
		);
	}
}

fn emit_other_sumstat_f64(out: &mut String, other_type: &str, v: Option<&Value>) {
	if let Some(n) = v.and_then(Value::as_f64) {
		let _ = writeln!(
			out,
			"      <sumStat type=\"other\" otherType=\"{}\">{}</sumStat>",
			escape_xml_attr(other_type),
			n
		);
	}
}

fn emit_categories(
	out: &mut String,
	field: &str,
	frequency: &Value,
	policy: &CatgryLimitPolicy,
	is_categorical: bool,
) -> Result<(), std::fmt::Error> {
	if !is_categorical {
		return Ok(());
	}

	let cap = policy.cap_for(field);
	if cap == 0 {
		return Ok(());
	}

	let Some(cats) = frequency.as_object().and_then(|m| m.get(field)).and_then(Value::as_array)
	else {
		return Ok(());
	};

	// Conservative rule: if categories exceed cap, omit categories entirely.
	if cats.len() > cap {
		return Ok(());
	}

	for c in cats {
		let value = c.get("value").map(json_value_to_string).unwrap_or_default();
		let count = c.get("count").and_then(Value::as_u64).unwrap_or(0);
		let percentage = c.get("percentage").and_then(Value::as_f64).unwrap_or(0.0);
		write!(
			out,
			"      <catgry>\n        <catValu>{}</catValu>\n        <labl>{}</labl>\n        <catStat type=\"freq\">{}</catStat>\n        <catStat type=\"percent\">{}</catStat>\n      </catgry>\n",
			escape_xml_text(&value),
			escape_xml_text(&value),
			count,
			percentage
		)?;
	}
	Ok(())
}

fn json_value_to_string(v: &Value) -> String {
	match v {
		Value::Null => String::new(),
		Value::Bool(b) => b.to_string(),
		Value::Number(n) => n.to_string(),
		Value::String(s) => s.to_string(),
		Value::Array(_) | Value::Object(_) => v.to_string(),
	}
}

fn escape_xml_text(s: &str) -> String {
	s.replace('&', "&amp;")
		.replace('<', "&lt;")
		.replace('>', "&gt;")
}

fn escape_xml_attr(s: &str) -> String {
	escape_xml_text(s)
		.replace('"', "&quot;")
		.replace('\'', "&apos;")
}
