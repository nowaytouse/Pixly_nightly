/**
 * 🔄 整合文件：Native Format Strategies
 * 
 * 整合前的4个文件：
 * - native_jpeg_strategy.rs (68行)
 * - native_png_strategy.rs (71行)  
 * - native_webp_strategy.rs (75行)
 * - native_avif_strategy.rs (77行)
 * 
 * 🔥 从@deprecated提取价值：dimension_validator.rs
 * - 各格式尺寸限制验证
 * - 避免编码器panic
 * 
 * 总计：291行 → 整合后约200行（减少31%，增强功能）
 */

use crate::converter::strategy::{ConversionStrategy, ConversionConfig, ConversionResult};
use crate::converter::{
    native_jpeg::{NativeJpegEncoder, JpegConfig},
    native_png::{NativePngEncoder, PngConfig},
    native_webp::{NativeWebPEncoder, WebPConfig},
    native_avif::{NativeAvifEncoder, AvifConfig, ChromaSampling},
};
use anyhow::{Result, bail};
use std::path::Path;
use std::time::Instant;
use tracing::{error, debug};

// ========================================
// 尺寸限制验证（从@deprecated提取）
// ========================================
struct DimensionLimits;

impl DimensionLimits {
    const WEBP_MAX: u32 = 16383;
    const JPEG_MAX: u32 = 65535;
    const PNG_SAFE_MAX: u32 = 100000;
    const AVIF_MAX: u32 = 65536;
    
    fn validate_dimensions(width: u32, height: u32, format: &str) -> Result<()> {
        let (max_dim, format_name) = match format.to_lowercase().as_str() {
            "webp" => (Self::WEBP_MAX, "WebP"),
            "jpg" | "jpeg" => (Self::JPEG_MAX, "JPEG"),
            "png" => (Self::PNG_SAFE_MAX, "PNG"),
            "avif" => (Self::AVIF_MAX, "AVIF"),
            _ => return Ok(()),
        };
        
        if width > max_dim || height > max_dim {
            error!("❌ {} dimension limit exceeded: {}x{} (max: {}x{})", format_name, width, height, max_dim, max_dim);
            bail!("Image dimensions {}x{} exceed {} limit (max {}x{})", width, height, format_name, max_dim, max_dim);
        }
        
        debug!("✅ Dimensions {}x{} OK for {}", width, height, format_name);
        Ok(())
    }
}

// ========================================
// JPEG Strategy
// ========================================
pub struct NativeJpegStrategy;

impl ConversionStrategy for NativeJpegStrategy {
    fn name(&self) -> &str {
        "Native JPEG (image)"
    }
    
    fn is_available(&self) -> bool {
        true // JPEG编码器始终可用（内置）
    }
    
    fn convert(&self, input: &Path, output: &Path, config: &ConversionConfig) -> Result<ConversionResult> {
        let start_time = Instant::now();
        
        // 🔥 实际使用尺寸验证功能
        let img = image::open(input)?;
        let (width, height) = img.dimensions();
        DimensionLimits::validate_dimensions(width, height, "jpeg")?;
        
        // 🔥 架构修正：到达策略层时参数必须已确定 - 不应有Option处理
        // 🔥 如果到这里还是Option，说明上层AI调用架构有问题
        let jpeg_config = JpegConfig {
            quality: config.quality,       // 直接使用 - 上层必须确保AI提供
            progressive: false,            // JPEG不支持progressive，固定false
        };
        
        NativeJpegEncoder::encode(input, output, &jpeg_config)?;
        
        Ok(ConversionResult {
            success: true,
            output_path: output.to_string_lossy().to_string(),
            input_size: std::fs::metadata(input)?.len(),
            output_size: std::fs::metadata(output)?.len(),
            compression_ratio: 0.0,
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            strategy_used: "Native JPEG".to_string(),
            error_message: None,
        })
    }
}

// ========================================
// PNG Strategy  
// ========================================
pub struct NativePngStrategy;

impl ConversionStrategy for NativePngStrategy {
    fn name(&self) -> &str {
        "Native PNG (png)"
    }
    
    fn is_available(&self) -> bool {
        true // PNG编码器始终可用（内置）
    }
    
    fn convert(&self, input: &Path, output: &Path, config: &ConversionConfig) -> Result<ConversionResult> {
        let start_time = Instant::now();
        
        // 🔥 实际使用尺寸验证功能
        let img = image::open(input)?;
        let (width, height) = img.dimensions();
        DimensionLimits::validate_dimensions(width, height, "png")?;
        
        let png_config = PngConfig {
            compression_level: config.compression_level.unwrap_or(6),
            filter_type: None,
        };
        
        NativePngEncoder::encode(input, output, &png_config)?;
        
        Ok(ConversionResult {
            success: true,
            output_path: output.to_string_lossy().to_string(),
            input_size: std::fs::metadata(input)?.len(),
            output_size: std::fs::metadata(output)?.len(),
            compression_ratio: 0.0,
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            strategy_used: "Native PNG".to_string(),
            error_message: None,
        })
    }
}

// ========================================
// WebP Strategy
// ========================================
pub struct NativeWebpStrategy;

impl ConversionStrategy for NativeWebpStrategy {
    fn name(&self) -> &str {
        "Native WebP (webp)"
    }
    
    fn is_available(&self) -> bool {
        true // WebP编码器始终可用（内置）
    }
    
    fn convert(&self, input: &Path, output: &Path, config: &ConversionConfig) -> Result<ConversionResult> {
        let start_time = Instant::now();
        
        // 🔥 实际使用尺寸验证功能
        let img = image::open(input)?;
        let (width, height) = img.dimensions();
        DimensionLimits::validate_dimensions(width, height, "webp")?;
        
        // 🔥 架构修正：到达策略层时参数必须已确定 - 不应有Option处理
        let webp_config = WebPConfig {
            quality: config.quality,       // 直接使用 - 上层必须确保AI提供
            lossless: config.lossless,     // 直接使用 - AI智能预测
            method: 4,                     // WebP压缩方法，固定值
        };
        
        NativeWebPEncoder::encode(input, output, &webp_config)?;
        
        Ok(ConversionResult {
            success: true,
            output_path: output.to_string_lossy().to_string(),
            input_size: std::fs::metadata(input)?.len(),
            output_size: std::fs::metadata(output)?.len(),
            compression_ratio: 0.0,
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            strategy_used: "Native WebP".to_string(),
            error_message: None,
        })
    }
}

// ========================================
// AVIF Strategy
// ========================================
pub struct NativeAvifStrategy;

impl ConversionStrategy for NativeAvifStrategy {
    fn name(&self) -> &str {
        "Native AVIF (ravif)"
    }
    
    fn is_available(&self) -> bool {
        true // AVIF编码器始终可用（内置）
    }
    
    fn convert(&self, input: &Path, output: &Path, config: &ConversionConfig) -> Result<ConversionResult> {
        let start_time = Instant::now();
        
        // 🔥 实际使用尺寸验证功能
        let img = image::open(input)?;
        let (width, height) = img.dimensions();
        DimensionLimits::validate_dimensions(width, height, "avif")?;
        
        // 🔥 架构修正：到达策略层时参数必须已确定 - 不应有Option处理
        let avif_config = AvifConfig {
            quality: config.quality as f32, // u8 -> f32转换
            speed: config.speed,           // 直接使用 - AI智能预测
            chroma_sampling: ChromaSampling::Yuv420, // 使用默认值
            preserve_alpha: true,          // 保留alpha通道
            threads: 0,                    // 自动线程数
        };
        
        NativeAvifEncoder::encode(input, output, &avif_config)?;
        
        Ok(ConversionResult {
            success: true,
            output_path: output.to_string_lossy().to_string(),
            input_size: std::fs::metadata(input)?.len(),
            output_size: std::fs::metadata(output)?.len(),
            compression_ratio: 0.0,
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            strategy_used: "Native AVIF".to_string(),
            error_message: None,
        })
    }
}
