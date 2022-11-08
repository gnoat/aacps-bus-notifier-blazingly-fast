use serde_derive::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize)]
pub struct Configs {
    pub schedule_url: String,
}

impl Configs {
    pub fn new(file_path: &str) -> Self {
        let text = fs::read_to_string(file_path).expect("Unable to open config file");
        let config: Configs = toml::from_str(&text[..]).expect("Cannot read config file.");

        config
    }
}
