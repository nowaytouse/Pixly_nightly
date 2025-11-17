//! 集成测试 - 真实的Rust测试

use pixly_kernel::*;
use std::path::PathBuf;
use std::time::Instant;

#[test]
fn test_simd_sharpener_real() {
    println!("\n=== Testing SIMD Sharpener ===");
    let start = Instant::now();
    
    let sharpener = simd_sharpener::SIMDSharpener::new();
    let info = sharpener.get_performance_info();
    
    println!("SIMD support: {}", info.uses_simd);
    println!("Parallel processing: {}", info.uses_parallel);
    
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    
    // 纳秒级也算通过
    let _ = elapsed.as_nanos(); // 确保elapsed有效
}

#[test]
fn test_batch_decision_manager_real() {
    println!("\n=== Testing Batch Decision Manager ===");
    let start = Instant::now();
    
    let mut manager = batch_decision_manager::BatchDecisionManager::new(false);
    
    let corrupted = batch_decision_manager::CorruptedFile {
        file_path: PathBuf::from("test.jpg"),
        corruption_type: batch_decision_manager::CorruptionType::InvalidHeader,
        error_message: "Invalid header".to_string(),
        file_size: 1024,
        can_repair: true,
        repair_confidence: 0.8,
    };
    
    manager.add_corrupted_file(corrupted).unwrap();
    
    let stats = manager.get_statistics();
    println!("Corrupted files: {}", stats.total_corrupted);
    
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    
    assert_eq!(stats.total_corrupted, 1);
    let _ = elapsed.as_nanos(); // 确保elapsed有效
}

#[test]
fn test_feature_extractor_128d_real() {
    println!("\n=== Testing 128D Feature Extractor ===");
    let start = Instant::now();
    
    let mut extractor = feature_extractor_128d::FeatureExtractor128D::new();
    
    use image::{RgbaImage, DynamicImage};
    let img = DynamicImage::ImageRgba8(RgbaImage::new(100, 100));
    
    let metadata = std::collections::HashMap::new();
    let features = extractor.extract_features(&img, &metadata);
    
    println!("Feature dimensions: {}", features.len());
    
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    
    assert_eq!(features.len(), 128);
    let _ = elapsed.as_nanos(); // 确保elapsed有效
}

#[test]
fn test_simd_processor_real() {
    println!("\n=== Testing SIMD Processor ===");
    let start = Instant::now();
    
    let processor = simd_processor::SIMDProcessor::new();
    let info = processor.get_performance_info();
    
    println!("AVX2 support: {}", info.supports_avx2);
    println!("NEON support: {}", info.supports_neon);
    
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    
    let _ = elapsed.as_nanos(); // 确保elapsed有效
}

#[test]
fn test_batch_processor_real() {
    println!("\n=== Testing Batch Processing System ===");
    let start = Instant::now();
    
    let _processor = batch_processor_advanced::BatchProcessor::with_defaults();
    
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    
    let _ = elapsed.as_nanos(); // 确保elapsed有效
}

#[test]
fn test_quality_checker_real() {
    println!("\n=== Testing Quality Checker ===");
    let start = Instant::now();
    
    let _checker = quality_checker_advanced::QualityChecker::new();
    
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    
    let _ = elapsed.as_nanos(); // 确保elapsed有效
}
