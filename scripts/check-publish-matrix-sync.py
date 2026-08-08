#!/usr/bin/env python3
"""Assert publish-windows.yml's matrix entries match publish.yml's Windows entries.

publish-windows.yml exists so a Windows-only failure can be retried without rebuilding
and re-uploading the other targets (publish.yml uploads with overwrite: true). That means
the Windows matrix entries are written twice, and if they drift, a retry ships binaries
built with different features than the full publish would have produced.

That drift is not hypothetical: `viz_static` sat in three near-duplicate publish workflows
and blew GitHub's 6h job cap on Windows (run 31243109948) before it was found.

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
WIN = ROOT / ".github/workflows/publish-windows.yml"


def windows_entries(path):
    doc = yaml.safe_load(path.read_text())
    jobs = doc.get("jobs", {})
    if "publish" not in jobs:
        sys.exit(f"{path.name}: no `publish` job")
    entries = jobs["publish"].get("strategy", {}).get("matrix", {}).get("job", [])
    if not isinstance(entries, list):
        sys.exit(f"{path.name}: `publish` job has no literal matrix list")
    return {
        e["target"]: e
        for e in entries
        if isinstance(e, dict) and "windows" in str(e.get("target", ""))
    }


def main():
    full = windows_entries(FULL)
    win = windows_entries(WIN)

    problems = []

    only_full = sorted(set(full) - set(win))
    only_win = sorted(set(win) - set(full))
    if only_full:
        problems.append(f"in {FULL.name} but not {WIN.name}: {', '.join(only_full)}")
    if only_win:
        problems.append(f"in {WIN.name} but not {FULL.name}: {', '.join(only_win)}")

    for target in sorted(set(full) & set(win)):
        a, b = full[target], win[target]
        for key in sorted(set(a) | set(b)):
            av, bv = a.get(key, "<missing>"), b.get(key, "<missing>")
            if av != bv:
                problems.append(
                    f"{target}: `{key}` differs\n"
                    f"    {FULL.name}: {av!r}\n"
                    f"    {WIN.name}: {bv!r}"
                )

    if not full:
        problems.append(f"{FULL.name}: found no Windows matrix entries — did the matrix move?")

    if problems:
        print("check-publish-matrix-sync: publish workflows disagree:\n")
        for p in problems:
            print(f"  {p}")
        print(
            "\nThe Windows entries in publish.yml and publish-windows.yml must be identical.\n"
            "Edit both, or the Windows-only retry will ship different binaries."
        )
        return 1

    print(
        f"check-publish-matrix-sync: OK — {len(full)} Windows "
        f"{'entry' if len(full) == 1 else 'entries'} match "
        f"({', '.join(sorted(full))})."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
