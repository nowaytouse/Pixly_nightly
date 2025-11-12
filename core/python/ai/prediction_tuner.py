"""
🧠 PIXLY v3.1 预测参数微调系统

替代Go knowledge/tuner.go的完整功能：
- 历史数据驱动参数优化 - 基于转换记录自动调参
- 置信度驱动探索决策 - 样本不足时触发探索
- 格式组合成功率分析 - 智能格式推荐
- 缓存优化系统 - TTL + 命中率统计
- 自适应学习引擎 - 持续改进预测准确性

完全本地化，基于SQLite + 统计分析实现
"""

import sqlite3
import time
import json
import threading
from datetime import datetime, timedelta
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, asdict
import statistics
import numpy as np

from .knowledge_system import LocalKnowledgeDatabase, get_knowledge_system


@dataclass
class TunedParams:
    """微调后的参数 - 替代Go TunedParams"""
    source_format: str
    target_format: str
    quality_goal: str
    
    # 优化后的参数
    optimal_saving: float      # 基于历史的最优节省率
    optimal_effort: int        # 基于文件大小的最优effort
    optimal_crf: int          # 基于质量的最优CRF
    optimal_speed: int        # AVIF编码速度
    optimal_quality: int      # 推荐质量参数
    optimal_distance: float   # 推荐distance参数
    
    # 元数据
    confidence: float         # 微调置信度（基于样本数）
    sample_count: int        # 样本数量
    avg_error: float         # 平均预测误差
    success_rate: float      # 成功率
    last_updated: datetime
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典"""
        result = asdict(self)
        result['last_updated'] = self.last_updated.isoformat()
        return result


@dataclass
class CachedTuning:
    """缓存的微调结果"""
    params: TunedParams
    cached_at: datetime
    hit_count: int = 0
    
    def is_expired(self, ttl_seconds: int) -> bool:
        """检查是否过期"""
        return (datetime.now() - self.cached_at).total_seconds() > ttl_seconds


@dataclass
class FormatCombination:
    """格式组合分析"""
    source_format: str
    target_format: str
    sample_count: int
    avg_saving: float
    success_count: int
    success_rate: float
    avg_quality: int
    confidence: float


class PredictionTuner:
    """
    🧠 预测参数微调器
    
    基于历史数据动态调整预测参数，实现自我优化
    """
    
    def __init__(self, cache_ttl_hours: int = 1):
        self.knowledge_system = get_knowledge_system()
        self.cache: Dict[str, CachedTuning] = {}
        self.cache_ttl_seconds = cache_ttl_hours * 3600
        self._lock = threading.RLock()
        
        # 统计信息
        self._stats = {
            "total_queries": 0,
            "cache_hits": 0,
            "cache_misses": 0,
            "tuning_computations": 0,
            "last_cleanup": datetime.now()
        }
        
        print(f"✅ 预测微调器初始化: 缓存TTL={cache_ttl_hours}小时")
    
    def get_tuned_params(self, source_format: str, target_format: str, 
                        quality_goal: str = "balanced") -> TunedParams:
        """
        获取微调后的参数
        
        Args:
            source_format: 源格式
            target_format: 目标格式  
            quality_goal: 质量目标 ("size", "balanced", "quality")
            
        Returns:
            TunedParams: 微调参数
        """
        with self._lock:
            self._stats["total_queries"] += 1
            
            cache_key = f"{source_format}->{target_format}:{quality_goal}"
            
            # 检查缓存
            cached = self._get_cached(cache_key)
            if cached:
                self._stats["cache_hits"] += 1
                print(f"🎯 使用缓存参数: {cache_key} (命中{cached.hit_count}次)")
                return cached.params
            
            # 缓存未命中，计算新参数
            self._stats["cache_misses"] += 1
            self._stats["tuning_computations"] += 1
            
            params = self._calculate_optimal_params(source_format, target_format, quality_goal)
            
            # 更新缓存
            self._set_cached(cache_key, params)
            
            print(f"🔧 计算新参数: {cache_key}, 置信度={params.confidence:.2f}")
            return params
    
    def _calculate_optimal_params(self, source_format: str, target_format: str,
                                quality_goal: str) -> TunedParams:
        """计算最优参数"""
        try:
            # 查询历史记录
            with sqlite3.connect(self.knowledge_system.database.db_path) as conn:
                cursor = conn.execute("""
                    SELECT 
                        COUNT(*) as sample_count,
                        AVG(actual_saving_percent) as avg_saving,
                        AVG(prediction_error_percent) as avg_error,
                        AVG(predicted_effort) as avg_effort,
                        AVG(predicted_crf) as avg_crf,
                        AVG(predicted_distance) as avg_distance,
                        AVG(CASE WHEN predicted_format = 'webp' THEN 6 
                                 WHEN predicted_format = 'avif' THEN 6
                                 ELSE 4 END) as avg_speed,
                        AVG(CASE WHEN validation_passed = 1 THEN 1 ELSE 0 END) as success_rate,
                        AVG(CASE WHEN ssim_value > 0 THEN ssim_value * 100 ELSE 85 END) as avg_quality
                    FROM conversion_records
                    WHERE original_format = ?
                      AND actual_format = ?
                      AND actual_output_size > 0
                      AND created_at > datetime('now', '-30 days')
                """, (source_format, target_format))
                
                row = cursor.fetchone()
                
                if not row or row[0] == 0:
                    # 没有历史数据，返回默认参数
                    return self._get_default_params(source_format, target_format, quality_goal)
                
                (sample_count, avg_saving, avg_error, avg_effort, avg_crf,
                 avg_distance, avg_speed, success_rate, avg_quality) = row
                
                # 处理NULL值
                avg_saving = avg_saving or 0.0
                avg_error = avg_error or 10.0
                avg_effort = int(avg_effort or 6)
                avg_crf = int(avg_crf or 23)
                avg_distance = avg_distance or 1.0
                avg_speed = int(avg_speed or 6)
                success_rate = success_rate or 0.8
                avg_quality = int(avg_quality or 85)
                
                # 计算置信度
                confidence = self._calculate_confidence(sample_count, success_rate, avg_error)
                
                # 根据质量目标调整参数
                adjusted_params = self._adjust_for_quality_goal(
                    quality_goal, avg_quality, avg_distance, avg_crf, avg_effort
                )
                
                return TunedParams(
                    source_format=source_format,
                    target_format=target_format,
                    quality_goal=quality_goal,
                    optimal_saving=avg_saving,
                    optimal_effort=adjusted_params["effort"],
                    optimal_crf=adjusted_params["crf"],
                    optimal_speed=avg_speed,
                    optimal_quality=adjusted_params["quality"],
                    optimal_distance=adjusted_params["distance"],
                    confidence=confidence,
                    sample_count=sample_count,
                    avg_error=avg_error,
                    success_rate=success_rate,
                    last_updated=datetime.now()
                )
                
        except Exception as e:
            print(f"❌ 参数计算失败: {e}")
            return self._get_default_params(source_format, target_format, quality_goal)
    
    def _adjust_for_quality_goal(self, quality_goal: str, base_quality: int,
                               base_distance: float, base_crf: int, 
                               base_effort: int) -> Dict[str, Any]:
        """根据质量目标调整参数"""
        adjustments = {
            "size": {
                "quality_delta": -10,
                "distance_delta": 0.3,
                "crf_delta": 5,
                "effort_delta": -1
            },
            "balanced": {
                "quality_delta": 0,
                "distance_delta": 0.0,
                "crf_delta": 0,
                "effort_delta": 0
            },
            "quality": {
                "quality_delta": 8,
                "distance_delta": -0.2,
                "crf_delta": -3,
                "effort_delta": 1
            }
        }
        
        adj = adjustments.get(quality_goal, adjustments["balanced"])
        
        return {
            "quality": max(70, min(100, base_quality + adj["quality_delta"])),
            "distance": max(0.0, min(2.0, base_distance + adj["distance_delta"])),
            "crf": max(18, min(51, base_crf + adj["crf_delta"])),
            "effort": max(1, min(9, base_effort + adj["effort_delta"]))
        }
    
    def _calculate_confidence(self, sample_count: int, success_rate: float,
                            avg_error: float) -> float:
        """计算置信度"""
        # 基于样本数的置信度
        if sample_count < 5:
            sample_confidence = 0.3
        elif sample_count < 10:
            sample_confidence = 0.5
        elif sample_count < 20:
            sample_confidence = 0.7
        elif sample_count < 50:
            sample_confidence = 0.8
        elif sample_count < 100:
            sample_confidence = 0.9
        else:
            sample_confidence = 0.95
        
        # 基于成功率的置信度
        success_confidence = min(0.95, success_rate)
        
        # 基于预测误差的置信度（误差越小置信度越高）
        error_confidence = max(0.1, 1.0 - (avg_error / 100.0))
        
        # 综合置信度
        confidence = (
            sample_confidence * 0.4 +
            success_confidence * 0.4 +
            error_confidence * 0.2
        )
        
        return max(0.1, min(0.99, confidence))
    
    def _get_default_params(self, source_format: str, target_format: str,
                          quality_goal: str) -> TunedParams:
        """获取默认参数（无历史数据时）"""
        # 基于格式和目标的默认参数
        defaults = {
            "size": {"quality": 75, "distance": 1.5, "crf": 28, "effort": 4},
            "balanced": {"quality": 85, "distance": 1.0, "crf": 23, "effort": 6},
            "quality": {"quality": 95, "distance": 0.5, "crf": 18, "effort": 8}
        }
        
        params = defaults.get(quality_goal, defaults["balanced"])
        
        # 根据目标格式调整
        if target_format == "avif":
            params["crf"] = max(18, params["crf"] - 2)  # AVIF效率更高
        elif target_format == "jxl":
            params["distance"] = max(0.0, params["distance"] - 0.2)
        
        return TunedParams(
            source_format=source_format,
            target_format=target_format,
            quality_goal=quality_goal,
            optimal_saving=30.0,  # 默认预期节省30%
            optimal_effort=params["effort"],
            optimal_crf=params["crf"],
            optimal_speed=6,
            optimal_quality=params["quality"],
            optimal_distance=params["distance"],
            confidence=0.5,  # 低置信度
            sample_count=0,
            avg_error=15.0,
            success_rate=0.8,
            last_updated=datetime.now()
        )
    
    def suggest_exploration(self, source_format: str, target_format: str,
                          confidence: float) -> bool:
        """建议是否需要探索（A/B测试）"""
        # 获取该格式组合的样本数
        try:
            with sqlite3.connect(self.knowledge_system.database.db_path) as conn:
                cursor = conn.execute("""
                    SELECT COUNT(*)
                    FROM conversion_records
                    WHERE original_format = ? AND actual_format = ?
                """, (source_format, target_format))
                
                sample_count = cursor.fetchone()[0]
                
                # 获取置信度阈值
                threshold = self._get_confidence_threshold(sample_count)
                
                # 如果置信度低于阈值，建议探索
                should_explore = confidence < threshold
                
                if should_explore:
                    print(f"🔍 建议探索: {source_format}->{target_format}, "
                         f"置信度{confidence:.2f} < 阈值{threshold:.2f}")
                
                return should_explore
                
        except Exception:
            return True  # 查询失败时建议探索
    
    def _get_confidence_threshold(self, sample_count: int) -> float:
        """获取置信度阈值"""
        if sample_count < 10:
            return 0.6  # 样本少，低阈值，容易触发探索
        elif sample_count < 50:
            return 0.75
        elif sample_count < 200:
            return 0.85
        else:
            return 0.90  # 大量样本，高阈值
    
    def get_format_combinations(self) -> List[FormatCombination]:
        """获取所有已知的格式组合分析"""
        try:
            with sqlite3.connect(self.knowledge_system.database.db_path) as conn:
                cursor = conn.execute("""
                    SELECT 
                        original_format,
                        actual_format,
                        COUNT(*) as count,
                        AVG(actual_saving_percent) as avg_saving,
                        SUM(CASE WHEN validation_passed = 1 THEN 1 ELSE 0 END) as success_count,
                        AVG(CASE WHEN ssim_value > 0 THEN ssim_value * 100 ELSE 85 END) as avg_quality
                    FROM conversion_records
                    WHERE actual_output_size > 0
                    GROUP BY original_format, actual_format
                    HAVING count >= 3
                    ORDER BY count DESC
                """)
                
                combinations = []
                for row in cursor.fetchall():
                    source_format, target_format, count, avg_saving, success_count, avg_quality = row
                    
                    success_rate = success_count / count if count > 0 else 0
                    confidence = self._calculate_confidence(count, success_rate, 10.0)
                    
                    combination = FormatCombination(
                        source_format=source_format,
                        target_format=target_format,
                        sample_count=count,
                        avg_saving=avg_saving or 0.0,
                        success_count=success_count,
                        success_rate=success_rate,
                        avg_quality=int(avg_quality or 85),
                        confidence=confidence
                    )
                    combinations.append(combination)
                
                return combinations
                
        except Exception as e:
            print(f"❌ 格式组合查询失败: {e}")
            return []
    
    def recommend_best_format(self, source_format: str, 
                            quality_goal: str = "balanced") -> Optional[str]:
        """推荐最佳目标格式"""
        combinations = self.get_format_combinations()
        
        # 筛选出该源格式的组合
        source_combinations = [c for c in combinations if c.source_format == source_format]
        
        if not source_combinations:
            # 没有历史数据，返回默认推荐
            defaults = {
                "jpeg": "webp",
                "png": "webp", 
                "webp": "avif",
                "bmp": "webp",
                "tiff": "jxl"
            }
            return defaults.get(source_format, "webp")
        
        # 根据质量目标选择最佳格式
        if quality_goal == "size":
            # 优先节省空间
            best = max(source_combinations, key=lambda c: c.avg_saving * c.confidence)
        elif quality_goal == "quality":
            # 优先质量
            best = max(source_combinations, key=lambda c: c.avg_quality * c.confidence)
        else:
            # 平衡模式：综合评分
            best = max(source_combinations, 
                      key=lambda c: (c.avg_saving * 0.5 + c.avg_quality * 0.5) * c.confidence)
        
        print(f"🎯 推荐格式: {source_format} -> {best.target_format} "
             f"(节省{best.avg_saving:.1f}%, 质量{best.avg_quality}, 置信度{best.confidence:.2f})")
        
        return best.target_format
    
    def _get_cached(self, cache_key: str) -> Optional[CachedTuning]:
        """获取缓存"""
        cached = self.cache.get(cache_key)
        if not cached:
            return None
        
        if cached.is_expired(self.cache_ttl_seconds):
            del self.cache[cache_key]
            return None
        
        cached.hit_count += 1
        return cached
    
    def _set_cached(self, cache_key: str, params: TunedParams):
        """设置缓存"""
        self.cache[cache_key] = CachedTuning(
            params=params,
            cached_at=datetime.now(),
            hit_count=0
        )
        
        # 定期清理过期缓存
        self._cleanup_expired_cache()
    
    def _cleanup_expired_cache(self):
        """清理过期缓存"""
        now = datetime.now()
        
        # 每小时清理一次
        if (now - self._stats["last_cleanup"]).total_seconds() < 3600:
            return
        
        expired_keys = []
        for key, cached in self.cache.items():
            if cached.is_expired(self.cache_ttl_seconds):
                expired_keys.append(key)
        
        for key in expired_keys:
            del self.cache[key]
        
        self._stats["last_cleanup"] = now
        
        if expired_keys:
            print(f"🧹 清理过期缓存: {len(expired_keys)}项")
    
    def clear_cache(self):
        """清除所有缓存"""
        with self._lock:
            self.cache.clear()
            print("🧹 缓存已清空")
    
    def get_cache_stats(self) -> Dict[str, Any]:
        """获取缓存统计"""
        with self._lock:
            total_hits = sum(cached.hit_count for cached in self.cache.values())
            hit_rate = (self._stats["cache_hits"] / max(1, self._stats["total_queries"])) * 100
            
            return {
                "cache_size": len(self.cache),
                "total_hits": total_hits,
                "cache_hit_rate": f"{hit_rate:.1f}%",
                "total_queries": self._stats["total_queries"],
                "cache_hits": self._stats["cache_hits"],
                "cache_misses": self._stats["cache_misses"],
                "tuning_computations": self._stats["tuning_computations"],
                "cache_ttl_hours": self.cache_ttl_seconds / 3600
            }
    
    def get_tuning_report(self) -> Dict[str, Any]:
        """生成微调报告"""
        combinations = self.get_format_combinations()
        cache_stats = self.get_cache_stats()
        
        # 按成功率排序
        top_combinations = sorted(combinations, 
                                key=lambda c: c.success_rate * c.confidence, 
                                reverse=True)[:10]
        
        return {
            "timestamp": datetime.now().isoformat(),
            "total_format_combinations": len(combinations),
            "top_performing_combinations": [
                {
                    "format": f"{c.source_format} -> {c.target_format}",
                    "success_rate": f"{c.success_rate:.1%}",
                    "avg_saving": f"{c.avg_saving:.1f}%",
                    "sample_count": c.sample_count,
                    "confidence": f"{c.confidence:.2f}"
                }
                for c in top_combinations
            ],
            "cache_performance": cache_stats,
            "recommendations": {
                "highly_confident": len([c for c in combinations if c.confidence > 0.8]),
                "needs_exploration": len([c for c in combinations if c.confidence < 0.6]),
                "total_samples": sum(c.sample_count for c in combinations)
            }
        }


# 全局微调器实例
_global_prediction_tuner: Optional[PredictionTuner] = None
_tuner_lock = threading.Lock()

def get_prediction_tuner() -> PredictionTuner:
    """获取全局预测微调器实例（单例模式）"""
    global _global_prediction_tuner
    
    with _tuner_lock:
        if _global_prediction_tuner is None:
            _global_prediction_tuner = PredictionTuner()
        return _global_prediction_tuner
