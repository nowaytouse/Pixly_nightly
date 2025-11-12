"""
🔌 PIXLY v3.0 插件管理器

基于Go废弃模块的企业级插件架构：
- 动态加载与热插拔
- 版本依赖管理  
- 插件沙箱隔离
- 性能监控集成
- 自动故障恢复

迁移自：Go http_gateway.go 的模块化设计理念
"""

import os
import sys
import json
import time
import traceback
import importlib.util
from pathlib import Path
from typing import Dict, List, Optional, Any, Callable
from dataclasses import dataclass, asdict
from concurrent.futures import ThreadPoolExecutor
import threading

from .plugin_interfaces import IPixlyPlugin, PluginLoadResult
from .plugin_registry import PluginRegistry
from .plugin_loader import PluginLoader
from ..monitoring.metrics_collector import MetricsCollector
from ..validation.validation_engine import ValidationEngine


@dataclass
class PluginConfig:
    """插件配置"""
    name: str
    version: str
    enabled: bool = True
    auto_reload: bool = False
    dependencies: List[str] = None
    config: Dict[str, Any] = None
    priority: int = 100  # 加载优先级 (数值越小优先级越高)
    
    def __post_init__(self):
        if self.dependencies is None:
            self.dependencies = []
        if self.config is None:
            self.config = {}


@dataclass 
class PluginStatus:
    """插件状态"""
    name: str
    version: str
    status: str  # loaded, failed, disabled, unloaded
    load_time: float
    error_message: Optional[str] = None
    last_heartbeat: float = 0.0
    restart_count: int = 0
    
    
class PluginManager:
    """
    🔌 企业级插件管理器
    
    核心特性：
    - 热插拔支持
    - 依赖解析
    - 自动故障恢复
    - 性能监控
    - 版本冲突检测
    """
    
    def __init__(self, plugins_dir: str = "plugins", 
                 config_path: str = "config/plugins.json"):
        self.plugins_dir = Path(plugins_dir)
        self.config_path = Path(config_path)
        
        # 核心组件
        self.registry = PluginRegistry()
        self.loader = PluginLoader()
        self.metrics = MetricsCollector()
        self.validator = ValidationEngine()
        
        # 状态管理
        self.loaded_plugins: Dict[str, IPixlyPlugin] = {}
        self.plugin_configs: Dict[str, PluginConfig] = {}
        self.plugin_status: Dict[str, PluginStatus] = {}
        
        # 线程安全
        self._lock = threading.RLock()
        self._executor = ThreadPoolExecutor(max_workers=4)
        
        # 监控线程
        self._monitor_thread = None
        self._stop_monitoring = False
        
        # 钩子函数
        self.hooks: Dict[str, List[Callable]] = {
            'before_load': [],
            'after_load': [],
            'before_unload': [],
            'after_unload': [],
            'on_error': []
        }
        
        # 初始化
        self._init_directories()
        self._load_config()
        
    def _init_directories(self):
        """初始化目录结构"""
        self.plugins_dir.mkdir(parents=True, exist_ok=True)
        self.config_path.parent.mkdir(parents=True, exist_ok=True)
        
        # 创建示例插件目录
        (self.plugins_dir / "processors").mkdir(exist_ok=True)
        (self.plugins_dir / "validators").mkdir(exist_ok=True) 
        (self.plugins_dir / "enhancers").mkdir(exist_ok=True)
        
    def _load_config(self):
        """加载插件配置"""
        if not self.config_path.exists():
            self._create_default_config()
            return
            
        try:
            with open(self.config_path, 'r', encoding='utf-8') as f:
                config_data = json.load(f)
                
            for plugin_data in config_data.get('plugins', []):
                config = PluginConfig(**plugin_data)
                self.plugin_configs[config.name] = config
                
        except Exception as e:
            print(f"⚠️ 加载插件配置失败: {e}")
            self._create_default_config()
    
    def _create_default_config(self):
        """创建默认配置"""
        default_config = {
            "version": "3.0.0",
            "auto_discovery": True,
            "hot_reload": True,
            "plugins": [
                {
                    "name": "core_processor",
                    "version": "1.0.0",
                    "enabled": True,
                    "auto_reload": True,
                    "dependencies": [],
                    "config": {},
                    "priority": 10
                }
            ]
        }
        
        with open(self.config_path, 'w', encoding='utf-8') as f:
            json.dump(default_config, f, indent=2, ensure_ascii=False)
    
    def register_hook(self, event: str, callback: Callable):
        """注册钩子函数"""
        if event in self.hooks:
            self.hooks[event].append(callback)
    
    def _call_hooks(self, event: str, *args, **kwargs):
        """调用钩子函数"""
        for callback in self.hooks.get(event, []):
            try:
                callback(*args, **kwargs)
            except Exception as e:
                print(f"⚠️ 钩子执行失败 {event}: {e}")
    
    def discover_plugins(self) -> List[str]:
        """自动发现插件"""
        plugins_found = []
        
        for plugin_file in self.plugins_dir.rglob("*.py"):
            if plugin_file.name.startswith('__'):
                continue
                
            # 检查是否为有效插件
            try:
                spec = importlib.util.spec_from_file_location(
                    plugin_file.stem, plugin_file
                )
                if spec and spec.loader:
                    plugins_found.append(plugin_file.stem)
                    
            except Exception:
                continue
        
        self.metrics.record_counter("plugins_discovered", len(plugins_found))
        return plugins_found
    
    def load_plugin(self, plugin_name: str, force_reload: bool = False) -> bool:
        """加载单个插件"""
        start_time = time.time()
        
        with self._lock:
            # 检查是否已加载
            if plugin_name in self.loaded_plugins and not force_reload:
                return True
            
            # 获取配置
            config = self.plugin_configs.get(plugin_name)
            if not config:
                config = PluginConfig(name=plugin_name, version="1.0.0")
                self.plugin_configs[plugin_name] = config
            
            if not config.enabled:
                return False
            
            # 调用前置钩子
            self._call_hooks('before_load', plugin_name, config)
            
            try:
                # 检查依赖
                if not self._check_dependencies(config.dependencies):
                    raise RuntimeError(f"插件依赖不满足: {config.dependencies}")
                
                # 加载插件
                result = self.loader.load_plugin(plugin_name, self.plugins_dir)
                
                if result.success and result.plugin:
                    # 验证插件
                    if self._validate_plugin(result.plugin):
                        # 初始化插件
                        if hasattr(result.plugin, 'initialize'):
                            result.plugin.initialize(config.config)
                        
                        # 注册插件
                        self.loaded_plugins[plugin_name] = result.plugin
                        self.registry.register_plugin(plugin_name, result.plugin)
                        
                        # 更新状态
                        self.plugin_status[plugin_name] = PluginStatus(
                            name=plugin_name,
                            version=config.version,
                            status="loaded",
                            load_time=time.time() - start_time,
                            last_heartbeat=time.time()
                        )
                        
                        # 调用后置钩子
                        self._call_hooks('after_load', plugin_name, result.plugin)
                        
                        # 记录指标
                        self.metrics.record_counter("plugin_loads_success")
                        self.metrics.record_histogram("plugin_load_time", 
                                                    time.time() - start_time)
                        
                        print(f"✅ 插件加载成功: {plugin_name}")
                        return True
                    else:
                        raise RuntimeError("插件验证失败")
                else:
                    raise RuntimeError(f"插件加载失败: {result.error}")
                    
            except Exception as e:
                error_msg = f"插件 {plugin_name} 加载失败: {e}"
                print(f"❌ {error_msg}")
                
                # 更新错误状态
                self.plugin_status[plugin_name] = PluginStatus(
                    name=plugin_name,
                    version=config.version,
                    status="failed",
                    load_time=time.time() - start_time,
                    error_message=str(e)
                )
                
                # 调用错误钩子
                self._call_hooks('on_error', plugin_name, e)
                
                # 记录指标
                self.metrics.record_counter("plugin_loads_failed")
                
                return False
    
    def _check_dependencies(self, dependencies: List[str]) -> bool:
        """检查插件依赖"""
        for dep in dependencies:
            if dep not in self.loaded_plugins:
                print(f"⚠️ 缺少依赖插件: {dep}")
                return False
        return True
    
    def _validate_plugin(self, plugin: IPixlyPlugin) -> bool:
        """验证插件接口"""
        try:
            # 检查必要方法
            required_methods = ['get_info', 'process']
            for method in required_methods:
                if not hasattr(plugin, method):
                    print(f"❌ 插件缺少必要方法: {method}")
                    return False
            
            # 调用验证引擎
            return self.validator.validate_plugin(plugin)
            
        except Exception as e:
            print(f"❌ 插件验证异常: {e}")
            return False
    
    def unload_plugin(self, plugin_name: str) -> bool:
        """卸载插件"""
        with self._lock:
            if plugin_name not in self.loaded_plugins:
                return True
            
            try:
                plugin = self.loaded_plugins[plugin_name]
                
                # 调用前置钩子
                self._call_hooks('before_unload', plugin_name, plugin)
                
                # 清理插件
                if hasattr(plugin, 'cleanup'):
                    plugin.cleanup()
                
                # 从注册表移除
                self.registry.unregister_plugin(plugin_name)
                
                # 移除引用
                del self.loaded_plugins[plugin_name]
                
                # 更新状态
                if plugin_name in self.plugin_status:
                    self.plugin_status[plugin_name].status = "unloaded"
                
                # 调用后置钩子
                self._call_hooks('after_unload', plugin_name)
                
                # 记录指标
                self.metrics.record_counter("plugin_unloads_success")
                
                print(f"✅ 插件卸载成功: {plugin_name}")
                return True
                
            except Exception as e:
                print(f"❌ 插件卸载失败 {plugin_name}: {e}")
                self.metrics.record_counter("plugin_unloads_failed")
                return False
    
    def reload_plugin(self, plugin_name: str) -> bool:
        """重新加载插件"""
        print(f"🔄 重新加载插件: {plugin_name}")
        
        if self.unload_plugin(plugin_name):
            return self.load_plugin(plugin_name, force_reload=True)
        
        return False
    
    def load_all_plugins(self) -> Dict[str, bool]:
        """加载所有配置的插件"""
        results = {}
        
        # 按优先级排序
        sorted_plugins = sorted(
            self.plugin_configs.items(),
            key=lambda x: x[1].priority
        )
        
        for plugin_name, config in sorted_plugins:
            if config.enabled:
                results[plugin_name] = self.load_plugin(plugin_name)
        
        return results
    
    def get_plugin_status(self) -> Dict[str, Dict[str, Any]]:
        """获取所有插件状态"""
        with self._lock:
            status_dict = {}
            for name, status in self.plugin_status.items():
                status_dict[name] = asdict(status)
            return status_dict
    
    def get_loaded_plugins(self) -> Dict[str, IPixlyPlugin]:
        """获取已加载的插件"""
        with self._lock:
            return self.loaded_plugins.copy()
    
    def start_monitoring(self):
        """启动插件监控"""
        if self._monitor_thread is not None:
            return
        
        self._stop_monitoring = False
        self._monitor_thread = threading.Thread(target=self._monitor_loop)
        self._monitor_thread.daemon = True
        self._monitor_thread.start()
        
        print("🔍 插件监控已启动")
    
    def stop_monitoring(self):
        """停止插件监控"""
        self._stop_monitoring = True
        if self._monitor_thread:
            self._monitor_thread.join(timeout=5.0)
            self._monitor_thread = None
        
        print("🛑 插件监控已停止")
    
    def _monitor_loop(self):
        """监控循环"""
        while not self._stop_monitoring:
            try:
                self._check_plugin_health()
                self._handle_auto_reload()
                time.sleep(5)  # 5秒检查一次
                
            except Exception as e:
                print(f"⚠️ 监控循环异常: {e}")
                time.sleep(10)
    
    def _check_plugin_health(self):
        """检查插件健康状态"""
        current_time = time.time()
        
        with self._lock:
            for name, plugin in self.loaded_plugins.items():
                try:
                    # 调用心跳检查
                    if hasattr(plugin, 'health_check'):
                        if plugin.health_check():
                            self.plugin_status[name].last_heartbeat = current_time
                        else:
                            print(f"⚠️ 插件健康检查失败: {name}")
                            self._handle_plugin_failure(name)
                            
                except Exception as e:
                    print(f"❌ 插件健康检查异常 {name}: {e}")
                    self._handle_plugin_failure(name)
    
    def _handle_plugin_failure(self, plugin_name: str):
        """处理插件故障"""
        if plugin_name not in self.plugin_status:
            return
        
        status = self.plugin_status[plugin_name]
        config = self.plugin_configs.get(plugin_name)
        
        if not config or not config.enabled:
            return
        
        # 增加重启计数
        status.restart_count += 1
        status.status = "failed"
        
        # 如果重启次数过多，禁用插件
        if status.restart_count >= 3:
            print(f"🚫 插件 {plugin_name} 重启次数过多，已禁用")
            config.enabled = False
            return
        
        # 尝试重新加载
        print(f"🔄 尝试重新加载故障插件: {plugin_name}")
        if self.reload_plugin(plugin_name):
            status.restart_count = 0  # 重置计数
    
    def _handle_auto_reload(self):
        """处理自动重载"""
        # 检查文件变更等逻辑
        pass
    
    def call_plugin_method(self, plugin_name: str, method_name: str, 
                          *args, **kwargs) -> Any:
        """安全调用插件方法"""
        with self._lock:
            if plugin_name not in self.loaded_plugins:
                raise RuntimeError(f"插件未加载: {plugin_name}")
            
            plugin = self.loaded_plugins[plugin_name]
            
            if not hasattr(plugin, method_name):
                raise RuntimeError(f"插件 {plugin_name} 不支持方法: {method_name}")
            
            try:
                return getattr(plugin, method_name)(*args, **kwargs)
                
            except Exception as e:
                print(f"❌ 插件方法调用失败 {plugin_name}.{method_name}: {e}")
                self.metrics.record_counter("plugin_method_calls_failed")
                raise
    
    def shutdown(self):
        """关闭插件管理器"""
        print("🛑 正在关闭插件管理器...")
        
        # 停止监控
        self.stop_monitoring()
        
        # 卸载所有插件
        with self._lock:
            plugin_names = list(self.loaded_plugins.keys())
            for name in plugin_names:
                self.unload_plugin(name)
        
        # 关闭线程池
        self._executor.shutdown(wait=True)
        
        print("✅ 插件管理器已关闭")
    
    def get_metrics(self) -> Dict[str, Any]:
        """获取插件系统指标"""
        return {
            "total_plugins": len(self.plugin_configs),
            "loaded_plugins": len(self.loaded_plugins),
            "failed_plugins": len([s for s in self.plugin_status.values() 
                                 if s.status == "failed"]),
            "plugin_status": self.get_plugin_status(),
            "system_metrics": self.metrics.get_all_metrics()
        }
