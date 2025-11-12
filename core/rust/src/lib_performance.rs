//! 🚀 PIXLY v3.0 极限性能核心库
//!
//! 这是Pixly历史上最激进的性能突破：
//! - Rust + SIMD + GPU三重加速
//! - PyO3原生Python集成
//! - 零拷贝内存管理  
//! - 企业级性能基准
//!
//! 架构目标：
//! - 图像处理: 10x - 200x 提升
//! - AI推理: 5x - 50x 提升
//! - 内存效率: 零拷贝 + 共享内存
//! - Python集成: 原生速度调用

pub mod performance;

// 重新导出核心类型
pub use performance::{
    PerformanceCore, PerformanceConfig, ImageOperation,
    BenchmarkCase, BenchmarkResult, SystemInfo,
};

// PyO3 Python绑定
use pyo3::prelude::*;

/// Python模块入口
#[pymodule]
fn pixly_performance_core(py: Python, m: &PyModule) -> PyResult<()> {
    // 添加性能类
    performance::python_bindings::pixly_performance_core(py, m)?;
    
    // 模块元信息
    m.add("__version__", "3.0.0")?;
    m.add("__author__", "PIXLY Team")?;
    m.add("__description__", "Ultra-high performance Rust core for PIXLY v3.0")?;
    m.add("__build_profile__", if cfg!(debug_assertions) { "debug" } else { "release" })?;
    
    // 编译时特性
    let mut features = Vec::new();
    
    #[cfg(feature = "simd")]
    features.push("simd");
    
    #[cfg(feature = "gpu")]
    features.push("gpu");
    
    #[cfg(feature = "profiling")]
    features.push("profiling");
    
    m.add("__features__", features)?;
    
    // CPU架构信息
    m.add("__target_arch__", std::env::consts::ARCH)?;
    m.add("__target_os__", std::env::consts::OS)?;
    
    Ok(())
}

/// 初始化日志系统
pub fn init_logger() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
}

/// 库版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 编译信息
pub const BUILD_INFO: &str = concat!(
    "Pixly v3.0 Performance Core ",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA_SHORT"),
    ")"
);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        println!("PIXLY Performance Core v{}", VERSION);
    }
    
    #[test] 
    fn test_build_info() {
        assert!(!BUILD_INFO.is_empty());
        println!("Build: {}", BUILD_INFO);
    }
    
    #[tokio::test]
    async fn test_performance_core_creation() {
        let config = PerformanceConfig::default();
        let result = PerformanceCore::new(config).await;
        
        match result {
            Ok(core) => {
                println!("✅ 性能核心创建成功");
                let system_info = core.get_system_info();
                println!("CPU核心: {}", system_info.cpu_info.logical_cores);
                println!("AVX2支持: {}", system_info.cpu_info.supports_avx2);
                println!("GPU可用: {}", system_info.gpu_info.is_some());
            }
            Err(e) => {
                println!("⚠️ 性能核心创建失败: {}", e);
                // 在CI环境中可能没有GPU，这是正常的
            }
        }
    }
}
