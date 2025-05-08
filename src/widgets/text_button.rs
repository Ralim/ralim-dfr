use super::TWidget;
use cairo::Context;
use input_linux::Key;
use std::time::Instant;

pub struct TextButton {
    pub text: String,
    pub changed: bool,
    pub active: bool,
    pub action: Key,
}

impl TextButton {
    pub fn new(text: &str, action: Key) -> Self {
        Self {
            action,
            active: false,
            changed: false,
            text: text.to_owned(),
        }
    }
}

impl TWidget for TextButton {
    fn render(
        &mut self,
        c: &Context,
        height: i32,
        button_left_edge: f64,
        button_width: u64,
        y_shift: f64,
    ) {
        let extents = c.text_extents(&self.text).unwrap();
        c.move_to(
            button_left_edge + (button_width as f64 / 2.0 - extents.width() / 2.0).round(),
            y_shift + (height as f64 / 2.0 + extents.height() / 2.0).round(),
        );
        c.show_text(&self.text).unwrap();
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
        None
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
