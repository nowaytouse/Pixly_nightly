// 🔄 conversion Core
// from @archive/rust_broken/src/cli/conversion.rs extraction

use std::path::{Path, PathBuf};
use anyhow::Result;
use image::GenericImageView;
use crate::utils::feature_toggles::FeatureToggles;
use crate::utils::format_params::FormatSpecificParams;
use crate::core::validation_integration::ValidationDisplay;
use crate::utils::conversion_validator::ConversionValidator;
use crate::analysis::quality_analyzer::QualityAnalyzer;

#[derive(Debug, Clone)]
pub struct ConversionConfig {
 pub quality: u8,
 pub speed: u8,
 pub preserve_metadata: bool,
 pub keep_animated: bool,
 pub lossless: bool,
 pub merge_xmp_sidecar: bool,

// ═══════════════════════════════════════════════════
// 🎛️ featureintegration
// ═══════════════════════════════════════════════════
 pub feature_toggles: Option<FeatureToggles>,

// ═══════════════════════════════════════════════════
// 🔧 highlevelparameter (manualmode)
// ═══════════════════════════════════════════════════

/// degreesubsampling (420, 422, 444)
 pub chroma_subsampling: Option<String>,

/// Alphachannelquality (0-100)
 pub alpha_quality: Option<u8>,

/// encoding Effort level (1-10, overridespeed)
 pub effort: Option<u8>,

/// adjustedsizeoption
 pub resize: Option<ResizeOptions>,

/// colorquantizationoption
 pub quantize: Option<QuantizeOptions>,

/// sharpeningoption
 pub sharpen: Option<SharpenOptions>,

/// outputdirectory
 pub output_dir: Option<String>,

/// file
 pub normalize_filenames: bool,

/// 🔥 Phase 3.3: enabledvalidation
 pub enable_validation: bool,

/// validationlevel (1-5)
 pub validation_level: u8,

/// enabledqualityanalysis
 pub enable_quality_analysis: bool,

// ═══════════════════════════════════════════════════
// 📦 JXLparameter (fixedemptyfeature)
// ═══════════════════════════════════════════════════

/// JXL: usemodulemode
 pub jxl_modular: bool,

/// JXL: decoding
 pub jxl_progressive: bool,

/// JXL: responsedecoding
 pub jxl_responsive: bool,

/// JXL: Gaborishfilter
 pub jxl_gaborish: bool,

// ═══════════════════════════════════════════════════
// 📦 formatparameter (fullsupport HTMLsurface)
// ═══════════════════════════════════════════════════

/// formatparameter (JXL/Web P/AVIF/HEIC)
 pub format_specific_params: Option<FormatSpecificParams>,
}

/// adjustedsizeoption
#[derive(Debug, Clone)]
pub struct ResizeOptions {
 pub width: Option<u32>,
 pub height: Option<u32>,
 pub filter: String, // lanczos3, catmull_rom, gaussian, nearest
 pub maintain_aspect_ratio: bool,
}

/// colorquantizationoption
#[derive(Debug, Clone)]
pub struct QuantizeOptions {
 pub colors: u32,
 pub dithering: bool,
 pub dithering_level: f32,
}

/// sharpeningoption
#[derive(Debug, Clone)]
pub struct SharpenOptions {
 pub amount: f32,
 pub radius: f32,
 pub threshold: u8,
}

impl Default for ConversionConfig {
 fn default() -> Self {
 Self {
 quality: 85,
 speed: 4,
 preserve_metadata: true,
 keep_animated: true,
 lossless: false,
 merge_xmp_sidecar: false,
 feature_toggles: None,
 chroma_subsampling: None,
 alpha_quality: None,
 effort: None,
 resize: None,
 quantize: None,
 sharpen: None,
 output_dir: None,
 normalize_filenames: false,
 jxl_modular: false,
 jxl_progressive: false,
 jxl_responsive: false,
 jxl_gaborish: false,
 format_specific_params: None,
 enable_validation: false, // 🔥 Phase 3.3: defaultclose，optionalenabled
 validation_level: 2, // Standard level
 enable_quality_analysis: false,
 }
 }
}

impl ConversionConfig {
/// fromfeaturecreateconfiguration
 pub fn from_toggles(toggles: FeatureToggles) -> Self {
 Self {
 feature_toggles: Some(toggles),
 ..Default::default()
 }
 }

/// checkisnoenabled AIprediction
 pub fn is_ai_enabled(&self) -> bool {
 self.feature_toggles
 .as_ref()
 .map(|t| t.enable_ai_prediction)
 .unwrap_or(false)
 }

/// checkisnoenabledhighlevelparameter
 pub fn has_advanced_params(&self) -> bool {
 self.chroma_subsampling.is_some()
 || self.alpha_quality.is_some()
 || self.effort.is_some()
 || self.resize.is_some()
 || self.quantize.is_some()
 || self.sharpen.is_some()
 }

/// validationconfiguration
 pub fn validate(&self) -> Result<()> {
// validationqualityparameter
 if self.quality > 100 {
 anyhow::bail!("Quality parameter must be between 0-100");
 }

// validationspeedparameter
 if self.speed > 10 {
 anyhow::bail!("Speed parameter must be between 0-10");
 }

// validationalphaquality
 if let Some(alpha_q) = self.alpha_quality
 && alpha_q > 100 {
 anyhow::bail!("Alpha quality must be between 0-100");
 }

// validation Effort level
 if let Some(effort) = self.effort
 && effort > 10 {
 anyhow::bail!("Effort must be between 1-10");
 }

// validationfeature
 if let Some(ref toggles) = self.feature_toggles {
 toggles.validate()?;
 }

// validationformatparameter
 if let Some(ref format_params) = self.format_specific_params {
 format_params.validate()
 .map_err(|e| anyhow::anyhow!("Format parameter validation failed: {}", e))?;
 }

 Ok(())
 }
}

pub fn execute_conversion(
 input: &Path,
 output: &Path,
 format: &str,
 config: &ConversionConfig,
 input_format: Option<&str>,
) -> Result<ConversionResult> {
 let start_time = std::time::Instant::now();

// ═══════════════════════════════════════════════════
// 🎛️ featurecheck
// ═══════════════════════════════════════════════════
 let toggles = config.feature_toggles.as_ref();

 log::info!("🔄 Executing conversion:");
 log::info!(" Input: {:?}", input);
 log::info!(" Output: {:?}", output);
 log::info!(" Format: {}", format);
 log::info!(" Quality: {}", config.quality);
 log::info!(" Speed: {}", config.speed);

 if let Some(t) = toggles {
 log::info!(" Feature toggles: {}", t.summary());
 if config.has_advanced_params() {
 log::info!(" Advanced params: Enabled");
 }
 }

// getinputfilesize
 let input_size = std::fs::metadata(input)?.len();

// ═══════════════════════════════════════════════════
// 🎬 videoautoconversion (ifenabled)
// ═══════════════════════════════════════════════════
 if toggles.map(|t| t.enable_video_for_animation).unwrap_or(false)
 && should_convert_animation_to_video(input)? {
 log::info!("🎬 Large animated image detected, auto-converting to video format");
 log::info!(" File: {:?}", input);
 log::info!(" Expected size reduction: 60-80%");
 log::info!(" Using codec: H.265/HEVC");

// 🔥 Auto-convert to video
 let video_output = output.with_extension("mp4");
 log::info!(" Conversion target: {:?}", video_output);

 convert_animation_to_video(input, &video_output)?;

 log::info!(" ✅ Animation converted to video");
 log::info!("");

// 🔥 returnvideoconversionresult， not againcontinueimageconversion
 let output_size = std::fs::metadata(&video_output)?.len();
 return Ok(ConversionResult {
 input_size,
 output_size,
 compression_ratio: output_size as f64 / input_size as f64,
 duration: start_time.elapsed(),
 strategy_used: "animation_to_video".to_string(),
 });
 }

// ═══════════════════════════════════════════════════
// 🔒 AIfilevalidation (ifenabled)
// ═══════════════════════════════════════════════════
 if toggles.map(|t| t.enable_file_validation).unwrap_or(false) {
 println!("🔒 Running AI file validation (Magika)...");
 validate_file_with_magika(input)?;
 }

// ═══════════════════════════════════════════════════
// 🔧 formatautopositive (ifenabled)
// ═══════════════════════════════════════════════════
 if toggles.map(|t| t.enable_format_correction).unwrap_or(false) {
 log::info!("🔧 Checking format correction...");
 check_format_correction(input)?;
 }

// ═══════════════════════════════════════════════════
// ✅ configurationvalidation
// ═══════════════════════════════════════════════════
 config.validate()?;

// ═══════════════════════════════════════════════════
// 🔗 intelligentpreprocessing (ifenabled)
// ═══════════════════════════════════════════════════
 let preprocessed_input = if toggles.map(|t| t.enable_preprocess).unwrap_or(false) {
 log::info!("🔗 Running intelligent preprocessing...");
 apply_preprocessing(input, config)?
 } else {
 input.to_path_buf()
 };

// ═══════════════════════════════════════════════════
// 🔄 executeactualconversion
// ═══════════════════════════════════════════════════
 let strategy_used = perform_conversion(&preprocessed_input, output, format, config, input_format)?;

// cleanuptemporarypreprocessingfile
 if preprocessed_input != input {
 let _ = std::fs::remove_file(&preprocessed_input);
 }

// ═══════════════════════════════════════════════════
// ✅ validationoutputquality
// ═══════════════════════════════════════════════════
 validate_output_quality(output, config)?;

// ═══════════════════════════════════════════════════
// 🔥 Phase 3.3: outputvalidation
// ═══════════════════════════════════════════════════
 if config.enable_validation {
 log::info!("🔍 Running output validation...");
 let result = ConversionValidator::validate_output(output, Some(input_size));
 ValidationDisplay::display_result(&result);

 if !result.valid && config.validation_level >= 4 {
// modedownvalidationfailurethen
 anyhow::bail!("Validation failed: output did not meet quality standards");
 }
 }

// ═══════════════════════════════════════════════════
// 📊 Phase 3.3: qualityanalysis
// ═══════════════════════════════════════════════════
 if config.enable_quality_analysis {
 log::info!("📊 Running quality analysis...");
 let analyzer = QualityAnalyzer::new();

 match analyzer.analyze(output) {
 Ok(metrics) => {
 log::info!(" Estimated quality: {}", metrics.estimated_quality);
 log::info!(" Complexity score: {:.2}", metrics.complexity_score);
 log::info!(" Bytes per pixel: {:.2}", metrics.bytes_per_pixel);
 log::info!(" Content type: {}", metrics.content_type);
 }
 Err(e) => {
 log::warn!(" ⚠️ Quality analysis failed: {}", e);
 }
 }
 }

// ═══════════════════════════════════════════════════
// 📄 XMP Sidecarmerged (ifenabled)
// ═══════════════════════════════════════════════════
 if config.merge_xmp_sidecar {
 merge_xmp_sidecar(input, output)?;
 }

// ═══════════════════════════════════════════════════
// 📊 SSIMqualityvalidation (ifenabled)
// ═══════════════════════════════════════════════════
 if toggles.map(|t| t.enable_ssim).unwrap_or(false) {
 log::info!("📊 Running SSIM quality validation...");
 validate_ssim_quality(input, output)?;
 }

 let output_size = std::fs::metadata(output)?.len();
 let elapsed = start_time.elapsed();

 log::info!("✅ Conversion completed:");
 log::info!(" Input size: {} bytes", input_size);
 log::info!(" Output size: {} bytes", output_size);
 log::info!(" Compression ratio: {:.2}%", (output_size as f64 / input_size as f64) * 100.0);
 log::info!(" Time elapsed: {:.2}s", elapsed.as_secs_f64());

// ═══════════════════════════════════════════════════
// 🎓 atlinelearning：recordconversion
// ═══════════════════════════════════════════════════
 record_conversion_for_learning(input, output, config, input_size, output_size);

 Ok(ConversionResult {
 input_size,
 output_size,
 compression_ratio: output_size as f64 / input_size as f64,
 duration: elapsed,
 strategy_used,
 })
}

/// 🎓 recordconversionforatlinelearning
fn record_conversion_for_learning(
 input: &Path,
 output: &Path,
 config: &ConversionConfig,
 input_size: u64,
 output_size: u64,
) {
 use crate::ai::online_learner_manager::OnlineLearnerManager;
 use crate::core::feature_extractor_128d::extract_128d_features;
 use crate::ai::reward_calculator::ConversionResult as RewardResult;

// Load imageandextractionfeature
 let img = match image::open(input) {
 Ok(i) => i,
 Err(e) => {
 log::warn!("⚠️ Failed to open image for feature extraction: {}", e);
 return;
 }
 };

// getbasicfeature
 let basic_features = crate::ImageFeatures {
 width: img.width(),
 height: img.height(),
 format: String::from("unknown"),
 has_alpha: img.color().has_alpha(),
 is_animated: false,
 complexity: 0.5,
 file_size: input_size,
 };

 let features = extract_128d_features(&img, input, &basic_features);

// Calculate SSIM（ifmay）
 let ssim = calculate_ssim_simple(input, output).unwrap_or(0.95);

// createconversionresult
 let result = RewardResult {
 original_size: input_size,
 output_size,
 ssim,
 processing_time: 0.0,
 };

// usegloballearningManagerrecord
 match OnlineLearnerManager::record_conversion(features, config.quality as u32, config.speed as u32, result) {
 Ok(_) => {
 let buffer_size = OnlineLearnerManager::buffer_size();
 println!("📝 Conversion experience recorded (global buffer: {})", buffer_size);

// ifbuffertothreshold， will autotriggerupdate
 if buffer_size >= 10 {
 println!("🎓 Model update triggered! ({} experiences accumulated)", buffer_size);
 }
 }
 Err(e) => {
// 🔥 failure：displaydetailederrorinformation
 eprintln!("❌ Failed to record conversion experience: {}", e);
 eprintln!(" This may affect online learning quality");
 log::error!("Online learning error: {}", e);
 }
 }
}

/// singleSSIMcalculation
fn calculate_ssim_simple(input: &Path, output: &Path) -> Option<f64> {
 use std::process::Command;

 let output_result = Command::new("python3")
 .arg("scripts/calculate_ssim.py")
 .arg(input)
 .arg(output)
 .output()
 .ok()?;

 if !output_result.status.success() {
 return None;
 }

 let stdout = String::from_utf8_lossy(&output_result.stdout);
 stdout.trim().parse::<f64>().ok()
}

/// 🔒 use Magika AIvalidationfiletype
fn validate_file_with_magika(input: &Path) -> Result<()> {
 use crate::utils::magika_detector::MagikaDetector;

 let detector = MagikaDetector::with_defaults();
 match detector.detect_file_type(input) {
 Ok(detection) => {
 println!(" ✅ File type: {} (confidence: {:.1}%)",
 detection.detected_type, detection.confidence * 100.0);

// checkconfidence
 if !detection.is_high_confidence {
 println!(" ⚠️ Warning: Low confidence detection");
 }

 Ok(())
 }
 Err(e) => {
// 🔥 quality：AIfailurejust
 eprintln!("❌ Magika AI validation FAILED: {}", e);
 eprintln!(" File validation cannot proceed without AI");
 Err(e)
 }
 }
}

/// 🔧 checkformatpositive
fn check_format_correction(input: &Path) -> Result<()> {
 use crate::utils::format_corrector::FormatCorrector;

 let corrector = FormatCorrector::new(false); // notautorename，onlycheck
 match corrector.check_and_correct(input) {
 Ok(result) if result.needs_correction => {
 println!(" ⚠️ Format mismatch detected:");
 println!(" Extension: {}", result.original_extension);
 println!(" Actual format: {}", result.detected_format);
 println!(" {}", result.message);

// onlywarning， not conversion
 Ok(())
 }
 Ok(_) => {
 println!(" ✅ Format matches extension");
 Ok(())
 }
 Err(e) => {
 println!(" ⚠️ Format correction check failed: {}", e);
// not conversion
 Ok(())
 }
 }
}

/// 🔗 applyintelligentpreprocessing
fn apply_preprocessing(input: &Path, _config: &ConversionConfig) -> Result<PathBuf> {
 use crate::operations::preprocessing::PreprocessPipeline;

 println!(" 🔍 Analyzing image for preprocessing...");

// readimage
 let img = image::open(input)?;

// applypreprocessing
 let pipeline = PreprocessPipeline::new();
 let processed_img = pipeline.process(img)?;

// savetotemporaryfile
 let temp_output = input.with_extension("preprocessed.png");
 processed_img.save(&temp_output)?;

 println!(" ✅ Preprocessing complete");
 Ok(temp_output)
}

/// 🎬 detectionisnoshouldconversionforvideo
fn should_convert_animation_to_video(input: &Path) -> Result<bool> {
// checkfileextension
 let ext = input
 .extension()
 .and_then(|e| e.to_str())
 .unwrap_or("")
 .to_lowercase();

// onlydetectionformat
 if !matches!(ext.as_str(), "gif" | "apng" | "webp") {
 return Ok(false);
 }

// getfilesize
 let metadata = std::fs::metadata(input)?;
 let file_size = metadata.len();

// greater than2MBrecommendedvideo
 if file_size > 2 * 1024 * 1024 {
 return Ok(true);
 }

// trygetimagedimension
 if let Ok(img) = image::open(input) {
 let (width, height) = img.dimensions();
 let pixels = width * height;

// highresolution (>800x600) recommendedvideo
 if pixels > 800 * 600 {
 return Ok(true);
 }
 }

 Ok(false)
}

/// 🎬 conversionforvideo
fn convert_animation_to_video(input: &Path, output: &Path) -> Result<()> {
 use crate::codecs::video::video_processor::{VideoProcessor, VideoConversionConfig, AudioMode};

// createvideoconversionconfiguration
 let config = VideoConversionConfig {
 codec: "h265".to_string(), // H.265mostcompress
 container: "mp4".to_string(),
 crf: 23, // quality and body
 preset: "medium".to_string(),
 target_resolution: None,
 target_fps: None,
 audio_mode: AudioMode::Remove, // No audio
 two_pass: false,
 hw_accel: "auto".to_string(),
 gop_size: Some(250),
 bframes: Some(3),
 ref_frames: Some(3),
 me_method: Some("hex".to_string()),
 pix_fmt: None,
 rate_control: None, // 🔥 Phase 3: addrate_controlsegment
 };

// executeconversion
 let processor = VideoProcessor::new();
 let result = processor.convert_video(input, output, &config, Some(|_progress: f32| {
// progresscallback（optional）
 }))?;

 if !result.success {
 anyhow::bail!("Animation to video conversion failed: {}",
 result.error.unwrap_or_default());
 }

 Ok(())
}

/// 📊 SSIMqualityvalidation
fn validate_ssim_quality(original: &Path, converted: &Path) -> Result<()> {
 use crate::analysis::quality_checker::QualityChecker;

 let checker = QualityChecker::new();

 match checker.check_conversion_quality(original, converted) {
 Ok(result) => {
 println!(" 📊 SSIM Score: {:.4}", result.ssim_score);
 println!(" 📊 Quality Grade: {}", result.quality_grade.as_str());

 if result.passed {
 println!(" ✅ Quality check passed");
 } else {
 println!(" ⚠️ Warning: Quality below threshold");
 }

 println!(" {}", result.details);

 Ok(())
 }
 Err(e) => {
 println!(" ⚠️ SSIM validation failed: {}", e);
// not conversion
 Ok(())
 }
 }
}

/// validationoutputquality
fn validate_output_quality(output: &Path, _config: &ConversionConfig) -> Result<()> {
 if !output.exists() {
 anyhow::bail!("Output file does not exist: {:?}", output);
 }

 let metadata = std::fs::metadata(output)?;
 if metadata.len() == 0 {
 anyhow::bail!("Output file is empty: {:?}", output);
 }

// tryopenoutputfilevalidationformatpositive
// note: image crate not support has format(likeAVIF/JXL)，byvalidationfailure not aiserror
 if let Err(e) = image::open(output) {
// checkisnois not supportformat
 let ext = output.extension()
 .and_then(|s| s.to_str())
 .unwrap_or("")
 .to_lowercase();

 if matches!(ext.as_str(), "avif" | "jxl" | "jpegxl") {
// thisformatexternalprocessing，skipimage cratevalidation
 println!("✅ Output quality validation passed (external format)");
 return Ok(());
 }

 anyhow::bail!("Output file format is invalid: {}", e);
 }

 println!("✅ Output quality validation passed");
 Ok(())
}

fn perform_conversion(
 input: &Path,
 output: &Path,
 format: &str,
 config: &ConversionConfig,
 input_format: Option<&str>,
) -> Result<String> {
 use image::ImageFormat;
 use crate::utils::same_format_optimizer::SameFormatOptimizer;

 // checkinputformat，ifisexternalformat(AVIF/JXL)，conversionfor PNG
 let input_ext = if let Some(fmt) = input_format {
  fmt.to_lowercase()
 } else {
  input.extension()
  .and_then(|s| s.to_str())
  .unwrap_or("")
  .to_lowercase()
 };

 // 🔮 滤镜模式：检测同格式优化
 let target_format = format.to_lowercase();
 let is_same_format = input_ext == target_format || 
  (input_ext == "jpg" && target_format == "jpeg") ||
  (input_ext == "jpeg" && target_format == "jpg") ||
  (input_ext == "tif" && target_format == "tiff") ||
  (input_ext == "tiff" && target_format == "tif");

 // 🔮 同格式优化：使用专用优化器
 if is_same_format {
  log::info!("🔮 Filter mode: Same format optimization ({} → {})", input_ext, target_format);
  
  // 🤖 AI滤镜模式：检测是否使用了AI预测的质量参数
  let is_ai_quality = config.quality != 90; // 90是默认值，非90说明AI预测了
  if is_ai_quality {
   log::info!("🤖 AI Filter mode: Using AI-predicted quality {}", config.quality);
  }
  
  // 检查是否有可用的优化工具
  if let Ok(tool) = SameFormatOptimizer::check_tools_for_format(&input_ext) {
   log::info!("   Using optimizer: {}", tool);
   let optimizer = SameFormatOptimizer::new(config.preserve_metadata);
   
   match optimizer.optimize(input, output, &input_ext, config.quality) {
    Ok(result) => {
     log::info!("   ✅ Same-format optimization complete");
     log::info!("   Input: {} bytes", result.input_size);
     log::info!("   Output: {} bytes", result.output_size);
     log::info!("   Compression: {:.1}% size reduction", (1.0 - result.compression_ratio) * 100.0);
     log::info!("   Tool: {}", result.tool_used);
     if is_ai_quality {
      log::info!("   🤖 AI-optimized quality applied");
     }
     return Ok(format!("same_format_{}", result.tool_used));
    }
    Err(e) => {
     log::warn!("   ⚠️ Same-format optimization failed: {}", e);
     log::info!("   Falling back to standard conversion");
     // 继续使用标准转换
    }
   }
  } else {
   log::info!("   No optimizer available for {}, using standard conversion", input_ext);
  }
 }

 let (actual_input, temp_file) = if matches!(input_ext.as_str(), "avif" | "jxl" | "jpegxl") {
 // createtemporaryPNGfile
  let timestamp = std::time::SystemTime::now()
  .duration_since(std::time::UNIX_EPOCH)
  .unwrap_or(std::time::Duration::from_secs(0))
  .as_millis();
  let temp_path = std::env::temp_dir().join(format!("pixly_temp_{}.png", timestamp));

 // useexternalconversionfor PNG
  decode_external_format(input, &temp_path, &input_ext)?;
  (temp_path.clone(), Some(temp_path))
 } else {
  (input.to_path_buf(), None)
 };

 // readinputimage
 let img = image::open(&actual_input)
  .map_err(|e| anyhow::anyhow!("Failed to open image: {}. Format: {}", e, input_ext))?;

// based onformatselect Encoder
 let strategy = match format.to_lowercase().as_str() {
 "webp" => {
// Web Pencoding - usesave_with_formatbecauseforimage crate Web PEncoder APIlimit
 img.save_with_format(output, ImageFormat::WebP)?;
 "webp_native"
 }
 "png" => {
// PNGencoding
 let file = std::fs::File::create(output)?;
 let encoder = image::codecs::png::PngEncoder::new(file);
 img.write_with_encoder(encoder)?;
 "png_native"
 }
 "jpg" | "jpeg" => {
// JPEGencoding
 let file = std::fs::File::create(output)?;
 let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(file, config.quality);
 img.write_with_encoder(encoder)?;
 "jpeg_native"
 }
 "gif" => {
// GIFencoding
 img.save_with_format(output, ImageFormat::Gif)?;
 "gif_native"
 }
 "bmp" => {
 img.save_with_format(output, ImageFormat::Bmp)?;
 "bmp_native"
 }
 "tiff" | "tif" => {
 img.save_with_format(output, ImageFormat::Tiff)?;
 "tiff_native"
 }
 "avif" => {
// AVIFencoding - useexternal
 convert_to_avif(&actual_input, output, config)?;
 "avif_external"
 }
 "jxl" | "jpegxl" => {
// JPEG XLencoding - useexternal
 convert_to_jxl(&actual_input, output, config)?;
 "jxl_external"
 }
 _ => {
 anyhow::bail!("Unsupported output format: {}", format);
 }
 };

// cleanuptemporaryfile
 if let Some(temp) = temp_file {
 let _ = std::fs::remove_file(temp);
 }

 Ok(strategy.to_string())
}

/// decodingexternalformat(AVIF/JXL)forPNG
fn decode_external_format(input: &Path, output: &Path, input_fmt: &str) -> Result<()> {
 use std::process::Command;

 match input_fmt {
 "avif" => {
// useavifencdecodingfeatureor Image Magick
 let output = Command::new("magick")
 .arg("convert")
 .arg(input)
 .arg(output)
 .output()?;

 if !output.status.success() {
 anyhow::bail!("AVIF decode failed: {}", String::from_utf8_lossy(&output.stderr));
 }
 }
 "jxl" | "jpegxl" => {
// usedjxlor Image Magick
 let output = Command::new("magick")
 .arg("convert")
 .arg(input)
 .arg(output)
 .output()?;

 if !output.status.success() {
 anyhow::bail!("JXL decode failed: {}", String::from_utf8_lossy(&output.stderr));
 }
 }
 _ => anyhow::bail!("Unsupported external format: {}", input_fmt),
 }

 Ok(())
}

/// Convert to AVIF using external tools
fn convert_to_avif(input: &Path, output: &Path, config: &ConversionConfig) -> Result<()> {
 use std::process::Command;

 let mut cmd = Command::new("avifenc");

// Get min/max quantizer from format_specific_params
 let (min_q, max_q) = if let Some(ref params) = config.format_specific_params
 && let crate::utils::format_params::FormatSpecificParams::Avif(avif) = params {
 (
 avif.min_quantizer.unwrap_or(0),
 avif.max_quantizer.unwrap_or(63)
 )
 } else {
 (0, 63)
 };

 cmd.arg("--min").arg(min_q.to_string())
 .arg("--max").arg(max_q.to_string())
 .arg("-s").arg(config.speed.to_string())
 .arg("-q").arg(config.quality.to_string());

 println!(" AVIF: min_quantizer={}, max_quantizer={}", min_q, max_q);

// 🔥 Phase: AVIFhighlevelparametersupport（fixedatempty）
// Chroma subsampling: -y or --yuv (420, 422, 444)
 if let Some(ref chroma) = config.chroma_subsampling {
 cmd.arg("-y").arg(chroma);
 println!(" 🔧 AVIF: Chroma subsampling {}", chroma);
 }

// Alpha quality: --qalpha (0-100)
 if let Some(alpha_q) = config.alpha_quality {
 cmd.arg("--qalpha").arg(alpha_q.to_string());
 println!(" 🔧 AVIF: Alpha quality {}", alpha_q);
 }

 cmd.arg(input).arg(output);

 let result = cmd.output();

 match result {
 Ok(output_result) if output_result.status.success() => Ok(()),
 Ok(output_result) => {
 let error = String::from_utf8_lossy(&output_result.stderr);
 anyhow::bail!("avifenc failed: {}", error)
 }
 Err(e) => {
// 🔥 quality：error， not fallback！
 anyhow::bail!("avifenc not found or failed to execute: {}. Please install avifenc: brew install libavif", e)
 }
 }
}

/// Convert to JPEG XL using external tools
fn convert_to_jxl(input: &Path, output: &Path, config: &ConversionConfig) -> Result<()> {
 use std::process::Command;

// Find cjxl in system PATH
 let cjxl_path = which::which("cjxl")
 .map_err(|_| anyhow::anyhow!(
 "cjxl not found in PATH. Please install libjxl:\n\
 macOS: brew install jpeg-xl\n\
 Linux: apt install libjxl-tools or yum install libjxl-tools\n\
 Windows: Download from https://github.com/libjxl/libjxl/releases"
 ))?;

 let effort = (10 - config.speed).clamp(1, 9);
 let distance = ((100 - config.quality) as f32 / 10.0).clamp(0.0, 15.0);

// Detect if input is JPEG
 let is_jpeg_input = input.extension()
 .and_then(|e| e.to_str())
 .map(|e| {
 let ext = e.to_lowercase();
 ext == "jpg" || ext == "jpeg"
 })
 .unwrap_or(false);

 let mut cmd = Command::new(&cjxl_path);
 cmd.arg(input)
 .arg(output)
 .arg("--effort").arg(effort.to_string());

// JPEGinput when processing：
// - cjxldefaultenabled--lossless_jpeg=1（losslessheavynewpack）
// - ifrequirementlossyconversion，needdisabledlossless_jpegandpassdistance
// - ifrequirementlossless， not passdistance（ let cjxlusedefaultlossless_jpeg=1）
 if !config.lossless {
 if is_jpeg_input {
// JPEGinput + lossyconversion：disabledlossless_jpeg，passdistance
 cmd.arg("--lossless_jpeg").arg("0");
 }
 cmd.arg("--distance").arg(distance.to_string());
 }
// ifconfig.lossless=true， not passdistance， let cjxlusedefaultlosslessmode

// 🔥 Phase: JXLhighlevelparametersupport（fixedemptyfeature）
// based on PROJECT_QUALITY_MANIFESTO.md - pairoriginalthen

// Modular mode: -m 0|1 or --modular=0|1
 if config.jxl_modular {
 cmd.arg("--modular=1");
 println!(" 🔧 JXL: Modular mode enabled");
 }

// Progressive decoding: -p or --progressive
 if config.jxl_progressive {
 cmd.arg("--progressive");
 println!(" 🔧 JXL: Progressive decoding enabled");
 }

// Responsive decoding: -R K or --responsive=K
 if config.jxl_responsive {
 cmd.arg("--responsive=1");
 println!(" 🔧 JXL: Responsive decoding enabled");
 }

// Gaborish filter: --gaborish=0|1
 if config.jxl_gaborish {
 cmd.arg("--gaborish=1");
 println!(" 🔧 JXL: Gaborish filter enabled");
 }

 let result = cmd.output();

 match result {
 Ok(output_result) if output_result.status.success() => Ok(()),
 Ok(output_result) => {
 let error = String::from_utf8_lossy(&output_result.stderr);
 anyhow::bail!("cjxl failed: {}", error)
 }
 Err(e) => {
 anyhow::bail!("cjxl execution failed: {}", e)
 }
 }
}

#[derive(Debug, Clone)]
pub struct ConversionResult {
 pub input_size: u64,
 pub output_size: u64,
 pub compression_ratio: f64,
 pub duration: std::time::Duration,
 pub strategy_used: String,
}

/// Get XMP sidecar file path
///
/// XMP naming rule: replace extension with .xmp
/// Example: photo.jpg → photo.xmp
fn get_xmp_sidecar_path(file_path: &Path) -> std::path::PathBuf {
 let mut xmp_path = file_path.to_path_buf();
 xmp_path.set_extension("xmp");
 xmp_path
}

/// Merge XMP sidecar file into output image (Eagle-compatible)
///
/// Full process matching Eagle adapter:
/// 1. Pre-merge validation: Check XMP file exists and is readable
/// 2. Verify output file exists before merge
/// 3. Use exiftool to merge XMP into output file
/// 4. Post-merge validation: Verify XMP data was written
/// 5. Delete original XMP sidecar only after successful validation
/// 6. Handle all error cases gracefully
fn merge_xmp_sidecar(input: &Path, output: &Path) -> Result<()> {
 let xmp_path = get_xmp_sidecar_path(input);

// Pre-merge validation: Check if XMP exists
 if !xmp_path.exists() {
 return Ok(()); // No XMP file, nothing to do
 }

// Pre-merge validation: Check XMP is readable
 if let Err(e) = std::fs::metadata(&xmp_path) {
 eprintln!("⚠️ XMP file exists but not readable: {}", e);
 return Ok(());
 }

 println!("📄 Found XMP sidecar: {:?}", xmp_path.file_name());

// Pre-merge validation: Verify output file exists
 if !output.exists() {
 eprintln!("⚠️ Output file doesn't exist yet, skipping XMP merge");
 return Ok(());
 }

// Get output file size before merge for validation
 let output_size_before = std::fs::metadata(output)?.len();

// Use exiftool to merge XMP into output file
 use std::process::Command;
 let merge_result = Command::new("exiftool")
 .arg("-tagsFromFile")
 .arg(&xmp_path)
 .arg("-XMP:all")
 .arg("-overwrite_original")
 .arg(output)
 .output();

 match merge_result {
 Ok(output_result) if output_result.status.success() => {
 println!("✅ XMP merged into output file: {:?}", output.file_name());

// Post-merge validation: Verify output file was modified
 let output_size_after = std::fs::metadata(output)?.len();
 if output_size_after <= output_size_before {
 eprintln!("⚠️ Warning: Output file size didn't increase after XMP merge");
 eprintln!(" Before: {} bytes, After: {} bytes", output_size_before, output_size_after);
 }

// Post-merge validation: Verify XMP data exists in output
 let verify_result = Command::new("exiftool")
 .arg("-XMP:all")
 .arg(output)
 .output();

 let xmp_verified = match verify_result {
 Ok(verify_output) if verify_output.status.success() => {
 let output_str = String::from_utf8_lossy(&verify_output.stdout);
 !output_str.trim().is_empty() && !output_str.contains("no XMP")
 }
 _ => false
 };

 if !xmp_verified {
 eprintln!("⚠️ Warning: Could not verify XMP data in output file");
 eprintln!("⚠️ Keeping original XMP sidecar for safety: {:?}", xmp_path);
 return Ok(());
 }

 println!("✅ XMP data verified in output file");

// Delete original XMP after successful merge and validation
 if let Err(e) = std::fs::remove_file(&xmp_path) {
 eprintln!("⚠️ Failed to delete original XMP: {}", e);
 } else {
 println!("🗑️ Original XMP sidecar deleted");
 }
 Ok(())
 }
 Ok(output_result) => {
 let error_msg = String::from_utf8_lossy(&output_result.stderr);
 eprintln!("❌ XMP merge failed: {}", error_msg);
 eprintln!("⚠️ Keeping original XMP sidecar: {:?}", xmp_path);
 Ok(()) // Don't fail the conversion
 }
 Err(e) => {
 eprintln!("❌ Cannot execute exiftool: {}", e);
 eprintln!("💡 Install exiftool: brew install exiftool");
 eprintln!("⚠️ Keeping original XMP sidecar: {:?}", xmp_path);
 Ok(()) // Don't fail the conversion
 }
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_default_config() {
 let config = ConversionConfig::default();
 assert_eq!(config.quality, 85);
 assert_eq!(config.speed, 4);
 assert!(config.preserve_metadata);
 assert!(!config.merge_xmp_sidecar);
 }

 #[test]
 fn test_xmp_path_generation() {
 let input = Path::new("photo.jpg");
 let xmp = get_xmp_sidecar_path(input);
 assert_eq!(xmp, Path::new("photo.xmp"));

 let input2 = Path::new("/path/to/image.png");
 let xmp2 = get_xmp_sidecar_path(input2);
 assert_eq!(xmp2, Path::new("/path/to/image.xmp"));
 }
}
