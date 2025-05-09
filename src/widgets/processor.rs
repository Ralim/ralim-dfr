use super::TWidget;
use crate::metrics::{CPUSample, CPUUsage};
use cairo::Context;
use input_linux::Key;
use std::time::{Duration, Instant};

pub struct ProcessorWidget {
    last_cpu: CPUUsage,
    changed: bool,
    active: bool,
    action: Key,
    last_sample_time: Instant,
    last_cpu_readings: CPUSample,
}

impl ProcessorWidget {
    pub fn new(_text: String, action: Key) -> Self {
        Self {
            action,
            active: false,
            changed: false,
            last_cpu: CPUUsage::default(),
            last_cpu_readings: CPUSample::default(),
            last_sample_time: Instant::now() - Duration::from_millis(4500),
        }
    }
}

impl TWidget for ProcessorWidget {
    fn render(
        &mut self,
        c: &Context,
        height: i32,
        button_left_edge: f64,
        button_width: u64,
        y_shift: f64,
    ) {
        if self.last_sample_time.elapsed().as_secs() > 4 {
            let new_readings = self.last_cpu.sample();
            self.last_cpu_readings = new_readings;
            self.last_sample_time = Instant::now();
        };
        // Make text coloured if load is high
        if self.last_cpu_readings.idle > 0 || self.last_cpu_readings.user > 0 {
            let scaled_bg = (self.last_cpu_readings.idle as f64) / 100.0;
            c.set_source_rgb(1.0, scaled_bg, scaled_bg);
        }

        let text = format!(
            "U: {}% S: {}% I: {}%",
            self.last_cpu_readings.user, self.last_cpu_readings.system, self.last_cpu_readings.idle
        );

        let text_extent = c.text_extents(&text).unwrap();
        c.move_to(
            button_left_edge + (button_width as f64 / 2.0 - text_extent.width() / 2.0).round(),
            y_shift + (height as f64 / 2.0 + text_extent.height() / 2.0).round(),
        );
        c.show_text(&text).unwrap();
    }
    fn set_active(&mut self, active: bool) -> bool {
        if self.active != active {
            self.active = active;
            self.changed = true;
            true
        } else {
            false
        }
    }

    fn get_action(&self) -> Key {
        self.action
    }
    fn next_draw_time(&self) -> Option<Instant> {
        Some(self.last_sample_time + Duration::from_secs(5))
    }
    fn changed(&self) -> bool {
        self.changed || self.last_sample_time.elapsed().as_secs() > 4
    }
    fn active(&self) -> bool {
        self.active
    }

    fn reset_changed(&mut self) {
        self.changed = false;
    }
}
