"""
📝 PIXLY v3.0 结构化日志系统

基于Go废弃模块logging.go的统一日志架构：
- 三端统一格式 (Go/Rust/Python)
- ISO 8601时间戳
- 结构化上下文
- 追踪ID支持
- 本地化输出

迁移自：Go logging.go 的日志格式和逻辑
"""

import time
import json
import threading
import traceback
import sys
import os
from enum import Enum
from dataclasses import dataclass, asdict
from typing import Dict, List, Optional, Any, TextIO
from pathlib import Path
import sqlite3


class LogLevel(Enum):
    """日志级别 - 与Go模块完全一致"""
    DEBUG = "DEBUG"
    INFO = "INFO" 
    WARNING = "WARNING"
    ERROR = "ERROR"


@dataclass
class LogEntry:
    """统一日志条目 - 与Go模块格式一致"""
    timestamp: str          # ISO 8601时间戳
    level: str              # 日志级别
    layer: str              # 层级标识 (python-core, rust-core, etc.)
    component: str          # 组件名称
    message: str            # 日志消息
    code: Optional[str] = None           # 错误码（可选）
    context: Optional[Dict[str, Any]] = None  # 上下文信息
    trace_id: Optional[str] = None       # 追踪ID（可选）
    file: Optional[str] = None           # 源文件
    line: Optional[int] = None           # 行号
    thread_id: Optional[str] = None      # 线程ID
    process_id: Optional[int] = None     # 进程ID
    
    def __post_init__(self):
        if self.context is None:
            self.context = {}
        if self.process_id is None:
            self.process_id = os.getpid()
        if self.thread_id is None:
            self.thread_id = threading.current_thread().name


class LogFormatter:
    """日志格式化器"""
    
    def __init__(self, enable_json: bool = False, 
                 show_file_location: bool = True):
        self.enable_json = enable_json
        self.show_file_location = show_file_location
    
    def format_entry(self, entry: LogEntry) -> str:
        """格式化日志条目"""
        if self.enable_json:
            return self._format_json(entry)
        else:
            return self._format_human_readable(entry)
    
    def _format_json(self, entry: LogEntry) -> str:
        """JSON格式输出"""
        entry_dict = asdict(entry)
        # 移除None值
        entry_dict = {k: v for k, v in entry_dict.items() if v is not None}
        return json.dumps(entry_dict, ensure_ascii=False)
    
    def _format_human_readable(self, entry: LogEntry) -> str:
        """人类可读格式输出"""
        # 时间戳 (简化显示)
        time_str = entry.timestamp.split('T')[1][:12]  # HH:MM:SS.mmm
        
        # 级别标识
        level_icons = {
            "DEBUG": "🔍",
            "INFO": "ℹ️", 
            "WARNING": "⚠️",
            "ERROR": "❌"
        }
        icon = level_icons.get(entry.level, "📝")
        
        # 基本信息
        parts = [
            f"{time_str}",
            f"{icon} {entry.level}",
            f"[{entry.component}]",
            entry.message
        ]
        
        base_msg = " ".join(parts)
        
        # 错误码
        if entry.code:
            base_msg += f" (Code: {entry.code})"
        
        # 追踪ID
        if entry.trace_id:
            base_msg += f" [Trace: {entry.trace_id[:8]}...]"
        
        # 文件位置
        if self.show_file_location and entry.file and entry.line:
            file_name = Path(entry.file).name
            base_msg += f" ({file_name}:{entry.line})"
        
        # 上下文信息 (另起一行)
        lines = [base_msg]
        if entry.context and entry.context:
            context_str = json.dumps(entry.context, ensure_ascii=False, indent=2)
            lines.append(f"   Context: {context_str}")
        
        return "\n".join(lines)


class StructuredLogger:
    """
    📝 企业级结构化日志系统
    
    功能特性：
    - 三端统一格式
    - 多输出目标
    - 本地化存储
    - 性能优化
    """
    
    def __init__(self, layer: str = "python-core", 
                 log_level: LogLevel = LogLevel.INFO,
                 enable_json: bool = False,
                 log_file: Optional[str] = None,
                 db_path: Optional[str] = None,
                 buffer_size: int = 100):
        
        self.layer = layer
        self.log_level = log_level
        self.formatter = LogFormatter(enable_json=enable_json)
        
        # 输出目标
        self.console_output = True
        self.file_output: Optional[TextIO] = None
        self.db_output: Optional[str] = db_path
        
        # 缓冲区 (提升性能)
        self.buffer_size = buffer_size
        self.log_buffer: List[LogEntry] = []
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 当前追踪上下文
        self._local = threading.local()
        
        # 初始化输出
        if log_file:
            self._init_file_output(log_file)
        
        if db_path:
            self._init_db_output(db_path)
    
    def _init_file_output(self, log_file: str):
        """初始化文件输出"""
        try:
            log_path = Path(log_file)
            log_path.parent.mkdir(parents=True, exist_ok=True)
            self.file_output = open(log_file, 'a', encoding='utf-8')
        except Exception as e:
            print(f"⚠️ 初始化日志文件失败: {e}")
    
    def _init_db_output(self, db_path: str):
        """初始化数据库输出"""
        try:
            Path(db_path).parent.mkdir(parents=True, exist_ok=True)
            conn = sqlite3.connect(db_path)
            cursor = conn.cursor()
            
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS logs (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp TEXT NOT NULL,
                    level TEXT NOT NULL,
                    layer TEXT NOT NULL,
                    component TEXT NOT NULL,
                    message TEXT NOT NULL,
                    code TEXT,
                    context TEXT,
                    trace_id TEXT,
                    file TEXT,
                    line INTEGER,
                    thread_id TEXT,
                    process_id INTEGER,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            # 创建索引
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_logs_timestamp ON logs(timestamp)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_logs_level ON logs(level)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_logs_component ON logs(component)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_logs_trace_id ON logs(trace_id)')
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 初始化日志数据库失败: {e}")
            self.db_output = None
    
    def set_trace_id(self, trace_id: str):
        """设置当前线程的追踪ID"""
        self._local.trace_id = trace_id
    
    def get_trace_id(self) -> Optional[str]:
        """获取当前线程的追踪ID"""
        return getattr(self._local, 'trace_id', None)
    
    def clear_trace_id(self):
        """清除当前线程的追踪ID"""
        self._local.trace_id = None
    
    def _should_log(self, level: LogLevel) -> bool:
        """检查是否应该记录此级别的日志"""
        level_priorities = {
            LogLevel.DEBUG: 0,
            LogLevel.INFO: 1,
            LogLevel.WARNING: 2,
            LogLevel.ERROR: 3
        }
        
        return level_priorities[level] >= level_priorities[self.log_level]
    
    def _get_caller_info(self) -> tuple:
        """获取调用者信息"""
        try:
            frame = sys._getframe(3)  # 跳过log方法的调用栈
            file_path = frame.f_code.co_filename
            line_number = frame.f_lineno
            return file_path, line_number
        except:
            return None, None
    
    def _log(self, level: LogLevel, component: str, message: str,
            code: Optional[str] = None, 
            context: Optional[Dict[str, Any]] = None,
            trace_id: Optional[str] = None):
        """内部日志记录方法"""
        
        if not self._should_log(level):
            return
        
        # 获取调用信息
        file_path, line_number = self._get_caller_info()
        
        # 使用传入的trace_id或当前线程的trace_id
        if trace_id is None:
            trace_id = self.get_trace_id()
        
        # 创建日志条目
        entry = LogEntry(
            timestamp=time.strftime("%Y-%m-%dT%H:%M:%S.%f", time.gmtime())[:-3] + "Z",
            level=level.value,
            layer=self.layer,
            component=component,
            message=message,
            code=code,
            context=context,
            trace_id=trace_id,
            file=file_path,
            line=line_number
        )
        
        # 输出日志
        self._output_log(entry)
    
    def _output_log(self, entry: LogEntry):
        """输出日志到各个目标"""
        with self._lock:
            # 添加到缓冲区
            self.log_buffer.append(entry)
            
            # 控制台输出 (立即)
            if self.console_output:
                formatted = self.formatter.format_entry(entry)
                print(formatted)
            
            # 文件输出 (立即)
            if self.file_output:
                try:
                    formatted = self.formatter.format_entry(entry)
                    self.file_output.write(formatted + "\n")
                    self.file_output.flush()
                except Exception as e:
                    print(f"⚠️ 写入日志文件失败: {e}")
            
            # 检查是否需要刷新缓冲区
            if len(self.log_buffer) >= self.buffer_size:
                self._flush_buffer()
    
    def _flush_buffer(self):
        """刷新缓冲区到数据库"""
        if not self.db_output or not self.log_buffer:
            return
        
        try:
            conn = sqlite3.connect(self.db_output)
            cursor = conn.cursor()
            
            for entry in self.log_buffer:
                cursor.execute('''
                    INSERT INTO logs 
                    (timestamp, level, layer, component, message, code, 
                     context, trace_id, file, line, thread_id, process_id)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                ''', (
                    entry.timestamp, entry.level, entry.layer, entry.component,
                    entry.message, entry.code,
                    json.dumps(entry.context) if entry.context else None,
                    entry.trace_id, entry.file, entry.line,
                    entry.thread_id, entry.process_id
                ))
            
            conn.commit()
            conn.close()
            
            # 清空缓冲区
            self.log_buffer.clear()
            
        except Exception as e:
            print(f"⚠️ 刷新日志缓冲区失败: {e}")
    
    def debug(self, component: str, message: str, **kwargs):
        """记录DEBUG级别日志"""
        self._log(LogLevel.DEBUG, component, message, **kwargs)
    
    def info(self, component: str, message: str, **kwargs):
        """记录INFO级别日志"""
        self._log(LogLevel.INFO, component, message, **kwargs)
    
    def warning(self, component: str, message: str, **kwargs):
        """记录WARNING级别日志"""
        self._log(LogLevel.WARNING, component, message, **kwargs)
    
    def error(self, component: str, message: str, **kwargs):
        """记录ERROR级别日志"""
        self._log(LogLevel.ERROR, component, message, **kwargs)
    
    def info_with_context(self, component: str, message: str, 
                         context: Dict[str, Any], **kwargs):
        """记录带上下文的INFO日志"""
        self._log(LogLevel.INFO, component, message, context=context, **kwargs)
    
    def error_with_context(self, component: str, message: str,
                          context: Dict[str, Any], **kwargs):
        """记录带上下文的ERROR日志"""
        self._log(LogLevel.ERROR, component, message, context=context, **kwargs)
    
    def log_exception(self, component: str, message: str, 
                     exception: Exception, **kwargs):
        """记录异常日志"""
        context = {
            "exception_type": type(exception).__name__,
            "exception_message": str(exception),
            "traceback": traceback.format_exc()
        }
        
        # 合并额外上下文
        if "context" in kwargs:
            context.update(kwargs["context"])
            kwargs["context"] = context
        else:
            kwargs["context"] = context
        
        self._log(LogLevel.ERROR, component, message, **kwargs)
    
    def flush(self):
        """强制刷新所有缓冲区"""
        with self._lock:
            self._flush_buffer()
            
            if self.file_output:
                self.file_output.flush()
    
    def close(self):
        """关闭日志系统"""
        with self._lock:
            # 刷新缓冲区
            self._flush_buffer()
            
            # 关闭文件
            if self.file_output:
                self.file_output.close()
                self.file_output = None
    
    def set_level(self, level: LogLevel):
        """设置日志级别"""
        self.log_level = level
    
    def set_json_output(self, enable: bool):
        """设置JSON输出格式"""
        self.formatter.enable_json = enable
    
    def set_console_output(self, enable: bool):
        """设置控制台输出"""
        self.console_output = enable
    
    def get_log_stats(self) -> Dict[str, Any]:
        """获取日志统计"""
        if not self.db_output:
            return {"message": "数据库日志未启用"}
        
        try:
            conn = sqlite3.connect(self.db_output)
            cursor = conn.cursor()
            
            # 总日志数
            cursor.execute("SELECT COUNT(*) FROM logs")
            total_logs = cursor.fetchone()[0]
            
            # 按级别统计
            cursor.execute("""
                SELECT level, COUNT(*) 
                FROM logs 
                GROUP BY level
            """)
            level_stats = dict(cursor.fetchall())
            
            # 按组件统计
            cursor.execute("""
                SELECT component, COUNT(*) 
                FROM logs 
                GROUP BY component 
                ORDER BY COUNT(*) DESC 
                LIMIT 10
            """)
            component_stats = dict(cursor.fetchall())
            
            # 最近1小时日志
            one_hour_ago = time.strftime("%Y-%m-%dT%H:%M:%S", 
                                       time.gmtime(time.time() - 3600))
            cursor.execute("""
                SELECT COUNT(*) 
                FROM logs 
                WHERE timestamp > ?
            """, (one_hour_ago,))
            recent_logs = cursor.fetchone()[0]
            
            conn.close()
            
            return {
                "total_logs": total_logs,
                "level_distribution": level_stats,
                "top_components": component_stats,
                "recent_hour_logs": recent_logs,
                "buffer_size": len(self.log_buffer)
            }
            
        except Exception as e:
            return {"error": f"获取日志统计失败: {e}"}
    
    def search_logs(self, query: str, level: Optional[str] = None,
                   component: Optional[str] = None, 
                   limit: int = 100) -> List[Dict[str, Any]]:
        """搜索日志"""
        if not self.db_output:
            return []
        
        try:
            conn = sqlite3.connect(self.db_output)
            cursor = conn.cursor()
            
            # 构建查询
            conditions = ["message LIKE ?"]
            params = [f"%{query}%"]
            
            if level:
                conditions.append("level = ?")
                params.append(level)
            
            if component:
                conditions.append("component = ?")
                params.append(component)
            
            sql = f"""
                SELECT timestamp, level, component, message, code, context, trace_id
                FROM logs 
                WHERE {' AND '.join(conditions)}
                ORDER BY timestamp DESC 
                LIMIT ?
            """
            params.append(limit)
            
            cursor.execute(sql, params)
            rows = cursor.fetchall()
            
            # 转换为字典列表
            columns = ["timestamp", "level", "component", "message", 
                      "code", "context", "trace_id"]
            results = []
            
            for row in rows:
                log_dict = dict(zip(columns, row))
                # 解析context
                if log_dict["context"]:
                    try:
                        log_dict["context"] = json.loads(log_dict["context"])
                    except:
                        pass
                results.append(log_dict)
            
            conn.close()
            return results
            
        except Exception as e:
            print(f"⚠️ 搜索日志失败: {e}")
            return []


# 全局默认日志实例
_default_logger = None

def get_logger(layer: str = "python-core") -> StructuredLogger:
    """获取默认日志实例"""
    global _default_logger
    
    if _default_logger is None:
        _default_logger = StructuredLogger(
            layer=layer,
            log_file="logs/pixly.log",
            db_path="data/logs.db"
        )
    
    return _default_logger

def setup_logging(layer: str = "python-core", 
                 level: LogLevel = LogLevel.INFO,
                 enable_json: bool = False,
                 log_file: Optional[str] = "logs/pixly.log",
                 db_path: Optional[str] = "data/logs.db") -> StructuredLogger:
    """设置全局日志配置"""
    global _default_logger
    
    _default_logger = StructuredLogger(
        layer=layer,
        log_level=level,
        enable_json=enable_json,
        log_file=log_file,
        db_path=db_path
    )
    
    return _default_logger

# 便捷函数
def debug(component: str, message: str, **kwargs):
    """记录DEBUG日志"""
    get_logger().debug(component, message, **kwargs)

def info(component: str, message: str, **kwargs):
    """记录INFO日志"""
    get_logger().info(component, message, **kwargs)

def warning(component: str, message: str, **kwargs):
    """记录WARNING日志"""
    get_logger().warning(component, message, **kwargs)

def error(component: str, message: str, **kwargs):
    """记录ERROR日志"""
    get_logger().error(component, message, **kwargs)

def log_exception(component: str, message: str, exception: Exception, **kwargs):
    """记录异常日志"""
    get_logger().log_exception(component, message, exception, **kwargs)
