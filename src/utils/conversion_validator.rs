/**
 * 🔥 multilevelconvertvalidate (Multi-Level Conversion Validator)
 *
 * from plugin/converter/js/plugin-modules/conversion-validator.js 
 * Phase 40.33: enhancedconvertcan
 *
 * validatelayerlevel:
 * - Level 1: inputfilevalidate (fileformat、size、path、)
 * - Level 2: convertparametervalidate (quality、speed、formatmatch)
 * - Level 3: modeavalidate (manual/intelligentmodeparametermatch)
 * - Level 4: outputvalidate (filegenerate、size、formatpositive)
 */
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};

/// validationresult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
/// isnopassvalidation
 pub valid: bool,
/// validationlevel (1-4)
 pub level: u8,
/// errorlist
 pub errors: Vec<String>,
/// warninglist
 pub warnings: Vec<String>,
}

impl ValidationResult {
 pub fn success(level: u8) -> Self {
 Self {
 valid: true,
 level,
 errors: Vec::new(),
 warnings: Vec::new(),
 }
 }

 pub fn failure(level: u8, errors: Vec<String>) -> Self {
 Self {
 valid: false,
 level,
 errors,
 warnings: Vec::new(),
 }
 }

 pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
 self.warnings = warnings;
 self
 }
}

/// inputfileinformation
#[derive(Debug, Clone)]
pub struct InputFile {
 pub file_path: PathBuf,
 pub name: String,
 pub ext: String,
 pub size: u64,
 pub is_animated: bool,
}

/// conversionconfiguration
#[derive(Debug, Clone)]
pub struct ConversionConfig {
 pub format: String,
 pub quality: Option<u32>,
 pub speed: Option<u32>,
 pub lossless: bool,
}

/// conversionmode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionMode {
 Manual,
 Smart,
 AI,
}

/// multilevelvalidation
pub struct ConversionValidator;

impl ConversionValidator {
/// 🔍 Level 1: inputfilevalidation
 pub fn validate_input_files(files: &[InputFile]) -> ValidationResult {
// 🔥 performanceoptimization：
 let mut errors = Vec::with_capacity(files.len());
 let mut warnings = Vec::with_capacity(files.len());

 if files.is_empty() {
 errors.push("❌ No files selected".to_string());
 return ValidationResult::failure(1, errors);
 }

 for (index, file) in files.iter().enumerate() {
 let file_num = index + 1;

// validationfilepath
 if file.file_path.as_os_str().is_empty() {
 errors.push(format!("❌ File #{}: Invalid file path", file_num));
 }

// validationfileextension
 if file.ext.is_empty() {
 errors.push(format!("❌ File #{} ({}): Missing file extension", file_num, file.name));
 }

// validationfilesize
 if file.size == 0 {
 warnings.push(format!("⚠️ File #{} ({}): File size is 0", file_num, file.name));
 } else if file.size > 1024 * 1024 * 1024 { // 1GB
 let size_gb = file.size as f64 / (1024.0 * 1024.0 * 1024.0);
 warnings.push(format!("⚠️ File #{} ({}): File too large ({:.2} GB)", file_num, file.name, size_gb));
 }

// validationfile
 if file.name.chars().any(|c| matches!(c, '<' | '>' | ':' | '"' | '|' | '?' | '*')) {
 warnings.push(format!("⚠️ File #{} ({}): Filename contains illegal characters", file_num, file.name));
 }

// validationfileisnoexists
 if !file.file_path.exists() {
 errors.push(format!("❌ File #{} ({}): File does not exist", file_num, file.name));
 }

// validationfileisnocan
 if file.file_path.exists()
 && let Err(e) = fs::metadata(&file.file_path) {
 errors.push(format!("❌ File #{} ({}): Cannot read file metadata: {}", file_num, file.name, e));
 }
 }

 if errors.is_empty() {
 ValidationResult::success(1).with_warnings(warnings)
 } else {
 ValidationResult::failure(1, errors).with_warnings(warnings)
 }
 }

/// 🔍 Level 2: conversionparametervalidation
 pub fn validate_parameters(config: &ConversionConfig, files: &[InputFile]) -> ValidationResult {
// 🔥 performanceoptimization：
 let mut errors = Vec::with_capacity(4);
 let mut warnings = Vec::with_capacity(4);

// validationtargetformat
 let valid_formats = ["jxl", "avif", "webp", "heic", "png", "jpg", "jpeg"];
 let format_lower = config.format.to_lowercase();
 if !valid_formats.contains(&format_lower.as_str()) {
 errors.push(format!("❌ Invalid target format: {}", config.format));
 }

// validationqualityparameter
 if let Some(quality) = config.quality
 && (!(1..=100).contains(&quality)) {
 errors.push(format!("❌ Invalid quality parameter: {} (should be 1-100)", quality));
 }

// validationspeedparameter
 if let Some(speed) = config.speed
 && speed > 10 {
 errors.push(format!("❌ Invalid speed parameter: {} (should be 0-10)", speed));
 }

// validationformatcompatibility
 if format_lower == "heic" {
// detectiontransparencydegree
 let has_transparency = files.iter().any(|f| {
 let ext = f.ext.to_lowercase();
 ext == ".png" || ext == ".gif" || ext == ".webp"
 });
 if has_transparency {
 warnings.push("⚠️ HEIC does not support transparency, transparent backgrounds may change color".to_string());
 }

// detectionanimation
 let has_animation = files.iter().any(|f| f.is_animated);
 if has_animation {
 warnings.push("⚠️ HEIC does not support animation, animation effects will be lost".to_string());
 }
 }

// validationlosslessmodecompatibility
 if config.lossless
 && (format_lower == "jpg" || format_lower == "jpeg") {
 errors.push("❌ JPEG format does not support lossless mode".to_string());
 }

// validation AVIF animationsupport
 if format_lower == "avif" {
 let has_animation = files.iter().any(|f| f.is_animated);
 if has_animation {
 warnings.push("⚠️ AVIF animation support is limited, may require special encoder".to_string());
 }
 }

// validation JXL parameter
 if (format_lower == "jxl" || format_lower == "jpegxl")
 && let Some(quality) = config.quality
 && quality < 60 {
 warnings.push(format!("⚠️ JXL quality parameter is low ({}), may affect visual quality", quality));
 }

 if errors.is_empty() {
 ValidationResult::success(2).with_warnings(warnings)
 } else {
 ValidationResult::failure(2, errors).with_warnings(warnings)
 }
 }

/// 🔍 Level 3: modeandparametermatchvalidation
 pub fn validate_mode_consistency(mode: ConversionMode, config: &ConversionConfig) -> ValidationResult {
 let mut errors = Vec::new();
 let mut warnings = Vec::new();

 match mode {
 ConversionMode::Manual => {
// manualmode：validation has requiredparameteralreadysetting
 if config.quality.is_none() {
 warnings.push("⚠️ Quality parameter not set in manual mode, will use default value".to_string());
 }
 if config.format.is_empty() {
 errors.push("❌ Target format must be specified in manual mode".to_string());
 }
 }
 ConversionMode::Smart | ConversionMode::AI => {
// intelligentmode：validation AI predictionwant
// note：thiscanadd AI availablecheck
 warnings.push("⚠️ Smart mode will use AI predicted parameters".to_string());
 }
 }

 if errors.is_empty() {
 ValidationResult::success(3).with_warnings(warnings)
 } else {
 ValidationResult::failure(3, errors).with_warnings(warnings)
 }
 }

/// 🔍 Level 4: outputvalidation（conversion after ）
 pub fn validate_output(output_path: &Path, expected_size: Option<u64>) -> ValidationResult {
 let mut errors = Vec::new();
 let mut warnings = Vec::new();

// validationfileisnoexists
 if !output_path.exists() {
 errors.push(format!("❌ Output file not generated: {}", output_path.display()));
 return ValidationResult::failure(4, errors);
 }

// validationfilesize
 match fs::metadata(output_path) {
 Ok(metadata) => {
 let actual_size = metadata.len();

// validationfilesize not for0
 if actual_size == 0 {
 errors.push(format!("❌ Output file size is 0: {}", output_path.display()));
 }

// validationfilesize合理
 if let Some(expected) = expected_size
 && expected > 0 {
 let ratio = actual_size as f64 / expected as f64;
 if ratio < 0.01 {
 warnings.push(format!("⚠️ Output file abnormally small ({:.2}% of original)", ratio * 100.0));
 } else if ratio > 10.0 {
 warnings.push(format!("⚠️ Output file abnormally large ({:.2}x of original)", ratio));
 }
 }
 }
 Err(e) => {
 errors.push(format!("❌ Output validation failed: {}", e));
 }
 }

 if errors.is_empty() {
 ValidationResult::success(4).with_warnings(warnings)
 } else {
 ValidationResult::failure(4, errors).with_warnings(warnings)
 }
 }

/// 🔍 fullvalidation
 pub fn validate_full_conversion(
 files: &[InputFile],
 config: &ConversionConfig,
 mode: ConversionMode,
 ) -> ValidationResult {
 tracing::info!("🔍 Starting multi-level validation");

// Level 1: inputvalidation
 let input_validation = Self::validate_input_files(files);
 if !input_validation.valid {
 tracing::error!("❌ Level 1 failed: Input file validation failed");
 return input_validation;
 }
 tracing::info!("✅ Level 1 passed: Input file validation");

// Level 2: parametervalidation
 let param_validation = Self::validate_parameters(config, files);
 if !param_validation.valid {
 tracing::error!("❌ Level 2 failed: Parameter validation failed");
 let mut result = param_validation;
 result.warnings.extend(input_validation.warnings);
 return result;
 }
 tracing::info!("✅ Level 2 passed: Parameter validation");

// Level 3: modeavalidation
 let mode_validation = Self::validate_mode_consistency(mode, config);
 if !mode_validation.valid {
 tracing::error!("❌ Level 3 failed: Mode consistency validation failed");
 let mut result = mode_validation;
 result.warnings.extend(input_validation.warnings);
 result.warnings.extend(param_validation.warnings);
 return result;
 }
 tracing::info!("✅ Level 3 passed: Mode consistency validation");

// 收集 has warning
 let mut all_warnings = input_validation.warnings;
 all_warnings.extend(param_validation.warnings);
 all_warnings.extend(mode_validation.warnings);

 tracing::info!("🎉 All validation levels passed");
 if !all_warnings.is_empty() {
 tracing::warn!("⚠️ Warnings: {:?}", all_warnings);
 }

 ValidationResult::success(3).with_warnings(all_warnings)
 }
}

#[cfg(test)]
mod tests {
 use super::*;
 use std::fs::File;
 use std::io::Write;
 use tempfile::TempDir;

 #[test]
 fn test_validate_empty_files() {
 let files = vec![];
 let result = ConversionValidator::validate_input_files(&files);
 assert!(!result.valid);
 assert_eq!(result.level, 1);
 assert!(!result.errors.is_empty());
 }

 #[test]
 fn test_validate_valid_files() {
 let temp_dir = TempDir::new().unwrap();
 let file_path = temp_dir.path().join("test.png");
 File::create(&file_path).unwrap();

 let files = vec![InputFile {
 file_path: file_path.clone(),
 name: "test.png".to_string(),
 ext: ".png".to_string(),
 size: 1024,
 is_animated: false,
 }];

 let result = ConversionValidator::validate_input_files(&files);
 assert!(result.valid);
 assert_eq!(result.level, 1);
 }

 #[test]
 fn test_validate_invalid_format() {
 let config = ConversionConfig {
 format: "invalid".to_string(),
 quality: Some(80),
 speed: Some(4),
 lossless: false,
 };

 let result = ConversionValidator::validate_parameters(&config, &[]);
 assert!(!result.valid);
 assert_eq!(result.level, 2);
 assert!(result.errors.iter().any(|e| e.contains("Invalid target format")));
 }

 #[test]
 fn test_validate_quality_range() {
 let config = ConversionConfig {
 format: "webp".to_string(),
 quality: Some(150),
 speed: Some(4),
 lossless: false,
 };

 let result = ConversionValidator::validate_parameters(&config, &[]);
 assert!(!result.valid);
 assert!(result.errors.iter().any(|e| e.contains("Invalid quality parameter")));
 }

 #[test]
 fn test_validate_jpeg_lossless() {
 let config = ConversionConfig {
 format: "jpeg".to_string(),
 quality: Some(90),
 speed: Some(4),
 lossless: true,
 };

 let result = ConversionValidator::validate_parameters(&config, &[]);
 assert!(!result.valid);
 assert!(result.errors.iter().any(|e| e.contains("JPEG format does not support lossless mode")));
 }

 #[test]
 fn test_validate_heic_transparency_warning() {
 let temp_dir = TempDir::new().unwrap();
 let file_path = temp_dir.path().join("test.png");
 File::create(&file_path).unwrap();

 let files = vec![InputFile {
 file_path,
 name: "test.png".to_string(),
 ext: ".png".to_string(),
 size: 1024,
 is_animated: false,
 }];

 let config = ConversionConfig {
 format: "heic".to_string(),
 quality: Some(80),
 speed: Some(4),
 lossless: false,
 };

 let result = ConversionValidator::validate_parameters(&config, &files);
 assert!(result.valid);
 assert!(result.warnings.iter().any(|w| w.contains("HEIC does not support transparency")));
 }

 #[test]
 fn test_validate_output_missing_file() {
 let result = ConversionValidator::validate_output(Path::new("/nonexistent/file.webp"), None);
 assert!(!result.valid);
 assert_eq!(result.level, 4);
 assert!(result.errors.iter().any(|e| e.contains("Output file not generated")));
 }

 #[test]
 fn test_validate_output_zero_size() {
 let temp_dir = TempDir::new().unwrap();
 let file_path = temp_dir.path().join("empty.webp");
 File::create(&file_path).unwrap(); // createemptyfile

 let result = ConversionValidator::validate_output(&file_path, None);
 assert!(!result.valid);
 assert!(result.errors.iter().any(|e| e.contains("Output file size is 0")));
 }

 #[test]
 fn test_validate_output_size_ratio() {
 let temp_dir = TempDir::new().unwrap();
 let file_path = temp_dir.path().join("test.webp");
 let mut file = File::create(&file_path).unwrap();
 file.write_all(&[0u8; 10]).unwrap(); // write10bytes

// 10 bytes vs 10,000 expected = 0.001 ratio (< 0.01)
 let result = ConversionValidator::validate_output(&file_path, Some(10_000));
 assert!(result.valid);
 assert!(result.warnings.iter().any(|w| w.contains("Output file abnormally small")),
 "Expected warning about small file, got: {:?}", result.warnings);
 }

 #[test]
 fn test_full_validation_success() {
 let temp_dir = TempDir::new().unwrap();
 let file_path = temp_dir.path().join("test.png");
 File::create(&file_path).unwrap();

 let files = vec![InputFile {
 file_path,
 name: "test.png".to_string(),
 ext: ".png".to_string(),
 size: 1024,
 is_animated: false,
 }];

 let config = ConversionConfig {
 format: "webp".to_string(),
 quality: Some(80),
 speed: Some(4),
 lossless: false,
 };

 let result = ConversionValidator::validate_full_conversion(&files, &config, ConversionMode::Manual);
 assert!(result.valid);
 assert_eq!(result.level, 3);
 }
}
