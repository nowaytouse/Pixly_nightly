"""
🧠 PIXLY v3.0 本地化知识库系统

替代Go knowledge包的完整实现：
- 转换记录追踪与分析
- AI预测准确性统计
- 异常案例检测与学习
- 格式特征智能分析
- 用户反馈集成优化

完全本地化，基于SQLite存储
"""

import sqlite3
import json
import time
import threading
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, asdict
from enum import Enum
import statistics

from .local_dispatcher import LocalPredictionRequest, LocalPredictionResult


@dataclass
class ConversionRecord:
    """转换记录 - 替代Go ConversionRecord"""
    id: int = 0
    created_at: datetime = None
    
    # 文件信息
    file_path: str = ""
    file_name: str = ""
    original_format: str = ""
    original_size: int = 0
    
    # 文件特征
    width: int = 0
    height: int = 0
    has_alpha: bool = False
    pix_fmt: str = ""
    is_animated: bool = False
    frame_count: int = 0
    estimated_quality: int = 0
    
    # 预测信息
    predictor_name: str = ""
    prediction_rule: str = ""
    prediction_confidence: float = 0.0
    prediction_time_ms: int = 0
    
    # 预测参数
    predicted_format: str = ""
    predicted_lossless: bool = False
    predicted_distance: float = 0.0
    predicted_effort: int = 0
    predicted_saving_percent: float = 0.0
    predicted_output_size: int = 0
    
    # 实际结果
    actual_format: str = ""
    actual_output_size: int = 0
    actual_conversion_time_ms: int = 0
    actual_saving_percent: float = 0.0
    actual_saving_bytes: int = 0
    
    # 预测准确性
    prediction_error_percent: float = 0.0
    was_explored: bool = False
    
    # 用户反馈
    user_rating: int = 0
    user_comment: str = ""
    
    def __post_init__(self):
        if self.created_at is None:
            self.created_at = datetime.now()


@dataclass  
class PredictionStats:
    """预测准确性统计"""
    id: int = 0
    predictor_name: str = ""
    prediction_rule: str = ""
    original_format: str = ""
    
    stats_from: datetime = None
    stats_to: datetime = None
    
    total_conversions: int = 0
    successful_conversions: int = 0
    
    avg_prediction_error_percent: float = 0.0
    median_prediction_error_percent: float = 0.0
    std_prediction_error_percent: float = 0.0
    
    avg_predicted_saving: float = 0.0
    avg_actual_saving: float = 0.0
    
    perfect_quality_count: int = 0
    good_quality_count: int = 0
    
    avg_conversion_time_ms: int = 0
    updated_at: datetime = None


class LocalKnowledgeDatabase:
    """
    🧠 本地化知识库数据库
    
    替代Go Database的SQLite实现
    """
    
    def __init__(self, db_path: str = "knowledge.db"):
        self.db_path = Path(db_path)
        self.db_path.parent.mkdir(parents=True, exist_ok=True)
        self._lock = threading.RLock()
        
        # 初始化数据库
        self._init_database()
        print(f"✅ 知识库数据库已初始化: {self.db_path}")
    
    def _init_database(self):
        """初始化数据库schema"""
        with sqlite3.connect(self.db_path) as conn:
            # 转换记录表
            conn.execute("""
                CREATE TABLE IF NOT EXISTS conversion_records (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    
                    -- 文件信息
                    file_path TEXT NOT NULL,
                    file_name TEXT NOT NULL,
                    original_format TEXT NOT NULL,
                    original_size INTEGER NOT NULL,
                    
                    -- 文件特征
                    width INTEGER DEFAULT 0,
                    height INTEGER DEFAULT 0,
                    has_alpha BOOLEAN DEFAULT FALSE,
                    pix_fmt TEXT DEFAULT '',
                    is_animated BOOLEAN DEFAULT FALSE,
                    frame_count INTEGER DEFAULT 0,
                    estimated_quality INTEGER DEFAULT 0,
                    
                    -- 预测信息
                    predictor_name TEXT NOT NULL,
                    prediction_rule TEXT NOT NULL,
                    prediction_confidence REAL DEFAULT 0.0,
                    prediction_time_ms INTEGER DEFAULT 0,
                    
                    -- 预测参数
                    predicted_format TEXT DEFAULT '',
                    predicted_lossless BOOLEAN DEFAULT FALSE,
                    predicted_distance REAL DEFAULT 0.0,
                    predicted_effort INTEGER DEFAULT 0,
                    predicted_saving_percent REAL DEFAULT 0.0,
                    predicted_output_size INTEGER DEFAULT 0,
                    
                    -- 实际结果
                    actual_format TEXT DEFAULT '',
                    actual_output_size INTEGER DEFAULT 0,
                    actual_conversion_time_ms INTEGER DEFAULT 0,
                    actual_saving_percent REAL DEFAULT 0.0,
                    actual_saving_bytes INTEGER DEFAULT 0,
                    
                    -- 预测准确性
                    prediction_error_percent REAL DEFAULT 0.0,
                    was_explored BOOLEAN DEFAULT FALSE,
                    
                    -- 用户反馈
                    user_rating INTEGER DEFAULT 0,
                    user_comment TEXT DEFAULT ''
                )
            """)
            
            # 预测统计表
            conn.execute("""
                CREATE TABLE IF NOT EXISTS prediction_stats (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    predictor_name TEXT NOT NULL,
                    prediction_rule TEXT NOT NULL,
                    original_format TEXT NOT NULL,
                    
                    stats_from TIMESTAMP,
                    stats_to TIMESTAMP,
                    
                    total_conversions INTEGER DEFAULT 0,
                    successful_conversions INTEGER DEFAULT 0,
                    
                    avg_prediction_error_percent REAL DEFAULT 0.0,
                    median_prediction_error_percent REAL DEFAULT 0.0,
                    std_prediction_error_percent REAL DEFAULT 0.0,
                    
                    avg_predicted_saving REAL DEFAULT 0.0,
                    avg_actual_saving REAL DEFAULT 0.0,
                    
                    perfect_quality_count INTEGER DEFAULT 0,
                    good_quality_count INTEGER DEFAULT 0,
                    
                    avg_conversion_time_ms INTEGER DEFAULT 0,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    
                    UNIQUE(predictor_name, prediction_rule, original_format)
                )
            """)
            
            # 创建索引
            conn.execute("CREATE INDEX IF NOT EXISTS idx_records_created ON conversion_records(created_at)")
            conn.execute("CREATE INDEX IF NOT EXISTS idx_records_predictor ON conversion_records(predictor_name, prediction_rule)")
            conn.execute("CREATE INDEX IF NOT EXISTS idx_records_format ON conversion_records(original_format)")
            
            conn.commit()
    
    def save_record(self, record: ConversionRecord) -> int:
        """保存转换记录"""
        with self._lock:
            with sqlite3.connect(self.db_path) as conn:
                cursor = conn.execute("""
                    INSERT INTO conversion_records (
                        created_at, file_path, file_name, original_format, original_size,
                        width, height, has_alpha, pix_fmt, is_animated, frame_count, estimated_quality,
                        predictor_name, prediction_rule, prediction_confidence, prediction_time_ms,
                        predicted_format, predicted_lossless, predicted_distance, predicted_effort,
                        predicted_saving_percent, predicted_output_size,
                        actual_format, actual_output_size, actual_conversion_time_ms,
                        actual_saving_percent, actual_saving_bytes,
                        prediction_error_percent, was_explored,
                        user_rating, user_comment
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                """, (
                    record.created_at, record.file_path, record.file_name, record.original_format, record.original_size,
                    record.width, record.height, record.has_alpha, record.pix_fmt, record.is_animated, 
                    record.frame_count, record.estimated_quality,
                    record.predictor_name, record.prediction_rule, record.prediction_confidence, record.prediction_time_ms,
                    record.predicted_format, record.predicted_lossless, record.predicted_distance, record.predicted_effort,
                    record.predicted_saving_percent, record.predicted_output_size,
                    record.actual_format, record.actual_output_size, record.actual_conversion_time_ms,
                    record.actual_saving_percent, record.actual_saving_bytes,
                    record.prediction_error_percent, record.was_explored,
                    record.user_rating, record.user_comment
                ))
                
                record_id = cursor.lastrowid
                conn.commit()
                
                print(f"✅ 保存转换记录: {record.file_name} (ID: {record_id})")
                return record_id
    
    def get_prediction_accuracy(self, predictor_name: str, days: int = 30) -> Dict[str, Any]:
        """获取预测准确性分析"""
        cutoff_date = datetime.now() - timedelta(days=days)
        
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.execute("""
                SELECT 
                    prediction_rule,
                    original_format,
                    COUNT(*) as total,
                    AVG(prediction_error_percent) as avg_error,
                    AVG(prediction_confidence) as avg_confidence,
                    AVG(actual_saving_percent) as avg_saving,
                    AVG(user_rating) as avg_rating
                FROM conversion_records 
                WHERE predictor_name = ? 
                AND created_at > ?
                AND actual_output_size > 0
                GROUP BY prediction_rule, original_format
                ORDER BY total DESC
            """, (predictor_name, cutoff_date))
            
            results = []
            for row in cursor.fetchall():
                results.append({
                    "rule": row[0],
                    "format": row[1], 
                    "total_conversions": row[2],
                    "avg_error_percent": round(row[3], 2),
                    "avg_confidence": round(row[4], 2),
                    "avg_saving_percent": round(row[5], 2),
                    "avg_user_rating": round(row[6], 1)
                })
            
            return {
                "predictor": predictor_name,
                "analysis_period_days": days,
                "rules_analysis": results,
                "generated_at": datetime.now().isoformat()
            }
    
    def analyze_format_performance(self, format_name: str) -> Dict[str, Any]:
        """分析特定格式的转换性能"""
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.execute("""
                SELECT 
                    predicted_format,
                    COUNT(*) as conversions,
                    AVG(actual_saving_percent) as avg_saving,
                    AVG(prediction_error_percent) as avg_error,
                    AVG(user_rating) as avg_rating,
                    SUM(CASE WHEN user_rating >= 4 THEN 1 ELSE 0 END) as good_ratings
                FROM conversion_records
                WHERE original_format = ?
                AND actual_output_size > 0
                GROUP BY predicted_format
                ORDER BY conversions DESC
            """, (format_name,))
            
            target_formats = []
            for row in cursor.fetchall():
                good_rate = (row[5] / row[1] * 100) if row[1] > 0 else 0
                target_formats.append({
                    "target_format": row[0],
                    "conversions": row[1],
                    "avg_saving_percent": round(row[2], 2),
                    "avg_error_percent": round(row[3], 2),
                    "avg_user_rating": round(row[4], 1),
                    "good_rating_percent": round(good_rate, 1)
                })
            
            return {
                "source_format": format_name,
                "target_formats_analysis": target_formats,
                "best_target": target_formats[0] if target_formats else None,
                "analyzed_at": datetime.now().isoformat()
            }
    
    def get_learning_insights(self) -> Dict[str, Any]:
        """获取机器学习洞察"""
        with sqlite3.connect(self.db_path) as conn:
            # 预测错误热点
            cursor = conn.execute("""
                SELECT 
                    original_format,
                    predicted_format,
                    COUNT(*) as error_count,
                    AVG(prediction_error_percent) as avg_error
                FROM conversion_records
                WHERE prediction_error_percent > 20
                GROUP BY original_format, predicted_format
                ORDER BY error_count DESC
                LIMIT 10
            """)
            
            error_hotspots = []
            for row in cursor.fetchall():
                error_hotspots.append({
                    "source": row[0],
                    "target": row[1], 
                    "error_count": row[2],
                    "avg_error": round(row[3], 2)
                })
            
            # 高评分规则
            cursor = conn.execute("""
                SELECT 
                    prediction_rule,
                    COUNT(*) as uses,
                    AVG(user_rating) as avg_rating,
                    AVG(actual_saving_percent) as avg_saving
                FROM conversion_records
                WHERE user_rating > 0
                GROUP BY prediction_rule
                HAVING COUNT(*) >= 5
                ORDER BY avg_rating DESC, uses DESC
                LIMIT 10
            """)
            
            top_rules = []
            for row in cursor.fetchall():
                top_rules.append({
                    "rule": row[0],
                    "usage_count": row[1],
                    "avg_rating": round(row[2], 1),
                    "avg_saving": round(row[3], 2)
                })
            
            return {
                "error_hotspots": error_hotspots,
                "top_performing_rules": top_rules,
                "insights_generated_at": datetime.now().isoformat()
            }


class KnowledgeRecordBuilder:
    """知识记录构建器 - 替代Go RecordBuilder"""
    
    def __init__(self):
        self.record = ConversionRecord()
    
    def with_file_info(self, path: str, name: str, format: str, size: int) -> 'KnowledgeRecordBuilder':
        """设置文件信息"""
        self.record.file_path = path
        self.record.file_name = name
        self.record.original_format = format
        self.record.original_size = size
        return self
    
    def with_prediction(self, request: LocalPredictionRequest, result: LocalPredictionResult, 
                       predictor_name: str, inference_time_ms: float) -> 'KnowledgeRecordBuilder':
        """设置预测信息"""
        self.record.predictor_name = predictor_name
        self.record.prediction_rule = result.reasoning or "ai_prediction"
        self.record.prediction_confidence = result.confidence
        self.record.prediction_time_ms = int(inference_time_ms)
        
        self.record.predicted_format = result.recommended_format or request.tool
        self.record.predicted_distance = result.distance
        self.record.predicted_saving_percent = (1 - result.distance) * 100
        
        return self
    
    def with_actual_result(self, format: str, output_size: int, conversion_time_ms: int) -> 'KnowledgeRecordBuilder':
        """设置实际结果"""
        self.record.actual_format = format
        self.record.actual_output_size = output_size
        self.record.actual_conversion_time_ms = conversion_time_ms
        
        # 计算实际节省
        if self.record.original_size > 0:
            self.record.actual_saving_bytes = self.record.original_size - output_size
            self.record.actual_saving_percent = (self.record.actual_saving_bytes / self.record.original_size) * 100
        
        # 计算预测误差
        if self.record.predicted_output_size > 0:
            error = abs(output_size - self.record.predicted_output_size) / output_size
            self.record.prediction_error_percent = error * 100
        
        return self
    
    def with_user_feedback(self, rating: int, comment: str = "") -> 'KnowledgeRecordBuilder':
        """设置用户反馈"""
        self.record.user_rating = rating
        self.record.user_comment = comment
        return self
    
    def build(self) -> ConversionRecord:
        """构建记录"""
        return self.record


class LocalKnowledgeSystem:
    """
    🧠 本地化知识库系统
    
    集成转换记录、预测分析、用户反馈的完整知识管理
    """
    
    def __init__(self, db_path: str = "data/knowledge.db"):
        self.database = LocalKnowledgeDatabase(db_path)
        self._lock = threading.RLock()
    
    def record_prediction_and_result(self, request: LocalPredictionRequest, prediction: LocalPredictionResult,
                                   actual_format: str, actual_size: int, conversion_time_ms: int,
                                   predictor_name: str = "local_ai") -> int:
        """记录预测和实际结果"""
        
        record = (KnowledgeRecordBuilder()
                 .with_file_info(request.image_path, Path(request.image_path).name, 
                                request.tool, 0)  # 原始大小需要单独获取
                 .with_prediction(request, prediction, predictor_name, prediction.inference_time_ms)
                 .with_actual_result(actual_format, actual_size, conversion_time_ms)
                 .build())
        
        return self.database.save_record(record)
    
    def add_user_feedback(self, record_id: int, rating: int, comment: str = "") -> bool:
        """添加用户反馈"""
        with sqlite3.connect(self.database.db_path) as conn:
            cursor = conn.execute("""
                UPDATE conversion_records 
                SET user_rating = ?, user_comment = ?
                WHERE id = ?
            """, (rating, comment, record_id))
            
            success = cursor.rowcount > 0
            conn.commit()
            
            if success:
                print(f"✅ 更新用户反馈: 记录{record_id}, 评分{rating}")
            
            return success
    
    def get_predictor_performance(self, predictor_name: str, days: int = 30) -> Dict[str, Any]:
        """获取预测器性能分析"""
        return self.database.get_prediction_accuracy(predictor_name, days)
    
    def get_format_insights(self, format_name: str) -> Dict[str, Any]:
        """获取格式转换洞察"""
        return self.database.analyze_format_performance(format_name)
    
    def get_ml_insights(self) -> Dict[str, Any]:
        """获取机器学习洞察"""
        return self.database.get_learning_insights()
    
    def generate_knowledge_report(self) -> Dict[str, Any]:
        """生成知识库报告"""
        with sqlite3.connect(self.database.db_path) as conn:
            # 总体统计
            cursor = conn.execute("SELECT COUNT(*) FROM conversion_records")
            total_records = cursor.fetchone()[0]
            
            cursor = conn.execute("SELECT COUNT(*) FROM conversion_records WHERE user_rating > 0")
            rated_records = cursor.fetchone()[0]
            
            cursor = conn.execute("SELECT AVG(user_rating) FROM conversion_records WHERE user_rating > 0")
            avg_rating_result = cursor.fetchone()
            avg_rating = avg_rating_result[0] if avg_rating_result[0] else 0
            
            return {
                "total_records": total_records,
                "rated_records": rated_records,
                "avg_user_rating": round(avg_rating, 1),
                "rating_coverage": round((rated_records / total_records * 100), 1) if total_records > 0 else 0,
                "report_generated_at": datetime.now().isoformat(),
                "database_path": str(self.database.db_path)
            }


# 全局知识库实例
_global_knowledge_system: Optional[LocalKnowledgeSystem] = None
_knowledge_lock = threading.Lock()

def get_knowledge_system() -> LocalKnowledgeSystem:
    """获取全局知识库系统实例（单例模式）"""
    global _global_knowledge_system
    
    with _knowledge_lock:
        if _global_knowledge_system is None:
            _global_knowledge_system = LocalKnowledgeSystem()
        return _global_knowledge_system
