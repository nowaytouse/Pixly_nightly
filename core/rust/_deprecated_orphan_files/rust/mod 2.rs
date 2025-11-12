//! 图像转换核心模块
//! 
//! 实现完整的图像格式转换功能

pub mod image_converter;
pub mod metadata;      // 📋 Phase 24: 完整元数据保留 (EXIF/XMP/ICC) ✅ 生产使用
pub mod ai_client;    // 🤖 Phase 32: AI服务客户端 (GO AI集成)
pub mod validator;
pub mod validation;    // 🔒 Phase 24: 多级输入输出验证系统
pub mod cache;         // 💾 Phase 24: 智能缓存系统
pub mod params;        // 🎯 Phase 25: 智能参数优化系统
pub mod quality;       // 🎯 Phase 25.3: 质量保证系统 (SSIM/PSNR)
pub mod eagle_adapter;
pub mod animation_detector;  // 🎬 Phase 36: 动画检测器 (GIF)
pub mod metadata_extended;   // ⚠️ Phase 40.18: 已废弃 - 功能已合并到 metadata.rs，保留作历史参考
pub mod native_avif;  // 🦀 Phase 13: 原生AVIF编码器 (rav1e)
pub mod native_webp;  // 🦀 Phase 15: 原生WebP编码器 (webp)
pub mod native_png;   // 🦀 Phase 18: 原生PNG编码器 (png)
pub mod native_jpeg;  // 🦀 Phase 20: 原生JPEG编码器 (image)
pub mod strategy;     // 🎯 Phase 14: 统一策略系统
pub mod strategies;   // 📦 Phase 14-15: 具体策略实现
pub mod batch;        // 🚀 Phase 19: 批量转换 + 进度回调 + 多线程并行
pub mod filename_normalizer;  // 🔤 Phase 40.8: 文件名规范化（中间处理+还原）
pub mod conversion_cache;     // 💾 Phase 40.12: 智能转换缓存系统
pub mod video_processor;      // 🎬 Phase 40.13: 视频处理核心
pub mod video_strategy;       // 🔥 Phase 40.24: 视频转换策略
pub mod media_analyzer;       // 📊 Phase 40.13: 统一媒体分析接口（图片+视频+动图）
pub mod gif_optimizer;        // 🎨 Phase 40.24.2: 动画 GIF 优化
pub mod batch_processor;      // ⚡ Phase 40.24.3: 高性能批量处理
pub mod error_recovery;       // 🔄 Phase 40.24.4: 错误恢复机制
pub mod progress;             // 📊 Phase 40.24.5: 统一进度回调系统
pub mod dimension_validator;  // 📐 Phase 40.24: 尺寸限制验证
pub mod file_manager;         // 📁 文件管理核心模块
pub mod magika_detector;      // 🤖 Phase 45.1: Magika AI文件类型检测 (Google Security Research)

pub use image_converter::{ImageConverter, ConversionConfig, ConversionResult};
pub use batch::{BatchConverter, BatchTask, BatchResult, Progress, create_batch_from_directory};
pub use metadata::{MetadataHandler, ExifData, CompleteMetadataConfig}; // 🔥 Phase 40.18: 添加CompleteMetadataConfig
pub use validator::{QualityValidator, ValidationResult};
pub use eagle_adapter::EagleAdapter;

// 导出原生编码器
pub use native_avif::{NativeAvifEncoder, AvifConfig, ChromaSampling};
pub use native_webp::{NativeWebPEncoder, WebPConfig};
pub use native_png::{NativePngEncoder, PngConfig};

// 导出策略系统
pub use strategy::{
    ConversionStrategy, StrategyType, StrategyManager,
    ConversionConfig as StrategyConfig,
    ConversionResult as StrategyResult,
    PredictionData,  // 🔥 Phase 40.21: AI预测数据
};
pub use strategies::{NativeAvifStrategy, NativeWebPStrategy, NativePngStrategy, CliStrategy, CliTool};

/// 转换选项
#[derive(Debug, Clone)]
pub struct ConvertOptions {
    /// 目标格式
    pub target_format: String,
    /// 质量 (0-100)
    pub quality: u8,
    /// 编码速度/effort
    pub speed: u8,
    /// 保留元数据
    pub preserve_metadata: bool,
    /// 保留动画
    pub keep_animated: bool,
    /// 原地替换
    pub in_place: bool,
}

impl Default for ConvertOptions {
    fn default() -> Self {
        Self {
            target_format: "jpeg".to_string(),
            quality: 75,
            speed: 4,
            preserve_metadata: true,
            keep_animated: true,
            in_place: false,
        }
    }
}

/// 转换状态
#[derive(Debug, Clone)]
pub struct ConversionStatus {
    pub success: bool,
    pub message: String,
    pub output_path: Option<String>,
    pub output_size: u64,
    pub processing_time_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_options_default() {
        let opts = ConvertOptions::default();
        assert_eq!(opts.quality, 75);
        assert_eq!(opts.speed, 4);
        assert!(opts.preserve_metadata);
    }
}
