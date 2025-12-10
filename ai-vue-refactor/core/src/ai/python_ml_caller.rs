/// Python MLcallmodule
///
/// Handles Rust ↔ Python ML Bridge process communication
/// Fully local, no network dependency
use std::process::Command;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use tracing::{info, warn, error};

/// Python MLpredictionrequest
#[derive(Serialize, Debug)]
pub struct MLPredictRequest {
 pub features: Vec<f64>, // 128dimensionfeature
 pub target_format: String,
 pub quality_mode: String,
}

/// Python MLpredictionresponse
#[derive(Deserialize, Debug)]
pub struct MLPredictResponse {
 pub quality: u8,
 pub effort: u8,
 pub lossless: bool,
 pub format_options: Vec<String>,
 pub confidence: f64,
 pub model_version: String,
}

/// Call Python ML Bridge for parameter prediction
///
/// # parameter
/// - `request`: ML prediction request（contains 128-dimensional features）
///
/// # return
/// - `Ok(MLPredictResponse)`: Python MLpredictionresult
/// - `Err`: Pythonexecutefailureorresponseparsefailure
///
/// # example
/// ```no_run
/// use pixly_kernel::ai::python_ml_caller::{call_python_ml, MLPredictRequest};
/// # fn main() -> anyhow::Result<()> {
/// let request = MLPredictRequest {
/// features: vec![1.0; 128],
/// target_format: "avif".to_string(),
/// quality_mode: "balanced".to_string(),
/// };
///
/// match call_python_ml(&request) {
/// Ok(response) => println!("Quality: {}", response.quality),
/// Err(e) => eprintln!("ML failed: {}", e),
/// }
/// # Ok(())
/// # }
/// ```
pub fn call_python_ml(request: &MLPredictRequest) -> Result<MLPredictResponse> {
 info!("Calling Python ML Bridge...");
 info!(" Format: {}, Mode: {}", request.target_format, request.quality_mode);

// 1. Serialize request to JSON
 let request_json = serde_json::to_string(request)
 .context("Failed to serialize ML request")?;

// 2. Call Python script
 let output = Command::new("python3")
 .arg("scripts/ml_bridge.py")
 .arg("--predict")
 .arg(&request_json)
 .output()
 .context("Failed to execute Python ML bridge (is python3 installed?)")?;

// 3. Check execution status
 if !output.status.success() {
 let stderr = String::from_utf8_lossy(&output.stderr);
 error!("Python ML execution failed: {}", stderr);
 anyhow::bail!("Python ML prediction failed: {}", stderr);
 }

// Capture and log Python stderr output (usually contains useful diagnostic info)
 let stderr = String::from_utf8_lossy(&output.stderr);
 if !stderr.is_empty() {
 for line in stderr.lines() {
 if !line.trim().is_empty() {
 info!(" Python: {}", line);
 }
 }
 }

// 4. Parse response (performance optimization: parse directly from bytes, avoid String allocation)
 let response: MLPredictResponse = serde_json::from_slice(&output.stdout)
 .context("Failed to parse Python ML response")?;

 info!("Python ML prediction received:");
 info!(" Quality: {}, Effort: {}, Lossless: {}",
 response.quality, response.effort, response.lossless);
 info!(" Confidence: {:.2}, Model: {}",
 response.confidence, response.model_version);

 Ok(response)
}

/// Check if Python ML Bridge is available
///
/// # Returns
/// - `true`: Both Python3 and ml_bridge.py are available
/// - `false`: Missing dependencies or script not found
pub fn is_python_ml_available() -> bool {
// Check if python3 exists
 let python_check = Command::new("python3")
 .arg("--version")
 .output();

 if python_check.is_err() {
 warn!("python3 not found in PATH");
 return false;
 }

// Check if ml_bridge.py exists
 let script_path = std::path::Path::new("scripts/ml_bridge.py");
 if !script_path.exists() {
 warn!("scripts/ml_bridge.py not found");
 return false;
 }

 info!("Python ML Bridge is available");
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
// This test may fail in CI environment，so only check for no panic
 let _ = is_python_ml_available();
 }
}


/// 🔥 Phase 1: Python ML caller（object-oriented encapsulation）
pub struct PythonMLCaller {
// canaddconfigurationfield
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

/// AI intelligent quality prediction
 pub fn predict_quality(&self, features: &[f64], optimize_mode: &str) -> Result<u8> {
 let request = MLPredictRequest {
 features: features.to_vec(),
 target_format: "auto".to_string(),
 quality_mode: optimize_mode.to_string(),
 };

 let response = call_python_ml(&request)?;
 Ok(response.quality)
 }

/// AI automatic parameter optimization
 pub fn optimize_params(&self, features: &[f64], optimize_mode: &str) -> Result<MLOptimizedParams> {
 let request = MLPredictRequest {
 features: features.to_vec(),
 target_format: "auto".to_string(),
 quality_mode: optimize_mode.to_string(),
 };

 let response = call_python_ml(&request)?;

 Ok(MLOptimizedParams {
 quality: response.quality,
 speed: response.effort,
 lossless: response.lossless,
 confidence: response.confidence,
 })
 }

/// AI intelligent preprocessing recommendations
 pub fn recommend_preprocess(&self, features: &[f64]) -> Result<PreprocessRecommendations> {
// Call Python script to get preprocessing recommendations
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

// Performance optimization: parse directly from bytes
 let recommendations: PreprocessRecommendations = serde_json::from_slice(&output.stdout)
 .context("Failed to parse preprocessing recommendations")?;

 Ok(recommendations)
 }
}

/// optimizedparameter
#[derive(Debug, Clone)]
pub struct MLOptimizedParams {
 pub quality: u8,
 pub speed: u8,
 pub lossless: bool,
 pub confidence: f64,
}

/// preprocessingrecommended
#[derive(Debug, Clone, Deserialize)]
pub struct PreprocessRecommendations {
 pub resize: Option<String>, // like: "1920x1080"
 pub quantize: Option<u8>, // colorcount
 pub sharpen: Option<f32>, // strongdegree
}
