use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

/// File collector with filtering and recursive support
pub struct FileCollector {
    /// File extensions to include (e.g., ["jpg", "png"])
    pub extensions: Vec<String>,
    
    /// Enable recursive directory traversal
    pub recursive: bool,
    
    /// Maximum recursion depth (None = unlimited)
    pub max_depth: Option<usize>,
    
    /// Follow symbolic links
    pub follow_symlinks: bool,
}

impl Default for FileCollector {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            recursive: false,
            max_depth: None,
            follow_symlinks: false,
        }
    }
}

impl FileCollector {
    /// Create new collector with extensions
    pub fn new(extensions: Vec<String>) -> Self {
        Self {
            extensions,
            ..Default::default()
        }
    }
    
    /// Enable recursive traversal
    pub fn recursive(mut self, enabled: bool) -> Self {
        self.recursive = enabled;
        self
    }
    
    /// Set max depth
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }
    
    /// Collect files from a single path (file or directory)
    pub fn collect(&self, path: &Path) -> Result<Vec<PathBuf>> {
        if !path.exists() {
            anyhow::bail!("Path does not exist: {}", path.display());
        }
        
        if path.is_file() {
            // Single file
            if self.matches_extension(path) {
                Ok(vec![path.to_path_buf()])
            } else {
                Ok(Vec::new())
            }
        } else if path.is_dir() {
            // Directory
            self.collect_from_dir(path, 0)
        } else {
            anyhow::bail!("Path is neither file nor directory: {}", path.display());
        }
    }
    
    /// Collect files from multiple paths
    pub fn collect_multiple(&self, paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut all_files = Vec::new();
        
        for path in paths {
            let files = self.collect(path)?;
            all_files.extend(files);
        }
        
        // Remove duplicates
        all_files.sort();
        all_files.dedup();
        
        Ok(all_files)
    }
    
    /// Collect files from directory
    fn collect_from_dir(&self, dir: &Path, depth: usize) -> Result<Vec<PathBuf>> {
        // Check max depth
        if let Some(max) = self.max_depth {
            if depth >= max {
                return Ok(Vec::new());
            }
        }
        
        let mut files = Vec::new();
        
        let entries = std::fs::read_dir(dir)
            .with_context(|| format!("Failed to read directory: {}", dir.display()))?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            // Skip symlinks if not following
            if !self.follow_symlinks && path.is_symlink() {
                continue;
            }
            
            if path.is_file() {
                if self.matches_extension(&path) {
                    files.push(path);
                }
            } else if path.is_dir() && self.recursive {
                let subfiles = self.collect_from_dir(&path, depth + 1)?;
                files.extend(subfiles);
            }
        }
        
        Ok(files)
    }
    
    /// Check if file matches extension filter
    fn matches_extension(&self, path: &Path) -> bool {
        // If no extensions specified, accept all
        if self.extensions.is_empty() {
            return true;
        }
        
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            self.extensions.iter().any(|e| e.to_lowercase() == ext_str)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    
    #[test]
    fn test_collect_single_file() {
        let temp = TempDir::new().unwrap();
        let file = temp.path().join("test.jpg");
        fs::write(&file, b"test").unwrap();
        
        let collector = FileCollector::new(vec!["jpg".to_string()]);
        let files = collector.collect(&file).unwrap();
        
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], file);
    }
    
    #[test]
    fn test_collect_directory_flat() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join("test1.jpg"), b"test").unwrap();
        fs::write(temp.path().join("test2.png"), b"test").unwrap();
        fs::write(temp.path().join("test3.txt"), b"test").unwrap();
        
        let collector = FileCollector::new(vec!["jpg".to_string(), "png".to_string()]);
        let files = collector.collect(temp.path()).unwrap();
        
        assert_eq!(files.len(), 2);
    }
    
    #[test]
    fn test_collect_directory_recursive() {
        let temp = TempDir::new().unwrap();
        let subdir = temp.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        
        fs::write(temp.path().join("test1.jpg"), b"test").unwrap();
        fs::write(subdir.join("test2.jpg"), b"test").unwrap();
        
        let collector = FileCollector::new(vec!["jpg".to_string()])
            .recursive(true);
        let files = collector.collect(temp.path()).unwrap();
        
        assert_eq!(files.len(), 2);
    }
    
    #[test]
    fn test_collect_with_max_depth() {
        let temp = TempDir::new().unwrap();
        let sub1 = temp.path().join("sub1");
        let sub2 = sub1.join("sub2");
        fs::create_dir_all(&sub2).unwrap();
        
        fs::write(temp.path().join("test0.jpg"), b"test").unwrap();
        fs::write(sub1.join("test1.jpg"), b"test").unwrap();
        fs::write(sub2.join("test2.jpg"), b"test").unwrap();
        
        let collector = FileCollector::new(vec!["jpg".to_string()])
            .recursive(true)
            .max_depth(1);
        let files = collector.collect(temp.path()).unwrap();
        
        // Should only get test0.jpg (depth 0)
        assert_eq!(files.len(), 1);
    }
    
    #[test]
    fn test_extension_filtering() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join("test1.jpg"), b"test").unwrap();
        fs::write(temp.path().join("test2.JPG"), b"test").unwrap();  // Case insensitive
        fs::write(temp.path().join("test3.png"), b"test").unwrap();
        
        let collector = FileCollector::new(vec!["jpg".to_string()]);
        let files = collector.collect(temp.path()).unwrap();
        
        assert_eq!(files.len(), 2);  // Both .jpg and .JPG
    }
}
