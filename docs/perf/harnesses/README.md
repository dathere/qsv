# Perf harnesses for the 2026-08 `qsv stats` hot-loop investigation

Supporting material for [`../2026-08-stats-hotloop-profile.md`](../2026-08-stats-hotloop-profile.md).
Every conclusion in that document came from one of these; they are checked in so the results can
be **reproduced or challenged** rather than taken on faith.

**No `Cargo.toml` is checked in on purpose** — a nested manifest inside the `qsv` package
directory would interfere with `cargo build`/`cargo package`. Create each harness outside the
repo, using the manifest below.

## Fixtures — start here

The dataset is the NYC 311 service-request sample (1M rows x 41 cols). `data_unsorted.csv` and
`data_sorted.csv` are row-shuffled / row-sorted copies of it:

```bash
cd /path/to/qsv                      # everything below assumes the repo root
DATA=NYC_311_SR_2010-2020-sample-1M.csv     # 538 MB; see the wiki benchmark page for the source
cp "$DATA" data_unsorted.csv                # already unordered as distributed
qsv sort --random --seed 42 "$DATA" -o data_unsorted.csv   # or force it
qsv sort "$DATA" -o data_sorted.csv

mkdir -p /tmp/statsperf
# column-major, non-empty values only (H1c, H5) -- add_with_parsed early-returns on len==0
python3 docs/perf/harnesses/dump_fields.py data_unsorted.csv cm /tmp/statsperf/fields.bin 200000
# row-major, empties preserved (layout) -- the TRUE per-row access pattern
python3 docs/perf/harnesses/dump_fields.py data_unsorted.csv rm /tmp/statsperf/fields_rm.bin 200000
# real date values for the dateparser harness
qsv select '"Created Date"' "$DATA" | tail -n +2 | head -200000 > /tmp/dates.txt
```

⚠️ A **column-major** fixture hides layout effects entirely — the real loop walks one `Stats`
per field across all columns, then advances a row. Use `rm` for anything about layout.

## Building a Rust harness

Copy the harness to `src/main.rs` of a scratch crate outside the repo, with the dependency line
its table below specifies. Each harness needs exactly one:

| harness | `[dependencies]` |
|---|---|
| `h1c_add_bytes.rs` | `qsv-stats = "0.55"` |
| `layout_hot_cold.rs` | `qsv-stats = "0.55"` and `hashbrown = "0.17"` |
| `h5_merge_reserve.rs` | `hashbrown = "0.17"` |
| `dateparser_precompiled_fmt.rs` | `chrono = "0.4"` |

```bash
H=layout_hot_cold                  # pick one from the table
mkdir -p /tmp/bench/src && cd /tmp/bench
cat > Cargo.toml <<'TOML'
[package]
name = "bench"
version = "0.1.0"
edition = "2021"
[dependencies]
qsv-stats = "0.55"
hashbrown = "0.17"
chrono    = "0.4"
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
TOML
cp /path/to/qsv/docs/perf/harnesses/$H.rs src/main.rs
cargo build --release
./target/release/bench eq                          # equivalence first
for r in 1 2 3; do ./target/release/bench A; ./target/release/bench B; done
```

Listing all four dependencies is harmless — unused ones are simply not linked. Use
`qsv-stats = { path = "../qsv-stats" }` instead if you are testing a local modification.
The release profile mirrors qsv's own (`opt-level=3`, fat LTO, `codegen-units=1`) so inlining
behaviour is comparable.

## End-to-end timings (hyperfine)

```bash
cargo build --locked --profile release-samply --bin qsv -F all_features   # ~1h44m, fat LTO
Q=./target/release-samply/qsv
qsv index "$DATA"        # the "default" rows below take the indexed-parallel path
clean() { find . -maxdepth 1 -name '*.stats.csv*' -delete; }
clean
hyperfine --warmup 1 --min-runs 5 \
  --prepare 'find . -maxdepth 1 -name "*.stats.csv*" -delete' \
  -n plain-j1            "$Q stats --force -j 1 $DATA" \
  -n plain-default       "$Q stats --force $DATA" \
  -n everything-j1       "$Q stats --force --everything -j 1 $DATA" \
  -n everything-default  "$Q stats --force --everything $DATA" \
  -n dates-default       "$Q stats --force --everything --infer-dates $DATA"
clean
```

⚠️ `--force` on every run and clear `*.stats.csv*` between them — a leftover cache silently
changes the code path. ⚠️ Never pass hyperfine `-i`: it turns a hard failure into a
plausible-looking timing.

## Harnesses

| file | question | result |
|---|---|---|
| `h1c_add_bytes.rs` | inline first-byte short-circuit + `Equal` fast path in `MinMax::add_bytes` | **-3.8%** of `add_bytes` (~1% wall) -> rejected |
| `h5_merge_reserve.rs` | reserve the merge total once vs 15 incremental reserves | **-17.9%** of a 28.66% phase (~5% wall) -> declined |
| `layout_hot_cold.rs` | hot/cold split of the 896-byte `Stats`, scaled 41->984 cols | **±1.2% = noise** -> refuted |
| `dateparser_precompiled_fmt.rs` | pre-compiled `Vec<Item>` vs `parse_from_str` | **-47.1%** -> filed as dathere/qsv-dateparser#11 |

## ⚠️ Run each variant in its OWN process

**This is not optional, and it is the single most important thing on this page.**

Every harness takes a variant argument: `eq` (equivalence check only), `A` (baseline), `B`
(candidate). **Time `A` and `B` in separate process invocations and compare across them.**

Running A-then-B *in one process* lets the first variant's allocations decide the second's heap
placement, and for `layout_hot_cold.rs` that confound was larger than the effect being measured
— it produced **+0.1% to +8.8% (B slower)** in one ordering and **-4% to -17% (B faster)** after
merely inserting a warm-up pass, from identical code. Under process isolation the true answer is
**±1.2%, i.e. nothing**. The confound is capable of manufacturing a fake win as easily as a fake
loss; do not trust an in-process A/B here.

```bash
./bench eq                     # assert A and B produce identical results
for r in 1 2 3; do ./bench A; ./bench B; done    # interleave rounds, compare means
```

## Equivalence checking

`eq` compares **full observable state**, not summaries:

- `h1c_add_bytes.rs` — asc/desc pair counts, min, max, last, per column.
- `h5_merge_reserve.rs` — the complete key->count map per column (an earlier version compared
  only summed cardinality, which would have passed on differing counts).
- `layout_hot_cold.rs` — nullcount, sum, string/length min-max, online mean/variance and
  cardinality per column. f64 fields are compared **bitwise** (`to_bits`) so that `NaN == NaN`;
  a naive `PartialEq` reports a spurious mismatch on empty accumulators.
- `dateparser_precompiled_fmt.rs` — value-by-value parse results.

## Profile analysis

```bash
samply record --save-only --unstable-presymbolicate --iteration-count 20 -r 4000 \
  -o /tmp/statsperf/s_plain_def.json.gz -- \
  ./target/release-samply/qsv stats --force NYC_311_SR_2010-2020-sample-1M.csv

python3 sym.py       /tmp/statsperf/s_plain_def.json.gz 25   # self-time by symbol
python3 callers.py   /tmp/statsperf/s_plain_def.json.gz      # inverted stacks for one leaf
python3 inclusive.py /tmp/statsperf/s_plain_def.json.gz "<symbol substring>" ...
```

⚠️ `samply record --save-only` **without** `--unstable-presymbolicate` produces an
**unsymbolicated** profile — every frame is a raw hex address. The symbols live in the sidecar
`.syms.json`, which `sym.py` joins back by rva range. Edit the `callers.py` target substring
(bottom of the file) to attribute a different leaf.
