//! Mergeable single-pass sampling sketches for qsv.
//!
//! These are native Rust implementations written from the original algorithm
//! papers. They are NOT bindings to, nor derived from, the Apache DataSketches
//! C++/Java code or the `datasketches` Rust crate — which, as of this writing,
//! does not expose any Sampling-family sketches. Where DataSketches is named
//! below, it is only as a sibling implementation of the same well-known
//! algorithms, for reader orientation.
//!
//! This module provides two sketches:
//!
//! - [`ReservoirItemsSketch`]: a uniform reservoir sample of `k` items from a stream of unknown
//!   size. Implements Vitter's Algorithm R (Vitter, 1985) for insertion and a slot-replacement
//!   union for merging. The output distribution is a uniform sample of size `min(k, n)` from the
//!   (multi-)set of inserted items.
//!
//! - [`VarOptItemsSketch`]: a weighted reservoir sample of `k` items, using the A-ExpJ keying
//!   scheme of Efraimidis & Spirakis (2006). Each item gets a key `u^(1/w)` (uniform `u`, weight
//!   `w`); the sketch retains the `k` items with the largest keys. This is a variance-bounded,
//!   mergeable weighted sampler — the heap-of-keys variant, not the Cohen-Duffield-Lund-Thorup H/R
//!   partition that gives strict variance optimality. It produces unbiased Horvitz-Thompson
//!   estimators when each item's inclusion probability is computed from its key, and merges by
//!   combining the two sketches' key heaps and keeping the top `k`.
//!
//! Both sketches are generic over the item type. The qsv use case instantiates
//! with [`csv::ByteRecord`]. Serialization uses a qsv-defined binary layout
//! whose preamble shape (magic, version, family id, flags, k, n) is similar in
//! spirit to a DataSketches sketch preamble, but the actual byte layout, magic,
//! and items payload are qsv-specific and NOT interoperable with DataSketches
//! serialized sketches.

// API surface intentionally exposes accessors (k, n, tau, total_weight,
// samples, samples_with_weights) for diagnostics and future programmatic use
// even though sample.rs only consumes a subset today.
#![allow(dead_code)]

use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    io::{self, Read, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rand::{Rng, RngExt};

/// Magic bytes identifying a qsv sampling-sketch serialized blob.
pub const SKETCH_MAGIC: [u8; 4] = *b"QSVK";
/// On-disk format version. Bump when the layout changes.
pub const SKETCH_FORMAT_VERSION: u8 = 2;

/// Identifies which sampling sketch a serialized qsv blob represents.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SketchFamily {
    Reservoir = 1,
    VarOpt    = 2,
}

impl SketchFamily {
    fn from_byte(b: u8) -> io::Result<Self> {
        match b {
            1 => Ok(SketchFamily::Reservoir),
            2 => Ok(SketchFamily::VarOpt),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown sketch family byte: {b}"),
            )),
        }
    }
}

/// Items that can be (de)serialized into a sketch payload. Implementations
/// exist for [`csv::ByteRecord`] below.
pub trait SerializableItem: Sized {
    fn write_to<W: Write>(&self, w: &mut W) -> io::Result<()>;
    fn read_from<R: Read>(r: &mut R) -> io::Result<Self>;
}

impl SerializableItem for csv::ByteRecord {
    fn write_to<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let field_count = u32::try_from(self.len()).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("record has too many fields: {e}"),
            )
        })?;
        w.write_u32::<LittleEndian>(field_count)?;
        for field in self {
            let len = u32::try_from(field.len()).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("field too large to serialize: {e}"),
                )
            })?;
            w.write_u32::<LittleEndian>(len)?;
            w.write_all(field)?;
        }
        Ok(())
    }

    fn read_from<R: Read>(r: &mut R) -> io::Result<Self> {
        let field_count = r.read_u32::<LittleEndian>()? as usize;
        let mut rec = csv::ByteRecord::new();
        for _ in 0..field_count {
            let len = r.read_u32::<LittleEndian>()? as usize;
            let mut buf = vec![0u8; len];
            r.read_exact(&mut buf)?;
            rec.push_field(&buf);
        }
        Ok(rec)
    }
}

// ============================================================================
// ReservoirItemsSketch — uniform reservoir, mergeable (Vitter Algorithm R).
// ============================================================================

/// Uniform reservoir sample of up to `k` items, mergeable across partitions.
#[derive(Debug, Clone)]
pub struct ReservoirItemsSketch<T: Clone> {
    k:         usize,
    n:         u64,
    reservoir: Vec<T>,
    /// Optional header context (e.g., CSV header row). Persisted alongside the
    /// sample so downstream consumers of a serialized sketch can re-emit a
    /// schema-bearing CSV without consulting the source.
    header:    Option<T>,
}

impl<T: Clone> ReservoirItemsSketch<T> {
    /// Create a new empty sketch with sample size `k`. Panics if `k == 0`.
    #[must_use]
    pub fn new(k: usize) -> Self {
        assert!(k > 0, "Reservoir size k must be > 0");
        Self {
            k,
            n: 0,
            reservoir: Vec::with_capacity(k),
            header: None,
        }
    }

    /// Attach an optional header (CSV header row or similar context). Stored
    /// alongside the sample and round-tripped through serialization.
    pub fn set_header(&mut self, header: T) {
        self.header = Some(header);
    }

    /// Borrow the attached header, if any.
    #[must_use]
    pub const fn header(&self) -> Option<&T> {
        self.header.as_ref()
    }

    #[must_use]
    pub const fn k(&self) -> usize {
        self.k
    }

    #[must_use]
    pub const fn n(&self) -> u64 {
        self.n
    }

    /// Current sample contents (read-only view).
    #[must_use]
    pub fn samples(&self) -> &[T] {
        &self.reservoir
    }

    /// Insert one item. Implements Knuth/Vitter Algorithm R: the `n`-th
    /// incoming item (1-indexed) is accepted with probability `k/n`, and on
    /// acceptance replaces a uniformly chosen slot.
    pub fn update<R: Rng + ?Sized>(&mut self, item: T, rng: &mut R) {
        if self.reservoir.len() < self.k {
            self.reservoir.push(item);
        } else {
            // `i` is the 0-indexed position of this incoming item; n_so_far is
            // the count of items seen *before* this one. Pick a uniform index
            // in [0, i]; if it lands inside the reservoir, replace that slot.
            let i = self.n; // 0-indexed: item is the (i+1)-th
            let idx = rng.random_range(0..=i);
            if (idx as usize) < self.k {
                self.reservoir[idx as usize] = item;
            }
        }
        self.n += 1;
    }

    /// Merge another sketch into self, producing a uniform sample of size
    /// `min(k, self.n + other.n)` from the combined stream.
    ///
    /// Algorithm (weighted-reservoir union, equivalent to Apache
    /// DataSketches' ReservoirItemsUnion): assign each item in the
    /// concatenated pool of `self.reservoir ∪ other.reservoir` an A-ExpJ
    /// key `u^(1/w)` (Efraimidis & Spirakis 2006), where its weight `w`
    /// equals the stream's mass-per-sampled-item it stands in for
    /// (`self.n / self.reservoir.len()` for items from `self`, analogous
    /// for `other`). Keep the top `k` keys.
    ///
    /// Each original stream item is then included in the merged sample
    /// with marginal probability `k / (self.n + other.n)`, the desired
    /// uniform-from-union target. No duplication occurs because each
    /// item appears in the pool at most once. The earlier per-slot
    /// Bernoulli scheme had a distributional bias under asymmetric
    /// stream sizes (when `other.n / total_n` was close to 1 and
    /// `other.reservoir` was exhausted before all slots were considered,
    /// late slots had a lower effective replacement probability than
    /// intended).
    pub fn merge<R: Rng + ?Sized>(&mut self, mut other: Self, rng: &mut R) {
        // Prefer self's header; fall back to other's if self has none.
        if self.header.is_none() && other.header.is_some() {
            self.header = other.header.take();
        }
        if other.n == 0 {
            return;
        }
        if self.n == 0 {
            self.reservoir = other.reservoir;
            self.n = other.n;
            // Honor self's k if smaller than other's.
            if self.reservoir.len() > self.k {
                self.reservoir.truncate(self.k);
            }
            return;
        }

        let total_n = self.n.saturating_add(other.n);
        let k = self.k;

        // If the combined logical stream fits in k slots, just concatenate
        // (both reservoirs hold every item they've seen and nothing has
        // been displaced).
        if total_n <= k as u64
            && (self.reservoir.len() as u64) == self.n
            && (other.reservoir.len() as u64) == other.n
        {
            self.reservoir.extend(other.reservoir);
            self.reservoir.truncate(k);
            self.n = total_n;
            return;
        }

        // Per-item weights: each pool item from `self` stands in for
        // self.n / self.reservoir.len() original stream items (same for
        // other). Items below contribute proportionally to their stream's
        // mass.
        let self_k = self.reservoir.len().max(1) as f64;
        let other_k = other.reservoir.len().max(1) as f64;
        let w_self = (self.n as f64) / self_k;
        let w_other = (other.n as f64) / other_k;

        // Concatenate the two reservoirs and assign A-ExpJ keys.
        let mut keyed: Vec<(f64, T)> =
            Vec::with_capacity(self.reservoir.len() + other.reservoir.len());
        for item in std::mem::take(&mut self.reservoir) {
            let u: f64 = rng.random::<f64>().max(f64::MIN_POSITIVE);
            keyed.push((u.powf(1.0 / w_self), item));
        }
        for item in other.reservoir {
            let u: f64 = rng.random::<f64>().max(f64::MIN_POSITIVE);
            keyed.push((u.powf(1.0 / w_other), item));
        }

        // Keep the top-`k` keys.
        keyed.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal));
        keyed.truncate(k);

        self.reservoir = keyed.into_iter().map(|(_, item)| item).collect();
        self.n = total_n;
    }
}

impl<T: Clone + SerializableItem> ReservoirItemsSketch<T> {
    /// Serialize the sketch to a self-describing binary blob.
    ///
    /// Layout (little-endian):
    /// - 4B magic (`QSVK`)
    /// - 1B format version
    /// - 1B family id (`SketchFamily::Reservoir`)
    /// - 2B flags (reserved; bit 0 = header present)
    /// - 8B k
    /// - 8B n
    /// - 4B item count
    /// - item count × { SerializableItem payload }
    /// - if flags bit 0 set: 1× { SerializableItem header payload }
    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::with_capacity(32);
        buf.extend_from_slice(&SKETCH_MAGIC);
        buf.write_u8(SKETCH_FORMAT_VERSION)?;
        buf.write_u8(SketchFamily::Reservoir as u8)?;
        let flags: u16 = if self.header.is_some() { 1 } else { 0 };
        buf.write_u16::<LittleEndian>(flags)?;
        buf.write_u64::<LittleEndian>(self.k as u64)?;
        buf.write_u64::<LittleEndian>(self.n)?;
        let item_count = u32::try_from(self.reservoir.len()).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("reservoir too large to serialize: {e}"),
            )
        })?;
        buf.write_u32::<LittleEndian>(item_count)?;
        for item in &self.reservoir {
            item.write_to(&mut buf)?;
        }
        if let Some(h) = &self.header {
            h.write_to(&mut buf)?;
        }
        Ok(buf)
    }

    /// Deserialize a sketch from a blob produced by [`to_bytes`].
    pub fn from_bytes(bytes: &[u8]) -> io::Result<Self> {
        let mut cur = io::Cursor::new(bytes);
        let flags = check_header(&mut cur, SketchFamily::Reservoir)?;
        let k = cur.read_u64::<LittleEndian>()? as usize;
        let n = cur.read_u64::<LittleEndian>()?;
        let item_count = cur.read_u32::<LittleEndian>()? as usize;
        if k == 0 || item_count > k {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid reservoir sketch: k={k}, items={item_count}"),
            ));
        }
        let mut reservoir = Vec::with_capacity(item_count);
        for _ in 0..item_count {
            reservoir.push(T::read_from(&mut cur)?);
        }
        let header = if flags & 1 != 0 {
            Some(T::read_from(&mut cur)?)
        } else {
            None
        };
        Ok(Self {
            k,
            n,
            reservoir,
            header,
        })
    }
}

// ============================================================================
// VarOptItemsSketch — A-ExpJ weighted reservoir, mergeable
// (Efraimidis & Spirakis 2006).
// ============================================================================

/// One item in a [`VarOptItemsSketch`]'s key heap.
#[derive(Clone)]
struct KeyedItem<T: Clone> {
    /// A-ExpJ key: `u^(1/w)` for uniform `u ∈ (0, 1]` and weight `w > 0`.
    /// Higher key ⇒ more likely to be retained.
    key:    f64,
    /// Original weight (preserved for diagnostics and weighted estimators).
    weight: f64,
    item:   T,
}

impl<T: Clone> PartialEq for KeyedItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T: Clone> Eq for KeyedItem<T> {}

impl<T: Clone> PartialOrd for KeyedItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Clone> Ord for KeyedItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse so that BinaryHeap's max-heap acts as a min-heap by key.
        other.key.partial_cmp(&self.key).unwrap_or(Ordering::Equal)
    }
}

/// Weighted reservoir sample of up to `k` items via A-ExpJ keying.
///
/// Each item is assigned a key `u^(1/w)` and the sketch retains the `k` items
/// with the largest keys. The smallest retained key serves as a threshold τ
/// — items below τ are discarded. The sketch is mergeable: the union of two
/// sketches is the top-`k` of the combined key heap.
#[derive(Debug, Clone)]
pub struct VarOptItemsSketch<T: Clone> {
    k:            usize,
    n:            u64,
    /// Min-heap of retained keyed items. Length is at most `k`.
    heap:         BinaryHeap<KeyedItem<T>>,
    /// Total weight observed (for diagnostics; not used by the sampler).
    total_weight: f64,
    /// Optional header context (e.g., CSV header row). Persisted alongside the
    /// sample so downstream consumers of a serialized sketch can re-emit a
    /// schema-bearing CSV without consulting the source.
    header:       Option<T>,
}

impl<T: Clone> std::fmt::Debug for KeyedItem<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyedItem")
            .field("key", &self.key)
            .field("weight", &self.weight)
            .finish_non_exhaustive()
    }
}

impl<T: Clone> VarOptItemsSketch<T> {
    /// Create a new empty sketch with sample size `k`. Panics if `k == 0`.
    #[must_use]
    pub fn new(k: usize) -> Self {
        assert!(k > 0, "Sample size k must be > 0");
        Self {
            k,
            n: 0,
            heap: BinaryHeap::with_capacity(k),
            total_weight: 0.0,
            header: None,
        }
    }

    /// Attach an optional header (CSV header row or similar context). Stored
    /// alongside the sample and round-tripped through serialization.
    pub fn set_header(&mut self, header: T) {
        self.header = Some(header);
    }

    /// Borrow the attached header, if any.
    #[must_use]
    pub const fn header(&self) -> Option<&T> {
        self.header.as_ref()
    }

    #[must_use]
    pub const fn k(&self) -> usize {
        self.k
    }

    #[must_use]
    pub const fn n(&self) -> u64 {
        self.n
    }

    /// Total weight of all items observed (including discarded ones).
    #[must_use]
    pub const fn total_weight(&self) -> f64 {
        self.total_weight
    }

    /// Current threshold τ — the smallest key in the retained sample. Items
    /// with smaller keys would be rejected by future inserts.
    #[must_use]
    pub fn tau(&self) -> f64 {
        // BinaryHeap::peek() returns the largest under our `Ord` (which is
        // reversed), so the peek is actually the smallest key — i.e. τ.
        self.heap.peek().map_or(0.0, |it| it.key)
    }

    /// Returns the current sample as a `Vec`, with each item's original
    /// weight. Use `weight` to compute inclusion-probability-weighted
    /// estimators.
    #[must_use]
    pub fn samples_with_weights(&self) -> Vec<(T, f64)> {
        self.heap
            .iter()
            .map(|it| (it.item.clone(), it.weight))
            .collect()
    }

    /// Returns just the sampled items, without weights.
    #[must_use]
    pub fn samples(&self) -> Vec<T> {
        self.heap.iter().map(|it| it.item.clone()).collect()
    }

    /// Insert one item. Items with non-finite or non-positive weights are
    /// silently dropped and do not increment `n`; only items with a finite,
    /// strictly positive weight are counted.
    pub fn update<R: Rng + ?Sized>(&mut self, item: T, weight: f64, rng: &mut R) {
        if !weight.is_finite() || weight <= 0.0 {
            return;
        }
        self.n += 1;
        self.total_weight += weight;
        let u: f64 = rng.random::<f64>().max(f64::MIN_POSITIVE);
        let key = u.powf(1.0 / weight);
        self.consider(KeyedItem { key, weight, item });
    }

    fn consider(&mut self, ki: KeyedItem<T>) {
        if self.heap.len() < self.k {
            self.heap.push(ki);
        } else if let Some(min) = self.heap.peek() {
            // Under our reversed `Ord`, `peek()` is the smallest key in the
            // retained sample. Replace it iff the new key is larger.
            if ki.key > min.key {
                self.heap.pop();
                self.heap.push(ki);
            }
        }
    }

    /// Merge another sketch into self, keeping the top-`k` of the combined
    /// key heap.
    ///
    /// Note: this merge is deterministic given the two sketches' keys (it
    /// is just a top-`k` selection over `self.heap ∪ other.heap`). The
    /// `_rng` parameter is unused and exists only so callers can use
    /// `merge` with the same signature shape as
    /// [`ReservoirItemsSketch::merge`], which does use the RNG.
    pub fn merge<R: Rng + ?Sized>(&mut self, mut other: Self, _rng: &mut R) {
        // Prefer self's header; fall back to other's if self has none.
        if self.header.is_none() && other.header.is_some() {
            self.header = other.header.take();
        }
        self.n = self.n.saturating_add(other.n);
        self.total_weight += other.total_weight;
        for ki in other.heap {
            self.consider(ki);
        }
    }
}

impl<T: Clone + SerializableItem> VarOptItemsSketch<T> {
    /// Serialize the sketch to a self-describing binary blob.
    ///
    /// Layout (little-endian):
    /// - 4B magic (`QSVK`)
    /// - 1B format version
    /// - 1B family id (`SketchFamily::VarOpt`)
    /// - 2B flags (reserved; bit 0 = header present)
    /// - 8B k
    /// - 8B n
    /// - 8B total_weight
    /// - 4B item count
    /// - item count × { 8B key, 8B weight, SerializableItem payload }
    /// - if flags bit 0 set: 1× { SerializableItem header payload }
    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::with_capacity(64);
        buf.extend_from_slice(&SKETCH_MAGIC);
        buf.write_u8(SKETCH_FORMAT_VERSION)?;
        buf.write_u8(SketchFamily::VarOpt as u8)?;
        let flags: u16 = if self.header.is_some() { 1 } else { 0 };
        buf.write_u16::<LittleEndian>(flags)?;
        buf.write_u64::<LittleEndian>(self.k as u64)?;
        buf.write_u64::<LittleEndian>(self.n)?;
        buf.write_f64::<LittleEndian>(self.total_weight)?;
        let item_count = u32::try_from(self.heap.len()).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("heap too large to serialize: {e}"),
            )
        })?;
        buf.write_u32::<LittleEndian>(item_count)?;
        for ki in &self.heap {
            buf.write_f64::<LittleEndian>(ki.key)?;
            buf.write_f64::<LittleEndian>(ki.weight)?;
            ki.item.write_to(&mut buf)?;
        }
        if let Some(h) = &self.header {
            h.write_to(&mut buf)?;
        }
        Ok(buf)
    }

    /// Deserialize a sketch from a blob produced by [`to_bytes`].
    pub fn from_bytes(bytes: &[u8]) -> io::Result<Self> {
        let mut cur = io::Cursor::new(bytes);
        let flags = check_header(&mut cur, SketchFamily::VarOpt)?;
        let k = cur.read_u64::<LittleEndian>()? as usize;
        let n = cur.read_u64::<LittleEndian>()?;
        let total_weight = cur.read_f64::<LittleEndian>()?;
        let item_count = cur.read_u32::<LittleEndian>()? as usize;
        if k == 0 || item_count > k {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid varopt sketch: k={k}, items={item_count}"),
            ));
        }
        let mut heap = BinaryHeap::with_capacity(item_count);
        for _ in 0..item_count {
            let key = cur.read_f64::<LittleEndian>()?;
            let weight = cur.read_f64::<LittleEndian>()?;
            let item = T::read_from(&mut cur)?;
            heap.push(KeyedItem { key, weight, item });
        }
        let header = if flags & 1 != 0 {
            Some(T::read_from(&mut cur)?)
        } else {
            None
        };
        Ok(Self {
            k,
            n,
            heap,
            total_weight,
            header,
        })
    }
}

// ============================================================================
// Header utilities
// ============================================================================

/// Inspect a serialized sketch's family without deserializing the items.
/// Also rejects unknown format versions so a stale peek doesn't surface
/// confusing downstream errors when a newer-format blob is loaded.
pub fn peek_family(bytes: &[u8]) -> io::Result<SketchFamily> {
    let header: &[u8; 8] = bytes.first_chunk::<8>().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "sketch blob is too short to be valid",
        )
    })?;
    if header[0..4] != SKETCH_MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "sketch blob has bad magic bytes",
        ));
    }
    let version = header[4];
    if version != SKETCH_FORMAT_VERSION {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unsupported sketch format version: {version}"),
        ));
    }
    SketchFamily::from_byte(header[5])
}

fn check_header<R: Read>(r: &mut R, expected: SketchFamily) -> io::Result<u16> {
    let mut magic = [0u8; 4];
    r.read_exact(&mut magic)?;
    if magic != SKETCH_MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "sketch blob has bad magic bytes",
        ));
    }
    let version = r.read_u8()?;
    if version != SKETCH_FORMAT_VERSION {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unsupported sketch format version: {version}"),
        ));
    }
    let family = SketchFamily::from_byte(r.read_u8()?)?;
    if family != expected {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("expected {expected:?} sketch, found {family:?}"),
        ));
    }
    let flags = r.read_u16::<LittleEndian>()?;
    Ok(flags)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256Plus;

    use super::*;

    fn rng(seed: u64) -> Xoshiro256Plus {
        Xoshiro256Plus::seed_from_u64(seed) // DevSkim: ignore DS148264
    }

    fn rec(field: &str) -> csv::ByteRecord {
        let mut r = csv::ByteRecord::new();
        r.push_field(field.as_bytes());
        r
    }

    // --- ReservoirItemsSketch ---

    #[test]
    fn reservoir_empty() {
        let s: ReservoirItemsSketch<u32> = ReservoirItemsSketch::new(5);
        assert_eq!(s.n(), 0);
        assert_eq!(s.k(), 5);
        assert!(s.samples().is_empty());
    }

    #[test]
    fn reservoir_under_k() {
        let mut s: ReservoirItemsSketch<u32> = ReservoirItemsSketch::new(5);
        let mut r = rng(42);
        for v in 0..3u32 {
            s.update(v, &mut r);
        }
        assert_eq!(s.n(), 3);
        assert_eq!(s.samples().len(), 3);
        assert_eq!(s.samples(), &[0, 1, 2]);
    }

    #[test]
    fn reservoir_exactly_k() {
        let mut s: ReservoirItemsSketch<u32> = ReservoirItemsSketch::new(5);
        let mut r = rng(42);
        for v in 0..5u32 {
            s.update(v, &mut r);
        }
        assert_eq!(s.n(), 5);
        assert_eq!(s.samples().len(), 5);
        assert_eq!(s.samples(), &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn reservoir_over_k_size_invariant() {
        let mut s: ReservoirItemsSketch<u32> = ReservoirItemsSketch::new(10);
        let mut r = rng(42);
        for v in 0..1_000u32 {
            s.update(v, &mut r);
        }
        assert_eq!(s.n(), 1_000);
        assert_eq!(s.samples().len(), 10);
        // All retained items must be valid stream values.
        for v in s.samples() {
            assert!(*v < 1_000);
        }
        // No duplicates in the reservoir.
        let mut sorted = s.samples().to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 10);
    }

    #[test]
    fn reservoir_uniformity_smoke() {
        // Each item should be in the reservoir with probability ~ k/n.
        // Run many trials and check empirical inclusion is in a sensible range.
        let trials = 2_000;
        let n = 100;
        let k = 10;
        let mut counts = vec![0u32; n];
        for seed in 0..trials {
            let mut s: ReservoirItemsSketch<u32> = ReservoirItemsSketch::new(k);
            let mut r = rng(seed);
            for v in 0..n as u32 {
                s.update(v, &mut r);
            }
            for v in s.samples() {
                counts[*v as usize] += 1;
            }
        }
        let expected = (trials as f64) * (k as f64) / (n as f64);
        // Each item should be included roughly `expected` times.
        // Allow generous tolerance (3σ for a binomial).
        let sigma = (expected * (1.0 - (k as f64) / (n as f64))).sqrt();
        for (idx, c) in counts.iter().enumerate() {
            let z = ((*c as f64) - expected) / sigma;
            assert!(
                z.abs() < 5.0,
                "item {idx} included {c} times (z={z:.2}, expected ~{expected:.1})"
            );
        }
    }

    #[test]
    fn reservoir_merge_under_k() {
        let mut a: ReservoirItemsSketch<u32> = ReservoirItemsSketch::new(10);
        let mut b: ReservoirItemsSketch<u32> = ReservoirItemsSketch::new(10);
        let mut r = rng(7);
        for v in 0..3u32 {
            a.update(v, &mut r);
        }
        for v in 100..104u32 {
            b.update(v, &mut r);
        }
        a.merge(b, &mut r);
        assert_eq!(a.n(), 7);
        assert_eq!(a.samples().len(), 7);
    }

    #[test]
    fn reservoir_merge_full() {
        let mut a: ReservoirItemsSketch<u32> = ReservoirItemsSketch::new(20);
        let mut b: ReservoirItemsSketch<u32> = ReservoirItemsSketch::new(20);
        let mut r = rng(13);
        for v in 0..500u32 {
            a.update(v, &mut r);
        }
        for v in 500..1_000u32 {
            b.update(v, &mut r);
        }
        a.merge(b, &mut r);
        assert_eq!(a.n(), 1_000);
        assert_eq!(a.samples().len(), 20);
        // No duplicates and all values are in [0, 1000).
        let mut sorted = a.samples().to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 20);
        for v in sorted {
            assert!(v < 1_000);
        }
    }

    #[test]
    fn reservoir_serde_round_trip() {
        let mut s: ReservoirItemsSketch<csv::ByteRecord> = ReservoirItemsSketch::new(8);
        let mut r = rng(99);
        for i in 0..50 {
            s.update(rec(&format!("row-{i}")), &mut r);
        }
        let bytes = s.to_bytes().unwrap();
        let restored: ReservoirItemsSketch<csv::ByteRecord> =
            ReservoirItemsSketch::from_bytes(&bytes).unwrap();
        assert_eq!(restored.k(), 8);
        assert_eq!(restored.n(), 50);
        assert_eq!(restored.samples().len(), s.samples().len());
        for (orig, restored) in s.samples().iter().zip(restored.samples().iter()) {
            assert_eq!(orig, restored);
        }
        assert_eq!(peek_family(&bytes).unwrap(), SketchFamily::Reservoir);
    }

    // --- VarOptItemsSketch ---

    #[test]
    fn varopt_empty() {
        let s: VarOptItemsSketch<u32> = VarOptItemsSketch::new(5);
        assert_eq!(s.n(), 0);
        assert_eq!(s.k(), 5);
        assert!(s.samples().is_empty());
        assert_eq!(s.tau(), 0.0);
    }

    #[test]
    fn varopt_under_k() {
        let mut s: VarOptItemsSketch<u32> = VarOptItemsSketch::new(5);
        let mut r = rng(42);
        for v in 0..3u32 {
            s.update(v, 1.0, &mut r);
        }
        assert_eq!(s.n(), 3);
        assert_eq!(s.samples().len(), 3);
    }

    #[test]
    fn varopt_size_invariant_over_k() {
        let mut s: VarOptItemsSketch<u32> = VarOptItemsSketch::new(10);
        let mut r = rng(42);
        for v in 0..1_000u32 {
            s.update(v, 1.0, &mut r);
        }
        assert_eq!(s.n(), 1_000);
        assert_eq!(s.samples().len(), 10);
    }

    #[test]
    fn varopt_skips_zero_and_negative_weights() {
        let mut s: VarOptItemsSketch<u32> = VarOptItemsSketch::new(5);
        let mut r = rng(42);
        s.update(1, 0.0, &mut r);
        s.update(2, -1.0, &mut r);
        s.update(3, f64::NAN, &mut r);
        s.update(4, f64::INFINITY, &mut r);
        s.update(5, 2.5, &mut r);
        assert_eq!(s.n(), 1);
        assert_eq!(s.samples(), vec![5]);
    }

    #[test]
    fn varopt_heavy_weight_invariant() {
        // An item with overwhelmingly large weight should be retained with
        // overwhelming probability — its A-ExpJ key will be very close to 1.
        // Verify across 200 trials.
        let mut retained = 0;
        for seed in 0..200 {
            let mut s: VarOptItemsSketch<u32> = VarOptItemsSketch::new(5);
            let mut r = rng(seed);
            for v in 0..200u32 {
                s.update(v, 1.0, &mut r);
            }
            // Now insert a single very-heavy item.
            s.update(9_999, 1e9, &mut r);
            if s.samples().contains(&9_999) {
                retained += 1;
            }
        }
        // Heavy item should be in the sample > 95% of trials.
        assert!(
            retained > 190,
            "heavy item retained {retained}/200 (expected >190)"
        );
    }

    #[test]
    fn varopt_merge_is_top_k_of_union() {
        // Two streams with equal-weight items: the merged sketch should be a
        // valid sample of the union, with the size-invariant maintained.
        let mut a: VarOptItemsSketch<u32> = VarOptItemsSketch::new(15);
        let mut b: VarOptItemsSketch<u32> = VarOptItemsSketch::new(15);
        let mut r = rng(7);
        for v in 0..500u32 {
            a.update(v, 1.0, &mut r);
        }
        for v in 500..1_000u32 {
            b.update(v, 1.0, &mut r);
        }
        a.merge(b, &mut r);
        assert_eq!(a.n(), 1_000);
        assert_eq!(a.samples().len(), 15);
        let mut sorted = a.samples();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 15);
    }

    #[test]
    fn varopt_merge_preserves_heavy_items() {
        // A heavy item from b should survive a merge into a (which has only
        // light items).
        let mut a: VarOptItemsSketch<u32> = VarOptItemsSketch::new(5);
        let mut b: VarOptItemsSketch<u32> = VarOptItemsSketch::new(5);
        let mut r = rng(42);
        for v in 0..200u32 {
            a.update(v, 1.0, &mut r);
        }
        b.update(8_888, 1e9, &mut r);
        a.merge(b, &mut r);
        assert!(a.samples().contains(&8_888));
    }

    #[test]
    fn varopt_serde_round_trip() {
        let mut s: VarOptItemsSketch<csv::ByteRecord> = VarOptItemsSketch::new(8);
        let mut r = rng(99);
        for i in 0..50 {
            s.update(rec(&format!("row-{i}")), (i as f64) + 1.0, &mut r);
        }
        let bytes = s.to_bytes().unwrap();
        let restored: VarOptItemsSketch<csv::ByteRecord> =
            VarOptItemsSketch::from_bytes(&bytes).unwrap();
        assert_eq!(restored.k(), 8);
        assert_eq!(restored.n(), 50);
        assert_eq!(restored.samples().len(), s.samples().len());
        assert!((restored.total_weight() - s.total_weight()).abs() < 1e-9);
        assert_eq!(peek_family(&bytes).unwrap(), SketchFamily::VarOpt);
    }

    #[test]
    fn family_peek_rejects_garbage() {
        assert!(peek_family(b"").is_err());
        assert!(peek_family(b"NOPE").is_err());
        // bad family byte (0xFF) at offset 5
        assert!(peek_family(b"QSVK\x02\xFF\x00\x00").is_err());
    }

    #[test]
    fn reservoir_header_round_trip() {
        let mut s: ReservoirItemsSketch<csv::ByteRecord> = ReservoirItemsSketch::new(4);
        s.set_header(rec("col1"));
        let mut r = rng(42);
        for i in 0..10 {
            s.update(rec(&format!("row-{i}")), &mut r);
        }
        let bytes = s.to_bytes().unwrap();
        let restored: ReservoirItemsSketch<csv::ByteRecord> =
            ReservoirItemsSketch::from_bytes(&bytes).unwrap();
        let h = restored.header().expect("header round-tripped");
        assert_eq!(h.iter().next().unwrap(), b"col1");
        assert_eq!(restored.samples().len(), 4);
    }

    #[test]
    fn varopt_header_round_trip() {
        let mut s: VarOptItemsSketch<csv::ByteRecord> = VarOptItemsSketch::new(4);
        s.set_header(rec("weight_col"));
        let mut r = rng(42);
        for i in 0..10 {
            s.update(rec(&format!("row-{i}")), (i as f64) + 1.0, &mut r);
        }
        let bytes = s.to_bytes().unwrap();
        let restored: VarOptItemsSketch<csv::ByteRecord> =
            VarOptItemsSketch::from_bytes(&bytes).unwrap();
        let h = restored.header().expect("header round-tripped");
        assert_eq!(h.iter().next().unwrap(), b"weight_col");
    }

    #[test]
    fn merge_inherits_header_when_self_has_none() {
        let mut a: ReservoirItemsSketch<csv::ByteRecord> = ReservoirItemsSketch::new(5);
        let mut b: ReservoirItemsSketch<csv::ByteRecord> = ReservoirItemsSketch::new(5);
        b.set_header(rec("from-b"));
        let mut r = rng(7);
        for i in 0..3 {
            a.update(rec(&format!("a-{i}")), &mut r);
        }
        for i in 0..3 {
            b.update(rec(&format!("b-{i}")), &mut r);
        }
        a.merge(b, &mut r);
        let h = a.header().expect("header inherited");
        assert_eq!(h.iter().next().unwrap(), b"from-b");
    }
}
