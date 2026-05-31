use clap::Parser;

use crate::cli::Cli;

mod cli;

pub fn run_cli() {
    let cli = Cli::parse();

    let name = cli.name;

    println!("my name is: {:?}", name);
}
