/**
 * 验证系统CLI演示
 * 
 * 展示如何在CLI中集成验证系统
 * 
 * 运行: cargo run --example validation_cli_demo
 */

use pixly_kernel::conversion_validator::*;
use pixly_kernel::validation_integration::*;
use std::path::PathBuf;

fn main() {
    println!("🔍 Validation System CLI Demo\n");
    
    // 示例1: 基础验证
    demo_basic_validation();
    
    // 示例2: 格式特定检查
    demo_format_specific_checks();
    
    // 示例3: 批量验证
    demo_batch_validation();
    
    // 示例4: 转换流程集成
    demo_conversion_flow();
}

fn demo_basic_validation() {
    println!("{}",  "═".repeat(60));
    println!("Example 1: Basic Validation");
    println!("{}\n", "═".repeat(60));
    
    // 模拟输入文件
    let files = vec![
        InputFile {
            file_path: PathBuf::from("test1.png"),
            name: "test1.png".to_string(),
            ext: ".png".to_string(),
            size: 1024 * 1024, // 1MB
            is_animated: false,
        },
        InputFile {
            file_path: PathBuf::from("test2.jpg"),
            name: "test2.jpg".to_string(),
            ext: ".jpg".to_string(),
            size: 2 * 1024 * 1024, // 2MB
            is_animated: false,
        },
    ];
    
    // 配置
    let config = ConversionConfig {
        format: "webp".to_string(),
        quality: Some(80),
        speed: Some(4),
        lossless: false,
    };
    
    // 执行验证
    let result = ConversionValidator::validate_full_conversion(
        &files,
        &config,
        ConversionMode::Manual,
    );
    
    // 显示结果
    ValidationDisplay::display_detailed_report(&result);
    println!();
}

fn demo_format_specific_checks() {
    println!("{}", "═".repeat(60));
    println!("Example 2: Format-Specific Checks");
    println!("{}\n", "═".repeat(60));
    
    // WebP检查
    println!("📋 WebP Format Check:");
    let files = vec![InputFile {
        file_path: PathBuf::from("large.png"),
        name: "large.png".to_string(),
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
    for warning in &warnings {
        println!("  {}", warning);
    }
    
    // AVIF检查
    println!("\n📋 AVIF Format Check:");
    let files = vec![InputFile {
        file_path: PathBuf::from("animated.gif"),
        name: "animated.gif".to_string(),
        ext: ".gif".to_string(),
        size: 1024 * 1024,
        is_animated: true,
    }];
    
    let config = ConversionConfig {
        format: "avif".to_string(),
        quality: Some(80),
        speed: Some(8),
        lossless: false,
    };
    
    let warnings = FormatSpecificChecks::check_avif(&files, &config);
    for warning in &warnings {
        println!("  {}", warning);
    }
    
    // JXL检查
    println!("\n📋 JXL Format Check:");
    let files = vec![InputFile {
        file_path: PathBuf::from("photo.jpg"),
        name: "photo.jpg".to_string(),
        ext: ".jpg".to_string(),
        size: 3 * 1024 * 1024,
        is_animated: false,
    }];
    
    let config = ConversionConfig {
        format: "jxl".to_string(),
        quality: Some(50),
        speed: Some(4),
        lossless: false,
    };
    
    let warnings = FormatSpecificChecks::check_jxl(&files, &config);
    for warning in &warnings {
        println!("  {}", warning);
    }
    
    println!();
}

fn demo_batch_validation() {
    println!("{}", "═".repeat(60));
    println!("Example 3: Batch Validation");
    println!("{}\n", "═".repeat(60));
    
    let paths = vec![
        PathBuf::from("file1.png"),
        PathBuf::from("file2.jpg"),
        PathBuf::from("file3.gif"),
    ];
    
    println!("Validating {} files...\n", paths.len());
    
    for (i, path) in paths.iter().enumerate() {
        println!("File {}: {}", i + 1, path.display());
        
        // 模拟验证
        let files = vec![InputFile {
            file_path: path.clone(),
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            ext: path.extension().map(|e| format!(".{}", e.to_str().unwrap())).unwrap_or_default(),
            size: 1024 * 1024,
            is_animated: false,
        }];
        
        let config = ConversionConfig {
            format: "webp".to_string(),
            quality: Some(80),
            speed: Some(4),
            lossless: false,
        };
        
        let result = ConversionValidator::validate_parameters(&config, &files);
        ValidationDisplay::display_summary(&result);
    }
    
    println!();
}

fn demo_conversion_flow() {
    println!("{}", "═".repeat(60));
    println!("Example 4: Conversion Flow Integration");
    println!("{}\n", "═".repeat(60));
    
    println!("🔄 Pre-conversion validation:");
    let files = vec![InputFile {
        file_path: PathBuf::from("input.png"),
        name: "input.png".to_string(),
        ext: ".png".to_string(),
        size: 1024 * 1024,
        is_animated: false,
    }];
    
    let config = ConversionConfig {
        format: "webp".to_string(),
        quality: Some(85),
        speed: Some(4),
        lossless: false,
    };
    
    let pre_result = ConversionValidator::validate_full_conversion(
        &files,
        &config,
        ConversionMode::Smart,
    );
    
    ValidationDisplay::display_result(&pre_result);
    
    if pre_result.valid {
        println!("\n✅ Validation passed, conversion can start");
        println!("🔄 Executing conversion...");
        println!("✅ Conversion completed");
        
        println!("\n🔍 Post-conversion validation:");
        // 模拟输出验证
        println!("  ✅ Output file exists");
        println!("  ✅ File size is normal");
        println!("  ✅ Compression ratio is reasonable");
    } else {
        println!("\n❌ Validation failed, conversion cancelled");
    }
    
    println!();
}
