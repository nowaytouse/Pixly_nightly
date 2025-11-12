//! 🎨 PIXLY Converter - 核心转换模块
//! 
//! 扁平化架构，所有模块在同一层级，便于维护和理解

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🎯 核心转换引擎
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
pub mod image_converter;
pub mod strategy;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🦀 原生编码器（Rust实现）
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
pub mod native_avif;
pub mod native_webp;
pub mod native_png;
pub mod native_jpeg;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 📦 转换策略实现
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
pub mod cli_strategy;
pub mod native_avif_strategy;
pub mod native_webp_strategy;
pub mod native_png_strategy;
pub mod native_jpeg_strategy;
pub mod gif_animation_strategy;
pub mod same_format_optimizer;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🎬 视频与动画处理
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
pub mod video_processor;
pub mod video_strategy;
pub mod animation_detector;
pub mod gif_optimizer;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 📊 媒体分析与验证
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
pub mod media_analyzer;
pub mod validation;
pub mod validator;
pub mod quality;
pub mod dimension_validator;
pub mod magika_detector;  // AI文件类型检测

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 📋 元数据处理
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
pub mod metadata;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🤖 AI集成
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
pub mod ai_client;
pub mod params;                 // 参数结构定义
pub mod ai_parameter_provider;  // Phase 46.9: AI参数提供器（调用Go AI服务）

#[cfg(test)]
pub mod param_tests;      // 参数优化测试

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🚀 批量处理与进度
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
pub mod batch;
pub mod batch_processor;
pub mod progress;
pub mod task_queue;
pub mod parallel_encoder;  // 🆕 Phase 46.14+ (R-005): 并行编码优化  // 🆕 Phase 46.14+: F-004 任务队列系统

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 💾 缓存与文件管理
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
pub mod cache;
// pub mod conversion_cache;  // 🗑️ Phase 47.5: 移除（未使用的孤儿代码）
pub mod file_manager;
pub mod filename_normalizer;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 🔄 Eagle集成
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// pub mod error_recovery;  // 🗑️ Phase 47.5: 移除（未使用的孤儿代码）
pub mod eagle_adapter;  // Eagle图库集成

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 📤 公共导出
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// 核心转换
pub use image_converter::{ImageConverter, ConversionConfig, ConversionResult};
pub use strategy::{
    ConversionStrategy, StrategyType, StrategyManager,
    ConversionConfig as StrategyConfig,
    ConversionResult as StrategyResult,
    PredictionData,
};

// 批量处理
pub use batch::{BatchConverter, BatchTask, BatchResult, Progress, create_batch_from_directory};
pub use task_queue::{TaskQueue, ConversionTask, TaskStatus, TaskPriority, QueueStats};  // F-004

// 元数据
pub use metadata::{MetadataHandler, ExifData, CompleteMetadataConfig};

// 验证
pub use validator::{QualityValidator, ValidationResult};

// 原生编码器
pub use native_avif::{NativeAvifEncoder, AvifConfig, ChromaSampling};
pub use native_webp::{NativeWebPEncoder, WebPConfig};
pub use native_png::{NativePngEncoder, PngConfig};

// 策略实现
pub use cli_strategy::{CliStrategy, CliTool};
pub use native_avif_strategy::NativeAvifStrategy;
pub use native_webp_strategy::NativeWebPStrategy;
pub use native_png_strategy::NativePngStrategy;

// 集成适配器
pub use eagle_adapter::EagleAdapter;

/// 转换选项
#[derive(Debug, Clone)]
pub struct ConvertOptions {
    pub target_format: String,
    pub quality: u8,
    pub speed: u8,
    pub preserve_metadata: bool,
    pub keep_animated: bool,
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

/// 注册所有转换策略
/// 
/// 在扁平化重构后，此函数从strategies/mod.rs迁移到这里
pub fn register_all_strategies(manager: &mut StrategyManager) {
    use tracing::info;
    
    info!("📋 Registering conversion strategies...");
    
    // 🎬 动画GIF策略 (优先级90)
    manager.register(Box::new(gif_animation_strategy::AnimatedGifStrategy));
    info!("  ✓ Animated GIF Strategy");
    
    // 🦀 原生编码器（优先级100）
    #[cfg(feature = "native-avif")]
    {
        manager.register(Box::new(NativeAvifStrategy));
        info!("  ✓ Native AVIF (rav1e)");
    }
    
    #[cfg(feature = "native-webp")]
    {
        manager.register(Box::new(NativeWebPStrategy));
        info!("  ✓ Native WebP");
    }
    
    manager.register(Box::new(NativePngStrategy));
    info!("  ✓ Native PNG");
    
    manager.register(Box::new(native_jpeg_strategy::NativeJpegStrategy));
    info!("  ✓ Native JPEG");
    
    // 📦 CLI工具策略
    manager.register(Box::new(CliStrategy::new(CliTool::Avifenc)));
    info!("  ✓ CLI AVIF (avifenc)");
    
    manager.register(Box::new(CliStrategy::new(CliTool::Cjxl)));
    info!("  ✓ CLI JXL (cjxl)");
    
    manager.register(Box::new(CliStrategy::new(CliTool::Djxl)));
    info!("  ✓ CLI JXL Decoder (djxl)");
    
    manager.register(Box::new(CliStrategy::new(CliTool::Cwebp)));
    info!("  ✓ CLI WebP (cwebp)");
    
    manager.register(Box::new(CliStrategy::new(CliTool::Magick)));
    info!("  ✓ CLI Generic (magick)");
    
    // 🔄 同格式优化器（优先级80）
    manager.register(Box::new(same_format_optimizer::SameFormatOptimizer::new()));
    info!("  ✓ Same-Format Optimizer");
    
    info!("🎯 {} strategies registered", manager.available_strategies().len());
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
