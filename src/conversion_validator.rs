/**
 * 🔥 多级转换验证系统 (Multi-Level Conversion Validator)
 * 
 * 从 plugin/converter/js/plugin-modules/conversion-validator.js 移植
 * Phase 40.33: 增强转换可靠性
 * 
 * 验证层级:
 * - Level 1: 输入文件验证 (文件格式、大小、路径、权限)
 * - Level 2: 转换参数验证 (质量、速度、格式匹配)
 * - Level 3: 模式一致性验证 (手动/智能模式参数匹配)
 * - Level 4: 输出验证 (文件生成、大小、格式正确性)
 */

use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};

/// 验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// 是否通过验证
    pub valid: bool,
    /// 验证级别 (1-4)
    pub level: u8,
    /// 错误列表
    pub errors: Vec<String>,
    /// 警告列表
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn success(level: u8) -> Self {
        Self {
            valid: true,
            level,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn failure(level: u8, errors: Vec<String>) -> Self {
        Self {
            valid: false,
            level,
            errors,
            warnings: Vec::new(),
        }
    }
    
    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings = warnings;
        self
    }
}

/// 输入文件信息
#[derive(Debug, Clone)]
pub struct InputFile {
    pub file_path: PathBuf,
    pub name: String,
    pub ext: String,
    pub size: u64,
    pub is_animated: bool,
}

/// 转换配置
#[derive(Debug, Clone)]
pub struct ConversionConfig {
    pub format: String,
    pub quality: Option<u32>,
    pub speed: Option<u32>,
    pub lossless: bool,
}

/// 转换模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionMode {
    Manual,
    Smart,
    AI,
}

/// 多级验证器
pub struct ConversionValidator;

impl ConversionValidator {
    /// 🔍 Level 1: 输入文件验证
    pub fn validate_input_files(files: &[InputFile]) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
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
            if file.file_path.exists() {
                if let Err(e) = fs::metadata(&file.file_path) {
                    errors.push(format!("❌ File #{} ({}): Cannot read file metadata: {}", file_num, file.name, e));
                }
            }
        }
        
        if errors.is_empty() {
            ValidationResult::success(1).with_warnings(warnings)
        } else {
            ValidationResult::failure(1, errors).with_warnings(warnings)
        }
    }
    
    /// 🔍 Level 2: 转换参数验证
    pub fn validate_parameters(config: &ConversionConfig, files: &[InputFile]) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // 验证目标格式
        let valid_formats = ["jxl", "avif", "webp", "heic", "png", "jpg", "jpeg"];
        let format_lower = config.format.to_lowercase();
        if !valid_formats.contains(&format_lower.as_str()) {
            errors.push(format!("❌ Invalid target format: {}", config.format));
        }
        
        // 验证质量参数
        if let Some(quality) = config.quality {
            if quality < 1 || quality > 100 {
                errors.push(format!("❌ Invalid quality parameter: {} (should be 1-100)", quality));
            }
        }
        
        // 验证速度参数
        if let Some(speed) = config.speed {
            if speed > 10 {
                errors.push(format!("❌ Invalid speed parameter: {} (should be 0-10)", speed));
            }
        }
        
        // 验证格式兼容性
        if format_lower == "heic" {
            // 检测透明度
            let has_transparency = files.iter().any(|f| {
                let ext = f.ext.to_lowercase();
                ext == ".png" || ext == ".gif" || ext == ".webp"
            });
            if has_transparency {
                warnings.push("⚠️ HEIC does not support transparency, transparent backgrounds may change color".to_string());
            }
            
            // 检测动画
            let has_animation = files.iter().any(|f| f.is_animated);
            if has_animation {
                warnings.push("⚠️ HEIC does not support animation, animation effects will be lost".to_string());
            }
        }
        
        // 验证无损模式兼容性
        if config.lossless {
            if format_lower == "jpg" || format_lower == "jpeg" {
                errors.push("❌ JPEG format does not support lossless mode".to_string());
            }
        }
        
        // 验证 AVIF 动画支持
        if format_lower == "avif" {
            let has_animation = files.iter().any(|f| f.is_animated);
            if has_animation {
                warnings.push("⚠️ AVIF animation support is limited, may require special encoder".to_string());
            }
        }
        
        // 验证 JXL 参数
        if format_lower == "jxl" || format_lower == "jpegxl" {
            if let Some(quality) = config.quality {
                if quality < 60 {
                    warnings.push(format!("⚠️ JXL quality parameter is low ({}), may affect visual quality", quality));
                }
            }
        }
        
        if errors.is_empty() {
            ValidationResult::success(2).with_warnings(warnings)
        } else {
            ValidationResult::failure(2, errors).with_warnings(warnings)
        }
    }
    
    /// 🔍 Level 3: 模式与参数匹配验证
    pub fn validate_mode_consistency(mode: ConversionMode, config: &ConversionConfig) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        match mode {
            ConversionMode::Manual => {
                // 手动模式：验证所有必需参数已设置
                if config.quality.is_none() {
                    warnings.push("⚠️ Quality parameter not set in manual mode, will use default value".to_string());
                }
                if config.format.is_empty() {
                    errors.push("❌ Target format must be specified in manual mode".to_string());
                }
            }
            ConversionMode::Smart | ConversionMode::AI => {
                // 智能模式：验证 AI 预测必要性
                // 注意：这里可以添加 AI 服务可用性检查
                warnings.push("⚠️ Smart mode will use AI predicted parameters".to_string());
            }
        }
        
        if errors.is_empty() {
            ValidationResult::success(3).with_warnings(warnings)
        } else {
            ValidationResult::failure(3, errors).with_warnings(warnings)
        }
    }
    
    /// 🔍 Level 4: 输出验证（转换后）
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
                
                // 验证文件大小不为0
                if actual_size == 0 {
                    errors.push(format!("❌ Output file size is 0: {}", output_path.display()));
                }
                
                // 验证文件大小合理性
                if let Some(expected) = expected_size {
                    if expected > 0 {
                        let ratio = actual_size as f64 / expected as f64;
                        if ratio < 0.01 {
                            warnings.push(format!("⚠️ Output file abnormally small ({:.2}% of original)", ratio * 100.0));
                        } else if ratio > 10.0 {
                            warnings.push(format!("⚠️ Output file abnormally large ({:.2}x of original)", ratio));
                        }
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
    
    /// 🔍 完整验证流程
    pub fn validate_full_conversion(
        files: &[InputFile],
        config: &ConversionConfig,
        mode: ConversionMode,
    ) -> ValidationResult {
        tracing::info!("🔍 Starting multi-level validation");
        
        // Level 1: 输入验证
        let input_validation = Self::validate_input_files(files);
        if !input_validation.valid {
            tracing::error!("❌ Level 1 failed: Input file validation failed");
            return input_validation;
        }
        tracing::info!("✅ Level 1 passed: Input file validation");
        
        // Level 2: 参数验证
        let param_validation = Self::validate_parameters(config, files);
        if !param_validation.valid {
            tracing::error!("❌ Level 2 failed: Parameter validation failed");
            let mut result = param_validation;
            result.warnings.extend(input_validation.warnings);
            return result;
        }
        tracing::info!("✅ Level 2 passed: Parameter validation");
        
        // Level 3: 模式一致性验证
        let mode_validation = Self::validate_mode_consistency(mode, config);
        if !mode_validation.valid {
            tracing::error!("❌ Level 3 failed: Mode consistency validation failed");
            let mut result = mode_validation;
            result.warnings.extend(input_validation.warnings);
            result.warnings.extend(param_validation.warnings);
            return result;
        }
        tracing::info!("✅ Level 3 passed: Mode consistency validation");
        
        // 收集所有警告
        let mut all_warnings = input_validation.warnings;
        all_warnings.extend(param_validation.warnings);
        all_warnings.extend(mode_validation.warnings);
        
        tracing::info!("🎉 All validation levels passed");
        if !all_warnings.is_empty() {
            tracing::warn!("⚠️ Warnings: {:?}", all_warnings);
        }
        
        ValidationResult::success(3).with_warnings(all_warnings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;
    
    #[test]
    fn test_validate_empty_files() {
        let files = vec![];
        let result = ConversionValidator::validate_input_files(&files);
        assert!(!result.valid);
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
        
        let result = ConversionValidator::validate_input_files(&files);
        assert!(result.valid);
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
        
        let result = ConversionValidator::validate_parameters(&config, &[]);
        assert!(!result.valid);
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
        
        let result = ConversionValidator::validate_parameters(&config, &[]);
        assert!(!result.valid);
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
        
        let result = ConversionValidator::validate_parameters(&config, &[]);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("JPEG format does not support lossless mode")));
    }
    
    #[test]
    fn test_validate_heic_transparency_warning() {
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
            format: "heic".to_string(),
            quality: Some(80),
            speed: Some(4),
            lossless: false,
        };
        
        let result = ConversionValidator::validate_parameters(&config, &files);
        assert!(result.valid);
        assert!(result.warnings.iter().any(|w| w.contains("HEIC does not support transparency")));
    }
    
    #[test]
    fn test_validate_output_missing_file() {
        let result = ConversionValidator::validate_output(Path::new("/nonexistent/file.webp"), None);
        assert!(!result.valid);
        assert_eq!(result.level, 4);
        assert!(result.errors.iter().any(|e| e.contains("Output file not generated")));
    }
    
    #[test]
    fn test_validate_output_zero_size() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("empty.webp");
        File::create(&file_path).unwrap(); // 创建空文件
        
        let result = ConversionValidator::validate_output(&file_path, None);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("Output file size is 0")));
    }
    
    #[test]
    fn test_validate_output_size_ratio() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.webp");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(&[0u8; 10]).unwrap(); // 写入10字节
        
        // 10 bytes vs 10,000 expected = 0.001 ratio (< 0.01)
        let result = ConversionValidator::validate_output(&file_path, Some(10_000));
        assert!(result.valid);
        assert!(result.warnings.iter().any(|w| w.contains("Output file abnormally small")), 
                "Expected warning about small file, got: {:?}", result.warnings);
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
        
        let result = ConversionValidator::validate_full_conversion(&files, &config, ConversionMode::Manual);
        assert!(result.valid);
        assert_eq!(result.level, 3);
    }
}
