"""
Rust-Python融合层 - 双核心架构的统一接口

实现Rust高性能计算与Python智能AI的完美融合：
- Rust处理密集计算 (图像处理、数学运算)
- Python处理AI推理 (模型预测、智能分析)
- 零拷贝数据交换
- 统一调用接口

架构设计：
    用户请求 → 融合调度器 → Rust核心 + Python AI → 融合结果
"""

from .rust_python_bridge import RustPythonBridge, get_global_bridge
from .unified_processor import UnifiedProcessor, get_unified_processor
from .performance_optimizer import PerformanceOptimizer

__all__ = [
    'RustPythonBridge',
    'get_global_bridge',
    'UnifiedProcessor', 
    'get_unified_processor',
    'PerformanceOptimizer'
]

# 版本信息
__version__ = "3.0.0"
__architecture__ = "rust_python_fusion"
__performance_target__ = "100k_ops_per_second"
