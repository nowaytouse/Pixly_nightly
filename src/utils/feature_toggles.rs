// 🎛️ 功能开关模块
// 统一管理所有辅助功能的开关状态

/// 功能开关配置
#[derive(Debug, Clone, Default)]
pub struct FeatureToggles {
    // ═══════════════════════════════════════════════════
    // 🧠 AI 机器学习功能
    // ═══════════════════════════════════════════════════
    
    /// 🎯 智能参数预测
    pub enable_ai_prediction: bool,
    
    /// 🔒 AI 文件验证 (Magika)
    pub enable_file_validation: bool,
    
    /// 📊 SSIM 质量验证
    pub enable_ssim: bool,
    
    /// ⚡ GPU 硬件加速
    pub enable_gpu: bool,
    
    /// 🔗 智能预处理
    pub enable_preprocess: bool,
    
    /// 🔧 格式自动修正
    pub enable_format_correction: bool,
    
    // ═══════════════════════════════════════════════════
    // 🎬 视频 AI 功能
    // ═══════════════════════════════════════════════════
    
    /// 🎬 动图转视频推荐
    pub enable_video_for_animation: bool,
    
    /// 🎞️ 场景检测
    pub enable_scene_detection: bool,
    
    /// 📊 VMAF 质量验证
    pub enable_vmaf: bool,
    
    /// 🔄 Two-Pass 编码
    pub enable_two_pass: bool,
}

impl FeatureToggles {
    /// 创建新的功能开关配置（所有功能默认关闭）
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 创建推荐配置（常用功能开启）
    pub fn recommended() -> Self {
        Self {
            enable_ai_prediction: true,
            enable_file_validation: true,
            enable_ssim: false,  // 性能影响较大，默认关闭
            enable_gpu: true,
            enable_preprocess: true,
            enable_format_correction: false,  // 实验性功能
            enable_video_for_animation: true,
            enable_scene_detection: false,  // 性能影响较大
            enable_vmaf: false,  // 性能影响较大
            enable_two_pass: false,  // 时间成本高
        }
    }
    
    /// 创建最大性能配置（所有优化功能开启）
    pub fn max_performance() -> Self {
        Self {
            enable_ai_prediction: true,
            enable_file_validation: true,
            enable_ssim: true,
            enable_gpu: true,
            enable_preprocess: true,
            enable_format_correction: true,
            enable_video_for_animation: true,
            enable_scene_detection: true,
            enable_vmaf: true,
            enable_two_pass: true,
        }
    }
    
    /// 检查是否有任何AI功能启用
    pub fn has_ai_features(&self) -> bool {
        self.enable_ai_prediction
            || self.enable_file_validation
            || self.enable_preprocess
            || self.enable_format_correction
    }
    
    /// 检查是否有任何视频AI功能启用
    pub fn has_video_ai_features(&self) -> bool {
        self.enable_video_for_animation
            || self.enable_scene_detection
            || self.enable_vmaf
            || self.enable_two_pass
    }
    
    /// 检查是否有任何质量验证功能启用
    pub fn has_quality_checks(&self) -> bool {
        self.enable_ssim || self.enable_vmaf
    }
    
    /// 生成功能摘要
    pub fn summary(&self) -> String {
        let mut features = Vec::new();
        
        if self.enable_ai_prediction {
            features.push("AI");
        }
        if self.enable_file_validation {
            features.push("FileVal");
        }
        if self.enable_ssim {
            features.push("SSIM");
        }
        if self.enable_gpu {
            features.push("GPU");
        }
        if self.enable_preprocess {
            features.push("Preproc");
        }
        if self.enable_format_correction {
            features.push("FmtCorr");
        }
        if self.enable_video_for_animation {
            features.push("Vid4Anim");
        }
        if self.enable_scene_detection {
            features.push("SceneDet");
        }
        if self.enable_vmaf {
            features.push("VMAF");
        }
        if self.enable_two_pass {
            features.push("2Pass");
        }
        
        if features.is_empty() {
            "None".to_string()
        } else {
            features.join(", ")
        }
    }
    
    /// 验证功能开关配置
    pub fn validate(&self) -> anyhow::Result<()> {
        // SSIM和VMAF不能同时启用（性能考虑）
        if self.enable_ssim && self.enable_vmaf {
            anyhow::bail!("SSIM and VMAF cannot be enabled simultaneously (performance impact)");
        }
        
        // Two-Pass需要场景检测支持
        if self.enable_two_pass && !self.enable_scene_detection {
            log::warn!("⚠️  Warning: Two-Pass encoding works best with scene detection enabled");
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let toggles = FeatureToggles::new();
        assert!(!toggles.enable_ai_prediction);
        assert!(!toggles.has_ai_features());
    }
    
    #[test]
    fn test_recommended_config() {
        let toggles = FeatureToggles::recommended();
        assert!(toggles.enable_ai_prediction);
        assert!(toggles.enable_gpu);
        assert!(toggles.has_ai_features());
    }
    
    #[test]
    fn test_max_performance_config() {
        let toggles = FeatureToggles::max_performance();
        assert!(toggles.enable_ssim);
        assert!(toggles.enable_two_pass);
        assert!(toggles.has_quality_checks());
    }
}
