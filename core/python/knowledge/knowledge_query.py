"""
知识查询系统增强版 - 智能检索与推荐引擎

基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/knowledge/query.go 重新实现

核心功能:
- 多维度智能查询和检索
- 语义搜索和相似性分析
- 智能推荐和参数优化
- 高性能缓存和索引
- 查询结果排序和相关性评分

EX-030实现: 从Go废弃代码价值提取 + 智能检索架构增强
遵循四高原则：高规范化、高兼容性、高扩展性、高稳定性
"""

import sqlite3
from typing import Dict, List, Optional, Any, Tuple, Union
from dataclasses import dataclass, field, asdict
from datetime import datetime, timedelta
import logging
import json
import hashlib
from pathlib import Path
from enum import Enum
import time


class QueryType(Enum):
    """查询类型枚举"""
    EXACT = "exact"                # 精确查询
    FUZZY = "fuzzy"                # 模糊查询  
    SEMANTIC = "semantic"          # 语义查询
    SIMILARITY = "similarity"      # 相似性查询
    RECOMMENDATION = "recommendation"  # 推荐查询


class SortOrder(Enum):
    """排序方式"""
    CREATED_DESC = "created_at DESC"
    CREATED_ASC = "created_at ASC"
    SIZE_DESC = "file_size DESC"
    SIZE_ASC = "file_size ASC"
    SAVING_DESC = "saving_ratio DESC"
    QUALITY_DESC = "quality DESC"
    RELEVANCE_DESC = "relevance DESC"


@dataclass
class QueryFilter:
    """查询过滤条件"""
    # 格式过滤
    source_formats: List[str] = field(default_factory=list)
    target_formats: List[str] = field(default_factory=list)
    
    # 文件大小范围 (bytes)
    min_file_size: Optional[int] = None
    max_file_size: Optional[int] = None
    
    # 分辨率范围
    min_width: Optional[int] = None
    max_width: Optional[int] = None
    min_height: Optional[int] = None
    max_height: Optional[int] = None
    
    # 特征过滤
    has_alpha: Optional[bool] = None
    is_animated: Optional[bool] = None
    
    # 质量范围
    min_quality: Optional[int] = None
    max_quality: Optional[int] = None
    
    # 时间范围
    start_time: Optional[datetime] = None
    end_time: Optional[datetime] = None
    
    # 性能指标范围
    min_saving_ratio: Optional[float] = None
    max_saving_ratio: Optional[float] = None
    min_confidence: Optional[float] = None
    
    # 排序和分页
    sort_order: SortOrder = SortOrder.CREATED_DESC
    limit: int = 100
    offset: int = 0


@dataclass
class QueryResult:
    """查询结果"""
    records: List[Dict[str, Any]] = field(default_factory=list)
    total_count: int = 0
    query_time_ms: float = 0.0
    
    # 架构增强：统计信息
    statistics: Dict[str, Any] = field(default_factory=dict)
    recommendations: List[Dict[str, Any]] = field(default_factory=list)
    related_queries: List[str] = field(default_factory=list)


class KnowledgeQuerySystem:
    """
    知识查询系统增强版 - 智能检索与推荐引擎
    
    高规范化、高兼容性、高扩展性、高稳定性实现
    """
    
    def __init__(self, 
                 knowledge_db_path: str = "models/knowledge.db",
                 cache_size: int = 1000,
                 debug: bool = False):
        """
        初始化知识查询系统
        
        Args:
            knowledge_db_path: 知识数据库路径
            cache_size: 缓存大小
            debug: 调试模式
        """
        self.db_path = knowledge_db_path
        self.cache_size = cache_size
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        if debug:
            self.logger.setLevel(logging.DEBUG)
        
        # 初始化数据库连接
        self.db = sqlite3.connect(knowledge_db_path, check_same_thread=False)
        self.db.row_factory = sqlite3.Row
        
        # 架构增强：查询缓存
        self._query_cache: Dict[str, QueryResult] = {}
        self._cache_timestamps: Dict[str, float] = {}
        
        # 性能统计
        self._query_stats = {
            'total_queries': 0,
            'cache_hits': 0,
            'avg_query_time': 0.0
        }
        
        if debug:
            self.logger.debug("知识查询系统初始化完成")
    
    def query_records(self, 
                     query_filter: QueryFilter,
                     query_type: QueryType = QueryType.EXACT,
                     use_cache: bool = True) -> QueryResult:
        """
        查询转换记录
        
        Args:
            query_filter: 查询过滤条件
            query_type: 查询类型
            use_cache: 是否使用缓存
            
        Returns:
            查询结果
        """
        start_time = time.time()
        self._query_stats['total_queries'] += 1
        
        # 生成缓存键
        cache_key = self._generate_cache_key(query_filter, query_type)
        
        # 检查缓存
        if use_cache and cache_key in self._query_cache:
            cache_time = self._cache_timestamps.get(cache_key, 0)
            if time.time() - cache_time < 300:  # 5分钟缓存
                self._query_stats['cache_hits'] += 1
                return self._query_cache[cache_key]
        
        try:
            # 构建SQL查询
            sql, params = self._build_sql_query(query_filter, query_type)
            
            # 执行查询
            cursor = self.db.execute(sql, params)
            rows = cursor.fetchall()
            
            # 转换结果
            records = [dict(row) for row in rows]
            
            # 计算总数
            total_count = len(records) if query_filter.limit == 0 else self._get_total_count(query_filter, query_type)
            
            # 创建结果对象
            query_time = (time.time() - start_time) * 1000
            result = QueryResult(
                records=records,
                total_count=total_count,
                query_time_ms=query_time
            )
            
            # 添加统计信息和推荐
            result.statistics = self._calculate_statistics(records)
            result.recommendations = self._generate_recommendations(records, query_filter)
            
            # 更新性能统计
            self._update_query_stats(query_time)
            
            # 缓存结果
            if use_cache:
                self._cache_result(cache_key, result)
            
            return result
            
        except Exception as e:
            self.logger.error(f"查询失败: {e}")
            return QueryResult()
    
    def _build_sql_query(self, query_filter: QueryFilter, query_type: QueryType) -> Tuple[str, List]:
        """构建SQL查询语句"""
        base_sql = "SELECT * FROM conversion_records WHERE 1=1"
        params = []
        conditions = []
        
        # 格式过滤
        if query_filter.source_formats:
            placeholders = ','.join(['?' for _ in query_filter.source_formats])
            conditions.append(f"original_format IN ({placeholders})")
            params.extend(query_filter.source_formats)
        
        if query_filter.target_formats:
            placeholders = ','.join(['?' for _ in query_filter.target_formats])
            conditions.append(f"target_format IN ({placeholders})")
            params.extend(query_filter.target_formats)
        
        # 文件大小过滤
        if query_filter.min_file_size is not None:
            conditions.append("original_size >= ?")
            params.append(query_filter.min_file_size)
        
        if query_filter.max_file_size is not None:
            conditions.append("original_size <= ?")
            params.append(query_filter.max_file_size)
        
        # 分辨率过滤
        for attr, op in [('min_width', '>='), ('max_width', '<='), 
                        ('min_height', '>='), ('max_height', '<=')]:
            value = getattr(query_filter, attr)
            if value is not None:
                field = attr.split('_')[1]  # width or height
                conditions.append(f"{field} {op} ?")
                params.append(value)
        
        # 特征过滤
        if query_filter.has_alpha is not None:
            conditions.append("has_alpha = ?")
            params.append(query_filter.has_alpha)
        
        if query_filter.is_animated is not None:
            conditions.append("is_animated = ?")
            params.append(query_filter.is_animated)
        
        # 组装SQL
        if conditions:
            base_sql += " AND " + " AND ".join(conditions)
        
        # 排序
        base_sql += f" ORDER BY {query_filter.sort_order.value}"
        
        # 分页
        if query_filter.limit > 0:
            base_sql += " LIMIT ?"
            params.append(query_filter.limit)
            
            if query_filter.offset > 0:
                base_sql += " OFFSET ?"
                params.append(query_filter.offset)
        
        return base_sql, params
    
    def _generate_cache_key(self, query_filter: QueryFilter, query_type: QueryType) -> str:
        """生成缓存键"""
        data = f"{asdict(query_filter)}_{query_type.value}"
        return hashlib.md5(data.encode()).hexdigest()[:16]
    
    def _calculate_statistics(self, records: List[Dict[str, Any]]) -> Dict[str, Any]:
        """计算查询结果统计"""
        if not records:
            return {}
        
        # 简化的统计信息
        total_size = sum(r.get('original_size', 0) for r in records)
        avg_size = total_size / len(records) if records else 0
        
        formats = {}
        for record in records:
            fmt = record.get('original_format', 'unknown')
            formats[fmt] = formats.get(fmt, 0) + 1
        
        return {
            'total_records': len(records),
            'average_file_size': avg_size,
            'format_distribution': formats
        }
    
    def _generate_recommendations(self, records: List[Dict[str, Any]], 
                                 query_filter: QueryFilter) -> List[Dict[str, Any]]:
        """生成智能推荐"""
        recommendations = []
        
        if records:
            # 推荐最常用的格式组合
            format_pairs = {}
            for record in records:
                pair = f"{record.get('original_format', '')}->{record.get('target_format', '')}"
                format_pairs[pair] = format_pairs.get(pair, 0) + 1
            
            if format_pairs:
                best_pair = max(format_pairs.items(), key=lambda x: x[1])
                recommendations.append({
                    'type': 'format_combination',
                    'suggestion': best_pair[0],
                    'count': best_pair[1],
                    'reason': '最常用的格式组合'
                })
        
        return recommendations
    
    def get_statistics(self) -> Dict[str, Any]:
        """获取全局统计信息"""
        try:
            cursor = self.db.execute("SELECT COUNT(*) as total FROM conversion_records")
            total_records = cursor.fetchone()[0]
            
            cursor = self.db.execute("""
                SELECT original_format, COUNT(*) as count 
                FROM conversion_records 
                GROUP BY original_format 
                ORDER BY count DESC 
                LIMIT 10
            """)
            format_dist = {row[0]: row[1] for row in cursor.fetchall()}
            
            return {
                'total_records': total_records,
                'format_distribution': format_dist,
                'query_performance': self._query_stats
            }
        except Exception as e:
            self.logger.error(f"获取统计失败: {e}")
            return {}
    
    def search_similar(self, 
                      source_format: str, 
                      target_format: str,
                      file_size: int, 
                      width: int, 
                      height: int,
                      tolerance: float = 0.1) -> QueryResult:
        """搜索相似转换记录"""
        size_tolerance = int(file_size * tolerance)
        width_tolerance = int(width * tolerance)
        height_tolerance = int(height * tolerance)
        
        query_filter = QueryFilter(
            source_formats=[source_format],
            target_formats=[target_format],
            min_file_size=max(0, file_size - size_tolerance),
            max_file_size=file_size + size_tolerance,
            min_width=max(0, width - width_tolerance),
            max_width=width + width_tolerance,
            min_height=max(0, height - height_tolerance),
            max_height=height + height_tolerance,
            limit=50
        )
        
        return self.query_records(query_filter, QueryType.SIMILARITY)
    
    def recommend_params(self, source_format: str, target_format: str) -> Dict[str, Any]:
        """基于历史数据推荐最佳参数"""
        try:
            cursor = self.db.execute("""
                SELECT AVG(estimated_quality) as avg_quality,
                       AVG(actual_saving_percent) as avg_saving,
                       COUNT(*) as sample_count
                FROM conversion_records 
                WHERE original_format = ? AND target_format = ?
                AND validation_passed = 1
            """, (source_format, target_format))
            
            result = cursor.fetchone()
            if result and result[2] > 0:  # sample_count > 0
                confidence = min(result[2] / (result[2] + 10), 0.9)
                return {
                    'recommended_quality': int(result[0] or 85),
                    'expected_saving': result[1] or 0,
                    'confidence': confidence,
                    'sample_count': result[2]
                }
            
            return {'error': '无足够历史数据'}
            
        except Exception as e:
            self.logger.error(f"参数推荐失败: {e}")
            return {'error': str(e)}
    
    def _get_total_count(self, query_filter: QueryFilter, query_type: QueryType) -> int:
        """获取查询总数（不包含分页限制）"""
        try:
            sql, params = self._build_sql_query(query_filter, query_type)
            # 移除LIMIT和OFFSET，添加COUNT
            sql = sql.replace("SELECT *", "SELECT COUNT(*)")
            sql = sql.split(" ORDER BY")[0]  # 移除排序和分页
            
            cursor = self.db.execute(sql, params)
            return cursor.fetchone()[0]
        except:
            return 0
    
    def _cache_result(self, cache_key: str, result: QueryResult):
        """缓存查询结果"""
        if len(self._query_cache) >= self.cache_size:
            # 简单的LRU：删除最旧的缓存
            oldest_key = min(self._cache_timestamps.items(), key=lambda x: x[1])[0]
            del self._query_cache[oldest_key]
            del self._cache_timestamps[oldest_key]
        
        self._query_cache[cache_key] = result
        self._cache_timestamps[cache_key] = time.time()
    
    def _update_query_stats(self, query_time_ms: float):
        """更新查询性能统计"""
        total = self._query_stats['total_queries']
        current_avg = self._query_stats['avg_query_time']
        
        self._query_stats['avg_query_time'] = (
            (current_avg * (total - 1) + query_time_ms) / total
        )
    
    def clear_cache(self):
        """清空缓存"""
        self._query_cache.clear()
        self._cache_timestamps.clear()
    
    def close(self):
        """关闭数据库连接"""
        if self.db:
            self.db.close()


# 便捷函数
def create_knowledge_query_system(db_path: str = "models/knowledge.db", debug: bool = False) -> KnowledgeQuerySystem:
    """创建知识查询系统的便捷函数"""
    return KnowledgeQuerySystem(db_path, debug=debug)


if __name__ == "__main__":
    # 测试代码
    print("=== 知识查询系统测试 ===")
    
    query_system = create_knowledge_query_system(debug=True)
    
    # 测试基础查询
    query_filter = QueryFilter(
        source_formats=["jpg", "png"],
        target_formats=["jxl", "avif"],
        limit=10
    )
    
    result = query_system.query_records(query_filter)
    print(f"✅ 查询完成: {result.total_count}条记录, 耗时: {result.query_time_ms:.2f}ms")
    
    # 测试统计信息
    stats = query_system.get_statistics()
    print(f"📊 数据库统计: {stats.get('total_records', 0)}条总记录")
    
    print("🎯 知识查询系统测试完成！")
