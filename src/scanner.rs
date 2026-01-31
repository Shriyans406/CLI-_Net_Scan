use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::time::Duration;

use crate::result::{ScanResult, PortState};

pub fn scan_port(ip: &str, port: u16, timeout_ms: u64) -> ScanResult {
    // 🔴 FIX: remove CIDR if present (e.g. 127.0.0.1/32 → 127.0.0.1)
    let clean_ip = ip.split('/').next().unwrap();

    let addr_str = format!("{}:{}", clean_ip, port);
    println!("[DEBUG] Trying {}", addr_str);

    let socket_addr: SocketAddr = match addr_str.to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(a) => a,
            None => {
                println!("[DEBUG] No socket addr");
                return ScanResult {
                    ip: clean_ip.to_string(),
                    port,
                    state: PortState::Closed,
                };
            }
        },
        Err(e) => {
            println!("[DEBUG] Addr error: {}", e);
            return ScanResult {
                ip: clean_ip.to_string(),
                port,
                state: PortState::Closed,
            };
        }
    };

    let timeout = Duration::from_millis(timeout_ms);

    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_) => {
            println!("[DEBUG] CONNECT SUCCESS {}", addr_str);
            ScanResult {
                ip: clean_ip.to_string(),
                port,
                state: PortState::Open,
            }
        }
        Err(e) => {
            println!("[DEBUG] CONNECT FAIL {} → {}", addr_str, e);
            ScanResult {
                ip: clean_ip.to_string(),
                port,
                state: PortState::Closed,
            }
        }
    }
}

