//! filetypedetectionmodule
//! 
//! useinferlibrary进line AI驱动filetypedetection

use anyhow::{Context, Result};
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};

/// filetypedetectionresult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure FileTypeDetection {
 /// AIdetectionfiletypelabel（如 "jpeg", "png", "gif"）
 pub detected_type: String,
 
 /// detection置信度 (0.0-1.0)
 pub confidence: f64,
 
 /// is否forhigh置信度detection（>0.9）
 pub is_high_confidence: bool,
 
 /// MIMEtype（如 "image/jpeg"）
 pub mime_type: Option<String>,
 
 /// detailed描述
 pub description: Option<String>,
 
 /// is否fortwo进制file
 pub is_binary: bool,
}

/// filesecurityvalidationresult
#[derive(Debug, Clone)]
pub structure SecurityValidation {
 /// is否passvalidation
 pub is_safe: bool,
 
 /// 扩展名anddetectiontypeis否match
 pub type_match: bool,
 
 /// detectionto扩展名
 pub expected_extension: Option<String>,
 
 /// actualdetectiontype
 pub detected_type: String,
 
 /// warninginformation
 pub warnings: Vec<String>,
 
 /// is否for可疑file（伪装、损坏 etc ）
 pub is_suspicious: bool,
}

/// filetypedetection
pub structure FileTypeDetector {
 /// minimum置信度threshold（below此value视for not 可靠）
 min_confidence: f64,
 
 /// is否enabled严格mode（detectionto not match when reject）
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
 
 /// createdefaultdetection（置信度 0.9，not严格mode）
 pub fn with_defaults() -> Self {
 Self::new(0.9, false)
 }
 
 /// detectionfiletype
 pub fn detect_file_type<P: AsRef<Path>>(&self, file_path: P) -> Result<FileTypeDetection> {
 let file_path = file_path.as_ref();
 
 // readfile before 几bytes进linedetection
 let file_data = fs::read(file_path)
 .context(format!("Failed to read file: {:?}", file_path))?;
 
 let detected_type = if let Some(kind) = infer::get(&file_data) {
 kind.extension().to_string()
 } else {
 // 回退tofile扩展名detection
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
 
 /// validationfilesecurity性（detection伪装file）
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
 
 /// check扩展名andtypeis否match
 pub fn extension_matches_type(&self, extension: &str, detected_type: &str) -> bool {
 let ext = extension.to_lowercase();
 let dtype = detected_type.to_lowercase();
 
 if ext == dtype {
 return true;
 }
 
 // 常见别名mapping
 matches!(
 (ext.as_str(), dtype.as_str()),
 ("jpg", "jpeg") | ("jpeg", "jpg") |
 ("tif", "tiff") | ("tiff", "tif") |
 ("htm", "html") | ("html", "htm") |
 ("mpg", "mpeg") | ("mpeg", "mpg")
 )
 }
 
 /// 判断is否for可executefiletype
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
