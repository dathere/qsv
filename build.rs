fn main() {
    // we use TARGET in --version and user-agent strings
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );
    // QSV_KIND is used to determine how qsv was built and is displayed in --version
    // check PERFORMANCE.md for more info
    println!(
        "cargo:rustc-env=QSV_KIND={}",
        std::env::var("QSV_KIND").unwrap_or_else(|_| "compiled".to_string())
    );

    // Once a build script prints any rerun-if-* line, cargo stops rerunning it on every
    // package change and reruns it only for the listed paths/vars, so list everything read here
    // (Cargo.toml carries the QSV_POLARS_REV marker).
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-env-changed=QSV_KIND");

    embed_tool_definitions();

    #[cfg(feature = "polars")]
    {
        use std::{fs, path::Path};

        // This is the string that is searched for in Cargo.toml to find the Polars revision
        const QSV_POLARS_REV: &str = "# QSV_POLARS_REV=";

        // QSV_POLARS_REV contains either the commit id short hash or the git tag
        // of the Polars version qsv was built against
        let cargo_toml_path = Path::new("Cargo.toml");
        let cargo_toml_content =
            fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");
        let polars_rev = cargo_toml_content.find(QSV_POLARS_REV).map_or_else(
            || "unknown".to_string(),
            |index| {
                let start_index = index + QSV_POLARS_REV.len();
                let end_index = cargo_toml_content[start_index..]
                    .find('\n')
                    .map_or(cargo_toml_content.len(), |i| start_index + i);
                let final_rev = cargo_toml_content[start_index..end_index]
                    .trim()
                    .to_string();
                if final_rev.is_empty() {
                    "release".to_string()
                } else {
                    final_rev
                }
            },
        );
        println!("cargo:rustc-env=QSV_POLARS_REV={polars_rev}");
    }
}

/// Generates `$OUT_DIR/embedded_tool_defs.rs`, which embeds the committed MCP skill JSONs
/// (`.claude/skills/qsv/qsv-<cmd>.json`) and help Markdown (`docs/help/<name>.md`) with
/// `include_str!` so installed binaries can export them (`--export-tool-definitions`,
/// `--tool-definition`). The generators that produce these files parse the repo's source, so
/// they can't run outside a checkout; the wiki-lint CI job keeps the committed copies current.
/// The directories are enumerated here so no hand-maintained file list can drift.
fn embed_tool_definitions() {
    use std::{fmt::Write as _, fs, path::Path};

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&manifest_dir);

    // collect (key, absolute path) pairs for files in `dir` whose name matches
    // `prefix`*`suffix`, keyed by the stem between them, sorted for deterministic output
    let collect = |dir: &str, prefix: &str, suffix: &str| -> Vec<(String, String)> {
        let dir_path = manifest_dir.join(dir);
        println!("cargo:rerun-if-changed={}", dir_path.display());
        let Ok(entries) = fs::read_dir(&dir_path) else {
            println!(
                "cargo:warning={} not found - no tool definitions will be embedded from it",
                dir_path.display()
            );
            return Vec::new();
        };
        let mut files: Vec<(String, String)> = entries
            .flatten()
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().into_owned();
                let key = name.strip_prefix(prefix)?.strip_suffix(suffix)?.to_string();
                if key.is_empty() {
                    return None;
                }
                Some((key, entry.path().to_string_lossy().into_owned()))
            })
            .collect();
        files.sort();
        files
    };

    let mut out = String::new();
    for (static_name, files) in [
        (
            "SKILL_JSONS",
            collect(".claude/skills/qsv", "qsv-", ".json"),
        ),
        ("HELP_MDS", collect("docs/help", "", ".md")),
    ] {
        writeln!(out, "pub static {static_name}: &[(&str, &str)] = &[").unwrap();
        for (key, path) in files {
            // Debug formatting escapes the path (e.g. Windows backslashes) as a string literal
            writeln!(out, "    ({key:?}, include_str!({path:?})),").unwrap();
        }
        writeln!(out, "];").unwrap();
    }

    let out_path = Path::new(&std::env::var("OUT_DIR").unwrap()).join("embedded_tool_defs.rs");
    fs::write(out_path, out).expect("Failed to write embedded_tool_defs.rs");
}
