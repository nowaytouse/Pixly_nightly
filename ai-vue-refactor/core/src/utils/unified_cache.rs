//! 🗄️ UnifiedcacheSystem
//!
//! providehighperformanceconversionresultcache：
//! - based oninsidehashcachekey
//! - LRUstrategy
//! - disk
//! - autocleanup

use std::path::PathBuf;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use anyhow::Result;
use serde::{Serialize, Deserialize};

/// cachebar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
/// inputfilehash
 pub input_hash: String,
/// conversionparameterhash
 pub params_hash: String,
/// outputfilepath
 pub output_path: PathBuf,
/// createtime
 pub created_at: u64,
/// most after time
 pub last_accessed: u64,
/// count
 pub access_count: u64,
/// outputfilesize
 pub output_size: u64,
}

/// cacheconfiguration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
/// cachedirectory
 pub cache_dir: PathBuf,
/// maximumcachesize（MB）
 pub max_size_mb: u64,
/// bartime（）
 pub expire_days: u64,
/// enabled LRU
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

/// Unifiedcache Manager
pub struct UnifiedCache {
 config: CacheConfig,
 entries: HashMap<String, CacheEntry>,
 total_size: u64,
}

impl UnifiedCache {
/// createnewcache Manager
 pub fn new(config: CacheConfig) -> Result<Self> {
// ensurecachedirectoryexists
 std::fs::create_dir_all(&config.cache_dir)?;

 let mut cache = Self {
 config,
 entries: HashMap::new(),
 total_size: 0,
 };

// load has cache
 cache.load_cache()?;

 Ok(cache)
 }

/// generatecachekey
 pub fn generate_key(input_hash: &str, params_hash: &str) -> String {
 format!("{}_{}", input_hash, params_hash)
 }

/// findcache
 pub fn get(&mut self, input_hash: &str, params_hash: &str) -> Option<CacheEntry> {
 let key = Self::generate_key(input_hash, params_hash);

 if let Some(entry) = self.entries.get_mut(&key) {
// updateinformation
 entry.last_accessed = SystemTime::now()
 .duration_since(SystemTime::UNIX_EPOCH)
 .unwrap_or(Duration::from_secs(0))
 .as_secs();
 entry.access_count += 1;

// checkoutputfileisnoexists
 if entry.output_path.exists() {
 return Some(entry.clone());
 } else {
// filealready be delete，removedcachebar
 self.entries.remove(&key);
 return None;
 }
 }

 None
 }

/// addcachebar
 pub fn put(&mut self, entry: CacheEntry) -> Result<()> {
 let key = Self::generate_key(&entry.input_hash, &entry.params_hash);

// checkisnoneed
 if self.config.enable_lru {
 self.evict_if_needed(entry.output_size)?;
 }

 self.total_size += entry.output_size;
 self.entries.insert(key, entry);

// 
 self.save_cache()?;

 Ok(())
 }

/// LRU
 fn evict_if_needed(&mut self, new_size: u64) -> Result<()> {
 let max_size = self.config.max_size_mb * 1024 * 1024;

 while self.total_size + new_size > max_size && !self.entries.is_empty() {
// findtomostfewusebar
 let lru_key = self.entries
 .iter()
 .min_by_key(|(_, e)| e.last_accessed)
 .map(|(k, _)| k.clone());

 if let Some(key) = lru_key {
 if let Some(entry) = self.entries.remove(&key) {
 self.total_size -= entry.output_size;
// deleteoutputfile
 let _ = std::fs::remove_file(&entry.output_path);
 log::info!("🗑️ Evicted cache entry: {}", key);
 }
 } else {
 break;
 }
 }

 Ok(())
 }

/// cleanupbar
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

/// loadcacheindex
 fn load_cache(&mut self) -> Result<()> {
 let index_path = self.config.cache_dir.join("cache_index.json");

 if !index_path.exists() {
 return Ok(());
 }

 let data = std::fs::read_to_string(&index_path)?;
 let entries: HashMap<String, CacheEntry> = serde_json::from_str(&data)?;

// validationbarandcalculationsize
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

/// savecacheindex
 fn save_cache(&self) -> Result<()> {
 let index_path = self.config.cache_dir.join("cache_index.json");
 let data = serde_json::to_string_pretty(&self.entries)?;
 std::fs::write(&index_path, data)?;
 Ok(())
 }

/// getcachestatistics
 pub fn get_stats(&self) -> CacheStats {
 CacheStats {
 total_entries: self.entries.len(),
 total_size_mb: self.total_size as f64 / (1024.0 * 1024.0),
 max_size_mb: self.config.max_size_mb as f64,
 usage_percent: (self.total_size as f64 / (self.config.max_size_mb as f64 * 1024.0 * 1024.0)) * 100.0,
 }
 }

/// clearcache
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

/// cachestatisticsinformation
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
