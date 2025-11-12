"""
日志分析器 - 智能日志分析和模式检测

提供高级日志分析功能:
- 错误模式检测
- 性能异常分析
- 日志趋势分析
- 智能告警

基于废弃Go日志系统的增强版本
"""

import re
import time
import statistics
from typing import Dict, List, Optional, Any, Tuple, Pattern
from dataclasses import dataclass, field
from datetime import datetime, timedelta
from collections import defaultdict, deque
from enum import Enum


class AlertLevel(Enum):
    """告警级别"""
    LOW = "low"
    MEDIUM = "medium" 
    HIGH = "high"
    CRITICAL = "critical"


@dataclass
class LogPattern:
    """日志模式定义"""
    name: str
    pattern: str
    description: str
    alert_level: AlertLevel = AlertLevel.LOW
    threshold_count: int = 1
    time_window_minutes: int = 5
    compiled_pattern: Optional[Pattern] = field(default=None, init=False)
    
    def __post_init__(self):
        """编译正则表达式"""
        try:
            self.compiled_pattern = re.compile(self.pattern, re.IGNORECASE)
        except re.error as e:
            print(f"模式编译失败 {self.name}: {e}")


@dataclass 
class LogMetrics:
    """日志指标"""
    total_entries: int = 0
    entries_by_level: Dict[str, int] = field(default_factory=lambda: defaultdict(int))
    entries_by_component: Dict[str, int] = field(default_factory=lambda: defaultdict(int))
    error_rate: float = 0.0
    avg_response_time_ms: float = 0.0
    
    # 时间范围
    start_time: Optional[datetime] = None
    end_time: Optional[datetime] = None
    
    # 性能指标
    performance_percentiles: Dict[str, float] = field(default_factory=dict)
    
    # 异常检测
    anomalies_detected: int = 0
    alerts_triggered: int = 0


@dataclass
class LogAlert:
    """日志告警"""
    pattern_name: str
    alert_level: AlertLevel
    message: str
    count: int
    time_window: timedelta
    triggered_at: datetime
    sample_logs: List[Dict[str, Any]] = field(default_factory=list)


class LogAnalyzer:
    """
    日志分析器 - 智能日志分析和监控
    
    提供实时日志分析和异常检测功能
    """
    
    def __init__(self, 
                 enable_realtime: bool = True,
                 max_history_entries: int = 10000,
                 debug: bool = False):
        """
        初始化日志分析器
        
        Args:
            enable_realtime: 启用实时分析
            max_history_entries: 最大历史记录数
            debug: 调试模式
        """
        self.enable_realtime = enable_realtime
        self.max_history_entries = max_history_entries
        self.debug = debug
        
        # 日志历史
        self._log_history: deque = deque(maxlen=max_history_entries)
        
        # 分析模式
        self._patterns: Dict[str, LogPattern] = {}
        
        # 告警历史
        self._alerts: List[LogAlert] = []
        
        # 性能数据
        self._performance_data: deque = deque(maxlen=1000)
        
        # 注册默认模式
        self._register_default_patterns()
        
        if debug:
            print("日志分析器初始化完成")
    
    def _register_default_patterns(self):
        """注册默认分析模式"""
        default_patterns = [
            LogPattern(
                name="error_spike",
                pattern=r"ERROR|CRITICAL|Exception|Error",
                description="错误激增检测",
                alert_level=AlertLevel.HIGH,
                threshold_count=5,
                time_window_minutes=1
            ),
            LogPattern(
                name="slow_response",
                pattern=r"duration_ms.*([5-9]\d{3,}|\d{5,})",  # >5000ms
                description="响应时间过慢",
                alert_level=AlertLevel.MEDIUM,
                threshold_count=3,
                time_window_minutes=2
            ),
            LogPattern(
                name="memory_warning",
                pattern=r"memory.*high|memory.*exceed|out.*of.*memory",
                description="内存使用警告",
                alert_level=AlertLevel.HIGH,
                threshold_count=1,
                time_window_minutes=5
            ),
            LogPattern(
                name="authentication_failure",
                pattern=r"auth.*fail|login.*fail|unauthorized|forbidden",
                description="认证失败检测",
                alert_level=AlertLevel.MEDIUM,
                threshold_count=10,
                time_window_minutes=3
            ),
            LogPattern(
                name="database_error",
                pattern=r"database.*error|connection.*timeout|sql.*error",
                description="数据库错误",
                alert_level=AlertLevel.HIGH,
                threshold_count=3,
                time_window_minutes=2
            )
        ]
        
        for pattern in default_patterns:
            self.register_pattern(pattern)
    
    def register_pattern(self, pattern: LogPattern):
        """注册分析模式"""
        self._patterns[pattern.name] = pattern
        if self.debug:
            print(f"注册分析模式: {pattern.name}")
    
    def analyze_log_entry(self, log_entry: Dict[str, Any]) -> List[LogAlert]:
        """分析单个日志条目"""
        # 添加到历史记录
        log_entry['analyzed_at'] = datetime.now()
        self._log_history.append(log_entry)
        
        # 提取性能数据
        self._extract_performance_data(log_entry)
        
        # 模式匹配检测
        alerts = []
        if self.enable_realtime:
            alerts = self._check_patterns(log_entry)
        
        return alerts
    
    def _extract_performance_data(self, log_entry: Dict[str, Any]):
        """提取性能数据"""
        duration_ms = log_entry.get('duration_ms')
        if duration_ms is not None:
            self._performance_data.append({
                'timestamp': log_entry.get('timestamp', datetime.now().isoformat()),
                'duration_ms': float(duration_ms),
                'component': log_entry.get('component', 'unknown')
            })
    
    def _check_patterns(self, log_entry: Dict[str, Any]) -> List[LogAlert]:
        """检查日志模式匹配"""
        alerts = []
        log_text = self._extract_log_text(log_entry)
        
        for pattern_name, pattern in self._patterns.items():
            if pattern.compiled_pattern and pattern.compiled_pattern.search(log_text):
                # 检查时间窗口内的匹配计数
                alert = self._check_threshold(pattern, log_entry)
                if alert:
                    alerts.append(alert)
                    self._alerts.append(alert)
        
        return alerts
    
    def _extract_log_text(self, log_entry: Dict[str, Any]) -> str:
        """提取日志文本用于模式匹配"""
        text_parts = []
        
        # 主要字段
        for field in ['message', 'error', 'level', 'component']:
            value = log_entry.get(field)
            if value:
                text_parts.append(str(value))
        
        # 元数据
        metadata = log_entry.get('metadata', {})
        if isinstance(metadata, dict):
            for key, value in metadata.items():
                text_parts.append(f"{key}:{value}")
        
        return " ".join(text_parts)
    
    def _check_threshold(self, pattern: LogPattern, current_log: Dict[str, Any]) -> Optional[LogAlert]:
        """检查模式阈值"""
        current_time = datetime.now()
        time_window = timedelta(minutes=pattern.time_window_minutes)
        
        # 统计时间窗口内的匹配数量
        match_count = 0
        sample_logs = []
        
        for log_entry in reversed(self._log_history):
            # 解析时间戳
            log_time = self._parse_timestamp(log_entry.get('timestamp'))
            if not log_time or (current_time - log_time) > time_window:
                continue
            
            # 检查是否匹配
            log_text = self._extract_log_text(log_entry)
            if pattern.compiled_pattern.search(log_text):
                match_count += 1
                if len(sample_logs) < 3:  # 保存样本
                    sample_logs.append(log_entry)
        
        # 检查是否达到阈值
        if match_count >= pattern.threshold_count:
            return LogAlert(
                pattern_name=pattern.name,
                alert_level=pattern.alert_level,
                message=f"{pattern.description}: {match_count}次匹配在{pattern.time_window_minutes}分钟内",
                count=match_count,
                time_window=time_window,
                triggered_at=current_time,
                sample_logs=sample_logs
            )
        
        return None
    
    def _parse_timestamp(self, timestamp_str: Optional[str]) -> Optional[datetime]:
        """解析时间戳"""
        if not timestamp_str:
            return None
        
        try:
            # 处理ISO格式
            if timestamp_str.endswith('Z'):
                timestamp_str = timestamp_str[:-1]
            
            # 尝试多种格式
            formats = [
                "%Y-%m-%dT%H:%M:%S.%f",
                "%Y-%m-%dT%H:%M:%S",
                "%Y-%m-%d %H:%M:%S.%f",
                "%Y-%m-%d %H:%M:%S"
            ]
            
            for fmt in formats:
                try:
                    return datetime.strptime(timestamp_str, fmt)
                except ValueError:
                    continue
            
        except Exception:
            pass
        
        return None
    
    def analyze_batch(self, log_entries: List[Dict[str, Any]]) -> LogMetrics:
        """批量分析日志"""
        metrics = LogMetrics()
        
        if not log_entries:
            return metrics
        
        # 基础统计
        metrics.total_entries = len(log_entries)
        
        performance_times = []
        error_count = 0
        
        # 时间范围
        timestamps = []
        
        for entry in log_entries:
            # 按级别统计
            level = entry.get('level', 'UNKNOWN')
            metrics.entries_by_level[level] += 1
            
            # 按组件统计
            component = entry.get('component', 'unknown')
            metrics.entries_by_component[component] += 1
            
            # 错误计数
            if level in ['ERROR', 'CRITICAL']:
                error_count += 1
            
            # 性能数据
            duration = entry.get('duration_ms')
            if duration is not None:
                try:
                    performance_times.append(float(duration))
                except (ValueError, TypeError):
                    pass
            
            # 时间戳
            timestamp = self._parse_timestamp(entry.get('timestamp'))
            if timestamp:
                timestamps.append(timestamp)
            
            # 实时分析
            if self.enable_realtime:
                alerts = self.analyze_log_entry(entry)
                metrics.alerts_triggered += len(alerts)
        
        # 计算指标
        metrics.error_rate = (error_count / metrics.total_entries * 100) if metrics.total_entries > 0 else 0
        
        if performance_times:
            metrics.avg_response_time_ms = statistics.mean(performance_times)
            
            # 百分位数
            performance_times.sort()
            metrics.performance_percentiles = {
                'p50': self._percentile(performance_times, 0.5),
                'p90': self._percentile(performance_times, 0.9),
                'p95': self._percentile(performance_times, 0.95),
                'p99': self._percentile(performance_times, 0.99)
            }
        
        # 时间范围
        if timestamps:
            metrics.start_time = min(timestamps)
            metrics.end_time = max(timestamps)
        
        return metrics
    
    def _percentile(self, sorted_data: List[float], percentile: float) -> float:
        """计算百分位数"""
        if not sorted_data:
            return 0.0
        
        index = percentile * (len(sorted_data) - 1)
        lower = int(index)
        upper = min(lower + 1, len(sorted_data) - 1)
        weight = index - lower
        
        return sorted_data[lower] * (1 - weight) + sorted_data[upper] * weight
    
    def get_recent_alerts(self, hours: int = 24) -> List[LogAlert]:
        """获取最近的告警"""
        cutoff_time = datetime.now() - timedelta(hours=hours)
        return [alert for alert in self._alerts if alert.triggered_at >= cutoff_time]
    
    def get_performance_summary(self, minutes: int = 60) -> Dict[str, Any]:
        """获取性能摘要"""
        cutoff_time = datetime.now() - timedelta(minutes=minutes)
        
        recent_data = []
        for perf_data in self._performance_data:
            timestamp = self._parse_timestamp(perf_data['timestamp'])
            if timestamp and timestamp >= cutoff_time:
                recent_data.append(perf_data['duration_ms'])
        
        if not recent_data:
            return {"message": "无性能数据"}
        
        return {
            "total_requests": len(recent_data),
            "avg_response_ms": statistics.mean(recent_data),
            "min_response_ms": min(recent_data),
            "max_response_ms": max(recent_data),
            "p95_response_ms": self._percentile(sorted(recent_data), 0.95),
            "time_window_minutes": minutes
        }
    
    def detect_anomalies(self, sensitivity: float = 2.0) -> List[Dict[str, Any]]:
        """异常检测"""
        anomalies = []
        
        # 性能异常检测
        if len(self._performance_data) > 10:
            recent_times = [data['duration_ms'] for data in list(self._performance_data)[-50:]]
            if len(recent_times) > 5:
                mean_time = statistics.mean(recent_times)
                std_time = statistics.stdev(recent_times)
                
                threshold = mean_time + sensitivity * std_time
                
                for data in list(self._performance_data)[-10:]:  # 检查最近10个
                    if data['duration_ms'] > threshold:
                        anomalies.append({
                            "type": "performance_anomaly",
                            "description": f"响应时间异常: {data['duration_ms']:.1f}ms (阈值: {threshold:.1f}ms)",
                            "timestamp": data['timestamp'],
                            "component": data['component'],
                            "value": data['duration_ms'],
                            "threshold": threshold
                        })
        
        return anomalies
    
    def get_stats(self) -> Dict[str, Any]:
        """获取分析器统计"""
        return {
            "total_log_entries": len(self._log_history),
            "registered_patterns": len(self._patterns),
            "total_alerts": len(self._alerts),
            "performance_data_points": len(self._performance_data),
            "recent_alerts_24h": len(self.get_recent_alerts(24)),
            "patterns": [
                {
                    "name": pattern.name,
                    "description": pattern.description,
                    "alert_level": pattern.alert_level.value,
                    "threshold": pattern.threshold_count,
                    "window_minutes": pattern.time_window_minutes
                }
                for pattern in self._patterns.values()
            ]
        }


if __name__ == "__main__":
    # 测试代码
    print("=== 日志分析器测试 ===")
    
    analyzer = LogAnalyzer(debug=True)
    
    # 测试日志分析
    test_logs = [
        {
            "timestamp": datetime.now().isoformat(),
            "level": "INFO",
            "component": "test",
            "message": "正常操作",
            "duration_ms": 150
        },
        {
            "timestamp": datetime.now().isoformat(),
            "level": "ERROR", 
            "component": "test",
            "message": "出现错误",
            "duration_ms": 8000
        },
        {
            "timestamp": datetime.now().isoformat(),
            "level": "ERROR",
            "component": "database", 
            "message": "database connection timeout",
            "duration_ms": 12000
        }
    ]
    
    # 批量分析
    metrics = analyzer.analyze_batch(test_logs)
    print(f"✅ 分析完成: {metrics.total_entries}条日志")
    print(f"📊 错误率: {metrics.error_rate:.1f}%")
    print(f"⏱️ 平均响应: {metrics.avg_response_time_ms:.1f}ms")
    
    # 告警检查
    alerts = analyzer.get_recent_alerts(1)
    print(f"🚨 告警数量: {len(alerts)}")
    
    # 异常检测
    anomalies = analyzer.detect_anomalies()
    print(f"⚠️ 异常检测: {len(anomalies)}个")
    
    # 统计信息
    stats = analyzer.get_stats()
    print(f"📈 分析器统计: {stats['registered_patterns']}个模式")
    
    print("🎯 日志分析器测试完成！")
