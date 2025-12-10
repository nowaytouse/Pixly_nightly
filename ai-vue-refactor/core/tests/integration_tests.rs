//! Integration Tests for Pixly Kernel
//!
//! These tests verify that different modules work together correctly.

use pixly_kernel::*;

// ============================================================
// AI Module Integration Tests
// ============================================================

mod ai_integration {
    use super::*;

    #[test]
    fn test_bayesian_optimizer_creation() {
        let optimizer = BayesianOptimizer::with_defaults();
        assert!(optimizer.get_observations().is_empty());
    }

    #[test]
    fn test_bayesian_optimizer_suggest() {
        let optimizer = BayesianOptimizer::with_defaults();
        let suggestion = optimizer.suggest_next_parameters();

        // Should suggest valid parameters
        assert!(suggestion.0 >= 60 && suggestion.0 <= 100); // quality
        assert!(suggestion.1 >= 1 && suggestion.1 <= 9);    // effort
    }

    #[test]
    fn test_alpha_predictor_creation() {
        let predictor = AlphaQualityPredictor::new();
        let _ = predictor;
    }

    #[test]
    fn test_bayesian_optimizer_with_observations() {
        let mut optimizer = BayesianOptimizer::with_defaults();

        let observation = Observation {
            quality: 85,
            effort: 6,
            lossless: false,
            actual_quality: 0.96,
            actual_size: 50000,
            score: 0.85,
        };

        optimizer.add_observation(observation);
        assert_eq!(optimizer.get_observations().len(), 1);

        let suggestion = optimizer.suggest_next_parameters();
        assert!(suggestion.0 >= 60 && suggestion.0 <= 100);
    }
}

// ============================================================
// Quality Analysis Integration Tests
// ============================================================

mod quality_integration {
    use super::*;

    #[test]
    fn test_quality_checker_creation() {
        let checker = QualityChecker::new();
        let _ = checker;
    }

    #[test]
    fn test_quality_presets() {
        let draft = QualityPreset::Draft;
        let standard = QualityPreset::Standard;
        let high = QualityPreset::High;
        let maximum = QualityPreset::Maximum;

        let draft_config = draft.webp_config();
        let standard_config = standard.webp_config();
        let high_config = high.webp_config();
        let maximum_config = maximum.webp_config();

        // Higher presets should have higher quality values
        assert!(standard_config.quality >= draft_config.quality);
        assert!(high_config.quality >= standard_config.quality);
        assert!(maximum_config.quality >= high_config.quality);
    }

    #[test]
    fn test_quality_preset_formats() {
        let preset = QualityPreset::High;

        let webp = preset.webp_config();
        let avif = preset.avif_config();
        let jxl = preset.jxl_config();

        assert!(webp.quality > 0);
        assert!(avif.quality > 0);
        assert!(jxl.quality > 0);
    }

    #[test]
    fn test_visual_quality_scorer_creation() {
        let scorer = VisualQualityScorer::new();
        let _ = scorer;
    }
}

// ============================================================
// Format Knowledge Integration Tests
// ============================================================

mod format_integration {
    use super::*;

    #[test]
    fn test_format_knowledge_base_creation() {
        let kb = FormatKnowledgeBase::new();

        let webp = kb.get_format("webp");
        assert!(webp.is_some());

        let webp_info = webp.unwrap();
        assert!(webp_info.capabilities.supports_alpha);
        assert!(webp_info.capabilities.supports_animation);
    }

    #[test]
    fn test_format_knowledge_avif() {
        let kb = FormatKnowledgeBase::new();

        let avif = kb.get_format("avif");
        assert!(avif.is_some());

        let avif_info = avif.unwrap();
        assert!(avif_info.capabilities.supports_alpha);
        assert!(avif_info.capabilities.supports_hdr);
    }

    #[test]
    fn test_format_knowledge_jxl() {
        let kb = FormatKnowledgeBase::new();

        let jxl = kb.get_format("jxl");
        assert!(jxl.is_some());

        let jxl_info = jxl.unwrap();
        assert!(jxl_info.capabilities.supports_lossless);
    }

    #[test]
    fn test_list_all_formats() {
        let kb = FormatKnowledgeBase::new();
        let formats = kb.list_all_formats();

        assert!(!formats.is_empty());
    }

    #[test]
    fn test_format_knowledge_unknown() {
        let kb = FormatKnowledgeBase::new();
        let knowledge = kb.get_format("unknown_format_xyz");
        assert!(knowledge.is_none());
    }
}

// ============================================================
// Validation Integration Tests
// ============================================================

mod validation_integration {
    #[test]
    fn test_validation_level_ordering() {
        use pixly_kernel::utils::validation::ValidationLevel;

        assert!(ValidationLevel::Deep > ValidationLevel::Format);
        assert!(ValidationLevel::Format > ValidationLevel::Basic);
    }
}

// ============================================================
// Codec Integration Tests
// ============================================================

mod codec_integration {
    use super::*;

    #[test]
    fn test_modern_format_converter_creation() {
        let converter = ModernFormatConverter::new();
        let support = converter.check_format_support();

        // WebP should always be supported
        assert!(support.webp);
    }

    #[test]
    fn test_avif_params_from_quality() {
        let high_quality = AVIFParams::from_quality(95);
        let low_quality = AVIFParams::from_quality(50);

        // Higher quality should have lower CRF
        assert!(high_quality.crf < low_quality.crf);
    }

    #[test]
    fn test_jxl_params_from_quality() {
        let high_quality = JXLParams::from_quality(95);
        let low_quality = JXLParams::from_quality(50);

        // Higher quality should have higher effort
        assert!(high_quality.effort >= low_quality.effort);

        // Very high quality might trigger lossless
        assert!(high_quality.lossless || high_quality.quality >= 95);
    }

    #[test]
    fn test_video_codec_selection() {
        let h264 = VideoCodec::H264;
        let h265 = VideoCodec::H265;
        let av1 = VideoCodec::AV1;
        let vp9 = VideoCodec::VP9;

        assert!(h264 != h265);
        assert!(h265 != av1);
        assert!(av1 != vp9);
    }

    #[test]
    fn test_invalid_quality_clamping() {
        let params = AVIFParams::from_quality(150);
        assert!(params.crf <= 63);

        let params = AVIFParams::from_quality(0);
        assert!(params.crf <= 63);
    }
}

// ============================================================
// Utility Integration Tests
// ============================================================

mod utils_integration {
    use super::*;

    #[test]
    fn test_i18n_messages() {
        let messages = I18nMessages::new();

        let msg = messages.get(MessageKey::FileNotExist);
        assert!(!msg.is_empty());
    }

    #[test]
    fn test_feature_toggles() {
        let toggles = FeatureToggles::default();
        let has_ai = toggles.has_ai_features();
        let _ = has_ai;
    }

    #[test]
    fn test_config_manager_defaults() {
        let manager = ConfigManager::new();
        let config = manager.config();

        assert!(config.defaults.quality > 0 && config.defaults.quality <= 100);
    }
}

// ============================================================
// Core Engine Integration Tests
// ============================================================

mod core_integration {
    use super::*;

    #[test]
    fn test_unified_conversion_engine_creation() {
        let config = UnifiedConversionConfig::default();
        let engine = UnifiedConversionEngine::new(config);
        assert!(engine.is_ok());
    }

    #[test]
    fn test_unified_conversion_config_defaults() {
        let config = UnifiedConversionConfig::default();

        assert!(config.target_quality_threshold > 0.0);
        assert!(config.max_retries > 0);
    }
}

// ============================================================
// Operations Integration Tests
// ============================================================

mod operations_integration {
    use super::*;

    #[test]
    fn test_color_quantizer_creation() {
        let config = QuantizationConfig::default();
        let quantizer = ColorQuantizer::new(config);
        let _ = quantizer;
    }

    #[test]
    fn test_simd_processor_creation() {
        let processor = SIMDProcessor::new();
        let _ = processor;
    }
}

// ============================================================
// Cross-Module Integration Tests
// ============================================================

mod cross_module {
    use super::*;

    #[test]
    fn test_quality_preset_to_params() {
        let presets = [
            QualityPreset::Draft,
            QualityPreset::Standard,
            QualityPreset::High,
            QualityPreset::Maximum,
        ];

        for preset in &presets {
            let config = preset.webp_config();

            let avif_params = AVIFParams::from_quality(config.quality as u8);
            assert!(avif_params.crf <= 63);

            let jxl_params = JXLParams::from_quality(config.quality as u8);
            assert!(jxl_params.effort <= 9);
        }
    }

    #[test]
    fn test_format_knowledge_to_codec_params() {
        let kb = FormatKnowledgeBase::new();
        let formats = ["webp", "avif", "jxl"];

        for format in &formats {
            let knowledge = kb.get_format(format);
            assert!(knowledge.is_some(), "Format {} should have knowledge", format);

            let info = knowledge.unwrap();

            if info.capabilities.supports_lossless {
                match *format {
                    "jxl" => {
                        let params = JXLParams::from_quality(100);
                        assert!(params.lossless || params.quality >= 95);
                    }
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn test_preset_config_for_different_formats() {
        let preset = QualityPreset::High;

        let webp_config = preset.config_for_format("webp");
        let avif_config = preset.config_for_format("avif");
        let jxl_config = preset.config_for_format("jxl");

        assert!(webp_config.quality > 0);
        assert!(avif_config.quality > 0);
        assert!(jxl_config.quality > 0);

        assert!(webp_config.effort > 0);
        assert!(avif_config.effort > 0);
        assert!(jxl_config.effort > 0);
    }
}
