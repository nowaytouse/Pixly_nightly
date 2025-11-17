/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 统一AI参数预测接口 (Unified AI Parameter Interface)
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 
 * Phase UC-002: 代码统一 - AI参数预测接口标准化
 * 
 * 🎯 统一目标:
 * - ✅ 标准化AI预测请求/响应格式
 * - ✅ 统一错误处理机制
 * - ✅ 提供一致的API接口
 * - ✅ 支持多种AI后端 (本地AI, 外部AI服务等)
 * 
 * 🔧 架构设计:
 * - Trait-based接口，支持多种AI实现
 * - 标准化的请求/响应结构
 * - 统一的错误类型和处理
 * - 内置fallback和重试机制
 */
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, warn, error};

use crate::image_params::{ImageCharacteristics, OptimizedParams};

/// 统一AI预测请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAIRequest {
    /// 目标格式 (avif, jxl, webp, png, jpeg)
    pub target_format: String,
    /// 图像特征数据
    pub image_characteristics: ImageCharacteristics,
    /// 用户偏好设置
    pub preferences: AIPreferences,
    /// 请求ID (用于追踪)
    pub request_id: String,
    /// 请求时间戳
    pub timestamp: u64,
}

/// 统一AI预测响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAIResponse {
    /// 优化参数结果
    pub optimized_params: OptimizedParams,
    /// AI预测置信度 (0.0-1.0)
    pub confidence: f32,
    /// AI模型版本信息
    pub model_version: String,
    /// 预测耗时 (毫秒)
    pub prediction_time_ms: u64,
    /// 响应时间戳
    pub timestamp: u64,
    /// 额外元数据
    pub metadata: std::collections::HashMap<String, String>,
}

/// AI用户偏好设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIPreferences {
    /// 偏好质量还是速度
    pub prefer_quality: bool,
    /// 目标压缩比 (可选)
    pub target_compression_ratio: Option<f32>,
    /// 最大文件大小限制 (字节)
    pub max_output_size: Option<u64>,
    /// 自定义参数覆盖
    pub custom_overrides: std::collections::HashMap<String, String>,
}

/// 统一AI错误类型
#[derive(Debug, thiserror::Error)]
pub enum UnifiedAIError {
    #[error("AI服务不可用: {message}")]
    ServiceUnavailable { message: String },
    #[error("AI预测超时 (超过 {timeout_ms}ms)")]
    PredictionTimeout { timeout_ms: u64 },
    #[error("无效的图像特征数据: {details}")]
    InvalidImageData { details: String },
    #[error("Unsupported target format: {format}")]
    UnsupportedFormat { format: String },
    #[error("AI prediction failed: {reason}")]
    PredictionFailed { reason: String },
    #[error("Network connection error: {details}")]
    NetworkError { details: String },
}

/// 统一AI预测接口 Trait
pub trait UnifiedAIPredictor: Send + Sync {
    /// 获取AI预测器名称
    fn name(&self) -> &str;
    
    /// 检查AI服务是否可用
    fn is_available(&self) -> bool;
    
    /// 获取AI参数预测
    fn predict_parameters(&self, request: UnifiedAIRequest) -> Result<UnifiedAIResponse, UnifiedAIError>;
    
    /// 获取支持的格式列表
    fn supported_formats(&self) -> Vec<String>;
    
    /// 获取AI模型信息
    fn model_info(&self) -> std::collections::HashMap<String, String>;
}

/// AI预测器管理器
pub struct UnifiedAIManager {
    predictors: Vec<Box<dyn UnifiedAIPredictor>>,
    default_timeout: Duration,
    retry_count: u32,
}

impl UnifiedAIManager {
    /// 创建新的AI管理器
    pub fn new() -> Self {
        Self {
            predictors: Vec::new(),
            default_timeout: Duration::from_secs(30),
            retry_count: 3,
        }
    }
    
    /// 注册AI预测器
    pub fn register_predictor(&mut self, predictor: Box<dyn UnifiedAIPredictor>) {
        info!("Registering AI predictor: {}", predictor.name());
        self.predictors.push(predictor);
    }
    
    /// 设置默认超时时间
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }
    
    /// 设置重试次数
    pub fn set_retry_count(&mut self, count: u32) {
        self.retry_count = count;
    }
    
    /// 获取最佳可用的AI预测
    pub fn get_best_prediction(&self, request: UnifiedAIRequest) -> Result<UnifiedAIResponse, UnifiedAIError> {
        if self.predictors.is_empty() {
            return Err(UnifiedAIError::ServiceUnavailable { 
                message: "没有可用的AI预测器".to_string() 
            });
        }
        
        let mut last_error = None;
        
        // 遍历所有预测器，找到第一个可用的
        for predictor in &self.predictors {
            if !predictor.is_available() {
                warn!("AI predictor {} is not available", predictor.name());
                continue;
            }
            
            if !predictor.supported_formats().contains(&request.target_format) {
                warn!("AI predictor {} does not support format {}", predictor.name(), request.target_format);
                continue;
            }
            
            // 尝试预测，带重试机制
            for attempt in 1..=self.retry_count {
                match predictor.predict_parameters(request.clone()) {
                    Ok(response) => {
                        info!("AI prediction succeeded - Predictor: {}, Attempt: {}/{}", 
                              predictor.name(), attempt, self.retry_count);
                        return Ok(response);
                    }
                    Err(e) => {
                        warn!("AI prediction failed - Predictor: {}, Attempt: {}/{}, Error: {}", 
                              predictor.name(), attempt, self.retry_count, e);
                        last_error = Some(e);
                        
                        if attempt < self.retry_count {
                            std::thread::sleep(Duration::from_millis(100 * attempt as u64));
                        }
                    }
                }
            }
        }
        
        // If all predictors fail, return the last error
        Err(last_error.unwrap_or(UnifiedAIError::ServiceUnavailable {
            message: "All AI predictors are unavailable".to_string()
        }))
    }
    
    /// 获取所有可用预测器的信息
    pub fn get_available_predictors(&self) -> Vec<std::collections::HashMap<String, String>> {
        self.predictors
            .iter()
            .filter(|p| p.is_available())
            .map(|p| {
                let mut info = p.model_info();
                info.insert("name".to_string(), p.name().to_string());
                info.insert("supported_formats".to_string(), p.supported_formats().join(","));
                info
            })
            .collect()
    }
}

impl Default for UnifiedAIManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AIPreferences {
    fn default() -> Self {
        Self {
            prefer_quality: false,
            target_compression_ratio: None,
            max_output_size: None,
            custom_overrides: std::collections::HashMap::new(),
        }
    }
}

/// 生成唯一请求ID
pub fn generate_request_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))  // 不会panic
        .as_millis();
    format!("ai_req_{}", timestamp)
}

/// 转换ImageCharacteristics为UnifiedAIRequest的便捷函数
pub fn create_ai_request(
    target_format: &str,
    chars: ImageCharacteristics,
    preferences: Option<AIPreferences>
) -> UnifiedAIRequest {
    UnifiedAIRequest {
        target_format: target_format.to_string(),
        image_characteristics: chars,
        preferences: preferences.unwrap_or_default(),
        request_id: generate_request_id(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }
}
