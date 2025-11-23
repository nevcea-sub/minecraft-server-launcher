use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use log::warn;

use crate::constants::*;

const CONFIG_EXAMPLE: &str = r#"# Minecraft Server Launcher Configuration
# Edit this file to customize server settings

# Minecraft version (use "latest" for the latest version)
minecraft_version = "latest"

# Minimum RAM in GB
min_ram = 2

# Maximum RAM in GB (will be auto-adjusted based on system RAM)
max_ram = 4

# Server arguments
server_args = ["nogui"]

# Working directory (optional, defaults to current directory)
# work_dir = "./server"
"#;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub minecraft_version: String,
    pub min_ram: u32,
    pub max_ram: u32,
    pub server_args: Vec<String>,
    pub work_dir: Option<String>,
    #[serde(skip)]
    pub auto_created: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            minecraft_version: DEFAULT_VERSION.to_string(),
            min_ram: DEFAULT_MIN_RAM,
            max_ram: DEFAULT_MAX_RAM,
            server_args: vec![DEFAULT_SERVER_ARG.to_string()],
            work_dir: None,
            auto_created: false,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        Self::load_from_path(Path::new(CONFIG_FILE))
    }

    pub fn load_from_path(config_path: &Path) -> Result<Self> {
        let was_created = !config_path.exists();
        let config = if !was_created {
            toml::from_str(&std::fs::read_to_string(config_path)
                .context("Failed to read config.toml")?)
                .context("Failed to parse config.toml")?
        } else {
            std::fs::write(config_path, CONFIG_EXAMPLE)
                .context("Failed to create config.toml")?;
            println!("Created config.toml with default settings. Edit it to customize.");
            Config {
                auto_created: true,
                ..Config::default()
            }
        };

        let mut config = config;
        Self::apply_env_overrides(&mut config);
        config.validate()?;
        Ok(config)
    }

    fn apply_env_overrides(config: &mut Self) {
        macro_rules! override_u32_field {
            ($env_var:expr, $field:ident) => {
                if let Ok(val) = std::env::var($env_var) {
                    if !val.is_empty() {
                        match val.parse::<u32>() {
                            Ok(parsed) => config.$field = parsed,
                            Err(_) => warn!("Failed to parse {} from environment variable {}. Using default value.", stringify!($field), $env_var),
                        }
                    }
                }
            };
        }

        if let Ok(version) = std::env::var(ENV_MINECRAFT_VERSION) {
            if !version.is_empty() {
                config.minecraft_version = version;
            }
        }
        override_u32_field!(ENV_MIN_RAM, min_ram);
        override_u32_field!(ENV_MAX_RAM, max_ram);
        if let Ok(work_dir) = std::env::var(ENV_WORK_DIR) {
            if !work_dir.is_empty() {
                config.work_dir = Some(work_dir);
            }
        }
    }

    pub fn reload(&mut self) -> Result<()> {
        let config_path = Path::new(CONFIG_FILE);
        if config_path.exists() {
            *self = Self::load_from_path(config_path)?;
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        if self.minecraft_version.is_empty() {
            anyhow::bail!("minecraft_version cannot be empty. Please specify a version or use 'latest'.");
        }
        
        if self.min_ram == 0 {
            anyhow::bail!("MIN_RAM must be greater than 0. Current value: {}", self.min_ram);
        }
        
        if self.min_ram > self.max_ram {
            anyhow::bail!(
                "MIN_RAM ({}) cannot be greater than MAX_RAM ({}). Please adjust your configuration.",
                self.min_ram, self.max_ram
            );
        }
        
        if self.max_ram > crate::constants::MAX_RAM_LIMIT {
            anyhow::bail!(
                "MAX_RAM ({}) exceeds the maximum allowed value ({}GB). This is likely a configuration error.",
                self.max_ram, crate::constants::MAX_RAM_LIMIT
            );
        }
        
        Ok(())
    }

    pub fn work_directory(&self) -> PathBuf {
        self.work_dir
            .as_deref()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(CURRENT_DIR))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.minecraft_version, "latest");
        assert_eq!(config.min_ram, 2);
        assert_eq!(config.max_ram, 4);
        assert_eq!(config.server_args, vec!["nogui"]);
        assert!(config.work_dir.is_none());
    }

    #[test]
    fn test_config_validate() {
        let mut config = Config::default();
        config.min_ram = 4;
        config.max_ram = 2;
        assert!(config.validate().is_err());

        config.min_ram = 0;
        config.max_ram = 4;
        assert!(config.validate().is_err());

        config.min_ram = 2;
        config.max_ram = 4;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_empty_version() {
        let mut config = Config::default();
        config.minecraft_version = String::new();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_max_ram_limit() {
        let mut config = Config::default();
        config.max_ram = crate::constants::MAX_RAM_LIMIT + 1;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_work_directory() {
        let config = Config::default();
        assert_eq!(config.work_directory(), PathBuf::from("."));

        let mut config = Config::default();
        config.work_dir = Some("./server".to_string());
        assert_eq!(config.work_directory(), PathBuf::from("./server"));
    }

    #[test]
    fn test_config_load_from_path() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        
        let config_content = r#"
minecraft_version = "1.21.1"
min_ram = 4
max_ram = 8
server_args = ["nogui", "test"]
"#;
        fs::write(&config_path, config_content).unwrap();

        let config = Config::load_from_path(&config_path).unwrap();
        assert_eq!(config.minecraft_version, "1.21.1");
        assert_eq!(config.min_ram, 4);
        assert_eq!(config.max_ram, 8);
        assert_eq!(config.server_args, vec!["nogui", "test"]);
    }


    #[test]
    fn test_config_load_invalid_toml() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        
        let invalid_content = r#"
minecraft_version = "1.21.1"
min_ram = invalid
max_ram = 8
"#;
        fs::write(&config_path, invalid_content).unwrap();

        let result = Config::load_from_path(&config_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("parse"));
    }

    #[test]
    fn test_config_load_partial_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        
        let partial_content = r#"
minecraft_version = "1.21.1"
min_ram = 2
max_ram = 4
server_args = []
"#;
        fs::write(&config_path, partial_content).unwrap();

        let result = Config::load_from_path(&config_path);
        assert!(result.is_ok(), "Failed to load config: {:?}", result.err());
        let config = result.unwrap();
        assert_eq!(config.minecraft_version, "1.21.1");
        assert_eq!(config.min_ram, 2);
        assert_eq!(config.max_ram, 4);
    }

    #[test]
    fn test_config_reload_updates_values() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(CONFIG_FILE);
        
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let initial_content = r#"
minecraft_version = "1.21.1"
min_ram = 2
max_ram = 4
server_args = ["nogui"]
"#;
        fs::write(&config_path, initial_content).unwrap();

        let mut config = Config::load().unwrap();
        assert_eq!(config.minecraft_version, "1.21.1");
        assert_eq!(config.max_ram, 4);

        let updated_content = r#"
minecraft_version = "1.20.1"
min_ram = 4
max_ram = 8
server_args = ["nogui", "test"]
"#;
        fs::write(&config_path, updated_content).unwrap();

        config.reload().unwrap();
        assert_eq!(config.minecraft_version, "1.20.1");
        assert_eq!(config.min_ram, 4);
        assert_eq!(config.max_ram, 8);
        assert_eq!(config.server_args, vec!["nogui", "test"]);
        
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_config_reload_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("nonexistent.toml");
        
        let mut config = Config::default();
        let result = Config::load_from_path(&config_path);
        assert!(result.is_ok());
        
        fs::remove_file(&config_path).ok();
        assert!(config.reload().is_ok());
    }

    #[test]
    fn test_config_env_override_invalid_number() {
        std::env::set_var(ENV_MIN_RAM, "invalid");
        std::env::set_var(ENV_MAX_RAM, "not_a_number");

        let mut config = Config::default();
        let original_min = config.min_ram;
        let original_max = config.max_ram;
        
        Config::apply_env_overrides(&mut config);

        assert_eq!(config.min_ram, original_min);
        assert_eq!(config.max_ram, original_max);

        std::env::remove_var(ENV_MIN_RAM);
        std::env::remove_var(ENV_MAX_RAM);
    }

    #[test]
    fn test_config_env_override_empty_string() {
        std::env::set_var(ENV_MINECRAFT_VERSION, "");
        std::env::set_var(ENV_MIN_RAM, "");

        let mut config = Config::default();
        let original_version = config.minecraft_version.clone();
        let original_min = config.min_ram;
        
        Config::apply_env_overrides(&mut config);

        assert_eq!(config.minecraft_version, original_version);
        assert_eq!(config.min_ram, original_min);

        std::env::remove_var(ENV_MINECRAFT_VERSION);
        std::env::remove_var(ENV_MIN_RAM);
    }

    #[test]
    fn test_config_validate_boundary_values() {
        let mut config = Config::default();
        config.min_ram = 1;
        config.max_ram = crate::constants::MAX_RAM_LIMIT;
        assert!(config.validate().is_ok());

        config.max_ram = crate::constants::MAX_RAM_LIMIT + 1;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_equal_min_max() {
        let mut config = Config::default();
        config.min_ram = 4;
        config.max_ram = 4;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_work_directory_edge_cases() {
        let mut config = Config::default();
        config.work_dir = Some(String::new());
        assert_eq!(config.work_directory(), PathBuf::from(""));

        config.work_dir = Some("/absolute/path".to_string());
        assert_eq!(config.work_directory(), PathBuf::from("/absolute/path"));

        config.work_dir = Some("../relative/path".to_string());
        assert_eq!(config.work_directory(), PathBuf::from("../relative/path"));
    }
}
