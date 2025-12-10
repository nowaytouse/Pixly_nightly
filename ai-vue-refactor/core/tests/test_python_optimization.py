#!/usr/bin/env python3
"""
Python优化效果测试
验证向量化和预分配优化的性能提升
"""

import sys
import time
import numpy as np
from pathlib import Path

# 添加scripts到路径
sys.path.insert(0, str(Path(__file__).parent.parent / "scripts"))

from ml_bridge import StandardFeatures

def test_to_vector_performance():
    """测试to_vector性能优化"""
    print("🧪 测试 StandardFeatures.to_vector() 性能...")
    
    # 创建测试特征
    features = StandardFeatures(
        basic=[1.0] * 16,
        color=[0.5] * 16,
        texture=[0.3] * 16,
        shape=[0.7] * 16,
        quality=[0.9] * 16,
        metadata=[0.2] * 32,
        context=[0.4] * 16
    )
    
    # 性能测试
    iterations = 10000
    start = time.perf_counter()
    for _ in range(iterations):
        vec = features.to_vector()
    elapsed = time.perf_counter() - start
    
    # 验证结果
    vec = features.to_vector()
    assert len(vec) == 128, f"Expected 128 dims, got {len(vec)}"
    assert vec.dtype == np.float64, f"Expected float64, got {vec.dtype}"
    
    print(f"✅ {iterations} iterations: {elapsed:.4f}s")
    print(f"   Average: {elapsed/iterations*1000:.4f}ms per call")
    print(f"   Vector shape: {vec.shape}, dtype: {vec.dtype}")
    
    return elapsed

def test_ensemble_prediction_performance():
    """测试集成预测向量化优化"""
    print("\n🧪 测试集成预测向量化...")
    
    # 模拟预测结果
    class MockPrediction:
        def __init__(self, quality, effort, confidence):
            self.quality = quality
            self.effort = effort
            self.confidence = confidence
            self.lossless = False
            self.estimated_size = 1000000
            self.estimated_quality = 0.9
    
    predictions = [
        MockPrediction(75, 6, 0.85),
        MockPrediction(80, 7, 0.90),
        MockPrediction(78, 6, 0.88),
    ]
    
    # 测试向量化计算
    start = time.perf_counter()
    iterations = 10000
    for _ in range(iterations):
        qualities = np.array([p.quality for p in predictions], dtype=np.int32)
        efforts = np.array([p.effort for p in predictions], dtype=np.int32)
        confidences = np.array([p.confidence for p in predictions], dtype=np.float64)
        
        avg_quality = int(qualities.mean())
        avg_effort = int(efforts.mean())
        avg_confidence = float(confidences.mean())
    
    elapsed = time.perf_counter() - start
    
    print(f"✅ {iterations} iterations: {elapsed:.4f}s")
    print(f"   Average: {elapsed/iterations*1000:.4f}ms per call")
    print(f"   Results: quality={avg_quality}, effort={avg_effort}, confidence={avg_confidence:.2f}")
    
    return elapsed

def test_video_feature_extraction():
    """测试视频特征提取优化"""
    print("\n🧪 测试视频特征提取...")
    
    # 模拟特征向量
    features_vec = [
        1920.0, 1080.0, 2073600.0, 5.5,  # width, height, pixels, size_mb
        0.0,  # padding
        300.0, 30.0, 10.0,  # frame_count, fps, duration
        1.0, 0.75, 1.0, 1.0  # has_audio, complexity, is_high_res, is_long
    ]
    
    # 测试向量化提取
    start = time.perf_counter()
    iterations = 10000
    for _ in range(iterations):
        features_arr = np.array(features_vec, dtype=np.float64)
        width, height, pixels, size_mb = features_arr[0:4]
        frame_count, fps, duration = features_arr[5:8]
        has_audio = features_arr[8] > 0.5
        scene_complexity = features_arr[9]
        is_high_res = features_arr[10] > 0.5
        is_long = features_arr[11] > 0.5
    
    elapsed = time.perf_counter() - start
    
    print(f"✅ {iterations} iterations: {elapsed:.4f}s")
    print(f"   Average: {elapsed/iterations*1000:.4f}ms per call")
    print(f"   Extracted: {width}x{height}, {duration}s, complexity={scene_complexity}")
    
    return elapsed

def main():
    print("=" * 60)
    print("🚀 Python优化性能测试")
    print("=" * 60)
    
    try:
        t1 = test_to_vector_performance()
        t2 = test_ensemble_prediction_performance()
        t3 = test_video_feature_extraction()
        
        print("\n" + "=" * 60)
        print("📊 测试总结")
        print("=" * 60)
        print(f"✅ 所有测试通过")
        print(f"   to_vector: {t1:.4f}s")
        print(f"   ensemble: {t2:.4f}s")
        print(f"   video_extract: {t3:.4f}s")
        print(f"   总耗时: {t1+t2+t3:.4f}s")
        
        return 0
    except Exception as e:
        print(f"\n❌ 测试失败: {e}")
        import traceback
        traceback.print_exc()
        return 1

if __name__ == "__main__":
    sys.exit(main())
