"""
⚠️ PIXLY v3.0 统一错误处理系统

基于Go废弃模块errors.go的错误处理架构：
- 响亮报错 > 静默降级
- 统一错误码系统
- 错误上下文追踪
- 多级错误严重度
- 错误聚合分析

迁移自：Go errors.go 的错误处理逻辑
"""

import time
import json
import threading
from enum import Enum
from dataclasses import dataclass, asdict
from typing import Dict, List, Optional, Any, Set
from collections import defaultdict, Counter


class ErrorSeverity(Enum):
    """错误严重级别"""
    INFO = "INFO"           # 信息提示
    WARNING = "WARNING"     # 警告（不影响主流程）
    ERROR = "ERROR"         # 业务级错误
    CRITICAL = "CRITICAL"   # 系统级致命错误


class ErrorCategory(Enum):
    """错误分类"""
    VALIDATION = "VAL"      # 验证类错误
    FILE = "FILE"           # 文件类错误
    NETWORK = "NET"         # 网络类错误（已废弃，仅兼容）
    SYSTEM = "SYS"          # 系统类错误
    BUSINESS = "BIZ"        # 业务类错误
    PLUGIN = "PLG"          # 插件类错误
    PERFORMANCE = "PERF"    # 性能类错误


@dataclass
class ValidationError:
    """统一错误结构"""
    code: str                              # 错误码
    message: str                           # 错误消息
    severity: ErrorSeverity                # 严重级别
    category: Optional[ErrorCategory] = None
    context: Dict[str, Any] = None         # 上下文信息
    timestamp: float = None                # 时间戳
    cause: Optional[str] = None            # 原始错误
    component: Optional[str] = None        # 组件名称
    trace_id: Optional[str] = None         # 追踪ID
    
    def __post_init__(self):
        if self.timestamp is None:
            self.timestamp = time.time()
        if self.context is None:
            self.context = {}
        if self.category is None:
            self.category = self._infer_category()
    
    def _infer_category(self) -> ErrorCategory:
        """推断错误分类"""
        code_upper = self.code.upper()
        
        if "VAL" in code_upper or "VALIDATION" in code_upper:
            return ErrorCategory.VALIDATION
        elif "FILE" in code_upper or "PATH" in code_upper:
            return ErrorCategory.FILE
        elif "SYS" in code_upper or "SYSTEM" in code_upper:
            return ErrorCategory.SYSTEM
        elif "BIZ" in code_upper or "BUSINESS" in code_upper:
            return ErrorCategory.BUSINESS
        elif "PLG" in code_upper or "PLUGIN" in code_upper:
            return ErrorCategory.PLUGIN
        elif "PERF" in code_upper or "PERFORMANCE" in code_upper:
            return ErrorCategory.PERFORMANCE
        else:
            return ErrorCategory.BUSINESS  # 默认业务错误
    
    def with_context(self, key: str, value: Any) -> 'ValidationError':
        """添加上下文信息"""
        if self.context is None:
            self.context = {}
        self.context[key] = value
        return self
    
    def with_cause(self, cause: str) -> 'ValidationError':
        """设置原始错误"""
        self.cause = cause
        return self
    
    def with_component(self, component: str) -> 'ValidationError':
        """设置组件名称"""
        self.component = component
        return self
    
    def with_trace_id(self, trace_id: str) -> 'ValidationError':
        """设置追踪ID"""
        self.trace_id = trace_id
        return self
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典"""
        data = asdict(self)
        data["severity"] = self.severity.value
        data["category"] = self.category.value if self.category else None
        return data
    
    def to_json(self) -> str:
        """转换为JSON"""
        return json.dumps(self.to_dict(), ensure_ascii=False, indent=2)


class ErrorPattern:
    """错误模式识别"""
    
    def __init__(self, pattern: str, description: str, 
                 threshold: int = 5, time_window: int = 300):
        self.pattern = pattern
        self.description = description
        self.threshold = threshold  # 阈值次数
        self.time_window = time_window  # 时间窗口（秒）
        self.occurrences: List[float] = []  # 发生时间
    
    def match(self, error: ValidationError) -> bool:
        """检查错误是否匹配模式"""
        return (self.pattern.lower() in error.code.lower() or 
                self.pattern.lower() in error.message.lower())
    
    def record_occurrence(self, timestamp: float = None):
        """记录发生"""
        if timestamp is None:
            timestamp = time.time()
        
        self.occurrences.append(timestamp)
        
        # 清理过期记录
        cutoff = timestamp - self.time_window
        self.occurrences = [t for t in self.occurrences if t > cutoff]
    
    def is_triggered(self) -> bool:
        """检查是否触发模式"""
        return len(self.occurrences) >= self.threshold
    
    def get_frequency(self) -> float:
        """获取频率（次/分钟）"""
        if not self.occurrences:
            return 0.0
        
        time_span = max(self.occurrences) - min(self.occurrences)
        if time_span == 0:
            return len(self.occurrences)
        
        return len(self.occurrences) / (time_span / 60)


class ValidationErrorCollector:
    """
    ⚠️ 企业级错误收集器
    
    功能特性：
    - 错误聚合统计
    - 模式识别告警
    - 错误趋势分析
    - 自动分类标记
    """
    
    def __init__(self, max_errors: int = 10000):
        self.max_errors = max_errors
        
        # 错误存储
        self.errors: List[ValidationError] = []
        self.error_counts: Counter = Counter()
        self.severity_counts: Counter = Counter()
        self.category_counts: Counter = Counter()
        
        # 错误模式
        self.patterns: List[ErrorPattern] = []
        
        # 时间窗口统计
        self.hourly_stats: Dict[str, Dict[str, int]] = defaultdict(lambda: defaultdict(int))
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 初始化默认模式
        self._init_default_patterns()
    
    def _init_default_patterns(self):
        """初始化默认错误模式"""
        self.patterns = [
            ErrorPattern(
                "validation", 
                "高频验证错误", 
                threshold=10, 
                time_window=300
            ),
            ErrorPattern(
                "file_not_found", 
                "文件不存在错误", 
                threshold=5, 
                time_window=180
            ),
            ErrorPattern(
                "permission", 
                "权限错误", 
                threshold=3, 
                time_window=300
            ),
            ErrorPattern(
                "timeout", 
                "超时错误", 
                threshold=5, 
                time_window=600
            ),
            ErrorPattern(
                "memory", 
                "内存不足错误", 
                threshold=2, 
                time_window=300
            )
        ]
    
    def collect_error(self, error: ValidationError):
        """收集错误"""
        with self._lock:
            # 添加到错误列表
            self.errors.append(error)
            
            # 限制错误数量
            if len(self.errors) > self.max_errors:
                self.errors = self.errors[-self.max_errors:]
            
            # 更新统计
            self.error_counts[error.code] += 1
            self.severity_counts[error.severity.value] += 1
            if error.category:
                self.category_counts[error.category.value] += 1
            
            # 更新小时统计
            hour_key = time.strftime("%Y-%m-%d %H", time.localtime(error.timestamp))
            self.hourly_stats[hour_key]["total"] += 1
            self.hourly_stats[hour_key][error.severity.value] += 1
            
            # 检查错误模式
            self._check_error_patterns(error)
    
    def _check_error_patterns(self, error: ValidationError):
        """检查错误模式"""
        for pattern in self.patterns:
            if pattern.match(error):
                pattern.record_occurrence(error.timestamp)
                
                if pattern.is_triggered():
                    self._handle_pattern_trigger(pattern, error)
    
    def _handle_pattern_trigger(self, pattern: ErrorPattern, error: ValidationError):
        """处理模式触发"""
        print(f"🚨 错误模式触发: {pattern.description}")
        print(f"   模式: {pattern.pattern}")
        print(f"   频率: {pattern.get_frequency():.1f} 次/分钟")
        print(f"   最新错误: {error.code} - {error.message}")
        print("-" * 50)
    
    def get_error_summary(self) -> Dict[str, Any]:
        """获取错误摘要"""
        with self._lock:
            total_errors = len(self.errors)
            
            if total_errors == 0:
                return {
                    "total_errors": 0,
                    "severity_distribution": {},
                    "category_distribution": {},
                    "top_errors": [],
                    "recent_errors": [],
                    "error_trends": {},
                    "patterns_triggered": []
                }
            
            # 最近错误
            recent_errors = self.errors[-10:]
            
            # 错误趋势
            now = time.time()
            last_24h = [e for e in self.errors if now - e.timestamp <= 86400]
            last_1h = [e for e in self.errors if now - e.timestamp <= 3600]
            
            # 触发的模式
            triggered_patterns = [
                {
                    "pattern": p.pattern,
                    "description": p.description,
                    "frequency": p.get_frequency(),
                    "occurrences": len(p.occurrences)
                }
                for p in self.patterns if p.is_triggered()
            ]
            
            return {
                "total_errors": total_errors,
                "severity_distribution": dict(self.severity_counts),
                "category_distribution": dict(self.category_counts),
                "top_errors": self.error_counts.most_common(10),
                "recent_errors": [e.to_dict() for e in recent_errors],
                "error_trends": {
                    "last_24h": len(last_24h),
                    "last_1h": len(last_1h),
                    "hourly_stats": dict(self.hourly_stats)
                },
                "patterns_triggered": triggered_patterns,
                "error_rate": {
                    "per_hour": len(last_1h),
                    "per_day": len(last_24h)
                }
            }
    
    def get_errors_by_severity(self, severity: ErrorSeverity, 
                              limit: int = 100) -> List[ValidationError]:
        """按严重级别获取错误"""
        with self._lock:
            return [e for e in self.errors if e.severity == severity][-limit:]
    
    def get_errors_by_category(self, category: ErrorCategory, 
                              limit: int = 100) -> List[ValidationError]:
        """按分类获取错误"""
        with self._lock:
            return [e for e in self.errors if e.category == category][-limit:]
    
    def get_errors_by_code(self, code: str, 
                          limit: int = 100) -> List[ValidationError]:
        """按错误码获取错误"""
        with self._lock:
            return [e for e in self.errors if e.code == code][-limit:]
    
    def get_recent_errors(self, minutes: int = 60, 
                         limit: int = 100) -> List[ValidationError]:
        """获取最近错误"""
        cutoff = time.time() - (minutes * 60)
        
        with self._lock:
            recent = [e for e in self.errors if e.timestamp > cutoff]
            return recent[-limit:]
    
    def search_errors(self, query: str, limit: int = 100) -> List[ValidationError]:
        """搜索错误"""
        query_lower = query.lower()
        
        with self._lock:
            matches = []
            for error in self.errors:
                if (query_lower in error.code.lower() or 
                    query_lower in error.message.lower() or
                    (error.component and query_lower in error.component.lower())):
                    matches.append(error)
            
            return matches[-limit:]
    
    def add_error_pattern(self, pattern: str, description: str,
                         threshold: int = 5, time_window: int = 300):
        """添加错误模式"""
        new_pattern = ErrorPattern(pattern, description, threshold, time_window)
        self.patterns.append(new_pattern)
    
    def clear_errors(self):
        """清空错误记录"""
        with self._lock:
            self.errors.clear()
            self.error_counts.clear()
            self.severity_counts.clear()
            self.category_counts.clear()
            self.hourly_stats.clear()
            
            # 重置模式
            for pattern in self.patterns:
                pattern.occurrences.clear()
    
    def export_errors(self, format: str = "json", 
                     limit: int = 1000) -> str:
        """导出错误数据"""
        with self._lock:
            errors_to_export = self.errors[-limit:]
            
            if format == "json":
                return json.dumps(
                    [e.to_dict() for e in errors_to_export],
                    ensure_ascii=False,
                    indent=2
                )
            elif format == "csv":
                import csv
                import io
                
                output = io.StringIO()
                writer = csv.writer(output)
                
                # 写入头部
                writer.writerow([
                    "timestamp", "code", "message", "severity", 
                    "category", "component", "trace_id"
                ])
                
                # 写入数据
                for error in errors_to_export:
                    writer.writerow([
                        time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(error.timestamp)),
                        error.code,
                        error.message,
                        error.severity.value,
                        error.category.value if error.category else "",
                        error.component or "",
                        error.trace_id or ""
                    ])
                
                return output.getvalue()
            else:
                raise ValueError(f"不支持的导出格式: {format}")
    
    def get_error_analytics(self) -> Dict[str, Any]:
        """获取错误分析"""
        with self._lock:
            if not self.errors:
                return {"message": "无错误数据"}
            
            now = time.time()
            
            # 时间分析
            timestamps = [e.timestamp for e in self.errors]
            time_span = max(timestamps) - min(timestamps)
            
            # 严重度分析
            critical_errors = [e for e in self.errors if e.severity == ErrorSeverity.CRITICAL]
            error_errors = [e for e in self.errors if e.severity == ErrorSeverity.ERROR]
            
            # 组件分析
            component_errors = defaultdict(int)
            for error in self.errors:
                if error.component:
                    component_errors[error.component] += 1
            
            # 趋势分析
            last_hour_errors = len([e for e in self.errors if now - e.timestamp <= 3600])
            last_day_errors = len([e for e in self.errors if now - e.timestamp <= 86400])
            
            return {
                "total_errors": len(self.errors),
                "time_span_hours": time_span / 3600,
                "error_rate_per_hour": len(self.errors) / max(time_span / 3600, 1),
                "critical_errors": len(critical_errors),
                "error_level_errors": len(error_errors),
                "most_problematic_component": max(component_errors.items(), key=lambda x: x[1]) if component_errors else None,
                "recent_trend": {
                    "last_hour": last_hour_errors,
                    "last_day": last_day_errors
                },
                "patterns_detected": len([p for p in self.patterns if p.is_triggered()]),
                "recommendations": self._generate_recommendations()
            }
    
    def _generate_recommendations(self) -> List[str]:
        """生成改进建议"""
        recommendations = []
        
        with self._lock:
            # 基于严重度的建议
            critical_count = self.severity_counts.get("CRITICAL", 0)
            if critical_count > 0:
                recommendations.append(f"发现 {critical_count} 个严重错误，建议立即处理")
            
            # 基于错误频率的建议
            if self.error_counts:
                most_common = self.error_counts.most_common(1)[0]
                if most_common[1] > 10:
                    recommendations.append(f"错误 '{most_common[0]}' 频繁出现({most_common[1]}次)，建议重点排查")
            
            # 基于模式的建议
            triggered_patterns = [p for p in self.patterns if p.is_triggered()]
            if triggered_patterns:
                recommendations.append(f"检测到 {len(triggered_patterns)} 个错误模式，建议进行系统性排查")
        
        return recommendations
