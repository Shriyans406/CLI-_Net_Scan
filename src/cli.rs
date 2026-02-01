use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "netscan")]
#[command(about = "A fast multi-threaded network scanner written in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Scan {
        /// Target IP, hostname, or CIDR
        target: String,

        /// Ports to scan (e.g. 22,80,1000-2000)
        #[arg(short, long, default_value = "1-1024")]
        ports: String,

        /// Connection timeout in milliseconds
        #[arg(short, long, default_value_t = 1000)]
        timeout: u64,

        /// Output in JSON format
        #[arg(long)]
        json: bool,

        /// Show verbose debug output
        #[arg(short, long)]
        verbose: bool,
    },
}

