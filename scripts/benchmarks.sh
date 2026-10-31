#!/usr/bin/env bash
#
# Usage: ./benchmarks.sh <argument>
#  where <argument> is a substring pattern of the benchmark name.
#  e.g. ./benchmarks.sh sort - will run benchmarks with "sort" in the benchmark name
#  if <argument> is omitted, all benchmarks are executed.
#
#  if <argument> is "reset", the benchmark data will be downloaded and prepared again.
#   though the results/benchmark_results.csv and results/run_info_history.tsv historical
#   archives will be preserved.
#  if <argument> is "clean", temporary files will be deleted.
#  if <argument> is "setup", setup and install all the required tools.
#  if <argument> is "accept", the benchmarks named after it (e.g. ./benchmarks.sh accept joinp count)
#   are moved from results/suspect_results.csv into the results - see suspect_reason below.
#  if <argument> is "help", help text is displayed.
#
# ==============================================================================================
#
# This script benchmarks Quicksilver (qsv) using a 520mb, 41 column, 1M row sample of
# NYC's 311 data. If it doesn't exist on your system, it will be downloaded for you.
#
# Though this script was primarily created to maintain the Benchmark page on the qsv site,
# it was also designed to be a useful tool for users to benchmark qsv on their own systems,
# so it be can run on hardware and workloads that reflect your requirements/environment.
#
# Make sure you're using a release-optimized `qsv`.
# If you can't use the prebuilt binaries at https://github.com/dathere/qsv/releases/latest,
# build it to have at least the apply, geocode, luau, profile, readstat, synthesize, to, viz and polars features enabled:
# `CARGO_BUILD_RUSTFLAGS='-C target-cpu=native' cargo build --release --locked -F feature_capable,apply,geocode,luau,profile,readstat,synthesize,to,viz,polars` or
# `CARGO_BUILD_RUSTFLAGS='-C target-cpu=native' cargo install --locked qsv -F feature_capable,apply,geocode,luau,profile,readstat,synthesize,to,viz,polars`
#
# This shell script has been tested on Linux and macOS. It should work on other Unix-like systems,
# but will NOT run on native Windows. If you're on Windows, you can run it using Cygwin or WSL
# (see https://www.cygwin.com/ and https://docs.microsoft.com/en-us/windows/wsl/install-win10).
# It requires hyperfine (https://github.com/sharkdp/hyperfine#hyperfine) to run the benchmarks.
# It also requires 7-Zip (https://www.7-zip.org/download.html) as we need the high compression
# ratio so we don't have to deal with git-lfs to host the large compressed file on GitHub.
#
# And of course, it dogfoods `qsv` as well to prepare the benchmark data, fetch the rowcount,
# and to parse and format the benchmark results. :)
# It uses the following commands: apply, cat, count, luau, safenames, sample, schema, search, select, slice,
# fixedwidth, snappy, sort, sqlp, table, tojsonl, to xlsx and writestat. It's a good example of how qsv can be used to automate data preparation & analysis tasks.

arg_pat="$1"

# the version of this script
bm_version=10.1.0

# runs of a benchmark whose means are within this percentage of each other tie in the rank
# column (see the delta & rank comments near the end)
rank_tie_pct=2

# thresholds of the invalid-benchmark detectors (see suspect_reason). A benchmark that trips
# one is QUARANTINED in results/suspect_results.csv instead of being recorded; review it and
# record it with `./benchmarks.sh accept <name>` if the result is real.
#  QSVBM_SPREAD_MAX: max slowest/fastest timed-run ratio. A larger spread is the signature of a
#   command that only did its work on some runs (to_sqlite timed its own failure this way).
#  QSVBM_SPEEDUP_PCT: flag a mean below this % of the benchmark's most recent archived version.
#  QSVBM_MEM_DROP_PCT: flag a peak memory below this % of its most recent archived peak memory.
# A failed invocation looks like a huge speedup that also skipped the work's memory. These are
# relative to each benchmark's OWN history: qsv's startup (~9 ms, ~20 MiB) is also the real
# cost of many cheap benchmarks (count_index, headers, cached stats), so no absolute floor works.
spread_max=${QSVBM_SPREAD_MAX:-3}
speedup_pct=${QSVBM_SPEEDUP_PCT:-33}
mem_drop_pct=${QSVBM_MEM_DROP_PCT:-25}

# what hyperfine measures. Explicit units pin the CSV export to seconds and MiB, and user/system
# are only exported when asked for. The archive keeps hyperfine 1.x's column names - see
# hf_result_row.
hf_metrics=time_wall_clock:s,time_user:s,time_system:s,memory_peak_resident:MiB

# CONFIGURABLE VARIABLES ---------------------------------------
# change as needed to reflect your environment/workloads

# the path to the qsv binary, change this if you're not using the prebuilt binaries
# e.g. you compiled a tuned version of qsv with different features and/or CPU optimizations enabled
# The prebuilt release `qsv` is PGO-optimized (see .github/workflows/publish.yml), so we
# default to it for the bulk of the benchmarks.
# qsv_bin=../target/release/qsvlite
# qsv_bin=../target/debug/qsv
qsv_bin=qsv
# the path to the python-enabled qsv binary, used ONLY for the py benchmarks.
# The python feature is NOT compiled into the regular prebuilt (PGO-optimized) qsv;
# only the qsvpyNNN flavors have it. The py benchmarks are skipped if this binary is
# missing or was built without the python feature (see has_python below).
qsv_py_bin=qsvpy314
# the path to the qsv binary that we dogfood to run the benchmarks
# we use several optional features when dogfooding qsv (apply, luau & to)
# and the user may be benchmarking a qsv binary variant that doesn't have these features enabled
qsv_benchmarker_bin=qsv
benchmark_data_url=https://raw.githubusercontent.com/wiki/dathere/qsv/files/NYC_311_SR_2010-2020-sample-1M.7z
# where to download the benchmark data compressed file - this could be a zip or 7z file
datazip=NYC_311_SR_2010-2020-sample-1M.7z
# where to store the benchmark data
data=NYC_311_SR_2010-2020-sample-1M.csv

# Hyperfine options - run `hyperfine --help`` for more info
# number of warmup runs for each benchmark.  A minimum of 2 is recommended
warmup_runs=2
# number of benchmark runs for each benchmark. A minimum of 3 is recommended
benchmark_runs=3
# ----------------------------  end of CONFIGURABLE VARIABLES

data_filename=$(basename -- "$data")
filestem="${data_filename%.*}"

# check if qsv is installed
if ! command -v "$qsv_bin" &>/dev/null; then
  echo "qsv could not be found"
  echo "Please install qsv from https://qsv.dathere.com"
  exit 1
fi

# get current version of qsv
raw_version=$("$qsv_bin" --version)
# get the version of qsv used to run this script
# we use this to determine if the user is using a different qsv binary
# than the one used to run this script
benchmarker_version=$("$qsv_benchmarker_bin" --version)

# if arg_pat is equal to "help", show usage
if [[ "$arg_pat" == "help" ]]; then
  echo "qsv Benchmark Script v$bm_version"
  echo ""
  echo "Usage: ./benchmarks.sh <argument>"
  echo ""
  echo " where <argument> is a substring pattern of the benchmark name."
  echo "       e.g. ./benchmarks.sh sort - will run benchmarks with \"sort\" in the benchmark name"
  echo "       if <argument> is omitted, all benchmarks will be executed."
  echo ""
  echo "       if <argument> is:"
  echo "          \"reset\": the benchmark data will be downloaded and prepared again."
  echo "                     though the results/benchmark_results.csv"
  echo "                     historical archive will be preserved."
  echo "          \"clean\": temporary files will be deleted."
  echo "          \"setup\": setup and install all the required tools."
  echo "         \"accept\": accept <name>... records the named benchmarks that the LATEST run"
  echo "                     quarantined in results/suspect_results.csv, without re-running them."
  echo "           \"help\": this help text is displayed."
  echo ""
  echo "Current configuration:"
  echo "  benchmarking: $raw_version"
  echo "  dogfooding: $benchmarker_version"
  echo "  qsv binary: $qsv_bin"
  echo "  qsv python binary: $qsv_py_bin (py benchmarks only)"
  echo "  qsv benchmarker binary: $qsv_benchmarker_bin"
  echo "  benchmark data: $data"
  echo "  benchmark data url: $benchmark_data_url"
  echo "  benchmark data zip: $datazip"
  echo "  warmup runs: $warmup_runs"
  echo "  benchmark runs: $benchmark_runs"
  exit 0
fi

# check if required tools/dependencies are installed ---------

# check if benchmarker_bin has the apply feature enabled
if [[ "$benchmarker_version" != *"apply;"* ]]; then
  echo "ERROR: $qsv_benchmarker_bin does not have the apply feature enabled."
  echo "The qsv apply command is needed to format the benchmarks results."
  exit 1
fi

# check if the benchmarker_bin has the luau feature enabled
if [[ "$benchmarker_version" != *"Luau"* ]]; then
  echo "ERROR: $qsv_benchmarker_bin does not have the luau feature enabled."
  echo "The qsv luau command is needed to aggregate the benchmarks results."
  exit 1
fi

# check if the benchmarker_bin has the polars feature enabled
if [[ "$benchmarker_version" != *"polars"* ]]; then
  echo "ERROR: $qsv_benchmarker_bin does not have the polars feature enabled."
  echo "The qsv sqlp command is needed to compute the delta & rank columns in the benchmark results."
  exit 1
fi

# check if the benchmarker_bin has the to feature enabled
if [[ "$benchmarker_version" != *"to;"* ]]; then
  # check if benchmark_data.xlsx exists
  if [ ! -r benchmark_data.xlsx ]; then
    echo "ERROR: $qsv_benchmarker_bin does not have the to feature enabled."
    echo "The qsv to xlsx command is needed to create an Excel spreadsheet"
    echo "as benchmark_data.xlsx does not exist."
    exit 1
  fi
fi

# check if the python-enabled qsv binary ($qsv_py_bin) is available and has the python
# feature. Unlike the checks above, this is a SOFT check - the python feature is not
# compiled into the regular prebuilt (PGO-optimized) qsv binaries (only the qsvpyNNN
# flavors), so we just skip the py benchmarks when it's absent instead of erroring out.
# The py benchmarks run with $qsv_py_bin while everything else uses the PGO-optimized
# $qsv_bin.
has_python=0
if command -v "$qsv_py_bin" &>/dev/null; then
  py_raw_version=$("$qsv_py_bin" --version)
  if [[ "$py_raw_version" == *"python-"* ]]; then
    has_python=1
  fi
fi
if [[ "$has_python" -eq 0 ]]; then
  echo "NOTE: $qsv_py_bin not found or lacks the python feature - skipping py benchmarks."
  echo "      Install a python-enabled binary (e.g. qsvpy314) to benchmark the py command."
else
  # The py_* results are recorded under $qsv_bin's version metadata, so the py binary's
  # base version must match $qsv_bin's or the Python rows would be misattributed to the
  # wrong qsv release. Skip the py benchmarks (rather than mislabel them) on a mismatch.
  qsv_base_version=$(echo "$raw_version" | cut -d' ' -f2 | cut -d'-' -f1)
  py_base_version=$(echo "$py_raw_version" | cut -d' ' -f2 | cut -d'-' -f1)
  if [[ "$qsv_base_version" != "$py_base_version" ]]; then
    has_python=0
    echo "NOTE: $qsv_py_bin version ($py_base_version) differs from $qsv_bin version ($qsv_base_version)"
    echo "      - skipping py benchmarks to avoid misattributing results to the wrong version."
  fi
fi

# check if $qsv_bin has the synthesize command. Like the python check above, this is a SOFT
# check - synthesize is feature-gated and is NOT compiled into every build (including some
# prebuilt binaries), so we skip the synthesize benchmark when it's absent instead of
# letting the full run record a failure for it.
has_synthesize=0
if "$qsv_bin" synthesize --help &>/dev/null; then
  has_synthesize=1
else
  echo "NOTE: $qsv_bin does not have the synthesize feature - skipping the synthesize benchmark."
  echo "      Build with the synthesize feature enabled to benchmark the synthesize command."
fi

# check if $qsv_bin has the readstat & writestat commands (both gated by the readstat feature).
# Also a SOFT check. The benchmarks' inputs are prepared by dogfooding writestat, so the
# benchmarker binary must have the feature too, or we skip the whole group.
has_readstat=0
if "$qsv_bin" readstat --help &>/dev/null && "$qsv_benchmarker_bin" writestat --help &>/dev/null; then
  has_readstat=1
else
  echo "NOTE: $qsv_bin or $qsv_benchmarker_bin does not have the readstat feature - skipping the"
  echo "      readstat & writestat benchmarks. Build both with the readstat feature enabled to run them."
fi

# check if $qsv_bin has the profile & viz commands. Also SOFT checks - both are feature-gated
# and profile in particular is NOT compiled into every prebuilt binary. An unknown command
# exits non-zero, so a missing one is detected here rather than published as an instant failure.
has_profile=0
if "$qsv_bin" profile --help &>/dev/null; then
  has_profile=1
else
  echo "NOTE: $qsv_bin does not have the profile feature - skipping the profile benchmark."
fi
has_viz=0
if "$qsv_bin" viz --help &>/dev/null; then
  has_viz=1
else
  echo "NOTE: $qsv_bin does not have the viz feature - skipping the viz benchmark."
fi

# check if the geoclient benchmarks can run. Another SOFT check: unlike every other benchmark,
# they call an online API (NYC's Geoclient), so they need a subscription key - or a self-hosted
# Geoclient in QSV_GEOCLIENT_API_URL, which needs none. Without either, every lookup fails.
has_geoclient=0
if "$qsv_bin" geocode --help &>/dev/null &&
  { [ -n "$QSV_GEOCLIENT_API_KEY" ] || [ -n "$QSV_GEOCLIENT_API_URL" ]; }; then
  has_geoclient=1
else
  echo "NOTE: $qsv_bin does not have the geocode feature, or neither QSV_GEOCLIENT_API_KEY nor"
  echo "      QSV_GEOCLIENT_API_URL is set - skipping the geoclient benchmarks."
fi

# set sevenz_bin to "7z" on Linux/Cygwin and "7zz" on macOS
if [[ "$OSTYPE" == "darwin"* ]]; then
  sevenz_bin=7zz
else
  sevenz_bin=7z
fi

# hyperfine 2.0 changed its CSV/JSON export formats, which this script reads, so 2.0 is our
# floor. hyperfine_too_old returns 0 (true) ONLY when an installed hyperfine can be
# positively identified as older - an unparseable version string is never treated as too
# old, so a future version-string change cannot block the benchmarks. Sets the global
# $hyperfine_version for error messages.
hyperfine_min_major=2
hyperfine_min_minor=0
function hyperfine_too_old {
  local raw major minor
  raw=$(hyperfine --version 2>/dev/null)
  # parsed with bash builtins rather than awk/cut ON PURPOSE: setup calls this BEFORE it
  # checks for (and installs) awk, so an awk dependency here would make a missing awk look
  # like an unparseable version and silently skip the upgrade.
  read -r _ hyperfine_version _ <<<"$raw"
  major=${hyperfine_version%%.*}
  minor=${hyperfine_version#*.}
  minor=${minor%%.*}
  [[ "$major" =~ ^[0-9]+$ && "$minor" =~ ^[0-9]+$ ]] || return 1
  ((major < hyperfine_min_major || (major == hyperfine_min_major && minor < hyperfine_min_minor)))
}

# if arg_pat is equal to "setup", setup and install all the required tools
if [[ "$arg_pat" == "setup" ]]; then

  need_sevenz=0
  need_hyperfine=0
  need_awk=0
  need_sed=0
  need_duckdb=0
  # set by any post-install verification that fails, so setup reports an honest exit status
  # instead of printing "All required tools installed!" over the top of its own warnings
  setup_failed=0

  # check if 7z is installed
  if ! command -v "$sevenz_bin" &>/dev/null; then
    need_sevenz=1
  fi

  # check if hyperfine is installed, and that it is new enough (2.0+ export formats).
  # need_hyperfine=2 means "installed but too old", which needs an upgrade, not an install
  # - `brew install` on an already-installed formula is a no-op, so without this a 1.18
  # user would bounce between the version error and a setup run that does nothing.
  if ! command -v hyperfine &>/dev/null; then
    need_hyperfine=1
  elif hyperfine_too_old; then
    need_hyperfine=2
  fi

  # check if awk is installed
  if ! command -v awk &>/dev/null; then
    need_awk=1
  fi

  # check if sed is installed
  if ! command -v sed &>/dev/null; then
    need_sed=1
  fi

  # check if duckdb is installed
  if ! command -v duckdb &>/dev/null; then
    need_duckdb=1
  fi

  # if all required tools are installed, exit
  if [[ "$need_sevenz" -eq 0 && "$need_hyperfine" -eq 0 && "$need_awk" -eq 0 && "$need_sed" -eq 0 && "$need_duckdb" -eq 0 ]]; then
    echo "> All required tools are installed..."
    exit
  fi

  # check if homebrew is installed, if not, install it
  # as we need it to install the required tools
  if ! command -v brew &>/dev/null; then
    echo -n "INFO: Homebrew could not be found. Do you wish to install? "
    read -p '(Requires sudo) (y/n)? ' -r CHOICE
    case "$CHOICE" in
    y | Y)
      echo "Installing brew. Please enter requested info when prompted."
      bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
      ;;
    n | N)
      echo "Not installing brew. Review https://brew.sh/"
      exit 1
      ;;
    *)
      echo 'Invalid entry (only y/n)'
      exit 1
      ;;
    esac
  fi

  # if 7z is not installed, install it
  if [[ "$need_sevenz" -eq 1 ]]; then
    echo "INFO: 7-Zip could not be found. Installing..."
    brew install 7zip
  fi

  # if hyperfine is not installed, install it; if it is too old, upgrade it
  if [[ "$need_hyperfine" -eq 1 ]]; then
    echo "INFO: hyperfine could not be found. Installing..."
    brew install hyperfine
  elif [[ "$need_hyperfine" -eq 2 ]]; then
    echo "INFO: hyperfine v$hyperfine_version is older than v$hyperfine_min_major.$hyperfine_min_minor.0. Upgrading..."
    brew upgrade hyperfine
  fi

  # `brew upgrade` only works on a brew-managed formula - a hyperfine installed from cargo,
  # apt or a downloaded tarball makes it exit non-zero - and the brew calls above are
  # unchecked, so confirm the outcome here. Without this the "All required tools installed!"
  # message below would claim a success that did not happen, and the too-old hyperfine would
  # only surface on the next run.
  if [[ "$need_hyperfine" -ne 0 ]]; then
    if ! command -v hyperfine &>/dev/null; then
      echo "WARNING: hyperfine is STILL not installed - the benchmarks cannot run without it."
      echo "         Install it manually: https://github.com/sharkdp/hyperfine#installation"
      setup_failed=1
    elif hyperfine_too_old; then
      echo "WARNING: hyperfine is STILL v$hyperfine_version (need v$hyperfine_min_major.$hyperfine_min_minor.0+)."
      echo "         If it was not installed with Homebrew, upgrade it with whatever installed it."
      setup_failed=1
    fi
  fi

  # if awk is not installed, install it
  if [[ "$need_awk" -eq 1 ]]; then
    echo "INFO: awk could not be found. Installing..."
    brew install gawk
  fi

  # if sed is not installed, install it
  if [[ "$need_sed" -eq 1 ]]; then
    echo "INFO: sed could not be found. Installing..."
    brew install gnu-sed
  fi

  # if duckdb is not installed, install it
  if [[ "$need_duckdb" -eq 1 ]]; then
    echo "INFO: duckdb could not be found. Installing..."
    brew install duckdb
  fi

  # a verification failure above must not be papered over by the success message, and setup
  # must exit non-zero so a caller chaining `./benchmarks.sh setup && ./benchmarks.sh` stops
  # here instead of running into the same error again.
  if [[ "$setup_failed" -eq 1 ]]; then
    echo "ERROR: setup did not complete - see the WARNING(s) above."
    exit 1
  fi

  echo "> All required tools installed! You can run ./benchmarks.sh now."
  exit
fi

# check if 7z is installed
if ! command -v "$sevenz_bin" &>/dev/null; then
  echo "ERROR: $sevenz_bin could not be found."
  echo "Please install 7-Zip v23.01 and above or run \"./benchmarks.sh setup\" to install it."
  exit 1
fi

# check if hyperfine is installed
if ! command -v hyperfine &>/dev/null; then
  echo "ERROR: hyperfine could not be found"
  echo "Please install hyperfine v2.0.0 and above or run \"./benchmarks.sh setup\" to install it."
  exit 1
fi

# hyperfine 2.0 changed the CSV and JSON exports this script reads (and added the peak memory
# measurement), so on 1.x every benchmark would be flagged as failed. Fail fast instead.
if hyperfine_too_old; then
  echo "ERROR: hyperfine v$hyperfine_version is too old (the 2.0 export formats require v$hyperfine_min_major.$hyperfine_min_minor.0+)"
  echo "Please run \"brew upgrade hyperfine\" or \"./benchmarks.sh setup\" to upgrade it."
  exit 1
fi

# check if awk is installed
if ! command -v awk &>/dev/null; then
  echo "ERROR: awk could not be found"
  echo "Please install awk or run \"./benchmarks.sh setup\" to install it."
  exit 1
fi

# check if sed is installed
if ! command -v sed &>/dev/null; then
  echo "ERROR: sed could not be found"
  echo "Please install sed or run \"./benchmarks.sh setup\" to install it."
  exit 1
fi

# check if duckdb is installed
if ! command -v duckdb &>/dev/null; then
  echo "ERROR: duckdb could not be found"
  echo "Please install duckdb or run \"./benchmarks.sh setup\" to install it."
  exit 1
fi

# qsv version metadata ----------------
version=$(echo "$raw_version" | cut -d' ' -f2 | cut -d'-' -f1)
# get target platform from version
platform=$(echo "$raw_version" | sed 's/.*(\([a-z0-9_-]*\) compiled with Rust.*/\1/')
# get qsv kind
kind=$(echo "$raw_version" | sed 's/.* \([a-zA-Z-]*\)$/\1/')

# get num cores & memory size (mem_size is total physical memory in bytes on every platform)
if [[ "$OSTYPE" == "darwin"* ]]; then
  # macOS
  num_cores=$(sysctl -n hw.ncpu)
  mem_size=$(sysctl -n hw.memsize)
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
  # Linux - column 2 of /Mem/ in `free -b` is total memory in bytes
  num_cores=$(nproc)
  mem_size=$(free -b | awk '/Mem/ {print $2}')
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
  # Windows - prefer PowerShell since wmic is deprecated/removed on recent
  # Windows 11 / Server 2025 builds. Fall back to wmic on legacy systems.
  # TotalPhysicalMemory is reported in bytes.
  if command -v powershell &>/dev/null; then
    num_cores=$(powershell -NoProfile -Command "(Get-CimInstance Win32_Processor | Measure-Object -Property NumberOfLogicalProcessors -Sum).Sum" | tr -d '\r')
    mem_size=$(powershell -NoProfile -Command "(Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory" | tr -d '\r')
  else
    num_cores=$(wmic cpu get NumberOfLogicalProcessors | grep -Eo '^[0-9]+')
    mem_size=$(wmic computersystem get TotalPhysicalMemory | grep -Eo '[0-9]+')
  fi
else
  echo "Unsupported operating system: $OSTYPE"
  exit 1
fi

function cleanup_files {
  # Clean up temporary files
  rm -f "$filestem".csv.*
  rm -f "$filestem".stats.*
  rm -f results/hf_result.csv
  rm -f results/hf_result.json
  rm -f results/hf_row.csv
  rm -f results/history_baseline.csv
  rm -f results/verify_stdout.csv
  rm -f results/suspect_work.csv
  rm -f results/suspect_entry.csv
  rm -f results/accept_rows.csv
  rm -f results/results_work.csv
  rm -f results/run_info_work.tsv
  rm -f results/entry.csv
  rm -r -f split_tempdir
  rm -r -f split_tempdir_chunks
  rm -r -f split_tempdir_kbs
  rm -r -f split_tempdir_idx
  rm -r -f split_tempdir_idx_j1
  rm -r -f split_tempdir_chunks_idx
  rm -r -f split_tempdir_chunks_idx_j1
  rm -f template_temp.txt
  rm -f benchmark_work.*
  rm -r -f benchmark_work
  rm -r -f benchmark_work_parquet
  rm -f extsort_sorted.csv
  rm -r -f partitioned
}

# ---------------------------------------
# Finalize benchmark results. Sort the latest results by version, tstamp & name.
# compute and add records per second for each benchmark using qsv's luau command.
# We compute recs_per_sec by dividing the number of rows in the benchmark data
# by the mean run time of the three runs.
# We then append/concatenate the latest results to benchmark_results.csv - which is
# a historical archive, so we can track performance over multiple releases.

# It is a function so `accept` can re-run it for the quarantined rows it moves into the
# results. It reads results/latest_results.csv (the new rows) and uses $version, $now and
# $bench_rowcounts.
function finalize_results {
  echo ""
  # sort the benchmark results by version, tstamp & name
  "$qsv_bin" sort --select version,tstamp,name results/latest_results.csv \
    -o results/results_work.csv

  # compute records per second for each benchmark using luau by dividing that benchmark's
  # input rowcount by its mean
  # we then round the result to a whole number. We also compute the total mean

  # we set the QSVBM_ROWCOUNT environment variable to the rowcount so it can be used
  # by the luau script by using the qsv.get_env() function. This is the DEFAULT divisor,
  # used by every benchmark that reads the full dataset.
  export QSVBM_ROWCOUNT=$rowcount
  # a handful of benchmarks read a SUBSET of the benchmark data, so dividing by the full
  # rowcount would overstate their throughput (qsv issue #4598). QSVBM_ROWCOUNTS carries
  # their per-benchmark rowcounts as a flat "name=rows;name=rows;" string, keyed on the
  # benchmark name - which is already a column in the results CSV, so no new column is
  # needed. Benchmarks absent from the map use QSVBM_ROWCOUNT as before.
  export QSVBM_ROWCOUNTS="$bench_rowcounts"
  if [ -n "$bench_rowcounts" ]; then
    printf "  Subset-input recs_per_sec divisors: %s\n" "$bench_rowcounts"
  fi
  # we run the benchmark_aggregations.luau script using qsv's luau command
  # total_mean is the total mean of all the benchmarks
  # it is computed in the END block of the script and is sent to stderr
  # which we redirect to a file named total_mean.txt
  "$qsv_benchmarker_bin" luau map recs_per_sec "file:benchmark_aggregations.luau" \
    results/results_work.csv -o results/latest_results.csv 2>total_mean.txt
  # we read the total_mean from the total_mean.txt file
  total_mean=$(<total_mean.txt)

  # Concatenate the final results of this run to results/bechmark_results.csv
  "$qsv_bin" cat rowskey results/latest_results.csv results/benchmark_results.csv \
    -o results/results_work.csv
  mv results/results_work.csv results/benchmark_results.csv

  # ---------------------------------------
  # Add delta & rank columns over the full historical archive by dogfooding qsv sqlp.
  #   delta = percent FASTER than this benchmark's previous version (the most recent earlier
  #           version, by tstamp; same-version re-runs are collapsed to their latest run).
  #           Positive = faster, negative = regression. Empty when there is no earlier version.
  #           tstamp is only hour-precise, so a version's latest tstamp can hold several rows
  #           (two re-runs in the same hour, or - from v3.8.0 to v6.1.1 of this script - the
  #           same command queued twice as luau_filter_no_globals_colidx). Those ties collapse
  #           to their FASTEST mean: deterministic (a row_number() pick was not, so every
  #           recompute could flip deltas) and robust to one noisy copy of the run.
  #           The final ORDER BY breaks the same ties on mean, so row order is stable too.
  #   rank  = speed rank of this run among ALL runs of the same benchmark (1 = fastest),
  #           counting runs within $rank_tie_pct% as ties: 1 + the number of runs whose mean
  #           is more than $rank_tie_pct% faster. A plain rank() made flat benchmarks look like
  #           regressions: their ~60 historical runs sit within a few percent, so a 0.8% slip
  #           moved validate_no_schema_index from rank 15 to 27.
  #   slower than best (%) = how much slower this run's mean is than the fastest run ever
  #           recorded for the benchmark (0 = it is the fastest). Unlike rank it has no
  #           cliffs: a 1% slip reads as about 1%, however many runs sit nearby.
  # We recompute over the whole archive on every run - this is self-healing: it backfills the
  # columns for pre-existing historical rows the first time, and keeps ranks correct as new runs
  # accrue. "user" is a reserved word in Polars SQL, so it is quoted as b."user". Base columns are
  # listed explicitly (not b.*) so re-runs whose input already has delta/rank drop them and
  # recompute cleanly, without duplicating columns.
  # We dogfood the benchmarker binary (not the binary under test) for sqlp, since the binary
  # under test may be a variant without the polars feature (e.g. qsvlite).
  "$qsv_benchmarker_bin" sqlp results/benchmark_results.csv -q "
  with latest as (
    select name, version, max(tstamp) as tstamp
    from _t_1
    group by name, version
  ),
  per_version as (
    select b.name, b.version, b.tstamp, min(b.mean) as mean
    from _t_1 b
    join latest l on b.name = l.name and b.version = l.version and b.tstamp = l.tstamp
    group by b.name, b.version, b.tstamp
  ),
  with_prev as (
    select name, version,
           lag(mean) over (partition by name order by tstamp asc) as prev_mean
    from per_version
  ),
  ranked as (
    select x.name, x.version, x.tstamp, x.mean,
           1 + sum(case when y.mean < x.mean * (1 - $rank_tie_pct / 100.0) then 1 else 0 end) as rank
    from _t_1 x
    join _t_1 y on x.name = y.name
    group by x.name, x.version, x.tstamp, x.mean
  ),
  best as (
    select name, min(mean) as best_mean
    from _t_1
    group by name
  )
  select b.version, b.tstamp, b.name, b.mean, b.stddev, b.median,
         b.\"user\", b.system, b.min, b.max, b.recs_per_sec,
         round(((wp.prev_mean - b.mean) / wp.prev_mean) * 100, 2) as \"delta (%)\",
         rk.rank as rank,
         round(((b.mean - bs.best_mean) / bs.best_mean) * 100, 2) as \"slower than best (%)\",
         b.peak_mem_mib
  from _t_1 b
  left join with_prev wp on b.name = wp.name and b.version = wp.version
  left join ranked rk on b.name = rk.name and b.version = rk.version
    and b.tstamp = rk.tstamp and b.mean = rk.mean
  left join best bs on b.name = bs.name
  order by b.tstamp desc, b.name asc, b.mean asc
  " -o results/results_work.csv
  mv results/results_work.csv results/benchmark_results.csv

  # re-derive latest_results.csv from the enriched archive so it too carries delta & rank
  # (total_mean was already captured above, so overwriting latest_results.csv here is safe)
  "$qsv_benchmarker_bin" sqlp results/benchmark_results.csv \
    -q "select * from _t_1 where version = '$version' and tstamp = '$now'" \
    -o results/results_work.csv
  "$qsv_bin" sort --select version,tstamp,name results/results_work.csv \
    -o results/latest_results.csv

  # make "display" versions of the results
  # i.e. number of decimal places is reduced to 3, and column order is changed so it's easier to read
  # with recs_per_sec moved from the back after mean followed by the rest of the stats columns

  # first - for benchmark_results_display.csv, move the recs_per_sec column after the
  # mean column using the `qsv select` command
  "$qsv_bin" select 'version,tstamp,name,mean,recs_per_sec,"delta (%)",rank,"slower than best (%)",stddev,median,user,system,min,max,peak_mem_mib' \
    results/benchmark_results.csv -o results/benchmark_results_display.csv

  # then, round the stats columns to 3 decimal places using the `qsv apply operations round` command
  # it defaults to 3 decimal places if the --formatstr option is not specified
  "$qsv_benchmarker_bin" apply operations round mean,stddev,median,user,system,min,max,peak_mem_mib \
    results/benchmark_results_display.csv -o results/results_work.csv
  mv results/results_work.csv results/benchmark_results_display.csv

  # do the same for latest_results_display.csv
  "$qsv_bin" select 'version,tstamp,name,mean,recs_per_sec,"delta (%)",rank,"slower than best (%)",stddev,median,user,system,min,max,peak_mem_mib' \
    results/latest_results.csv -o results/latest_results_display.csv
  "$qsv_benchmarker_bin" apply operations round mean,stddev,median,user,system,min,max,peak_mem_mib \
    results/latest_results_display.csv -o results/results_work.csv
  mv results/results_work.csv results/latest_results_display.csv
}

# if arg_pat is equal to "reset", download and prepare the benchmark data again
# the results/benchmark_results.csv historical archive will be preserved
if [[ "$arg_pat" == "reset" ]]; then
  rm -f "$datazip"
  rm -f "$filestem".*
  rm -f communityboards.csv
  rm -f geo_data.csv
  rm -f ods_data.csv
  rm -f pragmastats_50kdata.csv*
  rm -f fwf_100kdata.txt
  rm -f stat_100kdata*
  # the decompressed geoclient inputs only - NOT the committed .gz, and NOT the primed
  # geoclient_cache/ (re-priming costs ~45 minutes and ~100k API calls; rm -r it to re-prime)
  rm -f nyc_addresspoint_100k.csv geoclient_10k.csv
  rm -f data_to_exclude.csv
  rm -f data_unsorted.csv
  rm -f data_sorted.csv
  rm -f benchmark_data.xlsx
  rm -f benchmark_data.jsonl
  rm -f benchmark_data.json
  rm -f benchmark_data.snappy
  rm -f benchmark_data.parquet
  rm -f benchmark_data.csv.schema.json benchmark_data.csv.schema.json.tmp
  rm -f searchset_patterns.txt
  rm -f searchset_patterns_unicode.txt
  echo "> Benchmark data reset..."
  echo "  Historical benchmarks archive preserved in results/benchmark_results.csv"
  exit
fi

# if arg_pat is equal to "clean", clean up temporary files
if [[ "$arg_pat" == "clean" ]]; then
  cleanup_files
  echo "> Temporary files cleaned up..."
  exit
fi

# if arg_pat is equal to "accept", record the named benchmarks that a run quarantined (see
# quarantine_benchmark) WITHOUT re-running them. A one-benchmark rerun would not do: a pattern
# run rebuilds latest_results.csv from its own rows only, and tstamp is hour-precise, so the
# published latest results would shrink to that one benchmark. The quarantined rows keep the
# version, tstamp & logtime of the run that measured them, so finalize_results merges them back
# into it.
if [[ "$arg_pat" == "accept" ]]; then
  shift
  if [ "$#" -eq 0 ]; then
    echo "Usage: ./benchmarks.sh accept <benchmark name>..."
    exit 1
  fi
  if [ ! -s results/suspect_results.csv ]; then
    echo "ERROR: there are no quarantined benchmarks (results/suspect_results.csv is missing or empty)."
    exit 1
  fi
  for accept_name in "$@"; do
    if [[ ! "$accept_name" =~ ^[A-Za-z0-9_]+$ ]]; then
      echo "ERROR: \"$accept_name\" is not a benchmark name."
      exit 1
    fi
  done
  # only the LATEST run's quarantined rows can be accepted, so latest_results.csv and
  # latest_run_info.tsv keep describing the same run. A run is identified by its
  # second-precision logtime (tstamp is only hour-precise), which quarantine_benchmark stores
  # with each row and run_info records for the run.
  if [ ! -s results/latest_run_info.tsv ]; then
    echo "ERROR: results/latest_run_info.tsv is missing, so the latest run is unknown."
    exit 1
  fi
  IFS=$'\t' read -r version now logtime _ < <(sed -n 2p results/latest_run_info.tsv)
  accept_names=" $* "
  # suspect_results.csv columns: version,tstamp,logtime,name,rowcount,reason,... - the first
  # five never contain a comma, so whole lines can be selected by them
  awk -F, -v lt="$logtime" -v names="$accept_names" \
    'NR == 1 || ($3 == lt && index(names, " " $4 " "))' results/suspect_results.csv \
    >results/accept_rows.csv
  for accept_name in "$@"; do
    if ! awk -F, -v n="$accept_name" 'NR > 1 && $4 == n { found = 1 } END { exit !found }' results/accept_rows.csv; then
      echo "ERROR: $accept_name was not quarantined by the latest run (qsv $version, $logtime)."
      echo "  Only the latest run's quarantined results can be accepted."
      rm -f results/accept_rows.csv
      exit 1
    fi
  done
  # recs_per_sec divides by the rowcount stored with each row (subset benchmarks have their own)
  bench_rowcounts=$(awk -F, 'NR > 1 { printf "%s=%s;", $4, $5 }' results/accept_rows.csv)
  rowcount=$(awk -F, 'NR == 2 { print $5 }' results/accept_rows.csv)
  # finalize_results does not stop on errors, and the quarantined rows are the only copy of these
  # measurements. So back up the files it rewrites, count each name's rows in the archive for this
  # version & tstamp (hour-precise, so an earlier same-hour row may already be there), and only
  # keep the result - and drop the rows from quarantine - if every count grew by exactly the rows
  # accepted for that name. Otherwise restore the backups.
  finalized_files=(benchmark_results.csv benchmark_results_display.csv latest_results.csv latest_results_display.csv)
  for f in "${finalized_files[@]}"; do
    [ -f "results/$f" ] && cp "results/$f" "results/$f.accept_bak"
  done
  archive_rows() {
    awk -F, -v v="$version" -v t="$now" -v n="$1" '$1 == v && $2 == t && $3 == n { c++ } END { print c + 0 }' \
      results/benchmark_results.csv
  }
  # indexed, aligned with "$@": associative arrays need bash 4, and macOS's /bin/bash is 3.2
  rows_before=()
  for accept_name in "$@"; do
    rows_before+=("$(archive_rows "$accept_name")")
  done
  "$qsv_bin" select 'version,tstamp,name,mean,stddev,median,user,system,min,max,peak_mem_mib' \
    results/accept_rows.csv -o results/latest_results.csv
  echo "> Accepting $* into the qsv $version run of $logtime..."
  finalize_results
  accept_idx=0
  for accept_name in "$@"; do
    expected=$((rows_before[accept_idx] + $(awk -F, -v n="$accept_name" 'NR > 1 && $4 == n { c++ } END { print c + 0 }' results/accept_rows.csv)))
    if [ "$(archive_rows "$accept_name")" -ne "$expected" ]; then
      echo "ERROR: $accept_name did not reach results/benchmark_results.csv. The results were restored,"
      echo "  and the quarantined rows were kept. See the errors above."
      for f in "${finalized_files[@]}"; do
        [ -f "results/$f.accept_bak" ] && mv "results/$f.accept_bak" "results/$f"
      done
      rm -f results/accept_rows.csv
      exit 1
    fi
    ((accept_idx++))
  done
  for f in "${finalized_files[@]}"; do
    rm -f "results/$f.accept_bak"
  done
  awk -F, -v lt="$logtime" -v names="$accept_names" \
    'NR == 1 || !($3 == lt && index(names, " " $4 " "))' results/suspect_results.csv \
    >results/suspect_work.csv
  mv results/suspect_work.csv results/suspect_results.csv

  # the run's total_mean now includes the accepted benchmarks: add their means, rounded to 3
  # places as the luau script does, to the run's recorded total_mean, in both run info files.
  # (Re-summing latest_results.csv would not do: two runs of a version in the same hour share a
  # tstamp, so it can also hold the earlier run's rows.)
  total_mean=$(awk -F'\t' 'NR == 1 { for (i = 1; i <= NF; i++) if ($i == "total_mean") c = i; next }
    NR == 2 && c { print $c }' results/latest_run_info.tsv)
  total_mean=$(awk -F, -v base="${total_mean:-0}" 'NR == 1 { for (i = 1; i <= NF; i++) if ($i == "mean") c = i; next }
    { t += sprintf("%.3f", $c) } END { printf "%.3f", base + t }' results/accept_rows.csv)
  for run_info in results/latest_run_info.tsv results/run_info_history.tsv; do
    awk -F'\t' -v OFS='\t' -v lt="$logtime" -v tm="$total_mean" \
      'NR == 1 { for (i = 1; i <= NF; i++) if ($i == "total_mean") c = i; print; next }
       $3 == lt && c { $c = tm } { print }' "$run_info" >results/run_info_work.tsv
    mv results/run_info_work.tsv "$run_info"
  done
  rm -f results/accept_rows.csv results/results_work.csv total_mean.txt
  echo "> Done. results/latest_results.csv now holds the qsv $version run of $logtime, including $*."
  echo "  Its total mean is now $total_mean."
  exit
fi

echo "> Setting up Benchmark environment..."
SECONDS=0

cleanup_files

if [ ! -r "$data" ]; then
  echo "> Downloading Benchmark data..."
  if ! curl --fail -sS "$benchmark_data_url" -o "$datazip"; then
    echo "ERROR: Failed to download benchmark data from $benchmark_data_url"
    rm -f "$datazip"
    exit 1
  fi
  if ! "$sevenz_bin" e -y "$datazip"; then
    echo "ERROR: Failed to extract benchmark data from $datazip"
    rm -f "$datazip"
    exit 1
  fi
  echo ""
fi

# we get the rowcount, just in case the benchmark data was modified by the user to tailor
# the benchmark to their system/workload. We use the rowcount to compute records per second.
# This is the DEFAULT divisor; the few benchmarks that read a subset of the data get their
# own divisor - see rowcount_for_benchmark() further below.
rowcount=$("$qsv_bin" count --no-polars "$data")
printf "  Benchmark data rowcount: %'.0f\n" "$rowcount"
qsv_absolute_path=$(which "$qsv_bin")
benchmarker_absolute_path=$(which "$qsv_benchmarker_bin")
printf "  Benchmarking qsv binary: %s\n" "$qsv_absolute_path"
printf "  %s\n" "$raw_version"
printf "  Dogfooding qsv binary: %s\n" "$benchmarker_absolute_path"
echo ""

if [ ! -r communityboards.csv ]; then
  echo "> Downloading community board data..."
  if ! curl --fail -sS https://raw.githubusercontent.com/wiki/dathere/qsv/files/communityboards.csv -o communityboards.csv; then
    echo "ERROR: Failed to download communityboards.csv"
    rm -f communityboards.csv
    exit 1
  fi
  echo ""
fi

echo "> Preparing benchmark support data..."
# create an index so benchmark data preparation commands can run faster
if [ ! -f "$data.idx" ]; then
  "$qsv_benchmarker_bin" index "$data"
fi

if [ ! -r data_to_exclude.csv ]; then
  echo "   data_to_exclude.csv..."
  "$qsv_benchmarker_bin" sample --seed 42 1000 "$data" -o data_to_exclude.csv
fi

if [ ! -r data_unsorted.csv ]; then
  echo "   data_unsorted.csv..."
  "$qsv_benchmarker_bin" sort --seed 42 --random --faster "$data" -o data_unsorted.csv
fi

if [ ! -r data_sorted.csv ]; then
  echo "   data_sorted.csv..."
  "$qsv_benchmarker_bin" sort "$data" -o data_sorted.csv
fi

if [ ! -r benchmark_data.xlsx ]; then
  echo "   benchmark_data.xlsx..."
  "$qsv_benchmarker_bin" to xlsx benchmark_data.xlsx "$data"
fi

if [ ! -r benchmark_data.jsonl ]; then
  echo "   benchmark_data.jsonl..."
  "$qsv_benchmarker_bin" tojsonl "$data" --output benchmark_data.jsonl
fi

if [ ! -r benchmark_data.json ]; then
  echo "   benchmark_data.json..."
  "$qsv_benchmarker_bin" sqlp --format json "$data" -q 'select * from _t_1' --infer-len 127000 --rnull-values 'N/A' --output benchmark_data.json
fi

if [ ! -r benchmark_data.csv.schema.json ]; then
  echo "   benchmark_data.csv.schema.json..."
  # write to a temp file and rename on success: an interrupted run must not leave a truncated
  # schema behind, as the -r check above would keep it. validate then fails with exit 1, which
  # the --expect-exit 1 validate --valid-output benchmarks would accept as a successful timing.
  if "$qsv_benchmarker_bin" schema "$data" --stdout >benchmark_data.csv.schema.json.tmp; then
    mv benchmark_data.csv.schema.json.tmp benchmark_data.csv.schema.json
  else
    rm -f benchmark_data.csv.schema.json.tmp
    echo "Failed to generate benchmark_data.csv.schema.json" >&2
    exit 1
  fi
fi

if [ ! -r benchmark_data.snappy ]; then
  echo "   benchmark_data.snappy..."
  "$qsv_benchmarker_bin" snappy compress "$data" --output benchmark_data.snappy
fi

# geoconvert errors out on rows with empty coordinates, so prepare a filtered file with
# only rows that have BOTH a non-empty Latitude and Longitude (search '.' matches any
# non-empty field). Used by the geoconvert benchmark.
if [ ! -r geo_data.csv ]; then
  echo "   geo_data.csv..."
  "$qsv_benchmarker_bin" search -s Latitude '.' "$data" |
    "$qsv_benchmarker_bin" search -s Longitude '.' -o geo_data.csv
fi

# `to ods` writes the whole sheet as a single content.xml entry inside the ODS zip
# archive. The full 1M-row dataset overflows the 4GB ZIP (non-ZIP64) limit, which the
# underlying spreadsheet-ods crate does not handle, so we benchmark `to ods` on a
# 500k-row subset that stays comfortably under the limit. Used by the to_ods benchmark,
# whose recs_per_sec divides by this subset's own rowcount.
if [ ! -r ods_data.csv ]; then
  echo "   ods_data.csv..."
  "$qsv_benchmarker_bin" slice -l 500000 "$data" -o ods_data.csv
fi

if [ ! -r pragmastats_50kdata.csv ]; then
  echo "   pragmastats_50kdata.csv..."
  "$qsv_benchmarker_bin" slice -l 50000 "$data" -o pragmastats_50kdata.csv
fi

# `table --align leftfwf` pads every row to each column's widest value, so the full 1M-row
# dataset would balloon to ~2.2 GB of mostly spaces. Like to_ods, the fixedwidth benchmark reads
# a subset instead - 100k rows, ~220 MB - and divides by its own rowcount.
if [ ! -r fwf_100kdata.txt ]; then
  echo "   fwf_100kdata.txt..."
  "$qsv_benchmarker_bin" slice -l 100000 "$data" |
    "$qsv_benchmarker_bin" table --align leftfwf -o fwf_100kdata.txt
fi

# the parquet input for sqlp_parquet_input, so it can be compared with the CSV-reading
# sqlp_aggregations benchmark running the same query
if [ ! -r benchmark_data.parquet ]; then
  echo "   benchmark_data.parquet..."
  "$qsv_benchmarker_bin" sqlp "$data" -q --format parquet --infer-len 100000 'select * from _t_1' --output benchmark_data.parquet
fi

# Measure the subset inputs' rowcounts, so recs_per_sec divides by the rows a benchmark
# ACTUALLY read instead of the full dataset's $rowcount (qsv issue #4598).
# These MUST be measured unconditionally, OUTSIDE the `if [ ! -r ... ]` prep guards above:
# the subsets survive between runs (cleanup_files does not delete them - only `reset` does),
# so on a re-run the prep blocks are skipped and an inside-the-guard count would silently
# yield an empty value. Like $rowcount above, we measure rather than assume, in case the
# user tailored the benchmark data to their system/workload.
geo_rowcount=$("$qsv_benchmarker_bin" count --no-polars geo_data.csv)
ods_rowcount=$("$qsv_benchmarker_bin" count --no-polars ods_data.csv)
pragma_rowcount=$("$qsv_benchmarker_bin" count --no-polars pragmastats_50kdata.csv)
# the fixedwidth input is not CSV, so count the records it converts to
fwf_rowcount=$("$qsv_benchmarker_bin" fixedwidth fwf_100kdata.txt | "$qsv_benchmarker_bin" count --no-polars)

# readstat & writestat inputs. writestat refuses the NYC 311 column names for SPSS & Stata
# (they hold spaces), so the CSV is run through safenames first. A refusal exits instantly, and
# as hyperfine runs with -i, it would be published as a plausible timing. That is also why SPSS
# portable (.por) & SAS transport v5 (.xpt5) aren't benchmarked: they need names of at most
# 8 bytes, which safenames doesn't give. A 100k-row subset keeps writestat affordable - a
# full-size write takes 1.5 to 10 minutes per run. The readstat inputs are written from it too.
stat_rowcount=
if [[ "$has_readstat" -eq 1 ]]; then
  if [ ! -r stat_100kdata.csv ]; then
    echo "   stat_100kdata.csv..."
    "$qsv_benchmarker_bin" slice -l 100000 "$data" |
      "$qsv_benchmarker_bin" safenames -o stat_100kdata.csv
  fi
  for statfile in stat_100kdata.sav stat_100kdata.dta stat_100kdata.xpt; do
    if [ ! -r "$statfile" ]; then
      echo "   $statfile..."
      "$qsv_benchmarker_bin" writestat stat_100kdata.csv -o "$statfile"
    fi
  done
  if [ ! -r stat_100kdata_compressed.sav ]; then
    echo "   stat_100kdata_compressed.sav..."
    "$qsv_benchmarker_bin" writestat --compress stat_100kdata.csv -o stat_100kdata_compressed.sav
  fi
  stat_rowcount=$("$qsv_benchmarker_bin" count --no-polars stat_100kdata.csv)
fi

# geoclient inputs. nyc_addresspoint_100k.csv.gz is committed next to this script: 100,000
# distinct real NYC addresses, 20,000 per borough, sampled (seed 4242) from NYC Open Data's
# AddressPoint dataset (uf93-f8nk, fetched 2026-10-31) after dropping rows without a ZIP code
# or BIN and keeping one address point per house number + street + borough. Its bin column is
# the address point's own BIN, to check geocoding accuracy against.
# The public Geoclient API allows 2,500 requests/minute and 500,000/day, so an uncached run
# of all 100k rows takes over 40 minutes - and 2 warmup + 3 timed runs would spend the whole
# daily quota. So the uncached benchmark reads a 10k-row sample (~4 minutes a run), and only
# the cached one reads all 100k rows, from a result cache primed below.
# The cache lives in its own directory, set through QSV_CACHE_DIR (which overrides --cache-dir),
# so it never touches the user's ~/.qsv-cache. --cache-ttl 0 keeps its entries from expiring
# between benchmark runs (the default TTL is 14 days). Non-matches are cached too.
# QSV_DOTENV_PATH='<NONE>' turns off qsv's .env loading for these commands: a .env file is
# loaded with OVERRIDE, so one setting QSV_CACHE_DIR or QSV_GEOCLIENT_API_KEY would undo both
# the cache isolation and the offline guard below.
# The cached benchmark runs with an INVALID key (QSV_GEOCLIENT_API_KEY=offline-guard): the cache
# key doesn't include the API key, so cached rows are served as usual, but any row that needs
# the network fails with a fatal HTTP 401 and the run exits non-zero - so a cached result that
# made API calls is never recorded. (A self-hosted Geoclient needs no key, so there the guard
# can't tell; its requests stay local.)
geoclient_offline_key=offline-guard
geoclient_params='houseNumber={house_number},street={street},borough={borough}'
geoclient_cache_dir=geoclient_cache
geoclient_10k_rowcount=
geoclient_100k_rowcount=
has_geoclient_cache=0
if [[ "$has_geoclient" -eq 1 ]]; then
  if [ ! -r nyc_addresspoint_100k.csv ]; then
    echo "   nyc_addresspoint_100k.csv..."
    gzip -dc nyc_addresspoint_100k.csv.gz >nyc_addresspoint_100k.csv
  fi
  if [ ! -r geoclient_10k.csv ]; then
    echo "   geoclient_10k.csv..."
    "$qsv_benchmarker_bin" sample 10000 --seed 4242 nyc_addresspoint_100k.csv -o geoclient_10k.csv
  fi
  geoclient_10k_rowcount=$("$qsv_benchmarker_bin" count --no-polars geoclient_10k.csv)
  geoclient_100k_rowcount=$("$qsv_benchmarker_bin" count --no-polars nyc_addresspoint_100k.csv)

  # Prime the cache on EVERY run, with the exact request the cached benchmark sends (the cache
  # key is built from the endpoint and its params). On a warm cache this is one quick offline
  # pass; on a cold one - first run, or a qsv release that changed the cache file - it makes up
  # to 100k API calls. Without it, a cold cache would be timed as a "cached" run. It only runs
  # when the cached benchmark is selected, so a pattern run of other benchmarks never spends the
  # time or the API quota.
  # Lookups that fail transiently (timeouts, 5xx, 400s) are not cached, and the run still exits
  # 0 below the circuit breaker's threshold, so after each pass the offline guard checks that
  # every row is cached. A second pass fetches only the gaps; if that still leaves some, the
  # cached benchmark is skipped rather than timed with live requests in it.
  if [[ "geocode_geoclient_cache" == *"$arg_pat"* ]]; then
    echo "   priming the geoclient cache (a cold cache takes ~45 minutes and ~100k API calls)..."
    geoclient_prime_start=$SECONDS
    for _ in 1 2; do
      env QSV_DOTENV_PATH='<NONE>' QSV_CACHE_DIR="$geoclient_cache_dir" "$qsv_bin" geocode \
        geoclient address --endpoint address --params "$geoclient_params" -c geo_latlong \
        --cache-ttl 0 nyc_addresspoint_100k.csv -o /dev/null
      if env QSV_DOTENV_PATH='<NONE>' QSV_CACHE_DIR="$geoclient_cache_dir" \
        QSV_GEOCLIENT_API_KEY="$geoclient_offline_key" "$qsv_bin" geocode geoclient address \
        --endpoint address --params "$geoclient_params" -c geo_latlong --cache-ttl 0 \
        nyc_addresspoint_100k.csv -o /dev/null 2>/dev/null; then
        has_geoclient_cache=1
        break
      fi
    done
    if [[ "$has_geoclient_cache" -eq 1 ]]; then
      echo "   geoclient cache primed in $((SECONDS - geoclient_prime_start))s."
    else
      echo "NOTE: the geoclient cache is still missing rows after 2 priming passes - skipping the"
      echo "      geocode_geoclient_cache benchmark."
    fi
  fi
fi

# Look up the recs_per_sec divisor for a benchmark BY NAME. Most benchmarks read the full
# dataset and use $rowcount; the few that read a prepared subset are listed here. Sets the
# global `subset_rowcount` to that subset's rowcount, or "" meaning "use the full $rowcount".
#
# Keyed on the benchmark name - a fixed, controlled literal compared for EQUALITY - rather
# than by inspecting the command for input filenames. Filename inspection was tried and
# produced real collisions in both directions, because $data is configurable while the
# subset filenames are fixed:
#   - a $data that is a SUFFIX of a subset name (data.csv, a.csv) substring-matched all
#     three subsets, so every subset benchmark silently took the full-dataset divisor;
#   - a $data that CONTAINS a subset name (mygeo_data.csv, fixtures/geo_data.csv - note
#     `basename` above, so paths are supported) gave full-dataset commands a subset divisor.
# $data no longer takes part in the decision, so neither collision is expressible, and the
# "full dataset wins" rule is now structural: a full-dataset benchmark is simply absent here.
#
# The accepted cost is upkeep: a new subset benchmark must be added below. That is the same
# obligation the prep section's note already carries, and the divisor map is echoed on every
# run (see QSVBM_ROWCOUNTS below), so a missing entry is visible.
# IF YOU ADD A BENCHMARK THAT READS A SUBSET INPUT, ADD ITS NAME HERE.
function rowcount_for_benchmark {
  case "$1" in
  pragmastat_50k | pragmastat_50k_twosample | pragmastat_50k_index)
    subset_rowcount="$pragma_rowcount"
    ;;
  geoconvert_csv2geojsonl) subset_rowcount="$geo_rowcount" ;;
  to_ods) subset_rowcount="$ods_rowcount" ;;
  fixedwidth_100k) subset_rowcount="$fwf_rowcount" ;;
  geocode_geoclient_nocache) subset_rowcount="$geoclient_10k_rowcount" ;;
  geocode_geoclient_cache) subset_rowcount="$geoclient_100k_rowcount" ;;
  readstat_sav | readstat_sav_compressed | readstat_dta | readstat_dta_select | readstat_xpt | \
    writestat_sav | writestat_sav_compress | writestat_dta | writestat_xpt)
    subset_rowcount="$stat_rowcount"
    ;;
  *) subset_rowcount= ;;
  esac
}

# "name=rowcount;" pairs for benchmarks whose input is NOT the full $data, exported to
# benchmark_aggregations.luau as QSVBM_ROWCOUNTS. Benchmark names are [a-z0-9_] only
# (no "=" or ";"), so this flat format needs no escaping.
bench_rowcounts=""

if [ ! -r searchset_patterns.txt ]; then
  echo "   searchset_patterns.txt..."
  printf "homeless\npark\nNoise\n" >searchset_patterns.txt
fi

if [ ! -r searchset_patterns_unicode.txt ]; then
  echo "   searchset_patterns_unicode.txt..."
  printf "homeless\n💩\nNoise\n" >searchset_patterns_unicode.txt
fi

echo ""

schema=benchmark_data.csv.schema.json
# dynenum_schema is a hand-curated fixture checked into scripts/. Unlike $schema above, it
# is NOT auto-regenerated during prep — it tweaks the auto-generated schema with dynamicEnum
# constraints to exercise that validate code path. Edit the file by hand if the dataset changes.
dynenum_schema=benchmark_data-dynenum.csv.schema.json

commands_without_index=()
commands_without_index_name=()
commands_without_index_conclude=()
commands_without_index_prepare=()
commands_without_index_cleanup=()
commands_with_index=()
commands_with_index_name=()
commands_with_index_conclude=()
commands_with_index_prepare=()
commands_with_index_cleanup=()
commands_without_index_expect=()
commands_with_index_expect=()
commands_without_index_verify=()
commands_with_index_verify=()
# benchmarks kept out of the results because their command did not exit as expected
failed_benchmarks=()
# benchmarks quarantined in results/suspect_results.csv by suspect_reason or a --verify check
suspect_benchmarks=()

# We run hyperfine with -i, so a failing command does not abort the run - but hyperfine then
# records the failure as a timing, and an instant failure looks like a big speedup. That hid
# broken benchmarks for many releases (sqlp after a bad schema cache, count --width on quoted
# delimiters, pivotp_dates hitting --max-columns). So we check the exit codes of the timed
# runs in hyperfine's JSON export: exit_codes_ok returns 0 (true) only if every timed run
# exited with the expected code ($1). A missing export or a null code (killed by a signal)
# counts as a failure. jq is not needed - the codes are extracted with tr & grep.
# hf_exit_codes prints the timed runs' codes comma-separated, or nothing for a missing or
# non-v2 export. hyperfine 2.0's JSON (schema_version 2) has one "exit_code" per entry of
# measurements[], which holds the timed runs only (no warmups), like 1.x's "exit_codes" array.
function hf_exit_codes {
  [ -s results/hf_result.json ] || return 0
  local flat
  flat=$(tr -d ' \t\r\n' <results/hf_result.json)
  [[ "$flat" == *'"schema_version":2'* ]] || return 0
  grep -o '"exit_code":[^,}]*' <<<"$flat" | cut -d: -f2 | paste -sd, -
}

function exit_codes_ok {
  local expected="$1"
  local codes code_list code
  codes=$(hf_exit_codes)
  [ -n "$codes" ] || return 1
  IFS=',' read -r -a code_list <<<"$codes"
  for code in "${code_list[@]}"; do
    [ "$code" == "$expected" ] || return 1
  done
  return 0
}

function flag_failed_benchmark {
  local name="$1"
  local expected="$2"
  local codes
  codes=$(hf_exit_codes)
  echo "  !!! FAILED: $name did not exit with $expected on every timed run (exit codes: ${codes:-none - missing or unreadable hyperfine export})."
  echo "  !!! Its timing is NOT recorded. Run the command by hand to see the error."
  failed_benchmarks+=("$name")
}

# hf_result_row turns hyperfine 2.0's CSV export into the archive's columns:
# mean,stddev,median,user,system,min,max (hyperfine 1.x's names, so old and new rows line up;
# user/system were 1.x's MEAN user/system CPU times) plus peak_mem_mib, the LARGEST peak
# resident memory of the timed runs. Writes results/hf_row.csv (a header and one row).
function hf_result_row {
  "$qsv_bin" select 'time_wall_clock_mean,time_wall_clock_stddev,time_wall_clock_median,time_user_mean,time_system_mean,time_wall_clock_min,time_wall_clock_max,memory_peak_resident_max' \
    results/hf_result.csv |
    "$qsv_bin" rename 'mean,stddev,median,user,system,min,max,peak_mem_mib' -o results/hf_row.csv
}

# csv_value <file> <column>: the first data row's value in <column> (empty if absent).
# Only used on files this script writes, whose values never contain commas or quotes.
function csv_value {
  awk -F, -v col="$2" 'NR == 1 { for (i = 1; i <= NF; i++) if ($i == col) c = i; next }
    NR == 2 { if (c) print $c; exit }' "$1"
}

# suspect_reason <name> prints why the result in results/hf_row.csv looks like a failed or
# skipped run rather than real work, or nothing when it looks fine (see the thresholds at the
# top). The history checks compare against results/history_baseline.csv, the benchmark's most
# recent archived version; they are inert for a benchmark without history.
function suspect_reason {
  local name="$1"
  local mean min max mem base base_mean base_mem
  mean=$(csv_value results/hf_row.csv mean)
  min=$(csv_value results/hf_row.csv min)
  max=$(csv_value results/hf_row.csv max)
  mem=$(csv_value results/hf_row.csv peak_mem_mib)
  base=$(awk -F, -v n="$name" '$1 == n { print $2 "," $3; exit }' results/history_baseline.csv 2>/dev/null)
  base_mean=${base%%,*}
  base_mem=${base#*,}
  awk -v mean="$mean" -v min="$min" -v max="$max" -v mem="$mem" \
    -v base_mean="$base_mean" -v base_mem="$base_mem" \
    -v spread_max="$spread_max" -v speedup_pct="$speedup_pct" -v mem_drop_pct="$mem_drop_pct" '
    BEGIN {
      r = ""
      if (min > 0 && max / min > spread_max)
        r = r sprintf("timed runs spread %.1fx (slowest/fastest > %s); ", max / min, spread_max)
      if (base_mean != "" && base_mean > 0 && mean < base_mean * speedup_pct / 100)
        r = r sprintf("mean %.4fs is under %s%% of the last archived version (%.4fs); ", mean, speedup_pct, base_mean)
      if (mem != "" && base_mem != "" && base_mem > 0 && mem < base_mem * mem_drop_pct / 100)
        r = r sprintf("peak memory %.1f MiB is under %s%% of the last archived version (%.1f MiB); ", mem, mem_drop_pct, base_mem)
      printf "%s", r
    }'
}

# quarantine_benchmark <name> <reason> keeps a suspect result OUT of the results: its row goes to
# results/suspect_results.csv (with the reason, and the rowcount its recs_per_sec divides by)
# for review, and `./benchmarks.sh accept <name>` records it if it turns out to be real.
# Real big wins happen (writestat 10.7 s -> 0.2 s), so nothing is dropped automatically.
function quarantine_benchmark {
  local name="$1"
  local reason="${2//\"/\'}"
  rowcount_for_benchmark "$name"
  echo "  ??? SUSPECT: $name - ${reason%; }"
  echo "  ??? Its timing is NOT recorded. If it is real, run: ./benchmarks.sh accept $name"
  {
    echo "version,tstamp,logtime,name,rowcount,reason"
    printf '%s,%s,%s,%s,%s,"%s"\n' "$version" "$now" "$now_sec" "$name" "${subset_rowcount:-$rowcount}" "${reason%; }"
  } >results/suspect_work.csv
  "$qsv_bin" cat columns results/suspect_work.csv results/hf_row.csv -o results/suspect_entry.csv
  if [ -s results/suspect_results.csv ]; then
    "$qsv_bin" cat rowskey results/suspect_results.csv results/suspect_entry.csv \
      -o results/suspect_work.csv
    mv results/suspect_work.csv results/suspect_results.csv
  else
    mv results/suspect_entry.csv results/suspect_results.csv
  fi
  suspect_benchmarks+=("$name")
}

# record_benchmark <name> appends the result in results/hf_row.csv to latest_results.csv,
# after the version, tstamp & benchmark name
function record_benchmark {
  echo "version,tstamp,name" >results/results_work.csv
  echo "$version,$now,$1" >>results/results_work.csv
  "$qsv_bin" cat columns results/results_work.csv results/hf_row.csv -o results/entry.csv
  "$qsv_bin" cat rowskey results/latest_results.csv results/entry.csv \
    -o results/results_work.csv
  mv results/results_work.csv results/latest_results.csv
}

function add_command {
  local dest_array="$1"
  shift
  local cmd="$*"

  if [[ "$dest_array" == "without_index" ]]; then
    commands_without_index+=("$cmd")
  else
    commands_with_index+=("$cmd")
  fi
}

function run {
  local index=
  # --conclude CMD is handed to hyperfine's --conclude, which runs CMD after each timing
  # run WITHOUT timing it. Use it for benchmarks whose command refuses to re-run against
  # its own leftover output (e.g. `to sqlite` errors if the table already exists): without
  # per-run cleanup only the first run does real work and the rest measure an instant
  # failure. Doing the cleanup here instead of inside the timed command keeps it out of
  # the measurement and out of the command's exit status.
  # Since hyperfine 2.0, CMD always runs in a SHELL, even though the benchmarked command itself
  # runs without one (-N), so `;`, `&&`, globs and redirects work. (1.x ran it as a bare
  # program under -N.) ONE TRAP: if CMD exits non-zero, hyperfine ABORTS THE WHOLE BENCHMARK
  # RUN. -i does not cover it (that only ignores the benchmarked program's exit code).
  # `rm -f` is safe - it exits 0 on a missing file - but anything fancier needs its own guard.
  # --cleanup CMD is handed to hyperfine's --cleanup, which runs CMD ONCE, after ALL of this
  # benchmark's warmup and timing runs. Use it for benchmarks that leave behind a file that
  # changes how LATER benchmarks behave, but that must persist across this benchmark's own
  # runs (e.g. sqlp --cache-schema: the warmup writes the .pschema.json and the timed runs
  # measure loading it, but every later sqlp on $data would silently load it too).
  # The trap above applies to --cleanup as well.
  # --prepare CMD is handed to hyperfine's --prepare, which runs CMD before each warmup and
  # timing run WITHOUT timing it. Use it to start every run from the same state, e.g. `sync`
  # before the extsort benchmarks, so a run doesn't pay for flushing the previous run's
  # ~520 MB of output to disk. The same trap applies.
  # --expect-exit N is the exit code the command must return on every timed run (default 0).
  # A benchmark whose command exits with anything else is flagged and kept out of the results
  # (see exit_codes_ok). Only use it for commands that exit non-zero BY DESIGN while still
  # doing the work being measured, e.g. `sortcheck` on unsorted data, or `validate
  # --valid-output` when every row is valid - never to silence a real failure.
  # --verify CHECK checks that the command does the work being timed, which an exit code cannot
  # show (count printed 0 for two releases while exiting 0). Before the timed runs, the command
  # runs once more, untimed, with its stdout captured; then the shell line CHECK runs with
  # $QSVBM_STDOUT (that stdout's file), $QSVBM_QSV (the qsv under test) and $QSVBM_ROWCOUNT (the
  # benchmark data's rowcount) set. A non-zero CHECK quarantines the benchmark (see
  # quarantine_benchmark). It costs one extra run, so use it where a silent wrong result has
  # happened or is likely.
  local conclude=
  local prepare=
  local cleanup=
  local expect=0
  local verify=
  while true; do
    case "$1" in
    --index)
      index="yes"
      shift
      ;;
    --conclude)
      conclude="$2"
      shift 2
      ;;
    --prepare)
      prepare="$2"
      shift 2
      ;;
    --cleanup)
      cleanup="$2"
      shift 2
      ;;
    --expect-exit)
      expect="$2"
      shift 2
      ;;
    --verify)
      verify="$2"
      shift 2
      ;;
    *)
      break
      ;;
    esac
  done

  local name="$1"
  shift

  if [[ "$name" == *"$arg_pat"* ]]; then
    # record a per-benchmark rowcount override if this benchmark reads a subset of $data,
    # so its recs_per_sec is not computed against the full dataset (qsv issue #4598).
    # An empty subset_rowcount (not a subset benchmark, or a failed count) records nothing,
    # so the benchmark simply falls back to the full $rowcount as before.
    rowcount_for_benchmark "$name"
    if [ -n "$subset_rowcount" ]; then
      bench_rowcounts="${bench_rowcounts}${name}=${subset_rowcount};"
    fi
    # the _conclude, _prepare, _cleanup, _expect and _verify arrays are index-aligned with the
    # _name arrays, so every benchmark appends an entry - an empty one when it has no
    # --conclude/--prepare/--cleanup/--verify command, and 0 when it has no --expect-exit.
    if [ -z "$index" ]; then
      commands_without_index_name+=("$name")
      commands_without_index_conclude+=("$conclude")
      commands_without_index_prepare+=("$prepare")
      commands_without_index_cleanup+=("$cleanup")
      commands_without_index_expect+=("$expect")
      commands_without_index_verify+=("$verify")
      add_command "without_index" "$@"
    else
      commands_with_index_name+=("$name")
      commands_with_index_conclude+=("$conclude")
      commands_with_index_prepare+=("$prepare")
      commands_with_index_cleanup+=("$cleanup")
      commands_with_index_expect+=("$expect")
      commands_with_index_verify+=("$verify")
      add_command "with_index" "$@"
    fi
  fi
}

# ---------------------------------------
# Queue commands for benchmarking
# commands with an --index prefix will be benchmarked with an index and a stats cache
# template: run <benchmark name> <qsv command> <qsv command args>
# Note that several benchmarks assume the the benchmark_data is using the NYC 311 dataset, so
# the column names are hardcoded.
# If you're using a different dataset, you will need to modify the commands below to use the
# appropriate column names.

run apply_calcconv "$qsv_bin apply calcconv --formatstr \"{Unique_Key} meters in miles\" --new-column new_col $data"
run apply_dynfmt "$qsv_bin apply dynfmt --formatstr \"{Created_Date} {Complaint_Type} - {BBL} {City}\" --new-column new_col $data"
run apply_emptyreplace "$qsv_bin" apply emptyreplace \"Bridge Highway Name\" --replacement Unspecified "$data"
run apply_op_eudex "$qsv_bin apply operations lower,eudex Agency --comparand Queens --new-column Agency_queens_soundex $data"
run apply_op_string "$qsv_bin apply operations lower Agency $data"
run apply_op_sentiment "$qsv_bin apply operations sentiment \"Resolution Description\" --new-column Sentiment_Score $data"
run apply_op_similarity "$qsv_bin apply operations lower,simdln Agency --comparand brooklyn --new-column Agency_sim-brooklyn_score $data"
run apply_op_similarity_batchall "$qsv_bin apply operations lower,simdln Agency --comparand brooklyn --batch 0 --new-column Agency_sim-brooklyn_score $data"
run behead "$qsv_bin" behead "$data"
run behead_flexible "$qsv_bin" behead --flexible "$data"
run blake3 "$qsv_bin" blake3 "$data"
run blake3_j1 "$qsv_bin" blake3 -j 1 "$data"
run blake3_no_mmap "$qsv_bin" blake3 --no-mmap "$data"
run cat_columns "$qsv_bin" cat columns "$data" data_unsorted.csv
run cat_rows "$qsv_bin" cat rows "$data" data_unsorted.csv
run cat_rows_flexible "$qsv_bin" cat rows --flexible "$data" data_unsorted.csv
run cat_rowskey "$qsv_bin" cat rowskey "$data" data_unsorted.csv
# --verify: count must print the benchmark data's rowcount (--width appends ";<widths>").
# count printed 0 for 2.0.0-2.1.0 while exiting 0, and was timed as a fast run.
# shellcheck disable=SC2016 # expanded later, by the bash -c that runs the check
verify_count='test "$(cut -d";" -f1 "$QSVBM_STDOUT")" = "$QSVBM_ROWCOUNT"'
run --verify "$verify_count" count "$qsv_bin" count "$data"
run --verify "$verify_count" count_flexible "$qsv_bin" count --flexible "$data"
run --verify "$verify_count" count_no_polars "$qsv_bin" count --no-polars "$data"
run --index --verify "$verify_count" count_index "$qsv_bin" count "$data"
run --verify "$verify_count" count_width "$qsv_bin" count --width "$data"
run --index --verify "$verify_count" count_width_index "$qsv_bin" count --width "$data"
run datefmt "$qsv_bin datefmt \"Created Date\" $data"
run datefmt_multi "$qsv_bin datefmt \"Created Date,Closed Date,Due Date\" $data"
run datefmt_multi_batchall "$qsv_bin datefmt \"Created Date,Closed Date,Due Date\" --batch 0 $data"
run datefmt_multi_select "$qsv_bin datefmt '/(?i) date$/' $data"
run datefmt_formatstr_newcol "$qsv_bin datefmt --formatstr '%V' \"Created Date\" --new-column week_number $data"
run dedup "$qsv_bin" dedup "$data"
run dedup_sorted "$qsv_bin" dedup data_sorted.csv
run dedup_sorted_streaming "$qsv_bin" dedup --sorted data_sorted.csv
run denull "$qsv_bin" denull "$data"
run denull_apply "$qsv_bin" denull --apply "$data"
run diff "$qsv_bin" diff "$data" data_unsorted.csv
run enum "$qsv_bin" enum "$data"
run enum_hash "$qsv_bin" enum --hash 1- "$data"
run enum_uuid "$qsv_bin" enum --uuid4 "$data"
run enum_uuid7 "$qsv_bin" enum --uuid7 "$data"
run enum_constant "$qsv_bin" enum --constant "NYC" "$data"
run enum_copy "$qsv_bin" enum --copy Agency "$data"
run excel "$qsv_bin" excel benchmark_data.xlsx
run excel_j1 "$qsv_bin" excel -j 1 benchmark_data.xlsx
run excel_trim "$qsv_bin" excel --trim benchmark_data.xlsx
run excel_trim_j1 "$qsv_bin" excel --trim -j 1 benchmark_data.xlsx
run excel_error_format_formula "$qsv_bin" excel --error-format both benchmark_data.xlsx
run excel_metadata "$qsv_bin" excel --metadata c benchmark_data.xlsx
run excel_metadata_short "$qsv_bin" excel --metadata s benchmark_data.xlsx
run exclude "$qsv_bin" exclude \'Incident Zip\' "$data" \'Incident Zip\' data_to_exclude.csv
run --index exclude_index "$qsv_bin" exclude \'Incident Zip\' "$data" \'Incident Zip\' data_to_exclude.csv
run exclude_casei "$qsv_bin" exclude --ignore-case \'Incident Zip\' "$data" \'Incident Zip\' data_to_exclude.csv
run --index exclude_casei_index "$qsv_bin" exclude --ignore-case \'Incident Zip\' "$data" \'Incident Zip\' data_to_exclude.csv
run exclude_multi "$qsv_bin" exclude \'Incident Zip,Community Board,Agency\' "$data" \'Incident Zip,Community Board,Agency\' data_to_exclude.csv
run --index exclude_multi_index "$qsv_bin" exclude \'Incident Zip,Community Board,Agency\' "$data" \'Incident Zip,Community Board,Agency\' data_to_exclude.csv
run exclude_multi_casei "$qsv_bin" exclude --ignore-case \'Incident Zip,Community Board,Agency\' "$data" \'Incident Zip,Community Board,Agency\' data_to_exclude.csv
run --index exclude_multi_casei_index "$qsv_bin" exclude --ignore-case \'Incident Zip,Community Board,Agency\' "$data" \'Incident Zip,Community Board,Agency\' data_to_exclude.csv
run explode "$qsv_bin" explode City "-" "$data"
run extdedup "$qsv_bin" extdedup "$data"
run extdedup_csv "$qsv_bin" extdedup "$data" --select 1-5
# extsort writes the whole ~520 MB file each run (and spills temp files into ./), so its wall
# time swung 0.8-2.1 s between releases with unchanged code and CPU time, depending on how
# much of the previous run's output was still being written back. sync first.
run --prepare sync extsort "$qsv_bin" extsort data_unsorted.csv extsort_sorted.csv
run --prepare sync extsort_spill "$qsv_bin" extsort --memory-limit 100 data_unsorted.csv extsort_sorted.csv
run --prepare sync extsort_csv env QSV_AUTOINDEX_SIZE=1 bash -c \'"$qsv_bin" extsort data_unsorted.csv --select 1-5 extsort_sorted.csv\'
run fixedwidth_100k "$qsv_bin" fixedwidth fwf_100kdata.txt
run fill "$qsv_bin" fill -v Unspecified \'Address Type\' "$data"
run fixlengths "$qsv_bin" fixlengths "$data"
run flatten "$qsv_bin" flatten "$data"
run flatten_condensed "$qsv_bin" flatten "$data" --condense 50
run fmt "$qsv_bin" fmt --crlf "$data"
run fmt_no_crlf "$qsv_bin" fmt "$data"
run fmt_no_final_newline "$qsv_bin" fmt --no-final-newline "$data"
run foreach "$qsv_bin" foreach City \"echo {}\" "$data"
run frequency "$qsv_bin" frequency "$data"
run --index frequency_index "$qsv_bin" frequency "$data"
run --index frequency_index_stats_mode_auto env QSV_STATS_MODE=auto bash -c \'"$qsv_bin" frequency "$data"\'
run --index frequency_index_stats_mode_force env QSV_STATS_MODE=force bash -c \'"$qsv_bin" frequency "$data"\'
run --index frequency_index_stats_mode_none env QSV_STATS_MODE=none bash -c \'"$qsv_bin" frequency "$data"\'
run --index frequency_index_frequency_cache "$qsv_bin" frequency --frequency-jsonl "$data"
run frequency_no_limit "$qsv_bin" frequency --limit 0 "$data"
run --index frequency_no_limit_index "$qsv_bin" frequency --limit 0 "$data"
run frequency_other_sorted "$qsv_bin" frequency --other-sorted "$data"
run --index frequency_other_sorted_index "$qsv_bin" frequency --other-sorted "$data"
run frequency_selregex "$qsv_bin" frequency -s /^R/ "$data"
run frequency_j1 "$qsv_bin" frequency -j 1 "$data"
run frequency_ignorecase "$qsv_bin" frequency -i "$data"
run --index frequency_ignorecase_index "$qsv_bin" frequency -i "$data"
run frequency_selregex_ignorecase "$qsv_bin" frequency -s /^R/ -i "$data"
run frequency_j1_ignorecase "$qsv_bin" frequency -j 1 -i "$data"
run frequency_notrim "$qsv_bin" frequency --no-trim "$data"
run --index frequency_notrim_index "$qsv_bin" frequency --no-trim "$data"
run frequency_limit20 "$qsv_bin" frequency --limit 20 "$data"
run --index frequency_limit20_index "$qsv_bin" frequency --limit 20 "$data"
run frequency_sorted "$qsv_bin" frequency data_sorted.csv
run --index frequency_sorted_index "$qsv_bin" frequency data_sorted.csv
run frequency_sketch "$qsv_bin" frequency --sketch-method frequent_items "$data"
run --index frequency_sketch_index "$qsv_bin" frequency --sketch-method frequent_items "$data"
run frequency_weighted "$qsv_bin" frequency --weight "'X Coordinate (State Plane)'" "$data"
run geocode_suggest "$qsv_bin" geocode suggest City --new-column geocoded_city "$data"
run geocode_suggest_batchall "$qsv_bin" geocode suggest City --new-column geocoded_city --batch 0 "$data"
run geocode_reverse "$qsv_bin" geocode reverse Location --new-column geocoded_location "$data"
run geocode_reverse_batchall "$qsv_bin" geocode reverse Location --new-column geocoded_location --batch 0 "$data"
# Online geoclient lookups of real NYC addresses (see the geoclient inputs in the prep section).
# The two differ only in the cache: _nocache times live API calls on 10k rows - bounded by the
# API's rate limit as much as by qsv - and _cache times serving all 100k from the primed cache.
# _cache runs with an invalid key, so a row missing from the cache fails it (see the prep section).
# A bad key or an exhausted quota makes qsv exit non-zero, but a run whose lookups mostly fail
# for other reasons still exits 0, so --verify checks that at least 90% of the rows got NYC
# coordinates (AddressPoint addresses are Geosupport's own, so nearly all should match).
if [[ "$has_geoclient" -eq 1 ]]; then
  geoclient_geocoded='test "$("$QSVBM_QSV" search -s geo_latlong "^4[01][.][0-9]+, -7[34][.]" "$QSVBM_STDOUT" | "$QSVBM_QSV" count)" -ge '
  run --verify "${geoclient_geocoded}$((geoclient_10k_rowcount * 9 / 10))" geocode_geoclient_nocache \
    env "QSV_DOTENV_PATH='<NONE>'" "QSV_CACHE_DIR=$geoclient_cache_dir" "$qsv_bin" geocode geoclient \
    address --endpoint address --params "'$geoclient_params'" -c geo_latlong --no-cache geoclient_10k.csv
  if [[ "$has_geoclient_cache" -eq 1 ]]; then
    run --verify "${geoclient_geocoded}$((geoclient_100k_rowcount * 9 / 10))" geocode_geoclient_cache \
      env "QSV_DOTENV_PATH='<NONE>'" "QSV_CACHE_DIR=$geoclient_cache_dir" \
      "QSV_GEOCLIENT_API_KEY=$geoclient_offline_key" "$qsv_bin" geocode geoclient address \
      --endpoint address --params "'$geoclient_params'" -c geo_latlong --cache-ttl 0 \
      nyc_addresspoint_100k.csv
  fi
fi
run geoconvert_csv2geojsonl "$qsv_bin" geoconvert geo_data.csv csv geojsonl -y Latitude -x Longitude
run headers "$qsv_bin" headers "$data"
run headers_justnames "$qsv_bin" headers --just-names "$data"
run headers_justcount "$qsv_bin" headers --just-count "$data"
run implode "$qsv_bin" implode -k \'Community Board\' -v \'Complaint Type\' "-" "$data"
run index "$qsv_bin" index "$data"
run input "$qsv_bin" input "$data"
run json "$qsv_bin" json benchmark_data.json
run json_jaq "$qsv_bin" json "--jaq 'map(select(.Borough == \"BROOKLYN\"))'" benchmark_data.json
run join "$qsv_bin" join \'Community Board\' "$data" community_board communityboards.csv
run join_casei "$qsv_bin" join \'Community Board\' "$data" community_board --ignore-case communityboards.csv
# --verify: the join must produce rows. A schema/dtype problem can leave joinp with an empty
# result while still exiting 0, which times as a big speedup.
# shellcheck disable=SC2016 # expanded later, by the bash -c that runs the check
verify_has_rows='test "$("$QSVBM_QSV" count "$QSVBM_STDOUT")" -gt 0'
run --verify "$verify_has_rows" joinp "$qsv_bin" joinp \'Community Board\' "$data" community_board communityboards.csv
run --verify "$verify_has_rows" joinp_streaming "$qsv_bin" joinp \'Community Board\' "$data" community_board communityboards.csv --streaming
run --verify "$verify_has_rows" joinp_left "$qsv_bin" joinp \'Community Board\' "$data" community_board communityboards.csv --left
run joinp_left_anti "$qsv_bin" joinp \'Community Board\' "$data" community_board communityboards.csv --left-anti
run jsonl "$qsv_bin" jsonl benchmark_data.jsonl
run jsonl_batchall "$qsv_bin" jsonl benchmark_data.jsonl --batch 0
run jsonl_j1 "$qsv_bin" jsonl -j 1 benchmark_data.jsonl
run luau_filter "$qsv_bin" luau filter \"Location == \'\'\" "$data"
run luau_filter_colidx "$qsv_bin" luau filter --colindex \"Location == \'\'\" "$data"
run luau_filter_no_globals "$qsv_bin" luau filter --no-globals \"Location == \'\'\" "$data"
run luau_filter_no_globals_colidx "$qsv_bin" luau filter --no-globals --colindex \"Location == \'\'\" "$data"
run luau_multi "$qsv_bin" luau map dow,hourday,weekno "file:dt_format.luau" "$data"
run luau_multi_colidx "$qsv_bin" luau map dow,hourday,weekno "file:dt_format.luau" --colindex "$data"
run luau_multi_no_globals "$qsv_bin" luau map dow,hourday,weekno --no-globals "file:dt_format.luau" "$data"
run luau_multi_no_globals_colidx "$qsv_bin" luau map dow,hourday,weekno --no-globals --colindex "file:dt_format.luau" "$data"
run luau_script "$qsv_bin" luau map turnaround_time "file:turnaround_time.luau" "$data"
run luau_script_debug env QSV_LOG_LEVEL=debug bash -c \'"$qsv_bin" luau map turnaround_time "file:turnaround_time.luau" "$data"\'
run luau_script_colidx "$qsv_bin" luau map turnaround_time --colindex "file:turnaround_time.luau" "$data"
run luau_script_no_globals "$qsv_bin" luau map turnaround_time --no-globals "file:turnaround_time.luau" "$data"
run luau_script_no_globals_colidx "$qsv_bin" luau map turnaround_time --no-globals --colindex "file:turnaround_time.luau" "$data"
# py benchmarks mirror the luau ones above so the Luau-vs-Python interpreter
# performance gap is readily apparent (qsv issue #1351). They run with the python-enabled
# $qsv_py_bin (e.g. the qsvpyNNN flavors) instead of the PGO-optimized $qsv_bin, and only
# when that binary has the python feature; see has_python above.
if [[ "$has_python" -eq 1 ]]; then
  run py_filter "$qsv_py_bin py filter \"col['Location'] == ''\" $data"
  run py_dateformat "$qsv_py_bin py map dow --helper dt_format_py.py \"qsv_uh.dtformat(col['Created Date'])\" $data"
  run py_script "$qsv_py_bin py map turnaround_time --helper turnaround_time_py.py \"qsv_uh.turnaround(col['Created Date'], col['Closed Date'])\" $data"
  run py_script_batchall "$qsv_py_bin py map turnaround_time --batch 0 --helper turnaround_time_py.py \"qsv_uh.turnaround(col['Created Date'], col['Closed Date'])\" $data"
fi
run --index moarstats_index "$qsv_bin" moarstats "$data"
run --index moarstats_advanced_index "$qsv_bin" moarstats --advanced "$data"
run --index moarstats_bivariate_index "$qsv_bin" moarstats --bivariate "$data"
run --index moarstats_bivariate_all_index "$qsv_bin" moarstats --bivariate --bivariate-stats all "$data"
run --index moarstats_advanced_bivariate_index "$qsv_bin" moarstats --advanced --bivariate "$data"
run --index moarstats_advanced_bivariate_all_index "$qsv_bin" moarstats --advanced --bivariate --bivariate-stats all "$data"
run partition "$qsv_bin" partition \'Community Board\' partitioned "$data"
run pivotp_basic "$qsv_bin" pivotp "Agency" --index "Borough" --values \"Complaint Type\" "$data"
run pivotp_smart "$qsv_bin" pivotp "Agency" --index "Borough" --values \"Complaint Type\" --agg smart "$data"
# pivotp_dates (pivoting ON "Created Date") was retired: its ~841k distinct values exceed
# pivotp's default --max-columns of 100,000, which since 23.0.1 (#4590) refuses the pivot
# instantly, and with --max-columns 0 the 841k-column frame runs a 32 GB machine out of memory.
# Every date column in the dataset has over 100k distinct values, so instead the dates go on
# the index side: one row per parsed date, one column per borough. A new series - the old
# pivotp_dates rows are not comparable.
run pivotp_dates_rows "$qsv_bin" pivotp Borough --index \"Created Date\" --values \"Complaint Type\" --try-parsedates "$data"
# pragmastat appends ps_* columns to the stats cache and refuses to recompute them on
# subsequent runs unless --force is given. Without --force, only the first hyperfine run
# does real work and the rest error out on the existing cache, so we always pass --force
# to measure the actual computation on every run.
# These run on a 50k-row subset - the full 1M-row dataset takes ~2 hours per run. The
# benchmark names carry the _50k marker so they form their own series in the historical
# archive: the delta/rank columns compare a benchmark only against earlier runs of the
# same name, so the subset runs are never compared against the old full-dataset rows.
# recs_per_sec for these divides by pragmastats_50kdata.csv's own rowcount, not the full
# dataset's - see rowcount_for_benchmark() above and benchmark_aggregations.luau.
if [[ "$has_profile" -eq 1 ]]; then
  run profile "$qsv_bin" profile --no-dcat-discovery "$data" -o benchmark_work.metadata.json
fi
run pragmastat_50k "$qsv_bin" pragmastat --force pragmastats_50kdata.csv
run pragmastat_50k_twosample "$qsv_bin" pragmastat --twosample --force -s \'Latitude,Longitude\' pragmastats_50kdata.csv
run --index pragmastat_50k_index "$qsv_bin" pragmastat --force pragmastats_50kdata.csv
run pseudo "$qsv_bin" pseudo \'Unique Key\' "$data"
run pseudo_formatstr "$qsv_bin" pseudo \'Unique Key\' --formatstr 'ID-{}' --increment 5 "$data"
if [[ "$has_readstat" -eq 1 ]]; then
  run readstat_sav "$qsv_bin" readstat stat_100kdata.sav
  run readstat_sav_compressed "$qsv_bin" readstat stat_100kdata_compressed.sav
  run readstat_dta "$qsv_bin" readstat stat_100kdata.dta
  run readstat_dta_select "$qsv_bin" readstat --select unique_key,agency,borough stat_100kdata.dta
  run readstat_xpt "$qsv_bin" readstat stat_100kdata.xpt
fi
run rename "$qsv_bin" rename \'unique_key,created_date,closed_date,agency,agency_name,complaint_type,descriptor,loctype,zip,addr1,street,xstreet1,xstreet2,inter1,inter2,addrtype,city,landmark,facility_type,status,due_date,res_desc,res_act_date,comm_board,bbl,boro,xcoord,ycoord,opendata_type,parkname,parkboro,vehtype,taxi_boro,taxi_loc,bridge_hwy_name,bridge_hwy_dir,ramp,bridge_hwy_seg,lat,long,loc\' "$data"
run replace "$qsv_bin" replace \'zip\' \'postal\' "$data"
run --index replace_indexed "$qsv_bin" replace \'zip\' \'postal\' "$data"
run reverse "$qsv_bin" reverse "$data"
run --index reverse_index "$qsv_bin" reverse "$data"
run safenames "$qsv_bin" safenames "$data"
run sample_10 "$qsv_bin" sample 10 "$data"
run --index sample_10_index "$qsv_bin" sample 10 "$data"
run sample_1000 "$qsv_bin" sample 1000 "$data"
run --index sample_1000_index "$qsv_bin" sample 1000 "$data"
run sample_100000 "$qsv_bin" sample 100000 "$data"
run --index sample_100000_index "$qsv_bin" sample 100000 "$data"
run sample_100000_seeded "$qsv_bin" sample 100000 --seed 42 "$data"
run sample_100000_seeded_faster "$qsv_bin" sample 100000 --rng faster --seed 42 "$data"
run sample_100000_seeded_secure "$qsv_bin" sample 100000 --rng cryptosecure --seed 42 "$data"
run sample_bernoulli "$qsv_bin" sample --bernoulli 0.25 --seed 42 "$data"
run sample_1000_systematic "$qsv_bin" sample 1000 --systematic random --seed 42 "$data"
run sample_1000_stratified "$qsv_bin" sample 1000 --stratified "Agency" --seed 42 "$data"
run sample_1000_weighted "$qsv_bin" sample 1000 --weighted \"Incident Zip\" --seed 42 "$data"
run sample_500_cluster "$qsv_bin" sample 500 --cluster \"Incident Zip\" --seed 42 "$data"
run --index sample_100000_seeded_index "$qsv_bin" sample --seed 42 100000 "$data"
run --index sample_100000_seeded_index_faster "$qsv_bin" sample --rng faster --seed 42 100000 "$data"
run --index sample_100000_seeded_index_secure "$qsv_bin" sample --rng cryptosecure --seed 42 100000 "$data"
run --index sample_25pct_index "$qsv_bin" sample 0.25 "$data"
run --index sample_25pct_seeded_index "$qsv_bin" sample 0.25 --seed 42 "$data"
run schema "$qsv_bin" schema --force "$data"
run --index schema_index "$qsv_bin" schema "$data"
run scoresql "$qsv_bin" scoresql "$data" '"select Borough, count(*) from _t_1 group by Borough"'
run search "$qsv_bin" search -s \'Agency Name\' "'(?i)us'" "$data"
run search_unicode "$qsv_bin" search --unicode -s \'Agency Name\' "'(?i)us'" "$data"
run search_file "$qsv_bin" search "'(?i)us'" "$data"
run search_file_literal "$qsv_bin" search --literal "'$'" "$data"
run search_file_unicode "$qsv_bin" search --unicode "'(?i)us'" "$data"
run search_file_case_sensitive "$qsv_bin" search "'us'" "$data"
run search_file_case_sensitive_unicode "$qsv_bin" search --unicode "'us'" "$data"
run search_file_flag "$qsv_bin" search "'(?i)us'" "$data" --flag flagged
run search_file_flag_matchonly "$qsv_bin" search "'(?i)us'" "$data" --flag M
run --index search_index "$qsv_bin" search -s \'Agency Name\' "'(?i)us'" "$data"
run searchset "$qsv_bin" searchset searchset_patterns.txt "$data"
run searchset_ignorecase "$qsv_bin" searchset --ignore-case searchset_patterns.txt "$data"
run searchset_unicode "$qsv_bin" searchset searchset_patterns_unicode.txt --unicode "$data"
run --index searchset_index "$qsv_bin" searchset searchset_patterns.txt "$data"
run select "$qsv_bin" select \'Agency,Community Board\' "$data"
run select_regex "$qsv_bin" select /^L/ "$data"
run slice_one_middle "$qsv_bin" slice -i 500000 "$data"
run slice_last_1k "$qsv_bin" slice -s 1000 -l 1000 "$data"
run slice_last_1k_json "$qsv_bin" slice -s 1000 -l 1000 --json "$data"
run --index slice_one_middle_index "$qsv_bin" slice -i 500000 "$data"
run --index slice_last_1k_index "$qsv_bin" slice -s 1000 -l 1000 "$data"
run --index slice_last_1k_json_index "$qsv_bin" slice -s 1000 -l 1000 --json "$data"
run snappy_compress "$qsv_bin" snappy compress "$data" --output benchmark_data.snappy
run snappy_decompress "$qsv_bin" snappy decompress benchmark_data.snappy
run snappy_validate "$qsv_bin" snappy validate benchmark_data.snappy
run snappy_check "$qsv_bin" snappy check benchmark_data.snappy
run sniff "$qsv_bin" sniff "$data"
run sort "$qsv_bin" sort -s \'Incident Zip\' "$data"
run sort_random_seeded "$qsv_bin" sort --random --seed 42 "$data"
run sort_random_seeded_faster "$qsv_bin" sort --random --rng faster --seed 42 "$data"
run sort_random_seeded_secure "$qsv_bin" sort --random --rng cryptosecure --seed 42 "$data"
run sortcheck_sorted "$qsv_bin" sortcheck data_sorted.csv
# sortcheck exits 1 when the data is not sorted - which is what these two measure
run --expect-exit 1 sortcheck_unsorted "$qsv_bin" sortcheck data_unsorted.csv
run --expect-exit 1 sortcheck_unsorted_all "$qsv_bin" sortcheck --all data_unsorted.csv
run split "$qsv_bin" split --size 50000 split_tempdir "$data"
run split_chunks "$qsv_bin" split --chunks 20 split_tempdir_chunks "$data"
run split_kbsize "$qsv_bin" split --kb-size 10000 split_tempdir_kbs "$data"
run --index split_index "$qsv_bin" split --size 50000 split_tempdir_idx "$data"
run --index split_index_j1 "$qsv_bin" split --size 50000 -j 1 split_tempdir_idx_j1 "$data"
run --index split_chunks_index "$qsv_bin" split --chunks 20 split_tempdir_chunks_idx "$data"
run --index split_chunks_index_j1 "$qsv_bin" split --chunks 20 -j 1 split_tempdir_chunks_idx_j1 "$data"
run sqlp "$qsv_bin" sqlp "$data" -q --infer-len 100000 '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_aggregations "$qsv_bin" sqlp "$data" -q --infer-len 100000 '"select Borough, count(*) from _t_1 where \"Complaint Type\"='\''Noise'\'' group by Borough"'
# sqlp loads "$data".pschema.json automatically whenever it is newer than "$data", so the
# schema this benchmark caches must be removed when it is done (--cleanup, i.e. after its own
# timed runs have reused it). Otherwise every later sqlp benchmark on "$data" silently loads
# it too: they measure a cached-schema read instead of what their names say, and from 16.0.0
# to 24.0.0 - when --cache-schema wrote date columns Polars couldn't parse (#4746) - they all
# failed in ~70ms, which hyperfine's -i recorded as timings.
run --cleanup "rm -f $data.pschema.json" sqlp_aggregations_use_schema_cache "$qsv_bin" sqlp "$data" -q --infer-len 100000 --cache-schema '"select Borough, count(*) from _t_1 where \"Complaint Type\"='\''Noise'\'' group by Borough"'
run sqlp_aggregations_vs_duckdb duckdb :memory: '"select Borough, count(*) from read_csv_auto('\'''$data''\'') where \"Complaint Type\"='\''Noise'\'' group by Borough"'
run sqlp_parquet_input "$qsv_bin" sqlp SKIP_INPUT -q '"select Borough, count(*) from read_parquet('\''benchmark_data.parquet'\'') where \"Complaint Type\"='\''Noise'\'' group by Borough"'
run sqlp_aggregations_expensive "$qsv_bin" sqlp SKIP_INPUT -q --infer-len 100000 expensive.sql
run sqlp_aggregations_expensive_streaming "$qsv_bin" sqlp SKIP_INPUT -q --infer-len 100000 --streaming expensive.sql
run sqlp_aggregations_expensive_vs_duckdb duckdb :memory: -c \".read expensiveduckdb.sql\"
run sqlp_format_arrow "$qsv_bin" sqlp --format arrow "$data" -q --infer-len 100000 '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_format_avro "$qsv_bin" sqlp --format avro "$data" -q --infer-len 100000 '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_format_json "$qsv_bin" sqlp --format json "$data" -q --infer-len 100000 '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_format_jsonl "$qsv_bin" sqlp --format jsonl "$data" -q --infer-len 100000 '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_format_parquet "$qsv_bin" sqlp --format parquet "$data" -q --infer-len 100000 '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_format_parquet_statistics "$qsv_bin" sqlp --format parquet --statistics "$data" -q --infer-len 100000 '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_nooptimizations "$qsv_bin" sqlp "$data" -q --no-optimizations --infer-len 100000 '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_tryparsedates "$qsv_bin" sqlp "$data" -q --try-parsedates --infer-len 100000 '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_tryparsedates_inferlen "$qsv_bin" sqlp "$data" -q --infer-len 10000 --try-parsedates '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run stats "$qsv_bin" stats --force "$data"
# exact vs HyperLogLog cardinality on their own: under --everything the exact mode tracker
# runs anyway, so the stats_everything_approx pair cannot show what HLL alone costs
run stats_cardinality "$qsv_bin" stats "$data" --force --cardinality --cache-threshold 0
run stats_cardinality_approx "$qsv_bin" stats "$data" --force --cardinality --cardinality-method approx --cache-threshold 0
run stats_create_cache "$qsv_bin" stats --force "$data" --cache-threshold 1
run --index stats_index "$qsv_bin" stats --force "$data"
run --index stats_index_with_cache "$qsv_bin" stats "$data" --cache-threshold 1
run --index stats_index_j1 "$qsv_bin" stats -j 1 --force "$data"
run --index stats_index_j1_with_cache "$qsv_bin" stats -j 1 "$data" --cache-threshold 1
run stats_everything "$qsv_bin" stats "$data" --force --everything
run stats_everything_create_cache "$qsv_bin" stats "$data" --force --everything --cache-threshold 1
run stats_everything_infer_dates "$qsv_bin" stats "$data" --force --everything --infer-dates
run stats_everything_approx "$qsv_bin" stats "$data" --force --everything --quantile-method approx --cardinality-method approx --cache-threshold 0
run stats_everything_mode_cap "$qsv_bin" stats "$data" --force --everything --mode-cardinality-cap 100000 --cache-threshold 0
run stats_everything_j1 "$qsv_bin" stats "$data" --force --everything -j 1
run stats_everything_sorted "$qsv_bin" stats data_sorted.csv --force --everything
run --index stats_everything_index "$qsv_bin" stats "$data" --force --everything
run --index stats_everything_index_with_cache "$qsv_bin" stats "$data" --everything --cache-threshold 1
run --index stats_everything_infer_dates_index "$qsv_bin" stats "$data" --force --everything --infer-dates
run --index stats_everything_infer_dates_index_with_cache "$qsv_bin" stats "$data" --everything --infer-dates --cache-threshold 1
run --index stats_everything_approx_index "$qsv_bin" stats "$data" --force --everything --quantile-method approx --cardinality-method approx --cache-threshold 0
run --index stats_everything_index_j1 "$qsv_bin" stats "$data" --force --everything -j 1
run --index stats_everything_index_j1_with_cache "$qsv_bin" stats "$data" --everything -j 1 --cache-threshold 1
run --index stats_everything_sorted_index "$qsv_bin" stats data_sorted.csv --force --everything
run --index stats_everything_sorted_index_with_cache "$qsv_bin" stats data_sorted.csv --everything --cache-threshold 1
if [[ "$has_synthesize" -eq 1 ]]; then
  run synthesize "$qsv_bin" synthesize --seed 42 -n 1000 "$data"
fi
run table "$qsv_bin" table "$data"
run template "$qsv_bin" template --template-file template.tpl "$data" >/dev/null
run template_lookup_outdir "$qsv_bin" template --template-file template-with-cb-lookup.tpl "$data" >/dev/null
run to_xlsx "$qsv_bin" to xlsx benchmark_work.xlsx "$data"
# `to sqlite` ERRORS OUT if the target db already has the table, so the db must be
# removed between hyperfine runs — otherwise only the first run does real work and the
# rest measure an instant failure (masked by hyperfine's -i). --conclude runs the rm
# after each timing run but OUTSIDE the measurement, so unlike the old `bash -c` wrapper
# the cleanup costs nothing and cannot mask the conversion's exit status. (A bare `;rm`
# here would run once at queue time, not between runs — that was the original bug.)
run --conclude 'rm -f benchmark_work.db' to_sqlite "$qsv_bin" to sqlite benchmark_work.db "$data"
run to_datapackage "$qsv_bin" to datapackage benchmark_work.json "$data"
# to_ods uses a 500k-row subset (ods_data.csv); the full dataset overflows the ODS 4GB
# ZIP limit. See the ods_data.csv prep step above. Unlike `to sqlite`, `to ods`
# overwrites an existing output file cleanly, so no per-run cleanup is needed
# (cleanup_files removes benchmark_work.* at the end).
run to_ods "$qsv_bin" to ods benchmark_work.ods ods_data.csv
# --infer-len 127000 like the benchmark_data.json prep: polars' default inference window
# guesses Incident Zip as numeric and the conversion fails on "48090-2123".
# `to parquet` refuses to overwrite an existing output file, so - like to_sqlite - every run
# needs per-run cleanup; it creates the missing output directory itself.
run --conclude 'rm -r -f benchmark_work_parquet' to_parquet "$qsv_bin" to parquet --infer-len 127000 benchmark_work_parquet "$data"
run tojsonl "$qsv_bin" tojsonl "$data"
run tojsonl_batchall "$qsv_bin" tojsonl --batch 0 "$data"
run tojsonl_j1 "$qsv_bin" tojsonl -j 1 "$data"
run tojsonl_trim "$qsv_bin" tojsonl --trim "$data"
run tojsonl_trim_j1 "$qsv_bin" tojsonl --trim -j 1 "$data"
run tojsonl_no_boolean "$qsv_bin" tojsonl --no-boolean "$data"
run --index tojsonl_index "$qsv_bin" tojsonl "$data"
run --index tojsonl_index_j1 "$qsv_bin" tojsonl --jobs 1 "$data"
run transpose "$qsv_bin" transpose "$data"
run transpose_multipass "$qsv_bin" transpose --multipass "$data"
run validate "$qsv_bin" validate "$data" "$schema"
run validate_batchall "$qsv_bin" validate --batch 0 "$data" "$schema"
run validate_no_schema "$qsv_bin" validate "$data"
# with --valid-output, validate exits 1 when ALL rows are valid (as documented), after writing them all
run --expect-exit 1 validate_valid_output "$qsv_bin" validate "$data" "$schema" --valid-output -
run validate_dynenum "$qsv_bin" validate "$data" "$dynenum_schema"
run validate_dynenum_batchall "$qsv_bin" validate --batch 0 "$data" "$dynenum_schema"
run --expect-exit 1 validate_dynenum_valid_output "$qsv_bin" validate "$data" "$dynenum_schema" --valid-output -
if [[ "$has_readstat" -eq 1 ]]; then
  run writestat_sav "$qsv_bin" writestat stat_100kdata.csv -o benchmark_work.sav
  run writestat_sav_compress "$qsv_bin" writestat --compress stat_100kdata.csv -o benchmark_work.sav
  run writestat_dta "$qsv_bin" writestat stat_100kdata.csv -o benchmark_work.dta
  run writestat_xpt "$qsv_bin" writestat stat_100kdata.csv -o benchmark_work.xpt
fi
if [[ "$has_viz" -eq 1 ]]; then
  run viz_smart "$qsv_bin" viz smart "$data" -o benchmark_work.html
fi
run --index validate_index "$qsv_bin" validate "$data" "$schema"
run --index validate_batchall_index "$qsv_bin" validate --batch 0 "$data" "$schema"
run --index validate_no_schema_index "$qsv_bin" validate "$data"
run --index --expect-exit 1 validate_valid_output_index "$qsv_bin" validate "$data" "$schema" --valid-output -
run --index validate_dynenum_index "$qsv_bin" validate "$data" "$dynenum_schema"
run --index validate_dynenum_batchall_index "$qsv_bin" validate --batch 0 "$data" "$dynenum_schema"
run --index --expect-exit 1 validate_dynenum_valid_output_index "$qsv_bin" validate "$data" "$dynenum_schema" --valid-output -

# show count of commands to be benchmarked
with_index_count=${#commands_with_index[@]}
wo_index_count=${#commands_without_index[@]}
total_count=$((with_index_count + wo_index_count))
printf "> Commands to benchmark: %s, w/o index: %s, with index: %s\n\n" "$total_count" "$wo_index_count" "$with_index_count"
# ---------------------------------------
# Prepare benchmark results directory

# Check if a results directory exists, if it doesn't create it
if [ ! -d "results" ]; then
  mkdir results
fi

# Init latest_results.csv. It stores the benchmark results for this run
rm -f results/latest_results.csv
echo "version,tstamp,name,mean,stddev,median,user,system,min,max,peak_mem_mib" >results/latest_results.csv

# check if the file benchmark_results.csv exists, if it doesn't create it
# by copying the empty latest_results.csv
if [ ! -f "results/benchmark_results.csv" ]; then
  cp results/latest_results.csv results/benchmark_results.csv
fi

# get current time to the nearest hour
now=$(date +"%Y-%m-%d-%H")

# get current time to the nearest second
now_sec=$(date +"%Y-%m-%d-%H-%M-%S")

# ---------------------------------------
# Run hyperfine to compile benchmark results. Append each individual result to the latest_results.csv
# by dogfooding qsv's cat, luau, select & sort commands.

# first, run benchmarking without an index
# each command is run five times. Two warm-up runs & three benchmarked runs.
# this run will become the latest run, after which the previous run's quarantined results can no
# longer be accepted (see accept) - so say so if it still has some
if [ -s results/suspect_results.csv ] && [ -s results/latest_run_info.tsv ]; then
  prev_logtime=$(awk -F'\t' 'NR == 2 { print $3 }' results/latest_run_info.tsv)
  prev_suspects=$(awk -F, -v lt="$prev_logtime" 'NR > 1 && $3 == lt { printf "%s ", $4 }' results/suspect_results.csv)
  if [ -n "$prev_suspects" ]; then
    echo "> WARNING: the previous run ($prev_logtime) has quarantined results that were not accepted:"
    echo "    ${prev_suspects% }"
    echo "  Once this run finishes they can no longer be accepted. To keep any of them, stop this run"
    echo "  (Ctrl-C, any time before it finishes) and run ./benchmarks.sh accept <name>... first."
    echo ""
  fi
fi

# the rowcount --verify checks compare against (see run)
export QSVBM_ROWCOUNT=$rowcount

# history baseline for suspect_reason: per benchmark, the mean of its most recent archived version
# and the peak memory of its most recent version that has one (each version's latest run, its
# fastest row on an hour-precision tstamp tie, as in the delta computation). The LATEST version,
# not a median of several: every archived row passed these checks or was accepted by hand, so it
# is trusted, and a median would keep flagging a real, accepted speedup until it outnumbered the
# older, slower versions. Archives from before hyperfine 2.0 have no peak_mem_mib.
peak_mem_expr="null"
if head -n 1 results/benchmark_results.csv | tr ',' '\n' | grep -qx 'peak_mem_mib'; then
  peak_mem_expr="b.peak_mem_mib"
fi
"$qsv_benchmarker_bin" sqlp results/benchmark_results.csv -q "
with latest as (
  select name, version, max(tstamp) as tstamp
  from _t_1
  group by name, version
),
per_version as (
  select b.name, b.tstamp, min(b.mean) as mean, max($peak_mem_expr) as peak_mem_mib
  from _t_1 b
  join latest l on b.name = l.name and b.version = l.version and b.tstamp = l.tstamp
  group by b.name, b.version, b.tstamp
),
recent as (
  select name, mean,
         row_number() over (partition by name order by tstamp desc) as rn
  from per_version
),
recent_mem as (
  select name, peak_mem_mib,
         row_number() over (partition by name order by tstamp desc) as rn
  from per_version
  where peak_mem_mib is not null
)
select r.name, r.mean as base_mean, m.peak_mem_mib as base_mem
from recent r
left join recent_mem m on r.name = m.name and m.rn = 1
where r.rn = 1
" -o results/history_baseline.csv

# run_benchmark <name> <command> <expect> <verify> <prepare> <conclude> <cleanup> benchmarks one
# queued command and records its result - unless it failed (flag_failed_benchmark) or looks
# like it did not do the work (quarantine_benchmark).
function run_benchmark {
  local name="$1" command="$2" expect="$3" verify="$4"
  local hook_args=()
  [ -n "$5" ] && hook_args+=(--prepare "$5")
  [ -n "$6" ] && hook_args+=(--conclude "$6")
  [ -n "$7" ] && hook_args+=(--cleanup "$7")

  # the untimed --verify run. hyperfine runs it, so the queued command string is parsed exactly
  # as in the timed runs. (--output needs a path with a "/", or it is read as a policy name.)
  local verify_reason=
  if [ -n "$verify" ]; then
    rm -f results/verify_stdout.csv
    hyperfine -N --runs 1 -i --style none --output ./results/verify_stdout.csv \
      "${hook_args[@]}" "$command" >/dev/null
    if ! QSVBM_STDOUT=results/verify_stdout.csv QSVBM_QSV="$qsv_bin" bash -c "$verify"; then
      verify_reason="output check failed: $verify; "
    fi
  fi

  rm -f results/hf_result.json results/hf_result.csv
  hyperfine -N --warmup "$warmup_runs" -i --runs "$benchmark_runs" --metrics "$hf_metrics" \
    --export-csv results/hf_result.csv --export-json results/hf_result.json \
    "${hook_args[@]}" "$command"

  # a command that did not exit as expected is flagged, and its "timing" is not recorded
  if ! exit_codes_ok "$expect"; then
    flag_failed_benchmark "$name" "$expect"
    return
  fi
  hf_result_row
  local reason
  reason="${verify_reason}$(suspect_reason "$name")"
  if [ -n "$reason" ]; then
    quarantine_benchmark "$name" "$reason"
    return
  fi
  record_benchmark "$name"
}

echo "> Benchmarking WITHOUT INDEX..."
idx=0
name_idx=1
for command_no_index in "${commands_without_index[@]}"; do

  # remove the index files and the stats cache files
  rm -f "$data".idx
  rm -f pragmastats_50kdata.csv.idx
  rm -f "$filestem".stats.*

  pct_complete=$(((name_idx - 1) * 100 / total_count))
  echo "$name_idx. ${commands_without_index_name[$idx]} ($pct_complete%)"
  run_benchmark "${commands_without_index_name[$idx]}" "$command_no_index" \
    "${commands_without_index_expect[$idx]}" "${commands_without_index_verify[$idx]}" \
    "${commands_without_index_prepare[$idx]}" "${commands_without_index_conclude[$idx]}" \
    "${commands_without_index_cleanup[$idx]}"
  ((idx++))
  ((name_idx++))
done

# ---------------------------------------
# then, run benchmarks with an index and stats cache
# an index enables random access and unlocks multi-threading in several commands
# the stats cache enables faster stats computation as it will use the cached stats
# when its valid and available, instead of computing the stats from scratch
if [ "$with_index_count" -gt 0 ]; then
  echo "> Benchmarking WITH INDEX and STATS CACHE..."
  echo "  Preparing index and stats cache..."
  rm -f "$data".idx
  "$qsv_bin" index "$data"
  # the pragmastat_50k_index benchmark reads the 50k subset, not "$data", so it needs its
  # own index - without it, it silently measures the unindexed path and duplicates
  # pragmastat_50k instead of exercising pragmastat's indexed parallel-chunk reader.
  if [ -r pragmastats_50kdata.csv ]; then
    rm -f pragmastats_50kdata.csv.idx
    "$qsv_bin" index pragmastats_50kdata.csv
  fi
  "$qsv_bin" stats "$data" --everything --infer-dates --stats-jsonl --force \
    --output benchmark_work.stats.csv
fi

idx=0
for command_with_index in "${commands_with_index[@]}"; do
  pct_complete=$(((name_idx - 1) * 100 / total_count))
  echo "$name_idx. ${commands_with_index_name[$idx]} ($pct_complete%)"
  run_benchmark "${commands_with_index_name[$idx]}" "$command_with_index" \
    "${commands_with_index_expect[$idx]}" "${commands_with_index_verify[$idx]}" \
    "${commands_with_index_prepare[$idx]}" "${commands_with_index_conclude[$idx]}" \
    "${commands_with_index_cleanup[$idx]}"
  ((idx++))
  ((name_idx++))
done

finalize_results

# Clean up temporary files
cleanup_files

# ---------------------------------------
# Finalize benchmark run info. Append the run info to results/run_info_history.csv
# we use the TSV format as some of the data has commas/quotes/whitespace/semicolon, etc.

# get the environment variables used by qsv.
# NOTE: `--envlist` prints VALUES too (e.g. `QSV_CENSUS_API_KEY: "..."`), and
# run_info_history.tsv is committed & published (qsv.dathere.com/benchmarks and the
# GitHub wiki), so we record only the variable NAMES. We also collapse the
# potentially multi-line output to a single `;`-separated field so it cannot break
# the one-row-per-run TSV format. (The "No qsv-relevant environment variables set."
# message has no colon, so it passes through `cut` unchanged.)
qsv_envvars=$("$qsv_bin" --envlist | cut -d: -f1 | paste -sd ';' -)

elapsed=$SECONDS

# Init latest_run_info.csv. It stores the benchmark run info for this run
rm -f results/latest_run_info.tsv
echo -e "version\ttstamp\tlogtime\tbm_version\tplatform\tcores\tmem\tbinary\tkind\targument\ttotal_count\two_index_count\twith_index_count\twarmup_runs\tbenchmark_runs\telapsed_secs\ttotal_mean\tqsv_env\tversion_info\thyperfine_version" >results/latest_run_info.tsv

# check if the file run_info_history.csv exists, if it doesn't create it
# by copying the empty latest_run_info.csv
if [ ! -f "results/run_info_history.tsv" ]; then
  cp results/latest_run_info.tsv results/run_info_history.tsv
fi

# append the run info to latest_run_info.csv
echo -e "$version\t$now\t$now_sec\t$bm_version\t$platform\t$num_cores\t$mem_size\t$qsv_bin\t$kind\t$arg_pat\t$total_count\t$wo_index_count\t$with_index_count\t$warmup_runs\t$benchmark_runs\t$elapsed\t$total_mean\t$qsv_envvars\t$raw_version\t$hyperfine_version" >>results/latest_run_info.tsv

# now update the run_info_history.tsv
"$qsv_bin" cat rowskey results/latest_run_info.tsv results/run_info_history.tsv \
  -o results/run_info_work.tsv
mv results/run_info_work.tsv results/run_info_history.tsv

echo "> 100% DONE! $total_count benchmarks executed. Elapsed time: $elapsed seconds. Total mean: $total_mean"

if [ "${#failed_benchmarks[@]}" -gt 0 ]; then
  echo ""
  echo "> WARNING: ${#failed_benchmarks[@]} benchmark(s) FAILED and were NOT recorded in the results:"
  printf "    %s\n" "${failed_benchmarks[@]}"
  echo "  Run each command by hand to see the error. If a command exits non-zero BY DESIGN"
  echo "  while still doing the measured work, queue it with run --expect-exit N."
fi

if [ "${#suspect_benchmarks[@]}" -gt 0 ]; then
  echo ""
  echo "> WARNING: ${#suspect_benchmarks[@]} benchmark(s) look like they did not do the work, and were"
  echo "  QUARANTINED in results/suspect_results.csv instead of being recorded (its reason column says why):"
  printf "    %s\n" "${suspect_benchmarks[@]}"
  echo "  Check each one by running its qsv command DIRECTLY, not through benchmarks.sh: only the"
  echo "  latest run's quarantined results can be accepted, and any benchmarks.sh run (even a"
  echo "  one-benchmark pattern run) becomes the latest run. If a result is real (e.g. a genuine big"
  echo "  speedup), record it BEFORE the next benchmarks.sh run with:"
  echo "    ./benchmarks.sh accept ${suspect_benchmarks[*]}"
fi
