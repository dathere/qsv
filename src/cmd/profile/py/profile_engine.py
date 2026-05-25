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
from jinja2 import StrictUndefined  # noqa: E402
from jinja2.sandbox import SandboxedEnvironment  # noqa: E402


def _build_env() -> SandboxedEnvironment:
    """Mirrors ``FormulaProcessor.__init__``'s env construction, minus CKAN
    specifics."""
    env = SandboxedEnvironment(
        # Keep undefined values surfacing as empty strings, which is what
        # DP+'s default Jinja2 environment does (DP+ doesn't pass
        # StrictUndefined). That matches the user expectation that a missing
        # context key renders as "" rather than crashing the whole template.
        undefined=type("_LooseUndefined", (StrictUndefined,), {}).__bases__[0],
    )
    for f in jinja2_helpers.JINJA2_FILTERS:
        env.filters[f.__name__] = f
    for g in jinja2_helpers.JINJA2_GLOBALS:
        env.globals[g.__name__] = g
    return env


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
            entry["value"] = tmpl.render(**context)
        except Exception as exc:
            # Capture the short message + a one-line traceback hint so the
            # user can pinpoint which helper raised. The full traceback is
            # available via RUST_LOG=debug.
            entry["error"] = f"{type(exc).__name__}: {exc}"
            entry["_traceback"] = traceback.format_exc(limit=2)
        out.append(entry)

    return json.dumps(out)
