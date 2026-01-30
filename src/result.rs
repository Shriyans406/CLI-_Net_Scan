#[derive(Debug)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
}

#[derive(Debug)]
pub struct ScanResult {
    pub ip: String,
    pub port: u16,
    pub state: PortState,
}
