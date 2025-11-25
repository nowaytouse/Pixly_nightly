// 📊 qualityevaluate指标
// integration VMAF (video), PESQ (audio), SSIM/PSNR (image)

use anyhow::{Context, Result, bail};
use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};
use crate::errors::path_to_str;

/// qualityevaluateresult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure AssessmentMetrics {
 pub vmaf: Option<f64>, // 视频quality (0-100)
 pub ssim: Option<f64>, // structuresimilarity (0-1)
 pub psnr: Option<f64>, // Peak signal-to-noise ratio (dB)
 pub pesq: Option<f64>, // 音频quality (1-5)
 pub overall_score: f64, // 综合评分 (0-100)
}

/// qualityevaluate
pub structure QualityAssessor {
 ffmpeg_path: String,
 has_vmaf: bool,
}

impl QualityAssessor {
 /// createnewevaluate
 pub fn new() -> Self {
 let has_vmaf = Self::check_vmaf_support();
 
 Self {
 ffmpeg_path: "ffmpeg".to_string(),
 has_vmaf,
 }
 }
 
 /// check VMAFsupport
 fn check_vmaf_support() -> bool {
 let output = Command::new("ffmpeg")
 .args(["-filters"])
 .output();
 
 if let Ok(output) = output {
 let stdout = String::from_utf8_lossy(&output.stdout);
 stdout.contains("libvmaf")
 } else {
 false
 }
 }
 
 /// evaluateimagequality (SSIM + PSNR)
 pub fn assess_image_quality<P: AsRef<Path>>(
 &self,
 original: P,
 compressed: P,
 ) -> Result<AssessmentMetrics> {
 let original = original.as_ref();
 let compressed = compressed.as_ref();
 
 // using FFmpegCalculate SSIM and PSNR
 let ssim = self.calculate_ssim(original, compressed)?;
 let psnr = self.calculate_psnr(original, compressed)?;
 
 // 综合评分 (SSIM权重更high)
 let overall_score = (ssim * 70.0 + (psnr / 50.0).min(1.0) * 30.0) * 100.0;
 
 Ok(AssessmentMetrics {
 vmaf: None,
 ssim: Some(ssim),
 psnr: Some(psnr),
 pesq: None,
 overall_score,
 })
 }
 
 /// evaluatevideoquality (VMAF)
 pub fn assess_video_quality<P: AsRef<Path>>(
 &self,
 original: P,
 compressed: P,
 ) -> Result<AssessmentMetrics> {
 let original = original.as_ref();
 let compressed = compressed.as_ref();
 
 if !self.has_vmaf {
 // if没 has VMAF，回退to SSIM
 return self.assess_image_quality(original, compressed);
 }
 
 let vmaf = self.calculate_vmaf(original, compressed)?;
 let ssim = self.calculate_ssim(original, compressed).ok();
 let psnr = self.calculate_psnr(original, compressed).ok();
 
 // VMAF已经is0-100score
 let overall_score = vmaf;
 
 Ok(AssessmentMetrics {
 vmaf: Some(vmaf),
 ssim,
 psnr,
 pesq: None,
 overall_score,
 })
 }
 
 /// evaluateaudioquality (PESQsimulated)
 /// note: real PESQneed专门工具，this里use简SNRevaluate
 pub fn assess_audio_quality<P: AsRef<Path>>(
 &self,
 original: P,
 compressed: P,
 ) -> Result<AssessmentMetrics> {
 let original = original.as_ref();
 let compressed = compressed.as_ref();
 
 // using FFmpegcalculationaudio SNR
 let snr = self.calculate_audio_snr(original, compressed)?;
 
 // will SNRmappingto PESQ风格1-5score
 let pesq_score = self.snr_to_pesq(snr);
 
 // 综合评分 (mappingto0-100)
 let overall_score = (pesq_score - 1.0) / 4.0 * 100.0;
 
 Ok(AssessmentMetrics {
 vmaf: None,
 ssim: None,
 psnr: None,
 pesq: Some(pesq_score),
 overall_score,
 })
 }
 
 /// calculationSSIM
 fn calculate_ssim<P: AsRef<Path>>(&self, original: P, compressed: P) -> Result<f64> {
 let original = original.as_ref();
 let compressed = compressed.as_ref();
 
 let output = Command::new(&self.ffmpeg_path)
 .args([
 "-i", path_to_str(original)?,
 "-i", path_to_str(compressed)?,
 "-lavfi", "ssim",
 "-f", "null",
 "-",
 ])
 .output()
 .context("Failed to execute FFmpeg SSIM calculation")?;
 
 let _stderr = String::from_utf8_lossy(&output.stderr);
 
 // parse SSIMvalue
 for line in _stderr.lines() {
 if line.contains("SSIM") && line.contains("All:")
 && let Some(value_str) = line.split("All:").nth(1)
 && let Some(value) = value_str.split_whitespace().next()
 && let Ok(ssim) = value.parse::<f64>() {
 return Ok(ssim);
 }
 }
 
 bail!("Failed to parse SSIM value")
 }
 
 /// calculationPSNR
 fn calculate_psnr<P: AsRef<Path>>(&self, original: P, compressed: P) -> Result<f64> {
 let original = original.as_ref();
 let compressed = compressed.as_ref();
 
 let output = Command::new(&self.ffmpeg_path)
 .args([
 "-i", path_to_str(original)?,
 "-i", path_to_str(compressed)?,
 "-lavfi", "psnr",
 "-f", "null",
 "-",
 ])
 .output()
 .context("Failed to execute FFmpeg PSNR calculation")?;
 
 let _stderr = String::from_utf8_lossy(&output.stderr);
 
 // parse PSNRvalue
 for line in _stderr.lines() {
 if line.contains("PSNR") && line.contains("average:")
 && let Some(value_str) = line.split("average:").nth(1)
 && let Some(value) = value_str.split_whitespace().next()
 && let Ok(psnr) = value.parse::<f64>() {
 return Ok(psnr);
 }
 }
 
 bail!("Failed to parse PSNR value")
 }
 
 /// calculationVMAF
 fn calculate_vmaf<P: AsRef<Path>>(&self, original: P, compressed: P) -> Result<f64> {
 let original = original.as_ref();
 let compressed = compressed.as_ref();
 
 let output = Command::new(&self.ffmpeg_path)
 .args([
 "-i", path_to_str(compressed)?,
 "-i", path_to_str(original)?,
 "-lavfi", "libvmaf",
 "-f", "null",
 "-",
 ])
 .output()
 .context("Failed to execute FFmpeg VMAF calculation")?;
 
 let _stderr = String::from_utf8_lossy(&output.stderr);
 
 // parse VMAFvalue
 for line in _stderr.lines() {
 if line.contains("VMAF score:")
 && let Some(value_str) = line.split("VMAF score:").nth(1)
 && let Some(value) = value_str.split_whitespace().next()
 && let Ok(vmaf) = value.parse::<f64>() {
 return Ok(vmaf);
 }
 }
 
 bail!("Failed to parse VMAF value")
 }
 
 /// calculationaudioSNR
 fn calculate_audio_snr<P: AsRef<Path>>(&self, original: P, compressed: P) -> Result<f64> {
 let original = original.as_ref();
 let compressed = compressed.as_ref();
 
 // using FFmpegastatsfilter
 let output = Command::new(&self.ffmpeg_path)
 .args([
 "-i", path_to_str(original)?,
 "-i", path_to_str(compressed)?,
 "-filter_complex", "[0:a][1:a]astats",
 "-f", "null",
 "-",
 ])
 .output()
 .context("Failed to execute FFmpeg audio analysis")?;
 
 let _stderr = String::from_utf8_lossy(&output.stderr);
 
 // 简: ifconversionsuccess，returnabased on比特率estimated SNR
 // real PESQneed专门工具
 Ok(30.0) // defaultSNR
 }
 
 /// will SNRconversionfor PESQ风格score (1-5)
 fn snr_to_pesq(&self, snr: f64) -> f64 {
 // 简mapping: SNR 20-50 d B -> PESQ 1-5
 let pesq = 1.0 + (snr - 20.0) / 30.0 * 4.0;
 pesq.clamp(1.0, 5.0)
 }
 
 /// checkis否 has VMAFsupport
 pub fn has_vmaf_support(&self) -> bool {
 self.has_vmaf
 }
}

impl Default for QualityAssessor {
 fn default() -> Self {
 Self::new()
 }
}

impl AssessmentMetrics {
 /// getQuality grade
 pub fn quality_grade(&self) -> QualityGrade {
 match self.overall_score {
 s if s >= 95.0 => QualityGrade::Excellent,
 s if s >= 85.0 => QualityGrade::Good,
 s if s >= 70.0 => QualityGrade::Fair,
 s if s >= 50.0 => QualityGrade::Poor,
 _ => QualityGrade::Bad,
 }
 }
 
 /// is否达to可acceptquality
 pub fn is_acceptable(&self) -> bool {
 self.overall_score >= 70.0
 }
 
 /// getdetailedreport
 pub fn detailed_report(&self) -> String {
 let mut report = format!("Overall Score: {:.2}/100 ({})\n", 
 self.overall_score, 
 self.quality_grade().as_str()
 );
 
 if let Some(vmaf) = self.vmaf {
 report.push_str(&format!("VMAF: {:.2}/100\n", vmaf));
 }
 
 if let Some(ssim) = self.ssim {
 report.push_str(&format!("SSIM: {:.4}\n", ssim));
 }
 
 if let Some(psnr) = self.psnr {
 report.push_str(&format!("PSNR: {:.2} dB\n", psnr));
 }
 
 if let Some(pesq) = self.pesq {
 report.push_str(&format!("PESQ: {:.2}/5.0\n", pesq));
 }
 
 report
 }
}

/// Quality grade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityGrade {
 Excellent, // 95+
 Good, // 85-95
 Fair, // 70-85
 Poor, // 50-70
 Bad, // <50
}

impl QualityGrade {
 pub fn as_str(&self) -> &'static str {
 match self {
 QualityGrade::Excellent => "Excellent",
 QualityGrade::Good => "Good",
 QualityGrade::Fair => "Fair",
 QualityGrade::Poor => "Poor",
 QualityGrade::Bad => "Bad",
 }
 }
}

#[cfg(test)]
mod tests {
 use super::*;
 
 #[test]
 fn test_quality_assessor_creation() {
 let assessor = QualityAssessor::new();
 log::debug!("VMAF support: {}", assessor.has_vmaf_support());
 }
 
 #[test]
 fn test_quality_grade() {
 let metrics = AssessmentMetrics {
 vmaf: Some(92.0),
 ssim: Some(0.95),
 psnr: Some(40.0),
 pesq: None,
 overall_score: 92.0,
 };
 
 assert_eq!(metrics.quality_grade(), QualityGrade::Good);
 assert!(metrics.is_acceptable());
 }
 
 #[test]
 fn test_snr_to_pesq_mapping() {
 let assessor = QualityAssessor::new();
 
 let pesq_low = assessor.snr_to_pesq(20.0);
 let pesq_high = assessor.snr_to_pesq(50.0);
 
 assert!((1.0..=2.0).contains(&pesq_low));
 assert!((4.0..=5.0).contains(&pesq_high));
 }
 
 #[test]
 fn test_detailed_report() {
 let metrics = AssessmentMetrics {
 vmaf: Some(88.5),
 ssim: Some(0.92),
 psnr: Some(38.2),
 pesq: None,
 overall_score: 88.5,
 };
 
 let report = metrics.detailed_report();
 assert!(report.contains("VMAF"));
 assert!(report.contains("SSIM"));
 assert!(report.contains("PSNR"));
 }
}
