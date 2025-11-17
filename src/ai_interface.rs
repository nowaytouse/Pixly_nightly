// 🚀 统一AI参数预测接口
// 从 @archive/rust_broken/src/converter/unified_ai_interface.rs 提取并增强
//
// 核心功能:
// - 标准化AI预测请求/响应格式
// - 统一错误处理机制
// - 支持多种AI后端
// - 内置fallback和重试机制
// - Trait-based接口设计

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::collections::HashMap;

/// AI预测请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    pub target_format: String,
    pub image_width: u32,
    pub image_height: u32,
    pub file_size: u64,
    pub preferences: AIPreferences,
    pub request_id: String,
    pub timestamp: u64,
}

/// AI预测响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub quality: u8,
    pub speed: u8,
    pub lossless: bool,
    pub confidence: f32,
    pub model_version: String,
    pub prediction_time_ms: u64,
    pub timestamp: u64,
    pub metadata: HashMap<String, String>,
}

/// AI用户偏好
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AIPreferences {
    #[serde(default)]
    pub prefer_quality: bool,
    #[serde(default)]
    pub target_compression_ratio: Option<f32>,
    #[serde(default)]
    pub max_output_size: Option<u64>,
    #[serde(default)]
    pub custom_overrides: HashMap<String, String>,
}

/// AI错误类型
#[derive(Debug, thiserror::Error)]
pub enum AIError {
    #[error("AI服务不可用: {message}")]
    ServiceUnavailable { message: String },
    #[error("AI预测超时 (超过 {timeout_ms}ms)")]
    PredictionTimeout { timeout_ms: u64 },
    #[error("无效的图像数据: {details}")]
    InvalidImageData { details: String },
    #[error("不支持的格式: {format}")]
    UnsupportedFormat { format: String },
    #[error("AI预测失败: {reason}")]
    PredictionFailed { reason: String },
    #[error("网络连接错误: {details}")]
    NetworkError { details: String },
}

/// AI预测器trait
pub trait AIPredictor: Send + Sync {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    fn predict_parameters(&self, request: AIRequest) -> Result<AIResponse, AIError>;
    fn supported_formats(&self) -> Vec<String>;
    fn model_info(&self) -> HashMap<String, String>;
}

/// AI管理器
pub struct AIManager {
    predictors: Vec<Box<dyn AIPredictor>>,
    default_timeout: Duration,
    retry_count: u32,
}

impl AIManager {
    pub fn new() -> Self {
        Self {
            predictors: Vec::new(),
            default_timeout: Duration::from_secs(30),
            retry_count: 3,
        }
    }
    
    pub fn register_predictor(&mut self, predictor: Box<dyn AIPredictor>) {
        self.predictors.push(predictor);
    }
    
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }
    
    pub fn set_retry_count(&mut self, count: u32) {
        self.retry_count = count;
    }
    
    pub fn get_best_prediction(&self, request: AIRequest) -> Result<AIResponse, AIError> {
        if self.predictors.is_empty() {
            return Err(AIError::ServiceUnavailable { 
                message: "没有可用的AI预测器".to_string() 
            });
        }
        
        let mut last_error = None;
        
        for predictor in &self.predictors {
            if !predictor.is_available() {
                continue;
            }
            
            if !predictor.supported_formats().contains(&request.target_format) {
                continue;
            }
            
            for attempt in 1..=self.retry_count {
                match predictor.predict_parameters(request.clone()) {
                    Ok(response) => return Ok(response),
                    Err(e) => {
                        last_error = Some(e);
                        if attempt < self.retry_count {
                            std::thread::sleep(Duration::from_millis(100 * attempt as u64));
                        }
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or(AIError::ServiceUnavailable {
            message: "所有AI预测器都不可用".to_string()
        }))
    }
    
    pub fn get_available_predictors(&self) -> Vec<HashMap<String, String>> {
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

impl Default for AIManager {
    fn default() -> Self {
        Self::new()
    }
}

pub fn generate_request_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("ai_req_{}", timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct MockPredictor;
    
    impl AIPredictor for MockPredictor {
        fn name(&self) -> &str {
            "MockPredictor"
        }
        
        fn is_available(&self) -> bool {
            true
        }
        
        fn predict_parameters(&self, _request: AIRequest) -> Result<AIResponse, AIError> {
            Ok(AIResponse {
                quality: 85,
                speed: 4,
                lossless: false,
                confidence: 0.95,
                model_version: "1.0.0".to_string(),
                prediction_time_ms: 10,
                timestamp: 0,
                metadata: HashMap::new(),
            })
        }
        
        fn supported_formats(&self) -> Vec<String> {
            vec!["webp".to_string(), "avif".to_string()]
        }
        
        fn model_info(&self) -> HashMap<String, String> {
            let mut info = HashMap::new();
            info.insert("version".to_string(), "1.0.0".to_string());
            info
        }
    }
    
    #[test]
    fn test_ai_manager() {
        let mut manager = AIManager::new();
        manager.register_predictor(Box::new(MockPredictor));
        
        let request = AIRequest {
            target_format: "webp".to_string(),
            image_width: 1920,
            image_height: 1080,
            file_size: 1024000,
            preferences: AIPreferences::default(),
            request_id: generate_request_id(),
            timestamp: 0,
        };
        
        let result = manager.get_best_prediction(request);
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.quality, 85);
        assert_eq!(response.speed, 4);
    }
}
