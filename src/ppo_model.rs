// 🤖 PPO强化学习模型
// 加载和使用PPO训练数据进行预测

use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::{Context, Result};

/// PPO训练统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PPOTrainingStats {
    pub actor_losses: Vec<f64>,
    pub critic_losses: Vec<f64>,
    pub total_rewards: Vec<f64>,
}

impl PPOTrainingStats {
    /// 从JSON文件加载
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .context("Failed to read PPO training stats file")?;
        
        let stats: PPOTrainingStats = serde_json::from_str(&content)
            .context("Failed to parse PPO training stats")?;
        
        Ok(stats)
    }
    
    /// 获取最终actor损失
    pub fn final_actor_loss(&self) -> Option<f64> {
        self.actor_losses.last().copied()
    }
    
    /// 获取最终critic损失
    pub fn final_critic_loss(&self) -> Option<f64> {
        self.critic_losses.last().copied()
    }
    
    /// 获取平均奖励
    pub fn average_reward(&self) -> f64 {
        if self.total_rewards.is_empty() {
            return 0.0;
        }
        
        let sum: f64 = self.total_rewards.iter().sum();
        sum / self.total_rewards.len() as f64
    }
    
    /// 获取训练步数
    pub fn training_steps(&self) -> usize {
        self.actor_losses.len()
    }
    
    /// 检查训练是否收敛
    pub fn is_converged(&self) -> bool {
        if self.actor_losses.len() < 100 {
            return false;
        }
        
        // 检查最后100步的损失变化
        let last_100 = &self.actor_losses[self.actor_losses.len() - 100..];
        let mean: f64 = last_100.iter().sum::<f64>() / 100.0;
        let variance: f64 = last_100.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / 100.0;
        
        // 如果方差很小，认为已收敛
        variance < 0.0001
    }
}

/// PPO预测器
pub struct PPOPredictor {
    stats: PPOTrainingStats,
    enabled: bool,
}

impl PPOPredictor {
    /// 创建新的PPO预测器
    pub fn new(stats: PPOTrainingStats) -> Self {
        Self {
            stats,
            enabled: true,
        }
    }
    
    /// 从文件加载
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let stats = PPOTrainingStats::from_file(path)?;
        Ok(Self::new(stats))
    }
    
    /// 使用PPO调整质量参数
    /// 基于训练的奖励和损失来微调预测
    pub fn adjust_quality(&self, base_quality: u8, complexity: f64) -> u8 {
        if !self.enabled {
            return base_quality;
        }
        
        // 使用平均奖励作为调整因子
        let reward_factor = self.stats.average_reward();
        
        // 使用最终actor损失作为置信度
        let actor_loss = self.stats.final_actor_loss().unwrap_or(0.0);
        let confidence = 1.0 / (1.0 + actor_loss.abs());
        
        // 根据复杂度和PPO学习结果调整
        let adjustment = if complexity > 0.7 {
            // 高复杂度：PPO建议提高质量
            (reward_factor * confidence * 5.0) as i32
        } else if complexity < 0.3 {
            // 低复杂度：PPO建议降低质量节省空间
            -(reward_factor * confidence * 3.0) as i32
        } else {
            // 中等复杂度：小幅调整
            (reward_factor * confidence * 2.0) as i32
        };
        
        (base_quality as i32 + adjustment).clamp(60, 100) as u8
    }
    
    /// 预测最优速度参数
    pub fn predict_speed(&self, file_size_mb: f64) -> u8 {
        if !self.enabled {
            return 4;
        }
        
        let reward = self.stats.average_reward();
        
        // 根据文件大小和PPO学习结果预测速度
        let base_speed = if file_size_mb > 10.0 {
            2 // 大文件用快速预设
        } else if file_size_mb < 1.0 {
            6 // 小文件用慢速预设
        } else {
            4 // 中等文件用平衡预设
        };
        
        // PPO微调
        let adjustment = (reward * 2.0) as i32;
        (base_speed + adjustment).clamp(1, 10) as u8
    }
    
    /// 获取训练信息
    pub fn get_training_info(&self) -> String {
        format!(
            "PPO训练: {} 步, 平均奖励: {:.3}, Actor损失: {:.6}, Critic损失: {:.6}, 收敛: {}",
            self.stats.training_steps(),
            self.stats.average_reward(),
            self.stats.final_actor_loss().unwrap_or(0.0),
            self.stats.final_critic_loss().unwrap_or(0.0),
            if self.stats.is_converged() { "是" } else { "否" }
        )
    }
    
    /// 启用/禁用PPO
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
        
        // 高复杂度应该提高质量
        let adjusted = predictor.adjust_quality(80, 0.9);
        assert!(adjusted >= 80);
        
        // 低复杂度应该降低质量
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
        // 收敛的情况
        let converged_stats = PPOTrainingStats {
            actor_losses: vec![-0.03; 200],
            critic_losses: vec![0.01; 200],
            total_rewards: vec![0.85; 200],
        };
        assert!(converged_stats.is_converged());
        
        // 未收敛的情况
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
