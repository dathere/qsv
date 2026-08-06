// inspired by https://github.com/race604/dedup/blob/master/src/cache.rs
use std::path::{Path, PathBuf};

use foldhash::{HashSet, HashSetExt};
use log::debug;
use memmap2::MmapMut;
use odht::{Config, HashTable, UnHashFn, bytes_needed};
use tempfile::NamedTempFile;
use xxhash_rust::xxh3::xxh3_128;

use crate::{CliError, CliResult};

/// Load factor of the on-disk hash table, in percent.
const LOAD_FACTOR: u8 = 95;

/// Default initial capacity, in items, of the on-disk hash table.
/// The table doubles whenever it fills, so this only needs to be large enough to
/// avoid churn on small spills - 1 million items is a ~38 MB sparse temp file.
const DEFAULT_ODHT_CAPACITY: usize = 1_000_000;

/// Configuration for the external deduplication cache
#[derive(Debug, Clone)]
pub struct ExtDedupConfig {
    /// Initial capacity, in items, of the on-disk hash table.
    /// This is a starting point, not a ceiling - the table grows by doubling.
    pub odht_capacity: usize,
}

impl Default for ExtDedupConfig {
    fn default() -> Self {
        Self {
            odht_capacity: DEFAULT_ODHT_CAPACITY,
        }
    }
}

impl ExtDedupConfig {
    /// Create a new configuration with a custom initial on-disk capacity.
    #[allow(dead_code)]
    pub const fn new(odht_capacity: usize) -> Self {
        Self { odht_capacity }
    }
}

/// odht table configuration. Keys are 128-bit xxh3 digests of the item - ONE key
/// per item, regardless of how long the item is.
///
/// The previous scheme split each item into 127-byte chunks and stored one key
/// per chunk. That was unbounded in keys-per-item (which is how the fixed-size
/// table filled up and tripped odht's insert assert) and, worse, it was wrong:
/// membership was tested as "every chunk is present", so a row assembled from
/// one row's head and another row's tail was reported as a duplicate and
/// silently dropped from the output.
///
/// Hashing makes dedup probabilistic - two distinct items collide with
/// probability ~n^2/2^129, about 1.5e-21 at a billion rows. That is strictly
/// more exact than the scheme it replaces, which had real, reachable false
/// positives.
struct ExtDedupConfigImpl;

impl Config for ExtDedupConfigImpl {
    type EncodedKey = [u8; 16];
    type EncodedValue = [u8; 1];
    // The key is already a uniformly distributed digest, so odht only needs to
    // slice 4 bytes out of it rather than re-mix it with FxHash. odht derives the
    // bucket index from the low bits and the control byte from the high bits of
    // that u32, both of which are uniform over an xxh3 digest.
    type H = UnHashFn;
    type Key = [u8; 16];
    type Value = bool;

    #[inline]
    fn encode_key(k: &Self::Key) -> Self::EncodedKey {
        *k
    }

    #[inline]
    fn encode_value(v: &Self::Value) -> Self::EncodedValue {
        [*v as u8; 1]
    }

    #[inline]
    fn decode_key(k: &Self::EncodedKey) -> Self::Key {
        *k
    }

    #[inline]
    fn decode_value(v: &Self::EncodedValue) -> Self::Value {
        v[0] == 1
    }
}

pub struct ExtDedupCache {
    memo:             HashSet<String>,
    memo_limit:       u64,
    memo_size:        u64,
    temp_file:        Option<NamedTempFile>,
    mmap:             Option<MmapMut>,
    temp_dir:         PathBuf,
    disk_initialized: bool,
    /// Item capacity of the *current* on-disk table. Doubles on each growth, so
    /// it cannot live in the immutable `config`.
    disk_capacity:    usize,
    config:           ExtDedupConfig,
}

impl ExtDedupCache {
    /// Create a new `ExtDedupCache` with default configuration.
    ///
    /// # Arguments
    /// * `memo_limit` - Maximum memory usage in bytes before spilling to disk (0 = unlimited)
    /// * `temp_dir` - Directory for temporary files (None = system temp dir)
    pub fn new(memo_limit: u64, temp_dir: Option<PathBuf>) -> Self {
        Self::with_config(memo_limit, temp_dir, ExtDedupConfig::default())
    }

    /// Create a new `ExtDedupCache` with custom configuration.
    ///
    /// # Arguments
    /// * `memo_limit` - Maximum memory usage in bytes before spilling to disk (0 = unlimited)
    /// * `temp_dir` - Directory for temporary files (None = system temp dir)
    /// * `config` - Configuration for the cache
    pub fn with_config(memo_limit: u64, temp_dir: Option<PathBuf>, config: ExtDedupConfig) -> Self {
        Self {
            memo: HashSet::new(),
            memo_limit: if memo_limit == 0 {
                u64::MAX
            } else {
                memo_limit
            },
            memo_size: 0,
            temp_file: None,
            mmap: None,
            temp_dir: temp_dir.unwrap_or_else(std::env::temp_dir),
            disk_initialized: false,
            disk_capacity: 0,
            config,
        }
    }

    /// The on-disk key for an item: a single 128-bit digest, whatever the length.
    #[inline]
    fn item_key(item: &str) -> [u8; 16] {
        xxh3_128(item.as_bytes()).to_le_bytes()
    }

    /// Allocate and initialize a fresh on-disk table with room for `capacity` items,
    /// returning the backing temp file and its memory map.
    ///
    /// This is the ONE validated construction of the table. Every later access uses
    /// `from_raw_bytes_unchecked` and relies on `init_in_place` here having written
    /// a well-formed header.
    fn alloc_table(temp_dir: &Path, capacity: usize) -> CliResult<(NamedTempFile, MmapMut)> {
        let temp_file = tempfile::Builder::new()
            .prefix("qsv-extdedup-")
            .suffix(".tmp")
            .tempfile_in(temp_dir)?;

        let required_bytes = bytes_needed::<ExtDedupConfigImpl>(capacity, LOAD_FACTOR);
        temp_file.as_file().set_len(required_bytes as u64)?;

        // safety: the file was just sized to exactly `required_bytes`, and it is a
        // private temp file that nothing outside this cache holds a handle to.
        let mut mmap = unsafe { MmapMut::map_mut(temp_file.as_file())? };

        HashTable::<ExtDedupConfigImpl, &mut [u8]>::init_in_place(&mut mmap, capacity, LOAD_FACTOR)
            .map_err(|e| {
                CliError::Other(format!("Failed to initialize on-disk hash table: {e}"))
            })?;

        Ok((temp_file, mmap))
    }

    /// Create the on-disk table with room for `capacity` items.
    ///
    /// `capacity` is a parameter rather than read from `config` because the sole
    /// caller - the spill path - knows how many items are about to land and sizes
    /// the table from that.
    fn create_mmap(&mut self, capacity: usize) -> CliResult<()> {
        let capacity = capacity.max(1);
        let (temp_file, mmap) = Self::alloc_table(&self.temp_dir, capacity)?;

        self.mmap = Some(mmap);
        self.temp_file = Some(temp_file);
        self.disk_capacity = capacity;
        self.disk_initialized = true;
        Ok(())
    }

    /// Borrow the on-disk table for reading.
    ///
    /// safety: `alloc_table` initialized the map in place with `init_in_place`, and
    /// nothing else ever writes to it. Skipping `from_raw_bytes`' revalidation
    /// avoids re-running a header check plus 10 hash verifications on every row.
    #[inline]
    fn disk_table(&self) -> Option<HashTable<ExtDedupConfigImpl, &[u8]>> {
        let mmap = self.mmap.as_ref()?;
        Some(unsafe { HashTable::<ExtDedupConfigImpl, &[u8]>::from_raw_bytes_unchecked(&mmap[..]) })
    }

    /// Borrow the on-disk table for writing. See [`Self::disk_table`] for safety.
    #[inline]
    fn disk_table_mut(&mut self) -> Option<HashTable<ExtDedupConfigImpl, &mut [u8]>> {
        let mmap = self.mmap.as_mut()?;
        Some(unsafe {
            HashTable::<ExtDedupConfigImpl, &mut [u8]>::from_raw_bytes_unchecked(&mut mmap[..])
        })
    }

    /// Number of items in the on-disk table (0 when it has not been created yet).
    #[inline]
    fn disk_len(&self) -> usize {
        self.disk_table().map_or(0, |table| table.len())
    }

    /// Insert an item into the cache.
    ///
    /// Returns `Ok(true)` if the item was newly inserted, `Ok(false)` if it was
    /// already present in either tier.
    ///
    /// Any failure to reach the on-disk tier is a hard error. A dedup cache that
    /// silently forgets an item re-emits that item's duplicates into the output
    /// with a zero exit code, which is worse than failing outright.
    #[inline]
    pub fn insert(&mut self, item: &str) -> CliResult<bool> {
        // Membership must consult both tiers: once spilled, an item lives only on
        // disk and a memo-only check would treat it as new.
        if self.memo.contains(item) || self.contains_on_disk(item) {
            return Ok(false);
        }

        if self.disk_initialized {
            // Post-spill, disk is the system of record. Write there FIRST, then keep
            // a copy in memo purely as a read cache, so no item is ever memo-only
            // and the spill path can never lose one.
            self.insert_on_disk(item)?;
            if self.memo_size >= self.memo_limit {
                // everything in memo is already durable on disk, so this is a plain
                // eviction - nothing needs to be written out
                self.memo.clear();
                self.memo_size = 0;
            }
        } else if self.memo_size + item.len() as u64 > self.memo_limit {
            // adding this item would exceed the budget - spill what we have, then
            // record this item on disk too
            self.spill_to_disk()?;
            self.insert_on_disk(item)?;
        }

        self.memo.insert(item.to_owned());
        self.memo_size += item.len() as u64;
        Ok(true)
    }

    /// Check if an item exists in the cache (memory or disk).
    /// Returns true if the item is found, false otherwise.
    ///
    /// `extdedup` itself uses [`Self::insert`]'s return value (which consults both
    /// memo and disk) to fold the contains-then-insert pattern into a single call.
    /// This method is currently unused by the bin targets but kept as part of the
    /// cache's public API for read-only callers.
    #[inline]
    #[allow(dead_code)]
    pub fn contains(&self, item: &str) -> bool {
        self.memo.contains(item) || self.contains_on_disk(item)
    }

    /// Check if an item exists in the on-disk hash table.
    /// Returns false when the disk cache has not been initialized yet.
    #[inline]
    fn contains_on_disk(&self, item: &str) -> bool {
        if !self.disk_initialized {
            return false;
        }
        let key = Self::item_key(item);
        self.disk_table()
            .is_some_and(|table| table.contains_key(&key))
    }

    /// Insert an item into the on-disk table, growing the table first if it is full.
    fn insert_on_disk(&mut self, item: &str) -> CliResult<()> {
        if !self.disk_initialized {
            // `spill_to_disk` is the only thing that brings the disk tier online,
            // and every caller reaches here through it. Erroring rather than
            // lazily creating the table keeps a single sizing rule: creating it
            // here would size it from `config` and ignore what is in `memo`.
            return fail_clierror!("on-disk dedup cache used before it was created");
        }

        // odht's mmap-backed table cannot grow itself - `HashTable::insert` fires a
        // raw assert when full. `disk_capacity` is a conservative ceiling: odht
        // rounds its slot count up to a power of two, so the table's true maximum is
        // always >= disk_capacity. Checking against it keeps us clear of the assert
        // without replicating odht's private capacity math.
        if self.disk_len() >= self.disk_capacity {
            self.grow_disk_table()?;
        }

        let key = Self::item_key(item);
        let Some(mut table) = self.disk_table_mut() else {
            return fail_clierror!("on-disk dedup cache is not mapped");
        };
        table.insert(&key, &true);
        Ok(())
    }

    /// Double the on-disk table's capacity, rehashing the existing entries into it.
    ///
    /// Growing by rehashing (rather than chaining a second table) keeps every lookup
    /// at a single probe. Dedup is miss-dominated - every *unique* row is a miss - so
    /// a chain of N tables would make the common case N times more expensive.
    fn grow_disk_table(&mut self) -> CliResult<()> {
        let new_capacity = self.disk_capacity.checked_mul(2).ok_or_else(|| {
            CliError::OutOfMemory("on-disk dedup cache capacity overflowed".to_string())
        })?;
        debug!(
            "Growing on-disk dedup cache: {} -> {new_capacity} items",
            self.disk_capacity
        );

        let (new_temp_file, mut new_mmap) = Self::alloc_table(&self.temp_dir, new_capacity)?;

        if let Some(old_mmap) = self.mmap.as_ref() {
            // safety: see `disk_table` - both maps were initialized by `alloc_table`.
            let old_table = unsafe {
                HashTable::<ExtDedupConfigImpl, &[u8]>::from_raw_bytes_unchecked(&old_mmap[..])
            };
            let mut new_table = unsafe {
                HashTable::<ExtDedupConfigImpl, &mut [u8]>::from_raw_bytes_unchecked(
                    &mut new_mmap[..],
                )
            };
            for (key, value) in old_table.iter() {
                new_table.insert(&key, &value);
            }
        }

        // dropping the old mmap and temp file unmaps and deletes the old table
        self.mmap = Some(new_mmap);
        self.temp_file = Some(new_temp_file);
        self.disk_capacity = new_capacity;
        Ok(())
    }

    /// Move everything buffered in memory onto disk.
    ///
    /// On failure the in-memory buffer is left untouched (including `memo_size`) so
    /// that nothing is lost: an item that left `memo` without reaching disk would
    /// look new the next time it appeared, and its duplicate would be written into
    /// the deduplicated output.
    fn spill_to_disk(&mut self) -> CliResult<()> {
        debug!("Memory buffer is full, spilling to disk");
        let items = std::mem::take(&mut self.memo);

        if !self.disk_initialized {
            // size the initial table from what we are about to put into it, so that
            // wide rows (which spill after fewer items) do not over-allocate and
            // narrow rows do not immediately have to grow
            let capacity = self.config.odht_capacity.max(items.len().saturating_mul(4));
            if let Err(e) = self.create_mmap(capacity) {
                self.memo = items;
                return Err(e);
            }
        }

        let mut spill_result = Ok(());
        for item in &items {
            if let Err(e) = self.insert_on_disk(item) {
                spill_result = Err(e);
                break;
            }
        }
        if spill_result.is_err() {
            // items already written are simply re-inserted next time; disk inserts
            // are idempotent, so restoring the whole buffer cannot lose anything
            self.memo = items;
            return spill_result;
        }

        debug!("Spilled {} items to disk", items.len());
        self.memo_size = 0;
        Ok(())
    }
}

impl Drop for ExtDedupCache {
    fn drop(&mut self) {
        // Explicitly drop mmap first
        self.mmap.take();
        // temp_file will be automatically deleted when dropped
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use rand::{RngExt, distr::Alphanumeric, rng};
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_basic_cache() {
        let mut cache = ExtDedupCache::new(0, None);
        assert!(cache.insert("hello").unwrap());
        assert!(cache.insert("world").unwrap());

        assert!(cache.contains("hello"));
        assert!(cache.contains("world"));
        assert!(!cache.contains("other"));
    }

    #[test]
    fn test_limit_memory() {
        const ITEM_LEN: usize = 32;
        const MEMO_LIMIT: u64 = 1024;

        let mut cache = ExtDedupCache::new(MEMO_LIMIT, None);
        for _ in 0..10_000 {
            cache.insert(&rand_string(ITEM_LEN)).unwrap();

            // `extdedup`'s USAGE promises constant memory, so the in-memory tier
            // must keep being evicted long after the first spill - it may only
            // overshoot the budget by the one item that trips the check.
            assert!(
                cache.memo_size <= MEMO_LIMIT + ITEM_LEN as u64,
                "memo grew past the limit: {} bytes",
                cache.memo_size
            );
        }
        assert!(cache.disk_initialized);
    }

    #[test]
    fn test_disk_cache_operations() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = ExtDedupCache::new(100, Some(temp_dir.path().to_path_buf()));

        // Insert items that will trigger disk cache
        let items = vec!["item1", "item2", "item3"];
        for item in &items {
            assert!(cache.insert(item).unwrap());
        }

        // Verify all items are still accessible
        for item in &items {
            assert!(cache.contains(item));
        }

        // Verify non-existent items return false
        assert!(!cache.contains("nonexistent"));
    }

    #[test]
    fn test_large_string_single_key() {
        let mut cache = ExtDedupCache::new(0, None);

        // Create a string larger than the old 127-byte chunk size. Items are now
        // hashed to a single key, but long items are still worth round-tripping.
        let large_string = "a".repeat(300);
        assert!(cache.insert(&large_string).unwrap());
        assert!(cache.contains(&large_string));

        // Test with string exactly at the old chunk boundary
        let boundary_string = "b".repeat(127);
        assert!(cache.insert(&boundary_string).unwrap());
        assert!(cache.contains(&boundary_string));
    }

    #[test]
    fn test_duplicate_inserts() {
        let mut cache = ExtDedupCache::new(0, None);

        // First insert should return true
        assert!(cache.insert("duplicate").unwrap());

        // Second insert should return false
        assert!(!cache.insert("duplicate").unwrap());

        // Item should still be present
        assert!(cache.contains("duplicate"));
    }

    #[test]
    fn test_memory_limit_behavior() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = ExtDedupCache::new(30, Some(temp_dir.path().to_path_buf()));

        // Insert items that exceed memory limit
        let items = vec!["short", "medium_length_item", "another_item"];
        for item in &items {
            cache.insert(item).unwrap();
        }

        // All items should still be accessible
        for item in &items {
            assert!(cache.contains(item));
        }

        // and every one of them must have reached disk - nothing may be memo-only
        assert!(cache.disk_initialized);
        assert_eq!(cache.disk_len(), items.len());
    }

    #[test]
    fn test_edge_cases() {
        let mut cache = ExtDedupCache::new(0, None);

        // Test empty string
        assert!(cache.insert("").unwrap());
        assert!(cache.contains(""));

        // Test very long string
        let very_long = "x".repeat(1000);
        assert!(cache.insert(&very_long).unwrap());
        assert!(cache.contains(&very_long));

        // Test unicode string
        let unicode = "Hello 世界 🌍";
        assert!(cache.insert(unicode).unwrap());
        assert!(cache.contains(unicode));
    }

    #[test]
    fn test_temp_dir_creation() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().to_path_buf();

        let mut cache = ExtDedupCache::new(20, Some(temp_path.clone()));

        // Force disk cache creation by exceeding memory limit
        let items = vec!["test_item1", "test_item2", "test_item3"];
        for item in &items {
            cache.insert(item).unwrap();
        }

        // Verify temp files were created in the specified directory
        let entries: Vec<_> = fs::read_dir(&temp_path).unwrap().collect();
        assert!(!entries.is_empty());
    }

    #[test]
    fn test_zero_memory_limit() {
        let cache = ExtDedupCache::new(0, None);

        // With zero limit, should use unlimited memory
        assert_eq!(cache.memo_limit, u64::MAX);

        // Should not initialize disk cache immediately
        assert!(!cache.disk_initialized);
    }

    #[test]
    fn test_disk_cache_persistence() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = ExtDedupCache::new(50, Some(temp_dir.path().to_path_buf()));

        // Insert items to trigger disk cache (need to exceed 50 byte limit)
        let items = vec![
            "persistent1",
            "persistent2",
            "persistent3",
            "persistent4",
            "persistent5",
        ];
        for item in &items {
            cache.insert(item).unwrap();
        }

        // Verify disk cache is initialized
        assert!(cache.disk_initialized);

        // All items should be accessible
        for item in &items {
            assert!(cache.contains(item));
        }
    }

    #[test]
    fn test_custom_config() {
        let config = ExtDedupConfig::new(1000);
        let mut cache = ExtDedupCache::with_config(50, None, config);

        // Test that custom config works
        assert!(cache.insert("test").unwrap());
        assert!(cache.contains("test"));
    }

    /// Regression test for the chunk-set false positive.
    ///
    /// The on-disk tier used to split each item into 127-byte chunks and report an
    /// item present when *every* chunk was present - chunk-SET membership, not item
    /// identity. Two long items could therefore synthesize a third: `head_of_a ++
    /// tail_of_b` looked like a duplicate, and the unique row was silently dropped
    /// from the output.
    #[test]
    fn test_no_false_positive_from_chunk_mixing() {
        // memo_limit=1 forces every insert straight to the on-disk tier;
        // the in-memory tier stores whole strings and was never affected.
        let temp_dir = TempDir::new().unwrap();
        let mut cache = ExtDedupCache::new(1, Some(temp_dir.path().to_path_buf()));

        let head_a = "A".repeat(127);
        let tail_b = "B".repeat(127);
        let head_c = "C".repeat(127);
        let tail_d = "D".repeat(127);

        let item_ab = format!("{head_a}{tail_b}");
        let item_cd = format!("{head_c}{tail_d}");
        // Every 127-byte chunk of this item is already on disk, but the item itself
        // has never been seen.
        let item_ad = format!("{head_a}{tail_d}");

        assert!(cache.insert(&item_ab).unwrap());
        assert!(cache.insert(&item_cd).unwrap());
        assert!(
            cache.insert(&item_ad).unwrap(),
            "item_ad is new, but its chunks were borrowed from item_ab and item_cd"
        );
    }

    /// The on-disk table used to panic inside odht once it filled up. Scaled-down
    /// version of the reproduction in issue #4355: capacity 100 rounds up to 128
    /// slots, so odht asserted somewhere past ~121 items.
    #[test]
    fn test_disk_table_grows_past_initial_capacity() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = ExtDedupCache::with_config(
            1,
            Some(temp_dir.path().to_path_buf()),
            ExtDedupConfig::new(100),
        );

        for i in 0..500 {
            assert!(cache.insert(&format!("item-{i}")).unwrap());
        }
        assert_eq!(cache.disk_len(), 500);
        assert!(cache.disk_capacity > 100, "table should have grown");
    }

    /// Growth must be lossless in both directions: no item may disappear, and no
    /// duplicate or stale entry may survive the rehash.
    #[test]
    fn test_growth_preserves_contents() {
        const N: usize = 5_000;

        let temp_dir = TempDir::new().unwrap();
        // tiny initial capacity + memo_limit=1 => everything goes to disk and the
        // table has to double several times
        let mut cache = ExtDedupCache::with_config(
            1,
            Some(temp_dir.path().to_path_buf()),
            ExtDedupConfig::new(100),
        );

        for i in 0..N {
            assert!(
                cache.insert(&format!("item-{i}")).unwrap(),
                "item-{i} should be new"
            );
        }

        // every item must still be found after the rehashes
        for i in 0..N {
            assert!(
                !cache.insert(&format!("item-{i}")).unwrap(),
                "item-{i} was lost by a rehash"
            );
        }

        assert_eq!(
            cache.disk_len(),
            N,
            "on-disk item count drifted during growth"
        );
    }

    /// Smoke test for the hash key at a realistic scale, on the no-growth path.
    ///
    /// This does NOT prove `UnHashFn` is well distributed - odht compares full keys
    /// on probe, so a poorly spread hash costs probes, not correctness. What it does
    /// catch is the accounting going wrong at volume: a key-encoding or probe bug
    /// that dropped or double-counted entries shows up as an item-count mismatch.
    #[test]
    fn test_disk_item_count_matches_distinct_items() {
        const N: usize = 100_000;

        let temp_dir = TempDir::new().unwrap();
        // the default capacity comfortably holds N, so this isolates the hash
        // function from the growth path
        let mut cache = ExtDedupCache::new(1, Some(temp_dir.path().to_path_buf()));

        for i in 0..N {
            assert!(cache.insert(&format!("distinct-item-{i}")).unwrap());
        }

        assert_eq!(cache.disk_len(), N, "hash/probe mismatch loses items");
        assert_eq!(
            cache.disk_capacity, DEFAULT_ODHT_CAPACITY,
            "should not grow"
        );
    }

    fn rand_string(len: usize) -> String {
        rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
}
