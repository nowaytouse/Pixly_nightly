pub mod types;
pub mod sharpen;
pub mod ai;
pub mod local_ai;
pub mod video;
pub mod custom_presets;
pub mod format_recommender;
pub mod external_tools;
pub mod animation_strategy;
pub mod metadata_comprehensive;
pub mod ppo_model;
pub mod batch;
pub mod formats;
pub mod quality_predictor;
pub mod preprocessing;
pub mod core_processor;
pub mod batch_processor;
pub mod metadata_processor;
pub mod gif_processor;
pub mod performance;
pub mod transform;
pub mod validation;
pub mod quality_analyzer;
pub mod filename_normalizer;
pub mod feature_extractor;
pub mod time_estimator;
pub mod cache;
pub mod parallel;
pub mod progress;
pub mod ram_optimizer;
pub mod ai_interface;
pub mod format_optimizer;
pub mod strategy;
pub mod media_analyzer;
pub mod image_params;
pub mod quality_checker;
pub mod file_detector;
pub mod magika_detector;
pub mod eagle_adapter;
pub mod batch_decision;
pub mod dependency_checker;
pub mod cli_convert;
pub mod cli_batch;
pub mod cli_audio;
pub mod cli_analyze;
pub mod conversion_core;
pub mod metadata_handler;
pub mod video_handler;
pub mod gif_handler;
pub mod conversion_engine;
pub mod conversion_validator;
pub mod validation_integration;
pub mod memory_manager;
pub mod ml_predictor;
pub mod dynamic_concurrency;
pub mod linear_regression;
pub mod quality_metrics;
pub mod progress_tracker;
pub mod conflict_detector;
pub mod quality_reporter;
pub mod quality_adjuster;
pub mod worker_pool;
pub mod alpha_predictor;
pub mod chroma_predictor;
pub mod visual_quality_scorer;
pub mod smart_concurrency;
pub mod predictor_core;
pub mod image_analyzer;
pub mod training_data;
pub mod zero_copy_buffer;
pub mod simd_processor;
pub mod image_transform;
pub mod animation_detector;
pub mod file_type_detector;
pub mod video_processor;
pub mod ml_time_estimator;
pub mod gif_optimizer_advanced;
pub mod gif_animation_strategy;
pub mod ram_optimizer_advanced;
pub mod managed_memory;
pub mod same_format_optimizer;
pub mod batch_processor_advanced;
pub mod image_transformer;
pub mod color_quantizer;
pub mod smart_cache;
pub mod quality_checker_advanced;
pub mod simd_sharpener;
pub mod batch_decision_manager;
pub mod feature_extractor_128d;
pub mod ppo_model_enhanced;
pub mod modern_formats;
pub mod transparent_logger;
pub mod unified_conversion_engine;
pub mod log_manager;
pub mod file_collector;
pub mod batch_converter;
pub mod cli_main;
pub mod config_manager;

pub use types::*;
pub use ppo_model_enhanced::{EnhancedPPOPredictor, MediaType, TrainingSample};
pub use modern_formats::{ModernFormatConverter, AVIFParams, JXLParams, FormatSupport};
pub use unified_conversion_engine::{UnifiedConversionEngine, UnifiedConversionConfig};
pub use log_manager::{LogManager, LogLevel as LogManagerLevel, LogConfig};
pub use file_collector::FileCollector;
pub use batch_converter::{BatchConverter, BatchConverterConfig, BatchResult, ErrorStrategy};
pub use config_manager::{ConfigManager, Config};
pub use sharpen::*;
pub use ai::*;
pub use local_ai::*;
pub use video::*;
pub use batch::*;
pub use formats::*;
pub use quality_predictor::*;
pub use preprocessing::*;
pub use metadata_processor::*;
pub use gif_processor::*;
pub use performance::*;
pub use transform::*;
pub use validation::*;
pub use quality_analyzer::*;
pub use filename_normalizer::*;

// feature_extractor有名称冲突，使用模块导入
pub use feature_extractor::{FeatureExtractor, MetadataFeatures as MLMetadataFeatures, ContextFeatures};

// time_estimator和quality_predictor都导出ConversionParams，只保留一个
// pub use time_estimator::*;
pub use cache::*;
pub use parallel::*;
pub use progress::*;
pub use ram_optimizer::*;
pub use ai_interface::*;
pub use format_optimizer::*;
pub use strategy::*;
pub use media_analyzer::*;
pub use image_params::*;

// quality_checker和quality_analyzer都导出QualityMetrics，使用模块导入避免冲突
pub use quality_checker::QualityChecker;
pub use file_detector::*;

// 核心处理器和批处理器（避免名称冲突）
pub use core_processor::{ImageProcessor as CoreImageProcessor, ProcessingConfig as CoreProcessingConfig, ProcessingResult as CoreProcessingResult, ImageInfo as CoreImageInfo};
pub use batch_processor::{BatchProcessor as CoreBatchProcessor, BatchConfig as CoreBatchConfig, BatchResult as CoreBatchResult};

// 新增模块导出（避免名称冲突）
pub use eagle_adapter::*;
pub use batch_decision::*;
pub use dependency_checker::*;

// conversion_core和strategy都有ConversionConfig/ConversionResult，使用模块前缀
pub use conversion_core::execute_conversion;
pub use metadata_handler::*;

// video_handler和video都有VideoInfo，使用模块前缀
pub use video_handler::{VideoConversionConfig, AudioMode};

pub use gif_handler::*;

// conversion_engine和strategy都有ConversionResult，使用模块前缀
pub use conversion_engine::{ConversionEngine, ConversionEngineConfig, ConversionRequest};

pub use memory_manager::*;
pub use ml_predictor::*;
pub use dynamic_concurrency::*;
pub use linear_regression::*;

// quality_metrics和quality_analyzer都有QualityMetrics/QualityDistribution，使用模块前缀
// pub use quality_metrics::{QualityReporter, QualityReport, CompressionStats}; // 已被新的quality_metrics模块替代
pub use progress_tracker::{ProgressTracker, ProgressInfo};
pub use conflict_detector::{ConflictDetector, ConflictReport, Conflict, ConflictSeverity};
pub use quality_reporter::{Reporter, QualityMetrics as ReporterQualityMetrics};
pub use quality_adjuster::{QualityAdjuster, PredictionParams};
pub use worker_pool::{WorkerPool, calculate_megapixels};
pub use alpha_predictor::AlphaQualityPredictor;
pub use chroma_predictor::ChromaSubsamplingPredictor;
pub use visual_quality_scorer::{VisualQualityScorer, ImageFeatures as VQSImageFeatures, QualityRecommendation};
pub use smart_concurrency::{SmartConcurrencyManager, ConcurrencyStats};
pub use predictor_core::{PredictorCore, FileFeatures as PredictorFileFeatures, ConversionParams, Prediction};
pub use image_analyzer::{ImageAnalyzer, ImageQualityMetrics};
pub use training_data::{TrainingDataset, TrainingRecord, TrainingReport, load_training_report};

// CLI模块（通常不需要re-export，但保留声明）
// pub use cli_convert::*;
// pub use cli_batch::*;
// pub use cli_audio::*;
// pub use cli_analyze::*;
pub mod bayesian_optimizer;
pub mod automl;
pub mod ml_bridge;
pub mod ml_data_flow;
pub mod model_router;
pub mod format_knowledge;
pub mod feature_toggles;
pub mod unified_ai_interface;
pub mod format_params;

// 导出功能开关
pub use feature_toggles::{FeatureToggles, FeatureToggleManager};
pub use unified_ai_interface::{UnifiedAIManager, UnifiedAIRequest, UnifiedAIResponse, AIPreferences};
pub use format_params::{FormatSpecificParams, JxlParams, WebPParams, AvifParams, HeicParams};
