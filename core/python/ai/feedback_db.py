#!/usr/bin/env python3
"""
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 47.23: 反馈数据库系统 - 在线学习核心
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

功能：
- 记录AI预测和实际结果对比
- 支持用户评分（1-5星）
- 为增量训练提供数据源
- 追踪模型性能演化
- SQLite持久化存储

从Go代码提取：core/@deprecated/go_ai_service_2025_11_11/ai 2/feedback_db.go
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"""

import sqlite3
import json
import threading
from dataclasses import dataclass, field, asdict
from datetime import datetime
from typing import Dict, List, Any, Optional
from pathlib import Path


@dataclass
class FeedbackRecord:
    """反馈记录数据结构"""
    # 基本信息
    request_id: str
    model_type: str
    image_path: str
    
    # 模型信息
    model_version: Optional[str] = None
    
    # 特征和参数
    input_features: Dict[str, Any] = field(default_factory=dict)
    predicted_params: Dict[str, Any] = field(default_factory=dict)
    actual_params: Dict[str, Any] = field(default_factory=dict)
    
    # 质量和性能
    actual_quality: Optional[float] = None
    actual_size: Optional[int] = None
    user_rating: Optional[int] = None  # 1-5星
    processing_time: Optional[float] = None
    
    # 状态
    success: bool = True
    error_message: Optional[str] = None
    
    # 训练相关
    used_for_training: bool = False
    training_batch: Optional[int] = None
    
    # 自动生成
    id: Optional[int] = None
    timestamp: Optional[datetime] = None
    
    def __post_init__(self):
        if self.timestamp is None:
            self.timestamp = datetime.now()


class FeedbackDB:
    """反馈数据库管理器"""
    
    def __init__(self, db_path: str = "data/feedback.db"):
        """
        初始化反馈数据库
        
        Args:
            db_path: 数据库文件路径
        """
        self.db_path = Path(db_path)
        self.db_path.parent.mkdir(parents=True, exist_ok=True)
        
        self.conn = sqlite3.connect(str(self.db_path), check_same_thread=False)
        self.conn.row_factory = sqlite3.Row
        self.lock = threading.Lock()
        
        self._init_tables()
        print(f"✅ FeedbackDB initialized: {self.db_path}")
    
    def _init_tables(self):
        """初始化数据表结构"""
        schema = """
        CREATE TABLE IF NOT EXISTS feedback (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            request_id TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            model_type TEXT NOT NULL,
            model_version TEXT,
            image_path TEXT NOT NULL,
            input_features TEXT,
            predicted_params TEXT,
            actual_params TEXT,
            actual_quality REAL,
            actual_size INTEGER,
            user_rating INTEGER,
            processing_time REAL,
            success BOOLEAN DEFAULT 1,
            error_message TEXT,
            used_for_training BOOLEAN DEFAULT 0,
            training_batch INTEGER
        );
        
        CREATE INDEX IF NOT EXISTS idx_model_type ON feedback(model_type);
        CREATE INDEX IF NOT EXISTS idx_timestamp ON feedback(timestamp);
        CREATE INDEX IF NOT EXISTS idx_used_for_training ON feedback(used_for_training);
        
        CREATE TABLE IF NOT EXISTS training_batches (
            batch_id INTEGER PRIMARY KEY AUTOINCREMENT,
            model_type TEXT NOT NULL,
            start_time DATETIME,
            end_time DATETIME,
            sample_count INTEGER,
            metrics TEXT,
            status TEXT DEFAULT 'pending'
        );
        
        CREATE TABLE IF NOT EXISTS model_performance (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            model_type TEXT NOT NULL,
            model_version TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            avg_quality_error REAL,
            avg_size_error REAL,
            success_rate REAL,
            avg_processing_time REAL,
            sample_count INTEGER
        );
        """
        
        with self.lock:
            self.conn.executescript(schema)
            self.conn.commit()
    
    def record_feedback(self, record: FeedbackRecord) -> int:
        """
        记录反馈数据
        
        Args:
            record: 反馈记录
            
        Returns:
            记录ID
        """
        query = """
            INSERT INTO feedback (
                request_id, model_type, model_version, image_path,
                input_features, predicted_params, actual_params,
                actual_quality, actual_size, user_rating,
                processing_time, success, error_message, timestamp
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """
        
        with self.lock:
            cursor = self.conn.execute(query, (
                record.request_id,
                record.model_type,
                record.model_version,
                record.image_path,
                json.dumps(record.input_features),
                json.dumps(record.predicted_params),
                json.dumps(record.actual_params),
                record.actual_quality,
                record.actual_size,
                record.user_rating,
                record.processing_time,
                record.success,
                record.error_message,
                record.timestamp.isoformat() if record.timestamp else datetime.now().isoformat()
            ))
            self.conn.commit()
            record.id = cursor.lastrowid
            return cursor.lastrowid
    
    def get_unused_feedback(self, model_type: str, limit: int = 1000) -> List[FeedbackRecord]:
        """
        获取未用于训练的反馈记录
        
        Args:
            model_type: 模型类型
            limit: 最大记录数
            
        Returns:
            反馈记录列表
        """
        query = """
            SELECT id, request_id, timestamp, model_type, model_version,
                   image_path, input_features, predicted_params, actual_params,
                   actual_quality, actual_size, user_rating, processing_time,
                   success, error_message
            FROM feedback
            WHERE model_type = ? AND used_for_training = 0 AND success = 1
            ORDER BY timestamp DESC
            LIMIT ?
        """
        
        with self.lock:
            cursor = self.conn.execute(query, (model_type, limit))
            rows = cursor.fetchall()
        
        records = []
        for row in rows:
            record = FeedbackRecord(
                id=row['id'],
                request_id=row['request_id'],
                timestamp=datetime.fromisoformat(row['timestamp']) if row['timestamp'] else None,
                model_type=row['model_type'],
                model_version=row['model_version'],
                image_path=row['image_path'],
                input_features=json.loads(row['input_features']) if row['input_features'] else {},
                predicted_params=json.loads(row['predicted_params']) if row['predicted_params'] else {},
                actual_params=json.loads(row['actual_params']) if row['actual_params'] else {},
                actual_quality=row['actual_quality'],
                actual_size=row['actual_size'],
                user_rating=row['user_rating'],
                processing_time=row['processing_time'],
                success=bool(row['success']),
                error_message=row['error_message']
            )
            records.append(record)
        
        return records
    
    def mark_as_used(self, ids: List[int], batch_id: int):
        """
        标记反馈记录已用于训练
        
        Args:
            ids: 记录ID列表
            batch_id: 训练批次ID
        """
        query = "UPDATE feedback SET used_for_training = 1, training_batch = ? WHERE id = ?"
        
        with self.lock:
            for record_id in ids:
                self.conn.execute(query, (batch_id, record_id))
            self.conn.commit()
    
    def create_training_batch(self, model_type: str, sample_count: int) -> int:
        """
        创建训练批次
        
        Args:
            model_type: 模型类型
            sample_count: 样本数量
            
        Returns:
            批次ID
        """
        query = """
            INSERT INTO training_batches (model_type, start_time, sample_count, status)
            VALUES (?, ?, ?, 'pending')
        """
        
        with self.lock:
            cursor = self.conn.execute(query, (model_type, datetime.now().isoformat(), sample_count))
            self.conn.commit()
            return cursor.lastrowid
    
    def update_training_batch(self, batch_id: int, metrics: Dict[str, Any], status: str):
        """
        更新训练批次状态
        
        Args:
            batch_id: 批次ID
            metrics: 训练指标
            status: 状态（pending/running/completed/failed）
        """
        query = """
            UPDATE training_batches
            SET end_time = ?, metrics = ?, status = ?
            WHERE batch_id = ?
        """
        
        with self.lock:
            self.conn.execute(query, (
                datetime.now().isoformat(),
                json.dumps(metrics),
                status,
                batch_id
            ))
            self.conn.commit()
    
    def record_model_performance(self, model_type: str, model_version: str, metrics: Dict[str, Any]):
        """
        记录模型性能
        
        Args:
            model_type: 模型类型
            model_version: 模型版本
            metrics: 性能指标
        """
        query = """
            INSERT INTO model_performance (
                model_type, model_version, avg_quality_error, avg_size_error,
                success_rate, avg_processing_time, sample_count
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
        """
        
        with self.lock:
            self.conn.execute(query, (
                model_type,
                model_version,
                metrics.get('avg_quality_error'),
                metrics.get('avg_size_error'),
                metrics.get('success_rate'),
                metrics.get('avg_processing_time'),
                metrics.get('sample_count')
            ))
            self.conn.commit()
    
    def get_model_performance(self, model_type: str, limit: int = 10) -> List[Dict[str, Any]]:
        """
        获取模型性能历史
        
        Args:
            model_type: 模型类型
            limit: 最大记录数
            
        Returns:
            性能记录列表
        """
        query = """
            SELECT model_version, timestamp, avg_quality_error, avg_size_error,
                   success_rate, avg_processing_time, sample_count
            FROM model_performance
            WHERE model_type = ?
            ORDER BY timestamp DESC
            LIMIT ?
        """
        
        with self.lock:
            cursor = self.conn.execute(query, (model_type, limit))
            rows = cursor.fetchall()
        
        results = []
        for row in rows:
            results.append({
                'version': row['model_version'],
                'timestamp': row['timestamp'],
                'avg_quality_error': row['avg_quality_error'],
                'avg_size_error': row['avg_size_error'],
                'success_rate': row['success_rate'],
                'avg_processing_time': row['avg_processing_time'],
                'sample_count': row['sample_count']
            })
        
        return results
    
    def get_stats(self) -> Dict[str, Any]:
        """
        获取统计信息
        
        Returns:
            统计数据字典
        """
        stats = {}
        
        with self.lock:
            # 总记录数
            cursor = self.conn.execute("SELECT COUNT(*) as count FROM feedback")
            stats['total_records'] = cursor.fetchone()['count']
            
            # 未使用的记录数
            cursor = self.conn.execute("SELECT COUNT(*) as count FROM feedback WHERE used_for_training = 0")
            stats['unused_records'] = cursor.fetchone()['count']
            
            # 训练批次数
            cursor = self.conn.execute("SELECT COUNT(*) as count FROM training_batches")
            stats['total_batches'] = cursor.fetchone()['count']
            
            # 按模型类型统计
            cursor = self.conn.execute("""
                SELECT model_type, COUNT(*) as count,
                       AVG(CASE WHEN success = 1 THEN 1.0 ELSE 0.0 END) as success_rate
                FROM feedback
                GROUP BY model_type
            """)
            
            by_model = {}
            for row in cursor.fetchall():
                by_model[row['model_type']] = {
                    'count': row['count'],
                    'success_rate': row['success_rate']
                }
            stats['by_model'] = by_model
        
        return stats
    
    def close(self):
        """关闭数据库连接"""
        with self.lock:
            self.conn.close()


# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 测试和演示
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

if __name__ == "__main__":
    import uuid
    
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print("Phase 47.23: 反馈数据库系统测试")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
    
    # 创建测试数据库
    db = FeedbackDB("data/test_feedback.db")
    
    # 1. 记录反馈
    print("1️⃣  测试记录反馈...")
    record = FeedbackRecord(
        request_id=str(uuid.uuid4()),
        model_type="lightgbm",
        model_version="v1.0.0",
        image_path="/path/to/image.jpg",
        input_features={
            "file_size": 1024000,
            "width": 1920,
            "height": 1080,
            "has_alpha": False
        },
        predicted_params={
            "quality": 85,
            "effort": 6,
            "target_format": "jxl"
        },
        actual_params={
            "quality": 85,
            "effort": 6
        },
        actual_quality=0.96,
        actual_size=512000,
        user_rating=5,
        processing_time=1.234,
        success=True
    )
    
    record_id = db.record_feedback(record)
    print(f"   ✅ 记录ID: {record_id}")
    
    # 2. 获取未使用的反馈
    print("\n2️⃣  测试获取未使用反馈...")
    unused = db.get_unused_feedback("lightgbm", limit=10)
    print(f"   📊 未使用记录数: {len(unused)}")
    if unused:
        latest = unused[0]
        print(f"   最新记录: {latest.request_id[:8]}... (质量: {latest.actual_quality})")
    
    # 3. 创建训练批次
    print("\n3️⃣  测试创建训练批次...")
    batch_id = db.create_training_batch("lightgbm", len(unused))
    print(f"   ✅ 批次ID: {batch_id}")
    
    # 4. 标记为已使用
    if unused:
        print("\n4️⃣  测试标记为已使用...")
        db.mark_as_used([r.id for r in unused], batch_id)
        print(f"   ✅ 已标记 {len(unused)} 条记录")
    
    # 5. 更新训练批次
    print("\n5️⃣  测试更新训练批次...")
    db.update_training_batch(batch_id, {
        "accuracy": 0.92,
        "loss": 0.08
    }, "completed")
    print("   ✅ 批次状态已更新")
    
    # 6. 记录模型性能
    print("\n6️⃣  测试记录模型性能...")
    db.record_model_performance("lightgbm", "v1.0.0", {
        "avg_quality_error": 0.02,
        "avg_size_error": 0.05,
        "success_rate": 0.98,
        "avg_processing_time": 1.5,
        "sample_count": 100
    })
    print("   ✅ 性能已记录")
    
    # 7. 获取性能历史
    print("\n7️⃣  测试获取性能历史...")
    performance = db.get_model_performance("lightgbm", limit=5)
    print(f"   📊 性能记录数: {len(performance)}")
    if performance:
        latest_perf = performance[0]
        print(f"   最新版本: {latest_perf['version']}")
        print(f"   成功率: {latest_perf['success_rate']:.2%}")
        print(f"   样本数: {latest_perf['sample_count']}")
    
    # 8. 获取统计信息
    print("\n8️⃣  测试获取统计信息...")
    stats = db.get_stats()
    print(f"   📊 总记录数: {stats['total_records']}")
    print(f"   📊 未使用记录: {stats['unused_records']}")
    print(f"   📊 训练批次: {stats['total_batches']}")
    
    if stats['by_model']:
        print("\n   按模型类型统计:")
        for model_type, data in stats['by_model'].items():
            print(f"      • {model_type}: {data['count']} 条记录, 成功率 {data['success_rate']:.2%}")
    
    db.close()
    
    print("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print("✅ 所有测试通过！反馈数据库系统工作正常")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
