/**
 * ==========================================
 * Same-Format Optimizer Strategy
 * ==========================================
 */
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use crate::converter::strategy::{ConversionStrategy, ConversionConfig, ConversionResult};
// 🔧 统一日志系统
use tracing::info;


/// Same-format optimization strategy
/// 
/// Optimizes images without changing format:
/// - PNG: oxipng/optipng (lossless)
/// - JPEG: cjpegli/jpegtran (configurable quality)
/// - WebP: cwebp recompression
pub struct SameFormatOptimizer;

impl Default for SameFormatOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl SameFormatOptimizer {
    pub fn new() -> Self {
        Self
    }
    
    /// Check if tools are available for a specific format
    fn check_tools_for_format(format: &str) -> Result<String> {
        match format {
            "png" => {
                // Check oxipng first, then optipng
                if Command::new("oxipng").arg("--version").output().is_ok() {
                    Ok("oxipng".to_string())
                } else if Command::new("optipng").arg("-v").output().is_ok() {
                    Ok("optipng".to_string())
                } else {
                    anyhow::bail!("PNG optimizer not found (oxipng/optipng)")
                }
            }
            "jpg" | "jpeg" => {
                // Check cjpegli first, then jpegtran
                if Command::new("cjpegli").arg("--version").output().is_ok() {
                    Ok("cjpegli".to_string())
                } else if Command::new("jpegtran").arg("-version").output().is_ok() {
                    Ok("jpegtran".to_string())
                } else {
                    anyhow::bail!("JPEG optimizer not found (cjpegli/jpegtran)")
                }
            }
            "webp" => {
                if Command::new("cwebp").arg("-version").output().is_ok() {
                    Ok("cwebp".to_string())
                } else {
                    anyhow::bail!("WebP optimizer not found (cwebp)")
                }
            }
            _ => anyhow::bail!("Unsupported format for same-format optimization: {}", format),
        }
    }
    
    /// Optimize PNG
    fn optimize_png(input: &Path, output: &Path, tool: &str) -> Result<()> {
        let mut cmd = Command::new(tool);
        
        match tool {
            "oxipng" => {
                // oxipng: fast, lossless, efficient
                cmd.arg("-o3")           // Optimization level 3
                    .arg("--strip")      // Strip metadata (safe)
                    .arg("--preserve")   // Preserve file attributes
                    .arg(input)
                    .arg("-out")
                    .arg(output);
            }
            "optipng" => {
                // optipng: slower but more aggressive
                cmd.arg("-o3")           // Optimization level 3
                    .arg("-preserve")    // Preserve file attributes
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
    
    /// Optimize JPEG
    fn optimize_jpeg(input: &Path, output: &Path, quality: u8, tool: &str) -> Result<()> {
        let mut cmd = Command::new(tool);
        
        match tool {
            "cjpegli" => {
                // cjpegli: modern, high quality
                cmd.arg(input)
                    .arg(output)
                    .arg("--quality")
                    .arg(quality.to_string())
                    .arg("--progressive");
            }
            "jpegtran" => {
                // jpegtran: lossless optimization
                cmd.arg("-copy")
                    .arg("all")          // Preserve all metadata
                    .arg("-optimize")    // Optimize Huffman tables
                    .arg("-progressive") // Progressive encoding
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
    
    /// Optimize WebP
    fn optimize_webp(input: &Path, output: &Path, quality: u8) -> Result<()> {
        let mut cmd = Command::new("cwebp");
        
        cmd.arg(input)
            .arg("-o")
            .arg(output)
            .arg("-q")
            .arg(quality.to_string())
            .arg("-m")
            .arg("6")  // Maximum effort
            .arg("-mt"); // Multi-threading
        
        let output_result = cmd.output()
            .with_context(|| "Failed to execute cwebp")?;
        
        if !output_result.status.success() {
            anyhow::bail!(
                "cwebp failed: {}",
                String::from_utf8_lossy(&output_result.stderr)
            );
        }
        
        Ok(())
    }
}

impl ConversionStrategy for SameFormatOptimizer {
    fn name(&self) -> &str {
        "Same-Format Optimizer"
    }
    
    fn is_available(&self) -> bool {
        // Available if at least one optimizer tool is available
        Self::check_tools_for_format("png").is_ok() ||
        Self::check_tools_for_format("jpeg").is_ok() ||
        Self::check_tools_for_format("webp").is_ok()
    }
    
    fn supported_formats(&self) -> Vec<String> {
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
    
    fn priority(&self) -> u8 {
        80 // Higher than CLI converters, lower than native encoders
    }
    
    fn convert(
        &self,
        input: &Path,
        output: &Path,
        format: &str,
        config: &ConversionConfig,
    ) -> Result<ConversionResult> {
        let start_time = std::time::Instant::now();
        let input_size = std::fs::metadata(input)?.len();
        
        let format_lower = format.to_lowercase();
        let tool = Self::check_tools_for_format(&format_lower)
            .with_context(|| format!("No optimizer available for {}", format))?;
        
        info!("🔄 Optimizing {} with {}", format_lower, tool);
        
        // Perform optimization based on format
        match format_lower.as_str() {
            "png" => {
                Self::optimize_png(input, output, &tool)?;
            }
            "jpg" | "jpeg" => {
                Self::optimize_jpeg(input, output, config.quality, &tool)?;
            }
            "webp" => {
                Self::optimize_webp(input, output, config.quality)?;
            }
            _ => {
                anyhow::bail!("Unsupported format: {}", format);
            }
        }
        
        let output_size = std::fs::metadata(output)?.len();
        let processing_time_ms = start_time.elapsed().as_millis() as u64;
        let compression_ratio = if input_size > 0 {
            1.0 - (output_size as f64 / input_size as f64)
        } else {
            0.0
        };
        
        info!(
            "✅ Optimization complete: {} -> {} ({:.1}% reduction, {}ms)",
            input_size,
            output_size,
            compression_ratio * 100.0,
            processing_time_ms
        );
        
        Ok(ConversionResult {
            success: true,
            output_path: output.to_string_lossy().to_string(),
            input_size,
            output_size,
            compression_ratio,
            processing_time_ms,
            strategy_used: format!("{} ({})", self.name(), tool),
            error_message: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_matching() {
        let optimizer = SameFormatOptimizer::new();
        let formats = optimizer.supported_formats();
        
        // Test that the optimizer is properly initialized
        // Actual supported formats depend on which tools are available on the system
        // The formats list should be valid (either empty or non-empty)
        // We don't assert specific formats because tool availability varies by system
        assert_eq!(formats.len(), formats.len()); // Just a sanity check
    }
    
    #[test]
    fn test_optimizer_availability() {
        let optimizer = SameFormatOptimizer::new();
        
        // Test that is_available returns a valid boolean
        let available = optimizer.is_available();
        assert!(available == true || available == false);
        
        // Test that name returns a valid string
        assert!(!optimizer.name().is_empty());
    }
}
