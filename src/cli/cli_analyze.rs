// 🔍 CLI Analyzecommand - AImediaanalysis
// support、video、audiointelligentanalysis and parameterrecommended

use std::path::Path;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use crate::analysis::media_analyzer::MediaAnalyzer;

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct AnalyzeOptions {
 pub json: bool,
 /// 🔥 File format from Eagle metadata (for files without extension)
 pub format: Option<String>,
}


/// analysisresult（JSONoutput）
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
 pub media_type: String,
 pub features: Vec<f64>, // 🔥 128dimensionfeature
 pub basic_info: BasicInfo,
 pub recommendation: Option<Recommendation>,
 pub optimization_status: Option<OptimizationStatusInfo>,  // 🆕 优化状态
}

/// 优化状态信息
#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationStatusInfo {
 pub status: String,  // "optimal", "minor", "significant", "critical"
 pub can_skip: bool,
 pub current_size: u64,
 pub predicted_size: u64,
 pub savings_percent: f64,
 pub confidence: f64,
 pub method: String,
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

// 1. use Media Analyzeranalysisfile
 let analyzer = MediaAnalyzer::new();
 // 🔥 传递格式信息给 analyzer（来自 Eagle 元数据）
 let media_info = analyzer.analyze_with_format(input_path, options.format.as_deref())
 .with_context(|| format!("Failed to analyze media file: {:?}", input_path))?;

// 2. extractionfeature
 let (has_alpha, complexity) = detect_image_features(input_path)
 .with_context(|| format!("Failed to detect image features: {:?}", input_path))?;

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

// 3. extraction128dimensionalfeature
// JXL/AVIF/HEIC 等现代格式无法直接加载，使用默认特征向量
 let feature_vector = if matches!(basic_info.format.as_str(), "jxl" | "avif" | "heic" | "heif") {
  log::info!("Using default features for {} format", basic_info.format);
  // 返回128维的默认特征向量
  vec![0.5; 128]
 } else {
  // 标准格式可以直接打开
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

  crate::core::feature_extractor_128d::extract_128d_features(
   &img,
   input_path,
   &image_features
  )
 };

// 4. AIrecommended（ifenabled）
 // 🤖 AI recommendation (always enabled for analyze command)
 let recommendation = Some(get_ai_recommendation(&media_info, &basic_info, &feature_vector)?);
 
 // 🆕 5. 优化状态分析
 let optimization_status = analyze_optimization_status(input_path, options.format.as_deref())?;

 // 6. outputresult
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
 optimization_status,  // 🆕 优化状态
 };

 if options.json {
// JSONformatoutput（JSparse）
 let json = serde_json::to_string_pretty(&result)?;
 println!("{}", json);
 } else {
// classcanformat
 print_human_readable(&result);
 }

 Ok(())
}

/// getAIrecommendedparameter
///
/// ✅ usereal MLSystemlineprediction
/// - formatrecommended: src/format_recommender.rs
/// - UnifiedAIinterface: src/ai_interface.rs
fn get_ai_recommendation(media_info: &crate::analysis::media_analyzer::MediaInfo, basic_info: &BasicInfo, _features: &Vec<f64>) -> Result<Recommendation> {
 use crate::utils::format_recommender::{AIFormatRecommender, UserPreferences};
 use crate::utils::format_selector::FormatSelector; // 🔥 Phase 4: integratedintelligentformat

// 🤖 usereal AIformatrecommended
 log::info!("🤖 Using AI-powered format recommendation...");

// 🔥 Phase 4: use Format Selectorvalidationformatselect
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

// getmostrecommended（use Format Selectorsuggested）
 let best_recommendation = recommender.get_best_recommendation(
 &image_features,
 QualityMode::Balanced,
 &user_prefs
 ).context("AI recommendation failed")?;

// 🔥 Phase 4: priorityuse Format Selectorsuggested（ifconfidencehigh）
 let final_format = if format_recommendation.confidence > 0.7 {
 format_recommendation.recommended_format.clone()
 } else {
 best_recommendation.format.clone()
 };

 log::info!("✅ Final recommendation: {} (AI confidence: {:.0}%)",
 final_format.to_uppercase(),
 best_recommendation.confidence * 100.0);

// 🔥 Phase 4: usefinal_formatwhile not isbest_recommendation.format
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

// formatsize
 let estimated_size = if best_recommendation.estimated_size < 1024 * 1024 {
 format!("{:.1} KB", best_recommendation.estimated_size as f64 / 1024.0)
 } else {
 format!("{:.1} MB", best_recommendation.estimated_size as f64 / (1024.0 * 1024.0))
 };

 Ok(Recommendation {
 format: final_format, // 🔥 Phase 4: usingintelligentformat
 params,
 estimated_size,
 size_reduction: best_recommendation.space_saving,
 quality_score: format!("{}/100", best_recommendation.quality_score),
 confidence: best_recommendation.confidence,
 })
}

/// 🔥 detectionimagefeature（transparencydegree and complexity）
///
/// useimagelibrarylinerealimageanalysis + edgedetectioncalculationcomplexity
fn detect_image_features(path: &Path) -> Result<(bool, f64)> {
 // 获取文件扩展名
 let extension = path.extension()
  .and_then(|e| e.to_str())
  .map(|e| e.to_lowercase())
  .unwrap_or_default();

 // JXL/AVIF/HEIC 等现代格式无法用 image crate 打开
 if matches!(extension.as_str(), "jxl" | "avif" | "heic" | "heif") {
  log::info!("Using defaults for {} format (not supported by image crate)", extension);
  return Ok((false, 0.5));
 }

// tryopenimage
 let img = match image::open(path) {
  Ok(img) => img,
  Err(e) => {
// ifnoopen（mayisvideooraudio），returndefaultvalue
   eprintln!("⚠️ Cannot open as image ({}), using defaults", e);
   return Ok((false, 0.5));
  }
 };

// detectiontransparencydegree（realdetection）
 let has_alpha = img.color().has_alpha();

// 🔥 realcomplexitycalculation：based onedgedetection
 let complexity = calculate_image_complexity(&img);

 Ok((has_alpha, complexity))
}

/// 🔥 realimagecomplexitycalculation
///
/// use Sobeledgedetectionalgorithmcalculationimagecomplexity
///
/// method：
/// 1. conversionfordegree
/// 2. samplingpixel（processingwholelarge）
/// 3. calculationedgestrength
/// 4. normalizeto0-1range
fn calculate_image_complexity(img: &image::DynamicImage) -> f64 {
 use image::GenericImageView;

 let (width, height) = img.dimensions();
 let gray = img.to_luma8();

// samplingstrategy：largesampling，small全扫描
 let sample_rate = if width * height > 1_000_000 {
 10 // largeevery10pixelsampling1
 } else {
 1 // small全扫描
 };

 let mut edge_count = 0;
 let mut sample_count = 0;

// Sobeledgedetection（）
 for y in (1..height-1).step_by(sample_rate as usize) {
 for x in (1..width-1).step_by(sample_rate as usize) {
// calculation and gradient
 let gx = (gray.get_pixel(x+1, y)[0] as i32 - gray.get_pixel(x-1, y)[0] as i32).abs();
 let gy = (gray.get_pixel(x, y+1)[0] as i32 - gray.get_pixel(x, y-1)[0] as i32).abs();

 let gradient = ((gx * gx + gy * gy) as f64).sqrt();

// edgethreshold：gradient > 30 forisedge
 if gradient > 30.0 {
 edge_count += 1;
 }

 sample_count += 1;
 }
 }

// calculationedgedensity
 let edge_density = if sample_count > 0 {
 edge_count as f64 / sample_count as f64
 } else {
 0.0
 };

// normalizeto0-1range
// value：edgedensity > 0.3 forishighcomplexity
 (edge_density * 3.0).min(1.0)
}

/// printclasscanformat
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

// 🔥 Phase 3.1: output128dimensionalfeature（PPOtraininguse）
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
 
 // 🆕 显示优化状态
 if let Some(opt) = &result.optimization_status {
  println!("\n📊 Optimization Status:");
  
  let (icon, status_text) = match opt.status.as_str() {
   "optimal" => ("✅", "Already Optimized"),
   "minor" => ("👍", "Minor Improvement Possible"),
   "significant" => ("⚠️", "Significant Improvement Recommended"),
   "critical" => ("❌", "Critical Optimization Needed"),
   _ => ("❓", "Unknown"),
  };
  
  println!(" Status: {} {}", icon, status_text);
  println!(" Current Size: {:.2} MB", opt.current_size as f64 / (1024.0 * 1024.0));
  println!(" Predicted Size: {:.2} MB", opt.predicted_size as f64 / (1024.0 * 1024.0));
  println!(" Potential Savings: {:.1}%", opt.savings_percent);
  println!(" Can Skip: {}", if opt.can_skip { "Yes" } else { "No" });
  println!(" Confidence: {:.0}%", opt.confidence * 100.0);
  println!(" Method: {}", opt.method);
 }
 
 println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

/// 🆕 分析优化状态
fn analyze_optimization_status(path: &Path, format_hint: Option<&str>) -> Result<Option<OptimizationStatusInfo>> {
    use crate::core::optimization_analyzer;
    
    // 调用优化分析器
    match optimization_analyzer::analyze_file(path, false, format_hint) {
        Ok(report) => {
            // 提取状态字符串
            let status_str = match &report.status {
                optimization_analyzer::OptimizationStatus::AlreadyOptimized { .. } => "optimal",
                optimization_analyzer::OptimizationStatus::MinorImprovement { .. } => "minor",
                optimization_analyzer::OptimizationStatus::SignificantImprovement { .. } => "significant",
                optimization_analyzer::OptimizationStatus::CriticalImprovement { .. } => "critical",
            };
            
            // 提取尺寸信息
            let (current_size, predicted_size, savings_percent) = match &report.status {
                optimization_analyzer::OptimizationStatus::AlreadyOptimized { 
                    current_size, predicted_size, savings_percent, .. 
                } |
                optimization_analyzer::OptimizationStatus::MinorImprovement { 
                    current_size, predicted_size, savings_percent, .. 
                } |
                optimization_analyzer::OptimizationStatus::SignificantImprovement { 
                    current_size, predicted_size, savings_percent, .. 
                } |
                optimization_analyzer::OptimizationStatus::CriticalImprovement { 
                    current_size, predicted_size, savings_percent, .. 
                } => (*current_size, *predicted_size, *savings_percent),
            };
            
            Ok(Some(OptimizationStatusInfo {
                status: status_str.to_string(),
                can_skip: report.can_skip,
                current_size,
                predicted_size,
                savings_percent,
                confidence: report.confidence,
                method: report.method,
            }))
        }
        Err(e) => {
            // 分析失败时，返回 None 而不是错误
            log::warn!("Optimization analysis failed: {}", e);
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
 use super::*;

  #[test]
  fn test_default_options() {
   let options = AnalyzeOptions::default();
   assert!(!options.json);
   assert_eq!(options.format, None);
  }
}
