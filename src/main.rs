mod cli;
mod target;
mod ports;
mod scanner;
mod result;
mod report;

use clap::Parser;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

use cli::Cli;
use ports::parse_ports;
use report::{build_report, print_human_readable};
use scanner::scan_port;
use target::parse_target;

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
            // 4. Concurrent scanning
            // -----------------------------
            let workers = 50;
            let pool = ThreadPool::new(workers);
            let (tx, rx) = channel();

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

            // Close sending side
            drop(tx);

            // -----------------------------
            // 5. Collect results
            // -----------------------------
            let mut results = Vec::new();
            for result in rx {
                results.push(result);
            }

            // -----------------------------
            // 6. Build & print report (Phase 5)
            // -----------------------------
            let report = build_report(results);
            print_human_readable(&report);
        }
    }
}

