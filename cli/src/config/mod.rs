//! Configuration management

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;
use dirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct CliConfig {
    pub server: ServerConfig,
    pub ui: UiConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub url: String,
    pub api_key: Option<String>,
    pub timeout: u64,
    pub retry_attempts: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: Theme,
    pub refresh_interval: u64,
    pub show_progress: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    pub default_format: crate::OutputFormat,
    pub table_style: TableStyle,
    pub colors: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Theme {
    Default,
    Dark,
    Light,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TableStyle {
    Default,
    Compact,
    Markdown,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                url: "http://localhost:16080".to_string(),
                api_key: None,
                timeout: 30,
                retry_attempts: 3,
            },
            ui: UiConfig {
                theme: Theme::Default,
                refresh_interval: 1,
                show_progress: true,
            },
            output: OutputConfig {
                default_format: crate::OutputFormat::Table,
                table_style: TableStyle::Default,
                colors: true,
            },
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new(custom_path: Option<PathBuf>) -> Result<Self> {
        let config_path = if let Some(path) = custom_path {
            path
        } else {
            Self::get_default_config_path()?
        };
        
        Ok(Self { config_path })
    }
    
    fn get_default_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        
        Ok(config_dir.join("task-queue").join("config.yaml"))
    }
    
    pub fn load_config(&self) -> Result<CliConfig> {
        if self.config_path.exists() {
            let content = std::fs::read_to_string(&self.config_path)?;
            let config: CliConfig = serde_yaml::from_str(&content)?;
            Ok(config)
        } else {
            let config = CliConfig::default();
            self.save_config(&config)?;
            Ok(config)
        }
    }
    
    pub fn save_config(&self, config: &CliConfig) -> Result<()> {
        // Create directory if it doesn't exist
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = serde_yaml::to_string(config)?;
        std::fs::write(&self.config_path, content)?;
        
        Ok(())
    }
    
    pub fn reset_config(&self) -> Result<()> {
        let config = CliConfig::default();
        self.save_config(&config)
    }
}