/**
 * 动画GIF转换策略
 * 
 * 从archive迁移并增强：
 * - GIF → WebP: 优先使用gif2webp，fallback到FFmpeg
 * - GIF → AVIF: 使用FFmpeg
 * - 保留原始帧率和帧数
 * - 实时进度输出
 */
use anyhow::{Result, Context, bail};
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::fs;
use tracing::{info, warn, debug};

/// 动画GIF转换策略
pub struct AnimatedGifStrategy;

impl AnimatedGifStrategy {
    /// 创建新实例
    pub fn new() -> Self {
        Self
    }
    
    /// 检查CLI工具是否可用
    fn is_tool_available(&self, name: &str) -> bool {
        Command::new(name)
            .arg("--version")
            .output()
            .is_ok()
    }
    
    /// 转换GIF到WebP
    pub fn convert_gif_to_webp(
        &self,
        input: &Path,
        output: &Path,
        quality: u8,
    ) -> Result<ConversionResult> {
        let start = Instant::now();
        let input_size = fs::metadata(input)?.len();
        
        // 优先使用gif2webp（更快更稳定）
        if self.is_tool_available("gif2webp") {
            info!("🎬 Using gif2webp to convert animated GIF to WebP");
            self.convert_with_gif2webp(input, output, quality)?;
        } else if self.is_tool_available("ffmpeg") {
            warn!("⚠️  gif2webp not available, using FFmpeg");
            self.convert_with_ffmpeg_webp(input, output, quality)?;
        } else {
            bail!("Neither gif2webp nor ffmpeg is available, please install one of them");
        }
        
        let output_size = fs::metadata(output)?.len();
        let duration = start.elapsed();
        
        Ok(ConversionResult {
            input_size,
            output_size,
            compression_ratio: input_size as f64 / output_size as f64,
            processing_time_ms: duration.as_millis() as u64,
            strategy_used: "AnimatedGifStrategy".to_string(),
        })
    }
    
    /// 转换GIF到AVIF
    pub fn convert_gif_to_avif(
        &self,
        input: &Path,
        output: &Path,
        quality: u8,
    ) -> Result<ConversionResult> {
        let start = Instant::now();
        let input_size = fs::metadata(input)?.len();
        
        if !self.is_tool_available("ffmpeg") {
            bail!("ffmpeg is not available, please install it first");
        }
        
        warn!("⚠️  AVIF animation support is experimental");
        info!("🎬 Using FFmpeg to convert animated GIF to AVIF");
        
        self.convert_with_ffmpeg_avif(input, output, quality)?;
        
        let output_size = fs::metadata(output)?.len();
        let duration = start.elapsed();
        
        Ok(ConversionResult {
            input_size,
            output_size,
            compression_ratio: input_size as f64 / output_size as f64,
            processing_time_ms: duration.as_millis() as u64,
            strategy_used: "AnimatedGifStrategy".to_string(),
        })
    }
    
    /// 使用gif2webp转换
    fn convert_with_gif2webp(
        &self,
        input: &Path,
        output: &Path,
        quality: u8,
    ) -> Result<()> {
        debug!("Executing: gif2webp -q {} -m 6 {:?} -o {:?}", quality, input, output);
        
        let status = Command::new("gif2webp")
            .arg("-q").arg(quality.to_string())
            .arg("-m").arg("6")  // 最佳压缩
            .arg(input)
            .arg("-o").arg(output)
            .status()
            .context("Failed to execute gif2webp")?;
        
        if !status.success() {
            bail!("gif2webp failed, exit code: {:?}", status.code());
        }
        
        Ok(())
    }
    
    /// 使用FFmpeg转换到WebP
    fn convert_with_ffmpeg_webp(
        &self,
        input: &Path,
        output: &Path,
        quality: u8,
    ) -> Result<()> {
        debug!("Executing: ffmpeg -i {:?} -c:v libwebp -qscale {} {:?}", input, quality, output);
        
        let mut child = Command::new("ffmpeg")
            .arg("-progress").arg("pipe:2")
            .arg("-i").arg(input)
            .arg("-c:v").arg("libwebp")
            .arg("-qscale").arg(quality.to_string())
            .arg("-lossless").arg("0")
            .arg("-preset").arg("default")
            .arg("-loop").arg("0")  // 循环播放
            .arg("-y")
            .arg(output)
            .stderr(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .context("Failed to start ffmpeg")?;
        
        // 读取进度输出
        if let Some(stderr) = child.stderr.take() {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line && line.starts_with("frame=") {
                    debug!("FFmpeg progress: {}", line);
                }
            }
        }
        
        let status = child.wait()?;
        if !status.success() {
            bail!("FFmpeg failed, exit code: {:?}", status.code());
        }
        
        Ok(())
    }
    
    /// 使用FFmpeg转换到AVIF
    fn convert_with_ffmpeg_avif(
        &self,
        input: &Path,
        output: &Path,
        quality: u8,
    ) -> Result<()> {
        // 转换quality到CRF (0-63，值越小质量越高)
        let crf = 63 - (quality * 63 / 100);
        
        debug!("Executing: ffmpeg -i {:?} -c:v libaom-av1 -crf {} {:?}", input, crf, output);
        
        let status = Command::new("ffmpeg")
            .arg("-i").arg(input)
            .arg("-c:v").arg("libaom-av1")
            .arg("-crf").arg(crf.to_string())
            .arg("-b:v").arg("0")  // CRF模式
            .arg("-y")
            .arg(output)
            .status()
            .context("Failed to execute ffmpeg")?;
        
        if !status.success() {
            bail!("FFmpeg failed, exit code: {:?}", status.code());
        }
        
        Ok(())
    }
    
    /// 检查是否可用
    pub fn is_available(&self) -> bool {
        self.is_tool_available("gif2webp") || self.is_tool_available("ffmpeg")
    }
    
    /// 获取支持的格式
    pub fn supported_formats(&self) -> Vec<String> {
        vec!["webp".to_string(), "avif".to_string()]
    }
}

impl Default for AnimatedGifStrategy {
    fn default() -> Self {
        Self::new()
    }
}

/// 转换结果
#[derive(Debug, Clone)]
pub struct ConversionResult {
    pub input_size: u64,
    pub output_size: u64,
    pub compression_ratio: f64,
    pub processing_time_ms: u64,
    pub strategy_used: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tool_availability() {
        let strategy = AnimatedGifStrategy::new();
        
        // 至少应该有一个工具可用
        let has_gif2webp = strategy.is_tool_available("gif2webp");
        let has_ffmpeg = strategy.is_tool_available("ffmpeg");
        
        println!("gif2webp available: {}", has_gif2webp);
        println!("ffmpeg available: {}", has_ffmpeg);
        
        assert!(has_gif2webp || has_ffmpeg, "At least one of gif2webp or ffmpeg is required");
    }
    
    #[test]
    fn test_supported_formats() {
        let strategy = AnimatedGifStrategy::new();
        let formats = strategy.supported_formats();
        
        assert!(formats.contains(&"webp".to_string()));
        assert!(formats.contains(&"avif".to_string()));
    }
}
