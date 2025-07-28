use clap::{arg, Command};

pub fn edit_cmd() -> Command {
    Command::new("edit").args([arg!(--"no-headers"), arg!(--"in-place"), arg!(--output)])
}
