//! 🗄️ 统一缓存系统
//! 
//! 提供高性能的转换结果缓存：
//! - 基于内容哈希的缓存键
//! - LRU淘汰策略
//! - 磁盘持久化
//! - 自动过期清理

use std::path::PathBuf;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use anyhow::Result;
use serde::{Serialize, Deserialize};

/// 缓存条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// 输入文件哈希
    pub input_hash: String,
    /// 转换参数哈希
    pub params_hash: String,
    /// 输出文件路径
    pub output_path: PathBuf,
    /// 创建时间
    pub created_at: u64,
    /// 最后访问时间
    pub last_accessed: u64,
    /// 访问次数
    pub access_count: u64,
    /// 输出文件大小
    pub output_size: u64,
}

/// 缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 缓存目录
    pub cache_dir: PathBuf,
    /// 最大缓存大小（MB）
    pub max_size_mb: u64,
    /// 条目过期时间（天）
    pub expire_days: u64,
    /// 启用LRU淘汰
    pub enable_lru: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            cache_dir: PathBuf::from("cache"),
            max_size_mb: 1024, // 1GB
            expire_days: 30,
            enable_lru: true,
        }
    }
}

/// 统一缓存管理器
pub struct UnifiedCache {
    config: CacheConfig,
    entries: HashMap<String, CacheEntry>,
    total_size: u64,
}

impl UnifiedCache {
    /// 创建新的缓存管理器
    pub fn new(config: CacheConfig) -> Result<Self> {
        // 确保缓存目录存在
        std::fs::create_dir_all(&config.cache_dir)?;
        
        let mut cache = Self {
            config,
            entries: HashMap::new(),
            total_size: 0,
        };
        
        // 加载现有缓存
        cache.load_cache()?;
        
        Ok(cache)
    }

    /// 生成缓存键
    pub fn generate_key(input_hash: &str, params_hash: &str) -> String {
        format!("{}_{}", input_hash, params_hash)
    }

    /// 查找缓存
    pub fn get(&mut self, input_hash: &str, params_hash: &str) -> Option<CacheEntry> {
        let key = Self::generate_key(input_hash, params_hash);
        
        if let Some(entry) = self.entries.get_mut(&key) {
            // 更新访问信息
            entry.last_accessed = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs();
            entry.access_count += 1;
            
            // 检查输出文件是否存在
            if entry.output_path.exists() {
                return Some(entry.clone());
            } else {
                // 文件已被删除，移除缓存条目
                self.entries.remove(&key);
                return None;
            }
        }
        
        None
    }

    /// 添加缓存条目
    pub fn put(&mut self, entry: CacheEntry) -> Result<()> {
        let key = Self::generate_key(&entry.input_hash, &entry.params_hash);
        
        // 检查是否需要淘汰
        if self.config.enable_lru {
            self.evict_if_needed(entry.output_size)?;
        }
        
        self.total_size += entry.output_size;
        self.entries.insert(key, entry);
        
        // 持久化
        self.save_cache()?;
        
        Ok(())
    }

    /// LRU淘汰
    fn evict_if_needed(&mut self, new_size: u64) -> Result<()> {
        let max_size = self.config.max_size_mb * 1024 * 1024;
        
        while self.total_size + new_size > max_size && !self.entries.is_empty() {
            // 找到最少使用的条目
            let lru_key = self.entries
                .iter()
                .min_by_key(|(_, e)| e.last_accessed)
                .map(|(k, _)| k.clone());
            
            if let Some(key) = lru_key {
                if let Some(entry) = self.entries.remove(&key) {
                    self.total_size -= entry.output_size;
                    // 删除输出文件
                    let _ = std::fs::remove_file(&entry.output_path);
                    log::info!("🗑️ Evicted cache entry: {}", key);
                }
            } else {
                break;
            }
        }
        
        Ok(())
    }

    /// 清理过期条目
    pub fn cleanup_expired(&mut self) -> Result<usize> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        let expire_threshold = now - (self.config.expire_days * 24 * 3600);
        
        let mut removed = 0;
        let expired_keys: Vec<String> = self.entries
            .iter()
            .filter(|(_, e)| e.created_at < expire_threshold)
            .map(|(k, _)| k.clone())
            .collect();
        
        for key in expired_keys {
            if let Some(entry) = self.entries.remove(&key) {
                self.total_size -= entry.output_size;
                let _ = std::fs::remove_file(&entry.output_path);
                removed += 1;
            }
        }
        
        if removed > 0 {
            self.save_cache()?;
            log::info!("🧹 Cleaned up {} expired cache entries", removed);
        }
        
        Ok(removed)
    }

    /// 加载缓存索引
    fn load_cache(&mut self) -> Result<()> {
        let index_path = self.config.cache_dir.join("cache_index.json");
        
        if !index_path.exists() {
            return Ok(());
        }
        
        let data = std::fs::read_to_string(&index_path)?;
        let entries: HashMap<String, CacheEntry> = serde_json::from_str(&data)?;
        
        // 验证条目并计算总大小
        for (key, entry) in entries {
            if entry.output_path.exists() {
                self.total_size += entry.output_size;
                self.entries.insert(key, entry);
            }
        }
        
        log::info!("📦 Loaded {} cache entries ({:.2} MB)", 
                  self.entries.len(), 
                  self.total_size as f64 / (1024.0 * 1024.0));
        
        Ok(())
    }

    /// 保存缓存索引
    fn save_cache(&self) -> Result<()> {
        let index_path = self.config.cache_dir.join("cache_index.json");
        let data = serde_json::to_string_pretty(&self.entries)?;
        std::fs::write(&index_path, data)?;
        Ok(())
    }

    /// 获取缓存统计
    pub fn get_stats(&self) -> CacheStats {
        CacheStats {
            total_entries: self.entries.len(),
            total_size_mb: self.total_size as f64 / (1024.0 * 1024.0),
            max_size_mb: self.config.max_size_mb as f64,
            usage_percent: (self.total_size as f64 / (self.config.max_size_mb as f64 * 1024.0 * 1024.0)) * 100.0,
        }
    }

    /// 清空缓存
    pub fn clear(&mut self) -> Result<()> {
        for entry in self.entries.values() {
            let _ = std::fs::remove_file(&entry.output_path);
        }
        
        self.entries.clear();
        self.total_size = 0;
        self.save_cache()?;
        
        log::info!("🗑️ Cache cleared");
        Ok(())
    }
}

/// 缓存统计信息
#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_entries: usize,
    pub total_size_mb: f64,
    pub max_size_mb: f64,
    pub usage_percent: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_key_generation() {
        let key = UnifiedCache::generate_key("abc123", "def456");
        assert_eq!(key, "abc123_def456");
    }

    #[test]
    fn test_cache_stats() {
        let config = CacheConfig {
            cache_dir: PathBuf::from("test_cache"),
            max_size_mb: 100,
            expire_days: 7,
            enable_lru: true,
        };
        
        let cache = UnifiedCache::new(config).unwrap();
        let stats = cache.get_stats();
        
        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.max_size_mb, 100.0);
    }
}
