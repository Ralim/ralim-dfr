use cairo::Context;
use chrono::{Local, Locale};
use input_linux::{uinput::UInputHandle, Key};
use librsvg_rebind::{prelude::HandleExt, Rectangle};
use procfs::{CurrentSI, KernelStats};
use std::os::fd::AsRawFd;

use crate::{
    button_image::ButtonImage, config::ButtonConfig, constants::ICON_SIZE, cpu_uage::CPUUsage,
    graphics_load::try_load_image, toggle_key,
};

pub struct Button {
    pub image: ButtonImage,
    pub changed: bool,
    pub active: bool,
    pub action: Key,
    pub last_cpu: CPUUsage,
}

impl Button {
    pub fn with_config(cfg: ButtonConfig) -> Button {
        if let Some(text) = cfg.text {
            Button::new_text(text, cfg.action)
        } else if let Some(icon) = cfg.icon {
            Button::new_icon(&icon, cfg.theme, cfg.action)
        } else if let Some(time) = cfg.time {
            let locale = match cfg.locale {
                Some(l) => l,
                None => "POSIX".to_string(),
            };
            Button::new_time(cfg.action, time, locale)
        } else if let Some(_cpu) = cfg.processor {
            Button::new_processor(cfg.action)
        } else {
            panic!("Invalid config, a button must have either Text, Icon or Time")
        }
    }
    fn new_text(text: String, action: Key) -> Button {
        Button {
            action,
            active: false,
            changed: false,
            image: ButtonImage::Text(text),
            last_cpu: CPUUsage::default(),
        }
    }
    fn new_icon(path: impl AsRef<str>, theme: Option<impl AsRef<str>>, action: Key) -> Button {
        let image = try_load_image(path, theme).expect("failed to load icon");
        Button {
            action,
            image,
            active: false,
            changed: false,
            last_cpu: CPUUsage::default(),
        }
    }

    fn new_time(action: Key, format: String, locale: String) -> Button {
        Button {
            action,
            active: false,
            changed: false,
            image: ButtonImage::Time(format, locale),
            last_cpu: CPUUsage::default(),
        }
    }
    fn new_processor(action: Key) -> Button {
        Button {
            action,
            active: false,
            changed: false,
            image: ButtonImage::Processor(),
            last_cpu: CPUUsage::default(),
        }
    }
    pub fn render(
        &mut self,
        c: &Context,
        height: i32,
        button_left_edge: f64,
        button_width: u64,
        y_shift: f64,
    ) {
        match &self.image {
            ButtonImage::Text(text) => {
                let extents = c.text_extents(text).unwrap();
                c.move_to(
                    button_left_edge + (button_width as f64 / 2.0 - extents.width() / 2.0).round(),
                    y_shift + (height as f64 / 2.0 + extents.height() / 2.0).round(),
                );
                c.show_text(text).unwrap();
            }
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
            ButtonImage::Time(format, locale) => {
                let current_time = Local::now();
                let current_locale = Locale::try_from(locale.as_str()).unwrap_or(Locale::POSIX);
                let formatted_time = if format == "24hr" {
                    format!(
                        "{}:{}    {} {} {}",
                        current_time.format_localized("%H", current_locale),
                        current_time.format_localized("%M", current_locale),
                        current_time.format_localized("%a", current_locale),
                        current_time.format_localized("%-e", current_locale),
                        current_time.format_localized("%b", current_locale)
                    )
                } else {
                    format!(
                        "{}:{} {}    {} {} {}",
                        current_time.format_localized("%-l", current_locale),
                        current_time.format_localized("%M", current_locale),
                        current_time.format_localized("%p", current_locale),
                        current_time.format_localized("%a", current_locale),
                        current_time.format_localized("%-e", current_locale),
                        current_time.format_localized("%b", current_locale)
                    )
                };
                let time_extents = c.text_extents(&formatted_time).unwrap();
                c.move_to(
                    button_left_edge
                        + (button_width as f64 / 2.0 - time_extents.width() / 2.0).round(),
                    y_shift + (height as f64 / 2.0 + time_extents.height() / 2.0).round(),
                );
                c.show_text(&formatted_time).unwrap();
            }
            ButtonImage::Processor() => {
                let new_readings = self.last_cpu.sample();

                let text = format!(
                    "{}/{}/{}",
                    new_readings.system, new_readings.user, new_readings.idle
                );

                let text_extent = c.text_extents(&text).unwrap();
                c.move_to(
                    button_left_edge
                        + (button_width as f64 / 2.0 - text_extent.width() / 2.0).round(),
                    y_shift + (height as f64 / 2.0 + text_extent.height() / 2.0).round(),
                );
                c.show_text(&text).unwrap();
            }
        }
    }
    pub fn set_active<F>(&mut self, uinput: &mut UInputHandle<F>, active: bool)
    where
        F: AsRawFd,
    {
        if self.active != active {
            self.active = active;
            self.changed = true;

            toggle_key(uinput, self.action, active as i32);
        }
    }
}
