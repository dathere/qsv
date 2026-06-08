# jemalloc Tuning in qsv (and what we can borrow from Polars)

This note compares how **Polars** tunes jemalloc against what **qsv** already does, and
flags which Polars techniques are worth adopting. The headline: qsv and Polars tune
jemalloc in **opposite directions** because they have opposite lifecycles. Don't blindly
copy Polars' values — understand the goal first.

---

## TL;DR

| | Polars | qsv |
|---|---|---|
| Process lifecycle | Long-lived (embedded in a Python session) | Short-lived (one CLI command, then exit) |
| Optimization goal | **Low RSS** — give pages back to the OS quickly | **Throughput** — keep pages mapped for reuse |
| Decay tuning | **Shorten** decay (`dirty_decay_ms:500, muzzy_decay_ms:1000`) | **Disable** decay (`*_decay_ms = -1`) for aggregation commands |
| `background_thread` | On (Linux, via Cargo feature) | On (Linux, via runtime `mallctl`) |
| Transparent Huge Pages | Opt-in via `POLARS_THP=1` | **Not used** ← main thing worth borrowing |
| Config mechanism | `_RJEM_MALLOC_CONF` env var, set *before* alloc init | Runtime `mallctl` (`tikv-jemalloc-ctl`) |

**Net takeaway:** the one genuinely new lever from Polars is **Transparent Huge Pages
(THP)**. Everything else qsv already does, just via a different (and for qsv's purposes,
better) mechanism.

---

## What Polars does

### 1. Decay tuning (Python side)
`py-polars/src/polars/__init__.py`, run *before* the Rust bindings load:

```python
jemalloc_conf = "dirty_decay_ms:500,muzzy_decay_ms:1000"
if os.environ.get("POLARS_THP") == "1":
    jemalloc_conf += ",thp:always,metadata_thp:always"
if override := os.environ.get("_RJEM_MALLOC_CONF"):
    jemalloc_conf += "," + override
os.environ["_RJEM_MALLOC_CONF"] = jemalloc_conf
```

- `dirty_decay_ms:500` / `muzzy_decay_ms:1000` — **shorter than jemalloc's defaults**
  (10s / 10s). A long-lived process that briefly balloons then frees should hand pages
  back to the OS fast, or RSS stays high for the rest of the session. (Fixes pola-rs/polars#18088, #21829.)
- `thp:always,metadata_thp:always` — opt-in Transparent Huge Pages (see below).
- User `_RJEM_MALLOC_CONF` is appended last, so user keys win.

### 2. Build-time allocator features (Rust side)
`crates/polars-ooc/Cargo.toml`:

- Linux: `tikv-jemallocator` with `disable_initial_exec_tls` + `background_threads`.
- macOS: same **minus** `background_threads` (unsupported — jemalloc#843).

`disable_initial_exec_tls` matters because Polars' allocator is loaded as a **dynamic
library** (a Python extension). It avoids the `initial-exec` TLS model that can fail under
`dlopen`. **This does not apply to qsv** — qsv is a statically-linked binary, so this flag
is irrelevant.

---

## What qsv already does

All in `src/util.rs`, gated on `feature = "jemallocator"` (the default):

### Lever A — `init_allocator_runtime()`
Enables jemalloc `background_thread` at startup via `tikv_jemalloc_ctl::background_thread`.
Offloads page purging (`madvise`) onto background threads at **no RSS cost**. Best-effort:
the write is rejected on macOS, and the read-back records the real state in
`BACKGROUND_THREADS_ACTIVE`. Disabled by `QSV_NO_ALLOC_TUNING`.

This is the same end state as Polars' `background_threads` Cargo feature — qsv just reaches
it at runtime instead of build time, which is strictly more flexible.

### Lever B — `retain_alloc_pages_for_aggregation()`
For aggregation-heavy commands (`stats -E`, `frequency`, …) that churn many `Frequencies`
hashmaps across rayon workers, it sets `dirty_decay_ms = -1` and `muzzy_decay_ms = -1`
(**never return pages**) for the duration. Trades ~+16% peak RSS for ~6–9% faster runs by
eliminating `madvise` churn. Scoped per-command; qsv exits before retained pages matter, so
no restore is done. Skipped when Lever A is active (background purging already removes the
churn) or when `QSV_NO_ALLOC_TUNING` is set.

> **This is the inverse of Polars' decay tuning.** Polars *shortens* decay to lower RSS;
> qsv *disables* decay to raise throughput. Both are correct — for their respective
> lifecycles. **Do not copy Polars' `500/1000` decay values into qsv;** that would slow
> qsv's hot aggregation path for an RSS win qsv doesn't need (it's about to exit anyway).

---

## What's worth borrowing: Transparent Huge Pages (THP)

This is the only Polars lever qsv doesn't already have, and it could help qsv's
aggregation-heavy commands on Linux: large hashmaps over big arenas cause TLB pressure, and
2 MB huge pages cut TLB misses.

```
thp:always,metadata_thp:always
```

### The catch: `thp` / `metadata_thp` are **opt-only**
Unlike `dirty_decay_ms`/`muzzy_decay_ms` (writable at runtime via `mallctl`, which is how
qsv's Lever B works), `thp` and `metadata_thp` are **`opt.*` options** — read-only once
jemalloc initializes. They can **only** be set via the config env var *before* the
allocator wakes up. So qsv's existing runtime-`mallctl` approach **cannot** set THP; it has
to go through the env-var path, the same way Polars does.

### Two ways to expose it in qsv

**Option 1 — Document it for users (zero code).** THP is purely env-driven, so users on
Linux can already try it today:

```bash
# Note the variable name — see "Gotcha" below.
_RJEM_MALLOC_CONF="thp:always,metadata_thp:always" qsv stats -E bigfile.csv
```

Requirements:
- Linux only (macOS jemalloc has no usable THP).
- The kernel must allow it: `/sys/kernel/mm/transparent_hugepage/enabled` should be
  `always` or `madvise` (not `never`).
- The jemalloc that `tikv-jemallocator` builds must have THP support compiled in (the
  vendored build on Linux does).

**Option 2 — Add a `QSV_THP` lever (small code change), mirroring Polars' `POLARS_THP`.**
Because the value must be set before allocator init, qsv would need to set the env var and
**re-exec itself** once at startup if `QSV_THP=1` and the var isn't already applied (you
can't retro-set an `opt.*` knob in-process). Sketch:

```rust
// very early in main(), before any allocation-heavy work
#[cfg(all(feature = "jemallocator", not(feature = "mimalloc")))]
fn maybe_apply_thp() {
    use std::env;
    const SENTINEL: &str = "QSV_THP_APPLIED";
    if env::var_os(SENTINEL).is_some() { return; }            // already re-exec'd
    if !util::get_envvar_flag("QSV_THP") { return; }          // opt-in only
    if cfg!(not(target_os = "linux")) { return; }             // Linux only

    let mut conf = "thp:always,metadata_thp:always".to_string();
    if let Ok(existing) = env::var("_RJEM_MALLOC_CONF") {
        conf = format!("{existing},{conf}");                  // don't clobber user config
    }
    // SAFETY: single-threaded at this point, before allocator init work.
    unsafe {
        env::set_var("_RJEM_MALLOC_CONF", conf);
        env::set_var(SENTINEL, "1");
    }
    // re-exec so jemalloc reads the new opt.* config on a fresh start
    let exe = env::current_exe().expect("current_exe");
    let err = std::process::Command::new(exe)
        .args(env::args_os().skip(1))
        .exec();                                              // std::os::unix::process::CommandExt
    panic!("re-exec for QSV_THP failed: {err}");
}
```

Re-exec is a real cost (process restart), so gate it strictly behind opt-in and only when
THP isn't already configured. Benchmark before shipping — TLB wins on huge-page-friendly
workloads can be erased by huge-page allocation overhead on small inputs.

---

## Gotcha: the config env var name (`MALLOC_CONF` vs `_RJEM_MALLOC_CONF`)

Which env var jemalloc reads depends on a `tikv-jemallocator` build feature:

- **Without** `unprefixed_malloc_on_supported_platforms` (this is qsv's current build —
  `Cargo.toml` declares `tikv-jemallocator = { version = "0.7", optional = true }` with no
  features): the symbols are **prefixed**, so the config var is **`_RJEM_MALLOC_CONF`**.
- **With** that feature: jemalloc takes over the system `malloc` and reads bare
  **`MALLOC_CONF`**.

> **Heads-up:** `src/util.rs::show_env_vars()` currently surfaces the bare `MALLOC_CONF`
> name, but qsv's default build actually reads `_RJEM_MALLOC_CONF`. If you adopt any
> env-var-based tuning (THP or otherwise), verify the live name on your build first:
>
> ```bash
> # Should change behavior on the prefixed (default) build:
> _RJEM_MALLOC_CONF="stats_print:true" qsv --version 2>&1 | head
> # vs the unprefixed build:
> MALLOC_CONF="stats_print:true" qsv --version 2>&1 | head
> ```
>
> Whichever one prints a jemalloc stats dump is the name your binary honors. Consider
> updating `show_env_vars()` to list both.

---

## Other Polars knobs and whether qsv should care

| jemalloc option | Polars | qsv applicability |
|---|---|---|
| `dirty_decay_ms` / `muzzy_decay_ms` | shortens (low RSS) | qsv **disables** for throughput — keep as-is, don't copy Polars' direction |
| `background_thread` | Cargo feature (Linux) | already on via runtime `mallctl` (Lever A) |
| `thp` / `metadata_thp` | `POLARS_THP=1` | **borrow this** — Linux-only, env/opt path required |
| `disable_initial_exec_tls` | Cargo feature | **N/A** — only matters for dlopen'd dylibs; qsv is a static binary |

---

## How to test any change

1. Pick an aggregation-heavy workload on a reasonably large input:
   ```bash
   qsv stats -E bigfile.csv > /dev/null
   qsv frequency bigfile.csv > /dev/null
   ```
2. Measure wall-clock **and** peak RSS:
   ```bash
   /usr/bin/time -v qsv stats -E bigfile.csv > /dev/null   # Linux: "Maximum resident set size"
   ```
3. Compare four configs: default · `QSV_THP=1` · `QSV_NO_ALLOC_TUNING=1` · THP + retention.
4. Confirm the active allocator string in `qsv --version` (it reports `jemalloc` vs
   `jemalloc+bgthread`).
5. Repeat 3× — allocator effects are noisy; report medians.

Opt-out of all qsv tuning at any time with `QSV_NO_ALLOC_TUNING=1`.

---

## References

- Polars decay/THP config: `py-polars/src/polars/__init__.py`
- Polars allocator features: `crates/polars-ooc/Cargo.toml`, `crates/polars-ooc/src/global_alloc.rs`
- Polars issues: pola-rs/polars#18088, #21829; jemalloc#843 (macOS `background_thread`)
- qsv tuning: `src/util.rs` (`init_allocator_runtime`, `retain_alloc_pages_for_aggregation`), `src/main.rs` (`#[global_allocator]`)
- jemalloc tuning guide: <https://github.com/jemalloc/jemalloc/blob/dev/TUNING.md>
- `MALLOC_CONF` options: `man jemalloc` / <https://jemalloc.net/jemalloc.3.html>
