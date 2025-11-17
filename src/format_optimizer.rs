// 🚀 同格式优化器
// 从 @archive/rust_broken/src/converter/same_format_optimizer.rs 提取并增强
//
// 核心功能:
// - PNG优化 (oxipng/optipng)
// - JPEG优化 (cjpegli/jpegtran)
// - WebP重压缩优化
// - 无损/有损可配置
// - 自动工具检测

use anyhow::{Context, Result, bail};
use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};

/// 优化配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerConfig {
    pub quality: u8,
    pub lossless: bool,
    pub strip_metadata: bool,
    pub optimization_level: u8,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            quality: 85,
            lossless: false,
            strip_metadata: true,
            optimization_level: 3,
        }
    }
}

/// 优化结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerResult {
    pub success: bool,
    pub input_size: u64,
    pub output_size: u64,
    pub compression_ratio: f64,
    pub processing_time_ms: u64,
    pub tool_used: String,
}

/// 同格式优化器
pub struct FormatOptimizer {
    config: OptimizerConfig,
}

impl FormatOptimizer {
    pub fn new(config: OptimizerConfig) -> Self {
        Self { config }
    }
    
    pub fn with_defaults() -> Self {
        Self::new(OptimizerConfig::default())
    }
    
    fn check_tool(name: &str) -> bool {
        Command::new(name)
            .arg("--version")
            .output()
            .is_ok()
    }
    
    fn check_tools_for_format(format: &str) -> Result<String> {
        match format {
            "png" => {
                if Self::check_tool("oxipng") {
                    Ok("oxipng".to_string())
                } else if Self::check_tool("optipng") {
                    Ok("optipng".to_string())
                } else {
                    bail!("PNG optimizer not found (oxipng/optipng)")
                }
            }
            "jpg" | "jpeg" => {
                if Self::check_tool("cjpegli") {
                    Ok("cjpegli".to_string())
                } else if Self::check_tool("jpegtran") {
                    Ok("jpegtran".to_string())
                } else {
                    bail!("JPEG optimizer not found (cjpegli/jpegtran)")
                }
            }
            "webp" => {
                if Self::check_tool("cwebp") {
                    Ok("cwebp".to_string())
                } else {
                    bail!("WebP optimizer not found (cwebp)")
                }
            }
            _ => bail!("Unsupported format: {}", format),
        }
    }
    
    fn optimize_png(&self, input: &Path, output: &Path, tool: &str) -> Result<()> {
        let mut cmd = Command::new(tool);
        
        match tool {
            "oxipng" => {
                cmd.arg(format!("-o{}", self.config.optimization_level))
                    .arg("--strip").arg("safe")
                    .arg(input)
                    .arg("-out").arg(output);
            }
            "optipng" => {
                cmd.arg(format!("-o{}", self.config.optimization_level))
                    .arg("-preserve")
                    .arg("-out").arg(output)
                    .arg(input);
            }
            _ => bail!("Unknown PNG tool: {}", tool),
        }
        
        let output_result = cmd.output()
            .with_context(|| format!("Failed to execute {}", tool))?;
        
        if !output_result.status.success() {
            bail!("{} failed: {}", tool, String::from_utf8_lossy(&output_result.stderr));
        }
        
        Ok(())
    }
    
    fn optimize_jpeg(&self, input: &Path, output: &Path, tool: &str) -> Result<()> {
        let mut cmd = Command::new(tool);
        
        match tool {
            "cjpegli" => {
                cmd.arg(input)
                    .arg(output)
                    .arg("--quality").arg(self.config.quality.to_string())
                    .arg("--progressive");
            }
            "jpegtran" => {
                cmd.arg("-copy").arg("all")
                    .arg("-optimize")
                    .arg("-progressive")
                    .arg("-outfile").arg(output)
                    .arg(input);
            }
            _ => bail!("Unknown JPEG tool: {}", tool),
        }
        
        let output_result = cmd.output()
            .with_context(|| format!("Failed to execute {}", tool))?;
        
        if !output_result.status.success() {
            bail!("{} failed: {}", tool, String::from_utf8_lossy(&output_result.stderr));
        }
        
        Ok(())
    }
    
    fn optimize_webp(&self, input: &Path, output: &Path) -> Result<()> {
        let mut cmd = Command::new("cwebp");
        
        cmd.arg(input)
            .arg("-o").arg(output)
            .arg("-q").arg(self.config.quality.to_string())
            .arg("-m").arg("6")
            .arg("-mt");
        
        let output_result = cmd.output()
            .with_context(|| "Failed to execute cwebp")?;
        
        if !output_result.status.success() {
            bail!("cwebp failed: {}", String::from_utf8_lossy(&output_result.stderr));
        }
        
        Ok(())
    }
    
    pub fn optimize(&self, input: &Path, output: &Path, format: &str) -> Result<OptimizerResult> {
        let start_time = std::time::Instant::now();
        let input_size = std::fs::metadata(input)?.len();
        
        let format_lower = format.to_lowercase();
        let tool = Self::check_tools_for_format(&format_lower)
            .with_context(|| format!("No optimizer available for {}", format))?;
        
        match format_lower.as_str() {
            "png" => self.optimize_png(input, output, &tool)?,
            "jpg" | "jpeg" => self.optimize_jpeg(input, output, &tool)?,
            "webp" => self.optimize_webp(input, output)?,
            _ => bail!("Unsupported format: {}", format),
        }
        
        let output_size = std::fs::metadata(output)?.len();
        let processing_time_ms = start_time.elapsed().as_millis() as u64;
        let compression_ratio = if input_size > 0 {
            1.0 - (output_size as f64 / input_size as f64)
        } else {
            0.0
        };
        
        Ok(OptimizerResult {
            success: true,
            input_size,
            output_size,
            compression_ratio,
            processing_time_ms,
            tool_used: tool,
        })
    }
    
    pub fn is_available_for_format(format: &str) -> bool {
        Self::check_tools_for_format(format).is_ok()
    }
    
    pub fn supported_formats() -> Vec<String> {
        let mut formats = Vec::new();
        
        if Self::check_tools_for_format("png").is_ok() {
            formats.push("png".to_string());
        }
        if Self::check_tools_for_format("jpeg").is_ok() {
            formats.push("jpg".to_string());
            formats.push("jpeg".to_string());
        }
        if Self::check_tools_for_format("webp").is_ok() {
            formats.push("webp".to_string());
        }
        
        formats
    }
}

impl Default for FormatOptimizer {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_optimizer_creation() {
        let optimizer = FormatOptimizer::with_defaults();
        assert_eq!(optimizer.config.quality, 85);
        assert_eq!(optimizer.config.optimization_level, 3);
    }
    
    #[test]
    fn test_supported_formats() {
        let formats = FormatOptimizer::supported_formats();
        assert!(!formats.is_empty());
    }
}
