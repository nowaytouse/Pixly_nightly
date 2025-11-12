#!/usr/bin/env python3
"""
Unified Messaging System - 统一消息传递系统 (Python端)

Phase 46.14+: 三端统一消息显示

与Rust/Go保持一致的消息格式
"""

import json
import sys
import time
from enum import Enum
from typing import Optional, Dict, Any
from dataclasses import dataclass, field, asdict
import hashlib

class MessageType(str, Enum):
    """消息类型"""
    INFO = "info"
    WARNING = "warning"
    ERROR = "error"
    SUCCESS = "success"
    PROGRESS = "progress"
    STATUS = "status"

class MessageLevel(int, Enum):
    """消息级别"""
    DEBUG = 0
    INFO = 1
    WARNING = 2
    ERROR = 3
    CRITICAL = 4

@dataclass
class UnifiedMessage:
    """统一消息结构"""
    id: str
    type: MessageType
    level: MessageLevel
    source: str  # "python-ml"
    component: str
    message: str
    timestamp: int
    progress: Optional[int] = None
    data: Optional[Dict[str, Any]] = None
    error_code: Optional[str] = None
    trace_id: Optional[str] = None
    
    @staticmethod
    def generate_id() -> str:
        """生成消息ID"""
        timestamp = str(time.time_ns())
        hash_obj = hashlib.md5(timestamp.encode())
        return f"msg_{hash_obj.hexdigest()[:16]}"
    
    @classmethod
    def create(cls, msg_type: MessageType, level: MessageLevel, 
               component: str, message: str) -> 'UnifiedMessage':
        """创建新消息"""
        return cls(
            id=cls.generate_id(),
            type=msg_type,
            level=level,
            source="python-ml",
            component=component,
            message=message,
            timestamp=int(time.time())
        )
    
    @classmethod
    def info(cls, component: str, message: str) -> 'UnifiedMessage':
        """创建信息消息"""
        return cls.create(MessageType.INFO, MessageLevel.INFO, component, message)
    
    @classmethod
    def warning(cls, component: str, message: str) -> 'UnifiedMessage':
        """创建警告消息"""
        return cls.create(MessageType.WARNING, MessageLevel.WARNING, component, message)
    
    @classmethod
    def error(cls, component: str, message: str) -> 'UnifiedMessage':
        """创建错误消息"""
        return cls.create(MessageType.ERROR, MessageLevel.ERROR, component, message)
    
    @classmethod
    def success(cls, component: str, message: str) -> 'UnifiedMessage':
        """创建成功消息"""
        return cls.create(MessageType.SUCCESS, MessageLevel.INFO, component, message)
    
    @classmethod
    def progress(cls, component: str, message: str, progress: int) -> 'UnifiedMessage':
        """创建进度消息"""
        msg = cls.create(MessageType.PROGRESS, MessageLevel.INFO, component, message)
        msg.progress = min(progress, 100)
        return msg
    
    def with_error_code(self, code: str) -> 'UnifiedMessage':
        """设置错误码"""
        self.error_code = code
        return self
    
    def with_trace_id(self, trace_id: str) -> 'UnifiedMessage':
        """设置追踪ID"""
        self.trace_id = trace_id
        return self
    
    def with_data(self, data: Dict[str, Any]) -> 'UnifiedMessage':
        """设置附加数据"""
        self.data = data
        return self
    
    def to_json(self) -> str:
        """转换为JSON字符串"""
        # 确保progress是数值而不是方法
        progress_value = None
        if self.progress is not None:
            if callable(self.progress):
                # 如果是方法，不包含在JSON中
                progress_value = None
            else:
                progress_value = self.progress
        
        data = {
            'id': self.id,
            'timestamp': self.timestamp,
            'component': self.component,
            'level': self.level.value,
            'type': self.type.value,
            'message': self.message,
            'progress': progress_value,
            'error_code': self.error_code,
            'trace_id': self.trace_id,
            'source': self.source
        }
        return json.dumps(data)
    
    @classmethod
    def from_json(cls, json_str: str) -> 'UnifiedMessage':
        """从JSON解析"""
        data = json.loads(json_str)
        # 转换字符串为Enum
        data['type'] = MessageType(data['type'])
        data['level'] = MessageLevel(data['level'])
        return cls(**data)
    
    def emit(self):
        """发送消息（输出到stdout供其他端读取）"""
        # 输出到stdout，前缀PIXLY_MSG:供解析
        print(f"PIXLY_MSG:{self.to_json()}", file=sys.stdout, flush=True)
        
        # 同时记录到stderr（日志）
        if self.level == MessageLevel.DEBUG:
            print(f"🔍 [{self.component}] {self.message}", file=sys.stderr)
        elif self.level == MessageLevel.INFO:
            print(f"✅ [{self.component}] {self.message}", file=sys.stderr)
        elif self.level == MessageLevel.WARNING:
            print(f"⚠️  [{self.component}] {self.message}", file=sys.stderr)
        elif self.level in (MessageLevel.ERROR, MessageLevel.CRITICAL):
            print(f"❌ [{self.component}] {self.message}", file=sys.stderr)

# MessageChannel已移除 - 过度设计，直接使用快捷函数

# 快捷函数

def send_info(component: str, message: str):
    """发送信息消息"""
    UnifiedMessage.info(component, message).emit()

def send_warning(component: str, message: str):
    """发送警告消息"""
    UnifiedMessage.warning(component, message).emit()

def send_error(component: str, message: str, error_code: Optional[str] = None):
    """发送错误消息"""
    msg = UnifiedMessage.error(component, message)
    if error_code:
        msg = msg.with_error_code(error_code)
    msg.emit()

def send_success(component: str, message: str):
    """发送成功消息"""
    UnifiedMessage.success(component, message).emit()

def send_progress(component: str, message: str, progress: int):
    """发送进度消息"""
    UnifiedMessage.progress(component, message, progress).emit()

# MessageReader已移除 - 当前未使用进程间通信

# 示例用法
if __name__ == "__main__":
    # 使用快捷函数
    send_info("TestComponent", "测试信息消息")
    send_warning("TestComponent", "测试警告消息")
    send_error("TestComponent", "测试错误消息", "PIXLY-PYTHON-TEST-001")
    send_success("TestComponent", "测试成功消息")
    send_progress("TestComponent", "处理中...", 50)
