use std::{io, ops};

use csv_index::RandomAccessSimple;

use crate::CliResult;

/// Returns true if `idx_file` is a structurally complete `RandomAccessSimple` index.
///
/// `RandomAccessSimple::create` writes N big-endian u64 record offsets (the header row counts
/// as a record) followed by ONE trailing u64 holding N. A complete index is therefore always
/// exactly `(N + 1) * 8` bytes.
///
/// `RandomAccessSimple::open` reads ONLY those last 8 bytes and trusts them as the record
/// count - it never checks them against the file's length. So a partially-written index opens
/// without complaint and reports a byte OFFSET as its record count, and every consumer
/// (`count`, `stats`, `slice`, `sample`, ...) then returns a confident wrong answer at exit 0.
/// The mtime staleness check cannot catch it either, since a partial index is NEWER than the
/// data file. Issue #4615.
///
/// Zero-length and non-multiple-of-8 indexes already fail inside `open()`; this check covers
/// the dangerous case `open()` cannot see - a truncation that happens to land on an 8-byte
/// boundary. Note a headerless empty CSV yields a legitimate 8-byte index (`(0 + 1) * 8`), so
/// the floor is 8, not 16.
pub fn is_structurally_complete(idx_file: &mut std::fs::File) -> io::Result<bool> {
    use io::{Read, Seek};

    let len = idx_file.metadata()?.len();
    if len < 8 || len % 8 != 0 {
        return Ok(false);
    }

    idx_file.seek(io::SeekFrom::End(-8))?;
    let mut buf = [0_u8; 8];
    idx_file.read_exact(&mut buf)?;
    // `RandomAccessSimple::open` seeks from the end itself, so the cursor we leave behind does
    // not matter - but rewind anyway so the handle is in a predictable state.
    idx_file.rewind()?;

    let record_count = u64::from_be_bytes(buf);
    Ok(record_count
        .checked_add(1)
        .and_then(|n| n.checked_mul(8))
        .is_some_and(|expected| expected == len))
}

/// Indexed composes a CSV reader with a simple random access index.
pub struct Indexed<R, I> {
    csv_rdr: csv::Reader<R>,
    idx:     RandomAccessSimple<I>,
}

impl<R, I> ops::Deref for Indexed<R, I> {
    type Target = csv::Reader<R>;

    fn deref(&self) -> &csv::Reader<R> {
        &self.csv_rdr
    }
}

impl<R, I> ops::DerefMut for Indexed<R, I> {
    fn deref_mut(&mut self) -> &mut csv::Reader<R> {
        &mut self.csv_rdr
    }
}

impl<R: io::Read + io::Seek, I: io::Read + io::Seek> Indexed<R, I> {
    /// Opens an index.
    pub fn open(csv_rdr: csv::Reader<R>, idx_rdr: I) -> CliResult<Indexed<R, I>> {
        Ok(Indexed {
            csv_rdr,
            idx: RandomAccessSimple::open(idx_rdr)?,
        })
    }

    /// Return the number of records (not including the header record) in this
    /// index.
    #[inline]
    pub fn count(&self) -> u64 {
        if self.csv_rdr.has_headers() && !self.idx.is_empty() {
            self.idx.len() - 1
        } else {
            self.idx.len()
        }
    }

    /// Seek to the starting position of record `i`.
    #[inline]
    pub fn seek(&mut self, mut i: u64) -> CliResult<()> {
        if i >= self.count() {
            let msg = format!(
                "invalid record index {} (there are {} records)",
                i,
                self.count()
            );
            return fail!(io::Error::other(msg));
        }
        if self.csv_rdr.has_headers() {
            i += 1;
        }
        let pos = self.idx.get(i)?;
        self.csv_rdr.seek(pos)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::is_structurally_complete;

    /// A `RandomAccessSimple` index: `offsets` as big-endian u64s, then `count` as a trailing u64.
    fn idx_bytes(offsets: &[u64], count: u64) -> Vec<u8> {
        let mut v: Vec<u8> = offsets.iter().flat_map(|o| o.to_be_bytes()).collect();
        v.extend_from_slice(&count.to_be_bytes());
        v
    }

    fn check(bytes: &[u8]) -> bool {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("probe.idx");
        std::fs::File::create(&path)
            .unwrap()
            .write_all(bytes)
            .unwrap();
        let mut f = std::fs::File::open(&path).unwrap();
        is_structurally_complete(&mut f).unwrap()
    }

    #[test]
    fn accepts_complete_indexes() {
        // headerless empty CSV: no offsets, count 0 => a legitimate 8-byte index
        assert!(check(&idx_bytes(&[], 0)));
        // header only
        assert!(check(&idx_bytes(&[0], 1)));
        // header + 3 data rows, the 40-byte case from issue #4615
        assert!(check(&idx_bytes(&[0, 6, 12, 18], 4)));
    }

    #[test]
    fn rejects_truncated_indexes() {
        // the two bogus files observed in #4615: the trailing u64 is an OFFSET, not a count
        assert!(!check(&idx_bytes(&[0], 6))); // 16 bytes, claims 6 records
        assert!(!check(&idx_bytes(&[0, 6], 12))); // 24 bytes, claims 12 records
    }

    #[test]
    fn rejects_structurally_impossible_indexes() {
        assert!(!check(&[])); // zero length
        assert!(!check(&[0, 1, 2, 3])); // not a multiple of 8
        assert!(!check(&[0; 12])); // multiple of 4 but not of 8
    }

    #[test]
    fn saturating_count_does_not_overflow() {
        // (u64::MAX + 1) * 8 must not panic or wrap into a "valid" length
        assert!(!check(&idx_bytes(&[0], u64::MAX)));
        assert!(!check(&idx_bytes(&[0], u64::MAX / 8)));
    }
}
