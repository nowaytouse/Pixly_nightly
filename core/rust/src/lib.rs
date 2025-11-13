//! 🚀 PIXLY v3.0 - 极限性能图像处理核心
//!
//! 三重架构革命：
//! 1. HTTP网络架构 → 本地化架构 (400x吞吐提升)
//! 2. 单核心处理 → Rust+Python双核心融合
//! 3. 静态路由 → SIMD+GPU+机器学习智能优化
//!
//! 性能目标：
//! - 图像处理: 10x - 200x 提升
//! - AI推理: 5x - 50x 提升  
//! - 并发吞吐: 100k+ ops/sec
//! - 延迟: 亚毫秒级响应

// 原有模块 (保持兼容性)
#[macro_use]
pub mod logging;
pub use logging::*;

pub mod error;
pub mod messaging;
pub mod constants;
pub mod info;
pub mod converter;
pub mod preprocessing;
// pub mod bridge;  // 🔄 已迁移到@deprecated - 未使用

// 🚀 新增：极限性能模块 (v3.0)
pub mod performance;
pub mod python_bridge;

// 🚀 v3.1 新增：转换引擎模块
pub mod conversion_engine;
pub mod quality_predictor;

// 根据编译特性选择入口点
#[cfg(feature = "performance")]
pub use crate::lib_performance::*;

#[cfg(not(feature = "performance"))]
pub use crate::lib_legacy::*;

// 性能核心 (默认)
mod lib_performance {
    pub use super::performance::*;
    
    /// 库版本
    pub const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-performance");
    
    /// 初始化高性能日志系统
    pub fn init_logger() {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .format_timestamp_micros()
            .init();
    }
}

// 传统兼容模式
mod lib_legacy {
    /// 库版本 (兼容模式)
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    
    /// 初始化日志（兼容模式）
    pub fn init_logger() {
        env_logger::init();
    }
}
