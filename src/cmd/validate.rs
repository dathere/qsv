static USAGE: &str = r#"
Validates CSV data using two main modes:

JSON SCHEMA VALIDATION MODE:
===========================

This mode is invoked if a JSON Schema file (draft 2020-12) is provided.

The CSV data is validated against the JSON Schema. If the CSV data is valid, no output
files are created and the command returns an exit code of 0.

If invalid records are found, they are put into an "invalid" file, with the rest of the
records put into a "valid"" file.

A "validation-errors.tsv" report is also created with the following columns:

  * row_number: the row number of the invalid record
  * field: the field name of the invalid field
  * error: a validation error message detailing why the field is invalid

It uses the JSON Schema Validation Specification (draft 2020-12) to validate the CSV.
It validates the structure of the file, as well as the data types and domain/range of the fields.
See https://json-schema.org/draft/2020-12/json-schema-validation.html

qsv supports a custom format - `currency`. This format will only accept a valid currency, defined as: 

 1. ISO Currency Symbol (optional): This is the ISO 4217 three-character code or currency symbol
    (e.g. USD, EUR, JPY, $, €, ¥, etc.)
 2. Amount: This is the numerical value of the currency. More than 2 decimal places are allowed.
 3. Formats: Valid currency formats include:
      Standard: $1,000.00 or USD1000.00  
      Negative amounts: ($100.00) or -$100.00
      Different styles: 1.000,00 (used in some countries for euros)

qsv also supports two custom keywords - `dynamicEnum` and `uniqueCombinedWith`.

dynamicEnum
===========
`dynamicEnum` allows for dynamic validation against a reference CSV file.
It can be used to validate against a set of values unknown at the time of schema creation or
when the set of valid values is dynamic or too large to hardcode into the JSON Schema with `enum`.
The reference CSV file can be local or a URL (http/https, dathere & ckan schemes supported).
The "dynamicEnum" value has the form:

  // qsvlite binary variant only supports URIs which can be files on the local filesystem
  // or remote files (http and https schemes supported)
  dynamicEnum = "URI|colname" where colname is the column name or column index (0-based)

    // use data.csv from the current working directory; use the 1st column for validation
    dynamicEnum = "data.csv"

    // use data.csv in /lookup_dir directory; use the column "Agency" for validation
    dynamicEnum = "/lookupdir/data.csv|Agency"

    // get data.csv; use the 3rd column for validation (2 as the col index is 0-based)
    dynamicEnum = "https://example.com/data.csv|2"

  // on other qsv binary variants, dynamicEnum has expanded caching functionality
  dynamicEnum = "[cache_name;cache_age]|URI|colname" where cache_name and cache_age are optional

    // use data.csv from current working directory; cache it as data with a default 
    // cache age of 3600 seconds i.e. the cached data.csv expires after 1 hour
    dynamicEnum = "data.csv"

    // get data.csv; cache it as custom_name, cache age 600 seconds
    dynamicEnum = "custom_name;600|https://example.com/data.csv"

    // get data.csv; cache it as data, cache age 800 seconds
    dynamicEnum = ";800|https://example.com/data.csv"

    // get the top matching result for nyc_neighborhoods (signaled by trailing ?),
    // cache it as nyc_neighborhood_data.csv (NOTE: cache name is required when using CKAN scheme)
    // with a default cache age of 3600 seconds
    // be sure to set --ckan-api, otherwise it will default to datHere's CKAN (data.dathere.com)
    dynamicEnum = "nyc_neighborhood_data|ckan:://nyc_neighborhoods?"

    // get CKAN resource with id 1234567, cache it as resname, 3600 secs cache age
    // note that if the resource is a private resource, you'll need to set --ckan-token
    dynamicEnum = "resname|ckan:://1234567"

    // same as above but with a cache age of 100 seconds; use the borough column for validation
    dynamicEnum = "resname;100|ckan:://1234567|borough

    // get us_states.csv from datHere lookup tables
    dynamicEnum = "dathere://us_states.csv"

If colname is not specified, the first column of the CSV file is read and used for validation.

uniqueCombinedWith
==================
`uniqueCombinedWith` allows you to validate that combinations of values across specified columns
are unique. It can be used with either column names or column indices (0-based). For example:

    // Validate that combinations of name and email are unique
    uniqueCombinedWith = ["name", "email"]

    // Validate that combinations of columns at indices 1 and 2 are unique
    uniqueCombinedWith = [1, 2]

    // Validate that the combinations of named and indexed columns are unique
    uniqueCombinedWith = ["name", 2]

When a duplicate combination is found, the validation will fail and the error message will indicate
which columns had duplicate combinations (named columns first, then indexed columns). The invalid
records will be written to the .invalid file, while valid records will be written to the .valid file.

`uniqueCombinedWith` complements the standard `uniqueItems` keyword, which can only validate
uniqueness across a single column. 

-------------------------------------------------------

You can create a JSON Schema file from a reference CSV file using the `qsv schema` command.
Once the schema is created, you can fine-tune it to your needs and use it to validate other CSV
files that have the same structure.

Be sure to select a "training" CSV file that is representative of the data you want to validate
when creating a schema. The data types, domain/range and regular expressions inferred from the
reference CSV file should be appropriate for the data you want to validate. 

Typically, after creating a schema, you should edit it to fine-tune each field's inferred
validation rules.

For example, if we created a JSON schema file called "reference.schema.json" using the `schema` command.
And want to validate "mydata.csv" which we know has validation errors, the output files from running
`qsv validate mydata.csv reference.schema.json` are: 

  * mydata.csv.valid
  * mydata.csv.invalid
  * mydata.csv.validation-errors.tsv

With an exit code of 1 to indicate a validation error.

If we validate another CSV file, "mydata2.csv", which we know is valid, there are no output files,
and the exit code is 0.

If piped from stdin, the filenames will use `stdin.csv` as the base filename. For example:
  `cat mydata.csv | qsv validate reference.schema.json`

   * stdin.csv.valid
   * stdin.csv.invalid
   * stdin.csv.validation-errors.tsv

`validate` also has a `schema` subcommand to validate JSON Schema files. For example:
  `qsv validate schema myjsonschema.json`

RFC 4180 VALIDATION MODE:
========================

If run without a JSON Schema file, the CSV is validated for RFC 4180 CSV standard compliance
(see https://github.com/dathere/qsv#rfc-4180-csv-standard).

It also confirms if the CSV is UTF-8 encoded.

For both modes, returns exit code 0 when the CSV file is valid, exitcode > 0 otherwise.
If all records are valid, no output files are produced.

For examples, see the tests included in this file (denoted by '#[test]') or see
https://github.com/dathere/qsv/blob/master/tests/test_validate.rs.

Usage:
    qsv validate schema [<json-schema>]
    qsv validate [options] [<input>] [<json-schema>]
    qsv validate --help

Validate arguments:
    <input>                    Input CSV file to validate. If not provided, will read from stdin.
    <json-schema>              JSON Schema file to validate against. If not provided, `validate`
                               will run in RFC 4180 validation mode. The file can be a local file
                               or a URL (http and https schemes supported).

Validate options:
    --trim                     Trim leading and trailing whitespace from fields before validating.
    --no-format-validation     Disable JSON Schema format validation. Ignores all JSON Schema
                               "format" keywords (e.g. date,email, uri, currency, etc.). This is
                               useful when you want to validate the structure of the CSV file
                               w/o worrying about the data types and domain/range of the fields.
    --fail-fast                Stops on first error.
    --valid <suffix>           Valid record output file suffix. [default: valid]
    --invalid <suffix>         Invalid record output file suffix. [default: invalid]
    --json                     When validating without a JSON Schema, return the RFC 4180 check
                               as a JSON file instead of a message.
    --pretty-json              Same as --json, but pretty printed.
    --valid-output <file>      Change validation mode behavior so if ALL rows are valid, to pass it to
                               output, return exit code 1, and set stderr to the number of valid rows.
                               Setting this will override the default behavior of creating
                               a valid file only when there are invalid records.
                               To send valid records to stdout, use `-` as the filename.
    -j, --jobs <arg>           The number of jobs to run in parallel.
                               When not set, the number of jobs is set to the
                               number of CPUs detected.
    -b, --batch <size>         The number of rows per batch to load into memory,
                               before running in parallel. Automatically determined
                               for CSV files with more than 50000 rows.
                               Set to 0 to load all rows in one batch.
                               Set to 1 to force batch optimization even for files with
                               less than 50000 rows. [default: 50000]

                               FANCY REGEX OPTIONS:
    --fancy-regex              Use the fancy regex engine instead of the default regex engine
                               for validation.
                               The fancy engine supports advanced regex features such as
                               lookaround and backreferences, but is not as performant as
                               the default regex engine which guarantees linear-time matching,
                               prevents DoS attacks, and is more efficient for simple patterns.
    --backtrack-limit <limit>  Set the approximate number of backtracking steps allowed.
                               This is only used when --fancy-regex is set.
                               [default: 1000000]

                               OPTIONS FOR BOTH REGEX ENGINES:
    --size-limit <mb>          Set the approximate size limit, in megabytes, of a compiled regex.
                               [default: 50]
    --dfa-size-limit <mb>      Set the approximate capacity, in megabytes, of the cache of transitions
                               used by the engine's lazy Discrete Finite Automata.
                               [default: 10]
    
    --timeout <seconds>        Timeout for downloading json-schemas on URLs and for
                               'dynamicEnum' lookups on URLs. [default: 30]
    --cache-dir <dir>          The directory to use for caching downloaded dynamicEnum resources.
                               If the directory does not exist, qsv will attempt to create it.
                               If the QSV_CACHE_DIR envvar is set, it will be used instead.
                               Not available on qsvlite.
                               [default: ~/.qsv-cache]
    --ckan-api <url>           The URL of the CKAN API to use for downloading dynamicEnum
                               resources with the "ckan://" scheme.
                               If the QSV_CKAN_API envvar is set, it will be used instead.
                               Not available on qsvlite.
                               [default: https://data.dathere.com/api/3/action]
    --ckan-token <token>       The CKAN API token to use. Only required if downloading
                               private resources.
                               If the QSV_CKAN_TOKEN envvar is set, it will be used instead.
                               Not available on qsvlite.

Common options:
    -h, --help                 Display this message
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. It will be validated with the rest
                               of the rows. Otherwise, the first row will always
                               appear as the header row in the output.
                               Note that this option is only valid when running
                               in RFC 4180 validation mode as JSON Schema validation
                               requires headers.
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character.
    -p, --progressbar          Show progress bars. Not valid for stdin.
    -q, --quiet                Do not display validation summary message.
"#;

use std::{
    env,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    str,
    sync::{
        OnceLock,
        atomic::{AtomicU16, Ordering},
    },
};

use bitvec::prelude::*;
use csv::ByteRecord;
use foldhash::{HashSet, HashSetExt};
use indicatif::HumanCount;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{ProgressBar, ProgressDrawTarget};
use jsonschema::{
    Keyword, PatternOptions, ValidationError, Validator,
    output::BasicOutput,
    paths::{LazyLocation, Location},
};
use log::{debug, info, log_enabled};
use qsv_currency::Currency;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::IntoParallelRefIterator,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json, value::Number};
#[cfg(feature = "lite")]
use tempfile::NamedTempFile;

#[cfg(not(feature = "lite"))]
use crate::lookup;
#[cfg(not(feature = "lite"))]
use crate::lookup::{LookupTableOptions, load_lookup_table};
use crate::{
    CliError, CliResult,
    config::{Config, DEFAULT_RDR_BUFFER_CAPACITY, DEFAULT_WTR_BUFFER_CAPACITY, Delimiter},
    util,
};

// to save on repeated init/allocs
static NULL_TYPE: OnceLock<Value> = OnceLock::new();

static TIMEOUT_SECS: AtomicU16 = AtomicU16::new(30);

#[cfg(not(feature = "lite"))]
static QSV_CACHE_DIR: OnceLock<String> = OnceLock::new();

#[cfg(not(feature = "lite"))]
static CKAN_API: OnceLock<String> = OnceLock::new();

#[cfg(not(feature = "lite"))]
static CKAN_TOKEN: OnceLock<Option<String>> = OnceLock::new();
static DELIMITER: OnceLock<Option<Delimiter>> = OnceLock::new();

/// write to stderr and log::error, using ValidationError
macro_rules! fail_validation_error {
    ($($t:tt)*) => {{
        use log::error;
        let err = format!($($t)*);
        error!("{err}");
        Err(ValidationError::custom(
            Location::default(),
            Location::default(),
            &Value::Null,
            err,
        ))
    }};
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Args {
    cmd_schema:                bool,
    flag_trim:                 bool,
    flag_no_format_validation: bool,
    flag_fail_fast:            bool,
    flag_valid:                Option<String>,
    flag_invalid:              Option<String>,
    flag_json:                 bool,
    flag_pretty_json:          bool,
    flag_valid_output:         Option<String>,
    flag_jobs:                 Option<usize>,
    flag_batch:                usize,
    flag_no_headers:           bool,
    flag_delimiter:            Option<Delimiter>,
    flag_progressbar:          bool,
    flag_quiet:                bool,
    arg_input:                 Option<String>,
    arg_json_schema:           Option<String>,
    flag_fancy_regex:          bool,
    flag_backtrack_limit:      usize,
    flag_size_limit:           usize,
    flag_dfa_size_limit:       usize,
    flag_timeout:              u16,
    flag_cache_dir:            String,
    flag_ckan_api:             String,
    flag_ckan_token:           Option<String>,
}

enum JSONtypes {
    String,
    Number,
    Integer,
    Boolean,
    Unsupported,
}

#[derive(Serialize, Deserialize)]
struct RFC4180Struct {
    delimiter_char: char,
    header_row:     bool,
    quote_char:     char,
    num_records:    u64,
    num_fields:     u64,
    fields:         Vec<String>,
}

impl From<ValidationError<'_>> for CliError {
    fn from(err: ValidationError) -> CliError {
        CliError::Other(format!("{err}"))
    }
}

#[inline]
/// Checks if a given string represents a valid currency format.
fn currency_format_checker(s: &str) -> bool {
    Currency::from_str(s).is_ok_and(|c| {
        if c.symbol().is_empty() {
            true // allow empty currency symbol
        } else {
            qsv_currency::Currency::is_iso_currency(&c)
        }
    })
}

struct DynEnumValidator {
    dynenum_set: HashSet<String>,
}

impl DynEnumValidator {
    #[allow(dead_code)]
    const fn new(dynenum_set: HashSet<String>) -> Self {
        Self { dynenum_set }
    }
}

impl Keyword for DynEnumValidator {
    #[inline]
    fn validate<'instance>(
        &self,
        instance: &'instance Value,
        instance_path: &LazyLocation,
    ) -> Result<(), ValidationError<'instance>> {
        if self.dynenum_set.contains(instance.as_str().unwrap()) {
            Ok(())
        } else {
            let error = ValidationError::custom(
                Location::default(),
                instance_path.into(),
                instance,
                format!("{instance} is not a valid dynamicEnum value"),
            );
            Err(error)
        }
    }

    #[inline]
    fn is_valid(&self, instance: &Value) -> bool {
        if let Value::String(s) = instance {
            self.dynenum_set.contains(s)
        } else {
            false
        }
    }
}

struct UniqueCombinedWithValidator {
    column_names:      Vec<String>,
    column_indices:    Vec<usize>,
    seen_combinations: std::sync::RwLock<HashSet<String>>,
}

impl UniqueCombinedWithValidator {
    fn new(column_names: Vec<String>, column_indices: Vec<usize>) -> Self {
        Self {
            column_names,
            column_indices,
            seen_combinations: std::sync::RwLock::new(HashSet::new()),
        }
    }
}

impl Keyword for UniqueCombinedWithValidator {
    fn validate<'instance>(
        &self,
        instance: &'instance Value,
        instance_path: &LazyLocation,
    ) -> Result<(), ValidationError<'instance>> {
        let obj = instance.as_object().ok_or_else(|| {
            ValidationError::custom(
                Location::default(),
                instance_path.into(),
                instance,
                "Instance must be an object",
            )
        })?;

        let mut values = Vec::with_capacity(self.column_names.len() + self.column_indices.len());

        // Get values from column names
        for name in &self.column_names {
            if let Some(value) = obj.get(name) {
                values.push(value.to_string());
            }
        }

        // Get values from column indices by converting to array first
        if !self.column_indices.is_empty() {
            let array: Vec<_> = obj.values().collect();
            for &idx in &self.column_indices {
                if let Some(value) = array.get(idx) {
                    values.push(value.to_string());
                }
            }
        }

        let combination = values.join("|");
        let mut seen = self.seen_combinations.write().unwrap();

        if seen.contains(&combination) {
            let mut column_desc_parts =
                Vec::with_capacity(self.column_names.len() + self.column_indices.len());

            // Add named columns
            if !self.column_names.is_empty() {
                column_desc_parts.extend(self.column_names.iter().cloned());
            }

            // Add indexed columns
            if !self.column_indices.is_empty() {
                column_desc_parts.extend(
                    self.column_indices
                        .iter()
                        .map(std::string::ToString::to_string),
                );
            }

            let column_desc = column_desc_parts.join(", ");
            return Err(ValidationError::custom(
                Location::default(),
                instance_path.into(),
                instance,
                format!("Combination of values for columns {column_desc} is not unique"),
            ));
        }

        seen.insert(combination);
        drop(seen);
        Ok(())
    }

    fn is_valid(&self, instance: &Value) -> bool {
        let Some(obj) = instance.as_object() else {
            return false;
        };

        let mut values = Vec::with_capacity(self.column_names.len() + self.column_indices.len());

        // Get values from column names
        for name in &self.column_names {
            if let Some(value) = obj.get(name) {
                values.push(value.to_string());
            }
        }

        // Get values from column indices by converting to array first
        if !self.column_indices.is_empty() {
            let array: Vec<_> = obj.values().collect();
            for &idx in &self.column_indices {
                if let Some(value) = array.get(idx) {
                    values.push(value.to_string());
                }
            }
        }

        let combination = values.join("|");
        let seen = self.seen_combinations.read().unwrap();
        !seen.contains(&combination)
    }
}

#[allow(clippy::result_large_err)]
fn unique_combined_with_validator_factory<'a>(
    _parent: &'a Map<String, Value>,
    value: &'a Value,
    location: Location,
) -> Result<Box<dyn Keyword>, ValidationError<'a>> {
    // Get the array of column names/indices
    let columns = value.as_array().ok_or_else(|| {
        ValidationError::custom(
            Location::default(),
            location.clone(),
            value,
            "'uniqueCombinedWith' must be an array of column names or indices",
        )
    })?;

    let col_len = columns.len();
    let mut column_names = Vec::with_capacity(col_len);
    let mut column_indices = Vec::with_capacity(col_len);

    // Convert each column name/index to appropriate type
    for col in columns {
        // Try parsing as index first
        if let Some(idx) = col.as_u64() {
            column_indices.push(idx as usize);
        } else {
            // Try as string
            let name = col.as_str().ok_or_else(|| {
                ValidationError::custom(
                    Location::default(),
                    location.clone(),
                    col,
                    "Column names must be strings or numbers",
                )
            })?;
            column_names.push(name.to_string());
        }
    }

    // Validate that we have at least one column
    if column_names.is_empty() && column_indices.is_empty() {
        return Err(ValidationError::custom(
            Location::default(),
            location,
            value,
            "'uniqueCombinedWith' must specify at least one column",
        ));
    }

    Ok(Box::new(UniqueCombinedWithValidator::new(
        column_names,
        column_indices,
    )))
}

/// Parse the dynamicEnum URI string to extract cache_name, final_uri, cache_age and column
/// Format: "[cache_name;cache_age]|URL[|column]" where cache_name, cache_age and column are
/// optional
///
/// # Arguments
/// * `uri` - The dynamicEnum URI string to parse
///
/// # uri parsing examples:
/// lookup.csv
///    - cache_name: lookup, final_uri: lookup.csv, cache_age: 3600, column: None
///
/// lookup.csv|name
///    - cache_name: lookup, final_uri: lookup.csv, cache_age: 3600, column: Some(name)
///
/// lookup_name;600|lookup.csv
///    - cache_name: lookup_name, final_uri: lookup.csv, cache_age: 600, column: None
///
/// remote_lookup|https://example.com/remote.csv|col1
///    - cache_name: remote_lookup, final_uri: https://example.com/remote.csv, cache_age: 3600,
///      column: Some(col1)
///
/// https://example.com/remote.csv
///    - cache_name: remote, final_uri: https://example.com/remote.csv, cache_age: 3600, column:
///      None
///
/// # Returns
/// * `(String, String, i64, Option<String>)` - Tuple containing:
///   - cache_name: Name to use for caching the lookup table
///   - final_uri: The actual URI/URL to load the lookup table from
///   - cache_age: How long to cache the lookup table in seconds
///   - column: Optional column name/index to use from the lookup table
#[cfg(not(feature = "lite"))]
fn parse_dynenum_uri(uri: &str) -> (String, String, i64, Option<String>) {
    const DEFAULT_CACHE_AGE_SECS: i64 = 3600; // 1 hour

    // Extract cache name from URI (handles both URLs and local files)
    fn get_cache_name(uri: &str) -> String {
        // For URIs with schemes (http://, dathere://, ckan://, etc.)
        if uri.contains("://") {
            // Split on "://" and take everything after it
            let after_scheme = uri.split("://").nth(1).unwrap_or(uri);
            // Then take the last part of the path and remove .csv
            after_scheme
                .split('/')
                .next_back()
                .unwrap_or(after_scheme)
                .trim_end_matches(".csv")
                .to_string()
        } else {
            // For regular paths, split on both / and \ and take the last part
            uri.split(['/', '\\'])
                .next_back()
                .unwrap_or(uri)
                .trim_end_matches(".csv")
                .to_string()
        }
    }

    // Handle simple URL case with no pipe separators
    if !uri.contains('|') {
        let final_uri = uri.to_string();
        let cache_name = get_cache_name(&final_uri);
        return (cache_name, final_uri, DEFAULT_CACHE_AGE_SECS, None);
    }

    // Split the URI into parts
    let parts: Vec<&str> = uri.split('|').collect();

    // Get the final URI and handle cache configuration
    let (final_uri, cache_name, cache_age) = if parts[0].contains(';') {
        // Has cache config: "name;age|uri"
        let config_parts: Vec<&str> = parts[0].split(';').collect();
        let name = if config_parts[0].is_empty() {
            get_cache_name(parts[1])
        } else {
            config_parts[0].trim_end_matches(".csv").to_string()
        };
        let age = config_parts
            .get(1)
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(DEFAULT_CACHE_AGE_SECS);
        (parts[1].to_string(), name, age)
    } else if parts[1].contains("://") {
        // Has URL/scheme: "name|scheme://uri"
        (
            parts[1].to_string(),
            get_cache_name(parts[0]),
            DEFAULT_CACHE_AGE_SECS,
        )
    } else {
        // Simple case: "uri|column"
        (
            parts[0].to_string(),
            get_cache_name(parts[0]),
            DEFAULT_CACHE_AGE_SECS,
        )
    };

    // Extract column if present (last part if it's not the URI)
    let column = if parts.len() > 2 {
        Some(parts[2].to_string())
    } else if parts.len() == 2
        && !parts[1].contains("://")
        && !parts[1].to_lowercase().ends_with(".csv")
    {
        Some(parts[1].to_string())
    } else {
        None
    };

    (cache_name, final_uri, cache_age, column)
}

#[cfg(not(feature = "lite"))]
#[test]
fn test_parse_dynenum_uri() {
    // Test simple URL with no pipe separators
    let (cache_name, uri, cache_age, column) = parse_dynenum_uri("https://example.com/data.csv");
    assert_eq!(cache_name, "data");
    assert_eq!(uri, "https://example.com/data.csv");
    assert_eq!(cache_age, 3600);
    assert_eq!(column, None);

    // Test with custom cache name and age
    let (cache_name, uri, cache_age, column) =
        parse_dynenum_uri("custom_name;600|https://example.com/data.csv");
    assert_eq!(cache_name, "custom_name");
    assert_eq!(uri, "https://example.com/data.csv");
    assert_eq!(cache_age, 600);
    assert_eq!(column, None);

    // Test with column name
    let (cache_name, uri, cache_age, column) = parse_dynenum_uri("lookup.csv|name");
    assert_eq!(cache_name, "lookup");
    assert_eq!(uri, "lookup.csv");
    assert_eq!(cache_age, 3600);
    assert_eq!(column, Some("name".to_string()));

    // Test with cache config and column
    let (cache_name, uri, cache_age, column) = parse_dynenum_uri("MyCache;1800|lookup.csv|code");
    assert_eq!(cache_name, "MyCache");
    assert_eq!(uri, "lookup.csv");
    assert_eq!(cache_age, 1800);
    assert_eq!(column, Some("code".to_string()));

    // Test empty cache name with age and column
    let (cache_name, uri, cache_age, column) = parse_dynenum_uri(";1800|lookup.csv|code");
    assert_eq!(cache_name, "lookup");
    assert_eq!(uri, "lookup.csv");
    assert_eq!(cache_age, 1800);
    assert_eq!(column, Some("code".to_string()));

    // Test empty cache name with age but no column
    let (cache_name, uri, cache_age, column) = parse_dynenum_uri(";1800|lookup.csv");
    assert_eq!(cache_name, "lookup");
    assert_eq!(uri, "lookup.csv");
    assert_eq!(cache_age, 1800);
    assert_eq!(column, None);

    // Test simple local file path
    let (cache_name, uri, cache_age, column) = parse_dynenum_uri("lookup.csv");
    assert_eq!(cache_name, "lookup");
    assert_eq!(uri, "lookup.csv");
    assert_eq!(cache_age, 3600);
    assert_eq!(column, None);

    // Test simple fully qualified local file path in Windows
    let (cache_name, uri, cache_age, column) = parse_dynenum_uri(r#"c:\Users\jdoe\lookup.csv"#);
    assert_eq!(cache_name, "lookup");
    assert_eq!(uri, r#"c:\Users\jdoe\lookup.csv"#);
    assert_eq!(cache_age, 3600);
    assert_eq!(column, None);

    // Test simple local file path on *nix filesystem, with column
    let (cache_name, uri, cache_age, column) = parse_dynenum_uri("/tmp/lookup.csv|first_col");
    assert_eq!(cache_name, "lookup");
    assert_eq!(uri, "/tmp/lookup.csv");
    assert_eq!(cache_age, 3600);
    assert_eq!(column, Some("first_col".to_string()));

    // Test case-insensitive cache name generation
    let (cache_name, uri, cache_age, column) = parse_dynenum_uri("LookUp.csv");
    assert_eq!(cache_name, "LookUp");
    assert_eq!(uri, "LookUp.csv");
    assert_eq!(cache_age, 3600);
    assert_eq!(column, None);

    // Test CKAN URL with custom cache name
    let (cache_name, uri, cache_age, column) =
        parse_dynenum_uri("NYC_neighborhood_data|ckan://nyc_neighborhoods?");
    assert_eq!(cache_name, "NYC_neighborhood_data");
    assert_eq!(uri, "ckan://nyc_neighborhoods?");
    assert_eq!(cache_age, 3600);
    assert_eq!(column, None);

    // Test CKAN URL with custom cache name and age
    let (cache_name, uri, cache_age, column) =
        parse_dynenum_uri("NYC_neighborhood_data;5000|ckan://nyc_neighborhoods?");
    assert_eq!(cache_name, "NYC_neighborhood_data");
    assert_eq!(uri, "ckan://nyc_neighborhoods?");
    assert_eq!(cache_age, 5000);
    assert_eq!(column, None);

    // Test CKAN URL with custom cache name, age and column
    let (cache_name, uri, cache_age, column) =
        parse_dynenum_uri("NYC_neighborhood_data;5000|ckan://nyc_neighborhoods?|Neighborhood_Col");
    assert_eq!(cache_name, "NYC_neighborhood_data");
    assert_eq!(uri, "ckan://nyc_neighborhoods?");
    assert_eq!(cache_age, 5000);
    assert_eq!(column, Some("Neighborhood_Col".to_string()));

    // Test dathere URL with no options
    let (cache_name, uri, cache_age, column) = parse_dynenum_uri("dathere://us_states.csv");
    assert_eq!(cache_name, "us_states");
    assert_eq!(uri, "dathere://us_states.csv");
    assert_eq!(cache_age, 3600);
    assert_eq!(column, None);

    // Test dathere URL with column
    let (cache_name, uri, cache_age, column) =
        parse_dynenum_uri("dathere://us_states.csv|state_col");
    assert_eq!(cache_name, "us_states");
    assert_eq!(uri, "dathere://us_states.csv");
    assert_eq!(cache_age, 3600);
    assert_eq!(column, Some("state_col".to_string()));

    // Test dathere URL with custom cache name, age and column
    let (cache_name, uri, cache_age, column) =
        parse_dynenum_uri("usl_lookup;6000|dathere://us_states.csv|state_col");
    assert_eq!(cache_name, "usl_lookup");
    assert_eq!(uri, "dathere://us_states.csv");
    assert_eq!(cache_age, 6000);
    assert_eq!(column, Some("state_col".to_string()));
}

/// Factory function that creates a DynEnumValidator for validating against dynamic enums loaded
/// from CSV files.
///
/// This function takes a CSV file path or URL and loads its first column into a HashSet to validate
/// against. The CSV can be loaded from:
/// - Local filesystem
/// - HTTP/HTTPS URLs
/// - CKAN resources (requires --ckan-api and optionally --ckan-token)
/// - datHere lookup tables
///
/// The dynamicEnum value format is: "[cache_name;cache_age]|URL" where cache_name and cache_age are
/// optional. Examples:
/// - "https://example.com/data.csv" - Cache as data.csv with 1 hour default cache
/// - "custom_name;600|https://example.com/data.csv" - Cache as custom_name.csv for 600 seconds
/// - "resname|ckan://1234567" - Get CKAN resource ID 1234567, cache as resname.csv
///
/// # Arguments
/// * `_parent` - Parent JSON Schema object (unused)
/// * `value` - The dynamicEnum value string specifying the CSV source
/// * `location` - Location in the schema for error reporting
///
/// # Returns
/// * `Ok(Box<DynEnumValidator>)` - Validator initialized with values from first CSV column
/// * `Err(ValidationError)` - If loading/parsing CSV fails or value is not a string
#[cfg(not(feature = "lite"))]
#[allow(clippy::result_large_err)]
fn dyn_enum_validator_factory<'a>(
    _parent: &'a Map<String, Value>,
    value: &'a Value,
    location: Location,
) -> Result<Box<dyn Keyword>, ValidationError<'a>> {
    let uri = value.as_str().ok_or_else(|| {
        ValidationError::custom(
            Location::default(),
            location,
            value,
            "'dynamicEnum' must be set to a CSV file on the local filesystem or on a URL.",
        )
    })?;

    let (lookup_name, final_uri, cache_age_secs, column) = parse_dynenum_uri(uri);

    // Create lookup table options
    let opts = LookupTableOptions {
        name: lookup_name,
        uri: final_uri,
        cache_age_secs,
        cache_dir: QSV_CACHE_DIR.get().unwrap().to_string(),
        delimiter: DELIMITER.get().copied().flatten(),
        ckan_api_url: CKAN_API.get().cloned(),
        ckan_token: CKAN_TOKEN.get().and_then(std::clone::Clone::clone),
        timeout_secs: TIMEOUT_SECS.load(Ordering::Relaxed),
    };

    // Load the lookup table
    let lookup_result = match load_lookup_table(&opts) {
        Ok(result) => result,
        Err(e) => return fail_validation_error!("Error loading dynamicEnum lookup table: {e}"),
    };

    // Read the specified column into a HashSet
    let mut enum_set = HashSet::with_capacity(lookup_result.headers.len());
    let rconfig = Config::new(Some(lookup_result.filepath).as_ref());
    let mut rdr = match rconfig
        .flexible(true)
        .comment(Some(b'#'))
        .skip_format_check(true)
        .reader()
    {
        Ok(reader) => reader,
        Err(e) => return fail_validation_error!("Error opening dynamicEnum file: {e}"),
    };

    // Get column index based on name or default to first column
    let column_idx = if let Some(col_name) = column {
        // Try parsing as index first
        if let Ok(idx) = col_name.parse::<usize>() {
            idx
        } else {
            // Try finding column by name
            match rdr.headers() {
                Ok(headers) => {
                    let idx = headers.iter().position(|h| h == col_name);
                    match idx {
                        Some(i) => i,
                        None => {
                            return fail_validation_error!(
                                "Column '{}' not found in lookup table",
                                col_name
                            );
                        },
                    }
                },
                Err(e) => return fail_validation_error!("Error reading headers: {e}"),
            }
        }
    } else {
        0
    };

    for result in rdr.records() {
        match result {
            Ok(record) => {
                if let Some(value) = record.get(column_idx) {
                    enum_set.insert(value.to_owned());
                }
            },
            Err(e) => return fail_validation_error!("Error reading dynamicEnum file - {e}"),
        }
    }

    Ok(Box::new(DynEnumValidator::new(enum_set)))
}

#[cfg(feature = "lite")]
#[allow(clippy::result_large_err)]
fn dyn_enum_validator_factory<'a>(
    _parent: &'a Map<String, Value>,
    value: &'a Value,
    location: Location,
) -> Result<Box<dyn Keyword>, ValidationError<'a>> {
    if let Value::String(uri) = value {
        let temp_download = match NamedTempFile::new() {
            Ok(file) => file,
            Err(e) => return fail_validation_error!("Failed to create temporary file: {e}"),
        };

        // Split URI to get column specification
        let parts: Vec<&str> = uri.split('|').collect();
        let base_uri = parts[0];
        let column = parts.get(1).map(std::string::ToString::to_string);

        let dynenum_path = if base_uri.starts_with("http") {
            let valid_url = reqwest::Url::parse(base_uri).map_err(|e| {
                ValidationError::custom(
                    Location::default(),
                    location,
                    value,
                    format!("Error parsing dynamicEnum URL: {e}"),
                )
            })?;

            // download the CSV file from the URL
            let download_timeout = TIMEOUT_SECS.load(Ordering::Relaxed);
            let future = util::download_file(
                valid_url.as_str(),
                temp_download.path().to_path_buf(),
                false,
                None,
                Some(download_timeout),
                None,
            );
            match tokio::runtime::Runtime::new() {
                Ok(runtime) => {
                    if let Err(e) = runtime.block_on(future) {
                        return fail_validation_error!("Error downloading dynamicEnum file - {e}");
                    }
                },
                Err(e) => {
                    return fail_validation_error!("Error creating Tokio runtime - {e}");
                },
            }

            temp_download.path().to_str().unwrap().to_string()
        } else {
            // its a local file
            let uri_path = std::path::Path::new(base_uri);
            let uri_exists = uri_path.exists();
            if !uri_exists {
                return fail_validation_error!("dynamicEnum file not found - {base_uri}");
            }
            uri_path.to_str().unwrap().to_string()
        };

        // read the specified column into a HashSet
        let mut enum_set = HashSet::with_capacity(50);
        let rconfig = Config::new(Some(dynenum_path).as_ref());
        let mut rdr = match rconfig
            .flexible(true)
            .comment(Some(b'#'))
            .skip_format_check(true)
            .reader()
        {
            Ok(reader) => reader,
            Err(e) => return fail_validation_error!("Error opening dynamicEnum file: {e}"),
        };

        // Get column index based on name or default to first column
        let column_idx = if let Some(col_name) = column {
            // Try parsing as index first
            if let Ok(idx) = col_name.parse::<usize>() {
                idx
            } else {
                // Try finding column by name
                match rdr.headers() {
                    Ok(headers) => headers.iter().position(|h| h == col_name).unwrap_or(0),
                    Err(_) => 0,
                }
            }
        } else {
            0
        };

        for result in rdr.records() {
            match result {
                Ok(record) => {
                    if let Some(value) = record.get(column_idx) {
                        enum_set.insert(value.to_owned());
                    }
                },
                Err(e) => return fail_validation_error!("Error reading dynamicEnum file - {e}"),
            };
        }

        Ok(Box::new(DynEnumValidator::new(enum_set)))
    } else {
        Err(ValidationError::custom(
            Location::default(),
            location,
            value,
            "'dynamicEnum' must be set to a CSV file on the local filesystem or on a URL.",
        ))
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Is the JSON Schema file valid?
    if args.cmd_schema {
        if let Some(ref schema) = args.arg_json_schema {
            let schema_json_string = load_json(schema)?;
            let schema_json = serde_json::from_str(&schema_json_string)?;
            // First, try_is_valid the JSON Schema
            match jsonschema::meta::try_is_valid(&schema_json) {
                Ok(is_valid) => {
                    if is_valid {
                        // Now, try_validate the JSON Schema
                        let validated = jsonschema::meta::try_validate(&schema_json);
                        match validated {
                            Ok(Ok(())) => {
                                if !args.flag_quiet {
                                    winfo!("Valid JSON Schema.");
                                }
                                return Ok(());
                            },
                            Ok(Err(e)) => {
                                return fail_clierror!("JSON Schema Meta-Validation Error: {e}");
                            },
                            Err(e) => {
                                return fail_clierror!("JSON Schema Meta-Reference Error: {e}");
                            },
                        }
                    }
                    return fail_clierror!("Invalid JSON Schema.");
                },
                Err(e) => {
                    return fail_clierror!("JSON Schema Meta-Reference Error: {e}");
                },
            }
        }
        return fail_clierror!("No JSON Schema file supplied.");
    }

    TIMEOUT_SECS.store(
        util::timeout_secs(args.flag_timeout)? as u16,
        Ordering::Relaxed,
    );

    let mut rconfig = Config::new(args.arg_input.as_ref())
        .no_headers(args.flag_no_headers)
        .set_read_buffer(if std::env::var("QSV_RDR_BUFFER_CAPACITY").is_err() {
            DEFAULT_RDR_BUFFER_CAPACITY * 10
        } else {
            DEFAULT_RDR_BUFFER_CAPACITY
        });

    if args.flag_delimiter.is_some() {
        rconfig = rconfig.delimiter(args.flag_delimiter);
    }
    DELIMITER.set(args.flag_delimiter).unwrap();

    let mut rdr = rconfig.reader()?;

    // if no JSON Schema supplied, only let csv reader RFC4180-validate csv file
    if args.arg_json_schema.is_none() {
        // just read csv file and let csv reader report problems
        // since we're using csv::StringRecord, this will also detect non-utf8 sequences

        let flag_json = args.flag_json || args.flag_pretty_json;
        let flag_pretty_json = args.flag_pretty_json;

        // first, let's validate the header row
        let mut header_msg = String::new();
        let mut header_len = 0_usize;
        let mut field_vec: Vec<String> = Vec::new();
        if !args.flag_no_headers {
            let fields_result = rdr.headers();
            match fields_result {
                Ok(fields) => {
                    header_len = fields.len();
                    field_vec.reserve(header_len);
                    for field in fields {
                        field_vec.push(field.to_string());
                    }
                    let field_list = field_vec.join(r#"", ""#);
                    header_msg = format!(
                        "{} Columns: (\"{field_list}\");",
                        HumanCount(header_len as u64)
                    );
                },
                Err(e) => {
                    // we're returning a JSON error for the header,
                    // so we have more machine-friendly details
                    if flag_json {
                        // there's a UTF-8 error, so we report utf8 error metadata
                        if let csv::ErrorKind::Utf8 { pos, err } = e.kind() {
                            let header_error = json!({
                                "errors": [{
                                    "title" : "Header UTF-8 validation error",
                                    "detail" : format!("{e}"),
                                    "meta": {
                                        "record_position": format!("{pos:?}"),
                                        "record_error": format!("{err}"),
                                    }
                                }]
                            });
                            let json_error = if flag_pretty_json {
                                serde_json::to_string_pretty(&header_error).unwrap()
                            } else {
                                header_error.to_string()
                            };

                            return fail_encoding_clierror!("{json_error}");
                        }
                        // it's not a UTF-8 error, so we report a generic
                        // header validation error
                        let header_error = json!({
                            "errors": [{
                                "title" : "Header Validation error",
                                "detail" : format!("{e}"),
                            }]
                        });
                        let json_error = if flag_pretty_json {
                            serde_json::to_string_pretty(&header_error).unwrap()
                        } else {
                            header_error.to_string()
                        };
                        return fail_encoding_clierror!("{json_error}");
                    }
                    // we're not returning a JSON error, so we can use
                    // a user-friendly error message with suggestions
                    if let csv::ErrorKind::Utf8 { pos, err } = e.kind() {
                        return fail_encoding_clierror!(
                            "non-utf8 sequence detected in header, position {pos:?}.\n{err}\nUse \
                             `qsv input` to fix formatting and to handle non-utf8 sequences.\n
                             Alternatively, transcode your data to UTF-8 first using `iconv` or \
                             `recode`."
                        );
                    }
                    // its not a UTF-8 error, report a generic header validation error
                    return fail_clierror!("Header Validation error: {e}.");
                },
            }
        }

        // Now, let's validate the rest of the records the fastest way possible.
        // We do this by using csv::ByteRecord, which does not validate utf8
        // making for higher throughput and lower memory usage compared to csv::StringRecord
        // which validates each field SEPARATELY as a utf8 string.
        // Combined with simdutf8::basic::from_utf8(), we utf8-validate the entire record in one go
        // as a slice of bytes, this approach is much faster than csv::StringRecord's
        // per-field validation.
        let mut record = csv::ByteRecord::with_capacity(500, header_len);
        let mut result;
        let mut record_idx: u64 = 0;

        'rfc4180_check: loop {
            result = rdr.read_byte_record(&mut record);
            if let Err(e) = result {
                // read_byte_record() does not validate utf8, so we know this is not a utf8 error
                if flag_json {
                    // we're returning a JSON error, so we have more machine-friendly details
                    // using the JSON API error format

                    let validation_error = json!({
                        "errors": [{
                            "title" : "Validation error",
                            "detail" : format!("{e}"),
                            "meta": {
                                "last_valid_record": format!("{record_idx}"),
                            }
                        }]
                    });

                    let json_error = if flag_pretty_json {
                        serde_json::to_string_pretty(&validation_error).unwrap()
                    } else {
                        validation_error.to_string()
                    };

                    return fail!(json_error);
                }

                // we're not returning a JSON error, so we can use a
                // user-friendly error message with a fixlengths suggestion
                if let csv::ErrorKind::UnequalLengths {
                    expected_len: _,
                    len: _,
                    pos: _,
                } = e.kind()
                {
                    return fail_clierror!(
                        "Validation error: {e}.\nUse `qsv fixlengths` to fix record length issues."
                    );
                }
                return fail_clierror!("Validation error: {e}.\nLast valid record: {record_idx}");
            }

            // use SIMD accelerated UTF-8 validation, validate the entire record in one go
            if simdutf8::basic::from_utf8(record.as_slice()).is_err() {
                // there's a UTF-8 error, so we report utf8 error metadata
                if flag_json {
                    let validation_error = json!({
                        "errors": [{
                            "title" : "UTF-8 validation error",
                            "detail" : "Cannot parse CSV record as UTF-8",
                            "meta": {
                                "last_valid_record": format!("{record_idx}"),
                                "invalid_record": format!("{record:?}"),
                            }
                        }]
                    });

                    let json_error = if flag_pretty_json {
                        serde_json::to_string_pretty(&validation_error).unwrap()
                    } else {
                        validation_error.to_string()
                    };
                    return fail_encoding_clierror!("{json_error}");
                }
                // we're not returning a JSON error, so we can use a
                // user-friendly error message with utf8 transcoding suggestions
                return fail_encoding_clierror!(
                    r#"non-utf8 sequence at record {record_idx}.
Invalid record: {record:?}
Use `qsv input` to fix formatting and to handle non-utf8 sequences.
Alternatively, transcode your data to UTF-8 first using `iconv` or `recode`."#
                );
            }

            if result.is_ok_and(|more_data| !more_data) {
                // we've read the CSV to the end, so break out of loop
                break 'rfc4180_check;
            }
            record_idx += 1;
        } // end rfc4180_check loop

        // if we're here, we know the CSV is valid
        let msg = if flag_json {
            let rfc4180 = RFC4180Struct {
                delimiter_char: rconfig.get_delimiter() as char,
                header_row:     !rconfig.no_headers,
                quote_char:     rconfig.quote as char,
                num_records:    record_idx,
                num_fields:     header_len as u64,
                fields:         field_vec,
            };

            if flag_pretty_json {
                serde_json::to_string_pretty(&rfc4180).unwrap()
            } else {
                serde_json::to_string(&rfc4180).unwrap()
            }
        } else {
            let delim_display = if rconfig.get_delimiter() == b'\t' {
                "TAB".to_string()
            } else {
                (rconfig.get_delimiter() as char).to_string()
            };
            format!(
                "Valid: {header_msg} Records: {}; Delimiter: {delim_display}",
                HumanCount(record_idx)
            )
        };
        if !args.flag_quiet {
            woutinfo!("{msg}");
        }

        // we're done when validating without a schema
        return Ok(());
    }

    // if we're here, we're validating with a JSON Schema
    // JSONSchema validation requires headers
    if args.flag_no_headers {
        return fail_clierror!("Cannot validate CSV without headers against a JSON Schema.");
    }

    // prep progress bar
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();

    // for full row count, prevent CSV reader from aborting on inconsistent column count
    rconfig = rconfig.flexible(true);
    let record_count = util::count_rows(&rconfig)?;
    rconfig = rconfig.flexible(false);

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        util::prep_progress(&progress, record_count);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    let headers = rdr.byte_headers()?.clone();
    let header_len = headers.len();

    #[cfg(not(feature = "lite"))]
    let qsv_cache_dir = lookup::set_qsv_cache_dir(&args.flag_cache_dir)?;
    #[cfg(not(feature = "lite"))]
    QSV_CACHE_DIR.set(qsv_cache_dir)?;

    // check the QSV_CKAN_API environment variable
    #[cfg(not(feature = "lite"))]
    CKAN_API.set(if let Ok(api) = std::env::var("QSV_CKAN_API") {
        api
    } else {
        args.flag_ckan_api.clone()
    })?;

    // check the QSV_CKAN_TOKEN environment variable
    #[cfg(not(feature = "lite"))]
    CKAN_TOKEN
        .set(if let Ok(token) = std::env::var("QSV_CKAN_TOKEN") {
            Some(token)
        } else {
            args.flag_ckan_token.clone()
        })
        .unwrap();

    // parse and compile supplied JSON Schema
    let (schema_json, schema_compiled): (Value, Validator) =
        // safety: we know the schema is_some() because we checked above
        match load_json(&args.arg_json_schema.clone().unwrap()) {
            Ok(s) => {
                // Check for custom formats and keywords before parsing
                let has_currency_format = s.contains(r#""format": "currency""#);
                let has_dynamic_enum = s.contains("dynamicEnum");
                let has_unique_combined = s.contains("uniqueCombinedWith");

                // parse JSON string
                let mut s_slice = s.as_bytes().to_vec();
                match simd_json::serde::from_slice::<Value>(&mut s_slice) {
                    Ok(json) => {
                        // compile JSON Schema
                        let mut validator_options = Validator::options()
                            .should_validate_formats(!args.flag_no_format_validation);

                        // Add custom validators based on pre-checked flags
                        if has_currency_format {
                            validator_options = validator_options.with_format("currency", currency_format_checker);
                        }

                        if has_dynamic_enum {
                            validator_options = validator_options.with_keyword("dynamicEnum", dyn_enum_validator_factory);
                        }

                        if has_unique_combined {
                            validator_options = validator_options.with_keyword("uniqueCombinedWith", unique_combined_with_validator_factory);
                        }

                        if args.flag_fancy_regex {
                            let fancy_regex_options = PatternOptions::fancy_regex()
                                .backtrack_limit(args.flag_backtrack_limit)
                                .size_limit(args.flag_size_limit * (1 << 20))
                                .dfa_size_limit(args.flag_dfa_size_limit * (1 << 20));
                            validator_options = validator_options.with_pattern_options(fancy_regex_options);
                        } else {
                            let regex_options = PatternOptions::regex()
                                .size_limit(args.flag_size_limit * (1 << 20))
                                .dfa_size_limit(args.flag_dfa_size_limit * (1 << 20));
                            validator_options = validator_options.with_pattern_options(regex_options);
                        }

                        match validator_options.build(&json) {
                            Ok(schema) => (json, schema),
                            Err(e) => {
                                return fail_clierror!(r#"Cannot compile JSONschema. error: {e}
Try running `qsv validate schema {}` to check the JSON Schema file."#, args.arg_json_schema.unwrap());
                            },
                        }
                    },
                    Err(e) => {
                        return fail_clierror!(r#"Unable to parse JSONschema. error: {e}
Try running `qsv validate schema {}` to check the JSON Schema file."#, args.arg_json_schema.unwrap());
                    },
                }
            },
            Err(e) => {
                return fail_clierror!("Unable to retrieve JSONschema. error: {e}");
            },
        };

    if log::log_enabled!(log::Level::Debug) {
        // only log if debug is enabled
        // as it can be quite large and expensive to deserialize the schema
        debug!("schema json: {:?}", &schema_json);
    }

    // set this once, as this is used repeatedly in a hot loop
    NULL_TYPE.set(Value::String("null".to_string())).unwrap();

    // get JSON types for each column in CSV file
    let header_types = get_json_types(&headers, &schema_json)?;

    // how many rows read and processed as batches
    let mut row_number: u64 = 0;
    // how many invalid rows found
    let mut invalid_count: u64 = 0;

    // amortize memory allocation by reusing record
    let mut record = csv::ByteRecord::with_capacity(500, header_len);

    let num_jobs = util::njobs(args.flag_jobs);

    // amortize allocations
    let mut valid_flags: BitVec = BitVec::with_capacity(record_count as usize);
    let batch_size = util::optimal_batch_size(&rconfig, args.flag_batch, num_jobs);
    let mut batch = Vec::with_capacity(batch_size);
    let mut batch_validation_results: Vec<Option<String>> = Vec::with_capacity(batch_size);
    let mut validation_error_messages: Vec<String> = Vec::with_capacity(50);
    let flag_trim = args.flag_trim;
    let flag_fail_fast = args.flag_fail_fast;
    let mut itoa_buffer = itoa::Buffer::new();
    let batch_pariter_min_len = batch_size / num_jobs;

    // main loop to read CSV and construct batches for parallel processing.
    // each batch is processed via Rayon parallel iterator.
    // loop exits when batch is empty.
    'batch_loop: loop {
        for _ in 0..batch_size {
            match rdr.read_byte_record(&mut record) {
                Ok(true) => {
                    row_number += 1;
                    record.push_field(itoa_buffer.format(row_number).as_bytes());
                    if flag_trim {
                        record.trim();
                    }
                    // we use mem::take() to avoid cloning & clearing the record
                    batch.push(std::mem::take(&mut record));
                },
                Ok(false) => break, // nothing else to add to batch
                Err(e) => {
                    return fail_clierror!("Error reading row: {row_number}: {e}");
                },
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break 'batch_loop;
        }

        // do actual validation via Rayon parallel iterator
        // validation_results vector should have same row count and in same order as input CSV
        batch
            .par_iter()
            .with_min_len(batch_pariter_min_len)
            .map(|record| {
                // convert CSV record to JSON instance
                let json_instance = match to_json_instance(&header_types, header_len, record) {
                    Ok(obj) => obj,
                    Err(e) => {
                        // Only convert to string when we have an error
                        // safety: row number was added as last column. We can do index access, not
                        // use get(), and unwrap_unchecked safely since we know its there
                        let row_number_string = unsafe {
                            simdutf8::basic::from_utf8(&record[header_len]).unwrap_unchecked()
                        };
                        return Some(format!("{row_number_string}\t<RECORD>\t{e}"));
                    },
                };

                // validate JSON instance against JSON Schema
                match schema_compiled.apply(&json_instance).basic() {
                    BasicOutput::Valid(_) => None,
                    BasicOutput::Invalid(errors) => {
                        // Only convert to string when we have validation errors
                        // safety: see safety comment above
                        let row_number_string = unsafe {
                            simdutf8::basic::from_utf8(&record[header_len]).unwrap_unchecked()
                        };

                        // Preallocate the vector with the known size
                        let mut error_messages = Vec::with_capacity(errors.len());

                        // there can be multiple validation errors for a single record,
                        // squash multiple errors into one long String with linebreaks
                        for e in errors {
                            error_messages.push(format!(
                                "{row_number_string}\t{field}\t{error}",
                                field = e.instance_location().as_str().trim_start_matches('/'),
                                error = e.error_description()
                            ));
                        }
                        Some(error_messages.join("\n"))
                    },
                }
            })
            .collect_into_vec(&mut batch_validation_results);

        // write to validation error report, but keep Vec<bool> to gen valid/invalid files later
        // because Rayon collect() guarantees original order, we can sequentially append results
        // to vector with each batch
        let start_idx = valid_flags.len();
        valid_flags.extend(std::iter::repeat_n(true, batch_size));
        for (i, result) in batch_validation_results.iter().enumerate() {
            if let Some(validation_error_msg) = result {
                invalid_count += 1;
                unsafe { valid_flags.set_unchecked(start_idx + i, false) };
                validation_error_messages.push(validation_error_msg.to_owned());
            }
        }

        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.inc(batch.len() as u64);
        }
        batch.clear();

        // for fail-fast, exit loop if batch has any error
        if flag_fail_fast && invalid_count > 0 {
            break 'batch_loop;
        }
    } // end batch loop

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        progress.set_message(format!(
            " validated {} records.",
            HumanCount(progress.length().unwrap())
        ));
        util::finish_progress(&progress);
    }

    if invalid_count == 0 {
        // no invalid records found
        // see if we need to pass all valid records to output
        if let Some(valid_output) = args.flag_valid_output {
            // pass all valid records to output and return exit code 1
            let valid_path = if valid_output == "-" {
                // write to stdout
                None
            } else {
                Some(valid_output)
            };

            let mut valid_wtr = Config::new(valid_path.as_ref()).writer()?;
            valid_wtr.write_byte_record(&headers)?;

            let mut rdr = rconfig.reader()?;
            let mut record = csv::ByteRecord::new();
            while rdr.read_byte_record(&mut record)? {
                valid_wtr.write_byte_record(&record)?;
            }
            valid_wtr.flush()?;
            // return 1 as an exitcode and the number of valid rows to stderr
            return fail_clierror!("{row_number}");
        }
    } else {
        // there are invalid records. write out invalid/valid/errors output files.
        // if 100% invalid, valid file isn't needed, but this is rare so OK creating empty file.
        woutinfo!("Writing invalid/valid/error files...");

        let input_path = args
            .arg_input
            .clone()
            .unwrap_or_else(|| "stdin.csv".to_string());

        write_error_report(&input_path, validation_error_messages)?;

        let valid_suffix = args.flag_valid.unwrap_or_else(|| "valid".to_string());
        let invalid_suffix = args.flag_invalid.unwrap_or_else(|| "invalid".to_string());

        split_invalid_records(
            &rconfig,
            &valid_flags[..],
            &headers,
            &input_path,
            &valid_suffix,
            &invalid_suffix,
        )?;

        // done with validation; print output
        let fail_fast_msg = if args.flag_fail_fast {
            format!(
                "fail-fast enabled. stopped after row {}.\n",
                HumanCount(row_number)
            )
        } else {
            String::new()
        };

        return fail_clierror!(
            "{fail_fast_msg}{} out of {} records invalid.",
            HumanCount(invalid_count),
            HumanCount(row_number)
        );
    }

    if !args.flag_quiet {
        winfo!("All {} records valid.", HumanCount(row_number));
    }
    Ok(())
}

fn split_invalid_records(
    rconfig: &Config,
    valid_flags: &BitSlice,
    headers: &ByteRecord,
    input_path: &str,
    valid_suffix: &str,
    invalid_suffix: &str,
) -> CliResult<()> {
    // track how many rows read for splitting into valid/invalid
    // should not exceed row_number when aborted early due to fail-fast
    let mut split_row_num: usize = 0;

    // prepare output writers
    let mut valid_wtr =
        Config::new(Some(input_path.to_owned() + "." + valid_suffix).as_ref()).writer()?;
    valid_wtr.write_byte_record(headers)?;

    let mut invalid_wtr =
        Config::new(Some(input_path.to_owned() + "." + invalid_suffix).as_ref()).writer()?;
    invalid_wtr.write_byte_record(headers)?;

    let mut rdr = rconfig.reader()?;

    let valid_flags_len = valid_flags.len();

    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        // length of valid_flags is max number of rows we can split
        if split_row_num > valid_flags_len {
            break;
        }

        if valid_flags[split_row_num] {
            valid_wtr.write_byte_record(&record)?;
        } else {
            invalid_wtr.write_byte_record(&record)?;
        }
        split_row_num += 1;
    }

    valid_wtr.flush()?;
    invalid_wtr.flush()?;

    Ok(())
}

fn write_error_report(input_path: &str, validation_error_messages: Vec<String>) -> CliResult<()> {
    let wtr_capacitys = env::var("QSV_WTR_BUFFER_CAPACITY")
        .unwrap_or_else(|_| DEFAULT_WTR_BUFFER_CAPACITY.to_string());
    let wtr_buffer_size: usize = wtr_capacitys.parse().unwrap_or(DEFAULT_WTR_BUFFER_CAPACITY);

    let output_file = File::create(input_path.to_owned() + ".validation-errors.tsv")?;

    let mut output_writer = BufWriter::with_capacity(wtr_buffer_size, output_file);

    output_writer.write_all(b"row_number\tfield\terror\n")?;

    // write out error report
    for error_msg in validation_error_messages {
        output_writer.write_all(error_msg.as_bytes())?;
        // since writer is buffered, it's more efficient to do additional write than append Newline
        // to message
        output_writer.write_all(b"\n")?;
    }

    // flush error report; file gets closed automagically when out-of-scope
    output_writer.flush()?;

    Ok(())
}

/// convert CSV Record into JSON instance by referencing JSON types
#[inline]
fn to_json_instance(
    header_types: &[(String, JSONtypes)],
    header_len: usize,
    record: &ByteRecord,
) -> CliResult<Value> {
    let mut json_object_map = Map::with_capacity(header_len);

    let mut json_value;

    for ((key, json_type), value) in header_types.iter().zip(record.iter()) {
        if value.is_empty() {
            json_object_map.insert(key.clone(), Value::Null);
            continue;
        }

        json_value = match json_type {
            JSONtypes::String => {
                if let Ok(v) = simdutf8::basic::from_utf8(value) {
                    Value::String(v.to_owned())
                } else {
                    // don't return an error if the string fails utf8 validation
                    // send the lossy utf8 value
                    Value::String(String::from_utf8_lossy(value).into_owned())
                }
            },
            JSONtypes::Number => {
                if let Ok(float) = fast_float2::parse(value) {
                    Value::Number(Number::from_f64(float).unwrap_or_else(|| Number::from(0)))
                } else {
                    return fail_clierror!(
                        "Can't cast to Number. key: {key}, value: {}",
                        String::from_utf8_lossy(value)
                    );
                }
            },
            JSONtypes::Integer => {
                if let Ok(int) = atoi_simd::parse::<i64>(value) {
                    Value::Number(Number::from(int))
                } else {
                    return fail_clierror!(
                        "Can't cast to Integer. key: {key}, value: {}",
                        String::from_utf8_lossy(value)
                    );
                }
            },
            JSONtypes::Boolean => match value {
                b"true" | b"1" => Value::Bool(true),
                b"false" | b"0" => Value::Bool(false),
                _ => {
                    return fail_clierror!(
                        "Can't cast to Boolean. key: {key}, value: {}",
                        String::from_utf8_lossy(value)
                    );
                },
            },
            JSONtypes::Unsupported => unreachable!("we should never get an unsupported JSON type"),
        };

        json_object_map.insert(key.clone(), json_value);
    }

    Ok(Value::Object(json_object_map))
}

/// get JSON types for each column in CSV file
/// returns a Vector of tuples of column/header name (String) & JSON type (JSONtypes enum)
#[inline]
fn get_json_types(headers: &ByteRecord, schema: &Value) -> CliResult<Vec<(String, JSONtypes)>> {
    // make sure schema has expected structure
    let Some(schema_properties) = schema.get("properties") else {
        return fail_clierror!("JSON Schema missing 'properties' object");
    };

    // safety: we set NULL_TYPE in main() and it's never changed
    let null_type = NULL_TYPE.get().unwrap();

    let mut field_def: &Value;
    let mut field_type_def: &Value;
    let mut json_type: JSONtypes;
    let mut header_types: Vec<(String, JSONtypes)> = Vec::with_capacity(headers.len());

    // iterate over each CSV field and convert to JSON type
    for header in headers {
        let Ok(key) = simdutf8::basic::from_utf8(header) else {
            let s = String::from_utf8_lossy(header);
            return fail_encoding_clierror!("CSV header is not valid UTF-8: {s}");
        };

        field_def = schema_properties.get(key).unwrap_or(&Value::Null);
        field_type_def = field_def.get("type").unwrap_or(&Value::Null);

        json_type = match field_type_def {
            Value::String(s) => match s.as_str() {
                "string" => JSONtypes::String,
                "number" => JSONtypes::Number,
                "integer" => JSONtypes::Integer,
                "boolean" => JSONtypes::Boolean,
                _ => JSONtypes::Unsupported,
            },
            Value::Array(vec) => {
                let mut return_val = JSONtypes::String;
                for val in vec {
                    if *val == *null_type {
                        continue;
                    }
                    return_val = if let Some(s) = val.as_str() {
                        match s {
                            "string" => JSONtypes::String,
                            "number" => JSONtypes::Number,
                            "integer" => JSONtypes::Integer,
                            "boolean" => JSONtypes::Boolean,
                            _ => JSONtypes::Unsupported,
                        }
                    } else {
                        JSONtypes::String
                    };
                }
                return_val
            },
            _ => JSONtypes::String,
        };

        header_types.push((key.to_owned(), json_type));
    }
    Ok(header_types)
}

fn load_json(uri: &str) -> Result<String, String> {
    let json_string = match uri {
        url if url.to_lowercase().starts_with("http") => {
            use reqwest::blocking::Client;

            let client_timeout =
                std::time::Duration::from_secs(TIMEOUT_SECS.load(Ordering::Relaxed) as u64);

            let client = match Client::builder()
                // safety: we're using a validated QSV_USER_AGENT or the default user agent
                .user_agent(util::set_user_agent(None).unwrap())
                .brotli(true)
                .gzip(true)
                .deflate(true)
                .zstd(true)
                .use_rustls_tls()
                .http2_adaptive_window(true)
                .connection_verbose(
                    log_enabled!(log::Level::Debug) || log_enabled!(log::Level::Trace),
                )
                .timeout(client_timeout)
                .build()
            {
                Ok(c) => c,
                Err(e) => {
                    return fail_format!("Cannot build reqwest client: {e}.");
                },
            };

            match client.get(url).send() {
                Ok(response) => response.text().unwrap_or_default(),
                Err(e) => return fail_format!("Cannot read JSON at url {url}: {e}."),
            }
        },
        path => {
            let mut buffer = String::new();
            match File::open(path) {
                Ok(p) => {
                    BufReader::new(p)
                        .read_to_string(&mut buffer)
                        .unwrap_or_default();
                },
                Err(e) => return fail_format!("Cannot read JSON file {path}: {e}."),
            }
            buffer
        },
    };

    Ok(json_string)
}

/// Validate JSON instance against compiled JSON Schema
/// If invalid, returns Some(Vec<(String,String)>) holding the error messages
/// this is just for the tests below and is equivalent to the validation logic
/// in the main `validate` function which was inlined for performance reasons
#[cfg(test)]
fn validate_json_instance(
    instance: &Value,
    schema_compiled: &Validator,
) -> Option<Vec<(String, String)>> {
    match schema_compiled.apply(instance).basic() {
        BasicOutput::Valid(_) => None,
        BasicOutput::Invalid(errors) => Some(
            errors
                .iter()
                .map(|e| {
                    (
                        e.instance_location().to_string(),
                        e.error_description().to_string(),
                    )
                })
                .collect(),
        ),
    }
}

#[cfg(test)]
mod tests_for_csv_to_json_conversion {

    use serde_json::json;

    use super::*;

    /// get schema used for unit tests
    fn schema_json() -> Value {
        // from https://json-schema.org/learn/miscellaneous-examples.html
        serde_json::json!({
            "$id": "https://example.com/test.schema.json",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "test",
            "type": "object",
            "properties": {
                "A": {
                    "type": "string",
                },
                "B": {
                    "type": "number",
                },
                "C": {
                    "type": "integer",
                },
                "D": {
                    "type": "boolean",
                },
                "E": {
                    "type": ["string", "null"],
                },
                "F": {
                    "type": ["number", "null"],
                },
                "G": {
                    "type": ["integer", "null"],
                },
                "H": {
                    "type": ["boolean", "null"],
                },
                "I": {
                    "type": ["string", "null"],
                },
                "J": {
                    "type": ["number", "null"],
                },
                "K": {
                    "type": ["null", "integer"],
                },
                "L": {
                    "type": ["boolean", "null"],
                },
            }
        })
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_to_json_instance() {
        let _ = NULL_TYPE.get_or_init(|| Value::String("null".to_string()));
        let csv = "A,B,C,D,E,F,G,H,I,J,K,L
        hello,3.1415,300000000,true,,,,,hello,3.1415,300000000,true";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();
        let header_types = get_json_types(&headers, &schema_json()).unwrap();
        let mut record = rdr.byte_records().next().unwrap().unwrap();
        record.trim();

        assert_eq!(
            to_json_instance(&header_types, headers.len(), &record)
                .expect("can't convert csv to json instance"),
            json!({
                "A": "hello",
                "B": 3.1415,
                "C": 300_000_000,
                "D": true,
                "E": null,
                "F": null,
                "G": null,
                "H": null,
                "I": "hello",
                "J": 3.1415,
                "K": 300_000_000,
                "L": true,
            })
        );
    }

    #[test]
    fn test_to_json_instance_cast_integer_error() {
        let _ = NULL_TYPE.get_or_init(|| Value::String("null".to_string()));
        let csv = "A,B,C,D,E,F,G,H
        hello,3.1415,3.0e8,true,,,,";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();
        let header_types = get_json_types(&headers, &schema_json()).unwrap();

        let result = to_json_instance(
            &header_types,
            headers.len(),
            &rdr.byte_records().next().unwrap().unwrap(),
        );
        assert!(&result.is_err());
        let error = result.err().unwrap().to_string();
        assert_eq!("Can't cast to Integer. key: C, value: 3.0e8", error);
    }
}

#[cfg(test)]
mod tests_for_schema_validation {
    use super::*;

    fn schema_json() -> Value {
        // from https://json-schema.org/learn/miscellaneous-examples.html
        serde_json::json!({
            "$id": "https://example.com/person.schema.json",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "Person",
            "type": "object",
            "properties": {
                "title": {
                    "type": "string",
                    "description": "The person's title.",
                    "minLength": 2
                },
                "name": {
                    "type": "string",
                    "description": "The person's name.",
                    "minLength": 2
                },
                "age": {
                    "description": "Age in years which must be equal to or greater than 18.",
                    "type": "integer",
                    "minimum": 18
                }
            }
        })
    }

    fn compiled_schema() -> Validator {
        Validator::options()
            .build(&schema_json())
            .expect("Invalid schema")
    }

    #[test]
    fn test_validate_with_no_errors() {
        let _ = NULL_TYPE.get_or_init(|| Value::String("null".to_string()));
        let csv = "title,name,age
        Professor,Xaviers,60";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();
        let header_types = get_json_types(&headers, &schema_json()).unwrap();

        let record = &rdr.byte_records().next().unwrap().unwrap();

        let instance = to_json_instance(&header_types, headers.len(), record).unwrap();

        let result = validate_json_instance(&instance, &compiled_schema());

        assert!(result.is_none());
    }

    #[test]
    fn test_validate_with_error() {
        let _ = NULL_TYPE.get_or_init(|| Value::String("null".to_string()));
        let csv = "title,name,age
        Professor,X,60";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();
        let header_types = get_json_types(&headers, &schema_json()).unwrap();

        let record = &rdr.byte_records().next().unwrap().unwrap();

        let instance = to_json_instance(&header_types, headers.len(), record).unwrap();

        let result = validate_json_instance(&instance, &compiled_schema());

        assert!(result.is_some());

        assert_eq!(
            vec![(
                "/name".to_string(),
                "\"X\" is shorter than 2 characters".to_string()
            )],
            result.unwrap()
        );
    }
}

#[test]
fn test_validate_currency_email_dynamicenum_validator() {
    #[cfg(not(feature = "lite"))]
    let qsv_cache_dir = lookup::set_qsv_cache_dir("~/.qsv-cache").unwrap();
    #[cfg(not(feature = "lite"))]
    QSV_CACHE_DIR.get_or_init(|| qsv_cache_dir);

    fn schema_currency_json() -> Value {
        serde_json::json!({
            "$id": "https://example.com/person.schema.json",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "Person",
            "type": "object",
            "properties": {
                "title": {
                    "type": "string",
                    "description": "The person's title.",
                    "minLength": 2
                },
                "name": {
                    "type": "string",
                    "description": "The person's name.",
                    "minLength": 2
                },
                "fee": {
                    "description": "The required fee to see the person.",
                    "type": "string",
                    "format": "currency",
                },
                "email": {
                    "description": "The person's email.",
                    "type": "string",
                    "format": "email",
                },
                "agency": {
                    "description": "The person's agency.",
                    "type": "string",
                    "dynamicEnum": "https://raw.githubusercontent.com/dathere/qsv/refs/heads/master/scripts/NYC_agencies.csv",
                }
            }
        })
    }

    let _ = NULL_TYPE.get_or_init(|| Value::String("null".to_string()));
    let csv = "title,name,fee
    Professor,Xaviers,Ð 100.00";

    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let headers = rdr.byte_headers().unwrap().clone();
    let header_types = get_json_types(&headers, &schema_currency_json()).unwrap();

    let record = &rdr.byte_records().next().unwrap().unwrap();

    let instance = to_json_instance(&header_types, headers.len(), record).unwrap();

    let compiled_schema = Validator::options()
        .with_format("currency", currency_format_checker)
        .with_keyword("dynamicEnum", dyn_enum_validator_factory)
        .should_validate_formats(true)
        .build(&schema_currency_json())
        .expect("Invalid schema");

    let result = validate_json_instance(&instance, &compiled_schema);

    // Dogecoin is not an ISO currency
    assert_eq!(
        result,
        Some(vec![(
            "/fee".to_owned(),
            "\"Ð 100.00\" is not a \"currency\"".to_owned()
        )])
    );

    let csv = "title,name,fee,email
    Professor,Xaviers,Ð 100.00,thisisnotanemail";

    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let headers = rdr.byte_headers().unwrap().clone();
    let header_types = get_json_types(&headers, &schema_currency_json()).unwrap();

    let record = &rdr.byte_records().next().unwrap().unwrap();

    let instance = to_json_instance(&header_types, headers.len(), record).unwrap();

    let compiled_schema = Validator::options()
        .with_format("currency", currency_format_checker)
        .with_keyword("dynamicEnum", dyn_enum_validator_factory)
        .should_validate_formats(true)
        .build(&schema_currency_json())
        .expect("Invalid schema");

    let result = validate_json_instance(&instance, &compiled_schema);

    assert_eq!(
        result,
        Some(vec![
            (
                "/fee".to_owned(),
                "\"Ð 100.00\" is not a \"currency\"".to_owned()
            ),
            (
                "/email".to_owned(),
                "\"thisisnotanemail\" is not a \"email\"".to_owned()
            )
        ])
    );

    let csv = r#"title,name,fee,email,agency
    Professor,Xaviers,"USD60.02",x@men.com,DOITT
    He-man,Wolverine,"$100.00",claws@men.com,DPR
    Mr,Deadpool,"¥1,000,000.00",landfill@nomail.net,DSNY
    Mrs,T,"-€ 1.000.000,00",t+sheher@t.com,MODA
    Madam,X,"(EUR 1.999.000,12)",x123@aol.com,DOB
    SilicoGod,Vision,"1.000.000,00",singularity+is@here.ai,DOITT
    Dr,Strange,"€ 1.000.000,00",stranger.danger@xmen.com,NYFD
    Dr,Octopus,"WAX 100.000,00",octopussy@bond.net,DFTA
    Mr,Robot,"B 1,000,000",71076.964-compuserve,ABCD"#;

    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let headers = rdr.byte_headers().unwrap().clone();
    let header_types = get_json_types(&headers, &schema_currency_json()).unwrap();

    let compiled_schema = Validator::options()
        .with_format("currency", currency_format_checker)
        .with_keyword("dynamicEnum", dyn_enum_validator_factory)
        .should_validate_formats(true)
        .build(&schema_currency_json())
        .expect("Invalid schema");

    for (i, record) in rdr.byte_records().enumerate() {
        let record = record.unwrap();
        let instance = to_json_instance(&header_types, headers.len(), &record).unwrap();

        let result = validate_json_instance(&instance, &compiled_schema);

        match i {
            0 => assert_eq!(result, None),
            1 => assert_eq!(result, None),
            2 => assert_eq!(result, None),
            3 => assert_eq!(
                result,
                Some(vec![
                    (
                        "/name".to_owned(),
                        "\"T\" is shorter than 2 characters".to_owned()
                    ),
                    (
                        "/agency".to_owned(),
                        "\"MODA\" is not a valid dynamicEnum value".to_owned()
                    )
                ])
            ),
            4 => assert_eq!(
                result,
                Some(vec![(
                    "/name".to_owned(),
                    "\"X\" is shorter than 2 characters".to_owned()
                )])
            ),
            5 => assert_eq!(result, None),
            6 => assert_eq!(
                result,
                Some(vec![(
                    "/agency".to_owned(),
                    "\"NYFD\" is not a valid dynamicEnum value".to_owned()
                )])
            ),
            7 => assert_eq!(
                result,
                Some(vec![(
                    "/fee".to_owned(),
                    "\"WAX 100.000,00\" is not a \"currency\"".to_owned()
                )])
            ),
            8 => assert_eq!(
                result,
                Some(vec![
                    (
                        "/fee".to_owned(),
                        "\"B 1,000,000\" is not a \"currency\"".to_owned()
                    ),
                    (
                        "/email".to_owned(),
                        "\"71076.964-compuserve\" is not a \"email\"".to_owned()
                    ),
                    (
                        "/agency".to_owned(),
                        "\"ABCD\" is not a valid dynamicEnum value".to_owned()
                    )
                ])
            ),
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_load_json_via_url() {
    #[cfg(not(feature = "lite"))]
    let qsv_cache_dir = lookup::set_qsv_cache_dir("~/.qsv-cache").unwrap();
    #[cfg(not(feature = "lite"))]
    QSV_CACHE_DIR.get_or_init(|| qsv_cache_dir);

    let json_string_result = load_json("https://geojson.org/schema/FeatureCollection.json");
    assert!(&json_string_result.is_ok());

    let json_result: Result<Value, serde_json::Error> =
        serde_json::from_str(&json_string_result.unwrap());
    assert!(&json_result.is_ok());
}

#[test]
fn test_dyn_enum_validator() {
    #[cfg(not(feature = "lite"))]
    let qsv_cache_dir = lookup::set_qsv_cache_dir("~/.qsv-cache").unwrap();
    #[cfg(not(feature = "lite"))]
    QSV_CACHE_DIR.get_or_init(|| qsv_cache_dir);

    let schema = json!({"dynamicEnum": "https://raw.githubusercontent.com/dathere/qsv/refs/heads/master/resources/test/fruits.csv", "type": "string"});
    let validator = jsonschema::options()
        .with_keyword("dynamicEnum", dyn_enum_validator_factory)
        .build(&schema)
        .unwrap();

    assert!(validator.is_valid(&json!("banana")));
    assert!(validator.is_valid(&json!("strawberry")));
    assert!(validator.is_valid(&json!("apple")));
    assert!(!validator.is_valid(&json!("Apple")));
    assert!(!validator.is_valid(&json!("starapple")));
    assert!(!validator.is_valid(&json!("bananana")));
    assert!(!validator.is_valid(&json!("")));
    assert!(!validator.is_valid(&json!(5)));
    match validator.validate(&json!("lanzones")) {
        Err(e) => {
            assert_eq!(
                format!("{e:?}"),
                r#"ValidationError { instance: String("lanzones"), kind: Custom { message: "\"lanzones\" is not a valid dynamicEnum value" }, instance_path: Location(""), schema_path: Location("") }"#
            );
        },
        _ => {
            unreachable!("Expected an error, but validation succeeded.");
        },
    }
}
