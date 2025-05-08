use procfs::{CpuTime, CurrentSI};

use super::Percent;

pub struct CPUUsage {
    last_cpu: Option<CpuTime>,
}

#[derive(Debug, Default)]
pub struct CPUSample {
    pub idle: Percent,
    pub user: Percent,
    pub system: Percent,
    pub nice: Percent,
    pub guest: Percent,
    pub guest_nice: Percent,
    pub iowait: Percent,
    pub irq: Percent,
    pub softirq: Percent,
    pub steal: Percent,
}
impl CPUUsage {
    pub fn sample(&mut self) -> CPUSample {
        let new_sample = Self::take_stats();
        let mut measurements = CPUSample::default();
        if let Some(old_reading) = &self.last_cpu {
            if let Some(new_readings) = &new_sample {
                let ticks_passed = Self::total_time(new_readings) - Self::total_time(old_reading);
                measurements.idle = ((new_readings.idle.saturating_sub(old_reading.idle) * 100)
                    / ticks_passed) as u8;
                measurements.nice = ((new_readings.nice.saturating_sub(old_reading.nice) * 100)
                    / ticks_passed) as u8;
                measurements.system = ((new_readings.system.saturating_sub(old_reading.system)
                    * 100)
                    / ticks_passed) as u8;
                measurements.user = ((new_readings.user.saturating_sub(old_reading.user) * 100)
                    / ticks_passed) as u8;
                measurements.guest = ((new_readings
                    .guest
                    .unwrap_or_default()
                    .saturating_sub(old_reading.guest.unwrap_or_default())
                    * 100)
                    / ticks_passed) as u8;
                measurements.guest_nice = ((new_readings
                    .guest_nice
                    .unwrap_or_default()
                    .saturating_sub(old_reading.guest_nice.unwrap_or_default())
                    * 100)
                    / ticks_passed) as u8;
                measurements.iowait = ((new_readings
                    .iowait
                    .unwrap_or_default()
                    .saturating_sub(old_reading.iowait.unwrap_or_default())
                    * 100)
                    / ticks_passed) as u8;
                measurements.irq = ((new_readings
                    .irq
                    .unwrap_or_default()
                    .saturating_sub(old_reading.irq.unwrap_or_default())
                    * 100)
                    / ticks_passed) as u8;
                measurements.softirq = ((new_readings
                    .softirq
                    .unwrap_or_default()
                    .saturating_sub(old_reading.softirq.unwrap_or_default())
                    * 100)
                    / ticks_passed) as u8;
                measurements.steal = ((new_readings
                    .steal
                    .unwrap_or_default()
                    .saturating_sub(old_reading.steal.unwrap_or_default())
                    * 100)
                    / ticks_passed) as u8;
            }
        }
        self.last_cpu = new_sample;
        measurements
    }
    fn take_stats() -> Option<CpuTime> {
        if let Ok(new_stats) = procfs::KernelStats::current() {
            Some(new_stats.total)
        } else {
            None
        }
    }
    fn total_time(t: &CpuTime) -> u64 {
        t.idle
            + t.nice
            + t.system
            + t.user
            + t.guest.unwrap_or_default()
            + t.guest_nice.unwrap_or_default()
            + t.iowait.unwrap_or_default()
            + t.irq.unwrap()
            + t.softirq.unwrap_or_default()
            + t.steal.unwrap_or_default()
    }
}
impl Default for CPUUsage {
    fn default() -> Self {
        Self {
            last_cpu: Self::take_stats(),
        }
    }
}
