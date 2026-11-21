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

use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap, HashSet},
};

use indexmap::IndexMap;
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};

use super::{
    dictionary::SynthRelationship,
    faker_map::{self, Locale},
    generator::{
        ColumnGenerator, compute_null_ratio, date_buckets, effective_quantiles, float_decimals,
        floor_to, numeric_buckets, parse_f64, parse_source_epoch,
    },
    group_generator::{GroupGenerator, OrderedKind, Stratum, member_grid},
    report::Suppression,
};
use crate::{
    CliError, CliResult,
    cmd::describegpt::dictionary::{FrequencyRecord, StatsRecord},
    config::{Config, Delimiter},
};

/// One unit of work in the row-emission schedule.
#[derive(Serialize, Deserialize)]
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
    pub min_count:             u64,
    /// `--quantiles`: bucket count for numeric/date marginals and ordered gaps.
    pub quantiles:             usize,
    /// Source CSV column of each stats record (see `source_column_indices`).
    pub source_cols:           &'a [Option<usize>],
    pub correlation_threshold: f64,
    pub strict:                bool,
    pub input_path:            &'a str,
    pub delimiter:             Option<Delimiter>,
    /// Day-first reading of ambiguous source dates, as `stats` read them.
    pub prefer_dmy:            bool,
    /// What `min_count` dropped, per column or joint group, for `--report`.
    pub suppressions:          &'a RefCell<Vec<Suppression>>,
    /// Keep per-column warnings off stderr (the discovery probe, whose
    /// columns the real build warns about).
    pub quiet:                 bool,
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
    let (plans, absorbed) = plan_stratified(&name_to_idx, params);
    for (r, rel) in params.relationships.iter().enumerate() {
        if absorbed[r] {
            continue;
        }
        let resolved = match &plans[r] {
            Some(plan) => resolve_planned(plan, &name_to_idx, &claimed, params)?,
            None => resolve_relationship(rel, &name_to_idx, &claimed, params)?
                .into_iter()
                .collect(),
        };
        for group in resolved {
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
        let mut suppressed = None;
        let generator = ColumnGenerator::build_tracked(
            sr,
            freqs,
            content_type,
            params.total_rows,
            params.requested_rows,
            params.locale,
            params.consistent_fakes,
            params.min_count,
            &mut suppressed,
            !params.quiet,
            rng,
        );
        if let Some((distinct, rows)) = suppressed {
            params.suppressions.borrow_mut().push(Suppression {
                columns: vec![sr.field.clone()],
                distinct,
                rows,
                class: None,
            });
        }
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

/// Most distinct values a categorical `correlated` member may have, as for a
/// discovered `joint` member.
pub(super) const MAX_STRATUM_VALUES: u64 = 50;

/// Whether a column can be a categorical member of a `correlated` group: not a
/// number or a date, 1 to `MAX_STRATUM_VALUES` values, and not a faker column,
/// whose real values the group would print.
pub(super) fn is_stratum_candidate(col: usize, params: &ScheduleParams) -> bool {
    let sr = &params.stats_records[col];
    if matches!(
        sr.r#type.as_str(),
        "Integer" | "Float" | "Date" | "DateTime" | "NULL"
    ) || params
        .content_types
        .get(&sr.field)
        .is_some_and(|ct| faker_map::is_faker_token(ct))
    {
        return false;
    }
    // `cardinality` counts blank as one value.
    let values = sr.cardinality.saturating_sub(u64::from(sr.nullcount > 0));
    (1..=MAX_STRATUM_VALUES).contains(&values)
}

/// A `correlated` relationship with categorical members, to be generated by
/// strata, and the `joint` relationships it takes in because they share one of
/// those members: their columns become strata too.
struct StratifiedPlan {
    numeric:  Vec<usize>,
    /// Ascending.
    strata:   Vec<usize>,
    /// Positions in `strata` of the relationship's own categorical members.
    core:     Vec<usize>,
    /// Indices into `ScheduleParams::relationships`.
    absorbed: Vec<usize>,
}

/// Plan the stratified `correlated` relationships, by relationship index, and
/// mark the `joint` relationships they take in. A relationship with no
/// categorical member, or one that can't be a stratum, gets no plan and
/// resolves as before.
fn plan_stratified(
    name_to_idx: &HashMap<&str, usize>,
    params: &ScheduleParams,
) -> (Vec<Option<StratifiedPlan>>, Vec<bool>) {
    let rels = params.relationships;
    let indices = |rel: &SynthRelationship| -> Option<Vec<usize>> {
        rel.members
            .iter()
            .map(|m| name_to_idx.get(m.as_str()).copied())
            .collect()
    };
    let mut plans: Vec<Option<StratifiedPlan>> = Vec::with_capacity(rels.len());
    let mut absorbed = vec![false; rels.len()];
    for (r, rel) in rels.iter().enumerate() {
        let plan = (rel.kind == "correlated" && !absorbed[r])
            .then(|| indices(rel))
            .flatten()
            .and_then(|cols| {
                let (numeric, categorical): (Vec<usize>, Vec<usize>) =
                    cols.iter().partition(|&&c| {
                        matches!(params.stats_records[c].r#type.as_str(), "Integer" | "Float")
                    });
                (!numeric.is_empty()
                    && !categorical.is_empty()
                    && categorical.iter().all(|&c| is_stratum_candidate(c, params)))
                .then_some((numeric, categorical))
            })
            .map(|(numeric, declared)| {
                let mut strata = declared.clone();
                let mut taken_in = Vec::new();
                for (j, other) in rels.iter().enumerate() {
                    if j == r || absorbed[j] || other.kind != "joint" {
                        continue;
                    }
                    let Some(cols) = indices(other) else {
                        continue;
                    };
                    if cols.iter().any(|c| strata.contains(c))
                        && !cols.iter().any(|c| numeric.contains(c))
                    {
                        absorbed[j] = true;
                        taken_in.push(j);
                        strata.extend(cols);
                    }
                }
                strata.sort_unstable();
                strata.dedup();
                let core = (0..strata.len())
                    .filter(|&p| declared.contains(&strata[p]))
                    .collect();
                StratifiedPlan {
                    numeric,
                    strata,
                    core,
                    absorbed: taken_in,
                }
            });
        plans.push(plan);
    }
    (plans, absorbed)
}

/// Resolve a planned stratified group. When it can't be built, its numeric
/// members fall back to a plain `correlated` group (with at least two) and the
/// `joint` relationships it took in resolve on their own.
fn resolve_planned(
    plan: &StratifiedPlan,
    name_to_idx: &HashMap<&str, usize>,
    claimed: &HashSet<usize>,
    params: &ScheduleParams,
) -> CliResult<Vec<GroupGenerator>> {
    let stats = params.stats_records;
    let names =
        |cols: &[usize]| -> Vec<String> { cols.iter().map(|&c| stats[c].field.clone()).collect() };
    let all: Vec<usize> = plan.numeric.iter().chain(&plan.strata).copied().collect();
    if members_usable("correlated", &names(&all), &all, claimed, params)
        && let Some(group) = resolve_stratified(&plan.numeric, &plan.strata, &plan.core, params)?
    {
        return Ok(vec![group]);
    }
    let mut taken = claimed.clone();
    let mut out = Vec::new();
    if plan.numeric.len() >= 2
        && members_usable(
            "correlated",
            &names(&plan.numeric),
            &plan.numeric,
            &taken,
            params,
        )
        && let Some(group) = resolve_correlated(&plan.numeric, params)?
    {
        taken.extend(group.col_indices());
        out.push(group);
    }
    for &j in &plan.absorbed {
        if let Some(group) =
            resolve_relationship(&params.relationships[j], name_to_idx, &taken, params)?
        {
            taken.extend(group.col_indices());
            out.push(group);
        }
    }
    Ok(out)
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

    if !members_usable(&rel.kind, &rel.members, &indices, claimed, params) {
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

/// Whether a relationship's resolved members can form a group: no column twice,
/// each matched to a source column, and none already in another group. Warns
/// and returns false otherwise.
fn members_usable(
    kind: &str,
    members: &[String],
    indices: &[usize],
    claimed: &HashSet<usize>,
    params: &ScheduleParams,
) -> bool {
    // Reject a relationship that names the same column twice.
    let mut seen = HashSet::with_capacity(indices.len());
    if !indices.iter().all(|&i| seen.insert(i)) {
        log::warn!(
            "synthesize: ignoring '{kind}' relationship with duplicate members: {members:?}"
        );
        return false;
    }

    // Every member needs a resolved source column; guessing would learn from
    // the wrong one.
    if let Some(&i) = indices
        .iter()
        .find(|&&i| params.source_cols.get(i).copied().flatten().is_none())
    {
        log::warn!(
            "synthesize: ignoring '{kind}' relationship — column '{}' can't be matched to a \
             unique source column",
            params.stats_records[i].field
        );
        return false;
    }

    // A column may belong to only one group — the first relationship wins.
    if let Some(conflict) = indices.iter().copied().find(|i| claimed.contains(i)) {
        log::warn!(
            "synthesize: column '{}' is already in another relationship; ignoring '{kind}' \
             relationship {members:?}",
            params.stats_records[conflict].field,
        );
        return false;
    }
    true
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
        JointLearn::Ok {
            tuples,
            cumulative,
            suppressed,
        } => {
            if suppressed.0 > 0 {
                params.suppressions.borrow_mut().push(Suppression {
                    columns:  names.iter().map(|n| (*n).to_string()).collect(),
                    distinct: suppressed.0,
                    rows:     suppressed.1,
                    class:    None,
                });
            }
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
        JointLearn::AllSuppressed(distinct, rows) => {
            params.suppressions.borrow_mut().push(Suppression {
                columns: names.iter().map(|n| (*n).to_string()).collect(),
                distinct,
                rows,
                class: None,
            });
            let msg = format!(
                "joint relationship on {names:?}: all {distinct} distinct tuple(s) occur fewer \
                 than --min-count ({}) times",
                params.min_count
            );
            if params.strict {
                return Err(CliError::Other(format!(
                    "synthesize: {msg}. Lower --min-count or drop the relationship."
                )));
            }
            wwarn!("synthesize: {msg}; columns will be generated independently");
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
        /// (distinct tuples, rows) dropped by `--min-count`.
        suppressed: (usize, u64),
    },
    /// Distinct-tuple count exceeded `--joint-cardinality-cap`.
    OverCap(usize),
    /// The source has no data rows.
    Empty,
    /// Every distinct tuple occurred fewer than `--min-count` times.
    /// (distinct tuples, rows).
    AllSuppressed(usize, u64),
}

pub(super) fn source_cell<'r>(
    record: &'r csv::ByteRecord,
    params: &ScheduleParams,
    col: usize,
) -> std::borrow::Cow<'r, str> {
    // Lossily, as `stats`/`frequency` report the column's values.
    String::from_utf8_lossy(
        params
            .source_cols
            .get(col)
            .copied()
            .flatten()
            .and_then(|src| record.get(src))
            .unwrap_or_default(),
    )
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
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let tuple: Vec<String> = col_indices
            .iter()
            .map(|&i| source_cell(&record, params, i).to_string())
            .collect();
        *counts.entry(tuple).or_insert(0) += 1;
        if cap > 0 && counts.len() as u64 > cap {
            return Ok(JointLearn::OverCap(counts.len()));
        }
    }
    if counts.is_empty() {
        return Ok(JointLearn::Empty);
    }

    // Drop tuples seen fewer than --min-count times: a rare combination of
    // otherwise-common values can identify a single source row. Their weight is
    // redistributed over the kept tuples by the normalization below.
    let distinct = counts.len();
    let min_count = params.min_count;
    let mut dropped_rows = 0_u64;
    counts.retain(|_, c| {
        let keep = *c >= min_count;
        if !keep {
            dropped_rows += *c;
        }
        keep
    });
    if counts.is_empty() {
        return Ok(JointLearn::AllSuppressed(distinct, dropped_rows));
    }
    let suppressed = distinct - counts.len();
    if suppressed > 0 {
        log::info!(
            "synthesize: joint relationship — suppressed {suppressed} of {distinct} distinct \
             tuple(s) seen fewer than --min-count ({min_count}) times"
        );
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
    Ok(JointLearn::Ok {
        tuples,
        cumulative,
        suppressed: (suppressed, dropped_rows),
    })
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
    let decimals: Vec<Option<u32>> = member_cols
        .iter()
        .map(|&c| member_decimals(&stats[c]))
        .collect();

    // Every member, not just the anchor, needs a usable range. A member whose
    // range `--min-count` emptied must stay empty, which only independent
    // generation does; the anchor + gap chain would fill it. Followers are also
    // clamped into their own (trimmed) range, so anchor + gap can't reach a
    // follower's real extremes.
    let is_datetime = kind == OrderedKind::DateTime;
    let ranges: Vec<Option<(f64, f64)>> = member_cols
        .iter()
        .map(|&c| member_range(&stats[c], kind, is_datetime))
        .collect();
    if let Some(i) = ranges.iter().position(Option::is_none) {
        let msg = format!(
            "ordered relationship {names:?}: '{}' has no usable range",
            names[i]
        );
        if params.strict {
            return Err(CliError::Other(format!(
                "synthesize: {msg}. Lower --min-count or drop the relationship."
            )));
        }
        log::warn!("synthesize: {msg}; columns will be generated independently");
        return Ok(None);
    }
    let mut member_ranges: Vec<(f64, f64)> = ranges.into_iter().flatten().collect();
    let own_ranges = member_ranges.clone();
    // Members are non-decreasing, so no member may exceed any later member's
    // upper bound. Cap each upper bound by the ones after it; otherwise the
    // order floor (`max(prev)`) would push a follower past its trimmed range.
    // Members are emitted rounded to their grid (whole numbers for Integer,
    // source decimals for Float), so their caps are floored to that grid to
    // keep the rounded value within range and in order.
    let floor_grid = |i: usize, v: f64| floor_to(v, member_grid(kind, is_int[i], decimals[i]));
    if let Some(last) = member_ranges.len().checked_sub(1) {
        member_ranges[last].1 = floor_grid(last, member_ranges[last].1);
    }
    for i in (0..member_ranges.len().saturating_sub(1)).rev() {
        member_ranges[i].1 = floor_grid(i, member_ranges[i].1.min(member_ranges[i + 1].1));
    }
    if let Some(i) = member_ranges.iter().position(|&(lo, hi)| hi < lo) {
        let msg = format!(
            "ordered relationship {names:?}: '{}' can't stay within its trimmed range while \
             keeping the declared order",
            names[i]
        );
        if params.strict {
            return Err(CliError::Other(format!(
                "synthesize: {msg}. Lower --min-count or drop the relationship."
            )));
        }
        log::warn!("synthesize: {msg}; columns will be generated independently");
        return Ok(None);
    }

    // Anchor distribution, expressed in the numeric domain.
    let anchor_stats = &stats[member_cols[0]];
    let anchor_buckets: Vec<(f64, f64)> = match kind {
        OrderedKind::Numeric => {
            if let Some(buckets) = numeric_buckets(anchor_stats) {
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
            if let Some(buckets) = date_buckets(anchor_stats, is_datetime) {
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
    let (gap_samples, mut blank_counts) = learn_ordered_gaps(&member_cols, kind, params)?;
    let mut gap_buckets = Vec::with_capacity(gap_samples.len());
    for (i, samples) in gap_samples.into_iter().enumerate() {
        if samples.is_empty() {
            // Zero co-present rows is the extreme of too few samples; treat it
            // the same way, including the strict abort.
            let msg = format!(
                "ordered relationship {names:?}: no rows have both '{}' and '{}'",
                names[i],
                names[i + 1]
            );
            if params.strict {
                return Err(CliError::Other(format!(
                    "synthesize: {msg}. Drop the relationship."
                )));
            }
            log::warn!("synthesize: {msg}; columns will be generated independently");
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
        let n_samples = samples.len();
        let Some(buckets) = gap_buckets_from_samples(samples, params.min_count, params.quantiles)
        else {
            let msg = format!(
                "ordered relationship {names:?}: only {n_samples} row(s) have both '{}' and '{}'; \
                 trimming their gap by --min-count ({}) needs at least {}",
                names[i],
                names[i + 1],
                params.min_count,
                params.min_count.saturating_mul(2).saturating_sub(1)
            );
            if params.strict {
                return Err(CliError::Other(format!(
                    "synthesize: {msg}. Lower --min-count or drop the relationship."
                )));
            }
            log::warn!("synthesize: {msg}; columns will be generated independently");
            return Ok(None);
        };
        gap_buckets.push(buckets);
    }

    // Rows blank in every member are emitted blank outright; the rest draw
    // which members to blank from the patterns the source shows.
    let all_blank_rows = blank_counts
        .remove(vec![true; member_cols.len()].as_slice())
        .unwrap_or(0);
    let null_ratio = compute_null_ratio(all_blank_rows, params.total_rows);
    let other_rows: u64 = blank_counts.values().sum();
    let mut blank_patterns = Vec::with_capacity(blank_counts.len());
    let mut blank_cumulative = Vec::with_capacity(blank_counts.len());
    let mut seen = 0_u64;
    for (pattern, count) in blank_counts {
        seen += count;
        blank_patterns.push(pattern);
        #[allow(clippy::cast_precision_loss)]
        blank_cumulative.push(seen as f64 / other_rows as f64);
    }
    if let Some(last) = blank_cumulative.last_mut() {
        *last = 1.0;
    }
    // Each pattern caps a member only by the members it leaves filled after
    // it: a blank close date must not hold back the open dates of open cases.
    // A blank member has no range or grid of its own (it prints nothing, so
    // the generator doesn't round it): it only carries the next filled
    // member's cap back through the chain.
    let pattern_ranges: Vec<Vec<(f64, f64)>> = blank_patterns
        .iter()
        .map(|blanks| {
            let mut caps = own_ranges.clone();
            let mut later = f64::MAX;
            for i in (0..caps.len()).rev() {
                if blanks[i] {
                    caps[i] = (f64::MIN, later);
                } else {
                    let hi = caps[i].1.min(later);
                    caps[i].1 = if hi < f64::MAX { floor_grid(i, hi) } else { hi };
                    later = caps[i].1;
                }
            }
            caps
        })
        .collect();
    let date_format = ordered_date_format(&member_cols, params, kind);

    log::info!(
        "synthesize: ordered relationship on {names:?} — anchor '{}'",
        names[0]
    );
    Ok(Some(GroupGenerator::Ordered {
        member_cols,
        kind,
        is_int,
        decimals,
        date_format,
        anchor_buckets,
        gap_buckets,
        pattern_ranges,
        null_ratio,
        blank_patterns,
        blank_cumulative,
    }))
}

/// The decimals a numeric group member's values are rounded to: its source
/// `max_precision` for a Float column, `None` otherwise.
fn member_decimals(stats: &StatsRecord) -> Option<u32> {
    if stats.r#type == "Float" {
        float_decimals(stats)
    } else {
        None
    }
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
/// `member[i + 1] - member[i]` for every adjacent member pair, and how often
/// each pattern of blank members occurs. Values are parsed into the group's
/// numeric domain; rows where either member of a pair is null/unparseable
/// contribute no gap for that pair. A member is blank when its cell is empty,
/// as `stats` counts nulls.
fn learn_ordered_gaps(
    member_cols: &[usize],
    kind: OrderedKind,
    params: &ScheduleParams,
) -> CliResult<(Vec<Vec<f64>>, BTreeMap<Vec<bool>, u64>)> {
    let input = params.input_path.to_string();
    let config = Config::new(Some(&input)).delimiter(params.delimiter);
    let mut rdr = config.reader()?;

    let npairs = member_cols.len() - 1;
    let mut gaps: Vec<Vec<f64>> = vec![Vec::new(); npairs];
    let mut patterns: BTreeMap<Vec<bool>, u64> = BTreeMap::new();
    let is_date = matches!(kind, OrderedKind::Date | OrderedKind::DateTime);
    let is_datetime = kind == OrderedKind::DateTime;

    let mut record = csv::ByteRecord::new();
    let mut values: Vec<Option<f64>> = vec![None; member_cols.len()];
    let mut blanks: Vec<bool> = vec![false; member_cols.len()];
    while rdr.read_byte_record(&mut record)? {
        for ((slot, blank), &col) in values.iter_mut().zip(blanks.iter_mut()).zip(member_cols) {
            let cell = source_cell(&record, params, col);
            *blank = cell.is_empty();
            *slot = if is_date {
                #[allow(clippy::cast_precision_loss)]
                parse_source_epoch(&cell, is_datetime, params.prefer_dmy).map(|e| e as f64)
            } else {
                parse_f64(&cell)
            };
        }
        for (i, pair_gaps) in gaps.iter_mut().enumerate() {
            if let (Some(lo), Some(hi)) = (values[i], values[i + 1]) {
                pair_gaps.push(hi - lo);
            }
        }
        if let Some(count) = patterns.get_mut(blanks.as_slice()) {
            *count += 1;
        } else {
            patterns.insert(blanks.clone(), 1);
        }
    }
    Ok((gaps, patterns))
}

fn gap_buckets_from_samples(
    samples: Vec<f64>,
    min_count: u64,
    quantiles: usize,
) -> Option<Vec<(f64, f64)>> {
    let mut bounds = sample_bounds(samples, min_count, quantiles)?;
    // Clamping a sorted sequence to >= 0 keeps it sorted.
    for b in &mut bounds {
        *b = b.max(0.0);
    }
    buckets_from_bounds(&bounds)
}

/// Equal-mass bucket bounds for a sample, its ends trimmed `--min-count` values
/// in from each side, as the column marginals are; `None` with fewer than
/// `2 * min_count - 1` values.
fn sample_bounds(mut samples: Vec<f64>, min_count: u64, quantiles: usize) -> Option<Vec<f64>> {
    samples.sort_by(f64::total_cmp);
    let n = samples.len();
    let k = usize::try_from(min_count.max(1)).unwrap_or(usize::MAX);
    if n == 0 || n < k.saturating_mul(2).saturating_sub(1) {
        return None;
    }
    let (lo, hi) = (samples[k - 1], samples[n - k]);
    let fine = effective_quantiles(quantiles, n as u64, min_count);
    let buckets = fine.unwrap_or(4);
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    let at = |i: usize| {
        let idx = if fine.is_some() {
            // Nearest rank, as `stats --percentiles` cuts the column marginals:
            // ceil(n * i / buckets) - 1, zero-based.
            (n * i).div_ceil(buckets) - 1
        } else {
            (n as f64 * (i as f64 / buckets as f64)) as usize
        };
        samples[idx.min(n - 1)].clamp(lo, hi)
    };
    Some(
        std::iter::once(lo)
            .chain((1..buckets).map(at))
            .chain(std::iter::once(hi))
            .collect(),
    )
}

/// Adjacent-bound buckets, or one point bucket when every bound is equal.
fn buckets_from_bounds(bounds: &[f64]) -> Option<Vec<(f64, f64)>> {
    let (&min, &max) = (bounds.first()?, bounds.last()?);
    Some(if min < max {
        bounds
            .iter()
            .copied()
            .zip(bounds.iter().copied().skip(1))
            .collect()
    } else {
        vec![(min, min)]
    })
}

/// A member's `[lo, hi]` range in the ordered group's numeric domain (days for
/// `Date`, seconds for `DateTime`), from its (possibly trimmed) stats.
fn member_range(sr: &StatsRecord, kind: OrderedKind, is_datetime: bool) -> Option<(f64, f64)> {
    #[allow(clippy::cast_precision_loss)]
    match kind {
        OrderedKind::Numeric => {
            let b = numeric_buckets(sr)?;
            Some((b.first()?.0, b.last()?.1))
        },
        OrderedKind::Date | OrderedKind::DateTime => {
            let b = date_buckets(sr, is_datetime)?;
            Some((b.first()?.0 as f64, b.last()?.1 as f64))
        },
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
    let mut decimals = Vec::with_capacity(kept_cols.len());
    let mut null_ratios = Vec::with_capacity(kept_cols.len());
    for &col in &kept_cols {
        let sr = &stats[col];
        let Some(buckets) = numeric_buckets(sr) else {
            log::warn!(
                "synthesize: correlated relationship {names:?} dropped — column '{}' has no \
                 usable numeric range",
                sr.field
            );
            return Ok(None);
        };
        marginals.push(buckets);
        is_int.push(sr.r#type == "Integer");
        decimals.push(member_decimals(sr));
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
        decimals,
        null_ratios,
        strata: Vec::new(),
        strata_cumulative: Vec::new(),
    }))
}

/// Build a stratified `GroupGenerator::Correlated`: each source combination of
/// the `strata_cols` values is a stratum, drawn by its share of the rows, and
/// the `numeric_cols` are drawn from that stratum's own buckets, coupled by a
/// copula fitted on ranks within strata. `core` holds the positions in
/// `strata_cols` of the relationship's own categorical members; the rest came
/// from `joint` groups it took in. `None` (after a warning) when the group
/// can't be built; the caller then falls back.
fn resolve_stratified(
    numeric_cols: &[usize],
    strata_cols: &[usize],
    core: &[usize],
    params: &ScheduleParams,
) -> CliResult<Option<GroupGenerator>> {
    let stats = params.stats_records;
    let names: Vec<&str> = numeric_cols
        .iter()
        .chain(strata_cols)
        .map(|&i| stats[i].field.as_str())
        .collect();
    let strata_names: Vec<String> = strata_cols
        .iter()
        .map(|&i| stats[i].field.clone())
        .collect();
    let fail = |msg: String, fix: &str| -> CliResult<Option<GroupGenerator>> {
        if params.strict {
            return Err(CliError::Other(format!("synthesize: {msg}. {fix}")));
        }
        wwarn!(
            "synthesize: {msg}; its categorical members will be generated without the numeric ones"
        );
        Ok(None)
    };

    // --- One read: each row's stratum and numeric values -------------------
    let input = params.input_path.to_string();
    let config = Config::new(Some(&input)).delimiter(params.delimiter);
    let mut rdr = config.reader()?;
    let cap = params.joint_cardinality_cap;
    let mut ids: IndexMap<Vec<String>, u32> = IndexMap::new();
    let mut row_ids: Vec<u32> = Vec::new();
    let mut row_values: Vec<Vec<Option<f64>>> = vec![Vec::new(); numeric_cols.len()];
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let key: Vec<String> = strata_cols
            .iter()
            .map(|&i| source_cell(&record, params, i).into_owned())
            .collect();
        #[allow(clippy::cast_possible_truncation)]
        let next = ids.len() as u32;
        row_ids.push(*ids.entry(key).or_insert(next));
        if cap > 0 && ids.len() as u64 > cap {
            return fail(
                format!(
                    "correlated relationship {names:?}: {strata_names:?} hold more than \
                     --joint-cardinality-cap ({cap}) combinations"
                ),
                "Raise --joint-cardinality-cap or drop the relationship.",
            );
        }
        for (values, &col) in row_values.iter_mut().zip(numeric_cols) {
            values.push(parse_f64(&source_cell(&record, params, col)));
        }
    }
    if row_ids.is_empty() {
        log::warn!("synthesize: correlated relationship {names:?} found no source rows");
        return Ok(None);
    }

    // --- Strata seen at least --min-count times, sorted by value -----------
    // A rarer combination can identify a source row, so it is never printed,
    // as a `joint` group leaves out rare tuples.
    let mut counts = vec![0_u64; ids.len()];
    for &id in &row_ids {
        counts[id as usize] += 1;
    }
    // Ids were assigned in insertion order, so `keys[id]` is id's combination.
    let keys: Vec<Vec<String>> = ids.into_keys().collect();
    let mut kept: Vec<(Vec<String>, u32)> = (0..keys.len())
        .filter(|&id| counts[id] >= params.min_count)
        .map(|id| {
            #[allow(clippy::cast_possible_truncation)]
            (keys[id].clone(), id as u32)
        })
        .collect();
    let distinct = counts.len();
    let dropped_rows: u64 = counts.iter().filter(|&&c| c < params.min_count).sum();
    let suppressed = distinct - kept.len();
    if kept.is_empty() {
        return fail(
            format!(
                "correlated relationship {names:?}: all {distinct} combination(s) of \
                 {strata_names:?} occur fewer than --min-count ({}) times",
                params.min_count
            ),
            "Lower --min-count or drop the relationship.",
        );
    }
    kept.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    // Source id -> sorted stratum position.
    let mut position: Vec<Option<u32>> = vec![None; distinct];
    for (s, (_, id)) in kept.iter().enumerate() {
        #[allow(clippy::cast_possible_truncation)]
        {
            position[*id as usize] = Some(s as u32);
        }
    }
    let row_strata: Vec<Option<u32>> = row_ids.iter().map(|&id| position[id as usize]).collect();

    // --- Which members the strata explain ----------------------------------
    let k = numeric_cols.len();
    let threshold = params.correlation_threshold.abs();
    let eta: Vec<f64> = (0..k)
        .map(|j| {
            let (groups, values): (Vec<u32>, Vec<f64>) = row_strata
                .iter()
                .zip(&row_values[j])
                .filter_map(|(s, v)| Some(((*s)?, (*v)?)))
                .unzip();
            correlation_ratio(&groups, kept.len(), &values)
        })
        .collect();
    if eta.iter().all(|&e| e < threshold) {
        return fail(
            format!(
                "correlated relationship {names:?}: {strata_names:?} don't explain any numeric \
                 member at the correlation-ratio >= {threshold} threshold"
            ),
            "Lower --correlation-threshold or drop the relationship.",
        );
    }
    let complete: Vec<usize> = (0..row_strata.len())
        .filter(|&r| row_strata[r].is_some() && row_values.iter().all(|v| v[r].is_some()))
        .collect();
    let overall = if complete.len() >= 2 {
        spearman_matrix(
            &row_values
                .iter()
                .map(|v| complete.iter().map(|&r| v[r].unwrap_or_default()).collect())
                .collect::<Vec<Vec<f64>>>(),
        )
    } else {
        identity_matrix(k)
    };
    let keep: Vec<usize> = (0..k)
        .filter(|&j| {
            eta[j] >= threshold || (0..k).any(|i| i != j && overall[i][j].abs() >= threshold)
        })
        .collect();
    if keep.len() < k {
        let dropped: Vec<&str> = (0..k)
            .filter(|j| !keep.contains(j))
            .map(|j| names[j])
            .collect();
        let msg = format!(
            "correlated relationship {names:?}: {dropped:?} are not correlated with the group at \
             the threshold {threshold}"
        );
        if params.strict {
            return Err(CliError::Other(format!(
                "synthesize: {msg}. Lower --correlation-threshold or drop those members."
            )));
        }
        log::warn!("synthesize: {msg}; dropping them from the relationship");
    }

    // --- The copula, on ranks within strata --------------------------------
    let mut by_stratum: Vec<Vec<usize>> = vec![Vec::new(); kept.len()];
    for (r, s) in row_strata.iter().enumerate() {
        if let Some(s) = s {
            by_stratum[*s as usize].push(r);
        }
    }
    let mut within: Vec<Vec<f64>> = vec![Vec::new(); keep.len()];
    for rows in &by_stratum {
        let full: Vec<usize> = rows
            .iter()
            .copied()
            .filter(|&r| keep.iter().all(|&j| row_values[j][r].is_some()))
            .collect();
        if full.len() < 2 {
            continue;
        }
        #[allow(clippy::cast_precision_loss)]
        let n = full.len() as f64 + 1.0;
        for (col, &j) in within.iter_mut().zip(&keep) {
            let values: Vec<f64> = full
                .iter()
                .map(|&r| row_values[j][r].unwrap_or_default())
                .collect();
            col.extend(fractional_ranks(&values).iter().map(|rank| rank / n - 0.5));
        }
    }
    let corr: Vec<Vec<f64>> = if within.first().is_some_and(|c| c.len() >= 2) {
        (0..keep.len())
            .map(|a| {
                (0..keep.len())
                    .map(|b| {
                        if a == b {
                            1.0
                        } else {
                            pearson(&within[a], &within[b])
                        }
                    })
                    .collect()
            })
            .collect()
    } else {
        identity_matrix(keep.len())
    };
    let cholesky_l = cholesky_ridge(&corr);

    // --- Each stratum's buckets and blank shares ---------------------------
    // A stratum with too few values for buckets of its own uses the column's.
    let mut column_buckets = Vec::with_capacity(keep.len());
    for &j in &keep {
        let sr = &stats[numeric_cols[j]];
        let Some(buckets) = numeric_buckets(sr) else {
            log::warn!(
                "synthesize: correlated relationship {names:?} dropped — column '{}' has no \
                 usable numeric range",
                sr.field
            );
            return Ok(None);
        };
        column_buckets.push(buckets);
    }
    let buckets_of = |rows: &[usize], j: usize| {
        let present: Vec<f64> = rows.iter().filter_map(|&r| row_values[j][r]).collect();
        sample_bounds(present, params.min_count, params.quantiles)
            .and_then(|b| buckets_from_bounds(&b))
    };
    // When `joint` columns joined the strata, the declared categorical
    // members' value alone (a borough, not its city) is a coarser stratum. A
    // stratum too small for buckets of its own tries that one's first, and
    // the rows of a rare combination count toward the kept ones sharing its
    // coarser value, so each borough keeps its share and its numeric range.
    let core_key =
        |values: &[String]| -> Vec<String> { core.iter().map(|&p| values[p].clone()).collect() };
    let mut core_rows: BTreeMap<Vec<String>, Vec<usize>> = BTreeMap::new();
    let mut core_kept: BTreeMap<Vec<String>, u64> = BTreeMap::new();
    if core.len() < strata_cols.len() {
        for (values, id) in &kept {
            *core_kept.entry(core_key(values)).or_default() += counts[*id as usize];
        }
        for (r, &id) in row_ids.iter().enumerate() {
            let key = core_key(&keys[id as usize]);
            if core_kept.contains_key(&key) {
                core_rows.entry(key).or_default().push(r);
            }
        }
    }
    let core_buckets: BTreeMap<Vec<String>, Vec<Option<Vec<(f64, f64)>>>> = core_rows
        .iter()
        .map(|(key, rows)| {
            (
                key.clone(),
                keep.iter().map(|&j| buckets_of(rows, j)).collect(),
            )
        })
        .collect();
    #[allow(clippy::cast_precision_loss)]
    let weights: Vec<f64> = kept
        .iter()
        .map(|(values, id)| {
            let count = counts[*id as usize] as f64;
            let key = core_key(values);
            match (core_rows.get(&key), core_kept.get(&key)) {
                (Some(rows), Some(&kept_rows)) => count * rows.len() as f64 / kept_rows as f64,
                _ => count,
            }
        })
        .collect();
    let total: f64 = weights.iter().sum();
    let mut strata = Vec::with_capacity(kept.len());
    let mut strata_cumulative = Vec::with_capacity(kept.len());
    let mut seen = 0.0_f64;
    for (((values, _), rows), weight) in kept.into_iter().zip(&by_stratum).zip(&weights) {
        let mut marginals = Vec::with_capacity(keep.len());
        let mut null_ratios = Vec::with_capacity(keep.len());
        let coarser = core_buckets.get(&core_key(&values));
        for (pos, (fallback, &j)) in column_buckets.iter().zip(&keep).enumerate() {
            let present = rows.iter().filter(|&&r| row_values[j][r].is_some()).count();
            null_ratios.push(compute_null_ratio(
                (rows.len() - present) as u64,
                rows.len() as u64,
            ));
            marginals.push(
                buckets_of(rows, j)
                    .or_else(|| coarser.and_then(|b| b[pos].clone()))
                    .unwrap_or_else(|| fallback.clone()),
            );
        }
        seen += weight;
        strata_cumulative.push(seen / total);
        strata.push(Stratum {
            values,
            marginals,
            null_ratios,
        });
    }
    if let Some(last) = strata_cumulative.last_mut() {
        *last = 1.0;
    }

    if suppressed > 0 {
        params.suppressions.borrow_mut().push(Suppression {
            columns:  strata_names,
            distinct: suppressed,
            rows:     dropped_rows,
            class:    None,
        });
    }
    let kept_cols: Vec<usize> = keep.iter().map(|&j| numeric_cols[j]).collect();
    log::info!(
        "synthesize: correlated relationship on {names:?} — {} stratum/strata",
        strata.len()
    );
    Ok(Some(GroupGenerator::Correlated {
        is_int: kept_cols
            .iter()
            .map(|&c| stats[c].r#type == "Integer")
            .collect(),
        decimals: kept_cols
            .iter()
            .map(|&c| member_decimals(&stats[c]))
            .collect(),
        col_indices: kept_cols
            .into_iter()
            .chain(strata_cols.iter().copied())
            .collect(),
        cholesky_l,
        marginals: Vec::new(),
        null_ratios: Vec::new(),
        strata,
        strata_cumulative,
    }))
}

/// Rank-based correlation ratio (η): the square root of the share of `values`'
/// rank variance explained by `groups`, one group id (below `ngroups`) per
/// value. 0 when the values don't vary.
pub(super) fn correlation_ratio(groups: &[u32], ngroups: usize, values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    let ranks = fractional_ranks(values);
    #[allow(clippy::cast_precision_loss)]
    let mean = (values.len() as f64 + 1.0) / 2.0;
    let mut sums = vec![(0.0_f64, 0_u64); ngroups];
    let mut ss_total = 0.0_f64;
    for (&g, &rank) in groups.iter().zip(&ranks) {
        let d = rank - mean;
        ss_total = d.mul_add(d, ss_total);
        sums[g as usize].0 += rank;
        sums[g as usize].1 += 1;
    }
    if ss_total <= 0.0 {
        return 0.0;
    }
    #[allow(clippy::cast_precision_loss)]
    let ss_between: f64 = sums
        .iter()
        .filter(|(_, n)| *n > 0)
        .map(|&(sum, n)| {
            let d = sum / n as f64 - mean;
            n as f64 * d * d
        })
        .sum();
    (ss_between / ss_total).sqrt().min(1.0)
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
    let mut record = csv::ByteRecord::new();
    let mut rowvals: Vec<f64> = Vec::with_capacity(k);
    while rdr.read_byte_record(&mut record)? {
        rowvals.clear();
        for &col in member_cols {
            match parse_f64(&source_cell(&record, params, col)) {
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
pub(super) fn spearman_matrix(columns: &[Vec<f64>]) -> Vec<Vec<f64>> {
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
pub(super) fn pearson(a: &[f64], b: &[f64]) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::gap_buckets_from_samples;

    #[test]
    fn gap_buckets_trim_extreme_gaps() {
        // Gaps 1..=20 plus one 10,000 outlier; k = 5 trims to [5, 17].
        let samples: Vec<f64> = (1..=20).map(f64::from).chain([10_000.0]).collect();
        let buckets = gap_buckets_from_samples(samples, 5, 4).unwrap();
        assert_eq!(buckets.first().unwrap().0, 5.0);
        assert_eq!(buckets.last().unwrap().1, 17.0);
    }

    #[test]
    fn gap_buckets_k_one_keeps_full_range() {
        let samples: Vec<f64> = (1..=20).map(f64::from).chain([10_000.0]).collect();
        let buckets = gap_buckets_from_samples(samples, 1, 4).unwrap();
        assert_eq!(buckets.first().unwrap().0, 1.0);
        assert_eq!(buckets.last().unwrap().1, 10_000.0);
    }

    #[test]
    fn gap_buckets_too_few_samples_drop_the_relationship() {
        // 4 samples < 2k-1 = 9: no trimmed range exists; a mean of a few gaps
        // (or the one gap, with a single pair) would reproduce source gaps.
        assert_eq!(
            gap_buckets_from_samples(vec![1.0, 2.0, 3.0, 100.0], 5, 4),
            None
        );
        assert_eq!(gap_buckets_from_samples(vec![40.0], 5, 4), None);
    }

    #[test]
    fn gap_buckets_follow_quantiles() {
        let samples: Vec<f64> = (0..1000).map(f64::from).collect();
        let tenths = gap_buckets_from_samples(samples.clone(), 1, 10).unwrap();
        assert_eq!(tenths.len(), 10);
        // Nearest rank: the 10th percentile of 0..1000 is the 100th value, 99.
        assert_eq!(tenths[0], (0.0, 99.0));
        assert_eq!(tenths[9], (899.0, 999.0));
        // 1000 gaps at k=199 hold 5 buckets of 200 (> k) → capped from 100 to 5.
        assert_eq!(
            gap_buckets_from_samples(samples, 199, 100).unwrap().len(),
            5
        );

        // An exact-rank median keeps the zero-gap half as its own bucket.
        let halves: Vec<f64> = (0..100).map(|i| if i < 50 { 0.0 } else { 100.0 }).collect();
        assert_eq!(
            gap_buckets_from_samples(halves, 1, 2).unwrap(),
            vec![(0.0, 0.0), (0.0, 100.0)]
        );
        assert!(gap_buckets_from_samples(vec![40.0], 1, 4).is_some());
    }
}
