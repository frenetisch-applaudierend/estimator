use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Options {
    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,

    #[arg(long, default_value_t = 8080)]
    pub port: u16,

    #[arg(long)]
    pub assets_dir: PathBuf,
}

pub fn init() -> Options {
    Options::parse()
}
