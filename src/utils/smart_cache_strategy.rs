//! 🧠 intelligentcachestrategy
//! 
//! at unified_cache.rs 基础上addintelligentprediction and optimizationfeature

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// cache访问mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure AccessPattern {
 /// filepathmode
 pub path_pattern: String,
 /// 访问频率
 pub frequency: usize,
 /// 最 after 访问time
 pub last_access: u64,
 /// average间隔time（秒）
 pub avg_interval_secs: f64,
}

/// prediction性cachestrategy
pub structure PredictiveCacheStrategy {
 /// 访问modehistorical
 patterns: HashMap<String, AccessPattern>,
 /// 预热threshold（访问频率）
 preheat_threshold: usize,
 /// moderecognition窗口（秒）
 pattern_window_secs: u64,
}

impl PredictiveCacheStrategy {
 /// createnewpredictionstrategy
 pub fn new(preheat_threshold: usize, pattern_window_secs: u64) -> Self {
 Self {
 patterns: HashMap::new(),
 preheat_threshold,
 pattern_window_secs,
 }
 }
 
 /// record访问
 pub fn record_access(&mut self, path: &Path, timestamp: u64) {
 let path_str = path.to_string_lossy().to_string();
 
 let pattern = self.patterns.entry(path_str.clone()).or_insert(AccessPattern {
 path_pattern: path_str,
 frequency: 0,
 last_access: timestamp,
 avg_interval_secs: 0.0,
 });
 
 // update频率
 pattern.frequency += 1;
 
 // updateaverage间隔
 if pattern.last_access > 0 {
 let interval = timestamp.saturating_sub(pattern.last_access);
 pattern.avg_interval_secs = 
 (pattern.avg_interval_secs * (pattern.frequency - 1) as f64 + interval as f64) 
 / pattern.frequency as f64;
 }
 
 pattern.last_access = timestamp;
 }
 
 /// prediction下a次访问time
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
 
 /// getneed预热path
 pub fn get_preheat_candidates(&self, current_time: u64) -> Vec<PathBuf> {
 self.patterns
 .values()
 .filter(|p| {
 // high频访问且may很快 be 访问
 p.frequency >= self.preheat_threshold
 && p.avg_interval_secs > 0.0
 && (current_time - p.last_access) as f64 >= p.avg_interval_secs * 0.8
 })
 .map(|p| PathBuf::from(&p.path_pattern))
 .collect()
 }
 
 /// get访问statistics
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
 
 /// cleanup过期mode
 pub fn cleanup_old_patterns(&mut self, current_time: u64) -> usize {
 let window = self.pattern_window_secs;
 let before_count = self.patterns.len();
 
 self.patterns.retain(|_, pattern| {
 current_time - pattern.last_access < window
 });
 
 before_count - self.patterns.len()
 }
}

/// cachestatisticsinformation
#[derive(Debug, Serialize, Deserialize)]
pub structure CacheStatistics {
 /// 总mode数
 pub total_patterns: usize,
 /// 总访问count
 pub total_accesses: usize,
 /// high频mode数
 pub high_freq_patterns: usize,
 /// 预热threshold
 pub preheat_threshold: usize,
}

/// cachepriority级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CachePriority {
 /// lowpriority级
 Low = 1,
 /// 正常priority级
 Normal = 2,
 /// highpriority级
 High = 3,
 /// 关键priority级
 Critical = 4,
}

impl CachePriority {
 /// based on访问频率calculationpriority级
 pub fn from_frequency(frequency: usize) -> Self {
 match frequency {
 0..=2 => Self::Low,
 3..=10 => Self::Normal,
 11..=50 => Self::High,
 _ => Self::Critical,
 }
 }
 
 /// get权重（forLRUcalculation）
 pub fn weight(&self) -> f64 {
 match self {
 Self::Low => 1.0,
 Self::Normal => 2.0,
 Self::High => 4.0,
 Self::Critical => 8.0,
 }
 }
}

/// intelligentcache决策
pub structure SmartCacheDecision {
 /// is否shouldcache
 pub should_cache: bool,
 /// cachepriority级
 pub priority: CachePriority,
 /// recommended TTL（秒）
 pub recommended_ttl: Option<u64>,
 /// 决策原因
 pub reason: String,
}

/// intelligentcache决策引擎
pub structure CacheDecisionEngine {
 strategy: PredictiveCacheStrategy,
}

impl CacheDecisionEngine {
 /// createnew决策引擎
 pub fn new() -> Self {
 Self {
 strategy: PredictiveCacheStrategy::new(
 3, // 访问3次以上consider预热
 7 * 24 * 3600, // 保留7天mode
 ),
 }
 }
 
 /// 决定is否shouldcache
 pub fn decide(&mut self, 
 path: &Path, 
 file_size: u64,
 current_time: u64) -> SmartCacheDecision {
 // record访问
 self.strategy.record_access(path, current_time);
 
 // get访问mode
 let path_str = path.to_string_lossy().to_string();
 let pattern = self.strategy.patterns.get(&path_str);
 
 match pattern {
 Some(p) => {
 let priority = CachePriority::from_frequency(p.frequency);
 
 // 大filelowpriority级cache
 let adjusted_priority = if file_size > 10 * 1024 * 1024 {
 match priority {
 CachePriority::Critical => CachePriority::High,
 CachePriority::High => CachePriority::Normal,
 other => other,
 }
 } else {
 priority
 };
 
 // recommendedTTL
 let ttl = if p.avg_interval_secs > 0.0 {
 Some((p.avg_interval_secs * 1.5) as u64)
 } else {
 Some(3600) // default1小when
 };
 
 SmartCacheDecision {
 should_cache: p.frequency >= 1, // 修改：from第once访问就开始缓存
 priority: adjusted_priority,
 recommended_ttl: ttl,
 reason: format!(
 "Frequency: {}, Priority: {:?}, Avg interval: {:.0}s",
 p.frequency, adjusted_priority, p.avg_interval_secs
 ),
 }
 }
 None => {
 // 首次访问，defaultcache但lowpriority级
 SmartCacheDecision {
 should_cache: true,
 priority: CachePriority::Low,
 recommended_ttl: Some(3600),
 reason: "First access, low priority cache".to_string(),
 }
 }
 }
 }
 
 /// get预热suggested
 pub fn get_preheat_suggestions(&self, current_time: u64) -> Vec<PathBuf> {
 self.strategy.get_preheat_candidates(current_time)
 }
 
 /// getstatistics
 pub fn get_statistics(&self) -> CacheStatistics {
 self.strategy.get_statistics()
 }
 
 /// cleanup旧mode
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
 
 // recordmulti次访问
 strategy.record_access(path, 1000);
 strategy.record_access(path, 2000);
 strategy.record_access(path, 3000);
 
 // prediction下次访问
 let next = strategy.predict_next_access(path);
 assert!(next.is_some());
 assert!(next.unwrap() > 3000);
 }
 
 #[test]
 fn test_decision_engine() {
 let mut engine = CacheDecisionEngine::new();
 let path = Path::new("test.png");
 
 // 首次访问 - return Nonebranch，shouldcache但lowpriority级
 let decision1 = engine.decide(path, 1024, 1000);
 assert!(decision1.should_cache);
 assert_eq!(decision1.priority, CachePriority::Low);
 
 // second次访问 - frequency=2，shouldcache
 let decision2 = engine.decide(path, 1024, 2000);
 assert!(decision2.should_cache);
 
 // multi次访问 after - should提升priority级
 engine.decide(path, 1024, 3000);
 let decision4 = engine.decide(path, 1024, 4000);
 
 assert!(decision4.should_cache);
 assert!(decision4.priority > CachePriority::Low);
 }
 
 #[test]
 fn test_large_file_priority_adjustment() {
 let mut engine = CacheDecisionEngine::new();
 let path = Path::new("large.png");
 
 // simulatedhigh频访问
 for i in 0..20 {
 engine.decide(path, 20 * 1024 * 1024, 1000 + i * 100);
 }
 
 let decision = engine.decide(path, 20 * 1024 * 1024, 10000);
 
 // 大fileshoulddowngradepriority级
 assert!(decision.priority < CachePriority::Critical);
 }
 
 #[test]
 fn test_cleanup_old_patterns() {
 let mut strategy = PredictiveCacheStrategy::new(3, 1000);
 
 strategy.record_access(Path::new("old.png"), 1000);
 strategy.record_access(Path::new("new.png"), 2600); // 修正：ensure窗口内 (3500-2600=900 < 1000)
 
 let removed = strategy.cleanup_old_patterns(3500);
 
 assert_eq!(removed, 1); // old.png 应该be删除（3500-1000=2500 >= 1000）
 assert_eq!(strategy.patterns.len(), 1); // 只剩 new.png
 }
}
