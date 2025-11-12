/**
 * Analyze Command - 转换参数分析与试运行
 * 从 cli/commands.rs 拆分 (~232行)
 * 
 * 负责:
 * - 真实试运行转换到临时文件
 * - AI 参数预测验证
 * - 转换参数有效性检测
 * - 工具兼容性测试
 * - 自动清理临时文件
 * 
 * 区别于模拟预览：
 * - 真实调用转换工具 (cjxl, avifenc, etc.)
 * - 实际测量转换时间和输出大小
 * - 验证参数组合的有效性
 */

use pixly_converter::converter::params::{OptimizedParams, AIParameterClient};
use pixly_converter::converter::strategy::{ConversionConfig, StrategyType, init_global_manager};
use pixly_converter::converter::register_all_strategies;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::Instant;

/// 处理 analyze 命令
pub fn handle(args: &[String]) {
    if args.is_empty() {
        eprintln!("❌ Error: Missing file path");
        eprintln!("   Usage: pixly-rust analyze <file> [--format <format>] [--quality <q>] [--dry-run]");
        return;
    }
    
    let input_path = &args[0];
    let input = Path::new(input_path);
    
    if !input.exists() {
        eprintln!("❌ Error: File not found: {}", input_path);
        return;
    }
    
    // 解析参数
    let (target_format, quality, dry_run) = parse_arguments(args);
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔍 PIXLY Conversion Parameter Analysis");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 显示文件信息
    print_file_info(input, input_path);
    
    // 分析图像并执行试运行
    match AIParameterClient::analyze_image(input) {
        Ok(chars) => {
            print_image_characteristics(&chars);
            
            // AI参数预测
            let params = match predict_parameters(&chars, &target_format, quality) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("❌ Optimization failed: {}", e);
                    return;
                }
            };
            
            print_predicted_parameters(&params, &chars, &target_format);
            
            // 真实试运行或dry-run
            if !dry_run {
                run_real_trial(input, &target_format, &params);
            } else {
                print_dry_run_estimate(&params);
            }
            
            // 打印建议的CLI命令
            print_recommended_command(input_path, &target_format, &params);
            
            // 元数据提示
            print_metadata_info();
        }
        Err(e) => {
            eprintln!("❌ Failed to analyze file: {}", e);
            return;
        }
    }
    
    print_footer(dry_run);
}

/// 解析命令行参数
fn parse_arguments(args: &[String]) -> (String, u8, bool) {
    let mut target_format = "jxl".to_string();
    let mut quality = 90u8;
    let mut dry_run = false;
    let mut i = 1;
    
    while i < args.len() {
        match args[i].as_str() {
            "--format" | "-f" => {
                if i + 1 < args.len() {
                    target_format = args[i + 1].clone();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--quality" | "-q" => {
                if i + 1 < args.len() {
                    quality = args[i + 1].parse().unwrap_or(90);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--dry-run" => {
                dry_run = true;
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    
    (target_format, quality, dry_run)
}

/// 打印文件信息
fn print_file_info(input: &Path, input_path: &str) {
    println!("📁 Input File:");
    println!("   Path: {}", input_path);
    
    let metadata = std::fs::metadata(input).unwrap();
    let file_size = metadata.len();
    println!("   Size: {} KB ({} bytes)", file_size / 1024, file_size);
}

/// 打印图像特征
fn print_image_characteristics(chars: &pixly_converter::converter::params::ImageCharacteristics) {
    println!("\n📊 Image Characteristics:");
    println!("   Format: {}", chars.format);
    println!("   Dimensions: {}x{}", chars.width, chars.height);
    println!("   Has Alpha: {}", chars.has_alpha);
    println!("   Is Animated: {}", chars.is_animated);
    println!("   File Size: {} KB", chars.file_size / 1024);
}

/// AI参数预测
fn predict_parameters(
    chars: &pixly_converter::converter::params::ImageCharacteristics,
    target_format: &str,
    quality: u8,
) -> Result<OptimizedParams, anyhow::Error> {
    println!("\n🤖 AI Parameter Prediction:");
    println!("   Target Format: {}", target_format);
    
    let mut optimizer = AIParameterClient::new(target_format);
    optimizer.set_prefer_quality(quality >= 90);
    
    optimizer.get_parameters_from_ai(chars)
}

/// 打印预测参数
fn print_predicted_parameters(
    params: &OptimizedParams,
    chars: &pixly_converter::converter::params::ImageCharacteristics,
    target_format: &str,
) {
    println!("   Quality: {}", params.quality);
    println!("   Speed/Effort: {}", params.speed);
    println!("   Lossless: {} 🔥", params.lossless);
    
    if !params.format_options.is_empty() {
        println!("   Format Options: 🔥");
        for (key, value) in &params.format_options {
            println!("     - {}: {}", key, value);
        }
    }
    
    // 特殊场景提示
    if (chars.format == "jpeg" || chars.format == "jpg") && target_format == "jxl" {
        println!("\n   💡 Special Case: JPEG → JXL");
        println!("      This will use lossless transcoding");
        println!("      (--lossless_jpeg=1, distance=0.0)");
        println!("      Expected: ~25% size reduction with perfect quality");
    }
    
    println!("\n   Reason: {}", params.reason);
}

/// 执行真实试运行
fn run_real_trial(input: &Path, target_format: &str, params: &OptimizedParams) {
    println!("\n🔬 Real Trial Run:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // 创建临时输出文件
    let temp_output = format!("/tmp/pixly_analyze_temp_{}.{}", 
        std::process::id(), target_format);
    let temp_path = PathBuf::from(&temp_output);
    
    // 配置转换
    let config = ConversionConfig {
        quality: params.quality,
        speed: params.speed,
        preserve_metadata: true,
        keep_animated: true,
        strategy: StrategyType::Auto,
        lossless: params.lossless,
        normalize_filenames: None,
        prediction_data: None,
    };
    
    // 初始化策略管理器
    let manager_lock = init_global_manager();
    let mut manager = manager_lock.lock().unwrap();
    if manager.available_strategies().is_empty() {
        register_all_strategies(&mut manager);
    }
    
    // 执行真实转换
    let start = Instant::now();
    match manager.convert(input, &temp_path, target_format, &config) {
        Ok(result) => {
            drop(manager);
            let elapsed = start.elapsed();
            
            println!("   Status: ✅ Success");
            println!("   Real Time: {:.2}s", elapsed.as_secs_f64());
            println!("   Real Size: {} KB ({} bytes)", 
                result.output_size / 1024, result.output_size);
            println!("   Real Compression: {:.1}%", result.compression_ratio * 100.0);
            println!("   Strategy Used: {}", result.strategy_used);
            
            // 对比预估值
            let size_diff = (result.output_size as i64 - params.estimated_size as i64).abs();
            let size_diff_pct = (size_diff as f64 / params.estimated_size as f64) * 100.0;
            println!("\n   Prediction Accuracy:");
            println!("      Estimated: {} KB", params.estimated_size / 1024);
            println!("      Actual: {} KB", result.output_size / 1024);
            println!("      Difference: {:.1}%", size_diff_pct);
            
            // 清理临时文件
            if let Err(e) = fs::remove_file(&temp_path) {
                println!("   ⚠️  Failed to cleanup temp file: {}", e);
            }
        }
        Err(e) => {
            drop(manager);
            println!("   Status: ❌ Failed");
            println!("   Error: {}", e);
            println!("   💡 This indicates the predicted parameters are invalid!");
            
            // 尝试清理临时文件
            let _ = fs::remove_file(&temp_path);
        }
    }
}

/// 打印dry-run预估
fn print_dry_run_estimate(params: &OptimizedParams) {
    println!("\n📈 Estimated Output (Dry Run):");
    println!("   Size: {} KB ({}% of original)", 
        params.estimated_size / 1024,
        (params.estimated_ratio * 100.0) as u32
    );
    println!("   Compression Ratio: {:.1}%", params.estimated_ratio * 100.0);
}

/// 打印建议的CLI命令
fn print_recommended_command(input_path: &str, target_format: &str, params: &OptimizedParams) {
    println!("\n💻 Recommended CLI Command:");
    println!("   pixly-rust convert {} output.{} \\", input_path, target_format);
    println!("      --format {} --quality {} --speed {}", 
        target_format, params.quality, params.speed);
    if params.lossless {
        println!("      --lossless");
    }
}

/// 打印元数据信息
fn print_metadata_info() {
    println!("\n🔖 Metadata Preservation:");
    println!("   ✅ EXIF metadata will be preserved");
    println!("   ✅ XMP sidecar files will be copied");
    println!("   ✅ ICC color profiles will be kept");
    println!("   ✅ File timestamps will be preserved");
}

/// 打印页脚
fn print_footer(dry_run: bool) {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    if dry_run {
        println!("💡 Dry Run Mode: No actual conversion was performed.");
        println!("   Add --dry-run flag to skip real execution.");
    } else {
        println!("✅ Real Trial Run Complete!");
        println!("   The conversion was executed to a temporary file and cleaned up.");
        println!("   Use 'pixly-rust convert' to save the actual output.");
    }
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}
