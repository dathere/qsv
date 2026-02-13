// inspired by https://github.com/race604/dedup/blob/master/src/cache.rs
use std::path::PathBuf;

use foldhash::{HashSet, HashSetExt};
use log::debug;
use memmap2::MmapMut;
use odht::{Config, FxHashFn, HashTable, bytes_needed};
use tempfile::NamedTempFile;

/// Configuration for the external deduplication cache
#[derive(Debug, Clone)]
pub struct ExtDedupConfig {
    /// Initial capacity for the on-disk hash table
    pub odht_capacity: usize,
    /// Size of chunks for large items (must be <= 127)
    pub chunk_size:    usize,
}

impl Default for ExtDedupConfig {
    fn default() -> Self {
        Self {
            odht_capacity: 10_000_000, // 10 million initial capacity
            chunk_size:    127,
        }
    }
}

impl ExtDedupConfig {
    /// Create a new configuration with custom values
    #[allow(dead_code)]
    pub fn new(odht_capacity: usize, chunk_size: usize) -> Self {
        Self {
            odht_capacity,
            chunk_size: chunk_size.min(127), // Cap at 127 for odht compatibility
        }
    }
}

struct ExtDedupConfigImpl;

impl Config for ExtDedupConfigImpl {
    type EncodedKey = [u8; 128];
    // Max size for odht compatibility
    type EncodedValue = [u8; 1];
    type H = FxHashFn;
    type Key = [u8; 128];
    // Max size for odht compatibility
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
    config:           ExtDedupConfig,
}

impl ExtDedupCache {
    /// Create a new ExtDedupCache with default configuration.
    ///
    /// # Arguments
    /// * `memo_limit` - Maximum memory usage in bytes before spilling to disk (0 = unlimited)
    /// * `temp_dir` - Directory for temporary files (None = system temp dir)
    pub fn new(memo_limit: u64, temp_dir: Option<PathBuf>) -> Self {
        Self::with_config(memo_limit, temp_dir, ExtDedupConfig::default())
    }

    /// Create a new ExtDedupCache with custom configuration.
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
            config,
        }
    }

    fn create_mmap(&mut self) -> std::io::Result<()> {
        let temp_file = tempfile::Builder::new()
            .prefix("qsv-extdedup-")
            .suffix(".tmp")
            .tempfile_in(&self.temp_dir)?;

        // Calculate required space for the hash table
        let load_factor = 95;
        let required_bytes =
            bytes_needed::<ExtDedupConfigImpl>(self.config.odht_capacity, load_factor);

        // Ensure file is large enough
        temp_file.as_file().set_len(required_bytes as u64)?;

        let mut mmap = unsafe { MmapMut::map_mut(temp_file.as_file())? };

        // Initialize the hash table in the memory-mapped file
        HashTable::<ExtDedupConfigImpl, &mut [u8]>::init_in_place(
            &mut mmap,
            self.config.odht_capacity,
            load_factor,
        )
        .map_err(|e| std::io::Error::other(format!("Failed to initialize hash table: {e}")))?;

        self.mmap = Some(mmap);
        self.temp_file = Some(temp_file);
        self.disk_initialized = true;
        Ok(())
    }

    /// Insert an item into the cache.
    /// Returns true if the item was newly inserted, false if it already existed.
    #[inline]
    pub fn insert(&mut self, item: &str) -> bool {
        let res = self.memo.insert(item.to_owned());
        if res {
            self.memo_size += item.len() as u64;

            // Check if we need to dump to disk after adding this item
            if self.memo_size > self.memo_limit {
                self.dump_to_disk();
            } else if self.memo_size >= self.memo_limit && !self.disk_initialized {
                // Initialize disk cache when memory limit is reached
                if let Err(e) = self.create_mmap() {
                    debug!("Failed to initialize disk cache: {e}");
                }
            }

            // If disk cache is initialized, also insert there
            if self.disk_initialized {
                self.insert_on_disk(item);
            }
        }

        res
    }

    /// Check if an item exists in the cache (memory or disk).
    /// Returns true if the item is found, false otherwise.
    #[inline]
    pub fn contains(&self, item: &str) -> bool {
        if self.memo.contains(item) {
            return true;
        }

        // Work directly with the memory-mapped hash table
        if self.disk_initialized && self.mmap.is_some() {
            if let Some(mmap) = &self.mmap {
                // Create a temporary table reference to work with the mmap
                // safety: The mmap is created and initialized to hold a valid HashTable,
                // and is only accessed while it is valid and not mutably borrowed elsewhere.
                let table_result = HashTable::<ExtDedupConfigImpl, &[u8]>::from_raw_bytes(mmap);

                match table_result {
                    Ok(table) => {
                        let keys = self.item_to_keys(item);
                        keys.iter().all(|key| table.contains_key(key))
                    },
                    Err(_) => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Insert an item into the disk cache.
    /// Returns true if the operation was successful, false if it failed.
    fn insert_on_disk(&mut self, item: &str) -> bool {
        if !self.disk_initialized {
            debug!("Create new disk cache");
            match self.create_mmap() {
                Ok(()) => {
                    // The table is already initialized in the mmap
                },
                Err(e) => {
                    debug!("Failed to create memory map: {e}");
                    return false;
                },
            }
        }

        // Extract keys before mutable borrow
        let keys = self.item_to_keys(item);

        // Work directly with the memory-mapped hash table
        if let Some(mmap) = &mut self.mmap {
            // Create a temporary table reference to work with the mmap
            // safety: The mmap was created with the correct size and alignment for the hash table,
            // and is only accessed through this code path. We ensure exclusive mutable access to
            // the memory region, and the table is initialized before use. Therefore, it
            // is safe to construct a HashTable from these raw bytes.
            let table_result = HashTable::<ExtDedupConfigImpl, &mut [u8]>::from_raw_bytes(mmap);
            let mut table = match table_result {
                Ok(table) => table,
                Err(e) => {
                    debug!("Failed to validate memory-mapped hash table: {e}");
                    return false;
                },
            };

            // Insert all chunks for this item into the disk cache
            for key in keys {
                table.insert(&key, &true);
            }
            true
        } else {
            false
        }
    }

    fn item_to_keys(&self, item: &str) -> Vec<[u8; 128]> {
        item.as_bytes()
            .chunks(self.config.chunk_size)
            .enumerate()
            .map(|(i, chunk)| {
                let mut key = [0_u8; 128];
                key[127] = i as u8; // Use last byte for sequence number
                key[..chunk.len()].copy_from_slice(chunk);
                key
            })
            .collect()
    }

    /// Dump all items from memory cache to disk cache.
    /// Items that fail to insert on disk are lost (logged as debug).
    fn dump_to_disk(&mut self) {
        debug!("Memory cache is full, dump to disk");
        let keys = self.memo.drain().collect::<Vec<_>>();
        let mut successful_dumps = 0;
        let mut failed_dumps = 0;

        for key in keys {
            if self.insert_on_disk(&key) {
                successful_dumps += 1;
            } else {
                failed_dumps += 1;
                debug!("Failed to dump item to disk: {key}");
            }
        }

        debug!("Dumped {successful_dumps} items to disk, {failed_dumps} failed",);
        self.memo_size = 0;
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
        assert!(cache.insert("hello"));
        assert!(cache.insert("world"));

        assert!(cache.contains("hello"));
        assert!(cache.contains("world"));
        assert!(!cache.contains("other"));
    }

    #[test]
    fn test_limit_memory() {
        let mut cache = ExtDedupCache::new(1024, None);
        for _ in 0..100 {
            cache.insert(&rand_string(32));
        }
        assert!(cache.memo.len() < 100);
        assert!(cache.disk_initialized);
    }

    #[test]
    fn test_disk_cache_operations() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = ExtDedupCache::new(100, Some(temp_dir.path().to_path_buf()));

        // Insert items that will trigger disk cache
        let items = vec!["item1", "item2", "item3"];
        for item in &items {
            assert!(cache.insert(item));
        }

        // Verify all items are still accessible
        for item in &items {
            assert!(cache.contains(item));
        }

        // Verify non-existent items return false
        assert!(!cache.contains("nonexistent"));
    }

    #[test]
    fn test_large_string_chunking() {
        let mut cache = ExtDedupCache::new(0, None);

        // Create a string larger than CHUNK_SIZE (127)
        let large_string = "a".repeat(300);
        assert!(cache.insert(&large_string));
        assert!(cache.contains(&large_string));

        // Test with string exactly at chunk boundary
        let boundary_string = "b".repeat(127);
        assert!(cache.insert(&boundary_string));
        assert!(cache.contains(&boundary_string));
    }

    #[test]
    fn test_duplicate_inserts() {
        let mut cache = ExtDedupCache::new(0, None);

        // First insert should return true
        assert!(cache.insert("duplicate"));

        // Second insert should return false
        assert!(!cache.insert("duplicate"));

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
            cache.insert(item);
        }

        // All items should still be accessible
        for item in &items {
            assert!(cache.contains(item));
        }

        // Memory cache should be empty after dump
        assert!(cache.memo.is_empty());
        assert_eq!(cache.memo_size, 0);
    }

    #[test]
    fn test_edge_cases() {
        let mut cache = ExtDedupCache::new(0, None);

        // Test empty string
        assert!(cache.insert(""));
        assert!(cache.contains(""));

        // Test very long string (multiple chunks)
        let very_long = "x".repeat(1000);
        assert!(cache.insert(&very_long));
        assert!(cache.contains(&very_long));

        // Test unicode string
        let unicode = "Hello 世界 🌍";
        assert!(cache.insert(unicode));
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
            cache.insert(item);
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
            cache.insert(item);
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
        let config = ExtDedupConfig::new(1000, 64);
        let mut cache = ExtDedupCache::with_config(50, None, config);

        // Test that custom config works
        assert!(cache.insert("test"));
        assert!(cache.contains("test"));
    }

    fn rand_string(len: usize) -> String {
        rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
}
