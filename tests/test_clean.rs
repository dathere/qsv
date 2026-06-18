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

#[test]
fn clean_stale_removes_orphaned_index() {
    let wrk = Workdir::new("clean_stale_removes_orphaned_index");
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

    assert!(!wrk.path("data.csv.idx").exists());
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
