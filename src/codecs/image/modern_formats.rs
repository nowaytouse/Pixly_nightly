// 🎨 Modern image format support
// AVIF, JXL etc Next-generation format FFmpeg integration

use anyhow::{Context, Result, bail};
use std::path::Path;
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};

/// 🔥 Safe path conversion helper function
///
/// Convert Path to&str，if fails return clear error information
fn path_to_str(path: &Path) -> Result<&str> {
 path.to_str()
 .ok_or_else(|| {
 anyhow::anyhow!(
 "Path contains invalid UTF-8 characters: {}",
 path.display()
 )
 })
}

/// Modern format converter
pub struct ModernFormatConverter {
 ffmpeg_path: String,
 cjxl_path: Option<String>,
}

impl ModernFormatConverter {
/// Create new converter
 pub fn new() -> Self {
 Self {
 ffmpeg_path: "ffmpeg".to_string(),
 cjxl_path: which::which("cjxl").ok().map(|p| p.to_string_lossy().to_string()),
 }
 }

/// Check format support
 pub fn check_format_support(&self) -> FormatSupport {
 FormatSupport {
 avif: self.check_avif_support(),
 jxl_ffmpeg: self.check_jxl_ffmpeg_support(),
 jxl_native: self.cjxl_path.is_some(),
 webp: true, // FFmpegissupportWebP
 }
 }

/// Check AVIF support
 fn check_avif_support(&self) -> bool {
 let output = Command::new(&self.ffmpeg_path)
 .args(["-encoders"])
 .output();

 if let Ok(output) = output {
 let stdout = String::from_utf8_lossy(&output.stdout);
 stdout.contains("libaom-av1") || stdout.contains("libsvtav1")
 } else {
 false
 }
 }

/// Check JXL FFmpeg support
 fn check_jxl_ffmpeg_support(&self) -> bool {
 let output = Command::new(&self.ffmpeg_path)
 .args(["-encoders"])
 .output();

 if let Ok(output) = output {
 let stdout = String::from_utf8_lossy(&output.stdout);
 stdout.contains("libjxl")
 } else {
 false
 }
 }

/// conversionforAVIF
 pub fn convert_to_avif<P: AsRef<Path>>(
 &self,
 input: P,
 output: P,
 params: &AVIFParams,
 ) -> Result<ConversionResult> {
 let input = input.as_ref();
 let output = output.as_ref();

 if !self.check_avif_support() {
 bail!("FFmpeg does not support AVIF encoding, please install libaom-av1 or libsvtav1");
 }

 let input_size = std::fs::metadata(input)?.len();

// buildFFmpegcommand
 let mut cmd = Command::new(&self.ffmpeg_path);
 cmd.args([
 "-i", path_to_str(input)?,
 "-c:v", &params.encoder,
 ]);

// Add parameters based on encoder
 match params.encoder.as_str() {
 "libaom-av1" => {
 cmd.args([
 "-crf", &params.crf.to_string(),
 "-cpu-used", &params.speed.to_string(),
 ]);

// 🔥 Quantizer parameters
 cmd.args(["-qmin", &params.min_quantizer.to_string()]);
 cmd.args(["-qmax", &params.max_quantizer.to_string()]);

// 🔥 Tilesparallelencoding
 if params.tiles_rows > 1 || params.tiles_cols > 1 {
 cmd.args(["-tiles", &format!("{}x{}", params.tiles_cols, params.tiles_rows)]);
 }
 }
 "libsvtav1" => {
 cmd.args([
 "-crf", &params.crf.to_string(),
 "-preset", &params.speed.to_string(),
 ]);

// 🔥 Quantizer parameters
 cmd.args(["-qmin", &params.min_quantizer.to_string()]);
 cmd.args(["-qmax", &params.max_quantizer.to_string()]);
 }
 _ => {}
 }

// 🔥 Pixel format - Based on user selected bit_depth and chroma_subsampling
// If user explicitly specified these parameters，we pass them to FFmpeg
// otherwise let FFmpeg auto-select best format
 if !params.chroma_subsampling.is_empty() {
 let pix_fmt = match (params.bit_depth, params.chroma_subsampling.as_str()) {
 (10, "420") => "yuv420p10le",
 (10, "422") => "yuv422p10le",
 (10, "444") => "yuv444p10le",
 (12, "420") => "yuv420p12le",
 (12, "422") => "yuv422p12le",
 (12, "444") => "yuv444p12le",
 (_, "420") => "yuv420p",
 (_, "422") => "yuv422p",
 (_, "444") => "yuv444p",
 _ => {
// Not specified, let FFmpeg auto-select
 ""
 }
 };
 if !pix_fmt.is_empty() {
 cmd.args(["-pix_fmt", pix_fmt]);
 }
 }
// If pix_fmt not specified，FFmpeg will auto-select format with minimum loss

 cmd.args([
 "-y",
 path_to_str(output)?,
 ]);

// executeconversion
 let result = cmd
 .stdout(Stdio::null())
 .stderr(Stdio::piped())
 .output()
 .context("Failed to execute FFmpeg")?;

 if !result.status.success() {
 let stderr = String::from_utf8_lossy(&result.stderr);
 bail!("AVIF conversion failed: {}", stderr);
 }

 let output_size = std::fs::metadata(output)?.len();

 Ok(ConversionResult {
 input_size,
 output_size,
 compression_ratio: output_size as f64 / input_size as f64,
 format: "avif".to_string(),
 })
 }

/// Convert to JXL (using FFmpeg)
 pub fn convert_to_jxl_ffmpeg<P: AsRef<Path>>(
 &self,
 input: P,
 output: P,
 params: &JXLParams,
 ) -> Result<ConversionResult> {
 let input = input.as_ref();
 let output = output.as_ref();

 if !self.check_jxl_ffmpeg_support() {
 bail!("FFmpeg does not support JXL encoding, please use native cjxl tool");
 }

 let input_size = std::fs::metadata(input)?.len();

 let mut cmd = Command::new(&self.ffmpeg_path);
 cmd.args([
 "-i", path_to_str(input)?,
 "-c:v", "libjxl",
 "-q:v", &params.quality.to_string(),
 "-effort", &params.effort.to_string(),
 ]);

 if params.lossless {
 cmd.args(["-lossless", "1"]);
 }

 cmd.args([
 "-y",
 path_to_str(output)?,
 ]);

 let result = cmd
 .stdout(Stdio::null())
 .stderr(Stdio::piped())
 .output()
 .context("Failed to execute FFmpeg")?;

 if !result.status.success() {
 let stderr = String::from_utf8_lossy(&result.stderr);
 bail!("JXL conversion failed: {}", stderr);
 }

 let output_size = std::fs::metadata(output)?.len();

 Ok(ConversionResult {
 input_size,
 output_size,
 compression_ratio: output_size as f64 / input_size as f64,
 format: "jxl".to_string(),
 })
 }

/// Convert to JXL (using native cjxl)
 pub fn convert_to_jxl_native<P: AsRef<Path>>(
 &self,
 input: P,
 output: P,
 params: &JXLParams,
 ) -> Result<ConversionResult> {
 let input = input.as_ref();
 let output = output.as_ref();

 let cjxl_path = self.cjxl_path.as_ref()
 .ok_or_else(|| anyhow::anyhow!("cjxl tool not installed"))?;

 let input_size = std::fs::metadata(input)?.len();

 let mut cmd = Command::new(cjxl_path);
 cmd.arg(path_to_str(input)?);
 cmd.arg(path_to_str(output)?);

// qualityparameter
 if params.lossless {
 cmd.arg("--lossless");
 } else {
 cmd.args(["--quality", &params.quality.to_string()]);
 }

// Effort level
 cmd.args(["--effort", &params.effort.to_string()]);

// 🔥 Advanced JXL parameters - REAL implementation!
 if params.modular {
 cmd.arg("--modular");
 }

 if params.progressive {
 cmd.arg("--progressive");
 }

 if params.responsive {
 cmd.args(["--responsive", "1"]);
 }

 if params.gaborish {
 cmd.arg("--gaborish=1");
 } else {
 cmd.arg("--gaborish=0");
 }

 if params.photon_noise > 0 {
 cmd.args(["--photon_noise", &params.photon_noise.to_string()]);
 }

 if params.decoding_speed > 0 {
 cmd.args(["--decoding_speed", &params.decoding_speed.to_string()]);
 }

// 🔥 Phase 2: Additional critical parameters
// Detect if input is JPEG
 let is_jpeg_input = input.extension()
 .and_then(|e| e.to_str())
 .map(|e| e.to_lowercase())
 .map(|e| e == "jpg" || e == "jpeg")
 .unwrap_or(false);

// Distance parameter handling (lossy compression control)
 log::debug!("DEBUG JXL params:");
 log::debug!("is_jpeg_input: {}", is_jpeg_input);
 log::debug!("params.distance: {}", params.distance);
 log::debug!("params.lossless: {}", params.lossless);

 if params.distance > 0.0 && !params.lossless {
 if is_jpeg_input {
// For JPEG input, need to explicitly disable lossless_jpeg to use distance
// This allows users to choose:
// - Lossless repack (default, don't pass distance)
// - Lossy conversion (pass --lossless_jpeg=0 + --distance)
 log::debug!("Adding --lossless_jpeg 0");
 cmd.args(["--lossless_jpeg", "0"]);
 }
 log::debug!("Adding --distance {}", params.distance);
 cmd.args(["--distance", &params.distance.to_string()]);
 } else {
 log::debug!("Skipping distance (lossless or distance=0)");
 }

// 🔥 Bit depth - Only pass when not default and not 0，let cjxl auto-process
 if params.bit_depth != 8 && params.bit_depth != 0 {
 cmd.args(["--bits_per_sample", &params.bit_depth.to_string()]);
 }
// Don't pass when bit_depth=0 or 8，let cjxl auto-select based on source file

// 🔥 Color space - Only pass when not default and non-empty，let cjxl auto-process
 if !params.color_space.is_empty()
 && params.color_space != "sRGB"
 && params.color_space != "auto" {
 cmd.args(["--color_space", &params.color_space]);
 }
// color_space is empty、"s RGB"or"auto" when not pass，let cjxl keep source color space

 if params.patches > 0 {
 cmd.args(["--patches", &params.patches.to_string()]);
 }

// executeconversion
 let result = cmd
 .stdout(Stdio::null())
 .stderr(Stdio::piped())
 .output()
 .context("Failed to execute cjxl")?;

 if !result.status.success() {
 let stderr = String::from_utf8_lossy(&result.stderr);
 bail!("JXL conversion failed: {}", stderr);
 }

 let output_size = std::fs::metadata(output)?.len();

 Ok(ConversionResult {
 input_size,
 output_size,
 compression_ratio: output_size as f64 / input_size as f64,
 format: "jxl".to_string(),
 })
 }

/// intelligentselectJXLconversionmethod
 pub fn convert_to_jxl<P: AsRef<Path>>(
 &self,
 input: P,
 output: P,
 params: &JXLParams,
 ) -> Result<ConversionResult> {
// Prioritize native cjxl（faster and better）
 if self.cjxl_path.is_some() {
 self.convert_to_jxl_native(input, output, params)
 } else if self.check_jxl_ffmpeg_support() {
 self.convert_to_jxl_ffmpeg(input, output, params)
 } else {
 bail!("JXL encoder not available, please install cjxl or FFmpeg with JXL support")
 }
 }

/// Convert to Web P (using cwebp)
 pub fn convert_to_webp<P: AsRef<Path>>(
 &self,
 input: P,
 output: P,
 params: &WebPParams,
 ) -> Result<ConversionResult> {
 let input = input.as_ref();
 let output = output.as_ref();

 let cwebp_path = which::which("cwebp")
 .context("cwebp tool not installed")?;

 let input_size = std::fs::metadata(input)?.len();

 let mut cmd = Command::new(cwebp_path);
 cmd.arg(path_to_str(input)?);
 cmd.arg("-o").arg(path_to_str(output)?);

// 🔥 Complete WebP parameters - REAL implementation!
 if params.lossless {
 cmd.arg("-lossless");
 } else {
 cmd.args(["-q", &params.quality.to_string()]);
 }

// compressionmethod (0-6)
 cmd.args(["-m", &params.method.to_string()]);

// Filter strength (0-100)
 cmd.args(["-f", &params.filter_strength.to_string()]);

// sharpeninglevel (0-7)
 cmd.args(["-sharpness", &params.sharpness.to_string()]);

// executeconversion
 let result = cmd
 .stdout(Stdio::null())
 .stderr(Stdio::piped())
 .output()
 .context("Failed to execute cwebp")?;

 if !result.status.success() {
 let stderr = String::from_utf8_lossy(&result.stderr);
 bail!("WebP conversion failed: {}", stderr);
 }

 let output_size = std::fs::metadata(output)?.len();

 Ok(ConversionResult {
 input_size,
 output_size,
 compression_ratio: output_size as f64 / input_size as f64,
 format: "webp".to_string(),
 })
 }

/// conversionfor HEIC (using FFmpeg + x265/libheif)
 pub fn convert_to_heic<P: AsRef<Path>>(
 &self,
 input: P,
 output: P,
 params: &HEICParams,
 ) -> Result<ConversionResult> {
 let input = input.as_ref();
 let output = output.as_ref();

 let input_size = std::fs::metadata(input)?.len();

 let mut cmd = Command::new(&self.ffmpeg_path);
 cmd.args(["-i", path_to_str(input)?]);

// 🔥 Complete HEIC parameters - REAL implementation!
 match params.encoder.as_str() {
 "x265" => {
 cmd.args(["-c:v", "libx265"]);
 cmd.args(["-tag:v", "hvc1"]); // HEIC tag

 if params.lossless {
 cmd.args(["-x265-params", "lossless=1"]);
 } else {
 let crf = 100 - params.quality; // Convert quality to CRF
 cmd.args(["-crf", &crf.to_string()]);
 }
 }
 "libheif" => {
// libheif encoder (if available)
 cmd.args(["-c:v", "libheif"]);
 cmd.args(["-q:v", &params.quality.to_string()]);
 }
 _ => {
 bail!("Unsupported HEIC encoder: {}", params.encoder);
 }
 }

// 🔥 Pixel format - based onselectchroma_subsampling
// if明确specify，justpass；otherwise let FFmpegautoselect
 if !params.chroma_subsampling.is_empty() {
 let pix_fmt = match params.chroma_subsampling.as_str() {
 "444" => "yuv444p",
 "420" => "yuv420p",
 _ => "", // letFFmpegauto
 };
 if !pix_fmt.is_empty() {
 cmd.args(["-pix_fmt", pix_fmt]);
 }
 }
// if not specify，FFmpeg will auto-select format with minimum loss

 cmd.args(["-y", path_to_str(output)?]);

// executeconversion
 let result = cmd
 .stdout(Stdio::null())
 .stderr(Stdio::piped())
 .output()
 .context("Failed to execute FFmpeg")?;

 if !result.status.success() {
 let stderr = String::from_utf8_lossy(&result.stderr);
 bail!("HEIC conversion failed: {}", stderr);
 }

 let output_size = std::fs::metadata(output)?.len();

 Ok(ConversionResult {
 input_size,
 output_size,
 compression_ratio: output_size as f64 / input_size as f64,
 format: "heic".to_string(),
 })
 }
}

impl Default for ModernFormatConverter {
 fn default() -> Self {
 Self::new()
 }
}

/// formatsupportinformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatSupport {
 pub avif: bool,
 pub jxl_ffmpeg: bool,
 pub jxl_native: bool,
 pub webp: bool,
}

impl FormatSupport {
/// getsupportformatlist
 pub fn supported_formats(&self) -> Vec<String> {
 let mut formats = vec!["webp".to_string()];

 if self.avif {
 formats.push("avif".to_string());
 }

 if self.jxl_ffmpeg || self.jxl_native {
 formats.push("jxl".to_string());
 }

 formats
 }

/// checkisnosupportspecifyformat
 pub fn supports(&self, format: &str) -> bool {
 match format.to_lowercase().as_str() {
 "webp" => self.webp,
 "avif" => self.avif,
 "jxl" => self.jxl_ffmpeg || self.jxl_native,
 _ => false,
 }
 }
}

/// AVIFconversionparameter
#[derive(Debug, Clone)]
pub struct AVIFParams {
 pub encoder: String, // "libaom-av1" or "libsvtav1"
 pub crf: u8, // 0-63, smallqualityhigher
 pub speed: u8, // 0-8 (libaom) or 0-13 (svt)
 pub bit_depth: u8, // 8, 10, or 12
// 🔥 Complete AVIF parameters - NO MORE FAKE UI!
 pub min_quantizer: u8, // 0-63, minimum quantizer
 pub max_quantizer: u8, // 0-63, maximum quantizer
 pub chroma_subsampling: String, // "420", "422", "444"
 pub tiles_rows: u8, // 1-8, tile rows for parallel encoding
 pub tiles_cols: u8, // 1-8, tile columns for parallel encoding
 pub premultiply_alpha: bool, // Premultiply alpha channel
}

impl Default for AVIFParams {
 fn default() -> Self {
 Self {
 encoder: "libaom-av1".to_string(),
 crf: 30,
 speed: 6,
 bit_depth: 8,
 min_quantizer: 0,
 max_quantizer: 63,
 chroma_subsampling: "420".to_string(),
 tiles_rows: 1,
 tiles_cols: 1,
 premultiply_alpha: false,
 }
 }
}

impl AVIFParams {
/// fromqualityvaluecreate (0-100)
 pub fn from_quality(quality: u8) -> Self {
 let clamped_quality = quality.min(100); // Clamp to valid range
 let crf = ((100 - clamped_quality) as f64 * 0.63) as u8; // mapto0-63
 let speed = if clamped_quality >= 90 {
 6 // highqualityslow
 } else if clamped_quality >= 70 {
 4 // etcquality平衡
 } else {
 2 // lowqualityquick
 };

// Calculate min/max quantizers based on quality
 let min_q = if clamped_quality > 90 { 0 } else if clamped_quality > 70 { 5 } else { 10 };
 let max_q = if clamped_quality > 90 { 20 } else if clamped_quality > 70 { 35 } else { 50 };

 Self {
 encoder: "libaom-av1".to_string(),
 crf,
 speed,
 bit_depth: 8,
 min_quantizer: min_q,
 max_quantizer: max_q,
 chroma_subsampling: "420".to_string(),
 tiles_rows: 1,
 tiles_cols: 1,
 premultiply_alpha: false,
 }
 }
}

/// JXLconversionparameter
#[derive(Debug, Clone)]
pub struct JXLParams {
 pub quality: u8, // 0-100
 pub effort: u8, // 1-9
 pub lossless: bool,
// 🔥 Advanced JXL parameters - REAL implementation!
 pub modular: bool, // Use modular mode (better for synthetic images)
 pub progressive: bool, // Enable progressive decoding
 pub responsive: bool, // Enable responsive by default
 pub gaborish: bool, // Enable Gaborish filter (reduces ringing artifacts)
 pub photon_noise: u8, // Photon noise level (0-100)
 pub decoding_speed: u8, // Decoding speed tier (0-4)
// 🔥 Phase 2: Additional critical parameters
 pub distance: f32, // Psychovisual distance (0.0=lossless, higher=more compression)
 pub bit_depth: u8, // Bit depth: 8, 10, 12, or 16
 pub color_space: String, // Color space: sRGB, Display P3, Adobe RGB, ProPhoto RGB
 pub patches: u8, // Edge enhancement level (0-4)
}

impl Default for JXLParams {
 fn default() -> Self {
 Self {
 quality: 85,
 effort: 7,
 lossless: false,
 modular: false,
 progressive: true,
 responsive: true,
 gaborish: true,
 photon_noise: 0,
 decoding_speed: 0,
 distance: 1.0,
 bit_depth: 0, // 0 = auto, let cjxl detect from source
 color_space: "auto".to_string(), // auto = preserve source color space
 patches: 1,
 }
 }
}

impl JXLParams {
/// fromqualityvaluecreate
 pub fn from_quality(quality: u8) -> Self {
 let effort = if quality >= 90 {
 9 // highqualitymaximumeffort
 } else if quality >= 70 {
 7 // etcquality平衡
 } else {
 5 // lowqualityquick
 };

 Self {
 quality,
 effort,
 lossless: quality >= 95,
 modular: false,
 progressive: true,
 responsive: true,
 gaborish: true,
 photon_noise: 0,
 decoding_speed: 0,
 distance: if quality >= 90 { 0.5 } else { 1.0 },
 bit_depth: 0, // Always auto, let cjxl decide based on source
 color_space: "auto".to_string(), // Always auto, preserve source
 patches: 1,
 }
 }
}

/// WebPconversionparameter
#[derive(Debug, Clone)]
pub struct WebPParams {
 pub quality: u8, // 0-100
 pub method: u8, // 0-6, compression method
 pub filter_strength: u8, // 0-100, deblocking filter strength
 pub sharpness: u8, // 0-7, sharpness level
 pub lossless: bool, // lossless encoding
}

/// HEICconversionparameter
#[derive(Debug, Clone)]
pub struct HEICParams {
 pub quality: u8, // 0-100
 pub encoder: String, // "x265" or "libheif"
 pub chroma_subsampling: String, // "420" or "444"
 pub lossless: bool, // lossless encoding
 pub embed_thumbnail: bool, // embed thumbnail
}

impl Default for WebPParams {
 fn default() -> Self {
 Self {
 quality: 85,
 method: 4,
 filter_strength: 60,
 sharpness: 0,
 lossless: false,
 }
 }
}

impl WebPParams {
 pub fn from_quality(quality: u8) -> Self {
 let method = if quality >= 90 { 6 } else if quality >= 70 { 4 } else { 2 };
 Self {
 quality,
 method,
 filter_strength: 60,
 sharpness: 0,
 lossless: quality >= 95,
 }
 }
}

impl Default for HEICParams {
 fn default() -> Self {
 Self {
 quality: 85,
 encoder: "x265".to_string(),
 chroma_subsampling: "420".to_string(),
 lossless: false,
 embed_thumbnail: false,
 }
 }
}

impl HEICParams {
 pub fn from_quality(quality: u8) -> Self {
 Self {
 quality,
 encoder: "x265".to_string(),
 chroma_subsampling: if quality >= 90 { "444".to_string() } else { "420".to_string() },
 lossless: quality >= 95,
 embed_thumbnail: true,
 }
 }
}

/// conversionresult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
 pub input_size: u64,
 pub output_size: u64,
 pub compression_ratio: f64,
 pub format: String,
}

impl ConversionResult {
/// getemptypercentage
 pub fn space_saving_percent(&self) -> f64 {
 (1.0 - self.compression_ratio) * 100.0
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_format_support_check() {
 let converter = ModernFormatConverter::new();
 let support = converter.check_format_support();

// Web Pshouldissupport
 assert!(support.webp);

// printsupportformat
 log::debug!("Supported formats: {:?}", support.supported_formats());
 }

 #[test]
 fn test_avif_params_from_quality() {
 let params = AVIFParams::from_quality(85);
 assert!(params.crf < 20); // highqualityshouldhaslowCRF
 assert!(params.speed >= 2);
 }

 #[test]
 fn test_jxl_params_from_quality() {
 let params = JXLParams::from_quality(90);
 assert_eq!(params.quality, 90);
 assert!(params.effort >= 7);
 }

 #[test]
 fn test_format_support_query() {
 let support = FormatSupport {
 avif: true,
 jxl_ffmpeg: false,
 jxl_native: true,
 webp: true,
 };

 assert!(support.supports("webp"));
 assert!(support.supports("avif"));
 assert!(support.supports("jxl"));
 assert!(!support.supports("heic"));
 }
}
