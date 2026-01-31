use ipnetwork::IpNetwork;

pub fn parse_target(input: &str) -> Result<Vec<IpNetwork>, String> {
    let network: IpNetwork = input
        .parse()
        .map_err(|e| format!("Invalid target '{}': {}", input, e))?;

    Ok(vec![network])
}

