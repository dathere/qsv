#!/usr/bin/env python3
"""
check_semanticmd.py — regenerate-and-verify check for qsv describegpt's
`--format semanticmd` output, modeled on `qsv --generate-help-md`.

It converts a semantic-md data-dictionary markdown document to JSON using the
`semantic-md` package and the co-located `datadict.yaml` schema, validates the
result, asserts a few structural invariants, then compares it against the
committed `.json` artifact.

Usage:
    # verify the committed JSON is up to date (CI / pre-commit); non-zero on drift
    python3 check_semanticmd.py

    # regenerate and overwrite the committed JSON (like `--generate-help-md`)
    python3 check_semanticmd.py --update

    # check a different document
    python3 check_semanticmd.py --md path/to/doc.md --schema path/to/datadict.yaml

The `semantic-md` toolchain is bootstrapped into a local virtualenv
(`.semanticmd-venv/` next to this script) on first run so the check is hermetic
and does not touch the system Python.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import venv
from pathlib import Path

HERE = Path(__file__).resolve().parent
VENV_DIR = HERE / ".semanticmd-venv"

# Pinned so the check is reproducible. `semantic-md` is pre-1.0 and its parsing
# behavior can shift between releases.
REQUIREMENTS = [
    "semantic-md==0.0.2",
    "click",
    "pyyaml",
    "mistletoe",
    "jsonpatch",
]

DEFAULT_MD = HERE / "nyc311-describegpt-semanticmd.md"
DEFAULT_SCHEMA = HERE / "datadict.yaml"


def venv_python() -> Path:
    bindir = "Scripts" if os.name == "nt" else "bin"
    return VENV_DIR / bindir / ("python.exe" if os.name == "nt" else "python")


def ensure_venv() -> Path:
    """Create the venv and install pinned deps on first run; reuse afterwards."""
    py = venv_python()
    marker = VENV_DIR / ".requirements.ok"
    if py.exists() and marker.exists():
        return py
    print(f"[check_semanticmd] bootstrapping venv at {VENV_DIR} ...", file=sys.stderr)
    venv.create(VENV_DIR, with_pip=True)
    subprocess.run(
        [str(py), "-m", "pip", "install", "--quiet", "--upgrade", "pip"], check=True
    )
    subprocess.run(
        [str(py), "-m", "pip", "install", "--quiet", *REQUIREMENTS], check=True
    )
    marker.write_text("\n".join(REQUIREMENTS) + "\n")
    return py


def convert(py: Path, md: Path, schema: Path) -> dict:
    """Run the real semantic-md converter and return the parsed JSON."""
    cmd = [
        str(py),
        "-c",
        "from semantic_md.cli import cli; cli()",
        "json",
        str(md),
        "-",  # write JSON to stdout
        "-s",
        str(schema),
    ]
    proc = subprocess.run(cmd, capture_output=True, text=True)
    if proc.returncode != 0:
        sys.stderr.write(proc.stderr)
        raise SystemExit(
            f"[check_semanticmd] semantic-md conversion FAILED for {md.name}"
        )
    return json.loads(proc.stdout)


def assert_invariants(doc: dict) -> list[str]:
    """Structural sanity checks. Returns a list of human-readable failures."""
    errs: list[str] = []

    def need(cond: bool, msg: str) -> None:
        if not cond:
            errs.append(msg)

    need("dataset" in doc, "missing top-level `dataset`")
    need("schema" in doc, "missing top-level `schema`")
    need("resource" in doc, "missing top-level `resource`")
    if errs:
        return errs

    dataset, schema, resource = doc["dataset"], doc["schema"], doc["resource"]

    need(bool(dataset.get("name")), "dataset.name is empty")
    need(bool(dataset.get("overview")), "dataset.overview is empty")

    fields = schema.get("fields", [])
    columns = schema.get("columns", [])
    stats = resource.get("statistics", [])
    freqs = resource.get("frequencies", [])

    need(len(fields) > 0, "schema.fields is empty")
    # Every column detailed in the body should appear in the schema overview
    # table and the resource statistics table.
    need(
        len(fields) == len(columns) == len(stats),
        f"row-count mismatch: schema.fields={len(fields)}, "
        f"schema.columns={len(columns)}, resource.statistics={len(stats)}",
    )
    need(len(freqs) > 0, "resource.frequencies is empty")

    # No value should leak literal markdown backticks (the inline-code wrapper qsv
    # uses for identifiers). Markdown blob fields are exempt.
    blob_keys = {"info", "overview", "validation", "trailer", "text"}

    def scan(node, path="$"):
        if isinstance(node, dict):
            for k, v in node.items():
                scan(v, f"{path}.{k}")
        elif isinstance(node, list):
            for i, v in enumerate(node):
                scan(v, f"{path}[{i}]")
        elif isinstance(node, str) and "`" in node:
            if path.rsplit(".", 1)[-1] not in blob_keys:
                errs.append(f"backtick leaked into value at {path}: {node!r}")

    scan(doc)

    # Spot-check that identifiers were extracted without backticks.
    if fields:
        need(
            "`" not in fields[0].get("column", ""),
            "schema.fields[].column still has backticks",
        )
    return errs


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--md", type=Path, default=DEFAULT_MD)
    ap.add_argument("--schema", type=Path, default=DEFAULT_SCHEMA)
    ap.add_argument(
        "--json",
        type=Path,
        default=None,
        help="committed JSON artifact (default: <md> with .json suffix)",
    )
    ap.add_argument(
        "--update",
        action="store_true",
        help="overwrite the committed JSON instead of diffing against it",
    )
    args = ap.parse_args()

    md = args.md.resolve()
    schema = args.schema.resolve()
    out_json = (args.json or md.with_suffix(".json")).resolve()

    if not md.exists():
        raise SystemExit(f"[check_semanticmd] markdown not found: {md}")
    if not schema.exists():
        raise SystemExit(f"[check_semanticmd] schema not found: {schema}")

    py = ensure_venv()
    doc = convert(py, md, schema)

    errs = assert_invariants(doc)
    if errs:
        print("[check_semanticmd] FAILED structural invariants:", file=sys.stderr)
        for e in errs:
            print(f"  - {e}", file=sys.stderr)
        return 1

    rendered = json.dumps(doc, indent=2, ensure_ascii=False) + "\n"

    if args.update:
        out_json.write_text(rendered, encoding="utf-8")
        print(f"[check_semanticmd] wrote {out_json} ({len(rendered)} bytes)")
        return 0

    if not out_json.exists():
        print(
            f"[check_semanticmd] committed JSON missing: {out_json}\n"
            f"  run: python3 {Path(__file__).name} --update",
            file=sys.stderr,
        )
        return 1

    committed = out_json.read_text(encoding="utf-8")
    if committed != rendered:
        print(
            f"[check_semanticmd] DRIFT: {out_json.name} is out of date.\n"
            f"  regenerate with: python3 {Path(__file__).name} --update",
            file=sys.stderr,
        )
        return 1

    print(
        f"[check_semanticmd] OK — {md.name} converts cleanly and "
        f"{out_json.name} is up to date "
        f"({len(doc['schema']['fields'])} fields, "
        f"{len(doc['resource']['frequencies'])} frequency tables)."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
