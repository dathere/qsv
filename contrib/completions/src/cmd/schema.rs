use clap::{arg, Command};

pub fn schema_cmd() -> Command {
    Command::new("schema").args([
        arg!(--"enum-threshold"),
        arg!(--"ignore-case"),
        arg!(--"strict-dates"),
        arg!(--"strict-formats"),
        arg!(--"pattern-columns"),
        arg!(--"date-whitelist"),
        arg!(--"prefer-dmy"),
        arg!(--force),
        arg!(--stdout),
        arg!(--polars),
        arg!(--jobs),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--memcheck),
    ])
}
