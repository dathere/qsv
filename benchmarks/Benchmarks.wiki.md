# Benchmarks

qsv's benchmark suite runs 259 benchmarks on a **1,000,000-row × 41-column, 520 MB**
sample of NYC's 311 data, timed with [hyperfine](https://github.com/sharkdp/hyperfine)
(2 warmups + 3 timed runs each). See
[`scripts/results/README.md`](https://github.com/dathere/qsv/blob/master/scripts/results/README.md)
for the methodology and the raw CSVs.

> Looking for the **full per-command timing tables**? See the classic
> [tabular benchmarks at qsv.dathere.com](https://qsv.dathere.com/benchmarks). This page is the
> interactive, visual companion to it.

## 📊 Interactive dashboard

**➡️ [Open the interactive benchmark dashboard](https://dathere.github.io/qsv/benchmarks/)**

The dashboard is rendered by qsv's own `viz` command and is fully interactive (hover for
values, zoom, download PNGs). It covers the with/without-index advantage, the `sqlp`
schema-cache knob, throughput across every release, deep-dives into four flagship
commands (`stats`, `frequency`, `validate` and `moarstats`) showing they grew features
without losing speed, and this release's biggest speedups.

[![qsv benchmark dashboard](https://dathere.github.io/qsv/benchmarks/hero.png)](https://dathere.github.io/qsv/benchmarks/)

> **Note:** cross-version numbers are not strictly apples-to-apples — commands gain features
> over time. Treat the historical trend as a broad trajectory, not an exact speedup.

_Last generated for qsv 21.1.0 (aarch64-apple-darwin). Regenerate with
`python3 scripts/gen_benchmark_viz.py` and copy this page's link — the dashboard itself
redeploys to GitHub Pages automatically on push to `master`._
