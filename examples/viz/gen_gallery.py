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
import shutil
import subprocess
import sys
import tempfile

VIZ_DIR = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(VIZ_DIR))
GALLERY = os.path.join(VIZ_DIR, "gallery.html")
MARKER = "Plotly.newPlot(graph_div, "

# CDN plotly tag substituted for the ~4.6MB inline bundle in the smart-dashboard iframes.
# Keep the version in sync with the gallery head's <script src> (plotly.js v3.0.1).
PLOTLY_CDN = '<script src="https://cdn.plot.ly/plotly-3.0.1.min.js" charset="utf-8"></script>'

# Smart dashboards are embedded as iframes of the *genuine* `qsv viz smart` HTML output (so the
# full-width overview panels, themes and map buttons render exactly as the CLI produces them),
# rather than reconstructed as a lossy uniform sub-grid. Keyed by figure title -> iframe filename.
SMART_IFRAME = {
    "smart dashboard":                          "smart_sales.html",
    "smart dashboard (--smarter)":              "smart_smarter.html",
    "smart dashboard (--smarter, geospatial)":  "smart_geospatial.html",
    "smart dashboard (geographic outliers)":    "smart_geo_outliers.html",
    "smart dashboard (time-series)":            "smart_timeseries.html",
    "smart dashboard (--dictionary infer, treemap)":  "smart_dict_treemap.html",
    "smart dashboard (--dictionary infer, sunburst)": "smart_dict_sunburst.html",
}

# Iframe artifacts that depend on a live LLM (`--dictionary infer` calls describegpt against a
# local LM Studio / Ollama endpoint). Their committed HTML is REUSED as-is rather than regenerated,
# so a normal `gen_gallery.py` run stays LLM-free and deterministic. To refresh them, run the
# `qsv viz smart ... --dictionary infer` commands from README.md (with your LLM up) and re-cdnify.
PREGENERATED = {
    "smart_dict_treemap.html",
    "smart_dict_sunburst.html",
}

# CSS for the smart-dashboard iframes. `scrolling="no"` + `overflow:hidden` plus the postMessage
# auto-sizing below mean each iframe ends up exactly as tall as its dashboard — no inner scrollbar,
# no trailing whitespace. The height here is just an initial value before the first height message.
DASH_CSS = ("figure.full iframe.dash{width:100%;border:0;height:600px;display:block;"
            "border-radius:6px;overflow:hidden}")

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
    ("smart dashboard", "Auto-profiled overview: correlation heatmap + box plots + frequency bars.",
     True, ["smart", "sales_sample.csv", "--max-charts", "8"]),
    ("smart dashboard (--smarter)",
     "Same auto-profiler with `--smarter`, which runs `qsv moarstats --advanced` itself to enrich "
     "the stats cache in one step: the bimodal monthly_spend column renders as a histogram (a box "
     "plot would hide its two peaks), and the skewed account_age_days box is annotated with its "
     "skew direction and outlier share.",
     True, ["smart", "customer_spend.csv", "--smarter", "--max-charts", "8"]),
    ("smart dashboard (--smarter, geospatial)",
     "One `qsv viz smart seismic_events.csv --smarter --theme plotly_dark --grid-cols 3` command, "
     "10 auto-chosen panels — nearly every "
     "panel type at once on a synthetic catalog of Japanese earthquakes. Things the raw table hides "
     "but the dashboard makes obvious: depth_km is <b>bimodal</b> (two populations — shallow "
     "interplate quakes ~20&nbsp;km and the deep Wadati-Benioff slab ~450&nbsp;km — so --smarter "
     "draws a histogram, not a box that would average the peaks away); the points trace Japan's "
     "subduction arcs on the map; magnitude vs felt_reports is almost perfectly correlated "
     "(r=0.95); magnitude and felt_reports are right-skewed with flagged outliers; and the "
     "magnitude-over-time trend spikes during a September aftershock sequence. Coordinate columns "
     "are shown on the map only, not re-charted as distributions. Rendered with the built-in "
     "<code>plotly_dark</code> theme.",
     True, ["smart", "seismic_events.csv", "--smarter", "--theme", "plotly_dark", "--grid-cols", "3"]),
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
    ("map", "Earthquake points on token-free OpenStreetMap tiles; marker color = magnitude, size = depth.",
     False, ["map", "quakes.csv", "--lat", "lat", "--lon", "lon", "--color", "magnitude", "--size", "depth_km"]),
    ("map (density)", "DensityMapbox heatmap of the same points on a light Carto basemap.",
     False, ["map", "quakes.csv", "--lat", "lat", "--lon", "lon", "--density", "--style", "carto-positron"]),
    ("geo", "Same earthquakes on an offline natural-earth projection (no tiles, no token); marker "
     "color = magnitude. viz smart auto-uses this projection for global-extent coordinates.",
     False, ["geo", "quakes.csv", "--lat", "lat", "--lon", "lon", "--color", "magnitude",
             "--projection", "natural-earth"]),
    ("smart dashboard (time-series)",
     "Auto dashboard for stock_prices: a time-series trend panel (the first numeric column over the "
     "date) leads, alongside box-plot summaries of the OHLC columns.",
     True, ["smart", "stock_prices.csv", "--max-charts", "8"]),
    ("smart dashboard (--dictionary infer, treemap)",
     "Auto dashboard for customer_spend with a describegpt-inferred Data Dictionary "
     "(--dictionary infer) guiding panel selection & field labels. Two categorical dimensions "
     "(plan, region) form a shallow part-to-whole hierarchy, auto-rendered as a TREEMAP "
     "(area = size). Requires a local LLM; the committed HTML is reused on regen.",
     True, ["smart", "customer_spend.csv", "--dictionary", "infer"]),
    ("smart dashboard (--dictionary infer, sunburst)",
     "Auto dashboard for sales_sample with a describegpt-inferred Data Dictionary. Three "
     "categorical dimensions form a deeper hierarchy, auto-rendered as a SUNBURST (concentric "
     "rings emphasize parent-child structure). Requires a local LLM; the committed HTML is "
     "reused on regen.",
     True, ["smart", "sales_sample.csv", "--dictionary", "infer"]),
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
                f'<figure class="cell full"><figcaption><span class="t">{title}</span>'
                f'<span class="d">{desc}</span></figcaption>'
                f'<iframe src="{iframe_name}" class="dash" scrolling="no" loading="lazy" '
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
                f'<figure class="cell full"><figcaption><span class="t">{title}</span>'
                f'<span class="d">{desc}</span></figcaption>'
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
                f'<figure class="{cls}"><figcaption><span class="t">{title}</span>'
                f'<span class="d">{desc}</span></figcaption><div id="{gid}" class="plot"></div>'
                f'</figure>'
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
        + "</body></html>\n"
    )
    with open(GALLERY, "w", encoding="utf-8") as fh:
        fh.write(body)
    cleanup_sidecars()
    sys.stderr.write(f"wrote {GALLERY} ({len(body)} bytes, {len(figs)} figures)\n")


if __name__ == "__main__":
    main()
