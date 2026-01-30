mod cli;
mod target;
mod ports;
mod scanner;
mod result;
mod report;

use cli::Cli;
use scanner::scan_port;
use ports::parse_ports;
use target::parse_target;
use report::{build_report, print_human_readable};
use clap::Parser;


use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    // -----------------------------
    // 1. Parse CLI arguments
    // -----------------------------
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Scan {
            target,
            ports,
            timeout,
        } => {
            // -----------------------------
            // 2. Parse and validate target
            // -----------------------------
            let targets = match parse_target(&target) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Target error: {}", e);
                    return;
                }
            };

            // -----------------------------
            // 3. Parse ports
            // -----------------------------
            let port_list = parse_ports(&ports);

            // -----------------------------
            // 4. Concurrent scanning (Phase 4)
            // -----------------------------
        let workers = 50;
let pool = ThreadPool::new(workers);
let (tx, rx) = channel();

let mut all_results = Vec::new();

for ip in targets {
    for port in port_list.clone() {
        let tx = tx.clone();
        let ip = ip.clone();
        let timeout = timeout;

        pool.execute(move || {
            let ip_str = ip.to_string();
            let result = scan_port(&ip_str, port, timeout);
            tx.send(result).unwrap();
        });
    }
}

drop(tx);

for result in rx {
    all_results.push(result);
}

        }
    }
}

