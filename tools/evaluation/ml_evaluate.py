#!/usr/bin/env python3
"""
📊 ML模型评估脚本
评估所有模型在测试集上的性能
"""

import sys
import json
import numpy as np
from pathlib import Path
from typing import Dict, List, Tuple
from dataclasses import dataclass
import time

# 添加scripts目录到路径
sys.path.insert(0, str(Path(__file__).parent))

from ml_bridge import ModelRouter, StandardFeatures, ModelType, StandardPrediction

@dataclass
class EvaluationMetrics:
    """评估指标"""
    mae_quality: float  # 质量预测的平均绝对误差
    mae_effort: float   # effort预测的平均绝对误差
    accuracy_quality: float  # 质量预测准确率 (±5以内)
    accuracy_effort: float   # effort预测准确率 (±1以内)
    avg_confidence: float    # 平均置信度
    inference_time_ms: float # 平均推理时间(毫秒)
    total_samples: int       # 样本数量

def load_test_data(data_path: str = "models/training_data_final.json") -> List[Dict]:
    """加载测试数据"""
    print(f"📂 加载测试数据: {data_path}")
    
    data_file = Path(data_path)
    if not data_file.exists():
        print(f"  ❌ 文件不存在: {data_path}")
        return []
    
    with open(data_file, 'r') as f:
        data = json.load(f)
    
    # 使用最后20%作为测试集
    test_size = len(data) // 5
    test_data = data[-test_size:]
    
    print(f"  ✅ 加载 {len(test_data)} 个测试样本")
    return test_data

def evaluate_model(
    router: ModelRouter,
    model_type: ModelType,
    test_data: List[Dict],
    max_samples: int = 100
) -> EvaluationMetrics:
    """评估单个模型"""
    
    print(f"\n🧪 评估 {model_type.value.upper()} 模型...")
    
    errors_quality = []
    errors_effort = []
    correct_quality = 0
    correct_effort = 0
    confidences = []
    inference_times = []
    
    # 限制样本数量以加快评估
    samples = test_data[:max_samples]
    
    for i, sample in enumerate(samples):
        if (i + 1) % 20 == 0:
            print(f"  进度: {i+1}/{len(samples)}")
        
        try:
            # 提取特征
            features = StandardFeatures.from_vector(np.array(sample['features']))
            
            # 真实标签
            true_quality = sample['quality']
            true_effort = sample['effort']
            target_format = sample.get('format', 'webp')
            
            # 预测
            start_time = time.time()
            prediction = router.predict(
                model_type=model_type,
                features=features,
                target_format=target_format,
                quality_mode="balanced"
            )
            inference_time = (time.time() - start_time) * 1000  # 转换为毫秒
            
            # 计算误差
            error_quality = abs(prediction.quality - true_quality)
            error_effort = abs(prediction.effort - true_effort)
            
            errors_quality.append(error_quality)
            errors_effort.append(error_effort)
            
            # 计算准确率 (质量±5, effort±1)
            if error_quality <= 5:
                correct_quality += 1
            if error_effort <= 1:
                correct_effort += 1
            
            confidences.append(prediction.confidence)
            inference_times.append(inference_time)
            
        except Exception as e:
            print(f"  ⚠️  样本 {i} 预测失败: {e}")
            continue
    
    # 计算指标
    if not errors_quality:
        print(f"  ❌ 没有成功的预测")
        return None
    
    metrics = EvaluationMetrics(
        mae_quality=np.mean(errors_quality),
        mae_effort=np.mean(errors_effort),
        accuracy_quality=correct_quality / len(errors_quality),
        accuracy_effort=correct_effort / len(errors_effort),
        avg_confidence=np.mean(confidences),
        inference_time_ms=np.mean(inference_times),
        total_samples=len(errors_quality)
    )
    
    # 打印结果
    print(f"\n  📊 评估结果:")
    print(f"     样本数: {metrics.total_samples}")
    print(f"     Quality MAE: {metrics.mae_quality:.2f}")
    print(f"     Quality准确率: {metrics.accuracy_quality:.1%} (±5)")
    print(f"     Effort MAE: {metrics.mae_effort:.2f}")
    print(f"     Effort准确率: {metrics.accuracy_effort:.1%} (±1)")
    print(f"     平均置信度: {metrics.avg_confidence:.1%}")
    print(f"     推理时间: {metrics.inference_time_ms:.2f}ms")
    
    return metrics

def compare_models(results: Dict[ModelType, EvaluationMetrics]):
    """对比模型性能"""
    print("\n" + "=" * 70)
    print("📊 模型性能对比")
    print("=" * 70)
    
    # 表头
    print(f"\n{'模型':<15} {'Quality MAE':<15} {'Effort MAE':<15} {'推理时间':<15}")
    print("-" * 70)
    
    # 数据行
    for model_type, metrics in results.items():
        if metrics is None:
            continue
        print(f"{model_type.value.upper():<15} "
              f"{metrics.mae_quality:<15.2f} "
              f"{metrics.mae_effort:<15.2f} "
              f"{metrics.inference_time_ms:<15.2f}ms")
    
    # 找出最佳模型
    print("\n" + "=" * 70)
    print("🏆 最佳模型:")
    print("=" * 70)
    
    # 最低Quality MAE
    best_quality = min(results.items(), 
                      key=lambda x: x[1].mae_quality if x[1] else float('inf'))
    if best_quality[1]:
        print(f"  Quality预测: {best_quality[0].value.upper()} "
              f"(MAE={best_quality[1].mae_quality:.2f})")
    
    # 最低Effort MAE
    best_effort = min(results.items(),
                     key=lambda x: x[1].mae_effort if x[1] else float('inf'))
    if best_effort[1]:
        print(f"  Effort预测: {best_effort[0].value.upper()} "
              f"(MAE={best_effort[1].mae_effort:.2f})")
    
    # 最快推理
    best_speed = min(results.items(),
                    key=lambda x: x[1].inference_time_ms if x[1] else float('inf'))
    if best_speed[1]:
        print(f"  推理速度: {best_speed[0].value.upper()} "
              f"({best_speed[1].inference_time_ms:.2f}ms)")
    
    # 最高置信度
    best_confidence = max(results.items(),
                         key=lambda x: x[1].avg_confidence if x[1] else 0)
    if best_confidence[1]:
        print(f"  置信度: {best_confidence[0].value.upper()} "
              f"({best_confidence[1].avg_confidence:.1%})")

def save_results(results: Dict[ModelType, EvaluationMetrics], output_path: str):
    """保存评估结果"""
    print(f"\n💾 保存结果到: {output_path}")
    
    output_data = {
        'timestamp': time.strftime('%Y-%m-%d %H:%M:%S'),
        'models': {}
    }
    
    for model_type, metrics in results.items():
        if metrics is None:
            continue
        
        output_data['models'][model_type.value] = {
            'mae_quality': float(metrics.mae_quality),
            'mae_effort': float(metrics.mae_effort),
            'accuracy_quality': float(metrics.accuracy_quality),
            'accuracy_effort': float(metrics.accuracy_effort),
            'avg_confidence': float(metrics.avg_confidence),
            'inference_time_ms': float(metrics.inference_time_ms),
            'total_samples': metrics.total_samples
        }
    
    with open(output_path, 'w') as f:
        json.dump(output_data, f, indent=2)
    
    print(f"  ✅ 结果已保存")

def main():
    """主函数"""
    print("=" * 70)
    print("📊 Pixly ML模型评估")
    print("=" * 70)
    
    # 加载测试数据
    test_data = load_test_data()
    if not test_data:
        print("❌ 无法加载测试数据")
        return 1
    
    # 初始化路由器
    router = ModelRouter()
    
    # 评估所有可用模型
    results = {}
    for model_type, is_available in router.available_models.items():
        if not is_available:
            print(f"\n⏭️  跳过 {model_type.value.upper()} (不可用)")
            continue
        
        try:
            metrics = evaluate_model(router, model_type, test_data, max_samples=100)
            results[model_type] = metrics
        except Exception as e:
            print(f"\n❌ {model_type.value.upper()} 评估失败: {e}")
            import traceback
            traceback.print_exc()
            results[model_type] = None
    
    # 对比模型
    compare_models(results)
    
    # 保存结果
    timestamp = time.strftime('%Y%m%d_%H%M%S')
    output_path = f"models/evaluation_results_{timestamp}.json"
    save_results(results, output_path)
    
    print("\n" + "=" * 70)
    print("✅ 评估完成")
    print("=" * 70)
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
