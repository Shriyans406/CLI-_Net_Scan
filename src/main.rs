mod cli;
mod target;
mod ports;
mod scanner;
mod result;
mod report;
mod stats;

use clap::Parser;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

use cli::Cli;
use ports::parse_ports;
use scanner::scan_port;
use target::parse_target;
use report::{build_report, print_human_readable};
use stats::ScanStats;

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
            json,
            verbose,
        } => {
            // -----------------------------
            // 2. Parse target
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
                        let result = scan_port(&ip.to_string(), port, timeout, verbose);
                        tx.send(result).unwrap();
                    });
                }
            }

            drop(tx);

            // -----------------------------
            // 5. Collect results
            // -----------------------------
            let mut results = Vec::new();
            for r in rx {
                results.push(r);
            }

            // -----------------------------
            // 6. Build report
            // -----------------------------
            let report = build_report(results);

            // -----------------------------
            // 7. Build stats (Phase 9)
            // -----------------------------
            let stats = ScanStats::from_report(&report);

            // -----------------------------
            // 8. Output
            // -----------------------------
            if json {
                report::print_json(&report, &stats);
            } else {
                print_human_readable(&report, &stats);
            }
        }
    }
}

