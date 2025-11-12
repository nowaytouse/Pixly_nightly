/**
 * ==========================================
 * PIXLY Native AVIF Encoder
 * ==========================================
 *
 * 使用rav1e和ravif进行纯Rust AVIF编码
 *
 * 性能优势:
 * - 无CLI调用开销
 * - 直接内存操作
 * - 并行编码
 * - 精密质量控制
 *
 * @module NativeAVIFEncoder
 * @version 1.0.0
 * @date 2025-11-05
 */
use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
// 🔧 统一日志系统
use tracing::info;


#[cfg(feature = "native-avif")]
use ravif::Encoder;
#[cfg(feature = "native-avif")]
use rgb::FromSlice;

/// AVIF编码配置
#[derive(Debug, Clone)]
pub struct AvifConfig {
    /// 质量参数 (0-100, 默认85)
    pub quality: f32,
    
    /// 编码速度 (0-10, 默认4)
    /// 0 = 最慢最高质量, 10 = 最快最低质量
    pub speed: u8,
    
    /// 色度二次采样 (默认4:2:0)
    pub chroma_sampling: ChromaSampling,
    
    /// 是否保留alpha通道
    pub preserve_alpha: bool,
    
    /// 并行线程数 (0 = auto)
    pub threads: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum ChromaSampling {
    /// 4:4:4 - 最高质量
    Yuv444,
    /// 4:2:2 - 平衡
    Yuv422,
    /// 4:2:0 - 最佳压缩 (默认)
    Yuv420,
}

impl Default for AvifConfig {
    fn default() -> Self {
        Self {
            quality: 85.0,
            speed: 4,
            chroma_sampling: ChromaSampling::Yuv420,
            preserve_alpha: true,
            threads: 0, // auto
        }
    }
}

/// 原生AVIF编码器
pub struct NativeAvifEncoder;

impl NativeAvifEncoder {
    /// 将图像编码为AVIF
    ///
    /// # 参数
    /// - `input_path`: 输入图像路径
    /// - `output_path`: 输出AVIF路径
    /// - `config`: 编码配置
    ///
    /// # 返回
    /// - `Ok(())`: 编码成功
    /// - `Err`: 编码失败
    #[cfg(feature = "native-avif")]
    pub fn encode<P: AsRef<Path>>(
        input_path: P,
        output_path: P,
        config: &AvifConfig,
    ) -> Result<()> {
        info!("🦀 Native AVIF encoding: {:?}", input_path.as_ref());
        
        // 加载图像
        let img = image::open(&input_path)
            .with_context(|| format!("Failed to open image: {:?}", input_path.as_ref()))?;
        
        // 编码
        Self::encode_image(&img, output_path, config)?;
        
        info!("✅ AVIF encoding complete");
        Ok(())
    }
    
    /// 直接编码DynamicImage
    #[cfg(feature = "native-avif")]
    pub fn encode_image<P: AsRef<Path>>(
        img: &DynamicImage,
        output_path: P,
        config: &AvifConfig,
    ) -> Result<()> {
        let (width, height) = img.dimensions();
        
        // 转换为RGBA8
        let rgba = img.to_rgba8();
        let rgba_data = rgba.as_raw();
        
        // 创建RGBA8切片
        let rgba_slice = rgba_data.as_rgba();
        
        // 配置编码器
        let num_threads = if config.threads == 0 {
            Some(rayon::current_num_threads())
        } else {
            Some(config.threads)
        };
        
        let encoder = Encoder::new()
            .with_quality(config.quality)
            .with_speed(config.speed)
            .with_num_threads(num_threads)
            .with_alpha_quality(config.quality * 0.9);  // Alpha稍低质量
        
        // 创建RGBA8图像引用
        let img_ref = imgref::Img::new(rgba_slice, width as usize, height as usize);
        
        // 编码
        let encoded = encoder
            .encode_rgba(img_ref)
            .context("AVIF encoding failed")?;
        
        // 写入文件
        let output_file = File::create(&output_path)
            .with_context(|| format!("Failed to create output file: {:?}", output_path.as_ref()))?;
        let mut writer = BufWriter::new(output_file);
        
        std::io::Write::write_all(&mut writer, &encoded.avif_file)
            .context("Failed to write AVIF data")?;
        
        info!(
            "📊 AVIF: {}x{}, quality={}, speed={}, size={}KB",
            width,
            height,
            config.quality,
            config.speed,
            encoded.avif_file.len() / 1024
        );
        
        Ok(())
    }
    
    /// 不支持原生编码时的fallback提示
    #[cfg(not(feature = "native-avif"))]
    pub fn encode<P: AsRef<Path>>(
        _input_path: P,
        _output_path: P,
        _config: &AvifConfig,
    ) -> Result<()> {
        anyhow::bail!(
            "Native AVIF encoding not available. \
             Compile with --features native-avif or use CLI fallback."
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    #[cfg(feature = "native-avif")]
    fn test_avif_encoding() {
        // 创建测试图像
        let img = DynamicImage::new_rgba8(100, 100);
        
        // 临时输出目录
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test.avif");
        
        // 编码
        let config = AvifConfig::default();
        let result = NativeAvifEncoder::encode_image(&img, &output_path, &config);
        
        assert!(result.is_ok());
        assert!(output_path.exists());
    }
}
