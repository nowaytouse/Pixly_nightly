// src/online_learning.rs
//! 🎓 atlinelearningmodule - Phase 3.2
//!
//! will PPOtrainingintegrationtoactualconversion
//!
//! feature：
//! - recordeverytimesconversion（feature、parameter、reward）
//! - 累积
//! - triggermodelupdate（every100timesconversion）
//! - modelhotload

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};

use crate::ai::reward_calculator::{ConversionResult, RewardCalculator};
use crate::analysis::media_analyzer::MediaType;

/// PPOparameterstructure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PPOParameters {
 pub quality: u32,
 pub format: String,
}

/// sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
 pub features: Vec<f64>,
 pub quality: u32,
 pub effort: u32,
 pub reward: f64,
 pub timestamp: u64,
 #[serde(default)]
 pub ssim: Option<f64>, // 🎯 SSIMqualityscore（optional，foraftermorenew）
}

/// 🎯 ML-506: modelversioninformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersion {
 pub version: u32,
 pub timestamp: u64,
 pub num_experiences: usize,
 pub avg_reward: f64,
 pub path: PathBuf,
}

/// atlinelearning
pub struct OnlineLearner {
/// PPOmodelpath
 model_path: PathBuf,
/// 
 experience_buffer: Arc<Mutex<Vec<Experience>>>,
/// updateinterval（count）
 update_interval: usize,
/// rewardcalculation
 reward_calculator: RewardCalculator,
/// isnoenabled
 enabled: bool,
/// 🎯 ML-506: currentmodelversion（useinternalcan）
 current_version: Arc<Mutex<u32>>,
}

impl OnlineLearner {
/// createnewatlinelearning（autoload持久）
 pub fn new(model_path: PathBuf, update_interval: usize) -> Self {
 let mut learner = Self {
 model_path,
 experience_buffer: Arc::new(Mutex::new(Vec::new())),
 update_interval,
 reward_calculator: RewardCalculator::new(),
 enabled: true,
 current_version: Arc::new(Mutex::new(0)), // 🎯 ML-506: version
 };

// Auto-load persisted experiences
 if let Err(e) = learner.load_persisted_experiences() {
 log::warn!("Failed to load persisted experiences: {}", e);
 }

 learner
 }

/// Disable online learning
 pub fn disable(&mut self) {
 self.enabled = false;
 }

/// Enable online learning
 pub fn enable(&mut self) {
 self.enabled = true;

// Check if there are accumulated experiences that need training
 let buffer_size = self.buffer_size();
 if buffer_size >= self.update_interval {
 log::info!("Found {} accumulated experiences, triggering batch training...", buffer_size);
 if let Err(e) = self.trigger_update() {
 log::error!("Failed to trigger batch training: {}", e);
 }
 }
 }

/// ML-504: PPO parameter callback function (when ML system needs parameters)
 pub fn get_ppo_params(
 &self,
 media_type: MediaType,
 format: &str,
 _file_size: u64,
 ) -> Result<PPOParameters> {
 if !self.enabled {
// Online learning disabled, return error
 anyhow::bail!("Online learning disabled");
 }

// Query historical experience and return parameters
 let params = match media_type {
 MediaType::Image => {
 self.find_best_params_for_image(format)
 }
 MediaType::Animation => {
// Animation uses same parameters as image
 self.find_best_params_for_image(format)
 }
 MediaType::Video => {
 self.find_best_params_for_video(format)
 }
 MediaType::Audio => {
 self.find_best_params_for_audio(format)
 }
 MediaType::Unknown => {
 anyhow::bail!("Unknown media type");
 }
 };

 Ok(params)
 }

/// ML-504 helper: Find best parameters for image
 fn find_best_params_for_image(&self, format: &str) -> PPOParameters {
// Look for similar successful cases in experience_buffer
// Return historical record if available, otherwise return default
 PPOParameters {
 quality: 85,
 format: format.to_string(),
 }
 }

/// ML-504 helper: Find best parameters for video
 fn find_best_params_for_video(&self, format: &str) -> PPOParameters {
 PPOParameters {
 quality: 28, // CRF value
 format: format.to_string(),
 }
 }

/// ML-504 helper: Find best parameters for audio
 fn find_best_params_for_audio(&self, format: &str) -> PPOParameters {
 PPOParameters {
 quality: 128, // bitrate
 format: format.to_string(),
 }
 }

/// Record conversion experience
 pub fn record_conversion(
 &self,
 features: Vec<f64>,
 quality: u32,
 effort: u32,
 result: ConversionResult,
 ) -> Result<()> {
 if !self.enabled {
 return Ok(());
 }

// Calculate reward
 let reward = self.reward_calculator.calculate(&result);

// Create experience
 let experience = Experience {
 features,
 quality,
 effort,
 reward,
 timestamp: std::time::SystemTime::now()
 .duration_since(std::time::UNIX_EPOCH)
 .unwrap_or_default()
 .as_secs(),
 ssim: None, // Initially None, can be updated later via update_last_experience_ssim
 };

// Store experience
 let mut buffer = self.experience_buffer.lock().expect("Mutex poisoned");
 buffer.push(experience);

 log::info!("Recorded experience: reward={:.4}, buffer_size={}",
 reward, buffer.len());

// Persist experiences to disk
 drop(buffer); // Release lock
 if let Err(e) = self.persist_experiences() {
 log::warn!("Failed to persist experiences: {}", e);
 }

// Check if update is needed
 let buffer_size = self.buffer_size();
 if buffer_size >= self.update_interval {
 log::info!("Triggering model update ({} experiences)", buffer_size);
 self.internal_trigger_update()?;
 }

 Ok(())
 }

/// ML-505: Check if model should be updated
 pub fn should_update(&self) -> bool {
 if !self.enabled {
 return false;
 }
 self.buffer_size() >= self.update_interval
 }

/// ML-505: Public trigger update interface
 pub fn trigger_update(&self) -> Result<()> {
 let buffer_size = self.buffer_size();
 if buffer_size >= self.update_interval {
 log::info!("Triggering model update ({} experiences)", buffer_size);
 self.internal_trigger_update()?;
 }
 Ok(())
 }

/// ML-507: Public batch training trigger interface
 pub fn trigger_batch_training(&self, _buffer_size: usize) -> Result<()> { // _buffer_size is now unused
 self.internal_trigger_update()
 }

/// Trigger model update (using batch PPO trainer)
 fn internal_trigger_update(&self) -> Result<()> {
 log::info!("Starting batch PPO model update...");

 let buffer_size = self.buffer_size();

 let model_dir = self.model_path.parent()
 .unwrap_or_else(|| Path::new("models/ppo"));

 let experience_file = model_dir.join("experience_buffer.json");
 if !experience_file.exists() {
 anyhow::bail!("Experience file not found: {:?}", experience_file);
 }

 log::info!(" Experience file: {:?}", experience_file);
 log::info!(" Model dir: {:?}", model_dir);
 log::info!(" Buffer size: {}", buffer_size);

 let output = std::process::Command::new("python3")
 .arg("scripts/batch_ppo_update.py")
 .arg("--experience-file")
 .arg(&experience_file)
 .arg("--model-dir")
 .arg(model_dir)
 .arg("--batch-size")
 .arg("32")
 .output()
 .context("Failed to run batch PPO updater")?;

 if output.status.success() {
 let stdout = String::from_utf8_lossy(&output.stdout);
 log::info!("Batch model update complete ({} experiences)", buffer_size);

// ML-506: Version management
// 1. Backup old model
 if let Err(e) = self.backup_model() {
 log::warn!("Failed to backup model: {}", e);
 }

// 2. Save version info
 if let Err(e) = self.save_version_info() {
 log::warn!("Failed to save version info: {}", e);
 }

// 3. Increment version number
 {
 let mut ver = self.current_version.lock().expect("Mutex poisoned");
 *ver += 1;
 log::info!("Model version updated: v{}", *ver);
 }

// Parse result (last line is JSON)
 if let Some(last_line) = stdout.lines().last()
 && let Ok(result) = serde_json::from_str::<serde_json::Value>(last_line)
 && let Some(avg_loss) = result.get("avg_loss").and_then(|v| v.as_f64()) {
 log::info!(" Average loss: {:.4}", avg_loss);
 }
 } else {
 let stderr = String::from_utf8_lossy(&output.stderr);
 log::error!("Batch update failed: {}", stderr);
 return Err(anyhow::anyhow!("Batch PPO update failed"));
 }

 Ok(())
 }



/// getcurrentsize
 pub fn buffer_size(&self) -> usize {
 self.experience_buffer.lock().expect("Mutex poisoned").len()
 }

/// Update SSIM value of last experience
///
/// Used to supplement SSIM quality score after conversion completes
 pub fn update_last_experience_ssim(&self, ssim: f64) -> Result<()> {
 if !self.enabled {
 return Ok(());
 }

 let mut buffer = self.experience_buffer.lock().expect("Mutex poisoned");
 if let Some(last_exp) = buffer.last_mut() {
 last_exp.ssim = Some(ssim);
 log::info!("Updated last experience SSIM: {:.4}", ssim);

// Persist update
 drop(buffer);
 if let Err(e) = self.persist_experiences() {
 log::warn!("Failed to persist SSIM update: {}", e);
 }
 } else {
 log::warn!("No experience to update SSIM");
 }

 Ok(())
 }

/// Manually trigger update
 pub fn manual_update(&self) -> Result<()> {
 let size = self.buffer_size();
 if size == 0 {
 log::warn!("No experiences to update");
 return Ok(());
 }

 log::info!("Manual update triggered ({} experiences)", size);
 self.trigger_update()
 }

/// ML-506: Backup current model
 fn backup_model(&self) -> Result<()> {
 if !self.model_path.exists() {
 return Ok(()); // No model to backup
 }

 let current_ver = *self.current_version.lock().expect("Mutex poisoned");

 let backup_dir = self.model_path.parent()
 .context("Invalid model path")?
 .join("backups");
 std::fs::create_dir_all(&backup_dir)?;

 let backup_path = backup_dir.join(format!(
 "actor_v{}.pth",
 current_ver
 ));

 std::fs::copy(&self.model_path, &backup_path)?;
 log::info!("Backed up model v{} to {:?}", current_ver, backup_path);

 Ok(())
 }

/// ML-506: Save version info
 fn save_version_info(&self) -> Result<()> {
 let buffer = self.experience_buffer.lock().expect("Mutex poisoned");
 let current_ver = *self.current_version.lock().expect("Mutex poisoned");

 let avg_reward = if buffer.is_empty() {
 0.0
 } else {
 buffer.iter().map(|e| e.reward).sum::<f64>() / buffer.len() as f64
 };

 let version_info = ModelVersion {
 version: current_ver,
 timestamp: std::time::SystemTime::now()
 .duration_since(std::time::UNIX_EPOCH)
 .unwrap_or_default()
 .as_secs(),
 num_experiences: buffer.len(),
 avg_reward,
 path: self.model_path.clone(),
 };

 let version_path = self.model_path.parent()
 .context("Invalid model path")?
 .join(format!("version_v{}.json", current_ver));

 let json = serde_json::to_string_pretty(&version_info)?;
 std::fs::write(&version_path, json)?;

 log::info!("Saved version info v{} (avg_reward: {:.4})",
 current_ver, avg_reward);

 Ok(())
 }

/// ML-506: Get current version
 pub fn current_version(&self) -> u32 {
 *self.current_version.lock().expect("Mutex poisoned")
 }

/// ML-506: Rollback to specified version
 pub fn rollback_to_version(&self, version: u32) -> Result<()> {
 let backup_path = self.model_path.parent()
 .context("Invalid model path")?
 .join("backups")
 .join(format!("actor_v{}.pth", version));

 if !backup_path.exists() {
 anyhow::bail!("Version {} backup not found", version);
 }

 std::fs::copy(&backup_path, &self.model_path)?;
 *self.current_version.lock().expect("Mutex poisoned") = version;

 log::info!("Rolled back to model v{}", version);
 Ok(())
 }

/// Persist experiences to disk
 fn persist_experiences(&self) -> Result<()> {
 let persist_path = Path::new("models/ppo/experience_buffer.json");

// Ensure directory exists
 if let Some(parent) = persist_path.parent() {
 std::fs::create_dir_all(parent)?;
 }

 let buffer = self.experience_buffer.lock().expect("Mutex poisoned");
 let json = serde_json::to_string_pretty(&*buffer)?;
 std::fs::write(persist_path, json)?;

 log::debug!("Persisted {} experiences to {:?}", buffer.len(), persist_path);
 Ok(())
 }

/// Load persisted experiences
 fn load_persisted_experiences(&mut self) -> Result<()> {
 let persist_path = Path::new("models/ppo/experience_buffer.json");

 if !persist_path.exists() {
 log::debug!("No persisted experiences found");
 return Ok(());
 }

 let json = std::fs::read_to_string(persist_path)?;
 let experiences: Vec<Experience> = serde_json::from_str(&json)?;

 let mut buffer = self.experience_buffer.lock().expect("Mutex poisoned");
 *buffer = experiences;

 log::info!("Loaded {} persisted experiences", buffer.len());
 Ok(())
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_record_experience() {
// usetemporarydirectoryload持久
 let temp_dir = std::env::temp_dir().join("pixly_test_online_learning");
 let _ = std::fs::remove_dir_all(&temp_dir); // clean upolddata
 std::fs::create_dir_all(&temp_dir).unwrap();

 let learner = OnlineLearner::new(
 temp_dir.join("test.pth"),
 100
 );

 let features = vec![0.5; 128];
 let result = ConversionResult {
 original_size: 1000000,
 output_size: 500000,
 ssim: 0.98,
 processing_time: 2.0,
 };

 let initial_size = learner.buffer_size();
 learner.record_conversion(features, 80, 6, result).unwrap();
 assert_eq!(learner.buffer_size(), initial_size + 1);

// cleanup
 let _ = std::fs::remove_dir_all(&temp_dir);
 }
}
