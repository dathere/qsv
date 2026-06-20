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
t moarstats --bivariate "$data"
t moarstats --bivariate --bivariate-stats all "$data"
t moarstats --advanced --bivariate "$data"
t moarstats --advanced --bivariate --bivariate-stats all "$data"
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
