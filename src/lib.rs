// ! Pixly Kernel - AI-Driven Media Format Converter
//!
//! 🔥 架构原则 (2025-11-22 重构)
//! - 完全AI驱动（零硬编码规则）
//! - 零fallback hell
//! - 响亮失败（不静默降级）
//! - 模块化架构（清晰分离关注点）
//!
//! 详见: PROJECT_QUALITY_MANIFESTO.md

// 🚨 错误处理和类型定义 - 必须首先声明
pub mod errors;
pub mod types;
pub mod constants;

// 🏗️ Phase 2: 模块化架构 (2025-11-22)
// ================================
// 将82个平铺文件重组为清晰的模块化结构
// 参考: rimage, sharp, Symphonia 等优秀项目
// ================================

/// 核心转换引擎模块
pub mod core;

/// 编码器模块（image/video/audio）
pub mod codecs;

/// 预处理操作模块
pub mod operations;

/// AI和机器学习模块
pub mod ai;

/// 质量分析模块
pub mod analysis;

/// CLI相关模块
pub mod cli;

/// 工具函数模块
pub mod utils;

// 🔥 核心模块重新导出
pub use core::conversion_core::execute_conversion;
pub use core::core_processor::{
    ImageProcessor as CoreImageProcessor,
    ProcessingConfig as CoreProcessingConfig,
    ProcessingResult as CoreProcessingResult,
    ImageInfo as CoreImageInfo,
};
pub use core::performance::*;
pub use core::unified_conversion_engine::{
    UnifiedConversionEngine,
    UnifiedConversionConfig,
};

// 🎨 编码器重新导出
pub use codecs::image::modern_formats::{
    ModernFormatConverter,
    AVIFParams,
    JXLParams,
    FormatSupport,
};
pub use codecs::video::animation_strategy::{
    AnimationStrategy,
    AnimationInfo,
    AnimationStrategySelector,
    AnimationToVideoConverter,
    AnimationPreservation,
};
pub use codecs::video::h266::{H266Encoder, H266Params};  // 🔥 Phase 4: H.266/VVC
pub use codecs::video::core::*;
pub use codecs::video::video_processor::{
    VideoProcessor,
    VideoConversionConfig,
    AudioMode,
};

pub use codecs::video::video_strategy::{
    VideoCodec,
    QualityTarget,
    VideoConversionStrategy,
    AudioStrategy,
};
pub use codecs::audio::audio_processor::{
    AudioProcessor,
    AudioInfo,
    AudioConversionConfig,
    AudioConversionResult,
};

// 🔧 操作模块重新导出
pub use operations::preprocessing::*;
pub use operations::transform::*;
pub use operations::sharpen::*;
pub use operations::color_quantizer::{ColorQuantizer, QuantizationConfig};

// 🤖 AI模块重新导出
pub use ai::core::*;
pub use ai::ppo_model_enhanced::{EnhancedPPOPredictor, MediaType, TrainingSample};
pub use ai::online_learner_manager::OnlineLearnerManager;
pub use ai::bayesian_optimizer::{
    BayesianOptimizer,
    OptimizationObjective,
    ParameterSpace,
    Observation,
};
pub use ai::alpha_predictor::AlphaQualityPredictor;
pub use ai::ml_bridge::*;
pub use ai::python_ml_caller::*;

// 📊 分析模块重新导出
pub use analysis::quality_analyzer::*;
pub use analysis::quality_checker::{QualityChecker, AdvancedQualityMetrics, QualityGrade};
pub use analysis::quality_metrics::*;
pub use analysis::quality_reporter::{Reporter, QualityReport, CompressionStats};
pub use analysis::media_analyzer::*;
pub use analysis::visual_quality_scorer::{
    VisualQualityScorer,
    ImageFeatures as VQSImageFeatures,
    QualityRecommendation,
};

// 🖥️ CLI模块重新导出
pub use cli::cli_analyze::*;
pub use cli::cli_audio::*;
pub use cli::progress::*;
pub use cli::progress_tracker::{ProgressTracker, ProgressInfo};

// 🛠️ 工具模块重新导出
pub use utils::config_manager::{ConfigManager, Config};
pub use utils::dependency_checker::*;
pub use utils::dynamic_concurrency::*;
pub use utils::eagle_adapter::*;
pub use utils::external_tools::{ExternalTool, ToolStatus, ExternalToolChecker};
pub use utils::feature_toggles::FeatureToggles;
pub use utils::file_collector::FileCollector;
pub use utils::file_type_detector::{
    FileTypeDetector,
    FileTypeDetection,
    SecurityValidation,
};
pub use utils::filename_normalizer::*;
pub use utils::format_knowledge::*;
pub use utils::format_params::{
    FormatSpecificParams,
    JxlParams,
    WebPParams,
    AvifParams,
    HeicParams,
};
pub use utils::format_recommender::*;
pub use utils::format_selector::*;
pub use utils::image_params::*;
pub use utils::log_manager::{LogManager, LogLevel as LogManagerLevel, LogConfig};
pub use utils::magika_detector::*;
pub use utils::metadata_comprehensive::{
    ComprehensiveMetadata,
    TechnicalMetadata,
    DescriptiveMetadata,
    AdministrativeMetadata,
    StructuralMetadata,
    UsageMetadata,
    BusinessMetadata,
};
pub use utils::quality_presets::*;
pub use utils::same_format_optimizer::{
    SameFormatOptimizer,
    OptimizationResult as SFOptimizationResult,
};
pub use utils::simd_processor::{
    SIMDProcessor,
    SimdProcessor,
    SharpenConfig,
    SharpenPerformanceInfo,
};
pub use utils::time_estimator::*;
pub use utils::unified_ai_interface::{
    UnifiedAIManager,
    UnifiedAIRequest,
    UnifiedAIResponse,
    AIPreferences,
};
pub use utils::unified_cache::*;
pub use utils::unified_validator::{UnifiedValidator, ConversionConfig as ValidatorConfig, InputFile as ValidatorInputFile, ConversionMode};
pub use utils::i18n_messages::{I18nMessages, Language, MessageKey};
pub use utils::validation::*;
pub use utils::zero_copy_buffer::*;
pub use utils::custom_presets::{CustomPreset, PresetManager};

// Re-export common types
pub use types::*;
