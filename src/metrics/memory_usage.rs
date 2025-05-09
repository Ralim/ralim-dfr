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
            let mut consumed_memory = sample.mem_total - sample.mem_free;
            // Dont count cached memory as it can be freed
            let cached = sample.cached;
            consumed_memory = consumed_memory.saturating_sub(cached);
            MemorySample {
                used: ((consumed_memory * 100) / sample.mem_total) as u8,
            }
        } else {
            MemorySample::default()
        }
    }
    fn take_stats() -> Option<Meminfo> {
        procfs::Meminfo::current().ok()
    }
}
