use cairo::FontFace;

pub struct Config {
    pub show_button_outlines: bool,
    pub enable_pixel_shift: bool,
    pub font_face: FontFace,
    pub adaptive_brightness: bool,
    pub active_brightness: u32,
    pub dim_brightness: u32,
    pub off_brightness: u32,
}
