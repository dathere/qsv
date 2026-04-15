static USAGE: &str = r#"
Outputs tabular data as a pretty, colorized table that always fits into the
terminal.

Tabular data formats include CSV and its dialects, Arrow, Avro/IPC, Parquet,
JSON Array & JSONL. Note that non-CSV formats require the "polars" feature.

Requires buffering all tabular data into memory. Therefore, you should use the
'sample' or 'slice' command to trim down large CSV data before formatting
it with this command.

Color is turned off when redirecting or running CI. Set QSV_FORCE_COLOR=1
to override this behavior.

The color theme is detected based on the current terminal background color
if possible. Set QSV_THEME to DARK or LIGHT to skip detection. QSV_TERMWIDTH
can be used to override terminal size.

Usage:
    qsv color [options] [<input>]
    qsv color --help

color options:
    -C, --color            Force color on, even in situations where colors
                           would normally be disabled.
    -n, --row-numbers      Show row numbers.
    -t, --title <str>      Add a title row above the headers.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics.
"#;

use std::{fmt::Write, io::IsTerminal, str::FromStr};

use anstream::{AutoStream, ColorChoice};
use crossterm::style::{Attribute, Attributes, Color, ContentStyle, StyledContent};
use csv::ByteRecord;
use serde::Deserialize;
use strum_macros::EnumString;
use terminal_colorsaurus::{QueryOptions, ThemeMode, theme_mode};
use textwrap;

use crate::{
    CliResult,
    config::{Config, DEFAULT_WTR_BUFFER_CAPACITY, Delimiter},
    util::{self, get_envvar_flag},
};

#[derive(Deserialize)]
struct Args {
    arg_input:        Option<String>,
    flag_color:       bool,
    flag_delimiter:   Option<Delimiter>,
    flag_memcheck:    bool,
    flag_output:      Option<String>,
    flag_row_numbers: bool,
    flag_title:       Option<String>,
}

//
// our state
//

struct ColorStruct<'a> {
    colors:      Option<&'a Colors>,
    headers:     ByteRecord,
    layout:      Vec<usize>,
    pipe:        String,
    records:     Vec<ByteRecord>,
    row_numbers: bool,
}

//
// dark and light colors
//

macro_rules! hex {
    ($hex:expr) => {{
        const fn parse_hex(str: &str) -> Color {
            let bytes = str.as_bytes();
            assert!(bytes.len() == 7);
            let r = (hex_digit(bytes[1]) << 4) | hex_digit(bytes[2]);
            let g = (hex_digit(bytes[3]) << 4) | hex_digit(bytes[4]);
            let b = (hex_digit(bytes[5]) << 4) | hex_digit(bytes[6]);
            Color::Rgb { r, g, b }
        }

        const fn hex_digit(ch: u8) -> u8 {
            match ch {
                b'0'..=b'9' => ch - b'0',
                b'A'..=b'F' => ch - b'A' + 10,
                b'a'..=b'f' => ch - b'a' + 10,
                _ => 0,
            }
        }

        parse_hex($hex)
    }};
}

macro_rules! fg {
    ($fg: expr) => {
        ContentStyle {
            foreground_color: Some($fg),
            background_color: None,
            underline_color:  None,
            attributes:       Attributes::none(),
        }
    };
}

macro_rules! bold {
    ($fg: expr) => {
        ContentStyle {
            foreground_color: Some($fg),
            background_color: None,
            underline_color:  None,
            attributes:       Attributes::none().with(Attribute::Bold),
        }
    };
}

struct Colors {
    chrome:  ContentStyle,
    field:   ContentStyle,
    title:   ContentStyle,
    headers: [ContentStyle; 6],
}

// colors courtesy of tabiew/monokai
const COLORS_DARK: Colors = Colors {
    chrome:  fg!(hex!("#6a7282")),   // gray-500
    field:   fg!(hex!("#e5e7eb")),   // gray-200
    title:   bold!(hex!("#60a5fa")), // blue-400
    headers: [
        bold!(hex!("#ff6188")), // pink
        bold!(hex!("#fc9867")), // orange
        bold!(hex!("#ffd866")), // yellow
        bold!(hex!("#a9dc76")), // green
        bold!(hex!("#78dce8")), // cyan
        bold!(hex!("#ab9df2")), // purple
    ],
};

// colors courtesy of tabiew/monokai
const COLORS_LIGHT: Colors = Colors {
    chrome:  fg!(hex!("#6a7282")),   // gray-500
    field:   fg!(hex!("#1e2939")),   // gray-800
    title:   bold!(hex!("#2563eb")), // blue-600
    headers: [
        bold!(hex!("#ee4066")), // red
        bold!(hex!("#da7645")), // orange
        bold!(hex!("#ddb644")), // yellow
        bold!(hex!("#87ba54")), // green
        bold!(hex!("#56bac6")), // cyan
        bold!(hex!("#897bd0")), // purple
    ],
};

// which theme are we using?
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
#[strum(ascii_case_insensitive)]
enum Theme {
    Dark,
    Light,
    None,
}

//
// Autolayout columns into terminal width. This is copied from the very simple HTML table column
// algorithm. Returns a vector of column widths.
//

fn autolayout(columns: &[usize], term_width: usize) -> Vec<usize> {
    const FUDGE: usize = 2;

    if columns.is_empty() {
        // edge case
        return columns.to_vec();
    }

    let chrome_width = get_chrome_width(columns);

    // How much space is available, and do we already fit?
    let available = term_width.saturating_sub(chrome_width + FUDGE);
    if available >= get_data_width(columns) {
        return columns.to_vec();
    }

    // We don't fit, so we are going to shrink (truncate) some columns.
    // Potentially all the way down to a lower bound. But what is the lower
    // bound? It's nice to have a generous value so that narrow columns have a
    // shot at avoiding truncation. That isn't always possible, though.
    let lower_bound = (available / columns.len()).clamp(2, 10);

    // Calculate a "min" and a "max" for each column, then allocate available
    // space proportionally to each column. This is similar to the algorithm for
    // HTML tables.
    let min: Vec<usize> = columns.iter().map(|w| (*w).min(lower_bound)).collect();
    let max = columns; // Use reference to columns instead of cloning

    // W = difference between the available space and the minimum table width
    // D = difference between maximum and minimum table width
    // ratio = W / D
    // col.width = col.min + ((col.max - col.min) * ratio)
    let min_sum: usize = min.iter().sum();
    let max_sum: usize = max.iter().sum();
    if min_sum == max_sum {
        // edge case
        return min;
    }

    #[allow(clippy::cast_precision_loss)]
    let ratio = (available.saturating_sub(min_sum) as f64) / ((max_sum - min_sum) as f64);
    if ratio == 0.0 {
        // even min doesn't fit, we gotta overflow
        return min;
    }

    #[allow(clippy::cast_precision_loss)]
    let mut layout: Vec<usize> = min
        .iter()
        .zip(max.iter())
        .map(|(min, max)| min + ((max - min) as f64 * ratio) as usize)
        .collect();

    // because we always round down, there might be some extra space to distribute
    let extra_space = available.saturating_sub(get_data_width(&layout));
    if extra_space > 0 {
        let mut distribute: Vec<(usize, usize)> = max
            .iter()
            .zip(min.iter())
            .enumerate()
            .map(|(idx, (max, min))| (max - min, idx))
            .collect();

        // Sort by difference (descending), then by index (ascending) for stability
        distribute.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

        for (_, idx) in distribute.into_iter().take(extra_space) {
            layout[idx] += 1;
        }
    }

    layout
}

// |•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|
// ↑↑    ↑                                                 ↑
// 12    3    <-   three chrome chars per column           │
//                                                         │
//                                           extra chrome char at the end
// total width of chrome in one row, according to this layout
const fn get_chrome_width(layout: &[usize]) -> usize {
    layout.len() * 3 + 1
}

// width of all data in one row according to this layout
fn get_data_width(layout: &[usize]) -> usize {
    layout.iter().sum()
}

// total width of table, according to this layout
fn get_table_width(layout: &[usize]) -> usize {
    get_chrome_width(layout) + get_data_width(layout)
}

//
// Box-drawing characters for pretty separators.
//

const BOX: [[char; 5]; 4] = [
    ['╭', '─', '┬', '─', '╮'], // 0
    ['│', ' ', '│', ' ', '│'], // 1
    ['├', '─', '┼', '─', '┤'], // 2
    ['╰', '─', '┴', '─', '╯'], // 3
];

// take these from BOX
const NW: char = BOX[0][0];
const NE: char = BOX[0][4];
const SE: char = BOX[3][4];
const SW: char = BOX[3][0];
const N: char = BOX[0][2];
const E: char = BOX[2][4];
const S: char = BOX[3][2];
const W: char = BOX[2][0];
const C: char = BOX[2][2];
const BAR: char = BOX[0][1];
const PIPE: char = BOX[1][0];

//
// fill
//

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

const ELLIPSIS: &str = "…";
const ELLIPSIS_WIDTH: usize = 1; // Display width of ellipsis

fn truncate_to_display_width(s: &str, max_width: usize) -> &str {
    if max_width == 0 {
        return "";
    }

    let mut width = 0;
    let mut end = 0;

    for (idx, ch) in s.char_indices() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);
        if width + ch_width > max_width {
            break;
        }
        width += ch_width;
        end = idx + ch.len_utf8();
    }

    &s[..end]
}

#[derive(Clone, Copy)]
enum Align {
    Left,
    Center,
}

/// Fills a string to the given display width, writing to an existing buffer.
/// This is the optimized version used in hot paths to avoid allocations.
#[inline]
fn fill_into(s: &str, width: usize, align: Align, buffer: &mut String) {
    buffer.clear();

    if width == 0 {
        return;
    }

    let display_width = UnicodeWidthStr::width(s);

    match display_width.cmp(&width) {
        std::cmp::Ordering::Equal => {
            buffer.push_str(s);
        },
        std::cmp::Ordering::Less => {
            buffer.reserve(width);
            let pad = width - display_width;
            match align {
                Align::Left => {
                    buffer.push_str(s);
                    buffer.extend(std::iter::repeat_n(' ', pad));
                },
                Align::Center => {
                    let half = pad / 2;
                    buffer.extend(std::iter::repeat_n(' ', half));
                    buffer.push_str(s);
                    buffer.extend(std::iter::repeat_n(' ', pad - half));
                },
            }
        },
        std::cmp::Ordering::Greater => {
            if width != ELLIPSIS_WIDTH {
                let prefix = truncate_to_display_width(s, width - ELLIPSIS_WIDTH);
                buffer.reserve(prefix.len() + ELLIPSIS.len());
                buffer.push_str(prefix);
            }
            buffer.push_str(ELLIPSIS);
        },
    }
}

#[test]
fn test_fill() {
    let mut buffer = String::new();

    fill_into("", 0, Align::Left, &mut buffer);
    assert_eq!(buffer, "");

    fill_into("", 1, Align::Left, &mut buffer);
    assert_eq!(buffer, " ");

    fill_into("hello", 0, Align::Left, &mut buffer);
    assert_eq!(buffer, "");

    fill_into("hello", 1, Align::Left, &mut buffer);
    assert_eq!(buffer, "…");

    fill_into("hello", 3, Align::Left, &mut buffer);
    assert_eq!(buffer, "he…");

    fill_into("hello", 5, Align::Left, &mut buffer);
    assert_eq!(buffer, "hello");

    fill_into("hello", 8, Align::Left, &mut buffer);
    assert_eq!(buffer, "hello   ");
}

//
// field_width
//

#[inline]
fn field_width(field: &[u8]) -> usize {
    // Use display width for UTF-8 so East Asian wide chars/emoji align correctly.
    std::str::from_utf8(field).map_or_else(
        |_| field.len(),
        |s| {
            use unicode_width::UnicodeWidthStr;
            s.width()
        },
    )
}

#[test]
fn test_field_width() {
    assert_eq!(field_width(b""), 0);
    assert_eq!(field_width(b"hello"), 5);
    assert_eq!(field_width(b"\xF0\x9F\x91\x8B\xF0\x9F\x8C\x8D"), 4); // Emoji 👋🌍 (2 cols each)
}

//
// env helpers
//

fn qsv_force_color() -> bool {
    get_envvar_flag("QSV_FORCE_COLOR")
}

fn qsv_termwidth() -> Option<usize> {
    match std::env::var("QSV_TERMWIDTH").ok() {
        Some(s) => match s.parse::<usize>() {
            Ok(val) if (1..=1000).contains(&val) => Some(val),
            _ => None,
        },
        None => None,
    }
}

fn qsv_theme() -> Theme {
    match std::env::var("QSV_THEME").ok() {
        Some(s) => Theme::from_str(&s).unwrap_or(Theme::None),
        None => Theme::None,
    }
}

//
// get_termwidth
//

fn get_termwidth() -> usize {
    get_termwidth_with_env(qsv_termwidth())
}

fn get_termwidth_with_env(qsv_termwidth: Option<usize>) -> usize {
    if let Some(qsv_termwidth) = qsv_termwidth {
        qsv_termwidth
    } else if std::io::stdout().is_terminal() {
        textwrap::termwidth()
    } else {
        80
    }
}

#[test]
fn test_termwidth() {
    let default = textwrap::termwidth();
    assert_eq!(get_termwidth_with_env(None), default);
    assert_eq!(get_termwidth_with_env(Some(123)), 123);
}

//
// get_theme
//

fn get_theme(qsv_theme: Theme) -> Theme {
    #[allow(clippy::equatable_if_let)]
    if AutoStream::choice(&std::io::stdout()) == ColorChoice::Never {
        Theme::None
    } else if qsv_theme != Theme::None {
        qsv_theme
    } else if let Ok(ThemeMode::Light) = theme_mode(QueryOptions::default()) {
        Theme::Light
    } else {
        Theme::Dark
    }
}

#[test]
#[ignore = "depends on terminal color detection, environment and platform"]
fn test_get_theme() {
    // Ensure color output is enabled
    ColorChoice::Auto.write_global();
    assert_eq!(Theme::Dark, get_theme(Theme::Dark));
    assert_eq!(Theme::Light, get_theme(Theme::Light));

    // Now explicitly test None theme with color output disabled
    ColorChoice::Never.write_global();
    assert_eq!(Theme::None, get_theme(Theme::Dark));

    // Reset color choice to avoid interference with other tests
    ColorChoice::Auto.write_global();
}

//
// setup_color_choice
//

/// Determine if we should force color on or off. Cli flags always take precedence. Note that when
/// using ColorChoice::Auto, anstyle makes its own decision based on stdout tty, common env
/// variables, terminal detection, etc.
fn setup_color_choice(flag_color: bool, flag_output: bool, qsv_force_color: bool) {
    let color_choice = if flag_color {
        ColorChoice::Always
    } else if flag_output {
        ColorChoice::Never
    } else if qsv_force_color {
        ColorChoice::Always
    } else {
        ColorChoice::Auto
    };

    // tell anstyle
    color_choice.write_global();
}

#[test]
fn test_get_color_choice() {
    let test_cases = [
        // (flag_color, flag_output, qsv_force_color, expected)
        (false, false, false, ColorChoice::Auto),
        (true, false, false, ColorChoice::Always),
        (false, true, false, ColorChoice::Never),
        (false, false, true, ColorChoice::Always),
        (true, true, false, ColorChoice::Always),
    ];
    for (flag_color, flag_output, qsv_force_color, exp) in test_cases {
        setup_color_choice(flag_color, flag_output, qsv_force_color);
        assert_eq!(ColorChoice::global(), exp);
    }
}

//
// render_xxx
//

fn render_sep<W: std::io::Write>(
    out: &mut W,
    color_struct: &ColorStruct,
    (left, mid, right): (char, char, char),
) -> std::io::Result<()> {
    // construct str
    let mut text = String::new();
    text.push(left);
    for (idx, w) in color_struct.layout.iter().enumerate() {
        if idx > 0 {
            text.push(mid);
        }
        text.extend(std::iter::repeat_n(BAR, *w + 2));
    }
    text.push(right);

    let Some(colors) = color_struct.colors else {
        return writeln!(out, "{text}");
    };

    writeln!(out, "{}", StyledContent::new(colors.chrome, text))
}

fn render_title<W: std::io::Write>(
    out: &mut W,
    color_struct: &ColorStruct,
    title: &str,
) -> std::io::Result<()> {
    // center the title
    const EDGES: usize = 4; // |•xxxxxx•|
    let width = get_table_width(&color_struct.layout) - EDGES;
    let mut buf = String::new();
    fill_into(title, width, Align::Center, &mut buf);

    let mut line = String::new();
    line.push_str(&color_struct.pipe);
    line.push(' ');
    if let Some(colors) = color_struct.colors {
        let _ = write!(
            &mut line,
            "{}",
            StyledContent::new(colors.title, buf.as_str())
        );
    } else {
        line.push_str(buf.as_str());
    }
    line.push(' ');
    line.push_str(&color_struct.pipe);

    writeln!(out, "{line}")
}

// row number header and display width
const RN_HEADER: &str = "#";
const RN_WIDTH: usize = 1;

fn render_row<W: std::io::Write>(
    out: &mut W,
    color_struct: &ColorStruct,
    row_idx: usize,
    fill_buffer: &mut String,
) -> std::io::Result<()> {
    let layout = &color_struct.layout;

    // Pre-calculate approximate line size: table + ANSI codes
    let line_capacity = get_table_width(layout) + 100;
    let mut line = String::with_capacity(line_capacity);
    line.push_str(&color_struct.pipe);

    // Add row_numbers to line if necessary. Another approach would be to modify records earlier in
    // the flow, but that would be expensive.
    let mut col_idx = 0;
    if color_struct.row_numbers {
        let text = if row_idx == 0 {
            RN_HEADER
        } else {
            &row_idx.to_string() // field
        };
        render_cell(color_struct, text, row_idx, col_idx, fill_buffer, &mut line);
        col_idx += 1;
    }

    let record = &color_struct.records[row_idx];
    for field in record {
        let raw = String::from_utf8_lossy(field);
        render_cell(color_struct, &raw, row_idx, col_idx, fill_buffer, &mut line);
        col_idx += 1;
    }
    line.push('\n');

    out.write_all(line.as_bytes())
}

const PLACEHOLDER: &str = "—";

fn render_cell(
    color_struct: &ColorStruct,
    cell: &str,
    row_idx: usize,
    col_idx: usize,
    fill_buffer: &mut String,
    line: &mut String,
) {
    // switch to placeholder if necessary
    let cell = cell.trim();
    let placeholder = cell.is_empty();
    let cell = if placeholder { PLACEHOLDER } else { cell };

    // fill
    // safety: flexible(false) ensures all records have same field count as headers, so col_idx is
    // always within bounds of layout. When row_numbers is enabled, layout is sized to headers.len()
    // + 1, with layout[0] reserved for the row number column; otherwise it is sized to
    // headers.len().
    fill_into(cell, color_struct.layout[col_idx], Align::Left, fill_buffer);

    line.push(' ');
    if let Some(colors) = color_struct.colors {
        let style = if row_idx == 0 {
            colors.headers[col_idx % colors.headers.len()]
        } else if placeholder || (color_struct.row_numbers && col_idx == 0) {
            colors.chrome
        } else {
            colors.field
        };
        let _ = write!(line, "{}", StyledContent::new(style, fill_buffer));
    } else {
        // no styling
        line.push_str(fill_buffer);
    }
    line.push(' ');
    line.push_str(&color_struct.pipe);
}

//
// run
//

#[allow(clippy::cast_precision_loss)]
fn num_digits(x: usize) -> usize {
    if x == 0 {
        // edge case
        return 1;
    }
    ((x as f64).log10().floor() as usize) + 1
}

#[test]
fn test_num_digits() {
    assert_eq!(num_digits(0), 1);
    assert_eq!(num_digits(1), 1);
    assert_eq!(num_digits(9), 1);
    assert_eq!(num_digits(10), 2);
    assert_eq!(num_digits(9999), 4);
    assert_eq!(num_digits(10000), 5);
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(true)
        .flexible(false); // don't support ragged csvs for now

    // we're loading the entire file into memory, we need to check avail mem
    if let Some(path) = rconfig.path.clone() {
        util::mem_file_check(&path, false, args.flag_memcheck)?;
    }

    // setup ColorChoice based on args/env
    setup_color_choice(
        args.flag_color,
        args.flag_output.is_some(),
        qsv_force_color(),
    );

    //
    // read
    //

    let mut rdr = rconfig.reader()?;
    let records = rdr.byte_records().collect::<Result<Vec<_>, _>>()?;
    if records.is_empty() {
        // edge case
        return Ok(());
    }
    let headers = &records[0];
    if headers.is_empty() {
        // edge case
        return Ok(());
    }

    //
    // ColorStruct (our state)
    //

    let mut color_struct = ColorStruct {
        headers:     headers.clone(),
        records:     records.clone(),
        row_numbers: args.flag_row_numbers,
        // these get setup later
        colors:      None,
        layout:      Vec::new(),
        pipe:        String::new(),
    };
    let mut fill_buffer = String::new();

    // measure the maximum width for each column. Never <2 chars
    let mut columns: Vec<usize> = vec![2; color_struct.headers.len()];
    for rec in &records {
        for (idx, field) in rec.iter().enumerate() {
            columns[idx] = columns[idx].max(field_width(field));
        }
    }
    if color_struct.row_numbers {
        // prepend row number column
        columns.insert(0, num_digits(color_struct.records.len() - 1).max(RN_WIDTH));
    }
    color_struct.colors = match get_theme(qsv_theme()) {
        Theme::Dark => Some(&COLORS_DARK),
        Theme::Light => Some(&COLORS_LIGHT),
        Theme::None => None,
    };
    color_struct.layout = autolayout(&columns, get_termwidth());
    color_struct.pipe = if let Some(colors) = color_struct.colors {
        format!("{}", StyledContent::new(colors.chrome, PIPE))
    } else {
        PIPE.to_string()
    };

    //
    // write
    //

    let wconfig = Config::new(args.flag_output.as_ref())
        .delimiter(Some(Delimiter(b'\t')))
        .set_write_buffer(DEFAULT_WTR_BUFFER_CAPACITY * 4);
    let mut out = wconfig.io_writer()?;

    // title, or not
    if let Some(title) = args.flag_title {
        render_sep(&mut out, &color_struct, (NW, BAR, NE))?;
        render_title(&mut out, &color_struct, &title)?;
        render_sep(&mut out, &color_struct, (W, N, E))?;
    } else {
        render_sep(&mut out, &color_struct, (NW, N, NE))?;
    }

    for (idx, _) in records.iter().enumerate() {
        render_row(&mut out, &color_struct, idx, &mut fill_buffer)?;
        if idx == 0 {
            render_sep(&mut out, &color_struct, (W, C, E))?;
        }
    }
    render_sep(&mut out, &color_struct, (SW, S, SE))?;
    out.flush()?;

    Ok(())
}
