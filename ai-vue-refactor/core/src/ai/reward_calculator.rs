// src/reward_calculator.rs
//! 🎯 rewardfunctioncalculation - Phase 3.1
//!
//! forPPOstronglearningrewardcalculation
//!
//! rewardgroup：
//! - compressionreward：filesizereduce（0-1）
//! - qualitypenalty：SSIM < 0.95 when penalty
//! - speedpenalty：processingtimelongpenalty
//!
//! target：maximumcompression， when keephighquality and speed

// use anyhow::Result; //  when not need

/// conversionresult
#[derive(Debug, Clone)]
pub struct ConversionResult {
 pub original_size: u64,
 pub output_size: u64,
 pub ssim: f64,
 pub processing_time: f64,
}

/// rewardcalculation
pub struct RewardCalculator {
/// SSIMqualitythreshold（belowthisvalue will has penalty）
 pub quality_threshold: f64,
/// speedthreshold（，exceedsthisvalue will has penalty）
 pub speed_threshold: f64,
/// qualitypenaltyweight
 pub quality_penalty_weight: f64,
/// speedpenaltyweight
 pub speed_penalty_weight: f64,
}

impl Default for RewardCalculator {
 fn default() -> Self {
 Self {
 quality_threshold: 0.95,
 speed_threshold: 5.0,
 quality_penalty_weight: 2.0,
 speed_penalty_weight: 0.1,
 }
 }
}

impl RewardCalculator {
 pub fn new() -> Self {
 Self::default()
 }

/// calculationreward
///
/// returnvaluerange：-∞ ~ 1.0
/// - 1.0: compression（100%reduce，SSIM=1.0，speedfast）
/// - 0.0: nocompression
/// - negative: qualityorspeedissueheavy
 pub fn calculate(&self, result: &ConversionResult) -> f64 {
// 1. compressionreward（0-1）
 let compression_reward = self.calculate_compression_reward(result);

// 2. qualitypenalty（0ornegative）
 let quality_penalty = self.calculate_quality_penalty(result);

// 3. speedpenalty（0ornegative）
 let speed_penalty = self.calculate_speed_penalty(result);

// reward


 compression_reward + quality_penalty + speed_penalty
 }

/// calculationcompressionreward
 fn calculate_compression_reward(&self, result: &ConversionResult) -> f64 {
 if result.output_size >= result.original_size {
// filelarge，noreward
 return 0.0;
 }

 let reduction = (result.original_size - result.output_size) as f64;
 let ratio = reduction / result.original_size as f64;

// compression作forreward（0-1）
 ratio.clamp(0.0, 1.0)
 }

/// calculationqualitypenalty
 fn calculate_quality_penalty(&self, result: &ConversionResult) -> f64 {
 if result.ssim >= self.quality_threshold {
// quality，nopenalty
 return 0.0;
 }

// SSIMbelowthreshold，linepenalty
 let quality_loss = self.quality_threshold - result.ssim;


 -quality_loss * self.quality_penalty_weight
 }

/// calculationspeedpenalty
 fn calculate_speed_penalty(&self, result: &ConversionResult) -> f64 {
 if result.processing_time <= self.speed_threshold {
// speedfast，nopenalty
 return 0.0;
 }

// exceedsthreshold，linepenalty
 let time_excess = result.processing_time - self.speed_threshold;


 -time_excess * self.speed_penalty_weight
 }

/// calculationdetailedreward（fordebug and analysis）
 pub fn calculate_detailed(&self, result: &ConversionResult) -> RewardBreakdown {
 let compression_reward = self.calculate_compression_reward(result);
 let quality_penalty = self.calculate_quality_penalty(result);
 let speed_penalty = self.calculate_speed_penalty(result);
 let total = compression_reward + quality_penalty + speed_penalty;

 RewardBreakdown {
 compression_reward,
 quality_penalty,
 speed_penalty,
 total_reward: total,
 }
 }
}

/// rewarddetailed
#[derive(Debug, Clone)]
pub struct RewardBreakdown {
 pub compression_reward: f64,
 pub quality_penalty: f64,
 pub speed_penalty: f64,
 pub total_reward: f64,
}

impl std::fmt::Display for RewardBreakdown {
 fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
 write!(
 f,
 "Reward: {:.4} (compression: {:.4}, quality: {:.4}, speed: {:.4})",
 self.total_reward,
 self.compression_reward,
 self.quality_penalty,
 self.speed_penalty
 )
 }
}


#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_perfect_compression() {
 let calculator = RewardCalculator::new();
 let result = ConversionResult {
 original_size: 1000000,
 output_size: 500000, // 50% reduction
 ssim: 0.98, // High quality
 processing_time: 2.0, // Fast
 };

 let reward = calculator.calculate(&result);
 assert!(reward > 0.4 && reward < 0.6, "Expected ~0.5, got {}", reward);
 }

 #[test]
 fn test_quality_penalty() {
 let calculator = RewardCalculator::new();
 let result = ConversionResult {
 original_size: 1000000,
 output_size: 500000,
 ssim: 0.90, // Below threshold (0.95)
 processing_time: 2.0,
 };

 let breakdown = calculator.calculate_detailed(&result);
 assert!(breakdown.quality_penalty < 0.0, "Should have quality penalty");
 assert!(breakdown.total_reward < 0.5, "Total should be reduced");
 }

 #[test]
 fn test_speed_penalty() {
 let calculator = RewardCalculator::new();
 let result = ConversionResult {
 original_size: 1000000,
 output_size: 500000,
 ssim: 0.98,
 processing_time: 10.0, // Slow (> 5s threshold)
 };

 let breakdown = calculator.calculate_detailed(&result);
 assert!(breakdown.speed_penalty < 0.0, "Should have speed penalty");
 }

 #[test]
 fn test_no_compression() {
 let calculator = RewardCalculator::new();
 let result = ConversionResult {
 original_size: 1000000,
 output_size: 1000000, // No reduction
 ssim: 0.98,
 processing_time: 2.0,
 };

 let reward = calculator.calculate(&result);
 assert_eq!(reward, 0.0, "No compression should give 0 reward");
 }
}
