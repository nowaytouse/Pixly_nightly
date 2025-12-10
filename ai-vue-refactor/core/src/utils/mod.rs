// Utility modules
pub mod config_manager;
pub mod conversion_validator;
pub mod custom_presets;
pub mod dependency_checker;
pub mod dynamic_concurrency;
pub mod eagle_adapter;
pub mod external_tools;
pub mod error_recovery; // 🛡️ new：errorresumed
pub mod feature_toggles;
pub mod file_attributes;
pub mod file_collector;
pub mod file_type_detector;
pub mod filename_normalizer;
pub mod format_corrector;
pub mod format_knowledge;
pub mod format_params;
pub mod format_recommender;
pub mod format_selector;
pub mod i18n_messages; // 🌐 new：国际化消息
pub mod image_params;
pub mod log_manager;
pub mod magika_detector;
pub mod metadata_comprehensive;
pub mod modern_format_loader;
pub mod quality_presets;
pub mod same_format_optimizer;
pub mod simd_processor;
pub mod smart_cache_strategy; // 🧠 new：intelligentcachepolicy
pub mod time_estimator;
pub mod transparent_logger;
pub mod unified_ai_interface;
pub mod unified_cache;
pub mod unified_validator; // 🔥 new：avalidate
pub mod validation;
pub mod zero_copy_buffer;
