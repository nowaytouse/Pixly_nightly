/**
 * Dimension Validator - 尺寸限制验证
 * 
 * 🔥 各格式有不同的尺寸限制，需要预先检查避免panic
 * 
 * @module dimension_validator
 */
// 🔧 统一日志系统
use tracing::{error, debug};

use anyhow::{Result, bail};

/// 格式尺寸限制常量
pub struct DimensionLimits;

impl DimensionLimits {
    /// WebP最大尺寸: 16383x16383
    pub const WEBP_MAX: u32 = 16383;
    
    /// JPEG最大尺寸: 65535x65535
    pub const JPEG_MAX: u32 = 65535;
    
    /// PNG理论无限制，但实践中建议不超过
    pub const PNG_SAFE_MAX: u32 = 100000;
    
    /// AVIF最大尺寸（取决于编码器）
    pub const AVIF_MAX: u32 = 65536;
    
    /// JXL理论支持到1073741823，实际建议
    pub const JXL_SAFE_MAX: u32 = 262144;
    
    /// 检查尺寸是否在格式限制内
    pub fn validate_dimensions(width: u32, height: u32, format: &str) -> Result<()> {
        let format_lower = format.to_lowercase();
        
        let (max_dim, format_name) = match format_lower.as_str() {
            "webp" => (Self::WEBP_MAX, "WebP"),
            "jpg" | "jpeg" => (Self::JPEG_MAX, "JPEG"),
            "png" => (Self::PNG_SAFE_MAX, "PNG"),
            "avif" => (Self::AVIF_MAX, "AVIF"),
            "jxl" => (Self::JXL_SAFE_MAX, "JXL"),
            _ => return Ok(()), // 未知格式跳过检查
        };
        
        if width > max_dim || height > max_dim {
            error!(
                "❌ {} dimension limit exceeded: {}x{} (max: {}x{})",
                format_name, width, height, max_dim, max_dim
            );
            bail!(
                "Image dimensions {}x{} exceed {} limit (max {}x{}). \
                 Consider resizing or using a different format.",
                width, height, format_name, max_dim, max_dim
            );
        }
        
        debug!("✅ Dimensions {}x{} OK for {}", width, height, format_name);
        Ok(())
    }
    
    /// 检查是否建议使用CLI fallback
    pub fn should_use_cli_fallback(width: u32, height: u32, format: &str) -> bool {
        let format_lower = format.to_lowercase();
        
        match format_lower.as_str() {
            "webp" => width > Self::WEBP_MAX || height > Self::WEBP_MAX,
            "avif" => width > Self::AVIF_MAX || height > Self::AVIF_MAX,
            _ => false,
        }
    }
    
    /// 获取格式的最大尺寸
    pub fn get_max_dimension(format: &str) -> Option<u32> {
        let format_lower = format.to_lowercase();
        
        match format_lower.as_str() {
            "webp" => Some(Self::WEBP_MAX),
            "jpg" | "jpeg" => Some(Self::JPEG_MAX),
            "png" => Some(Self::PNG_SAFE_MAX),
            "avif" => Some(Self::AVIF_MAX),
            "jxl" => Some(Self::JXL_SAFE_MAX),
            _ => None,
        }
    }
}
