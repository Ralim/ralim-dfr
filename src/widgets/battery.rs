use cairo::Context;
use input_linux::Key;
use starship_battery::Manager;
use std::time::{Duration, Instant};


use super::TWidget;

pub struct BatteryWidget {
    pub changed: bool,
    pub active: bool,
    pub action: Key,
    pub last_draw_time: Instant,
    pub manager: Manager,
}

impl BatteryWidget {
    pub fn new(_text: String, action: Key) -> Self {
        let manager = starship_battery::Manager::new().expect("Cant bind battery");
        // Load in the battery status icons (TODO)
        Self {
            action,
            manager,
            active: false,
            changed: false,
            last_draw_time: Instant::now(),
        }
    }
}

impl TWidget for BatteryWidget {
    fn render(
        &mut self,
        c: &Context,
        height: i32,
        button_left_edge: f64,
        button_width: u64,
        y_shift: f64,
    ) {
        if let Ok(batteries) = self.manager.batteries() {
            if let Some(battery) = batteries.filter_map(|b| b.ok()).next() {
                let soc = battery.state_of_charge();
                if soc.value < 0.2 {
                    c.set_source_rgb(1.0, 0.1, 0.1);
                } else if soc.value < 0.5 {
                    c.set_source_rgb(1.0, 0.5, 0.5);
                }
                match battery.state() {
                    starship_battery::State::Unknown => {}
                    starship_battery::State::Charging => c.set_source_rgb(0.0, 1.0, 0.0),
                    starship_battery::State::Discharging => {}
                    starship_battery::State::Empty => {}
                    starship_battery::State::Full => {}
                }

                let text = format!("{}%", soc.value * 100.0);

                let text_extent = c.text_extents(&text).unwrap();
                c.move_to(
                    button_left_edge
                        + (button_width as f64 / 2.0 - text_extent.width() / 2.0).round(),
                    y_shift + (height as f64 / 2.0 + text_extent.height() / 2.0).round(),
                );
                c.show_text(&text).unwrap();
                self.last_draw_time = Instant::now();
            }
        }
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
