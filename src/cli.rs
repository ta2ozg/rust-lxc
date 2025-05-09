use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "rust-lxc", about = "LXC container management")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        #[clap(long)]
        name: String,
        #[clap(long)]
        template: String,
    },
    Start {
        #[clap(long)]
        name: String,
    },
    Stop {
        #[clap(long)]
        name: String,
    },
    Delete {
        #[clap(long)]
        name: String,
    },
    Ls,
    Shutdown {
        #[clap(long)]
        name: String,
    },
}