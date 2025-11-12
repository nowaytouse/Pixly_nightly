"""
模型版本管理器
基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/model_manager.go 重新实现

功能:
- 多模型版本并存管理
- 模型性能指标跟踪（准确率、RMSE、MAE、推理时间）
- 模型激活/默认状态控制
- 文件校验和验证
- 支持模型热切换
- A/B测试权重管理

EX-011实现: 从Go废弃代码价值提取
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
        
        # 线程锁保护并发访问
        self._lock = threading.RLock()
        
        # 日志器
        self.logger = logging.getLogger(__name__)
        
        # 加载配置
        self._load_config()

    def add_model_version(self, model_type: str, version: str, 
                         model_path: str, description: str = "", 
                         metrics: Optional[ModelMetrics] = None) -> bool:
        """
        添加模型版本
        
        Args:
            model_type: 模型类型
            version: 版本号
            model_path: 模型文件路径
            description: 版本描述
            metrics: 性能指标
            
        Returns:
            bool: 是否添加成功
        """
        with self._lock:
            try:
                # 验证文件存在
                if not Path(model_path).exists():
                    self.logger.error(f"模型文件不存在: {model_path}")
                    return False
                
                # 创建模型版本对象
                model_version = ModelVersion(
                    version=version,
                    path=model_path,
                    description=description,
                    metrics=metrics
                )
                
                # 添加到存储
                if model_type not in self.model_versions:
                    self.model_versions[model_type] = {}
                
                self.model_versions[model_type][version] = model_version
                
                # 如果是第一个版本，设为默认和激活
                if len(self.model_versions[model_type]) == 1:
                    model_version.is_default = True
                    model_version.is_active = True
                    
                    # 创建默认配置
                    if model_type not in self.model_configs:
                        self.model_configs[model_type] = ModelConfig(
                            type=model_type,
                            version=version
                        )
                
                # 保存配置
                self._save_config()
                
                self.logger.info(f"添加模型版本成功: {model_type} v{version}")
                return True
                
            except Exception as e:
                self.logger.error(f"添加模型版本失败: {e}")
                return False

    def get_active_model(self, model_type: str) -> Optional[ModelVersion]:
        """
        获取激活的模型版本
        
        Args:
            model_type: 模型类型
            
        Returns:
            ModelVersion: 激活的模型版本，如果没有则返回None
        """
        with self._lock:
            if model_type not in self.model_versions:
                return None
            
            for version, model in self.model_versions[model_type].items():
                if model.is_active:
                    return model
            
            # 如果没有激活的，返回默认的
            for version, model in self.model_versions[model_type].items():
                if model.is_default:
                    return model
            
            return None

    def activate_model(self, model_type: str, version: str) -> bool:
        """
        激活指定模型版本
        
        Args:
            model_type: 模型类型
            version: 版本号
            
        Returns:
            bool: 是否激活成功
        """
        with self._lock:
            try:
                if (model_type not in self.model_versions or 
                    version not in self.model_versions[model_type]):
                    self.logger.error(f"模型版本不存在: {model_type} v{version}")
                    return False
                
                # 验证文件完整性
                target_model = self.model_versions[model_type][version]
                if not target_model.verify_checksum():
                    self.logger.error(f"模型文件校验失败: {model_type} v{version}")
                    return False
                
                # 取消其他版本的激活状态
                for v, model in self.model_versions[model_type].items():
                    model.is_active = (v == version)
                
                # 更新配置
                if model_type in self.model_configs:
                    self.model_configs[model_type].version = version
                else:
                    self.model_configs[model_type] = ModelConfig(
                        type=model_type,
                        version=version
                    )
                
                self._save_config()
                
                self.logger.info(f"激活模型版本: {model_type} v{version}")
                return True
                
            except Exception as e:
                self.logger.error(f"激活模型版本失败: {e}")
                return False

    def set_default_model(self, model_type: str, version: str) -> bool:
        """
        设置默认模型版本
        
        Args:
            model_type: 模型类型
            version: 版本号
            
        Returns:
            bool: 是否设置成功
        """
        with self._lock:
            try:
                if (model_type not in self.model_versions or 
                    version not in self.model_versions[model_type]):
                    return False
                
                # 取消其他版本的默认状态
                for v, model in self.model_versions[model_type].items():
                    model.is_default = (v == version)
                
                self._save_config()
                
                self.logger.info(f"设置默认模型版本: {model_type} v{version}")
                return True
                
            except Exception as e:
                self.logger.error(f"设置默认模型版本失败: {e}")
                return False

    def update_model_metrics(self, model_type: str, version: str, 
                           metrics: ModelMetrics) -> bool:
        """
        更新模型性能指标
        
        Args:
            model_type: 模型类型
            version: 版本号
            metrics: 新的性能指标
            
        Returns:
            bool: 是否更新成功
        """
        with self._lock:
            try:
                if (model_type not in self.model_versions or 
                    version not in self.model_versions[model_type]):
                    return False
                
                self.model_versions[model_type][version].metrics = metrics
                self._save_config()
                
                self.logger.info(f"更新模型指标: {model_type} v{version}")
                return True
                
            except Exception as e:
                self.logger.error(f"更新模型指标失败: {e}")
                return False

    def remove_model_version(self, model_type: str, version: str) -> bool:
        """
        移除模型版本
        
        Args:
            model_type: 模型类型
            version: 版本号
            
        Returns:
            bool: 是否移除成功
        """
        with self._lock:
            try:
                if (model_type not in self.model_versions or 
                    version not in self.model_versions[model_type]):
                    return False
                
                model = self.model_versions[model_type][version]
                
                # 不能移除激活或默认的模型
                if model.is_active or model.is_default:
                    self.logger.error(f"不能移除激活或默认模型: {model_type} v{version}")
                    return False
                
                # 移除模型版本
                del self.model_versions[model_type][version]
                
                # 如果该类型没有模型了，移除配置
                if not self.model_versions[model_type]:
                    del self.model_versions[model_type]
                    if model_type in self.model_configs:
                        del self.model_configs[model_type]
                
                self._save_config()
                
                self.logger.info(f"移除模型版本: {model_type} v{version}")
                return True
                
            except Exception as e:
                self.logger.error(f"移除模型版本失败: {e}")
                return False

    def list_model_versions(self, model_type: Optional[str] = None) -> Dict[str, List[Dict[str, Any]]]:
        """
        列出模型版本
        
        Args:
            model_type: 指定模型类型，None表示所有类型
            
        Returns:
            Dict: 模型版本信息
        """
        with self._lock:
            result = {}
            
            target_types = [model_type] if model_type else list(self.model_versions.keys())
            
            for mtype in target_types:
                if mtype in self.model_versions:
                    result[mtype] = []
                    for version, model in self.model_versions[mtype].items():
                        model_info = model.to_dict()
                        model_info['file_exists'] = Path(model.path).exists()
                        model_info['checksum_valid'] = model.verify_checksum()
                        result[mtype].append(model_info)
                    
                    # 按创建时间排序（新的在前）
                    result[mtype].sort(key=lambda x: x['created_at'], reverse=True)
            
            return result

    def get_model_statistics(self) -> Dict[str, Any]:
        """
        获取模型管理器统计信息
        
        Returns:
            Dict: 统计信息
        """
        with self._lock:
            stats = {
                'total_model_types': len(self.model_versions),
                'total_versions': sum(len(versions) for versions in self.model_versions.values()),
                'active_models': {},
                'model_types': []
            }
            
            for model_type, versions in self.model_versions.items():
                type_stats = {
                    'type': model_type,
                    'total_versions': len(versions),
                    'active_version': None,
                    'default_version': None
                }
                
                for version, model in versions.items():
                    if model.is_active:
                        type_stats['active_version'] = version
                        stats['active_models'][model_type] = version
                    if model.is_default:
                        type_stats['default_version'] = version
                
                stats['model_types'].append(type_stats)
            
            return stats

    def _load_config(self) -> None:
        """加载配置文件"""
        try:
            if not self.config_file.exists():
                return
            
            with open(self.config_file, 'r', encoding='utf-8') as f:
                data = json.load(f)
            
            # 加载模型版本
            for model_type, versions_data in data.get('model_versions', {}).items():
                self.model_versions[model_type] = {}
                
                for version, version_data in versions_data.items():
                    # 解析创建时间
                    created_at = datetime.fromisoformat(version_data['created_at'])
                    
                    # 解析性能指标
                    metrics = None
                    if 'metrics' in version_data and version_data['metrics']:
                        metrics = ModelMetrics(**version_data['metrics'])
                    
                    # 创建模型版本对象
                    model_version = ModelVersion(
                        version=version_data['version'],
                        path=version_data['path'],
                        checksum=version_data.get('checksum', ''),
                        created_at=created_at,
                        metrics=metrics,
                        is_active=version_data.get('is_active', False),
                        is_default=version_data.get('is_default', False),
                        description=version_data.get('description', '')
                    )
                    
                    self.model_versions[model_type][version] = model_version
            
            # 加载模型配置
            for model_type, config_data in data.get('model_configs', {}).items():
                self.model_configs[model_type] = ModelConfig(**config_data)
            
            self.logger.info(f"加载模型配置成功: {len(self.model_versions)}个模型类型")
            
        except Exception as e:
            self.logger.error(f"加载模型配置失败: {e}")

    def _save_config(self) -> None:
        """保存配置文件"""
        try:
            data = {
                'model_versions': {},
                'model_configs': {}
            }
            
            # 保存模型版本
            for model_type, versions in self.model_versions.items():
                data['model_versions'][model_type] = {}
                for version, model in versions.items():
                    data['model_versions'][model_type][version] = model.to_dict()
            
            # 保存模型配置
            for model_type, config in self.model_configs.items():
                data['model_configs'][model_type] = asdict(config)
            
            # 原子写入
            temp_file = self.config_file.with_suffix('.tmp')
            with open(temp_file, 'w', encoding='utf-8') as f:
                json.dump(data, f, indent=2, ensure_ascii=False)
            
            temp_file.replace(self.config_file)
            
        except Exception as e:
            self.logger.error(f"保存模型配置失败: {e}")


# 全局模型管理器实例
_global_model_manager: Optional[ModelManager] = None
_manager_lock = threading.Lock()

def get_model_manager(storage_dir: str = "models", 
                     config_file: str = "model_config.json") -> ModelManager:
    """
    获取全局模型管理器实例（单例模式）
    
    Args:
        storage_dir: 模型存储目录
        config_file: 配置文件路径
        
    Returns:
        ModelManager: 模型管理器实例
    """
    global _global_model_manager
    
    with _manager_lock:
        if _global_model_manager is None:
            _global_model_manager = ModelManager(storage_dir, config_file)
        return _global_model_manager


if __name__ == "__main__":
    # 测试代码
    print("=== 模型版本管理器测试 ===")
    
    # 创建测试目录
    import tempfile
    import os
    
    with tempfile.TemporaryDirectory() as temp_dir:
        # 创建模型管理器
        manager = ModelManager(
            storage_dir=os.path.join(temp_dir, "models"),
            config_file=os.path.join(temp_dir, "config.json")
        )
        
        # 创建测试模型文件
        test_model_path = os.path.join(temp_dir, "test_model.pkl")
        with open(test_model_path, 'w') as f:
            f.write("test model content")
        
        # 添加模型版本
        metrics = ModelMetrics(accuracy=0.95, rmse=0.1, mae=0.05, inference_time_ms=10.5)
        success = manager.add_model_version(
            model_type=ModelType.LIGHTGBM,
            version="1.0.0",
            model_path=test_model_path,
            description="初始版本",
            metrics=metrics
        )
        
        print(f"添加模型版本: {success}")
        
        # 获取激活模型
        active_model = manager.get_active_model(ModelType.LIGHTGBM)
        print(f"激活模型: {active_model.version if active_model else 'None'}")
        
        # 列出所有版本
        versions = manager.list_model_versions(ModelType.LIGHTGBM)
        print(f"模型版本列表: {len(versions.get(ModelType.LIGHTGBM, []))}个版本")
        
        # 获取统计信息
        stats = manager.get_model_statistics()
        print(f"统计信息: {stats['total_model_types']}个模型类型")
