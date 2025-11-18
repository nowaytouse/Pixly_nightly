#!/usr/bin/env python3
"""
ML桥接层 - Python训练 ↔ Rust推理
统一特征定义和数据流

🤖 多模型路由系统：
- LightGBM: 传统梯度提升模型（快速、准确）
- PPO: 强化学习模型（在线学习、自适应）
- Bayesian: 贝叶斯优化器（不确定性量化）
- Ensemble: 集成模型（多模型投票）
"""

import json
import numpy as np
import argparse
import sys
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass, asdict
from enum import Enum

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


class ModelType(Enum):
    """模型类型枚举"""
    LIGHTGBM = "lightgbm"
    PPO = "ppo"
    BAYESIAN = "bayesian"
    ENSEMBLE = "ensemble"


class ModelRouter:
    """
    🤖 多模型路由器
    
    根据场景和性能自动选择最佳模型：
    - LightGBM: 默认模型，快速准确
    - PPO: 在线学习，自适应优化
    - Bayesian: 不确定性量化
    - Ensemble: 多模型集成
    """
    
    def __init__(self, models_dir: str = "models"):
        self.models_dir = Path(models_dir)
        self.available_models = self._scan_available_models()
        
    def _scan_available_models(self) -> Dict[str, bool]:
        """扫描可用的模型"""
        available = {
            ModelType.LIGHTGBM: False,
            ModelType.PPO: False,
            ModelType.BAYESIAN: False,
            ModelType.ENSEMBLE: False,
        }
        
        # 检查LightGBM模型
        if (self.models_dir / "lightgbm_config.json").exists():
            available[ModelType.LIGHTGBM] = True
            
        # 检查PPO模型
        ppo_dir = self.models_dir / "ppo"
        if ppo_dir.exists() and (ppo_dir / "actor_network.pth").exists():
            available[ModelType.PPO] = True
            
        # 检查Bayesian优化器
        if (self.models_dir / "bayesian_config.json").exists():
            available[ModelType.BAYESIAN] = True
            
        # Ensemble需要至少2个模型
        if sum(available.values()) >= 2:
            available[ModelType.ENSEMBLE] = True
            
        return available
    
    def select_model(self, features: StandardFeatures, target_format: str, 
                    quality_mode: str, prefer_model: Optional[str] = None) -> ModelType:
        """
        选择最佳模型
        
        策略：
        1. 如果指定prefer_model，优先使用
        2. 如果有PPO模型且在线学习启用，使用PPO
        3. 如果有Ensemble且场景复杂，使用Ensemble
        4. 默认使用LightGBM
        5. 如果都不可用，使用规则引擎
        """
        # 1. 用户指定模型
        if prefer_model:
            model_type = ModelType(prefer_model.lower())
            if self.available_models.get(model_type, False):
                return model_type
        
        # 2. PPO在线学习（如果启用）
        if self.available_models.get(ModelType.PPO, False):
            # PPO适合需要自适应的场景
            if quality_mode == "balanced":
                return ModelType.PPO
        
        # 3. Ensemble集成（复杂场景）
        if self.available_models.get(ModelType.ENSEMBLE, False):
            # 判断场景复杂度
            vec = features.to_vector()
            complexity = np.std(vec)  # 特征方差作为复杂度指标
            if complexity > 2.0:
                return ModelType.ENSEMBLE
        
        # 4. LightGBM默认
        if self.available_models.get(ModelType.LIGHTGBM, False):
            return ModelType.LIGHTGBM
        
        # 5. Bayesian（不确定性场景）
        if self.available_models.get(ModelType.BAYESIAN, False):
            return ModelType.BAYESIAN
        
        # 6. 无模型可用，返回None（使用规则引擎）
        return None
    
    def predict(self, model_type: ModelType, features: StandardFeatures, 
               target_format: str, quality_mode: str) -> StandardPrediction:
        """
        使用指定模型进行预测
        """
        if model_type == ModelType.LIGHTGBM:
            return self._predict_lightgbm(features, target_format, quality_mode)
        elif model_type == ModelType.PPO:
            return self._predict_ppo(features, target_format, quality_mode)
        elif model_type == ModelType.BAYESIAN:
            return self._predict_bayesian(features, target_format, quality_mode)
        elif model_type == ModelType.ENSEMBLE:
            return self._predict_ensemble(features, target_format, quality_mode)
        else:
            # Fallback到智能规则引擎
            return self._predict_rule_based(features, target_format, quality_mode)
    
    def _predict_lightgbm(self, features: StandardFeatures, target_format: str, 
                         quality_mode: str) -> StandardPrediction:
        """LightGBM模型预测"""
        try:
            import lightgbm as lgb
            
            # 加载模型
            model_path = self.models_dir / f"lightgbm_{target_format}_quality.txt"
            if not model_path.exists():
                model_path = self.models_dir / "lightgbm_default_quality.txt"
            
            if model_path.exists():
                booster = lgb.Booster(model_file=str(model_path))
                vec = features.to_vector().reshape(1, -1)
                quality = int(booster.predict(vec)[0])
                confidence = 0.85
            else:
                # 模型文件不存在，使用规则
                return self._predict_rule_based(features, target_format, quality_mode)
            
            # Effort预测（简化版）
            effort = self._estimate_effort(features, quality_mode)
            
            return StandardPrediction(
                quality=quality,
                effort=effort,
                lossless=(quality >= 95),
                format=target_format,
                confidence=confidence,
                estimated_size=int(features.to_vector()[3] * 0.5),  # 简化估算
                estimated_quality=0.95,
                model_version="lightgbm-v1.0"
            )
        except ImportError:
            print("⚠️ LightGBM not installed, falling back to rules", file=sys.stderr)
            return self._predict_rule_based(features, target_format, quality_mode)
    
    def _predict_ppo(self, features: StandardFeatures, target_format: str, 
                    quality_mode: str) -> StandardPrediction:
        """PPO强化学习模型预测"""
        try:
            import torch
            
            # 加载PPO模型
            actor_path = self.models_dir / "ppo" / "actor_network.pth"
            if actor_path.exists():
                # TODO: 实现PPO推理
                # 当前使用规则引擎
                pass
            
            return self._predict_rule_based(features, target_format, quality_mode)
        except ImportError:
            print("⚠️ PyTorch not installed, falling back to rules", file=sys.stderr)
            return self._predict_rule_based(features, target_format, quality_mode)
    
    def _predict_bayesian(self, features: StandardFeatures, target_format: str, 
                         quality_mode: str) -> StandardPrediction:
        """贝叶斯优化器预测"""
        # TODO: 实现贝叶斯优化
        return self._predict_rule_based(features, target_format, quality_mode)
    
    def _predict_ensemble(self, features: StandardFeatures, target_format: str, 
                         quality_mode: str) -> StandardPrediction:
        """集成模型预测（多模型投票）"""
        predictions = []
        
        # 收集所有可用模型的预测
        if self.available_models.get(ModelType.LIGHTGBM, False):
            predictions.append(self._predict_lightgbm(features, target_format, quality_mode))
        
        if self.available_models.get(ModelType.PPO, False):
            predictions.append(self._predict_ppo(features, target_format, quality_mode))
        
        if len(predictions) == 0:
            return self._predict_rule_based(features, target_format, quality_mode)
        
        # 加权平均
        avg_quality = int(np.mean([p.quality for p in predictions]))
        avg_effort = int(np.mean([p.effort for p in predictions]))
        avg_confidence = np.mean([p.confidence for p in predictions])
        
        return StandardPrediction(
            quality=avg_quality,
            effort=avg_effort,
            lossless=any(p.lossless for p in predictions),
            format=target_format,
            confidence=avg_confidence,
            estimated_size=int(np.mean([p.estimated_size for p in predictions])),
            estimated_quality=np.mean([p.estimated_quality for p in predictions]),
            model_version="ensemble-v1.0"
        )
    
    def _predict_rule_based(self, features: StandardFeatures, target_format: str, 
                           quality_mode: str) -> StandardPrediction:
        """
        智能规则引擎（比Rust的硬编码更好）
        
        基于特征的自适应规则
        """
        vec = features.to_vector()
        
        # 基础质量映射
        quality_map = {
            "speed": 65,
            "balanced": 75,
            "quality": 85,
            "lossless": 100
        }
        base_quality = quality_map.get(quality_mode, 75)
        
        # 🔥 自适应调整（基于特征）
        # Basic特征 (0-15)
        width, height = vec[0], vec[1]
        pixels = width * height
        file_size_mb = vec[3]
        has_alpha = vec[5] > 0.5
        is_animated = vec[6] > 0.5
        complexity = vec[7]
        
        # 调整1: 高分辨率图像
        if pixels > 4_000_000:
            base_quality -= 5  # 大图片降低质量以控制文件大小
        
        # 调整2: 透明度
        if has_alpha:
            base_quality += 3  # 透明图像需要更高质量
        
        # 调整3: 复杂度
        if complexity > 0.7:
            base_quality += 5  # 复杂图像需要更高质量
        elif complexity < 0.3:
            base_quality -= 3  # 简单图像可以降低质量
        
        # 调整4: 动画
        if is_animated:
            base_quality -= 5  # 动画帧数多，降低单帧质量
        
        # 限制范围
        quality = max(50, min(100, base_quality))
        
        # Effort估算
        effort = self._estimate_effort(features, quality_mode)
        
        return StandardPrediction(
            quality=quality,
            effort=effort,
            lossless=(quality >= 95),
            format=target_format,
            confidence=0.65,  # 规则引擎置信度较低
            estimated_size=int(file_size_mb * 1024 * 1024 * 0.5),
            estimated_quality=0.92,
            model_version="rule-based-v2.0"
        )
    
    def _estimate_effort(self, features: StandardFeatures, quality_mode: str) -> int:
        """估算编码effort"""
        vec = features.to_vector()
        pixels = vec[0] * vec[1]
        
        if quality_mode == "speed":
            return 4
        elif quality_mode == "quality":
            return 8
        else:  # balanced
            return 6 if pixels > 2_000_000 else 7


def main():
    """CLI入口"""
    parser = argparse.ArgumentParser(description="Python ML Bridge")
    parser.add_argument('--predict', type=str, help='JSON prediction request')
    parser.add_argument('--test', action='store_true', help='Run tests')
    parser.add_argument('--list-models', action='store_true', help='List available models')
    
    args = parser.parse_args()
    
    if args.test:
        # 运行测试
        print("Testing ML bridge...", file=sys.stderr)
        
        features = StandardFeatures(
            basic=[1920.0, 1080.0, 2073600.0, 2.5, 1.78, 0.0, 0.0, 0.65] + [0.0] * 8,
            color=[0.5] * 16,
            texture=[0.3] * 16,
            shape=[0.4] * 16,
            quality=[0.6] * 16,
            metadata=[0.0] * 32,
            context=[0.0] * 16
        )
        
        vec = features.to_vector()
        assert len(vec) == 128, f"Invalid dimension: {len(vec)}"
        print(f"✅ Feature vector: {len(vec)} dimensions", file=sys.stderr)
        
        bridge = MLBridge()
        assert bridge.validate_features(features)
        print("✅ Feature validation: OK", file=sys.stderr)
        
        router = ModelRouter()
        print(f"✅ Available models: {router.available_models}", file=sys.stderr)
        
        model_type = router.select_model(features, "avif", "balanced")
        print(f"✅ Selected model: {model_type}", file=sys.stderr)
        
        prediction = router.predict(model_type, features, "avif", "balanced")
        print(f"✅ Prediction: quality={prediction.quality}, effort={prediction.effort}", file=sys.stderr)
        
        print("\n✅ All tests passed!", file=sys.stderr)
        return
    
    if args.list_models:
        router = ModelRouter()
        print(json.dumps({
            "available_models": {k.value: v for k, v in router.available_models.items()}
        }))
        return
    
    if args.predict:
        # 解析请求
        request = json.loads(args.predict)
        
        # 🎬 检查是否为视频预测
        if request.get('target_format') == 'video':
            # 视频参数预测
            features_vec = request['features']
            quality_mode = request['quality_mode']
            
            # 提取视频特征（前16维）
            width = features_vec[0]
            height = features_vec[1]
            pixels = features_vec[2]
            size_mb = features_vec[3]
            frame_count = features_vec[5]
            fps = features_vec[6]
            duration = features_vec[7]
            has_audio = features_vec[8] > 0.5
            scene_complexity = features_vec[9]
            is_high_res = features_vec[10] > 0.5
            is_long = features_vec[11] > 0.5
            
            print(f"🎬 Video prediction: {width}x{height}, {duration:.1f}s, complexity={scene_complexity:.2f}", file=sys.stderr)
            
            # 智能视频参数预测
            # 编解码器选择
            if is_high_res:
                codec = "h265"  # 高分辨率优先H.265
            else:
                codec = "h264"  # 标准分辨率H.264足够
            
            # CRF质量参数
            if quality_mode == "quality":
                crf = 18 if scene_complexity > 0.7 else 20
            elif quality_mode == "size":
                crf = 28 if scene_complexity > 0.7 else 26
            else:  # balanced
                crf = 23 if scene_complexity > 0.7 else 23
            
            # Preset速度
            if is_long:
                preset = "faster"  # 长视频优先速度
            elif quality_mode == "quality":
                preset = "slow"
            elif quality_mode == "size":
                preset = "slower"
            else:
                preset = "medium"
            
            # Two-pass编码
            two_pass = is_long and quality_mode != "speed"
            
            # 构建响应
            response = {
                "quality": crf,
                "effort": 6,  # 视频不使用effort
                "lossless": False,
                "format_options": [
                    f"codec={codec}",
                    f"preset={preset}",
                    f"two_pass={two_pass}"
                ],
                "confidence": 0.85,
                "model_version": "video-rule-v1.0"
            }
            
            print(f"✅ Video params: codec={codec}, crf={crf}, preset={preset}, two_pass={two_pass}", file=sys.stderr)
            print(json.dumps(response))
        else:
            # 图像预测（原有逻辑）
            # 重建特征
            features = StandardFeatures.from_vector(np.array(request['features']))
            
            # 路由和预测
            router = ModelRouter()
            model_type = router.select_model(
                features, 
                request['target_format'], 
                request['quality_mode'],
                request.get('prefer_model')
            )
            
            print(f"🤖 Using model: {model_type.value if model_type else 'rule-based'}", file=sys.stderr)
            
            prediction = router.predict(model_type, features, request['target_format'], request['quality_mode'])
            
            # 输出JSON响应
            print(prediction.to_json())
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
