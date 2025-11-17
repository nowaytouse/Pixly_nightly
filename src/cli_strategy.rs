/**
 * CLI Tool Strategy
 * 
 * 使用命令行工具进行转换（高质量方案）
 * Phase 47.18: 不是fallback，而是一种可选的高质量转换策略
 */
use crate::converter::strategy::{ConversionStrategy, ConversionConfig, ConversionResult};
use std::path::Path;
use anyhow::{Context, Result};
use std::process::Command;
use std::time::Instant;
// 🔧 统一日志系统
use tracing::{info, debug};

/// CLI工具类型
#[derive(Debug, Clone)]
pub enum CliTool {
    /// avifenc - AVIF编码器
    Avifenc,
    /// cjxl - JPEG XL编码器
    Cjxl,
    /// djxl - JPEG XL解码器 🔥 用于JXL→其他格式
    Djxl,
    /// cwebp - WebP编码器
    Cwebp,
    /// ImageMagick convert
    Magick,
}

impl CliTool {
    /// 获取工具命令名
    pub fn command(&self) -> &str {
        match self {
            Self::Avifenc => "avifenc",
            Self::Cjxl => "cjxl",
            Self::Djxl => "djxl",
            Self::Cwebp => "cwebp",
            Self::Magick => "magick",
        }
    }
    
    /// 构建命令行参数
    pub fn build_args(&self, input: &Path, output: &Path, config: &ConversionConfig) -> Vec<String> {
        match self {
            Self::Avifenc => vec![
                "-q".to_string(),
                config.quality.to_string(),
                "-s".to_string(),
                config.speed.to_string(),
                input.to_string_lossy().to_string(),
                output.to_string_lossy().to_string(),
            ],
            Self::Cjxl => {
                // 🔥 Phase 39: JXL智能参数优化（基于AI预测）
                // 🔥 Phase 47.14: 增强JPEG兼容性检测和异常处理
                
                let is_jpeg_input = input.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| matches!(e.to_lowercase().as_str(), "jpg" | "jpeg"))
                    .unwrap_or(false);
                
                
                let mut args = vec![
                input.to_string_lossy().to_string(),
                output.to_string_lossy().to_string(),
                ];
                
                // 🔥 Phase 39: 基于AI预测的lossless参数决策
                // 关键原则：lossless_jpeg=1时，不能设置quality<100
                
                // 🔥 修复：cjxl的--effort必须在1-10范围内
                // AI可能返回speed=0，需要映射到valid range
                let effort = if config.speed == 0 {
                    7  // 默认balanced effort
                } else if config.speed > 10 {
                    10  // 最大effort
                } else {
                    config.speed.max(1)  // 至少为1
                };
                
                // 🔥 修复: JPEG输入时智能判断是否使用无损转码
                // 只有在明确指定无损或quality=100时才使用lossless_jpeg
                if is_jpeg_input && (config.lossless || config.quality == 100) {
                    // JPEG输入 + 无损要求 -> 使用无损转码
                    // 优势: 完全相同质量 + ~25%大小减少
                    args.push("--lossless_jpeg=1".to_string());
                    // ⚠️ lossless_jpeg=1时，不设置quality参数
                    // cjxl会自动使用quality=100
                    args.push("-e".to_string());
                    args.push(effort.to_string());
                    info!("🎯 JXL lossless JPEG transcoding, effort={}", effort);
                } else if is_jpeg_input {
                    // JPEG输入 + 有损转换
                    // 🔥 不设置默认的lossless_jpeg，避免和quality参数冲突
                    args.push("--lossless_jpeg=0".to_string());  // 明确禁用lossless_jpeg
                    args.push("-q".to_string());
                    args.push(config.quality.to_string());
                    args.push("-e".to_string());
                    args.push(effort.to_string());
                    debug!("🎨 JXL lossy JPEG conversion: quality={}, effort={}", config.quality, effort);
                } else {
                    // 非JPEG输入的普通转换
                    args.push("-q".to_string());
                    args.push(config.quality.to_string());
                    args.push("-e".to_string());
                    args.push(effort.to_string());
                    debug!("🎨 JXL lossy conversion: quality={}, effort={}", config.quality, effort);
                }
                
                args
            },
            Self::Djxl => {
                // djxl: JXL解码器（JXL → PNG/JPEG等）
                // djxl input.jxl output.png
                vec![
                    input.to_string_lossy().to_string(),
                    output.to_string_lossy().to_string(),
                ]
            },
            Self::Cwebp => vec![
                "-q".to_string(),
                config.quality.to_string(),
                input.to_string_lossy().to_string(),
                "-o".to_string(),
                output.to_string_lossy().to_string(),
            ],
            Self::Magick => vec![
                "convert".to_string(),
                input.to_string_lossy().to_string(),
                "-quality".to_string(),
                config.quality.to_string(),
                output.to_string_lossy().to_string(),
            ],
        }
    }
    
    /// 支持的格式
    pub fn supported_formats(&self) -> Vec<String> {
        match self {
            Self::Avifenc => vec!["avif".to_string()],
            Self::Cjxl => vec!["jxl".to_string()],
            Self::Djxl => vec!["png".to_string(), "jpg".to_string(), "jpeg".to_string()], // JXL解码输出
            Self::Cwebp => vec!["webp".to_string()],
            Self::Magick => vec!["jpg".to_string(), "jpeg".to_string(), "png".to_string(), "heic".to_string()],
        }
    }
    
    /// 检查工具是否可用
    pub fn is_available(&self) -> bool {
        Command::new(self.command())
            .arg("--version")
            .output()
            .is_ok()
    }
}

/// CLI策略实现
pub struct CliStrategy {
    tool: CliTool,
}

impl CliStrategy {
    pub fn new(tool: CliTool) -> Self {
        Self { tool }
    }
}

impl ConversionStrategy for CliStrategy {
    fn name(&self) -> &str {
        match self.tool {
            CliTool::Avifenc => "CLI AVIF (avifenc)",
            CliTool::Cjxl => "CLI JXL Encoder (cjxl)",
            CliTool::Djxl => "CLI JXL Decoder (djxl)",
            CliTool::Cwebp => "CLI WebP (cwebp)",
            CliTool::Magick => "CLI Generic (magick)",
        }
    }
    
    fn is_available(&self) -> bool {
        self.tool.is_available()
    }
    
    fn supported_formats(&self) -> Vec<String> {
        self.tool.supported_formats()
    }
    
    fn convert(
        &self,
        input: &Path,
        output: &Path,
        _format: &str,
        config: &ConversionConfig,
    ) -> Result<ConversionResult> {
        let start = Instant::now();
        
        let input_path = input;
        let output_path = output;
        
        // 构建命令
        let args = self.tool.build_args(input_path, output_path, config);
        
        // 执行命令
        let output = Command::new(self.tool.command())
            .args(&args)
            .output()
            .with_context(|| format!("Failed to execute {}", self.tool.command()))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("CLI tool failed: {}", stderr);
        }
        
        // 收集结果
        let input_size = std::fs::metadata(input_path)?.len();
        let output_size = std::fs::metadata(output_path)?.len();
        let elapsed = start.elapsed().as_millis() as u64;
        
        Ok(ConversionResult {
            success: true,
            output_path: output_path.to_string_lossy().to_string(),
            input_size,
            output_size,
            compression_ratio: 1.0 - (output_size as f64 / input_size as f64),
            processing_time_ms: elapsed,
            strategy_used: self.name().to_string(),
            error_message: None,
        })
    }
    
    fn priority(&self) -> u8 {
        50 // CLI工具优先级中等（作为fallback）
    }
}
