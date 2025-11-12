// 🔧 PIXLY Rust统一日志模块
// 遵循CROSS_PLATFORM_LOG_SPEC.md规范
// 与JS插件和GO服务保持一致的日志级别和格式
//
// ✅ Phase 46.14+ 三端统一日志规范 (2025-11-11)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 统一日志格式：
// {
//   "timestamp": "2025-11-11T12:00:00.000Z",
//   "level": "INFO",
//   "layer": "rust-core",         // ✅ 固定值
//   "component": "strategy_manager",
//   "message": "Conversion completed",
//   "code": "PIXLY-RUST-FILE-002", // 可选
//   "context": {...},              // 可选
//   "trace_id": "req-123456"       // 可选
// }
//
// 错误码格式：PIXLY-RUST-{CATEGORY}-{NUMBER}
// - VAL: 验证类错误
// - FILE: 文件类错误
// - NET: 网络类错误
// - SYS: 系统类错误
// - BIZ: 业务类错误
//
// 参见：docs/architecture/PROJECT_QUALITY_MANIFESTO.md
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

use serde::Serialize;
use tracing::Level;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// 统一日志级别（与JS/GO一致）
#[derive(Debug, Clone, Copy, Serialize)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => Level::ERROR,
            LogLevel::Warn => Level::WARN,
            LogLevel::Info => Level::INFO,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Trace => Level::TRACE,
        }
    }
}

impl From<&str> for LogLevel {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "ERROR" => LogLevel::Error,
            "WARN" => LogLevel::Warn,
            "INFO" => LogLevel::Info,
            "DEBUG" => LogLevel::Debug,
            "TRACE" => LogLevel::Trace,
            _ => LogLevel::Info, // 默认
        }
    }
}

/// 日志条目结构（用于JSON输出）
/// Phase 46.13: 保留用于未来的结构化日志功能
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub layer: String,
    pub component: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

#[allow(dead_code)]
impl LogEntry {
    /// 创建新的日志条目
    pub fn new(
        level: &str,
        component: &str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            level: level.to_string(),
            layer: "rust-core".to_string(),  // ✅ 三端统一：rust-core
            component: component.to_string(),
            message: message.into(),
            code: None,
            context: None,
            trace_id: None,
        }
    }

    /// 添加错误码
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// 添加上下文
    pub fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = Some(context);
        self
    }

    /// 添加追踪ID
    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    /// 输出日志（JSON格式）
    pub fn log_json(&self) {
        let json = serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string());
        println!("{}", json);
    }

    /// 输出日志（人类可读格式）
    pub fn log_pretty(&self) {
        let icon = match self.level.as_str() {
            "CRITICAL" => "💥",
            "ERROR" => "❌",
            "WARNING" => "⚠️",
            "INFO" => "✅",
            "DEBUG" => "🔍",
            "TRACE" => "📍",
            _ => "ℹ️",
        };

        println!(
            "{} {} [{}] {}: {}",
            icon,
            self.timestamp,
            self.component,
            self.code.as_deref().unwrap_or(""),
            self.message
        );

        if let Some(ctx) = &self.context {
            println!("   Context: {}", ctx);
        }
    }
}

/// 初始化日志系统
/// 
/// # 参数
/// - `log_level`: 日志级别字符串 ("ERROR", "WARN", "INFO", "DEBUG", "TRACE")
/// - `json_output`: 是否输出JSON格式（用于与JS通信）
/// 
/// # 示例
/// ```
/// // 标准输出（人类可读）
/// init_logging("INFO", false);
/// 
/// // JSON输出（JS解析）
/// init_logging("DEBUG", true);
/// ```
pub fn init_logging(log_level: &str, json_output: bool) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new(format!("pixly_rust={}", log_level.to_lowercase()))
        });

    if json_output {
        // JSON格式输出（用于JS插件解析）
        tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .json()
                    .with_current_span(true)
                    .with_span_list(false)
                    .with_target(true)
                    .with_level(true)
            )
            .init();
    } else {
        // 人类可读格式输出（开发调试）
        tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .with_target(true)
                    .with_level(true)
                    .with_ansi(true)
            )
            .init();
    }
}

/// 日志宏包装（简化使用）
/// 
/// # 示例
/// ```
/// use crate::logging::*;
/// 
/// log_error!("File conversion failed", file = "photo.jpg");
/// log_warn!("Low memory detected", available_mb = 100);
/// log_info!("Image converted", input = "a.jpg", output = "a.webp", duration_ms = 123);
/// log_debug!("Processing frame", frame_num = 42, total = 100);
/// log_trace!("Function entered", func = "convert_image");
/// ```
#[macro_export]
macro_rules! log_error {
    ($msg:expr $(, $key:ident = $value:expr)*) => {
        tracing::error!($($key = ?$value,)* "{}", $msg);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($msg:expr $(, $key:ident = $value:expr)*) => {
        tracing::warn!($($key = ?$value,)* "{}", $msg);
    };
}

#[macro_export]
macro_rules! log_info {
    ($msg:expr $(, $key:ident = $value:expr)*) => {
        tracing::info!($($key = ?$value,)* "{}", $msg);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($msg:expr $(, $key:ident = $value:expr)*) => {
        tracing::debug!($($key = ?$value,)* "{}", $msg);
    };
}

#[macro_export]
macro_rules! log_trace {
    ($msg:expr $(, $key:ident = $value:expr)*) => {
        tracing::trace!($($key = ?$value,)* "{}", $msg);
    };
}

/// 性能追踪Span（用于计时）
/// 
/// # 示例
/// ```
/// use crate::logging::*;
/// 
/// let _span = perf_span!("image_conversion", format = "webp", quality = 85);
/// // ... 执行转换 ...
/// // Span结束时自动记录duration
/// ```
#[macro_export]
macro_rules! perf_span {
    ($name:expr $(, $key:ident = $value:expr)*) => {
        tracing::info_span!($name, $($key = ?$value),*).entered()
    };
}

/// 记录验证错误（带错误码）
/// 
/// # 示例
/// ```
/// use crate::logging::*;
/// 
/// log_validation_error!(
///     "PIXLY-CORE-VAL-001",
///     "Quality参数超出范围",
///     quality = 150,
///     expected = "1-100"
/// );
/// ```
#[macro_export]
macro_rules! log_validation_error {
    ($code:expr, $msg:expr $(, $key:ident = $value:expr)*) => {
        {
            let context = serde_json::json!({
                $(stringify!($key): $value,)*
            });
            
            $crate::logging::LogEntry::new("ERROR", "validator", $msg)
                .with_code($code)
                .with_context(context)
                .log_pretty();
        }
    };
}

/// 记录验证警告（带错误码）
#[macro_export]
macro_rules! log_validation_warning {
    ($code:expr, $msg:expr $(, $key:ident = $value:expr)*) => {
        {
            let context = serde_json::json!({
                $(stringify!($key): $value,)*
            });
            
            $crate::logging::LogEntry::new("WARNING", "validator", $msg)
                .with_code($code)
                .with_context(context)
                .log_pretty();
        }
    };
}

/// 记录文件错误（带错误码）
#[macro_export]
macro_rules! log_file_error {
    ($code:expr, $msg:expr $(, $key:ident = $value:expr)*) => {
        {
            let context = serde_json::json!({
                $(stringify!($key): $value,)*
            });
            
            $crate::logging::LogEntry::new("ERROR", "file_handler", $msg)
                .with_code($code)
                .with_context(context)
                .log_pretty();
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_conversion() {
        assert_eq!(LogLevel::from("ERROR") as u8, 0);
        assert_eq!(LogLevel::from("info") as u8, 2);
        assert_eq!(LogLevel::from("unknown") as u8, 2); // 默认INFO
    }

    #[test]
    fn test_logging_macros() {
        // 忽略init错误（可能已经初始化过）
        let _ = std::panic::catch_unwind(|| {
            init_logging("DEBUG", false);
        });
        
        log_info!("Test message");
        log_info!("Test with fields", count = 5, name = "test");
        log_debug!("Debug message", value = 42);
    }

    #[test]
    fn test_perf_span() {
        // 忽略init错误（可能已经初始化过）
        let _ = std::panic::catch_unwind(|| {
            init_logging("INFO", false);
        });
        
        let _span = perf_span!("test_operation", input = "test.jpg");
        // 模拟操作
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
