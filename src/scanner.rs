use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

use crate::result::{ScanResult, PortState};

pub fn scan_port(ip: &str, port: u16, timeout_ms: u64) -> ScanResult {
    let address = format!("{}:{}", ip, port);
    let socket: SocketAddr = address.parse().unwrap();

    let timeout = Duration::from_millis(timeout_ms);

    match TcpStream::connect_timeout(&socket, timeout) {
        Ok(_) => ScanResult {
            ip: ip.to_string(),
            port,
            state: PortState::Open,
        },
        Err(err) => {
            if err.kind() == std::io::ErrorKind::TimedOut {
                ScanResult {
                    ip: ip.to_string(),
                    port,
                    state: PortState::Filtered,
                }
            } else {
                ScanResult {
                    ip: ip.to_string(),
                    port,
                    state: PortState::Closed,
                }
            }
        }
    }
}
