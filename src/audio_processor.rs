//! 🎵 音频处理核心模块
//! 
//! 提供音频格式转换、编码参数优化、质量分析等功能
//! 与图像和视频处理完全统一的架构

use anyhow::{Result, Context, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

/// 音频信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioInfo {
    pub path: PathBuf,
    pub size: u64,
    pub codec: String,
    pub container: String,
    pub sample_rate: u32,
    pub bit_rate: u32,
    pub duration: f32,
    pub channels: u8,
    pub bit_depth: Option<u8>,
}

/// 音频转换配置
#[derive(Debug, Clone)]
pub struct AudioConversionConfig {
    pub codec: String,
    pub container: String,
    pub bitrate: u32,  // kbps
    pub sample_rate: Option<u32>,
    pub channels: Option<u8>,
    pub quality: u8,  // 0-10 for VBR
}

impl Default for AudioConversionConfig {
    fn default() -> Self {
        Self {
            codec: "opus".to_string(),  // 默认使用最现代的Opus
            container: "ogg".to_string(),
            bitrate: 128,
            sample_rate: None,
            channels: None,
            quality: 5,
        }
    }
}

/// 音频转换结果
#[derive(Debug)]
pub struct AudioConversionResult {
    pub success: bool,
    pub output_path: PathBuf,
    pub original_size: u64,
    pub converted_size: u64,
    pub compression_ratio: f32,
    pub duration: f32,
    pub error: Option<String>,
}

/// 音频处理器
pub struct AudioProcessor {
    ffmpeg_path: String,
    ffprobe_path: String,
}

impl Default for AudioProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioProcessor {
    pub fn new() -> Self {
        Self {
            ffmpeg_path: "ffmpeg".to_string(),
            ffprobe_path: "ffprobe".to_string(),
        }
    }
    
    /// 检查FFmpeg是否可用
    pub fn check_ffmpeg(&self) -> Result<bool> {
        let output = Command::new(&self.ffmpeg_path)
            .arg("-version")
            .output();
        
        Ok(output.is_ok())
    }
    
    /// 分析音频信息
    pub fn analyze_audio(&self, audio_path: &Path) -> Result<AudioInfo> {
        if !audio_path.exists() {
            bail!("Audio file not found: {:?}", audio_path);
        }
        
        let output = Command::new(&self.ffprobe_path)
            .args([
                "-v", "quiet",
                "-print_format", "json",
                "-show_format",
                "-show_streams",
                audio_path.to_str().unwrap(),
            ])
            .output()
            .context("Failed to run ffprobe")?;
        
        if !output.status.success() {
            bail!("FFprobe failed: {}", String::from_utf8_lossy(&output.stderr));
        }
        
        let json_str = String::from_utf8_lossy(&output.stdout);
        let probe_data: serde_json::Value = serde_json::from_str(&json_str)
            .context("Failed to parse ffprobe JSON")?;
        
        let streams = probe_data["streams"].as_array()
            .context("No streams found")?;
        
        let audio_stream = streams.iter()
            .find(|s| s["codec_type"] == "audio")
            .context("No audio stream found")?;
        
        let format = &probe_data["format"];
        
        Ok(AudioInfo {
            path: audio_path.to_path_buf(),
            size: std::fs::metadata(audio_path)?.len(),
            codec: audio_stream["codec_name"].as_str().unwrap_or("unknown").to_string(),
            container: format["format_name"].as_str().unwrap_or("unknown").to_string(),
            sample_rate: audio_stream["sample_rate"].as_str()
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0),
            bit_rate: format["bit_rate"].as_str()
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0) / 1000,
            duration: format["duration"].as_str()
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(0.0),
            channels: audio_stream["channels"].as_u64().unwrap_or(0) as u8,
            bit_depth: audio_stream["bits_per_sample"].as_u64().map(|b| b as u8),
        })
    }
    
    /// 转换音频
    pub fn convert_audio<F>(
        &self,
        input: &Path,
        output: &Path,
        config: &AudioConversionConfig,
        progress_callback: Option<F>,
    ) -> Result<AudioConversionResult>
    where
        F: Fn(f32) + Send + 'static,
    {
        let start_time = std::time::Instant::now();
        let original_size = std::fs::metadata(input)?.len();
        
        let input_info = self.analyze_audio(input)?;
        
        let mut cmd = Command::new(&self.ffmpeg_path);
        cmd.arg("-i").arg(input)
            .arg("-y")
            .arg("-hide_banner")
            .arg("-loglevel").arg("info")
            .arg("-progress").arg("pipe:2");
        
        // 选择编码器
        let encoder = self.select_encoder(&config.codec);
        cmd.arg("-c:a").arg(&encoder);
        
        // 设置比特率或质量
        if config.codec == "opus" || config.codec == "vorbis" {
            // VBR模式
            cmd.arg("-q:a").arg(config.quality.to_string());
        } else {
            // CBR模式
            cmd.arg("-b:a").arg(format!("{}k", config.bitrate));
        }
        
        // 设置采样率
        if let Some(sr) = config.sample_rate {
            cmd.arg("-ar").arg(sr.to_string());
        }
        
        // 设置声道数
        if let Some(ch) = config.channels {
            cmd.arg("-ac").arg(ch.to_string());
        }
        
        // 设置容器格式
        cmd.arg("-f").arg(&config.container);
        cmd.arg(output);
        
        cmd.stdout(Stdio::piped())
           .stderr(Stdio::piped());
        
        let mut child = cmd.spawn()
            .context("Failed to spawn ffmpeg")?;
        
        if let (Some(callback), Some(stderr)) = (progress_callback, child.stderr.take()) {
            let duration = input_info.duration;
            std::thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines().map_while(Result::ok) {
                    if let Some(time) = parse_ffmpeg_time(&line) {
                        let progress = if duration > 0.0 {
                            (time / duration * 100.0).min(100.0)
                        } else {
                            0.0
                        };
                        callback(progress);
                    }
                }
            });
        }
        
        let status = child.wait()
            .context("Failed to wait for ffmpeg")?;
        
        if !status.success() {
            return Ok(AudioConversionResult {
                success: false,
                output_path: output.to_path_buf(),
                original_size,
                converted_size: 0,
                compression_ratio: 0.0,
                duration: start_time.elapsed().as_secs_f32(),
                error: Some("FFmpeg conversion failed".to_string()),
            });
        }
        
        let converted_size = std::fs::metadata(output)?.len();
        let compression_ratio = original_size as f32 / converted_size as f32;
        
        Ok(AudioConversionResult {
            success: true,
            output_path: output.to_path_buf(),
            original_size,
            converted_size,
            compression_ratio,
            duration: start_time.elapsed().as_secs_f32(),
            error: None,
        })
    }
    
    /// 选择编码器
    fn select_encoder(&self, codec: &str) -> String {
        match codec {
            "opus" => "libopus".to_string(),
            "vorbis" => "libvorbis".to_string(),
            "aac" => "aac".to_string(),
            "mp3" => "libmp3lame".to_string(),
            "flac" => "flac".to_string(),
            _ => "libopus".to_string(),  // 默认Opus
        }
    }
    
    /// 获取推荐的音频配置（基于格式知识库）
    pub fn get_recommended_config(&self, source_codec: &str, target_codec: &str) -> AudioConversionConfig {
        // 基于格式知识库的推荐
        match target_codec {
            "opus" => AudioConversionConfig {
                codec: "opus".to_string(),
                container: "ogg".to_string(),
                bitrate: 128,
                sample_rate: Some(48000),
                channels: None,
                quality: 5,
            },
            "aac" => AudioConversionConfig {
                codec: "aac".to_string(),
                container: "m4a".to_string(),
                bitrate: 192,
                sample_rate: Some(48000),
                channels: None,
                quality: 5,
            },
            "flac" => AudioConversionConfig {
                codec: "flac".to_string(),
                container: "flac".to_string(),
                bitrate: 0,  // 无损
                sample_rate: None,
                channels: None,
                quality: 8,
            },
            "mp3" => AudioConversionConfig {
                codec: "mp3".to_string(),
                container: "mp3".to_string(),
                bitrate: 192,
                sample_rate: Some(44100),
                channels: None,
                quality: 2,
            },
            _ => AudioConversionConfig::default(),
        }
    }
}

fn parse_ffmpeg_time(line: &str) -> Option<f32> {
    if line.starts_with("out_time_ms=") {
        let time_str = line.strip_prefix("out_time_ms=")?;
        let time_us: i64 = time_str.parse().ok()?;
        Some(time_us as f32 / 1_000_000.0)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_processor_creation() {
        let processor = AudioProcessor::new();
        assert_eq!(processor.ffmpeg_path, "ffmpeg");
    }

    #[test]
    fn test_default_config() {
        let config = AudioConversionConfig::default();
        assert_eq!(config.codec, "opus");  // 默认使用最现代的Opus
        assert_eq!(config.container, "ogg");
    }
    
    #[test]
    fn test_recommended_configs() {
        let processor = AudioProcessor::new();
        
        // Opus推荐
        let opus_config = processor.get_recommended_config("mp3", "opus");
        assert_eq!(opus_config.codec, "opus");
        assert_eq!(opus_config.sample_rate, Some(48000));
        
        // AAC推荐
        let aac_config = processor.get_recommended_config("mp3", "aac");
        assert_eq!(aac_config.codec, "aac");
        assert_eq!(aac_config.container, "m4a");
        
        // FLAC推荐
        let flac_config = processor.get_recommended_config("mp3", "flac");
        assert_eq!(flac_config.codec, "flac");
        assert_eq!(flac_config.bitrate, 0);  // 无损
    }
}
