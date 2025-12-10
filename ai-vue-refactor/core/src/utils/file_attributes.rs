/// filepropertymodule
///
/// feature：
/// 1. filetime（createtime、modifiedtime、time）
/// 2.  mac OS extensionproperty（xattr）
/// 3.  Linux extensionproperty
/// 4. Windows ADS support（passfilecopied）
use anyhow::{Context, Result};
use std::path::Path;
use std::fs;

#[cfg(unix)]
use filetime::{FileTime, set_file_times};

/// filepropertyfast
#[derive(Debug, Clone)]
pub struct FileAttributes {
/// modifiedtime
 pub modified: Option<std::time::SystemTime>,
/// time
 pub accessed: Option<std::time::SystemTime>,
/// mac OS/Linux extensionproperty
 #[cfg(any(target_os = "macos", target_os = "linux"))]
 pub xattrs: Vec<(String, Vec<u8>)>,
}

impl FileAttributes {
/// catchfile has property
 pub fn capture(path: &Path) -> Result<Self> {
 let metadata = fs::metadata(path)
 .with_context(|| format!("Failed to read metadata: {:?}", path))?;

 let modified = metadata.modified().ok();
 let accessed = metadata.accessed().ok();

 #[cfg(any(target_os = "macos", target_os = "linux"))]
 let xattrs = Self::capture_xattrs(path)?;

 Ok(Self {
 modified,
 accessed,
 #[cfg(any(target_os = "macos", target_os = "linux"))]
 xattrs,
 })
 }

/// catchextensionproperty（mac OS/Linux）
 #[cfg(any(target_os = "macos", target_os = "linux"))]
 fn capture_xattrs(path: &Path) -> Result<Vec<(String, Vec<u8>)>> {
 use xattr;

 let mut attrs = Vec::new();

 match xattr::list(path) {
 Ok(names) => {
 for name in names {
 if let Some(name_str) = name.to_str()
 && let Ok(Some(value)) = xattr::get(path, &name) {
 attrs.push((name_str.to_string(), value));
 }
 }
 }
 Err(e) => {
// extensionpropertyreadfailure not shouldconversion
 log::warn!("Failed to read xattrs from {:?}: {}", path, e);
 }
 }

 Ok(attrs)
 }

/// applypropertytotargetfile
 pub fn apply(&self, path: &Path) -> Result<()> {
// 1. recoverytime
 self.apply_timestamps(path)?;

// 2. recoveryextensionproperty
 #[cfg(any(target_os = "macos", target_os = "linux"))]
 self.apply_xattrs(path)?;

 Ok(())
 }

/// applytime
 fn apply_timestamps(&self, path: &Path) -> Result<()> {
 if let (Some(modified), Some(accessed)) = (self.modified, self.accessed) {
 #[cfg(unix)]
 {
 let mtime = FileTime::from_system_time(modified);
 let atime = FileTime::from_system_time(accessed);

 set_file_times(path, atime, mtime)
 .with_context(|| format!("Failed to set file times: {:?}", path))?;

 log::debug!("Restored timestamps for {:?}", path);
 }

 #[cfg(windows)]
 {
// Windows usedifferent API
 use std::os::windows::fs::MetadataExt;
 use std::os::windows::io::AsRawHandle;
 use std::fs::File;

// Windows timerecoveryneeduse Win32 API
// thisprocessing，onlyrecoverymodifiedtime
 let file = File::options().write(true).open(path)?;
 file.set_modified(modified)?;

 log::debug!("Restored modified time for {:?}", path);
 }
 }

 Ok(())
 }

/// applyextensionproperty（mac OS/Linux）
 #[cfg(any(target_os = "macos", target_os = "linux"))]
 fn apply_xattrs(&self, path: &Path) -> Result<()> {
 use xattr;

 for (name, value) in &self.xattrs {
 match xattr::set(path, name, value) {
 Ok(_) => {
 log::debug!("Restored xattr '{}' for {:?}", name, path);
 }
 Err(e) => {
// Some extended attributes may fail to set (permissions, unsupported filesystem, etc.)
 log::warn!("Failed to set xattr '{}' for {:?}: {}", name, path, e);
 }
 }
 }

 if !self.xattrs.is_empty() {
 log::info!("Restored {} extended attributes", self.xattrs.len());
 }

 Ok(())
 }
}

/// function：filepropertyconversion
///
/// use：
/// ```no_run
/// use pixly_kernel::utils::file_attributes::FileAttributes;
/// use std::path::Path;
/// # fn main() -> anyhow::Result<()> {
/// let input_path = Path::new("input.jpg");
/// let output_path = Path::new("output.webp");
/// let attrs = FileAttributes::capture(&input_path)?;
/// // ... executeconversion ...
/// attrs.apply(&output_path)?;
/// # Ok(())
/// # }
/// ```
pub fn preserve_attributes<F>(input: &Path, output: &Path, convert_fn: F) -> Result<()>
where
 F: FnOnce() -> Result<()>,
{
// 1. catchoriginalproperty
 let attrs = FileAttributes::capture(input)?;

// 2. executeconversion
 convert_fn()?;

// 3. recoveryproperty
 attrs.apply(output)?;

 Ok(())
}

#[cfg(test)]
mod tests {
 use super::*;
 use std::fs::File;
 use std::io::Write;
 use tempfile::tempdir;

 #[test]
 fn test_capture_and_apply_timestamps() {
 let dir = tempdir().unwrap();
 let input = dir.path().join("input.txt");
 let output = dir.path().join("output.txt");

// createtestfile
 File::create(&input).unwrap().write_all(b"test").unwrap();

// waitaensuretimedifferent
 std::thread::sleep(std::time::Duration::from_secs(1));

// catchproperty
 let attrs = FileAttributes::capture(&input).unwrap();

// createoutputfile
 File::create(&output).unwrap().write_all(b"converted").unwrap();

// applyproperty
 attrs.apply(&output).unwrap();

// validationtime
 let input_meta = fs::metadata(&input).unwrap();
 let output_meta = fs::metadata(&output).unwrap();

 assert_eq!(
 input_meta.modified().unwrap(),
 output_meta.modified().unwrap()
 );
 }

 #[test]
 #[cfg(any(target_os = "macos", target_os = "linux"))]
 fn test_capture_and_apply_xattrs() {
 use xattr;

 let dir = tempdir().unwrap();
 let input = dir.path().join("input.txt");
 let output = dir.path().join("output.txt");

// createtestfile
 File::create(&input).unwrap().write_all(b"test").unwrap();

// settingextensionproperty
 xattr::set(&input, "user.test", b"test_value").unwrap();

// catchproperty
 let attrs = FileAttributes::capture(&input).unwrap();
 assert!(!attrs.xattrs.is_empty());

// createoutputfile
 File::create(&output).unwrap().write_all(b"converted").unwrap();

// applyproperty
 attrs.apply(&output).unwrap();

// validationextensionproperty
 let value = xattr::get(&output, "user.test").unwrap().unwrap();
 assert_eq!(value, b"test_value");
 }
}
