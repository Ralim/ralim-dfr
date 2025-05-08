use procfs::{Current, Meminfo};

use super::Percent;

#[derive(Debug, Default)]
pub struct MemoryUsage {}

#[derive(Debug, Default)]
pub struct MemorySample {
    pub used: Percent,
}
impl MemoryUsage {
    pub fn sample() -> MemorySample {
        if let Some(sample) = Self::take_stats() {
            let available_mem = sample.mem_available.unwrap_or_default();
            MemorySample {
                used: 100_u8 - ((available_mem * 100) / sample.mem_total) as u8,
            }
        } else {
            MemorySample::default()
        }
    }
    fn take_stats() -> Option<Meminfo> {
        procfs::Meminfo::current().ok()
    }
}
