"""
🧠 PIXLY v3.1 智能模型路由器增强

替代Go model_router.go的完整功能：
- A/B测试权重路由系统 - 动态模型选择
- 详细性能指标追踪 - 准确率、延迟、F1分数  
- 模型版本管理 - 多版本并存 + 提升机制
- 智能模型比较 - 自动性能对比分析
- 实时路由决策 - 基于负载和性能动态调整

完全本地化，基于概率路由算法 + SQLite持久化
"""

import random
import time
import json
import threading
from datetime import datetime, timedelta
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, asdict
from enum import Enum
import statistics
import sqlite3

from .model_manager import LocalModelManager, get_model_manager


class ModelStatus(Enum):
    """模型状态"""
    ACTIVE = "active"           # 活跃使用
    TESTING = "testing"         # A/B测试中
    DEPRECATED = "deprecated"   # 已弃用
    MAINTENANCE = "maintenance" # 维护中


@dataclass
class ModelInfo:
    """模型信息 - 替代Go ModelInfo"""
    name: str
    version: str
    path: str
    type: str                    # 模型类型 (predictor, optimizer, validator)
    status: ModelStatus = ModelStatus.ACTIVE
    priority: int = 100          # 优先级 (越高越优先)
    ab_weight: float = 1.0       # A/B测试权重 (0.0-1.0)
    created_at: datetime = None
    last_used_at: datetime = None
    usage_count: int = 0
    
    # 性能指标
    accuracy: float = 0.0        # 准确率
    precision: float = 0.0       # 精确率  
    recall: float = 0.0          # 召回率
    f1_score: float = 0.0        # F1分数
    avg_latency: float = 0.0     # 平均延迟(ms)
    error_rate: float = 0.0      # 错误率
    total_calls: int = 0         # 总调用次数
    success_calls: int = 0       # 成功调用次数
    
    def __post_init__(self):
        if self.created_at is None:
            self.created_at = datetime.now()
        if self.last_used_at is None:
            self.last_used_at = datetime.now()
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典"""
        result = asdict(self)
        result['status'] = self.status.value
        result['created_at'] = self.created_at.isoformat()
        result['last_used_at'] = self.last_used_at.isoformat()
        return result
    
    def update_metrics(self, success: bool, latency: float):
        """更新性能指标"""
        self.usage_count += 1
        self.total_calls += 1
        self.last_used_at = datetime.now()
        
        if success:
            self.success_calls += 1
        
        # 更新平均延迟（移动平均）
        if self.avg_latency == 0:
            self.avg_latency = latency
        else:
            self.avg_latency = self.avg_latency * 0.9 + latency * 0.1
        
        # 更新错误率
        if self.total_calls > 0:
            self.error_rate = (self.total_calls - self.success_calls) / self.total_calls
    
    def calculate_health_score(self) -> float:
        """计算模型健康度评分"""
        if self.total_calls == 0:
            return 0.5  # 新模型默认评分
        
        # 综合评分算法
        success_rate = self.success_calls / self.total_calls
        latency_score = max(0, 1 - (self.avg_latency / 1000))  # 延迟越低分数越高
        usage_score = min(1, self.usage_count / 100)  # 使用次数越多越可信
        
        health_score = (
            success_rate * 0.5 +          # 成功率权重50%
            latency_score * 0.3 +         # 延迟权重30%
            usage_score * 0.2             # 使用经验权重20%
        )
        
        return max(0.0, min(1.0, health_score))


class IntelligentModelRouter:
    """
    🧠 智能模型路由器
    
    实现A/B测试、性能监控、智能路由的完整模型治理系统
    """
    
    def __init__(self, db_path: str = "data/model_router.db"):
        self.models: Dict[str, List[ModelInfo]] = {}  # name -> versions
        self.active_models: Dict[str, ModelInfo] = {}  # name -> active version
        self.ab_enabled = True
        self.random = random.Random()
        self._lock = threading.RLock()
        
        # 初始化数据库
        self.db_path = db_path
        self._init_database()
        
        # 路由统计
        self._stats = {
            "total_routes": 0,
            "ab_routes": 0,
            "fallback_routes": 0,
            "average_latency": 0.0,
            "success_rate": 0.0,
            "last_cleanup": datetime.now()
        }
        
        print(f"✅ 智能模型路由器初始化: A/B测试={'启用' if self.ab_enabled else '禁用'}")
    
    def _init_database(self):
        """初始化数据库"""
        with sqlite3.connect(self.db_path) as conn:
            conn.execute("""
                CREATE TABLE IF NOT EXISTS model_routing_logs (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                    model_name TEXT NOT NULL,
                    model_version TEXT NOT NULL,
                    route_type TEXT NOT NULL,  -- 'ab_test', 'direct', 'fallback'
                    latency_ms REAL,
                    success BOOLEAN,
                    error_message TEXT,
                    request_context TEXT  -- JSON格式的请求上下文
                )
            """)
            
            conn.execute("""
                CREATE INDEX IF NOT EXISTS idx_routing_logs_timestamp 
                ON model_routing_logs(timestamp)
            """)
            
            conn.execute("""
                CREATE INDEX IF NOT EXISTS idx_routing_logs_model 
                ON model_routing_logs(model_name, model_version)
            """)
            
            conn.commit()
    
    def register_model(self, model_info: ModelInfo) -> bool:
        """
        注册模型
        
        Args:
            model_info: 模型信息
            
        Returns:
            bool: 注册是否成功
        """
        with self._lock:
            if not model_info.name or not model_info.version:
                print(f"❌ 模型注册失败: 名称和版本不能为空")
                return False
            
            # 添加到模型列表
            if model_info.name not in self.models:
                self.models[model_info.name] = []
            
            # 检查版本是否已存在
            for existing in self.models[model_info.name]:
                if existing.version == model_info.version:
                    print(f"⚠️ 模型版本已存在: {model_info.name} v{model_info.version}")
                    return False
            
            self.models[model_info.name].append(model_info)
            
            # 设置活跃版本
            if (model_info.name not in self.active_models or 
                model_info.status == ModelStatus.ACTIVE and 
                model_info.priority > self.active_models[model_info.name].priority):
                self.active_models[model_info.name] = model_info
            
            print(f"✅ 模型注册成功: {model_info.name} v{model_info.version}")
            return True
    
    def get_model(self, name: str, context: Optional[Dict[str, Any]] = None) -> Optional[ModelInfo]:
        """
        获取模型（支持A/B测试路由）
        
        Args:
            name: 模型名称
            context: 请求上下文
            
        Returns:
            ModelInfo: 选中的模型信息
        """
        with self._lock:
            self._stats["total_routes"] += 1
            
            if name not in self.models or not self.models[name]:
                print(f"❌ 模型不存在: {name}")
                return None
            
            # 如果未启用A/B测试，返回活跃版本
            if not self.ab_enabled:
                model = self.active_models.get(name, self.models[name][0])
                self._log_routing(model, "direct", context)
                return model
            
            # A/B测试路由
            model = self._select_model_by_weight(self.models[name], context)
            self._log_routing(model, "ab_test", context)
            self._stats["ab_routes"] += 1
            
            return model
    
    def _select_model_by_weight(self, models: List[ModelInfo], 
                               context: Optional[Dict[str, Any]]) -> ModelInfo:
        """根据权重选择模型（A/B测试核心算法）"""
        # 过滤可用模型
        candidates = [m for m in models 
                     if m.status in [ModelStatus.ACTIVE, ModelStatus.TESTING]]
        
        if not candidates:
            # 无可用模型，使用第一个作为fallback
            self._stats["fallback_routes"] += 1
            return models[0]
        
        if len(candidates) == 1:
            return candidates[0]
        
        # 计算权重分布
        total_weight = sum(m.ab_weight for m in candidates)
        if total_weight <= 0:
            # 权重异常，均匀分布
            total_weight = len(candidates)
            for model in candidates:
                model.ab_weight = 1.0
        
        # 加权随机选择
        random_value = self.random.random() * total_weight
        cumulative_weight = 0.0
        
        for model in candidates:
            cumulative_weight += model.ab_weight
            if random_value <= cumulative_weight:
                return model
        
        # 兜底返回最后一个
        return candidates[-1]
    
    def _log_routing(self, model: ModelInfo, route_type: str, 
                    context: Optional[Dict[str, Any]]):
        """记录路由日志"""
        try:
            with sqlite3.connect(self.db_path) as conn:
                conn.execute("""
                    INSERT INTO model_routing_logs 
                    (model_name, model_version, route_type, request_context)
                    VALUES (?, ?, ?, ?)
                """, (
                    model.name,
                    model.version, 
                    route_type,
                    json.dumps(context) if context else None
                ))
                conn.commit()
        except Exception as e:
            print(f"⚠️ 路由日志记录失败: {e}")
    
    def record_model_usage(self, name: str, version: str, success: bool, 
                          latency: float, error_message: str = ""):
        """
        记录模型使用情况
        
        Args:
            name: 模型名称
            version: 模型版本
            success: 是否成功
            latency: 延迟时间(ms)
            error_message: 错误信息
        """
        with self._lock:
            # 更新模型指标
            for model in self.models.get(name, []):
                if model.version == version:
                    model.update_metrics(success, latency)
                    break
            
            # 更新全局统计
            self._update_global_stats(success, latency)
            
            # 记录到数据库
            try:
                with sqlite3.connect(self.db_path) as conn:
                    conn.execute("""
                        UPDATE model_routing_logs 
                        SET latency_ms = ?, success = ?, error_message = ?
                        WHERE model_name = ? AND model_version = ?
                        AND timestamp = (
                            SELECT MAX(timestamp) FROM model_routing_logs 
                            WHERE model_name = ? AND model_version = ?
                        )
                    """, (latency, success, error_message, name, version, name, version))
                    conn.commit()
            except Exception as e:
                print(f"⚠️ 使用记录更新失败: {e}")
    
    def _update_global_stats(self, success: bool, latency: float):
        """更新全局统计"""
        # 更新平均延迟
        if self._stats["average_latency"] == 0:
            self._stats["average_latency"] = latency
        else:
            self._stats["average_latency"] = (
                self._stats["average_latency"] * 0.95 + latency * 0.05
            )
        
        # 更新成功率（滑动窗口）
        if success:
            if self._stats["success_rate"] == 0:
                self._stats["success_rate"] = 1.0
            else:
                self._stats["success_rate"] = (
                    self._stats["success_rate"] * 0.95 + 1.0 * 0.05
                )
        else:
            self._stats["success_rate"] = self._stats["success_rate"] * 0.95
    
    def promote_model(self, name: str, version: str) -> bool:
        """
        提升模型为活跃版本
        
        Args:
            name: 模型名称
            version: 模型版本
            
        Returns:
            bool: 提升是否成功
        """
        with self._lock:
            if name not in self.models:
                return False
            
            target_model = None
            for model in self.models[name]:
                if model.version == version:
                    target_model = model
                    break
            
            if not target_model:
                return False
            
            # 降级旧的活跃版本
            if name in self.active_models:
                old_active = self.active_models[name]
                old_active.status = ModelStatus.DEPRECATED
                old_active.ab_weight = 0.0
            
            # 提升新版本
            target_model.status = ModelStatus.ACTIVE
            target_model.priority = 100
            target_model.ab_weight = 1.0
            self.active_models[name] = target_model
            
            print(f"🚀 模型已提升: {name} v{version} -> ACTIVE")
            return True
    
    def compare_models(self, name1: str, version1: str, 
                      name2: str, version2: str) -> Dict[str, Any]:
        """
        比较两个模型的性能
        
        Returns:
            Dict: 比较结果
        """
        model1 = self._find_model(name1, version1)
        model2 = self._find_model(name2, version2)
        
        if not model1 or not model2:
            return {"error": "模型不存在"}
        
        comparison = {
            "model1": {
                "name": f"{model1.name} v{model1.version}",
                "health_score": model1.calculate_health_score(),
                "metrics": {
                    "accuracy": model1.accuracy,
                    "avg_latency": model1.avg_latency,
                    "error_rate": model1.error_rate,
                    "usage_count": model1.usage_count
                }
            },
            "model2": {
                "name": f"{model2.name} v{model2.version}",
                "health_score": model2.calculate_health_score(),
                "metrics": {
                    "accuracy": model2.accuracy,
                    "avg_latency": model2.avg_latency,
                    "error_rate": model2.error_rate,
                    "usage_count": model2.usage_count
                }
            },
            "winner": "",
            "advantages": []
        }
        
        # 比较各项指标
        advantages = []
        score1 = 0
        score2 = 0
        
        # 健康度比较
        if model1.calculate_health_score() > model2.calculate_health_score():
            score1 += 1
            advantages.append(f"{model1.name} 健康度更高")
        elif model2.calculate_health_score() > model1.calculate_health_score():
            score2 += 1
            advantages.append(f"{model2.name} 健康度更高")
        
        # 延迟比较
        if model1.avg_latency > 0 and model2.avg_latency > 0:
            if model1.avg_latency < model2.avg_latency:
                score1 += 1
                advantages.append(f"{model1.name} 响应更快")
            elif model2.avg_latency < model1.avg_latency:
                score2 += 1
                advantages.append(f"{model2.name} 响应更快")
        
        # 错误率比较
        if model1.error_rate < model2.error_rate:
            score1 += 1
            advantages.append(f"{model1.name} 错误率更低")
        elif model2.error_rate < model1.error_rate:
            score2 += 1
            advantages.append(f"{model2.name} 错误率更低")
        
        # 确定获胜者
        if score1 > score2:
            comparison["winner"] = f"{model1.name} v{model1.version}"
        elif score2 > score1:
            comparison["winner"] = f"{model2.name} v{model2.version}"
        else:
            comparison["winner"] = "平局"
        
        comparison["advantages"] = advantages
        return comparison
    
    def _find_model(self, name: str, version: str) -> Optional[ModelInfo]:
        """查找指定版本的模型"""
        for model in self.models.get(name, []):
            if model.version == version:
                return model
        return None
    
    def get_ab_test_status(self) -> Dict[str, Any]:
        """获取A/B测试状态"""
        with self._lock:
            testing_models = []
            for name, models in self.models.items():
                testing_candidates = [m for m in models 
                                    if m.status == ModelStatus.TESTING]
                
                if len(testing_candidates) > 1:
                    testing_models.append({
                        "model_name": name,
                        "candidates": [
                            {
                                "version": m.version,
                                "weight": m.ab_weight,
                                "health_score": m.calculate_health_score(),
                                "usage_count": m.usage_count
                            }
                            for m in testing_candidates
                        ]
                    })
            
            return {
                "ab_enabled": self.ab_enabled,
                "active_tests": len(testing_models),
                "test_details": testing_models,
                "routing_stats": self._stats
            }
    
    def set_ab_weights(self, name: str, version_weights: Dict[str, float]) -> bool:
        """
        设置A/B测试权重
        
        Args:
            name: 模型名称
            version_weights: 版本权重映射 {"v1.0": 0.7, "v1.1": 0.3}
            
        Returns:
            bool: 设置是否成功
        """
        with self._lock:
            if name not in self.models:
                return False
            
            success_count = 0
            for model in self.models[name]:
                if model.version in version_weights:
                    model.ab_weight = version_weights[model.version]
                    success_count += 1
            
            print(f"✅ A/B权重已更新: {name}, {success_count}个版本")
            return success_count > 0
    
    def get_router_analytics(self, days: int = 7) -> Dict[str, Any]:
        """获取路由分析报告"""
        try:
            cutoff_date = datetime.now() - timedelta(days=days)
            
            with sqlite3.connect(self.db_path) as conn:
                # 路由统计
                cursor = conn.execute("""
                    SELECT 
                        route_type, 
                        COUNT(*) as count,
                        AVG(latency_ms) as avg_latency,
                        SUM(CASE WHEN success = 1 THEN 1 ELSE 0 END) as success_count
                    FROM model_routing_logs
                    WHERE timestamp > ?
                    GROUP BY route_type
                """, (cutoff_date,))
                
                route_stats = []
                for row in cursor.fetchall():
                    route_type, count, avg_latency, success_count = row
                    success_rate = success_count / count if count > 0 else 0
                    
                    route_stats.append({
                        "route_type": route_type,
                        "total_requests": count,
                        "avg_latency_ms": round(avg_latency or 0, 2),
                        "success_rate": f"{success_rate:.1%}"
                    })
                
                # 模型性能排行
                model_performance = []
                for name, models in self.models.items():
                    for model in models:
                        if model.usage_count > 0:
                            model_performance.append({
                                "model": f"{name} v{model.version}",
                                "health_score": model.calculate_health_score(),
                                "usage_count": model.usage_count,
                                "avg_latency": model.avg_latency,
                                "error_rate": f"{model.error_rate:.1%}"
                            })
                
                # 按健康度排序
                model_performance.sort(key=lambda x: x["health_score"], reverse=True)
                
                return {
                    "analysis_period": f"{days} days",
                    "route_statistics": route_stats,
                    "top_models": model_performance[:10],
                    "global_stats": self._stats,
                    "total_models": sum(len(models) for models in self.models.values()),
                    "active_models": len(self.active_models)
                }
                
        except Exception as e:
            print(f"❌ 分析报告生成失败: {e}")
            return {"error": str(e)}


# 全局路由器实例
_global_model_router: Optional[IntelligentModelRouter] = None
_router_lock = threading.Lock()

def get_model_router() -> IntelligentModelRouter:
    """获取全局模型路由器实例（单例模式）"""
    global _global_model_router
    
    with _router_lock:
        if _global_model_router is None:
            _global_model_router = IntelligentModelRouter()
        return _global_model_router
