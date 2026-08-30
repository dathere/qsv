# Perf harnesses for the 2026-08 `qsv stats` hot-loop investigation

Supporting material for [`../2026-08-stats-hotloop-profile.md`](../2026-08-stats-hotloop-profile.md).
Every conclusion in that document came from one of these; they are checked in so the results can
be **reproduced or challenged** rather than taken on faith.

**No `Cargo.toml` is checked in on purpose** — a nested manifest inside the `qsv` package
directory would interfere with `cargo build`/`cargo package`. Create each harness outside the
repo, using the manifest below.

## Building a Rust harness

```bash
mkdir -p /tmp/bench/src && cd /tmp/bench
cat > Cargo.toml <<'TOML'
[package]
name = "bench"
version = "0.1.0"
edition = "2021"
[dependencies]
# h1c_add_bytes.rs / layout_hot_cold.rs:
qsv-stats = { path = "/path/to/qsv-stats" }   # or = "0.55"
hashbrown = "0.17"
# h5_merge_reserve.rs:  hashbrown = "0.17"
# dateparser_precompiled_fmt.rs:  chrono = "0.4"
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
TOML
cp /path/to/qsv/docs/perf/harnesses/<harness>.rs src/main.rs
cargo build --release && ./target/release/bench
```

The release profile above intentionally mirrors qsv's own (`opt-level=3`, fat LTO,
`codegen-units=1`) so inlining behaviour is comparable.

## Fixtures

```bash
# column-major, non-empty values only (H1c, H5 merge)
./dump_fields.py data_unsorted.csv cm /tmp/statsperf/fields.bin 200000
# row-major, empties preserved (layout) -- this is the TRUE per-row access pattern
./dump_fields.py data_unsorted.csv rm /tmp/statsperf/fields_rm.bin 200000
# real date values for the dateparser harness
qsv select '"Created Date"' NYC_311_SR_2010-2020-sample-1M.csv | tail -n +2 | head -200000 > /tmp/dates.txt
```

⚠️ A **column-major** harness hides layout effects entirely — the real loop walks one `Stats`
per field across all columns, then advances a row. Use `rm` for anything about layout.

## Harnesses

| file | question | result |
|---|---|---|
| `h1c_add_bytes.rs` | inline first-byte short-circuit + `Equal` fast path in `MinMax::add_bytes` | **-7.6%** of `add_bytes` (~2% wall) -> rejected |
| `h5_merge_reserve.rs` | reserve the merge total once vs 15 incremental reserves | **-17.5%** of a 28.66% phase (~5% wall) -> declined |
| `layout_hot_cold.rs` | hot/cold split of the 896-byte `Stats`, scaled 41->984 cols | **+0.1%** at realistic widths -> refuted |
| `dateparser_precompiled_fmt.rs` | pre-compiled `Vec<Item>` vs `parse_from_str` | **-47.1%** -> filed as dathere/qsv-dateparser#11 |

Each Rust harness **asserts output equivalence** between variants before timing, so a "win" that
changed results would fail rather than be reported.

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
