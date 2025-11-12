/*
 * 🔥 Phase 40.12: 智能转换缓存系统
 * 
 * 职责：
 * - 缓存转换结果，避免重复转换
 * - 基于文件内容哈希识别相同文件
 * - 自动过期管理
 * - 并发安全访问
 * 
 * 架构原则：
 * - 使用DashMap实现并发安全
 * - 使用SHA256哈希识别文件
 * - 缓存持久化到磁盘
 */
// 🔧 统一日志系统
use tracing::{info, debug};


use anyhow::{Result, Context};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::SystemTime;
use sha2::{Sha256, Digest};

/// 转换缓存项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// 源文件哈希
    pub source_hash: String,
    /// 源文件路径（用于显示）
    pub source_path: String,
    /// 目标格式
    pub target_format: String,
    /// 转换参数（quality, speed等）
    pub params: ConversionParams,
    /// 输出文件路径
    pub output_path: PathBuf,
    /// 原始文件大小
    pub original_size: u64,
    /// 转换后大小
    pub converted_size: u64,
    /// 转换时间（秒）
    pub conversion_time: f64,
    /// 缓存创建时间
    pub created_at: u64,
    /// 是否仍然有效
    pub is_valid: bool,
}

/// 转换参数（用于缓存key）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversionParams {
    pub quality: u8,
    pub speed: u8,
    pub lossless: bool,
    pub keep_animated: bool,
}

impl ConversionParams {
    pub fn to_key_string(&self) -> String {
        format!("q{}_s{}_l{}_a{}", 
                self.quality, 
                self.speed, 
                if self.lossless { 1 } else { 0 },
                if self.keep_animated { 1 } else { 0 })
    }
}

/// 转换缓存管理器
pub struct ConversionCache {
    /// 内存缓存（并发安全）
    cache: DashMap<String, CacheEntry>,
    /// 缓存目录
    #[allow(dead_code)]  // 🔥 Phase 44: 保留字段以备将来使用（磁盘缓存功能）
    cache_dir: PathBuf,
    /// 缓存过期时间（天）
    expire_days: u64,
    /// 是否启用
    enabled: bool,
    /// 持久化路径
    persist_path: Option<PathBuf>,
}

impl ConversionCache {
    /// 创建新的缓存管理器
    pub fn new(cache_dir: Option<PathBuf>, expire_days: u64) -> Result<Self> {
        let cache_dir = cache_dir.unwrap_or_else(|| {
            dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("pixly")
                .join("conversion_cache")
        });
        
        // 创建缓存目录
        fs::create_dir_all(&cache_dir)
            .context("Failed to create cache directory")?;
        
        let mut cache_manager = Self {
            cache: DashMap::new(),
            cache_dir,
            expire_days,
            enabled: true,
            persist_path: None,
        };
        
        // 加载现有缓存
        cache_manager.load_from_disk()?;
        
        Ok(cache_manager)
    }
    
    /// 生成缓存key
    fn generate_cache_key(
        source_hash: &str,
        target_format: &str,
        params: &ConversionParams,
    ) -> String {
        format!("{}:{}:{}", source_hash, target_format, params.to_key_string())
    }
    
    /// 计算文件哈希
    pub fn calculate_file_hash(file_path: &Path) -> Result<String> {
        let content = fs::read(file_path)
            .context("Failed to read file for hashing")?;
        
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let hash = hasher.finalize();
        
        Ok(format!("{:x}", hash))
    }
    
    /// 查询缓存
    pub fn get(
        &self,
        source_path: &Path,
        target_format: &str,
        params: &ConversionParams,
    ) -> Result<Option<CacheEntry>> {
        if !self.enabled {
            return Ok(None);
        }
        
        // 计算源文件哈希
        let source_hash = Self::calculate_file_hash(source_path)?;
        let cache_key = Self::generate_cache_key(&source_hash, target_format, params);
        
        // 查询缓存
        if let Some(entry) = self.cache.get(&cache_key) {
            let entry = entry.clone();
            
            // 检查是否过期
            if !self.is_expired(&entry) && entry.is_valid {
                // 检查输出文件是否存在
                if entry.output_path.exists() {
                    debug!("✅ Cache hit: {}", cache_key);
                    return Ok(Some(entry));
                } else {
                    debug!("⚠️  Cache invalid: output file missing");
                }
            } else {
                debug!("⚠️  Cache expired: {}", cache_key);
            }
        }
        
        Ok(None)
    }
    
    /// 添加缓存
    #[allow(clippy::too_many_arguments)]
    pub fn set(
        &mut self,
        source_path: &Path,
        target_format: &str,
        params: ConversionParams,
        output_path: PathBuf,
        original_size: u64,
        converted_size: u64,
        conversion_time: f64,
    ) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        
        let source_hash = Self::calculate_file_hash(source_path)?;
        let cache_key = Self::generate_cache_key(&source_hash, target_format, &params);
        
        let entry = CacheEntry {
            source_hash,
            source_path: source_path.to_string_lossy().to_string(),
            target_format: target_format.to_string(),
            params,
            output_path,
            original_size,
            converted_size,
            conversion_time,
            created_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            is_valid: true,
        };
        
        self.cache.insert(cache_key.clone(), entry.clone());
        
        // 持久化到磁盘
        self.save_entry_to_disk(&cache_key, &entry)?;
        
        debug!("✅ Cached: {}", cache_key);
        
        Ok(())
    }
    
    /// 检查是否过期
    fn is_expired(&self, entry: &CacheEntry) -> bool {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let age_days = (now - entry.created_at) / 86400;
        age_days > self.expire_days
    }
    
    /// 保存单个缓存项到磁盘（增量持久化）
    fn save_entry_to_disk(&self, key: &str, entry: &CacheEntry) -> Result<()> {
        if let Some(ref persist_path) = self.persist_path {
            // 🔥 真实的增量持久化实现
            let cache_dir = persist_path.parent()
                .context("Invalid cache path")?;
            
            // 确保目录存在
            if !cache_dir.exists() {
                std::fs::create_dir_all(cache_dir)
                    .context("Failed to create cache directory")?;
            }
            
            // 为每个条目创建单独的文件（增量式）
            // 使用key的哈希作为文件名以避免特殊字符
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let hash = hasher.finish();
            
            let entry_file = cache_dir.join(format!("cache_{:016x}.json", hash));
            
            // 序列化并写入
            let json = serde_json::to_string_pretty(entry)
                .context("Failed to serialize cache entry")?;
            
            std::fs::write(&entry_file, json)
                .context("Failed to write cache entry")?;
            
            debug!("💾 Saved cache entry: {} -> {:?}", key, entry_file);
        }
        
        Ok(())
    }
    
    /// 从磁盘加载所有缓存条目
    pub fn load_from_disk(&mut self) -> Result<usize> {
        if let Some(ref persist_path) = self.persist_path {
            let cache_dir = persist_path.parent()
                .context("Invalid cache path")?;
            
            if !cache_dir.exists() {
                debug!("Cache directory doesn't exist, skipping load");
                return Ok(0);
            }
            
            let mut loaded = 0;
            
            // 读取所有cache_*.json文件
            for entry in std::fs::read_dir(cache_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() && 
                   path.file_name()
                       .and_then(|n| n.to_str())
                       .map(|n| n.starts_with("cache_") && n.ends_with(".json"))
                       .unwrap_or(false) 
                {
                                         if let Ok(json) = std::fs::read_to_string(&path) {
                        if let Ok(cache_entry) = serde_json::from_str::<CacheEntry>(&json) {
                            // 使用source_hash重建key
                            let key = Self::generate_cache_key(
                                &cache_entry.source_hash, 
                                &cache_entry.target_format, 
                                &cache_entry.params
                            );
                            self.cache.insert(key, cache_entry);
                            loaded += 1;
                        }
                    }
                }
            }
            
            if loaded > 0 {
                info!("💾 Loaded {} cache entries from disk", loaded);
            }
            
            Ok(loaded)
        } else {
            Ok(0)
        }
    }
    
    /// 持久化所有缓存（完整保存）
    pub fn persist_all(&self) -> Result<()> {
        if self.persist_path.is_some() {
            // 保存所有条目
            let mut saved = 0;
            for item in &self.cache {
                let (key, entry) = item.pair();
                if let Ok(()) = self.save_entry_to_disk(key, entry) {
                    saved += 1;
                }
            }
            
            if saved > 0 {
                info!("💾 Persisted {} cache entries", saved);
            }
            
            Ok(())
        } else {
            Ok(())
        }
    }
    
    /// 清理过期缓存
    pub fn cleanup_expired(&mut self) -> usize {
        let mut removed = 0;
        
        self.cache.retain(|_key, entry| {
            if self.is_expired(entry) {
                removed += 1;
                false
            } else {
                true
            }
        });
        
        if removed > 0 {
            info!("🗑️  Cleaned up {} expired cache entries", removed);
        }
        
        removed
    }
    
    /// 获取缓存统计
    pub fn stats(&self) -> CacheStats {
        let total = self.cache.len();
        let mut valid = 0;
        let mut expired = 0;
        let mut total_savings: i64 = 0;
        
        for entry in self.cache.iter() {
            if self.is_expired(&entry) {
                expired += 1;
            } else {
                valid += 1;
                total_savings += entry.original_size as i64 - entry.converted_size as i64;
            }
        }
        
        CacheStats {
            total,
            valid,
            expired,
            total_savings_bytes: total_savings,
        }
    }
}

/// 缓存统计
#[derive(Debug)]
pub struct CacheStats {
    pub total: usize,
    pub valid: usize,
    pub expired: usize,
    pub total_savings_bytes: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_key_generation() {
        let params = ConversionParams {
            quality: 85,
            speed: 4,
            lossless: false,
            keep_animated: true,
        };
        
        let key = ConversionCache::generate_cache_key("abc123", "avif", &params);
        assert!(key.contains("abc123"));
        assert!(key.contains("avif"));
        assert!(key.contains("q85"));
    }
}
