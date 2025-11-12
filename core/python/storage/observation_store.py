"""
SQLite观测存储器
基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/storage/sqlite_store.go 重新实现

功能:
- 训练观测数据持久化
- 索引优化查询性能  
- 时间序列数据管理
- 批量数据操作支持
- 统计信息和性能分析

EX-014实现: 从Go废弃代码价值提取
遵循四高原则：高规范化、高兼容性、高扩展性、高稳定性
"""

import sqlite3
import json
import time
import threading
import os
import platform
import logging
from pathlib import Path
from typing import Dict, List, Optional, Any, Tuple, Union
from dataclasses import dataclass, field, asdict
from datetime import datetime, timezone
import contextlib
from concurrent.futures import ThreadPoolExecutor, as_completed


@dataclass
class Observation:
    """观测记录（与Go版本兼容）"""
    id: Optional[int] = None               # 数据库ID，插入后自动填充
    tool: str = ""                         # 工具名称（webp, avif等）
    target_mode: str = ""                  # 目标模式（quality, balanced, size等）
    quality: int = 0                       # 质量参数
    distance: float = 0.0                  # 距离度量
    reward: float = 0.0                    # 奖励分数
    ssim: float = 0.0                      # SSIM指标
    file_size: int = 0                     # 文件大小（字节）
    timestamp: int = 0                     # Unix时间戳
    
    def __post_init__(self):
        """初始化后处理"""
        if self.timestamp == 0:
            self.timestamp = int(time.time())
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典"""
        return asdict(self)
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'Observation':
        """从字典创建"""
        return cls(**data)


@dataclass
class Statistics:
    """统计信息"""
    total: int = 0                         # 总观测数
    avg_reward: float = 0.0                # 平均奖励
    avg_ssim: float = 0.0                  # 平均SSIM
    avg_quality: float = 0.0               # 平均质量
    first_observation: int = 0             # 第一个观测时间
    last_observation: int = 0              # 最后观测时间
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典"""
        return asdict(self)


@dataclass 
class ObservationStoreConfig:
    """观测存储配置"""
    db_path: str = "data/observations.db"  # 数据库路径
    connection_timeout: float = 30.0       # 连接超时时间
    max_connections: int = 5               # 最大连接数
    enable_wal: bool = True                # 启用WAL模式
    cache_size: int = 1000                 # 缓存大小（页数）
    auto_vacuum: bool = True               # 自动清理
    sync_mode: str = "NORMAL"              # 同步模式
    
    def __post_init__(self):
        """初始化后处理"""
        # 确保数据库目录存在
        db_dir = Path(self.db_path).parent
        db_dir.mkdir(parents=True, exist_ok=True)


class ObservationStore:
    """
    SQLite观测存储器
    高规范化、高兼容性、高扩展性、高稳定性实现
    """
    
    def __init__(self, config: Optional[ObservationStoreConfig] = None, debug: bool = False):
        """
        初始化观测存储器
        
        Args:
            config: 配置对象，None使用默认配置
            debug: 调试模式
        """
        self.config = config or ObservationStoreConfig()
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        self._connections = {}  # 线程本地连接池
        self._connection_lock = threading.RLock()
        
        if debug:
            logging.basicConfig(level=logging.DEBUG)
            
        # 初始化数据库
        self._init_database()
        
        if self.debug:
            self.logger.debug(f"观测存储器初始化完成: {self.config.db_path}")
    
    def _init_database(self) -> None:
        """初始化数据库"""
        try:
            with self._get_connection() as conn:
                self._create_tables(conn)
                self._create_indexes(conn)
                self._configure_database(conn)
                
                if self.debug:
                    self.logger.debug("数据库初始化完成")
                    
        except Exception as e:
            self.logger.error(f"数据库初始化失败: {e}")
            raise RuntimeError(f"无法初始化观测存储数据库: {e}")
    
    def _create_tables(self, conn: sqlite3.Connection) -> None:
        """创建数据表"""
        schema = """
        CREATE TABLE IF NOT EXISTS observations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tool TEXT NOT NULL,
            target_mode TEXT NOT NULL,
            quality INTEGER NOT NULL,
            distance REAL NOT NULL,
            reward REAL NOT NULL,
            ssim REAL NOT NULL,
            file_size INTEGER NOT NULL,
            timestamp INTEGER NOT NULL
        );
        """
        
        conn.execute(schema)
        conn.commit()
    
    def _create_indexes(self, conn: sqlite3.Connection) -> None:
        """创建索引优化查询性能"""
        indexes = [
            "CREATE INDEX IF NOT EXISTS idx_tool_mode ON observations (tool, target_mode);",
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON observations (timestamp);",
            "CREATE INDEX IF NOT EXISTS idx_reward ON observations (reward);",
            "CREATE INDEX IF NOT EXISTS idx_tool_timestamp ON observations (tool, timestamp);"
        ]
        
        for index_sql in indexes:
            conn.execute(index_sql)
        
        conn.commit()
        
        if self.debug:
            self.logger.debug("数据库索引创建完成")
    
    def _configure_database(self, conn: sqlite3.Connection) -> None:
        """配置数据库性能参数"""
        configurations = [
            f"PRAGMA cache_size = {self.config.cache_size};",
            f"PRAGMA synchronous = {self.config.sync_mode};",
            f"PRAGMA auto_vacuum = {'FULL' if self.config.auto_vacuum else 'NONE'};",
            "PRAGMA temp_store = MEMORY;",
            "PRAGMA mmap_size = 268435456;",  # 256MB内存映射
        ]
        
        # 启用WAL模式提高并发性能
        if self.config.enable_wal:
            configurations.append("PRAGMA journal_mode = WAL;")
        
        for config_sql in configurations:
            conn.execute(config_sql)
            
        conn.commit()
    
    @contextlib.contextmanager
    def _get_connection(self):
        """获取数据库连接（线程安全）"""
        thread_id = threading.get_ident()
        
        with self._connection_lock:
            if thread_id not in self._connections:
                try:
                    conn = sqlite3.connect(
                        self.config.db_path,
                        timeout=self.config.connection_timeout,
                        check_same_thread=False
                    )
                    conn.row_factory = sqlite3.Row  # 启用字典式访问
                    self._connections[thread_id] = conn
                    
                    if self.debug:
                        self.logger.debug(f"为线程{thread_id}创建新数据库连接")
                        
                except sqlite3.Error as e:
                    raise RuntimeError(f"无法连接到数据库: {e}")
            
            conn = self._connections[thread_id]
        
        try:
            yield conn
        except Exception as e:
            conn.rollback()
            raise e
    
    def save(self, observation: Observation) -> int:
        """
        保存观测记录
        
        Args:
            observation: 观测记录
            
        Returns:
            int: 插入的记录ID
        """
        try:
            with self._get_connection() as conn:
                cursor = conn.cursor()
                
                query = """
                INSERT INTO observations 
                (tool, target_mode, quality, distance, reward, ssim, file_size, timestamp)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                """
                
                cursor.execute(query, (
                    observation.tool,
                    observation.target_mode,
                    observation.quality,
                    observation.distance,
                    observation.reward,
                    observation.ssim,
                    observation.file_size,
                    observation.timestamp or int(time.time())
                ))
                
                observation_id = cursor.lastrowid
                conn.commit()
                
                # 更新观测记录的ID
                observation.id = observation_id
                
                if self.debug:
                    self.logger.debug(f"保存观测记录: ID={observation_id}, "
                                    f"工具={observation.tool}, 模式={observation.target_mode}")
                
                return observation_id
                
        except sqlite3.Error as e:
            self.logger.error(f"保存观测记录失败: {e}")
            raise RuntimeError(f"无法保存观测记录: {e}")
    
    def save_batch(self, observations: List[Observation]) -> List[int]:
        """
        批量保存观测记录
        
        Args:
            observations: 观测记录列表
            
        Returns:
            List[int]: 插入的记录ID列表
        """
        if not observations:
            return []
            
        try:
            with self._get_connection() as conn:
                cursor = conn.cursor()
                
                query = """
                INSERT INTO observations 
                (tool, target_mode, quality, distance, reward, ssim, file_size, timestamp)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                """
                
                # 准备批量插入数据
                data = []
                for obs in observations:
                    data.append((
                        obs.tool,
                        obs.target_mode,
                        obs.quality,
                        obs.distance,
                        obs.reward,
                        obs.ssim,
                        obs.file_size,
                        obs.timestamp or int(time.time())
                    ))
                
                # 批量执行
                cursor.executemany(query, data)
                
                # 获取插入的ID范围
                last_id = cursor.lastrowid
                first_id = last_id - len(observations) + 1
                inserted_ids = list(range(first_id, last_id + 1))
                
                # 更新观测记录的ID
                for i, obs in enumerate(observations):
                    obs.id = inserted_ids[i]
                
                conn.commit()
                
                if self.debug:
                    self.logger.debug(f"批量保存{len(observations)}条观测记录")
                
                return inserted_ids
                
        except sqlite3.Error as e:
            self.logger.error(f"批量保存观测记录失败: {e}")
            raise RuntimeError(f"无法批量保存观测记录: {e}")
    
    def load(self, tool: str, target_mode: str, limit: int = 1000) -> List[Observation]:
        """
        加载观测记录（按工具和模式过滤，最新优先）
        
        Args:
            tool: 工具名称
            target_mode: 目标模式
            limit: 限制数量
            
        Returns:
            List[Observation]: 观测记录列表
        """
        try:
            with self._get_connection() as conn:
                cursor = conn.cursor()
                
                query = """
                SELECT id, tool, target_mode, quality, distance, reward, ssim, file_size, timestamp
                FROM observations
                WHERE tool = ? AND target_mode = ?
                ORDER BY timestamp DESC
                LIMIT ?
                """
                
                cursor.execute(query, (tool, target_mode, limit))
                rows = cursor.fetchall()
                
                observations = []
                for row in rows:
                    obs = Observation(
                        id=row['id'],
                        tool=row['tool'],
                        target_mode=row['target_mode'],
                        quality=row['quality'],
                        distance=row['distance'],
                        reward=row['reward'],
                        ssim=row['ssim'],
                        file_size=row['file_size'],
                        timestamp=row['timestamp']
                    )
                    observations.append(obs)
                
                if self.debug:
                    self.logger.debug(f"加载{len(observations)}条观测记录: "
                                    f"工具={tool}, 模式={target_mode}")
                
                return observations
                
        except sqlite3.Error as e:
            self.logger.error(f"加载观测记录失败: {e}")
            raise RuntimeError(f"无法加载观测记录: {e}")
    
    def count(self, tool: str, target_mode: str) -> int:
        """
        统计观测数量
        
        Args:
            tool: 工具名称
            target_mode: 目标模式
            
        Returns:
            int: 观测数量
        """
        try:
            with self._get_connection() as conn:
                cursor = conn.cursor()
                
                query = """
                SELECT COUNT(*) as count
                FROM observations
                WHERE tool = ? AND target_mode = ?
                """
                
                cursor.execute(query, (tool, target_mode))
                result = cursor.fetchone()
                
                count = result['count'] if result else 0
                
                if self.debug:
                    self.logger.debug(f"观测数量统计: {count}条 "
                                    f"(工具={tool}, 模式={target_mode})")
                
                return count
                
        except sqlite3.Error as e:
            self.logger.error(f"统计观测数量失败: {e}")
            raise RuntimeError(f"无法统计观测数量: {e}")
    
    def get_statistics(self, tool: str, target_mode: str) -> Statistics:
        """
        获取统计信息
        
        Args:
            tool: 工具名称
            target_mode: 目标模式
            
        Returns:
            Statistics: 统计信息
        """
        try:
            with self._get_connection() as conn:
                cursor = conn.cursor()
                
                query = """
                SELECT
                    COUNT(*) as total,
                    COALESCE(AVG(reward), 0) as avg_reward,
                    COALESCE(AVG(ssim), 0) as avg_ssim,
                    COALESCE(AVG(quality), 0) as avg_quality,
                    COALESCE(MIN(timestamp), 0) as first_observation,
                    COALESCE(MAX(timestamp), 0) as last_observation
                FROM observations
                WHERE tool = ? AND target_mode = ?
                """
                
                cursor.execute(query, (tool, target_mode))
                result = cursor.fetchone()
                
                if result and result['total'] > 0:
                    stats = Statistics(
                        total=result['total'],
                        avg_reward=float(result['avg_reward']),
                        avg_ssim=float(result['avg_ssim']),
                        avg_quality=float(result['avg_quality']),
                        first_observation=result['first_observation'],
                        last_observation=result['last_observation']
                    )
                else:
                    stats = Statistics()
                
                if self.debug:
                    self.logger.debug(f"统计信息: {stats.total}条观测, "
                                    f"平均奖励={stats.avg_reward:.3f}")
                
                return stats
                
        except sqlite3.Error as e:
            self.logger.error(f"获取统计信息失败: {e}")
            raise RuntimeError(f"无法获取统计信息: {e}")
    
    def get_top_performing(self, tool: str, target_mode: str, limit: int = 10) -> List[Observation]:
        """
        获取表现最佳的N个观测
        
        Args:
            tool: 工具名称
            target_mode: 目标模式
            limit: 限制数量
            
        Returns:
            List[Observation]: 表现最佳的观测记录
        """
        try:
            with self._get_connection() as conn:
                cursor = conn.cursor()
                
                query = """
                SELECT id, tool, target_mode, quality, distance, reward, ssim, file_size, timestamp
                FROM observations
                WHERE tool = ? AND target_mode = ?
                ORDER BY reward DESC
                LIMIT ?
                """
                
                cursor.execute(query, (tool, target_mode, limit))
                rows = cursor.fetchall()
                
                observations = []
                for row in rows:
                    obs = Observation(
                        id=row['id'],
                        tool=row['tool'],
                        target_mode=row['target_mode'],
                        quality=row['quality'],
                        distance=row['distance'],
                        reward=row['reward'],
                        ssim=row['ssim'],
                        file_size=row['file_size'],
                        timestamp=row['timestamp']
                    )
                    observations.append(obs)
                
                if self.debug:
                    self.logger.debug(f"获取{len(observations)}条最佳观测记录")
                
                return observations
                
        except sqlite3.Error as e:
            self.logger.error(f"获取最佳观测记录失败: {e}")
            raise RuntimeError(f"无法获取最佳观测记录: {e}")
    
    def clear_old(self, tool: str, older_than: int) -> int:
        """
        清除旧观测记录
        
        Args:
            tool: 工具名称
            older_than: Unix时间戳，删除比此时间更早的记录
            
        Returns:
            int: 删除的记录数
        """
        try:
            with self._get_connection() as conn:
                cursor = conn.cursor()
                
                query = """
                DELETE FROM observations
                WHERE tool = ? AND timestamp < ?
                """
                
                cursor.execute(query, (tool, older_than))
                deleted_count = cursor.rowcount
                conn.commit()
                
                if self.debug:
                    self.logger.debug(f"清除{deleted_count}条旧观测记录: 工具={tool}")
                
                return deleted_count
                
        except sqlite3.Error as e:
            self.logger.error(f"清除旧观测记录失败: {e}")
            raise RuntimeError(f"无法清除旧观测记录: {e}")
    
    def get_database_info(self) -> Dict[str, Any]:
        """获取数据库信息"""
        try:
            with self._get_connection() as conn:
                cursor = conn.cursor()
                
                # 基本信息
                cursor.execute("SELECT COUNT(*) as total FROM observations")
                total_records = cursor.fetchone()['total']
                
                # 表大小信息
                cursor.execute("PRAGMA page_count")
                page_count = cursor.fetchone()[0]
                
                cursor.execute("PRAGMA page_size")
                page_size = cursor.fetchone()[0]
                
                db_size = page_count * page_size
                
                # WAL模式信息
                cursor.execute("PRAGMA journal_mode")
                journal_mode = cursor.fetchone()[0]
                
                # 工具统计
                cursor.execute("""
                SELECT tool, COUNT(*) as count
                FROM observations
                GROUP BY tool
                ORDER BY count DESC
                """)
                tool_stats = {row['tool']: row['count'] for row in cursor.fetchall()}
                
                info = {
                    'database_path': self.config.db_path,
                    'total_records': total_records,
                    'database_size_bytes': db_size,
                    'database_size_mb': round(db_size / (1024 * 1024), 2),
                    'journal_mode': journal_mode,
                    'tool_statistics': tool_stats,
                    'config': asdict(self.config)
                }
                
                return info
                
        except sqlite3.Error as e:
            self.logger.error(f"获取数据库信息失败: {e}")
            return {'error': str(e)}
    
    def close(self) -> None:
        """关闭所有数据库连接"""
        with self._connection_lock:
            for thread_id, conn in self._connections.items():
                try:
                    conn.close()
                    if self.debug:
                        self.logger.debug(f"关闭线程{thread_id}的数据库连接")
                except Exception as e:
                    self.logger.error(f"关闭数据库连接失败: {e}")
            
            self._connections.clear()
    
    def __enter__(self):
        """上下文管理器入口"""
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        """上下文管理器出口"""
        self.close()


# 全局存储实例管理
_global_store: Optional[ObservationStore] = None
_global_store_lock = threading.RLock()


def get_observation_store(config: Optional[ObservationStoreConfig] = None) -> ObservationStore:
    """
    获取全局观测存储实例（单例模式）
    
    Args:
        config: 配置对象，仅在首次调用时有效
        
    Returns:
        ObservationStore: 存储实例
    """
    global _global_store
    
    with _global_store_lock:
        if _global_store is None:
            _global_store = ObservationStore(config)
        
        return _global_store


# 便捷函数
def save_observation(tool: str, target_mode: str, quality: int, distance: float,
                    reward: float, ssim: float, file_size: int,
                    config: Optional[ObservationStoreConfig] = None) -> int:
    """
    便捷函数：保存观测记录
    
    Returns:
        int: 插入的记录ID
    """
    store = get_observation_store(config)
    observation = Observation(
        tool=tool,
        target_mode=target_mode,
        quality=quality,
        distance=distance,
        reward=reward,
        ssim=ssim,
        file_size=file_size
    )
    return store.save(observation)


def load_observations(tool: str, target_mode: str, limit: int = 100,
                     config: Optional[ObservationStoreConfig] = None) -> List[Observation]:
    """
    便捷函数：加载观测记录
    
    Returns:
        List[Observation]: 观测记录列表
    """
    store = get_observation_store(config)
    return store.load(tool, target_mode, limit)


def get_observation_statistics(tool: str, target_mode: str,
                              config: Optional[ObservationStoreConfig] = None) -> Statistics:
    """
    便捷函数：获取观测统计信息
    
    Returns:
        Statistics: 统计信息
    """
    store = get_observation_store(config)
    return store.get_statistics(tool, target_mode)


if __name__ == "__main__":
    # 测试代码
    import random
    
    print("=== SQLite观测存储器测试 ===")
    
    # 创建测试配置
    config = ObservationStoreConfig(
        db_path="test_observations.db",
        enable_wal=True
    )
    
    # 测试存储器
    with ObservationStore(config, debug=True) as store:
        print(f"✅ 存储器初始化成功")
        
        # 测试保存观测
        test_obs = Observation(
            tool="webp",
            target_mode="balanced",
            quality=80,
            distance=0.1,
            reward=0.85,
            ssim=0.92,
            file_size=1024000
        )
        
        obs_id = store.save(test_obs)
        print(f"✅ 保存观测记录，ID: {obs_id}")
        
        # 批量保存测试数据
        test_observations = []
        for i in range(10):
            obs = Observation(
                tool=random.choice(["webp", "avif", "jpeg"]),
                target_mode=random.choice(["quality", "balanced", "size"]),
                quality=random.randint(60, 95),
                distance=random.uniform(0.01, 0.5),
                reward=random.uniform(0.5, 1.0),
                ssim=random.uniform(0.8, 0.99),
                file_size=random.randint(50000, 2000000)
            )
            test_observations.append(obs)
        
        ids = store.save_batch(test_observations)
        print(f"✅ 批量保存{len(ids)}条观测记录")
        
        # 测试加载和统计
        loaded = store.load("webp", "balanced", 5)
        print(f"✅ 加载{len(loaded)}条观测记录")
        
        count = store.count("webp", "balanced")
        print(f"✅ 观测数量统计: {count}")
        
        stats = store.get_statistics("webp", "balanced")
        print(f"✅ 统计信息: 总计{stats.total}条, 平均奖励{stats.avg_reward:.3f}")
        
        # 测试最佳表现
        top_performing = store.get_top_performing("webp", "balanced", 3)
        print(f"✅ 获取{len(top_performing)}条最佳观测")
        
        # 数据库信息
        db_info = store.get_database_info()
        print(f"✅ 数据库大小: {db_info.get('database_size_mb', 0):.2f} MB")
        print(f"   总记录数: {db_info.get('total_records', 0)}")
        
    print("🎯 所有测试完成！")
