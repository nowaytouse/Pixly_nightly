/**
 * HTTP API 数据模型
 */
use serde::{Deserialize, Serialize};

/// 转换请求
#[derive(Debug, Deserialize, Clone)]
pub struct ConvertRequest {
    /// 输入文件路径
    pub input: String,
    
    /// 输出文件路径
    pub output: String,
    
    /// 目标格式 (avif, webp, png, jpeg, jxl, heic)
    pub format: String,
    
    /// 质量参数 (0-100, 默认85)
    #[serde(default = "default_quality")]
    pub quality: u8,
    
    /// 速度参数 (0-10, 默认4)
    #[serde(default = "default_speed")]
    pub speed: u8,
    
    /// 是否无损模式
    #[serde(default)]
    pub lossless: bool,
    
    /// 是否保留元数据
    #[serde(default = "default_true")]
    pub preserve_metadata: bool,
    
    /// 是否保留动画
    #[serde(default = "default_true")]
    pub keep_animated: bool,
    
    /// 🔥 Phase 46.8: 参数来源标记 ("user" | "ai" | "hybrid")
    #[serde(default)]
    pub params_source: Option<String>,
    
    /// 🔥 Phase 46.8: AI推荐的置信度 (0.0-1.0)
    #[serde(default)]
    pub ai_confidence: Option<f64>,
}

fn default_quality() -> u8 { 85 }
fn default_speed() -> u8 { 4 }
fn default_true() -> bool { true }

/// 转换响应
#[derive(Debug, Serialize)]
pub struct ConvertResponse {
    /// 是否成功
    pub success: bool,
    
    /// 错误信息（如果有）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    
    /// 输出路径
    #[serde(rename = "outputPath")]
    pub output_path: String,
    
    /// 文件大小（字节）
    #[serde(rename = "fileSize")]
    pub file_size: u64,
    
    /// 处理时间（毫秒）
    pub duration: u64,
    
    /// 使用的策略 (Native AVIF, CLI AVIF, 等)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    
    /// 压缩比 (0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_ratio: Option<f64>,
    
    /// 🔥 Phase 46.6: 实际使用的参数（参数回显）
    #[serde(skip_serializing_if = "Option::is_none", rename = "actualParams")]
    pub actual_params: Option<ActualParams>,
}

/// 🔥 Phase 46.6: 实际使用的转换参数
/// 
/// 用途：参数完整性验证
/// - UI发送的参数 vs Rust实际使用的参数
/// - 验证参数传递过程的完整性
/// - 增强系统可靠性和可信度
#[derive(Debug, Serialize, Clone)]
pub struct ActualParams {
    /// 实际使用的质量参数
    pub quality: u8,
    
    /// 实际使用的速度参数
    pub speed: u8,
    
    /// 实际使用的无损模式
    pub lossless: bool,
    
    /// 实际使用的格式
    pub format: String,
    
    /// 是否保留元数据
    pub preserve_metadata: bool,
    
    /// 是否保留动画
    pub keep_animated: bool,
    
    /// 使用的策略类型
    pub strategy_type: String,
    
    /// 参数来源 ("user" | "ai" | "hybrid")
    pub params_source: String,
    
    /// 🔥 Phase 46.8: AI置信度（如果是AI推荐）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_confidence: Option<f64>,
}

/// 健康检查响应
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// 状态
    pub status: String,
    
    /// Rust可用性
    pub rust_available: bool,
    
    /// 时间戳
    pub timestamp: u64,
    
    /// 可用策略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_strategies: Option<Vec<String>>,
    
    /// 支持格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_formats: Option<Vec<String>>,
}
