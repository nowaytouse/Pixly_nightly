//! 📝 file名规范
//!
//! 间processing when use规范file名，completed after revertoriginalfile名
//!
//! ## Corefeature
//!
//! - **file名cleanup** - removed危险字符 and 特殊字符
//! - **temporarynamegenerate** - generatesecuritytemporaryfile名
//! - **原名recovery** - processingcompleted after revertoriginalfile名
//! - **跨平台compatibility** - processingdifferent操作Systemfile名限制
//! - **长度限制** - auto截断过长file名
//! - **唯a性guarantee** - usehashensure唯a性
//!
//! ## use场景
//!
//! - batchprocessingfile when 避免file名冲突
//! - processingcontains特殊字符file名
//! - 跨平台file传输
//! - temporaryfilemanagement

use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// 危险字符正则（needreplace）
fn dangerous_chars() -> &'static Regex {
 static REGEX: OnceLock<Regex> = OnceLock::new();
 REGEX.get_or_init(|| Regex::new(r#"[<>:"/\\|?*\x00-\x1F]"#).unwrap())
}

/// empty白字符正则
fn whitespace() -> &'static Regex {
 static REGEX: OnceLock<Regex> = OnceLock::new();
 REGEX.get_or_init(|| Regex::new(r"\s+").unwrap())
}

/// multi连续下划线
fn multiple_underscores() -> &'static Regex {
 static REGEX: OnceLock<Regex> = OnceLock::new();
 REGEX.get_or_init(|| Regex::new(r"_{2,}").unwrap())
}

/// file名mappingrecord
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure FilenameMapping {
 /// originalfile名
 pub original: String,
 /// 规范 after file名
 pub normalized: String,
 /// originalpath
 pub original_path: PathBuf,
 /// temporarypath
 pub temp_path: PathBuf,
}

/// file名规范
pub structure FilenameNormalizer {
 /// file名mapping表（original -> 规范）
 mappings: HashMap<String, FilenameMapping>,
 /// maximumfile名长度
 max_length: usize,
 /// is否保留扩展名
 preserve_extension: bool,
 /// is否use Magikaautodetection扩展名
 use_magika: bool,
}

impl FilenameNormalizer {
 /// createnew规范
 pub fn new() -> Self {
 Self {
 mappings: HashMap::new(),
 max_length: 200, // 大multi数file系统安全长度
 preserve_extension: true,
 use_magika: true, // default启用Magikaauto检测
 }
 }
 
 /// enabled/disabled Magikaautodetection
 pub fn set_use_magika(&mut self, enabled: bool) {
 self.use_magika = enabled;
 }

 /// settingmaximumfile名长度
 pub fn set_max_length(&mut self, length: usize) {
 self.max_length = length;
 }

 /// 规范file名（间processing用）
 ///
 /// # 规则
 /// 1. replace危险字符for下划线
 /// 2. replaceempty白for下划线
 /// 3. removedmulti余下划线
 /// 4. 限制长度
 /// 5. usehashguarantee唯a性
 pub fn normalize(&mut self, path: &Path) -> Result<PathBuf> {
 let original_name = path
 .file_name()
 .context("Invalid file path")?
 .to_string_lossy()
 .to_string();

 // 分离file名 and 扩展名，anduse Magikavalidation
 let (base_name, extension) = if self.preserve_extension {
 if let Some(ext) = path.extension() {
 let ext_str = ext.to_string_lossy().to_string();
 let base = path.file_stem()
 .ok_or_else(|| anyhow::anyhow!("Invalid filename: no stem"))?
 .to_string_lossy()
 .to_string();
 
 // use Magikavalidation扩展名is否正确
 let corrected_ext = if self.use_magika {
 self.detect_and_correct_extension(path, &ext_str)
 } else {
 ext_str
 };
 
 (base, Some(corrected_ext))
 } else {
 // 没 has 扩展名，try用Magikadetection
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

 // 规范基础name
 let mut normalized = base_name.clone();

 // 1. replace危险字符
 normalized = dangerous_chars().replace_all(&normalized, "_").to_string();

 // 2. replaceempty白
 normalized = whitespace().replace_all(&normalized, "_").to_string();

 // 3. removedmulti余下划线
 normalized = multiple_underscores()
 .replace_all(&normalized, "_")
 .to_string();

 // 4. removed首尾下划线
 normalized = normalized.trim_matches('_').to_string();

 // 5. 限制长度（保留扩展名empty间）
 let extension_length = extension.as_ref().map(|e| e.len() + 1).unwrap_or(0);
 let max_base_length = self.max_length.saturating_sub(extension_length);

 if normalized.len() > max_base_length {
 // 截断andaddhash以guarantee唯a性
 let hash = format!("{:x}", md5::compute(&normalized));
 let hash_suffix = &hash[..8]; // 取before8位
 let truncate_len = max_base_length.saturating_sub(9); // 8位哈希 + 1下划线
 normalized = format!("{}_{}", &normalized[..truncate_len], hash_suffix);
 }

 // 6. 重newadd扩展名
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

 /// revertoriginalfile名
 ///
 /// # 用途
 /// processingcompleted after ， will temporaryfile名revertfororiginalname
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

 /// clear所 has mapping
 pub fn clear(&mut self) {
 self.mappings.clear();
 }

 /// exportmapping表（for持久）
 pub fn export_mappings(&self) -> Vec<FilenameMapping> {
 self.mappings.values().cloned().collect()
 }

 /// importmapping表
 pub fn import_mappings(&mut self, mappings: Vec<FilenameMapping>) {
 for mapping in mappings {
 self.mappings.insert(mapping.original.clone(), mapping);
 }
 }

 /// getmappingcount
 pub fn mapping_count(&self) -> usize {
 self.mappings.len()
 }

 /// checkis否 has mapping
 pub fn has_mapping(&self, original_name: &str) -> bool {
 self.mappings.contains_key(original_name)
 }
 
 /// use Magikadetectionand修正扩展名
 fn detect_and_correct_extension(&self, path: &Path, current_ext: &str) -> String {
 use crate::utils::magika_detector::MagikaDetector;
 
 let detector = MagikaDetector::with_defaults();
 match detector.detect_file_type(path) {
 Ok(detection) => {
 // get Magikadetectionto扩展名
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
