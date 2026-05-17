# synthesize

> Generate a synthetic CSV that is statistically faithful to a source CSV. Runs `stats` + `frequency` on the source so synthesized columns reproduce its per-column attributes — frequency-weighted sampling for categorical columns, quartile-bucketed numeric/date generation, null-ratio preservation. With a Data Dictionary from `describegpt --dictionary --infer-content-type`, semantic Content Types pick realistic [fake-rs](https://github.com/cksac/fake-rs) fakers (names, emails, addresses, UUIDs, etc.) for non-enumerable columns. Fully reproducible with `--seed`.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/synthesize/mod.rs](https://github.com/dathere/qsv/blob/master/src/cmd/synthesize/mod.rs)** | [📇](TableOfContents.md#legend "uses an index when available.")🎲[🤖](TableOfContents.md#legend "command uses Natural Language Processing or Generative AI.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Synthesize Options](#synthesize-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Generates a synthetic CSV that is statistically faithful to a source CSV.

`synthesize` analyzes <input> with `stats` and `frequency`, then emits N rows of
fake data that reproduce the source's per-column attributes:  

* Categorical / low-cardinality columns are reproduced by frequency-weighted
sampling of their *real* value set — cardinality, weights and repetition
structure are preserved exactly.
* Numeric and date/datetime columns are reproduced with quartile buckets, so
the shape of the distribution (not just its [min,max] range) is preserved.
* Null ratios are reproduced per column.

When a Data Dictionary is supplied (via --dictionary, or generated on the fly
with --infer-content-type), each column's semantic Content Type picks a
realistic faker (names, emails, addresses, UUIDs, etc.) for columns that are
NOT fully enumerated by `frequency`. For bounded-cardinality faker columns
(cardinality < requested rows and below an internal cap of 100,000), a fixed
pool of distinct fake values is pre-generated and sampled from, so the column's
cardinality is preserved. For very high cardinality columns above this cap, a
fresh fake value is generated per row instead — distinct count is approximate
in that case.

When `stats` provides string-length statistics (min_length / max_length /
avg_length / stddev_length) AND the column is routed to an unstructured text
generator (lorem_*, free_text, or the no-faker fallback), synthesized values
are truncated so their character lengths follow Normal(avg_length,
stddev_length) clamped to [min_length, max_length]. This applies to unstructured
pooled values as well — a low-cardinality free-text column still gets its
generated pool entries truncated. Structured semantic fakers (email, name,
uuid, phone, address parts, etc.) ignore these stats — truncating them would
corrupt their format, so their pools are reproduced verbatim. Frequency-
enumerated values are always reproduced verbatim and are never truncated.

Columns are generated independently — cross-column correlation is not modeled.

With --seed, output is fully reproducible.


<a name="examples"></a>

## Examples [↩](#nav)

> Pure statistical synthesis — no dictionary needed

```console
qsv synthesize data.csv -n 1000 --seed 42 > synthetic.csv
```

> First, generate the Data Dictionary with describegpt

```console
qsv describegpt data.csv --dictionary --infer-content-type --format JSON -o dict.json
```

> Then layer in semantic fakers from the dictionary

```console
qsv synthesize data.csv --dictionary dict.json -n 1000 > synthetic.csv
```

> Let synthesize build the dictionary itself (needs an LLM API key)

```console
qsv synthesize data.csv --infer-content-type -n 1000 > synthetic.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_synthesize.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv synthesize [options] <input>
qsv synthesize --help
```

<a name="synthesize-options"></a>

## Synthesize Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑dictionary`&nbsp; | string | Data Dictionary JSON file produced by `describegpt --dictionary --infer-content-type --format JSON`. Layers semantic Content Types onto generation. If omitted, generation is purely type/frequency-based. |  |
| &nbsp;`‑‑infer‑content‑type`&nbsp; | flag | Generate the Data Dictionary on the fly by invoking `describegpt --dictionary --infer-content-type` on <input>. Requires an LLM API key (QSV_LLM_APIKEY). Ignored if --dictionary is given. |  |
| &nbsp;`‑n,`<br>`‑‑rows`&nbsp; | integer | Number of synthetic rows to generate. | `100` |
| &nbsp;`‑‑seed`&nbsp; | integer | RNG seed for fully reproducible output. |  |
| &nbsp;`‑‑locale`&nbsp; | string | Locale for faker-backed columns. Case-insensitive. Supported: en, fr_fr, de_de, it_it, pt_br, pt_pt, ja_jp, zh_cn, zh_tw, ar_sa, cy_gb, fa_ir, nl_nl, tr_tr. Sparse locales (those without per-category data in fake-rs) silently fall back to en data for the missing categories — e.g. lorem text under a non-en locale is still English, since only zh_cn has localized lorem data. | `en` |
| &nbsp;`‑‑freq‑limit`&nbsp; | integer | Frequency pool depth passed to the internal `frequency` run as --limit. A column is reproduced via exact frequency-weighted sampling only when its cardinality is fully captured within this limit; higher values reproduce more columns verbatim. 0 means unlimited. | `100` |
| &nbsp;`‑‑stats‑options`&nbsp; | string | Extra options appended to the internal `stats` run. Note: cardinality, quartiles and date inference are always enabled — do not re-specify them here. |  |
| &nbsp;`‑‑consistent‑fakes`&nbsp; | flag | For structured-faker columns with bounded cardinality (cardinality fully captured by `frequency`), build a stable source-value -> fake-value mapping so the same source value always produces the same fake in the output. Preserves the source frequency distribution and overrides the default "emit real values when frequency-enumerated" behavior for structured fakers (names, emails, addresses, etc.). Has no effect on unstructured columns (lorem_*, free_text, unknown), all-unique columns, or non-faker columns. Useful for deidentified synthesis where you want stable joins on the faked columns. |  |
| &nbsp;`‑j,`<br>`‑‑jobs`&nbsp; | integer | Number of jobs to use for the internal `stats` and `frequency` runs. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`‑d,`<br>`‑‑delimiter`&nbsp; | string | The field delimiter for reading the input CSV. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/synthesize/mod.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/synthesize/mod.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
