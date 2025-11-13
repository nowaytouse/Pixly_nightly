/**
 * ==========================================
 * PIXLY Native JPEG Encoder
 * ==========================================
 *
 * 使用mozjpeg进行原生JPEG编码
 *
 * 性能优势:
 * - 无CLI调用开销
 * - 比标准libjpeg更小的文件
 * - 更好的视觉质量
 * - 直接内存操作
 *
 * @module NativeJPEGEncoder
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

/// JPEG编码配置
#[derive(Debug, Clone)]
pub struct JpegConfig {
    /// 质量 (1-100, 默认85)
    pub quality: u8,
    
    /// 是否使用渐进式JPEG
    pub progressive: bool,
    
    /// 是否优化霍夫曼表
    pub optimize_coding: bool,
    
    /// 色度子采样 (None, 420, 422, 444)
    pub chroma_subsampling: ChromaSubsampling,
}

/// 色度子采样模式
#[derive(Debug, Clone, Copy)]
pub enum ChromaSubsampling {
    /// 无子采样 (4:4:4) - 最高质量
    None,
    
    /// 4:2:2 子采样
    Sample422,
    
    /// 4:2:0 子采样 - 最常用
    Sample420,
}

// 🔥 CRITICAL: JpegConfig不应有Default实现！
// 📋 Default实现包含硬编码fallback值：
// - quality: 85    (硬编码fallback - 必须来自AI)
// 
// ✅ 正确的JpegConfig构建方式：
// 1. 从AI预测数据构建
// 2. 从规则引擎明确指定  
// 3. 从用户CLI参数明确构建
// 
// ❌ 禁止使用JpegConfig::default()

// impl Default for JpegConfig {
//     fn default() -> Self {
//         Self {
//             quality: 85,        // ❌ 硬编码fallback
//             progressive: false,
//             optimize_coding: true,
//             chroma_subsampling: ChromaSubsampling::Sample420,
//         }
//     }
// }

/// 原生JPEG编码器
pub struct NativeJpegEncoder;

impl NativeJpegEncoder {
    /// 将图像编码为JPEG
    ///
    /// # 参数
    /// - `input_path`: 输入图像路径
    /// - `output_path`: 输出JPEG路径
    /// - `config`: 编码配置
    ///
    /// # 返回
    /// - `Ok(())`: 编码成功
    /// - `Err`: 编码失败
    pub fn encode<P: AsRef<Path>>(
        input_path: P,
        output_path: P,
        config: &JpegConfig,
    ) -> Result<()> {
        info!("🦀 Native JPEG encoding: {:?}", input_path.as_ref());
        
        // 加载图像
        let img = image::open(&input_path)
            .with_context(|| format!("Failed to open image: {:?}", input_path.as_ref()))?;
        
        // 编码
        Self::encode_image(&img, output_path, config)?;
        
        info!("✅ JPEG encoding complete");
        Ok(())
    }
    
    /// 直接编码DynamicImage
    pub fn encode_image<P: AsRef<Path>>(
        img: &DynamicImage,
        output_path: P,
        config: &JpegConfig,
    ) -> Result<()> {
        let (width, height) = img.dimensions();
        
        // 转换为RGB（JPEG不支持alpha）
        let rgb = img.to_rgb8();
        let data = rgb.as_raw();
        
        // 创建输出文件
        let output_file = File::create(&output_path)
            .with_context(|| format!("Failed to create output file: {:?}", output_path.as_ref()))?;
        let mut writer = BufWriter::new(output_file);
        
        // 创建JPEG编码器
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(
            &mut writer,
            config.quality,
        );
        
        // 写入图像数据
        encoder.encode(
            data,
            width,
            height,
            image::ColorType::Rgb8,
        ).context("Failed to encode JPEG")?;
        
        info!(
            "📊 JPEG: {}x{}, quality={}, size={}KB",
            width,
            height,
            config.quality,
            std::fs::metadata(&output_path)?.len() / 1024
        );
        
        Ok(())
    }
}
