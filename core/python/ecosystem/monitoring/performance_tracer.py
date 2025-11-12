"""
📈 PIXLY v3.0 性能分析与追踪引擎

基于Go废弃模块的企业级性能分析架构：
- 分布式链路追踪
- 实时性能分析
- 热点代码识别
- 性能瓶颈诊断
- 自动优化建议

迁移自：Go http_gateway.go 的性能监控逻辑
"""

import time
import uuid
import json
import threading
import traceback
import functools
import cProfile
import pstats
from collections import defaultdict, deque
from dataclasses import dataclass, asdict
from typing import Dict, List, Optional, Any, Callable, Set, Union
from contextlib import contextmanager
from concurrent.futures import ThreadPoolExecutor
import io
import sqlite3

from .metrics_collector import MetricsCollector


@dataclass
class TraceSpan:
    """追踪跨度"""
    trace_id: str
    span_id: str
    parent_span_id: Optional[str]
    operation_name: str
    start_time: float
    end_time: Optional[float] = None
    duration_ms: Optional[float] = None
    tags: Dict[str, Any] = None
    logs: List[Dict[str, Any]] = None
    status: str = "active"  # active, finished, error
    error: Optional[str] = None
    
    def __post_init__(self):
        if self.tags is None:
            self.tags = {}
        if self.logs is None:
            self.logs = []


@dataclass 
class PerformanceSnapshot:
    """性能快照"""
    timestamp: float
    cpu_percent: float
    memory_mb: float
    active_spans: int
    completed_spans: int
    error_spans: int
    avg_duration_ms: float
    p95_duration_ms: float
    p99_duration_ms: float
    hotspots: List[Dict[str, Any]] = None
    
    def __post_init__(self):
        if self.hotspots is None:
            self.hotspots = []


@dataclass
class PerformanceBottleneck:
    """性能瓶颈"""
    component: str
    operation: str
    avg_duration_ms: float
    call_count: int
    total_time_ms: float
    impact_score: float  # 影响分数
    suggestions: List[str] = None
    
    def __post_init__(self):
        if self.suggestions is None:
            self.suggestions = []


class TraceContext:
    """追踪上下文"""
    
    def __init__(self):
        self._storage = threading.local()
    
    def get_current_span(self) -> Optional[TraceSpan]:
        """获取当前跨度"""
        return getattr(self._storage, 'current_span', None)
    
    def set_current_span(self, span: Optional[TraceSpan]):
        """设置当前跨度"""
        self._storage.current_span = span
    
    def get_trace_id(self) -> Optional[str]:
        """获取当前追踪ID"""
        span = self.get_current_span()
        return span.trace_id if span else None


# 全局追踪上下文
_trace_context = TraceContext()


class PerformanceProfiler:
    """性能分析器"""
    
    def __init__(self, enabled: bool = True):
        self.enabled = enabled
        self.profiles: Dict[str, cProfile.Profile] = {}
        self.profile_results: Dict[str, Dict[str, Any]] = {}
        self._lock = threading.RLock()
    
    def start_profiling(self, profile_name: str = "default"):
        """开始性能分析"""
        if not self.enabled:
            return
        
        with self._lock:
            if profile_name in self.profiles:
                return  # 已在分析中
            
            profiler = cProfile.Profile()
            profiler.enable()
            self.profiles[profile_name] = profiler
    
    def stop_profiling(self, profile_name: str = "default") -> Optional[Dict[str, Any]]:
        """停止性能分析并返回结果"""
        if not self.enabled:
            return None
        
        with self._lock:
            profiler = self.profiles.pop(profile_name, None)
            if not profiler:
                return None
            
            profiler.disable()
            
            # 分析结果
            stats_stream = io.StringIO()
            stats = pstats.Stats(profiler, stream=stats_stream)
            stats.sort_stats('cumulative')
            stats.print_stats(20)  # 前20个函数
            
            # 获取热点函数
            hotspots = []
            for func_info, (cc, nc, tt, ct, callers) in stats.stats.items():
                filename, lineno, funcname = func_info
                hotspots.append({
                    "function": f"{filename}:{lineno}({funcname})",
                    "call_count": cc,
                    "total_time": tt,
                    "cumulative_time": ct,
                    "per_call_time": ct / cc if cc > 0 else 0
                })
            
            # 按累积时间排序
            hotspots.sort(key=lambda x: x["cumulative_time"], reverse=True)
            
            result = {
                "profile_name": profile_name,
                "timestamp": time.time(),
                "stats_text": stats_stream.getvalue(),
                "hotspots": hotspots[:10],  # 前10个热点
                "total_calls": sum(cc for (cc, nc, tt, ct, callers) in stats.stats.values()),
                "total_time": sum(tt for (cc, nc, tt, ct, callers) in stats.stats.values())
            }
            
            self.profile_results[profile_name] = result
            return result
    
    @contextmanager
    def profile(self, profile_name: str = None):
        """性能分析上下文管理器"""
        if profile_name is None:
            profile_name = f"profile_{int(time.time())}"
        
        self.start_profiling(profile_name)
        try:
            yield profile_name
        finally:
            self.stop_profiling(profile_name)
    
    def get_profile_result(self, profile_name: str) -> Optional[Dict[str, Any]]:
        """获取分析结果"""
        return self.profile_results.get(profile_name)
    
    def clear_results(self):
        """清除分析结果"""
        with self._lock:
            self.profile_results.clear()


class PerformanceTracer:
    """
    📈 企业级性能追踪器
    
    功能特性：
    - 分布式链路追踪
    - 性能热点分析
    - 自动瓶颈诊断
    - 优化建议生成
    """
    
    def __init__(self, db_path: str = "data/traces.db", 
                 metrics_collector: MetricsCollector = None):
        
        self.db_path = db_path
        self.metrics = metrics_collector or MetricsCollector()
        
        # 追踪数据存储
        self.active_spans: Dict[str, TraceSpan] = {}
        self.completed_spans: Dict[str, TraceSpan] = {}
        
        # 性能统计
        self.operation_stats: Dict[str, List[float]] = defaultdict(list)
        self.error_counts: Dict[str, int] = defaultdict(int)
        
        # 性能分析器
        self.profiler = PerformanceProfiler()
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 数据持久化
        self._persistence_thread = None
        self._stop_persistence = False
        
        # 性能快照
        self.snapshots: deque = deque(maxlen=1000)
        
        # 瓶颈检测阈值
        self.bottleneck_threshold_ms = 1000  # 1秒
        self.bottleneck_call_threshold = 10   # 最少10次调用
        
        # 初始化
        self._init_database()
        self._start_persistence()
    
    def _init_database(self):
        """初始化数据库"""
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            # 创建追踪表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS traces (
                    trace_id TEXT NOT NULL,
                    span_id TEXT PRIMARY KEY,
                    parent_span_id TEXT,
                    operation_name TEXT NOT NULL,
                    start_time REAL NOT NULL,
                    end_time REAL,
                    duration_ms REAL,
                    tags TEXT,
                    logs TEXT,
                    status TEXT,
                    error TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            # 创建性能快照表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS performance_snapshots (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp REAL NOT NULL,
                    cpu_percent REAL,
                    memory_mb REAL,
                    active_spans INTEGER,
                    completed_spans INTEGER,
                    error_spans INTEGER,
                    avg_duration_ms REAL,
                    p95_duration_ms REAL,
                    p99_duration_ms REAL,
                    hotspots TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            # 创建瓶颈检测表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS bottlenecks (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    component TEXT NOT NULL,
                    operation TEXT NOT NULL,
                    avg_duration_ms REAL NOT NULL,
                    call_count INTEGER NOT NULL,
                    total_time_ms REAL NOT NULL,
                    impact_score REAL NOT NULL,
                    suggestions TEXT,
                    detected_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            # 创建索引
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_traces_trace_id ON traces(trace_id)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_traces_operation ON traces(operation_name)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_snapshots_timestamp ON performance_snapshots(timestamp)')
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 初始化追踪数据库失败: {e}")
    
    def start_trace(self, operation_name: str, 
                   trace_id: Optional[str] = None,
                   parent_span_id: Optional[str] = None,
                   tags: Dict[str, Any] = None) -> TraceSpan:
        """开始追踪"""
        
        if trace_id is None:
            trace_id = str(uuid.uuid4())
        
        span_id = str(uuid.uuid4())
        
        # 如果没有指定父跨度，尝试从上下文获取
        if parent_span_id is None:
            current_span = _trace_context.get_current_span()
            if current_span:
                parent_span_id = current_span.span_id
                trace_id = current_span.trace_id  # 继承追踪ID
        
        span = TraceSpan(
            trace_id=trace_id,
            span_id=span_id,
            parent_span_id=parent_span_id,
            operation_name=operation_name,
            start_time=time.time(),
            tags=tags or {}
        )
        
        with self._lock:
            self.active_spans[span_id] = span
        
        # 设置为当前跨度
        _trace_context.set_current_span(span)
        
        # 记录指标
        self.metrics.record_counter("trace_spans_started", 
                                   labels={"operation": operation_name})
        
        return span
    
    def finish_trace(self, span: TraceSpan, error: Optional[str] = None):
        """结束追踪"""
        
        end_time = time.time()
        span.end_time = end_time
        span.duration_ms = (end_time - span.start_time) * 1000
        
        if error:
            span.status = "error"
            span.error = error
            self.error_counts[span.operation_name] += 1
            
            # 记录错误指标
            self.metrics.record_counter("trace_spans_error",
                                       labels={"operation": span.operation_name})
        else:
            span.status = "finished"
        
        with self._lock:
            # 移除活跃跨度
            self.active_spans.pop(span.span_id, None)
            
            # 添加到完成跨度
            self.completed_spans[span.span_id] = span
            
            # 更新操作统计
            self.operation_stats[span.operation_name].append(span.duration_ms)
        
        # 清除当前跨度上下文
        if _trace_context.get_current_span() == span:
            _trace_context.set_current_span(None)
        
        # 记录指标
        self.metrics.record_histogram("trace_duration_ms", span.duration_ms,
                                     labels={"operation": span.operation_name})
        
        self.metrics.record_counter("trace_spans_finished",
                                   labels={"operation": span.operation_name})
    
    def add_span_tag(self, span: TraceSpan, key: str, value: Any):
        """添加跨度标签"""
        span.tags[key] = value
    
    def add_span_log(self, span: TraceSpan, message: str, 
                    level: str = "info", **kwargs):
        """添加跨度日志"""
        log_entry = {
            "timestamp": time.time(),
            "level": level,
            "message": message,
            **kwargs
        }
        span.logs.append(log_entry)
    
    @contextmanager
    def trace(self, operation_name: str, tags: Dict[str, Any] = None):
        """追踪上下文管理器"""
        span = self.start_trace(operation_name, tags=tags)
        try:
            yield span
        except Exception as e:
            self.finish_trace(span, error=str(e))
            raise
        else:
            self.finish_trace(span)
    
    def trace_function(self, operation_name: Optional[str] = None, 
                      tags: Dict[str, Any] = None):
        """函数追踪装饰器"""
        def decorator(func):
            op_name = operation_name or f"{func.__module__}.{func.__name__}"
            
            @functools.wraps(func)
            def wrapper(*args, **kwargs):
                with self.trace(op_name, tags=tags) as span:
                    # 添加函数信息
                    self.add_span_tag(span, "function", func.__name__)
                    self.add_span_tag(span, "module", func.__module__)
                    
                    return func(*args, **kwargs)
            
            return wrapper
        return decorator
    
    def get_trace_summary(self, trace_id: str) -> Dict[str, Any]:
        """获取追踪摘要"""
        trace_spans = []
        
        with self._lock:
            # 从活跃跨度查找
            for span in self.active_spans.values():
                if span.trace_id == trace_id:
                    trace_spans.append(span)
            
            # 从完成跨度查找
            for span in self.completed_spans.values():
                if span.trace_id == trace_id:
                    trace_spans.append(span)
        
        if not trace_spans:
            return {"trace_id": trace_id, "spans": [], "total_spans": 0}
        
        # 计算统计信息
        total_duration = sum(s.duration_ms or 0 for s in trace_spans if s.duration_ms)
        error_count = sum(1 for s in trace_spans if s.status == "error")
        
        return {
            "trace_id": trace_id,
            "total_spans": len(trace_spans),
            "completed_spans": len([s for s in trace_spans if s.status == "finished"]),
            "error_spans": error_count,
            "total_duration_ms": total_duration,
            "spans": [asdict(s) for s in trace_spans]
        }
    
    def analyze_performance(self) -> Dict[str, Any]:
        """分析性能数据"""
        
        with self._lock:
            operations_analysis = {}
            
            for operation, durations in self.operation_stats.items():
                if not durations:
                    continue
                
                sorted_durations = sorted(durations)
                count = len(durations)
                
                operations_analysis[operation] = {
                    "call_count": count,
                    "avg_duration_ms": sum(durations) / count,
                    "min_duration_ms": min(durations),
                    "max_duration_ms": max(durations),
                    "p50_duration_ms": sorted_durations[int(count * 0.5)],
                    "p95_duration_ms": sorted_durations[int(count * 0.95)],
                    "p99_duration_ms": sorted_durations[int(count * 0.99)],
                    "error_count": self.error_counts.get(operation, 0),
                    "error_rate": self.error_counts.get(operation, 0) / count
                }
        
        # 检测瓶颈
        bottlenecks = self.detect_bottlenecks()
        
        return {
            "timestamp": time.time(),
            "operations": operations_analysis,
            "bottlenecks": [asdict(b) for b in bottlenecks],
            "active_spans": len(self.active_spans),
            "completed_spans": len(self.completed_spans)
        }
    
    def detect_bottlenecks(self) -> List[PerformanceBottleneck]:
        """检测性能瓶颈"""
        bottlenecks = []
        
        with self._lock:
            for operation, durations in self.operation_stats.items():
                if len(durations) < self.bottleneck_call_threshold:
                    continue
                
                avg_duration = sum(durations) / len(durations)
                if avg_duration < self.bottleneck_threshold_ms:
                    continue
                
                total_time = sum(durations)
                call_count = len(durations)
                
                # 计算影响分数 (平均耗时 * 调用次数)
                impact_score = avg_duration * call_count
                
                # 生成优化建议
                suggestions = []
                if avg_duration > 5000:  # 5秒以上
                    suggestions.append("考虑异步处理或缓存优化")
                if call_count > 1000:
                    suggestions.append("高频调用，考虑批处理或合并操作")
                if avg_duration > 2000:  # 2秒以上
                    suggestions.append("检查是否有数据库查询或IO瓶颈")
                
                bottleneck = PerformanceBottleneck(
                    component="operation",
                    operation=operation,
                    avg_duration_ms=avg_duration,
                    call_count=call_count,
                    total_time_ms=total_time,
                    impact_score=impact_score,
                    suggestions=suggestions
                )
                
                bottlenecks.append(bottleneck)
        
        # 按影响分数排序
        bottlenecks.sort(key=lambda x: x.impact_score, reverse=True)
        return bottlenecks[:10]  # 返回前10个瓶颈
    
    def take_performance_snapshot(self) -> PerformanceSnapshot:
        """拍摄性能快照"""
        
        import psutil
        
        # 收集系统信息
        cpu_percent = psutil.cpu_percent()
        memory_mb = psutil.virtual_memory().used / 1024 / 1024
        
        # 收集追踪信息
        with self._lock:
            active_spans = len(self.active_spans)
            completed_spans = len(self.completed_spans)
            error_spans = sum(self.error_counts.values())
            
            # 计算平均耗时
            all_durations = []
            for durations in self.operation_stats.values():
                all_durations.extend(durations)
            
            if all_durations:
                sorted_durations = sorted(all_durations)
                count = len(sorted_durations)
                avg_duration = sum(all_durations) / count
                p95_duration = sorted_durations[int(count * 0.95)]
                p99_duration = sorted_durations[int(count * 0.99)]
            else:
                avg_duration = p95_duration = p99_duration = 0
        
        # 获取热点信息
        hotspots = []
        profile_result = self.profiler.get_profile_result("default")
        if profile_result and "hotspots" in profile_result:
            hotspots = profile_result["hotspots"][:5]  # 前5个热点
        
        snapshot = PerformanceSnapshot(
            timestamp=time.time(),
            cpu_percent=cpu_percent,
            memory_mb=memory_mb,
            active_spans=active_spans,
            completed_spans=completed_spans,
            error_spans=error_spans,
            avg_duration_ms=avg_duration,
            p95_duration_ms=p95_duration,
            p99_duration_ms=p99_duration,
            hotspots=hotspots
        )
        
        self.snapshots.append(snapshot)
        return snapshot
    
    def start_profiling(self, profile_name: str = "default"):
        """开始性能分析"""
        self.profiler.start_profiling(profile_name)
    
    def stop_profiling(self, profile_name: str = "default") -> Optional[Dict[str, Any]]:
        """停止性能分析"""
        return self.profiler.stop_profiling(profile_name)
    
    def _start_persistence(self):
        """启动数据持久化"""
        if self._persistence_thread is not None:
            return
        
        self._stop_persistence = False
        self._persistence_thread = threading.Thread(target=self._persistence_loop)
        self._persistence_thread.daemon = True
        self._persistence_thread.start()
    
    def _persistence_loop(self):
        """持久化循环"""
        while not self._stop_persistence:
            try:
                self._save_completed_spans()
                self._save_performance_snapshot()
                time.sleep(60)  # 每分钟保存一次
                
            except Exception as e:
                print(f"⚠️ 追踪数据持久化异常: {e}")
                time.sleep(10)
    
    def _save_completed_spans(self):
        """保存完成的跨度"""
        spans_to_save = []
        
        with self._lock:
            # 获取需要保存的跨度 (超过1000个时保存)
            if len(self.completed_spans) > 1000:
                spans_items = list(self.completed_spans.items())
                spans_to_save = [span for _, span in spans_items[:500]]  # 保存前500个
                
                # 从内存中移除已保存的
                for span in spans_to_save:
                    self.completed_spans.pop(span.span_id, None)
        
        if spans_to_save:
            try:
                conn = sqlite3.connect(self.db_path)
                cursor = conn.cursor()
                
                for span in spans_to_save:
                    cursor.execute('''
                        INSERT INTO traces 
                        (trace_id, span_id, parent_span_id, operation_name, 
                         start_time, end_time, duration_ms, tags, logs, status, error)
                        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    ''', (
                        span.trace_id, span.span_id, span.parent_span_id,
                        span.operation_name, span.start_time, span.end_time,
                        span.duration_ms, json.dumps(span.tags),
                        json.dumps(span.logs), span.status, span.error
                    ))
                
                conn.commit()
                conn.close()
                
            except Exception as e:
                print(f"⚠️ 保存追踪数据失败: {e}")
    
    def _save_performance_snapshot(self):
        """保存性能快照"""
        snapshot = self.take_performance_snapshot()
        
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            cursor.execute('''
                INSERT INTO performance_snapshots
                (timestamp, cpu_percent, memory_mb, active_spans, completed_spans,
                 error_spans, avg_duration_ms, p95_duration_ms, p99_duration_ms, hotspots)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ''', (
                snapshot.timestamp, snapshot.cpu_percent, snapshot.memory_mb,
                snapshot.active_spans, snapshot.completed_spans, snapshot.error_spans,
                snapshot.avg_duration_ms, snapshot.p95_duration_ms, snapshot.p99_duration_ms,
                json.dumps(snapshot.hotspots)
            ))
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 保存性能快照失败: {e}")
    
    def get_performance_report(self) -> Dict[str, Any]:
        """生成性能报告"""
        analysis = self.analyze_performance()
        snapshot = self.take_performance_snapshot()
        
        # 获取最近的快照趋势
        recent_snapshots = list(self.snapshots)[-10:]  # 最近10个快照
        
        return {
            "timestamp": time.time(),
            "current_snapshot": asdict(snapshot),
            "trend_snapshots": [asdict(s) for s in recent_snapshots],
            "performance_analysis": analysis,
            "profiling_available": len(self.profiler.profile_results) > 0,
            "recommendations": self._generate_recommendations(analysis)
        }
    
    def _generate_recommendations(self, analysis: Dict[str, Any]) -> List[str]:
        """生成性能优化建议"""
        recommendations = []
        
        # 基于瓶颈分析的建议
        bottlenecks = analysis.get("bottlenecks", [])
        if bottlenecks:
            recommendations.append(f"发现 {len(bottlenecks)} 个性能瓶颈，建议优先处理影响分数最高的操作")
        
        # 基于操作统计的建议
        operations = analysis.get("operations", {})
        slow_ops = [op for op, stats in operations.items() 
                   if stats["avg_duration_ms"] > 1000]
        if slow_ops:
            recommendations.append(f"有 {len(slow_ops)} 个操作平均耗时超过1秒，建议进行优化")
        
        high_error_ops = [op for op, stats in operations.items() 
                         if stats["error_rate"] > 0.1]
        if high_error_ops:
            recommendations.append(f"有 {len(high_error_ops)} 个操作错误率超过10%，建议检查错误处理")
        
        return recommendations
    
    def cleanup_old_data(self, days: int = 7):
        """清理旧数据"""
        cutoff_time = time.time() - (days * 24 * 3600)
        
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            # 清理旧追踪数据
            cursor.execute('DELETE FROM traces WHERE start_time < ?', (cutoff_time,))
            
            # 清理旧快照数据
            cursor.execute('DELETE FROM performance_snapshots WHERE timestamp < ?', (cutoff_time,))
            
            # 清理旧瓶颈数据
            cursor.execute('DELETE FROM bottlenecks WHERE detected_at < datetime(?, "unixepoch")', (cutoff_time,))
            
            conn.commit()
            conn.close()
            
            print(f"✅ 清理了 {days} 天前的追踪数据")
            
        except Exception as e:
            print(f"⚠️ 清理追踪数据失败: {e}")
    
    def shutdown(self):
        """关闭性能追踪器"""
        self._stop_persistence = True
        if self._persistence_thread:
            self._persistence_thread.join(timeout=10.0)
        
        # 保存剩余数据
        self._save_completed_spans()
        
        print("📈 性能追踪器已关闭")
