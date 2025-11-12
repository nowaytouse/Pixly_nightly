/**
 * PIXLY Rust Converter - Main Entry Point
 * 
 * High-performance image converter with native Rust encoders
 * and intelligent parameter optimization.
 */

use std::env;

mod cli;
mod logging;

use cli::{
    handle_convert_command, 
    handle_batch_command, 
    handle_info_command,
    handle_analyze_command,
    handle_eagle_command,
    handle_gif_optimize_command,
    handle_video_command,  // 🎬 Phase 40.24: 视频转换命令
    handle_detect_command,  // 🔥 Phase 45.3: AI文件类型检测命令
    show_tui_manual,
    show_quick_help
};

const VERSION: &str = "0.3.0";

fn main() {
    // 🔧 初始化统一日志系统（JSON模式，供JS插件解析）
    // 可通过环境变量PIXLY_LOG_LEVEL和PIXLY_LOG_JSON控制
    let log_level = env::var("PIXLY_LOG_LEVEL").unwrap_or_else(|_| "INFO".to_string());
    let json_output = env::var("PIXLY_LOG_JSON").is_ok();
    logging::init_logging(&log_level, json_output);
    
    log_info!("PIXLY Rust Converter started", version = VERSION);
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("❌ Error: No command specified");
        show_quick_help();
        std::process::exit(1);
    }
    
    let command = &args[1];
    
    // 🔥 Phase 40.19: 检查必需依赖（快速模式，仅在需要时检查）
    // --check-deps 命令会进行详细检查
    if command != "--help" && command != "-h" && 
       command != "--version" && command != "-v" &&
       command != "--check-deps" {
        if !cli::dependency_checker::check_required_dependencies_quiet() {
            eprintln!("💡 提示：运行 'pixly-rust --check-deps' 查看详细依赖信息");
            std::process::exit(1);
        }
    }

    match command.as_str() {
        "convert" => {
            handle_convert_command(&args[2..]);
        }
        "batch" => {
            handle_batch_command(&args[2..]);
        }
        "info" => {
            handle_info_command(&args[2..]);
        }
        "eagle" => {
            handle_eagle_command(&args[2..]);
        }
        "analyze" => {
            handle_analyze_command(&args[2..]);
        }
        "video" => {
            handle_video_command(&args[2..]);
        }
        "detect" => {  // 🔥 Phase 45.3: Magika AI 文件类型检测
            handle_detect_command(&args[2..]);
        }
        "gif-optimize" | "gif" => {
            handle_gif_optimize_command(&args[2..]);
        }
        "file-info" => {
            // 🔥 Phase 40.7.15: 文件元数据分析（供JavaScript调用）
            if args.len() < 3 {
                eprintln!("❌ Error: Missing file path");
                eprintln!("   Usage: pixly-rust file-info <file>");
                std::process::exit(1);
            }
            let input_path = std::path::Path::new(&args[2]);
            if let Err(e) = cli::analyze::handle_analyze_file(input_path) {
                eprintln!("❌ Error: {}", e);
                std::process::exit(1);
            }
        }
        "--help" | "-h" => {
            show_tui_manual();
        }
        "--version" | "-v" => {
            println!("Pixly Rust Converter v{}", VERSION);
        }
        "--check-deps" => {
            // 🔥 Phase 40.19: 检查外部依赖
            let all_ok = cli::dependency_checker::check_and_report_dependencies(true);
            if !all_ok {
                std::process::exit(1);
            }
        }
        // AI相关命令 (Phase 33 - 插件集成)
        "--get-active-models" => {
            if let Err(e) = cli::ai_commands::get_active_models() {
                eprintln!("❌ Error: {}", e);
                std::process::exit(1);
    }
        }
        "--get-model-stats" => {
            if let Err(e) = cli::ai_commands::get_model_stats() {
                eprintln!("❌ Error: {}", e);
                std::process::exit(1);
        }
        }
        "--test-ai-service" => {
            if let Err(e) = cli::ai_commands::test_ai_service() {
                eprintln!("❌ Error: {}", e);
                std::process::exit(1);
            }
        }
        "--send-feedback" => {
            if args.len() < 3 {
                eprintln!("❌ Error: --send-feedback requires JSON data");
                eprintln!("Usage: pixly-rust --send-feedback '<json>'");
                std::process::exit(1);
            }
            if let Err(e) = cli::ai_commands::send_feedback(&args[2]) {
                eprintln!("❌ Error: {}", e);
                std::process::exit(1);
            }
        }
        "--smart-recommend" => {
            if args.len() < 3 {
                eprintln!("❌ Error: --smart-recommend requires file info JSON");
                eprintln!("Usage: pixly-rust --smart-recommend '<json>'");
                std::process::exit(1);
        }
            if let Err(e) = cli::ai_commands::smart_recommend(&args[2]) {
                eprintln!("❌ Error: {}", e);
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("❌ Error: Unknown command '{}'", command);
            show_quick_help();
            std::process::exit(1);
        }
    }
}
