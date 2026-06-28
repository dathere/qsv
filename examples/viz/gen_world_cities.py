#!/usr/bin/env python3
"""Build examples/viz/world_cities.csv from GeoNames cities15000 + countryInfo.

- Filter to city-proper population > 500,000.
- Schema preserved: city,country,continent,lat,lon,metro_population_m,elevation_m,avg_annual_temp_c
- continent uses plotly.js geo `scope` vocabulary (Oceania, North/South America, ...).
- elevation_m is REAL (GeoNames elevation, falling back to the SRTM dem).
- avg_annual_temp_c is SYNTHESIZED (rough latitude+elevation model; not measured).
"""
import csv
import sys

POP_THRESHOLD = 500_000

# GeoNames continent codes -> plotly.js geo scope continent names.
CONTINENT = {
    "AF": "Africa",
    "AS": "Asia",
    "EU": "Europe",
    "NA": "North America",
    "OC": "Oceania",
    "SA": "South America",
    "AN": "Antarctica",
}


def load_countries(path):
    out = {}
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            if line.startswith("#"):
                continue
            f = line.rstrip("\n").split("\t")
            if len(f) < 9 or not f[0]:
                continue
            out[f[0]] = (f[4], CONTINENT.get(f[8], ""))
    return out


def synth_temp(lat, elev):
    # Rough mean-annual-temperature proxy: quadratic falloff with latitude plus a
    # 6.5 C/km environmental lapse rate. Deliberately approximate (no coastal,
    # monsoon, or ocean-current effects).
    t = 27.0 - 0.0066 * lat * lat - 0.0065 * max(elev, 0.0)
    return round(t, 1)


def main(cities_path, country_path, out_path):
    countries = load_countries(country_path)
    rows = []
    with open(cities_path, encoding="utf-8") as fh:
        for line in fh:
            f = line.rstrip("\n").split("\t")
            try:
                pop = int(f[14])
            except (IndexError, ValueError):
                continue
            if pop <= POP_THRESHOLD:
                continue
            cc = f[8]
            country, continent = countries.get(cc, ("", ""))
            if not country or not continent:
                continue
            lat = float(f[4])
            lon = float(f[5])
            elev_raw = f[15].strip()
            if elev_raw not in ("", "0"):
                try:
                    elev = int(round(float(elev_raw)))
                except ValueError:
                    elev = None
            else:
                elev = None
            if elev is None:
                try:
                    dem = int(f[16])
                    elev = dem if dem > -9999 else 0
                except (IndexError, ValueError):
                    elev = 0
            rows.append({
                "city": f[2],
                "country": country,
                "continent": continent,
                "lat": f"{lat:.4f}",
                "lon": f"{lon:.4f}",
                "metro_population_m": round(pop / 1_000_000, 3),
                "elevation_m": elev,
                "avg_annual_temp_c": synth_temp(lat, float(elev)),
            })

    # Group by continent (alpha), then population desc within each continent.
    rows.sort(key=lambda r: (r["continent"], -r["metro_population_m"]))

    cols = ["city", "country", "continent", "lat", "lon",
            "metro_population_m", "elevation_m", "avg_annual_temp_c"]
    with open(out_path, "w", newline="", encoding="utf-8") as fh:
        w = csv.DictWriter(fh, fieldnames=cols, lineterminator="\n")
        w.writeheader()
        w.writerows(rows)
    print(f"wrote {len(rows)} cities to {out_path}")
    by_c = {}
    for r in rows:
        by_c[r["continent"]] = by_c.get(r["continent"], 0) + 1
    for c in sorted(by_c):
        print(f"  {c}: {by_c[c]}")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2], sys.argv[3])
