use clap::{arg, Command};

pub fn pivotp_cmd() -> Command {
    Command::new("pivotp").args([
        arg!(--index),
        arg!(--values),
        arg!(--agg),
        arg!(--"sort-columns"),
        arg!(--"col-separator"),
        arg!(--validate),
        arg!(--"try-parsedates"),
        arg!(--"infer-len"),
        arg!(--"decimal-comma"),
        arg!(--"ignore-errors"),
        arg!(--output),
        arg!(--delimiter),
        arg!(--quiet),
    ])
}
