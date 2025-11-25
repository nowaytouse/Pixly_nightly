/// formatautopositivemodule
///
/// feature：
/// 1. detectionfileextensionandactualformatisnomatch
/// 2. autopositiveerrorextension
/// 3. processingformatissue
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::fs;

/// formatpositiveresult
#[derive(Debug, Clone)]
pub struct FormatCorrectionResult {
/// isnoneedpositive
 pub needs_correction: bool,

/// originalextension
 pub original_extension: String,

/// detectiontoactualformat
 pub detected_format: String,

/// positive after path（ifneedpositive）
 pub corrected_path: Option<PathBuf>,

/// positivedescription
 pub message: String,
}

/// formatpositive
pub struct FormatCorrector {
/// isnoautorenamedfile
 auto_rename: bool,
}

impl FormatCorrector {
/// createnewformatpositive
 pub fn new(auto_rename: bool) -> Self {
 Self { auto_rename }
 }

/// checkandpositivefileformat
 pub fn check_and_correct(&self, path: &Path) -> Result<FormatCorrectionResult> {
// 1. getfileextension
 let original_ext = path
 .extension()
 .and_then(|e| e.to_str())
 .unwrap_or("")
 .to_lowercase();

 if original_ext.is_empty() {
 return Ok(FormatCorrectionResult {
 needs_correction: false,
 original_extension: String::new(),
 detected_format: String::new(),
 corrected_path: None,
 message: "No extension to check".to_string(),
 });
 }

// 2. detectionactualformat
 let detected_format = self.detect_actual_format(path)?;

// 3. compareextension and actualformat
 if self.formats_match(&original_ext, &detected_format) {
 return Ok(FormatCorrectionResult {
 needs_correction: false,
 original_extension: original_ext,
 detected_format,
 corrected_path: None,
 message: "Extension matches actual format".to_string(),
 });
 }

// 4. needpositive
 let corrected_path = if self.auto_rename {
 Some(self.rename_file(path, &detected_format)?)
 } else {
 Some(path.with_extension(&detected_format))
 };

 Ok(FormatCorrectionResult {
 needs_correction: true,
 original_extension: original_ext.clone(),
 detected_format: detected_format.clone(),
 corrected_path,
 message: format!(
 "Format mismatch: extension '{}' but actual format is '{}'",
 original_ext, detected_format
 ),
 })
 }

/// detectionfileactualformat
 fn detect_actual_format(&self, path: &Path) -> Result<String> {
// readfileheadbytes
 let mut file = fs::File::open(path)?;
 let mut buffer = [0u8; 12];
 use std::io::Read;
 let bytes_read = file.read(&mut buffer)?;

// ensurereadbytes
 if bytes_read < 4 {
 return Err(anyhow::anyhow!("File too small to detect format"));
 }

// based onformat
 let format = match &buffer[0..4] {
// PNG: 89 50 4E 47
 [0x89, 0x50, 0x4E, 0x47] => "png",
// JPEG: FF D8 FF
 [0xFF, 0xD8, 0xFF, _] => "jpg",
// GIF: 47 49 46 38
 [0x47, 0x49, 0x46, 0x38] => "gif",
// WebP: RIFF....WEBP
 [0x52, 0x49, 0x46, 0x46] if &buffer[8..12] == b"WEBP" => "webp",
// AVIF: ....ftypavif
 _ if buffer.windows(8).any(|w| w == b"ftypavif") => "avif",
// JXL: FF 0A or 00 00 00 0C 4A 58 4C 20
 [0xFF, 0x0A, ..] => "jxl",
 [0x00, 0x00, 0x00, 0x0C] if &buffer[4..8] == b"JXL " => "jxl",
// HEIC: ....ftypheic
 _ if buffer.windows(8).any(|w| w == b"ftypheic") => "heic",
// default：use infer crate
 _ => {
 use infer;
 if let Some(kind) = infer::get(&buffer) {
 kind.extension()
 } else {
 "unknown"
 }
 }
 };

 Ok(format.to_string())
 }

/// checkformatisnomatch
 fn formats_match(&self, ext: &str, format: &str) -> bool {
// processingalias
 let normalized_ext = self.normalize_format(ext);
 let normalized_format = self.normalize_format(format);

 normalized_ext == normalized_format
 }

/// formatname（processingalias）
// 🚀 performanceoptimization: use&'static str
 fn normalize_format(&self, format: &str) -> String {
 match format {
 "jpg" | "jpeg" => String::from("jpeg"),
 "tif" | "tiff" => String::from("tiff"),
 _ => format.to_lowercase(),
 }
 }

/// renamedfile
 fn rename_file(&self, path: &Path, new_ext: &str) -> Result<PathBuf> {
 let new_path = path.with_extension(new_ext);

 fs::rename(path, &new_path)
 .with_context(|| format!("Failed to rename {:?} to {:?}", path, new_path))?;

 Ok(new_path)
 }
}

impl Default for FormatCorrector {
 fn default() -> Self {
 Self::new(false)
 }
}

#[cfg(test)]
mod tests {
 use super::*;
 use std::io::Write;
 use tempfile::tempdir;

 #[test]
 fn test_detect_png_format() {
 let dir = tempdir().unwrap();
 let file_path = dir.path().join("test.jpg"); // errorextension

// write PNG 
 let mut file = fs::File::create(&file_path).unwrap();
 file.write_all(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]).unwrap();

 let corrector = FormatCorrector::new(false);
 let result = corrector.check_and_correct(&file_path).unwrap();

 assert!(result.needs_correction);
 assert_eq!(result.original_extension, "jpg");
 assert_eq!(result.detected_format, "png");
 }

 #[test]
 fn test_formats_match() {
 let corrector = FormatCorrector::new(false);

 assert!(corrector.formats_match("jpg", "jpeg"));
 assert!(corrector.formats_match("jpeg", "jpg"));
 assert!(corrector.formats_match("png", "png"));
 assert!(!corrector.formats_match("png", "jpg"));
 }
}
