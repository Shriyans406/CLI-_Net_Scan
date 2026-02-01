use crate::result::{ScanMap, ScanResult, PortState};
use serde_json::json;



pub fn build_report(results: Vec<ScanResult>) -> ScanMap {
    let mut map: ScanMap = ScanMap::new();

    for result in results {
        map.entry(result.ip.clone())
            .or_insert_with(Vec::new)
            .push(result);
    }

    map
}

pub fn print_human_readable(report: &ScanMap) {
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
}

pub fn print_json(report: &ScanMap) {
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

    println!(
        "{}",
        serde_json::to_string_pretty(&output).unwrap()
    );
}

