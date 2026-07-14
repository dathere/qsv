#!/usr/bin/env python3
"""Regenerate benchmarks/index.html — an interactive `qsv viz` benchmark dashboard.

This dogfoods qsv itself: it reads the CSVs that `scripts/benchmarks.sh` produces
in `scripts/results/`, shapes them with qsv (`search`/`sort`/`slice`/`sqlp`/`luau`),
renders each chart with `qsv viz`, and assembles them into one page under
`benchmarks/` that is deployed to GitHub Pages by `viz-gallery-pages.yml`
(https://dathere.github.io/qsv/benchmarks/).

The dashboard doubles as a `viz` showcase: it spans bar, grouped-bar, line, heatmap,
treemap, box and scatter-bubble traces, each chosen to fit its story. Every `qsv viz`
run sets QSV_VIZ_CDN=1 (plotly loaded from CDN, so the committed HTML stays small) and
QSV_VIZ_NO_COMPRESS=1 (plain-text output).

Usage (from the repo root), after a fresh benchmark run updates scripts/results/:

    cargo build --bin qsv -F all_features
    python3 scripts/gen_benchmark_viz.py
    git add benchmarks && git commit   # Pages redeploys on push to master

Set QSV_BIN to point at a specific binary; otherwise target/{debug,release}/qsv or a
`qsv` on PATH is used.

A static PNG (benchmarks/hero.png) for the Wiki thumbnail is rendered best-effort via the
`viz_static` feature (needs a local Chrome/Firefox); it's skipped gracefully if unavailable.

Publishing the Wiki "Benchmarks" page is a separate MANUAL step (the wiki is its own
repo and interactive Plotly can't render in wiki markdown, which strips <script>):

    git clone https://github.com/dathere/qsv.wiki.git
    # copy benchmarks/Benchmarks.wiki.md (a ready-to-use starter page is written there)
    cp benchmarks/Benchmarks.wiki.md qsv.wiki/Benchmarks.md
    git -C qsv.wiki add Benchmarks.md && git -C qsv.wiki commit && git -C qsv.wiki push

NOTE on the historical trend/heatmap: cross-version numbers are NOT strictly
apples-to-apples — commands gain features over time (see scripts/results/README.md),
so the dashboard carries that caveat prominently. The trend line spans the FULL release
history and, for each command, follows the fastest variant available at the time (base
scan early on, indexed variant once it exists — for search/searchset that step lands at
10.0.0). The heatmap keeps a recent window for legibility. `count` is excluded from the
shared-scale throughput charts (it just reads the row count from the .idx — tens of
millions of "records/sec") but appears in the per-row-normalized heatmap.
"""
import csv
import os
import re
import shutil
import subprocess
import sys
import tempfile
import textwrap
from html import escape as html_escape

SCRIPTS = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(SCRIPTS)
RESULTS = os.path.join(SCRIPTS, "results")
OUT = os.path.join(REPO, "benchmarks")
LATEST = os.path.join(RESULTS, "latest_results.csv")
HISTORY = os.path.join(RESULTS, "benchmark_results.csv")
RUN_INFO = os.path.join(RESULTS, "latest_run_info.tsv")

# Recent-release window for the heatmap (the trend line uses the FULL history).
HEATMAP_VERSIONS = 15

PAGES_URL = "https://dathere.github.io/qsv/benchmarks/"
# Deep-link base for the per-chart "see the qsv viz command" shortcut (line range appended).
SOURCE_URL = "https://github.com/dathere/qsv/blob/master/scripts/gen_benchmark_viz.py"

# Index-advantage chart: only commands whose _index variant is a STARK win (flat pairs like
# validate and sample are deliberately excluded — an index barely helps them).
INDEX_PAIR_COMMANDS = ["stats", "frequency", "search", "searchset", "tojsonl"]
# Full-history trend & heatmap: marquee commands shown with their index variant wherever it
# exists (all releases for stats/frequency; from 10.0.0 for search/searchset — the base
# variant is used before, so each line stays continuous and the index-adoption jump is visible).
# `count` is in the heatmap only (per-row normalized); it's excluded from the trend line and
# the shared-scale charts because its ~90M "records/sec" index read would squash everything.
TREND_NAMES = ["stats", "frequency", "search", "searchset"]
HEATMAP_NAMES = ["count", "stats", "frequency", "search", "searchset"]
# Flagship deep-dives: variant sets whose growth-since-first-release shows throughput held (or
# improved) even as the two top commands gained features over ~60 releases.
STATS_GROWTH = ["stats", "stats_index", "stats_everything", "stats_everything_index"]
FREQ_GROWTH = ["frequency", "frequency_index", "frequency_no_limit", "frequency_sorted"]
# "Index superpowers": commands whose index win is not just skipping the opening scan but doing
# an order of magnitude less work — seeking straight to the wanted rows (slice, sample) or reusing
# cached statistics (schema). Shown as a speedup FACTOR so the biggest multiple reads as the
# tallest bar (schema ~78x), which an absolute axis would bury under slice's larger raw throughput.
SUPERPOWER_COMMANDS = ["schema", "slice_one_middle", "slice_last_1k", "slice_last_1k_json",
                       "sample_10", "sample_1000", "frequency_ignorecase"]
# Command families with several benchmarks each → readable per-family box.
FAMILY_BOX = ["apply", "exclude", "frequency", "luau", "sample", "search", "slice",
              "split", "sqlp", "stats", "tojsonl", "validate"]
# Deltas beyond this magnitude are treated as measurement variance / changed-benchmark
# artifacts (see README caveat) and dropped from the change charts so they stay readable.
DELTA_CLAMP = 100.0
# Plausibility ceiling for the normalized flagship growth charts: any non-`count` command
# reporting more than this many recs/sec is a glitch run (only count's .idx read exceeds it),
# and an unfiltered upward spike would rescale the whole normalized chart. See prep_growth.
GROWTH_CEILING = 10_000_000


def find_qsv():
    if os.environ.get("QSV_BIN"):
        return os.environ["QSV_BIN"]
    for rel in ("target/debug/qsv", "target/release/qsv"):
        cand = os.path.join(REPO, rel)
        if os.path.exists(cand):
            return cand
    found = shutil.which("qsv")
    if found:
        return found
    sys.exit("qsv binary not found: build it (cargo build --bin qsv -F all_features) or set QSV_BIN")


QSV = find_qsv()
TMP = tempfile.mkdtemp(prefix="benchviz_")


def tmp(name):
    return os.path.join(TMP, name)


def qsv(args, out, stdin=None):
    """Run `qsv <args>`, writing stdout to file `out`. `stdin` is an optional input file path.

    The subcommand's own input file (if any) is passed inside `args`; `stdin` is only for the
    piped stages that read from stdin. Returns `out` for easy chaining."""
    with open(out, "w", encoding="utf-8") as fh:
        fin = open(stdin, "r", encoding="utf-8") if stdin else None
        try:
            subprocess.run([QSV, *args], check=True, stdout=fh, stdin=fin,
                           stderr=subprocess.PIPE, text=True)
        except subprocess.CalledProcessError as e:
            sys.exit(f"qsv {' '.join(args)} failed:\n{e.stderr}")
        finally:
            if fin:
                fin.close()
    return out


def viz(subcmd, src, flags, slug, title, desc):
    """Render one chart to benchmarks/<slug>.html and return its figure descriptor."""
    out = os.path.join(OUT, f"{slug}.html")
    env = {**os.environ, "QSV_VIZ_CDN": "1", "QSV_VIZ_NO_COMPRESS": "1", "QSV_PROGRESSBAR": "0"}
    try:
        subprocess.run([QSV, "viz", subcmd, src, *flags, "-o", out],
                       check=True, capture_output=True, text=True, env=env)
    except subprocess.CalledProcessError as e:
        sys.exit(f"qsv viz {subcmd} ({slug}) failed:\n{e.stderr}")
    with open(out, encoding="utf-8") as fh:
        html = fh.read()
    with open(out, "w", encoding="utf-8") as fh:
        fh.write(inject_resize_reporter(html))
    return {"slug": slug, "title": title, "desc": desc}


# postMessage height reporter injected into each chart iframe so the parent page can size it
# exactly to its content (no inner scrollbar, no trailing whitespace). Mirrors examples/viz.
RESIZE_REPORTER_JS = (
    "<script>(function(){function r(){parent.postMessage("
    "{qsvVizHeight:document.documentElement.scrollHeight},\"*\");}"
    "addEventListener(\"load\",r);addEventListener(\"resize\",r);"
    "if(window.ResizeObserver)new ResizeObserver(r).observe(document.body);"
    "setTimeout(r,200);setTimeout(r,800);})();</script>")

RESIZE_LISTENER_JS = (
    "<script>addEventListener(\"message\",function(e){"
    "var h=e.data&&e.data.qsvVizHeight;if(typeof h!==\"number\")return;"
    "var f=document.getElementsByClassName(\"chart\");"
    "for(var i=0;i<f.length;i++)if(f[i].contentWindow===e.source){"
    "if(Math.abs(f[i].clientHeight-h)>1)f[i].style.height=h+\"px\";break;}});</script>")


def inject_resize_reporter(html):
    html = re.sub(r"<script>[^<]*qsvVizHeight[^<]*</script>\n?", "", html)
    idx = html.rfind("</body>")
    if idx == -1:
        return html + RESIZE_REPORTER_JS
    return html[:idx] + RESIZE_REPORTER_JS + "\n" + html[idx:]


def read_run_info():
    with open(RUN_INFO, encoding="utf-8") as fh:
        rows = list(csv.DictReader(fh, delimiter="\t"))
    return rows[0] if rows else {}


def recent_versions(n):
    """The n most-recent distinct versions. benchmark_results.csv is newest-first, so take
    the first n distinct `version` values in file order."""
    seen, out = set(), []
    with open(HISTORY, encoding="utf-8") as fh:
        for row in csv.DictReader(fh):
            v = row["version"]
            if v not in seen:
                seen.add(v)
                out.append(v)
                if len(out) >= n:
                    break
    return out


def alt(names):
    """A qsv regex alternation matching any of `names` exactly (dots escaped)."""
    return "^(" + "|".join(re.escape(x) for x in names) + ")$"


# ---------------------------------------------------------------------------- data prep

def prep_index():
    # add `command` (name minus trailing _index) and `index_status`, then keep the marquee pairs
    f1 = qsv(["luau", "map", "command",
              'local n=name; if n:sub(-6)=="_index" then return n:sub(1,#n-6) else return n end',
              LATEST], tmp("idx1.csv"))
    f2 = qsv(["luau", "map", "index_status",
              '(name:sub(-6)=="_index") and "with index" or "no index"', f1], tmp("idx2.csv"))
    return qsv(["search", "-s", "command", alt(INDEX_PAIR_COMMANDS), f2], tmp("index.csv"))


def prep_count():
    # `count` on its own axis: with an index it reads the precomputed row count → effectively
    # instant (an order of magnitude over the plain scan), a range too wide for the shared charts.
    f = qsv(["search", "-s", "name", r"^count(_index)?$", LATEST], tmp("cnt0.csv"))
    return qsv(["luau", "map", "index_status",
                '(name:sub(-6)=="_index") and "with index" or "no index"', f], tmp("count.csv"))


def prep_superpowers():
    """CSV of `(command, speedup)` — the index speedup FACTOR for the seek/reuse-tier commands,
    sorted so the biggest multiple is the first (tallest) bar. Returns (path, sorted_rows) so the
    prose can cite the top multiples. Computed in Python because it needs each command's index vs
    non-index ratio, an awkward pivot to express in one qsv pass."""
    vals = {}
    with open(LATEST, encoding="utf-8") as fh:
        for r in csv.DictReader(fh):
            try:
                vals[r["name"]] = float(r["recs_per_sec"])
            except (KeyError, ValueError):
                pass
    rows = []
    for cmd in SUPERPOWER_COMMANDS:
        base, idx = vals.get(cmd), vals.get(cmd + "_index")
        if base and idx and base > 0:
            rows.append((cmd, idx / base))
    rows.sort(key=lambda r: -r[1])
    dst = tmp("superpowers.csv")
    with open(dst, "w", encoding="utf-8", newline="") as fh:
        w = csv.writer(fh)
        w.writerow(["command", "speedup"])
        for cmd, ratio in rows:
            w.writerow([cmd, f"{ratio:.1f}"])
    return dst, rows


def prep_sqlp():
    # schema-cache knob only — the *_vs_duckdb variants are a different (engine) comparison.
    f = qsv(["search", "-s", "name",
             r"^sqlp(_aggregations(_expensive)?(_use_schema_cache|_streaming)?)?$",
             LATEST], tmp("sqlp0.csv"))
    return qsv(["sort", "-s", "recs_per_sec", "-N", "-R", f], tmp("sqlp.csv"))


def merge_index(commands, dst):
    """Full-history long CSV `(command, version, tstamp, recs_per_sec)` where each command uses
    its `_index` variant for every release that has one, else the base variant — so a series
    stays continuous across the release where index support was added (the jump is the story)."""
    pat = "^(" + "|".join(re.escape(c) + "(_index)?" for c in commands) + ")$"
    f = qsv(["search", "-s", "name", pat, HISTORY], tmp("mi0.csv"))
    f = qsv(["luau", "map", "command",
             'local n=name; if n:sub(-6)=="_index" then return n:sub(1,#n-6) else return n end',
             f], tmp("mi1.csv"))
    f = qsv(["luau", "map", "has_index", '(name:sub(-6)=="_index") and 1 or 0', f], tmp("mi2.csv"))
    # One row per (command, version): prefer the index variant (has_index DESC), and among the
    # runs of the chosen variant take the LATEST (tstamp DESC) — not MAX(recs_per_sec), which on a
    # re-benchmarked release would report the fastest run rather than the current one.
    return qsv(["sqlp", f,
                "SELECT command, version, tstamp, recs_per_sec FROM ("
                "SELECT command, version, tstamp, recs_per_sec, ROW_NUMBER() OVER "
                "(PARTITION BY command, version ORDER BY has_index DESC, tstamp DESC) AS rn "
                "FROM _t_1) WHERE rn = 1 ORDER BY tstamp"], dst)


def prep_trend():
    f = merge_index(TREND_NAMES, tmp("trend_m.csv"))
    return qsv(["sort", "-s", "tstamp", f], tmp("trend.csv"))


def prep_heatmap(versions):
    f = merge_index(HEATMAP_NAMES, tmp("hm_m.csv"))
    f = qsv(["search", "-s", "version", alt(versions), f], tmp("hm1.csv"))
    # normalize each command to its own recent peak so between-command scale differences don't
    # wash out the per-command trajectory (the "relative to peak" story)
    return qsv(["sqlp", f,
                "SELECT command AS name, version, CAST(recs_per_sec AS DOUBLE) / "
                "MAX(CAST(recs_per_sec AS DOUBLE)) OVER (PARTITION BY command) AS rel FROM _t_1"],
               tmp("heatmap.csv"))


def prep_growth(names, dst):
    """Per variant, throughput normalized to its own first benchmarked release (=1.0), so the
    growth multiple is directly readable and unequal scales don't hide any series' trajectory.

    The historical data has occasional glitch rows — a benchmark that errored/short-circuited
    records an impossible recs_per_sec (e.g. frequency_sorted @ 6.0.1 = 52M/s, mean 18ms). On a
    normalized axis a single such spike flattens every real line, so drop physically-impossible
    rows: no non-`count` command approaches GROWTH_CEILING, and `count` is never in a growth set.
    Applied inside the WHERE so the =1.0 baseline is a real release, not a glitch.

    A release that was benchmarked more than once (the history has duplicate `(name, version)`
    rows) is collapsed to its LATEST run first, so the growth story is per-release, not per-run —
    the normalization and `last_rel()` see one point per version, not duplicated x-axis points."""
    f = qsv(["search", "-s", "name", alt(names), HISTORY], tmp("gr0.csv"))
    f = qsv(["sort", "-s", "tstamp", f], tmp("gr1.csv"))
    return qsv(["sqlp", f,
                "SELECT name, version, tstamp, CAST(recs_per_sec AS DOUBLE) / "
                "FIRST_VALUE(CAST(recs_per_sec AS DOUBLE)) OVER "
                "(PARTITION BY name ORDER BY tstamp) AS rel FROM ("
                "SELECT name, version, tstamp, recs_per_sec, ROW_NUMBER() OVER "
                "(PARTITION BY name, version ORDER BY tstamp DESC) AS rn FROM _t_1 "
                f"WHERE CAST(recs_per_sec AS DOUBLE) < {GROWTH_CEILING}) "
                "WHERE rn = 1 ORDER BY tstamp"], dst)


def prep_treemap():
    return qsv(["luau", "map", "family", 'name:match("^[^_]+")', LATEST], tmp("treemap.csv"))


def prep_gainers():
    f = qsv(["search", "-s", "delta (%)", r"^\d", LATEST], tmp("g0.csv"))  # positive deltas only
    f = qsv(["sort", "-s", "delta (%)", "-N", "-R", f], tmp("g1.csv"))
    return qsv(["slice", "--len", "15", f], tmp("gainers.csv"))


def prep_delta_box():
    f1 = qsv(["luau", "map", "family", 'name:match("^[^_]+")', LATEST], tmp("db0.csv"))
    f2 = qsv(["search", "-s", "family", alt(FAMILY_BOX), f1], tmp("db1.csv"))
    # drop rows with no delta or an out-of-range artifact delta
    f3 = qsv(["search", "-s", "delta (%)", r"^-?\d", f2], tmp("db2.csv"))
    return qsv(["luau", "filter",
                f'local d = tonumber(col["delta (%)"]); return d ~= nil and math.abs(d) <= {DELTA_CLAMP}',
                f3], tmp("delta_box.csv"))


# ---------------------------------------------------------------------------- assembly

PAGE_CSS = """
:root{--fg:#1f2733;--muted:#5b6673;--bg:#ffffff;--card:#fbfcfe;--border:#e6e9f0;--accent:#2a6df4}
*{box-sizing:border-box}
body{margin:0;font:15px/1.55 -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Helvetica,Arial,sans-serif;
color:var(--fg);background:var(--bg)}
header{max-width:1180px;margin:0 auto;padding:34px 22px 8px}
h1{margin:0 0 6px;font-size:26px}
.sub{color:var(--muted);margin:0 0 16px}
.meta{display:flex;flex-wrap:wrap;gap:8px 18px;font-size:13px;color:var(--muted);
border:1px solid var(--border);background:var(--card);border-radius:8px;padding:12px 16px}
.meta b{color:var(--fg);font-weight:600}
.caveat{max-width:1180px;margin:16px auto 0;padding:12px 16px;font-size:13.5px;color:#7a5b00;
background:#fff8e6;border:1px solid #f2e2ad;border-radius:8px}
main{max-width:1180px;margin:0 auto;padding:8px 22px 60px}
section{margin-top:30px}
h2{font-size:19px;margin:0 0 4px}
.desc{color:var(--muted);margin:0 0 12px;font-size:14px}
iframe.chart{width:100%;border:1px solid var(--border);border-radius:8px;height:420px;display:block;
background:var(--card)}
footer{max-width:1180px;margin:0 auto;padding:0 22px 50px;color:var(--muted);font-size:13px}
a{color:var(--accent)}
code{background:#eef1f6;border-radius:4px;padding:1px 5px;font-size:12.5px}
details.vizsrc{margin-top:10px;font-size:13px}
details.vizsrc summary{cursor:pointer;color:var(--accent);user-select:none;width:max-content}
details.vizsrc pre{overflow-x:auto;background:var(--card);border:1px solid var(--border);
border-radius:8px;padding:12px 14px;margin:10px 0 8px;font-size:12.5px;line-height:1.45;
font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace}
details.vizsrc pre code{background:none;padding:0;font-size:inherit}
details.vizsrc .srclink{font-size:12.5px}
"""


def viz_source_blocks(slugs):
    """Map each chart slug → (dedented_snippet, start_lineno, end_lineno) for its
    `figs.append(viz(...))` block in this file's OWN source — powering the per-chart "see the qsv
    viz command" shortcut (inline snippet + a GitHub #L range). A slug appears as the literal
    `"<slug>", "` (quoted, then comma-quote for the title) only at its `viz()` call site, never in
    prose, so it uniquely locates the block; from that anchor walk back to the `figs.append(viz(`
    opener and forward to the block's closing `))` line."""
    with open(os.path.abspath(__file__), encoding="utf-8") as fh:
        lines = fh.read().splitlines()
    blocks = {}
    for slug in slugs:
        needle = f'"{slug}", "'
        anchor = next((i for i, ln in enumerate(lines) if needle in ln), None)
        if anchor is None:
            continue
        start = anchor
        while start >= 0 and "figs.append(viz(" not in lines[start]:
            start -= 1
        end = anchor
        while end < len(lines) and not lines[end].rstrip().endswith("))"):
            end += 1
        if start < 0 or end >= len(lines):
            continue
        snippet = textwrap.dedent("\n".join(lines[start:end + 1]))
        blocks[slug] = (snippet, start + 1, end + 1)  # 1-based, inclusive
    return blocks


def build_index(figs, info):
    ver = info.get("version", "?")
    plat = info.get("platform", "?")
    cores = info.get("cores", "?")
    mem_bytes = info.get("mem", "")
    mem = f"{round(int(mem_bytes) / 2**30)} GiB" if mem_bytes.isdigit() else mem_bytes
    binfo = info.get("version_info", "")
    chip = ""
    m = re.search(r"(Apple [^-;)]+)", binfo)
    if m:
        chip = m.group(1).strip()

    parts = ["<!doctype html>", "<html lang=en><head><meta charset=utf-8>",
             "<meta name=viewport content='width=device-width,initial-scale=1'>",
             "<title>qsv benchmark dashboard</title>",
             f"<style>{PAGE_CSS}</style></head><body>"]
    parts.append("<header>")
    parts.append("<h1>qsv benchmark dashboard</h1>")
    parts.append(f"<p class=sub>Interactive charts of the qsv benchmark suite, rendered with "
                 f"<code>qsv viz</code> — a live showcase of the command charting its own performance.</p>")
    meta = [f"<span><b>qsv</b> {html_escape(ver)}</span>",
            f"<span><b>Platform</b> {html_escape(plat)}</span>",
            f"<span><b>CPU</b> {html_escape(chip or '?')} ({html_escape(cores)} cores)</span>",
            f"<span><b>RAM</b> {html_escape(mem)}</span>",
            "<span><b>Dataset</b> NYC 311 — 1M rows × 41 cols (520 MB)</span>"]
    parts.append("<div class=meta>" + "".join(meta) + "</div>")
    parts.append("</header>")
    parts.append(
        "<div class=caveat><b>Read me first.</b> Throughput is records/sec = 1,000,000 rows ÷ mean "
        "of 3 runs (2 warmups) via hyperfine. Cross-version numbers are <b>not strictly "
        "apples-to-apples</b> — commands gain features over time, so a command's benchmark in an "
        "old release may do less work than today's. Read the historical charts (the trend, the "
        "flagship deep-dives and the heatmap) as broad trajectory, not exact speedup. "
        "<code>count</code> gets its own panel below (not the shared-scale charts) because with an "
        "index it just reads a stored row count (tens of millions of records/sec) and would flatten "
        "every other bar.</div>")
    src_blocks = viz_source_blocks([f["slug"] for f in figs])
    parts.append("<main>")
    for f in figs:
        parts.append("<section>")
        parts.append(f"<h2>{html_escape(f['title'])}</h2>")
        parts.append(f"<p class=desc>{html_escape(f['desc'])}</p>")
        parts.append(f"<iframe class=chart loading=lazy scrolling=no src='{f['slug']}.html'></iframe>")
        blk = src_blocks.get(f["slug"])
        if blk:
            snippet, start, end = blk
            parts.append(
                "<details class=vizsrc><summary>see the qsv viz command</summary>"
                f"<pre><code>{html_escape(snippet)}</code></pre>"
                f"<a class=srclink href='{SOURCE_URL}#L{start}-L{end}' target=_blank rel=noopener>"
                f"view on GitHub ↗ (gen_benchmark_viz.py, lines {start}–{end})</a></details>")
        parts.append("</section>")
    parts.append("</main>")
    parts.append(
        "<footer>Regenerated by <code>scripts/gen_benchmark_viz.py</code> from "
        "<code>scripts/results/*.csv</code>. Charts are interactive — hover for values, drag to zoom, "
        "use the toolbar to pan or download a PNG. Under each chart, "
        "<b>see the qsv viz command</b> reveals the exact <code>qsv viz</code> call that produced it. "
        f"Source benchmarks: <a href='https://github.com/dathere/qsv/blob/master/scripts/results/'>"
        "scripts/results</a>.</footer>")
    parts.append(RESIZE_LISTENER_JS)
    parts.append("</body></html>")
    with open(os.path.join(OUT, "index.html"), "w", encoding="utf-8") as fh:
        fh.write("\n".join(parts))


WIKI_MD = """\
# Benchmarks

qsv's benchmark suite runs {total} benchmarks on a **1,000,000-row × 41-column, 520 MB**
sample of NYC's 311 data, timed with [hyperfine](https://github.com/sharkdp/hyperfine)
(2 warmups + 3 timed runs each). See
[`scripts/results/README.md`](https://github.com/dathere/qsv/blob/master/scripts/results/README.md)
for the methodology and the raw CSVs.

> Looking for the **full per-command timing tables**? See the classic
> [tabular benchmarks at qsv.dathere.com](https://qsv.dathere.com/benchmarks). This page is the
> interactive, visual companion to it.

## 📊 Interactive dashboard

**➡️ [Open the interactive benchmark dashboard]({url})**

The dashboard is rendered by qsv's own `viz` command and is fully interactive (hover for
values, zoom, download PNGs). It covers the with/without-index advantage, the `sqlp`
schema-cache knob, throughput across every release, deep-dives into the two flagship
commands (`stats` and `frequency`) showing they grew features without losing speed, and
this release's biggest speedups.

[![qsv benchmark dashboard]({url}hero.png)]({url})

> **Note:** cross-version numbers are not strictly apples-to-apples — commands gain features
> over time. Treat the historical trend as a broad trajectory, not an exact speedup.

_Last generated for qsv {version} ({platform}). Regenerate with
`python3 scripts/gen_benchmark_viz.py` and copy this page's link — the dashboard itself
redeploys to GitHub Pages automatically on push to `master`._
"""


def write_wiki_stub(info, total):
    md = WIKI_MD.format(url=PAGES_URL, version=info.get("version", "?"),
                        platform=info.get("platform", "?"), total=total)
    with open(os.path.join(OUT, "Benchmarks.wiki.md"), "w", encoding="utf-8") as fh:
        fh.write(md)


def render_hero(index_src):
    """Best-effort static PNG (benchmarks/hero.png) for the Wiki thumbnail — the index-advantage
    chart. Needs the `viz_static` feature + a local Chrome/Firefox (webdriver auto-downloads), so
    it is skipped gracefully if unavailable (the interactive dashboard never needs it)."""
    out = os.path.join(OUT, "hero.png")
    env = {**os.environ, "QSV_PROGRESSBAR": "0"}
    try:
        subprocess.run([QSV, "viz", "bar", index_src, "--x", "command", "--y", "recs_per_sec",
                        "--series", "index_status", "--width", "1200", "--height", "600",
                        "--title", "The index advantage — qsv throughput with vs without an index",
                        "--y-title", "records/sec", "-o", out],
                       check=True, capture_output=True, text=True, env=env)
        print("Wrote hero.png (static PNG for the Wiki)")
    except subprocess.CalledProcessError as e:
        print(f"NOTE: skipped hero.png (viz_static/browser unavailable): "
              f"{(e.stderr or '').strip().splitlines()[-1] if e.stderr else e}")


def count_rows(path):
    with open(path, encoding="utf-8") as fh:
        return sum(1 for _ in fh) - 1


def last_rel(csv_path, name):
    """Final (latest-release) `rel` value for a series in a prep_growth output — i.e. its total
    growth multiple vs its first benchmarked release. Lets the prose cite exact, never-stale
    multiples computed from the data being charted."""
    val = None
    with open(csv_path, encoding="utf-8") as fh:
        for row in csv.DictReader(fh):
            if row["name"] == name:
                val = float(row["rel"])
    return val


def main():
    os.makedirs(OUT, exist_ok=True)
    info = read_run_info()
    all_vers = recent_versions(10_000)          # newest-first, every release in the history
    first_release, latest_release = all_vers[-1], all_vers[0]
    n_releases = len(all_vers)
    hm_versions = recent_versions(HEATMAP_VERSIONS)
    total = count_rows(LATEST)

    figs = []
    index_src = prep_index()
    figs.append(viz("bar", index_src,
                    ["--x", "command", "--y", "recs_per_sec", "--series", "index_status",
                     "--title", "The index advantage", "--y-title", "records/sec"],
                    "index_advantage", "The index advantage",
                    "Build an index once and qsv can skip the opening scan on every run after. "
                    "The payoff is lopsided — a ~6x jump for stats and ~3x for search, but next "
                    "to nothing for streaming commands — so only the standouts are shown here."))
    figs.append(viz("bar", prep_count(),
                    ["--x", "index_status", "--y", "recs_per_sec",
                     "--title", "count: the index-read superpower", "--y-title", "records/sec"],
                    "count_callout", "count with an index is effectively instant",
                    "The index advantage at its extreme. With an index, count doesn't scan at all — "
                    "it reads a row count qsv already stored, a ~10x jump from ~9M to ~90M rows/sec. "
                    "It sits on its own axis precisely because that number would flatten every "
                    "other bar on the page."))
    sp_src, sp_rows = prep_superpowers()
    sp_top_cmd, sp_top_x = (sp_rows[0] if sp_rows else ("schema", 0.0))
    figs.append(viz("bar", sp_src,
                    ["--x", "command", "--y", "speedup",
                     "--title", "The index superpowers — times faster with an index",
                     "--y-title", "× faster with an index"],
                    "index_superpowers", "The index superpowers",
                    "Some commands don't just skip the opening scan with an index — they skip almost "
                    "all the work. slice and sample seek straight to the rows they need; schema reuses "
                    f"cached statistics. That's a different order of magnitude: {sp_top_cmd} runs "
                    f"{sp_top_x:.0f}x faster, and slice and sample tens of times over — versus the "
                    "low single digits for the scan-skippers above."))
    figs.append(viz("bar", prep_sqlp(),
                    ["--x", "name", "--y", "recs_per_sec",
                     "--title", "sqlp tuning: the Polars schema-cache knob",
                     "--y-title", "records/sec"],
                    "sqlp_tuning", "sqlp tuning: the schema-cache knob",
                    "sqlp infers a schema before it runs. Cache that schema and the re-inference "
                    "cost disappears on the next query — worth about a third more throughput on "
                    "these aggregations, for a one-line option."))
    figs.append(viz("line", prep_trend(),
                    ["--x", "version", "--y", "recs_per_sec", "--series", "command",
                     "--title", f"Throughput across every release ({first_release} → {latest_release})",
                     "--y-title", "records/sec"],
                    "trend", "The long view — every release",
                    f"Records/sec for the marquee commands across all {n_releases} releases since "
                    f"{first_release}. Each line follows the fastest path available at the time: the "
                    "plain scan early on, then the indexed variant once search and searchset learned "
                    "to use an index at 10.0.0 — the visible step up. Broad trajectory only (see the "
                    "note above); count is omitted for scale."))
    stats_src = prep_growth(STATS_GROWTH, tmp("stats_growth.csv"))
    freq_src = prep_growth(FREQ_GROWTH, tmp("freq_growth.csv"))
    s_base, s_heavy = last_rel(stats_src, "stats"), last_rel(stats_src, "stats_everything_index")
    f_base, f_idx = last_rel(freq_src, "frequency"), last_rel(freq_src, "frequency_index")
    figs.append(viz("line", stats_src,
                    ["--x", "version", "--y", "rel", "--series", "name",
                     "--title", "Flagship deep-dive: stats got richer AND faster",
                     "--y-title", "speed vs first release (1.0 = launch)"],
                    "stats_growth", "Flagship deep-dive: stats",
                    "qsv's most-used command, indexed to its own launch speed. Over the years stats "
                    "kept adding statistics — cardinality, quartiles, MAD, skewness — yet never got "
                    f"slower: plain stats now runs {s_base:.1f}x its first-release speed, and the "
                    f"full --everything pass with an index {s_heavy:.1f}x. Features grew; the curve "
                    "still points up. (The odd single-release dip is a failed benchmark run, not a "
                    "regression.)"))
    figs.append(viz("line", freq_src,
                    ["--x", "version", "--y", "rel", "--series", "name",
                     "--title", "Flagship deep-dive: frequency held its ground",
                     "--y-title", "speed vs first release (1.0 = launch)"],
                    "freq_growth", "Flagship deep-dive: frequency",
                    "The same story for qsv's second flagship. As frequency gained sorted, "
                    "case-insensitive and unlimited modes, throughput climbed rather than eroded — "
                    f"base frequency is now {f_base:.1f}x its launch speed and the indexed run "
                    f"{f_idx:.1f}x. Newer modes join partway, each measured from its own debut; the "
                    "transient dips are measurement artifacts, not regressions."))
    figs.append(viz("heatmap", prep_heatmap(hm_versions),
                    ["--x", "version", "--y", "name", "--z", "rel",
                     "--title", "Relative throughput vs each command's recent peak"],
                    "heatmap", "Relative throughput heatmap",
                    "The indexed marquee commands over the recent window, each row normalized to its "
                    "own peak (1.0 = that command's fastest release). Normalizing per row lets a "
                    "90M-rows/sec count and a 600k-rows/sec frequency share one canvas — the colour "
                    "shows trajectory, not absolute speed."))
    figs.append(viz("treemap", prep_treemap(),
                    ["--cols", "family,name", "--value", "mean", "--agg", "sum",
                     "--title", "Where the suite spends time (mean run time)"],
                    "time_spent", "Where the suite spends its time",
                    "The whole suite by wall-clock, family then benchmark. The biggest tiles are "
                    "where a speedup would move the needle most — a map of where the optimization "
                    "effort is best spent."))
    figs.append(viz("scatter", prep_gainers(),
                    ["--x", "name", "--y", "delta (%)", "--size", "delta (%)", "--color", "delta (%)",
                     "--title", "Biggest speedups this release", "--y-title", "% faster vs previous version"],
                    "gainers", "Biggest speedups this release",
                    "The 15 benchmarks that improved most over the previous release. Bigger, brighter "
                    "bubbles are larger wins — the percentage cut in mean run time from one version "
                    "to the next."))
    figs.append(viz("box", prep_delta_box(),
                    ["--y", "delta (%)", "--x", "family",
                     "--title", "Release-over-release change by command family",
                     "--y-title", "% faster vs previous version"],
                    "change_by_family", "Change distribution by family",
                    "The wider view behind the speedups: the spread of per-release change within each "
                    "family (above zero = faster). Most families cluster just north of zero — steady, "
                    f"unglamorous progress. Extreme outliers (|Δ| > {int(DELTA_CLAMP)}%, usually "
                    "measurement noise) are omitted."))

    build_index(figs, info)
    render_hero(index_src)
    write_wiki_stub(info, total)
    shutil.rmtree(TMP, ignore_errors=True)
    print(f"Wrote {len(figs)} charts + index.html to {OUT}")
    print(f"Preview: (cd {OUT} && python3 -m http.server 8000) then open http://localhost:8000/")


if __name__ == "__main__":
    main()
