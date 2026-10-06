# Benchmarks

qsv's benchmark suite runs 286 benchmarks on a **1,000,000-row × 41-column, 520 MB**
sample of NYC's 311 data, timed with [hyperfine](https://github.com/sharkdp/hyperfine)
(2 warmups + 3 timed runs each). See
[`scripts/results/README.md`](https://github.com/dathere/qsv/blob/master/scripts/results/README.md)
for the methodology and the raw CSVs.

> [!NOTE]
> **24.0.0 — built without PGO.** The Apple Silicon prebuilt benchmarked here skipped
> profile-guided optimization ([#4740](https://github.com/dathere/qsv/issues/4740)); 21.1.0-23.0.1 were PGO builds. Even so,
> the median benchmark held level with 23.0.1. The few that slowed by 16-35% —
> `split_chunks_index_j1`, `extdedup`, `snappy_compress` and `stats_index` — fell back to their
> pre-PGO (21.0.0 and earlier) range, consistent with the lost PGO. `sortcheck_unsorted` is the
> exception: it held at ~10 ms through non-PGO and PGO releases alike and is ~19 ms here, so PGO
> doesn't explain it. Meanwhile 23.0.1's un-indexed regression is fixed
> ([#4603](https://github.com/dathere/qsv/issues/4603)): un-indexed `count` is ~6x faster than on 23.0.1, and frequency,
> searchset and extsort cut 39-57% off their 23.0.1 run times. Those recoveries, plus validate's
> ~2x faster plain scan, top this release's speedups.

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

_Last generated for qsv 24.0.0 (aarch64-apple-darwin). Regenerate with
`python3 scripts/gen_benchmark_viz.py` and copy this page's link — the Data Schematic itself
redeploys to GitHub Pages automatically on push to `master`._
