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
    -w, --width <arg>      The minimum width of each column.
                           [default: 2]
    -p, --pad <arg>        The minimum number of spaces between each column.
                           [default: 0]
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
    -c, --condense <arg>   Limits the length of each field to the value
                           specified. If the field is UTF-8 encoded, then
                           <arg> refers to the number of code points.
                           Otherwise, it refers to the number of bytes.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    --monochrome           Force disable color output (default is auto-on).
    --max-width <cols>     Target width (columns). Defaults to terminal width
                           when stdout is a TTY.
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
    arg_input:       Option<String>,
    flag_width:      usize,
    flag_pad:        usize,
    flag_output:     Option<String>,
    flag_delimiter:  Option<Delimiter>,
    flag_align:      Align,
    flag_condense:   Option<usize>,
    flag_memcheck:   bool,
    flag_monochrome: bool,
    flag_max_width:  Option<usize>,
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

fn autolayout(
    max_widths: &[usize],
    pad: usize,
    min_width: usize,
    target_width: Option<usize>,
) -> Option<Vec<usize>> {
    const FUDGE: usize = 2;
    let screen_width = target_width?;
    if max_widths.is_empty() {
        return None;
    }

    // Available width after accounting for borders, separators, padding, inner padding, and fudge.
    let cols = max_widths.len();
    let chrome = cols + 1; // borders + vertical separators
    let pad_total = pad.saturating_mul(cols.saturating_sub(1));
    let inner_pad_total = 2 * INNER_PAD * cols;
    let available_total = screen_width.saturating_sub(FUDGE);
    if available_total <= chrome + pad_total + inner_pad_total {
        return Some(vec![min_width; cols]);
    }
    // Available content width for fields (excluding inner padding, chrome, pad).
    let available = available_total
        .saturating_sub(chrome)
        .saturating_sub(pad_total)
        .saturating_sub(inner_pad_total);

    // Width of data as-is.
    let data_width: usize = max_widths.iter().sum();
    if available >= data_width {
        return None;
    }

    // Lower bound per column tries to give every column a chance.
    // Ensure at least 3 to leave room for an ellipsis when truncated.
    let lower_bound = (available / cols).clamp(2, 10).max(min_width.max(3));

    let min: Vec<usize> = max_widths.iter().map(|w| (*w).min(lower_bound)).collect();
    let max: Vec<usize> = max_widths.to_vec();

    let min_sum: usize = min.iter().sum();
    if available <= min_sum {
        // Even our minimum table won't fit.
        return Some(min);
    }

    let max_sum: usize = max.iter().sum();
    let denom = max_sum.saturating_sub(min_sum);
    if denom == 0 {
        return Some(min);
    }

    let ratio = (available - min_sum) as f64 / denom as f64;
    if ratio <= 0.0 {
        return Some(min);
    }

    let mut widths: Vec<usize> = min
        .iter()
        .zip(max.iter())
        .map(|(min_w, max_w)| {
            let extra = ((*max_w - *min_w) as f64 * ratio).floor() as usize;
            min_w + extra
        })
        .collect();

    // Because of floor() we might have spare columns to distribute.
    let current_sum: usize = widths.iter().sum();
    if available > current_sum {
        let mut distribute: Vec<(usize, usize)> = max
            .iter()
            .zip(min.iter())
            .enumerate()
            .map(|(idx, (max_w, min_w))| (max_w - min_w, idx))
            .collect();
        distribute.sort_by(|a, b| b.cmp(a));
        let extra_space = available - current_sum;
        for (_, idx) in distribute.into_iter().take(extra_space) {
            widths[idx] += 1;
        }
    }

    // Final safety: if rounding/ceil made us overflow, trim the widest columns
    // (but never below min_width) until we fit within the terminal.
    let chrome = cols + 1;
    let inner_pad_total = 2 * INNER_PAD * cols;
    let limit = screen_width.saturating_sub(FUDGE);
    let total_with_pad: usize = widths.iter().sum::<usize>() + pad_total + chrome + inner_pad_total;
    if total_with_pad > limit {
        let mut order: Vec<usize> = (0..widths.len()).collect();
        order.sort_by_key(|&i| std::cmp::Reverse(widths[i]));
        let mut excess = total_with_pad - limit;
        while excess > 0 {
            let mut trimmed = false;
            for &idx in &order {
                if widths[idx] > min_width {
                    widths[idx] -= 1;
                    excess -= 1;
                    trimmed = true;
                    if excess == 0 {
                        break;
                    }
                }
            }
            if !trimmed {
                break;
            }
        }
    }

    Some(widths)
}

const INNER_PAD: usize = 1; // spaces inside each cell on both sides.

// Box-drawing characters for pretty separators.
const BOX_TOP: (char, char, char, char) = ('╭', '┬', '╮', '─');
const BOX_MID: (char, char, char, char) = ('├', '┼', '┤', '─');
const BOX_BOT: (char, char, char, char) = ('╰', '┴', '╯', '─');

fn make_border_line(
    widths: &[usize],
    pad: usize,
    chars: (char, char, char, char),
) -> Vec<String> {
    let (left, mid, right, fill) = chars;
    let mut fields = Vec::with_capacity(widths.len());
    for (i, w) in widths.iter().enumerate() {
        let mut s = String::new();
        if i == 0 {
            s.push(left);
        } else {
            s.push(mid);
        }
        s.extend(std::iter::repeat(fill).take(*w));
        if i + 1 < widths.len() {
            s.extend(std::iter::repeat(fill).take(pad));
        }
        if i + 1 == widths.len() {
            s.push(right);
        }
        fields.push(s);
    }
    fields
}

fn align_cell(s: &str, width: usize, align: Align) -> String {
    match align {
        Align::Left | Align::LeftEndTab | Align::LeftFwf => format!("{s:<width$}"),
        Align::Right => format!("{s:>width$}"),
        Align::Center => {
            if width == 0 {
                return String::new();
            }
            let pad_total = width.saturating_sub(s.chars().count());
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
    pad: usize,
    chars: (char, char, char, char),
    color_enabled: bool,
) -> std::io::Result<()> {
    let parts = make_border_line(widths, pad, chars);
    let line = parts.join("");
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
    pad: usize,
    align: Align,
    header: bool,
    color_enabled: bool,
    theme: Theme,
) -> std::io::Result<()> {
    let mut line = String::new();
    line.push('│');
    for (i, field) in record.iter().enumerate() {
        let col_width = widths[i];
        let content_width = col_width.saturating_sub(2 * INNER_PAD);
        let text = condense_to_width(field, content_width);
        let aligned_inner = align_cell(&text, content_width, align);
        let mut cell = String::new();
        cell.push_str(&" ".repeat(INNER_PAD));
        cell.push_str(&aligned_inner);
        cell.push_str(&" ".repeat(INNER_PAD));
        let styled = colorize_field(&cell, header, i, theme, color_enabled);
        line.push_str(&styled);
        if i + 1 < record.len() {
            line.extend(std::iter::repeat(' ').take(pad));
            line.push('│');
        } else {
            line.push('│');
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

    let mut max_widths: Vec<usize> = Vec::new();
    for rec in &records {
        if rec.len() > max_widths.len() {
            max_widths.resize(rec.len(), 0);
        }
        for (idx, field) in rec.iter().enumerate() {
            let width = field_width(field).max(args.flag_width);
            if width > max_widths[idx] {
                max_widths[idx] = width;
            }
        }
    }

    let target_width = args.flag_max_width.or_else(|| {
        if stdout_is_tty {
            Some(textwrap::termwidth())
        } else {
            None
        }
    });

    let content_widths = if let Some(n) = args.flag_condense {
        Some(vec![n.max(args.flag_width); max_widths.len()])
    } else {
        autolayout(&max_widths, args.flag_pad, args.flag_width, target_width)
    };

    let content_widths = content_widths.as_deref().unwrap_or(&max_widths);
    let render_widths: Vec<usize> = content_widths
        .iter()
        .map(|w| w.saturating_add(2 * INNER_PAD))
        .collect();

    // Top border
    write_border_line(&mut out, &render_widths, args.flag_pad, BOX_TOP, color_enabled)?;

    // Write header (first record)
    write_row_string(
        &mut out,
        &records[0],
        &render_widths,
        args.flag_pad,
        args.flag_align,
        true,
        color_enabled,
        theme,
    )?;

    // Header separator
    write_border_line(&mut out, &render_widths, args.flag_pad, BOX_MID, color_enabled)?;

    for rec in records.iter().skip(1) {
        write_row_string(
            &mut out,
            rec,
            &render_widths,
            args.flag_pad,
            args.flag_align,
            false,
            color_enabled,
            theme,
        )?;
    }

    // Bottom border
    write_border_line(&mut out, &render_widths, args.flag_pad, BOX_BOT, color_enabled)?;

    out.flush()?;
    Ok(())
}
