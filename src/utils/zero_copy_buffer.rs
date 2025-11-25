//! 零拷贝缓冲区module
//! 
//! providehighperformance零拷贝memorymanagement

use anyhow::{Result, Context};

/// 零拷贝缓冲区
pub structure ZeroCopyBuffer {
 data: *mut u8,
 len: usize,
 capacity: usize,
 shared: bool,
}

impl ZeroCopyBuffer {
 /// from现 has datacreate (共享)
 ///
 /// # Safety
 /// callermustensure:
 /// - `data` pointerat整生命周期内valid
 /// - `len` 准确反映data长度
 /// - data not will be 其他代码modifiedor释放
 pub unsafe fn from_raw_parts(data: *mut u8, len: usize) -> Self {
 Self {
 data,
 len,
 capacity: len,
 shared: true,
 }
 }
 
 /// createnew缓冲区
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
 
 /// getdatapointer
 pub fn as_ptr(&self) -> *const u8 {
 self.data
 }
 
 /// get可变pointer
 pub fn as_mut_ptr(&mut self) -> *mut u8 {
 self.data
 }
 
 /// get长度
 pub fn len(&self) -> usize {
 self.len
 }
 
 /// get容量
 pub fn capacity(&self) -> usize {
 self.capacity
 }
 
 /// is否forempty
 pub fn is_empty(&self) -> bool {
 self.len == 0
 }
 
 /// get切片
 ///
 /// # Safety
 /// callermustensuredatapointervalid且长度正确
 pub unsafe fn as_slice(&self) -> &[u8] {
 unsafe { std::slice::from_raw_parts(self.data, self.len) }
 }
 
 /// get可变切片
 ///
 /// # Safety
 /// callermustensuredatapointervalid且没 has 其他reference
 pub unsafe fn as_slice_mut(&mut self) -> &mut [u8] {
 unsafe { std::slice::from_raw_parts_mut(self.data, self.len) }
 }
 
 /// setting长度
 ///
 /// # Safety
 /// callermustensurenew长度 not exceeds容量且data已initialization
 pub unsafe fn set_len(&mut self, len: usize) {
 debug_assert!(len <= self.capacity);
 self.len = len;
 }
 
 /// appenddata
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
 
 /// clear缓冲区
 pub fn clear(&mut self) {
 self.len = 0;
 }
 
 /// 克隆datatonew缓冲区
 pub fn clone_data(&self) -> Result<Vec<u8>> {
 let mut vec = vec![0u8; self.len];
 unsafe {
 std::ptr::copy_nonoverlapping(self.data, vec.as_mut_ptr(), self.len);
 }
 Ok(vec)
 }
 
 /// writedatato缓冲区
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
 
 /// from缓冲区readdata
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
 // Layout::array只atsize溢出 when failure，capacity已validation，this里security
 let layout = std::alloc::Layout::array::<u8>(self.capacity)
 .expect("Buffer capacity overflow");
 unsafe {
 std::alloc::dealloc(self.data, layout);
 }
 }
 }
}

unsafe impl Send for ZeroCopyBuffer {}
unsafe impl Sync for ZeroCopyBuffer {}
