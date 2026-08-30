# `qsv stats` hot-loop perf investigation — measured findings

Scope: `src/cmd/stats.rs` + `qsv-stats` 0.55. Method: profile-first (see the approved plan).
Machine: **Apple M4 Max, 16 cores, 68 GB**. Fixture: `NYC_311_SR_2010-2020-sample-1M.csv`
(538 MB, 1,000,000 rows x 41 cols) and its `data_sorted.csv` / `data_unsorted.csv` twins.

## MEASURED denominator (hyperfine, min-runs 5, no `-i`, cache cleared between runs)

Binary: `target/release-samply/qsv` 22.0.1, aarch64-apple-darwin, **Apple M4 Max (16 cores)**,
Rust 1.98. ⚠️ `release-samply` is `panic="unwind"` + debug info, *not* the shipping
`panic="abort"` profile — fine for A/B, so don't quote these as "qsv stats takes X".

⚠️ **An index exists** (`NYC_311_...csv.idx`), so every "default" run below takes the **indexed
parallel** path (~1173% CPU). The plan's 870 ms figure was the *unindexed* number and was the
wrong denominator.

| shape | `-j 1` | default (indexed, parallel) |
|---|---:|---:|
| plain | 1.147 s ± 0.006 | **115.0 ms** ± 4.5 |
| `--everything` | 2.528 s ± 0.072 | **349.7 ms** ± 7.1 |
| `--everything --infer-dates` | — | **603.9 ms** ± 4.4 |

(115 ms matches the June-2026 indexed 129 ms; 350 ms beats June's indexed `--everything` 425 ms.)

## MEASURED profile (samply 1000-4000 Hz, `--unstable-presymbolicate`, self-time)

`-j 1` isolates the per-record loop; default shows where users actually are.

| symbol | plain -j1 | plain def | everything -j1 | everything def | dates -j1 |
|---|---:|---:|---:|---:|---:|
| `csv_core::Reader::read_record` | 29.5% | 25.2% | 14.3% | 6.7% | 8.1% |
| `stats::Args::{sequential,parallel}_stats` | 20.0% | 19.3% | 30.9% | 13.5% | 18.3% |
| **`TypedMinMax::add_with_parsed`** | **18.0%** | **16.7%** | **13.1%** | **5.8%** | **6.8%** |
| **`_platform_memcmp`** | **11.1%** | **9.0%** | **13.9%** | **10.8%** | **9.4%** |
| **`_platform_memmove`** | **7.6%** | **7.0%** | **5.3%** | **3.2%** | **3.1%** |
| `hashbrown ...::entry` | - | - | - | 14.4% | - |
| `hashbrown ...::reserve_rehash` | - | - | 2.9% | 7.0% | 1.4% |
| `rayon sort recurse::<Partial<f64>>` | - | - | **1.8%** | - | - |
| chrono `StrftimeItems::next` | - | - | - | - | 13.4% |
| chrono `format::parse::parse` | - | - | - | - | 5.8% |
| `__psynch_cvwait` (thread idle) | - | 5.8% | - | 11.6% | 4.1% |

**H1 cluster** (`add_with_parsed` + memcmp + memmove) = **36.7%** plain -j1, **32.7%** plain
default, **32.3%** everything -j1. It is the dominant application-level cost.



## Structural baseline (verified first-hand, not assumed)

The hot loop is already heavily tuned; there is no obvious waste:

- `Args::add_row` (`stats.rs:3095`) dispatches via `stats.get_unchecked_mut(i)` over a
  `Selection::select(row)` `Scan` — no bounds check, no `zip`, no per-row allocation.
- Fields are `&[u8]` borrowed straight from the `ByteRecord` buffer: **no `from_utf8`, no
  `to_string`, no `format!` anywhere in the loop.**
- `compute` reuses one `ByteRecord`; `compute_pipelined` (the default) recycles 1024-row
  batches through a return channel — zero steady-state allocation.
- `FieldType::from_sample` (`stats.rs:6457`) short-circuits once a column is `TString` and
  threads parsed `i64`/`f64` downstream, so nothing is parsed twice.
  `atoi_simd` / `fast_float2` / `simdutf8` already in use.
- `Frequencies::add_borrowed_capped` uses hashbrown `entry_ref` — one probe, allocation only
  on a newly admitted key.
- Allocator (jemalloc), release profile (LTO, `codegen-units=1`), hasher (foldhash) all tuned.

## H1 — `MinMax<Vec<u8>>::add_bytes`, the one unconditional per-field cost

`TypedMinMax::add_with_parsed` (`stats.rs:6717`) calls `self.strings.add_bytes(sample)` for
**every non-empty field of every row, regardless of column type**. Steady state
(`qsv-stats/src/minmax.rs:295`, `len >= 2` arm) does per call:
1 memcmp vs `last_value`, 1 conditional memcmp vs `min`/`max`, then `clear()` +
`extend_from_slice` — a memcpy — into `last_value`.

`minmax` is gated on `which.range`, which is **on by default**, so this is a cost of the
*plain* run, not just `--everything`.

**Measured scale** (200k-row sample, extrapolated to 1M; empty fields excluded because
`add_with_parsed` early-returns on `sample_len == 0`):

| | per 1M-row run |
|---|---|
| `add_bytes` calls | **28.1 M** (~28/row; 32% of all fields are empty and never reach it) |
| bytes memcpy'd | **496 MB** |
| avg non-empty field length | 17.7 bytes |

It is **deliberate and correct** — a column can widen to `TString` mid-stream, so lexical
min/max, `min_length`/`max_length`, `sort_order` and `sortiness` must cover earlier rows.
Verified: `show()` reads `self.strings` **only** for `TString` (`stats.rs:6787`); numeric
columns take sortiness from `self.integers`/`self.floats` (`:6843`, `:6866`). So for a column
that ends numeric, all of this work is discarded — but it cannot be skipped soundly, because
whether a column widens is not knowable mid-stream. Do **not** "fix" it by gating on type.

### The mechanism the profile revealed (which structural reading missed)

`_platform_memcmp` (9-14%) and `_platform_memmove` (3-8%) are **out-of-line libc calls made on
~17.7-byte buffers**. At that size the *call* rivals the work. `add_bytes` is
`#[inline(always)]`, so it fused into `add_with_parsed` — but its slice `>=` lowered to a
`memcmp` call and its `extend_from_slice` to a `memmove` call, and those did not inline.

### H1c — short-circuit the libc calls — ⛔ TESTED, -7.6% of `add_bytes` ~= 2% wall. REJECTED.

> **Verdict first:** this was the leading candidate coming out of the profile and it **did not
> survive its microbenchmark** (see [the gate](#microbenchmark-gate--results-this-is-where-four-hypotheses-died)
> below). The reasoning in this section is preserved because the *mechanism* is real and the
> patch is correct — it is simply too small to ship. Do not revive it on the strength of the
> profile numbers alone.

**Lexicographic order of two byte slices is decided by the first differing byte.** So an
inlined first-byte check before the generic compare removes the `memcmp` call for those
comparisons — measured at **50.2%** of them, not the "large majority" first assumed — and it is
*exactly* output-identical, being a pure short-circuit of a comparison, not a change to it:

```rust
// last is non-empty whenever len >= 2; sample is non-empty (add_with_parsed early-returns on 0)
let (a0, b0) = (sample[0], last[0]);
let ord = if a0 != b0 { a0.cmp(&b0) } else { sample.cmp(last.as_slice()) };
```

Same trick applies to the `min`/`max` comparisons. Pair it with a short-length inline copy path
for `last_value` to attack the `memmove` side. **H1a rides along free**: compute `ord` once,
and on `Ordering::Equal` skip both the copy *and* the `max` comparison (`max >= last == sample`,
so it cannot be exceeded).

**Ceiling — verified by caller attribution, not assumed.** Inverted-stack breakdown of the
libc leaves (this was a real risk of over-attribution; it was checked):

| profile | `memcmp` from `add_with_parsed` | `memmove` from `add_with_parsed` | H1c-addressable |
|---|---:|---:|---:|
| plain -j1 | **97.8%** of memcmp = 10.86% of profile | **97.4%** of memmove = 7.41% | **18.3%** |
| everything def | 59.3% of memcmp = 6.42% of profile | (rest: hashbrown key-eq 2.92%, mode-sort 1.43%) | ~8% |

So on the **plain** path — the most common invocation — essentially *all* memcmp/memmove is
`add_bytes`, and the addressable share is **18.3%**. On `everything --default` ~40% of memcmp
belongs to hashbrown key-equality and mode sorting instead, so the ceiling there is about half
as good.

⚠️ **The projection built on that ceiling ("remove half => ~9%") was WRONG.** It rested on an
unverified assumption — that libc *call overhead*, not byte movement, dominates at ~17.7 bytes.
The microbenchmark below shows Apple's `_platform_memcmp` is already short-length-tuned, so
removing the calls wins only **-7.6% of `add_bytes`**, i.e. ~2% of wall time. **The lesson is
the same one the 11.1 ms sort estimate taught: a plausible per-unit cost is not a result until
it is measured.** The assumption was flagged as load-bearing before testing, and testing killed
it — which is exactly why it was flagged.

### H1a (consecutive-duplicate memcpy skip) — subsumed into H1c, not worth shipping alone

Measured `Ordering::Equal` rate: 15.5% unsorted / 29.9% sorted (predictor is **run-length, not
cardinality**). Against the real profile that is ~1.2% of memmove + ~0.9% of memcmp ≈ **~2%**
of the plain -j1 run. Too small to justify a qsv-stats release by itself; free as part of H1c.
(An earlier draft called this "refuted at 0.24%" using byte volume where call count was the
right unit — that understated it by ~10x.)

### H1b (inline small buffer) — DEPRIORITIZED

The pre-registered discriminator was "only if memory-stall-bound, not merely high self-time."
The profile shows the cost is in `memcmp`/`memmove` **call** frames, not in stalls attributable
to chasing `last`/`min`/`max` pointers. That points at H1c (remove the calls), not at H1b
(relocate the buffers). Growing `Stats` for no measured stall would be speculative.

## H3 — the `--everything` sort — ⛔ REFUTED BY MEASUREMENT. Drop it.

`rayon::slice::sort::recurse::<stats::Partial<f64>, ...>` is **1.77%** of `everything -j1` —
not "the dominant single cost". The 11.1 ms/1M-row-numeric-column figure was real *per numeric
column*, but NYC 311 has 41 columns of which only a handful are numeric, so it never became a
qsv-level cost. A `select_nth_unstable` rewrite has a ceiling of a fraction of 1.77%, against a
**real correctness risk**: `Unsorted<T>` stores `Partial<T>`, whose `cmp` falls back to
`Ordering::Less` on `None` and is documented as *not a valid total order*, so selection and
sort are not equivalent (NaN is the trigger — `OnlineStats::add_f64` filters NaN,
`Unsorted::add` does not). **Bad risk/reward. Do not pursue.**

The pre-checks that made this a live hypothesis (t-digest not shadowing it under `--everything`)
were correct; the hypothesis simply did not survive its denominator. `/tmp/nanfix.csv` was
built for a test that is no longer needed — the correct outcome of measuring first.

## H4 — `--infer-dates` — CONFIRMED, FILED UPSTREAM as dathere/qsv-dateparser#11

chrono/`qsv_dateparser` is ~25% of the dates run. `StrftimeItems` alone is **19.17% inclusive**,
and caller attribution is unambiguous — **100%** of those samples come from
`Utc::datetime_from_str` <- `qsv_dateparser::datetime::Parse`.

**Mechanism:** all 21 `parse_from_str` call sites in `qsv-dateparser/src/datetime.rs` pass
*string literal* formats, and chrono re-parses the format through `StrftimeItems` on every
call. Worse, the `.or_else` chains try formats in order: `slash_mdy_hms` walks 5 formats for a
4-digit year, so `MM/DD/YYYY hh:mm:ss AM/PM` (the NYC 311 shape) burns **3 guaranteed-to-fail
format parses before the 4th succeeds** — 4 `StrftimeItems` walks per value per row.

**Measured fix** (200,000 real `Created Date` values, chrono 0.4, min-of-5; outputs asserted
identical value-by-value):

| | time | per value |
|---|---:|---:|
| current (`parse_from_str`, format re-parsed per call) | 67.97 ms | 340 ns |
| pre-compiled `Vec<Item>` (format parsed once) | 35.95 ms | 180 ns |
| **delta** | **-47.1%** | |

Filed with full evidence + a suggested `LazyLock<Vec<Item<'static>>>` patch shape:
**https://github.com/dathere/qsv-dateparser/issues/11**. Fixing it upstream and bumping the dep
is the one change from this whole investigation that lands a real win in qsv, and it costs qsv
nothing but a version bump.

## H5 (NEW, from the profile) — hashbrown rehashing on the parallel path

Only visible in the default/indexed profile, and large: `HashMap::entry` 14.4% +
`reserve_rehash` **7.0%** + `insert` 2.8% + `foldhash::hash_bytes_long` 1.9% ≈ **26%** of
`everything --default`. `reserve_rehash` at 7% means the `Frequencies` map is **outgrowing its
initial capacity and rehashing**. **Mechanism confirmed (the first guess was wrong).** The indexed-parallel path *does* size
each worker correctly to its chunk — `stats.rs:2916` passes `chunk_size` as `expected_rows`,
and it is neither a chunk-accounting bug nor clamp saturation (1M/16 => chunk 62,500, so
`(62_500/10).clamp(16, 65_536)` = 6,250, well under the 65,536 cap).

The real cause is the **10%-cardinality heuristic** at `stats.rs:4877-4879`. NYC 311 has
several ~100%-unique columns (`Unique Key`, `Created Date`, addresses). For those the map must
reach ~62,500 entries but starts at 6,250 — about **3.3 rehash doublings per chunk per column**,
every run. That is the 7.0% `reserve_rehash`.

**Cheap, local to `src/cmd/stats.rs`, no qsv-stats release.** But note the trade-off the
heuristic exists to protect: hashbrown's `with_capacity` eagerly allocates its bucket array, so
raising it costs RSS — and **peak RSS is a tracked metric here** (June's -43%). Any fix must
A/B **RSS alongside wall time**. A cardinality-aware size from the stats cache (when present)
would beat a blanket raise.

## MICROBENCHMARK GATE — results (this is where four hypotheses died)

qsv-stats-scale benchmarks build in ~3 s vs a ~1h44m LTO rebuild of qsv, so every candidate was
gated here first. Both harnesses **replay the real workload**: 5,611,870 actual non-empty field
values from `data_unsorted.csv` (200k rows x 41 cols), dumped to `/tmp/statsperf/fields.bin`.

Measured field-length distribution (non-empty fields only):
`<=8 B: 41.4% | <=16 B: 70.6%`, avg 17.7 B. Consecutive-comparison outcomes:
**first byte decides 50.2%**, exactly equal 17.2%, full compare needed 32.6%.
(I had claimed the first byte would decide "the large majority" — it decides **half**.)

### H1c microbenchmark — `-7.6%`, so ~2% of wall time. NOT worth a release.

Variant A = verbatim `MinMax::add_bytes` steady state. Variant B = inline first-byte
short-circuit + `Equal` fast path (skip copy *and* max-compare). Equivalence asserted on
asc/desc/min/max/last across all 41 columns: **identical**.

| | time | per value |
|---|---:|---:|
| A current | 54.92 ms | 9.79 ns |
| B H1c | 50.76 ms | 9.04 ns |
| **delta** | **-7.6%** | |

`add_bytes` is ~24-33% of the run, so -7.6% of it is **~2% of wall time**. The load-bearing
assumption — that libc call overhead dominates at ~17.7 bytes — **did not hold**: Apple's
`_platform_memcmp` is already short-length-tuned. Same failure shape as the 11.1 ms sort figure.
**Do not ship H1c on its own.**

### H5 re-diagnosed twice, and only the third diagnosis was right

1. First guess: chunk workers sized off the wrong record count. **Wrong** — `stats.rs:2916`
   correctly passes `chunk_size`.
2. Second guess: clamp saturation. **Wrong** — 1M/16 => 6,250, far below the 65,536 cap.
3. Actual: caller attribution of `reserve_rehash` shows **62.5% comes from
   `Frequencies::merge`**, only 37.5% from per-chunk building.

Per-chunk build rehashing (2.61% of profile) is the **inherent amortized cost of doubling** —
growing to 62,500 re-inserts ~2x the final size no matter the starting capacity (from 6,250:
~93,750 re-insertions; from 256: ~99,700). Only exact pre-sizing removes it, and per-chunk
cardinality is unknowable on a first pass. **Not fixable cheaply. Drop.**

The merge is the real target. `merge_chunks_in_order` (`stats.rs:2607-2612`) makes **chunk 0's
`Vec<Stats>` the accumulator** (`None => merged = Some(chunk_stats)`), so its `Frequencies` maps
start at `(chunk_size/10).clamp(16, 65_536)` = 6,250 and must grow to the whole file's
cardinality — up to 1,000,000 for `Unique Key` (~8 doublings). `Commute::merge`
(`frequency.rs:441`) then does `self.data.reserve(v.data.len())` **once per incoming chunk**,
i.e. 15 incremental reserves into a growing table.

### H5-merge microbenchmark — `-17.5%`, worth ~5% of wall time. THE ONE SURVIVOR.

Same 41-column replay, split into 16 chunks, maps built exactly as `Stats::new` +
`add_borrowed` do. A = current incremental `reserve(v.len())` per chunk. B = sum the incoming
chunk lens and `reserve` once. Final entry count identical (1,455,804).

| | time |
|---|---:|
| A incremental reserve | 50.10 ms |
| B reserve-total-once | 41.31 ms |
| **delta** | **-17.5%** |

**Inclusive** profile share of `Frequencies::merge` in `everything --default` is **28.66%** —
far more than its 4.35% rehash leaf implied. So -17.5% of it is **~5% of wall time** (~17 ms of
349.7 ms), on the indexed-parallel path users actually hit.

⚠️ The fix is **not** local to `src/cmd/stats.rs`: `Frequencies` exposes `len()` but **no
`reserve()`**, and `Commute::merge` receives one chunk at a time so it cannot know the total.
It needs either a bulk-merge/reserve API in qsv-stats (=> point release) or restructuring
`merge_chunks_in_order`. Note frequency merging is order-independent (only `MinMax` sortiness
stitching needs file order), so a bulk merge is legal for `Frequencies` specifically.

## LAYOUT — measured, and the answer is no

The original question was about the layout of the hot-loop data structures. It was tested
directly, and **`size_of::<Stats>()` had never been measured** — the byte figures in the struct
comments are hand-maintained and there is no `size_of` assertion in the file. Real values
(replicating the struct against the actual qsv-stats types; datasketches slots are
pointer-sized handles):

| | bytes |
|---|---:|
| `FieldType` | 1 |
| `WhichStats` | 40 |
| `Option<TypedSum>` | 32 |
| `Option<OnlineStats>` (x3) | 72 each |
| `Option<Frequencies<Vec<u8>>>` | 40 |
| `MinMax<f64>` / `<i64>` / `<usize>` | 80 each |
| `MinMax<Vec<u8>>` | 112 |
| `TypedMinMax` | **432** (48% of `Stats`) |
| **`size_of::<Stats>()`** | **896** (align 64) |

At 41 columns that is **35 KB walked per row**. Of each 896 bytes, **224 (25.0%) are dead in a
default unweighted run** (`weighted_online` 72, `weighted_modes` 40, `weighted_unsorted_stats`
24, `unsorted_stats` 32, `tdigest` 8, `hll` 8, `which` 40 — and `which` is `clone()`d
identically into all 41 columns). Within `TypedMinMax`, **240 of 432 bytes are untouched for a
String column**.

That is a textbook case for an array-of-structs hot/cold split. **It does not work.**

Benchmark: real 200k-row x 41-col field data replayed **row-major** — the true access pattern,
striding one `Stats` per field (the earlier H1c/H5 harnesses were column-major, which would
have hidden this). Layout A = current 896 B inline. Layout B = 704 B hot struct + a parallel
`Vec<ColdB>` holding the 224 dead bytes. Column count scaled by replicating real columns:

| cols | A KB/row | B KB/row | A ms | B ms | delta |
|---:|---:|---:|---:|---:|---:|
| 41 | 35 | 28 | 48.3 | 48.3 | **+0.1%** |
| 123 | 107 | 84 | 170.0 | 174.3 | +2.5% |
| 246 | 215 | 169 | 389.0 | 392.3 | +0.8% |
| 492 | 430 | 338 | 936.3 | 953.5 | +1.8% |
| 984 | 861 | 676 | 2418.1 | 2631.8 | +8.8% |

Three independent runs at 41 columns gave -0.7%, -1.4%, +0.1% — a noise floor of about
+/-1.5%, so the realistic-width result is **exactly zero**. Shrinking the per-row working set by
21% buys nothing, and at extreme widths the split is measurably *worse* (the stride change
appears to interact badly with cache-set mapping; the mechanism is not worth chasing since the
direction is wrong anyway).

**Why layout is the wrong lever here.** The per-row walk is a short, perfectly regular,
hardware-prefetchable stride, and 35 KB sits inside L1D on this machine. The profile agrees:
time is in `read_record`, `add_with_parsed`, `_platform_memcmp` and `_platform_memmove` —
**compute and libc calls, not memory stalls**. The H1c harness measured 9.79 ns per field
value, roughly 30 cycles, which is real work rather than a miss. A loop that is compute-bound
does not get faster from a denser struct.

This also retires the related ideas by the same evidence: boxing/shrinking `TypedMinMax`
(already declined once as "hot"), moving `which` out to a shared reference, and a full
struct-of-arrays rewrite. All target a working set that is not the bottleneck.

## Status / outcome

**Seven hypotheses. All measured. One win, and it is in another crate.**

| # | hypothesis | measured | outcome |
|---|---|---|---|
| H3 | `--everything` sort -> `select_nth_unstable` | **1.77%** of everything -j1 | ⛔ refuted; also a real non-total-order risk |
| H1a | skip memcpy on consecutive duplicates | ~2% of plain -j1 | ⛔ too small alone |
| H1b | inline small buffer in `MinMax` | discriminator not met (cost is in calls, not stalls) | ⛔ dropped |
| H1c | inline first-byte short-circuit | **-7.6%** of `add_bytes` = ~2% wall | ⛔ too small for a release cycle |
| H5-build | `Frequencies` per-chunk capacity | 2.61%, inherent to amortized doubling | ⛔ not cheaply fixable |
| H5-merge | reserve the merge total once | **-17.5%** of a **28.66%** phase = **~5% wall** | ⛔ **DECLINED 2026-08-30** — not worth a qsv-stats release cycle + un-patch dance + ~1h44m verify rebuild |
| LAYOUT | hot/cold split of the 896 B `Stats` | **+0.1%** at 41 cols (noise floor +/-1.5%) | ⛔ refuted; worse at extreme widths |
| **H4** | **pre-compile strftime formats** | **-47.1%**, output identical | ✅ **FILED: dathere/qsv-dateparser#11** |

### The one actionable item

**dathere/qsv-dateparser#11.** `StrftimeItems` is 19.17% inclusive of the `--infer-dates` run,
100% attributed to `qsv_dateparser`. Pre-compiling the literal formats into `Vec<Item>` measures
**-47.1%** with byte-identical output. When it lands upstream, qsv only needs a dep bump.

### Do not re-open (each died to a number recorded above)

H3, H1a, H1b, H1c, H5-build, H5-merge, and every layout variant — including the ones retired by
the layout evidence without a separate experiment: boxing/shrinking `TypedMinMax`, hoisting the
duplicated `which` to a shared reference, and a struct-of-arrays rewrite. All target a working
set that is not the bottleneck.

Also still settled from earlier work, and not revisited here: hasher swap (foldhash won a
benchmarked shootout), algebraic float ops in `online.rs` (naive Welford conversion is 2.5x
slower), ragged-CSV `unwrap_unchecked` (declined), U1-U12 finder leads, multi-worker unindexed
compute (changes fp results).

### Honest framing

The hot loop was **already well optimized** — the June-2026 work banked -40% / -42% / -43% RSS,
and this pass confirms why nothing large remains: time sits in `csv_core::read_record` (25-29%
of the plain run) and in genuinely necessary per-field accumulator work. The loop is
**compute-bound, not memory-bound**, which is why layout changes do nothing.

"Measured, and no" was pre-registered as a valid outcome before any code was written, and it is
the correct answer here. Every number that killed an idea is recorded above so the next person
does not re-run these experiments.

### Reproducing

- Profiles: `samply record --save-only --unstable-presymbolicate --iteration-count N -r 4000`
  (`--save-only` alone yields an **unsymbolicated** profile — raw hex addresses; the sidecar
  `.syms.json` is what carries the symbols, and must be joined by rva range).
- Microbenchmarks replay **real field data** dumped from `data_unsorted.csv`, and the layout
  harness replays **row-major** — the true access pattern. A column-major harness hides layout
  effects entirely.
- Every A/B asserted output equivalence, not just timing.
