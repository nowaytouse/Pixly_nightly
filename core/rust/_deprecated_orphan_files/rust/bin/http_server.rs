/**
 *   ==========================================
 * PIXLY HTTP Server - Rust Image Converter
 *   ==========================================
 * 
 * 独立HTTP服务器，提供REST API
 * 
 * Usage:
 *   pixly-http-server [--host HOST] [--port PORT]
 * 
 * Default: http://0.0.0.0:8765
 *   ==========================================
 */
use pixly_converter::server::{run_server, get_server_config};
use std::env;

/// 显示TUI说明书
fn show_tui_manual() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   ____  ____  __ __   __  __  __  ");
    println!("  |  _ \\|  _ \\|  |\\ \\ / / | |  \\ \\ ");
    println!("  | |_) | |_) | | | \\ V /  | |   \\ \\");
    println!("  |  __/|  _ <| | |  > <   | |___/ /");
    println!("  |_|   |_| \\_\\__|_|/_/\\_\\ |______/  ");
    println!();
    println!("  PIXLY HTTP Server - Rust Image Conversion Service");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("📚 使用说明");
    println!();
    println!("  pixly-http-server [OPTIONS]");
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📦 环境变量配置");
    println!();
    println!("  PIXLY_HOST        服务器监听地址 (默认: 0.0.0.0)");
    println!("  PIXLY_PORT        服务器端口 (默认: 8080)");
    println!("  PORT              备用端口配置");
    println!("  RUST_LOG          日志级别 (info/debug/warn/error)");
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🚀 快速开始");
    println!();
    println!("  # 启动服务 (默认端口8080)");
    println!("  ./pixly-http-server");
    println!();
    println!("  # 指定端口");
    println!("  PIXLY_PORT=3000 ./pixly-http-server");
    println!();
    println!("  # 启用调试日志");
    println!("  RUST_LOG=debug ./pixly-http-server");
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🌐 API端点");
    println!();
    println!("  GET  /health              健康检查");
    println!("  GET  /api/health          健康检查 (详细信息)");
    println!("  POST /api/rust/convert    图像转换");
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 转换API示例");
    println!();
    println!("  curl -X POST http://localhost:8080/api/rust/convert \\");
    println!("    -H \"Content-Type: application/json\" \\");
    println!("    -d '{{");
    println!("      \"input\": \"/path/to/image.jpg\",");
    println!("      \"output\": \"/path/to/output.avif\",");
    println!("      \"format\": \"avif\",");
    println!("      \"quality\": 85,");
    println!("      \"speed\": 4,");
    println!("      \"lossless\": false,");
    println!("      \"preserve_metadata\": true,");
    println!("      \"keep_animated\": true");
    println!("    }}'");
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✨ 核心功能");
    println!();
    println!("  🦀 原生Rust编码器: AVIF/WebP/PNG/JPEG");
    println!("  🔧 CLI工具fallback: JXL/HEIC/GIF");
    println!("  📋 元数据完整保留: EXIF/XMP/ICC/Timestamps");
    println!("  🔒 多级安全验证: 6级验证系统");
    println!("  💾 智能缓存系统: LRU + SHA256哈希");
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💡 推荐使用Eagle插件版本以获得最佳体验");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 检查命令行参数
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        let arg = &args[1];
        if arg == "--help" || arg == "-h" || arg == "help" {
            show_tui_manual();
            return Ok(());
        }
    }
    
    // 没有参数时，显示TUI并继续启动服务
    if args.len() == 1 {
        show_tui_manual();
        println!("\n🚀 Starting server...\n");
    }
    
    let (host, port) = get_server_config();
    run_server(&host, port).await
}
