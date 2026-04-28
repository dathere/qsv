static USAGE: &str = r#"
Partitions the given CSV data into chunks based on the value of a column.

See `split` command to split a CSV data by row count, by number of chunks or
by kb-size.

The files are written to the output directory with filenames based on the
values in the partition column and the `--filename` flag.

Note: To account for case-insensitive file system collisions (e.g. macOS APFS
and Windows NTFS), the command will add a number suffix to the filename if the
value is already in use.

EXAMPLE:

Partition nyc311.csv file into separate files based on the value of the
"Borough" column in the current directory:
  $ qsv partition Borough . --filename "nyc311-{}.csv" nyc311.csv

will create the following files, each containing the data for each borough:
    nyc311-Bronx.csv
    nyc311-Brooklyn.csv
    nyc311-Manhattan.csv
    nyc311-Queens.csv
    nyc311-Staten_Island.csv

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_partition.rs.

Usage:
    qsv partition [options] <column> <outdir> [<input>]
    qsv partition --help

partition arguments:
    <column>                 The column to use as a key for partitioning.
                             You can use the `--select` option to select
                             the column by name or index, but only one
                             column can be used for partitioning.
                             See `select` command for more details.
    <outdir>                 The directory to write the output files to.
    <input>                  The CSV file to read from. If not specified, then
                             the input will be read from stdin.

partition options:
    --filename <filename>    A filename template to use when constructing the
                             names of the output files.  The string '{}' will
                             be replaced by a value based on the partition column,
                             but sanitized for shell safety.
                             [default: {}.csv]
    -p, --prefix-length <n>  Truncate the partition column after the
                             specified number of bytes when creating the
                             output file.
    --drop                   Drop the partition column from results.
    --limit <n>              Limit the number of simultaneously open files.
                             Useful for partitioning large datasets with many
                             unique values to avoid "too many open files" errors.
                             Data is processed in batches until all unique values
                             are processed.
                             If not set, it will be automatically set to the
                             system limit with a 10% safety margin.
                             If set to 0, it will process all data at once,
                             regardless of the system's open files limit.

Common options:
    -h, --help               Display this message
    -n, --no-headers         When set, the first row will NOT be interpreted
                             as column names. Otherwise, the first row will
                             appear in all chunks as the header row.
    -d, --delimiter <arg>    The field delimiter for reading CSV data.
                             Must be a single character. (default: ,)
"#;

use std::{collections::hash_map::Entry, fs, io, path::Path};

use foldhash::{HashMap, HashMapExt, HashSet, HashSetExt};
use serde::Deserialize;
use sysinfo::System;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    regex_oncelock,
    select::SelectColumns,
    util::{self, FilenameTemplate},
};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Deserialize)]
struct Args {
    arg_column:         SelectColumns,
    arg_input:          Option<String>,
    arg_outdir:         String,
    flag_filename:      FilenameTemplate,
    flag_prefix_length: Option<usize>,
    flag_drop:          bool,
    flag_no_headers:    bool,
    flag_delimiter:     Option<Delimiter>,
    flag_limit:         Option<usize>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    // if no input file is provided, use stdin and save to a temp file
    if args.arg_input.is_none() {
        // Get or initialize temp directory that persists until program exit
        let temp_dir =
            crate::config::TEMP_FILE_DIR.get_or_init(|| tempfile::TempDir::new().unwrap().keep());

        // Create a temporary file with .csv extension to store stdin input
        let mut temp_file = tempfile::Builder::new()
            .suffix(".csv")
            .tempfile_in(temp_dir)?;
        io::copy(&mut io::stdin(), &mut temp_file)?;

        // Keep temp file from being deleted when it goes out of scope;
        // it will be deleted when the program exits when TEMP_FILE_DIR is cleaned up.
        let (_, temp_path) = temp_file
            .keep()
            .map_err(|e| format!("Failed to keep temporary stdin file: {e}"))?;

        // Round-tripping through `String` requires UTF-8; bail out cleanly on
        // exotic temp paths instead of panicking or silently corrupting via
        // lossy conversion.
        let temp_path_str = temp_path.to_str().ok_or_else(|| {
            format!(
                "Temporary stdin file path is not valid UTF-8: {}",
                temp_path.display()
            )
        })?;
        args.arg_input = Some(temp_path_str.to_owned());
    }

    fs::create_dir_all(&args.arg_outdir)?;

    // It would be nice to support efficient parallel partitions, but doing
    // so would involve more complicated inter-thread communication, with
    // multiple readers and writers, and some way of passing buffers
    // between them.
    args.sequential_partition()
}

impl Args {
    /// Configuration for our reader.
    fn rconfig(&self) -> Config {
        Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers_flag(self.flag_no_headers)
            .select(self.arg_column.clone())
    }

    /// Get the column to use as a key.
    #[allow(clippy::unused_self)]
    fn key_column(&self, rconfig: &Config, headers: &csv::ByteRecord) -> CliResult<usize> {
        let select_cols = rconfig.selection(headers)?;
        if select_cols.len() == 1 {
            Ok(select_cols[0])
        } else {
            fail!("can only partition on one column")
        }
    }

    /// A basic sequential partition with optional batching for file limit.
    fn sequential_partition(&mut self) -> CliResult<()> {
        let rconfig = self.rconfig();
        let mut rdr = rconfig.reader()?;
        let headers = rdr.byte_headers()?.clone();
        let key_col = self.key_column(&rconfig, &headers)?;
        let mut writer_gen = WriterGenerator::new(self.flag_filename.clone());

        // default to 256 if no limit is set or sysinfo cannot get the limit
        let sys_limit = System::open_files_limit().unwrap_or(256);

        let limit = match self.flag_limit {
            Some(0) => {
                return self.process_all_data(&mut rdr, &headers, key_col, &mut writer_gen);
            },
            Some(limit) if limit > sys_limit => {
                return fail_incorrectusage_clierror!(
                    "Limit is greater than system limit ({limit} > {sys_limit})"
                );
            },
            Some(limit) => limit,
            None => {
                // 90% of the system limit with 10% safety margin
                let auto_limit = (sys_limit * 9) / 10;
                if auto_limit == 0 {
                    // Pathologically small `sys_limit` (e.g. <10) — `chunks(0)`
                    // would panic, so fall back to processing everything at once.
                    log::info!(
                        "System open-file limit too small ({sys_limit}); processing all data at \
                         once"
                    );
                    return self.process_all_data(&mut rdr, &headers, key_col, &mut writer_gen);
                }
                log::info!(
                    "Auto-setting limit to {auto_limit} based on system limit with 10% safety \
                     margin"
                );
                auto_limit
            },
        };

        self.process_in_batches(limit, &headers, key_col, &mut writer_gen)
    }

    /// Compute the partition key for a column value, applying `--prefix-length`.
    fn key_for<'a>(&self, column: &'a [u8]) -> &'a [u8] {
        match self.flag_prefix_length {
            Some(len) if len < column.len() => &column[0..len],
            _ => column,
        }
    }

    /// Process all data at once (original behavior when no limit is specified).
    fn process_all_data(
        &self,
        rdr: &mut csv::Reader<Box<dyn io::Read + Send>>,
        headers: &csv::ByteRecord,
        key_col: usize,
        r#gen: &mut WriterGenerator,
    ) -> CliResult<()> {
        let mut writers: HashMap<Vec<u8>, BoxedWriter> = HashMap::new();
        let mut row = csv::ByteRecord::new();

        while rdr.read_byte_record(&mut row)? {
            self.process_row(&mut writers, &row, key_col, headers, r#gen)?;
        }

        // Final flush of all writers
        for (_, mut writer) in writers {
            writer.flush()?;
        }
        Ok(())
    }

    /// Process data in batches to respect the file limit.
    /// First pass collects all unique keys; then for each batch of up to
    /// `limit` keys we re-scan the file and write only rows in that batch.
    fn process_in_batches(
        &self,
        limit: usize,
        headers: &csv::ByteRecord,
        key_col: usize,
        writer_gen: &mut WriterGenerator,
    ) -> CliResult<()> {
        // First pass: collect all unique keys
        let mut unique_keys: HashSet<Vec<u8>> = HashSet::new();
        let mut row = csv::ByteRecord::new();

        let mut rdr = self.rconfig().reader()?;
        let _ = rdr.byte_headers()?; // Skip headers

        while rdr.read_byte_record(&mut row)? {
            unique_keys.insert(self.key_for(&row[key_col]).to_vec());
        }

        // Convert to sorted vector for consistent processing
        let mut sorted_keys: Vec<_> = unique_keys.into_iter().collect();
        sorted_keys.sort_unstable();

        // Process in batches that don't exceed the limit
        for chunk in sorted_keys.chunks(limit) {
            let chunk_set: HashSet<&[u8]> = chunk.iter().map(Vec::as_slice).collect();
            let mut writers: HashMap<Vec<u8>, BoxedWriter> = HashMap::with_capacity(chunk.len());

            let mut rdr = self.rconfig().reader()?;
            let _ = rdr.byte_headers()?; // Skip headers

            while rdr.read_byte_record(&mut row)? {
                if chunk_set.contains(self.key_for(&row[key_col])) {
                    self.process_row(&mut writers, &row, key_col, headers, writer_gen)?;
                }
            }

            // Flush all writers in this batch
            for (_, mut writer) in writers {
                writer.flush()?;
            }
        }

        Ok(())
    }

    /// Process a single row and write it to the appropriate writer.
    fn process_row(
        &self,
        writers: &mut HashMap<Vec<u8>, BoxedWriter>,
        row: &csv::ByteRecord,
        key_col: usize,
        headers: &csv::ByteRecord,
        writer_gen: &mut WriterGenerator,
    ) -> CliResult<()> {
        let key = self.key_for(&row[key_col]);

        let wtr = match writers.entry(key.to_vec()) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => {
                // We have a new key, so make a new writer.
                let mut wtr = writer_gen.writer(&*self.arg_outdir, key)?;
                if !self.flag_no_headers {
                    if self.flag_drop {
                        wtr.write_record(
                            headers
                                .iter()
                                .enumerate()
                                .filter_map(|(i, h)| if i == key_col { None } else { Some(h) }),
                        )?;
                    } else {
                        wtr.write_record(headers)?;
                    }
                }
                e.insert(wtr)
            },
        };

        if self.flag_drop {
            wtr.write_record(
                row.iter().enumerate().filter_map(
                    |(i, e)| {
                        if i == key_col { None } else { Some(e) }
                    },
                ),
            )?;
        } else {
            wtr.write_byte_record(row)?;
        }
        Ok(())
    }
}

type BoxedWriter = csv::Writer<Box<dyn io::Write + 'static>>;

/// Generates unique filenames based on CSV values.
struct WriterGenerator {
    template: FilenameTemplate,
    counter:  usize,
    /// Lowercased forms of every name handed out, so collision checks are O(1)
    /// and cover both exact and case-insensitive (APFS/NTFS) clashes.
    used_ci:  HashSet<String>,
}

impl WriterGenerator {
    fn new(template: FilenameTemplate) -> WriterGenerator {
        WriterGenerator {
            template,
            counter: 1,
            used_ci: HashSet::new(),
        }
    }

    /// Create a CSV writer for `key`.  Does not add headers.
    fn writer<P>(&mut self, path: P, key: &[u8]) -> io::Result<BoxedWriter>
    where
        P: AsRef<Path>,
    {
        let unique_value = self.unique_value(key);
        self.template.writer(path.as_ref(), &unique_value)
    }

    /// Generate a unique value for `key`, suitable for use in a
    /// "shell-safe" filename.  If you pass `key` twice, you'll get two
    /// different values. Also handles case-insensitive file system collisions.
    fn unique_value(&mut self, key: &[u8]) -> String {
        // Sanitize our key.
        let safe = regex_oncelock!(r"\W")
            .replace_all(&String::from_utf8_lossy(key), "")
            .into_owned();
        let base = if safe.is_empty() {
            "empty".to_owned()
        } else {
            safe
        };

        if self.used_ci.insert(base.to_lowercase()) {
            return base;
        }

        loop {
            let candidate = format!("{}_{}", &base, self.counter);
            // We'll run out of other things long before we ever
            // get a panic with strict_add
            self.counter = self.counter.strict_add(1);

            if self.used_ci.insert(candidate.to_lowercase()) {
                return candidate;
            }
        }
    }
}
