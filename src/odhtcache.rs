// inspired by https://github.com/race604/dedup/blob/master/src/cache.rs
use std::{collections::HashSet, path::PathBuf};

use log::debug;
use memmap2::MmapMut;
use odht::{Config, FxHashFn, HashTable, bytes_needed};
use tempfile::NamedTempFile;

struct ExtDedupConfig;

const ODHT_CAPACITY: usize = 10_000_000; // 10 million initial capacity
const CHUNK_SIZE: usize = 127;

impl Config for ExtDedupConfig {
    type EncodedKey = [u8; CHUNK_SIZE + 1];
    type EncodedValue = [u8; 1];
    type H = FxHashFn;
    type Key = [u8; CHUNK_SIZE + 1];
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
}

impl ExtDedupCache {
    pub fn new(memo_limit: u64, temp_dir: Option<PathBuf>) -> Self {
        Self {
            memo:             HashSet::new(),
            memo_limit:       if memo_limit == 0 {
                u64::MAX
            } else {
                memo_limit
            },
            memo_size:        0,
            temp_file:        None,
            mmap:             None,
            temp_dir:         temp_dir.unwrap_or_else(std::env::temp_dir),
            disk_initialized: false,
        }
    }

    fn create_mmap(&mut self) -> std::io::Result<()> {
        let temp_file = tempfile::Builder::new()
            .prefix("qsv-extdedup-")
            .suffix(".tmp")
            .tempfile_in(&self.temp_dir)?;

        // Calculate required space for the hash table
        let load_factor = 95;
        let required_bytes = bytes_needed::<ExtDedupConfig>(ODHT_CAPACITY, load_factor);

        // Ensure file is large enough
        temp_file.as_file().set_len(required_bytes as u64)?;

        let mut mmap = unsafe { MmapMut::map_mut(temp_file.as_file())? };

        // Initialize the hash table in the memory-mapped file
        HashTable::<ExtDedupConfig, &mut [u8]>::init_in_place(
            &mut mmap,
            ODHT_CAPACITY,
            load_factor,
        )
        .map_err(|e| std::io::Error::other(format!("Failed to initialize hash table: {e}")))?;

        self.mmap = Some(mmap);
        self.temp_file = Some(temp_file);
        self.disk_initialized = true;
        Ok(())
    }

    #[inline]
    pub fn insert(&mut self, item: &str) -> bool {
        if self.memo_size >= self.memo_limit {
            self.dump_to_disk();
        }

        let mut res = self.memo.insert(item.to_owned());
        if res {
            self.memo_size += item.len() as u64;
            if self.disk_initialized {
                res = self.insert_on_disk(item);
                // debug!("Insert on disk: {res}");
            }
        }

        res
    }

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
                let table =
                    unsafe { HashTable::<ExtDedupConfig, &[u8]>::from_raw_bytes_unchecked(mmap) };

                ExtDedupCache::item_to_keys(item).all(|key| table.contains_key(&key))
            } else {
                false
            }
        } else {
            false
        }
    }

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

        // Work directly with the memory-mapped hash table
        if let Some(mmap) = &mut self.mmap {
            // Create a temporary table reference to work with the mmap
            // safety: The mmap was created with the correct size and alignment for the hash table,
            // and is only accessed through this code path. We ensure exclusive mutable access to the
            // memory region, and the table is initialized before use. Therefore, it is safe to construct
            // a HashTable from these raw bytes.
            let mut table =
                unsafe { HashTable::<ExtDedupConfig, &mut [u8]>::from_raw_bytes_unchecked(mmap) };

            let mut res = false;
            for key in ExtDedupCache::item_to_keys(item) {
                res = table.insert(&key, &true).is_none() || res;
            }
            res
        } else {
            false
        }
    }

    fn item_to_keys(item: &str) -> impl Iterator<Item = [u8; CHUNK_SIZE + 1]> + '_ {
        item.as_bytes()
            .chunks(CHUNK_SIZE)
            .enumerate()
            .map(|(i, chunk)| {
                let mut key = [0_u8; CHUNK_SIZE + 1];
                key[CHUNK_SIZE] = i as u8;
                key[..chunk.len()].copy_from_slice(chunk);
                key
            })
    }

    fn dump_to_disk(&mut self) {
        debug!("Memory cache is full, dump to disk");
        let keys = self.memo.drain().collect::<Vec<_>>();
        for key in keys {
            self.insert_on_disk(&key);
        }
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
    use rand::{Rng, distr::Alphanumeric, rng};

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

    fn rand_string(len: usize) -> String {
        rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
}
