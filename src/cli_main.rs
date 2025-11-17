/// 🔥 Pixly CLI - Phase 3 完整实现
/// 
/// 架构原则：
/// - 集成 Phase 2 批量处理系统
/// - 使用统一日志管理
/// - 完整的命令行参数解析
/// - 支持单文件、多文件、目录处理

use std::path::PathBuf;
use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{
    BatchConverter, BatchConverterConfig, ErrorStrategy,
    FileCollector, LogManager, LogConfig,
    conversion_core::{execute_conversion, ConversionConfig},
};

/// Pixly - Modern image/video conversion tool with AI optimization
#[derive(Parser)]
#[command(name = "pixly")]
#[command(about = "Modern image/video conversion tool with AI optimization", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Log mode (production, development, verbose)
    #[arg(long, global = true, default_value = "production")]
    log_mode: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert a single file
    Convert {
        /// Input file
        input: String,
        
        /// Output file
        output: String,
        
        /// Quality (1-100)
        #[arg(short, long, default_value = "85")]
        quality: u8,
        
        /// Speed (1-10)
        #[arg(short, long, default_value = "4")]
        speed: u8,
        
        /// Preserve metadata
        #[arg(long, default_value = "true")]
        metadata: bool,
        
        /// Merge XMP sidecar
        #[arg(long)]
        merge_xmp: bool,
        
        /// Keep animation
        #[arg(long, default_value = "true")]
        animated: bool,
        
        /// Lossless mode
        #[arg(long)]
        lossless: bool,
        
        /// Overwrite existing files
        #[arg(long)]
        overwrite: bool,
        
        /// In-place conversion (replace input file)
        #[arg(long)]
        in_place: bool,
    },
    
    /// Batch convert files
    Batch {
        /// Input directories or file list
        #[arg(required = true)]
        inputs: Vec<String>,
        
        /// Output directory
        #[arg(short, long)]
        output: String,
        
        /// Target format
        #[arg(short, long)]
        format: String,
        
        /// Quality (1-100)
        #[arg(short, long, default_value = "85")]
        quality: u8,
        
        /// Speed (1-10)
        #[arg(short, long, default_value = "4")]
        speed: u8,
        
        /// Recursive processing
        #[arg(short, long)]
        recursive: bool,
        
        /// Maximum recursion depth
        #[arg(long)]
        max_depth: Option<usize>,
        
        /// File extension filter (comma-separated)
        #[arg(long)]
        extensions: Option<String>,
        
        /// Number of parallel jobs
        #[arg(short = 'j', long)]
        parallel: Option<usize>,
        
        /// Error handling strategy (continue, stop, retry)
        #[arg(long, default_value = "continue")]
        on_error: String,
        
        /// Maximum retry count (only when on-error=retry)
        #[arg(long, default_value = "3")]
        max_retries: usize,
        
        /// Overwrite existing files
        #[arg(long)]
        overwrite: bool,
        
        /// Show progress
        #[arg(long, default_value = "true")]
        progress: bool,
        
        /// Preserve metadata
        #[arg(long, default_value = "true")]
        metadata: bool,
        
        /// Merge XMP sidecar
        #[arg(long)]
        merge_xmp: bool,
        
        /// Keep animation
        #[arg(long, default_value = "true")]
        animated: bool,
        
        /// Lossless mode
        #[arg(long)]
        lossless: bool,
        
        /// In-place conversion (replace input files)
        #[arg(long)]
        in_place: bool,
    },
    
    /// Convert directory
    Directory {
        /// Input directory
        input: String,
        
        /// Output directory
        #[arg(short, long)]
        output: String,
        
        /// Target format
        #[arg(short, long)]
        format: String,
        
        /// Quality (1-100)
        #[arg(short, long, default_value = "85")]
        quality: u8,
        
        /// Speed (1-10)
        #[arg(short, long, default_value = "4")]
        speed: u8,
        
        /// Recursive processing
        #[arg(short, long)]
        recursive: bool,
        
        /// Maximum recursion depth
        #[arg(long)]
        max_depth: Option<usize>,
        
        /// File extension filter (comma-separated)
        #[arg(long, default_value = "jpg,jpeg,png,webp,gif,bmp,tiff,avif,jxl")]
        extensions: String,
        
        /// Number of parallel jobs
        #[arg(short = 'j', long)]
        parallel: Option<usize>,
        
        /// Error handling strategy (continue, stop, retry)
        #[arg(long, default_value = "continue")]
        on_error: String,
        
        /// Maximum retry count (only when on-error=retry)
        #[arg(long, default_value = "3")]
        max_retries: usize,
        
        /// Overwrite existing files
        #[arg(long)]
        overwrite: bool,
        
        /// Show progress
        #[arg(long, default_value = "true")]
        progress: bool,
        
        /// Preserve metadata
        #[arg(long, default_value = "true")]
        metadata: bool,
        
        /// Merge XMP sidecar
        #[arg(long)]
        merge_xmp: bool,
        
        /// Keep animation
        #[arg(long, default_value = "true")]
        animated: bool,
        
        /// Lossless mode
        #[arg(long)]
        lossless: bool,
        
        /// In-place conversion (replace input files)
        #[arg(long)]
        in_place: bool,
    },
}

/// CLI 主入口
pub fn run() -> Result<()> {
    let cli = Cli::parse();
    
    // 设置日志模式
    setup_logging(&cli.log_mode);
    
    // 执行命令
    match cli.command {
        Commands::Convert {
            input,
            output,
            quality,
            speed,
            metadata,
            merge_xmp,
            animated,
            lossless,
            overwrite,
            in_place,
        } => {
            handle_convert(
                &input,
                &output,
                quality,
                speed,
                metadata,
                merge_xmp,
                animated,
                lossless,
                overwrite,
                in_place,
            )
        }
        
        Commands::Batch {
            inputs,
            output,
            format,
            quality,
            speed,
            recursive,
            max_depth,
            extensions,
            parallel,
            on_error,
            max_retries,
            overwrite,
            progress,
            metadata,
            merge_xmp,
            animated,
            lossless,
            in_place,
        } => {
            handle_batch(
                inputs,
                &output,
                &format,
                quality,
                speed,
                recursive,
                max_depth,
                extensions,
                parallel,
                &on_error,
                max_retries,
                overwrite,
                progress,
                metadata,
                merge_xmp,
                animated,
                lossless,
                in_place,
            )
        }
        
        Commands::Directory {
            input,
            output,
            format,
            quality,
            speed,
            recursive,
            max_depth,
            extensions,
            parallel,
            on_error,
            max_retries,
            overwrite,
            progress,
            metadata,
            merge_xmp,
            animated,
            lossless,
            in_place,
        } => {
            handle_directory(
                &input,
                &output,
                &format,
                quality,
                speed,
                recursive,
                max_depth,
                &extensions,
                parallel,
                &on_error,
                max_retries,
                overwrite,
                progress,
                metadata,
                merge_xmp,
                animated,
                lossless,
                in_place,
            )
        }
    }
}

/// 设置日志模式
fn setup_logging(mode: &str) {
    let config = match mode {
        "development" | "dev" => LogConfig::development(),
        "verbose" | "v" => LogConfig::verbose(),
        _ => LogConfig::production(),
    };
    
    LogManager::global().set_config(config);
}

/// Handle single file conversion
fn handle_convert(
    input: &str,
    output: &str,
    quality: u8,
    speed: u8,
    metadata: bool,
    merge_xmp: bool,
    animated: bool,
    lossless: bool,
    overwrite: bool,
    in_place: bool,
) -> Result<()> {
    let logger = LogManager::global();
    
    logger.header("Single File Conversion");
    logger.log(crate::log_manager::LogLevel::Info, &format!("Input: {}", input));
    
    // Handle in-place conversion
    if in_place {
        return handle_in_place_convert(
            input,
            quality,
            speed,
            metadata,
            merge_xmp,
            animated,
            lossless,
        );
    }
    
    logger.log(crate::log_manager::LogLevel::Info, &format!("Output: {}", output));
    
    // 检查输入文件
    let input_path = PathBuf::from(input);
    if !input_path.exists() {
        anyhow::bail!("Input file does not exist: {}", input);
    }
    
    // 检查输出文件
    let output_path = PathBuf::from(output);
    if output_path.exists() && !overwrite {
        anyhow::bail!("Output file already exists (use --overwrite to replace)");
    }
    
    // 提取格式
    let format = output_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("webp");
    
    // 创建配置
    let config = ConversionConfig {
        quality,
        speed,
        preserve_metadata: metadata,
        keep_animated: animated,
        lossless,
        merge_xmp_sidecar: merge_xmp,
        feature_toggles: None,
        chroma_subsampling: None,
        alpha_quality: None,
        effort: None,
        resize: None,
        quantize: None,
        sharpen: None,
        output_dir: None,
        normalize_filenames: false,
        format_specific_params: None,
    };
    
    // 执行转换
    let start = std::time::Instant::now();
    execute_conversion(&input_path, &output_path, format, &config)?;
    let elapsed = start.elapsed();
    
    logger.log(
        crate::log_manager::LogLevel::Info,
        &format!("✅ Conversion completed in {:.2}s", elapsed.as_secs_f64())
    );
    
    Ok(())
}

/// Handle in-place conversion with multi-level verification
fn handle_in_place_convert(
    input: &str,
    quality: u8,
    speed: u8,
    metadata: bool,
    merge_xmp: bool,
    animated: bool,
    lossless: bool,
) -> Result<()> {
    let logger = LogManager::global();
    
    logger.log(crate::log_manager::LogLevel::Warning, "⚠️  In-place conversion mode");
    logger.log(crate::log_manager::LogLevel::Info, "Original file will be replaced after verification");
    
    // Level 1: Verify input file exists and is readable
    let input_path = PathBuf::from(input);
    if !input_path.exists() {
        anyhow::bail!("Input file does not exist: {}", input);
    }
    
    let input_metadata = std::fs::metadata(&input_path)?;
    if !input_metadata.is_file() {
        anyhow::bail!("Input path is not a file: {}", input);
    }
    
    let original_size = input_metadata.len();
    logger.log(
        crate::log_manager::LogLevel::Verbose,
        &format!("Original file size: {} bytes", original_size)
    );
    
    // Level 2: Create temporary output file
    let temp_output = input_path.with_extension("pixly_temp");
    logger.log(
        crate::log_manager::LogLevel::Verbose,
        &format!("Temporary file: {}", temp_output.display())
    );
    
    // Extract format from input
    let format = input_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("webp");
    
    // Create conversion config
    let config = ConversionConfig {
        quality,
        speed,
        preserve_metadata: metadata,
        keep_animated: animated,
        lossless,
        merge_xmp_sidecar: merge_xmp,
        feature_toggles: None,
        chroma_subsampling: None,
        alpha_quality: None,
        effort: None,
        resize: None,
        quantize: None,
        sharpen: None,
        output_dir: None,
        normalize_filenames: false,
        format_specific_params: None,
    };
    
    // Level 3: Execute conversion to temporary file
    logger.log(crate::log_manager::LogLevel::Info, "Converting to temporary file...");
    let start = std::time::Instant::now();
    execute_conversion(&input_path, &temp_output, format, &config)?;
    let elapsed = start.elapsed();
    
    // Level 4: Verify temporary file was created successfully
    if !temp_output.exists() {
        anyhow::bail!("Temporary file was not created");
    }
    
    let temp_metadata = std::fs::metadata(&temp_output)?;
    let temp_size = temp_metadata.len();
    
    if temp_size == 0 {
        std::fs::remove_file(&temp_output)?;
        anyhow::bail!("Temporary file is empty (0 bytes)");
    }
    
    logger.log(
        crate::log_manager::LogLevel::Info,
        &format!("Temporary file created: {} bytes", temp_size)
    );
    
    // Level 5: Verify file integrity (basic check)
    logger.log(crate::log_manager::LogLevel::Verbose, "Verifying file integrity...");
    
    // Try to open the file to verify it's valid
    if let Err(e) = image::open(&temp_output) {
        std::fs::remove_file(&temp_output)?;
        anyhow::bail!("Temporary file verification failed: {}", e);
    }
    
    logger.log(crate::log_manager::LogLevel::Info, "✅ Temporary file verified");
    
    // Level 6: Create backup of original (optional safety measure)
    let backup_path = input_path.with_extension("pixly_backup");
    logger.log(
        crate::log_manager::LogLevel::Verbose,
        &format!("Creating backup: {}", backup_path.display())
    );
    
    std::fs::copy(&input_path, &backup_path)?;
    
    // Level 7: Replace original file
    logger.log(crate::log_manager::LogLevel::Info, "Replacing original file...");
    
    if let Err(e) = std::fs::rename(&temp_output, &input_path) {
        // Restore from backup if replacement fails
        logger.log(crate::log_manager::LogLevel::Error, &format!("Failed to replace file: {}", e));
        logger.log(crate::log_manager::LogLevel::Info, "Restoring from backup...");
        std::fs::rename(&backup_path, &input_path)?;
        anyhow::bail!("File replacement failed, original restored");
    }
    
    // Level 8: Final verification
    let final_metadata = std::fs::metadata(&input_path)?;
    let final_size = final_metadata.len();
    
    if final_size == 0 {
        // Critical error: restore from backup
        logger.log(crate::log_manager::LogLevel::Error, "Final file is empty!");
        logger.log(crate::log_manager::LogLevel::Info, "Restoring from backup...");
        std::fs::rename(&backup_path, &input_path)?;
        anyhow::bail!("Final verification failed, original restored");
    }
    
    // Level 9: Remove backup
    std::fs::remove_file(&backup_path)?;
    
    // Success
    let compression_ratio = (final_size as f64 / original_size as f64) * 100.0;
    logger.log(
        crate::log_manager::LogLevel::Info,
        &format!("✅ In-place conversion completed in {:.2}s", elapsed.as_secs_f64())
    );
    logger.log(
        crate::log_manager::LogLevel::Info,
        &format!("Original: {} bytes → Final: {} bytes ({:.1}%)", 
            original_size, final_size, compression_ratio)
    );
    
    Ok(())
}

/// Handle batch conversion
#[allow(clippy::too_many_arguments)]
fn handle_batch(
    inputs: Vec<String>,
    output: &str,
    format: &str,
    quality: u8,
    speed: u8,
    recursive: bool,
    max_depth: Option<usize>,
    extensions: Option<String>,
    parallel: Option<usize>,
    on_error: &str,
    max_retries: usize,
    overwrite: bool,
    progress: bool,
    metadata: bool,
    merge_xmp: bool,
    animated: bool,
    lossless: bool,
    _in_place: bool,
) -> Result<()> {
    let logger = LogManager::global();
    logger.header("Batch Conversion");
    
    // 解析扩展名
    let ext_list = if let Some(exts) = extensions {
        exts.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        vec!["jpg", "jpeg", "png", "webp", "gif", "bmp", "tiff", "avif", "jxl"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    };
    
    // 收集文件
    let mut collector = FileCollector::new(ext_list).recursive(recursive);
    if let Some(depth) = max_depth {
        collector = collector.max_depth(depth);
    }
    
    let input_paths: Vec<PathBuf> = inputs.iter().map(PathBuf::from).collect();
    let files = collector.collect_multiple(&input_paths)?;
    
    logger.log(
        crate::log_manager::LogLevel::Info,
        &format!("Found {} files to convert", files.len())
    );
    
    // 创建批量转换配置
    let error_strategy = parse_error_strategy(on_error, max_retries);
    let use_retry = matches!(error_strategy, ErrorStrategy::RetryOnError { .. });
    
    let config = create_batch_config(
        quality,
        speed,
        metadata,
        merge_xmp,
        animated,
        lossless,
        parallel,
        error_strategy,
        overwrite,
        progress,
    );
    
    // 执行批量转换
    let converter = BatchConverter::new(config);
    let output_dir = PathBuf::from(output);
    
    let result = if use_retry {
        converter.convert_with_retry(files, &output_dir, format, max_retries)?
    } else {
        converter.convert_batch(files, &output_dir, format)?
    };
    
    // 显示结果
    result.log_summary();
    
    Ok(())
}

/// Handle directory conversion
#[allow(clippy::too_many_arguments)]
fn handle_directory(
    input: &str,
    output: &str,
    format: &str,
    quality: u8,
    speed: u8,
    recursive: bool,
    max_depth: Option<usize>,
    extensions: &str,
    parallel: Option<usize>,
    on_error: &str,
    max_retries: usize,
    overwrite: bool,
    progress: bool,
    metadata: bool,
    merge_xmp: bool,
    animated: bool,
    lossless: bool,
    _in_place: bool,
) -> Result<()> {
    let logger = LogManager::global();
    logger.header("Directory Conversion");
    
    // 解析扩展名
    let ext_list: Vec<String> = extensions
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    
    // 创建批量转换配置
    let error_strategy = parse_error_strategy(on_error, max_retries);
    let use_retry = matches!(error_strategy, ErrorStrategy::RetryOnError { .. });
    
    let config = create_batch_config(
        quality,
        speed,
        metadata,
        merge_xmp,
        animated,
        lossless,
        parallel,
        error_strategy,
        overwrite,
        progress,
    );
    
    // 执行目录转换
    let converter = BatchConverter::new(config);
    let input_dir = PathBuf::from(input);
    let output_dir = PathBuf::from(output);
    
    let mut collector = FileCollector::new(ext_list).recursive(recursive);
    if let Some(depth) = max_depth {
        collector = collector.max_depth(depth);
    }
    
    let files = collector.collect(&input_dir)?;
    
    logger.log(
        crate::log_manager::LogLevel::Info,
        &format!("Found {} files in directory", files.len())
    );
    
    let result = if use_retry {
        converter.convert_with_retry(files, &output_dir, format, max_retries)?
    } else {
        converter.convert_batch(files, &output_dir, format)?
    };
    
    // 显示结果
    result.log_summary();
    
    Ok(())
}

/// 解析错误策略
fn parse_error_strategy(on_error: &str, max_retries: usize) -> ErrorStrategy {
    match on_error {
        "stop" => ErrorStrategy::StopOnError,
        "retry" => ErrorStrategy::RetryOnError { max_retries },
        _ => ErrorStrategy::ContinueOnError,
    }
}

/// 创建批量转换配置
#[allow(clippy::too_many_arguments)]
fn create_batch_config(
    quality: u8,
    speed: u8,
    metadata: bool,
    merge_xmp: bool,
    animated: bool,
    lossless: bool,
    parallel: Option<usize>,
    error_strategy: ErrorStrategy,
    overwrite: bool,
    progress: bool,
) -> BatchConverterConfig {
    let conversion_config = ConversionConfig {
        quality,
        speed,
        preserve_metadata: metadata,
        keep_animated: animated,
        lossless,
        merge_xmp_sidecar: merge_xmp,
        feature_toggles: None,
        chroma_subsampling: None,
        alpha_quality: None,
        effort: None,
        resize: None,
        quantize: None,
        sharpen: None,
        output_dir: None,
        normalize_filenames: false,
        format_specific_params: None,
    };
    
    BatchConverterConfig {
        conversion_config,
        max_parallel: parallel.unwrap_or_else(num_cpus::get),
        error_strategy,
        overwrite,
        show_progress: progress,
    }
}
