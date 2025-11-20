// 🎯 自定义预设系统
// 实现用户自定义质量预设管理

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::{Context, Result};

/// 自定义预设
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomPreset {
    pub name: String,
    pub quality_min: u8,
    pub quality_max: u8,
    pub speed_preference: u8,
    pub lossless_threshold: f64,
    pub format_preferences: Vec<String>,
    pub description: String,
}

impl CustomPreset {
    /// 平衡预设 - 压缩与质量平衡
    /// 
    /// 参数：quality=90, distance=0.5
    /// 格式：根据源文件智能推荐
    pub fn balanced() -> Self {
        Self {
            name: "balanced".to_string(),
            quality_min: 90,
            quality_max: 90,
            speed_preference: 4,
            lossless_threshold: 0.0,
            format_preferences: vec![], // 空列表表示使用智能推荐
            description: "Balanced: q=90, d=0.5, smart format selection".to_string(),
        }
    }
    
    /// 超高质量预设 - 最激进的无损压缩
    /// 
    /// 参数：quality=100, distance=0 (完全无损)
    /// 格式：根据源文件智能推荐
    pub fn ultra_high_quality() -> Self {
        Self {
            name: "ultra_high_quality".to_string(),
            quality_min: 100,
            quality_max: 100,
            speed_preference: 9,
            lossless_threshold: 1.0,
            format_preferences: vec![], // 空列表表示使用智能推荐
            description: "Ultra: q=100, d=0, lossless, smart format selection".to_string(),
        }
    }
    
    /// 验证预设参数
    pub fn validate(&self) -> Result<()> {
        if self.quality_min > self.quality_max {
            anyhow::bail!("Quality minimum cannot be greater than maximum");
        }
        
        if self.quality_min < 1 || self.quality_max > 100 {
            anyhow::bail!("Quality value must be between 1-100");
        }
        
        if self.speed_preference < 1 || self.speed_preference > 10 {
            anyhow::bail!("Speed preference must be between 1-10");
        }
        
        if self.lossless_threshold < 0.0 || self.lossless_threshold > 1.0 {
            anyhow::bail!("Lossless threshold must be between 0.0-1.0");
        }
        
        Ok(())
    }
}

/// 预设管理器
pub struct PresetManager {
    presets: HashMap<String, CustomPreset>,
    config_path: PathBuf,
}

impl PresetManager {
    /// 创建新的预设管理器
    pub fn new() -> Self {
        let mut manager = Self {
            presets: HashMap::new(),
            config_path: Self::default_config_path(),
        };
        
        // 加载内置预设
        manager.load_builtin_presets();
        
        // 尝试加载用户预设
        let _ = manager.load_user_presets();
        
        manager
    }
    
    /// 获取默认配置路径
    pub fn default_config_path() -> PathBuf {
        // 使用home目录代替dirs crate
        std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(".pixly")
            .join("presets.json")
    }
    
    /// 加载内置预设 - 只有两个核心预设
    fn load_builtin_presets(&mut self) {
        let builtins = vec![
            CustomPreset::balanced(),
            CustomPreset::ultra_high_quality(),
        ];
        
        for preset in builtins {
            self.presets.insert(preset.name.clone(), preset);
        }
    }
    
    /// 加载用户预设
    pub fn load_user_presets(&mut self) -> Result<()> {
        if !self.config_path.exists() {
            return Ok(());
        }
        
        let content = std::fs::read_to_string(&self.config_path)
            .context("Failed to read preset config file")?;
        
        let user_presets: HashMap<String, CustomPreset> = serde_json::from_str(&content)
            .context("Failed to parse preset config")?;
        
        for (name, preset) in user_presets {
            preset.validate()?;
            self.presets.insert(name, preset);
        }
        
        Ok(())
    }
    
    /// 保存用户预设
    pub fn save_user_presets(&self) -> Result<()> {
        // 创建配置目录
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }
        
        // 只保存非内置预设
        let builtin_names = [
            "photography_high", "web_optimized", "fast_processing",
            "archive_quality", "social_media"
        ];
        
        let user_presets: HashMap<String, CustomPreset> = self.presets
            .iter()
            .filter(|(name, _)| !builtin_names.contains(&name.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        
        let content = serde_json::to_string_pretty(&user_presets)
            .context("Failed to serialize presets")?;
        
        std::fs::write(&self.config_path, content)
            .context("Failed to write preset config file")?;
        
        Ok(())
    }
    
    /// 添加预设
    pub fn add_preset(&mut self, preset: CustomPreset) -> Result<()> {
        preset.validate()?;
        self.presets.insert(preset.name.clone(), preset);
        self.save_user_presets()?;
        Ok(())
    }
    
    /// 获取预设
    pub fn get_preset(&self, name: &str) -> Option<&CustomPreset> {
        self.presets.get(name)
    }
    
    /// 删除预设
    pub fn remove_preset(&mut self, name: &str) -> Result<()> {
        // 不允许删除内置预设
        let builtin_names = [
            "photography_high", "web_optimized", "fast_processing",
            "archive_quality", "social_media"
        ];
        
        if builtin_names.contains(&name) {
            anyhow::bail!("Cannot delete built-in preset");
        }
        
        self.presets.remove(name);
        self.save_user_presets()?;
        Ok(())
    }
    
    /// 列出所有预设
    pub fn list_presets(&self) -> Vec<&CustomPreset> {
        self.presets.values().collect()
    }
    
    /// 获取预设数量
    pub fn count(&self) -> usize {
        self.presets.len()
    }
}

impl Default for PresetManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_builtin_presets() {
        let manager = PresetManager::new();
        // 至少有2个核心预设: balanced 和 ultra_high_quality
        // 可能还有用户自定义预设
        assert!(manager.count() >= 2);
        
        let balanced = manager.get_preset("balanced").unwrap();
        assert_eq!(balanced.quality_min, 90);
        assert_eq!(balanced.quality_max, 90);
        
        let ultra = manager.get_preset("ultra_high_quality").unwrap();
        assert_eq!(ultra.quality_min, 100);
        assert_eq!(ultra.quality_max, 100);
    }
    
    #[test]
    fn test_preset_validation() {
        let mut preset = CustomPreset::balanced();
        assert!(preset.validate().is_ok());
        
        preset.quality_min = 110;
        assert!(preset.validate().is_err());
    }
    
    #[test]
    fn test_add_custom_preset() {
        let mut manager = PresetManager::new();
        
        let custom = CustomPreset {
            name: "test_preset".to_string(),
            quality_min: 80,
            quality_max: 90,
            speed_preference: 5,
            lossless_threshold: 0.2,
            format_preferences: vec!["webp".to_string()],
            description: "测试预设".to_string(),
        };
        
        assert!(manager.add_preset(custom).is_ok());
        assert!(manager.get_preset("test_preset").is_some());
    }
}
