static USAGE: &str = r#"
Sorts CSV data in lexicographical, natural, numerical, reverse, unique or random order.

Note that this requires reading all of the CSV data into memory. If
you need to sort a large file that may not fit into memory, use the
extsort command instead.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_sort.rs.

Usage:
    qsv sort [options] [<input>]
    qsv sort --help

sort options:
    -s, --select <arg>      Select a subset of columns to sort.
                            See 'qsv select --help' for the format details.
    -N, --numeric           Compare according to string numerical value
    --natural               Compare strings using natural sort order
                            (treats numbers within strings as actual numbers, e.g.
                            "data1.txt", "data2.txt", "data10.txt", as opposed to
                            "data1.txt", "data10.txt", "data2.txt" when sorting
                            lexicographically)
                            https://en.wikipedia.org/wiki/Natural_sort_order
                            When combined with --numeric, --natural takes precedence.
    -R, --reverse           Reverse order
    -i, --ignore-case       Compare strings disregarding case.
                            Has no effect under --numeric (numbers are case-less).
    -u, --unique            When set, identical consecutive lines will be dropped
                            to keep only one line per sorted value. The same
                            comparison mode used to sort the input is also used
                            here, so unique-equality always agrees with the sort.

                            RANDOM SORTING OPTIONS:
    --random                Randomize (scramble) the data by row.
                            When set, the comparison flags (numeric,
                            natural, reverse, ignore-case) are ignored
                            for the shuffle itself, but still apply to
                            unique-filtering if --unique is also set.
    --seed <number>         Random Number Generator (RNG) seed to use if --random is set
    --rng <kind>            The RNG algorithm to use if --random is set.
                            Three RNGs are supported:
                            * standard: Use the standard RNG.
                              1.5 GB/s throughput.
                            * faster: Use faster RNG using the Xoshiro256Plus algorithm.
                              8 GB/s throughput.
                            * cryptosecure: Use cryptographically secure HC128 algorithm.
                              Recommended by eSTREAM (https://www.ecrypt.eu.org/stream/).
                              2.1 GB/s throughput though slow initialization.
                            [default: standard]


    -j, --jobs <arg>        The number of jobs to run in parallel.
                            When not set, the number of jobs is set to the
                            number of CPUs detected.
    --faster                When set, the sort will be faster. This is done by
                            using a faster sorting algorithm that is not "stable"
                            (i.e. the order of identical values is not guaranteed
                            to be preserved). It has the added side benefit that the
                            sort will also be in-place (i.e. does not allocate),
                            which is useful for sorting large files that will
                            otherwise NOT fit in memory using the default allocating
                            stable sort.

Common options:
    -h, --help              Display this message
    -o, --output <file>     Write output to <file> instead of stdout.
    -n, --no-headers        When set, the first row will not be interpreted
                            as headers. Namely, it will be sorted with the rest
                            of the rows. Otherwise, the first row will always
                            appear as the header row in the output.
    -d, --delimiter <arg>   The field delimiter for reading CSV data.
                            Must be a single character. (default: ,)
    --memcheck              Check if there is enough memory to load the entire
                            CSV into memory using CONSERVATIVE heuristics.
                            Ignored if --random or --faster is set.
"#;

use std::{cmp::Ordering, str::FromStr};

use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};
use rand_hc::Hc128Rng;
use rand_xoshiro::Xoshiro256Plus;
use rayon::slice::ParallelSliceMut;
use serde::Deserialize;
use simdutf8::basic::from_utf8;
use strum_macros::EnumString;

use self::Number::{Float, Int};
use crate::{
    CliResult,
    config::{Config, Delimiter},
    select::SelectColumns,
    util,
};

#[derive(Deserialize)]
struct Args {
    arg_input:        Option<String>,
    flag_select:      SelectColumns,
    flag_numeric:     bool,
    flag_natural:     bool,
    flag_reverse:     bool,
    flag_ignore_case: bool,
    flag_unique:      bool,
    flag_random:      bool,
    flag_seed:        Option<u64>,
    flag_rng:         String,
    flag_jobs:        Option<usize>,
    flag_faster:      bool,
    flag_output:      Option<String>,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
    flag_memcheck:    bool,
}

#[derive(Debug, EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
enum RngKind {
    Standard,
    Faster,
    Cryptosecure,
}

/// Selected once at startup based on the comparison flags. Drives both the
/// sort dispatch and the `--unique` filter so they always agree on equality.
/// Precedence: `--natural` > `--numeric` > `--ignore-case` > lex.
/// `--ignore-case` only applies under lex and natural; numeric ignores it.
#[derive(Clone, Copy)]
enum SortMode {
    Lex,
    LexIgnoreCase,
    Natural,
    NaturalIgnoreCase,
    Numeric,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let numeric = args.flag_numeric;
    let natural = args.flag_natural;
    let reverse = args.flag_reverse;
    let random = args.flag_random;
    let faster = args.flag_faster;
    let ignore_case = args.flag_ignore_case;
    let seed = args.flag_seed;

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers)
        .select(args.flag_select);

    let Ok(rng_kind) = RngKind::from_str(&args.flag_rng) else {
        return fail_incorrectusage_clierror!(
            "Invalid RNG algorithm `{}`. Supported RNGs are: standard, faster, cryptosecure.",
            args.flag_rng
        );
    };

    // we're loading the entire file into memory, we need to check avail memory.
    // we only check if we're doing a stable sort and its not --random,
    // because --faster sorts in-place (non-allocating) and --random shuffles.
    if let Some(path) = rconfig.path.clone()
        && !faster
        && !random
    {
        util::mem_file_check(&path, false, args.flag_memcheck)?;
    }

    let mut rdr = rconfig.reader()?;
    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    util::njobs(args.flag_jobs);

    let mut all = rdr.byte_records().collect::<Result<Vec<_>, _>>()?;

    // Pick the comparison mode once. The same mode drives the sort and the
    // --unique filter, so unique-equality always agrees with what the sort
    // grouped (previously --unique used its own if/else chain that silently
    // disagreed with the sort under e.g. --numeric --natural).
    let mode = if natural {
        if ignore_case {
            SortMode::NaturalIgnoreCase
        } else {
            SortMode::Natural
        }
    } else if numeric {
        SortMode::Numeric
    } else if ignore_case {
        SortMode::LexIgnoreCase
    } else {
        SortMode::Lex
    };

    if random {
        match rng_kind {
            RngKind::Standard => {
                if let Some(val) = seed {
                    let mut rng = StdRng::seed_from_u64(val); //DevSkim: ignore DS148264
                    all.shuffle(&mut rng); //DevSkim: ignore DS148264
                } else {
                    let mut rng = ::rand::rng();
                    all.shuffle(&mut rng); //DevSkim: ignore DS148264
                }
            },
            RngKind::Faster => {
                let mut rng = match seed {
                    None => rand::make_rng::<Xoshiro256Plus>(),
                    Some(sd) => Xoshiro256Plus::seed_from_u64(sd), // DevSkim: ignore DS148264
                };
                SliceRandom::shuffle(&mut *all, &mut rng); //DevSkim: ignore DS148264
            },
            RngKind::Cryptosecure => {
                // Build seed_32 only when --seed is provided. The previous
                // implementation pre-generated a 32-byte buffer from the
                // process RNG and then threw it away on the unseeded path,
                // wasting entropy and a syscall on every random sort.
                let mut rng: Hc128Rng = match seed {
                    None => rand::make_rng::<Hc128Rng>(),
                    Some(sd) => {
                        let mut seed_32 = [0u8; 32];
                        seed_32[..8].copy_from_slice(&sd.to_le_bytes());
                        Hc128Rng::from_seed(seed_32)
                    },
                };
                SliceRandom::shuffle(&mut *all, &mut rng);
            },
        }
    } else {
        // Hoist comparison dispatch out of the closure: each branch
        // monomorphizes with a single comparison function known at compile
        // time. This collapses the previous 16-arm tuple match (which
        // re-evaluated `if ignore_case` per row) into one macro plus a
        // 5-arm `match mode`.
        macro_rules! do_sort {
            ($cmp:expr) => {{
                if reverse {
                    if faster {
                        all.par_sort_unstable_by(|r1, r2| $cmp(sel.select(r2), sel.select(r1)));
                    } else {
                        all.par_sort_by(|r1, r2| $cmp(sel.select(r2), sel.select(r1)));
                    }
                } else if faster {
                    all.par_sort_unstable_by(|r1, r2| $cmp(sel.select(r1), sel.select(r2)));
                } else {
                    all.par_sort_by(|r1, r2| $cmp(sel.select(r1), sel.select(r2)));
                }
            }};
        }
        match mode {
            SortMode::Lex => do_sort!(iter_cmp),
            SortMode::LexIgnoreCase => do_sort!(iter_cmp_ignore_case),
            SortMode::Natural => do_sort!(iter_cmp_natural),
            SortMode::NaturalIgnoreCase => do_sort!(iter_cmp_natural_ignore_case),
            SortMode::Numeric => do_sort!(iter_cmp_num),
        }
    }

    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;
    rconfig.write_headers(&mut rdr, &mut wtr)?;
    if args.flag_unique {
        // Use the same `mode` as the sort so unique-equality always matches
        // what the sort grouped as adjacent.
        macro_rules! unique_filter {
            ($cmp:expr) => {{
                let mut iter = all.iter();
                if let Some(first) = iter.next() {
                    wtr.write_byte_record(first)?;
                    let mut prev = first;
                    for current in iter {
                        if $cmp(sel.select(prev), sel.select(current)) != Ordering::Equal {
                            wtr.write_byte_record(current)?;
                        }
                        prev = current;
                    }
                }
            }};
        }
        match mode {
            SortMode::Lex => unique_filter!(iter_cmp),
            SortMode::LexIgnoreCase => unique_filter!(iter_cmp_ignore_case),
            SortMode::Natural => unique_filter!(iter_cmp_natural),
            SortMode::NaturalIgnoreCase => unique_filter!(iter_cmp_natural_ignore_case),
            SortMode::Numeric => unique_filter!(iter_cmp_num),
        }
    } else {
        for r in &all {
            wtr.write_byte_record(r)?;
        }
    }
    Ok(wtr.flush()?)
}

/// Order `a` and `b` lexicographically using `Ord`
#[inline]
pub fn iter_cmp<A, L, R>(mut a: L, mut b: R) -> Ordering
where
    A: Ord,
    L: Iterator<Item = A>,
    R: Iterator<Item = A>,
{
    loop {
        match (a.next(), b.next()) {
            (None, None) => return Ordering::Equal,
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            (Some(x), Some(y)) => match x.cmp(&y) {
                Ordering::Equal => (),
                non_eq => return non_eq,
            },
        }
    }
}

/// Try parsing `a` and `b` as numbers when ordering
#[inline]
pub fn iter_cmp_num<'a, L, R>(mut a: L, mut b: R) -> Ordering
where
    L: Iterator<Item = &'a [u8]>,
    R: Iterator<Item = &'a [u8]>,
{
    loop {
        match (next_num(&mut a), next_num(&mut b)) {
            (None, None) => return Ordering::Equal,
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            (Some(x), Some(y)) => match compare_num(x, y) {
                Ordering::Equal => (),
                non_eq => return non_eq,
            },
        }
    }
}

/// Compare two cell-iterators ignoring case.
///
/// Cells that are pure ASCII compare via a zero-allocation byte-wise lowercase
/// fold (the common case). Non-ASCII cells fall back to allocating a
/// lowercased `String`. Cells that are not valid UTF-8 fall back to a raw
/// byte comparison so a deterministic order is still produced.
#[inline]
pub fn iter_cmp_ignore_case<'a, L, R>(mut a: L, mut b: R) -> Ordering
where
    L: Iterator<Item = &'a [u8]>,
    R: Iterator<Item = &'a [u8]>,
{
    loop {
        match (a.next(), b.next()) {
            (None, None) => return Ordering::Equal,
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            (Some(x), Some(y)) => match cmp_ignore_case(x, y) {
                Ordering::Equal => (),
                non_eq => return non_eq,
            },
        }
    }
}

#[inline]
fn cmp_ignore_case(a: &[u8], b: &[u8]) -> Ordering {
    // ASCII fast path: zero-allocation byte-wise lowercase compare.
    if a.is_ascii() && b.is_ascii() {
        return a
            .iter()
            .map(u8::to_ascii_lowercase)
            .cmp(b.iter().map(u8::to_ascii_lowercase));
    }
    // Unicode slow path: allocate lowercased Strings.
    match (from_utf8(a).ok(), from_utf8(b).ok()) {
        (Some(sa), Some(sb)) => sa.to_lowercase().cmp(&sb.to_lowercase()),
        // Invalid UTF-8 on either side: fall back to raw byte comparison.
        _ => a.cmp(b),
    }
}

/// Order `a` and `b` using natural sort order
#[inline]
pub fn iter_cmp_natural<'a, L, R>(mut a: L, mut b: R) -> Ordering
where
    L: Iterator<Item = &'a [u8]>,
    R: Iterator<Item = &'a [u8]>,
{
    loop {
        match (a.next(), b.next()) {
            (None, None) => return Ordering::Equal,
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            (Some(x), Some(y)) => match compare_natural_bytes(x, y, false) {
                Ordering::Equal => (),
                non_eq => return non_eq,
            },
        }
    }
}

/// Order `a` and `b` using natural sort order, ignoring case
#[inline]
pub fn iter_cmp_natural_ignore_case<'a, L, R>(mut a: L, mut b: R) -> Ordering
where
    L: Iterator<Item = &'a [u8]>,
    R: Iterator<Item = &'a [u8]>,
{
    loop {
        match (a.next(), b.next()) {
            (None, None) => return Ordering::Equal,
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            (Some(x), Some(y)) => match compare_natural_bytes(x, y, true) {
                Ordering::Equal => (),
                non_eq => return non_eq,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Number {
    Int(i64),
    Float(f64),
}

#[inline]
fn compare_num(n1: Number, n2: Number) -> Ordering {
    match (n1, n2) {
        (Int(i1), Int(i2)) => i1.cmp(&i2),
        #[allow(clippy::cast_precision_loss)]
        (Int(i1), Float(f2)) => compare_float(i1 as f64, f2),
        #[allow(clippy::cast_precision_loss)]
        (Float(f1), Int(i2)) => compare_float(f1, i2 as f64),
        (Float(f1), Float(f2)) => compare_float(f1, f2),
    }
}

#[allow(clippy::inline_always)]
// This function is part of a performance-critical hot path. Inlining it
// avoids the overhead of a function call, improving performance.
#[inline(always)]
fn compare_float(f1: f64, f2: f64) -> Ordering {
    f1.partial_cmp(&f2).unwrap_or(Ordering::Equal)
}

#[inline]
fn next_num<'a, X>(xs: &mut X) -> Option<Number>
where
    X: Iterator<Item = &'a [u8]>,
{
    match xs.next() {
        Some(bytes) => {
            if let Ok(i) = atoi_simd::parse::<i64, false, false>(bytes) {
                Some(Number::Int(i))
            } else {
                // If parsing as i64 failed, try parsing as f64
                from_utf8(bytes)
                    .ok()
                    .and_then(|s| s.parse::<f64>().ok())
                    .map(Number::Float)
            }
        },
        None => None,
    }
}

#[inline]
fn compare_natural_bytes(a: &[u8], b: &[u8], ignore_case: bool) -> Ordering {
    let mut a_pos = 0;
    let mut b_pos = 0;

    let mut a_byte;
    let mut b_byte;

    let mut num_comparison;
    let mut char_comparison;

    let mut a_num;
    let mut b_num;
    let mut a_end;
    let mut b_end;

    let mut a_char;
    let mut b_char;

    while a_pos < a.len() && b_pos < b.len() {
        a_byte = a[a_pos];
        b_byte = b[b_pos];

        // If both are ASCII digits, collect the full numbers and compare them
        if a_byte.is_ascii_digit() && b_byte.is_ascii_digit() {
            (a_num, a_end) = collect_number_from_bytes(a, a_pos);
            (b_num, b_end) = collect_number_from_bytes(b, b_pos);

            num_comparison = a_num.cmp(&b_num);
            if num_comparison != Ordering::Equal {
                return num_comparison;
            }

            a_pos = a_end;
            b_pos = b_end;
        } else if a_byte.is_ascii_digit() {
            // Digits come before non-digits
            return Ordering::Less;
        } else if b_byte.is_ascii_digit() {
            // Digits come before non-digits
            return Ordering::Greater;
        } else {
            // Both are non-digits, compare normally
            a_char = if ignore_case {
                a_byte.to_ascii_lowercase()
            } else {
                a_byte
            };
            b_char = if ignore_case {
                b_byte.to_ascii_lowercase()
            } else {
                b_byte
            };

            char_comparison = a_char.cmp(&b_char);
            if char_comparison != Ordering::Equal {
                return char_comparison;
            }
            a_pos += 1;
            b_pos += 1;
        }
    }

    // If we've exhausted one string but not the other
    a_pos.cmp(&b_pos)
}

#[inline]
fn collect_number_from_bytes(bytes: &[u8], start: usize) -> (i64, usize) {
    let mut pos = start;

    // Find the end of the digit sequence
    while pos < bytes.len() && bytes[pos].is_ascii_digit() {
        pos += 1;
    }

    // Parse the number using SIMD-optimized parsing
    let num = atoi_simd::parse::<i64, false, false>(&bytes[start..pos]).unwrap_or(0);
    (num, pos)
}
