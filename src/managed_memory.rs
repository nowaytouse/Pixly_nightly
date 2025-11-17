//! 托管内存模块
//! 
//! 提供自动内存管理和内存池功能

use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::ptr::NonNull;

/// 内存块
#[derive(Debug)]
pub struct MemoryBlock {
    ptr: NonNull<u8>,
    size: usize,
    alignment: usize,
    in_use: bool,
}

unsafe impl Send for MemoryBlock {}
unsafe impl Sync for MemoryBlock {}

impl MemoryBlock {
    pub fn new(size: usize, alignment: usize) -> Result<Self> {
        let layout = std::alloc::Layout::from_size_align(size, alignment)?;
        
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            anyhow::bail!("Memory allocation failed: {} bytes", size);
        }
        
        Ok(Self {
            ptr: NonNull::new(ptr).unwrap(),
            size,
            alignment,
            in_use: false,
        })
    }
    
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }
    
    /// # Safety
    /// 调用者必须确保内存块在使用期间有效
    pub unsafe fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.size) }
    }
    
    /// # Safety
    /// 调用者必须确保内存块在使用期间有效且没有其他引用
    pub unsafe fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.size) }
    }
}

impl Drop for MemoryBlock {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::from_size_align(self.size, self.alignment).unwrap();
        unsafe {
            std::alloc::dealloc(self.ptr.as_ptr(), layout);
        }
    }
}

/// 内存统计
#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    pub total_allocations: u64,
    pub active_allocations: u64,
    pub peak_usage: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

/// 内存管理器
pub struct MemoryManager {
    pool: Arc<Mutex<VecDeque<MemoryBlock>>>,
    #[allow(dead_code)]
    total_size: usize,
    block_size: usize,
    alignment: usize,
    stats: Arc<Mutex<MemoryStats>>,
}

impl MemoryManager {
    pub fn new(total_size: usize) -> Result<Self> {
        let block_size = 4 * 1024 * 1024; // 4MB块
        let alignment = 64; // 64字节对齐
        let num_blocks = total_size / block_size;
        
        let mut pool = VecDeque::with_capacity(num_blocks);
        
        for _ in 0..num_blocks {
            let block = MemoryBlock::new(block_size, alignment)?;
            pool.push_back(block);
        }
        
        Ok(Self {
            pool: Arc::new(Mutex::new(pool)),
            total_size,
            block_size,
            alignment,
            stats: Arc::new(Mutex::new(MemoryStats::default())),
        })
    }
    
    pub fn allocate(&self, size: usize) -> Result<ManagedMemory> {
        let mut stats = self.stats.lock().unwrap();
        stats.total_allocations += 1;
        
        if size <= self.block_size {
            let mut pool = self.pool.lock().unwrap();
            if let Some(mut block) = pool.pop_front() {
                block.in_use = true;
                stats.cache_hits += 1;
                stats.active_allocations += 1;
                
                if stats.active_allocations > stats.peak_usage {
                    stats.peak_usage = stats.active_allocations;
                }
                
                drop(stats);
                return Ok(ManagedMemory::new(block, self.stats.clone()));
            } else {
                stats.cache_misses += 1;
            }
        }
        
        let block = MemoryBlock::new(size, self.alignment)?;
        stats.active_allocations += 1;
        
        if stats.active_allocations > stats.peak_usage {
            stats.peak_usage = stats.active_allocations;
        }
        
        drop(stats);
        Ok(ManagedMemory::new(block, self.stats.clone()))
    }
    
    pub fn get_stats(&self) -> MemoryStats {
        self.stats.lock().unwrap().clone()
    }
    
    pub fn warmup(&self) {
        let mut temp_allocations = Vec::new();
        
        for _ in 0..5 {
            if let Ok(mem) = self.allocate(self.block_size) {
                temp_allocations.push(mem);
            }
        }
        
        drop(temp_allocations);
    }
}

/// 托管内存
pub struct ManagedMemory {
    block: Option<MemoryBlock>,
    stats: Arc<Mutex<MemoryStats>>,
}

impl ManagedMemory {
    fn new(block: MemoryBlock, stats: Arc<Mutex<MemoryStats>>) -> Self {
        Self {
            block: Some(block),
            stats,
        }
    }
    
    pub fn as_ptr(&self) -> *mut u8 {
        self.block.as_ref().unwrap().as_ptr()
    }
    
    pub fn size(&self) -> usize {
        self.block.as_ref().unwrap().size
    }
    
    /// # Safety
    /// 调用者必须确保内存块在使用期间有效
    pub unsafe fn as_slice(&self) -> &[u8] {
        unsafe { self.block.as_ref().unwrap().as_slice() }
    }
    
    /// # Safety
    /// 调用者必须确保内存块在使用期间有效且没有其他引用
    pub unsafe fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { self.block.as_mut().unwrap().as_slice_mut() }
    }
    
    pub fn write_data(&mut self, data: &[u8]) -> Result<()> {
        if data.len() > self.size() {
            anyhow::bail!("Data size {} exceeds memory block size {}", data.len(), self.size());
        }
        
        unsafe {
            let slice = self.as_slice_mut();
            slice[..data.len()].copy_from_slice(data);
        }
        
        Ok(())
    }
    
    pub fn read_data(&self, len: usize) -> Result<&[u8]> {
        if len > self.size() {
            anyhow::bail!("Read length {} exceeds memory block size {}", len, self.size());
        }
        
        unsafe {
            let slice = self.as_slice();
            Ok(&slice[..len])
        }
    }
}

impl Drop for ManagedMemory {
    fn drop(&mut self) {
        let mut stats = self.stats.lock().unwrap();
        stats.active_allocations = stats.active_allocations.saturating_sub(1);
    }
}
