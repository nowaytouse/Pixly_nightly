/**
 * ==========================================
 * PIXLY Native WebP Encoder
 * ==========================================
 *
 * 使用webp crate进行纯Rust WebP编码
 *
 * 性能优势:
 * - 无CLI调用开销
 * - 直接内存操作
 * - 有损/无损模式
 * - 精密质量控制
 *
 * @module NativeWebPEncoder
 * @version 1.0.0
 * @date 2025-11-05
 */
use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView};
use std::fs::File;
use std::io::Write;
use std::path::Path;
// 🔧 统一日志系统
use tracing::{info, error};

#[cfg(feature = "native-webp")]
use webp::{Encoder, WebPMemory};

/// WebP编码配置
#[derive(Debug, Clone)]
pub struct WebPConfig {
    /// 质量参数 (0-100, 默认85)
    pub quality: f32,
    
    /// 是否无损模式
    pub lossless: bool,
    
    /// 是否保留alpha通道
    pub preserve_alpha: bool,
}

impl Default for WebPConfig {
    fn default() -> Self {
        Self {
            quality: 85.0,
            lossless: false,
            preserve_alpha: true,
        }
    }
}

/// 原生WebP编码器
pub struct NativeWebPEncoder;

impl NativeWebPEncoder {
    /// 将图像编码为WebP
    ///
    /// # 参数
    /// - `input_path`: 输入图像路径
    /// - `output_path`: 输出WebP路径
    /// - `config`: 编码配置
    ///
    /// # 返回
    /// - `Ok(())`: 编码成功
    /// - `Err`: 编码失败
    #[cfg(feature = "native-webp")]
    pub fn encode<P: AsRef<Path>>(
        input_path: P,
        output_path: P,
        config: &WebPConfig,
    ) -> Result<()> {
        info!("🦀 Native WebP encoding: {:?}", input_path.as_ref());
        
        // 加载图像
        let img = image::open(&input_path)
            .with_context(|| format!("Failed to open image: {:?}", input_path.as_ref()))?;
        
        // 编码
        Self::encode_image(&img, output_path, config)?;
        
        info!("✅ WebP encoding complete");
        Ok(())
    }
    
    /// 直接编码DynamicImage
    #[cfg(feature = "native-webp")]
    pub fn encode_image<P: AsRef<Path>>(
        img: &DynamicImage,
        output_path: P,
        config: &WebPConfig,
    ) -> Result<WebPMemory> {
        let (width, height) = img.dimensions();
        
        // 🔥 WebP尺寸限制检查 (16383x16383)
        const WEBP_MAX_DIMENSION: u32 = 16383;
        if width > WEBP_MAX_DIMENSION || height > WEBP_MAX_DIMENSION {
            error!("❌ WebP dimension limit exceeded: {}x{} (max: {}x{})",
                width, height, WEBP_MAX_DIMENSION, WEBP_MAX_DIMENSION);
            anyhow::bail!(
                "Image dimensions {}x{} exceed WebP limit (max {}x{}). \
                 Consider using CLI fallback or resizing the image.",
                width, height, WEBP_MAX_DIMENSION, WEBP_MAX_DIMENSION
            );
        }
        
        // 编码（根据alpha处理）
        // 🔥 捕获编码错误而不是panic
        let webp_data = if config.preserve_alpha {
            // 使用RGBA
            let rgba = img.to_rgba8();
            let encoder = Encoder::from_rgba(rgba.as_raw(), width, height);
            if config.lossless {
                encoder.encode_lossless()
            } else {
                encoder.encode(config.quality)
            }
        } else {
            // 使用RGB
            let rgb = img.to_rgb8();
            let encoder = Encoder::from_rgb(rgb.as_raw(), width, height);
            if config.lossless {
                encoder.encode_lossless()
            } else {
                encoder.encode(config.quality)
            }
        };
        
        // 🔥 检查编码是否成功（WebPMemory可能包含错误状态）
        if webp_data.is_empty() {
            error!("❌ WebP encoding failed: encoder returned empty data");
            anyhow::bail!("WebP encoding failed");
        }
        
        // 写入文件
        let mut output_file = File::create(&output_path)
            .with_context(|| format!("Failed to create output file: {:?}", output_path.as_ref()))?;
        
        output_file.write_all(&webp_data)
            .context("Failed to write WebP data")?;
        
        info!(
            "📊 WebP: {}x{}, quality={}, lossless={}, size={}KB",
            width,
            height,
            config.quality,
            config.lossless,
            webp_data.len() / 1024
        );
        
        Ok(webp_data)
    }
    
    /// 不支持原生编码时的fallback提示
    #[cfg(not(feature = "native-webp"))]
    pub fn encode<P: AsRef<Path>>(
        _input_path: P,
        _output_path: P,
        _config: &WebPConfig,
    ) -> Result<()> {
        anyhow::bail!(
            "Native WebP encoding not available. \
             Compile with --features native-webp or use CLI fallback."
        )
    }
}
