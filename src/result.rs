use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum PortState {
    Open,
    Closed,
    //Filtered,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub ip: String,
    pub port: u16,
    pub state: PortState,
}

pub type ScanMap = HashMap<String, Vec<ScanResult>>;

