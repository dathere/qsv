# synthesize

> Generate a synthetic CSV that is statistically faithful to a source CSV. Runs `stats` + `frequency` on the source so synthesized columns reproduce its per-column attributes — frequency-weighted sampling for categorical columns, quartile-bucketed numeric/date generation, null-ratio preservation. With a Data Dictionary from `describegpt --dictionary --infer-content-type`, semantic Content Types pick realistic [fake-rs](https://github.com/cksac/fake-rs) fakers (names, emails, addresses, UUIDs, etc.) for non-enumerable columns. A dictionary `relationships` array preserves inter-column structure within each row — `joint` (functional dependencies like city/state/zip), `ordered` (monotonic chains like created_date ≤ closed_date) and `correlated` (numeric correlation via a Gaussian copula). Fully reproducible with `--seed`. Output is not anonymized: it can contain real source values, though by default values and `joint` tuples seen fewer than 5 times are dropped from the real values it samples, and numeric/date range ends are trimmed so a lone outlier can't set them (`--min-count`). `--report` writes a JSON summary of how many synthetic rows exactly match a source row (numbers and dates compared by value) and which values and `joint` tuples `--min-count` dropped.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/synthesize/mod.rs](https://github.com/dathere/qsv/blob/master/src/cmd/synthesize/mod.rs)** | [📇](TableOfContents.md#legend "uses an index when available.")[🎲](TableOfContents.md#legend "randomly generated or randomized output with a --seed option for reproducibility.")[🤖](TableOfContents.md#legend "command uses Natural Language Processing or Generative AI.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Synthesize Options](#synthesize-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Generates a synthetic CSV that is statistically faithful to a source CSV.

`synthesize` analyzes <input> with `stats` and `frequency`, then emits N rows of
fake data that reproduce the source's per-column attributes:  

* Categorical / low-cardinality columns are reproduced by frequency-weighted
sampling of their *real* value set — cardinality, weights and repetition
structure are preserved, minus values rarer than the --min-count.
* Numeric and date/datetime columns are reproduced with quartile buckets, so
the shape of the distribution (not just its [min,max] range) is preserved.
The range ends are trimmed by the --min-count (see below).
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

When the Data Dictionary declares `relationships`, the named columns are
generated *jointly* so inter-column structure survives into each output row:  

* joint      — categorical / functional-dependency groups (e.g.
city/state/zip). Whole value-tuples are sampled from the
source by frequency, so only real co-occurring combinations
are emitted.
* ordered    — columns that must keep a monotonic order within a row (e.g.
created_date <= closed_date). The anchor column is generated
from its own distribution; each later column is the anchor
plus a non-negative gap drawn from the gap distribution
learned from the source.
* correlated — numeric columns whose correlation should be preserved. A
Gaussian copula couples the columns while leaving each
column's own distribution unchanged.

Relationships are read from the dictionary's `relationships` array — inferred
by `describegpt` or hand-authored. Columns not named by any relationship are
still generated independently. Pass --no-relationships to disable relationship
modeling entirely.

Privacy note: synthetic output is not anonymized. It can contain real values
and real value combinations copied from <input>. By default, values and
`joint` tuples seen fewer than 5 times in <input> are dropped from the real
values that columns and joint groups sample from (see the --min-count
option). The count is of rows, not people, so in data with many rows per
person one person's value can still pass it. That limits copying but does
not stop it:  

* A column fully enumerated by `frequency` (cardinality within
the --freq-limit) is sampled from its real values, so any value at or
above the --min-count can be emitted verbatim. Setting the --freq-limit
to 0 removes that cap, so any column that is not all-unique can be
reproduced from its real values. Setting the --min-count to 1 lets
values that occur only once through as well.
* With a Data Dictionary, enumerated name, email, address and other
structured-faker columns still emit their real values unless you set
the --consistent-fakes option. That option only covers columns with up
to 100,000 distinct values; larger enumerated columns stay real.
* A `joint` relationship emits whole real value-tuples (e.g. a
city/zip/age combination) seen at least --min-count times.
* Numeric and date ranges are trimmed so that at least --min-count rows
lie at or beyond each end (the k-th smallest and k-th largest values),
so a lone outlier no longer sets the range. The ends are themselves real
values, and values inside the range are drawn uniformly per quartile, so
integers and dates can still land exactly on real values. With
the --min-count at 1 the range is the real min and max, and an outlier
can reappear at or near its real value.

Review the output before sharing it, or training on it, as de-identified data.

With --seed, output is fully reproducible, whatever the --jobs setting.
Rows are generated in parallel, in fixed blocks of 8192 rows. With --seed
and --shard, several machines can each generate one slice of a dataset.


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

> Preserve inter-column relationships declared in the dictionary
> (e.g. city/state/zip, created_date <= closed_date)

```console
qsv synthesize data.csv --dictionary dict.json -n 1000 > synthetic.csv
```

> Generate 1B rows as 4 slices, e.g. one per machine, then join them
> into the same file a single run would write

```console
qsv synthesize data.csv -n 1000000000 --seed 42 --shard 1/4 -o part1.csv
```

```console
qsv synthesize data.csv -n 1000000000 --seed 42 --shard 2/4 -o part2.csv
```

...
```console
qsv cat rows part1.csv part2.csv part3.csv part4.csv -o synthetic.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_synthesize.rs).

See also <https://github.com/dathere/qsv/wiki/AI-and-Documentation#synthesize>

<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv synthesize [options] <input>
qsv synthesize --help
```

<a name="synthesize-options"></a>

## Synthesize Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑dictionary`&nbsp; | string | Data Dictionary JSON file produced by `describegpt --dictionary --infer-content-type --format JSON`. Layers semantic Content Types onto generation. If omitted, generation is purely type/frequency-based. |  |
| &nbsp;`‑‑infer‑content‑type`&nbsp; | flag | Generate the Data Dictionary on the fly by invoking `describegpt --dictionary --infer-content-type` on <input>. Requires an LLM API key (QSV_LLM_APIKEY). Ignored if --dictionary is given. |  |
| &nbsp;`‑n,`<br>`‑‑rows`&nbsp; | integer | Number of synthetic rows to generate. | `100` |
| &nbsp;`‑‑seed`&nbsp; | integer | RNG seed for fully reproducible output. The same seed gives the same rows at any --jobs, unless stats uses approximate estimators: either asked for via --stats-options (e.g. `--quantile-method approx`), or switched on by stats itself (with a warning) when the input won't fit in memory. |  |
| &nbsp;`‑‑shard`&nbsp; | string | Generate only slice k of N (k from 1 to N) of the rows --rows asks for. Requires --seed. Each slice is a run of whole 8192-row blocks, and slices 1 to N joined in order (with `qsv cat rows`) are the same as one unsharded run. Every slice must use the same <input>, options, qsv version and dictionary file, if any (--infer-content-type without --dictionary is refused). Each slice still profiles all of <input> itself, so if stats or frequency warns on one machine that it switched to approximate estimators for lack of memory, that slice comes from a different fit and the joined rows won't match one run. Each slice gets a header row even when it has no rows (when N exceeds the number of blocks). With --report, the counts are for this slice only. |  |
| &nbsp;`‑‑locale`&nbsp; | string | Locale for faker-backed columns. Case-insensitive. Supported: en, fr_fr, de_de, it_it, pt_br, pt_pt, ja_jp, zh_cn, zh_tw, ar_sa, cy_gb, fa_ir, nl_nl, tr_tr. Sparse locales (those without per-category data in fake-rs) silently fall back to en data for the missing categories — e.g. lorem text under a non-en locale is still English, since only zh_cn has localized lorem data. | `en` |
| &nbsp;`‑‑freq‑limit`&nbsp; | integer | Frequency pool depth passed to the internal `frequency` run as --limit. A column is reproduced via exact frequency-weighted sampling only when its cardinality is fully captured within this limit; higher values reproduce more columns verbatim. 0 means unlimited. | `100` |
| &nbsp;`‑‑min‑count`&nbsp; | integer | Privacy threshold. A real value of an enumerated column, or a real `joint` value-tuple, that occurs fewer than <k> times in <input> is dropped from the values it samples; its share is redistributed over the kept values. An enumerated column whose values are all below <k> falls back to its faker or type-based generator (numeric and date columns to their trimmed range), and a joint group to independent columns, or aborts under the --strict-relationships option. A warning is printed when either happens. 1 disables suppression. Counts are of rows, not people: where one person has many rows (transactions, visits, logs), one person's value can pass <k> alone. Raise <k>, or dedupe to one row per person first. Numeric and date ranges, and the gaps of `ordered` relationships, are also trimmed to their k-th smallest and k-th largest values (one extra pass over <input>). A numeric or date column with fewer than 2k-1 non-null values is left empty unless its real values are sampled. | `5` |
| &nbsp;`‑‑report`&nbsp; | string | Write a JSON report of what the run reproduced from <input>: source and synthetic row counts, how many synthetic rows exactly match a source row (numbers and dates compared by value, other columns as text), and the values and joint tuples dropped by the --min-count option. Takes one extra pass over <input> and holds a hash per distinct source row in memory. A row with a float, free-text or faker column rarely matches exactly, so a low match rate does not by itself show that the output is private. A joint group that falls back to independent columns is listed, and so is each of its columns that also drops values, so suppressed rows can be counted more than once. Must not be the input, dictionary or output file. |  |
| &nbsp;`‑‑stats‑options`&nbsp; | string | Extra options appended to the internal `stats` run. Note: cardinality, quartiles and date inference are always enabled — do not re-specify them here. Column selection (--select/-s) and --weight are rejected; run `qsv select` on <input> first. |  |
| &nbsp;`‑‑consistent‑fakes`&nbsp; | flag | For structured-faker columns with bounded cardinality (cardinality fully captured by `frequency`), build a stable source-value -> fake-value mapping so the same source value always produces the same fake in the output. Preserves the source frequency distribution and overrides the default "emit real values when frequency-enumerated" behavior for structured fakers (names, emails, addresses, etc.). Has no effect on unstructured columns (lorem_*, free_text, unknown), all-unique columns, or non-faker columns. Useful for deidentified synthesis where you want stable joins on the faked columns. |  |
| &nbsp;`‑‑no‑relationships`&nbsp; | flag | Disable inter-column relationship modeling. Every column is generated independently even when the dictionary declares a `relationships` array. |  |
| &nbsp;`‑‑joint‑cardinality‑cap`&nbsp; | integer | Maximum number of distinct value-tuples a `joint` relationship may have. A joint group above this cap falls back to independent generation (or aborts under --strict-relationships). 0 means unlimited. | `100000` |
| &nbsp;`‑‑correlation‑threshold`&nbsp; | float | Minimum absolute Spearman correlation for a pair of columns to stay in a `correlated` relationship. Weakly-correlated members are dropped. | `0.3` |
| &nbsp;`‑‑strict‑relationships`&nbsp; | flag | Abort instead of warning-and-degrading when a declared relationship fails validation. |  |
| &nbsp;`‑j,`<br>`‑‑jobs`&nbsp; | integer | Number of jobs to use for the internal `stats` and `frequency` runs and for generating rows. |  |

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
