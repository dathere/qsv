static USAGE: &str = r#"
Modify headers of a CSV to only have "safe" names - guaranteed "database-ready" names
(optimized specifically for PostgreSQL column identifiers). 

Fold to lowercase. Trim leading & trailing whitespaces. Replace whitespace/non-alphanumeric
characters with _. If name starts with a number & check_first_char is true, prepend the unsafe prefix.
If a header with the same name already exists, append a sequence suffix (e.g. col, col_2, col_3).
Names are limited to 60 bytes in length (snapped to UTF-8 char boundary, including any
duplicate-disambiguation suffix). Empty names are replaced with the unsafe prefix.

In addition, specifically because of CKAN Datastore requirements:
- Headers with leading underscores are replaced with "unsafe_" prefix.
- Headers that are named "_id" are renamed to "reserved__id".

These CKAN Datastore options can be configured via the --prefix & --reserved options, respectively.

In Always (a) and Conditional (c) mode, returns number of modified headers to stderr,
and sends CSV with safe headers output to stdout.

In Verify (v) mode, returns number of unsafe headers to stderr.
In Verbose (V) mode, returns number of headers; duplicate count and unsafe & safe headers to stderr.
No stdout output is generated in Verify and Verbose mode.

In JSON (j) mode, returns Verbose mode info in minified JSON to stdout.
In Pretty JSON (J) mode, returns Verbose mode info in pretty printed JSON to stdout.

Given data.csv:
 c1,12_col,Col with Embedded Spaces,,Column!@Invalid+Chars,c1
 1,a2,a3,a4,a5,a6

  $ qsv safenames data.csv
  c1,unsafe_12_col,col_with_embedded_spaces,unsafe_,column__invalid_chars,c1_2
  1,a2,a3,a4,a5,a6
  stderr: 5

  Conditionally rename headers, allowing "quoted identifiers":
  $ qsv safenames --mode c data.csv
  c1,unsafe_12_col,Col with Embedded Spaces,unsafe_,column__invalid_chars,c1_2
  1,a2,a3,a4,a5,a6
  stderr: 4

  Verify how many "unsafe" headers are found:
  $ qsv safenames --mode v data.csv
  stderr: 4

  Verbose mode:
  $ qsv safenames --mode V data.csv
  stderr: 6 header/s
  1 duplicate/s: "c1:2"
  4 unsafe header/s: ["12_col", "Col with Embedded Spaces", "", "Column!@Invalid+Chars"]
  1 safe header/s: ["c1"]

Note that even if "Col with Embedded Spaces" is technically safe, it is generally discouraged.
Though it can be created as a "quoted identifier" in PostgreSQL, it is still marked "unsafe"
by default, unless mode is set to "conditional." 

It is discouraged because the embedded spaces can cause problems later on.
(see https://lerner.co.il/2013/11/30/quoting-postgresql/ for more info).

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_safenames.rs.

Usage:
    qsv safenames [options] [<input>]
    qsv safenames --help

safenames options:
    --mode <mode>          Rename header names to "safe" names — guaranteed
                           "database-ready" names. Mode is selected by the FIRST
                           character: c/C conditional, a/A always, v verify,
                           V Verbose, j JSON, J pretty JSON (case matters for
                           v vs V and j vs J; --mode verbose maps to 'v', NOT V).
                           Mode details:
                             c, C  - conditional. Check first before renaming;
                                     preserves "quoted identifiers" (mixed case
                                     with embedded spaces).
                             a, A  - always. Rename every header, even safe ones.
                             v     - verify. Count unsafe headers; result to stderr.
                             V     - Verbose. Like verify, but also lists header
                                     count, duplicates, unsafe & safe headers.
                             j     - JSON. Verbose data as minified JSON to stdout.
                             J     - Pretty JSON. Verbose data as pretty-printed JSON.
                           Quoted identifiers are only treated as safe in
                           conditional mode; verify, Verbose, and the JSON modes
                           flag them as unsafe.
                           [default: Always]
    --reserved <list>      Comma-delimited list of additional case-insensitive reserved names
                           that should be considered "unsafe." If a header name is found in 
                           the reserved list, it will be prefixed with "reserved_".
                           [default: _id]
    --prefix <string>      Certain systems do not allow header names to start with "_" (e.g. CKAN Datastore).
                           This option allows the specification of the unsafe prefix to use when a header
                           starts with "_". [default: unsafe_]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
                           Note that no output is generated for Verify and
                           Verbose modes.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use foldhash::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::{
    CliResult,
    config::{Config, Delimiter},
    util,
};

#[derive(Deserialize)]
struct Args {
    arg_input:      Option<String>,
    flag_mode:      String,
    flag_reserved:  String,
    flag_prefix:    String,
    flag_output:    Option<String>,
    flag_delimiter: Option<Delimiter>,
}

#[derive(PartialEq)]
enum SafeNameMode {
    Always,
    Conditional,
    Verify,
    VerifyVerbose,
    VerifyVerboseJSON,
    VerifyVerbosePrettyJSON,
}

#[derive(Serialize, Deserialize)]
struct SafeNamesStruct {
    header_count:      usize,
    duplicate_count:   usize,
    duplicate_headers: Vec<String>,
    unsafe_headers:    Vec<String>,
    safe_headers:      Vec<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // set SafeNames Mode
    let first_letter = args.flag_mode.chars().next().unwrap_or_default();
    let safenames_mode = match first_letter {
        'c' | 'C' => SafeNameMode::Conditional,
        'a' | 'A' => SafeNameMode::Always,
        'v' => SafeNameMode::Verify,
        'V' => SafeNameMode::VerifyVerbose,
        'j' => SafeNameMode::VerifyVerboseJSON,
        'J' => SafeNameMode::VerifyVerbosePrettyJSON,
        _ => {
            return fail_clierror!("Invalid mode: {}", args.flag_mode);
        },
    };

    let reserved_names_vec: Vec<String> = args
        .flag_reserved
        .split(',')
        .map(str::to_lowercase)
        .collect();

    let rconfig = Config::new(args.arg_input.as_ref()).delimiter(args.flag_delimiter);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;
    let old_headers = rdr.byte_headers()?;

    // Lossy decode is needed because safe_header_names operates on StringRecord;
    // we discard `lossy_headers` after building noquote_headers and write
    // safe_headers directly in the always/conditional path.
    let lossy_headers = csv::StringRecord::from_byte_record_lossy(old_headers.clone());

    // trim enclosing quotes and spaces from headers as it messes up safenames
    // csv library will automatically add quotes when necessary when we write it
    let mut noquote_headers = csv::StringRecord::new();
    for header in &lossy_headers {
        noquote_headers.push_field(header.trim_matches(|c| c == '"' || c == ' '));
    }

    let (safe_headers, changed_count) = util::safe_header_names(
        &noquote_headers,
        true,
        safenames_mode == SafeNameMode::Conditional,
        Some(&reserved_names_vec),
        &args.flag_prefix,
        false,
    );
    if let SafeNameMode::Conditional | SafeNameMode::Always = safenames_mode {
        // write CSV with safe headers
        wtr.write_record(&safe_headers)?;
        let mut record = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut record)? {
            wtr.write_record(&record)?;
        }
        wtr.flush()?;

        eprintln!("{changed_count}");
    } else {
        // Verify or VerifyVerbose Mode
        // Compare each header positionally against its rewritten form so that
        // headers renamed by the duplicate-suffixing pass (e.g. col1 -> col1_2)
        // are correctly counted as unsafe — keeps verify counts in sync with
        // always-mode's changed_count.
        let mut safenames_vec: Vec<String> = Vec::new();
        let mut unsafenames_vec: Vec<String> = Vec::new();
        let mut seen_safe: HashSet<String> = HashSet::default();
        let mut counts: HashMap<String, u16> = HashMap::default();

        for (i, header_name) in noquote_headers.iter().enumerate() {
            *counts.entry(header_name.to_string()).or_insert(0) += 1;
            if safe_headers[i] == header_name {
                // safe_headers is the positional rewritten-header vector; the
                // displayed safe-header list is deduped here via seen_safe so
                // we only show each unchanged safe name once. unsafe_headers
                // below still records every offending position so the count
                // matches always-mode's changed_count — hence a duplicated
                // header (e.g. 5x "col1") shows up once in safe and four times
                // in unsafe.
                if seen_safe.insert(header_name.to_string()) {
                    safenames_vec.push(header_name.to_string());
                }
            } else {
                unsafenames_vec.push(header_name.to_string());
            }
        }

        let headers_count = noquote_headers.len();
        let unsafe_count = unsafenames_vec.len();
        let safe_count = safenames_vec.len();

        // Sort duplicate entries for deterministic output (HashMap iteration
        // order is otherwise unstable across runs).
        let mut dupes: Vec<(String, u16)> = counts.into_iter().filter(|&(_, v)| v > 1).collect();
        dupes.sort();
        let dupe_count = dupes.len();
        let duplicate_headers: Vec<String> =
            dupes.into_iter().map(|(k, v)| format!("{k}:{v}")).collect();

        let safenames_struct = SafeNamesStruct {
            header_count: headers_count,
            duplicate_count: dupe_count,
            duplicate_headers,
            unsafe_headers: unsafenames_vec.clone(),
            safe_headers: safenames_vec.clone(),
        };
        match safenames_mode {
            SafeNameMode::VerifyVerbose => {
                eprintln!(
                    r#"{num_headers} header/s
{dupe_count} duplicate/s: {dupe_headers:?}
{unsafe_count} unsafe header/s: {unsafenames_vec:?}
{num_safeheaders} safe header/s: {safenames_vec:?}"#,
                    dupe_headers = safenames_struct.duplicate_headers.join(", "),
                    num_headers = headers_count,
                    num_safeheaders = safe_count
                );
            },
            SafeNameMode::VerifyVerboseJSON | SafeNameMode::VerifyVerbosePrettyJSON => {
                let json = if safenames_mode == SafeNameMode::VerifyVerbosePrettyJSON {
                    simd_json::to_string_pretty(&safenames_struct)?
                } else {
                    simd_json::to_string(&safenames_struct)?
                };
                println!("{json}");
            },
            _ => eprintln!("{unsafe_count}"),
        }
    }

    Ok(())
}
