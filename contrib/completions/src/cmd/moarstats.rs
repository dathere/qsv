use clap::{arg, Command};

pub fn moarstats_cmd() -> Command {
    Command::new("moarstats").args([
        arg!(--advanced),
        arg!(--epsilon),
        arg!(--"stats-options"),
        arg!(--round),
        arg!(--"use-percentiles"),
        arg!(--"pct-thresholds"),
        arg!(--"xsd-gdate-scan"),
        arg!(--bivariate),
        arg!(--"bivariate-stats"),
        arg!(--"cardinality-threshold"),
        arg!(--"join-inputs"),
        arg!(--"join-keys"),
        arg!(--"join-type"),
        arg!(--progressbar),
        arg!(--force),
        arg!(--jobs),
        arg!(--output),
    ])
}
