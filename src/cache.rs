// 🚀 统一智能缓存系统
// 从 @archive/rust_broken/src/converter/unified_cache.rs 提取并增强
// 
// 核心功能:
// - 智能缓存键生成（基于内容+参数）
// - LRU + TTL 双重过期策略
// - 缓存命中率统计和监控
// - 并发安全访问
// - 自动清理过期缓存

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::time::{SystemTime, Duration, Instant};
use std::sync::{Arc, RwLock, Mutex};
use std::fs;
use serde::{Deserialize, Serialize};

/// 缓存优先级
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CachePriority {
    Low,
    Normal,
    High,
    Critical,
}

/// 缓存条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub source_path: PathBuf,
    pub cache_path: PathBuf,
    pub params_hash: String,
    pub created_at: SystemTime,
    pub last_accessed: SystemTime,
    pub access_count: u64,
    pub size_bytes: u64,
    pub priority: CachePriority,
    pub checksum: String,
}

/// 缓存统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_entries: u64,
    pub hits: u64,
    pub misses: u64,
    pub total_size_bytes: u64,
    pub avg_access_count: f64,
    pub oldest_entry: Option<SystemTime>,
    pub newest_entry: Option<SystemTime>,
    pub hit_rate_percent: f64,
    pub cleanup_count: u64,
}

impl CacheStats {
    fn new() -> Self {
        Self {
            total_entries: 0,
            hits: 0,
            misses: 0,
            total_size_bytes: 0,
            avg_access_count: 0.0,
            oldest_entry: None,
            newest_entry: None,
            hit_rate_percent: 0.0,
            cleanup_count: 0,
        }
    }

    fn update(&mut self, entries: &HashMap<String, CacheEntry>) {
        self.total_entries = entries.len() as u64;
        self.total_size_bytes = entries.values().map(|e| e.size_bytes).sum();
        
        if self.total_entries > 0 {
            self.avg_access_count = entries.values().map(|e| e.access_count).sum::<u64>() as f64 / self.total_entries as f64;
        }

        self.oldest_entry = entries.values().map(|e| e.created_at).min();
        self.newest_entry = entries.values().map(|e| e.created_at).max();

        let total_requests = self.hits + self.misses;
        self.hit_rate_percent = if total_requests > 0 {
            (self.hits as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };
    }
}

/// 缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub cache_dir: PathBuf,
    pub max_size_bytes: u64,
    pub max_entries: u64,
    pub ttl: Duration,
    pub cleanup_interval: Duration,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            cache_dir: PathBuf::from("./cache"),
            max_size_bytes: 1024 * 1024 * 1024, // 1GB
            max_entries: 10000,
            ttl: Duration::from_secs(7 * 24 * 3600), // 7天
            cleanup_interval: Duration::from_secs(3600), // 1小时
        }
    }
}

/// 智能缓存系统
#[derive(Debug)]
pub struct SmartCache {
    config: CacheConfig,
    entries: Arc<RwLock<HashMap<String, CacheEntry>>>,
    stats: Arc<RwLock<CacheStats>>,
    last_cleanup: Arc<Mutex<Instant>>,
}

impl SmartCache {
    /// 创建新的缓存系统
    pub fn new(config: CacheConfig) -> Result<Self> {
        fs::create_dir_all(&config.cache_dir)?;

        let cache = Self {
            config,
            entries: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(CacheStats::new())),
            last_cleanup: Arc::new(Mutex::new(Instant::now())),
        };

        cache.load_cache_index()?;
        Ok(cache)
    }

    /// 生成智能缓存键
    pub fn generate_cache_key(&self, source_path: &Path, params: &str) -> Result<String> {
        let mut hasher = blake3::Hasher::new();
        
        hasher.update(source_path.to_string_lossy().as_bytes());
        
        let metadata = fs::metadata(source_path)?;
        let file_size = metadata.len();
        hasher.update(&file_size.to_le_bytes());

        // 智能采样策略
        if file_size <= 10 * 1024 * 1024 {
            let content = fs::read(source_path)?;
            hasher.update(&content);
        } else {
            use std::io::{Read, Seek, SeekFrom};
            let mut file = fs::File::open(source_path)?;
            let sample_size = 8192;
            let mut buffer = vec![0u8; sample_size];

            file.read_exact(&mut buffer).ok();
            hasher.update(&buffer);

            file.seek(SeekFrom::Start(file_size / 2)).ok();
            file.read_exact(&mut buffer).ok();
            hasher.update(&buffer);

            file.seek(SeekFrom::End(-(sample_size as i64))).ok();
            file.read_exact(&mut buffer).ok();
            hasher.update(&buffer);
        }

        hasher.update(params.as_bytes());
        Ok(hex::encode(hasher.finalize().as_bytes()))
    }

    /// 获取缓存
    pub fn get(&self, key: &str) -> Option<PathBuf> {
        let mut entries = self.entries.write().unwrap();
        let mut stats = self.stats.write().unwrap();

        if let Some(entry) = entries.get_mut(key) {
            if entry.created_at.elapsed().unwrap_or(Duration::MAX) > self.config.ttl {
                self.remove_cache_file(&entry.cache_path);
                entries.remove(key);
                stats.misses += 1;
                return None;
            }

            entry.last_accessed = SystemTime::now();
            entry.access_count += 1;
            let cache_path = entry.cache_path.clone();
            
            stats.hits += 1;
            drop(entries);
            let entries_for_stats = self.entries.read().unwrap();
            stats.update(&entries_for_stats);

            Some(cache_path)
        } else {
            stats.misses += 1;
            None
        }
    }

    /// 设置缓存
    pub fn set(&self, key: String, source_path: PathBuf, cache_path: PathBuf, params_hash: String, priority: CachePriority) -> Result<()> {
        let size_bytes = fs::metadata(&cache_path)?.len();
        let checksum = self.generate_file_checksum(&cache_path)?;

        let entry = CacheEntry {
            key: key.clone(),
            source_path,
            cache_path,
            params_hash,
            created_at: SystemTime::now(),
            last_accessed: SystemTime::now(),
            access_count: 0,
            size_bytes,
            priority,
            checksum,
        };

        let mut entries = self.entries.write().unwrap();
        entries.insert(key, entry);

        let mut stats = self.stats.write().unwrap();
        stats.update(&entries);

        self.maybe_cleanup()?;
        Ok(())
    }

    fn generate_file_checksum(&self, path: &Path) -> Result<String> {
        let content = fs::read(path)?;
        let hash = blake3::hash(&content);
        Ok(hex::encode(hash.as_bytes()))
    }

    fn remove_cache_file(&self, path: &Path) {
        fs::remove_file(path).ok();
    }

    /// 清理过期缓存
    pub fn cleanup(&self) -> Result<u64> {
        let mut entries = self.entries.write().unwrap();
        let now = SystemTime::now();
        let mut removed_count = 0u64;

        let expired_keys: Vec<String> = entries
            .iter()
            .filter(|(_, entry)| {
                now.duration_since(entry.last_accessed)
                    .unwrap_or(Duration::ZERO) > self.config.ttl
            })
            .map(|(key, _)| key.clone())
            .collect();

        for key in expired_keys {
            if let Some(entry) = entries.remove(&key) {
                self.remove_cache_file(&entry.cache_path);
                removed_count += 1;
            }
        }

        // LRU清理
        if entries.len() > self.config.max_entries as usize {
            let excess = entries.len() - self.config.max_entries as usize;
            
            let mut sorted_entries: Vec<_> = entries.iter().collect();
            sorted_entries.sort_by_key(|(_, entry)| entry.last_accessed);
            
            let to_remove: Vec<String> = sorted_entries
                .into_iter()
                .take(excess)
                .map(|(key, _)| key.clone())
                .collect();

            for key in to_remove {
                if let Some(entry) = entries.remove(&key) {
                    self.remove_cache_file(&entry.cache_path);
                    removed_count += 1;
                }
            }
        }

        let mut stats = self.stats.write().unwrap();
        stats.cleanup_count += 1;
        stats.update(&entries);

        *self.last_cleanup.lock().unwrap() = Instant::now();
        Ok(removed_count)
    }

    fn maybe_cleanup(&self) -> Result<()> {
        let last_cleanup = *self.last_cleanup.lock().unwrap();
        if last_cleanup.elapsed() >= self.config.cleanup_interval {
            self.cleanup()?;
        }
        Ok(())
    }

    fn load_cache_index(&self) -> Result<()> {
        let index_path = self.config.cache_dir.join("index.json");
        if !index_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&index_path)?;
        let entries: HashMap<String, CacheEntry> = serde_json::from_str(&content)?;

        let valid_entries: HashMap<String, CacheEntry> = entries
            .into_iter()
            .filter(|(_, entry)| entry.cache_path.exists())
            .collect();

        *self.entries.write().unwrap() = valid_entries;
        
        let mut stats = self.stats.write().unwrap();
        let entries = self.entries.read().unwrap();
        stats.update(&entries);

        Ok(())
    }

    pub fn save_cache_index(&self) -> Result<()> {
        let entries = self.entries.read().unwrap();
        let index_path = self.config.cache_dir.join("index.json");
        
        let content = serde_json::to_string_pretty(&*entries)?;
        fs::write(&index_path, content)?;
        Ok(())
    }

    pub fn get_stats(&self) -> CacheStats {
        let entries = self.entries.read().unwrap();
        let mut stats = self.stats.write().unwrap();
        stats.update(&entries);
        stats.clone()
    }

    pub fn clear_all(&self) -> Result<u64> {
        let mut entries = self.entries.write().unwrap();
        let count = entries.len() as u64;
        
        for entry in entries.values() {
            self.remove_cache_file(&entry.cache_path);
        }
        
        entries.clear();
        
        let mut stats = self.stats.write().unwrap();
        stats.update(&entries);
        
        Ok(count)
    }
}

impl Drop for SmartCache {
    fn drop(&mut self) {
        self.save_cache_index().ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::write;

    #[test]
    fn test_cache_key_generation() {
        let temp_dir = TempDir::new().unwrap();
        let config = CacheConfig {
            cache_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        
        let cache = SmartCache::new(config).unwrap();
        
        let test_file = temp_dir.path().join("test.txt");
        write(&test_file, "test content").unwrap();
        
        let key1 = cache.generate_cache_key(&test_file, "params1").unwrap();
        let key2 = cache.generate_cache_key(&test_file, "params2").unwrap();
        let key3 = cache.generate_cache_key(&test_file, "params1").unwrap();
        
        assert_eq!(key1, key3);
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_cache_set_and_get() {
        let temp_dir = TempDir::new().unwrap();
        let config = CacheConfig {
            cache_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        
        let cache = SmartCache::new(config).unwrap();
        
        let cache_file = temp_dir.path().join("cached.dat");
        write(&cache_file, "cached content").unwrap();
        
        let key = "test-key".to_string();
        let source = temp_dir.path().join("source.txt");
        write(&source, "source content").unwrap();
        
        cache.set(key.clone(), source, cache_file.clone(), "hash123".to_string(), CachePriority::Normal).unwrap();
        
        let result = cache.get(&key);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), cache_file);
        
        let result = cache.get("nonexistent");
        assert!(result.is_none());
    }
}
