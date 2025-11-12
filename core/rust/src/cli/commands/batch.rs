/**
 * Batch Command - 批量转换命令处理
 * 从cli/commands.rs拆分 (~240行)
 * 
 * 负责:
 * - 批量文件收集
 * - 并行转换处理
 * - 进度显示
 * - 统计报告
 */

use std::path::PathBuf;
use std::process;
use std::io::Write;
use pixly_converter::converter::PredictionData;
use pixly_converter::converter::ai_client::{AIClient, PredictionRequest};
use pixly_converter::converter::strategy::{init_global_manager, StrategyType, ConversionConfig as StrategyConfig};
use pixly_converter::converter::register_all_strategies;  // Phase 46.12: 修正导入路径
use image;

/// 处理batch命令
pub fn handle(args: &[String]) {
    if args.len() < 3 {
        eprintln!("❌ Error: Missing arguments for batch");
        eprintln!("   Usage: pixly-rust batch <input_dir> <output_dir> <format>");
        process::exit(1);
    }

    let input_dir = PathBuf::from(&args[0]);
    let output_dir = PathBuf::from(&args[1]);
    let format = args[2].to_lowercase();

    // 解析选项
    let options = parse_options(args);
    
    println!("🔄 Batch Conversion:");
    println!("   Input:  {}", input_dir.display());
    println!("   Output: {}", output_dir.display());
    println!("   Format: {}", format);
    println!("   Quality: {}, Speed: {}", options.quality, options.speed);
    println!();

    // 收集输入文件
    let input_files = collect_input_files(&input_dir);
    if input_files.is_empty() {
        println!("⚠️  No image files found in {}", input_dir.display());
        return;
    }
    
    println!("📋 Found {} images to convert\n", input_files.len());
    
    // 创建输出目录
    ensure_output_dir(&output_dir);
    
    // 执行批量转换
    perform_batch_conversion(&input_files, &output_dir, &format, &options);
}

/// 批量转换选项
struct BatchOptions {
    quality: u8,
    speed: u8,
    preserve_metadata: bool,
    keep_animated: bool,
    lossless: bool,
}

impl Default for BatchOptions {
    fn default() -> Self {
        Self {
            quality: 85,
            speed: 4,
            preserve_metadata: true,
            keep_animated: true,
            lossless: false,
        }
    }
}

/// 解析命令行选项
fn parse_options(args: &[String]) -> BatchOptions {
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
            _ => {
                i += 1;
            }
        }
    }
    
    options
}

/// 收集输入文件
fn collect_input_files(input_dir: &PathBuf) -> Vec<PathBuf> {
    let mut input_files = Vec::new();
    
    if input_dir.is_file() {
        input_files.push(input_dir.clone());
    } else if input_dir.is_dir() {
        match std::fs::read_dir(input_dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            let ext_str = ext.to_string_lossy().to_lowercase();
                            if matches!(ext_str.as_str(), 
                                "jpg" | "jpeg" | "png" | "webp" | "gif" | "bmp" | "tiff") {
                                input_files.push(path);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to read input directory: {}", e);
                process::exit(1);
            }
        }
    }
    
    input_files
}

/// 确保输出目录存在
fn ensure_output_dir(output_dir: &PathBuf) {
    if !output_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(output_dir) {
            eprintln!("❌ Failed to create output directory: {}", e);
            process::exit(1);
        }
    }
}

/// 执行批量转换
fn perform_batch_conversion(
    input_files: &[PathBuf],
    output_dir: &PathBuf,
    format: &str,
    options: &BatchOptions,
) {
    // 初始化策略管理器
    let manager_lock = init_global_manager();
    let mut manager = manager_lock.lock().expect("Failed to lock strategy manager");
    
    // 注册所有策略
    if manager.available_strategies().is_empty() {
        register_all_strategies(&mut manager);
    }
    
    // 🔥 Phase 47.16: 正面解决AI服务问题后恢复AI功能
    let ai_client = AIClient::new(Default::default());
    let ai_available = ai_client.is_available();
    if !ai_available {
        println!("\n⚠️  AI service not available, proceeding without AI predictions");
        println!("   (Start Python AI service for better optimization: python3 tools/pixly_http_server.py)\n");
    } else {
        println!("\n✅ AI service available, using AI optimization\n");
    }
    
    // 🔥 Phase 47.19 (O-003): 使用并行处理优化性能
    let start_time = std::time::Instant::now();
    
    // 无损模式下强制quality=100
    let final_quality = if options.lossless { 100 } else { options.quality };
    
    // 使用rayon并行处理
    use rayon::prelude::*;
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    let successful = Arc::new(AtomicUsize::new(0));
    let failed = Arc::new(AtomicUsize::new(0));
    let failures = Arc::new(Mutex::new(Vec::new()));
    let total = input_files.len();
    
    // 进度显示
    let completed = Arc::new(AtomicUsize::new(0));
    
    println!("\n🚀 Starting parallel batch conversion ({} files)...", total);
    println!("   Using {} CPU cores", num_cpus::get());
    
    // 并行处理所有文件
    input_files.par_iter().for_each(|input_path| {
        let filename = input_path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        
        let output_filename = format!("{}.{}", filename, format);
        let output_path = output_dir.join(output_filename);
        
        // 显示进度
        let current = completed.fetch_add(1, Ordering::SeqCst) + 1;
        println!("[{}/{}] Converting: {}", current, total, filename);
        
        // AI预测 - 每个线程独立获取预测
        let prediction_data = if ai_available {
            // 克隆AI客户端供并行使用
            let thread_ai_client = AIClient::new(Default::default());
            get_ai_prediction(input_path, format, final_quality, options, &thread_ai_client)
        } else {
            None
        };
        
        // 创建转换配置
        let strategy_config = StrategyConfig {
            quality: final_quality,
            speed: options.speed,
            preserve_metadata: options.preserve_metadata,
            keep_animated: options.keep_animated,
            strategy: StrategyType::Auto,
            lossless: options.lossless,
            normalize_filenames: None,
            prediction_data,
        };
        
        // 获取线程本地的管理器
        let thread_manager_lock = init_global_manager();
        let mut thread_manager = thread_manager_lock.lock().expect("Failed to lock strategy manager");
        if thread_manager.available_strategies().is_empty() {
            register_all_strategies(&mut thread_manager);
        }
        
        // 执行转换
        match thread_manager.convert(input_path, &output_path, format, &strategy_config) {
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
    
    let successful = successful.load(Ordering::SeqCst);
    let failed = failed.load(Ordering::SeqCst);
    let failures = failures.lock().unwrap().clone();
    
    drop(manager);
    
    let elapsed = start_time.elapsed();
    
    // 打印统计信息
    print_statistics(successful, failed, &failures, elapsed);
}

/// 获取AI预测数据
fn get_ai_prediction(
    input_path: &PathBuf,
    format: &str,
    quality: u8,
    options: &BatchOptions,
    ai_client: &AIClient,
) -> Option<PredictionData> {
    let input_ext = input_path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    // 获取图像尺寸
    let (width, height) = if let Ok(img) = image::open(input_path) {
        (img.width(), img.height())
    } else {
        (1920, 1080)
    };
    
    let file_size = std::fs::metadata(input_path)
        .map(|m| m.len())
        .unwrap_or(0);
    
    let prediction_request = PredictionRequest {
        image_path: Some(input_path.to_string_lossy().to_string()),
        input_format: input_ext,
        target_format: format.to_string(),
        width,
        height,
        file_size,
        has_alpha: false,
        has_animation: false,
        color_space: None,
        bit_depth: None,
        priority: "balanced".to_string(),
        preserve_quality: true,
        model_type: None,
        request_id: None,
        enable_bayesian: Some(true),
        enable_ppo: Some(true),
        enable_smart_quality: Some(true),
        enable_auto_optimize: Some(true),
        enable_video_for_anim: Some(true),
    };
    
    match ai_client.predict(&prediction_request) {
        Ok(response) => Some(PredictionData {
            format: format.to_string(),
            quality: response.quality.unwrap_or(quality),
            speed: response.speed.unwrap_or(options.speed),
            lossless: response.lossless.unwrap_or(options.lossless),
            predicted_size: None,
            confidence: Some(response.confidence),
            preprocessing_steps: response.preprocessing_steps.clone(),
            optimization_path: response.optimization_path.clone(),
        }),
        Err(_) => None,
    }
}

/// 打印统计信息
fn print_statistics(
    successful: usize,
    failed: usize,
    failures: &[(String, String)],
    elapsed: std::time::Duration,
) {
    println!("\n\n✨ Batch Conversion Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   ✅ Successful: {}", successful);
    println!("   ❌ Failed: {}", failed);
    println!("   ⏱️  Time: {:.2}s", elapsed.as_secs_f64());
    
    if successful > 0 {
        let avg_time = elapsed.as_secs_f64() / successful as f64;
        println!("   📊 Average: {:.2}s per image", avg_time);
    }
    
    if !failures.is_empty() {
        println!("\n❌ Failures:");
        for (filename, error) in failures {
            println!("   - {}: {}", filename, error);
        }
    }
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
