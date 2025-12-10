//! 🔍 validationSystem
//!
//! providemultilevelfile and imagevalidationfeature
//!
//! ## validationlevel
//!
//! 1. **Basic** - basiccheck(fileexists)
//! 2. **Format** - formatcheck
//! 3. **Integrity** - fullcheck(filehead/tail)
//! 4. **Deep** - depthcheck(fulldecoding)
//! 5. **Security** - securitycheck
//! 6. **Anti Cheat** - check
//! 7. **Dimensions** - dimensionvalidation(inputoutputa致)
//! 8. **Quality** - qualityvalidation(elementdata、SSIM)

use anyhow::Result;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::path::Path;
use super::modern_format_loader;

/// validationlevel
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ValidationLevel {
/// Level 1: basiccheck (exists)
 Basic = 1,
/// Level 2: formatcheck
 Format = 2,
/// Level 3: fullcheck (filehead/tail)
 Integrity = 3,
/// Level 4: depthcheck (fulldecoding)
 Deep = 4,
/// Level 5: securitycheck
 Security = 5,
/// Level 6: check
 AntiCheat = 6,
/// Level 7: dimensionvalidation (inputoutputa致)
 Dimensions = 7,
/// Level 8: qualityvalidation (elementdata、SSIM)
 Quality = 8,
}

/// validationresult
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ValidationResult {
/// isnopassvalidation
 pub passed: bool,
/// validationlevel
 pub level: u8,
/// detectiontoformat
 pub detected_format: Option<String>,
/// filesize
 pub file_size: u64,
/// imagedimension (width, height)
 pub dimensions: Option<(u32, u32)>,
/// isnoanimation
 pub is_animated: Option<bool>,

// Level 7 & 8: inputoutputcomparisonvalidation
/// inputfiledimension
 #[serde(skip_serializing_if = "Option::is_none")]
 pub input_dimensions: Option<(u32, u32)>,
/// outputfiledimension
 #[serde(skip_serializing_if = "Option::is_none")]
 pub output_dimensions: Option<(u32, u32)>,
/// dimensionisnomatch
 #[serde(skip_serializing_if = "Option::is_none")]
 pub dimensions_match: Option<bool>,
/// elementdataisno
 #[serde(skip_serializing_if = "Option::is_none")]
 pub metadata_preserved: Option<bool>,
/// qualityscore (SSIM, 0.0-1.0)
 #[serde(skip_serializing_if = "Option::is_none")]
 pub quality_score: Option<f64>,
/// qualityisnocanaccept
 #[serde(skip_serializing_if = "Option::is_none")]
 pub quality_acceptable: Option<bool>,

/// errorinformation
 pub errors: Vec<String>,
/// warninginformation
 pub warnings: Vec<String>,
}


/// filevalidation
pub struct FileValidator;

impl FileValidator {
/// validationfile - basiclevel
 pub fn validate_basic<P: AsRef<Path>>(path: P) -> Result<ValidationResult> {
 let path = path.as_ref();
 let mut result = ValidationResult {
 level: ValidationLevel::Basic as u8,
 ..Default::default()
 };

// Check if file exists
 if !path.exists() {
 result.errors.push("File does not exist".to_string());
 return Ok(result);
 }

// getfilesize
 let metadata = std::fs::metadata(path)?;
 result.file_size = metadata.len();

 if result.file_size == 0 {
 result.errors.push("File size is 0".to_string());
 return Ok(result);
 }

 result.passed = true;
 Ok(result)
 }

/// validationfile - formatlevel
 pub fn validate_format<P: AsRef<Path>>(path: P) -> Result<ValidationResult> {
 let path = path.as_ref();
 let mut result = Self::validate_basic(path)?;

 if !result.passed {
 return Ok(result);
 }

 result.level = ValidationLevel::Format as u8;

// detectionformat
 if let Some(ext) = path.extension() {
 result.detected_format = Some(ext.to_string_lossy().to_lowercase());
 } else {
 result.warnings.push("Unable to detect file format".to_string());
 }

 result.passed = result.detected_format.is_some();
 Ok(result)
 }

/// validationfile - depthlevel(fulldecoding)
 pub fn validate_deep<P: AsRef<Path>>(path: P) -> Result<ValidationResult> {
 let path = path.as_ref();
 let mut result = Self::validate_format(path)?;

 if !result.passed {
 return Ok(result);
 }

 result.level = ValidationLevel::Deep as u8;

// tryfulldecoding (支持 AVIF/JXL/HEIC 等现代格式)
 match modern_format_loader::load_image(path) {
 Ok(img) => {
 let (width, height) = img.dimensions();
 result.dimensions = Some((width, height));
 result.passed = true;
 }
 Err(e) => {
 result.errors.push(format!("Image decoding failed: {}", e));
 result.passed = false;
 }
 }

 Ok(result)
 }

/// validationinputoutputa致
 pub fn validate_consistency<P: AsRef<Path>>(
 input_path: P,
 output_path: P,
 ) -> Result<ValidationResult> {
 let input_path = input_path.as_ref();
 let output_path = output_path.as_ref();

 let mut result = Self::validate_deep(output_path)?;

 if !result.passed {
 return Ok(result);
 }

 result.level = ValidationLevel::Dimensions as u8;

// getinputdimension (支持现代格式)
 if let Ok(input_img) = modern_format_loader::load_image(input_path) {
 let (input_w, input_h) = input_img.dimensions();
 result.input_dimensions = Some((input_w, input_h));
 }

// checkdimensionisnomatch
 if let (Some(input_dims), Some(output_dims)) =
 (result.input_dimensions, result.dimensions)
 {
 result.dimensions_match = Some(input_dims == output_dims);
 result.passed = result.dimensions_match.unwrap_or(false);
 }

 Ok(result)
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_validation_level_ordering() {
 assert!(ValidationLevel::Basic < ValidationLevel::Format);
 assert!(ValidationLevel::Format < ValidationLevel::Deep);
 assert!(ValidationLevel::Deep < ValidationLevel::Quality);
 }

 #[test]
 fn test_validation_result_default() {
 let result = ValidationResult::default();
 assert!(!result.passed);
 assert_eq!(result.level, 0);
 assert!(result.errors.is_empty());
 }

 #[test]
 fn test_validate_nonexistent_file() {
 let result = FileValidator::validate_basic("/nonexistent/file.jpg");
 assert!(result.is_ok());
 let result = result.unwrap();
 assert!(!result.passed);
 assert!(!result.errors.is_empty());
 }
}
