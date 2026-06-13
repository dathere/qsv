#!/usr/bin/env bash
#
# qsv-tune.sh - profile this machine and recommend optimal qsv performance settings.
#
# Implements the idea in qsv issue #2829: inspect the current machine's capabilities
# (RAM, CPU cores, disk type) and the qsv binary's compiled-in memory allocator, then
# emit a tuned set of QSV_* environment variables in `.env` format.
#
# DRY-RUN BY DEFAULT: with no arguments it prints an annotated, ready-to-use `.env`
# block to stdout and writes NOTHING. Redirect it to capture the suggestion:
#
#     scripts/qsv-tune.sh > my-qsv.env
#
# Use --write to merge the tuned QSV_* settings into an actual `.env` file. The
# settings are wrapped in sentinel markers so re-running is idempotent (it replaces
# the previous qsv-tune block instead of duplicating it) and a `.bak` backup is made:
#
#     scripts/qsv-tune.sh --write           # merge into ./.env
#     scripts/qsv-tune.sh --write path/.env # merge into a specific file
#
# Allocator tuning (jemalloc/mimalloc) is reported SEPARATELY as `export` lines, NOT
# written into `.env`: allocators read their configuration at process start, before
# qsv loads `.env`, so those variables must live in the real shell environment.
#
# Supported on Linux and macOS. See scripts/qsv-tune.ps1 for a Windows (best-effort)
# equivalent. The recommended values are heuristic starting points - always benchmark
# against your own data shapes (see docs/PERFORMANCE.md).
#
# Usage:
#   scripts/qsv-tune.sh [--write [PATH]] [--force] [-h|--help]
#
# Options:
#   --write [PATH]   Merge the QSV_* block into PATH (default: ./.env) instead of
#                    printing to stdout. Creates a <PATH>.bak backup first.
#   --force          With --write, overwrite the whole file instead of merging just
#                    the managed qsv-tune block.
#   -h, --help       Show this help.
#
# Hidden testing overrides (not for normal use):
#   --debug-total-mem-bytes N   Pretend total RAM is N bytes.
#   --debug-allocator KIND      Pretend the allocator is jemalloc|mimalloc|standard.

set -euo pipefail

readonly MARK_START="# >>> qsv-tune (generated) >>>"
readonly MARK_END="# <<< qsv-tune <<<"

# ---------------------------------------------------------------------------
# argument parsing
# ---------------------------------------------------------------------------
WRITE=0
FORCE=0
TARGET_ENV=".env"
DEBUG_TOTAL_MEM=""
DEBUG_ALLOCATOR=""

usage() { sed -n '2,46p' "$0" | sed 's/^# \{0,1\}//'; }

while [ $# -gt 0 ]; do
    case "$1" in
        --write)
            WRITE=1
            if [ $# -gt 1 ] && [ "${2#-}" = "$2" ]; then
                TARGET_ENV="$2"
                shift
            fi
            ;;
        --force) FORCE=1 ;;
        --debug-total-mem-bytes)
            DEBUG_TOTAL_MEM="${2:-}"
            shift
            ;;
        --debug-allocator)
            DEBUG_ALLOCATOR="${2:-}"
            shift
            ;;
        -h | --help)
            usage
            exit 0
            ;;
        *)
            printf 'qsv-tune: unknown argument: %s\n\n' "$1" >&2
            usage >&2
            exit 2
            ;;
    esac
    shift
done

# ---------------------------------------------------------------------------
# helpers
# ---------------------------------------------------------------------------
log() { printf '%s\n' "$*" >&2; }

human() {
    # bytes -> human readable (GiB/MiB), one decimal
    awk -v b="$1" 'BEGIN {
        if (b >= 1073741824) printf "%.1f GiB", b / 1073741824;
        else if (b >= 1048576) printf "%.1f MiB", b / 1048576;
        else printf "%d B", b;
    }'
}

OS="$(uname -s)"

detect_total_mem() {
    if [ -n "$DEBUG_TOTAL_MEM" ]; then
        printf '%s' "$DEBUG_TOTAL_MEM"
        return
    fi
    case "$OS" in
        Darwin) sysctl -n hw.memsize ;;
        Linux) awk '/^MemTotal:/ {print $2 * 1024; exit}' /proc/meminfo ;;
        *) printf '0' ;;
    esac
}

detect_avail_mem() {
    case "$OS" in
        Darwin)
            # available ~= (free + inactive) pages * page size
            vm_stat | awk '
                /page size of/ { for (i=1;i<=NF;i++) if ($i ~ /^[0-9]+$/) ps=$i }
                /Pages free:/ { gsub(/[^0-9]/,"",$3); free=$3 }
                /Pages inactive:/ { gsub(/[^0-9]/,"",$3); inact=$3 }
                END { print (free + inact) * ps }'
            ;;
        Linux) awk '/^MemAvailable:/ {print $2 * 1024; exit}' /proc/meminfo ;;
        *) printf '0' ;;
    esac
}

detect_cpus() {
    case "$OS" in
        Darwin) sysctl -n hw.logicalcpu ;;
        Linux) nproc ;;
        *) getconf _NPROCESSORS_ONLN 2> /dev/null || printf '1' ;;
    esac
}

# Echoes one of: SSD HDD unknown
detect_disk_type() {
    local dir="${PWD}"
    case "$OS" in
        Darwin)
            local dev
            dev="$(df "$dir" 2> /dev/null | awk 'NR==2 {print $1}')"
            if [ -n "$dev" ] && command -v diskutil > /dev/null 2>&1; then
                local ss
                ss="$(diskutil info "$dev" 2> /dev/null | awk -F: '/Solid State/ {gsub(/^[ \t]+/,"",$2); print $2; exit}')"
                case "$ss" in
                    Yes*) printf 'SSD'; return ;;
                    No*) printf 'HDD'; return ;;
                esac
            fi
            # NVMe internal storage is always SSD
            if system_profiler SPNVMeDataType 2> /dev/null | grep -q 'Model'; then
                printf 'SSD'
                return
            fi
            printf 'unknown'
            ;;
        Linux)
            local src base
            src="$(df --output=source "$dir" 2> /dev/null | tail -1)"
            base="$(basename "$src")"
            # nvme devices are always SSD
            case "$base" in
                nvme*)
                    printf 'SSD'
                    return
                    ;;
            esac
            # strip trailing partition digits (sda1 -> sda)
            base="$(printf '%s' "$base" | sed -E 's/[0-9]+$//')"
            local rot="/sys/block/${base}/queue/rotational"
            if [ -r "$rot" ]; then
                case "$(cat "$rot")" in
                    0) printf 'SSD' ;;
                    1) printf 'HDD' ;;
                    *) printf 'unknown' ;;
                esac
            else
                printf 'unknown'
            fi
            ;;
        *) printf 'unknown' ;;
    esac
}

# Echoes one of: jemalloc mimalloc standard unknown
detect_allocator() {
    if [ -n "$DEBUG_ALLOCATOR" ]; then
        printf '%s' "$DEBUG_ALLOCATOR"
        return
    fi
    if ! command -v qsv > /dev/null 2>&1; then
        printf 'unknown'
        return
    fi
    local ver
    ver="$(qsv --version 2> /dev/null || true)"
    case "$ver" in
        *jemalloc*) printf 'jemalloc' ;;
        *mimalloc*) printf 'mimalloc' ;;
        *standard*) printf 'standard' ;;
        *) printf 'unknown' ;;
    esac
}

# ---------------------------------------------------------------------------
# profile the machine
# ---------------------------------------------------------------------------
TOTAL_MEM="$(detect_total_mem)"
AVAIL_MEM="$(detect_avail_mem)"
CPUS="$(detect_cpus)"
DISK="$(detect_disk_type)"
ALLOC="$(detect_allocator)"

[ -z "$TOTAL_MEM" ] && TOTAL_MEM=0
[ -z "$AVAIL_MEM" ] && AVAIL_MEM=0
[ -z "$CPUS" ] && CPUS=1

GIB=$((1024 * 1024 * 1024))

# ---------------------------------------------------------------------------
# heuristics -> recommended QSV_* values
# ---------------------------------------------------------------------------
MAX_JOBS="$CPUS"

# free-memory headroom: tighter on big-RAM boxes, generous on small ones
if [ "$TOTAL_MEM" -lt $((8 * GIB)) ]; then
    HEADROOM_PCT=30
    MEMORY_CHECK=true
elif [ "$TOTAL_MEM" -gt $((32 * GIB)) ]; then
    HEADROOM_PCT=10
    MEMORY_CHECK=false
else
    HEADROOM_PCT=20
    MEMORY_CHECK=false
fi

# auto-index threshold: cheap random access on SSD, costlier on HDD
case "$DISK" in
    HDD) AUTOINDEX_SIZE=104857600 ;; # 100 MiB
    *) AUTOINDEX_SIZE=10485760 ;;    # 10 MiB (SSD / unknown - assume modern SSD)
esac

# I/O buffers: defaults are fine on SSD; bump on HDD or very large RAM
if [ "$DISK" = "HDD" ] || [ "$TOTAL_MEM" -ge $((32 * GIB)) ]; then
    RDR_BUF=1048576 # 1 MiB
    WTR_BUF=2097152 # 2 MiB
    BUF_TUNED=1
else
    RDR_BUF=131072 # 128 KiB (default)
    WTR_BUF=524288 # 512 KiB (default)
    BUF_TUNED=0
fi

# guidance figure for chunk-memory env vars (kept dynamic by default)
CHUNK_GUIDE_MB=$(((AVAIL_MEM / 1048576) / (CPUS > 0 ? CPUS : 1)))

# ---------------------------------------------------------------------------
# build the .env block (QSV_* settings only)
# ---------------------------------------------------------------------------
build_env_block() {
    cat << EOF
${MARK_START}
# Generated by scripts/qsv-tune.sh - heuristic starting points, benchmark before relying on them.
# Machine profile: ${OS} | RAM total $(human "$TOTAL_MEM") / avail $(human "$AVAIL_MEM") | ${CPUS} logical CPUs | disk ${DISK} | allocator ${ALLOC}

# Use one job per logical CPU for multithreaded commands.
QSV_MAX_JOBS = ${MAX_JOBS}

# Reserve ${HEADROOM_PCT}% of memory before loading a whole file (non-streaming mode).
QSV_FREEMEMORY_HEADROOM_PCT = ${HEADROOM_PCT}

# CONSERVATIVE memory check (avail + swap, platform factor) - on when RAM is tight.
QSV_MEMORY_CHECK = ${MEMORY_CHECK}

# Auto-create an index for files >= this size (bytes). $( [ "$DISK" = HDD ] && echo "100 MiB - indexing reads cost more on HDD." || echo "10 MiB - random-access indexing is cheap on SSD." )
QSV_AUTOINDEX_SIZE = ${AUTOINDEX_SIZE}

EOF

    if [ "$BUF_TUNED" -eq 1 ]; then
        cat << EOF
# Larger I/O buffers $( [ "$DISK" = HDD ] && echo "help spinning disks." || echo "help with plenty of RAM." )
QSV_RDR_BUFFER_CAPACITY = ${RDR_BUF}
QSV_WTR_BUFFER_CAPACITY = ${WTR_BUF}

EOF
    else
        cat << EOF
# I/O buffers left at defaults (good for SSD). Uncomment to enlarge.
# QSV_RDR_BUFFER_CAPACITY = 1048576
# QSV_WTR_BUFFER_CAPACITY = 2097152

EOF
    fi

    cat << EOF
# Chunk sizing for parallel stats/frequency. Dynamic (0) is usually best; the
# computed avail_mem/jobs figure (~${CHUNK_GUIDE_MB} MB) is only a hint if you set a fixed cap.
# QSV_STATS_CHUNK_MEMORY_MB = 0
# QSV_FREQ_CHUNK_MEMORY_MB = 0

# Keep qsv's built-in allocator tuning on. Set true only in RSS-constrained
# (<4 GiB) environments or for diagnostics.
QSV_NO_ALLOC_TUNING = false

# Skipping the mime-type/format check is faster but removes a safety net. Opt in only
# if you trust your inputs (or hit false positives).
# QSV_SKIP_FORMAT_CHECK = false
${MARK_END}
EOF
}

# ---------------------------------------------------------------------------
# build the allocator advice (export lines - NOT for .env)
# ---------------------------------------------------------------------------
build_alloc_advice() {
    cat << 'EOF'
# ===========================================================================
# Allocator tuning (advanced, optional)
# ===========================================================================
# IMPORTANT: allocators read their config at PROCESS START, before qsv loads
# `.env`. Put these in your real shell environment (export / setx), NOT in `.env`.
# They are workload- and platform-sensitive - measure before adopting.
EOF
    case "$ALLOC" in
        jemalloc)
            cat << 'EOF'
#
# qsv (jemalloc) already auto-tunes the two levers that reliably help its batch
# workloads (background_thread purging + dirty/muzzy page retention) unless
# QSV_NO_ALLOC_TUNING=true - so do NOT re-set those here.
#
# These two extra levers are deliberately left OFF by qsv (mixed results - see
# docs/ENVIRONMENT_VARIABLES.md). Try them only if your own benchmarks show a win:
#   # Linux + large-RAM only: put jemalloc metadata on transparent huge pages
#   # export _RJEM_MALLOC_CONF=metadata_thp:auto
#   # per-CPU arenas (rarely helps - rayon workers migrate across CPUs)
#   # export _RJEM_MALLOC_CONF=percpu_arena:percpu
# (On binaries without the vendored-jemalloc prefix, use MALLOC_CONF instead of _RJEM_MALLOC_CONF.)
EOF
            ;;
        mimalloc)
            cat << 'EOF'
#
# qsv is using mimalloc. Batch-friendly options to try (commented out - measure first):
#   # Hold freed OS pages longer to cut purge overhead on hashmap-heavy commands:
#   # export MIMALLOC_PURGE_DELAY=10000
#   # Allow 2-4 MiB large OS pages where permitted (needs privileges/THP):
#   # export MIMALLOC_ALLOW_LARGE_OS_PAGES=1
#   # Reserve N x 1 GiB huge pages at startup (Linux, requires huge pages configured):
#   # export MIMALLOC_RESERVE_HUGE_OS_PAGES=1
# See the mimalloc docs (linked in docs/ENVIRONMENT_VARIABLES.md) for the full list.
EOF
            ;;
        standard)
            cat << 'EOF'
#
# qsv is using the standard system allocator - no allocator env tuning available.
# For batch/large-file workloads, consider a qsv build with jemalloc or mimalloc.
EOF
            ;;
        *)
            cat << 'EOF'
#
# Could not detect the allocator (is `qsv` on PATH?). Run `qsv --version` and look
# for `jemalloc`, `mimalloc`, or `standard` to know which tuning applies.
EOF
            ;;
    esac
}

# ---------------------------------------------------------------------------
# emit
# ---------------------------------------------------------------------------
ENV_BLOCK="$(build_env_block)"
ALLOC_ADVICE="$(build_alloc_advice)"

if [ "$WRITE" -eq 0 ]; then
    # dry-run: everything to stdout
    printf '%s\n\n%s\n' "$ENV_BLOCK" "$ALLOC_ADVICE"
    log "qsv-tune: dry-run only (nothing written). Use --write to merge into ${TARGET_ENV}."
    exit 0
fi

# --write path
if [ "$FORCE" -eq 1 ]; then
    [ -f "$TARGET_ENV" ] && cp "$TARGET_ENV" "${TARGET_ENV}.bak"
    printf '%s\n' "$ENV_BLOCK" > "$TARGET_ENV"
    log "qsv-tune: wrote ${TARGET_ENV} (overwrote; backup at ${TARGET_ENV}.bak)."
elif [ -f "$TARGET_ENV" ]; then
    cp "$TARGET_ENV" "${TARGET_ENV}.bak"
    if grep -qF "$MARK_START" "$TARGET_ENV"; then
        # replace existing managed block in place
        tmp="$(mktemp)"
        awk -v s="$MARK_START" -v e="$MARK_END" '
            $0 == s {skip=1}
            skip && $0 == e {skip=0; next}
            !skip {print}
        ' "$TARGET_ENV" > "$tmp"
        printf '%s\n' "$ENV_BLOCK" >> "$tmp"
        mv "$tmp" "$TARGET_ENV"
        log "qsv-tune: updated qsv-tune block in ${TARGET_ENV} (backup at ${TARGET_ENV}.bak)."
    else
        printf '\n%s\n' "$ENV_BLOCK" >> "$TARGET_ENV"
        log "qsv-tune: appended qsv-tune block to ${TARGET_ENV} (backup at ${TARGET_ENV}.bak)."
    fi
else
    printf '%s\n' "$ENV_BLOCK" > "$TARGET_ENV"
    log "qsv-tune: created ${TARGET_ENV}."
fi

log ""
log "Allocator advice (export in your shell, NOT in .env):"
printf '%s\n' "$ALLOC_ADVICE" >&2
