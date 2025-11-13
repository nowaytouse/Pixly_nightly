/**
 * Native WebP Strategy
 * 
 * 使用webp crate进行原生WebP编码
 */
use crate::converter::strategy::{ConversionStrategy, ConversionConfig, ConversionResult};
use crate::converter::native_webp::{NativeWebPEncoder, WebPConfig};
use anyhow::Result;
use std::path::Path;
use std::time::Instant;

pub struct NativeWebPStrategy;

impl ConversionStrategy for NativeWebPStrategy {
    fn name(&self) -> &str {
        "Native WebP (webp)"
    }
    
    fn is_available(&self) -> bool {
        // 检查是否编译了native-webp feature
        cfg!(feature = "native-webp")
    }
    
    fn supported_formats(&self) -> Vec<String> {
        vec!["webp".to_string()]
    }
    
    fn convert(
        &self,
        input: &Path,
        output: &Path,
        _format: &str,
        config: &ConversionConfig,
    ) -> Result<ConversionResult> {
        let start = Instant::now();
        
        #[cfg(feature = "native-webp")]
        {
            // 转换配置
            let webp_config = WebPConfig {
                quality: config.quality as f32,
                lossless: false, // 可以从config扩展
                preserve_alpha: true,
            };
            
            // 执行编码
            NativeWebPEncoder::encode(input, output, &webp_config)?;
            
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
        
        #[cfg(not(feature = "native-webp"))]
        {
            Err(anyhow::anyhow!("Native WebP encoding not available. Compile with --features native-webp"))
        }
    }
    
    fn priority(&self) -> u8 {
        100 // 原生编码器优先级最高
    }
}
