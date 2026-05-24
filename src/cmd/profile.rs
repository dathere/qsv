static USAGE: &str = r#"
Extract and infer DCAT-3 / Croissant metadata from a CSV, optionally driven by a
CKAN scheming YAML spec.

This is the non-interactive, qsv-native counterpart to what datapusher-plus (DP+)
does in CKAN: run statistical + frequency analysis on the input, build a Jinja2
context (`package`, `resource`, `dpps`, `dppf`, `dpp`), then evaluate every
`formula` / `suggestion_formula` field declared in the scheming YAML. The
resulting `.metadata.json` carries both a CKAN-shaped block and a best-effort
DCAT-US v3 projection, ready for qsv pro and DP+ to prepopulate CKAN packages.

Helpers and filters are reused from DP+'s `jinja2_helpers.py` via an embedded
Python interpreter (qsv's `python` feature). A working `python3` with the
`jinja2` package installed is required at runtime.

For an example spec file, see:
  https://github.com/dathere/datapusher-plus/blob/main/ckanext/datapusher_plus/dataset-druf.yaml

For more extensive examples, see https://github.com/dathere/qsv/blob/master/tests/test_profile.rs.

Usage:
    qsv profile [options] [<input>]
    qsv profile --help

profile options:
    --spec <yaml>             CKAN scheming YAML spec file. If omitted, only the
                              inferred `dpp` block (lat/lon/date columns, dataset
                              stats) is emitted; no formulas are evaluated.
    --package-meta <json>     Optional JSON file with seed package fields (title,
                              owner_org, etc.) merged into the formula context
                              before evaluation.
    --resource-meta <json>    Same, for the resource dict.
    --no-dcat                 Skip the DCAT-US v3 projection block.
    --no-ckan                 Skip the CKAN-shape block.
    --force                   Force recomputing cardinality and unique values
                              even if a stats cache file exists.
    -j, --jobs <arg>          The number of jobs to run in parallel for the
                              underlying stats/frequency passes. When not set,
                              the number of jobs is set to the number of CPUs
                              detected.
    -o, --output <file>       Output JSON path. Default: <input>.metadata.json.

Common options:
    -h, --help                Display this message
    -n, --no-headers          When set, the first row will not be interpreted
                              as headers. Namely, it will be processed with the
                              rest of the rows. Otherwise, the first row will
                              always appear as the header row in the output.
    -d, --delimiter <arg>     The field delimiter for reading CSV data.
                              Must be a single character.
    --memcheck                Check if there is enough memory to load the entire
                              CSV into memory using CONSERVATIVE heuristics.
"#;

use serde::Deserialize;

use crate::{CliResult, util};

#[derive(Debug, Deserialize)]
struct Args {
    arg_input:          Option<String>,
    flag_spec:          Option<String>,
    flag_package_meta:  Option<String>,
    flag_resource_meta: Option<String>,
    flag_no_dcat:       bool,
    flag_no_ckan:       bool,
    #[allow(dead_code)]
    flag_force:         bool,
    #[allow(dead_code)]
    flag_jobs:          Option<usize>,
    flag_output:        Option<String>,
    #[allow(dead_code)]
    flag_no_headers:    bool,
    #[allow(dead_code)]
    flag_delimiter:     Option<crate::config::Delimiter>,
    #[allow(dead_code)]
    flag_memcheck:      bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Scaffolding only — the full implementation is gated behind tasks #5–#9
    // (spec parsing, context builder, PyO3 engine, CKAN+DCAT projection).
    // This stub validates that the command is wired into the CLI dispatch and
    // its flags parse, without yet doing any analysis or formula evaluation.
    let _ = (
        &args.arg_input,
        &args.flag_spec,
        &args.flag_package_meta,
        &args.flag_resource_meta,
        args.flag_no_dcat,
        args.flag_no_ckan,
        &args.flag_output,
    );

    Err(crate::CliError::Other(
        "`qsv profile` is under active development (see issue #1705) and is not yet functional. \
         The command surface is in place; the analysis + formula-evaluation pipeline lands next."
            .into(),
    ))
}
