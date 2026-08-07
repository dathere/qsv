#!/usr/bin/env bash
#
# pgo-train.sh - lean, single-pass PGO training harness for qsv.
#
# Usage: ./pgo-train.sh <path-to-instrumented-qsv> [--minimal]
#
#   <path-to-instrumented-qsv>  REQUIRED. The PGO-instrumented qsv binary built by
#                               `cargo pgo instrument build` (see build-pgo.sh).
#   --minimal                   Skip the large data download and the heavier feature
#                               paths (polars/geocode/to). Use on Windows runners and
#                               for reduced-feature binaries (qsvlite-style).
#
# Unlike benchmarks.sh, this is NOT a benchmark: it does not use hyperfine and runs
# each command exactly ONCE. Its sole purpose is to exercise a representative spread
# of qsv's hot and feature-gated code paths so the instrumented binary emits useful
# PGO profile data (.profraw files). cargo-pgo bakes an absolute -Cprofile-generate
# path into the instrumented binary, so the profraw files land in target/pgo-profiles/
# regardless of this script's working directory.
#
# Commands that are not compiled into a given binary variant are tolerated and skipped
# (see the `t` helper), so the same harness works against full and reduced builds.
#
# Data is mirrored from benchmarks.sh: the 520MB NYC 311 1M-row sample. Keep the
# benchmark_data_url / filename variables below in sync with scripts/benchmarks.sh.

set -uo pipefail

# ---- args -------------------------------------------------------------------
if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <path-to-instrumented-qsv> [--minimal]" >&2
  exit 1
fi

# resolve the instrumented binary to an absolute path BEFORE we cd into the work dir
qsv_bin="$1"
shift
if command -v realpath &>/dev/null; then
  qsv_bin=$(realpath "$qsv_bin" 2>/dev/null || echo "$qsv_bin")
fi
if [[ ! -x "$qsv_bin" ]]; then
  echo "ERROR: instrumented qsv binary not found or not executable: $qsv_bin" >&2
  exit 1
fi

minimal=0
for a in "$@"; do
  case "$a" in
    --minimal) minimal=1 ;;
    *) echo "WARNING: ignoring unknown argument: $a" >&2 ;;
  esac
done

# ---- data variables (keep in sync with benchmarks.sh) -----------------------
benchmark_data_url=https://raw.githubusercontent.com/wiki/dathere/qsv/files/NYC_311_SR_2010-2020-sample-1M.7z
communityboards_url=https://raw.githubusercontent.com/wiki/dathere/qsv/files/communityboards.csv
datazip=NYC_311_SR_2010-2020-sample-1M.7z
data=NYC_311_SR_2010-2020-sample-1M.csv

# 7z is 7zz on macOS, 7z elsewhere (Linux/Cygwin/git-bash) - same as benchmarks.sh
if [[ "$OSTYPE" == "darwin"* ]]; then
  sevenz_bin=7zz
else
  sevenz_bin=7z
fi

# isolated working dir so we never pollute scripts/ or the repo root
work_dir="${PGO_TRAIN_DIR:-target/pgo-train}"
mkdir -p "$work_dir"
cd "$work_dir" || { echo "ERROR: cannot cd into $work_dir" >&2; exit 1; }

echo "> PGO training qsv: $qsv_bin"
echo "  work dir: $(pwd)"
echo "  minimal mode: $minimal"
"$qsv_bin" --version || true
echo ""

# ---- training-command helper ------------------------------------------------
# Runs a qsv invocation once, tolerating non-zero exits (a reduced binary may not
# have the subcommand, or the command may need an optional feature). The whole point
# is to drive instrumented code, not to assert success - so failures are logged and
# training continues.
t() {
  echo "  train: qsv $*"
  "$qsv_bin" "$@" >/dev/null 2>&1 || echo "    (skipped/failed: qsv $1)"
}

# ---- data acquisition -------------------------------------------------------
if [[ "$minimal" -eq 0 ]]; then
  if ! command -v "$sevenz_bin" &>/dev/null; then
    echo "WARNING: $sevenz_bin not found; falling back to --minimal training." >&2
    minimal=1
  fi
fi

if [[ "$minimal" -eq 0 && ! -r "$data" ]]; then
  echo "> Downloading training data..."
  if ! curl --fail -sS "$benchmark_data_url" -o "$datazip"; then
    echo "WARNING: failed to download $benchmark_data_url; falling back to --minimal." >&2
    rm -f "$datazip"
    minimal=1
  elif ! "$sevenz_bin" e -y "$datazip" >/dev/null; then
    echo "WARNING: failed to extract $datazip; falling back to --minimal." >&2
    rm -f "$datazip"
    minimal=1
  fi
  rm -f "$datazip"
fi

# In minimal mode (or if the download failed), synthesize a small but non-trivial
# CSV by dogfooding qsv itself, so we still train the core CSV engine on Windows or
# when the network/7z is unavailable.
if [[ "$minimal" -eq 1 || ! -r "$data" ]]; then
  data=pgo_train_minimal.csv
  if [[ ! -r "$data" ]]; then
    echo "> Generating minimal training data ($data)..."
    {
      echo "id,name,category,amount,city,date"
      for i in $(seq 1 20000); do
        echo "$i,name_$((i % 997)),cat_$((i % 13)),$(( (i * 7) % 10000 )).$((i % 100)),city_$((i % 53)),2024-$(printf '%02d' $((i % 12 + 1)))-$(printf '%02d' $((i % 28 + 1)))"
      done
    } >"$data"
  fi
fi

echo "> Training on: $data ($(wc -l <"$data" 2>/dev/null || echo '?') lines)"
echo ""

# ---- support data (best-effort; tolerated) ----------------------------------
if [[ "$minimal" -eq 0 && ! -r communityboards.csv ]]; then
  curl --fail -sS "$communityboards_url" -o communityboards.csv || rm -f communityboards.csv
fi

# index speeds up & exercises multithreaded code paths during training
t index "$data"

# ---- core CSV-engine training (all binary variants) -------------------------
t count "$data"
t count --no-polars "$data"
t datefmt "Created Date" "$data"
t datefmt --formatstr '%V' "Created Date" --new-column week_number "$data"
t explode City "-" "$data"
t headers "$data"
t select 1-5 "$data"
t slice --start 0 --len 5000 "$data"
t sample --seed 42 10000 "$data"
t search -s 1 "[0-9]" "$data"
t frequency "$data"
t frequency -i "$data"
t frequency --limit 0 "$data"
t stats "$data"
t stats --everything "$data"  
t stats --everything --infer-dates "$data"
t dedup "$data"
t sort "$data"
t flatten "$data"
t flatten "$data" --condense 50
t split --size 50000 pgo_train_split_size "$data"
t split --chunks 20 pgo_train_split_chunks "$data"
t cat rows "$data" "$data"
t behead "$data"
t fixlengths "$data"
t replace "[0-9]" "X" "$data"
t extdedup "$data" pgo_train_extdedup.csv
t extdedup "$data" --select 1-5
t extsort "$data" pgo_train_extsort.csv
t extsort "$data" --select 1-5

# ---- feature-gated paths (skipped automatically on reduced binaries) --------
t apply calcconv --formatstr "{Unique_Key} meters in miles" --new-column new_col "$data"
t apply dynfmt --formatstr "{Created_Date} {Complaint_Type} - {BBL} {City}" --new-column new_col "$data"
t apply emptyreplace "Bridge Highway Name" --replacement Unspecified "$data"
t apply operations lower,eudex Agency --comparand Queens --new-column Agency_queens_soundex "$data"
t apply operations lower 2 "$data"
t schema "$data" --stdout
t tojsonl "$data" --output pgo_train.jsonl
t jsonl pgo_train.jsonl --batch 0
t snappy compress "$data" --output pgo_train.snappy
t snappy decompress pgo_train.snappy
t snappy validate pgo_train.snappy
t validate "$data"
t moarstats "$data"
t moarstats --advanced "$data"

# `--bivariate-stats all` USED TO BE a memory bomb. The default (--bivariate-stats fast =
# pearson + covariance) uses streaming algorithms and stays cheap, which is why plain
# `--bivariate` finishes in ~40s. Passing "all" additionally enables mi/nmi/u, and those make
# EVERY field pair accumulate a joint-frequency map. Those maps were keyed by owned String
# pairs -- on the 1M-row/41-column training file, 780 pairs x ~123M joint cells at ~200 bytes
# each -- and the -C/--cardinality-threshold guard that would have skipped MI for
# high-cardinality fields defaulted to 1000000, exactly the row count here, so it never
# engaged.
#
# Measured peak RSS was 23.0 GiB against a GitHub hosted runner's 15.57 GiB, so it OOM-killed
# the runner VM outright; the agent died, the `t` helper below never saw an exit code, and
# training just stopped. Not arch-specific - in run 31069999861 BOTH full-training targets
# died on this exact command with "runner has received a shutdown signal" / exit 143:
# aarch64-unknown-linux-gnu (job 92515824811) and x86_64-unknown-linux-gnu (job
# 92515824877). Windows escaped it only because it trains with --minimal.
#
# FIXED in #4356: joint keys are now a packed pair of u32 symbols from per-column value
# dictionaries, columns are decoded once per record instead of once per pair, and -C defaults
# to half the row count (floored at 1000) instead of a fixed 1,000,000. Measured peak RSS on
# the FULL 1M rows is now 8.05 GiB, well inside a hosted runner, and wall clock went 455s ->
# 64s. The full file would survive today.
#
# The slice stays anyway, for two reasons that have nothing to do with the old OOM:
#   1. PGO records WHICH branches execute, not how many rows flow through them, so a slice
#      buys the same profile coverage. Volume was never the point.
#   2. 8.05 GiB is still ~52% of a hosted runner's RAM. A training harness should not sit that
#      close to the ceiling when it gains nothing by doing so.
# The slice is indexed on purpose: compute_all_bivariatestats() only takes the parallel
# chunked path when an index exists and the row count clears PARALLEL_THRESHOLD (10_000 in
# src/cmd/moarstats.rs), and that parallel path is the one worth profiling.
#
# Peak RSS by slice size, re-measured after #4356 (previous figures in parentheses):
#   1,000,000 rows -> 8.05 GiB (was 23.00)   200,000 -> 3.01 GiB (was ~11.1)
#      50,000 rows -> 1.31 GiB (was  4.12)
# 50k keeps the parallel path (5x PARALLEL_THRESHOLD) at ~8% of a hosted runner's RAM.
biv_data="$data"
biv_bounded=0
if [[ "$minimal" -eq 0 ]]; then
  biv_slice=pgo_train_bivariate.csv
  if "$qsv_bin" slice --len 50000 "$data" --output "$biv_slice" >/dev/null 2>&1; then
    # index so the bivariate parallel path (not the sequential fallback) gets trained
    "$qsv_bin" index "$biv_slice" >/dev/null 2>&1 || true
    biv_data="$biv_slice"
    biv_bounded=1
  else
    echo "    (bivariate slice failed; skipping the --bivariate-stats all variants)"
  fi
else
  # minimal-mode data is already tiny (20k rows x 6 cols = 15 pairs), so it is bounded
  biv_bounded=1
fi

# Default --bivariate-stats (fast = pearson + covariance) streams and is safe even on the
# full file: in run 31069999861 aarch64 ran this over all 1M rows in 41s and survived.
t moarstats --bivariate "$biv_data"
t moarstats --advanced --bivariate "$biv_data"

# The "all" variants run ONLY against bounded input. Since #4356 the full file would no
# longer kill the runner (8.05 GiB vs 15.57 GiB), so this is now headroom rather than
# survival - but still do NOT fall back to the full file when the slice is unavailable:
# `t` tolerates a SKIPPED command (harmless), whereas silently promoting training to a
# 8 GiB, 64s workload trades a no-op for the largest single step in the run.
if [[ "$biv_bounded" -eq 1 ]]; then
  t moarstats --bivariate --bivariate-stats all "$biv_data"
  t moarstats --advanced --bivariate --bivariate-stats all "$biv_data"
fi
t blake3 "$data"
t luau map newcol "1 + 1" "$data"
t profile "$data"
t synthesize "$data" -n 1000 --seed 42 -o pgo_train_synth.csv

if [[ "$minimal" -eq 0 ]]; then
  # heavier paths that need the real dataset and the full-feature build
  t searchset <(printf "homeless\npark\nNoise\n") "$data"
  t to xlsx pgo_train.xlsx "$data"
  t excel pgo_train.xlsx
  t excel --metadata c pgo_train.xlsx
  # polars-backed paths - the biggest PGO win
  t sqlp "$data" "select * from _t_1 limit 1000"
  t sqlp "$data" "select \"Borough\", count(*) from _t_1 group by \"Borough\""
  t pivotp "Agency" --index "Borough" --values "Complaint Type" "$data"
  t pivotp "Agency" --index "Borough" --values "Complaint Type" --agg smart "$data"
  t pivotp "Created Date" --index "Borough" --values "Complaint Type" --try-parsedates "$data"
  if [[ -r communityboards.csv ]]; then
    t joinp "Community Board" "$data" community_board communityboards.csv
    t joinp "Community Board" "$data" community_board communityboards.csv --streaming
  fi
  t luau map newcol "1 + 1" "$data"
  # geocode auto-downloads the Geonames index on first run (network required)
  t geocode suggest City --new-column geocoded_city "$data"
  t geocode reverse Location --new-column geocoded_location "$data"
fi

echo ""
echo "> PGO training complete."
# cargo-pgo writes profiles to <project>/target/pgo-profiles. Derive that from the
# instrumented binary path (target/<triple>/<profile>/qsv) - which we resolved to an
# absolute path above - so the summary is correct even when PGO_TRAIN_DIR is elsewhere.
target_dir="$(cd "$(dirname "$qsv_bin")/../.." 2>/dev/null && pwd)"
profile_dir="$target_dir/pgo-profiles"
[[ -d "$profile_dir" ]] && echo "  profraw files: $(ls -1 "$profile_dir"/*.profraw 2>/dev/null | wc -l | tr -d ' ') in $profile_dir"
exit 0
