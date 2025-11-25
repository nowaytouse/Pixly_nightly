// 🤖 PPOstronglearningmodel
// load and use PPOtrainingdatalineprediction

use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::{Context, Result};

/// PPOtrainingstatisticsdata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PPOTrainingStats {
 pub actor_losses: Vec<f64>,
 pub critic_losses: Vec<f64>,
 pub total_rewards: Vec<f64>,
}

impl PPOTrainingStats {
/// fromJSONfileload
 pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
 let content = std::fs::read_to_string(path)
 .context("Failed to read PPO training stats file")?;

 let stats: PPOTrainingStats = serde_json::from_str(&content)
 .context("Failed to parse PPO training stats")?;

 Ok(stats)
 }

/// getmostactorloss
 pub fn final_actor_loss(&self) -> Option<f64> {
 self.actor_losses.last().copied()
 }

/// getmostcriticloss
 pub fn final_critic_loss(&self) -> Option<f64> {
 self.critic_losses.last().copied()
 }

/// getaveragereward
 pub fn average_reward(&self) -> f64 {
 if self.total_rewards.is_empty() {
 return 0.0;
 }

 let sum: f64 = self.total_rewards.iter().sum();
 sum / self.total_rewards.len() as f64
 }

/// gettrainingstep
 pub fn training_steps(&self) -> usize {
 self.actor_losses.len()
 }

/// checktrainingisno
 pub fn is_converged(&self) -> bool {
 if self.actor_losses.len() < 100 {
 return false;
 }

// checkmost after 100steploss
 let last_100 = &self.actor_losses[self.actor_losses.len() - 100..];
 let mean: f64 = last_100.iter().sum::<f64>() / 100.0;
 let variance: f64 = last_100.iter()
 .map(|x| (x - mean).powi(2))
 .sum::<f64>() / 100.0;

// ifvarianceverysmall，foralready
 variance < 0.0001
 }
}

/// PPOprediction
pub struct PPOPredictor {
 stats: PPOTrainingStats,
 enabled: bool,
}

impl PPOPredictor {
/// createnewPPOprediction
 pub fn new(stats: PPOTrainingStats) -> Self {
 Self {
 stats,
 enabled: true,
 }
 }

/// fromfileload
 pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
 let stats = PPOTrainingStats::from_file(path)?;
 Ok(Self::new(stats))
 }

/// use PPOadjustedqualityparameter
/// based ontrainingreward and lossfine-tunedprediction
 pub fn adjust_quality(&self, base_quality: u8, complexity: f64) -> u8 {
 if !self.enabled {
 return base_quality;
 }

// useaveragerewardforadjustedbecausesub
 let reward_factor = self.stats.average_reward();

// usemostactorlossforconfidence
 let actor_loss = self.stats.final_actor_loss().unwrap_or(0.0);
 let confidence = 1.0 / (1.0 + actor_loss.abs());

// based oncomplexity and PPOlearningresultadjusted
 let adjustment = if complexity > 0.7 {
// highcomplexity：PPOsuggestedhighquality
 (reward_factor * confidence * 5.0) as i32
 } else if complexity < 0.3 {
// lowcomplexity：PPOsuggestedlowqualityempty
 -(reward_factor * confidence * 3.0) as i32
 } else {
// etc complexity：smalladjusted
 (reward_factor * confidence * 2.0) as i32
 };

 (base_quality as i32 + adjustment).clamp(60, 100) as u8
 }

/// predictionoptimalspeedparameter
 pub fn predict_speed(&self, file_size_mb: f64) -> u8 {
 if !self.enabled {
 return 4;
 }

 let reward = self.stats.average_reward();

// based onfilesize and PPOlearningresultpredictionspeed
 let base_speed = if file_size_mb > 10.0 {
 2 // largefilequickpreset
 } else if file_size_mb < 1.0 {
 6 // smallfileslowpreset
 } else {
 4 // etcfile平衡preset
 };

// PPOfine-tuned
 let adjustment = (reward * 2.0) as i32;
 (base_speed + adjustment).clamp(1, 10) as u8
 }

/// gettraininginformation
 pub fn get_training_info(&self) -> String {
 format!(
 "PPO training: {} steps, avg reward: {:.3}, actor loss: {:.6}, critic loss: {:.6}, converged: {}",
 self.stats.training_steps(),
 self.stats.average_reward(),
 self.stats.final_actor_loss().unwrap_or(0.0),
 self.stats.final_critic_loss().unwrap_or(0.0),
 if self.stats.is_converged() { "yes" } else { "no" }
 )
 }

/// enabled/disabledPPO
 pub fn set_enabled(&mut self, enabled: bool) {
 self.enabled = enabled;
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_ppo_stats_creation() {
 let stats = PPOTrainingStats {
 actor_losses: vec![-0.05, -0.04, -0.03],
 critic_losses: vec![0.03, 0.02, 0.01],
 total_rewards: vec![0.85, 0.85, 0.85],
 };

 assert_eq!(stats.training_steps(), 3);
 assert_eq!(stats.average_reward(), 0.85);
 assert_eq!(stats.final_actor_loss(), Some(-0.03));
 }

 #[test]
 fn test_ppo_quality_adjustment() {
 let stats = PPOTrainingStats {
 actor_losses: vec![-0.03; 100],
 critic_losses: vec![0.01; 100],
 total_rewards: vec![0.85; 100],
 };

 let predictor = PPOPredictor::new(stats);

// highcomplexityshouldhighquality
 let adjusted = predictor.adjust_quality(80, 0.9);
 assert!(adjusted >= 80);

// lowcomplexityshouldlowquality
 let adjusted = predictor.adjust_quality(80, 0.2);
 assert!(adjusted <= 80);
 }

 #[test]
 fn test_ppo_speed_prediction() {
 let stats = PPOTrainingStats {
 actor_losses: vec![-0.03; 100],
 critic_losses: vec![0.01; 100],
 total_rewards: vec![0.85; 100],
 };

 let predictor = PPOPredictor::new(stats);

 let speed = predictor.predict_speed(15.0);
 assert!((1..=10).contains(&speed));
 }

 #[test]
 fn test_convergence_check() {
// 情况
 let converged_stats = PPOTrainingStats {
 actor_losses: vec![-0.03; 200],
 critic_losses: vec![0.01; 200],
 total_rewards: vec![0.85; 200],
 };
 assert!(converged_stats.is_converged());

// not yet情况
 let mut diverged_losses = Vec::new();
 for i in 0..200 {
 diverged_losses.push(-0.05 + (i as f64) * 0.001);
 }
 let diverged_stats = PPOTrainingStats {
 actor_losses: diverged_losses,
 critic_losses: vec![0.01; 200],
 total_rewards: vec![0.85; 200],
 };
 assert!(!diverged_stats.is_converged());
 }
}
