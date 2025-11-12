"""
知识库数据库系统 - 转换记录生命周期管理

基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/knowledge/database.go 重新实现

核心功能:
- 完整转换记录生命周期跟踪
- 预测vs实际结果对比存储  
- 数据库schema自动管理
- 高性能查询索引
- 种子数据初始化
- 统计分析支持

EX-021实现: 从Go废弃代码价值提取
遵循四高原则：高规范化、高兼容性、高扩展性、高稳定性
"""

import sqlite3
import json
from datetime import datetime
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, asdict
from pathlib import Path
import logging
import threading


@dataclass
class ConversionRecord:
    """转换记录数据结构"""
    # 基础信息
    id: Optional[int] = None
    created_at: Optional[datetime] = None
    file_path: str = ""
    file_name: str = ""
    original_format: str = ""
    original_size: int = 0
    width: int = 0
    height: int = 0
    has_alpha: bool = False
    pix_fmt: str = ""
    is_animated: bool = False
    frame_count: int = 0
    estimated_quality: float = 0.0
    
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
    predicted_lossless_jpeg: bool = False
    predicted_crf: int = 0
    predicted_speed: int = 0
    predicted_saving_percent: float = 0.0
    predicted_output_size: int = 0
    
    # 实际结果
    actual_format: str = ""
    actual_output_size: int = 0
    actual_conversion_time_ms: int = 0
    actual_saving_percent: float = 0.0
    actual_saving_bytes: int = 0
    
    # 验证信息
    validation_method: str = ""
    validation_passed: bool = False
    pixel_diff_percent: float = 0.0
    psnr_value: float = 0.0
    ssim_value: float = 0.0
    prediction_error_percent: float = 0.0
    was_explored: bool = False
    
    # 用户反馈
    user_rating: Optional[int] = None
    user_comment: str = ""
    
    # 系统信息
    pixly_version: str = ""
    host_os: str = ""


class KnowledgeDatabase:
    """
    知识库数据库 - 转换记录生命周期管理
    
    高规范化、高兼容性、高扩展性、高稳定性实现
    """
    
    def __init__(self, db_path: str = "data/knowledge.db", debug: bool = False):
        """
        初始化知识库数据库
        
        Args:
            db_path: 数据库文件路径
            debug: 调试模式
        """
        self.db_path = Path(db_path)
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        self._lock = threading.RLock()
        
        if debug:
            self.logger.setLevel(logging.DEBUG)
            self.logger.debug(f"知识库数据库初始化: {db_path}")
        
        # 确保数据库目录存在
        self.db_path.parent.mkdir(parents=True, exist_ok=True)
        
        # 初始化数据库
        self._init_database()
    
    def _init_database(self) -> None:
        """初始化数据库schema"""
        with self._lock:
            try:
                with sqlite3.connect(self.db_path) as conn:
                    # 创建主转换记录表
                    conn.execute("""
                        CREATE TABLE IF NOT EXISTS conversion_records (
                            id INTEGER PRIMARY KEY AUTOINCREMENT,
                            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                            
                            -- 文件信息
                            file_path TEXT NOT NULL,
                            file_name TEXT,
                            original_format TEXT,
                            original_size INTEGER,
                            width INTEGER,
                            height INTEGER,
                            has_alpha BOOLEAN,
                            pix_fmt TEXT,
                            is_animated BOOLEAN,
                            frame_count INTEGER,
                            estimated_quality REAL,
                            
                            -- 预测信息
                            predictor_name TEXT NOT NULL,
                            prediction_rule TEXT,
                            prediction_confidence REAL,
                            prediction_time_ms INTEGER,
                            
                            -- 预测参数
                            predicted_format TEXT,
                            predicted_lossless BOOLEAN,
                            predicted_distance REAL,
                            predicted_effort INTEGER,
                            predicted_lossless_jpeg BOOLEAN,
                            predicted_crf INTEGER,
                            predicted_speed INTEGER,
                            predicted_saving_percent REAL,
                            predicted_output_size INTEGER,
                            
                            -- 实际结果
                            actual_format TEXT,
                            actual_output_size INTEGER,
                            actual_conversion_time_ms INTEGER,
                            actual_saving_percent REAL,
                            actual_saving_bytes INTEGER,
                            
                            -- 验证信息
                            validation_method TEXT,
                            validation_passed BOOLEAN,
                            pixel_diff_percent REAL,
                            psnr_value REAL,
                            ssim_value REAL,
                            prediction_error_percent REAL,
                            was_explored BOOLEAN,
                            
                            -- 用户反馈
                            user_rating INTEGER,
                            user_comment TEXT,
                            
                            -- 系统信息
                            pixly_version TEXT,
                            host_os TEXT
                        )
                    """)
                    
                    # 创建性能优化索引
                    indexes = [
                        "CREATE INDEX IF NOT EXISTS idx_predictor_rule_format ON conversion_records(predictor_name, prediction_rule, original_format)",
                        "CREATE INDEX IF NOT EXISTS idx_validation_passed ON conversion_records(validation_passed)",
                        "CREATE INDEX IF NOT EXISTS idx_prediction_error ON conversion_records(prediction_error_percent)",
                        "CREATE INDEX IF NOT EXISTS idx_created_at ON conversion_records(created_at)",
                        "CREATE INDEX IF NOT EXISTS idx_actual_saving ON conversion_records(actual_saving_percent)",
                        "CREATE INDEX IF NOT EXISTS idx_quality_metrics ON conversion_records(ssim_value, psnr_value)"
                    ]
                    
                    for index_sql in indexes:
                        conn.execute(index_sql)
                    
                    # 创建统计视图
                    conn.execute("""
                        CREATE VIEW IF NOT EXISTS conversion_stats AS
                        SELECT 
                            predictor_name,
                            prediction_rule,
                            original_format,
                            COUNT(*) as total_conversions,
                            AVG(prediction_error_percent) as avg_prediction_error,
                            AVG(actual_saving_percent) as avg_space_saving,
                            AVG(ssim_value) as avg_ssim,
                            AVG(psnr_value) as avg_psnr,
                            SUM(CASE WHEN validation_passed = 1 THEN 1 ELSE 0 END) as successful_conversions
                        FROM conversion_records 
                        GROUP BY predictor_name, prediction_rule, original_format
                    """)
                    
                    if self.debug:
                        self.logger.debug("数据库schema初始化完成")
                        
            except Exception as e:
                self.logger.error(f"数据库初始化失败: {e}")
                raise
    
    def save_record(self, record: ConversionRecord) -> Optional[int]:
        """
        保存转换记录
        
        Args:
            record: 转换记录
            
        Returns:
            记录ID，失败返回None
        """
        with self._lock:
            try:
                # 设置创建时间
                if record.created_at is None:
                    record.created_at = datetime.now()
                
                with sqlite3.connect(self.db_path) as conn:
                    # 转换为字典并过滤None值
                    data = asdict(record)
                    
                    # 移除id字段（自动生成）
                    if 'id' in data:
                        del data['id']
                    
                    # 转换datetime为字符串
                    if 'created_at' in data and data['created_at']:
                        data['created_at'] = data['created_at'].isoformat()
                    
                    # 构建SQL
                    columns = list(data.keys())
                    placeholders = ['?' for _ in columns]
                    values = [data[col] for col in columns]
                    
                    sql = f"""
                        INSERT INTO conversion_records ({', '.join(columns)})
                        VALUES ({', '.join(placeholders)})
                    """
                    
                    cursor = conn.execute(sql, values)
                    record_id = cursor.lastrowid
                    
                    if self.debug:
                        self.logger.debug(f"记录保存成功: ID={record_id}, "
                                        f"文件={record.file_name}, "
                                        f"预测器={record.predictor_name}")
                    
                    return record_id
                    
            except Exception as e:
                self.logger.error(f"保存记录失败: {e}")
                return None
    
    def get_record(self, record_id: int) -> Optional[ConversionRecord]:
        """
        获取转换记录
        
        Args:
            record_id: 记录ID
            
        Returns:
            转换记录，如果不存在返回None
        """
        with self._lock:
            try:
                with sqlite3.connect(self.db_path) as conn:
                    conn.row_factory = sqlite3.Row
                    
                    cursor = conn.execute(
                        "SELECT * FROM conversion_records WHERE id = ?",
                        (record_id,)
                    )
                    row = cursor.fetchone()
                    
                    if row:
                        return self._row_to_record(row)
                    else:
                        return None
                        
            except Exception as e:
                self.logger.error(f"获取记录失败: {e}")
                return None
    
    def query_records(self,
                     predictor_name: Optional[str] = None,
                     original_format: Optional[str] = None,
                     validation_passed: Optional[bool] = None,
                     limit: int = 100,
                     order_by: str = "created_at DESC") -> List[ConversionRecord]:
        """
        查询转换记录
        
        Args:
            predictor_name: 预测器名称过滤
            original_format: 原始格式过滤
            validation_passed: 验证通过状态过滤
            limit: 最大返回数量
            order_by: 排序方式
            
        Returns:
            转换记录列表
        """
        with self._lock:
            try:
                conditions = []
                params = []
                
                if predictor_name:
                    conditions.append("predictor_name = ?")
                    params.append(predictor_name)
                
                if original_format:
                    conditions.append("original_format = ?")
                    params.append(original_format)
                
                if validation_passed is not None:
                    conditions.append("validation_passed = ?")
                    params.append(validation_passed)
                
                where_clause = ""
                if conditions:
                    where_clause = "WHERE " + " AND ".join(conditions)
                
                sql = f"""
                    SELECT * FROM conversion_records 
                    {where_clause}
                    ORDER BY {order_by}
                    LIMIT ?
                """
                params.append(limit)
                
                with sqlite3.connect(self.db_path) as conn:
                    conn.row_factory = sqlite3.Row
                    cursor = conn.execute(sql, params)
                    
                    records = []
                    for row in cursor.fetchall():
                        record = self._row_to_record(row)
                        if record:
                            records.append(record)
                    
                    return records
                    
            except Exception as e:
                self.logger.error(f"查询记录失败: {e}")
                return []
    
    def _row_to_record(self, row: sqlite3.Row) -> Optional[ConversionRecord]:
        """将数据库行转换为ConversionRecord对象"""
        try:
            data = dict(row)
            
            # 转换created_at
            if data.get('created_at'):
                data['created_at'] = datetime.fromisoformat(data['created_at'])
            
            return ConversionRecord(**data)
            
        except Exception as e:
            self.logger.error(f"行转换失败: {e}")
            return None
    
    def get_statistics(self) -> Dict[str, Any]:
        """
        获取统计信息
        
        Returns:
            统计数据字典
        """
        with self._lock:
            try:
                with sqlite3.connect(self.db_path) as conn:
                    # 总体统计
                    cursor = conn.execute("""
                        SELECT 
                            COUNT(*) as total_records,
                            AVG(actual_saving_percent) as avg_saving,
                            AVG(prediction_error_percent) as avg_error,
                            AVG(ssim_value) as avg_ssim,
                            AVG(psnr_value) as avg_psnr,
                            SUM(CASE WHEN validation_passed = 1 THEN 1 ELSE 0 END) as passed_count
                        FROM conversion_records
                    """)
                    overall = cursor.fetchone()
                    
                    # 按格式统计
                    cursor = conn.execute("""
                        SELECT 
                            original_format,
                            COUNT(*) as count,
                            AVG(actual_saving_percent) as avg_saving,
                            AVG(prediction_error_percent) as avg_error
                        FROM conversion_records 
                        GROUP BY original_format
                        ORDER BY count DESC
                    """)
                    by_format = cursor.fetchall()
                    
                    # 按预测器统计
                    cursor = conn.execute("""
                        SELECT 
                            predictor_name,
                            COUNT(*) as count,
                            AVG(prediction_error_percent) as avg_error,
                            AVG(prediction_confidence) as avg_confidence
                        FROM conversion_records 
                        GROUP BY predictor_name
                        ORDER BY count DESC
                    """)
                    by_predictor = cursor.fetchall()
                    
                    return {
                        'overall': {
                            'total_records': overall[0] or 0,
                            'avg_saving_percent': overall[1] or 0.0,
                            'avg_prediction_error': overall[2] or 0.0,
                            'avg_ssim': overall[3] or 0.0,
                            'avg_psnr': overall[4] or 0.0,
                            'validation_success_rate': (overall[5] or 0) / (overall[0] or 1)
                        },
                        'by_format': [
                            {
                                'format': row[0],
                                'count': row[1],
                                'avg_saving_percent': row[2] or 0.0,
                                'avg_prediction_error': row[3] or 0.0
                            }
                            for row in by_format
                        ],
                        'by_predictor': [
                            {
                                'predictor': row[0],
                                'count': row[1],
                                'avg_error': row[2] or 0.0,
                                'avg_confidence': row[3] or 0.0
                            }
                            for row in by_predictor
                        ]
                    }
                    
            except Exception as e:
                self.logger.error(f"获取统计失败: {e}")
                return {}
    
    def has_seed_data(self) -> bool:
        """
        检查是否有种子数据
        
        Returns:
            是否存在数据
        """
        with self._lock:
            try:
                with sqlite3.connect(self.db_path) as conn:
                    cursor = conn.execute("SELECT COUNT(*) FROM conversion_records")
                    count = cursor.fetchone()[0]
                    return count > 0
                    
            except Exception:
                return False
    
    def clear_data(self) -> bool:
        """
        清空所有数据（慎用）
        
        Returns:
            是否成功
        """
        with self._lock:
            try:
                with sqlite3.connect(self.db_path) as conn:
                    conn.execute("DELETE FROM conversion_records")
                    
                if self.debug:
                    self.logger.debug("数据库已清空")
                    
                return True
                
            except Exception as e:
                self.logger.error(f"清空数据失败: {e}")
                return False
    
    def close(self) -> None:
        """关闭数据库连接"""
        if self.debug:
            self.logger.debug("知识库数据库已关闭")


# 便捷函数
def create_knowledge_db(db_path: str = "data/knowledge.db") -> KnowledgeDatabase:
    """
    创建知识库数据库的便捷函数
    
    Args:
        db_path: 数据库路径
        
    Returns:
        知识库数据库实例
    """
    return KnowledgeDatabase(db_path)


def save_conversion_record(record: ConversionRecord, 
                         db_path: str = "data/knowledge.db") -> Optional[int]:
    """
    保存转换记录的便捷函数
    
    Args:
        record: 转换记录
        db_path: 数据库路径
        
    Returns:
        记录ID
    """
    db = KnowledgeDatabase(db_path)
    return db.save_record(record)


if __name__ == "__main__":
    # 测试代码
    print("=== 知识库数据库系统测试 ===")
    
    # 创建测试数据库
    db = KnowledgeDatabase("data/test_knowledge.db", debug=True)
    
    # 测试保存记录
    test_record = ConversionRecord(
        file_path="/test/image.jpg",
        file_name="image.jpg",
        original_format="jpg",
        original_size=1024000,
        width=1920,
        height=1080,
        predictor_name="lightgbm",
        prediction_rule="auto",
        predicted_format="jxl",
        predicted_saving_percent=35.0,
        actual_output_size=665600,
        actual_saving_percent=35.0,
        validation_passed=True,
        ssim_value=0.98,
        psnr_value=45.2
    )
    
    record_id = db.save_record(test_record)
    print(f"✅ 测试记录保存: ID={record_id}")
    
    # 测试获取记录
    retrieved = db.get_record(record_id) if record_id else None
    print(f"✅ 记录检索: {'成功' if retrieved else '失败'}")
    
    # 测试查询记录
    records = db.query_records(predictor_name="lightgbm", limit=5)
    print(f"✅ 记录查询: 找到{len(records)}条记录")
    
    # 测试统计信息
    stats = db.get_statistics()
    print(f"✅ 统计信息: 总记录{stats.get('overall', {}).get('total_records', 0)}条")
    
    db.close()
    print("🎯 知识库数据库系统测试完成！")
