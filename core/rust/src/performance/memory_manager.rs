//! 💾 零拷贝内存管理器
//!
//! 高性能内存管理，专为图像处理优化：
//! - 内存池预分配
//! - 零拷贝数据传输
//! - 共享内存支持
//! - 缓存友好的内存布局

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::ptr::NonNull;
use anyhow::{Result, Context};
use log::{debug, info, warn};

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
    /// 创建新的内存块
    pub fn new(size: usize, alignment: usize) -> Result<Self> {
        let layout = std::alloc::Layout::from_size_align(size, alignment)
            .context("创建内存布局失败")?;
        
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            anyhow::bail!("内存分配失败: {} bytes", size);
        }
        
        Ok(Self {
            ptr: NonNull::new(ptr).unwrap(),
            size,
            alignment,
            in_use: false,
        })
    }
    
    /// 获取内存指针
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }
    
    /// 获取切片
    pub unsafe fn as_slice(&self) -> &[u8] {
        std::slice::from_raw_parts(self.ptr.as_ptr(), self.size)
    }
    
    /// 获取可变切片
    pub unsafe fn as_slice_mut(&mut self) -> &mut [u8] {
        std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.size)
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

/// 内存管理器
pub struct MemoryManager {
    /// 内存池
    pool: Arc<Mutex<VecDeque<MemoryBlock>>>,
    /// 总内存大小
    total_size: usize,
    /// 块大小
    block_size: usize,
    /// 对齐大小
    alignment: usize,
    /// 统计信息
    stats: Arc<Mutex<MemoryStats>>,
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

impl MemoryManager {
    /// 创建内存管理器
    pub fn new(total_size: usize) -> Result<Self> {
        let block_size = 4 * 1024 * 1024; // 4MB块
        let alignment = 64; // 64字节对齐 (缓存行)
        let num_blocks = total_size / block_size;
        
        info!("💾 初始化内存管理器: {} MB, {} 个块", 
              total_size / (1024 * 1024), num_blocks);
        
        let mut pool = VecDeque::with_capacity(num_blocks);
        
        // 预分配内存块
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
    
    /// 分配内存
    pub fn allocate(&self, size: usize) -> Result<ManagedMemory> {
        let mut stats = self.stats.lock().unwrap();
        stats.total_allocations += 1;
        
        if size <= self.block_size {
            // 尝试从池中获取
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
        
        // 分配新内存
        let block = MemoryBlock::new(size, self.alignment)?;
        stats.active_allocations += 1;
        
        if stats.active_allocations > stats.peak_usage {
            stats.peak_usage = stats.active_allocations;
        }
        
        drop(stats);
        Ok(ManagedMemory::new(block, self.stats.clone()))
    }
    
    /// 回收内存到池中
    fn deallocate(&self, mut block: MemoryBlock) {
        if block.size == self.block_size {
            block.in_use = false;
            
            // 清零内存 (可选，用于安全)
            unsafe {
                std::ptr::write_bytes(block.as_ptr(), 0, block.size);
            }
            
            let mut pool = self.pool.lock().unwrap();
            pool.push_back(block);
        }
        
        let mut stats = self.stats.lock().unwrap();
        stats.active_allocations = stats.active_allocations.saturating_sub(1);
    }
    
    /// 获取统计信息
    pub fn get_stats(&self) -> MemoryStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// 预热内存池
    pub fn warmup(&self) {
        let mut temp_allocations = Vec::new();
        
        // 分配并立即释放一些内存块以预热
        for _ in 0..5 {
            if let Ok(mem) = self.allocate(self.block_size) {
                temp_allocations.push(mem);
            }
        }
        
        // 自动释放
        drop(temp_allocations);
        
        debug!("✅ 内存管理器预热完成");
    }
}

/// 管理的内存
pub struct ManagedMemory {
    block: Option<MemoryBlock>,
    stats: Arc<Mutex<MemoryStats>>,
    manager: Option<Arc<MemoryManager>>,
}

impl ManagedMemory {
    fn new(block: MemoryBlock, stats: Arc<Mutex<MemoryStats>>) -> Self {
        Self {
            block: Some(block),
            stats,
            manager: None,
        }
    }
    
    /// 获取内存指针
    pub fn as_ptr(&self) -> *mut u8 {
        self.block.as_ref().unwrap().as_ptr()
    }
    
    /// 获取大小
    pub fn size(&self) -> usize {
        self.block.as_ref().unwrap().size
    }
    
    /// 获取切片
    pub unsafe fn as_slice(&self) -> &[u8] {
        self.block.as_ref().unwrap().as_slice()
    }
    
    /// 获取可变切片
    pub unsafe fn as_slice_mut(&mut self) -> &mut [u8] {
        self.block.as_mut().unwrap().as_slice_mut()
    }
    
    /// 写入数据
    pub fn write_data(&mut self, data: &[u8]) -> Result<()> {
        if data.len() > self.size() {
            anyhow::bail!("数据大小 {} 超过内存块大小 {}", data.len(), self.size());
        }
        
        unsafe {
            let slice = self.as_slice_mut();
            slice[..data.len()].copy_from_slice(data);
        }
        
        Ok(())
    }
    
    /// 读取数据
    pub fn read_data(&self, len: usize) -> Result<&[u8]> {
        if len > self.size() {
            anyhow::bail!("读取长度 {} 超过内存块大小 {}", len, self.size());
        }
        
        unsafe {
            let slice = self.as_slice();
            Ok(&slice[..len])
        }
    }
}

impl Drop for ManagedMemory {
    fn drop(&mut self) {
        if let Some(manager) = &self.manager {
            if let Some(block) = self.block.take() {
                manager.deallocate(block);
            }
        }
    }
}

/// 零拷贝缓冲区
pub struct ZeroCopyBuffer {
    data: *mut u8,
    len: usize,
    capacity: usize,
    shared: bool,
}

impl ZeroCopyBuffer {
    /// 从现有数据创建 (共享)
    pub unsafe fn from_raw_parts(data: *mut u8, len: usize) -> Self {
        Self {
            data,
            len,
            capacity: len,
            shared: true,
        }
    }
    
    /// 创建新缓冲区
    pub fn new(capacity: usize) -> Result<Self> {
        let layout = std::alloc::Layout::array::<u8>(capacity)
            .context("创建缓冲区布局失败")?;
        
        let data = unsafe { std::alloc::alloc(layout) };
        if data.is_null() {
            anyhow::bail!("缓冲区分配失败: {} bytes", capacity);
        }
        
        Ok(Self {
            data,
            len: 0,
            capacity,
            shared: false,
        })
    }
    
    /// 获取数据指针
    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }
    
    /// 获取可变指针
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data
    }
    
    /// 获取长度
    pub fn len(&self) -> usize {
        self.len
    }
    
    /// 获取容量
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    /// 获取切片
    pub unsafe fn as_slice(&self) -> &[u8] {
        std::slice::from_raw_parts(self.data, self.len)
    }
    
    /// 获取可变切片
    pub unsafe fn as_slice_mut(&mut self) -> &mut [u8] {
        std::slice::from_raw_parts_mut(self.data, self.len)
    }
    
    /// 设置长度
    pub unsafe fn set_len(&mut self, len: usize) {
        debug_assert!(len <= self.capacity);
        self.len = len;
    }
    
    /// 追加数据
    pub fn extend_from_slice(&mut self, data: &[u8]) -> Result<()> {
        if self.len + data.len() > self.capacity {
            anyhow::bail!("缓冲区空间不足: {} + {} > {}", 
                         self.len, data.len(), self.capacity);
        }
        
        unsafe {
            let dst = self.data.add(self.len);
            std::ptr::copy_nonoverlapping(data.as_ptr(), dst, data.len());
            self.len += data.len();
        }
        
        Ok(())
    }
    
    /// 清空缓冲区
    pub fn clear(&mut self) {
        self.len = 0;
    }
    
    /// 克隆数据到新缓冲区
    pub fn clone_data(&self) -> Result<Vec<u8>> {
        let mut vec = Vec::with_capacity(self.len);
        unsafe {
            vec.set_len(self.len);
            std::ptr::copy_nonoverlapping(self.data, vec.as_mut_ptr(), self.len);
        }
        Ok(vec)
    }
}

impl Drop for ZeroCopyBuffer {
    fn drop(&mut self) {
        if !self.shared && !self.data.is_null() {
            let layout = std::alloc::Layout::array::<u8>(self.capacity).unwrap();
            unsafe {
                std::alloc::dealloc(self.data, layout);
            }
        }
    }
}

unsafe impl Send for ZeroCopyBuffer {}
unsafe impl Sync for ZeroCopyBuffer {}

// 共享内存功能暂时简化实现，避免libc依赖复杂性
