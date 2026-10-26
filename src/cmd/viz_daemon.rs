//! Optional "live mode" for `viz smart` Data Schematics, backed by the `qsv-clientd` daemon.
//!
//! Every smart HTML page embeds a `qsv-schematic-source` JSON block (provenance: which qsv drew
//! it, from what data, with which arguments). By default the page also carries a small bridge
//! script that probes `http://127.0.0.1:<port>/v1/hello` once on load. When no daemon answers,
//! the bridge does nothing at all, so the page stays self-contained (`docs/DATA_SCHEMATIC.md`,
//! MUST 7).
//! When `qsv-clientd` answers, the bridge offers a "Live data" chip; pairing is never automatic
//! and needs the user to approve a code in a shell.
//!
//! The daemon's side of this contract is documented at
//! <https://github.com/dathere/qsv-client/blob/main/docs/DAEMON.md>.

use std::path::Path;

/// The id of the embedded provenance JSON. The daemon locates the block by this id.
pub(crate) const SOURCE_SCRIPT_ID: &str = "qsv-schematic-source";

/// The `qsv-clientd` browser-tier port when `QSV_CLIENTD_PORT` is unset or invalid.
pub(crate) const DEFAULT_CLIENTD_PORT: u16 = 14470;

/// `viz smart` options whose value can name a local file. Their value is reduced to a basename
/// in `viz_args` (and the flag listed in `redacted`) whenever it is path-like, so a published
/// schematic never leaks the directory layout of the machine that rendered it.
pub(crate) const FILE_VALUED_FLAGS: &[&str] =
    &["--dictionary", "--dictionary-context", "--geojson"];

/// The `viz smart` invocation as the schematic records it, captured from the raw argv in `run`
/// (before any flag is rewritten, e.g. a deferred `--geojson auto` resolving to a cache path).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SourceArgs {
    /// Arguments after `viz smart`, without the input positional and `-o/--output`.
    pub(crate) viz_args: Vec<String>,
    /// The file-valued flags in `viz_args` whose value was reduced to a basename.
    pub(crate) redacted: Vec<String>,
    /// The raw `<input>` positional, exactly as typed (`None` for stdin).
    pub(crate) input:    Option<String>,
}

impl SourceArgs {
    /// Split `argv` (the full `qsv viz smart ...` argv `run` receives) into the recorded
    /// arguments. `takes_value` reports a flag's arity per the docopt USAGE text.
    pub(crate) fn from_argv(argv: &[&str], takes_value: impl Fn(&str) -> bool) -> Self {
        // skip `qsv viz smart`: everything up to and including the first `smart` that follows
        // `viz` (or, failing that, the first `smart` at all)
        let start = argv
            .windows(2)
            .position(|w| matches!(w, ["viz", "smart"]))
            .map(|i| i + 2)
            .or_else(|| argv.iter().position(|a| *a == "smart").map(|i| i + 1))
            .unwrap_or(0);

        let mut out = Self::default();
        let mut after_dashdash = false;
        let mut i = start;
        while i < argv.len() {
            let arg = argv[i];
            i += 1;
            if after_dashdash || arg == "-" || !arg.starts_with('-') {
                // a positional: the first is `<input>` (recorded, never emitted)
                if out.input.is_none() {
                    out.input = Some(arg.to_string());
                } else {
                    if after_dashdash && !out.viz_args.iter().any(|a| a == "--") {
                        out.viz_args.push("--".to_string());
                    }
                    out.viz_args.push(arg.to_string());
                }
                continue;
            }
            if arg == "--" {
                after_dashdash = true;
                continue;
            }
            if let Some(long) = arg.strip_prefix("--") {
                let (name, inline) = match long.split_once('=') {
                    Some((n, v)) => (n, Some(v)),
                    None => (long, None),
                };
                let flag = format!("--{name}");
                let value_follows = inline.is_none() && takes_value(&flag);
                let value = if value_follows {
                    let v = argv.get(i).copied();
                    if v.is_some() {
                        i += 1;
                    }
                    v
                } else {
                    inline
                };
                if flag == "--output" {
                    continue;
                }
                let value = match value {
                    Some(v) if FILE_VALUED_FLAGS.contains(&flag.as_str()) => {
                        let (kept, redacted) = redact_file_value(&flag, v);
                        if redacted && !out.redacted.contains(&flag) {
                            out.redacted.push(flag.clone());
                        }
                        Some(kept)
                    },
                    other => other.map(str::to_string),
                };
                match (inline.is_some(), value) {
                    (true, Some(v)) => out.viz_args.push(format!("{flag}={v}")),
                    (false, Some(v)) => {
                        out.viz_args.push(flag);
                        out.viz_args.push(v);
                    },
                    (_, None) => out.viz_args.push(flag),
                }
                continue;
            }
            // a short flag or a cluster of them (`-n`, `-d ';'`, `-d';'`, `-nd ';'`, `-oout.html`,
            // `-no/x/out.html`), walked letter by letter as docopt does: the first letter that
            // takes a value claims the rest of the cluster, or the next argument when nothing is
            // left, as its value. Preceding switch letters are kept as flags of their own. No
            // file-valued option has a short alias except `-o`, which is dropped with its value.
            let letters = &arg[1..];
            let mut valued: Option<(String, Option<String>)> = None;
            for (pos, ch) in letters.char_indices() {
                let flag = format!("-{ch}");
                if takes_value(&flag) {
                    let rest = &letters[pos + ch.len_utf8()..];
                    let value = if rest.is_empty() {
                        let next = argv.get(i).map(|v| (*v).to_string());
                        if next.is_some() {
                            i += 1;
                        }
                        next
                    } else {
                        Some(rest.to_string())
                    };
                    valued = Some((flag, value));
                    break;
                }
                out.viz_args.push(flag);
            }
            if let Some((flag, value)) = valued
                && flag != "-o"
            {
                out.viz_args.push(flag);
                out.viz_args.extend(value);
            }
        }
        out
    }
}

/// The value `flag` is recorded with, and whether it was reduced to a basename. Keywords
/// (`--dictionary infer`, `--geojson auto|census:county|<shortcut>`) and URLs are not paths and
/// are kept verbatim; anything path-like (a separator, or an existing file) keeps only its
/// basename. `--dictionary-context` always names a file.
fn redact_file_value(flag: &str, value: &str) -> (String, bool) {
    let is_url = value.contains("://");
    let path_like = flag == "--dictionary-context"
        || (!is_url && (value.contains('/') || value.contains('\\') || Path::new(value).is_file()));
    if !path_like {
        return (value.to_string(), false);
    }
    let base = value
        .rsplit(['/', '\\'])
        .find(|s| !s.is_empty())
        .unwrap_or(value)
        .to_string();
    (base, true)
}

/// The input as a plain, readable local file — the only kind of input the schematic
/// fingerprints. `None` for stdin, URLs (`http(s)://`, `ckan://`, ...), `dc:` disk-cache handles
/// and anything that is not a regular file.
pub(crate) fn local_input_file(input: Option<&str>) -> Option<&Path> {
    let input = input?;
    if input == "-" || input.contains("://") || input.starts_with("dc:") {
        return None;
    }
    let path = Path::new(input);
    std::fs::metadata(path)
        .is_ok_and(|m| m.is_file())
        .then_some(path)
}

/// BLAKE3 of a file's raw bytes: `(lowercase hex digest, byte count)`. Memory-mapped and hashed
/// on all cores (`util::hash_blake3_file`), so the extra pass over the input stays cheap.
pub(crate) fn blake3_file(path: &Path) -> crate::CliResult<(String, u64)> {
    let bytes = std::fs::metadata(path)?.len();
    Ok((crate::util::hash_blake3_file(path)?, bytes))
}

/// The `qsv-schematic-source` provenance object.
pub(crate) fn source_json(
    src: &SourceArgs,
    fingerprint: Option<(String, u64)>,
    rows: u64,
    pid: Option<&str>,
    source: Option<serde_json::Value>,
) -> serde_json::Value {
    use serde_json::{Value, json};

    let mut obj = serde_json::Map::new();
    obj.insert("schema".into(), json!(1));
    obj.insert("qsv_version".into(), json!(env!("CARGO_PKG_VERSION")));
    obj.insert(
        "schematic_uuid".into(),
        json!(uuid::Uuid::new_v4().to_string()),
    );
    // omitted, not null, when the input is not a plain local file (stdin, URL, `dc:` handle)
    if let Some((digest, bytes)) = fingerprint {
        obj.insert(
            "fingerprint".into(),
            json!({"algo": "blake3", "digest": digest, "bytes": bytes}),
        );
    }
    // omitted, not null, unless the data is a public download (see `public_source`)
    if let Some(source) = source {
        obj.insert("source".into(), source);
    }
    obj.insert("rows".into(), json!(rows));
    obj.insert("viz_args".into(), json!(src.viz_args));
    obj.insert("redacted".into(), json!(src.redacted));
    obj.insert("pid".into(), json!(pid));
    Value::Object(obj)
}

/// Where a `dc:` input's data came from, for the provenance `source` object: the cache entry's
/// source as given to `qsv get`, when [`public_source`] allows it. Reads the cache index only.
#[cfg(feature = "get")]
pub(crate) fn dc_source(input: Option<&str>) -> Option<serde_json::Value> {
    let name = input?.strip_prefix("dc:")?;
    let entry = crate::diskcache::dc_entry_meta(name).ok().flatten()?;
    public_source(
        &entry.source_uri,
        entry.anonymous_fetch,
        entry.ckan_api_url.as_deref(),
    )
}

/// Query keys (lower-cased, as substrings) that mark a URL as carrying a credential: API
/// keys, tokens, and presigned-URL signatures (S3 `X-Amz-*`, GCS `X-Goog-*`, Azure SAS `sig`).
#[cfg(feature = "get")]
const CREDENTIAL_QUERY_MARKERS: &[&str] = &[
    "token",
    "key",
    "secret",
    "passw",
    "sig",
    "credential",
    "auth",
    "session",
    "x-amz-",
    "x-goog-",
];

/// The `{"url", "ckan_api"?}` provenance object for data downloaded from `uri`, or `None`
/// unless anyone can download it the same way. A published schematic must not reveal where
/// private data lives, so this records only:
///
/// - a download that sent no credentials (`anonymous_fetch == Some(true)`: no CKAN API token;
///   `None`, from an older cache entry or a local/cloud source, is not enough), from
/// - an `http(s)://`, `ckan://` or `dathere://` source (never a local path, nor `s3://`, `gs://`,
///   `az://`, which name infrastructure and are credentialed by nature),
/// - with no user info and no query key that looks like a credential, checked against the request
///   `qsv get` expands the source into, and a credential-free CKAN API URL.
#[cfg(feature = "get")]
pub(crate) fn public_source(
    uri: &str,
    anonymous_fetch: Option<bool>,
    ckan_api: Option<&str>,
) -> Option<serde_json::Value> {
    if anonymous_fetch != Some(true) {
        return None;
    }
    let lower = uri.to_ascii_lowercase();
    if lower.starts_with("ckan://") {
        // `qsv get` pastes an id into `resource_show?id=<id>` unencoded, so a `&` or `=` in it
        // would add query keys; a `<name>?` search is URL-encoded, so any name is safe
        let rest = uri["ckan://".len()..].trim();
        if !rest.ends_with('?') && rest.contains(['&', '=', '#']) {
            return None;
        }
        let mut obj = serde_json::json!({ "url": uri });
        if let Some(api) = ckan_api {
            // a credentialed API URL is private infrastructure: record nothing, not a bare id
            if !credential_free_http_url(api) {
                return None;
            }
            obj["ckan_api"] = serde_json::json!(api);
        }
        return Some(obj);
    }
    if lower.starts_with("dathere://") {
        // expands to a raw GitHub URL with the rest as its path: check it as that request
        let expanded = format!("https://raw.invalid/{}", &uri["dathere://".len()..]);
        return credential_free_http_url(&expanded).then(|| serde_json::json!({ "url": uri }));
    }
    credential_free_http_url(uri).then(|| serde_json::json!({ "url": uri }))
}

#[cfg(feature = "get")]
fn credential_free_http_url(value: &str) -> bool {
    let Ok(url) = url::Url::parse(value) else {
        return false;
    };
    matches!(url.scheme(), "http" | "https")
        && url.username().is_empty()
        && url.password().is_none()
        && !url.query_pairs().any(|(k, _)| {
            let k = k.to_ascii_lowercase();
            CREDENTIAL_QUERY_MARKERS.iter().any(|m| k.contains(m))
        })
}

/// The `qsv-clientd` port the bridge probes: `QSV_CLIENTD_PORT` when it is a valid non-zero
/// port, else [`DEFAULT_CLIENTD_PORT`].
pub(crate) fn clientd_port() -> u16 {
    std::env::var("QSV_CLIENTD_PORT")
        .ok()
        .and_then(|v| v.trim().parse::<u16>().ok())
        .filter(|p| *p != 0)
        .unwrap_or(DEFAULT_CLIENTD_PORT)
}

/// The bridge chrome (CSS + script) for a smart HTML page.
pub(crate) fn bridge_chrome(port: u16) -> String {
    format!(
        "<style id=\"qsv-live-style\">{BRIDGE_CSS}</style>\n<script \
         id=\"qsv-daemon-bridge\">{}</script>",
        BRIDGE_JS.replace("__QSV_CLIENTD_PORT__", &port.to_string())
    )
}

// Raw strings throughout (see `smart_html_page` for why escaped templates are avoided).
const BRIDGE_CSS: &str = r"
.qsv-live-chip, .qsv-live-panel, .qsv-live-dialog, .qsv-live-toast {
  position: fixed; z-index: 2147482000; font: 13px/1.4 system-ui, -apple-system, 'Segoe UI', sans-serif;
  color: var(--qsv-page-ink, #222); background: var(--qsv-page-bg, #fff);
  border: 1px solid var(--qsv-link, #3b6fb6); border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,.18);
}
.qsv-live-chip { left: 12px; bottom: 12px; padding: 4px 12px; border-radius: 999px; cursor: pointer; color: var(--qsv-link, #3b6fb6); font-weight: 600; }
.qsv-live-chip:hover, .qsv-live-chip:focus-visible { background: color-mix(in srgb, var(--qsv-link, #3b6fb6) 12%, var(--qsv-page-bg, #fff)); }
.qsv-live-panel { left: 12px; bottom: 12px; width: min(560px, calc(100vw - 24px)); max-height: 70vh; display: flex; flex-direction: column; }
.qsv-live-head { display: flex; align-items: center; gap: 8px; padding: 6px 10px; border-bottom: 1px solid rgba(127,127,127,.3); }
.qsv-live-head strong { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.qsv-live-body { padding: 8px 10px; overflow: auto; display: flex; flex-direction: column; gap: 8px; }
.qsv-live-panel.qsv-live-collapsed .qsv-live-body { display: none; }
.qsv-live-row { display: flex; flex-wrap: wrap; align-items: center; gap: 6px; }
.qsv-live-panel button, .qsv-live-dialog button, .qsv-live-panel select, .qsv-live-panel input {
  font: inherit; color: inherit; background: transparent; border: 1px solid rgba(127,127,127,.5); border-radius: 4px; padding: 2px 8px;
}
.qsv-live-panel input { min-width: 0; flex: 1; }
.qsv-live-panel button:hover, .qsv-live-dialog button:hover { border-color: var(--qsv-link, #3b6fb6); }
.qsv-live-panel button.qsv-live-hot { border-color: #d9822b; box-shadow: 0 0 0 2px rgba(217,130,43,.35); }
.qsv-live-status { opacity: .8; }
.qsv-live-out { margin: 0; max-height: 40vh; overflow: auto; font: 12px/1.35 ui-monospace, Menlo, Consolas, monospace; white-space: pre; }
.qsv-live-out table { border-collapse: collapse; }
.qsv-live-out th, .qsv-live-out td { border: 1px solid rgba(127,127,127,.35); padding: 1px 6px; text-align: left; vertical-align: top; }
.qsv-live-grid { white-space: normal; font: 12px/1.35 system-ui, -apple-system, 'Segoe UI', sans-serif; }
/* DataTables' scrollY body is the one scroller; at its ends the wheel stops instead of moving the page */
.qsv-live-out.qsv-live-grid { max-height: none; overflow: visible; }
.qsv-live-grid .dt-scroll-body { overscroll-behavior: contain; }
.qsv-live-grid td { white-space: pre; font-family: ui-monospace, Menlo, Consolas, monospace; }
.qsv-live-dialog { left: 50%; top: 20%; transform: translateX(-50%); width: min(460px, calc(100vw - 24px)); padding: 14px 16px; }
.qsv-live-code { font: 600 22px ui-monospace, Menlo, Consolas, monospace; letter-spacing: 2px; text-align: center; margin: 6px 0; }
.qsv-live-cmd { display: block; user-select: all; -webkit-user-select: all; font: 13px ui-monospace, Menlo, Consolas, monospace; padding: 6px 8px; border-radius: 4px; background: rgba(127,127,127,.15); margin: 6px 0; }
.qsv-live-note { font-size: 12px; opacity: .85; }
.qsv-live-toast { right: 12px; bottom: 56px; max-width: min(420px, calc(100vw - 24px)); padding: 8px 12px; border-color: #c0392b; }
/* DateTime mounts the Advanced Filter's date picker on body at z-index 2050: lift it over the panel */
div.dt-datetime { --dt-datetime_zindex: 2147482001; }
";

const BRIDGE_JS: &str = r#"
(function () {
  "use strict";
  // no once-per-window guard: Re-render replaces the page with document.write, which keeps the
  // same window, and the re-rendered page's bridge must run to reconnect
  if (typeof fetch !== "function") return;
  // Only a schematic opened from disk or served from loopback can be paired (qsv-clientd's
  // browser_origins defaults to ["null"]). A hosted copy must never contact the viewer's machine:
  // the probe could not succeed, and browsers' local-network protections would prompt every
  // viewer. So anywhere else: no fetch, no UI, no timers, no stored-token resume.
  var LOOPBACK_HOSTS = ["127.0.0.1", "localhost", "[::1]", "::1"];
  if (!(location.protocol === "file:" || LOOPBACK_HOSTS.indexOf(location.hostname) !== -1)) return;
  var BASE = "http://127.0.0.1:" + __QSV_CLIENTD_PORT__;
  var KEY_PREFIX = "qsv-live-ds-";
  var COMMANDS = ["count", "headers", "stats", "frequency", "slice", "search", "select", "sort",
    "dedup", "sample", "table"];
  var SRC = {};
  try {
    var srcEl = document.getElementById("qsv-schematic-source");
    SRC = (srcEl && JSON.parse(srcEl.textContent)) || {};
  } catch (e) { SRC = {}; }
  var DIGEST = (SRC.fingerprint && SRC.fingerprint.digest) || null;
  var live = { token: null, dataset: null, chip: null, panel: null, out: null, status: null,
    rerender: null, dialog: null, pollTimer: null, title: null, table: null, gen: 0,
    req: 0, pairGen: 0, pairing: false };

  function el(tag, cls, text) {
    var e = document.createElement(tag);
    if (cls) e.className = cls;
    if (text !== undefined && text !== null) e.textContent = String(text);
    return e;
  }
  function button(text, onClick) {
    var b = el("button", null, text);
    b.type = "button";
    b.addEventListener("click", onClick);
    return b;
  }
  function store() { try { return window.sessionStorage; } catch (e) { return null; } }

  function call(method, path, body, opts) {
    opts = opts || {};
    var headers = {};
    var tok = opts.anon ? null : (opts.token || live.token);
    if (tok) headers.Authorization = "Bearer " + tok;
    var init = { method: method, mode: "cors", credentials: "omit", cache: "no-store",
      headers: headers };
    if (body !== undefined) {
      headers["Content-Type"] = "application/json";
      init.body = JSON.stringify(body);
    }
    return fetch(BASE + path, init).then(function (r) {
      return r.text().then(function (t) {
        if (r.ok && opts.raw) return t;
        var j = null;
        try { j = t ? JSON.parse(t) : null; } catch (e) { j = null; }
        if (!r.ok) {
          var d = (j && j.daemon_error) || {};
          var err = new Error(d.message || ("qsv-clientd replied HTTP " + r.status));
          err.status = r.status;
          err.code = d.code;
          throw err;
        }
        return j;
      });
    });
  }

  var toastTimer = null;
  function toast(msg) {
    var t = document.getElementById("qsv-live-toast");
    if (!t) {
      t = el("div", "qsv-live-toast");
      t.id = "qsv-live-toast";
      t.setAttribute("role", "alert");
      document.body.appendChild(t);
    }
    t.textContent = String(msg);
    t.hidden = false;
    clearTimeout(toastTimer);
    toastTimer = setTimeout(function () { t.hidden = true; }, 7000);
  }
  function fail(e) {
    if (e && e.status === 401) { dropToken(); return; }
    toast((e && e.message) || String(e));
  }

  function hello() {
    var ctl = typeof AbortController === "function" ? new AbortController() : null;
    // A closed port fails at once; the timeout only bounds a listener that never answers. It is
    // wall-clock time on a main thread that may still be busy rendering plotly, hence not tighter.
    var timer = setTimeout(function () { if (ctl) ctl.abort(); }, 1500);
    var init = { mode: "cors", credentials: "omit", cache: "no-store" };
    if (ctl) init.signal = ctl.signal;
    return fetch(BASE + "/v1/hello", init)
      .then(function (r) { return r.ok ? r.json() : null; })
      .then(function (j) {
        clearTimeout(timer);
        return !!(j && j.service === "qsv-clientd" && j.api === 1);
      }, function () { clearTimeout(timer); return false; });
  }

  // ---- stored tokens: keyed by dataset id, so a re-render (new schematic_uuid) or a fetch (new
  // digest) keeps the connection in this tab
  function storedTokens() {
    var s = store(), out = [];
    if (!s) return out;
    for (var i = 0; i < s.length; i++) {
      var k = s.key(i);
      if (!k || k.indexOf(KEY_PREFIX) !== 0) continue;
      try {
        var v = JSON.parse(s.getItem(k));
        if (v && v.token && v.dataset && v.dataset.id) out.push({ key: k, token: v.token, id: v.dataset.id });
      } catch (e) { /* ignore a malformed entry */ }
    }
    return out;
  }
  function saveToken() {
    var s = store();
    if (!s || !live.token || !live.dataset) return;
    try {
      s.setItem(KEY_PREFIX + live.dataset.id, JSON.stringify({ token: live.token, dataset: live.dataset }));
    } catch (e) { /* storage full or blocked: the connection just won't survive a reload */ }
  }
  function dropToken() {
    var s = store();
    if (s && live.dataset) { try { s.removeItem(KEY_PREFIX + live.dataset.id); } catch (e) {} }
    live.token = null;
    live.dataset = null;
    live.req++;
    if (live.panel) { live.panel.remove(); live.panel = null; }
    toast("The qsv-clientd connection expired or was revoked. Connect again to continue.");
    showChip();
  }
  // Resume only a stored token whose dataset is THIS page's data (its digest equals the page's
  // fingerprint). Tabs on one loopback origin share sessionStorage, so a token paired by another
  // schematic must not silently connect this page to that dataset; non-matching entries are left
  // alone (they may belong to another page). A page without a fingerprint never auto-resumes.
  // After a Re-render the daemon records the fingerprint of the data it rendered from, which is
  // the new page's own fingerprint, so a re-rendered page still reconnects.
  function resume() {
    if (!DIGEST) return Promise.resolve(false);
    var cands = storedTokens();
    var found = [];
    return cands.reduce(function (p, c) {
      return p.then(function () {
        return call("GET", "/v1/datasets/" + encodeURIComponent(c.id), undefined, { token: c.token })
          .then(function (ds) { if (ds && ds.id) found.push({ token: c.token, dataset: ds }); },
            function (e) {
              if (e && e.status === 401) { try { store().removeItem(c.key); } catch (x) {} }
            });
      });
    }, Promise.resolve()).then(function () {
      for (var i = 0; i < found.length; i++) {
        var fp = found[i].dataset.fingerprint;
        if (fp && fp.digest === DIGEST) {
          connected(found[i].token, found[i].dataset);
          return true;
        }
      }
      return false;
    });
  }

  // ---- the chip and the pairing dialog
  function showChip() {
    if (live.chip) { live.chip.hidden = false; return; }
    var c = button("Live data: connect to qsv-clientd", startPair);
    c.className = "qsv-live-chip";
    c.id = "qsv-live-chip";
    c.title = "A qsv-clientd daemon is running on this computer. Connecting needs your approval in a shell.";
    document.body.appendChild(c);
    live.chip = c;
  }
  // Cancel can land while a poll is in flight: pairGen makes its answer, even an approval, a no-op
  // (and keeps it off a pairing dialog opened afterwards)
  function closeDialog() {
    live.pairGen++;
    clearTimeout(live.pollTimer);
    if (live.dialog) { live.dialog.remove(); live.dialog = null; }
  }
  function startPair() {
    // the chip stays clickable until the dialog exists: a second click meanwhile would open a
    // second dialog that no Cancel can close
    if (live.dialog || live.pairing) return;
    live.pairing = true;
    var body = {};
    if (SRC.schematic_uuid) body.schematic_uuid = SRC.schematic_uuid;
    if (DIGEST) body.digest = DIGEST;
    call("POST", "/v1/pair", body, { anon: true }).then(function (j) {
      live.pairing = false;
      if (!j || !j.request_id || !j.code) throw new Error("qsv-clientd sent an unexpected pairing reply");
      var d = el("div", "qsv-live-dialog");
      d.id = "qsv-live-dialog";
      d.setAttribute("role", "dialog");
      d.setAttribute("aria-label", "Connect to qsv-clientd");
      d.appendChild(el("strong", null, "Connect this Data Schematic to qsv-clientd"));
      d.appendChild(el("div", "qsv-live-code", j.code));
      d.appendChild(el("div", null, "To approve, run this in a shell:"));
      d.appendChild(el("code", "qsv-live-cmd", j.approve_with || ("qsv-clientd approve " + j.code)));
      d.appendChild(el("div", "qsv-live-note",
        "If approval reports that the request matches no dataset, register the data first: " +
        "qsv-clientd register <data file> --schematic <this html file>, then connect again."));
      var st = el("div", "qsv-live-status", "Waiting for approval\u2026");
      d.appendChild(st);
      var row = el("div", "qsv-live-row");
      row.appendChild(button("Cancel", closeDialog));
      d.appendChild(row);
      document.body.appendChild(d);
      live.dialog = d;
      poll(j.request_id, Date.now(), st, live.pairGen);
    }).catch(function (e) { live.pairing = false; fail(e); });
  }
  function poll(id, t0, st, g) {
    if (Date.now() - t0 > 300000) {
      st.textContent = "The pairing code expired. Close this and connect again.";
      return;
    }
    live.pollTimer = setTimeout(function () {
      if (g !== live.pairGen) return;
      call("GET", "/v1/pair/" + encodeURIComponent(id), undefined, { anon: true }).then(function (j) {
        if (g !== live.pairGen) return;
        var s = j && j.status;
        if (s === "approved" && j.token && j.dataset) {
          closeDialog();
          connected(j.token, j.dataset);
        } else if (s === "denied") {
          st.textContent = "The pairing request was denied.";
        } else if (s === "unknown") {
          st.textContent = "qsv-clientd no longer knows this request (it expired or the daemon restarted). Connect again.";
        } else {
          poll(id, t0, st, g);
        }
      }, function (e) { if (g === live.pairGen) st.textContent = (e && e.message) || String(e); });
    }, 1500);
  }

  // ---- the connected panel
  function setStatus(msg) { if (live.status) live.status.textContent = msg; }
  // DataTables keeps every instance, with its rows and a ResizeObserver, in global settings
  // until it is destroyed, whatever happens to its DOM
  function dropTable() {
    live.gen++;  // answers still on their way for the old table are dropped
    if (!live.table) return;
    try { live.table.destroy(); } catch (e) {}
    live.table = null;
  }
  function showText(text, note) {
    dropTable();
    live.out.textContent = "";
    var pre = el("pre", "qsv-live-out", text);
    live.out.appendChild(pre);
    if (note) setStatus(note);
  }
  // Paged, sortable and searchable by the page's own DataTables, the one the data viewer
  // embeds, when it has one (not with --preview-threshold 0); else a plain table.
  var PLAIN_MAX = 500;
  var DRILL_LIMIT = 500;  // qsv-clientd's /rows maximum
  // The data viewer's export: every filtered row, not the page, with the CSV-injection guard
  // restated (supplying exportOptions replaces csvHtml5's own default). The Select extension is
  // not in the bundle, so nothing narrows the export to selected rows.
  function exportButton(name) {
    name = String(name || "rows").replace(/[^\p{L}\p{N}._-]+/gu, "_")
      .replace(/^[._]+|_+$/g, "").slice(0, 80) || "rows";
    return {
      extend: "csv",
      text: "Export CSV",
      filename: "qsv-live-" + name,
      // scrollX gives the table an empty tfoot, written as a blank line at the end of the file
      footer: false,
      exportOptions: { escapeExcelFormula: true }
    };
  }
  // With a stored result (qsv-clientd kept output too big to send whole), every page, search
  // and column sort is asked of the daemon, which runs qsv on the stored file.
  // exportName: the export file's name, for rows held in the page
  function showTable(headers, rows, note, result, exportName) {
    dropTable();
    var gen = live.gen;
    live.out.textContent = "";
    var wrap = el("div", "qsv-live-out qsv-live-grid");
    var table = el("table");
    var thead = el("thead"), tr = el("tr");
    headers.forEach(function (h) { tr.appendChild(el("th", null, h)); });
    thead.appendChild(tr);
    table.appendChild(thead);
    wrap.appendChild(table);
    live.out.appendChild(wrap);
    setStatus(note);
    function plain() {
      if (!table.isConnected) return;
      var tbody = el("tbody");
      rows.slice(0, PLAIN_MAX).forEach(function (r) {
        var row = el("tr");
        (r || []).forEach(function (v) { row.appendChild(el("td", null, v)); });
        tbody.appendChild(row);
      });
      table.appendChild(tbody);
      var total = result ? result.total : rows.length;
      var shown = Math.min(rows.length, PLAIN_MAX);
      if (total > shown) setStatus(note + " \u2014 showing the first " + shown);
    }
    var ensure = window.qsvEnsureDataTables;
    if (typeof ensure !== "function") return plain();
    ensure().then(function () {
      if (!table.isConnected) return;  // another result replaced this one meanwhile
      var DT = window.DataTable;
      if (typeof DT !== "function") return plain();
      var opts = {
        // render.text(): cells are data, never markup
        columns: headers.map(function () { return { render: DT.render.text(), defaultContent: "" }; }),
        order: [], pageLength: 50, lengthChange: false, scrollX: true,
        // the rows scroll in the panel; 10 a page always fit, so the wheel scrolled the page
        scrollY: "30vh", scrollCollapse: true
      };
      if (result) {
        opts.serverSide = true;
        opts.processing = true;
        opts.orderMulti = false;  // qsv-clientd sorts by one column
        opts.searchDelay = 400;  // one qsv search per pause in typing, not per keystroke
        opts.ajax = function (data, callback) {
          var o = data.order && data.order[0];
          var body = { result: result.id, start: data.start, length: data.length,
            search: (data.search && data.search.value) || "" };
          if (o) body.order = { column: o.column, desc: o.dir === "desc" };
          call("POST", dsPath("/page"), body).then(function (j) {
            if (gen !== live.gen) return;  // a newer result replaced this table
            j = j || {};
            callback({ draw: data.draw, recordsTotal: j.total || 0,
              recordsFiltered: j.filtered || 0, data: j.rows || [] });
          }, function (e) {
            if (gen !== live.gen) return;
            callback({ draw: data.draw, recordsTotal: result.total, recordsFiltered: 0, data: [] });
            // replaced by a newer result, or the data changed since: the query must run again
            if (e && (e.status === 409 || e.status === 410)) setStatus(e.message);
            else fail(e);
          });
        };
      } else {
        opts.data = rows;
        opts.deferRender = true;
        // Rows held in the page get the data viewer's Advanced Filter and CSV export. A stored
        // result has neither: both would act on the one page in hand, not the whole result.
        opts.language = {
          searchBuilder: { button: { 0: "Advanced Filter", _: "Advanced Filter (%d)" } }
        };
        opts.layout = {
          topStart: { buttons: [{
            extend: "searchBuilder",
            // the drawer's date conditions (timestamps compare by their date part)
            config: { conditions: window.qsvSbConditions ? window.qsvSbConditions() : {} }
          }, exportButton(exportName)] }
        };
      }
      live.table = new DT(table, opts);
    }, plain);
  }
  // A stored drill-down of up to READ_ALL_MAX rows is read whole into the page, DRILL_LIMIT rows a
  // /page request, so it gets the Advanced Filter and the CSV export; a larger one is paged by
  // qsv-clientd. A newer query or drill (or a disconnect) stops the reading.
  var READ_ALL_MAX = 10000;
  function showRows(j, label) {
    var rows = j.rows || [];
    var res = j.result;
    var headers = (res && res.headers) || j.headers || [];
    function inPage(all) {
      showTable(headers, all, label + ": " + all.length + " row(s)", null, label);
    }
    function paged() {
      showTable(headers, rows, label + ": " + res.total + " row(s)", res);
    }
    if (res && !(j.offset > 0) && rows.length >= res.total) return inPage(rows);
    // without the data viewer (--preview-threshold 0) there is no DataTables to give the rows
    // to, only the plain table's first PLAIN_MAX
    if (res && !(j.offset > 0) && res.total <= READ_ALL_MAX &&
        typeof window.qsvEnsureDataTables === "function") {
      var r = live.req, all = rows.slice();
      var next = function () {
        if (r !== live.req) return;
        setStatus(label + ": reading " + all.length + " of " + res.total + " row(s)…");
        call("POST", dsPath("/page"),
          { result: res.id, start: all.length, length: DRILL_LIMIT, search: "" }
        ).then(function (p) {
          if (r !== live.req) return;
          var got = (p && p.rows) || [];
          all = all.concat(got);
          if (all.length >= res.total) return inPage(all);
          if (got.length) return next();
          paged();  // the result came up short: let qsv-clientd page it
        }, function () {
          // replaced, or the data changed: the paged table reports it from qsv-clientd
          if (r === live.req) paged();
        });
      };
      return next();
    }
    if (res) return paged();
    showTable(j.headers || [], rows, label + ": " + rows.length + " row(s)" +
      (rows.length >= DRILL_LIMIT ? " (first " + DRILL_LIMIT + ")" : ""), null, label);
  }
  function splitArgs(s) {
    var out = [], cur = "", quoted = false, has = false;
    for (var i = 0; i < s.length; i++) {
      var c = s.charAt(i);
      if (c === '"') { quoted = !quoted; has = true; }
      else if (!quoted && /\s/.test(c)) { if (has) { out.push(cur); cur = ""; has = false; } }
      else { cur += c; has = true; }
    }
    if (has) out.push(cur);
    return out;
  }
  function dsPath(suffix) { return "/v1/datasets/" + encodeURIComponent(live.dataset.id) + suffix; }
  // Only the latest query or drill may show its answer: a slower earlier one, or one still on its
  // way when the page disconnects or reconnects, must not overwrite what came after it
  function ask(suffix, body, done) {
    var r = ++live.req;
    call("POST", dsPath(suffix), body).then(function (j) {
      if (r === live.req) done(j || {});
    }, function (e) { if (r === live.req) fail(e); });
  }

  function drill(column, value) {
    if (!live.token) return;
    if (live.panel) live.panel.classList.remove("qsv-live-collapsed");
    live.colInput.value = column;
    live.valInput.value = value;
    setStatus("Fetching rows where " + column + " = " + value + "\u2026");
    ask("/rows", { column: column, value: value, limit: DRILL_LIMIT },
      function (j) { showRows(j, column + " = " + value); });
  }

  function showSource() {
    var d = live.dataset;
    if (!live.title || !d) return;
    live.title.textContent = "Live: " + (d.title || d.id) + (d.fetched ? " (fetched copy)" : "");
    live.title.title = d.fetched
      ? "qsv-clientd is reading the data it last fetched from the source, not the registered " +
        "file. Run qsv-clientd register <file> to switch back to the file."
      : "";
  }
  function connected(token, dataset) {
    live.token = token;
    live.dataset = dataset;
    live.req++;
    saveToken();
    if (live.chip) live.chip.hidden = true;
    dropTable();
    if (live.panel) live.panel.remove();
    var p = el("div", "qsv-live-panel");
    p.id = "qsv-live-panel";
    p.setAttribute("role", "region");
    p.setAttribute("aria-label", "qsv-clientd live data");
    var head = el("div", "qsv-live-head");
    live.title = el("strong");
    head.appendChild(live.title);
    showSource();
    head.appendChild(button("\u2013", function () {
      p.classList.toggle("qsv-live-collapsed");
      this.textContent = p.classList.contains("qsv-live-collapsed") ? "+" : "\u2013";
    }));
    p.appendChild(head);
    var body = el("div", "qsv-live-body");

    var actions = el("div", "qsv-live-row");
    actions.appendChild(button("Refresh", function () {
      setStatus("Refreshing\u2026");
      call("POST", dsPath("/refresh"), {}).then(function (j) {
        j = j || {};
        setStatus("Rows: " + j.rows + (j.changed ? " \u2014 the data changed since this schematic was rendered; Re-render to update the charts."
          : " \u2014 unchanged since this schematic was rendered."));
        if (j.changed && live.rerender) live.rerender.classList.add("qsv-live-hot");
      }).catch(fail);
    }));
    if (dataset.can_render !== false) {
      live.rerender = button("Re-render", function () {
        setStatus("Re-rendering (this can take a while)\u2026");
        call("POST", dsPath("/render"), {}, { raw: true }).then(function (html) {
          saveToken();
          // document.write keeps this window, and with it window.DataTable and every instance
          // (ours and the data viewer's) with its rows, which the new page can't reach
          dropTable();
          try { if (window.DataTable) window.DataTable.tables({ api: true }).destroy(); } catch (e) {}
          document.open();
          document.write(html);
          document.close();
        }).catch(fail);
      });
      actions.appendChild(live.rerender);
    }
    if (dataset.can_fetch) {
      actions.appendChild(button("Fetch latest", function () {
        setStatus("Fetching the latest data\u2026");
        call("POST", dsPath("/fetch"), {}).then(function (j) {
          j = j || {};
          setStatus(j.changed ? "Fetched new data" + (j.via ? " via " + j.via : "") + ". Re-render to update the charts."
            : "The source has not changed.");
          // a fetch may have switched the daemon to the fetched copy
          call("GET", dsPath(""), undefined).then(function (d) {
            if (d) { live.dataset = d; showSource(); }
          }, function () {});
          if (j.changed && live.rerender) live.rerender.classList.add("qsv-live-hot");
        }).catch(fail);
      }));
    }
    actions.appendChild(button("Disconnect", function () {
      // withdraw the token on the daemon too, which also deletes this page's stored result
      // (an older qsv-clientd without the route answers 404: nothing to do then)
      call("POST", dsPath("/disconnect"), {}).catch(function () {});
      var s = store();
      if (s) { try { s.removeItem(KEY_PREFIX + live.dataset.id); } catch (e) {} }
      live.token = null;
      live.dataset = null;
      live.req++;
      dropTable();
      p.remove();
      live.panel = null;
      showChip();
    }));
    body.appendChild(actions);

    var q = el("form", "qsv-live-row");
    var sel = el("select");
    sel.setAttribute("aria-label", "qsv command");
    COMMANDS.forEach(function (c) { var o = el("option", null, c); o.value = c; sel.appendChild(o); });
    var argIn = el("input");
    argIn.type = "text";
    argIn.placeholder = "arguments, e.g. -s col --limit 5";
    argIn.setAttribute("aria-label", "command arguments");
    var run = el("button", null, "Run");
    run.type = "submit";
    q.appendChild(sel);
    q.appendChild(argIn);
    q.appendChild(run);
    q.addEventListener("submit", function (ev) {
      ev.preventDefault();
      var cmd = sel.value;
      setStatus("Running qsv " + cmd + "\u2026");
      ask("/query", { command: cmd, args: splitArgs(argIn.value) }, function (j) {
        var cut = j.truncated ? " \u2014 output truncated at qsv-clientd's size limit" : "";
        if (j.result) {
          showTable(j.result.headers || j.headers || [], j.rows || [],
            "qsv " + cmd + ": " + j.result.total + " row(s)", j.result);
        } else if (j.headers && j.rows) {
          showTable(j.headers, j.rows, "qsv " + cmd + ": " + j.rows.length + " row(s)" + cut, null,
            cmd);
        } else {
          showText(j.stdout || "", "qsv " + cmd + cut);
        }
      });
    });
    body.appendChild(q);

    var rows = el("form", "qsv-live-row");
    live.colInput = el("input");
    live.colInput.placeholder = "column";
    live.colInput.setAttribute("aria-label", "column");
    live.valInput = el("input");
    live.valInput.placeholder = "value (click a bar to fill)";
    live.valInput.setAttribute("aria-label", "value");
    var show = el("button", null, "Rows");
    show.type = "submit";
    rows.appendChild(live.colInput);
    rows.appendChild(live.valInput);
    rows.appendChild(show);
    rows.addEventListener("submit", function (ev) {
      ev.preventDefault();
      drill(live.colInput.value, live.valInput.value);
    });
    body.appendChild(rows);

    live.status = el("div", "qsv-live-status", "Connected. Click a bar in a frequency chart to see its rows.");
    body.appendChild(live.status);
    live.out = el("div");
    body.appendChild(live.out);
    p.appendChild(body);
    document.body.appendChild(p);
    live.panel = p;
  }

  // ---- drill-down: frequency charts are marked with data-qsv-column (one chart per element) or,
  // on the single-plot grid, data-qsv-axis-columns (x-axis id -> column). Each also lists the
  // exact category keys of its synthetic null/"Other" bars (data-qsv-agg-keys, or
  // data-qsv-axis-agg-keys per axis), which have no rows to show; only those are skipped, since a
  // real category may contain the same zero-width character those keys end in. Plotly renders
  // lazily, so retry until each graph div has grown its event API.
  function barValue(ev, aggKeys) {
    var pt = ev && ev.points && ev.points[0];
    if (!pt) return null;
    var o = (pt.data && pt.data.orientation) || "v";
    var v = pt.label !== undefined && pt.label !== null ? pt.label : (o === "h" ? pt.y : pt.x);
    if (v === undefined || v === null) return null;
    v = String(v);
    if (aggKeys && aggKeys.indexOf(v) !== -1) return null;
    return v;
  }
  function jsonAttr(node, name, fallback) {
    try {
      var v = JSON.parse(node.getAttribute(name));
      return v === null || v === undefined ? fallback : v;
    } catch (e) { return fallback; }
  }
  function graphOf(node) {
    if (typeof node.on === "function") return node;
    var g = node.querySelector(".js-plotly-plot");
    return g && typeof g.on === "function" ? g : null;
  }
  function bindDrill() {
    var tries = 0;
    (function hook() {
      var pending = false;
      var marked = document.querySelectorAll("[data-qsv-column], [data-qsv-axis-columns]");
      for (var i = 0; i < marked.length; i++) {
        var node = marked[i];
        if (node.__qsvLiveBound) continue;
        var gd = graphOf(node);
        if (!gd) { pending = true; continue; }
        node.__qsvLiveBound = true;
        (function (node, gd) {
          var col = node.getAttribute("data-qsv-column");
          var aggKeys = col !== null ? jsonAttr(node, "data-qsv-agg-keys", []) : null;
          var byAxis = null, aggByAxis = null;
          if (col === null) {
            byAxis = jsonAttr(node, "data-qsv-axis-columns", {});
            aggByAxis = jsonAttr(node, "data-qsv-axis-agg-keys", {});
          }
          var own = Object.prototype.hasOwnProperty;
          gd.on("plotly_click", function (ev) {
            if (!live.token) return;
            var c = col, keys = aggKeys;
            if (byAxis) {
              var pt = ev && ev.points && ev.points[0];
              var ax = pt && ((pt.fullData && pt.fullData.xaxis) || (pt.data && pt.data.xaxis) || "x");
              c = own.call(byAxis, ax) ? byAxis[ax] : null;
              keys = own.call(aggByAxis, ax) ? aggByAxis[ax] : [];
            }
            var v = barValue(ev, keys);
            if (c !== null && v !== null) drill(c, v);
          });
        })(node, gd);
      }
      if (pending && ++tries < 200) setTimeout(hook, 100);
    })();
  }

  function start() {
    hello().then(function (ok) {
      if (!ok) return;
      bindDrill();
      return resume().then(function (resumed) { if (!resumed) showChip(); });
    }).catch(function () { /* stay inert */ });
  }
  // probe once the page (plotly bootstrap included) has loaded, so the probe never competes with it
  if (document.readyState === "complete") start();
  else window.addEventListener("load", start, { once: true });
})();
"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn takes_value(flag: &str) -> bool {
        matches!(
            flag,
            "-o" | "--output"
                | "--dictionary"
                | "--dictionary-context"
                | "--geojson"
                | "--limit"
                | "--dataset-pid"
                | "-d"
                | "--delimiter"
        )
    }

    fn parse(argv: &[&str]) -> SourceArgs {
        SourceArgs::from_argv(argv, takes_value)
    }

    #[test]
    fn strips_input_and_output_in_every_form() {
        for argv in [
            &[
                "qsv", "viz", "smart", "data.csv", "-o", "out.html", "--limit", "5",
            ][..],
            &[
                "qsv", "viz", "smart", "--output", "out.html", "data.csv", "--limit", "5",
            ],
            &[
                "qsv",
                "viz",
                "smart",
                "--output=out.html",
                "--limit",
                "5",
                "data.csv",
            ],
            &[
                "qsv",
                "viz",
                "smart",
                "-oout.html",
                "--limit",
                "5",
                "data.csv",
            ],
        ] {
            let s = parse(argv);
            assert_eq!(s.viz_args, ["--limit", "5"], "{argv:?}");
            assert_eq!(s.input.as_deref(), Some("data.csv"), "{argv:?}");
            assert!(s.redacted.is_empty());
        }
    }

    #[test]
    fn short_option_clusters_are_walked_letter_by_letter() {
        // `-n` is a switch, `-d` and `-o` take values (as in viz's USAGE)
        let s = parse(&["qsv", "viz", "smart", "-nd", ";", "/data/in.csv"]);
        assert_eq!(s.input.as_deref(), Some("/data/in.csv"));
        assert_eq!(s.viz_args, ["-n", "-d", ";"]);

        let s = parse(&["qsv", "viz", "smart", "-no/x/out.html", "/data/in.csv"]);
        assert_eq!(s.input.as_deref(), Some("/data/in.csv"));
        assert_eq!(s.viz_args, ["-n"]);

        let s = parse(&["qsv", "viz", "smart", "-d;", "/data/in.csv"]);
        assert_eq!(s.input.as_deref(), Some("/data/in.csv"));
        assert_eq!(s.viz_args, ["-d", ";"]);

        let s = parse(&[
            "qsv",
            "viz",
            "smart",
            "-n",
            "-o",
            "/x/out.html",
            "/data/in.csv",
        ]);
        assert_eq!(s.input.as_deref(), Some("/data/in.csv"));
        assert_eq!(s.viz_args, ["-n"]);

        // a value-taking letter claims the REST of the cluster, even letters that look like flags
        let s = parse(&["qsv", "viz", "smart", "-dn", "/data/in.csv"]);
        assert_eq!(s.input.as_deref(), Some("/data/in.csv"));
        assert_eq!(s.viz_args, ["-d", "n"]);

        // no path ever reaches viz_args
        for argv in [
            &["qsv", "viz", "smart", "-nd", ";", "/data/in.csv"][..],
            &["qsv", "viz", "smart", "-no/x/out.html", "/data/in.csv"],
            &["qsv", "viz", "smart", "-d;", "/data/in.csv"],
            &[
                "qsv",
                "viz",
                "smart",
                "-n",
                "-o",
                "/x/out.html",
                "/data/in.csv",
            ],
        ] {
            let s = parse(argv);
            assert!(
                s.viz_args.iter().all(|a| !a.contains('/')),
                "{argv:?} -> {:?}",
                s.viz_args
            );
        }
    }

    #[test]
    fn option_values_are_not_mistaken_for_the_input() {
        let s = parse(&[
            "qsv",
            "viz",
            "smart",
            "--dataset-pid",
            "x",
            "-d",
            ";",
            "in.csv",
        ]);
        assert_eq!(s.input.as_deref(), Some("in.csv"));
        assert_eq!(s.viz_args, ["--dataset-pid", "x", "-d", ";"]);
    }

    #[test]
    fn file_values_keep_only_their_basename() {
        let s = parse(&[
            "qsv",
            "viz",
            "smart",
            "in.csv",
            "--dictionary",
            "/secret/dir/dict.json",
            "--geojson=auto",
            "--dictionary-context",
            "notes.md",
        ]);
        assert_eq!(
            s.viz_args,
            [
                "--dictionary",
                "dict.json",
                "--geojson=auto",
                "--dictionary-context",
                "notes.md"
            ]
        );
        assert_eq!(s.redacted, ["--dictionary", "--dictionary-context"]);
    }

    #[test]
    fn keywords_and_urls_are_not_redacted() {
        let s = parse(&[
            "qsv",
            "viz",
            "smart",
            "in.csv",
            "--dictionary",
            "infer",
            "--geojson",
            "https://example.com/a/b.geojson",
        ]);
        assert_eq!(
            s.viz_args,
            [
                "--dictionary",
                "infer",
                "--geojson",
                "https://example.com/a/b.geojson"
            ]
        );
        assert!(s.redacted.is_empty());
    }

    #[test]
    fn stdin_input_is_recorded_and_never_fingerprinted() {
        let s = parse(&["qsv", "viz", "smart", "-", "--limit", "3"]);
        assert_eq!(s.input.as_deref(), Some("-"));
        assert_eq!(s.viz_args, ["--limit", "3"]);
        assert!(local_input_file(Some("-")).is_none());
        assert!(local_input_file(None).is_none());
        assert!(local_input_file(Some("https://example.com/x.csv")).is_none());
        assert!(local_input_file(Some("dc:foo")).is_none());
    }

    #[test]
    fn source_json_omits_the_fingerprint_without_one() {
        let v = source_json(&SourceArgs::default(), None, 3, None, None);
        assert!(v.get("fingerprint").is_none());
        assert!(v.get("source").is_none());
        assert_eq!(v["pid"], serde_json::Value::Null);
        assert_eq!(v["redacted"], serde_json::json!([]));
    }

    #[test]
    fn source_json_records_a_source() {
        let src = serde_json::json!({"url": "https://h.x/d.csv"});
        let v = source_json(&SourceArgs::default(), None, 3, None, Some(src.clone()));
        assert_eq!(v["source"], src);
    }

    #[cfg(feature = "get")]
    #[test]
    fn public_source_records_only_credential_free_downloads() {
        use serde_json::json;
        let api = Some("https://data.example.gov/api/3/action");
        let ok = |uri: &str| public_source(uri, Some(true), None);

        assert_eq!(
            ok("https://data.example.gov/dataset/d/resource/r/download/x.csv"),
            Some(json!({"url": "https://data.example.gov/dataset/d/resource/r/download/x.csv"}))
        );
        assert_eq!(
            ok("https://h.x/q?layer=0&format=csv"),
            Some(json!({"url": "https://h.x/q?layer=0&format=csv"}))
        );
        assert_eq!(
            public_source("ckan://abc", Some(true), api),
            Some(json!({"url": "ckan://abc", "ckan_api": api}))
        );
        assert_eq!(
            public_source("CKAN://abc", Some(true), None),
            Some(json!({"url": "CKAN://abc"}))
        );
        for search in ["ckan://housing?", "ckan://R&D?", "ckan://a=b #2?"] {
            assert_eq!(ok(search), Some(json!({ "url": search })), "{search}");
        }
        // a CKAN API that itself carries credentials is private infrastructure: no source at
        // all, not a bare id that names no site
        for bad_api in [
            "https://u:p@h.x/api/3/action",
            "https://h.x/api/3/action?api_key=S",
        ] {
            assert_eq!(public_source("ckan://abc", Some(true), Some(bad_api)), None);
        }
        // `qsv get` pastes a ckan:// id into resource_show's query unencoded
        for uri in [
            "ckan://abc&api_key=SECRET",
            "ckan://abc&x=1",
            "ckan://abc#frag",
        ] {
            assert_eq!(public_source(uri, Some(true), api), None, "{uri}");
        }
        assert_eq!(
            ok("dathere://boston311.csv"),
            Some(json!({"url": "dathere://boston311.csv"}))
        );
        assert_eq!(ok("dathere://boston311.csv?token=SECRET"), None);
        assert_eq!(ok("dathere://x.csv?X-Amz-Signature=S"), None);

        // fetched with a CKAN token (a private dataset, or a token set for every fetch), or
        // not known to have been fetched anonymously (older entries, local and cloud sources)
        assert_eq!(public_source("ckan://abc", Some(false), api), None);
        assert_eq!(public_source("https://h.x/d.csv", None, None), None);

        for uri in [
            "/home/me/private/data.csv",
            "C:\\Users\\me\\data.csv",
            "data.csv",
            "file:///etc/passwd",
            "s3://bucket/key.csv",
            "gs://bucket/key.csv",
            "az://container/key.csv",
            "ftp://h.x/d.csv",
            "https://user:pass@h.x/d.csv",
            "https://user@h.x/d.csv",
            "https://h.x/d.csv?api_key=1",
            "https://h.x/d.csv?Token=1",
            "https://s3.amazonaws.com/b/k.csv?X-Amz-Signature=abc&X-Amz-Expires=86400",
            "https://acct.blob.core.windows.net/c/k.csv?sv=2022&sig=abc",
            "https://storage.googleapis.com/b/k?X-Goog-Signature=abc",
            "https://h.x/d.csv?auth=1",
            "https://h.x/d.csv?password=1",
            "not a url",
        ] {
            assert_eq!(ok(uri), None, "{uri} must not be recorded");
        }
    }
}
