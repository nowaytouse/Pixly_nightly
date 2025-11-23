//! 🧠 智能缓存策略
//! 
//! 在 unified_cache.rs 基础上增加智能预测和优化功能

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// 缓存访问模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPattern {
    /// 文件路径模式
    pub path_pattern: String,
    /// 访问频率
    pub frequency: usize,
    /// 最后访问时间
    pub last_access: u64,
    /// 平均间隔时间（秒）
    pub avg_interval_secs: f64,
}

/// 预测性缓存策略
pub struct PredictiveCacheStrategy {
    /// 访问模式历史
    patterns: HashMap<String, AccessPattern>,
    /// 预热阈值（访问频率）
    preheat_threshold: usize,
    /// 模式识别窗口（秒）
    pattern_window_secs: u64,
}

impl PredictiveCacheStrategy {
    /// 创建新的预测策略
    pub fn new(preheat_threshold: usize, pattern_window_secs: u64) -> Self {
        Self {
            patterns: HashMap::new(),
            preheat_threshold,
            pattern_window_secs,
        }
    }
    
    /// 记录访问
    pub fn record_access(&mut self, path: &Path, timestamp: u64) {
        let path_str = path.to_string_lossy().to_string();
        
        let pattern = self.patterns.entry(path_str.clone()).or_insert(AccessPattern {
            path_pattern: path_str,
            frequency: 0,
            last_access: timestamp,
            avg_interval_secs: 0.0,
        });
        
        // 更新频率
        pattern.frequency += 1;
        
        // 更新平均间隔
        if pattern.last_access > 0 {
            let interval = timestamp.saturating_sub(pattern.last_access);
            pattern.avg_interval_secs = 
                (pattern.avg_interval_secs * (pattern.frequency - 1) as f64 + interval as f64) 
                / pattern.frequency as f64;
        }
        
        pattern.last_access = timestamp;
    }
    
    /// 预测下一次访问时间
    pub fn predict_next_access(&self, path: &Path) -> Option<u64> {
        let path_str = path.to_string_lossy().to_string();
        
        self.patterns.get(&path_str).map(|pattern| {
            if pattern.frequency >= 2 {
                pattern.last_access + pattern.avg_interval_secs as u64
            } else {
                u64::MAX
            }
        })
    }
    
    /// 获取需要预热的路径
    pub fn get_preheat_candidates(&self, current_time: u64) -> Vec<PathBuf> {
        self.patterns
            .values()
            .filter(|p| {
                // 高频访问且可能很快被访问
                p.frequency >= self.preheat_threshold
                    && p.avg_interval_secs > 0.0
                    && (current_time - p.last_access) as f64 >= p.avg_interval_secs * 0.8
            })
            .map(|p| PathBuf::from(&p.path_pattern))
            .collect()
    }
    
    /// 获取访问统计
    pub fn get_statistics(&self) -> CacheStatistics {
        let total_accesses: usize = self.patterns.values().map(|p| p.frequency).sum();
        let high_freq_count = self.patterns.values()
            .filter(|p| p.frequency >= self.preheat_threshold)
            .count();
        
        CacheStatistics {
            total_patterns: self.patterns.len(),
            total_accesses,
            high_freq_patterns: high_freq_count,
            preheat_threshold: self.preheat_threshold,
        }
    }
    
    /// 清理过期模式
    pub fn cleanup_old_patterns(&mut self, current_time: u64) -> usize {
        let window = self.pattern_window_secs;
        let before_count = self.patterns.len();
        
        self.patterns.retain(|_, pattern| {
            current_time - pattern.last_access < window
        });
        
        before_count - self.patterns.len()
    }
}

/// 缓存统计信息
#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStatistics {
    /// 总模式数
    pub total_patterns: usize,
    /// 总访问次数
    pub total_accesses: usize,
    /// 高频模式数
    pub high_freq_patterns: usize,
    /// 预热阈值
    pub preheat_threshold: usize,
}

/// 缓存优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CachePriority {
    /// 低优先级
    Low = 1,
    /// 正常优先级
    Normal = 2,
    /// 高优先级
    High = 3,
    /// 关键优先级
    Critical = 4,
}

impl CachePriority {
    /// 根据访问频率计算优先级
    pub fn from_frequency(frequency: usize) -> Self {
        match frequency {
            0..=2 => Self::Low,
            3..=10 => Self::Normal,
            11..=50 => Self::High,
            _ => Self::Critical,
        }
    }
    
    /// 获取权重（用于LRU计算）
    pub fn weight(&self) -> f64 {
        match self {
            Self::Low => 1.0,
            Self::Normal => 2.0,
            Self::High => 4.0,
            Self::Critical => 8.0,
        }
    }
}

/// 智能缓存决策
pub struct SmartCacheDecision {
    /// 是否应该缓存
    pub should_cache: bool,
    /// 缓存优先级
    pub priority: CachePriority,
    /// 推荐的TTL（秒）
    pub recommended_ttl: Option<u64>,
    /// 决策原因
    pub reason: String,
}

/// 智能缓存决策引擎
pub struct CacheDecisionEngine {
    strategy: PredictiveCacheStrategy,
}

impl CacheDecisionEngine {
    /// 创建新的决策引擎
    pub fn new() -> Self {
        Self {
            strategy: PredictiveCacheStrategy::new(
                3,  // 访问3次以上才考虑预热
                7 * 24 * 3600,  // 保留7天的模式
            ),
        }
    }
    
    /// 决定是否应该缓存
    pub fn decide(&mut self, 
                  path: &Path, 
                  file_size: u64,
                  current_time: u64) -> SmartCacheDecision {
        // 记录访问
        self.strategy.record_access(path, current_time);
        
        // 获取访问模式
        let path_str = path.to_string_lossy().to_string();
        let pattern = self.strategy.patterns.get(&path_str);
        
        match pattern {
            Some(p) => {
                let priority = CachePriority::from_frequency(p.frequency);
                
                // 大文件低优先级缓存
                let adjusted_priority = if file_size > 10 * 1024 * 1024 {
                    match priority {
                        CachePriority::Critical => CachePriority::High,
                        CachePriority::High => CachePriority::Normal,
                        other => other,
                    }
                } else {
                    priority
                };
                
                // 推荐TTL
                let ttl = if p.avg_interval_secs > 0.0 {
                    Some((p.avg_interval_secs * 1.5) as u64)
                } else {
                    Some(3600) // 默认1小时
                };
                
                SmartCacheDecision {
                    should_cache: p.frequency >= 1,  // 修改：从第一次访问就开始缓存
                    priority: adjusted_priority,
                    recommended_ttl: ttl,
                    reason: format!(
                        "Frequency: {}, Priority: {:?}, Avg interval: {:.0}s",
                        p.frequency, adjusted_priority, p.avg_interval_secs
                    ),
                }
            }
            None => {
                // 首次访问，默认缓存但低优先级
                SmartCacheDecision {
                    should_cache: true,
                    priority: CachePriority::Low,
                    recommended_ttl: Some(3600),
                    reason: "First access, low priority cache".to_string(),
                }
            }
        }
    }
    
    /// 获取预热建议
    pub fn get_preheat_suggestions(&self, current_time: u64) -> Vec<PathBuf> {
        self.strategy.get_preheat_candidates(current_time)
    }
    
    /// 获取统计
    pub fn get_statistics(&self) -> CacheStatistics {
        self.strategy.get_statistics()
    }
    
    /// 清理旧模式
    pub fn cleanup(&mut self, current_time: u64) -> usize {
        self.strategy.cleanup_old_patterns(current_time)
    }
}

impl Default for CacheDecisionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_priority_from_frequency() {
        assert_eq!(CachePriority::from_frequency(1), CachePriority::Low);
        assert_eq!(CachePriority::from_frequency(5), CachePriority::Normal);
        assert_eq!(CachePriority::from_frequency(20), CachePriority::High);
        assert_eq!(CachePriority::from_frequency(100), CachePriority::Critical);
    }
    
    #[test]
    fn test_cache_priority_weight() {
        assert_eq!(CachePriority::Low.weight(), 1.0);
        assert_eq!(CachePriority::Critical.weight(), 8.0);
    }
    
    #[test]
    fn test_predictive_strategy() {
        let mut strategy = PredictiveCacheStrategy::new(3, 86400);
        let path = Path::new("test.png");
        
        // 记录多次访问
        strategy.record_access(path, 1000);
        strategy.record_access(path, 2000);
        strategy.record_access(path, 3000);
        
        // 预测下次访问
        let next = strategy.predict_next_access(path);
        assert!(next.is_some());
        assert!(next.unwrap() > 3000);
    }
    
    #[test]
    fn test_decision_engine() {
        let mut engine = CacheDecisionEngine::new();
        let path = Path::new("test.png");
        
        // 首次访问 - 返回None分支，应该缓存但低优先级
        let decision1 = engine.decide(path, 1024, 1000);
        assert!(decision1.should_cache);
        assert_eq!(decision1.priority, CachePriority::Low);
        
        // 第二次访问 - frequency=2，应该缓存
        let decision2 = engine.decide(path, 1024, 2000);
        assert!(decision2.should_cache);
        
        // 多次访问后 - 应该提升优先级
        engine.decide(path, 1024, 3000);
        let decision4 = engine.decide(path, 1024, 4000);
        
        assert!(decision4.should_cache);
        assert!(decision4.priority > CachePriority::Low);
    }
    
    #[test]
    fn test_large_file_priority_adjustment() {
        let mut engine = CacheDecisionEngine::new();
        let path = Path::new("large.png");
        
        // 模拟高频访问
        for i in 0..20 {
            engine.decide(path, 20 * 1024 * 1024, 1000 + i * 100);
        }
        
        let decision = engine.decide(path, 20 * 1024 * 1024, 10000);
        
        // 大文件应该降级优先级
        assert!(decision.priority < CachePriority::Critical);
    }
    
    #[test]
    fn test_cleanup_old_patterns() {
        let mut strategy = PredictiveCacheStrategy::new(3, 1000);
        
        strategy.record_access(Path::new("old.png"), 1000);
        strategy.record_access(Path::new("new.png"), 2600);  // 修正：确保在窗口内 (3500-2600=900 < 1000)
        
        let removed = strategy.cleanup_old_patterns(3500);
        
        assert_eq!(removed, 1);  // old.png 应该被删除（3500-1000=2500 >= 1000）
        assert_eq!(strategy.patterns.len(), 1);  // 只剩 new.png
    }
}
