use clap::{arg, Command};

pub fn lens_cmd() -> Command {
    Command::new("lens").args([
        arg!(--delimiter),
        arg!(--"tab-separated"),
        arg!(--"no-headers"),
        arg!(--columns),
        arg!(--filter),
        arg!(--find),
        arg!(--"ignore-case"),
        arg!(--"freeze-columns"),
        arg!(--monochrome),
        arg!(--"wrap-mode"),
        arg!(--prompt),
        arg!(--"echo-column"),
        arg!(--debug),
    ])
}
