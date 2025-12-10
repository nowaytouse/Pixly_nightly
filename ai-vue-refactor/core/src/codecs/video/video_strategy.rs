/**
 * Video Conversion Strategy - video conversion strategy module
 *
 * Phase 40.24: Video conversion strategy enhancement
 * - Support more formats (HEVC/VP9/AV1)
 * - Hardware acceleration (NVENC/VAAPI/VideoToolbox)
 * - Quality assessment (VMAF)
 */
use serde::{Deserialize, Serialize};

/// Video encoder type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VideoCodec {
    /// H.264 (x264) - most compatible
    H264,
    /// H.265/HEVC (x265) - high compression
    H265,
    /// VP9 - Web optimized, open source
    VP9,
    /// AV1 - latest standard, highest compression
    AV1,
}

impl VideoCodec {
    /// Get FFmpeg encoder name
    pub fn ffmpeg_codec_name(&self) -> &str {
        match self {
            VideoCodec::H264 => "libx264",
            VideoCodec::H265 => "libx265",
            VideoCodec::VP9 => "libvpx-vp9",
            VideoCodec::AV1 => "libaom-av1",
        }
    }

    /// Get recommended container format
    pub fn recommended_container(&self) -> &str {
        match self {
            VideoCodec::H264 | VideoCodec::H265 => "mp4",
            VideoCodec::VP9 | VideoCodec::AV1 => "webm",
        }
    }

    /// Get hardware encoder (if supported)
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

/// Video quality target
#[derive(Debug, Clone, PartialEq)]
pub enum QualityTarget {
    /// Highest quality (almost lossless)
    Highest,
    /// High quality
    High,
    /// Balanced quality and size
    Balanced,
    /// Prioritize small file size
    Small,
    /// Minimum file size
    Smallest,
}

impl QualityTarget {
    /// Get recommended CRF value
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

/// Video conversion strategy
pub struct VideoConversionStrategy {
    /// Encoder selection
    pub codec: VideoCodec,
    /// CRF quality value
    pub crf: u8,
    /// Preset speed
    pub preset: String,
    /// Hardware acceleration
    pub hw_accel: Option<String>,
    /// Two-pass encoding
    pub two_pass: bool,
    /// Recommended container
    pub container: String,
}

impl VideoConversionStrategy {
    /// Automatically select the best strategy
    ///
    /// Based on input video features and target needs, automatically select the best conversion strategy.
    ///
    /// # Arguments
    ///
    /// * `width` - Video width
    /// * `height` - Video height
    /// * `duration` - Video duration (seconds)
    /// * `target` - Quality target
    /// * `prefer_web` - Whether to prioritize Web compatibility
    ///
    pub fn auto_select(
        width: u32,
        height: u32,
        duration: f64,
        target: QualityTarget,
        prefer_web: bool,
    ) -> Self {
        // Select encoder based on resolution
        let codec = if prefer_web {
            // Web priority: use VP9 or H.264
            if width * height > 1920 * 1080 {
                VideoCodec::VP9 // 4K+ use VP9
            } else {
                VideoCodec::H264 // 1080p and below use H.264
            }
        } else {
            // Non-Web priority: better compression
            if width * height > 1920 * 1080 {
                VideoCodec::H265 // 4K/8K use H.265
            } else {
                VideoCodec::H264 // 1080p and below use H.264
            }
        };

        // Get CRF based on quality target
        let crf = target.recommended_crf(&codec);

        // Select preset based on resolution and duration
        let preset = if width * height > 1920 * 1080 {
            // 4K+ use fast preset
            "fast".to_string()
        } else if duration > 300.0 {
            // Long video (>5 min) use medium
            "medium".to_string()
        } else {
            // Short video use slow for better quality
            "slow".to_string()
        };

        // Hardware acceleration detection (currently disabled, need runtime detection)
        let hw_accel = None;

        // High quality targets consider two-pass encoding
        let two_pass = matches!(target, QualityTarget::Highest | QualityTarget::High)
            && duration < 600.0; // Only enable for <10 min videos

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

    /// Strategy for streaming optimization
    pub fn for_streaming(width: u32, height: u32) -> Self {
        let codec = if width * height > 1920 * 1080 {
            VideoCodec::H265
        } else {
            VideoCodec::H264
        };

        Self {
            codec: codec.clone(),
            crf: 23, // Balanced quality
            preset: "veryfast".to_string(), // Fast encoding
            hw_accel: None,
            two_pass: false,
            container: codec.recommended_container().to_string(),
        }
    }

    /// Strategy for archive optimization
    pub fn for_archive(_width: u32, _height: u32) -> Self {
        let codec = VideoCodec::H265; // Prioritize compression

        Self {
            codec: codec.clone(),
            crf: 20, // High quality
            preset: "slow".to_string(), // Slow for high quality
            hw_accel: None,
            two_pass: true, // Two-pass encoding
            container: codec.recommended_container().to_string(),
        }
    }

    /// Strategy for Web optimization
    pub fn for_web(width: u32, height: u32) -> Self {
        let codec = if width * height > 1920 * 1080 {
            VideoCodec::VP9
        } else {
            VideoCodec::H264
        };

        Self {
            codec: codec.clone(),
            crf: 28, // Web optimized size
            preset: "medium".to_string(),
            hw_accel: None,
            two_pass: false,
            container: codec.recommended_container().to_string(),
        }
    }
}

/// Audio conversion strategy
#[derive(Debug, Clone)]
pub struct AudioStrategy {
    /// Audio encoder
    pub codec: String,
    /// Bitrate (kbps)
    pub bitrate: u32,
    /// Sample rate (Hz)
    pub sample_rate: Option<u32>,
    /// Number of channels
    pub channels: Option<u8>,
}

impl AudioStrategy {
    /// Automatically select audio strategy
    pub fn auto_select(video_codec: &VideoCodec, quality_target: &QualityTarget) -> Self {
        let (codec, bitrate) = match video_codec {
            VideoCodec::H264 | VideoCodec::H265 => {
                // MP4 use AAC
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
                // WebM use Opus
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
            sample_rate: None, // Keep original
            channels: None,    // Keep original
        }
    }

    /// Copy audio stream (no re-encoding)
    pub fn copy() -> Self {
        Self {
            codec: "copy".to_string(),
            bitrate: 0,
            sample_rate: None,
            channels: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_codec_ffmpeg_name() {
        assert_eq!(VideoCodec::H264.ffmpeg_codec_name(), "libx264");
        assert_eq!(VideoCodec::H265.ffmpeg_codec_name(), "libx265");
        assert_eq!(VideoCodec::VP9.ffmpeg_codec_name(), "libvpx-vp9");
        assert_eq!(VideoCodec::AV1.ffmpeg_codec_name(), "libaom-av1");
    }

    #[test]
    fn test_video_codec_container() {
        assert_eq!(VideoCodec::H264.recommended_container(), "mp4");
        assert_eq!(VideoCodec::H265.recommended_container(), "mp4");
        assert_eq!(VideoCodec::VP9.recommended_container(), "webm");
        assert_eq!(VideoCodec::AV1.recommended_container(), "webm");
    }

    #[test]
    fn test_hw_encoder_nvenc() {
        assert_eq!(
            VideoCodec::H264.hw_encoder("nvenc"),
            Some("h264_nvenc".to_string())
        );
        assert_eq!(
            VideoCodec::H265.hw_encoder("nvenc"),
            Some("hevc_nvenc".to_string())
        );
        assert_eq!(VideoCodec::VP9.hw_encoder("nvenc"), None);
    }

    #[test]
    fn test_hw_encoder_videotoolbox() {
        assert_eq!(
            VideoCodec::H264.hw_encoder("videotoolbox"),
            Some("h264_videotoolbox".to_string())
        );
        assert_eq!(
            VideoCodec::H265.hw_encoder("videotoolbox"),
            Some("hevc_videotoolbox".to_string())
        );
    }

    #[test]
    fn test_quality_target_crf() {
        // H264 CRF values
        assert_eq!(QualityTarget::Highest.recommended_crf(&VideoCodec::H264), 18);
        assert_eq!(QualityTarget::Balanced.recommended_crf(&VideoCodec::H264), 23);
        assert_eq!(QualityTarget::Smallest.recommended_crf(&VideoCodec::H264), 32);

        // H265 should have higher CRF for same quality
        assert!(
            QualityTarget::Balanced.recommended_crf(&VideoCodec::H265)
                > QualityTarget::Balanced.recommended_crf(&VideoCodec::H264)
        );
    }

    #[test]
    fn test_auto_select_web_preference() {
        // 4K with web preference should use VP9
        let strategy = VideoConversionStrategy::auto_select(
            3840, 2160, 60.0,
            QualityTarget::Balanced,
            true, // prefer_web
        );
        assert_eq!(strategy.codec, VideoCodec::VP9);
        assert_eq!(strategy.container, "webm");

        // 1080p with web preference should use H.264 for compatibility
        let strategy = VideoConversionStrategy::auto_select(
            1920, 1080, 60.0,
            QualityTarget::Balanced,
            true,
        );
        assert_eq!(strategy.codec, VideoCodec::H264);
    }

    #[test]
    fn test_auto_select_archive_preference() {
        // 4K without web preference should use H.265
        let strategy = VideoConversionStrategy::auto_select(
            3840, 2160, 60.0,
            QualityTarget::Balanced,
            false, // no prefer_web
        );
        assert_eq!(strategy.codec, VideoCodec::H265);
        assert_eq!(strategy.container, "mp4");
    }

    #[test]
    fn test_auto_select_two_pass() {
        // High quality + short duration should enable two-pass
        let strategy = VideoConversionStrategy::auto_select(
            1920, 1080, 60.0,
            QualityTarget::Highest,
            false,
        );
        assert!(strategy.two_pass);

        // Balanced quality should not enable two-pass
        let strategy = VideoConversionStrategy::auto_select(
            1920, 1080, 60.0,
            QualityTarget::Balanced,
            false,
        );
        assert!(!strategy.two_pass);

        // Very long video should not enable two-pass even for high quality
        let strategy = VideoConversionStrategy::auto_select(
            1920, 1080, 700.0, // >600 seconds
            QualityTarget::Highest,
            false,
        );
        assert!(!strategy.two_pass);
    }

    #[test]
    fn test_for_streaming() {
        let strategy = VideoConversionStrategy::for_streaming(1920, 1080);
        assert_eq!(strategy.preset, "veryfast");
        assert!(!strategy.two_pass);
    }

    #[test]
    fn test_for_archive() {
        let strategy = VideoConversionStrategy::for_archive(1920, 1080);
        assert_eq!(strategy.codec, VideoCodec::H265);
        assert_eq!(strategy.preset, "slow");
        assert!(strategy.two_pass);
    }

    #[test]
    fn test_for_web() {
        let strategy = VideoConversionStrategy::for_web(1920, 1080);
        assert_eq!(strategy.codec, VideoCodec::H264);
        assert_eq!(strategy.crf, 28);
    }

    #[test]
    fn test_audio_strategy_auto_select() {
        // MP4 codecs should use AAC
        let audio = AudioStrategy::auto_select(&VideoCodec::H264, &QualityTarget::Balanced);
        assert_eq!(audio.codec, "aac");
        assert_eq!(audio.bitrate, 128);

        // WebM codecs should use Opus
        let audio = AudioStrategy::auto_select(&VideoCodec::VP9, &QualityTarget::Balanced);
        assert_eq!(audio.codec, "libopus");
        assert_eq!(audio.bitrate, 96);
    }

    #[test]
    fn test_audio_strategy_quality_scaling() {
        let highest = AudioStrategy::auto_select(&VideoCodec::H264, &QualityTarget::Highest);
        let smallest = AudioStrategy::auto_select(&VideoCodec::H264, &QualityTarget::Smallest);
        assert!(highest.bitrate > smallest.bitrate);
    }

    #[test]
    fn test_audio_copy() {
        let audio = AudioStrategy::copy();
        assert_eq!(audio.codec, "copy");
        assert_eq!(audio.bitrate, 0);
    }
}
