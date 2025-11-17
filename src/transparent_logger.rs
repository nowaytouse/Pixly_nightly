// 🔍 透明日志系统
// 让用户清楚了解每一步操作的详细信息

use std::time::Instant;
use serde::{Deserialize, Serialize};

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,    // 调试信息
    Info,     // 一般信息
    Detail,   // 详细信息
    Warning,  // 警告
    Error,    // 错误
}

impl LogLevel {
    pub fn emoji(&self) -> &'static str {
        match self {
            LogLevel::Debug => "🔧",
            LogLevel::Info => "ℹ️",
            LogLevel::Detail => "📋",
            LogLevel::Warning => "⚠️",
            LogLevel::Error => "❌",
        }
    }
    
    pub fn color_code(&self) -> &'static str {
        match self {
            LogLevel::Debug => "\x1b[36m",    // Cyan
            LogLevel::Info => "\x1b[32m",     // Green
            LogLevel::Detail => "\x1b[34m",   // Blue
            LogLevel::Warning => "\x1b[33m",  // Yellow
            LogLevel::Error => "\x1b[31m",    // Red
        }
    }
}

/// 透明日志记录器
pub struct TransparentLogger {
    enabled: bool,
    show_timestamps: bool,
    show_details: bool,
    indent_level: usize,
}

impl TransparentLogger {
    /// 创建新的日志记录器
    pub fn new() -> Self {
        Self {
            enabled: true,
            show_timestamps: true,
            show_details: true,
            indent_level: 0,
        }
    }
    
    /// 设置是否启用
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// 设置是否显示时间戳
    pub fn set_show_timestamps(&mut self, show: bool) {
        self.show_timestamps = show;
    }
    
    /// 设置是否显示详细信息
    pub fn set_show_details(&mut self, show: bool) {
        self.show_details = show;
    }
    
    /// 增加缩进
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }
    
    /// 减少缩进
    pub fn dedent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }
    
    /// 记录日志
    pub fn log(&self, level: LogLevel, message: &str) {
        if !self.enabled {
            return;
        }
        
        let indent = "  ".repeat(self.indent_level);
        let timestamp = if self.show_timestamps {
            use std::time::SystemTime;
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            format!("[{:.3}s] ", now.as_secs_f64() % 1000.0)
        } else {
            String::new()
        };
        
        println!(
            "{}{}{} {} {}{}",
            timestamp,
            indent,
            level.emoji(),
            level.color_code(),
            message,
            "\x1b[0m" // Reset color
        );
    }
    
    /// 记录带详细信息的日志
    pub fn log_with_details(&self, level: LogLevel, message: &str, details: &[(&str, String)]) {
        self.log(level, message);
        
        if self.show_details && !details.is_empty() {
            let indent = "  ".repeat(self.indent_level + 1);
            for (key, value) in details {
                println!("{}  {} {}: {}", indent, "→", key, value);
            }
        }
    }
    
    /// 记录操作开始
    pub fn log_operation_start(&self, operation: &str) {
        self.log(LogLevel::Info, &format!("Starting: {}", operation));
    }
    
    /// 记录操作完成
    pub fn log_operation_end(&self, operation: &str, duration: std::time::Duration) {
        self.log(
            LogLevel::Info,
            &format!("Completed: {} (elapsed: {:.2}s)", operation, duration.as_secs_f64())
        );
    }
    
    /// 记录分隔线
    pub fn log_separator(&self) {
        if self.enabled {
            let indent = "  ".repeat(self.indent_level);
            println!("{}{}", indent, "─".repeat(60));
        }
    }
    
    /// 记录标题
    pub fn log_header(&self, title: &str) {
        if self.enabled {
            let indent = "  ".repeat(self.indent_level);
            println!("\n{}╔═══════════════════════════════════════════════════════════╗", indent);
            println!("{}║ {:^57} ║", indent, title);
            println!("{}╚═══════════════════════════════════════════════════════════╝", indent);
        }
    }
}

impl Default for TransparentLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for TransparentLogger {
    fn clone(&self) -> Self {
        Self {
            enabled: self.enabled,
            show_timestamps: self.show_timestamps,
            show_details: self.show_details,
            indent_level: self.indent_level,
        }
    }
}

/// 操作追踪器
pub struct OperationTracker {
    logger: TransparentLogger,
    operation_name: String,
    start_time: Instant,
}

impl OperationTracker {
    /// 开始追踪操作
    pub fn start(logger: TransparentLogger, operation_name: &str) -> Self {
        logger.log_operation_start(operation_name);
        
        Self {
            logger,
            operation_name: operation_name.to_string(),
            start_time: Instant::now(),
        }
    }
    
    /// 记录步骤
    pub fn log_step(&self, step: &str) {
        self.logger.log(LogLevel::Detail, &format!("→ {}", step));
    }
    
    /// 记录详细信息
    pub fn log_details(&self, details: &[(&str, String)]) {
        self.logger.log_with_details(LogLevel::Detail, "详细信息:", details);
    }
    
    /// 记录警告
    pub fn log_warning(&self, warning: &str) {
        self.logger.log(LogLevel::Warning, warning);
    }
    
    /// 记录错误
    pub fn log_error(&self, error: &str) {
        self.logger.log(LogLevel::Error, error);
    }
    
    /// 获取日志记录器
    pub fn logger(&self) -> &TransparentLogger {
        &self.logger
    }
}

impl OperationTracker {
    /// 完成操作
    pub fn finish(self) {
        // Drop will be called automatically
    }
}

impl Drop for OperationTracker {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        self.logger.log_operation_end(&self.operation_name, duration);
    }
}

/// 全局日志宏
#[macro_export]
macro_rules! log_transparent {
    ($logger:expr, $level:expr, $($arg:tt)*) => {
        $logger.log($level, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $($arg:tt)*) => {
        $logger.log($crate::transparent_logger::LogLevel::Info, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_detail {
    ($logger:expr, $($arg:tt)*) => {
        $logger.log($crate::transparent_logger::LogLevel::Detail, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warning {
    ($logger:expr, $($arg:tt)*) => {
        $logger.log($crate::transparent_logger::LogLevel::Warning, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $($arg:tt)*) => {
        $logger.log($crate::transparent_logger::LogLevel::Error, &format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_logger_creation() {
        let logger = TransparentLogger::new();
        assert!(logger.enabled);
        assert!(logger.show_timestamps);
        assert!(logger.show_details);
    }
    
    #[test]
    fn test_log_levels() {
        let logger = TransparentLogger::new();
        logger.log(LogLevel::Info, "Test info");
        logger.log(LogLevel::Warning, "Test warning");
        logger.log(LogLevel::Error, "Test error");
    }
    
    #[test]
    fn test_operation_tracker() {
        let logger = TransparentLogger::new();
        let tracker = OperationTracker::start(logger, "Test Operation");
        tracker.log_step("Step 1");
        tracker.log_step("Step 2");
        // Drop will log completion
    }
}
