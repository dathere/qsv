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
                           [default: 2]
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

use std::{borrow::Cow, io::IsTerminal};

use qsv_tabwriter::{Alignment, TabWriter};
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

impl From<Align> for Alignment {
    fn from(align: Align) -> Self {
        match align {
            Align::Left => Alignment::Left,
            Align::Right => Alignment::Right,
            Align::Center => Alignment::Center,
            Align::LeftEndTab => Alignment::LeftEndTab,
            Align::LeftFwf => Alignment::LeftFwf,
        }
    }
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

    // Available width after accounting for small safety fudge and padding.
    let cols = max_widths.len();
    let pad_total = pad.saturating_mul(cols.saturating_sub(1));
    let available = screen_width.saturating_sub(FUDGE).saturating_sub(pad_total);

    // Width of data as-is.
    let data_width: usize = max_widths.iter().sum();
    if available >= data_width {
        return None;
    }

    // Lower bound per column tries to give every column a chance.
    let lower_bound = (available / cols).clamp(2, 10).max(min_width);

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

    Some(widths)
}

fn write_row<W: std::io::Write>(
    wtr: &mut csv::Writer<W>,
    record: &csv::ByteRecord,
    condense: Option<&[usize]>,
    header: bool,
    color_enabled: bool,
    theme: Theme,
) -> csv::Result<()> {
    if !color_enabled {
        return wtr.write_record(record.iter().enumerate().map(|(col_idx, f)| {
            let per_col = condense.map(|v| {
                let width = v[col_idx];
                if width > 3 { width - 3 } else { width }
            });
            util::condense(Cow::Borrowed(f), per_col)
        }));
    }

    let mut styled = Vec::with_capacity(record.len());
    for (col_idx, field) in record.iter().enumerate() {
        let per_col = condense.map(|v| {
            let width = v[col_idx];
            if width > 3 { width - 3 } else { width }
        });
        let condensed = util::condense(Cow::Borrowed(field), per_col);
        styled.push(style_field(condensed.as_ref(), header, col_idx, theme));
    }
    wtr.write_record(styled.iter())
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

    let tw = TabWriter::new(wconfig.io_writer()?)
        .minwidth(args.flag_width)
        .padding(args.flag_pad)
        .alignment(args.flag_align.into())
        .ansi(color_enabled);
    let mut wtr = wconfig.from_writer(tw);
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

    let condense_vec = if let Some(n) = args.flag_condense {
        Some(vec![n.max(args.flag_width); max_widths.len()])
    } else {
        autolayout(&max_widths, args.flag_pad, args.flag_width, target_width)
    };

    let condense_ref = condense_vec.as_deref();

    // Write header (first record)
    write_row(
        &mut wtr,
        &records[0],
        condense_ref,
        true,
        color_enabled,
        theme,
    )?;

    if color_enabled {
        let widths_for_sep = condense_ref.unwrap_or(&max_widths);
        let sep_record: Vec<String> = widths_for_sep
            .iter()
            .map(|w| {
                let sep_len = (*w).max(3);
                let sep_str = "─".repeat(sep_len);
                format!("\u{001b}[38;5;245m{sep_str}\u{001b}[0m")
            })
            .collect();
        wtr.write_record(sep_record.iter())?;
    }

    for rec in records.iter().skip(1) {
        write_row(&mut wtr, rec, condense_ref, false, color_enabled, theme)?;
    }

    Ok(wtr.flush()?)
}
