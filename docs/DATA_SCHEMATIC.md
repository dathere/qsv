# The Data Schematic

A **Data Schematic** is a single, self-contained rendering of a dataset's schema and
statistics in which every claim shown is checkable against the data it describes.

This document defines the format. It is deliberately tool-neutral: `qsv viz smart`
produces one, but a Data Schematic is not a qsv artifact any more than a histogram is a
matplotlib artifact. The last section describes qsv's implementation.

The key words MUST, MUST NOT, SHOULD and MAY are used as in RFC 2119.

---

## Why not "data dictionary"

A dictionary is alphabetical, atomized and unordered. You look up one entry at a time,
and the relationships between entries are invisible — which is a fair description of a
field list in a PDF appendix, and a poor description of what a dataset actually is.

Datasets have structure that lives *between* fields: measures that correlate, stages that
follow one another in a process, dimensions that nest into a hierarchy, a date column that
paces a numeric one, a latitude that is meaningless without its longitude. A dictionary has
nowhere to put any of it.

A schematic does. A schematic is defined by the thing a dictionary lacks — it shows
components *and how they connect*, drawn to a stated convention, so that a stranger can
reconstruct the system and check the drawing against the thing it depicts. A wrong
schematic is a stronger claim than a misleading chart: a circuit diagram with a backwards
diode is not "potentially confusing," it is false, and anyone with the board in front of
them can prove it.

That falsifiability is the point. Anyone can produce a plausible data dictionary, and no
reader can check it without going to the data. A Data Schematic asserts structure that the
file itself can refute.

---

## Conformance

### MUST

1. **Everything shown is derived.** Every number, panel and label traces to a computation
   over the dataset being described. A figure with no computation behind it MUST NOT
   appear, however plausible it is.

2. **The field inventory is complete.** Every column in the source is accounted for.
   Columns that are not drawn MUST be listed with the reason they were omitted
   (identifier-like, all-empty, beyond a stated cap), so that absence is a statement
   rather than a gap.

3. **Structure is shown, not just fields.** Where relationships between fields exist —
   correlation, process order, hierarchy, temporal pacing, spatial pairing — the schematic
   MUST show them. Where they are absent, it MUST say so rather than leaving the reader to
   infer that none were looked for.

4. **Declared semantics are declared, never guessed.** Facts that no statistic can settle
   — which columns are process stages and in what direction, what a measure's target is,
   what canonical scale a ratio lives on — MUST come from an explicit declaration in the
   schema, and MUST be attributed to that declaration where they are displayed. A
   schematic MUST NOT infer them from column names or ordering.

5. **Form follows the data.** An encoding MUST NOT assert a relation the data contradicts.
   Where the chosen form carries a claim (a funnel's band widths claim containment; a
   sankey's ribbons claim flow), the schematic MUST verify the claim holds and fall back
   to a form that does not make it when the claim fails — stating which form was used and
   why.

6. **Denominators are disclosed.** Any aggregate computed over a population other than the
   naive one (all rows, all non-null values) MUST state the population it used at the point
   of display.

7. **It is self-contained.** A schematic MUST render from the file alone, with no network
   access to its source system, its producing tool, or a live service. Optional enrichment
   that requires the network MUST degrade rather than fail.

8. **It is checkable.** A schematic MUST carry, or resolvably reference, the artifacts
   needed to re-derive it: the schema it was built from, the computed statistics, and the
   frequency counts it charted — together with the tool and version that produced them.

9. **Provenance is stated.** The producing tool and version, the identity of the source
   dataset, and the time of generation MUST appear in the schematic itself.

10. **Limits are disclosed in place.** Sampling, capping, truncation and approximation MUST
    be disclosed on the panel affected, not only in a global note. A reader who looks at one
    panel MUST be able to tell whether they are looking at all of the data.

### SHOULD

- A persistent identifier (DOI or other citable URL) for the source dataset.
- A human-readable rendering of the schema alongside the visuals, cross-linked in both
  directions, so the schematic doubles as the dictionary it supersedes.
- A static export path, for archival and for readers without a modern browser.
- Localization of the schematic's own UI, independent of the language of the data.
- Encodings that survive greyscale printing and carry text alternatives.

### MAY

- Interactive exploration of the underlying rows.
- Geospatial enrichment (reverse geocoding, boundary overlays, extent summaries).
- **Model-inferred semantics** — labels, roles, concepts and descriptions proposed by a
  language model. Permitted for *meaning* only, never for *values*: a model MAY propose
  that a column is a measure of currency, and MUST NOT propose what it sums to. Inferred
  semantics MUST be marked as inferred, and MUST be editable so a human can correct them
  and re-render.

### Non-conforming

A rendering is **not** a Data Schematic if any of the following hold. These are the failure
modes the format exists to exclude, so they are listed as flatly as possible:

- A displayed number was authored rather than computed.
- A narrative claim appears that no displayed computation supports.
- An encoding asserts a relation the data contradicts.
- Sampling, capping or truncation happened silently.
- Viewing it requires a live service, a login, or the producing tool.
- A baseline, target or prior period was fabricated to make a delta computable.

---

## Conformance levels

Three levels, each a superset of the last. Most published artifacts will sit at Level 1;
Level 3 is for schematics that must survive being forwarded to someone with no access to
the producer.

| Level | Name | Requires | Claim |
|---|---|---|---|
| 1 | Derived | MUST 1, 2, 5, 6, 7, 10 | Everything shown was computed, and its limits are visible. |
| 2 | Structured | + MUST 3, 4 | Relationships are shown, and declared semantics are attributed. |
| 3 | Attested | + MUST 8, 9 with verifiable attestation | A third party can re-derive it and compare. |

Level 3 requires that the bundled inputs carry integrity digests and that the attestation
identifies the computation, not merely the output — so that "I re-ran this and got
something else" is a well-formed statement.

---

## Relationship to a data dictionary

A data dictionary is the **degenerate case**: a Data Schematic with the relationships and
the derivations stripped out. Every existing data dictionary is therefore a valid, if
impoverished, Level 1 schematic, and no publisher has to discard one to adopt this.

The upgrade path is additive:

| Have | Add | Get |
|---|---|---|
| Field list with descriptions | Computed statistics per field | Level 1 |
| The above | Declared relationships in the schema | Level 2 |
| The above | Bundled inputs + attestation | Level 3 |

---

## Relationship to existing standards

A Data Schematic is a **rendering profile**, not a competing schema language. It carries a
schema; it does not replace one.

- **JSON Schema** — the natural carrier for the field inventory, roles and declarations.
  Declarations that JSON Schema has no vocabulary for (process pipelines, targets, canonical
  scales) belong in a namespaced extension keyword rather than in prose.
- **Frictionless Table Schema** — an equally valid carrier at Level 1; lacks a relationship
  vocabulary, so Level 2 needs an extension.
- **DCAT / DCAT-US** — describes a dataset's *catalog* metadata. A schematic describes its
  *internal* structure. They compose: a DCAT distribution can point at a schematic, and a
  schematic can carry the dataset's PID.

---

## A known limitation

Schematics conventionally describe systems — things with flow, ordering and causation.
Many datasets are not systems. A one-table survey extract with independent columns and no
declared relationships yields a schematic that is, honestly, a data dictionary with better
typography.

This is worth stating rather than papering over, for two reasons. It is the correct output
for that input; and the emptiness is itself a finding. A schematic that shows no structure
is telling you either that the dataset has none, or that its publisher declared none — and
those are different problems with the same appearance, which is exactly the kind of thing a
reader should be able to see at a glance.

---

## qsv's implementation

`qsv viz smart` produces a **qsv Schematic**. `viz smart` is the command; the schematic is
the artifact.

| Requirement | Mechanism |
|---|---|
| 1 — Derived | Panels are selected and populated from the stats and frequency caches; no panel is model-authored. |
| 2 — Complete inventory | Skipped columns (identifier-like, all-empty, redundant twins, beyond `--max-charts`) are named in a stderr note. Per-column *reasons* are given only for the null-sentinel and non-numeric-measure diagnostics; the general skip list names columns without attributing a reason to each. |
| 3 — Structure | Correlation heatmap, NMI panels under `--bivariate`, hierarchy panels, time-series pairing, lat/lon map. |
| 4 — Declared, not guessed | The pipeline panel is drawn *only* from an `x-qsv.relationships` declaration; `target` is never inferred. |
| 5 — Form follows data | A declared pipeline renders as a funnel only while the stage **totals never increase**; otherwise as a bridge, with the reason in the subtitle. Row-wise containment is measured and disclosed separately in the subtitle, but does not decide the form — the two can disagree in either direction. |
| 6 — Denominators | Pipeline totals sum over rows complete across all declared stages; the subtitle states that basis. |
| 7 — Self-contained | plotly.js is embedded by default (`QSV_VIZ_CDN` opts out). Continental/global extents and all static exports use an offline ScatterGeo projection; a *local*-extent panel uses a MapLibre tile basemap, which needs network at view time. The reverse-geocode overlay degrades to absent when offline. |
| 8 — Checkable | `--dict-info` bundles the schema, the charted frequency counts and the consumed stats sidecars into the HTML, each under a 4 MB cap. The human-readable `<stem>.stats.csv` is deliberately not offered, since viz never reads it. |
| 9 — Provenance | Header metadata table: `Generated by: qsv <version>` and `Compiled:` always; `--dataset-pid` adds a citable identifier. Local paths are redacted, so the table carries no dataset filename. |
| 10 — Limits in place | Sampled violins are titled "(sampled)"; snap/drop coverage is noted beneath the map. |

Level reached: **2 by default.**

**Level 3 is not reached today.** `--dict-info` satisfies the *bundling* half of MUST 8 —
the schema, the charted frequency counts and the consumed stats sidecars all travel inside
the HTML — but the bundled inputs carry **no integrity digests**, which MUST 8's attestation
requires. Digests are not implemented: the embedded sidecars are base64 payloads with no
hash alongside them. (The `sha384` values in qsv's output are Subresource Integrity for
CDN-loaded third-party JavaScript — unrelated to the bundle.) Adding per-input digests plus
an attestation that identifies the *computation* rather than merely the output is the
remaining work for Level 3.

---

*This document was produced using Claude, an AI assistant by Anthropic. Content should be reviewed for accuracy.*
