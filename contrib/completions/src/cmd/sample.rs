use clap::{arg, Command};

pub fn sample_cmd() -> Command {
    Command::new("sample").args([
        arg!(--seed),
        arg!(--rng),
        arg!(--"user-agent"),
        arg!(--timeout),
        arg!(--"max-size"),
        arg!(--bernoulli),
        arg!(--systematic),
        arg!(--stratified),
        arg!(--weighted),
        arg!(--cluster),
        arg!(--timeseries),
        arg!(--"ts-interval"),
        arg!(--"ts-start"),
        arg!(--"ts-adaptive"),
        arg!(--"ts-aggregate"),
        arg!(--"ts-input-tz"),
        arg!(--"ts-prefer-dmy"),
        arg!(--force),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
