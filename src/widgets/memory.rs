use cairo::Context;
use input_linux::Key;
use std::time::{Duration, Instant};

use crate::metrics::MemoryUsage;

use super::TWidget;

pub struct MemoryWidget {
    pub changed: bool,
    pub active: bool,
    pub action: Key,
    pub last_draw_time: Instant,
}

impl MemoryWidget {
    pub fn new(_text: String, action: Key) -> Self {
        Self {
            action,
            active: false,
            changed: false,
            last_draw_time: Instant::now(),
        }
    }
}

impl TWidget for MemoryWidget {
    fn render(
        &mut self,
        c: &Context,
        height: i32,
        button_left_edge: f64,
        button_width: u64,
        y_shift: f64,
    ) {
        let readings = MemoryUsage::sample();

        let text = format!("{}%", readings.used);

        let text_extent = c.text_extents(&text).unwrap();
        c.move_to(
            button_left_edge + (button_width as f64 / 2.0 - text_extent.width() / 2.0).round(),
            y_shift + (height as f64 / 2.0 + text_extent.height() / 2.0).round(),
        );
        c.show_text(&text).unwrap();
        self.last_draw_time = Instant::now();
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
        Some(self.last_draw_time + Duration::from_secs(5))
    }
    fn changed(&self) -> bool {
        self.changed || self.last_draw_time.elapsed().as_secs() > 4
    }
    fn active(&self) -> bool {
        self.active
    }

    fn reset_changed(&mut self) {
        self.changed = false;
    }
}
