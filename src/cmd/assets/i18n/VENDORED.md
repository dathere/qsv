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
| `datatables/*.json` | DataTables combo `dt-3.0.0/b-4.0.0/cc-2.0.0/date-2.0.0/sb-2.0.0` | `DATATABLES_CDN_COMBO`, `src/cmd/viz.rs` |
| `plotly/plotly-locale-*.js` | plotly.js **3.7.0** | `PLOTLY_CDN_VERSION`, `src/cmd/viz.rs` (and the bundle shipped by the `plotly` crate) |

The plotly version was confirmed from the **header banner** of the crate's bundled
`plotly/resource/plotly.min.js` (`/** * plotly.js v3.7.0 ... */`), not by grepping for a
`version:` string — in a minified bundle such a string belongs to whichever sub-library happens to
sit at that offset.

Note that plotly's locale files are genuinely version-specific: `plotly-locale-es-3.7.0.js` is
3,318 bytes while `plotly-locale-es-latest.js` is 3,235. Always fetch the pinned version.

## Files

### `datatables/es.json` — 10,700 bytes

* Source: <https://cdn.datatables.net/plug-ins/3.0.0/i18n/es-ES.json>
* License: MIT, Copyright (C) 2008-2026 SpryMedia Ltd. Full text in
  `src/cmd/assets/LICENSE-DataTables.txt` (shared with the vendored DataTables bundle).
* Supplies every DataTables-authored string in the data-viewer drawer: pagination, search,
  "no matching records", and the `columnControl` widget strings (the pinned combo includes
  `cc-2.0.0`, and this file covers it).
* **qsv overrides exactly one key** at assembly time: `searchBuilder.button`. The vendored file
  renders it literally as "Constructor de búsqueda"; qsv deliberately names that control
  "Advanced Filter" (translated from qsv's own catalog, `viz.drawer.advanced_filter*`) because it
  sits alongside the per-column ColumnControl widgets and what distinguishes it is the
  cross-column AND/OR logic. See `datatables_language_block` in `src/cmd/viz.rs`.
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
