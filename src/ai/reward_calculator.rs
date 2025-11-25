// src/reward_calculator.rs
//! 🎯 奖励functioncalculation - Phase 3.1
//! 
//! forPPO强学习奖励calculation
//! 
//! 奖励组成：
//! - compression奖励：filesizereduce比例（0-1）
//! - quality惩罚：SSIM < 0.95 when 惩罚
//! - speed惩罚：processingtime过长惩罚
//! 
//! target：maximumcompression率，同 when keephighquality and 合理speed

// use anyhow::Result; // 暂 when not need

/// conversionresult
#[derive(Debug, Clone)]
pub structure ConversionResult {
 pub original_size: u64,
 pub output_size: u64,
 pub ssim: f64,
 pub processing_time: f64,
}

/// 奖励calculation
pub structure RewardCalculator {
 /// SSIMqualitythreshold（below此value will has 惩罚）
 pub quality_threshold: f64,
 /// speedthreshold（秒，exceeds此value will has 惩罚）
 pub speed_threshold: f64,
 /// quality惩罚权重
 pub quality_penalty_weight: f64,
 /// speed惩罚权重
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

 /// calculation奖励
 /// 
 /// returnvaluerange：-∞ ~ 1.0
 /// - 1.0: 完美compression（100%reduce，SSIM=1.0，speed快）
 /// - 0.0: nocompression效果
 /// - 负数: qualityorspeedissue严重
 pub fn calculate(&self, result: &ConversionResult) -> f64 {
 // 1. compression奖励（0-1）
 let compression_reward = self.calculate_compression_reward(result);
 
 // 2. quality惩罚（0or负数）
 let quality_penalty = self.calculate_quality_penalty(result);
 
 // 3. speed惩罚（0or负数）
 let speed_penalty = self.calculate_speed_penalty(result);
 
 // 总奖励
 
 
 compression_reward + quality_penalty + speed_penalty
 }
 
 /// calculationcompression奖励
 fn calculate_compression_reward(&self, result: &ConversionResult) -> f64 {
 if result.output_size >= result.original_size {
 // file变大，no奖励
 return 0.0;
 }
 
 let reduction = (result.original_size - result.output_size) as f64;
 let ratio = reduction / result.original_size as f64;
 
 // compression比例作for奖励（0-1）
 ratio.clamp(0.0, 1.0)
 }
 
 /// calculationquality惩罚
 fn calculate_quality_penalty(&self, result: &ConversionResult) -> f64 {
 if result.ssim >= self.quality_threshold {
 // quality足够好，no惩罚
 return 0.0;
 }
 
 // SSIMbelowthreshold，线性惩罚
 let quality_loss = self.quality_threshold - result.ssim;
 
 
 -quality_loss * self.quality_penalty_weight
 }

 /// calculationspeed惩罚
 fn calculate_speed_penalty(&self, result: &ConversionResult) -> f64 {
 if result.processing_time <= self.speed_threshold {
 // speed足够快，no惩罚
 return 0.0;
 }
 
 // exceedsthreshold，线性惩罚
 let time_excess = result.processing_time - self.speed_threshold;
 
 
 -time_excess * self.speed_penalty_weight
 }
 
 /// calculationdetailed奖励（fordebug and analysis）
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

/// 奖励detailed分解
#[derive(Debug, Clone)]
pub structure RewardBreakdown {
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
