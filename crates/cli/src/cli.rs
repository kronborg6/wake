use clap::{Args, Parser, Subcommand};
use common::domain::container::{ContainerRestartPolicy, ContainerStateStatusEnum};

#[derive(Debug, Parser)]
#[command(name = "dwake-cli")]
#[command(version, about = "smiple way to change docker restat policy")]
pub struct Cli {
    #[arg(short = 'n', long, alias = "name", alias = "id")]
    pub locator: Option<String>,
    #[arg(short, long, requires = "locator")]
    pub action: Option<ContainerRestartPolicy>,
    #[arg(short, long)]
    pub list: bool,
    #[arg(short, long, requires = "locator")]
    pub status: Option<ContainerStateStatusEnum>,
}
