"""
👁️ PIXLY v3.0 可观测性引擎

企业级统一观测解决方案：
- 多维数据聚合
- 实时监控仪表板
- 智能异常检测  
- 性能趋势分析
- 本地化存储架构

完全本地化实现，零网络依赖
"""

import time
import json
import threading
import sqlite3
from dataclasses import dataclass, asdict
from typing import Dict, List, Optional, Any, Callable
from pathlib import Path
from collections import defaultdict, deque
import statistics

from ..monitoring.metrics_collector import MetricsCollector
from ..monitoring.health_monitor import HealthMonitor  
from ..monitoring.performance_tracer import PerformanceTracer
from ..logging.log_aggregator import LogAggregator
from ..validation.error_collector import ValidationErrorCollector


@dataclass
class ObservabilitySnapshot:
    """可观测性快照"""
    timestamp: float
    metrics_summary: Dict[str, Any]
    health_summary: Dict[str, Any] 
    performance_summary: Dict[str, Any]
    log_summary: Dict[str, Any]
    error_summary: Dict[str, Any]
    alerts: List[Dict[str, Any]]
    recommendations: List[str]


class ObservabilityEngine:
    """
    👁️ 企业级可观测性引擎
    
    功能特性：
    - 统一数据聚合
    - 实时监控面板
    - 智能分析报告
    - 异常检测告警
    """
    
    def __init__(self, db_path: str = "data/observability.db"):
        self.db_path = db_path
        
        # 核心组件集成
        self.metrics_collector = MetricsCollector()
        self.health_monitor = HealthMonitor(self.metrics_collector)
        self.performance_tracer = PerformanceTracer(metrics_collector=self.metrics_collector)
        self.log_aggregator = LogAggregator()
        self.error_collector = ValidationErrorCollector()
        
        # 快照历史
        self.snapshots: deque = deque(maxlen=1000)
        
        # 告警规则
        self.alert_rules: List[Dict[str, Any]] = []
        self.active_alerts: Dict[str, Dict[str, Any]] = {}
        
        # 监控线程
        self._monitor_thread = None
        self._stop_monitoring = False
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 初始化
        self._init_database()
        self._init_default_alerts()
        self.start_monitoring()
    
    def _init_database(self):
        """初始化观测数据库"""
        try:
            Path(self.db_path).parent.mkdir(parents=True, exist_ok=True)
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            # 快照表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS observability_snapshots (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp REAL NOT NULL,
                    metrics_summary TEXT,
                    health_summary TEXT,
                    performance_summary TEXT,
                    log_summary TEXT,
                    error_summary TEXT,
                    alerts TEXT,
                    recommendations TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            # 告警历史表
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS alert_history (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    alert_id TEXT NOT NULL,
                    rule_name TEXT NOT NULL,
                    severity TEXT NOT NULL,
                    message TEXT NOT NULL,
                    context TEXT,
                    triggered_at REAL NOT NULL,
                    resolved_at REAL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )
            ''')
            
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_snapshots_timestamp ON observability_snapshots(timestamp)')
            cursor.execute('CREATE INDEX IF NOT EXISTS idx_alerts_triggered ON alert_history(triggered_at)')
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 初始化观测数据库失败: {e}")
    
    def _init_default_alerts(self):
        """初始化默认告警规则"""
        self.alert_rules = [
            {
                "name": "high_error_rate",
                "description": "高错误率告警",
                "condition": "error_rate > 0.1",
                "severity": "critical",
                "cooldown": 300
            },
            {
                "name": "performance_degradation", 
                "description": "性能下降告警",
                "condition": "avg_response_time > 5000",
                "severity": "warning",
                "cooldown": 600
            },
            {
                "name": "system_resource_high",
                "description": "系统资源占用高",
                "condition": "cpu_usage > 80 or memory_usage > 85", 
                "severity": "warning",
                "cooldown": 300
            }
        ]
    
    def take_snapshot(self) -> ObservabilitySnapshot:
        """拍摄可观测性快照"""
        
        # 收集各组件数据
        metrics_summary = self.metrics_collector.get_all_metrics()
        health_summary = self.health_monitor.get_health_summary()
        performance_summary = self.performance_tracer.analyze_performance()
        log_summary = self.log_aggregator.export_analysis_report()
        error_summary = self.error_collector.get_error_summary()
        
        # 检测告警
        alerts = self._check_alert_conditions(
            metrics_summary, health_summary, performance_summary
        )
        
        # 生成建议
        recommendations = self._generate_recommendations(
            metrics_summary, health_summary, error_summary
        )
        
        snapshot = ObservabilitySnapshot(
            timestamp=time.time(),
            metrics_summary=metrics_summary,
            health_summary=health_summary,
            performance_summary=performance_summary,
            log_summary=log_summary,
            error_summary=error_summary,
            alerts=alerts,
            recommendations=recommendations
        )
        
        # 保存快照
        with self._lock:
            self.snapshots.append(snapshot)
        
        return snapshot
    
    def _check_alert_conditions(self, metrics: Dict, health: Dict, 
                               performance: Dict) -> List[Dict[str, Any]]:
        """检查告警条件"""
        alerts = []
        
        for rule in self.alert_rules:
            try:
                # 构建评估上下文
                context = {
                    "error_rate": health.get("status_counts", {}).get("critical", 0) / max(health.get("total_components", 1), 1),
                    "avg_response_time": performance.get("operations", {}).get("avg_duration_ms", 0),
                    "cpu_usage": 0,  # 从健康检查获取
                    "memory_usage": 0
                }
                
                # 尝试从健康检查获取系统指标
                for comp_name, comp_data in health.get("components", {}).items():
                    if "system" in comp_name:
                        # 简化处理，实际应该解析详细数据
                        break
                
                # 评估条件
                if eval(rule["condition"], {"__builtins__": {}}, context):
                    alert_id = f"{rule['name']}_{int(time.time())}"
                    
                    alert = {
                        "id": alert_id,
                        "rule_name": rule["name"],
                        "description": rule["description"],
                        "severity": rule["severity"],
                        "message": f"告警触发: {rule['description']}",
                        "context": context,
                        "triggered_at": time.time()
                    }
                    
                    alerts.append(alert)
                    
                    # 记录到活跃告警
                    self.active_alerts[alert_id] = alert
                    
            except Exception as e:
                print(f"⚠️ 告警规则评估失败 {rule['name']}: {e}")
        
        return alerts
    
    def _generate_recommendations(self, metrics: Dict, health: Dict, 
                                errors: Dict) -> List[str]:
        """生成优化建议"""
        recommendations = []
        
        # 基于健康状态的建议
        critical_count = health.get("status_counts", {}).get("critical", 0)
        if critical_count > 0:
            recommendations.append(f"发现{critical_count}个严重健康问题，建议立即检查")
        
        # 基于错误统计的建议
        error_rate = errors.get("error_rate", {}).get("per_hour", 0)
        if error_rate > 10:
            recommendations.append(f"每小时错误数({error_rate})过高，建议排查错误日志")
        
        # 基于性能的建议
        total_validations = metrics.get("counters", {}).get("total_validations", 0)
        if total_validations > 1000:
            recommendations.append("验证调用频繁，建议启用缓存优化")
        
        return recommendations
    
    def get_dashboard_data(self) -> Dict[str, Any]:
        """获取仪表板数据"""
        if not self.snapshots:
            return {"message": "暂无观测数据"}
        
        latest_snapshot = self.snapshots[-1]
        
        # 计算趋势数据
        recent_snapshots = list(self.snapshots)[-10:]
        trends = self._calculate_trends(recent_snapshots)
        
        return {
            "current_status": asdict(latest_snapshot),
            "trends": trends,
            "active_alerts_count": len(self.active_alerts),
            "total_snapshots": len(self.snapshots),
            "last_updated": latest_snapshot.timestamp
        }
    
    def _calculate_trends(self, snapshots: List[ObservabilitySnapshot]) -> Dict[str, Any]:
        """计算趋势数据"""
        if len(snapshots) < 2:
            return {}
        
        # 提取时间序列数据
        timestamps = [s.timestamp for s in snapshots]
        error_rates = []
        component_counts = []
        
        for snapshot in snapshots:
            health = snapshot.health_summary
            total_comps = health.get("total_components", 1)
            critical_comps = health.get("status_counts", {}).get("critical", 0)
            error_rate = critical_comps / max(total_comps, 1)
            
            error_rates.append(error_rate)
            component_counts.append(total_comps)
        
        return {
            "error_rate_trend": {
                "values": error_rates,
                "timestamps": timestamps,
                "avg": statistics.mean(error_rates) if error_rates else 0,
                "trend": "increasing" if len(error_rates) > 1 and error_rates[-1] > error_rates[0] else "stable"
            },
            "component_count_trend": {
                "values": component_counts,
                "current": component_counts[-1] if component_counts else 0,
                "change": component_counts[-1] - component_counts[0] if len(component_counts) > 1 else 0
            }
        }
    
    def start_monitoring(self):
        """启动监控"""
        if self._monitor_thread is not None:
            return
        
        self._stop_monitoring = False
        self._monitor_thread = threading.Thread(target=self._monitoring_loop)
        self._monitor_thread.daemon = True
        self._monitor_thread.start()
        
        print("👁️ 可观测性引擎已启动")
    
    def _monitoring_loop(self):
        """监控循环"""
        while not self._stop_monitoring:
            try:
                # 拍摄快照
                snapshot = self.take_snapshot()
                
                # 保存到数据库
                self._save_snapshot(snapshot)
                
                # 清理过期数据
                self._cleanup_old_data()
                
                time.sleep(60)  # 每分钟一次
                
            except Exception as e:
                print(f"⚠️ 可观测性监控异常: {e}")
                time.sleep(10)
    
    def _save_snapshot(self, snapshot: ObservabilitySnapshot):
        """保存快照到数据库"""
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            cursor.execute('''
                INSERT INTO observability_snapshots 
                (timestamp, metrics_summary, health_summary, performance_summary,
                 log_summary, error_summary, alerts, recommendations)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            ''', (
                snapshot.timestamp,
                json.dumps(snapshot.metrics_summary),
                json.dumps(snapshot.health_summary),
                json.dumps(snapshot.performance_summary),
                json.dumps(snapshot.log_summary),
                json.dumps(snapshot.error_summary),
                json.dumps(snapshot.alerts),
                json.dumps(snapshot.recommendations)
            ))
            
            # 保存告警历史
            for alert in snapshot.alerts:
                cursor.execute('''
                    INSERT INTO alert_history
                    (alert_id, rule_name, severity, message, context, triggered_at)
                    VALUES (?, ?, ?, ?, ?, ?)
                ''', (
                    alert["id"], alert["rule_name"], alert["severity"],
                    alert["message"], json.dumps(alert["context"]), alert["triggered_at"]
                ))
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 保存观测快照失败: {e}")
    
    def _cleanup_old_data(self):
        """清理过期数据"""
        try:
            cutoff_time = time.time() - (7 * 24 * 3600)  # 保留7天
            
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            cursor.execute('DELETE FROM observability_snapshots WHERE timestamp < ?', (cutoff_time,))
            cursor.execute('DELETE FROM alert_history WHERE triggered_at < ?', (cutoff_time,))
            
            conn.commit()
            conn.close()
            
        except Exception as e:
            print(f"⚠️ 清理观测数据失败: {e}")
    
    def get_system_report(self) -> Dict[str, Any]:
        """生成系统报告"""
        latest_snapshot = self.take_snapshot()
        
        # 系统健康评分
        health_score = self._calculate_health_score(latest_snapshot)
        
        # 性能评分
        performance_score = self._calculate_performance_score(latest_snapshot)
        
        return {
            "report_timestamp": latest_snapshot.timestamp,
            "overall_health_score": health_score,
            "performance_score": performance_score,
            "summary": {
                "total_components": latest_snapshot.health_summary.get("total_components", 0),
                "healthy_components": latest_snapshot.health_summary.get("status_counts", {}).get("healthy", 0),
                "active_alerts": len(latest_snapshot.alerts),
                "recommendations_count": len(latest_snapshot.recommendations)
            },
            "details": asdict(latest_snapshot),
            "recommendations": latest_snapshot.recommendations
        }
    
    def _calculate_health_score(self, snapshot: ObservabilitySnapshot) -> float:
        """计算健康评分 (0-100)"""
        health = snapshot.health_summary
        
        total = health.get("total_components", 1)
        healthy = health.get("status_counts", {}).get("healthy", 0)
        critical = health.get("status_counts", {}).get("critical", 0)
        
        if total == 0:
            return 100.0
        
        # 基础健康分数
        base_score = (healthy / total) * 100
        
        # 严重问题惩罚
        critical_penalty = (critical / total) * 30
        
        # 告警惩罚
        alert_penalty = min(len(snapshot.alerts) * 5, 20)
        
        score = max(0, base_score - critical_penalty - alert_penalty)
        return round(score, 1)
    
    def _calculate_performance_score(self, snapshot: ObservabilitySnapshot) -> float:
        """计算性能评分 (0-100)"""
        performance = snapshot.performance_summary
        
        # 基础性能分数
        base_score = 100.0
        
        # 基于平均响应时间的惩罚
        operations = performance.get("operations", {})
        for op_name, op_stats in operations.items():
            avg_time = op_stats.get("avg_duration_ms", 0)
            if avg_time > 1000:  # 1秒以上
                base_score -= min((avg_time - 1000) / 100, 20)
        
        # 基于错误率的惩罚  
        for op_name, op_stats in operations.items():
            error_rate = op_stats.get("error_rate", 0)
            if error_rate > 0.05:  # 5%以上错误率
                base_score -= error_rate * 30
        
        score = max(0, base_score)
        return round(score, 1)
    
    def stop_monitoring(self):
        """停止监控"""
        self._stop_monitoring = True
        if self._monitor_thread:
            self._monitor_thread.join(timeout=10.0)
        
        print("👁️ 可观测性引擎已停止")
    
    def shutdown(self):
        """关闭可观测性引擎"""
        self.stop_monitoring()
        
        # 关闭子组件
        if hasattr(self.health_monitor, 'shutdown'):
            self.health_monitor.shutdown()
        if hasattr(self.performance_tracer, 'shutdown'):
            self.performance_tracer.shutdown()
        if hasattr(self.log_aggregator, 'shutdown'):
            self.log_aggregator.shutdown()
        
        print("👁️ 可观测性引擎已关闭")
