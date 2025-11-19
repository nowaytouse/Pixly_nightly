// src/online_learner_manager.rs
//! 🎓 在线学习器管理器 - 全局单例
//! 
//! 功能：
//! - 全局单例OnlineLearner
//! - 跨转换会话持久化经验缓冲
//! - 自动触发模型更新

use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use once_cell::sync::Lazy;
use crate::online_learning::OnlineLearner;
use crate::reward_calculator::ConversionResult;

/// 全局在线学习器实例
static GLOBAL_LEARNER: Lazy<Arc<Mutex<OnlineLearner>>> = Lazy::new(|| {
    // 🔥 默认禁用，需要显式启用
    let mut learner = OnlineLearner::new(
        PathBuf::from("models/ppo/actor_online.pth"),
        10  // 每10次转换更新一次模型（降低阈值以便测试）
    );
    learner.disable();  // 🔥 默认禁用
    Arc::new(Mutex::new(learner))
});

/// 全局启用标志
static ENABLED: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| {
    Arc::new(Mutex::new(false))
});

/// 在线学习器管理器
pub struct OnlineLearnerManager;

impl OnlineLearnerManager {
    /// 启用在线学习
    pub fn enable() {
        let mut enabled = ENABLED.lock().unwrap();
        *enabled = true;
        
        let mut learner = GLOBAL_LEARNER.lock().unwrap();
        learner.enable();
        
        log::info!("🎓 Online learning ENABLED");
    }
    
    /// 检查是否启用
    pub fn is_enabled() -> bool {
        *ENABLED.lock().unwrap()
    }
    
    /// 记录转换经验
    pub fn record_conversion(
        features: Vec<f64>,
        quality: u32,
        effort: u32,
        result: ConversionResult,
    ) -> anyhow::Result<()> {
        // 🔥 检查是否启用
        if !Self::is_enabled() {
            return Ok(());  // 静默跳过
        }
        
        let learner = GLOBAL_LEARNER.lock().unwrap();
        learner.record_conversion(features, quality, effort, result)?;
        
        let buffer_size = learner.buffer_size();
        log::info!("📝 Experience recorded (global buffer: {})", buffer_size);
        
        // 🎯 ML-505: 自动触发机制
        // 当缓冲区达到阈值时，自动触发模型更新
        if learner.should_update() {
            log::info!("🎓 Auto-triggering model update ({} experiences)", buffer_size);
            
            // 释放锁后执行更新（避免死锁）
            drop(learner);
            
            match Self::auto_update() {
                Ok(_) => log::info!("✅ Auto-update completed successfully"),
                Err(e) => {
                    log::error!("❌ Auto-update failed: {}", e);
                    // 不返回错误，避免阻塞转换流程
                }
            }
        }
        
        Ok(())
    }
    
    /// 🎯 ML-505: 自动更新模型
    fn auto_update() -> anyhow::Result<()> {
        let learner = GLOBAL_LEARNER.lock().unwrap();
        learner.trigger_update()
    }
    
    /// 获取当前缓冲区大小
    pub fn buffer_size() -> usize {
        GLOBAL_LEARNER.lock().unwrap().buffer_size()
    }
    
    /// 手动触发模型更新
    pub fn manual_update() -> anyhow::Result<()> {
        let learner = GLOBAL_LEARNER.lock().unwrap();
        learner.manual_update()
    }
    
    /// 禁用在线学习
    pub fn disable() {
        let mut learner = GLOBAL_LEARNER.lock().unwrap();
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
