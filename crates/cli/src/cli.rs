use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dwake-cli")]
#[command(version, about = "smiple way to change docker restat policy")]
pub struct Cli {
    #[arg(short, long)]
    pub name: String,
}
