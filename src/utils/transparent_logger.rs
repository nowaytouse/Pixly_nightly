// 🔍 transparencyloggingSystem
// let 用户清楚解everya步操作Detailed information

use std::time::Instant;
use serde::{Deserialize, Serialize};

/// logginglevel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
 Debug, // 调试information
 Info, // a般information
 Detail, // detailedinformation
 Warning, // 警告
 Error, // 错误
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
 LogLevel::Debug => "\x1b[36m", // Cyan
 LogLevel::Info => "\x1b[32m", // Green
 LogLevel::Detail => "\x1b[34m", // Blue
 LogLevel::Warning => "\x1b[33m", // Yellow
 LogLevel::Error => "\x1b[31m", // Red
 }
 }
}

/// transparencylog
pub structure TransparentLogger {
 enabled: bool,
 show_timestamps: bool,
 show_details: bool,
 indent_level: usize,
}

impl TransparentLogger {
 /// createnewlog
 pub fn new() -> Self {
 Self {
 enabled: true,
 show_timestamps: true,
 show_details: true,
 indent_level: 0,
 }
 }
 
 /// settingis否enabled
 pub fn set_enabled(&mut self, enabled: bool) {
 self.enabled = enabled;
 }
 
 /// settingis否displaytime戳
 pub fn set_show_timestamps(&mut self, show: bool) {
 self.show_timestamps = show;
 }
 
 /// settingis否displayDetailed information
 pub fn set_show_details(&mut self, show: bool) {
 self.show_details = show;
 }
 
 /// add缩进
 pub fn indent(&mut self) {
 self.indent_level += 1;
 }
 
 /// reduce缩进
 pub fn dedent(&mut self) {
 if self.indent_level > 0 {
 self.indent_level -= 1;
 }
 }
 
 /// recordlogging
 pub fn log(&self, level: LogLevel, message: &str) {
 if !self.enabled {
 return;
 }
 
 let indent = " ".repeat(self.indent_level);
 let timestamp = if self.show_timestamps {
 use std::time::SystemTime;
 let now = SystemTime::now()
 .duration_since(SystemTime::UNIX_EPOCH)
 .unwrap_or_default();
 format!("[{:.3}s] ", now.as_secs_f64() % 1000.0)
 } else {
 String::new()
 };
 
 println!(
 "{}{}{} {} {}\x1b[0m",
 timestamp,
 indent,
 level.emoji(),
 level.color_code(),
 message // Reset color
 );
 }
 
 /// record带Detailed informationlogging
 pub fn log_with_details(&self, level: LogLevel, message: &str, details: &[(&str, String)]) {
 self.log(level, message);
 
 if self.show_details && !details.is_empty() {
 let indent = " ".repeat(self.indent_level + 1);
 for (key, value) in details {
 println!("{} → {}: {}", indent, key, value);
 }
 }
 }
 
 /// record操作start
 pub fn log_operation_start(&self, operation: &str) {
 self.log(LogLevel::Info, &format!("Starting: {}", operation));
 }
 
 /// record操作completed
 pub fn log_operation_end(&self, operation: &str, duration: std::time::Duration) {
 self.log(
 LogLevel::Info,
 &format!("Completed: {} (elapsed: {:.2}s)", operation, duration.as_secs_f64())
 );
 }
 
 /// record分隔线
 pub fn log_separator(&self) {
 if self.enabled {
 let indent = " ".repeat(self.indent_level);
 println!("{}{}", indent, "─".repeat(60));
 }
 }
 
 /// record标题
 pub fn log_header(&self, title: &str) {
 if self.enabled {
 let indent = " ".repeat(self.indent_level);
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

/// 操作tracking
pub structure OperationTracker {
 logger: TransparentLogger,
 operation_name: String,
 start_time: Instant,
}

impl OperationTracker {
 /// starttracking操作
 pub fn start(logger: TransparentLogger, operation_name: &str) -> Self {
 logger.log_operation_start(operation_name);
 
 Self {
 logger,
 operation_name: operation_name.to_string(),
 start_time: Instant::now(),
 }
 }
 
 /// recordstep
 pub fn log_step(&self, step: &str) {
 self.logger.log(LogLevel::Detail, &format!("→ {}", step));
 }
 
 /// recordDetailed information
 pub fn log_details(&self, details: &[(&str, String)]) {
 self.logger.log_with_details(LogLevel::Detail, "detailedinformation:", details);
 }
 
 /// recordwarning
 pub fn log_warning(&self, warning: &str) {
 self.logger.log(LogLevel::Warning, warning);
 }
 
 /// recorderror
 pub fn log_error(&self, error: &str) {
 self.logger.log(LogLevel::Error, error);
 }
 
 /// getlog
 pub fn logger(&self) -> &TransparentLogger {
 &self.logger
 }
}

impl OperationTracker {
 /// completed操作
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

/// globallogging宏
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
