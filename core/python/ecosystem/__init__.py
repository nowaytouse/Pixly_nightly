# 🌐 PIXLY v3.0 生态系统建设
#
# 第四轮架构增强：从废弃Go模块中萃取企业级基础设施
# ================================
#
# 🎯 目标：
# 1. 插件架构与开发工具
# 2. 企业级监控与可观测性
# 3. 开发体验优化
#
# 📦 模块结构：
# ├── plugins/          # 插件系统
# ├── monitoring/       # 监控系统
# ├── devtools/         # 开发工具
# ├── cli/             # 命令行工具
# └── validation/      # 企业级验证

__version__ = "3.0.0"
__author__ = "PIXLY Team"
__description__ = "Pixly v3.0 Ecosystem - 企业级生态系统建设"

# 导出核心组件
from .plugins import PluginManager
from .monitoring import MetricsCollector, HealthMonitor
from .devtools import DevToolsManager
from .cli import CLIManager
from .validation import ValidationEngine

__all__ = [
    'PluginManager',
    'MetricsCollector', 
    'HealthMonitor',
    'DevToolsManager',
    'CLIManager',
    'ValidationEngine'
]
