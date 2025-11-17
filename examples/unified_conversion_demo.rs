// 🎯 统一转换引擎演示
// 展示PPO模型、现代格式、质量评估的完整工作流程

use anyhow::Result;
use std::path::PathBuf;

// 注意：需要在Cargo.toml中添加这些依赖
// 这里假设已经在lib.rs中导出了这些模块

fn main() -> Result<()> {
    println!("🚀 Unified Conversion Engine Demo");
    println!("=" .repeat(60));
    
    // 配置
    let config = pixly::UnifiedConversionConfig {
        use_ppo: true,
        use_quality_assessment: true,
        target_quality_threshold: 70.0,
        max_retries: 3,
        ppo_model_path: Some(PathBuf::from("models/ppo_training_all_media_20251117_003038.json")),
    };
    
    // 创建引擎
    println!("\n📦 Initializing conversion engine...");
    let engine = pixly::UnifiedConversionEngine::new(config)?;
    
    // 显示引擎信息
    println!("\n{}", engine.get_info());
    
    // 示例1: 图像转换 (WebP)
    println!("\n" + &"=".repeat(60));
    println!("📸 Example 1: Image Conversion (WebP)");
    println!("=" .repeat(60));
    
    let image_request = pixly::ConversionRequest {
        input_path: PathBuf::from("test_data/sample.jpg"),
        output_path: PathBuf::from("/tmp/output.webp"),
        target_format: "webp".to_string(),
        media_type: pixly::MediaType::Image,
        quality: None, // 使用PPO自动预测
    };
    
    if image_request.input_path.exists() {
        match engine.convert(image_request) {
            Ok(result) => {
                println!("\n✅ Conversion successful!");
                println!("   Input size: {} bytes", result.input_size);
                println!("   Output size: {} bytes", result.output_size);
                println!("   Compression ratio: {:.2}%", result.compression_ratio * 100.0);
                println!("   Used PPO: {}", result.used_ppo);
                println!("   Retries: {}", result.retries);
                
                if let Some(metrics) = result.quality_metrics {
                    println!("\n📊 Quality Assessment:");
                    println!("{}", metrics.detailed_report());
                }
            }
            Err(e) => {
                println!("❌ Conversion failed: {}", e);
            }
        }
    } else {
        println!("⚠️  Test file does not exist, skipping");
    }
    
    // 示例2: 图像转换 (AVIF)
    println!("\n" + &"=".repeat(60));
    println!("🎨 Example 2: Image Conversion (AVIF - Next-Gen Format)");
    println!("=" .repeat(60));
    
    let avif_request = pixly::ConversionRequest {
        input_path: PathBuf::from("test_data/sample.jpg"),
        output_path: PathBuf::from("/tmp/output.avif"),
        target_format: "avif".to_string(),
        media_type: pixly::MediaType::Image,
        quality: None,
    };
    
    if avif_request.input_path.exists() {
        match engine.convert(avif_request) {
            Ok(result) => {
                println!("\n✅ AVIF conversion successful!");
                println!("   Compression ratio: {:.2}%", result.compression_ratio * 100.0);
                
                if let Some(metrics) = result.quality_metrics {
                    println!("   Quality score: {:.1}/100", metrics.overall_score);
                }
            }
            Err(e) => {
                println!("❌ AVIF conversion failed: {}", e);
            }
        }
    }
    
    // 示例3: 视频转换
    println!("\n" + &"=".repeat(60));
    println!("🎬 Example 3: Video Conversion (WebM)");
    println!("=" .repeat(60));
    
    let video_request = pixly::ConversionRequest {
        input_path: PathBuf::from("test_data/sample.mp4"),
        output_path: PathBuf::from("/tmp/output.webm"),
        target_format: "webm".to_string(),
        media_type: pixly::MediaType::Video,
        quality: None,
    };
    
    if video_request.input_path.exists() {
        match engine.convert(video_request) {
            Ok(result) => {
                println!("\n✅ Video conversion successful!");
                println!("   Compression ratio: {:.2}%", result.compression_ratio * 100.0);
                
                if let Some(metrics) = result.quality_metrics {
                    if let Some(vmaf) = metrics.vmaf {
                        println!("   VMAF score: {:.1}/100", vmaf);
                    }
                }
            }
            Err(e) => {
                println!("❌ Video conversion failed: {}", e);
            }
        }
    } else {
        println!("⚠️  Test video does not exist, skipping");
    }
    
    // 示例4: 音频转换
    println!("\n" + &"=".repeat(60));
    println!("🎵 Example 4: Audio Conversion (AAC)");
    println!("=" .repeat(60));
    
    let audio_request = pixly::ConversionRequest {
        input_path: PathBuf::from("test_data/sample.mp3"),
        output_path: PathBuf::from("/tmp/output.aac"),
        target_format: "aac".to_string(),
        media_type: pixly::MediaType::Audio,
        quality: None,
    };
    
    if audio_request.input_path.exists() {
        match engine.convert(audio_request) {
            Ok(result) => {
                println!("\n✅ Audio conversion successful!");
                println!("   Compression ratio: {:.2}%", result.compression_ratio * 100.0);
                
                if let Some(metrics) = result.quality_metrics {
                    if let Some(pesq) = metrics.pesq {
                        println!("   PESQ score: {:.2}/5.0", pesq);
                    }
                }
            }
            Err(e) => {
                println!("❌ Audio conversion failed: {}", e);
            }
        }
    } else {
        println!("⚠️  Test audio does not exist, skipping");
    }
    
    // 示例5: 格式推荐
    println!("\n" + &"=".repeat(60));
    println!("💡 Example 5: Smart Format Recommendation");
    println!("=" .repeat(60));
    
    println!("\nRecommendations based on PPO training data:");
    println!("   Best image format: {}", engine.recommend_format(pixly::MediaType::Image));
    println!("   Best video format: {}", engine.recommend_format(pixly::MediaType::Video));
    println!("   Best audio format: {}", engine.recommend_format(pixly::MediaType::Audio));
    
    println!("\n" + &"=".repeat(60));
    println!("🎉 Demo completed!");
    println!("=" .repeat(60));
    
    Ok(())
}
