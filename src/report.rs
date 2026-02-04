use crate::result::{ScanMap, PortState};
use crate::stats::ScanStats;
use serde_json::json;
use std::fs::File;
use std::io::Write;

pub fn build_report(results: Vec<crate::result::ScanResult>) -> ScanMap {
    let mut map: ScanMap = ScanMap::new();

    for result in results {
        map.entry(result.ip.clone())
            .or_insert_with(Vec::new)
            .push(result);
    }

    map
}

/* ---------- HUMAN READABLE ---------- */

pub fn print_human_readable(report: &ScanMap, stats: &ScanStats) {
    for (ip, results) in report {
        println!("Host: {}", ip);

        let open_ports: Vec<u16> = results
            .iter()
            .filter(|r| matches!(r.state, PortState::Open))
            .map(|r| r.port)
            .collect();

        if open_ports.is_empty() {
            println!("  No open ports found");
        } else {
            println!("  Open ports: {:?}", open_ports);
        }

        println!();
    }

    print_stats(stats);
}

/* ---------- JSON ---------- */

pub fn print_json(report: &ScanMap, stats: &ScanStats) -> String {
    let mut output = serde_json::Map::new();

    for (ip, results) in report {
        let open_ports: Vec<u16> = results
            .iter()
            .filter(|r| matches!(r.state, PortState::Open))
            .map(|r| r.port)
            .collect();

        output.insert(
            ip.clone(),
            json!({
                "open_ports": open_ports
            }),
        );
    }

    json!({
        "report": output,
        "stats": {
            "total_ports": stats.total,
            "open": stats.open,
            "closed": stats.closed,
            "filtered": stats.filtered
        }
    })
    .to_string()
}

/* ---------- FILE EXPORT ---------- */

pub fn write_to_file(path: &str, content: &str) {
    let mut file = File::create(path)
        .expect("Failed to create output file");
    file.write_all(content.as_bytes())
        .expect("Failed to write output file");
}

/* ---------- STATS ---------- */

fn print_stats(stats: &ScanStats) {
    println!("Scan Summary");
    println!("------------");
    println!("Total ports scanned : {}", stats.total);
    println!("Open                : {}", stats.open);
    println!("Closed              : {}", stats.closed);
    println!("Filtered            : {}", stats.filtered);
}

