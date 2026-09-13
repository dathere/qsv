#!/usr/bin/env python3
"""Build examples/viz/wpa_211_requests.csv from the WPRDC 2-1-1 Service Requests feed.

REAL requests on REAL counties -- nothing here is synthesized.

The gallery's tristate_county_incidents figure teaches the count-vs-rate fallacy with
SYNTHETIC incidents whose rates were constructed to invert the ranking. This fixture
teaches the same lesson on a real helpline log, and the honest result is different:
the ranking does NOT invert. Allegheny County leads both maps. What moves is the
MIDDLE -- Venango climbs 11th -> 3rd, Cambria 5th -> 2nd, Butler falls 8th -> 13th --
and that is the point. Per-capita is a different question, not a trick that flips a map.

What is real and what is not
----------------------------
REAL:  every row. These are 2-1-1 helpline requests as published by the United Way of
       Southwestern Pennsylvania via the WPRDC. The resource ENTIRE - no window, no
       sample, no thinning.
REAL:  the county boundaries and the population the rate panel divides by. Neither is
       in this file -- `viz smart --geojson auto --denominator census@2024` fetches the
       boundaries from Census TIGERweb and the ACS 5-year B01003 population from the
       Census Data API at render time.
FAKE:  nothing.

Scope: the whole 2023-2025 resource, and why not the 2020-2023 one
-----------------------------------------------------------------
This takes the upstream resource ENTIRE -- every row it publishes, 2023-09-22 through
2025-08-27 -- rather than a window or a sample. Nothing is thinned, so every count on
the rendered page is the real total for that county over that span.

The WPRDC dataset also carries an OLDER resource, "2-1-1 Requests (2020-2023)"
(94f52ddb-acf3-4728-93b9-4544988f1d6f). It is deliberately NOT concatenated here, and
the reason is a data shape, not a preference:

  * 5.4% of its rows carry a PIPE-DELIMITED MULTI-COUNTY value --
    "Allegheny County|Westmoreland County" and 480-odd more combinations, 500 distinct
    "county" strings in all. Those are ZIP codes straddling a county line. Every way of
    resolving them is a judgment call that changes the map: drop them and lose 5.4% of
    the volume, split them and double-count, take the first and silently bias toward
    whichever county sorts earlier.
  * it spans 40+ states, where the 2023-2025 resource is 100% Pennsylvania.

A figure whose whole point is a defensible per-capita denominator should not open with
an undocumented tie-break, so the clean resource is used whole and the messy one is
left out rather than quietly reshaped.

Why no county_fips column
-------------------------
An earlier draft added one. It is not needed: `--geojson auto` fetches the county
boundaries AND canonicalizes the county-NAME column to Census GEOIDs, which is what
lets the fetched population key onto them. Measured: 25 of 25 counties resolve.

Do NOT "improve" this by deriving a FIPS column with `qsv geocode`. Its suggest/
suggestnow subcommands search CITY names and return the closest city's county, so over
county names they are a silent wrong answer (the issue #4417 Part B finding:
"Litchfield County" -> Maricopa County AZ; "Allegheny County" works only by luck, via
Pittsburgh). refresh_external_dashboards.sh records the same class of trap for
PennDOT's own county code.

Why zip_code is dropped
-----------------------
The upstream feed carries one. Tagging a second geo concept would create a competing
choropleth candidate scoring against a county-keyed map -- the same reason
tristate_county_incidents tags only ONE geography. It is dropped here rather than
mis-tagged.

ACS_VINTAGE below is documentation only -- this script makes no Census request. It
records the release the committed blurb's per-1,000 numbers were computed against, and
MUST stay in step with `--denominator census@2024` in gen_gallery.py. Divide these
counts by a different release and the caption stops reproducing.

Source & license
----------------
2-1-1 Service Requests, United Way of Southwestern Pennsylvania, via the Western
Pennsylvania Regional Data Center: https://data.wprdc.org/dataset/211-requests
Resource "2-1-1 Requests/Referrals (2023-2025) from Western Pennsylvania",
id 0ce22487-5707-4e64-8e59-5b189cfc69cb. Licensed Creative Commons Attribution.

Usage: gen_wpa_211_requests.py [outdir]   (default: this script's directory)
"""
import csv
import io
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

WPRDC_RESOURCE = "0ce22487-5707-4e64-8e59-5b189cfc69cb"
WPRDC_DUMP = f"https://data.wprdc.org/datastore/dump/{WPRDC_RESOURCE}"
DATASET_PID = "https://data.wprdc.org/dataset/211-requests"

# Keep in step with `--denominator census@2024` in gen_gallery.py. See the docstring.
ACS_VINTAGE = 2024

# The resource entire -- no window, no sample. These bounds are ASSERTED, not applied:
# they record the span the committed blurb's numbers were computed over, so an upstream
# extension past them trips the row-count check below rather than silently re-cutting
# the figure.
SPAN_LO = "2023-09-22"
SPAN_HI = "2025-08-27"

# Asserted, not assumed: a silently reshaped upstream feed must fail this script rather
# than quietly re-cut the figure the committed blurb describes.
EXPECTED_ROWS = 279464
EXPECTED_COUNTIES = 25

OUT_COLUMNS = [
    "contact_date",
    "county",
    "state",
    "gender",
    "age_range",
    "level_1_classification",
    "needs_met",
]

# The portal returns transient failures under load, exactly as api.census.gov does for
# the tristate fixture. Retry with backoff; a real outage still surfaces after the last
# attempt.
FETCH_ATTEMPTS = 5
FETCH_BACKOFF_SECS = 3


def fetch(url):
    last = None
    for attempt in range(1, FETCH_ATTEMPTS + 1):
        try:
            with urllib.request.urlopen(url, timeout=300) as resp:  # noqa: S310
                return resp.read()
        except (urllib.error.HTTPError, urllib.error.URLError, TimeoutError) as exc:
            last = exc
            if attempt == FETCH_ATTEMPTS:
                break
            wait = FETCH_BACKOFF_SECS * attempt
            print(f"  {url}: {exc} -- retrying in {wait}s "
                  f"({attempt}/{FETCH_ATTEMPTS})", file=sys.stderr)
            time.sleep(wait)
    sys.exit(f"FAILED after {FETCH_ATTEMPTS} attempts: {url}\n  last error: {last}")


def normalize_county(raw):
    """'Allegheny' and 'Allegheny County' both appear upstream -- the 2020-2023 and
    2023-2025 resources disagree on the suffix. Emit one spelling so the frequency bar
    cannot split a county across two bars."""
    name = (raw or "").strip()
    if not name:
        return ""
    if name.lower().endswith(" county"):
        name = name[: -len(" county")].strip()
    return f"{name} County" if name else ""


def build(raw_bytes):
    reader = csv.DictReader(io.StringIO(raw_bytes.decode("utf-8")))
    rows = []
    for row in reader:
        date = (row.get("contact_date") or "").strip()
        if not date:
            continue
        county = normalize_county(row.get("county"))
        if not county:
            continue
        rows.append({
            "contact_date": date,
            "county": county,
            # Constant by construction: this is the Western PA 2-1-1 region. Carried so
            # the dictionary can tag it geo.state, which scopes the boundary fetch to
            # one state instead of probing the nation.
            "state": "PA",
            "gender": (row.get("gender") or "").strip(),
            "age_range": (row.get("age_range") or "").strip(),
            "level_1_classification": (row.get("level_1_classification") or "").strip(),
            "needs_met": (row.get("needs_met") or "").strip(),
        })
    # Sort so a re-run is byte-identical regardless of upstream row order.
    rows.sort(key=lambda r: tuple(r[c] for c in OUT_COLUMNS))
    return rows


def emit(rows, path):
    with path.open("w", newline="", encoding="utf-8") as fh:
        writer = csv.DictWriter(fh, fieldnames=OUT_COLUMNS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def report(rows, path):
    counties = sorted({r["county"] for r in rows})
    lo = min(r["contact_date"] for r in rows)
    hi = max(r["contact_date"] for r in rows)
    print(f"{path}: {len(rows):,} rows, {len(counties)} counties, "
          f"{path.stat().st_size / 1048576:.2f} MB", file=sys.stderr)
    print(f"  span {lo}..{hi}   denominator release ACS {ACS_VINTAGE} "
          f"(fetched at render time, not stored here)", file=sys.stderr)

    if len(counties) != EXPECTED_COUNTIES:
        sys.exit(f"\nFAIL: {len(counties)} counties, expected {EXPECTED_COUNTIES}. "
                 f"The upstream feed changed shape: {counties}")
    if (lo, hi) != (SPAN_LO, SPAN_HI):
        sys.exit(f"\nFAIL: span {lo}..{hi}, expected {SPAN_LO}..{SPAN_HI}. The upstream "
                 "resource grew or was re-cut; every count in the gen_gallery.py blurb "
                 "is computed over the old span.")
    if len(rows) != EXPECTED_ROWS:
        sys.exit(f"\nFAIL: {len(rows):,} rows, expected {EXPECTED_ROWS:,}. The upstream "
                 "feed was revised. Re-check the blurb's counts in gen_gallery.py "
                 "against the new cut before bumping EXPECTED_ROWS.")
    print(f"\nOK: {EXPECTED_ROWS:,} rows across {EXPECTED_COUNTIES} counties, as the "
          "committed blurb describes.", file=sys.stderr)


def main():
    outdir = Path(sys.argv[1] if len(sys.argv) > 1 else Path(__file__).parent)
    print(f"fetching {WPRDC_DUMP}", file=sys.stderr)
    rows = build(fetch(WPRDC_DUMP))
    path = outdir / "wpa_211_requests.csv"
    emit(rows, path)
    report(rows, path)


if __name__ == "__main__":
    main()
