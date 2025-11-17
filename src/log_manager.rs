/// 🔥 统一日志管理系统
/// 
/// 架构原则：
/// - 所有日志必须通过此管理器
/// - 支持开发/生产模式切换
/// - 日志级别可配置
/// - 移除所有硬编码println!

use std::sync::{Arc, Mutex, OnceLock};
use std::fmt;

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// 调试信息 - 仅开发模式
    Debug = 0,
    /// 详细信息 - 开发模式和verbose模式
    Verbose = 1,
    /// 一般信息 - 总是显示
    Info = 2,
    /// 警告信息 - 总是显示
    Warning = 3,
    /// 错误信息 - 总是显示
    Error = 4,
}

impl LogLevel {
    pub fn emoji(&self) -> &'static str {
        match self {
            LogLevel::Debug => "🔧",
            LogLevel::Verbose => "📋",
            LogLevel::Info => "ℹ️",
            LogLevel::Warning => "⚠️",
            LogLevel::Error => "❌",
        }
    }
    
    pub fn color_code(&self) -> &'static str {
        match self {
            LogLevel::Debug => "\x1b[36m",    // Cyan
            LogLevel::Verbose => "\x1b[34m",  // Blue
            LogLevel::Info => "\x1b[32m",     // Green
            LogLevel::Warning => "\x1b[33m",  // Yellow
            LogLevel::Error => "\x1b[31m",    // Red
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Verbose => write!(f, "VERBOSE"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// 日志配置
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// 最小日志级别
    pub min_level: LogLevel,
    /// 是否显示时间戳
    pub show_timestamp: bool,
    /// 是否显示颜色
    pub show_color: bool,
    /// 是否显示级别标签
    pub show_level: bool,
    /// 开发模式（显示所有日志）
    pub dev_mode: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            min_level: LogLevel::Info,
            show_timestamp: false,
            show_color: true,
            show_level: false,
            dev_mode: false,
        }
    }
}

impl LogConfig {
    /// 生产模式配置
    pub fn production() -> Self {
        Self {
            min_level: LogLevel::Info,
            show_timestamp: false,
            show_color: true,
            show_level: false,
            dev_mode: false,
        }
    }
    
    /// 开发模式配置
    pub fn development() -> Self {
        Self {
            min_level: LogLevel::Debug,
            show_timestamp: true,
            show_color: true,
            show_level: true,
            dev_mode: true,
        }
    }
    
    /// Verbose模式配置
    pub fn verbose() -> Self {
        Self {
            min_level: LogLevel::Verbose,
            show_timestamp: false,
            show_color: true,
            show_level: false,
            dev_mode: false,
        }
    }
}

/// 全局日志管理器
pub struct LogManager {
    config: Arc<Mutex<LogConfig>>,
}

impl LogManager {
    fn new() -> Self {
        Self {
            config: Arc::new(Mutex::new(LogConfig::default())),
        }
    }
    
    /// 获取全局实例
    pub fn global() -> &'static LogManager {
        static INSTANCE: OnceLock<LogManager> = OnceLock::new();
        INSTANCE.get_or_init(LogManager::new)
    }
    
    /// 设置配置
    pub fn set_config(&self, config: LogConfig) {
        *self.config.lock().unwrap() = config;
    }
    
    /// 获取配置
    pub fn get_config(&self) -> LogConfig {
        self.config.lock().unwrap().clone()
    }
    
    /// 记录日志
    pub fn log(&self, level: LogLevel, message: &str) {
        let config = self.config.lock().unwrap().clone();
        
        // 检查日志级别
        if level < config.min_level {
            return;
        }
        
        // 构建日志消息
        let mut output = String::new();
        
        // 时间戳
        if config.show_timestamp {
            use std::time::SystemTime;
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            output.push_str(&format!("[{:.3}s] ", now.as_secs_f64() % 1000.0));
        }
        
        // 级别标签
        if config.show_level {
            output.push_str(&format!("[{}] ", level));
        }
        
        // Emoji和颜色
        if config.show_color {
            output.push_str(&format!("{} {}{}\x1b[0m", 
                level.emoji(), 
                level.color_code(), 
                message
            ));
        } else {
            output.push_str(&format!("{} {}", level.emoji(), message));
        }
        
        println!("{}", output);
    }
    
    /// 记录带详细信息的日志
    pub fn log_with_details(&self, level: LogLevel, message: &str, details: &[(&str, &str)]) {
        self.log(level, message);
        
        let config = self.config.lock().unwrap().clone();
        if level >= config.min_level && !details.is_empty() {
            for (key, value) in details {
                println!("  → {}: {}", key, value);
            }
        }
    }
    
    /// 记录分隔线
    pub fn separator(&self) {
        let config = self.config.lock().unwrap().clone();
        if config.min_level <= LogLevel::Verbose {
            println!("{}", "─".repeat(60));
        }
    }
    
    /// 记录标题
    pub fn header(&self, title: &str) {
        let config = self.config.lock().unwrap().clone();
        if config.min_level <= LogLevel::Info {
            println!("\n╔═══════════════════════════════════════════════════════════╗");
            println!("║ {:^57} ║", title);
            println!("╚═══════════════════════════════════════════════════════════╝");
        }
    }
}

/// 便捷宏
#[macro_export]
macro_rules! log_mgr_debug {
    ($($arg:tt)*) => {
        $crate::log_manager::LogManager::global().log(
            $crate::log_manager::LogLevel::Debug,
            &format!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! log_mgr_verbose {
    ($($arg:tt)*) => {
        $crate::log_manager::LogManager::global().log(
            $crate::log_manager::LogLevel::Verbose,
            &format!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! log_mgr_info {
    ($($arg:tt)*) => {
        $crate::log_manager::LogManager::global().log(
            $crate::log_manager::LogLevel::Info,
            &format!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! log_mgr_warning {
    ($($arg:tt)*) => {
        $crate::log_manager::LogManager::global().log(
            $crate::log_manager::LogLevel::Warning,
            &format!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! log_mgr_error {
    ($($arg:tt)*) => {
        $crate::log_manager::LogManager::global().log(
            $crate::log_manager::LogLevel::Error,
            &format!($($arg)*)
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_levels() {
        let manager = LogManager::global();
        
        // 测试不同级别
        manager.log(LogLevel::Debug, "Debug message");
        manager.log(LogLevel::Info, "Info message");
        manager.log(LogLevel::Warning, "Warning message");
        manager.log(LogLevel::Error, "Error message");
    }
    
    #[test]
    fn test_config_modes() {
        let manager = LogManager::global();
        
        // 生产模式
        manager.set_config(LogConfig::production());
        let config = manager.get_config();
        assert_eq!(config.min_level, LogLevel::Info);
        assert!(!config.dev_mode);
        
        // 开发模式
        manager.set_config(LogConfig::development());
        let config = manager.get_config();
        assert_eq!(config.min_level, LogLevel::Debug);
        assert!(config.dev_mode);
    }
    
    #[test]
    fn test_macros() {
        log_mgr_debug!("Debug: {}", "test");
        log_mgr_info!("Info: {}", "test");
        log_mgr_warning!("Warning: {}", "test");
        log_mgr_error!("Error: {}", "test");
    }
}
