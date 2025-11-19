// 🚨 错误处理模块 - 必须首先声明
pub mod errors;

// 📋 Phase 4 模块复活化 (2025-11-19)
// ================================
// 纠正草率处理错误，负责任地集成高价值模块
// 
// Phase 4.1: 立即集成6个最高价值模块
// - simd_processor: SIMD加速处理 (性能提升2-4x)
// - smart_cache: 智能LRU缓存系统
// - metadata_comprehensive: 最全面的元数据保留
// - bayesian_optimizer: 贝叶斯参数优化
// - animation_strategy: 动画编码策略选择
// - external_tools: 外部工具检测和管理
// 
// 详细计划: docs/MODULE_RESURRECTION_PLAN_PHASE4.md
// ================================

pub mod types;
pub mod sharpen;
pub mod ai;
pub mod video;
pub mod format_recommender;
pub mod ppo_model;
pub mod preprocessing;
pub mod core_processor;
pub mod performance;
pub mod transform;
pub mod validation;
pub mod quality_analyzer;
pub mod filename_normalizer;
pub mod feature_extractor;
pub mod time_estimator;
pub mod progress;
pub mod format_selector;  // Phase 4: 智能格式选择
pub mod media_analyzer;
pub mod image_params;
pub mod quality_checker;
pub mod magika_detector;
pub mod eagle_adapter;
pub mod dependency_checker;
pub mod cli_audio;
pub mod cli_analyze;  // ✅ AI-powered media analysis
pub mod conversion_core;
pub mod audio_processor;  // 🎵 Phase 1: 音频处理核心（从僵尸代码中恢复）

// Phase 3.1: PPO强化学习
pub mod reward_calculator;
// Phase 3.2: 在线学习
pub mod online_learning;
pub mod online_learner_manager;
pub mod python_ml_caller;  // 🔥 Python ML Bridge调用模块
pub mod video_features;  // 🎬 视频特征提取
pub mod file_attributes;  // 🔥 文件属性保留（时间戳 + 扩展属性）
pub mod format_corrector;  // 🔧 格式自动修正
pub mod conversion_validator;
pub mod validation_integration;
pub mod ml_predictor;
pub mod dynamic_concurrency;
pub mod linear_regression;
pub mod quality_metrics;
pub mod progress_tracker;
pub mod quality_reporter;
pub mod alpha_predictor;
pub mod zero_copy_buffer;
pub mod video_processor;
pub mod video_strategy;  // 🎬 Phase 1: 视频编码策略（从僵尸代码恢复）
// TODO Phase 2: unified_cache/parallel/progress 需要重构依赖后集成
// pub mod unified_cache;
// pub mod unified_parallel;
// pub mod unified_progress;
pub mod feature_extractor_128d;
pub mod ppo_model_enhanced;
pub mod modern_formats;
pub mod transparent_logger;
pub mod unified_conversion_engine;
pub mod log_manager;
pub mod file_collector;
pub mod config_manager;

// 🔥 Phase 4.1: 高价值模块集成
pub mod simd_processor;           // SIMD加速处理
pub mod smart_cache;              // 智能LRU缓存
pub mod metadata_comprehensive;   // 最全面的元数据保留
pub mod bayesian_optimizer;       // 贝叶斯参数优化
pub mod animation_strategy;       // 动画编码策略
pub mod external_tools;           // 外部工具管理

// 🔥 Phase 4.2: 中等价值模块集成
pub mod automl;                   // 自动机器学习
pub mod visual_quality_scorer;    // 视觉质量评分
pub mod simd_sharpener;           // SIMD锐化算法
pub mod ml_time_estimator;        // ML时间估算
pub mod same_format_optimizer;    // 同格式优化
pub mod quality_checker_advanced; // 高级质量检查
pub mod custom_presets;           // 自定义预设管理
pub mod file_type_detector;       // 文件类型检测

pub use types::*;
pub use ppo_model_enhanced::{EnhancedPPOPredictor, MediaType, TrainingSample};
pub use modern_formats::{ModernFormatConverter, AVIFParams, JXLParams, FormatSupport};
pub use unified_conversion_engine::{UnifiedConversionEngine, UnifiedConversionConfig};
pub use log_manager::{LogManager, LogLevel as LogManagerLevel, LogConfig};
pub use file_collector::FileCollector;
pub use config_manager::{ConfigManager, Config};

// 🔥 Phase 4.1: 导出高价值模块
pub use simd_processor::{SIMDProcessor, SimdProcessor};
pub use smart_cache::{SmartCache, CacheEntry, CacheStats};
pub use metadata_comprehensive::{
    ComprehensiveMetadata, TechnicalMetadata, DescriptiveMetadata,
    AdministrativeMetadata, StructuralMetadata, UsageMetadata, BusinessMetadata
};
pub use bayesian_optimizer::{BayesianOptimizer, OptimizationObjective, ParameterSpace, Observation};
pub use animation_strategy::{
    AnimationStrategy, AnimationInfo, AnimationStrategySelector, 
    AnimationToVideoConverter, AnimationPreservation
};
pub use external_tools::{ExternalTool, ToolStatus, ExternalToolChecker};

// 🔥 Phase 4.2: 导出中等价值模块
pub use automl::{AutoML, AutoMLConfig, ModelType, FeatureImportance, ModelMetrics};
pub use visual_quality_scorer::{VisualQualityScorer, ImageFeatures as VQSImageFeatures, QualityRecommendation};
pub use simd_sharpener::{SIMDSharpener, SharpenConfig, SharpenPerformanceInfo};
pub use ml_time_estimator::{TimeEstimator, TimeEstimate, FileFeatures as MLFileFeatures, ConversionRecord};
pub use same_format_optimizer::{SameFormatOptimizer, OptimizationResult as SFOptimizationResult};
pub use quality_checker_advanced::{QualityChecker as AdvancedQualityChecker, QualityMetrics as AdvancedQualityMetrics, QualityAssessment};
pub use custom_presets::{CustomPreset, PresetManager};
pub use file_type_detector::{FileTypeDetector, FileTypeDetection, SecurityValidation};
pub use sharpen::*;
pub use ai::*;
pub use video::*;
pub use preprocessing::*;
pub use performance::*;
pub use transform::*;
pub use validation::*;
pub use quality_analyzer::*;
pub use filename_normalizer::*;

// feature_extractor有名称冲突，使用模块导入
pub use feature_extractor::{FeatureExtractor, MetadataFeatures as MLMetadataFeatures, ContextFeatures};

// time_estimator和quality_predictor都导出ConversionParams，只保留一个
// pub use time_estimator::*;
pub use progress::*;
pub use media_analyzer::*;
pub use image_params::*;

// quality_checker和quality_analyzer都导出QualityMetrics，使用模块导入避免冲突
pub use quality_checker::QualityChecker;

// 核心处理器和批处理器（避免名称冲突）
pub use core_processor::{ImageProcessor as CoreImageProcessor, ProcessingConfig as CoreProcessingConfig, ProcessingResult as CoreProcessingResult, ImageInfo as CoreImageInfo};

// 新增模块导出（避免名称冲突）
pub use eagle_adapter::*;
pub use dependency_checker::*;

// conversion_core和strategy都有ConversionConfig/ConversionResult，使用模块前缀
pub use conversion_core::execute_conversion;

// video_handler和video都有VideoInfo，使用模块前缀


// conversion_engine和strategy都有ConversionResult，使用模块前缀

pub use ml_predictor::*;
pub use dynamic_concurrency::*;
pub use linear_regression::*;

// quality_metrics和quality_analyzer都有QualityMetrics/QualityDistribution，使用模块前缀
// pub use quality_metrics::{QualityReporter, QualityReport, CompressionStats}; // 已被新的quality_metrics模块替代
pub use progress_tracker::{ProgressTracker, ProgressInfo};
pub use quality_reporter::{Reporter, QualityMetrics as ReporterQualityMetrics};
pub use alpha_predictor::AlphaQualityPredictor;

// CLI模块（通常不需要re-export，但保留声明）
// pub use cli_convert::*;
// pub use cli_batch::*;
// pub use cli_audio::*;
// pub use cli_analyze::*;
pub mod ml_bridge;
pub mod format_knowledge;
pub mod feature_toggles;
pub mod unified_ai_interface;
pub mod format_params;

// 导出功能开关
pub use feature_toggles::FeatureToggles;
pub use unified_ai_interface::{UnifiedAIManager, UnifiedAIRequest, UnifiedAIResponse, AIPreferences};
pub use format_params::{FormatSpecificParams, JxlParams, WebPParams, AvifParams, HeicParams};
pub use audio_processor::{AudioProcessor, AudioInfo, AudioConversionConfig, AudioConversionResult};
pub use video_strategy::{VideoCodec, QualityTarget, VideoConversionStrategy, AudioStrategy};

