static USAGE: &str = r#"
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
    util,
};

#[derive(Deserialize)]
struct Args {
    cmd_rows:        bool,
    cmd_rowskey:     bool,
    cmd_columns:     bool,
    flag_group:      String,
    flag_group_name: String,
    arg_input:       Vec<PathBuf>,
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

fn get_parentdir_and_file(path: &Path, stem_only: bool) -> String {
    //safety: we know that this is a valid pathbuf
    let file_info = if stem_only {
        path.file_stem()
    } else {
        path.file_name()
    }
    .unwrap();

    let parent_dir = path.parent().unwrap();

    parent_dir.join(file_info).to_string_lossy().into_owned()
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    let tmpdir = tempfile::tempdir()?;
    args.arg_input = util::process_input(args.arg_input, &tmpdir, "")?;
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
        let mut rdr;

        let mut configs = self.configs()?.into_iter();

        // the first file is special, as it has the headers
        // if --no-headers is set, we just write the first file
        if let Some(conf) = configs.next() {
            rdr = conf.reader()?;
            conf.write_headers(&mut rdr, &mut wtr)?;
            while rdr.read_byte_record(&mut row)? {
                wtr.write_byte_record(&row)?;
            }
        }

        // the rest of the files are just written
        // as fast as possible, as we don't need to
        // worry about headers
        for conf in configs {
            rdr = conf.reader()?;
            while rdr.read_byte_record(&mut row)? {
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

        // stdin is already materialized to a real file by util::process_input()
        // before we get here, so all configs have a Some(path).

        let mut columns_global: FhashIndexSet<Box<[u8]>> = FhashIndexSet::default();

        if group_flag {
            columns_global.insert(self.flag_group_name.as_bytes().to_vec().into_boxed_slice());
        }

        // synthetic headers per file when --no-headers is set; we keep a Vec
        // so the second pass can re-use the exact widths discovered in the
        // first pass (re-scanning the file is O(rows) and we already scanned).
        let configs = self.configs()?;
        let mut synthetic_headers: Vec<csv::ByteRecord> = if self.flag_no_headers {
            Vec::with_capacity(configs.len())
        } else {
            Vec::new()
        };

        // First pass: collect the global column set in insertion order.
        for conf in &configs {
            let mut rdr = conf.reader()?;

            if self.flag_no_headers {
                // synthesize "_c_1", "_c_2", ... from the width of this file's first row.
                let mut first = csv::ByteRecord::new();
                rdr.read_byte_record(&mut first)?;
                let mut th = csv::ByteRecord::with_capacity(64, first.len());
                for n in 0..first.len() {
                    th.push_field(format!("_c_{}", n + 1).as_bytes());
                }
                for field in &th {
                    columns_global.insert(field.to_vec().into_boxed_slice());
                }
                synthetic_headers.push(th);
            } else {
                let header = rdr.byte_headers()?;
                for field in header {
                    columns_global.insert(field.to_vec().into_boxed_slice());
                    if group_flag && field == self.flag_group_name.as_bytes() {
                        wwarn!(
                            "Column `{}` in file `{:?}` collides with --group-name; the file's \
                             value will override the grouping value for its rows.",
                            self.flag_group_name,
                            conf.path,
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
        let mut grouping_value = String::new();
        let mut columns_of_this_file: FhashIndexMap<Box<[u8]>, usize> = FhashIndexMap::default();
        columns_of_this_file.reserve(num_columns_global);
        let mut col_map: Vec<Option<usize>> = Vec::with_capacity(num_columns_global);
        let mut row = csv::ByteRecord::with_capacity(4096, num_columns_global);

        for (file_idx, conf) in configs.into_iter().enumerate() {
            let conf_pathbuf = conf.path.clone().ok_or_else(|| {
                crate::CliError::Other("cat rowskey: input is missing a file path".to_string())
            })?;
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
                            "Duplicate column `{}` name in file `{:?}`.",
                            String::from_utf8_lossy(field),
                            conf.path,
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

            // set grouping_value
            // canonicalize() can fail (broken symlink, perms); propagate instead of panic.
            match group_kind {
                GroupKind::FullPath => {
                    grouping_value.clear();
                    grouping_value.push_str(&conf_pathbuf.canonicalize()?.to_string_lossy());
                },
                GroupKind::ParentDirFName => {
                    grouping_value = get_parentdir_and_file(&conf_pathbuf, false);
                },
                GroupKind::ParentDirFStem => {
                    grouping_value = get_parentdir_and_file(&conf_pathbuf, true);
                },
                GroupKind::FName => {
                    grouping_value.clear();
                    if let Some(name) = conf_pathbuf.file_name() {
                        grouping_value.push_str(&name.to_string_lossy());
                    }
                },
                GroupKind::FStem => {
                    grouping_value.clear();
                    if let Some(stem) = conf_pathbuf.file_stem() {
                        grouping_value.push_str(&stem.to_string_lossy());
                    }
                },
                GroupKind::None => {},
            }
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
