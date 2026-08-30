use chrono::format::{parse, Item, Parsed, StrftimeItems};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::sync::LazyLock;
use std::time::Instant;

// the exact 4-digit-year chain qsv-dateparser's slash_mdy_hms walks
const FMTS: [&str; 5] = [
    "%m/%d/%Y %H:%M:%S", "%m/%d/%Y %H:%M", "%m/%d/%Y %H:%M:%S%.f",
    "%m/%d/%Y %I:%M:%S %P", "%m/%d/%Y %I:%M %P",
];

// B: parse each format string ONCE into Items
static ITEMS: LazyLock<Vec<Vec<Item<'static>>>> = LazyLock::new(|| {
    FMTS.iter().map(|f| StrftimeItems::new(f).parse().unwrap()).collect()
});

#[inline]
fn current(input: &str) -> Option<NaiveDateTime> {
    for f in FMTS { if let Ok(d) = NaiveDateTime::parse_from_str(input, f) { return Some(d); } }
    None
}
#[inline]
fn precompiled(input: &str) -> Option<NaiveDateTime> {
    for items in ITEMS.iter() {
        let mut p = Parsed::new();
        if parse(&mut p, input, items.iter()).is_ok() {
            if let Ok(d) = p.to_naive_datetime_with_offset(0) { return Some(d); }
        }
    }
    None
}

fn main() {
    let data: Vec<String> = std::fs::read_to_string("/tmp/dates.txt").unwrap()
        .lines().map(str::to_string).collect();
    LazyLock::force(&ITEMS);
    // equivalence
    let mut ok = 0usize;
    for s in &data { assert_eq!(current(s), precompiled(s), "mismatch on {s}"); if current(s).is_some() { ok+=1; } }
    println!("{} values, {} parsed OK, outputs identical\n", data.len(), ok);
    let (mut a, mut b) = (f64::MAX, f64::MAX);
    for _ in 0..5 {
        let t=Instant::now(); let mut n=0u64;
        for s in &data { if current(s).is_some() { n+=1; } }
        let e=t.elapsed().as_secs_f64(); if e<a {a=e;} std::hint::black_box(n);
        let t=Instant::now(); let mut n=0u64;
        for s in &data { if precompiled(s).is_some() { n+=1; } }
        let e=t.elapsed().as_secs_f64(); if e<b {b=e;} std::hint::black_box(n);
    }
    println!("  A current (parse_from_str, format re-parsed each call) : {:8.2} ms  ({:.0} ns/value)", a*1e3, a*1e9/data.len() as f64);
    println!("  B precompiled Vec<Item> (parse format once)            : {:8.2} ms  ({:.0} ns/value)", b*1e3, b*1e9/data.len() as f64);
    println!("  delta                                                 : {:+.1}%", 100.0*(b-a)/a);
    let _ = DateTime::<Utc>::from_timestamp(0,0);
}
