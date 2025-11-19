/// 🔥 配置管理器 - Phase 4
/// 
/// 架构原则：
/// - 支持配置文件（TOML格式）
/// - 支持预设模式
/// - 命令行参数优先级最高
/// - 环境变量次之
/// - 配置文件最后

use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
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

impl Default for Config {
    fn default() -> Self {
        Self {
            defaults: DefaultSettings::default(),
            logging: LoggingSettings::default(),
            batch: BatchSettings::default(),
        }
    }
}

/// 默认设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultSettings {
    /// 默认质量
    #[serde(default = "default_quality")]
    pub quality: u8,
    
    /// 默认速度
    #[serde(default = "default_speed")]
    pub speed: u8,
    
    /// 默认格式
    #[serde(default = "default_format")]
    pub format: String,
    
    /// 保留元数据
    #[serde(default = "default_true")]
    pub preserve_metadata: bool,
    
    /// 保持动画
    #[serde(default = "default_true")]
    pub keep_animated: bool,
    
    /// 合并 XMP
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



/// 日志设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    /// 日志级别
    #[serde(default = "default_log_level")]
    pub level: String,
    
    /// 显示时间戳
    #[serde(default)]
    pub show_timestamp: bool,
    
    /// 显示颜色
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

/// 批量处理设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSettings {
    /// 默认并行任务数
    #[serde(default = "default_parallel")]
    pub parallel: usize,
    
    /// 错误处理策略
    #[serde(default = "default_error_strategy")]
    pub on_error: String,
    
    /// 最大重试次数
    #[serde(default = "default_max_retries")]
    pub max_retries: usize,
    
    /// 显示进度
    #[serde(default = "default_true")]
    pub show_progress: bool,
    
    /// 默认覆盖
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

// 默认值函数
fn default_quality() -> u8 { 85 }
fn default_speed() -> u8 { 4 }
fn default_format() -> String { "webp".to_string() }
fn default_true() -> bool { true }
fn default_log_level() -> String { "production".to_string() }
fn default_parallel() -> usize { num_cpus::get() }
fn default_error_strategy() -> String { "continue".to_string() }
fn default_max_retries() -> usize { 3 }

/// 配置管理器
pub struct ConfigManager {
    config: Config,
    #[allow(dead_code)]
    config_path: Option<PathBuf>,
}

impl ConfigManager {
    /// 创建新的配置管理器
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            config_path: None,
        }
    }
    
    /// 从文件加载配置
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
    
    /// 尝试从默认位置加载配置
    pub fn load_default() -> Self {
        // 尝试加载顺序：
        // 1. 当前目录的 .pixly.toml
        // 2. 用户主目录的 ~/.pixly/config.toml
        // 3. 使用默认配置
        
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
    
    /// 保存配置到文件
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        
        // 创建父目录
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(&self.config)
            .context("Failed to serialize config")?;
        
        std::fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;
        
        Ok(())
    }
    
    /// 获取配置
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
        
        // 保存默认配置
        let manager = ConfigManager::new();
        manager.save_to_file(&config_path).unwrap();
        
        // 加载配置
        let loaded = ConfigManager::load_from_file(&config_path).unwrap();
        assert_eq!(loaded.config.defaults.quality, 85);
    }
    

}
