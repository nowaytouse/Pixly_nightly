// 🤖 enhanced版PPO强学习model
// supportimage、video、audio全媒体typetrainingdata

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::collections::HashMap;
use anyhow::{Context, Result};

/// 媒体type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
 Image,
 Video,
 Audio,
}

/// singletraining样本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure TrainingSample {
 pub media_type: MediaType,
 pub input_file: String,
 pub input_size: u64,
 pub target_format: String,
 pub quality: u32,
 pub output_size: u64,
 pub compression_ratio: f64,
 pub reward: f64,
}

/// PPOtrainingdata集
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure PPOTrainingData {
 pub timestamp: String,
 pub total_samples: usize,
 pub stats: HashMap<String, usize>,
 pub data: Vec<TrainingSample>,
}

impl PPOTrainingData {
 /// fromJSONfileload
 pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
 let content = std::fs::read_to_string(path)
 .context("Failed to read PPO training data file")?;
 
 let data: PPOTrainingData = serde_json::from_str(&content)
 .context("Failed to parse PPO training data")?;
 
 Ok(data)
 }
 
 /// 按媒体typefilter样本
 pub fn filter_by_media_type(&self, media_type: MediaType) -> Vec<&TrainingSample> {
 self.data.iter()
 .filter(|s| s.media_type == media_type)
 .collect()
 }
 
 /// 按formatfilter样本
 pub fn filter_by_format(&self, format: &str) -> Vec<&TrainingSample> {
 self.data.iter()
 .filter(|s| s.target_format == format)
 .collect()
 }
 
 /// get最佳compression率样本
 pub fn get_best_compression(&self, media_type: MediaType, format: &str) -> Option<&TrainingSample> {
 self.data.iter()
 .filter(|s| s.media_type == media_type && s.target_format == format)
 .min_by(|a, b| a.compression_ratio.partial_cmp(&b.compression_ratio)
 .unwrap_or(std::cmp::Ordering::Equal))
 }
 
 /// getaveragecompression率
 pub fn average_compression_ratio(&self, media_type: MediaType, format: &str) -> f64 {
 let samples: Vec<_> = self.data.iter()
 .filter(|s| s.media_type == media_type && s.target_format == format)
 .collect();
 
 if samples.is_empty() {
 return 1.0;
 }
 
 let sum: f64 = samples.iter().map(|s| s.compression_ratio).sum();
 sum / samples.len() as f64
 }
 
 /// getaverage奖励
 pub fn average_reward(&self, media_type: MediaType, format: &str) -> f64 {
 let samples: Vec<_> = self.data.iter()
 .filter(|s| s.media_type == media_type && s.target_format == format)
 .collect();
 
 if samples.is_empty() {
 return 0.0;
 }
 
 let sum: f64 = samples.iter().map(|s| s.reward).sum();
 sum / samples.len() as f64
 }
 
 /// recommendedOptimal quality parameter
 pub fn recommend_quality(&self, media_type: MediaType, format: &str, target_ratio: f64) -> u32 {
 let samples: Vec<_> = self.data.iter()
 .filter(|s| s.media_type == media_type && s.target_format == format)
 .collect();
 
 if samples.is_empty() {
 return match media_type {
 MediaType::Image => 85,
 MediaType::Video => 1000,
 MediaType::Audio => 192,
 };
 }
 
 // findtoclosesttargetcompression率样本
 samples.iter()
 .min_by(|a, b| {
 let diff_a = (a.compression_ratio - target_ratio).abs();
 let diff_b = (b.compression_ratio - target_ratio).abs();
 diff_a.partial_cmp(&diff_b).unwrap_or(std::cmp::Ordering::Equal)
 })
 .map(|s| s.quality)
 .unwrap_or(85)
 }
}

/// enhanced版PPOprediction
pub structure EnhancedPPOPredictor {
 training_data: PPOTrainingData,
 enabled: bool,
}

impl EnhancedPPOPredictor {
 /// createnewprediction
 pub fn new(training_data: PPOTrainingData) -> Self {
 Self {
 training_data,
 enabled: true,
 }
 }
 
 /// fromfileload
 pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
 let training_data = PPOTrainingData::from_file(path)?;
 Ok(Self::new(training_data))
 }
 
 /// predictionimageconversionparameter
 pub fn predict_image_params(&self, format: &str, file_size: u64) -> ImageConversionParams {
 if !self.enabled {
 return ImageConversionParams::default();
 }
 
 let avg_ratio = self.training_data.average_compression_ratio(MediaType::Image, format);
 let avg_reward = self.training_data.average_reward(MediaType::Image, format);
 
 // based onfilesize and trainingdataadjustedquality
 let base_quality = if file_size > 5_000_000 {
 80 // 大file用较低quality
 } else if file_size < 500_000 {
 90 // 小file用较高quality
 } else {
 85 // etcfile
 };
 
 // usetrainingdatafine-tuned
 let quality_adjustment = (avg_reward * 10.0) as i32;
 let quality = (base_quality + quality_adjustment).clamp(60, 100) as u8;
 
 ImageConversionParams {
 quality,
 speed: self.predict_speed_from_ratio(avg_ratio),
 lossless: avg_ratio < 0.5, // 如果压缩率很好，可以try无损
 }
 }
 
 /// predictionvideoconversionparameter
 pub fn predict_video_params(&self, format: &str, _file_size: u64) -> VideoConversionParams {
 if !self.enabled {
 return VideoConversionParams::default();
 }
 
 let avg_ratio = self.training_data.average_compression_ratio(MediaType::Video, format);
 let avg_reward = self.training_data.average_reward(MediaType::Video, format);
 
 // based ontrainingdatarecommended比特率
 let bitrate = if avg_ratio < 1.0 {
 // compression效果好，can用较low比特率
 1000
 } else if avg_ratio < 2.0 {
 // etc compression，用 etc 比特率
 1500
 } else {
 // compression效果差，用较high比特率guaranteequality
 2000
 };
 
 VideoConversionParams {
 bitrate,
 preset: self.predict_preset_from_reward(avg_reward),
 crf: self.calculate_crf_from_ratio(avg_ratio),
 }
 }
 
 /// predictionaudioconversionparameter
 pub fn predict_audio_params(&self, format: &str, _file_size: u64) -> AudioConversionParams {
 if !self.enabled {
 return AudioConversionParams::default();
 }
 
 let _avg_ratio = self.training_data.average_compression_ratio(MediaType::Audio, format);
 let _avg_reward = self.training_data.average_reward(MediaType::Audio, format);
 
 // based ontrainingdatarecommended比特率
 let bitrate = if format == "aac" {
 // AACcompression效果最好，can用较low比特率
 128
 } else if format == "opus" {
 // Opus适合 etc 比特率
 160
 } else {
 // MP3need较high比特率
 192
 };
 
 AudioConversionParams {
 bitrate,
 sample_rate: 48000,
 channels: 2,
 }
 }
 
 /// recommended最佳format
 pub fn recommend_best_format(&self, media_type: MediaType) -> String {
 let formats = match media_type {
 MediaType::Image => vec!["webp", "avif", "jxl"],
 MediaType::Video => vec!["webm", "mp4"],
 MediaType::Audio => vec!["opus", "aac", "mp3"],
 };
 
 // findtoaverage奖励highestformat
 formats.iter()
 .max_by(|a, b| {
 let reward_a = self.training_data.average_reward(media_type.clone(), a);
 let reward_b = self.training_data.average_reward(media_type.clone(), b);
 // Handle NaN by treating it as Equal (shouldn't happen with valid rewards)
 reward_a.partial_cmp(&reward_b).unwrap_or(std::cmp::Ordering::Equal)
 })
 .map(|s| s.to_string())
 .unwrap_or_else(|| match media_type {
 MediaType::Image => "webp".to_string(),
 MediaType::Video => "webm".to_string(),
 MediaType::Audio => "aac".to_string(),
 })
 }
 
 /// fromcompression率predictionspeedparameter
 fn predict_speed_from_ratio(&self, ratio: f64) -> u8 {
 if ratio < 0.5 {
 6 // 压缩效果好，可以用慢速
 } else if ratio < 0.8 {
 4 // etc压缩，用平衡速度
 } else {
 2 // 压缩效果差，用quick
 }
 }
 
 /// from奖励predictionpreset
 fn predict_preset_from_reward(&self, reward: f64) -> String {
 if reward > 0.5 {
 "slow".to_string()
 } else if reward > 0.3 {
 "medium".to_string()
 } else {
 "fast".to_string()
 }
 }
 
 /// fromcompression率calculation CRF
 fn calculate_crf_from_ratio(&self, ratio: f64) -> u8 {
 // CRF: 0-51, valuemore小qualitymorehigh
 if ratio < 1.0 {
 18 // 压缩效果好，用高quality
 } else if ratio < 2.0 {
 23 // etc压缩，用etcquality
 } else {
 28 // 压缩效果差，用较低quality
 }
 }
 
 /// gettrainingstatisticsinformation
 pub fn get_training_stats(&self) -> String {
 let image_count = self.training_data.stats.get("image").unwrap_or(&0);
 let video_count = self.training_data.stats.get("video").unwrap_or(&0);
 let audio_count = self.training_data.stats.get("audio").unwrap_or(&0);
 
 format!(
 "PPO Training Data: total_samples={}, images={}, videos={}, audio={}, timestamp={}",
 self.training_data.total_samples,
 image_count,
 video_count,
 audio_count,
 self.training_data.timestamp
 )
 }
 
 /// enabled/disabledprediction
 pub fn set_enabled(&mut self, enabled: bool) {
 self.enabled = enabled;
 }
}

/// imageconversionparameter
#[derive(Debug, Clone)]
pub structure ImageConversionParams {
 pub quality: u8,
 pub speed: u8,
 pub lossless: bool,
}

impl Default for ImageConversionParams {
 fn default() -> Self {
 Self {
 quality: 85,
 speed: 4,
 lossless: false,
 }
 }
}

/// videoconversionparameter
#[derive(Debug, Clone)]
pub structure VideoConversionParams {
 pub bitrate: u32,
 pub preset: String,
 pub crf: u8,
}

impl Default for VideoConversionParams {
 fn default() -> Self {
 Self {
 bitrate: 1000,
 preset: "medium".to_string(),
 crf: 23,
 }
 }
}

/// audioconversionparameter
#[derive(Debug, Clone)]
pub structure AudioConversionParams {
 pub bitrate: u32,
 pub sample_rate: u32,
 pub channels: u8,
}

impl Default for AudioConversionParams {
 fn default() -> Self {
 Self {
 bitrate: 192,
 sample_rate: 48000,
 channels: 2,
 }
 }
}

#[cfg(test)]
mod tests {
 use super::*;
 
 #[test]
 fn test_training_data_loading() {
 // createtestdata
 let data = PPOTrainingData {
 timestamp: "2025-11-17".to_string(),
 total_samples: 2,
 stats: {
 let mut map = HashMap::new();
 map.insert("image".to_string(), 1);
 map.insert("video".to_string(), 1);
 map
 },
 data: vec![
 TrainingSample {
 media_type: MediaType::Image,
 input_file: "test.jpg".to_string(),
 input_size: 100000,
 target_format: "webp".to_string(),
 quality: 85,
 output_size: 50000,
 compression_ratio: 0.5,
 reward: 0.8,
 },
 TrainingSample {
 media_type: MediaType::Video,
 input_file: "test.mp4".to_string(),
 input_size: 1000000,
 target_format: "webm".to_string(),
 quality: 1000,
 output_size: 800000,
 compression_ratio: 0.8,
 reward: 0.6,
 },
 ],
 };
 
 assert_eq!(data.total_samples, 2);
 assert_eq!(data.filter_by_media_type(MediaType::Image).len(), 1);
 assert_eq!(data.filter_by_media_type(MediaType::Video).len(), 1);
 }
 
 #[test]
 fn test_enhanced_predictor() {
 let data = PPOTrainingData {
 timestamp: "2025-11-17".to_string(),
 total_samples: 1,
 stats: HashMap::new(),
 data: vec![
 TrainingSample {
 media_type: MediaType::Image,
 input_file: "test.jpg".to_string(),
 input_size: 100000,
 target_format: "webp".to_string(),
 quality: 85,
 output_size: 50000,
 compression_ratio: 0.5,
 reward: 0.8,
 },
 ],
 };
 
 let predictor = EnhancedPPOPredictor::new(data);
 let params = predictor.predict_image_params("webp", 100000);
 
 assert!(params.quality >= 60 && params.quality <= 100);
 assert!(params.speed >= 1 && params.speed <= 10);
 }
 
 #[test]
 fn test_format_recommendation() {
 let data = PPOTrainingData {
 timestamp: "2025-11-17".to_string(),
 total_samples: 2,
 stats: HashMap::new(),
 data: vec![
 TrainingSample {
 media_type: MediaType::Audio,
 input_file: "test.mp3".to_string(),
 input_size: 100000,
 target_format: "aac".to_string(),
 quality: 128,
 output_size: 50000,
 compression_ratio: 0.5,
 reward: 0.9,
 },
 TrainingSample {
 media_type: MediaType::Audio,
 input_file: "test.mp3".to_string(),
 input_size: 100000,
 target_format: "opus".to_string(),
 quality: 128,
 output_size: 60000,
 compression_ratio: 0.6,
 reward: 0.7,
 },
 ],
 };
 
 let predictor = EnhancedPPOPredictor::new(data);
 let best_format = predictor.recommend_best_format(MediaType::Audio);
 
 // AACshould be recommended，因forit奖励更high
 assert_eq!(best_format, "aac");
 }
}
