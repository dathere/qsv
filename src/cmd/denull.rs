static USAGE: &str = r#"
Detect null sentinels - literal text like "NULL" or "N/A" standing in for a missing
value - that stop a numeric column from being recognized as numeric.

A cell holding the text "NULL" is a VALUE, not a null. `qsv stats` therefore types
the whole column as String, its nullcount stays 0, and no quartiles are computed.
Everything downstream degrades quietly: `viz smart` drops the column, `schema`
declares it a string, and `describegpt` describes a category that isn't one.

denull scans each column ONCE, with bounded memory, and partitions its values into
those that parse as a finite number and those that don't. A column is CONFIRMED when
every non-numeric value it holds is a known null sentinel and at least two distinct
numeric values remain.

A column is REJECTED - with the reason - when it cannot be promoted anyway: another
value is not a sentinel ("OK"), its numbers carry leading zeros and are really codes
("007"), or it buries the sentinel under more than --max-distinct other non-numeric
values.

Only columns worth acting on are listed: those holding a known sentinel, and those
that are predominantly numeric, whose few odd values are candidates for a sentinel
denull does not know yet - name them with --add-vocab. An ordinary categorical is
not a near miss and is not reported; nor is a free-text column that merely happens
to be unpromotable. Use --all-columns to see everything scanned.

The scan is exhaustive, not sampled: a column is never confirmed on the strength of
the values that happen to sort first. A genuine free-text column disqualifies itself
as soon as it accumulates --max-distinct different non-numeric values, so memory
stays flat. A 434 MB, 86-column file peaks at ~40 MB - the same as a type-inference
pass, and ~19x less than an exhaustive frequency table of every distinct value.

By default denull only REPORTS; it never rewrites your data. Pass --apply to
rewrite it, blanking sentinels ONLY in the columns denull CONFIRMED. A column it
REJECTED is copied through untouched, as is every column it did not scan:

  $ qsv denull --apply data.csv -o clean.csv
  $ qsv stats clean.csv --everything

Cleaning is per-column, which is what a single `qsv replace` pass cannot do: it
takes one regex across all selected columns, so it cannot blank "NULL" in one
column and "-" in another while leaving a literal "-" alone in a third.

Once blanked, `qsv stats` treats those cells as MISSING: it excludes them from mean,
stddev and the quartiles, and counts them in `nullcount` and `sparsity`. Do not reach
for the `--nulls` option of `qsv stats` to "restore" them. That option puts the blanks
back into the denominator while they contribute nothing to the sum, which is the same
as imputing zero. On a column that is 54% sentinel, that pulls the mean from 271 down
to 123 and SHRINKS the reported standard error - more confidence in a worse number -
while leaving the median and quartiles alone, so the summary stops agreeing with
itself. The `--nulls` option is for data where an empty cell genuinely MEANS zero (no
events, no charge). A well with no recorded casing depth does not have a casing depth
of zero.

Statistics over the cleaned column are still complete-case: they describe the rows
that HAVE a value. If a value is missing for a reason correlated with the value
itself, the estimate is biased. denull does not create that bias - before it ran, the
column was a String with no statistics at all - but it does not remove it either. It
makes the missingness visible so you can reason about it.

Numeric sentinels (-999, -9999, 9999) are deliberately NOT detected. They parse as
valid numbers, so no scan can distinguish them from real data - a depth-to-water
reading of -140 ft is an artesian well, not a missing value. Only a human or a
domain-aware model can propose those, and only a human should apply them.

The `sentinels` column lists the sentinel tokens OBSERVED in that column. They are
only safe to remove when the verdict is `confirmed`.

Examples:

  Report every column holding a null sentinel:
    $ qsv denull data.csv

  Restrict to a few columns, and emit JSON for a script to consume:
    $ qsv denull -s HoleDepth,WellDepth,CasingDepth --json data.csv

  Treat the site-specific "no reading" marker as a sentinel too:
    $ qsv denull --add-vocab "no reading,not recorded" data.csv

  Show every scanned column, including those with nothing to report:
    $ qsv denull --all-columns data.csv

  Blank the sentinels in every confirmed column; the report goes to stderr:
    $ qsv denull --apply data.csv -o clean.csv

For the tests, see https://github.com/dathere/qsv/blob/master/tests/test_denull.rs.

Usage:
    qsv denull [options] [<input>]
    qsv denull --help

denull options:
    -s, --select <arg>     Select the columns to scan. See `qsv select --help`
                           for the full selection syntax.
    --vocab <list>         Comma-separated null sentinel vocabulary, REPLACING
                           the built-in list. Matched case-insensitively after
                           trimming surrounding whitespace.
    --add-vocab <list>     Comma-separated tokens to ADD to the built-in list.
                           Use this for site-specific markers.
    --max-distinct <n>     Abandon a column once it holds this many distinct
                           non-numeric values. Guards memory on free-text
                           columns and bounds the report.
                           [default: 16]
    --all-columns          Also report columns with nothing to flag. By default
                           only columns with a verdict are listed.
    --apply                Rewrite the data instead of only reporting it. Blanks
                           the sentinels in every CONFIRMED column and writes the
                           CSV to <output> (or stdout), sending the report to
                           stderr. Rejected and unscanned columns pass through
                           untouched. Needs a file input, and <output> must not
                           be the input file.
    --json                 Emit the report as a JSON array instead of CSV.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write the report here instead of stdout.
    -n, --no-headers       When set, the first row will NOT be interpreted as
                           column names. Columns are named col_1, col_2, etc.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::{
    collections::{HashMap, HashSet},
    io,
};

use serde::{Deserialize, Serialize};

use crate::{
    CliResult,
    config::{Config, Delimiter},
    select::SelectColumns,
    util,
};

#[derive(Deserialize)]
struct Args {
    arg_input:         Option<String>,
    flag_select:       SelectColumns,
    flag_vocab:        Option<String>,
    flag_add_vocab:    Option<String>,
    flag_max_distinct: usize,
    flag_all_columns:  bool,
    flag_apply:        bool,
    flag_json:         bool,
    flag_output:       Option<String>,
    flag_no_headers:   bool,
    flag_delimiter:    Option<Delimiter>,
}

/// The built-in null sentinel vocabulary. Deliberately contains no numeric or date
/// tokens: those parse as valid values, so a scan cannot tell them apart from real
/// data (see the `--vocab` discussion in USAGE).
const DEFAULT_VOCAB: &[&str] = &[
    "null", "\\n", "na", "n/a", "n.a.", "#n/a", "nan", "nil", "none", "-", "--", "?", "??", ".",
    "..", "unknown", "unk", "missing", "tbd", "void", "#null!", "(blank)", "(empty)", "empty",
];

#[derive(Serialize)]
struct Finding {
    field:         String,
    verdict:       String,
    sentinels:     String,
    rows_affected: u64,
    pct_affected:  f64,
    numeric_rows:  u64,
    promotes_to:   String,
    evidence:      String,
}

/// Per-column tally accumulated in a single pass. `offenders` is capped at
/// `max_distinct`; once it overflows, `too_many` latches and the map stops growing, so a
/// free-text column's memory stays flat no matter how many distinct values it holds.
struct ColumnTally {
    offenders:       HashMap<Vec<u8>, u64>,
    too_many:        bool,
    numeric_rows:    u64,
    /// every non-empty cell that did not parse as a finite number, counted even after
    /// `too_many` latches. Lets `judge` ask "is this column predominantly numeric?"
    nonnumeric_rows: u64,
    /// distinct numeric values, capped at 2 - we only need to know "at least two"
    numeric_sample:  Vec<Vec<u8>>,
    all_integer:     bool,
    /// A zero-padded numeric ("007", "05.10") is a CODE, not a quantity. Masking a
    /// sentinel here would promote it to a number and silently eat its leading
    /// zeros, so a single sighting disqualifies the column. Mirrors the
    /// leading-zero rule in `stats`' type inference.
    zero_padded:     bool,
    /// Distinct sentinels observed, keyed by their lower-cased (vocabulary) identity and
    /// valued by the first spelling met, so the report can echo the casing in the data.
    /// Tracked independently of `offenders` because a sentinel can be met both before
    /// that map fills and after it has stopped growing, and both sightings are evidence.
    ///
    /// Needs no cap: a token only lands here if it is IN the vocabulary, and every casing
    /// of it collapses to one key, so the map can never outgrow the vocabulary itself.
    ///
    /// Non-empty also means "this column held a sentinel", which is what decides whether
    /// a REJECTED column is worth reporting at all. A column of `A..F` codes is not a
    /// near miss - it is an ordinary categorical, and saying "rejected" about it is noise.
    seen_sentinels:  HashMap<Vec<u8>, Vec<u8>>,
}

/// Cells this long or shorter are lower-cased on the stack. Longer ones are rare enough
/// (only a custom `--add-vocab` token can be one) to be worth a heap allocation.
const SENTINEL_STACK: usize = 32;

/// The sentinel vocabulary plus the length of its longest token. The cutoff is DERIVED,
/// never hard-coded: a fixed 16-byte limit silently made any longer `--add-vocab` token
/// unmatchable, which is exactly what `--add-vocab` exists to support.
struct Vocab {
    set:     HashSet<String>,
    max_len: usize,
}

impl Vocab {
    /// Case-insensitive membership. Invalid UTF-8 can never be a member: the vocabulary is
    /// `String`, so a non-UTF-8 cell is by definition not a sentinel.
    fn contains(&self, trimmed: &[u8]) -> bool {
        sentinel_identity(trimmed, self).is_some()
    }
}

impl ColumnTally {
    fn new() -> Self {
        Self {
            offenders:       HashMap::new(),
            too_many:        false,
            numeric_rows:    0,
            nonnumeric_rows: 0,
            numeric_sample:  Vec::new(),
            all_integer:     true,
            zero_padded:     false,
            seen_sentinels:  HashMap::new(),
        }
    }

    fn add(&mut self, cell: &[u8], max_distinct: usize, vocab: &Vocab) {
        let trimmed = trim_ascii(cell);
        if trimmed.is_empty() {
            return;
        }
        if let Ok(txt) = std::str::from_utf8(trimmed)
            && let Ok(v) = txt.parse::<f64>()
            && v.is_finite()
        {
            if is_zero_padded(trimmed) {
                self.zero_padded = true;
            }
            self.numeric_rows += 1;
            if txt.parse::<i64>().is_err() {
                self.all_integer = false;
            }
            if self.numeric_sample.len() < 2 && !self.numeric_sample.iter().any(|s| s == trimmed) {
                self.numeric_sample.push(trimmed.to_vec());
            }
            return;
        }
        self.nonnumeric_rows += 1;
        // repeat offender: the hot path for a free-text column, one hash probe
        if let Some(c) = self.offenders.get_mut(trimmed) {
            *c += 1;
            return;
        }
        // Probe every novel non-numeric cell, before AND after `too_many` latches: a
        // column can meet "NULL" while the offender map is still filling and "N/A" only
        // after it has stopped growing, and the report owes the user both. Repeat values
        // already returned above, and `sentinel_identity` rejects anything longer than the
        // longest sentinel on a length compare, so free text pays very little for this -
        // measured at ~2% on a 414 MB, 86-column file.
        if let Some(identity) = sentinel_identity(trimmed, vocab)
            && !self.seen_sentinels.contains_key(&identity)
        {
            self.seen_sentinels.insert(identity, trimmed.to_vec());
        }
        if self.too_many {
            return;
        }
        if self.offenders.len() >= max_distinct {
            // `offenders` is NOT cleared: it is already capped at `max_distinct`, so
            // retaining it costs nothing and lets the report name examples of the
            // disqualifying values. Note this cell is never inserted - it is the one that
            // overflows the map - which is exactly why sentinels are tracked separately.
            self.too_many = true;
        } else {
            self.offenders.insert(trimmed.to_vec(), 1);
        }
    }
}

/// A few non-sentinel values that disqualified an overflowed column, so the report can
/// show WHAT it choked on rather than only that it did.
fn offender_examples(tally: &ColumnTally, vocab: &Vocab) -> String {
    let mut ex: Vec<String> = tally
        .offenders
        .keys()
        .filter(|k| !vocab.contains(k))
        .map(|k| String::from_utf8_lossy(k).into_owned())
        .collect();
    ex.sort_unstable();
    ex.truncate(4);
    ex.join(",")
}

/// The sentinel tokens actually observed, for the report's `sentinels` column.
fn seen_list(tally: &ColumnTally) -> String {
    let mut seen: Vec<String> = tally
        .seen_sentinels
        .values()
        .filter_map(|k| std::str::from_utf8(k).ok())
        .map(ToString::to_string)
        .collect();
    seen.sort_unstable();
    seen.join(",")
}

/// The vocabulary identity of a cell, or `None` if it is not a sentinel. Case-insensitive
/// and allocation-free: the vocabulary is ASCII, so a stack buffer and
/// `make_ascii_lowercase` suffice. This runs on the free-text hot path (every novel short
/// non-numeric cell), where a per-cell `to_lowercase()` String would be a needless
/// allocation on every one of millions of rows. Callers allocate only when the identity
/// turns out to be one they have not seen before.
fn sentinel_identity(trimmed: &[u8], vocab: &Vocab) -> Option<Vec<u8>> {
    let len = trimmed.len();
    if len > vocab.max_len {
        return None;
    }
    let mut stack = [0_u8; SENTINEL_STACK];
    let mut heap: Vec<u8>;
    let lower: &[u8] = if len <= SENTINEL_STACK {
        stack[..len].copy_from_slice(trimmed);
        stack[..len].make_ascii_lowercase();
        &stack[..len]
    } else {
        heap = trimmed.to_vec();
        heap.make_ascii_lowercase();
        &heap
    };
    // A non-UTF-8 cell cannot be in a `String` vocabulary, so it is not a sentinel - it is
    // an offender, and `judge` must see it as one.
    let token = std::str::from_utf8(lower).ok()?;
    vocab.set.contains(token).then(|| lower.to_vec())
}

/// Surrounding ASCII whitespace is not part of a cell's identity: " NULL " is a sentinel.
fn trim_ascii(cell: &[u8]) -> &[u8] {
    let mut s = cell;
    while let [first, rest @ ..] = s
        && first.is_ascii_whitespace()
    {
        s = rest;
    }
    while let [rest @ .., last] = s
        && last.is_ascii_whitespace()
    {
        s = rest;
    }
    s
}

/// A leading `0` followed by another digit marks a padded code (`007`, `05.10`).
/// A bare `0`, or `0.5`, is a real number.
fn is_zero_padded(sample: &[u8]) -> bool {
    let body = sample.strip_prefix(b"-").unwrap_or(sample);
    matches!(body, [b'0', second, ..] if second.is_ascii_digit())
}

fn build_vocab(args: &Args) -> Vocab {
    let mut vocab: Vec<String> = args.flag_vocab.as_ref().map_or_else(
        || DEFAULT_VOCAB.iter().map(|s| (*s).to_string()).collect(),
        |v| {
            v.split(',')
                .map(|t| t.trim().to_lowercase())
                .filter(|t| !t.is_empty())
                .collect()
        },
    );
    if let Some(extra) = &args.flag_add_vocab {
        vocab.extend(
            extra
                .split(',')
                .map(|t| t.trim().to_lowercase())
                .filter(|t| !t.is_empty()),
        );
    }
    let max_len = vocab.iter().map(String::len).max().unwrap_or(0);
    Vocab {
        set: vocab.into_iter().collect(),
        max_len,
    }
}

/// Turn one column's tally into a verdict. The ONLY confirming path requires every
/// non-numeric value to be a known sentinel AND at least two distinct numeric values
/// to survive - one number plus a pile of "NULL"s is not a distribution.
fn judge(
    tally: &ColumnTally,
    vocab: &Vocab,
    total_rows: u64,
) -> Option<(String, String, String, String)> {
    // "Could this column be numeric at all?" gates everything else. A column with fewer
    // than two distinct numeric values is an ordinary categorical (or free text, or a
    // date), and reporting a verdict on it is noise: `CompletionSource` holds one stray
    // numeric-looking code among 7,499 letters and is not a failed sentinel column, it is
    // simply not our business. This check must precede the `too_many` bail-out, which
    // fires for free text and dates alike.
    if tally.numeric_sample.len() < 2 {
        return None;
    }
    // An OVERFLOWED column can never be promoted - it has more than --max-distinct
    // distinct non-numeric values. Absent a KNOWN sentinel it has no proposal to make,
    // so it stays silent: `DEEDBOOK` (535,836 numeric rows, >16 junk tokens) is a data
    // quality curiosity, not a null sentinel finding.
    if tally.too_many {
        if tally.seen_sentinels.is_empty() {
            return None;
        }
        return Some((
            "rejected:too-many-distinct".to_string(),
            seen_list(tally),
            String::new(),
            format!(
                "exceeded --max-distinct other distinct non-numeric values, e.g. {}",
                offender_examples(tally, vocab)
            ),
        ));
    }
    if tally.offenders.is_empty() {
        return None; // nothing non-numeric: already a clean numeric column
    }
    // Otherwise print a column only when it is plausibly a sentinel column. Either:
    //   (a) it holds a KNOWN sentinel, or
    //   (b) it is predominantly numeric, which makes its handful of non-numeric values
    //       strong candidates for an UNKNOWN sentinel the user should name via
    //       --add-vocab ("no reading", a site-specific marker, ...).
    // Without this gate a clean 86-column file reports 14 rejections for ordinary
    // categoricals (`HEATINGCOOLING` = A..F) and buries the findings that matter.
    if tally.seen_sentinels.is_empty() && tally.numeric_rows <= tally.nonnumeric_rows {
        return None;
    }

    // `filter_map(from_utf8)` here would DROP invalid-UTF-8 offenders, emptying `off_vocab`
    // and confirming a column whose junk is not a sentinel at all. Render lossily instead,
    // so such a value is still counted as the offender it is.
    let mut off_vocab: Vec<String> = tally
        .offenders
        .keys()
        .filter(|k| !vocab.contains(k))
        .map(|k| String::from_utf8_lossy(k).into_owned())
        .collect();
    if !off_vocab.is_empty() {
        off_vocab.sort_unstable();
        off_vocab.truncate(6);
        return Some((
            "rejected:off-vocab".to_string(),
            seen_list(tally),
            String::new(),
            format!(
                "non-sentinel values present: {}{}",
                off_vocab.join(","),
                if !tally.seen_sentinels.is_empty() {
                    ""
                } else {
                    " (add with --add-vocab if they mean 'missing')"
                }
            ),
        ));
    }
    if tally.zero_padded {
        return Some((
            "rejected:zero-padded".to_string(),
            seen_list(tally),
            String::new(),
            "leading zeros mark a code (zip/FIPS), not a quantity".to_string(),
        ));
    }
    let mut sentinels: Vec<String> = tally
        .offenders
        .keys()
        .filter_map(|k| std::str::from_utf8(k).ok())
        .map(ToString::to_string)
        .collect();
    sentinels.sort_unstable();
    let promotes_to = if tally.all_integer {
        "Integer"
    } else {
        "Float"
    };
    let pct = if total_rows == 0 {
        0.0
    } else {
        #[allow(clippy::cast_precision_loss)]
        {
            tally.offenders.values().sum::<u64>() as f64 * 100.0 / total_rows as f64
        }
    };
    Some((
        "confirmed".to_string(),
        sentinels.join(","),
        promotes_to.to_string(),
        format!(
            "{} numeric row(s), {:.2}% sentinel",
            tally.numeric_rows, pct
        ),
    ))
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let vocab = build_vocab(&args);

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.flag_select);

    // Both guards must fire BEFORE the scan: discovering after a full pass that the input
    // cannot be reopened, or that we are about to truncate it, is too late.
    if args.flag_apply {
        // Test the RESOLVED path, not `arg_input`: `Config` maps the conventional "-" to
        // stdin, so an `arg_input.is_some()` check would wave `denull --apply -` through,
        // and pass 2 would reopen an exhausted stdin and emit an empty file.
        let Some(in_path) = rconfig.path.clone() else {
            return fail_clierror!("--apply requires an input file path (stdin is not supported).");
        };
        if let Some(out) = &args.flag_output {
            let out_path = std::path::Path::new(out);
            // `is_same_file` compares file identity (inode/dev, or file-id on Windows), so
            // it catches `./data.csv`, a symlink, AND a hard link. Canonicalizing cannot:
            // two hard links to one inode have two distinct canonical paths, and writing
            // through either truncates the input pass 2 is still reading.
            if out_path.exists() && same_file::is_same_file(&in_path, out_path).unwrap_or(false) {
                return fail_clierror!(
                    "--apply cannot write to its own input ({out}); pass a different --output."
                );
            }
        }
    }

    let mut rdr = rconfig.reader()?;
    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    let names: Vec<String> = sel
        .iter()
        .map(|&i| {
            if rconfig.no_headers {
                format!("col_{}", i + 1)
            } else {
                String::from_utf8_lossy(&headers[i]).to_string()
            }
        })
        .collect();

    let mut tallies: Vec<ColumnTally> = (0..sel.len()).map(|_| ColumnTally::new()).collect();
    let mut total_rows = 0_u64;
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        total_rows += 1;
        for (k, &idx) in sel.iter().enumerate() {
            if let Some(cell) = record.get(idx) {
                tallies[k].add(cell, args.flag_max_distinct, &vocab);
            }
        }
    }

    let mut findings: Vec<Finding> = Vec::new();
    for (k, tally) in tallies.iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let pct = if total_rows == 0 {
            0.0
        } else {
            tally.offenders.values().sum::<u64>() as f64 * 100.0 / total_rows as f64
        };
        match judge(tally, &vocab, total_rows) {
            Some((verdict, sentinels, promotes_to, evidence)) => findings.push(Finding {
                field: names[k].clone(),
                verdict,
                sentinels,
                rows_affected: tally.offenders.values().sum::<u64>(),
                pct_affected: (pct * 100.0).round() / 100.0,
                numeric_rows: tally.numeric_rows,
                promotes_to,
                evidence,
            }),
            None if args.flag_all_columns => findings.push(Finding {
                field:         names[k].clone(),
                verdict:       "clean".to_string(),
                sentinels:     String::new(),
                rows_affected: 0,
                pct_affected:  0.0,
                numeric_rows:  tally.numeric_rows,
                promotes_to:   String::new(),
                evidence:      String::new(),
            }),
            None => {},
        }
    }

    // With --apply the primary sink carries the cleaned CSV, so the report moves to
    // stderr. Without it, the report IS the output.
    if args.flag_apply {
        emit_report(&findings, args.flag_json, None, true)?;
        return apply(
            &rconfig,
            args.flag_output.as_ref(),
            &vocab,
            &tallies,
            &sel,
            total_rows,
        );
    }
    emit_report(&findings, args.flag_json, args.flag_output.as_ref(), false)
}

/// Write the findings as CSV or JSON, either to `output`/stdout or to stderr.
fn emit_report(
    findings: &[Finding],
    json: bool,
    output: Option<&String>,
    to_stderr: bool,
) -> CliResult<()> {
    if json {
        let rendered = serde_json::to_string_pretty(findings)? + "\n";
        match (to_stderr, output) {
            (true, _) => eprint!("{rendered}"),
            (false, Some(path)) => std::fs::write(path, rendered)?,
            (false, None) => print!("{rendered}"),
        }
        return Ok(());
    }

    let mut wtr: csv::Writer<Box<dyn io::Write>> = if to_stderr {
        csv::Writer::from_writer(Box::new(io::stderr()))
    } else {
        Config::new(output).writer()?
    };
    wtr.write_record([
        "field",
        "verdict",
        "sentinels",
        "rows_affected",
        "pct_affected",
        "numeric_rows",
        "promotes_to",
        "evidence",
    ])?;
    for f in findings {
        wtr.write_record([
            &f.field,
            &f.verdict,
            &f.sentinels,
            &f.rows_affected.to_string(),
            &format!("{:.2}", f.pct_affected),
            &f.numeric_rows.to_string(),
            &f.promotes_to,
            &f.evidence,
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

/// Rewrite the data, blanking sentinels ONLY in columns that `judge` CONFIRMED.
///
/// Two passes over the file rather than one pass plus a buffer of every record: the
/// tallies are already built, so all pass 2 carries is one small identity set per
/// confirmed column, bounded by the vocabulary. Memory stays flat on a 400 MB file,
/// which is the property `denull` advertises.
fn apply(
    rconfig: &Config,
    output: Option<&String>,
    vocab: &Vocab,
    tallies: &[ColumnTally],
    sel: &crate::select::Selection,
    total_rows: u64,
) -> CliResult<()> {
    // Blank a cell only when its OWN column was confirmed and the cell is one of the exact
    // values that column actually held. A global set would let column A's "NULL" blank a
    // stray "NULL" in an unconfirmed column C.
    //
    // The mask set is `offenders`, the SAME set `judge` confirmed from - not
    // `seen_sentinels`. A confirmed column is never overflowed, so `offenders` holds every
    // one of its distinct non-numeric values, byte for byte. Masking from `seen_sentinels`
    // instead let the two disagree: it is keyed by vocabulary identity, so a sentinel
    // longer than the old fixed cutoff, or one that is not valid UTF-8, was confirmed by
    // `judge` and then silently skipped by `apply` - promising Integer and delivering
    // String.
    let mut confirmed: HashMap<usize, HashSet<Vec<u8>>> = HashMap::new();
    let mut expected_blanks = 0_u64;
    for (k, &idx) in sel.iter().enumerate() {
        let tally = &tallies[k];
        if let Some((verdict, ..)) = judge(tally, vocab, total_rows)
            && verdict == "confirmed"
        {
            expected_blanks += tally.offenders.values().sum::<u64>();
            confirmed.insert(idx, tally.offenders.keys().cloned().collect());
        }
    }

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(output).writer()?;

    let headers = rdr.byte_headers()?.clone();
    if !rconfig.no_headers {
        wtr.write_byte_record(&headers)?;
    }

    let mut record = csv::ByteRecord::new();
    let mut cleaned = csv::ByteRecord::new();
    let mut blanked = 0_u64;
    while rdr.read_byte_record(&mut record)? {
        cleaned.clear();
        for (idx, cell) in record.iter().enumerate() {
            // Untouched cells are copied byte-for-byte.
            let blank = confirmed
                .get(&idx)
                .is_some_and(|sentinels| sentinels.contains(trim_ascii(cell)));
            if blank {
                blanked += 1;
                cleaned.push_field(b"");
            } else {
                cleaned.push_field(cell);
            }
        }
        wtr.write_byte_record(&cleaned)?;
    }
    wtr.flush()?;
    if confirmed.is_empty() {
        eprintln!("denull: no column confirmed; data copied through unchanged.");
        return Ok(());
    }
    eprintln!(
        "denull: blanked {blanked} cell(s) across {} confirmed column(s).",
        confirmed.len()
    );
    // The report told the user exactly how many cells it would blank. If the rewrite does
    // not agree, the two halves of this command have drifted apart, and silently shipping
    // a file that still holds sentinels is the worst outcome available.
    if blanked != expected_blanks {
        return fail_clierror!(
            "denull: internal inconsistency - reported {expected_blanks} sentinel cell(s) but \
             blanked {blanked}. The output may be incomplete; please file an issue."
        );
    }
    Ok(())
}
