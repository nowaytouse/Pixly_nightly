"""
性能优化器 - Rust-Python融合的智能优化

自动分析和优化融合架构的性能：
- 实时性能监控
- 智能路由优化
- 资源使用分析
- 性能预测和建议

通过机器学习不断优化Rust和Python的任务分配
"""

import time
import threading
import statistics
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, field
from datetime import datetime, timedelta
from enum import Enum
from collections import deque, defaultdict
import numpy as np
import logging


class OptimizationLevel(Enum):
    """优化级别"""
    CONSERVATIVE = "conservative"    # 保守优化
    BALANCED = "balanced"           # 平衡优化
    AGGRESSIVE = "aggressive"       # 激进优化


@dataclass
class PerformanceMetric:
    """性能指标"""
    timestamp: datetime
    processor_type: str  # 'rust', 'python', 'hybrid'
    operation_type: str
    data_size: int
    processing_time_ms: float
    memory_usage_mb: float = 0.0
    cpu_usage_percent: float = 0.0
    success: bool = True
    
    # 计算性能分数 (越高越好)
    @property
    def performance_score(self) -> float:
        if not self.success:
            return 0.0
        
        # 基于处理时间和数据大小的效率分数
        throughput = self.data_size / (self.processing_time_ms + 1)  # 避免除零
        base_score = min(100.0, throughput * 1000)  # 标准化到100分
        
        # 内存和CPU惩罚
        memory_penalty = min(10.0, self.memory_usage_mb / 100.0)
        cpu_penalty = min(10.0, self.cpu_usage_percent / 100.0 * 10)
        
        return max(0.0, base_score - memory_penalty - cpu_penalty)


@dataclass 
class OptimizationRule:
    """优化规则"""
    name: str
    condition: str  # 条件表达式
    recommendation: str  # 推荐处理器
    confidence: float = 0.8  # 规则置信度
    priority: int = 1  # 规则优先级


class PerformancePredictor:
    """性能预测器"""
    
    def __init__(self, history_size: int = 1000):
        """
        初始化性能预测器
        
        Args:
            history_size: 历史记录大小
        """
        self.history_size = history_size
        
        # 按处理器类型和操作类型分组的历史数据
        self._metrics_history: Dict[str, deque] = defaultdict(lambda: deque(maxlen=history_size))
        
        # 预测模型参数 (简单线性模型)
        self._model_params: Dict[str, Dict[str, float]] = defaultdict(lambda: {
            'time_intercept': 1.0,
            'time_slope': 0.001,
            'memory_base': 10.0,
            'memory_factor': 0.0001
        })
        
        self._lock = threading.RLock()
    
    def add_metric(self, metric: PerformanceMetric):
        """添加性能指标"""
        key = f"{metric.processor_type}_{metric.operation_type}"
        
        with self._lock:
            self._metrics_history[key].append(metric)
            
            # 当有足够数据时更新模型
            if len(self._metrics_history[key]) >= 10:
                self._update_model(key)
    
    def predict_performance(self, 
                          processor_type: str,
                          operation_type: str,
                          data_size: int) -> Dict[str, float]:
        """
        预测性能
        
        Args:
            processor_type: 处理器类型
            operation_type: 操作类型
            data_size: 数据大小
            
        Returns:
            预测结果: {'time_ms', 'memory_mb', 'score'}
        """
        key = f"{processor_type}_{operation_type}"
        
        with self._lock:
            params = self._model_params[key]
            
            # 预测处理时间
            predicted_time = params['time_intercept'] + params['time_slope'] * data_size
            
            # 预测内存使用
            predicted_memory = params['memory_base'] + params['memory_factor'] * data_size
            
            # 计算预测性能分数
            throughput = data_size / (predicted_time + 1)
            predicted_score = min(100.0, throughput * 1000)
            
            return {
                'time_ms': predicted_time,
                'memory_mb': predicted_memory,
                'score': predicted_score,
                'confidence': self._get_prediction_confidence(key)
            }
    
    def _update_model(self, key: str):
        """更新预测模型"""
        metrics = list(self._metrics_history[key])
        
        if len(metrics) < 5:
            return
        
        # 提取数据
        data_sizes = [m.data_size for m in metrics]
        processing_times = [m.processing_time_ms for m in metrics]
        memory_usages = [m.memory_usage_mb for m in metrics]
        
        # 简单线性回归
        if len(set(data_sizes)) > 1:  # 确保有变化
            # 时间预测模型
            mean_size = statistics.mean(data_sizes)
            mean_time = statistics.mean(processing_times)
            
            numerator = sum((s - mean_size) * (t - mean_time) for s, t in zip(data_sizes, processing_times))
            denominator = sum((s - mean_size) ** 2 for s in data_sizes)
            
            if denominator > 0:
                slope = numerator / denominator
                intercept = mean_time - slope * mean_size
                
                self._model_params[key]['time_slope'] = slope
                self._model_params[key]['time_intercept'] = max(0.1, intercept)
            
            # 内存预测模型
            if memory_usages and any(m > 0 for m in memory_usages):
                mean_memory = statistics.mean(memory_usages)
                mem_numerator = sum((s - mean_size) * (m - mean_memory) for s, m in zip(data_sizes, memory_usages))
                
                if denominator > 0:
                    mem_slope = mem_numerator / denominator
                    mem_intercept = mean_memory - mem_slope * mean_size
                    
                    self._model_params[key]['memory_factor'] = max(0, mem_slope)
                    self._model_params[key]['memory_base'] = max(1.0, mem_intercept)
    
    def _get_prediction_confidence(self, key: str) -> float:
        """获取预测置信度"""
        history_length = len(self._metrics_history[key])
        
        if history_length < 5:
            return 0.2
        elif history_length < 20:
            return 0.5
        elif history_length < 50:
            return 0.7
        else:
            return 0.9
    
    def get_best_processor(self, 
                          operation_type: str,
                          data_size: int,
                          processors: List[str] = ['rust', 'python', 'hybrid']) -> Tuple[str, float]:
        """
        获取最佳处理器
        
        Args:
            operation_type: 操作类型
            data_size: 数据大小
            processors: 可选处理器列表
            
        Returns:
            (best_processor, confidence)
        """
        best_processor = 'python'
        best_score = 0.0
        best_confidence = 0.0
        
        for processor in processors:
            prediction = self.predict_performance(processor, operation_type, data_size)
            
            if prediction['score'] > best_score:
                best_processor = processor
                best_score = prediction['score']
                best_confidence = prediction['confidence']
        
        return best_processor, best_confidence


class PerformanceOptimizer:
    """
    性能优化器 - 智能优化Rust-Python融合性能
    
    持续学习和优化系统性能
    """
    
    def __init__(self, 
                 optimization_level: OptimizationLevel = OptimizationLevel.BALANCED,
                 enable_learning: bool = True,
                 debug: bool = False):
        """
        初始化性能优化器
        
        Args:
            optimization_level: 优化级别
            enable_learning: 启用机器学习
            debug: 调试模式
        """
        self.optimization_level = optimization_level
        self.enable_learning = enable_learning
        self.debug = debug
        
        # 性能预测器
        self.predictor = PerformancePredictor()
        
        # 性能统计
        self._performance_stats = {
            'total_optimizations': 0,
            'successful_optimizations': 0,
            'performance_improvement_percent': 0.0,
            'last_optimization': None
        }
        
        # 优化规则
        self._optimization_rules: List[OptimizationRule] = []
        self._load_default_rules()
        
        # 实时监控
        self._monitoring_active = True
        self._monitoring_thread: Optional[threading.Thread] = None
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 日志
        self.logger = logging.getLogger(__name__)
        
        if debug:
            self.logger.debug("性能优化器初始化完成")
    
    def _load_default_rules(self):
        """加载默认优化规则"""
        default_rules = [
            OptimizationRule(
                name="large_image_rust",
                condition="operation_type.startswith('image_') and data_size > 1000000",
                recommendation="rust",
                confidence=0.9,
                priority=1
            ),
            OptimizationRule(
                name="ai_inference_python",
                condition="operation_type.startswith('ai_') or 'analyze' in operation_type",
                recommendation="python",
                confidence=0.85,
                priority=2
            ),
            OptimizationRule(
                name="small_data_python",
                condition="data_size < 10000",
                recommendation="python",
                confidence=0.7,
                priority=3
            ),
            OptimizationRule(
                name="batch_processing_hybrid",
                condition="operation_type.startswith('batch_')",
                recommendation="hybrid",
                confidence=0.8,
                priority=2
            ),
            OptimizationRule(
                name="real_time_rust",
                condition="'real_time' in operation_type or 'urgent' in operation_type",
                recommendation="rust",
                confidence=0.9,
                priority=1
            )
        ]
        
        self._optimization_rules.extend(default_rules)
    
    def optimize_processor_selection(self,
                                   operation_type: str,
                                   data_size: int,
                                   current_processor: Optional[str] = None) -> Dict[str, Any]:
        """
        优化处理器选择
        
        Args:
            operation_type: 操作类型
            data_size: 数据大小
            current_processor: 当前处理器
            
        Returns:
            优化建议
        """
        
        optimization_result = {
            'recommended_processor': current_processor or 'python',
            'confidence': 0.5,
            'reason': 'default',
            'expected_improvement_percent': 0.0,
            'rules_applied': [],
            'prediction': None
        }
        
        try:
            # 1. 基于规则的优化
            rule_recommendation = self._apply_optimization_rules(operation_type, data_size)
            
            # 2. 基于机器学习的预测
            ml_recommendation = None
            if self.enable_learning:
                ml_recommendation = self._apply_ml_optimization(operation_type, data_size)
            
            # 3. 综合决策
            final_recommendation = self._combine_recommendations(
                rule_recommendation, 
                ml_recommendation, 
                current_processor
            )
            
            optimization_result.update(final_recommendation)
            
            # 4. 计算预期改进
            if current_processor and current_processor != final_recommendation['recommended_processor']:
                improvement = self._estimate_improvement(
                    current_processor,
                    final_recommendation['recommended_processor'],
                    operation_type,
                    data_size
                )
                optimization_result['expected_improvement_percent'] = improvement
            
            # 更新统计
            with self._lock:
                self._performance_stats['total_optimizations'] += 1
                self._performance_stats['last_optimization'] = datetime.now()
            
            if self.debug:
                self.logger.debug(f"处理器优化: {operation_type} -> {optimization_result['recommended_processor']}")
        
        except Exception as e:
            self.logger.error(f"处理器优化失败: {e}")
        
        return optimization_result
    
    def _apply_optimization_rules(self, operation_type: str, data_size: int) -> Dict[str, Any]:
        """应用优化规则"""
        
        matched_rules = []
        
        # 创建评估上下文
        context = {
            'operation_type': operation_type,
            'data_size': data_size
        }
        
        for rule in sorted(self._optimization_rules, key=lambda r: r.priority):
            try:
                # 安全评估条件
                if self._safe_eval(rule.condition, context):
                    matched_rules.append(rule)
            except Exception as e:
                self.logger.warning(f"规则评估失败 {rule.name}: {e}")
        
        if matched_rules:
            # 选择最高优先级和置信度的规则
            best_rule = max(matched_rules, key=lambda r: (r.priority, r.confidence))
            return {
                'recommended_processor': best_rule.recommendation,
                'confidence': best_rule.confidence,
                'reason': f'rule: {best_rule.name}',
                'rules_applied': [rule.name for rule in matched_rules]
            }
        
        return {
            'recommended_processor': 'python',
            'confidence': 0.3,
            'reason': 'no_matching_rules',
            'rules_applied': []
        }
    
    def _apply_ml_optimization(self, operation_type: str, data_size: int) -> Dict[str, Any]:
        """应用机器学习优化"""
        
        try:
            # 获取最佳处理器预测
            best_processor, confidence = self.predictor.get_best_processor(
                operation_type, 
                data_size,
                ['rust', 'python', 'hybrid']
            )
            
            # 获取详细预测
            prediction = self.predictor.predict_performance(
                best_processor,
                operation_type,
                data_size
            )
            
            return {
                'recommended_processor': best_processor,
                'confidence': confidence,
                'reason': 'ml_prediction',
                'prediction': prediction
            }
        
        except Exception as e:
            self.logger.error(f"ML优化失败: {e}")
            return {
                'recommended_processor': 'python',
                'confidence': 0.2,
                'reason': 'ml_error',
                'prediction': None
            }
    
    def _combine_recommendations(self,
                               rule_rec: Dict[str, Any],
                               ml_rec: Optional[Dict[str, Any]],
                               current_processor: Optional[str]) -> Dict[str, Any]:
        """综合推荐结果"""
        
        # 基于优化级别决策
        if self.optimization_level == OptimizationLevel.CONSERVATIVE:
            # 保守模式：只在高置信度时切换
            if rule_rec['confidence'] > 0.8:
                return rule_rec
            elif current_processor:
                return {
                    'recommended_processor': current_processor,
                    'confidence': 0.6,
                    'reason': 'conservative_keep_current',
                    'rules_applied': rule_rec['rules_applied']
                }
        
        elif self.optimization_level == OptimizationLevel.AGGRESSIVE:
            # 激进模式：优先ML预测
            if ml_rec and ml_rec['confidence'] > 0.4:
                return ml_rec
            elif rule_rec['confidence'] > 0.5:
                return rule_rec
        
        # 平衡模式：综合考虑规则和ML
        if ml_rec and rule_rec:
            # 如果两者推荐一致
            if ml_rec['recommended_processor'] == rule_rec['recommended_processor']:
                combined_confidence = min(0.95, (ml_rec['confidence'] + rule_rec['confidence']) / 2)
                return {
                    'recommended_processor': rule_rec['recommended_processor'],
                    'confidence': combined_confidence,
                    'reason': 'rule_ml_consensus',
                    'rules_applied': rule_rec['rules_applied'],
                    'prediction': ml_rec.get('prediction')
                }
            
            # 如果两者不一致，选择置信度更高的
            if ml_rec['confidence'] > rule_rec['confidence']:
                return ml_rec
        
        return rule_rec
    
    def _estimate_improvement(self,
                            current_processor: str,
                            recommended_processor: str,
                            operation_type: str,
                            data_size: int) -> float:
        """估算性能改进百分比"""
        
        try:
            # 预测当前处理器性能
            current_pred = self.predictor.predict_performance(
                current_processor, operation_type, data_size
            )
            
            # 预测推荐处理器性能
            recommended_pred = self.predictor.predict_performance(
                recommended_processor, operation_type, data_size
            )
            
            # 计算改进百分比
            if current_pred['time_ms'] > 0:
                time_improvement = (current_pred['time_ms'] - recommended_pred['time_ms']) / current_pred['time_ms'] * 100
                return max(0.0, min(50.0, time_improvement))  # 限制在0-50%
            
        except Exception as e:
            self.logger.error(f"改进估算失败: {e}")
        
        return 0.0
    
    def _safe_eval(self, condition: str, context: Dict[str, Any]) -> bool:
        """安全条件评估"""
        
        # 简单的条件解析，避免使用eval
        try:
            # 支持基本条件
            if 'operation_type.startswith' in condition:
                # 解析 operation_type.startswith('prefix')
                import re
                match = re.search(r"operation_type\.startswith\('([^']+)'\)", condition)
                if match:
                    prefix = match.group(1)
                    return context['operation_type'].startswith(prefix)
            
            elif 'data_size >' in condition:
                # 解析 data_size > number
                import re
                match = re.search(r'data_size > (\d+)', condition)
                if match:
                    threshold = int(match.group(1))
                    return context['data_size'] > threshold
            
            elif 'data_size <' in condition:
                # 解析 data_size < number
                import re
                match = re.search(r'data_size < (\d+)', condition)
                if match:
                    threshold = int(match.group(1))
                    return context['data_size'] < threshold
            
            elif 'in operation_type' in condition:
                # 解析 'keyword' in operation_type
                import re
                match = re.search(r"'([^']+)' in operation_type", condition)
                if match:
                    keyword = match.group(1)
                    return keyword in context['operation_type']
            
            # 组合条件 (and/or)
            if ' and ' in condition:
                parts = condition.split(' and ')
                return all(self._safe_eval(part.strip(), context) for part in parts)
            
            if ' or ' in condition:
                parts = condition.split(' or ')
                return any(self._safe_eval(part.strip(), context) for part in parts)
                
        except Exception as e:
            self.logger.warning(f"条件评估错误: {condition}, {e}")
        
        return False
    
    def record_performance(self, 
                         processor_type: str,
                         operation_type: str,
                         data_size: int,
                         processing_time_ms: float,
                         memory_usage_mb: float = 0.0,
                         success: bool = True):
        """记录性能数据"""
        
        metric = PerformanceMetric(
            timestamp=datetime.now(),
            processor_type=processor_type,
            operation_type=operation_type,
            data_size=data_size,
            processing_time_ms=processing_time_ms,
            memory_usage_mb=memory_usage_mb,
            success=success
        )
        
        if self.enable_learning:
            self.predictor.add_metric(metric)
        
        # 更新成功统计
        if success:
            with self._lock:
                self._performance_stats['successful_optimizations'] += 1
        
        if self.debug:
            self.logger.debug(f"性能记录: {processor_type} - {operation_type}, 耗时: {processing_time_ms:.3f}ms")
    
    def get_optimization_stats(self) -> Dict[str, Any]:
        """获取优化统计"""
        
        with self._lock:
            stats = dict(self._performance_stats)
            
            # 计算优化成功率
            if stats['total_optimizations'] > 0:
                stats['optimization_success_rate'] = (
                    stats['successful_optimizations'] / stats['total_optimizations'] * 100
                )
            else:
                stats['optimization_success_rate'] = 0.0
            
            return {
                'optimization_stats': stats,
                'optimization_level': self.optimization_level.value,
                'learning_enabled': self.enable_learning,
                'total_rules': len(self._optimization_rules),
                'predictor_confidence': self._get_average_prediction_confidence()
            }
    
    def _get_average_prediction_confidence(self) -> float:
        """获取平均预测置信度"""
        
        if not self.enable_learning:
            return 0.0
        
        try:
            confidences = []
            
            # 测试几个典型场景的预测置信度
            test_scenarios = [
                ('image_compress', 1000000),
                ('ai_inference', 50000),
                ('batch_process', 500000)
            ]
            
            for op_type, data_size in test_scenarios:
                _, confidence = self.predictor.get_best_processor(op_type, data_size)
                confidences.append(confidence)
            
            return statistics.mean(confidences) if confidences else 0.0
            
        except Exception:
            return 0.0
    
    def add_optimization_rule(self, rule: OptimizationRule):
        """添加自定义优化规则"""
        
        with self._lock:
            self._optimization_rules.append(rule)
            
            # 按优先级排序
            self._optimization_rules.sort(key=lambda r: r.priority)
        
        if self.debug:
            self.logger.debug(f"添加优化规则: {rule.name}")
    
    def shutdown(self):
        """关闭优化器"""
        
        self._monitoring_active = False
        
        if self._monitoring_thread and self._monitoring_thread.is_alive():
            self._monitoring_thread.join(timeout=3)
        
        self.logger.info("性能优化器已关闭")


if __name__ == "__main__":
    # 测试代码
    print("=== 性能优化器测试 ===")
    
    optimizer = PerformanceOptimizer(
        optimization_level=OptimizationLevel.BALANCED,
        enable_learning=True,
        debug=True
    )
    
    # 测试处理器选择优化
    print("🔧 测试处理器选择优化...")
    
    test_cases = [
        ('image_compress', 2000000),  # 大图像压缩
        ('ai_inference', 50000),      # AI推理
        ('batch_process', 1000000),   # 批量处理
        ('image_analyze', 100000),    # 图像分析
    ]
    
    for operation, size in test_cases:
        result = optimizer.optimize_processor_selection(operation, size)
        print(f"  {operation}({size} bytes): {result['recommended_processor']} (置信度: {result['confidence']:.2f})")
        print(f"    原因: {result['reason']}")
        
        # 模拟记录性能
        simulated_time = np.random.uniform(1, 50)
        optimizer.record_performance(
            result['recommended_processor'],
            operation,
            size,
            simulated_time,
            memory_usage_mb=np.random.uniform(5, 50)
        )
    
    # 再次测试，应该有学习效果
    print("\\n🧠 学习后再次测试...")
    for operation, size in test_cases[:2]:
        result = optimizer.optimize_processor_selection(operation, size)
        print(f"  {operation}({size} bytes): {result['recommended_processor']} (置信度: {result['confidence']:.2f})")
    
    # 优化统计
    stats = optimizer.get_optimization_stats()
    print(f"\\n📊 优化统计:")
    print(f"  总优化次数: {stats['optimization_stats']['total_optimizations']}")
    print(f"  成功率: {stats['optimization_stats']['optimization_success_rate']:.1f}%")
    print(f"  预测置信度: {stats['predictor_confidence']:.2f}")
    print(f"  优化规则数: {stats['total_rules']}")
    
    optimizer.shutdown()
    print("🎯 性能优化器测试完成！")
