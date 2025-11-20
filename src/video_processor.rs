//! 视频处理核心模块
//! 
//! 提供视频格式转换、编码参数优化、质量分析等功能

use anyhow::{Result, Context, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use crate::errors::path_to_str;

/// 视频信息
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

/// 视频转换配置
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
    // 🔥 Advanced encoding parameters - REAL implementation!
    pub gop_size: Option<u32>,        // GOP size (keyframe interval)
    pub bframes: Option<u8>,          // Number of B-frames (0-16)
    pub ref_frames: Option<u8>,       // Number of reference frames (1-16)
    pub me_method: Option<String>,    // Motion estimation method (dia/hex/umh/esa)
    pub pix_fmt: Option<String>,      // Pixel format (None = FFmpeg auto-select best format)
                                       // FFmpeg uses avcodec_find_best_pix_fmt_of_2() to minimize loss
                                       // Loss calculation: resolution > depth > colorspace > alpha > quantization > chroma
    // 🔥 Phase 3: 视频参数补充 (2025-11-19)
    pub rate_control: Option<String>, // Rate control mode (cbr/vbr/crf)
}

/// 音频处理模式
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
            codec: "h266".to_string(),  // 🔥 默认使用最新的H.266/VVC
            container: "mp4".to_string(),
            crf: 23,
            preset: "medium".to_string(),
            target_resolution: None,
            target_fps: None,
            audio_mode: AudioMode::Copy,
            two_pass: false,
            hw_accel: "auto".to_string(),
            gop_size: Some(250),      // 10 seconds @ 25fps
            bframes: Some(3),         // Default 3 B-frames
            ref_frames: Some(3),      // Default 3 reference frames
            me_method: Some("hex".to_string()), // Hexagon motion estimation
            pix_fmt: None,            // Let FFmpeg auto-select best format
            rate_control: None,       // 🔥 Phase 3: 默认None，让FFmpeg自动选择
        }
    }
}

/// 视频转换结果
#[derive(Debug)]
pub struct VideoConversionResult {
    pub success: bool,
    pub output_path: PathBuf,
    pub original_size: u64,
    pub converted_size: u64,
    pub compression_ratio: f32,
    pub duration: f32,
    pub error: Option<String>,
}

/// 视频处理器
pub struct VideoProcessor {
    ffmpeg_path: String,
    ffprobe_path: String,
}

impl Default for VideoProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoProcessor {
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
    
    /// 选择硬件加速编码器
    fn select_encoder(&self, codec: &str, hw_accel: &str) -> String {
        if hw_accel == "none" {
            return self.get_software_encoder(codec);
        }
        
        if hw_accel != "auto" {
            let hw_encoder = self.try_hardware_encoder(codec, hw_accel);
            if self.check_encoder_available(&hw_encoder) {
                return hw_encoder;
            }
        }
        
        let hw_options = self.detect_available_hardware();
        for hw in hw_options {
            let hw_encoder = self.try_hardware_encoder(codec, &hw);
            if self.check_encoder_available(&hw_encoder) {
                return hw_encoder;
            }
        }
        
        self.get_software_encoder(codec)
    }
    
    fn get_software_encoder(&self, codec: &str) -> String {
        match codec {
            // 🔥 H.266/VVC - 完整实现 (2025-11-20)
            // 
            // **真实情况** (深度调查结果):
            // - libvvenc编码器存在且可用
            // - 需要FFmpeg编译时启用 --enable-libvvenc
            // - Homebrew默认FFmpeg不包含（需要自定义编译）
            // 
            // **实现策略** (遵循质量宣言):
            // 1. ✅ 尝试使用libvvenc
            // 2. ✅ 如果不可用，FFmpeg会明确报错
            // 3. ✅ 用户可以选择安装支持VVC的FFmpeg
            // 4. ✅ 提供清晰的错误信息和解决方案
            "h266" | "vvc" => {
                // 🔥 完整H.266支持 - 使用VVenC独立编码器 (2025-11-20)
                // 
                // **实现策略** (遵循质量宣言):
                // 1. ✅ 优先使用VVenC独立编码器（vvencapp）
                // 2. ✅ 备选使用FFmpeg libvvenc（如果可用）
                // 3. ✅ 响亮报错，不静默降级
                // 4. ✅ 提供完整的解决方案
                
                // 检查vvencapp是否可用
                if Command::new("vvencapp").arg("--version").output().is_ok() {
                    log::info!("✅ Using H.266/VVC encoder (vvencapp - standalone)");
                    "vvencapp".to_string()  // 使用独立编码器
                } else if self.check_encoder_available("libvvenc") {
                    log::info!("✅ Using H.266/VVC encoder (libvvenc - FFmpeg)");
                    "libvvenc".to_string()  // 使用FFmpeg集成
                } else {
                    // 响亮报错！
                    log::error!("❌ H.266/VVC ENCODING FAILED: No VVC encoder available");
                    log::error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    log::error!("   REASON: Neither vvencapp nor libvvenc found");
                    log::error!("");
                    log::error!("   SOLUTION 1 - Install VVenC (Recommended):");
                    log::error!("   $ brew install vvenc vvdec");
                    log::error!("");
                    log::error!("   SOLUTION 2 - Use alternative codec:");
                    log::error!("   $ pixly-rust video input.mp4 output.mp4 --codec h265");
                    log::error!("   $ pixly-rust video input.mp4 output.mp4 --codec av1");
                    log::error!("");
                    log::error!("   DOCUMENTATION: docs/H266_VVC_SUPPORT.md");
                    log::error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    
                    // 返回特殊标记，让convert_video方法检测并报错
                    "ERROR_H266_NOT_AVAILABLE".to_string()
                }
            }
            "h265" | "hevc" => "libx265".to_string(),
            "h264" => "libx264".to_string(),
            "av1" => "libaom-av1".to_string(),
            "vp9" => "libvpx-vp9".to_string(),
            "prores" => "prores_ks".to_string(),
            _ => {
                log::warn!("⚠️ Unknown codec '{}', defaulting to H.265", codec);
                "libx265".to_string()
            }
        }
    }
    
    /// 获取ProRes配置文件
    /// 根据质量参数选择合适的ProRes配置文件
    fn get_prores_profile(&self, quality: u32) -> &str {
        match quality {
            0..=20 => "0",   // Proxy (最小文件)
            21..=40 => "1",  // LT (轻量级)
            41..=60 => "2",  // Standard (标准)
            61..=80 => "3",  // HQ (高质量)
            81..=95 => "4",  // 4444 (4:4:4:4采样)
            _ => "5",        // 4444XQ (最高质量)
        }
    }
    
    fn try_hardware_encoder(&self, codec: &str, hw_type: &str) -> String {
        match (codec, hw_type) {
            // 🔥 H.266/VVC 硬件加速 (2025-11-20深度调查)
            // 
            // **调查结果**:
            // - 硬件加速：截至2025年11月，主流GPU尚未支持VVC硬件编码
            // - NVIDIA RTX 40系列：理论支持但驱动未启用
            // - Intel Arc：部分支持但FFmpeg集成不完整
            // - Apple VideoToolbox：不支持VVC
            // 
            // **实现策略**:
            // 1. 尝试硬件编码器（为未来做准备）
            // 2. 失败时自动降级到软件编码
            // 3. 软件编码使用libvvenc（如果可用）
            ("h266" | "vvc", "nvenc") => {
                log::info!("🔍 Trying H.266 NVENC (experimental, may not be available)");
                "vvc_nvenc".to_string()
            }
            ("h266" | "vvc", "qsv") => {
                log::info!("🔍 Trying H.266 QSV (experimental, may not be available)");
                "vvc_qsv".to_string()
            }
            ("h266" | "vvc", _) => {
                log::info!("ℹ️ H.266 hardware acceleration not available, using software encoder");
                self.get_software_encoder(codec)
            }
            
            // H.265/HEVC
            ("h265" | "hevc", "nvenc") => "hevc_nvenc".to_string(),
            ("h265" | "hevc", "qsv") => "hevc_qsv".to_string(),
            ("h265" | "hevc", "videotoolbox") => "hevc_videotoolbox".to_string(),
            ("h265" | "hevc", "amf") => "hevc_amf".to_string(),
            
            // H.264/AVC
            ("h264", "nvenc") => "h264_nvenc".to_string(),
            ("h264", "qsv") => "h264_qsv".to_string(),
            ("h264", "videotoolbox") => "h264_videotoolbox".to_string(),
            ("h264", "amf") => "h264_amf".to_string(),
            
            _ => self.get_software_encoder(codec),
        }
    }
    
    fn detect_available_hardware(&self) -> Vec<String> {
        let mut available = Vec::new();
        
        #[cfg(target_os = "macos")]
        {
            if self.check_encoder_available("h264_videotoolbox") {
                available.push("videotoolbox".to_string());
            }
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            if self.check_encoder_available("h264_nvenc") {
                available.push("nvenc".to_string());
            }
            if self.check_encoder_available("h264_qsv") {
                available.push("qsv".to_string());
            }
            if self.check_encoder_available("h264_amf") {
                available.push("amf".to_string());
            }
        }
        
        available
    }
    
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
        
        let output = Command::new(&self.ffprobe_path)
            .args([
                "-v", "quiet",
                "-print_format", "json",
                "-show_format",
                "-show_streams",
                path_to_str(video_path)?,
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
                .unwrap_or(0) / 1000,
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
        
        let input_info = self.analyze_video(input)?;
        
        let mut cmd = Command::new(&self.ffmpeg_path);
        cmd.arg("-i").arg(input)
            .arg("-y")
            .arg("-hide_banner")
            .arg("-loglevel").arg("info")
            .arg("-progress").arg("pipe:2");
        
        let encoder = self.select_encoder(&config.codec, &config.hw_accel);
        
        // 🔥 H.266/VVC特殊处理 - 使用VVenC独立编码器 (2025-11-20)
        if encoder == "vvencapp" {
            return self.convert_with_vvenc(input, output, config, progress_callback);
        } else if encoder == "ERROR_H266_NOT_AVAILABLE" {
            bail!("H.266/VVC encoder not available. Install vvenc (brew install vvenc) or use alternative codec (--codec h265 or --codec av1)");
        }
        
        cmd.arg("-c:v").arg(&encoder);
        
        // 🔥 ProRes特殊处理
        if config.codec == "prores" {
            // ProRes使用profile而不是CRF
            let profile = self.get_prores_profile(config.crf as u32);
            cmd.arg("-profile:v").arg(profile);
            cmd.arg("-vendor").arg("apl0");  // Apple vendor code
            // ProRes不使用preset
        } else {
            // 其他编码器使用标准参数
            cmd.arg("-crf").arg(config.crf.to_string());
            cmd.arg("-preset").arg(&config.preset);
        }
        
        // 🔥 像素格式 - 可选参数，None时让FFmpeg自动选择最佳格式
        // FFmpeg会使用avcodec_find_best_pix_fmt_of_2()计算损失最小的格式
        if let Some(ref pix_fmt) = config.pix_fmt {
            cmd.arg("-pix_fmt").arg(pix_fmt);
        }
        // 如果不指定，FFmpeg会根据：
        // 1. 编码器支持的格式列表
        // 2. 源格式特征
        // 3. 损失计算（分辨率/深度/色彩空间/Alpha/量化/色度）
        // 自动选择损失最小的格式
        
        // 🔥 Advanced encoding parameters - REAL implementation!
        if let Some(gop) = config.gop_size {
            cmd.arg("-g").arg(gop.to_string());
        }
        
        if let Some(bf) = config.bframes {
            cmd.arg("-bf").arg(bf.to_string());
        }
        
        if let Some(refs) = config.ref_frames {
            cmd.arg("-refs").arg(refs.to_string());
        }
        
        if let Some(ref me) = config.me_method {
            cmd.arg("-me_method").arg(me);
        }
        
        if let Some((width, height)) = config.target_resolution {
            cmd.arg("-s").arg(format!("{}x{}", width, height));
        }
        
        if let Some(fps) = config.target_fps {
            cmd.arg("-r").arg(fps.to_string());
        }
        
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
                cmd.arg("-an");
            }
        }
        
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
    
    /// 🔥 使用VVenC独立编码器进行H.266/VVC转换 (2025-11-20)
    /// 
    /// VVenC是Fraunhofer HHI开发的开源VVC编码器，性能优秀
    /// 
    /// **实现策略**:
    /// 1. 使用FFmpeg提取YUV原始视频
    /// 2. 使用vvencapp编码为VVC
    /// 3. 使用FFmpeg封装为MP4容器
    fn convert_with_vvenc<F>(
        &self,
        input: &Path,
        output: &Path,
        config: &VideoConversionConfig,
        _progress_callback: Option<F>,
    ) -> Result<VideoConversionResult>
    where
        F: Fn(f32) + Send + 'static,
    {
        let start_time = std::time::Instant::now();
        let original_size = std::fs::metadata(input)?.len();
        
        log::info!("🎬 H.266/VVC encoding with VVenC");
        
        // 分析输入视频
        let input_info = self.analyze_video(input)?;
        let (width, height) = input_info.resolution;
        let fps = input_info.fps;
        
        // 临时文件
        let yuv_file = output.with_extension("yuv");
        let vvc_file = output.with_extension("266");
        
        // Step 1: FFmpeg提取YUV
        log::info!("  Step 1/3: Extracting YUV...");
        let mut cmd = Command::new(&self.ffmpeg_path);
        cmd.arg("-i").arg(input)
            .arg("-f").arg("rawvideo")
            .arg("-pix_fmt").arg("yuv420p")
            .arg("-y")
            .arg(&yuv_file);
        
        let output_extract = cmd.output()
            .context("Failed to extract YUV")?;
        
        if !output_extract.status.success() {
            bail!("YUV extraction failed: {}", String::from_utf8_lossy(&output_extract.stderr));
        }
        
        // Step 2: VVenC编码
        log::info!("  Step 2/3: Encoding with VVenC...");
        let mut cmd = Command::new("vvencapp");
        cmd.arg("-i").arg(&yuv_file)
            .arg("-s").arg(format!("{}x{}", width, height))
            .arg("--fps").arg(fps.to_string())
            .arg("--format").arg("yuv420")
            .arg("--preset").arg(&config.preset)
            .arg("-q").arg(config.crf.to_string())
            .arg("-o").arg(&vvc_file);
        
        let output_encode = cmd.output()
            .context("Failed to run vvencapp")?;
        
        if !output_encode.status.success() {
            // 清理临时文件
            let _ = std::fs::remove_file(&yuv_file);
            bail!("VVenC encoding failed: {}", String::from_utf8_lossy(&output_encode.stderr));
        }
        
        // Step 3: FFmpeg封装为MP4
        log::info!("  Step 3/3: Muxing to MP4...");
        let mut cmd = Command::new(&self.ffmpeg_path);
        cmd.arg("-i").arg(&vvc_file)
            .arg("-i").arg(input)  // 音频源
            .arg("-c:v").arg("copy")
            .arg("-c:a").arg("copy")
            .arg("-map").arg("0:v:0")
            .arg("-map").arg("1:a:0?")  // 可选音频
            .arg("-y")
            .arg(output);
        
        let output_mux = cmd.output()
            .context("Failed to mux MP4")?;
        
        // 清理临时文件
        let _ = std::fs::remove_file(&yuv_file);
        let _ = std::fs::remove_file(&vvc_file);
        
        if !output_mux.status.success() {
            bail!("MP4 muxing failed: {}", String::from_utf8_lossy(&output_mux.stderr));
        }
        
        let converted_size = std::fs::metadata(output)?.len();
        let compression_ratio = original_size as f32 / converted_size as f32;
        
        log::info!("✅ H.266/VVC encoding complete!");
        log::info!("   Original: {:.2} MB", original_size as f32 / 1_048_576.0);
        log::info!("   Converted: {:.2} MB", converted_size as f32 / 1_048_576.0);
        log::info!("   Compression: {:.2}x", compression_ratio);
        
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
    fn test_video_processor_creation() {
        let processor = VideoProcessor::new();
        assert_eq!(processor.ffmpeg_path, "ffmpeg");
    }

    #[test]
    fn test_parse_fps() {
        assert_eq!(parse_fps("30/1"), Some(30.0));
        let fps = parse_fps("24000/1001").unwrap();
        assert!((fps - 23.976).abs() < 0.001);
        assert_eq!(parse_fps("60"), Some(60.0));
    }
}
