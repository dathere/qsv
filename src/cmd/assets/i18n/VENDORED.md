# Vendored localization assets for `qsv viz`

Third-party i18n files embedded into the `qsv` binary via `include_str!` and emitted inline in
generated dashboards. Both are MIT licensed. Nothing here is fetched at runtime — a generated
dashboard stays fully self-contained and offline-capable, which is why these are vendored rather
than referenced from a CDN.

Retrieved **2026-07-30**.

## Version pinning — these MUST track the library pins

Each file is version-matched to the library it localizes. **When either pin below is bumped,
re-download the matching locale files in the same commit**, or the strings can drift out of sync
with the library that consumes them:

| Asset | Pinned to | Where the pin lives |
|---|---|---|
| `datatables/*.json` | DataTables combo `dt-3.0.1/b-4.0.1/cc-2.0.0/date-2.0.0/sb-2.0.0` | `DATATABLES_CDN_COMBO`, `src/cmd/viz.rs` |
| `plotly/plotly-locale-*.js` | plotly.js **3.7.0** | `PLOTLY_CDN_VERSION`, `src/cmd/viz.rs` (and the bundle shipped by the `plotly` crate) |

A bump does not always mean a re-download — it means a re-*check*. On **2026-08-02**, when the
DataTables pin moved from `dt-3.0.0/b-4.0.0` to `dt-3.0.1/b-4.0.1`, all seven `datatables/*.json`
files were compared by sha256 against `https://cdn.datatables.net/plug-ins/3.0.1/i18n/<code>.json`
and every one was **byte-identical** to the 3.0.0 plug-in locale already vendored here. The files
below are therefore unchanged since the 2026-07-30 retrieval; only their documented source URLs
moved to the new version path.

The plotly version was confirmed from the **header banner** of the crate's bundled
`plotly/resource/plotly.min.js` (`/** * plotly.js v3.7.0 ... */`), not by grepping for a
`version:` string — in a minified bundle such a string belongs to whichever sub-library happens to
sit at that offset.

Note that plotly's locale files are genuinely version-specific: `plotly-locale-es-3.7.0.js` is
3,318 bytes while `plotly-locale-es-latest.js` is 3,235. Always fetch the pinned version.

## Files

### `datatables/es.json` — 10,700 bytes

* Source: <https://cdn.datatables.net/plug-ins/3.0.1/i18n/es-ES.json>
* License: MIT, Copyright (C) 2008-2026 SpryMedia Ltd. Full text in
  `src/cmd/assets/LICENSE-DataTables.txt` (shared with the vendored DataTables bundle).
* Supplies every DataTables-authored string in the data-viewer drawer: pagination, search,
  "no matching records", and the `columnControl` widget strings (the pinned combo includes
  `cc-2.0.0`, and this file covers it).
* **qsv overrides `searchBuilder.button` in this file** at assembly time, as it does for every
  locale. The vendored file renders it literally as "Constructor de búsqueda"; qsv deliberately
  names that control "Advanced Filter" (translated from qsv's own catalog,
  `viz.drawer.advanced_filter*`) because it sits alongside the per-column ColumnControl widgets and
  what distinguishes it is the cross-column AND/OR logic. See `datatables_language_block` in
  `src/cmd/viz.rs`. It is **not** the only splice-time override in play — see
  [Fixing a bad string in a vendored DataTables locale](#fixing-a-bad-string-in-a-vendored-datatables-locale)
  for the rule and for the `zh-CN` date-condition correction.
* `%d` in the counted form is a **DataTables** placeholder, not a rust-i18n one, and must reach
  the page verbatim. rust-i18n only interpolates `%{name}`, so it passes through untouched;
  `assert_data_drawer_lang_needle_is_current` guards this.

### `plotly/plotly-locale-es.js` — 3,318 bytes

* Source: <https://cdn.plot.ly/plotly-locale-es-3.7.0.js>
* License: MIT, Copyright (C) 2012-2026 Plotly, Inc.
* Localizes modebar tooltips, and as a bonus the axis month/day names and the decimal/thousands
  separators.
* The file **self-registers**, which is what makes it safe to emit on either side of the bundle:
  `typeof Plotly === "undefined" ? window.PlotlyLocales.push(locale) : Plotly.register(locale)`.
* Registering a locale does not *select* it — `Plotly.setPlotConfig({locale})` does, and only for
  renders that follow. qsv emits that call in two places because plotly arrives on two schedules;
  see `plotly_setlocale_js` in `src/cmd/viz.rs`.

### fr, de, it, pt-BR — retrieved 2026-07-30

Same licences, same pins as the `es` pair above. **The local file name is the `LOCALES` tag; the
CDN name often is not** — DataTables region-qualifies most European languages:

| Local file | Bytes | Fetched from |
|---|---|---|
| `datatables/fr.json` | 10,838 | `.../3.0.1/i18n/fr-FR.json` |
| `datatables/de.json` | 10,523 | `.../3.0.1/i18n/de-DE.json` |
| `datatables/it.json` | 10,443 | `.../3.0.1/i18n/it-IT.json` |
| `datatables/pt-BR.json` | 8,659 | `.../3.0.1/i18n/pt-BR.json` |
| `plotly/plotly-locale-fr.js` | 3,505 | `plotly-locale-fr-3.7.0.js` |
| `plotly/plotly-locale-de.js` | 3,225 | `plotly-locale-de-3.7.0.js` |
| `plotly/plotly-locale-it.js` | 3,317 | `plotly-locale-it-3.7.0.js` |
| `plotly/plotly-locale-pt-BR.js` | 3,345 | `plotly-locale-pt-br-3.7.0.js` |

**Why the tag is `pt-BR` and not `pt`.** plotly publishes **no generic `pt` locale** —
`https://cdn.plot.ly/plotly-locale-pt-3.7.0.js` returns 403; only `pt-br` exists, and its internal
id is `pt-BR`. Since `plotly_setlocale_js` passes the `LOCALES` tag straight to
`Plotly.setPlotConfig`, a `pt` row would register a locale that is then never selected — an English
modebar inside a translated dashboard, with no error anywhere. `--language pt` still resolves, via
the `pt` alias on the row. **Verify registered-vs-selected after adding any language**: the id in
the vendored file (`name:"…"`) must equal the tag, e.g.

```bash
grep -oE 'name:"[A-Za-z-]+",dictionary' plotly/plotly-locale-<tag>.js   # registered
grep -oE 'setPlotConfig\(\{locale: "[^"]*"' <rendered>.html             # selected
```

**Incomplete DataTables coverage — deliberate, do not "fix" by hand-editing a vendored file.**
DataTables falls back to its own English default for any key its language object omits:

* `it.json` omits `lengthLabels` and `orderClear`;
* `pt-BR.json` omits the whole `columnControl` group, so the per-column search widgets
  (ColumnControl 2.0.0) stay English in a Brazilian Portuguese drawer.

### ja, zh-CN — retrieved 2026-07-30

| Local file | Bytes | Fetched from |
|---|---|---|
| `datatables/ja.json` | 8,845 | `.../3.0.1/i18n/ja.json` |
| `datatables/zh-CN.json` | 7,844 | `.../3.0.1/i18n/**zh**.json` |
| `plotly/plotly-locale-ja.js` | 5,183 | `plotly-locale-ja-3.7.0.js` |
| `plotly/plotly-locale-zh-CN.js` | 4,164 | `plotly-locale-**zh-cn**-3.7.0.js` |

**`zh-CN` is the tag for two independent reasons**, and it is the same trap `pt-BR` documents above:

1. plotly ships only `plotly-locale-zh-cn`, internal id **`zh-CN`**, with no generic `zh` — so a `zh`
   tag would register a locale `setPlotConfig` never selects.
2. DataTables' bare `zh.json` **is Simplified** — verified by inspection (`搜索`, `没有找到匹配的记录`)
   — with Traditional published separately as `zh-HANT.json`. Calling the row `zh` would imply we
   serve a script we do not ship.

`--language zh` resolves through the row's alias; `zh-Hant` deliberately does **not**, because
`parse_lang`'s regional fallback matches base tags only. Without that rule (fixed in the same
branch) a Traditional request would have silently returned Simplified — a wrong *script*, not
merely a wrong dialect.

**Neither file has `columnControl`**, and both also omit `lengthLabels` and `orderClear`, so those
controls keep DataTables' English defaults — the same shape as `pt-BR.json`. Of the seven vendored
DataTables locales only `es`, `fr` and `de` are complete.

## Fixing a bad string in a vendored DataTables locale

**Do not edit the file here.** It must stay a byte-faithful copy of the CDN, or the next version
bump silently reverts the fix. Two steps instead:

1. **Correct it locally at splice time**, in `datatables_language_block` — the same `get_mut`
   override already used for `searchBuilder.button`. Pair it with a test that pins the corrected
   output *and* asserts the vendored file still needs correcting, so the override is deleted rather
   than left shadowing a string upstream has since fixed. `zh_date_condition_labels_are_corrected`
   is the worked example (upstream zh has its SearchBuilder date conditions inverted: `after` reads
   早于 "earlier than", `before` reads 晚于 "later than").

2. **Send the fix upstream — but NOT as a pull request.** DataTables' `contributing.md` is explicit:

   > In the case of i18n Plugins, we ask that you don't create a pull request and instead make use
   > of the management system that we have in place for this on our website.

   The evidence backs the request: <https://github.com/DataTables/Plugins> has ~7 open i18n PRs,
   the oldest from **2015**, none merged. Use <https://datatables.net/plug-ins/i18n/> instead —
   every language row has a **Contribute** button opening an "Edit Language Options" form with one
   input per key (including `searchBuilder`, `columnControl` and `lengthLabels`, i.e. the groups
   several of our files are missing). No account is needed; submissions land in a moderation queue
   ("pending sets ... submitted for approval"). Note the form states that contributing **licenses
   the change under MIT**, and asks for a contributor name — so it is a decision for the
   maintainer, not something to submit unattended.

   The canonical source is `i18n/<code>.json` in the `DataTables/Plugins` repo; our
   `datatables/zh-CN.json` was verified byte-identical to both that file and the CDN.

## Adding a language

1. `src/cmd/locales/<bcp47>.yml` (rust-i18n catalog; the key-parity test requires one per
   `LOCALES` row).
2. `datatables/<bcp47>.json` from `https://cdn.datatables.net/plug-ins/<dt-version>/i18n/<code>.json`
   — verify the exact file name at the CDN, it is region-qualified for some languages (`es-ES`,
   `pt-BR`) and bare for others (`ja`, `ko`).
3. `plotly/plotly-locale-<tag>.js` from `https://cdn.plot.ly/plotly-locale-<tag>-<version>.js`.
4. A row in `LOCALES` (`src/cmd/viz_i18n.rs`) and an arm in each of `plotly_locale_block` /
   `datatables_language_block` (`src/cmd/viz.rs`).

A curated language with no vendored file still works — the qsv chrome is translated and only the
library strings stay English, which is the deliberate fallback in both match arms.
