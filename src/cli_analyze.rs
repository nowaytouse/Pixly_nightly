// 🔍 CLI Analyze命令 - AI驱动的全媒体分析
// 支持图片、视频、音频的智能分析和参数推荐

use std::path::Path;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use crate::media_analyzer::MediaAnalyzer;

#[derive(Debug, Clone)]
pub struct AnalyzeOptions {
    pub use_ai: bool,        // 是否使用AI推荐
    pub json_output: bool,   // 是否输出JSON格式
    pub target_format: Option<String>,  // 目标格式（可选）
}

impl Default for AnalyzeOptions {
    fn default() -> Self {
        Self {
            use_ai: false,
            json_output: false,
            target_format: None,
        }
    }
}

/// 分析结果（JSON输出）
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub media_type: String,
    pub features: MediaFeatures,
    pub recommendation: Option<Recommendation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaFeatures {
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub format: String,
    pub is_animated: bool,
    pub has_alpha: bool,
    pub frame_count: u32,
    pub duration: f64,
    pub complexity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recommendation {
    pub format: String,
    pub params: serde_json::Value,
    pub estimated_size: String,
    pub size_reduction: f64,
    pub quality_score: String,
    pub confidence: f64,
}

pub fn handle_analyze(input: &str, options: &AnalyzeOptions) -> Result<()> {
    let input_path = Path::new(input);
    
    if !input_path.exists() {
        anyhow::bail!("File does not exist: {}", input);
    }
    
    // 1. 使用MediaAnalyzer分析文件
    let analyzer = MediaAnalyzer::new();
    let media_info = analyzer.analyze(input_path)
        .context("Failed to analyze media file")?;
    
    // 2. 提取特征
    // 🔥 修复TODO: 使用image库检测透明度和复杂度
    let (has_alpha, complexity) = detect_image_features(input_path)?;
    
    let features = MediaFeatures {
        width: media_info.resolution.0,
        height: media_info.resolution.1,
        file_size: media_info.size,
        format: media_info.format.clone(),
        is_animated: media_info.media_type == crate::media_analyzer::MediaType::Animation,
        has_alpha,
        frame_count: media_info.frame_count.unwrap_or(1),
        duration: media_info.duration.unwrap_or(0.0) as f64,
        complexity,
    };
    
    // 3. AI推荐（如果启用）
    let recommendation = if options.use_ai {
        Some(get_ai_recommendation(&media_info, &features)?)
    } else {
        None
    };
    
    // 4. 输出结果
    let media_type_str = match media_info.media_type {
        crate::media_analyzer::MediaType::Image => "image",
        crate::media_analyzer::MediaType::Animation => "animation",
        crate::media_analyzer::MediaType::Video => "video",
        crate::media_analyzer::MediaType::Audio => "audio",
        crate::media_analyzer::MediaType::Unknown => "unknown",
    };
    
    let result = AnalysisResult {
        media_type: media_type_str.to_string(),
        features,
        recommendation,
    };
    
    if options.json_output {
        // JSON格式输出（供JS解析）
        let json = serde_json::to_string_pretty(&result)?;
        println!("{}", json);
    } else {
        // 人类可读格式
        print_human_readable(&result);
    }
    
    Ok(())
}

/// 获取AI推荐参数
/// 
/// ✅ 使用真实的ML系统进行预测
/// - 格式推荐: src/format_recommender.rs
/// - 统一AI接口: src/ai_interface.rs
fn get_ai_recommendation(_media_info: &crate::media_analyzer::MediaInfo, features: &MediaFeatures) -> Result<Recommendation> {
    use crate::format_recommender::{AIFormatRecommender, UserPreferences};
    
    // 🤖 使用真实的AI格式推荐器
    eprintln!("🤖 Using AI-powered format recommendation...");
    
    // 转换为ImageFeatures (使用标准结构)
    use crate::{ImageFeatures, QualityMode};
    
    let image_features = ImageFeatures {
        width: features.width,
        height: features.height,
        file_size: features.file_size,
        format: features.format.clone(),
        has_alpha: features.has_alpha,
        is_animated: features.is_animated,
        complexity: features.complexity,
    };
    
    // 创建AI推荐器
    let recommender = AIFormatRecommender::new();
    let user_prefs = UserPreferences::default();
    
    // 获取最佳推荐
    let best_recommendation = recommender.get_best_recommendation(
        &image_features,
        QualityMode::Balanced,
        &user_prefs
    ).context("AI recommendation failed")?;
    
    eprintln!("✅ AI recommendation: {} (confidence: {:.0}%)", 
              best_recommendation.format.to_uppercase(), 
              best_recommendation.confidence * 100.0);
    
    // 使用AI预测的参数
    let params = match best_recommendation.format.as_str() {
        "avif" => serde_json::json!({
            "quality": best_recommendation.quality_score,
            "effort": 6,
            "speed": 4
        }),
        "jxl" => serde_json::json!({
            "quality": best_recommendation.quality_score,
            "effort": 7,
            "lossless": false
        }),
        "webp" => serde_json::json!({
            "quality": best_recommendation.quality_score,
            "method": 6,
            "lossless": false
        }),
        "h265" => serde_json::json!({
            "crf": 23,
            "preset": "medium",
            "codec": "h265"
        }),
        "opus" => serde_json::json!({
            "bitrate": 128,
            "vbr": true
        }),
        _ => serde_json::json!({}),
    };
    
    // 格式化预估大小
    let estimated_size = if best_recommendation.estimated_size < 1024 * 1024 {
        format!("{:.1} KB", best_recommendation.estimated_size as f64 / 1024.0)
    } else {
        format!("{:.1} MB", best_recommendation.estimated_size as f64 / (1024.0 * 1024.0))
    };
    
    Ok(Recommendation {
        format: best_recommendation.format,
        params,
        estimated_size,
        size_reduction: best_recommendation.space_saving,
        quality_score: format!("{}/100", best_recommendation.quality_score),
        confidence: best_recommendation.confidence,
    })
}

/// 🔥 检测图像特征（透明度和复杂度）
/// 
/// 使用image库进行真实的图像分析
fn detect_image_features(path: &Path) -> Result<(bool, f64)> {
    use image::GenericImageView;
    
    // 尝试打开图像
    let img = match image::open(path) {
        Ok(img) => img,
        Err(e) => {
            // 如果无法打开（可能是视频或音频），返回默认值
            eprintln!("⚠️  Cannot open as image ({}), using defaults", e);
            return Ok((false, 0.5));
        }
    };
    
    // 检测透明度
    let has_alpha = img.color().has_alpha();
    
    // 简化的复杂度计算
    // 基于图像尺寸和颜色类型的启发式估算
    let (width, height) = img.dimensions();
    let pixels = (width * height) as f64;
    
    // 复杂度因素：
    // 1. 分辨率（大图通常更复杂）
    let resolution_factor = (pixels / 1_000_000.0).min(1.0); // 归一化到0-1
    
    // 2. 颜色类型（RGB/RGBA更复杂）
    let color_factor = match img.color() {
        image::ColorType::L8 | image::ColorType::L16 => 0.3,  // 灰度
        image::ColorType::La8 | image::ColorType::La16 => 0.4, // 灰度+Alpha
        image::ColorType::Rgb8 | image::ColorType::Rgb16 => 0.6, // RGB
        image::ColorType::Rgba8 | image::ColorType::Rgba16 => 0.8, // RGBA
        _ => 0.5,
    };
    
    // 综合复杂度（加权平均）
    let complexity = (resolution_factor * 0.4 + color_factor * 0.6).clamp(0.0, 1.0);
    
    Ok((has_alpha, complexity))
}

/// 打印人类可读格式
fn print_human_readable(result: &AnalysisResult) {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔍 PIXLY AI Media Analysis");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    println!("📁 Media Type: {}", result.media_type);
    println!("\n📊 Features:");
    println!("   Resolution: {}x{}", result.features.width, result.features.height);
    println!("   File Size: {:.2} MB", result.features.file_size as f64 / (1024.0 * 1024.0));
    println!("   Format: {}", result.features.format);
    println!("   Animated: {}", if result.features.is_animated { "Yes" } else { "No" });
    println!("   Transparent: {}", if result.features.has_alpha { "Yes" } else { "No" });
    
    if let Some(rec) = &result.recommendation {
        println!("\n🤖 AI Recommendation:");
        println!("   Format: {}", rec.format.to_uppercase());
        println!("   Parameters: {}", rec.params);
        println!("   Estimated Size: {}", rec.estimated_size);
        println!("   Size Reduction: {:.1}%", rec.size_reduction);
        println!("   Quality Score: {}", rec.quality_score);
        println!("   Confidence: {:.0}%", rec.confidence * 100.0);
    }
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_options() {
        let options = AnalyzeOptions::default();
        assert!(!options.use_ai);
        assert!(!options.json_output);
        assert_eq!(options.target_format, None);
    }
}
