//! 零拷贝缓冲区模块
//! 
//! 提供高性能零拷贝内存管理

use anyhow::{Result, Context};

/// 零拷贝缓冲区
pub struct ZeroCopyBuffer {
    data: *mut u8,
    len: usize,
    capacity: usize,
    shared: bool,
}

impl ZeroCopyBuffer {
    /// 从现有数据创建 (共享)
    ///
    /// # Safety
    /// 调用者必须确保:
    /// - `data` 指针在整个生命周期内有效
    /// - `len` 准确反映数据长度
    /// - 数据不会被其他代码修改或释放
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
            .context("Failed to create buffer layout")?;
        
        let data = unsafe { std::alloc::alloc(layout) };
        if data.is_null() {
            anyhow::bail!("Buffer allocation failed: {} bytes", capacity);
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
    ///
    /// # Safety
    /// 调用者必须确保数据指针有效且长度正确
    pub unsafe fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data, self.len) }
    }
    
    /// 获取可变切片
    ///
    /// # Safety
    /// 调用者必须确保数据指针有效且没有其他引用
    pub unsafe fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.data, self.len) }
    }
    
    /// 设置长度
    ///
    /// # Safety
    /// 调用者必须确保新长度不超过容量且数据已初始化
    pub unsafe fn set_len(&mut self, len: usize) {
        debug_assert!(len <= self.capacity);
        self.len = len;
    }
    
    /// 追加数据
    pub fn extend_from_slice(&mut self, data: &[u8]) -> Result<()> {
        if self.len + data.len() > self.capacity {
            anyhow::bail!("Insufficient buffer space: {} + {} > {}", 
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
        let mut vec = vec![0u8; self.len];
        unsafe {
            std::ptr::copy_nonoverlapping(self.data, vec.as_mut_ptr(), self.len);
        }
        Ok(vec)
    }
    
    /// 写入数据到缓冲区
    pub fn write_data(&mut self, data: &[u8]) -> Result<()> {
        if data.len() > self.capacity {
            anyhow::bail!("Data size {} exceeds buffer capacity {}", data.len(), self.capacity);
        }
        
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), self.data, data.len());
            self.len = data.len();
        }
        
        Ok(())
    }
    
    /// 从缓冲区读取数据
    pub fn read_data(&self, len: usize) -> Result<&[u8]> {
        if len > self.len {
            anyhow::bail!("Read length {} exceeds buffer length {}", len, self.len);
        }
        
        unsafe {
            Ok(std::slice::from_raw_parts(self.data, len))
        }
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
