use crate::{
    fonts::{FontConfig, Pattern},
    function_layer::FunctionLayer,
};
use anyhow::Error;
use cairo::FontFace;
use freetype::Library as FtLibrary;
use input_linux::Key;
use serde::Deserialize;
use std::fs::read_to_string;

use super::{USER_CFG_PATH, config_struct::Config, widget::ButtonConfig};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ConfigProxy {
    show_button_outlines: Option<bool>,
    enable_pixel_shift: Option<bool>,
    font_template: Option<String>,
    adaptive_brightness: Option<bool>,
    active_brightness: Option<u32>,
    primary_layer_keys: Option<Vec<ButtonConfig>>,
    fn_layer_keys: Option<Vec<ButtonConfig>>,
}

pub fn load_config(width: u16) -> (Config, [FunctionLayer; 2]) {
    let mut base =
        toml::from_str::<ConfigProxy>(&read_to_string("/usr/share/tiny-dfr/config.toml").unwrap())
            .unwrap();
    let user = read_to_string(USER_CFG_PATH)
        .map_err::<Error, _>(|e| e.into())
        .and_then(|r| Ok(toml::from_str::<ConfigProxy>(&r)?));
    if let Ok(user) = user {
        base.show_button_outlines = user.show_button_outlines.or(base.show_button_outlines);
        base.enable_pixel_shift = user.enable_pixel_shift.or(base.enable_pixel_shift);
        base.font_template = user.font_template.or(base.font_template);
        base.adaptive_brightness = user.adaptive_brightness.or(base.adaptive_brightness);
        base.fn_layer_keys = user.fn_layer_keys.or(base.fn_layer_keys);
        base.primary_layer_keys = user.primary_layer_keys.or(base.primary_layer_keys);
        base.active_brightness = user.active_brightness.or(base.active_brightness);
    };
    let mut media_layer_keys = base.fn_layer_keys.unwrap();
    let mut primary_layer_keys = base.primary_layer_keys.unwrap();
    // If the device doesn't have a physical Esc key, inject a soft one
    if width >= 2170 {
        for layer in [&mut media_layer_keys, &mut primary_layer_keys] {
            layer.insert(
                0,
                ButtonConfig {
                    text: Some("esc".into()),
                    action: Key::Esc,
                    icon: None,
                    theme: None,
                    stretch: None,
                    time: None,
                    locale: None,
                    battery: None,
                    processor: None,
                    memory: None,
                },
            );
        }
    }
    let fn_layer = FunctionLayer::with_config(media_layer_keys);
    let primary_layer = FunctionLayer::with_config(primary_layer_keys);

    let cfg = Config {
        show_button_outlines: base.show_button_outlines.unwrap(),
        enable_pixel_shift: base.enable_pixel_shift.unwrap(),
        adaptive_brightness: base.adaptive_brightness.unwrap(),
        font_face: load_font(&base.font_template.unwrap()),
        active_brightness: base.active_brightness.unwrap(),
    };
    (cfg, [primary_layer, fn_layer])
}

fn load_font(name: &str) -> FontFace {
    let fontconfig = FontConfig::new();
    let mut pattern = Pattern::new(name);
    fontconfig.perform_substitutions(&mut pattern);
    let pat_match = match fontconfig.match_pattern(&pattern) {
        Ok(pat) => pat,
        Err(_) => panic!(
            "Unable to find specified font. If you are using the default config, make sure you have at least one font installed"
        ),
    };
    let file_name = pat_match.get_file_name();
    let file_idx = pat_match.get_font_index();
    let ft_library = FtLibrary::init().unwrap();
    let face = ft_library.new_face(file_name, file_idx).unwrap();
    FontFace::create_from_ft(&face).unwrap()
}
