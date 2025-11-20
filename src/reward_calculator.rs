// src/reward_calculator.rs
//! 🎯 奖励函数计算器 - Phase 3.1
//! 
//! 用于PPO强化学习的奖励计算
//! 
//! 奖励组成：
//! - 压缩奖励：文件大小减少比例（0-1）
//! - 质量惩罚：SSIM < 0.95时的惩罚
//! - 速度惩罚：处理时间过长的惩罚
//! 
//! 目标：最大化压缩率，同时保持高质量和合理速度

// use anyhow::Result;  // 暂时不需要

/// 转换结果
#[derive(Debug, Clone)]
pub struct ConversionResult {
    pub original_size: u64,
    pub output_size: u64,
    pub ssim: f64,
    pub processing_time: f64,
}

/// 奖励计算器
pub struct RewardCalculator {
    /// SSIM质量阈值（低于此值会有惩罚）
    pub quality_threshold: f64,
    /// 速度阈值（秒，超过此值会有惩罚）
    pub speed_threshold: f64,
    /// 质量惩罚权重
    pub quality_penalty_weight: f64,
    /// 速度惩罚权重
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

    /// 计算奖励
    /// 
    /// 返回值范围：-∞ ~ 1.0
    /// - 1.0: 完美压缩（100%减少，SSIM=1.0，速度快）
    /// - 0.0: 无压缩效果
    /// - 负数: 质量或速度问题严重
    pub fn calculate(&self, result: &ConversionResult) -> f64 {
        // 1. 压缩奖励（0-1）
        let compression_reward = self.calculate_compression_reward(result);
        
        // 2. 质量惩罚（0或负数）
        let quality_penalty = self.calculate_quality_penalty(result);
        
        // 3. 速度惩罚（0或负数）
        let speed_penalty = self.calculate_speed_penalty(result);
        
        // 总奖励
        
        
        compression_reward + quality_penalty + speed_penalty
    }
    
    /// 计算压缩奖励
    fn calculate_compression_reward(&self, result: &ConversionResult) -> f64 {
        if result.output_size >= result.original_size {
            // 文件变大了，无奖励
            return 0.0;
        }
        
        let reduction = (result.original_size - result.output_size) as f64;
        let ratio = reduction / result.original_size as f64;
        
        // 压缩比例作为奖励（0-1）
        ratio.clamp(0.0, 1.0)
    }
    
    /// 计算质量惩罚
    fn calculate_quality_penalty(&self, result: &ConversionResult) -> f64 {
        if result.ssim >= self.quality_threshold {
            // 质量足够好，无惩罚
            return 0.0;
        }
        
        // SSIM低于阈值，线性惩罚
        let quality_loss = self.quality_threshold - result.ssim;
        
        
        -quality_loss * self.quality_penalty_weight
    }

    /// 计算速度惩罚
    fn calculate_speed_penalty(&self, result: &ConversionResult) -> f64 {
        if result.processing_time <= self.speed_threshold {
            // 速度足够快，无惩罚
            return 0.0;
        }
        
        // 超过阈值，线性惩罚
        let time_excess = result.processing_time - self.speed_threshold;
        
        
        -time_excess * self.speed_penalty_weight
    }
    
    /// 计算详细奖励（用于调试和分析）
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

/// 奖励详细分解
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
            output_size: 500000,  // 50% reduction
            ssim: 0.98,           // High quality
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
            ssim: 0.90,           // Below threshold (0.95)
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
