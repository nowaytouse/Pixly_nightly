// 🔍 CLI Analyze命令 - AI驱动的全媒体分析
// 支持图片、视频、音频的智能分析和参数推荐

use std::path::Path;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use crate::analysis::media_analyzer::MediaAnalyzer;

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct AnalyzeOptions {
    pub use_ai: bool,        // 是否使用AI推荐
    pub json_output: bool,   // 是否输出JSON格式
    pub target_format: Option<String>,  // 目标格式（可选）
}


/// 分析结果（JSON输出）
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub media_type: String,
    pub features: Vec<f64>,  // 🔥 128维特征向量
    pub basic_info: BasicInfo,
    pub recommendation: Option<Recommendation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicInfo {
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
    let (has_alpha, complexity) = detect_image_features(input_path)?;
    
    // 构建BasicInfo
    let basic_info = BasicInfo {
        width: media_info.resolution.0,
        height: media_info.resolution.1,
        file_size: media_info.size,
        format: media_info.format.clone(),
        is_animated: media_info.media_type == crate::analysis::media_analyzer::MediaType::Animation,
        has_alpha,
        frame_count: media_info.frame_count.unwrap_or(1),
        duration: media_info.duration.unwrap_or(0.0) as f64,
        complexity,
    };
    
    // 3. 提取128维特征向量
    // 需要加载图像
    let img = image::open(input_path)
        .context("Failed to load image for feature extraction")?;
    
    // 转换BasicInfo为ImageFeatures
    let image_features = crate::ImageFeatures {
        width: basic_info.width,
        height: basic_info.height,
        file_size: basic_info.file_size,
        format: basic_info.format.clone(),
        has_alpha: basic_info.has_alpha,
        is_animated: basic_info.is_animated,
        complexity: basic_info.complexity,
    };
    
    let feature_vector = crate::core::feature_extractor_128d::extract_128d_features(
        &img,
        input_path,
        &image_features
    );
    
    // 4. AI推荐（如果启用）
    let recommendation = if options.use_ai {
        Some(get_ai_recommendation(&media_info, &basic_info, &feature_vector)?)
    } else {
        None
    };
    
    // 5. 输出结果
    let media_type_str = match media_info.media_type {
        crate::analysis::media_analyzer::MediaType::Image => "image",
        crate::analysis::media_analyzer::MediaType::Animation => "animation",
        crate::analysis::media_analyzer::MediaType::Video => "video",
        crate::analysis::media_analyzer::MediaType::Audio => "audio",
        crate::analysis::media_analyzer::MediaType::Unknown => "unknown",
    };
    
    let result = AnalysisResult {
        media_type: media_type_str.to_string(),
        features: feature_vector,
        basic_info,
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
fn get_ai_recommendation(media_info: &crate::analysis::media_analyzer::MediaInfo, basic_info: &BasicInfo, _features: &Vec<f64>) -> Result<Recommendation> {
    use crate::utils::format_recommender::{AIFormatRecommender, UserPreferences};
    use crate::utils::format_selector::FormatSelector; // 🔥 Phase 4: 集成智能格式选择
    
    // 🤖 使用真实的AI格式推荐器
    eprintln!("🤖 Using AI-powered format recommendation...");
    
    // 🔥 Phase 4: 先使用FormatSelector验证格式选择
    let selector = FormatSelector::new(false);
    let input_path = Path::new(&media_info.path);
    let format_recommendation = selector.select_best_format(input_path, None)
        .context("Format selection failed")?;
    
    eprintln!("🎯 Smart format selection: {} (confidence: {:.0}%)", 
              format_recommendation.recommended_format.to_uppercase(),
              format_recommendation.confidence * 100.0);
    eprintln!("   Reason: {}", format_recommendation.reason);
    
    // 转换为ImageFeatures (使用标准结构)
    use crate::{ImageFeatures, QualityMode};
    
    let image_features = ImageFeatures {
        width: basic_info.width,
        height: basic_info.height,
        file_size: basic_info.file_size,
        format: basic_info.format.clone(),
        has_alpha: basic_info.has_alpha,
        is_animated: basic_info.is_animated,
        complexity: basic_info.complexity,
    };
    
    // 创建AI推荐器
    let recommender = AIFormatRecommender::new();
    let user_prefs = UserPreferences::default();
    
    // 获取最佳推荐（使用FormatSelector的建议）
    let best_recommendation = recommender.get_best_recommendation(
        &image_features,
        QualityMode::Balanced,
        &user_prefs
    ).context("AI recommendation failed")?;
    
    // 🔥 Phase 4: 优先使用FormatSelector的建议（如果置信度高）
    let final_format = if format_recommendation.confidence > 0.7 {
        format_recommendation.recommended_format.clone()
    } else {
        best_recommendation.format.clone()
    };
    
    eprintln!("✅ Final recommendation: {} (AI confidence: {:.0}%)", 
              final_format.to_uppercase(), 
              best_recommendation.confidence * 100.0);
    
    // 🔥 Phase 4: 使用final_format而不是best_recommendation.format
    let params = match final_format.as_str() {
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
        format: final_format, // 🔥 Phase 4: 使用智能选择的格式
        params,
        estimated_size,
        size_reduction: best_recommendation.space_saving,
        quality_score: format!("{}/100", best_recommendation.quality_score),
        confidence: best_recommendation.confidence,
    })
}

/// 🔥 检测图像特征（透明度和复杂度）
/// 
/// 使用image库进行真实的图像分析 + 边缘检测计算复杂度
fn detect_image_features(path: &Path) -> Result<(bool, f64)> {
    // 尝试打开图像
    let img = match image::open(path) {
        Ok(img) => img,
        Err(e) => {
            // 如果无法打开（可能是视频或音频），返回默认值
            eprintln!("⚠️  Cannot open as image ({}), using defaults", e);
            return Ok((false, 0.5));
        }
    };
    
    // 检测透明度（真实检测）
    let has_alpha = img.color().has_alpha();
    
    // 🔥 真实的复杂度计算：基于边缘检测
    let complexity = calculate_image_complexity(&img);
    
    Ok((has_alpha, complexity))
}

/// 🔥 真实的图像复杂度计算
/// 
/// 使用Sobel边缘检测算法计算图像复杂度
/// 
/// 方法：
/// 1. 转换为灰度图
/// 2. 采样像素（避免处理整个大图）
/// 3. 计算边缘强度
/// 4. 归一化到0-1范围
fn calculate_image_complexity(img: &image::DynamicImage) -> f64 {
    use image::GenericImageView;
    
    let (width, height) = img.dimensions();
    let gray = img.to_luma8();
    
    // 采样策略：大图采样，小图全扫描
    let sample_rate = if width * height > 1_000_000 {
        10 // 大图每10个像素采样1个
    } else {
        1  // 小图全扫描
    };
    
    let mut edge_count = 0;
    let mut sample_count = 0;
    
    // Sobel边缘检测（简化版）
    for y in (1..height-1).step_by(sample_rate as usize) {
        for x in (1..width-1).step_by(sample_rate as usize) {
            // 计算水平和垂直梯度
            let gx = (gray.get_pixel(x+1, y)[0] as i32 - gray.get_pixel(x-1, y)[0] as i32).abs();
            let gy = (gray.get_pixel(x, y+1)[0] as i32 - gray.get_pixel(x, y-1)[0] as i32).abs();
            
            let gradient = ((gx * gx + gy * gy) as f64).sqrt();
            
            // 边缘阈值：梯度 > 30 认为是边缘
            if gradient > 30.0 {
                edge_count += 1;
            }
            
            sample_count += 1;
        }
    }
    
    // 计算边缘密度
    let edge_density = if sample_count > 0 {
        edge_count as f64 / sample_count as f64
    } else {
        0.0
    };
    
    // 归一化到0-1范围
    // 经验值：边缘密度 > 0.3 认为是高复杂度
    (edge_density * 3.0).min(1.0)
}

/// 打印人类可读格式
fn print_human_readable(result: &AnalysisResult) {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔍 PIXLY AI Media Analysis");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    println!("📁 Media Type: {}", result.media_type);
    println!("\n📊 Basic Info:");
    println!("   Resolution: {}x{}", result.basic_info.width, result.basic_info.height);
    println!("   File Size: {:.2} MB", result.basic_info.file_size as f64 / (1024.0 * 1024.0));
    println!("   Format: {}", result.basic_info.format);
    println!("   Animated: {}", if result.basic_info.is_animated { "Yes" } else { "No" });
    println!("   Transparent: {}", if result.basic_info.has_alpha { "Yes" } else { "No" });
    println!("   Complexity: {:.2}", result.basic_info.complexity);
    
    // 🔥 Phase 3.1: 输出128维特征向量（供PPO训练使用）
    println!("\n🧬 Features (128-dim):");
    println!("   [{}]", result.features.iter()
        .map(|f| format!("{:.6}", f))
        .collect::<Vec<_>>()
        .join(", "));
    
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
