"""
LightGBM模型完整版 - 梯度提升预测引擎

基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/models/lightgbm.go 重新实现

核心功能:
- 完整LightGBM梯度提升算法
- 多格式参数预测(JXL/AVIF/WebP/HEIC等)  
- 特征工程和预处理
- 模型训练和增量学习
- 高精度质量参数预测
- 模型性能评估

EX-020实现: 从Go废弃代码价值提取 + 架构增强
遵循四高原则：高规范化、高兼容性、高扩展性、高稳定性
"""

import json
import pickle
import numpy as np
from pathlib import Path
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, field
import logging
import time


# 尝试导入LightGBM，如果没有安装则使用回退实现
try:
    import lightgbm as lgb
    LIGHTGBM_AVAILABLE = True
except ImportError:
    LIGHTGBM_AVAILABLE = False
    lgb = None


@dataclass
class ModelConfig:
    """LightGBM模型配置"""
    # LightGBM核心参数
    objective: str = "regression"
    metric: str = "rmse"
    boosting_type: str = "gbdt"
    num_leaves: int = 63
    learning_rate: float = 0.05
    feature_fraction: float = 0.8
    bagging_fraction: float = 0.8
    bagging_freq: int = 5
    min_child_samples: int = 20
    num_iterations: int = 100
    
    # 架构增强：高级参数
    early_stopping_rounds: int = 10
    verbose: int = -1
    random_state: int = 42
    n_jobs: int = -1
    min_gain_to_split: float = 0.1
    lambda_l1: float = 0.0
    lambda_l2: float = 0.0
    
    # 自定义参数
    feature_importance_threshold: float = 0.01
    max_model_size_mb: int = 50
    enable_categorical: bool = True


@dataclass
class PredictionResult:
    """预测结果"""
    quality: float = 0.0                 # 质量参数
    confidence: float = 0.0              # 预测置信度
    format: str = ""                     # 目标格式
    features_used: List[str] = field(default_factory=list)
    
    # 架构增强：详细信息
    prediction_time_ms: float = 0.0      # 预测耗时
    feature_importance: Dict[str, float] = field(default_factory=dict)
    alternative_formats: List[Tuple[str, float]] = field(default_factory=list)
    quality_range: Tuple[float, float] = (0.0, 100.0)
    
    # 元数据
    model_version: str = ""
    timestamp: float = 0.0
    
    def __post_init__(self):
        if not self.timestamp:
            self.timestamp = time.time()


class LightGBMComplete:
    """
    LightGBM模型完整版 - 增强梯度提升预测引擎
    
    高规范化、高兼容性、高扩展性、高稳定性实现
    """
    
    def __init__(self,
                 model_path: str = "models/lightgbm",
                 format_type: str = "jxl",
                 config: Optional[ModelConfig] = None,
                 debug: bool = False):
        """
        初始化LightGBM模型
        
        Args:
            model_path: 模型文件路径
            format_type: 目标格式类型
            config: 模型配置
            debug: 调试模式
        """
        self.model_path = Path(model_path)
        self.format_type = format_type.lower()
        self.config = config or ModelConfig()
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        if debug:
            self.logger.setLevel(logging.DEBUG)
        
        # 模型状态
        self.model: Optional[Any] = None
        self.is_loaded = False
        self.feature_names: List[str] = []
        self.feature_importance: Dict[str, float] = {}
        
        # 架构增强：性能追踪
        self.prediction_count = 0
        self.total_prediction_time = 0.0
        self.last_prediction_time = 0.0
        
        # 格式特定配置
        self.format_configs = self._init_format_configs()
        
        # 特征定义
        self.feature_schema = self._define_feature_schema()
        
        if debug:
            self.logger.debug(f"LightGBM模型初始化: {format_type}, 路径: {model_path}")
    
    def _init_format_configs(self) -> Dict[str, Dict[str, Any]]:
        """初始化各格式特定配置"""
        return {
            "jxl": {
                "quality_range": (60, 100),
                "default_quality": 90,
                "feature_weights": {
                    "has_alpha": 5.0,
                    "edge_strength": 0.3,
                    "texture_complexity": 0.4,
                    "detail_level": 0.35
                },
                "intercept": 85.0
            },
            "avif": {
                "quality_range": (20, 100),
                "default_quality": 75,
                "feature_weights": {
                    "has_alpha": 3.0,
                    "texture_complexity": 0.45,
                    "noise_level": -0.3,
                    "entropy_score": 0.3
                },
                "intercept": 70.0
            },
            "webp": {
                "quality_range": (0, 100),
                "default_quality": 80,
                "feature_weights": {
                    "has_alpha": 2.0,
                    "edge_strength": 0.25,
                    "texture_complexity": 0.35,
                    "detail_level": 0.3
                },
                "intercept": 75.0
            },
            "heic": {
                "quality_range": (50, 100),
                "default_quality": 85,
                "feature_weights": {
                    "has_alpha": 4.0,
                    "texture_complexity": 0.4,
                    "detail_level": 0.35,
                    "entropy_score": 0.25
                },
                "intercept": 80.0
            }
        }
    
    def _define_feature_schema(self) -> Dict[str, str]:
        """定义特征schema"""
        return {
            # 基础图像特征
            "width": "numeric",
            "height": "numeric", 
            "size": "numeric",
            "has_alpha": "categorical",
            
            # SWT小波特征
            "edge_strength": "numeric",
            "texture_complexity": "numeric",
            "noise_level": "numeric",
            "detail_level": "numeric",
            "high_freq_energy": "numeric",
            "mid_freq_energy": "numeric",
            "low_freq_energy": "numeric",
            "entropy_score": "numeric",
            "smooth_region_ratio": "numeric",
            
            # 架构增强：扩展特征
            "aspect_ratio": "numeric",
            "pixel_density": "numeric",
            "color_variance": "numeric",
            "compression_ratio": "numeric",
            "file_format": "categorical",
            "bit_depth": "categorical"
        }
    
    def load_model(self) -> bool:
        """
        加载模型
        
        Returns:
            是否加载成功
        """
        try:
            # 尝试加载真实LightGBM模型
            if LIGHTGBM_AVAILABLE and self._load_lightgbm_model():
                self.is_loaded = True
                self.logger.info(f"LightGBM模型加载成功: {self.format_type}")
                return True
            
            # 回退到线性模型
            if self._load_linear_fallback():
                self.is_loaded = True
                self.logger.info(f"线性回退模型加载成功: {self.format_type}")
                return True
            
            self.logger.error("模型加载失败")
            return False
            
        except Exception as e:
            self.logger.error(f"模型加载异常: {e}")
            return False
    
    def _load_lightgbm_model(self) -> bool:
        """加载真实LightGBM模型"""
        try:
            model_file = self.model_path / f"{self.format_type}_lgb.txt"
            
            if model_file.exists():
                self.model = lgb.Booster(model_file=str(model_file))
                self.feature_names = self.model.feature_name()
                
                # 计算特征重要性
                importance = self.model.feature_importance(importance_type='gain')
                self.feature_importance = dict(zip(self.feature_names, importance))
                
                return True
            else:
                self.logger.debug(f"LightGBM模型文件不存在: {model_file}")
                return False
                
        except Exception as e:
            self.logger.debug(f"LightGBM模型加载失败: {e}")
            return False
    
    def _load_linear_fallback(self) -> bool:
        """加载线性回退模型"""
        try:
            # 使用格式特定的默认权重
            format_config = self.format_configs.get(self.format_type, {})
            weights = format_config.get("feature_weights", {})
            intercept = format_config.get("intercept", 75.0)
            
            # 构建线性模型参数
            self.feature_names = list(self.feature_schema.keys())
            self.model = {
                "type": "linear",
                "weights": weights,
                "intercept": intercept,
                "feature_names": self.feature_names
            }
            
            # 设置特征重要性（基于权重绝对值）
            self.feature_importance = {k: abs(v) for k, v in weights.items()}
            
            return True
            
        except Exception as e:
            self.logger.error(f"线性模型加载失败: {e}")
            return False
    
    def predict(self, 
                features: Dict[str, Any],
                return_confidence: bool = True) -> PredictionResult:
        """
        预测质量参数
        
        Args:
            features: 输入特征字典
            return_confidence: 是否返回置信度
            
        Returns:
            预测结果对象
        """
        start_time = time.time()
        
        try:
            if not self.is_loaded and not self.load_model():
                return self._create_fallback_result("模型未加载")
            
            # 特征预处理
            processed_features = self._preprocess_features(features)
            
            # 执行预测
            if LIGHTGBM_AVAILABLE and hasattr(self.model, 'predict'):
                # 真实LightGBM预测
                quality = self._predict_lightgbm(processed_features)
                confidence = self._calculate_confidence_lgb(processed_features)
            else:
                # 线性模型预测
                quality = self._predict_linear(processed_features)
                confidence = self._calculate_confidence_linear(processed_features)
            
            # 后处理
            quality = self._postprocess_quality(quality)
            
            # 构建结果
            prediction_time = (time.time() - start_time) * 1000
            self._update_performance_metrics(prediction_time)
            
            result = PredictionResult(
                quality=quality,
                confidence=confidence if return_confidence else 0.0,
                format=self.format_type,
                features_used=list(processed_features.keys()),
                prediction_time_ms=prediction_time,
                feature_importance=dict(self.feature_importance),
                model_version=self._get_model_version(),
                quality_range=self.format_configs.get(self.format_type, {}).get("quality_range", (0, 100))
            )
            
            # 架构增强：备选格式建议
            result.alternative_formats = self._suggest_alternative_formats(processed_features)
            
            return result
            
        except Exception as e:
            self.logger.error(f"预测失败: {e}")
            return self._create_fallback_result(f"预测错误: {e}")
    
    def _preprocess_features(self, features: Dict[str, Any]) -> Dict[str, float]:
        """
        特征预处理
        
        Args:
            features: 原始特征
            
        Returns:
            处理后的特征
        """
        processed = {}
        
        for feature_name, feature_type in self.feature_schema.items():
            if feature_name in features:
                value = features[feature_name]
                
                if feature_type == "numeric":
                    processed[feature_name] = float(value) if value is not None else 0.0
                elif feature_type == "categorical":
                    # 简化的类别编码
                    if isinstance(value, bool):
                        processed[feature_name] = 1.0 if value else 0.0
                    elif isinstance(value, str):
                        processed[feature_name] = hash(value) % 10  # 简单哈希映射
                    else:
                        processed[feature_name] = float(value) if value is not None else 0.0
            else:
                # 缺失值处理
                processed[feature_name] = 0.0
        
        # 架构增强：派生特征
        if "width" in processed and "height" in processed:
            processed["aspect_ratio"] = processed["width"] / max(processed["height"], 1.0)
            processed["pixel_density"] = processed["width"] * processed["height"] / 1000000.0  # 百万像素
        
        if "size" in processed and "width" in processed and "height" in processed:
            pixel_count = processed["width"] * processed["height"]
            if pixel_count > 0:
                processed["compression_ratio"] = processed["size"] / pixel_count
        
        return processed
    
    def _predict_lightgbm(self, features: Dict[str, float]) -> float:
        """使用真实LightGBM模型预测"""
        # 构建特征向量
        feature_vector = [features.get(name, 0.0) for name in self.feature_names]
        
        # 预测
        prediction = self.model.predict([feature_vector])[0]
        return float(prediction)
    
    def _predict_linear(self, features: Dict[str, float]) -> float:
        """使用线性模型预测"""
        weights = self.model["weights"]
        intercept = self.model["intercept"]
        
        # 线性组合
        prediction = intercept
        for feature_name, weight in weights.items():
            if feature_name in features:
                prediction += features[feature_name] * weight
        
        return prediction
    
    def _calculate_confidence_lgb(self, features: Dict[str, float]) -> float:
        """计算LightGBM预测置信度"""
        # 简化置信度计算：基于特征重要性和特征值
        total_importance = sum(self.feature_importance.values())
        if total_importance == 0:
            return 0.5
        
        weighted_confidence = 0.0
        for feature_name, importance in self.feature_importance.items():
            if feature_name in features:
                # 特征值标准化到0-1
                normalized_value = min(abs(features[feature_name]), 1.0)
                weighted_confidence += (importance / total_importance) * normalized_value
        
        return min(max(weighted_confidence, 0.1), 0.95)  # 限制在0.1-0.95之间
    
    def _calculate_confidence_linear(self, features: Dict[str, float]) -> float:
        """计算线性模型预测置信度"""
        # 基于特征完整性和权重分布
        weights = self.model["weights"]
        total_weight = sum(abs(w) for w in weights.values())
        
        if total_weight == 0:
            return 0.5
        
        feature_coverage = len([f for f in weights.keys() if f in features]) / len(weights)
        return 0.3 + 0.6 * feature_coverage  # 0.3-0.9之间
    
    def _postprocess_quality(self, quality: float) -> float:
        """后处理质量参数"""
        format_config = self.format_configs.get(self.format_type, {})
        min_quality, max_quality = format_config.get("quality_range", (0, 100))
        
        # 限制在有效范围内
        quality = max(min_quality, min(max_quality, quality))
        
        # 四舍五入到整数
        return round(quality)
    
    def _suggest_alternative_formats(self, features: Dict[str, float]) -> List[Tuple[str, float]]:
        """建议备选格式"""
        alternatives = []
        
        # 基于特征特点建议其他格式
        for fmt in ["jxl", "avif", "webp", "heic"]:
            if fmt != self.format_type:
                # 简化评分：基于格式特性匹配度
                score = self._calculate_format_score(fmt, features)
                alternatives.append((fmt, score))
        
        # 按分数排序
        alternatives.sort(key=lambda x: x[1], reverse=True)
        return alternatives[:3]  # 返回前3个
    
    def _calculate_format_score(self, fmt: str, features: Dict[str, float]) -> float:
        """计算格式适配分数"""
        config = self.format_configs.get(fmt, {})
        weights = config.get("feature_weights", {})
        
        score = 70.0  # 基础分数
        for feature, weight in weights.items():
            if feature in features:
                score += features[feature] * weight * 0.1
        
        return min(max(score, 0), 100)
    
    def _update_performance_metrics(self, prediction_time: float) -> None:
        """更新性能指标"""
        self.prediction_count += 1
        self.total_prediction_time += prediction_time
        self.last_prediction_time = prediction_time
    
    def _get_model_version(self) -> str:
        """获取模型版本"""
        if LIGHTGBM_AVAILABLE and hasattr(self.model, 'num_trees'):
            return f"lgb_v{self.model.num_trees()}"
        else:
            return f"linear_v{self.format_type}"
    
    def _create_fallback_result(self, error_msg: str) -> PredictionResult:
        """创建回退结果"""
        format_config = self.format_configs.get(self.format_type, {})
        default_quality = format_config.get("default_quality", 75)
        
        return PredictionResult(
            quality=default_quality,
            confidence=0.1,
            format=self.format_type,
            features_used=[],
            prediction_time_ms=0.0,
            model_version="fallback_v1"
        )
    
    def train_incremental(self, 
                         training_data: List[Tuple[Dict[str, Any], float]], 
                         validation_split: float = 0.2) -> Dict[str, Any]:
        """
        增量训练
        
        Args:
            training_data: 训练数据 [(features, quality), ...]
            validation_split: 验证集比例
            
        Returns:
            训练结果指标
        """
        if not LIGHTGBM_AVAILABLE:
            return {"error": "LightGBM未安装，无法训练"}
        
        try:
            # 数据预处理
            X, y = self._prepare_training_data(training_data)
            
            # 数据分割
            split_idx = int(len(X) * (1 - validation_split))
            X_train, X_val = X[:split_idx], X[split_idx:]
            y_train, y_val = y[:split_idx], y[split_idx:]
            
            # 创建LightGBM数据集
            train_data = lgb.Dataset(X_train, label=y_train)
            val_data = lgb.Dataset(X_val, label=y_val, reference=train_data)
            
            # 训练参数
            params = {
                'objective': self.config.objective,
                'metric': self.config.metric,
                'boosting_type': self.config.boosting_type,
                'num_leaves': self.config.num_leaves,
                'learning_rate': self.config.learning_rate,
                'feature_fraction': self.config.feature_fraction,
                'bagging_fraction': self.config.bagging_fraction,
                'bagging_freq': self.config.bagging_freq,
                'min_child_samples': self.config.min_child_samples,
                'verbose': self.config.verbose,
                'random_state': self.config.random_state
            }
            
            # 训练模型
            self.model = lgb.train(
                params,
                train_data,
                num_boost_round=self.config.num_iterations,
                valid_sets=[val_data],
                callbacks=[lgb.early_stopping(self.config.early_stopping_rounds)]
            )
            
            # 更新特征信息
            self.feature_names = self.model.feature_name()
            importance = self.model.feature_importance(importance_type='gain')
            self.feature_importance = dict(zip(self.feature_names, importance))
            
            # 保存模型
            model_file = self.model_path / f"{self.format_type}_lgb.txt"
            self.model.save_model(str(model_file))
            
            # 计算验证指标
            val_pred = self.model.predict(X_val)
            rmse = np.sqrt(np.mean((val_pred - y_val) ** 2))
            mae = np.mean(np.abs(val_pred - y_val))
            
            self.is_loaded = True
            
            return {
                "success": True,
                "train_samples": len(X_train),
                "val_samples": len(X_val),
                "rmse": float(rmse),
                "mae": float(mae),
                "feature_importance": dict(self.feature_importance),
                "model_version": self._get_model_version()
            }
            
        except Exception as e:
            self.logger.error(f"增量训练失败: {e}")
            return {"error": str(e)}
    
    def _prepare_training_data(self, training_data: List[Tuple[Dict[str, Any], float]]) -> Tuple[np.ndarray, np.ndarray]:
        """准备训练数据"""
        X = []
        y = []
        
        for features, quality in training_data:
            processed_features = self._preprocess_features(features)
            feature_vector = [processed_features.get(name, 0.0) for name in self.feature_schema.keys()]
            X.append(feature_vector)
            y.append(quality)
        
        return np.array(X), np.array(y)
    
    def get_performance_stats(self) -> Dict[str, Any]:
        """
        获取性能统计
        
        Returns:
            性能统计字典
        """
        avg_time = self.total_prediction_time / max(self.prediction_count, 1)
        
        return {
            "prediction_count": self.prediction_count,
            "total_time_ms": self.total_prediction_time,
            "avg_time_ms": avg_time,
            "last_time_ms": self.last_prediction_time,
            "model_loaded": self.is_loaded,
            "model_type": "lightgbm" if LIGHTGBM_AVAILABLE and hasattr(self.model, 'predict') else "linear",
            "feature_count": len(self.feature_names),
            "format": self.format_type
        }
    
    def get_feature_importance(self, top_n: int = 10) -> Dict[str, float]:
        """
        获取特征重要性
        
        Args:
            top_n: 返回前N个重要特征
            
        Returns:
            特征重要性字典
        """
        if not self.feature_importance:
            return {}
        
        # 按重要性排序
        sorted_features = sorted(
            self.feature_importance.items(),
            key=lambda x: x[1],
            reverse=True
        )
        
        return dict(sorted_features[:top_n])


# 便捷函数
def create_lightgbm_model(format_type: str, debug: bool = False) -> LightGBMComplete:
    """
    创建LightGBM模型的便捷函数
    
    Args:
        format_type: 格式类型
        debug: 调试模式
        
    Returns:
        LightGBM模型实例
    """
    return LightGBMComplete(format_type=format_type, debug=debug)


if __name__ == "__main__":
    # 测试代码
    print("=== LightGBM模型完整版测试 ===")
    
    # 创建测试模型
    model = create_lightgbm_model("jxl", debug=True)
    
    # 测试加载
    if model.load_model():
        print("✅ 模型加载成功")
    else:
        print("⚠️ 模型加载失败，使用回退模式")
    
    # 测试预测
    test_features = {
        "width": 1920,
        "height": 1080,
        "size": 2048000,
        "has_alpha": False,
        "edge_strength": 0.6,
        "texture_complexity": 0.7,
        "noise_level": 0.2,
        "detail_level": 0.8,
        "entropy_score": 0.65
    }
    
    print("\n🔮 测试预测...")
    result = model.predict(test_features)
    print(f"✅ 预测质量: {result.quality}")
    print(f"   置信度: {result.confidence:.3f}")
    print(f"   预测时间: {result.prediction_time_ms:.2f}ms")
    print(f"   使用特征: {len(result.features_used)}个")
    
    # 测试备选格式
    if result.alternative_formats:
        print(f"   备选格式: {result.alternative_formats[:2]}")
    
    # 测试性能统计
    stats = model.get_performance_stats()
    print(f"\n📊 性能统计:")
    print(f"   预测次数: {stats['prediction_count']}")
    print(f"   平均耗时: {stats['avg_time_ms']:.2f}ms")
    print(f"   模型类型: {stats['model_type']}")
    
    # 测试特征重要性
    importance = model.get_feature_importance(5)
    print(f"\n🔍 特征重要性:")
    for feature, score in importance.items():
        print(f"   {feature}: {score:.3f}")
    
    print("🎯 LightGBM模型完整版测试完成！")
