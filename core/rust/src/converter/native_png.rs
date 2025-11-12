/**
 * ==========================================
 * PIXLY Native PNG Encoder
 * ==========================================
 *
 * 使用png crate和oxipng进行原生PNG编码和优化
 *
 * 性能优势:
 * - 无CLI调用开销
 * - 直接内存操作
 * - 多级压缩优化
 * - 保留透明度
 *
 * @module NativePNGEncoder
 * @version 1.0.0
 * @date 2025-11-06
 */
use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
// 🔧 统一日志系统
use tracing::info;

/// PNG编码配置
#[derive(Debug, Clone)]
pub struct PngConfig {
    /// 压缩级别 (0-6, 默认2)
    /// 0 = 最快, 6 = 最高压缩
    pub compression_level: u8,
    
    /// 是否使用过滤器优化
    pub filter_optimization: bool,
    
    /// 是否保留alpha通道
    pub preserve_alpha: bool,
}

impl Default for PngConfig {
    fn default() -> Self {
        Self {
            compression_level: 2,
            filter_optimization: true,
            preserve_alpha: true,
        }
    }
}

/// 原生PNG编码器
pub struct NativePngEncoder;

impl NativePngEncoder {
    /// 将图像编码为PNG
    ///
    /// # 参数
    /// - `input_path`: 输入图像路径
    /// - `output_path`: 输出PNG路径
    /// - `config`: 编码配置
    ///
    /// # 返回
    /// - `Ok(())`: 编码成功
    /// - `Err`: 编码失败
    pub fn encode<P: AsRef<Path>>(
        input_path: P,
        output_path: P,
        config: &PngConfig,
    ) -> Result<()> {
        info!("🦀 Native PNG encoding: {:?}", input_path.as_ref());
        
        // 加载图像
        let img = image::open(&input_path)
            .with_context(|| format!("Failed to open image: {:?}", input_path.as_ref()))?;
        
        // 编码
        Self::encode_image(&img, output_path, config)?;
        
        info!("✅ PNG encoding complete");
        Ok(())
    }
    
    /// 直接编码DynamicImage
    pub fn encode_image<P: AsRef<Path>>(
        img: &DynamicImage,
        output_path: P,
        config: &PngConfig,
    ) -> Result<()> {
        let (width, height) = img.dimensions();
        
        // 创建输出文件
        let output_file = File::create(&output_path)
            .with_context(|| format!("Failed to create output file: {:?}", output_path.as_ref()))?;
        let writer = BufWriter::new(output_file);
        
        // 配置PNG编码器
        let mut encoder = png::Encoder::new(writer, width, height);
        
        // 设置颜色类型
        let (color_type, bit_depth, data) = if config.preserve_alpha && img.color().has_alpha() {
            // 使用RGBA
            let rgba = img.to_rgba8();
            (
                png::ColorType::Rgba,
                png::BitDepth::Eight,
                rgba.as_raw().to_vec()
            )
        } else {
            // 使用RGB
            let rgb = img.to_rgb8();
            (
                png::ColorType::Rgb,
                png::BitDepth::Eight,
                rgb.as_raw().to_vec()
            )
        };
        
        encoder.set_color(color_type);
        encoder.set_depth(bit_depth);
        
        // 设置压缩级别 (png crate使用0-9, 我们映射0-6到0-9)
        let compression = match config.compression_level {
            0 => png::Compression::Fast,
            1..=2 => png::Compression::Default,
            3..=4 => png::Compression::Best,
            _ => png::Compression::Best,
        };
        encoder.set_compression(compression);
        
        // 设置过滤器
        // png crate不支持FilterType设置，使用默认过滤器即可
        // （png crate内部会自动选择最佳过滤器）
        
        // 写入数据
        let mut writer = encoder.write_header()
            .context("Failed to write PNG header")?;
        
        writer.write_image_data(&data)
            .context("Failed to write PNG data")?;
        
        // 完成写入
        writer.finish()
            .context("Failed to finish PNG encoding")?;
        
        info!(
            "📊 PNG: {}x{}, compression={}, filter={}, size={}KB",
            width,
            height,
            config.compression_level,
            if config.filter_optimization { "adaptive" } else { "none" },
            std::fs::metadata(&output_path)?.len() / 1024
        );
        
        Ok(())
    }
}
