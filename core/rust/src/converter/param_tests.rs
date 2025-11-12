/**
 * Parameter optimization tests
 */

use crate::converter::params::{ImageCharacteristics, AIParameterClient, OptimizedParams};

#[test]
fn test_avif_optimization() {
    let chars = ImageCharacteristics {
        width: 1920,
        height: 1080,
        file_size: 2_000_000,
        format: "png".to_string(),
        has_alpha: false,
        is_animated: false,
        complexity: 0.5,
        path: None,
    };

    let optimizer = AIParameterClient::new("avif");
    let params = optimizer.get_parameters_from_ai(&chars).unwrap();

    assert!(params.quality >= 75 && params.quality <= 95);
    assert!(params.speed >= 2 && params.speed <= 8);
}

#[test]
fn test_jxl_lossless_jpeg() {
    let chars = ImageCharacteristics {
        width: 1920,
        height: 1080,
        file_size: 500_000,
        format: "jpeg".to_string(),
        has_alpha: false,
        is_animated: false,
        complexity: 0.5,
        path: None,
    };

    let optimizer = AIParameterClient::new("jxl");
    let params = optimizer.get_parameters_from_ai(&chars).unwrap();

    assert_eq!(params.quality, 100);
    assert!(params.lossless);
    assert!(params.format_options.iter().any(|(k, v)| k == "lossless-jpeg" && v == "1"));
}

#[test]
fn test_webp_optimization() {
    let chars = ImageCharacteristics {
        width: 1920,
        height: 1080,
        file_size: 500_000,
        format: "png".to_string(),
        has_alpha: false,
        is_animated: false,
        complexity: 0.4,
        path: None,
    };

    let optimizer = AIParameterClient::new("webp");
    let params = optimizer.get_parameters_from_ai(&chars).unwrap();

    assert!(params.quality >= 80 && params.quality <= 95);
    assert!(params.format_options.iter().any(|(k, v)| k == "method" && v == "6"));
}

#[test]
fn test_png_optimization() {
    let chars = ImageCharacteristics {
        width: 1920,
        height: 1080,
        file_size: 2_000_000,
        format: "png".to_string(),
        has_alpha: true,
        is_animated: false,
        complexity: 0.7,
        path: None,
    };

    let optimizer = AIParameterClient::new("png");
    let params = optimizer.get_parameters_from_ai(&chars).unwrap();

    assert_eq!(params.quality, 100);
    assert!(params.lossless);
}

#[test]
fn test_small_image_optimization() {
    let chars = ImageCharacteristics {
        width: 800,
        height: 600,
        file_size: 100_000,
        format: "png".to_string(),
        has_alpha: false,
        is_animated: false,
        complexity: 0.3,
        path: None,
    };

    let optimizer = AIParameterClient::new("avif");
    let params = optimizer.get_parameters_from_ai(&chars).unwrap();

    // Small images should get higher quality
    assert!(params.quality >= 80);
    assert!(params.estimated_size > 0);
}

#[test]
fn test_large_image_optimization() {
    let chars = ImageCharacteristics {
        width: 3840,
        height: 2160,
        file_size: 10_000_000,
        format: "png".to_string(),
        has_alpha: false,
        is_animated: false,
        complexity: 0.8,
        path: None,
    };

    let optimizer = AIParameterClient::new("avif");
    let params = optimizer.get_parameters_from_ai(&chars).unwrap();

    // Large images should prioritize compression
    assert!(params.speed >= 4);
    assert!(params.estimated_ratio < 0.8);
}

#[test]
fn test_alpha_channel_handling() {
    let chars = ImageCharacteristics {
        width: 1920,
        height: 1080,
        file_size: 1_500_000,
        format: "png".to_string(),
        has_alpha: true,
        is_animated: false,
        complexity: 0.5,
        path: None,
    };

    let optimizer = AIParameterClient::new("avif");
    let params = optimizer.get_parameters_from_ai(&chars).unwrap();

    // Alpha channel should increase quality
    assert!(params.quality >= 80);
    assert!(params.format_options.iter().any(|(k, _)| k == "alpha-quality"));
}

#[test]
fn test_high_complexity_image() {
    let chars = ImageCharacteristics {
        width: 1920,
        height: 1080,
        file_size: 3_000_000,
        format: "png".to_string(),
        has_alpha: false,
        is_animated: false,
        complexity: 0.9,
        path: None,
    };

    let optimizer = AIParameterClient::new("avif");
    let params = optimizer.get_parameters_from_ai(&chars).unwrap();

    // High complexity should maintain quality
    assert!(params.quality >= 80);
}

#[test]
fn test_jpeg_format_detection() {
    let chars = ImageCharacteristics {
        width: 1920,
        height: 1080,
        file_size: 500_000,
        format: "jpg".to_string(),
        has_alpha: false,
        is_animated: false,
        complexity: 0.5,
        path: None,
    };

    let optimizer = AIParameterClient::new("jxl");
    let params = optimizer.get_parameters_from_ai(&chars).unwrap();

    // JPEG to JXL should use lossless transcoding
    assert_eq!(params.quality, 100);
    assert!(params.lossless);
}

#[test]
fn test_optimization_params_builder() {
    let params = OptimizedParams {
        quality: 85,
        speed: 4,
        lossless: false,
        format_options: vec![
            ("tune".to_string(), "ssim".to_string()),
        ],
        estimated_size: 1_000_000,
        estimated_ratio: 0.5,
        reason: "Test optimization".to_string(),
    };

    assert_eq!(params.quality, 85);
    assert_eq!(params.speed, 4);
    assert!(!params.lossless);
    assert_eq!(params.estimated_size, 1_000_000);
    assert!((params.estimated_ratio - 0.5).abs() < 0.01);
}
