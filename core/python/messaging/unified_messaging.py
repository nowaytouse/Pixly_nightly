"""
统一消息系统 - 跨语言通信架构

基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/messaging.go 重新实现

核心功能:
- 跨语言统一消息传递(Go/Rust/Python/JS)
- 结构化消息格式(类型/级别/来源/组件)
- 进度追踪和错误码管理
- 实时消息广播机制(stdout管道)
- 调试和追踪ID支持

EX-023实现: 从Go废弃代码价值提取 + 架构增强
遵循四高原则：高规范化、高兼容性、高扩展性、高稳定性
"""

import json
import sys
import threading
import time
from datetime import datetime
from typing import Dict, List, Optional, Any, Callable, Union
from dataclasses import dataclass, asdict
from enum import Enum
import uuid
import logging


class MessageType(Enum):
    """消息类型枚举"""
    INFO = "info"
    WARNING = "warning"
    ERROR = "error"
    DEBUG = "debug"
    PROGRESS = "progress"
    STATUS = "status"
    COMMAND = "command"
    RESPONSE = "response"
    
    # 架构增强：新增类型
    METRICS = "metrics"
    EVENT = "event"
    NOTIFICATION = "notification"


class MessageLevel(Enum):
    """消息级别枚举"""
    TRACE = 10
    DEBUG = 20
    INFO = 30
    WARN = 40
    ERROR = 50
    FATAL = 60


@dataclass
class MessageContext:
    """消息上下文"""
    source: str = "unknown"              # 消息来源
    component: str = "system"            # 组件名称
    trace_id: str = ""                   # 追踪ID
    user_id: str = ""                    # 用户ID
    request_id: str = ""                 # 请求ID
    session_id: str = ""                 # 会话ID
    
    # 架构增强：扩展上下文
    timestamp: float = 0.0               # 时间戳
    hostname: str = ""                   # 主机名
    process_id: int = 0                  # 进程ID
    thread_id: int = 0                   # 线程ID
    tags: Dict[str, str] = None          # 自定义标签
    
    def __post_init__(self):
        if self.tags is None:
            self.tags = {}
        if not self.timestamp:
            self.timestamp = time.time()
        if not self.trace_id:
            self.trace_id = str(uuid.uuid4())[:8]


@dataclass
class ProgressMessage:
    """进度消息"""
    current: int = 0                     # 当前进度
    total: int = 100                     # 总量
    stage: str = ""                      # 当前阶段
    message: str = ""                    # 进度消息
    percentage: float = 0.0              # 百分比
    eta_seconds: float = 0.0             # 预计剩余时间
    
    # 架构增强：增强进度信息
    rate_per_second: float = 0.0         # 处理速率
    elapsed_seconds: float = 0.0         # 已用时间
    stage_progress: Dict[str, int] = None # 各阶段进度
    
    def __post_init__(self):
        if self.stage_progress is None:
            self.stage_progress = {}
        if self.total > 0:
            self.percentage = (self.current / self.total) * 100.0


@dataclass
class UnifiedMessage:
    """统一消息格式"""
    type: MessageType = MessageType.INFO
    level: MessageLevel = MessageLevel.INFO
    message: str = ""
    context: MessageContext = None
    data: Dict[str, Any] = None
    
    # 特殊消息字段
    progress: Optional[ProgressMessage] = None
    error_code: str = ""
    error_details: Dict[str, Any] = None
    
    # 架构增强：元数据
    message_id: str = ""
    parent_id: str = ""                  # 父消息ID
    correlation_id: str = ""             # 关联ID
    priority: int = 0                    # 优先级
    ttl_seconds: float = 0               # 生存时间
    retry_count: int = 0                 # 重试次数
    
    def __post_init__(self):
        if self.context is None:
            self.context = MessageContext()
        if self.data is None:
            self.data = {}
        if self.error_details is None:
            self.error_details = {}
        if not self.message_id:
            self.message_id = str(uuid.uuid4())[:12]
        if not self.correlation_id:
            self.correlation_id = self.context.trace_id
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典格式"""
        result = {
            "message_id": self.message_id,
            "type": self.type.value,
            "level": self.level.value,
            "message": self.message,
            "timestamp": datetime.now().isoformat(),
            "context": asdict(self.context),
            "data": self.data
        }
        
        if self.progress:
            result["progress"] = asdict(self.progress)
        
        if self.error_code:
            result["error_code"] = self.error_code
            result["error_details"] = self.error_details
        
        # 架构增强：元数据
        if self.parent_id:
            result["parent_id"] = self.parent_id
        if self.correlation_id != self.context.trace_id:
            result["correlation_id"] = self.correlation_id
        if self.priority:
            result["priority"] = self.priority
        if self.retry_count:
            result["retry_count"] = self.retry_count
        
        return result
    
    def to_json(self) -> str:
        """转换为JSON格式"""
        return json.dumps(self.to_dict(), ensure_ascii=False)


class UnifiedMessaging:
    """
    统一消息系统 - 跨语言通信架构
    
    高规范化、高兼容性、高扩展性、高稳定性实现
    """
    
    def __init__(self, 
                 component: str = "system",
                 debug: bool = False,
                 enable_stdout: bool = True):
        """
        初始化统一消息系统
        
        Args:
            component: 组件名称
            debug: 调试模式
            enable_stdout: 是否启用stdout输出
        """
        self.component = component
        self.debug = debug
        self.enable_stdout = enable_stdout
        self.logger = logging.getLogger(__name__)
        
        # 内部状态
        self._lock = threading.RLock()
        self._message_handlers: List[Callable[[UnifiedMessage], None]] = []
        self._message_history: List[UnifiedMessage] = []
        self._max_history = 1000
        
        # 架构增强：新特性
        self._performance_metrics: Dict[str, Any] = {}
        self._error_rates: Dict[str, int] = {}
        self._message_filters: List[Callable[[UnifiedMessage], bool]] = []
        self._async_handlers: List[Callable] = []
        
        if debug:
            self.logger.setLevel(logging.DEBUG)
            self.logger.debug(f"统一消息系统初始化: {component}")
    
    def emit(self, 
             type_: MessageType,
             message: str,
             level: MessageLevel = MessageLevel.INFO,
             context: Optional[MessageContext] = None,
             data: Optional[Dict[str, Any]] = None,
             **kwargs) -> str:
        """
        发射消息
        
        Args:
            type_: 消息类型
            message: 消息内容
            level: 消息级别
            context: 消息上下文
            data: 附加数据
            **kwargs: 其他参数
            
        Returns:
            消息ID
        """
        # 创建默认上下文
        if context is None:
            context = MessageContext(
                component=self.component,
                process_id=os.getpid() if 'os' in globals() else 0,
                thread_id=threading.get_ident()
            )
        
        # 创建消息对象
        msg = UnifiedMessage(
            type=type_,
            level=level,
            message=message,
            context=context,
            data=data or {}
        )
        
        # 应用kwargs
        for key, value in kwargs.items():
            if hasattr(msg, key):
                setattr(msg, key, value)
        
        # 处理消息
        self._process_message(msg)
        
        return msg.message_id
    
    def info(self, message: str, **kwargs) -> str:
        """发送信息消息"""
        return self.emit(MessageType.INFO, message, MessageLevel.INFO, **kwargs)
    
    def warning(self, message: str, **kwargs) -> str:
        """发送警告消息"""
        return self.emit(MessageType.WARNING, message, MessageLevel.WARN, **kwargs)
    
    def error(self, message: str, error_code: str = "", **kwargs) -> str:
        """发送错误消息"""
        kwargs['error_code'] = error_code
        return self.emit(MessageType.ERROR, message, MessageLevel.ERROR, **kwargs)
    
    def debug(self, message: str, **kwargs) -> str:
        """发送调试消息"""
        if self.debug:
            return self.emit(MessageType.DEBUG, message, MessageLevel.DEBUG, **kwargs)
        return ""
    
    def progress(self, 
                current: int, 
                total: int, 
                stage: str = "", 
                message: str = "",
                **kwargs) -> str:
        """
        发送进度消息
        
        Args:
            current: 当前进度
            total: 总量
            stage: 当前阶段
            message: 进度消息
            **kwargs: 其他参数
            
        Returns:
            消息ID
        """
        progress_data = ProgressMessage(
            current=current,
            total=total,
            stage=stage,
            message=message
        )
        
        kwargs['progress'] = progress_data
        return self.emit(MessageType.PROGRESS, message, MessageLevel.INFO, **kwargs)
    
    def status(self, status: str, data: Optional[Dict[str, Any]] = None, **kwargs) -> str:
        """发送状态消息"""
        kwargs['data'] = data or {}
        return self.emit(MessageType.STATUS, status, MessageLevel.INFO, **kwargs)
    
    def metrics(self, metrics: Dict[str, Any], **kwargs) -> str:
        """发送指标消息"""
        kwargs['data'] = metrics
        return self.emit(MessageType.METRICS, "metrics_update", MessageLevel.INFO, **kwargs)
    
    def event(self, event_name: str, event_data: Dict[str, Any], **kwargs) -> str:
        """发送事件消息"""
        kwargs['data'] = event_data
        return self.emit(MessageType.EVENT, event_name, MessageLevel.INFO, **kwargs)
    
    def _process_message(self, message: UnifiedMessage) -> None:
        """
        处理消息
        
        Args:
            message: 统一消息对象
        """
        try:
            with self._lock:
                # 应用过滤器
                if not all(filter_fn(message) for filter_fn in self._message_filters):
                    return
                
                # 记录到历史
                self._add_to_history(message)
                
                # 更新性能指标
                self._update_metrics(message)
                
                # 输出到stdout（跨语言通信）
                if self.enable_stdout:
                    self._emit_to_stdout(message)
                
                # 触发处理器
                self._trigger_handlers(message)
                
                # 记录日志
                self._log_message(message)
                
        except Exception as e:
            # 避免消息处理本身出错
            self.logger.error(f"消息处理失败: {e}")
    
    def _emit_to_stdout(self, message: UnifiedMessage) -> None:
        """输出到stdout（Go/Rust等可以读取）"""
        try:
            json_str = message.to_json()
            print(f"UNIFIED_MESSAGE: {json_str}", flush=True)
            
        except Exception as e:
            # 静默处理JSON序列化错误
            self.logger.error(f"stdout输出失败: {e}")
    
    def _trigger_handlers(self, message: UnifiedMessage) -> None:
        """触发消息处理器"""
        for handler in self._message_handlers:
            try:
                handler(message)
            except Exception as e:
                self.logger.error(f"消息处理器执行失败: {e}")
    
    def _add_to_history(self, message: UnifiedMessage) -> None:
        """添加到消息历史"""
        self._message_history.append(message)
        
        # 限制历史记录数量
        if len(self._message_history) > self._max_history:
            self._message_history = self._message_history[-self._max_history//2:]
    
    def _update_metrics(self, message: UnifiedMessage) -> None:
        """更新性能指标"""
        msg_type = message.type.value
        
        # 消息计数
        if msg_type not in self._performance_metrics:
            self._performance_metrics[msg_type] = {"count": 0, "last_time": 0}
        
        self._performance_metrics[msg_type]["count"] += 1
        self._performance_metrics[msg_type]["last_time"] = time.time()
        
        # 错误率统计
        if message.level == MessageLevel.ERROR:
            self._error_rates[msg_type] = self._error_rates.get(msg_type, 0) + 1
    
    def _log_message(self, message: UnifiedMessage) -> None:
        """记录到日志系统"""
        log_level = {
            MessageLevel.DEBUG: logging.DEBUG,
            MessageLevel.INFO: logging.INFO,
            MessageLevel.WARN: logging.WARNING,
            MessageLevel.ERROR: logging.ERROR,
            MessageLevel.FATAL: logging.CRITICAL
        }.get(message.level, logging.INFO)
        
        log_msg = f"[{message.context.component}] {message.message}"
        if message.context.trace_id:
            log_msg += f" [trace:{message.context.trace_id}]"
        
        self.logger.log(log_level, log_msg)
    
    def add_handler(self, handler: Callable[[UnifiedMessage], None]) -> None:
        """
        添加消息处理器
        
        Args:
            handler: 处理器函数，接收UnifiedMessage参数
        """
        with self._lock:
            self._message_handlers.append(handler)
    
    def add_filter(self, filter_fn: Callable[[UnifiedMessage], bool]) -> None:
        """
        添加消息过滤器
        
        Args:
            filter_fn: 过滤器函数，返回True表示通过
        """
        with self._lock:
            self._message_filters.append(filter_fn)
    
    def get_history(self, limit: int = 100) -> List[Dict[str, Any]]:
        """
        获取消息历史
        
        Args:
            limit: 最大返回数量
            
        Returns:
            消息历史列表
        """
        with self._lock:
            recent = self._message_history[-limit:] if limit > 0 else self._message_history
            return [msg.to_dict() for msg in recent]
    
    def get_metrics(self) -> Dict[str, Any]:
        """
        获取性能指标
        
        Returns:
            指标字典
        """
        with self._lock:
            return {
                "performance": dict(self._performance_metrics),
                "error_rates": dict(self._error_rates),
                "history_count": len(self._message_history),
                "handlers_count": len(self._message_handlers),
                "filters_count": len(self._message_filters)
            }
    
    def clear_history(self) -> None:
        """清空消息历史"""
        with self._lock:
            self._message_history.clear()
    
    def set_max_history(self, max_size: int) -> None:
        """
        设置最大历史记录数
        
        Args:
            max_size: 最大记录数
        """
        with self._lock:
            self._max_history = max_size
    
    def create_context(self, 
                      source: str = "", 
                      trace_id: str = "",
                      **kwargs) -> MessageContext:
        """
        创建消息上下文
        
        Args:
            source: 消息来源
            trace_id: 追踪ID
            **kwargs: 其他上下文参数
            
        Returns:
            消息上下文对象
        """
        context = MessageContext(
            source=source or self.component,
            component=self.component,
            trace_id=trace_id,
            process_id=os.getpid() if 'os' in globals() else 0,
            thread_id=threading.get_ident()
        )
        
        # 应用其他参数
        for key, value in kwargs.items():
            if hasattr(context, key):
                setattr(context, key, value)
        
        return context


# 全局消息实例（便捷使用）
_global_messaging: Optional[UnifiedMessaging] = None


def init_global_messaging(component: str = "global", debug: bool = False) -> UnifiedMessaging:
    """
    初始化全局消息系统
    
    Args:
        component: 组件名称
        debug: 调试模式
        
    Returns:
        全局消息实例
    """
    global _global_messaging
    _global_messaging = UnifiedMessaging(component, debug)
    return _global_messaging


def get_messaging() -> Optional[UnifiedMessaging]:
    """
    获取全局消息实例
    
    Returns:
        全局消息实例
    """
    return _global_messaging


# 便捷函数
def emit_info(message: str, **kwargs) -> str:
    """发送全局信息消息"""
    if _global_messaging:
        return _global_messaging.info(message, **kwargs)
    return ""


def emit_error(message: str, error_code: str = "", **kwargs) -> str:
    """发送全局错误消息"""
    if _global_messaging:
        return _global_messaging.error(message, error_code, **kwargs)
    return ""


def emit_progress(current: int, total: int, stage: str = "", message: str = "") -> str:
    """发送全局进度消息"""
    if _global_messaging:
        return _global_messaging.progress(current, total, stage, message)
    return ""


import os  # 导入os模块用于getpid


if __name__ == "__main__":
    # 测试代码
    print("=== 统一消息系统测试 ===")
    
    # 初始化消息系统
    messaging = UnifiedMessaging("test_component", debug=True)
    
    # 添加测试处理器
    def test_handler(msg: UnifiedMessage):
        print(f"🔔 处理器收到消息: {msg.type.value} - {msg.message}")
    
    messaging.add_handler(test_handler)
    
    # 测试各种消息类型
    print("\n📤 测试消息发送...")
    messaging.info("测试信息消息")
    messaging.warning("测试警告消息") 
    messaging.error("测试错误消息", error_code="E001")
    messaging.debug("测试调试消息")
    
    # 测试进度消息
    print("\n📊 测试进度消息...")
    for i in range(0, 101, 25):
        messaging.progress(i, 100, "processing", f"处理进度 {i}%")
    
    # 测试状态和事件
    messaging.status("system_ready", {"version": "1.0", "mode": "test"})
    messaging.event("user_login", {"user_id": "test123", "timestamp": time.time()})
    messaging.metrics({"cpu_usage": 45.2, "memory_mb": 512})
    
    # 查看历史和指标
    print(f"\n📋 消息历史: {len(messaging.get_history())}条")
    metrics = messaging.get_metrics()
    print(f"📈 性能指标: {metrics['performance']}")
    
    # 测试全局消息
    print("\n🌐 测试全局消息...")
    init_global_messaging("global_test", debug=True)
    emit_info("全局信息测试")
    emit_progress(50, 100, "global", "全局进度测试")
    
    print("🎯 统一消息系统测试完成！")
