"""
Local Interoperability System - 本地互操作系统

基于PyO3和直接内存共享的本地化架构，替代HTTP网络通信

核心组件:
- PyO3 Rust-Python绑定
- 共享内存通信
- 直接函数调用
- 进程内嵌入
"""

from .pyo3_bridge import PyO3Bridge, RustFunction
from .memory_channel import MemoryChannel, SharedMemoryPool

__all__ = [
    'PyO3Bridge',
    'RustFunction', 
    'MemoryChannel',
    'SharedMemoryPool'
]
