static USAGE: &str = r#"
Splits the given CSV data into chunks. It has three modes: by size (rowcount),
by number of chunks and by kb-size.

See `partition` command for splitting by a column value.

When splitting by size, the CSV data is split into chunks of the given number of
rows. The last chunk may have fewer rows if the number of records is not evenly
divisible by the given rowcount.

When splitting by number of chunks, the CSV data is split into the given number of
chunks. The number of rows in each chunk is determined by the number of records in
the CSV data and the number of desired chunks. If the number of records is not evenly
divisible by the number of chunks, the last chunk will have fewer records.

When splitting by kb-size, the CSV data is split into chunks of the given size in kilobytes.
The number of rows in each chunk may vary, but the size of each chunk will not exceed the
desired size.

Uses multithreading to go faster if the CSV has an index when splitting by size or
by number of chunks. Splitting by kb-size is always done sequentially with a single thread.

The default is to split by size with a chunk size of 500.

The files are written to the directory given with the name '{start}.csv',
where {start} is the index of the first record of the chunk (starting at 0).

Examples:
  # Create files with names like chunk_0.csv, chunk_100.csv, etc.
  # in the directory 'outdir', creating the directory if it does not exist.
  qsv split outdir --size 100 --filename chunk_{}.csv input.csv

  # Create files with names like chunk_00000.csv, chunk_00100.csv, etc.
  # in the directory 'outdir/subdir', creating the directories if they do not exist.
  qsv split outdir/subdir -s 100 --filename chunk_{}.csv --pad 5 input.csv

  # Create files like 0.csv, 100.csv, etc. in the current directory.
  qsv split . -s 100 input.csv

  # Create files with names like 0.csv, 994.csv, etc. in the directory
  # 'outdir', creating the directory if it does not exist. Each file will be close
  # to 1000KB in size.
  qsv split outdir --kb-size 1000 input.csv

  # Read from stdin and create files like 0.csv, 1000.csv, etc. in the directory
  # 'mysplitoutput', creating it if it does not exist.
  cat in.csv | qsv split mysplitoutput -s 1000

  # Split into 10 chunks. Files are named with the zero-based starting row index
  # of each chunk (e.g. 0.csv, N.csv, 2N.csv, ...) in the directory 'outdir'.
  qsv split outdir --chunks 10 input.csv

  # Same, using 4 parallel jobs. Note that the input CSV must have an index.
  qsv split splitoutdir -c 10 -j 4 input.csv

  # This will create files with names like 0.csv, 100.csv, etc. in the directory
  # 'outdir', and then run the command "gzip" on each chunk.
  qsv split outdir -s 100 --filter "gzip $FILE" input.csv

  # WINDOWS: This will create files with names like 0.zip, 100.zip, etc. in the directory
  # 'outdir', and then run the command "Compress-Archive" on each chunk.
  qsv split outdir --filter "powershell Compress-Archive -Path $FILE -Destination {}.zip" input.csv

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_split.rs.

Usage:
    qsv split [options] (--size <arg> | --chunks <arg> | --kb-size <arg>) <outdir> [<input>]
    qsv split --help

split arguments:
    <outdir>              The directory where the output files will be written.
                          If it does not exist, it will be created.
    <input>               The CSV file to read. If not given, input is read from
                          STDIN.

split options:
    -s, --size <arg>       The number of records to write into each chunk.
                           [default: 500]
    -c, --chunks <arg>     The number of chunks to split the data into.
                           This option is mutually exclusive with --size.
                           The number of rows in each chunk is determined by
                           the number of records in the CSV data and the number
                           of desired chunks. If the number of records is not evenly
                           divisible by the number of chunks, the last chunk will
                           have fewer records.
    -k, --kb-size <arg>    The size of each chunk in kilobytes. The number of rows
                           in each chunk may vary, but the size of each chunk will
                           not exceed the desired size.
                           This option is mutually exclusive with --size and --chunks.

    -j, --jobs <arg>       The number of splitting jobs to run in parallel.
                           This only works when the given CSV data has
                           an index already created. Note that a file handle
                           is opened for each job.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.
    --filename <filename>  A filename template to use when constructing
                           the names of the output files.  The string '{}'
                           will be replaced by the zero-based row number
                           of the first row in the chunk.
                           [default: {}.csv]
    --pad <arg>            The zero padding width that is used in the
                           generated filename.
                           [default: 0]

                            FILTER OPTIONS:
    --filter <command>      Run the specified command on each chunk after it is written.
                            The command should use the FILE environment variable
                            ($FILE on Linux/macOS, %FILE% on Windows), which is
                            set to the path of the output file for each chunk.
                            The string '{}' in the command will be replaced by the
                            zero-based row number of the first row in the chunk.
    --filter-cleanup        Cleanup the original output filename AFTER the filter command
                            is run successfully for EACH chunk. If the filter command is not
                            successful, the original filename is not removed.
                            Only valid when --filter is used.
    --filter-ignore-errors  Ignore errors when running the filter command.
                            Only valid when --filter is used.

Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, the first row will NOT be interpreted
                           as column names. Otherwise, the first row will
                           appear in all chunks as the header row.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -q, --quiet            Do not display an output summary to stderr.
"#;

use std::{fs, io, path::Path, process::Command};

use log::debug;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    index::Indexed,
    util::{self, FilenameTemplate},
};

#[derive(Clone, Deserialize)]
struct Args {
    arg_input:                 Option<String>,
    arg_outdir:                String,
    flag_size:                 usize,
    flag_chunks:               Option<usize>,
    flag_kb_size:              Option<usize>,
    flag_jobs:                 Option<usize>,
    flag_filename:             FilenameTemplate,
    flag_pad:                  usize,
    flag_no_headers:           bool,
    flag_delimiter:            Option<Delimiter>,
    flag_quiet:                bool,
    flag_filter:               Option<String>,
    flag_filter_cleanup:       bool,
    flag_filter_ignore_errors: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;
    if args.flag_size == 0 {
        return fail_incorrectusage_clierror!("--size must be greater than 0.");
    }
    if let Some(0) = args.flag_chunks {
        return fail_incorrectusage_clierror!("--chunks must be greater than 0.");
    }
    if let Some(0) = args.flag_kb_size {
        return fail_incorrectusage_clierror!("--kb-size must be greater than 0.");
    }

    // --filter-cleanup and --filter-ignore-errors only make sense with --filter
    if args.flag_filter.is_none() && (args.flag_filter_cleanup || args.flag_filter_ignore_errors) {
        return fail_incorrectusage_clierror!(
            "--filter-cleanup and --filter-ignore-errors require --filter to be set."
        );
    }

    // check if outdir is set correctly
    if Path::new(&args.arg_outdir).is_file() && args.arg_input.is_none() {
        return fail_incorrectusage_clierror!("<outdir> is not specified or is a file.");
    }

    fs::create_dir_all(&args.arg_outdir)?;

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

        // Get path as string, unwrap is safe as temp files are always valid UTF-8
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        // Keep temp file from being deleted when it goes out of scope
        // it will be deleted when the program exits when TEMP_FILE_DIR is deleted
        temp_file
            .keep()
            .map_err(|e| format!("Failed to keep temporary stdin file: {e}"))?;

        args.arg_input = Some(temp_path);
    }

    if let Some(kb_size) = args.flag_kb_size {
        args.split_by_kb_size(kb_size)
    } else {
        // we're splitting by rowcount or by number of chunks
        match args.rconfig().indexed()? {
            Some(idx) => args.parallel_split(&idx),
            None => args.sequential_split(),
        }
    }
}

impl Args {
    fn split_by_kb_size(&self, chunk_size: usize) -> CliResult<()> {
        let rconfig = self.rconfig();
        let mut rdr = rconfig.reader()?;
        let headers = rdr.byte_headers()?.clone();

        // Build a representative chunk path (chunk-0) so the measurement Config
        // resolves the same extension-driven settings as `new_writer` (e.g. a
        // `--filename foo_{}.tsv` template uses tab delimiters). This way the
        // budget tracker and the on-disk writer agree on per-record bytes even
        // when the user picks a non-CSV extension or relies on extension-based
        // sniffing.
        let sample_chunk_path = Path::new(&self.arg_outdir)
            .join(
                self.flag_filename
                    .filename(&format!("{:0>width$}", 0, width = self.flag_pad)),
            )
            .display()
            .to_string();
        let measure_cfg = Config::new(Some(&sample_chunk_path));

        // `Config::from_writer` writes a UTF-8 BOM at writer construction when
        // QSV_OUTPUT_BOM is set. The real chunk writer emits that BOM exactly
        // once per file, so subtract it from each per-record measurement and
        // re-add it once below when initializing per-chunk byte usage.
        const UTF8_BOM_LEN: usize = 3;
        let bom_overhead = if util::get_envvar_flag("QSV_OUTPUT_BOM") {
            UTF8_BOM_LEN
        } else {
            0
        };

        // Single helper used for both header and per-row measurement. Reuses
        // one buffer to avoid allocating per row.
        let mut measure_buf: Vec<u8> = Vec::with_capacity(256);
        let mut measure_record = |record: &csv::ByteRecord| -> CliResult<usize> {
            measure_buf.clear();
            let mut m = measure_cfg.from_writer(&mut measure_buf);
            m.write_byte_record(record)?;
            m.flush()?;
            drop(m);
            Ok(measure_buf.len().saturating_sub(bom_overhead))
        };

        let header_byte_size = if self.flag_no_headers {
            0
        } else {
            measure_record(&headers)?
        };

        let chunk_size_bytes = chunk_size * 1024;
        // Each chunk file actually contains: [BOM?] + header + rows.
        let per_chunk_fixed_overhead = bom_overhead + header_byte_size;
        if chunk_size_bytes <= per_chunk_fixed_overhead {
            return fail_incorrectusage_clierror!(
                "--kb-size {chunk_size}KB ({chunk_size_bytes} bytes) is too small to fit the \
                 header row ({header_byte_size} bytes). Increase --kb-size or use --no-headers."
            );
        }

        // Open chunk files lazily so an empty input produces zero chunks
        // (no phantom `0.csv` containing only the header row).
        let mut wtr: Option<csv::Writer<Box<dyn io::Write + 'static>>> = None;
        let mut i: usize = 0; // total data rows processed
        let mut num_chunks: usize = 0;
        let mut chunk_start: usize = 0;
        let mut chunk_used_bytes = per_chunk_fixed_overhead;
        let mut row = csv::ByteRecord::new();

        while rdr.read_byte_record(&mut row)? {
            let row_size = measure_record(&row)?;

            let need_new_chunk = match &wtr {
                None => true,
                // Roll over if adding this row would exceed the budget. Always
                // allow at least one data row per chunk (otherwise an oversized
                // single row would loop forever with empty chunks).
                Some(_) => {
                    chunk_used_bytes + row_size > chunk_size_bytes
                        && chunk_used_bytes > per_chunk_fixed_overhead
                },
            };

            if need_new_chunk {
                if let Some(mut w) = wtr.take() {
                    w.flush()?;
                    if self.flag_filter.is_some() {
                        self.run_filter_command(chunk_start, self.flag_pad)?;
                    }
                    chunk_start = i;
                }
                wtr = Some(self.new_writer(&headers, chunk_start, self.flag_pad)?);
                chunk_used_bytes = per_chunk_fixed_overhead;
                num_chunks += 1;
            }

            // safety: `wtr` was just set above when `need_new_chunk` was true,
            // and was Some on every prior iteration once initialized.
            let active = wtr.as_mut().unwrap();
            active.write_byte_record(&row)?;
            chunk_used_bytes += row_size;
            i += 1;
        }

        if let Some(mut w) = wtr {
            w.flush()?;
            if self.flag_filter.is_some() {
                self.run_filter_command(chunk_start, self.flag_pad)?;
            }
        }

        if !self.flag_quiet {
            eprintln!(
                "Wrote {} chunk/s to '{}'. Size/chunk: <= {}KB; Num records: {}",
                num_chunks,
                dunce::canonicalize(Path::new(&self.arg_outdir))?.display(),
                chunk_size,
                i
            );
        }

        Ok(())
    }

    fn sequential_split(&self) -> CliResult<()> {
        let rconfig = self.rconfig();
        let mut rdr = rconfig.reader()?;
        let headers = rdr.byte_headers()?.clone();

        #[allow(clippy::cast_precision_loss)]
        let chunk_size = if let Some(flag_chunks) = self.flag_chunks {
            let count = util::count_rows(&rconfig)?;
            let chunk = flag_chunks;
            if chunk == 0 {
                return fail_incorrectusage_clierror!("--chunk must be greater than 0.");
            }
            (count as f64 / chunk as f64).ceil() as usize
        } else {
            self.flag_size
        };

        let mut wtr = self.new_writer(&headers, 0, self.flag_pad)?;
        let mut i: usize = 0;
        let mut nchunks: usize = 0;
        let mut row = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut row)? {
            if i > 0 && i.is_multiple_of(chunk_size) {
                wtr.flush()?;
                // Run filter command if specified
                if self.flag_filter.is_some() {
                    self.run_filter_command(i - chunk_size, self.flag_pad)?;
                }
                nchunks += 1;
                wtr = self.new_writer(&headers, i, self.flag_pad)?;
            }
            wtr.write_byte_record(&row)?;
            i += 1;
        }
        wtr.flush()?;
        // Run filter command for the last chunk if specified.
        // Skip when input had zero data rows: the unconditional new_writer above
        // already created (an empty) `0.csv`, but `i == 0` would underflow below.
        if self.flag_filter.is_some() && i > 0 {
            // Calculate the start index for the last chunk
            let last_chunk_start = ((i - 1) / chunk_size) * chunk_size;
            self.run_filter_command(last_chunk_start, self.flag_pad)?;
        }

        if !self.flag_quiet {
            eprintln!(
                "Wrote {} chunk/s to '{}'. Rows/chunk: {} Num records: {}",
                nchunks + 1,
                dunce::canonicalize(Path::new(&self.arg_outdir))?.display(),
                chunk_size,
                i
            );
        }

        Ok(())
    }

    fn parallel_split(&self, idx: &Indexed<fs::File, fs::File>) -> CliResult<()> {
        let chunk_size;
        let idx_count = idx.count();

        #[allow(clippy::cast_precision_loss)]
        let nchunks = if let Some(flag_chunks) = self.flag_chunks {
            chunk_size = (idx_count as f64 / flag_chunks as f64).ceil() as usize;
            flag_chunks
        } else {
            chunk_size = self.flag_size;
            util::num_of_chunks(idx_count as usize, self.flag_size)
        };
        if nchunks == 1 {
            // there's only one chunk, we can just do a sequential split
            // which has less overhead and better error handling
            return self.sequential_split();
        }

        util::njobs(self.flag_jobs);

        // Each worker writes its chunk independently; errors from any worker
        // short-circuit the whole operation via try_for_each.
        (0..nchunks)
            .into_par_iter()
            .try_for_each(|i| -> CliResult<()> {
                let conf = self.rconfig();
                // safety: indexed() returned Some at the call site; the index is
                // guaranteed to exist because parallel_split is only entered when
                // the caller observed a valid index. A failed re-open here is a
                // genuine I/O error and should propagate.
                let mut idx = conf.indexed()?.ok_or_else(|| {
                    crate::CliError::Other("indexed CSV vanished during parallel split".to_string())
                })?;
                let headers = idx.byte_headers()?;

                let mut wtr = self.new_writer(headers, i * chunk_size, self.flag_pad)?;

                idx.seek((i * chunk_size) as u64)?;
                for row in idx.byte_records().take(chunk_size) {
                    let write_row = row?;
                    wtr.write_byte_record(&write_row)?;
                }
                wtr.flush()?;

                if self.flag_filter.is_some() {
                    self.run_filter_command(i * chunk_size, self.flag_pad)?;
                }
                Ok(())
            })?;

        if !self.flag_quiet {
            eprintln!(
                "Wrote {} chunk/s to '{}'. Rows/chunk: {} Num records: {}",
                nchunks,
                dunce::canonicalize(Path::new(&self.arg_outdir))?.display(),
                chunk_size,
                idx_count
            );
        }

        Ok(())
    }

    fn new_writer(
        &self,
        headers: &csv::ByteRecord,
        start: usize,
        width: usize,
    ) -> CliResult<csv::Writer<Box<dyn io::Write + 'static>>> {
        let dir = Path::new(&self.arg_outdir);
        let path = dir.join(self.flag_filename.filename(&format!("{start:0>width$}")));
        let spath = Some(path.display().to_string());
        let mut wtr = Config::new(spath.as_ref()).writer()?;
        if !self.rconfig().no_headers {
            wtr.write_record(headers)?;
        }
        Ok(wtr)
    }

    fn run_filter_command(&self, start: usize, width: usize) -> CliResult<()> {
        if let Some(ref filter_cmd) = self.flag_filter {
            let outdir = Path::new(&self.arg_outdir).canonicalize()?;
            let filename = self.flag_filename.filename(&format!("{start:0>width$}"));
            let file_path = outdir.join(&filename);

            debug!(
                "Processing filter command for file: {}",
                file_path.display()
            );

            // Check if the file exists before running the filter command
            if !file_path.exists() {
                wwarn!(
                    "File {} does not exist, skipping filter command",
                    file_path.display()
                );
                return Ok(());
            }

            // Replace {} in the command with the start index
            let cmd = filter_cmd.replace("{}", &format!("{start:0>width$}"));
            debug!("Filter command template: {cmd}");

            // Use dunce to get a canonicalized path that works well on Windows
            // on non-Windows systems, its equivalent to std::fs::canonicalize
            let canonical_path = match dunce::canonicalize(&file_path) {
                Ok(path) => path,
                Err(e) => {
                    return fail_clierror!(
                        "Failed to canonicalize path {}: {e}",
                        file_path.display()
                    );
                },
            };

            let path_str = canonical_path.to_string_lossy().to_string();
            debug!("Canonicalized path: {path_str}");

            let canonical_outdir = match dunce::canonicalize(&outdir) {
                Ok(path) => path,
                Err(e) => {
                    return fail_clierror!(
                        "Failed to canonicalize outdir path {}: {e}",
                        outdir.display()
                    );
                },
            };

            // Execute the command using the appropriate shell based on platform.
            // On Windows we pass the entire command as a single argument to `cmd /C`
            // so that quoted arguments containing spaces are preserved (a previous
            // implementation split on whitespace, which corrupted such commands).
            let status = if cfg!(windows) {
                debug!("Running Windows command: cmd /C {cmd}");
                Command::new("cmd")
                    .arg("/C")
                    .arg(&cmd)
                    .current_dir(&canonical_outdir)
                    .env("FILE", path_str)
                    .status()
            } else {
                debug!("Running Unix command: sh -c {cmd}");
                Command::new("sh")
                    .arg("-c")
                    .arg(&cmd)
                    .current_dir(&canonical_outdir)
                    .env("FILE", path_str)
                    .status()
            };

            let status = match status {
                Ok(status) => status,
                Err(e) => {
                    return fail_clierror!("Failed to execute filter command: {e}");
                },
            };

            if !status.success() && !self.flag_filter_ignore_errors {
                return fail_clierror!(
                    "Filter command failed with exit code: {}",
                    status.code().unwrap_or(-1)
                );
            }

            // Cleanup the original output filename if the filter command was successful
            if self.flag_filter_cleanup {
                debug!("Cleaning up original file: {}", file_path.display());
                if let Err(e) = fs::remove_file(&file_path) {
                    wwarn!("Failed to remove file {}: {e}", file_path.display());
                }
            }
        }
        Ok(())
    }

    fn rconfig(&self) -> Config {
        Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers_flag(self.flag_no_headers)
    }
}
