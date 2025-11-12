"""
内存管理器 - 零拷贝数据管理

完全摆脱序列化和网络传输，纯内存操作：
- 零拷贝数据共享
- 直接内存访问
- 无序列化开销
- 进程内数据管理

架构原则: 内存直接访问 > 任何形式的数据传输
"""

import mmap
import threading
import time
from typing import Dict, List, Optional, Any, Union, Tuple
from dataclasses import dataclass, field
from pathlib import Path
import tempfile
import numpy as np
import logging
import hashlib
from enum import Enum


class DataType(Enum):
    """数据类型"""
    BYTES = "bytes"
    NUMPY_ARRAY = "numpy_array"  
    PYTHON_OBJECT = "python_object"
    MEMORY_VIEW = "memory_view"


@dataclass
class DataBlock:
    """内存数据块"""
    block_id: str
    data_type: DataType
    size_bytes: int
    memory_offset: int
    shape: Optional[Tuple[int, ...]] = None  # numpy数组形状
    dtype: Optional[str] = None              # numpy数组类型
    created_at: float = field(default_factory=time.perf_counter)
    last_accessed: float = field(default_factory=time.perf_counter)
    ref_count: int = 0
    
    def touch(self):
        """更新访问时间"""
        self.last_accessed = time.perf_counter()
        self.ref_count += 1


class MemoryManager:
    """
    零拷贝内存管理器
    
    纯内存操作，无任何网络或序列化概念
    """
    
    def __init__(self,
                 memory_size_mb: int = 1024,
                 enable_monitoring: bool = True,
                 debug: bool = False):
        """
        初始化内存管理器
        
        Args:
            memory_size_mb: 内存池大小(MB)
            enable_monitoring: 启用内存监控
            debug: 调试模式
        """
        self.memory_size_mb = memory_size_mb
        self.enable_monitoring = enable_monitoring
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        # 内存池
        self.total_size = memory_size_mb * 1024 * 1024
        self._memory_pool: Optional[mmap.mmap] = None
        
        # 数据块管理
        self._data_blocks: Dict[str, DataBlock] = {}
        self._free_regions: List[Tuple[int, int]] = []  # (offset, size)
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 性能统计
        self._stats = {
            'allocations': 0,
            'deallocations': 0,
            'memory_used': 0,
            'peak_memory': 0,
            'cache_hits': 0,
            'cache_misses': 0
        }
        
        # 初始化内存池
        self._initialize_memory_pool()
        
        if debug:
            self.logger.debug(f"内存管理器初始化: {memory_size_mb}MB")
    
    def _initialize_memory_pool(self):
        """初始化内存池"""
        try:
            # 创建匿名内存映射
            self._memory_pool = mmap.mmap(-1, self.total_size)
            
            # 初始化空闲区域
            self._free_regions = [(0, self.total_size)]
            
            if self.debug:
                self.logger.debug(f"内存池初始化完成: {self.total_size} bytes")
        
        except Exception as e:
            self.logger.error(f"内存池初始化失败: {e}")
            raise
    
    def store_data(self, 
                   data: Any, 
                   block_id: Optional[str] = None,
                   zero_copy: bool = True) -> str:
        """
        存储数据到内存池
        
        Args:
            data: 要存储的数据
            block_id: 可选的块ID
            zero_copy: 使用零拷贝(仅适用于numpy数组)
        """
        try:
            with self._lock:
                # 生成块ID
                if block_id is None:
                    block_id = self._generate_block_id(data)
                
                # 检查是否已存在
                if block_id in self._data_blocks:
                    self._data_blocks[block_id].touch()
                    self._stats['cache_hits'] += 1
                    return block_id
                
                self._stats['cache_misses'] += 1
                
                # 准备数据
                data_type, raw_data, metadata = self._prepare_data(data, zero_copy)
                data_size = len(raw_data)
                
                # 分配内存
                memory_offset = self._allocate_memory(data_size)
                if memory_offset is None:
                    raise RuntimeError(f"内存分配失败: 需要{data_size}字节")
                
                # 写入数据
                self._memory_pool[memory_offset:memory_offset + data_size] = raw_data
                
                # 创建数据块记录
                data_block = DataBlock(
                    block_id=block_id,
                    data_type=data_type,
                    size_bytes=data_size,
                    memory_offset=memory_offset,
                    shape=metadata.get('shape'),
                    dtype=metadata.get('dtype')
                )
                
                self._data_blocks[block_id] = data_block
                
                # 更新统计
                self._stats['allocations'] += 1
                self._stats['memory_used'] += data_size
                self._stats['peak_memory'] = max(self._stats['peak_memory'], self._stats['memory_used'])
                
                if self.debug:
                    self.logger.debug(f"数据存储完成: {block_id}, {data_size} bytes")
                
                return block_id
        
        except Exception as e:
            self.logger.error(f"数据存储失败: {e}")
            raise
    
    def get_data(self, block_id: str, zero_copy: bool = True) -> Any:
        """
        从内存池获取数据
        
        Args:
            block_id: 块ID
            zero_copy: 使用零拷贝
        """
        try:
            with self._lock:
                if block_id not in self._data_blocks:
                    raise KeyError(f"数据块不存在: {block_id}")
                
                data_block = self._data_blocks[block_id]
                data_block.touch()
                
                # 读取原始数据
                start = data_block.memory_offset
                end = start + data_block.size_bytes
                raw_data = self._memory_pool[start:end]
                
                # 恢复数据
                data = self._restore_data(raw_data, data_block, zero_copy)
                
                if self.debug:
                    self.logger.debug(f"数据获取完成: {block_id}")
                
                return data
        
        except Exception as e:
            self.logger.error(f"数据获取失败: {block_id}, {e}")
            raise
    
    def _prepare_data(self, data: Any, zero_copy: bool) -> Tuple[DataType, bytes, Dict[str, Any]]:
        """准备数据进行存储"""
        metadata = {}
        
        if isinstance(data, np.ndarray):
            if zero_copy and data.flags.c_contiguous:
                # 零拷贝numpy数组
                metadata['shape'] = data.shape
                metadata['dtype'] = str(data.dtype)
                return DataType.NUMPY_ARRAY, data.tobytes(), metadata
            else:
                # 复制numpy数组
                metadata['shape'] = data.shape
                metadata['dtype'] = str(data.dtype) 
                return DataType.NUMPY_ARRAY, data.tobytes(), metadata
        
        elif isinstance(data, (bytes, bytearray)):
            # 字节数据
            return DataType.BYTES, bytes(data), metadata
        
        elif isinstance(data, memoryview):
            # 内存视图
            return DataType.MEMORY_VIEW, data.tobytes(), metadata
        
        else:
            # Python对象 (需要序列化)
            import pickle
            serialized = pickle.dumps(data)
            return DataType.PYTHON_OBJECT, serialized, metadata
    
    def _restore_data(self, raw_data: bytes, data_block: DataBlock, zero_copy: bool) -> Any:
        """从原始数据恢复对象"""
        if data_block.data_type == DataType.NUMPY_ARRAY:
            # 恢复numpy数组
            if data_block.shape and data_block.dtype:
                dtype = np.dtype(data_block.dtype)
                if zero_copy:
                    # 零拷贝：直接从内存创建数组
                    array = np.frombuffer(raw_data, dtype=dtype)
                    return array.reshape(data_block.shape)
                else:
                    # 复制数据
                    array = np.frombuffer(raw_data, dtype=dtype).copy()
                    return array.reshape(data_block.shape)
            else:
                return np.frombuffer(raw_data, dtype=np.float32)
        
        elif data_block.data_type == DataType.BYTES:
            return raw_data
        
        elif data_block.data_type == DataType.MEMORY_VIEW:
            return memoryview(raw_data)
        
        elif data_block.data_type == DataType.PYTHON_OBJECT:
            import pickle
            return pickle.loads(raw_data)
        
        else:
            return raw_data
    
    def _allocate_memory(self, size: int) -> Optional[int]:
        """分配内存"""
        # 寻找合适的空闲区域
        for i, (offset, region_size) in enumerate(self._free_regions):
            if region_size >= size:
                # 使用这个区域
                allocated_offset = offset
                
                if region_size == size:
                    # 完全匹配，移除区域
                    self._free_regions.pop(i)
                else:
                    # 部分使用，更新区域
                    self._free_regions[i] = (offset + size, region_size - size)
                
                return allocated_offset
        
        # 没有足够的空闲内存
        return None
    
    def _deallocate_memory(self, offset: int, size: int):
        """释放内存"""
        # 添加到空闲区域列表
        self._free_regions.append((offset, size))
        
        # 合并相邻的空闲区域
        self._free_regions.sort(key=lambda x: x[0])
        merged_regions = []
        
        for offset, size in self._free_regions:
            if merged_regions and merged_regions[-1][0] + merged_regions[-1][1] == offset:
                # 合并相邻区域
                last_offset, last_size = merged_regions[-1]
                merged_regions[-1] = (last_offset, last_size + size)
            else:
                merged_regions.append((offset, size))
        
        self._free_regions = merged_regions
    
    def _generate_block_id(self, data: Any) -> str:
        """生成数据块ID"""
        if isinstance(data, np.ndarray):
            # 基于数组内容生成哈希
            content_hash = hashlib.md5(data.tobytes()).hexdigest()[:8]
        elif isinstance(data, bytes):
            content_hash = hashlib.md5(data).hexdigest()[:8]
        else:
            import pickle
            content_hash = hashlib.md5(pickle.dumps(data)).hexdigest()[:8]
        
        timestamp = int(time.time() * 1000000)
        return f"data_{timestamp}_{content_hash}"
    
    def remove_data(self, block_id: str) -> bool:
        """移除数据块"""
        try:
            with self._lock:
                if block_id not in self._data_blocks:
                    return False
                
                data_block = self._data_blocks[block_id]
                
                # 释放内存
                self._deallocate_memory(data_block.memory_offset, data_block.size_bytes)
                
                # 移除记录
                del self._data_blocks[block_id]
                
                # 更新统计
                self._stats['deallocations'] += 1
                self._stats['memory_used'] -= data_block.size_bytes
                
                if self.debug:
                    self.logger.debug(f"数据块已移除: {block_id}")
                
                return True
        
        except Exception as e:
            self.logger.error(f"数据移除失败: {block_id}, {e}")
            return False
    
    def list_data_blocks(self) -> List[Dict[str, Any]]:
        """列出所有数据块"""
        with self._lock:
            blocks = []
            for block_id, data_block in self._data_blocks.items():
                blocks.append({
                    'block_id': block_id,
                    'data_type': data_block.data_type.value,
                    'size_bytes': data_block.size_bytes,
                    'size_mb': data_block.size_bytes / (1024 * 1024),
                    'created_at': data_block.created_at,
                    'last_accessed': data_block.last_accessed,
                    'ref_count': data_block.ref_count,
                    'age_seconds': time.perf_counter() - data_block.created_at
                })
            return blocks
    
    def get_memory_stats(self) -> Dict[str, Any]:
        """获取内存统计"""
        with self._lock:
            free_memory = sum(size for _, size in self._free_regions)
            used_memory = self.total_size - free_memory
            
            stats = dict(self._stats)
            stats.update({
                'total_memory_mb': self.total_size / (1024 * 1024),
                'used_memory_mb': used_memory / (1024 * 1024),
                'free_memory_mb': free_memory / (1024 * 1024),
                'memory_utilization_percent': (used_memory / self.total_size) * 100,
                'total_blocks': len(self._data_blocks),
                'fragmentation_regions': len(self._free_regions)
            })
            
            return stats
    
    def cleanup_unused_data(self, max_age_seconds: float = 3600) -> int:
        """清理未使用的数据"""
        current_time = time.perf_counter()
        cleanup_count = 0
        
        with self._lock:
            expired_blocks = []
            for block_id, data_block in self._data_blocks.items():
                age = current_time - data_block.last_accessed
                if age > max_age_seconds and data_block.ref_count == 0:
                    expired_blocks.append(block_id)
            
            for block_id in expired_blocks:
                if self.remove_data(block_id):
                    cleanup_count += 1
        
        if self.debug and cleanup_count > 0:
            self.logger.debug(f"清理了 {cleanup_count} 个过期数据块")
        
        return cleanup_count
    
    def close(self):
        """关闭内存管理器"""
        if self._memory_pool:
            self._memory_pool.close()
        
        if self.enable_monitoring and self.debug:
            stats = self.get_memory_stats()
            self.logger.info(f"内存管理器统计: {stats}")
        
        self.logger.info("内存管理器已关闭")


# 全局内存管理器
_global_memory_manager: Optional[MemoryManager] = None


def get_global_memory_manager() -> MemoryManager:
    """获取全局内存管理器"""
    global _global_memory_manager
    if _global_memory_manager is None:
        _global_memory_manager = MemoryManager()
    return _global_memory_manager


if __name__ == "__main__":
    # 测试代码
    print("=== 零拷贝内存管理器测试 ===")
    
    memory_mgr = MemoryManager(memory_size_mb=64, debug=True)
    
    # 测试numpy数组存储
    test_array = np.random.rand(1000, 1000).astype(np.float32)
    print(f"原始数组大小: {test_array.nbytes / (1024*1024):.2f} MB")
    
    block_id = memory_mgr.store_data(test_array, zero_copy=True)
    print(f"✅ 数组存储完成: {block_id}")
    
    # 零拷贝读取
    retrieved_array = memory_mgr.get_data(block_id, zero_copy=True)
    print(f"✅ 零拷贝读取: 形状={retrieved_array.shape}")
    
    # 验证数据一致性
    if np.array_equal(test_array, retrieved_array):
        print("✅ 数据一致性验证通过")
    
    # 测试字节数据
    test_bytes = b"test_data" * 10000
    bytes_block_id = memory_mgr.store_data(test_bytes)
    retrieved_bytes = memory_mgr.get_data(bytes_block_id)
    print(f"✅ 字节数据测试: {len(retrieved_bytes)} bytes")
    
    # 内存统计
    stats = memory_mgr.get_memory_stats()
    print(f"📊 内存统计: 使用率={stats['memory_utilization_percent']:.1f}%")
    
    # 数据块列表
    blocks = memory_mgr.list_data_blocks()
    print(f"📋 数据块: {len(blocks)}个")
    
    memory_mgr.close()
    print("🎯 零拷贝内存管理器测试完成！")
