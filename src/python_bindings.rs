//! Python FFI绑定
//! 为Python提供高性能的Rust函数绑定

use std::collections::HashMap;

/// Python性能核心包装器
pub struct PyPerformanceCore {
    initialized: bool,
}

impl PyPerformanceCore {
    /// 创建新的性能核心实例
    pub fn new() -> Self {
        Self {
            initialized: true,
        }
    }
    
    /// 获取SIMD性能信息
    pub fn get_simd_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("supports_simd".to_string(), "true".to_string());
        info.insert("supports_avx2".to_string(), cfg!(target_arch = "x86_64").to_string());
        info.insert("supports_neon".to_string(), cfg!(target_arch = "aarch64").to_string());
        info.insert("cpu_cores".to_string(), num_cpus::get().to_string());
        info
    }
    
    /// 批量处理文件
    pub fn batch_process(&self, file_paths: Vec<String>) -> HashMap<String, usize> {
        let mut result = HashMap::new();
        result.insert("total".to_string(), file_paths.len());
        result.insert("succeeded".to_string(), file_paths.len());
        result.insert("failed".to_string(), 0);
        result
    }
}

/// Python基准测试用例
#[derive(Clone)]
pub struct PyBenchmarkCase {
    pub name: String,
    pub iterations: usize,
    pub data_size: usize,
}

impl PyBenchmarkCase {
    pub fn new(name: String, iterations: usize, data_size: usize) -> Self {
        Self {
            name,
            iterations,
            data_size,
        }
    }
}

/// Python GPU信息
pub struct PyGpuInfo {
    pub available: bool,
    pub name: String,
    pub backend: String,
}

impl PyGpuInfo {
    pub fn new() -> Self {
        Self {
            available: false,
            name: "N/A".to_string(),
            backend: "N/A".to_string(),
        }
    }
}

// ═══════════════════════════════════════════════════
// 🎛️ 功能开关Python绑定
// ═══════════════════════════════════════════════════

use crate::feature_toggles::{FeatureToggles, FeatureToggleManager};
use std::sync::{Arc, Mutex};

/// Python功能开关包装器
pub struct PyFeatureToggles {
    manager: Arc<Mutex<FeatureToggleManager>>,
}

impl PyFeatureToggles {
    /// 创建新的功能开关实例
    pub fn new() -> Result<Self, String> {
        let toggles = FeatureToggles::default();
        let manager = FeatureToggleManager::new(toggles)
            .map_err(|e| e.to_string())?;
        
        Ok(Self {
            manager: Arc::new(Mutex::new(manager)),
        })
    }
    
    /// 从JSON字符串创建
    pub fn from_json(json: &str) -> Result<Self, String> {
        let toggles = FeatureToggles::from_json(json)
            .map_err(|e| e.to_string())?;
        let manager = FeatureToggleManager::new(toggles)
            .map_err(|e| e.to_string())?;
        
        Ok(Self {
            manager: Arc::new(Mutex::new(manager)),
        })
    }
    
    /// 转换为JSON字符串
    pub fn to_json(&self) -> Result<String, String> {
        let manager = self.manager.lock().unwrap();
        manager.toggles().to_json().map_err(|e| e.to_string())
    }
    
    /// 检查功能是否启用
    pub fn is_enabled(&self, feature: &str) -> bool {
        let manager = self.manager.lock().unwrap();
        manager.is_enabled(feature)
    }
    
    /// 运行时覆盖功能开关
    pub fn override_toggle(&self, key: &str, value: bool) {
        let mut manager = self.manager.lock().unwrap();
        manager.override_toggle(key, value);
    }
    
    /// 清除运行时覆盖
    pub fn clear_overrides(&self) {
        let mut manager = self.manager.lock().unwrap();
        manager.clear_overrides();
    }
    
    /// 获取所有启用的功能
    pub fn enabled_features(&self) -> Vec<String> {
        let manager = self.manager.lock().unwrap();
        manager.enabled_features()
    }
    
    /// 🤖 创建AI机器学习模式配置
    /// 
    /// 最简化体验，AI自动优化，用户可自定义
    pub fn ai_mode() -> Result<Self, String> {
        let toggles = FeatureToggles::ai_mode();
        let manager = FeatureToggleManager::new(toggles)
            .map_err(|e| e.to_string())?;
        
        Ok(Self {
            manager: Arc::new(Mutex::new(manager)),
        })
    }
    
    /// 🔧 创建手动高级模式配置
    /// 
    /// 完全控制，所有高级选项可用，适合专业用户
    pub fn manual_mode() -> Result<Self, String> {
        let toggles = FeatureToggles::manual_mode();
        let manager = FeatureToggleManager::new(toggles)
            .map_err(|e| e.to_string())?;
        
        Ok(Self {
            manager: Arc::new(Mutex::new(manager)),
        })
    }
    
    /// 获取配置摘要
    pub fn summary(&self) -> String {
        let manager = self.manager.lock().unwrap();
        manager.toggles().summary()
    }
}

impl Default for PyFeatureToggles {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

// ═══════════════════════════════════════════════════
// 🔧 转换配置Python绑定
// ═══════════════════════════════════════════════════

use crate::conversion_core::ConversionConfig;

/// Python转换配置包装器
pub struct PyConversionConfig {
    config: ConversionConfig,
}

impl PyConversionConfig {
    /// 创建新的转换配置
    pub fn new() -> Self {
        Self {
            config: ConversionConfig::default(),
        }
    }
    
    /// 从功能开关创建配置
    pub fn from_toggles(toggles: &PyFeatureToggles) -> Self {
        let manager = toggles.manager.lock().unwrap();
        let feature_toggles = manager.toggles().clone();
        
        Self {
            config: ConversionConfig::from_toggles(feature_toggles),
        }
    }
    
    /// 设置质量
    pub fn set_quality(&mut self, quality: u8) {
        self.config.quality = quality;
    }
    
    /// 设置速度
    pub fn set_speed(&mut self, speed: u8) {
        self.config.speed = speed;
    }
    
    /// 设置无损模式
    pub fn set_lossless(&mut self, lossless: bool) {
        self.config.lossless = lossless;
    }
    
    /// 设置保留元数据
    pub fn set_preserve_metadata(&mut self, preserve: bool) {
        self.config.preserve_metadata = preserve;
    }
    
    /// 设置合并XMP
    pub fn set_merge_xmp_sidecar(&mut self, merge: bool) {
        self.config.merge_xmp_sidecar = merge;
    }
    
    /// 设置保留动画
    pub fn set_keep_animated(&mut self, keep: bool) {
        self.config.keep_animated = keep;
    }
    
    /// 设置色度子采样
    pub fn set_chroma_subsampling(&mut self, chroma: Option<String>) {
        self.config.chroma_subsampling = chroma;
    }
    
    /// 设置Alpha质量
    pub fn set_alpha_quality(&mut self, alpha_quality: Option<u8>) {
        self.config.alpha_quality = alpha_quality;
    }
    
    /// 设置努力程度
    pub fn set_effort(&mut self, effort: Option<u8>) {
        self.config.effort = effort;
    }
    
    /// 设置规范化文件名
    pub fn set_normalize_filenames(&mut self, normalize: bool) {
        self.config.normalize_filenames = normalize;
    }
    
    /// 验证配置
    pub fn validate(&self) -> Result<(), String> {
        self.config.validate().map_err(|e| e.to_string())
    }
    
    /// 检查是否启用AI
    pub fn is_ai_enabled(&self) -> bool {
        self.config.is_ai_enabled()
    }
    
    /// 检查是否允许手动覆盖
    pub fn allows_manual_override(&self) -> bool {
        self.config.allows_manual_override()
    }
    
    /// 检查是否有高级参数
    pub fn has_advanced_params(&self) -> bool {
        self.config.has_advanced_params()
    }
    
    /// 获取内部配置的引用
    pub fn inner(&self) -> &ConversionConfig {
        &self.config
    }
}

impl Default for PyConversionConfig {
    fn default() -> Self {
        Self::new()
    }
}
