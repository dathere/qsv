use clap::{arg, Command};

pub fn split_cmd() -> Command {
    Command::new("split").args([
        arg!(--size),
        arg!(--chunks),
        arg!(--"kb-size"),
        arg!(--jobs),
        arg!(--filename),
        arg!(--pad),
        arg!(--filter),
        arg!(--"filter-cleanup"),
        arg!(--"filter-ignore-errors"),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--quiet),
    ])
}
