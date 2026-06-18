use crate::workdir::Workdir;

// materialize the three default caches (index, stats, frequency) next to `name`
fn make_caches(wrk: &Workdir, name: &str) {
    let mut cmd = wrk.command("stats");
    cmd.arg(name);
    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("frequency");
    cmd.args(["--frequency-jsonl", name]);
    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("index");
    cmd.arg(name);
    wrk.assert_success(&mut cmd);
}

#[test]
fn clean_force_removes_default_caches() {
    let wrk = Workdir::new("clean_force_removes_default_caches");
    wrk.create(
        "data.csv",
        vec![svec!["h1", "h2"], svec!["a", "1"], svec!["b", "2"]],
    );
    make_caches(&wrk, "data.csv");

    // sanity: caches were created
    assert!(wrk.path("data.csv.idx").exists());
    assert!(wrk.path("data.stats.csv").exists());
    assert!(wrk.path("data.stats.csv.json").exists());
    assert!(wrk.path("data.stats.csv.data.jsonl").exists());
    assert!(wrk.path("data.freq.csv.data.jsonl").exists());

    let mut cmd = wrk.command("clean");
    cmd.arg("--force");
    wrk.assert_success(&mut cmd);

    // all caches removed, source preserved
    assert!(!wrk.path("data.csv.idx").exists());
    assert!(!wrk.path("data.stats.csv").exists());
    assert!(!wrk.path("data.stats.csv.json").exists());
    assert!(!wrk.path("data.stats.csv.data.jsonl").exists());
    assert!(!wrk.path("data.freq.csv.data.jsonl").exists());
    assert!(wrk.path("data.csv").exists());
}

#[test]
fn clean_default_is_dry_run() {
    let wrk = Workdir::new("clean_default_is_dry_run");
    wrk.create("data.csv", vec![svec!["h1", "h2"], svec!["a", "1"]]);
    make_caches(&wrk, "data.csv");

    let mut cmd = wrk.command("clean");
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Dry-run"), "stdout: {stdout}");

    // nothing deleted
    assert!(wrk.path("data.csv.idx").exists());
    assert!(wrk.path("data.stats.csv.json").exists());
    assert!(wrk.path("data.freq.csv.data.jsonl").exists());
}

#[test]
fn clean_dry_run_wins_over_force() {
    let wrk = Workdir::new("clean_dry_run_wins_over_force");
    wrk.create("data.csv", vec![svec!["h1", "h2"], svec!["a", "1"]]);
    make_caches(&wrk, "data.csv");

    let mut cmd = wrk.command("clean");
    cmd.args(["--dry-run", "--force"]);
    wrk.assert_success(&mut cmd);

    // --dry-run wins: nothing deleted
    assert!(wrk.path("data.csv.idx").exists());
    assert!(wrk.path("data.stats.csv.json").exists());
}

#[test]
fn clean_preserves_user_file_without_sidecar() {
    let wrk = Workdir::new("clean_preserves_user_file_without_sidecar");
    wrk.create("data.csv", vec![svec!["h1", "h2"], svec!["a", "1"]]);
    make_caches(&wrk, "data.csv");

    // a user file that merely looks like a stats cache, but has no .json sidecar
    wrk.create_from_string("report.stats.csv", "x\n1\n");

    let mut cmd = wrk.command("clean");
    cmd.arg("--force");
    wrk.assert_success(&mut cmd);

    // real cache removed, decoy preserved
    assert!(!wrk.path("data.stats.csv").exists());
    assert!(wrk.path("report.stats.csv").exists());
}

#[test]
fn clean_stale_keeps_fresh_removes_stale() {
    use filetime::{FileTime, set_file_mtime};

    let wrk = Workdir::new("clean_stale_keeps_fresh_removes_stale");
    wrk.create("data.csv", vec![svec!["h1", "h2"], svec!["a", "1"]]);
    make_caches(&wrk, "data.csv");

    // caches are fresh (newer than source) -> --stale removes nothing
    let mut cmd = wrk.command("clean");
    cmd.args(["--stale", "--force"]);
    wrk.assert_success(&mut cmd);
    assert!(wrk.path("data.csv.idx").exists());
    assert!(wrk.path("data.stats.csv.json").exists());
    assert!(wrk.path("data.freq.csv.data.jsonl").exists());

    // make the source NEWER than the caches -> they are now stale
    let future = FileTime::from_unix_time(FileTime::now().unix_seconds() + 3600, 0);
    set_file_mtime(wrk.path("data.csv"), future).unwrap();

    let mut cmd = wrk.command("clean");
    cmd.args(["--stale", "--force"]);
    wrk.assert_success(&mut cmd);

    assert!(!wrk.path("data.csv.idx").exists());
    assert!(!wrk.path("data.stats.csv.json").exists());
    assert!(!wrk.path("data.freq.csv.data.jsonl").exists());
    assert!(wrk.path("data.csv").exists());
}

// An orphaned index (source deleted) cannot be fully validated against its source,
// so clean conservatively KEEPS it rather than risk deleting an unrelated file.
#[test]
fn clean_stale_keeps_orphaned_index() {
    let wrk = Workdir::new("clean_stale_keeps_orphaned_index");
    wrk.create("data.csv", vec![svec!["h1", "h2"], svec!["a", "1"]]);

    let mut cmd = wrk.command("index");
    cmd.arg("data.csv");
    wrk.assert_success(&mut cmd);
    assert!(wrk.path("data.csv.idx").exists());

    // delete the source -> the index is now orphaned
    std::fs::remove_file(wrk.path("data.csv")).unwrap();

    let mut cmd = wrk.command("clean");
    cmd.args(["--stale", "--force"]);
    wrk.assert_success(&mut cmd);

    // orphaned index is preserved (can't validate offset bounds without the source)
    assert!(wrk.path("data.csv.idx").exists());
}

// regression: an .idx with valid first/last entries but a malformed INTERMEDIATE
// offset (out of order / out of bounds) must not pass validation and be deleted.
#[test]
fn clean_preserves_index_with_bad_intermediate_offset() {
    let wrk = Workdir::new("clean_preserves_index_with_bad_intermediate_offset");
    wrk.create_from_string("data.csv", "h1,h2\na,1\n");

    // 4 big-endian u64 entries: offsets [0, 5, 3] then trailing count 3.
    // first==0 and count==entries-1 hold, but offsets are NOT monotonic (5 > 3).
    let mut bytes = Vec::new();
    for v in [0u64, 5, 3, 3] {
        bytes.extend_from_slice(&v.to_be_bytes());
    }
    std::fs::write(wrk.path("data.csv.idx"), &bytes).unwrap();

    let mut cmd = wrk.command("clean");
    cmd.arg("--force");
    wrk.assert_success(&mut cmd);

    assert!(wrk.path("data.csv.idx").exists());
}

// regression: a frequency cache built through a symlink is stored beside the
// canonical target (metadata keeps the symlink name). clean --stale must resolve
// the source from the cache's own stem and NOT delete the fresh cache.
#[cfg(unix)]
#[test]
fn clean_stale_symlinked_frequency_input() {
    let wrk = Workdir::new("clean_stale_symlinked_frequency_input");
    std::fs::create_dir_all(wrk.path("target")).unwrap();
    wrk.create_from_string("target/data.csv", "h1,h2\na,1\n");
    std::os::unix::fs::symlink(wrk.path("target/data.csv"), wrk.path("link.csv")).unwrap();

    // build the frequency cache via the symlink; it lands beside the canonical target
    let mut cmd = wrk.command("frequency");
    cmd.args(["--frequency-jsonl", "link.csv"]);
    wrk.assert_success(&mut cmd);
    assert!(wrk.path("target/data.freq.csv.data.jsonl").exists());

    // clean --stale the target dir; the cache is fresh -> must be preserved
    let mut cmd = wrk.command("clean");
    cmd.args(["--stale", "--force"]);
    cmd.arg(wrk.path("target"));
    wrk.assert_success(&mut cmd);

    assert!(wrk.path("target/data.freq.csv.data.jsonl").exists());
}

#[test]
fn clean_recursive_directory_argument() {
    let wrk = Workdir::new("clean_recursive_directory_argument");
    wrk.create("top.csv", vec![svec!["h1", "h2"], svec!["a", "1"]]);
    std::fs::create_dir_all(wrk.path("sub")).unwrap();
    wrk.create_from_string("sub/nested.csv", "h1,h2\na,1\n");

    let mut cmd = wrk.command("index");
    cmd.arg("top.csv");
    wrk.assert_success(&mut cmd);
    let mut cmd = wrk.command("index");
    cmd.arg("sub/nested.csv");
    wrk.assert_success(&mut cmd);

    assert!(wrk.path("top.csv.idx").exists());
    assert!(wrk.path("sub/nested.csv.idx").exists());

    // non-recursive clean of "." removes only the top-level index
    let mut cmd = wrk.command("clean");
    cmd.args(["--force", "."]);
    wrk.assert_success(&mut cmd);
    assert!(!wrk.path("top.csv.idx").exists());
    assert!(wrk.path("sub/nested.csv.idx").exists());

    // recursive clean removes the nested index too
    let mut cmd = wrk.command("clean");
    cmd.args(["--recursive", "--force", "."]);
    wrk.assert_success(&mut cmd);
    assert!(!wrk.path("sub/nested.csv.idx").exists());
}

#[test]
fn clean_optin_categories_gated() {
    let wrk = Workdir::new("clean_optin_categories_gated");
    wrk.create("data.csv", vec![svec!["h1", "h2"], svec!["a", "1"]]);

    // user OUTPUTS (not caches) from schema/validate/moarstats
    wrk.create_from_string(
        "data.csv.schema.json",
        r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object"}"#,
    );
    wrk.create_from_string("data.csv.valid", "h1,h2\na,1\n");
    wrk.create_from_string("data.stats.bivariate.csv", "col1,col2,corr\nh1,h2,0.5\n");

    // default clean must NOT touch opt-in outputs
    let mut cmd = wrk.command("clean");
    cmd.arg("--force");
    wrk.assert_success(&mut cmd);
    assert!(wrk.path("data.csv.schema.json").exists());
    assert!(wrk.path("data.csv.valid").exists());
    assert!(wrk.path("data.stats.bivariate.csv").exists());

    // each opt-in flag removes only its own category
    let mut cmd = wrk.command("clean");
    cmd.args(["--schema", "--force"]);
    wrk.assert_success(&mut cmd);
    assert!(!wrk.path("data.csv.schema.json").exists());
    assert!(wrk.path("data.csv.valid").exists());
    assert!(wrk.path("data.stats.bivariate.csv").exists());

    let mut cmd = wrk.command("clean");
    cmd.args(["--validate", "--force"]);
    wrk.assert_success(&mut cmd);
    assert!(!wrk.path("data.csv.valid").exists());
    assert!(wrk.path("data.stats.bivariate.csv").exists());

    let mut cmd = wrk.command("clean");
    cmd.args(["--moarstats", "--force"]);
    wrk.assert_success(&mut cmd);
    assert!(!wrk.path("data.stats.bivariate.csv").exists());
}

// regression: a user .idx file that is not a real qsv csv-index must not be
// deleted, even when it is 8-byte aligned and has a same-named sibling.
#[test]
fn clean_preserves_non_index_idx_file() {
    let wrk = Workdir::new("clean_preserves_non_index_idx_file");
    // a sibling so the strip-".idx" source exists
    wrk.create_from_string("mydata", "not a csv index source\n");
    // 16 bytes (8-aligned) whose first u64 is nonzero -> not a csv-index
    std::fs::write(wrk.path("mydata.idx"), [1u8; 16]).unwrap();

    let mut cmd = wrk.command("clean");
    cmd.arg("--force");
    wrk.assert_success(&mut cmd);

    assert!(wrk.path("mydata.idx").exists());
}

// regression: a fresh cache created with a relative arg_input must NOT be deleted
// by `clean --stale` run from a different cwd (the source must resolve as a
// sibling of the cache, not relative to the cwd).
#[test]
fn clean_stale_relative_path_from_other_cwd() {
    let wrk = Workdir::new("clean_stale_relative_path_from_other_cwd");
    wrk.create("data.csv", vec![svec!["h1", "h2"], svec!["a", "1"]]);

    // create the frequency cache with a relative arg_input ("data.csv")
    let mut cmd = wrk.command("frequency");
    cmd.args(["--frequency-jsonl", "data.csv"]);
    wrk.assert_success(&mut cmd);
    assert!(wrk.path("data.freq.csv.data.jsonl").exists());

    // run clean --stale from a DIFFERENT directory, targeting the workdir abs path
    let subdir = wrk.path("elsewhere");
    std::fs::create_dir_all(&subdir).unwrap();
    let workdir_abs = wrk.path(".");

    let mut cmd = wrk.command("clean");
    cmd.current_dir(&subdir);
    cmd.args(["--stale", "--force"]);
    cmd.arg(&workdir_abs);
    wrk.assert_success(&mut cmd);

    // the cache is fresh, not orphaned -> must be preserved
    assert!(wrk.path("data.freq.csv.data.jsonl").exists());
}

// regression: a legacy frequency cache (no canonical_input_path in metadata)
// can't have its source resolved reliably, so clean --stale conservatively KEEPS
// it rather than risk deleting a fresh cache based on a guessed source.
#[test]
fn clean_stale_keeps_legacy_frequency_cache() {
    use filetime::{FileTime, set_file_mtime};

    let wrk = Workdir::new("clean_stale_keeps_legacy_frequency_cache");
    wrk.create("data.csv", vec![svec!["h1", "h2"], svec!["a", "1"]]);

    // a legacy cache: valid qsv metadata but WITHOUT canonical_input_path
    let meta = r#"{"arg_input":"data.csv","flag_high_card_threshold":0,"flag_high_card_pct":0,"flag_no_nulls":false,"flag_no_headers":false,"flag_delimiter":",","record_count":1,"column_count":2,"date_generated":"2020-01-01T00:00:00Z","qsv_version":"0.0.0","selection_signature":""}"#;
    wrk.create_from_string("data.freq.csv.data.jsonl", &format!("{meta}\n"));

    // make the source NEWER so a naive staleness check would delete the cache
    let future = FileTime::from_unix_time(FileTime::now().unix_seconds() + 3600, 0);
    set_file_mtime(wrk.path("data.csv"), future).unwrap();

    let mut cmd = wrk.command("clean");
    cmd.args(["--stale", "--force"]);
    wrk.assert_success(&mut cmd);

    assert!(wrk.path("data.freq.csv.data.jsonl").exists());
}

// regression: a fresh frequency cache whose source has an UPPERCASE extension
// must be resolved via canonical_input_path (not a lowercase-only stem guess) and
// kept under --stale.
#[test]
fn clean_stale_keeps_uppercase_extension_cache() {
    let wrk = Workdir::new("clean_stale_keeps_uppercase_extension_cache");
    wrk.create("data.CSV", vec![svec!["h1", "h2"], svec!["a", "1"]]);

    let mut cmd = wrk.command("frequency");
    cmd.args(["--frequency-jsonl", "data.CSV"]);
    wrk.assert_success(&mut cmd);
    assert!(wrk.path("data.freq.csv.data.jsonl").exists());

    let mut cmd = wrk.command("clean");
    cmd.args(["--stale", "--force"]);
    wrk.assert_success(&mut cmd);

    assert!(wrk.path("data.freq.csv.data.jsonl").exists());
}

// regression: when two sources share a cache stem (data.tsv built the cache while
// data.csv also exists), staleness must use the ACTUAL source (data.tsv via
// canonical_input_path), not the first same-stem sibling. Here data.csv is made
// newer than the cache while data.tsv is unchanged — a stem guess would wrongly
// delete the fresh cache.
#[test]
fn clean_stale_same_stem_uses_correct_source() {
    use filetime::{FileTime, set_file_mtime};

    let wrk = Workdir::new("clean_stale_same_stem_uses_correct_source");
    wrk.create_with_delim("data.tsv", vec![svec!["h1", "h2"], svec!["a", "1"]], b'\t');
    wrk.create("data.csv", vec![svec!["h1", "h2"], svec!["a", "1"]]);

    // build the cache from data.tsv
    let mut cmd = wrk.command("frequency");
    cmd.args(["--frequency-jsonl", "data.tsv"]);
    wrk.assert_success(&mut cmd);
    assert!(wrk.path("data.freq.csv.data.jsonl").exists());

    // make the OTHER same-stem file newer than the cache; the real source is not
    let future = FileTime::from_unix_time(FileTime::now().unix_seconds() + 3600, 0);
    set_file_mtime(wrk.path("data.csv"), future).unwrap();

    let mut cmd = wrk.command("clean");
    cmd.args(["--stale", "--force"]);
    wrk.assert_success(&mut cmd);

    // canonical_input_path points at data.tsv (unchanged) -> fresh -> kept
    assert!(wrk.path("data.freq.csv.data.jsonl").exists());
}

#[test]
fn clean_single_file_input() {
    let wrk = Workdir::new("clean_single_file_input");
    wrk.create("a.csv", vec![svec!["h1", "h2"], svec!["x", "1"]]);
    wrk.create("b.csv", vec![svec!["h1", "h2"], svec!["y", "2"]]);
    make_caches(&wrk, "a.csv");
    make_caches(&wrk, "b.csv");

    // cleaning a single source file only removes THAT file's caches
    let mut cmd = wrk.command("clean");
    cmd.args(["--force", "a.csv"]);
    wrk.assert_success(&mut cmd);

    assert!(!wrk.path("a.csv.idx").exists());
    assert!(!wrk.path("a.stats.csv.json").exists());
    assert!(wrk.path("b.csv.idx").exists());
    assert!(wrk.path("b.stats.csv.json").exists());
}
