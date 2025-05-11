use super::TWidget;
use cairo::Context;
use chrono::{Local, Locale, Timelike};
use input_linux::Key;
use std::time::{Duration, Instant};

pub struct TimeWidget {
    pub format: String,
    pub locale: String,
    pub changed: bool,
    pub active: bool,
    pub action: Key,
}

impl TimeWidget {
    pub fn new(format: String, locale: Option<String>, action: Key) -> Self {
        let locale = match locale {
            Some(l) => l,
            None => "POSIX".to_owned(),
        };
        Self {
            action,
            active: false,
            changed: false,
            format,
            locale,
        }
    }
}

impl TWidget for TimeWidget {
    fn render(
        &mut self,
        c: &Context,
        height: i32,
        button_left_edge: f64,
        button_width: u64,
        y_shift: f64,
    ) {
        let current_time = Local::now();
        let current_locale = Locale::try_from(self.locale.as_str()).unwrap_or(Locale::POSIX);
        let formatted_time = current_time
            .format_localized(&self.format, current_locale)
            .to_string();

        let time_extents = c.text_extents(&formatted_time).unwrap();
        c.move_to(
            button_left_edge + (button_width as f64 / 2.0 - time_extents.width() / 2.0).round(),
            y_shift + (height as f64 / 2.0 + time_extents.height() / 2.0).round(),
        );
        c.show_text(&formatted_time).unwrap();
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
        let now = Local::now();
        Some(Instant::now() + Duration::from_secs(60_u32.saturating_sub(now.second()) as u64))
    }
    fn changed(&self) -> bool {
        self.changed
    }

    fn active(&self) -> bool {
        self.active
    }

    fn reset_changed(&mut self) {
        self.changed = false;
    }
}
