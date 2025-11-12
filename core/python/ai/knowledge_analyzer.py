"""
知识分析器 - 预测准确性分析系统

基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/knowledge/analyzer.go 重新实现

核心功能:
- 预测准确性深度分析（平均误差、中位数误差、异常检测）
- 空间节省效果评估（预测vs实际对比）
- 质量分析系统（PSNR/SSIM阈值评估）
- 格式对比分析（PNG/JPG/WebP/AVIF全格式）
- 智能建议系统（参数优化建议生成）
- 异常案例检测（性能瓶颈识别）

EX-016实现: 从Go废弃代码价值提取
遵循四高原则：高规范化、高兼容性、高扩展性、高稳定性
"""

import sqlite3
import statistics
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass
from pathlib import Path
import logging


@dataclass
class AnalysisResult:
    """分析结果数据结构"""
    predictor_name: str
    prediction_rule: str
    original_format: str
    
    # 样本统计
    total_samples: int = 0
    successful_samples: int = 0
    success_rate: float = 0.0
    
    # 预测准确性
    avg_prediction_error: float = 0.0
    median_prediction_error: float = 0.0
    max_prediction_error: float = 0.0
    min_prediction_error: float = 0.0
    
    # 空间节省
    avg_predicted_saving: float = 0.0
    avg_actual_saving: float = 0.0
    saving_difference: float = 0.0  # predicted - actual
    
    # 质量
    perfect_quality_count: int = 0
    perfect_quality_rate: float = 0.0
    good_quality_count: int = 0  # PSNR > 40 或 SSIM > 0.95
    good_quality_rate: float = 0.0
    
    # 性能
    avg_conversion_time_ms: int = 0
    
    # 建议
    recommendations: List[str] = None
    
    def __post_init__(self):
        if self.recommendations is None:
            self.recommendations = []


@dataclass
class AnomalyCase:
    """异常案例"""
    id: int
    file_path: str
    predicted_value: float
    actual_value: float
    error_magnitude: float
    error_type: str


class KnowledgeAnalyzer:
    """
    知识分析器 - 预测准确性分析系统
    
    高规范化、高兼容性、高扩展性、高稳定性实现
    """
    
    def __init__(self, 
                 db_path: str = "data/knowledge.db", 
                 debug: bool = False):
        """
        初始化知识分析器
        
        Args:
            db_path: 知识库数据库路径
            debug: 调试模式
        """
        self.db_path = Path(db_path)
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        if debug:
            self.logger.setLevel(logging.DEBUG)
            self.logger.debug(f"知识分析器初始化，数据库: {db_path}")
        
        # 确保数据库存在
        self._ensure_database()
    
    def _ensure_database(self) -> None:
        """确保数据库和表结构存在"""
        self.db_path.parent.mkdir(parents=True, exist_ok=True)
        
        with sqlite3.connect(self.db_path) as conn:
            # 创建转换记录表（如果不存在）
            conn.execute("""
                CREATE TABLE IF NOT EXISTS conversion_records (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
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
                    
                    predictor_name TEXT NOT NULL,
                    prediction_rule TEXT,
                    prediction_confidence REAL,
                    prediction_time_ms INTEGER,
                    
                    predicted_format TEXT,
                    predicted_lossless BOOLEAN,
                    predicted_distance REAL,
                    predicted_effort INTEGER,
                    predicted_lossless_jpeg BOOLEAN,
                    predicted_crf INTEGER,
                    predicted_speed INTEGER,
                    predicted_saving_percent REAL,
                    predicted_output_size INTEGER,
                    
                    actual_format TEXT,
                    actual_output_size INTEGER,
                    actual_conversion_time_ms INTEGER,
                    actual_saving_percent REAL,
                    actual_saving_bytes INTEGER,
                    
                    validation_method TEXT,
                    validation_passed BOOLEAN,
                    pixel_diff_percent REAL,
                    psnr_value REAL,
                    ssim_value REAL,
                    prediction_error_percent REAL,
                    was_explored BOOLEAN,
                    
                    user_rating INTEGER,
                    user_comment TEXT,
                    
                    pixly_version TEXT,
                    host_os TEXT
                )
            """)
            
            # 创建索引优化查询性能
            conn.execute("CREATE INDEX IF NOT EXISTS idx_predictor_rule_format ON conversion_records(predictor_name, prediction_rule, original_format)")
            conn.execute("CREATE INDEX IF NOT EXISTS idx_validation_passed ON conversion_records(validation_passed)")
            conn.execute("CREATE INDEX IF NOT EXISTS idx_prediction_error ON conversion_records(prediction_error_percent)")
    
    def analyze_predictor(self, 
                         predictor_name: str, 
                         rule: str, 
                         format: str) -> Optional[AnalysisResult]:
        """
        分析特定预测器的准确性
        
        Args:
            predictor_name: 预测器名称
            rule: 预测规则
            format: 原始格式
            
        Returns:
            分析结果，如果没有数据则返回None
        """
        query = """
            SELECT 
                COUNT(*) as total,
                SUM(CASE WHEN validation_passed = 1 THEN 1 ELSE 0 END) as successful,
                AVG(prediction_error_percent) as avg_error,
                MAX(prediction_error_percent) as max_error,
                MIN(prediction_error_percent) as min_error,
                AVG(predicted_saving_percent) as avg_pred_saving,
                AVG(actual_saving_percent) as avg_actual_saving,
                SUM(CASE WHEN pixel_diff_percent = 0 THEN 1 ELSE 0 END) as perfect_count,
                SUM(CASE WHEN psnr_value > 40 OR ssim_value > 0.95 THEN 1 ELSE 0 END) as good_count,
                AVG(actual_conversion_time_ms) as avg_time
            FROM conversion_records
            WHERE predictor_name = ? AND prediction_rule = ? AND original_format = ?
        """
        
        try:
            with sqlite3.connect(self.db_path) as conn:
                cursor = conn.execute(query, (predictor_name, rule, format))
                row = cursor.fetchone()
                
                if not row or row[0] == 0:  # total = 0
                    if self.debug:
                        self.logger.debug(f"没有找到样本数据: {predictor_name}/{rule}/{format}")
                    return None
                
                # 解析查询结果
                (total, successful, avg_error, max_error, min_error, 
                 avg_pred_saving, avg_actual_saving, perfect_count, 
                 good_count, avg_time) = row
                
                result = AnalysisResult(
                    predictor_name=predictor_name,
                    prediction_rule=rule,
                    original_format=format,
                    total_samples=total,
                    successful_samples=successful,
                    success_rate=successful / total if total > 0 else 0.0,
                    avg_prediction_error=avg_error or 0.0,
                    max_prediction_error=max_error or 0.0,
                    min_prediction_error=min_error or 0.0,
                    avg_predicted_saving=(avg_pred_saving or 0.0) / 100.0,
                    avg_actual_saving=(avg_actual_saving or 0.0) / 100.0,
                    perfect_quality_count=perfect_count,
                    perfect_quality_rate=perfect_count / total if total > 0 else 0.0,
                    good_quality_count=good_count,
                    good_quality_rate=good_count / total if total > 0 else 0.0,
                    avg_conversion_time_ms=int(avg_time or 0)
                )
                
                # 计算空间节省差异
                result.saving_difference = result.avg_predicted_saving - result.avg_actual_saving
                
                # 计算中位数误差
                result.median_prediction_error = self._calculate_median_error(
                    predictor_name, rule, format
                )
                
                # 生成建议
                result.recommendations = self._generate_recommendations(result)
                
                if self.debug:
                    self.logger.debug(f"预测器分析完成: {predictor_name}, 样本数: {total}, "
                                    f"平均误差: {result.avg_prediction_error:.2%}")
                
                return result
                
        except Exception as e:
            self.logger.error(f"分析预测器失败: {e}")
            return None
    
    def _calculate_median_error(self, predictor_name: str, rule: str, format: str) -> float:
        """计算预测误差中位数"""
        query = """
            SELECT prediction_error_percent
            FROM conversion_records
            WHERE predictor_name = ? AND prediction_rule = ? AND original_format = ?
              AND prediction_error_percent IS NOT NULL
            ORDER BY prediction_error_percent
        """
        
        try:
            with sqlite3.connect(self.db_path) as conn:
                cursor = conn.execute(query, (predictor_name, rule, format))
                errors = [row[0] for row in cursor.fetchall()]
                
                return statistics.median(errors) if errors else 0.0
                
        except Exception:
            return 0.0
    
    def _generate_recommendations(self, result: AnalysisResult) -> List[str]:
        """生成优化建议"""
        recommendations = []
        
        # 预测准确性建议
        if result.avg_prediction_error > 0.20:
            recommendations.append(
                f"⚠️ 预测误差较大({result.avg_prediction_error:.1%})，建议调整预测参数"
            )
        elif result.avg_prediction_error < 0.05:
            recommendations.append(
                f"✅ 预测准确性优秀(误差仅{result.avg_prediction_error:.1%})"
            )
        
        # 空间节省建议
        if result.saving_difference > 0.10:
            recommendations.append(
                f"📊 预测过于乐观，实际节省比预测少{result.saving_difference:.1%}"
            )
        elif result.saving_difference < -0.10:
            recommendations.append(
                f"🎯 预测过于保守，实际节省比预测多{-result.saving_difference:.1%}"
            )
        
        # 质量建议
        if result.perfect_quality_rate >= 0.95:
            recommendations.append(
                f"🏆 质量完美率{result.perfect_quality_rate:.1%}，无损转换非常稳定"
            )
        elif result.perfect_quality_rate < 0.80:
            recommendations.append(
                f"⚠️ 完美质量率仅{result.perfect_quality_rate:.1%}，建议检查转换参数"
            )
        
        # 成功率建议
        if result.success_rate < 0.90:
            recommendations.append(
                f"❌ 成功率偏低({result.success_rate:.1%})，需要优化转换流程"
            )
        
        # 性能建议
        if result.avg_conversion_time_ms > 10000:
            recommendations.append(
                f"⏱️ 平均转换时间较长({result.avg_conversion_time_ms/1000:.1f}s)，考虑优化参数"
            )
        
        return recommendations
    
    def compare_formats(self) -> Dict[str, AnalysisResult]:
        """
        对比不同格式的转换效果
        
        Returns:
            格式对比结果字典
        """
        formats = ["png", "jpg", "jpeg", "gif", "webp", "avif", "jxl", "heic"]
        results = {}
        
        for format_name in formats:
            # 查询该格式的主要预测规则
            query = """
                SELECT predictor_name, prediction_rule, COUNT(*) as count
                FROM conversion_records
                WHERE original_format = ?
                GROUP BY predictor_name, prediction_rule
                ORDER BY count DESC
                LIMIT 1
            """
            
            try:
                with sqlite3.connect(self.db_path) as conn:
                    cursor = conn.execute(query, (format_name,))
                    row = cursor.fetchone()
                    
                    if row:
                        predictor_name, rule, count = row
                        result = self.analyze_predictor(predictor_name, rule, format_name)
                        if result:
                            results[format_name] = result
                            
            except Exception as e:
                if self.debug:
                    self.logger.debug(f"对比格式{format_name}失败: {e}")
                continue
        
        return results
    
    def get_top_anomalies(self, limit: int = 10) -> List[AnomalyCase]:
        """
        获取最严重的异常案例
        
        Args:
            limit: 最大返回数量
            
        Returns:
            异常案例列表
        """
        query = """
            SELECT id, file_path, predicted_saving_percent, actual_saving_percent,
                   prediction_error_percent
            FROM conversion_records
            WHERE prediction_error_percent IS NOT NULL
            ORDER BY prediction_error_percent DESC
            LIMIT ?
        """
        
        anomalies = []
        
        try:
            with sqlite3.connect(self.db_path) as conn:
                cursor = conn.execute(query, (limit,))
                
                for row in cursor.fetchall():
                    anomaly = AnomalyCase(
                        id=row[0],
                        file_path=row[1],
                        predicted_value=row[2] or 0.0,
                        actual_value=row[3] or 0.0,
                        error_magnitude=row[4] or 0.0,
                        error_type="prediction_error"
                    )
                    anomalies.append(anomaly)
                    
        except Exception as e:
            self.logger.error(f"获取异常案例失败: {e}")
        
        return anomalies
    
    def generate_report(self) -> str:
        """
        生成完整的分析报告
        
        Returns:
            报告文本
        """
        # 获取总体统计
        summary = self.get_stats_summary()
        
        # 对比不同格式
        format_results = self.compare_formats()
        
        # 生成报告
        report = "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
        report += "📊 Pixly v3.0 知识库分析报告\n"
        report += "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n"
        
        # 总体统计
        report += "🎯 总体统计:\n"
        report += f"  总转换次数: {summary.get('total_conversions', 0)}\n"
        report += f"  平均空间节省: {summary.get('avg_saving_percent', 0):.1f}%\n"
        report += f"  质量通过率: {summary.get('quality_pass_rate', 0):.1f}%\n"
        report += f"  平均预测误差: {summary.get('avg_prediction_error', 0):.1f}%\n\n"
        
        # 各格式详情
        report += "📈 各格式转换效果:\n\n"
        for format_name, result in format_results.items():
            report += f"  [{format_name.upper()}]\n"
            report += f"    样本数: {result.total_samples}\n"
            report += f"    预测误差: {result.avg_prediction_error:.1%} (中位数: {result.median_prediction_error:.1%})\n"
            report += f"    空间节省: {result.avg_actual_saving:.1%} (预测: {result.avg_predicted_saving:.1%})\n"
            report += f"    完美质量率: {result.perfect_quality_rate:.1%}\n"
            
            if result.recommendations:
                report += "    建议:\n"
                for rec in result.recommendations:
                    report += f"      {rec}\n"
            report += "\n"
        
        report += "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
        
        return report
    
    def get_stats_summary(self) -> Dict[str, Any]:
        """获取统计摘要"""
        query = """
            SELECT 
                COUNT(*) as total_conversions,
                AVG(actual_saving_percent) as avg_saving_percent,
                AVG(CASE WHEN validation_passed = 1 THEN 1.0 ELSE 0.0 END) * 100 as quality_pass_rate,
                AVG(prediction_error_percent) as avg_prediction_error
            FROM conversion_records
        """
        
        try:
            with sqlite3.connect(self.db_path) as conn:
                cursor = conn.execute(query)
                row = cursor.fetchone()
                
                if row:
                    return {
                        'total_conversions': row[0] or 0,
                        'avg_saving_percent': row[1] or 0.0,
                        'quality_pass_rate': row[2] or 0.0,
                        'avg_prediction_error': row[3] or 0.0
                    }
                    
        except Exception as e:
            self.logger.error(f"获取统计摘要失败: {e}")
        
        return {}
    
    def optimize_prediction(self, format: str) -> Dict[str, Any]:
        """
        根据历史数据优化预测
        
        Args:
            format: 格式名称
            
        Returns:
            优化建议参数
        """
        query = """
            SELECT 
                AVG(actual_saving_percent) as optimal_saving,
                AVG(predicted_effort) as optimal_effort,
                AVG(predicted_distance) as optimal_distance,
                AVG(predicted_crf) as optimal_crf
            FROM conversion_records
            WHERE original_format = ? 
              AND validation_passed = 1
              AND actual_saving_percent > 0
        """
        
        try:
            with sqlite3.connect(self.db_path) as conn:
                cursor = conn.execute(query, (format,))
                row = cursor.fetchone()
                
                if row:
                    return {
                        'format': format,
                        'optimal_saving': row[0] or 0.0,
                        'optimal_effort': int(row[1] or 7),
                        'optimal_distance': row[2] or 1.0,
                        'optimal_crf': int(row[3] or 23)
                    }
                    
        except Exception as e:
            self.logger.error(f"优化预测失败: {e}")
        
        return {}


if __name__ == "__main__":
    # 测试代码
    print("=== 知识分析器测试 ===")
    
    analyzer = KnowledgeAnalyzer("data/test_knowledge.db", debug=True)
    
    # 测试预测器分析
    result = analyzer.analyze_predictor("lightgbm", "auto", "jpg")
    if result:
        print(f"✅ 分析结果: 样本数={result.total_samples}, "
              f"成功率={result.success_rate:.2%}, "
              f"平均误差={result.avg_prediction_error:.2%}")
    else:
        print("⚠️ 没有找到分析数据")
    
    # 测试格式对比
    formats = analyzer.compare_formats()
    print(f"✅ 格式对比: 找到{len(formats)}个格式数据")
    
    # 测试异常案例
    anomalies = analyzer.get_top_anomalies(5)
    print(f"✅ 异常案例: 找到{len(anomalies)}个异常案例")
    
    print("🎯 知识分析器测试完成！")
