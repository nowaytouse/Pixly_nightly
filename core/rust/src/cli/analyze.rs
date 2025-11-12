/*
 * 🔥 Phase 40.14: 文件分析CLI命令
 * 
 * 职责：
 * - 提供CLI接口调用MediaAnalyzer
 * - 输出JSON格式的文件分析结果
 * 
 * 架构原则（@PROJECT_QUALITY_MANIFESTO.md）：
 * - 使用MediaAnalyzer统一分析接口（避免重复造轮子）
 * - CLI层仅负责格式转换和输出
 * - 真实性原则：真正调用分析器，不造假数据
 */

use std::path::Path;
use anyhow::{Result, Context};
use serde::Serialize;
use pixly_converter::converter::media_analyzer::{MediaAnalyzer, MediaInfo, MediaType};

/// CLI输出格式（向后兼容旧的file-info命令）
#[derive(Serialize)]
pub struct FileAnalysis {
    /// 文件大小（字节）
    pub file_size: u64,
    /// 文件大小（MB，保留2位小数）
    pub file_size_mb: f64,
    /// 图片宽度
    pub width: u32,
    /// 图片高度
    pub height: u32,
    /// 文件格式
    pub format: String,
    /// 是否为动画
    pub is_animated: bool,
    /// 预估帧数（动画GIF/视频）
    pub estimated_frames: u32,
    /// 预估转换时间（秒）
    pub estimated_seconds: f64,
    /// 转换速度类别
    pub speed_category: String,
    /// 媒体类型
    pub media_type: String,
    /// 帧率（动画/视频）
    pub fps: Option<f32>,
    /// 时长（秒，动画/视频）
    pub duration: Option<f32>,
    /// 比特率（kbps，视频）
    pub bitrate: Option<u32>,
    /// 是否有音频（视频）
    pub has_audio: bool,
}

impl From<MediaInfo> for FileAnalysis {
    fn from(info: MediaInfo) -> Self {
        let file_size_mb = info.size as f64 / (1024.0 * 1024.0);
        
        // 计算预估转换时间
        let estimated_seconds = estimate_conversion_time(&info);
        let speed_category = categorize_speed(estimated_seconds);
        
        // 媒体类型转换
        let media_type = match info.media_type {
            MediaType::Video => "video",
            MediaType::Animation => "animation",
            MediaType::Image => "image",
            MediaType::Unknown => "unknown",
        }.to_string();
        
        let is_animated = matches!(info.media_type, MediaType::Animation | MediaType::Video);
        let estimated_frames = info.frame_count.unwrap_or(1);
        
        Self {
            file_size: info.size,
            file_size_mb,
            width: info.resolution.0,
            height: info.resolution.1,
            format: info.format,
            is_animated,
            estimated_frames,
            estimated_seconds,
            speed_category,
            media_type,
            fps: info.fps,
            duration: info.duration,
            bitrate: info.bitrate,
            has_audio: info.has_audio,
        }
    }
}

/// 估算转换时间（基于媒体信息）
fn estimate_conversion_time(info: &MediaInfo) -> f64 {
    match info.media_type {
        MediaType::Video => {
            // 视频：基于时长和分辨率
            let duration = info.duration.unwrap_or(1.0) as f64;
            let pixels = info.resolution.0 as f64 * info.resolution.1 as f64;
            let base_time = duration * 0.5; // 基础时间：时长的一半
            let resolution_factor = (pixels / 2_000_000.0).max(1.0); // 2MP为基准
            base_time * resolution_factor
        }
        MediaType::Animation => {
            // 动画：基于帧数
            let frames = info.frame_count.unwrap_or(30) as f64;
            (frames / 10.0).max(1.0) // 每10帧约1秒
        }
        MediaType::Image => {
            // 静态图片：基于文件大小
            let size_mb = info.size as f64 / (1024.0 * 1024.0);
            if size_mb < 1.0 {
                1.0
            } else if size_mb < 5.0 {
                2.0
            } else if size_mb < 10.0 {
                3.0
            } else {
                5.0
            }
        }
        MediaType::Unknown => 2.0,
    }
}

/// 分类速度
fn categorize_speed(seconds: f64) -> String {
    if seconds < 2.0 {
        "fast".to_string()
    } else if seconds < 5.0 {
        "medium".to_string()
    } else if seconds < 10.0 {
        "slow".to_string()
    } else {
        "very_slow".to_string()
    }
}

/// 分析文件并返回JSON元数据（使用MediaAnalyzer）
pub fn analyze_file(input: &Path) -> Result<FileAnalysis> {
    // ✅ 使用MediaAnalyzer统一接口（避免重复造轮子）
    let analyzer = MediaAnalyzer::new();
    let media_info = analyzer.analyze(input)
        .context("Failed to analyze media file")?;
    
    // 转换为CLI输出格式
    Ok(FileAnalysis::from(media_info))
}

/// CLI命令处理器：输出JSON到stdout
pub fn handle_analyze_file(input: &Path) -> Result<()> {
    let analysis = analyze_file(input)?;
    
    // 输出JSON
    let json = serde_json::to_string_pretty(&analysis)
        .context("Failed to serialize analysis result")?;
    
    println!("{}", json);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_speed_categorization() {
        assert_eq!(categorize_speed(1.0), "fast");
        assert_eq!(categorize_speed(3.0), "medium");
        assert_eq!(categorize_speed(7.0), "slow");
        assert_eq!(categorize_speed(15.0), "very_slow");
    }
}
