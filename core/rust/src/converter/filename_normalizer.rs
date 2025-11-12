// 🔥 Phase 40.8: 文件名规范化模块
// 作用：中间处理时使用规范化文件名，完成后还原原始文件名
// 架构原则：
// - 职责：文件名清理、临时名称生成、原名恢复
// - 依赖：Rust文件系统API
// - 安全性：处理特殊字符、长度限制、跨平台兼容

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::{Result, Context, bail};
use serde::{Serialize, Deserialize};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    /// 危险字符正则（需要替换）
    static ref DANGEROUS_CHARS: Regex = Regex::new(r#"[<>:"/\\|?*\x00-\x1F]"#).unwrap();
    
    /// 空白字符正则
    static ref WHITESPACE: Regex = Regex::new(r"\s+").unwrap();
    
    /// 多个连续下划线
    static ref MULTIPLE_UNDERSCORES: Regex = Regex::new(r"_{2,}").unwrap();
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
}

impl FilenameNormalizer {
    /// 创建新的规范化器
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
            max_length: 200, // 大多数文件系统的安全长度
            preserve_extension: true,
        }
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
    /// 5. 转换为小写（可选）
    pub fn normalize(&mut self, path: &Path) -> Result<PathBuf> {
        let original_name = path.file_name()
            .context("Invalid file path")?
            .to_string_lossy()
            .to_string();
        
        // 分离文件名和扩展名
        let (base_name, extension) = if self.preserve_extension {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_string();
                let base = path.file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                (base, Some(ext_str))
            } else {
                (original_name.clone(), None)
            }
        } else {
            (original_name.clone(), None)
        };
        
        // 规范化基础名称
        let mut normalized = base_name.clone();
        
        // 1. 替换危险字符
        normalized = DANGEROUS_CHARS.replace_all(&normalized, "_").to_string();
        
        // 2. 替换空白
        normalized = WHITESPACE.replace_all(&normalized, "_").to_string();
        
        // 3. 移除多余的下划线
        normalized = MULTIPLE_UNDERSCORES.replace_all(&normalized, "_").to_string();
        
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
        let temp_name = temp_path.file_name()
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
        
        bail!("No mapping found for temp file: {}", temp_name);
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
        let path = Path::new("test file.png");
        let result = normalizer.normalize(path).unwrap();
        assert_eq!(result.file_name().unwrap().to_str().unwrap(), "test_file.png");
    }
    
    #[test]
    fn test_normalize_dangerous_chars() {
        let mut normalizer = FilenameNormalizer::new();
        let path = Path::new("file<with>bad:chars.jpg");
        let result = normalizer.normalize(path).unwrap();
        let normalized = result.file_name().unwrap().to_str().unwrap();
        assert!(!normalized.contains('<'));
        assert!(!normalized.contains('>'));
        assert!(!normalized.contains(':'));
    }
    
    #[test]
    fn test_restore() {
        let mut normalizer = FilenameNormalizer::new();
        let original = Path::new("my file.png");
        let temp = normalizer.normalize(original).unwrap();
        let restored = normalizer.restore(&temp).unwrap();
        assert_eq!(restored.file_name().unwrap().to_str().unwrap(), "my file.png");
    }
}
