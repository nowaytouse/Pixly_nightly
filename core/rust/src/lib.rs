//! PIXLY Converter - High-performance image and video converter core
//! 
//! 保守集成计划：
//! - 阶段 0: ✅ 项目初始化
//! - 阶段 1: ⏳ 只读工具（图像信息、格式检测）
//! - 阶段 2: ⏳ 验证工具（质量验证、文件验证）
//! - 阶段 3: ⏳ 元数据工具
//! - 阶段 4+: 待评估

// 🔥 Phase 43.2: FFI已废弃，迁移到HTTP API（移除449行旧代码）
#[macro_use]
pub mod logging;

// Re-export logging macros for easy access
pub use logging::*;

pub mod error;      // 统一错误码系统 (Phase 46.8)
pub mod messaging;  // Phase 46.14+: 统一消息传递系统
pub mod constants;  // 统一常量配置 (Phase 46.8)
pub mod info;
pub mod converter;  // 完整转换功能
pub mod preprocessing;  // 预处理管道 (Phase 46.14, 参考Rimage)
pub mod bridge;     // Python桥接器 (EX-010)

#[cfg(feature = "http-server")]
pub mod server;  // HTTP服务器 (Phase 22)

/// 库版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 初始化日志（可选）
pub fn init_logger() {
    env_logger::init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        println!("PIXLY Converter v{}", VERSION);
    }
}
