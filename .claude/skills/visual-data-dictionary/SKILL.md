---
name: visual-data-dictionary
description: Build a Visual Data Dictionary — an interactive qsv viz smart dashboard (a Data Schematic) driven by an LLM-inferred JSON Schema data dictionary, with the dictionary browsable beside the charts. Use when the user asks for a visual data dictionary, a documented dashboard, a dictionary-driven dashboard or Data Schematic, or wants to explore and document a CSV at the same time. Optionally bins rows into GeoJSON regions.
argument-hint: "<input.csv> [geojson]"
---

# /visual-data-dictionary

> **Scope: repo-local.** This lives beside `build-dashboard`, `release-prep` and
> `review-respond` at the top of `.claude/skills/`, which `package-plugin.js` and
> `package-mcpb.js` do **not** archive — they ship only `.claude/skills/skills/`.
> So this skill is available when working in the qsv repo and is *not* part of the
> distributed plugin. That is deliberate: the packaged skills drive qsv through the
> `mcp__qsv__*` MCP tools, while this one drives the **qsv CLI** directly and needs
> `Bash` plus `python3`. Shipping it would require rewriting it against the MCP
> tool surface, which has no equivalent for the GeoJSON inspection or the HTML
> verification below (nor for the optional Stage 6 browser pass).
>
> **Requires:** `qsv` on `PATH`, `python3`, and an LLM endpoint for `describegpt`.
> Stage 6 (optional) additionally needs a browser-automation MCP — any one
> (Playwright MCP, claude-in-chrome, …); skip the stage when none is available.

Turn a CSV into a self-contained HTML Data Schematic whose panels are chosen from an
LLM-inferred data dictionary, with that dictionary embedded beside the charts.

Four stages, plus one optional fine-tune and one optional browser pass, in this
order and no other:

1. **denull** — blank null sentinels so numeric columns are actually numeric
2. **describegpt** — infer a JSON Schema data dictionary from the *cleaned* data
   - **2.5 fine-tune** (optional) — hand-correct the dictionary in a terminal UI
     before it drives the Data Schematic
3. **geojson** (optional) — pick a feature id key by inspecting the file
4. **viz smart** — render the Data Schematic, dictionary-driven, dictionary-embedded
   - **5 verify** — check the HTML, then report
   - **6 tour refinement** (optional, browser) — step through the guided Tour and
     refine its `x-qsv.tour` narration against what actually rendered

The order is load-bearing. Clean first, then describe, then draw. A dictionary
built from dirty data documents a `String` column that is really a number, and
`viz smart` will then chart it as a category or skip it outright.

## IMPORTANT

You must **execute bash commands**. Never invent qsv flags — if unsure, run
`qsv <cmd> --help`. Skip any step already satisfied by conversation context.
Defer to CLAUDE.md when it conflicts with this skill.

## Naming

Given input `data.csv`, derive:

| var | value | note |
|---|---|---|
| `STEM` | `data` | basename minus extension |
| `WORK` | `data.denulled.csv`, or `data.csv` if nothing was cleaned | what stages 2–4 read |
| `SCHEMA` | `<WORK stem>.schema.json` | `viz --dictionary infer` reuses this exact name |
| `OUT` | `data.html` | **always the ORIGINAL stem**, per the user's expectation |

Never write to the input path. `denull --apply` refuses to overwrite its own
input (it compares file identity, so a hard link is caught too), but pick a
distinct `-o` anyway.

---

## Stage 0 — Preconditions

```bash
command -v qsv >/dev/null || { echo "qsv not on PATH"; exit 1; }
test -f "$INPUT" || { echo "no such file: $INPUT"; exit 1; }
qsv headers "$INPUT" | head -30
qsv count "$INPUT"
```

Only CSV/TSV/SSV. If handed a spreadsheet, convert first (`qsv excel`).

Build the index and stats cache once — every later stage reuses them:

```bash
qsv index "$INPUT"
qsv stats "$INPUT" --everything --stats-jsonl --force > /dev/null
```

## Stage 1 — denull

**Report first. Always show the user before changing their data.**

```bash
qsv denull "$INPUT"
```

Read the `verdict` column:

- **No rows, or no `confirmed` row** → nothing to clean. Set `WORK="$INPUT"` and
  go to Stage 2. Do not create a copy.
- **One or more `confirmed`** → show the table, then:

```bash
qsv denull --apply "$INPUT" -o "${STEM}.denulled.csv"
qsv index "${STEM}.denulled.csv"
qsv stats "${STEM}.denulled.csv" --everything --stats-jsonl --force > /dev/null
```

Set `WORK="${STEM}.denulled.csv"`.

`--apply` prints its report to **stderr** and the cleaned CSV to `-o`, and blanks
sentinels **only** in the columns it confirmed. Every other column is copied
through byte-for-byte.

Sanity check worth doing: each confirmed column's `rows_affected` should equal
its `nullcount` in the new stats.

```bash
qsv stats "${STEM}.denulled.csv" | qsv select field,type,nullcount | qsv table
```

Two things to tell the user, because they are not obvious:

- `denull` only confirms columns that would **promote to a numeric type** once
  blanked. A categorical column holding `NULL` (e.g. `status` = ok/pending/NULL)
  is deliberately left alone — blanking it promotes nothing. Stage 2 will still
  surface it.
- Numeric sentinels (`-999`, `9999`) are **not** detectable by any scan: they
  parse as valid numbers. Only Stage 2's LLM can propose them, and only a human
  should apply them.

## Stage 2 — describegpt → JSON Schema dictionary

### Resolve the LLM endpoint

Detect, then prompt only if nothing is found. Do **not** print key values.

```bash
for v in QSV_LLM_BASE_URL OPENAI_API_KEY QSV_LLM_APIKEY ANTHROPIC_API_KEY; do
  val=$(printenv "$v" 2>/dev/null); [ -n "$val" ] && echo "$v is set"
done
curl -s -m 2 http://localhost:1234/v1/models >/dev/null 2>&1 && echo "LM Studio on :1234"
curl -s -m 2 http://localhost:11434/api/tags  >/dev/null 2>&1 && echo "ollama on :11434"
```

Both LM Studio and ollama speak the OpenAI-compatible API, so **both** list models
the same way and both take a `/v1` base URL. Only the port differs:

| server | `--base-url` | list models |
|---|---|---|
| LM Studio | `http://localhost:1234/v1` | `curl -s http://localhost:1234/v1/models` |
| ollama | `http://localhost:11434/v1` | `curl -s http://localhost:11434/v1/models` |

```bash
# honor an explicit QSV_LLM_BASE_URL first; only probe local servers when it is unset
BASE_URL="${QSV_LLM_BASE_URL:-}"
[ -z "$BASE_URL" ] && curl -s -m 2 http://localhost:1234/v1/models  >/dev/null 2>&1 && BASE_URL=http://localhost:1234/v1
[ -z "$BASE_URL" ] && curl -s -m 2 http://localhost:11434/api/tags >/dev/null 2>&1 && BASE_URL=http://localhost:11434/v1

[ -n "$BASE_URL" ] && curl -s "$BASE_URL/models" \
  | python3 -c 'import sys,json;[print(m["id"]) for m in json.load(sys.stdin)["data"]]'
```

`/api/tags` is only a liveness probe for ollama — it returns ollama's native
shape, not the OpenAI `{"data":[...]}` envelope. List models from `/v1/models`
either way.

If nothing is found, use **AskUserQuestion** for base URL + model. Never guess a
model name. Offer the models the server actually reports; do not type one from
memory.

### Generate

First ask with **AskUserQuestion**: *"Who is the Data Schematic's guided Tour
for?"* — default `TOUR_AUDIENCE="Explain like I'm 10"`; any free-text audience
works ("a board of directors", "data journalists", …).

```bash
qsv describegpt "$WORK" \
  --dictionary --description --two-pass --infer-content-type \
  --format JSONSchema \
  --tour-audience "$TOUR_AUDIENCE" \
  ${BASE_URL:+--base-url "$BASE_URL"} --model "$MODEL" \
  -o "$SCHEMA"
```

- `--infer-content-type` is **mandatory here**, not optional: `viz smart` routes
  panels off each field's `role` and `concept`, and those are only inferred under
  this flag. Without it the dictionary loads and changes nothing. It is also the
  only way to get the two dictionary hints that unlock extra panels:
  per-field `x-qsv.gauge_range` (turns a measure's KPI tile into a gauge; kept
  only when the observed data lies inside the range) and the dataset-level
  `x-qsv.relationships` array, whose `"kind": "pipeline"` entry is the **only**
  source of the pipeline funnel/bridge panel.
- Pass `--context-file <file>` when the user has a glossary, README or codebook.
  Better context yields better roles, concepts and labels, hence a better
  Data Schematic. (`viz --dictionary-context` is the same thing for the `infer` path,
  which this skill does not take.)
- `--two-pass` roughly doubles cost and latency. It is what lets the model relate
  fields to one another (`street_no` + `street` + `city` + `zip` = one address),
  which is what makes the routing good.
- Naming it `<WORK stem>.schema.json` means a later
  `qsv viz smart "$WORK" --dictionary infer` finds and **reuses** it instead of
  paying for the LLM again. Delete the file to force a re-infer.
- `--tour-audience` makes describegpt also write a dataset-level `x-qsv.tour`
  narration — the prose the Data Schematic's guided Tour speaks — in the
  audience's register. The audience shapes ONLY the tour prose; labels and
  descriptions keep their normal register. Stage 6 refines it in a browser.

Optionally add `--infer-null-values` to have the model propose null sentinels
into each property's `x-qsv` object, split into `null_values` (confirmed present
by qsv) and `null_candidates` (guesses, each stamped `confirm_required: true`).
This is the only route to numeric sentinels like `-999`. It is **reported, never
applied** — nothing downstream acts on it.

Verify the dictionary carries what `viz` needs before spending time on Stage 4:

```bash
python3 - "$SCHEMA" <<'PY'
import json, sys
s = json.load(open(sys.argv[1]))
p = s["properties"]
have = sum(1 for v in p.values() if v.get("x-qsv", {}).get("role"))
print(f"role/concept on {have}/{len(p)} columns")
if have == 0:
    print("WARNING: no roles inferred — was --infer-content-type passed?")
# panel-unlocking hints, so the user knows up front what will/won't be drawn
gauges = [k for k, v in p.items() if (v.get("x-qsv") or {}).get("gauge_range")]
# viz reads pipelines ONLY from the dataset-level x-qsv (see xq_pipelines in
# src/cmd/viz.rs) — a root-level "relationships" array draws nothing.
rels = (s.get("x-qsv") or {}).get("relationships") or []
pipes = [r for r in rels if r.get("kind") == "pipeline"]
print(f"gauge_range on {len(gauges)} measure(s): {', '.join(gauges) or '(none)'}")
print(f"relationships: {len(rels)} ({len(pipes)} pipeline -> funnel/bridge panel)")
if not rels and s.get("relationships"):
    print("WARNING: relationships found at the ROOT, not under x-qsv — viz ignores"
          " those. Is this the flat JSON dictionary instead of JSONSchema?")
tour = (s.get("x-qsv") or {}).get("tour")
if tour:
    print(f"tour: version {tour.get('version')}, audience {tour.get('audience')!r}, "
          f"{len(tour.get('overrides') or {})} override(s), "
          f"{len(tour.get('panels') or {})} panel narration(s)")
else:
    print("tour: (none — was --tour-audience passed?)")
PY
```

No `gauge_range` and no `pipeline` is a perfectly normal outcome — most datasets
have neither a canonical-scale measure nor a staged process. Say so and move on;
both can be hand-added later (see Stage 2.5).

## Stage 2.5 — Fine-tune the dictionary (optional, TUI)

`describegpt` is a good first draft, not gospel — and the draft is not even
stable: because the semantic half comes from an LLM, inferring twice over the
same data can return different `role`/`concept` assignments, and `role` decides
which panel a column gets (qsv issue #4407). **This stage is what makes a
Data Schematic reproducible.** The corrected dictionary — reviewed, kept beside
the data, committed if the data is versioned — is the artifact of record; every
later run reuses it instead of re-rolling the model.

The five fields that actually steer `viz smart` — `x-qsv.role`,
`x-qsv.concept`, `title` (label), `description` and `x-qsv.aggregation` — are
worth a human pass when the model mislabels a column: a code that should be an
`identifier` charted as a `measure`, a `geo.*` key left `unknown`, a per-unit
price summed into a meaningless total, a bland label. `edit_dictionary.py`
(beside this `SKILL.md`) is a curses UI that walks every column and, as you
edit, **previews how `viz smart` will route it** (Skip / Dimension / Temporal /
MapCoord / ProjectedCoord / Measure — the last showing its aggregation,
`Measure(sum)` for an additive amount, `Measure(mean)` for a ratio, a duration,
or anything you tag `aggregation: mean`), so you see the effect before
rendering. It touches only those five fields, preserves every other key, and
rewrites the file only if you save.

`aggregation` is the one field qsv can also **drop** on read, and the `!` flag
mirrors exactly when that happens: the token must be `sum`/`mean`,
`x-qsv.qsv_type` must be `Integer`/`Float` (or absent), `x-qsv.role` must be
empty or exactly `measure`, *and* the column must route to a measure at all. A
value failing any of those is flagged and left out of the ROUTE preview, because
viz silently falls back to its own name heuristic there. It catches the three
easy mistakes: `"average"` instead of `"mean"`, an aggregation left behind on a
column you just re-roled to `dimension`, and one on a column nothing classifies
(`Defer→stats`), where viz's stats floor discards it outright.

**Offer it with AskUserQuestion:** *"Hand-tune the data dictionary in a TUI
before rendering?"* If **no**, go straight to Stage 3 — but say plainly that the
Data Schematic then rests on an unreviewed draft, and that the dictionary can be
tuned and re-rendered at any time without paying for the LLM again.

If **yes**, you cannot drive it yourself — a curses TUI needs the user's real
terminal, and your Bash tool is a captured, non-interactive shell (the script
detects this and refuses). So run it **out-of-band**:

1. Show the current routing so the user knows the starting point (this works
   without a TTY):

   ```bash
   python3 "$SKILL_DIR/edit_dictionary.py" --summary "$SCHEMA"
   ```

   where `$SKILL_DIR` is this skill's own directory (the folder holding this
   `SKILL.md`).

2. Tell the user to run this in **their own terminal**, then **end your turn and
   wait** — do not proceed:

   ```
   python3 "<skill dir>/edit_dictionary.py" "<SCHEMA path>"
   ```

   Keys: `↑↓` move · `r` role · `c` concept · `l` label · `d` description ·
   `a` aggregation · `s` save · `q` quit. `role`/`concept` open a filterable
   picker (type to filter; off-vocab values are allowed but flagged with `*`).
   `aggregation` offers `sum` / `mean` / clear only — that is the whole
   vocabulary qsv accepts — and clearing removes the key rather than blanking
   it, restoring qsv's own guess.

3. When the user says they're done, re-read the file: re-run the **Stage 2
   coverage check** and the `--summary` above, and show a short **before/after**
   of any rows whose role/concept/route changed. Then continue to Stage 3.

Because the dictionary keeps its `<WORK stem>.schema.json` name, Stage 4 picks up
the edited file with no extra wiring. If the user edits nothing, the file is
untouched byte-for-byte — treat that as a normal "looks good" outcome.

Scope note: the TUI deliberately does **not** edit null sentinels
(`--infer-null-values` output). Those are reported-never-applied and have no
`viz smart` effect, so editing them here would change nothing downstream.

Five keys that *do* affect the Data Schematic are outside the TUI, and are
**hand-edited in the JSON** — this is the supported path for them, not a
violation of the "never hand-write the schema" rule:

| key | where | effect |
|---|---|---|
| `x-qsv.gauge_range` | per property, `[min, max]` | KPI tile becomes a **gauge**. `describegpt` proposes it for canonical-scale measures; qsv drops it if the data falls outside the range |
| `x-qsv.target` | per property, a number | KPI tile gains a **"vs target" delta**. Never inferred — it is a goal only the user knows |
| `x-qsv.currency` | per property, an ISO-4217 code (`"USD"`) | KPI tile is prefixed with the currency **symbol** (`$192B`) and the panel subtitle names the currency. `describegpt` proposes it for money columns; qsv drops it unless the column is a numeric measure that reads as money (concept `measure.money` or `measure.amount`, or content type `money`) |
| `x-qsv.relationships` | dataset level, `{"kind":"pipeline", …}` | draws the **pipeline** panel |
| `x-qsv.tour` | dataset level | replaces the guided Tour's built-in narration: `overrides` keyed by step id, `panels` keyed by RAW field name or `@kind` token, `panel_order` picks/orders the panel spotlights (cap 6). `version` MUST stay the **integer** `1` — viz silently discards the whole block on anything else. Plain text only; `language` (if present) is BCP-47 and must match the page locale or overrides are dropped. Written by `--tour-audience` (Stage 2), refined in Stage 6 |

(`x-qsv.aggregation` used to be a fifth row here; it is now edited with `a` in
the TUI above. Its meaning is unchanged: `sum` or `mean` on a numeric measure,
declaring how the column combines across a group and overriding qsv's
column-NAME heuristic in both directions. Use `mean` for anything per-unit or
per-record — a unit price, a rating, a temperature, a duration — and `sum` only
for a quantity each row contributes. It is the **language-neutral** signal: the
name heuristic is English-first and cannot read non-English column names, issue
#4401.)

For a pipeline, both encodings are hand-editable — stages as columns
(`"members"` in process order, **widest/upstream first**, the opposite direction
from `"kind":"ordered"`), or stages as row values (`"stage_column"` + an ordered
`"stages"` list + an optional `"value_column"` to sum). Declared order is
authoritative: if a stage outruns its predecessor, viz draws a **bridge** of
signed differences instead of a funnel, rather than a band wider than the one
above it. Offer these edits only when the Stage 2 check showed a plausible
candidate; do not invent a target.

## Stage 3 — GeoJSON (optional)

Ask with **AskUserQuestion**: *"Bin rows into GeoJSON regions?"*

If **no**, skip to Stage 4 with no geo flags.

If **yes**:

### 3a. Check the data can actually be binned

`viz smart`'s GeoJSON panel uses **point-in-polygon binning**: each row's
`--lat`/`--lon` is tested against the polygons. Without a coordinate pair there
is nothing to bin, and the flag will quietly produce no map panel.

```bash
qsv headers "$WORK" | grep -iE 'lat|lon|lng|y_|x_|coord'
```

If no plausible pair exists, tell the user the GeoJSON will have no effect and
offer to proceed without it. Do not pass `--geojson` into a dead end.

### 3b. Get the file

Accept a local path, an `http(s)` URL, or a shortcut name defined in
`QSV_GEOJSON_SHORTCUTS` (a JSON map of `name` → `{path, id}`; the shortcut's `id`
supplies `--feature-id-key` when you don't pass one).

### 3c. Discover the feature id key — do not guess it

`--feature-id-key` defaults to `id`, which is usually wrong. In `viz smart`'s
point-in-polygon mode the key **labels each binned region**, so it must be
present on every feature, unique across all of them, and *meaningful to a human*.
Uniqueness alone is not enough: `properties.shape_area` is perfectly unique and
completely useless as a label.

The script accepts the **same three source forms** `--geojson` does — a local path,
an `http(s)` URL, or a `QSV_GEOJSON_SHORTCUTS` name. If you only handle local
paths here, a URL or shortcut fails at discovery even though `viz` would have
accepted it.

```bash
python3 - "$GEOJSON" <<'PY'
import json, sys, re, os, collections, urllib.request

def load_geojson(src):
    """Local path, http(s) URL, or a QSV_GEOJSON_SHORTCUTS name.

    Mirror viz's resolution order (src/cmd/viz.rs resolve_and_validate_geojson): an
    http(s) URL or an EXISTING local file is a direct source; only a value that is
    neither is looked up as a shortcut NAME. This keeps a local file whose name
    collides with a shortcut loading as the file (as viz does), and it never lets
    a malformed QSV_GEOJSON_SHORTCUTS break a direct file/URL input.
    """
    hint = None
    is_url = src.startswith(("http://", "https://"))
    if not is_url and not os.path.isfile(src):
        raw = os.environ.get("QSV_GEOJSON_SHORTCUTS")
        if not raw:
            sys.exit(f"--geojson '{src}' is not an existing file or http(s) URL, "
                     "and QSV_GEOJSON_SHORTCUTS is not set")
        shortcuts = json.loads(raw)           # invalid JSON surfaces as an error
        if src not in shortcuts:
            sys.exit(f"unknown --geojson shortcut '{src}'; "
                     f"defined: {', '.join(sorted(shortcuts)) or '(none)'}")
        entry = shortcuts[src]
        hint = entry.get("id")                # shortcut may carry its own id key
        src = entry["path"]
        is_url = src.startswith(("http://", "https://"))
    if is_url:
        with urllib.request.urlopen(src, timeout=30) as r:
            return json.loads(r.read().decode("utf-8")), src, hint
    with open(src) as fh:
        return json.load(fh), src, hint

g, resolved, hint = load_geojson(sys.argv[1])
feats = g.get("features", [])
if not feats:
    sys.exit("no features")
print(f"source: {resolved}")
if hint:
    print(f"shortcut supplies --feature-id-key {hint} (override below if you prefer)")

# Geometry-derived / bookkeeping fields: unique, but meaningless as a region label.
NOISE = re.compile(r"shape|area|leng|length|perim|acres|sqmi|aland|awater|"
                   r"intptlat|intptlon|^lat|^lon|_x$|_y$|"
                   r"date|time|edited|created|updated|version", re.I)

def floatish(v):
    return isinstance(v, float) or (isinstance(v, str) and re.fullmatch(r"[+-]?\d+\.\d+", v.strip()))

cands = collections.defaultdict(list)
for f in feats:
    if f.get("id") is not None:
        cands["id"].append(f["id"])
    for k, v in (f.get("properties") or {}).items():
        if isinstance(v, (str, int, float)):
            cands[f"properties.{k}"].append(v)

good, other = [], []
for key, vals in cands.items():
    if len(vals) != len(feats):                    # missing on some feature
        continue
    if len(set(map(str, vals))) != len(feats):     # not unique
        continue
    demote = bool(NOISE.search(key)) or all(floatish(v) for v in vals)
    (other if demote else good).append((key, vals[:3]))

def show(title, rows):
    print(f"\n{title}")
    if not rows:
        print("  (none)")
    for key, sample in rows:
        print(f"  {key:<32} e.g. {sample}")

print(f"{len(feats)} features")
show("RECOMMENDED feature-id-key (unique, meaningful):", good)
show("Unique but geometry/bookkeeping - avoid:", other)
if not good and not other:
    print("\nNo property is unique across all features. This GeoJSON cannot key regions as-is.")
PY
```

Offer the **RECOMMENDED** keys via **AskUserQuestion**, favouring a short region
code or name (`properties.nta2020`, `properties.hood`) over a surrogate key
(`properties.OBJECTID`, a GUID) — the value is what the user reads on hover.
If nothing is unique, say so plainly: the GeoJSON cannot key regions as-is.

Optionally also pick `--feature-name-key` (e.g. `properties.name`) for
human-readable hover labels. When omitted, common name keys are auto-detected.

## Stage 4 — Render

Ask for `--dataset-pid` with **AskUserQuestion** (a persistent identifier — a DOI,
ARK, Handle, or a URL). It is optional; allow the user to skip it.

```bash
qsv viz smart "$WORK" \
  --smarter --bivariate \
  --dictionary "$SCHEMA" --dict-info \
  ${GEOJSON:+--geojson "$GEOJSON" --feature-id-key "$FEATURE_ID_KEY"} \
  ${DATASET_PID:+--dataset-pid "$DATASET_PID"} \
  -o "$OUT"
```

- `--smarter` runs `qsv moarstats --advanced` first, enriching the stats cache
  with distribution shape (bimodality, entropy, skewness, outlier share, Gini —
  the last unlocks Lorenz curves for the most unequal additive measures). Costs
  one extra pass and writes `<stem>.stats.csv` + sidecars + `.idx`. It applies
  only under default parsing: `--no-headers` or a custom `--delimiter` silently
  falls back to the standard Data Schematic.
- `--bivariate` adds a normalized-mutual-information heatmap plus — only when
  there are more than 8 chartable columns — a ranked "top relationships" bar.
  **It implicitly turns on `--dictionary infer` when `--dictionary` is not set**
  — so passing `$SCHEMA` explicitly is what stops viz from calling the LLM a
  *second* time. Never pass `--bivariate` without a dictionary in this workflow.
  Capped at 50 columns; wider datasets skip both panels with a warning.
- `--dict-info` embeds the dictionary in a side drawer next to the plots, adds an
  info icon per panel, and a "Data Dictionary" link under the title. The drawer
  also carries **download buttons for the sidecars this run actually read** —
  the schema, the charted frequency counts, the stats cache + metadata, and the
  bivariate CSV — all bundled *into* the HTML, so a recipient needs no access to
  your machine. Absolute local paths are stripped from the embedded metadata
  (sharing a Data Schematic does not disclose your directory layout); sidecars over
  4 MB are skipped with a note. **HTML only** — ignored with a note when
  exporting an image.
- `-o` must end in `.html`. An image extension (`.png`, `.svg`, …) silently
  switches viz to the static-export path, which needs a browser/webdriver and
  drops `--dict-info`.

### The data viewer drawer (`--preview-threshold`, default 50000)

Independent of `--dictionary`/`--dict-info`: an **(Explore)** link beside the row
count in the metadata table opens the underlying rows in a searchable bottom
drawer. Every row is embedded while the dataset has at most `<n>` rows; above
that only the first `<n>` are, and the link reads **(Preview)**.

This is the one flag here with a real cost: **embedded rows grow the HTML — and
the reader's browser memory — in proportion to rows × columns.** Tell the user
the size (Stage 5 prints it) rather than letting them discover it. Lower the
threshold, or pass `--preview-threshold 0` to drop the viewer entirely, when the
Data Schematic is meant to be emailed around.

### `--photos` — ask first, never enable silently

If a column holds image URLs, `--photos` makes dwelling on a map point reveal
that row's photo. It is **off by default and deliberately so**: images load from
whatever third-party host *the data* names, so every person who opens the
Data Schematic requests those URLs directly and reveals their IP to that host. Only
pass it if the user asks for it after being told that. HTML tile-map panel only.

## Stage 5 — Verify, then report

Never claim success without checking. `viz smart` prints what it skipped to
**stderr** — surface that to the user verbatim; it is the most useful line it
emits.

```bash
test -s "$OUT" || { echo "no Data Schematic written"; exit 1; }
python3 - "$OUT" <<'PY'
import re, sys
h = open(sys.argv[1], encoding="utf-8", errors="replace").read()
print(f"{len(h)/1e6:.1f} MB")
print("dictionary drawer embedded:", "qsv-dict-drawer" in h)
print("dictionary back-links:", h.count("View chart"))
m = re.search(r"Data — [^\"<]{0,60}", h)
print("data viewer:", m.group(0) if m else "disabled / not embedded")
print("guided tour config:", "qsv-tour-config" in h)
print("tour narration in dict drawer:", 'class="qsv-dict-tour"' in h)
PY
```

`View chart` counts the dictionary's back-links to panels, **not** the panels
themselves — viz emits one only where a matching panel element exists. Use the
stderr note for what was drawn and skipped; that is authoritative.

Then tell the user:

- which columns `denull` cleaned, and how many cells were blanked
- how many columns got a `role`/`concept` from the dictionary
- which columns `viz smart` **skipped, and why** (its stderr note names them)
- whether the KPI row, any gauge tile, and the pipeline panel rendered — and if
  a hint from Stage 2 was dropped, viz says why on stderr (a `gauge_range` whose
  range excludes the data, a pipeline naming a missing column)
- the data viewer's state: all rows (Explore) or a truncated preview, and what
  it costs in file size
- the GeoJSON coverage note, if any (points that fell outside every region)
- the path to `$OUT`

If the user can open a browser, offer to render it. Do not assert the Data Schematic
"looks right" — you cannot see it. **Unless you have a browser-automation MCP —
then Stage 6 lets you.**

## Stage 6 — Tour refinement (optional, browser)

The `x-qsv.tour` narration from Stage 2 was written blind: the LLM never saw
which panels `viz smart` actually drew. With a browser you can close that loop —
step through the Tour, judge each narration against what is really on screen,
and refine the schema. Skip this stage (and say so) when no browser-automation
MCP is available.

**Tool-agnostic:** use whatever browser-automation MCP is available — Playwright
MCP, claude-in-chrome, or any other. Every check below is specified by
selector/anchor, not by tool.

1. **Open** `file://$OUT` (or serve it locally if the tool requires http).
2. **Read the resolved tour** from the element `#qsv-tour-config` — its JSON
   payload lists the steps this page actually built: which step ids exist, the
   prose each carries, and for each panel step its **`key`** — the stable
   raw-field-name or `@kind` token that `x-qsv.tour.panels`/`panel_order`
   address. Use `key`, never the display `title` (titles are decorated:
   "activated (right-skewed)"). This is ground truth; the schema's
   `overrides`/`panels` only applied where a matching step/panel exists.
3. **Replay the tour.** It auto-runs only on first visit — click the **Tour**
   pill in the header, or remove every `localStorage` key starting with
   `qsv-viz-tour-seen-` (the key is suffixed with a page hash *and* the
   pathname, so clear by prefix) and reload.
4. **Step through and judge** each popover against three things: (a) the
   audience's register, (b) what is actually visible on that panel — narration
   must never reference a chart that wasn't drawn or numbers that aren't shown,
   and (c) the collapsed `<details class="qsv-dict-tour">` section on the
   dictionary page (open the drawers with the page's `qsvOpenDict` /
   `qsvOpenData` links as the tour does).
5. **Refine `$SCHEMA`** with an inline `python3` JSON merge — load, mutate ONLY
   `s["x-qsv"]["tour"]`, dump. Never `sed`/regex the file, never touch other
   keys. Unlike the Stage 2 LLM pass, you can now see which panels rendered, so
   you MAY also set `@kind`-token `panels` entries (`@kpi`, `@correlation`,
   `@timeseries`, `@map`, `@choropleth`, `@scatter`, …) and a `panel_order`
   array to pick and order the spotlights (viz caps them at 6). Keep `version`
   the integer `1`; keep `language` BCP-47 matching the page locale.
6. **Re-render and re-verify.** Stage 4 passes `--dictionary "$SCHEMA"` by
   path, so re-running it is cheap (no LLM call, no sidecar-reuse trap). Re-open
   the page and spot-check the changed steps. At most **two** refinement loops —
   then report what changed and stop.

## Guardrails

- **Never** run `denull --apply` with `-o` pointing at the input, and never with
  `-` (stdin). It refuses both, but don't rely on that.
- If `denull` confirms nothing, do **not** create a `.denulled.csv`. An empty
  transform step is noise.
- Never hand-write the JSON Schema. It comes from `describegpt`. `role`,
  `concept`, `title`, `description` and `x-qsv.aggregation` are adjusted only
  through the Stage 2.5 `edit_dictionary.py` TUI — never by editing the JSON by
  hand (an off-vocab `role`/`concept` typed into the raw file silently routes a
  column to the wrong panel; the TUI validates against the vocab and flags
  drift). The exceptions are the five keys the TUI does not own —
  `x-qsv.gauge_range`, `x-qsv.target`, `x-qsv.currency` and the dataset-level
  `x-qsv.relationships` and `x-qsv.tour` — which qsv documents as hand-edited.
  `x-qsv.tour` is freeform prose (nothing for a validator to check), but three
  of its fields are load-bearing: `version` must stay the integer `1`,
  `language` must stay BCP-47 matching the page locale, and prose is plain
  text only — never HTML or Markdown. When editing it, never touch any other
  key in the schema.
- The Stage 2.5 TUI is **out-of-band**: it needs the user's real terminal. Never
  try to launch it through your Bash tool and "drive" it — that shell is not a
  TTY, and the script will refuse. Print the command, wait, then re-read.
- `--dictionary infer` runs describegpt **without** `--infer-null-values`. If you
  want null sentinels in the dictionary, you must generate it yourself and pass
  the **path**.
- Statistics over cleaned columns are **complete-case**: they describe the rows
  that have a value. `denull` makes the missingness visible; it does not make it
  ignorable. Do not reach for `qsv stats --nulls` to "restore" the blanks — that
  imputes zero for the mean-family statistics while the median and quartiles keep
  ignoring them, so the summary stops agreeing with itself.

## Example

```
/visual-data-dictionary NMBGMRManualWaterLevels.csv
```

1. `denull` confirms 6 columns (`HoleDepth`, `WellDepth`, `CasingDiameter`,
   `CasingDepth`, `DepthToWaterBGS`, `DataQuality`), blanks 8,278 cells; all
   6 promote from `String` to `Integer`/`Float`.
2. `describegpt` writes `NMBGMRManualWaterLevels.denulled.schema.json` with
   `role`/`concept` on 25/25 columns.
3. User declines GeoJSON (the file has UTM `Easting`/`Northing`, not lat/lon).
4. `viz smart --smarter --bivariate --dict-info` writes
   `NMBGMRManualWaterLevels.html`, charting the numeric columns and skipping
   `_id` / `PointID` (identifiers) and the date columns (which feed the
   time-series panel instead). Report the counts viz actually prints on stderr —
   panel selection moves with each release, so never quote a remembered number.

Before cleaning, `viz smart` skipped 11 columns and warned that 5 of them looked
like numeric data held back by a literal `NULL`. That warning is the reason
Stage 1 exists.

A GeoJSON run reports its binning coverage on stderr — pass it on verbatim:

```
viz smart: 54 of 409 points were snapped to the nearest region
           (cap 0.24 km, auto-derived from region size and coordinate precision)
```

`denull` finding nothing is a normal outcome, not a failure. Say so and move on.
