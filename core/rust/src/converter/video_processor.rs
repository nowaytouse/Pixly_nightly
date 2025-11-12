/*
 * 🔥 Phase 40.13: 视频处理核心模块
 * 
 * 职责：
 * - 视频格式转换（MP4, MOV, WebM, HEVC, AV1）
 * - 视频编码参数优化
 * - 视频质量分析（分辨率、比特率、帧率）
 * - 视频元数据提取
 * 
 * 架构原则：
 * - Rust专注文件处理和实际转换
 * - 调用FFmpeg进行视频处理
 * - 不做AI决策（由Go负责）
 * - 提供详细的进度回调
 */
// 🔧 统一日志系统
use tracing::{info, warn};

use anyhow::{Result, Context, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

/// 视频信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    /// 文件路径
    pub path: PathBuf,
    /// 文件大小（字节）
    pub size: u64,
    /// 视频编码器
    pub codec: String,
    /// 容器格式
    pub container: String,
    /// 分辨率（宽x高）
    pub resolution: (u32, u32),
    /// 帧率
    pub fps: f32,
    /// 比特率（kbps）
    pub bitrate: u32,
    /// 时长（秒）
    pub duration: f32,
    /// 音频编码器
    pub audio_codec: Option<String>,
    /// 是否有音频
    pub has_audio: bool,
}

/// 视频转换配置
#[derive(Debug, Clone)]
pub struct VideoConversionConfig {
    /// 目标编码器（h264, h265/hevc, vp9, av1）
    pub codec: String,
    /// 目标容器（mp4, mov, webm, mkv）
    pub container: String,
    /// CRF质量（0-51，越小质量越高）
    pub crf: u8,
    /// 预设速度（ultrafast, fast, medium, slow, veryslow）
    pub preset: String,
    /// 目标分辨率（None=保持原样）
    pub target_resolution: Option<(u32, u32)>,
    /// 目标帧率（None=保持原样）
    pub target_fps: Option<f32>,
    /// 音频处理（copy, aac, opus, remove）
    pub audio_mode: AudioMode,
    /// 两次编码（更好的质量）
    pub two_pass: bool,
    /// 硬件加速（auto, nvenc, qsv, videotoolbox, none）
    pub hw_accel: String,
}

/// 音频处理模式
#[derive(Debug, Clone, PartialEq)]
pub enum AudioMode {
    /// 复制原音频流
    Copy,
    /// 重新编码为AAC
    AAC { bitrate: u32 },
    /// 重新编码为Opus
    Opus { bitrate: u32 },
    /// 移除音频
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

/// 视频转换结果
#[derive(Debug)]
pub struct VideoConversionResult {
    /// 是否成功
    pub success: bool,
    /// 输出文件路径
    pub output_path: PathBuf,
    /// 原始大小
    pub original_size: u64,
    /// 转换后大小
    pub converted_size: u64,
    /// 压缩比
    pub compression_ratio: f32,
    /// 转换时间（秒）
    pub duration: f32,
    /// 错误信息（如果失败）
    pub error: Option<String>,
}

/// 视频处理器
pub struct VideoProcessor {
    /// FFmpeg路径
    ffmpeg_path: String,
    /// FFprobe路径
    ffprobe_path: String,
}

impl Default for VideoProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoProcessor {
    /// 创建新的视频处理器
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
    
    /// 🔥 Phase 40.42d: 选择硬件加速编码器
    fn select_encoder(&self, codec: &str, hw_accel: &str) -> String {
        // 如果明确要求不使用硬件加速
        if hw_accel == "none" {
            return self.get_software_encoder(codec);
        }
        
        // 如果指定了特定的硬件加速
        if hw_accel != "auto" {
            let hw_encoder = self.try_hardware_encoder(codec, hw_accel);
            if self.check_encoder_available(&hw_encoder) {
                return hw_encoder;
            } else {
                warn!("⚠️  Hardware encoder {} not available, falling back to software", hw_encoder);
                return self.get_software_encoder(codec);
            }
        }
        
        // auto模式：按优先级尝试
        let hw_options = self.detect_available_hardware();
        for hw in hw_options {
            let hw_encoder = self.try_hardware_encoder(codec, &hw);
            if self.check_encoder_available(&hw_encoder) {
                info!("🚀 Using hardware acceleration: {}", hw);
                return hw_encoder;
            }
        }
        
        // 全部失败，使用软件编码
        self.get_software_encoder(codec)
    }
    
    /// 获取软件编码器
    fn get_software_encoder(&self, codec: &str) -> String {
        match codec {
            "h264" => "libx264".to_string(),
            "h265" | "hevc" => "libx265".to_string(),
            "vp9" => "libvpx-vp9".to_string(),
            "av1" => "libaom-av1".to_string(),
            _ => "libx265".to_string(),
        }
    }
    
    /// 尝试获取硬件编码器名称
    fn try_hardware_encoder(&self, codec: &str, hw_type: &str) -> String {
        match (codec, hw_type) {
            // NVIDIA NVENC
            ("h264", "nvenc") => "h264_nvenc".to_string(),
            ("h265" | "hevc", "nvenc") => "hevc_nvenc".to_string(),
            
            // Intel QSV
            ("h264", "qsv") => "h264_qsv".to_string(),
            ("h265" | "hevc", "qsv") => "hevc_qsv".to_string(),
            
            // Apple VideoToolbox (macOS)
            ("h264", "videotoolbox") => "h264_videotoolbox".to_string(),
            ("h265" | "hevc", "videotoolbox") => "hevc_videotoolbox".to_string(),
            
            // AMD AMF
            ("h264", "amf") => "h264_amf".to_string(),
            ("h265" | "hevc", "amf") => "hevc_amf".to_string(),
            
            // 不支持的组合，返回软件编码
            _ => self.get_software_encoder(codec),
        }
    }
    
    /// 检测可用的硬件加速（按优先级排序）
    fn detect_available_hardware(&self) -> Vec<String> {
        let mut available = Vec::new();
        
        // macOS优先使用VideoToolbox
        #[cfg(target_os = "macos")]
        {
            if self.check_encoder_available("h264_videotoolbox") {
                available.push("videotoolbox".to_string());
            }
        }
        
        // Windows/Linux检测NVIDIA
        #[cfg(not(target_os = "macos"))]
        {
            if self.check_encoder_available("h264_nvenc") {
                available.push("nvenc".to_string());
            }
            
            // Intel QSV
            if self.check_encoder_available("h264_qsv") {
                available.push("qsv".to_string());
            }
            
            // AMD AMF
            if self.check_encoder_available("h264_amf") {
                available.push("amf".to_string());
            }
        }
        
        available
    }
    
    /// 检查编码器是否可用
    fn check_encoder_available(&self, encoder: &str) -> bool {
        let output = Command::new(&self.ffmpeg_path)
            .arg("-hide_banner")
            .arg("-encoders")
            .output();
        
        if let Ok(output) = output {
            let encoders = String::from_utf8_lossy(&output.stdout);
            encoders.contains(encoder)
        } else {
            false
        }
    }
    
    /// 分析视频信息
    pub fn analyze_video(&self, video_path: &Path) -> Result<VideoInfo> {
        if !video_path.exists() {
            bail!("Video file not found: {:?}", video_path);
        }
        
        // 使用ffprobe获取视频信息
        let output = Command::new(&self.ffprobe_path)
            .args([
                "-v", "quiet",
                "-print_format", "json",
                "-show_format",
                "-show_streams",
                video_path.to_str().unwrap(),
            ])
            .output()
            .context("Failed to run ffprobe")?;
        
        if !output.status.success() {
            bail!("FFprobe failed: {}", String::from_utf8_lossy(&output.stderr));
        }
        
        let json_str = String::from_utf8_lossy(&output.stdout);
        let probe_data: serde_json::Value = serde_json::from_str(&json_str)
            .context("Failed to parse ffprobe JSON")?;
        
        // 提取视频流信息
        let streams = probe_data["streams"].as_array()
            .context("No streams found")?;
        
        let video_stream = streams.iter()
            .find(|s| s["codec_type"] == "video")
            .context("No video stream found")?;
        
        let audio_stream = streams.iter()
            .find(|s| s["codec_type"] == "audio");
        
        let format = &probe_data["format"];
        
        Ok(VideoInfo {
            path: video_path.to_path_buf(),
            size: std::fs::metadata(video_path)?.len(),
            codec: video_stream["codec_name"].as_str().unwrap_or("unknown").to_string(),
            container: format["format_name"].as_str().unwrap_or("unknown").to_string(),
            resolution: (
                video_stream["width"].as_u64().unwrap_or(0) as u32,
                video_stream["height"].as_u64().unwrap_or(0) as u32,
            ),
            fps: video_stream["r_frame_rate"].as_str()
                .and_then(parse_fps)
                .unwrap_or(0.0),
            bitrate: format["bit_rate"].as_str()
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0) / 1000, // 转换为kbps
            duration: format["duration"].as_str()
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(0.0),
            audio_codec: audio_stream.map(|s| 
                s["codec_name"].as_str().unwrap_or("unknown").to_string()
            ),
            has_audio: audio_stream.is_some(),
        })
    }
    
    /// 转换视频
    pub fn convert_video<F>(
        &self,
        input: &Path,
        output: &Path,
        config: &VideoConversionConfig,
        progress_callback: Option<F>,
    ) -> Result<VideoConversionResult>
    where
        F: Fn(f32) + Send + 'static,
    {
        let start_time = std::time::Instant::now();
        let original_size = std::fs::metadata(input)?.len();
        
        // 分析输入视频
        let input_info = self.analyze_video(input)?;
        
        info!("🎬 转换视频: {:?} → {:?}", input.file_name(), output.file_name());
        info!("   源: {} {}x{} @ {}fps", 
                  input_info.codec, 
                  input_info.resolution.0, 
                  input_info.resolution.1,
                  input_info.fps);
        
        // 构建FFmpeg命令
        let mut cmd = Command::new(&self.ffmpeg_path);
        cmd.arg("-i").arg(input)
            .arg("-y") // 覆盖输出文件
            .arg("-hide_banner")
            .arg("-loglevel").arg("info")
            .arg("-progress").arg("pipe:2"); // 输出进度到stderr
        
        // 🔥 Phase 40.38: 添加pixel format转换（GIF/PNG可能有alpha通道，H.265/H.264不支持）
        cmd.arg("-pix_fmt").arg("yuv420p");
        
        // 🔥 Phase 40.42d: 硬件加速编码器选择
        let encoder = self.select_encoder(&config.codec, &config.hw_accel);
        info!("   Encoder: {} (hw_accel: {})", encoder, config.hw_accel);
        cmd.arg("-c:v").arg(&encoder);
        
        // CRF质量
        cmd.arg("-crf").arg(config.crf.to_string());
        
        // 预设
        cmd.arg("-preset").arg(&config.preset);
        
        // 分辨率
        if let Some((width, height)) = config.target_resolution {
            cmd.arg("-s").arg(format!("{}x{}", width, height));
        }
        
        // 帧率
        if let Some(fps) = config.target_fps {
            cmd.arg("-r").arg(fps.to_string());
        }
        
        // 音频处理
        match &config.audio_mode {
            AudioMode::Copy => {
                cmd.arg("-c:a").arg("copy");
            }
            AudioMode::AAC { bitrate } => {
                cmd.arg("-c:a").arg("aac")
                   .arg("-b:a").arg(format!("{}k", bitrate));
            }
            AudioMode::Opus { bitrate } => {
                cmd.arg("-c:a").arg("libopus")
                   .arg("-b:a").arg(format!("{}k", bitrate));
            }
            AudioMode::Remove => {
                cmd.arg("-an"); // 无音频
            }
        }
        
        // 🔥 Phase 40.41: 显式指定容器格式
        cmd.arg("-f").arg(&config.container);
        
        // 🔥 Phase 40.42e: Two-pass编码支持
        if config.two_pass {
            info!("   🎬 Two-pass encoding enabled");
            
            // Pass 1: 分析视频，生成统计
            let passlogfile = std::env::temp_dir().join(format!("ffmpeg2pass_{}", std::process::id()));
            let passlogfile_str = passlogfile.to_string_lossy();
            
            info!("   📊 Pass 1/2: Analyzing video...");
            let mut pass1_cmd = Command::new(&self.ffmpeg_path);
            pass1_cmd.arg("-i").arg(input)
                     .arg("-y")
                     .arg("-hide_banner")
                     .arg("-loglevel").arg("error");
            
            // 复制视频编码参数到Pass 1
            pass1_cmd.arg("-pix_fmt").arg("yuv420p");
            pass1_cmd.arg("-c:v").arg(&encoder);
            pass1_cmd.arg("-crf").arg(config.crf.to_string());
            pass1_cmd.arg("-preset").arg(&config.preset);
            
            if let Some((width, height)) = config.target_resolution {
                pass1_cmd.arg("-s").arg(format!("{}x{}", width, height));
            }
            if let Some(fps) = config.target_fps {
                pass1_cmd.arg("-r").arg(fps.to_string());
            }
            
            // Pass 1参数
            pass1_cmd.arg("-pass").arg("1")
                     .arg("-passlogfile").arg(passlogfile_str.as_ref())
                     .arg("-an")  // Pass 1不需要音频
                     .arg("-f").arg(&config.container)
                     .arg("/dev/null");  // macOS/Linux: 输出到null
            
            // 执行Pass 1
            let pass1_output = pass1_cmd.output()
                .context("Failed to execute pass 1")?;
            
            if !pass1_output.status.success() {
                let stderr = String::from_utf8_lossy(&pass1_output.stderr);
                bail!("Pass 1 failed: {}", stderr);
            }
            
            info!("   ✅ Pass 1 complete");
            info!("   🎬 Pass 2/2: Final encoding...");
            
            // Pass 2: 使用统计文件进行最终编码
            cmd.arg("-pass").arg("2")
               .arg("-passlogfile").arg(passlogfile_str.as_ref());
        }
        
        // 输出文件
        cmd.arg(output);
        
        // 执行转换（或Pass 2）
        cmd.stdout(Stdio::piped())
           .stderr(Stdio::piped());
        
        let mut child = cmd.spawn()
            .context("Failed to spawn ffmpeg")?;
        
        // 处理进度
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
            return Ok(VideoConversionResult {
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
        
        info!("✅ 视频转换完成: {:.2}MB → {:.2}MB (压缩比: {:.2}x)", 
                  original_size as f32 / 1024.0 / 1024.0,
                  converted_size as f32 / 1024.0 / 1024.0,
                  compression_ratio);
        
        // 🔥 Phase 40.42e: 清理two-pass临时文件
        if config.two_pass {
            let passlogfile = std::env::temp_dir().join(format!("ffmpeg2pass_{}", std::process::id()));
            let _ = std::fs::remove_file(&passlogfile);
            let _ = std::fs::remove_file(format!("{}-0.log", passlogfile.display()));
            info!("   🗑️  Cleaned up two-pass temporary files");
        }
        
        Ok(VideoConversionResult {
            success: true,
            output_path: output.to_path_buf(),
            original_size,
            converted_size,
            compression_ratio,
            duration: start_time.elapsed().as_secs_f32(),
            error: None,
        })
    }
}

/// 解析帧率字符串（如"30/1"）
fn parse_fps(fps_str: &str) -> Option<f32> {
    let parts: Vec<&str> = fps_str.split('/').collect();
    if parts.len() == 2 {
        let num: f32 = parts[0].parse().ok()?;
        let den: f32 = parts[1].parse().ok()?;
        Some(num / den)
    } else {
        fps_str.parse().ok()
    }
}

/// 从FFmpeg输出解析当前时间
fn parse_ffmpeg_time(line: &str) -> Option<f32> {
    if line.starts_with("out_time_ms=") {
        let time_str = line.strip_prefix("out_time_ms=")?;
        let time_us: i64 = time_str.parse().ok()?;
        Some(time_us as f32 / 1_000_000.0)
    } else {
        None
    }
}
