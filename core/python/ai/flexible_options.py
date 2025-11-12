"""
🎯 PIXLY v3.1 自由化开关选项系统

替代废弃Go双精度模式的纯自由化配置：
- 完全自由的功能开关组合
- 用户自定义处理流程  
- 灵活的质量曲线配置
- 智能化vs手动化平衡控制
- 性能vs质量权衡自定义

零约束的配置系统，用户完全控制所有处理参数
"""

import json
from typing import Dict, List, Optional, Any, Union, Callable
from dataclasses import dataclass, asdict, field
from enum import Enum
from pathlib import Path

from .constants import QUALITY_PRESETS, QualityPreset, validate_quality


class ProcessingMode(Enum):
    """处理模式"""
    MANUAL = "manual"              # 完全手动控制
    ASSISTED = "assisted"          # AI辅助建议
    AUTOMATIC = "automatic"        # 全自动智能
    CUSTOM = "custom"             # 自定义流程


class QualityCurve(Enum):
    """质量曲线类型"""
    LINEAR = "linear"             # 线性质量递减
    EXPONENTIAL = "exponential"   # 指数质量递减
    LOGARITHMIC = "logarithmic"   # 对数质量递减
    CUSTOM = "custom"            # 自定义曲线


@dataclass
class AdvancedFeatureToggles:
    """高级功能开关"""
    # AI智能化开关
    enable_ai_prediction: bool = True           # AI预测
    enable_bayesian_optimization: bool = True   # 贝叶斯优化
    enable_advanced_pipeline: bool = True       # 多阶段流水线
    enable_parameter_tuning: bool = True        # 参数微调
    enable_format_recommendation: bool = True   # 格式推荐
    
    # 特征分析开关
    enable_advanced_features: bool = True       # 高级特征提取
    enable_content_detection: bool = True       # 内容类型识别
    enable_quality_assessment: bool = True      # 质量评估
    enable_compression_scoring: bool = True     # 压缩性评分
    
    # 性能优化开关
    enable_rust_acceleration: bool = True       # Rust加速
    enable_simd_optimization: bool = True       # SIMD优化
    enable_gpu_acceleration: bool = False       # GPU加速
    enable_parallel_processing: bool = True     # 并行处理
    
    # 质量控制开关
    enable_quality_validation: bool = True      # 质量验证
    enable_ssim_checking: bool = True           # SSIM检查
    enable_adaptive_quality: bool = True        # 自适应质量
    enable_lossless_fallback: bool = False      # 无损回退
    
    # 用户体验开关
    enable_progress_tracking: bool = True       # 进度跟踪
    enable_preview_generation: bool = False     # 预览生成
    enable_batch_optimization: bool = True      # 批量优化
    enable_auto_backup: bool = False           # 自动备份


@dataclass
class CustomQualityCurve:
    """自定义质量曲线"""
    curve_type: QualityCurve = QualityCurve.LINEAR
    control_points: List[Tuple[float, float]] = field(default_factory=list)  # (输入, 输出)
    smoothness: float = 0.5                    # 曲线平滑度
    min_quality: int = 70                      # 最低质量
    max_quality: int = 100                     # 最高质量
    
    def evaluate(self, input_value: float) -> float:
        """评估曲线在指定点的输出值"""
        if self.curve_type == QualityCurve.LINEAR:
            return min(self.max_quality, max(self.min_quality, 
                      self.min_quality + input_value * (self.max_quality - self.min_quality)))
        
        elif self.curve_type == QualityCurve.EXPONENTIAL:
            import math
            exp_value = math.exp(input_value * 2) - 1
            normalized = exp_value / (math.exp(2) - 1)
            return self.min_quality + normalized * (self.max_quality - self.min_quality)
        
        elif self.curve_type == QualityCurve.LOGARITHMIC:
            import math
            log_value = math.log(1 + input_value * 9) / math.log(10)
            return self.min_quality + log_value * (self.max_quality - self.min_quality)
        
        elif self.curve_type == QualityCurve.CUSTOM and self.control_points:
            return self._interpolate_control_points(input_value)
        
        return self.min_quality + input_value * (self.max_quality - self.min_quality)
    
    def _interpolate_control_points(self, x: float) -> float:
        """在控制点之间插值"""
        if not self.control_points:
            return (self.min_quality + self.max_quality) / 2
        
        # 简单线性插值
        sorted_points = sorted(self.control_points, key=lambda p: p[0])
        
        if x <= sorted_points[0][0]:
            return sorted_points[0][1]
        
        if x >= sorted_points[-1][0]:
            return sorted_points[-1][1]
        
        # 找到x所在的区间
        for i in range(len(sorted_points) - 1):
            x1, y1 = sorted_points[i]
            x2, y2 = sorted_points[i + 1]
            
            if x1 <= x <= x2:
                # 线性插值
                t = (x - x1) / (x2 - x1)
                return y1 + t * (y2 - y1)
        
        return (self.min_quality + self.max_quality) / 2


@dataclass
class ProcessingPipeline:
    """自定义处理流水线"""
    steps: List[str] = field(default_factory=list)  # 处理步骤顺序
    step_configs: Dict[str, Dict[str, Any]] = field(default_factory=dict)  # 步骤配置
    conditional_execution: Dict[str, str] = field(default_factory=dict)  # 条件执行规则
    parallel_steps: List[List[str]] = field(default_factory=list)  # 可并行执行的步骤组
    
    def add_step(self, step_name: str, config: Dict[str, Any] = None, 
                 condition: str = None):
        """添加处理步骤"""
        if step_name not in self.steps:
            self.steps.append(step_name)
        
        if config:
            self.step_configs[step_name] = config
        
        if condition:
            self.conditional_execution[step_name] = condition
    
    def remove_step(self, step_name: str):
        """移除处理步骤"""
        if step_name in self.steps:
            self.steps.remove(step_name)
            self.step_configs.pop(step_name, None)
            self.conditional_execution.pop(step_name, None)
    
    def set_parallel_group(self, step_names: List[str]):
        """设置并行执行组"""
        self.parallel_steps.append(step_names)


@dataclass
class FlexibleOptions:
    """
    🎯 自由化选项配置系统
    
    完全自由的配置系统，用户可自定义所有处理参数和流程
    """
    # 基础配置
    processing_mode: ProcessingMode = ProcessingMode.ASSISTED
    quality_preset: str = "balanced"
    custom_quality_curve: Optional[CustomQualityCurve] = None
    
    # 功能开关
    feature_toggles: AdvancedFeatureToggles = field(default_factory=AdvancedFeatureToggles)
    
    # 自定义流水线
    custom_pipeline: Optional[ProcessingPipeline] = None
    
    # 高级参数配置
    advanced_params: Dict[str, Any] = field(default_factory=dict)
    
    # 用户偏好
    user_preferences: Dict[str, Any] = field(default_factory=dict)
    
    def __post_init__(self):
        """初始化后处理"""
        if self.custom_quality_curve is None:
            self.custom_quality_curve = CustomQualityCurve()
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典"""
        result = asdict(self)
        result['processing_mode'] = self.processing_mode.value
        return result
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'FlexibleOptions':
        """从字典创建"""
        # 处理枚举类型
        if 'processing_mode' in data:
            data['processing_mode'] = ProcessingMode(data['processing_mode'])
        
        # 处理嵌套对象
        if 'feature_toggles' in data and isinstance(data['feature_toggles'], dict):
            data['feature_toggles'] = AdvancedFeatureToggles(**data['feature_toggles'])
        
        if 'custom_quality_curve' in data and isinstance(data['custom_quality_curve'], dict):
            curve_data = data['custom_quality_curve']
            if 'curve_type' in curve_data:
                curve_data['curve_type'] = QualityCurve(curve_data['curve_type'])
            data['custom_quality_curve'] = CustomQualityCurve(**curve_data)
        
        if 'custom_pipeline' in data and isinstance(data['custom_pipeline'], dict):
            data['custom_pipeline'] = ProcessingPipeline(**data['custom_pipeline'])
        
        return cls(**data)
    
    def enable_feature(self, feature_name: str, enabled: bool = True):
        """启用/禁用功能"""
        if hasattr(self.feature_toggles, feature_name):
            setattr(self.feature_toggles, feature_name, enabled)
    
    def set_quality_preset(self, preset_name: str):
        """设置质量预设"""
        if preset_name in QUALITY_PRESETS:
            self.quality_preset = preset_name
    
    def create_custom_pipeline(self) -> ProcessingPipeline:
        """创建自定义流水线"""
        if self.custom_pipeline is None:
            self.custom_pipeline = ProcessingPipeline()
        return self.custom_pipeline
    
    def set_advanced_param(self, key: str, value: Any):
        """设置高级参数"""
        self.advanced_params[key] = value
    
    def set_user_preference(self, key: str, value: Any):
        """设置用户偏好"""
        self.user_preferences[key] = value
    
    def get_effective_quality(self, base_quality: int) -> int:
        """获取有效质量值（应用自定义曲线）"""
        if self.custom_quality_curve and self.custom_quality_curve.curve_type != QualityCurve.LINEAR:
            # 将quality值归一化到0-1
            normalized = (base_quality - 1) / 99.0
            # 应用自定义曲线
            curved_value = self.custom_quality_curve.evaluate(normalized)
            return int(curved_value)
        
        return base_quality
    
    def is_feature_enabled(self, feature_name: str) -> bool:
        """检查功能是否启用"""
        return getattr(self.feature_toggles, feature_name, False)
    
    def get_processing_strategy(self) -> Dict[str, Any]:
        """获取处理策略配置"""
        return {
            "mode": self.processing_mode.value,
            "ai_enabled": self.feature_toggles.enable_ai_prediction,
            "rust_acceleration": self.feature_toggles.enable_rust_acceleration,
            "parallel_processing": self.feature_toggles.enable_parallel_processing,
            "quality_validation": self.feature_toggles.enable_quality_validation,
            "custom_pipeline": self.custom_pipeline is not None,
            "advanced_features": self.feature_toggles.enable_advanced_features
        }


class FlexibleOptionsManager:
    """
    🎯 自由化选项管理器
    
    管理和持久化用户的自由化配置
    """
    
    def __init__(self, config_dir: str = "data/user_configs"):
        self.config_dir = Path(config_dir)
        self.config_dir.mkdir(parents=True, exist_ok=True)
        self.current_options: Optional[FlexibleOptions] = None
        
        # 预设配置
        self.presets = self._create_presets()
    
    def _create_presets(self) -> Dict[str, FlexibleOptions]:
        """创建预设配置"""
        presets = {}
        
        # 完全手动模式
        manual = FlexibleOptions(
            processing_mode=ProcessingMode.MANUAL,
            quality_preset="high"
        )
        manual.feature_toggles.enable_ai_prediction = False
        manual.feature_toggles.enable_bayesian_optimization = False
        manual.feature_toggles.enable_advanced_pipeline = False
        presets["manual"] = manual
        
        # AI辅助模式
        assisted = FlexibleOptions(
            processing_mode=ProcessingMode.ASSISTED,
            quality_preset="balanced"
        )
        presets["assisted"] = assisted
        
        # 全自动智能模式
        automatic = FlexibleOptions(
            processing_mode=ProcessingMode.AUTOMATIC,
            quality_preset="balanced"
        )
        automatic.feature_toggles.enable_adaptive_quality = True
        presets["automatic"] = automatic
        
        # 性能优先模式
        performance = FlexibleOptions(
            processing_mode=ProcessingMode.ASSISTED,
            quality_preset="web"
        )
        performance.feature_toggles.enable_rust_acceleration = True
        performance.feature_toggles.enable_simd_optimization = True
        performance.feature_toggles.enable_parallel_processing = True
        performance.feature_toggles.enable_preview_generation = False
        presets["performance"] = performance
        
        # 质量优先模式
        quality = FlexibleOptions(
            processing_mode=ProcessingMode.AUTOMATIC,
            quality_preset="maximum"
        )
        quality.feature_toggles.enable_quality_validation = True
        quality.feature_toggles.enable_ssim_checking = True
        quality.feature_toggles.enable_lossless_fallback = True
        presets["quality"] = quality
        
        return presets
    
    def load_options(self, profile_name: str = "default") -> FlexibleOptions:
        """加载配置"""
        config_file = self.config_dir / f"{profile_name}.json"
        
        if config_file.exists():
            try:
                with open(config_file, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                    options = FlexibleOptions.from_dict(data)
                    self.current_options = options
                    return options
            except Exception as e:
                print(f"⚠️ 配置加载失败: {e}")
        
        # 使用默认辅助模式
        default_options = self.presets["assisted"]
        self.current_options = default_options
        return default_options
    
    def save_options(self, options: FlexibleOptions, profile_name: str = "default"):
        """保存配置"""
        config_file = self.config_dir / f"{profile_name}.json"
        
        try:
            with open(config_file, 'w', encoding='utf-8') as f:
                json.dump(options.to_dict(), f, indent=2, ensure_ascii=False)
            print(f"✅ 配置已保存: {profile_name}")
        except Exception as e:
            print(f"❌ 配置保存失败: {e}")
    
    def get_preset(self, preset_name: str) -> Optional[FlexibleOptions]:
        """获取预设配置"""
        return self.presets.get(preset_name)
    
    def list_presets(self) -> List[str]:
        """列出所有预设"""
        return list(self.presets.keys())
    
    def create_custom_preset(self, name: str, options: FlexibleOptions):
        """创建自定义预设"""
        self.presets[name] = options
        self.save_options(options, f"preset_{name}")
    
    def apply_quick_toggle(self, toggle_name: str, enabled: bool) -> bool:
        """快速切换功能"""
        if self.current_options is None:
            self.current_options = self.load_options()
        
        try:
            self.current_options.enable_feature(toggle_name, enabled)
            return True
        except AttributeError:
            return False
    
    def get_current_summary(self) -> Dict[str, Any]:
        """获取当前配置摘要"""
        if self.current_options is None:
            self.current_options = self.load_options()
        
        return {
            "processing_mode": self.current_options.processing_mode.value,
            "quality_preset": self.current_options.quality_preset,
            "ai_enabled": self.current_options.feature_toggles.enable_ai_prediction,
            "rust_acceleration": self.current_options.feature_toggles.enable_rust_acceleration,
            "advanced_features": self.current_options.feature_toggles.enable_advanced_features,
            "custom_pipeline": self.current_options.custom_pipeline is not None,
            "total_toggles": len(asdict(self.current_options.feature_toggles)),
            "enabled_toggles": sum(1 for v in asdict(self.current_options.feature_toggles).values() if v),
            "advanced_params": len(self.current_options.advanced_params),
            "user_preferences": len(self.current_options.user_preferences)
        }


# 全局选项管理器实例
_global_options_manager: Optional[FlexibleOptionsManager] = None

def get_options_manager() -> FlexibleOptionsManager:
    """获取全局选项管理器实例（单例模式）"""
    global _global_options_manager
    
    if _global_options_manager is None:
        _global_options_manager = FlexibleOptionsManager()
    
    return _global_options_manager
