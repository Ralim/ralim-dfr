mod config_file;
mod config_struct;
mod manager;
mod widget;

const USER_CFG_PATH: &str = "/etc/tiny-dfr/config.toml";

pub use self::config_struct::Config;
pub use self::manager::*;
pub use self::widget::*;
