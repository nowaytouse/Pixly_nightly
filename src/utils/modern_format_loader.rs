/// 🔥 现代格式图像加载器
///
/// 统一处理 AVIF/JXL/HEIC 等 image crate 不支持的格式
/// 通过外部工具解码后加载
///
/// 支持格式：
/// - AVIF (AV1 Image File Format)
/// - JXL (JPEG XL)
/// - HEIC/HEIF (High Efficiency Image Format)

use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView};
use std::path::Path;
use std::process::Command;

/// 现代格式列表（image crate 不支持解码）
pub const MODERN_FORMATS: &[&str] = &["avif", "jxl", "jpegxl", "heic", "heif"];

/// 检查是否为现代格式
pub fn is_modern_format(path: &Path) -> bool {
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    
    MODERN_FORMATS.contains(&ext.as_str())
}

/// 检查扩展名是否为现代格式
pub fn is_modern_format_ext(ext: &str) -> bool {
    MODERN_FORMATS.contains(&ext.to_lowercase().as_str())
}

/// 🔥 统一的图像加载函数
///
/// 自动检测格式，对现代格式使用外部工具解码
pub fn load_image(path: &Path) -> Result<DynamicImage> {
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    
    if is_modern_format_ext(&ext) {
        load_modern_format(path, &ext)
    } else {
        image::open(path)
            .with_context(|| format!("Failed to open image: {:?}", path))
    }
}

/// 🔥 加载现代格式图像
///
/// 通过外部工具解码为临时 PNG，然后加载
fn load_modern_format(path: &Path, format: &str) -> Result<DynamicImage> {
    // 创建临时文件
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or(std::time::Duration::from_secs(0))
        .as_millis();
    let temp_path = std::env::temp_dir().join(format!("pixly_decode_{}.png", timestamp));
    
    // 使用外部工具解码
    decode_to_png(path, &temp_path, format)?;
    
    // 加载解码后的 PNG
    let img = image::open(&temp_path)
        .with_context(|| format!("Failed to open decoded image: {:?}", temp_path))?;
    
    // 清理临时文件
    let _ = std::fs::remove_file(&temp_path);
    
    Ok(img)
}

/// 使用外部工具解码为 PNG
fn decode_to_png(input: &Path, output: &Path, format: &str) -> Result<()> {
    let input_str = input.to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid input path"))?;
    let output_str = output.to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid output path"))?;
    
    match format {
        "avif" => decode_avif(input_str, output_str),
        "jxl" | "jpegxl" => decode_jxl(input_str, output_str),
        "heic" | "heif" => decode_heic(input_str, output_str),
        _ => anyhow::bail!("Unsupported modern format: {}", format),
    }
}

/// 解码 AVIF
fn decode_avif(input: &str, output: &str) -> Result<()> {
    // 尝试 avifdec
    if let Ok(status) = Command::new("avifdec")
        .args([input, output])
        .status()
    {
        if status.success() {
            return Ok(());
        }
    }
    
    // 尝试 ImageMagick
    if let Ok(status) = Command::new("magick")
        .args(["convert", input, output])
        .status()
    {
        if status.success() {
            return Ok(());
        }
    }
    
    // 尝试 ffmpeg
    if let Ok(status) = Command::new("ffmpeg")
        .args(["-y", "-i", input, "-frames:v", "1", output])
        .status()
    {
        if status.success() {
            return Ok(());
        }
    }
    
    anyhow::bail!("Failed to decode AVIF. Please install avifdec, ImageMagick, or ffmpeg.")
}

/// 解码 JXL
fn decode_jxl(input: &str, output: &str) -> Result<()> {
    // 尝试 djxl
    if let Ok(status) = Command::new("djxl")
        .args([input, output])
        .status()
    {
        if status.success() {
            return Ok(());
        }
    }
    
    // 尝试 ImageMagick
    if let Ok(status) = Command::new("magick")
        .args(["convert", input, output])
        .status()
    {
        if status.success() {
            return Ok(());
        }
    }
    
    anyhow::bail!("Failed to decode JXL. Please install djxl or ImageMagick with JXL support.")
}

/// 解码 HEIC
fn decode_heic(input: &str, output: &str) -> Result<()> {
    // 尝试 ImageMagick
    if let Ok(status) = Command::new("magick")
        .args(["convert", input, output])
        .status()
    {
        if status.success() {
            return Ok(());
        }
    }
    
    // 尝试 heif-convert
    if let Ok(status) = Command::new("heif-convert")
        .args([input, output])
        .status()
    {
        if status.success() {
            return Ok(());
        }
    }
    
    // 尝试 ffmpeg
    if let Ok(status) = Command::new("ffmpeg")
        .args(["-y", "-i", input, "-frames:v", "1", output])
        .status()
    {
        if status.success() {
            return Ok(());
        }
    }
    
    anyhow::bail!("Failed to decode HEIC. Please install ImageMagick, heif-convert, or ffmpeg.")
}

/// 🔥 尝试加载图像（失败返回 None）
///
/// 用于非关键路径，失败时不报错
pub fn try_load_image(path: &Path) -> Option<DynamicImage> {
    load_image(path).ok()
}

/// 获取图像尺寸（支持现代格式）
pub fn get_dimensions(path: &Path) -> Result<(u32, u32)> {
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    
    if is_modern_format_ext(&ext) {
        get_dimensions_external(path, &ext)
    } else {
        let img = image::open(path)?;
        Ok(img.dimensions())
    }
}

/// 通过外部工具获取尺寸
fn get_dimensions_external(path: &Path, format: &str) -> Result<(u32, u32)> {
    let path_str = path.to_str().unwrap_or_default();
    
    // 尝试 ImageMagick identify
    if let Ok(output) = Command::new("identify")
        .args(["-format", "%w %h", path_str])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = stdout.trim().split_whitespace().collect();
            if parts.len() >= 2 {
                if let (Ok(w), Ok(h)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    return Ok((w, h));
                }
            }
        }
    }
    
    // JXL 专用：尝试 jxlinfo
    if format == "jxl" || format == "jpegxl" {
        if let Ok(output) = Command::new("jxlinfo")
            .arg(path_str)
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if line.contains("Size:") || line.contains("size:") {
                        if let Some(caps) = regex::Regex::new(r"(\d+)\s*x\s*(\d+)")
                            .ok()
                            .and_then(|re| re.captures(line))
                        {
                            if let (Some(w), Some(h)) = (caps.get(1), caps.get(2)) {
                                if let (Ok(w), Ok(h)) = (w.as_str().parse::<u32>(), h.as_str().parse::<u32>()) {
                                    return Ok((w, h));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // HEIC 专用：尝试 exiftool
    if format == "heic" || format == "heif" {
        if let Ok(output) = Command::new("exiftool")
            .args(["-ImageWidth", "-ImageHeight", "-s", "-s", "-s", path_str])
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.trim().lines().collect();
                if lines.len() >= 2 {
                    if let (Ok(w), Ok(h)) = (lines[0].parse::<u32>(), lines[1].parse::<u32>()) {
                        return Ok((w, h));
                    }
                }
            }
        }
    }
    
    // 如果所有方法都失败，返回默认值
    log::warn!("Could not determine dimensions for {} file: {:?}, using defaults", format, path);
    Ok((1920, 1080))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_modern_format() {
        assert!(is_modern_format_ext("avif"));
        assert!(is_modern_format_ext("jxl"));
        assert!(is_modern_format_ext("heic"));
        assert!(!is_modern_format_ext("png"));
        assert!(!is_modern_format_ext("jpg"));
    }
}
