use std::fs;

use filetime::{FileTime, set_file_times};

use crate::workdir::Workdir;

#[test]
fn index_outdated_count() {
    let wrk = Workdir::new("index_outdated_count");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
        ],
    );

    let md = fs::metadata(wrk.path("in.csv.idx")).unwrap();
    set_file_times(
        wrk.path("in.csv"),
        future_time(FileTime::from_last_modification_time(&md)),
        future_time(FileTime::from_last_access_time(&md)),
    )
    .unwrap();

    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");

    // count works even if index is stale
    let expected_count = 2;
    let got_count: usize = wrk.stdout(&mut cmd);
    rassert_eq!(got_count, expected_count);
}

#[test]
fn index_outdated_stats() {
    let wrk = Workdir::new("index_outdated_stats");

    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "3"],
            svec!["b", "2"],
            svec!["c", "1"],
            svec!["d", "0"],
        ],
    );

    let md = fs::metadata(wrk.path("in.csv.idx")).unwrap();
    set_file_times(
        wrk.path("in.csv"),
        future_time(FileTime::from_last_access_time(&md)),
        future_time(FileTime::from_last_modification_time(&md)),
    )
    .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(2));

    // even if the index is stale, stats should succeed
    // as the index is automatically updated
    let mut cmd = wrk.command("stats");
    cmd.args(["in.csv"]);

    wrk.assert_success(&mut cmd);
}

#[test]
fn index_outdated_index() {
    let wrk = Workdir::new("index_outdated_index");

    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
        ],
    );

    let md = fs::metadata(wrk.path("in.csv.idx")).unwrap();
    set_file_times(
        wrk.path("in.csv"),
        future_time(FileTime::from_last_access_time(&md)),
        future_time(FileTime::from_last_modification_time(&md)),
    )
    .unwrap();

    // slice should NOT fail if the index is stale
    // as stale indexes are automatically updated
    let mut cmd = wrk.command("slice");
    cmd.arg("-i").arg("2").arg("in.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn index_autoindex_threshold_reached() {
    let wrk = Workdir::new("index_autoindex_threshold_reached");

    wrk.create(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
            svec!["d", "4"],
        ],
    );

    // slice should automatically create an index
    // as the file size is greater than the QSV_AUTOINDEX_SIZE threshold
    let mut cmd = wrk.command("slice");
    cmd.env("QSV_AUTOINDEX_SIZE", "1")
        .arg("-i")
        .arg("2")
        .arg("in.csv");
    wrk.assert_success(&mut cmd);

    // index should be created
    assert!(wrk.path("in.csv.idx").exists());
}

#[test]
fn index_autoindex_threshold_not_reached() {
    let wrk = Workdir::new("index_autoindex_threshold_not_reached");

    wrk.create(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
            svec!["d", "4"],
        ],
    );

    // slice will NOT automatically create an index
    // as the file size is less than the QSV_AUTOINDEX_SIZE threshold
    let mut cmd = wrk.command("slice");
    cmd.env("QSV_AUTOINDEX_SIZE", "10000000")
        .arg("-i")
        .arg("2")
        .arg("in.csv");
    wrk.assert_success(&mut cmd);

    // index should NOT be created
    assert!(!wrk.path("in.csv.idx").exists());
}

/// A failed `qsv index` must leave the filesystem as it found it. It used to write straight to
/// the destination, so the ragged record below aborted the build AFTER `File::create` had
/// already truncated the target and the dropped `BufWriter` had flushed a partial index there.
#[test]
fn index_failure_leaves_no_index() {
    let wrk = Workdir::new("index_failure_leaves_no_index");
    wrk.create_from_string("in.csv", "a,b,c\n1,2,3\n4,5,6,7,8\n9,8,7\n");

    let mut cmd = wrk.command("index");
    cmd.arg("in.csv");
    wrk.assert_err(&mut cmd);

    assert!(
        !wrk.path("in.csv.idx").exists(),
        "a failed index build left an index behind"
    );
    // and no temp file leaked either
    let leaked: Vec<_> = fs::read_dir(wrk.path("."))
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with(".qsv-index-"))
        .collect();
    assert!(leaked.is_empty(), "temp index files leaked: {leaked:?}");
}

/// Re-indexing a file that has since become ragged must not destroy the good index that is
/// already there. `File::create` truncated it before reading a single record, so a 40-byte
/// index became a bogus 16-byte one.
#[test]
fn index_failure_preserves_existing_index() {
    let wrk = Workdir::new("index_failure_preserves_existing_index");
    wrk.create_from_string("in.csv", "a,b,c\n1,2,3\n4,5,6\n9,8,7\n");

    let mut build = wrk.command("index");
    build.arg("in.csv");
    wrk.assert_success(&mut build);
    let good_idx = fs::read(wrk.path("in.csv.idx")).unwrap();
    assert!(!good_idx.is_empty());

    // the data file becomes ragged, so re-indexing it fails
    wrk.create_from_string("in.csv", "a,b,c\n1,2,3\n4,5,6,7,8\n9,8,7\n");

    let mut rebuild = wrk.command("index");
    rebuild.arg("in.csv");
    wrk.assert_err(&mut rebuild);

    assert_eq!(
        fs::read(wrk.path("in.csv.idx")).unwrap(),
        good_idx,
        "a failed re-index replaced the existing index"
    );
}

/// A truncated index that happens to end on an 8-byte boundary opens without complaint and
/// reports a byte OFFSET as its record count. `count` believed it and returned a wrong number
/// at exit 0; it must now ignore the index and scan instead.
#[test]
fn truncated_index_is_ignored() {
    let wrk = Workdir::new("truncated_index_is_ignored");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["4", "5", "6"],
            svec!["9", "8", "7"],
        ],
    );

    // 4 records (header included) => 5 u64s => 40 bytes. Lop off the trailing count and the
    // last offset: the new trailing u64 is an OFFSET, which the reader takes as a count.
    let idx = fs::read(wrk.path("in.csv.idx")).unwrap();
    assert_eq!(idx.len(), 40);
    fs::write(wrk.path("in.csv.idx"), &idx[..24]).unwrap();

    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");
    let got: usize = wrk.stdout(&mut cmd);
    rassert_eq!(got, 3);
}

/// The issue's exact repro: a bogus index must not let a malformed CSV report a confident row
/// count. Without the index, `count` surfaces the real CSV error.
#[test]
fn bogus_index_does_not_mask_a_csv_error() {
    let wrk = Workdir::new("bogus_index_does_not_mask_a_csv_error");
    wrk.create_from_string("in.csv", "a,b,c\n1,2,3\n4,5,6,7,8\n9,8,7\n");

    // exactly what a failed pre-fix `qsv index` used to leave behind: two offsets, no count
    let bogus = [0_u64, 6_u64]
        .iter()
        .flat_map(|o| o.to_be_bytes())
        .collect::<Vec<u8>>();
    assert_eq!(bogus.len(), 16);
    fs::write(wrk.path("in.csv.idx"), &bogus).unwrap();

    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");
    wrk.assert_err(&mut cmd);
}

/// The index must keep the umask default, not the 0600 a `tempfile`-based build would carry
/// through the rename. Probed against a file this test creates the same way, so it holds under
/// any umask (roborev 4371).
#[cfg(unix)]
#[test]
fn index_keeps_umask_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let wrk = Workdir::new("index_keeps_umask_permissions");
    wrk.create_from_string("in.csv", "a,b,c\n1,2,3\n4,5,6\n9,8,7\n");

    let mut cmd = wrk.command("index");
    cmd.arg("in.csv");
    wrk.assert_success(&mut cmd);

    // oracle: what mode does an ordinary File::create get in this directory?
    let probe = wrk.path("umask-probe");
    fs::File::create(&probe).unwrap();
    let expected = fs::metadata(&probe).unwrap().permissions().mode() & 0o777;

    let got = fs::metadata(wrk.path("in.csv.idx"))
        .unwrap()
        .permissions()
        .mode()
        & 0o777;
    assert_eq!(
        got, expected,
        "index mode {got:o} != umask default {expected:o}"
    );
}

/// `RandomAccessSimple::create` writes the header's offset BEFORE it reads the first data row,
/// so a pre-fix `qsv index` on a CSV whose FIRST data row is ragged left an 8-byte index holding
/// just that offset. It satisfies the `(count + 1) * 8` length invariant with count 0, and
/// `count` read it back as a confident 0 rows at exit 0. (roborev 4823)
#[test]
fn eight_byte_index_is_ignored() {
    let wrk = Workdir::new("eight_byte_index_is_ignored");
    wrk.create_from_string("in.csv", "a,b,c\n1,2,3,4,5\n6,7,8\n");

    // exactly what the pre-fix writer left behind: the header offset, no trailing count
    fs::write(wrk.path("in.csv.idx"), 0_u64.to_be_bytes()).unwrap();

    // the CSV is ragged, so ignoring the index surfaces the real error instead of "0"
    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");
    wrk.assert_err(&mut cmd);
}

/// The same 8-byte index over a VALID CSV must also be ignored - it claims zero records for a
/// file that has two.
#[test]
fn eight_byte_index_does_not_zero_a_valid_file() {
    let wrk = Workdir::new("eight_byte_index_does_not_zero_a_valid_file");
    wrk.create_from_string("in.csv", "a,b,c\n1,2,3\n4,5,6\n");
    fs::write(wrk.path("in.csv.idx"), 0_u64.to_be_bytes()).unwrap();

    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");
    let got: usize = wrk.stdout(&mut cmd);
    rassert_eq!(got, 2);
}

/// Refusing every 8-byte index costs only the index of a zero-record CSV, where a scan is free.
/// Commands must still work on such a file rather than erroring.
#[test]
fn empty_csv_still_works_without_a_usable_index() {
    let wrk = Workdir::new("empty_csv_still_works_without_a_usable_index");
    wrk.create_from_string("in.csv", "");

    let mut build = wrk.command("index");
    build.arg("in.csv");
    wrk.assert_success(&mut build);
    // an index over zero records is exactly 8 bytes, and is deliberately not trusted
    assert_eq!(fs::metadata(wrk.path("in.csv.idx")).unwrap().len(), 8);

    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");
    let got: usize = wrk.stdout(&mut cmd);
    rassert_eq!(got, 0);
}

fn future_time(ft: FileTime) -> FileTime {
    let secs = ft.unix_seconds();
    FileTime::from_unix_time(secs + 10_000, 0)
}
