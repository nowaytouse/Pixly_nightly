"""
📊 PIXLY v3.0 日志聚合器

基于Go废弃模块logging.go的聚合架构：
- 多源日志收集
- 实时日志分析
- 错误模式检测
- 性能指标提取
- 本地化存储

迁移自：Go logging.go 的聚合逻辑
"""

import time
import json
import threading
import sqlite3
from collections import defaultdict, deque, Counter
from dataclasses import dataclass, asdict
from typing import Dict, List, Optional, Any, Callable
from pathlib import Path
import re

from .structured_logger import LogEntry, LogLevel


@dataclass
class LogStats:
    """日志统计"""
    total_count: int
    error_count: int
    warning_count: int
    info_count: int
    debug_count: int
    unique_components: int
    error_rate: float
    avg_logs_per_minute: float
    top_components: List[tuple]
    recent_errors: List[LogEntry]


@dataclass
class LogPattern:
    """日志模式"""
    pattern_id: str
    regex: str
    description: str
    severity: str
    count: int = 0
    last_occurrence: Optional[float] = None
    samples: List[LogEntry] = None
    
    def __post_init__(self):
        if self.samples is None:
            self.samples = []


class LogAggregator:
    """
    📊 企业级日志聚合器
    
    功能特性：
    - 多源日志收集
    - 实时统计分析
    - 模式识别告警
    - 性能趋势分析
    """
    
    def __init__(self, db_path: str = "data/log_aggregation.db",
                 window_size: int = 1000, retention_hours: int = 72):
        
        self.db_path = db_path
        self.window_size = window_size
        self.retention_hours = retention_hours
        
        # 内存窗口 (最新N条日志)
        self.log_window: deque = deque(maxlen=window_size)
        
        # 实时统计
        self.component_counts: Counter = Counter()
        self.level_counts: Counter = Counter()
        self.hourly_counts: Dict[str, int] = defaultdict(int)
        
        # 错误检测
        self.error_patterns: List[LogPattern] = []
        self.pattern_callbacks: List[Callable] = []
        
        # 性能指标
        self.metrics: Dict[str, deque] = {
            "logs_per_minute": deque(maxlen=60),
            "error_rate": deque(maxlen=60),
            "component_diversity": deque(maxlen=60)
        }
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 聚合线程
        self._aggregation_thread = None
        self._stop_aggregation = False
        
        # 初始化
        self._init_database()
        self._init_default_patterns()
        self._start_aggregation()
    
    def _init_database(self):
        """初始化聚合数据库"""
        try:
            Path(self.db_path).parent.mkdir(parents=True, exist_ok=True)
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            # 日志聚合表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS log_aggregations (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    period_start TEXT NOT NULL,
                    period_end TEXT NOT NULL,
                    total_logs INTEGER NOT NULL,
                    error_logs INTEGER NOT NULL,
                    warning_logs INTEGER NOT NULL,
                    info_logs INTEGER NOT NULL,
                    debug_logs INTEGER NOT NULL,
                    unique_components INTEGER NOT NULL,
                    error_rate REAL NOT NULL,
                    top_components TEXT,
                    patterns_detected TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            # 错误模式表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS error_patterns (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    pattern_id TEXT UNIQUE NOT NULL,
                    regex TEXT NOT NULL,
                    description TEXT NOT NULL,
                    severity TEXT NOT NULL,
                    total_matches INTEGER DEFAULT 0,
                    last_occurrence TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            # 性能指标表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS performance_metrics (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp TEXT NOT NULL,
                    metric_name TEXT NOT NULL,
                    metric_value REAL NOT NULL,
                    context TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            # 创建索引
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_aggregations_period ON log_aggregations(period_start, period_end)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_patterns_id ON error_patterns(pattern_id)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON performance_metrics(timestamp)')
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 初始化聚合数据库失败: {e}")
    
    def _init_default_patterns(self):
        """初始化默认错误模式"""
        default_patterns = [
            LogPattern(
                pattern_id="auth_failure",
                regex=r"(authentication|login|auth).*fail",
                description="认证失败",
                severity="high"
            ),
            LogPattern(
                pattern_id="file_not_found",
                regex=r"(file|path).*not found|no such file",
                description="文件不存在",
                severity="medium"
            ),
            LogPattern(
                pattern_id="permission_denied",
                regex=r"permission denied|access denied|forbidden",
                description="权限拒绝",
                severity="high"
            ),
            LogPattern(
                pattern_id="connection_timeout",
                regex=r"(connection|request).*timeout|timed out",
                description="连接超时",
                severity="medium"
            ),
            LogPattern(
                pattern_id="memory_error",
                regex=r"out of memory|memory.*error|allocation.*fail",
                description="内存错误",
                severity="critical"
            ),
            LogPattern(
                pattern_id="validation_error",
                regex=r"validation.*fail|invalid.*parameter|bad.*request",
                description="验证错误",
                severity="medium"
            )
        ]
        
        self.error_patterns.extend(default_patterns)
    
    def add_log_entry(self, entry: LogEntry):
        """添加日志条目"""
        with self._lock:
            # 添加到窗口
            self.log_window.append(entry)
            
            # 更新统计
            self.component_counts[entry.component] += 1
            self.level_counts[entry.level] += 1
            
            # 小时统计
            hour_key = entry.timestamp[:13]  # YYYY-MM-DDTHH
            self.hourly_counts[hour_key] += 1
            
            # 检查错误模式
            if entry.level in ["ERROR", "WARNING"]:
                self._check_patterns(entry)
    
    def _check_patterns(self, entry: LogEntry):
        """检查错误模式"""
        message_lower = entry.message.lower()
        
        for pattern in self.error_patterns:
            try:
                if re.search(pattern.regex, message_lower, re.IGNORECASE):
                    pattern.count += 1
                    pattern.last_occurrence = time.time()
                    
                    # 保存样例 (最多保存5个)
                    pattern.samples.append(entry)
                    if len(pattern.samples) > 5:
                        pattern.samples = pattern.samples[-5:]
                    
                    # 触发回调
                    self._trigger_pattern_callbacks(pattern, entry)
                    
            except re.error as e:
                print(f"⚠️ 正则表达式错误 {pattern.pattern_id}: {e}")
    
    def _trigger_pattern_callbacks(self, pattern: LogPattern, entry: LogEntry):
        """触发模式回调"""
        for callback in self.pattern_callbacks:
            try:
                callback(pattern, entry)
            except Exception as e:
                print(f"⚠️ 模式回调执行失败: {e}")
    
    def add_pattern_callback(self, callback: Callable):
        """添加模式回调"""
        self.pattern_callbacks.append(callback)
    
    def add_custom_pattern(self, pattern_id: str, regex: str, 
                          description: str, severity: str = "medium"):
        """添加自定义模式"""
        pattern = LogPattern(
            pattern_id=pattern_id,
            regex=regex,
            description=description,
            severity=severity
        )
        
        with self._lock:
            # 移除同名模式
            self.error_patterns = [p for p in self.error_patterns 
                                 if p.pattern_id != pattern_id]
            
            # 添加新模式
            self.error_patterns.append(pattern)
    
    def get_current_stats(self) -> LogStats:
        """获取当前统计"""
        with self._lock:
            if not self.log_window:
                return LogStats(
                    total_count=0, error_count=0, warning_count=0,
                    info_count=0, debug_count=0, unique_components=0,
                    error_rate=0.0, avg_logs_per_minute=0.0,
                    top_components=[], recent_errors=[]
                )
            
            total = len(self.log_window)
            error_count = self.level_counts.get("ERROR", 0)
            warning_count = self.level_counts.get("WARNING", 0)
            info_count = self.level_counts.get("INFO", 0)
            debug_count = self.level_counts.get("DEBUG", 0)
            
            error_rate = error_count / total if total > 0 else 0.0
            
            # 计算平均每分钟日志数
            if len(self.log_window) >= 2:
                time_span = self._parse_timestamp(self.log_window[-1].timestamp) - \
                           self._parse_timestamp(self.log_window[0].timestamp)
                avg_per_minute = len(self.log_window) / max(time_span / 60, 1)
            else:
                avg_per_minute = 0.0
            
            # 最近错误
            recent_errors = [entry for entry in list(self.log_window)[-50:] 
                           if entry.level == "ERROR"][-10:]
            
            return LogStats(
                total_count=total,
                error_count=error_count,
                warning_count=warning_count,
                info_count=info_count,
                debug_count=debug_count,
                unique_components=len(self.component_counts),
                error_rate=error_rate,
                avg_logs_per_minute=avg_per_minute,
                top_components=self.component_counts.most_common(10),
                recent_errors=recent_errors
            )
    
    def _parse_timestamp(self, timestamp_str: str) -> float:
        """解析时间戳"""
        try:
            # 处理ISO 8601格式: 2023-11-12T15:30:45.123Z
            timestamp_str = timestamp_str.replace('Z', '+00:00')
            from datetime import datetime
            dt = datetime.fromisoformat(timestamp_str.replace('Z', ''))
            return dt.timestamp()
        except:
            return time.time()
    
    def get_pattern_analysis(self) -> Dict[str, Any]:
        """获取模式分析"""
        with self._lock:
            patterns_data = []
            
            for pattern in self.error_patterns:
                if pattern.count > 0:
                    patterns_data.append({
                        "pattern_id": pattern.pattern_id,
                        "description": pattern.description,
                        "severity": pattern.severity,
                        "count": pattern.count,
                        "last_occurrence": pattern.last_occurrence,
                        "samples_count": len(pattern.samples)
                    })
            
            # 按严重度和计数排序
            severity_order = {"critical": 0, "high": 1, "medium": 2, "low": 3}
            patterns_data.sort(key=lambda x: (severity_order.get(x["severity"], 3), -x["count"]))
            
            return {
                "total_patterns": len(self.error_patterns),
                "active_patterns": len(patterns_data),
                "patterns": patterns_data,
                "pattern_summary": {
                    "critical": len([p for p in patterns_data if p["severity"] == "critical"]),
                    "high": len([p for p in patterns_data if p["severity"] == "high"]),
                    "medium": len([p for p in patterns_data if p["severity"] == "medium"]),
                    "low": len([p for p in patterns_data if p["severity"] == "low"])
                }
            }
    
    def get_performance_trends(self) -> Dict[str, Any]:
        """获取性能趋势"""
        with self._lock:
            return {
                "logs_per_minute": list(self.metrics["logs_per_minute"]),
                "error_rate_trend": list(self.metrics["error_rate"]),
                "component_diversity": list(self.metrics["component_diversity"]),
                "current_stats": asdict(self.get_current_stats())
            }
    
    def _start_aggregation(self):
        """启动聚合线程"""
        if self._aggregation_thread is not None:
            return
        
        self._stop_aggregation = False
        self._aggregation_thread = threading.Thread(target=self._aggregation_loop)
        self._aggregation_thread.daemon = True
        self._aggregation_thread.start()
    
    def _aggregation_loop(self):
        """聚合循环"""
        last_save_time = time.time()
        
        while not self._stop_aggregation:
            try:
                current_time = time.time()
                
                # 更新实时指标
                self._update_metrics()
                
                # 每5分钟保存一次聚合数据
                if current_time - last_save_time >= 300:
                    self._save_aggregation_data()
                    last_save_time = current_time
                
                # 清理过期数据
                self._cleanup_old_data()
                
                time.sleep(60)  # 每分钟执行一次
                
            except Exception as e:
                print(f"⚠️ 日志聚合循环异常: {e}")
                time.sleep(10)
    
    def _update_metrics(self):
        """更新实时指标"""
        with self._lock:
            stats = self.get_current_stats()
            
            # 更新指标队列
            self.metrics["logs_per_minute"].append(stats.avg_logs_per_minute)
            self.metrics["error_rate"].append(stats.error_rate)
            self.metrics["component_diversity"].append(stats.unique_components)
    
    def _save_aggregation_data(self):
        """保存聚合数据"""
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            # 获取当前统计
            stats = self.get_current_stats()
            
            # 时间范围
            current_time = time.time()
            period_start = time.strftime("%Y-%m-%dT%H:%M:%SZ", 
                                       time.gmtime(current_time - 300))
            period_end = time.strftime("%Y-%m-%dT%H:%M:%SZ", 
                                     time.gmtime(current_time))
            
            # 保存聚合数据
            cursor.execute('''
                INSERT INTO log_aggregations 
                (period_start, period_end, total_logs, error_logs, warning_logs,
                 info_logs, debug_logs, unique_components, error_rate, 
                 top_components, patterns_detected)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ''', (
                period_start, period_end, stats.total_count, stats.error_count,
                stats.warning_count, stats.info_count, stats.debug_count,
                stats.unique_components, stats.error_rate,
                json.dumps(stats.top_components), 
                json.dumps(self.get_pattern_analysis())
            ))
            
            # 更新模式统计
            for pattern in self.error_patterns:
                if pattern.count > 0:
                    cursor.execute('''
                        INSERT OR REPLACE INTO error_patterns 
                        (pattern_id, regex, description, severity, total_matches, last_occurrence)
                        VALUES (?, ?, ?, ?, ?, ?)
                    ''', (
                        pattern.pattern_id, pattern.regex, pattern.description,
                        pattern.severity, pattern.count,
                        time.strftime("%Y-%m-%dT%H:%M:%SZ", 
                                     time.gmtime(pattern.last_occurrence)) if pattern.last_occurrence else None
                    ))
            
            # 保存性能指标
            timestamp = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(current_time))
            
            for metric_name, values in self.metrics.items():
                if values:
                    cursor.execute('''
                        INSERT INTO performance_metrics 
                        (timestamp, metric_name, metric_value)
                        VALUES (?, ?, ?)
                    ''', (timestamp, metric_name, values[-1]))
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 保存聚合数据失败: {e}")
    
    def _cleanup_old_data(self):
        """清理过期数据"""
        try:
            cutoff_time = time.time() - (self.retention_hours * 3600)
            cutoff_str = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(cutoff_time))
            
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            # 清理聚合数据
            cursor.execute('DELETE FROM log_aggregations WHERE period_start < ?', (cutoff_str,))
            
            # 清理性能指标
            cursor.execute('DELETE FROM performance_metrics WHERE timestamp < ?', (cutoff_str,))
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 清理聚合数据失败: {e}")
    
    def export_analysis_report(self) -> Dict[str, Any]:
        """导出分析报告"""
        with self._lock:
            stats = self.get_current_stats()
            patterns = self.get_pattern_analysis()
            trends = self.get_performance_trends()
            
            return {
                "report_timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
                "summary": {
                    "total_logs": stats.total_count,
                    "error_rate": stats.error_rate,
                    "unique_components": stats.unique_components,
                    "avg_logs_per_minute": stats.avg_logs_per_minute
                },
                "level_distribution": {
                    "errors": stats.error_count,
                    "warnings": stats.warning_count,
                    "info": stats.info_count,
                    "debug": stats.debug_count
                },
                "top_components": dict(stats.top_components),
                "pattern_analysis": patterns,
                "performance_trends": trends,
                "recommendations": self._generate_recommendations(stats, patterns)
            }
    
    def _generate_recommendations(self, stats: LogStats, 
                                patterns: Dict[str, Any]) -> List[str]:
        """生成优化建议"""
        recommendations = []
        
        # 基于错误率的建议
        if stats.error_rate > 0.1:
            recommendations.append(f"错误率过高({stats.error_rate:.1%})，建议重点排查错误日志")
        
        # 基于模式的建议
        critical_patterns = patterns["pattern_summary"].get("critical", 0)
        if critical_patterns > 0:
            recommendations.append(f"检测到{critical_patterns}个严重错误模式，需要立即处理")
        
        high_patterns = patterns["pattern_summary"].get("high", 0)
        if high_patterns > 0:
            recommendations.append(f"检测到{high_patterns}个高优先级错误模式，建议优先处理")
        
        # 基于组件的建议
        if len(stats.top_components) > 0:
            top_component = stats.top_components[0]
            if top_component[1] > stats.total_count * 0.5:
                recommendations.append(f"组件'{top_component[0]}'产生了超过50%的日志，可能存在问题")
        
        return recommendations
    
    def shutdown(self):
        """关闭聚合器"""
        self._stop_aggregation = True
        if self._aggregation_thread:
            self._aggregation_thread.join(timeout=10.0)
        
        # 保存最后的聚合数据
        self._save_aggregation_data()
        
        print("📊 日志聚合器已关闭")
