/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * aAIparameterpredictioninterface (Unified AI Parameter Interface)
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 *
 * Phase UC-002: 代码a - AIparameterpredictioninterfacestandard
 *
 * 🎯 atarget:
 * - ✅ standardAIpredictionrequest/responseformat
 * - ✅ aerrorprocessmachine
 * - ✅ aAPIinterface
 * - ✅ supportmultitypeAIafterend (AI, externalAIetc)
 *
 * 🔧 design:
 * - Trait-basedinterface，supportmultitypeAIimplementation
 * - standardrequest/responsestructure
 * - aerrortype and process
 * - insidefallback and retrymachine
 */
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, warn};

use super::image_params::{ImageCharacteristics, OptimizedParams};

/// UnifiedAIpredictionrequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAIRequest {
/// targetformat (avif, jxl, webp, png, jpeg)
 pub target_format: String,
/// imagefeaturedata
 pub image_characteristics: ImageCharacteristics,
/// setting
 pub preferences: AIPreferences,
/// request ID (fortracking)
 pub request_id: String,
/// requesttime
 pub timestamp: u64,
}

/// UnifiedAIpredictionresponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAIResponse {
/// optimizationparameterresult
 pub optimized_params: OptimizedParams,
/// AIpredictionconfidence (0.0-1.0)
 pub confidence: f32,
/// AImodelversioninformation
 pub model_version: String,
/// prediction when ()
 pub prediction_time_ms: u64,
/// responsetime
 pub timestamp: u64,
/// outsideelementdata
 pub metadata: std::collections::HashMap<String, String>,
}

/// AIsetting
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AIPreferences {
/// qualityalsoisspeed
 pub prefer_quality: bool,
/// targetcompression (optional)
 pub target_compression_ratio: Option<f32>,
/// maximumfilesizelimit (bytes)
 pub max_output_size: Option<u64>,
/// customparameteroverride
 pub custom_overrides: std::collections::HashMap<String, String>,
}

/// UnifiedAIerrortype
#[derive(Debug, thiserror::Error)]
pub enum UnifiedAIError {
 #[error("AI service unavailable: {message}")]
 ServiceUnavailable { message: String },
 #[error("AI prediction timeout (exceeded {timeout_ms}ms)")]
 PredictionTimeout { timeout_ms: u64 },
 #[error("Invalid image feature data: {details}")]
 InvalidImageData { details: String },
 #[error("Unsupported target format: {format}")]
 UnsupportedFormat { format: String },
 #[error("AI prediction failed: {reason}")]
 PredictionFailed { reason: String },
 #[error("Network connection error: {details}")]
 NetworkError { details: String },
}

/// UnifiedAIpredictioninterface Trait
pub trait UnifiedAIPredictor: Send + Sync {
/// get AIpredictionname
 fn name(&self) -> &str;

/// check AIisnoavailable
 fn is_available(&self) -> bool;

/// getAIparameterprediction
/// 🔥 Performance: Takes reference to avoid cloning
 fn predict_parameters(&self, request: &UnifiedAIRequest) -> Result<UnifiedAIResponse, UnifiedAIError>;

/// getsupportformatlist
 fn supported_formats(&self) -> Vec<String>;

/// getAImodelinformation
 fn model_info(&self) -> std::collections::HashMap<String, String>;
}

/// AIprediction Manager
pub struct UnifiedAIManager {
 predictors: Vec<Box<dyn UnifiedAIPredictor>>,
 default_timeout: Duration,
 retry_count: u32,
}

impl UnifiedAIManager {
/// createnewAIManager
 pub fn new() -> Self {
 Self {
 predictors: Vec::new(),
 default_timeout: Duration::from_secs(30),
 retry_count: 3,
 }
 }

/// AIprediction
 pub fn register_predictor(&mut self, predictor: Box<dyn UnifiedAIPredictor>) {
 info!("Registering AI predictor: {}", predictor.name());
 self.predictors.push(predictor);
 }

/// settingdefaulttimeouttime
 pub fn set_timeout(&mut self, timeout: Duration) {
 self.default_timeout = timeout;
 }

/// settingretrycount
 pub fn set_retry_count(&mut self, count: u32) {
 self.retry_count = count;
 }

/// getmostavailable AIprediction
 pub fn get_best_prediction(&self, request: UnifiedAIRequest) -> Result<UnifiedAIResponse, UnifiedAIError> {
 if self.predictors.is_empty() {
 return Err(UnifiedAIError::ServiceUnavailable {
 message: "No available AI predictors".to_string()
 });
 }

 let mut last_error = None;

//  has prediction，findtofirstavailable
 for predictor in &self.predictors {
 if !predictor.is_available() {
 warn!("AI predictor {} is not available", predictor.name());
 continue;
 }

 if !predictor.supported_formats().contains(&request.target_format) {
 warn!("AI predictor {} does not support format {}", predictor.name(), request.target_format);
 continue;
 }

// tryprediction，retrymachine
// 🔥 Performance: Use reference to avoid cloning request in retry loop
 for attempt in 1..=self.retry_count {
 match predictor.predict_parameters(&request) {
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

/// get has availablepredictioninformation
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


/// generatearequest ID
pub fn generate_request_id() -> String {
 use std::time::{SystemTime, UNIX_EPOCH, Duration};
 let timestamp = SystemTime::now()
 .duration_since(UNIX_EPOCH)
 .unwrap_or(Duration::from_secs(0)) // notwillpanic
 .as_millis();
 format!("ai_req_{}", timestamp)
}

/// conversion Image Characteristicsfor Unified AIRequestfunction
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
 .unwrap_or_else(|_| std::time::Duration::from_secs(0))
 .as_secs(),
 }
}
