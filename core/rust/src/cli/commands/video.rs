/**
 * Video Command - 视频转换命令处理
 * 从 cli/commands.rs 拆分 (~254行)
 * 
 * 负责:
 * - 视频格式转换 (H.264/H.265/AV1)
 * - 编码参数配置
 * - AI 参数预测
 * - Eagle 集成（动图转视频原地替换）
 */

use pixly_converter::converter::video_processor::{VideoProcessor, VideoConversionConfig, AudioMode};
use std::process;

/// 处理 video 命令
pub fn handle(args: &[String]) {
    if args.len() < 2 {
        eprintln!("❌ Error: video command requires input and output paths");
        eprintln!("Usage: pixly-rust video <input> <output> [options]");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --codec <codec>      编码器 (h264, h265, av1)");
        eprintln!("  --crf <value>        CRF质量 (0-51)");
        eprintln!("  --preset <preset>    编码预设 (ultrafast, fast, medium, slow, veryslow)");
        eprintln!("  --container <format> 容器格式 (mp4, mov, webm, mkv)");
        eprintln!("  --ai                 使用AI预测参数");
        eprintln!("  --width <pixels>     目标宽度");
        eprintln!("  --height <pixels>    目标高度");
        process::exit(1);
    }
    
    let input = &args[0];
    let output = &args[1];
    
    // 解析参数
    let mut codec = String::from("h264");
    let mut crf: Option<u8> = None;
    let mut preset = String::from("medium");
    let mut container = String::from("mp4");
    let mut use_ai = false;
    let mut target_width: Option<u32> = None;
    let mut target_height: Option<u32> = None;
    
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--codec" => {
                if i + 1 < args.len() {
                    codec = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("❌ Error: --codec requires a value");
                    process::exit(1);
                }
            }
            "--crf" => {
                if i + 1 < args.len() {
                    crf = Some(args[i + 1].parse().expect("Invalid CRF value"));
                    i += 2;
                } else {
                    eprintln!("❌ Error: --crf requires a value");
                    process::exit(1);
                }
            }
            "--preset" => {
                if i + 1 < args.len() {
                    preset = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("❌ Error: --preset requires a value");
                    process::exit(1);
                }
            }
            "--container" => {
                if i + 1 < args.len() {
                    container = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("❌ Error: --container requires a value");
                    process::exit(1);
                }
            }
            "--ai" => {
                use_ai = true;
                i += 1;
            }
            "--width" => {
                if i + 1 < args.len() {
                    target_width = Some(args[i + 1].parse().expect("Invalid width"));
                    i += 2;
                } else {
                    eprintln!("❌ Error: --width requires a value");
                    process::exit(1);
                }
            }
            "--height" => {
                if i + 1 < args.len() {
                    target_height = Some(args[i + 1].parse().expect("Invalid height"));
                    i += 2;
                } else {
                    eprintln!("❌ Error: --height requires a value");
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("❌ Unknown option: {}", args[i]);
                process::exit(1);
            }
        }
    }
    
    // 🔥 AI集成：预测最优参数
    if use_ai {
        println!("🤖 AI prediction: enabled");
        println!("⚠️ AI video prediction will be called during conversion");
        // 注意：实际预测会在video_processor内部进行
    }
    
    // 设置默认CRF
    let final_crf = crf.unwrap_or(23);
    
    // 构建配置
    let target_resolution = if target_width.is_some() && target_height.is_some() {
        Some((target_width.unwrap(), target_height.unwrap()))
    } else {
        None
    };
    
    let config = VideoConversionConfig {
        codec: codec.clone(),
        container: container.clone(),
        crf: final_crf,
        preset: preset.clone(),
        target_resolution,
        target_fps: None,
        audio_mode: AudioMode::Copy,
        two_pass: false,
        hw_accel: String::from("auto"),
    };
    
    println!("🎬 Converting video...");
    println!("   Input: {}", input);
    println!("   Output: {}", output);
    println!("   Codec: {}", codec);
    println!("   CRF: {}", final_crf);
    println!("   Preset: {}", preset);
    println!("   Container: {}", container);
    
    // 🔥 Phase 46.1: 检测动图输入（用于原地替换）
    let input_path = std::path::Path::new(input);
    let input_ext = input_path.extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    let is_animated_image = matches!(input_ext.as_str(), "gif" | "apng" | "webp");
    
    // 执行转换
    let processor = VideoProcessor::new();
    match processor.convert_video(
        input_path,
        std::path::Path::new(output),
        &config,
        Some(|progress| {
            print!("\r   Progress: {:.1}%", progress * 100.0);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }),
    ) {
        Ok(result) => {
            println!("\n✅ Video conversion complete!");
            println!("   Original: {:.2} MB", result.original_size as f64 / 1_048_576.0);
            println!("   Converted: {:.2} MB", result.converted_size as f64 / 1_048_576.0);
            println!("   Duration: {:.2}s", result.duration);
            if result.compression_ratio > 0.0 {
                println!("   Compression: {:.1}%", (1.0 - result.compression_ratio) * 100.0);
            }
            
            // 🔥 Phase 46.1: 动图转视频原地替换 + Eagle metadata更新
            // ✅ 遵循架构原则：文件操作在Rust内核完成
            if is_animated_image {
                handle_animated_image_replacement(input_path, output, &input_ext);
            }
        }
        Err(e) => {
            eprintln!("\n❌ Video conversion failed: {}", e);
            process::exit(1);
        }
    }
}

/// 处理动图转视频的原地替换
fn handle_animated_image_replacement(input_path: &std::path::Path, output: &str, input_ext: &str) {
    use pixly_converter::converter::eagle_adapter::EagleAdapter;
    
    println!("\n🔄 Phase 46.1: Animated image → Video in-place replacement");
    
    // 转换为绝对路径以便检测Eagle库
    let abs_input_path = if input_path.is_absolute() {
        input_path.to_path_buf()
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(input_path),
            Err(_) => input_path.to_path_buf(),
        }
    };
    
    // 检测Eagle .info目录
    if let Some(parent) = abs_input_path.parent() {
        if parent.file_name()
            .and_then(|n| n.to_str())
            .map_or(false, |n| n.ends_with(".info"))
        {
            println!("   Eagle: ✅ Detected .info directory");
            
            // 使用EagleAdapter更新metadata
            if let Some(library_path) = parent.parent() {
                let adapter = EagleAdapter::new(library_path);
                
                // 读取并更新metadata.json
                match adapter.parse_info_dir(parent) {
                    Ok(mut metadata) => {
                        // 更新文件扩展名（gif → mp4/webm）
                        let output_path = std::path::Path::new(output);
                        let new_ext = output_path.extension()
                            .and_then(|s| s.to_str())
                            .unwrap_or("mp4");
                        
                        metadata.ext = new_ext.to_string();
                        
                        // 更新修改时间
                        if let Ok(metadata_sys) = std::fs::metadata(output_path) {
                            if let Ok(modified) = metadata_sys.modified() {
                                if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                                    metadata.modification_time = duration.as_millis() as u64;
                                }
                            }
                        }
                        
                        // 写回metadata.json
                        if let Err(e) = adapter.update_metadata(parent, &metadata) {
                            println!("   Eagle: ⚠️  Failed to update metadata: {}", e);
                        } else {
                            println!("   Eagle: ✅ Updated metadata.json (ext: {} → {})", input_ext, new_ext);
                        }
                        
                        // 删除原始动图文件
                        if abs_input_path.exists() {
                            match std::fs::remove_file(&abs_input_path) {
                                Ok(_) => println!("   Replace: ✅ Deleted original animated image"),
                                Err(e) => println!("   Replace: ⚠️  Failed to delete original: {}", e),
                            }
                        } else {
                            println!("   Replace: ℹ️  Original file already gone");
                        }
                    }
                    Err(e) => {
                        println!("   Eagle: ⚠️  Failed to parse metadata: {}", e);
                        println!("   Replace: Skipped (metadata read failed)");
                    }
                }
            }
        } else {
            println!("   Replace: ℹ️  Not in Eagle library, keeping both files");
        }
    } else {
        println!("   Replace: ⚠️  Cannot determine parent directory");
    }
}
