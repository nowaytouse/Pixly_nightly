/**
 * Eagle Command - Eagle图库批量优化
 * 从 cli/commands.rs 拆分 (~167行)
 * 
 * 负责:
 * - Eagle库扫描
 * - 批量图像优化
 * - 并发处理
 * - 进度统计报告
 */

use pixly_converter::converter::eagle_adapter::EagleAdapter;
use std::process;
use std::time::Instant;

/// 处理 eagle 命令
pub fn handle(args: &[String]) {
    if args.is_empty() {
        print_usage();
        process::exit(1);
    }

    let subcommand = &args[0];
    
    match subcommand.as_str() {
        "scan" => handle_scan(&args[1..]),
        "optimize" => handle_optimize(&args[1..]),
        _ => {
            eprintln!("❌ Error: Unknown Eagle subcommand: {}", subcommand);
            process::exit(1);
        }
    }
}

/// 打印使用说明
fn print_usage() {
    eprintln!("❌ Error: Missing Eagle command");
    eprintln!("   Usage: pixly-rust eagle <subcommand> [options]");
    eprintln!("");
    eprintln!("Subcommands:");
    eprintln!("  scan <library_path>           - 扫描Eagle库");
    eprintln!("  optimize <library_path> <format> - 批量优化Eagle库");
    eprintln!("");
    eprintln!("Options for 'optimize':");
    eprintln!("  --quality <1-100>             - 质量参数 (default: 85)");
    eprintln!("  --dry-run                     - 仅测试，不实际转换");
}

/// 处理Eagle扫描命令
fn handle_scan(args: &[String]) {
    if args.is_empty() {
        eprintln!("❌ Error: Missing library path");
        eprintln!("   Usage: pixly-rust eagle scan <library_path>");
        process::exit(1);
    }

    let library_path = &args[0];
    
    println!("🔍 扫描Eagle库: {}", library_path);
    
    let adapter = EagleAdapter::new(library_path);
    
    match adapter.scan_library() {
        Ok(image_dirs) => {
            println!("✅ 扫描完成!");
            println!("📊 统计:");
            println!("   - 总图像数: {}", image_dirs.len());
            println!("");
            println!("💡 使用 'pixly-rust eagle optimize' 开始批量优化");
        }
        Err(e) => {
            eprintln!("❌ 扫描失败: {}", e);
            process::exit(1);
        }
    }
}

/// 处理Eagle批量优化命令
fn handle_optimize(args: &[String]) {
    if args.len() < 2 {
        eprintln!("❌ Error: Missing arguments");
        eprintln!("   Usage: pixly-rust eagle optimize <library_path> <format> [options]");
        eprintln!("");
        eprintln!("Formats: avif, webp, jxl, png, jpg");
        eprintln!("");
        eprintln!("Options:");
        eprintln!("  --quality <1-100>  Quality parameter (default: 85)");
        eprintln!("  --threads <N>      Number of threads (default: auto, 0=auto)");
        eprintln!("  --dry-run          Test mode, no actual conversion");
        process::exit(1);
    }

    let library_path = &args[0];
    let format = &args[1];
    
    // 解析选项
    let (quality, threads, dry_run) = parse_optimize_options(args);
    
    println!("🚀 Eagle批量优化");
    println!("   库路径: {}", library_path);
    println!("   目标格式: {}", format.to_uppercase());
    println!("   质量: {}", quality);
    println!("   并发线程: {}", if threads == 0 { 
        format!("auto ({})", num_cpus::get()) 
    } else { 
        threads.to_string() 
    });
    if dry_run {
        println!("   模式: DRY RUN（测试）");
    }
    println!("");
    
    let adapter = EagleAdapter::new(library_path);
    let start_time = Instant::now();
    
    match adapter.batch_optimize_with_concurrency(format, quality, dry_run, threads) {
        Ok(report) => {
            print_optimization_report(&report, start_time.elapsed().as_secs_f64(), dry_run);
        }
        Err(e) => {
            eprintln!("❌ 批量优化失败: {}", e);
            process::exit(1);
        }
    }
}

/// 解析优化选项
fn parse_optimize_options(args: &[String]) -> (u8, usize, bool) {
    let mut quality = 85u8;
    let mut dry_run = false;
    let mut threads = 0usize; // 0 = auto-detect
    
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--quality" => {
                if i + 1 < args.len() {
                    quality = args[i + 1].parse().unwrap_or(85).clamp(1, 100);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--threads" => {
                if i + 1 < args.len() {
                    threads = args[i + 1].parse().unwrap_or(0);
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
    
    (quality, threads, dry_run)
}

/// 打印优化报告
fn print_optimization_report(
    report: &pixly_converter::converter::eagle_adapter::BatchOptimizeReport,
    duration: f64,
    dry_run: bool,
) {
    println!("");
    println!("════════════════════════════════════════");
    println!("✅ 批量优化完成!");
    println!("════════════════════════════════════════");
    println!("📊 统计:");
    println!("   - 总图像数: {}", report.total);
    println!("   - 已处理: {}", report.processed);
    println!("   - 跳过: {} (已是目标格式)", report.skipped);
    println!("   - 失败: {}", report.failed);
    if !dry_run {
        println!("   - 节省空间: {:.2} MB", report.saved_bytes as f64 / 1024.0 / 1024.0);
    }
    println!("   - 耗时: {:.2}s", duration);
    
    if !report.errors.is_empty() {
        println!("");
        println!("⚠️  错误列表:");
        for (i, err) in report.errors.iter().enumerate().take(10) {
            println!("   {}. {:?}", i + 1, err.path.file_name().unwrap_or_default());
            println!("      {}", err.error);
        }
        if report.errors.len() > 10 {
            println!("   ... 还有 {} 个错误", report.errors.len() - 10);
        }
    }
}
