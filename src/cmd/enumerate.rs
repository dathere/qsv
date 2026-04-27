static USAGE: &str = r#"
Add a new column enumerating the lines of a CSV file. This can be useful to keep
track of a specific line order, give a unique identifier to each line or even
make a copy of the contents of a column.

The enum function has six modes of operation:

  1. INCREMENT. Add an incremental identifier to each of the lines:
    $ qsv enum file.csv

  2. UUID4. Add a uuid v4 to each of the lines:
    $ qsv enum --uuid4 file.csv

  3. UUID7. Add a uuid v7 to each of the lines:
    $ qsv enum --uuid7 file.csv

  4. CONSTANT. Create a new column filled with a given value:
    $ qsv enum --constant 0

  5. COPY. Copy the contents of a column to a new one:
    $ qsv enum --copy names

  6. HASH. Create a new column with the deterministic hash of the given column/s.
     The hash uses the xxHash algorithm and is platform-agnostic.
     (see https://github.com/DoumanAsh/xxhash-rust for more information):
    $ qsv enum --hash 1- // hash all columns, auto-ignores existing "hash" column
    $ qsv enum --hash col2,col3,col4 // hash specific columns
    $ qsv enum --hash col2 // hash a single column
    $ qsv enum --hash /record_id|name|address/ // hash columns that match a regex
    $ qsv enum --hash !/record_id/ // hash all columns except the record_id column

  Finally, you should also be able to shuffle the lines of a CSV file by sorting
  on the generated uuid4s:
    $ qsv enum --uuid4 file.csv | qsv sort -s uuid4 > shuffled.csv

  This will shuffle the lines of the file.csv file as uuids generated using the v4
  specification are random and for practical purposes, are unique (1 in 2^122).
  See https://en.wikipedia.org/wiki/Universally_unique_identifier#Collisions

  However, sorting on uuid7 identifiers will not work as they are time-based
  and monotonically increasing, and will not shuffle the lines.

Examples:

  # Add an incremental index column starting from 0 (default)
  qsv enum data.csv

  # Add an incremental index column starting from 100 and incrementing by 10
  qsv enum --start 100 --increment 10 data.csv

  # Add a uuid v4 column
  qsv enum --uuid4 data.csv

  # Add a uuid v7 column
  qsv enum --uuid7 data.csv
    
  # Add a constant column with the value "active"
  qsv enum --constant active data.csv
    
  # Add a constant column with null values
  qsv enum --constant "<NULL>" data.csv
    
  # Add a copy of the "username" column as "username_copy"
  qsv enum --copy username data.csv

  # Add a hash column with the hash of columns "first_name" and "last_name"
  qsv enum --hash first_name,last_name data.csv

  # Add a hash column with the hash of all columns except an existing "hash" column
  qsv enum --hash 1- data.csv

  # Add a hash column with the hash of all columns except "id" and "uuid" columns
  qsv enum --hash "!id,!uuid" data.csv

  # Add a hash column with the hash of all columns that match the regex "record|name|address"
  qsv enum --hash "/record|name|address/" data.csv

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_enumerate.rs.

Usage:
    qsv enum [options] [<input>]
    qsv enum --help

enum options:
    -c, --new-column <name>  Name of the column to create.
                             Will default to "index".
    --start <value>          The value to start the enumeration from.
                             Only applies in Increment mode.
                             (default: 0)
    --increment <value>      The value to increment the enumeration by.
                             Only applies in Increment mode.
                             (default: 1)
    --constant <value>       Fill a new column with the given value.
                             Changes the default column name to "constant" unless
                             overridden by --new-column.
                             To specify a null value, pass the literal "<NULL>".
    --copy <column>          Name of a column to copy.
                             Changes the default column name to "{column}_copy"
                             unless overridden by --new-column.
    --uuid4                  When set, the column will be populated with
                             uuids (v4) instead of the incremental identifier.
                             Changes the default column name to "uuid4" unless
                             overridden by --new-column.
    --uuid7                  When set, the column will be populated with
                             uuids (v7) instead of the incremental identifier.
                             uuid v7 is a time-based uuid and is monotonically increasing.
                             See https://buildkite.com/blog/goodbye-integers-hello-uuids
                             Changes the default column name to "uuid7" unless
                             overridden by --new-column.
    --hash <columns>         Create a new column filled with the hash of the
                             given column/s. Use "1-" to hash all columns.
                             Changes the default column name to "hash" unless
                             overridden by --new-column.
                             Will remove an existing "hash" column if it exists.

                             The <columns> argument specify the columns to use
                             in the hash. Columns can be referenced by name or index,
                             starting at 1. Specify multiple columns by separating
                             them with a comma. Specify a range of columns with `-`.
                             (See 'qsv select --help' for the full syntax.)

Common options:
    -h, --help               Display this message
    -o, --output <file>      Write output to <file> instead of stdout.
    -n, --no-headers         When set, the first row will not be interpreted
                             as headers.
    -d, --delimiter <arg>    The field delimiter for reading CSV data.
                             Must be a single character. (default: ,)
"#;

use serde::Deserialize;
use uuid::Uuid;
use xxhash_rust::xxh3::Xxh3;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    select::{SelectColumns, Selection},
    util,
};

const NULL_VALUE: &str = "<NULL>";

#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    flag_new_column: Option<String>,
    flag_start:      u64,
    flag_increment:  Option<u64>,
    flag_constant:   Option<String>,
    flag_copy:       Option<SelectColumns>,
    flag_uuid4:      bool,
    flag_uuid7:      bool,
    flag_hash:       Option<SelectColumns>,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
}

#[derive(PartialEq)]
enum EnumOperation {
    Increment,
    Uuid4,
    Uuid7,
    Constant,
    Copy,
    Hash,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let mut rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;

    let mut headers = rdr.byte_headers()?.clone();
    let mut hash_index = None;

    let mut copy_index = 0;
    let copy_operation = if let Some(column_name) = args.flag_copy {
        rconfig = rconfig.select(column_name);
        let sel = rconfig.selection(&headers)?;
        copy_index = *sel.iter().next().unwrap();
        true
    } else {
        false
    };

    let hash_sel = if let Some(hash_columns) = &args.flag_hash {
        hash_index = headers.iter().position(|col| col == b"hash");

        let original_selection = rconfig
            .clone()
            .select(hash_columns.clone())
            .selection(&headers)?;

        let filtered_indices: Vec<usize> = original_selection
            .iter()
            .copied()
            .filter(|&i| Some(i) != hash_index)
            .collect();

        // Reject the degenerate case where the user's --hash selection resolves
        // to *only* the existing "hash" column. Otherwise the auto-exclusion
        // would leave nothing to hash, producing the same digest on every row.
        if filtered_indices.is_empty() {
            return fail_clierror!(
                "--hash selection resolves only to the existing \"hash\" column; nothing left to \
                 hash after auto-exclusion."
            );
        }

        // Build the Selection directly from the filtered indices to avoid an
        // ambiguous parse/selection round-trip — `SelectColumns::parse("")`
        // would produce an empty selector list, and `selection()` then returns
        // *all* columns, silently re-introducing the "hash" column.
        Some(Selection::from_indices(filtered_indices))
    } else {
        None
    };

    let constant_value = if args.flag_constant == Some(NULL_VALUE.to_string()) {
        b""
    } else {
        args.flag_constant.as_deref().unwrap_or("").as_bytes()
    };

    let enum_operation = if args.flag_constant.is_some() {
        EnumOperation::Constant
    } else if args.flag_uuid4 {
        EnumOperation::Uuid4
    } else if args.flag_uuid7 {
        EnumOperation::Uuid7
    } else if copy_operation {
        EnumOperation::Copy
    } else if args.flag_hash.is_some() {
        EnumOperation::Hash
    } else {
        EnumOperation::Increment
    };

    if !rconfig.no_headers {
        if enum_operation == EnumOperation::Hash
            && let Some(hash_idx) = hash_index
        {
            headers = headers
                .into_iter()
                .enumerate()
                .filter_map(|(i, field)| if i == hash_idx { None } else { Some(field) })
                .collect();
        }
        let column_name = match args.flag_new_column {
            Some(new_column_name) => new_column_name,
            _ => match enum_operation {
                EnumOperation::Increment => "index".to_string(),
                EnumOperation::Uuid4 => "uuid4".to_string(),
                EnumOperation::Uuid7 => "uuid7".to_string(),
                EnumOperation::Constant => "constant".to_string(),
                EnumOperation::Copy => {
                    let current_header = match simdutf8::compat::from_utf8(&headers[copy_index]) {
                        Ok(s) => s,
                        Err(e) => return fail_clierror!("Could not parse header as utf-8!: {e}"),
                    };
                    format!("{current_header}_copy")
                },
                EnumOperation::Hash => "hash".to_string(),
            },
        };
        headers.push_field(column_name.as_bytes());
        wtr.write_byte_record(&headers)?;
    }

    // amortize allocations
    let mut record = csv::ByteRecord::new();
    let mut counter: u64 = args.flag_start;
    let mut colcopy: Vec<u8> = Vec::new();
    let increment = args.flag_increment.unwrap_or(1);
    let mut hasher = Xxh3::new();
    let mut filtered_record = csv::ByteRecord::new();
    let uuid7_ctxt = uuid::ContextV7::new();
    let mut uuid;

    while rdr.read_byte_record(&mut record)? {
        match enum_operation {
            EnumOperation::Increment => {
                record.push_field(itoa::Buffer::new().format(counter).as_bytes());
                counter += increment;
            },
            EnumOperation::Uuid4 => {
                uuid = Uuid::new_v4();
                record.push_field(
                    uuid.as_hyphenated()
                        .encode_lower(&mut Uuid::encode_buffer())
                        .as_bytes(),
                );
            },
            EnumOperation::Uuid7 => {
                uuid = Uuid::new_v7(uuid::Timestamp::now(&uuid7_ctxt));
                record.push_field(
                    uuid.as_hyphenated()
                        .encode_lower(&mut Uuid::encode_buffer())
                        .as_bytes(),
                );
            },
            EnumOperation::Constant => {
                record.push_field(constant_value);
            },
            EnumOperation::Copy => {
                colcopy.clear();
                colcopy.extend_from_slice(&record[copy_index]);
                record.push_field(&colcopy);
            },
            EnumOperation::Hash => {
                // Hash raw bytes, length-prefixing each field so distinct row
                // contents cannot collide via concatenation
                // (e.g. ["ab","c"] vs ["a","bc"]).
                hasher.reset();
                if let Some(ref sel) = hash_sel {
                    for &i in sel.iter() {
                        let field = &record[i];
                        hasher.update(&(field.len() as u64).to_le_bytes());
                        hasher.update(field);
                    }
                }
                let hash = hasher.digest();

                if let Some(hash_idx) = hash_index {
                    filtered_record.clear();
                    for (i, field) in record.iter().enumerate() {
                        if i != hash_idx {
                            filtered_record.push_field(field);
                        }
                    }
                    std::mem::swap(&mut record, &mut filtered_record);
                }
                record.push_field(itoa::Buffer::new().format(hash).as_bytes());
            },
        }

        wtr.write_byte_record(&record)?;
    }
    Ok(wtr.flush()?)
}
