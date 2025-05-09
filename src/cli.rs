use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rust-lxc")]
#[command(about = "A CLI tool to manage LXC containers", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new LXC container
    Create {
        #[arg(short, long)]
        name: String,
    },
    /// Starts an LXC container
    Start {
        #[arg(short, long)]
        name: String,
    },
    /// Stops an LXC container
    Stop {
        #[arg(short, long)]
        name: String,
    },
    /// Deletes an LXC container
    Delete {
        #[arg(short, long)]
        name: String,
    },
    /// Lists an LXC containers on the system
    Ls,
    /// Shutdown an LXC container
    Shutdown {
        #[arg(short, long)]
        name: String,
    },
}
