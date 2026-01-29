use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "netscan")]
#[command(author = "You")]
#[command(version = "1.0")]
#[command(about = "CLI Network Scanner")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Scan {
        target: String,

        #[arg(short, long, default_value = "1-1024")]
        ports: String,

        #[arg(short, long, default_value_t = 1)]
        timeout: u64,
    },
}

