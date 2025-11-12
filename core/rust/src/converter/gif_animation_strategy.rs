/**
 * Animated GIF Conversion Strategy
 * 
 * 处理动画GIF转换：
 * - GIF → WebP: 使用gif2webp (最佳方案，保留动画)
 * - GIF → AVIF: 使用FFmpeg (AVIF动画支持有限)
 */
use crate::converter::strategy::{ConversionStrategy, ConversionConfig, ConversionResult};
use crate::converter::animation_detector::is_animated_gif;
use anyhow::{Result, Context, bail};
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use std::fs;
// 🔧 统一日志系统
use tracing::{info, warn};


pub struct AnimatedGifStrategy;

impl AnimatedGifStrategy {
    /// 检查CLI工具是否可用
    fn is_tool_available(&self, name: &str) -> bool {
        Command::new(name)
            .arg("--version")
            .output()
            .is_ok()
    }
    
    /// 使用gif2webp转换
    fn convert_gif_to_webp(&self, input: &Path, output: &Path, config: &ConversionConfig) -> Result<()> {
        // 🔥 Phase 40.7.14: 优先使用gif2webp（稳定），fallback到FFmpeg
        // gif2webp更快，FFmpeg的-progress输出不可靠
        if self.is_tool_available("gif2webp") {
            info!("🎬 Converting animated GIF to WebP using gif2webp");
            self.convert_gif_to_webp_legacy(input, output, config)
        } else if self.is_tool_available("ffmpeg") {
            warn!("⚠️  gif2webp not available, using FFmpeg");
            self.convert_gif_to_webp_ffmpeg(input, output, config)
        } else {
            bail!("Neither gif2webp nor ffmpeg found in PATH");
        }
    }
    
    /// 使用FFmpeg转换GIF到WebP（有进度输出）
    fn convert_gif_to_webp_ffmpeg(&self, input: &Path, output: &Path, config: &ConversionConfig) -> Result<()> {
        use std::process::Stdio;
        use std::io::{BufRead, BufReader};
        
        println!(r#"{{"type":"progress","percent":0,"message":"开始分析GIF"}}"#);
        
        // WebP quality mapping: -q 0-100 → -qscale 1-100 (higher is better)
        let qscale = config.quality;
        
        // 🔥 Phase 40.7.13: 使用-progress pipe:2输出实时进度到stderr
        let mut child = Command::new("ffmpeg")
            .arg("-progress").arg("pipe:2")  // 实时进度输出到stderr
            .arg("-i").arg(input)
            .arg("-c:v").arg("libwebp")
            .arg("-qscale").arg(qscale.to_string())
            .arg("-lossless").arg("0") // 有损压缩
            .arg("-preset").arg("default")
            .arg("-loop").arg("0") // 循环播放
            .arg("-y") // 覆盖输出
            .arg(output)
            .stderr(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .context("Failed to spawn ffmpeg")?;
        
        let stderr = child.stderr.take().context("Failed to capture stderr")?;
        let reader = BufReader::new(stderr);
        
        let mut total_frames = 0;
        #[allow(unused_assignments)]  // 🔥 Phase 44: 初始值将在解析中被覆盖
        let mut current_frame = 0;
        let mut last_percent = 0;
        
        for line in reader.lines() {
            let line = line?;
            
            // 解析Duration（总帧数估算）
            if line.contains("Duration:") {
                if let Some(cap) = line.split("Duration: ").nth(1) {
                    if let Some(time) = cap.split(',').next() {
                        // Duration: 00:00:07.00
                        let parts: Vec<&str> = time.split(':').collect();
                        if parts.len() == 3 {
                            let seconds: f32 = parts[2].parse().unwrap_or(1.0);
                            total_frames = (seconds * 10.0) as u32; // 估算帧数
                            eprintln!("📊 Estimated {} frames", total_frames);
                        }
                    }
                }
            }
            
            // 🔥 Phase 40.7.13: 解析-progress输出（key=value格式）
            // frame=5
            // fps=0.9
            // out_time_ms=5000000
            if line.starts_with("frame=") {
                if let Some(frame_str) = line.strip_prefix("frame=") {
                    if let Ok(frame) = frame_str.trim().parse::<u32>() {
                        current_frame = frame;
                        if total_frames > 0 && current_frame > 0 {
                            let percent = ((current_frame as f32 / total_frames as f32) * 100.0).min(99.0) as u32;
                            // 只在百分比变化时输出（避免重复）
                            if percent != last_percent {
                                println!(r#"{{"type":"progress","percent":{},"message":"处理帧 {}/{}"}}"#, percent, current_frame, total_frames);
                                last_percent = percent;
                            }
                        }
                    }
                }
            }
            
            // 传统格式解析（fallback）
            if line.contains("frame=") && !line.starts_with("frame=") {
                if let Some(frame_str) = line.split("frame=").nth(1) {
                    if let Some(frame_num) = frame_str.split_whitespace().next() {
                        if let Ok(frame) = frame_num.parse::<u32>() {
                            current_frame = frame;
                            if total_frames > 0 {
                                let percent = ((current_frame as f32 / total_frames as f32) * 100.0).min(99.0) as u32;
                                if percent != last_percent {
                                    println!(r#"{{"type":"progress","percent":{},"message":"处理帧 {}/{}"}}"#, percent, current_frame, total_frames);
                                    last_percent = percent;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        let status = child.wait()?;
        if !status.success() {
            bail!("FFmpeg failed with exit code: {:?}", status.code());
        }
        
        println!(r#"{{"type":"progress","percent":100,"message":"转换完成"}}"#);
        Ok(())
    }
    
    /// 使用gif2webp转换（无进度，旧方法）
    fn convert_gif_to_webp_legacy(&self, input: &Path, output: &Path, config: &ConversionConfig) -> Result<()> {
        println!(r#"{{"type":"progress","percent":0,"message":"使用gif2webp转换"}}"#);
        
        let status = Command::new("gif2webp")
            .arg("-q").arg(config.quality.to_string())
            .arg("-m").arg("6")
            .arg(input)
            .arg("-o").arg(output)
            .status()
            .context("Failed to execute gif2webp")?;
        
        if !status.success() {
            bail!("gif2webp failed with exit code: {:?}", status.code());
        }
        
        println!(r#"{{"type":"progress","percent":100,"message":"转换完成"}}"#);
        Ok(())
    }
    
    /// 使用FFmpeg转换GIF到AVIF
    fn convert_gif_to_avif(&self, input: &Path, output: &Path, config: &ConversionConfig) -> Result<()> {
        if !self.is_tool_available("ffmpeg") {
            bail!("ffmpeg not found in PATH. Please install it first (e.g., brew install ffmpeg)");
        }
        
        warn!("⚠️  AVIF animation support is experimental");
        info!("🎬 Converting animated GIF to AVIF using FFmpeg");
        
        let quality_crf = 63 - (config.quality * 63 / 100); // 转换quality到CRF (0-63)
        
        let status = Command::new("ffmpeg")
            .arg("-i").arg(input)
            .arg("-c:v").arg("libaom-av1")
            .arg("-crf").arg(quality_crf.to_string())
            .arg("-b:v").arg("0") // Use CRF mode
            .arg("-y") // Overwrite output
            .arg(output)
            .status()
            .context("Failed to execute ffmpeg")?;
        
        if !status.success() {
            bail!("ffmpeg failed with exit code: {:?}", status.code());
        }
        
        Ok(())
    }
}

impl ConversionStrategy for AnimatedGifStrategy {
    fn name(&self) -> &str {
        "Animated GIF (gif2webp/ffmpeg)"
    }
    
    fn is_available(&self) -> bool {
        // 检查是否有gif2webp或ffmpeg
        self.is_tool_available("gif2webp") || self.is_tool_available("ffmpeg")
    }
    
    fn supported_formats(&self) -> Vec<String> {
        vec!["webp".to_string(), "avif".to_string()]
    }
    
    fn convert(
        &self,
        input: &Path,
        output: &Path,
        format: &str,
        config: &ConversionConfig,
    ) -> Result<ConversionResult> {
        let start = Instant::now();
        
        // 检查是否为动画GIF
        if !is_animated_gif(input)? {
            // 不是动画GIF，让其他策略处理
            bail!("Not an animated GIF, let other strategies handle it");
        }
        
        let input_size = fs::metadata(input)?.len();
        
        // 根据目标格式选择转换方法
        match format.to_lowercase().as_str() {
            "webp" => self.convert_gif_to_webp(input, output, config)?,
            "avif" => self.convert_gif_to_avif(input, output, config)?,
            _ => bail!("Animated GIF to {} is not supported by this strategy", format),
        }
        
        let output_size = fs::metadata(output)?.len();
        let duration = start.elapsed().as_millis() as u64;
        
        Ok(ConversionResult {
            success: true,
            output_path: output.to_string_lossy().to_string(),
            input_size,
            output_size,
            compression_ratio: (input_size as f64 / output_size as f64),
            processing_time_ms: duration,
            strategy_used: self.name().to_string(),
            error_message: None,
        })
    }
    
    fn priority(&self) -> u8 {
        90 // 高优先级，优先处理动画GIF
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_strategy_availability() {
        let strategy = AnimatedGifStrategy;
        // 基本测试
        assert!(!strategy.name().is_empty());
        assert!(strategy.supported_formats().contains(&"webp".to_string()));
    }
}
