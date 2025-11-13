/**
 * Native PNG Strategy
 * 
 * 使用png crate进行原生PNG编码
 */
use crate::converter::strategy::{ConversionStrategy, ConversionConfig, ConversionResult};
use crate::converter::native_png::{NativePngEncoder, PngConfig};
use anyhow::Result;
use std::path::Path;
use std::time::Instant;

pub struct NativePngStrategy;

impl ConversionStrategy for NativePngStrategy {
    fn name(&self) -> &str {
        "Native PNG (png)"
    }
    
    fn is_available(&self) -> bool {
        // PNG编码器始终可用（内置）
        true
    }
    
    fn supported_formats(&self) -> Vec<String> {
        vec!["png".to_string()]
    }
    
    fn convert(
        &self,
        input: &Path,
        output: &Path,
        _format: &str,
        config: &ConversionConfig,
    ) -> Result<ConversionResult> {
        let start = Instant::now();
        
        // 转换配置 (quality映射到compression_level)
        // quality 0-100 -> compression 0-6
        let compression_level = ((100 - config.quality) * 6 / 100).min(6);
        
        let png_config = PngConfig {
            compression_level,
            filter_optimization: true,
            preserve_alpha: true,
        };
        
        // 执行编码
        NativePngEncoder::encode(input, output, &png_config)?;
        
        // 收集结果
        let input_size = std::fs::metadata(input)?.len();
        let output_size = std::fs::metadata(output)?.len();
        let elapsed = start.elapsed().as_millis() as u64;
        
        Ok(ConversionResult {
            success: true,
            output_path: output.to_string_lossy().to_string(),
            input_size,
            output_size,
            compression_ratio: 1.0 - (output_size as f64 / input_size as f64),
            processing_time_ms: elapsed,
            strategy_used: self.name().to_string(),
            error_message: None,
        })
    }
    
    fn priority(&self) -> u8 {
        100 // 原生编码器优先级最高
    }
}
