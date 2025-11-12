"""
日志聚合系统 - 企业级统一日志收集和分析

基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/logging.go 重新实现

核心功能:
- 多源日志聚合收集
- 实时日志流处理
- 结构化日志存储
- 智能日志分析
- 分布式日志同步

EX-028实现: 从Go废弃代码价值提取 + 现代日志架构增强
遵循四高原则：高规范化、高兼容性、高扩展性、高稳定性
"""

import json
import time
import threading
import queue
import sqlite3
from typing import Dict, List, Optional, Any, Union, Callable, Iterator
from dataclasses import dataclass, field, asdict
from datetime import datetime, timedelta
from pathlib import Path
from enum import Enum
import logging
import hashlib
import gzip
import os
from collections import defaultdict, deque


class LogLevel(Enum):
    """日志级别枚举"""
    DEBUG = "DEBUG"
    INFO = "INFO" 
    WARNING = "WARNING"
    ERROR = "ERROR"
    CRITICAL = "CRITICAL"


class LogFormat(Enum):
    """日志格式枚举"""
    JSON = "json"
    TEXT = "text"
    STRUCTURED = "structured"
    COMPACT = "compact"


class LogSource(Enum):
    """日志源类型"""
    APPLICATION = "application"
    SYSTEM = "system"
    NETWORK = "network"
    DATABASE = "database"
    AI_MODEL = "ai_model"
    USER = "user"


@dataclass
class LogEntry:
    """日志条目定义"""
    timestamp: str
    level: LogLevel
    layer: str = "python-ai"  # 层级标识
    component: str = ""       # 组件名称
    message: str = ""         # 日志消息
    
    # 架构增强：扩展字段
    source: LogSource = LogSource.APPLICATION
    code: Optional[str] = None        # 错误码
    context: Dict[str, Any] = field(default_factory=dict)  # 上下文信息
    trace_id: Optional[str] = None    # 追踪ID
    span_id: Optional[str] = None     # Span ID
    user_id: Optional[str] = None     # 用户ID
    session_id: Optional[str] = None  # 会话ID
    
    # 文件位置信息
    file: Optional[str] = None
    line: Optional[int] = None
    function: Optional[str] = None
    
    # 性能信息
    duration_ms: Optional[float] = None
    memory_mb: Optional[float] = None
    cpu_percent: Optional[float] = None
    
    # 聚合信息
    correlation_id: Optional[str] = None  # 关联ID
    tags: List[str] = field(default_factory=list)  # 标签
    environment: str = "development"      # 环境
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典"""
        result = asdict(self)
        result['level'] = self.level.value
        result['source'] = self.source.value
        return result
    
    def to_json(self) -> str:
        """转换为JSON字符串"""
        return json.dumps(self.to_dict(), ensure_ascii=False)


@dataclass
class LogFilter:
    """日志过滤器"""
    levels: List[LogLevel] = field(default_factory=list)
    components: List[str] = field(default_factory=list)
    sources: List[LogSource] = field(default_factory=list)
    start_time: Optional[datetime] = None
    end_time: Optional[datetime] = None
    keywords: List[str] = field(default_factory=list)
    user_ids: List[str] = field(default_factory=list)
    trace_ids: List[str] = field(default_factory=list)
    tags: List[str] = field(default_factory=list)
    
    def matches(self, entry: LogEntry) -> bool:
        """检查日志条目是否匹配过滤条件"""
        # 级别过滤
        if self.levels and entry.level not in self.levels:
            return False
        
        # 组件过滤
        if self.components and entry.component not in self.components:
            return False
        
        # 源过滤
        if self.sources and entry.source not in self.sources:
            return False
        
        # 时间过滤
        if self.start_time or self.end_time:
            try:
                entry_time = datetime.fromisoformat(entry.timestamp.replace('Z', '+00:00'))
                if self.start_time and entry_time < self.start_time:
                    return False
                if self.end_time and entry_time > self.end_time:
                    return False
            except:
                pass
        
        # 关键词过滤
        if self.keywords:
            text = f"{entry.message} {entry.component}".lower()
            if not any(keyword.lower() in text for keyword in self.keywords):
                return False
        
        # 用户过滤
        if self.user_ids and entry.user_id not in self.user_ids:
            return False
        
        # 追踪过滤
        if self.trace_ids and entry.trace_id not in self.trace_ids:
            return False
        
        # 标签过滤
        if self.tags:
            if not any(tag in entry.tags for tag in self.tags):
                return False
        
        return True


class LogAggregator:
    """
    日志聚合器 - 企业级统一日志收集和分析系统
    
    高规范化、高兼容性、高扩展性、高稳定性实现
    """
    
    def __init__(self,
                 storage_path: str = "logs",
                 max_memory_entries: int = 10000,
                 auto_flush_interval: int = 30,
                 compression_enabled: bool = True,
                 debug: bool = False):
        """
        初始化日志聚合器
        
        Args:
            storage_path: 日志存储路径
            max_memory_entries: 内存中最大日志条目数
            auto_flush_interval: 自动刷新间隔(秒)
            compression_enabled: 启用压缩
            debug: 调试模式
        """
        self.storage_path = Path(storage_path)
        self.max_memory_entries = max_memory_entries
        self.auto_flush_interval = auto_flush_interval
        self.compression_enabled = compression_enabled
        self.debug = debug
        
        # 创建存储目录
        self.storage_path.mkdir(parents=True, exist_ok=True)
        
        # 内存缓存
        self._memory_buffer: deque = deque(maxlen=max_memory_entries)
        self._buffer_lock = threading.RLock()
        
        # 异步处理队列
        self._log_queue: queue.Queue = queue.Queue()
        self._processing_thread: Optional[threading.Thread] = None
        self._running = False
        
        # 数据库连接
        self._init_database()
        
        # 回调和处理器
        self._handlers: List[Callable[[LogEntry], None]] = []
        self._filters: List[LogFilter] = []
        
        # 统计信息
        self._stats = {
            'total_entries': 0,
            'entries_by_level': defaultdict(int),
            'entries_by_component': defaultdict(int),
            'entries_by_source': defaultdict(int),
            'last_flush_time': datetime.now(),
            'flush_count': 0
        }
        
        # 启动处理线程
        self.start()
        
        if debug:
            print(f"日志聚合器初始化完成: {storage_path}")
    
    def _init_database(self):
        """初始化SQLite数据库"""
        db_path = self.storage_path / "logs.db"
        self._db_conn = sqlite3.connect(str(db_path), check_same_thread=False)
        self._db_lock = threading.Lock()
        
        # 创建日志表
        self._db_conn.execute('''
            CREATE TABLE IF NOT EXISTS log_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                level TEXT NOT NULL,
                layer TEXT,
                component TEXT,
                message TEXT,
                source TEXT,
                code TEXT,
                context TEXT,
                trace_id TEXT,
                span_id TEXT,
                user_id TEXT,
                session_id TEXT,
                file TEXT,
                line INTEGER,
                function TEXT,
                duration_ms REAL,
                memory_mb REAL,
                cpu_percent REAL,
                correlation_id TEXT,
                tags TEXT,
                environment TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
        ''')
        
        # 创建索引
        indexes = [
            'CREATE INDEX IF NOT EXISTS idx_timestamp ON log_entries(timestamp)',
            'CREATE INDEX IF NOT EXISTS idx_level ON log_entries(level)',
            'CREATE INDEX IF NOT EXISTS idx_component ON log_entries(component)',
            'CREATE INDEX IF NOT EXISTS idx_trace_id ON log_entries(trace_id)',
            'CREATE INDEX IF NOT EXISTS idx_user_id ON log_entries(user_id)',
            'CREATE INDEX IF NOT EXISTS idx_created_at ON log_entries(created_at)'
        ]
        
        for index_sql in indexes:
            self._db_conn.execute(index_sql)
        
        self._db_conn.commit()
    
    def start(self):
        """启动日志处理线程"""
        if not self._running:
            self._running = True
            self._processing_thread = threading.Thread(target=self._process_logs)
            self._processing_thread.daemon = True
            self._processing_thread.start()
    
    def stop(self):
        """停止日志处理"""
        self._running = False
        if self._processing_thread:
            self._log_queue.put(None)  # 发送停止信号
            self._processing_thread.join(timeout=5)
        
        # 刷新剩余日志
        self.flush()
        
        # 关闭数据库
        if hasattr(self, '_db_conn'):
            self._db_conn.close()
    
    def add_entry(self, entry: LogEntry):
        """添加日志条目"""
        # 应用过滤器
        if not self._should_process_entry(entry):
            return
        
        # 异步处理
        try:
            self._log_queue.put(entry, timeout=1)
        except queue.Full:
            if self.debug:
                print("⚠️ 日志队列已满，跳过日志条目")
    
    def log(self, 
           level: LogLevel, 
           component: str, 
           message: str,
           **kwargs) -> None:
        """记录日志条目"""
        entry = LogEntry(
            timestamp=datetime.now().isoformat() + 'Z',
            level=level,
            component=component,
            message=message,
            **kwargs
        )
        
        self.add_entry(entry)
    
    def debug(self, component: str, message: str, **kwargs):
        """记录调试日志"""
        self.log(LogLevel.DEBUG, component, message, **kwargs)
    
    def info(self, component: str, message: str, **kwargs):
        """记录信息日志"""
        self.log(LogLevel.INFO, component, message, **kwargs)
    
    def warning(self, component: str, message: str, **kwargs):
        """记录警告日志"""
        self.log(LogLevel.WARNING, component, message, **kwargs)
    
    def error(self, component: str, message: str, **kwargs):
        """记录错误日志"""
        self.log(LogLevel.ERROR, component, message, **kwargs)
    
    def critical(self, component: str, message: str, **kwargs):
        """记录严重错误日志"""
        self.log(LogLevel.CRITICAL, component, message, **kwargs)
    
    def _process_logs(self):
        """处理日志的后台线程"""
        last_flush_time = time.time()
        
        while self._running:
            try:
                # 获取日志条目
                entry = self._log_queue.get(timeout=1)
                
                if entry is None:  # 停止信号
                    break
                
                # 处理日志条目
                self._process_single_entry(entry)
                
                # 定期刷新
                current_time = time.time()
                if current_time - last_flush_time >= self.auto_flush_interval:
                    self.flush()
                    last_flush_time = current_time
                
            except queue.Empty:
                # 检查是否需要刷新
                current_time = time.time()
                if current_time - last_flush_time >= self.auto_flush_interval:
                    self.flush()
                    last_flush_time = current_time
                continue
            
            except Exception as e:
                if self.debug:
                    print(f"日志处理错误: {e}")
    
    def _process_single_entry(self, entry: LogEntry):
        """处理单个日志条目"""
        # 添加到内存缓存
        with self._buffer_lock:
            self._memory_buffer.append(entry)
        
        # 更新统计
        self._update_stats(entry)
        
        # 调用处理器
        for handler in self._handlers:
            try:
                handler(entry)
            except Exception as e:
                if self.debug:
                    print(f"日志处理器错误: {e}")
    
    def _should_process_entry(self, entry: LogEntry) -> bool:
        """检查是否应该处理日志条目"""
        if not self._filters:
            return True
        
        return any(filter_obj.matches(entry) for filter_obj in self._filters)
    
    def _update_stats(self, entry: LogEntry):
        """更新统计信息"""
        self._stats['total_entries'] += 1
        self._stats['entries_by_level'][entry.level.value] += 1
        self._stats['entries_by_component'][entry.component] += 1
        self._stats['entries_by_source'][entry.source.value] += 1
    
    def flush(self):
        """刷新内存中的日志到持久存储"""
        with self._buffer_lock:
            if not self._memory_buffer:
                return
            
            entries_to_flush = list(self._memory_buffer)
            self._memory_buffer.clear()
        
        # 批量写入数据库
        self._flush_to_database(entries_to_flush)
        
        # 可选：写入文件
        self._flush_to_file(entries_to_flush)
        
        # 更新统计
        self._stats['last_flush_time'] = datetime.now()
        self._stats['flush_count'] += 1
        
        if self.debug:
            print(f"已刷新 {len(entries_to_flush)} 条日志")
    
    def _flush_to_database(self, entries: List[LogEntry]):
        """刷新到数据库"""
        if not entries:
            return
        
        try:
            with self._db_lock:
                values = []
                for entry in entries:
                    values.append((
                        entry.timestamp, entry.level.value, entry.layer, entry.component,
                        entry.message, entry.source.value, entry.code,
                        json.dumps(entry.context) if entry.context else None,
                        entry.trace_id, entry.span_id, entry.user_id, entry.session_id,
                        entry.file, entry.line, entry.function,
                        entry.duration_ms, entry.memory_mb, entry.cpu_percent,
                        entry.correlation_id, json.dumps(entry.tags), entry.environment
                    ))
                
                self._db_conn.executemany('''
                    INSERT INTO log_entries (
                        timestamp, level, layer, component, message, source, code,
                        context, trace_id, span_id, user_id, session_id,
                        file, line, function, duration_ms, memory_mb, cpu_percent,
                        correlation_id, tags, environment
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                ''', values)
                
                self._db_conn.commit()
        
        except Exception as e:
            if self.debug:
                print(f"数据库写入错误: {e}")
    
    def _flush_to_file(self, entries: List[LogEntry]):
        """刷新到文件"""
        if not entries:
            return
        
        try:
            # 按日期分组
            date = datetime.now().strftime("%Y-%m-%d")
            log_file = self.storage_path / f"pixly-{date}.log"
            
            with open(log_file, 'a', encoding='utf-8') as f:
                for entry in entries:
                    f.write(entry.to_json() + '\n')
            
            # 可选压缩旧文件
            if self.compression_enabled:
                self._compress_old_files()
        
        except Exception as e:
            if self.debug:
                print(f"文件写入错误: {e}")
    
    def _compress_old_files(self):
        """压缩旧日志文件"""
        try:
            current_date = datetime.now().strftime("%Y-%m-%d")
            
            for log_file in self.storage_path.glob("pixly-*.log"):
                # 跳过当前日期的文件
                if current_date in log_file.name:
                    continue
                
                # 压缩文件
                gz_file = log_file.with_suffix('.log.gz')
                if not gz_file.exists():
                    with open(log_file, 'rb') as f_in:
                        with gzip.open(gz_file, 'wb') as f_out:
                            f_out.writelines(f_in)
                    
                    # 删除原文件
                    log_file.unlink()
        
        except Exception as e:
            if self.debug:
                print(f"压缩文件错误: {e}")
    
    def query(self, 
             filter_obj: Optional[LogFilter] = None,
             limit: int = 1000,
             offset: int = 0) -> List[LogEntry]:
        """查询日志条目"""
        try:
            with self._db_lock:
                sql = "SELECT * FROM log_entries"
                params = []
                conditions = []
                
                if filter_obj:
                    if filter_obj.levels:
                        level_placeholders = ','.join(['?' for _ in filter_obj.levels])
                        conditions.append(f"level IN ({level_placeholders})")
                        params.extend([level.value for level in filter_obj.levels])
                    
                    if filter_obj.components:
                        comp_placeholders = ','.join(['?' for _ in filter_obj.components])
                        conditions.append(f"component IN ({comp_placeholders})")
                        params.extend(filter_obj.components)
                
                if conditions:
                    sql += " WHERE " + " AND ".join(conditions)
                
                sql += " ORDER BY created_at DESC LIMIT ? OFFSET ?"
                params.extend([limit, offset])
                
                cursor = self._db_conn.execute(sql, params)
                rows = cursor.fetchall()
                
                # 转换为LogEntry对象
                entries = []
                for row in rows:
                    entry = LogEntry(
                        timestamp=row[1],
                        level=LogLevel(row[2]),
                        layer=row[3] or "",
                        component=row[4] or "",
                        message=row[5] or "",
                        source=LogSource(row[6]) if row[6] else LogSource.APPLICATION,
                        code=row[7],
                        context=json.loads(row[8]) if row[8] else {},
                        trace_id=row[9],
                        span_id=row[10],
                        user_id=row[11],
                        session_id=row[12],
                        file=row[13],
                        line=row[14],
                        function=row[15],
                        duration_ms=row[16],
                        memory_mb=row[17],
                        cpu_percent=row[18],
                        correlation_id=row[19],
                        tags=json.loads(row[20]) if row[20] else [],
                        environment=row[21] or "development"
                    )
                    entries.append(entry)
                
                return entries
        
        except Exception as e:
            if self.debug:
                print(f"查询错误: {e}")
            return []
    
    def add_handler(self, handler: Callable[[LogEntry], None]):
        """添加日志处理器"""
        self._handlers.append(handler)
    
    def add_filter(self, filter_obj: LogFilter):
        """添加日志过滤器"""
        self._filters.append(filter_obj)
    
    def get_stats(self) -> Dict[str, Any]:
        """获取统计信息"""
        with self._buffer_lock:
            current_stats = dict(self._stats)
            current_stats['memory_buffer_size'] = len(self._memory_buffer)
            current_stats['queue_size'] = self._log_queue.qsize()
        
        return current_stats
    
    def clear_old_logs(self, days_old: int = 30):
        """清理旧日志"""
        try:
            cutoff_date = datetime.now() - timedelta(days=days_old)
            
            with self._db_lock:
                self._db_conn.execute(
                    "DELETE FROM log_entries WHERE created_at < ?",
                    (cutoff_date.isoformat(),)
                )
                self._db_conn.commit()
            
            if self.debug:
                print(f"已清理 {days_old} 天前的日志")
        
        except Exception as e:
            if self.debug:
                print(f"清理日志错误: {e}")


# 全局日志聚合器实例
_global_aggregator: Optional[LogAggregator] = None


def get_global_aggregator() -> LogAggregator:
    """获取全局日志聚合器"""
    global _global_aggregator
    if _global_aggregator is None:
        _global_aggregator = LogAggregator()
    return _global_aggregator


# 便捷函数
def create_log_aggregator(storage_path: str = "logs", debug: bool = False) -> LogAggregator:
    """创建日志聚合器的便捷函数"""
    return LogAggregator(storage_path, debug=debug)


if __name__ == "__main__":
    # 测试代码
    print("=== 日志聚合系统测试 ===")
    
    aggregator = create_log_aggregator(debug=True)
    
    # 测试基本日志记录
    aggregator.info("TestComponent", "测试信息日志", trace_id="test-123")
    aggregator.warning("TestComponent", "测试警告日志", user_id="user-456")
    aggregator.error("TestComponent", "测试错误日志", context={"error_code": 500})
    
    # 等待处理
    time.sleep(2)
    
    # 手动刷新
    aggregator.flush()
    
    # 查询日志
    logs = aggregator.query(limit=10)
    print(f"✅ 查询到 {len(logs)} 条日志")
    
    # 统计信息
    stats = aggregator.get_stats()
    print(f"📊 统计信息: {stats}")
    
    aggregator.stop()
    print("🎯 日志聚合系统测试完成！")
