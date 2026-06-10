# stats.rs Performance Work (June 2026, branch perf/stats-hotpath)

## Branch state
- qsv branch `perf/stats-hotpath` (4 commits on top of master): ByteRecord reuse + capacity hints (8503dfd75), Frequencies modes tracker (7eaf358b8), unindexed pipeline + parallel column merge (f7473a92b), help regen (31f2236b6).
- qsv-stats branch `perf/counted-runs-modes` (0cc26a2): `Frequencies::modes_antimodes` (shared `modes_antimodes_from_runs` core extracted from unsorted.rs), `add_borrowed_capped`, serde + manual PartialEq, hashbrown "serde" feature. Property test proves Unsorted/Frequencies modes parity.
- **RELEASED 2026-06-10**: qsv-stats 0.54.0 published to crates.io (tag 0.54.0, master pushed, commit d4458a9). qsv branch un-patched (d8cd50b15): dep = "0.54", path patch removed, verified vs published crate (stats 792 + moarstats 88 + pragmastat 47 tests). Cargo.lock has BOTH qsv-stats 0.53.0 (via csvs_convert git fork - harmless, no shared types; bump the fork later) and 0.54.0. qsv branch NOT yet pushed/PR'd.

## Results (NYC 311 1M x 41, M-series, release build, vs v21.0.0 baseline)
- default unindexed: 1.452s -> 0.870s (-40%, pipeline) | indexed: 138.9 -> 129.3ms (-7%)
- --everything unindexed: 2.281 -> 2.117s | indexed: 730.7 -> **425.0ms (-42%)**
- --infer-dates unindexed: 4.291 -> 3.582s (-17%) | indexed: 569 -> 552ms
- peak RSS (-E indexed): 2.39GB -> 1.37GB (-43%)

## Key learnings
- csv fork's ByteRecords iterator does `clone_truncated()` per row; `read_byte_record` into reused buffer eliminates 2 allocs+copy/row. After `seek()`, `seeked=true` bypasses header skip — direct reads match iterator semantics.
- `RECORD_COUNT` OnceLock is set AFTER compute, so Stats::new's with_capacity always used the 10k fallback; `repeat_n(Stats::new())` clones discarded capacity anyway (Vec::clone allocs len, not capacity).
- Indexed-parallel sortiness was nondeterministic (completion-order `merge_all(recv.iter())`). Now: incremental chunk-ORDER merge -> deterministic, == sequential. Collect-then-merge inflates RSS (all chunks resident); merge incrementally with a pending HashMap. Frequencies-map merges serialize on the main thread -> par_iter_mut over COLUMNS fixed a +23% everything_idx regression (became -42%).
- `--mode-cardinality-cap` semantics changed (user-approved): unweighted now fires on UNIQUE values (was total samples), matching weighted. cap=0 default unaffected.
- idx vs noidx fp differences in geometric_mean/variance (Chan merge vs Welford) are pre-existing/inherent; sortiness no longer differs.
- Benchmark harness in /tmp/qsv-bench (capture.sh, bench.sh, compare.py + ref-* output dirs, NYC 311 CSV) — regenerate if cleaned.
- Deferred ideas: multi-worker unindexed compute (changes fp results), boxing TypedMinMax (rejected, hot), weighted_modes entry_ref micro-opt (Step 5, not done).
