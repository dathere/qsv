#[cfg(all(feature = "apply", feature = "feature_capable"))]
pub mod apply;
#[cfg(feature = "datapusher_plus")]
pub mod applydp;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod behead;
#[cfg(any(feature = "feature_capable", feature = "datapusher_plus"))]
pub mod blake3;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod cat;
#[cfg(any(feature = "feature_capable", feature = "datapusher_plus"))]
pub mod clean;
#[cfg(feature = "clipboard")]
pub mod clipboard;
#[cfg(feature = "color")]
pub mod color;
pub mod count;
pub mod datefmt;
pub mod dedup;
#[cfg(feature = "feature_capable")]
pub mod denull;
// describegpt is excluded from qsvlite (it pulls in ~3.8 MiB of LLM/cache/redis
// infrastructure that nothing else in the lite build uses). Use qsv, qsvmcp, or
// qsvdp for describegpt.
#[cfg(not(feature = "lite"))]
pub mod describegpt;
pub mod diff;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod edit;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod enumerate;
pub mod excel;
pub mod exclude;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod explode;
pub mod extdedup;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod extsort;
#[cfg(all(feature = "fetch", feature = "feature_capable"))]
pub mod fetch;
#[cfg(all(feature = "fetch", feature = "feature_capable"))]
pub mod fetchpost;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod fill;
#[cfg(feature = "feature_capable")]
pub mod fixedwidth;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod fixlengths;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod flatten;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod fmt;
#[cfg(all(feature = "foreach", not(feature = "lite")))]
pub mod foreach;
pub mod frequency;
#[cfg(feature = "geocode")]
pub mod geocode;
#[cfg(feature = "geocode")]
pub mod geoconvert;
#[cfg(feature = "get")]
pub mod get;
pub mod headers;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod implode;
pub mod index;
pub mod input;
pub mod join;
#[cfg(all(
    feature = "polars",
    any(feature = "feature_capable", feature = "datapusher_plus")
))]
pub mod joinp;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod json;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod jsonl;
#[cfg(feature = "lens")]
pub mod lens;
#[cfg(feature = "mcp")]
pub mod log;
#[cfg(feature = "luau")]
pub mod luau;
pub mod moarstats;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod partition;
#[cfg(all(
    feature = "polars",
    any(feature = "feature_capable", feature = "datapusher_plus")
))]
pub mod pivotp;
pub mod pragmastat;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod pro;
#[cfg(feature = "profile")]
pub mod profile;
#[cfg(feature = "prompt")]
pub mod prompt;
pub mod pseudo;
#[cfg(all(feature = "python", feature = "feature_capable"))]
pub mod python;
pub mod rename;
pub mod replace;
pub mod reverse;
pub mod safenames;
pub mod sample;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod schema;
#[cfg(all(
    feature = "polars",
    any(feature = "feature_capable", feature = "datapusher_plus")
))]
pub mod scoresql;
pub mod search;
pub mod searchset;
pub mod select;
pub mod slice;
pub mod snappy;
pub mod sniff;
pub mod sort;
pub mod sortcheck;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod split;
#[cfg(all(
    feature = "polars",
    any(feature = "feature_capable", feature = "datapusher_plus")
))]
pub mod sqlp;
pub mod stats;
#[cfg(all(feature = "synthesize", feature = "feature_capable"))]
pub mod synthesize;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod table;
#[cfg(any(feature = "feature_capable", feature = "datapusher_plus"))]
pub mod template;
#[cfg(all(feature = "to", feature = "feature_capable"))]
pub mod to;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod tojsonl;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod transpose;
pub mod validate;
// `viz` (and therefore `viz_static`, which enables it) does not work on big-endian
// targets. Fail loudly here rather than shipping a silently broken `viz` command:
// `distrib_features` and `all_features` both pull in `viz_static`, so a plain
// `cargo build -F distrib_features` on s390x would otherwise compile clean.
// NOTE powerpc64LE is LITTLE-endian, so this guard does not fire for ppc64le — viz is
// simply never enabled in that target's publish workflow.
#[cfg(all(feature = "viz", target_endian = "big"))]
compile_error!(
    "the `viz` feature (and `viz_static`, which enables it) is not supported on big-endian \
     targets. Build without them, e.g. drop `viz`/`viz_static` from --features, or use a feature \
     set that excludes them."
);

#[cfg(all(feature = "viz", feature = "feature_capable"))]
pub mod viz;
#[cfg(all(feature = "viz", feature = "feature_capable"))]
pub mod viz_census;
#[cfg(all(feature = "viz", feature = "feature_capable"))]
pub mod viz_i18n;
