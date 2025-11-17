// 💾 内存管理器
// 从 @archive/rust_broken/src/performance/memory_manager.rs 提取

use std::sync::{Arc, Mutex};
use anyhow::Result;

#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    pub total_allocations: u64,
    pub active_allocations: u64,
    pub peak_usage: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

pub struct MemoryManager {
    total_size: usize,
    block_size: usize,
    alignment: usize,
    stats: Arc<Mutex<MemoryStats>>,
}

impl MemoryManager {
    pub fn new(total_size: usize) -> Result<Self> {
        let block_size = 4 * 1024 * 1024;
        let alignment = 64;
        
        Ok(Self {
            total_size,
            block_size,
            alignment,
            stats: Arc::new(Mutex::new(MemoryStats::default())),
        })
    }
    
    pub fn get_stats(&self) -> MemoryStats {
        self.stats.lock().unwrap().clone()
    }
    
    pub fn total_size(&self) -> usize {
        self.total_size
    }
    
    pub fn block_size(&self) -> usize {
        self.block_size
    }
    
    pub fn alignment(&self) -> usize {
        self.alignment
    }
    
    pub fn allocate_aligned(&self, size: usize) -> Result<Vec<u8>> {
        let aligned_size = size.div_ceil(self.alignment) * self.alignment;
        let buffer = vec![0u8; aligned_size];
        
        let mut stats = self.stats.lock().unwrap();
        stats.total_allocations += 1;
        stats.active_allocations += 1;
        
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_manager() {
        let manager = MemoryManager::new(64 * 1024 * 1024).unwrap();
        assert_eq!(manager.total_size(), 64 * 1024 * 1024);
        assert_eq!(manager.block_size(), 4 * 1024 * 1024);
    }
    
    #[test]
    fn test_stats() {
        let manager = MemoryManager::new(64 * 1024 * 1024).unwrap();
        let stats = manager.get_stats();
        assert_eq!(stats.total_allocations, 0);
    }
}
