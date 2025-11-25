/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 格式专属参数 (Format-Specific Parameters)
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 
 * 🎯 target:
 * - fullsupportHTML界面所has格式专属参数
 * - 与Eagle插件HTML界面completely对应
 * - 类型安全参数验证
 */
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════
// 📦 JXL (JPEG XL) parameter
// ═══════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure JxlParams {
 /// Effort level (1-9) - 对应HTML: jxl Effort
 pub effort: Option<u8>,
 
 /// distancevalue (0-15) - 对应HTML: jxl Distance
 pub distance: Option<f32>,
 
 /// Bit depth (8/10/12/16) - 对应HTML: jxl Bit Depth
 pub bit_depth: Option<u8>,
 
 /// Color space - 对应HTML: jxl Color Space
 pub color_space: Option<String>,
 
 /// 边缘enhanced (0-4) - 对应HTML: jxl Patches
 pub patches: Option<u8>,
}

impl Default for JxlParams {
 fn default() -> Self {
 Self {
 effort: Some(7),
 distance: Some(1.0),
 bit_depth: Some(8),
 color_space: Some("sRGB".to_string()),
 patches: Some(1),
 }
 }
}

impl JxlParams {
 pub fn validate(&self) -> Result<(), String> {
 if let Some(effort) = self.effort
 && !(1..=9).contains(&effort) {
 return Err(format!("JXL effort must be 1-9, got {}", effort));
 }
 
 if let Some(distance) = self.distance
 && !(0.0..=15.0).contains(&distance) {
 return Err(format!("JXL distance must be 0-15, got {}", distance));
 }
 
 if let Some(bit_depth) = self.bit_depth
 && ![8, 10, 12, 16].contains(&bit_depth) {
 return Err(format!("JXL bit_depth must be 8/10/12/16, got {}", bit_depth));
 }
 
 if let Some(patches) = self.patches
 && patches > 4 {
 return Err(format!("JXL patches must be 0-4, got {}", patches));
 }
 
 Ok(())
 }
}

// ═══════════════════════════════════════════════════
// 🌐 WebP parameter
// ═══════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure WebPParams {
 /// compressionmethod (0-6) - 对应HTML: webp Method
 pub method: Option<u8>,
 
 /// Filter strength (0-100) - 对应HTML: webp Filter Strength
 pub filter_strength: Option<u8>,
 
 /// sharpeninglevel (0-7) - 对应HTML: webp Sharpness
 pub sharpness: Option<u8>,
 
 /// 分段count (1-4) - 对应HTML: webp Segments
 pub segments: Option<u8>,
 
 /// SNSstrength (0-100) - 对应HTML: webp Sns Strength
 pub sns_strength: Option<u8>,
 
 /// autofilter - 对应HTML: webp Auto Filter
 pub auto_filter: bool,
 
 /// exactmode - 对应HTML: webp Exact Mode
 pub exact_mode: bool,
 
 /// removed Alpha - 对应HTML: webp No Alpha
 pub no_alpha: bool,
 
 /// lowmemorymode - 对应HTML: webp Low Memory
 pub low_memory: bool,
 
 /// encoding遍数 (1-10) - 对应HTML: webp Pass
 pub pass: Option<u8>,
}

impl Default for WebPParams {
 fn default() -> Self {
 Self {
 method: Some(4),
 filter_strength: Some(60),
 sharpness: Some(0),
 segments: Some(4),
 sns_strength: Some(50),
 auto_filter: true,
 exact_mode: false,
 no_alpha: false,
 low_memory: false,
 pass: Some(1),
 }
 }
}

impl WebPParams {
 pub fn validate(&self) -> Result<(), String> {
 if let Some(method) = self.method
 && method > 6 {
 return Err(format!("WebP method must be 0-6, got {}", method));
 }
 
 if let Some(filter_strength) = self.filter_strength
 && filter_strength > 100 {
 return Err(format!("WebP filter_strength must be 0-100, got {}", filter_strength));
 }
 
 if let Some(sharpness) = self.sharpness
 && sharpness > 7 {
 return Err(format!("WebP sharpness must be 0-7, got {}", sharpness));
 }
 
 if let Some(segments) = self.segments
 && !(1..=4).contains(&segments) {
 return Err(format!("WebP segments must be 1-4, got {}", segments));
 }
 
 if let Some(sns_strength) = self.sns_strength
 && sns_strength > 100 {
 return Err(format!("WebP sns_strength must be 0-100, got {}", sns_strength));
 }
 
 if let Some(pass) = self.pass
 && !(1..=10).contains(&pass) {
 return Err(format!("WebP pass must be 1-10, got {}", pass));
 }
 
 Ok(())
 }
}

// ═══════════════════════════════════════════════════
// 🎬 AVIF parameter
// ═══════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure AvifParams {
 /// encodingspeed (0-10) - 对应HTML: avif Speed
 pub speed: Option<u8>,
 
 /// minimumquantization (0-63) - 对应HTML: avif Min Quantizer
 pub min_quantizer: Option<u8>,
 
 /// maximumquantization (0-63) - 对应HTML: avif Max Quantizer
 pub max_quantizer: Option<u8>,
 
 /// 色度子sampling - 对应HTML: avif Chroma Subsampling
 pub chroma_subsampling: Option<String>,
 
 /// Bit depth (8/10/12) - 对应HTML: avif Bit Depth
 pub bit_depth: Option<u8>,
 
 /// Tilesline数 (1-8) - 对应HTML: avif Tiles Rows
 pub tiles_rows: Option<u8>,
 
 /// Tiles列数 (1-8) - 对应HTML: avif Tiles Cols
 pub tiles_cols: Option<u8>,
 
 /// 预乘Alpha - 对应HTML: avif Premultiply
 pub premultiply_alpha: bool,
}

impl Default for AvifParams {
 fn default() -> Self {
 Self {
 speed: Some(6),
 min_quantizer: Some(0),
 max_quantizer: Some(63),
 chroma_subsampling: Some("420".to_string()),
 bit_depth: Some(8),
 tiles_rows: Some(1),
 tiles_cols: Some(1),
 premultiply_alpha: false,
 }
 }
}

impl AvifParams {
 pub fn validate(&self) -> Result<(), String> {
 if let Some(speed) = self.speed
 && speed > 10 {
 return Err(format!("AVIF speed must be 0-10, got {}", speed));
 }
 
 if let Some(min_q) = self.min_quantizer
 && min_q > 63 {
 return Err(format!("AVIF min_quantizer must be 0-63, got {}", min_q));
 }
 
 if let Some(max_q) = self.max_quantizer
 && max_q > 63 {
 return Err(format!("AVIF max_quantizer must be 0-63, got {}", max_q));
 }
 
 if let Some(bit_depth) = self.bit_depth
 && ![8, 10, 12].contains(&bit_depth) {
 return Err(format!("AVIF bit_depth must be 8/10/12, got {}", bit_depth));
 }
 
 if let Some(rows) = self.tiles_rows
 && !(1..=8).contains(&rows) {
 return Err(format!("AVIF tiles_rows must be 1-8, got {}", rows));
 }
 
 if let Some(cols) = self.tiles_cols
 && !(1..=8).contains(&cols) {
 return Err(format!("AVIF tiles_cols must be 1-8, got {}", cols));
 }
 
 Ok(())
 }
}

// ═══════════════════════════════════════════════════
// 🍎 HEIC parameter
// ═══════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure HeicParams {
 /// compressionquality (1-100) - 对应HTML: heic Quality
 pub quality: Option<u8>,
 
 /// Encoder - 对应HTML: heic Encoder
 pub encoder: Option<String>,
 
 /// 色度子sampling - 对应HTML: heic Chroma Subsampling
 pub chroma_subsampling: Option<String>,
 
 /// losslessencoding - 对应HTML: heic Lossless
 pub lossless: bool,
 
 /// 嵌入缩略图 - 对应HTML: heic Thumb Embed
 pub embed_thumbnail: bool,
}

impl Default for HeicParams {
 fn default() -> Self {
 Self {
 quality: Some(85),
 encoder: Some("x265".to_string()),
 chroma_subsampling: Some("444".to_string()),
 lossless: false,
 embed_thumbnail: true,
 }
 }
}

impl HeicParams {
 pub fn validate(&self) -> Result<(), String> {
 if let Some(quality) = self.quality
 && !(1..=100).contains(&quality) {
 return Err(format!("HEIC quality must be 1-100, got {}", quality));
 }
 
 if let Some(ref encoder) = self.encoder
 && !["x265", "libheif"].contains(&encoder.as_str()) {
 return Err(format!("HEIC encoder must be x265 or libheif, got {}", encoder));
 }
 
 if let Some(ref chroma) = self.chroma_subsampling
 && !["420", "422", "444"].contains(&chroma.as_str()) {
 return Err(format!("HEIC chroma_subsampling must be 420/422/444, got {}", chroma));
 }
 
 Ok(())
 }
}

// ═══════════════════════════════════════════════════
// 🎯 Unifiedformatparameterenum
// ═══════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format", content = "params")]
pub enum FormatSpecificParams {
 Jxl(JxlParams),
 WebP(WebPParams),
 Avif(AvifParams),
 Heic(HeicParams),
 None,
}

impl FormatSpecificParams {
 pub fn validate(&self) -> Result<(), String> {
 match self {
 FormatSpecificParams::Jxl(params) => params.validate(),
 FormatSpecificParams::WebP(params) => params.validate(),
 FormatSpecificParams::Avif(params) => params.validate(),
 FormatSpecificParams::Heic(params) => params.validate(),
 FormatSpecificParams::None => Ok(()),
 }
 }
}

#[cfg(test)]
mod tests {
 use super::*;
 
 #[test]
 fn test_jxl_params_default() {
 let params = JxlParams::default();
 assert_eq!(params.effort, Some(7));
 assert_eq!(params.distance, Some(1.0));
 assert!(params.validate().is_ok());
 }
 
 #[test]
 fn test_jxl_params_validation() {
 let params = JxlParams {
 effort: Some(10),
 ..Default::default()
 };
 assert!(params.validate().is_err());
 
 let params = JxlParams {
 effort: Some(7),
 distance: Some(20.0),
 ..Default::default()
 };
 assert!(params.validate().is_err());
 }
 
 #[test]
 fn test_webp_params_default() {
 let params = WebPParams::default();
 assert_eq!(params.method, Some(4));
 assert!(params.auto_filter);
 assert!(params.validate().is_ok());
 }
 
 #[test]
 fn test_avif_params_default() {
 let params = AvifParams::default();
 assert_eq!(params.speed, Some(6));
 assert_eq!(params.chroma_subsampling, Some("420".to_string()));
 assert!(params.validate().is_ok());
 }
 
 #[test]
 fn test_heic_params_default() {
 let params = HeicParams::default();
 assert_eq!(params.quality, Some(85));
 assert_eq!(params.encoder, Some("x265".to_string()));
 assert!(params.validate().is_ok());
 }
}
