use std::time::Instant;

use super::TWidget;
use crate::{button_image::ButtonImage, constants::ICON_SIZE, graphics_load::try_load_image};
use cairo::Context;
use input_linux::Key;
use librsvg_rebind::{prelude::HandleExt, Rectangle};

pub struct ImageButton {
    pub image: ButtonImage,
    pub changed: bool,
    pub active: bool,
    pub action: Key,
}

impl ImageButton {
    pub fn new(path: impl AsRef<str>, theme: Option<impl AsRef<str>>, action: Key) -> Self {
        let image = try_load_image(path, theme).expect("failed to load icon");
        Self {
            action,
            active: false,
            changed: false,
            image,
        }
    }
}

impl TWidget for ImageButton {
    fn render(
        &mut self,
        c: &Context,
        height: i32,
        button_left_edge: f64,
        button_width: u64,
        y_shift: f64,
    ) {
        match &self.image {
            ButtonImage::Svg(svg) => {
                let x =
                    button_left_edge + (button_width as f64 / 2.0 - (ICON_SIZE / 2) as f64).round();
                let y = y_shift + ((height as f64 - ICON_SIZE as f64) / 2.0).round();

                svg.render_document(c, &Rectangle::new(x, y, ICON_SIZE as f64, ICON_SIZE as f64))
                    .unwrap();
            }
            ButtonImage::Bitmap(surf) => {
                let x =
                    button_left_edge + (button_width as f64 / 2.0 - (ICON_SIZE / 2) as f64).round();
                let y = y_shift + ((height as f64 - ICON_SIZE as f64) / 2.0).round();
                c.set_source_surface(surf, x, y).unwrap();
                c.rectangle(x, y, ICON_SIZE as f64, ICON_SIZE as f64);
                c.fill().unwrap();
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
