/**
 *   ==========================================
 * HTTP Server Module - HTTP服务器模块
 *   ==========================================
 * 
 * 提供RESTful API接口:
 * - POST /api/rust/convert - 图片转换
 * - GET  /health - 健康检查
 * 
 * 支持功能:
 * - 并发请求处理
 * - 流式文件上传
 * - 错误处理和日志
 * - CORS跨域支持
 *   ==========================================
 */
#[allow(clippy::module_inception)]
mod server;

#[cfg(feature = "http-server")]
pub mod handlers;
#[cfg(feature = "http-server")]
pub mod models;
#[cfg(feature = "http-server")]
pub use handlers::{health_handler, convert_handler};
#[cfg(feature = "http-server")]
pub use models::{ConvertRequest, ConvertResponse, HealthResponse};
#[cfg(feature = "http-server")]
pub use server::{run_server, get_server_config};
