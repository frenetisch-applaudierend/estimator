use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Options {
    #[arg(short, long, default_value = "0.0.0.0")]
    pub host: String,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    #[arg(short, long)]
    pub assets_dir: PathBuf,
}

pub fn init() -> Options {
    Options::parse()
}
