// 🚨 Pixly统一错误码系统
//
// Phase 46.8: 三端统一的错误处理
// Phase 46.14+: 三端统一错误码规范 (2025-11-11)
// 核心原则：响亮报错 > 静默降级
//
// 错误码格式：PIXLY-RUST-{CATEGORY}-{NUMBER}
// - VAL: 验证类错误（参数验证、格式检查）
// - FILE: 文件类错误（读写、损坏）
// - NET: 网络类错误（超时、连接）
// - SYS: 系统类错误（内存、工具缺失）
// - BIZ: 业务类错误（转换、处理）
//
// 参见：docs/architecture/PROJECT_QUALITY_MANIFESTO.md

use serde::{Serialize, Deserialize};
use std::fmt;
use thiserror::Error;

/// 错误严重级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// 致命错误，系统无法继续
    #[serde(rename = "CRITICAL")]
    Critical,
    /// 错误，操作失败
    #[serde(rename = "ERROR")]
    Error,
    /// 警告，潜在问题
    #[serde(rename = "WARNING")]
    Warning,
    /// 信息，仅提示
    #[serde(rename = "INFO")]
    Info,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Critical => write!(f, "CRITICAL"),
            Self::Error => write!(f, "ERROR"),
            Self::Warning => write!(f, "WARNING"),
            Self::Info => write!(f, "INFO"),
        }
    }
}

/// Pixly统一错误类型
#[derive(Error, Debug, Serialize)]
pub struct PixlyError {
    /// 错误码 (PIXLY-RUST-VAL-001)
    pub code: String,
    /// 错误消息（技术详情）
    pub message: String,
    /// 用户友好消息
    pub user_message: String,
    /// 错误严重级别
    pub severity: ErrorSeverity,
    /// 上下文信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

impl fmt::Display for PixlyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl PixlyError {
    /// 创建新错误
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
        user_message: impl Into<String>,
        severity: ErrorSeverity,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            user_message: user_message.into(),
            severity,
            context: None,
        }
    }

    /// 添加上下文信息
    pub fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = Some(context);
        self
    }

    /// 是否应该阻断流程
    pub fn should_block(&self) -> bool {
        matches!(self.severity, ErrorSeverity::Error | ErrorSeverity::Critical)
    }
}

/// 错误码常量
pub mod error_codes {
    // ========== 参数验证错误 (VAL) ==========
    
    /// 参数超出范围
    pub const VAL_OUT_OF_RANGE: &str = "PIXLY-RUST-VAL-001";
    
    /// 参数类型错误
    pub const VAL_TYPE_ERROR: &str = "PIXLY-RUST-VAL-002";
    
    /// 必填参数缺失
    pub const VAL_MISSING_REQUIRED: &str = "PIXLY-RUST-VAL-003";
    
    /// 参数组合冲突
    pub const VAL_CONFLICT: &str = "PIXLY-RUST-VAL-004";
    
    /// AI置信度过低
    pub const VAL_LOW_CONFIDENCE: &str = "PIXLY-RUST-VAL-005";
    
    /// 参数完整性验证失败
    pub const VAL_INTEGRITY_FAILED: &str = "PIXLY-RUST-VAL-006";
    
    // ========== 文件错误 (FILE) ==========
    
    /// 文件不存在
    pub const FILE_NOT_FOUND: &str = "PIXLY-RUST-FILE-001";
    
    /// 文件无法读取
    pub const FILE_READ_ERROR: &str = "PIXLY-RUST-FILE-002";
    
    /// 文件格式不支持
    pub const FILE_FORMAT_UNSUPPORTED: &str = "PIXLY-RUST-FILE-003";
    
    /// 文件已损坏
    pub const FILE_CORRUPTED: &str = "PIXLY-RUST-FILE-004";
    
    /// 文件类型伪装
    pub const FILE_TYPE_MISMATCH: &str = "PIXLY-RUST-FILE-005";
    
    /// 输出文件写入失败
    pub const FILE_WRITE_ERROR: &str = "PIXLY-RUST-FILE-006";
    
    // ========== 系统错误 (SYS) ==========
    
    /// 内存不足
    pub const SYS_OUT_OF_MEMORY: &str = "PIXLY-RUST-SYS-001";
    
    /// 工具未安装
    pub const SYS_TOOL_NOT_FOUND: &str = "PIXLY-RUST-SYS-002";
    
    /// 工具执行失败
    pub const SYS_TOOL_EXECUTION_FAILED: &str = "PIXLY-RUST-SYS-003";
    
    // ========== 业务逻辑错误 (BIZ) ==========
    
    /// 图像尺寸过大
    pub const BIZ_IMAGE_TOO_LARGE: &str = "PIXLY-RUST-BIZ-001";
    
    /// 批量任务失败
    pub const BIZ_BATCH_FAILED: &str = "PIXLY-RUST-BIZ-002";
}

/// 错误构建器
pub struct ErrorBuilder;

impl ErrorBuilder {
    /// 参数超出范围错误
    pub fn val_out_of_range(param: &str, value: impl fmt::Display, expected: &str) -> PixlyError {
        PixlyError::new(
            error_codes::VAL_OUT_OF_RANGE,
            format!("{} 参数超出范围: {} (应为 {})", param, value, expected),
            format!("参数设置有误，请检查 {} 的值（应为 {}）", param, expected),
            ErrorSeverity::Error,
        )
        .with_context(serde_json::json!({
            "parameter": param,
            "value": value.to_string(),
            "expected": expected,
        }))
    }

    /// 必填参数缺失
    pub fn val_missing_required(param: &str) -> PixlyError {
        PixlyError::new(
            error_codes::VAL_MISSING_REQUIRED,
            format!("必填参数缺失: {}", param),
            format!("缺少必要参数：{}", param),
            ErrorSeverity::Error,
        )
        .with_context(serde_json::json!({
            "parameter": param,
        }))
    }

    /// 参数组合冲突
    pub fn val_conflict(reason: impl Into<String>) -> PixlyError {
        let reason_str = reason.into();
        PixlyError::new(
            error_codes::VAL_CONFLICT,
            format!("参数冲突: {}", reason_str),
            format!("参数设置冲突：{}", reason_str),
            ErrorSeverity::Error,
        )
        .with_context(serde_json::json!({
            "reason": reason_str,
        }))
    }

    /// AI置信度过低
    pub fn val_low_confidence(confidence: f64, threshold: f64) -> PixlyError {
        PixlyError::new(
            error_codes::VAL_LOW_CONFIDENCE,
            format!("AI置信度过低: {:.1}% (阈值: {:.1}%)", confidence * 100.0, threshold * 100.0),
            "AI预测不够确定，建议手动设置参数".to_string(),
            ErrorSeverity::Warning,
        )
        .with_context(serde_json::json!({
            "confidence": confidence,
            "threshold": threshold,
        }))
    }

    /// 文件不存在
    pub fn file_not_found(path: impl fmt::Display) -> PixlyError {
        PixlyError::new(
            error_codes::FILE_NOT_FOUND,
            format!("输入文件不存在: {}", path),
            "找不到指定的图像文件".to_string(),
            ErrorSeverity::Error,
        )
        .with_context(serde_json::json!({
            "path": path.to_string(),
        }))
    }

    /// 文件格式不支持
    pub fn file_format_unsupported(format: &str) -> PixlyError {
        PixlyError::new(
            error_codes::FILE_FORMAT_UNSUPPORTED,
            format!("不支持的文件格式: {} (仅支持 jpg/png/webp/avif/jxl 等)", format),
            "文件格式不支持，请使用 jpg、png 或 webp 格式".to_string(),
            ErrorSeverity::Error,
        )
        .with_context(serde_json::json!({
            "format": format,
            "supported": ["jpg", "png", "webp", "avif", "jxl", "gif"],
        }))
    }

    /// 文件类型伪装（警告）
    pub fn file_type_mismatch(extension: &str, actual_type: &str) -> PixlyError {
        PixlyError::new(
            error_codes::FILE_TYPE_MISMATCH,
            format!("文件类型不匹配: 扩展名 {}, 实际为 {}", extension, actual_type),
            "文件类型不匹配，建议修正文件扩展名".to_string(),
            ErrorSeverity::Warning,
        )
        .with_context(serde_json::json!({
            "extension": extension,
            "actual_type": actual_type,
        }))
    }

    /// 工具未安装
    pub fn sys_tool_not_found(tool: &str) -> PixlyError {
        PixlyError::new(
            error_codes::SYS_TOOL_NOT_FOUND,
            format!("转换工具未安装: {}", tool),
            format!("转换工具 {} 未安装，请运行安装脚本", tool),
            ErrorSeverity::Error,
        )
        .with_context(serde_json::json!({
            "tool": tool,
        }))
    }

    /// 工具执行失败
    pub fn sys_tool_execution_failed(tool: &str, exit_code: i32) -> PixlyError {
        PixlyError::new(
            error_codes::SYS_TOOL_EXECUTION_FAILED,
            format!("工具执行失败: {} 返回错误码 {}", tool, exit_code),
            "图像转换失败，请检查图像是否正常".to_string(),
            ErrorSeverity::Error,
        )
        .with_context(serde_json::json!({
            "tool": tool,
            "exit_code": exit_code,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = ErrorBuilder::val_out_of_range("quality", 150, "1-100");
        assert_eq!(err.code, "PIXLY-CORE-VAL-001");
        assert!(err.should_block());
        assert!(err.message.contains("150"));
    }

    #[test]
    fn test_low_confidence_warning() {
        let err = ErrorBuilder::val_low_confidence(0.45, 0.5);
        assert_eq!(err.severity, ErrorSeverity::Warning);
        assert!(!err.should_block()); // 警告不应阻断
    }

    #[test]
    fn test_error_serialization() {
        let err = ErrorBuilder::file_not_found("/test/file.jpg");
        let json = serde_json::to_string(&err).unwrap();
        assert!(json.contains("PIXLY-CORE-FILE-001"));
        assert!(json.contains("/test/file.jpg"));
    }
}
