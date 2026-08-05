# Publish Workflow Audit — qsv v21.1.0
Generated: 2026-08-05 | Audited commit: e0a7a8d1f | Resolved in: 6f994012e (#4350)

> **Scope note.** This is a *publish/CI workflow* audit, not part of the
> `AUDIT_REPORT_<date>.md` documentation-audit series in this directory.
>
> **Status: largely RESOLVED by #4350.** Sections 1-3b are the original findings and are
> preserved as the historical record. See "Status after #4350" immediately below for what
> shipped and what is still open. Two claims in the original text were wrong and are
> corrected inline — search for **CORRECTION**.

## Status after #4350

Shipped:

- `viz`/`viz_static` enabled on 10 little-endian publish targets across 5 workflows (was 3).
- `compile_error!` guard on the `viz` feature in `src/cmd/mod.rs` for big-endian targets,
  plus the endianness note moved onto `viz` in `Cargo.toml` (§5 items 1 and 2).
- New `.github/workflows/rust-linux-arm64.yml` — `aarch64-unknown-linux-gnu` previously had
  no test workflow of any kind, the only published platform without one.
- Base-`viz` step added to `rust-musl.yml` (its existing run uses `lite`, which cannot cover
  viz since `viz.rs` is gated on `feature_capable`).
- `rust-macos.yml` now compiles the `viz_static` tier.
- Five viz tests gated on `#[cfg(feature = "geocode")]`, matching four siblings already gated.

Still open:

- **`viz` on the musl and aarch64-linux *publish* targets.** The CI that justifies it is now
  merged and green (three consecutive passes each), so this is a four-line follow-up:
  `publish.yml:59`, `publish.yml:148`, `publish-portable.yml:50`, `publish-portable.yml:120`.
- **Windows has no `viz_static` coverage** despite shipping that tier from three workflows.
  Adding it to `rust-windows.yml` was tried and reverted: it pushed that job from ~27min past
  its 90min timeout (~54 extra crates — plotly_static, fantoccini, native-tls, icu_segmenter,
  tar). It needs its own paths-filtered workflow mirroring `rust-viz-static.yml`, not a
  bolt-on. macOS was measured and is unaffected (22-26min vs 18-38min on master).
- **`publish.yml`'s Windows `viz_static` builds have never run** (tag-triggered; viz postdates
  21.1.0). Expect materially longer Windows publish times at the next tag.
- §4 (`util.rs::version()` tokens) and §5 item 3 (README/FEATURES note) — not addressed.
- `sniff_github_blob_url` fetches a live `github.com` URL from CI and fails intermittently;
  it cost a re-run during #4350. Pre-existing, unrelated to viz.

---

Question: "Are the GH Publish workflows up-to-date?"

Answer: **Mechanically yes, functionally no.** Action versions and runner images are
current (dependabot-managed). The **feature lists have drifted badly** — the prebuilt
`qsv` binaries do not ship several commands that shipped in the source months ago.

## 1. Action versions / runners — UP TO DATE

`.github/dependabot.yml` has a daily `github-actions` ecosystem entry, so pinned
actions track upstream:

- `actions/checkout@v7`, `actions/setup-python@v7`, `actions/cache@v6`,
  `actions/upload-artifact@v7`
- `svenstaro/upload-release-action@v2`, `robinraju/release-downloader@v1.13`,
  `taiki-e/install-action@v2.85.6`, `WyriHaximus/github-action-get-previous-tag@v2`
- `dtolnay/rust-toolchain@master` (unpinned by design; `toolchain: stable`)

Minor nit: 8 workflows still use `ubuntu-22.04` for the lightweight
`analyze-tags`/lint jobs (`publish.yml`, `publish-portable.yml`, `publish-qsvpy.yml`,
`publish-deb-package.yml`, `test-publish.yml`, `macOS-arm64-selfhosted-publish.yml`,
`macOS-arm64-selfhosted-publish-qsvpy.yml`, `devskim.yml`). Build jobs are already on
`ubuntu-24.04`. Worth moving before GitHub retires the 22.04 image.

## 2. Feature lists — STALE (the real finding)

`README.md:298` states the prebuilt `qsv` enables **"all applicable features except
Python"**. `Cargo.toml`'s `distrib_features` is the machine-readable version of that
promise:

```
feature_capable, apply, fetch, foreach, geocode, geoconnex, get, get_cloud,
luau, mcp, polars, profile, synthesize, to, viz_static
```

No publish workflow uses `distrib_features`. Each hand-maintains a comma-separated
`addl-build-args` list, and **not one of them was updated** when these features landed:

| feature | added to `distrib_features` | commit |
|---|---|---|
| `synthesize` | 2026-05-15 | `3df17e54f` (#3854) |
| `profile` | 2026-05-24 | `a243e5151` (#3898) |
| `get` / `get_cloud` | 2026-06-05 | `2db0ebea3` (#2263 / #3953) |
| `geoconnex` | 2026-06-21 | `5b046ecc1` (#4052) |
| `mcp` | — | (never in any publish list) |

`.github/` contains **zero** occurrences of `get_cloud`, `geoconnex`, or `synthesize`,
so this is an omission, not a documented decision.

These are hard `#[cfg]` gates in `src/main.rs` (lines 156, 189, 205, 247, 481, 501,
508, 535, 606, 630, 637, 664), so the commands are **absent from the shipped binary**,
not merely degraded:

- `qsv get` — `#[cfg(all(feature = "get", feature = "feature_capable"))]`
- `qsv profile` — `#[cfg(feature = "profile")]`
- `qsv synthesize` — `#[cfg(all(feature = "synthesize", feature = "feature_capable"))]`
- `qsv mcp` — `#[cfg(feature = "mcp")]`

Note `get` also backs the `dc:` input prefix used by *every* command, so its absence is
wider than one subcommand.

### Per-target gap in `publish.yml` (vs. distrib_features + magika/self_update/ui)

| target | missing |
|---|---|
| `x86_64-unknown-linux-gnu` | geoconnex, get, get_cloud, mcp, profile, synthesize, clipboard |
| `x86_64-pc-windows-msvc` | geoconnex, get, get_cloud, mcp, profile, synthesize, clipboard |
| `x86_64-pc-windows-gnu` | above + magika |
| `aarch64-unknown-linux-gnu` | above + geocode, luau, polars, to, viz_static, magika, prompt |
| `x86_64-unknown-linux-musl` | above (luau exclusion is documented at README:183/307) |

`aarch64-unknown-linux-gnu` is the surprise: ARM64 Linux ships without luau, polars,
`to`, geocode, or viz. Only the musl luau exclusion is documented in the README. This
looks like a leftover from when that target was QEMU cross-compiled — the matrix has
since switched to a native hosted ARM runner (`ubuntu-24.04-arm`) but the reduced
feature list was never revisited. (The `qsvdp`/`qsvmcp` skip on this target *is*
deliberate and commented — polars fat-LTO OOMs the runner — which may also explain
`polars`, but not `get`/`synthesize`/`luau`.)

### Same drift in the sibling workflows

- `publish-portable.yml` — also missing `viz_static` on every target
- `publish-qsvpy.yml` — missing `viz_static` + the five above
- `macOS-arm64-selfhosted-publish.yml` — missing the five (no `viz_static` on macOS is
  expected/known)
- `publish-nightly.yml`, `macOS-arm64-selfhosted-publish-nightly.yml` — missing the five
  + `magika`, `color`, `viz_static`
- `publish-aarch64-pc-windows-msvc.yml`, `publish-powerpc64le-*`, `publish-s390x-*` —
  narrow lists; `viz_static` exclusion on big-endian/headless is intentional per the
  `Cargo.toml` comment, the rest is not
- `test-publish.yml` — furthest behind (`apply,luau,fetch,foreach,self_update,geocode,polars,to`),
  so it does not actually smoke-test what `publish.yml` builds

## 3. Separate question: the .deb package

`Cargo.toml [package.metadata.deb] features = ["feature_capable"]` — the `qsv.deb`
is built with **only** `feature_capable`: no apply, fetch, luau, polars, geocode, to,
viz, get, profile, synthesize, mcp, self_update. That line dates to 2024 (`9c329e3b7`)
and predates most features. This may be a deliberate minimal-dependency Debian choice,
but it is not documented anywhere, and it means `qsv.deb` != prebuilt `qsv`.

## 3b. `viz` / `viz_static` specifically

Accepting that the lists are hand-maintained *because* features are platform-specific,
viz is still the weakest spot — because the platform argument does not explain what is
actually there.

**`viz` in any form appears in exactly 3 places, all `viz_static`, all in `publish.yml`:**
x86_64-unknown-linux-gnu (:44), x86_64-pc-windows-msvc (:83), x86_64-pc-windows-gnu (:109).
Every other published artifact — portable, nightly, qsvpy, macOS ARM64, Windows ARM64,
ppc64le, s390x, aarch64-linux, musl, .deb — ships **no viz at all**.

### The base `viz` tier is never used anywhere

`Cargo.toml` deliberately splits the feature in two:

- `viz` — self-contained interactive HTML (`plotly_embed_js`), **no browser at runtime**,
  no polars. Deps: plotly, opener, geojson, rust-i18n, base64-simd.
- `viz_static` — adds PNG/SVG/PDF/JPEG/WebP export via plotly_static +
  webdriver-downloader; *"Requires a browser at runtime; keep out of big-endian /
  headless-only publish targets."*

That two-tier design exists precisely so a platform that can't do headless-browser export
can still ship interactive dashboards. **No publish workflow ever selects plain `viz`.**
It is `viz_static` or nothing — so the middle tier the design provides is unused.

None of viz's dependencies are target-gated. The only `target.'cfg(...)'` tables in
`Cargo.toml` are datasketches (big-endian) at :553 and polars avx512 (x86_64) at :561 —
neither touches viz.

### Same target, different answer — so not a platform constraint

`x86_64-unknown-linux-gnu` gets `viz_static` in `publish.yml` but **no viz** in three
other workflows building the identical target:

| workflow | x86_64-unknown-linux-gnu features | viz? |
|---|---|---|
| `publish.yml:44` | `…,magika,color,viz_static` | ✅ |
| `publish-portable.yml:39` | `…,magika,color` | ❌ |
| `publish-qsvpy.yml:36` | `…,python,lens,prompt,magika,color` | ❌ |
| `publish-nightly.yml:36` | `…,to,lens,prompt,nightly-polars` | ❌ |

Portable's rationale is "no CPU features" — but viz has no CPU-feature dependency
(`base64-simd` is already pulled in by `apply` and `feature_capable`, both of which
portable enables). These three look like the list simply wasn't updated when viz landed
2026-06-18 (`2f83bcd8e`), which touched only `publish.yml`.

### macOS ARM64 is the clearest gap

`rust-macos.yml:45` runs `cargo test --features=apply,fetch,foreach,geocode,luau,python,polars,to,feature_capable,ui,viz`
on **every PR** — macOS is proven to build and pass the viz test suite. Yet
`macOS-arm64-selfhosted-publish.yml:38` ships
`apply,fetch,foreach,self_update,luau,polars,to,geocode,lens,magika,color` — no viz.

Apple Silicon is a mainstream qsv prebuilt, so Mac users currently get no `viz` and no
`viz smart` Amalgram at all, despite CI proving the base tier works there.

### Exclusions that ARE defensible

| target | viz CI coverage | verdict |
|---|---|---|
| ppc64le (`rust-ppc64le.yml:45`) | no viz tested | reasonable — untested (NOT big-endian; see CORRECTION in §5) |
| s390x (`rust-s390x.yml:63`) | no viz tested | reasonable — untested + big-endian |
| Windows ARM64 (`rust-windows-arm64.yml:50`) | no viz tested | reasonable — untested |
| musl (`rust-musl.yml:52`) | `lite` only | reasonable — no signal |
| aarch64-linux | no dedicated viz CI | ambiguous — worth a decision |

Note `viz_static` on big-endian is explicitly discouraged by the `Cargo.toml` comment,
but plain `viz` has no such constraint — it is only *untested* there, which is a
different (and fixable) reason.

### Windows & Linux artifact inventory (viz status)

Even on the two platforms that *do* get viz, only `publish.yml`'s x86_64 builds have it.

**Linux — 1 of 11 artifacts has viz:**

| workflow | target | viz? |
|---|---|---|
| `publish.yml:44` | x86_64-linux-gnu | ✅ `viz_static` |
| `publish.yml` | x86_64-linux-musl | ❌ |
| `publish.yml` | aarch64-linux-gnu | ❌ |
| `publish-portable.yml:39` | x86_64-linux-gnu (`qsvp`) | ❌ |
| `publish-portable.yml:50` | x86_64-linux-musl | ❌ |
| `publish-portable.yml:120` | aarch64-linux-gnu | ❌ |
| `publish-nightly.yml:36` | x86_64-linux-gnu | ❌ |
| `publish-qsvpy.yml:36` | x86_64-linux-gnu (`qsvpy311/312/313`) | ❌ |
| `publish-powerpc64le-*` | ppc64le | ❌ |
| `publish-s390x-*` | s390x | ❌ |
| `.deb` (`Cargo.toml` metadata) | x86_64 | ❌ |

**Windows — 2 of 7 artifacts have viz:**

| workflow | target | viz? |
|---|---|---|
| `publish.yml:83` | x86_64-windows-msvc | ✅ `viz_static` |
| `publish.yml:109` | x86_64-windows-gnu | ✅ `viz_static` |
| `publish-portable.yml:70` | x86_64-windows-msvc | ❌ |
| `publish-portable.yml:91` | x86_64-windows-gnu | ❌ |
| `publish-nightly.yml:45` | x86_64-windows-msvc | ❌ |
| `publish-qsvpy.yml:52` | x86_64-windows-msvc | ❌ |
| `publish-aarch64-pc-windows-msvc.yml:42` | aarch64-windows-msvc | ❌ |

### The portable gap is the one with a documented user consequence

`README.md:167` tells users that if they hit SIGILL faults on x86_64, the **portable**
binaries (`qsvp`, `qsvplite`, `qsvpdp`) are the remedy. But portable is not a
CPU-flags-only variant of the main build — it silently drops features:

| target | `publish.yml` minus `publish-portable.yml` |
|---|---|
| x86_64-linux-gnu | `viz_static` |
| x86_64-windows-msvc | `viz_static`, `magika`, `color` |
| x86_64-windows-gnu | `viz_static`, `color`, `foreach` |
| x86_64-linux-musl | `lens`, `color` |
| aarch64-linux-gnu | `color` |

None of these have a CPU-feature tie — portable's stated rationale is
`target-cpu=native` removal only. `base64-simd` (viz's only SIMD-adjacent dep) is
already pulled in by `apply` and `feature_capable`, both of which portable enables.
So a user following the documented SIGILL workaround loses `viz` entirely, and on
Windows also loses magika and colored output.

Two smaller inconsistencies in the same file: `publish-portable.yml:44` still builds
musl on `ubuntu-22.04` (all other build jobs moved to 24.04), and `:119` cross-compiles
aarch64 with `use-cross: true` on `ubuntu-24.04` while `publish.yml` builds it natively
on `ubuntu-24.04-arm`.

### What CI actually proves (complete viz coverage map)

⚠️ First, a correction that changes how this evidence must be read: **no release has ever
shipped viz.** Tag 21.1.0 is 2026-06-14; the viz commit `2f83bcd8e` is 2026-06-18. So
`publish.yml`'s existing `viz_static` entries are *configured but never released* — they
are not battle-tested evidence. All confidence must come from CI.

| CI workflow | runner | arch | toolchain | viz tier tested |
|---|---|---|---|---|
| `rust.yml:67` | ubuntu-latest | x86_64 | stable | **`viz_static`** (compiles static tier; runs non-ignored viz tests) |
| `rust-viz-static.yml:68` | ubuntu-latest | x86_64 | stable | **`viz_static`, actually executed** w/ browser — but paths-filtered + weekly, not per-PR |
| `rust-macos.yml:45` | macos-26 | **arm64** | stable | **`viz`** (base tier, per-PR) |
| `rust-windows.yml:49` | windows-latest | x86_64 | stable | **`viz`** (base tier, per-PR, `--no-default-features`) |
| `rust-windows-arm64.yml:50` | windows-11-arm | arm64 | stable | ❌ none |
| `rust-ppc64le.yml:45` | ubuntu-24.04-ppc64le | ppc64le | stable | ❌ none |
| `rust-s390x.yml:63` | ubuntu-24.04-s390x | s390x | stable | ❌ none |
| `rust-musl.yml:52` | ubuntu-latest | x86_64-musl | stable | ❌ none (`lite` only) |
| `rust-beta.yml:40` | ubuntu-latest | x86_64 | beta | ❌ none |
| `rust-nightly-bleeding-edge.yml:44` | ubuntu-latest | x86_64 | nightly | ❌ none |
| `rust-polars-nightly.yml:40` | ubuntu-latest | x86_64 | nightly | ❌ none |
| `rust-polars-pinned-nightly.yml:41` | ubuntu-24.04 | x86_64 | nightly-2026-04-01 | ❌ none |

Note `macos-26` is Apple Silicon, which is exactly the
`macOS-arm64-selfhosted-publish.yml` target (`aarch64-apple-darwin`) — a direct match.

### Verdict: what we can confidently add

**Tier 1 — add now, CI-backed on the same OS + arch + toolchain:**

| # | workflow / target | add | evidence |
|---|---|---|---|
| 1 | `macOS-arm64-selfhosted-publish.yml:38` (aarch64-apple-darwin) | **`viz`** | `rust-macos.yml` runs viz tests on macos-26 arm64 stable, per-PR |
| 2 | `publish-portable.yml:39` (x86_64-linux-gnu) | **`viz_static`** | `rust.yml` + `rust-viz-static.yml`, same runner/arch/toolchain |
| 3 | `publish-qsvpy.yml:36` (x86_64-linux-gnu) | **`viz_static`** | same; `rust.yml` also tests `python` alongside |
| 4 | `publish-portable.yml:70` (x86_64-windows-msvc) | **`viz_static`** | `rust-windows.yml` proves base viz on the same runner w/ `--no-default-features` |
| 5 | `publish-qsvpy.yml:52` (x86_64-windows-msvc) | **`viz_static`** | same |

Portable's rationale (`target-cpu=native` off) does not interact with viz — `base64-simd`,
viz's only SIMD-adjacent dep, is already present via `apply`/`feature_capable`.

For 4 & 5 the *executed* Windows evidence is base `viz` only; the static tier is executed
only on Linux. If you want strict parity of evidence, ship `viz` (not `viz_static`) on
Windows portable/qsvpy — or add a Windows job to `rust-viz-static.yml` first.

**Tier 2 — consistent with an existing decision, but no CI at all:**

| # | workflow / target | add | caveat |
|---|---|---|---|
| 6 | `publish-portable.yml:91` (x86_64-windows-gnu) | `viz_static` | **no CI covers windows-gnu for any feature**; `publish.yml:109` already configures viz_static there, so this only matches an existing (also untested) choice |

**Tier 3 — do NOT add without new CI first:**

| workflow / target | why |
|---|---|
| `publish-nightly.yml`, `macOS-arm64-selfhosted-publish-nightly.yml` | **zero** nightly-toolchain CI tests viz, and these build with `-Z build-std=std` — a materially different build mode |
| `publish.yml` aarch64-unknown-linux-gnu, `publish-portable.yml:120` | no ARM-Linux viz CI exists |
| `publish-aarch64-pc-windows-msvc.yml:42` | `rust-windows-arm64.yml` exists but tests no viz |
| `publish-s390x-*` | **big-endian — neither `viz` nor `viz_static` works there. Permanent exclusion, not a CI gap.** See §5 |
| `publish-powerpc64le-*` | ppc64le is **little**-endian (see the CORRECTION in §5), so the endianness rule does not apply. Excluded for other reasons: no viz CI, and polars does not compile on PowerPC |
| `publish.yml`/`publish-portable.yml` musl | `rust-musl.yml` tests `lite` only |

**Cheapest way to expand the confident set:** add `viz` to the test line in
`rust-windows-arm64.yml`, and add a nightly viz job. Each is a one-token edit; whatever
comes back green becomes shippable. **Do NOT do this for `rust-s390x.yml`** — it is big-endian
and the `compile_error!` guard will reject the build by design; see §5. `rust-ppc64le.yml` is
little-endian and not covered by that rule, but polars does not compile on PowerPC, so any viz
experiment there would need a feature set that omits polars.

## 5. Big-endian: `viz` and `viz_static` do not work there (per maintainer)

Neither tier functions on big-endian targets. This is a **permanent platform exclusion, not
missing test coverage** — an earlier draft of this document wrongly suggested adding `viz` to
the big-endian CI workflows to "find out". Do not.

> **CORRECTION.** Earlier revisions of this document (and the first version of the
> `Cargo.toml`/`src/cmd/mod.rs` comments shipped in #4350) listed **ppc64le as big-endian.
> It is not.** Verified:
>
> ```
> powerpc64le-unknown-linux-gnu → target_endian="little"
> s390x-unknown-linux-gnu       → target_endian="big"
> powerpc64-unknown-linux-gnu   → target_endian="big"
> ```
>
> So the guarded target among qsv's publish matrix is **s390x** (and powerpc64 BE, which qsv
> does not publish). The `compile_error!` deliberately does **not** fire for ppc64le. `viz` is
> absent from the ppc64le prebuilt for unrelated reasons: it has no viz CI coverage, and polars
> does not compile on PowerPC (see `docs/publishing_assets/qsv-powerpc64le-*.txt`).
>
> Whether `viz` would actually *work* on ppc64le is therefore an open question, not a settled
> exclusion — the maintainer's "not on big-endian" constraint does not reach it.
> Caught by roborev job 4051; corrected in the code by commit `63a27f0b6`.
>
> Read every "big-endian (ppc64le, s390x)" phrasing below as **s390x only**.

The constraint is currently **undocumented and unenforced** for the base tier:

- `Cargo.toml:667` states the big-endian caveat **only on `viz_static`** ("Requires a
  browser at runtime; keep out of big-endian / headless-only publish targets") — and
  frames it as a browser/headless issue, not an endianness one.
- The `viz` feature comment (`Cargo.toml:653-663`) says nothing about endianness.
- **`src/cmd/viz.rs` contains zero `#[cfg(target_endian …)]` guards**, unlike every other
  endian-sensitive module in the tree (`src/cmd/frequency.rs`, `src/cmd/stats.rs`,
  `src/cmd/schema.rs`, `src/cmd/pragmastat.rs`, `src/util.rs` all guard it).

So `cargo build -F viz` on s390x compiles and produces a **silently broken** binary. Nothing
in the build or the docs stops a packager or user from doing this. (Per the CORRECTION above,
this does not apply to ppc64le, which is little-endian.)

Suggested hardening — **items 1 and 2 shipped in #4350; item 3 is still open**:

1. Move the big-endian note onto the **`viz`** feature comment in `Cargo.toml`, so it
   covers both tiers, and state it as an endianness constraint distinct from
   `viz_static`'s separate browser-at-runtime requirement.
2. Add a `compile_error!` (or a `#[cfg(target_endian = "big")]` guard) so a big-endian
   `-F viz` build fails loudly instead of shipping a broken command.
3. Add a line to `docs/FEATURES.md` / the README feature table noting viz is
   little-endian only.

Note this does not affect any of the Tier 1 changes already applied — x86_64 and
aarch64-apple-darwin are all little-endian.

## 4. Same drift in `qsv --version`

`src/util.rs::version()` (lines 677-742) builds its `enabled_features` token string from
the same kind of hand-maintained `#[cfg]` list, and it too was never updated. It emits
tokens for apply, fetch, foreach, geocode, luau, magika, prompt, python, to, viz,
viz_static, polars, self_update — and has **no token** for `get`, `get_cloud`, `profile`,
`synthesize`, `mcp`, `geoconnex`, `lens`, `color`, or `clipboard`.

So `qsv --version` cannot currently be used to detect the packaging gap, and would need
fixing first for the CI check recommended below.

## Recommended fix

⚠️ **Superseded in part.** An earlier draft recommended replacing the hand-maintained
lists with `distrib_features` wholesale, and asserting every binary matches it. That is
wrong: the lists are hand-maintained *because* features are platform-specific, and
`distrib_features` itself includes `viz_static`, which per §5 cannot build on big-endian
and is unwanted on musl / ARM-Linux. A blanket `distrib_features` rollout or a global
equality assertion would break those targets. Revised:

1. Add the missing tokens to `util.rs::version()` (`get`, `get_cloud`, `profile`,
   `synthesize`, `mcp`, `geoconnex`, `lens`, `color`, `clipboard`) so the shipped binary
   can at least *report* what it has.
2. Keep the per-target lists hand-maintained. Where a feature is omitted, add a short
   comment naming the reason (platform limitation vs. not-yet-evaluated), as
   `Cargo.toml` already does for datasketches/avx512. Most current omissions carry no
   such comment, which is what made this audit necessary.
3. Scope any drift assertion to the **flagship little-endian targets only**
   (x86_64-unknown-linux-gnu, x86_64-pc-windows-msvc): assert those two match a named
   feature set, rather than asserting all targets match `distrib_features`.
4. Consider whether `distrib_features` should still contain `viz_static` given §5, or
   whether big-endian builds need their own umbrella feature.

Given 21.1.0 was tagged 2026-06-14 and a 22.0.0 release is in preparation, this should
land before the next tag.
