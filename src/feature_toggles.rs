/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * Kernel功能开关系统 (Kernel Feature Toggles)
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 
 * 🎯 核心目标:
 * - ✅ Rust Kernel层面的功能开关控制
 * - ✅ 支持AI预测和手动参数两种模式
 * - ✅ 完整的参数验证和标准化
 * - ✅ 与Eagle插件完全兼容
 * 
 * 🔧 功能开关:
 * - AI预测开关 (enable_ai_prediction)
 * - 验证系统开关 (enable_validation)
 * - 元数据保留开关 (preserve_metadata)
 * - XMP合并开关 (merge_xmp_sidecar)
 * - 动画保留开关 (keep_animated)
 * - 文件名规范化开关 (normalize_filenames)
 * - SIMD加速开关 (enable_simd)
 * - 缓存系统开关 (enable_cache)
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

/// Kernel功能开关配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureToggles {
    // ═══════════════════════════════════════════════════
    // 🤖 AI功能开关 (对应HTML AI标签页)
    // ═══════════════════════════════════════════════════
    
    /// 启用AI参数预测 (对应HTML: enableAI)
    pub enable_ai_prediction: bool,
    
    /// 智能质量预测 (对应HTML: enableSmartQuality)
    pub enable_smart_quality: bool,
    
    /// 自动参数优化/贝叶斯优化 (对应HTML: enableAutoOptimize)
    pub enable_auto_optimize: bool,
    
    /// SSIM质量验证 (对应HTML: enableSSIMValidation)
    pub enable_ssim_validation: bool,
    
    /// 动画转视频推荐 (对应HTML: enableVideoForAnimation)
    pub enable_video_for_animation: bool,
    
    /// PPO强化学习 (对应HTML: enablePPO, 隐藏checkbox)
    pub enable_ppo: bool,
    
    /// 自定义期望格式 (对应HTML: expectedFormatSelect)
    pub expected_format: Option<String>,
    
    /// AI预测超时时间(秒)
    pub ai_prediction_timeout_secs: u64,
    
    /// AI预测最小置信度阈值 (0.0-1.0)
    pub ai_min_confidence: f32,
    
    /// AI预测失败时使用默认参数
    pub ai_fallback_to_defaults: bool,
    
    // ═══════════════════════════════════════════════════
    // ✅ 验证系统开关
    // ═══════════════════════════════════════════════════
    
    /// 启用参数验证
    pub enable_validation: bool,
    
    /// 启用输入文件验证
    pub enable_file_validation: bool,
    
    /// 启用输出质量验证
    pub enable_quality_validation: bool,
    
    /// 验证失败时是否中止转换
    pub abort_on_validation_failure: bool,
    
    // ═══════════════════════════════════════════════════
    // 📄 元数据和文件处理开关
    // ═══════════════════════════════════════════════════
    
    /// 保留元数据
    pub preserve_metadata: bool,
    
    /// 合并XMP sidecar文件
    pub merge_xmp_sidecar: bool,
    
    /// 保留动画帧
    pub keep_animated: bool,
    
    /// 规范化输出文件名
    pub normalize_filenames: bool,
    
    // ═══════════════════════════════════════════════════
    // ⚡ 性能优化开关
    // ═══════════════════════════════════════════════════
    
    /// 启用SIMD加速
    pub enable_simd: bool,
    
    /// 启用缓存系统
    pub enable_cache: bool,
    
    /// 启用并行处理
    pub enable_parallel: bool,
    
    /// 最大并发转换数
    pub max_concurrent_conversions: usize,
    
    // ═══════════════════════════════════════════════════
    // 🔧 高级参数开关
    // ═══════════════════════════════════════════════════
    
    /// 允许手动覆盖AI预测参数
    pub allow_manual_override: bool,
    
    /// 启用高级参数 (chroma_subsampling, alpha_quality等)
    pub enable_advanced_params: bool,
    
    /// 启用实验性功能
    pub enable_experimental: bool,
}

impl Default for FeatureToggles {
    fn default() -> Self {
        Self {
            // AI功能 - 默认启用 (对应HTML AI标签页)
            enable_ai_prediction: true,
            enable_smart_quality: true,
            enable_auto_optimize: true,
            enable_ssim_validation: true,
            enable_video_for_animation: false,
            enable_ppo: true,
            expected_format: None,
            ai_prediction_timeout_secs: 30,
            ai_min_confidence: 0.7,
            ai_fallback_to_defaults: true,
            
            // 验证系统 - 默认启用 (对应HTML: enableValidation)
            enable_validation: true,
            // 文件验证 - 默认禁用 (对应HTML: enableFileValidation)
            enable_file_validation: false,
            enable_quality_validation: true,
            abort_on_validation_failure: false,
            
            // 元数据处理 - 默认保留元数据 (对应HTML: preserveMetadata)
            preserve_metadata: true,
            merge_xmp_sidecar: false,
            // 保留动画 - 默认启用 (对应HTML: keepAnimated)
            keep_animated: true,
            // 规范化文件名 - 默认禁用 (对应HTML: normalizeFilenames)
            normalize_filenames: false,
            
            // 性能优化 - 默认启用
            enable_simd: true,
            enable_cache: true,
            enable_parallel: true,
            max_concurrent_conversions: 4,
            
            // 高级参数 - 默认允许
            allow_manual_override: true,
            enable_advanced_params: true,
            enable_experimental: false,
        }
    }
}

impl FeatureToggles {
    /// 创建新的功能开关配置
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 从JSON字符串加载配置
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
    
    /// 转换为JSON字符串
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    
    /// 从HashMap加载配置
    pub fn from_map(map: HashMap<String, serde_json::Value>) -> Result<Self> {
        let json = serde_json::to_string(&map)?;
        Self::from_json(&json)
    }
    
    /// 验证配置有效性
    pub fn validate(&self) -> Result<()> {
        // 验证AI置信度阈值
        if self.ai_min_confidence < 0.0 || self.ai_min_confidence > 1.0 {
            anyhow::bail!("AI confidence threshold must be between 0.0-1.0");
        }
        
        // 验证超时时间
        if self.ai_prediction_timeout_secs == 0 {
            anyhow::bail!("AI prediction timeout must be greater than 0");
        }
        
        // 验证并发数
        if self.max_concurrent_conversions == 0 {
            anyhow::bail!("Maximum concurrent conversions must be greater than 0");
        }
        
        Ok(())
    }
    
    /// 获取配置摘要
    pub fn summary(&self) -> String {
        format!(
            "FeatureToggles {{ AI: {}, Validation: {}, Metadata: {}, SIMD: {}, Cache: {} }}",
            self.enable_ai_prediction,
            self.enable_validation,
            self.preserve_metadata,
            self.enable_simd,
            self.enable_cache
        )
    }
    
    /// 🤖 AI机器学习模式 - 最简化体验，AI自动优化所有参数
    /// 
    /// 特点：
    /// - AI自动预测所有参数
    /// - 用户可以自定义选项
    /// - 最简化的操作体验
    /// - 适合普通用户
    pub fn ai_mode() -> Self {
        Self {
            // AI功能全开
            enable_ai_prediction: true,
            ai_fallback_to_defaults: true,
            
            // 允许用户自定义覆盖
            allow_manual_override: true,
            enable_advanced_params: true,
            
            // 完整验证保证质量
            enable_validation: true,
            enable_file_validation: true,
            enable_quality_validation: true,
            
            // 性能优化全开
            enable_simd: true,
            enable_cache: true,
            enable_parallel: true,
            
            ..Default::default()
        }
    }
    
    /// 🔧 手动高级模式 - 完全控制，所有高级选项可用
    /// 
    /// 特点：
    /// - 关闭AI预测，完全手动控制
    /// - 所有高级参数可用
    /// - 适合专业用户和CLI使用
    /// - 插件高级模式和CLI都使用此配置
    pub fn manual_mode() -> Self {
        Self {
            // 关闭AI，完全手动
            enable_ai_prediction: false,
            allow_manual_override: true,
            enable_advanced_params: true,
            
            // 完整验证
            enable_validation: true,
            enable_file_validation: true,
            enable_quality_validation: true,
            
            // 性能优化全开
            enable_simd: true,
            enable_cache: true,
            enable_parallel: true,
            
            ..Default::default()
        }
    }
}

/// 功能开关管理器
pub struct FeatureToggleManager {
    toggles: FeatureToggles,
    runtime_overrides: HashMap<String, bool>,
}

impl FeatureToggleManager {
    /// 创建新的管理器
    pub fn new(toggles: FeatureToggles) -> Result<Self> {
        toggles.validate()?;
        Ok(Self {
            toggles,
            runtime_overrides: HashMap::new(),
        })
    }
    
    /// 获取当前配置
    pub fn toggles(&self) -> &FeatureToggles {
        &self.toggles
    }
    
    /// 更新配置
    pub fn update_toggles(&mut self, toggles: FeatureToggles) -> Result<()> {
        toggles.validate()?;
        self.toggles = toggles;
        Ok(())
    }
    
    /// 运行时覆盖单个开关
    pub fn override_toggle(&mut self, key: &str, value: bool) {
        self.runtime_overrides.insert(key.to_string(), value);
    }
    
    /// 清除运行时覆盖
    pub fn clear_overrides(&mut self) {
        self.runtime_overrides.clear();
    }
    
    /// 检查功能是否启用 (考虑运行时覆盖)
    pub fn is_enabled(&self, feature: &str) -> bool {
        if let Some(&override_value) = self.runtime_overrides.get(feature) {
            return override_value;
        }
        
        match feature {
            "ai_prediction" => self.toggles.enable_ai_prediction,
            "validation" => self.toggles.enable_validation,
            "file_validation" => self.toggles.enable_file_validation,
            "quality_validation" => self.toggles.enable_quality_validation,
            "preserve_metadata" => self.toggles.preserve_metadata,
            "merge_xmp_sidecar" => self.toggles.merge_xmp_sidecar,
            "keep_animated" => self.toggles.keep_animated,
            "normalize_filenames" => self.toggles.normalize_filenames,
            "simd" => self.toggles.enable_simd,
            "cache" => self.toggles.enable_cache,
            "parallel" => self.toggles.enable_parallel,
            "manual_override" => self.toggles.allow_manual_override,
            "advanced_params" => self.toggles.enable_advanced_params,
            "experimental" => self.toggles.enable_experimental,
            _ => false,
        }
    }
    
    /// 获取所有启用的功能列表
    pub fn enabled_features(&self) -> Vec<String> {
        let features = vec![
            ("ai_prediction", self.toggles.enable_ai_prediction),
            ("validation", self.toggles.enable_validation),
            ("file_validation", self.toggles.enable_file_validation),
            ("quality_validation", self.toggles.enable_quality_validation),
            ("preserve_metadata", self.toggles.preserve_metadata),
            ("merge_xmp_sidecar", self.toggles.merge_xmp_sidecar),
            ("keep_animated", self.toggles.keep_animated),
            ("normalize_filenames", self.toggles.normalize_filenames),
            ("simd", self.toggles.enable_simd),
            ("cache", self.toggles.enable_cache),
            ("parallel", self.toggles.enable_parallel),
            ("manual_override", self.toggles.allow_manual_override),
            ("advanced_params", self.toggles.enable_advanced_params),
            ("experimental", self.toggles.enable_experimental),
        ];
        
        features
            .into_iter()
            .filter(|(_, enabled)| *enabled)
            .map(|(name, _)| name.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_toggles() {
        let toggles = FeatureToggles::default();
        assert!(toggles.enable_ai_prediction);
        assert!(toggles.enable_validation);
        assert!(toggles.preserve_metadata);
        assert!(toggles.enable_simd);
    }
    
    #[test]
    fn test_ai_mode() {
        let toggles = FeatureToggles::ai_mode();
        // AI模式：AI预测开启，但允许用户自定义
        assert!(toggles.enable_ai_prediction);
        assert!(toggles.allow_manual_override);
        assert!(toggles.enable_advanced_params);
    }
    
    #[test]
    fn test_manual_mode() {
        let toggles = FeatureToggles::manual_mode();
        // 手动模式：AI关闭，完全手动控制
        assert!(!toggles.enable_ai_prediction);
        assert!(toggles.allow_manual_override);
        assert!(toggles.enable_advanced_params);
    }
    
    #[test]
    fn test_validation() {
        let mut toggles = FeatureToggles::default();
        assert!(toggles.validate().is_ok());
        
        toggles.ai_min_confidence = 1.5;
        assert!(toggles.validate().is_err());
        
        toggles.ai_min_confidence = 0.7;
        toggles.max_concurrent_conversions = 0;
        assert!(toggles.validate().is_err());
    }
    
    #[test]
    fn test_manager() {
        let toggles = FeatureToggles::default();
        let mut manager = FeatureToggleManager::new(toggles).unwrap();
        
        assert!(manager.is_enabled("ai_prediction"));
        
        manager.override_toggle("ai_prediction", false);
        assert!(!manager.is_enabled("ai_prediction"));
        
        manager.clear_overrides();
        assert!(manager.is_enabled("ai_prediction"));
    }
    
    #[test]
    fn test_json_serialization() {
        let toggles = FeatureToggles::default();
        let json = toggles.to_json().unwrap();
        let restored = FeatureToggles::from_json(&json).unwrap();
        
        assert_eq!(toggles.enable_ai_prediction, restored.enable_ai_prediction);
        assert_eq!(toggles.enable_validation, restored.enable_validation);
    }
}
