"""
📊 PIXLY v3.0 指标收集器

基于Go废弃模块的企业级监控设计：
- 多维度指标收集 (Counter, Histogram, Gauge)
- 时序数据聚合
- 实时指标导出
- 自动阈值告警
- 性能热点分析

迁移自：Go logging.go 和 http_gateway.go 的监控逻辑
"""

import time
import json
import threading
from collections import defaultdict, deque
from dataclasses import dataclass, asdict
from typing import Dict, List, Optional, Any, Callable, Tuple
import statistics
import sqlite3


@dataclass
class MetricPoint:
    """指标数据点"""
    name: str
    value: float
    timestamp: float
    labels: Dict[str, str] = None
    metric_type: str = "gauge"  # counter, histogram, gauge
    
    def __post_init__(self):
        if self.labels is None:
            self.labels = {}


@dataclass
class HistogramBucket:
    """直方图桶"""
    le: float  # less equal 上界
    count: int


@dataclass
class MetricSummary:
    """指标汇总"""
    name: str
    count: int
    sum: float
    min_value: float
    max_value: float
    avg_value: float
    p50: float
    p95: float
    p99: float
    last_updated: float


class MetricsCollector:
    """
    📊 企业级指标收集器
    
    功能特性：
    - 高性能内存存储
    - 多种指标类型支持
    - 自动聚合计算
    - 时序数据导出
    - SQLite持久化
    """
    
    def __init__(self, db_path: str = "data/metrics.db", 
                 retention_hours: int = 24,
                 aggregation_interval: int = 60):
        
        self.db_path = db_path
        self.retention_hours = retention_hours
        self.aggregation_interval = aggregation_interval
        
        # 内存存储
        self._counters: Dict[str, float] = defaultdict(float)
        self._gauges: Dict[str, float] = {}
        self._histograms: Dict[str, deque] = defaultdict(lambda: deque(maxlen=10000))
        
        # 时序数据 (最近N小时)
        max_points = retention_hours * 3600 // aggregation_interval
        self._timeseries: Dict[str, deque] = defaultdict(
            lambda: deque(maxlen=max_points)
        )
        
        # 标签索引
        self._label_index: Dict[str, Dict[str, List[str]]] = defaultdict(
            lambda: defaultdict(list)
        )
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 聚合线程
        self._aggregation_thread = None
        self._stop_aggregation = False
        
        # 告警回调
        self._alert_callbacks: List[Callable] = []
        self._alert_thresholds: Dict[str, Dict[str, float]] = {}
        
        # 初始化
        self._init_database()
        self._start_aggregation()
    
    def _init_database(self):
        """初始化数据库"""
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            # 创建指标表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS metrics (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    value REAL NOT NULL,
                    timestamp REAL NOT NULL,
                    metric_type TEXT NOT NULL,
                    labels TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            # 创建聚合表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS metric_aggregates (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    period_start REAL NOT NULL,
                    period_end REAL NOT NULL,
                    count INTEGER NOT NULL,
                    sum_value REAL NOT NULL,
                    min_value REAL NOT NULL,
                    max_value REAL NOT NULL,
                    avg_value REAL NOT NULL,
                    p50_value REAL,
                    p95_value REAL,
                    p99_value REAL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            # 创建索引
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_metrics_name_time ON metrics(name, timestamp)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_aggregates_name_period ON metric_aggregates(name, period_start)')
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 初始化指标数据库失败: {e}")
    
    def record_counter(self, name: str, increment: float = 1.0, 
                      labels: Dict[str, str] = None):
        """记录计数器指标"""
        with self._lock:
            full_name = self._get_metric_name_with_labels(name, labels)
            self._counters[full_name] += increment
            
            # 记录到时序数据
            point = MetricPoint(
                name=name,
                value=increment,
                timestamp=time.time(),
                labels=labels,
                metric_type="counter"
            )
            self._timeseries[full_name].append(point)
            
            # 更新标签索引
            if labels:
                for key, value in labels.items():
                    self._label_index[name][key].append(value)
    
    def record_gauge(self, name: str, value: float, 
                    labels: Dict[str, str] = None):
        """记录仪表指标"""
        with self._lock:
            full_name = self._get_metric_name_with_labels(name, labels)
            self._gauges[full_name] = value
            
            # 记录到时序数据
            point = MetricPoint(
                name=name,
                value=value,
                timestamp=time.time(),
                labels=labels,
                metric_type="gauge"
            )
            self._timeseries[full_name].append(point)
    
    def record_histogram(self, name: str, value: float, 
                        labels: Dict[str, str] = None):
        """记录直方图指标"""
        with self._lock:
            full_name = self._get_metric_name_with_labels(name, labels)
            self._histograms[full_name].append(value)
            
            # 记录到时序数据
            point = MetricPoint(
                name=name,
                value=value,
                timestamp=time.time(),
                labels=labels,
                metric_type="histogram"
            )
            self._timeseries[full_name].append(point)
    
    def _get_metric_name_with_labels(self, name: str, 
                                   labels: Dict[str, str] = None) -> str:
        """生成带标签的指标名称"""
        if not labels:
            return name
        
        label_str = ",".join(f"{k}={v}" for k, v in sorted(labels.items()))
        return f"{name}{{{label_str}}}"
    
    def get_counter_value(self, name: str, labels: Dict[str, str] = None) -> float:
        """获取计数器值"""
        full_name = self._get_metric_name_with_labels(name, labels)
        return self._counters.get(full_name, 0.0)
    
    def get_gauge_value(self, name: str, labels: Dict[str, str] = None) -> Optional[float]:
        """获取仪表值"""
        full_name = self._get_metric_name_with_labels(name, labels)
        return self._gauges.get(full_name)
    
    def get_histogram_summary(self, name: str, 
                            labels: Dict[str, str] = None) -> Optional[MetricSummary]:
        """获取直方图汇总"""
        full_name = self._get_metric_name_with_labels(name, labels)
        values = list(self._histograms.get(full_name, []))
        
        if not values:
            return None
        
        sorted_values = sorted(values)
        count = len(values)
        
        return MetricSummary(
            name=name,
            count=count,
            sum=sum(values),
            min_value=min(values),
            max_value=max(values),
            avg_value=statistics.mean(values),
            p50=sorted_values[int(count * 0.5)] if count > 0 else 0,
            p95=sorted_values[int(count * 0.95)] if count > 0 else 0,
            p99=sorted_values[int(count * 0.99)] if count > 0 else 0,
            last_updated=time.time()
        )
    
    def get_timeseries(self, name: str, labels: Dict[str, str] = None,
                      start_time: Optional[float] = None,
                      end_time: Optional[float] = None) -> List[MetricPoint]:
        """获取时序数据"""
        full_name = self._get_metric_name_with_labels(name, labels)
        points = list(self._timeseries.get(full_name, []))
        
        # 时间过滤
        if start_time or end_time:
            filtered_points = []
            for point in points:
                if start_time and point.timestamp < start_time:
                    continue
                if end_time and point.timestamp > end_time:
                    continue
                filtered_points.append(point)
            points = filtered_points
        
        return points
    
    def get_all_metrics(self) -> Dict[str, Any]:
        """获取所有指标快照"""
        with self._lock:
            snapshot = {
                "timestamp": time.time(),
                "counters": dict(self._counters),
                "gauges": dict(self._gauges),
                "histograms": {}
            }
            
            # 计算直方图统计
            for name, values in self._histograms.items():
                if values:
                    snapshot["histograms"][name] = {
                        "count": len(values),
                        "sum": sum(values),
                        "min": min(values),
                        "max": max(values),
                        "avg": statistics.mean(values)
                    }
            
            return snapshot
    
    def export_prometheus_format(self) -> str:
        """导出Prometheus格式指标"""
        lines = []
        timestamp_ms = int(time.time() * 1000)
        
        with self._lock:
            # 导出计数器
            for name, value in self._counters.items():
                lines.append(f"# TYPE {name} counter")
                lines.append(f"{name} {value} {timestamp_ms}")
            
            # 导出仪表
            for name, value in self._gauges.items():
                lines.append(f"# TYPE {name} gauge") 
                lines.append(f"{name} {value} {timestamp_ms}")
            
            # 导出直方图
            for name, values in self._histograms.items():
                if values:
                    lines.append(f"# TYPE {name} histogram")
                    lines.append(f"{name}_count {len(values)} {timestamp_ms}")
                    lines.append(f"{name}_sum {sum(values)} {timestamp_ms}")
                    
                    # 添加分位数
                    sorted_vals = sorted(values)
                    count = len(sorted_vals)
                    if count > 0:
                        p50 = sorted_vals[int(count * 0.5)]
                        p95 = sorted_vals[int(count * 0.95)]
                        p99 = sorted_vals[int(count * 0.99)]
                        
                        lines.append(f'{name}{{quantile="0.5"}} {p50} {timestamp_ms}')
                        lines.append(f'{name}{{quantile="0.95"}} {p95} {timestamp_ms}')
                        lines.append(f'{name}{{quantile="0.99"}} {p99} {timestamp_ms}')
        
        return "\n".join(lines) + "\n"
    
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
        while not self._stop_aggregation:
            try:
                self._perform_aggregation()
                self._check_alerts()
                self._cleanup_old_data()
                
                time.sleep(self.aggregation_interval)
                
            except Exception as e:
                print(f"⚠️ 指标聚合异常: {e}")
                time.sleep(10)
    
    def _perform_aggregation(self):
        """执行聚合计算"""
        current_time = time.time()
        period_start = current_time - self.aggregation_interval
        
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            with self._lock:
                # 聚合直方图数据
                for name, values in self._histograms.items():
                    if not values:
                        continue
                    
                    # 计算统计值
                    vals_list = list(values)
                    sorted_vals = sorted(vals_list)
                    count = len(vals_list)
                    
                    if count > 0:
                        summary = MetricSummary(
                            name=name,
                            count=count,
                            sum=sum(vals_list),
                            min_value=min(vals_list),
                            max_value=max(vals_list),
                            avg_value=statistics.mean(vals_list),
                            p50=sorted_vals[int(count * 0.5)],
                            p95=sorted_vals[int(count * 0.95)],
                            p99=sorted_vals[int(count * 0.99)],
                            last_updated=current_time
                        )
                        
                        # 保存到数据库
                        cursor.execute('''
                            INSERT INTO metric_aggregates 
                            (name, period_start, period_end, count, sum_value, 
                             min_value, max_value, avg_value, p50_value, p95_value, p99_value)
                            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                        ''', (
                            name, period_start, current_time,
                            summary.count, summary.sum, summary.min_value,
                            summary.max_value, summary.avg_value,
                            summary.p50, summary.p95, summary.p99
                        ))
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 聚合数据保存失败: {e}")
    
    def _check_alerts(self):
        """检查告警阈值"""
        for metric_name, thresholds in self._alert_thresholds.items():
            try:
                current_value = None
                
                # 获取当前值
                if metric_name in self._gauges:
                    current_value = self._gauges[metric_name]
                elif metric_name in self._counters:
                    current_value = self._counters[metric_name]
                
                if current_value is None:
                    continue
                
                # 检查阈值
                for threshold_type, threshold_value in thresholds.items():
                    should_alert = False
                    
                    if threshold_type == "max" and current_value > threshold_value:
                        should_alert = True
                    elif threshold_type == "min" and current_value < threshold_value:
                        should_alert = True
                    
                    if should_alert:
                        alert_data = {
                            "metric": metric_name,
                            "value": current_value,
                            "threshold_type": threshold_type,
                            "threshold_value": threshold_value,
                            "timestamp": time.time()
                        }
                        
                        self._trigger_alert(alert_data)
                        
            except Exception as e:
                print(f"⚠️ 检查告警失败 {metric_name}: {e}")
    
    def _trigger_alert(self, alert_data: Dict[str, Any]):
        """触发告警"""
        for callback in self._alert_callbacks:
            try:
                callback(alert_data)
            except Exception as e:
                print(f"⚠️ 告警回调执行失败: {e}")
    
    def _cleanup_old_data(self):
        """清理过期数据"""
        cutoff_time = time.time() - (self.retention_hours * 3600)
        
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            # 清理原始指标
            cursor.execute('DELETE FROM metrics WHERE timestamp < ?', (cutoff_time,))
            
            # 清理聚合数据
            cursor.execute('DELETE FROM metric_aggregates WHERE period_start < ?', (cutoff_time,))
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 清理过期数据失败: {e}")
    
    def add_alert_threshold(self, metric_name: str, threshold_type: str, 
                           threshold_value: float):
        """添加告警阈值"""
        if metric_name not in self._alert_thresholds:
            self._alert_thresholds[metric_name] = {}
        
        self._alert_thresholds[metric_name][threshold_type] = threshold_value
    
    def add_alert_callback(self, callback: Callable):
        """添加告警回调"""
        self._alert_callbacks.append(callback)
    
    def get_metric_names(self) -> List[str]:
        """获取所有指标名称"""
        names = set()
        names.update(self._counters.keys())
        names.update(self._gauges.keys()) 
        names.update(self._histograms.keys())
        return list(names)
    
    def shutdown(self):
        """关闭指标收集器"""
        self._stop_aggregation = True
        if self._aggregation_thread:
            self._aggregation_thread.join(timeout=5.0)
        
        print("📊 指标收集器已关闭")
