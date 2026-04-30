static USAGE: &str = r#"
Replace the value of a cell specified by its row and column.

Example:

items.csv
```csv
item,color
shoes,blue
flashlight,gray
```

# To output the data with the color of the shoes as green instead of blue
$ qsv edit items.csv color 0 green

```csv
item,color
shoes,green
flashlight,gray
```

You may also choose to specify the column name by its index (in this case 1).
Specifying a column as a number is prioritized by index rather than name.
If there is no newline (\n) at the end of the input data, it may be added to the output.

Usage:
    qsv edit [options] <input> <column> <row> <value>
    qsv edit --help

edit arguments:
    input                  The file from which to edit a cell value. Use '-' for standard input.
                           Must be either CSV, TSV, TAB, or SSV data.
    column                 The cell's column name or index. Indices start from the first column as 0.
                           Providing a value of underscore (_) selects the last column.
    row                    The cell's row index. Indices start from the first non-header row as 0.
    value                  The new value to replace the old cell content with.

If <row> is out of range:
  - in stdout/--output mode, the input is passed through unchanged with a warning on stderr.
  - in --in-place mode, the command errors and the input file is left untouched.

edit options:
    -i, --in-place         Overwrite the input file data with the output.
                           The input file is renamed to a .bak file in the same directory.
                           If the .bak file already exists, the command errors instead of overwriting it.
                           Symbolic links are rejected; pass the resolved path instead.
                           (Other Windows reparse points such as junction points are not detected.)

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       Start row indices from the header row as 0 (allows editing the header row).
"#;

use csv::Writer;
use serde::Deserialize;
use tempfile::NamedTempFile;

use crate::{CliResult, config::Config, util};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    arg_column:      String,
    arg_row:         usize,
    arg_value:       String,
    flag_in_place:   bool,
    flag_output:     Option<String>,
    flag_no_headers: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let input = args.arg_input;
    let column = args.arg_column;
    let row = args.arg_row;
    let in_place = args.flag_in_place;
    let value = args.arg_value;
    let no_headers = args.flag_no_headers;

    // --in-place needs a real, non-symlink on-disk path. Validate up front so
    // we don't create a tempfile or do any work for an obviously-bad request.
    let in_place_path = if in_place {
        let p = match input.as_deref() {
            Some(p) if p != "-" => std::path::PathBuf::from(p),
            _ => {
                return fail_clierror!(
                    "--in-place requires an input file path (stdin is not supported).",
                );
            },
        };
        let metadata = std::fs::symlink_metadata(&p)?;
        if metadata.file_type().is_symlink() {
            return fail_clierror!(
                "--in-place does not support symlinks; pass the resolved path instead.",
            );
        }
        Some(p)
    } else {
        None
    };

    // Build the CSV reader and iterate over each record.
    let conf = Config::new(input.as_ref()).no_headers(true);
    let mut rdr = conf.reader()?;

    // Place the in-place tempfile in the same directory as the input so the
    // final persist is a same-filesystem rename (atomic and fast).
    let mut tempfile = if let Some(p) = in_place_path.as_ref() {
        let parent = p
            .parent()
            .filter(|d| !d.as_os_str().is_empty())
            .unwrap_or_else(|| std::path::Path::new("."));
        Some(NamedTempFile::new_in(parent)?)
    } else {
        None
    };
    let mut wtr: Writer<Box<dyn std::io::Write>> = if let Some(tf) = tempfile.as_mut() {
        csv::Writer::from_writer(Box::new(tf.as_file_mut()))
    } else {
        Config::new(args.flag_output.as_ref()).writer()?
    };

    let headers = rdr.headers()?;
    let column_index: usize = if column == "_" {
        match headers.len().checked_sub(1) {
            Some(i) => i,
            None => return fail_clierror!("Invalid column selected."),
        }
    } else if let Ok(c) = column.parse::<usize>() {
        if c >= headers.len() {
            return fail_clierror!("Invalid column selected.");
        }
        c
    } else {
        match headers.iter().position(|h| column.as_str() == h) {
            Some(i) => i,
            None => return fail_clierror!("Invalid column selected."),
        }
    };

    let mut record = csv::ByteRecord::new();
    #[allow(clippy::bool_to_int_with_if)]
    let mut current_row: usize = if no_headers { 1 } else { 0 };
    let Some(target_row) = row.checked_add(1) else {
        return fail_clierror!("Row index too large.");
    };
    let mut row_matched = false;
    while rdr.read_byte_record(&mut record)? {
        if current_row == target_row {
            row_matched = true;
            let mut updated = csv::ByteRecord::new();
            for (i, field) in record.iter().enumerate() {
                if i == column_index {
                    updated.push_field(value.as_bytes());
                } else {
                    updated.push_field(field);
                }
            }
            wtr.write_byte_record(&updated)?;
        } else {
            wtr.write_byte_record(&record)?;
        }
        current_row += 1;
    }

    wtr.flush()?;
    drop(wtr);

    // For in-place edits, missing rows are a hard error (we don't want to rename
    // the input to .bak and replace it with an unchanged copy). For stdout/output
    // mode, warn but still emit the unchanged CSV so callers piping output get a
    // valid pass-through with exit 0.
    if let (Some(tempfile), Some(input_path)) = (tempfile, in_place_path) {
        if !row_matched {
            return fail_clierror!("Row {row} not found.");
        }
        let backup_path = input_path.with_added_extension("bak");
        // Atomically reserve the backup path with create_new so a concurrent
        // process can't slip in between an existence check and the rename.
        // Works on any filesystem (unlike hard_link, which fails on FAT/SMB).
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&backup_path)
        {
            Ok(_) => {},
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                return fail_clierror!(
                    "Backup file {} already exists; refusing to overwrite.",
                    backup_path.display()
                );
            },
            Err(e) => return Err(e.into()),
        }
        // Move input over our placeholder. std::fs::rename is documented to
        // replace the destination on all platforms (per std docs: "This
        // function will replace the destination file if it already exists.").
        // On Windows, this is backed by FileRenameInfoEx on supported
        // filesystems and by MoveFileEx with REPLACE_EXISTING otherwise — the
        // only case rename refuses to overwrite is directory-on-directory,
        // which doesn't apply here since our placeholder is a regular file.
        std::fs::rename(&input_path, &backup_path)?;
        // Persist the edited tempfile to input_path. Since the tempfile lives
        // in the same directory as the input, this is an atomic same-fs rename.
        // The window between this and the rename above is two same-dir rename
        // syscalls — much smaller than the prior copy, but not zero. We accept
        // the small window rather than copy-then-persist (which would halve
        // throughput on every successful edit by doubling disk I/O); the only
        // loser is a concurrent reader hitting ENOENT.
        // NOTE: the rollback branch below is not covered by an automated test
        // — reliably forcing tempfile.persist to fail after a successful rename
        // requires platform-specific filesystem manipulation.
        if let Err(e) = tempfile.persist(&input_path) {
            // Best-effort rollback: restore the original from the backup we
            // just took, so the user isn't left without input_path.
            let recovery = match std::fs::rename(&backup_path, &input_path) {
                Ok(()) => format!("original restored from {}", backup_path.display()),
                Err(rollback_err) => format!(
                    "rollback also failed ({}); original remains at {}",
                    rollback_err,
                    backup_path.display()
                ),
            };
            return fail_clierror!(
                "Failed to install edited file at {}: {}. {}.",
                input_path.display(),
                e.error,
                recovery
            );
        }
    } else if !row_matched {
        eprintln!("Warning: row {row} not found; input passed through unchanged.");
    }

    Ok(())
}
