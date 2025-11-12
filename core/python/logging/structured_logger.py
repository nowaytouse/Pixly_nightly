"""
结构化日志器 - 基于废弃Go logging.go的Python实现

提供与Go版本兼容的结构化日志功能：
- 多级别日志 (Debug, Info, Warning, Error)
- JSON和人类可读格式
- 上下文信息和追踪ID
- 性能日志记录

基于 @deprecated/go_ai_service_2025_11_11/ai 2/logging.go
"""

import json
import time
import threading
from typing import Dict, List, Optional, Any, Union
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
import logging
import inspect
import traceback


class LogLevel(Enum):
    """日志级别 - 对应Go版本"""
    DEBUG = "DEBUG"
    INFO = "INFO"
    WARNING = "WARNING"  
    ERROR = "ERROR"
    CRITICAL = "CRITICAL"


@dataclass
class LogContext:
    """日志上下文 - 对应Go LogEntry"""
    trace_id: Optional[str] = None
    span_id: Optional[str] = None
    user_id: Optional[str] = None
    session_id: Optional[str] = None
    component: str = ""
    operation: str = ""
    metadata: Dict[str, Any] = field(default_factory=dict)


class StructuredLogger:
    """
    结构化日志器 - Go logging.go的Python实现
    
    提供与废弃Go版本兼容的日志功能
    """
    
    def __init__(self, 
                 component: str = "",
                 output_format: str = "json",  # "json" 或 "human"
                 enable_caller_info: bool = True,
                 debug: bool = False):
        """
        初始化结构化日志器
        
        Args:
            component: 组件名称
            output_format: 输出格式 (json/human)
            enable_caller_info: 启用调用者信息
            debug: 调试模式
        """
        self.component = component
        self.output_format = output_format
        self.enable_caller_info = enable_caller_info
        self.debug = debug
        
        # 日志上下文
        self._context: LogContext = LogContext(component=component)
        self._context_lock = threading.local()
        
        # Python标准日志器
        self._logger = logging.getLogger(f"structured.{component}")
        self._logger.setLevel(logging.DEBUG if debug else logging.INFO)
        
        # 设置格式化器
        self._setup_formatter()
    
    def _setup_formatter(self):
        """设置日志格式化器"""
        if not self._logger.handlers:
            handler = logging.StreamHandler()
            
            if self.output_format == "json":
                formatter = logging.Formatter('%(message)s')
            else:
                # 人类可读格式
                formatter = logging.Formatter(
                    '%(asctime)s [%(levelname)s] %(name)s: %(message)s'
                )
            
            handler.setFormatter(formatter)
            self._logger.addHandler(handler)
    
    def set_context(self, context: LogContext):
        """设置日志上下文"""
        self._context = context
    
    def with_context(self, **kwargs) -> 'StructuredLogger':
        """创建带上下文的日志器副本"""
        new_logger = StructuredLogger(
            component=self.component,
            output_format=self.output_format,
            enable_caller_info=self.enable_caller_info,
            debug=self.debug
        )
        
        # 复制并更新上下文
        new_context = LogContext(
            trace_id=self._context.trace_id,
            span_id=self._context.span_id,
            user_id=self._context.user_id,
            session_id=self._context.session_id,
            component=self._context.component,
            operation=self._context.operation,
            metadata=dict(self._context.metadata)
        )
        
        # 更新上下文
        for key, value in kwargs.items():
            if hasattr(new_context, key):
                setattr(new_context, key, value)
            else:
                new_context.metadata[key] = value
        
        new_logger.set_context(new_context)
        return new_logger
    
    def _create_log_entry(self, 
                         level: LogLevel, 
                         message: str,
                         **extra_fields) -> Dict[str, Any]:
        """创建日志条目 - 对应Go LogEntry结构"""
        entry = {
            "timestamp": datetime.now().isoformat() + 'Z',
            "level": level.value,
            "component": self._context.component,
            "message": message,
            "layer": "python-ai"  # 对应Go版本的Layer字段
        }
        
        # 添加上下文信息
        if self._context.trace_id:
            entry["trace_id"] = self._context.trace_id
        if self._context.span_id:
            entry["span_id"] = self._context.span_id
        if self._context.user_id:
            entry["user_id"] = self._context.user_id
        if self._context.session_id:
            entry["session_id"] = self._context.session_id
        if self._context.operation:
            entry["operation"] = self._context.operation
        
        # 添加调用者信息 (对应Go版本的File/Line)
        if self.enable_caller_info:
            caller_info = self._get_caller_info()
            if caller_info:
                entry.update(caller_info)
        
        # 添加额外字段
        if self._context.metadata:
            entry["metadata"] = self._context.metadata
        
        if extra_fields:
            entry.update(extra_fields)
        
        return entry
    
    def _get_caller_info(self) -> Optional[Dict[str, Any]]:
        """获取调用者信息"""
        try:
            frame = inspect.currentframe()
            # 向上找到非日志器的调用者
            for _ in range(5):  # 最多向上5层
                frame = frame.f_back
                if frame is None:
                    break
                
                filename = frame.f_code.co_filename
                if not filename.endswith(('structured_logger.py', 'log_aggregator.py')):
                    return {
                        "file": filename,
                        "line": frame.f_lineno,
                        "function": frame.f_code.co_name
                    }
        except:
            pass
        
        return None
    
    def _log(self, level: LogLevel, message: str, **kwargs):
        """内部日志方法"""
        entry = self._create_log_entry(level, message, **kwargs)
        
        if self.output_format == "json":
            log_message = json.dumps(entry, ensure_ascii=False)
        else:
            # 人类可读格式
            context_info = ""
            if entry.get("trace_id"):
                context_info += f"[{entry['trace_id']}] "
            if entry.get("operation"):
                context_info += f"({entry['operation']}) "
            
            log_message = f"{context_info}{message}"
        
        # 使用Python标准日志器输出
        if level == LogLevel.DEBUG:
            self._logger.debug(log_message)
        elif level == LogLevel.INFO:
            self._logger.info(log_message)
        elif level == LogLevel.WARNING:
            self._logger.warning(log_message)
        elif level == LogLevel.ERROR:
            self._logger.error(log_message)
        elif level == LogLevel.CRITICAL:
            self._logger.critical(log_message)
    
    # 对应Go版本的日志方法
    def debug(self, message: str, **kwargs):
        """调试日志 - 对应Go Debug()"""
        self._log(LogLevel.DEBUG, message, **kwargs)
    
    def info(self, message: str, **kwargs):
        """信息日志 - 对应Go Info()"""
        self._log(LogLevel.INFO, message, **kwargs)
    
    def warning(self, message: str, **kwargs):
        """警告日志 - 对应Go Warning()"""
        self._log(LogLevel.WARNING, **kwargs)
    
    def error(self, message: str, **kwargs):
        """错误日志 - 对应Go Error()"""
        self._log(LogLevel.ERROR, message, **kwargs)
    
    def critical(self, message: str, **kwargs):
        """严重错误日志"""
        self._log(LogLevel.CRITICAL, message, **kwargs)


class PerformanceLogger:
    """
    性能日志器 - 对应Go PerformanceLogger
    
    用于记录操作耗时和性能指标
    """
    
    def __init__(self, logger: StructuredLogger):
        """
        初始化性能日志器
        
        Args:
            logger: 结构化日志器实例
        """
        self.logger = logger
        self._timers: Dict[str, float] = {}
        self._lock = threading.RLock()
    
    def start_timer(self, operation: str) -> str:
        """开始计时 - 对应Go StartTimer()"""
        timer_id = f"{operation}_{int(time.time() * 1000000)}"
        
        with self._lock:
            self._timers[timer_id] = time.perf_counter()
        
        return timer_id
    
    def end_timer(self, timer_id: str, operation: str = "", **metadata):
        """结束计时并记录 - 对应Go EndTimer()"""
        end_time = time.perf_counter()
        
        with self._lock:
            start_time = self._timers.pop(timer_id, None)
        
        if start_time is None:
            self.logger.warning(f"计时器不存在: {timer_id}")
            return
        
        duration_ms = (end_time - start_time) * 1000
        
        # 记录性能日志
        self.logger.info(
            f"性能统计: {operation}",
            duration_ms=duration_ms,
            timer_id=timer_id,
            performance=True,
            **metadata
        )
        
        return duration_ms
    
    def log_performance(self, 
                       operation: str,
                       duration_ms: float,
                       **metadata):
        """直接记录性能数据"""
        self.logger.info(
            f"性能统计: {operation}",
            duration_ms=duration_ms,
            performance=True,
            **metadata
        )
    
    def measure(self, operation: str):
        """上下文管理器用法 - 自动计时"""
        class TimerContext:
            def __init__(self, perf_logger: PerformanceLogger, op: str):
                self.perf_logger = perf_logger
                self.operation = op
                self.timer_id = None
            
            def __enter__(self):
                self.timer_id = self.perf_logger.start_timer(self.operation)
                return self
            
            def __exit__(self, exc_type, exc_val, exc_tb):
                if self.timer_id:
                    self.perf_logger.end_timer(self.timer_id, self.operation)
        
        return TimerContext(self, operation)


# 便捷函数 - 对应Go版本的全局函数
def get_logger(component: str = "", output_format: str = "json") -> StructuredLogger:
    """获取结构化日志器"""
    return StructuredLogger(component, output_format)


def get_performance_logger(component: str = "") -> PerformanceLogger:
    """获取性能日志器"""
    logger = get_logger(component)
    return PerformanceLogger(logger)


if __name__ == "__main__":
    # 测试代码
    print("=== 结构化日志器测试 ===")
    
    # 基础日志测试
    logger = get_logger("test_component", "human")
    logger = logger.with_context(trace_id="trace-123", operation="test_operation")
    
    logger.info("测试信息日志", user_id="user-456")
    logger.warning("测试警告日志", error_code="WARN_001")
    logger.error("测试错误日志", error_details={"code": 500, "message": "Internal Error"})
    
    # 性能日志测试
    perf_logger = get_performance_logger("test_performance")
    
    with perf_logger.measure("test_operation"):
        time.sleep(0.1)  # 模拟操作
    
    print("✅ 结构化日志器测试完成")
