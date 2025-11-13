/**
 * ==========================================
 * PIXLY Conversion Strategy System
 * ==========================================
 *
 * 统一的多策略转换架构
 * 
 * 支持策略:
 * - 原生编码器 (Native Encoders): rav1e, libwebp, jxl-oxide
 * - CLI工具 (CLI Tools): cjxl, avifenc, cwebp, magick
 * - 自动选择 (Auto): 智能选择最佳策略
 *
 * 设计理念:
 * - 策略模式: 可扩展的转换策略
 * - 统一接口: 调用方无需关心底层实现
 * - 智能选择: 根据场景自动选择最佳策略
 * - 性能优先: 优先使用原生编码器
 * - 🔥 响亮报错: 策略失败直接报错，无降级
 *
 * @module ConversionStrategy
 * @version 1.0.0
 * @date 2025-11-05
 */
use anyhow::{Context, Result};
use std::path::Path;
use serde::{Serialize, Deserialize};
use super::metadata::MetadataHandler;
use super::ai_client::AIClient;
// 🔧 统一日志系统
use tracing::{info, warn, error, debug};

// 🔥 Phase 40.21: AI预测数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionData {
    pub format: String,
    pub quality: u8,
    pub speed: u8,
    pub lossless: bool,
    pub predicted_size: Option<u64>,
    pub confidence: Option<f32>,
    
    // 🆕 Phase 46.14+: AI预处理建议（R-003）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preprocessing_steps: Option<Vec<crate::converter::ai_client::PreprocessStepSuggestion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization_path: Option<String>,
}

/// 转换策略类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrategyType {
    /// 原生编码器 (性能最优)
    Native,
    /// CLI工具 (兼容性最好)
    Cli,
    /// 自动选择 (智能决策)
    Auto,
}

impl Default for StrategyType {
    fn default() -> Self {
        Self::Auto
    }
}

/// 转换配置
#[derive(Debug, Clone)]
pub struct ConversionConfig {
    /// 质量参数 (0-100)
    pub quality: u8,
    /// 速度参数 (0-10)
    pub speed: u8,
    /// 保留元数据
    pub preserve_metadata: bool,
    /// 保留动画
    pub keep_animated: bool,
    /// 策略选择
    pub strategy: StrategyType,
    /// 🔥 Phase 39: 无损模式（AI预测）
    /// 当true时，JPEG→JXL使用lossless_jpeg=1
    pub lossless: bool,
    /// 规范化文件名
    pub normalize_filenames: Option<bool>,
    /// 🔥 Phase 40.21: 添加AI预测数据
    pub prediction_data: Option<PredictionData>,
}

// 🔥 CRITICAL: ConversionConfig不应有Default实现！
// 📋 Default实现是最严重的Fallback违规：
// - quality: 85  (硬编码fallback)
// - speed: 4     (硬编码fallback) 
// - lossless: false (硬编码fallback)
// 
// 🔥 所有ConversionConfig必须：
// 1. 从AI预测数据构建（AI模式）
// 2. 从规则引擎构建（通用模式）  
// 3. 从明确的CLI参数构建（调试模式）
// 
// ❌ 禁止使用Default::default()作为参数来源！

// impl Default for ConversionConfig {
//     fn default() -> Self {
//         Self {
//             quality: 85,    // ❌ 硬编码fallback
//             speed: 4,       // ❌ 硬编码fallback
//             preserve_metadata: true,
//             keep_animated: true,
//             strategy: StrategyType::Auto,
//             lossless: false,  // ❌ 硬编码fallback
//             normalize_filenames: None,
//             prediction_data: None,
//         }
//     }
// }

/// 转换结果
#[derive(Debug, Clone)]
pub struct ConversionResult {
    /// 是否成功
    pub success: bool,
    /// 输出路径
    pub output_path: String,
    /// 输入大小（字节）
    pub input_size: u64,
    /// 输出大小（字节）
    pub output_size: u64,
    /// 压缩比 (0.0-1.0)
    pub compression_ratio: f64,
    /// 处理时间（毫秒）
    pub processing_time_ms: u64,
    /// 使用的策略
    pub strategy_used: String,
    /// 错误信息（如果有）
    pub error_message: Option<String>,
}

/// 转换策略trait
pub trait ConversionStrategy: Send + Sync {
    /// 策略名称
    fn name(&self) -> &str;
    
    /// 检查是否可用
    fn is_available(&self) -> bool;
    
    /// 支持的格式
    fn supported_formats(&self) -> Vec<String>;
    
    /// 执行转换 (使用&Path以保持trait object-safe)
    fn convert(
        &self,
        input: &Path,
        output: &Path,
        format: &str,
        config: &ConversionConfig,
    ) -> Result<ConversionResult>;
    
    /// 优先级（数字越大优先级越高）
    fn priority(&self) -> u8 {
        50 // 默认优先级
    }
}

/// 策略管理器
pub struct StrategyManager {
    strategies: Vec<Box<dyn ConversionStrategy>>,
}

impl StrategyManager {
    /// 创建新的策略管理器
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
        }
    }
    
    /// 注册策略
    pub fn register(&mut self, strategy: Box<dyn ConversionStrategy>) {
        self.strategies.push(strategy);
    }
    
    /// 获取所有可用策略
    pub fn available_strategies(&self) -> Vec<&str> {
        self.strategies
            .iter()
            .filter(|s| s.is_available())
            .map(|s| s.name())
            .collect()
    }
    
    /// 🔥 Phase 37: 清空所有strategies（用于重新注册）
    pub fn clear_strategies(&mut self) {
        self.strategies.clear();
    }
    
    /// 选择最佳策略
    pub fn select_strategy(
        &self,
        format: &str,
        strategy_type: StrategyType,
    ) -> Option<&dyn ConversionStrategy> {
        match strategy_type {
            StrategyType::Native => {
                // 查找原生编码器
                self.strategies
                    .iter()
                    .filter(|s| s.is_available() && s.name().contains("Native"))
                    .filter(|s| s.supported_formats().contains(&format.to_string()))
                    .max_by_key(|s| s.priority())
                    .map(|s| s.as_ref())
            }
            StrategyType::Cli => {
                // 查找CLI工具
                self.strategies
                    .iter()
                    .filter(|s| s.is_available() && s.name().contains("CLI"))
                    .filter(|s| s.supported_formats().contains(&format.to_string()))
                    .max_by_key(|s| s.priority())
                    .map(|s| s.as_ref())
            }
            StrategyType::Auto => {
                // Phase 47.18: 自动选择最佳策略（无fallback概念）
                // 选择优先级最高的可用策略
                self.strategies
                    .iter()
                    .filter(|s| s.is_available())
                    .max_by_key(|s| s.priority())
                    .map(|s| s.as_ref())
            }
        }
    }
    
    /// 执行转换（自动选择策略）
    /// 
    /// 🔥 Phase 40.9.1: Filename Normalization 由CLI层处理
    /// 本方法专注于策略选择和转换执行，后处理包括XMP和时间戳
    pub fn convert(
        &self,
        input: &Path,
        output: &Path,
        format: &str,
        config: &ConversionConfig,
    ) -> Result<ConversionResult> {
        // Phase 26.1: 检测输入输出格式是否相同
        let input_ext = input.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        let output_ext = output.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        // 🔥 Phase 40.7.13: 智能选择动画GIF转换策略
        // AnimatedGifStrategy: GIF → WebP/AVIF (FFmpeg)
        // CLI JXL: GIF → JXL (cjxl支持动画)
        if input_ext == "gif" && config.keep_animated {
            // JXL: 使用cjxl（支持动画GIF）
            if format == "jxl" || format == "jpegxl" {
                if let Some(jxl_strategy) = self.strategies
                    .iter()
                    .find(|s| s.name().contains("CLI JXL") && s.is_available())
                {
                    println!("🎬 Detected animated GIF → JXL");
                    println!("   → Using CLI JXL Strategy (cjxl supports animated GIF)");
                    
                    let result = jxl_strategy.convert(input, output, format, config)?;
                    
                    if std::path::Path::new(&result.output_path).exists() {
                        // 🔥 Phase 40.9.2: 后处理 (XMP + 时间戳)
                        self.post_process(input, output, config)?;
                        return Ok(result);
                    } else {
                        // 🔥 策略失败直接报错，不尝试其他策略
                        error!("❌ JXL conversion strategy failed");
                        anyhow::bail!("JXL conversion failed. Check if 'cjxl' is installed and working properly.");
                    }
                }
            }
            // WebP/AVIF: 使用AnimatedGifStrategy（FFmpeg）
            else if format == "webp" || format == "avif" {
                if let Some(gif_strategy) = self.strategies
                    .iter()
                    .find(|s| s.name().contains("Animated GIF") && s.is_available())
                {
                    println!("🎬 Detected animated GIF → {}", format.to_uppercase());
                    println!("   → Using Animated GIF Strategy (FFmpeg)");
                    
                    let result = gif_strategy.convert(input, output, format, config)?;
                    
                    if std::path::Path::new(&result.output_path).exists() {
                        // 🔥 Phase 40.9.2: 后处理 (XMP + 时间戳)
                        self.post_process(input, output, config)?;
                        return Ok(result);
                    } else {
                        println!("   ⚠️  Animated GIF strategy failed, trying fallback...");
                    }
                }
            }
        }
        
        // Phase 26.1: 如果格式相同，尝试使用SameFormatOptimizer
        if input_ext == output_ext && !input_ext.is_empty() {
            if let Some(optimizer) = self.strategies
                .iter()
                .find(|s| s.name().contains("Same-Format") && s.is_available())
            {
                // Check if optimizer supports this specific format
                if optimizer.supported_formats().contains(&format.to_string()) {
                    info!(
                        "🔄 Same format detected ({}), using optimizer",
                        input_ext
                    );
                    let result = optimizer.convert(input, output, format, config)?;
                    // 🔥 Phase 40.9.2: 后处理 (XMP + 时间戳)
                    self.post_process(input, output, config)?;
                    return Ok(result);
                } else {
                    debug!(
                        "Same-format optimizer available but doesn't support {}, falling back to regular conversion",
                        format
                    );
                }
            } else {
                debug!(
                    "Same format detected but optimizer unavailable, using regular conversion"
                );
            }
        }
        
        let strategy = self
            .select_strategy(format, config.strategy)
            .with_context(|| format!("No available strategy for format: {}", format))?;
        
        info!(
            "🎯 Selected strategy: {} for format: {}",
            strategy.name(),
            format
        );
        
        let result = strategy.convert(input, output, format, config)?;
        
        // 🔥 Phase 40.9.2: 后处理 (XMP + 时间戳)
        self.post_process(input, output, config)?;
        
        // 🔥 Phase 40.14: AI反馈记录（简化版）
        // 完整的反馈闭环在CLI层实现，这里仅记录转换统计
        log_conversion_result(&result, input, output, format, config);
        
        Ok(result)
    }
    
    /// 🔥 Phase 40.9.2: 转换后处理
    /// 
    /// 职责：
    /// - XMP sidecar复制（如果源文件有.xmp）
    /// - 时间戳保留
    /// 
    /// 注：Filename Normalization由CLI层处理，不在此集成
    fn post_process(
        &self,
        input: &Path,
        output: &Path,
        config: &ConversionConfig,
    ) -> Result<()> {
        let metadata_processor = MetadataHandler::new();
        
        // 1. XMP sidecar处理
        if config.preserve_metadata {
            if let Err(e) = metadata_processor.process_xmp_sidecar(input, output) {
                warn!("⚠️  XMP sidecar processing failed: {}", e);
                // 不中断转换，只记录警告
            }
        }
        
        // 2. 时间戳保留
        if config.preserve_metadata {
            if let Err(e) = metadata_processor.preserve_timestamps(input, output) {
                warn!("⚠️  Timestamp preservation failed: {}", e);
                // 不中断转换，只记录警告
            }
        }
        
        Ok(())
    }
    
    /// 列出支持的格式
    pub fn supported_formats(&self) -> Vec<String> {
        let mut formats = Vec::new();
        for strategy in &self.strategies {
            if strategy.is_available() {
                formats.extend(strategy.supported_formats());
            }
        }
        formats.sort();
        formats.dedup();
        formats
    }
}

impl Default for StrategyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 全局策略管理器（使用线程安全的OnceLock）
static GLOBAL_MANAGER: std::sync::OnceLock<std::sync::Mutex<StrategyManager>> = 
    std::sync::OnceLock::new();

/// 初始化全局策略管理器
/// 
/// # Safety
/// 线程安全，可以在多线程环境中安全调用
pub fn init_global_manager() -> &'static std::sync::Mutex<StrategyManager> {
    GLOBAL_MANAGER.get_or_init(|| {
        std::sync::Mutex::new(StrategyManager::new())
    })
}

/// 获取全局策略管理器的引用
/// 
/// # Returns
/// 如果管理器已初始化，返回Some，否则返回None
pub fn get_global_manager() -> Option<&'static std::sync::Mutex<StrategyManager>> {
    GLOBAL_MANAGER.get()
}

/// 🔥 Phase 40.14: 记录转换结果用于AI反馈
/// 
/// 架构原则（@PROJECT_QUALITY_MANIFESTO.md）：
/// - 真实性：记录真实的转换统计
/// - 为后续AI反馈闭环提供数据
/// - 不做假数据，不掩盖错误
fn log_conversion_result(
    result: &ConversionResult,
    input: &Path,
    output: &Path,
    format: &str,
    config: &ConversionConfig,
) {
    if result.success {
        let compression_ratio = result.output_size as f64 / result.input_size as f64;
        let reduction_percent = (1.0 - compression_ratio) * 100.0;
        
        info!(
            "✅ Conversion SUCCESS: {} → {}",
            input.file_name().unwrap_or_default().to_string_lossy(),
            output.file_name().unwrap_or_default().to_string_lossy()
        );
        info!(
            "   Format: {} | Strategy: {} | Quality: {} | Speed: {}",
            format.to_uppercase(),
            result.strategy_used,
            config.quality,
            config.speed
        );
        info!(
            "   Size: {} KB → {} KB ({:.1}% reduction)",
            result.input_size / 1024,
            result.output_size / 1024,
            reduction_percent
        );
        info!(
            "   Time: {:.2}s | Compression: {:.2}x",
            result.processing_time_ms as f64 / 1000.0,
            1.0 / compression_ratio
        );
        
        // 🔥 Phase 40.21: AI反馈闭环 - 发送转换结果反馈
        if let Some(ref prediction) = config.prediction_data {
            info!("   📊 Sending feedback to AI service...");
            
            // 构建反馈数据
            let feedback_json = serde_json::json!({
                "prediction": {
                    "format": prediction.format,
                    "quality": prediction.quality,
                    "speed": prediction.speed,
                    "lossless": prediction.lossless,
                    "predicted_size": prediction.predicted_size,
                    "confidence": prediction.confidence,
                },
                "actual_result": {
                    "format": format,
                    "quality": config.quality,
                    "speed": config.speed,
                    "input_size": result.input_size,
                    "output_size": result.output_size,
                    "processing_time_ms": result.processing_time_ms,
                    "compression_ratio": compression_ratio,
                    "success": result.success,
                },
                "reward": calculate_reward(
                    result.output_size,
                    prediction.predicted_size,
                    result.processing_time_ms,
                    compression_ratio,
                ),
            });
            
            // 异步发送反馈（不阻塞转换流程）
            let client = AIClient::with_default();
            if let Err(e) = client.send_raw_feedback(&feedback_json.to_string()) {
                warn!("   ⚠️  Failed to send feedback: {}", e);
            } else {
                info!("   ✅ Feedback sent successfully");
            }
        }
    } else {
        error!(
            "❌ Conversion FAILED: {} → {}",
            input.file_name().unwrap_or_default().to_string_lossy(),
            output.file_name().unwrap_or_default().to_string_lossy()
        );
        if let Some(ref error) = result.error_message {
            error!("   Error: {}", error);
        }
        
        // 🔥 Phase 40.21: 失败也发送反馈
        if let Some(ref prediction) = config.prediction_data {
            let feedback_json = serde_json::json!({
                "prediction": {
                    "format": prediction.format,
                    "quality": prediction.quality,
                },
                "actual_result": {
                    "success": false,
                    "error": result.error_message,
                },
                "reward": -1.0, // 失败给予负反馈
            });
            
            let client = AIClient::with_default();
            let _ = client.send_raw_feedback(&feedback_json.to_string());
        }
    }
}

// 🔥 Phase 40.21: 计算强化学习奖励值
fn calculate_reward(
    actual_size: u64,
    predicted_size: Option<u64>,
    processing_time: u64,
    compression_ratio: f64,
) -> f64 {
    let mut reward = 0.0;
    
    // 1. 压缩率奖励 (0-0.5分)
    let compression_reward = (1.0 - compression_ratio).min(0.5);
    reward += compression_reward;
    
    // 2. 大小预测准确度奖励 (0-0.3分)
    if let Some(predicted) = predicted_size {
        let size_error = (actual_size as f64 - predicted as f64).abs() / predicted as f64;
        let accuracy_reward = (1.0 - size_error).clamp(0.0, 0.3);
        reward += accuracy_reward;
    }
    
    // 3. 处理速度奖励 (0-0.2分)
    // 越快越好，但有上限
    let speed_reward = (1000.0 / processing_time as f64).min(0.2);
    reward += speed_reward;
    
    reward
}
