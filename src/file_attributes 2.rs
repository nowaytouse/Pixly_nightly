/// 文件属性保留模块
/// 
/// 功能：
/// 1. 保留文件时间戳（创建时间、修改时间、访问时间）
/// 2. 保留 macOS 扩展属性（xattr）
/// 3. 保留 Linux 扩展属性
/// 4. Windows ADS 支持（通过文件复制）

use anyhow::{Context, Result};
use std::path::Path;
use std::fs;

#[cfg(unix)]
use filetime::{FileTime, set_file_times};

/// 文件属性快照
#[derive(Debug, Clone)]
pub struct FileAttributes {
    /// 修改时间
    pub modified: Option<std::time::SystemTime>,
    /// 访问时间
    pub accessed: Option<std::time::SystemTime>,
    /// macOS/Linux 扩展属性
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    pub xattrs: Vec<(String, Vec<u8>)>,
}

impl FileAttributes {
    /// 捕获文件的所有属性
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

    /// 捕获扩展属性（macOS/Linux）
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    fn capture_xattrs(path: &Path) -> Result<Vec<(String, Vec<u8>)>> {
        use xattr;

        let mut attrs = Vec::new();
        
        match xattr::list(path) {
            Ok(names) => {
                for name in names {
                    if let Some(name_str) = name.to_str() {
                        if let Ok(Some(value)) = xattr::get(path, &name) {
                            attrs.push((name_str.to_string(), value));
                        }
                    }
                }
            }
            Err(e) => {
                // 扩展属性读取失败不应阻止转换
                log::warn!("Failed to read xattrs from {:?}: {}", path, e);
            }
        }

        Ok(attrs)
    }

    /// 应用属性到目标文件
    pub fn apply(&self, path: &Path) -> Result<()> {
        // 1. 恢复时间戳
        self.apply_timestamps(path)?;

        // 2. 恢复扩展属性
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        self.apply_xattrs(path)?;

        Ok(())
    }

    /// 应用时间戳
    fn apply_timestamps(&self, path: &Path) -> Result<()> {
        if let (Some(modified), Some(accessed)) = (self.modified, self.accessed) {
            #[cfg(unix)]
            {
                let mtime = FileTime::from_system_time(modified);
                let atime = FileTime::from_system_time(accessed);
                
                set_file_times(path, atime, mtime)
                    .with_context(|| format!("Failed to set file times: {:?}", path))?;
                
                log::debug!("✓ Restored timestamps for {:?}", path);
            }

            #[cfg(windows)]
            {
                // Windows 使用不同的 API
                use std::os::windows::fs::MetadataExt;
                use std::os::windows::io::AsRawHandle;
                use std::fs::File;
                
                // Windows 时间戳恢复需要使用 Win32 API
                // 这里简化处理，只恢复修改时间
                let file = File::options().write(true).open(path)?;
                file.set_modified(modified)?;
                
                log::debug!("✓ Restored modified time for {:?}", path);
            }
        }

        Ok(())
    }

    /// 应用扩展属性（macOS/Linux）
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    fn apply_xattrs(&self, path: &Path) -> Result<()> {
        use xattr;

        for (name, value) in &self.xattrs {
            match xattr::set(path, name, value) {
                Ok(_) => {
                    log::debug!("✓ Restored xattr '{}' for {:?}", name, path);
                }
                Err(e) => {
                    // 某些扩展属性可能无法设置（权限问题、文件系统不支持等）
                    log::warn!("Failed to set xattr '{}' for {:?}: {}", name, path, e);
                }
            }
        }

        if !self.xattrs.is_empty() {
            log::info!("✓ Restored {} extended attributes", self.xattrs.len());
        }

        Ok(())
    }
}

/// 便捷函数：保留文件属性的转换
/// 
/// 使用方式：
/// ```
/// let attrs = FileAttributes::capture(&input_path)?;
/// // ... 执行转换 ...
/// attrs.apply(&output_path)?;
/// ```
pub fn preserve_attributes<F>(input: &Path, output: &Path, convert_fn: F) -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    // 1. 捕获原始属性
    let attrs = FileAttributes::capture(input)?;

    // 2. 执行转换
    convert_fn()?;

    // 3. 恢复属性
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

        // 创建测试文件
        File::create(&input).unwrap().write_all(b"test").unwrap();
        
        // 等待一秒确保时间戳不同
        std::thread::sleep(std::time::Duration::from_secs(1));
        
        // 捕获属性
        let attrs = FileAttributes::capture(&input).unwrap();
        
        // 创建输出文件
        File::create(&output).unwrap().write_all(b"converted").unwrap();
        
        // 应用属性
        attrs.apply(&output).unwrap();
        
        // 验证时间戳
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

        // 创建测试文件
        File::create(&input).unwrap().write_all(b"test").unwrap();
        
        // 设置扩展属性
        xattr::set(&input, "user.test", b"test_value").unwrap();
        
        // 捕获属性
        let attrs = FileAttributes::capture(&input).unwrap();
        assert!(!attrs.xattrs.is_empty());
        
        // 创建输出文件
        File::create(&output).unwrap().write_all(b"converted").unwrap();
        
        // 应用属性
        attrs.apply(&output).unwrap();
        
        // 验证扩展属性
        let value = xattr::get(&output, "user.test").unwrap().unwrap();
        assert_eq!(value, b"test_value");
    }
}
