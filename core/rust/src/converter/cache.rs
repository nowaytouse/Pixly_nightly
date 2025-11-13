/**
 *   ==========================================
 * Smart Cache System - 智能缓存系统
 *   ==========================================
 * 
 * 实现高效的图片转换缓存机制:
 * - 基于内容的智能缓存键生成
 * - 自动过期和清理机制
 * - 缓存命中率统计
 * - 并发安全访问
 * 
 * 缓存策略:
 * - 小文件(<10MB): 完整内容哈希
 * - 大文件(≥10MB): 采样哈希(头/中/尾)
 * - 包含转换参数在缓存键中
 * 
 * 过期策略:
 * - 默认7天未访问自动清理
 * - LRU淘汰机制
 * - 可配置缓存大小限制
 *   ==========================================
 */
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use std::fs;
use serde::{Deserialize, Serialize};
use blake3;
// 🔧 统一日志系统
use tracing::{info, warn, debug};

/// 缓存条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// 缓存键
    pub key: String,
    /// 源文件路径
    pub source_path: PathBuf,
    /// 缓存文件路径
    pub cached_path: PathBuf,
    /// 转换参数哈希
    pub params_hash: String,
    /// 文件大小
    pub file_size: u64,
    /// 创建时间
    pub created_at: u64,
    /// 最后访问时间
    pub last_accessed: u64,
    /// 访问次数
    pub access_count: u32,
}

/// 缓存统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    /// 总缓存条目数
    pub total_entries: usize,
    /// 缓存总大小 (字节)
    pub total_size: u64,
    /// 缓存命中次数
    pub hits: u64,
    /// 缓存未命中次数
    pub misses: u64,
    /// 缓存命中率
    pub hit_rate: f64,
}

/// 缓存管理器
pub struct CacheManager {
    /// 缓存目录
    cache_dir: PathBuf,
    /// 缓存索引
    index: HashMap<String, CacheEntry>,
    /// 最大缓存大小 (字节)
    max_cache_size: u64,
    /// 最大条目数
    max_entries: usize,
    /// 缓存过期时间 (秒)
    ttl: Duration,
    /// 统计信息
    stats: CacheStats,
    /// 是否启用缓存
    enabled: bool,
}

impl CacheManager {
    /// 创建新的缓存管理器
    /// 
    /// # 参数
    /// - `cache_dir`: 缓存目录路径
    pub fn new<P: AsRef<Path>>(cache_dir: P) -> Result<Self> {
        let cache_dir = cache_dir.as_ref().to_path_buf();
        
        // 创建缓存目录
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)
                .context("Failed to create cache directory")?;
        }
        
        let mut manager = Self {
            cache_dir,
            index: HashMap::new(),
            max_cache_size: 5 * 1024 * 1024 * 1024, // 5GB
            max_entries: 10000,
            ttl: Duration::from_secs(30 * 24 * 3600), // 30天
            stats: CacheStats {
                total_entries: 0,
                total_size: 0,
                hits: 0,
                misses: 0,
                hit_rate: 0.0,
            },
            enabled: true,
        };
        
        // 加载现有缓存索引
        manager.load_index()?;
        
        info!("📦 Cache manager initialized: {} entries, {} bytes",
                   manager.index.len(), manager.stats.total_size);
        
        Ok(manager)
    }
    
    /// 设置是否启用缓存
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// 设置最大缓存大小
    pub fn set_max_size(&mut self, size: u64) {
        self.max_cache_size = size;
    }
    
    /// 设置缓存过期时间
    pub fn set_ttl(&mut self, ttl: Duration) {
        self.ttl = ttl;
    }
    
    /// 生成缓存键
    /// 
    /// 基于:
    /// - 源文件内容哈希
    /// - 转换参数
    /// - 目标格式
    pub fn generate_key<P: AsRef<Path>>(
        &self,
        source: P,
        format: &str,
        params: &HashMap<String, String>,
    ) -> Result<String> {
        let mut hasher = blake3::Hasher::new();
        
        // 读取文件内容并计算哈希 (对于大文件，只读取部分内容)
        let file_hash = Self::hash_file(source.as_ref())?;
        hasher.update(file_hash.as_bytes());
        
        // 添加格式
        hasher.update(format.as_bytes());
        
        // 添加参数 (排序以保证一致性)
        let mut sorted_params: Vec<_> = params.iter().collect();
        sorted_params.sort_by_key(|(k, _)| *k);
        for (key, value) in sorted_params {
            hasher.update(key.as_bytes());
            hasher.update(value.as_bytes());
        }
        
        let hash = hasher.finalize();
        Ok(hash.to_hex().to_string())
    }
    
    /// 计算文件哈希 (快速版本)
    fn hash_file(path: &Path) -> Result<String> {
        use std::io::{Read, Seek, SeekFrom};
        
        let mut file = fs::File::open(path)?;
        let file_size = file.metadata()?.len();
        let mut hasher = blake3::Hasher::new();
        
        // 对于大文件，只哈希头部、中部和尾部
        if file_size > 10 * 1024 * 1024 { // > 10MB
            let mut buffer = vec![0u8; 1024 * 1024]; // 1MB chunks
            
            // 头部1MB
            let _ = file.read(&mut buffer)?;
            hasher.update(&buffer);
            
            // 中部1MB
            file.seek(SeekFrom::Start(file_size / 2))?;
            let _ = file.read(&mut buffer)?;
            hasher.update(&buffer);
            
            // 尾部1MB
            file.seek(SeekFrom::End(-(buffer.len() as i64)))?;
            let _ = file.read(&mut buffer)?;
            hasher.update(&buffer);
            
            // 添加文件大小到哈希
            hasher.update(&file_size.to_le_bytes());
        } else {
            // 小文件，读取全部内容
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            hasher.update(&buffer);
        }
        
        let hash = hasher.finalize();
        Ok(hash.to_hex().to_string())
    }
    
    /// 查找缓存
    /// 
    /// # 返回
    /// - `Some(PathBuf)`: 缓存文件路径
    /// - `None`: 缓存未命中
    pub fn get(&mut self, key: &str) -> Option<PathBuf> {
        if !self.enabled {
            return None;
        }
        
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // 首先检查条目是否存在且有效
        let should_remove = if let Some(entry) = self.index.get(key) {
            // 检查缓存是否过期
            if now - entry.created_at > self.ttl.as_secs() {
                debug!("⏰ Cache expired: {}", key);
                true
            }
            // 检查缓存文件是否存在
            else if !entry.cached_path.exists() {
                warn!("⚠️  Cache file missing: {:?}", entry.cached_path);
                true
            } else {
                false
            }
        } else {
            self.stats.misses += 1;
            self.update_hit_rate();
            debug!("❌ Cache miss: {}", key);
            return None;
        };
        
        // 如果需要移除，先移除再返回
        if should_remove {
            self.remove(key);
            self.stats.misses += 1;
            self.update_hit_rate();
            return None;
        }
        
        // 现在可以安全地获取可变引用并更新
        if let Some(entry) = self.index.get_mut(key) {
            entry.last_accessed = now;
            entry.access_count += 1;
            let cached_path = entry.cached_path.clone();
            let access_count = entry.access_count;
            
            // 更新统计（在不可变借用之后）
            self.stats.hits += 1;
            self.update_hit_rate();
            
            info!("🎯 Cache hit: {} (accessed {} times)", key, access_count);
            Some(cached_path)
        } else {
            None
        }
    }
    
    /// 添加缓存
    pub fn put<P: AsRef<Path>>(
        &mut self,
        key: String,
        source: P,
        cached_file: P,
        params_hash: String,
    ) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        
        let cached_path = cached_file.as_ref().to_path_buf();
        let file_size = fs::metadata(&cached_path)?.len();
        
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let entry = CacheEntry {
            key: key.clone(),
            source_path: source.as_ref().to_path_buf(),
            cached_path,
            params_hash,
            file_size,
            created_at: now,
            last_accessed: now,
            access_count: 0,
        };
        
        // 检查是否需要清理缓存
        self.maybe_evict(file_size)?;
        
        // 添加到索引
        self.index.insert(key.clone(), entry);
        self.stats.total_entries = self.index.len();
        self.stats.total_size += file_size;
        
        // 保存索引
        self.save_index()?;
        
        info!("💾 Cached: {} ({} bytes)", key, file_size);
        Ok(())
    }
    
    /// 移除缓存
    pub fn remove(&mut self, key: &str) {
        if let Some(entry) = self.index.remove(key) {
            // 删除缓存文件
            if entry.cached_path.exists() {
                let _ = fs::remove_file(&entry.cached_path);
            }
            
            self.stats.total_entries = self.index.len();
            self.stats.total_size = self.stats.total_size.saturating_sub(entry.file_size);
            
            debug!("🗑️  Removed cache: {}", key);
        }
    }
    
    /// LRU淘汰策略
    fn maybe_evict(&mut self, new_entry_size: u64) -> Result<()> {
        // 检查是否超过最大条目数
        while self.index.len() >= self.max_entries {
            self.evict_lru()?;
        }
        
        // 检查是否超过最大缓存大小
        while self.stats.total_size + new_entry_size > self.max_cache_size {
            self.evict_lru()?;
        }
        
        Ok(())
    }
    
    /// 淘汰最少使用的缓存条目
    fn evict_lru(&mut self) -> Result<()> {
        // 找到最少访问的条目
        let lru_key = self.index.iter()
            .min_by_key(|(_, entry)| (entry.last_accessed, entry.access_count))
            .map(|(k, _)| k.clone());
        
        if let Some(key) = lru_key {
            debug!("🔄 Evicting LRU cache: {}", key);
            self.remove(&key);
        }
        
        Ok(())
    }
    
    /// 清理过期缓存
    pub fn cleanup_expired(&mut self) -> Result<usize> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let expired_keys: Vec<String> = self.index.iter()
            .filter(|(_, entry)| now - entry.created_at > self.ttl.as_secs())
            .map(|(k, _)| k.clone())
            .collect();
        
        let count = expired_keys.len();
        for key in expired_keys {
            self.remove(&key);
        }
        
        if count > 0 {
            info!("🧹 Cleaned up {} expired cache entries", count);
            self.save_index()?;
        }
        
        Ok(count)
    }
    
    /// 清空所有缓存
    pub fn clear_all(&mut self) -> Result<()> {
        info!("🗑️  Clearing all cache...");
        
        for entry in self.index.values() {
            if entry.cached_path.exists() {
                let _ = fs::remove_file(&entry.cached_path);
            }
        }
        
        self.index.clear();
        self.stats.total_entries = 0;
        self.stats.total_size = 0;
        
        self.save_index()?;
        Ok(())
    }
    
    /// 获取缓存统计
    pub fn get_stats(&self) -> &CacheStats {
        &self.stats
    }
    
    /// 更新命中率
    fn update_hit_rate(&mut self) {
        let total = self.stats.hits + self.stats.misses;
        if total > 0 {
            self.stats.hit_rate = self.stats.hits as f64 / total as f64;
        }
    }
    
    /// 加载缓存索引
    fn load_index(&mut self) -> Result<()> {
        let index_path = self.cache_dir.join("cache_index.json");
        
        if !index_path.exists() {
            return Ok(());
        }
        
        let content = fs::read_to_string(&index_path)
            .context("Failed to read cache index")?;
        
        let index: HashMap<String, CacheEntry> = serde_json::from_str(&content)
            .context("Failed to parse cache index")?;
        
        // 验证缓存文件是否存在
        let mut valid_index = HashMap::new();
        let mut total_size = 0u64;
        
        for (key, entry) in index {
            if entry.cached_path.exists() {
                total_size += entry.file_size;
                valid_index.insert(key, entry);
            }
        }
        
        self.index = valid_index;
        self.stats.total_entries = self.index.len();
        self.stats.total_size = total_size;
        
        Ok(())
    }
    
    /// 保存缓存索引
    fn save_index(&self) -> Result<()> {
        let index_path = self.cache_dir.join("cache_index.json");
        
        let content = serde_json::to_string_pretty(&self.index)
            .context("Failed to serialize cache index")?;
        
        fs::write(&index_path, content)
            .context("Failed to write cache index")?;
        
        Ok(())
    }
    
    /// 获取缓存文件路径
    pub fn get_cache_path(&self, key: &str, format: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.{}", key, format))
    }
}
