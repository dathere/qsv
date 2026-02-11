use clap::{arg, Command};

pub fn color_cmd() -> Command {
    Command::new("color").args([
        arg!(--color),
        arg!(--"row-numbers"),
        arg!(--title),
        arg!(--output),
        arg!(--delimiter),
        arg!(--memcheck),
    ])
}
