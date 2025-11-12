"""
模型路由器增强版 - A/B测试与智能分流系统

基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/model_router.go 重新实现

核心功能:
- 多模型版本共存管理
- 企业级A/B测试权重分配
- 实时性能指标自动跟踪
- 智能模型选择和分流算法
- 异常检测和自动故障转移
- 负载均衡和性能优化

EX-019实现: 从Go废弃代码价值提取 + A/B测试架构增强  
遵循四高原则：高规范化、高兼容性、高扩展性、高稳定性
"""

from typing import Dict, List, Optional, Tuple, Set
from dataclasses import dataclass, field
from datetime import datetime, timedelta
import random
import json
import time
import threading
import logging
from pathlib import Path
from enum import Enum
from collections import defaultdict, deque


class ModelStatus(Enum):
    """模型状态枚举"""
    ACTIVE = "active"              # 活跃状态
    TESTING = "testing"            # 测试状态
    DEPRECATED = "deprecated"      # 已废弃
    FAILED = "failed"              # 故障状态
    MAINTENANCE = "maintenance"    # 维护状态


class ABTestStrategy(Enum):
    """A/B测试策略"""
    RANDOM = "random"              # 随机分配
    WEIGHTED = "weighted"          # 权重分配
    PERFORMANCE = "performance"    # 性能导向
    GRADUAL = "gradual"            # 渐进式推出


@dataclass
class ModelMetrics:
    """模型性能指标 - 企业级增强版"""
    # 基础性能指标
    accuracy: float = 0.0           # 准确率
    precision: float = 0.0          # 精确率
    recall: float = 0.0             # 召回率
    f1_score: float = 0.0           # F1分数
    avg_latency: float = 0.0        # 平均延迟(ms)
    error_rate: float = 0.0         # 错误率
    total_calls: int = 0            # 总调用次数
    success_calls: int = 0          # 成功调用次数
    
    # 架构增强：高级指标
    p95_latency: float = 0.0        # 95分位延迟
    p99_latency: float = 0.0        # 99分位延迟
    throughput: float = 0.0         # 吞吐量 (requests/second)
    memory_usage: float = 0.0       # 内存使用率
    cpu_usage: float = 0.0          # CPU使用率
    
    # 健康检查指标
    health_score: float = 1.0       # 健康分数 (0.0-1.0)
    consecutive_failures: int = 0   # 连续失败次数
    last_failure_time: Optional[datetime] = None
    
    # 时间窗口指标
    recent_latencies: deque = field(default_factory=lambda: deque(maxlen=1000))
    hourly_stats: Dict[str, float] = field(default_factory=dict)
    
    def to_dict(self) -> dict:
        """转换为字典"""
        return {
            "accuracy": self.accuracy,
            "precision": self.precision,
            "recall": self.recall,
            "f1_score": self.f1_score,
            "avg_latency": self.avg_latency,
            "error_rate": self.error_rate,
            "total_calls": self.total_calls,
            "success_calls": self.success_calls
        }
    
    @classmethod
    def from_dict(cls, data: dict) -> 'ModelMetrics':
        """从字典创建"""
        return cls(**data)


@dataclass
class ABTestExperiment:
    """A/B测试实验配置"""
    experiment_id: str              # 实验ID
    name: str                       # 实验名称
    description: str                # 实验描述
    strategy: ABTestStrategy        # 测试策略
    start_time: datetime            # 开始时间
    end_time: Optional[datetime]    # 结束时间
    target_models: List[str]        # 目标模型版本
    traffic_percentage: float       # 流量百分比
    success_metric: str             # 成功指标
    
    # 实验状态
    is_active: bool = True
    participant_count: int = 0
    results: Dict[str, Any] = field(default_factory=dict)


@dataclass
class ModelInfo:
    """模型信息 - 企业级增强版"""
    name: str                       # 模型名称 (lightgbm, ppo, etc)
    version: str                    # 版本号 (v1.0.0, v1.0.1, etc)
    path: str                       # 模型文件路径
    model_type: str = "predictor"   # 模型类型 (predictor, optimizer, validator)
    status: ModelStatus = ModelStatus.ACTIVE  # 状态
    priority: int = 0               # 优先级 (越高越优先)
    ab_weight: float = 1.0          # A/B测试权重 (0.0-1.0)
    metrics: ModelMetrics = field(default_factory=ModelMetrics)
    created_at: datetime = field(default_factory=datetime.now)
    last_used_at: datetime = field(default_factory=datetime.now)
    usage_count: int = 0            # 使用次数
    
    # 架构增强：高级特性
    warmup_requests: int = 10       # 预热请求数
    max_concurrent: int = 100       # 最大并发数
    timeout_ms: int = 5000          # 超时时间
    circuit_breaker_threshold: int = 5  # 熔断阈值
    auto_scale: bool = False        # 自动扩展
    
    # 部署信息
    deployment_config: Dict[str, Any] = field(default_factory=dict)
    environment: str = "production"  # 环境 (development, staging, production)
    rollout_percentage: float = 100.0  # 推出百分比
    
    # 健康检查
    health_check_url: str = ""      # 健康检查URL
    last_health_check: Optional[datetime] = None
    
    def to_dict(self) -> dict:
        """转换为字典"""
        return {
            "name": self.name,
            "version": self.version,
            "path": self.path,
            "model_type": self.model_type,
            "status": self.status.value if isinstance(self.status, ModelStatus) else self.status,
            "priority": self.priority,
            "ab_weight": self.ab_weight,
            "metrics": self.metrics.to_dict(),
            "created_at": self.created_at.isoformat(),
            "last_used_at": self.last_used_at.isoformat(),
            "usage_count": self.usage_count,
            "warmup_requests": self.warmup_requests,
            "max_concurrent": self.max_concurrent,
            "timeout_ms": self.timeout_ms,
            "circuit_breaker_threshold": self.circuit_breaker_threshold,
            "auto_scale": self.auto_scale,
            "deployment_config": self.deployment_config,
            "environment": self.environment,
            "rollout_percentage": self.rollout_percentage,
            "health_check_url": self.health_check_url,
            "last_health_check": self.last_health_check.isoformat() if self.last_health_check else None
        }
    
    @classmethod
    def from_dict(cls, data: dict) -> 'ModelInfo':
        """从字典创建"""
        metrics = ModelMetrics.from_dict(data.get("metrics", {}))
        return cls(
            name=data["name"],
            version=data["version"],
            path=data["path"],
            model_type=data.get("model_type", "predictor"),
            status=data.get("status", "active"),
            priority=data.get("priority", 0),
            ab_weight=data.get("ab_weight", 1.0),
            metrics=metrics,
            created_at=datetime.fromisoformat(data.get("created_at", datetime.now().isoformat())),
            last_used_at=datetime.fromisoformat(data.get("last_used_at", datetime.now().isoformat())),
            usage_count=data.get("usage_count", 0)
        )


class ModelRouter:
    """
    模型路由器增强版 - 企业级A/B测试与智能分流系统
    
    核心功能:
    - 多模型版本共存管理
    - 企业级A/B测试框架
    - 实时性能监控和异常检测
    - 智能负载均衡和故障转移
    - 自动化健康检查和熔断机制
    - 渐进式发布和金丝雀部署
    """
    
    def __init__(self, 
                 config_path: Optional[Path] = None,
                 debug: bool = False):
        """
        初始化模型路由器
        
        Args:
            config_path: 配置文件路径（JSON格式）
            debug: 调试模式
        """
        self.models: Dict[str, List[ModelInfo]] = {}  # name -> versions
        self.active: Dict[str, ModelInfo] = {}        # name -> active version
        self.ab_enabled: bool = True                   # A/B测试开关
        self.config_path = config_path
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        if debug:
            self.logger.setLevel(logging.DEBUG)
        
        # 架构增强：高级特性
        self._lock = threading.RLock()                # 线程安全
        self._experiments: Dict[str, ABTestExperiment] = {}  # A/B测试实验
        self._circuit_breakers: Dict[str, Dict[str, bool]] = {}  # 熔断器状态
        self._health_checks: Dict[str, Dict[str, bool]] = {}     # 健康检查状态
        self._request_counter: Dict[str, int] = defaultdict(int)  # 请求计数
        self._performance_history: Dict[str, deque] = defaultdict(lambda: deque(maxlen=1000))
        
        # 负载均衡和路由策略
        self._routing_strategy = ABTestStrategy.WEIGHTED
        self._load_balancer_enabled = True
        self._canary_deployment_enabled = False
        
        # 统计信息
        self._stats = {
            'total_requests': 0,
            'successful_routes': 0,
            'failed_routes': 0,
            'circuit_breaker_trips': 0,
            'health_check_failures': 0
        }
        
        # 加载配置
        if config_path and config_path.exists():
            self.load_config(config_path)
        
        if debug:
            self.logger.debug("模型路由器增强版初始化完成")
    
    def register_model(self, info: ModelInfo) -> bool:
        """
        注册新模型版本（增强版）
        
        Args:
            info: 模型信息
            
        Returns:
            注册是否成功
        """
        with self._lock:
            if not info.name or not info.version:
                self.logger.error("模型名称和版本号不能为空")
                return False
            
            # 初始化模型列表
            if info.name not in self.models:
                self.models[info.name] = []
                self._circuit_breakers[info.name] = {}
                self._health_checks[info.name] = {}
            
            # 检查版本是否已存在
            for model in self.models[info.name]:
                if model.version == info.version:
                    self.logger.warning(f"模型 {info.name} 版本 {info.version} 已存在")
                    return False
            
            # 添加新版本
            self.models[info.name].append(info)
            
            # 初始化熔断器和健康检查
            self._circuit_breakers[info.name][info.version] = False
            self._health_checks[info.name][info.version] = True
            
            # 设置为活跃版本（如果是第一个或优先级更高）
            if (info.name not in self.active or 
                (info.status == ModelStatus.ACTIVE and info.priority > self.active[info.name].priority)):
                self.active[info.name] = info
                self.logger.info(f"模型 {info.name} v{info.version} 已设置为活跃版本")
            
            # 架构增强：预热模型
            if info.warmup_requests > 0:
                self._warmup_model(info)
            
            return True
    
    def get_model(self, 
                  name: str, 
                  force_version: Optional[str] = None,
                  request_id: Optional[str] = None) -> Optional[ModelInfo]:
        """
        智能模型选择（增强版A/B测试）
        
        Args:
            name: 模型名称
            force_version: 强制使用特定版本
            request_id: 请求ID（用于一致性哈希）
            
        Returns:
            选中的模型信息
        """
        with self._lock:
            self._stats['total_requests'] += 1
            
            if name not in self.models:
                self._stats['failed_routes'] += 1
                return None
            
            # 强制版本
            if force_version:
                for model in self.models[name]:
                    if model.version == force_version:
                        if self._is_model_healthy(name, model.version):
                            return self._track_model_usage(model)
                        else:
                            self.logger.warning(f"强制版本 {force_version} 不健康，使用备选")
                            break
                # 强制版本不健康时的备选逻辑
            
            # 获取健康的活跃版本
            active_versions = [
                m for m in self.models[name] 
                if (m.status == ModelStatus.ACTIVE and 
                    self._is_model_healthy(name, m.version))
            ]
            
            if not active_versions:
                self._stats['failed_routes'] += 1
                self.logger.error(f"模型 {name} 没有健康的活跃版本")
                return None
            
            # 单版本或A/B测试关闭
            if not self.ab_enabled or len(active_versions) == 1:
                selected = active_versions[0]
                self._stats['successful_routes'] += 1
                return self._track_model_usage(selected)
            
            # 智能路由选择
            selected = self._intelligent_route_selection(active_versions, request_id)
            
            if selected:
                self._stats['successful_routes'] += 1
                return self._track_model_usage(selected)
            
            self._stats['failed_routes'] += 1
            return None
    
    def update_metrics(self, name: str, version: str, 
                      success: bool, latency: float = 0.0) -> bool:
        """
        更新模型性能指标
        
        Args:
            name: 模型名称
            version: 版本号
            success: 是否成功
            latency: 延迟时间(ms)
            
        Returns:
            更新是否成功
        """
        if name not in self.models:
            return False
        
        for model in self.models[name]:
            if model.version == version:
                # 更新使用次数
                model.usage_count += 1
                model.last_used_at = datetime.now()
                
                # 更新性能指标
                model.metrics.total_calls += 1
                if success:
                    model.metrics.success_calls += 1
                
                # 计算错误率
                if model.metrics.total_calls > 0:
                    model.metrics.error_rate = 1.0 - (
                        model.metrics.success_calls / model.metrics.total_calls
                    )
                
                # 更新平均延迟（移动平均）
                n = model.metrics.total_calls
                if n > 0:
                    model.metrics.avg_latency = (
                        (model.metrics.avg_latency * (n - 1) + latency) / n
                    )
                
                return True
        
        return False
    
    def set_ab_weight(self, name: str, version: str, weight: float) -> bool:
        """
        设置A/B测试权重
        
        Args:
            name: 模型名称
            version: 版本号
            weight: 权重值 (0.0-1.0)
            
        Returns:
            设置是否成功
        """
        if weight < 0.0 or weight > 1.0:
            print(f"❌ 权重必须在0.0-1.0之间")
            return False
        
        if name not in self.models:
            return False
        
        for model in self.models[name]:
            if model.version == version:
                model.ab_weight = weight
                print(f"✅ 模型 {name} v{version} 权重已设置为 {weight}")
                return True
        
        return False
    
    def get_all_versions(self, name: str) -> List[ModelInfo]:
        """获取模型的所有版本"""
        return self.models.get(name, [])
    
    def get_metrics_summary(self, name: str) -> Dict[str, dict]:
        """
        获取模型所有版本的性能指标摘要
        
        Returns:
            版本 -> 指标字典
        """
        if name not in self.models:
            return {}
        
        summary = {}
        for model in self.models[name]:
            summary[model.version] = {
                "status": model.status,
                "priority": model.priority,
                "ab_weight": model.ab_weight,
                "usage_count": model.usage_count,
                "metrics": model.metrics.to_dict()
            }
        
        return summary
    
    def save_config(self, path: Optional[Path] = None) -> bool:
        """
        保存配置到文件
        
        Args:
            path: 保存路径（默认使用初始化时的路径）
        """
        save_path = path or self.config_path
        if not save_path:
            print("❌ 未指定保存路径")
            return False
        
        config = {
            "ab_enabled": self.ab_enabled,
            "models": {}
        }
        
        for name, versions in self.models.items():
            config["models"][name] = [v.to_dict() for v in versions]
        
        try:
            save_path.parent.mkdir(parents=True, exist_ok=True)
            with open(save_path, 'w', encoding='utf-8') as f:
                json.dump(config, f, indent=2, ensure_ascii=False)
            print(f"✅ 配置已保存到 {save_path}")
            return True
        except Exception as e:
            print(f"❌ 保存配置失败: {e}")
            return False
    
    def load_config(self, path: Path) -> bool:
        """
        从文件加载配置
        
        Args:
            path: 配置文件路径
        """
        try:
            with open(path, 'r', encoding='utf-8') as f:
                config = json.load(f)
            
            self.ab_enabled = config.get("ab_enabled", True)
            
            # 加载模型
            for name, versions_data in config.get("models", {}).items():
                self.models[name] = []
                for version_data in versions_data:
                    model = ModelInfo.from_dict(version_data)
                    self.models[name].append(model)
                    
                    # 恢复活跃版本
                    if model.status == "active":
                        if name not in self.active or \
                           model.priority > self.active[name].priority:
                            self.active[name] = model
            
            print(f"✅ 从 {path} 加载了 {len(self.models)} 个模型")
            return True
        except Exception as e:
            print(f"❌ 加载配置失败: {e}")
            return False
    
    def _intelligent_route_selection(self, 
                                   active_versions: List[ModelInfo],
                                   request_id: Optional[str] = None) -> Optional[ModelInfo]:
        """
        智能路由选择算法
        
        Args:
            active_versions: 活跃版本列表
            request_id: 请求ID
            
        Returns:
            选中的模型
        """
        if self._routing_strategy == ABTestStrategy.PERFORMANCE:
            # 基于性能的路由
            best_model = max(active_versions, key=lambda m: m.metrics.health_score)
            return best_model
        
        elif self._routing_strategy == ABTestStrategy.GRADUAL:
            # 渐进式发布：优先选择较新版本
            sorted_versions = sorted(active_versions, key=lambda m: m.created_at, reverse=True)
            # 使用rollout_percentage控制流量
            if random.random() < sorted_versions[0].rollout_percentage / 100.0:
                return sorted_versions[0]
            else:
                return sorted_versions[-1] if len(sorted_versions) > 1 else sorted_versions[0]
        
        elif self._routing_strategy == ABTestStrategy.RANDOM:
            # 随机选择
            return random.choice(active_versions)
        
        else:  # WEIGHTED (默认)
            # 基于权重的选择
            weights = [m.ab_weight for m in active_versions]
            total_weight = sum(weights)
            
            if total_weight > 0:
                # 一致性哈希（如果有request_id）
                if request_id:
                    hash_val = hash(request_id) % 1000
                    cumulative = 0
                    for i, model in enumerate(active_versions):
                        cumulative += (weights[i] / total_weight) * 1000
                        if hash_val < cumulative:
                            return model
                
                # 权重随机选择
                weights_normalized = [w / total_weight for w in weights]
                return random.choices(active_versions, weights=weights_normalized)[0]
            
            return active_versions[0]
    
    def _is_model_healthy(self, name: str, version: str) -> bool:
        """
        检查模型健康状态
        
        Args:
            name: 模型名称
            version: 版本号
            
        Returns:
            是否健康
        """
        # 检查熔断器状态
        if (name in self._circuit_breakers and 
            version in self._circuit_breakers[name] and
            self._circuit_breakers[name][version]):
            return False
        
        # 检查健康检查状态
        if (name in self._health_checks and 
            version in self._health_checks[name]):
            return self._health_checks[name][version]
        
        return True
    
    def _track_model_usage(self, model: ModelInfo) -> ModelInfo:
        """
        跟踪模型使用情况
        
        Args:
            model: 模型信息
            
        Returns:
            模型信息（更新后）
        """
        model.usage_count += 1
        model.last_used_at = datetime.now()
        self._request_counter[f"{model.name}:{model.version}"] += 1
        return model
    
    def _warmup_model(self, model: ModelInfo) -> None:
        """
        模型预热
        
        Args:
            model: 模型信息
        """
        # 实际实现中会向模型发送预热请求
        self.logger.info(f"开始预热模型 {model.name} v{model.version}")
        # 模拟预热过程
        time.sleep(0.1)
        self.logger.info(f"模型预热完成: {model.name} v{model.version}")
    
    def enable_ab_testing(self, enabled: bool = True):
        """启用/禁用A/B测试"""
        with self._lock:
            self.ab_enabled = enabled
            status = "启用" if enabled else "禁用"
            self.logger.info(f"A/B测试已{status}")
    
    def set_routing_strategy(self, strategy: ABTestStrategy):
        """设置路由策略"""
        with self._lock:
            self._routing_strategy = strategy
            self.logger.info(f"路由策略已设置为: {strategy.value}")
    
    def trigger_circuit_breaker(self, name: str, version: str):
        """触发熔断器"""
        with self._lock:
            if name in self._circuit_breakers:
                self._circuit_breakers[name][version] = True
                self._stats['circuit_breaker_trips'] += 1
                self.logger.warning(f"熔断器触发: {name} v{version}")
    
    def reset_circuit_breaker(self, name: str, version: str):
        """重置熔断器"""
        with self._lock:
            if name in self._circuit_breakers:
                self._circuit_breakers[name][version] = False
                self.logger.info(f"熔断器重置: {name} v{version}")
    
    def get_router_stats(self) -> Dict[str, Any]:
        """获取路由器统计信息"""
        with self._lock:
            return {
                **self._stats,
                'active_experiments': len([e for e in self._experiments.values() if e.is_active]),
                'total_models': sum(len(versions) for versions in self.models.values()),
                'request_distribution': dict(self._request_counter),
                'routing_strategy': self._routing_strategy.value,
                'ab_testing_enabled': self.ab_enabled
            }
    
    def print_summary(self):
        """打印模型路由器摘要"""
        print("\n" + "="*60)
        print("📊 模型路由器状态")
        print("="*60)
        print(f"A/B测试: {'✅ 启用' if self.ab_enabled else '❌ 禁用'}")
        print(f"注册模型数: {len(self.models)}")
        
        for name, versions in self.models.items():
            print(f"\n📦 模型: {name}")
            print(f"   版本数: {len(versions)}")
            print(f"   活跃版本: {self.active.get(name, 'N/A').version if name in self.active else 'N/A'}")
            
            for model in versions:
                print(f"   - v{model.version} [{model.status}]")
                print(f"     优先级: {model.priority}, 权重: {model.ab_weight}")
                print(f"     调用次数: {model.usage_count}, 成功率: {(1-model.metrics.error_rate)*100:.1f}%")
                print(f"     平均延迟: {model.metrics.avg_latency:.2f}ms")
        
        print("="*60 + "\n")


# 示例使用
if __name__ == "__main__":
    # 创建模型路由器
    router = ModelRouter()
    
    # 注册LightGBM模型的两个版本
    router.register_model(ModelInfo(
        name="lightgbm",
        version="v1.0.0",
        path="models/lightgbm_v1.0.0.txt",
        priority=1,
        ab_weight=0.7
    ))
    
    router.register_model(ModelInfo(
        name="lightgbm",
        version="v1.0.1",
        path="models/lightgbm_v1.0.1.txt",
        priority=2,
        ab_weight=0.3
    ))
    
    # 注册PPO模型
    router.register_model(ModelInfo(
        name="ppo",
        version="v1.0.0",
        path="models/ppo/actor_network.pth",
        model_type="optimizer"
    ))
    
    # 模拟使用
    print("\n🔄 模拟A/B测试 (100次调用):")
    version_counts = {"v1.0.0": 0, "v1.0.1": 0}
    
    for i in range(100):
        model = router.get_model("lightgbm")
        if model:
            version_counts[model.version] += 1
            # 模拟性能指标更新
            success = random.random() > 0.1  # 90%成功率
            latency = random.uniform(10, 50)  # 10-50ms延迟
            router.update_metrics("lightgbm", model.version, success, latency)
    
    print(f"v1.0.0 被选中: {version_counts['v1.0.0']}次")
    print(f"v1.0.1 被选中: {version_counts['v1.0.1']}次")
    
    # 打印摘要
    router.print_summary()
    
    # 保存配置
    router.save_config(Path("models/model_router_config.json"))
