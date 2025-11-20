/**
 * 🚀 统一智能缓存系统 - 恢复和增强版
 * 
 * Phase 4: 从@archive恢复缓存功能，并与统一架构集成
 * 
 * 🎯 价值恢复 + 增强：
 * - 智能缓存键生成（基于内容+参数）
 * - LRU + TTL 双重过期策略
 * - 缓存命中率统计和监控
 * - 与unified_error_system集成
 * - 与unified_progress集成
 * - 并发安全访问
 * 
 * @module unified_cache
 * @enhanced_from @archive/cache.rs
 */
use crate::error::{PixlyError, ErrorBuilder, ErrorSeverity};
use crate::converter::unified_progress::{UnifiedProgressTracker, ProgressLevel};
use tracing::{info, warn, debug};

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::time::{SystemTime, Duration, Instant};
use std::sync::{Arc, RwLock, Mutex};
use std::fs;
use serde::{Deserialize, Serialize};
use blake3;

/// 统一缓存条目 - 增强版
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCacheEntry {
    /// 缓存键
    pub key: String,
    /// 源文件路径
    pub source_path: PathBuf,
    /// 缓存文件路径
    pub cache_path: PathBuf,
    /// 转换参数哈希
    pub params_hash: String,
    /// 创建时间
    pub created_at: SystemTime,
    /// 最后访问时间
    pub last_accessed: SystemTime,
    /// 访问次数
    pub access_count: u64,
    /// 缓存大小（字节）
    pub size_bytes: u64,
    /// 🆕 缓存优先级
    pub priority: CachePriority,
    /// 🆕 压缩状态
    pub is_compressed: bool,
    /// 🆕 校验和
    pub checksum: String,
}

/// 缓存优先级
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CachePriority {
    Low,
    Normal,
    High,
    Critical,
}

/// 缓存统计信息 - 增强版
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCacheStats {
    /// 总缓存项数
    pub total_entries: u64,
    /// 缓存命中次数
    pub hits: u64,
    /// 缓存未命中次数
    pub misses: u64,
    /// 总缓存大小（字节）
    pub total_size_bytes: u64,
    /// 平均访问次数
    pub avg_access_count: f64,
    /// 最老的缓存项创建时间
    pub oldest_entry: Option<SystemTime>,
    /// 最新的缓存项创建时间
    pub newest_entry: Option<SystemTime>,
    /// 🆕 命中率百分比
    pub hit_rate_percent: f64,
    /// 🆕 清理次数
    pub cleanup_count: u64,
    /// 🆕 压缩节省的空间
    pub compression_saved_bytes: u64,
}

impl UnifiedCacheStats {
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
            compression_saved_bytes: 0,
        }
    }

    fn update(&mut self, entries: &HashMap<String, UnifiedCacheEntry>) {
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

/// 缓存配置 - 增强版
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCacheConfig {
    /// 缓存目录
    pub cache_dir: PathBuf,
    /// 最大缓存大小（字节）
    pub max_size_bytes: u64,
    /// 最大条目数
    pub max_entries: u64,
    /// TTL（存活时间）
    pub ttl: Duration,
    /// 清理间隔
    pub cleanup_interval: Duration,
    /// 🆕 启用压缩
    pub enable_compression: bool,
    /// 🆕 压缩阈值（超过此大小才压缩）
    pub compression_threshold_bytes: u64,
    /// 🆕 并发级别
    pub concurrency_level: usize,
}

impl Default for UnifiedCacheConfig {
    fn default() -> Self {
        Self {
            cache_dir: PathBuf::from("./cache"),
            max_size_bytes: 1024 * 1024 * 1024, // 1GB
            max_entries: 10000,
            ttl: Duration::from_secs(7 * 24 * 3600), // 7天
            cleanup_interval: Duration::from_secs(3600), // 1小时
            enable_compression: true,
            compression_threshold_bytes: 1024 * 1024, // 1MB
            concurrency_level: 4,
        }
    }
}

/// 统一智能缓存系统 - 增强版
#[derive(Debug)]
pub struct UnifiedSmartCache {
    config: UnifiedCacheConfig,
    entries: Arc<RwLock<HashMap<String, UnifiedCacheEntry>>>,
    stats: Arc<RwLock<UnifiedCacheStats>>,
    last_cleanup: Arc<Mutex<Instant>>,
}

impl UnifiedSmartCache {
    /// 创建新的统一缓存系统
    pub fn new(config: UnifiedCacheConfig) -> Result<Self, PixlyError> {
        // 确保缓存目录存在
        if let Err(e) = fs::create_dir_all(&config.cache_dir) {
            return Err(ErrorBuilder::new()
                .code("CACHE-001")
                .message(&format!("Failed to create cache directory: {}", e))
                .severity(ErrorSeverity::Error)
                .build());
        }

        let cache = Self {
            config,
            entries: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(UnifiedCacheStats::new())),
            last_cleanup: Arc::new(Mutex::new(Instant::now())),
        };

        // 加载现有缓存
        cache.load_cache_index()?;

        info!("Unified cache system initialized at {:?}", cache.config.cache_dir);
        Ok(cache)
    }

    /// 生成智能缓存键
    pub fn generate_cache_key(&self, source_path: &Path, params: &str) -> Result<String, PixlyError> {
        let mut hasher = blake3::Hasher::new();
        
        // 添加文件路径
        hasher.update(source_path.to_string_lossy().as_bytes());
        
        // 读取文件内容进行哈希
        let metadata = fs::metadata(source_path).map_err(|e| {
            ErrorBuilder::new()
                .code("CACHE-002")
                .message(&format!("Failed to read file metadata: {}", e))
                .severity(ErrorSeverity::Error)
                .build()
        })?;

        let file_size = metadata.len();
        hasher.update(&file_size.to_le_bytes());

        // 智能采样策略
        if file_size <= 10 * 1024 * 1024 { // 小于10MB，完整哈希
            let content = fs::read(source_path).map_err(|e| {
                ErrorBuilder::new()
                    .code("CACHE-003")
                    .message(&format!("Failed to read file content: {}", e))
                    .severity(ErrorSeverity::Error)
                    .build()
            })?;
            hasher.update(&content);
        } else { // 大文件，采样哈希
            let mut file = fs::File::open(source_path).map_err(|e| {
                ErrorBuilder::new()
                    .code("CACHE-004")
                    .message(&format!("Failed to open file for sampling: {}", e))
                    .severity(ErrorSeverity::Error)
                    .build()
            })?;

            use std::io::{Read, Seek, SeekFrom};
            let sample_size = 8192; // 8KB采样
            let mut buffer = vec![0u8; sample_size];

            // 采样开头
            file.read_exact(&mut buffer).ok();
            hasher.update(&buffer);

            // 采样中间
            file.seek(SeekFrom::Start(file_size / 2)).ok();
            file.read_exact(&mut buffer).ok();
            hasher.update(&buffer);

            // 采样结尾
            file.seek(SeekFrom::End(-(sample_size as i64))).ok();
            file.read_exact(&mut buffer).ok();
            hasher.update(&buffer);
        }

        // 添加转换参数
        hasher.update(params.as_bytes());

        Ok(hex::encode(hasher.finalize().as_bytes()))
    }

    /// 获取缓存
    pub fn get(&self, key: &str) -> Option<PathBuf> {
        let mut entries = self.entries.write().unwrap();
        let mut stats = self.stats.write().unwrap();

        if let Some(entry) = entries.get_mut(key) {
            // 检查TTL
            if entry.created_at.elapsed().unwrap_or(Duration::MAX) > self.config.ttl {
                // 过期，删除
                self.remove_cache_file(&entry.cache_path);
                entries.remove(key);
                stats.misses += 1;
                return None;
            }

            // 更新访问信息
            entry.last_accessed = SystemTime::now();
            entry.access_count += 1;
            let cache_path = entry.cache_path.clone();
            let is_compressed = entry.is_compressed;
            
            stats.hits += 1;
            // 临时释放entries锁以避免借用冲突
            drop(entries);
            let entries_for_stats = self.entries.read().unwrap();
            stats.update(&entries_for_stats);
            drop(entries_for_stats);

            debug!("Cache hit for key: {}", key);
            
            // 如果是压缩文件，需要解压缩
            if is_compressed {
                match self.decompress_cache_file(&cache_path) {
                    Ok(decompressed_path) => Some(decompressed_path),
                    Err(e) => {
                        warn!("Failed to decompress cache file: {:?}", e);
                        None
                    }
                }
            } else {
                Some(cache_path)
            }
        } else {
            stats.misses += 1;
            debug!("Cache miss for key: {}", key);
            None
        }
    }
    
    /// 解压缩缓存文件
    fn decompress_cache_file(&self, compressed_path: &Path) -> Result<PathBuf, PixlyError> {
        use std::io::{Read, Write};
        use flate2::read::GzDecoder;
        
        // 生成解压缩后的临时文件路径
        let decompressed_path = compressed_path.with_extension("");
        
        // 如果解压缩文件已存在，直接返回
        if decompressed_path.exists() {
            return Ok(decompressed_path);
        }
        
        // 读取压缩文件
        let compressed_file = fs::File::open(compressed_path)
            .map_err(|e| ErrorBuilder::new()
                .code("CACHE-016")
                .message(&format!("Failed to open compressed file: {}", e))
                .severity(ErrorSeverity::Error)
                .build())?;
        
        let mut decoder = GzDecoder::new(compressed_file);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer)
            .map_err(|e| ErrorBuilder::new()
                .code("CACHE-017")
                .message(&format!("Failed to decompress file: {}", e))
                .severity(ErrorSeverity::Error)
                .build())?;
        
        // 写入解压缩文件
        let mut decompressed_file = fs::File::create(&decompressed_path)
            .map_err(|e| ErrorBuilder::new()
                .code("CACHE-018")
                .message(&format!("Failed to create decompressed file: {}", e))
                .severity(ErrorSeverity::Error)
                .build())?;
        
        decompressed_file.write_all(&buffer)
            .map_err(|e| ErrorBuilder::new()
                .code("CACHE-019")
                .message(&format!("Failed to write decompressed data: {}", e))
                .severity(ErrorSeverity::Error)
                .build())?;
        
        Ok(decompressed_path)
    }

    /// 设置缓存
    pub fn set(&self, key: String, source_path: PathBuf, cache_path: PathBuf, params_hash: String, priority: CachePriority) -> Result<(), PixlyError> {
        let mut size_bytes = fs::metadata(&cache_path)
            .map_err(|e| {
                ErrorBuilder::new()
                    .code("CACHE-005")
                    .message(&format!("Failed to get cache file size: {}", e))
                    .severity(ErrorSeverity::Error)
                    .build()
            })?
            .len();

        // 生成校验和
        let checksum = self.generate_file_checksum(&cache_path)?;

        // 检查是否需要压缩
        let mut is_compressed = false;
        let mut final_cache_path = cache_path.clone();
        
        if self.config.enable_compression && size_bytes > self.config.compression_threshold_bytes {
            // 尝试压缩缓存文件
            match self.compress_cache_file(&cache_path) {
                Ok(compressed_path) => {
                    let compressed_size = fs::metadata(&compressed_path)?.len();
                    
                    // 只有压缩后更小才使用压缩版本
                    if compressed_size < size_bytes {
                        // 删除原文件，使用压缩版本
                        let _ = fs::remove_file(&cache_path);
                        final_cache_path = compressed_path;
                        
                        let saved = size_bytes - compressed_size;
                        size_bytes = compressed_size;
                        is_compressed = true;
                        
                        debug!("Cache file compressed: saved {} bytes ({:.1}%)", 
                               saved, (saved as f64 / size_bytes as f64) * 100.0);
                        
                        // 更新统计
                        let mut stats = self.stats.write().unwrap();
                        stats.compression_saved_bytes += saved;
                    } else {
                        // 压缩后更大，删除压缩文件，使用原文件
                        let _ = fs::remove_file(&compressed_path);
                    }
                }
                Err(e) => {
                    debug!("Cache compression failed: {}, using uncompressed", e);
                }
            }
        }

        let entry = UnifiedCacheEntry {
            key: key.clone(),
            source_path,
            cache_path: final_cache_path,
            params_hash,
            created_at: SystemTime::now(),
            last_accessed: SystemTime::now(),
            access_count: 0,
            size_bytes,
            priority,
            is_compressed,
            checksum,
        };

        let mut entries = self.entries.write().unwrap();
        entries.insert(key.clone(), entry);

        let mut stats = self.stats.write().unwrap();
        stats.update(&entries);

        info!("Cache entry added: {} ({} bytes{})", key, size_bytes, 
              if is_compressed { ", compressed" } else { "" });

        // 检查是否需要清理
        self.maybe_cleanup()?;

        Ok(())
    }
    
    /// 压缩缓存文件
    fn compress_cache_file(&self, path: &Path) -> Result<PathBuf, PixlyError> {
        use std::io::{Read, Write};
        use flate2::Compression;
        use flate2::write::GzEncoder;
        
        let compressed_path = path.with_extension("cache.gz");
        
        // 读取原文件
        let mut file = fs::File::open(path)
            .map_err(|e| ErrorBuilder::new()
                .code("CACHE-011")
                .message(&format!("Failed to open file for compression: {}", e))
                .severity(ErrorSeverity::Error)
                .build())?;
        
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| ErrorBuilder::new()
                .code("CACHE-012")
                .message(&format!("Failed to read file for compression: {}", e))
                .severity(ErrorSeverity::Error)
                .build())?;
        
        // 压缩
        let compressed_file = fs::File::create(&compressed_path)
            .map_err(|e| ErrorBuilder::new()
                .code("CACHE-013")
                .message(&format!("Failed to create compressed file: {}", e))
                .severity(ErrorSeverity::Error)
                .build())?;
        
        let mut encoder = GzEncoder::new(compressed_file, Compression::default());
        encoder.write_all(&buffer)
            .map_err(|e| ErrorBuilder::new()
                .code("CACHE-014")
                .message(&format!("Failed to write compressed data: {}", e))
                .severity(ErrorSeverity::Error)
                .build())?;
        
        encoder.finish()
            .map_err(|e| ErrorBuilder::new()
                .code("CACHE-015")
                .message(&format!("Failed to finish compression: {}", e))
                .severity(ErrorSeverity::Error)
                .build())?;
        
        Ok(compressed_path)
    }

    /// 生成文件校验和
    fn generate_file_checksum(&self, path: &Path) -> Result<String, PixlyError> {
        let content = fs::read(path).map_err(|e| {
            ErrorBuilder::new()
                .code("CACHE-006")
                .message(&format!("Failed to read file for checksum: {}", e))
                .severity(ErrorSeverity::Error)
                .build()
        })?;

        let hash = blake3::hash(&content);
        Ok(hex::encode(hash.as_bytes()))
    }

    /// 删除缓存
    pub fn remove(&self, key: &str) -> bool {
        let mut entries = self.entries.write().unwrap();
        if let Some(entry) = entries.remove(key) {
            self.remove_cache_file(&entry.cache_path);
            
            let mut stats = self.stats.write().unwrap();
            stats.update(&entries);
            
            debug!("Cache entry removed: {}", key);
            true
        } else {
            false
        }
    }

    /// 获取缓存统计
    pub fn get_stats(&self) -> UnifiedCacheStats {
        let entries = self.entries.read().unwrap();
        let mut stats = self.stats.write().unwrap();
        stats.update(&entries);
        stats.clone()
    }

    /// 清理过期缓存
    pub fn cleanup(&self) -> Result<u64, PixlyError> {
        let progress = UnifiedProgressTracker::new(
            ProgressLevel::Operation,
            0, // 总数未知，后续更新
            "Cache cleanup".to_string()
        );

        let mut entries = self.entries.write().unwrap();
        let now = SystemTime::now();
        let mut removed_count = 0u64;
        let mut removed_size = 0u64;

        // 收集过期条目
        let expired_keys: Vec<String> = entries
            .iter()
            .filter(|(_, entry)| {
                now.duration_since(entry.last_accessed)
                    .unwrap_or(Duration::ZERO) > self.config.ttl
            })
            .map(|(key, _)| key.clone())
            .collect();

        progress.update(0, Some(format!("Found {} expired entries", expired_keys.len()))).ok();

        // 删除过期条目
        for (i, key) in expired_keys.iter().enumerate() {
            if let Some(entry) = entries.remove(key) {
                removed_size += entry.size_bytes;
                self.remove_cache_file(&entry.cache_path);
                removed_count += 1;
            }
            
            progress.update(i as u64 + 1, Some(format!("Removed expired entry: {}", key))).ok();
        }

        // 检查是否超过大小限制，LRU清理
        if entries.len() > self.config.max_entries as usize {
            let excess = entries.len() - self.config.max_entries as usize;
            
            // 按最后访问时间排序
            let mut sorted_entries: Vec<_> = entries.iter().collect();
            sorted_entries.sort_by_key(|(_, entry)| entry.last_accessed);
            
            let to_remove: Vec<String> = sorted_entries
                .into_iter()
                .take(excess)
                .map(|(key, _)| key.clone())
                .collect();

            for key in to_remove {
                if let Some(entry) = entries.remove(&key) {
                    removed_size += entry.size_bytes;
                    self.remove_cache_file(&entry.cache_path);
                    removed_count += 1;
                }
            }
        }

        let mut stats = self.stats.write().unwrap();
        stats.cleanup_count += 1;
        stats.update(&entries);

        *self.last_cleanup.lock().unwrap() = Instant::now();

        info!("Cache cleanup completed: removed {} entries ({} bytes)", removed_count, removed_size);
        Ok(removed_count)
    }

    /// 可能执行清理
    fn maybe_cleanup(&self) -> Result<(), PixlyError> {
        let last_cleanup = *self.last_cleanup.lock().unwrap();
        if last_cleanup.elapsed() >= self.config.cleanup_interval {
            self.cleanup()?;
        }
        Ok(())
    }

    /// 删除缓存文件
    fn remove_cache_file(&self, path: &Path) {
        if let Err(e) = fs::remove_file(path) {
            warn!("Failed to remove cache file {:?}: {}", path, e);
        }
    }

    /// 加载缓存索引
    fn load_cache_index(&self) -> Result<(), PixlyError> {
        let index_path = self.config.cache_dir.join("index.json");
        if !index_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&index_path).map_err(|e| {
            ErrorBuilder::new()
                .code("CACHE-007")
                .message(&format!("Failed to read cache index: {}", e))
                .severity(ErrorSeverity::Warning)
                .build()
        })?;

        let entries: HashMap<String, UnifiedCacheEntry> = serde_json::from_str(&content).map_err(|e| {
            ErrorBuilder::new()
                .code("CACHE-008")
                .message(&format!("Failed to parse cache index: {}", e))
                .severity(ErrorSeverity::Warning)
                .build()
        })?;

        // 验证缓存文件是否存在
        let valid_entries: HashMap<String, UnifiedCacheEntry> = entries
            .into_iter()
            .filter(|(_, entry)| entry.cache_path.exists())
            .collect();

        *self.entries.write().unwrap() = valid_entries;
        
        let mut stats = self.stats.write().unwrap();
        let entries = self.entries.read().unwrap();
        stats.update(&entries);

        info!("Loaded {} cache entries from index", entries.len());
        Ok(())
    }

    /// 保存缓存索引
    pub fn save_cache_index(&self) -> Result<(), PixlyError> {
        let entries = self.entries.read().unwrap();
        let index_path = self.config.cache_dir.join("index.json");
        
        let content = serde_json::to_string_pretty(&*entries).map_err(|e| {
            ErrorBuilder::new()
                .code("CACHE-009")
                .message(&format!("Failed to serialize cache index: {}", e))
                .severity(ErrorSeverity::Error)
                .build()
        })?;

        fs::write(&index_path, content).map_err(|e| {
            ErrorBuilder::new()
                .code("CACHE-010")
                .message(&format!("Failed to write cache index: {}", e))
                .severity(ErrorSeverity::Error)
                .build()
        })?;

        debug!("Cache index saved to {:?}", index_path);
        Ok(())
    }

    /// 清空所有缓存
    pub fn clear_all(&self) -> Result<u64, PixlyError> {
        let mut entries = self.entries.write().unwrap();
        let count = entries.len() as u64;
        
        for entry in entries.values() {
            self.remove_cache_file(&entry.cache_path);
        }
        
        entries.clear();
        
        let mut stats = self.stats.write().unwrap();
        stats.update(&entries);
        
        info!("All cache entries cleared: {} items", count);
        Ok(count)
    }
}

impl Drop for UnifiedSmartCache {
    fn drop(&mut self) {
        if let Err(e) = self.save_cache_index() {
            warn!("Failed to save cache index on drop: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::write;

    fn create_test_cache() -> (UnifiedSmartCache, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config = UnifiedCacheConfig {
            cache_dir: temp_dir.path().to_path_buf(),
            max_size_bytes: 1024 * 1024, // 1MB for testing
            max_entries: 10,
            ttl: Duration::from_secs(3600),
            cleanup_interval: Duration::from_secs(60),
            enable_compression: false,
            compression_threshold_bytes: 1024,
            concurrency_level: 2,
        };
        
        let cache = UnifiedSmartCache::new(config).unwrap();
        (cache, temp_dir)
    }

    #[test]
    fn test_cache_key_generation() {
        let (cache, temp_dir) = create_test_cache();
        
        // 创建测试文件
        let test_file = temp_dir.path().join("test.txt");
        write(&test_file, "test content").unwrap();
        
        let key1 = cache.generate_cache_key(&test_file, "params1").unwrap();
        let key2 = cache.generate_cache_key(&test_file, "params2").unwrap();
        let key3 = cache.generate_cache_key(&test_file, "params1").unwrap();
        
        // 相同文件+相同参数应该生成相同key
        assert_eq!(key1, key3);
        // 相同文件+不同参数应该生成不同key
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_cache_set_and_get() {
        let (cache, temp_dir) = create_test_cache();
        
        let cache_file = temp_dir.path().join("cached.dat");
        write(&cache_file, "cached content").unwrap();
        
        let key = "test-key".to_string();
        let source = temp_dir.path().join("source.txt");
        write(&source, "source content").unwrap();
        
        // 设置缓存
        cache.set(key.clone(), source, cache_file.clone(), "hash123".to_string(), CachePriority::Normal).unwrap();
        
        // 获取缓存
        let result = cache.get(&key);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), cache_file);
        
        // 获取不存在的缓存
        let result = cache.get("nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_cache_stats() {
        let (cache, temp_dir) = create_test_cache();
        
        let cache_file = temp_dir.path().join("cached.dat");
        write(&cache_file, "cached content").unwrap();
        
        let source = temp_dir.path().join("source.txt");
        write(&source, "source content").unwrap();
        
        // 添加缓存条目
        cache.set("key1".to_string(), source.clone(), cache_file.clone(), "hash1".to_string(), CachePriority::Normal).unwrap();
        
        // 检查统计
        let stats = cache.get_stats();
        assert_eq!(stats.total_entries, 1);
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        
        // 命中缓存
        cache.get("key1");
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.hit_rate_percent, 100.0);
        
        // 未命中缓存
        cache.get("nonexistent");
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hit_rate_percent, 50.0);
    }
}
