static USAGE: &str = r#"
Implodes multiple rows into one by grouping on key column(s) and joining the
values of another column with the given separator. The inverse of `explode`.

Examples:

```csv
name,color
John,blue
John,yellow
John,light red
Mary,red
```

# Can be imploded by key column "name", joining the "color" column with "; "
$ qsv implode -k name -v color "; " data.csv

```csv
name,color
John,blue; yellow; light red
Mary,red
```

# With `-r colors` the value column is renamed
$ qsv implode -k name -v color -r colors "; " data.csv

```csv
name,colors
John,blue; yellow; light red
Mary,red
```

Only the key column(s) and the value column appear in the output; any other
columns are dropped.

By default, all input rows are buffered in memory and groups are emitted in the
order keys are first seen. If the input is already sorted by the key column(s),
use --sorted to stream groups as they are seen (memory proportional to the
largest group, not the whole input).

Usage:
    qsv implode [options] -k <keys> -v <value> <separator> [<input>]
    qsv implode --help

implode options:
    -k, --keys <keys>      Key column(s) to group by. Supports the usual
                           selector syntax (e.g. "name", "1", "1-3", "a,c").
    -v, --value <value>    The column whose values will be joined per group.
                           Must resolve to exactly one column.
    -r, --rename <name>    New name for the imploded value column.
    --sorted               Assume input is pre-sorted by the key column(s).
                           Streams groups as they are seen; memory is bounded
                           by the size of the largest group.
    --skip-empty           Skip empty values when joining. By default, empty
                           values are included as empty tokens so that
                           round-tripping with `explode` is lossless.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::collections::hash_map::Entry;

use foldhash::HashMap;
use serde::Deserialize;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    select::{SelectColumns, Selection},
    util,
};

#[derive(Deserialize)]
struct Args {
    flag_keys:       SelectColumns,
    flag_value:      SelectColumns,
    arg_separator:   String,
    arg_input:       Option<String>,
    flag_rename:     Option<String>,
    flag_sorted:     bool,
    flag_skip_empty: bool,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
}

#[inline]
fn extract_key_value(
    record: &csv::ByteRecord,
    key_sel: &Selection,
    value_idx: usize,
) -> (Vec<Vec<u8>>, Vec<u8>) {
    let key = key_sel
        .iter()
        .map(|&i| record.get(i).unwrap_or(&[]).to_vec())
        .collect();
    let value = record.get(value_idx).unwrap_or(&[]).to_vec();
    (key, value)
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;

    let byte_headers = rdr.byte_headers()?.clone();

    let use_names = !rconfig.no_headers;
    let key_sel = args.flag_keys.selection(&byte_headers, use_names)?;
    let value_sel = args.flag_value.selection(&byte_headers, use_names)?;

    if value_sel.len() != 1 {
        return fail_incorrectusage_clierror!(
            "--value must resolve to exactly one column, got {}",
            value_sel.len()
        );
    }
    let value_idx = value_sel[0];

    if key_sel.contains(&value_idx) {
        return fail_incorrectusage_clierror!("--value column must not also be a key column");
    }

    // Build output headers: key columns (in the order of the selection) + value column.
    if !rconfig.no_headers {
        let mut out_headers = csv::ByteRecord::new();
        for &i in key_sel.iter() {
            out_headers.push_field(&byte_headers[i]);
        }
        if let Some(new_name) = args.flag_rename.as_deref() {
            out_headers.push_field(new_name.as_bytes());
        } else {
            out_headers.push_field(&byte_headers[value_idx]);
        }
        wtr.write_byte_record(&out_headers)?;
    }

    let separator_bytes = args.arg_separator.as_bytes();
    let skip_empty = args.flag_skip_empty;

    let mut record = csv::ByteRecord::new();

    if args.flag_sorted {
        // Streaming path — assumes input is sorted by key columns.
        let mut current_key: Option<Vec<Vec<u8>>> = None;
        let mut current_values: Vec<Vec<u8>> = Vec::new();

        while rdr.read_byte_record(&mut record)? {
            let (key, value) = extract_key_value(&record, &key_sel, value_idx);
            let include = !(skip_empty && value.is_empty());

            match &current_key {
                Some(ck) if ck == &key => {
                    if include {
                        current_values.push(value);
                    }
                },
                _ => {
                    if let Some(ck) = current_key.take() {
                        write_group(&mut wtr, &ck, &current_values, separator_bytes)?;
                    }
                    current_values.clear();
                    if include {
                        current_values.push(value);
                    }
                    current_key = Some(key);
                },
            }
        }
        if let Some(ck) = current_key.take() {
            write_group(&mut wtr, &ck, &current_values, separator_bytes)?;
        }
    } else {
        // Buffered path — emits groups in first-seen order.
        let mut order: Vec<Vec<Vec<u8>>> = Vec::new();
        let mut groups: HashMap<Vec<Vec<u8>>, Vec<Vec<u8>>> = HashMap::default();

        while rdr.read_byte_record(&mut record)? {
            let (key, value) = extract_key_value(&record, &key_sel, value_idx);
            let include = !(skip_empty && value.is_empty());

            match groups.entry(key) {
                Entry::Occupied(mut o) => {
                    if include {
                        o.get_mut().push(value);
                    }
                },
                Entry::Vacant(v) => {
                    order.push(v.key().clone());
                    let vals = if include { vec![value] } else { Vec::new() };
                    v.insert(vals);
                },
            }
        }

        for key in &order {
            let vals = groups
                .get(key)
                .expect("key present in order must be in groups");
            write_group(&mut wtr, key, vals, separator_bytes)?;
        }
    }

    Ok(wtr.flush()?)
}

fn write_group<W: std::io::Write>(
    wtr: &mut csv::Writer<W>,
    key: &[Vec<u8>],
    values: &[Vec<u8>],
    separator: &[u8],
) -> CliResult<()> {
    let mut out = csv::ByteRecord::new();
    for k in key {
        out.push_field(k);
    }
    out.push_field(&values.join(separator));
    wtr.write_byte_record(&out)?;
    Ok(())
}
