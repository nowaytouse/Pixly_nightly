"""
🧠 PIXLY v3.1 多阶段智能执行流水线

替代Go precision_modes.go的AdvancedPipeline功能：
- 工具特定的multi-stage优化pipeline
- 条件执行引擎 - 基于图像特征智能决策
- 动态参数微调 - 纹理复杂度/噪声驱动优化
- JXL/AVIF/WebP高级选项优化

完全本地化，零网络依赖的智能化处理流程
"""

import time
import json
from typing import Dict, List, Optional, Any, Callable
from dataclasses import dataclass, asdict
from enum import Enum
import threading

from .bayesian_optimizer import BayesianOptimizer, Observation
from .knowledge_system import get_knowledge_system


class PrecisionMode(Enum):
    """精度模式"""
    BASIC = "basic"          # 基础模式：仅贝叶斯优化
    ADVANCED = "advanced"    # 进阶模式：多阶段智能执行


@dataclass
class ImageFeaturesAdvanced:
    """高级图像特征 - 替代Go ImageFeaturesAdvanced"""
    # 基础特征
    width: int = 0
    height: int = 0
    has_alpha: bool = False
    is_animated: bool = False
    frame_count: int = 0
    
    # SWT小波特征（9维）
    edge_strength: float = 0.0      # 边缘强度
    texture_complexity: float = 0.0  # 纹理复杂度
    noise_level: float = 0.0         # 噪声级别
    detail_level: float = 0.0        # 细节级别
    high_freq_energy: float = 0.0    # 高频能量
    mid_freq_energy: float = 0.0     # 中频能量
    low_freq_energy: float = 0.0     # 低频能量
    overall_quality: float = 0.0     # 整体质量
    compression_score: float = 0.0   # 压缩性评分
    
    # 颜色特征
    color_space: str = "sRGB"
    color_range: float = 0.0         # 颜色范围
    saturation: float = 0.0          # 饱和度
    brightness: float = 0.0          # 亮度
    contrast: float = 0.0            # 对比度
    
    # 内容特征
    is_photo: bool = False           # 是否照片
    is_document: bool = False        # 是否文档
    is_screenshot: bool = False      # 是否截图
    has_text: bool = False           # 是否包含文字
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典格式"""
        return asdict(self)
    
    def get_complexity_score(self) -> float:
        """计算综合复杂度评分"""
        return (
            self.texture_complexity * 0.4 +
            self.edge_strength * 0.3 +
            self.detail_level * 0.2 +
            self.noise_level * 0.1
        ) / 100.0


@dataclass
class Stage:
    """执行阶段"""
    name: str
    description: str
    order: int                                    # 执行顺序
    options: Dict[str, Any]                       # 阶段选项
    required: bool = True                         # 是否必须
    condition: Optional[Callable] = None          # 执行条件
    
    def should_execute(self, features: ImageFeaturesAdvanced) -> bool:
        """判断是否应该执行此阶段"""
        if not self.condition:
            return self.required
        return self.condition(features)


@dataclass
class StageResult:
    """阶段执行结果"""
    name: str
    order: int
    executed: bool
    options: Dict[str, Any]
    execution_time_ms: float = 0.0
    success: bool = True
    error_message: str = ""


@dataclass
class AdvancedResult:
    """高级模式执行结果"""
    tool: str
    mode: str
    base_quality: int
    base_distance: float
    final_quality: int
    final_distance: float
    stages: List[StageResult]
    total_execution_time_ms: float = 0.0
    confidence: float = 0.0
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典格式"""
        return {
            "tool": self.tool,
            "mode": self.mode,
            "base_quality": self.base_quality,
            "base_distance": self.base_distance,
            "final_quality": self.final_quality,
            "final_distance": self.final_distance,
            "stages": [asdict(stage) for stage in self.stages],
            "total_execution_time_ms": self.total_execution_time_ms,
            "confidence": self.confidence
        }


class AdvancedPipeline:
    """
    🧠 多阶段智能执行流水线
    
    替代Go AdvancedPipeline的完整Python实现
    """
    
    def __init__(self, tool: str, features: ImageFeaturesAdvanced):
        self.tool = tool
        self.features = features
        self.stages = self._build_stages()
        self.bayesian_opt = BayesianOptimizer(tool, debug=False)
        self.knowledge_system = get_knowledge_system()
        self._lock = threading.RLock()
        
        print(f"✅ 高级流水线初始化: {tool} (特征维度: {len(asdict(features))})")
    
    def execute(self, target_mode: str = "balanced") -> AdvancedResult:
        """
        执行多阶段流水线（最佳顺序）
        
        Args:
            target_mode: 目标模式 ("size", "balanced", "quality")
            
        Returns:
            AdvancedResult: 执行结果
        """
        start_time = time.time()
        
        result = AdvancedResult(
            tool=self.tool,
            mode=target_mode,
            base_quality=0,
            base_distance=0.0,
            final_quality=0,
            final_distance=0.0,
            stages=[]
        )
        
        try:
            with self._lock:
                # 阶段1: 贝叶斯参数预测
                base_quality, base_distance = self._predict_base_parameters(target_mode)
                result.base_quality = base_quality
                result.base_distance = base_distance
                
                # 阶段2-N: 执行各个阶段（按顺序）
                executed_stages = []
                for stage in sorted(self.stages, key=lambda s: s.order):
                    stage_result = self._execute_stage(stage)
                    if stage_result:
                        executed_stages.append(stage_result)
                
                result.stages = executed_stages
                
                # 最终: 参数微调
                final_quality = self._fine_tune_quality(base_quality, target_mode)
                final_distance = self._fine_tune_distance(base_distance, target_mode)
                
                result.final_quality = final_quality
                result.final_distance = final_distance
                result.confidence = self._calculate_confidence(executed_stages)
                
                execution_time = (time.time() - start_time) * 1000
                result.total_execution_time_ms = execution_time
                
                print(f"🎯 流水线执行完成: {len(executed_stages)}阶段, {execution_time:.1f}ms")
                
        except Exception as e:
            result.stages.append(StageResult(
                name="pipeline_error",
                order=-1,
                executed=False,
                options={},
                success=False,
                error_message=str(e)
            ))
            print(f"❌ 流水线执行失败: {e}")
        
        return result
    
    def _predict_base_parameters(self, target_mode: str) -> tuple[int, float]:
        """使用贝叶斯优化预测基础参数"""
        try:
            quality, distance = self.bayesian_opt.predict_optimal(target_mode)
            return quality, distance
        except Exception as e:
            print(f"⚠️ 贝叶斯预测失败，使用默认值: {e}")
            # 默认参数
            if target_mode == "quality":
                return 95, 0.5
            elif target_mode == "size":
                return 75, 1.5
            else:  # balanced
                return 85, 1.0
    
    def _execute_stage(self, stage: Stage) -> Optional[StageResult]:
        """执行单个阶段"""
        if not stage.should_execute(self.features):
            return None
        
        start_time = time.time()
        
        try:
            # 实际执行阶段逻辑
            options = self._process_stage_options(stage)
            
            stage_result = StageResult(
                name=stage.name,
                order=stage.order,
                executed=True,
                options=options,
                execution_time_ms=(time.time() - start_time) * 1000,
                success=True
            )
            
            return stage_result
            
        except Exception as e:
            return StageResult(
                name=stage.name,
                order=stage.order,
                executed=False,
                options=stage.options,
                execution_time_ms=(time.time() - start_time) * 1000,
                success=False,
                error_message=str(e)
            )
    
    def _process_stage_options(self, stage: Stage) -> Dict[str, Any]:
        """处理阶段选项（根据图像特征动态调整）"""
        options = stage.options.copy()
        
        # 根据阶段名称和图像特征调整选项
        if stage.name == "色彩空间优化":
            if self.features.is_photo and self.features.color_range > 0.8:
                options["color_space"] = "rec2020"
            elif self.features.has_alpha:
                options["preserve_alpha"] = True
                
        elif stage.name == "预处理增强":
            options["denoise"] = self.features.noise_level > 20
            options["sharpen"] = self.features.edge_strength < 40
            options["contrast"] = self.features.contrast < 0.5
            
        elif stage.name.endswith("高级选项"):
            # 工具特定优化
            if self.tool == "jxl":
                options["modular"] = self.features.is_document or self.features.has_text
                options["progressive"] = self.features.width * self.features.height > 2000000
                options["gaborish"] = self.features.edge_strength > 60
            elif self.tool == "avif":
                options["tiles"] = self.features.width * self.features.height > 4000000
                options["speed"] = 6 if self.features.texture_complexity > 50 else 4
            elif self.tool == "webp":
                options["method"] = 6 if self.features.texture_complexity > 60 else 4
                options["auto_filter"] = True
                
        return options
    
    def _fine_tune_quality(self, base_quality: int, target_mode: str) -> int:
        """根据图像特征微调质量参数"""
        adjustment = 0
        
        # 高纹理复杂度 → 质量+5
        if self.features.texture_complexity > 80:
            adjustment += 5
        
        # 高噪声 → 质量-3（噪声图像不需要太高质量）
        if self.features.noise_level > 30:
            adjustment -= 3
        
        # 文档/截图 → 质量+3（需要保持锐度）
        if self.features.is_document or self.features.is_screenshot:
            adjustment += 3
        
        # 目标模式调整
        if target_mode == "quality":
            adjustment += 5
        elif target_mode == "size":
            adjustment -= 5
        
        final_quality = base_quality + adjustment
        
        # 限制在合理范围
        return max(70, min(100, final_quality))
    
    def _fine_tune_distance(self, base_distance: float, target_mode: str) -> float:
        """根据图像特征微调distance参数"""
        adjustment = 0.0
        
        # 高可压缩性 → distance+0.2
        if self.features.compression_score > 0.8:
            adjustment += 0.2
        
        # 低噪声+高边缘 → distance-0.1（可以更精确）
        if self.features.noise_level < 10 and self.features.edge_strength > 60:
            adjustment -= 0.1
        
        # 目标模式调整
        if target_mode == "quality":
            adjustment -= 0.3
        elif target_mode == "size":
            adjustment += 0.3
        
        final_distance = base_distance + adjustment
        
        # 限制在合理范围
        return max(0.0, min(2.0, final_distance))
    
    def _calculate_confidence(self, stages: List[StageResult]) -> float:
        """计算执行置信度"""
        if not stages:
            return 0.5
        
        success_count = sum(1 for stage in stages if stage.success)
        success_rate = success_count / len(stages)
        
        # 基于成功率和图像特征确定性计算置信度
        feature_certainty = 1.0 - (self.features.noise_level / 100.0)
        
        confidence = (success_rate * 0.7 + feature_certainty * 0.3)
        return max(0.1, min(0.99, confidence))
    
    def _build_stages(self) -> List[Stage]:
        """构建执行阶段"""
        stages = []
        
        # 阶段1: 色彩空间优化
        stages.append(Stage(
            name="色彩空间优化",
            description="根据图像特性选择最优色彩空间",
            order=1,
            required=False,
            options={"color_space": "auto"},
            condition=lambda f: f.is_photo  # 仅照片需要
        ))
        
        # 阶段2: 预处理增强
        stages.append(Stage(
            name="预处理增强",
            description="降噪、锐化等预处理",
            order=2,
            required=False,
            options={
                "denoise": False,
                "sharpen": False,
                "contrast": False
            },
            condition=lambda f: f.noise_level > 20 or f.edge_strength < 40
        ))
        
        # 阶段3: 工具特定优化
        if self.tool == "jxl":
            stages.append(Stage(
                name="JXL高级选项",
                description="Modular、Progressive、Gaborish等",
                order=3,
                required=True,
                options={
                    "modular": False,
                    "progressive": False,
                    "responsive": True,
                    "gaborish": False
                }
            ))
        elif self.tool == "avif":
            stages.append(Stage(
                name="AVIF高级选项",
                description="Tiles、Speed、Chroma等",
                order=3,
                required=True,
                options={
                    "tiles": False,
                    "speed": 6,
                    "chroma": "444"
                }
            ))
        elif self.tool == "webp":
            stages.append(Stage(
                name="WebP高级选项",
                description="Method、Filter、Preprocessing等",
                order=3,
                required=True,
                options={
                    "method": 4,
                    "auto_filter": True,
                    "preprocessing": 4
                }
            ))
        
        # 阶段4: SSIM验证
        stages.append(Stage(
            name="SSIM质量验证",
            description="转换后验证SSIM，低于阈值重试",
            order=4,
            required=True,
            options={
                "enabled": True,
                "min_ssim": 0.94
            }
        ))
        
        return stages
    
    def add_historical_observation(self, observation: Observation):
        """添加历史观测数据"""
        self.bayesian_opt.add_observation(observation)
    
    def get_pipeline_stats(self) -> Dict[str, Any]:
        """获取流水线统计信息"""
        return {
            "tool": self.tool,
            "total_stages": len(self.stages),
            "required_stages": len([s for s in self.stages if s.required]),
            "optional_stages": len([s for s in self.stages if not s.required]),
            "feature_dimensions": len(asdict(self.features)),
            "bayesian_observations": len(self.bayesian_opt.observations)
        }


class PrecisionPredictor:
    """
    🧠 精度预测器（统一接口）
    
    替代Go PrecisionPredictor的统一接口实现
    """
    
    def __init__(self, mode: PrecisionMode, tool: str, features: ImageFeaturesAdvanced):
        self.mode = mode
        self.tool = tool
        self.features = features
        
        # 初始化组件
        self.basic_optimizer = BayesianOptimizer(tool, debug=False)
        self.advanced_pipeline = AdvancedPipeline(tool, features)
        
        print(f"✅ 精度预测器初始化: {mode.value}模式, 工具={tool}")
    
    def predict(self, target_mode: str = "balanced") -> Dict[str, Any]:
        """
        执行预测（根据模式）
        
        Args:
            target_mode: 目标模式
            
        Returns:
            Dict: 预测结果
        """
        if self.mode == PrecisionMode.BASIC:
            # 基础模式：仅贝叶斯优化
            quality, distance = self.basic_optimizer.predict_optimal(target_mode)
            return {
                "mode": "basic",
                "quality": quality,
                "distance": distance,
                "tool": self.tool,
                "target_mode": target_mode
            }
        
        elif self.mode == PrecisionMode.ADVANCED:
            # 高级模式：多阶段执行
            result = self.advanced_pipeline.execute(target_mode)
            return result.to_dict()
        
        else:
            raise ValueError(f"Unknown precision mode: {self.mode}")
    
    def add_observation(self, observation: Observation):
        """添加观测数据"""
        self.basic_optimizer.add_observation(observation)
        self.advanced_pipeline.add_historical_observation(observation)
    
    def get_statistics(self) -> Dict[str, Any]:
        """获取统计信息"""
        basic_stats = self.basic_optimizer.get_statistics()
        pipeline_stats = self.advanced_pipeline.get_pipeline_stats()
        
        return {
            "mode": self.mode.value,
            "basic_optimizer": basic_stats,
            "advanced_pipeline": pipeline_stats,
            "total_observations": len(self.basic_optimizer.observations)
        }


def create_sample_features(width: int = 1920, height: int = 1080) -> ImageFeaturesAdvanced:
    """创建示例图像特征（用于测试）"""
    return ImageFeaturesAdvanced(
        width=width,
        height=height,
        has_alpha=False,
        is_animated=False,
        frame_count=1,
        edge_strength=65.0,
        texture_complexity=45.0,
        noise_level=15.0,
        detail_level=70.0,
        high_freq_energy=35.0,
        mid_freq_energy=50.0,
        low_freq_energy=60.0,
        overall_quality=85.0,
        compression_score=0.75,
        color_space="sRGB",
        color_range=0.8,
        saturation=0.6,
        brightness=0.5,
        contrast=0.7,
        is_photo=True,
        is_document=False,
        is_screenshot=False,
        has_text=False
    )


# 全局预测器实例缓存
_predictor_cache: Dict[str, PrecisionPredictor] = {}
_cache_lock = threading.Lock()

def get_precision_predictor(mode: PrecisionMode, tool: str, 
                          features: ImageFeaturesAdvanced) -> PrecisionPredictor:
    """获取精度预测器实例（带缓存）"""
    cache_key = f"{mode.value}_{tool}_{hash(str(features.to_dict()))}"
    
    with _cache_lock:
        if cache_key not in _predictor_cache:
            _predictor_cache[cache_key] = PrecisionPredictor(mode, tool, features)
        return _predictor_cache[cache_key]
