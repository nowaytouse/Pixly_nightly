# 🔌 PIXLY v3.0 插件系统
#
# 基于Go废弃模块的架构经验构建
# 支持动态加载、热插拔、版本管理

from .plugin_manager import PluginManager
from .plugin_registry import PluginRegistry  
from .plugin_loader import PluginLoader
from .plugin_interfaces import IPixlyPlugin, IProcessorPlugin, IValidatorPlugin

__all__ = [
    'PluginManager',
    'PluginRegistry',
    'PluginLoader', 
    'IPixlyPlugin',
    'IProcessorPlugin',
    'IValidatorPlugin'
]
