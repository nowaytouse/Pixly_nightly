/**
 * Info Command - 媒体信息查询
 * 从 cli/commands.rs 拆分 (~65行)
 * 
 * 负责:
 * - 显示图片/视频基本信息
 * - 分辨率、格式、编码等
 * - JSON/人类可读输出
 */

use pixly_converter::converter::media_analyzer::MediaAnalyzer;
use std::path::Path;
use std::process;

/// 处理 info 命令
pub fn handle(args: &[String]) {
    if args.is_empty() {
        eprintln!("❌ Error: Missing file path");
        eprintln!("   Usage: pixly-rust info <file> [--json]");
        process::exit(1);
    }

    let file_path = &args[0];
    let json_output = args.contains(&"--json".to_string());
    
    // 使用MediaAnalyzer获取完整信息
    let analyzer = MediaAnalyzer::new();
    match analyzer.analyze(Path::new(file_path)) {
        Ok(info) => {
            if json_output {
                output_json(&info);
            } else {
                output_human_readable(&info, file_path);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to analyze file: {}", e);
            process::exit(1);
        }
    }
}

/// JSON 格式输出
fn output_json(info: &pixly_converter::converter::media_analyzer::MediaInfo) {
    match serde_json::to_string_pretty(&info) {
        Ok(json) => println!("{}", json),
        Err(e) => {
            eprintln!("❌ Failed to serialize to JSON: {}", e);
            process::exit(1);
        }
    }
}

/// 人类可读格式输出
fn output_human_readable(info: &pixly_converter::converter::media_analyzer::MediaInfo, file_path: &str) {
    println!("\n📋 Media Information:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   File: {}", file_path);
    println!("   Type: {:?}", info.media_type);
    println!("   Size: {} bytes", info.size);
    println!("   Format: {}", info.format);
    println!("   Resolution: {}x{}", info.resolution.0, info.resolution.1);
    
    if let Some(fps) = info.fps {
        println!("   FPS: {:.2}", fps);
    }
    if let Some(frames) = info.frame_count {
        println!("   Frames: {}", frames);
    }
    if let Some(duration) = info.duration {
        println!("   Duration: {:.2}s", duration);
    }
    if let Some(bitrate) = info.bitrate {
        println!("   Bitrate: {} kbps", bitrate);
    }
    if info.has_audio {
        println!("   Audio: Yes{}", 
            if let Some(codec) = &info.audio_codec {
                format!(" ({})", codec)
            } else {
                String::new()
            }
        );
    }
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}
