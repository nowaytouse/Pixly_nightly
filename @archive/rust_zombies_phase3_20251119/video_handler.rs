// 🎬 视频处理增强
// 从 @archive/rust_broken/src/converter/video_processor.rs 提取

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub path: PathBuf,
    pub size: u64,
    pub codec: String,
    pub container: String,
    pub resolution: (u32, u32),
    pub fps: f32,
    pub bitrate: u32,
    pub duration: f32,
    pub audio_codec: Option<String>,
    pub has_audio: bool,
}

#[derive(Debug, Clone)]
pub struct VideoConversionConfig {
    pub codec: String,
    pub container: String,
    pub crf: u8,
    pub preset: String,
    pub target_resolution: Option<(u32, u32)>,
    pub target_fps: Option<f32>,
    pub audio_mode: AudioMode,
    pub two_pass: bool,
    pub hw_accel: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AudioMode {
    Copy,
    AAC { bitrate: u32 },
    Opus { bitrate: u32 },
    Remove,
}

impl Default for VideoConversionConfig {
    fn default() -> Self {
        Self {
            codec: "h265".to_string(),
            container: "mp4".to_string(),
            crf: 23,
            preset: "medium".to_string(),
            target_resolution: None,
            target_fps: None,
            audio_mode: AudioMode::Copy,
            two_pass: false,
            hw_accel: "auto".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = VideoConversionConfig::default();
        assert_eq!(config.codec, "h265");
        assert_eq!(config.crf, 23);
    }
}
