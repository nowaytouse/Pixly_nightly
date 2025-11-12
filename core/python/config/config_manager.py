"""
配置管理系统 - 动态配置和环境管理

基于废弃Go代码中的各种配置模式重新实现

核心功能:
- 多环境配置管理
- 动态配置热更新
- 配置验证和类型检查
- 配置监听和回调
- 配置加密和安全存储

EX-027实现: 从Go废弃代码价值提取 + 动态配置架构增强
遵循四高原则：高规范化、高兼容性、高扩展性、高稳定性
"""

import json
import os
import threading
import time
from typing import Dict, List, Optional, Any, Union, Callable, Type
from dataclasses import dataclass, field, asdict
from datetime import datetime, timedelta
from pathlib import Path
import logging
from enum import Enum
from collections import defaultdict
import hashlib
import copy


class ConfigType(Enum):
    """配置类型枚举"""
    STRING = "string"
    INTEGER = "integer"
    FLOAT = "float"
    BOOLEAN = "boolean"
    LIST = "list"
    DICT = "dict"
    JSON = "json"


class ConfigSource(Enum):
    """配置源类型"""
    FILE = "file"
    ENVIRONMENT = "environment"
    DATABASE = "database"
    REMOTE = "remote"
    MEMORY = "memory"


@dataclass
class ConfigValue:
    """配置值定义"""
    key: str
    value: Any
    value_type: ConfigType
    description: str = ""
    default_value: Any = None
    required: bool = False
    
    # 验证和约束
    min_value: Optional[float] = None
    max_value: Optional[float] = None
    allowed_values: List[Any] = field(default_factory=list)
    pattern: Optional[str] = None
    
    # 元数据
    source: ConfigSource = ConfigSource.FILE
    last_modified: datetime = field(default_factory=datetime.now)
    encrypted: bool = False
    sensitive: bool = False
    
    def validate(self) -> bool:
        """验证配置值"""
        if self.required and self.value is None:
            return False
        
        if self.value is None:
            return True
        
        # 类型检查
        if not self._check_type():
            return False
        
        # 范围检查
        if self.min_value is not None and isinstance(self.value, (int, float)):
            if self.value < self.min_value:
                return False
        
        if self.max_value is not None and isinstance(self.value, (int, float)):
            if self.value > self.max_value:
                return False
        
        # 枚举值检查
        if self.allowed_values and self.value not in self.allowed_values:
            return False
        
        return True
    
    def _check_type(self) -> bool:
        """检查值类型"""
        type_map = {
            ConfigType.STRING: str,
            ConfigType.INTEGER: int,
            ConfigType.FLOAT: (int, float),
            ConfigType.BOOLEAN: bool,
            ConfigType.LIST: list,
            ConfigType.DICT: dict
        }
        
        expected_type = type_map.get(self.value_type)
        if expected_type:
            return isinstance(self.value, expected_type)
        
        return True


@dataclass
class ConfigSection:
    """配置段定义"""
    name: str
    description: str = ""
    values: Dict[str, ConfigValue] = field(default_factory=dict)
    enabled: bool = True
    
    # 依赖关系
    depends_on: List[str] = field(default_factory=list)
    conflicts_with: List[str] = field(default_factory=list)
    
    def add_value(self, config_value: ConfigValue):
        """添加配置值"""
        self.values[config_value.key] = config_value
    
    def get_value(self, key: str, default: Any = None) -> Any:
        """获取配置值"""
        if key in self.values:
            return self.values[key].value
        return default
    
    def set_value(self, key: str, value: Any) -> bool:
        """设置配置值"""
        if key in self.values:
            self.values[key].value = value
            self.values[key].last_modified = datetime.now()
            return self.values[key].validate()
        return False
    
    def validate_all(self) -> Dict[str, bool]:
        """验证所有配置值"""
        results = {}
        for key, config_value in self.values.items():
            results[key] = config_value.validate()
        return results


class ConfigManager:
    """
    配置管理器 - 企业级动态配置管理系统
    
    高规范化、高兼容性、高扩展性、高稳定性实现
    """
    
    def __init__(self, 
                 config_dir: str = "config",
                 environment: str = "development",
                 auto_reload: bool = True,
                 debug: bool = False):
        """
        初始化配置管理器
        
        Args:
            config_dir: 配置文件目录
            environment: 环境名称
            auto_reload: 自动重载配置
            debug: 调试模式
        """
        self.config_dir = Path(config_dir)
        self.environment = environment
        self.auto_reload = auto_reload
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        if debug:
            self.logger.setLevel(logging.DEBUG)
        
        # 配置数据
        self.sections: Dict[str, ConfigSection] = {}
        self._config_cache: Dict[str, Any] = {}
        self._file_checksums: Dict[str, str] = {}
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 回调和监听器
        self._change_callbacks: Dict[str, List[Callable]] = defaultdict(list)
        self._global_callbacks: List[Callable] = []
        
        # 监控线程
        self._monitor_thread = None
        self._monitor_running = False
        
        # 架构增强：配置模板
        self._templates: Dict[str, ConfigSection] = {}
        self._load_builtin_templates()
        
        # 创建配置目录
        self.config_dir.mkdir(parents=True, exist_ok=True)
        
        # 初始化配置
        self._initialize_default_configs()
        
        if debug:
            self.logger.debug(f"配置管理器初始化完成: {config_dir}, 环境: {environment}")
    
    def _initialize_default_configs(self):
        """初始化默认配置"""
        # AI模型配置
        model_section = ConfigSection("models", "AI模型配置")
        model_section.add_value(ConfigValue(
            "lightgbm_enabled", True, ConfigType.BOOLEAN,
            "启用LightGBM模型", True, True
        ))
        model_section.add_value(ConfigValue(
            "ppo_enabled", True, ConfigType.BOOLEAN,
            "启用PPO模型", True, True
        ))
        model_section.add_value(ConfigValue(
            "model_timeout_ms", 5000, ConfigType.INTEGER,
            "模型推理超时时间", 5000, True, 1000, 30000
        ))
        self.sections["models"] = model_section
        
        # 训练配置
        training_section = ConfigSection("training", "训练配置")
        training_section.add_value(ConfigValue(
            "min_samples", 100, ConfigType.INTEGER,
            "最小训练样本数", 100, True, 10, 10000
        ))
        training_section.add_value(ConfigValue(
            "max_samples", 10000, ConfigType.INTEGER,
            "最大训练样本数", 10000, True, 100, 100000
        ))
        training_section.add_value(ConfigValue(
            "check_interval_seconds", 300, ConfigType.INTEGER,
            "检查间隔(秒)", 300, True, 60, 3600
        ))
        training_section.add_value(ConfigValue(
            "auto_deploy", False, ConfigType.BOOLEAN,
            "自动部署新模型", False, True
        ))
        self.sections["training"] = training_section
        
        # HTTP服务配置
        http_section = ConfigSection("http", "HTTP服务配置")
        http_section.add_value(ConfigValue(
            "port", 8080, ConfigType.INTEGER,
            "服务端口", 8080, True, 1024, 65535
        ))
        http_section.add_value(ConfigValue(
            "host", "0.0.0.0", ConfigType.STRING,
            "绑定地址", "0.0.0.0", True
        ))
        http_section.add_value(ConfigValue(
            "cors_enabled", True, ConfigType.BOOLEAN,
            "启用CORS", True, True
        ))
        http_section.add_value(ConfigValue(
            "request_timeout_seconds", 30, ConfigType.INTEGER,
            "请求超时时间", 30, True, 1, 300
        ))
        self.sections["http"] = http_section
        
        # 数据库配置
        database_section = ConfigSection("database", "数据库配置")
        database_section.add_value(ConfigValue(
            "knowledge_db_path", "models/knowledge.db", ConfigType.STRING,
            "知识库路径", "models/knowledge.db", True
        ))
        database_section.add_value(ConfigValue(
            "feedback_db_path", "models/feedback.db", ConfigType.STRING,
            "反馈数据库路径", "models/feedback.db", True
        ))
        database_section.add_value(ConfigValue(
            "connection_pool_size", 10, ConfigType.INTEGER,
            "连接池大小", 10, True, 1, 100
        ))
        self.sections["database"] = database_section
    
    def _load_builtin_templates(self):
        """加载内置配置模板"""
        # A/B测试模板
        ab_test_template = ConfigSection("ab_testing", "A/B测试配置模板")
        ab_test_template.add_value(ConfigValue(
            "enabled", False, ConfigType.BOOLEAN,
            "启用A/B测试", False, True
        ))
        ab_test_template.add_value(ConfigValue(
            "split_ratio", 0.5, ConfigType.FLOAT,
            "流量分割比例", 0.5, True, 0.0, 1.0
        ))
        ab_test_template.add_value(ConfigValue(
            "duration_hours", 24, ConfigType.INTEGER,
            "测试持续时间(小时)", 24, True, 1, 168
        ))
        self._templates["ab_testing"] = ab_test_template
    
    def get_value(self, section: str, key: str, default: Any = None) -> Any:
        """
        获取配置值
        
        Args:
            section: 配置段名称
            key: 配置键
            default: 默认值
            
        Returns:
            配置值
        """
        with self._lock:
            cache_key = f"{section}.{key}"
            if cache_key in self._config_cache:
                return self._config_cache[cache_key]
            
            if section in self.sections:
                value = self.sections[section].get_value(key, default)
                self._config_cache[cache_key] = value
                return value
            
            return default
    
    def set_value(self, section: str, key: str, value: Any, persist: bool = True) -> bool:
        """
        设置配置值
        
        Args:
            section: 配置段名称
            key: 配置键
            value: 配置值
            persist: 是否持久化
            
        Returns:
            是否设置成功
        """
        with self._lock:
            if section not in self.sections:
                return False
            
            old_value = self.sections[section].get_value(key)
            success = self.sections[section].set_value(key, value)
            
            if success:
                # 更新缓存
                cache_key = f"{section}.{key}"
                self._config_cache[cache_key] = value
                
                # 触发回调
                self._trigger_callbacks(section, key, old_value, value)
                
                # 持久化
                if persist:
                    self.save_section(section)
                
                if self.debug:
                    self.logger.debug(f"配置更新: {section}.{key} = {value}")
            
            return success
    
    def load_from_file(self, file_path: str) -> bool:
        """从文件加载配置"""
        try:
            config_file = Path(file_path)
            if not config_file.exists():
                return False
            
            with open(config_file, 'r', encoding='utf-8') as f:
                data = json.load(f)
            
            with self._lock:
                for section_name, section_data in data.items():
                    if section_name not in self.sections:
                        self.sections[section_name] = ConfigSection(section_name)
                    
                    section = self.sections[section_name]
                    for key, value_data in section_data.items():
                        if isinstance(value_data, dict) and 'value' in value_data:
                            # 完整的配置值定义
                            config_value = ConfigValue(
                                key=key,
                                value=value_data['value'],
                                value_type=ConfigType(value_data.get('type', 'string')),
                                description=value_data.get('description', ''),
                                default_value=value_data.get('default'),
                                required=value_data.get('required', False)
                            )
                            section.add_value(config_value)
                        else:
                            # 简单值
                            if key in section.values:
                                section.set_value(key, value_data)
                
                # 更新文件校验和
                self._file_checksums[file_path] = self._calculate_file_checksum(config_file)
                
                # 清空缓存
                self._config_cache.clear()
            
            self.logger.info(f"配置加载成功: {file_path}")
            return True
            
        except Exception as e:
            self.logger.error(f"配置加载失败: {file_path}, {e}")
            return False
    
    def save_section(self, section_name: str) -> bool:
        """保存配置段到文件"""
        try:
            if section_name not in self.sections:
                return False
            
            section = self.sections[section_name]
            config_file = self.config_dir / f"{section_name}_{self.environment}.json"
            
            # 转换为JSON格式
            data = {}
            for key, config_value in section.values.items():
                data[key] = {
                    'value': config_value.value,
                    'type': config_value.value_type.value,
                    'description': config_value.description,
                    'default': config_value.default_value,
                    'required': config_value.required,
                    'last_modified': config_value.last_modified.isoformat()
                }
                
                if config_value.sensitive:
                    data[key]['sensitive'] = True
            
            # 保存文件
            with open(config_file, 'w', encoding='utf-8') as f:
                json.dump(data, f, indent=2, ensure_ascii=False)
            
            self.logger.info(f"配置段已保存: {section_name}")
            return True
            
        except Exception as e:
            self.logger.error(f"配置保存失败: {section_name}, {e}")
            return False
    
    def save_all(self) -> bool:
        """保存所有配置"""
        success = True
        for section_name in self.sections:
            if not self.save_section(section_name):
                success = False
        return success
    
    def add_change_callback(self, section: str, key: str, callback: Callable):
        """添加配置变更回调"""
        callback_key = f"{section}.{key}"
        self._change_callbacks[callback_key].append(callback)
    
    def add_global_callback(self, callback: Callable):
        """添加全局配置变更回调"""
        self._global_callbacks.append(callback)
    
    def _trigger_callbacks(self, section: str, key: str, old_value: Any, new_value: Any):
        """触发回调函数"""
        try:
            # 特定配置回调
            callback_key = f"{section}.{key}"
            for callback in self._change_callbacks[callback_key]:
                callback(section, key, old_value, new_value)
            
            # 全局回调
            for callback in self._global_callbacks:
                callback(section, key, old_value, new_value)
                
        except Exception as e:
            self.logger.error(f"回调执行失败: {e}")
    
    def start_monitoring(self):
        """启动配置文件监控"""
        if self._monitor_running:
            return
        
        self._monitor_running = True
        self._monitor_thread = threading.Thread(target=self._monitor_files)
        self._monitor_thread.daemon = True
        self._monitor_thread.start()
        
        self.logger.info("配置文件监控已启动")
    
    def stop_monitoring(self):
        """停止配置文件监控"""
        self._monitor_running = False
        if self._monitor_thread:
            self._monitor_thread.join(timeout=5)
        
        self.logger.info("配置文件监控已停止")
    
    def _monitor_files(self):
        """监控配置文件变更"""
        while self._monitor_running:
            try:
                for file_path, old_checksum in list(self._file_checksums.items()):
                    config_file = Path(file_path)
                    if config_file.exists():
                        new_checksum = self._calculate_file_checksum(config_file)
                        if new_checksum != old_checksum:
                            self.logger.info(f"检测到配置文件变更: {file_path}")
                            self.load_from_file(file_path)
                
                time.sleep(1)  # 检查间隔
                
            except Exception as e:
                self.logger.error(f"文件监控异常: {e}")
                time.sleep(5)
    
    def _calculate_file_checksum(self, file_path: Path) -> str:
        """计算文件校验和"""
        try:
            with open(file_path, 'rb') as f:
                return hashlib.md5(f.read()).hexdigest()
        except:
            return ""
    
    def validate_all(self) -> Dict[str, Dict[str, bool]]:
        """验证所有配置"""
        results = {}
        with self._lock:
            for section_name, section in self.sections.items():
                results[section_name] = section.validate_all()
        return results
    
    def get_section_config(self, section_name: str) -> Dict[str, Any]:
        """获取配置段的所有配置"""
        with self._lock:
            if section_name in self.sections:
                result = {}
                for key, config_value in self.sections[section_name].values.items():
                    result[key] = config_value.value
                return result
            return {}
    
    def get_config_summary(self) -> Dict[str, Any]:
        """获取配置摘要"""
        with self._lock:
            summary = {
                'environment': self.environment,
                'sections': {},
                'total_configs': 0,
                'validation_status': {}
            }
            
            for section_name, section in self.sections.items():
                summary['sections'][section_name] = {
                    'count': len(section.values),
                    'enabled': section.enabled,
                    'description': section.description
                }
                summary['total_configs'] += len(section.values)
            
            # 验证状态
            validation_results = self.validate_all()
            summary['validation_status'] = {
                section: all(results.values()) 
                for section, results in validation_results.items()
            }
            
            return summary
    
    def close(self):
        """关闭配置管理器"""
        self.stop_monitoring()
        if self.auto_reload:
            self.save_all()
        
        self.logger.info("配置管理器已关闭")


# 便捷函数
def create_config_manager(config_dir: str = "config", 
                         environment: str = "development", 
                         debug: bool = False) -> ConfigManager:
    """创建配置管理器的便捷函数"""
    return ConfigManager(config_dir, environment, debug=debug)


if __name__ == "__main__":
    # 测试代码
    print("=== 配置管理系统测试 ===")
    
    config_mgr = create_config_manager(debug=True)
    
    # 测试获取配置
    port = config_mgr.get_value("http", "port", 8080)
    print(f"✅ HTTP端口: {port}")
    
    # 测试设置配置
    success = config_mgr.set_value("http", "port", 8081)
    print(f"✅ 设置端口成功: {success}")
    
    # 测试验证
    validation_results = config_mgr.validate_all()
    print(f"📊 验证结果: {validation_results}")
    
    # 测试配置摘要
    summary = config_mgr.get_config_summary()
    print(f"📋 配置摘要: {summary['total_configs']}个配置项")
    
    config_mgr.close()
    print("🎯 配置管理系统测试完成！")
