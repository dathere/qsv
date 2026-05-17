#!/usr/bin/env python3
"""
docs-drift-check.py — Detect drift between documentation and source of truth.

This script re-derives the following facts from authoritative sources and
flags any documentation file that contradicts them:

  1. Cargo feature-set membership for the meta-features `distrib_features`,
     `all_features`, and `qsvmcp` (source: Cargo.toml).
  2. Source-file line counts referenced in contributor docs as
     "~NNN lines" (source: wc -l on the named .rs file).
  3. The MAX_STAT_COLUMNS constant referenced as "up to N (summary) statistics"
     (source: src/cmd/stats.rs).
  4. The 🤯 "loads entire CSV into memory" command set duplicated between
     docs/PERFORMANCE.md (prose) and docs/PERFORMANCE_TLDR.md (bullets);
     both docs must list the same commands.
  5. The qsv crate version referenced in dotenv.template's QSV_USER_AGENT
     example (source: Cargo.toml package.version).

Exit codes:
  0 — no drift detected
  1 — drift detected (one or more findings)
  2 — script error (missing files, parse failure, etc.)

The line-count check uses a percent-tolerance because contributor docs are
intentionally approximate. The default is 10% — adjust via --line-tolerance.

Usage:
  python3 scripts/docs-drift-check.py                # run with defaults
  python3 scripts/docs-drift-check.py --line-tolerance 15
  python3 scripts/docs-drift-check.py --json         # machine-readable output
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

try:
    import tomllib  # Python 3.11+
except ModuleNotFoundError:  # pragma: no cover
    print("docs-drift-check requires Python 3.11+ (tomllib).", file=sys.stderr)
    sys.exit(2)


REPO_ROOT = Path(__file__).resolve().parent.parent


# ---------------------------------------------------------------------------
# Findings model
# ---------------------------------------------------------------------------


@dataclass
class Finding:
    file: str
    line: int | None
    category: str
    message: str

    def format(self) -> str:
        loc = f"{self.file}:{self.line}" if self.line else self.file
        return f"  [{self.category}] {loc} — {self.message}"


@dataclass
class Report:
    findings: list[Finding] = field(default_factory=list)

    def add(self, **kwargs: object) -> None:
        self.findings.append(Finding(**kwargs))  # type: ignore[arg-type]

    def __bool__(self) -> bool:
        return bool(self.findings)


# ---------------------------------------------------------------------------
# Source-of-truth extraction
# ---------------------------------------------------------------------------


def load_cargo_toml() -> dict:
    with open(REPO_ROOT / "Cargo.toml", "rb") as fh:
        return tomllib.load(fh)


def get_qsv_version(cargo: dict) -> str:
    return cargo["package"]["version"]


def get_features(cargo: dict) -> dict[str, set[str]]:
    """Return raw feature lists for distrib_features, all_features, qsvmcp.

    Values are the *direct* feature dependencies as declared in
    `[features]`, not the transitive closure.
    """
    features = cargo["features"]
    keys = ("distrib_features", "all_features", "qsvmcp")
    out: dict[str, set[str]] = {}
    for k in keys:
        if k not in features:
            raise KeyError(f"Cargo.toml [features] is missing {k!r}")
        # Strip any feature-of-dep references (entries containing a slash,
        # like "crc32fast/nightly"). The meta-features audited here
        # (distrib_features, all_features, qsvmcp) only enumerate other
        # named features and don't currently use "dep:"-prefixed entries —
        # those appear only in leaf feature definitions (e.g.
        # `synthesize = ["dep:fake", "dep:time"]`) which we don't read.
        out[k] = {f for f in features[k] if "/" not in f}
    return out


def expand_all_features_for_docs(features: dict[str, set[str]]) -> set[str]:
    """`all_features` references `distrib_features` directly; expand it.

    Docs typically enumerate the *user-facing* features included in
    `all_features` (e.g., "apply, fetch, foreach, ...") rather than
    saying "distrib_features + magika + self_update + ui". So for the
    docs-comparison check we expand one level.
    """
    raw = features["all_features"]
    expanded = set(raw)
    if "distrib_features" in expanded:
        expanded.remove("distrib_features")
        expanded |= features["distrib_features"]
    return expanded


MAX_STAT_COLUMNS_RE = re.compile(
    r"^\s*(?:pub\s+)?const\s+MAX_STAT_COLUMNS\s*:\s*\w+\s*=\s*(\d+)\s*;",
    re.MULTILINE,
)


def get_max_stat_columns() -> int:
    stats = (REPO_ROOT / "src/cmd/stats.rs").read_text(encoding="utf-8")
    m = MAX_STAT_COLUMNS_RE.search(stats)
    if not m:
        raise RuntimeError(
            "Could not find MAX_STAT_COLUMNS in src/cmd/stats.rs — "
            "update docs-drift-check.py to match the new declaration.",
        )
    return int(m.group(1))


def count_lines(path: Path) -> int:
    # Match `wc -l` semantics: count newline characters.
    return path.read_bytes().count(b"\n")


# ---------------------------------------------------------------------------
# Doc-side checks
# ---------------------------------------------------------------------------


# Files where we audit `~NNN lines` claims, mapped to the source file the
# claim describes. Add new entries as new contributor quick-reference docs
# emerge.
LINE_COUNT_TARGETS: dict[str, tuple[str, ...]] = {
    "src/cmd/stats.rs": (
        "docs/contributor/STATS_QUICK_REFERENCE.md",
    ),
    "src/cmd/frequency.rs": (
        "docs/contributor/FREQUENCY_QUICK_REFERENCE.md",
    ),
    "src/cmd/index.rs": (
        "docs/contributor/INDEX_QUICK_REFERENCE.md",
    ),
    "tests/test_stats.rs": (
        "docs/contributor/STATS_QUICK_REFERENCE.md",
    ),
    "tests/test_frequency.rs": (
        "docs/contributor/FREQUENCY_QUICK_REFERENCE.md",
    ),
    "tests/test_index.rs": (
        "docs/contributor/INDEX_QUICK_REFERENCE.md",
    ),
}


# Lines of the form: `path/to/file.rs` (~NNN lines)   or   `path/to/file.rs (~NNN lines)`
LINE_CLAIM_RE = re.compile(
    r"""
    `?(?P<path>[A-Za-z0-9_/.\-]+\.rs)`?     # the source path
    [^\n]*?                                 # anything (e.g. " | Main implementation ")
    \(\s*~\s*(?P<count>[\d,]+)\s+lines\s*\) # the (~NNN lines) marker
    """,
    re.VERBOSE,
)


def check_line_counts(report: Report, tolerance_pct: float) -> None:
    actual: dict[str, int] = {}
    for src_rel in LINE_COUNT_TARGETS:
        src = REPO_ROOT / src_rel
        if not src.exists():
            report.add(
                file=src_rel,
                line=None,
                category="config",
                message="source file not found; docs-drift-check.py is out of date",
            )
            continue
        actual[src_rel] = count_lines(src)

    doc_files = {d for targets in LINE_COUNT_TARGETS.values() for d in targets}
    for doc_rel in sorted(doc_files):
        doc = REPO_ROOT / doc_rel
        if not doc.exists():
            continue
        text = doc.read_text(encoding="utf-8")
        for lineno, line in enumerate(text.splitlines(), start=1):
            for m in LINE_CLAIM_RE.finditer(line):
                claimed_path = m.group("path")
                # Only audit known targets; ignore stray references.
                if claimed_path not in actual:
                    continue
                claimed = int(m.group("count").replace(",", ""))
                truth = actual[claimed_path]
                drift = abs(claimed - truth) / truth * 100.0
                if drift > tolerance_pct:
                    report.add(
                        file=doc_rel,
                        line=lineno,
                        category="line-count",
                        message=(
                            f"{claimed_path} claimed ~{claimed:,} lines; "
                            f"actual {truth:,} ({drift:.1f}% drift > "
                            f"{tolerance_pct:.0f}% tolerance)"
                        ),
                    )


# Files in which we audit feature-set enumerations.
# Each entry maps a doc file to a list of (regex, expected_set_key) checks.
# The regex must contain a named group "features" capturing the
# comma/space-separated feature list to compare.
FEATURE_LIST_CHECKS: list[tuple[str, str, re.Pattern[str]]] = [
    # docs/FEATURES.md — all_features shortcut bullet
    (
        "docs/FEATURES.md",
        "all_features",
        re.compile(
            r"`all_features`[^\n]*?all features enabled \((?P<features>[^)]+)\)",
        ),
    ),
    # docs/FEATURES.md — qsvmcp variant bullet
    (
        "docs/FEATURES.md",
        "qsvmcp",
        re.compile(
            r"`qsvmcp`[^\n]*?MCP[^\n]*?server use with (?P<features>[^.]+?)\s+features\.",
        ),
    ),
    # docs/FEATURES.md — distrib_features bullet
    (
        "docs/FEATURES.md",
        "distrib_features",
        re.compile(
            r"`distrib_features`[^\n]*?core distribution features enabled \((?P<features>[^)]+)\)",
        ),
    ),
    # README.md — qsvmcp variant description
    (
        "README.md",
        "qsvmcp",
        re.compile(
            r"`qsvmcp`[^\n]*?server use with (?P<features>[^.]+?)\s+features enabled\.",
        ),
    ),
    # docs/contributor/PROJECT_TECHNICAL_OVERVIEW.md — qsvmcp bullet
    (
        "docs/contributor/PROJECT_TECHNICAL_OVERVIEW.md",
        "qsvmcp",
        re.compile(
            r"qsvmcp[^\n]*?built with `qsvmcp` feature \((?P<features>[^)]+)\)",
        ),
    ),
]


def _parse_feature_blob(blob: str) -> set[str]:
    """Tokenize a comma-separated feature list into a set.

    The "and" between the last two entries (e.g. "geocode, luau, ..., and to")
    is normalized to a comma first. Whitespace-only separators are NOT
    supported because none of the audited doc anchors use that style — all
    real enumerations in qsv docs use commas, optionally with a trailing
    "and".
    """
    cleaned = re.sub(r"\band\b", ",", blob)
    return {tok for tok in (t.strip().strip(".,`") for t in cleaned.split(",")) if tok}


def check_feature_lists(report: Report, cargo: dict) -> None:
    features = get_features(cargo)
    truth_sets: dict[str, set[str]] = {
        "distrib_features": features["distrib_features"],
        "all_features": expand_all_features_for_docs(features),
        "qsvmcp": features["qsvmcp"],
    }
    # `feature_capable` is a build-time stub flag pulled in by the binary-
    # variant meta-features (qsvmcp, distrib_features, all_features). It has
    # no dependencies and isn't a user-facing capability, so docs enumerate
    # the *content* features only ("geocode, luau, mcp, ..."). Strip it
    # uniformly so doc comparisons match.
    for k in truth_sets:
        truth_sets[k] = truth_sets[k] - {"feature_capable"}

    for doc_rel, key, pattern in FEATURE_LIST_CHECKS:
        doc = REPO_ROOT / doc_rel
        if not doc.exists():
            report.add(
                file=doc_rel,
                line=None,
                category="config",
                message="doc file not found; docs-drift-check.py is out of date",
            )
            continue
        text = doc.read_text(encoding="utf-8")
        m = pattern.search(text)
        if not m:
            report.add(
                file=doc_rel,
                line=None,
                category="feature-list",
                message=(
                    f"could not locate the {key!r} feature enumeration via "
                    f"the configured regex — re-anchor docs-drift-check.py"
                ),
            )
            continue
        claimed = _parse_feature_blob(m.group("features"))
        truth = truth_sets[key]
        lineno = text[: m.start()].count("\n") + 1
        missing = truth - claimed
        extra = claimed - truth
        if missing or extra:
            parts = []
            if missing:
                parts.append(f"missing: {', '.join(sorted(missing))}")
            if extra:
                parts.append(f"unexpected: {', '.join(sorted(extra))}")
            report.add(
                file=doc_rel,
                line=lineno,
                category="feature-list",
                message=f"{key} enumeration drift — {'; '.join(parts)}",
            )


# Files in which we audit the MAX_STAT_COLUMNS claim.
# Each entry is (doc_rel, regex with named group "n"). Regexes are anchored
# to the canonical wording the doc uses for the *total* stat-column count;
# bare patterns like `(\d+)\s+statistics` would collide with other
# stat-count claims in the same file (e.g., STATS_DEFINITIONS.md:991 has
# "up to 17 statistics" describing the schema-side cap).
STAT_COLUMN_CHECKS: list[tuple[str, re.Pattern[str]]] = [
    (
        "README.md",
        re.compile(r"Compute up to (?P<n>\d+)\s+\[summary statistics\]"),
    ),
    (
        "docs/STATS_DEFINITIONS.md",
        re.compile(r"\*?\*?Total:\s*(?P<n>\d+)\s+statistics"),
    ),
    (
        "docs/contributor/STATS_QUICK_REFERENCE.md",
        re.compile(r"Supports (?:up to )?(?P<n>\d+)\+?\s+output columns"),
    ),
    (
        "docs/PERFORMANCE.md",
        re.compile(r"up to (?P<n>\d+) total\*?\*?\s+columns?"),
    ),
]


def check_stat_columns(report: Report) -> None:
    truth = get_max_stat_columns()
    for doc_rel, pattern in STAT_COLUMN_CHECKS:
        doc = REPO_ROOT / doc_rel
        if not doc.exists():
            continue
        text = doc.read_text(encoding="utf-8")
        m = pattern.search(text)
        if not m:
            report.add(
                file=doc_rel,
                line=None,
                category="stat-columns",
                message=(
                    "could not locate the MAX_STAT_COLUMNS claim via the "
                    "configured regex — re-anchor docs-drift-check.py"
                ),
            )
            continue
        n = int(m.group("n"))
        if n != truth:
            lineno = text[: m.start()].count("\n") + 1
            report.add(
                file=doc_rel,
                line=lineno,
                category="stat-columns",
                message=(
                    f"MAX_STAT_COLUMNS is {truth}; doc still says {n}"
                ),
            )


# Extract the set of bare command names (backticked, leading-asterisk strip)
# from a chunk of text. Used to compare the 🤯 OOM lists in PERFORMANCE.md
# and PERFORMANCE_TLDR.md.
BACKTICKED_CMD_RE = re.compile(r"`([a-z][a-z0-9_]*)`")


# Anchor: the prose sentence in PERFORMANCE.md that enumerates 🤯 commands.
PERFORMANCE_OOM_RE = re.compile(
    r"exploding head[^\n]*?-\s*🤯\s*\)\s*,\s*that\s+require\s+qsv\s+to\s+load\s+the\s+entire\s+CSV\s+into\s+memory\s*-\s*(?P<list>[^.]+)\.",
)


# Anchor: the bullet block in PERFORMANCE_TLDR.md that lists 🤯 commands.
PERFORMANCE_TLDR_OOM_RE = re.compile(
    r"full memory loading[^\n]*?🤯\)[^\n]*?\*\*\s*\n(?P<list>(?:-\s+`[^`]+`[^\n]*\n)+)",
)


def check_oom_lists_in_sync(report: Report) -> None:
    """Cross-check that PERFORMANCE.md and PERFORMANCE_TLDR.md list the same
    🤯 ("loads entire CSV into memory") commands.

    The two docs duplicate the same command set in different formats — the
    long doc as prose, the TLDR as a bullet list — so they drift
    independently. We extract the bare command names from each (ignoring
    parenthetical qualifiers like "when not using --sorted") and report any
    asymmetry."""
    perf = REPO_ROOT / "docs/PERFORMANCE.md"
    tldr = REPO_ROOT / "docs/PERFORMANCE_TLDR.md"
    if not perf.exists() or not tldr.exists():
        return

    perf_text = perf.read_text(encoding="utf-8")
    tldr_text = tldr.read_text(encoding="utf-8")

    m_perf = PERFORMANCE_OOM_RE.search(perf_text)
    m_tldr = PERFORMANCE_TLDR_OOM_RE.search(tldr_text)
    if not m_perf:
        report.add(
            file="docs/PERFORMANCE.md",
            line=None,
            category="oom-list",
            message=(
                "could not locate the 🤯 OOM-command sentence via the "
                "configured regex — re-anchor docs-drift-check.py"
            ),
        )
        return
    if not m_tldr:
        report.add(
            file="docs/PERFORMANCE_TLDR.md",
            line=None,
            category="oom-list",
            message=(
                "could not locate the 🤯 OOM-command bullet list via the "
                "configured regex — re-anchor docs-drift-check.py"
            ),
        )
        return

    perf_cmds = set(BACKTICKED_CMD_RE.findall(m_perf.group("list")))
    tldr_cmds = set(BACKTICKED_CMD_RE.findall(m_tldr.group("list")))

    if perf_cmds != tldr_cmds:
        missing_in_tldr = perf_cmds - tldr_cmds
        missing_in_perf = tldr_cmds - perf_cmds
        if missing_in_tldr:
            lineno = tldr_text[: m_tldr.start()].count("\n") + 1
            report.add(
                file="docs/PERFORMANCE_TLDR.md",
                line=lineno,
                category="oom-list",
                message=(
                    f"🤯 list is missing commands present in PERFORMANCE.md: "
                    f"{', '.join(sorted(missing_in_tldr))}"
                ),
            )
        if missing_in_perf:
            lineno = perf_text[: m_perf.start()].count("\n") + 1
            report.add(
                file="docs/PERFORMANCE.md",
                line=lineno,
                category="oom-list",
                message=(
                    f"🤯 list is missing commands present in PERFORMANCE_TLDR.md: "
                    f"{', '.join(sorted(missing_in_perf))}"
                ),
            )


USER_AGENT_EXAMPLE_RE = re.compile(
    r"^#\s*QSV_USER_AGENT\s*=\s*qsv/(?P<version>\d+\.\d+\.\d+)",
    re.MULTILINE,
)


def check_user_agent_version(report: Report, cargo: dict) -> None:
    truth = get_qsv_version(cargo)
    doc_rel = "dotenv.template"
    doc = REPO_ROOT / doc_rel
    if not doc.exists():
        return
    text = doc.read_text(encoding="utf-8")
    m = USER_AGENT_EXAMPLE_RE.search(text)
    if not m:
        return  # example line removed/restructured; not an error
    claimed = m.group("version")
    if claimed != truth:
        lineno = text[: m.start()].count("\n") + 1
        report.add(
            file=doc_rel,
            line=lineno,
            category="version",
            message=(
                f"QSV_USER_AGENT example uses qsv/{claimed}; "
                f"Cargo.toml is qsv/{truth}"
            ),
        )


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[1])
    parser.add_argument(
        "--line-tolerance",
        type=float,
        default=10.0,
        help="Percent drift permitted in '~NNN lines' claims (default: 10).",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Emit machine-readable JSON instead of human-readable text.",
    )
    args = parser.parse_args(argv)

    report = Report()
    try:
        cargo = load_cargo_toml()
        check_feature_lists(report, cargo)
        check_line_counts(report, args.line_tolerance)
        check_stat_columns(report)
        check_oom_lists_in_sync(report)
        check_user_agent_version(report, cargo)
    except (OSError, KeyError, RuntimeError) as exc:
        print(f"docs-drift-check: error: {exc}", file=sys.stderr)
        return 2

    if args.json:
        print(
            json.dumps(
                {
                    "ok": not report,
                    "findings": [f.__dict__ for f in report.findings],
                },
                indent=2,
            ),
        )
    else:
        if not report:
            print("docs-drift-check: no drift detected.")
        else:
            print(
                f"docs-drift-check: {len(report.findings)} finding(s):\n",
            )
            for f in report.findings:
                print(f.format())
            print(
                "\nFix the doc lines above (or update docs-drift-check.py "
                "if a doc anchor moved).",
            )
    return 1 if report else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
