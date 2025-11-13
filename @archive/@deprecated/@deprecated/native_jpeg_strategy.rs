/**
 * Native JPEG Strategy
 * 
 * 使用image crate的JPEG编码器进行原生编码
 */
use crate::converter::strategy::{ConversionStrategy, ConversionConfig, ConversionResult};
use crate::converter::native_jpeg::{NativeJpegEncoder, JpegConfig};
use anyhow::Result;
use std::path::Path;
use std::time::Instant;

pub struct NativeJpegStrategy;

impl ConversionStrategy for NativeJpegStrategy {
    fn name(&self) -> &str {
        "Native JPEG (image)"
    }
    
    fn is_available(&self) -> bool {
        // JPEG编码器始终可用（内置）
        true
    }
    
    fn supported_formats(&self) -> Vec<String> {
        vec!["jpg".to_string(), "jpeg".to_string()]
    }
    
    fn convert(
        &self,
        input: &Path,
        output: &Path,
        _format: &str,
        config: &ConversionConfig,
    ) -> Result<ConversionResult> {
        let start = Instant::now();
        
        // 转换配置
        let jpeg_config = JpegConfig {
            quality: config.quality,
            progressive: false,  // 可根据需求调整
            optimize_coding: true,
            chroma_subsampling: crate::converter::native_jpeg::ChromaSubsampling::Sample420,
        };
        
        // 执行编码
        NativeJpegEncoder::encode(input, output, &jpeg_config)?;
        
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
