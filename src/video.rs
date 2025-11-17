use crate::types::QualityMode;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::{Command, Stdio};

/// 规范化的视频特征
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFeatures {
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub bitrate_kbps: u32,
    pub duration_secs: f32,
    pub has_audio: bool,
    /// Source video codec (e.g. h264/h265/vp9/av1)
    pub codec: String,
    /// Source container (e.g. mp4/mov/webm/mkv)
    pub container: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoCodec {
    H264,
    H265,
    H266, // VVC/H.266 - Next-gen codec
    Vp9,
    Av1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoAudioMode {
    Copy,
    Aac,
    Opus,
    Remove,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoEncodingPlan {
    pub target_codec: VideoCodec,
    pub target_container: String,
    pub crf: u8,
    pub preset: &'static str,
    pub two_pass: bool,
    pub audio_mode: VideoAudioMode,
}

/// 视频预测请求，用于 CLI / 跨语言调用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoPredictionRequest {
    pub features: VideoFeatures,
    pub quality_mode: QualityMode,
}

/// 视频处理器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoProcessorConfig {
    pub quality: u8,
    pub codec: VideoCodec,
    pub hardware_acceleration: bool,
}

impl Default for VideoProcessorConfig {
    fn default() -> Self {
        Self {
            quality: 23,
            codec: VideoCodec::H264,
            hardware_acceleration: false,
        }
    }
}

/// 视频处理器 - 实际执行FFmpeg转换
#[derive(Debug)]
pub struct VideoProcessor {
    config: VideoProcessorConfig,
}

impl VideoProcessor {
    /// 创建新的视频处理器
    pub fn new(config: VideoProcessorConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建
    pub fn with_defaults() -> Self {
        Self::new(VideoProcessorConfig::default())
    }

    /// 检查FFmpeg是否可用
    pub fn check_ffmpeg() -> Result<bool> {
        match Command::new("ffmpeg")
            .arg("-version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
        {
            Ok(status) => Ok(status.success()),
            Err(_) => Ok(false),
        }
    }

    /// 转换视频文件
    pub fn convert<P: AsRef<Path>>(
        &self,
        input: P,
        output: P,
        plan: &VideoEncodingPlan,
    ) -> Result<VideoConversionResult> {
        let input_path = input.as_ref();
        let output_path = output.as_ref();

        if !Self::check_ffmpeg()? {
            anyhow::bail!("FFmpeg is not available");
        }

        tracing::info!(
            "Converting video: {} -> {}",
            input_path.display(),
            output_path.display()
        );

        let start_time = std::time::Instant::now();
        let input_size = std::fs::metadata(input_path)?.len();

        // 构建FFmpeg命令
        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-i").arg(input_path);

        // 添加编解码器参数
        self.add_codec_args(&mut cmd, plan)?;

        // 添加质量参数
        cmd.arg("-crf").arg(plan.crf.to_string());

        // 预设
        cmd.arg("-preset").arg(plan.preset);

        // 音频处理
        match plan.audio_mode {
            VideoAudioMode::Copy => {
                cmd.arg("-c:a").arg("copy");
            }
            VideoAudioMode::Aac => {
                cmd.arg("-c:a").arg("aac");
            }
            VideoAudioMode::Opus => {
                cmd.arg("-c:a").arg("libopus");
            }
            VideoAudioMode::Remove => {
                cmd.arg("-an");
            }
        }

        // 输出文件
        cmd.arg(output_path);
        cmd.arg("-y");

        // 执行转换
        let output = cmd.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("FFmpeg failed: {}", stderr);
        }

        let duration = start_time.elapsed().as_millis() as u64;
        let output_size = std::fs::metadata(output_path)?.len();

        Ok(VideoConversionResult {
            input_path: input_path.to_string_lossy().into_owned(),
            output_path: output_path.to_string_lossy().into_owned(),
            input_size,
            output_size,
            duration_ms: duration,
            codec_used: format!("{:?}", plan.target_codec),
        })
    }

    /// 添加编解码器参数
    fn add_codec_args(&self, cmd: &mut Command, plan: &VideoEncodingPlan) -> Result<()> {
        match plan.target_codec {
            VideoCodec::H264 => {
                cmd.arg("-c:v").arg("libx264");
                if self.config.hardware_acceleration {
                    cmd.arg("-hwaccel").arg("auto");
                }
            }
            VideoCodec::H265 => {
                cmd.arg("-c:v").arg("libx265");
                if self.config.hardware_acceleration {
                    cmd.arg("-hwaccel").arg("auto");
                }
            }
            VideoCodec::H266 => {
                // H.266/VVC support via libvvenc or libvvdec
                // Note: Requires FFmpeg compiled with VVC support
                cmd.arg("-c:v").arg("libvvenc");
                // VVC-specific optimizations
                cmd.arg("-vvenc-params").arg("preset=medium");
                if self.config.hardware_acceleration {
                    cmd.arg("-hwaccel").arg("auto");
                }
            }
            VideoCodec::Vp9 => {
                cmd.arg("-c:v").arg("libvpx-vp9");
            }
            VideoCodec::Av1 => {
                cmd.arg("-c:v").arg("libaom-av1");
            }
        }

        Ok(())
    }

    /// 获取视频信息
    pub fn get_info<P: AsRef<Path>>(input: P) -> Result<VideoInfo> {
        let input_path = input.as_ref();

        let output = Command::new("ffprobe")
            .arg("-v")
            .arg("quiet")
            .arg("-print_format")
            .arg("json")
            .arg("-show_format")
            .arg("-show_streams")
            .arg(input_path)
            .output()?;

        if !output.status.success() {
            anyhow::bail!("ffprobe failed to get video information");
        }

        let metadata = std::fs::metadata(input_path)?;

        Ok(VideoInfo {
            path: input_path.to_path_buf(),
            size_bytes: metadata.len(),
            duration_seconds: 0.0,
            width: 0,
            height: 0,
            fps: 0.0,
            codec: "unknown".to_string(),
        })
    }
}

/// 视频转换结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConversionResult {
    pub input_path: String,
    pub output_path: String,
    pub input_size: u64,
    pub output_size: u64,
    pub duration_ms: u64,
    pub codec_used: String,
}

/// 视频信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub path: std::path::PathBuf,
    pub size_bytes: u64,
    pub duration_seconds: f64,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub codec: String,
}

/// 视频 AI 预测器（纯本地策略，不做 IO/子进程）
#[derive(Default)]
pub struct VideoAIPredictor;

impl VideoAIPredictor {
    pub fn new() -> Self {
        Self
    }

    pub fn predict_video_plan(
        &self,
        features: &VideoFeatures,
        quality_mode: QualityMode,
    ) -> VideoEncodingPlan {
        let is_uhd = features.width >= 3840 || features.height >= 2160;
        let is_8k = features.width >= 7680 || features.height >= 4320;
        let is_hd = features.width >= 1920 || features.height >= 1080;

        // Select target codec
        let target_codec = if is_8k {
            // 8K video prioritizes H.266 for best compression ratio
            VideoCodec::H266
        } else if is_uhd && matches!(quality_mode, QualityMode::Quality | QualityMode::Lossless) {
            // 4K高质量模式可选H.266
            VideoCodec::H266
        } else if is_uhd {
            VideoCodec::H265
        } else if features.container.to_lowercase() == "webm" {
            VideoCodec::Vp9
        } else {
            VideoCodec::H264
        };

        let target_container = if features.container.to_lowercase() == "webm" {
            "webm".to_string()
        } else {
            "mp4".to_string()
        };

        // 基础 CRF 按质量模式和分辨率调整
        let base_crf: u8 = match quality_mode {
            QualityMode::Speed => 28,
            QualityMode::Balanced => 24,
            QualityMode::Quality => 20,
            QualityMode::Lossless => 18,
        };

        let mut crf = base_crf;
        if is_uhd {
            crf = crf.saturating_add(2); // 更高分辨率略微放宽 CRF
        } else if !is_hd {
            crf = crf.saturating_sub(2); // 小分辨率可以给更多质量
        }

        // Shorter videos can be more aggressive
        if features.duration_secs < 60.0 {
            crf = crf.saturating_sub(1);
        }

        // 预设速度
        let preset = match quality_mode {
            QualityMode::Speed => "veryfast",
            QualityMode::Balanced => "medium",
            QualityMode::Quality | QualityMode::Lossless => "slow",
        };

        // Two-pass encoding: enabled for long videos + quality priority
        let two_pass = matches!(quality_mode, QualityMode::Quality | QualityMode::Lossless)
            && features.duration_secs > 120.0;

        // 音频策略
        let audio_mode = if !features.has_audio {
            VideoAudioMode::Remove
        } else if target_container == "webm" {
            VideoAudioMode::Opus
        } else {
            VideoAudioMode::Aac
        };

        VideoEncodingPlan {
            target_codec,
            target_container,
            crf,
            preset,
            two_pass,
            audio_mode,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_plan_for_hd_balanced() {
        let predictor = VideoAIPredictor::new();
        let features = VideoFeatures {
            width: 1920,
            height: 1080,
            fps: 30.0,
            bitrate_kbps: 8000,
            duration_secs: 90.0,
            has_audio: true,
            codec: "h264".to_string(),
            container: "mp4".to_string(),
        };

        let plan = predictor.predict_video_plan(&features, QualityMode::Balanced);
        assert_eq!(plan.target_container, "mp4");
        assert!(plan.crf >= 20 && plan.crf <= 30);
        assert_eq!(plan.audio_mode, VideoAudioMode::Aac);
    }

    #[test]
    fn test_video_plan_for_short_webm() {
        let predictor = VideoAIPredictor::new();
        let features = VideoFeatures {
            width: 1280,
            height: 720,
            fps: 30.0,
            bitrate_kbps: 4000,
            duration_secs: 30.0,
            has_audio: true,
            codec: "vp9".to_string(),
            container: "webm".to_string(),
        };

        let plan = predictor.predict_video_plan(&features, QualityMode::Quality);
        assert_eq!(plan.target_container, "webm");
        assert_eq!(plan.audio_mode, VideoAudioMode::Opus);
    }

    #[test]
    fn test_video_plan_for_uhd_quality_two_pass() {
        let predictor = VideoAIPredictor::new();
        let features = VideoFeatures {
            width: 3840,
            height: 2160,
            fps: 30.0,
            bitrate_kbps: 20000,
            duration_secs: 600.0,
            has_audio: true,
            codec: "h264".to_string(),
            container: "mp4".to_string(),
        };

        let plan = predictor.predict_video_plan(&features, QualityMode::Quality);

        // 4K Quality mode now uses H.266 for better compression ratio
        match plan.target_codec {
            VideoCodec::H266 => {}
            _ => panic!("expected H266 for UHD video in Quality mode"),
        }

        assert_eq!(plan.target_container, "mp4");
        assert!(plan.two_pass);
    }

    #[test]
    fn test_video_plan_without_audio_removes_audio() {
        let predictor = VideoAIPredictor::new();
        let features = VideoFeatures {
            width: 1920,
            height: 1080,
            fps: 30.0,
            bitrate_kbps: 8000,
            duration_secs: 120.0,
            has_audio: false,
            codec: "h264".to_string(),
            container: "mp4".to_string(),
        };

        let plan = predictor.predict_video_plan(&features, QualityMode::Balanced);
        assert_eq!(plan.audio_mode, VideoAudioMode::Remove);
    }

    #[test]
    fn test_video_processor_creation() {
        let processor = VideoProcessor::with_defaults();
        assert_eq!(processor.config.quality, 23);
    }

    #[test]
    fn test_video_processor_config_default() {
        let config = VideoProcessorConfig::default();
        assert_eq!(config.quality, 23);
        assert!(matches!(config.codec, VideoCodec::H264));
    }

    #[test]
    fn test_video_plan_for_8k_uses_h266() {
        let predictor = VideoAIPredictor::new();
        let features = VideoFeatures {
            width: 7680,
            height: 4320,
            fps: 60.0,
            bitrate_kbps: 100000,
            duration_secs: 120.0,
            has_audio: true,
            codec: "h265".to_string(),
            container: "mp4".to_string(),
        };

        let plan = predictor.predict_video_plan(&features, QualityMode::Quality);
        
        // 8K video should use H.266 for best compression ratio
        assert!(matches!(plan.target_codec, VideoCodec::H266));
        assert_eq!(plan.target_container, "mp4");
    }

    #[test]
    fn test_video_plan_for_4k_quality_uses_h266() {
        let predictor = VideoAIPredictor::new();
        let features = VideoFeatures {
            width: 3840,
            height: 2160,
            fps: 30.0,
            bitrate_kbps: 50000,
            duration_secs: 300.0,
            has_audio: true,
            codec: "h264".to_string(),
            container: "mp4".to_string(),
        };

        let plan = predictor.predict_video_plan(&features, QualityMode::Quality);
        
        // 4K高质量模式应该使用H.266
        assert!(matches!(plan.target_codec, VideoCodec::H266));
    }
}
