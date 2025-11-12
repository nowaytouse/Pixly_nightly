"""
模型路由和A/B测试系统

Phase 47.21 (EX-001): 从废弃Go代码提取
源文件: core/@deprecated/go_ai_service_2025_11_11/ai 2/model_router.go

核心功能:
- 多模型版本共存管理
- A/B测试权重自动分配
- 性能指标自动跟踪
- 智能模型选择算法

作者: Pixly Team
日期: 2025-11-12
"""

from typing import Dict, List, Optional
from dataclasses import dataclass, field
from datetime import datetime
import random
import json
from pathlib import Path


@dataclass
class ModelMetrics:
    """模型性能指标"""
    accuracy: float = 0.0           # 准确率
    precision: float = 0.0          # 精确率
    recall: float = 0.0             # 召回率
    f1_score: float = 0.0           # F1分数
    avg_latency: float = 0.0        # 平均延迟(ms)
    error_rate: float = 0.0         # 错误率
    total_calls: int = 0            # 总调用次数
    success_calls: int = 0          # 成功调用次数
    
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
class ModelInfo:
    """模型信息"""
    name: str                       # 模型名称 (lightgbm, ppo, etc)
    version: str                    # 版本号 (v1.0.0, v1.0.1, etc)
    path: str                       # 模型文件路径
    model_type: str = "predictor"   # 模型类型 (predictor, optimizer, validator)
    status: str = "active"          # 状态 (active, testing, deprecated)
    priority: int = 0               # 优先级 (越高越优先)
    ab_weight: float = 1.0          # A/B测试权重 (0.0-1.0)
    metrics: ModelMetrics = field(default_factory=ModelMetrics)
    created_at: datetime = field(default_factory=datetime.now)
    last_used_at: datetime = field(default_factory=datetime.now)
    usage_count: int = 0            # 使用次数
    
    def to_dict(self) -> dict:
        """转换为字典"""
        return {
            "name": self.name,
            "version": self.version,
            "path": self.path,
            "model_type": self.model_type,
            "status": self.status,
            "priority": self.priority,
            "ab_weight": self.ab_weight,
            "metrics": self.metrics.to_dict(),
            "created_at": self.created_at.isoformat(),
            "last_used_at": self.last_used_at.isoformat(),
            "usage_count": self.usage_count
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
    模型路由器 - 支持多版本A/B测试
    
    功能:
    - 注册和管理多个模型版本
    - 基于权重的A/B测试
    - 自动性能指标跟踪
    - 智能模型选择
    """
    
    def __init__(self, config_path: Optional[Path] = None):
        """
        初始化模型路由器
        
        Args:
            config_path: 配置文件路径（JSON格式）
        """
        self.models: Dict[str, List[ModelInfo]] = {}  # name -> versions
        self.active: Dict[str, ModelInfo] = {}        # name -> active version
        self.ab_enabled: bool = True                   # A/B测试开关
        self.config_path = config_path
        
        # 加载配置
        if config_path and config_path.exists():
            self.load_config(config_path)
    
    def register_model(self, info: ModelInfo) -> bool:
        """
        注册新模型版本
        
        Args:
            info: 模型信息
            
        Returns:
            注册是否成功
        """
        if not info.name or not info.version:
            print(f"❌ 模型名称和版本号不能为空")
            return False
        
        # 初始化模型列表
        if info.name not in self.models:
            self.models[info.name] = []
        
        # 检查版本是否已存在
        for model in self.models[info.name]:
            if model.version == info.version:
                print(f"⚠️ 模型 {info.name} 版本 {info.version} 已存在")
                return False
        
        # 添加新版本
        self.models[info.name].append(info)
        
        # 设置为活跃版本（如果是第一个或优先级更高）
        if info.name not in self.active or \
           (info.status == "active" and info.priority > self.active[info.name].priority):
            self.active[info.name] = info
            print(f"✅ 模型 {info.name} v{info.version} 已设置为活跃版本")
        
        return True
    
    def get_model(self, name: str, force_version: Optional[str] = None) -> Optional[ModelInfo]:
        """
        获取模型（支持A/B测试）
        
        Args:
            name: 模型名称
            force_version: 强制使用特定版本
            
        Returns:
            选中的模型信息
        """
        if name not in self.models:
            return None
        
        # 强制版本
        if force_version:
            for model in self.models[name]:
                if model.version == force_version:
                    return model
            return None
        
        # 获取所有活跃版本
        active_versions = [m for m in self.models[name] if m.status == "active"]
        if not active_versions:
            return None
        
        # 单版本或A/B测试关闭
        if not self.ab_enabled or len(active_versions) == 1:
            return active_versions[0]
        
        # A/B测试：按权重随机选择
        weights = [m.ab_weight for m in active_versions]
        total_weight = sum(weights)
        
        # 归一化权重
        if total_weight > 0:
            weights = [w / total_weight for w in weights]
            selected = random.choices(active_versions, weights=weights)[0]
            return selected
        
        return active_versions[0]
    
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
    
    def enable_ab_testing(self, enabled: bool = True):
        """启用/禁用A/B测试"""
        self.ab_enabled = enabled
        status = "启用" if enabled else "禁用"
        print(f"✅ A/B测试已{status}")
    
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
