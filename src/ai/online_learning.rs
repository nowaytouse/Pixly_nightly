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

use crate::ai::reward_calculator::{ConversionResult, RewardCalculator};

/// 经验样本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub features: Vec<f64>,
    pub quality: u32,
    pub effort: u32,
    pub reward: f64,
    pub timestamp: u64,
    #[serde(default)]
    pub ssim: Option<f64>, // 🎯 SSIM质量分数（可选，用于后续更新）
}

/// 🎯 ML-506: 模型版本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersion {
    pub version: u32,
    pub timestamp: u64,
    pub num_experiences: usize,
    pub avg_reward: f64,
    pub path: PathBuf,
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
    /// 🎯 ML-506: 当前模型版本（使用内部可变性）
    current_version: Arc<Mutex<u32>>,
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
            current_version: Arc::new(Mutex::new(0)),  // 🎯 ML-506: 初始版本
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
        
        // 🔥 检查是否有积累的经验需要训练
        let buffer_size = self.buffer_size();
        if buffer_size >= self.update_interval {
            println!("🎓 Found {} accumulated experiences, triggering batch training...", buffer_size);
            if let Err(e) = self.trigger_update() {
                eprintln!("❌ Failed to trigger batch training: {}", e);
            }
        }
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
            ssim: None, // 初始为None，后续可通过update_last_experience_ssim更新
        };
        
        // 存储经验
        let mut buffer = self.experience_buffer.lock().expect("Mutex poisoned");
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
            self.internal_trigger_update()?;
        }
        
        Ok(())
    }
    
    /// 🎯 ML-505: 检查是否应该更新模型
    pub fn should_update(&self) -> bool {
        if !self.enabled {
            return false;
        }
        self.buffer_size() >= self.update_interval
    }
    
    /// 🎯 ML-505: 公开的触发更新接口
    pub fn trigger_update(&self) -> Result<()> {
        self.internal_trigger_update()
    }
    
    /// 触发模型更新（使用批量PPO训练器）
    fn internal_trigger_update(&self) -> Result<()> {
        println!("🚀 Starting batch PPO model update...");
        log::info!("🚀 Starting batch PPO model update...");
        
        let buffer_size = self.buffer_size();
        
        // 🔥 使用批量训练器（一次性处理所有经验）
        let model_dir = self.model_path.parent()
            .unwrap_or_else(|| std::path::Path::new("models/ppo"));
        
        let experience_file = model_dir.join("experience_buffer.json");
        
        println!("   Experience file: {:?}", experience_file);
        println!("   Model dir: {:?}", model_dir);
        println!("   Buffer size: {}", buffer_size);
        
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
            log::info!("✅ Batch model update complete ({} experiences)", buffer_size);
            
            // 🎯 ML-506: 版本管理
            // 1. 备份旧模型
            if let Err(e) = self.backup_model() {
                log::warn!("⚠️  Failed to backup model: {}", e);
            }
            
            // 2. 保存版本信息
            if let Err(e) = self.save_version_info() {
                log::warn!("⚠️  Failed to save version info: {}", e);
            }
            
            // 3. 增加版本号
            {
                let mut ver = self.current_version.lock().expect("Mutex poisoned");
                *ver += 1;
                log::info!("📈 Model version updated: v{}", *ver);
            }
            
            // 解析结果（最后一行是JSON）
            if let Some(last_line) = stdout.lines().last()
                && let Ok(result) = serde_json::from_str::<serde_json::Value>(last_line)
                    && let Some(avg_loss) = result.get("avg_loss").and_then(|v| v.as_f64()) {
                        log::info!("   Average loss: {:.4}", avg_loss);
                    }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("❌ Batch update failed: {}", stderr);
            return Err(anyhow::anyhow!("Batch PPO update failed"));
        }
        
        Ok(())
    }


    
    /// 获取当前缓冲大小
    pub fn buffer_size(&self) -> usize {
        self.experience_buffer.lock().expect("Mutex poisoned").len()
    }
    
    /// 🎯 更新最后一个经验的SSIM值
    /// 
    /// 用于在转换完成后补充SSIM质量评分
    pub fn update_last_experience_ssim(&self, ssim: f64) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        
        let mut buffer = self.experience_buffer.lock().expect("Mutex poisoned");
        if let Some(last_exp) = buffer.last_mut() {
            last_exp.ssim = Some(ssim);
            log::info!("📊 Updated last experience SSIM: {:.4}", ssim);
            
            // 持久化更新
            drop(buffer);
            if let Err(e) = self.persist_experiences() {
                log::warn!("⚠️  Failed to persist SSIM update: {}", e);
            }
        } else {
            log::warn!("⚠️  No experience to update SSIM");
        }
        
        Ok(())
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
    
    /// 🎯 ML-506: 备份当前模型
    fn backup_model(&self) -> Result<()> {
        if !self.model_path.exists() {
            return Ok(()); // 没有模型可备份
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
        log::info!("💾 Backed up model v{} to {:?}", current_ver, backup_path);
        
        Ok(())
    }
    
    /// 🎯 ML-506: 保存版本信息
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
        
        log::info!("📝 Saved version info v{} (avg_reward: {:.4})", 
                   current_ver, avg_reward);
        
        Ok(())
    }
    
    /// 🎯 ML-506: 获取当前版本
    pub fn current_version(&self) -> u32 {
        *self.current_version.lock().expect("Mutex poisoned")
    }
    
    /// 🎯 ML-506: 回滚到指定版本
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
        
        log::info!("⏮️  Rolled back to model v{}", version);
        Ok(())
    }
    
    /// 🔥 持久化经验到磁盘
    fn persist_experiences(&self) -> Result<()> {
        let persist_path = Path::new("models/ppo/experience_buffer.json");
        
        // 确保目录存在
        if let Some(parent) = persist_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let buffer = self.experience_buffer.lock().expect("Mutex poisoned");
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
        
        let mut buffer = self.experience_buffer.lock().expect("Mutex poisoned");
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
        // 使用临时目录避免加载持久化的经验
        let temp_dir = std::env::temp_dir().join("pixly_test_online_learning");
        let _ = std::fs::remove_dir_all(&temp_dir); // 清理旧数据
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
        
        // 清理
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
