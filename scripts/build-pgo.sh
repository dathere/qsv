#!/usr/bin/env bash
#
# build-pgo.sh - parameterized Profile-Guided Optimization (PGO) build orchestrator
#                for qsv, used by both CI (publish workflows) and local users.
#
# It wraps cargo-pgo to run the three-step PGO cycle for ONE binary/target:
#   1. instrument build  (thin-LTO - fat-LTO + instrumentation is slow/fragile)
#   2. train             (scripts/pgo-train.sh exercises representative code paths)
#   3. optimize build    (keeps the fat-LTO from [profile.release], using the
#                         collected profiles via -Cprofile-use)
#
# The optimized binary is left in target/<target>/<profile>/<bin>, exactly where the
# publish workflows already look, so no packaging changes are needed.
#
# Configuration is via environment variables (CI sets these from the matrix; locally
# you can export them or rely on the defaults):
#
#   TARGET            REQUIRED. Rust target triple, e.g. x86_64-unknown-linux-gnu
#   BIN               Binary to build (default: qsv)
#   QSV_FEATURES      Comma-separated feature list WITHOUT the --features= prefix,
#                     e.g. "apply,luau,fetch,self_update,geocode,polars,to,lens".
#                     For the qsv binary, ",feature_capable" is appended automatically.
#   DEFAULT_FEATURES  Passthrough, e.g. "--no-default-features" (Windows) or "" (default)
#   PROFILE           Cargo profile (default: auto - "release-luau" if QSV_FEATURES
#                     contains "luau" else "release"). The main qsv binary needs
#                     release-luau (panic=unwind) when built with luau (qsv #3937).
#   PGO_LTO           OPTIONAL optimize-build LTO override (e.g. "thin" or "off").
#                     Leave unset to keep the fat-LTO from Cargo.toml. This is the
#                     escape hatch for memory-constrained runners (aarch64 OOM).
#   RUSTFLAGS         OPTIONAL passthrough (e.g. "-C target-cpu=native"). cargo-pgo
#                     appends its own -Cprofile-generate/-use flags to this.
#   TRAIN_FLAGS       OPTIONAL flags for pgo-train.sh (e.g. "--minimal" on Windows)
#
# Example (local, host target):
#   TARGET=$(rustc -vV | sed -n 's/host: //p') \
#   QSV_FEATURES=apply,luau,fetch,self_update,geocode,polars,to,lens,prompt,magika,color \
#   ./scripts/build-pgo.sh

set -euo pipefail

# locate repo root (this script lives in <root>/scripts/)
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$script_dir/.." && pwd)"
cd "$repo_root"

# ---- configuration ----------------------------------------------------------
: "${TARGET:?ERROR: TARGET (rust target triple) must be set}"
BIN="${BIN:-qsv}"
QSV_FEATURES="${QSV_FEATURES:-}"
DEFAULT_FEATURES="${DEFAULT_FEATURES:-}"
TRAIN_FLAGS="${TRAIN_FLAGS:-}"
RUSTFLAGS="${RUSTFLAGS:-}"
export RUSTFLAGS

# derive the profile if not explicitly set
if [[ -z "${PROFILE:-}" ]]; then
  if [[ ",$QSV_FEATURES," == *",luau,"* ]]; then
    PROFILE="release-luau"
  else
    PROFILE="release"
  fi
fi

# the qsv binary always builds with feature_capable (mirrors publish.yml)
features_arg=""
if [[ -n "$QSV_FEATURES" ]]; then
  if [[ "$BIN" == "qsv" ]]; then
    features_arg="--features=${QSV_FEATURES},feature_capable"
  else
    features_arg="--features=${QSV_FEATURES}"
  fi
fi

# cargo's per-profile env override key: profile name uppercased, '-' -> '_'
profile_env_key="$(echo "$PROFILE" | tr '[:lower:]-' '[:upper:]_')"

echo "================ qsv PGO build ================"
echo "  target          : $TARGET"
echo "  bin             : $BIN"
echo "  profile         : $PROFILE"
echo "  features        : ${features_arg:-(none)}"
echo "  default-features: ${DEFAULT_FEATURES:-(default)}"
echo "  RUSTFLAGS       : ${RUSTFLAGS:-(none)}"
echo "  PGO_LTO override: ${PGO_LTO:-(none - keep Cargo.toml fat-LTO)}"
echo "  train flags     : ${TRAIN_FLAGS:-(none)}"
echo "==============================================="

cargo_args=(--bin "$BIN" --target "$TARGET" --profile "$PROFILE" --locked)
[[ -n "$features_arg" ]] && cargo_args+=("$features_arg")
# DEFAULT_FEATURES is a single token (e.g. --no-default-features); intentional split
# shellcheck disable=SC2206
[[ -n "$DEFAULT_FEATURES" ]] && cargo_args+=($DEFAULT_FEATURES)

# ---- toolchain gate ---------------------------------------------------------
echo ""
echo ">>> [0/3] toolchain check (llvm-tools-preview + cargo-pgo)"
rustup component add llvm-tools-preview
if ! cargo pgo info &>/dev/null; then
  echo "cargo-pgo not found; installing..."
  cargo install cargo-pgo
fi
cargo pgo info || true

# ---- 1. instrument build (thin-LTO) ----------------------------------------
echo ""
echo ">>> [1/3] instrument build (thin-LTO)"
# start from a clean profile set so stale profraw data does not leak in
rm -rf target/pgo-profiles
# override LTO to thin for the instrument build only (fat-LTO + instrumentation is
# slow and historically fragile, rustc #115344). We set the env keys for both the
# release and release-luau profiles so whichever PROFILE is active is covered.
CARGO_PROFILE_RELEASE_LTO=thin \
CARGO_PROFILE_RELEASE_LUAU_LTO=thin \
  cargo pgo instrument build -- "${cargo_args[@]}"

instrumented_bin="target/$TARGET/$PROFILE/$BIN"
if [[ ! -x "$instrumented_bin" ]]; then
  echo "ERROR: instrumented binary not found at $instrumented_bin" >&2
  exit 1
fi

# ---- 2. train ---------------------------------------------------------------
echo ""
echo ">>> [2/3] train"
# shellcheck disable=SC2086
"$script_dir/pgo-train.sh" "$instrumented_bin" $TRAIN_FLAGS

# ---- 3. optimize build ------------------------------------------------------
echo ""
echo ">>> [3/3] optimize build (keeps Cargo.toml fat-LTO unless PGO_LTO is set)"
if [[ -n "${PGO_LTO:-}" ]]; then
  echo "    (LTO override: ${profile_env_key} -> $PGO_LTO)"
  env "CARGO_PROFILE_${profile_env_key}_LTO=$PGO_LTO" \
    cargo pgo optimize build -- "${cargo_args[@]}"
else
  cargo pgo optimize build -- "${cargo_args[@]}"
fi

optimized_bin="target/$TARGET/$PROFILE/$BIN"
echo ""
echo "================ PGO build done ================"
echo "  optimized binary: $repo_root/$optimized_bin"
echo "================================================"

# ---- future: LLVM BOLT (Post-Link Optimization) -----------------------------
# BOLT is a later phase (Linux x86_64 first). cargo-pgo can drive it after the PGO
# optimize build, e.g.:
#   cargo pgo bolt build -- "${cargo_args[@]}"        # BOLT-instrument
#   "$script_dir/pgo-train.sh" <bolt-instrumented> $TRAIN_FLAGS
#   cargo pgo bolt optimize -- "${cargo_args[@]}"     # apply BOLT profile
# Intentionally NOT enabled yet - see issue #1448.
