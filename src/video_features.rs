//! 🎬 视频特征提取模块
//! 
//! 为Python ML视频参数预测提供特征提取

use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use crate::errors::path_to_str;

/// 视频特征结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFeatures {
    /// 帧数
    pub frame_count: u32,
    /// 帧率
    pub fps: f64,
    /// 时长（秒）
    pub duration: f64,
    /// 宽度
    pub width: u32,
    /// 高度
    pub height: u32,
    /// 码率（bps）
    pub bitrate: u64,
    /// 文件大小（字节）
    pub file_size: u64,
    /// 编解码器
    pub codec: String,
    /// 是否有音频
    pub has_audio: bool,
    /// 场景复杂度（0-1）
    pub scene_complexity: f64,
}

impl VideoFeatures {
    /// 像素总数
    pub fn pixels(&self) -> u64 {
        (self.width as u64) * (self.height as u64)
    }
    
    /// 文件大小(MB)
    pub fn size_mb(&self) -> f64 {
        self.file_size as f64 / (1024.0 * 1024.0)
    }
    
    /// 宽高比
    pub fn aspect_ratio(&self) -> f64 {
        if self.height > 0 {
            self.width as f64 / self.height as f64
        } else {
            1.0
        }
    }
    
    /// 是否为高分辨率视频 (>= 1080p)
    pub fn is_high_resolution(&self) -> bool {
        self.width >= 1920 || self.height >= 1080
    }
    
    /// 是否为长视频 (> 5分钟)
    pub fn is_long_video(&self) -> bool {
        self.duration > 300.0
    }
}

/// 使用ffprobe提取视频特征
pub fn extract_video_features(path: &Path) -> Result<VideoFeatures> {
    // 检查ffprobe是否可用
    let ffprobe_check = Command::new("ffprobe")
        .arg("-version")
        .output();
    
    if ffprobe_check.is_err() {
        anyhow::bail!("ffprobe not found. Please install FFmpeg.");
    }
    
    // 调用ffprobe获取视频信息
    let output = Command::new("ffprobe")
        .args(&[
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
    
    // 解析JSON
    let info: serde_json::Value = serde_json::from_slice(&output.stdout)
        .context("Failed to parse ffprobe output")?;
    
    // 提取视频流信息
    let video_stream = info["streams"]
        .as_array()
        .and_then(|streams| {
            streams.iter().find(|s| s["codec_type"] == "video")
        })
        .context("No video stream found")?;
    
    // 提取音频流信息
    let has_audio = info["streams"]
        .as_array()
        .map(|streams| {
            streams.iter().any(|s| s["codec_type"] == "audio")
        })
        .unwrap_or(false);
    
    // 提取格式信息
    let format = &info["format"];
    
    // 解析各个字段
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
    
    // 解析帧率
    let fps_str = video_stream["r_frame_rate"]
        .as_str()
        .unwrap_or("0/1");
    
    let fps = parse_fps(fps_str);
    
    // 计算帧数
    let frame_count = if duration > 0.0 && fps > 0.0 {
        (duration * fps) as u32
    } else {
        video_stream["nb_frames"]
            .as_str()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0)
    };
    
    // 估算场景复杂度（基于码率和分辨率）
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

/// 解析帧率字符串（如 "30/1" 或 "30000/1001"）
fn parse_fps(fps_str: &str) -> f64 {
    let parts: Vec<&str> = fps_str.split('/').collect();
    if parts.len() == 2 {
        if let (Ok(num), Ok(den)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
            if den > 0.0 {
                return num / den;
            }
        }
    }
    0.0
}

/// 估算场景复杂度
fn estimate_scene_complexity(bitrate: u64, width: u32, height: u32, duration: f64) -> f64 {
    if duration <= 0.0 {
        return 0.5;
    }
    
    let pixels = (width as u64) * (height as u64);
    let expected_bitrate = pixels * 30 / 10;  // 简单估算
    
    let complexity = if expected_bitrate > 0 {
        (bitrate as f64 / expected_bitrate as f64).min(1.0)
    } else {
        0.5
    };
    
    complexity
}

/// 将视频特征转换为128维向量（用于ML预测）
pub fn video_features_to_128d(features: &VideoFeatures) -> Vec<f64> {
    let mut vec = Vec::with_capacity(128);
    
    // Basic video features (16维)
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
    
    // 编解码器编码
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
    
    vec.push((features.bitrate as f64).ln());  // 对数码率
    vec.push(0.0);  // 保留
    vec.push(0.0);  // 保留
    
    // 补齐到128维（其他维度用0填充）
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
        assert_eq!(vec[0], 1920.0);  // width
        assert_eq!(vec[1], 1080.0);  // height
    }
}
