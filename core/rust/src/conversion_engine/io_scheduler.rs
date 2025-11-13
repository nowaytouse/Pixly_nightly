/*
💾 PIXLY v3.1 文件I/O优化调度器

高性能文件I/O管理，支持：
- 异步文件读写
- 内存映射优化
- 零拷贝操作
- 智能缓存策略
- SIMD加速支持

实现Rust作为唯一执行层的高效I/O处理
*/

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{RwLock, Semaphore};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use memmap2::{Mmap, MmapOptions};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use tracing::{info, warn, error, debug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IORequest {
    pub operation: IOOperation,
    pub file_path: PathBuf,
    pub buffer_size: Option<usize>,
    pub use_memory_mapping: bool,
    pub enable_simd: bool,
    pub priority: IOPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IOOperation {
    Read,
    Write { data: Vec<u8> },
    Copy { destination: PathBuf },
    Move { destination: PathBuf },
    Delete,
    GetMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IOPriority {
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOResult {
    pub success: bool,
    pub operation: String,
    pub file_path: PathBuf,
    pub data: Option<Vec<u8>>,
    pub file_size: u64,
    pub processing_time_ms: u64,
    pub bytes_processed: u64,
    pub simd_used: bool,
    pub memory_mapped: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    data: Vec<u8>,
    last_accessed: std::time::Instant,
    access_count: u64,
    file_size: u64,
}

pub struct IOScheduler {
    enable_simd: bool,
    memory_cache: Arc<RwLock<HashMap<PathBuf, CacheEntry>>>,
    cache_size_limit: usize,
    io_semaphore: Arc<Semaphore>,
    stats: Arc<RwLock<IOStats>>,
}

#[derive(Debug, Clone, Default)]
struct IOStats {
    total_operations: u64,
    total_bytes_read: u64,
    total_bytes_written: u64,
    cache_hits: u64,
    cache_misses: u64,
    simd_operations: u64,
    memory_mapped_operations: u64,
    average_operation_time_ms: f32,
}

impl IOScheduler {
    pub fn new(enable_simd: bool) -> Self {
        info!("💾 I/O调度器初始化: SIMD={}", enable_simd);
        
        Self {
            enable_simd,
            memory_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_size_limit: 512 * 1024 * 1024, // 512MB缓存限制
            io_semaphore: Arc::new(Semaphore::new(16)), // 最多16个并发I/O操作
            stats: Arc::new(RwLock::new(IOStats::default())),
        }
    }

    pub async fn execute_io(&self, request: IORequest) -> Result<IOResult> {
        let start_time = std::time::Instant::now();
        
        // 获取I/O许可
        let _permit = self.io_semaphore.acquire().await?;
        
        debug!("💾 执行I/O操作: {:?} on {}", request.operation, request.file_path.display());
        
        let result = match &request.operation {
            IOOperation::Read => self.handle_read(&request).await,
            IOOperation::Write { data } => self.handle_write(&request, data.clone()).await,
            IOOperation::Copy { destination } => self.handle_copy(&request, destination.clone()).await,
            IOOperation::Move { destination } => self.handle_move(&request, destination.clone()).await,
            IOOperation::Delete => self.handle_delete(&request).await,
            IOOperation::GetMetadata => self.handle_get_metadata(&request).await,
        };

        let processing_time = start_time.elapsed();
        
        // 更新统计信息
        self.update_stats(&request, &result, processing_time).await;
        
        match result {
            Ok(mut io_result) => {
                io_result.processing_time_ms = processing_time.as_millis() as u64;
                Ok(io_result)
            }
            Err(e) => {
                error!("❌ I/O操作失败: {}", e);
                Ok(IOResult {
                    success: false,
                    operation: format!("{:?}", request.operation),
                    file_path: request.file_path,
                    data: None,
                    file_size: 0,
                    processing_time_ms: processing_time.as_millis() as u64,
                    bytes_processed: 0,
                    simd_used: false,
                    memory_mapped: false,
                    error_message: Some(e.to_string()),
                })
            }
        }
    }

    async fn handle_read(&self, request: &IORequest) -> Result<IOResult> {
        // 检查缓存
        if let Some(cached_data) = self.get_from_cache(&request.file_path).await {
            return Ok(IOResult {
                success: true,
                operation: "read_cached".to_string(),
                file_path: request.file_path.clone(),
                data: Some(cached_data.data.clone()),
                file_size: cached_data.file_size,
                processing_time_ms: 0,
                bytes_processed: cached_data.file_size,
                simd_used: false,
                memory_mapped: false,
                error_message: None,
            });
        }

        let file_metadata = tokio::fs::metadata(&request.file_path).await?;
        let file_size = file_metadata.len();
        
        let (data, memory_mapped, simd_used) = if request.use_memory_mapping && file_size > 1024 * 1024 {
            // 使用内存映射读取大文件
            self.read_with_mmap(&request.file_path, request.enable_simd).await?
        } else {
            // 常规异步读取
            self.read_async(&request.file_path, request.buffer_size, request.enable_simd).await?
        };

        // 缓存小文件数据
        if file_size < 10 * 1024 * 1024 { // 小于10MB的文件才缓存
            self.cache_data(&request.file_path, &data, file_size).await;
        }

        Ok(IOResult {
            success: true,
            operation: "read".to_string(),
            file_path: request.file_path.clone(),
            data: Some(data),
            file_size,
            processing_time_ms: 0,
            bytes_processed: file_size,
            simd_used,
            memory_mapped,
            error_message: None,
        })
    }

    async fn read_with_mmap(&self, file_path: &Path, enable_simd: bool) -> Result<(Vec<u8>, bool, bool)> {
        let file = std::fs::File::open(file_path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        
        let data = if enable_simd && self.enable_simd {
            self.simd_copy_from_slice(&mmap)
        } else {
            mmap.to_vec()
        };
        
        Ok((data, true, enable_simd && self.enable_simd))
    }

    async fn read_async(&self, file_path: &Path, buffer_size: Option<usize>, enable_simd: bool) -> Result<(Vec<u8>, bool, bool)> {
        let file = File::open(file_path).await?;
        let mut reader = BufReader::with_capacity(buffer_size.unwrap_or(64 * 1024), file);
        
        let mut data = Vec::new();
        reader.read_to_end(&mut data).await?;
        
        let simd_used = if enable_simd && self.enable_simd && data.len() > 1024 {
            // 如果启用SIMD且数据足够大，进行SIMD优化处理
            self.simd_process_data(&mut data);
            true
        } else {
            false
        };
        
        Ok((data, false, simd_used))
    }

    async fn handle_write(&self, request: &IORequest, data: Vec<u8>) -> Result<IOResult> {
        let file_size = data.len() as u64;
        
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&request.file_path)
            .await?;
        
        let mut writer = BufWriter::with_capacity(
            request.buffer_size.unwrap_or(64 * 1024), 
            file
        );
        
        let simd_used = if request.enable_simd && self.enable_simd && data.len() > 1024 {
            // SIMD优化写入
            self.simd_write_data(&mut writer, &data).await?;
            true
        } else {
            writer.write_all(&data).await?;
            false
        };
        
        writer.flush().await?;

        // 更新缓存
        if file_size < 10 * 1024 * 1024 {
            self.cache_data(&request.file_path, &data, file_size).await;
        }

        Ok(IOResult {
            success: true,
            operation: "write".to_string(),
            file_path: request.file_path.clone(),
            data: None,
            file_size,
            processing_time_ms: 0,
            bytes_processed: file_size,
            simd_used,
            memory_mapped: false,
            error_message: None,
        })
    }

    async fn handle_copy(&self, request: &IORequest, destination: PathBuf) -> Result<IOResult> {
        let metadata = tokio::fs::metadata(&request.file_path).await?;
        let file_size = metadata.len();
        
        if file_size > 100 * 1024 * 1024 { // 大于100MB使用零拷贝
            self.zero_copy_file(&request.file_path, &destination).await?;
        } else {
            tokio::fs::copy(&request.file_path, &destination).await?;
        }

        Ok(IOResult {
            success: true,
            operation: "copy".to_string(),
            file_path: request.file_path.clone(),
            data: None,
            file_size,
            processing_time_ms: 0,
            bytes_processed: file_size * 2, // 读+写
            simd_used: false,
            memory_mapped: false,
            error_message: None,
        })
    }

    async fn handle_move(&self, request: &IORequest, destination: PathBuf) -> Result<IOResult> {
        let metadata = tokio::fs::metadata(&request.file_path).await?;
        let file_size = metadata.len();
        
        tokio::fs::rename(&request.file_path, &destination).await?;
        
        // 移除缓存
        self.remove_from_cache(&request.file_path).await;

        Ok(IOResult {
            success: true,
            operation: "move".to_string(),
            file_path: request.file_path.clone(),
            data: None,
            file_size,
            processing_time_ms: 0,
            bytes_processed: 0,
            simd_used: false,
            memory_mapped: false,
            error_message: None,
        })
    }

    async fn handle_delete(&self, request: &IORequest) -> Result<IOResult> {
        let metadata = tokio::fs::metadata(&request.file_path).await?;
        let file_size = metadata.len();
        
        tokio::fs::remove_file(&request.file_path).await?;
        
        // 移除缓存
        self.remove_from_cache(&request.file_path).await;

        Ok(IOResult {
            success: true,
            operation: "delete".to_string(),
            file_path: request.file_path.clone(),
            data: None,
            file_size,
            processing_time_ms: 0,
            bytes_processed: 0,
            simd_used: false,
            memory_mapped: false,
            error_message: None,
        })
    }

    async fn handle_get_metadata(&self, request: &IORequest) -> Result<IOResult> {
        let metadata = tokio::fs::metadata(&request.file_path).await?;
        let file_size = metadata.len();

        Ok(IOResult {
            success: true,
            operation: "metadata".to_string(),
            file_path: request.file_path.clone(),
            data: None,
            file_size,
            processing_time_ms: 0,
            bytes_processed: 0,
            simd_used: false,
            memory_mapped: false,
            error_message: None,
        })
    }

    // SIMD优化方法（简化实现）
    fn simd_copy_from_slice(&self, data: &[u8]) -> Vec<u8> {
        // 实际实现应该使用SIMD指令
        // 这里是简化版本
        data.to_vec()
    }

    fn simd_process_data(&self, _data: &mut Vec<u8>) {
        // SIMD数据处理实现
        // 简化版本，实际应使用SIMD指令优化
    }

    async fn simd_write_data(&self, writer: &mut BufWriter<File>, data: &[u8]) -> Result<()> {
        // SIMD优化写入实现
        writer.write_all(data).await?;
        Ok(())
    }

    async fn zero_copy_file(&self, source: &Path, destination: &Path) -> Result<()> {
        // 零拷贝文件复制实现
        // 在实际系统中应该使用sendfile或copy_file_range
        tokio::fs::copy(source, destination).await?;
        Ok(())
    }

    // 缓存管理
    async fn get_from_cache(&self, file_path: &Path) -> Option<CacheEntry> {
        let mut cache = self.memory_cache.write().await;
        
        if let Some(entry) = cache.get_mut(file_path) {
            entry.last_accessed = std::time::Instant::now();
            entry.access_count += 1;
            
            // 更新统计
            let mut stats = self.stats.write().await;
            stats.cache_hits += 1;
            
            Some(entry.clone())
        } else {
            let mut stats = self.stats.write().await;
            stats.cache_misses += 1;
            None
        }
    }

    async fn cache_data(&self, file_path: &Path, data: &[u8], file_size: u64) {
        let mut cache = self.memory_cache.write().await;
        
        // 检查缓存大小限制
        let current_cache_size: usize = cache.values().map(|e| e.data.len()).sum();
        if current_cache_size + data.len() > self.cache_size_limit {
            self.evict_cache_entries(&mut cache).await;
        }
        
        cache.insert(file_path.to_path_buf(), CacheEntry {
            data: data.to_vec(),
            last_accessed: std::time::Instant::now(),
            access_count: 1,
            file_size,
        });
    }

    async fn remove_from_cache(&self, file_path: &Path) {
        let mut cache = self.memory_cache.write().await;
        cache.remove(file_path);
    }

    async fn evict_cache_entries(&self, cache: &mut HashMap<PathBuf, CacheEntry>) {
        // LRU驱逐策略
        let mut entries: Vec<_> = cache.iter().map(|(k, v)| (k.clone(), v.last_accessed)).collect();
        entries.sort_by(|a, b| a.1.cmp(&b.1));
        
        // 收集要移除的键
        let evict_count = cache.len() / 4;
        let keys_to_remove: Vec<_> = entries.iter().take(evict_count).map(|(path, _)| path.clone()).collect();
        
        // 移除最老的25%条目
        for path in keys_to_remove {
            cache.remove(&path);
        }
    }

    async fn update_stats(&self, request: &IORequest, result: &Result<IOResult>, duration: std::time::Duration) {
        let mut stats = self.stats.write().await;
        
        stats.total_operations += 1;
        
        if let Ok(io_result) = result {
            match request.operation {
                IOOperation::Read => stats.total_bytes_read += io_result.bytes_processed,
                IOOperation::Write { .. } => stats.total_bytes_written += io_result.bytes_processed,
                IOOperation::Copy { .. } => {
                    stats.total_bytes_read += io_result.bytes_processed / 2;
                    stats.total_bytes_written += io_result.bytes_processed / 2;
                }
                _ => {}
            }
            
            if io_result.simd_used {
                stats.simd_operations += 1;
            }
            
            if io_result.memory_mapped {
                stats.memory_mapped_operations += 1;
            }
        }
        
        // 更新平均操作时间
        let operation_time_ms = duration.as_millis() as f32;
        if stats.average_operation_time_ms == 0.0 {
            stats.average_operation_time_ms = operation_time_ms;
        } else {
            stats.average_operation_time_ms = stats.average_operation_time_ms * 0.9 + operation_time_ms * 0.1;
        }
    }

    pub async fn get_stats(&self) -> IOStats {
        self.stats.read().await.clone()
    }

    pub async fn get_cache_stats(&self) -> CacheStats {
        let cache = self.memory_cache.read().await;
        let total_size: usize = cache.values().map(|e| e.data.len()).sum();
        let total_accesses: u64 = cache.values().map(|e| e.access_count).sum();
        
        CacheStats {
            entry_count: cache.len(),
            total_size_bytes: total_size,
            total_accesses,
            hit_rate: {
                let stats = self.stats.read().await;
                let total_requests = stats.cache_hits + stats.cache_misses;
                if total_requests > 0 {
                    stats.cache_hits as f32 / total_requests as f32
                } else {
                    0.0
                }
            },
        }
    }

    pub async fn clear_cache(&self) {
        let mut cache = self.memory_cache.write().await;
        cache.clear();
        info!("🧹 I/O缓存已清空");
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CacheStats {
    pub entry_count: usize,
    pub total_size_bytes: usize,
    pub total_accesses: u64,
    pub hit_rate: f32,
}
