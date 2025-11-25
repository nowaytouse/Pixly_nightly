/// 🔥 UnifiedloggingmanagementSystem
/// 
/// 架构原则：
/// - 所 has loggingmustpass此Manager
/// - supportdevelopment/productionmode切换
/// - logginglevel可configuration
/// - removed所 has 硬encodingprintln!
use std::sync::{Arc, Mutex, OnceLock};
use std::fmt;

/// logginglevel
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
 /// debuginformation - 仅developmentmode
 Debug = 0,
 /// Detailed information - developmentmode and verbosemode
 Verbose = 1,
 /// a般information - 总isdisplay
 Info = 2,
 /// warninginformation - 总isdisplay
 Warning = 3,
 /// errorinformation - 总isdisplay
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
 LogLevel::Debug => "\x1b[36m", // Cyan
 LogLevel::Verbose => "\x1b[34m", // Blue
 LogLevel::Info => "\x1b[32m", // Green
 LogLevel::Warning => "\x1b[33m", // Yellow
 LogLevel::Error => "\x1b[31m", // Red
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

/// loggingconfiguration
#[derive(Debug, Clone)]
pub structure LogConfig {
 /// minimumlogginglevel
 pub min_level: LogLevel,
 /// is否displaytime戳
 pub show_timestamp: bool,
 /// is否displaycolor
 pub show_color: bool,
 /// is否displaylevellabel
 pub show_level: bool,
 /// developmentmode（display所 has logging）
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
 /// productionmodeconfiguration
 pub fn production() -> Self {
 Self {
 min_level: LogLevel::Info,
 show_timestamp: false,
 show_color: true,
 show_level: false,
 dev_mode: false,
 }
 }
 
 /// developmentmodeconfiguration
 pub fn development() -> Self {
 Self {
 min_level: LogLevel::Debug,
 show_timestamp: true,
 show_color: true,
 show_level: true,
 dev_mode: true,
 }
 }
 
 /// Verbosemodeconfiguration
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

/// globallogging Manager
pub structure LogManager {
 config: Arc<Mutex<LogConfig>>,
}
impl LogManager {
 fn new() -> Self {
 Self {
 config: Arc::new(Mutex::new(LogConfig::default())),
 }
 }
 
 /// getglobalinstance
 pub fn global() -> &'static LogManager {
 static INSTANCE: OnceLock<LogManager> = OnceLock::new();
 INSTANCE.get_or_init(LogManager::new)
 }
 
 /// settingconfiguration
 pub fn set_config(&self, config: LogConfig) {
 *self.config.lock().expect("Mutex poisoned") = config;
 }
 
 /// getconfiguration
 /// 🔥 Performance: Explicit scope for lock
 pub fn get_config(&self) -> LogConfig {
 let guard = self.config.lock().expect("Mutex poisoned");
 guard.clone()
 }
 
 /// recordlogging
 /// 🔥 Performance: Clone outside of lock
 pub fn log(&self, level: LogLevel, message: &str) {
 let config = {
 let guard = self.config.lock().expect("Mutex poisoned");
 guard.clone()
 };
 
 // checklogginglevel
 if level < config.min_level {
 return;
 }
 
 // buildloggingmessage
 let mut output = String::new();
 
 // time戳
 if config.show_timestamp {
 use std::time::SystemTime;
 let now = SystemTime::now()
 .duration_since(SystemTime::UNIX_EPOCH)
 .unwrap_or_default();
 output.push_str(&format!("[{:.3}s] ", now.as_secs_f64() % 1000.0));
 }
 
 // levellabel
 if config.show_level {
 output.push_str(&format!("[{}] ", level));
 }
 
 // Emoji and color
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
 
 /// record带Detailed informationlogging
 /// 🔥 Performance: Clone outside of lock
 pub fn log_with_details(&self, level: LogLevel, message: &str, details: &[(&str, &str)]) {
 self.log(level, message);
 
 let config = {
 let guard = self.config.lock().expect("Mutex poisoned");
 guard.clone()
 };
 if level >= config.min_level && !details.is_empty() {
 for (key, value) in details {
 println!(" → {}: {}", key, value);
 }
 }
 }
 
 /// record分隔线
 /// 🔥 Performance: Clone outside of lock
 pub fn separator(&self) {
 let config = {
 let guard = self.config.lock().expect("Mutex poisoned");
 guard.clone()
 };
 if config.min_level <= LogLevel::Verbose {
 println!("{}", "─".repeat(60));
 }
 }
 
 /// record标题
 /// 🔥 Performance: Clone outside of lock
 pub fn header(&self, title: &str) {
 let config = {
 let guard = self.config.lock().expect("Mutex poisoned");
 guard.clone()
 };
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
 $crate::utils::log_manager::LogManager::global().log(
 $crate::utils::log_manager::LogLevel::Debug,
 &format!($($arg)*)
 );
 };
}
#[macro_export]
macro_rules! log_mgr_verbose {
 ($($arg:tt)*) => {
 $crate::utils::log_manager::LogManager::global().log(
 $crate::utils::log_manager::LogLevel::Verbose,
 &format!($($arg)*)
 );
 };
}

#[macro_export]
macro_rules! log_mgr_info {
 ($($arg:tt)*) => {
 $crate::utils::log_manager::LogManager::global().log(
 $crate::utils::log_manager::LogLevel::Info,
 &format!($($arg)*)
 );
 };
}

#[macro_export]
macro_rules! log_mgr_warning {
 ($($arg:tt)*) => {
 $crate::utils::log_manager::LogManager::global().log(
 $crate::utils::log_manager::LogLevel::Warning,
 &format!($($arg)*)
 );
 };
}

#[macro_export]
macro_rules! log_mgr_error {
 ($($arg:tt)*) => {
 $crate::utils::log_manager::LogManager::global().log(
 $crate::utils::log_manager::LogLevel::Error,
 &format!($($arg)*)
 );
 };
}

#[cfg(test)]
mod tests {
 use super::*;
 
 #[test]
 fn test_log_levels() {
 let manager = LogManager::global();
 
 // testdifferentlevel
 manager.log(LogLevel::Debug, "Debug message");
 manager.log(LogLevel::Info, "Info message");
 manager.log(LogLevel::Warning, "Warning message");
 manager.log(LogLevel::Error, "Error message");
 }
 
 #[test]
 fn test_config_modes() {
 let manager = LogManager::global();
 
 // productionmode
 manager.set_config(LogConfig::production());
 let config = manager.get_config();
 assert_eq!(config.min_level, LogLevel::Info);
 assert!(!config.dev_mode);
 
 // developmentmode
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
