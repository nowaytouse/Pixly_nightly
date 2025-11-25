/**
 * 🔥 验证系统integratedmodule
 * 
 * willmulti级验证系统integratedtoconvert流程
 * - CLIintegrated
 * - convert流程integrated
 * - 格式特定checkenhanced
 * - 异步验证support
 */
use crate::utils::conversion_validator::*;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};

/// CLIvalidationresultdisplay
pub structure ValidationDisplay;
impl ValidationDisplay {
 /// displayvalidationresult
 pub fn display_result(result: &ValidationResult) {
 if result.valid {
 log::info!("✅ Validation passed");
 log::info!(" Validation level: Level {}", result.level);
 
 if !result.warnings.is_empty() {
 log::info!("\n⚠️ Warnings:");
 for warning in &result.warnings {
 log::info!(" {}", warning);
 }
 }
 } else {
 log::error!("❌ Validation failed");
 log::error!(" Failed at level: Level {}", result.level);
 
 if !result.errors.is_empty() {
 log::error!("\nErrors:");
 for error in &result.errors {
 log::error!(" {}", error);
 }
 }
 
 if !result.warnings.is_empty() {
 log::warn!("\nWarnings:");
 for warning in &result.warnings {
 log::warn!(" {}", warning);
 }
 }
 }
 }
 
 /// display简洁validation摘要
 pub fn display_summary(result: &ValidationResult) {
 if result.valid {
 print!("✅ ");
 } else {
 print!("❌ ");
 }
 
 if !result.warnings.is_empty() {
 print!("⚠️ {} warnings ", result.warnings.len());
 }
 
 if !result.errors.is_empty() {
 print!("❌ {} errors ", result.errors.len());
 }
 
 log::info!("");
 }
 
 /// displaydetailedvalidationreport
 pub fn display_detailed_report(result: &ValidationResult) {
 log::info!("\n{}", "═".repeat(60));
 log::info!(" Validation Report");
 log::info!("{}", "═".repeat(60));
 
 // status
 let status = if result.valid {
 "Passed ✅"
 } else {
 "Failed ❌"
 };
 log::info!("Status: {}", status);
 log::info!("Level: Level {}", result.level);
 
 // error
 if !result.errors.is_empty() {
 log::info!("\nError List:");
 for (i, error) in result.errors.iter().enumerate() {
 log::info!(" {}. {}", i + 1, error);
 }
 }
 
 // warning
 if !result.warnings.is_empty() {
 log::info!("\nWarning List:");
 for (i, warning) in result.warnings.iter().enumerate() {
 log::info!(" {}. {}", i + 1, warning);
 }
 }
 
 log::info!("{}", "═".repeat(60));
 }
}

/// conversion流程validation
pub structure ConversionFlowValidator;
impl ConversionFlowValidator {
 /// 🔥 realanimationdetection（useffprobe）
 /// 
 /// useffprobe精准detectionfileframe数
 /// 
 /// note：needffprobe工具，ifunavailable则fallbackto扩展名判断
 fn detect_animation_heuristic(path: &Path, ext: &str) -> bool {
 use std::process::Command;
 
 // tryuseffprobe精准detection
 if let Ok(output) = Command::new("ffprobe")
 .arg("-v").arg("error")
 .arg("-select_streams").arg("v:0")
 .arg("-show_entries").arg("stream=nb_frames")
 .arg("-of").arg("default=noprint_wrappers=1:nokey=1")
 .arg(path)
 .output()
 && output.status.success()
 && let Ok(frames_str) = String::from_utf8(output.stdout)
 && let Ok(frames) = frames_str.trim().parse::<u32>() {
 // exceeds1frame就isanimation
 return frames > 1;
 }
 
 // Fallback: based on扩展名判断（ffprobeunavailable when ）
 let ext_lower = ext.to_lowercase();
 matches!(ext_lower.as_str(), ".gif" | ".apng" | ".webp")
 }
 
 /// validationconversion before 准备work
 pub fn validate_pre_conversion(
 input_paths: &[PathBuf],
 target_format: &str,
 quality: Option<u32>,
 speed: Option<u32>,
 lossless: bool,
 mode: ConversionMode,
 ) -> Result<ValidationResult> {
 // buildinputfilelist
 let mut files = Vec::new();
 for path in input_paths {
 let metadata = std::fs::metadata(path)
 .with_context(|| format!("Cannot read file: {}", path.display()))?;
 
 let name = path.file_name()
 .and_then(|n| n.to_str())
 .unwrap_or("unknown")
 .to_string();
 
 let ext = path.extension()
 .and_then(|e| e.to_str())
 .map(|e| format!(".{}", e))
 .unwrap_or_default();
 
 // useheuristicmethoddetectionanimation
 let is_animated = Self::detect_animation_heuristic(path, &ext);
 
 files.push(InputFile {
 file_path: path.clone(),
 name,
 ext,
 size: metadata.len(),
 is_animated,
 });
 }
 
 // buildconfiguration
 let config = ConversionConfig {
 format: target_format.to_string(),
 quality,
 speed,
 lossless,
 };
 
 // executefullvalidation
 let result = ConversionValidator::validate_full_conversion(&files, &config, mode);
 
 Ok(result)
 }
 
 /// validationconversion after output
 pub fn validate_post_conversion(
 output_path: &Path,
 input_size: u64,
 ) -> Result<ValidationResult> {
 let result = ConversionValidator::validate_output(output_path, Some(input_size));
 Ok(result)
 }
 
 /// batchvalidationmultifile
 pub fn validate_batch(
 input_paths: &[PathBuf],
 target_format: &str,
 quality: Option<u32>,
 speed: Option<u32>,
 lossless: bool,
 ) -> Result<Vec<ValidationResult>> {
 let mut results = Vec::new();
 
 for path in input_paths {
 let result = Self::validate_pre_conversion(
 std::slice::from_ref(path),
 target_format,
 quality,
 speed,
 lossless,
 ConversionMode::Manual,
 )?;
 results.push(result);
 }
 
 Ok(results)
 }
}

/// formatspecificcheckenhanced
pub structure FormatSpecificChecks;
impl FormatSpecificChecks {
 /// Web Pspecificcheck
 pub fn check_webp(files: &[InputFile], config: &ConversionConfig) -> Vec<String> {
 let mut warnings = Vec::new();
 
 // check大图
 for file in files {
 if file.size > 16 * 1024 * 1024 { // 16MB
 warnings.push(format!(
 "⚠️ File {} is large ({:.2} MB), WebP encoding may be slow",
 file.name,
 file.size as f64 / (1024.0 * 1024.0)
 ));
 }
 }
 
 // checkqualitysetting
 if let Some(quality) = config.quality
 && quality < 70 {
 warnings.push(format!(
 "⚠️ WebP quality setting is low ({}), may show visible compression artifacts",
 quality
 ));
 }
 
 warnings
 }
 
 /// AVIFspecificcheck
 pub fn check_avif(files: &[InputFile], config: &ConversionConfig) -> Vec<String> {
 let mut warnings = Vec::new();
 
 // checkanimation
 let has_animation = files.iter().any(|f| f.is_animated);
 if has_animation {
 warnings.push("⚠️ AVIF animation support is limited, recommend using WebP or GIF".to_string());
 }
 
 // checkspeedsetting
 if let Some(speed) = config.speed
 && speed > 6 {
 warnings.push(format!(
 "⚠️ AVIF speed setting is high ({}), may affect compression efficiency",
 speed
 ));
 }
 
 // check大图
 for file in files {
 let pixels = file.size / 3; // 粗略估计
 if pixels > 4000 * 4000 {
 warnings.push(format!(
 "⚠️ File {} has high resolution, AVIF encoding may take longer",
 file.name
 ));
 }
 }
 
 warnings
 }
 
 /// JXLspecificcheck
 pub fn check_jxl(files: &[InputFile], config: &ConversionConfig) -> Vec<String> {
 let mut warnings = Vec::new();
 
 // check JPEGinput
 let has_jpeg = files.iter().any(|f| {
 let ext = f.ext.to_lowercase();
 ext == ".jpg" || ext == ".jpeg"
 });
 
 if has_jpeg && !config.lossless {
 warnings.push("💡 Tip: JPEG → JXL recommend using lossless mode to utilize JPEG repackaging".to_string());
 }
 
 // checkqualitysetting
 if let Some(quality) = config.quality
 && quality < 60 {
 warnings.push(format!(
 "⚠️ JXL quality setting is low ({}), may not be better than WebP or AVIF",
 quality
 ));
 }
 
 warnings
 }
 
 /// PNGspecificcheck
 pub fn check_png(files: &[InputFile], _config: &ConversionConfig) -> Vec<String> {
 let mut warnings = Vec::new();
 
 // check大file
 for file in files {
 if file.size > 10 * 1024 * 1024 { // 10MB
 warnings.push(format!(
 "⚠️ File {} is large ({:.2} MB), PNG compression may take longer",
 file.name,
 file.size as f64 / (1024.0 * 1024.0)
 ));
 }
 }
 
 warnings
 }
 
 /// execute所 has formatspecificcheck
 pub fn check_all(files: &[InputFile], config: &ConversionConfig) -> Vec<String> {
 let format = config.format.to_lowercase();
 
 match format.as_str() {
 "webp" => Self::check_webp(files, config),
 "avif" => Self::check_avif(files, config),
 "jxl" | "jpegxl" => Self::check_jxl(files, config),
 "png" => Self::check_png(files, config),
 _ => Vec::new(),
 }
 }
}


#[cfg(test)]
mod tests {
 use super::*;
 use std::fs::File;
 use tempfile::TempDir;
 
 #[test]
 fn test_validation_display() {
 let result = ValidationResult {
 valid: true,
 level: 3,
 errors: vec![],
 warnings: vec!["Test warning".to_string()],
 };
 
 // thistest只isensure not will panic
 ValidationDisplay::display_result(&result);
 ValidationDisplay::display_summary(&result);
 ValidationDisplay::display_detailed_report(&result);
 }
 
 #[test]
 fn test_format_specific_checks_webp() {
 let temp_dir = TempDir::new().unwrap();
 let file_path = temp_dir.path().join("test.png");
 File::create(&file_path).unwrap();
 
 let files = vec![InputFile {
 file_path,
 name: "test.png".to_string(),
 ext: ".png".to_string(),
 size: 20 * 1024 * 1024, // 20MB
 is_animated: false,
 }];
 
 let config = ConversionConfig {
 format: "webp".to_string(),
 quality: Some(60),
 speed: Some(4),
 lossless: false,
 };
 
 let warnings = FormatSpecificChecks::check_webp(&files, &config);
 assert!(!warnings.is_empty());
 assert!(warnings.iter().any(|w| w.contains("large")));
 assert!(warnings.iter().any(|w| w.contains("quality setting is low")));
 }
 
 #[test]
 fn test_format_specific_checks_avif() {
 let files = vec![InputFile {
 file_path: PathBuf::from("test.gif"),
 name: "test.gif".to_string(),
 ext: ".gif".to_string(),
 size: 1024,
 is_animated: true,
 }];
 
 let config = ConversionConfig {
 format: "avif".to_string(),
 quality: Some(80),
 speed: Some(8),
 lossless: false,
 };
 
 let warnings = FormatSpecificChecks::check_avif(&files, &config);
 assert!(!warnings.is_empty());
 assert!(warnings.iter().any(|w| w.contains("animation")));
 assert!(warnings.iter().any(|w| w.contains("speed setting is high")));
 }
 
 #[test]
 fn test_format_specific_checks_jxl() {
 let files = vec![InputFile {
 file_path: PathBuf::from("test.jpg"),
 name: "test.jpg".to_string(),
 ext: ".jpg".to_string(),
 size: 1024,
 is_animated: false,
 }];
 
 let config = ConversionConfig {
 format: "jxl".to_string(),
 quality: Some(50),
 speed: Some(4),
 lossless: false,
 };
 
 let warnings = FormatSpecificChecks::check_jxl(&files, &config);
 assert!(!warnings.is_empty());
 assert!(warnings.iter().any(|w| w.contains("JPEG") || w.contains("quality")));
 }
}
