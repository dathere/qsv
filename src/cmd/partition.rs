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

use std::{collections::HashSet, fs, io, path::Path};

use foldhash::{HashMap, HashMapExt};
use regex::Regex;
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
            .no_headers(self.flag_no_headers)
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
        let mut r#gen = WriterGenerator::new(self.flag_filename.clone());

        // default to 256 if no limit is set or sysinfo cannot get the limit
        let sys_limit = System::open_files_limit().unwrap_or(256);

        // If no limit is specified, get the system limit and set the limit to 90% of it
        if let Some(limit) = self.flag_limit {
            if limit == 0 {
                return self.process_all_data(&mut rdr, &headers, key_col, &mut r#gen);
            }

            if limit > sys_limit {
                return fail_incorrectusage_clierror!(
                    "Limit is greater than system limit ({limit} > {sys_limit})"
                );
            }
        } else {
            let auto_limit = sys_limit * 90 / 100;
            log::info!(
                "Auto-setting limit to {auto_limit} based on system limit with 10% safety margin"
            );
            self.flag_limit = Some(auto_limit);
        }

        // Process data in batches to respect the file limit
        if self.flag_limit.is_some() && self.flag_limit.unwrap() != 0 {
            return self.process_in_batches(&mut rdr, &headers, key_col, &mut r#gen);
        }

        // Otherwise, process all data at once
        self.process_all_data(&mut rdr, &headers, key_col, &mut r#gen)
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
    fn process_in_batches(
        &self,
        rdr: &mut csv::Reader<Box<dyn io::Read + Send>>,
        headers: &csv::ByteRecord,
        key_col: usize,
        r#gen: &mut WriterGenerator,
    ) -> CliResult<()> {
        let limit = self.flag_limit.unwrap();
        let mut writers: HashMap<Vec<u8>, BoxedWriter> = HashMap::new();
        let mut row = csv::ByteRecord::new();
        let mut pending_rows: Vec<(csv::ByteRecord, Vec<u8>)> = Vec::new();

        // First pass: collect all rows and group them by key
        while rdr.read_byte_record(&mut row)? {
            let column = &row[key_col];
            let key = match self.flag_prefix_length {
                Some(len) if len < column.len() => &column[0..len],
                _ => column,
            };
            let key_vec = key.to_vec();
            pending_rows.push((row.clone(), key_vec));
        }

        // Process rows in batches
        let mut processed_keys: HashSet<Vec<u8>> = HashSet::new();
        let mut current_batch: Vec<(csv::ByteRecord, Vec<u8>)> = Vec::new();

        for (row, key_vec) in pending_rows {
            // If we've reached the limit and this is a new key, process the current batch
            if writers.len() >= limit
                && !writers.contains_key(&key_vec)
                && !current_batch.is_empty()
            {
                self.process_batch(&mut writers, &current_batch, key_col, headers, r#gen)?;
                processed_keys.extend(writers.keys().cloned());
                self.flush_and_close_writers(&mut writers)?;
                writers.clear();
                current_batch.clear();
            }

            current_batch.push((row, key_vec));
        }

        // Process the final batch
        if !current_batch.is_empty() {
            self.process_batch(&mut writers, &current_batch, key_col, headers, r#gen)?;
        }

        // Final flush of remaining writers
        for (_, mut writer) in writers {
            writer.flush()?;
        }
        Ok(())
    }

    /// Process a batch of rows for the same keys.
    fn process_batch(
        &self,
        writers: &mut HashMap<Vec<u8>, BoxedWriter>,
        batch: &[(csv::ByteRecord, Vec<u8>)],
        key_col: usize,
        headers: &csv::ByteRecord,
        r#gen: &mut WriterGenerator,
    ) -> CliResult<()> {
        for (row, _key_vec) in batch {
            self.process_row(writers, row, key_col, headers, r#gen)?;
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
        r#gen: &mut WriterGenerator,
    ) -> CliResult<()> {
        let column = &row[key_col];
        let key = match self.flag_prefix_length {
            Some(len) if len < column.len() => &column[0..len],
            _ => column,
        };
        let key_vec = key.to_vec();

        let wtr = if let Some(writer) = writers.get_mut(&key_vec) {
            writer
        } else {
            // We have a new key, so make a new writer.
            let mut wtr = r#gen.writer(&*self.arg_outdir, key)?;
            if !self.flag_no_headers {
                if self.flag_drop {
                    wtr.write_record(
                        headers
                            .iter()
                            .enumerate()
                            .filter_map(|(i, e)| if i == key_col { None } else { Some(e) }),
                    )?;
                } else {
                    wtr.write_record(&*headers)?;
                }
            }
            writers.insert(key_vec.clone(), wtr);
            // safety: we just inserted the key into the map, so it must be present
            unsafe { writers.get_mut(&key_vec).unwrap_unchecked() }
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

    /// Flush and close all writers in the current batch.
    fn flush_and_close_writers(
        &self,
        writers: &mut HashMap<Vec<u8>, BoxedWriter>,
    ) -> CliResult<()> {
        for (_, mut writer) in writers.drain() {
            writer.flush()?;
        }
        Ok(())
    }
}

type BoxedWriter = csv::Writer<Box<dyn io::Write + 'static>>;

/// Generates unique filenames based on CSV values.
struct WriterGenerator {
    template:      FilenameTemplate,
    counter:       usize,
    used:          HashSet<String>,
    non_word_char: Regex,
}

impl WriterGenerator {
    fn new(template: FilenameTemplate) -> WriterGenerator {
        WriterGenerator {
            template,
            counter: 1,
            used: HashSet::new(),
            non_word_char: regex_oncelock!(r"\W").clone(),
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
        let safe = self
            .non_word_char
            .replace_all(&String::from_utf8_lossy(key), "")
            .into_owned();
        let base = if safe.is_empty() {
            "empty".to_owned()
        } else {
            safe
        };

        // Check for both exact and case-insensitive collisions
        // to ensure uniqueness on case-insensitive file systems
        let base_lower = base.to_lowercase();
        let has_collision = self.used.contains(&base)
            || self
                .used
                .iter()
                .any(|used| used.to_lowercase() == base_lower);

        if has_collision {
            loop {
                let candidate = format!("{}_{}", &base, self.counter);
                let candidate_lower = candidate.to_lowercase();
                self.counter = self.counter.checked_add(1).unwrap_or_else(|| {
                    // We'll run out of other things long before we ever
                    // reach this, but we'll check just for correctness and
                    // completeness.
                    panic!("Cannot generate unique value")
                });

                // Check for both exact and case-insensitive collisions
                let candidate_has_collision = self.used.contains(&candidate)
                    || self
                        .used
                        .iter()
                        .any(|used| used.to_lowercase() == candidate_lower);

                if !candidate_has_collision {
                    self.used.insert(candidate.clone());
                    return candidate;
                }
            }
        } else {
            self.used.insert(base.clone());
            base
        }
    }
}
