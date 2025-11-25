/**
 * MLlayer - Pythontraining ↔ Rustinference
 *
 * afeature and data
 */
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// featureextractionfunctiontype
type FeatureExtractorFn = Box<dyn Fn(&image::DynamicImage) -> StandardFeatures + Send + Sync>;

/// standardizefeature (128dimensional)
/// and Pythontrainingkeepfullya
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StandardFeatures {
/// basicfeature (16dimensional) - imageproperty
 pub basic: [f64; 16],

/// colorfeature (16dimensional) - color and complexity
 pub color: [f64; 16],

/// texturefeature (16dimensional) - edge and textureinformation
 pub texture: [f64; 16],

/// feature (16dimensional) - how manywhat and structure
 pub shape: [f64; 16],

/// qualityfeature (16dimensional) - noise and cleardegree
 pub quality: [f64; 16],

/// elementdatafeature (32dimensional) - EXIF and fileproperty
 pub metadata: [f64; 32],

/// contextfeature (16dimensional) - processinghistorical and environment
 pub context: [f64; 16],
}

impl StandardFeatures {
/// conversionfor128dimensional (and Pythona)
 pub fn to_vector(&self) -> Vec<f64> {
 let mut vec = Vec::with_capacity(128);
 vec.extend_from_slice(&self.basic);
 vec.extend_from_slice(&self.color);
 vec.extend_from_slice(&self.texture);
 vec.extend_from_slice(&self.shape);
 vec.extend_from_slice(&self.quality);
 vec.extend_from_slice(&self.metadata);
 vec.extend_from_slice(&self.context);
 vec
 }

/// from128dimensionalcreate (Pythonoutput)
 pub fn from_vector(vec: &[f64]) -> Result<Self, String> {
 if vec.len() != 128 {
 return Err(format!("Invalid feature vector length: {} (expected 128)", vec.len()));
 }

// security：lengthalreadyvalidationfor128
 Ok(Self {
 basic: vec[0..16].try_into().expect("slice length verified"),
 color: vec[16..32].try_into().expect("slice length verified"),
 texture: vec[32..48].try_into().expect("slice length verified"),
 shape: vec[48..64].try_into().expect("slice length verified"),
 quality: vec[64..80].try_into().expect("slice length verified"),
 metadata: vec[80..112].try_into().expect("slice length verified"),
 context: vec[112..128].try_into().expect("slice length verified"),
 })
 }

/// conversionfor JSON (Pythoncommunication)
 pub fn to_json(&self) -> Result<String, String> {
 serde_json::to_string(self).map_err(|e| e.to_string())
 }

/// from JSONcreate (Pythoncommunication)
 pub fn from_json(json: &str) -> Result<Self, String> {
 serde_json::from_str(json).map_err(|e| e.to_string())
 }
}

/// standardizepredictionresult
/// and Pythonmodeloutputkeepa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardPrediction {
/// recommendedquality (0-100)
 pub quality: u32,

/// recommendedspeed/effort (0-9)
 pub effort: u32,

/// isnorecommendedlossless
 pub lossless: bool,

/// recommendedformat
 pub format: String,

/// predictionconfidence (0-1)
 pub confidence: f64,

/// filesize (bytes)
 pub estimated_size: u64,

/// quality (SSIM)
 pub estimated_quality: f64,

/// modelversion
 pub model_version: String,
}

/// trainingsample (forfeedback)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSample {
/// feature
 pub features: StandardFeatures,

/// actualuseparameter
 pub actual_quality: u32,
 pub actual_effort: u32,
 pub actual_lossless: bool,
 pub actual_format: String,

/// actualresult
 pub result_size: u64,
 pub result_quality: f64, // SSIM
 pub processing_time: f64,

/// feedback (optional)
 pub user_rating: Option<f64>,

/// time
 pub timestamp: i64,
}

impl TrainingSample {
/// conversionforPythontrainingformat
///
/// 🔥 usesecurityserialize， not will panic
 pub fn to_training_format(&self) -> HashMap<String, serde_json::Value> {
 let mut data = HashMap::new();

// Safe serialization macro - logs warning and skips on failure
 macro_rules! safe_insert {
 ($key:expr, $value:expr) => {
 match serde_json::to_value($value) {
 Ok(v) => { data.insert($key.to_string(), v); },
 Err(e) => {
 log::warn!("Failed to serialize {}: {}", $key, e);
 }
 }
 };
 }

// feature
 safe_insert!("features", self.features.to_vector());

// label
 safe_insert!("quality", self.actual_quality);
 safe_insert!("effort", self.actual_effort);
 safe_insert!("lossless", self.actual_lossless);
 safe_insert!("format", &self.actual_format);

// result
 safe_insert!("result_size", self.result_size);
 safe_insert!("result_quality", self.result_quality);
 safe_insert!("processing_time", self.processing_time);

 if let Some(rating) = self.user_rating {
 safe_insert!("user_rating", rating);
 }

 safe_insert!("timestamp", self.timestamp);

 data
 }
}

/// MLbridge - Unified Python and Rust
pub struct MLBridge {
/// modelversion
 #[allow(dead_code)] // fornot yetversioncheck
 model_version: String,

/// featureextraction
 feature_extractor: Option<FeatureExtractorFn>,
}

impl MLBridge {
/// createnewbridge
 pub fn new(model_version: String) -> Self {
 Self {
 model_version,
 feature_extractor: None,
 }
 }

/// settingfeatureextraction
 pub fn with_feature_extractor<F>(mut self, extractor: F) -> Self
 where
 F: Fn(&image::DynamicImage) -> StandardFeatures + Send + Sync + 'static
 {
 self.feature_extractor = Some(Box::new(extractor));
 self
 }

/// extractionstandardizefeature
 pub fn extract_features(&self, img: &image::DynamicImage) -> StandardFeatures {
 if let Some(extractor) = &self.feature_extractor {
 extractor(img)
 } else {
// defaultfeatureextraction
 self.default_feature_extraction(img)
 }
 }

/// defaultfeatureextraction
 fn default_feature_extraction(&self, img: &image::DynamicImage) -> StandardFeatures {
// usefeature_extractor_128dfunctionAPI
 use crate::core::feature_extractor_128d::extract_128d_features;

// createbasicfeature
 let basic_features = crate::ImageFeatures {
 width: img.width(),
 height: img.height(),
 file_size: 0, // not yet
 format: "unknown".to_string(),
 has_alpha: img.color().has_alpha(),
 is_animated: false,
 complexity: 0.5, // defaultvalue
 };

 let features_vec = extract_128d_features(img, std::path::Path::new(""), &basic_features);

// Safe feature conversion - returns default on failure
 StandardFeatures::from_vector(&features_vec)
 .unwrap_or_else(|e| {
 log::error!("Failed to create StandardFeatures from vector: {}", e);
 log::warn!("Using default StandardFeatures as fallback");
 StandardFeatures::default()
 })
 }

/// savetrainingsampleto JSON (Pythontraininguse)
 pub fn save_training_sample(&self, sample: &TrainingSample, path: &str) -> Result<(), String> {
 let json = serde_json::to_string_pretty(&sample.to_training_format())
 .map_err(|e| e.to_string())?;
 std::fs::write(path, json).map_err(|e| e.to_string())?;
 Ok(())
 }

/// batchsavetrainingsample
 pub fn save_training_batch(&self, samples: &[TrainingSample], path: &str) -> Result<(), String> {
 let batch: Vec<_> = samples.iter()
 .map(|s| s.to_training_format())
 .collect();

 let json = serde_json::to_string_pretty(&batch)
 .map_err(|e| e.to_string())?;
 std::fs::write(path, json).map_err(|e| e.to_string())?;
 Ok(())
 }

/// loadPythonmodelpredictionresult
 pub fn load_prediction(&self, json: &str) -> Result<StandardPrediction, String> {
 serde_json::from_str(json).map_err(|e| e.to_string())
 }

/// validationfeaturea性
 pub fn validate_features(&self, features: &StandardFeatures) -> Result<(), String> {
 let vec = features.to_vector();

// Check dimension
 if vec.len() != 128 {
 return Err(format!("Invalid feature dimension: {}", vec.len()));
 }

// Check NaN
 if vec.iter().any(|&x| x.is_nan()) {
 return Err("Features contain NaN".to_string());
 }

// Check Inf
 if vec.iter().any(|&x| x.is_infinite()) {
 return Err("Features contain Inf".to_string());
 }

 Ok(())
 }
}

impl Default for MLBridge {
 fn default() -> Self {
 Self::new("1.0.0".to_string())
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_standard_features_vector_conversion() {
 let features = StandardFeatures {
 basic: [1.0; 16],
 color: [2.0; 16],
 texture: [3.0; 16],
 shape: [4.0; 16],
 quality: [5.0; 16],
 metadata: [6.0; 32],
 context: [7.0; 16],
 };

 let vec = features.to_vector();
 assert_eq!(vec.len(), 128);
 assert_eq!(vec[0], 1.0);
 assert_eq!(vec[16], 2.0);
 assert_eq!(vec[112], 7.0);
 }

 #[test]
 fn test_features_json_serialization() {
 let features = StandardFeatures {
 basic: [1.0; 16],
 color: [2.0; 16],
 texture: [3.0; 16],
 shape: [4.0; 16],
 quality: [5.0; 16],
 metadata: [6.0; 32],
 context: [7.0; 16],
 };

 let json = features.to_json().unwrap();
 let restored = StandardFeatures::from_json(&json).unwrap();

 assert_eq!(restored.basic[0], 1.0);
 assert_eq!(restored.color[0], 2.0);
 }

 #[test]
 fn test_training_sample_format() {
 let features = StandardFeatures {
 basic: [1.0; 16],
 color: [2.0; 16],
 texture: [3.0; 16],
 shape: [4.0; 16],
 quality: [5.0; 16],
 metadata: [6.0; 32],
 context: [7.0; 16],
 };

 let sample = TrainingSample {
 features,
 actual_quality: 75,
 actual_effort: 6,
 actual_lossless: false,
 actual_format: "avif".to_string(),
 result_size: 50000,
 result_quality: 0.96,
 processing_time: 1.5,
 user_rating: Some(4.5),
 timestamp: 1700000000,
 };

 let format = sample.to_training_format();
 assert!(format.contains_key("features"));
 assert!(format.contains_key("quality"));
 assert!(format.contains_key("result_size"));
 }

 #[test]
 fn test_ml_bridge_validation() {
 let bridge = MLBridge::default();

 let features = StandardFeatures {
 basic: [1.0; 16],
 color: [2.0; 16],
 texture: [3.0; 16],
 shape: [4.0; 16],
 quality: [5.0; 16],
 metadata: [6.0; 32],
 context: [7.0; 16],
 };

 assert!(bridge.validate_features(&features).is_ok());
 }
}
