#!/usr/bin/env python3
"""Regenerate examples/viz/gallery.html from the current qsv binary.

`gallery.html` is a lightweight, CDN-plotly page rendering every `qsv viz`
chart type from the sample datasets in this directory. It is a checked-in
*artifact*; this script is the source of truth for how it's produced.

For each figure it runs the documented `qsv viz` command, extracts the figure
JSON object that plotly-rs emits (`Plotly.newPlot(graph_div, {...})`) from the
self-contained output, and reassembles them into one page that loads plotly from
the CDN (so the committed file stays small). The static scaffold (head / style /
header) is reused verbatim from the existing gallery, so re-running this only
changes figure content and order.

Usage (from the repo root), after changing viz output or the datasets:

    cargo build --bin qsv -F all_features
    python3 examples/viz/gen_gallery.py

Set QSV_BIN to point at a specific binary; otherwise target/{debug,release}/qsv
or a `qsv` on PATH is used. Re-run and commit gallery.html if the diff is what
you expect. The per-figure commands below are mirrored in README.md.
"""
import json
import os
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from html import escape as html_escape

VIZ_DIR = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(VIZ_DIR))
GALLERY = os.path.join(VIZ_DIR, "gallery.html")
MARKER = "Plotly.newPlot(graph_div, "

# CDN plotly tag substituted for the ~4.6MB inline bundle in the smart-dashboard iframes.
# Keep the version in sync with the gallery head's <script src> (plotly.js v3.6.0).
PLOTLY_CDN = '<script src="https://cdn.plot.ly/plotly-3.6.0.min.js" charset="utf-8"></script>'

# Smart dashboards are embedded as iframes of the *genuine* `qsv viz smart` HTML output (so the
# full-width overview panels, themes and map buttons render exactly as the CLI produces them),
# rather than reconstructed as a lossy uniform sub-grid. Keyed by figure title -> iframe filename.
SMART_IFRAME = {
    "smart dashboard":                          "smart_sales.html",
    "smart dashboard (--smarter)":              "smart_smarter.html",
    "smart dashboard (--smarter, geospatial)":  "smart_geospatial.html",
    "smart dashboard (geographic outliers)":    "smart_geo_outliers.html",
    "smart dashboard (time-series)":            "smart_timeseries.html",
    "smart dashboard (per-US-state choropleth)":      "smart_us_choropleth.html",
    "smart dashboard (--dictionary infer, treemap)":  "smart_dict_treemap.html",
    "smart dashboard (--dictionary infer, sunburst)": "smart_dict_sunburst.html",
    "smart dashboard (--dictionary infer, world choropleth)": "smart_world_choropleth.html",
    "smart dashboard (--smarter, --dictionary infer, NYC 311 metro choropleth)": "smart_nyc311.html",
}

# Iframe artifacts that depend on a live LLM (`--dictionary infer` calls describegpt against a
# local LM Studio / Ollama endpoint). Their committed HTML is REUSED as-is rather than regenerated,
# so a normal `gen_gallery.py` run stays LLM-free and deterministic. To refresh them, run the
# `qsv viz smart ... --dictionary infer` commands from README.md (with your LLM up) and re-cdnify.
PREGENERATED = {
    "smart_dict_treemap.html",
    "smart_dict_sunburst.html",
    "smart_world_choropleth.html",
    "smart_nyc311.html",
}

# CSS for the smart-dashboard iframes. `scrolling="no"` + `overflow:hidden` plus the postMessage
# auto-sizing below mean each iframe ends up exactly as tall as its dashboard — no inner scrollbar,
# no trailing whitespace. The height here is just an initial value before the first height message.
DASH_CSS = ("figure.full iframe.dash{width:100%;border:0;height:600px;display:block;"
            "border-radius:6px;overflow:hidden}")

# GitHub-style copy icons (Octicons, 16x16, fill=currentColor). The button shows the "copy" icon
# (two overlapping squares) and swaps to a green "check" on success — both are present in the
# button and toggled by the `.ok` class (no innerHTML churn, no SVG strings in JS).
COPY_ICON_SVG = ('<svg class="ci-copy" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true">'
                 '<path d="M0 6.75C0 5.784.784 5 1.75 5h1.5a.75.75 0 0 1 0 1.5h-1.5a.25.25 0 0 0-.25'
                 '.25v7.5c0 .138.112.25.25.25h7.5a.25.25 0 0 0 .25-.25v-1.5a.75.75 0 0 1 1.5 0v1.5A1.'
                 '75 1.75 0 0 1 9.25 16h-7.5A1.75 1.75 0 0 1 0 14.25Z"></path>'
                 '<path d="M5 1.75C5 .784 5.784 0 6.75 0h7.5C15.216 0 16 .784 16 1.75v7.5A1.75 1.75 0 '
                 '0 1 14.25 11h-7.5A1.75 1.75 0 0 1 5 9.25Zm1.75-.25a.25.25 0 0 0-.25.25v7.5c0 .138.11'
                 '2.25.25.25h7.5a.25.25 0 0 0 .25-.25v-7.5a.25.25 0 0 0-.25-.25Z"></path></svg>')
CHECK_ICON_SVG = ('<svg class="ci-check" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true">'
                  '<path d="M13.78 4.22a.75.75 0 0 1 0 1.06l-7.25 7.25a.75.75 0 0 1-1.06 0L1.72 9.78a.'
                  '751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018L6 11.94l6.72-6.72a.75.75 0 0 1 1.'
                  '06 0Z"></path></svg>')

# Styling for the per-figure, copy-pasteable `qsv viz` command block rendered under each
# description. The block wraps long commands (with a ` \` shell continuation) and `<pre>` preserves
# those newlines; overflow-x:auto still scrolls any stray over-wide line. `.cmdbox` is the relative
# anchor for the GitHub-style icon-only Copy button (top-right); pre.cmd's right padding keeps text
# clear of it. The button is subtle by default, darker on hover, and shows a green check on success.
CMD_CSS = ("figure .cmdbox{position:relative;margin:6px 4px 0}"
           "figure pre.cmd{background:#f3f5f9;border-radius:6px;padding:8px 40px 8px 10px;"
           "margin:0;overflow-x:auto;font:11.5px/1.4 SFMono-Regular,Consolas,Menlo,monospace;"
           "color:#2A3F5F;white-space:pre}"
           "figure button.copy{position:absolute;top:6px;right:6px;display:inline-flex;"
           "align-items:center;justify-content:center;width:28px;height:28px;padding:0;"
           "border:1px solid transparent;background:transparent;color:#57606a;border-radius:6px;"
           "cursor:pointer}"
           "figure button.copy:hover{color:#24292f;background:#eef1f6;border-color:#d4d9e3}"
           "figure button.copy svg{width:16px;height:16px;fill:currentColor;display:block}"
           "figure button.copy .ci-check{display:none}"
           "figure button.copy.ok{color:#1a7f37;border-color:transparent;background:transparent}"
           "figure button.copy.ok .ci-copy{display:none}"
           "figure button.copy.ok .ci-check{display:block}")

# Injected once into the gallery: copies a command block's single-line form (data-cmd) to the
# clipboard. Uses the async Clipboard API when available (https / localhost) and falls back to a
# hidden-textarea + execCommand("copy") for file:// where the API is absent. Flips the button label
# to "Copied!" for ~1.2s on success.
COPY_JS = (
    "<script>document.addEventListener(\"click\",function(e){"
    "var b=e.target.closest&&e.target.closest(\"button.copy\");if(!b)return;"
    "var cmd=b.getAttribute(\"data-cmd\");"
    "function ok(){if(b._t)clearTimeout(b._t);b.classList.add(\"ok\");b.title=\"Copied!\";"
    "b._t=setTimeout(function(){b.classList.remove(\"ok\");b.title=\"Copy\";b._t=null;},1200);}"
    "if(navigator.clipboard&&navigator.clipboard.writeText){"
    "navigator.clipboard.writeText(cmd).then(ok,function(){fallback();});}else{fallback();}"
    "function fallback(){var t=document.createElement(\"textarea\");t.value=cmd;"
    "t.style.position=\"fixed\";t.style.opacity=\"0\";document.body.appendChild(t);t.select();"
    "try{if(document.execCommand(\"copy\"))ok();}catch(err){}document.body.removeChild(t);}"
    "});</script>")

# Injected into each smart_*.html so the dashboard reports its real rendered height to the parent
# gallery. postMessage works cross-origin (e.g. when the gallery is opened over file://), unlike
# reading iframe.contentWindow.document; the ResizeObserver re-reports after plotly's async relayout.
RESIZE_REPORTER_JS = (
    "<script>(function(){function r(){parent.postMessage("
    "{qsvVizHeight:document.documentElement.scrollHeight},\"*\");}"
    "addEventListener(\"load\",r);addEventListener(\"resize\",r);"
    "if(window.ResizeObserver)new ResizeObserver(r).observe(document.body);"
    "setTimeout(r,200);setTimeout(r,800);})();</script>")

# Added to the gallery once: sizes each iframe to the height its dashboard reports (matched by
# comparing window references, which is allowed cross-origin). The reported height is
# documentElement.scrollHeight = max(content, viewport), so set the iframe to exactly that and
# ONLY when it actually differs (>1px) from the iframe's current height — never add padding on top.
# Otherwise, since enlarging the iframe enlarges the child viewport, the next report would echo the
# new height and the iframe would creep upward 1 step per resize. With this guard it converges to
# the content height: once iframe == content, the report equals the current height and we stop.
RESIZE_LISTENER_JS = (
    "<script>addEventListener(\"message\",function(e){"
    "var h=e.data&&e.data.qsvVizHeight;if(typeof h!==\"number\")return;"
    "var f=document.getElementsByClassName(\"dash\");"
    "for(var i=0;i<f.length;i++)if(f[i].contentWindow===e.source){"
    "if(Math.abs(f[i].clientHeight-h)>1)f[i].style.height=h+\"px\";break;}});</script>")

BANNER = (
    "<!-- AUTO-GENERATED by examples/viz/gen_gallery.py — do not edit by hand.\n"
    "     Regenerate (from the repo root) after changing viz output or the datasets:\n"
    "       cargo build --bin qsv -F all_features && python3 examples/viz/gen_gallery.py\n"
    "-->"
)

# (title, description, full_width, [viz args]). Order matters: the full-width smart dashboards
# lead and close the contiguous run of individual chart types.
FIGURES = [
    ("smart dashboard (--smarter, geospatial)",
     "One `qsv viz smart seismic_events.csv --smarter --theme plotly_dark --grid-cols 3 "
     "--geojson japan_prefectures.geojson --feature-id-key properties.id` command, 11 auto-chosen "
     "panels — nearly every "
     "panel type at once on a synthetic catalog of Japanese earthquakes. Things the raw table hides "
     "but the dashboard makes obvious: depth_km is <b>bimodal</b> (two populations — shallow "
     "interplate quakes ~20&nbsp;km and the deep Wadati-Benioff slab ~450&nbsp;km — so --smarter "
     "draws a histogram, not a box that would average the peaks away); the points trace Japan's "
     "subduction arcs on the map; and a <b>prefecture choropleth</b> bins each quake into the "
     "GeoJSON region that contains it (point-in-polygon, no geocoding). Most of this catalog is "
     "offshore Pacific seismicity, so under the default 10&nbsp;km snap cap the far-offshore quakes "
     "are dropped (287 of 417 here — the panel title reports it) and the on-land/near-coast "
     "prefectures are colored; raise <code>--snap-max-dist</code> to snap distant quakes to the "
     "nearest prefecture instead, or <code>--no-snap</code> to drop every offshore point. "
     "magnitude vs felt_reports is almost perfectly correlated "
     "(r=0.95); magnitude and felt_reports are right-skewed with flagged outliers; and the "
     "magnitude-over-time trend spikes during a September aftershock sequence. Coordinate columns "
     "are shown on the map only, not re-charted as distributions. Rendered with the built-in "
     "<code>plotly_dark</code> theme.",
     True, ["smart", "seismic_events.csv", "--smarter", "--theme", "plotly_dark", "--grid-cols", "3",
            "--geojson", "japan_prefectures.geojson", "--feature-id-key", "properties.id"]),
    ("smart dashboard (geographic outliers)",
     "`qsv viz smart delivery_stops.csv` — delivery stops clustered in metro Denver with four "
     "bad-geocode strays. Points far from the cluster centroid (beyond the Tukey far-out fence of "
     "their distances) are flagged as geographic <b>outliers</b>: drawn as distinct amber markers, "
     "drawn outside the purple (filled) spatial-extent box, and excluded from the auto-zoom — so the "
     "default view stays tight on the core cluster. A second, dashed-magenta no-fill box marks the "
     "full extent (core + outliers); use the <b>Core extent</b> / <b>Full extent</b> buttons at the "
     "top-left of the map to jump between the tight core view and the full spread (where the strays "
     "and the magenta box become visible). In the "
     "full <code>qsv viz smart</code> HTML output the spatial-extent label calls them out — "
     "<i>Colorado, United States &mdash; 4 outliers (Wyoming, Kansas &amp; Nebraska)</i> — while "
     "strays within the core's own jurisdiction are folded back in silently instead. "
     "Each stop also carries delivery attributes (<code>packages</code>, <code>weight_kg</code>, "
     "<code>distance_km</code>, <code>delivery_minutes</code>, a <code>vehicle</code> class and a "
     "<code>delivered_date</code>), so beyond the map the auto-profiler fills the dashboard out with "
     "box plots, frequency bars, a correlation heatmap, the strongest-pair scatter "
     "(packages vs weight_kg) and a delivered-over-time trend — all without <code>--smarter</code>.",
     True, ["smart", "delivery_stops.csv"]),
    ("smart dashboard",
     "Auto-profiled overview: correlation heatmap + box plots + frequency bars, led by a "
     "drill-down sunburst. `viz smart` now SKIPS an auto hierarchy when the candidate dimensions "
     "are statistically independent (nesting them would just replicate each level's marginal); "
     "sales_sample's region/payment_method/product_category are independent, so "
     "`--hierarchy-style sunburst` is passed to deliberately showcase the interactive sunburst.",
     True, ["smart", "sales_sample.csv", "--hierarchy-style", "sunburst", "--max-charts", "8"]),
    ("smart dashboard (--smarter)",
     "Same auto-profiler with `--smarter`, which runs `qsv moarstats --advanced` itself to enrich "
     "the stats cache in one step: the bimodal monthly_spend column renders as a histogram (a box "
     "plot would hide its two peaks), and the skewed account_age_days box is annotated with its "
     "skew direction and outlier share.",
     True, ["smart", "customer_spend.csv", "--smarter", "--max-charts", "8"]),
    ("bar", "Revenue by region (aggregated sum).",
     False, ["bar", "sales_sample.csv", "--x", "region", "--y", "revenue", "--agg", "sum"]),
    ("line", "Closing price over time.",
     False, ["line", "stock_prices.csv", "--x", "date", "--y", "close"]),
    ("scatter", "Units sold vs revenue.",
     False, ["scatter", "sales_sample.csv", "--x", "units_sold", "--y", "revenue"]),
    ("scatter (bubble)", "Units vs revenue; marker size = shipping cost, color = profit margin %.",
     False, ["scatter", "sales_sample.csv", "--x", "units_sold", "--y", "revenue",
             "--size", "shipping_cost", "--color", "profit_margin_pct"]),
    ("scatter3d", "Units vs revenue vs shipping cost in 3D; marker color = profit margin %.",
     False, ["scatter3d", "sales_sample.csv", "--x", "units_sold", "--y", "revenue",
             "--z", "shipping_cost", "--color", "profit_margin_pct"]),
    ("histogram", "Distribution of unit price.",
     False, ["histogram", "sales_sample.csv", "--x", "unit_price"]),
    ("box", "Spread of revenue (Tukey whiskers; points beyond the fences shown as outliers).",
     False, ["box", "sales_sample.csv", "--y", "revenue"]),
    ("box (grouped)", "Revenue spread per region — real Tukey whiskers + every (jittered) point overlaid (--box-points all).",
     False, ["box", "sales_sample.csv", "--y", "revenue", "--x", "region", "--box-points", "all"]),
    ("pie (donut)", "Revenue share by product category.",
     False, ["pie", "sales_sample.csv", "--x", "product_category", "--y", "revenue", "--donut"]),
    ("heatmap (correlation)", "Pearson correlation matrix over numeric columns.",
     False, ["heatmap", "sales_sample.csv"]),
    ("scatter (correlated pair)",
     "The most strongly correlated numeric pair (discount_pct vs profit_margin_pct, r=-0.99). "
     "viz smart auto-adds this as a drill-down beside the correlation heatmap.",
     False, ["scatter", "sales_sample.csv", "--x", "discount_pct", "--y", "profit_margin_pct"]),
    ("contour", "2D density of units sold vs revenue (binned into a 20x20 grid). viz smart uses "
     "this instead of the pair scatter for large datasets, where a scatter would overplot.",
     False, ["contour", "sales_sample.csv", "--x", "units_sold", "--y", "revenue", "--bins", "20"]),
    ("heatmap (pivot)", "Region x category grid of revenue.",
     False, ["heatmap", "sales_sample.csv", "--x", "region", "--y", "product_category", "--z", "revenue"]),
    ("candlestick", "OHLC price action.",
     False, ["candlestick", "stock_prices.csv", "--x", "date", "--ohlc-open", "open",
             "--high", "high", "--low", "low", "--close", "close"]),
    ("ohlc", "Open-high-low-close bars.",
     False, ["ohlc", "stock_prices.csv", "--x", "date", "--ohlc-open", "open",
             "--high", "high", "--low", "low", "--close", "close"]),
    ("sankey", "Web session funnel (duplicate edges aggregated).",
     False, ["sankey", "web_flows.csv", "--source", "source", "--target", "target", "--value", "sessions"]),
    ("radar", "Multi-axis brand comparison (per-axis mean per series).",
     False, ["radar", "product_ratings.csv", "--cols", "battery,camera,performance,display,value,design",
             "--series", "brand"]),
    ("treemap", "Part-to-whole spend by plan then region, sized by summed monthly_spend. Rounded "
     "tiles + white separators come from the treemap-specific marker; non-numeric/negative measure "
     "cells are rejected so proportions can't silently misstate.",
     False, ["treemap", "customer_spend.csv", "--cols", "plan,region", "--value", "monthly_spend",
             "--agg", "sum"]),
    ("sunburst", "Three-level hierarchy (region -> product_category -> payment_method) as concentric "
     "rings, sized by row count; inner rings are parents, outer rings their children. Opens at two "
     "rings (maxdepth) so labels stay legible instead of crowding a ~100-sector outer ring; click a "
     "sector to drill in and the deeper ring's labels grow back. Hover always shows value + percent.",
     False, ["sunburst", "sales_sample.csv", "--cols", "region,product_category,payment_method"]),
    ("map", "Earthquake points on token-free OpenStreetMap tiles; marker color = magnitude, size = depth.",
     False, ["map", "quakes.csv", "--lat", "lat", "--lon", "lon", "--color", "magnitude", "--size", "depth_km"]),
    ("map (density)", "DensityMapbox heatmap of the same points on a light Carto basemap.",
     False, ["map", "quakes.csv", "--lat", "lat", "--lon", "lon", "--density", "--style", "carto-positron"]),
    ("geo", "Same earthquakes on an offline natural-earth projection (no tiles, no token); marker "
     "color = magnitude. viz smart auto-uses this projection for global-extent coordinates.",
     False, ["geo", "quakes.csv", "--lat", "lat", "--lon", "lon", "--color", "magnitude",
             "--projection", "natural-earth"]),
    ("choropleth", "Filled-region map coloring countries by GDP, matched by ISO-3 code on a "
     "token-free projection basemap. Use --location-mode usa-states / country-names / geojson-id "
     "for other region keys, --map for a MapLibre tile basemap, or --geocode to derive codes from "
     "lat/lon or place names.",
     False, ["choropleth", "country_stats.csv", "--locations", "iso3", "--value", "gdp_usd_tn",
             "--color-scale", "viridis"]),
    ("choropleth (US states)", "Same chart, <code>--location-mode usa-states</code>: state codes "
     "matched to Plotly's built-in US-state geometry on the token-free albers-usa projection "
     "(CONUS + Alaska/Hawaii insets) — no GeoJSON needed. States are colored by renewable-electricity "
     "share.",
     False, ["choropleth", "us_state_stats.csv", "--locations", "state", "--value",
             "renewable_electricity_pct", "--location-mode", "usa-states"]),
    ("choropleth (MapLibre + GeoJSON)", "<code>--map</code> draws the filled regions on an "
     "interactive MapLibre <b>tile</b> basemap (token-free carto-positron) instead of a projection. "
     "The regions come from a custom GeoJSON (<code>--geojson</code> local file or URL) matched to "
     "the data by <code>--feature-id-key</code> — here the near-rectangular western states, colored "
     "by installed wind capacity. The view auto-centers and zooms to the GeoJSON extent (shown "
     "full-width so the computed zoom frames the regions as the CLI does — a tile map's zoom is "
     "fixed, so a narrow grid cell would crop it).",
     True, ["choropleth", "western_states.csv", "--locations", "state", "--value",
             "wind_capacity_gw", "--geojson", "western_states.geojson", "--feature-id-key", "id",
             "--map", "--style", "carto-positron"]),
    ("smart dashboard (time-series)",
     "Auto dashboard for stock_prices: a time-series trend panel (the first numeric column over the "
     "date) leads, alongside box-plot summaries of the OHLC columns.",
     True, ["smart", "stock_prices.csv", "--max-charts", "8"]),
    ("smart dashboard (per-US-state choropleth)",
     "`qsv viz smart us_cities.csv` — `viz smart` reverse-geocodes each point; because every city "
     "resolves to a US state, it adds a per-US-<b>state</b> choropleth (cities-per-state, albers-usa) "
     "beside the point map, alongside the usual box plots, frequency bars and the strongest-pair "
     "scatter. (The point map's <i>spatial extent</i> caption counts the data's bounding-box corners, "
     "which spill into neighboring countries and ocean — the choropleth instead resolves each city to "
     "its own state.) No flags, no LLM — the state fill is derived purely from the lat/lon columns.",
     True, ["smart", "us_cities.csv"]),
    ("smart dashboard (--dictionary infer, treemap)",
     "Auto dashboard for customer_spend with a describegpt-inferred Data Dictionary "
     "(--dictionary infer) guiding panel selection & field labels. Two categorical dimensions "
     "(plan, region) form a shallow part-to-whole hierarchy, auto-rendered as a TREEMAP "
     "(area = size). Requires a local LLM; the committed HTML is reused on regen.",
     True, ["smart", "customer_spend.csv", "--dictionary", "infer"]),
    ("smart dashboard (--dictionary infer, sunburst)",
     "Auto dashboard for sales_sample with a describegpt-inferred Data Dictionary. Its three "
     "categorical dimensions are statistically independent, so the auto-profiler skips the "
     "hierarchy by default; `--hierarchy-style sunburst` forces a SUNBURST here (concentric rings "
     "emphasize parent-child structure) to showcase the chart. Requires a local LLM; the committed "
     "HTML is reused on regen.",
     True, ["smart", "sales_sample.csv", "--dictionary", "infer", "--hierarchy-style", "sunburst"]),
    ("smart dashboard (--dictionary infer, world choropleth)",
     "`qsv viz smart world_cities.csv --dictionary infer` — <b>1,179 cities</b> with population "
     "over 500,000 across <b>six inhabited continents</b> (GeoNames-derived): `viz smart` "
     "reverse-geocodes every point and adds a per-<b>country</b> choropleth (cities-per-country, "
     "ISO-3) <b>framed to the filled-country geometries</b> via Plotly <code>fitbounds</code> — so "
     "the regions are never clipped at the viewport edge — beside the dense natural-earth point map "
     "(crimson markers so coastal/island points read against the ocean), plus a six-continent "
     "breakdown. A describegpt-inferred Data Dictionary supplies the friendly field labels (e.g. "
     "<i>Metro Population</i>, <i>Avg Annual Temp</i>). The <code>continent</code> column follows "
     "the <a href=\"https://plotly.com/javascript/reference/layout/geo/#layout-geo-scope\">plotly.js "
     "geo <code>scope</code></a> continent vocabulary (<i>Oceania</i>, <i>North America</i>, …). "
     "<b>Note:</b> <code>elevation_m</code> is real (GeoNames), while <code>avg_annual_temp_c</code> "
     "is a rough synthetic proxy (latitude + elevation-lapse model), so treat it as illustrative. "
     "Requires a local LLM; the committed HTML is reused on regen.",
     True, ["smart", "world_cities.csv", "--dictionary", "infer"]),
    ("smart dashboard (--smarter, --dictionary infer, NYC 311 metro choropleth)",
     "`qsv viz smart nyc_311.csv --smarter --dictionary infer --geojson nyc_neighborhoods.geojson` "
     "— a <b>10,000-row</b> sample of NYC 311 service requests (2010–2020) profiled into ~38 "
     "auto-chosen panels, nearly every panel type at once on a real, wide municipal dataset. The "
     "headline panel is the <b>metro choropleth</b>: each request's lat/lon is binned by "
     "point-in-polygon into one of <b>188 NYC neighborhood</b> polygons (no geocoding), and because "
     "the matched regions span a city-scale extent, <code>viz smart</code> now draws the filled "
     "regions on an interactive <b>MapLibre tile basemap</b> (token-free carto tiles, fine "
     "street/coastline detail) instead of the coarse projection basemap it uses for "
     "country/continental choropleths. The leading <b>point map</b> flags bad-geocode "
     "<b>outliers</b> (here, <i>9 in Pennsylvania</i>): a nice illustration of the hover's value — "
     "for the stray PA point, the tooltip shows the record's own <i>Incident City: NEW YORK</i> "
     "right above its reverse-geocoded <i>Pennsylvania, United States</i>, so a real Manhattan "
     "complaint saddled with corrupt coordinates is self-evident at a glance. "
     "<code>--smarter</code> runs <code>moarstats --advanced</code> "
     "to enrich the stats cache (so skewed/bimodal numerics render as histograms with outlier "
     "annotations rather than averaged-away boxes), and <code>--dictionary infer</code> calls "
     "describegpt against a local LLM to supply friendly field labels (e.g. <i>Complaint Creation "
     "Date</i>, <i>Resolution Deadline</i>). Alongside the maps the auto-profiler fills the dashboard "
     "with box plots, frequency bars, a correlation heatmap and a created-over-time trend. Requires a "
     "local LLM; the committed HTML is reused on regen.",
     True, ["smart", "nyc_311.csv", "--smarter", "--dictionary", "infer",
            "--geojson", "nyc_neighborhoods.geojson"]),
]


def find_qsv():
    if os.environ.get("QSV_BIN"):
        return os.environ["QSV_BIN"]
    for rel in ("target/debug/qsv", "target/release/qsv"):
        cand = os.path.join(REPO, rel)
        if os.path.exists(cand):
            return cand
    found = shutil.which("qsv")
    if found:
        return found
    sys.exit("qsv binary not found: build it (cargo build --bin qsv -F all_features) or set QSV_BIN")


def viz_command_tokens(title, args):
    """The `qsv viz` command for a figure as a token list, runnable from this directory.

    Dataset paths in `args` are already relative to examples/viz; a slugged `-o <slug>.html`
    (derived from the unique title) is appended so the command writes a viewable artifact
    instead of flooding stdout. This is display text only — `gen_gallery.py` runs each figure
    into a tempfile, so no example output file is written to the tree. Args are shlex-quoted so
    each token is atomic (safe to wrap on token boundaries even if a value contained spaces)."""
    slug = re.sub(r"[^a-z0-9]+", "_", title.lower()).strip("_")
    return ["qsv", "viz", *(shlex.quote(a) for a in args), "-o", f"{slug}.html"]


def viz_command(title, args):
    """The single-line command string — used verbatim as the copy-to-clipboard source."""
    return " ".join(viz_command_tokens(title, args))


def wrap_command_lines(tokens, width=60):
    """Wrap a token list into lines, breaking BEFORE a flag (`-`/`--` token) once the current
    line reaches `width`. Keeps each flag with its value on the same line and never splits a
    token. Returns the list of line strings (joined later with a ` \\` shell continuation)."""
    lines, cur = [], ""
    for tok in tokens:
        if not cur:
            cur = tok
        elif tok.startswith("-") and len(cur) >= width:
            lines.append(cur)
            cur = tok
        else:
            cur += " " + tok
    if cur:
        lines.append(cur)
    return lines


# Continuation joiner: trailing space + backslash + newline + 2-space indent. The displayed,
# wrapped command stays a valid shell command if pasted as-is; the Copy button copies the
# single-line form (data-cmd) for the cleanest paste.
WRAP_SEP = " \\\n  "


def figcaption_html(title, desc, args):
    """A figure's `<figcaption>`: title, description, then the runnable command block with a
    Copy button. The displayed command is wrapped for readability; the button copies the
    single-line form."""
    tokens = viz_command_tokens(title, args)
    display = html_escape(WRAP_SEP.join(wrap_command_lines(tokens)))
    oneline = html_escape(" ".join(tokens), quote=True)
    return (f'<figcaption><span class="t">{title}</span>'
            f'<span class="d">{desc}</span>'
            f'<div class="cmdbox"><button class="copy" type="button" title="Copy" '
            f'aria-label="Copy command to clipboard" data-cmd="{oneline}">'
            f'{COPY_ICON_SVG}{CHECK_ICON_SVG}</button>'
            f'<pre class="cmd"><code>{display}</code></pre></div></figcaption>')


def scan_object(html, brace_start):
    """Brace-scan the balanced {...} object starting at brace_start; return (obj, end_index)."""
    assert html[brace_start] == "{", f"expected object at {brace_start}, got {html[brace_start]!r}"
    depth, in_str, esc = 0, False, False
    for i in range(brace_start, len(html)):
        c = html[i]
        if in_str:
            if esc:
                esc = False
            elif c == "\\":
                esc = True
            elif c == '"':
                in_str = False
            continue
        if c == '"':
            in_str = True
        elif c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return json.loads(html[brace_start:i + 1]), i + 1
    raise ValueError("unbalanced braces after newPlot marker")


def extract_fig_json(html):
    """The single grid-form figure object: `Plotly.newPlot(graph_div, {...})`."""
    obj, _ = scan_object(html, html.index(MARKER) + len(MARKER))
    return obj


def extract_inline_panels(html):
    """The inline-div smart dashboard form (used when a map panel forces it, or for >8 panels):
    a series of `Plotly.newPlot("qsv-viz-panel-N", {...})` calls, each a self-contained figure.
    Returns the list of panel objects, or None when the output isn't the inline form."""
    needle = 'Plotly.newPlot("qsv-viz-panel-'
    if needle not in html:
        return None
    panels, idx = [], 0
    while True:
        i = html.find(needle, idx)
        if i < 0:
            break
        obj, idx = scan_object(html, html.index("{", i))
        panels.append(obj)
    return panels


def run_html(qsv, args):
    """Run `qsv viz <args>` and return the self-contained HTML output as a string."""
    fd, out = tempfile.mkstemp(suffix=".html")
    os.close(fd)
    try:
        subprocess.run([qsv, "viz", *args, "-o", out], cwd=VIZ_DIR,
                       check=True, capture_output=True, text=True)
        with open(out, encoding="utf-8") as fh:
            return fh.read()
    finally:
        os.unlink(out)


def run_fig(qsv, args):
    html = run_html(qsv, args)
    panels = extract_inline_panels(html)
    if panels is not None:
        return {"panels": panels}          # inline multi-panel dashboard
    return {"fig": extract_fig_json(html)}  # single grid-form figure


def cdnify(html):
    """Shrink a self-contained `qsv viz smart` page (~4.5MB) to a few KB for committing as an
    iframe source: swap the inline plotly bundle for a CDN <script src>. The genuine layout,
    theme, light/dark toggle and interactivity are preserved. `qsv viz smart` itself now embeds
    only the plotly bundle (it drops the MathJax/LaTeX helper that plotly-rs ships alongside it,
    since the dashboards use no LaTeX), so the plotly bundle is the sole `<script>` to replace —
    it's at script[0] (head); the per-panel `newPlot` calls and the toggle script follow it."""
    blocks = list(re.finditer(r"<script\b[^>]*>.*?</script>", html, flags=re.S))
    if not blocks or "plotly.js v" not in blocks[0].group(0):
        raise ValueError("unexpected viz smart HTML structure (plotly bundle not at script[0])")
    bundle = blocks[0]
    return html[: bundle.start()] + PLOTLY_CDN + html[bundle.end():]


def inject_resize_reporter(html):
    """Add the postMessage height reporter just before </body> so the iframe can be auto-sized to
    the dashboard with no inner scrollbar and no trailing whitespace."""
    return html.replace("</body>", RESIZE_REPORTER_JS + "\n</body>", 1)


def grid_cols(args):
    """The --grid-cols value from a smart dashboard's args (default 2)."""
    if "--grid-cols" in args:
        return int(args[args.index("--grid-cols") + 1])
    return 2


def cleanup_sidecars():
    # `viz smart` writes a stats cache next to the CSV; `--smarter` (via its internal
    # `moarstats --advanced`) also auto-creates an `.idx` index. Don't leave either in the tree
    # (the committed datasets ship without them).
    for f in os.listdir(VIZ_DIR):
        if ".stats.csv" in f or ".stats.jsonl" in f or f.endswith(".idx"):
            os.unlink(os.path.join(VIZ_DIR, f))


def main():
    qsv = find_qsv()
    # reuse the existing scaffold verbatim: everything up to and including `<div class="grid">`,
    # minus any previous banner (re-added below so it stays a single, current copy)
    with open(GALLERY, encoding="utf-8") as fh:
        existing = fh.read()
    head = existing[: existing.index('<div class="grid">') + len('<div class="grid">')]
    head = re.sub(r"<!-- AUTO-GENERATED by examples/viz/gen_gallery\.py.*?-->\n?", "", head, flags=re.S)
    head = head.replace("<!doctype html>\n", "<!doctype html>\n" + BANNER + "\n", 1)
    # iframe styling for the embedded smart-dashboard pages. Drop any prior rule first (the head is
    # reused verbatim across runs) so the current DASH_CSS always wins.
    head = re.sub(r"\s*figure\.full iframe\.dash\{[^}]*\}", "", head)
    head = head.replace("</style>", " " + DASH_CSS + "\n</style>", 1)
    # per-figure command block styling (idempotent: drop all prior cmd rules before re-adding)
    head = re.sub(r"\s*figure (?:pre\.cmd|\.cmdbox|button\.copy)[^{]*\{[^}]*\}", "", head)
    head = head.replace("</style>", " " + CMD_CSS + "\n</style>", 1)

    figs, fig_divs, plots = [], [], []
    for idx, fig in enumerate(FIGURES):
        title, desc, full, args = fig
        gid = f"g{idx}"
        iframe_name = SMART_IFRAME.get(title)
        if iframe_name:
            # embed the genuine `qsv viz smart` output (CDN-slimmed) as a full-width iframe so the
            # real full-width overview panels, theme and map buttons render as the CLI produces them
            if iframe_name in PREGENERATED:
                # LLM-dependent (`--dictionary infer`): reuse the committed, already-cdnified HTML so
                # regen stays offline & deterministic (refresh it manually — see README commands).
                sys.stderr.write(f"[{idx}] {title}: reusing pre-generated {iframe_name}\n")
            else:
                sys.stderr.write(f"[{idx}] {title}: qsv viz {' '.join(args)} -> {iframe_name}\n")
                html = inject_resize_reporter(cdnify(run_html(qsv, args)))
                with open(os.path.join(VIZ_DIR, iframe_name), "w", encoding="utf-8") as fh:
                    fh.write(html)
            figs.append(None)  # keep FIGS index aligned with idx for the non-iframe figures
            fig_divs.append(
                f'<figure class="cell full">{figcaption_html(title, desc, args)}'
                # allow fullscreen so each dashboard's in-iframe Plotly "Fullscreen" modebar
                # button (gd.requestFullscreen()) isn't blocked by the iframe permissions policy.
                f'<iframe src="{iframe_name}" class="dash" scrolling="no" loading="lazy" '
                f'allowfullscreen allow="fullscreen" '
                f'title="{title}"></iframe></figure>'
            )
            continue
        sys.stderr.write(f"[{idx}] {title}: qsv viz {' '.join(args)}\n")
        result = run_fig(qsv, args)
        if "panels" in result:
            # inline multi-panel dashboard: store the panel list and render a nested sub-grid of
            # independent plots (one <div> + newPlot per panel) inside a single full-width cell
            panels = result["panels"]
            figs.append(panels)
            cols = grid_cols(args)
            cells = "".join(
                f'<div id="{gid}-p{k}" style="height:340px"></div>' for k in range(len(panels)))
            fig_divs.append(
                f'<figure class="cell full">{figcaption_html(title, desc, args)}'
                f'<div style="display:grid;grid-template-columns:repeat({cols},minmax(0,1fr));'
                f'gap:14px">{cells}</div></figure>'
            )
            for k in range(len(panels)):
                plots.append(
                    f'Plotly.newPlot("{gid}-p{k}", FIGS[{idx}][{k}].data, '
                    f'FIGS[{idx}][{k}].layout || {{}}, '
                    f'Object.assign({{responsive:true}}, FIGS[{idx}][{k}].config || {{}}));'
                )
        else:
            figs.append(result["fig"])
            cls = "cell full" if full else "cell"
            fig_divs.append(
                f'<figure class="{cls}">{figcaption_html(title, desc, args)}'
                f'<div id="{gid}" class="plot"></div></figure>'
            )
            plots.append(
                f'Plotly.newPlot("{gid}", FIGS[{idx}].data, FIGS[{idx}].layout || {{}}, '
                f'Object.assign({{responsive:true}}, FIGS[{idx}].config || {{}}));'
            )

    figs_json = "const FIGS = [" + ",".join(
        json.dumps(f, ensure_ascii=False, separators=(",", ":")) for f in figs) + "];"
    body = (
        head + "\n"
        + "\n".join(fig_divs) + "\n"
        + "</div>\n<script>\n"
        + figs_json + "\n"
        + "\n".join(plots) + "\n"
        + "</script>\n"
        + RESIZE_LISTENER_JS + "\n"
        + COPY_JS + "\n"
        + "</body></html>\n"
    )
    with open(GALLERY, "w", encoding="utf-8") as fh:
        fh.write(body)
    cleanup_sidecars()
    sys.stderr.write(f"wrote {GALLERY} ({len(body)} bytes, {len(figs)} figures)\n")


if __name__ == "__main__":
    main()
