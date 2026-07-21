#!/usr/bin/env python3
"""Regenerate examples/viz/gallery.html from the current qsv binary.

`gallery.html` is a lightweight, CDN-plotly page rendering every `qsv viz`
chart type from the sample datasets in this directory. It is a checked-in
*artifact*; this script is the source of truth for how it's produced.

For each figure it runs the documented `qsv viz` command, extracts the figure
JSON object that plotly-rs emits (`Plotly.newPlot(graph_div, {...})`) from the
output, and reassembles them into one page that loads plotly from the CDN (so the
committed file stays small). The static scaffold (head / style / header) is reused
verbatim from the existing gallery, so re-running this only changes figure content
and order.

Every `qsv viz` run here sets QSV_VIZ_CDN=1, so viz itself emits the CDN
`<script src>` tag and the smart-dashboard iframes are committable as-is.

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

# Smart dashboards are embedded as iframes of the *genuine* `qsv viz smart` HTML output (so the
# full-width overview panels, themes and map buttons render exactly as the CLI produces them),
# rather than reconstructed as a lossy uniform sub-grid. Keyed by figure title -> iframe filename.
SMART_IFRAME = {
    "smart dashboard":                          "smart_sales.html",
    "smart dashboard (KPI gauges & target delta)": "smart_sales_kpi.html",
    "smart dashboard (--smarter)":              "smart_smarter.html",
    "smart dashboard (--smarter, geospatial)":  "smart_geospatial.html",
    "smart dashboard (geographic outliers)":    "smart_geo_outliers.html",
    "smart dashboard (time-series)":            "smart_timeseries.html",
    "smart dashboard (per-US-state choropleth)":      "smart_us_choropleth.html",
    "smart dashboard (--dictionary infer, treemap)":  "smart_dict_treemap.html",
    "smart dashboard (--dictionary infer, sunburst)": "smart_dict_sunburst.html",
    "smart dashboard (--dictionary infer, world choropleth)": "smart_world_choropleth.html",
    "smart dashboard (--smarter, committed --dictionary, NYC 311 metro choropleth)": "smart_nyc311.html",
    "smart dashboard (--smarter, curated --dictionary, region-code zip choropleth)": "smart_allegheny_dogs.html",
    "smart dashboard (animated geo, world events)":   "smart_world_events.html",
    "smart dashboard (Gapminder bubble, regions growth)": "smart_regions_growth.html",
    "smart dashboard (--smarter, Gini/Lorenz inequality + log-skew boxes)": "smart_cms_medicare.html",
}

# Iframe artifacts that depend on a live LLM (`--dictionary infer` calls describegpt against a
# local LM Studio / Ollama endpoint). Their committed HTML is REUSED as-is rather than regenerated,
# so a normal `gen_gallery.py` run stays LLM-free and deterministic. To refresh them in-place, run
# with QSV_VIZ_REGEN_LLM=1 and your local LLM up (LM Studio / Ollama) — main() then treats this set
# as empty so these `--dictionary infer` figures are regenerated and re-cdnified like the others.
# NOTE: because they are reused as-is, these figures intentionally LAG new smart-dashboard features
# until an LLM-backed regen — e.g. they do not yet show the KPI/Completeness overview row that the
# deterministic dashboards above carry. This is deferred on purpose: they will be regenerated in one
# pass once describegpt emits the gauge_range/target dictionary hints, so the KPI row lands with its
# gauges/deltas rather than as bare number tiles now.
PREGENERATED = {
    "smart_dict_treemap.html",
    "smart_dict_sunburst.html",
    "smart_world_choropleth.html",
    # smart_geospatial.html (Japan): --bivariate now implies --dictionary infer here (no
    # explicit --dictionary is passed), so it needs a live LLM like the other infer figures.
    "smart_geospatial.html",
    # smart_nyc311.html is NOT here: it now uses a committed curated dictionary
    # (nyc311_dict.schema.json), so --bivariate doesn't imply --dictionary infer (already
    # set) and it still regenerates deterministically without an LLM.
}

# CSS for the smart-dashboard iframes. `scrolling="no"` + `overflow:hidden` plus the postMessage
# auto-sizing below mean each iframe ends up exactly as tall as its dashboard — no inner scrollbar,
# no trailing whitespace. The height here is just an initial value before the first height message.
DASH_CSS = ("figure.full iframe.dash{width:100%;border:0;height:600px;display:block;"
            "border-radius:6px;overflow:hidden}")

# The final gallery entry is a clickable SCREENSHOT (not a plotly figure): a scaled preview image
# that opens the full, standalone `pitt311data.html` dashboard in a new popup window (that page is
# ~19MB — too large to embed as an iframe like the smart_*.html dashboards). Portrait image, so cap
# its rendered height and center it; the border + hover lift mirror the surrounding figure chrome.
SHOT_CSS = ("figure a.shot{display:block;text-align:center;text-decoration:none;cursor:pointer}"
            "figure a.shot img{max-width:100%;max-height:900px;height:auto;border:1px solid #e6e9f0;"
            "border-radius:6px;box-shadow:0 1px 3px rgba(0,0,0,.06)}"
            "figure a.shot:hover img{border-color:#c3ccdb;box-shadow:0 2px 8px rgba(0,0,0,.12)}")

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
           # pre-wrap (not plain `pre`) so a long single-line command wraps before the absolutely
           # positioned Copy button instead of scrolling underneath it; the 40px right padding above
           # reserves the button gutter, and overflow-wrap:anywhere lets a stray over-wide token
           # break on very narrow (mobile) widths rather than clip.
           "color:#2A3F5F;white-space:pre-wrap;overflow-wrap:anywhere}"
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

# Navigation + responsive layout, injected once (between /*qsv-nav*/.../*end-qsv-nav*/ markers so a
# regen strips and re-adds it cleanly). Three jobs: (1) a sticky, collapsed-by-default "Jump to a
# chart" Table of Contents (the page is ~35 figures / very tall, so a jump list is the only sane way
# to navigate) — each link targets a per-figure id; scroll-margin-top keeps the sticky bar from
# covering a jumped-to figure's title, and scroll-behavior:smooth animates the jump. (2) a
# max-width:760px breakpoint that collapses the 2-col grid (set in the verbatim head) to a single
# column so charts and command blocks stop being crushed/clipped on phones. (3) the TOC's own chrome.
NAV_CSS = (
    "html{scroll-behavior:smooth}"
    "figure{scroll-margin-top:64px}"
    "details.toc{position:sticky;top:0;z-index:30;max-width:1200px;margin:0 auto 16px;background:#fff;"
    "border:1px solid #e6e9f0;border-radius:10px;box-shadow:0 1px 3px rgba(0,0,0,.05)}"
    "details.toc>summary{cursor:pointer;list-style:none;user-select:none;font-weight:600;font-size:13px;"
    "color:#2A3F5F;padding:10px 14px}"
    "details.toc>summary::-webkit-details-marker{display:none}"
    'details.toc>summary::after{content:"\\25be";float:right;color:#6b7a94;font-weight:400}'
    'details.toc[open]>summary::after{content:"\\25b4"}'
    "details.toc .toc-count{color:#8a97ad;font-weight:400;font-size:12px;margin-left:6px}"
    "details.toc .toc-links{display:grid;grid-template-columns:repeat(auto-fill,minmax(210px,1fr));"
    "gap:1px 16px;padding:6px 14px 12px;max-height:54vh;overflow:auto;border-top:1px solid #eef1f6}"
    "details.toc .toc-links a{display:block;font-size:12.5px;color:#3056d3;text-decoration:none;"
    "padding:3px 6px;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;border-radius:4px}"
    "details.toc .toc-links a:hover{text-decoration:underline;background:#f3f5f9}"
    "@media (max-width:760px){body{padding:14px}"
    ".grid{grid-template-columns:1fr;gap:16px}"
    "figure.full .plot{height:520px}.plot{height:320px}"
    "details.toc .toc-links{grid-template-columns:1fr;max-height:60vh}}")

# Collapse the sticky "Jump to a chart" panel as soon as one of its links is clicked, so the open
# Table of Contents doesn't sit over the figure the reader just jumped to. The collapse happens
# before the browser's default in-page hash navigation, and figure{scroll-margin-top} keeps the
# target clear of the now-collapsed (thin) sticky bar.
TOC_JS = (
    "<script>document.addEventListener(\"click\",function(e){"
    "var a=e.target.closest&&e.target.closest(\"details.toc .toc-links a\");if(!a)return;"
    "var d=a.closest(\"details.toc\");if(d)d.open=false;});</script>")

# Injected once into the gallery: keeps a jumped-to figure in view while the page height is still
# settling. The dashboard iframes are lazy-loaded and grow from 600px to their real height on load,
# so iframes ABOVE a jump target load mid-scroll and push the target out of view — the reason
# "Jump to a chart" felt unreliable. On a TOC click / hashchange we arm the target for a short window
# and re-align it (instantly, respecting figure{scroll-margin-top}) on every dashboard resize report
# and on a coarse timer, so drift from iframes growing above it is corrected. Any manual scroll input
# ends the window immediately so this never fights the reader.
JUMP_JS = (
    "<script>(function(){var target=null,until=0,iv=null,moved=false;"
    "function stop(){if(iv){clearInterval(iv);iv=null;}}"
    # instant (not smooth): the page sets scroll-behavior:smooth, but a correcting re-align must be
    # instant. A smooth scrollIntoView restarted every tick as iframes above grow would perpetually
    # re-ease and never actually reach the target.
    "function snap(){if(target&&Date.now()<until){target.scrollIntoView("
    "{block:\"start\",behavior:\"instant\"});}else{stop();}}"
    "function arm(el){if(!el)return;target=el;until=Date.now()+2500;snap();"
    "if(!iv)iv=setInterval(snap,120);}"
    "document.addEventListener(\"click\",function(e){"
    "var a=e.target.closest&&e.target.closest(\"details.toc .toc-links a\");if(!a)return;"
    "var href=a.getAttribute(\"href\");if(!href||href.charAt(0)!==\"#\")return;"
    "arm(document.getElementById(href.slice(1)));});"
    "addEventListener(\"hashchange\",function(){"
    "if(location.hash.length>1)arm(document.getElementById(location.hash.slice(1)));});"
    # dashboard iframes report their height (qsvVizHeight -> re-snap) and forward the reader's own
    # scroll input (qsvUserScroll -> the reader took over, cancel and stop re-aligning).
    "addEventListener(\"message\",function(e){var d=e.data;if(!d)return;"
    "if(typeof d.qsvVizHeight===\"number\")snap();"
    "else if(d.qsvUserScroll){until=0;moved=true;stop();}});"
    # scrolls over the non-iframe margins land on the parent window directly.
    "[\"wheel\",\"touchstart\",\"keydown\"].forEach(function(t){"
    "addEventListener(t,function(){until=0;moved=true;},{passive:true});});"
    # also arm on initial load with an existing fragment (a shared/bookmarked deep link like
    # gallery.html#fig-...), so lazy iframes settling after the browser's initial hash scroll don't
    # push the target out of view. Re-arm on load once the many heavy iframes have settled — but not
    # if the reader already scrolled away (moved), so a late load never yanks them back. Explicit TOC
    # clicks / hashchange still arm unconditionally.
    "function armFromHash(){if(!moved&&location.hash.length>1)"
    "arm(document.getElementById(location.hash.slice(1)));}"
    "armFromHash();addEventListener(\"load\",armFromHash);})();</script>")

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
    "setTimeout(r,200);setTimeout(r,800);"
    # a deep-linked dashboard fills the viewport, so the reader's scroll input lands INSIDE this
    # iframe, not the parent gallery window. Forward it so the parent's jump stabilizer can tell the
    # reader has taken over and stop re-aligning / cancel its pending on-load re-arm (see JUMP_JS).
    # capture phase: the gl3d scroll-fix installs a capture-phase wheel handler on the plot div that
    # stopPropagation()s (so a 3D panel scrolls the page instead of being eaten); a bubble-phase
    # listener here would never see that wheel. Capturing at the window fires before the plot div, so
    # the reader's scroll over a 3D panel still cancels the parent's jump stabilizer.
    "var us=function(){parent.postMessage({qsvUserScroll:1},\"*\");};"
    "[\"wheel\",\"touchstart\",\"keydown\"].forEach(function(t){"
    "addEventListener(t,us,{capture:true,passive:true});});})();</script>")

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

# The standalone (non-smart) figures are reconstructed in this page's own <script> via
# Plotly.newPlot, so — unlike the genuine `qsv viz` HTML output and the smart-dashboard iframes —
# they don't carry qsv's injected fullscreen modebar button. Define the same button here and add it
# to each figure's config (`modeBarButtonsToAdd`) so the gallery's standalone charts match the CLI.
# Plain JS (no <script> wrapper): emitted inside the page script, before the newPlot calls. The
# click handler toggles the native Fullscreen API on the graph div plotly passes in; a single
# document-level `fullscreenchange` listener then resizes the plot so it actually fills the
# fullscreen viewport on enter (and restores on exit) — mirroring the CLI handler's resize.
#
# Maps mirror the CLI's fit (see FULLSCREEN_SCRIPT in src/cmd/viz.rs): MapLibre bakes an
# absolute zoom for a fixed assumed px size — standalone `viz map` HTML frames against 1000x600
# (the only map figures reconstructed on this page are standalone; the map-bearing smart dashboards
# are iframes that carry their own CLI prelude). Since MapLibre zoom is logarithmic, the optimal zoom
# for any real container size is bakedZoom + log2(min(curW/1000, curH/600)) keeping the baked center
# (qsvFitTarget); curW/curH are domain-scaled (subplot px). The fit is applied by aiming the GL map
# camera directly (qsvApplyCamera -> _subplot.map.jumpTo), NEVER Plotly.relayout/react — those throw
# on a MapLibre choroplethmap in our pinned plotly fork and blank the layer. Applied on initial
# display and on every fullscreenchange, recomputing from the once-captured baked reference so
# toggles don't drift.
FS_BUTTON_JS = (
    'var qsvFsBtn={name:"qsv-fullscreen",title:"Toggle fullscreen",'
    'icon:{width:512,height:512,path:"M512 512v-208l-80 80-96-96-48 48 96 96-80 80z '
    'M512 0h-208l80 80-96 96 48 48 96-96 80 80z M0 512h208l-80-80 96-96-48-48-96 96-80-80z '
    'M0 0v208l80-80 96 96 48-48-96-96 80-80z"},'
    'click:function(gd){try{var p=document.fullscreenElement?document.exitFullscreen()'
    ':gd.requestFullscreen();if(p&&p.catch)p.catch(function(){});}catch(e){}}};'
    # "Toggle legend" button — mirrors FULLSCREEN_SCRIPT in src/cmd/viz.rs. Reads the EFFECTIVE
    # state from _fullLayout (plotly defaults showlegend:true for multi-trace charts that never set
    # it) so the first click hides a default-visible legend. No-op on maps: Plotly.relayout throws on
    # a MapLibre choroplethmap in our pinned fork, and a choropleth legend is a colorbar (not
    # governed by showlegend). Uses qsvMapKeys (defined below; function decls hoist).
    'var qsvLegendBtn={name:"qsv-legend",title:"Toggle legend",'
    'icon:{width:512,height:512,path:"M0 96h96v64H0z M160 112h352v32H160z M0 224h96v64H0z '
    'M160 240h352v32H160z M0 352h96v64H0z M160 368h352v32H160z"},'
    'click:function(gd){try{if(qsvMapKeys(gd).length)return;var fl=gd._fullLayout||gd.layout||{};'
    'var p=Plotly.relayout(gd,{showlegend:!fl.showlegend});if(p&&p.catch)p.catch(function(){});}'
    'catch(e){}}};'
    'function qsvMapKeys(gd){var lay=(gd&&gd.layout)||{};return Object.keys(lay).filter('
    'function(k){return /^map\\d*$/.test(k)&&lay[k]&&typeof lay[k].zoom==="number";});}'
    'function qsvCaptureBaked(gd){if(gd.__qsvBaked)return;gd.__qsvBaked={};var lay=gd.layout||{};'
    'qsvMapKeys(gd).forEach(function(k){gd.__qsvBaked[k]={z:lay[k].zoom,c:lay[k].center};});}'
    'function qsvPlotPx(gd){var fl=gd._fullLayout||{};'
    'return{w:fl.width||gd.clientWidth||0,h:fl.height||gd.clientHeight||0};}'
    'function qsvFitTarget(gd,k,plotW,plotH){var b=gd.__qsvBaked&&gd.__qsvBaked[k];'
    'if(!b||typeof b.z!=="number"||!(plotW>0)||!(plotH>0))return null;'
    'var aw=1000,ah=600,lay=gd.layout||{};var dom=(lay[k]&&lay[k].domain)||{};'
    'var dx=(dom.x&&dom.x.length===2)?(dom.x[1]-dom.x[0]):1;'
    'var dy=(dom.y&&dom.y.length===2)?(dom.y[1]-dom.y[0]):1;'
    'var ratio=Math.min((dx*plotW)/aw,(dy*plotH)/ah);if(!isFinite(ratio)||ratio<=0)return null;'
    'return{zoom:b.z+Math.log2(ratio),center:b.c};}'
    # Aim the GL map camera directly (gd._fullLayout[k]._subplot.map.jumpTo) — NEVER Plotly.relayout/
    # react, which throw on a MapLibre choroplethmap in our pinned plotly fork ("setData of
    # undefined") and blank the layer. Mirrors applyFitCamera in src/cmd/viz.rs.
    'function qsvApplyCamera(gd,plotW,plotH){if(!gd||!gd.__qsvBaked)return;'
    'var fl=gd._fullLayout||{},lay=gd.layout||{};Object.keys(gd.__qsvBaked).forEach(function(k){'
    'var t=qsvFitTarget(gd,k,plotW,plotH);if(!t)return;'
    'var sp=fl[k]&&fl[k]._subplot,m=sp&&sp.map;if(!m||typeof m.jumpTo!=="function")return;'
    'var dest={zoom:t.zoom};'
    'if(t.center&&typeof t.center.lon==="number"&&typeof t.center.lat==="number")'
    'dest.center=[t.center.lon,t.center.lat];try{m.jumpTo(dest);}catch(e){}'
    'if(lay[k]){lay[k].zoom=t.zoom;if(t.center)lay[k].center=t.center;}});}'
    # capture baked once, wait (bounded) for each map subplot's _subplot.map to attach, then aim.
    'function qsvFitNow(gd,tries){if(tries===undefined)tries=20;qsvCaptureBaked(gd);'
    'var fl=gd._fullLayout||{};var ready=Object.keys(gd.__qsvBaked||{}).every(function(k){'
    'var sp=fl[k]&&fl[k]._subplot;return sp&&sp.map;});'
    'if(!ready&&tries>0){setTimeout(function(){qsvFitNow(gd,tries-1);},100);return;}'
    'var px=qsvPlotPx(gd);qsvApplyCamera(gd,px.w,px.h);}'
    # scrollZoom control (mirrors setScrollZoom/applyScrollZoom in src/cmd/viz.rs, fix #4150): plotly
    # turns wheel-zoom ON by default for geo/map/gl3d subplots, so a wheel/two-finger scroll over a
    # map, choropleth or 3D figure zooms the subplot and swallows the PAGE scroll. Disable it inline
    # (so scrolling scrolls the page); re-enable in fullscreen (no page to scroll). geo/gl3d follow
    # plotly's live read of gd._context.scrollZoom; a MapLibre `map` subplot ignores the render
    # config at the GL-handler level, so toggle its wheel-zoom handler DIRECTLY once the map
    # attaches. Drag-pan (and 3D drag-rotate) is left untouched.
    'function qsvSetScrollZoom(gd,on){try{if(gd._context)gd._context.scrollZoom=on;}catch(e){}'
    'var fl=gd._fullLayout||{};qsvMapKeys(gd).forEach(function(k){'
    'var sp=fl[k]&&fl[k]._subplot,m=sp&&sp.map;'
    'if(m&&m.scrollZoom&&typeof m.scrollZoom.enable==="function"){'
    'try{if(on)m.scrollZoom.enable();else m.scrollZoom.disable();}catch(e){}}});}'
    # wait (bounded) for each MapLibre subplot's _subplot.map to attach (geo/gl3d/non-map figures are
    # ready immediately since qsvMapKeys is empty), then assert the scrollZoom state for the current
    # fullscreen mode. Published as a global so any re-render path can re-assert it.
    'function qsvApplyScrollZoom(gd,tries){if(tries===undefined)tries=20;var fl=gd._fullLayout||{};'
    'var ready=qsvMapKeys(gd).every(function(k){var sp=fl[k]&&fl[k]._subplot;return sp&&sp.map;});'
    'if(!ready&&tries>0){setTimeout(function(){qsvApplyScrollZoom(gd,tries-1);},100);return;}'
    'qsvSetScrollZoom(gd,document.fullscreenElement===gd);}'
    'window.__qsvRefitScrollZoom=qsvApplyScrollZoom;'
    # gl3d (3D scene) panels need a DIFFERENT fix than geo/map: plotly's WebGL canvas swallows the
    # wheel (preventDefault) on EVERY wheel event — even created with scrollZoom:false, which
    # suppresses the zoom but NOT the preventDefault — so a scroll over a 3D chart neither zooms NOR
    # scrolls the page. The robust fix is a capture-phase wheel interceptor: inline we stopPropagation
    # so the canvas handler never runs (never preventDefaults) and the wheel scrolls the PAGE; in
    # fullscreen we let it through. The listener sits on gd so it survives any re-render. Drag-rotate
    # uses pointer events, so it is untouched.
    'function qsvSceneKeys(gd){var fl=gd._fullLayout||{};'
    'return Object.keys(fl).filter(function(k){return /^scene\\d*$/.test(k);});}'
    'function qsvInstallGl3dFix(gd){if(gd.__qsvGl3dFix||!qsvSceneKeys(gd).length)return;'
    'gd.__qsvGl3dFix=true;gd.addEventListener("wheel",function(e){'
    'if(document.fullscreenElement!==gd)e.stopPropagation();},{capture:true,passive:true});}'
    'var qsvTries=0;function qsvInitFit(){if(typeof Plotly==="undefined"){'
    'if(qsvTries++<100)setTimeout(qsvInitFit,50);return;}var pending=false;'
    'document.querySelectorAll(".js-plotly-plot").forEach(function(gd){if(gd.__qsvInitFit)return;'
    'if(gd.data){gd.__qsvInitFit=true;qsvFitNow(gd);qsvApplyScrollZoom(gd);qsvInstallGl3dFix(gd);'
    # Core/Full extent buttons bake an assumed-px zoom; adopt the clicked extent as the new fit
    # reference and re-aim the camera for the current size (mirrors the CLI handler in viz.rs).
    'if(gd.on)gd.on("plotly_buttonclicked",function(ed){try{'
    'var args=ed&&ed.button&&ed.button.args&&ed.button.args[0];if(!args||!gd.__qsvBaked)return;'
    'var hit=false;Object.keys(gd.__qsvBaked).forEach(function(k){var z=args[k+".zoom"];'
    'if(typeof z!=="number")return;var c=args[k+".center"];'
    'gd.__qsvBaked[k]={z:z,c:(c!==undefined?c:gd.__qsvBaked[k].c)};hit=true;});'
    'if(hit)setTimeout(function(){qsvFitNow(gd);},0);}catch(e){}});'
    '}else pending=true;});'
    'if(pending&&qsvTries++<100)setTimeout(qsvInitFit,50);}setTimeout(qsvInitFit,0);'
    # Plotly.Plots.resize is async; fit only AFTER it resolves (mirrors viz.rs) so qsvFitNow reads
    # the post-resize dims rather than stale pre-change ones.
    'function qsvResizeThenFit(gd){var rp;try{rp=Plotly.Plots.resize(gd);}catch(e){}'
    # re-assert scrollZoom for the current fullscreen mode after the async resize resolves (mirrors
    # the fullscreenchange handler in src/cmd/viz.rs #4150): enable wheel-zoom entering fullscreen,
    # disable on exit — read post-resize so a MapLibre subplot's GL handle is attached.
    'var qsvDone=function(){qsvFitNow(gd);qsvApplyScrollZoom(gd);};'
    'if(rp&&rp.then)rp.then(qsvDone).catch(qsvDone);'
    'else qsvDone();}'
    'document.addEventListener("fullscreenchange",function(){try{'
    'var el=document.fullscreenElement;'
    'if(el&&el.classList&&el.classList.contains("js-plotly-plot")){qsvResizeThenFit(el);}'
    'else{document.querySelectorAll(".js-plotly-plot").forEach(function(gd){'
    'try{qsvResizeThenFit(gd);}catch(e){}});}'
    '}catch(e){}});')

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
     "One command, 13 auto-chosen panels — nearly every "
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
     "magnitude-over-time trend spikes during a September aftershock sequence. "
     "<code>--bivariate</code> adds an <b>NMI association heatmap</b> spanning every column type, "
     "not just the continuous-numeric ones the Pearson heatmap covers — it surfaces "
     "depth_km and felt_reports as strongly associated with occurrence_date (NMI=0.93 and 0.89), "
     "a temporal-clustering signal (the same September aftershock sequence) a Pearson matrix "
     "restricted to numeric pairs alone cannot express against a date column. Coordinate columns "
     "are shown on the map only, not re-charted as distributions. Rendered with the built-in "
     "<code>plotly_dark</code> theme.",
     True, ["smart", "seismic_events.csv", "--smarter", "--bivariate", "--theme", "plotly_dark",
            "--grid-cols", "3", "--geojson", "japan_prefectures.geojson"]),
    ("smart dashboard (geographic outliers)",
     "Delivery stops clustered in metro Denver with four "
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
    ("smart dashboard (KPI gauges & target delta)",
     "The KPI overview row driven by a hand-authored `--dictionary` "
     "(sales_kpi_dict.schema.json). Two optional `x-qsv` hints turn plain measure tiles into "
     "richer KPIs: a `gauge_range` of [0,1] draws Discount % and Profit Margin % as GAUGES on "
     "their canonical ratio scale, and a `target` of 0.25 on Profit Margin adds a \"vs target\" "
     "DELTA (the mean is 0.21, so a red -0.042 below the goal). qsv keeps a gauge only when the "
     "data lies within its range, so a mis-scaled range can't mislead; `gauge_range` is what "
     "`--dictionary infer` emits for canonical-scale ratios, while `target` is a business goal you "
     "hand-author (never LLM-inferred). Overall dataset completeness rides quietly in the header "
     "table, not as a tile.",
     True, ["smart", "sales_sample.csv", "--dictionary", "sales_kpi_dict.schema.json"]),
    ("smart dashboard (--smarter)",
     "Same auto-profiler with `--smarter`, which runs `qsv moarstats --advanced` itself to enrich "
     "the stats cache in one step: the bimodal monthly_spend column renders as a histogram (a box "
     "plot would hide its two peaks), and the skewed account_age_days box is annotated with its "
     "skew direction and outlier share.",
     True, ["smart", "customer_spend.csv", "--smarter", "--max-charts", "8"]),
    ("smart dashboard (--smarter, Gini/Lorenz inequality + log-skew boxes)",
     "<b>Medicare payments to individual practitioners</b> (CMS &mdash; a 30k sample), with "
     "<code>medical_payment</code> and <code>total_patients</code> per provider. These are additive "
     "amounts across <i>comparable</i> units, so <code>--smarter</code> recognizes them as inequality "
     "measures and adds a <b>Lorenz curve</b> for each (Gini 0.65 / 0.61) &mdash; the further the "
     "curve bows below the diagonal, the more concentrated the payments. The distribution boxes are "
     "<b>skew-aware</b>: a heavily right-skewed money column would squash a linear box into a sliver "
     "against its largest values, so <code>viz smart</code> draws it on a <b>log axis</b> instead, "
     "keeping the median and quartiles legible.",
     True, ["smart", "cms_medicare_providers.csv", "--smarter"]),
    ("bar", "Revenue by region (aggregated sum).",
     False, ["bar", "sales_sample.csv", "--x", "region", "--y", "revenue", "--agg", "sum"]),
    ("bar (animated slider)",
     "Revenue by product category, <b>animated over an ordinal column</b> with "
     "<code>--slider</code>: each distinct satisfaction rating (1&ndash;5) becomes an animation "
     "frame, with a <b>&#9654; Play</b>/<b>&#9208; Pause</b> button and a scrub slider to step "
     "through them (Gapminder-style). Axis ranges are <b>pinned across frames</b> so the bars stay "
     "comparable frame to frame instead of rescaling. <code>--slider</code> also works on line and "
     "scatter, may be split into animated traces with <code>--series</code>, and can accumulate "
     "with <code>--slider-cumulative</code>.",
     False, ["bar", "sales_sample.csv", "--x", "product_category", "--y", "revenue", "--agg", "sum",
             "--slider", "satisfaction"]),
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
    ("violin", "Revenue distribution per region — a KDE density silhouette around an inner "
     "quartile box + mean line, revealing shape (modes, shoulders) a box hides. viz smart "
     "auto-picks this for columns in the bimodality ambiguity band (--violin auto).",
     False, ["violin", "sales_sample.csv", "--y", "revenue", "--x", "region"]),
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
    ("radar", "Multi-axis brand comparison (per-axis mean per series).",
     False, ["radar", "product_ratings.csv", "--cols", "battery,camera,performance,display,value,design",
             "--series", "brand"]),
    ("sankey", "Web session funnel (duplicate edges aggregated).",
     True, ["sankey", "web_flows.csv", "--source", "source", "--target", "target", "--value", "sessions"]),
    ("treemap", "Part-to-whole spend by plan then region, sized by summed monthly_spend. Rounded "
     "tiles + white separators come from the treemap-specific marker; non-numeric/negative measure "
     "cells are rejected so proportions can't silently misstate.",
     True, ["treemap", "customer_spend.csv", "--cols", "plan,region", "--value", "monthly_spend",
             "--agg", "sum"]),
    ("sunburst", "Three-level hierarchy (region -> product_category -> payment_method) as concentric "
     "rings, sized by row count; inner rings are parents, outer rings their children. Opens at two "
     "rings (maxdepth) so labels stay legible instead of crowding a ~100-sector outer ring; click a "
     "sector to drill in and the deeper ring's labels grow back. Hover always shows value + percent.",
     True, ["sunburst", "sales_sample.csv", "--cols", "region,product_category,payment_method"]),
    ("icicle", "Same three-level hierarchy (region -> product_category -> payment_method) as a rectangular "
     "icicle: parents on the left, children fanning right, each rectangle sized by row count. The flat "
     "left-to-right layout keeps deep labels readable where a sunburst's outer ring would crowd; click a "
     "rectangle to zoom into that branch. Hover shows label + value + percent of parent.",
     True, ["icicle", "sales_sample.csv", "--cols", "region,product_category,payment_method"]),
    ("splom", "Scatter-plot matrix of four numeric columns (units_sold, revenue, discount_pct, "
     "profit_margin_pct): every pairwise scatter in an N x N grid with shared axes, so correlation "
     "structure is legible at a glance. viz smart auto-adds this panel when 3+ correlated numeric "
     "columns exist, capped at 6 dims selected by correlation participation.",
     True, ["splom", "sales_sample.csv", "--cols", "units_sold,revenue,discount_pct,profit_margin_pct"]),
    ("parcats", "Parallel-categories flow over three categorical columns (region -> product_category "
     "-> payment_method): each ribbon is a category combination, sized by how many rows share it, so "
     "co-occurrence between the dimensions is visible without implying a part-to-whole nesting. "
     "Ribbons are <b>colored by their first-axis category</b> and bundled (like a Sankey), and each "
     "axis opens ordered by frequency with a <b>&#8645; category order</b> toggle that flips every "
     "axis between frequency and alphabetical order. viz "
     "smart auto-adds this panel for 3-4 associated many-to-many categoricals (and suppresses the "
     "hierarchy on the same columns); genuine rollup trees still auto-select a treemap/sunburst.",
     True, ["parcats", "sales_sample.csv", "--cols", "region,product_category,payment_method"]),
    ("map", "Earthquake points on token-free OpenStreetMap tiles; marker color = magnitude, size = depth.",
     False, ["map", "quakes.csv", "--lat", "lat", "--lon", "lon", "--color", "magnitude", "--size", "depth_km"]),
    ("map (density)", "DensityMap heatmap of the same points on a light Carto basemap, "
     "<b>weighted by magnitude</b> (via <code>--color</code>) so stronger quakes glow hotter "
     "&mdash; hovering a point shows its magnitude, not just the coordinates.",
     False, ["map", "quakes.csv", "--lat", "lat", "--lon", "lon", "--density", "--color", "magnitude",
             "--style", "carto-positron"]),
    ("geo", "Same earthquakes on an offline natural-earth projection (no tiles, no token); marker "
     "color = magnitude. viz smart auto-uses this projection for global-extent coordinates.",
     False, ["geo", "quakes.csv", "--lat", "lat", "--lon", "lon", "--color", "magnitude",
             "--projection", "natural-earth"]),
    ("geo (animated slider)",
     "An <b>animated point map</b>: <code>--slider</code> steps through the region column, "
     "revealing the world's seismicity <b>continent by continent</b>, each region in its own "
     "color via <code>--series</code>. With <code>--slider-cumulative</code> the points "
     "<b>accumulate</b> as the animation plays (Play/Pause + a scrub slider). Unlike the MapLibre "
     "tile map, <code>scattergeo</code> animates natively, so <code>--slider</code> is supported "
     "on <code>viz geo</code> &mdash; use it instead of <code>viz map</code> for animated point "
     "maps.",
     False, ["geo", "quakes.csv", "--lat", "lat", "--lon", "lon", "--slider", "region",
             "--series", "region", "--slider-cumulative", "--projection", "natural-earth"]),
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
     "date) leads; the strongest-correlated pair drill-down (open vs close) is shown as a <b>static "
     "scatter</b> — that relationship is a near-perfect line whose 2-D shape doesn't evolve, so "
     "the judicious animation gate withholds the (uninformative) animated version — alongside "
     "box-plot summaries of the OHLC columns.",
     True, ["smart", "stock_prices.csv", "--max-charts", "8"]),
    ("smart dashboard (per-US-state choropleth)",
     "`viz smart` reverse-geocodes each point; because every city "
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
     "<b>1,179 cities</b> with population "
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
    ("smart dashboard (--smarter, committed --dictionary, NYC 311 metro choropleth)",
     "A <b>10,000-row</b> sample of NYC 311 service requests (2010–2020) profiled into auto-chosen "
     "panels on a real, wide municipal dataset. The "
     "headline panel is the <b>metro choropleth</b>: each request's lat/lon is binned into NYC's "
     "<b>188 neighborhood</b> polygons (no geocoding — 110 of them receive requests, and 10 of the "
     "7,464 located points fall too far from any polygon to snap), and because the matched regions "
     "span a city-scale extent, <code>viz smart</code> draws the filled "
     "regions on an interactive <b>MapLibre tile basemap</b> (token-free carto tiles, fine "
     "street/coastline detail) instead of the coarse projection basemap it uses for "
     "country/continental choropleths. The leading <b>point map</b> flags bad-geocode "
     "<b>outliers</b> (here, <i>9 in Pennsylvania</i>): a nice illustration of the hover's value — "
     "for a stray PA point, the tooltip shows the record's own <i>Incident City: NEW YORK</i> "
     "right above its reverse-geocoded <i>Pennsylvania, United States</i>, so a real Manhattan "
     "complaint saddled with corrupt coordinates is self-evident at a glance. "
     "<code>--smarter</code> runs <code>moarstats --advanced</code> "
     "to enrich the stats cache, and a committed <code>--dictionary</code> "
     "(<code>nyc311_dict.schema.json</code> — a Data Dictionary generated for this dataset, then "
     "reviewed and committed alongside it) tags the record's identifier columns (<i>Unique Key</i>, "
     "<i>BBL</i>) so they're skipped rather than charted as quantities, labels the State-Plane "
     "coordinates as coordinate dimensions, and supplies friendly names (e.g. "
     "<i>Complaint Creation Date</i>, <i>Resolution Deadline</i>), so the "
     "profiler treats a service-request log as volume-and-category data. Alongside the maps the "
     "auto-profiler fills the dashboard with frequency bars, an <b>hour-of-day</b> seasonality "
     "profile, a time trend, a <b>parallel-categories (parcats) flow</b> over 3-4 associated "
     "categorical columns (co-occurrence ribbons, auto-chosen over a nested treemap/sunburst for "
     "this many-to-many set) and a mean-by-borough panel. New here is an "
     "auto-selected, colored <b>Sankey flow</b> (<i>Agency Code → Submission Channel</i>): it "
     "traces how each city agency's complaints arrive by channel, the thickest ribbons being "
     "<i>HPD → PHONE</i> (1,426) and <i>NYPD → PHONE</i> (1,348). "
     "<code>--bivariate</code> adds an <b>NMI association heatmap</b> across the <b>36 charted "
     "columns</b> plus a ranked "
     "<b>Top Relationships</b> panel — a horizontal <b>multivariate lollipop</b> where each dot's "
     "position is the pair's NMI on a value axis <b>zoomed</b> to the shown band (so near-ceiling "
     "associations separate instead of crushing together at 1.0), its <b>size</b> encodes "
     "co-occurrence support, and an <b>amber</b> dot flags a nonlinear pair. The top pair is a "
     "purely categorical one a Pearson-only heatmap could never surface, since neither column is "
     "numeric: <i>Borough</i> × <i>Park Borough</i> (NMI=1.0, n=10,000 of 10,000). It also flags a "
     "genuine <b>nonlinear</b> pair in amber — <i>Closed Date</i> × <i>Due Date</i> (NMI=0.9994): a "
     "complaint's actual close date is almost perfectly rank-associated with its deadline, yet how "
     "far the two land apart varies by complaint type, curving the relationship in a way a linear "
     "correlation alone would understate. The ranking is <b>support-weighted</b>: a pair only "
     "qualifies "
     "when its co-occurring row count is at least 10% of the best-supported pair's, so a "
     "technically-perfect NMI from two sparsely-populated columns can't crowd out a more broadly "
     "meaningful one — the top of the ranking instead surfaces genuinely dataset-wide pairs like "
     "<i>Borough</i> × <i>Park Borough</i> (n=10,000) and "
     "<i>Due Date</i> × <i>Resolution Action Updated Date</i> (NMI=0.9996, n=3,467). "
     "<code>--dict-info</code> adds a per-panel info icon and a slide-over <b>Data Dictionary</b> "
     "drawer sourced from the same committed schema, with an <b>Export&nbsp;JSONSchema</b> download "
     "that saves the dictionary verbatim. The run stays fully "
     "<b>deterministic</b> and offline: because the dictionary is committed, <code>--bivariate</code> "
     "never triggers a live <code>--dictionary infer</code> pass.",
     True, ["smart", "nyc_311.csv", "--smarter", "--bivariate", "--dict-info",
            "--dictionary", "nyc311_dict.schema.json",
            "--geojson", "nyc_neighborhoods.geojson"]),
    ("smart dashboard (--smarter, curated --dictionary, region-code zip choropleth)",
     "All <b>50,013</b> Allegheny County lifetime dog licenses, profiled into auto-chosen panels. "
     "The headline panel is a <b>summary choropleth keyed off a region-code COLUMN</b>, not "
     "coordinates: this dataset has <i>no</i> lat/lon, only an <code>OwnerZip</code> column, so "
     "<code>viz smart</code> aggregates <b>licenses per zip</b> and fills the matching "
     "<a href=\"https://data.wprdc.org/dataset/allegheny-county-zip-code-boundaries2/"
     "resource/14e5de97-0a5f-4521-84f6-ba74413db598\">Allegheny County zip-boundary</a> "
     "polygons from "
     "<code>--geojson</code> (<code>--feature-id-key properties.ZIP</code>). The key column is "
     "auto-chosen by matching each geo-dimension column's values against the boundary ids — here "
     "<b>118 zips</b> match, from 1 license (rural fringe) to <b>2,866</b> in zip 15237 (suburban "
     "North Hills). Because the matched regions span a metro-scale extent, the fill lands on an "
     "interactive <b>MapLibre tile basemap</b> (token-free carto tiles) rather than the coarse "
     "projection basemap used for country/continental choropleths. A curated "
     "<code>--dictionary</code> (<code>allegheny_dogs_dict.schema.json</code>) tags "
     "<code>OwnerZip</code> as <code>geo.zip_code</code> — the signal that turns a numeric zip "
     "code into a choropleth key instead of a frequency bar — and marks <code>_id</code>/"
     "<code>DogName</code> as identifiers so they're skipped. This dataset carries no numeric "
     "<b>measure</b>, so only the per-zip <b>count</b> map is drawn; a dataset that also tags a "
     "measure column (e.g. a sale price) additionally gets a per-zip <b>median-of-measure</b> "
     "choropleth beside it. Alongside the map the profiler fills the dashboard with "
     "<code>LicenseType</code>/<code>Breed</code>/<code>Color</code> frequency bars and, via "
     "<code>--bivariate</code>, an <b>NMI association heatmap</b> (Breed ↔ Color) plus a ranked "
     "Top&nbsp;Relationships panel. <code>--dict-info</code> adds a per-panel info icon and a "
     "slide-over <b>Data Dictionary</b> drawer sourced from the same curated schema. Fully "
     "deterministic — no LLM needed (the curated dictionary is reviewed and committed, exactly "
     "like an <code>--dictionary infer</code> pass would produce).",
     True, ["smart", "allegheny_dog_licenses.csv", "--smarter", "--bivariate", "--dict-info",
            "--dictionary", "allegheny_dogs_dict.schema.json",
            "--geojson", "allegheny_zip_boundaries.geojson",
            "--feature-id-key", "properties.ZIP"]),
    ("smart dashboard (animated geo, world events)",
     "Auto dashboard for world_events_dated: global-extent dated points across six continents "
     "(lon span ~300&deg;, lat span ~99&deg;). Because the extent is continental/global, "
     "<code>viz smart</code> draws a <b>ScatterGeo projection basemap</b> (which animates natively, "
     "unlike a MapLibre tile map) and adds an <b>animated geographic reveal</b>: the points "
     "accumulate <b>cumulatively over monthly time buckets</b> (Play/Pause + scrub slider). A "
     "city-scale point cloud would stay a static map — the animation only fires for large extents.",
     True, ["smart", "world_events_dated.csv"]),
    ("smart dashboard (Gapminder bubble, regions growth)",
     "Auto dashboard for regions_growth: a low-cardinality categorical entity "
     "(<code>region</code>) + a measure pair (<code>gdp_index</code> vs "
     "<code>wellbeing_index</code>) + a monthly date drive a <b>Gapminder-style animated bubble "
     "chart</b> &mdash; one bubble per region, each tracing a <b>distinct curved path</b> through "
     "the measure space over time, sized by per-cell record count and colored by region (Play/Pause "
     "+ scrub slider, legend of regions). Strict quality gates (min rows per cell, near-complete "
     "per-entity panel) keep it a real drift story rather than noisy jitter.",
     True, ["smart", "regions_growth.csv"]),
]

# A final, non-plotly gallery entry: a clickable screenshot that links out to the full, standalone
# "Pittsburgh 311 smart visual data dictionary" page. That page (pitt311data.html) is a ~19MB
# self-contained `qsv viz smart` dashboard — too large to embed inline as an iframe like the
# smart_*.html figures — so the gallery shows a scaled screenshot that opens it in a new popup
# window. Kept OUT of FIGURES so it never runs through `qsv viz` as a chart command; its `cmd` is
# the verbatim command that produced the dashboard (rendered as a copy-pasteable block, unwrapped).
SCREENSHOT = {
    "title": "Pittsburgh 311 smart visual data dictionary",
    "desc": (
        "A full <code>qsv viz smart</code> <b>visual data dictionary</b> over real "
        "<b>Pittsburgh 311</b> service requests from the "
        '<a href="https://data.wprdc.org/dataset/pittsburgh-311-data">Western Pennsylvania '
        "Regional Data Center (WPRDC)</a>. The dashboard bins each request's lat/lon by "
        "point-in-polygon into Pittsburgh's neighborhood polygons "
        "(<code>--geojson pittsburgh-neighborhoods</code>, no geocoding); a curated "
        "<code>--dictionary</code> (<code>pitt311data.schema.json</code>) tags identifier/code "
        "columns and supplies friendly field labels, and <code>--dict-info</code> renders that "
        "dictionary as its own in-page <b>Data Dictionary</b> tab (with a hover/click info icon "
        "on every panel title). <code>--smarter</code> enriches the stats cache "
        "(<code>moarstats --advanced</code>) and <code>--bivariate</code> adds the NMI "
        "association heatmap plus the ranked top-relationships panel, while "
        "<code>--dataset-pid</code> adds a clickable citation link back to the source dataset. "
        "The standalone page is a ~19&nbsp;MB self-contained dashboard &mdash; too large to embed "
        "inline &mdash; so this is a screenshot: <b>click it to open the fully interactive "
        "dashboard in a new window</b>."),
    "image": "pitt311data-visual-datadic.png",
    "href":  "pitt311data.html",
    "cmd":   ("qsv viz smart pitt311data.csv --smarter --dictionary pitt311data.schema.json "
              "--dict-info --bivariate -o pitt311data-smart-visual-datadic.html "
              "--geojson pittsburgh-neighborhoods "
              "--dataset-pid https://data.wprdc.org/dataset/pittsburgh-311-data"),
}


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


def anchor_id(title):
    """Stable, hyphenated DOM id / URL fragment for a figure (e.g. `fig-choropleth-us-states`),
    so the Table of Contents can deep-link to it and the fragment is shareable. Titles are unique,
    so the derived ids are too."""
    return "fig-" + re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")


def toc_label(title):
    """Display text for a figure's Table-of-Contents link. The many `smart dashboard (...)` figures
    share a long `smart dashboard ` prefix that buries the distinguishing parenthetical and gets
    ellipsis-truncated in the TOC grid, so move the common `dashboard` word to the end (e.g.
    `smart dashboard (--smarter, geospatial)` -> `smart (--smarter, geospatial) dashboard`). The
    figure title itself, its anchor id and the iframe lookup keys are unchanged — this only affects
    the TOC link text. Bare `smart dashboard` (no parenthetical) round-trips unchanged."""
    prefix = "smart dashboard"
    if title.startswith(prefix):
        return "smart" + title[len(prefix):] + " dashboard"
    return title


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


def command_block_html(tokens):
    """The copy-pasteable command block (wrapped display form + icon-only Copy button) for a token
    list. Shared by the per-figure figcaptions and the final screenshot entry — the latter shows a
    verbatim command rather than one synthesized from viz args."""
    display = html_escape(WRAP_SEP.join(wrap_command_lines(tokens)))
    oneline = html_escape(" ".join(tokens), quote=True)
    return (f'<div class="cmdbox"><button class="copy" type="button" title="Copy" '
            f'aria-label="Copy command to clipboard" data-cmd="{oneline}">'
            f'{COPY_ICON_SVG}{CHECK_ICON_SVG}</button>'
            f'<pre class="cmd"><code>{display}</code></pre></div>')


def figcaption_html(title, desc, args):
    """A figure's `<figcaption>`: title, description, then the runnable command block with a
    Copy button. The displayed command is wrapped for readability; the button copies the
    single-line form."""
    return (f'<figcaption><span class="t">{title}</span>'
            f'<span class="d">{desc}</span>'
            f'{command_block_html(viz_command_tokens(title, args))}</figcaption>')


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


# Markers of a non-plaintext payload in viz output. All three are literals viz emits — a
# script-tag content type, a JS call, and the exact typed-array serialization — none of which
# scraped user data can produce.
#
# The two encodings QSV_VIZ_NO_COMPRESS disables have DIFFERENT thresholds, so checking only the
# gzip ones is not enough: a figure gzips at >=64KB of JSON (MAP_FIG_GZ_MIN_BYTES) but a coordinate
# array becomes base64 float32 at just >=64 elements (BDATA_MIN_LEN). A small map panel (e.g.
# delivery_stops.csv, 201 rows) therefore emits `bdata` and no gzip marker at all.
COMPRESSED_MARKERS = (
    'type="application/gzip-b64"',       # gzipped bundle / figure payload
    "qsvNewPlotGz(",                     # deferred render of a gzipped figure
    '"dtype":"float32","bdata":"',       # base64 float32 typed array (fields serialized in order)
)


def assert_plaintext(html, source):
    """Fail loudly if viz output is not fully plain text.

    `extract_inline_panels` scrapes plaintext `Plotly.newPlot(...)` calls; a gzipped panel is
    emitted as `qsvNewPlotGz(...)` + a base64 blob instead, so it would be skipped *silently* —
    a map panel would simply vanish from the gallery with no error. Typed arrays don't break the
    scrape (they parse as nested JSON) but do turn committed iframes into undiffable base64.
    QSV_VIZ_NO_COMPRESS in `run_html` prevents both; this asserts it actually took effect (e.g.
    it wasn't dropped, or overridden to a falsy value)."""
    found = [m for m in COMPRESSED_MARKERS if m in html]
    if found:
        raise ValueError(
            f"{source} is not plain text ({', '.join(found)}). The gallery needs plain-text "
            "output to scrape figure JSON and to commit diffable iframes — run_html sets "
            "QSV_VIZ_NO_COMPRESS=1 for this; check it is not being overridden (a falsy value in "
            "the environment or a .env file disables it). Pre-generated iframes must have been "
            "produced the same way."
        )


def run_html(qsv, args):
    """Run `qsv viz <args>` and return its HTML output as a string.

    QSV_VIZ_CDN makes viz emit a plotly CDN `<script src>` instead of the ~4.6MB inline bundle,
    so the smart-dashboard iframes are small enough to commit. QSV_VIZ_NO_COMPRESS keeps figure
    payloads in the fully readable plain form (no gzip+DecompressionStream, no base64 float32
    typed arrays), which this script needs to scrape figure JSON out of the HTML.

    Note QSV_VIZ_CDN alone is not enough: it only swaps the plotly *bundle* for a CDN tag, while
    map-panel *figures* keep their gzip+bdata encoding."""
    fd, out = tempfile.mkstemp(suffix=".html")
    os.close(fd)
    try:
        subprocess.run([qsv, "viz", *args, "-o", out], cwd=VIZ_DIR,
                       check=True, capture_output=True, text=True,
                       env={**os.environ, "QSV_VIZ_CDN": "1", "QSV_VIZ_NO_COMPRESS": "1"})
        with open(out, encoding="utf-8") as fh:
            html = fh.read()
    finally:
        os.unlink(out)
    assert_plaintext(html, f"`qsv viz {' '.join(args)}` output")
    return html


def run_fig(qsv, args):
    html = run_html(qsv, args)
    panels = extract_inline_panels(html)
    if panels is not None:
        return {"panels": panels}          # inline multi-panel dashboard
    return {"fig": extract_fig_json(html)}  # single grid-form figure


def inject_resize_reporter(html):
    """Add the postMessage height/user-scroll reporter just before the page's real </body> so the
    iframe can be auto-sized to the dashboard with no inner scrollbar and no trailing whitespace.
    Anchor on the LAST </body>, not the first: `--dict-info` dashboards embed a complete standalone
    HTML document (with its own </body></html>) as a string inside the qsvOpenDictTab script, so a
    first-match replace would inject the reporter inside that script string, where it never runs —
    leaving the iframe stuck at its initial height (the Allegheny dog-licenses dashboard clipping).

    Idempotent: strip any previously-injected reporter (a <script> containing qsvVizHeight; it has no
    literal `<` in its body) before re-adding the current one, so re-running against an already-built
    dashboard (e.g. the reused pre-generated ones) refreshes the reporter instead of duplicating it."""
    html = re.sub(r"<script>[^<]*qsvVizHeight[^<]*</script>\n?", "", html)
    idx = html.rfind("</body>")
    if idx == -1:
        return html + RESIZE_REPORTER_JS
    return html[:idx] + RESIZE_REPORTER_JS + "\n" + html[idx:]


def grid_cols(args):
    """The --grid-cols value from a smart dashboard's args (default 2)."""
    if "--grid-cols" in args:
        return int(args[args.index("--grid-cols") + 1])
    return 2


def cleanup_sidecars():
    # `viz smart` writes a stats cache next to the CSV; `--smarter` (via its internal
    # `moarstats --advanced`) also auto-creates an `.idx` index; `--bivariate` additionally
    # writes a `*.stats.bivariate.csv` sidecar (a separate moarstats output, so it doesn't
    # match the `.stats.csv` substring check below). Don't leave any of these in the tree
    # (the committed datasets ship without them).
    for f in os.listdir(VIZ_DIR):
        if (".stats.csv" in f or ".stats.jsonl" in f or ".stats.bivariate.csv" in f
                or f.endswith(".idx")):
            os.unlink(os.path.join(VIZ_DIR, f))


def main():
    qsv = find_qsv()
    # QSV_VIZ_REGEN_LLM opts into regenerating the `--dictionary infer` dashboards live (needs a
    # local LLM up); otherwise their committed HTML is reused so a normal run stays LLM-free.
    # Only an explicit truthy value enables it, so QSV_VIZ_REGEN_LLM=0/false/off (or empty) stays
    # off rather than enabling LLM work just because the var is present.
    regen_llm = os.environ.get("QSV_VIZ_REGEN_LLM", "").strip().lower() in {"1", "true", "yes", "on"}
    pregenerated = set() if regen_llm else PREGENERATED
    if not pregenerated:
        sys.stderr.write("QSV_VIZ_REGEN_LLM set: regenerating LLM dashboards (local LLM required)\n")
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
    # clickable-screenshot styling for the final "visual data dictionary" entry
    # (idempotent: drop any prior `figure a.shot` rules before re-adding)
    head = re.sub(r"\s*figure a\.shot[^{]*\{[^}]*\}", "", head)
    head = head.replace("</style>", " " + SHOT_CSS + "\n</style>", 1)
    # opaque backdrop when a reconstructed figure is sent fullscreen via its modebar button
    # (idempotent: drop any prior rule before re-adding)
    head = re.sub(r"\s*\.js-plotly-plot:fullscreen\{[^}]*\}", "", head)
    head = head.replace("</style>", " .js-plotly-plot:fullscreen{background:#fff}\n</style>", 1)
    # navigation + responsive CSS (idempotent: drop any prior marked block before re-adding)
    head = re.sub(r"\s*/\*qsv-nav\*/.*?/\*end-qsv-nav\*/", "", head, flags=re.S)
    head = head.replace("</style>", " /*qsv-nav*/" + NAV_CSS + "/*end-qsv-nav*/\n</style>", 1)
    # sticky "Jump to a chart" Table of Contents, generated from FIGURES so it always lists every
    # figure in order. Inserted right before the grid (idempotent: strip any prior copy first).
    head = re.sub(r'<details class="toc">.*?</details>\n', "", head, flags=re.S)
    toc_links = "".join(
        f'<a href="#{anchor_id(t)}">{html_escape(toc_label(t))}</a>' for (t, _d, _f, _a) in FIGURES)
    # final entry: the clickable screenshot / visual data dictionary (a link-out, not a chart)
    toc_links += (f'<a href="#{anchor_id(SCREENSHOT["title"])}">'
                  f'{html_escape(SCREENSHOT["title"])}</a>')
    toc_html = (f'<details class="toc"><summary>Jump to a chart'
                f'<span class="toc-count">{len(FIGURES)} charts + data dictionary</span></summary>'
                f'<div class="toc-links">{toc_links}</div></details>\n')
    head = head.replace('<div class="grid">', toc_html + '<div class="grid">', 1)

    figs, fig_divs, plots = [], [], []
    for idx, fig in enumerate(FIGURES):
        title, desc, full, args = fig
        gid = f"g{idx}"
        anchor = anchor_id(title)  # TOC deep-link target; distinct from gid (the inner plot div id)
        iframe_name = SMART_IFRAME.get(title)
        if iframe_name:
            # embed the genuine `qsv viz smart` output (CDN-slimmed) as a full-width iframe so the
            # real full-width overview panels, theme and map buttons render as the CLI produces them
            if iframe_name in pregenerated:
                # LLM-dependent (`--dictionary infer`): reuse the committed, already-cdnified HTML so
                # regen stays offline & deterministic (refresh it manually — see README commands).
                # Still refresh the injected reporter in place (idempotent) so the height/user-scroll
                # postMessage stays current across all dashboards, not just the regenerated ones.
                sys.stderr.write(f"[{idx}] {title}: reusing pre-generated {iframe_name}\n")
                pre_path = os.path.join(VIZ_DIR, iframe_name)
                with open(pre_path, encoding="utf-8") as fh:
                    existing = fh.read()
                # reused verbatim, so it never passed through run_html's check — validate it here,
                # else a dashboard refreshed without QSV_VIZ_NO_COMPRESS gets silently re-committed
                assert_plaintext(existing, f"pre-generated {iframe_name}")
                refreshed = inject_resize_reporter(existing)
                if refreshed != existing:
                    with open(pre_path, "w", encoding="utf-8") as fh:
                        fh.write(refreshed)
            else:
                sys.stderr.write(f"[{idx}] {title}: qsv viz {' '.join(args)} -> {iframe_name}\n")
                html = inject_resize_reporter(run_html(qsv, args))
                with open(os.path.join(VIZ_DIR, iframe_name), "w", encoding="utf-8") as fh:
                    fh.write(html)
            figs.append(None)  # keep FIGS index aligned with idx for the non-iframe figures
            fig_divs.append(
                f'<figure class="cell full" id="{anchor}">{figcaption_html(title, desc, args)}'
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
                f'<figure class="cell full" id="{anchor}">{figcaption_html(title, desc, args)}'
                f'<div style="display:grid;grid-template-columns:repeat({cols},minmax(0,1fr));'
                f'gap:14px">{cells}</div></figure>'
            )
            for k in range(len(panels)):
                plots.append(
                    f'Plotly.newPlot("{gid}-p{k}", FIGS[{idx}][{k}].data, '
                    f'FIGS[{idx}][{k}].layout || {{}}, '
                    f'Object.assign({{responsive:true,scrollZoom:false,modeBarButtonsToAdd:[qsvFsBtn,qsvLegendBtn]}}, '
                    f'FIGS[{idx}][{k}].config || {{}}));'
                )
        else:
            figs.append(result["fig"])
            cls = "cell full" if full else "cell"
            fig_divs.append(
                f'<figure class="{cls}" id="{anchor}">{figcaption_html(title, desc, args)}'
                f'<div id="{gid}" class="plot"></div></figure>'
            )
            # animation frames live on the figure object but the positional newPlot(div,data,layout,
            # config) form drops them, so re-add via Plotly.addFrames once newPlot resolves (mirrors
            # the CLI's FULLSCREEN_SCRIPT enhance() fix). No-op for the non-animated figures.
            plots.append(
                f'Plotly.newPlot("{gid}", FIGS[{idx}].data, FIGS[{idx}].layout || {{}}, '
                f'Object.assign({{responsive:true,scrollZoom:false,modeBarButtonsToAdd:[qsvFsBtn,qsvLegendBtn]}}, '
                f'FIGS[{idx}].config || {{}})).then(function(){{'
                f'var fr=FIGS[{idx}].frames; if(fr&&fr.length)Plotly.addFrames("{gid}",fr);}});'
            )

    # Final, clickable-screenshot entry (a full-width cell, no plotly figure): opens the standalone
    # pitt311data.html dashboard in a new popup window on click; target=_blank is the no-JS fallback
    # and the onclick's window.open gives the popup its own sized window. NOT added to `figs`/`plots`
    # — it carries no Plotly data or newPlot call, so the FIGS array stays aligned with the charts.
    shot = SCREENSHOT
    shot_anchor = anchor_id(shot["title"])
    fig_divs.append(
        f'<figure class="cell full" id="{shot_anchor}">'
        f'<figcaption><span class="t">{shot["title"]}</span>'
        f'<span class="d">{shot["desc"]}</span>'
        f'{command_block_html(shlex.split(shot["cmd"]))}</figcaption>'
        f'<a class="shot" href="{shot["href"]}" target="_blank" rel="noopener" '
        f'''onclick="window.open(this.href,'pitt311data','popup,width=1280,height=900');'''
        f'''return false;" '''
        f'title="Open the full interactive dashboard in a new window">'
        f'<img src="{shot["image"]}" loading="lazy" '
        f'alt="{html_escape(shot["title"])} screenshot"/></a></figure>'
    )

    figs_json = "const FIGS = [" + ",".join(
        json.dumps(f, ensure_ascii=False, separators=(",", ":")) for f in figs) + "];"
    body = (
        head + "\n"
        + "\n".join(fig_divs) + "\n"
        + "</div>\n<script>\n"
        + figs_json + "\n"
        + FS_BUTTON_JS + "\n"
        + "\n".join(plots) + "\n"
        + "</script>\n"
        + RESIZE_LISTENER_JS + "\n"
        + COPY_JS + "\n"
        + TOC_JS + "\n"
        + JUMP_JS + "\n"
        + "</body></html>\n"
    )
    # The reused scaffold carries its own plotly CDN <script> tag, which therefore never updates:
    # it stayed pinned to an old version with no Subresource Integrity even after `qsv viz` began
    # emitting a version-pinned, SRI-protected tag. Re-derive it from what viz ACTUALLY emitted
    # into the dashboards this run, so the gallery frame and its iframes always agree and the
    # single source of truth stays qsv itself rather than a second hardcoded version here.
    def _emitted_cdn_tag():
        # only REGENERATED dashboards: the pre-generated LLM ones are reused verbatim and so
        # predate whatever viz emits today (that is exactly the staleness being corrected here)
        for name in SMART_IFRAME.values():
            if name in pregenerated:
                continue
            path = os.path.join(VIZ_DIR, name)
            if not os.path.exists(path):
                continue
            with open(path, encoding="utf-8") as fh:
                m = re.search(r'<script src="https://cdn\.plot\.ly/[^>]*></script>', fh.read())
            if m:
                return m.group(0)
        return None

    emitted_tag = _emitted_cdn_tag()
    if emitted_tag:
        body, n = re.subn(
            r'<script src="https://cdn\.plot\.ly/[^>]*></script>', emitted_tag, body, count=1)
        if n:
            sys.stderr.write(f"scaffold plotly tag synced to viz output: {emitted_tag}\n")

    with open(GALLERY, "w", encoding="utf-8") as fh:
        fh.write(body)
    cleanup_sidecars()
    sys.stderr.write(f"wrote {GALLERY} ({len(body)} bytes, {len(figs)} figures)\n")


if __name__ == "__main__":
    main()
