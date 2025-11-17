//! 格式支持模块 - 从 @archive/rust_v2_clean 提取并增强
//! 
//! 定义支持的图像格式和相关配置
//! 
//! 增强点：
//! - 添加更多现代格式支持 (AVIF, JXL)
//! - 改进格式检测和验证
//! - 统一格式配置接口

use serde::{Deserialize, Serialize};
use std::fmt;

/// 支持的图像格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupportedFormat {
    Jpeg,
    Png,
    WebP,
    Avif,
    Jxl,
    Gif,
}

/// 格式配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatConfig {
    pub format: SupportedFormat,
    pub quality: Option<u8>,
    pub compression: Option<u8>,
    pub lossless: bool,
}

/// 通用格式信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatInfo {
    pub name: String,
    pub extensions: Vec<String>,
    pub mime_type: String,
    pub supported: bool,
    pub lossless_capable: bool,
    pub animation_capable: bool,
    pub alpha_capable: bool,
}

impl SupportedFormat {
    /// 获取所有支持的格式
    pub fn all() -> Vec<Self> {
        vec![
            Self::Jpeg,
            Self::Png,
            Self::WebP,
            Self::Avif,
            Self::Jxl,
            Self::Gif,
        ]
    }

    /// 从字符串解析格式
    pub fn parse_format(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "png" => Some(Self::Png),
            "webp" => Some(Self::WebP),
            "avif" => Some(Self::Avif),
            "jxl" | "jpegxl" => Some(Self::Jxl),
            "gif" => Some(Self::Gif),
            _ => None,
        }
    }

    /// 获取文件扩展名
    pub fn extension(self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::WebP => "webp",
            Self::Avif => "avif",
            Self::Jxl => "jxl",
            Self::Gif => "gif",
        }
    }

    /// 获取MIME类型
    pub fn mime_type(self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::WebP => "image/webp",
            Self::Avif => "image/avif",
            Self::Jxl => "image/jxl",
            Self::Gif => "image/gif",
        }
    }

    /// 是否支持无损压缩
    pub fn supports_lossless(self) -> bool {
        matches!(self, Self::Png | Self::WebP | Self::Avif | Self::Jxl)
    }

    /// 是否支持动画
    pub fn supports_animation(self) -> bool {
        matches!(self, Self::Gif | Self::WebP | Self::Avif | Self::Jxl)
    }

    /// 是否支持透明通道
    pub fn supports_alpha(self) -> bool {
        matches!(self, Self::Png | Self::WebP | Self::Avif | Self::Jxl | Self::Gif)
    }

    /// 获取格式详细信息
    pub fn info(self) -> FormatInfo {
        FormatInfo {
            name: self.to_string(),
            extensions: vec![self.extension().to_string()],
            mime_type: self.mime_type().to_string(),
            supported: true,
            lossless_capable: self.supports_lossless(),
            animation_capable: self.supports_animation(),
            alpha_capable: self.supports_alpha(),
        }
    }
}

impl fmt::Display for SupportedFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Jpeg => write!(f, "JPEG"),
            Self::Png => write!(f, "PNG"),
            Self::WebP => write!(f, "WebP"),
            Self::Avif => write!(f, "AVIF"),
            Self::Jxl => write!(f, "JPEG XL"),
            Self::Gif => write!(f, "GIF"),
        }
    }
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            format: SupportedFormat::Jpeg,
            quality: Some(85),
            compression: None,
            lossless: false,
        }
    }
}

/// 获取所有支持的格式信息
pub fn get_supported_formats() -> Vec<FormatInfo> {
    SupportedFormat::all()
        .into_iter()
        .map(|format| format.info())
        .collect()
}

/// 从文件扩展名检测格式
pub fn detect_format_from_extension(path: &str) -> Option<SupportedFormat> {
    let ext = std::path::Path::new(path)
        .extension()?
        .to_str()?
        .to_lowercase();
    SupportedFormat::parse_format(&ext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_parsing() {
        assert_eq!(SupportedFormat::parse_format("jpg"), Some(SupportedFormat::Jpeg));
        assert_eq!(SupportedFormat::parse_format("jpeg"), Some(SupportedFormat::Jpeg));
        assert_eq!(SupportedFormat::parse_format("png"), Some(SupportedFormat::Png));
        assert_eq!(SupportedFormat::parse_format("webp"), Some(SupportedFormat::WebP));
        assert_eq!(SupportedFormat::parse_format("avif"), Some(SupportedFormat::Avif));
        assert_eq!(SupportedFormat::parse_format("jxl"), Some(SupportedFormat::Jxl));
        assert_eq!(SupportedFormat::parse_format("unknown"), None);
    }

    #[test]
    fn test_format_properties() {
        let jpeg = SupportedFormat::Jpeg;
        assert_eq!(jpeg.extension(), "jpg");
        assert_eq!(jpeg.mime_type(), "image/jpeg");
        assert_eq!(jpeg.to_string(), "JPEG");
        assert!(!jpeg.supports_lossless());
        assert!(!jpeg.supports_animation());
        assert!(!jpeg.supports_alpha());

        let webp = SupportedFormat::WebP;
        assert!(webp.supports_lossless());
        assert!(webp.supports_animation());
        assert!(webp.supports_alpha());
    }

    #[test]
    fn test_supported_formats_list() {
        let formats = get_supported_formats();
        assert_eq!(formats.len(), 6);
        assert!(formats.iter().all(|f| f.supported));
    }

    #[test]
    fn test_detect_format_from_extension() {
        assert_eq!(
            detect_format_from_extension("test.jpg"),
            Some(SupportedFormat::Jpeg)
        );
        assert_eq!(
            detect_format_from_extension("test.PNG"),
            Some(SupportedFormat::Png)
        );
        assert_eq!(
            detect_format_from_extension("/path/to/image.webp"),
            Some(SupportedFormat::WebP)
        );
        assert_eq!(detect_format_from_extension("test.unknown"), None);
    }
}
