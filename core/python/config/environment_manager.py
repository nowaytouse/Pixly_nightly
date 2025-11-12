"""
环境管理器 - 多环境配置和运行时环境管理

核心功能:
- 多环境配置管理 (dev/staging/prod)
- 环境变量自动加载和覆盖
- 环境特定的配置文件管理
- 运行时环境检测和切换
- 环境安全策略和权限控制

架构增强: 基于Go原版配置模式 + 现代环境管理
遵循四高原则：高规范化、高兼容性、高扩展性、高稳定性
"""

import os
import json
from typing import Dict, List, Optional, Any, Set
from dataclasses import dataclass, field
from datetime import datetime
from pathlib import Path
import logging
from enum import Enum
import re


class Environment(Enum):
    """环境类型枚举"""
    DEVELOPMENT = "development"
    TESTING = "testing" 
    STAGING = "staging"
    PRODUCTION = "production"
    LOCAL = "local"


class EnvironmentTier(Enum):
    """环境等级"""
    DEV = "dev"        # 开发环境
    TEST = "test"      # 测试环境
    STAGING = "staging"    # 预发布环境
    PROD = "production"    # 生产环境


@dataclass
class EnvironmentConfig:
    """环境配置定义"""
    name: str
    environment: Environment
    tier: EnvironmentTier
    description: str = ""
    
    # 配置文件路径
    config_files: List[str] = field(default_factory=list)
    
    # 环境变量前缀
    env_prefix: str = "PIXLY_"
    
    # 安全设置
    debug_enabled: bool = True
    logging_level: str = "INFO"
    sensitive_data_allowed: bool = True
    
    # 服务配置
    services: Dict[str, Dict[str, Any]] = field(default_factory=dict)
    
    # 依赖和约束
    required_env_vars: Set[str] = field(default_factory=set)
    forbidden_env_vars: Set[str] = field(default_factory=set)
    
    # 元数据
    created_at: datetime = field(default_factory=datetime.now)
    last_activated: Optional[datetime] = None
    is_default: bool = False


class EnvironmentManager:
    """
    环境管理器 - 企业级多环境配置管理
    
    高规范化、高兼容性、高扩展性、高稳定性实现
    """
    
    def __init__(self, 
                 config_dir: str = "config",
                 default_environment: Environment = Environment.DEVELOPMENT,
                 auto_detect: bool = True,
                 debug: bool = False):
        """
        初始化环境管理器
        
        Args:
            config_dir: 配置目录
            default_environment: 默认环境
            auto_detect: 自动检测环境
            debug: 调试模式
        """
        self.config_dir = Path(config_dir)
        self.default_environment = default_environment
        self.auto_detect = auto_detect
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        if debug:
            self.logger.setLevel(logging.DEBUG)
        
        # 环境配置
        self.environments: Dict[str, EnvironmentConfig] = {}
        self.current_environment: Optional[EnvironmentConfig] = None
        
        # 环境变量缓存
        self._env_cache: Dict[str, str] = {}
        self._original_env: Dict[str, str] = dict(os.environ)
        
        # 初始化预定义环境
        self._initialize_predefined_environments()
        
        # 自动检测当前环境
        if auto_detect:
            self._detect_current_environment()
        
        if debug:
            self.logger.debug(f"环境管理器初始化完成，当前环境: {self.get_current_environment_name()}")
    
    def _initialize_predefined_environments(self):
        """初始化预定义环境"""
        # 开发环境
        dev_config = EnvironmentConfig(
            name="development",
            environment=Environment.DEVELOPMENT,
            tier=EnvironmentTier.DEV,
            description="开发环境",
            config_files=["models_development.json", "http_development.json"],
            debug_enabled=True,
            logging_level="DEBUG",
            services={
                "http": {"port": 8080, "host": "localhost"},
                "database": {"path": "dev_data.db", "pool_size": 5}
            },
            required_env_vars={"PIXLY_ENV"},
            is_default=True
        )
        self.environments["development"] = dev_config
        
        # 测试环境
        test_config = EnvironmentConfig(
            name="testing",
            environment=Environment.TESTING,
            tier=EnvironmentTier.TEST,
            description="测试环境",
            config_files=["models_testing.json", "http_testing.json"],
            debug_enabled=True,
            logging_level="INFO",
            services={
                "http": {"port": 8081, "host": "localhost"},
                "database": {"path": "test_data.db", "pool_size": 3}
            },
            required_env_vars={"PIXLY_ENV", "PIXLY_TEST_TOKEN"}
        )
        self.environments["testing"] = test_config
        
        # 预发布环境
        staging_config = EnvironmentConfig(
            name="staging",
            environment=Environment.STAGING,
            tier=EnvironmentTier.STAGING,
            description="预发布环境",
            config_files=["models_staging.json", "http_staging.json"],
            debug_enabled=False,
            logging_level="INFO",
            services={
                "http": {"port": 8080, "host": "0.0.0.0"},
                "database": {"path": "staging_data.db", "pool_size": 10}
            },
            required_env_vars={"PIXLY_ENV", "PIXLY_API_KEY"},
            forbidden_env_vars={"DEBUG", "PIXLY_DEBUG"}
        )
        self.environments["staging"] = staging_config
        
        # 生产环境
        prod_config = EnvironmentConfig(
            name="production",
            environment=Environment.PRODUCTION,
            tier=EnvironmentTier.PROD,
            description="生产环境",
            config_files=["models_production.json", "http_production.json"],
            debug_enabled=False,
            logging_level="WARNING",
            sensitive_data_allowed=False,
            services={
                "http": {"port": 80, "host": "0.0.0.0"},
                "database": {"path": "/data/production.db", "pool_size": 20}
            },
            required_env_vars={"PIXLY_ENV", "PIXLY_API_KEY", "PIXLY_SECRET_KEY"},
            forbidden_env_vars={"DEBUG", "PIXLY_DEBUG", "PIXLY_TEST_MODE"}
        )
        self.environments["production"] = prod_config
    
    def _detect_current_environment(self):
        """自动检测当前环境"""
        # 从环境变量检测
        env_name = os.getenv("PIXLY_ENV", os.getenv("ENV", "")).lower()
        if env_name in self.environments:
            self.activate_environment(env_name)
            return
        
        # 从部署指标检测
        if os.getenv("KUBERNETES_SERVICE_HOST"):
            # Kubernetes环境
            if "prod" in os.getenv("HOSTNAME", ""):
                self.activate_environment("production")
            elif "staging" in os.getenv("HOSTNAME", ""):
                self.activate_environment("staging")
            else:
                self.activate_environment("testing")
            return
        
        # 从文件系统检测
        if Path("/etc/production-marker").exists():
            self.activate_environment("production")
            return
        
        if Path("/etc/staging-marker").exists():
            self.activate_environment("staging")
            return
        
        # 默认开发环境
        self.activate_environment("development")
    
    def activate_environment(self, environment_name: str) -> bool:
        """
        激活指定环境
        
        Args:
            environment_name: 环境名称
            
        Returns:
            是否激活成功
        """
        if environment_name not in self.environments:
            self.logger.error(f"环境不存在: {environment_name}")
            return False
        
        env_config = self.environments[environment_name]
        
        # 验证环境要求
        if not self._validate_environment_requirements(env_config):
            self.logger.error(f"环境要求验证失败: {environment_name}")
            return False
        
        # 激活环境
        self.current_environment = env_config
        env_config.last_activated = datetime.now()
        
        # 设置环境变量
        self._apply_environment_variables(env_config)
        
        # 加载环境配置文件
        self._load_environment_configs(env_config)
        
        self.logger.info(f"环境已激活: {environment_name}")
        return True
    
    def _validate_environment_requirements(self, env_config: EnvironmentConfig) -> bool:
        """验证环境要求"""
        # 检查必需的环境变量
        for required_var in env_config.required_env_vars:
            if not os.getenv(required_var):
                self.logger.error(f"缺少必需的环境变量: {required_var}")
                return False
        
        # 检查禁止的环境变量
        for forbidden_var in env_config.forbidden_env_vars:
            if os.getenv(forbidden_var):
                self.logger.error(f"存在禁止的环境变量: {forbidden_var}")
                return False
        
        return True
    
    def _apply_environment_variables(self, env_config: EnvironmentConfig):
        """应用环境变量"""
        # 设置基础环境变量
        os.environ["PIXLY_ENV"] = env_config.name
        os.environ["PIXLY_ENVIRONMENT"] = env_config.environment.value
        os.environ["PIXLY_TIER"] = env_config.tier.value
        
        # 设置调试模式
        if env_config.debug_enabled:
            os.environ["PIXLY_DEBUG"] = "1"
        else:
            os.environ.pop("PIXLY_DEBUG", None)
        
        # 设置日志级别
        os.environ["PIXLY_LOG_LEVEL"] = env_config.logging_level
        
        # 设置服务配置
        for service_name, service_config in env_config.services.items():
            for key, value in service_config.items():
                env_var_name = f"{env_config.env_prefix}{service_name.upper()}_{key.upper()}"
                os.environ[env_var_name] = str(value)
    
    def _load_environment_configs(self, env_config: EnvironmentConfig):
        """加载环境特定的配置文件"""
        for config_file in env_config.config_files:
            config_path = self.config_dir / config_file
            if config_path.exists():
                try:
                    with open(config_path, 'r', encoding='utf-8') as f:
                        config_data = json.load(f)
                    
                    # 将配置数据设置为环境变量
                    self._set_config_env_vars(config_data, env_config.env_prefix)
                    
                    if self.debug:
                        self.logger.debug(f"配置文件已加载: {config_file}")
                
                except Exception as e:
                    self.logger.error(f"配置文件加载失败: {config_file}, {e}")
    
    def _set_config_env_vars(self, config_data: Dict[str, Any], prefix: str):
        """将配置数据设置为环境变量"""
        def _flatten_dict(d: Dict[str, Any], parent_key: str = '') -> Dict[str, Any]:
            items = []
            for k, v in d.items():
                new_key = f"{parent_key}_{k.upper()}" if parent_key else k.upper()
                if isinstance(v, dict):
                    items.extend(_flatten_dict(v, new_key).items())
                else:
                    items.append((new_key, str(v)))
            return dict(items)
        
        flattened = _flatten_dict(config_data)
        for key, value in flattened.items():
            env_var_name = f"{prefix}{key}"
            os.environ[env_var_name] = value
    
    def get_current_environment_name(self) -> str:
        """获取当前环境名称"""
        if self.current_environment:
            return self.current_environment.name
        return "unknown"
    
    def get_environment_info(self, environment_name: str = None) -> Optional[Dict[str, Any]]:
        """获取环境信息"""
        if environment_name is None:
            env_config = self.current_environment
        else:
            env_config = self.environments.get(environment_name)
        
        if not env_config:
            return None
        
        return {
            "name": env_config.name,
            "environment": env_config.environment.value,
            "tier": env_config.tier.value,
            "description": env_config.description,
            "debug_enabled": env_config.debug_enabled,
            "logging_level": env_config.logging_level,
            "services": env_config.services,
            "last_activated": env_config.last_activated.isoformat() if env_config.last_activated else None,
            "is_default": env_config.is_default
        }
    
    def list_environments(self) -> Dict[str, Dict[str, Any]]:
        """列出所有环境"""
        return {name: self.get_environment_info(name) for name in self.environments}
    
    def is_production(self) -> bool:
        """是否为生产环境"""
        return (self.current_environment and 
                self.current_environment.environment == Environment.PRODUCTION)
    
    def is_development(self) -> bool:
        """是否为开发环境"""
        return (self.current_environment and 
                self.current_environment.environment == Environment.DEVELOPMENT)
    
    def is_testing(self) -> bool:
        """是否为测试环境"""
        return (self.current_environment and 
                self.current_environment.environment == Environment.TESTING)
    
    def get_env_var(self, key: str, default: str = None) -> Optional[str]:
        """获取环境变量（带前缀处理）"""
        # 尝试带前缀的变量
        if self.current_environment:
            prefixed_key = f"{self.current_environment.env_prefix}{key}"
            value = os.getenv(prefixed_key)
            if value is not None:
                return value
        
        # 尝试原始变量名
        return os.getenv(key, default)
    
    def set_env_var(self, key: str, value: str, use_prefix: bool = True):
        """设置环境变量"""
        if use_prefix and self.current_environment:
            key = f"{self.current_environment.env_prefix}{key}"
        
        os.environ[key] = value
        self._env_cache[key] = value
    
    def get_service_config(self, service_name: str) -> Dict[str, Any]:
        """获取服务配置"""
        if self.current_environment and service_name in self.current_environment.services:
            return self.current_environment.services[service_name].copy()
        return {}
    
    def validate_current_environment(self) -> Dict[str, Any]:
        """验证当前环境配置"""
        if not self.current_environment:
            return {"valid": False, "errors": ["未激活任何环境"]}
        
        errors = []
        warnings = []
        
        # 检查必需环境变量
        for required_var in self.current_environment.required_env_vars:
            if not os.getenv(required_var):
                errors.append(f"缺少必需的环境变量: {required_var}")
        
        # 检查配置文件
        missing_configs = []
        for config_file in self.current_environment.config_files:
            config_path = self.config_dir / config_file
            if not config_path.exists():
                missing_configs.append(config_file)
        
        if missing_configs:
            warnings.append(f"缺少配置文件: {', '.join(missing_configs)}")
        
        # 安全检查
        if self.current_environment.tier == EnvironmentTier.PROD:
            if self.current_environment.debug_enabled:
                warnings.append("生产环境启用了调试模式")
            if os.getenv("PIXLY_DEBUG"):
                errors.append("生产环境存在调试标志")
        
        return {
            "valid": len(errors) == 0,
            "errors": errors,
            "warnings": warnings,
            "environment": self.current_environment.name
        }
    
    def create_environment_snapshot(self) -> Dict[str, Any]:
        """创建当前环境快照"""
        if not self.current_environment:
            return {}
        
        # 收集环境变量
        env_vars = {}
        prefix = self.current_environment.env_prefix
        for key, value in os.environ.items():
            if key.startswith(prefix) or key in ["PATH", "HOME", "USER"]:
                env_vars[key] = value
        
        return {
            "environment": self.get_environment_info(),
            "env_vars": env_vars,
            "snapshot_time": datetime.now().isoformat()
        }
    
    def restore_original_environment(self):
        """恢复原始环境变量"""
        # 清除当前环境变量
        keys_to_remove = []
        for key in os.environ:
            if self.current_environment and key.startswith(self.current_environment.env_prefix):
                keys_to_remove.append(key)
        
        for key in keys_to_remove:
            del os.environ[key]
        
        # 恢复原始环境变量
        os.environ.clear()
        os.environ.update(self._original_env)
        
        self.current_environment = None
        self._env_cache.clear()
        
        self.logger.info("环境已重置为原始状态")


# 全局环境管理器实例
_global_env_manager: Optional[EnvironmentManager] = None


def get_environment_manager() -> EnvironmentManager:
    """获取全局环境管理器实例"""
    global _global_env_manager
    if _global_env_manager is None:
        _global_env_manager = EnvironmentManager()
    return _global_env_manager


def get_current_environment() -> str:
    """获取当前环境名称"""
    return get_environment_manager().get_current_environment_name()


def is_production() -> bool:
    """是否为生产环境"""
    return get_environment_manager().is_production()


def is_development() -> bool:
    """是否为开发环境"""
    return get_environment_manager().is_development()


# 便捷函数
def create_environment_manager(config_dir: str = "config", 
                             default_environment: Environment = Environment.DEVELOPMENT,
                             debug: bool = False) -> EnvironmentManager:
    """创建环境管理器的便捷函数"""
    return EnvironmentManager(config_dir, default_environment, debug=debug)


if __name__ == "__main__":
    # 测试代码
    print("=== 环境管理器测试 ===")
    
    env_mgr = create_environment_manager(debug=True)
    
    # 测试环境信息
    current_env = env_mgr.get_current_environment_name()
    print(f"✅ 当前环境: {current_env}")
    
    # 测试环境列表
    environments = env_mgr.list_environments()
    print(f"📋 可用环境: {list(environments.keys())}")
    
    # 测试环境切换
    success = env_mgr.activate_environment("testing")
    print(f"🔄 切换到测试环境: {success}")
    
    # 测试环境验证
    validation = env_mgr.validate_current_environment()
    print(f"✅ 环境验证: {validation['valid']}")
    
    # 测试服务配置
    http_config = env_mgr.get_service_config("http")
    print(f"⚙️ HTTP服务配置: {http_config}")
    
    print("🎯 环境管理器测试完成！")
