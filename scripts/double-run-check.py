#!/usr/bin/env python3
"""
double-run-check.py — Find integration tests that run one command more than once.

`tests/workdir.rs` helpers each spawn the qsv binary. A test that calls two of
them on the same `Command` runs that command twice, so the properties it
asserts describe DIFFERENT executions:

    wrk.assert_success(&mut cmd);                           // run #1: exit status
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);  // run #2: the data

Either direction can be masked — a run that succeeds while emitting the wrong
data, or a second run that fails yet still emits parseable output. The `*_on_success`
/ `*_on_error` helpers exist to capture one `Output` and assert against that.
This script finds the sites that still don't.

Usage:
    scripts/double-run-check.py [--json] [--hazards] [PATH ...]

    (no flags)  summary counts by shape and by file
    --json      one JSON object per site on stdout, for scripted conversion
    --hazards   only sites where converting could change WHEN the command runs

Exit status is 0 regardless of findings; this is a reporting tool, not a gate.

Two classes of finding must NOT be blindly converted, and are reported separately:

  * `mutated`  — a `.arg()` / `.env()` call sits between the two runs, so they
    are deliberately two different commands.
  * `hazard`   — a statement between the two runs observes or alters state
    (`.exists()`, `read_to_string`, `wrk.create*`, `std::fs::`). Collapsing to a
    single run moves WHEN the command executes relative to that statement. The
    single run must be anchored at the EARLIEST original execution point, or
    assertions that used to observe the command's side effects start running
    before it and pass vacuously.

Calls in different `if`/`else` branches are NOT double runs; each line's
enclosing block is identified so those are excluded (`tests/test_slice.rs::test_slice`
is the case that proves it — `assert_success` in the `--json` arm, `read_stdout`
in the `else`).

A test that runs one command twice ON PURPOSE (caching, idempotence, indexed vs
unindexed) opts out with a comment anywhere in the function:

    // double-run-check: intentional -- second request must hit the disk cache
"""

import argparse
import collections
import glob
import json
import os
import re
import sys

# Workdir helpers that spawn the binary. `output_stderr` is included: it runs the
# command like the rest, it just also carries a legacy "No error" sentinel.
RUNNERS = [
    "output",
    "run",
    "stdout",
    "read_stdout",
    "read_stdout_on_success",
    "read_stdout_and_stderr",
    "read_stdout_and_stderr_on_success",
    "read_stdout_and_stderr_on_error",
    "stdout_and_stderr",
    "stdout_and_stderr_on_success",
    "stdout_and_stderr_on_error",
    "stdout_on_success",
    "stderr_on_success",
    "stderr_on_error",
    "output_stderr",
    "assert_success",
    "assert_err",
]
RUN_RE = re.compile(r"wrk\.(" + "|".join(RUNNERS) + r")(?:::<[^>]*>)?\(&mut (\w+)\)")
# a fresh `let mut cmd = wrk.command(..)` starts a new logical command, even when
# it reuses the name of an earlier one
BIND_RE = re.compile(
    r"^\s*let mut (\w+)\s*(?::[^=]*)?=\s*(?:[\w:]*::)?(?:wrk\.command|Command::new)"
)
FN_RE = re.compile(r"fn (\w+)\(")
MUT_RE = re.compile(r"\.\s*(args?|envs?|env_remove|env_clear|current_dir|stdin)\s*\(")
# statements whose meaning depends on whether the command has run yet
HAZARD_RE = re.compile(
    r"\.exists\(\)|read_to_string|read_csv|wrk\.path\(|wrk\.from_str|load_test"
    r"|std::fs::|fs::(read|write|metadata|remove)|File::create"
    r"|wrk\.create(_from_string|_indexed|_with_delim)?\("
)
# helpers that only assert the exit status vs. those that read what the command produced
STATUS = {"assert_success", "assert_err"}
# a test that runs one command twice deliberately says so with this marker
OPT_OUT_RE = re.compile(r"//\s*double-run-check:\s*intentional")


def blank_literals(src):
    """Replace the CONTENTS of string/char literals and comments with spaces.

    Line structure is preserved, so line numbers still line up. Without this the
    braces inside a `r#"[{"a": 1}]"#` fixture are counted as blocks, which
    corrupts block identity for the rest of the function and silently hides
    real double-run sites (tests/test_search.rs::search_indexed_parallel_json
    and five in tests/test_excel.rs are the cases that proved it).

    Char and byte-char literals matter for the same reason and for a worse one:
    `b'{'` (tests/test_viz.rs) unbalances the brace stack, and `split_once('"')`
    (also test_viz) would otherwise open a bogus string literal that blanks
    everything up to the next quote. A `'` that is not a char literal is a
    lifetime and is left alone.
    """
    out = []
    i, n = 0, len(src)
    while i < n:
        c = src[i]
        # raw string: r, r#, r##, ...
        if c == "r" and i + 1 < n and src[i + 1] in '"#':
            j = i + 1
            hashes = 0
            while j < n and src[j] == "#":
                hashes += 1
                j += 1
            if j < n and src[j] == '"':
                close = '"' + "#" * hashes
                end = src.find(close, j + 1)
                end = n if end == -1 else end + len(close)
                out.append(" " * (end - i) if "\n" not in src[i:end] else re.sub(r"[^\n]", " ", src[i:end]))
                i = end
                continue
        if c == '"':
            j = i + 1
            while j < n:
                if src[j] == "\\":
                    j += 2
                    continue
                if src[j] == '"':
                    j += 1
                    break
                j += 1
            out.append(re.sub(r"[^\n]", " ", src[i:j]))
            i = j
            continue
        # char / byte-char literal: 'x', '\n', b'{'. A `'` that is NOT one of
        # these is a lifetime (`&'a str`, `'static`) and must be left alone.
        if c == "'":
            j = i + 1
            if j < n and src[j] == "\\":
                j += 2
                while j < n and src[j] != "'":
                    j += 1
                j += 1
            elif j + 1 < n and src[j + 1] == "'":
                j += 2
            else:
                out.append(c)  # lifetime
                i += 1
                continue
            out.append(re.sub(r"[^\n]", " ", src[i:j]))
            i = j
            continue
        if c == "/" and i + 1 < n and src[i + 1] == "/":
            j = src.find("\n", i)
            j = n if j == -1 else j
            out.append(" " * (j - i))
            i = j
            continue
        if c == "/" and i + 1 < n and src[i + 1] == "*":
            j = src.find("*/", i + 2)
            j = n if j == -1 else j + 2
            out.append(re.sub(r"[^\n]", " ", src[i:j]))
            i = j
            continue
        out.append(c)
        i += 1
    return "".join(out)


def scan(path):
    """Yield one dict per command variable that is run more than once in a test fn."""
    raw = open(path, encoding="utf-8").read()
    lines = raw.split("\n")
    # braces are counted on a literal-free copy; everything else reads the real source
    code_lines = blank_literals(raw).split("\n")
    fns = [(i, m.group(1)) for i, l in enumerate(lines) if (m := FN_RE.match(l))]
    fns.append((len(lines), None))

    for (start, name), (end, _) in zip(fns, fns[1:]):
        body = lines[start:end]
        # Identify the enclosing block of every line by a stack of unique block ids,
        # not by brace DEPTH: the two arms of an if/else sit at the same depth but are
        # different blocks, and only one of them runs.
        blocks = []
        stack = []
        next_id = 0
        for line in code_lines[start:end]:
            trimmed = line.strip()
            # a leading `}` closes the block this line's statement is NOT in,
            # so pop before recording (`} else {` belongs to neither arm)
            closes_first = trimmed.startswith("}")
            if closes_first and stack:
                stack.pop()
            blocks.append(tuple(stack))
            for ch in trimmed[1:] if closes_first else trimmed:
                if ch == "{":
                    stack.append(next_id)
                    next_id += 1
                elif ch == "}" and stack:
                    stack.pop()

        # a test that runs one command twice ON PURPOSE (caching, idempotence,
        # index-vs-no-index) opts out by saying so
        if OPT_OUT_RE.search("\n".join(body)):
            continue

        generation = collections.Counter()
        events = collections.defaultdict(list)
        for k, line in enumerate(body):
            if line.strip().startswith("//"):
                continue
            if m := BIND_RE.match(line):
                generation[m.group(1)] += 1
            for m in RUN_RE.finditer(line):
                events[(m.group(2), generation[m.group(2)])].append((k, m.group(1)))

        for (var, _gen), evs in events.items():
            if len(evs) < 2:
                continue
            first, last = evs[0][0], evs[-1][0]
            # calls in different blocks (if/else arms, separate loop bodies) are
            # alternatives, not sequential runs of the same command
            if any(blocks[k] != blocks[first] for k, _ in evs):
                continue
            between = body[first + 1 : last]
            names = [n for _, n in evs]
            yield {
                "file": path,
                "fn": name,
                "var": var,
                "line": start + 1 + first,
                "end_line": start + 1 + last,
                "runners": names,
                "runs": len(evs),
                "reads_data": bool({n for n in names} - STATUS),
                "mutated": bool(MUT_RE.search("\n".join(between))),
                "hazard": [b.strip()[:100] for b in between if HAZARD_RE.search(b)],
            }


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[1])
    ap.add_argument("paths", nargs="*", default=None)
    ap.add_argument("--json", action="store_true", help="emit one JSON object per site")
    ap.add_argument("--hazards", action="store_true", help="only ordering-hazard sites")
    args = ap.parse_args()

    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    paths = args.paths or sorted(glob.glob(os.path.join(root, "tests", "test_*.rs")))

    sites = [s for p in paths for s in scan(p)]
    if args.hazards:
        sites = [s for s in sites if s["hazard"] and not s["mutated"]]

    if args.json:
        for s in sites:
            print(json.dumps(s))
        return

    convertible = [s for s in sites if not s["mutated"] and not s["hazard"]]
    print(f"double-run sites:            {len(sites)}")
    print(f"  deliberately two commands: {sum(1 for s in sites if s['mutated'])}")
    print(f"  ordering hazard:           {sum(1 for s in sites if s['hazard'] and not s['mutated'])}")
    print(f"  mechanically convertible:  {len(convertible)}")
    if not sites:
        return
    print("\nby shape:")
    for shape, n in collections.Counter(
        tuple(sorted(set(s["runners"]))) for s in sites
    ).most_common():
        print(f"  {n:4d}  {' + '.join(shape)}")
    print("\nby file:")
    for f, n in collections.Counter(s["file"] for s in sites).most_common():
        print(f"  {n:4d}  {os.path.relpath(f, root)}")


if __name__ == "__main__":
    sys.exit(main())
