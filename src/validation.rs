//! 🔍 验证系统
//!
//! 提供多级别的文件和图像验证功能
//!
//! ## 验证级别
//!
//! 1. **Basic** - 基础检查(文件存在性)
//! 2. **Format** - 格式检查
//! 3. **Integrity** - 完整性检查(文件头/尾)
//! 4. **Deep** - 深度检查(完整解码)
//! 5. **Security** - 安全检查
//! 6. **AntiCheat** - 防作弊检查
//! 7. **Dimensions** - 尺寸验证(输入输出一致性)
//! 8. **Quality** - 质量验证(元数据、SSIM)

use anyhow::Result;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// 验证级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ValidationLevel {
    /// Level 1: 基础检查 (存在性)
    Basic = 1,
    /// Level 2: 格式检查
    Format = 2,
    /// Level 3: 完整性检查 (文件头/尾)
    Integrity = 3,
    /// Level 4: 深度检查 (完整解码)
    Deep = 4,
    /// Level 5: 安全检查
    Security = 5,
    /// Level 6: 防作弊检查
    AntiCheat = 6,
    /// Level 7: 尺寸验证 (输入输出一致性)
    Dimensions = 7,
    /// Level 8: 质量验证 (元数据、SSIM)
    Quality = 8,
}

/// 验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ValidationResult {
    /// 是否通过验证
    pub passed: bool,
    /// 验证级别
    pub level: u8,
    /// 检测到的格式
    pub detected_format: Option<String>,
    /// 文件大小
    pub file_size: u64,
    /// 图像尺寸 (宽度, 高度)
    pub dimensions: Option<(u32, u32)>,
    /// 是否动画
    pub is_animated: Option<bool>,

    // Level 7 & 8: 输入输出对比验证
    /// 输入文件尺寸
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_dimensions: Option<(u32, u32)>,
    /// 输出文件尺寸
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_dimensions: Option<(u32, u32)>,
    /// 尺寸是否匹配
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions_match: Option<bool>,
    /// 元数据是否保留
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_preserved: Option<bool>,
    /// 质量分数 (SSIM, 0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_score: Option<f64>,
    /// 质量是否可接受
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_acceptable: Option<bool>,

    /// 错误信息
    pub errors: Vec<String>,
    /// 警告信息
    pub warnings: Vec<String>,
}


/// 文件验证器
pub struct FileValidator;

impl FileValidator {
    /// 验证文件 - 基础级别
    pub fn validate_basic<P: AsRef<Path>>(path: P) -> Result<ValidationResult> {
        let path = path.as_ref();
        let mut result = ValidationResult {
            level: ValidationLevel::Basic as u8,
            ..Default::default()
        };

        // 检查文件是否存在
        if !path.exists() {
            result.errors.push("文件不存在".to_string());
            return Ok(result);
        }

        // 获取文件大小
        let metadata = std::fs::metadata(path)?;
        result.file_size = metadata.len();

        if result.file_size == 0 {
            result.errors.push("文件大小为0".to_string());
            return Ok(result);
        }

        result.passed = true;
        Ok(result)
    }

    /// 验证文件 - 格式级别
    pub fn validate_format<P: AsRef<Path>>(path: P) -> Result<ValidationResult> {
        let path = path.as_ref();
        let mut result = Self::validate_basic(path)?;

        if !result.passed {
            return Ok(result);
        }

        result.level = ValidationLevel::Format as u8;

        // 检测格式
        if let Some(ext) = path.extension() {
            result.detected_format = Some(ext.to_string_lossy().to_lowercase());
        } else {
            result.warnings.push("无法检测文件格式".to_string());
        }

        result.passed = result.detected_format.is_some();
        Ok(result)
    }

    /// 验证文件 - 深度级别(完整解码)
    pub fn validate_deep<P: AsRef<Path>>(path: P) -> Result<ValidationResult> {
        let path = path.as_ref();
        let mut result = Self::validate_format(path)?;

        if !result.passed {
            return Ok(result);
        }

        result.level = ValidationLevel::Deep as u8;

        // 尝试完整解码
        match image::open(path) {
            Ok(img) => {
                let (width, height) = img.dimensions();
                result.dimensions = Some((width, height));
                result.passed = true;
            }
            Err(e) => {
                result.errors.push(format!("Image decoding failed: {}", e));
                result.passed = false;
            }
        }

        Ok(result)
    }

    /// 验证输入输出一致性
    pub fn validate_consistency<P: AsRef<Path>>(
        input_path: P,
        output_path: P,
    ) -> Result<ValidationResult> {
        let input_path = input_path.as_ref();
        let output_path = output_path.as_ref();

        let mut result = Self::validate_deep(output_path)?;

        if !result.passed {
            return Ok(result);
        }

        result.level = ValidationLevel::Dimensions as u8;

        // 获取输入尺寸
        if let Ok(input_img) = image::open(input_path) {
            let (input_w, input_h) = input_img.dimensions();
            result.input_dimensions = Some((input_w, input_h));
        }

        // 检查尺寸是否匹配
        if let (Some(input_dims), Some(output_dims)) =
            (result.input_dimensions, result.dimensions)
        {
            result.dimensions_match = Some(input_dims == output_dims);
            result.passed = result.dimensions_match.unwrap_or(false);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_level_ordering() {
        assert!(ValidationLevel::Basic < ValidationLevel::Format);
        assert!(ValidationLevel::Format < ValidationLevel::Deep);
        assert!(ValidationLevel::Deep < ValidationLevel::Quality);
    }

    #[test]
    fn test_validation_result_default() {
        let result = ValidationResult::default();
        assert!(!result.passed);
        assert_eq!(result.level, 0);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_nonexistent_file() {
        let result = FileValidator::validate_basic("/nonexistent/file.jpg");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.passed);
        assert!(!result.errors.is_empty());
    }
}
