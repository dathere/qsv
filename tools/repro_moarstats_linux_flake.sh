#!/usr/bin/env bash
#
# Reproduce harness for the Linux-only flake on
#   test_moarstats::moarstats_join_type_full_runs_and_writes_bivariate
# (qsv PR #3834 CI run 25545197594).
#
# Build a qsv binary on the branch fix/moarstats-linux-stats-validation
# (or master + the fix cherry-picked) with all_features, then run this
# script on the same Ubuntu VM as CI:
#
#   QSV_BIN=/path/to/qsv ./tools/repro_moarstats_linux_flake.sh
#
# What it does:
#   - Replicates the failing test's exact qsv invocations as a shell loop:
#       qsv stats --everything primary.csv
#       qsv moarstats --bivariate --join-inputs secondary.csv \
#           --join-keys id,id --join-type full primary.csv
#     then inspects primary.stats.bivariate.joined.csv for value2a/value2b.
#   - Runs N parallel workers, each looping ITERS times, to mimic CI's
#     concurrent test load (which is what surfaced the macOS flake).
#   - Optionally runs background I/O churn (DD_LOAD=1) to push page-cache
#     pressure, which is the most plausible Linux-specific trigger.
#   - Classifies each failure into one of three buckets:
#       (a) NEW_DIAGNOSTIC — the fix's strict validation fired
#         ("Joined-stats subprocess output is missing stats records…").
#         This *confirms* the root cause is the qsv-stats-on-joined-CSV
#         subprocess handoff, not the join itself.
#       (b) OLD_BIVARIATE_MISSING — the bivariate output is missing
#         value2a/value2b columns and the strict validation did NOT fire.
#         This means the joined CSV itself was corrupted (or the parent's
#         second header re-read also saw the truncation), pointing at
#         join_datasets_internal rather than the subprocess handoff.
#       (c) OTHER — qsv exited non-zero for some other reason, or output
#         was not produced. Dump and stop.
#
# Output: per-failure log under $OUT_DIR; final summary line.
#
# Don't expect this to reproduce on macOS. The flake is Linux-specific.

set -u

QSV_BIN="${QSV_BIN:-qsv}"
WORKERS="${WORKERS:-8}"
ITERS="${ITERS:-200}"        # iterations per worker -> WORKERS*ITERS total
DD_LOAD="${DD_LOAD:-0}"      # 1 = run background I/O churn
OUT_DIR="${OUT_DIR:-$PWD/repro_logs_$(date +%Y%m%d_%H%M%S)}"
KEEP_PASSING_LOGS="${KEEP_PASSING_LOGS:-0}"

mkdir -p "$OUT_DIR"

if ! command -v "$QSV_BIN" >/dev/null 2>&1 && [ ! -x "$QSV_BIN" ]; then
    echo "ERROR: qsv binary not found at: $QSV_BIN" >&2
    echo "Set QSV_BIN to the absolute path of your built qsv binary." >&2
    exit 2
fi

QSV_BIN_ABS="$(command -v "$QSV_BIN" || readlink -f "$QSV_BIN")"
echo "qsv binary:      $QSV_BIN_ABS"
echo "qsv version:     $("$QSV_BIN_ABS" --version 2>&1 | head -1)"
echo "workers:         $WORKERS"
echo "iters/worker:    $ITERS"
echo "total runs:      $((WORKERS * ITERS))"
echo "dd background:   $DD_LOAD"
echo "log dir:         $OUT_DIR"
echo

# Background I/O churn: writes/reads/syncs random files in a scratch dir to
# pressure the page cache. Mimics the heavy parallel I/O load CI sees when
# many tests run together. Disabled by default; enable with DD_LOAD=1.
DD_PIDS=()
cleanup() {
    if [ "${#DD_PIDS[@]}" -gt 0 ]; then
        for pid in "${DD_PIDS[@]}"; do
            kill "$pid" 2>/dev/null || true
        done
    fi
    wait 2>/dev/null || true
}
trap cleanup EXIT INT TERM

if [ "$DD_LOAD" = "1" ]; then
    LOAD_DIR="$(mktemp -d -t qsv_repro_load_XXXXXX)"
    echo "Starting background I/O load in $LOAD_DIR ..."
    for i in 1 2 3 4; do
        (
            while true; do
                f="$LOAD_DIR/churn_${i}_$RANDOM"
                dd if=/dev/urandom of="$f" bs=64K count=64 status=none 2>/dev/null
                sync "$f" 2>/dev/null || sync
                cat "$f" >/dev/null 2>&1
                rm -f "$f"
            done
        ) &
        DD_PIDS+=("$!")
    done
fi

# One worker run: replicates the failing test for $ITERS iterations.
# Emits "PASS"/"FAIL_NEW_DIAGNOSTIC"/"FAIL_OLD_BIVARIATE"/"FAIL_OTHER" lines
# to stdout (one per iteration), and dumps full context to $OUT_DIR on
# failure.
run_worker() {
    local worker_id="$1"
    local iter
    local wd
    local rc
    local biv
    local moarstats_log

    for iter in $(seq 1 "$ITERS"); do
        wd="$(mktemp -d -t qsv_repro_w${worker_id}_i${iter}_XXXXXX)"

        # Mirror tests/test_moarstats.rs:4589+ exactly.
        cat >"$wd/primary.csv" <<'EOF'
id,value1,value1b
1,10,100
2,20,200
4,40,400
1,10,100
EOF
        cat >"$wd/secondary.csv" <<'EOF'
id,value2a,value2b
1,1000,10000
2,2000,20000
3,3000,30000
EOF

        # Step 1: baseline stats (test runs this first; matches wrk.command).
        # Run from $wd so qsv-emitted side files (primary.stats.csv,
        # primary.stats.csv.jsonl) land in $wd, mirroring the integration
        # test's Workdir behavior.
        ( cd "$wd" && "$QSV_BIN_ABS" stats --everything primary.csv ) \
                >"$wd/stats.stdout" 2>"$wd/stats.stderr"
        rc=$?
        if [ "$rc" -ne 0 ]; then
            echo "FAIL_OTHER worker=$worker_id iter=$iter step=stats rc=$rc wd=$wd"
            cp -r "$wd" "$OUT_DIR/fail_other_w${worker_id}_i${iter}_$(date +%s)"
            rm -rf "$wd"
            continue
        fi

        # Step 2: moarstats with full join + bivariate. CWD is $wd so the
        # relative --join-inputs secondary.csv resolves correctly (the
        # spawned qsv join subprocess inherits CWD from the parent).
        moarstats_log="$wd/moarstats.stderr"
        ( cd "$wd" && "$QSV_BIN_ABS" moarstats \
                --bivariate \
                --join-inputs secondary.csv \
                --join-keys id,id \
                --join-type full \
                primary.csv ) \
                >"$wd/moarstats.stdout" 2>"$moarstats_log"
        rc=$?
        if [ "$rc" -ne 0 ]; then
            # Did the new strict-coverage validation fire? Root-cause-confirming.
            if grep -q "Joined-stats subprocess output is missing stats records" \
                    "$moarstats_log" 2>/dev/null; then
                echo "FAIL_NEW_DIAGNOSTIC worker=$worker_id iter=$iter rc=$rc wd=$wd"
                cp -r "$wd" "$OUT_DIR/fail_new_w${worker_id}_i${iter}_$(date +%s)"
            else
                echo "FAIL_OTHER worker=$worker_id iter=$iter rc=$rc wd=$wd"
                cp -r "$wd" "$OUT_DIR/fail_other_w${worker_id}_i${iter}_$(date +%s)"
            fi
            rm -rf "$wd"
            continue
        fi

        # Step 3: inspect bivariate output. Matches assert_bivariate_columns_present.
        biv="$wd/primary.stats.bivariate.joined.csv"
        if [ ! -f "$biv" ]; then
            echo "FAIL_OTHER worker=$worker_id iter=$iter step=missing_biv wd=$wd"
            cp -r "$wd" "$OUT_DIR/fail_other_w${worker_id}_i${iter}_$(date +%s)"
            rm -rf "$wd"
            continue
        fi

        # Concatenate the field1 and field2 columns from the bivariate
        # output and check that value2a + value2b appear at least once.
        # awk is used to avoid pulling in csvlens/qsv for the check.
        if ! awk -F, '
                NR == 1 { for (i=1; i<=NF; i++) if ($i=="field1") f1=i; else if ($i=="field2") f2=i; next }
                { seen[$f1]=1; seen[$f2]=1 }
                END {
                    if (!("value2a" in seen) || !("value2b" in seen)) {
                        for (k in seen) printf "%s,", k
                        print ""
                        exit 1
                    }
                }
            ' "$biv" >/dev/null 2>"$wd/biv_check.stderr"; then
            echo "FAIL_OLD_BIVARIATE worker=$worker_id iter=$iter wd=$wd"
            cp -r "$wd" "$OUT_DIR/fail_old_biv_w${worker_id}_i${iter}_$(date +%s)"
            rm -rf "$wd"
            continue
        fi

        echo "PASS worker=$worker_id iter=$iter"
        if [ "$KEEP_PASSING_LOGS" != "1" ]; then
            rm -rf "$wd"
        fi
    done
}

# Spawn workers in parallel and aggregate results.
RESULTS_FIFO="$(mktemp -u -t qsv_repro_fifo_XXXXXX)"
mkfifo "$RESULTS_FIFO"

(
    for w in $(seq 1 "$WORKERS"); do
        run_worker "$w" &
    done
    wait
) >"$RESULTS_FIFO" &

PASS=0
FAIL_NEW=0
FAIL_OLD=0
FAIL_OTHER=0

while IFS= read -r line; do
    case "$line" in
        PASS\ *) PASS=$((PASS+1)) ;;
        FAIL_NEW_DIAGNOSTIC\ *)
            FAIL_NEW=$((FAIL_NEW+1))
            echo "*** $line" ;;
        FAIL_OLD_BIVARIATE\ *)
            FAIL_OLD=$((FAIL_OLD+1))
            echo "*** $line" ;;
        FAIL_OTHER\ *)
            FAIL_OTHER=$((FAIL_OTHER+1))
            echo "*** $line" ;;
    esac
    # Live progress every 100 results.
    total=$((PASS + FAIL_NEW + FAIL_OLD + FAIL_OTHER))
    if [ $((total % 100)) -eq 0 ] && [ "$total" -gt 0 ]; then
        echo "  progress: $total runs (pass=$PASS new_diag=$FAIL_NEW old_biv=$FAIL_OLD other=$FAIL_OTHER)"
    fi
done <"$RESULTS_FIFO"

rm -f "$RESULTS_FIFO"

echo
echo "=== Summary ==="
echo "  total runs           : $((PASS + FAIL_NEW + FAIL_OLD + FAIL_OTHER))"
echo "  pass                 : $PASS"
echo "  FAIL_NEW_DIAGNOSTIC  : $FAIL_NEW   (root cause confirmed: subprocess CSV->stats handoff)"
echo "  FAIL_OLD_BIVARIATE   : $FAIL_OLD   (bivariate dropped columns despite strict check passing)"
echo "  FAIL_OTHER           : $FAIL_OTHER  (qsv error / missing output / etc.)"
echo "  log dir              : $OUT_DIR"
echo

if [ "$FAIL_NEW" -gt 0 ]; then
    echo "ROOT CAUSE CONFIRMED: the qsv stats subprocess (run on the joined CSV"
    echo "by moarstats) is producing stats for fewer columns than the joined CSV"
    echo "actually contains. The fix should target the subprocess handoff:"
    echo "  - re-fsync the joined CSV right before spawning qsv stats, or"
    echo "  - posix_fadvise(POSIX_FADV_DONTNEED) the joined CSV before spawn,"
    echo "  - or retry qsv stats on coverage-mismatch."
    echo
    echo "Inspect a sample failure under: $OUT_DIR/fail_new_*"
    exit 1
fi

if [ "$FAIL_OLD" -gt 0 ]; then
    echo "DIFFERENT ROOT CAUSE: bivariate output dropped value2a/value2b but the"
    echo "strict coverage check did NOT fire. The joined CSV likely re-truncated"
    echo "between the parent's coverage re-read and the bivariate computation,"
    echo "OR the bivariate writer itself is the culprit. Investigate"
    echo "join_datasets_internal and the bivariate writer in src/cmd/moarstats.rs."
    echo
    echo "Inspect a sample failure under: $OUT_DIR/fail_old_biv_*"
    exit 1
fi

if [ "$FAIL_OTHER" -gt 0 ]; then
    echo "Unexpected failures (qsv error / missing output). See $OUT_DIR/fail_other_*"
    exit 1
fi

echo "All runs passed. Increase WORKERS, ITERS, or set DD_LOAD=1 and rerun."
exit 0
