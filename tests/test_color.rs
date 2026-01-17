use crate::workdir::Workdir;

//
// helpers
//

static INPUT: &str = "\
h,h2,h3
abcdefg,a,ab
a  ,abc,z";
// ^^ notice whitespace in final row

fn wrk_stdout(data: &str, termwidth: &str) -> String {
    let wrk = Workdir::new("color");
    wrk.create_from_string("in.csv", data);
    let mut cmd = wrk.command("color");
    cmd.env("QSV_TERMWIDTH", termwidth);
    cmd.arg("in.csv");
    wrk.stdout(&mut cmd)
}

//
// basic test
//

#[test]
fn color() {
    // edge cases
    assert_eq!(wrk_stdout("", "100"), ""); // empty
    assert_eq!(wrk_stdout("\n", "100"), ""); // no cols

    static SMALL: &str = "\
╭────┬────┬────╮
│ h  │ h2 │ h3 │
├────┼────┼────┤
│ a… │ a  │ ab │
│ a  │ a… │ z  │
╰────┴────┴────╯";
    assert_eq!(wrk_stdout(INPUT, "5"), SMALL);

    static LARGE: &str = "\
╭─────────┬─────┬────╮
│ h       │ h2  │ h3 │
├─────────┼─────┼────┤
│ abcdefg │ a   │ ab │
│ a       │ abc │ z  │
╰─────────┴─────┴────╯";
    assert_eq!(wrk_stdout(INPUT, "100"), LARGE);
}

#[test]
fn color_ragged() {
    let wrk = Workdir::new("color").flexible(true);
    wrk.create_from_string("in.csv", "1\n1,2");
    let mut cmd = wrk.command("color");
    cmd.arg("in.csv");
    cmd.env("QSV_TERMWIDTH", "100");
    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.contains("found record with 2 fields, but the previous record has 1 fields"));
}

#[test]
fn color_with_color() {
    static INPUT: &str = "\
  h,h2
  a,b";

    static OUTPUT: &str = "\
XX╭─────┬────╮YY
XX│YY EE[38;2;255;97;136mEE[1mh  EE[0m XX│YY EE[38;2;252;152;103mEE[1mh2EE[0m XX│YY
XX├─────┼────┤YY
XX│YY EE[38;2;229;231;235ma  YY XX│YY EE[38;2;229;231;235mb YY XX│YY
XX╰─────┴────╯YY";

    let wrk = Workdir::new("color").flexible(true);
    wrk.create_from_string("in.csv", INPUT);
    let mut cmd = wrk.command("color");
    cmd.arg("in.csv");
    cmd.env("FORCE_COLOR", "1");
    cmd.env("QSV_THEME", "DARK");
    cmd.env("QSV_TERMWIDTH", "100");
    let got: String = wrk.stdout(&mut cmd);

    let output = OUTPUT
        .replace("EE", "\u{1b}") // escape
        .replace("XX", "\u{1b}[38;2;106;114;130m") // chrome color
        .replace("YY", "\u{1b}[39m"); // reset
    assert_eq!(got, output);
}
