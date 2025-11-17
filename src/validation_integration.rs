/**
 * 🔥 验证系统集成模块
 * 
 * 将多级验证系统集成到转换流程中
 * - CLI集成
 * - 转换流程集成
 * - 格式特定检查增强
 * - 异步验证支持
 */

use crate::conversion_validator::*;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};

/// CLI验证结果显示器
pub struct ValidationDisplay;

impl ValidationDisplay {
    /// 显示验证结果
    pub fn display_result(result: &ValidationResult) {
        if result.valid {
            println!("✅ Validation passed");
            println!("   Validation level: Level {}", result.level);
            
            if !result.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &result.warnings {
                    println!("   {}", warning);
                }
            }
        } else {
            println!("❌ Validation failed");
            println!("   Failed at level: Level {}", result.level);
            
            if !result.errors.is_empty() {
                println!("\nErrors:");
                for error in &result.errors {
                    println!("   {}", error);
                }
            }
            
            if !result.warnings.is_empty() {
                println!("\nWarnings:");
                for warning in &result.warnings {
                    println!("   {}", warning);
                }
            }
        }
    }
    
    /// 显示简洁的验证摘要
    pub fn display_summary(result: &ValidationResult) {
        if result.valid {
            print!("✅ ");
        } else {
            print!("❌ ");
        }
        
        if !result.warnings.is_empty() {
            print!("⚠️ {} warnings ", result.warnings.len());
        }
        
        if !result.errors.is_empty() {
            print!("❌ {} errors ", result.errors.len());
        }
        
        println!();
    }
    
    /// 显示详细的验证报告
    pub fn display_detailed_report(result: &ValidationResult) {
        println!("\n{}", "═".repeat(60));
        println!("  Validation Report");
        println!("{}", "═".repeat(60));
        
        // 状态
        let status = if result.valid {
            "Passed ✅"
        } else {
            "Failed ❌"
        };
        println!("Status: {}", status);
        println!("Level: Level {}", result.level);
        
        // 错误
        if !result.errors.is_empty() {
            println!("\nError List:");
            for (i, error) in result.errors.iter().enumerate() {
                println!("  {}. {}", i + 1, error);
            }
        }
        
        // 警告
        if !result.warnings.is_empty() {
            println!("\nWarning List:");
            for (i, warning) in result.warnings.iter().enumerate() {
                println!("  {}. {}", i + 1, warning);
            }
        }
        
        println!("{}", "═".repeat(60));
    }
}

/// 转换流程验证器
pub struct ConversionFlowValidator;

impl ConversionFlowValidator {
    /// 验证转换前的准备工作
    pub fn validate_pre_conversion(
        input_paths: &[PathBuf],
        target_format: &str,
        quality: Option<u32>,
        speed: Option<u32>,
        lossless: bool,
        mode: ConversionMode,
    ) -> Result<ValidationResult> {
        // 构建输入文件列表
        let mut files = Vec::new();
        for path in input_paths {
            let metadata = std::fs::metadata(path)
                .with_context(|| format!("Cannot read file: {}", path.display()))?;
            
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            let ext = path.extension()
                .and_then(|e| e.to_str())
                .map(|e| format!(".{}", e))
                .unwrap_or_default();
            
            files.push(InputFile {
                file_path: path.clone(),
                name,
                ext,
                size: metadata.len(),
                is_animated: false, // TODO: 检测动画
            });
        }
        
        // 构建配置
        let config = ConversionConfig {
            format: target_format.to_string(),
            quality,
            speed,
            lossless,
        };
        
        // 执行完整验证
        let result = ConversionValidator::validate_full_conversion(&files, &config, mode);
        
        Ok(result)
    }
    
    /// 验证转换后的输出
    pub fn validate_post_conversion(
        output_path: &Path,
        input_size: u64,
    ) -> Result<ValidationResult> {
        let result = ConversionValidator::validate_output(output_path, Some(input_size));
        Ok(result)
    }
    
    /// 批量验证多个文件
    pub fn validate_batch(
        input_paths: &[PathBuf],
        target_format: &str,
        quality: Option<u32>,
        speed: Option<u32>,
        lossless: bool,
    ) -> Result<Vec<ValidationResult>> {
        let mut results = Vec::new();
        
        for path in input_paths {
            let result = Self::validate_pre_conversion(
                &[path.clone()],
                target_format,
                quality,
                speed,
                lossless,
                ConversionMode::Manual,
            )?;
            results.push(result);
        }
        
        Ok(results)
    }
}

/// 格式特定检查增强
pub struct FormatSpecificChecks;

impl FormatSpecificChecks {
    /// WebP特定检查
    pub fn check_webp(files: &[InputFile], config: &ConversionConfig) -> Vec<String> {
        let mut warnings = Vec::new();
        
        // 检查大图
        for file in files {
            if file.size > 16 * 1024 * 1024 { // 16MB
                warnings.push(format!(
                    "⚠️ 文件 {} 较大 ({:.2} MB)，WebP编码可能较慢",
                    file.name,
                    file.size as f64 / (1024.0 * 1024.0)
                ));
            }
        }
        
        // 检查质量设置
        if let Some(quality) = config.quality {
            if quality < 70 {
                warnings.push(format!(
                    "⚠️ WebP质量设置较低 ({}), 可能出现明显压缩痕迹",
                    quality
                ));
            }
        }
        
        warnings
    }
    
    /// AVIF特定检查
    pub fn check_avif(files: &[InputFile], config: &ConversionConfig) -> Vec<String> {
        let mut warnings = Vec::new();
        
        // 检查动画
        let has_animation = files.iter().any(|f| f.is_animated);
        if has_animation {
            warnings.push("⚠️ AVIF动画支持有限，建议使用WebP或GIF".to_string());
        }
        
        // 检查速度设置
        if let Some(speed) = config.speed {
            if speed > 6 {
                warnings.push(format!(
                    "⚠️ AVIF速度设置较高 ({}), 可能影响压缩效率",
                    speed
                ));
            }
        }
        
        // 检查大图
        for file in files {
            let pixels = file.size / 3; // 粗略估计
            if pixels > 4000 * 4000 {
                warnings.push(format!(
                    "⚠️ 文件 {} 分辨率较高，AVIF编码可能需要较长时间",
                    file.name
                ));
            }
        }
        
        warnings
    }
    
    /// JXL特定检查
    pub fn check_jxl(files: &[InputFile], config: &ConversionConfig) -> Vec<String> {
        let mut warnings = Vec::new();
        
        // 检查JPEG输入
        let has_jpeg = files.iter().any(|f| {
            let ext = f.ext.to_lowercase();
            ext == ".jpg" || ext == ".jpeg"
        });
        
        if has_jpeg && !config.lossless {
            warnings.push("💡 提示: JPEG → JXL 建议使用无损模式以利用JPEG重新打包".to_string());
        }
        
        // 检查质量设置
        if let Some(quality) = config.quality {
            if quality < 60 {
                warnings.push(format!(
                    "⚠️ JXL质量设置较低 ({}), 可能不如WebP或AVIF",
                    quality
                ));
            }
        }
        
        warnings
    }
    
    /// PNG特定检查
    pub fn check_png(files: &[InputFile], _config: &ConversionConfig) -> Vec<String> {
        let mut warnings = Vec::new();
        
        // 检查大文件
        for file in files {
            if file.size > 10 * 1024 * 1024 { // 10MB
                warnings.push(format!(
                    "⚠️ 文件 {} 较大 ({:.2} MB)，PNG压缩可能需要较长时间",
                    file.name,
                    file.size as f64 / (1024.0 * 1024.0)
                ));
            }
        }
        
        warnings
    }
    
    /// 执行所有格式特定检查
    pub fn check_all(files: &[InputFile], config: &ConversionConfig) -> Vec<String> {
        let format = config.format.to_lowercase();
        
        match format.as_str() {
            "webp" => Self::check_webp(files, config),
            "avif" => Self::check_avif(files, config),
            "jxl" | "jpegxl" => Self::check_jxl(files, config),
            "png" => Self::check_png(files, config),
            _ => Vec::new(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;
    
    #[test]
    fn test_validation_display() {
        let result = ValidationResult {
            valid: true,
            level: 3,
            errors: vec![],
            warnings: vec!["测试警告".to_string()],
        };
        
        // 这个测试只是确保不会panic
        ValidationDisplay::display_result(&result);
        ValidationDisplay::display_summary(&result);
        ValidationDisplay::display_detailed_report(&result);
    }
    
    #[test]
    fn test_format_specific_checks_webp() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.png");
        File::create(&file_path).unwrap();
        
        let files = vec![InputFile {
            file_path,
            name: "test.png".to_string(),
            ext: ".png".to_string(),
            size: 20 * 1024 * 1024, // 20MB
            is_animated: false,
        }];
        
        let config = ConversionConfig {
            format: "webp".to_string(),
            quality: Some(60),
            speed: Some(4),
            lossless: false,
        };
        
        let warnings = FormatSpecificChecks::check_webp(&files, &config);
        assert!(!warnings.is_empty());
        assert!(warnings.iter().any(|w| w.contains("较大")));
        assert!(warnings.iter().any(|w| w.contains("质量设置较低")));
    }
    
    #[test]
    fn test_format_specific_checks_avif() {
        let files = vec![InputFile {
            file_path: PathBuf::from("test.gif"),
            name: "test.gif".to_string(),
            ext: ".gif".to_string(),
            size: 1024,
            is_animated: true,
        }];
        
        let config = ConversionConfig {
            format: "avif".to_string(),
            quality: Some(80),
            speed: Some(8),
            lossless: false,
        };
        
        let warnings = FormatSpecificChecks::check_avif(&files, &config);
        assert!(!warnings.is_empty());
        assert!(warnings.iter().any(|w| w.contains("动画")));
        assert!(warnings.iter().any(|w| w.contains("速度设置较高")));
    }
    
    #[test]
    fn test_format_specific_checks_jxl() {
        let files = vec![InputFile {
            file_path: PathBuf::from("test.jpg"),
            name: "test.jpg".to_string(),
            ext: ".jpg".to_string(),
            size: 1024,
            is_animated: false,
        }];
        
        let config = ConversionConfig {
            format: "jxl".to_string(),
            quality: Some(50),
            speed: Some(4),
            lossless: false,
        };
        
        let warnings = FormatSpecificChecks::check_jxl(&files, &config);
        assert!(!warnings.is_empty());
        assert!(warnings.iter().any(|w| w.contains("JPEG") || w.contains("质量")));
    }
}
