use crate::{
    fonts::{FontConfig, Pattern},
    function_layer::FunctionLayer,
};
use anyhow::Error;
use cairo::FontFace;
use freetype::Library as FtLibrary;
use input_linux::Key;
use nix::{
    errno::Errno,
    sys::inotify::{AddWatchFlags, InitFlags, Inotify, WatchDescriptor},
};
use serde::Deserialize;
use std::{fs::read_to_string, os::fd::AsFd};

const USER_CFG_PATH: &str = "/etc/tiny-dfr/config.toml";

pub struct Config {
    pub show_button_outlines: bool,
    pub enable_pixel_shift: bool,
    pub font_face: FontFace,
    pub adaptive_brightness: bool,
    pub active_brightness: u32,
}

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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ButtonConfig {
    #[serde(alias = "Svg")]
    pub icon: Option<String>,
    pub text: Option<String>,
    pub theme: Option<String>,
    pub time: Option<String>,
    pub processor: Option<String>,
    pub memory: Option<String>,
    pub battery: Option<String>,
    pub locale: Option<String>,
    pub action: Key,
    pub stretch: Option<usize>,
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

fn load_config(width: u16) -> (Config, [FunctionLayer; 2]) {
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

pub struct ConfigManager {
    inotify_fd: Inotify,
    watch_desc: Option<WatchDescriptor>,
}

fn arm_inotify(inotify_fd: &Inotify) -> Option<WatchDescriptor> {
    let flags = AddWatchFlags::IN_MOVED_TO | AddWatchFlags::IN_CLOSE | AddWatchFlags::IN_ONESHOT;
    match inotify_fd.add_watch(USER_CFG_PATH, flags) {
        Ok(wd) => Some(wd),
        Err(Errno::ENOENT) => None,
        e => Some(e.unwrap()),
    }
}

impl ConfigManager {
    pub fn new() -> ConfigManager {
        let inotify_fd = Inotify::init(InitFlags::IN_NONBLOCK).unwrap();
        let watch_desc = arm_inotify(&inotify_fd);
        ConfigManager {
            inotify_fd,
            watch_desc,
        }
    }
    pub fn load_config(&self, width: u16) -> (Config, [FunctionLayer; 2]) {
        load_config(width)
    }
    pub fn update_config(
        &mut self,
        cfg: &mut Config,
        layers: &mut [FunctionLayer; 2],
        width: u16,
    ) -> bool {
        if self.watch_desc.is_none() {
            self.watch_desc = arm_inotify(&self.inotify_fd);
            return false;
        }
        let evts = match self.inotify_fd.read_events() {
            Ok(e) => e,
            Err(Errno::EAGAIN) => Vec::new(),
            r => r.unwrap(),
        };
        let mut ret = false;
        for evt in evts {
            if evt.wd != self.watch_desc.unwrap() {
                continue;
            }
            let parts = load_config(width);
            *cfg = parts.0;
            *layers = parts.1;
            ret = true;
            self.watch_desc = arm_inotify(&self.inotify_fd);
        }
        ret
    }
    pub fn fd(&self) -> &impl AsFd {
        &self.inotify_fd
    }
}
