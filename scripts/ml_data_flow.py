#!/usr/bin/env python3
"""
ML数据流管理器 - Python端
统一Python训练 ↔ Rust推理的完整数据流
"""

import json
import numpy as np
from pathlib import Path
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass
from ml_bridge import StandardFeatures, StandardPrediction, TrainingSample, MLBridge


class MLDataFlow:
    """
    ML数据流管理器
    与Rust端完全对齐
    """
    
    def __init__(self, model_version: str = "1.0.0"):
        self.bridge = MLBridge(model_version)
        self.training_buffer: List[TrainingSample] = []
        self.max_buffer_size = 1000
        self.auto_save_path: Optional[Path] = None
    
    def with_auto_save(self, path: str) -> 'MLDataFlow':
        """设置自动保存路径"""
        self.auto_save_path = Path(path)
        return self
    
    def with_buffer_size(self, size: int) -> 'MLDataFlow':
        """设置缓冲区大小"""
        self.max_buffer_size = size
        return self
    
    def load_features_from_rust(self, json_str: str) -> StandardFeatures:
        """
        从Rust加载特征 (Rust → Python训练)
        """
        return StandardFeatures.from_json(json_str)
    
    def load_features_from_file(self, path: Path) -> StandardFeatures:
        """从文件加载特征"""
        with open(path, 'r') as f:
            return StandardFeatures.from_json(f.read())
    
    def load_training_samples(self, path: Path) -> List[TrainingSample]:
        """
        加载训练样本 (Rust → Python训练)
        """
        with open(path, 'r') as f:
            data = json.load(f)
        
        samples = []
        for item in data:
            features = StandardFeatures.from_vector(np.array(item['features']))
            sample = TrainingSample(
                features=features,
                actual_quality=item['quality'],
                actual_effort=item['effort'],
                actual_lossless=bool(item['lossless']),
                actual_format=item['format'],
                result_size=item['result_size'],
                result_quality=item['result_quality'],
                processing_time=item['processing_time'],
                user_rating=item.get('user_rating'),
                timestamp=item['timestamp']
            )
            samples.append(sample)
        
        return samples
    
    def export_prediction_to_rust(self, prediction: StandardPrediction, path: Path):
        """
        导出预测结果给Rust (Python → Rust推理)
        """
        with open(path, 'w') as f:
            f.write(prediction.to_json())
    
    def prepare_training_dataset(
        self, 
        samples: List[TrainingSample]
    ) -> Tuple[np.ndarray, Dict[str, np.ndarray]]:
        """
        准备训练数据集
        
        返回:
            X: 特征矩阵 (n_samples, 128)
            y: 标签字典
        """
        return self.bridge.prepare_training_data(samples)
    
    def validate_data_flow(self) -> bool:
        """验证数据流一致性"""
        # 创建测试特征
        test_features = StandardFeatures(
            basic=[1.0] * 16,
            color=[2.0] * 16,
            texture=[3.0] * 16,
            shape=[4.0] * 16,
            quality=[5.0] * 16,
            metadata=[6.0] * 32,
            context=[7.0] * 16
        )
        
        # 测试向量转换
        vec = test_features.to_vector()
        assert len(vec) == 128, f"Invalid feature dimension: {len(vec)}"
        
        # 测试JSON序列化
        json_str = test_features.to_json()
        restored = StandardFeatures.from_json(json_str)
        
        # 验证一致性
        restored_vec = restored.to_vector()
        assert np.allclose(vec, restored_vec, atol=1e-10), "Feature mismatch"
        
        return True
    
    def create_training_pipeline(
        self,
        input_dir: Path,
        output_model_path: Path
    ) -> Dict[str, any]:
        """
        创建完整的训练流水线
        
        1. 加载Rust导出的训练样本
        2. 准备训练数据
        3. 训练模型
        4. 导出模型
        
        返回训练统计信息
        """
        stats = {
            'samples_loaded': 0,
            'samples_valid': 0,
            'training_time': 0.0,
            'model_path': str(output_model_path)
        }
        
        # 加载所有训练样本
        all_samples = []
        for json_file in input_dir.glob('*.json'):
            try:
                samples = self.load_training_samples(json_file)
                all_samples.extend(samples)
                stats['samples_loaded'] += len(samples)
            except Exception as e:
                print(f"Warning: Failed to load {json_file}: {e}")
        
        if not all_samples:
            raise ValueError("没有找到训练样本")
        
        # 验证样本
        valid_samples = []
        for sample in all_samples:
            try:
                self.bridge.validate_features(sample.features)
                valid_samples.append(sample)
                stats['samples_valid'] += 1
            except Exception as e:
                print(f"Warning: Sample validation failed: {e}")
        
        # 准备训练数据
        X, y = self.prepare_training_dataset(valid_samples)
        
        print(f"Training data prepared:")
        print(f"   - Samples: {len(valid_samples)}")
        print(f"   - Feature shape: {X.shape}")
        print(f"   - Labels: {list(y.keys())}")
        
        # TODO: 实际训练模型
        # model = train_model(X, y)
        # save_model(model, output_model_path)
        
        return stats
    
    def create_inference_pipeline(
        self,
        model_path: Path,
        features: StandardFeatures
    ) -> StandardPrediction:
        """
        创建推理流水线
        
        1. 加载训练好的模型
        2. 提取特征向量
        3. 模型预测
        4. 返回标准化预测结果
        """
        # 验证特征
        self.bridge.validate_features(features)
        
        # 提取特征向量
        X = features.to_vector().reshape(1, -1)
        
        # TODO: 实际模型推理
        # predictions = model.predict(X)
        
        # 模拟预测结果
        prediction = self.bridge.create_prediction(
            quality=75,
            effort=6,
            lossless=False,
            format="avif",
            confidence=0.85,
            estimated_size=50000,
            estimated_quality=0.96
        )
        
        return prediction


def demonstrate_data_flow():
    """演示完整的数据流"""
    print("Demonstrating ML data flow...")
    print()
    
    # 创建数据流管理器
    flow = MLDataFlow(model_version="1.0.0")
    
    # 1. 验证数据流一致性
    print("Step 1: Validating data flow consistency...")
    assert flow.validate_data_flow()
    print("   Data flow validation: OK")
    print()
    
    # 2. 模拟Rust导出的特征
    print("Step 2: Rust -> Python feature transfer...")
    rust_features = StandardFeatures(
        basic=[0.5, 0.6, 0.7, 0.8] + [0.0] * 12,
        color=[0.1, 0.2, 0.3, 0.4] + [0.0] * 12,
        texture=[0.2, 0.3, 0.4, 0.5] + [0.0] * 12,
        shape=[0.3, 0.4, 0.5, 0.6] + [0.0] * 12,
        quality=[0.4, 0.5, 0.6, 0.7] + [0.0] * 12,
        metadata=[0.5] * 32,
        context=[0.6] * 16
    )
    
    # 转换为JSON (模拟Rust序列化)
    json_str = rust_features.to_json()
    print(f"   JSON size: {len(json_str)} bytes")
    
    # Python接收 (模拟Python反序列化)
    python_features = flow.load_features_from_rust(json_str)
    print(f"   Features received: {len(python_features.to_vector())} dimensions")
    print()
    
    # 3. 模拟训练样本
    print("Step 3: Collecting training samples...")
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
    print(f"   Training data: X={X.shape}, y={len(y)} labels")
    print()
    
    # 4. 模拟预测结果
    print("Step 4: Python -> Rust prediction transfer...")
    prediction = flow.bridge.create_prediction(
        quality=75,
        effort=6,
        lossless=False,
        format="avif",
        confidence=0.85,
        estimated_size=50000,
        estimated_quality=0.96
    )
    
    # 转换为JSON (模拟Python序列化)
    pred_json = prediction.to_json()
    print(f"   Prediction JSON size: {len(pred_json)} bytes")
    
    # Rust接收 (模拟Rust反序列化)
    print(f"   Prediction result: quality={prediction.quality}, format={prediction.format}")
    print()
    
    print("Data flow demonstration complete!")
    print()
    print("Summary:")
    print("   Rust -> Python: Feature extraction -> JSON -> Training")
    print("   Python -> Rust: Model prediction -> JSON -> Inference")
    print("   Feature dimension: 128 (standardized)")
    print("   JSON communication: Bidirectional serialization")


if __name__ == "__main__":
    demonstrate_data_flow()
