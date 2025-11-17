//! 📝 文件名规范化器
//!
//! 中间处理时使用规范化文件名，完成后还原原始文件名
//!
//! ## 核心功能
//!
//! - **文件名清理** - 移除危险字符和特殊字符
//! - **临时名称生成** - 生成安全的临时文件名
//! - **原名恢复** - 处理完成后还原原始文件名
//! - **跨平台兼容** - 处理不同操作系统的文件名限制
//! - **长度限制** - 自动截断过长的文件名
//! - **唯一性保证** - 使用哈希确保唯一性
//!
//! ## 使用场景
//!
//! - 批量处理文件时避免文件名冲突
//! - 处理包含特殊字符的文件名
//! - 跨平台文件传输
//! - 临时文件管理

use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// 危险字符正则（需要替换）
fn dangerous_chars() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r#"[<>:"/\\|?*\x00-\x1F]"#).unwrap())
}

/// 空白字符正则
fn whitespace() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"\s+").unwrap())
}

/// 多个连续下划线
fn multiple_underscores() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"_{2,}").unwrap())
}

/// 文件名映射记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilenameMapping {
    /// 原始文件名
    pub original: String,
    /// 规范化后的文件名
    pub normalized: String,
    /// 原始路径
    pub original_path: PathBuf,
    /// 临时路径
    pub temp_path: PathBuf,
}

/// 文件名规范化器
pub struct FilenameNormalizer {
    /// 文件名映射表（原始 -> 规范化）
    mappings: HashMap<String, FilenameMapping>,
    /// 最大文件名长度
    max_length: usize,
    /// 是否保留扩展名
    preserve_extension: bool,
    /// 是否使用Magika自动检测扩展名
    use_magika: bool,
}

impl FilenameNormalizer {
    /// 创建新的规范化器
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
            max_length: 200, // 大多数文件系统的安全长度
            preserve_extension: true,
            use_magika: true, // 默认启用Magika自动检测
        }
    }
    
    /// 启用/禁用Magika自动检测
    pub fn set_use_magika(&mut self, enabled: bool) {
        self.use_magika = enabled;
    }

    /// 设置最大文件名长度
    pub fn set_max_length(&mut self, length: usize) {
        self.max_length = length;
    }

    /// 规范化文件名（中间处理用）
    ///
    /// # 规则
    /// 1. 替换危险字符为下划线
    /// 2. 替换空白为下划线
    /// 3. 移除多余的下划线
    /// 4. 限制长度
    /// 5. 使用哈希保证唯一性
    pub fn normalize(&mut self, path: &Path) -> Result<PathBuf> {
        let original_name = path
            .file_name()
            .context("Invalid file path")?
            .to_string_lossy()
            .to_string();

        // 分离文件名和扩展名，并使用Magika验证
        let (base_name, extension) = if self.preserve_extension {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_string();
                let base = path.file_stem().unwrap().to_string_lossy().to_string();
                
                // 使用Magika验证扩展名是否正确
                let corrected_ext = if self.use_magika {
                    self.detect_and_correct_extension(path, &ext_str)
                } else {
                    ext_str
                };
                
                (base, Some(corrected_ext))
            } else {
                // 没有扩展名，尝试用Magika检测
                let detected_ext = if self.use_magika {
                    self.detect_extension(path)
                } else {
                    None
                };
                
                (original_name.clone(), detected_ext)
            }
        } else {
            (original_name.clone(), None)
        };

        // 规范化基础名称
        let mut normalized = base_name.clone();

        // 1. 替换危险字符
        normalized = dangerous_chars().replace_all(&normalized, "_").to_string();

        // 2. 替换空白
        normalized = whitespace().replace_all(&normalized, "_").to_string();

        // 3. 移除多余的下划线
        normalized = multiple_underscores()
            .replace_all(&normalized, "_")
            .to_string();

        // 4. 移除首尾下划线
        normalized = normalized.trim_matches('_').to_string();

        // 5. 限制长度（保留扩展名空间）
        let extension_length = extension.as_ref().map(|e| e.len() + 1).unwrap_or(0);
        let max_base_length = self.max_length.saturating_sub(extension_length);

        if normalized.len() > max_base_length {
            // 截断并添加哈希以保证唯一性
            let hash = format!("{:x}", md5::compute(&normalized));
            let hash_suffix = &hash[..8]; // 取前8位
            let truncate_len = max_base_length.saturating_sub(9); // 8位哈希 + 1个下划线
            normalized = format!("{}_{}", &normalized[..truncate_len], hash_suffix);
        }

        // 6. 重新添加扩展名
        let final_name = if let Some(ext) = extension {
            format!("{}.{}", normalized, ext)
        } else {
            normalized
        };

        // 构建临时路径
        let parent = path.parent().unwrap_or_else(|| Path::new(""));
        let temp_path = parent.join(&final_name);

        // 保存映射
        let mapping = FilenameMapping {
            original: original_name.clone(),
            normalized: final_name,
            original_path: path.to_path_buf(),
            temp_path: temp_path.clone(),
        };

        self.mappings.insert(original_name, mapping);

        Ok(temp_path)
    }

    /// 还原原始文件名
    ///
    /// # 用途
    /// 处理完成后，将临时文件名还原为原始名称
    pub fn restore(&self, temp_path: &Path) -> Result<PathBuf> {
        let temp_name = temp_path
            .file_name()
            .context("Invalid temp path")?
            .to_string_lossy()
            .to_string();

        // 查找映射
        for mapping in self.mappings.values() {
            if mapping.normalized == temp_name {
                let parent = temp_path.parent().unwrap_or_else(|| Path::new(""));
                return Ok(parent.join(&mapping.original));
            }
        }

        anyhow::bail!("No mapping found for temp file: {}", temp_name)
    }

    /// 获取原始路径
    pub fn get_original_path(&self, normalized_name: &str) -> Option<&PathBuf> {
        for mapping in self.mappings.values() {
            if mapping.normalized == normalized_name {
                return Some(&mapping.original_path);
            }
        }
        None
    }

    /// 清除所有映射
    pub fn clear(&mut self) {
        self.mappings.clear();
    }

    /// 导出映射表（用于持久化）
    pub fn export_mappings(&self) -> Vec<FilenameMapping> {
        self.mappings.values().cloned().collect()
    }

    /// 导入映射表
    pub fn import_mappings(&mut self, mappings: Vec<FilenameMapping>) {
        for mapping in mappings {
            self.mappings.insert(mapping.original.clone(), mapping);
        }
    }

    /// 获取映射数量
    pub fn mapping_count(&self) -> usize {
        self.mappings.len()
    }

    /// 检查是否有映射
    pub fn has_mapping(&self, original_name: &str) -> bool {
        self.mappings.contains_key(original_name)
    }
    
    /// 使用Magika检测并修正扩展名
    fn detect_and_correct_extension(&self, path: &Path, current_ext: &str) -> String {
        use crate::magika_detector::MagikaDetector;
        
        let detector = MagikaDetector::with_defaults();
        match detector.detect_file_type(path) {
            Ok(detection) => {
                // 获取Magika检测到的扩展名
                let detected_ext = &detection.detected_type;
                
                // 如果检测到的扩展名与当前不同，使用检测到的
                if detected_ext != &current_ext.to_lowercase() {
                    println!("📝 Extension corrected: .{} → .{} (confidence: {:.2}%)", 
                             current_ext, detected_ext, detection.confidence * 100.0);
                    detected_ext.clone()
                } else {
                    current_ext.to_string()
                }
            }
            Err(_) => {
                // Magika检测失败，保持原扩展名
                current_ext.to_string()
            }
        }
    }
    
    /// 使用Magika检测扩展名（无扩展名文件）
    fn detect_extension(&self, path: &Path) -> Option<String> {
        use crate::magika_detector::MagikaDetector;
        
        let detector = MagikaDetector::with_defaults();
        match detector.detect_file_type(path) {
            Ok(detection) => {
                let ext = detection.detected_type.clone();
                println!("📝 Extension detected: .{} (confidence: {:.2}%)", 
                         ext, detection.confidence * 100.0);
                Some(ext)
            }
            Err(_) => None
        }
    }
}

impl Default for FilenameNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_simple() {
        let mut normalizer = FilenameNormalizer::new();
        let path = Path::new("test file.jpg");
        let result = normalizer.normalize(path).unwrap();
        assert_eq!(result.file_name().unwrap().to_str().unwrap(), "test_file.jpg");
    }

    #[test]
    fn test_normalize_dangerous_chars() {
        let mut normalizer = FilenameNormalizer::new();
        let path = Path::new("test<>:file?.jpg");
        let result = normalizer.normalize(path).unwrap();
        let filename = result.file_name().unwrap().to_str().unwrap();
        assert!(!filename.contains('<'));
        assert!(!filename.contains('>'));
        assert!(!filename.contains(':'));
        assert!(!filename.contains('?'));
    }

    #[test]
    fn test_normalize_multiple_spaces() {
        let mut normalizer = FilenameNormalizer::new();
        let path = Path::new("test   multiple   spaces.jpg");
        let result = normalizer.normalize(path).unwrap();
        assert_eq!(
            result.file_name().unwrap().to_str().unwrap(),
            "test_multiple_spaces.jpg"
        );
    }

    #[test]
    fn test_normalize_long_filename() {
        let mut normalizer = FilenameNormalizer::new();
        normalizer.set_max_length(50);
        let long_name = "a".repeat(100);
        let path = PathBuf::from(format!("{}.jpg", long_name));
        let result = normalizer.normalize(&path).unwrap();
        let filename = result.file_name().unwrap().to_str().unwrap();
        assert!(filename.len() <= 50);
        assert!(filename.ends_with(".jpg"));
    }

    #[test]
    fn test_restore() {
        let mut normalizer = FilenameNormalizer::new();
        let original = Path::new("test file.jpg");
        let normalized = normalizer.normalize(original).unwrap();
        let restored = normalizer.restore(&normalized).unwrap();
        assert_eq!(
            restored.file_name().unwrap().to_str().unwrap(),
            "test file.jpg"
        );
    }

    #[test]
    fn test_export_import_mappings() {
        let mut normalizer1 = FilenameNormalizer::new();
        normalizer1.normalize(Path::new("test1.jpg")).unwrap();
        normalizer1.normalize(Path::new("test2.jpg")).unwrap();

        let mappings = normalizer1.export_mappings();
        assert_eq!(mappings.len(), 2);

        let mut normalizer2 = FilenameNormalizer::new();
        normalizer2.import_mappings(mappings);
        assert_eq!(normalizer2.mapping_count(), 2);
    }

    #[test]
    fn test_clear_mappings() {
        let mut normalizer = FilenameNormalizer::new();
        normalizer.normalize(Path::new("test.jpg")).unwrap();
        assert_eq!(normalizer.mapping_count(), 1);
        normalizer.clear();
        assert_eq!(normalizer.mapping_count(), 0);
    }
}
