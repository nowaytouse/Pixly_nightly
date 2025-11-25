//! Video feature extraction module
//!
//! Provides feature extraction for Python ML video parameter prediction

use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use crate::errors::path_to_str;

/// videofeaturestructure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFeatures {
/// frame
 pub frame_count: u32,
/// frame
 pub fps: f64,
/// when long（）
 pub duration: f64,
/// width
 pub width: u32,
/// height
 pub height: u32,
/// 码（bps）
 pub bitrate: u64,
/// filesize（bytes）
 pub file_size: u64,
/// decoding
 pub codec: String,
/// isno has audio
 pub has_audio: bool,
/// complexity（0-1）
 pub scene_complexity: f64,
}

impl VideoFeatures {
/// pixeltotal
 pub fn pixels(&self) -> u64 {
 (self.width as u64) * (self.height as u64)
 }

/// filesize(MB)
 pub fn size_mb(&self) -> f64 {
 self.file_size as f64 / (1024.0 * 1024.0)
 }

/// widehigh
 pub fn aspect_ratio(&self) -> f64 {
 if self.height > 0 {
 self.width as f64 / self.height as f64
 } else {
 1.0
 }
 }

/// isnoforhighresolutionvideo (>= 1080p)
 pub fn is_high_resolution(&self) -> bool {
 self.width >= 1920 || self.height >= 1080
 }

/// isnoforlongvideo (> 5)
 pub fn is_long_video(&self) -> bool {
 self.duration > 300.0
 }
}

/// Use ffprobe to extract video features
pub fn extract_video_features(path: &Path) -> Result<VideoFeatures> {
// Check if ffprobe is available
 let ffprobe_check = Command::new("ffprobe")
 .arg("-version")
 .output();

 if ffprobe_check.is_err() {
 anyhow::bail!("ffprobe not found. Please install FFmpeg.");
 }

// Call ffprobe to get video info
 let output = Command::new("ffprobe")
 .args([
 "-v", "quiet",
 "-print_format", "json",
 "-show_format",
 "-show_streams",
 path_to_str(path)?
 ])
 .output()
 .context("Failed to execute ffprobe")?;

 if !output.status.success() {
 let stderr = String::from_utf8_lossy(&output.stderr);
 anyhow::bail!("ffprobe failed: {}", stderr);
 }

// Parse JSON
 let info: serde_json::Value = serde_json::from_slice(&output.stdout)
 .context("Failed to parse ffprobe output")?;

// Extract video stream info
 let video_stream = info["streams"]
 .as_array()
 .and_then(|streams| {
 streams.iter().find(|s| s["codec_type"] == "video")
 })
 .context("No video stream found")?;

// Extract audio stream info
 let has_audio = info["streams"]
 .as_array()
 .map(|streams| {
 streams.iter().any(|s| s["codec_type"] == "audio")
 })
 .unwrap_or(false);

// Extract format info
 let format = &info["format"];

// Parse individual fields
 let width = video_stream["width"]
 .as_u64()
 .unwrap_or(0) as u32;

 let height = video_stream["height"]
 .as_u64()
 .unwrap_or(0) as u32;

 let codec = video_stream["codec_name"]
 .as_str()
 .unwrap_or("unknown")
 .to_string();

 let duration = format["duration"]
 .as_str()
 .and_then(|s| s.parse::<f64>().ok())
 .unwrap_or(0.0);

 let bitrate = format["bit_rate"]
 .as_str()
 .and_then(|s| s.parse::<u64>().ok())
 .unwrap_or(0);

 let file_size = format["size"]
 .as_str()
 .and_then(|s| s.parse::<u64>().ok())
 .unwrap_or(0);

// Parse frame rate
 let fps_str = video_stream["r_frame_rate"]
 .as_str()
 .unwrap_or("0/1");

 let fps = parse_fps(fps_str);

// Calculate frame count
 let frame_count = if duration > 0.0 && fps > 0.0 {
 (duration * fps) as u32
 } else {
 video_stream["nb_frames"]
 .as_str()
 .and_then(|s| s.parse::<u32>().ok())
 .unwrap_or(0)
 };

// Estimate scene complexity (based on bitrate and resolution)
 let scene_complexity = estimate_scene_complexity(bitrate, width, height, duration);

 Ok(VideoFeatures {
 frame_count,
 fps,
 duration,
 width,
 height,
 bitrate,
 file_size,
 codec,
 has_audio,
 scene_complexity,
 })
}

/// Parse frame rate string (e.g. "30/1" or "30000/1001")
fn parse_fps(fps_str: &str) -> f64 {
 let parts: Vec<&str> = fps_str.split('/').collect();
 if parts.len() == 2
 && let (Ok(num), Ok(den)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>())
 && den > 0.0 {
 return num / den;
 }
 0.0
}

/// Estimate scene complexity
fn estimate_scene_complexity(bitrate: u64, width: u32, height: u32, duration: f64) -> f64 {
 if duration <= 0.0 {
 return 0.5;
 }

 let pixels = (width as u64) * (height as u64);
 let expected_bitrate = pixels * 30 / 10; // Simple estimate



 if expected_bitrate > 0 {
 (bitrate as f64 / expected_bitrate as f64).min(1.0)
 } else {
 0.5
 }
}

/// Convert video features to 128-dimensional vector (for ML prediction)
pub fn video_features_to_128d(features: &VideoFeatures) -> Vec<f64> {
 let mut vec = Vec::with_capacity(128);

// Basic video features (16 dimensions)
 vec.push(features.width as f64);
 vec.push(features.height as f64);
 vec.push(features.pixels() as f64);
 vec.push(features.size_mb());
 vec.push(features.aspect_ratio());
 vec.push(features.frame_count as f64);
 vec.push(features.fps);
 vec.push(features.duration);
 vec.push(if features.has_audio { 1.0 } else { 0.0 });
 vec.push(features.scene_complexity);
 vec.push(if features.is_high_resolution() { 1.0 } else { 0.0 });
 vec.push(if features.is_long_video() { 1.0 } else { 0.0 });

// Codec encoding
 let codec_code = match features.codec.as_str() {
 "h264" => 1.0,
 "h265" | "hevc" => 2.0,
 "vp9" => 3.0,
 "av1" => 4.0,
 "vp8" => 5.0,
 "mpeg4" => 6.0,
 _ => 0.0,
 };
 vec.push(codec_code);

 vec.push((features.bitrate as f64).ln()); // Log bitrate
 vec.push(0.0); // Reserved
 vec.push(0.0); // Reserved

// Pad to 128 dimensions (other dimensions filled with 0)
 while vec.len() < 128 {
 vec.push(0.0);
 }

 vec
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_parse_fps() {
 assert_eq!(parse_fps("30/1"), 30.0);
 assert_eq!(parse_fps("30000/1001"), 29.97002997002997);
 assert_eq!(parse_fps("invalid"), 0.0);
 }

 #[test]
 fn test_video_features_to_128d() {
 let features = VideoFeatures {
 frame_count: 300,
 fps: 30.0,
 duration: 10.0,
 width: 1920,
 height: 1080,
 bitrate: 5000000,
 file_size: 10000000,
 codec: "h264".to_string(),
 has_audio: true,
 scene_complexity: 0.7,
 };

 let vec = video_features_to_128d(&features);
 assert_eq!(vec.len(), 128);
 assert_eq!(vec[0], 1920.0); // width
 assert_eq!(vec[1], 1080.0); // height
 }
}
