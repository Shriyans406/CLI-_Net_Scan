use ipnetwork::IpNetwork;
use std::str::FromStr;

pub fn parse_target(input: &str) -> Result<Vec<IpNetwork>, String> {
    match IpNetwork::from_str(input) {
        Ok(net) => Ok(vec![net]),
        Err(_) => Err("Invalid IP or CIDR target".to_string()),
    }
}
