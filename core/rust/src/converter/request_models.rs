/**
 * 🔄 整合文件：统一转换请求/响应模型
 * 
 * 🔥 从@deprecated/server/models.rs提取价值：
 * - 标准化的转换请求结构
 * - 完整的响应模型
 * - AI参数验证结构
 * 
 * 用途：CLI、Python桥接、批量处理统一使用
 */

use serde::{Deserialize, Serialize};

/// 统一转换请求结构
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnifiedConvertRequest {
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
    #[serde(default = "default_preserve_metadata")]
    pub preserve_metadata: bool,
    
    /// 是否使用AI优化
    #[serde(default)]
    pub use_ai: bool,
    
    /// 自定义宽度（可选）
    #[serde(default)]
    pub width: Option<u32>,
    
    /// 自定义高度（可选）
    #[serde(default)]
    pub height: Option<u32>,
}

/// 转换响应
#[derive(Debug, Serialize, Clone)]
pub struct UnifiedConvertResponse {
    /// 是否成功
    pub success: bool,
    
    /// 错误消息
    pub error: Option<String>,
    
    /// 输入文件大小（字节）
    pub input_size: u64,
    
    /// 输出文件大小（字节）
    pub output_size: Option<u64>,
    
    /// 压缩比 (0.0-1.0)
    pub compression_ratio: Option<f64>,
    
    /// 处理时间（毫秒）
    pub processing_time_ms: u64,
    
    /// 使用的转换策略
    pub strategy_used: String,
    
    /// AI预测的参数（如果使用AI）
    pub ai_params: Option<ActualParams>,
    
    /// 质量验证结果
    pub validation: Option<QualityValidation>,
}

/// 健康检查响应
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// 服务状态
    pub status: String,
    
    /// 当前时间戳
    pub timestamp: u64,
    
    /// 可用的转换策略
    pub available_strategies: Option<Vec<String>>,
    
    /// 系统信息
    pub system_info: SystemInfo,
    
    /// 性能统计
    pub performance: PerformanceStats,
}

/// AI参数验证结果
#[derive(Debug, Serialize, Clone)]
pub struct ActualParams {
    /// 实际使用的质量值
    pub quality: u8,
    
    /// 实际使用的速度值
    pub speed: u8,
    
    /// 是否为无损模式
    pub lossless: bool,
    
    /// 预测置信度 (0.0-1.0)
    pub confidence: f32,
    
    /// AI模型版本
    pub model_version: String,
}

/// 质量验证结果
#[derive(Debug, Serialize, Clone)]
pub struct QualityValidation {
    /// SSIM值 (结构相似性)
    pub ssim: Option<f64>,
    
    /// PSNR值 (峰值信噪比)
    pub psnr: Option<f64>,
    
    /// 质量等级 (A+ to F)
    pub grade: String,
    
    /// 验证是否通过
    pub passed: bool,
}

/// 系统信息
#[derive(Debug, Serialize, Clone)]
pub struct SystemInfo {
    /// CPU核心数
    pub cpu_cores: usize,
    
    /// 可用内存 (MB)
    pub memory_mb: u64,
    
    /// 是否支持SIMD
    pub simd_support: bool,
    
    /// 是否支持GPU加速
    pub gpu_support: bool,
}

/// 性能统计
#[derive(Debug, Serialize, Clone)]
pub struct PerformanceStats {
    /// 总处理数量
    pub total_processed: u64,
    
    /// 成功处理数量
    pub successful: u64,
    
    /// 失败处理数量
    pub failed: u64,
    
    /// 平均处理时间 (ms)
    pub avg_processing_time_ms: f32,
    
    /// 总节省空间 (bytes)
    pub total_space_saved: u64,
}

// 默认值函数
fn default_quality() -> u8 { 85 }
fn default_speed() -> u8 { 4 }
fn default_preserve_metadata() -> bool { true }

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            cpu_cores: num_cpus::get(),
            memory_mb: 0, // 将通过实际检测填充
            simd_support: cfg!(target_feature = "sse2") || cfg!(target_feature = "neon"),
            gpu_support: false, // 待GPU模块完成后更新
        }
    }
}

impl Default for PerformanceStats {
    fn default() -> Self {
        Self {
            total_processed: 0,
            successful: 0,
            failed: 0,
            avg_processing_time_ms: 0.0,
            total_space_saved: 0,
        }
    }
}

impl UnifiedConvertRequest {
    /// 创建简单的转换请求
    pub fn simple(input: &str, output: &str, format: &str) -> Self {
        Self {
            input: input.to_string(),
            output: output.to_string(),
            format: format.to_string(),
            quality: default_quality(),
            speed: default_speed(),
            lossless: false,
            preserve_metadata: default_preserve_metadata(),
            use_ai: false,
            width: None,
            height: None,
        }
    }
    
    /// 创建AI优化的转换请求
    pub fn with_ai(input: &str, output: &str, format: &str) -> Self {
        let mut request = Self::simple(input, output, format);
        request.use_ai = true;
        request
    }
    
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.input.is_empty() {
            return Err("Input path cannot be empty".to_string());
        }
        
        if self.output.is_empty() {
            return Err("Output path cannot be empty".to_string());
        }
        
        if self.quality > 100 {
            return Err("Quality must be between 0-100".to_string());
        }
        
        if self.speed > 10 {
            return Err("Speed must be between 0-10".to_string());
        }
        
        let valid_formats = ["avif", "webp", "png", "jpeg", "jpg", "jxl", "heic"];
        if !valid_formats.contains(&self.format.to_lowercase().as_str()) {
            return Err(format!("Unsupported format: {}. Valid formats: {:?}", self.format, valid_formats));
        }
        
        Ok(())
    }
}
