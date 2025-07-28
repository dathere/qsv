// inspired by https://github.com/race604/dedup/blob/master/src/cache.rs
use std::{collections::HashSet, path::PathBuf};

use log::debug;
use memmap2::MmapMut;
use odht::{Config, FxHashFn, HashTableOwned, bytes_needed};
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
    memo:       HashSet<String>,
    disk:       Option<HashTableOwned<ExtDedupConfig>>,
    memo_limit: u64,
    memo_size:  u64,
    temp_file:  Option<NamedTempFile>,
    mmap:       Option<MmapMut>,
    temp_dir:   PathBuf,
}

impl ExtDedupCache {
    pub fn new(memo_limit: u64, temp_dir: Option<PathBuf>) -> Self {
        Self {
            memo:       HashSet::new(),
            disk:       None,
            memo_limit: if memo_limit == 0 {
                u64::MAX
            } else {
                memo_limit
            },
            memo_size:  0,
            temp_file:  None,
            mmap:       None,
            temp_dir:   temp_dir.unwrap_or_else(std::env::temp_dir),
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

        // Create a properly initialized table
        let table = HashTableOwned::<ExtDedupConfig>::with_capacity(ODHT_CAPACITY, load_factor);

        // Copy the initialized bytes to mmap
        let raw_bytes = table.raw_bytes();
        if mmap.len() >= raw_bytes.len() {
            mmap[..raw_bytes.len()].copy_from_slice(raw_bytes);
            mmap.flush()?;
        } else {
            return Err(std::io::Error::other("Mmap size too small for ODHT table"));
        }

        self.mmap = Some(mmap);
        self.temp_file = Some(temp_file);
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
            if self.disk.is_some() {
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

        return if let Some(ref disk) = self.disk {
            ExtDedupCache::item_to_keys(item).all(|key| disk.contains_key(&key))
        } else {
            false
        };
    }

    fn insert_on_disk(&mut self, item: &str) -> bool {
        if self.disk.is_none() {
            debug!("Create new disk cache");
            match self.create_mmap() {
                Ok(()) => {
                    if let Some(mmap) = &mut self.mmap {
                        // Create the table from the properly initialized mmap
                        self.disk = Some(unsafe {
                            HashTableOwned::<ExtDedupConfig>::from_raw_bytes_unchecked(mmap)
                        });
                    }
                },
                Err(e) => {
                    debug!("Failed to create memory map: {e}");
                    // Fallback to regular HashTableOwned if mmap fails
                    self.disk = Some(HashTableOwned::<ExtDedupConfig>::with_capacity(
                        1_000_000, 95,
                    ));
                },
            }
        }

        let mut res = false;
        if let Some(disk) = &mut self.disk {
            for key in ExtDedupCache::item_to_keys(item) {
                res = disk.insert(&key, &true).is_none() || res;
            }
        }
        res
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
        assert!(cache.disk.is_some());
        assert!(cache.disk.as_ref().unwrap().len() > 0);
    }

    fn rand_string(len: usize) -> String {
        rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
}
