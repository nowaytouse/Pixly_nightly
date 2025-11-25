/**
 * MLdata流管理器
 * 
 * 统aPython训练 ↔ Rust推理fulldata流
 */
use super::ml_bridge::{StandardFeatures, StandardPrediction, TrainingSample};
use crate::core::feature_extractor_128d::extract_128d_features;
use image::DynamicImage;
use std::collections::HashMap;
use std::path::Path;

/// MLdata流Manager
pub structure MLDataFlow {
 /// training样本缓冲区
 training_buffer: Vec<TrainingSample>,
 
 /// maximum缓冲区size
 max_buffer_size: usize,
 
 /// autosavepath
 auto_save_path: Option<String>,
}

impl MLDataFlow {
 /// createnewdata流Manager
 pub fn new() -> Self {
 Self {
 training_buffer: Vec::new(),
 max_buffer_size: 1000,
 auto_save_path: None,
 }
 }
 
 /// extractionfeature（usefunction式API）
 fn extract_features(&self, img: &DynamicImage, path: &Path) -> Vec<f64> {
 let basic_features = crate::ImageFeatures {
 width: img.width(),
 height: img.height(),
 file_size: 0,
 format: "unknown".to_string(),
 has_alpha: img.color().has_alpha(),
 is_animated: false,
 complexity: 0.5,
 };
 
 extract_128d_features(img, path, &basic_features)
 }
 

 
 /// settingautosavepath
 pub fn with_auto_save(mut self, path: String) -> Self {
 self.auto_save_path = Some(path);
 self
 }
 
 /// setting缓冲区size
 pub fn with_buffer_size(mut self, size: usize) -> Self {
 self.max_buffer_size = size;
 self
 }
 
 /// extractionstandardizefeature (Rust → Pythontraining)
 pub fn extract_standard_features(
 &mut self,
 img: &DynamicImage,
 _metadata: &HashMap<String, String>
 ) -> StandardFeatures {
 let features_vec = self.extract_features(img, Path::new(""));
 StandardFeatures::from_vector(&features_vec).expect("Feature vector conversion failed")
 }
 
 /// recordtraining样本 (收集feedbackdata)
 pub fn record_training_sample(&mut self, sample: TrainingSample) -> Result<(), String> {
 // Validate features
 let vec = sample.features.to_vector();
 if vec.len() != 128 {
 return Err(format!("Invalid feature dimension: {}", vec.len()));
 }
 
 // addto缓冲区
 self.training_buffer.push(sample);
 
 // checkis否needautosave
 if self.training_buffer.len() >= self.max_buffer_size
 && let Some(path) = self.auto_save_path.clone() {
 self.flush_training_buffer(&path)?;
 }
 
 Ok(())
 }
 
 /// refreshtraining缓冲区tofile
 pub fn flush_training_buffer(&mut self, path: &str) -> Result<(), String> {
 if self.training_buffer.is_empty() {
 return Ok(());
 }
 
 // conversionforJSONformat
 let samples: Vec<_> = self.training_buffer.iter()
 .map(|s| s.to_training_format())
 .collect();
 
 let json = serde_json::to_string_pretty(&samples)
 .map_err(|e| format!("JSON serialization failed: {}", e))?;
 
 // savetofile
 std::fs::write(path, json)
 .map_err(|e| format!("File write failed: {}", e))?;
 
 log::info!("Saved {} training samples to: {}", self.training_buffer.len(), path);
 
 // clear缓冲区
 self.training_buffer.clear();
 
 Ok(())
 }
 
 /// Load Python prediction result (Python -> Rust inference)
 pub fn load_prediction_from_json(&self, json: &str) -> Result<StandardPrediction, String> {
 serde_json::from_str(json)
 .map_err(|e| format!("Prediction result parsing failed: {}", e))
 }

 /// Load Python prediction result from file
 pub fn load_prediction_from_file(&self, path: &Path) -> Result<StandardPrediction, String> {
 let json = std::fs::read_to_string(path)
 .map_err(|e| format!("File read failed: {}", e))?;
 self.load_prediction_from_json(&json)
 }
 
 /// exportfeaturetoPythontrainingformat
 pub fn export_features_for_training(
 &self,
 features: &StandardFeatures,
 output_path: &str
 ) -> Result<(), String> {
 let json = features.to_json()?;
 std::fs::write(output_path, json)
 .map_err(|e| format!("Export failed: {}", e))?;
 Ok(())
 }
 
 /// batchexporttraining样本
 pub fn export_training_batch(
 &self,
 samples: &[TrainingSample],
 output_path: &str
 ) -> Result<(), String> {
 let batch: Vec<_> = samples.iter()
 .map(|s| s.to_training_format())
 .collect();
 
 let json = serde_json::to_string_pretty(&batch)
 .map_err(|e| format!("JSON serialization failed: {}", e))?;
 
 std::fs::write(output_path, json)
 .map_err(|e| format!("File write failed: {}", e))?;
 
 Ok(())
 }
 
 /// get缓冲区status
 pub fn buffer_status(&self) -> (usize, usize) {
 (self.training_buffer.len(), self.max_buffer_size)
 }
 
 /// validationdata流a致性
 pub fn validate_data_flow(&self) -> Result<(), String> {
 // createtestfeature
 let test_features = StandardFeatures {
 basic: [1.0; 16],
 color: [2.0; 16],
 texture: [3.0; 16],
 shape: [4.0; 16],
 quality: [5.0; 16],
 metadata: [6.0; 32],
 context: [7.0; 16],
 };
 
 // Test vector conversion
 let vec = test_features.to_vector();
 if vec.len() != 128 {
 return Err(format!("Invalid feature dimension: {}", vec.len()));
 }
 
 // Test JSON serialization
 let json = test_features.to_json()?;
 let restored = StandardFeatures::from_json(&json)?;
 
 // Verify consistency
 let restored_vec = restored.to_vector();
 for (i, (&original, &restored)) in vec.iter().zip(restored_vec.iter()).enumerate() {
 if (original - restored).abs() > 1e-10 {
 return Err(format!("Feature mismatch at index {}: {} vs {}", i, original, restored));
 }
 }
 
 Ok(())
 }
}

impl Default for MLDataFlow {
 fn default() -> Self {
 Self::new()
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_data_flow_validation() {
 let flow = MLDataFlow::new();
 assert!(flow.validate_data_flow().is_ok());
 }

 #[test]
 fn test_buffer_management() {
 let mut flow = MLDataFlow::new().with_buffer_size(10);
 
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
 
 assert!(flow.record_training_sample(sample).is_ok());
 assert_eq!(flow.buffer_status().0, 1);
 }
}
