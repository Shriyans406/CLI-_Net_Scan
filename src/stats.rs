use crate::result::{ScanMap, PortState};

#[derive(Debug)]
pub struct ScanStats {
    pub total: usize,
    pub open: usize,
    pub closed: usize,
    pub filtered: usize,
}

impl ScanStats {
    pub fn from_report(report: &ScanMap) -> Self {
        let mut stats = ScanStats {
            total: 0,
            open: 0,
            closed: 0,
            filtered: 0,
        };

        for results in report.values() {
            for r in results {
                stats.total += 1;
                match r.state {
                    PortState::Open => stats.open += 1,
                    PortState::Closed => stats.closed += 1,
                    PortState::Filtered => stats.filtered += 1,
                }
            }
        }

        stats
    }
}

