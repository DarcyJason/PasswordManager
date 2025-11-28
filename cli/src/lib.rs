use clap::Parser;

use crate::{commands::parse_cli, options::Cli};

pub mod commands;
pub mod errors;
pub mod options;

pub fn run_cli() {
    let cli = Cli::parse();
    parse_cli(cli);
}
