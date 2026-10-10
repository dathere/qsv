pub(crate) static USAGE: &str = r#"
Concatenate CSV files by row or by column.

When concatenating by column, the columns will be written in the same order as
the inputs given. The number of rows in the result is always equivalent to
the minimum number of rows across all given CSV data. (This behavior can be
reversed with the '--pad' flag.)

Concatenating by rows can be done in two ways:

'rows' subcommand: 
   All CSV data must have the same number of columns (unless --flexible is enabled)
   and in the same order. 
   If you need to rearrange the columns or fix the lengths of records, use the
   'select' or 'fixlengths' commands. Also, only the headers of the *first* CSV
   data given are used. Headers in subsequent inputs are ignored. (This behavior
   can be disabled with --no-headers.)

'rowskey' subcommand:
   CSV data can have different numbers of columns and in different orders. All
   columns are written in insertion order. If a column is missing in a row, an
   empty field is written. If a column is missing in the header, an empty field
   is written for all rows.

Examples:

  # Concatenate CSV files by rows:
  qsv cat rows file1.csv file2.csv -o combined.csv

  # Concatenate CSV files by rows, adding a grouping column with the filename:
  qsv cat rowskey --group fname --group-name source_file file1.csv file2.csv -o combined_with_keys.csv

  # Concatenate CSV files by columns:
  qsv cat columns file1.csv file2.csv -o combined_columns.csv

  # Concatenate all CSV files in a directory by rows:
  qsv cat rows path/to/csv_directory -o combined.csv

  # Concatenate all CSV files listed in a .infile-list file by rows:
  qsv cat rows path/to/files_to_combine.infile-list -o combined.csv

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_cat.rs.
See also https://github.com/dathere/qsv/wiki/Transform-and-Reshape#cat

Usage:
    qsv cat rows    [options] [<input>...]
    qsv cat rowskey [options] [<input>...]
    qsv cat columns [options] [<input>...]
    qsv cat --help

cat arguments:
    <input>...              The CSV file(s) to read. Use '-' for standard input.
                            If input is a directory, all files in the directory will
                            be read as input.
                            If the input is a file with a '.infile-list' extension,
                            the file will be read as a list of input files.
                            If the input are snappy-compressed files(s), it will be
                            decompressed automatically.

cat options:
                             COLUMNS OPTION:
    -p, --pad                When concatenating columns, this flag will cause
                             all records to appear. It will pad each row if
                             other CSV data isn't long enough.

                             ROWS OPTION:
    --flexible               When concatenating rows, this flag turns off validation
                             that the input and output CSVs have the same number of columns.
                             This is faster, but may result in invalid CSV data.

                             ROWSKEY OPTIONS:
    -g, --group <grpkind>    When concatenating with rowskey, you can specify a grouping value
                             which will be used as the first column in the output. This is useful
                             when you want to know which file a row came from. Valid values are
                             'fullpath', 'parentdirfname', 'parentdirfstem', 'fname', 'fstem' and 'none'.
                             For a file at /data/2024/jan/sales.csv, 'fullpath' gives
                             '/data/2024/jan/sales.csv', 'parentdirfname' gives 'jan/sales.csv',
                             'parentdirfstem' gives 'jan/sales', 'fname' gives 'sales.csv' and
                             'fstem' gives 'sales'.
                             'fullpath' and the parentdir kinds use the file's real location
                             (symlinks and relative paths resolved), so the result does not
                             depend on how the path was typed. Use a parentdir kind to tell
                             apart same-named files in different directories.
                             For a compressed input, 'fullpath' names the real file
                             (/data/jan/sales.csv.sz, or /data/jan/2024.zip/sales.csv for an entry
                             sales.csv in a zip archive). The other kinds treat it as if unpacked
                             in place, so 'parentdirfname' gives jan/sales.csv and 2024/sales.csv.
                             Piped stdin is named 'stdin.csv' ('<stdin>' for 'fullpath').
                             A new column will be added to the beginning of each row using --group-name.
                             If 'none' is specified, no grouping column will be added.
                             [default: none]
    -N, --group-name <arg>   When concatenating with rowskey, this flag provides the name
                             for the new grouping column. [default: file]
                             
Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will NOT be interpreted
                           as column names. Note that this has no effect when
                           concatenating columns.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use indexmap::{IndexMap, IndexSet};
use serde::Deserialize;
use strum_macros::EnumString;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    util::{self, InputSource},
};

#[derive(Deserialize)]
struct Args {
    cmd_rows:        bool,
    cmd_rowskey:     bool,
    cmd_columns:     bool,
    flag_group:      String,
    flag_group_name: String,
    arg_input:       Vec<PathBuf>,
    /// where each `arg_input` path came from, after `process_input`
    #[serde(skip)]
    input_sources:   Vec<InputSource>,
    flag_pad:        bool,
    flag_flexible:   bool,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
}

#[derive(Debug, EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
enum GroupKind {
    FullPath,
    ParentDirFName,
    ParentDirFStem,
    FName,
    FStem,
    None,
}

/// `path` must already be resolved (see `InputSource::resolved_path`).
fn get_parentdir_and_file(path: &Path, stem_only: bool) -> String {
    let file_info = if stem_only {
        path.file_stem()
    } else {
        path.file_name()
    }
    .unwrap_or_default();

    // a file at the filesystem root has no parent directory name
    match path.parent().and_then(Path::file_name) {
        Some(parent_name) => Path::new(parent_name).join(file_info),
        None => PathBuf::from(file_info),
    }
    .to_string_lossy()
    .into_owned()
}

/// The `--group` value for an input, never its temp file. `fullpath` names the
/// real source (`InputSource::source_path`); the other kinds name an unpacked
/// input as if unpacked in place (`InputSource::logical_path`).
fn group_value(group_kind: &GroupKind, source: &InputSource) -> std::io::Result<String> {
    // stdin has no location: its parentdir kinds give a bare `stdin.csv`, as for
    // a file at the filesystem root
    let logical = source
        .logical_path()
        .unwrap_or_else(|| PathBuf::from("stdin.csv"));
    let name = |stem_only: bool| {
        if stem_only {
            logical.file_stem()
        } else {
            logical.file_name()
        }
        .map_or_else(String::new, |n| n.to_string_lossy().into_owned())
    };
    Ok(match group_kind {
        GroupKind::FullPath => source.source_path()?.map_or_else(
            || "<stdin>".to_string(),
            |p| p.to_string_lossy().into_owned(),
        ),
        GroupKind::ParentDirFName | GroupKind::ParentDirFStem => {
            let resolved = source.resolved_path()?.unwrap_or_else(|| logical.clone());
            get_parentdir_and_file(&resolved, *group_kind == GroupKind::ParentDirFStem)
        },
        GroupKind::FName => name(false),
        GroupKind::FStem => name(true),
        GroupKind::None => String::new(),
    })
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    let tmpdir = tempfile::tempdir()?;
    (args.arg_input, args.input_sources) =
        util::process_input_with_sources(args.arg_input, &tmpdir, "")?
            .into_iter()
            .unzip();
    if args.cmd_rows {
        args.cat_rows()
    } else if args.cmd_rowskey {
        args.cat_rowskey()
    } else if args.cmd_columns {
        args.cat_columns()
    } else {
        unreachable!();
    }
}

impl Args {
    #[inline]
    fn configs(&self) -> CliResult<Vec<Config>> {
        util::many_configs(
            &self.arg_input,
            self.flag_delimiter,
            self.flag_no_headers,
            // we can set flexible to true if we are using rowskey
            // as we don't need to validate that the number of columns
            // are the same across all files, increasing performance
            self.flag_flexible || self.cmd_rowskey,
        )
        .map_err(From::from)
    }

    fn cat_rows(&self) -> CliResult<()> {
        let mut row = csv::ByteRecord::new();
        let mut wtr = Config::new(self.flag_output.as_ref())
            .flexible(self.flag_flexible)
            .writer()?;

        // Readers are always flexible; without --flexible, record widths are
        // checked here instead, BEFORE each write. The csv writer only checks a
        // record's width after writing its fields, which left a half-written
        // record in the output, and neither it nor the reader names the file.
        let configs = self.configs()?.into_iter().map(|conf| conf.flexible(true));
        let mut width: Option<usize> = None;
        let mut have_headers = false;

        for (conf, source) in configs.zip(&self.input_sources) {
            let mut rdr = conf.reader()?;
            // A file's records must match its own header too, as a strict reader
            // would require, not just the output width: a later file whose
            // header is skipped must not pass with records that contradict it.
            let file_width = if self.flag_no_headers {
                None
            } else {
                Some(rdr.byte_headers()?.len())
            };
            if !have_headers {
                // The first non-empty file supplies the headers. An empty (0-byte)
                // file has none, so taking it as the source would write none, and
                // the next file's reader would then skip its header row.
                let headers = rdr.byte_headers()?;
                if headers.is_empty() {
                    continue;
                }
                have_headers = true;
                if !self.flag_no_headers {
                    width = Some(headers.len());
                    wtr.write_byte_record(headers)?;
                }
            }
            while rdr.read_byte_record(&mut row)? {
                if !self.flag_flexible {
                    let expected = match (file_width, width) {
                        (Some(fw), _) if fw != row.len() => Some((fw, "its header has")),
                        (_, Some(w)) if w != row.len() => Some((w, "the records before it have")),
                        (_, None) => {
                            width = Some(row.len());
                            None
                        },
                        _ => None,
                    };
                    if let Some((n, what)) = expected {
                        return fail_clierror!(
                            "`{}` line {}: found a record with {} fields, but {what} {n}. Use \
                             --flexible to allow records of different lengths, or fix the input \
                             with 'fixlengths' or 'select'.",
                            source.display_name(),
                            row.position().map_or(0, csv::Position::line),
                            row.len(),
                        );
                    }
                }
                wtr.write_byte_record(&row)?;
            }
        }

        Ok(wtr.flush()?)
    }

    // this algorithm is largely inspired by https://github.com/vi/csvcatrow by @vi
    // https://github.com/dathere/qsv/issues/527
    fn cat_rowskey(&self) -> CliResult<()> {
        // foldhash is a faster hasher than the default one used by IndexSet and IndexMap
        type FhashIndexSet<T> = IndexSet<T, foldhash::fast::RandomState>;
        type FhashIndexMap<T, T2> = IndexMap<T, T2, foldhash::fast::RandomState>;

        let Ok(group_kind) = GroupKind::from_str(&self.flag_group) else {
            return fail_incorrectusage_clierror!(
                "Invalid grouping value `{}`. Valid values are 'fullpath', 'parentdirfname', \
                 'parentdirfstem', 'fname', 'fstem' and 'none'.",
                self.flag_group
            );
        };
        let group_flag = group_kind != GroupKind::None;

        let mut columns_global: FhashIndexSet<Box<[u8]>> = FhashIndexSet::default();

        if group_flag {
            columns_global.insert(self.flag_group_name.as_bytes().to_vec().into_boxed_slice());
        }

        // synthetic headers per file when --no-headers is set; we keep a Vec
        // so the second pass can re-use the widths found by the first pass's scan.
        let configs = self.configs()?;
        let mut synthetic_headers: Vec<csv::ByteRecord> = if self.flag_no_headers {
            Vec::with_capacity(configs.len())
        } else {
            Vec::new()
        };

        // First pass: collect the global column set in insertion order.
        let group_name_bytes = self.flag_group_name.as_bytes();
        for (conf, source) in configs.iter().zip(&self.input_sources) {
            let path_display = source.display_name();
            let mut rdr = conf.reader()?;

            if self.flag_no_headers {
                // synthesize "_c_1", "_c_2", ... from the width of this file's WIDEST row.
                // rowskey reads flexibly, so a later row can be wider than the first;
                // sizing from the first row alone silently dropped the extra fields.
                let mut rec = csv::ByteRecord::new();
                let mut width = 0;
                while rdr.read_byte_record(&mut rec)? {
                    width = width.max(rec.len());
                }
                let mut th = csv::ByteRecord::with_capacity(64, width);
                for n in 0..width {
                    th.push_field(format!("_c_{}", n + 1).as_bytes());
                }
                for field in &th {
                    columns_global.insert(field.to_vec().into_boxed_slice());
                    if group_flag && field == group_name_bytes {
                        wwarn!(
                            "Synthetic column `{}` in file `{}` collides with --group-name; the \
                             file's value will override the grouping value for its rows.",
                            self.flag_group_name,
                            path_display,
                        );
                    }
                }
                synthetic_headers.push(th);
            } else {
                let header = rdr.byte_headers()?;
                for field in header {
                    columns_global.insert(field.to_vec().into_boxed_slice());
                    if group_flag && field == group_name_bytes {
                        wwarn!(
                            "Column `{}` in file `{}` collides with --group-name; the file's \
                             value will override the grouping value for its rows.",
                            self.flag_group_name,
                            path_display,
                        );
                    }
                }
            }
        }
        let num_columns_global = columns_global.len();

        // Second pass: write rows, projecting each file's columns onto the global schema.
        // The writer is flexible: we already know every column appears in columns_global,
        // so we can skip the per-row column-count validation.
        let mut wtr = Config::new(self.flag_output.as_ref())
            .flexible(true)
            .writer()?;
        let mut new_row = csv::ByteRecord::with_capacity(4096, num_columns_global);

        if !self.flag_no_headers {
            for c in &columns_global {
                new_row.push_field(c);
            }
            wtr.write_byte_record(&new_row)?;
        }

        // amortize allocations across files
        let mut columns_of_this_file: FhashIndexMap<Box<[u8]>, usize> = FhashIndexMap::default();
        columns_of_this_file.reserve(num_columns_global);
        let mut col_map: Vec<Option<usize>> = Vec::with_capacity(num_columns_global);
        let mut row = csv::ByteRecord::with_capacity(4096, num_columns_global);

        for (file_idx, (conf, source)) in configs.into_iter().zip(&self.input_sources).enumerate() {
            let mut rdr = conf.reader()?;

            // Build columns_of_this_file from either the synthesized header
            // (no-headers) or the file's actual header.
            columns_of_this_file.clear();
            if self.flag_no_headers {
                // safety: built in the first pass, one entry per file in order.
                let th = &synthetic_headers[file_idx];
                for (n, field) in th.iter().enumerate() {
                    columns_of_this_file.insert(field.to_vec().into_boxed_slice(), n);
                }
            } else {
                let header = rdr.byte_headers()?;
                for (n, field) in header.iter().enumerate() {
                    let fi = field.to_vec().into_boxed_slice();
                    if let indexmap::map::Entry::Vacant(entry) = columns_of_this_file.entry(fi) {
                        entry.insert(n);
                    } else {
                        wwarn!(
                            "Duplicate column `{}` name in file `{}`.",
                            util::bytes_to_cow_str(field),
                            source.display_name(),
                        );
                    }
                }
            }

            // Precompute the global -> file column index mapping once per file.
            // Hot loop below is then a flat Vec walk: no per-cell hashmap probes.
            col_map.clear();
            col_map.extend(
                columns_global
                    .iter()
                    .map(|c| columns_of_this_file.get(c).copied()),
            );

            // canonicalize() can fail (broken symlink, perms); propagate instead of panic.
            let grouping_value = group_value(&group_kind, source)?;
            let grouping_value_bytes = grouping_value.as_bytes();

            while rdr.read_byte_record(&mut row)? {
                new_row.clear();
                for (col_idx, slot) in col_map.iter().enumerate() {
                    match slot {
                        Some(idx) => new_row.push_field(row.get(*idx).unwrap_or(b"")),
                        None if group_flag && col_idx == 0 => {
                            new_row.push_field(grouping_value_bytes);
                        },
                        None => new_row.push_field(b""),
                    }
                }
                wtr.write_byte_record(&new_row)?;
            }
        }

        wtr.flush()?;
        Ok(())
    }

    fn cat_columns(&self) -> CliResult<()> {
        let mut wtr = Config::new(self.flag_output.as_ref()).writer()?;
        let mut rdrs = self
            .configs()?
            .into_iter()
            .map(|conf| conf.no_headers(true).reader())
            .collect::<Result<Vec<_>, _>>()?;

        // Find the lengths of each record. If a length varies, then an error
        // will occur so we can rely on the first length being the correct one.
        let mut lengths = vec![];
        for rdr in &mut rdrs {
            lengths.push(rdr.byte_headers()?.len());
        }

        let mut iters = rdrs
            .iter_mut()
            .map(csv::Reader::byte_records)
            .collect::<Vec<_>>();

        // safety: there's always a first element
        let mut record = csv::ByteRecord::with_capacity(1024, *lengths.first().unwrap());

        'OUTER: loop {
            record.clear();
            let mut num_done = 0;
            for (iter, &len) in iters.iter_mut().zip(lengths.iter()) {
                match iter.next() {
                    None => {
                        num_done += 1;
                        if self.flag_pad {
                            for _ in 0..len {
                                record.push_field(b"");
                            }
                        } else {
                            break 'OUTER;
                        }
                    },
                    Some(Ok(next)) => record.extend(&next),
                    Some(Err(err)) => return fail!(err),
                }
            }
            // Only needed when `--pad` is set.
            // When not set, the OUTER loop breaks when the shortest iterator
            // is exhausted.
            if num_done >= iters.len() {
                break 'OUTER;
            }
            wtr.write_byte_record(&record)?;
        }
        Ok(wtr.flush()?)
    }
}
