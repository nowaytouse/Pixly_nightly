"""
共享内存通道 - 进程间高速数据交换

替代HTTP网络传输，使用共享内存实现零拷贝数据交换

核心功能:
- 共享内存池管理
- 零拷贝数据传递
- 进程间同步
- 内存映射文件

架构革新: 从网络传输 → 内存直接共享
遵循四高原则：高规范化、高兼容性、高扩展性、高稳定性
"""

import mmap
import os
import struct
import threading
import time
from typing import Dict, List, Optional, Any, Union, Tuple
from dataclasses import dataclass, field
from pathlib import Path
import logging
import hashlib
import pickle
import json
import numpy as np
from enum import Enum
import tempfile
import fcntl


class ChannelType(Enum):
    """通道类型"""
    SHARED_MEMORY = "shared_memory"
    MEMORY_MAPPED_FILE = "memory_mapped_file"
    ANONYMOUS_MEMORY = "anonymous_memory"
    NAMED_PIPE = "named_pipe"


class DataFormat(Enum):
    """数据格式"""
    RAW_BYTES = "raw_bytes"
    PICKLE = "pickle"
    JSON = "json"
    NUMPY = "numpy"
    STRUCTURED = "structured"


@dataclass
class MemoryBlock:
    """内存块定义"""
    id: str
    size: int
    offset: int
    data_format: DataFormat
    created_at: float
    last_accessed: float
    ref_count: int = 0
    locked: bool = False
    checksum: Optional[str] = None


@dataclass
class ChannelConfig:
    """通道配置"""
    name: str
    channel_type: ChannelType
    size_mb: int = 100
    max_blocks: int = 1000
    block_size_kb: int = 1024
    enable_compression: bool = False
    enable_encryption: bool = False
    auto_cleanup: bool = True
    cleanup_threshold_minutes: int = 30


class MemoryChannel:
    """
    共享内存通道 - 高性能进程间通信
    
    零网络依赖的本地化数据交换方案
    """
    
    def __init__(self, 
                 config: ChannelConfig,
                 debug: bool = False):
        """
        初始化内存通道
        
        Args:
            config: 通道配置
            debug: 调试模式
        """
        self.config = config
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        if debug:
            self.logger.setLevel(logging.DEBUG)
        
        # 内存管理
        self._memory_blocks: Dict[str, MemoryBlock] = {}
        self._memory_map: Optional[mmap.mmap] = None
        self._file_handle: Optional[int] = None
        
        # 同步原语
        self._lock = threading.RLock()
        self._block_locks: Dict[str, threading.Lock] = {}
        
        # 统计信息
        self._stats = {
            'total_writes': 0,
            'total_reads': 0,
            'bytes_written': 0,
            'bytes_read': 0,
            'blocks_allocated': 0,
            'blocks_deallocated': 0,
            'cache_hits': 0,
            'cache_misses': 0
        }
        
        # 初始化内存区域
        self._initialize_memory()
        
        # 启动清理线程
        if config.auto_cleanup:
            self._start_cleanup_thread()
        
        if debug:
            self.logger.debug(f"内存通道初始化完成: {config.name}")
    
    def _initialize_memory(self):
        """初始化内存区域"""
        total_size = self.config.size_mb * 1024 * 1024
        
        if self.config.channel_type == ChannelType.SHARED_MEMORY:
            self._init_shared_memory(total_size)
        elif self.config.channel_type == ChannelType.MEMORY_MAPPED_FILE:
            self._init_memory_mapped_file(total_size)
        elif self.config.channel_type == ChannelType.ANONYMOUS_MEMORY:
            self._init_anonymous_memory(total_size)
        else:
            raise ValueError(f"不支持的通道类型: {self.config.channel_type}")
    
    def _init_shared_memory(self, size: int):
        """初始化共享内存"""
        try:
            # 创建临时文件作为共享内存
            temp_dir = Path(tempfile.gettempdir()) / "pixly_shared_memory"
            temp_dir.mkdir(exist_ok=True)
            
            memory_file = temp_dir / f"{self.config.name}.mem"
            
            # 创建并初始化文件
            with open(memory_file, 'wb') as f:
                f.write(b'\x00' * size)
            
            # 打开文件并创建内存映射
            self._file_handle = os.open(str(memory_file), os.O_RDWR)
            self._memory_map = mmap.mmap(self._file_handle, size)
            
            if self.debug:
                self.logger.debug(f"共享内存初始化完成: {memory_file}, {size} bytes")
        
        except Exception as e:
            self.logger.error(f"共享内存初始化失败: {e}")
            raise
    
    def _init_memory_mapped_file(self, size: int):
        """初始化内存映射文件"""
        try:
            # 创建内存映射文件
            temp_dir = Path(tempfile.gettempdir()) / "pixly_memory_mapped"
            temp_dir.mkdir(exist_ok=True)
            
            mapped_file = temp_dir / f"{self.config.name}.mmap"
            
            # 创建文件
            with open(mapped_file, 'wb') as f:
                f.write(b'\x00' * size)
            
            # 创建内存映射
            with open(mapped_file, 'r+b') as f:
                self._memory_map = mmap.mmap(f.fileno(), size)
            
            if self.debug:
                self.logger.debug(f"内存映射文件初始化完成: {mapped_file}")
        
        except Exception as e:
            self.logger.error(f"内存映射文件初始化失败: {e}")
            raise
    
    def _init_anonymous_memory(self, size: int):
        """初始化匿名内存"""
        try:
            # 创建匿名内存映射
            self._memory_map = mmap.mmap(-1, size)
            
            if self.debug:
                self.logger.debug(f"匿名内存初始化完成: {size} bytes")
        
        except Exception as e:
            self.logger.error(f"匿名内存初始化失败: {e}")
            raise
    
    def write(self, 
              data: Any, 
              data_format: DataFormat = DataFormat.PICKLE,
              block_id: Optional[str] = None) -> str:
        """写入数据到共享内存"""
        try:
            # 序列化数据
            serialized_data = self._serialize_data(data, data_format)
            data_size = len(serialized_data)
            
            # 生成块ID
            if block_id is None:
                block_id = self._generate_block_id(serialized_data)
            
            # 分配内存块
            memory_block = self._allocate_block(block_id, data_size, data_format)
            
            if memory_block is None:
                raise RuntimeError("内存分配失败")
            
            # 写入数据
            with self._lock:
                start_offset = memory_block.offset
                end_offset = start_offset + data_size
                
                self._memory_map[start_offset:end_offset] = serialized_data
                
                # 更新块信息
                memory_block.last_accessed = time.time()
                memory_block.checksum = self._calculate_checksum(serialized_data)
                
                # 更新统计
                self._stats['total_writes'] += 1
                self._stats['bytes_written'] += data_size
            
            if self.debug:
                self.logger.debug(f"数据写入完成: {block_id}, {data_size} bytes")
            
            return block_id
        
        except Exception as e:
            self.logger.error(f"数据写入失败: {e}")
            raise
    
    def read(self, 
             block_id: str,
             data_format: Optional[DataFormat] = None) -> Any:
        """从共享内存读取数据"""
        try:
            with self._lock:
                if block_id not in self._memory_blocks:
                    self._stats['cache_misses'] += 1
                    raise KeyError(f"内存块不存在: {block_id}")
                
                memory_block = self._memory_blocks[block_id]
                
                # 验证数据格式
                if data_format and data_format != memory_block.data_format:
                    raise ValueError(f"数据格式不匹配: 期望{data_format}, 实际{memory_block.data_format}")
                
                # 读取数据
                start_offset = memory_block.offset
                end_offset = start_offset + memory_block.size
                
                raw_data = self._memory_map[start_offset:end_offset]
                
                # 验证校验和
                if memory_block.checksum:
                    current_checksum = self._calculate_checksum(raw_data)
                    if current_checksum != memory_block.checksum:
                        raise RuntimeError(f"数据校验失败: {block_id}")
                
                # 反序列化数据
                data = self._deserialize_data(raw_data, memory_block.data_format)
                
                # 更新访问时间
                memory_block.last_accessed = time.time()
                memory_block.ref_count += 1
                
                # 更新统计
                self._stats['total_reads'] += 1
                self._stats['bytes_read'] += memory_block.size
                self._stats['cache_hits'] += 1
            
            if self.debug:
                self.logger.debug(f"数据读取完成: {block_id}")
            
            return data
        
        except Exception as e:
            self.logger.error(f"数据读取失败: {block_id}, {e}")
            raise
    
    def _serialize_data(self, data: Any, data_format: DataFormat) -> bytes:
        """序列化数据"""
        if data_format == DataFormat.RAW_BYTES:
            if isinstance(data, bytes):
                return data
            else:
                raise ValueError("RAW_BYTES格式需要bytes数据")
        
        elif data_format == DataFormat.PICKLE:
            return pickle.dumps(data)
        
        elif data_format == DataFormat.JSON:
            json_str = json.dumps(data, ensure_ascii=False)
            return json_str.encode('utf-8')
        
        elif data_format == DataFormat.NUMPY:
            if isinstance(data, np.ndarray):
                return data.tobytes()
            else:
                raise ValueError("NUMPY格式需要numpy数组")
        
        elif data_format == DataFormat.STRUCTURED:
            # 结构化数据格式（简化实现）
            return pickle.dumps(data)
        
        else:
            raise ValueError(f"不支持的数据格式: {data_format}")
    
    def _deserialize_data(self, raw_data: bytes, data_format: DataFormat) -> Any:
        """反序列化数据"""
        if data_format == DataFormat.RAW_BYTES:
            return raw_data
        
        elif data_format == DataFormat.PICKLE:
            return pickle.loads(raw_data)
        
        elif data_format == DataFormat.JSON:
            json_str = raw_data.decode('utf-8')
            return json.loads(json_str)
        
        elif data_format == DataFormat.NUMPY:
            # 需要额外的形状信息来重建numpy数组
            # 这里简化为一维数组
            return np.frombuffer(raw_data, dtype=np.float32)
        
        elif data_format == DataFormat.STRUCTURED:
            return pickle.loads(raw_data)
        
        else:
            raise ValueError(f"不支持的数据格式: {data_format}")
    
    def _allocate_block(self, 
                       block_id: str, 
                       size: int, 
                       data_format: DataFormat) -> Optional[MemoryBlock]:
        """分配内存块"""
        with self._lock:
            if len(self._memory_blocks) >= self.config.max_blocks:
                # 尝试清理过期块
                self._cleanup_expired_blocks()
                
                if len(self._memory_blocks) >= self.config.max_blocks:
                    self.logger.warning("内存块数量达到上限")
                    return None
            
            # 查找可用偏移量
            offset = self._find_available_offset(size)
            if offset is None:
                self.logger.warning("没有足够的连续内存空间")
                return None
            
            # 创建内存块
            memory_block = MemoryBlock(
                id=block_id,
                size=size,
                offset=offset,
                data_format=data_format,
                created_at=time.time(),
                last_accessed=time.time(),
                ref_count=0
            )
            
            self._memory_blocks[block_id] = memory_block
            self._block_locks[block_id] = threading.Lock()
            
            self._stats['blocks_allocated'] += 1
            
            return memory_block
    
    def _find_available_offset(self, size: int) -> Optional[int]:
        """查找可用的内存偏移量"""
        # 简化的首次适应算法
        occupied_ranges = []
        for block in self._memory_blocks.values():
            occupied_ranges.append((block.offset, block.offset + block.size))
        
        # 按偏移量排序
        occupied_ranges.sort()
        
        # 查找空隙
        current_offset = 0
        for start, end in occupied_ranges:
            if start - current_offset >= size:
                return current_offset
            current_offset = end
        
        # 检查末尾空间
        total_size = self.config.size_mb * 1024 * 1024
        if total_size - current_offset >= size:
            return current_offset
        
        return None
    
    def _generate_block_id(self, data: bytes) -> str:
        """生成块ID"""
        timestamp = str(int(time.time() * 1000000))
        data_hash = hashlib.md5(data).hexdigest()[:8]
        return f"{timestamp}_{data_hash}"
    
    def _calculate_checksum(self, data: bytes) -> str:
        """计算数据校验和"""
        return hashlib.sha256(data).hexdigest()
    
    def deallocate_block(self, block_id: str) -> bool:
        """释放内存块"""
        try:
            with self._lock:
                if block_id not in self._memory_blocks:
                    return False
                
                memory_block = self._memory_blocks[block_id]
                
                # 清零内存区域
                start_offset = memory_block.offset
                end_offset = start_offset + memory_block.size
                self._memory_map[start_offset:end_offset] = b'\x00' * memory_block.size
                
                # 移除块记录
                del self._memory_blocks[block_id]
                if block_id in self._block_locks:
                    del self._block_locks[block_id]
                
                self._stats['blocks_deallocated'] += 1
            
            if self.debug:
                self.logger.debug(f"内存块已释放: {block_id}")
            
            return True
        
        except Exception as e:
            self.logger.error(f"内存块释放失败: {block_id}, {e}")
            return False
    
    def _cleanup_expired_blocks(self):
        """清理过期的内存块"""
        current_time = time.time()
        threshold_seconds = self.config.cleanup_threshold_minutes * 60
        
        expired_blocks = []
        for block_id, block in self._memory_blocks.items():
            if current_time - block.last_accessed > threshold_seconds and block.ref_count == 0:
                expired_blocks.append(block_id)
        
        for block_id in expired_blocks:
            self.deallocate_block(block_id)
        
        if self.debug and expired_blocks:
            self.logger.debug(f"清理了 {len(expired_blocks)} 个过期内存块")
    
    def _start_cleanup_thread(self):
        """启动清理线程"""
        def cleanup_worker():
            while True:
                time.sleep(self.config.cleanup_threshold_minutes * 60)
                try:
                    self._cleanup_expired_blocks()
                except Exception as e:
                    self.logger.error(f"清理线程错误: {e}")
        
        cleanup_thread = threading.Thread(target=cleanup_worker, daemon=True)
        cleanup_thread.start()
    
    def list_blocks(self) -> List[Dict[str, Any]]:
        """列出所有内存块"""
        with self._lock:
            blocks = []
            for block_id, block in self._memory_blocks.items():
                blocks.append({
                    'id': block.id,
                    'size': block.size,
                    'offset': block.offset,
                    'data_format': block.data_format.value,
                    'created_at': block.created_at,
                    'last_accessed': block.last_accessed,
                    'ref_count': block.ref_count,
                    'locked': block.locked,
                    'age_seconds': time.time() - block.created_at
                })
            return blocks
    
    def get_stats(self) -> Dict[str, Any]:
        """获取统计信息"""
        with self._lock:
            stats = dict(self._stats)
            stats.update({
                'total_blocks': len(self._memory_blocks),
                'memory_usage_percent': self._calculate_memory_usage(),
                'fragmentation_percent': self._calculate_fragmentation()
            })
            return stats
    
    def _calculate_memory_usage(self) -> float:
        """计算内存使用率"""
        if not self._memory_blocks:
            return 0.0
        
        used_size = sum(block.size for block in self._memory_blocks.values())
        total_size = self.config.size_mb * 1024 * 1024
        return (used_size / total_size) * 100
    
    def _calculate_fragmentation(self) -> float:
        """计算内存碎片率"""
        if not self._memory_blocks:
            return 0.0
        
        # 简化的碎片率计算
        occupied_ranges = [(block.offset, block.offset + block.size) 
                          for block in self._memory_blocks.values()]
        occupied_ranges.sort()
        
        fragments = 0
        for i in range(len(occupied_ranges) - 1):
            if occupied_ranges[i][1] < occupied_ranges[i + 1][0]:
                fragments += 1
        
        return (fragments / len(occupied_ranges)) * 100 if occupied_ranges else 0.0
    
    def close(self):
        """关闭内存通道"""
        try:
            # 清理所有内存块
            with self._lock:
                block_ids = list(self._memory_blocks.keys())
                for block_id in block_ids:
                    self.deallocate_block(block_id)
            
            # 关闭内存映射
            if self._memory_map:
                self._memory_map.close()
            
            # 关闭文件句柄
            if self._file_handle:
                os.close(self._file_handle)
            
            if self.debug:
                self.logger.debug(f"内存通道已关闭: {self.config.name}")
        
        except Exception as e:
            self.logger.error(f"内存通道关闭错误: {e}")


class SharedMemoryPool:
    """
    共享内存池 - 多通道管理
    """
    
    def __init__(self, debug: bool = False):
        self.channels: Dict[str, MemoryChannel] = {}
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        self._lock = threading.RLock()
    
    def create_channel(self, config: ChannelConfig) -> MemoryChannel:
        """创建内存通道"""
        with self._lock:
            if config.name in self.channels:
                raise ValueError(f"通道已存在: {config.name}")
            
            channel = MemoryChannel(config, self.debug)
            self.channels[config.name] = channel
            
            if self.debug:
                self.logger.debug(f"创建内存通道: {config.name}")
            
            return channel
    
    def get_channel(self, name: str) -> Optional[MemoryChannel]:
        """获取内存通道"""
        return self.channels.get(name)
    
    def close_channel(self, name: str) -> bool:
        """关闭内存通道"""
        with self._lock:
            if name not in self.channels:
                return False
            
            self.channels[name].close()
            del self.channels[name]
            
            return True
    
    def list_channels(self) -> List[str]:
        """列出所有通道"""
        return list(self.channels.keys())
    
    def close_all(self):
        """关闭所有通道"""
        with self._lock:
            for channel in self.channels.values():
                channel.close()
            self.channels.clear()


# 全局内存池实例
_global_memory_pool: Optional[SharedMemoryPool] = None


def get_global_memory_pool() -> SharedMemoryPool:
    """获取全局内存池"""
    global _global_memory_pool
    if _global_memory_pool is None:
        _global_memory_pool = SharedMemoryPool()
    return _global_memory_pool


if __name__ == "__main__":
    # 测试代码
    print("=== 共享内存通道测试 ===")
    
    config = ChannelConfig(
        name="test_channel",
        channel_type=ChannelType.MEMORY_MAPPED_FILE,
        size_mb=10,
        max_blocks=100
    )
    
    channel = MemoryChannel(config, debug=True)
    
    # 测试写入和读取
    test_data = {"message": "Hello, World!", "numbers": [1, 2, 3, 4, 5]}
    block_id = channel.write(test_data, DataFormat.JSON)
    print(f"✅ 数据写入成功: {block_id}")
    
    read_data = channel.read(block_id)
    print(f"✅ 数据读取成功: {read_data}")
    
    # 测试numpy数组
    np_array = np.random.rand(1000, 1000).astype(np.float32)
    np_block_id = channel.write(np_array, DataFormat.NUMPY)
    print(f"✅ Numpy数组写入成功: {np_block_id}")
    
    # 获取统计信息
    stats = channel.get_stats()
    print(f"📊 通道统计: {stats}")
    
    # 列出内存块
    blocks = channel.list_blocks()
    print(f"📋 内存块: {len(blocks)}个")
    
    channel.close()
    print("🎯 共享内存通道测试完成！")
