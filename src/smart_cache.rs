//! 智能缓存系统
//! 
//! 提供高性能缓存功能：
//! - LRU缓存策略
//! - TTL过期管理
//! - 缓存统计
//! - 并发安全

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::time::{SystemTime, Duration, Instant};
use std::sync::{Arc, RwLock, Mutex};
use std::fs;
use serde::{Deserialize, Serialize};

/// 缓存条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub source_path: PathBuf,
    pub cache_path: PathBuf,
    pub created_at: SystemTime,
    pub last_accessed: SystemTime,
    pub access_count: u64,
    pub size_bytes: u64,
}

/// 缓存统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_entries: u64,
    pub hits: u64,
    pub misses: u64,
    pub total_size_bytes: u64,
    pub hit_rate_percent: f64,
}

impl CacheStats {
    fn new() -> Self {
        Self {
            total_entries: 0,
            hits: 0,
            misses: 0,
            total_size_bytes: 0,
            hit_rate_percent: 0.0,
        }
    }
    
    fn update(&mut self, entries: &HashMap<String, CacheEntry>) {
        self.total_entries = entries.len() as u64;
        self.total_size_bytes = entries.values().map(|e| e.size_bytes).sum();
        
        let total_requests = self.hits + self.misses;
        self.hit_rate_percent = if total_requests > 0 {
            (self.hits as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };
    }
}

/// 缓存配置
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub cache_dir: PathBuf,
    pub max_size_bytes: u64,
    pub max_entries: u64,
    pub ttl: Duration,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            cache_dir: PathBuf::from("./cache"),
            max_size_bytes: 1024 * 1024 * 1024, // 1GB
            max_entries: 10000,
            ttl: Duration::from_secs(7 * 24 * 3600), // 7天
        }
    }
}

/// 智能缓存系统
pub struct SmartCache {
    config: CacheConfig,
    entries: Arc<RwLock<HashMap<String, CacheEntry>>>,
    stats: Arc<RwLock<CacheStats>>,
    last_cleanup: Arc<Mutex<Instant>>,
}

impl SmartCache {
    pub fn new(config: CacheConfig) -> Result<Self> {
        fs::create_dir_all(&config.cache_dir)?;
        
        Ok(Self {
            config,
            entries: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(CacheStats::new())),
            last_cleanup: Arc::new(Mutex::new(Instant::now())),
        })
    }
    
    /// 生成缓存键
    pub fn generate_key(&self, source_path: &Path, params: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        source_path.hash(&mut hasher);
        params.hash(&mut hasher);
        
        format!("{:x}", hasher.finish())
    }
    
    /// 获取缓存
    pub fn get(&self, key: &str) -> Option<PathBuf> {
        let mut entries = self.entries.write().unwrap();
        let mut stats = self.stats.write().unwrap();
        
        if let Some(entry) = entries.get_mut(key) {
            // 检查是否过期
            if let Ok(elapsed) = entry.created_at.elapsed() && elapsed > self.config.ttl {
                // 过期，删除
                entries.remove(key);
                stats.misses += 1;
                return None;
            }
            
            // 更新访问信息
            entry.last_accessed = SystemTime::now();
            entry.access_count += 1;
            
            stats.hits += 1;
            Some(entry.cache_path.clone())
        } else {
            stats.misses += 1;
            None
        }
    }
    
    /// 添加缓存
    pub fn put(&self, key: String, source_path: PathBuf, cache_path: PathBuf, size_bytes: u64) -> Result<()> {
        let mut entries = self.entries.write().unwrap();
        
        // 检查是否需要清理
        self.cleanup_if_needed(&mut entries)?;
        
        let entry = CacheEntry {
            key: key.clone(),
            source_path,
            cache_path,
            created_at: SystemTime::now(),
            last_accessed: SystemTime::now(),
            access_count: 0,
            size_bytes,
        };
        
        entries.insert(key, entry);
        
        let mut stats = self.stats.write().unwrap();
        stats.update(&entries);
        
        Ok(())
    }
    
    /// 清理过期缓存
    fn cleanup_if_needed(&self, entries: &mut HashMap<String, CacheEntry>) -> Result<()> {
        let mut last_cleanup = self.last_cleanup.lock().unwrap();
        
        // 每小时清理一次
        if last_cleanup.elapsed() < Duration::from_secs(3600) {
            return Ok(());
        }
        
        let now = SystemTime::now();
        let mut to_remove = Vec::new();
        
        // 找出过期的条目
        for (key, entry) in entries.iter() {
            if let Ok(elapsed) = now.duration_since(entry.created_at) && elapsed > self.config.ttl {
                to_remove.push(key.clone());
            }
        }
        
        // 删除过期条目
        for key in &to_remove {
            if let Some(entry) = entries.remove(key) {
                let _ = fs::remove_file(&entry.cache_path);
            }
        }
        
        // 如果缓存太大，删除最少使用的
        let total_size: u64 = entries.values().map(|e| e.size_bytes).sum();
        if total_size > self.config.max_size_bytes || entries.len() > self.config.max_entries as usize {
            let mut sorted: Vec<_> = entries.iter().map(|(k, e)| (k.clone(), e.clone())).collect();
            sorted.sort_by_key(|(_, e)| e.access_count);
            
            let to_remove_count = (entries.len() as f64 * 0.2) as usize; // 删除20%
            for (key, entry) in sorted.iter().take(to_remove_count) {
                let _ = fs::remove_file(&entry.cache_path);
                entries.remove(key);
            }
        }
        
        *last_cleanup = Instant::now();
        Ok(())
    }
    
    /// 获取统计信息
    pub fn get_stats(&self) -> CacheStats {
        let entries = self.entries.read().unwrap();
        let mut stats = self.stats.write().unwrap();
        stats.update(&entries);
        stats.clone()
    }
    
    /// 清空缓存
    pub fn clear(&self) -> Result<()> {
        let mut entries = self.entries.write().unwrap();
        
        for entry in entries.values() {
            let _ = fs::remove_file(&entry.cache_path);
        }
        
        entries.clear();
        
        let mut stats = self.stats.write().unwrap();
        *stats = CacheStats::new();
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_creation() {
        let config = CacheConfig::default();
        let cache = SmartCache::new(config);
        assert!(cache.is_ok());
    }
    
    #[test]
    fn test_cache_key_generation() {
        let config = CacheConfig::default();
        let cache = SmartCache::new(config).unwrap();
        
        let key1 = cache.generate_key(Path::new("test.jpg"), "quality=90");
        let key2 = cache.generate_key(Path::new("test.jpg"), "quality=90");
        let key3 = cache.generate_key(Path::new("test.jpg"), "quality=80");
        
        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }
    
    #[test]
    fn test_cache_stats() {
        let config = CacheConfig::default();
        let cache = SmartCache::new(config).unwrap();
        
        let stats = cache.get_stats();
        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
    }
}
