/**
 * HTTP 服务器
 */
use actix_web::{web, App, HttpServer, middleware};
use std::io;
// 🔧 统一日志系统
use tracing::info;

use crate::server::handlers::{health_handler, convert_handler};
use crate::converter::strategy::init_global_manager;
use crate::converter::register_all_strategies;  // 🔥 重构：从converter模块直接导出

/// 运行HTTP服务器
/// 
/// # 参数
/// - `host`: 监听地址 (默认 "0.0.0.0")
/// - `port`: 监听端口 (默认 8080)
///
/// # 返回
/// - `Ok(())`: 服务器正常退出
/// - `Err`: 服务器启动失败
pub async fn run_server(host: &str, port: u16) -> io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("🦀 Initializing PIXLY Rust Converter HTTP Service");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // 初始化全局策略管理器 (在新作用域中完成,避免跨await持有锁)
    {
        let manager_lock = init_global_manager();
        let mut manager = manager_lock.lock().unwrap();
        
        // 注册所有策略
        register_all_strategies(&mut manager);
        
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("🌐 Server Configuration:");
        info!("   Host: {}", host);
        info!("   Port: {}", port);
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("📋 API Endpoints:");
        info!("   GET  http://{}:{}/health", host, port);
        info!("   POST http://{}:{}/api/rust/convert", host, port);
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        // manager在此作用域结束时自动释放
    }
    
    // 创建HTTP服务器
    let bind_addr = format!("{}:{}", host, port);
    info!("🚀 Starting server on {}", bind_addr);
    
    HttpServer::new(|| {
        App::new()
            // 日志中间件
            .wrap(middleware::Logger::default())
            // CORS中间件 (允许跨域请求)
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )
            // 路由
            .route("/health", web::get().to(health_handler))
            .route("/api/health", web::get().to(health_handler))  // 兼容旧客户端
            .route("/api/rust/convert", web::post().to(convert_handler))
    })
    .bind(&bind_addr)?
    .run()
    .await
}

/// 从环境变量获取配置
pub fn get_server_config() -> (String, u16) {
    let host = std::env::var("PIXLY_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PIXLY_PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .or_else(|| std::env::var("PORT").ok().and_then(|s| s.parse::<u16>().ok()))
        .unwrap_or(8080);
    
    (host, port)
}
