use crate::workdir::Workdir;

const DATA: &str = "letter,number\nalpha,13\nbeta,24\n";

/// Build a workdir holding `in.csv` with known content.
fn setup(name: &str) -> Workdir {
    let wrk = Workdir::new(name);
    wrk.create_from_string("in.csv", DATA);
    wrk
}

/// The issue #4580 repro: `qsv select ... in.csv -o in.csv` used to exit 0 with a zeroed
/// input. `select` is used throughout this file because it is one of the 18 affected
/// commands AND is present in every binary flavor (qsv, qsvlite, qsvdp, qsvmcp).
#[test]
fn output_guard_blocks_plain_repro() {
    let wrk = setup("output_guard_blocks_plain_repro");

    let mut cmd = wrk.command("select");
    cmd.args(["letter,number", "in.csv"])
        .args(["--output", "in.csv"]);
    wrk.assert_err(&mut cmd);

    assert_eq!(
        wrk.read_to_string("in.csv").unwrap(),
        DATA,
        "the input must be left untouched"
    );
}

/// The error must name the guard, not some downstream symptom of an emptied file.
#[test]
fn output_guard_error_message() {
    let wrk = setup("output_guard_error_message");

    let mut cmd = wrk.command("select");
    cmd.args(["letter,number", "in.csv"])
        .args(["--output", "in.csv"]);
    let got = wrk.output_stderr(&mut cmd);
    assert!(
        got.contains("is the same file as an input"),
        "unexpected stderr: {got}"
    );
}

/// `./in.csv` is a different string but the same file - canonicalization-free identity.
#[test]
fn output_guard_blocks_dot_slash() {
    let wrk = setup("output_guard_blocks_dot_slash");

    let mut cmd = wrk.command("select");
    cmd.args(["letter,number", "in.csv"])
        .args(["--output", "./in.csv"]);
    wrk.assert_err(&mut cmd);

    assert_eq!(wrk.read_to_string("in.csv").unwrap(), DATA);
}

/// `--output=<file>` and the attached short form `-o<file>` must be scanned too.
#[test]
fn output_guard_blocks_attached_forms() {
    let wrk = setup("output_guard_blocks_attached_forms");

    let mut cmd = wrk.command("select");
    cmd.args(["letter,number", "in.csv"]).arg("--output=in.csv");
    wrk.assert_err(&mut cmd);
    assert_eq!(wrk.read_to_string("in.csv").unwrap(), DATA);

    let mut cmd = wrk.command("select");
    cmd.args(["letter,number", "in.csv"]).arg("-oin.csv");
    wrk.assert_err(&mut cmd);
    assert_eq!(wrk.read_to_string("in.csv").unwrap(), DATA);
}

/// A symlink has its own canonical path but the same inode - it must still be caught.
#[cfg(target_family = "unix")]
#[test]
fn output_guard_blocks_symlink() {
    let wrk = setup("output_guard_blocks_symlink");
    std::os::unix::fs::symlink(wrk.path("in.csv"), wrk.path("link.csv")).unwrap();

    let mut cmd = wrk.command("select");
    cmd.args(["letter,number", "link.csv"])
        .args(["--output", "in.csv"]);
    wrk.assert_err(&mut cmd);

    assert_eq!(wrk.read_to_string("in.csv").unwrap(), DATA);
}

/// Two hard links to one inode have two DISTINCT canonical paths, so only a file-identity
/// check catches this. This is the case that rules out a canonicalize-and-compare guard.
#[cfg(target_family = "unix")]
#[test]
fn output_guard_blocks_hard_link() {
    let wrk = setup("output_guard_blocks_hard_link");
    std::fs::hard_link(wrk.path("in.csv"), wrk.path("hard.csv")).unwrap();

    let mut cmd = wrk.command("select");
    cmd.args(["letter,number", "hard.csv"])
        .args(["--output", "in.csv"]);
    wrk.assert_err(&mut cmd);

    assert_eq!(wrk.read_to_string("in.csv").unwrap(), DATA);
}

/// Negative case: a genuinely different --output still works.
#[test]
fn output_guard_allows_different_output() {
    let wrk = setup("output_guard_allows_different_output");

    let mut cmd = wrk.command("select");
    cmd.args(["letter,number", "in.csv"])
        .args(["--output", "out.csv"]);
    wrk.assert_success(&mut cmd);

    assert_eq!(wrk.read_to_string("out.csv").unwrap(), DATA);
    assert_eq!(wrk.read_to_string("in.csv").unwrap(), DATA);
}

/// Negative case: overwriting an unrelated file that already exists is legitimate and
/// must not be mistaken for the input.
#[test]
fn output_guard_allows_overwriting_unrelated_file() {
    let wrk = setup("output_guard_allows_overwriting_unrelated_file");
    wrk.create_from_string("other.csv", "stale\n1\n");

    let mut cmd = wrk.command("select");
    cmd.args(["letter,number", "in.csv"])
        .args(["--output", "other.csv"]);
    wrk.assert_success(&mut cmd);

    assert_eq!(wrk.read_to_string("other.csv").unwrap(), DATA);
}

/// The guard runs after docopt, so --help still short-circuits.
#[test]
fn output_guard_does_not_shadow_help() {
    let wrk = setup("output_guard_does_not_shadow_help");

    let mut cmd = wrk.command("select");
    cmd.args(["letter,number", "in.csv"])
        .args(["--output", "in.csv"])
        .arg("--help");
    let got = wrk.stdout::<String>(&mut cmd);
    assert!(
        got.contains("Select columns from CSV data"),
        "--help should print usage, got: {got}"
    );
}

/// An input supplied as an ATTACHED flag value must be caught too.
///
/// This is the case a raw-argv scan cannot see: docopt accepts `--flag value`,
/// `--flag=value` and `-fvalue` interchangeably, but only the first spelling puts the path
/// in an argv token of its own. Scanning tokens therefore reads `--payload-tpl=payload.tpl`
/// as one opaque non-path string, misses that the template is an input, and lets the
/// command zero it - the exact bug this guard exists to prevent. Reading docopt's PARSED
/// values instead makes every spelling equivalent.
///
/// Gated on `fetch` because no command in every binary flavor takes an input via a flag;
/// the guard itself is shared code, so the flavor does not affect what is being tested.
#[cfg(all(feature = "fetch", feature = "feature_capable"))]
#[test]
fn output_guard_catches_an_input_passed_as_an_attached_flag_value() {
    const TPL: &str = "{\"n\": {{ number }}}\n";

    // long attached form: --payload-tpl=<file>
    let wrk = setup("output_guard_catches_an_attached_long_flag_value");
    wrk.create_from_string("payload.tpl", TPL);
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("--payload-tpl=payload.tpl")
        .args(["letter", "in.csv"])
        .args(["--output", "payload.tpl"]);
    wrk.assert_err(&mut cmd);
    assert_eq!(
        wrk.read_to_string("payload.tpl").unwrap(),
        TPL,
        "the template is an input; it must not be truncated"
    );

    // short attached form: -t<file>
    let wrk = setup("output_guard_catches_an_attached_short_flag_value");
    wrk.create_from_string("payload.tpl", TPL);
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("-tpayload.tpl")
        .args(["letter", "in.csv"])
        .args(["--output", "payload.tpl"]);
    wrk.assert_err(&mut cmd);
    assert_eq!(wrk.read_to_string("payload.tpl").unwrap(), TPL);
}

/// A docopt-populated DEFAULT must not be mistaken for an input.
///
/// The parsed map carries defaults alongside user-supplied values, and `frequency`'s
/// `--sketch-method` defaults to `exact` — so an unfiltered scan refuses to write a file
/// named `exact` over a value nobody passed and nobody can see. Note that a mere
/// presence-in-argv filter does NOT fix this: when the candidate and the output are the
/// same string, the `-o` spelling is itself an occurrence. The guard therefore requires a
/// second, independent occurrence when the strings coincide.
///
/// `frequency` is present in every binary flavor, so this needs no cfg gate.
#[test]
fn output_guard_ignores_docopt_defaults() {
    let wrk = setup("output_guard_ignores_docopt_defaults");
    wrk.create_from_string("exact", "stale\n");

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").args(["--output", "exact"]);
    wrk.assert_success(&mut cmd);

    assert_ne!(
        wrk.read_to_string("exact").unwrap(),
        "stale\n",
        "the output should have been written"
    );
}

/// The counterpart: the same string typed as a real argument still counts. This is
/// documented limitation #1 (docopt reports values, not their role), pinned so the
/// defaults filter above cannot quietly widen into ignoring user-supplied values.
#[test]
fn output_guard_still_catches_a_user_typed_value_matching_the_output() {
    let wrk = setup("output_guard_still_catches_a_user_typed_value");
    wrk.create_from_string("exact", "stale\n");

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--sketch-method", "exact"])
        .args(["--output", "exact"]);
    wrk.assert_err(&mut cmd);

    assert_eq!(
        wrk.read_to_string("exact").unwrap(),
        "stale\n",
        "a refusal must leave the file untouched"
    );
}
