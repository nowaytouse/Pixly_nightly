"""
🧠 PIXLY v3.0 本地化AI调度器

替代Go HTTP Gateway的本地化AI预测调度系统：
- 零网络依赖AI调度
- 直接函数调用替代HTTP请求
- 内存对象传递替代JSON序列化
- 多模型智能路由
- 实时性能监控

完全本地化实现，彻底废弃HTTP架构
"""

import time
import json
import threading
from pathlib import Path
from typing import Dict, List, Optional, Any, Union, Callable
from dataclasses import dataclass, asdict
from enum import Enum
import numpy as np


class PredictionStatus(Enum):
    """预测状态"""
    SUCCESS = "success"
    FAILED = "failed"
    TIMEOUT = "timeout"
    MODEL_UNAVAILABLE = "model_unavailable"


@dataclass
class LocalPredictionRequest:
    """本地化预测请求"""
    image_path: str
    tool: str = "jxl"
    target_quality: int = 90
    optimize_mode: str = "balanced"
    
    # 高级选项
    return_advanced_params: bool = False
    enable_feature_analysis: bool = False
    enable_quality_constraint: bool = False
    expected_format: Optional[str] = None
    enable_format_recommendation: bool = False
    enable_video_for_anim: bool = False
    enable_bayesian: bool = False
    enable_ppo: bool = False
    enable_smart_quality: bool = False
    enable_auto_optimize: bool = False
    
    # 模型选择
    model_type: str = "auto"  # lightgbm, ppo, baseline, auto, ensemble
    request_id: Optional[str] = None
    
    # 性能选项
    timeout_seconds: float = 30.0
    enable_caching: bool = True


@dataclass
class LocalPredictionResult:
    """本地化预测结果"""
    success: bool
    status: PredictionStatus
    
    # 预测参数
    quality: int
    distance: float
    effort: int = 6
    lossless: bool = False
    
    # 高级信息
    confidence: float = 0.0
    model_used: str = "unknown"
    model_version: str = "1.0.0"
    inference_time_ms: float = 0.0
    
    # 格式建议
    recommended_format: Optional[str] = None
    format_reason: Optional[str] = None
    used_format: Optional[str] = None
    is_custom_format: bool = False
    
    # 扩展数据
    advanced_params: Dict[str, Any] = None
    merged_params: Dict[str, Any] = None
    features: Dict[str, Any] = None
    reasoning: Optional[str] = None
    preprocessing_steps: List[Dict[str, Any]] = None
    optimization_path: Optional[str] = None
    
    # 错误信息
    error_message: Optional[str] = None
    error_code: Optional[str] = None
    
    def __post_init__(self):
        if self.advanced_params is None:
            self.advanced_params = {}
        if self.merged_params is None:
            self.merged_params = {}
        if self.features is None:
            self.features = {}
        if self.preprocessing_steps is None:
            self.preprocessing_steps = []


@dataclass
class ModelPerformanceStats:
    """模型性能统计"""
    model_name: str
    request_count: int = 0
    success_count: int = 0
    error_count: int = 0
    total_inference_time_ms: float = 0.0
    avg_inference_time_ms: float = 0.0
    avg_confidence: float = 0.0
    last_used: float = 0.0
    
    def update(self, inference_time_ms: float, confidence: float, success: bool):
        """更新统计信息"""
        self.request_count += 1
        self.total_inference_time_ms += inference_time_ms
        
        if success:
            self.success_count += 1
        else:
            self.error_count += 1
        
        # 计算平均值
        if self.request_count > 0:
            self.avg_inference_time_ms = self.total_inference_time_ms / self.request_count
            self.avg_confidence = (self.avg_confidence * (self.request_count - 1) + confidence) / self.request_count
        
        self.last_used = time.time()


class LocalAIDispatcher:
    """
    🧠 本地化AI调度器
    
    替代Go HTTPGateway的核心功能：
    - 本地预测调度 (替代/api/v1/predict)
    - 视频预测支持 (替代/api/v1/predict/video) 
    - 模型管理 (替代/api/v1/models/*)
    - 健康检查 (替代/api/v1/health)
    - 性能监控 (本地化统计)
    """
    
    def __init__(self, models_dir: str = "models", 
                 config_dir: str = "config",
                 cache_dir: str = "cache"):
        
        self.models_dir = Path(models_dir)
        self.config_dir = Path(config_dir)
        self.cache_dir = Path(cache_dir)
        
        # 确保目录存在
        for directory in [self.models_dir, self.config_dir, self.cache_dir]:
            directory.mkdir(parents=True, exist_ok=True)
        
        # 模型管理
        self.model_manager = None  # 延迟初始化
        self.available_models: Dict[str, Any] = {}
        
        # 预测引擎
        self.prediction_engines: Dict[str, Any] = {}
        
        # 缓存系统
        self.prediction_cache: Dict[str, LocalPredictionResult] = {}
        self.cache_hits = 0
        self.cache_misses = 0
        
        # 性能统计
        self.model_stats: Dict[str, ModelPerformanceStats] = {}
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 初始化
        self._initialize_models()
        self._load_prediction_engines()
    
    def _initialize_models(self):
        """初始化可用模型"""
        try:
            # 导入模型管理器和Rust桥接器
            from .model_manager import LocalModelManager
            from .prediction_engine import PredictionEngine
            from .rust_bridge_adapter import get_rust_bridge_adapter
            
            self.model_manager = LocalModelManager(self.models_dir)
            self.rust_adapter = get_rust_bridge_adapter()
            
            # 获取可用模型
            self.available_models = self.model_manager.list_available_models()
            
            print(f"✅ 初始化AI调度器: {len(self.available_models)} 个模型可用")
            
        except ImportError as e:
            print(f"⚠️ 模型管理器导入失败: {e}")
            # 创建基础模型配置
            self.available_models = {
                "baseline": {"type": "baseline", "priority": 50, "enabled": True},
                "lightgbm": {"type": "lightgbm", "priority": 100, "enabled": True},
                "ppo": {"type": "ppo", "priority": 80, "enabled": False}
            }
    
    def _load_prediction_engines(self):
        """加载预测引擎"""
        try:
            # 导入预测引擎
            from .prediction_engine import PredictionEngine
            
            # 为每个模型创建预测引擎
            for model_name, model_config in self.available_models.items():
                if model_config.get("enabled", False):
                    engine = PredictionEngine(
                        model_name=model_name,
                        model_type=model_config["type"],
                        models_dir=self.models_dir
                    )
                    self.prediction_engines[model_name] = engine
                    
                    # 初始化性能统计
                    self.model_stats[model_name] = ModelPerformanceStats(model_name)
            
            print(f"✅ 加载预测引擎: {len(self.prediction_engines)} 个引擎就绪")
            
        except ImportError as e:
            print(f"⚠️ 预测引擎导入失败: {e}")
            # 创建模拟预测引擎
            self._create_fallback_engines()
    
    def _create_fallback_engines(self):
        """创建后备预测引擎"""
        for model_name in self.available_models:
            self.prediction_engines[model_name] = MockPredictionEngine(model_name)
            self.model_stats[model_name] = ModelPerformanceStats(model_name)
    
    # ========== 核心预测接口 (替代Go HTTPGateway) ==========
    
    def predict_with_rust_features(self, request_dict: Dict[str, Any]) -> LocalPredictionResult:
        """使用Rust特征提取进行预测"""
        try:
            # 解析请求
            request = LocalPredictionRequest(
                image_path=request_dict.get("image_path", ""),
                tool=request_dict.get("tool", "webp"),
                target_quality=request_dict.get("target_quality", 85),
                optimize_mode=request_dict.get("optimize_mode", "balanced"),
                enable_caching=request_dict.get("enable_caching", True)
            )
            
            # 如果有Rust适配器且就绪，使用Rust加速
            if hasattr(self, 'rust_adapter') and self.rust_adapter.is_rust_ready():
                return self.rust_adapter.predict_with_rust_features(request)
            else:
                # 回退到原有预测逻辑
                return self.predict_local(request)
                
        except Exception as e:
            return LocalPredictionResult(
                success=False,
                status=PredictionStatus.FAILED,
                quality=85,
                distance=1.0,
                error_message=f"Rust预测失败: {e}",
                error_code="RUST_PREDICTION_ERROR"
            )
    
    def predict_batch(self, requests: List[Dict[str, Any]]) -> List[LocalPredictionResult]:
        """
        本地化图像预测 (替代 /api/v1/predict)
        
        Args:
            request: 本地预测请求对象
        """
        results = []
        for request_dict in requests:
            result = self.predict_with_rust_features(request_dict)
            results.append(result)
        return results
    
    def predict_image(self, request: LocalPredictionRequest) -> LocalPredictionResult:
        """
        本地化图像预测 (替代 /api/v1/predict)
        
        Args:
            request: 本地预测请求对象
            
        Returns:
            LocalPredictionResult: 预测结果
        """
        start_time = time.time()
        
        try:
            # 输入验证
            validation_error = self._validate_request(request)
            if validation_error:
                return LocalPredictionResult(
                    success=False,
                    status=PredictionStatus.FAILED,
                    quality=90,
                    distance=1.0,
                    error_message=validation_error,
                    error_code="VALIDATION_ERROR"
                )
            
            # 缓存检查
            if request.enable_caching:
                cached_result = self._check_cache(request)
                if cached_result:
                    self.cache_hits += 1
                    return cached_result
            
            self.cache_misses += 1
            
            # 模型选择
            selected_model = self._select_model(request)
            if not selected_model:
                return LocalPredictionResult(
                    success=False,
                    status=PredictionStatus.MODEL_UNAVAILABLE,
                    quality=90,
                    distance=1.0,
                    error_message="No suitable model available",
                    error_code="MODEL_UNAVAILABLE"
                )
            
            # 执行预测
            result = self._execute_prediction(request, selected_model)
            
            # 更新性能统计
            inference_time = (time.time() - start_time) * 1000
            result.inference_time_ms = inference_time
            result.model_used = selected_model
            
            self._update_model_stats(selected_model, inference_time, result.confidence, result.success)
            
            # 缓存结果
            if request.enable_caching and result.success:
                self._cache_result(request, result)
            
            return result
            
        except Exception as e:
            # 异常处理
            inference_time = (time.time() - start_time) * 1000
            error_result = LocalPredictionResult(
                success=False,
                status=PredictionStatus.FAILED,
                quality=90,
                distance=1.0,
                inference_time_ms=inference_time,
                error_message=f"Prediction failed: {str(e)}",
                error_code="PREDICTION_ERROR"
            )
            
            print(f"❌ 预测失败: {e}")
            return error_result
    
    def predict_video(self, video_path: str, **kwargs) -> LocalPredictionResult:
        """
        本地化视频预测 (替代 /api/v1/predict/video)
        
        Args:
            video_path: 视频文件路径
            **kwargs: 其他预测参数
            
        Returns:
            LocalPredictionResult: 预测结果
        """
        # 创建视频预测请求
        request = LocalPredictionRequest(
            image_path=video_path,  # 复用image_path字段
            tool=kwargs.get('tool', 'mp4'),
            target_quality=kwargs.get('target_quality', 90),
            optimize_mode=kwargs.get('optimize_mode', 'balanced')
        )
        
        # 标记为视频预测
        request.expected_format = "video"
        
        # 执行预测 (复用图像预测逻辑)
        result = self.predict_image(request)
        
        # 视频特定后处理
        if result.success:
            result.reasoning = f"视频预测: {result.reasoning or ''}"
            
        return result
    
    def _validate_request(self, request: LocalPredictionRequest) -> Optional[str]:
        """验证预测请求"""
        if not request.image_path:
            return "Image path is required"
        
        image_path = Path(request.image_path)
        if not image_path.exists():
            return f"Image file not found: {request.image_path}"
        
        if request.target_quality < 1 or request.target_quality > 100:
            return "Target quality must be between 1 and 100"
        
        valid_tools = {"jxl", "avif", "webp", "png", "jpeg", "mp4", "mov"}
        if request.tool not in valid_tools:
            return f"Unsupported tool: {request.tool}"
        
        return None
    
    def _check_cache(self, request: LocalPredictionRequest) -> Optional[LocalPredictionResult]:
        """检查预测缓存"""
        cache_key = self._generate_cache_key(request)
        
        with self._lock:
            return self.prediction_cache.get(cache_key)
    
    def _cache_result(self, request: LocalPredictionRequest, result: LocalPredictionResult):
        """缓存预测结果"""
        cache_key = self._generate_cache_key(request)
        
        with self._lock:
            # 限制缓存大小
            if len(self.prediction_cache) > 1000:
                # 删除最老的缓存项
                oldest_key = next(iter(self.prediction_cache))
                del self.prediction_cache[oldest_key]
            
            self.prediction_cache[cache_key] = result
    
    def _generate_cache_key(self, request: LocalPredictionRequest) -> str:
        """生成缓存键"""
        import hashlib
        
        # 基于关键参数生成哈希
        key_data = f"{request.image_path}:{request.tool}:{request.target_quality}:{request.optimize_mode}:{request.model_type}"
        return hashlib.md5(key_data.encode()).hexdigest()[:16]
    
    def _select_model(self, request: LocalPredictionRequest) -> Optional[str]:
        """选择最优模型"""
        if request.model_type == "auto" or request.model_type == "ensemble":
            # 自动选择：优先级最高的可用模型
            available_models = [(name, config) for name, config in self.available_models.items() 
                              if config.get("enabled", False) and name in self.prediction_engines]
            
            if not available_models:
                return None
            
            # 按优先级排序
            available_models.sort(key=lambda x: x[1].get("priority", 0), reverse=True)
            return available_models[0][0]
        
        elif request.model_type in self.available_models:
            # 指定模型
            if (self.available_models[request.model_type].get("enabled", False) and
                request.model_type in self.prediction_engines):
                return request.model_type
        
        # 后备模型
        return "baseline" if "baseline" in self.prediction_engines else None
    
    def _execute_prediction(self, request: LocalPredictionRequest, model_name: str) -> LocalPredictionResult:
        """执行预测"""
        engine = self.prediction_engines[model_name]
        
        try:
            # 调用预测引擎
            if hasattr(engine, 'predict'):
                result = engine.predict(request)
            else:
                # 后备预测逻辑
                result = self._fallback_prediction(request, model_name)
            
            return result
            
        except Exception as e:
            return LocalPredictionResult(
                success=False,
                status=PredictionStatus.FAILED,
                quality=request.target_quality,
                distance=1.0,
                model_used=model_name,
                error_message=f"Engine prediction failed: {str(e)}",
                error_code="ENGINE_ERROR"
            )
    
    def _fallback_prediction(self, request: LocalPredictionRequest, model_name: str) -> LocalPredictionResult:
        """后备预测逻辑"""
        # 基于优化模式的简单规则
        quality_map = {
            "size": max(70, request.target_quality - 10),
            "balanced": request.target_quality,
            "quality": min(100, request.target_quality + 5)
        }
        
        predicted_quality = quality_map.get(request.optimize_mode, request.target_quality)
        
        # 模拟置信度
        confidence = 0.7 + (abs(predicted_quality - 90) / 100 * 0.2)
        
        return LocalPredictionResult(
            success=True,
            status=PredictionStatus.SUCCESS,
            quality=predicted_quality,
            distance=0.95 + np.random.random() * 0.04,  # 0.95-0.99
            effort=6,
            confidence=confidence,
            model_used=model_name,
            model_version="1.0.0",
            recommended_format=request.tool,
            format_reason=f"基于{request.optimize_mode}模式的推荐",
            used_format=request.tool,
            reasoning=f"使用{model_name}模型，{request.optimize_mode}模式优化"
        )
    
    def _update_model_stats(self, model_name: str, inference_time_ms: float, 
                          confidence: float, success: bool):
        """更新模型统计"""
        with self._lock:
            if model_name in self.model_stats:
                self.model_stats[model_name].update(inference_time_ms, confidence, success)
    
    # ========== 管理接口 (替代Go模型管理端点) ==========
    
    def list_models(self) -> Dict[str, Any]:
        """列出可用模型 (替代 /api/v1/models)"""
        models_info = {}
        
        for model_name, model_config in self.available_models.items():
            stats = self.model_stats.get(model_name, ModelPerformanceStats(model_name))
            
            models_info[model_name] = {
                "name": model_name,
                "type": model_config.get("type", "unknown"),
                "enabled": model_config.get("enabled", False),
                "priority": model_config.get("priority", 0),
                "version": model_config.get("version", "1.0.0"),
                "stats": {
                    "request_count": stats.request_count,
                    "success_rate": stats.success_count / max(1, stats.request_count),
                    "avg_inference_time_ms": stats.avg_inference_time_ms,
                    "avg_confidence": stats.avg_confidence,
                    "last_used": stats.last_used
                }
            }
        
        return {
            "models": models_info,
            "total_models": len(models_info),
            "active_models": len([m for m in models_info.values() if m["enabled"]])
        }
    
    def get_health_status(self) -> Dict[str, Any]:
        """获取健康状态 (替代 /api/v1/health)"""
        total_engines = len(self.prediction_engines)
        working_engines = 0
        
        # 检查每个引擎的健康状态
        engine_status = {}
        for name, engine in self.prediction_engines.items():
            try:
                # 简单健康检查
                is_healthy = hasattr(engine, 'predict') or hasattr(engine, 'is_healthy')
                engine_status[name] = "healthy" if is_healthy else "unhealthy"
                if is_healthy:
                    working_engines += 1
            except Exception:
                engine_status[name] = "error"
        
        overall_health = "healthy" if working_engines == total_engines else ("degraded" if working_engines > 0 else "unhealthy")
        
        return {
            "status": overall_health,
            "version": "3.0.0",
            "ready": working_engines > 0,
            "engines": engine_status,
            "stats": {
                "total_engines": total_engines,
                "working_engines": working_engines,
                "cache_hits": self.cache_hits,
                "cache_misses": self.cache_misses,
                "cache_hit_rate": self.cache_hits / max(1, self.cache_hits + self.cache_misses)
            }
        }
    
    def get_capabilities(self) -> Dict[str, Any]:
        """获取服务能力 (替代 /api/v1/capabilities)"""
        return {
            "version": "3.0.0",
            "architecture": "local_native",
            "features": {
                "image_prediction": True,
                "video_prediction": True,
                "multi_model_support": True,
                "caching": True,
                "performance_monitoring": True,
                "zero_network_dependency": True
            },
            "supported_formats": {
                "image": ["jxl", "avif", "webp", "png", "jpeg"],
                "video": ["mp4", "mov", "avi", "mkv", "webm"]
            },
            "models": list(self.available_models.keys()),
            "optimization_modes": ["size", "balanced", "quality", "universal"],
            "local_dispatcher": True,
            "http_gateway_replacement": True
        }
    
    def get_performance_stats(self) -> Dict[str, Any]:
        """获取性能统计"""
        with self._lock:
            stats = {
                "dispatcher": {
                    "cache_hits": self.cache_hits,
                    "cache_misses": self.cache_misses,
                    "cache_hit_rate": self.cache_hits / max(1, self.cache_hits + self.cache_misses),
                    "cached_results": len(self.prediction_cache)
                },
                "models": {}
            }
            
            for model_name, model_stats in self.model_stats.items():
                stats["models"][model_name] = asdict(model_stats)
            
            return stats


class MockPredictionEngine:
    """模拟预测引擎 (用于测试)"""
    
    def __init__(self, model_name: str):
        self.model_name = model_name
    
    def predict(self, request: LocalPredictionRequest) -> LocalPredictionResult:
        """模拟预测"""
        # 简单的模拟逻辑
        confidence = 0.8 + np.random.random() * 0.15
        
        return LocalPredictionResult(
            success=True,
            status=PredictionStatus.SUCCESS,
            quality=request.target_quality,
            distance=0.95 + np.random.random() * 0.04,
            confidence=confidence,
            model_used=self.model_name,
            model_version="1.0.0-mock",
            reasoning=f"模拟{self.model_name}模型预测"
        )
    
    def is_healthy(self) -> bool:
        return True
