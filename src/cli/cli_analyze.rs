// 🔍 CLI Analyzecommand - AI驱动全媒体analysis
// support图片、video、audiointelligentanalysis and parameterrecommended

use std::path::Path;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use crate::analysis::media_analyzer::MediaAnalyzer;

#[derive(Debug, Clone)]
#[derive(Default)]
pub structure AnalyzeOptions {
 pub use_ai: bool, // whetherusingAIrecommended
 pub json_output: bool, // whether输出JSON格式
 pub target_format: Option<String>, // target格式（可选）
}


/// analysisresult（JSONoutput）
#[derive(Debug, Serialize, Deserialize)]
pub structure AnalysisResult {
 pub media_type: String,
 pub features: Vec<f64>, // 🔥 128维特征向量
 pub basic_info: BasicInfo,
 pub recommendation: Option<Recommendation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub structure BasicInfo {
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
pub structure Recommendation {
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
 
 // 1. use Media Analyzeranalysisfile
 let analyzer = MediaAnalyzer::new();
 let media_info = analyzer.analyze(input_path)
 .context("Failed to analyze media file")?;
 
 // 2. extractionfeature
 let (has_alpha, complexity) = detect_image_features(input_path)?;
 
 // buildBasicInfo
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
 
 // 3. extraction128dimensionalfeature向量
 // needLoad image
 let img = image::open(input_path)
 .context("Failed to load image for feature extraction")?;
 
 // conversionBasicInfoforImageFeatures
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
 
 // 4. AIrecommended（ifenabled）
 let recommendation = if options.use_ai {
 Some(get_ai_recommendation(&media_info, &basic_info, &feature_vector)?)
 } else {
 None
 };
 
 // 5. outputresult
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
 // JSONformatoutput（供JSparse）
 let json = serde_json::to_string_pretty(&result)?;
 println!("{}", json);
 } else {
 // 人类可读format
 print_human_readable(&result);
 }
 
 Ok(())
}

/// getAIrecommendedparameter
/// 
/// ✅ usereal MLSystem进lineprediction
/// - formatrecommended: src/format_recommender.rs
/// - UnifiedAIinterface: src/ai_interface.rs
fn get_ai_recommendation(media_info: &crate::analysis::media_analyzer::MediaInfo, basic_info: &BasicInfo, _features: &Vec<f64>) -> Result<Recommendation> {
 use crate::utils::format_recommender::{AIFormatRecommender, UserPreferences};
 use crate::utils::format_selector::FormatSelector; // 🔥 Phase 4: integrated智能格式选择
 
 // 🤖 usereal AIformatrecommended
 log::info!("🤖 Using AI-powered format recommendation...");
 
 // 🔥 Phase 4: 先use Format Selectorvalidationformatselect
 let selector = FormatSelector::new(false);
 let input_path = Path::new(&media_info.path);
 let format_recommendation = selector.select_best_format(input_path, None)
 .context("Format selection failed")?;
 
 log::info!("🎯 Smart format selection: {} (confidence: {:.0}%)", 
 format_recommendation.recommended_format.to_uppercase(),
 format_recommendation.confidence * 100.0);
 log::info!(" Reason: {}", format_recommendation.reason);
 
 // conversionfor Image Features (usestandardstructure)
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
 
 // create AIrecommended
 let recommender = AIFormatRecommender::new();
 let user_prefs = UserPreferences::default();
 
 // get最佳recommended（use Format Selectorsuggested）
 let best_recommendation = recommender.get_best_recommendation(
 &image_features,
 QualityMode::Balanced,
 &user_prefs
 ).context("AI recommendation failed")?;
 
 // 🔥 Phase 4: priorityuse Format Selectorsuggested（if置信度high）
 let final_format = if format_recommendation.confidence > 0.7 {
 format_recommendation.recommended_format.clone()
 } else {
 best_recommendation.format.clone()
 };
 
 log::info!("✅ Final recommendation: {} (AI confidence: {:.0}%)", 
 final_format.to_uppercase(), 
 best_recommendation.confidence * 100.0);
 
 // 🔥 Phase 4: usefinal_format而 not isbest_recommendation.format
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
 
 // format预估size
 let estimated_size = if best_recommendation.estimated_size < 1024 * 1024 {
 format!("{:.1} KB", best_recommendation.estimated_size as f64 / 1024.0)
 } else {
 format!("{:.1} MB", best_recommendation.estimated_size as f64 / (1024.0 * 1024.0))
 };
 
 Ok(Recommendation {
 format: final_format, // 🔥 Phase 4: using智能选择格式
 params,
 estimated_size,
 size_reduction: best_recommendation.space_saving,
 quality_score: format!("{}/100", best_recommendation.quality_score),
 confidence: best_recommendation.confidence,
 })
}

/// 🔥 detectionimagefeature（transparency度 and 复杂度）
/// 
/// useimagelibrary进linerealimageanalysis + 边缘detectioncalculation复杂度
fn detect_image_features(path: &Path) -> Result<(bool, f64)> {
 // try打开image
 let img = match image::open(path) {
 Ok(img) => img,
 Err(e) => {
 // ifno法打开（mayisvideooraudio），returndefaultvalue
 eprintln!("⚠️ Cannot open as image ({}), using defaults", e);
 return Ok((false, 0.5));
 }
 };
 
 // detectiontransparency度（realdetection）
 let has_alpha = img.color().has_alpha();
 
 // 🔥 real复杂度calculation：based on边缘detection
 let complexity = calculate_image_complexity(&img);
 
 Ok((has_alpha, complexity))
}

/// 🔥 realimage复杂度calculation
/// 
/// use Sobel边缘detectionalgorithmcalculationimage复杂度
/// 
/// method：
/// 1. conversionfor灰度图
/// 2. samplingpixel（避免processing整大图）
/// 3. calculation边缘strength
/// 4. normalizeto0-1range
fn calculate_image_complexity(img: &image::DynamicImage) -> f64 {
 use image::GenericImageView;
 
 let (width, height) = img.dimensions();
 let gray = img.to_luma8();
 
 // samplingstrategy：大图sampling，小图全扫描
 let sample_rate = if width * height > 1_000_000 {
 10 // 大图every10pixelsampling1
 } else {
 1 // 小图全扫描
 };
 
 let mut edge_count = 0;
 let mut sample_count = 0;
 
 // Sobel边缘detection（简版）
 for y in (1..height-1).step_by(sample_rate as usize) {
 for x in (1..width-1).step_by(sample_rate as usize) {
 // calculation水平 and 垂直梯度
 let gx = (gray.get_pixel(x+1, y)[0] as i32 - gray.get_pixel(x-1, y)[0] as i32).abs();
 let gy = (gray.get_pixel(x, y+1)[0] as i32 - gray.get_pixel(x, y-1)[0] as i32).abs();
 
 let gradient = ((gx * gx + gy * gy) as f64).sqrt();
 
 // 边缘threshold：梯度 > 30 认foris边缘
 if gradient > 30.0 {
 edge_count += 1;
 }
 
 sample_count += 1;
 }
 }
 
 // calculation边缘密度
 let edge_density = if sample_count > 0 {
 edge_count as f64 / sample_count as f64
 } else {
 0.0
 };
 
 // normalizeto0-1range
 // 经验value：边缘密度 > 0.3 认forishigh复杂度
 (edge_density * 3.0).min(1.0)
}

/// print人类可读format
fn print_human_readable(result: &AnalysisResult) {
 println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
 println!("🔍 PIXLY AI Media Analysis");
 println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
 
 println!("📁 Media Type: {}", result.media_type);
 println!("\n📊 Basic Info:");
 println!(" Resolution: {}x{}", result.basic_info.width, result.basic_info.height);
 println!(" File Size: {:.2} MB", result.basic_info.file_size as f64 / (1024.0 * 1024.0));
 println!(" Format: {}", result.basic_info.format);
 println!(" Animated: {}", if result.basic_info.is_animated { "Yes" } else { "No" });
 println!(" Transparent: {}", if result.basic_info.has_alpha { "Yes" } else { "No" });
 println!(" Complexity: {:.2}", result.basic_info.complexity);
 
 // 🔥 Phase 3.1: output128dimensionalfeature向量（供PPOtraininguse）
 println!("\n🧬 Features (128-dim):");
 println!(" [{}]", result.features.iter()
 .map(|f| format!("{:.6}", f))
 .collect::<Vec<_>>()
 .join(", "));
 
 if let Some(rec) = &result.recommendation {
 println!("\n🤖 AI Recommendation:");
 println!(" Format: {}", rec.format.to_uppercase());
 println!(" Parameters: {}", rec.params);
 println!(" Estimated Size: {}", rec.estimated_size);
 println!(" Size Reduction: {:.1}%", rec.size_reduction);
 println!(" Quality Score: {}", rec.quality_score);
 println!(" Confidence: {:.0}%", rec.confidence * 100.0);
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
