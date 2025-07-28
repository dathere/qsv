use clap::{arg, Command};

pub fn geoconvert_cmd() -> Command {
    Command::new("geoconvert").args([
        arg!(--geometry),
        arg!(--latitude),
        arg!(--longitude),
        arg!(--"max-length"),
        arg!(--output),
    ])
}
