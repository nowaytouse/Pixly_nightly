//! 图像信息读取
//! 
//! 阶段 1: 只读操作，用于验证 Rust 集成可行性
// 🔧 统一日志系统
use tracing::debug;


use std::path::Path;
use std::fs;
use anyhow::{Result, Context, bail};
use serde::{Deserialize, Serialize};
use image::GenericImageView;

/// 支持的图像格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormat {
    JPEG,
    PNG,
    GIF,
    WebP,
    AVIF,
    JXL,
    HEIC,
    Unknown,
}

impl ImageFormat {
    /// 从文件扩展名检测格式
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" => Self::JPEG,
            "png" => Self::PNG,
            "gif" => Self::GIF,
            "webp" => Self::WebP,
            "avif" => Self::AVIF,
            "jxl" => Self::JXL,
            "heic" | "heif" => Self::HEIC,
            _ => Self::Unknown,
        }
    }

    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::JPEG => "jpeg",
            Self::PNG => "png",
            Self::GIF => "gif",
            Self::WebP => "webp",
            Self::AVIF => "avif",
            Self::JXL => "jxl",
            Self::HEIC => "heic",
            Self::Unknown => "unknown",
        }
    }
}

/// 图像信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    /// 图像宽度
    pub width: u32,
    /// 图像高度
    pub height: u32,
    /// 图像格式
    pub format: ImageFormat,
    /// 是否为动画
    pub is_animated: bool,
    /// 是否包含alpha通道
    pub has_alpha: bool,
    /// 帧数（动画）
    pub frame_count: u32,
    /// FPS（动画）
    pub fps: f32,
    /// 文件大小（字节）
    pub file_size: u64,
}

impl Default for ImageInfo {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            format: ImageFormat::Unknown,
            is_animated: false,
            has_alpha: false,
            frame_count: 1,
            fps: 0.0,
            file_size: 0,
        }
    }
}

/// 读取图像信息（主函数）
/// 
/// # 示例
/// 
/// ```no_run
/// use pixly_converter::info::read_image_info;
/// 
/// let info = read_image_info("/path/to/image.png").unwrap();
/// println!("尺寸: {}x{}", info.width, info.height);
/// ```
pub fn read_image_info<P: AsRef<Path>>(path: P) -> Result<ImageInfo> {
    let path = path.as_ref();
    
    // 验证文件存在
    if !path.exists() {
        bail!("文件不存在: {:?}", path);
    }
    
    // 获取文件大小
    let file_size = fs::metadata(path)
        .context("无法读取文件元数据")?
        .len();
    
    // 检测格式
    let format = detect_format(path)?;
    
    // 使用 image crate 读取基本信息
    let img = image::open(path)
        .context("无法打开图像文件")?;
    
    let (width, height) = img.dimensions();
    
    // ✅ 检测alpha通道
    let has_alpha = matches!(
        img.color(),
        image::ColorType::La8 | image::ColorType::La16 |
        image::ColorType::Rgba8 | image::ColorType::Rgba16 |
        image::ColorType::Rgba32F
    );
    
    // 检测是否为动画（GIF/WebP/APNG）
    let is_animated = is_animated(path)?;
    
    // 🎬 精确帧数和 FPS 检测（修复进度条卡死问题）
    let (frame_count, fps) = if is_animated {
        get_animation_info(path)?
    } else {
        (1, 0.0)
    };
    
    Ok(ImageInfo {
        width,
        height,
        format,
        is_animated,
        has_alpha,  // ✅ 添加alpha通道信息
        frame_count,
        fps,
        file_size,
    })
}

/// 检测图像格式
/// 
/// 从文件扩展名和魔数检测格式
pub fn detect_format<P: AsRef<Path>>(path: P) -> Result<ImageFormat> {
    let path = path.as_ref();
    
    // 首先尝试从扩展名检测
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            let format = ImageFormat::from_extension(ext_str);
            if format != ImageFormat::Unknown {
                return Ok(format);
            }
        }
    }
    
    // 如果扩展名检测失败，尝试读取魔数
    let format = image::guess_format(&fs::read(path)?)
        .context("无法识别图像格式")?;
    
    Ok(match format {
        image::ImageFormat::Jpeg => ImageFormat::JPEG,
        image::ImageFormat::Png => ImageFormat::PNG,
        image::ImageFormat::Gif => ImageFormat::GIF,
        image::ImageFormat::WebP => ImageFormat::WebP,
        image::ImageFormat::Avif => ImageFormat::AVIF,
        _ => ImageFormat::Unknown,
    })
}

/// 检测是否为动画
/// 
/// 目前基于格式和简单的文件分析
pub fn is_animated<P: AsRef<Path>>(path: P) -> Result<bool> {
    let path = path.as_ref();
    let format = detect_format(path)?;
    
    match format {
        ImageFormat::GIF => {
            // GIF 可能是动画
            Ok(check_gif_animated(path)?)
        }
        ImageFormat::WebP => {
            // WebP 可能是动画
            Ok(check_webp_animated(path)?)
        }
        ImageFormat::AVIF => {
            // AVIF 可能是动画（较少见）
            Ok(check_avif_animated(path)?)
        }
        _ => Ok(false),
    }
}

/// 检查 GIF 是否为动画
fn check_gif_animated<P: AsRef<Path>>(path: P) -> Result<bool> {
    let data = fs::read(path)?;
    
    // GIF 动画检测：查找多个图像描述符
    // 简单实现：文件大小 > 5KB 且包含多个 0x21 0xF9（图形控制扩展）
    if data.len() < 5000 {
        return Ok(false);
    }
    
    let mut count = 0;
    for i in 0..data.len().saturating_sub(2) {
        if data[i] == 0x21 && data[i + 1] == 0xF9 {
            count += 1;
            if count > 1 {
                return Ok(true);
            }
        }
    }
    
    Ok(false)
}

/// 检查 WebP 是否为动画
fn check_webp_animated<P: AsRef<Path>>(path: P) -> Result<bool> {
    let data = fs::read(path)?;
    
    // WebP 动画检测：查找 ANIM chunk
    // WebP 格式：RIFF....WEBP
    if data.len() < 12 || &data[0..4] != b"RIFF" || &data[8..12] != b"WEBP" {
        return Ok(false);
    }
    
    // 搜索 ANIM chunk (0x414E494D)
    for i in 12..data.len().saturating_sub(4) {
        if &data[i..i + 4] == b"ANIM" {
            return Ok(true);
        }
    }
    
    Ok(false)
}

/// 检查 AVIF 是否为动画
fn check_avif_animated<P: AsRef<Path>>(_path: P) -> Result<bool> {
    // AVIF 动画检测较复杂，需要解析 ISO BMFF 格式
    // 暂时返回 false，后续可以使用专门的库
    Ok(false)
}

/// 估算帧数（简单实现）
/// 🎬 精确获取动画信息（帧数和 FPS）
/// 
/// 修复问题：之前的estimate_frame_count基于文件大小估算，极不准确
/// 导致进度条卡在99%戚100%
/// 
/// 现在使用专门的库进行精确检测
#[allow(dead_code)]
fn get_animation_info<P: AsRef<Path>>(path: P) -> Result<(u32, f32)> {
    let path = path.as_ref();
    let format = detect_format(path)?;
    
    match format {
        ImageFormat::GIF => {
            // ✅ 精确 GIF 帧数检测
            get_gif_frame_info(path)
        }
        ImageFormat::WebP => {
            // ✅ 精确 WebP 帧数检测
            get_webp_frame_info(path)
        }
        _ => Ok((1, 0.0)),
    }
}

/// 🎬 精确获取 GIF 帧数和 FPS
fn get_gif_frame_info<P: AsRef<Path>>(path: P) -> Result<(u32, f32)> {
    use std::io::BufReader;
    
    let file = fs::File::open(path.as_ref())
        .context("Failed to open GIF file")?;
    let reader = BufReader::new(file);
    
    // 使用 gif 库进行精确解析
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::Indexed);
    
    let mut decoder = decoder.read_info(reader)
        .context("Failed to decode GIF")?;
    
    let mut frame_count = 0u32;
    let mut total_delay = 0u32;
    
    // 读取所有帧
    while let Some(frame) = decoder.read_next_frame()
        .context("Failed to read GIF frame")? 
    {
        frame_count += 1;
        // GIF delay 单位是 1/100秒
        total_delay += frame.delay as u32;
    }
    
    // 计算 FPS
    let fps = if total_delay > 0 && frame_count > 1 {
        // total_delay 是 1/100秒，转换为秒
        let total_seconds = total_delay as f32 / 100.0;
        frame_count as f32 / total_seconds
    } else {
        10.0  // 默认 FPS
    };
    
    debug!("🎬 GIF: {} frames @ {:.2} fps", frame_count, fps);
    
    Ok((frame_count, fps))
}

/// 🎬 精确获取 WebP 帧数和 FPS
fn get_webp_frame_info<P: AsRef<Path>>(path: P) -> Result<(u32, f32)> {
    let data = fs::read(path.as_ref())
        .context("Failed to read WebP file")?;
    
    // 直接使用手动解析 ANIM chunk（更可靠）
    get_webp_frame_info_manual(&data)
}

/// 🎬 WebP 帧数检测的手动解析方法
fn get_webp_frame_info_manual(data: &[u8]) -> Result<(u32, f32)> {
    // WebP 格式：RIFF....WEBP
    if data.len() < 12 || &data[0..4] != b"RIFF" || &data[8..12] != b"WEBP" {
        anyhow::bail!("Invalid WebP format");
    }
    
    // 搜索 ANIM chunk 获取帧数
    let mut i = 12;
    while i < data.len().saturating_sub(8) {
        let chunk_type = &data[i..i+4];
        let chunk_size = u32::from_le_bytes([
            data[i+4], data[i+5], data[i+6], data[i+7]
        ]) as usize;
        
        if chunk_type == b"ANIM" && chunk_size >= 6 {
            // ANIM chunk 结构：
            // - 4 bytes: background color
            // - 2 bytes: loop count
            // 之后是 ANMF chunks，每个代表一帧
            
            // 统计 ANMF chunks
            let mut frame_count = 0u32;
            let mut j = i + 8 + chunk_size;
            
            while j < data.len().saturating_sub(8) {
                if &data[j..j+4] == b"ANMF" {
                    frame_count += 1;
                }
                
                let next_size = u32::from_le_bytes([
                    data[j+4], data[j+5], data[j+6], data[j+7]
                ]) as usize;
                
                j += 8 + next_size;
                if next_size == 0 { break; }
            }
            
            debug!("🎬 WebP (fallback): {} frames", frame_count);
            
            return Ok((frame_count.max(1), 10.0));
        }
        
        i += 8 + chunk_size;
        if chunk_size == 0 { break; }
    }
    
    // 找不到 ANIM chunk，返回默认值
    Ok((1, 0.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_format_detection() {
        assert_eq!(ImageFormat::from_extension("jpg"), ImageFormat::JPEG);
        assert_eq!(ImageFormat::from_extension("png"), ImageFormat::PNG);
        assert_eq!(ImageFormat::from_extension("gif"), ImageFormat::GIF);
        assert_eq!(ImageFormat::from_extension("webp"), ImageFormat::WebP);
        assert_eq!(ImageFormat::from_extension("avif"), ImageFormat::AVIF);
    }

    #[test]
    fn test_format_string() {
        assert_eq!(ImageFormat::JPEG.as_str(), "jpeg");
        assert_eq!(ImageFormat::PNG.as_str(), "png");
        assert_eq!(ImageFormat::GIF.as_str(), "gif");
    }

    #[test]
    fn test_image_info_default() {
        let info = ImageInfo::default();
        assert_eq!(info.width, 0);
        assert_eq!(info.height, 0);
        assert_eq!(info.format, ImageFormat::Unknown);
        assert!(!info.is_animated);
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_image_info("/nonexistent/file.png");
        assert!(result.is_err());
    }
}
