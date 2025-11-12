// 🧠 PIXLY v3.0 Rust-Python桥接器配置系统
//
// 为插件UI端提供灵活的功能开关：
// - 模块化功能启用/禁用
// - 性能级别调节
// - 缓存策略配置
// - 调试模式控制

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

/// 功能开关配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    /// SIMD加速特征提取
    pub enable_simd_features: bool,
    
    /// 零拷贝内存共享
    pub enable_zero_copy: bool,
    
    /// 预测结果缓存
    pub enable_prediction_cache: bool,
    
    /// 批量预测优化
    pub enable_batch_processing: bool,
    
    /// GPU加速 (如果可用)
    pub enable_gpu_acceleration: bool,
    
    /// 高级特征提取 (SWT, 纹理分析等)
    pub enable_advanced_features: bool,
    
    /// 实时性能监控
    pub enable_performance_monitoring: bool,
    
    /// 调试模式 (详细日志)
    pub enable_debug_mode: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            enable_simd_features: true,
            enable_zero_copy: true,
            enable_prediction_cache: true,
            enable_batch_processing: true,
            enable_gpu_acceleration: false, // 默认关闭，需要显式启用
            enable_advanced_features: true,
            enable_performance_monitoring: true,
            enable_debug_mode: false,
        }
    }
}

/// 性能配置级别
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PerformanceLevel {
    /// 节能模式 (最小CPU使用)
    PowerSaver,
    /// 平衡模式 (默认)
    Balanced,
    /// 高性能模式 (最大速度)
    HighPerformance,
    /// 极限模式 (所有优化)
    Maximum,
}

impl Default for PerformanceLevel {
    fn default() -> Self {
        Self::Balanced
    }
}

/// 缓存策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 最大缓存条目数
    pub max_entries: usize,
    
    /// 缓存TTL (秒)
    pub ttl_seconds: u64,
    
    /// 是否启用LRU淘汰
    pub enable_lru_eviction: bool,
    
    /// 内存限制 (MB)
    pub memory_limit_mb: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 1000,
            ttl_seconds: 3600, // 1小时
            enable_lru_eviction: true,
            memory_limit_mb: 128,
        }
    }
}

/// 特征提取配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureExtractionConfig {
    /// 启用的特征类型
    pub enabled_features: Vec<String>,
    
    /// SIMD向量宽度 (8, 16, 32)
    pub simd_width: usize,
    
    /// 边缘检测阈值
    pub edge_threshold: f32,
    
    /// 纹理分析窗口大小
    pub texture_window_size: u32,
    
    /// 频域分析层数
    pub frequency_levels: u32,
}

impl Default for FeatureExtractionConfig {
    fn default() -> Self {
        Self {
            enabled_features: vec![
                "color_complexity".to_string(),
                "brightness".to_string(),
                "contrast".to_string(),
                "edge_density".to_string(),
                "texture_score".to_string(),
                "swt_energy".to_string(),
            ],
            simd_width: 8,
            edge_threshold: 10.0,
            texture_window_size: 7,
            frequency_levels: 3,
        }
    }
}

/// 主配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    /// 功能开关
    pub features: FeatureFlags,
    
    /// 性能级别
    pub performance_level: PerformanceLevel,
    
    /// 缓存配置
    pub cache: CacheConfig,
    
    /// 特征提取配置
    pub feature_extraction: FeatureExtractionConfig,
    
    /// Python模块路径
    pub python_module_path: String,
    
    /// 模型目录
    pub models_directory: String,
    
    /// 自定义参数
    pub custom_params: HashMap<String, serde_json::Value>,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            features: FeatureFlags::default(),
            performance_level: PerformanceLevel::default(),
            cache: CacheConfig::default(),
            feature_extraction: FeatureExtractionConfig::default(),
            python_module_path: "core.python.ai".to_string(),
            models_directory: "models".to_string(),
            custom_params: HashMap::new(),
        }
    }
}

impl BridgeConfig {
    /// 从JSON文件加载配置
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }
    
    /// 保存配置到JSON文件
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// 从环境变量加载配置
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        // 功能开关
        if let Ok(val) = std::env::var("PIXLY_ENABLE_SIMD") {
            config.features.enable_simd_features = val.parse().unwrap_or(true);
        }
        
        if let Ok(val) = std::env::var("PIXLY_ENABLE_GPU") {
            config.features.enable_gpu_acceleration = val.parse().unwrap_or(false);
        }
        
        if let Ok(val) = std::env::var("PIXLY_DEBUG_MODE") {
            config.features.enable_debug_mode = val.parse().unwrap_or(false);
        }
        
        // 性能级别
        if let Ok(val) = std::env::var("PIXLY_PERFORMANCE_LEVEL") {
            config.performance_level = match val.to_lowercase().as_str() {
                "powersaver" => PerformanceLevel::PowerSaver,
                "balanced" => PerformanceLevel::Balanced,
                "high" => PerformanceLevel::HighPerformance,
                "maximum" => PerformanceLevel::Maximum,
                _ => PerformanceLevel::Balanced,
            };
        }
        
        // 模型目录
        if let Ok(val) = std::env::var("PIXLY_MODELS_DIR") {
            config.models_directory = val;
        }
        
        config
    }
    
    /// 应用性能级别预设
    pub fn apply_performance_preset(&mut self) {
        match self.performance_level {
            PerformanceLevel::PowerSaver => {
                self.features.enable_simd_features = false;
                self.features.enable_gpu_acceleration = false;
                self.features.enable_batch_processing = false;
                self.features.enable_advanced_features = false;
                self.cache.max_entries = 100;
                self.feature_extraction.simd_width = 4;
            },
            PerformanceLevel::Balanced => {
                self.features.enable_simd_features = true;
                self.features.enable_gpu_acceleration = false;
                self.features.enable_batch_processing = true;
                self.features.enable_advanced_features = true;
                self.cache.max_entries = 500;
                self.feature_extraction.simd_width = 8;
            },
            PerformanceLevel::HighPerformance => {
                self.features.enable_simd_features = true;
                self.features.enable_gpu_acceleration = true;
                self.features.enable_batch_processing = true;
                self.features.enable_advanced_features = true;
                self.cache.max_entries = 1000;
                self.feature_extraction.simd_width = 16;
            },
            PerformanceLevel::Maximum => {
                self.features.enable_simd_features = true;
                self.features.enable_gpu_acceleration = true;
                self.features.enable_batch_processing = true;
                self.features.enable_advanced_features = true;
                self.features.enable_performance_monitoring = true;
                self.cache.max_entries = 2000;
                self.feature_extraction.simd_width = 32;
            },
        }
    }
    
    /// 获取UI友好的配置摘要
    pub fn get_ui_summary(&self) -> HashMap<String, serde_json::Value> {
        let mut summary = HashMap::new();
        
        summary.insert("performance_level".to_string(), 
                      serde_json::json!(format!("{:?}", self.performance_level)));
        
        summary.insert("simd_enabled".to_string(), 
                      serde_json::json!(self.features.enable_simd_features));
        
        summary.insert("gpu_enabled".to_string(), 
                      serde_json::json!(self.features.enable_gpu_acceleration));
        
        summary.insert("cache_size".to_string(), 
                      serde_json::json!(self.cache.max_entries));
        
        summary.insert("features_count".to_string(), 
                      serde_json::json!(self.feature_extraction.enabled_features.len()));
        
        summary.insert("debug_mode".to_string(), 
                      serde_json::json!(self.features.enable_debug_mode));
        
        summary
    }
    
    /// 验证配置合法性
    pub fn validate(&self) -> Result<()> {
        // SIMD宽度检查
        if ![4, 8, 16, 32].contains(&self.feature_extraction.simd_width) {
            return Err(anyhow::anyhow!("无效的SIMD宽度: {}", self.feature_extraction.simd_width));
        }
        
        // 缓存大小检查
        if self.cache.max_entries == 0 {
            return Err(anyhow::anyhow!("缓存大小不能为0"));
        }
        
        // 模型目录检查
        if !std::path::Path::new(&self.models_directory).exists() {
            println!("⚠️ 模型目录不存在: {}", self.models_directory);
        }
        
        Ok(())
    }
}

/// 配置构建器 (便于UI调用)
pub struct ConfigBuilder {
    config: BridgeConfig,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: BridgeConfig::default(),
        }
    }
    
    /// 设置性能级别
    pub fn performance_level(mut self, level: PerformanceLevel) -> Self {
        self.config.performance_level = level;
        self.config.apply_performance_preset();
        self
    }
    
    /// 启用/禁用SIMD
    pub fn simd(mut self, enabled: bool) -> Self {
        self.config.features.enable_simd_features = enabled;
        self
    }
    
    /// 启用/禁用GPU
    pub fn gpu(mut self, enabled: bool) -> Self {
        self.config.features.enable_gpu_acceleration = enabled;
        self
    }
    
    /// 设置缓存大小
    pub fn cache_size(mut self, size: usize) -> Self {
        self.config.cache.max_entries = size;
        self
    }
    
    /// 设置调试模式
    pub fn debug(mut self, enabled: bool) -> Self {
        self.config.features.enable_debug_mode = enabled;
        self
    }
    
    /// 设置模型目录
    pub fn models_dir(mut self, dir: &str) -> Self {
        self.config.models_directory = dir.to_string();
        self
    }
    
    /// 添加自定义参数
    pub fn custom_param<T: serde::Serialize>(mut self, key: &str, value: T) -> Self {
        self.config.custom_params.insert(key.to_string(), serde_json::to_value(value).unwrap());
        self
    }
    
    /// 构建最终配置
    pub fn build(self) -> Result<BridgeConfig> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
