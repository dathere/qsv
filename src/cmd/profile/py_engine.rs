//! PyO3 bridge that evaluates the spec's `formula` / `suggestion_formula`
//! templates against the qsv-built analysis context, using DP+'s vendored
//! `jinja2_helpers.py` (see `py/` directory) for filter and global resolution.
//!
//! The Python side lives in three files that are baked into the binary via
//! `include_str!` and re-emitted into a per-invocation tempdir at runtime:
//!
//!   1. `qsv_ckan_stubs.py`  -- installs sys.modules stubs for the CKAN imports `jinja2_helpers`
//!      reaches for.
//!   2. `jinja2_helpers.py`  -- DP+'s vendored helpers (filters + globals).
//!   3. `profile_engine.py`  -- the `evaluate(context_json, formulas_json)` entry point we actually
//!      call.
//!
//! Why a tempdir rather than `PyModule::from_code`?
//! `jinja2_helpers` does `import ckanext.datapusher_plus.config as conf` at
//! *module-import time*, before our stubs would have a chance to register if
//! we tried loading it directly via `from_code`. By writing all three files
//! to a directory we control and prepending it to `sys.path`, the engine
//! script's `import qsv_ckan_stubs; qsv_ckan_stubs.install()` call runs
//! first, satisfying the CKAN imports `jinja2_helpers` performs.

use pyo3::{Python, prelude::*, types::PyModule};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tempfile::TempDir;

use super::spec::Spec;
use crate::{CliError, CliResult};

const STUBS_PY: &str = include_str!("py/qsv_ckan_stubs.py");
const HELPERS_PY: &str = include_str!("py/jinja2_helpers.py");
const ENGINE_PY: &str = include_str!("py/profile_engine.py");

/// One formula extracted from a scheming Field, ready to send to Python.
#[derive(Debug, Serialize)]
struct FormulaSpec<'a> {
    field_name: &'a str,
    /// `"formula"` or `"suggestion_formula"`.
    kind:       &'static str,
    /// `"dataset"` or `"resource"`.
    scope:      &'static str,
    template:   &'a str,
}

/// One result entry returned from `profile_engine.evaluate`.
#[derive(Debug, Deserialize, Serialize)]
pub struct FormulaResult {
    pub field_name: String,
    pub kind:       String,
    pub scope:      String,
    pub value:      Option<Value>,
    pub error:      Option<String>,
}

/// Evaluate every `formula` / `suggestion_formula` in `spec` against
/// `context`. Returns one `FormulaResult` per template encountered. If the
/// spec has no formulas, returns `Ok(vec![])` without spinning up Python.
pub fn evaluate_spec(spec: &Spec, context: &Value) -> CliResult<Vec<FormulaResult>> {
    // ---- 1. flatten spec into a Vec<FormulaSpec> -------------------------
    let mut formulas: Vec<FormulaSpec> = Vec::new();
    for f in spec.real_dataset_fields() {
        push_if_some(&mut formulas, f, "dataset");
    }
    for f in spec.real_resource_fields() {
        push_if_some(&mut formulas, f, "resource");
    }
    if formulas.is_empty() {
        return Ok(Vec::new());
    }

    let context_json = serde_json::to_string(context).map_err(|e| {
        CliError::Other(format!(
            "could not serialize analysis context for Python: {e}"
        ))
    })?;
    let formulas_json = serde_json::to_string(&formulas).map_err(|e| {
        CliError::Other(format!("could not serialize formulas list for Python: {e}"))
    })?;

    // ---- 2. lay out the three .py files in a tempdir --------------------
    let tmp =
        TempDir::new().map_err(|e| CliError::Other(format!("could not create py tempdir: {e}")))?;
    std::fs::write(tmp.path().join("qsv_ckan_stubs.py"), STUBS_PY)
        .map_err(|e| CliError::Other(format!("could not stage qsv_ckan_stubs.py: {e}")))?;
    std::fs::write(tmp.path().join("jinja2_helpers.py"), HELPERS_PY)
        .map_err(|e| CliError::Other(format!("could not stage jinja2_helpers.py: {e}")))?;
    std::fs::write(tmp.path().join("profile_engine.py"), ENGINE_PY)
        .map_err(|e| CliError::Other(format!("could not stage profile_engine.py: {e}")))?;
    let tmp_path = tmp
        .path()
        .to_str()
        .ok_or_else(|| CliError::Other("py tempdir path is not valid UTF-8".to_string()))?
        .to_string();

    // ---- 3. fire up Python, import engine, call evaluate ----------------
    let result_json = Python::attach(|py| -> PyResult<String> {
        let sys = PyModule::import(py, "sys")?;
        let path = sys.getattr("path")?;
        path.call_method1("insert", (0i64, tmp_path.as_str()))?;

        let engine = PyModule::import(py, "profile_engine")?;
        let result_obj =
            engine.call_method1("evaluate", (context_json.as_str(), formulas_json.as_str()))?;
        let s: String = result_obj.extract()?;
        Ok(s)
    })
    .map_err(|e| {
        CliError::Other(format!(
            "qsv profile: Python engine raised an exception -- check that `python3` is available \
             and that the `jinja2` package is installed (`pip install jinja2`). Detail: {e}"
        ))
    })?;

    // Ensure the tempdir survives until after the Python call completed.
    drop(tmp);

    // ---- 4. parse Python's JSON return value ----------------------------
    // The engine returns a JSON array on success; on a (rare) internal
    // error it returns an object `{"_engine_error": "..."}`.
    let parsed: Value = serde_json::from_str(&result_json).map_err(|e| {
        CliError::Other(format!(
            "qsv profile: could not parse Python engine result as JSON: {e}\n  raw: {result_json}"
        ))
    })?;
    if let Some(err) = parsed.get("_engine_error").and_then(|v| v.as_str()) {
        return Err(CliError::Other(format!(
            "qsv profile: Python engine reported: {err}"
        )));
    }
    let results: Vec<FormulaResult> = serde_json::from_value(parsed).map_err(|e| {
        CliError::Other(format!(
            "qsv profile: could not deserialize formula results: {e}"
        ))
    })?;
    Ok(results)
}

fn push_if_some<'a>(
    out: &mut Vec<FormulaSpec<'a>>,
    field: &'a super::spec::Field,
    scope: &'static str,
) {
    let Some(name) = field.field_name.as_deref() else {
        return;
    };
    if let Some(t) = field.formula.as_deref() {
        if !t.trim().is_empty() {
            out.push(FormulaSpec {
                field_name: name,
                kind: "formula",
                scope,
                template: t,
            });
        }
    }
    if let Some(t) = field.suggestion_formula.as_deref() {
        if !t.trim().is_empty() {
            out.push(FormulaSpec {
                field_name: name,
                kind: "suggestion_formula",
                scope,
                template: t,
            });
        }
    }
}
