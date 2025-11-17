#!/usr/bin/env python3
"""
ML桥接层 - Python训练 ↔ Rust推理
统一特征定义和数据流
"""

import json
import numpy as np
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass, asdict

@dataclass
class StandardFeatures:
    """
    标准化特征向量 (128维)
    与Rust推理保持完全一致
    """
    basic: List[float]      # 16维 - 图像基本属性
    color: List[float]      # 16维 - 颜色分布和复杂度
    texture: List[float]    # 16维 - 边缘和纹理信息
    shape: List[float]      # 16维 - 几何和结构
    quality: List[float]    # 16维 - 噪声和清晰度
    metadata: List[float]   # 32维 - EXIF和文件属性
    context: List[float]    # 16维 - 处理历史和环境
    
    def to_vector(self) -> np.ndarray:
        """转换为128维向量"""
        return np.concatenate([
            self.basic,
            self.color,
            self.texture,
            self.shape,
            self.quality,
            self.metadata,
            self.context
        ])
    
    @classmethod
    def from_vector(cls, vec: np.ndarray) -> 'StandardFeatures':
        """从128维向量创建"""
        assert len(vec) == 128, f"Invalid feature vector length: {len(vec)} (expected 128)"
        
        return cls(
            basic=vec[0:16].tolist(),
            color=vec[16:32].tolist(),
            texture=vec[32:48].tolist(),
            shape=vec[48:64].tolist(),
            quality=vec[64:80].tolist(),
            metadata=vec[80:112].tolist(),
            context=vec[112:128].tolist()
        )
    
    def to_json(self) -> str:
        """转换为JSON (Rust通信)"""
        return json.dumps(asdict(self))
    
    @classmethod
    def from_json(cls, json_str: str) -> 'StandardFeatures':
        """从JSON创建 (Rust通信)"""
        data = json.loads(json_str)
        return cls(**data)


@dataclass
class StandardPrediction:
    """
    标准化预测结果
    与Rust推理输出保持一致
    """
    quality: int            # 推荐质量 (0-100)
    effort: int             # 推荐速度/effort (0-9)
    lossless: bool          # 是否推荐无损
    format: str             # 推荐格式
    confidence: float       # 预测置信度 (0-1)
    estimated_size: int     # 预估文件大小 (bytes)
    estimated_quality: float  # 预估质量 (SSIM)
    model_version: str      # 模型版本
    
    def to_json(self) -> str:
        """转换为JSON (Rust通信)"""
        return json.dumps(asdict(self))
    
    @classmethod
    def from_json(cls, json_str: str) -> 'StandardPrediction':
        """从JSON创建 (Rust通信)"""
        data = json.loads(json_str)
        return cls(**data)


@dataclass
class TrainingSample:
    """
    训练样本 (用于反馈)
    与Rust收集的数据保持一致
    """
    features: StandardFeatures
    
    # 实际使用的参数
    actual_quality: int
    actual_effort: int
    actual_lossless: bool
    actual_format: str
    
    # 实际结果
    result_size: int
    result_quality: float  # SSIM
    processing_time: float
    
    # 用户反馈 (可选)
    user_rating: Optional[float] = None
    
    # 时间戳
    timestamp: int = 0
    
    def to_training_format(self) -> Dict:
        """转换为训练格式"""
        return {
            'features': self.features.to_vector().tolist(),
            'quality': self.actual_quality,
            'effort': self.actual_effort,
            'lossless': int(self.actual_lossless),
            'format': self.actual_format,
            'result_size': self.result_size,
            'result_quality': self.result_quality,
            'processing_time': self.processing_time,
            'user_rating': self.user_rating,
            'timestamp': self.timestamp
        }
    
    @classmethod
    def from_rust_json(cls, json_str: str) -> 'TrainingSample':
        """从Rust JSON创建"""
        data = json.loads(json_str)
        features = StandardFeatures.from_vector(np.array(data['features']))
        
        return cls(
            features=features,
            actual_quality=data['quality'],
            actual_effort=data['effort'],
            actual_lossless=bool(data['lossless']),
            actual_format=data['format'],
            result_size=data['result_size'],
            result_quality=data['result_quality'],
            processing_time=data['processing_time'],
            user_rating=data.get('user_rating'),
            timestamp=data['timestamp']
        )


class MLBridge:
    """
    ML桥接器 - 统一Python训练和Rust推理
    """
    
    def __init__(self, model_version: str = "1.0.0"):
        self.model_version = model_version
    
    def validate_features(self, features: StandardFeatures) -> bool:
        """验证特征一致性"""
        vec = features.to_vector()
        
        # 检查维度
        if len(vec) != 128:
            raise ValueError(f"特征维度错误: {len(vec)}")
        
        # 检查NaN
        if np.isnan(vec).any():
            raise ValueError("特征包含NaN")
        
        # 检查Inf
        if np.isinf(vec).any():
            raise ValueError("特征包含Inf")
        
        return True
    
    def prepare_training_data(self, samples: List[TrainingSample]) -> Tuple[np.ndarray, Dict[str, np.ndarray]]:
        """
        准备训练数据
        
        返回:
            X: 特征矩阵 (n_samples, 128)
            y: 标签字典 {
                'quality': (n_samples,),
                'effort': (n_samples,),
                'lossless': (n_samples,),
                'size': (n_samples,)
            }
        """
        X = np.array([s.features.to_vector() for s in samples])
        
        y = {
            'quality': np.array([s.actual_quality for s in samples]),
            'effort': np.array([s.actual_effort for s in samples]),
            'lossless': np.array([int(s.actual_lossless) for s in samples]),
            'size': np.array([s.result_size for s in samples])
        }
        
        return X, y
    
    def create_prediction(self, quality: int, effort: int, lossless: bool, 
                         format: str, confidence: float, 
                         estimated_size: int, estimated_quality: float) -> StandardPrediction:
        """创建标准化预测结果"""
        return StandardPrediction(
            quality=quality,
            effort=effort,
            lossless=lossless,
            format=format,
            confidence=confidence,
            estimated_size=estimated_size,
            estimated_quality=estimated_quality,
            model_version=self.model_version
        )


if __name__ == "__main__":
    # 测试
    print("Testing ML bridge...")
    
    # 创建测试特征
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
    print(f"Feature vector: {len(vec)} dimensions")
    
    # 测试JSON序列化
    json_str = features.to_json()
    restored = StandardFeatures.from_json(json_str)
    assert restored.basic[0] == 1.0
    print("JSON serialization: OK")
    
    # 测试训练样本
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
    
    training_format = sample.to_training_format()
    assert 'features' in training_format
    assert len(training_format['features']) == 128
    print("Training sample format: OK")
    
    # 测试桥接器
    bridge = MLBridge()
    assert bridge.validate_features(features)
    print("Feature validation: OK")
    
    print("\nAll tests passed!")
    print(f"Feature dimension: 128")
    print(f"Python <-> Rust bridge ready")
