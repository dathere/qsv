static USAGE: &str = r#"
Randomly samples CSV data.

It supports seven sampling methods:
- RESERVOIR: the default sampling method when NO INDEX is present and no sampling method
  is specified. Visits every CSV record exactly once, using MEMORY PROPORTIONAL to the
  sample size (k) - O(k).
  https://en.wikipedia.org/wiki/Reservoir_sampling

- INDEXED: the default sampling method when an INDEX is present and no sampling method
  is specified. Uses random I/O to sample efficiently, as it only visits records selected
  by random indexing, using MEMORY PROPORTIONAL to the sample size (k) - O(k).
  https://en.wikipedia.org/wiki/Random_access

- BERNOULLI: the sampling method when the --bernoulli option is specified.
  Each record has an independent probability p of being selected, where p is
  specified by the <sample-size> argument. For example, if p=0.1, then each record
  has a 10% chance of being selected, regardless of the other records. The final
  sample size is random and follows a binomial distribution. Uses CONSTANT MEMORY - O(1).
  When sampling from a remote URL, processes the file in chunks without downloading it
  entirely, making it especially efficient for sampling large remote files.
  https://en.wikipedia.org/wiki/Bernoulli_sampling

- SYSTEMATIC: the sampling method when the --systematic option is specified.
  Selects every nth record from the input, where n is the integer part of <sample-size>
  and the fraction part is the percentage of the population to sample.
  For example, if <sample-size> is 10.5, it will select every 10th record and 50% of the
  population. If <sample-size> is a whole number (no fractional part), it will select
  every nth record for the whole population. Uses CONSTANT memory - O(1). The starting
  point can be specified as "random" or "first". Useful for time series data or when you
  want evenly spaced samples.
  https://en.wikipedia.org/wiki/Systematic_sampling

- STRATIFIED: the sampling method when the --stratified option is specified.
  Stratifies the population by the specified column and then samples from each stratum.
  Particularly useful when a population has distinct subgroups (strata) that are
  heterogeneous within but homogeneous between in terms of the variable of interest. 
  For example, if you want to sample 1,000 records from a population of 100,000 across the US,
  you can stratify the population by US state and then sample 20 records from each stratum.
  This will ensure that you have a representative sample from each of the 50 states.
  The sample size must be a whole number. Uses MEMORY PROPORTIONAL to the
  number of strata (s) and samples per stratum (k) as specified by <sample-size> - O(s*k).
  https://en.wikipedia.org/wiki/Stratified_sampling

- WEIGHTED: the sampling method when the --weighted option is specified.
  Samples records with probabilities proportional to values in a specified weight column.
  Records with higher weights are more likely to be selected. For example, if you have
  sales data and want to sample transactions weighted by revenue, high-value transactions
  will have a higher chance of being included. Non-numeric weights are treated as zero.
  The weights are automatically normalized using the maximum weight in the dataset.
  Specify the desired sample size with <sample-size>. Uses MEMORY PROPORTIONAL to the
  sample size (k) - O(k).
  "Weighted random sampling with a reservoir" https://doi.org/10.1016/j.ipl.2005.11.003

- CLUSTER: the sampling method when the --cluster option is specified.
  Samples entire groups of records together based on a cluster identifier column.
  The number of clusters is specified by the <sample-size> argument.
  Useful when records are naturally grouped (e.g., by household, neighborhood, etc.).
  For example, if you have records grouped by neighborhood and specify a sample size of 10,
  it will randomly select 10 neighborhoods and include ALL records from those neighborhoods
  in the output. This ensures that natural groupings in the data are preserved.
  Uses MEMORY PROPORTIONAL to the number of clusters (c) - O(c).
  https://en.wikipedia.org/wiki/Cluster_sampling

Supports sampling from CSVs on remote URLs. Note that the entire file is downloaded first
to a temporary file before sampling begins for all sampling methods except Bernoulli, which
streams the file as it samples it, stopping when the desired sample size is reached or the
end of the file is reached.

Sampling from stdin is also supported for all sampling methods, copying stdin to a in-memory
buffer first before sampling begins.

If a stats cache is available, it will be used to do extra checks on systematic,
weighted and cluster sampling, and to speed up sampling in general.

This command is intended to provide a means to sample from a CSV data set that
is too big to fit into memory (for example, for use with commands like
'qsv stats' with the '--everything' option). 

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_sample.rs.

Usage:
    qsv sample [options] <sample-size> [<input>]
    qsv sample --help

sample arguments:
    <input>                The CSV file to sample. This can be a local file,
                           stdin, or a URL (http and https schemes supported).

    <sample-size>          When using INDEXED, RESERVOIR or WEIGHTED sampling, the sample size.
                             Can either be a whole number or a value between value between 0 and 1.
                             If a fraction, specifies the sample size as a percentage of the population. 
                             (e.g. 0.15 - 15 percent of the CSV)
                           When using BERNOULLI sampling, the probability of selecting each record
                             (between 0 and 1).
                           When using SYSTEMATIC sampling, the integer part is the interval between
                             records to sample & the fractional part is the percentage of the
                             population to sample. When there is no fractional part, it will
                             select every nth record for the entire population.
                           When using STRATIFIED sampling, the stratum sample size.
                           When using CLUSTER sampling, the number of clusters.                       

sample options:
    --seed <number>        Random Number Generator (RNG) seed.
    --rng <kind>           The Random Number Generator (RNG) algorithm to use.
                           Three RNGs are supported:
                            - standard: Use the standard RNG.
                              1.5 GB/s throughput.
                            - faster: Use faster RNG using the Xoshiro256Plus algorithm.
                              8 GB/s throughput.
                            - cryptosecure: Use cryptographically secure HC128 algorithm.
                              Recommended by eSTREAM (https://www.ecrypt.eu.org/stream/).
                              2.1 GB/s throughput though slow initialization.
                           [default: standard]

                           SAMPLING METHODS:
    --bernoulli            Use Bernoulli sampling instead of indexed or reservoir sampling.
                           When this flag is set, <sample-size> must be between
                           0 and 1 and represents the probability of selecting each record.
    --systematic <arg>     Use systematic sampling (every nth record as specified by <sample-size>).
                           If <arg> is "random", the starting point is randomly chosen between 0 & n.
                           If <arg> is "first", the starting point is the first record.
                           The sample size must be a whole number. Uses CONSTANT memory - O(1).
    --stratified <col>     Use stratified sampling. The strata column is specified by <col>.
                           Can be either a column name or 0-based column index.
                           The sample size must be a whole number. Uses MEMORY PROPORTIONAL to the
                           number of strata (s) and samples per stratum (k) - O(s*k).
    --weighted <col>       Use weighted sampling. The weight column is specified by <col>.
                           Can be either a column name or 0-based column index.
                           The column will be parsed as a number. Records with non-number weights
                           will be skipped.
                           Uses MEMORY PROPORTIONAL to the sample size (k) - O(k).
    --cluster <col>        Use cluster sampling. The cluster column is specified by <col>.
                           Can be either a column name or 0-based column index.
                           Uses MEMORY PROPORTIONAL to the number of clusters (c) - O(c).

                           REMOTE FILE OPTIONS:
    --user-agent <agent>   Specify custom user agent to use when the input is a URL.
                           It supports the following variables -
                           $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME, $QSV_KIND and $QSV_COMMAND.
                           Try to follow the syntax here -
                           https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent
    --timeout <secs>       Timeout for downloading URLs in seconds.
                           [default: 30]
    --max-size <mb>        Maximum size of the file to download in MB before sampling.
                           Will download the entire file if not specified.
                           If the CSV is partially downloaded, the sample will be taken
                           only from the downloaded portion.
    --force                Do not use stats cache, even if its available.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will be considered as part of
                           the population to sample from. (When not set, the
                           first row is the header row and will always appear
                           in the output.)
    -d, --delimiter <arg>  The field delimiter for reading/writing CSV data.
                           Must be a single character. (default: ,)
"#;

use std::{
    collections::{HashMap, HashSet},
    io,
    str::FromStr,
};

use futures_util::StreamExt;
use rand::{
    Rng, SeedableRng,
    distr::{Bernoulli, Distribution},
    prelude::IndexedRandom,
    rngs::StdRng,
};
use rand_hc::Hc128Rng;
use rand_xoshiro::Xoshiro256Plus;
use rayon::prelude::ParallelSliceMut;
use reqwest;
use serde::Deserialize;
use strum_macros::EnumString;
use tempfile::NamedTempFile;
use url::Url;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    select::SelectColumns,
    util,
    util::{SchemaArgs, StatsMode, get_stats_records},
};
#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    arg_sample_size: f64,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
    flag_seed:       Option<u64>,
    flag_rng:        String,
    flag_user_agent: Option<String>,
    flag_timeout:    Option<u16>,
    flag_max_size:   Option<u64>,
    flag_bernoulli:  bool,
    flag_systematic: Option<String>,
    flag_stratified: Option<String>,
    flag_weighted:   Option<String>,
    flag_cluster:    Option<String>,
    flag_force:      bool,
}

impl Args {
    fn get_column_index(
        header: &csv::ByteRecord,
        column_spec: &str,
        purpose: &str,
    ) -> CliResult<usize> {
        // Try parsing as number first
        if let Ok(idx) = column_spec.parse::<usize>() {
            if idx < header.len() {
                return Ok(idx);
            }
            return fail_incorrectusage_clierror!(
                "{} column index {} is out of bounds (max: {})",
                purpose,
                idx,
                header.len() - 1
            );
        }

        // If not a number, try to find column by name
        for (i, field) in header.iter().enumerate() {
            if column_spec == String::from_utf8_lossy(field) {
                return Ok(i);
            }
        }

        fail_incorrectusage_clierror!("Could not find {} column named '{}'", purpose, column_spec)
    }

    fn get_strata_column(&self, header: &csv::ByteRecord) -> CliResult<usize> {
        match &self.flag_stratified {
            Some(col) => Self::get_column_index(header, col, "strata"),
            None => {
                fail_incorrectusage_clierror!(
                    "--stratified <col> is required for stratified sampling"
                )
            },
        }
    }

    fn get_weight_column(&self, header: &csv::ByteRecord) -> CliResult<usize> {
        match &self.flag_weighted {
            Some(col) => Self::get_column_index(header, col, "weight"),
            None => {
                fail_incorrectusage_clierror!("--weighted <col> is required for weighted sampling")
            },
        }
    }

    fn get_cluster_column(&self, header: &csv::ByteRecord) -> CliResult<usize> {
        match &self.flag_cluster {
            Some(col) => Self::get_column_index(header, col, "cluster"),
            None => {
                fail_incorrectusage_clierror!("--cluster <col> is required for cluster sampling")
            },
        }
    }
}

#[derive(Debug, EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
enum RngKind {
    Standard,
    Faster,
    Cryptosecure,
}

#[derive(PartialEq)]
enum SamplingMethod {
    Bernoulli,
    Systematic,
    Stratified,
    Weighted,
    Cluster,
    Default,
}

// trait to handle different RNG types
trait RngProvider: Sized {
    type RngType: Rng + SeedableRng;

    fn get_name() -> &'static str;

    fn create(seed: Option<u64>) -> Self::RngType {
        if let Some(seed) = seed {
            Self::RngType::seed_from_u64(seed) // DevSkim: ignore DS148264
        } else {
            Self::RngType::from_os_rng()
        }
    }
}

// Implement for each RNG type
struct StandardRng;
impl RngProvider for StandardRng {
    type RngType = StdRng;

    fn get_name() -> &'static str {
        "standard"
    }
}

struct FasterRng;
impl RngProvider for FasterRng {
    type RngType = Xoshiro256Plus;

    fn get_name() -> &'static str {
        "faster"
    }
}

struct CryptoRng;
impl RngProvider for CryptoRng {
    type RngType = Hc128Rng;

    fn get_name() -> &'static str {
        "cryptosecure"
    }
}

fn check_stats_cache(
    args: &Args,
    method: &SamplingMethod,
) -> CliResult<(Option<u64>, Option<f64>, Option<u64>)> {
    if args.flag_force {
        return Ok((None, None, None));
    }

    // Set stats config
    let schema_args = SchemaArgs {
        arg_input:            args.arg_input.clone(),
        flag_no_headers:      args.flag_no_headers,
        flag_delimiter:       args.flag_delimiter,
        flag_jobs:            None,
        flag_polars:          false,
        flag_memcheck:        false,
        flag_force:           args.flag_force,
        flag_prefer_dmy:      false,
        flag_dates_whitelist: String::new(),
        flag_enum_threshold:  0,
        flag_ignore_case:     false,
        flag_strict_dates:    false,
        flag_pattern_columns: SelectColumns::parse("")?,
        flag_stdout:          false,
    };

    // Get stats records
    match get_stats_records(&schema_args, StatsMode::Frequency) {
        Ok((csv_fields, stats, dataset_stats)) => {
            // Get row count from stats cache
            let rowcount = dataset_stats
                .get("qsv__rowcount")
                .and_then(|rc| rc.parse::<f64>().ok())
                .map(|rc| rc as u64);

            let mut max_weight = None;
            let mut cardinality = None;
            match method {
                SamplingMethod::Weighted => {
                    // For weighted sampling, get max weight
                    if let Some(weight_col) = &args.flag_weighted {
                        let idx = if weight_col.chars().all(char::is_numeric) {
                            weight_col.parse::<usize>().ok()
                        } else {
                            csv_fields
                                .iter()
                                .position(|field| field == weight_col.as_bytes())
                        };

                        if let Some(idx) = idx
                            && let Some(col_stats) = stats.get(idx)
                        {
                            let min_weight = col_stats
                                .min
                                .clone()
                                .unwrap()
                                .parse::<f64>()
                                .unwrap_or_default();
                            if min_weight < 0.0 {
                                return fail_incorrectusage_clierror!(
                                    "Weights must be non-negative. Lowest weight: {min_weight}"
                                );
                            }

                            max_weight = col_stats.max.clone().unwrap().parse::<f64>().ok();
                        }
                    }
                },
                SamplingMethod::Cluster => {
                    // For cluster sampling, get cardinality
                    if let Some(cluster_col) = &args.flag_cluster {
                        let idx = if cluster_col.chars().all(char::is_numeric) {
                            cluster_col.parse::<usize>().ok()
                        } else {
                            csv_fields
                                .iter()
                                .position(|field| field == cluster_col.as_bytes())
                        };

                        if let Some(idx) = idx {
                            cardinality = stats.get(idx).map(|col_stats| col_stats.cardinality);
                        }
                    }
                },
                _ => {},
            }

            Ok((rowcount, max_weight, cardinality))
        },
        _ => Ok((None, None, None)),
    }
}

// "streaming" bernoulli sampling
#[allow(clippy::future_not_send)]
async fn stream_bernoulli_sampling(uri: &str, args: &Args, rng_kind: &RngKind) -> CliResult<()> {
    let default_delim = match std::env::var("QSV_DEFAULT_DELIMITER") {
        Ok(delim) => Delimiter::decode_delimiter(&delim).unwrap().as_byte(),
        _ => b',',
    };

    // Create output writer
    let mut wtr = Config::new(args.flag_output.as_ref())
        .delimiter(args.flag_delimiter)
        .writer()?;

    let client = reqwest::Client::builder()
        .user_agent(util::set_user_agent(args.flag_user_agent.clone())?)
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .zstd(true)
        .use_rustls_tls()
        .http2_adaptive_window(true)
        .connection_verbose(
            log::log_enabled!(log::Level::Debug) || log::log_enabled!(log::Level::Trace),
        )
        .read_timeout(std::time::Duration::from_secs(
            util::timeout_secs(args.flag_timeout.unwrap_or(30)).unwrap_or(30),
        ))
        .build()?;

    // Get the response
    let response = client.get(uri).send().await?;
    let mut stream = response.bytes_stream();

    // Write headers if present
    if !args.flag_no_headers {
        let mut header_bytes = Vec::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            header_bytes.extend_from_slice(&chunk);

            // Try to read headers from accumulated bytes
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(true)
                .delimiter(default_delim)
                .from_reader(&header_bytes[..]);

            if let Ok(headers) = rdr.headers() {
                wtr.write_record(headers)?;
                break;
            }
        }
    }

    let mut std_rng = StandardRng::create(args.flag_seed);
    let mut faster_rng = FasterRng::create(args.flag_seed);
    let mut crypto_rng = CryptoRng::create(args.flag_seed);

    // Process records using streaming
    let mut record = csv::ByteRecord::new();
    let mut buffer = Vec::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        buffer.extend_from_slice(&chunk);

        // Find the last complete record by looking for the last newline
        if let Some(pos) = buffer.iter().rposition(|&b| b == b'\n') {
            // Process only up to the last complete record
            let mut csv_reader = csv::ReaderBuilder::new()
                .has_headers(args.flag_no_headers)
                .delimiter(default_delim)
                .from_reader(&buffer[..=pos]);

            while matches!(csv_reader.read_byte_record(&mut record), Ok(true)) {
                match rng_kind {
                    RngKind::Standard => {
                        if std_rng.random_bool(args.arg_sample_size) {
                            wtr.write_byte_record(&record)?;
                        }
                    },
                    RngKind::Faster => {
                        if faster_rng.random_bool(args.arg_sample_size) {
                            wtr.write_byte_record(&record)?;
                        }
                    },
                    RngKind::Cryptosecure => {
                        if crypto_rng.random_bool(args.arg_sample_size) {
                            wtr.write_byte_record(&record)?;
                        }
                    },
                }
            }

            // Keep the remaining bytes (after the last newline) in the buffer
            buffer.drain(..=pos);
        }
    }

    // Process any remaining records in the buffer
    if !buffer.is_empty() {
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(args.flag_no_headers)
            .delimiter(default_delim)
            .from_reader(&buffer[..]);

        while matches!(csv_reader.read_byte_record(&mut record), Ok(true)) {
            match rng_kind {
                RngKind::Standard => {
                    if std_rng.random_bool(args.arg_sample_size) {
                        wtr.write_byte_record(&record)?;
                    }
                },
                RngKind::Faster => {
                    if faster_rng.random_bool(args.arg_sample_size) {
                        wtr.write_byte_record(&record)?;
                    }
                },
                RngKind::Cryptosecure => {
                    if crypto_rng.random_bool(args.arg_sample_size) {
                        wtr.write_byte_record(&record)?;
                    }
                },
            }
        }
    }

    Ok(wtr.flush()?)
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    if args.arg_sample_size.is_sign_negative() {
        return fail_incorrectusage_clierror!("Sample size cannot be negative.");
    }

    // Validate that only one sampling method is selected
    let methods = [
        args.flag_bernoulli,
        args.flag_systematic.is_some(),
        args.flag_stratified.is_some(),
        args.flag_weighted.is_some(),
        args.flag_cluster.is_some(),
    ];
    if methods.iter().filter(|&&x| x).count() > 1 {
        return fail_incorrectusage_clierror!("Only one sampling method can be specified");
    }

    let Ok(rng_kind) = RngKind::from_str(&args.flag_rng) else {
        return fail_incorrectusage_clierror!(
            "Invalid RNG algorithm `{}`. Supported RNGs are: standard, faster, cryptosecure.",
            args.flag_rng
        );
    };

    let sampling_method = match (
        args.flag_bernoulli,
        args.flag_systematic.is_some(),
        args.flag_stratified.is_some(),
        args.flag_weighted.is_some(),
        args.flag_cluster.is_some(),
    ) {
        (true, _, _, _, _) => SamplingMethod::Bernoulli,
        (_, true, _, _, _) => SamplingMethod::Systematic,
        (_, _, true, _, _) => SamplingMethod::Stratified,
        (_, _, _, true, _) => SamplingMethod::Weighted,
        (_, _, _, _, true) => SamplingMethod::Cluster,
        (false, false, false, false, false) => SamplingMethod::Default,
    };

    let temp_download = NamedTempFile::new()?;

    args.arg_input = match args.arg_input {
        Some(ref uri) if Url::parse(uri).is_ok() && uri.starts_with("http") => {
            // For bernoulli sampling with remote file, handle specially
            if sampling_method == SamplingMethod::Bernoulli {
                log::info!("Streaming Bernoulli sampling remote file");

                let rt = tokio::runtime::Runtime::new()?;
                rt.block_on(stream_bernoulli_sampling(uri, &args, &rng_kind))?;
                return Ok(());
            }

            // For other cases, download entire file
            let max_size_bytes = args.flag_max_size.map(|mb| mb * 1024 * 1024);
            let future = util::download_file(
                uri,
                temp_download.path().to_path_buf(),
                false,
                Some(util::set_user_agent(args.flag_user_agent.clone())?),
                args.flag_timeout,
                max_size_bytes,
            );
            tokio::runtime::Runtime::new()?.block_on(future)?;
            // safety: temp_download is a NamedTempFile, so we know can unwrap.to_string
            Some(temp_download.path().to_str().unwrap().to_string())
        },
        Some(uri) => Some(uri), // local file
        None => None,
    };

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .flexible(true)
        .skip_format_check(true);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(args.flag_output.as_ref())
        .delimiter(args.flag_delimiter)
        .writer()?;

    // Write headers unless --no-headers is specified
    rconfig.write_headers(&mut rdr, &mut wtr)?;

    let mut sample_size = args.arg_sample_size;

    match sampling_method {
        SamplingMethod::Bernoulli => {
            if args.arg_sample_size >= 1.0 || args.arg_sample_size <= 0.0 {
                return fail_incorrectusage_clierror!(
                    "Bernoulli sampling requires a probability between 0 and 1"
                );
            }

            sample_bernoulli(
                &mut rdr,
                &mut wtr,
                args.arg_sample_size,
                args.flag_seed,
                &rng_kind,
            )?;
        },
        SamplingMethod::Systematic => {
            let starting_point = match args.flag_systematic.as_deref().map(str::to_lowercase) {
                Some(arg) if arg == "random" || arg == "first" => arg,
                Some(_) => {
                    return fail_incorrectusage_clierror!(
                        "Systematic sampling starting point must be either 'random' or 'first'"
                    );
                },
                None => String::from("random"),
            };

            let (rowcount_stats, _, _) = check_stats_cache(&args, &SamplingMethod::Systematic)?;
            let row_count = if let Some(rc) = rowcount_stats {
                rc
            } else {
                match util::count_rows(&rconfig) {
                    Ok(rc) => rc,
                    _ => {
                        return fail!(
                            "Cannot get rowcount. Systematic sampling requires a rowcount."
                        );
                    },
                }
            };

            sample_systematic(
                &mut rdr,
                &mut wtr,
                args.arg_sample_size,
                row_count,
                &starting_point,
                args.flag_seed,
                &rng_kind,
            )?;
        },
        SamplingMethod::Stratified => {
            let strata_column = args.get_strata_column(&rdr.byte_headers()?.clone())?;
            sample_stratified(
                &mut rdr,
                &mut wtr,
                strata_column,
                args.arg_sample_size as usize,
                args.flag_seed,
                &rng_kind,
            )?;
        },
        SamplingMethod::Weighted => {
            let weight_column = args.get_weight_column(&rdr.byte_headers()?.clone())?;

            // Get max_weight from cache if available
            let (rowcount, max_weight, _) = check_stats_cache(&args, &SamplingMethod::Weighted)?;

            // determine sample size
            #[allow(clippy::cast_precision_loss)]
            let sample_size = if let Some(rc) = rowcount {
                if args.arg_sample_size < 1.0 {
                    (rc as f64 * args.arg_sample_size).round() as usize
                } else {
                    args.arg_sample_size as usize
                }
            } else if args.arg_sample_size < 1.0 {
                let rowcount = util::count_rows(&rconfig)?;
                (rowcount as f64 * args.arg_sample_size).round() as usize
            } else {
                args.arg_sample_size as usize
            };

            sample_weighted(
                &rconfig,
                &mut rdr,
                &mut wtr,
                weight_column,
                max_weight,
                sample_size,
                args.flag_seed,
                &rng_kind,
            )?;
        },
        SamplingMethod::Cluster => {
            let cluster_column = args.get_cluster_column(&rdr.byte_headers()?.clone())?;

            // Get cardinality from cache if available
            let (_, _, cardinality) = check_stats_cache(&args, &SamplingMethod::Cluster)?;

            sample_cluster(
                &rconfig,
                &mut rdr,
                &mut wtr,
                cluster_column,
                cardinality,
                args.arg_sample_size as usize,
                args.flag_seed,
                &rng_kind,
            )?;
        },
        SamplingMethod::Default => {
            // no sampling method is specified, so we do indexed sampling
            // if an index is present
            if let Some(mut idx) = rconfig.indexed()? {
                #[allow(clippy::cast_precision_loss)]
                if sample_size < 1.0 {
                    sample_size *= idx.count() as f64;
                }

                let sample_count = sample_size as usize;
                let total_count = idx.count().try_into().unwrap();

                match rng_kind {
                    RngKind::Standard => {
                        log::info!("doing standard INDEXED sampling...");
                        let mut rng = StandardRng::create(args.flag_seed);
                        sample_indices(&mut rng, total_count, sample_count, |i| {
                            idx.seek(i as u64)?;
                            Ok(wtr.write_byte_record(&idx.byte_records().next().unwrap()?)?)
                        })?;
                    },
                    RngKind::Faster => {
                        log::info!("doing --faster INDEXED sampling...");
                        let mut rng = FasterRng::create(args.flag_seed);
                        sample_indices(&mut rng, total_count, sample_count, |i| {
                            idx.seek(i as u64)?;
                            Ok(wtr.write_byte_record(&idx.byte_records().next().unwrap()?)?)
                        })?;
                    },
                    RngKind::Cryptosecure => {
                        log::info!("doing --cryptosecure INDEXED sampling...");
                        let mut rng = CryptoRng::create(args.flag_seed);
                        sample_indices(&mut rng, total_count, sample_count, |i| {
                            idx.seek(i as u64)?;
                            Ok(wtr.write_byte_record(&idx.byte_records().next().unwrap()?)?)
                        })?;
                    },
                }
            } else {
                // No sampling method is specified and no index is present
                // do reservoir sampling

                #[allow(clippy::cast_precision_loss)]
                let sample_size = if args.arg_sample_size < 1.0 {
                    // Get rowcount from stats cache if available
                    let (rowcount_stats, _, _) =
                        check_stats_cache(&args, &SamplingMethod::Default)?;

                    if let Some(rc) = rowcount_stats {
                        (rc as f64 * args.arg_sample_size).round() as u64
                    } else {
                        match util::count_rows(&rconfig) {
                            Ok(rc) => {
                                // we don't have a stats cache, get the rowcount the "regular"
                                // way
                                (rc as f64 * args.arg_sample_size).round() as u64
                            },
                            _ => {
                                return fail!(
                                    "Cannot get rowcount. Percentage sampling requires a rowcount."
                                );
                            },
                        }
                    }
                } else {
                    args.arg_sample_size as u64
                };

                sample_reservoir(&mut rdr, &mut wtr, sample_size, args.flag_seed, &rng_kind)?;
            }
        },
    }

    Ok(wtr.flush()?)
}

fn sample_reservoir<R: io::Read, W: io::Write>(
    rdr: &mut csv::Reader<R>,
    wtr: &mut csv::Writer<W>,
    sample_size: u64,
    seed: Option<u64>,
    rng_kind: &RngKind,
) -> CliResult<()> {
    let mut reservoir = Vec::with_capacity(sample_size as usize);
    let mut records = rdr.byte_records().enumerate();

    // Pre-fill reservoir
    // Note that we use by_ref() to avoid consuming the iterator
    // and we only take the first sample_size records
    for (_, row) in records.by_ref().take(sample_size as usize) {
        reservoir.push(row?);
    }

    match rng_kind {
        RngKind::Standard => {
            do_reservoir_sampling::<StandardRng>(&mut records, &mut reservoir, sample_size, seed)
        },
        RngKind::Faster => {
            do_reservoir_sampling::<FasterRng>(&mut records, &mut reservoir, sample_size, seed)
        },
        RngKind::Cryptosecure => {
            do_reservoir_sampling::<CryptoRng>(&mut records, &mut reservoir, sample_size, seed)
        },
    }?;

    // Write the reservoir to output
    for record in reservoir {
        wtr.write_byte_record(&record)?;
    }

    Ok(())
}

// Generic reservoir sampling implementation using constant memory
fn do_reservoir_sampling<T: RngProvider>(
    records: &mut impl Iterator<Item = (usize, Result<csv::ByteRecord, csv::Error>)>,
    reservoir: &mut [csv::ByteRecord],
    sample_size: u64,
    seed: Option<u64>,
) -> CliResult<()> {
    log::info!("doing {} RESERVOIR sampling...", T::get_name());
    let mut rng = T::create(seed);
    let mut random_idx: usize;

    // Process remaining records using Algorithm R (Robert Floyd)
    for (i, row) in records {
        random_idx = rng.random_range(0..=i);
        if random_idx < sample_size as usize {
            unsafe {
                *reservoir.get_unchecked_mut(random_idx) = row?;
            }
        }
    }
    Ok(())
}

fn sample_bernoulli<R: io::Read, W: io::Write>(
    rdr: &mut csv::Reader<R>,
    wtr: &mut csv::Writer<W>,
    probability: f64,
    seed: Option<u64>,
    rng_kind: &RngKind,
) -> CliResult<()> {
    let mut records = rdr.byte_records();

    match rng_kind {
        RngKind::Standard => {
            do_bernoulli_sampling::<StandardRng>(&mut records, wtr, probability, seed)
        },
        RngKind::Faster => do_bernoulli_sampling::<FasterRng>(&mut records, wtr, probability, seed),
        RngKind::Cryptosecure => {
            do_bernoulli_sampling::<CryptoRng>(&mut records, wtr, probability, seed)
        },
    }
}

// Generic bernoulli sampling implementation using constant memory
fn do_bernoulli_sampling<T: RngProvider>(
    records: &mut impl Iterator<Item = Result<csv::ByteRecord, csv::Error>>,
    wtr: &mut csv::Writer<impl io::Write>,
    probability: f64,
    seed: Option<u64>,
) -> CliResult<()> {
    log::info!("doing {} BERNOULLI sampling...", T::get_name());
    let mut rng = T::create(seed);

    let dist =
        Bernoulli::new(probability).map_err(|_| "probability must be between 0.0 and 1.0")?;

    for row in records {
        if dist.sample(&mut rng) {
            wtr.write_byte_record(&row?)?;
        }
    }
    Ok(())
}

// Helper function to sample indices using constant memory
fn sample_indices<F>(
    rng: &mut impl Rng,
    total_count: usize,
    sample_count: usize,
    mut process_index: F,
) -> CliResult<()>
where
    F: FnMut(usize) -> CliResult<()>,
{
    if sample_count > total_count {
        return fail!("Sample size cannot be larger than population size");
    }

    // Store selected indices in a sorted vec of size k
    let mut selected = Vec::with_capacity(sample_count);

    // Fill first k positions
    for i in 0..sample_count {
        selected.push(i);
    }

    // Process remaining positions using reservoir sampling
    for i in sample_count..total_count {
        let j = rng.random_range(0..=i);
        if j < sample_count {
            unsafe { *selected.get_unchecked_mut(j) = i };
        }
    }

    // Process indices in order to avoid seeking back and forth
    selected.par_sort_unstable();
    for idx in selected {
        process_index(idx)?;
    }

    Ok(())
}

// Systematic sampling implementation
fn sample_systematic<R: io::Read, W: io::Write>(
    rdr: &mut csv::Reader<R>,
    wtr: &mut csv::Writer<W>,
    sample_size: f64,
    row_count: u64,
    starting_point: &str,
    seed: Option<u64>,
    rng_kind: &RngKind,
) -> CliResult<()> {
    if sample_size <= 0.0 {
        return fail_incorrectusage_clierror!("Sample size must be positive");
    }

    // Split sample_size into integer and fractional parts
    let interval = sample_size.trunc() as usize;
    let percentage = sample_size.fract();

    if interval == 0 {
        return fail_incorrectusage_clierror!("Interval must be at least 1");
    }

    // Calculate target sample size based on percentage
    #[allow(clippy::cast_precision_loss)]
    let target_count = if percentage > 0.0 {
        ((row_count as f64) * percentage).round() as u64
    } else {
        row_count
    };

    // Select starting point
    let start = if starting_point == "random" {
        match rng_kind {
            RngKind::Standard => {
                let mut rng = StandardRng::create(seed);
                rng.random_range(0..interval)
            },
            RngKind::Faster => {
                let mut rng = FasterRng::create(seed);
                rng.random_range(0..interval)
            },
            RngKind::Cryptosecure => {
                let mut rng = CryptoRng::create(seed);
                rng.random_range(0..interval)
            },
        }
    } else {
        0 // starting point is the first record
    };

    // Select records at regular intervals
    let mut selected_count = 0;
    for (i, record) in rdr.byte_records().enumerate().skip(start) {
        if i.is_multiple_of(interval) && selected_count < target_count {
            wtr.write_byte_record(&record?)?;
            selected_count += 1;
        }
    }

    Ok(())
}

// Stratified sampling implementation
fn sample_stratified<R: io::Read, W: io::Write>(
    rdr: &mut csv::Reader<R>,
    wtr: &mut csv::Writer<W>,
    strata_column: usize,
    samples_per_stratum: usize,
    seed: Option<u64>,
    rng_kind: &RngKind,
) -> CliResult<()> {
    const ESTIMATED_STRATA_COUNT: usize = 100;

    // Pre-allocate with capacity for better performance
    let mut strata_counts: HashMap<Vec<u8>, usize> = HashMap::with_capacity(ESTIMATED_STRATA_COUNT);
    let mut records = Vec::with_capacity(ESTIMATED_STRATA_COUNT * samples_per_stratum);
    let mut curr_record;

    // First pass: count strata and collect records
    for record in rdr.byte_records() {
        curr_record = record?;
        let stratum = curr_record
            .get(strata_column)
            .ok_or_else(|| format!("Strata column index {strata_column} out of bounds"))?
            .to_vec();
        *strata_counts.entry(stratum.clone()).or_default() += 1;
        records.push(curr_record);
    }

    let strata_count = strata_counts.len();
    if strata_count == 0 {
        return fail_incorrectusage_clierror!("No valid strata found in the data");
    }

    // Initialize reservoirs with capacity
    let mut reservoirs: HashMap<Vec<u8>, Vec<csv::ByteRecord>> =
        HashMap::with_capacity(strata_count);
    for stratum in strata_counts.keys() {
        reservoirs.insert(stratum.clone(), Vec::with_capacity(samples_per_stratum));
    }

    // Create RNG and perform sampling
    match rng_kind {
        RngKind::Standard => {
            let mut rng = StandardRng::create(seed);
            do_stratified_sampling(
                records.into_iter(),
                &mut reservoirs,
                strata_column,
                samples_per_stratum,
                &mut rng,
            )?;
        },
        RngKind::Faster => {
            let mut rng = FasterRng::create(seed);
            do_stratified_sampling(
                records.into_iter(),
                &mut reservoirs,
                strata_column,
                samples_per_stratum,
                &mut rng,
            )?;
        },
        RngKind::Cryptosecure => {
            let mut rng = CryptoRng::create(seed);
            do_stratified_sampling(
                records.into_iter(),
                &mut reservoirs,
                strata_column,
                samples_per_stratum,
                &mut rng,
            )?;
        },
    }

    // Write results in deterministic order
    let mut strata: Vec<_> = reservoirs.keys().collect();
    strata.par_sort_unstable();
    for stratum in strata {
        if let Some(records) = reservoirs.get(stratum) {
            for record in records {
                wtr.write_byte_record(record)?;
            }
        }
    }

    Ok(())
}

fn do_stratified_sampling<T: Rng + ?Sized>(
    records: impl Iterator<Item = csv::ByteRecord>,
    reservoirs: &mut HashMap<Vec<u8>, Vec<csv::ByteRecord>>,
    strata_column: usize,
    samples_per_stratum: usize,
    rng: &mut T,
) -> CliResult<()> {
    let mut records_seen: HashMap<Vec<u8>, usize> = HashMap::with_capacity(reservoirs.len());

    for record in records {
        let stratum = record
            .get(strata_column)
            .ok_or_else(|| format!("Strata column index {strata_column} out of bounds"))?
            .to_vec();

        let seen = records_seen.entry(stratum.clone()).or_default();

        if let Some(reservoir) = reservoirs.get_mut(&stratum) {
            if reservoir.len() < samples_per_stratum {
                reservoir.push(record);
            } else {
                let j = rng.random_range(0..=*seen);
                if j < samples_per_stratum {
                    // safety: we know that j is within the bounds of the reservoir
                    unsafe { *reservoir.get_unchecked_mut(j) = record };
                }
            }
            *seen += 1;
        }
    }
    Ok(())
}

// Weighted sampling implementation
fn sample_weighted<R: io::Read, W: io::Write>(
    rconfig: &Config,
    rdr: &mut csv::Reader<R>,
    wtr: &mut csv::Writer<W>,
    weight_column: usize,
    max_weight_stats: Option<f64>,
    sample_size: usize,
    seed: Option<u64>,
    rng_kind: &RngKind,
) -> CliResult<()> {
    let max_weight = if let Some(wt) = max_weight_stats {
        wt
    } else {
        // We don't have a stats cache, do a first pass to find maximum weight
        let mut max_weight_scan = 0.0f64;
        let mut curr_record;
        for record in rdr.byte_records() {
            curr_record = record?;

            let weight: f64 = fast_float2::parse(
                curr_record
                    .get(weight_column)
                    .ok_or_else(|| format!("Weight column index {weight_column} out of bounds"))?,
            )
            .unwrap_or(0.0);

            if weight < 0.0 {
                return fail_incorrectusage_clierror!("Weights must be non-negative: ({weight})");
            }
            max_weight_scan = max_weight_scan.max(weight);
        }
        max_weight_scan
    };

    if max_weight == 0.0 {
        return fail_incorrectusage_clierror!("All weights are zero");
    }

    // Second pass: acceptance-rejection sampling
    let mut rdr2 = rconfig.reader()?;

    match rng_kind {
        RngKind::Standard => {
            log::info!("doing standard WEIGHTED sampling...");
            let mut rng = StandardRng::create(seed);
            do_weighted_sampling(
                &mut rdr2.byte_records(),
                wtr,
                weight_column,
                sample_size,
                max_weight,
                &mut rng,
            )?;
        },
        RngKind::Faster => {
            log::info!("doing --faster WEIGHTED sampling...");
            let mut rng = FasterRng::create(seed);
            do_weighted_sampling(
                &mut rdr2.byte_records(),
                wtr,
                weight_column,
                sample_size,
                max_weight,
                &mut rng,
            )?;
        },
        RngKind::Cryptosecure => {
            log::info!("doing --cryptosecure WEIGHTED sampling...");
            let mut rng = CryptoRng::create(seed);
            do_weighted_sampling(
                &mut rdr2.byte_records(),
                wtr,
                weight_column,
                sample_size,
                max_weight,
                &mut rng,
            )?;
        },
    }

    Ok(())
}

// Helper function to handle the actual sampling with any RNG type
fn do_weighted_sampling<T: Rng + ?Sized>(
    records: &mut impl Iterator<Item = Result<csv::ByteRecord, csv::Error>>,
    wtr: &mut csv::Writer<impl io::Write>,
    weight_column: usize,
    sample_size: usize,
    max_weight: f64,
    rng: &mut T,
) -> CliResult<()> {
    use std::collections::HashSet;

    let mut selected = HashSet::with_capacity(sample_size);
    let mut attempts = 0;
    let max_attempts = sample_size * 100; // Prevent infinite loops
    let mut curr_record;
    let mut selected_len = 0;
    let mut records_exhausted = false;

    while selected_len < sample_size && attempts < max_attempts && !records_exhausted {
        let mut any_records = false;
        for (i, record) in records.enumerate() {
            any_records = true;
            if selected_len >= sample_size {
                break;
            }

            curr_record = record?;

            let weight: f64 = fast_float2::parse(
                curr_record
                    .get(weight_column)
                    .ok_or_else(|| format!("Weight column index {weight_column} out of bounds"))?,
            )
            .unwrap_or(0.0);

            if weight < 0.0 {
                return fail_incorrectusage_clierror!("Weights must be non-negative: ({weight})");
            }

            // Modified acceptance-rejection method to handle zero weights
            let include_flag = if weight == 0.0 {
                false
            } else {
                rng.random::<f64>() <= (weight / max_weight)
            };

            if include_flag && !selected.contains(&i) {
                selected.insert(i);
                selected_len += 1;
                wtr.write_byte_record(&curr_record)?;
            }

            attempts += 1;
            if attempts >= max_attempts {
                break;
            }
        }
        records_exhausted = !any_records;
    }

    if selected_len < sample_size {
        wwarn!("Could only sample {selected_len} records out of requested {sample_size}");
    }

    Ok(())
}

// Cluster sampling implementation
fn sample_cluster<R: io::Read, W: io::Write>(
    rconfig: &Config,
    rdr: &mut csv::Reader<R>,
    wtr: &mut csv::Writer<W>,
    cluster_column: usize,
    cluster_cardinality: Option<u64>,
    requested_clusters: usize,
    seed: Option<u64>,
    rng_kind: &RngKind,
) -> CliResult<()> {
    const ESTIMATED_CLUSTER_COUNT: usize = 100;

    let cluster_count = if let Some(cardinality) = cluster_cardinality {
        if requested_clusters > cardinality as usize {
            return fail_incorrectusage_clierror!(
                "Requested sample size ({requested_clusters}) exceeds number of clusters \
                 ({cardinality})",
            );
        }
        requested_clusters
    } else {
        ESTIMATED_CLUSTER_COUNT
    };

    // Use HashSet for faster lookups of unique clusters
    let mut unique_clusters: HashSet<Vec<u8>> = HashSet::with_capacity(cluster_count);
    let mut all_clusters: Vec<Vec<u8>> = Vec::with_capacity(cluster_count);
    let mut curr_record;

    // First pass: collect unique clusters
    for record in rdr.byte_records() {
        curr_record = record?;
        let cluster = curr_record
            .get(cluster_column)
            .ok_or_else(|| format!("Cluster column index {cluster_column} out of bounds"))?
            .to_vec();

        if unique_clusters.insert(cluster.clone()) {
            all_clusters.push(cluster);
        }
    }

    if unique_clusters.is_empty() {
        return fail_incorrectusage_clierror!("No valid clusters found in the data");
    }

    // Select clusters
    let selected_clusters: HashSet<Vec<u8>> = match rng_kind {
        RngKind::Standard => {
            let mut rng = StandardRng::create(seed);
            all_clusters
                .choose_multiple(&mut rng, requested_clusters.min(all_clusters.len()))
                .cloned()
                .collect()
        },
        RngKind::Faster => {
            let mut rng = FasterRng::create(seed);
            all_clusters
                .choose_multiple(&mut rng, requested_clusters.min(all_clusters.len()))
                .cloned()
                .collect()
        },
        RngKind::Cryptosecure => {
            let mut rng = CryptoRng::create(seed);
            all_clusters
                .choose_multiple(&mut rng, requested_clusters.min(all_clusters.len()))
                .cloned()
                .collect()
        },
    };

    // Second pass: output records from selected clusters
    let mut rdr2 = rconfig.reader()?;
    let mut curr_record;
    for record in rdr2.byte_records() {
        curr_record = record?;
        let cluster = curr_record
            .get(cluster_column)
            .ok_or_else(|| format!("Cluster column index {cluster_column} out of bounds"))?
            .to_vec();

        if selected_clusters.contains(&cluster) {
            wtr.write_byte_record(&curr_record)?;
        }
    }

    Ok(())
}
