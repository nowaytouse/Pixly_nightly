//! 🔍 统一验证框架
//! 
//! 合并 `validation.rs` 和 `conversion_validator.rs` 的功能
//! 提供完整的文件、图像和转换验证功能
//! 
//! ## 验证架构
//! 
//! ### 通用验证 (来自 validation.rs)
//! - **Basic**: 文件存在性和大小
//! - **Format**: 格式检测
//! - **Integrity**: 文件完整性
//! - **Deep**: 完整解码验证
//! - **Dimensions**: 尺寸一致性
//! - **Quality**: 质量评估
//! 
//! ### 转换专用验证 (来自 conversion_validator.rs)
//! - **InputFiles**: 输入文件批量验证
//! - **Parameters**: 转换参数验证
//! - **ModeConsistency**: 模式与参数匹配验证
//! - **Output**: 输出文件验证

use anyhow::Result;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use rayon::prelude::*;

/// 验证级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ValidationLevel {
    /// Level 1: 基础检查 (存在性、大小)
    Basic = 1,
    /// Level 2: 格式检查
    Format = 2,
    /// Level 3: 完整性检查 (文件头/尾)
    Integrity = 3,
    /// Level 4: 深度检查 (完整解码)
    Deep = 4,
    /// Level 5: 尺寸验证 (输入输出一致性)
    Dimensions = 5,
    /// Level 6: 质量验证 (元数据、SSIM)
    Quality = 6,
}

/// 验证结果（统一结构）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidationResult {
    /// 是否通过验证
    pub passed: bool,
    /// 是否有效（兼容旧 API）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
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
    
    // Level 5-6: 输入输出对比验证
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

impl ValidationResult {
    /// 创建成功结果
    pub fn success(level: u8) -> Self {
        Self {
            passed: true,
            valid: Some(true),
            level,
            ..Default::default()
        }
    }
    
    /// 创建失败结果
    pub fn failure(level: u8, errors: Vec<String>) -> Self {
        Self {
            passed: false,
            valid: Some(false),
            level,
            errors,
            ..Default::default()
        }
    }
    
    /// 添加警告
    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings = warnings;
        self
    }
    
    /// 设置格式
    pub fn with_format(mut self, format: String) -> Self {
        self.detected_format = Some(format);
        self
    }
    
    /// 设置尺寸
    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.dimensions = Some((width, height));
        self
    }
}

/// 输入文件信息（转换专用）
#[derive(Debug, Clone)]
pub struct InputFile {
    pub file_path: PathBuf,
    pub name: String,
    pub ext: String,
    pub size: u64,
    pub is_animated: bool,
}

/// 转换配置（转换专用）
#[derive(Debug, Clone)]
pub struct ConversionConfig {
    pub format: String,
    pub quality: Option<u32>,
    pub speed: Option<u32>,
    pub lossless: bool,
}

/// 转换模式（转换专用）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionMode {
    Manual,
    Smart,
    AI,
}

/// 统一验证器
pub struct UnifiedValidator;

impl UnifiedValidator {
    // ========================================
    // 通用文件验证 API (来自 validation.rs)
    // ========================================
    
    /// ✅ Level 1: 基础验证 - 文件存在性和大小
    pub fn validate_basic<P: AsRef<Path>>(path: P) -> Result<ValidationResult> {
        let path = path.as_ref();
        let mut result = ValidationResult {
            level: ValidationLevel::Basic as u8,
            ..Default::default()
        };
        
        // 检查文件是否存在
        if !path.exists() {
            result.errors.push("File does not exist".to_string());
            return Ok(result);
        }
        
        // 获取文件大小
        let metadata = std::fs::metadata(path)?;
        result.file_size = metadata.len();
        
        if result.file_size == 0 {
            result.errors.push("File size is 0".to_string());
            return Ok(result);
        }
        
        result.passed = true;
        result.valid = Some(true);
        Ok(result)
    }
    
    /// ✅ Level 2: 格式验证
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
            result.warnings.push("Unable to detect file format".to_string());
        }
        
        result.passed = result.detected_format.is_some();
        result.valid = Some(result.passed);
        Ok(result)
    }
    
    /// ✅ Level 4: 深度验证 - 完整解码
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
                result.valid = Some(true);
            }
            Err(e) => {
                result.errors.push(format!("Image decoding failed: {}", e));
                result.passed = false;
                result.valid = Some(false);
            }
        }
        
        Ok(result)
    }
    
    /// ✅ Level 5: 尺寸一致性验证 - 输入输出对比
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
            result.valid = Some(result.passed);
        }
        
        Ok(result)
    }
    
    // ========================================
    // 转换专用验证 API (来自 conversion_validator.rs)
    // ========================================
    
    /// 🔍 转换验证 - Level 1: 输入文件批量验证
    pub fn validate_input_files(files: &[InputFile]) -> ValidationResult {
        let mut errors = Vec::with_capacity(files.len());
        let mut warnings = Vec::with_capacity(files.len());
        
        if files.is_empty() {
            errors.push("❌ No files selected".to_string());
            return ValidationResult::failure(1, errors);
        }
        
        for (index, file) in files.iter().enumerate() {
            let file_num = index + 1;
            
            // 验证文件路径
            if file.file_path.as_os_str().is_empty() {
                errors.push(format!("❌ File #{}: Invalid file path", file_num));
            }
            
            // 验证文件扩展名
            if file.ext.is_empty() {
                errors.push(format!("❌ File #{} ({}): Missing file extension", file_num, file.name));
            }
            
            // 验证文件大小
            if file.size == 0 {
                warnings.push(format!("⚠️ File #{} ({}): File size is 0", file_num, file.name));
            } else if file.size > 1024 * 1024 * 1024 { // 1GB
                let size_gb = file.size as f64 / (1024.0 * 1024.0 * 1024.0);
                warnings.push(format!("⚠️ File #{} ({}): File too large ({:.2} GB)", file_num, file.name, size_gb));
            }
            
            // 验证文件名合法性
            if file.name.chars().any(|c| matches!(c, '<' | '>' | ':' | '"' | '|' | '?' | '*')) {
                warnings.push(format!("⚠️ File #{} ({}): Filename contains illegal characters", file_num, file.name));
            }
            
            // 验证文件是否存在
            if !file.file_path.exists() {
                errors.push(format!("❌ File #{} ({}): File does not exist", file_num, file.name));
            }
            
            // 验证文件是否可读
            if file.file_path.exists()
                && let Err(e) = fs::metadata(&file.file_path) {
                    errors.push(format!("❌ File #{} ({}): Cannot read file metadata: {}", file_num, file.name, e));
                }
        }
        
        if errors.is_empty() {
            ValidationResult::success(1).with_warnings(warnings)
        } else {
            ValidationResult::failure(1, errors).with_warnings(warnings)
        }
    }
    
    /// 🚀 转换验证 - Level 1 (并行版本): 输入文件批量验证 - 使用并行处理
    /// 
    /// **性能优化**: 对于大量文件（>10个），使用 Rayon 并行处理可显著提升性能
    /// 
    /// # 示例
    /// ```no_run
    /// use pixly_kernel::{UnifiedValidator, ValidatorInputFile};
    /// use std::path::PathBuf;
    /// 
    /// let files = vec![/* ... */];
    /// let result = UnifiedValidator::validate_input_files_parallel(&files);
    /// ```
    pub fn validate_input_files_parallel(files: &[InputFile]) -> ValidationResult {
        use std::sync::{Arc, Mutex};
        
        if files.is_empty() {
            return ValidationResult::failure(1, vec!["❌ No files selected".to_string()]);
        }
        
        // 使用 Arc<Mutex> 收集错误和警告（线程安全）
        let errors = Arc::new(Mutex::new(Vec::new()));
        let warnings = Arc::new(Mutex::new(Vec::new()));
        
        // 并行验证每个文件
        files.par_iter().enumerate().for_each(|(index, file)| {
            let file_num = index + 1;
            let mut local_errors = Vec::new();
            let mut local_warnings = Vec::new();
            
            // 验证文件路径
            if file.file_path.as_os_str().is_empty() {
                local_errors.push(format!("❌ File #{}: Invalid file path", file_num));
            }
            
            // 验证文件扩展名
            if file.ext.is_empty() {
                local_errors.push(format!("❌ File #{} ({}): Missing file extension", file_num, file.name));
            }
            
            // 验证文件大小
            if file.size == 0 {
                local_warnings.push(format!("⚠️ File #{} ({}): File size is 0", file_num, file.name));
            } else if file.size > 1024 * 1024 * 1024 { // 1GB
                let size_gb = file.size as f64 / (1024.0 * 1024.0 * 1024.0);
                local_warnings.push(format!("⚠️ File #{} ({}): File too large ({:.2} GB)", file_num, file.name, size_gb));
            }
            
            // 验证文件名合法性
            if file.name.chars().any(|c| matches!(c, '<' | '>' | ':' | '"' | '|' | '?' | '*')) {
                local_warnings.push(format!("⚠️ File #{} ({}): Filename contains illegal characters", file_num, file.name));
            }
            
            // 验证文件是否存在
            if !file.file_path.exists() {
                local_errors.push(format!("❌ File #{} ({}): File does not exist", file_num, file.name));
            }
            
            // 验证文件是否可读
            if file.file_path.exists() && let Err(e) = fs::metadata(&file.file_path) {
                local_errors.push(format!("❌ File #{} ({}): Cannot read file metadata: {}", file_num, file.name, e));
            }
            
            // 合并到全局结果
            if !local_errors.is_empty() && let Ok(mut errs) = errors.lock() {
                errs.extend(local_errors);
            }
            if !local_warnings.is_empty() && let Ok(mut warns) = warnings.lock() {
                warns.extend(local_warnings);
            }
        });
        
        // 提取最终结果
        let final_errors = match Arc::try_unwrap(errors) {
            Ok(mutex) => mutex.into_inner().unwrap_or_default(),
            Err(arc) => arc.lock().unwrap().clone(),
        };
        let final_warnings = match Arc::try_unwrap(warnings) {
            Ok(mutex) => mutex.into_inner().unwrap_or_default(),
            Err(arc) => arc.lock().unwrap().clone(),
        };
        
        if final_errors.is_empty() {
            ValidationResult::success(1).with_warnings(final_warnings)
        } else {
            ValidationResult::failure(1, final_errors).with_warnings(final_warnings)
        }
    }
    
    /// 🔍 转换验证 - Level 2: 转换参数验证
    pub fn validate_parameters(config: &ConversionConfig, files: &[InputFile]) -> ValidationResult {
        let mut errors = Vec::with_capacity(4);
        let mut warnings = Vec::with_capacity(4);
        
        // 验证目标格式
        let valid_formats = ["jxl", "avif", "webp", "heic", "png", "jpg", "jpeg"];
        let format_lower = config.format.to_lowercase();
        if !valid_formats.contains(&format_lower.as_str()) {
            errors.push(format!("❌ Invalid target format: {}", config.format));
        }
        
        // 验证质量参数
        if let Some(quality) = config.quality
            && (!(1..=100).contains(&quality)) {
                errors.push(format!("❌ Invalid quality parameter: {} (should be 1-100)", quality));
            }
        
        // 验证速度参数
        if let Some(speed) = config.speed
            && speed > 10 {
                errors.push(format!("❌ Invalid speed parameter: {} (should be 0-10)", speed));
            }
        
        // 验证格式兼容性
        if format_lower == "heic" {
            let has_transparency = files.iter().any(|f| {
                let ext = f.ext.to_lowercase();
                ext == ".png" || ext == ".gif" || ext == ".webp"
            });
            if has_transparency {
                warnings.push("⚠️ HEIC does not support transparency, transparent backgrounds may change color".to_string());
            }
            
            let has_animation = files.iter().any(|f| f.is_animated);
            if has_animation {
                warnings.push("⚠️ HEIC does not support animation, animation effects will be lost".to_string());
            }
        }
        
        // 验证无损模式兼容性
        if config.lossless
            && (format_lower == "jpg" || format_lower == "jpeg") {
                errors.push("❌ JPEG format does not support lossless mode".to_string());
            }
        
        // 验证 AVIF 动画支持
        if format_lower == "avif" {
            let has_animation = files.iter().any(|f| f.is_animated);
            if has_animation {
                warnings.push("⚠️ AVIF animation support is limited, may require special encoder".to_string());
            }
        }
        
        // 验证 JXL 参数
        if (format_lower == "jxl" || format_lower == "jpegxl")
            && let Some(quality) = config.quality
                && quality < 60 {
                    warnings.push(format!("⚠️ JXL quality parameter is low ({}), may affect visual quality", quality));
                }
        
        if errors.is_empty() {
            ValidationResult::success(2).with_warnings(warnings)
        } else {
            ValidationResult::failure(2, errors).with_warnings(warnings)
        }
    }
    
    /// 🔍 转换验证 - Level 3: 模式与参数匹配验证
    pub fn validate_mode_consistency(mode: ConversionMode, config: &ConversionConfig) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        match mode {
            ConversionMode::Manual => {
                if config.quality.is_none() {
                    warnings.push("⚠️ Quality parameter not set in manual mode, will use default value".to_string());
                }
                if config.format.is_empty() {
                    errors.push("❌ Target format must be specified in manual mode".to_string());
                }
            }
            ConversionMode::Smart | ConversionMode::AI => {
                warnings.push("ℹ️ Smart mode will use AI predicted parameters".to_string());
            }
        }
        
        if errors.is_empty() {
            ValidationResult::success(3).with_warnings(warnings)
        } else {
            ValidationResult::failure(3, errors).with_warnings(warnings)
        }
    }
    
    /// 🔍 转换验证 - Level 4: 输出文件验证
    pub fn validate_output(output_path: &Path, expected_size: Option<u64>) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // 验证文件是否存在
        if !output_path.exists() {
            errors.push(format!("❌ Output file not generated: {}", output_path.display()));
            return ValidationResult::failure(4, errors);
        }
        
        // 验证文件大小
        match fs::metadata(output_path) {
            Ok(metadata) => {
                let actual_size = metadata.len();
                
                if actual_size == 0 {
                    errors.push(format!("❌ Output file size is 0: {}", output_path.display()));
                }
                
                if let Some(expected) = expected_size
                    && expected > 0 {
                        let ratio = actual_size as f64 / expected as f64;
                        if ratio < 0.01 {
                            warnings.push(format!("⚠️ Output file abnormally small ({:.2}% of original)", ratio * 100.0));
                        } else if ratio > 10.0 {
                            warnings.push(format!("⚠️ Output file abnormally large ({:.2}x of original)", ratio));
                        }
                    }
            }
            Err(e) => {
                errors.push(format!("❌ Output validation failed: {}", e));
            }
        }
        
        if errors.is_empty() {
            ValidationResult::success(4).with_warnings(warnings)
        } else {
            ValidationResult::failure(4, errors).with_warnings(warnings)
        }
    }
    
    /// 🔍 完整转换验证流程（组合所有级别）
    pub fn validate_full_conversion(
        files: &[InputFile],
        config: &ConversionConfig,
        mode: ConversionMode,
    ) -> ValidationResult {
        log::info!("🔍 Starting multi-level validation");
        
        // Level 1: 输入验证
        let input_validation = Self::validate_input_files(files);
        if !input_validation.passed {
            log::error!("❌ Level 1 failed: Input file validation failed");
            return input_validation;
        }
        log::info!("✅ Level 1 passed: Input file validation");
        
        // Level 2: 参数验证
        let param_validation = Self::validate_parameters(config, files);
        if !param_validation.passed {
            log::error!("❌ Level 2 failed: Parameter validation failed");
            let mut result = param_validation;
            result.warnings.extend(input_validation.warnings);
            return result;
        }
        log::info!("✅ Level 2 passed: Parameter validation");
        
        // Level 3: 模式一致性验证
        let mode_validation = Self::validate_mode_consistency(mode, config);
        if !mode_validation.passed {
            log::error!("❌ Level 3 failed: Mode consistency validation failed");
            let mut result = mode_validation;
            result.warnings.extend(input_validation.warnings);
            result.warnings.extend(param_validation.warnings);
            return result;
        }
        log::info!("✅ Level 3 passed: Mode consistency validation");
        
        // 收集所有警告
        let mut all_warnings = input_validation.warnings;
        all_warnings.extend(param_validation.warnings);
        all_warnings.extend(mode_validation.warnings);
        
        log::info!("🎉 All validation levels passed");
        if !all_warnings.is_empty() {
            log::warn!("⚠️ Warnings: {:?}", all_warnings);
        }
        
        ValidationResult::success(3).with_warnings(all_warnings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;
    
    // ========================================
    // 通用验证测试
    // ========================================
    
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
        let result = UnifiedValidator::validate_basic("/nonexistent/file.jpg");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.passed);
        assert!(!result.errors.is_empty());
    }
    
    // ========================================
    // 转换专用验证测试
    // ========================================
    
    #[test]
    fn test_validate_empty_files() {
        let files = vec![];
        let result = UnifiedValidator::validate_input_files(&files);
        assert!(!result.passed);
        assert_eq!(result.level, 1);
        assert!(!result.errors.is_empty());
    }
    
    #[test]
    fn test_validate_valid_files() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.png");
        File::create(&file_path).unwrap();
        
        let files = vec![InputFile {
            file_path: file_path.clone(),
            name: "test.png".to_string(),
            ext: ".png".to_string(),
            size: 1024,
            is_animated: false,
        }];
        
        let result = UnifiedValidator::validate_input_files(&files);
        assert!(result.passed);
        assert_eq!(result.level, 1);
    }
    
    #[test]
    fn test_validate_invalid_format() {
        let config = ConversionConfig {
            format: "invalid".to_string(),
            quality: Some(80),
            speed: Some(4),
            lossless: false,
        };
        
        let result = UnifiedValidator::validate_parameters(&config, &[]);
        assert!(!result.passed);
        assert_eq!(result.level, 2);
        assert!(result.errors.iter().any(|e| e.contains("Invalid target format")));
    }
    
    #[test]
    fn test_validate_quality_range() {
        let config = ConversionConfig {
            format: "webp".to_string(),
            quality: Some(150),
            speed: Some(4),
            lossless: false,
        };
        
        let result = UnifiedValidator::validate_parameters(&config, &[]);
        assert!(!result.passed);
        assert!(result.errors.iter().any(|e| e.contains("Invalid quality parameter")));
    }
    
    #[test]
    fn test_validate_jpeg_lossless() {
        let config = ConversionConfig {
            format: "jpeg".to_string(),
            quality: Some(90),
            speed: Some(4),
            lossless: true,
        };
        
        let result = UnifiedValidator::validate_parameters(&config, &[]);
        assert!(!result.passed);
        assert!(result.errors.iter().any(|e| e.contains("JPEG format does not support lossless mode")));
    }
    
    #[test]
    fn test_validate_output_missing_file() {
        let result = UnifiedValidator::validate_output(Path::new("/nonexistent/file.webp"), None);
        assert!(!result.passed);
        assert_eq!(result.level, 4);
        assert!(result.errors.iter().any(|e| e.contains("Output file not generated")));
    }
    
    #[test]
    fn test_full_validation_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.png");
        File::create(&file_path).unwrap();
        
        let files = vec![InputFile {
            file_path,
            name: "test.png".to_string(),
            ext: ".png".to_string(),
            size: 1024,
            is_animated: false,
        }];
        
        let config = ConversionConfig {
            format: "webp".to_string(),
            quality: Some(80),
            speed: Some(4),
            lossless: false,
        };
        
        let result = UnifiedValidator::validate_full_conversion(&files, &config, ConversionMode::Manual);
        assert!(result.passed);
        assert_eq!(result.level, 3);
    }
    
    // ========================================
    // 并行验证测试
    // ========================================
    
    #[test]
    fn test_parallel_validation_empty() {
        let files = vec![];
        let result = UnifiedValidator::validate_input_files_parallel(&files);
        assert!(!result.passed);
        assert_eq!(result.level, 1);
    }
    
    #[test]
    fn test_parallel_validation_valid() {
        let temp_dir = TempDir::new().unwrap();
        
        // 创建多个测试文件
        let file_paths: Vec<_> = (0..5)
            .map(|i| {
                let path = temp_dir.path().join(format!("test_{}.png", i));
                File::create(&path).unwrap();
                path
            })
            .collect();
        
        let files: Vec<_> = file_paths.into_iter().enumerate()
            .map(|(i, path)| InputFile {
                file_path: path,
                name: format!("test_{}.png", i),
                ext: ".png".to_string(),
                size: 1024,
                is_animated: false,
            })
            .collect();
        
        let result = UnifiedValidator::validate_input_files_parallel(&files);
        assert!(result.passed);
        assert_eq!(result.level, 1);
    }
    
    #[test]
    fn test_parallel_vs_sequential_consistency() {
        let temp_dir = TempDir::new().unwrap();
        
        // 创建测试文件
        let file_paths: Vec<_> = (0..10)
            .map(|i| {
                let path = temp_dir.path().join(format!("test_{}.png", i));
                File::create(&path).unwrap();
                path
            })
            .collect();
        
        let files: Vec<_> = file_paths.into_iter().enumerate()
            .map(|(i, path)| InputFile {
                file_path: path,
                name: format!("test_{}.png", i),
                ext: ".png".to_string(),
                size: 1024 * (i as u64 + 1),
                is_animated: false,
            })
            .collect();
        
        // 串行验证
        let sequential_result = UnifiedValidator::validate_input_files(&files);
        
        // 并行验证
        let parallel_result = UnifiedValidator::validate_input_files_parallel(&files);
        
        // 验证结果一致性
        assert_eq!(sequential_result.passed, parallel_result.passed);
        assert_eq!(sequential_result.level, parallel_result.level);
        assert_eq!(sequential_result.errors.len(), parallel_result.errors.len());
        // 警告数量应该相同（虽然顺序可能不同）
        assert_eq!(sequential_result.warnings.len(), parallel_result.warnings.len());
    }
}
