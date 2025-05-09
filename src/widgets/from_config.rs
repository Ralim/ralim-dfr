use super::{
    battery::BatteryWidget, image_button::ImageButton, memory::MemoryWidget,
    processor::ProcessorWidget, TWidget, TextButton, TimeWidget,
};
use crate::config::ButtonConfig;

pub fn new_widget_from_config(cfg: ButtonConfig) -> Box<dyn TWidget> {
    if let Some(text) = cfg.text {
        Box::new(TextButton::new(&text, cfg.action))
    } else if let Some(icon) = cfg.icon {
        Box::new(ImageButton::new(&icon, cfg.theme, cfg.action))
    } else if let Some(text) = cfg.processor {
        Box::new(ProcessorWidget::new(text, cfg.action))
    } else if let Some(text) = cfg.memory {
        Box::new(MemoryWidget::new(text, cfg.action))
    } else if let Some(format) = cfg.time {
        Box::new(TimeWidget::new(format, cfg.locale, cfg.action))
    } else if let Some(text) = cfg.battery {
        Box::new(BatteryWidget::new(text, cfg.action))
    } else {
        eprintln!("Cant handle config {:?}", cfg);
        panic!("Invalid Widget Config");
    }
}
