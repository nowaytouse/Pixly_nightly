// src/online_learning.rs
//! 🎓 在线学习模块 - Phase 3.2
//! 
//! 将PPO训练集成到实际转换流程中
//! 
//! 功能：
//! - 记录每次转换的经验（特征、参数、奖励）
//! - 累积经验缓冲
//! - 定期触发模型更新（每100次转换）
//! - 模型热加载

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};

use crate::reward_calculator::{ConversionResult, RewardCalculator};

/// 经验样本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub features: Vec<f64>,
    pub quality: u32,
    pub effort: u32,
    pub reward: f64,
    pub timestamp: u64,
}

/// 在线学习器
pub struct OnlineLearner {
    /// PPO模型路径
    model_path: PathBuf,
    /// 经验缓冲
    experience_buffer: Arc<Mutex<Vec<Experience>>>,
    /// 更新间隔（经验数量）
    update_interval: usize,
    /// 奖励计算器
    reward_calculator: RewardCalculator,
    /// 是否启用
    enabled: bool,
}

impl OnlineLearner {
    /// 创建新的在线学习器
    pub fn new(model_path: PathBuf, update_interval: usize) -> Self {
        Self {
            model_path,
            experience_buffer: Arc::new(Mutex::new(Vec::new())),
            update_interval,
            reward_calculator: RewardCalculator::new(),
            enabled: true,
        }
    }
    
    /// 禁用在线学习
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    
    /// 记录转换经验
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
        
        // 计算奖励
        let reward = self.reward_calculator.calculate(&result);
        
        // 创建经验
        let experience = Experience {
            features,
            quality,
            effort,
            reward,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        
        // 存储经验
        let mut buffer = self.experience_buffer.lock().unwrap();
        buffer.push(experience);
        
        log::info!("📝 Recorded experience: reward={:.4}, buffer_size={}", 
                   reward, buffer.len());
        
        // 检查是否需要更新
        if buffer.len() >= self.update_interval {
            log::info!("🎓 Triggering model update ({} experiences)", buffer.len());
            drop(buffer); // 释放锁
            self.trigger_update()?;
        }
        
        Ok(())
    }
    
    /// 触发模型更新
    fn trigger_update(&self) -> Result<()> {
        // 导出经验到临时文件
        let temp_file = std::env::temp_dir().join("pixly_experiences.json");
        self.export_experiences(&temp_file)?;
        
        log::info!("🚀 Starting PPO model update...");
        
        // 调用Python训练脚本
        let output = std::process::Command::new("python3")
            .arg("scripts/train_ppo_update.py")
            .arg("--experiences")
            .arg(&temp_file)
            .arg("--model")
            .arg(&self.model_path)
            .arg("--epochs")
            .arg("5")
            .output()
            .context("Failed to run PPO update script")?;
        
        if output.status.success() {
            log::info!("✅ Model update complete");
            // 清空缓冲
            self.experience_buffer.lock().unwrap().clear();
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("❌ Model update failed: {}", stderr);
        }
        
        Ok(())
    }

    
    /// 导出经验到文件
    fn export_experiences(&self, path: &Path) -> Result<()> {
        let buffer = self.experience_buffer.lock().unwrap();
        let json = serde_json::to_string_pretty(&*buffer)?;
        std::fs::write(path, json)?;
        log::info!("💾 Exported {} experiences to {:?}", buffer.len(), path);
        Ok(())
    }
    
    /// 获取当前缓冲大小
    pub fn buffer_size(&self) -> usize {
        self.experience_buffer.lock().unwrap().len()
    }
    
    /// 手动触发更新
    pub fn manual_update(&self) -> Result<()> {
        let size = self.buffer_size();
        if size == 0 {
            log::warn!("⚠️  No experiences to update");
            return Ok(());
        }
        
        log::info!("🎓 Manual update triggered ({} experiences)", size);
        self.trigger_update()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_record_experience() {
        let learner = OnlineLearner::new(
            PathBuf::from("models/ppo/test.pth"),
            100
        );
        
        let features = vec![0.5; 128];
        let result = ConversionResult {
            original_size: 1000000,
            output_size: 500000,
            ssim: 0.98,
            processing_time: 2.0,
        };
        
        learner.record_conversion(features, 80, 6, result).unwrap();
        assert_eq!(learner.buffer_size(), 1);
    }
}
