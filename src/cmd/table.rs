static USAGE: &str = r##"
Outputs CSV data as a table with columns in alignment.

Though this command is primarily designed for DISPLAYING CSV data using
"elastic tabstops" so its more human-readable, it can also be used to convert
CSV data to other special machine-readable formats:
 -  a more human-readable TSV format with the "leftendtab" alignment option
 -  Fixed-Width format with the "leftfwf" alignment option - similar to "left",
    but with the first line being a comment (prefixed with "#") that enumerates
    the position (1-based, comma-separated) of each column (e.g. "#1,10,15").

This will not work well if the CSV data contains large fields.

Note that formatting a table requires buffering all CSV data into memory.
Therefore, you should use the 'sample' or 'slice' command to trim down large
CSV data before formatting it with this command.

Usage:
    qsv table [options] [<input>]
    qsv table --help

table options:
    -a, --align <arg>      How entries should be aligned in a column.
                           Options: "left", "right", "center". "leftendtab" & "leftfwf"
                           "leftendtab" is a special alignment that similar to "left"
                           but with whitespace padding ending with a tab character.
                           The resulting output still validates as a valid TSV file,
                           while also being more human-readable (aka "aligned" TSV).
                           "leftfwf" is similar to "left" with Fixed Width Format allgnment.
                           The first line is a comment (prefixed with "#") that enumerates
                           the position (1-based, comma-separated) of each column.
                           [default: left]
    --monochrome           Force disable color output (default is auto-on).

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics.
"##;

use std::io::IsTerminal;

use serde::Deserialize;
use textwrap;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    util,
};

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    flag_output: Option<String>,
    flag_delimiter: Option<Delimiter>,
    flag_align: Align,
    flag_memcheck: bool,
    flag_monochrome: bool,
}

#[derive(Deserialize, Clone, Copy)]
enum Align {
    Left,
    Right,
    Center,
    LeftEndTab,
    LeftFwf,
}

#[inline]
fn field_width(field: &[u8]) -> usize {
    // Prefer char count for UTF-8 so emoji/wide chars don't explode layout.
    std::str::from_utf8(field)
        .map(|s| s.chars().count())
        .unwrap_or_else(|_| field.len())
}

// Fit columns into terminal width. This is copied from the very simple HTML
// table column algorithm. Returns a vector of column widths, or None if no
// adjustment is needed.
fn autolayout(columns: &[usize], term_width: usize) -> Option<Vec<usize>> {
    if columns.is_empty() {
        return None;
    }

    // |•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|
    // ↑↑    ↑                                                 ↑
    // 12    3    <-   three chrome chars per column           │
    //                                                         │
    //                                           extra chrome char at the end
    let chrome_width = columns.len() * 3 + 1;

    // How much space is available, and do we already fit?
    const FUDGE: usize = 2;
    let available = term_width - chrome_width - FUDGE;
    let data_width = columns.iter().sum();
    if available >= data_width {
        return None;
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
    let max: Vec<usize> = columns.to_vec();

    // W = difference between the available space and the minimum table width
    // D = difference between maximum and minimum table width
    // ratio = W / D
    // col.width = col.min + ((col.max - col.min) * ratio)
    let min_sum: usize = min.iter().sum();
    let max_sum: usize = max.iter().sum();
    let ratio = ((available - min_sum) as f64) / ((max_sum - min_sum) as f64);
    if ratio <= 0.0 {
        // even min doesn't fit, we gotta overflow
        return Some(min);
    }

    let mut widths: Vec<usize> = min
        .iter()
        .zip(max.iter())
        .map(|(min, max)| min + ((max - min) as f64 * ratio) as usize)
        .collect();

    // because we always round down, there might be some extra space to distribute
    let data_width: usize = widths.iter().sum();
    let extra_space = available - data_width;
    if extra_space > 0 {
        let mut distribute: Vec<(usize, usize)> = max
            .iter()
            .zip(min.iter())
            .enumerate()
            .map(|(ii, (max, min))| (max - min, ii))
            .collect();

        // Sort by difference (descending), then by index (ascending) for stability
        distribute.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

        for (_, idx) in distribute.into_iter().take(extra_space) {
            widths[idx] += 1;
        }
    }

    Some(widths)
}

// Box-drawing characters for pretty separators.
const BOX: [[char; 5]; 4] = [
    ['╭', '─', '┬', '─', '╮'], // 0
    ['│', ' ', '│', ' ', '│'], // 1
    ['├', '─', '┼', '─', '┤'], // 2
    ['╰', '─', '┴', '─', '╯'], // 3
];

// take these from BOX
const NW: char = BOX[0][0];
const N: char = BOX[0][2];
const NE: char = BOX[0][4];
const W: char = BOX[2][0];
const C: char = BOX[2][2];
const E: char = BOX[2][4];
const SW: char = BOX[3][0];
const S: char = BOX[3][2];
const SE: char = BOX[3][4];
const BAR: char = BOX[0][1];
const PIPE: char = BOX[1][0];

fn make_border_line(widths: &[usize], chars: (char, char, char)) -> String {
    let (left, mid, right) = chars;
    let mut line = String::new();
    line.push(left);
    for (idx, w) in widths.iter().enumerate() {
        if idx > 0 {
            line.push(mid);
        }
        line.extend(std::iter::repeat(BAR).take(*w + 2));
    }
    line.push(right);
    line
}

fn align_cell(s: &str, width: usize, align: Align) -> String {
    match align {
        Align::Left | Align::LeftEndTab | Align::LeftFwf => format!("{s:<width$}"),
        Align::Right => format!("{s:>width$}"),
        Align::Center => {
            if width == 0 {
                return String::new();
            }
            let pad_total = width - s.chars().count();
            let left = pad_total / 2;
            let right = pad_total - left;
            format!(
                "{left_spaces}{s}{right_spaces}",
                left_spaces = " ".repeat(left),
                right_spaces = " ".repeat(right)
            )
        },
    }
}

fn colorize_field(
    aligned: &str,
    header: bool,
    col_idx: usize,
    theme: Theme,
    color_enabled: bool,
) -> String {
    if !color_enabled {
        return aligned.to_string();
    }
    let bytes = style_field(aligned.as_bytes(), header, col_idx, theme);
    String::from_utf8_lossy(&bytes).into_owned()
}

fn write_border_line<W: std::io::Write>(
    out: &mut W,
    widths: &[usize],
    chars: (char, char, char),
    color_enabled: bool,
) -> std::io::Result<()> {
    let line = make_border_line(widths, chars);
    if color_enabled {
        writeln!(out, "\u{001b}[38;5;245m{line}\u{001b}[0m")
    } else {
        writeln!(out, "{line}")
    }
}

fn condense_to_width(field: &[u8], width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    let s = String::from_utf8_lossy(field);
    let len = s.chars().count();
    if len <= width {
        return s.into_owned();
    }
    if width == 1 {
        return "…".to_string();
    }
    let mut out = String::new();
    let mut used = 0;
    for ch in s.chars() {
        if used + 1 >= width {
            break;
        }
        out.push(ch);
        used += 1;
    }
    out.push('…');
    out
}

fn write_row_string<W: std::io::Write>(
    out: &mut W,
    record: &csv::ByteRecord,
    widths: &[usize],
    align: Align,
    header: bool,
    color_enabled: bool,
    theme: Theme,
) -> std::io::Result<()> {
    const PAD: usize = 0;
    let mut line = String::new();
    line.push(PIPE);
    for (i, field) in record.iter().enumerate() {
        let col_width = widths[i];
        let content_width = col_width;
        let text = condense_to_width(field, content_width);
        let aligned_inner = align_cell(&text, content_width, align);
        let mut cell = String::new();
        cell.push_str(&" ");
        cell.push_str(&aligned_inner);
        cell.push_str(&" ");
        let styled = colorize_field(&cell, header, i, theme, color_enabled);
        line.push_str(&styled);
        if i + 1 < record.len() {
            line.extend(std::iter::repeat(' ').take(PAD));
            line.push(PIPE);
        } else {
            line.push(PIPE);
        }
    }
    line.push('\n');
    out.write_all(line.as_bytes())
}

#[derive(Clone, Copy)]
enum Theme {
    Light,
    Dark,
}

fn choose_light_theme() -> bool {
    // Use termbg detection; default to dark on failure.
    match termbg::theme(std::time::Duration::from_millis(100)) {
        Ok(termbg::Theme::Light) => true,
        Ok(termbg::Theme::Dark) => false,
        _ => false,
    }
}

fn style_field(field: &[u8], header: bool, col_idx: usize, theme: Theme) -> Vec<u8> {
    const RESET: &[u8] = b"\x1b[0m";
    let header_palette_dark: [&[u8]; 6] = [
        b"\x1b[38;5;204;1m",
        b"\x1b[38;5;209;1m",
        b"\x1b[38;5;221;1m",
        b"\x1b[38;5;114;1m",
        b"\x1b[38;5;81;1m",
        b"\x1b[38;5;141;1m",
    ];
    let header_palette_light: [&[u8]; 6] = [
        b"\x1b[38;5;203;1m",
        b"\x1b[38;5;172;1m",
        b"\x1b[38;5;130;1m",
        b"\x1b[38;5;34;1m",
        b"\x1b[38;5;31;1m",
        b"\x1b[38;5;90;1m",
    ];

    let mut out = Vec::with_capacity(field.len() + 24);
    if header {
        let palette = match theme {
            Theme::Dark => header_palette_dark,
            Theme::Light => header_palette_light,
        };
        out.extend_from_slice(palette[col_idx % palette.len()]);
    }
    out.extend_from_slice(field);
    out.extend_from_slice(RESET);
    out
}
pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let stdout_is_tty = std::io::stdout().is_terminal();
    let force_color = std::env::var_os("FORCE_COLOR").is_some();
    let no_color_env = std::env::var_os("NO_COLOR").is_some() || std::env::var_os("CI").is_some();
    let color_enabled = if args.flag_monochrome {
        false
    } else if no_color_env {
        false
    } else if force_color {
        true
    } else if !stdout_is_tty {
        false
    } else {
        true
    };
    let theme = if choose_light_theme() {
        Theme::Light
    } else {
        Theme::Dark
    };

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(true)
        .flexible(true);

    // we're loading the entire file into memory, we need to check avail mem
    if let Some(path) = rconfig.path.clone() {
        util::mem_file_check(&path, false, args.flag_memcheck)?;
    }

    let wconfig = Config::new(args.flag_output.as_ref()).delimiter(Some(Delimiter(b'\t')));
    let mut out = wconfig.io_writer()?;
    let mut rdr = rconfig.reader()?;

    // Load all records to measure column widths and layout.
    let mut record = csv::ByteRecord::new();
    let mut records: Vec<csv::ByteRecord> = Vec::new();
    while rdr.read_byte_record(&mut record)? {
        records.push(record.clone());
    }

    if records.is_empty() {
        return Ok(());
    }

    let mut columns: Vec<usize> = Vec::new();
    for rec in &records {
        if rec.len() > columns.len() {
            columns.resize(rec.len(), 0);
        }
        for (idx, field) in rec.iter().enumerate() {
            let width = field_width(field).max(2);
            if width > columns[idx] {
                columns[idx] = width;
            }
        }
    }

    let termwidth = if stdout_is_tty {
        textwrap::termwidth()
    } else {
        80
    };

    let content_widths = autolayout(&columns, termwidth);
    let content_widths = content_widths.as_deref().unwrap_or(&columns);
    let render_widths: Vec<usize> = content_widths.iter().map(|w| w + 0).collect();

    // Top border
    write_border_line(&mut out, &render_widths, (NW, N, NE), color_enabled)?;

    // Write header (first record)
    write_row_string(
        &mut out,
        &records[0],
        &render_widths,
        args.flag_align,
        true,
        color_enabled,
        theme,
    )?;

    // Header separator
    write_border_line(&mut out, &render_widths, (W, C, E), color_enabled)?;

    for rec in records.iter().skip(1) {
        write_row_string(
            &mut out,
            rec,
            &render_widths,
            args.flag_align,
            false,
            color_enabled,
            theme,
        )?;
    }

    // Bottom border
    write_border_line(&mut out, &render_widths, (SW, S, SE), color_enabled)?;

    out.flush()?;
    Ok(())
}
