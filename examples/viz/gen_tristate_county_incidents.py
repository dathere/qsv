#!/usr/bin/env python3
"""Build examples/viz/tristate_county_incidents.csv + tristate_counties.geojson.

SYNTHETIC incidents on REAL counties.

The gallery's sibling figure (district_requests.csv + district_boundaries.geojson) teaches
the count-vs-rate fallacy on six hand-drawn rectangles in a Kansas field. This fixture
teaches the same lesson on geography a reader recognizes: the 210 counties of Ohio,
Pennsylvania and West Virginia (88 + 67 + 55), keyed by real 5-digit county FIPS.

What is real and what is not
----------------------------
REAL:  the county GEOIDs, names and boundaries (TIGERweb, written to
       tristate_counties.geojson beside the CSV), and the population the rate panel divides
       by (ACS 5-year B01003, fetched by `--denominator census@2024`).
FAKE:  the incident rows themselves. There is no real tri-state incident feed behind this;
       the counts are DERIVED from the real populations by the formula below.

Why derived rather than random: the figure's whole claim is "the count map is a population
map, and dividing by population inverts the ranking". A reader must be able to check that.
With this script the check is mechanical -- rerun it and diff.

count_i = round(acs_pop_i * rate_i / 1000)

    rate_i  = base_i * (0.85 + 0.30 * h_i)          # per 1,000 residents
    base_i  = R_HI - (R_HI - R_LO) * (i / (N - 1))  # i = rank ASCENDING by population
    h_i     = blake2b(fips) / 2**64                 # deterministic per-county wobble

So the smallest county gets the highest base rate and the largest the lowest, with a +/-15%
hash-derived wobble so the anti-correlation is not a suspiciously perfect straight line.
blake2b keyed on the FIPS (not random) is what makes re-runs byte-identical.

The spread is deliberate: population across these 210 counties spans ~100x while the rate
spans only 4x, so raw COUNT still tracks population -- the count choropleth genuinely is a
population map -- while the RATE choropleth cleanly inverts the top of the ranking. The
script asserts that inversion (top-10-by-count and top-10-by-rate must not overlap) rather
than trusting it.

There is deliberately NO numeric column in the output. `viz smart` will chart any
role=measure column as a third map panel, and this figure must show exactly two: count and
rate. It also carries no population column -- the point is that the denominator is FETCHED,
not shipped, so a committed copy would only invite a mismatched comparison.

ACS_VINTAGE below MUST stay in step with `--denominator census@2024` in gen_gallery.py.
The committed counts are computed against that release; divide them by a different one and
the caption's numbers stop reproducing.

Source & license
----------------
US Census Bureau TIGERweb, Counties layer (keyless):
https://tigerweb.geo.census.gov/arcgis/rest/services/TIGERweb/State_County/MapServer/1/query
  where=STATE IN ('39','42','54')  outFields=GEOID,NAME  outSR=4326  f=geojson
  maxAllowableOffset=0.001   (server-side generalization; see fetch_boundaries)

US Census Bureau Data API, ACS 5-year detailed tables, total population:
https://api.census.gov/data/2024/acs/acs5?get=B01003_001E,NAME&for=county:*&in=state:NN

Works of the US Census Bureau are US Government works in the PUBLIC DOMAIN under
17 U.S.C. section 105. The generated incident rows are ours, MIT-licensed with qsv.

NOTE, and this is the one place this script contradicts its sibling: gen_northeast_states.py
says "Do NOT switch to api.census.gov: it now rejects keyless requests". That advice holds
for the POPULATION ESTIMATES it uses, which are published as keyless static CSVs. It does
NOT hold here -- ACS table B01003 is published ONLY on the Data API, so a key is genuinely
required. Set QSV_CENSUS_API_KEY (free: https://api.census.gov/data/key_signup.html).
Only this script and a COLD `--denominator census` cache need it; once
~/.qsv-cache/viz-denominators is warm, `qsv viz` makes no request and needs no key.

Usage: gen_tristate_county_incidents.py [outdir]   (default: this script's directory)
"""
import csv
import hashlib
import json
import os
import statistics
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

# Ohio, Pennsylvania, West Virginia. Contiguous, and together they read unmistakably as a
# map -- Lake Erie along the top, the West Virginia panhandles, the Ohio River.
STATE_FIPS = {"39": "OH", "42": "PA", "54": "WV"}
EXPECTED_COUNTIES = 210  # 88 OH + 67 PA + 55 WV

# Keep in step with `--denominator census@2024` in gen_gallery.py. See the docstring.
ACS_VINTAGE = 2024

# Per-1,000-resident incident rate at the extremes of the population ranking. R_HI applies
# to the LEAST populous county, R_LO to the most. The 4x spread is small next to the ~100x
# population spread, which is exactly what keeps raw count tracking population.
R_LO = 0.6
R_HI = 2.4
WOBBLE_LO = 0.85
WOBBLE_SPAN = 0.30

TIGERWEB_COUNTIES = (
    "https://tigerweb.geo.census.gov/arcgis/rest/services/TIGERweb/"
    "State_County/MapServer/1/query"
)
# ArcGIS server-side generalization, in degrees. See fetch_boundaries().
MAX_ALLOWABLE_OFFSET = 0.001

ACS_ROOT = "https://api.census.gov/data"
ACS_POPULATION_TABLE = "B01003_001E"

# Cycled per county so every county carries a spread of both dimensions. Six types x three
# severities = 18 combinations; counties get 8+ rows each, so the smallest still shows
# several. Cycled rather than sampled to keep the file deterministic.
INCIDENT_TYPES = [
    "Road obstruction",
    "Downed line",
    "Water main",
    "Signal outage",
    "Debris",
    "Flooding",
]
SEVERITIES = ["Minor", "Moderate", "Severe"]


# api.census.gov returns transient 503s under load -- observed repeatedly while building this
# fixture, on the same URL that had just succeeded. Retry with backoff rather than failing the
# whole generation; a real outage still surfaces after the last attempt.
FETCH_ATTEMPTS = 5
FETCH_BACKOFF_SECS = 3


def fetch(url, params=None):
    if params:
        url = f"{url}?{urllib.parse.urlencode(params)}"
    last = None
    for attempt in range(1, FETCH_ATTEMPTS + 1):
        try:
            with urllib.request.urlopen(url, timeout=60) as resp:  # noqa: S310
                return resp.read()
        except (urllib.error.HTTPError, urllib.error.URLError, TimeoutError) as exc:
            last = exc
            if attempt == FETCH_ATTEMPTS:
                break
            wait = FETCH_BACKOFF_SECS * attempt
            print(f"  retry {attempt}/{FETCH_ATTEMPTS - 1} after {exc} "
                  f"(sleeping {wait}s)", file=sys.stderr)
            time.sleep(wait)
    sys.exit(f"giving up on {url.split('?')[0]} after {FETCH_ATTEMPTS} attempts: {last}")


def fetch_boundaries():
    """The 210 county polygons, server-simplified, as a GeoJSON FeatureCollection.

    Simplification is not optional here. At full resolution TIGERweb returns 17.8 MB for
    these three states, and `viz smart` embeds the geometry once PER CHOROPLETH TRACE -- this
    figure has two (count and rate), so an unsimplified build inlines ~34 MB into
    gallery.html and roughly triples the committed page. maxAllowableOffset is ArcGIS
    server-side generalization, in degrees. Measured against this exact query:

        offset  0      17.81 MB
        offset  0.001   0.36 MB   <- chosen
        offset  0.002   0.23 MB
        offset  0.005   0.12 MB

    0.001 buys a ~50x reduction and is the conservative pick for COUNTY-sized features.
    Compare gen_northeast_states.py, which picks 0.002 for eleven whole states: coarser
    features tolerate a coarser offset, so do not copy that number here without re-checking
    how the small eastern-Ohio counties render.
    """
    codes = ",".join(f"'{s}'" for s in sorted(STATE_FIPS))
    raw = fetch(
        TIGERWEB_COUNTIES,
        {
            "where": f"STATE IN ({codes})",
            "outFields": "GEOID,NAME",
            "returnGeometry": "true",
            "maxAllowableOffset": MAX_ALLOWABLE_OFFSET,
            "outSR": "4326",
            "f": "geojson",
        },
    )
    fc = json.loads(raw)
    if "features" not in fc:
        sys.exit(f"TIGERweb returned no features: {str(fc)[:300]}")
    # ArcGIS does not guarantee result ordering, so sort for byte-stable re-runs.
    fc["features"].sort(key=lambda f: f["properties"]["GEOID"])
    return fc


def emit_geojson(fc, path):
    """One feature per line so a re-run diffs readably rather than as one giant line."""
    with open(path, "w", encoding="utf-8") as fh:
        fh.write('{"type":"FeatureCollection","features":[\n')
        for i, feat in enumerate(fc["features"]):
            fh.write(json.dumps(feat, separators=(",", ":"), sort_keys=True))
            fh.write(",\n" if i < len(fc["features"]) - 1 else "\n")
        fh.write("]}\n")


def fetch_population(key):
    """GEOID -> ACS 5-year total population, one keyed request per state."""
    out = {}
    for state in sorted(STATE_FIPS):
        raw = fetch(
            f"{ACS_ROOT}/{ACS_VINTAGE}/acs/acs5",
            {
                "get": ACS_POPULATION_TABLE,
                "for": "county:*",
                "in": f"state:{state}",
                "key": key,
            },
        )
        rows = json.loads(raw)
        header, body = rows[0], rows[1:]
        pop_i = header.index(ACS_POPULATION_TABLE)
        st_i, cty_i = header.index("state"), header.index("county")
        for row in body:
            geoid = f"{row[st_i]:0>2}{row[cty_i]:0>3}"
            out[geoid] = int(row[pop_i])
    return out


def wobble(fips):
    """Deterministic per-county multiplier in [0.85, 1.15). Hashed from the FIPS so a
    re-run reproduces the CSV byte for byte."""
    digest = hashlib.blake2b(fips.encode(), digest_size=8).digest()
    unit = int.from_bytes(digest, "big") / 2**64
    return WOBBLE_LO + WOBBLE_SPAN * unit


def build_counts(names, pops):
    """[(fips, name, state, population, count, rate_per_1k)], ranked ascending by pop."""
    items = sorted(pops.items(), key=lambda kv: (kv[1], kv[0]))
    n = len(items)
    rows = []
    for i, (fips, pop) in enumerate(items):
        base = R_HI - (R_HI - R_LO) * (i / (n - 1))
        rate = base * wobble(fips)
        count = round(pop * rate / 1000.0)
        rows.append((fips, names[fips], STATE_FIPS[fips[:2]], pop, count, rate))
    return rows


def emit(rows, path):
    """One row per incident, sorted by FIPS, cycling type x severity within each county.

    `county` is written STATE-QUALIFIED ("Montgomery County, OH"). In this tri-state set 30
    of the 175 distinct county names are carried by more than one county -- Montgomery and
    Franklin are both OH and PA, Mercer and Wayne are all three states -- so a bare name is
    not an identity. The choropleth panels are keyed on county_fips and are unaffected, but
    `viz smart` also draws a top-N frequency bar over this column, and with bare names that
    bar silently SUMS different counties into one row (Montgomery would read 905, which is
    no county's figure). Qualifying by the enclosing state is the same remedy the choropleth
    path applies to name-keyed regions; see issue #4548.
    """
    written = 0
    with open(path, "w", newline="", encoding="utf-8") as fh:
        w = csv.writer(fh, lineterminator="\n")
        w.writerow(["county_fips", "county", "state", "incident_type", "severity"])
        for fips, name, state, _pop, count, _rate in sorted(rows):
            for j in range(count):
                w.writerow([
                    fips,
                    f"{name}, {state}",
                    state,
                    INCIDENT_TYPES[j % len(INCIDENT_TYPES)],
                    SEVERITIES[(j // len(INCIDENT_TYPES)) % len(SEVERITIES)],
                ])
                written += 1
    return written


def report(rows, written, path):
    """The audit trail for the gallery caption. If the top-10 overlap is ever non-empty,
    the figure's central claim is false and the caption must not be written."""
    by_count = sorted(rows, key=lambda r: -r[4])
    by_rate = sorted(rows, key=lambda r: -r[5])
    rates = [r[5] for r in rows]

    print(f"{path}: {written:,} rows, {len(rows)} counties", file=sys.stderr)
    print(f"median rate: {statistics.median(rates):.2f} per 1,000", file=sys.stderr)
    print("\ntop 5 by COUNT (should be the most populous):", file=sys.stderr)
    for fips, name, state, pop, count, rate in by_count[:5]:
        print(f"  {fips} {name}, {state:2}  pop {pop:>9,}  count {count:>6,}  "
              f"rate {rate:.2f}", file=sys.stderr)
    print("\ntop 5 by RATE (should be the rural tail):", file=sys.stderr)
    for fips, name, state, pop, count, rate in by_rate[:5]:
        print(f"  {fips} {name}, {state:2}  pop {pop:>9,}  count {count:>6,}  "
              f"rate {rate:.2f}", file=sys.stderr)

    overlap = {r[0] for r in by_count[:10]} & {r[0] for r in by_rate[:10]}
    if overlap:
        sys.exit(f"\nFAIL: top-10 by count and by rate overlap: {sorted(overlap)}. "
                 "The count-vs-rate inversion this figure teaches does not hold.")
    print("\nOK: top-10 by count and top-10 by rate are disjoint.", file=sys.stderr)


def main():
    outdir = Path(sys.argv[1] if len(sys.argv) > 1 else Path(__file__).parent)
    key = os.environ.get("QSV_CENSUS_API_KEY")
    if not key:
        sys.exit("QSV_CENSUS_API_KEY is not set. ACS table B01003 is published only on the "
                 "Census Data API, which rejects keyless requests. Request a free key at "
                 "https://api.census.gov/data/key_signup.html")

    fc = fetch_boundaries()
    names = {f["properties"]["GEOID"]: f["properties"]["NAME"] for f in fc["features"]}
    pops = fetch_population(key)

    if len(names) != EXPECTED_COUNTIES:
        sys.exit(f"TIGERweb returned {len(names)} counties, expected {EXPECTED_COUNTIES}")
    missing = sorted(set(names) - set(pops))
    if missing:
        sys.exit(f"no ACS {ACS_VINTAGE} population for {len(missing)} counties: {missing[:5]}")
    nonpositive = sorted(f for f in names if pops[f] <= 0)
    if nonpositive:
        sys.exit(f"non-positive population for: {nonpositive[:5]}")

    geo_path = outdir / "tristate_counties.geojson"
    emit_geojson(fc, geo_path)
    print(f"{geo_path}: {len(fc['features'])} features, "
          f"{geo_path.stat().st_size/1048576:.2f} MB "
          f"(maxAllowableOffset={MAX_ALLOWABLE_OFFSET})", file=sys.stderr)

    rows = build_counts(names, {f: pops[f] for f in names})
    path = outdir / "tristate_county_incidents.csv"
    written = emit(rows, path)
    report(rows, written, path)


if __name__ == "__main__":
    main()
