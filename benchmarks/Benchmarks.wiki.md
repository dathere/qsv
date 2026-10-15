# Benchmarks

qsv's benchmark suite runs 305 benchmarks on a **1,000,000-row × 41-column, 520 MB**
sample of NYC's 311 data, timed with [hyperfine](https://github.com/sharkdp/hyperfine)
(2 warmups + 3 timed runs each). See
[`scripts/results/README.md`](https://github.com/dathere/qsv/blob/master/scripts/results/README.md)
for the methodology and the raw CSVs.

> [!NOTE]
> **25.0.0 — PGO is back.** The Apple Silicon prebuilt is profile-guided again ([#4757](https://github.com/dathere/qsv/pull/4757)),
> and the median benchmark held level with 24.0.0. `sortcheck_unsorted` is back to ~11 ms, and
> `split_chunks_index_j1` and `stats_index` are back in their 23.0.1 range. `extdedup` and
> `snappy_compress` did not recover. They are still 19-31% slower than on 23.0.1. The big wins:
> `writestat` is 35-50x faster ([#4747](https://github.com/dathere/qsv/pull/4747), which takes column types from the stats cache
> instead of Polars inference). Polars 2.0 ([#4760](https://github.com/dathere/qsv/pull/4760)) makes `pivotp` ~5x, `joinp` ~2.6x
> and plain `sqlp` ~2x faster. CSV → GeoJSON `geoconvert` is ~3x faster now that it streams
> ([#4800](https://github.com/dathere/qsv/pull/4800)), and `template` is 21% faster on MiniJinja 3.0 ([#4785](https://github.com/dathere/qsv/pull/4785)). On the
> other side, `count` and `flatten` slowed 13-17%. The 19 benchmarks new in suite 9.9.0 have no
> earlier baseline, so they don't appear in the change charts.
>
> **Don't read the `sqlp_format_*`, `sqlp_lowmemory`, `sqlp_nooptimizations` or
> `sqlp_tryparsedates*` numbers:** since 16.0.0 the benchmark machine has timed failed runs of
> them (~0.06-0.47 s, while plain `sqlp` on the same query takes 0.8-1.6 s). This is under
> investigation.

> Looking for the **full per-command timing tables**? See the classic
> [tabular benchmarks at qsv.dathere.com](https://qsv.dathere.com/benchmarks). This page is the
> interactive, visual companion to it.

## 📊 Interactive Data Schematic

**➡️ [Open the interactive benchmark Data Schematic](https://dathere.github.io/qsv/benchmarks/)**

The Data Schematic is rendered by qsv's own `viz` command and is fully interactive (hover for
values, zoom, download PNGs). It covers the with/without-index advantage, the `sqlp`
schema-cache knob, throughput across every release, deep-dives into four flagship
commands (`stats`, `frequency`, `validate` and `moarstats`) showing they grew features
without losing speed, and this release's biggest speedups.

[![qsv benchmark Data Schematic](https://dathere.github.io/qsv/benchmarks/hero.png)](https://dathere.github.io/qsv/benchmarks/)

> **Note:** cross-version numbers are not strictly apples-to-apples — commands gain features
> over time. Treat the historical trend as a broad trajectory, not an exact speedup.

_Last generated for qsv 25.0.0 (aarch64-apple-darwin). Regenerate with
`python3 scripts/gen_benchmark_viz.py` and copy this page's link — the Data Schematic itself
redeploys to GitHub Pages automatically on push to `master`._
