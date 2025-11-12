"""
Local Direct Call Architecture - 本地直接调用架构

完全废弃HTTP网关，使用本地调度器
"""

from .local_dispatcher import LocalDispatcher, get_local_dispatcher
from .local_dispatcher import predict_params_local, process_image_local, health_check_local

__all__ = [
    'LocalDispatcher',
    'get_local_dispatcher',
    'predict_params_local',
    'process_image_local', 
    'health_check_local'
]
