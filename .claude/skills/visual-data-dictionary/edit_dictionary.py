#!/usr/bin/env python3
"""Fine-tune a describegpt JSON Schema data dictionary for `qsv viz smart`.

A small terminal UI (curses) to review every column and adjust the five fields
that steer the Data Schematic:

    role         x-qsv.role         (dimension | measure | identifier | timestamp)
    concept      x-qsv.concept      (namespaced token, e.g. geo.state, id.natural_key)
    label        title              (shown in the dictionary drawer + hovers)
    description  description        (shown in the dictionary drawer)
    aggregation  x-qsv.aggregation  (sum | mean — how a MEASURE combines in a group)

An inferred dictionary is a DRAFT: the LLM half is not reproducible, so the same
data can come back with different roles run to run, and role decides which panel
a column gets. This tool is how you ratify or correct that draft; the corrected
file — not the model — is what makes a later render reproducible (qsv issue
#4407).

As you edit, each column shows a ROUTE preview (Skip / Dimension / Temporal /
MapCoord / ProjectedCoord / Measure, the last carrying its aggregation as
Measure(sum) or Measure(mean)) derived from its concept, role and — when set —
its explicit aggregation.
This is the concept→role projection BEFORE `viz smart` applies its content_type
and stats/label guardrails, so the final route can differ (unresolved columns
show "Defer→stats"). Treat it as guidance on the effect of a concept/role edit,
not the definitive `viz smart` disposition. Every other key (examples,
cardinality, null_count, qsv_type, min/max, …) is preserved byte-for-byte, and
the file is only rewritten if you actually change something and save.

`aggregation` is the one field qsv can also DROP on read, and the "!" flag is an
exact mirror of when that happens. `honored_agg` reproduces viz's whole read gate
— the token must be `sum`/`mean` (trimmed, ASCII-folded), `qsv_type` must be
Integer/Float or absent, and `role` must be empty or exactly `measure` — and the
value is additionally only applied on a route that resolves to Measure. A value
failing either test is flagged and left out of the ROUTE preview, because viz
falls back to its own name heuristic there. That includes "Defer→stats": viz's
stats floor returns no aggregation at all, so an aggregation on a column nothing
resolves is simply lost.

    HOW TO RUN — this needs YOUR real terminal, not an agent's captured shell:

        python3 edit_dictionary.py path/to/<stem>.schema.json

    Keys: ↑↓ move · r role · c concept · l label · d desc · a aggregation ·
          s save · q quit
    Edit, press `s` to save, `q` to quit, then tell the agent to continue.
    `viz smart` reuses the same file path, so no other wiring is needed.

Non-interactive helper (safe to run anywhere, no TTY needed):

        python3 edit_dictionary.py --summary path/to/<stem>.schema.json

    prints a COLUMN / ROLE / CONCEPT / AGG / ROUTE table — handy for a
    before/after diff. ROUTE is the same concept/role projection described
    above, not the final `viz smart` route.

The routing preview mirrors `route_from_concept` / `route_from_role` in
src/cmd/viz.rs (~line 15480), and the explicit-aggregation override mirrors
`derive_semantics` (~line 15898, qsv issue #4401); the vocab lists mirror
src/cmd/describegpt/dictionary.rs:146 (ROLE_VOCAB) and :168 (CONCEPT_VOCAB).
Off-vocab values are allowed (a new concept may exist in newer qsv) but flagged
with a warning rather than blocked.
"""

import json
import os
import sys
import tempfile

# ── controlled vocabulary — mirror of qsv source (free text allowed, warned) ──
# ROLE_VOCAB    -> src/cmd/describegpt/dictionary.rs:146
# CONCEPT_VOCAB -> src/cmd/describegpt/dictionary.rs:168
ROLE_VOCAB = ["dimension", "measure", "identifier", "timestamp"]

CONCEPT_VOCAB = [
    # spatial
    "geo.zip_code", "geo.city", "geo.county", "geo.county_fips",
    "geo.state", "geo.state_fips", "geo.country", "geo.latitude",
    "geo.longitude", "geo.coordinate_pair", "geo.street_address",
    "geo.census_tract", "geo.crs_stateplane_x", "geo.crs_stateplane_y",
    # temporal
    "time.event_timestamp", "time.created_at", "time.closed_at",
    "time.updated_at", "time.due_at", "time.date", "time.duration",
    # identifiers
    "id.surrogate_key", "id.natural_key", "id.foreign_key", "id.uuid",
    # organizations
    "org.agency", "org.company", "org.industry",
    # sensitive personal data
    "pii.email", "pii.phone", "pii.full_name", "pii.address",
    # quantities / categoricals
    "measure.count", "measure.amount", "measure.money", "measure.ratio",
    "category.status", "category.type", "category.channel",
    # NYC-domain extension
    "nyc.bbl", "nyc.borough", "nyc.community_board", "nyc.complaint_type",
    # fallback
    "unknown",
]

# The four editable fields, in display order.
FIELDS = ["role", "concept", "label", "description", "aggregation"]

# `sum`/`mean` ONLY. viz's Agg enum also has Min/Max, but the KPI overview row has
# exactly two headline forms across its locale catalogs, so those are deliberately
# not part of the x-qsv.aggregation vocabulary (qsv issue #4401).
AGG_VOCAB = ["sum", "mean"]

# Sentinel offered in the aggregation picker to REMOVE the key (fall back to qsv's
# own extensive-vs-intensive guess). Storing "" instead would leave a key viz has
# to ignore, and would survive into the committed sidecar as noise.
AGG_CLEAR = "(clear — let qsv guess)"


# ─────────────────────────── pure routing preview ───────────────────────────
# Mirror of src/cmd/viz.rs `route_from_concept` / `route_from_role`
# (~line 15480). Precedence is concept → role → (defer to stats). We do NOT
# reimplement the content_type/stats fallback: those fields are not edited here,
# so anything unresolved by role/concept is shown as "Defer".

def route_from_concept(concept):
    """Return a route label for a concept token, or None to defer to role."""
    if not concept:
        return None
    ns, _, leaf = concept.partition(".")
    if ns == "geo":
        if leaf in ("latitude", "longitude", "coordinate_pair"):
            return "MapCoord"
        if leaf in ("crs_stateplane_x", "crs_stateplane_y"):
            return "ProjectedCoord"
        return "Dimension"
    if ns == "time":
        # a duration is a SPAN, not a point in time: it has no place on a time
        # axis, so viz routes it to a Mean measure (summing "days to close" is
        # meaningless in the way a total revenue is not).
        return "Measure(mean)" if leaf == "duration" else "Temporal"
    if ns in ("id", "pii"):
        return "Skip"
    if ns in ("org", "category", "nyc"):
        return "Dimension"
    if ns == "measure":
        return "Measure(mean)" if leaf == "ratio" else "Measure(sum)"
    return None  # unknown namespace or bare "unknown" -> defer to role


def route_from_role(role):
    """Return a route label for a role token, or None to defer further."""
    return {
        "timestamp": "Temporal",
        "identifier": "Skip",
        "dimension": "Dimension",
        "measure": "Measure(sum)",
    }.get(role)


def normalized_agg(value):
    """viz's aggregation token, or "" if viz would not honor this value.

    Mirrors `xq_aggregation` in src/cmd/viz.rs: trim, ASCII-case-fold, then accept
    ONLY the closed two-token vocabulary — an unknown verb yields None there, so
    the column silently falls back to the label heuristic. The fold is ASCII-only
    on purpose: Python's Unicode `str.lower()` maps characters like "ſ" (U+017F)
    and "K" (U+212A) onto ASCII, so it would accept tokens Rust's
    `to_ascii_lowercase` rejects, and the preview would disagree with viz.
    """
    folded = "".join(c.lower() if c.isascii() else c for c in (value or "").strip())
    return folded if folded in AGG_VOCAB else ""


def honored_agg(aggregation, role, qsv_type):
    """The aggregation viz will KEEP for this column, or "" if it drops it.

    A full mirror of `xq_aggregation`'s read gate in src/cmd/viz.rs — all three
    conditions, not just the token:

      * token — in the closed vocabulary (see `normalized_agg`)
      * qsv_type — Integer/Float. ABSENT is admissible: viz only rejects when the
        key is present AND says non-numeric, so a hand-written dictionary that
        omits it still passes. describegpt emits it on every property.
      * role — empty or exactly "measure", trimmed. Empty is admissible because
        `concept` alone can establish the measure, which is the precedence
        `derive_semantics` uses.

    An aggregation failing any of these is dropped by viz and the column falls
    back to the label heuristic — so it must not appear in the ROUTE preview.
    """
    if not normalized_agg(aggregation):
        return ""
    if (qsv_type or "").strip() not in ("", "Integer", "Float"):
        return ""
    if (role or "").strip() not in ("", "measure"):
        return ""
    return normalized_agg(aggregation)


def content_type_is_measure(content_type):
    """Whether `content_type` alone routes a column to Measure in `derive_semantics`.

    `route_from_content_type` (src/cmd/viz.rs) has exactly ONE arm returning
    Route::Measure — "money", the only numeric token in the vocabulary. Every other
    token is Dimension/MapCoord/Temporal/Defer/Skip, and the stats floor below it
    returns `agg: None`. So "money" is the single case where a column unresolved by
    concept and role can still carry an explicit aggregation, which is why it is
    special-cased here instead of mirroring the whole content_type vocabulary.

    The base token is the part before ":" — the suffix carries an strftime format
    (`"datetime:%Y-%m-%d"`), which routing throws away.
    """
    base = (content_type or "").split(":", 1)[0].strip()
    return base == "money"


def effective_route(role, concept, aggregation="", qsv_type="", content_type=""):
    """Concept→role route projection (concept first, then role).

    A preview of the concept/role contribution to routing only; it does NOT model
    `viz smart`'s stats/label guardrails, nor the content_type mapping beyond its
    one numeric arm (see `content_type_is_measure`), so the final route can differ.
    Anything still unresolved is shown as "Defer→stats".

    An explicit `x-qsv.aggregation` overrides the projected sum/mean, but ONLY when
    viz would honor it: it must survive the full read gate (`honored_agg`) AND land
    on a route that resolves to Measure — mirroring `derive_semantics`, which
    consults it under `route == Route::Measure`. Anything else is ignored here
    exactly as viz ignores it; showing it would promise a route viz will never
    produce.
    """
    route = route_from_concept(concept) or route_from_role(role)
    if route is None:
        # step 3 in derive_semantics: content_type, of which only "money" is a
        # measure. Everything else either routes elsewhere or hits the stats floor,
        # where viz returns agg: None and the explicit aggregation is lost.
        route = "Measure(sum)" if content_type_is_measure(content_type) else "Defer→stats"
    agg = honored_agg(aggregation, role, qsv_type)
    if agg and route.startswith("Measure"):
        return f"Measure({agg})"
    return route


# ──────────────────────────── pure schema model ─────────────────────────────

def load_schema(path):
    with open(path, encoding="utf-8") as fh:
        return json.load(fh)


def _prop(schema, name):
    return schema["properties"][name]


def get_field(schema, name, field):
    """Read one editable field for a column (empty string when absent)."""
    prop = _prop(schema, name)
    xq = prop.get("x-qsv") or {}
    if field == "role":
        return xq.get("role", "") or ""
    if field == "concept":
        return xq.get("concept", "") or ""
    if field == "label":
        return prop.get("title", prop.get("label", xq.get("label", ""))) or ""
    if field == "description":
        return prop.get("description", "") or ""
    if field == "aggregation":
        return xq.get("aggregation", "") or ""
    raise ValueError(f"unknown field {field!r}")


def apply_edit(schema, name, field, value):
    """Set one editable field. Return True iff the value actually changed."""
    if get_field(schema, name, field) == value:
        return False
    prop = _prop(schema, name)
    if field == "aggregation":
        # empty means REMOVE, not store "" — see AGG_CLEAR
        if value:
            prop.setdefault("x-qsv", {})["aggregation"] = value
        elif isinstance(prop.get("x-qsv"), dict):
            prop["x-qsv"].pop("aggregation", None)
    elif field in ("role", "concept"):
        prop.setdefault("x-qsv", {})[field] = value
    elif field == "label":
        prop["title"] = value
    elif field == "description":
        prop["description"] = value
    else:
        raise ValueError(f"unknown field {field!r}")
    return True


def columns(schema):
    """Ordered per-column view: name, the four fields, route, off-vocab flags."""
    out = []
    for name in schema.get("properties", {}):
        role = get_field(schema, name, "role")
        concept = get_field(schema, name, "concept")
        agg = get_field(schema, name, "aggregation")
        xq = _prop(schema, name).get("x-qsv") or {}
        qsv_type = xq.get("qsv_type", "") or ""
        content_type = xq.get("content_type", "") or ""
        route = effective_route(role, concept, agg, qsv_type, content_type)
        out.append({
            "name": name,
            "role": role,
            "concept": concept,
            "label": get_field(schema, name, "label"),
            "description": get_field(schema, name, "description"),
            "aggregation": agg,
            "route": route,
            "role_off_vocab": bool(role) and role not in ROLE_VOCAB,
            "concept_off_vocab": bool(concept) and concept not in CONCEPT_VOCAB,
            # Flagged whenever viz ignores the aggregation: it fails the read gate
            # (bad token / non-numeric qsv_type / non-measure role), or the column
            # does not route to Measure — including "Defer→stats", where the stats
            # floor in `derive_semantics` returns agg: None and the value is simply
            # lost. That last case needs no exemption now that the one content_type
            # arm which rescues it ("money") resolves to Measure above.
            "agg_ineffective": bool(agg)
            and (not honored_agg(agg, role, qsv_type) or not route.startswith("Measure")),
        })
    return out


def serialize(schema):
    return json.dumps(schema, indent=2, ensure_ascii=False) + "\n"


def save_atomic(schema, path):
    """Write serialized schema to path via temp file + atomic rename."""
    data = serialize(schema)
    d = os.path.dirname(os.path.abspath(path)) or "."
    fd, tmp = tempfile.mkstemp(dir=d, prefix=".dict-", suffix=".tmp")
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as fh:
            fh.write(data)
        os.replace(tmp, path)
    except BaseException:
        try:
            os.unlink(tmp)
        except OSError:
            pass
        raise


def format_summary(schema):
    """Plain COLUMN / ROLE / CONCEPT / AGG / ROUTE table (no curses)."""
    cols = columns(schema)
    w_name = max([len("COLUMN")] + [len(c["name"]) for c in cols])
    w_role = max([len("ROLE")] + [len(c["role"] or "-") for c in cols])
    w_con = max([len("CONCEPT")] + [len(c["concept"] or "-") for c in cols])
    w_agg = max([len("AGG")] + [len(c["aggregation"] or "-") for c in cols])
    rows = ["  {:<{n}}  {:<{r}}  {:<{c}}  {:<{a}}  {}".format(
        "COLUMN", "ROLE", "CONCEPT", "AGG", "ROUTE",
        n=w_name, r=w_role, c=w_con, a=w_agg)]
    for c in cols:
        flag = " "
        if c["role_off_vocab"] or c["concept_off_vocab"]:
            flag = "*"
        if c["agg_ineffective"]:
            flag = "!"
        rows.append("{} {:<{n}}  {:<{r}}  {:<{c}}  {:<{a}}  {}".format(
            flag, c["name"], c["role"] or "-", c["concept"] or "-",
            c["aggregation"] or "-", c["route"],
            n=w_name, r=w_role, c=w_con, a=w_agg))
    if any(c["agg_ineffective"] for c in cols):
        rows.append("")
        rows.append("! = aggregation viz drops on read — not sum/mean, or not on a measure")
    return "\n".join(rows)


# ──────────────────────────────── curses UI ─────────────────────────────────

def _set_cursor(visible):
    """Best-effort cursor visibility; a no-op on terminals that reject it."""
    import curses
    try:
        curses.curs_set(visible)
    except curses.error:
        pass


def _safe_addstr(win, y, x, text, attr=0):
    """addstr that never raises at the screen edge."""
    try:
        h, w = win.getmaxyx()
        if 0 <= y < h and x < w:
            win.addstr(y, x, text[: max(0, w - x - 1)], attr)
    except Exception:
        pass


def _text_input(stdscr, prompt, initial):
    """One-line editor. Returns the new string, or None if cancelled (Esc)."""
    import curses
    buf = list(initial or "")
    _set_cursor(1)
    while True:
        h, w = stdscr.getmaxyx()
        _safe_addstr(stdscr, h - 1, 0, " " * (w - 1))
        s = f"{prompt}: {''.join(buf)}"
        _safe_addstr(stdscr, h - 1, 0, s, curses.A_REVERSE)
        stdscr.move(h - 1, min(len(s), w - 1))
        stdscr.refresh()
        ch = stdscr.get_wch()
        if ch in ("\n", "\r", curses.KEY_ENTER):
            _set_cursor(0)
            return "".join(buf)
        if ch == "\x1b":  # Esc
            _set_cursor(0)
            return None
        if ch in (curses.KEY_BACKSPACE, "\x7f", "\b"):
            if buf:
                buf.pop()
        elif isinstance(ch, str) and ch.isprintable():
            buf.append(ch)


def _pick(stdscr, title, options, current, allow_free):
    """Filterable picker. Type to filter; a non-matching entry is offered as
    free text when allow_free. Returns the chosen string, or None (Esc)."""
    import curses
    buf = ""
    idx = 0
    while True:
        exact = buf in options
        matches = [o for o in options if buf.lower() in o.lower()] if buf else list(options)
        entries = list(matches)
        if allow_free and buf and not exact:
            entries = [f"«use typed: {buf}»"] + entries
        if not entries:
            entries = ["(no match — type to add, or Esc)"] if allow_free else ["(no match — Esc)"]
        idx = max(0, min(idx, len(entries) - 1))

        h, w = stdscr.getmaxyx()
        stdscr.erase()
        _safe_addstr(stdscr, 0, 0, f" {title}   (current: {current or '-'})", curses.A_BOLD)
        _safe_addstr(stdscr, 1, 0, f" filter: {buf}", curses.A_REVERSE)
        _safe_addstr(stdscr, 2, 0, " ↑↓ move · Enter select · type to filter"
                                   + (" · Enter on «use typed» = free text" if allow_free else "")
                                   + " · Esc cancel")
        top = 4
        avail = h - top - 1
        start = max(0, idx - avail + 1)
        for row, o in enumerate(entries[start:start + avail]):
            attr = curses.A_REVERSE if start + row == idx else 0
            marker = "→ " if start + row == idx else "  "
            _safe_addstr(stdscr, top + row, 0, marker + o, attr)
        stdscr.refresh()

        ch = stdscr.get_wch()
        if ch == "\x1b":
            return None
        if ch in ("\n", "\r", curses.KEY_ENTER):
            sel = entries[idx]
            if sel.startswith("(no match"):
                continue
            if allow_free and buf and not exact and idx == 0:
                return buf
            return sel
        if ch in (curses.KEY_BACKSPACE, "\x7f", "\b"):
            buf = buf[:-1]
            idx = 0
        elif ch == curses.KEY_UP:
            idx -= 1
        elif ch == curses.KEY_DOWN:
            idx += 1
        elif isinstance(ch, str) and ch.isprintable():
            buf += ch
            idx = 0


def _draw(stdscr, schema, cur, top, dirty, status):
    import curses
    cols = columns(schema)
    h, w = stdscr.getmaxyx()
    stdscr.erase()
    title = "Fine-tune data dictionary"
    flag = "  ● unsaved" if dirty else ""
    _safe_addstr(stdscr, 0, 0, f" {title}{flag}", curses.A_BOLD)
    _safe_addstr(stdscr, 1, 0, " ↑↓ move · r role · c concept · l label · d desc · a agg"
                               " · s save · q quit")
    # column header
    nw = min(22, max(8, max((len(c["name"]) for c in cols), default=8)))
    hdr = f" {'COLUMN':<{nw}} {'ROLE':<10} {'CONCEPT':<22} {'AGG':<5} {'ROUTE':<14} LABEL"
    _safe_addstr(stdscr, 3, 0, hdr, curses.A_UNDERLINE)
    body_top = 4
    avail = h - body_top - 1
    if cur < top:
        top = cur
    if cur >= top + avail:
        top = cur - avail + 1
    for row, c in enumerate(cols[top:top + avail]):
        i = top + row
        role = (c["role"] or "-") + ("*" if c["role_off_vocab"] else "")
        con = (c["concept"] or "-") + ("*" if c["concept_off_vocab"] else "")
        agg = (c["aggregation"] or "-") + ("!" if c["agg_ineffective"] else "")
        line = (f" {c['name']:<{nw}} {role:<10} {con:<22} {agg:<5} "
                f"{c['route']:<14} {c['label']}")
        attr = curses.A_REVERSE if i == cur else 0
        _safe_addstr(stdscr, body_top + row, 0, line, attr)
    msg = status or (
        "* = value not in the known vocab (allowed, just unusual) · "
        "! = aggregation viz will drop (not sum/mean, or not a measure) · "
        "ROUTE = concept/role preview, before viz smart's content_type & stats guardrails"
    )
    _safe_addstr(stdscr, h - 1, 0, msg[: w - 1], curses.A_DIM)
    stdscr.refresh()
    return top


def _run(stdscr, path):
    import curses
    _set_cursor(0)
    schema = load_schema(path)
    names = list(schema.get("properties", {}))
    if not names:
        return "No 'properties' in schema — nothing to edit."
    cur, top, dirty, status = 0, 0, False, ""

    def edit(field):
        nonlocal dirty, status
        name = names[cur]
        old = get_field(schema, name, field)
        if field == "role":
            val = _pick(stdscr, f"{name} — role", ROLE_VOCAB, old, allow_free=True)
        elif field == "concept":
            val = _pick(stdscr, f"{name} — concept", CONCEPT_VOCAB, old, allow_free=True)
        elif field == "aggregation":
            val = _pick(stdscr, f"{name} — aggregation", AGG_VOCAB + [AGG_CLEAR], old,
                        allow_free=False)
            if val == AGG_CLEAR:
                val = ""
        else:
            val = _text_input(stdscr, f"{name} — {field}", old)
        if val is None:
            status = "cancelled"
            return
        if apply_edit(schema, name, field, val):
            dirty = True
            status = f"{name}.{field} → {val or '(empty)'}"
        else:
            status = "unchanged"

    while True:
        top = _draw(stdscr, schema, cur, top, dirty, status)
        status = ""
        try:
            ch = stdscr.get_wch()
        except curses.error:
            continue
        if ch in (curses.KEY_DOWN, "j"):
            cur = min(cur + 1, len(names) - 1)
        elif ch in (curses.KEY_UP, "k"):
            cur = max(cur - 1, 0)
        elif ch == curses.KEY_NPAGE:
            cur = min(cur + 10, len(names) - 1)
        elif ch == curses.KEY_PPAGE:
            cur = max(cur - 10, 0)
        elif ch in ("g", curses.KEY_HOME):
            cur = 0
        elif ch in ("G", curses.KEY_END):
            cur = len(names) - 1
        elif ch == "r":
            edit("role")
        elif ch == "c":
            edit("concept")
        elif ch == "l":
            edit("label")
        elif ch == "d":
            edit("description")
        elif ch == "a":
            edit("aggregation")
        elif ch == "s":
            if dirty:
                save_atomic(schema, path)
                dirty = False
                status = f"saved → {path}"
            else:
                status = "nothing to save"
        elif ch == "q":
            if not dirty:
                return "No changes."
            choice = _pick(stdscr, "Unsaved changes", ["save and quit", "quit without saving"],
                           "", allow_free=False)
            if choice is None:
                continue
            if choice.startswith("save"):
                save_atomic(schema, path)
                return f"Saved → {path}"
            return "Quit without saving."


def main(argv):
    args = argv[1:]
    if not args or args[0] in ("-h", "--help"):
        print(__doc__)
        return 0
    summary = False
    if args[0] == "--summary":
        summary = True
        args = args[1:]
    if not args:
        print("error: no schema path given", file=sys.stderr)
        return 2
    path = args[0]
    if not os.path.isfile(path):
        print(f"error: no such file: {path}", file=sys.stderr)
        return 2
    try:
        schema = load_schema(path)
    except (OSError, json.JSONDecodeError) as e:
        print(f"error: cannot read JSON: {e}", file=sys.stderr)
        return 2

    if summary:
        print(format_summary(schema))
        return 0

    if not (sys.stdin.isatty() and sys.stdout.isatty()):
        print("This is an interactive TUI — run it in your own terminal:\n"
              f"    python3 {os.path.basename(__file__)} {path}\n"
              "(An agent's captured shell is not a real terminal.)\n"
              "For a non-interactive view use:  --summary", file=sys.stderr)
        return 2

    try:
        import curses
    except ImportError:
        print("error: the 'curses' module is unavailable in this Python build.",
              file=sys.stderr)
        return 2
    result = curses.wrapper(_run, path)
    if result:
        print(result)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
