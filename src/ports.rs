pub fn parse_ports(input: &str) -> Vec<u16> {
    let mut ports = Vec::new();

    for part in input.split(',') {
        if part.contains('-') {
            let bounds: Vec<&str> = part.split('-').collect();
            let start: u16 = bounds[0].parse().unwrap();
            let end: u16 = bounds[1].parse().unwrap();

            for p in start..=end {
                ports.push(p);
            }
        } else {
            ports.push(part.parse().unwrap());
        }
    }

    ports
}
