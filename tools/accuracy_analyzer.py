#!/usr/bin/env python3
"""
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 47.23: 预测准确性分析器
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

功能：
- 分析预测器准确性统计
- 计算平均预测误差
- 评估空间节省效果
- 质量保持率分析
- 生成性能改进建议

从Go代码提取：core/@deprecated/go_ai_service_2025_11_11/ai 2/knowledge/analyzer.go
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"""

import sys
from pathlib import Path
from dataclasses import dataclass, field
from typing import List, Dict, Any

# 导入反馈数据库
sys.path.insert(0, str(Path(__file__).parent.parent / 'core' / 'python'))
from ai.feedback_db import FeedbackDB


@dataclass
class AnalysisResult:
    """分析结果数据结构"""
    # 基本信息
    model_type: str
    model_version: str
    
    # 样本统计
    total_samples: int = 0
    successful_samples: int = 0
    success_rate: float = 0.0
    
    # 预测准确性
    avg_prediction_error: float = 0.0  # 平均预测误差（百分比）
    max_prediction_error: float = 0.0
    min_prediction_error: float = 0.0
    
    # 空间节省
    avg_predicted_saving: float = 0.0
    avg_actual_saving: float = 0.0
    saving_difference: float = 0.0  # predicted - actual
    
    # 质量
    perfect_quality_count: int = 0  # 100%完美
    perfect_quality_rate: float = 0.0
    good_quality_count: int = 0     # quality > 0.95
    good_quality_rate: float = 0.0
    
    # 性能
    avg_processing_time: float = 0.0
    
    # 建议
    recommendations: List[str] = field(default_factory=list)


class AccuracyAnalyzer:
    """预测准确性分析器"""
    
    def __init__(self, feedback_db: FeedbackDB):
        """
        初始化分析器
        
        Args:
            feedback_db: 反馈数据库实例
        """
        self.db = feedback_db
    
    def analyze_model(self, model_type: str, model_version: str = None) -> AnalysisResult:
        """
        分析模型性能
        
        Args:
            model_type: 模型类型
            model_version: 模型版本（可选）
            
        Returns:
            分析结果
        """
        # 获取所有成功的反馈记录
        records = self.db.get_unused_feedback(model_type, limit=10000)
        
        # 如果指定版本，过滤
        if model_version:
            records = [r for r in records if r.model_version == model_version]
        
        if not records:
            return AnalysisResult(
                model_type=model_type,
                model_version=model_version or "all"
            )
        
        # 初始化结果
        result = AnalysisResult(
            model_type=model_type,
            model_version=model_version or "all",
            total_samples=len(records)
        )
        
        # 统计数据
        successful = [r for r in records if r.success]
        result.successful_samples = len(successful)
        result.success_rate = len(successful) / len(records) if records else 0.0
        
        # 计算误差和节省
        errors = []
        predicted_savings = []
        actual_savings = []
        quality_scores = []
        processing_times = []
        
        for record in successful:
            # 计算预测误差（如果有实际参数）
            if record.actual_params and record.predicted_params:
                # 简化：基于quality参数的误差
                pred_q = record.predicted_params.get('quality', 0)
                actual_q = record.actual_params.get('quality', 0)
                if pred_q and actual_q:
                    error = abs(pred_q - actual_q) / actual_q * 100 if actual_q else 0
                    errors.append(error)
            
            # 空间节省（需要原始大小和实际大小）
            if record.input_features and record.actual_size:
                original_size = record.input_features.get('file_size', 0)
                if original_size:
                    actual_saving = (1 - record.actual_size / original_size) * 100
                    actual_savings.append(actual_saving)
            
            # 质量分数
            if record.actual_quality:
                quality_scores.append(record.actual_quality)
                if record.actual_quality >= 0.99:
                    result.perfect_quality_count += 1
                if record.actual_quality >= 0.95:
                    result.good_quality_count += 1
            
            # 处理时间
            if record.processing_time:
                processing_times.append(record.processing_time)
        
        # 计算统计数据
        if errors:
            result.avg_prediction_error = sum(errors) / len(errors)
            result.max_prediction_error = max(errors)
            result.min_prediction_error = min(errors)
        
        if actual_savings:
            result.avg_actual_saving = sum(actual_savings) / len(actual_savings)
        
        if quality_scores:
            result.perfect_quality_rate = result.perfect_quality_count / len(quality_scores)
            result.good_quality_rate = result.good_quality_count / len(quality_scores)
        
        if processing_times:
            result.avg_processing_time = sum(processing_times) / len(processing_times)
        
        # 生成建议
        result.recommendations = self._generate_recommendations(result)
        
        return result
    
    def _generate_recommendations(self, result: AnalysisResult) -> List[str]:
        """
        生成性能改进建议
        
        Args:
            result: 分析结果
            
        Returns:
            建议列表
        """
        recommendations = []
        
        # 成功率建议
        if result.success_rate < 0.9:
            recommendations.append(f"⚠️  成功率偏低 ({result.success_rate:.1%})，建议检查错误日志")
        
        # 预测误差建议
        if result.avg_prediction_error > 10:
            recommendations.append(f"⚠️  平均预测误差较大 ({result.avg_prediction_error:.1f}%)，建议重新训练模型")
        elif result.avg_prediction_error > 5:
            recommendations.append(f"💡 预测误差中等 ({result.avg_prediction_error:.1f}%)，可考虑收集更多训练数据")
        
        # 质量保持率建议
        if result.good_quality_rate < 0.95:
            recommendations.append(f"⚠️  优质输出率偏低 ({result.good_quality_rate:.1%})，建议调整质量参数")
        
        # 性能建议
        if result.avg_processing_time > 5.0:
            recommendations.append(f"💡 平均处理时间较长 ({result.avg_processing_time:.2f}s)，可考虑优化")
        
        # 空间节省建议
        if result.avg_actual_saving < 30:
            recommendations.append(f"💡 空间节省率较低 ({result.avg_actual_saving:.1f}%)，可尝试更激进的压缩策略")
        
        if not recommendations:
            recommendations.append("✅ 模型性能良好，无需调整")
        
        return recommendations


# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 测试和演示
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

if __name__ == "__main__":
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print("Phase 47.23: 预测准确性分析器测试")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
    
    # 创建测试数据库（使用之前的测试数据）
    db = FeedbackDB("data/test_feedback.db")
    analyzer = AccuracyAnalyzer(db)
    
    # 分析模型
    print("1️⃣  分析模型性能...")
    result = analyzer.analyze_model("lightgbm", "v1.0.0")
    
    print(f"\n📊 分析结果:")
    print(f"   模型: {result.model_type} {result.model_version}")
    print(f"   总样本数: {result.total_samples}")
    print(f"   成功样本数: {result.successful_samples}")
    print(f"   成功率: {result.success_rate:.2%}")
    
    if result.avg_prediction_error > 0:
        print(f"\n📈 预测准确性:")
        print(f"   平均误差: {result.avg_prediction_error:.2f}%")
        print(f"   最大误差: {result.max_prediction_error:.2f}%")
        print(f"   最小误差: {result.min_prediction_error:.2f}%")
    
    if result.avg_actual_saving > 0:
        print(f"\n💾 空间节省:")
        print(f"   实际节省: {result.avg_actual_saving:.2f}%")
    
    if result.perfect_quality_count > 0:
        print(f"\n✨ 质量统计:")
        print(f"   完美质量: {result.perfect_quality_count} ({result.perfect_quality_rate:.2%})")
        print(f"   优质输出: {result.good_quality_count} ({result.good_quality_rate:.2%})")
    
    if result.avg_processing_time > 0:
        print(f"\n⏱️  性能:")
        print(f"   平均处理时间: {result.avg_processing_time:.3f}s")
    
    print(f"\n💡 建议:")
    for rec in result.recommendations:
        print(f"   {rec}")
    
    db.close()
    
    print("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print("✅ 预测准确性分析器测试完成")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
