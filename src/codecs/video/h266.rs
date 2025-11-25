// H.266/VVC video encoder support
// 🔥 Phase 4: Add H.266/VVC (latest generation video encoding standard)

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// H.266/VVC encoder configuration
#[derive(Debug, Clone)]
pub structure H266Params {
 /// CRF quality (0-51, lower is higher quality)
 pub crf: u8,
 /// encodingpreset (ultrafast, fast, medium, slow, veryslow)
 pub preset: String,
 /// Chroma subsampling (420, 422, 444)
 pub chroma_subsampling: String,
 /// Bit depth (8, 10)
 pub bit_depth: u8,
 /// GOPsize
 pub gop_size: Option<u32>,
 /// Bframecount
 pub bframes: Option<u8>,
}

impl Default for H266Params {
 fn default() -> Self {
 Self {
 crf: 23,
 preset: "medium".to_string(),
 chroma_subsampling: "420".to_string(),
 bit_depth: 8,
 gop_size: Some(250),
 bframes: Some(3),
 }
 }
}

impl H266Params {
 /// Create parameters from quality value
 pub fn from_quality(quality: u8) -> Self {
 let crf = Self::quality_to_crf(quality);
 Self {
 crf,
 ..Default::default()
 }
 }
 
 /// qualityvalue(0-100)conversionfor CRF(0-51)
 fn quality_to_crf(quality: u8) -> u8 {
 // quality 100 = crf 0 (highest quality)
 // quality 0 = crf 51 (lowest quality)
 ((100 - quality) as f32 * 0.51).round() as u8
 }
}

/// H.266/VVC encoder
pub structure H266Encoder;

impl H266Encoder {
 /// Check if VVC encoder is available
 pub fn is_available() -> bool {
 // Check vvencapp (Fraunhofer VVen C)
 Command::new("vvencapp")
 .arg("--help")
 .output()
 .is_ok()
 ||
 // Or check if FFmpeg supports libvvenc
 Command::new("ffmpeg")
 .args(["-codecs"])
 .output()
 .ok()
 .map(|output| {
 let codecs = String::from_utf8_lossy(&output.stdout);
 codecs.contains("libvvenc") || codecs.contains("vvc")
 })
 .unwrap_or(false)
 }
 
 /// Convert video using VVen C encoder
 pub fn encode_with_vvenc(
 input: &Path,
 output: &Path,
 params: &H266Params,
 ) -> Result<()> {
 let mut cmd = Command::new("vvencapp");
 
 cmd.arg("-i").arg(input)
 .arg("-o").arg(output)
 .arg("--qp").arg(params.crf.to_string())
 .arg("--preset").arg(&params.preset);
 
 if let Some(gop) = params.gop_size {
 cmd.arg("--gop").arg(gop.to_string());
 }
 
 if params.bit_depth == 10 {
 cmd.arg("--bitdepth").arg("10");
 }
 
 let output = cmd.output()
 .context("Failed to execute vvencapp")?;
 
 if !output.status.success() {
 let stderr = String::from_utf8_lossy(&output.stderr);
 anyhow::bail!("VVenC encoding failed: {}", stderr);
 }
 
 Ok(())
 }
 
 /// Encode using FFmpeg libvvenc
 pub fn encode_with_ffmpeg(
 input: &Path,
 output: &Path,
 params: &H266Params,
 ) -> Result<()> {
 let mut cmd = Command::new("ffmpeg");
 
 cmd.arg("-i").arg(input)
 .arg("-c:v").arg("libvvenc")
 .arg("-qp").arg(params.crf.to_string())
 .arg("-preset").arg(&params.preset);
 
 if let Some(gop) = params.gop_size {
 cmd.arg("-g").arg(gop.to_string());
 }
 
 if let Some(bf) = params.bframes {
 cmd.arg("-bf").arg(bf.to_string());
 }
 
 // Color format
 cmd.arg("-pix_fmt").arg(format!("yuv{}", params.chroma_subsampling));
 
 if params.bit_depth == 10 {
 cmd.arg("-pix_fmt").arg(format!("yuv{}p10le", params.chroma_subsampling));
 }
 
 cmd.arg("-y").arg(output);
 
 let output = cmd.output()
 .context("Failed to execute ffmpeg with libvvenc")?;
 
 if !output.status.success() {
 let stderr = String::from_utf8_lossy(&output.stderr);
 anyhow::bail!("FFmpeg VVC encoding failed: {}", stderr);
 }
 
 Ok(())
 }
 
 /// Auto-select best encoder and encode
 pub fn encode(
 input: &Path,
 output: &Path,
 params: &H266Params,
 ) -> Result<()> {
 // Prioritize native vvencapp
 if Command::new("vvencapp").arg("--help").output().is_ok() {
 Self::encode_with_vvenc(input, output, params)
 }
 // fallbacktoffmpeglibvvenc
 else if Self::is_available() {
 Self::encode_with_ffmpeg(input, output, params)
 }
 else {
 anyhow::bail!(
 "H.266/VVC encoder not available. Please install:\n\
 - vvencapp: https://github.com/fraunhoferhhi/vvenc\n\
 - or FFmpeg with libvvenc support"
 )
 }
 }
}

#[cfg(test)]
mod tests {
 use super::*;
 
 #[test]
 fn test_quality_to_crf() {
 let params = H266Params::from_quality(100);
 assert_eq!(params.crf, 0);
 
 let params = H266Params::from_quality(50);
 assert!(params.crf >= 24 && params.crf <= 26);
 
 let params = H266Params::from_quality(0);
 assert_eq!(params.crf, 51);
 }
 
 #[test]
 fn test_default_params() {
 let params = H266Params::default();
 assert_eq!(params.crf, 23);
 assert_eq!(params.preset, "medium");
 assert_eq!(params.bit_depth, 8);
 }
}
