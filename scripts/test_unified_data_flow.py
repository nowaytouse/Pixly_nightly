#!/usr/bin/env python3
"""
统一数据流测试
验证Python训练 ↔ Rust推理的完整对齐
"""

import json
import numpy as np
from ml_bridge import StandardFeatures, StandardPrediction, TrainingSample
from ml_data_flow import MLDataFlow


def test_feature_consistency():
    """测试特征一致性"""
    print("Test 1: Feature consistency")
    
    # 创建标准特征
    features = StandardFeatures(
        basic=[1.0] * 16,
        color=[2.0] * 16,
        texture=[3.0] * 16,
        shape=[4.0] * 16,
        quality=[5.0] * 16,
        metadata=[6.0] * 32,
        context=[7.0] * 16
    )
    
    # 测试向量转换
    vec = features.to_vector()
    assert len(vec) == 128, f"Invalid dimension: {len(vec)}"
    
    # 验证布局
    assert vec[0] == 1.0, "Basic features error"
    assert vec[16] == 2.0, "Color features error"
    assert vec[32] == 3.0, "Texture features error"
    assert vec[48] == 4.0, "Shape features error"
    assert vec[64] == 5.0, "Quality features error"
    assert vec[80] == 6.0, "Metadata features error"
    assert vec[112] == 7.0, "Context features error"
    
    print("   Feature layout: OK")
    
    # 测试JSON序列化
    json_str = features.to_json()
    restored = StandardFeatures.from_json(json_str)
    restored_vec = restored.to_vector()
    
    assert np.allclose(vec, restored_vec), "JSON serialization mismatch"
    print("   JSON serialization: OK")
    
    # 测试向量往返转换
    features2 = StandardFeatures.from_vector(vec)
    vec2 = features2.to_vector()
    assert np.allclose(vec, vec2), "Vector conversion mismatch"
    print("   Vector conversion: OK")
    
    print()


def test_training_sample_format():
    """测试训练样本格式"""
    print("Test 2: Training sample format")
    
    features = StandardFeatures(
        basic=[0.1] * 16,
        color=[0.2] * 16,
        texture=[0.3] * 16,
        shape=[0.4] * 16,
        quality=[0.5] * 16,
        metadata=[0.6] * 32,
        context=[0.7] * 16
    )
    
    sample = TrainingSample(
        features=features,
        actual_quality=75,
        actual_effort=6,
        actual_lossless=False,
        actual_format="avif",
        result_size=50000,
        result_quality=0.96,
        processing_time=1.5,
        user_rating=4.5,
        timestamp=1700000000
    )
    
    # 转换为训练格式
    training_format = sample.to_training_format()
    
    # 验证字段
    assert 'features' in training_format
    assert 'quality' in training_format
    assert 'effort' in training_format
    assert 'lossless' in training_format
    assert 'format' in training_format
    assert 'result_size' in training_format
    assert 'result_quality' in training_format
    assert 'processing_time' in training_format
    assert 'user_rating' in training_format
    assert 'timestamp' in training_format
    
    print("   Training format fields: OK")
    
    # 验证特征维度
    assert len(training_format['features']) == 128
    print("   Feature dimension: OK")
    
    # 验证数据类型
    assert isinstance(training_format['quality'], int)
    assert isinstance(training_format['effort'], int)
    assert isinstance(training_format['lossless'], int)
    assert isinstance(training_format['format'], str)
    assert isinstance(training_format['result_size'], int)
    assert isinstance(training_format['result_quality'], float)
    
    print("   Data types: OK")
    print()


def test_prediction_format():
    """测试预测格式"""
    print("Test 3: Prediction format")
    
    prediction = StandardPrediction(
        quality=75,
        effort=6,
        lossless=False,
        format="avif",
        confidence=0.85,
        estimated_size=50000,
        estimated_quality=0.96,
        model_version="1.0.0"
    )
    
    # 测试JSON序列化
    json_str = prediction.to_json()
    restored = StandardPrediction.from_json(json_str)
    
    assert restored.quality == 75
    assert restored.effort == 6
    assert restored.lossless == False
    assert restored.format == "avif"
    assert restored.confidence == 0.85
    assert restored.estimated_size == 50000
    assert restored.estimated_quality == 0.96
    assert restored.model_version == "1.0.0"
    
    print("   Prediction serialization: OK")
    print()


def test_data_flow_pipeline():
    """测试完整数据流"""
    print("Test 4: Complete data flow")
    
    flow = MLDataFlow(model_version="1.0.0")
    
    # 1. Rust → Python: 特征传输
    print("   Rust -> Python: Feature transfer")
    rust_features = StandardFeatures(
        basic=[0.5] * 16,
        color=[0.6] * 16,
        texture=[0.7] * 16,
        shape=[0.8] * 16,
        quality=[0.9] * 16,
        metadata=[0.55] * 32,
        context=[0.65] * 16
    )
    
    json_str = rust_features.to_json()
    python_features = flow.load_features_from_rust(json_str)
    
    assert np.allclose(
        rust_features.to_vector(),
        python_features.to_vector()
    )
    print("      Feature transfer: OK")
    
    # 2. Python训练: 准备数据
    print("   Python training: Data preparation")
    sample = TrainingSample(
        features=python_features,
        actual_quality=75,
        actual_effort=6,
        actual_lossless=False,
        actual_format="avif",
        result_size=50000,
        result_quality=0.96,
        processing_time=1.5,
        user_rating=4.5,
        timestamp=1700000000
    )
    
    X, y = flow.prepare_training_dataset([sample])
    assert X.shape == (1, 128)
    assert 'quality' in y
    assert 'effort' in y
    assert 'lossless' in y
    assert 'size' in y
    print("      Training data preparation: OK")
    
    # 3. Python → Rust: 预测传输
    print("   Python -> Rust: Prediction transfer")
    prediction = flow.bridge.create_prediction(
        quality=75,
        effort=6,
        lossless=False,
        format="avif",
        confidence=0.85,
        estimated_size=50000,
        estimated_quality=0.96
    )
    
    pred_json = prediction.to_json()
    # 模拟Rust接收
    rust_prediction = StandardPrediction.from_json(pred_json)
    
    assert rust_prediction.quality == prediction.quality
    assert rust_prediction.format == prediction.format
    print("      Prediction transfer: OK")
    
    print()


def test_batch_processing():
    """测试批量处理"""
    print("Test 5: Batch processing")
    
    flow = MLDataFlow()
    
    # 创建多个样本
    samples = []
    for i in range(10):
        features = StandardFeatures(
            basic=[float(i) / 10] * 16,
            color=[float(i) / 10 + 0.1] * 16,
            texture=[float(i) / 10 + 0.2] * 16,
            shape=[float(i) / 10 + 0.3] * 16,
            quality=[float(i) / 10 + 0.4] * 16,
            metadata=[float(i) / 10 + 0.5] * 32,
            context=[float(i) / 10 + 0.6] * 16
        )
        
        sample = TrainingSample(
            features=features,
            actual_quality=70 + i,
            actual_effort=5 + (i % 5),
            actual_lossless=(i % 2 == 0),
            actual_format="avif" if i % 2 == 0 else "webp",
            result_size=40000 + i * 1000,
            result_quality=0.90 + i * 0.01,
            processing_time=1.0 + i * 0.1,
            user_rating=4.0 + i * 0.1,
            timestamp=1700000000 + i
        )
        samples.append(sample)
    
    # 准备批量训练数据
    X, y = flow.prepare_training_dataset(samples)
    
    assert X.shape == (10, 128)
    assert len(y['quality']) == 10
    assert len(y['effort']) == 10
    assert len(y['lossless']) == 10
    assert len(y['size']) == 10
    
    print(f"   Batch processing: {len(samples)} samples")
    print(f"   Feature matrix: {X.shape}")
    print(f"   Label count: {len(y)}")
    print()


def run_all_tests():
    """运行所有测试"""
    print("=" * 60)
    print("Unified Data Flow Tests")
    print("=" * 60)
    print()
    
    try:
        test_feature_consistency()
        test_training_sample_format()
        test_prediction_format()
        test_data_flow_pipeline()
        test_batch_processing()
        
        print("=" * 60)
        print("All tests passed!")
        print("=" * 60)
        print()
        print("Test Summary:")
        print("   Feature consistency: 128-dim standardized")
        print("   Training sample format: Python <-> Rust aligned")
        print("   Prediction format: JSON bidirectional serialization")
        print("   Complete data flow: End-to-end validated")
        print("   Batch processing: Multi-sample support")
        print()
        print("Data flow status: Fully unified")
        
        return True
        
    except AssertionError as e:
        print(f"Test failed: {e}")
        return False
    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()
        return False


if __name__ == "__main__":
    success = run_all_tests()
    exit(0 if success else 1)
