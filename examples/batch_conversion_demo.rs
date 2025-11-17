/// 🔥 批量转换系统演示
/// 
/// 展示完整的批量处理功能：
/// - 单文件转换
/// - 多文件批量转换
/// - 目录递归转换
/// - 错误处理和重试
/// - 统一日志系统

use pixly_kernel::{
    BatchConverter, BatchConverterConfig, ErrorStrategy,
    FileCollector, LogManager, LogConfig,
};
use pixly_kernel::log_manager::LogLevel;
use pixly_kernel::conversion_core::ConversionConfig;
use std::path::PathBuf;
use anyhow::Result;

fn main() -> Result<()> {
    // 设置日志系统
    setup_logging();
    
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║           Batch Conversion System Demo                   ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    
    // 演示1: 单文件转换
    demo_single_file()?;
    
    // 演示2: 多文件批量转换
    demo_batch_conversion()?;
    
    // 演示3: 目录递归转换
    demo_directory_conversion()?;
    
    // 演示4: 带重试的转换
    demo_retry_conversion()?;
    
    // 演示5: 不同错误策略
    demo_error_strategies()?;
    
    println!("\n✅ All demos completed!");
    
    Ok(())
}

fn setup_logging() {
    // 开发模式：显示所有日志
    LogManager::global().set_config(LogConfig::development());
    
    // 生产模式：只显示重要信息
    // LogManager::global().set_config(LogConfig::production());
    
    // Verbose模式：显示详细信息
    // LogManager::global().set_config(LogConfig::verbose());
}

fn demo_single_file() -> Result<()> {
    println!("\n📝 Demo 1: Single File Conversion");
    println!("{}", "─".repeat(60));
    
    let config = BatchConverterConfig {
        conversion_config: ConversionConfig::default(),
        max_parallel: 1,
        overwrite: true,
        show_progress: true,
        ..Default::default()
    };
    
    let _converter = BatchConverter::new(config);
    
    // 示例：转换单个文件
    // _converter.convert_single(
    //     &PathBuf::from("input.jpg"),
    //     &PathBuf::from("output.webp"),
    //     "webp",
    // )?;
    
    println!("✅ Single file conversion demo completed");
    
    Ok(())
}

fn demo_batch_conversion() -> Result<()> {
    println!("\n📝 Demo 2: Batch Conversion");
    println!("{}", "─".repeat(60));
    
    let config = BatchConverterConfig {
        conversion_config: ConversionConfig::default(),
        max_parallel: 4,
        error_strategy: ErrorStrategy::ContinueOnError,
        overwrite: true,
        show_progress: true,
    };
    
    let _converter = BatchConverter::new(config);
    
    // 示例：批量转换多个文件
    let input_files = vec![
        // PathBuf::from("image1.jpg"),
        // PathBuf::from("image2.png"),
        // PathBuf::from("image3.webp"),
    ];
    
    if !input_files.is_empty() {
        let result = _converter.convert_batch(
            input_files,
            &PathBuf::from("output"),
            "avif",
        )?;
        
        println!("\n📊 Batch Result:");
        println!("  Total: {}", result.total);
        println!("  Success: {}", result.success);
        println!("  Failed: {}", result.failed);
        println!("  Success rate: {:.1}%", result.success_rate() * 100.0);
    } else {
        println!("ℹ️  No input files provided (demo mode)");
    }
    
    println!("✅ Batch conversion demo completed");
    
    Ok(())
}

fn demo_directory_conversion() -> Result<()> {
    println!("\n📝 Demo 3: Directory Conversion");
    println!("{}", "─".repeat(60));
    
    let config = BatchConverterConfig {
        conversion_config: ConversionConfig::default(),
        max_parallel: 4,
        error_strategy: ErrorStrategy::ContinueOnError,
        overwrite: false,
        show_progress: true,
    };
    
    let _converter = BatchConverter::new(config);
    
    // 示例：递归转换目录
    // let result = _converter.convert_directory(
    //     &PathBuf::from("input_dir"),
    //     &PathBuf::from("output_dir"),
    //     "webp",
    //     vec!["jpg".to_string(), "png".to_string()],
    //     true, // recursive
    // )?;
    
    println!("ℹ️  Directory conversion demo (skipped - no input directory)");
    println!("✅ Directory conversion demo completed");
    
    Ok(())
}

fn demo_retry_conversion() -> Result<()> {
    println!("\n📝 Demo 4: Retry Conversion");
    println!("{}", "─".repeat(60));
    
    let config = BatchConverterConfig {
        conversion_config: ConversionConfig::default(),
        max_parallel: 2,
        error_strategy: ErrorStrategy::RetryOnError { max_retries: 3 },
        overwrite: true,
        show_progress: true,
    };
    
    let _converter = BatchConverter::new(config);
    
    // 示例：带重试的转换
    let input_files = vec![
        // PathBuf::from("problematic1.jpg"),
        // PathBuf::from("problematic2.png"),
    ];
    
    if !input_files.is_empty() {
        let result = _converter.convert_with_retry(
            input_files,
            &PathBuf::from("output"),
            "jxl",
            3, // max retries
        )?;
        
        println!("\n📊 Retry Result:");
        println!("  Total: {}", result.total);
        println!("  Success: {}", result.success);
        println!("  Failed: {}", result.failed);
    } else {
        println!("ℹ️  No input files provided (demo mode)");
    }
    
    println!("✅ Retry conversion demo completed");
    
    Ok(())
}

fn demo_error_strategies() -> Result<()> {
    println!("\n📝 Demo 5: Error Strategies");
    println!("{}", "─".repeat(60));
    
    // 策略1: 继续处理
    println!("\n1️⃣  ContinueOnError - Process all files, collect errors");
    let config1 = BatchConverterConfig {
        error_strategy: ErrorStrategy::ContinueOnError,
        ..Default::default()
    };
    let _ = BatchConverter::new(config1);
    
    // 策略2: 遇错即停
    println!("\n2️⃣  StopOnError - Stop on first error");
    let config2 = BatchConverterConfig {
        error_strategy: ErrorStrategy::StopOnError,
        ..Default::default()
    };
    let _ = BatchConverter::new(config2);
    
    // 策略3: 重试失败
    println!("\n3️⃣  RetryOnError - Retry failed conversions");
    let config3 = BatchConverterConfig {
        error_strategy: ErrorStrategy::RetryOnError { max_retries: 3 },
        ..Default::default()
    };
    let _ = BatchConverter::new(config3);
    
    println!("\n✅ Error strategies demo completed");
    
    Ok(())
}

/// 演示文件收集器
#[allow(dead_code)]
fn demo_file_collector() -> Result<()> {
    println!("\n📝 File Collector Demo");
    println!("{}", "─".repeat(60));
    
    // 收集特定扩展名的文件
    let _collector = FileCollector::new(vec!["jpg".to_string(), "png".to_string()])
        .recursive(true)
        .max_depth(3);
    
    // let files = collector.collect(&PathBuf::from("input_dir"))?;
    // println!("Found {} files", files.len());
    
    println!("✅ File collector demo completed");
    
    Ok(())
}

/// 演示日志级别
#[allow(dead_code)]
fn demo_log_levels() {
    println!("\n📝 Log Levels Demo");
    println!("{}", "─".repeat(60));
    
    let logger = LogManager::global();
    
    // 不同级别的日志
    logger.log(LogLevel::Debug, "This is a debug message");
    logger.log(LogLevel::Verbose, "This is a verbose message");
    logger.log(LogLevel::Info, "This is an info message");
    logger.log(LogLevel::Warning, "This is a warning message");
    logger.log(LogLevel::Error, "This is an error message");
    
    // 带详细信息的日志
    logger.log_with_details(
        LogLevel::Info,
        "Conversion details",
        &[
            ("Input", "image.jpg"),
            ("Output", "image.webp"),
            ("Quality", "85"),
        ],
    );
    
    // 分隔线和标题
    logger.separator();
    logger.header("Section Title");
    
    println!("✅ Log levels demo completed");
}
