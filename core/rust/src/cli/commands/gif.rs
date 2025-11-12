/**
 * GIF Command - GIF 优化命令处理
 * 从 cli/commands.rs 拆分 (~197行)
 * 
 * 负责:
 * - GIF 文件优化
 * - 预设配置 (web/quality)
 * - 有损/无损压缩
 * - 帧优化和颜色优化
 */

use pixly_converter::converter::gif_optimizer::{GifOptimizer, GifOptimizationConfig, FrameOptimization};
use std::path::PathBuf;
use std::process;

/// 处理 gif-optimize 命令
pub fn handle(args: &[String]) {
    if args.is_empty() {
        print_usage();
        process::exit(1);
    }
    
    let input_path = PathBuf::from(&args[0]);
    if !input_path.exists() {
        eprintln!("❌ Error: Input file not found: {:?}", input_path);
        process::exit(1);
    }
    
    // 解析输出路径
    let output_path = if args.len() > 1 && !args[1].starts_with("--") {
        PathBuf::from(&args[1])
    } else {
        let stem = input_path.file_stem().unwrap().to_str().unwrap();
        input_path.with_file_name(format!("{}_optimized.gif", stem))
    };
    
    println!("🎨 GIF Optimizer");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📥 Input:  {:?}", input_path.file_name().unwrap());
    println!("📤 Output: {:?}", output_path.file_name().unwrap());
    println!("");
    
    // 解析参数
    let (config, use_preset) = parse_arguments(args);
    
    // 应用预设或使用自定义配置
    let optimizer = if let Some(preset) = use_preset {
        println!("🎯 Using preset: {}", preset);
        match preset.as_str() {
            "web" => GifOptimizer::for_web(),
            "quality" => GifOptimizer::for_quality(),
            _ => {
                eprintln!("❌ Error: Unknown preset: {}. Use 'web' or 'quality'", preset);
                process::exit(1);
            }
        }
    } else {
        GifOptimizer::new(config)
    };
    
    // 分析输入 GIF
    analyze_input_gif(&input_path, &optimizer);
    
    // 执行优化
    optimize_gif(&optimizer, &input_path, &output_path);
}

/// 打印使用说明
fn print_usage() {
    eprintln!("❌ Error: Missing input file");
    eprintln!("Usage: pixly-rust gif-optimize <input.gif> [output.gif] [options]");
    eprintln!("");
    eprintln!("Options:");
    eprintln!("  --preset <web|quality>   Use preset configuration");
    eprintln!("  --lossy                  Enable lossy compression");
    eprintln!("  --lossy-quality <0-200>  Lossy compression quality (default: 80)");
    eprintln!("  --color-opt <1-3>        Color optimization level (default: 2)");
    eprintln!("  --frame-opt <none|basic|balanced|aggressive>  Frame optimization");
    eprintln!("  --max-width <pixels>     Maximum width (resize if larger)");
    eprintln!("  --max-fps <fps>          Maximum frame rate");
    eprintln!("  --strip-metadata         Remove metadata (default: true)");
}

/// 解析命令行参数
fn parse_arguments(args: &[String]) -> (GifOptimizationConfig, Option<String>) {
    let mut config = GifOptimizationConfig::default();
    let mut use_preset: Option<String> = None;
    let mut i = if args.len() > 1 && !args[1].starts_with("--") { 2 } else { 1 };
    
    while i < args.len() {
        match args[i].as_str() {
            "--preset" => {
                if i + 1 < args.len() {
                    use_preset = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("❌ Error: Missing value for --preset");
                    process::exit(1);
                }
            }
            "--lossy" => {
                config.lossy_compression = true;
                i += 1;
            }
            "--lossy-quality" => {
                if i + 1 < args.len() {
                    config.lossy_quality = args[i + 1].parse().unwrap_or(80);
                    i += 2;
                } else {
                    eprintln!("❌ Error: Missing value for --lossy-quality");
                    process::exit(1);
                }
            }
            "--color-opt" => {
                if i + 1 < args.len() {
                    config.color_optimization = args[i + 1].parse().unwrap_or(2);
                    i += 2;
                } else {
                    eprintln!("❌ Error: Missing value for --color-opt");
                    process::exit(1);
                }
            }
            "--frame-opt" => {
                if i + 1 < args.len() {
                    config.frame_optimization = match args[i + 1].as_str() {
                        "none" => FrameOptimization::None,
                        "basic" => FrameOptimization::Basic,
                        "balanced" => FrameOptimization::Balanced,
                        "aggressive" => FrameOptimization::Aggressive,
                        _ => FrameOptimization::Balanced,
                    };
                    i += 2;
                } else {
                    eprintln!("❌ Error: Missing value for --frame-opt");
                    process::exit(1);
                }
            }
            "--max-width" => {
                if i + 1 < args.len() {
                    config.max_width = args[i + 1].parse().ok();
                    i += 2;
                } else {
                    eprintln!("❌ Error: Missing value for --max-width");
                    process::exit(1);
                }
            }
            "--max-fps" => {
                if i + 1 < args.len() {
                    config.max_fps = args[i + 1].parse().ok();
                    i += 2;
                } else {
                    eprintln!("❌ Error: Missing value for --max-fps");
                    process::exit(1);
                }
            }
            "--strip-metadata" => {
                config.strip_metadata = true;
                i += 1;
            }
            _ => {
                eprintln!("⚠️  Warning: Unknown option: {}", args[i]);
                i += 1;
            }
        }
    }
    
    (config, use_preset)
}

/// 分析输入 GIF
fn analyze_input_gif(input_path: &PathBuf, optimizer: &GifOptimizer) {
    println!("📊 Analyzing input GIF...");
    match GifOptimizer::analyze(input_path) {
        Ok(info) => {
            println!("   Dimensions: {}x{}", info.width, info.height);
            println!("   Frames: {}", info.frame_count);
            println!("   Duration: {:.2}s", info.duration);
            println!("   FPS: {:.1}", info.fps);
            println!("   Size: {}", info.file_size_readable());
            
            // 估算优化后大小
            let estimated = optimizer.estimate_optimized_size(&info);
            let estimated_mb = estimated as f64 / (1024.0 * 1024.0);
            let reduction = ((info.file_size - estimated) as f64 / info.file_size as f64) * 100.0;
            println!("");
            println!("📈 Estimated optimization:");
            println!("   Target size: ~{:.2} MB", estimated_mb);
            println!("   Reduction: ~{:.1}%", reduction);
            println!("");
        }
        Err(e) => {
            eprintln!("⚠️  Warning: Failed to analyze GIF: {}", e);
            println!("");
        }
    }
}

/// 执行 GIF 优化
fn optimize_gif(optimizer: &GifOptimizer, input_path: &PathBuf, output_path: &PathBuf) {
    println!("🔧 Optimizing GIF...");
    match optimizer.optimize(input_path, output_path) {
        Ok(result) => {
            println!("");
            println!("✅ Optimization complete!");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("   Original:  {:.2} MB", result.original_size as f64 / (1024.0 * 1024.0));
            println!("   Optimized: {:.2} MB", result.optimized_size as f64 / (1024.0 * 1024.0));
            
            if result.is_improved() {
                println!("   Saved:     {:.2} MB ({:.1}%)", 
                    result.bytes_saved() as f64 / (1024.0 * 1024.0),
                    result.reduction_percent);
            } else {
                println!("   ℹ️  No size reduction (output >= input)");
            }
            
            println!("   Time:      {:.2}s", result.elapsed_ms as f64 / 1000.0);
            println!("");
            println!("📤 Output: {}", output_path.display());
        }
        Err(e) => {
            eprintln!("");
            eprintln!("❌ Optimization failed: {}", e);
            eprintln!("");
            eprintln!("💡 Troubleshooting:");
            eprintln!("   - Make sure 'gifsicle' is installed (brew install gifsicle)");
            eprintln!("   - Make sure 'ffmpeg' is installed if using --max-width or --max-fps");
            eprintln!("   - Check that the input file is a valid GIF");
            process::exit(1);
        }
    }
}
