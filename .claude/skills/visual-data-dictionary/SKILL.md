---
name: visual-data-dictionary
description: Build a Visual Data Dictionary — an interactive qsv viz smart dashboard driven by an LLM-inferred JSON Schema data dictionary, with the dictionary browsable beside the charts. Use when the user asks for a visual data dictionary, a documented dashboard, a dictionary-driven dashboard, or wants to explore and document a CSV at the same time. Optionally bins rows into GeoJSON regions.
argument-hint: "<input.csv> [geojson]"
---

# /visual-data-dictionary

Turn a CSV into a self-contained HTML dashboard whose panels are chosen from an
LLM-inferred data dictionary, with that dictionary embedded beside the charts.

Four stages, in this order and no other:

1. **denull** — blank null sentinels so numeric columns are actually numeric
2. **describegpt** — infer a JSON Schema data dictionary from the *cleaned* data
3. **geojson** (optional) — pick a feature id key by inspecting the file
4. **viz smart** — render the dashboard, dictionary-driven, dictionary-embedded

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

If a local server answers, list its models and use one:

```bash
curl -s http://localhost:1234/v1/models | python3 -c 'import sys,json;[print(m["id"]) for m in json.load(sys.stdin)["data"]]'
```

If nothing is found, use **AskUserQuestion** for base URL + model. Never guess a
model name.

### Generate

```bash
qsv describegpt "$WORK" \
  --dictionary --description --two-pass --infer-content-type \
  --format JSONSchema \
  --base-url "$BASE_URL" --model "$MODEL" \
  -o "$SCHEMA"
```

- `--infer-content-type` is **mandatory here**, not optional: `viz smart` routes
  panels off each field's `role` and `concept`, and those are only inferred under
  this flag. Without it the dictionary loads and changes nothing.
- `--two-pass` roughly doubles cost and latency. It is what lets the model relate
  fields to one another (`street_no` + `street` + `city` + `zip` = one address),
  which is what makes the routing good.
- Naming it `<WORK stem>.schema.json` means a later
  `qsv viz smart "$WORK" --dictionary infer` finds and **reuses** it instead of
  paying for the LLM again. Delete the file to force a re-infer.

Optionally add `--infer-null-values` to have the model propose null sentinels
into each property's `x-qsv` object, split into `null_values` (confirmed present
by qsv) and `null_candidates` (guesses, each stamped `confirm_required: true`).
This is the only route to numeric sentinels like `-999`. It is **reported, never
applied** — nothing downstream acts on it.

Verify the dictionary carries what `viz` needs before spending time on Stage 4:

```bash
python3 - "$SCHEMA" <<'PY'
import json, sys
p = json.load(open(sys.argv[1]))["properties"]
have = sum(1 for v in p.values() if v.get("x-qsv", {}).get("role"))
print(f"role/concept on {have}/{len(p)} columns")
if have == 0:
    print("WARNING: no roles inferred — was --infer-content-type passed?")
PY
```

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

```bash
python3 - "$GEOJSON" <<'PY'
import json, sys, re, collections

g = json.load(open(sys.argv[1]))
feats = g.get("features", [])
if not feats:
    sys.exit("no features")

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
  with distribution shape (bimodality, entropy, skewness, outlier share). Costs
  one extra pass and writes `<stem>.stats.csv` + sidecars + `.idx`.
- `--bivariate` adds a normalized-mutual-information heatmap plus a ranked
  "top relationships" bar. **It implicitly turns on `--dictionary infer` when
  `--dictionary` is not set** — so passing `$SCHEMA` explicitly is what stops viz
  from calling the LLM a *second* time. Never pass `--bivariate` without a
  dictionary in this workflow.
- `--dict-info` embeds the dictionary in a side drawer next to the plots, adds an
  info icon per panel, and a "Data Dictionary" link under the title. **HTML only**
  — it is ignored with a note when exporting an image.
- `-o` must end in `.html`. An image extension (`.png`, `.svg`, …) silently
  switches viz to the static-export path, which needs a browser/webdriver and
  drops `--dict-info`.

## Stage 5 — Verify, then report

Never claim success without checking. `viz smart` prints what it skipped to
**stderr** — surface that to the user verbatim; it is the most useful line it
emits.

```bash
test -s "$OUT" || { echo "no dashboard written"; exit 1; }
python3 - "$OUT" <<'PY'
import sys
h = open(sys.argv[1], encoding="utf-8", errors="replace").read()
print(f"{len(h)/1e6:.1f} MB")
print("dictionary drawer embedded:", "dict-drawer" in h)
print("per-panel info icons:", h.count("View chart"), "entries")
PY
```

Then tell the user:

- which columns `denull` cleaned, and how many cells were blanked
- how many columns got a `role`/`concept` from the dictionary
- which columns `viz smart` **skipped, and why** (its stderr note names them)
- the GeoJSON coverage note, if any (points that fell outside every region)
- the path to `$OUT`

If the user can open a browser, offer to render it. Do not assert the dashboard
"looks right" — you cannot see it.

## Guardrails

- **Never** run `denull --apply` with `-o` pointing at the input, and never with
  `-` (stdin). It refuses both, but don't rely on that.
- If `denull` confirms nothing, do **not** create a `.denulled.csv`. An empty
  transform step is noise.
- Never hand-write the JSON Schema. It comes from `describegpt` or not at all.
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
   `NMBGMRManualWaterLevels.html`, charting 31 panels and skipping 4 columns
   (`_id`, `PointID` — identifiers; `CompletionDate`, `DateMeasured` — dates).

Before cleaning, `viz smart` skipped 11 columns and warned that 5 of them looked
like numeric data held back by a literal `NULL`. That warning is the reason
Stage 1 exists.

A GeoJSON run reports its binning coverage on stderr — pass it on verbatim:

```
viz smart: 54 of 409 points were snapped to the nearest region
           (cap 0.24 km, auto-derived from region size and coordinate precision)
```

`denull` finding nothing is a normal outcome, not a failure. Say so and move on.
