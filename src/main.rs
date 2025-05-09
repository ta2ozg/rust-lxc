use clap::Parser;
use anyhow::Result;
use rust_lxc::cli::{Cli, Commands};
use rust_lxc::commands;

fn main() -> Result<()> {
    let cli = Cli::try_parse()?; // add try_parse

    match cli.command {
        Commands::Create { name } => commands::create::run(&name)?,
        Commands::Start { name } => commands::start::run(&name)?,
        Commands::Stop { name } => commands::stop::run(&name)?,
        Commands::Delete { name } => commands::delete::run(&name)?,
        Commands::Ls => commands::ls::run()?,
        Commands::Shutdown { name } => commands::shutdown::run(&name)?,
    }

    Ok(())
}
