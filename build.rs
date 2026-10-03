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

    embed_command_metadata();

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

/// Writes `$OUT_DIR/cmd_meta.rs`: the few source-tree inputs the tool-definition and
/// help-Markdown generators need beyond each command's USAGE text (which is already compiled
/// in). This lets an installed qsv/qsvmcp generate definitions for itself at runtime
/// (`<command> --help --format json|md`, `--export-tool-definitions`).
///
/// Raw source fragments are embedded rather than parsed here, so the runtime path runs the
/// same parsers on them as `--update-mcp-skills`/`--generate-help-md` run on the full files.
/// No `rerun-if-changed` lines: cargo's default reruns this on any change to a packaged file,
/// which covers README.md and src/.
fn embed_command_metadata() {
    use std::{fmt::Write as _, fs, path::Path};

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = Path::new(&manifest_dir);

    let readme = fs::read_to_string(root.join("README.md")).unwrap_or_default();
    let util_src = fs::read_to_string(root.join("src/util.rs")).unwrap_or_default();

    let mut out = String::new();
    let fragment = readme_fragment(&readme);
    writeln!(out, "pub static README_FRAGMENT: &str = {fragment:?};").unwrap();

    // (source stem, source path, args fragment) for each src/cmd/<x>.rs and src/cmd/<x>/mod.rs
    let mut sources = Vec::new();
    if let Ok(entries) = fs::read_dir(root.join("src/cmd")) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            let (stem, rel) = if path.is_dir() {
                (name.clone(), format!("src/cmd/{name}/mod.rs"))
            } else if let Some(stem) = name.strip_suffix(".rs") {
                if stem == "mod" {
                    continue;
                }
                (stem.to_string(), format!("src/cmd/{name}"))
            } else {
                continue;
            };
            let Ok(src) = fs::read_to_string(root.join(&rel)) else {
                continue;
            };
            let Some(args_src) = args_fragment(&src, &util_src) else {
                continue;
            };
            sources.push((stem, rel, args_src));
        }
    }
    sources.sort();
    writeln!(out, "pub static CMD_SOURCES: &[(&str, &str, &str)] = &[").unwrap();
    for (stem, rel, args_src) in sources {
        writeln!(out, "    ({stem:?}, {rel:?}, {args_src:?}),").unwrap();
    }
    writeln!(out, "];").unwrap();

    let out_path = Path::new(&std::env::var("OUT_DIR").unwrap()).join("cmd_meta.rs");
    fs::write(out_path, out).expect("Failed to write cmd_meta.rs");
}

/// The README lines the generators read: the command-table rows (in their original order) and
/// the legend, from its anchor up to the next heading. The generators stop reading the legend
/// at the first heading, horizontal rule or footnote, so this is a superset of what they see.
fn readme_fragment(readme: &str) -> String {
    let mut fragment = String::new();
    for line in readme.lines() {
        if line.contains("](docs/help/") || line.contains("](/src/cmd/") {
            fragment.push_str(line);
            fragment.push('\n');
        }
    }
    if let Some(start) = readme.find("<a name=\"legend_deeplink\">") {
        for line in readme[start..].lines() {
            fragment.push_str(line);
            fragment.push('\n');
            if line.trim_start().starts_with('#') {
                break;
            }
        }
    }
    fragment
}

/// The parts of a command's source that the generators read for option types: its
/// `util::get_args` line(s), its `struct Args` block, and the block of the struct named in the
/// `get_args` annotation (looked up in the command's source, then in `src/util.rs`).
/// `None` for a module with no `util::get_args` call (not a command).
fn args_fragment(src: &str, util_src: &str) -> Option<String> {
    let get_args_lines: Vec<&str> = src
        .lines()
        .filter(|l| l.trim_start().starts_with("let ") && l.contains("util::get_args"))
        .collect();
    let first = get_args_lines.first()?;

    let mut fragment = String::new();
    for line in &get_args_lines {
        fragment.push_str(line);
        fragment.push('\n');
    }
    if let Some(block) = struct_block(src, "Args") {
        fragment.push_str(block);
        fragment.push('\n');
    }
    // `let [mut] args: [util::]Name = util::get_args(..)`
    let annotated = first
        .split_once(':')
        .and_then(|(_, rest)| rest.split_once('='))
        .map(|(ty, _)| ty.trim().trim_start_matches("util::").to_string());
    if let Some(name) = annotated.filter(|n| n != "Args")
        && let Some(block) = struct_block(src, &name).or_else(|| struct_block(util_src, &name))
    {
        fragment.push_str(block);
        fragment.push('\n');
    }
    Some(fragment)
}

/// The text of the declaration of `struct <name>`, from the start of its line through the
/// matching closing brace. Only lines that declare the struct (optionally `pub`/`pub(..)`)
/// match, not mentions in comments or strings.
fn struct_block<'a>(src: &'a str, name: &str) -> Option<&'a str> {
    let mut offset = 0;
    for line in src.split_inclusive('\n') {
        let line_start = offset;
        offset += line.len();
        let mut decl = line.trim_start();
        if let Some(rest) = decl.strip_prefix("pub") {
            decl = match rest.strip_prefix('(') {
                Some(r) => r.split_once(')').map_or(rest, |(_, after)| after),
                None => rest,
            }
            .trim_start();
        }
        let Some(after) = decl
            .strip_prefix("struct ")
            .and_then(|d| d.trim_start().strip_prefix(name))
        else {
            continue;
        };
        if after
            .chars()
            .next()
            .is_some_and(|c| c.is_alphanumeric() || c == '_')
        {
            continue;
        }
        let open = line_start + src[line_start..].find('{')?;
        let mut depth = 0usize;
        for (i, c) in src[open..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(&src[line_start..=open + i]);
                    }
                },
                _ => {},
            }
        }
        return None;
    }
    None
}
