"""
🧠 PIXLY v3.1 智能模型路由器

替代Go model_manager.go + model_router.go的完整功能：
- 本地AI模型管理和加载
- A/B测试权重路由系统
- 模型性能监控和评估  
- 智能模型比较分析
- 零网络依赖的模型治理
- LightGBM模型支持

基于废弃Go ModelManager完全重新设计
"""

import json
import hashlib
import time
import threading
from pathlib import Path
from typing import Dict, List, Optional, Any
from dataclasses import dataclass, field, asdict
from datetime import datetime
import logging

@dataclass
class ModelMetrics:
    """模型性能指标"""
    accuracy: float = 0.0          # 准确率
    rmse: float = 0.0              # 均方根误差
    mae: float = 0.0               # 平均绝对误差
    inference_time_ms: float = 0.0 # 推理时间（毫秒）
    test_samples: int = 0          # 测试样本数
    
    def to_dict(self) -> Dict[str, float]:
        """转换为字典"""
        return asdict(self)

@dataclass
class ModelVersion:
    """模型版本"""
    version: str                    # 版本号（如 "1.0.0"）
    path: str                      # 模型文件路径
    checksum: str = ""             # 文件校验和
    created_at: datetime = field(default_factory=datetime.now)  # 创建时间
    metrics: Optional[ModelMetrics] = None  # 性能指标
    is_active: bool = False        # 是否激活
    is_default: bool = False       # 是否默认
    description: str = ""          # 版本描述
    
    def __post_init__(self):
        """初始化后处理"""
        if self.metrics is None:
            self.metrics = ModelMetrics()
        
        # 计算文件校验和
        if not self.checksum and Path(self.path).exists():
            self.checksum = self._calculate_checksum()
    
    def _calculate_checksum(self) -> str:
        """计算文件MD5校验和"""
        try:
            with open(self.path, 'rb') as f:
                return hashlib.md5(f.read()).hexdigest()
        except Exception:
            return ""
    
    def verify_checksum(self) -> bool:
        """验证文件校验和"""
        if not self.checksum:
            return False
        
        current_checksum = self._calculate_checksum()
        return current_checksum == self.checksum
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典"""
        data = asdict(self)
        data['created_at'] = self.created_at.isoformat()
        if self.metrics:
            data['metrics'] = self.metrics.to_dict()
        return data

class ModelType:
    """模型类型常量"""
    LIGHTGBM = "lightgbm"         # LightGBM模型
    PPO = "ppo"                   # PPO强化学习模型
    TRANSFORMER = "transformer"   # Transformer模型
    ENSEMBLE = "ensemble"         # 集成模型
    BASELINE = "baseline"         # 基线规则引擎

@dataclass
class ModelConfig:
    """模型配置"""
    type: str                     # 模型类型
    version: str                  # 使用的版本
    weight: float = 1.0           # 集成权重（0-1）
    enabled: bool = True          # 是否启用
    priority: int = 1             # 优先级（越大越优先）

class ModelManager:
    """
    模型管理器
    支持多模型版本管理、热切换、A/B测试等功能
    """
    
    def __init__(self, storage_dir: str = "models", config_file: str = "model_config.json"):
        """
        初始化模型管理器
        
        Args:
            storage_dir: 模型存储目录
            config_file: 配置文件路径
        """
        self.storage_dir = Path(storage_dir)
        self.config_file = Path(config_file)
        self.storage_dir.mkdir(parents=True, exist_ok=True)
        
        # 模型版本存储：{model_type: {version: ModelVersion}}
        self.model_versions: Dict[str, Dict[str, ModelVersion]] = {}
        
        # 模型配置存储：{model_type: ModelConfig}
        self.model_configs: Dict[str, ModelConfig] = {}
        
        # A/B测试配置
        self.ab_test_config: Optional['LocalABTestConfig'] = None
        
        # 线程锁
        self._lock = threading.RLock()
        
        # 加载配置
        self._load_configurations()
        self._initialize_default_models()
    
    def _load_configurations(self):
        """加载模型配置"""
        if self.config_file.exists():
            try:
                with open(self.config_file, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                
                # 加载模型版本
                for model_type, versions in data.get('versions', {}).items():
                    self.model_versions[model_type] = {}
                    for version_id, version_data in versions.items():
                        version = ModelVersion(
                            version=version_data['version'],
                            path=version_data['path'],
                            checksum=version_data.get('checksum', ''),
                            is_active=version_data.get('is_active', False),
                            is_default=version_data.get('is_default', False),
                            description=version_data.get('description', '')
                        )
                        if 'metrics' in version_data:
                            metrics_data = version_data['metrics']
                            version.metrics = ModelMetrics(**metrics_data)
                        
                        self.model_versions[model_type][version_id] = version
                
                # 加载模型配置
                for model_type, config_data in data.get('configs', {}).items():
                    config = ModelConfig(
                        type=config_data['type'],
                        version=config_data['version'],
                        weight=config_data.get('weight', 1.0),
                        enabled=config_data.get('enabled', True),
                        priority=config_data.get('priority', 1)
                    )
                    self.model_configs[model_type] = config
                
                print(f"✅ 加载模型配置: {len(self.model_versions)} 个模型类型")
                
            except Exception as e:
                print(f"⚠️ 加载模型配置失败: {e}")
    
    def _initialize_default_models(self):
        """初始化默认模型配置"""
        # 确保基础模型配置存在
        default_configs = {
            ModelType.LIGHTGBM: ModelConfig(
                type=ModelType.LIGHTGBM,
                version="1.0.0",
                weight=0.6,
                enabled=True,
                priority=100
            ),
            ModelType.PPO: ModelConfig(
                type=ModelType.PPO,
                version="1.0.0", 
                weight=0.3,
                enabled=False,  # 默认禁用，待训练
                priority=80
            ),
            ModelType.BASELINE: ModelConfig(
                type=ModelType.BASELINE,
                version="1.0.0",
                weight=0.1,
                enabled=True,
                priority=50
            )
        }
        
        # 添加缺失的配置
        for model_type, default_config in default_configs.items():
            if model_type not in self.model_configs:
                self.model_configs[model_type] = default_config
        
        # 检查PPO模型文件
        self._check_ppo_models()
    
    def _check_ppo_models(self):
        """检查PPO模型文件并自动启用"""
        ppo_model_paths = [
            "models/ppo/actor_network.pth",
            "models/ppo/critic_network.pth"
        ]
        
        all_exist = all(Path(path).exists() for path in ppo_model_paths)
        
        if all_exist and ModelType.PPO in self.model_configs:
            # 启用PPO模型
            self.model_configs[ModelType.PPO].enabled = True
            print("✅ 检测到PPO模型文件，已自动启用")
            
            # 注册PPO模型版本
            if ModelType.PPO not in self.model_versions:
                self.model_versions[ModelType.PPO] = {}
            
            ppo_version = ModelVersion(
                version="1.0.0",
                path="models/ppo/",
                is_active=True,
                is_default=True,
                description="PPO强化学习模型"
            )
            
            self.model_versions[ModelType.PPO]["1.0.0"] = ppo_version
    
    def list_available_models(self) -> Dict[str, Any]:
        """列出所有可用模型"""
        with self._lock:
            models = {}
            
            for model_type, config in self.model_configs.items():
                model_info = {
                    "type": config.type,
                    "enabled": config.enabled,
                    "priority": config.priority,
                    "weight": config.weight,
                    "current_version": config.version,
                    "versions": []
                }
                
                # 添加版本信息
                if model_type in self.model_versions:
                    for version_id, version in self.model_versions[model_type].items():
                        version_info = {
                            "version": version.version,
                            "path": version.path,
                            "is_active": version.is_active,
                            "is_default": version.is_default,
                            "description": version.description,
                            "checksum_valid": version.verify_checksum() if version.checksum else None
                        }
                        
                        if version.metrics:
                            version_info["metrics"] = version.metrics.to_dict()
                        
                        model_info["versions"].append(version_info)
                
                models[model_type] = model_info
            
            return models
    
    def select_model(self, request_id: Optional[str] = None, 
                    model_type: Optional[str] = None) -> Optional[str]:
        """
        选择模型 (本地化A/B测试支持)
        
        Args:
            request_id: 请求ID (用于A/B测试分流)
            model_type: 指定模型类型
            
        Returns:
            选中的模型类型
        """
        with self._lock:
            # 如果指定了模型类型
            if model_type and model_type in self.model_configs:
                config = self.model_configs[model_type]
                if config.enabled:
                    return model_type
            
            # 本地化A/B测试
            if self.ab_test_config and self.ab_test_config.enabled and request_id:
                selected_model = self._ab_test_select(request_id)
                if selected_model:
                    return selected_model
            
            # 默认选择：按优先级选择最高的启用模型
            enabled_models = [
                (model_type, config) for model_type, config in self.model_configs.items()
                if config.enabled
            ]
            
            if not enabled_models:
                return ModelType.BASELINE  # 后备到基线模型
            
            # 按优先级排序
            enabled_models.sort(key=lambda x: x[1].priority, reverse=True)
            return enabled_models[0][0]
    
    def _ab_test_select(self, request_id: str) -> Optional[str]:
        """本地化A/B测试模型选择"""
        if not self.ab_test_config:
            return None
        
        # 简单哈希分流 (本地化实现)
        import hashlib
        hash_value = int(hashlib.md5(request_id.encode()).hexdigest()[:8], 16)
        split_point = hash_value % 100
        
        if split_point < (self.ab_test_config.split_ratio * 100):
            return self.ab_test_config.model_a
        else:
            return self.ab_test_config.model_b
    
    def register_model_version(self, model_type: str, version: ModelVersion) -> bool:
        """注册模型版本"""
        with self._lock:
            try:
                if model_type not in self.model_versions:
                    self.model_versions[model_type] = {}
                
                self.model_versions[model_type][version.version] = version
                
                # 如果这是第一个版本，设为默认
                if len(self.model_versions[model_type]) == 1:
                    version.is_default = True
                    version.is_active = True
                
                # 保存配置
                self._save_configurations()
                
                print(f"✅ 注册模型版本: {model_type} v{version.version}")
                return True
                
            except Exception as e:
                print(f"❌ 注册模型版本失败: {e}")
                return False
    
    def update_model_config(self, model_type: str, config: ModelConfig) -> bool:
        """更新模型配置"""
        with self._lock:
            try:
                self.model_configs[model_type] = config
                self._save_configurations()
                
                print(f"✅ 更新模型配置: {model_type}")
                return True
                
            except Exception as e:
                print(f"❌ 更新模型配置失败: {e}")
                return False
    
    def enable_model(self, model_type: str) -> bool:
        """启用模型"""
        with self._lock:
            if model_type in self.model_configs:
                self.model_configs[model_type].enabled = True
                self._save_configurations()
                print(f"✅ 启用模型: {model_type}")
                return True
            return False
    
    def disable_model(self, model_type: str) -> bool:
        """禁用模型"""
        with self._lock:
            if model_type in self.model_configs:
                self.model_configs[model_type].enabled = False
                self._save_configurations()
                print(f"✅ 禁用模型: {model_type}")
                return True
            return False
    
    def start_ab_test(self, model_a: str, model_b: str, 
                     split_ratio: float = 0.5, duration_hours: float = 24) -> bool:
        """启动本地化A/B测试"""
        with self._lock:
            try:
                # 验证模型存在
                if (model_a not in self.model_configs or 
                    model_b not in self.model_configs):
                    return False
                
                self.ab_test_config = LocalABTestConfig(
                    enabled=True,
                    split_ratio=split_ratio,
                    model_a=model_a,
                    model_b=model_b,
                    start_time=time.time(),
                    duration_hours=duration_hours
                )
                
                print(f"✅ 启动A/B测试: {model_a} vs {model_b} ({split_ratio*100:.1f}% / {(1-split_ratio)*100:.1f}%)")
                return True
                
            except Exception as e:
                print(f"❌ 启动A/B测试失败: {e}")
                return False
    
    def stop_ab_test(self) -> Optional[Dict[str, Any]]:
        """停止A/B测试并返回结果"""
        with self._lock:
            if self.ab_test_config and self.ab_test_config.enabled:
                self.ab_test_config.enabled = False
                
                # 返回测试结果
                result = {
                    "model_a": self.ab_test_config.model_a,
                    "model_b": self.ab_test_config.model_b,
                    "split_ratio": self.ab_test_config.split_ratio,
                    "duration_hours": self.ab_test_config.duration_hours,
                    "actual_duration_hours": (time.time() - self.ab_test_config.start_time) / 3600,
                    "results": self.ab_test_config.results
                }
                
                print(f"✅ 停止A/B测试")
                return result
            
            return None
    
    def record_model_performance(self, model_type: str, inference_time_ms: float,
                               accuracy: Optional[float] = None, success: bool = True):
        """记录模型性能"""
        with self._lock:
            # 更新A/B测试结果
            if (self.ab_test_config and self.ab_test_config.enabled and 
                model_type in [self.ab_test_config.model_a, self.ab_test_config.model_b]):
                
                if model_type not in self.ab_test_config.results:
                    self.ab_test_config.results[model_type] = LocalABTestResult(model_type)
                
                result = self.ab_test_config.results[model_type]
                result.request_count += 1
                result.total_latency_ms += inference_time_ms
                result.avg_latency_ms = result.total_latency_ms / result.request_count
                
                if accuracy is not None:
                    result.total_accuracy += accuracy
                    result.avg_accuracy = result.total_accuracy / result.request_count
                
                if not success:
                    result.error_count += 1
                
                result.error_rate = result.error_count / result.request_count
    
    def get_model_stats(self) -> Dict[str, Any]:
        """获取模型统计信息"""
        with self._lock:
            stats = {
                "total_models": len(self.model_configs),
                "enabled_models": len([c for c in self.model_configs.values() if c.enabled]),
                "model_details": {},
                "ab_test": None
            }
            
            # 模型详细信息
            for model_type, config in self.model_configs.items():
                stats["model_details"][model_type] = {
                    "enabled": config.enabled,
                    "priority": config.priority,
                    "weight": config.weight,
                    "version_count": len(self.model_versions.get(model_type, {}))
                }
            
            # A/B测试信息
            if self.ab_test_config and self.ab_test_config.enabled:
                stats["ab_test"] = {
                    "active": True,
                    "model_a": self.ab_test_config.model_a,
                    "model_b": self.ab_test_config.model_b,
                    "split_ratio": self.ab_test_config.split_ratio,
                    "running_hours": (time.time() - self.ab_test_config.start_time) / 3600,
                    "results": {
                        model_type: {
                            "request_count": result.request_count,
                            "avg_latency_ms": result.avg_latency_ms,
                            "avg_accuracy": result.avg_accuracy,
                            "error_rate": result.error_rate
                        }
                        for model_type, result in self.ab_test_config.results.items()
                    }
                }
            
            return stats
    
    def _save_configurations(self):
        """保存配置到文件"""
        try:
            data = {
                "versions": {},
                "configs": {}
            }
            
            # 保存模型版本
            for model_type, versions in self.model_versions.items():
                data["versions"][model_type] = {}
                for version_id, version in versions.items():
                    data["versions"][model_type][version_id] = version.to_dict()
            
            # 保存模型配置
            for model_type, config in self.model_configs.items():
                data["configs"][model_type] = asdict(config)
            
            # 写入文件
            with open(self.config_file, 'w', encoding='utf-8') as f:
                json.dump(data, f, indent=2)
                
        except Exception as e:
            print(f"⚠️ 保存模型配置失败: {e}")


@dataclass
class LocalABTestConfig:
    """本地化A/B测试配置"""
    enabled: bool = False
    split_ratio: float = 0.5  # A组占比
    model_a: str = ""
    model_b: str = ""
    start_time: float = 0.0
    duration_hours: float = 24.0
    results: Dict[str, 'LocalABTestResult'] = field(default_factory=dict)


@dataclass  
class LocalABTestResult:
    """本地化A/B测试结果"""
    model_type: str
    request_count: int = 0
    total_latency_ms: float = 0.0
    avg_latency_ms: float = 0.0
    total_accuracy: float = 0.0
    avg_accuracy: float = 0.0
    error_count: int = 0
    error_rate: float = 0.0


# 向后兼容的别名
LocalModelManager = ModelManager
