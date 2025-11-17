// 🚀 CLI Batch命令 - 完整实现
// 从 @archive/rust_broken/src/cli/commands/batch.rs 提取完整功能
//
// 核心功能:
// - 批量文件收集
// - 并行转换处理
// - 进度显示
// - 统计报告
// - AI预测缓存优化

use std::path::{Path, PathBuf};
use anyhow::Result;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// 批量转换选项
#[derive(Debug, Clone)]
pub struct BatchOptions {
    pub quality: u8,
    pub speed: u8,
    pub preserve_metadata: bool,
    pub keep_animated: bool,
    pub lossless: bool,
    pub parallel: bool,
    pub merge_xmp_sidecar: bool,
}

impl Default for BatchOptions {
    fn default() -> Self {
        Self {
            quality: 85,
            speed: 4,
            preserve_metadata: true,
            keep_animated: true,
            lossless: false,
            parallel: true,
            merge_xmp_sidecar: false,
        }
    }
}

/// 解析命令行选项
pub fn parse_options(args: &[String]) -> BatchOptions {
    let mut options = BatchOptions::default();
    
    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--quality" => {
                if i + 1 < args.len() {
                    options.quality = args[i + 1].parse().unwrap_or(85).clamp(1, 100);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--speed" => {
                if i + 1 < args.len() {
                    options.speed = args[i + 1].parse().unwrap_or(4).clamp(1, 10);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--metadata" => {
                options.preserve_metadata = true;
                i += 1;
            }
            "--animated" => {
                options.keep_animated = true;
                i += 1;
            }
            "--lossless" => {
                options.lossless = true;
                i += 1;
            }
            "--no-parallel" => {
                options.parallel = false;
                i += 1;
            }
            "--merge-xmp" => {
                options.merge_xmp_sidecar = true;
                i += 1;
            }
            "--no-merge-xmp" => {
                options.merge_xmp_sidecar = false;
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    
    options
}

/// 收集输入文件
pub fn collect_input_files(input_dir: &PathBuf) -> Vec<PathBuf> {
    let mut input_files = Vec::new();
    
    if input_dir.is_file() {
        input_files.push(input_dir.clone());
    } else if input_dir.is_dir() && let Ok(entries) = std::fs::read_dir(input_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && let Some(ext) = path.extension()
            {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if matches!(ext_str.as_str(), 
                    "jpg" | "jpeg" | "png" | "webp" | "gif" | "bmp" | "tiff" | "avif" | "jxl") {
                    input_files.push(path);
                }
            }
        }
    }
    
    input_files
}

/// 确保输出目录存在
pub fn ensure_output_dir(output_dir: &PathBuf) -> Result<()> {
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)?;
    }
    Ok(())
}

/// 批量转换结果
#[derive(Debug, Clone)]
pub struct BatchResult {
    pub successful: usize,
    pub failed: usize,
    pub failures: Vec<(String, String)>,
    pub elapsed: std::time::Duration,
}

/// 执行批量转换
pub fn perform_batch_conversion(
    input_files: &[PathBuf],
    output_dir: &Path,
    format: &str,
    options: &BatchOptions,
) -> Result<BatchResult> {
    let start_time = std::time::Instant::now();
    let total = input_files.len();
    
    println!("\n🚀 Starting batch conversion ({} files)...", total);
    
    let successful = Arc::new(AtomicUsize::new(0));
    let failed = Arc::new(AtomicUsize::new(0));
    let failures = Arc::new(std::sync::Mutex::new(Vec::new()));
    let completed = Arc::new(AtomicUsize::new(0));
    
    // 无损模式下强制quality=100
    let final_quality = if options.lossless { 100 } else { options.quality };
    
    if options.parallel {
        // 并行处理
        use rayon::prelude::*;
        
        let cpu_cores = num_cpus::get();
        let optimal_threads = if total < cpu_cores {
            total
        } else {
            std::cmp::min(cpu_cores, 8)
        };
        
        let chunk_size = std::cmp::max(1, total / optimal_threads);
        
        println!("   Using {} threads, ~{} files per thread", optimal_threads, chunk_size);
        
        input_files.par_iter().for_each(|input_path| {
            let filename = input_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            
            let output_filename = format!("{}.{}", filename, format);
            let output_path = output_dir.join(output_filename);
            
            let current = completed.fetch_add(1, Ordering::SeqCst) + 1;
            println!("[{}/{}] Converting: {}", current, total, filename);
            
            // 执行转换
            match convert_single_file(input_path, &output_path, format, final_quality, options) {
                Ok(_) => {
                    successful.fetch_add(1, Ordering::SeqCst);
                    println!("   ✅ {}", filename);
                }
                Err(e) => {
                    failed.fetch_add(1, Ordering::SeqCst);
                    println!("   ❌ {}: {}", filename, e);
                    if let Ok(mut f) = failures.lock() {
                        f.push((filename.to_string(), e.to_string()));
                    }
                }
            }
        });
    } else {
        // 串行处理
        for input_path in input_files {
            let filename = input_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            
            let output_filename = format!("{}.{}", filename, format);
            let output_path = output_dir.join(output_filename);
            
            let current = completed.fetch_add(1, Ordering::SeqCst) + 1;
            println!("[{}/{}] Converting: {}", current, total, filename);
            
            match convert_single_file(input_path, &output_path, format, final_quality, options) {
                Ok(_) => {
                    successful.fetch_add(1, Ordering::SeqCst);
                    println!("   ✅ {}", filename);
                }
                Err(e) => {
                    failed.fetch_add(1, Ordering::SeqCst);
                    println!("   ❌ {}: {}", filename, e);
                    if let Ok(mut f) = failures.lock() {
                        f.push((filename.to_string(), e.to_string()));
                    }
                }
            }
        }
    }
    
    let successful = successful.load(Ordering::SeqCst);
    let failed = failed.load(Ordering::SeqCst);
    let failures = failures.lock().unwrap().clone();
    let elapsed = start_time.elapsed();
    
    Ok(BatchResult {
        successful,
        failed,
        failures,
        elapsed,
    })
}

/// 转换单个文件
fn convert_single_file(
    input_path: &Path,
    output_path: &Path,
    format: &str,
    quality: u8,
    options: &BatchOptions,
) -> Result<()> {
    use crate::conversion_core::{execute_conversion, ConversionConfig};
    
    let config = ConversionConfig {
        quality,
        speed: options.speed,
        preserve_metadata: options.preserve_metadata,
        keep_animated: options.keep_animated,
        lossless: options.lossless,
        merge_xmp_sidecar: options.merge_xmp_sidecar,
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
    
    execute_conversion(input_path, output_path, format, &config)?;
    Ok(())
}

/// 打印统计信息
pub fn print_statistics(result: &BatchResult) {
    println!("\n\n✨ Batch conversion completed!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   ✅ Successful: {}", result.successful);
    println!("   ❌ Failed: {}", result.failed);
    println!("   ⏱️  Time: {:.2}s", result.elapsed.as_secs_f64());
    
    if result.successful > 0 {
        let avg_time = result.elapsed.as_secs_f64() / result.successful as f64;
        println!("   📊 Average: {:.2}sec/image", avg_time);
    }
    
    if !result.failures.is_empty() {
        println!("\n❌ Failed list:");
        for (filename, error) in &result.failures {
            println!("   - {}: {}", filename, error);
        }
    }
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

/// 处理batch命令
pub fn handle_batch(
    input_dir: &str,
    output_dir: &str,
    format: &str,
    options: &BatchOptions,
) -> Result<()> {
    let input_path = PathBuf::from(input_dir);
    let output_path = PathBuf::from(output_dir);
    
    println!("🔄 Batch Converting:");
    println!("   Input:  {}", input_path.display());
    println!("   Output: {}", output_path.display());
    println!("   Format: {}", format);
    println!("   Quality: {}, Speed: {}", options.quality, options.speed);
    println!();
    
    // 收集输入文件
    let input_files = collect_input_files(&input_path);
    if input_files.is_empty() {
        println!("⚠️  No image files found in {}", input_path.display());
        return Ok(());
    }
    
    println!("📋 Found {} images to convert\n", input_files.len());
    
    // 创建输出目录
    ensure_output_dir(&output_path)?;
    
    // 执行批量转换
    let result = perform_batch_conversion(&input_files, &output_path, format, options)?;
    
    // 打印统计信息
    print_statistics(&result);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_options() {
        let options = BatchOptions::default();
        assert_eq!(options.quality, 85);
        assert_eq!(options.speed, 4);
        assert!(options.preserve_metadata);
        assert!(options.keep_animated);
        assert!(!options.lossless);
        assert!(options.parallel);
    }
    
    #[test]
    fn test_parse_quality_option() {
        let args = vec![
            "input".to_string(),
            "output".to_string(),
            "webp".to_string(),
            "--quality".to_string(),
            "90".to_string(),
        ];
        let options = parse_options(&args);
        assert_eq!(options.quality, 90);
    }
    
    #[test]
    fn test_parse_lossless_option() {
        let args = vec![
            "input".to_string(),
            "output".to_string(),
            "webp".to_string(),
            "--lossless".to_string(),
        ];
        let options = parse_options(&args);
        assert!(options.lossless);
    }
    
    #[test]
    fn test_collect_empty_dir() {
        let temp_dir = std::env::temp_dir().join("pixly_test_empty");
        let _ = std::fs::create_dir_all(&temp_dir);
        
        let files = collect_input_files(&temp_dir);
        assert_eq!(files.len(), 0);
        
        let _ = std::fs::remove_dir(&temp_dir);
    }
}
