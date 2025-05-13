use input_linux::Key;
use serde::Deserialize;

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
