use clap::Parser;

use crate::{cli::Cli, commands::list::findContainer};

use common::{port::container::ContainerRuntime, service::container::ContainerService};

mod cli;
mod commands;

pub async fn run_cli<R>(service: &ContainerService<R>)
where
    R: ContainerRuntime + Send + Sync,
{
    let cli = Cli::parse();

    println!("{:?}", cli);

    let name = cli.locator.expect("arh");

    let action = cli.action;

    // println!("my name is: {:?}", name);
    println!("action: {:?}", action);

    let _ = findContainer(service, name.as_ref()).await;

    // let check = fuzzy::compare("kronborg", "mikkel");
    // println!("{check:?}");
}
