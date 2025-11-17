// 📋 元数据处理器增强
// 从 @archive/rust_broken/src/converter/metadata.rs 提取

use std::path::Path;
use std::process::Command;
use anyhow::Result;

pub struct MetadataHandler {
    preserve: bool,
}

impl MetadataHandler {
    pub fn new() -> Self {
        Self { preserve: true }
    }
    
    pub fn set_preserve(&mut self, preserve: bool) {
        self.preserve = preserve;
    }
    
    pub fn is_exiftool_available(&self) -> bool {
        Command::new("exiftool")
            .arg("-ver")
            .output()
            .is_ok()
    }
    
    pub fn copy_metadata(&self, input: &Path, output: &Path) -> Result<()> {
        if !self.preserve {
            return Ok(());
        }
        
        if !self.is_exiftool_available() {
            anyhow::bail!("exiftool is not available");
        }
        
        let output = Command::new("exiftool")
            .arg("-TagsFromFile")
            .arg(input)
            .arg("-all:all")
            .arg("-overwrite_original")
            .arg(output)
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("exiftool failed: {}", stderr);
        }
        
        Ok(())
    }
    
    pub fn preserve_timestamps(&self, _input: &Path, _output: &Path) -> Result<()> {
        // 简化实现：时间戳保留功能
        // 实际实现需要filetime或utime crate
        Ok(())
    }
}

impl Default for MetadataHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_handler() {
        let handler = MetadataHandler::new();
        assert!(handler.preserve);
    }
    
    #[test]
    fn test_set_preserve() {
        let mut handler = MetadataHandler::new();
        handler.set_preserve(false);
        assert!(!handler.preserve);
    }
}
