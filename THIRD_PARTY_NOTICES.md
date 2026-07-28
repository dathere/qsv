# Third-party notices

qsv is distributed under the MIT license (see [`LICENSE-MIT`](LICENSE-MIT) and
[`COPYING`](COPYING)).

This file records the third-party open source software and data that qsv
vendors, embeds, or redistributes, together with the copyright notices and
license texts those licenses require us to carry. It covers material that
travels with a qsv binary, a release archive, the crates.io tarball, or the
HTML that `qsv viz` generates.

It does **not** attempt to enumerate qsv's ordinary Cargo dependencies, which
are fetched from crates.io at build time and carry their own license metadata;
run `cargo tree` or a tool such as `cargo-about` for that inventory.

## Summary

| Component | Version / pin | License | How qsv ships it |
|---|---|---|---|
| [xsv](https://github.com/BurntSushi/xsv) | fork point, Sept 2021 | MIT | qsv is a fork; upstream copyright retained in `LICENSE-MIT` |
| [plotly.js](https://github.com/plotly/plotly.js) | 3.7.0 | MIT | embedded in `qsv viz` HTML output (gzip+base64), or referenced by CDN |
| [MapLibre GL JS](https://github.com/maplibre/maplibre-gl-js) | as bundled in plotly.js 3.7.0 | BSD-3-Clause | bundled *inside* plotly.js; ships wherever plotly.js does |
| [plotly.rs](https://github.com/plotly/plotly.rs) | git rev `00fe051` | MIT | Cargo dependency, compiled into the `qsv` binary |
| [DataTables](https://datatables.net/) + Buttons, DateTime, Responsive, SearchBuilder | `dt-3.0.0/b-4.0.0/date-2.0.0/r-4.0.0/sb-2.0.0` | MIT | vendored in `src/cmd/assets/`, compiled in and embedded in `viz smart` HTML |
| [LuaDate](https://github.com/Tieske/date) | 2.2.1 | MIT | vendored in `resources/luau/vendor/luadate/`, compiled into the binary |
| [DCAT-US v3 schemas](https://github.com/GSA/dcat-us) | commit `cf87890` | CC0-1.0 | vendored in `resources/dcat-us-v3/`, compiled into the binary |
| [DCAT-AP v3 SHACL](https://github.com/SEMICeu/DCAT-AP) | release 3.0.0 | CC-BY-4.0 | vendored in `resources/dcat-ap-v3/`, compiled into the binary |
| [geoconnex SHACL](https://github.com/internetofwater/nabu) | commit `e5d6ad3` | Apache-2.0 | vendored in `resources/geoconnex/`, compiled into the binary |
| [GeoNames](https://www.geonames.org/) | `cities15000` + `countryInfo` | CC-BY-4.0 | source for the `examples/viz/world_cities.csv` sample |
| OpenStreetMap / CARTO basemap tiles | n/a | ODbL 1.0 / CARTO terms | fetched at view time by MapLibre; not redistributed |

---

## xsv

qsv is a fork of [xsv](https://github.com/BurntSushi/xsv) (forked September
2021). The upstream copyright is retained alongside datHere's in the project's
own license file, reproduced here for completeness.

```text
The MIT License (MIT)

Copyright © 2015 Andrew Gallant
Copyright © 2026 datHere, Inc.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```

---

## plotly.js

Upstream: <https://github.com/plotly/plotly.js> — version **3.7.0**.

`qsv viz` embeds the plotly.js runtime directly into the HTML it generates
(gzip-compressed and base64-encoded), so every generated chart and dashboard is
self-contained and works offline. With `QSV_VIZ_CDN` set, the bundle is
referenced from a version-pinned CDN URL with Subresource Integrity instead.

The bundle reaches qsv through the `plotly` Rust crate's bundled resource; its
own copyright header (`plotly.js v3.7.0 / Copyright 2012-2026, Plotly, Inc. /
Licensed under the MIT license`) is preserved intact in the embedded payload.

```text
MIT License

Copyright (c) 2016-2024 Plotly Technologies Inc.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```

---

## MapLibre GL JS

Upstream: <https://github.com/maplibre/maplibre-gl-js>.

MapLibre GL JS is **bundled inside plotly.js** — it is not a separate qsv
dependency. It powers the tile basemaps behind `qsv viz`'s `--map` mode, and it
ships wherever the plotly.js bundle ships. Its license also covers code
inherited from mapbox-gl-js v1.13, glfx.js, and d3-color; the upstream
`LICENSE.txt` is reproduced in full below.

```text
Copyright (c) 2023, MapLibre contributors

All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

    * Redistributions of source code must retain the above copyright notice,
      this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright notice,
      this list of conditions and the following disclaimer in the documentation
      and/or other materials provided with the distribution.
    * Neither the name of MapLibre GL JS nor the names of its contributors
      may be used to endorse or promote products derived from this software
      without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.


-------------------------------------------------------------------------------

Contains code from mapbox-gl-js v1.13 and earlier

Version v1.13 of mapbox-gl-js and earlier are licensed under a BSD-3-Clause license

Copyright (c) 2020, Mapbox
Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice,
  this list of conditions and the following disclaimer.
* Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.
* Neither the name of Mapbox GL JS nor the names of its contributors
  may be used to endorse or promote products derived from this software
  without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.


-------------------------------------------------------------------------------

Contains code from glfx.js

Copyright (C) 2011 by Evan Wallace

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

--------------------------------------------------------------------------------

Contains a portion of d3-color https://github.com/d3/d3-color

Copyright 2010-2016 Mike Bostock
All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice, this
  list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.

* Neither the name of the author nor the names of contributors may be used to
  endorse or promote products derived from this software without specific prior
  written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

---

## plotly.rs

Upstream: <https://github.com/plotly/plotly.rs> — pinned to git rev
`00fe0512e9f924bd0fc2edfc6cc617a1604f4e0d`.

The Rust charting API behind the `viz` command. An ordinary Cargo dependency,
compiled into the `qsv` binary.

```text
The MIT License (MIT)

Copyright (c) 2024 Plotly, Inc

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```

---

## DataTables

Upstream: <https://datatables.net/> — download-builder combination
`dt-3.0.0/b-4.0.0/date-2.0.0/r-4.0.0/sb-2.0.0`, comprising DataTables 3.0.0,
Buttons 4.0.0, DateTime 2.0.0, Responsive 4.0.0 and SearchBuilder 2.0.0.

Vendored unmodified at [`src/cmd/assets/datatables.min.js`](src/cmd/assets/datatables.min.js)
and [`src/cmd/assets/datatables.min.css`](src/cmd/assets/datatables.min.css),
compiled into the binary and embedded into the HTML `qsv viz smart` generates
(the data viewer drawer).

These are components of the freely-licensed DataTables suite, **not** the
commercial DataTables Plus suite (CardView, Editor, …). The MIT license is
conditioned on retaining the original copyright notice, so the bundle's
`/*! … */` banners are preserved in the minified files. See
[`src/cmd/assets/LICENSE-DataTables.txt`](src/cmd/assets/LICENSE-DataTables.txt).

```text
Copyright (C) 2008-2026, SpryMedia Ltd.

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## LuaDate

Upstream: <https://github.com/Tieske/date> — version 2.2.1.

Vendored at [`resources/luau/vendor/luadate/date.lua`](resources/luau/vendor/luadate/date.lua)
and compiled into the binary, providing date manipulation for the `luau`
command. See [`resources/luau/vendor/README.md`](resources/luau/vendor/README.md).

```text
The MIT License (MIT) http://opensource.org/licenses/MIT

Copyright (c) 2013-2021 Thijs Schreijer

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```

---

## DCAT-US v3 schemas

Upstream: <https://github.com/GSA/dcat-us>, `jsonschema/definitions`, pinned to
commit `cf8789002b1f60c2c7247de475dd565427e5b8b7`.

26 JSON Schema files vendored at [`resources/dcat-us-v3/`](resources/dcat-us-v3/)
and compiled into the binary for the `profile` command's DCAT-US validation.
File hashes are pinned in `MANIFEST.json` and enforced in CI.

These schemas are © General Services Administration. As a work of the United
States Government they are not subject to domestic copyright protection under
17 USC § 105, and are additionally released under the
[CC0 1.0 Universal public domain dedication](https://creativecommons.org/publicdomain/zero/1.0/).
qsv adds no copyright claim over the vendored content.

## DCAT-AP v3 SHACL shapes

Upstream: <https://github.com/SEMICeu/DCAT-AP>, release 3.0.0,
`releases/3.0.0/shacl/dcat-ap-SHACL.ttl`.

Vendored at [`resources/dcat-ap-v3/shacl/`](resources/dcat-ap-v3/shacl/) and
compiled into the binary for the `profile` command's DCAT-AP validation.

© SEMIC.eu / European Commission, licensed under
[Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)
(CC-BY-4.0). qsv redistributes the file unmodified.

## geoconnex SHACL shapes

Upstream: <https://github.com/internetofwater/nabu>,
`shacl_validator/shapes/geoconnex.ttl`, pinned to commit
`e5d6ad390a2cf9b0272676757713b1bf1757f75b`.

Vendored at [`resources/geoconnex/shacl/`](resources/geoconnex/shacl/) and
compiled into the binary for the `profile` command's geoconnex validation.

© Internet of Water contributors, licensed under the
[Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0). The upstream
project ships no `NOTICE` file. qsv redistributes the file unmodified.

---

## Code adapted from other projects

Beyond whole vendored components, a few places in qsv's own source were copied or adapted from
other MIT-licensed projects. The code lives in qsv's files (and carries a pointer to this section
at the site), so their copyright notices are reproduced here.

### tabiew

Upstream: <https://github.com/shshemi/tabiew>

The Monokai light/dark terminal palettes in `src/cmd/color.rs` (`COLORS_DARK` / `COLORS_LIGHT`).

```text
MIT License

Copyright (c) 2024 Shayan Hashemi

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

### polars-cli

Upstream: <https://github.com/pola-rs/polars-cli>

The `OutputMode::execute_query` implementation in `src/cmd/sqlp.rs`, copied from `src/main.rs`.

```text
Copyright (c) 2020 Ritchie Vink

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

### Acknowledged influences (not copied)

Two further places name an upstream as an influence rather than a source — the algorithm was
reimplemented, not copied, so no license text applies:

* `src/odhtcache.rs` — "inspired by" [race604/dedup](https://github.com/race604/dedup)
* `src/cmd/cat.rs` — the `cat_rowskey` approach, "largely inspired by"
  [vi/csvcatrow](https://github.com/vi/csvcatrow)

A third, the `autolayout` column-width routine in `src/cmd/color.rs`, is documented in-place as
**provenance unknown**: its original comment named no project, author, URL or license, and none
could be identified. It is deliberately left uncredited rather than credited to a guess.

---

## Sample data

The datasets under [`examples/viz/`](examples/viz/) exist to demonstrate the
`viz` command and are not part of the qsv binary. See
[`examples/viz/README.md`](examples/viz/README.md) for the per-dataset
inventory.

* **`world_cities.csv`** is derived from [GeoNames](https://www.geonames.org/)
  (`cities15000` + `countryInfo`), licensed under
  [Creative Commons Attribution 4.0](https://creativecommons.org/licenses/by/4.0/).
  Built by `examples/viz/gen_world_cities.py`; the `avg_annual_temp_c` column is
  synthesized by qsv, the rest is GeoNames-derived.
* **`allegheny_dog_licenses.csv`** — Allegheny County dog licenses, via the
  [Western Pennsylvania Regional Data Center](https://data.wprdc.org/).
* **`nyc_311.csv`**, **`nyc_capital_projects.csv`** — samples of
  [NYC Open Data](https://opendata.cityofnewyork.us/).
* **`cms_medicare_providers.csv`** — sample of
  [CMS](https://data.cms.gov/) provider data.
* **Boundary GeoJSON** (`allegheny_zip_boundaries.geojson`,
  `nyc_neighborhoods.geojson`, `japan_prefectures.geojson`) — provenance under
  review; these are third-party-derived and their upstream source and license
  have not yet been confirmed. `western_states.geojson` is hand-authored by the
  qsv project.

## Basemap tiles

`qsv viz --map` renders raster/vector tiles fetched at view time from
OpenStreetMap or CARTO. Tile data is **not** redistributed by qsv — it is
requested by the viewer's browser. Attribution is rendered by MapLibre's own
attribution control, and is additionally stated in the generated page footer:

* Map data © [OpenStreetMap](https://www.openstreetmap.org/copyright)
  contributors, licensed under the
  [Open Database License (ODbL) 1.0](https://opendatacommons.org/licenses/odbl/).
* Carto Positron / Dark Matter basemap styles © [CARTO](https://carto.com/attributions).

---

## Reporting an omission

If you believe something is vendored or redistributed here without correct
attribution, please open an issue at
<https://github.com/dathere/qsv/issues>. We would rather over-attribute than
under-attribute.
