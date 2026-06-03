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

The conversion runs in-process against the installed `semantic-md` package
(no subprocess, no shell). Install the pinned toolchain once with:

    pip install semantic-md==0.0.2 mistletoe==1.5.1 jsonpatch==1.33 pyyaml==6.0.3
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent

# Fully pinned for reproducibility: `semantic-md` is pre-1.0 and its parsing
# behavior (and that of its parser deps) can shift between releases, which would
# silently change the generated JSON. If you bump these, regenerate with --update.
# The check itself is the backstop — it diffs against the committed artifact, so a
# behavior change in any dependency fails loudly rather than drifting unnoticed.
REQUIREMENTS = "semantic-md==0.0.2 mistletoe==1.5.1 jsonpatch==1.33 pyyaml==6.0.3"

DEFAULT_MD = HERE / "nyc311-describegpt-semanticmd.md"
DEFAULT_SCHEMA = HERE / "datadict.yaml"


def convert(md: Path, schema: Path) -> dict:
    """Convert `md` to a JSON object using `schema`, in-process.

    Mirrors the steps semantic-md's own CLI performs, but without shelling out.
    """
    try:
        from semantic_md import convert as smd
    except ImportError as exc:  # pragma: no cover - environment guard
        raise SystemExit(
            f"[check_semanticmd] cannot import `semantic_md` ({exc}).\n"
            f"  install the toolchain with: pip install {REQUIREMENTS}"
        )

    front, body = smd.md_parse_front_matter(md.read_text(encoding="utf-8"))

    # Honor the schema named in the document front matter when --schema is left
    # at its default, resolving it relative to the document's directory.
    declared = front.get("semantic-md")
    if schema == DEFAULT_SCHEMA.resolve() and declared:
        schema = (md.parent / declared).resolve()

    s = smd.Schema.read(schema.read_text(encoding="utf-8"))
    parsed = smd.md_parse_body(body, s)
    return smd.to_json(parsed, s)


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
    # Frequency tables are per-column and optional (a column only gets one when it
    # has a frequency distribution), so a valid document may legitimately have none.
    # When present, each entry must carry its source column name.
    need(isinstance(freqs, list), "resource.frequencies is not a list")
    need(
        all(isinstance(f, dict) and f.get("column") for f in freqs),
        "a resource.frequencies entry is missing its `column`",
    )

    # No value should leak literal markdown backticks (the inline-code wrapper qsv
    # uses for identifiers). Markdown blob fields are exempt.
    blob_keys = {"info", "overview", "validation", "trailer", "grain"}

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

    doc = convert(md, schema)

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
