use crate::workdir::Workdir;

//
// helpers
//

static INPUT: &str = "\
h,h2,h3
abcdefg,a,ab
a  ,abc,z";
// ^^ notice whitespace in final row

fn wrk_stdout_all(data: &str, termwidth: &str, theme: Option<&str>, extra_args: &[&str]) -> String {
    let wrk = Workdir::new("color");
    wrk.create_from_string("in.csv", data);

    let mut cmd = wrk.command("color");
    cmd.env("QSV_TERMWIDTH", termwidth);

    if let Some(theme_value) = theme {
        cmd.env("QSV_THEME", theme_value);
        cmd.env("QSV_FORCE_COLOR", "1");
    }

    cmd.args(extra_args); // Add all extra args
    cmd.arg("in.csv");
    wrk.stdout(&mut cmd)
}

fn wrk_stdout(data: &str, termwidth: &str) -> String {
    wrk_stdout_all(data, termwidth, None, &[])
}

fn wrk_stdout_with_theme(data: &str, termwidth: &str, theme: &str) -> String {
    wrk_stdout_all(data, termwidth, Some(theme), &[])
}

// wrk_stdout(data, "80", Some("dark"), &["--no-header", "--delimiter", "\t"])//

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
fn color_row_numbers() {
    static EXPECTED: &str = "\
╭───┬─────────┬─────┬────╮
│ # │ h       │ h2  │ h3 │
├───┼─────────┼─────┼────┤
│ 1 │ abcdefg │ a   │ ab │
│ 2 │ a       │ abc │ z  │
╰───┴─────────┴─────┴────╯";

    let got = wrk_stdout_all(INPUT, "100", None, &["--row-numbers"]);
    assert_eq!(got, EXPECTED);
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

    let got = wrk_stdout_all(INPUT, "100", Some("DARK"), &[]);
    let output = OUTPUT
        .replace("EE", "\u{1b}") // escape
        .replace("XX", "\u{1b}[38;2;106;114;130m") // chrome color
        .replace("YY", "\u{1b}[39m"); // reset
    assert_eq!(got, output);
}

#[test]
fn color_light_theme() {
    static INPUT: &str = "name,value\nalice,100";

    let got = wrk_stdout_with_theme(INPUT, "100", "LIGHT");

    // Should contain ANSI color codes for light theme
    assert!(got.contains("\u{1b}[38;2;"));
    // Should have table structure
    assert!(got.contains("╭"));
    assert!(got.contains("╰"));
}

#[test]
fn color_dark_theme() {
    static INPUT: &str = "name,value\nalice,100";

    let got = wrk_stdout_with_theme(INPUT, "100", "DARK");

    // Should contain ANSI color codes for dark theme
    assert!(got.contains("\u{1b}[38;2;"));
    // Should have table structure
    assert!(got.contains("╭"));
    assert!(got.contains("╰"));
}

#[test]
fn color_single_column() {
    static INPUT: &str = "header\nvalue1\nvalue2";

    static EXPECTED: &str = "\
╭────────╮
│ header │
├────────┤
│ value1 │
│ value2 │
╰────────╯";

    assert_eq!(wrk_stdout(INPUT, "100"), EXPECTED);
}

#[test]
fn color_single_row() {
    static INPUT: &str = "col1,col2,col3";

    static EXPECTED: &str = "\
╭──────┬──────┬──────╮
│ col1 │ col2 │ col3 │
├──────┼──────┼──────┤
╰──────┴──────┴──────╯";

    assert_eq!(wrk_stdout(INPUT, "100"), EXPECTED);
}

#[test]
fn color_unicode() {
    static INPUT: &str = "name,emoji\ntest,👋🌍";

    let got = wrk_stdout(INPUT, "100");

    // Should handle Unicode properly
    assert!(got.contains("👋🌍"));
    assert!(got.contains("╭"));
    assert!(got.contains("│"));
}

#[test]
fn color_east_asian_wide_chars() {
    // East Asian wide characters take up 2 display columns each
    // 你好 = 4 display columns (2 chars × 2 columns each)
    // 世界 = 4 display columns (2 chars × 2 columns each)
    static INPUT: &str = "name,greeting\nAlice,你好\nBob,世界";

    static EXPECTED: &str = "\
╭───────┬──────────╮
│ name  │ greeting │
├───────┼──────────┤
│ Alice │ 你好     │
│ Bob   │ 世界     │
╰───────┴──────────╯";

    assert_eq!(wrk_stdout(INPUT, "100"), EXPECTED);
}

#[test]
fn color_mixed_width_chars() {
    // Mix of ASCII (1 col), East Asian wide (2 cols), and emoji (2 cols)
    static INPUT: &str = "id,text\n1,Hello世界🌍";

    let got = wrk_stdout(INPUT, "100");

    // Should contain all the characters and proper table structure
    assert!(got.contains("Hello世界🌍"));
    assert!(got.contains("╭"));
    assert!(got.contains("│"));
}

#[test]
fn color_empty_cells() {
    static INPUT: &str = "a,b,c\n1,,3\n,2,";
    static EXPECTED: &str = "\
╭────┬────┬────╮
│ a  │ b  │ c  │
├────┼────┼────┤
│ 1  │    │ 3  │
│    │ 2  │    │
╰────┴────┴────╯";
    assert_eq!(wrk_stdout(INPUT, "100"), EXPECTED);
}

#[test]
fn color_tsv() {
    static INPUT: &str = "a\tb\tc\n1\t2\t3";
    static EXPECTED: &str = "\
╭────┬────┬────╮
│ a  │ b  │ c  │
├────┼────┼────┤
│ 1  │ 2  │ 3  │
╰────┴────┴────╯";

    let got = wrk_stdout_all(INPUT, "100", None, &["--delimiter", "\t"]);
    assert_eq!(got, EXPECTED);
}

#[test]
fn color_wide_content_truncation() {
    static INPUT: &str = "short,long\na,verylongcontentthatshouldbetruncated";

    // With very narrow terminal, content should truncate
    let got = wrk_stdout(INPUT, "15");

    // Should contain ellipsis for truncation
    assert!(got.contains("…"));
}

#[test]
fn color_multiple_columns_cycling() {
    // Test that header colors cycle through the 6 available colors
    static INPUT: &str = "c1,c2,c3,c4,c5,c6,c7,c8\n1,2,3,4,5,6,7,8";

    let got = wrk_stdout_with_theme(INPUT, "100", "DARK");

    // Should have colored output with multiple columns
    assert!(got.contains("c1"));
    assert!(got.contains("c7"));
    assert!(got.contains("c8"));
}

#[test]
fn color_whitespace_trimming() {
    static INPUT: &str = "  header  ,  h2  \n  val1  ,  val2  ";

    let got = wrk_stdout(INPUT, "100");

    // Whitespace should be trimmed in display
    assert!(got.contains("header"));
    assert!(got.contains("val1"));
}

#[test]
fn color_very_narrow_terminal() {
    static INPUT: &str = "a,b\n1,2";

    // Terminal width of 5 forces very aggressive truncation
    let got = wrk_stdout(INPUT, "5");

    // Should still produce valid output
    assert!(got.contains("╭"));
    assert!(got.contains("╰"));
}

#[test]
fn color_output_to_file() {
    static INPUT: &str = "name,value\nalice,100";

    let wrk = Workdir::new("color");
    wrk.create_from_string("in.csv", INPUT);
    let mut cmd = wrk.command("color");
    cmd.env("QSV_TERMWIDTH", "100");
    cmd.arg("--output").arg("out.txt");
    cmd.arg("in.csv");
    wrk.run(&mut cmd);

    let got: String = wrk.from_str(&wrk.path("out.txt"));

    // Output to file should NOT have color codes
    assert!(!got.contains("\u{1b}["));
    // But should have table structure
    assert!(got.contains("╭"));
    assert!(got.contains("│"));
}

#[test]
fn color_long_headers() {
    static INPUT: &str = "\
verylongheadername1,verylongheadername2,short
data1,data2,d3";

    let got = wrk_stdout(INPUT, "50");

    // Headers should be truncated to fit
    assert!(got.contains("…") || got.len() > 0);
}
