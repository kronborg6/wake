use clap::Parser;

use crate::cli::Cli;

mod cli;
mod commands;

pub fn run_cli() {
    let cli = Cli::parse();

    println!("{:?}", cli);

    let name = cli.locator;

    let action = cli.action;

    println!("my name is: {:?}", name);
    println!("action: {:?}", action);
}
