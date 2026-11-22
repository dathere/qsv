//! `--discover-relationships`: find `ordered`, `correlated` and `joint` groups
//! in the source and propose them as dictionary relationships.
//!
//! Eligibility comes from the generator each column would get on its own, so
//! it can't drift from the fit: a `joint` member is enumerated from its
//! frequency pool, a `correlated` or `ordered` member is drawn from quantile
//! buckets. Faker, filler, empty and constant columns never qualify. Measures
//! come from one read of the source (every row up to `SAMPLE_ROWS`, a fixed
//! stride beyond), and use no RNG and no hash-map iteration order, so the
//! result depends only on the input and the flags.

use std::collections::{BTreeMap, HashMap, HashSet};

use super::{
    dictionary::SynthRelationship,
    faker_map,
    generator::{ColumnGenerator, parse_f64, parse_source_epoch},
    relationships::{
        EmitUnit, ScheduleParams, correlation_ratio_of_ranks, is_stratum_candidate, pearson,
        ranks_of, source_cell,
    },
};
use crate::{CliResult, config::Config};

/// Floor for both bias-corrected Cramér's V and |Spearman|.
const THRESHOLD: f64 = 0.7;
/// Most kept values a `joint` member may have.
const MAX_JOINT_VALUES: usize = 50;
/// Most candidate columns of each family (`joint`; numeric and date).
const MAX_CANDIDATES: usize = 64;
/// Most members in one discovered group.
const MAX_GROUP: usize = 8;
/// Rows read for the measures; larger inputs are read at a fixed stride.
const SAMPLE_ROWS: u64 = 100_000;
/// Most rows, in percent, where an `ordered` pair may run backwards: the fit
/// warns above this share, and clamps the gaps either way.
const MAX_REVERSED_PCT: usize = 1;
/// Share of rows a `joint` group's tuples must keep at `--min-count`.
const MIN_JOINT_SURVIVAL: f64 = 0.8;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Domain {
    Numeric,
    Date,
    DateTime,
}

/// One discovered group, by output column index.
pub(crate) struct Found {
    pub(crate) kind:    &'static str,
    /// `ordered`: low to high, the anchor first. Others: ascending.
    pub(crate) members: Vec<usize>,
    /// The weakest pair score that joined the group.
    pub(crate) score:   f64,
}

impl Found {
    /// This group as a relationship the schedule resolves like a declared one.
    pub(crate) fn relationship(&self, params: &ScheduleParams) -> SynthRelationship {
        let members: Vec<String> = self
            .members
            .iter()
            .map(|&c| params.stats_records[c].field.clone())
            .collect();
        SynthRelationship {
            kind: self.kind.to_string(),
            anchor: (self.kind == "ordered").then(|| members[0].clone()),
            members,
        }
    }

    /// The dictionary `relationships` entry for this group.
    pub(crate) fn to_json(&self, params: &ScheduleParams) -> serde_json::Value {
        let names: Vec<&str> = self
            .members
            .iter()
            .map(|&c| params.stats_records[c].field.as_str())
            .collect();
        let mut entry = serde_json::json!({
            "kind": self.kind,
            "members": names,
        });
        if self.kind == "ordered" {
            entry["anchor"] = names[0].into();
        }
        entry["score"] = ((self.score * 10_000.0).round() / 10_000.0).into();
        entry
    }
}

/// Propose relationship groups among the columns `probe` generates
/// independently, leaving out `excluded`. `probe` is the schedule built with no
/// relationships, in column order.
pub(crate) fn discover(
    params: &ScheduleParams,
    probe: &[EmitUnit],
    excluded: &HashSet<usize>,
) -> CliResult<Vec<Found>> {
    let stats = params.stats_records;
    let mut joint: Vec<usize> = Vec::new();
    let mut numeric: Vec<(usize, Domain)> = Vec::new();
    for unit in probe {
        let EmitUnit::Independent { col_idx, generator } = unit else {
            continue;
        };
        let col = *col_idx;
        if excluded.contains(&col) || params.source_cols.get(col).copied().flatten().is_none() {
            continue;
        }
        let sr = &stats[col];
        // A faker-typed column writes real values when enumerated (without
        // --consistent-fakes), and a `joint` group would link them into real
        // tuples (a first and last name), so it is never proposed.
        if params
            .content_types
            .get(&sr.field)
            .is_some_and(|ct| faker_map::is_faker_token(ct))
        {
            continue;
        }
        match (generator, sr.r#type.as_str()) {
            (ColumnGenerator::FrequencyWeighted { values, .. }, _)
                if (2..=MAX_JOINT_VALUES).contains(&values.len()) =>
            {
                joint.push(col);
            },
            // An integer unique in every filled row is an identifier, ordered
            // and correlated with anything else that grows over time.
            // (`cardinality` counts blank as one value.)
            (ColumnGenerator::NumericQuantile { .. }, ty @ ("Integer" | "Float"))
                if sr.cardinality >= 2
                    && !(ty == "Integer"
                        && sr.cardinality - u64::from(sr.nullcount > 0)
                            >= params.total_rows.saturating_sub(sr.nullcount)) =>
            {
                numeric.push((col, Domain::Numeric));
            },
            (ColumnGenerator::DateQuantile { .. }, "Date") if sr.cardinality >= 2 => {
                numeric.push((col, Domain::Date));
            },
            (ColumnGenerator::DateQuantile { .. }, "DateTime") if sr.cardinality >= 2 => {
                numeric.push((col, Domain::DateTime));
            },
            _ => {},
        }
    }
    for (family, cands) in [("joint", joint.len()), ("numeric or date", numeric.len())] {
        if cands > MAX_CANDIDATES {
            let skipped: Vec<&str> = if family == "joint" {
                joint[MAX_CANDIDATES..]
                    .iter()
                    .map(|&c| stats[c].field.as_str())
                    .collect()
            } else {
                numeric[MAX_CANDIDATES..]
                    .iter()
                    .map(|&(c, _)| stats[c].field.as_str())
                    .collect()
            };
            wwarn!(
                "synthesize: --discover-relationships checks the first {MAX_CANDIDATES} {family} \
                 columns; skipped {skipped:?}"
            );
        }
    }
    joint.truncate(MAX_CANDIDATES);
    numeric.truncate(MAX_CANDIDATES);
    // Nothing to pair: no two columns of one family, and no categorical and
    // numeric column to pair across them.
    if joint.len() < 2
        && numeric.len() < 2
        && (joint.is_empty() || !numeric.iter().any(|&(_, d)| d == Domain::Numeric))
    {
        return Ok(Vec::new());
    }

    // --- One read of the source ---------------------------------------------
    let stride = params.total_rows.div_ceil(SAMPLE_ROWS).max(1);
    let config = Config::new(Some(&params.input_path.to_string())).delimiter(params.delimiter);
    let mut rdr = config.reader()?;
    let mut record = csv::ByteRecord::new();
    let mut codebooks: Vec<HashMap<String, u32>> = vec![HashMap::new(); joint.len()];
    let mut codes: Vec<Vec<u32>> = vec![Vec::new(); joint.len()];
    let mut values: Vec<Vec<Option<f64>>> = vec![Vec::new(); numeric.len()];
    let mut sampled = 0_usize;
    let mut row = 0_u64;
    while rdr.read_byte_record(&mut record)? {
        if row.is_multiple_of(stride) {
            sampled += 1;
            for ((book, col_codes), &col) in codebooks.iter_mut().zip(&mut codes).zip(&joint) {
                #[allow(clippy::cast_possible_truncation)]
                let next = book.len() as u32;
                let code = *book
                    .entry(source_cell(&record, params, col).into_owned())
                    .or_insert(next);
                col_codes.push(code);
            }
            for (col_values, &(col, domain)) in values.iter_mut().zip(&numeric) {
                let cell = source_cell(&record, params, col);
                #[allow(clippy::cast_precision_loss)]
                col_values.push(match domain {
                    Domain::Numeric => parse_f64(&cell),
                    Domain::Date => {
                        parse_source_epoch(&cell, false, params.prefer_dmy).map(|e| e as f64)
                    },
                    Domain::DateTime => {
                        parse_source_epoch(&cell, true, params.prefer_dmy).map(|e| e as f64)
                    },
                });
            }
        }
        row += 1;
    }
    if sampled < 2 {
        return Ok(Vec::new());
    }

    let mut numeric_pos: Vec<Option<usize>> = vec![None; stats.len()];
    for (k, &(col, _)) in numeric.iter().enumerate() {
        numeric_pos[col] = Some(k);
    }
    // An ordered chain writes every member in its anchor's date format.
    let date_format = |col: usize| {
        params
            .content_types
            .get(&stats[col].field)
            .and_then(|ct| faker_map::parse_date_format(ct))
    };
    let mut owner: Vec<Option<usize>> = vec![None; stats.len()];
    let mut groups: Vec<Found> = Vec::new();

    // --- ordered, then correlated (numeric and date pairs) -------------------
    let orders: Vec<Vec<usize>> = values.iter().map(|v| sorted_present(v)).collect();
    let min_pairs = (params.min_count.saturating_mul(2).saturating_sub(1)).max(3);
    let mut ordered_edges: Vec<(f64, usize, usize)> = Vec::new();
    let mut correlated_edges: Vec<(f64, usize, usize)> = Vec::new();
    for a in 0..numeric.len() {
        for b in (a + 1)..numeric.len() {
            let (col_a, dom_a) = numeric[a];
            let (col_b, dom_b) = numeric[b];
            // Only like domains pair: numbers for `correlated`, dates of one
            // kind for `ordered`.
            if dom_a != dom_b {
                continue;
            }
            let pair = pair_stats(&values[a], &values[b], &orders[a], &orders[b]);
            if (pair.n as u64) < min_pairs {
                continue;
            }
            // Numeric pairs go to `correlated` even when one never exceeds the
            // other: a copula keeps their correlation, while an ordered chain
            // draws each gap apart from its anchor and weakens it.
            if dom_a != Domain::Numeric && pair.rho >= THRESHOLD {
                // A pair is ordered when the later date comes first in at most
                // MAX_REVERSED_PCT of the rows holding both (the fit clamps
                // those) and their ranges overlap (one wholly below the other
                // is ordered trivially). A tie, as for two columns equal in
                // every row, goes to the earlier column first, so the chain
                // keeps them equal.
                let few = |reversed: usize| reversed * 100 <= pair.n * MAX_REVERSED_PCT;
                let edge = if few(pair.n_gt) && pair.n_lt >= pair.n_gt && pair.max_a > pair.min_b {
                    Some((col_a, col_b))
                } else if few(pair.n_lt) && pair.n_gt > pair.n_lt && pair.max_b > pair.min_a {
                    Some((col_b, col_a))
                } else {
                    None
                };
                if let Some((lo, hi)) = edge
                    && date_format(lo) == date_format(hi)
                {
                    ordered_edges.push((pair.rho, lo, hi));
                }
            }
            if dom_a == Domain::Numeric && pair.rho.abs() >= THRESHOLD {
                correlated_edges.push((pair.rho.abs(), col_a, col_b));
            }
        }
    }
    sort_edges(&mut ordered_edges);
    for (score, lo, hi) in ordered_edges {
        match (owner[lo], owner[hi]) {
            (None, None) => {
                claim(&mut owner, &mut groups, "ordered", vec![lo, hi], score);
            },
            (Some(g), None)
                if groups[g].kind == "ordered"
                    && groups[g].members.last() == Some(&lo)
                    && groups[g].members.len() < MAX_GROUP =>
            {
                groups[g].members.push(hi);
                extend(&mut owner, &mut groups[g], g, hi, score);
            },
            (None, Some(g))
                if groups[g].kind == "ordered"
                    && groups[g].members.first() == Some(&hi)
                    && groups[g].members.len() < MAX_GROUP =>
            {
                groups[g].members.insert(0, lo);
                extend(&mut owner, &mut groups[g], g, lo, score);
            },
            _ => {},
        }
    }
    let complete_rows = |members: &[usize]| {
        let cols: Vec<&[Option<f64>]> = members
            .iter()
            .filter_map(|&c| numeric_pos[c].map(|k| values[k].as_slice()))
            .collect();
        (0..sampled)
            .filter(|&r| cols.iter().all(|c| c[r].is_some()))
            .count() as u64
    };
    sort_edges(&mut correlated_edges);
    for (score, a, b) in correlated_edges {
        match (owner[a], owner[b]) {
            (None, None) => claim(&mut owner, &mut groups, "correlated", vec![a, b], score),
            (Some(g), None) | (None, Some(g))
                if groups[g].kind == "correlated" && groups[g].members.len() < MAX_GROUP =>
            {
                let new = if owner[a].is_none() { a } else { b };
                // The fit correlates rows where every member holds a value; a
                // member that leaves too few would sink the whole group.
                let mut members = groups[g].members.clone();
                members.push(new);
                members.sort_unstable();
                if complete_rows(&members) >= min_pairs {
                    groups[g].members = members;
                    extend(&mut owner, &mut groups[g], g, new, score);
                }
            },
            _ => {},
        }
    }

    // --- joint ---------------------------------------------------------------
    // Survival is measured on the sampled rows, so --min-count scales with the
    // stride; below SAMPLE_ROWS rows it is exact.
    let min_kept = params.min_count.div_ceil(stride).max(1);
    let mut joint_pos: Vec<Option<usize>> = vec![None; stats.len()];
    for (k, &col) in joint.iter().enumerate() {
        joint_pos[col] = Some(k);
    }
    let fits = |members: &[usize]| {
        let cols: Vec<&[u32]> = members
            .iter()
            .filter_map(|&c| joint_pos[c].map(|k| codes[k].as_slice()))
            .collect();
        joint_survives(&cols, sampled, min_kept, params.joint_cardinality_cap)
    };
    let mut joint_edges: Vec<(f64, usize, usize)> = Vec::new();
    for a in 0..joint.len() {
        for b in (a + 1)..joint.len() {
            let v = cramers_v(&codes[a], &codes[b], codebooks[a].len(), codebooks[b].len());
            if v >= THRESHOLD {
                joint_edges.push((v, joint[a], joint[b]));
            }
        }
    }
    sort_edges(&mut joint_edges);
    for (score, a, b) in joint_edges {
        match (owner[a], owner[b]) {
            (None, None) if fits(&[a, b]) => {
                claim(&mut owner, &mut groups, "joint", vec![a, b], score);
            },
            (Some(g), None) | (None, Some(g))
                if groups[g].kind == "joint" && groups[g].members.len() < MAX_GROUP =>
            {
                let new = if owner[a].is_none() { a } else { b };
                let mut members = groups[g].members.clone();
                members.push(new);
                members.sort_unstable();
                if fits(&members) {
                    groups[g].members = members;
                    extend(&mut owner, &mut groups[g], g, new, score);
                }
            },
            _ => {},
        }
    }

    // --- categorical members of correlated groups ----------------------------
    // A categorical column whose values explain a numeric column's (η) joins
    // that column's `correlated` group as a stratum. One already in a `joint`
    // group stays there too: the fit takes the whole `joint` group into the
    // strata, so each of its tuples keeps its own numeric distributions.
    let mut mixed_edges: Vec<(f64, usize, usize)> = Vec::new();
    let candidates: Vec<(usize, usize)> = joint
        .iter()
        .enumerate()
        .filter(|&(_, &cat)| is_stratum_candidate(cat, params))
        .map(|(a, &cat)| (a, cat))
        .collect();
    // Each numeric column is ranked once, over the sampled rows holding a
    // value, and scored against every categorical candidate (blank is a
    // category) before the next; with no candidate, nothing is ranked.
    for (k, &(num, domain)) in numeric.iter().enumerate() {
        if candidates.is_empty() || domain != Domain::Numeric {
            continue;
        }
        let rows: Vec<usize> = (0..sampled).filter(|&r| values[k][r].is_some()).collect();
        if (rows.len() as u64) < min_pairs {
            continue;
        }
        let present: Vec<f64> = rows
            .iter()
            .map(|&r| values[k][r].unwrap_or_default())
            .collect();
        let ranks = ranks_of(&present);
        for &(a, cat) in &candidates {
            let groups_of: Vec<u32> = rows.iter().map(|&r| codes[a][r]).collect();
            let eta = correlation_ratio_of_ranks(&groups_of, codebooks[a].len(), &ranks);
            if eta >= THRESHOLD {
                mixed_edges.push((eta, cat, num));
            }
        }
    }
    sort_edges(&mut mixed_edges);
    // The correlated group a `joint` group's columns are strata of.
    let mut attached: Vec<Option<usize>> = vec![None; groups.len()];
    let mut dead = vec![false; groups.len()];
    // `joint` groups are settled by now, so each column's stays fixed below.
    let joint_owner: Vec<Option<usize>> = owner
        .iter()
        .map(|o| o.filter(|&g| groups[g].kind == "joint"))
        .collect();
    let joint_of = |c: usize| joint_owner[c];
    for (score, cat, num) in mixed_edges {
        let group_of_cat = match owner[cat] {
            Some(g) if groups[g].kind == "correlated" => Some(g),
            Some(g) if groups[g].kind == "joint" => attached[g],
            _ => None,
        };
        let group_of_num = owner[num];
        if group_of_num.is_some() && group_of_num == group_of_cat {
            continue;
        }
        // The group's members after this edge, and any group it swallows.
        let (target, mut members, swallowed) = match (group_of_cat, group_of_num) {
            (None, None) => (None, vec![cat, num], None),
            (None, Some(h)) => (
                Some(h),
                [groups[h].members.as_slice(), &[cat]].concat(),
                None,
            ),
            (Some(h), None) => (
                Some(h),
                [groups[h].members.as_slice(), &[num]].concat(),
                None,
            ),
            (Some(h1), Some(h2)) => (
                Some(h1),
                [groups[h1].members.as_slice(), groups[h2].members.as_slice()].concat(),
                Some(h2),
            ),
        };
        members.sort_unstable();
        members.dedup();
        // Strata: the group's categorical members plus every `joint` group
        // attached to it, or about to be.
        let mut strata: Vec<usize> = members
            .iter()
            .copied()
            .filter(|&c| numeric_pos[c].is_none())
            .collect();
        for g in 0..groups.len() {
            let joins = attached[g].is_some_and(|h| Some(h) == target || Some(h) == swallowed)
                || members.iter().any(|&c| joint_of(c) == Some(g));
            if joins {
                strata.extend(&groups[g].members);
            }
        }
        strata.sort_unstable();
        strata.dedup();
        // The cap counts the group's own members; the `joint` columns its
        // strata take in are capped by their own group.
        if members.len() > MAX_GROUP || complete_rows(&members) < min_pairs || !fits(&strata) {
            continue;
        }
        let h = target.unwrap_or_else(|| {
            groups.push(Found {
                kind: "correlated",
                members: Vec::new(),
                score,
            });
            attached.push(None);
            dead.push(false);
            groups.len() - 1
        });
        if let Some(h2) = swallowed {
            dead[h2] = true;
            groups[h].score = groups[h].score.min(groups[h2].score);
            groups[h2].members.clear();
            for a in &mut attached {
                if *a == Some(h2) {
                    *a = Some(h);
                }
            }
        }
        for &c in &members {
            match joint_of(c) {
                Some(g) if numeric_pos[c].is_none() => attached[g] = Some(h),
                _ => owner[c] = Some(h),
            }
        }
        groups[h].members = members;
        groups[h].score = groups[h].score.min(score);
    }
    Ok(groups
        .into_iter()
        .zip(dead)
        .filter_map(|(g, dead)| (!dead).then_some(g))
        .collect())
}

/// Strongest first, then by column position.
fn sort_edges(edges: &mut [(f64, usize, usize)]) {
    edges.sort_by(|x, y| y.0.total_cmp(&x.0).then(x.1.cmp(&y.1)).then(x.2.cmp(&y.2)));
}

fn claim(
    owner: &mut [Option<usize>],
    groups: &mut Vec<Found>,
    kind: &'static str,
    members: Vec<usize>,
    score: f64,
) {
    for &m in &members {
        owner[m] = Some(groups.len());
    }
    groups.push(Found {
        kind,
        members,
        score,
    });
}

fn extend(owner: &mut [Option<usize>], group: &mut Found, g: usize, col: usize, score: f64) {
    owner[col] = Some(g);
    group.score = group.score.min(score);
}

/// Row positions holding a value, in ascending value order.
fn sorted_present(values: &[Option<f64>]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..values.len()).filter(|&r| values[r].is_some()).collect();
    order.sort_by(|&x, &y| {
        values[x]
            .unwrap_or_default()
            .total_cmp(&values[y].unwrap_or_default())
    });
    order
}

struct PairStats {
    /// Rows where both columns hold a value.
    n:     usize,
    /// Spearman's rho over those rows.
    rho:   f64,
    /// Rows where a < b, and where a > b.
    n_lt:  usize,
    n_gt:  usize,
    min_a: f64,
    max_a: f64,
    min_b: f64,
    max_b: f64,
}

fn pair_stats(
    a: &[Option<f64>],
    b: &[Option<f64>],
    order_a: &[usize],
    order_b: &[usize],
) -> PairStats {
    let ranks_a = masked_ranks(order_a, a, b);
    let ranks_b = masked_ranks(order_b, b, a);
    let mut stats = PairStats {
        n:     0,
        rho:   0.0,
        n_lt:  0,
        n_gt:  0,
        min_a: f64::INFINITY,
        max_a: f64::NEG_INFINITY,
        min_b: f64::INFINITY,
        max_b: f64::NEG_INFINITY,
    };
    let mut ra = Vec::new();
    let mut rb = Vec::new();
    for (row, (x, y)) in a.iter().zip(b).enumerate() {
        let (Some(x), Some(y)) = (*x, *y) else {
            continue;
        };
        stats.n += 1;
        stats.n_lt += usize::from(x < y);
        stats.n_gt += usize::from(x > y);
        stats.min_a = stats.min_a.min(x);
        stats.max_a = stats.max_a.max(x);
        stats.min_b = stats.min_b.min(y);
        stats.max_b = stats.max_b.max(y);
        ra.push(ranks_a[row]);
        rb.push(ranks_b[row]);
    }
    if stats.n >= 2 {
        stats.rho = pearson(&ra, &rb);
    }
    stats
}

/// Fractional ranks (ties share their average) of `vals` over the rows where
/// `other` also holds a value, indexed by row; other rows get NaN. `order` is
/// `vals`' rows in value order, so this is linear.
fn masked_ranks(order: &[usize], vals: &[Option<f64>], other: &[Option<f64>]) -> Vec<f64> {
    let kept: Vec<usize> = order
        .iter()
        .copied()
        .filter(|&r| other[r].is_some())
        .collect();
    let mut ranks = vec![f64::NAN; vals.len()];
    let mut i = 0;
    while i < kept.len() {
        let mut j = i;
        while j + 1 < kept.len() && vals[kept[j + 1]] == vals[kept[i]] {
            j += 1;
        }
        #[allow(clippy::cast_precision_loss)]
        let avg = (i + j) as f64 / 2.0 + 1.0;
        for &r in &kept[i..=j] {
            ranks[r] = avg;
        }
        i = j + 1;
    }
    ranks
}

/// Bias-corrected Cramér's V (Bergsma 2013) of two coded columns with `k` and
/// `r` distinct codes. Raw V reaches 1 for any near-unique column; the
/// correction removes that inflation.
fn cramers_v(a: &[u32], b: &[u32], k: usize, r: usize) -> f64 {
    let n = a.len();
    if n < 2 || k < 2 || r < 2 {
        return 0.0;
    }
    let mut row_tot = vec![0_u64; k];
    let mut col_tot = vec![0_u64; r];
    // Ordered map: the sum below runs in key order, so the score is the same
    // on every run.
    let mut cells: BTreeMap<(u32, u32), u64> = BTreeMap::new();
    for (&x, &y) in a.iter().zip(b) {
        row_tot[x as usize] += 1;
        col_tot[y as usize] += 1;
        *cells.entry((x, y)).or_insert(0) += 1;
    }
    #[allow(clippy::cast_precision_loss)]
    let ratio: f64 = cells
        .iter()
        .map(|(&(x, y), &o)| {
            let o = o as f64;
            o * o / (row_tot[x as usize] as f64 * col_tot[y as usize] as f64)
        })
        .sum();
    #[allow(clippy::cast_precision_loss)]
    let (n, k, r) = (n as f64, k as f64, r as f64);
    let phi2 = (ratio - 1.0).max(0.0);
    let phi2c = (phi2 - (k - 1.0) * (r - 1.0) / (n - 1.0)).max(0.0);
    let kc = k - (k - 1.0).powi(2) / (n - 1.0);
    let rc = r - (r - 1.0).powi(2) / (n - 1.0);
    let denom = (kc - 1.0).min(rc - 1.0);
    if denom <= 0.0 {
        0.0
    } else {
        (phi2c / denom).sqrt().min(1.0)
    }
}

/// Whether a `joint` group on these coded columns stays within
/// `--joint-cardinality-cap` and keeps `MIN_JOINT_SURVIVAL` of the rows in
/// tuples seen at least `min_kept` times.
fn joint_survives(cols: &[&[u32]], rows: usize, min_kept: u64, cap: u64) -> bool {
    let mut counts: HashMap<Vec<u32>, u64> = HashMap::new();
    for row in 0..rows {
        let tuple: Vec<u32> = cols.iter().map(|c| c[row]).collect();
        *counts.entry(tuple).or_insert(0) += 1;
        if cap > 0 && counts.len() as u64 > cap {
            return false;
        }
    }
    // Integer sums: the iteration order doesn't change the result.
    let kept: u64 = counts.values().filter(|&&c| c >= min_kept).sum();
    #[allow(clippy::cast_precision_loss)]
    let share = kept as f64 / rows as f64;
    share >= MIN_JOINT_SURVIVAL
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cramers_v_is_one_for_a_bijection_and_zero_for_independence() {
        let a: Vec<u32> = (0..400).map(|i| i % 4).collect();
        let v = cramers_v(&a, &a, 4, 4);
        assert!((v - 1.0).abs() < 1e-9, "{v}");
        // Every combination equally often: no association.
        let b: Vec<u32> = (0..400).map(|i| (i / 4) % 4).collect();
        assert!(cramers_v(&a, &b, 4, 4) < 1e-9);
    }

    #[test]
    fn cramers_v_matches_a_reference_implementation() {
        // A 10x10 table with 18 of every 25 blocks of ten rows on the
        // diagonal. The reference sums chi-squared over every cell, in
        // Python: 0.7365520301199807.
        let (a, b): (Vec<u32>, Vec<u32>) = (0..600_u32)
            .map(|i| {
                let (p, blk) = (i % 10, i / 10);
                let q = if blk % 25 < 18 {
                    p
                } else {
                    (p + 1 + blk % 9) % 10
                };
                (p, q)
            })
            .unzip();
        let v = cramers_v(&a, &b, 10, 10);
        assert!((v - 0.736_552_030_119_980_7).abs() < 1e-12, "{v}");
    }

    #[test]
    fn pair_rho_is_spearman_over_the_rows_both_columns_hold() {
        let a = [
            Some(1.0),
            Some(4.0),
            None,
            Some(2.0),
            Some(9.0),
            Some(3.0),
            Some(3.0),
        ];
        let b = [
            Some(2.0),
            Some(1.0),
            Some(5.0),
            None,
            Some(8.0),
            Some(7.0),
            Some(6.0),
        ];
        let pair = pair_stats(&a, &b, &sorted_present(&a), &sorted_present(&b));
        let both: Vec<(f64, f64)> = a
            .iter()
            .zip(&b)
            .filter_map(|(x, y)| Some(((*x)?, (*y)?)))
            .collect();
        let columns = vec![
            both.iter().map(|p| p.0).collect::<Vec<_>>(),
            both.iter().map(|p| p.1).collect::<Vec<_>>(),
        ];
        let want = super::super::relationships::spearman_matrix(&columns)[0][1];
        assert_eq!(pair.n, both.len());
        assert!((pair.rho - want).abs() < 1e-12, "{} vs {want}", pair.rho);
    }

    #[test]
    fn masked_ranks_skip_rows_missing_in_the_other_column() {
        let vals = [Some(3.0), Some(1.0), Some(2.0), Some(2.0)];
        let other = [Some(0.0), None, Some(0.0), Some(0.0)];
        let ranks = masked_ranks(&sorted_present(&vals), &vals, &other);
        assert!(ranks[1].is_nan());
        assert_eq!(&ranks[2..], &[1.5, 1.5]);
        assert!((ranks[0] - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn joint_survival_counts_rows_in_kept_tuples() {
        // 8 rows in a common tuple, 2 in singletons: 80% survive at 2.
        let a = [0, 0, 0, 0, 0, 0, 0, 0, 1, 2];
        assert!(joint_survives(&[&a], 10, 2, 0));
        assert!(!joint_survives(&[&a], 10, 9, 0));
        // Three distinct tuples exceed a cap of 2.
        assert!(!joint_survives(&[&a], 10, 1, 2));
    }
}
