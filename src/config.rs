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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Sem arquivo de configuração, o padrão é 2
    fn test_default_config_loading() {
        let config = load_config();
        assert_eq!(config.level, 2);
    }

    #[test]
    // Testa a leitura de um arquivo de configuração
    fn test_toml_parsing() {
        let toml_content = "level = 5\noverwrite = false";
        let config: OptiConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.level, 5);
        assert!(!config.overwrite);
    }
}
