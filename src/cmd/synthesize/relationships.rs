//! Inter-column relationship resolution and the row-emission schedule.
//!
//! By default `synthesize` builds one `ColumnGenerator` per column and emits
//! every column independently. When the data dictionary declares
//! `relationships`, the columns named by a relationship are grouped and
//! generated *jointly* by a `GroupGenerator`, so inter-column structure
//! (functional dependencies, ordering, correlation) survives into the synthetic
//! output.
//!
//! This module resolves the declared relationships against the real columns,
//! learns each group's parameters from the source CSV, and produces the ordered
//! `EmitUnit` schedule the row loop walks. A column belongs to at most one
//! group; any relationship that fails resolution or validation is dropped, and
//! its columns fall back to independent generation.

use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;
use rand::rngs::StdRng;

use super::{
    dictionary::SynthRelationship,
    faker_map::{self, Locale},
    generator::{
        ColumnGenerator, compute_null_ratio, date_quartile_buckets, numeric_quartile_buckets,
        parse_epoch, parse_f64,
    },
    group_generator::{GroupGenerator, OrderedKind},
};
use crate::{
    CliError, CliResult,
    cmd::describegpt::dictionary::{FrequencyRecord, StatsRecord},
    config::{Config, Delimiter},
};

/// One unit of work in the row-emission schedule.
pub(crate) enum EmitUnit {
    /// A single column generated independently.
    Independent {
        col_idx:   usize,
        generator: ColumnGenerator,
    },
    /// A relationship group spanning several columns, generated jointly.
    Group(GroupGenerator),
}

impl EmitUnit {
    /// Smallest output column index this unit fills — used to order the
    /// schedule deterministically.
    fn primary_col_idx(&self) -> usize {
        match self {
            EmitUnit::Independent { col_idx, .. } => *col_idx,
            EmitUnit::Group(g) => g.col_indices().iter().copied().min().unwrap_or(usize::MAX),
        }
    }
}

/// Inputs needed to build the row-emission schedule.
pub(crate) struct ScheduleParams<'a> {
    pub stats_records:         &'a [StatsRecord],
    pub frequency_records:     &'a [FrequencyRecord],
    pub content_types:         &'a HashMap<String, String>,
    pub relationships:         &'a [SynthRelationship],
    pub total_rows:            u64,
    pub requested_rows:        u64,
    pub locale:                Locale,
    pub consistent_fakes:      bool,
    pub joint_cardinality_cap: u64,
    pub correlation_threshold: f64,
    pub strict:                bool,
    pub input_path:            &'a str,
    pub delimiter:             Option<Delimiter>,
}

/// Resolve the declared relationships and build the ordered `EmitUnit`
/// schedule: one `Group` unit per valid relationship plus one `Independent`
/// unit per ungrouped column.
pub(crate) fn build_emit_schedule(
    params: &ScheduleParams,
    rng: &mut StdRng,
) -> CliResult<Vec<EmitUnit>> {
    let stats = params.stats_records;
    let ncols = stats.len();

    // Column name -> output column index.
    let mut name_to_idx: HashMap<&str, usize> = HashMap::with_capacity(ncols);
    for (i, sr) in stats.iter().enumerate() {
        name_to_idx.insert(sr.field.as_str(), i);
    }

    // Resolve relationships into groups, claiming columns as we go. A column
    // belongs to at most one group; a relationship whose columns are already
    // claimed (or that fails resolution/learning) is dropped, leaving those
    // columns to be generated independently.
    let mut claimed: HashSet<usize> = HashSet::new();
    let mut groups: Vec<GroupGenerator> = Vec::new();
    for rel in params.relationships {
        if let Some(group) = resolve_relationship(rel, &name_to_idx, &claimed, params)? {
            for &idx in group.col_indices() {
                claimed.insert(idx);
            }
            groups.push(group);
        }
    }

    // Frequency lookup for the independent columns.
    let mut freq_by_field: HashMap<&str, Vec<&FrequencyRecord>> = HashMap::new();
    for fr in params.frequency_records {
        freq_by_field.entry(fr.field.as_str()).or_default().push(fr);
    }

    // Build one EmitUnit per independent (unclaimed) column. Iterating in
    // ascending column order keeps the build-time RNG draw sequence
    // deterministic for a given --seed.
    let mut schedule: Vec<EmitUnit> = Vec::with_capacity(ncols);
    for (i, sr) in stats.iter().enumerate() {
        if claimed.contains(&i) {
            continue;
        }
        let freqs = freq_by_field
            .get(sr.field.as_str())
            .map(Vec::as_slice)
            .unwrap_or_default();
        let content_type = params
            .content_types
            .get(&sr.field)
            .map_or("unknown", String::as_str);
        let generator = ColumnGenerator::build(
            sr,
            freqs,
            content_type,
            params.total_rows,
            params.requested_rows,
            params.locale,
            params.consistent_fakes,
            rng,
        );
        schedule.push(EmitUnit::Independent {
            col_idx: i,
            generator,
        });
    }
    for group in groups {
        schedule.push(EmitUnit::Group(group));
    }

    // Walk the schedule in a deterministic order during emission: sort by the
    // smallest output column index each unit fills.
    schedule.sort_by_key(EmitUnit::primary_col_idx);
    Ok(schedule)
}

/// Resolve one declared relationship to a `GroupGenerator`, or `None` when it is
/// dropped (unknown column, duplicate members, column conflict, unsupported
/// kind, or failed learning/validation).
fn resolve_relationship(
    rel: &SynthRelationship,
    name_to_idx: &HashMap<&str, usize>,
    claimed: &HashSet<usize>,
    params: &ScheduleParams,
) -> CliResult<Option<GroupGenerator>> {
    if rel.members.len() < 2 {
        log::warn!(
            "synthesize: ignoring relationship with fewer than 2 members: {:?}",
            rel.members
        );
        return Ok(None);
    }

    // Resolve member names to output column indices.
    let mut indices = Vec::with_capacity(rel.members.len());
    for member in &rel.members {
        let Some(&idx) = name_to_idx.get(member.as_str()) else {
            log::warn!(
                "synthesize: ignoring '{}' relationship — unknown column '{member}'",
                rel.kind
            );
            return Ok(None);
        };
        indices.push(idx);
    }

    // Reject a relationship that names the same column twice.
    let mut seen = HashSet::with_capacity(indices.len());
    if !indices.iter().all(|&i| seen.insert(i)) {
        log::warn!(
            "synthesize: ignoring '{}' relationship with duplicate members: {:?}",
            rel.kind,
            rel.members
        );
        return Ok(None);
    }

    // A column may belong to only one group — the first relationship wins.
    if let Some(conflict) = indices.iter().copied().find(|i| claimed.contains(i)) {
        log::warn!(
            "synthesize: column '{}' is already in another relationship; ignoring '{}' \
             relationship {:?}",
            params.stats_records[conflict].field,
            rel.kind,
            rel.members
        );
        return Ok(None);
    }

    match rel.kind.as_str() {
        "joint" => resolve_joint(&indices, params),
        "ordered" => resolve_ordered(rel, &indices, params),
        "correlated" => resolve_correlated(&indices, params),
        other => {
            log::warn!(
                "synthesize: ignoring relationship with unknown kind '{other}': {:?}",
                rel.members
            );
            Ok(None)
        },
    }
}

/// Build a `GroupGenerator::Joint` from the source's observed value-tuples.
fn resolve_joint(indices: &[usize], params: &ScheduleParams) -> CliResult<Option<GroupGenerator>> {
    // Tuple values are stored in ascending column order.
    let mut col_indices = indices.to_vec();
    col_indices.sort_unstable();

    let names: Vec<&str> = col_indices
        .iter()
        .map(|&i| params.stats_records[i].field.as_str())
        .collect();

    match learn_joint_tuples(&col_indices, params)? {
        JointLearn::Ok { tuples, cumulative } => {
            log::info!(
                "synthesize: joint relationship on {names:?} — {} distinct tuple(s)",
                tuples.len()
            );
            Ok(Some(GroupGenerator::Joint {
                col_indices,
                tuples,
                cumulative,
            }))
        },
        JointLearn::OverCap(distinct) => {
            let msg = format!(
                "joint relationship on {names:?} has at least {distinct} distinct tuples, \
                 exceeding --joint-cardinality-cap ({})",
                params.joint_cardinality_cap
            );
            if params.strict {
                return Err(CliError::Other(format!(
                    "synthesize: {msg}. Raise --joint-cardinality-cap or drop the relationship."
                )));
            }
            log::warn!("synthesize: {msg}; columns will be generated independently");
            Ok(None)
        },
        JointLearn::Empty => {
            log::warn!(
                "synthesize: joint relationship on {names:?} found no source rows; columns will \
                 be generated independently"
            );
            Ok(None)
        },
    }
}

/// Outcome of learning the distinct value-tuples of a joint relationship group.
enum JointLearn {
    /// Distinct tuples and their normalized cumulative weights.
    Ok {
        tuples:     Vec<Vec<String>>,
        cumulative: Vec<f64>,
    },
    /// Distinct-tuple count exceeded `--joint-cardinality-cap`.
    OverCap(usize),
    /// The source has no data rows.
    Empty,
}

/// Stream the source CSV once, counting the distinct value-tuples formed by the
/// group's member columns. Bails out early as `OverCap` once the distinct count
/// exceeds `--joint-cardinality-cap` (0 = unlimited), so memory stays bounded.
fn learn_joint_tuples(col_indices: &[usize], params: &ScheduleParams) -> CliResult<JointLearn> {
    let input = params.input_path.to_string();
    let config = Config::new(Some(&input)).delimiter(params.delimiter);
    let mut rdr = config.reader()?;

    let cap = params.joint_cardinality_cap;
    let mut counts: IndexMap<Vec<String>, u64> = IndexMap::new();
    let mut record = csv::StringRecord::new();
    while rdr.read_record(&mut record)? {
        let tuple: Vec<String> = col_indices
            .iter()
            .map(|&i| record.get(i).unwrap_or_default().to_string())
            .collect();
        *counts.entry(tuple).or_insert(0) += 1;
        if cap > 0 && counts.len() as u64 > cap {
            return Ok(JointLearn::OverCap(counts.len()));
        }
    }
    if counts.is_empty() {
        return Ok(JointLearn::Empty);
    }

    // Sort tuples lexicographically for a stable, reproducible order
    // independent of source row order.
    let mut pairs: Vec<(Vec<String>, u64)> = counts.into_iter().collect();
    pairs.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    #[allow(clippy::cast_precision_loss)]
    let total: f64 = pairs.iter().map(|(_, c)| *c as f64).sum();
    let mut tuples = Vec::with_capacity(pairs.len());
    let mut cumulative = Vec::with_capacity(pairs.len());
    let mut acc = 0.0_f64;
    for (tuple, count) in pairs {
        #[allow(clippy::cast_precision_loss)]
        {
            acc += count as f64 / total;
        }
        tuples.push(tuple);
        cumulative.push(acc);
    }
    // Pin the last element to exactly 1.0 to avoid floating-point drift.
    if let Some(last) = cumulative.last_mut() {
        *last = 1.0;
    }
    Ok(JointLearn::Ok { tuples, cumulative })
}

/// Build a `GroupGenerator::Ordered` (anchor + learned gap) from a monotonic
/// chain of columns. `indices` are the member output-column indices in
/// member (low-to-high) declared order; `indices[0]` is the anchor.
fn resolve_ordered(
    rel: &SynthRelationship,
    indices: &[usize],
    params: &ScheduleParams,
) -> CliResult<Option<GroupGenerator>> {
    let stats = params.stats_records;
    let member_cols = indices.to_vec();
    let names: Vec<&str> = member_cols
        .iter()
        .map(|&i| stats[i].field.as_str())
        .collect();

    // The anchor is always the first member; warn if `anchor` says otherwise.
    if let Some(anchor) = &rel.anchor
        && anchor != &rel.members[0]
    {
        log::warn!(
            "synthesize: ordered relationship {names:?} — declared anchor '{anchor}' is not the \
             first member; using '{}' as the anchor",
            rel.members[0]
        );
    }

    // Every member must share a numeric or date domain.
    let types: Vec<&str> = member_cols
        .iter()
        .map(|&i| stats[i].r#type.as_str())
        .collect();
    let Some(kind) = classify_ordered_kind(&types) else {
        log::warn!(
            "synthesize: ordered relationship {names:?} dropped — members must all be numeric, \
             all Date, or all DateTime"
        );
        return Ok(None);
    };
    // Per-member integer formatting — keeps an Integer column emitting integers
    // even in a mixed Integer/Float ordered group.
    let is_int: Vec<bool> = types.iter().map(|t| *t == "Integer").collect();

    // Anchor distribution, expressed in the numeric domain.
    let anchor_stats = &stats[member_cols[0]];
    let anchor_buckets: Vec<(f64, f64)> = match kind {
        OrderedKind::Numeric => {
            if let Some(buckets) = numeric_quartile_buckets(anchor_stats) {
                buckets
            } else {
                log::warn!(
                    "synthesize: ordered relationship {names:?} dropped — anchor '{}' has no \
                     usable numeric range",
                    names[0]
                );
                return Ok(None);
            }
        },
        OrderedKind::Date | OrderedKind::DateTime => {
            let is_datetime = kind == OrderedKind::DateTime;
            if let Some(buckets) = date_quartile_buckets(anchor_stats, is_datetime) {
                #[allow(clippy::cast_precision_loss)]
                buckets
                    .into_iter()
                    .map(|(lo, hi)| (lo as f64, hi as f64))
                    .collect()
            } else {
                log::warn!(
                    "synthesize: ordered relationship {names:?} dropped — anchor '{}' has no \
                     usable date range",
                    names[0]
                );
                return Ok(None);
            }
        },
    };

    // Learn the consecutive-gap distributions from the source.
    let gap_samples = learn_ordered_gaps(&member_cols, kind, params)?;
    let mut gap_buckets = Vec::with_capacity(gap_samples.len());
    for (i, samples) in gap_samples.into_iter().enumerate() {
        if samples.is_empty() {
            log::warn!(
                "synthesize: ordered relationship {names:?} dropped — no rows with both '{}' and \
                 '{}' present",
                names[i],
                names[i + 1]
            );
            return Ok(None);
        }
        // Negative gaps mean the source itself violates the declared order.
        #[allow(clippy::cast_precision_loss)]
        let neg_frac = samples.iter().filter(|g| **g < 0.0).count() as f64 / samples.len() as f64;
        if neg_frac > 0.01 {
            let msg = format!(
                "ordered relationship {names:?}: '{}' precedes '{}' in {:.1}% of source rows",
                names[i + 1],
                names[i],
                neg_frac * 100.0
            );
            if params.strict {
                return Err(CliError::Other(format!(
                    "synthesize: {msg}. Fix the source ordering or drop the relationship."
                )));
            }
            log::warn!(
                "synthesize: {msg}; such gaps are clamped to keep the synthetic output ordered"
            );
        }
        gap_buckets.push(gap_buckets_from_samples(samples));
    }

    let null_ratio = compute_null_ratio(anchor_stats.nullcount, params.total_rows);
    let date_format = ordered_date_format(&member_cols, params, kind);

    log::info!(
        "synthesize: ordered relationship on {names:?} — anchor '{}'",
        names[0]
    );
    Ok(Some(GroupGenerator::Ordered {
        member_cols,
        kind,
        is_int,
        date_format,
        anchor_buckets,
        gap_buckets,
        null_ratio,
    }))
}

/// Classify the shared domain of an ordered group's member columns. Numeric
/// members (Integer and/or Float) share the `Numeric` domain — per-member
/// integer formatting is tracked separately. Returns `None` for a mixed or
/// non-orderable set of `stats` types.
fn classify_ordered_kind(types: &[&str]) -> Option<OrderedKind> {
    if types.iter().all(|t| matches!(*t, "Integer" | "Float")) {
        Some(OrderedKind::Numeric)
    } else if types.iter().all(|t| *t == "Date") {
        Some(OrderedKind::Date)
    } else if types.iter().all(|t| *t == "DateTime") {
        Some(OrderedKind::DateTime)
    } else {
        None
    }
}

/// The chrono strftime output format for a date-typed ordered group, taken from
/// the anchor column's dictionary `content_type` (a `date:<fmt>` /
/// `datetime:<fmt>` token). `None` for numeric groups or untagged date columns.
fn ordered_date_format(
    member_cols: &[usize],
    params: &ScheduleParams,
    kind: OrderedKind,
) -> Option<String> {
    if !matches!(kind, OrderedKind::Date | OrderedKind::DateTime) {
        return None;
    }
    let anchor_name = &params.stats_records[member_cols[0]].field;
    params
        .content_types
        .get(anchor_name)
        .and_then(|ct| faker_map::parse_date_format(ct))
}

/// Stream the source CSV once, collecting the consecutive gaps
/// `member[i + 1] - member[i]` for every adjacent member pair. Values are
/// parsed into the group's numeric domain; rows where either member of a pair
/// is null/unparseable contribute no gap for that pair.
fn learn_ordered_gaps(
    member_cols: &[usize],
    kind: OrderedKind,
    params: &ScheduleParams,
) -> CliResult<Vec<Vec<f64>>> {
    let input = params.input_path.to_string();
    let config = Config::new(Some(&input)).delimiter(params.delimiter);
    let mut rdr = config.reader()?;

    let npairs = member_cols.len() - 1;
    let mut gaps: Vec<Vec<f64>> = vec![Vec::new(); npairs];
    let is_date = matches!(kind, OrderedKind::Date | OrderedKind::DateTime);
    let is_datetime = kind == OrderedKind::DateTime;

    let mut record = csv::StringRecord::new();
    let mut values: Vec<Option<f64>> = vec![None; member_cols.len()];
    while rdr.read_record(&mut record)? {
        for (slot, &col) in values.iter_mut().zip(member_cols) {
            let cell = record.get(col).unwrap_or_default();
            *slot = if is_date {
                #[allow(clippy::cast_precision_loss)]
                parse_epoch(cell, is_datetime).map(|e| e as f64)
            } else {
                parse_f64(cell)
            };
        }
        for (i, pair_gaps) in gaps.iter_mut().enumerate() {
            if let (Some(lo), Some(hi)) = (values[i], values[i + 1]) {
                pair_gaps.push(hi - lo);
            }
        }
    }
    Ok(gaps)
}

/// Build clamped quartile gap buckets from a non-empty set of gap samples.
/// Every bound is clamped to >= 0 so the synthetic chain is always ordered.
fn gap_buckets_from_samples(mut samples: Vec<f64>) -> Vec<(f64, f64)> {
    samples.sort_by(f64::total_cmp);
    let n = samples.len();
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    let at = |frac: f64| samples[((n as f64 * frac) as usize).min(n - 1)];
    let mut bounds = [samples[0], at(0.25), at(0.5), at(0.75), samples[n - 1]];
    // Clamping a sorted sequence to >= 0 keeps it sorted.
    for b in &mut bounds {
        *b = b.max(0.0);
    }
    let [min, q1, q2, q3, max] = bounds;
    if min < max {
        vec![(min, q1), (q1, q2), (q2, q3), (q3, max)]
    } else {
        vec![(min, min)]
    }
}

/// Build a `GroupGenerator::Correlated` (Gaussian copula) for a set of
/// numeric columns whose correlation should be preserved.
fn resolve_correlated(
    indices: &[usize],
    params: &ScheduleParams,
) -> CliResult<Option<GroupGenerator>> {
    let stats = params.stats_records;
    let member_cols: Vec<usize> = indices.to_vec();
    let names: Vec<&str> = member_cols
        .iter()
        .map(|&i| stats[i].field.as_str())
        .collect();

    // Every member must be numeric.
    if let Some(bad) = member_cols
        .iter()
        .find(|&&i| !matches!(stats[i].r#type.as_str(), "Integer" | "Float"))
    {
        log::warn!(
            "synthesize: correlated relationship {names:?} dropped — column '{}' is not numeric",
            stats[*bad].field
        );
        return Ok(None);
    }

    // Collect complete-case rows and estimate the Spearman correlation matrix.
    let columns = learn_correlated_columns(&member_cols, params)?;
    if columns[0].len() < 2 {
        log::warn!(
            "synthesize: correlated relationship {names:?} dropped — fewer than 2 rows have every \
             member present"
        );
        return Ok(None);
    }
    let corr = spearman_matrix(&columns);

    // Keep only members correlated with at least one other at the threshold.
    let k = member_cols.len();
    let threshold = params.correlation_threshold.abs();
    let keep: Vec<bool> = (0..k)
        .map(|m| (0..k).any(|j| j != m && corr[m][j].abs() >= threshold))
        .collect();
    let kept: Vec<usize> = (0..k).filter(|&m| keep[m]).collect();

    if kept.len() < k {
        let dropped: Vec<&str> = (0..k).filter(|&m| !keep[m]).map(|m| names[m]).collect();
        let msg = format!(
            "correlated relationship {names:?}: {dropped:?} are not correlated with the group at \
             the |Spearman| >= {threshold} threshold"
        );
        if params.strict {
            return Err(CliError::Other(format!(
                "synthesize: {msg}. Lower --correlation-threshold or drop those members."
            )));
        }
        log::warn!("synthesize: {msg}; dropping them from the relationship");
    }
    if kept.len() < 2 {
        log::warn!(
            "synthesize: correlated relationship {names:?} dropped — fewer than 2 members remain \
             after the correlation-threshold filter"
        );
        return Ok(None);
    }

    // Subset to the kept members.
    let kept_cols: Vec<usize> = kept.iter().map(|&m| member_cols[m]).collect();
    let kept_corr: Vec<Vec<f64>> = kept
        .iter()
        .map(|&a| kept.iter().map(|&b| corr[a][b]).collect())
        .collect();

    // Per-column marginals (quartile buckets), integer flags, and null ratios.
    // Each member keeps its own marginal null ratio — the copula couples the
    // non-null values without forcing nulls to co-occur.
    let mut marginals = Vec::with_capacity(kept_cols.len());
    let mut is_int = Vec::with_capacity(kept_cols.len());
    let mut null_ratios = Vec::with_capacity(kept_cols.len());
    for &col in &kept_cols {
        let sr = &stats[col];
        let Some(buckets) = numeric_quartile_buckets(sr) else {
            log::warn!(
                "synthesize: correlated relationship {names:?} dropped — column '{}' has no \
                 usable numeric range",
                sr.field
            );
            return Ok(None);
        };
        marginals.push(buckets);
        is_int.push(sr.r#type == "Integer");
        null_ratios.push(compute_null_ratio(sr.nullcount, params.total_rows));
    }

    let cholesky_l = cholesky_ridge(&kept_corr);

    let kept_names: Vec<&str> = kept.iter().map(|&m| names[m]).collect();
    log::info!("synthesize: correlated relationship on {kept_names:?}");
    Ok(Some(GroupGenerator::Correlated {
        col_indices: kept_cols,
        cholesky_l,
        marginals,
        is_int,
        null_ratios,
    }))
}

/// Stream the source CSV once, collecting the numeric values of the member
/// columns for every *complete-case* row (every member present and numeric).
/// Returns one value vector per member, all the same length.
fn learn_correlated_columns(
    member_cols: &[usize],
    params: &ScheduleParams,
) -> CliResult<Vec<Vec<f64>>> {
    let input = params.input_path.to_string();
    let config = Config::new(Some(&input)).delimiter(params.delimiter);
    let mut rdr = config.reader()?;

    let k = member_cols.len();
    let mut columns: Vec<Vec<f64>> = vec![Vec::new(); k];
    let mut record = csv::StringRecord::new();
    let mut rowvals: Vec<f64> = Vec::with_capacity(k);
    while rdr.read_record(&mut record)? {
        rowvals.clear();
        for &col in member_cols {
            match parse_f64(record.get(col).unwrap_or_default()) {
                Some(v) => rowvals.push(v),
                None => break,
            }
        }
        if rowvals.len() == k {
            for (column, &v) in columns.iter_mut().zip(&rowvals) {
                column.push(v);
            }
        }
    }
    Ok(columns)
}

/// The Spearman rank-correlation matrix of the given columns. Spearman (rather
/// than Pearson) is used because the copula operates on ranks, and rank
/// correlation survives the marginal transform.
fn spearman_matrix(columns: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let k = columns.len();
    let ranks: Vec<Vec<f64>> = columns.iter().map(|c| fractional_ranks(c)).collect();
    let mut matrix = vec![vec![0.0_f64; k]; k];
    for i in 0..k {
        matrix[i][i] = 1.0;
        for j in (i + 1)..k {
            let c = pearson(&ranks[i], &ranks[j]);
            matrix[i][j] = c;
            matrix[j][i] = c;
        }
    }
    matrix
}

/// Fractional ranks (ties share their average rank) of a value vector.
fn fractional_ranks(values: &[f64]) -> Vec<f64> {
    let n = values.len();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| values[a].total_cmp(&values[b]));
    let mut ranks = vec![0.0_f64; n];
    let mut i = 0;
    while i < n {
        let mut j = i;
        // Tie detection for average-rank assignment: we want bit-exact equality
        // of sort keys, not "within epsilon" — equal-key runs share a rank.
        #[allow(clippy::float_cmp)]
        while j + 1 < n && values[order[j + 1]] == values[order[i]] {
            j += 1;
        }
        #[allow(clippy::cast_precision_loss)]
        let avg = (i + j) as f64 / 2.0 + 1.0;
        for &orig in &order[i..=j] {
            ranks[orig] = avg;
        }
        i = j + 1;
    }
    ranks
}

/// Pearson correlation of two equal-length vectors. A zero-variance input
/// yields `0.0` (an undefined correlation is treated as "uncorrelated").
fn pearson(a: &[f64], b: &[f64]) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let n = a.len() as f64;
    let mean_a = a.iter().sum::<f64>() / n;
    let mean_b = b.iter().sum::<f64>() / n;
    let (mut cov, mut var_a, mut var_b) = (0.0_f64, 0.0_f64, 0.0_f64);
    for (x, y) in a.iter().zip(b) {
        let da = x - mean_a;
        let db = y - mean_b;
        cov = da.mul_add(db, cov);
        var_a = da.mul_add(da, var_a);
        var_b = db.mul_add(db, var_b);
    }
    if var_a <= 0.0 || var_b <= 0.0 {
        0.0
    } else {
        (cov / (var_a.sqrt() * var_b.sqrt())).clamp(-1.0, 1.0)
    }
}

/// Cholesky factorization of a correlation matrix, with a ridge fallback for
/// the not-quite-positive-definite estimates that pairwise Spearman can
/// produce. Each row of the result is normalized to unit length, so `L z`
/// (with `z` standard normal) has components of exactly unit variance — the
/// copula keeps every marginal intact regardless of the ridge.
fn cholesky_ridge(corr: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let k = corr.len();
    let mut ridge = 0.0_f64;
    let mut l = loop {
        if let Some(factor) = cholesky(corr, ridge) {
            break factor;
        }
        ridge = if ridge == 0.0 { 1e-9 } else { ridge * 10.0 };
        if ridge > 1.0 {
            // Pathological matrix — fall back to the identity (independent
            // columns); marginals are still reproduced exactly.
            break identity_matrix(k);
        }
    };
    for row in &mut l {
        let norm = row.iter().map(|v| v * v).sum::<f64>().sqrt();
        if norm > 0.0 {
            for v in row.iter_mut() {
                *v /= norm;
            }
        }
    }
    l
}

/// Lower-triangular Cholesky factor of `corr + ridge * I`, or `None` when that
/// matrix is not positive definite.
#[allow(clippy::needless_range_loop)]
fn cholesky(corr: &[Vec<f64>], ridge: f64) -> Option<Vec<Vec<f64>>> {
    let k = corr.len();
    let mut l = vec![vec![0.0_f64; k]; k];
    for i in 0..k {
        for j in 0..=i {
            let mut sum = corr[i][j];
            if i == j {
                sum += ridge;
            }
            for m in 0..j {
                sum = l[i][m].mul_add(-l[j][m], sum);
            }
            if i == j {
                if sum <= 1e-12 {
                    return None;
                }
                l[i][i] = sum.sqrt();
            } else {
                l[i][j] = sum / l[j][j];
            }
        }
    }
    Some(l)
}

/// The `k`-by-`k` identity matrix.
fn identity_matrix(k: usize) -> Vec<Vec<f64>> {
    let mut m = vec![vec![0.0_f64; k]; k];
    for (i, row) in m.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    m
}
