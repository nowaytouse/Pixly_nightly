/// Python ML调用模块
/// 
/// 负责Rust ↔ Python ML Bridge的进程通信
/// 完全本地化，无网络依赖
use std::process::Command;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use tracing::{info, warn, error};

/// Python ML预测请求
#[derive(Serialize, Debug)]
pub struct MLPredictRequest {
    pub features: Vec<f64>,  // 128维特征向量
    pub target_format: String,
    pub quality_mode: String,
}

/// Python ML预测响应
#[derive(Deserialize, Debug)]
pub struct MLPredictResponse {
    pub quality: u8,
    pub effort: u8,
    pub lossless: bool,
    pub format_options: Vec<String>,
    pub confidence: f64,
    pub model_version: String,
}

/// 调用Python ML Bridge进行参数预测
/// 
/// # 参数
/// - `request`: ML预测请求（包含128维特征）
/// 
/// # 返回
/// - `Ok(MLPredictResponse)`: Python ML预测结果
/// - `Err`: Python执行失败或响应解析失败
/// 
/// # 示例
/// ```no_run
/// use pixly_kernel::python_ml_caller::{call_python_ml, MLPredictRequest};
/// # fn main() -> anyhow::Result<()> {
/// let request = MLPredictRequest {
///     features: vec![1.0; 128],
///     target_format: "avif".to_string(),
///     quality_mode: "balanced".to_string(),
/// };
/// 
/// match call_python_ml(&request) {
///     Ok(response) => println!("Quality: {}", response.quality),
///     Err(e) => eprintln!("ML failed: {}", e),
/// }
/// # Ok(())
/// # }
/// ```
pub fn call_python_ml(request: &MLPredictRequest) -> Result<MLPredictResponse> {
    info!("🐍 Calling Python ML Bridge...");
    info!("   Format: {}, Mode: {}", request.target_format, request.quality_mode);
    
    // 1. 序列化请求为JSON
    let request_json = serde_json::to_string(request)
        .context("Failed to serialize ML request")?;
    
    // 2. 调用Python脚本
    let output = Command::new("python3")
        .arg("scripts/ml_bridge.py")
        .arg("--predict")
        .arg(&request_json)
        .output()
        .context("Failed to execute Python ML bridge (is python3 installed?)")?;
    
    // 3. 检查执行状态
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("❌ Python ML execution failed: {}", stderr);
        anyhow::bail!("Python ML prediction failed: {}", stderr);
    }
    
    // 4. 解析响应 (🚀 性能优化: 直接从字节解析，避免String分配)
    let response: MLPredictResponse = serde_json::from_slice(&output.stdout)
        .context("Failed to parse Python ML response")?;
    
    info!("✅ Python ML prediction received:");
    info!("   Quality: {}, Effort: {}, Lossless: {}", 
        response.quality, response.effort, response.lossless);
    info!("   Confidence: {:.2}, Model: {}", 
        response.confidence, response.model_version);
    
    Ok(response)
}

/// 检查Python ML Bridge是否可用
/// 
/// # 返回
/// - `true`: Python3和ml_bridge.py都可用
/// - `false`: 缺少依赖或脚本不存在
pub fn is_python_ml_available() -> bool {
    // 检查python3是否存在
    let python_check = Command::new("python3")
        .arg("--version")
        .output();
    
    if python_check.is_err() {
        warn!("⚠️ python3 not found in PATH");
        return false;
    }
    
    // 检查ml_bridge.py是否存在
    let script_path = std::path::Path::new("scripts/ml_bridge.py");
    if !script_path.exists() {
        warn!("⚠️ scripts/ml_bridge.py not found");
        return false;
    }
    
    info!("✅ Python ML Bridge is available");
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ml_request_serialization() {
        let request = MLPredictRequest {
            features: vec![1.0; 128],
            target_format: "avif".to_string(),
            quality_mode: "balanced".to_string(),
        };
        
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("avif"));
        assert!(json.contains("balanced"));
    }

    #[test]
    fn test_ml_response_deserialization() {
        let json = r#"{
            "quality": 75,
            "effort": 6,
            "lossless": false,
            "format_options": ["--speed=6"],
            "confidence": 0.85,
            "model_version": "python-ml-v1.0"
        }"#;
        
        let response: MLPredictResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.quality, 75);
        assert_eq!(response.effort, 6);
        assert!(!response.lossless);
        assert_eq!(response.confidence, 0.85);
    }

    #[test]
    fn test_python_availability_check() {
        // 这个测试可能在CI环境中失败，所以只是检查不panic
        let _ = is_python_ml_available();
    }
}


/// 🔥 Phase 1: Python ML调用器（面向对象封装）
pub struct PythonMLCaller {
    // 可以添加配置字段
}

impl Default for PythonMLCaller {
    fn default() -> Self {
        Self::new()
    }
}

impl PythonMLCaller {
    pub fn new() -> Self {
        Self {}
    }
    
    /// 🔥 AI智能质量预测
    pub fn predict_quality(&self, features: &[f64], optimize_mode: &str) -> Result<u8> {
        let request = MLPredictRequest {
            features: features.to_vec(),
            target_format: "auto".to_string(),
            quality_mode: optimize_mode.to_string(),
        };
        
        let response = call_python_ml(&request)?;
        Ok(response.quality)
    }
    
    /// 🔥 AI自动参数优化
    pub fn optimize_params(&self, features: &[f64], optimize_mode: &str) -> Result<OptimizedParams> {
        let request = MLPredictRequest {
            features: features.to_vec(),
            target_format: "auto".to_string(),
            quality_mode: optimize_mode.to_string(),
        };
        
        let response = call_python_ml(&request)?;
        
        Ok(OptimizedParams {
            quality: response.quality,
            speed: response.effort,
            lossless: response.lossless,
            confidence: response.confidence,
        })
    }
    
    /// 🔥 AI智能预处理推荐
    pub fn recommend_preprocess(&self, features: &[f64]) -> Result<PreprocessRecommendations> {
        // 调用Python脚本获取预处理推荐
        let features_json = serde_json::to_string(features)?;
        
        let output = Command::new("python3")
            .arg("scripts/ml_bridge.py")
            .arg("--recommend-preprocess")
            .arg(&features_json)
            .output()
            .context("Failed to execute Python ML bridge for preprocessing")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Python preprocessing recommendation failed: {}", stderr);
        }
        
        // 🚀 性能优化: 直接从字节解析
        let recommendations: PreprocessRecommendations = serde_json::from_slice(&output.stdout)
            .context("Failed to parse preprocessing recommendations")?;
        
        Ok(recommendations)
    }
}

/// 优化后的参数
#[derive(Debug, Clone)]
pub struct OptimizedParams {
    pub quality: u8,
    pub speed: u8,
    pub lossless: bool,
    pub confidence: f64,
}

/// 预处理推荐
#[derive(Debug, Clone, Deserialize)]
pub struct PreprocessRecommendations {
    pub resize: Option<String>,      // 例如: "1920x1080"
    pub quantize: Option<u8>,        // 颜色数量
    pub sharpen: Option<f32>,        // 锐化强度
}
