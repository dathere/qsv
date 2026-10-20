use std::{env, path::Path, str};

use sysinfo::System;

#[cfg(feature = "polars")]
use super::QSV_POLARS_REV;
use super::{
    BACKGROUND_THREADS_ACTIVE, OTHER_ENV_VARS, QSV_KIND, TARGET, max_jobs, mem_file_check, num_cpus,
};

/// A compiled-in optional feature, as reported by `--version` and `--capabilities`.
pub struct EnabledFeature {
    /// stable, machine-readable feature name (e.g. `polars`)
    pub name:    &'static str,
    /// version detail, where the feature has one (e.g. the Polars version)
    pub version: Option<String>,
    /// the token `--version` has always printed for this feature
    pub token:   String,
}

impl EnabledFeature {
    #[allow(dead_code)] // a featureless qsvlite build enables no plain features
    fn plain(name: &'static str) -> Self {
        Self {
            name,
            version: None,
            token: name.to_string(),
        }
    }
}

/// The optional features compiled into this binary, in `--version` order. Single source for
/// both the `--version` string and `--capabilities`, so the two cannot drift.
#[allow(clippy::vec_init_then_push)] // every push is cfg-gated
pub fn enabled_features() -> Vec<EnabledFeature> {
    #[allow(unused_mut)]
    let mut features: Vec<EnabledFeature> = Vec::new();

    #[cfg(all(feature = "apply", not(feature = "lite")))]
    features.push(EnabledFeature::plain("apply"));
    #[cfg(all(feature = "fetch", not(feature = "lite")))]
    features.push(EnabledFeature::plain("fetch"));
    #[cfg(all(feature = "foreach", not(feature = "lite")))]
    features.push(EnabledFeature::plain("foreach"));
    #[cfg(all(feature = "geocode", not(feature = "lite")))]
    features.push(EnabledFeature::plain("geocode"));

    #[cfg(all(feature = "luau", not(feature = "lite")))]
    {
        let luau = mlua::Lua::new();
        let (version, token) = match luau.load("return _VERSION").eval() {
            Ok(mlua::Value::String(luaustring_val)) => {
                let string_val = luaustring_val.to_string_lossy();
                if string_val == "Luau" {
                    (None, "Luau - version not specified".to_string())
                } else {
                    (
                        string_val.strip_prefix("Luau ").map(str::to_string),
                        string_val.to_string(),
                    )
                }
            },
            Ok(_) => (None, "Luau - ?".to_string()),
            Err(e) => (None, format!("Luau - cannot retrieve version: {e}")),
        };
        features.push(EnabledFeature {
            name: "luau",
            version,
            token,
        });
    }
    #[cfg(all(feature = "magika", feature = "feature_capable"))]
    features.push(EnabledFeature::plain("magika"));
    #[cfg(all(feature = "prompt", feature = "feature_capable"))]
    features.push(EnabledFeature::plain("prompt"));

    #[cfg(all(feature = "python", not(feature = "lite")))]
    {
        let pyversion = pyo3::Python::attach(|_| pyo3::Python::version_str().to_string());
        features.push(EnabledFeature {
            name:    "python",
            token:   format!("python-{pyversion}"),
            version: Some(pyversion),
        });
    }
    #[cfg(all(feature = "to", not(feature = "lite")))]
    features.push(EnabledFeature::plain("to"));
    #[cfg(all(feature = "viz", not(feature = "lite")))]
    features.push(EnabledFeature::plain("viz"));
    #[cfg(all(feature = "viz_static", not(feature = "lite")))]
    features.push(EnabledFeature::plain("viz_static"));
    #[allow(clippy::const_is_empty)]
    #[cfg(all(feature = "polars", not(feature = "lite")))]
    {
        let detail = if QSV_POLARS_REV.is_empty() {
            polars::VERSION.to_string()
        } else {
            format!("{}:{}", polars::VERSION, QSV_POLARS_REV)
        };
        features.push(EnabledFeature {
            name:    "polars",
            token:   format!("polars-{detail}"),
            version: Some(polars::VERSION.to_string()),
        });
    }
    #[cfg(feature = "self_update")]
    features.push(EnabledFeature::plain("self_update"));

    features
}

/// `--capabilities`: a machine-readable summary of this binary, for programs that drive qsv
/// as a subprocess and need to know what it can do without parsing `--version`.
pub fn capabilities_json(commands: &[&str]) -> String {
    let features = enabled_features();
    let feature_versions: serde_json::Map<String, serde_json::Value> = features
        .iter()
        .filter_map(|f| {
            f.version
                .as_ref()
                .map(|v| (f.name.to_string(), serde_json::Value::String(v.clone())))
        })
        .collect();
    let caps = serde_json::json!({
        "binary": option_env!("CARGO_BIN_NAME").unwrap_or("qsv"),
        "version": env!("CARGO_PKG_VERSION"),
        "target": TARGET,
        "kind": QSV_KIND,
        "features": features.iter().map(|f| f.name).collect::<Vec<_>>(),
        "feature_versions": feature_versions,
        "commands": commands,
        "error_formats": ["text", "json"],
        "max_jobs": max_jobs(),
        "num_cpus": num_cpus(),
    });
    // safety: serializing a serde_json::Value cannot fail
    serde_json::to_string_pretty(&caps).unwrap()
}

pub fn version() -> String {
    let mut feature_tokens = String::new();
    for feature in enabled_features() {
        feature_tokens.push_str(&feature.token);
        // historical format: every token is `;`-terminated except self_update, which is last
        if feature.name != "self_update" {
            feature_tokens.push(';');
        }
    }
    feature_tokens.push('-');

    // get max_file_size & memory info. max_file_size is based on QSV_FREEMEMORY_HEADROOM_PCT
    // setting and is only enforced when qsv is running in "non-streaming" mode (i.e. needs to
    // load the entire file into memory).
    let mut sys = System::new();
    sys.refresh_memory();
    let avail_mem = sys.available_memory();
    let total_mem = sys.total_memory();
    let free_swap = sys.free_swap();
    let max_file_size = mem_file_check(Path::new(""), true, false).unwrap_or(0) as u64;

    // we also get System info to help with debugging and logging.
    // we just need the CPU model name, so we ask for nothing
    sys.refresh_cpu_list(sysinfo::CpuRefreshKind::nothing());
    let os_version = System::long_os_version().unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = System::kernel_long_version();
    let cpu_brand = sys
        .cpus()
        .first()
        .map_or("Unknown", |cpu| cpu.brand().trim());
    let physical_cpu_count = System::physical_core_count().unwrap_or(0);

    #[cfg(all(feature = "mimalloc", not(feature = "jemallocator")))]
    let malloc_kind = {
        let mimalloc_version = mimalloc::MiMalloc.version();
        format!("mimalloc {mimalloc_version}")
    };
    #[cfg(all(feature = "jemallocator", not(feature = "mimalloc")))]
    let malloc_kind = if BACKGROUND_THREADS_ACTIVE.load(std::sync::atomic::Ordering::Relaxed) {
        "jemalloc+bgthread"
    } else {
        "jemalloc"
    };
    #[cfg(not(any(feature = "mimalloc", feature = "jemallocator")))]
    let malloc_kind = "standard";
    let (qsvtype, maj, min, pat, pre, rustversion) = (
        option_env!("CARGO_BIN_NAME"),
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
        option_env!("CARGO_PKG_VERSION_PRE"),
        option_env!("CARGO_PKG_RUST_VERSION"),
    );
    if let (Some(qsvtype), Some(maj), Some(min), Some(pat), Some(pre), Some(rustversion)) =
        (qsvtype, maj, min, pat, pre, rustversion)
    {
        if pre.is_empty() {
            format!(
                "{qsvtype} {maj}.{min}.{pat}-{malloc_kind}-{feature_tokens}{maxjobs}-{numcpus};\
                 {max_file_size}-{free_swap}-{avail_mem}-{total_mem} ({TARGET} compiled with Rust \
                 {rustversion};{os_version}-{kernel_version};{cpu_brand}-{physical_cpu_count}) \
                 {QSV_KIND}",
                maxjobs = max_jobs(),
                numcpus = num_cpus(),
                max_file_size = indicatif::HumanBytes(max_file_size),
                free_swap = indicatif::HumanBytes(free_swap),
                avail_mem = indicatif::HumanBytes(avail_mem),
                total_mem = indicatif::HumanBytes(total_mem),
            )
        } else {
            format!(
                "{qsvtype} {maj}.{min}.\
                 {pat}-{pre}-{malloc_kind}-{feature_tokens}{maxjobs}-{numcpus};\
                 {max_file_size}-{free_swap}-{avail_mem}-{total_mem} ({TARGET} compiled with Rust \
                 {rustversion}) {QSV_KIND}",
                maxjobs = max_jobs(),
                numcpus = num_cpus(),
                max_file_size = indicatif::HumanBytes(max_file_size),
                free_swap = indicatif::HumanBytes(free_swap),
                avail_mem = indicatif::HumanBytes(avail_mem),
                total_mem = indicatif::HumanBytes(total_mem),
            )
        }
    } else {
        String::new()
    }
}

pub fn show_env_vars() {
    let mut env_var_set = false;
    for (n, v) in env::vars_os() {
        // safety: we know that the env::vars_os() will not fail
        let env_var = n.into_string().unwrap();
        #[cfg(all(feature = "mimalloc", not(feature = "jemallocator")))]
        if env_var.starts_with("QSV_")
            || env_var.starts_with("MIMALLOC_")
            || OTHER_ENV_VARS.contains(&env_var.to_ascii_lowercase().as_str())
        {
            env_var_set = true;
            woutinfo!("{env_var}: {v:?}");
        }
        #[cfg(all(feature = "jemallocator", not(feature = "mimalloc")))]
        if env_var.starts_with("QSV_")
            || env_var.starts_with("JEMALLOC_")
            || env_var == "MALLOC_CONF"
            || env_var == "_RJEM_MALLOC_CONF"
            || OTHER_ENV_VARS.contains(&env_var.to_ascii_lowercase().as_str())
        {
            env_var_set = true;
            woutinfo!("{env_var}: {v:?}");
        }
        #[cfg(not(any(feature = "mimalloc", feature = "jemallocator")))]
        if env_var.starts_with("QSV_")
            || OTHER_ENV_VARS.contains(&env_var.to_ascii_lowercase().as_str())
        {
            env_var_set = true;
            woutinfo!("{env_var}: {v:?}");
        }
        #[cfg(feature = "polars")]
        if env_var.starts_with("POLARS_") {
            env_var_set = true;
            woutinfo!("{env_var}: {v:?}");
        }
    }
    if !env_var_set {
        woutinfo!("No qsv-relevant environment variables set.");
    }
}
