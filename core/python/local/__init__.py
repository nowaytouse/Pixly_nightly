"""
Local Direct Call Architecture - 纯本地直接调用架构

完全摆脱网络概念的本地化架构:
- 没有网关、没有HTTP、没有API
- 没有序列化、没有网络传输
- 只有直接函数调用和内存共享

核心原则:
- 函数直接调用 > 网络请求
- 内存共享 > 数据传输  
- 零拷贝 > 序列化
- 进程内 > 进程间
"""

from .function_registry import FunctionRegistry, LocalFunction
from .memory_manager import MemoryManager, DataBlock
from .call_dispatcher import CallDispatcher, DirectCall

__all__ = [
    'FunctionRegistry',
    'LocalFunction',
    'MemoryManager', 
    'DataBlock',
    'CallDispatcher',
    'DirectCall'
]
