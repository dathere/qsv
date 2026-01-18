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
    arg_input:      Option<String>,
    flag_output:    Option<String>,
    flag_delimiter: Option<Delimiter>,
    flag_memcheck:  bool,
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
    headers: [ContentStyle; 6],
}

// colors courtesy of tabiew/monokai
const COLORS_DARK: Colors = Colors {
    chrome:  fg!(hex!("#6a7282")), // gray-500
    field:   fg!(hex!("#e5e7eb")), // gray-200
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
    chrome:  fg!(hex!("#6a7282")), // gray-500
    field:   fg!(hex!("#1e2939")), // gray-800
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

    // |•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|
    // ↑↑    ↑                                                 ↑
    // 12    3    <-   three chrome chars per column           │
    //                                                         │
    //                                           extra chrome char at the end
    let chrome_width = columns.len() * 3 + 1;

    // How much space is available, and do we already fit?
    let available = term_width.saturating_sub(chrome_width + FUDGE);
    let data_width: usize = columns.iter().sum();
    if available >= data_width {
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
    let data_width: usize = layout.iter().sum();
    let extra_space = available.saturating_sub(data_width);
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

/// Fills a string to the given display width, writing to an existing buffer.
/// This is the optimized version used in hot paths to avoid allocations.
#[inline]
fn fill_into(s: &str, width: usize, buffer: &mut String) {
    buffer.clear();

    if width == 0 {
        return;
    }

    let trimmed = s.trim();
    let display_width = UnicodeWidthStr::width(trimmed);

    match display_width.cmp(&width) {
        std::cmp::Ordering::Equal => {
            buffer.push_str(trimmed);
        },
        std::cmp::Ordering::Less => {
            let pad = width - display_width;
            buffer.reserve(trimmed.len() + pad);
            buffer.push_str(trimmed);
            buffer.extend(std::iter::repeat_n(' ', pad));
        },
        std::cmp::Ordering::Greater => {
            if width != ELLIPSIS_WIDTH {
                let prefix = truncate_to_display_width(trimmed, width - ELLIPSIS_WIDTH);
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

    fill_into("", 0, &mut buffer);
    assert_eq!(buffer, "");

    fill_into("", 1, &mut buffer);
    assert_eq!(buffer, " ");

    fill_into("hello", 0, &mut buffer);
    assert_eq!(buffer, "");

    fill_into("hello", 1, &mut buffer);
    assert_eq!(buffer, "…");

    fill_into("hello", 3, &mut buffer);
    assert_eq!(buffer, "he…");

    fill_into("  hello  ", 3, &mut buffer); // trim
    assert_eq!(buffer, "he…");

    fill_into("hello", 5, &mut buffer);
    assert_eq!(buffer, "hello");

    fill_into("hello", 8, &mut buffer);
    assert_eq!(buffer, "hello   ");
}

//
// colorize
//

/// Colorizes a string and writes it to an existing buffer.
/// This is the optimized version used in hot paths to avoid allocations.
#[inline]
fn colorize_into(
    s: &str,
    header: bool,
    col_idx: usize,
    colors: Option<&Colors>,
    buffer: &mut String,
) {
    buffer.clear();

    let Some(colors) = colors else {
        buffer.push_str(s);
        return;
    };

    let style = if header {
        colors.headers[col_idx % colors.headers.len()]
    } else {
        colors.field
    };

    // Manually write ANSI codes instead of format!() to avoid allocation
    let _ = write!(buffer, "{}", StyledContent::new(style, s));
}

#[test]
fn test_colorize() {
    let mut buffer = String::new();

    colorize_into("FOO", true, 0, Some(&COLORS_DARK), &mut buffer);
    assert_eq!(buffer, "\u{1b}[38;2;255;97;136m\u{1b}[1mFOO\u{1b}[0m");

    colorize_into("BAR", false, 0, Some(&COLORS_LIGHT), &mut buffer);
    assert_eq!(buffer, "\u{1b}[38;2;30;41;57mBAR\u{1b}[39m");

    colorize_into("FOO", true, 0, None, &mut buffer);
    assert_eq!(buffer, "FOO");

    colorize_into("BAR", false, 0, None, &mut buffer);
    assert_eq!(buffer, "BAR");
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

fn force_color() -> bool {
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

fn get_theme(output: bool) -> Theme {
    get_theme_with_env(output, force_color(), qsv_theme())
}

fn get_theme_with_env(output: bool, force_color: bool, qsv_theme: Theme) -> Theme {
    ColorChoice::Auto.write_global(); // reset (for tests)

    // short circuit
    if output {
        ColorChoice::Never.write_global();
    } else if force_color {
        ColorChoice::Always.write_global();
    }

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
fn test_get_theme() {
    assert_eq!(Theme::Dark, get_theme_with_env(false, true, Theme::Dark));
    assert_eq!(Theme::Light, get_theme_with_env(false, true, Theme::Light));
    assert_eq!(Theme::None, get_theme_with_env(true, true, Theme::Dark));
}

//
// render_xxx
//

fn render_sep<W: std::io::Write>(
    out: &mut W,
    layout: &[usize],
    (left, mid, right): (char, char, char),
    colors: Option<&Colors>,
) -> std::io::Result<()> {
    // construct str
    let mut text = String::new();
    text.push(left);
    for (idx, w) in layout.iter().enumerate() {
        if idx > 0 {
            text.push(mid);
        }
        text.extend(std::iter::repeat_n(BAR, *w + 2));
    }
    text.push(right);

    let Some(colors) = colors else {
        return writeln!(out, "{text}");
    };

    writeln!(out, "{}", StyledContent::new(colors.chrome, text))
}

fn render_row<W: std::io::Write>(
    out: &mut W,
    record: &csv::ByteRecord,
    layout: &[usize],
    header: bool,
    colors: Option<&Colors>,
    pipe_str: &str,
    fill_buffer: &mut String,
    colorize_buffer: &mut String,
) -> std::io::Result<()> {
    // Pre-calculate approximate line size:
    // layout.iter().sum() + (layout.len() * 3) for pipes/spaces + ANSI codes
    let line_capacity = layout.iter().sum::<usize>() + (layout.len() * 3) + 100;
    let mut line = String::with_capacity(line_capacity);

    line.push_str(pipe_str);
    for (idx, field) in record.iter().enumerate() {
        // safety: flexible(false) ensures all records have same field count as headers,
        // so idx is always within bounds of layout (which is sized to headers.len())
        let raw = String::from_utf8_lossy(field);
        fill_into(&raw, layout[idx], fill_buffer);
        colorize_into(fill_buffer, header, idx, colors, colorize_buffer);
        line.push(' ');
        line.push_str(colorize_buffer);
        line.push(' ');
        line.push_str(pipe_str);
    }
    line.push('\n');
    out.write_all(line.as_bytes())
}

//
// run
//

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
    // layout, look and feel
    //

    // measure the maximum width for each column. Never <2 chars
    let mut columns: Vec<usize> = vec![2; headers.len()];
    for rec in &records {
        for (idx, field) in rec.iter().enumerate() {
            columns[idx] = columns[idx].max(field_width(field));
        }
    }
    let colors = match get_theme(args.flag_output.is_some()) {
        Theme::Dark => Some(&COLORS_DARK),
        Theme::Light => Some(&COLORS_LIGHT),
        Theme::None => None,
    };
    let layout = autolayout(&columns, get_termwidth());

    //
    // write
    //

    let wconfig = Config::new(args.flag_output.as_ref())
        .delimiter(Some(Delimiter(b'\t')))
        .set_write_buffer(DEFAULT_WTR_BUFFER_CAPACITY * 4);
    let mut out = wconfig.io_writer()?;

    // Cache pipe_str to avoid repeated allocations
    let pipe_str = if let Some(colors) = colors {
        format!("{}", StyledContent::new(colors.chrome, PIPE))
    } else {
        PIPE.to_string()
    };

    // Create reusable buffers for fill and colorize operations
    let mut fill_buffer = String::new();
    let mut colorize_buffer = String::new();

    render_sep(&mut out, &layout, (NW, N, NE), colors)?;
    render_row(
        &mut out,
        headers,
        &layout,
        true,
        colors,
        &pipe_str,
        &mut fill_buffer,
        &mut colorize_buffer,
    )?;
    render_sep(&mut out, &layout, (W, C, E), colors)?;
    for rec in records.iter().skip(1) {
        render_row(
            &mut out,
            rec,
            &layout,
            false,
            colors,
            &pipe_str,
            &mut fill_buffer,
            &mut colorize_buffer,
        )?;
    }
    render_sep(&mut out, &layout, (SW, S, SE), colors)?;
    out.flush()?;

    Ok(())
}
