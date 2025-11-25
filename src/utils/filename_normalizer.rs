//! 📝 file
//!
//! processing when usefile，completed after revertoriginalfile
//!
//! ## Corefeature
//!
//! - **filecleanup** - removed and 
//! - **temporarynamegenerate** - generatesecuritytemporaryfile
//! - **originalrecovery** - processingcompleted after revertoriginalfile
//! - **compatibility** - processingdifferentSystemfilelimit
//! - **lengthlimit** - autolongfile
//! - **aguarantee** - usehashensurea
//!
//! ## use
//!
//! - batchprocessingfile when file冲突
//! - processingcontainsfile
//! - file
//! - temporaryfilemanagement

use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// positivethen（needreplace）
fn dangerous_chars() -> &'static Regex {
 static REGEX: OnceLock<Regex> = OnceLock::new();
 REGEX.get_or_init(|| Regex::new(r#"[<>:"/\\|?*\x00-\x1F]"#).unwrap())
}

/// emptypositivethen
fn whitespace() -> &'static Regex {
 static REGEX: OnceLock<Regex> = OnceLock::new();
 REGEX.get_or_init(|| Regex::new(r"\s+").unwrap())
}

/// multidownline
fn multiple_underscores() -> &'static Regex {
 static REGEX: OnceLock<Regex> = OnceLock::new();
 REGEX.get_or_init(|| Regex::new(r"_{2,}").unwrap())
}

/// filemappingrecord
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilenameMapping {
/// originalfile
 pub original: String,
///  after file
 pub normalized: String,
/// originalpath
 pub original_path: PathBuf,
/// temporarypath
 pub temp_path: PathBuf,
}

/// file
pub struct FilenameNormalizer {
/// filemapping（original -> ）
 mappings: HashMap<String, FilenameMapping>,
/// maximumfilelength
 max_length: usize,
/// isnoextension
 preserve_extension: bool,
/// isnouse Magikaautodetectionextension
 use_magika: bool,
}

impl FilenameNormalizer {
/// createnew
 pub fn new() -> Self {
 Self {
 mappings: HashMap::new(),
 max_length: 200, // largemultifilelength
 preserve_extension: true,
 use_magika: true, // defaultenabledMagikaauto
 }
 }

/// enabled/disabled Magikaautodetection
 pub fn set_use_magika(&mut self, enabled: bool) {
 self.use_magika = enabled;
 }

/// settingmaximumfilelength
 pub fn set_max_length(&mut self, length: usize) {
 self.max_length = length;
 }

/// file（processing）
///
/// # then
/// 1. replacefordownline
/// 2. replaceemptyfordownline
/// 3. removedmultidownline
/// 4. limitlength
/// 5. usehashguaranteea
 pub fn normalize(&mut self, path: &Path) -> Result<PathBuf> {
 let original_name = path
 .file_name()
 .context("Invalid file path")?
 .to_string_lossy()
 .to_string();

// file and extension，anduse Magikavalidation
 let (base_name, extension) = if self.preserve_extension {
 if let Some(ext) = path.extension() {
 let ext_str = ext.to_string_lossy().to_string();
 let base = path.file_stem()
 .ok_or_else(|| anyhow::anyhow!("Invalid filename: no stem"))?
 .to_string_lossy()
 .to_string();

// use Magikavalidationextensionisnopositive
 let corrected_ext = if self.use_magika {
 self.detect_and_correct_extension(path, &ext_str)
 } else {
 ext_str
 };

 (base, Some(corrected_ext))
 } else {
//  has extension，tryMagikadetection
 let detected_ext = if self.use_magika {
 self.detect_extension(path)
 } else {
 None
 };

 (original_name.clone(), detected_ext)
 }
 } else {
 (original_name.clone(), None)
 };

// basicname
 let mut normalized = base_name.clone();

// 1. replace
 normalized = dangerous_chars().replace_all(&normalized, "_").to_string();

// 2. replaceempty
 normalized = whitespace().replace_all(&normalized, "_").to_string();

// 3. removedmultidownline
 normalized = multiple_underscores()
 .replace_all(&normalized, "_")
 .to_string();

// 4. removedfirsttaildownline
 normalized = normalized.trim_matches('_').to_string();

// 5. limitlength（extensionempty）
 let extension_length = extension.as_ref().map(|e| e.len() + 1).unwrap_or(0);
 let max_base_length = self.max_length.saturating_sub(extension_length);

 if normalized.len() > max_base_length {
// andaddhashbyguaranteea
 let hash = format!("{:x}", md5::compute(&normalized));
 let hash_suffix = &hash[..8]; // before8
 let truncate_len = max_base_length.saturating_sub(9); // 8哈希 + 1downline
 normalized = format!("{}_{}", &normalized[..truncate_len], hash_suffix);
 }

// 6. heavynewaddextension
 let final_name = if let Some(ext) = extension {
 format!("{}.{}", normalized, ext)
 } else {
 normalized
 };

// buildtemporarypath
 let parent = path.parent().unwrap_or_else(|| Path::new(""));
 let temp_path = parent.join(&final_name);

// savemapping
 let mapping = FilenameMapping {
 original: original_name.clone(),
 normalized: final_name,
 original_path: path.to_path_buf(),
 temp_path: temp_path.clone(),
 };

 self.mappings.insert(original_name, mapping);

 Ok(temp_path)
 }

/// revertoriginalfile
///
/// # 途
/// processingcompleted after ， will temporaryfilerevertfororiginalname
 pub fn restore(&self, temp_path: &Path) -> Result<PathBuf> {
 let temp_name = temp_path
 .file_name()
 .context("Invalid temp path")?
 .to_string_lossy()
 .to_string();

// findmapping
 for mapping in self.mappings.values() {
 if mapping.normalized == temp_name {
 let parent = temp_path.parent().unwrap_or_else(|| Path::new(""));
 return Ok(parent.join(&mapping.original));
 }
 }

 anyhow::bail!("No mapping found for temp file: {}", temp_name)
 }

/// getoriginalpath
 pub fn get_original_path(&self, normalized_name: &str) -> Option<&PathBuf> {
 for mapping in self.mappings.values() {
 if mapping.normalized == normalized_name {
 return Some(&mapping.original_path);
 }
 }
 None
 }

/// clear has mapping
 pub fn clear(&mut self) {
 self.mappings.clear();
 }

/// exportmapping（for）
 pub fn export_mappings(&self) -> Vec<FilenameMapping> {
 self.mappings.values().cloned().collect()
 }

/// importmapping
 pub fn import_mappings(&mut self, mappings: Vec<FilenameMapping>) {
 for mapping in mappings {
 self.mappings.insert(mapping.original.clone(), mapping);
 }
 }

/// getmappingcount
 pub fn mapping_count(&self) -> usize {
 self.mappings.len()
 }

/// checkisno has mapping
 pub fn has_mapping(&self, original_name: &str) -> bool {
 self.mappings.contains_key(original_name)
 }

/// use Magikadetectionandpositiveextension
 fn detect_and_correct_extension(&self, path: &Path, current_ext: &str) -> String {
 use crate::utils::magika_detector::MagikaDetector;

 let detector = MagikaDetector::with_defaults();
 match detector.detect_file_type(path) {
 Ok(detection) => {
// get Magikadetectiontoextension
 let detected_ext = &detection.detected_type;

// If detected extension differs from current, use detected one
 if detected_ext != &current_ext.to_lowercase() {
 log::info!("Extension corrected: .{} → .{} (confidence: {:.2}%)",
 current_ext, detected_ext, detection.confidence * 100.0);
 detected_ext.clone()
 } else {
 current_ext.to_string()
 }
 }
 Err(_) => {
// Magika detection failed, keep original extension
 current_ext.to_string()
 }
 }
 }

/// Use Magika to detect extension (for files without extension)
 fn detect_extension(&self, path: &Path) -> Option<String> {
 use crate::utils::magika_detector::MagikaDetector;

 let detector = MagikaDetector::with_defaults();
 match detector.detect_file_type(path) {
 Ok(detection) => {
 let ext = detection.detected_type.clone();
 log::info!("Extension detected: .{} (confidence: {:.2}%)",
 ext, detection.confidence * 100.0);
 Some(ext)
 }
 Err(_) => None
 }
 }
}

impl Default for FilenameNormalizer {
 fn default() -> Self {
 Self::new()
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_normalize_simple() {
 let mut normalizer = FilenameNormalizer::new();
 let path = Path::new("test file.jpg");
 let result = normalizer.normalize(path).unwrap();
 assert_eq!(result.file_name().unwrap().to_str().unwrap(), "test_file.jpg");
 }

 #[test]
 fn test_normalize_dangerous_chars() {
 let mut normalizer = FilenameNormalizer::new();
 let path = Path::new("test<>:file?.jpg");
 let result = normalizer.normalize(path).unwrap();
 let filename = result.file_name().unwrap().to_str().unwrap();
 assert!(!filename.contains('<'));
 assert!(!filename.contains('>'));
 assert!(!filename.contains(':'));
 assert!(!filename.contains('?'));
 }

 #[test]
 fn test_normalize_multiple_spaces() {
 let mut normalizer = FilenameNormalizer::new();
 let path = Path::new("test multiple spaces.jpg");
 let result = normalizer.normalize(path).unwrap();
 assert_eq!(
 result.file_name().unwrap().to_str().unwrap(),
 "test_multiple_spaces.jpg"
 );
 }

 #[test]
 fn test_normalize_long_filename() {
 let mut normalizer = FilenameNormalizer::new();
 normalizer.set_max_length(50);
 let long_name = "a".repeat(100);
 let path = PathBuf::from(format!("{}.jpg", long_name));
 let result = normalizer.normalize(&path).unwrap();
 let filename = result.file_name().unwrap().to_str().unwrap();
 assert!(filename.len() <= 50);
 assert!(filename.ends_with(".jpg"));
 }

 #[test]
 fn test_restore() {
 let mut normalizer = FilenameNormalizer::new();
 let original = Path::new("test file.jpg");
 let normalized = normalizer.normalize(original).unwrap();
 let restored = normalizer.restore(&normalized).unwrap();
 assert_eq!(
 restored.file_name().unwrap().to_str().unwrap(),
 "test file.jpg"
 );
 }

 #[test]
 fn test_export_import_mappings() {
 let mut normalizer1 = FilenameNormalizer::new();
 normalizer1.normalize(Path::new("test1.jpg")).unwrap();
 normalizer1.normalize(Path::new("test2.jpg")).unwrap();

 let mappings = normalizer1.export_mappings();
 assert_eq!(mappings.len(), 2);

 let mut normalizer2 = FilenameNormalizer::new();
 normalizer2.import_mappings(mappings);
 assert_eq!(normalizer2.mapping_count(), 2);
 }

 #[test]
 fn test_clear_mappings() {
 let mut normalizer = FilenameNormalizer::new();
 normalizer.normalize(Path::new("test.jpg")).unwrap();
 assert_eq!(normalizer.mapping_count(), 1);
 normalizer.clear();
 assert_eq!(normalizer.mapping_count(), 0);
 }
}
