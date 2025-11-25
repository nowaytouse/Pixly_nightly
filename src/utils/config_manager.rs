/// 🔥 configuration Manager - Phase 4
/// 
/// 架构原则：
/// - supportconfigurationfile（TOMLformat）
/// - supportpresetmode
/// - commandlineparameterpriority级highest
/// - environmentvariable次之
/// - configurationfile最 after 
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Configuration file structureure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub structure Config {
 /// Default settings
 #[serde(default)]
 pub defaults: DefaultSettings,
 
 /// Logging settings
 #[serde(default)]
 pub logging: LoggingSettings,
 
 /// Batch processing settings
 #[serde(default)]
 pub batch: BatchSettings,
}

/// defaultsetting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure DefaultSettings {
 /// defaultquality
 #[serde(default = "default_quality")]
 pub quality: u8,
 
 /// defaultspeed
 #[serde(default = "default_speed")]
 pub speed: u8,
 
 /// defaultformat
 #[serde(default = "default_format")]
 pub format: String,
 
 /// 保留元data
 #[serde(default = "default_true")]
 pub preserve_metadata: bool,
 
 /// keepanimation
 #[serde(default = "default_true")]
 pub keep_animated: bool,
 
 /// merged XMP
 #[serde(default)]
 pub merge_xmp: bool,
}
impl Default for DefaultSettings {
 fn default() -> Self {
 Self {
 quality: 85,
 speed: 4,
 format: "webp".to_string(),
 preserve_metadata: true,
 keep_animated: true,
 merge_xmp: false,
 }
 }
}



/// loggingsetting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure LoggingSettings {
 /// logginglevel
 #[serde(default = "default_log_level")]
 pub level: String,
 
 /// displaytime戳
 #[serde(default)]
 pub show_timestamp: bool,
 
 /// displaycolor
 #[serde(default = "default_true")]
 pub show_color: bool,
}
impl Default for LoggingSettings {
 fn default() -> Self {
 Self {
 level: "production".to_string(),
 show_timestamp: false,
 show_color: true,
 }
 }
}

/// batchprocessingsetting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure BatchSettings {
 /// defaultparalleltask数
 #[serde(default = "default_parallel")]
 pub parallel: usize,
 
 /// errorprocessingstrategy
 #[serde(default = "default_error_strategy")]
 pub on_error: String,
 
 /// Maximum retry count
 #[serde(default = "default_max_retries")]
 pub max_retries: usize,
 
 /// displayprogress
 #[serde(default = "default_true")]
 pub show_progress: bool,
 
 /// defaultoverride
 #[serde(default)]
 pub overwrite: bool,
}
impl Default for BatchSettings {
 fn default() -> Self {
 Self {
 parallel: num_cpus::get(),
 on_error: "continue".to_string(),
 max_retries: 3,
 show_progress: true,
 overwrite: false,
 }
 }
}

// defaultvaluefunction
fn default_quality() -> u8 { 85 }
fn default_speed() -> u8 { 4 }
fn default_format() -> String { "webp".to_string() }
fn default_true() -> bool { true }
fn default_log_level() -> String { "production".to_string() }
fn default_parallel() -> usize { num_cpus::get() }
fn default_error_strategy() -> String { "continue".to_string() }
fn default_max_retries() -> usize { 3 }

/// configuration Manager
pub structure ConfigManager {
 config: Config,
 #[allow(dead_code)]
 config_path: Option<PathBuf>,
}
impl ConfigManager {
 /// createnewconfiguration Manager
 pub fn new() -> Self {
 Self {
 config: Config::default(),
 config_path: None,
 }
 }
 
 /// fromfileloadconfiguration
 pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
 let path = path.as_ref();
 let content = std::fs::read_to_string(path)
 .with_context(|| format!("Failed to read config file: {}", path.display()))?;
 
 let config: Config = toml::from_str(&content)
 .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
 
 Ok(Self {
 config,
 config_path: Some(path.to_path_buf()),
 })
 }
 
 /// tryfromdefaultpositionloadconfiguration
 pub fn load_default() -> Self {
 // tryloadsequential：
 // 1. currentdirectory .pixly.toml
 // 2. 用户主directory ~/.pixly/config.toml
 // 3. usedefaultconfiguration
 
 if let Ok(config) = Self::load_from_file(".pixly.toml") {
 return config;
 }
 
 if let Some(home) = dirs::home_dir() {
 let config_path = home.join(".pixly").join("config.toml");
 if let Ok(config) = Self::load_from_file(&config_path) {
 return config;
 }
 }
 
 Self::new()
 }
 
 /// saveconfigurationtofile
 pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
 let path = path.as_ref();
 
 // create父directory
 if let Some(parent) = path.parent() {
 std::fs::create_dir_all(parent)?;
 }
 
 let content = toml::to_string_pretty(&self.config)
 .context("Failed to serialize config")?;
 
 std::fs::write(path, content)
 .with_context(|| format!("Failed to write config file: {}", path.display()))?;
 
 Ok(())
 }
 
 /// getconfiguration
 pub fn config(&self) -> &Config {
 &self.config
 }
 
 /// Get mutable configuration
 pub fn config_mut(&mut self) -> &mut Config {
 &mut self.config
 }
 
 /// Create default configuration file
 pub fn create_default_config<P: AsRef<Path>>(path: P) -> Result<()> {
 let config = Config::default();
 let content = toml::to_string_pretty(&config)
 .context("Failed to serialize default config")?;
 
 let path = path.as_ref();
 if let Some(parent) = path.parent() {
 std::fs::create_dir_all(parent)?;
 }
 
 std::fs::write(path, content)
 .with_context(|| format!("Failed to write config file: {}", path.display()))?;
 
 Ok(())
 }
}

impl Default for ConfigManager {
 fn default() -> Self {
 Self::new()
 }
}

#[cfg(test)]
mod tests {
 use super::*;
 use tempfile::TempDir;
 
 #[test]
 fn test_default_config() {
 let config = Config::default();
 assert_eq!(config.defaults.quality, 85);
 assert_eq!(config.defaults.speed, 4);
 assert_eq!(config.defaults.format, "webp");
 }
 
 #[test]
 fn test_save_and_load() {
 let temp_dir = TempDir::new().unwrap();
 let config_path = temp_dir.path().join("test.toml");
 
 // savedefaultconfiguration
 let manager = ConfigManager::new();
 manager.save_to_file(&config_path).unwrap();
 
 // loadconfiguration
 let loaded = ConfigManager::load_from_file(&config_path).unwrap();
 assert_eq!(loaded.config.defaults.quality, 85);
 }
 

}
