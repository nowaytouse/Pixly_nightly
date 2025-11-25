// src/online_learner_manager.rs
//! 🎓 atlinelearningManager - globalsingleton
//!
//! feature：
//! - globalsingletonOnlineLearner
//! - conversionsession
//! - autotriggermodelupdate

use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use once_cell::sync::Lazy;
use super::online_learning::OnlineLearner;
use super::reward_calculator::ConversionResult;

/// globalatlinelearninginstance
static GLOBAL_LEARNER: Lazy<Arc<Mutex<OnlineLearner>>> = Lazy::new(|| {
// 🔥 defaultdisabled，needenabled
 let mut learner = OnlineLearner::new(
 PathBuf::from("models/ppo/actor_online.pth"),
 10 // every10timesconvertmorenewoncemodel（lowthresholdbytest）
 );
 learner.disable(); // 🔥 defaultdisabled
 Arc::new(Mutex::new(learner))
});

/// globalenabled
static ENABLED: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| {
 Arc::new(Mutex::new(false))
});

/// atlinelearningManager
pub struct OnlineLearnerManager;

impl OnlineLearnerManager {
/// Enable online learning
 pub fn enable() {
 let mut enabled = ENABLED.lock().expect("Mutex poisoned");
 *enabled = true;

 let mut learner = GLOBAL_LEARNER.lock().expect("Mutex poisoned");
 learner.enable();

 log::info!("Online learning enabled");
 }

/// checkisnoenabled
 pub fn is_enabled() -> bool {
 *ENABLED.lock().expect("Mutex poisoned")
 }

/// Record conversion experience
 pub fn record_conversion(
 features: Vec<f64>,
 quality: u32,
 effort: u32,
 result: ConversionResult,
 ) -> anyhow::Result<()> {
// Check if enabled
 if !Self::is_enabled() {
 return Ok(()); // Silent skip
 }

 let learner = GLOBAL_LEARNER.lock().expect("Mutex poisoned");
 learner.record_conversion(features, quality, effort, result)?;

 let buffer_size = learner.buffer_size();
 log::info!("Experience recorded (global buffer: {})", buffer_size);

// ML-505: Auto-trigger mechanism
// When buffer reaches threshold, automatically trigger model update
 if learner.should_update() {
 log::info!("Auto-triggering model update ({} experiences)", buffer_size);

// Release lock before update (avoid deadlock)
 drop(learner);

 match Self::auto_update() {
 Ok(_) => log::info!("Auto-update completed successfully"),
 Err(e) => {
 log::error!("Auto-update failed: {}", e);
// Don't return error to avoid blocking conversion flow
 }
 }
 }

 Ok(())
 }

/// ML-505: Auto-update model
 fn auto_update() -> anyhow::Result<()> {
 let learner = GLOBAL_LEARNER.lock().expect("Mutex poisoned");
 learner.trigger_update()
 }

/// Get current buffer size
 pub fn buffer_size() -> usize {
 GLOBAL_LEARNER.lock().expect("Mutex poisoned").buffer_size()
 }

/// Update SSIM value of last experience
 pub fn update_last_experience_ssim(ssim: f64) -> anyhow::Result<()> {
 let learner = GLOBAL_LEARNER.lock().expect("Mutex poisoned");
 learner.update_last_experience_ssim(ssim)
 }

/// Manually trigger model update
 pub fn manual_update() -> anyhow::Result<()> {
 let learner = GLOBAL_LEARNER.lock().expect("Mutex poisoned");
 learner.manual_update()
 }

/// Disable online learning
 pub fn disable() {
 let mut learner = GLOBAL_LEARNER.lock().expect("Mutex poisoned");
 learner.disable();
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_global_learner() {
 let features = vec![0.5; 128];
 let result = ConversionResult {
 original_size: 1000000,
 output_size: 500000,
 ssim: 0.98,
 processing_time: 2.0,
 };

 OnlineLearnerManager::record_conversion(features, 80, 6, result).unwrap();
 assert!(OnlineLearnerManager::buffer_size() > 0);
 }
}
