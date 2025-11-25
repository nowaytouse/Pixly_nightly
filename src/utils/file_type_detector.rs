//! filetypedetectionmodule
//!
//! useinferlibraryline AIfiletypedetection

use anyhow::{Context, Result};
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};

/// filetypedetectionresult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeDetection {
/// AIdetectionfiletypelabel（like "jpeg", "png", "gif"）
 pub detected_type: String,

/// detectionconfidence (0.0-1.0)
 pub confidence: f64,

/// isnoforhighconfidencedetection（>0.9）
 pub is_high_confidence: bool,

/// MIMEtype（like "image/jpeg"）
 pub mime_type: Option<String>,

/// detaileddescription
 pub description: Option<String>,

/// isnofortwo制file
 pub is_binary: bool,
}

/// filesecurityvalidationresult
#[derive(Debug, Clone)]
pub struct SecurityValidation {
/// isnopassvalidation
 pub is_safe: bool,

/// extensionanddetectiontypeisnomatch
 pub type_match: bool,

/// detectiontoextension
 pub expected_extension: Option<String>,

/// actualdetectiontype
 pub detected_type: String,

/// warninginformation
 pub warnings: Vec<String>,

/// isnoforcanfile（、 etc ）
 pub is_suspicious: bool,
}

/// filetypedetection
pub struct FileTypeDetector {
/// minimumconfidencethreshold（belowthisvaluefor not can）
 min_confidence: f64,

/// isnoenabledmode（detectionto not match when reject）
 strict_mode: bool,
}

impl FileTypeDetector {
/// createnewdetection
 pub fn new(min_confidence: f64, strict_mode: bool) -> Self {
 Self {
 min_confidence,
 strict_mode,
 }
 }

/// createdefaultdetection（confidence 0.9，notmode）
 pub fn with_defaults() -> Self {
 Self::new(0.9, false)
 }

/// detectionfiletype
 pub fn detect_file_type<P: AsRef<Path>>(&self, file_path: P) -> Result<FileTypeDetection> {
 let file_path = file_path.as_ref();

// readfile before how manybyteslinedetection
 let file_data = fs::read(file_path)
 .context(format!("Failed to read file: {:?}", file_path))?;

 let detected_type = if let Some(kind) = infer::get(&file_data) {
 kind.extension().to_string()
 } else {
// tofileextensiondetection
 file_path.extension()
 .and_then(|ext| ext.to_str())
 .unwrap_or("unknown")
 .to_string()
 };

 let confidence = if detected_type != "unknown" { 0.95 } else { 0.1 };

 let detection = FileTypeDetection {
 detected_type: detected_type.clone(),
 confidence,
 is_high_confidence: confidence >= self.min_confidence,
 mime_type: infer::get(&file_data).map(|kind| kind.mime_type().to_string()),
 description: Some(format!("File type: {}", detected_type)),
 is_binary: !detected_type.starts_with("text"),
 };

 Ok(detection)
 }

/// validationfilesecurity（detectionfile）
 pub fn validate_security(
 &self,
 file_path: &Path,
 expected_extension: Option<&str>,
 ) -> Result<SecurityValidation> {
 let detection = self.detect_file_type(file_path)?;

 let file_extension = file_path
 .extension()
 .and_then(|e| e.to_str())
 .map(|e| e.to_lowercase());

 let mut warnings = Vec::new();
 let mut is_suspicious = false;

 let type_match = if let Some(expected) = expected_extension {
 let matches = self.extension_matches_type(expected, &detection.detected_type);

 if !matches && detection.is_high_confidence {
 warnings.push(format!(
 "File type mismatch: Expected '{}' but detected '{}' (confidence: {:.1}%)",
 expected,
 detection.detected_type,
 detection.confidence * 100.0
 ));
 is_suspicious = true;
 }

 matches
 } else if let Some(file_ext) = &file_extension {
 let matches = self.extension_matches_type(file_ext, &detection.detected_type);

 if !matches && detection.is_high_confidence {
 warnings.push(format!(
 "Extension mismatch: File has '.{}' but detected as '{}' (confidence: {:.1}%)",
 file_ext,
 detection.detected_type,
 detection.confidence * 100.0
 ));
 is_suspicious = true;
 }

 matches
 } else {
 if detection.is_high_confidence {
 warnings.push(format!(
 "No extension found, detected as '{}' (confidence: {:.1}%)",
 detection.detected_type,
 detection.confidence * 100.0
 ));
 }
 true
 };

 if self.is_executable_type(&detection.detected_type) && detection.is_high_confidence {
 warnings.push(format!(
 "SECURITY WARNING: Detected as executable type '{}'",
 detection.detected_type
 ));
 is_suspicious = true;
 }

 if !detection.is_high_confidence {
 warnings.push(format!(
 "Low confidence detection: {:.1}% (threshold: {:.1}%)",
 detection.confidence * 100.0,
 self.min_confidence * 100.0
 ));
 }

 let is_safe = if self.strict_mode {
 type_match && !is_suspicious && detection.is_high_confidence
 } else {
 !self.is_executable_type(&detection.detected_type) || type_match
 };

 Ok(SecurityValidation {
 is_safe,
 type_match,
 expected_extension: expected_extension.map(|s| s.to_string()),
 detected_type: detection.detected_type,
 warnings,
 is_suspicious,
 })
 }

/// checkextensionandtypeisnomatch
 pub fn extension_matches_type(&self, extension: &str, detected_type: &str) -> bool {
 let ext = extension.to_lowercase();
 let dtype = detected_type.to_lowercase();

 if ext == dtype {
 return true;
 }

// aliasmapping
 matches!(
 (ext.as_str(), dtype.as_str()),
 ("jpg", "jpeg") | ("jpeg", "jpg") |
 ("tif", "tiff") | ("tiff", "tif") |
 ("htm", "html") | ("html", "htm") |
 ("mpg", "mpeg") | ("mpeg", "mpg")
 )
 }

/// isnoforcanexecutefiletype
 fn is_executable_type(&self, detected_type: &str) -> bool {
 matches!(
 detected_type.to_lowercase().as_str(),
 "exe" | "dll" | "so" | "dylib" | "sh" | "bat" | "cmd" | "msi" | "app"
 )
 }
}

impl Default for FileTypeDetector {
 fn default() -> Self {
 Self::with_defaults()
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_extension_matching() {
 let detector = FileTypeDetector::with_defaults();
 assert!(detector.extension_matches_type("jpg", "jpeg"));
 assert!(detector.extension_matches_type("jpeg", "jpg"));
 assert!(!detector.extension_matches_type("png", "jpeg"));
 }
}
