static USAGE: &str = r#"
Create a new computed column or filter rows by evaluating a Python expression on 
every row of a CSV file.

The executed Python has 4 ways to reference cell values (as strings):
  1. Directly by using column name (e.g. amount) as a local variable. If a column
     name has spaces and other special characters, they are replaced with underscores
     (e.g. "unit cost" -> unit_cost, "test-units/sec" -> test_units_sec)
  2. Indexing cell value by column name as an attribute: col.amount
  3. Indexing cell value by column name as a key: col["amount"]
  4. Indexing cell value by column position: col[0]

Of course, if your input has no headers, then 4. will be the only available
option.

Some usage examples:

  Sum numeric columns 'a' and 'b' and call new column 'c'
  $ qsv py map c "int(a) + int(b)"
  $ qsv py map c "int(col.a) + int(col['b'])"
  $ qsv py map c "int(col[0]) + int(col[1])"

  Use Python f-strings to calculate using multiple columns (qty, fruit & "unit cost") 
    and format into a new column 'formatted'
  $ qsv py map formatted 'f"{qty} {fruit} cost ${(float(unit_cost) * float(qty)):.2f}"'

  You can even have conditionals in your f-string:
  $ qsv py map formatted \
   'f"""{qty} {fruit} cost ${(float(unit_cost) * float(qty)):.2f}. Its quite {"cheap" if ((float(unit_cost) * float(qty)) < 20.0) else "expensive"}!"""'

  Note how we needed to use triple double quotes for the f-string, so we can use the literals
  "cheap" and "expensive" in the f-string expression.

  Strip and prefix cell values
  $ qsv py map prefixed "'clean_' + a.strip()"

  Filter some lines based on numerical filtering
  $ qsv py filter "int(a) > 45"

  Load helper file with function to compute Fibonacci sequence of the column "num_col"
  $ qsv py map --helper fibonacci.py fib qsv_uh.fibonacci(num_col) data.csv

  Below is a detailed example of the --helper option:

  Use case:
  Need to calculate checksum/md5sum of some columns. First column (c1) is "id", and do md5sum of
  the rest of the columns (c2, c3 and c4).

  Given test.csv:
    c1,c2,c3,c4
    1,a2,a3,a4
    2,b2,b3,b4
    3,c2,c3,c4

  and hashhelper.py:
    import hashlib
    def md5hash (*args):
        s = ",".join(args)
        return(hashlib.md5(s.encode('utf-8')).hexdigest())

  with the following command:
  $ qsv py map --helper hashhelper.py hashcol 'qsv_uh.md5hash(c2,c3,c4)' test.csv

  we get:
  c1,c2,c3,c4,hashcol
  1,a2,a3,a4,cb675342ed940908eef0844d17c35fab
  2,b2,b3,b4,7d594b33f82bdcbc1cfa6f924a84c4cd
  3,c2,c3,c4,6eabbfdbfd9ab6ae7737fb2b82f6a1af
  
  Note that qsv with the `python` feature enabled will panic on startup even if you're not
  using the `py` command if Python's shared libraries are not found.
  
  Also, the following Python modules are automatically loaded and available to the user -
  builtins, math, random & datetime. The user can import additional modules with the --helper option,
  with the ability to use any Python module that's installed in the current Python virtualenv. 

  The Python expression is evaluated on a per record basis.
  With "py map", if the expression is invalid for a record, "<ERROR>" is returned for that record.
  With "py filter", if the expression is invalid for a record, that record is not filtered.

  If any record has an invalid result, an exitcode of 1 is returned and an error count is logged.

For more extensive examples, see https://github.com/dathere/qsv/blob/master/tests/test_py.rs.

Usage:
    qsv py map [options] -n <expression> [<input>]
    qsv py map [options] <new-column> <expression> [<input>]
    qsv py map --helper <file> [options] <new-column> <expression> [<input>]
    qsv py filter [options] <expression> [<input>]
    qsv py map --help
    qsv py filter --help
    qsv py --help

py argument:
    <expression>           Can either be a Python expression, or if it starts with
                           "file:" or ends with ".py" - the filepath from which to
                           load the Python expression.
                           Note that argument expects a SINGLE expression, and not
                           a full-blown Python script. Use the --helper option
                           to load helper code that you can call from the expression.

py options:
    -f, --helper <file>    File containing Python code that's loaded into the 
                           qsv_uh Python module. Functions with a return statement
                           in the file can be called with the prefix "qsv_uh".
                           The returned value is used in the map or filter operation.

    -b, --batch <size>     The number of rows per batch to process before
                           releasing memory and acquiring a new GILpool.
                           Set to 0 to process the entire file in one batch.
                           [default: 50000]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. Namely, it will be sorted with the rest
                           of the rows. Otherwise, the first row will always
                           appear as the header row in the output.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -p, --progressbar      Show progress bars. Not valid for stdin.
"#;

use std::{ffi::CString, fs};

use indicatif::{ProgressBar, ProgressDrawTarget};
use pyo3::{
    PyErr, PyResult, Python, intern,
    prelude::*,
    types::{PyDict, PyList, PyModule},
};
use serde::Deserialize;

use crate::{
    CliError, CliResult,
    config::{Config, Delimiter},
    util,
};

const HELPERS: &str = r#"
def cast_as_string(value):
    if isinstance(value, str):
        return value
    return str(value)

def cast_as_bool(value):
    return bool(value)

class QSVRow(object):
    def __init__(self, headers):
        self.__data = None
        self.__headers = headers
        self.__mapping = {h: i for i, h in enumerate(headers)}

    def _update_underlying_data(self, row_data):
        self.__data = row_data

    def __getitem__(self, key):
        if isinstance(key, int):
            return self.__data[key]

        return self.__data[self.__mapping[key]]

    def __getattr__(self, key):
        return self.__data[self.__mapping[key]]
"#;

#[derive(Deserialize)]
struct Args {
    cmd_map:          bool,
    cmd_filter:       bool,
    arg_new_column:   Option<String>,
    arg_expression:   String,
    flag_batch:       usize,
    flag_helper:      Option<String>,
    arg_input:        Option<String>,
    flag_output:      Option<String>,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
    flag_progressbar: bool,
}

impl From<PyErr> for CliError {
    fn from(err: PyErr) -> CliError {
        CliError::Other(err.to_string())
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;

    let debug_flag = log::log_enabled!(log::Level::Debug);

    if debug_flag {
        Python::attach(|py| {
            let msg = format!("Detected Python={}", py.version());
            winfo!("{msg}");
        });
    }

    let expression = if let Some(expression_filepath) =
        args.arg_expression.strip_prefix(util::FILE_PATH_PREFIX)
    {
        match fs::read_to_string(expression_filepath) {
            Ok(file_contents) => file_contents,
            Err(e) => {
                return fail_clierror!(
                    "Cannot read Python expression file '{expression_filepath}': {e}"
                );
            },
        }
    } else if std::path::Path::new(&args.arg_expression)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("py"))
    {
        match fs::read_to_string(&args.arg_expression) {
            Ok(file_contents) => file_contents,
            Err(e) => {
                return fail_clierror!(
                    "Cannot read Python expression file '{}': {e}",
                    args.arg_expression
                );
            },
        }
    } else {
        args.arg_expression.clone()
    };

    let mut helper_text = String::new();
    if let Some(helper_file) = &args.flag_helper {
        helper_text = match fs::read_to_string(helper_file) {
            Ok(contents) => contents,
            Err(e) => {
                return fail_clierror!("Cannot read Python helper file '{helper_file}': {e}");
            },
        }
    }

    let mut headers = rdr.headers()?.clone();
    let headers_len = headers.len();

    if rconfig.no_headers {
        headers = csv::StringRecord::new();

        for i in 0..headers_len {
            headers.push_field(itoa::Buffer::new().format(i));
        }
    }

    // Compute Python-safe local-variable names from the input headers BEFORE
    // any new output column is appended below, so header_vec.len() ==
    // headers_len and the per-row loop doesn't need a .take() guard.
    let (header_vec, _) = util::safe_header_names(&headers, true, false, None, "_", false);

    if !rconfig.no_headers {
        if !args.cmd_filter {
            let new_column = args
                .arg_new_column
                .as_ref()
                .ok_or("Specify new column name")?;
            headers.push_field(new_column);
        }

        wtr.write_record(&headers)?;
    }

    // prep progress bar
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    // amortize memory allocation by reusing record
    let mut batch_record = csv::StringRecord::new();
    let mut error_count = 0_usize;

    let batch_size = if args.flag_batch == 0 {
        util::count_rows(&rconfig)? as usize
    } else {
        args.flag_batch
    };

    // reuse batch buffers
    let mut batch = Vec::with_capacity(batch_size);

    // safety: safe to unwrap as these are statically defined
    let helpers_code = CString::new(HELPERS).unwrap();
    let helpers_filename = CString::new("qsv_helpers.py").unwrap();
    let helpers_module_name = CString::new("qsv_helpers").unwrap();
    let user_helpers_code = CString::new(helper_text)
        .map_err(|e| format!("Failed to create CString from helper text: {e}"))?;
    // safety: safe to unwrap as these are statically defined
    let user_helpers_filename = CString::new("qsv_user_helpers.py").unwrap();
    let user_helpers_module_name = CString::new("qsv_uh").unwrap();

    let arg_expression = CString::new(expression)
        .map_err(|e| format!("Failed to create CString from expression: {e}"))?;

    let mut row_number = 0_u64;

    // Build modules and the QSVRow instance ONCE up front. They don't change
    // across batches, so re-creating them each batch was wasted work.
    // We hold them as Py<...> across Python::attach calls and re-bind per batch.
    let (
        helpers_pymod,
        user_helpers_pymod,
        builtins_pymod,
        math_pymod,
        random_pymod,
        datetime_pymod,
        py_row_obj,
    ) = Python::attach(|py| -> PyResult<_> {
        let helpers =
            PyModule::from_code(py, &helpers_code, &helpers_filename, &helpers_module_name)?;
        let user_helpers = PyModule::from_code(
            py,
            &user_helpers_code,
            &user_helpers_filename,
            &user_helpers_module_name,
        )?;
        let builtins = PyModule::import(py, "builtins")?;
        let math_module = PyModule::import(py, "math")?;
        let random_module = PyModule::import(py, "random")?;
        let datetime_module = PyModule::import(py, "datetime")?;

        // Pass only the input headers (not the appended map output column) to
        // QSVRow so its `__mapping` lines up with the row data; otherwise
        // `col["<new-column>"]` would map to an out-of-range index.
        let py_row = helpers
            .getattr("QSVRow")?
            .call1((headers.iter().take(headers_len).collect::<Vec<&str>>(),))?;

        Ok((
            helpers.unbind(),
            user_helpers.unbind(),
            builtins.unbind(),
            math_module.unbind(),
            random_module.unbind(),
            datetime_module.unbind(),
            py_row.unbind(),
        ))
    })?;

    // main loop to read CSV and construct batches.
    // we batch Python operations so that the GILPool does not get very large
    // as we release the pool after each batch
    // loop exits when batch is empty.
    'batch_loop: loop {
        for _ in 0..batch_size {
            match rdr.read_record(&mut batch_record) {
                Ok(has_data) => {
                    if has_data {
                        batch.push(std::mem::take(&mut batch_record));
                    } else {
                        // nothing else to add to batch
                        break;
                    }
                },
                Err(e) => {
                    return fail_clierror!("Error reading file: {e}");
                },
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break 'batch_loop;
        }

        Python::attach(|py| -> PyResult<()> {
            let helpers = helpers_pymod.bind(py);
            let user_helpers = user_helpers_pymod.bind(py);
            let builtins = builtins_pymod.bind(py);
            let math_module = math_pymod.bind(py);
            let random_module = random_pymod.bind(py);
            let datetime_module = datetime_pymod.bind(py);
            let py_row = py_row_obj.bind(py);

            let batch_globals = PyDict::new(py);
            let batch_locals = PyDict::new(py);

            batch_globals.set_item(intern!(py, "qsv_uh"), user_helpers)?;
            batch_globals.set_item(intern!(py, "__builtins__"), builtins)?;
            batch_globals.set_item(intern!(py, "math"), math_module)?;
            batch_globals.set_item(intern!(py, "random"), random_module)?;
            batch_globals.set_item(intern!(py, "datetime"), datetime_module)?;

            batch_locals.set_item(intern!(py, "col"), py_row)?;

            let error_result = intern!(py, "<ERROR>");

            for record in batch.iter_mut() {
                row_number += 1;

                // Tolerate jagged rows: short records yield "" via unwrap_or_default,
                // long records are ignored beyond header_vec.len() (== headers_len).
                // The PyList is built in the same pass that sets the per-column
                // locals, and storing the row data directly in a Python object
                // releases the &str borrows on `record` before push_field below.
                let row_data = PyList::empty(py);
                for (i, key) in header_vec.iter().enumerate() {
                    let cell_value = record.get(i).unwrap_or_default();
                    batch_locals.set_item(key, cell_value)?;
                    row_data.append(cell_value)?;
                }
                py_row.call_method1(intern!(py, "_update_underlying_data"), (row_data,))?;

                let result =
                    match py.eval(&arg_expression, Some(&batch_globals), Some(&batch_locals)) {
                        Ok(result) => result,
                        Err(e) => {
                            error_count += 1;
                            if debug_flag {
                                log::error!("Expression error:{row_number}-{e:?}");
                            }
                            e.print_and_set_sys_last_vars(py);
                            error_result.as_any().clone()
                        },
                    };

                if args.cmd_map {
                    let result = helpers
                        .getattr(intern!(py, "cast_as_string"))?
                        .call1((result,))?;
                    let value: String = result.extract()?;

                    record.push_field(&value);
                    if let Err(e) = wtr.write_record(&*record) {
                        // we do this since we cannot use the ? operator here
                        // since this closure returns a PyResult
                        // this is converted to a CliError::Other anyway
                        return Err(pyo3::PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                            "cannot write record ({row_number}-{e})"
                        )));
                    }
                } else if args.cmd_filter {
                    let result = helpers
                        .getattr(intern!(py, "cast_as_bool"))?
                        .call1((result,))?;
                    let include_record: bool = result.extract()?;

                    if include_record && let Err(e) = wtr.write_record(&*record) {
                        return Err(pyo3::PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                            "cannot write record ({row_number}-{e})"
                        )));
                    }
                }
            }

            Ok(())
        })?;
        if show_progress {
            progress.inc(batch.len() as u64);
        }

        batch.clear();
    } // end batch loop

    if show_progress {
        util::finish_progress(&progress);
    }

    wtr.flush()?;

    if error_count > 0 {
        return fail_clierror!("Python errors encountered: {error_count}");
    }

    Ok(())
}
