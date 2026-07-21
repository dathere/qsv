#!/usr/bin/env bash
#
# viz-golden-check.sh -- byte-level regression harness for `qsv viz smart`.
#
# Renders a fixed matrix of `viz smart` dashboards into an output directory so that two runs can be
# byte-compared. Intended for refactors of `src/cmd/viz.rs` (especially `build_smart` and the
# `SmartCtx` phases), where the invariant is *byte-identical output* and a passing unit test proves
# very little: panel ordering (the order-sensitive `insert(0, ..)` front-slot sequencing), axis
# assignment and domain math can all shift without tripping an assertion.
#
# Usage:
#   scripts/viz-golden-check.sh <outdir>
#
# Typical refactor loop:
#   cargo build --locked --bin qsv -F all_features
#   scripts/viz-golden-check.sh /tmp/viz-before
#   ... make the change, rebuild ...
#   scripts/viz-golden-check.sh /tmp/viz-after
#   diff -rq /tmp/viz-before /tmp/viz-after      # must be empty
#
# Baselines are NOT committed: the 14 HTML dashboards total ~31 MB, and `viz smart` output changes
# legitimately (new panels, plotly bumps), so a stored golden would churn. Capture the "before"
# from the pre-change binary instead.
#
# DETERMINISM: `viz smart` embeds a "Compiled:" wall-clock timestamp, and `--smarter` prints
# "(elapsed: N.NNs)". Both are normalized below -- without that, ~11 of 15 cases differ between two
# runs of the SAME binary and every diff is noise. If you add a case, re-verify that running this
# script twice against one unmodified binary produces zero diffs BEFORE trusting any comparison.
#
# COVERAGE: the cases are the deterministic (no-LLM) `viz smart` invocations from
# examples/viz/gen_gallery.py, plus a headerless run, a static PNG export, and a locally generated
# cyclic-seasonality fixture. Between them they exercise every phase of `build_smart`: the
# association heatmap / Sankey / correlation / 3D / both animated panels, Lorenz, parcats, treemap,
# sunburst, the cyclic polar panel, the KPI row, --max-charts trimming, the dictionary page and the
# metadata table. `--dictionary infer` runs are deliberately excluded (they need a live LLM).

set -uo pipefail

if [[ $# -ne 1 ]]; then
    echo "usage: $0 <outdir>" >&2
    exit 2
fi

SELF="$(cd "$(dirname "$0")" && pwd)"
REPO="$(cd "$SELF/.." && pwd)"
QSV="${QSV:-$REPO/target/debug/qsv}"
FIX="$REPO/examples/viz"
OUT="$1"

if [[ ! -x "$QSV" ]]; then
    echo "error: qsv binary not found at $QSV" >&2
    echo "       build it with: cargo build --locked --bin qsv -F all_features" >&2
    echo "       (or set QSV=/path/to/qsv)" >&2
    exit 2
fi

mkdir -p "$OUT"
rc_total=0

run_case() {
    local name="$1"
    shift
    local ext="html"
    [[ "$name" == img_* ]] && ext="png"

    (cd "$FIX" && "$QSV" viz "$@" --output "$OUT/$name.$ext") \
        >"$OUT/$name.stdout" 2>"$OUT/$name.stderr"
    local rc=$?

    # Normalize the two by-design nondeterministic sinks so a byte-diff is meaningful.
    # perl (not `sed -i`) because in-place editing is spelled incompatibly on BSD vs GNU sed.
    if [[ "$ext" == "html" ]]; then
        perl -i -pe 's/\d{4}-\d{2}-\d{2} \d{2}:\d{2} UTC/<TIMESTAMP>/g' "$OUT/$name.$ext"
    fi
    perl -i -pe 's/\(elapsed: [\d.]+s\)/(elapsed: <T>)/g' "$OUT/$name.stderr"

    if [[ $rc -ne 0 ]]; then
        echo "FAIL($rc) $name" >&2
        rc_total=1
    else
        echo "ok      $name"
    fi
}

run_case seismic     smart seismic_events.csv --smarter --bivariate --theme plotly_dark \
                           --grid-cols 3 --geojson japan_prefectures.geojson
run_case delivery    smart delivery_stops.csv
run_case sales_sun   smart sales_sample.csv --hierarchy-style sunburst --max-charts 8
run_case sales_kpi   smart sales_sample.csv --dictionary sales_kpi_dict.schema.json
run_case cust_smart  smart customer_spend.csv --smarter --max-charts 8
run_case cms         smart cms_medicare_providers.csv --smarter
run_case stocks      smart stock_prices.csv --max-charts 8
run_case uscities    smart us_cities.csv
run_case nyc311      smart nyc_311.csv --smarter --bivariate --dict-info \
                           --dictionary nyc311_dict.schema.json \
                           --geojson nyc_neighborhoods.geojson
run_case allegheny   smart allegheny_dog_licenses.csv --smarter --bivariate --dict-info \
                           --dictionary allegheny_dogs_dict.schema.json \
                           --geojson allegheny_zip_boundaries.geojson \
                           --feature-id-key properties.ZIP
run_case worldevents smart world_events_dated.csv
run_case regions     smart regions_growth.csv
run_case headerless  smart sales_sample.csv --no-headers
run_case img_sales   smart sales_sample.csv
# cyclic-seasonality (polar) panel -- no examples/viz fixture has the hour-of-day periodicity that
# `build_cyclic_panel` needs, so this one ships with the harness.
run_case cyclic      smart "$SELF/viz_golden_cyclic_events.csv"

exit "$rc_total"
