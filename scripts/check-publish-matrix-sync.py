#!/usr/bin/env python3
"""Assert the per-platform retry workflows' matrix entries match publish.yml's.

publish-windows.yml and publish-linux.yml exist so a failure confined to one platform can
be retried without rebuilding and re-uploading the other targets (publish.yml uploads with
overwrite: true, and PGO output is not deterministic). That means those matrix entries are
written twice, and if they drift, a retry ships binaries built with different features than
the full publish would have produced.

That drift is not hypothetical: `viz_static` sat in three near-duplicate publish workflows
and blew GitHub's 6h job cap on Windows (run 31243109948) before it was found.

The two retry files differ in how complete they must be:

  publish-windows.yml  must cover EVERY Windows target in publish.yml. Windows targets fail
                       together (shared runner image and job cap), so a partial retry file
                       is a bug.
  publish-linux.yml    may cover a SUBSET of publish.yml's Linux targets. It was added for
                       x86_64-unknown-linux-gnu, which failed three times in run
                       34670874141 while musl and aarch64 published fine (see #4589).
                       Requiring all three would force copies of entries that have never
                       needed a retry - more duplication to drift, for no benefit.

In both cases every entry that IS present must match publish.yml exactly, and an entry
present only in a retry file is always an error.

Note what the subset relaxation costs: a Linux target MISSING from publish-linux.yml is
silent by design, so "1 Linux entry match" means one entry was checked, NOT that Linux is
covered. Adding a Linux target to publish.yml does not oblige anyone to add it here.

Exits non-zero with a diff if the entries disagree.
"""

import sys
from pathlib import Path

try:
    import yaml
except ImportError:
    sys.exit("PyYAML required: pip install pyyaml")

ROOT = Path(__file__).resolve().parent.parent
FULL = ROOT / ".github/workflows/publish.yml"

# (retry workflow, platform label, substring identifying the platform's targets,
#  must the retry file cover every such target in publish.yml?)
RETRIES = [
    (ROOT / ".github/workflows/publish-windows.yml", "Windows", "windows", True),
    (ROOT / ".github/workflows/publish-linux.yml", "Linux", "linux", False),
]


def entries(path, needle):
    doc = yaml.safe_load(path.read_text())
    jobs = doc.get("jobs", {})
    if "publish" not in jobs:
        sys.exit(f"{path.name}: no `publish` job")
    matrix = jobs["publish"].get("strategy", {}).get("matrix", {}).get("job", [])
    if not isinstance(matrix, list):
        sys.exit(f"{path.name}: `publish` job has no literal matrix list")
    return {
        e["target"]: e
        for e in matrix
        if isinstance(e, dict) and needle in str(e.get("target", ""))
    }


def compare(full, retry, retry_name, label, require_all):
    problems = []

    # An entry only in the retry file is always wrong: it would build a target the real
    # release does not, using settings nothing else validates.
    for target in sorted(set(retry) - set(full)):
        problems.append(f"in {retry_name} but not {FULL.name}: {target}")

    if require_all:
        for target in sorted(set(full) - set(retry)):
            problems.append(f"in {FULL.name} but not {retry_name}: {target}")

    for target in sorted(set(full) & set(retry)):
        a, b = full[target], retry[target]
        for key in sorted(set(a) | set(b)):
            av, bv = a.get(key, "<missing>"), b.get(key, "<missing>")
            if av != bv:
                problems.append(
                    f"{target}: `{key}` differs\n"
                    f"    {FULL.name}: {av!r}\n"
                    f"    {retry_name}: {bv!r}"
                )

    if not full:
        problems.append(
            f"{FULL.name}: found no {label} matrix entries — did the matrix move?"
        )
    if not retry:
        problems.append(
            f"{retry_name}: found no {label} matrix entries — did the matrix move?"
        )

    return problems


def main():
    all_problems = []
    summaries = []

    for path, label, needle, require_all in RETRIES:
        if not path.exists():
            all_problems.append(f"{path.name}: missing")
            continue
        full = entries(FULL, needle)
        retry = entries(path, needle)
        problems = compare(full, retry, path.name, label, require_all)
        if problems:
            all_problems.append(f"--- {path.name} ---")
            all_problems.extend(problems)
        else:
            shared = sorted(set(full) & set(retry))
            summaries.append(
                f"{path.name}: {len(shared)} {label} "
                f"{'entry' if len(shared) == 1 else 'entries'} match "
                f"({', '.join(shared)})"
            )

    if all_problems:
        print("check-publish-matrix-sync: publish workflows disagree:\n")
        for p in all_problems:
            print(f"  {p}")
        print(
            "\nA retry workflow's matrix entries must be identical to publish.yml's for\n"
            "the same target. Edit both, or the platform-only retry will ship different\n"
            "binaries than the full publish would have."
        )
        return 1

    print("check-publish-matrix-sync: OK")
    for s in summaries:
        print(f"  {s}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
