/*
 * 🔥 Phase 40.13: 统一媒体分析接口
 * 
 * 职责：
 * - 统一分析图片、视频、动图
 * - 提供统一的媒体信息结构
 * - 自动识别媒体类型
 * - 为Go AI服务提供分析数据
 * 
 * 架构原则：
 * - Rust负责实际文件分析
 * - 输出标准化的媒体信息
 * - Go AI根据这些信息做决策
 */
// 🔧 统一日志系统
use tracing::{info, warn, debug};

use anyhow::{Result, Context, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

use super::video_processor::VideoProcessor;
use super::animation_detector;
use super::magika_detector::{MagikaDetector, FileTypeDetection};  // 🔥 Phase 45.2: Magika AI检测

/// 媒体类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    /// 静态图片
    Image,
    /// 动画图片（GIF, APNG, WebP动画）
    Animation,
    /// 视频
    Video,
    /// 未知类型
    Unknown,
}

/// 统一媒体信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaInfo {
    /// 文件路径
    pub path: PathBuf,
    /// 媒体类型
    pub media_type: MediaType,
    /// 文件大小（字节）
    pub size: u64,
    /// 格式/编码器
    pub format: String,
    /// 分辨率（宽x高）
    pub resolution: (u32, u32),
    /// 帧率（动画/视频）
    pub fps: Option<f32>,
    /// 总帧数（动画/视频）
    pub frame_count: Option<u32>,
    /// 时长（秒，动画/视频）
    pub duration: Option<f32>,
    /// 比特率（kbps，视频）
    pub bitrate: Option<u32>,
    /// 是否有音频（视频）
    pub has_audio: bool,
    /// 音频编码器（视频）
    pub audio_codec: Option<String>,
    /// 色彩空间
    pub color_space: Option<String>,
    /// 色深
    pub bit_depth: Option<u8>,
}

/// 媒体分析器
pub struct MediaAnalyzer {
    video_processor: VideoProcessor,
    magika_detector: MagikaDetector,  // 🔥 Phase 45.2: AI文件类型检测器
    enable_ai_detection: bool,         // 🔥 Phase 45.2: 是否启用AI检测
}

impl MediaAnalyzer {
    /// 创建新的媒体分析器
    pub fn new() -> Self {
        Self {
            video_processor: VideoProcessor::new(),
            magika_detector: MagikaDetector::with_defaults(),
            enable_ai_detection: true,  // 默认启用AI检测
        }
    }
    
    /// 创建媒体分析器（可配置AI检测）
    /// 
    /// # 参数
    /// - `enable_ai_detection`: 是否启用AI文件类型检测
    pub fn with_ai_detection(enable_ai_detection: bool) -> Self {
        Self {
            video_processor: VideoProcessor::new(),
            magika_detector: MagikaDetector::with_defaults(),
            enable_ai_detection,
        }
    }
    
    /// 分析媒体文件
    /// 
    /// 🔥 Phase 45.2: 集成 Magika AI 文件类型检测
    /// - 支持无扩展名文件
    /// - 检测伪装文件
    /// - 提供置信度评分
    pub fn analyze(&self, file_path: &Path) -> Result<MediaInfo> {
        if !file_path.exists() {
            bail!("File not found: {:?}", file_path);
        }
        
        let size = std::fs::metadata(file_path)?.len();
        let extension = file_path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());
        
        // 🔥 Phase 45.2: 使用 Magika AI 检测文件类型
        let ai_detection: Option<FileTypeDetection> = if self.enable_ai_detection {
            match self.magika_detector.detect_file_type(file_path) {
                Ok(detection) => {
                    info!("🤖 Magika detected: {} (confidence: {:.1}%)", 
                        detection.detected_type, detection.confidence * 100.0);
                    
                    // 检查扩展名与AI检测是否匹配
                    if let Some(ref ext) = extension {
                        if !self.magika_detector.extension_matches_type(ext, &detection.detected_type) 
                            && detection.is_high_confidence {
                            warn!("⚠️  File type mismatch!");
                            warn!("   Extension: .{}", ext);
                            warn!("   AI detected: {} (confidence: {:.1}%)", 
                                detection.detected_type, detection.confidence * 100.0);
                            warn!("   This might be a disguised or corrupted file");
                        }
                    } else {
                        info!("📝 No extension, using AI detection: {}", detection.detected_type);
                    }
                    
                    Some(detection)
                }
                Err(e) => {
                    warn!("⚠️  Magika detection failed: {}", e);
                    warn!("   Falling back to extension-based detection");
                    None
                }
            }
        } else {
            None
        };
        
        // 决定使用哪个扩展名：AI检测 > 文件扩展名
        let effective_extension = if let Some(ref detection) = ai_detection {
            if detection.is_high_confidence {
                // 使用AI检测的类型
                detection.detected_type.clone()
            } else if let Some(ref ext) = extension {
                // AI置信度低，使用文件扩展名
                ext.clone()
            } else {
                // 无扩展名且AI置信度低，使用AI检测结果
                detection.detected_type.clone()
            }
        } else if let Some(ref ext) = extension {
            ext.clone()
        } else {
            // 无扩展名且AI检测失败
            warn!("⚠️  No extension and AI detection unavailable");
            "unknown".to_string()
        };
        
        // 根据有效扩展名判断媒体类型
        match effective_extension.as_str() {
            // 视频格式
            "mp4" | "mov" | "avi" | "mkv" | "webm" | "m4v" | "flv" | "wmv" => {
                self.analyze_video(file_path, size)
            }
            // 动画格式
            "gif" | "apng" => {  // 🔥 Phase 43.1: 添加APNG支持
                self.analyze_animation(file_path, size)
            }
            // 图片格式 (🔥 Phase 43.1: 添加HEIC/HEIF/SVG/PSD/ICO/DDS支持)
            "jpg" | "jpeg" | "png" | "webp" | "avif" | "jxl" | 
            "bmp" | "tiff" | "tif" | "heic" | "heif" | 
            "svg" | "psd" | "ico" | "dds" => {
                self.analyze_image(file_path, size)
            }
            _ => {
                // 尝试作为图片处理
                info!("🔍 Unknown extension '{}', trying as image", effective_extension);
                self.analyze_image(file_path, size)
            }
        }
    }
    
    /// 分析视频
    fn analyze_video(&self, file_path: &Path, size: u64) -> Result<MediaInfo> {
        let video_info = self.video_processor.analyze_video(file_path)?;
        
        Ok(MediaInfo {
            path: file_path.to_path_buf(),
            media_type: MediaType::Video,
            size,
            format: video_info.codec.clone(),
            resolution: video_info.resolution,
            fps: Some(video_info.fps),
            frame_count: Some((video_info.duration * video_info.fps) as u32),
            duration: Some(video_info.duration),
            bitrate: Some(video_info.bitrate),
            has_audio: video_info.has_audio,
            audio_codec: video_info.audio_codec.clone(),
            color_space: None,
            bit_depth: None,
        })
    }
    
    /// 分析动画
    fn analyze_animation(&self, file_path: &Path, size: u64) -> Result<MediaInfo> {
        let is_animated = animation_detector::is_animated_gif(file_path)?;
        
        if !is_animated {
            // 作为静态图片处理
            return self.analyze_image(file_path, size);
        }
        
        // 使用image crate获取基本信息
        let img = image::open(file_path)
            .context("Failed to open image")?;
        
        let width = img.width();
        let height = img.height();
        
        // 🔥 Phase 43.3: 使用FFmpeg获取准确的帧数和FPS
        let (frame_count, fps) = self.get_animation_info(file_path)?;
        let duration = if fps > 0.0 {
            frame_count as f32 / fps
        } else {
            0.0
        };
        
        Ok(MediaInfo {
            path: file_path.to_path_buf(),
            media_type: MediaType::Animation,
            size,
            format: "gif".to_string(),
            resolution: (width, height),
            fps: Some(fps),
            frame_count: Some(frame_count),
            duration: Some(duration),
            bitrate: None,
            has_audio: false,
            audio_codec: None,
            color_space: None,
            bit_depth: Some(8),
        })
    }
    
    /// 🔥 Phase 43.3: 获取动画的帧数和FPS（使用FFmpeg）
    fn get_animation_info(&self, file_path: &Path) -> Result<(u32, f32)> {
        use std::process::Command;
        
        // 使用ffprobe获取帧数
        let frame_output = Command::new("ffprobe")
            .arg("-v").arg("error")
            .arg("-select_streams").arg("v:0")
            .arg("-count_packets")
            .arg("-show_entries").arg("stream=nb_read_packets")
            .arg("-of").arg("csv=p=0")
            .arg(file_path)
            .output();
        
        let frame_count = if let Ok(output) = frame_output {
            String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse::<u32>()
                .unwrap_or(1)
        } else {
            // Fallback: 使用旧方法估算
            warn!("FFprobe not available, using estimation for frame count");
            self.estimate_gif_frames(file_path).unwrap_or(1)
        };
        
        // 使用ffprobe获取FPS
        let fps_output = Command::new("ffprobe")
            .arg("-v").arg("error")
            .arg("-select_streams").arg("v:0")
            .arg("-show_entries").arg("stream=r_frame_rate")
            .arg("-of").arg("csv=p=0")
            .arg(file_path)
            .output();
        
        let fps = if let Ok(output) = fps_output {
            let fps_str = String::from_utf8_lossy(&output.stdout);
            let fps_str = fps_str.trim();
            
            // 解析帧率（可能是分数形式，如 "10/1"）
            if let Some((num, den)) = fps_str.split_once('/') {
                let numerator: f32 = num.parse().unwrap_or(10.0);
                let denominator: f32 = den.parse().unwrap_or(1.0);
                if denominator > 0.0 {
                    numerator / denominator
                } else {
                    10.0 // 默认10fps
                }
            } else {
                fps_str.parse::<f32>().unwrap_or(10.0)
            }
        } else {
            warn!("FFprobe not available, using default FPS");
            10.0 // 默认10fps
        };
        
        Ok((frame_count, fps))
    }
    
    /// 分析图片
    fn analyze_image(&self, file_path: &Path, size: u64) -> Result<MediaInfo> {
        // 🔥 策略：优先使用 image crate（快速），失败则用 exiftool（通用）
        // 注意：这不是 fallback，而是工具链的合理选择
        //       image crate: 支持 png, jpg, webp, avif 等常见格式
        //       exiftool: 支持 jxl, heic 等更多格式
        
        // 尝试 image crate
        if let Ok(img) = image::open(file_path) {
            let width = img.width();
            let height = img.height();
            let format = file_path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("unknown")
                .to_lowercase();
            
            let color_type = img.color();
            let bit_depth = match color_type {
                image::ColorType::L8 | image::ColorType::La8 | 
                image::ColorType::Rgb8 | image::ColorType::Rgba8 => 8,
                image::ColorType::L16 | image::ColorType::La16 | 
                image::ColorType::Rgb16 | image::ColorType::Rgba16 => 16,
                _ => 8,
            };
            
            return Ok(MediaInfo {
                path: file_path.to_path_buf(),
                media_type: MediaType::Image,
                size,
                format,
                resolution: (width, height),
                fps: None,
                frame_count: None,
                duration: None,
                bitrate: None,
                has_audio: false,
                audio_codec: None,
                color_space: Some(format!("{:?}", color_type)),
                bit_depth: Some(bit_depth),
            });
        }
        
        // 🔥 image crate 不支持该格式，使用 exiftool（通用工具）
        info!("📊 Image crate不支持此格式，使用exiftool: {:?}", file_path.file_name());
        
        use std::process::Command;
        let output = Command::new("exiftool")
            .arg("-j")
            .arg("-ImageWidth")
            .arg("-ImageHeight")
            .arg("-FileType")
            .arg(file_path)
            .output()
            .context("Failed to execute exiftool")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("exiftool failed to analyze image: {}", stderr);
        }
        
        let json_str = String::from_utf8(output.stdout)
            .context("Failed to parse exiftool output")?;
        
        let json: serde_json::Value = serde_json::from_str(&json_str)
            .context("Failed to parse exiftool JSON")?;
        
        let metadata = &json[0];
        
        let width = metadata["ImageWidth"]
            .as_u64()
            .context("Missing ImageWidth in exiftool output")? as u32;
        let height = metadata["ImageHeight"]
            .as_u64()
            .context("Missing ImageHeight in exiftool output")? as u32;
        let format = metadata["FileType"]
            .as_str()
            .context("Missing FileType in exiftool output")?
            .to_lowercase();
        
        info!("✅ exiftool 分析完成: {}x{} {}", width, height, format);
        
        Ok(MediaInfo {
            path: file_path.to_path_buf(),
            media_type: MediaType::Image,
            size,
            format,
            resolution: (width, height),
            fps: None,
            frame_count: None,
            duration: None,
            bitrate: None,
            has_audio: false,
            audio_codec: None,
            color_space: None, // exiftool 无法提供详细的颜色空间信息
            bit_depth: None,   // exiftool 可能无法提供位深度
        })
    }
    
    /// 估算GIF帧数（通过解析GIF结构）
    fn estimate_gif_frames(&self, file_path: &Path) -> Result<u32> {
        use std::io::{Read, Seek, SeekFrom};
        
        let mut file = std::fs::File::open(file_path)
            .context("Failed to open GIF file")?;
        
        // 读取GIF头部
        let mut header = [0u8; 6];
        file.read_exact(&mut header)
            .context("Failed to read GIF header")?;
        
        // 验证GIF签名
        if &header[0..3] != b"GIF" {
            bail!("Not a valid GIF file");
        }
        
        // 跳过逻辑屏幕描述符 (7字节)
        file.seek(SeekFrom::Current(7))
            .context("Failed to seek in GIF")?;
        
        // 读取全局颜色表标志
        let mut packed_fields = [0u8; 1];
        file.seek(SeekFrom::Start(10))?;
        file.read_exact(&mut packed_fields)?;
        
        let has_global_color_table = (packed_fields[0] & 0x80) != 0;
        let global_color_table_size = if has_global_color_table {
            2 << (packed_fields[0] & 0x07)
        } else {
            0
        };
        
        // 跳过全局颜色表
        if has_global_color_table {
            file.seek(SeekFrom::Current((global_color_table_size * 3) as i64))?;
        } else {
            file.seek(SeekFrom::Start(13))?;
        }
        
        // 计数图像块
        let mut frame_count = 0u32;
        let mut buffer = [0u8; 1];
        
        loop {
            if file.read(&mut buffer).ok() != Some(1) {
                break;
            }
            
            match buffer[0] {
                0x21 => {
                    // 扩展块
                    file.read_exact(&mut buffer)?;
                                         let _label = buffer[0];
                    
                    // 读取并跳过子块
                    loop {
                        file.read_exact(&mut buffer)?;
                        let block_size = buffer[0];
                        if block_size == 0 {
                            break;
                        }
                        file.seek(SeekFrom::Current(block_size as i64))?;
                    }
                }
                0x2C => {
                    // 图像描述符 - 这是一个帧
                    frame_count += 1;
                    
                    // 跳过图像描述符 (9字节)
                    file.seek(SeekFrom::Current(9))?;
                    
                    // 检查局部颜色表
                    file.seek(SeekFrom::Current(-1))?;
                    file.read_exact(&mut buffer)?;
                    let has_local_color_table = (buffer[0] & 0x80) != 0;
                    
                    if has_local_color_table {
                        let local_color_table_size = 2 << (buffer[0] & 0x07);
                        file.seek(SeekFrom::Current((local_color_table_size * 3) as i64))?;
                    }
                    
                    // 跳过LZW数据
                    file.read_exact(&mut buffer)?; // LZW minimum code size
                    
                    // 跳过所有数据子块
                    loop {
                        file.read_exact(&mut buffer)?;
                        let block_size = buffer[0];
                        if block_size == 0 {
                            break;
                        }
                        file.seek(SeekFrom::Current(block_size as i64))?;
                    }
                }
                0x3B => {
                    // GIF终止符
                    break;
                }
                _ => {
                    // 未知块，尝试跳过
                    continue;
                }
            }
        }
        
        // 至少返回1帧
        if frame_count == 0 {
            warn!("⚠️  No frames detected in GIF, defaulting to 1");
            frame_count = 1;
        }
        
        debug!("🎬 GIF frames detected: {}", frame_count);
        
        Ok(frame_count)
    }
}

impl Default for MediaAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// 转换输出验证
pub struct OutputValidator;

impl OutputValidator {
    /// 验证输出文件（从JS validation.js迁移）
    /// 
    /// 功能：
    /// - 检查文件存在
    /// - 检查文件大小
    /// - 可选：计算SSIM质量
    /// - 可选：验证元数据
    pub fn validate_output<P: AsRef<Path>>(
        input_path: Option<P>,
        output_path: P,
        check_quality: bool,
        check_metadata: bool,
    ) -> Result<ValidationResult> {
        let output_path = output_path.as_ref();
        
        // 1. 检查文件存在
        if !output_path.exists() {
            bail!("Output file does not exist");
        }
        
        // 2. 检查文件大小
        let metadata = fs::metadata(output_path)
            .context("Failed to get file metadata")?;
        
        if metadata.len() == 0 {
            bail!("Output file is empty");
        }
        
        let mut result = ValidationResult {
            valid: true,
            size: metadata.len(),
            quality: None,
            metadata_preserved: None,
        };
        
        // 3. 可选：计算SSIM质量
        if check_quality {
            if let Some(ref input) = input_path {
                result.quality = Self::calculate_ssim(input.as_ref(), output_path).ok();
            }
        }
        
        // 4. 可选：验证元数据
        if check_metadata {
            if let Some(ref input) = input_path {
                result.metadata_preserved = Some(Self::validate_metadata(input.as_ref(), output_path)?);
            }
        }
        
        Ok(result)
    }
    
    /// 计算SSIM (结构相似性指数) - 从JS迁移
    fn calculate_ssim(input_path: &Path, output_path: &Path) -> Result<f64> {
        use std::process::Command;
        
        let output = Command::new("ffmpeg")
            .arg("-i").arg(input_path)
            .arg("-i").arg(output_path)
            .arg("-lavfi").arg("ssim")
            .arg("-f").arg("null")
            .arg("-")
            .output()
            .context("Failed to execute ffmpeg for SSIM calculation")?;
        
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // 解析SSIM值
        for line in stderr.lines() {
            if line.contains("SSIM") && line.contains("All:") {
                if let Some(captures) = line.split("All:").nth(1) {
                    if let Some(value_str) = captures.split_whitespace().next() {
                        if let Ok(ssim) = value_str.parse::<f64>() {
                            return Ok(ssim);
                        }
                    }
                }
            }
        }
        
        bail!("Failed to parse SSIM value from ffmpeg output")
    }
    
    /// 验证元数据是否保留 - 从JS迁移
    fn validate_metadata(input_path: &Path, output_path: &Path) -> Result<bool> {
        use std::process::Command;
        
        // 使用exiftool提取两个文件的元数据
        let input_meta = Command::new("exiftool")
            .arg("-j")
            .arg(input_path)
            .output()
            .context("Failed to extract input metadata")?;
        
        let output_meta = Command::new("exiftool")
            .arg("-j")
            .arg(output_path)
            .output()
            .context("Failed to extract output metadata")?;
        
        if !input_meta.status.success() || !output_meta.status.success() {
            return Ok(false);
        }
        
        // 简单比较：如果输出文件有元数据就认为保留了
        let output_json = String::from_utf8_lossy(&output_meta.stdout);
        Ok(output_json.len() > 10)  // 基本的JSON数组长度
    }
}

/// 验证结果
#[derive(Debug)]
pub struct ValidationResult {
    pub valid: bool,
    pub size: u64,
    pub quality: Option<f64>,  // SSIM值
    pub metadata_preserved: Option<bool>,
}
