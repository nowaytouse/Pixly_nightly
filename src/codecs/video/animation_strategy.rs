// 🎬 animationconversionstrategySystem
// intelligentselectoptimalanimationprocessingstrategy

use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};

/// animationconversionstrategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationStrategy {
/// FFmpegprocessing（largeframeGIF）
 FFmpegBased,
/// frame（smallframeanimation）
 FrameStitching,
/// conversion（staticorextremelyfewframe）
 DirectConversion,
/// outputforvideo
 VideoOutput,
}

/// animationinformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationInfo {
 pub frame_count: u32,
 pub fps: f32,
 pub duration_secs: f32,
 pub width: u32,
 pub height: u32,
 pub file_size: u64,
 pub has_alpha: bool, // whetherhaschannel - key！
}

/// animationconfiguration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationPreservation {
/// originalframe（defaulttrue）
 pub preserve_frame_count: bool,
/// originalFPS（defaulttrue）
 pub preserve_fps: bool,
/// originalframe
 pub original_frame_count: u32,
/// originalFPS
 pub original_fps: f32,
/// targetFPS（NoneuseoriginalFPS）
 pub target_fps: Option<f32>,
}

impl Default for AnimationPreservation {
 fn default() -> Self {
 Self {
 preserve_frame_count: true, // defaultframe
 preserve_fps: true, // defaultFPS
 original_frame_count: 0,
 original_fps: 0.0,
 target_fps: None, // defaultusingoriginalFPS
 }
 }
}

/// animationstrategyselect
pub struct AnimationStrategySelector;

impl AnimationStrategySelector {
/// selectoptimalstrategy
/// based onactualfilefeature，whilenotformatname！
/// strategyfullyfileinside：frame、transparencydegree、resolution
 pub fn select_strategy(
 info: &AnimationInfo,
 _target_format: &str, // notagainusing！formatnotpolicy！
 output_as_video: bool,
 ) -> AnimationStrategy {
// ifrequirementoutputforvideo
 if output_as_video {
 return AnimationStrategy::VideoOutput;
 }

// Static image or single frame - based on actual frame count
 if info.frame_count <= 1 {
 return AnimationStrategy::DirectConversion;
 }

// Strategy completely based on file actual features, don't trust format!

// Animation + transparency: needs special handling to ensure transparency quality
 if info.has_alpha {
 if info.frame_count > 50 {
// Large frame transparent animation: FFmpeg processing
 AnimationStrategy::FFmpegBased
 } else {
// Small frame transparent animation: frame stitching ensures quality
 AnimationStrategy::FrameStitching
 }
 } else {
// No transparency: select by frame count and resolution
 let pixel_count = info.width * info.height;
 let is_high_res = pixel_count > 1920 * 1080;

 if info.frame_count > 100 || (info.frame_count > 50 && is_high_res) {
// Large frame count or high resolution: FFmpeg processing
 AnimationStrategy::FFmpegBased
 } else if info.frame_count > 10 {
// Medium frame count: frame stitching
 AnimationStrategy::FrameStitching
 } else {
// Small frame count: direct conversion
 AnimationStrategy::DirectConversion
 }
 }
 }

/// Calculate recommended target FPS
 pub fn recommend_target_fps(info: &AnimationInfo, strategy: AnimationStrategy) -> f32 {
 match strategy {
 AnimationStrategy::FFmpegBased => {
// FFmpeg processing, keep original FPS (no limit)
 info.fps
 }
 AnimationStrategy::FrameStitching => {
// Frame stitching, keep original FPS
 info.fps
 }
 AnimationStrategy::DirectConversion => {
// Direct conversion, keep original FPS
 info.fps
 }
 AnimationStrategy::VideoOutput => {
// Video output, keep original FPS (no limit)
 info.fps
 }
 }
 }

/// processingtime
 pub fn estimate_processing_time(info: &AnimationInfo, strategy: AnimationStrategy) -> f64 {
 let base_time = (info.frame_count as f64) * 0.1; // 0.1 seconds per frame

 match strategy {
 AnimationStrategy::FFmpegBased => base_time * 0.5, // FFmpeg is fast
 AnimationStrategy::FrameStitching => base_time * 1.5, // Frame stitching is slow
 AnimationStrategy::DirectConversion => base_time * 0.8, // Direct conversion is medium
 AnimationStrategy::VideoOutput => base_time * 0.6, // Video output is relatively fast
 }
 }

/// Generate strategy description
 pub fn get_strategy_description(strategy: AnimationStrategy) -> &'static str {
 match strategy {
 AnimationStrategy::FFmpegBased =>
 "Process large frame animations with FFmpeg, best performance",
 AnimationStrategy::FrameStitching =>
 "Process medium frame animations with frame stitching, best quality",
 AnimationStrategy::DirectConversion =>
 "Direct conversion for small frame animations, fastest speed",
 AnimationStrategy::VideoOutput =>
 "Output as video format, best compatibility",
 }
 }
}

/// animationvideo Converter
pub struct AnimationToVideoConverter;

impl AnimationToVideoConverter {
/// will animationconversionforvideo
 pub fn convert_to_video(
 input: &Path,
 output: &Path,
 info: &AnimationInfo,
 preservation: &AnimationPreservation,
 ) -> Result<()> {
 use std::process::Command;

 let target_fps = preservation.target_fps.unwrap_or(info.fps);

// Convert using FFmpeg
 let output_result = Command::new("ffmpeg")
 .arg("-i").arg(input)
 .arg("-r").arg(target_fps.to_string())
 .arg("-c:v").arg("libx264")
 .arg("-preset").arg("medium")
 .arg("-crf").arg("23")
 .arg("-pix_fmt").arg("yuv420p")
 .arg("-y")
 .arg(output)
 .output()
 .context("Failed to execute FFmpeg")?;

 if !output_result.status.success() {
 let stderr = String::from_utf8_lossy(&output_result.stderr);
 anyhow::bail!("FFmpeg conversion failed: {}", stderr);
 }

 Ok(())
 }

/// check FFmpegisnoavailable
 pub fn is_ffmpeg_available() -> bool {
 Command::new("ffmpeg")
 .arg("-version")
 .output()
 .is_ok()
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_strategy_selection() {
 let info = AnimationInfo {
 frame_count: 150,
 fps: 30.0,
 duration_secs: 5.0,
 width: 800,
 height: 600,
 file_size: 5_000_000,
 has_alpha: false,
 };

 let strategy = AnimationStrategySelector::select_strategy(&info, "avif", false);
 assert_eq!(strategy, AnimationStrategy::FFmpegBased);
 }

 #[test]
 fn test_small_frame_strategy() {
 let info = AnimationInfo {
 frame_count: 5,
 fps: 10.0,
 duration_secs: 0.5,
 width: 400,
 height: 300,
 file_size: 500_000,
 has_alpha: false,
 };

 let strategy = AnimationStrategySelector::select_strategy(&info, "webp", false);
 assert_eq!(strategy, AnimationStrategy::DirectConversion);
 }

 #[test]
 fn test_video_output_strategy() {
 let info = AnimationInfo {
 frame_count: 50,
 fps: 24.0,
 duration_secs: 2.0,
 width: 1920,
 height: 1080,
 file_size: 10_000_000,
 has_alpha: false,
 };

// requirementoutputforvideo
 let strategy = AnimationStrategySelector::select_strategy(&info, "mp4", true);
 assert_eq!(strategy, AnimationStrategy::VideoOutput);

// not requirementvideooutput when ，based onactualfeatureselect
 let strategy2 = AnimationStrategySelector::select_strategy(&info, "mp4", false);
 assert!(matches!(strategy2, AnimationStrategy::FFmpegBased | AnimationStrategy::FrameStitching));
 }

 #[test]
 fn test_fps_recommendation() {
 let info = AnimationInfo {
 frame_count: 100,
 fps: 60.0,
 duration_secs: 1.67,
 width: 1920,
 height: 1080,
 file_size: 8_000_000,
 has_alpha: false,
 };

 let fps = AnimationStrategySelector::recommend_target_fps(
 &info,
 AnimationStrategy::FFmpegBased
 );
 assert_eq!(fps, 60.0); // originalFPS
 }

 #[test]
 fn test_static_image() {
 let info = AnimationInfo {
 frame_count: 1,
 fps: 0.0,
 duration_secs: 0.0,
 width: 1920,
 height: 1080,
 file_size: 2_000_000,
 has_alpha: false,
 };

 let strategy = AnimationStrategySelector::select_strategy(&info, "avif", false);
 assert_eq!(strategy, AnimationStrategy::DirectConversion);
 }

 #[test]
 fn test_transparent_animation_strategy() {
// testtransparencyanimationstrategyselect
 let info_with_alpha = AnimationInfo {
 frame_count: 60,
 fps: 30.0,
 duration_secs: 2.0,
 width: 800,
 height: 600,
 file_size: 3_000_000,
 has_alpha: true, // haschannel
 };

// transparencyanimationshouldusing FFmpegor Frame Stitching
 let strategy = AnimationStrategySelector::select_strategy(&info_with_alpha, "webp", false);
 assert!(matches!(strategy, AnimationStrategy::FFmpegBased | AnimationStrategy::FrameStitching));
 }
}
