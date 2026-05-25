# profile_engine.py
#
# qsv-side Python entry point that the Rust ``py_engine`` module calls into.
# Builds a Jinja2 SandboxedEnvironment, registers every filter / global that
# DP+'s vendored ``jinja2_helpers`` exposes, then evaluates the formulas that
# the Rust side extracted from the scheming YAML spec.
#
# Contract with Rust:
#
#     evaluate(context_json: str, formulas_json: str) -> str
#
# where:
#   * context_json -- JSON string with the same keys our Rust ``context.rs``
#                     produces: ``package``, ``resource``, ``dpps``, ``dppf``, ``dpp``.
#   * formulas_json -- JSON array of {field_name, kind, template, scope}
#                      where:
#                          field_name  is the CKAN field identifier
#                          kind        is either "formula" or "suggestion_formula"
#                          template    is the raw Jinja2 source
#                          scope       is "dataset" or "resource"
#                      The Rust side already pre-filtered: every entry has a
#                      non-empty template.
#
# Return value: JSON string of the form
#     [
#       {"field_name": "...", "kind": "...", "scope": "...",
#        "value": "...", "error": null},
#       ...
#     ]
#
# Errors during render are *not* fatal -- they surface as ``error`` strings on
# the corresponding entry, so a failing formula in one field doesn't abort the
# whole profile pass.

from __future__ import annotations

import json
import traceback
from typing import Any

import qsv_ckan_stubs

qsv_ckan_stubs.install()

import jinja2_helpers  # noqa: E402  (must come after stubs.install())
from jinja2.sandbox import SandboxedEnvironment  # noqa: E402


def _build_env() -> SandboxedEnvironment:
    """Mirrors ``FormulaProcessor.__init__``'s env construction, minus CKAN
    specifics. Uses Jinja2's default ``Undefined`` (not ``StrictUndefined``)
    so a missing optional context key renders as the empty string rather
    than crashing the whole template -- matching DP+'s default Jinja2 env."""
    env = SandboxedEnvironment()
    for f in jinja2_helpers.JINJA2_FILTERS:
        env.filters[f.__name__] = f
    for g in jinja2_helpers.JINJA2_GLOBALS:
        env.globals[g.__name__] = g
    return env


def _normalize_suggestion(value: Any) -> Any:
    """Suggestion outputs are "soft" -- a render that produces empty/whitespace
    or the literal string ``"None"`` should surface as JSON ``null`` so
    downstream consumers can treat "no useful suggestion" uniformly. Hard
    ``formula`` results are left untouched: an explicit ``""`` may be the
    intended value.
    """
    if not isinstance(value, str):
        return value
    stripped = value.strip()
    if stripped == "" or stripped == "None":
        return None
    return value


def evaluate(context_json: str, formulas_json: str) -> str:
    try:
        context: dict[str, Any] = json.loads(context_json)
    except Exception as exc:  # pragma: no cover -- Rust serializes JSON
        return json.dumps({"_engine_error": f"context_json parse failed: {exc}"})

    try:
        formulas: list[dict[str, Any]] = json.loads(formulas_json)
    except Exception as exc:  # pragma: no cover
        return json.dumps({"_engine_error": f"formulas_json parse failed: {exc}"})

    env = _build_env()

    out: list[dict[str, Any]] = []
    for spec in formulas:
        field = spec.get("field_name") or "<unknown>"
        kind = spec.get("kind") or "formula"
        scope = spec.get("scope") or "dataset"
        template_src = spec.get("template") or ""
        entry: dict[str, Any] = {
            "field_name": field,
            "kind": kind,
            "scope": scope,
            "value": None,
            "error": None,
        }
        try:
            tmpl = env.from_string(template_src)
            rendered = tmpl.render(**context)
            if kind == "suggestion_formula":
                rendered = _normalize_suggestion(rendered)
            entry["value"] = rendered
        except Exception as exc:
            # Capture the short message plus a short formatted traceback
            # under a private `_traceback` field on the result entry so
            # `qsv profile`'s output JSON carries enough context to diagnose
            # which helper raised. Use `jq '.formula_results[]._traceback'`
            # on the resulting metadata file to inspect.
            entry["error"] = f"{type(exc).__name__}: {exc}"
            entry["_traceback"] = traceback.format_exc(limit=2)
        out.append(entry)

    return json.dumps(out)
