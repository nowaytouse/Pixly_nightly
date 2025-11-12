/**
 * Unified Messaging System - 统一消息传递系统
 * 
 * 🔥 Phase 46.14+: 三端统一消息显示
 * 
 * 功能：
 * - 统一消息格式（进度/状态/错误/通知）
 * - 跨端消息传递（Go ↔ Rust ↔ Python）
 * - 实时进度更新
 * - 多级别消息（Info/Warning/Error/Success）
 * 
 * @module messaging
 */
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
// 🔧 统一日志系统
use tracing::{info, warn, error};

/// 消息类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    /// 信息消息
    Info,
    /// 警告消息
    Warning,
    /// 错误消息
    Error,
    /// 成功消息
    Success,
    /// 进度更新
    Progress,
    /// 状态变更
    Status,
}

/// 消息级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MessageLevel {
    /// 调试（开发用）
    Debug = 0,
    /// 普通信息
    Info = 1,
    /// 警告
    Warning = 2,
    /// 错误
    Error = 3,
    /// 严重错误
    Critical = 4,
}

/// 统一消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMessage {
    /// 消息ID（唯一标识）
    pub id: String,
    
    /// 消息类型
    #[serde(rename = "type")]
    pub msg_type: MessageType,
    
    /// 消息级别
    pub level: MessageLevel,
    
    /// 来源（go-ai/rust-core/python-ml）
    pub source: String,
    
    /// 组件名称
    pub component: String,
    
    /// 消息内容
    pub message: String,
    
    /// 时间戳（Unix时间戳）
    pub timestamp: u64,
    
    /// 进度百分比（0-100，仅Progress类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u8>,
    
    /// 附加数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    
    /// 错误码（仅Error类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    
    /// 追踪ID（用于关联多条消息）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

impl UnifiedMessage {
    /// 创建新消息
    pub fn new(
        msg_type: MessageType,
        level: MessageLevel,
        component: &str,
        message: String,
    ) -> Self {
        let id = Self::generate_id();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            id,
            msg_type,
            level,
            source: "rust-core".to_string(),
            component: component.to_string(),
            message,
            timestamp,
            progress: None,
            data: None,
            error_code: None,
            trace_id: None,
        }
    }
    
    /// 生成消息ID
    fn generate_id() -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros();
        
        let mut hasher = DefaultHasher::new();
        now.hash(&mut hasher);
        
        format!("msg_{:x}", hasher.finish())
    }
    
    /// 创建信息消息
    pub fn info(component: &str, message: String) -> Self {
        Self::new(MessageType::Info, MessageLevel::Info, component, message)
    }
    
    /// 创建警告消息
    pub fn warning(component: &str, message: String) -> Self {
        Self::new(MessageType::Warning, MessageLevel::Warning, component, message)
    }
    
    /// 创建错误消息
    pub fn error(component: &str, message: String) -> Self {
        Self::new(MessageType::Error, MessageLevel::Error, component, message)
    }
    
    /// 创建成功消息
    pub fn success(component: &str, message: String) -> Self {
        Self::new(MessageType::Success, MessageLevel::Info, component, message)
    }
    
    /// 创建进度消息
    pub fn progress(component: &str, message: String, progress: u8) -> Self {
        let mut msg = Self::new(MessageType::Progress, MessageLevel::Info, component, message);
        msg.progress = Some(progress.min(100));
        msg
    }
    
    /// 设置错误码
    pub fn with_error_code(mut self, code: String) -> Self {
        self.error_code = Some(code);
        self
    }
    
    /// 设置追踪ID
    pub fn with_trace_id(mut self, trace_id: String) -> Self {
        self.trace_id = Some(trace_id);
        self
    }
    
    /// 设置附加数据
    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
    
    /// 转换为JSON字符串
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }
    
    /// 从JSON字符串解析
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
    
    /// 发送到标准输出（供其他端读取）
    pub fn emit(&self) {
        // JSON格式输出到stdout，供Go/Python读取
        println!("PIXLY_MSG:{}", self.to_json());
        
        // 同时记录到日志
        match self.level {
            MessageLevel::Debug => {
                tracing::debug!("[{}] {}", self.component, self.message);
            }
            MessageLevel::Info => {
                info!("[{}] {}", self.component, self.message);
            }
            MessageLevel::Warning => {
                warn!("[{}] {}", self.component, self.message);
            }
            MessageLevel::Error | MessageLevel::Critical => {
                error!("[{}] {}", self.component, self.message);
            }
        }
    }
}

// MessageChannel已移除 - 过度设计，直接使用快捷函数即可

/// 快捷函数：发送信息消息
pub fn send_info(component: &str, message: String) {
    UnifiedMessage::info(component, message).emit();
}

/// 快捷函数：发送警告消息
pub fn send_warning(component: &str, message: String) {
    UnifiedMessage::warning(component, message).emit();
}

/// 快捷函数：发送错误消息
pub fn send_error(component: &str, message: String, error_code: Option<String>) {
    let mut msg = UnifiedMessage::error(component, message);
    if let Some(code) = error_code {
        msg = msg.with_error_code(code);
    }
    msg.emit();
}

/// 快捷函数：发送成功消息
pub fn send_success(component: &str, message: String) {
    UnifiedMessage::success(component, message).emit();
}

/// 快捷函数：发送进度消息
pub fn send_progress(component: &str, message: String, progress: u8) {
    UnifiedMessage::progress(component, message, progress).emit();
}
