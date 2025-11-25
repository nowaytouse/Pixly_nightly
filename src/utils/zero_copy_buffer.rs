//! module
//!
//! providehighperformancememorymanagement

use anyhow::{Result, Context};

/// 
pub struct ZeroCopyBuffer {
 data: *mut u8,
 len: usize,
 capacity: usize,
 shared: bool,
}

impl ZeroCopyBuffer {
/// from has datacreate ()
///
/// # Safety
/// callermustensure:
/// - `data` pointeratwholeperiodinsidevalid
/// - `len` datalength
/// - data not will be itsmodifiedor
 pub unsafe fn from_raw_parts(data: *mut u8, len: usize) -> Self {
 Self {
 data,
 len,
 capacity: len,
 shared: true,
 }
 }

/// createnew
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

/// getcanpointer
 pub fn as_mut_ptr(&mut self) -> *mut u8 {
 self.data
 }

/// getlength
 pub fn len(&self) -> usize {
 self.len
 }

/// get
 pub fn capacity(&self) -> usize {
 self.capacity
 }

/// isnoforempty
 pub fn is_empty(&self) -> bool {
 self.len == 0
 }

/// get
///
/// # Safety
/// callermustensuredatapointervalidandlengthpositive
 pub unsafe fn as_slice(&self) -> &[u8] {
 unsafe { std::slice::from_raw_parts(self.data, self.len) }
 }

/// getcan
///
/// # Safety
/// callermustensuredatapointervalidand has itsreference
 pub unsafe fn as_slice_mut(&mut self) -> &mut [u8] {
 unsafe { std::slice::from_raw_parts_mut(self.data, self.len) }
 }

/// settinglength
///
/// # Safety
/// callermustensurenewlength not exceedsanddataalreadyinitialization
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

/// clear
 pub fn clear(&mut self) {
 self.len = 0;
 }

/// datatonew
 pub fn clone_data(&self) -> Result<Vec<u8>> {
 let mut vec = vec![0u8; self.len];
 unsafe {
 std::ptr::copy_nonoverlapping(self.data, vec.as_mut_ptr(), self.len);
 }
 Ok(vec)
 }

/// writedatato
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

/// fromreaddata
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
// Layout::arrayonlyatsize when failure，capacityalreadyvalidation，thissecurity
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
