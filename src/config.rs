use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct OptiConfig {
    pub level: u8,
    pub overwrite: bool,
}

impl Default for OptiConfig {
    fn default() -> Self {
        Self {
            level: 2,
            overwrite: true,
        }
    }
}

pub fn load_config() -> OptiConfig {
    let config_path = Path::new("optirust.toml");

    if config_path.exists() {
        let content = fs::read_to_string(config_path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        OptiConfig::default()
    }
}
