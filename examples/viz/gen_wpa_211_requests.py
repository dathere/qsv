#!/usr/bin/env python3
"""Fetch examples/viz/wpa_211_requests.csv: the WPRDC 2-1-1 Service Requests resource, verbatim.

REAL requests on REAL counties -- nothing here is synthesized, and nothing is reshaped.
The file is the portal's own datastore dump, byte for byte, so a Live Data Schematic of it
(qsv-clientd's Fetch latest) gets data of exactly the shape the gallery page was rendered
from. It is not committed (46 MB): it is attached to the qsv 27.0.1 GitHub release, and
this script re-downloads it from the portal and checks the bytes.

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
the rendered page is the real total for that county over that span, less the 11 rows
the map cannot place (below).

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

The 11 rows the map cannot place
--------------------------------
65 rows have no `state`. `--geojson auto` qualifies each county name by the row's state
(422 US county names belong to more than one state), so a stateless row resolves only
when its county name is unique nationally. Westmoreland (5 rows), Erie (4) and Crawford
(2) are not -- there are Erie counties in NY, OH and PA -- so those 11 rows are left off
the map (`viz smart` reports them on stderr when it renders). Every other stateless row still lands. Filling the blanks
in with "Pennsylvania" would be true, but it would also make this file stop being the
portal's file, which is the property the live schematic depends on.

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

Why zip_code is not tagged as a geography
-----------------------------------------
Tagging a second geo concept would create a competing choropleth candidate scoring
against a county-keyed map -- the same reason tristate_county_incidents tags only ONE
geography. The column stays in the file (it is the portal's file), and
wpa_211_requests_dict.schema.json marks it an identifier instead, so the map stays keyed
on county.

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
import hashlib
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
# than quietly re-cut the figure the committed blurb describes. EXPECTED_SHA256 is the
# strongest of these: the gallery page's input fingerprint, the 27.0.1 release asset and
# visitors' `qsv-clientd register` all depend on these exact bytes.
EXPECTED_ROWS = 279464
EXPECTED_COUNTIES = 25
EXPECTED_SHA256 = "be2a1b488063e9465e7d23c4c193abe539dea661563cd408d6848344043a666b"

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


def parse(raw_bytes):
    return list(csv.DictReader(io.StringIO(raw_bytes.decode("utf-8"))))


def report(rows, path):
    counties = sorted({(r["county"] or "").strip() for r in rows} - {""})
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
    raw = fetch(WPRDC_DUMP)
    digest = hashlib.sha256(raw).hexdigest()
    if digest != EXPECTED_SHA256:
        sys.exit(f"FAIL: sha256 {digest}, expected {EXPECTED_SHA256}. The portal's dump "
                 "changed, so it no longer matches the gallery page's fingerprint or the "
                 "27.0.1 release asset. Re-check the blurb, re-render the page and replace "
                 "the asset before bumping EXPECTED_SHA256.")
    path = outdir / "wpa_211_requests.csv"
    path.write_bytes(raw)
    report(parse(raw), path)


if __name__ == "__main__":
    main()
