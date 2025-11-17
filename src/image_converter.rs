//! 图像转换器核心实现

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use std::fs;
use anyhow::{Result, Context, bail};
use serde::{Deserialize, Serialize};

/// 转换配置 (由batch.rs内部使用)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionConfig {
    pub quality: u8,
    pub speed: u8,
    pub preserve_metadata: bool,
    pub keep_animated: bool,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        Self {
            quality: 75,
            speed: 4,
            preserve_metadata: true,
            keep_animated: true,
        }
    }
}

/// 转换结果 (ImageConverter内部使用)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub success: bool,
    pub input_path: String,
    pub output_path: String,
    pub input_size: u64,
    pub output_size: u64,
    pub compression_ratio: f64,
    pub processing_time_ms: u64,
    pub error_message: Option<String>,
}

/// 图像转换器
pub struct ImageConverter {
    config: ConversionConfig,
}

impl ImageConverter {
    /// 创建新的转换器
    pub fn new(config: ConversionConfig) -> Self {
        Self { config }
    }
    
    /// 转换图像
    pub fn convert<P: AsRef<Path>>(&self, input: P, output: P, format: &str) -> Result<ConversionResult> {
        let start = Instant::now();
        let input_path = input.as_ref();
        let output_path = output.as_ref();
        
        // 验证输入文件
        if !input_path.exists() {
            bail!("Input file does not exist: {:?}", input_path);
        }
        
        let input_size = fs::metadata(input_path)?.len();
        
        // 根据格式调用相应的转换函数
        let result = match format.to_lowercase().as_str() {
            "jpeg" | "jpg" => self.convert_to_jpeg(input_path, output_path),
            "png" => self.convert_to_png(input_path, output_path),
            "webp" => self.convert_to_webp(input_path, output_path),
            "avif" => self.convert_to_avif(input_path, output_path),
            "jxl" => self.convert_to_jxl(input_path, output_path),
            "gif" => self.convert_to_gif(input_path, output_path),
            _ => bail!("Unsupported format: {}", format),
        };
        
        let processing_time = start.elapsed().as_millis() as u64;
        
        match result {
            Ok(_) => {
                let output_size = fs::metadata(output_path)?.len();
                let compression_ratio = if input_size > 0 {
                    (output_size as f64 / input_size as f64) * 100.0
                } else {
                    0.0
                };
                
                Ok(ConversionResult {
                    success: true,
                    input_path: input_path.to_string_lossy().to_string(),
                    output_path: output_path.to_string_lossy().to_string(),
                    input_size,
                    output_size,
                    compression_ratio,
                    processing_time_ms: processing_time,
                    error_message: None,
                })
            }
            Err(e) => {
                Ok(ConversionResult {
                    success: false,
                    input_path: input_path.to_string_lossy().to_string(),
                    output_path: output_path.to_string_lossy().to_string(),
                    input_size,
                    output_size: 0,
                    compression_ratio: 0.0,
                    processing_time_ms: processing_time,
                    error_message: Some(e.to_string()),
                })
            }
        }
    }
    
    /// 转换为 JPEG
    fn convert_to_jpeg(&self, input: &Path, output: &Path) -> Result<()> {
        // 使用 image crate 进行转换
        let img = image::open(input)
            .context("无法打开图像")?;
        
        // 转换为 RGB
        let rgb_img = img.to_rgb8();
        
        // 保存为 JPEG
        rgb_img.save_with_format(output, image::ImageFormat::Jpeg)
            .context("Failed to save JPEG")?;
        
        Ok(())
    }
    
    /// 转换为 PNG
    fn convert_to_png(&self, input: &Path, output: &Path) -> Result<()> {
        let img = image::open(input)
            .context("无法打开图像")?;
        
        img.save_with_format(output, image::ImageFormat::Png)
            .context("Failed to save PNG")?;
        
        Ok(())
    }
    
    /// 转换为 WebP（调用 cwebp）
    fn convert_to_webp(&self, input: &Path, output: &Path) -> Result<()> {
        // 查找 cwebp
        let cwebp = self.find_tool("cwebp")?;
        
        let mut cmd = Command::new(cwebp);
        cmd.arg("-q").arg(self.config.quality.to_string())
           .arg(input)
           .arg("-o").arg(output);
        
        if self.config.preserve_metadata {
            cmd.arg("-metadata").arg("all");
        }
        
        let output_result = cmd.output()
            .context("Failed to execute cwebp")?;
        
        if !output_result.status.success() {
            bail!("cwebp failed: {}", String::from_utf8_lossy(&output_result.stderr));
        }
        
        Ok(())
    }
    
    /// 转换为 AVIF（调用 avifenc）
    fn convert_to_avif(&self, input: &Path, output: &Path) -> Result<()> {
        let avifenc = self.find_tool("avifenc")?;
        
        let mut cmd = Command::new(avifenc);
        cmd.arg(input)
           .arg(output)
           .arg("-q").arg(self.config.quality.to_string())
           .arg("-s").arg(self.config.speed.to_string());
        
        let output_result = cmd.output()
            .context("Failed to execute avifenc")?;
        
        if !output_result.status.success() {
            bail!("avifenc failed: {}", String::from_utf8_lossy(&output_result.stderr));
        }
        
        Ok(())
    }
    
    /// 转换为 JXL（调用 cjxl）
    fn convert_to_jxl(&self, input: &Path, output: &Path) -> Result<()> {
        let cjxl = self.find_tool("cjxl")?;
        
        let mut cmd = Command::new(cjxl);
        cmd.arg(input)
           .arg(output)
           .arg("-q").arg(self.config.quality.to_string())
           .arg("-e").arg(self.config.speed.to_string());
        
        let output_result = cmd.output()
            .context("Failed to execute cjxl")?;
        
        if !output_result.status.success() {
            bail!("cjxl failed: {}", String::from_utf8_lossy(&output_result.stderr));
        }
        
        Ok(())
    }
    
    /// 转换为 GIF（使用 FFmpeg 优化）
    fn convert_to_gif(&self, input: &Path, output: &Path) -> Result<()> {
        let ffmpeg = self.find_tool("ffmpeg")?;
        
        // 使用 FFmpeg 优化 GIF
        // -i input: 输入文件
        // -vf "split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse": 生成优化的调色板
        // -loop 0: 无限循环
        let mut cmd = Command::new(ffmpeg);
        cmd.arg("-i").arg(input)
           .arg("-vf").arg("split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse")
           .arg("-loop").arg("0")
           .arg("-y") // 覆盖输出文件
           .arg(output);
        
        let output_result = cmd.output()
            .context("Failed to execute ffmpeg")?;
        
        if !output_result.status.success() {
            bail!("ffmpeg failed: {}", String::from_utf8_lossy(&output_result.stderr));
        }
        
        Ok(())
    }
    
    /// 查找工具路径
    fn find_tool(&self, tool_name: &str) -> Result<PathBuf> {
        // 常见路径
        let paths = vec![
            format!("/opt/homebrew/bin/{}", tool_name),
            format!("/usr/local/bin/{}", tool_name),
            format!("/opt/local/bin/{}", tool_name),
            format!("/usr/bin/{}", tool_name),
        ];
        
        for path in paths {
            if Path::new(&path).exists() {
                return Ok(PathBuf::from(path));
            }
        }
        
        // 尝试从 PATH 中查找
        if let Ok(output) = Command::new("which").arg(tool_name).output() {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path_str.is_empty() {
                    return Ok(PathBuf::from(path_str));
                }
            }
        }
        
        bail!("Tool not found: {}", tool_name);
    }
}
