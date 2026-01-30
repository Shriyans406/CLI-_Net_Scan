use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::time::Duration;

use crate::result::{ScanResult, PortState};

pub fn scan_port(ip: &str, port: u16, timeout_ms: u64) -> ScanResult {
    let addr_str = format!("{}:{}", ip, port);

    let socket_addr: SocketAddr = match addr_str.to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(a) => a,
            None => {
                return ScanResult {
                    ip: ip.to_string(),
                    port,
                    state: PortState::Closed,
                };
            }
        },
        Err(_) => {
            return ScanResult {
                ip: ip.to_string(),
                port,
                state: PortState::Closed,
            };
        }
    };

    let timeout = Duration::from_millis(timeout_ms);

    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_) => ScanResult {
            ip: ip.to_string(),
            port,
            state: PortState::Open,
        },
        Err(_) => ScanResult {
            ip: ip.to_string(),
            port,
            state: PortState::Closed,
        },
    }
}

