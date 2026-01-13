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

use owo_colors::{OwoColorize, Rgb};
use serde::Deserialize;
use supports_color::Stream;
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
// table column algorithm. Returns a vector of column widths.
fn autolayout(columns: &[usize], term_width: usize) -> Vec<usize> {
    // |•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|•xxxx•|
    // ↑↑    ↑                                                 ↑
    // 12    3    <-   three chrome chars per column           │
    //                                                         │
    //                                           extra chrome char at the end
    let chrome_width = columns.len() * 3 + 1;

    // How much space is available, and do we already fit?
    const FUDGE: usize = 2;
    let available = term_width - chrome_width - FUDGE;
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
    let max: Vec<usize> = columns.to_vec();

    // W = difference between the available space and the minimum table width
    // D = difference between maximum and minimum table width
    // ratio = W / D
    // col.width = col.min + ((col.max - col.min) * ratio)
    let min_sum: usize = min.iter().sum();
    let max_sum: usize = max.iter().sum();
    let min_sum = min_sum as f64;
    let max_sum = max_sum as f64;
    let ratio = (available as f64 - min_sum) / (max_sum - min_sum);
    if ratio <= 0.0 {
        // even min doesn't fit, we gotta overflow
        return min;
    }

    let mut widths: Vec<usize> = min
        .iter()
        .zip(max.iter())
        .map(|(min, max)| min + ((max - min) as f64 * ratio) as usize)
        .collect();

    // because we always round down, there might be some extra space to distribute
    let data_width: usize = widths.iter().sum();
    if available > data_width {
        let extra_space = available - data_width;
        let mut distribute: Vec<(usize, usize)> = max
            .iter()
            .zip(min.iter())
            .enumerate()
            .map(|(idx, (max, min))| (max - min, idx))
            .collect();

        // Sort by difference (descending), then by index (ascending) for stability
        distribute.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

        for (_, idx) in distribute.into_iter().take(extra_space) {
            widths[idx] += 1;
        }
    }

    widths
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
// dark and light color themes
//

struct Theme {
    chrome: Rgb,
    field: Rgb,
    headers: [Rgb; 6],
}

const DARK: Theme = Theme {
    chrome: Rgb(0x6a, 0x72, 0x82), // gray-500
    field: Rgb(0xe5, 0xe7, 0xeb),  // gray-200
    headers: [
        Rgb(0xff, 0x61, 0x88), // pink
        Rgb(0xfc, 0x98, 0x67), // orange
        Rgb(0xff, 0xd8, 0x66), // yellow
        Rgb(0xa9, 0xdc, 0x76), // green
        Rgb(0x78, 0xdc, 0xe8), // cyan
        Rgb(0xab, 0x9d, 0xf2), // purple
    ],
};

const LIGHT: Theme = Theme {
    chrome: Rgb(0x6a, 0x72, 0x82), // gray-500
    field: Rgb(0x1e, 0x29, 0x39),  // gray-800
    headers: [
        Rgb(0xee, 0x40, 0x66), // red
        Rgb(0xda, 0x76, 0x45), // orange
        Rgb(0xdd, 0xb6, 0x44), // yellow
        Rgb(0x87, 0xba, 0x54), // green
        Rgb(0x56, 0xba, 0xc6), // cyan
        Rgb(0x89, 0x7b, 0xd0), // purple
    ],
};

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

fn truncate(field: &[u8], width: usize) -> String {
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

fn format_field(text: &str, header: bool, col_idx: usize, theme: Option<&Theme>) -> String {
    let Some(theme) = theme else {
        return text.to_string();
    };

    if header {
        let color = theme.headers[col_idx % theme.headers.len()];
        format!("{}", text.color(color).bold())
    } else {
        format!("{}", text.color(theme.field))
    }
}

fn render_separator<W: std::io::Write>(
    out: &mut W,
    widths: &[usize],
    (left, mid, right): (char, char, char),
    theme: Option<&Theme>,
) -> std::io::Result<()> {
    // construct str
    let mut str = String::new();
    str.push(left);
    for (idx, w) in widths.iter().enumerate() {
        if idx > 0 {
            str.push(mid);
        }
        str.extend(std::iter::repeat(BAR).take(*w + 2));
    }
    str.push(right);

    // now write it
    if let Some(theme) = theme {
        writeln!(out, "{}", str.color(theme.chrome))
    } else {
        writeln!(out, "{str}")
    }
}

fn render_row<W: std::io::Write>(
    out: &mut W,
    record: &csv::ByteRecord,
    widths: &[usize],
    align: Align,
    header: bool,
    theme: Option<&Theme>,
) -> std::io::Result<()> {
    let pipe_str = if let Some(theme) = theme {
        format!("{}", PIPE.color(theme.chrome))
    } else {
        PIPE.to_string()
    };

    let mut line = String::new();
    line.push_str(&pipe_str);
    for (idx, field) in record.iter().enumerate() {
        let text = truncate(field, widths[idx]);
        let aligned = align_cell(&text, widths[idx], align);
        let styled = format_field(&aligned, header, idx, theme);
        line.push_str(&" ");
        line.push_str(&styled);
        line.push_str(&" ");
        line.push_str(&pipe_str);
    }
    line.push('\n');
    out.write_all(line.as_bytes())
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
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

    // load all records
    let mut record = csv::ByteRecord::new();
    let mut records: Vec<csv::ByteRecord> = Vec::new();
    while rdr.read_byte_record(&mut record)? {
        records.push(record.clone());
    }
    if records.is_empty() {
        return Ok(());
    }

    // measure column widths
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

    // determine theme
    let theme: Option<&Theme> = if args.flag_monochrome {
        None
    } else if supports_color::on(Stream::Stdout).is_none() {
        None
    } else if let Ok(termbg::Theme::Light) = termbg::theme(std::time::Duration::from_millis(100)) {
        Some(&LIGHT)
    } else {
        Some(&DARK)
    };

    // layout
    let termwidth = if std::io::stdout().is_terminal() {
        textwrap::termwidth()
    } else {
        80
    };
    let widths = autolayout(&columns, termwidth);

    // write
    render_separator(&mut out, &widths, (NW, N, NE), theme)?;
    render_row(&mut out, &records[0], &widths, args.flag_align, true, theme)?;
    render_separator(&mut out, &widths, (W, C, E), theme)?;
    for rec in records.iter().skip(1) {
        render_row(&mut out, rec, &widths, args.flag_align, false, theme)?;
    }
    render_separator(&mut out, &widths, (SW, S, SE), theme)?;

    out.flush()?;
    Ok(())
}
