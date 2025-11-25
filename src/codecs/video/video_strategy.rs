/**
 * Video Conversion Strategy - 视频convert策略module
 * 
 * 🔥 Phase 40.24: 视频convert策略enhanced
 * - supportmore格式 (HEVC/VP9/AV1)
 * - 硬件加速 (NVENC/VAAPI/VideoToolbox)
 * - qualityassessment (VMAF)
 */
use serde::{Deserialize, Serialize};
/// video Encodertype
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VideoCodec {
 /// H.264 (x264) - 通用compatibility性最好
 H264,
 /// H.265/HEVC (x265) - highcompression率
 H265,
 /// VP9 - Weboptimization，开source
 VP9,
 /// AV1 - lateststandard，highestcompression率
 AV1,
}

impl VideoCodec {
 /// get FFmpeg Encodername
 pub fn ffmpeg_codec_name(&self) -> &str {
 match self {
 VideoCodec::H264 => "libx264",
 VideoCodec::H265 => "libx265",
 VideoCodec::VP9 => "libvpx-vp9",
 VideoCodec::AV1 => "libaom-av1",
 }
 }
 
 /// getrecommended容format
 pub fn recommended_container(&self) -> &str {
 match self {
 VideoCodec::H264 | VideoCodec::H265 => "mp4",
 VideoCodec::VP9 | VideoCodec::AV1 => "webm",
 }
 }
 
 /// get硬件加速Encoder（ifsupport）
 pub fn hw_encoder(&self, hw_type: &str) -> Option<String> {
 match (self, hw_type) {
 (VideoCodec::H264, "nvenc") => Some("h264_nvenc".to_string()),
 (VideoCodec::H264, "qsv") => Some("h264_qsv".to_string()),
 (VideoCodec::H264, "videotoolbox") => Some("h264_videotoolbox".to_string()),
 (VideoCodec::H265, "nvenc") => Some("hevc_nvenc".to_string()),
 (VideoCodec::H265, "qsv") => Some("hevc_qsv".to_string()),
 (VideoCodec::H265, "videotoolbox") => Some("hevc_videotoolbox".to_string()),
 _ => None,
 }
 }
}

/// videoqualitytarget
#[derive(Debug, Clone, PartialEq)]
pub enum QualityTarget {
 /// highest quality（几乎lossless）
 Highest,
 /// highquality
 High,
 /// balancedquality and size
 Balanced,
 /// priority小file
 Small,
 /// minimumfilesize
 Smallest,
}

impl QualityTarget {
 /// getrecommended CRF value
 pub fn recommended_crf(&self, codec: &VideoCodec) -> u8 {
 match codec {
 VideoCodec::H264 => match self {
 QualityTarget::Highest => 18,
 QualityTarget::High => 20,
 QualityTarget::Balanced => 23,
 QualityTarget::Small => 28,
 QualityTarget::Smallest => 32,
 },
 VideoCodec::H265 => match self {
 QualityTarget::Highest => 22,
 QualityTarget::High => 24,
 QualityTarget::Balanced => 28,
 QualityTarget::Small => 32,
 QualityTarget::Smallest => 36,
 },
 VideoCodec::VP9 => match self {
 QualityTarget::Highest => 15,
 QualityTarget::High => 20,
 QualityTarget::Balanced => 31,
 QualityTarget::Small => 40,
 QualityTarget::Smallest => 50,
 },
 VideoCodec::AV1 => match self {
 QualityTarget::Highest => 20,
 QualityTarget::High => 25,
 QualityTarget::Balanced => 35,
 QualityTarget::Small => 45,
 QualityTarget::Smallest => 55,
 },
 }
 }
}

/// videoconversionstrategy
pub structure VideoConversionStrategy {
 /// Encoderselect
 pub codec: VideoCodec,
 /// CRF qualityvalue
 pub crf: u8,
 /// presetspeed
 pub preset: String,
 /// 硬件加速
 pub hw_accel: Option<String>,
 /// 两遍encoding
 pub two_pass: bool,
 /// recommended容
 pub container: String,
}

impl VideoConversionStrategy {
 /// autoselect最佳strategy
 /// 
 /// based oninputvideofeature and target需求，autoselect最佳conversionstrategy。
 /// 
 /// # Arguments
 /// 
 /// * `width` - videowidth
 /// * `height` - videoheight
 /// * `duration` - video when 长（秒）
 /// * `target` - qualitytarget
 /// * `prefer_web` - is否priority Webcompatibility性
 /// 
 pub fn auto_select(
 width: u32,
 height: u32,
 duration: f64,
 target: QualityTarget,
 prefer_web: bool,
 ) -> Self {
 // based onresolution and 用途select Encoder
 let codec = if prefer_web {
 // Web priorityuse VP9 or H.264
 if width * height > 1920 * 1080 {
 VideoCodec::VP9 // 4K+ using VP9
 } else {
 VideoCodec::H264 // 1080p 及以下using H.264
 }
 } else {
 // not Web prioritycompression率
 if width * height > 1920 * 1080 {
 VideoCodec::H265 // 4K/8K using H.265
 } else {
 VideoCodec::H264 // 1080p 及以下using H.264
 }
 };
 
 // based onqualitytargetget CRF
 let crf = target.recommended_crf(&codec);
 
 // based onresolution and when 长selectpreset
 let preset = if width * height > 1920 * 1080 {
 // 4K+ use fast
 "fast".to_string()
 } else if duration > 300.0 {
 // 长video（>5分钟）use medium
 "medium".to_string()
 } else {
 // 短videouse slow 获得betterquality
 "slow".to_string()
 };
 
 // detection硬件加速（暂 when disabled，needrun when detection）
 let hw_accel = None;
 
 // highqualitytargetconsidering两遍encoding
 let two_pass = matches!(target, QualityTarget::Highest | QualityTarget::High) 
 && duration < 600.0; // 只对<10分钟视频启用
 
 let container = codec.recommended_container().to_string();
 
 Self {
 codec,
 crf,
 preset,
 hw_accel,
 two_pass,
 container,
 }
 }
 
 /// for流媒体optimizationstrategy
 pub fn for_streaming(width: u32, height: u32) -> Self {
 let codec = if width * height > 1920 * 1080 {
 VideoCodec::H265
 } else {
 VideoCodec::H264
 };
 
 Self {
 codec: codec.clone(),
 crf: 23, // 平衡quality
 preset: "veryfast".to_string(), // quickencode
 hw_accel: None,
 two_pass: false,
 container: codec.recommended_container().to_string(),
 }
 }
 
 /// for存档optimizationstrategy
 pub fn for_archive(_width: u32, _height: u32) -> Self {
 let codec = VideoCodec::H265; // 存档优先压缩率
 
 Self {
 codec: codec.clone(),
 crf: 20, // 高quality
 preset: "slow".to_string(), // 慢速高quality
 hw_accel: None,
 two_pass: true, // 两遍encode
 container: codec.recommended_container().to_string(),
 }
 }
 
 /// forWeboptimizationstrategy
 pub fn for_web(width: u32, height: u32) -> Self {
 let codec = if width * height > 1920 * 1080 {
 VideoCodec::VP9
 } else {
 VideoCodec::H264
 };
 
 Self {
 codec: codec.clone(),
 crf: 28, // Web 优化大小
 preset: "medium".to_string(),
 hw_accel: None,
 two_pass: false,
 container: codec.recommended_container().to_string(),
 }
 }
}

/// audioconversionstrategy
#[derive(Debug, Clone)]
pub structure AudioStrategy {
 /// audio Encoder
 pub codec: String,
 /// 比特率（kbps）
 pub bitrate: u32,
 /// sampling率（Hz）
 pub sample_rate: Option<u32>,
 /// 声道数
 pub channels: Option<u8>,
}

impl AudioStrategy {
 /// autoselectaudiostrategy
 pub fn auto_select(video_codec: &VideoCodec, quality_target: &QualityTarget) -> Self {
 let (codec, bitrate) = match video_codec {
 VideoCodec::H264 | VideoCodec::H265 => {
 // MP4 容use AAC
 let bitrate = match quality_target {
 QualityTarget::Highest => 256,
 QualityTarget::High => 192,
 QualityTarget::Balanced => 128,
 QualityTarget::Small => 96,
 QualityTarget::Smallest => 64,
 };
 ("aac".to_string(), bitrate)
 }
 VideoCodec::VP9 | VideoCodec::AV1 => {
 // Web M 容use Opus
 let bitrate = match quality_target {
 QualityTarget::Highest => 192,
 QualityTarget::High => 128,
 QualityTarget::Balanced => 96,
 QualityTarget::Small => 64,
 QualityTarget::Smallest => 48,
 };
 ("libopus".to_string(), bitrate)
 }
 };
 
 Self {
 codec,
 bitrate,
 sample_rate: None, // 保持原样
 channels: None, // 保持原样
 }
 }
 
 /// copiedaudio流（ not 重newencoding）
 pub fn copy() -> Self {
 Self {
 codec: "copy".to_string(),
 bitrate: 0,
 sample_rate: None,
 channels: None,
 }
 }
}
