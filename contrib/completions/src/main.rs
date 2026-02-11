mod usage_parser;

use std::{io, process::exit};

use clap_complete::{
    generate,
    shells::{Bash, Elvish, Fish, PowerShell, Zsh},
};
use clap_complete_fig::Fig;
use clap_complete_nushell::Nushell;

const VALID_SHELLS: &[&str] = &[
    "bash",
    "zsh",
    "fish",
    "powershell",
    "nushell",
    "fig",
    "elvish",
];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args_error =
        "Please provide an argument of either: bash, zsh, fish, powershell, nushell, fig, elvish";
    if args.len() != 2 {
        println!("{args_error}");
        exit(1);
    }

    // Validate the shell argument before doing any expensive work
    let shell = args[1].as_str();
    if !VALID_SHELLS.contains(&shell) {
        println!("{args_error}");
        exit(1);
    }

    let repo_root = usage_parser::find_repo_root().unwrap_or_else(|| {
        eprintln!(
            "Error: Must be run from within the qsv repository \
             (where Cargo.toml and src/cmd/ exist)."
        );
        exit(1);
    });

    let mut cmd = usage_parser::build_cli(&repo_root);

    match shell {
        "bash" => generate(Bash, &mut cmd, "qsv", &mut io::stdout()),
        "zsh" => generate(Zsh, &mut cmd, "qsv", &mut io::stdout()),
        "fish" => generate(Fish, &mut cmd, "qsv", &mut io::stdout()),
        "powershell" => generate(PowerShell, &mut cmd, "qsv", &mut io::stdout()),
        "nushell" => generate(Nushell, &mut cmd, "qsv", &mut io::stdout()),
        "fig" => generate(Fig, &mut cmd, "qsv", &mut io::stdout()),
        "elvish" => generate(Elvish, &mut cmd, "qsv", &mut io::stdout()),
        _ => unreachable!(),
    }
}
