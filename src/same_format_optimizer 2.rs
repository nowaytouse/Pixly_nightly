//! 同格式优化策略
//! 
//! 在不改变格式的情况下优化图像：
//! - PNG: oxipng/optipng (无损)
//! - JPEG: mozjpeg/jpegtran (可配置质量)
//! - WebP: cwebp重压缩

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use std::fs;

/// 同格式优化器
pub struct SameFormatOptimizer {
    preserve_metadata: bool,
}

impl SameFormatOptimizer {
    pub fn new(preserve_metadata: bool) -> Self {
        Self { preserve_metadata }
    }
    
    /// 检查特定格式的工具是否可用
    pub fn check_tools_for_format(format: &str) -> Result<String> {
        match format {
            "png" => {
                if Command::new("oxipng").arg("--version").output().is_ok() {
                    Ok("oxipng".to_string())
                } else if Command::new("optipng").arg("-v").output().is_ok() {
                    Ok("optipng".to_string())
                } else {
                    anyhow::bail!("PNG optimizer not found (requires oxipng or optipng)")
                }
            }
            "jpg" | "jpeg" => {
                if Command::new("mozjpeg").arg("--version").output().is_ok() {
                    Ok("mozjpeg".to_string())
                } else if Command::new("jpegtran").arg("-version").output().is_ok() {
                    Ok("jpegtran".to_string())
                } else {
                    anyhow::bail!("JPEG optimizer not found (requires mozjpeg or jpegtran)")
                }
            }
            "webp" => {
                if Command::new("cwebp").arg("-version").output().is_ok() {
                    Ok("cwebp".to_string())
                } else {
                    anyhow::bail!("WebP optimizer not found (requires cwebp)")
                }
            }
            _ => anyhow::bail!("Unsupported same-format optimization format: {}", format),
        }
    }
    
    /// 优化PNG
    pub fn optimize_png(&self, input: &Path, output: &Path, tool: &str) -> Result<()> {
        let mut cmd = Command::new(tool);
        
        match tool {
            "oxipng" => {
                cmd.arg("-o3")           // 优化级别3
                    .arg("--strip")      // 移除元数据（安全）
                    .arg("--preserve")   // 保留文件属性
                    .arg(input)
                    .arg("-out")
                    .arg(output);
            }
            "optipng" => {
                cmd.arg("-o3")           // 优化级别3
                    .arg("-preserve")    // 保留文件属性
                    .arg("-out")
                    .arg(output)
                    .arg(input);
            }
            _ => anyhow::bail!("Unknown PNG tool: {}", tool),
        }
        
        let output_result = cmd.output()
            .with_context(|| format!("Failed to execute {}", tool))?;
        
        if !output_result.status.success() {
            anyhow::bail!(
                "{} failed: {}",
                tool,
                String::from_utf8_lossy(&output_result.stderr)
            );
        }
        
        Ok(())
    }
    
    /// 优化JPEG
    pub fn optimize_jpeg(&self, input: &Path, output: &Path, quality: u8, tool: &str) -> Result<()> {
        let mut cmd = Command::new(tool);
        
        match tool {
            "mozjpeg" => {
                cmd.arg("-quality")
                    .arg(quality.to_string())
                    .arg("-progressive")
                    .arg("-outfile")
                    .arg(output)
                    .arg(input);
            }
            "jpegtran" => {
                cmd.arg("-copy")
                    .arg(if self.preserve_metadata { "all" } else { "none" })
                    .arg("-optimize")
                    .arg("-progressive")
                    .arg("-outfile")
                    .arg(output)
                    .arg(input);
            }
            _ => anyhow::bail!("Unknown JPEG tool: {}", tool),
        }
        
        let output_result = cmd.output()
            .with_context(|| format!("Failed to execute {}", tool))?;
        
        if !output_result.status.success() {
            anyhow::bail!(
                "{} failed: {}",
                tool,
                String::from_utf8_lossy(&output_result.stderr)
            );
        }
        
        Ok(())
    }
    
    /// 优化WebP
    pub fn optimize_webp(&self, input: &Path, output: &Path, quality: u8) -> Result<()> {
        let mut cmd = Command::new("cwebp");
        
        cmd.arg(input)
            .arg("-o")
            .arg(output)
            .arg("-q")
            .arg(quality.to_string())
            .arg("-m")
            .arg("6")  // 最大努力
            .arg("-mt"); // 多线程
        
        if self.preserve_metadata {
            cmd.arg("-metadata").arg("all");
        }
        
        let output_result = cmd.output()
            .with_context(|| "Failed to execute cwebp")?;
        
        if !output_result.status.success() {
            anyhow::bail!(
                "cwebp 失败: {}",
                String::from_utf8_lossy(&output_result.stderr)
            );
        }
        
        Ok(())
    }
    
    /// 执行同格式优化
    pub fn optimize(&self, input: &Path, output: &Path, format: &str, quality: u8) -> Result<OptimizationResult> {
        let start = Instant::now();
        let input_size = fs::metadata(input)?.len();
        
        let tool = Self::check_tools_for_format(format)?;
        
        match format {
            "png" => self.optimize_png(input, output, &tool)?,
            "jpg" | "jpeg" => self.optimize_jpeg(input, output, quality, &tool)?,
            "webp" => self.optimize_webp(input, output, quality)?,
            _ => anyhow::bail!("Unsupported format: {}", format),
        }
        
        let output_size = fs::metadata(output)?.len();
        let processing_time = start.elapsed();
        
        Ok(OptimizationResult {
            input_size,
            output_size,
            compression_ratio: output_size as f64 / input_size as f64,
            processing_time_ms: processing_time.as_millis() as u64,
            tool_used: tool,
        })
    }
}

/// 优化结果
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub input_size: u64,
    pub output_size: u64,
    pub compression_ratio: f64,
    pub processing_time_ms: u64,
    pub tool_used: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tool_detection() {
        // 测试PNG工具检测
        let png_result = SameFormatOptimizer::check_tools_for_format("png");
        if png_result.is_ok() {
            println!("PNG tool available: {:?}", png_result.unwrap());
        }
        
        // 测试JPEG工具检测
        let jpeg_result = SameFormatOptimizer::check_tools_for_format("jpeg");
        if jpeg_result.is_ok() {
            println!("JPEG tool available: {:?}", jpeg_result.unwrap());
        }
        
        // 测试WebP工具检测
        let webp_result = SameFormatOptimizer::check_tools_for_format("webp");
        if webp_result.is_ok() {
            println!("WebP tool available: {:?}", webp_result.unwrap());
        }
    }
    
    #[test]
    fn test_optimizer_creation() {
        let optimizer = SameFormatOptimizer::new(true);
        assert!(optimizer.preserve_metadata);
        
        let optimizer2 = SameFormatOptimizer::new(false);
        assert!(!optimizer2.preserve_metadata);
    }
}
