mod cli;
mod target;
mod discovery;
mod scanner;
mod output;
mod errors;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();

    match args.command {
        cli::Commands::Scan {
            target,
            ports,
            timeout,
        } => {
            match target::parse_target(&target) {
                Ok(networks) => {
                    println!("Parsed target: {:?}", networks);
                    println!("Ports: {}", ports);
                    println!("Timeout: {}s", timeout);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}
