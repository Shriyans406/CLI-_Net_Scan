mod cli;
mod target;
mod ports;
mod scanner;
mod result;

use cli::Cli;
use scanner::scan_port;
use ports::parse_ports;
use target::parse_target;
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
            // 4. Scan each IP and port
            // -----------------------------
            let workers = 50; // concurrency limit
let pool = ThreadPool::new(workers);

let (tx, rx) = channel();

for ip in targets {
    for port in &port_list {
        let tx = tx.clone();
        let ip = ip.clone();
        let timeout = timeout;

        pool.execute(move || {
            let result = scan_port(&ip, *port, timeout);
            tx.send(result).unwrap();
        });
    }
}

drop(tx); // VERY IMPORTANT

for result in rx {
    println!("{:?}", result);
}

            }
        }
    }
}
