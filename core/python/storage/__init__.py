"""
存储模块
基于废弃Go代码价值提取

EX-014实现: SQLite观测存储器
功能:
- observation_store.py: 训练观测数据持久化存储
"""

from .observation_store import (
    Observation,
    Statistics,
    ObservationStoreConfig,
    ObservationStore,
    get_observation_store,
    save_observation,
    load_observations,
    get_observation_statistics
)

__all__ = [
    # 数据结构
    'Observation',
    'Statistics',
    'ObservationStoreConfig',
    
    # 核心存储器
    'ObservationStore',
    'get_observation_store',
    
    # 便捷函数
    'save_observation',
    'load_observations', 
    'get_observation_statistics',
]
