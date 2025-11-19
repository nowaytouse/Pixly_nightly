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
    /// 创建新的在线学习器（自动加载持久化的经验）
    pub fn new(model_path: PathBuf, update_interval: usize) -> Self {
        let mut learner = Self {
            model_path,
            experience_buffer: Arc::new(Mutex::new(Vec::new())),
            update_interval,
            reward_calculator: RewardCalculator::new(),
            enabled: true,
        };
        
        // 🔥 自动加载持久化的经验
        if let Err(e) = learner.load_persisted_experiences() {
            log::warn!("⚠️  Failed to load persisted experiences: {}", e);
        }
        
        learner
    }
    
    /// 禁用在线学习
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    /// 启用在线学习
    pub fn enable(&mut self) {
        self.enabled = true;
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
        
        // 🔥 持久化经验到磁盘
        drop(buffer); // 释放锁
        if let Err(e) = self.persist_experiences() {
            log::warn!("⚠️  Failed to persist experiences: {}", e);
        }
        
        // 检查是否需要更新
        let buffer_size = self.buffer_size();
        if buffer_size >= self.update_interval {
            log::info!("🎓 Triggering model update ({} experiences)", buffer_size);
            self.trigger_update()?;
        }
        
        Ok(())
    }
    
    /// 触发模型更新（使用在线PPO训练器）
    fn trigger_update(&self) -> Result<()> {
        log::info!("🚀 Starting online PPO model update...");
        
        // 获取所有经验
        let buffer = self.experience_buffer.lock().unwrap();
        
        // 逐个更新模型
        for (idx, exp) in buffer.iter().enumerate() {
            // 构建转换结果JSON
            let conversion_result = serde_json::json!({
                "features": exp.features,
                "action": {
                    "quality": exp.quality,
                    "effort": exp.effort
                },
                "original_size": 1000000, // 从reward反推（简化）
                "converted_size": ((1.0 - exp.reward) * 1000000.0) as u64,
                "ssim": 0.95 // 默认值
            });
            
            // 调用在线训练器，使用配置的模型路径
            let model_dir = self.model_path.parent()
                .unwrap_or_else(|| std::path::Path::new("models/ppo"));
            
            let output = std::process::Command::new("python3")
                .arg("scripts/online_ppo_trainer.py")
                .arg("--conversion-result")
                .arg(conversion_result.to_string())
                .arg("--model-dir")
                .arg(model_dir)
                .output()
                .context("Failed to run online PPO trainer")?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                log::error!("❌ Update failed for experience {}: {}", idx, stderr);
            }
        }
        
        log::info!("✅ Online model update complete ({} experiences)", buffer.len());
        
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
    
    /// 🔥 持久化经验到磁盘
    fn persist_experiences(&self) -> Result<()> {
        let persist_path = Path::new("models/ppo/experience_buffer.json");
        
        // 确保目录存在
        if let Some(parent) = persist_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let buffer = self.experience_buffer.lock().unwrap();
        let json = serde_json::to_string_pretty(&*buffer)?;
        std::fs::write(persist_path, json)?;
        
        log::debug!("💾 Persisted {} experiences to {:?}", buffer.len(), persist_path);
        Ok(())
    }
    
    /// 🔥 加载持久化的经验
    fn load_persisted_experiences(&mut self) -> Result<()> {
        let persist_path = Path::new("models/ppo/experience_buffer.json");
        
        if !persist_path.exists() {
            log::debug!("No persisted experiences found");
            return Ok(());
        }
        
        let json = std::fs::read_to_string(persist_path)?;
        let experiences: Vec<Experience> = serde_json::from_str(&json)?;
        
        let mut buffer = self.experience_buffer.lock().unwrap();
        *buffer = experiences;
        
        log::info!("📂 Loaded {} persisted experiences", buffer.len());
        Ok(())
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
