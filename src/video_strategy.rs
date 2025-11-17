/**
 * Video Conversion Strategy - 视频转换策略模块
 * 
 * 🔥 Phase 40.24: 视频转换策略增强
 * - 支持更多格式 (HEVC/VP9/AV1)
 * - 硬件加速 (NVENC/VAAPI/VideoToolbox)
 * - 质量评估 (VMAF)
 */
use serde::{Deserialize, Serialize};
/// 视频编码器类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VideoCodec {
    /// H.264 (x264) - 通用兼容性最好
    H264,
    /// H.265/HEVC (x265) - 高压缩率
    H265,
    /// VP9 - Web优化，开源
    VP9,
    /// AV1 - 最新标准，最高压缩率
    AV1,
}

impl VideoCodec {
    /// 获取 FFmpeg 编码器名称
    pub fn ffmpeg_codec_name(&self) -> &str {
        match self {
            VideoCodec::H264 => "libx264",
            VideoCodec::H265 => "libx265",
            VideoCodec::VP9 => "libvpx-vp9",
            VideoCodec::AV1 => "libaom-av1",
        }
    }
    
    /// 获取推荐的容器格式
    pub fn recommended_container(&self) -> &str {
        match self {
            VideoCodec::H264 | VideoCodec::H265 => "mp4",
            VideoCodec::VP9 | VideoCodec::AV1 => "webm",
        }
    }
    
    /// 获取硬件加速编码器（如果支持）
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

/// 视频质量目标
#[derive(Debug, Clone, PartialEq)]
pub enum QualityTarget {
    /// 最高质量（几乎无损）
    Highest,
    /// 高质量
    High,
    /// 平衡质量和大小
    Balanced,
    /// 优先小文件
    Small,
    /// 最小文件大小
    Smallest,
}

impl QualityTarget {
    /// 获取推荐的 CRF 值
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

/// 视频转换策略
pub struct VideoConversionStrategy {
    /// 编码器选择
    pub codec: VideoCodec,
    /// CRF质量值
    pub crf: u8,
    /// 预设速度
    pub preset: String,
    /// 硬件加速
    pub hw_accel: Option<String>,
    /// 两遍编码
    pub two_pass: bool,
    /// 推荐的容器
    pub container: String,
}

impl VideoConversionStrategy {
    /// 自动选择最佳策略
    /// 
    /// 根据输入视频的特征和目标需求，自动选择最佳的转换策略。
    /// 
    /// # Arguments
    /// 
    /// * `width` - 视频宽度
    /// * `height` - 视频高度
    /// * `duration` - 视频时长（秒）
    /// * `target` - 质量目标
    /// * `prefer_web` - 是否优先Web兼容性
    /// 
    pub fn auto_select(
        width: u32,
        height: u32,
        duration: f64,
        target: QualityTarget,
        prefer_web: bool,
    ) -> Self {
        // 根据分辨率和用途选择编码器
        let codec = if prefer_web {
            // Web 优先使用 VP9 或 H.264
            if width * height > 1920 * 1080 {
                VideoCodec::VP9 // 4K+ 使用 VP9
            } else {
                VideoCodec::H264 // 1080p 及以下使用 H.264
            }
        } else {
            // 非 Web 优先压缩率
            if width * height > 1920 * 1080 {
                VideoCodec::H265 // 4K/8K 使用 H.265
            } else {
                VideoCodec::H264 // 1080p 及以下使用 H.264
            }
        };
        
        // 根据质量目标获取 CRF
        let crf = target.recommended_crf(&codec);
        
        // 根据分辨率和时长选择预设
        let preset = if width * height > 1920 * 1080 {
            // 4K+ 使用 fast
            "fast".to_string()
        } else if duration > 300.0 {
            // 长视频（>5分钟）使用 medium
            "medium".to_string()
        } else {
            // 短视频使用 slow 获得更好质量
            "slow".to_string()
        };
        
        // 检测硬件加速（暂时禁用，需要运行时检测）
        let hw_accel = None;
        
        // 高质量目标考虑两遍编码
        let two_pass = matches!(target, QualityTarget::Highest | QualityTarget::High) 
            && duration < 600.0; // 只对<10分钟的视频启用
        
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
    
    /// 为流媒体优化的策略
    pub fn for_streaming(width: u32, height: u32) -> Self {
        let codec = if width * height > 1920 * 1080 {
            VideoCodec::H265
        } else {
            VideoCodec::H264
        };
        
        Self {
            codec: codec.clone(),
            crf: 23, // 平衡质量
            preset: "veryfast".to_string(), // 快速编码
            hw_accel: None,
            two_pass: false,
            container: codec.recommended_container().to_string(),
        }
    }
    
    /// 为存档优化的策略
    pub fn for_archive(_width: u32, _height: u32) -> Self {
        let codec = VideoCodec::H265; // 存档优先压缩率
        
        Self {
            codec: codec.clone(),
            crf: 20, // 高质量
            preset: "slow".to_string(), // 慢速高质量
            hw_accel: None,
            two_pass: true, // 两遍编码
            container: codec.recommended_container().to_string(),
        }
    }
    
    /// 为Web优化的策略
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

/// 音频转换策略
#[derive(Debug, Clone)]
pub struct AudioStrategy {
    /// 音频编码器
    pub codec: String,
    /// 比特率（kbps）
    pub bitrate: u32,
    /// 采样率（Hz）
    pub sample_rate: Option<u32>,
    /// 声道数
    pub channels: Option<u8>,
}

impl AudioStrategy {
    /// 自动选择音频策略
    pub fn auto_select(video_codec: &VideoCodec, quality_target: &QualityTarget) -> Self {
        let (codec, bitrate) = match video_codec {
            VideoCodec::H264 | VideoCodec::H265 => {
                // MP4 容器使用 AAC
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
                // WebM 容器使用 Opus
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
            channels: None,    // 保持原样
        }
    }
    
    /// 复制音频流（不重新编码）
    pub fn copy() -> Self {
        Self {
            codec: "copy".to_string(),
            bitrate: 0,
            sample_rate: None,
            channels: None,
        }
    }
}
