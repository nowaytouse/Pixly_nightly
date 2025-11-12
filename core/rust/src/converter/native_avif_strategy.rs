/**
 * Native AVIF Strategy
 * 
 * 使用rav1e进行原生AVIF编码
 */
use crate::converter::strategy::{ConversionStrategy, ConversionConfig, ConversionResult};
use crate::converter::native_avif::{NativeAvifEncoder, AvifConfig};
use anyhow::Result;
use std::path::Path;
use std::time::Instant;

pub struct NativeAvifStrategy;

impl ConversionStrategy for NativeAvifStrategy {
    fn name(&self) -> &str {
        "Native AVIF (rav1e)"
    }
    
    fn is_available(&self) -> bool {
        // 检查是否编译了native-avif feature
        cfg!(feature = "native-avif")
    }
    
    fn supported_formats(&self) -> Vec<String> {
        vec!["avif".to_string()]
    }
    
    fn convert(
        &self,
        input: &Path,
        output: &Path,
        _format: &str,
        config: &ConversionConfig,
    ) -> Result<ConversionResult> {
        let start = Instant::now();
        
        #[cfg(feature = "native-avif")]
        {
            // 转换配置
            let avif_config = AvifConfig {
                quality: config.quality as f32,
                speed: config.speed,
                preserve_alpha: true,
                threads: 0,
                ..Default::default()
            };
            
            // 执行编码
            NativeAvifEncoder::encode(input, output, &avif_config)?;
            
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
        
        #[cfg(not(feature = "native-avif"))]
        {
            Err(anyhow::anyhow!("Native AVIF encoding not available. Compile with --features native-avif"))
        }
    }
    
    fn priority(&self) -> u8 {
        100 // 原生编码器优先级最高
    }
}
