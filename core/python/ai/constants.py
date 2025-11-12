"""
🎯 PIXLY v3.1 统一常量系统

基于废弃Go constants.go重新设计的Python实现：
- 参数范围统一管理
- 格式支持列表  
- AI阈值配置
- 验证消息标准化
- 性能常量优化

确保Python/Rust使用相同的参数范围和阈值
"""

from typing import Dict, List, Tuple, Any
from dataclasses import dataclass
from enum import Enum


# ============================================================================
# 参数范围常量
# ============================================================================

@dataclass(frozen=True)
class QualityRange:
    """质量参数范围"""
    min: int = 1
    max: int = 100
    default: int = 85
    
    def validate(self, value: int) -> bool:
        """验证质量值是否在范围内"""
        return self.min <= value <= self.max


@dataclass(frozen=True)
class SpeedRange:
    """速度/努力参数范围"""
    min: int = 0
    max: int = 10
    default: int = 6
    
    def validate(self, value: int) -> bool:
        """验证速度值是否在范围内"""
        return self.min <= value <= self.max


@dataclass(frozen=True)
class DistanceRange:
    """JXL Distance参数范围"""
    min: float = 0.0
    max: float = 15.0
    default: float = 1.0
    
    def validate(self, value: float) -> bool:
        """验证distance值是否在范围内"""
        return self.min <= value <= self.max


# 全局参数范围实例
QUALITY_RANGE = QualityRange()
SPEED_RANGE = SpeedRange()
DISTANCE_RANGE = DistanceRange()

# 工具特定参数范围
AVIF_QUANTIZER_RANGE = (0, 63)      # AVIF Quantizer范围
WEBP_METHOD_RANGE = (0, 6)          # WebP Method范围  
FFMPEG_CRF_RANGE = (0, 51)          # FFmpeg CRF范围

# 图像尺寸限制
IMAGE_DIM_MIN = 1
IMAGE_DIM_MAX = 65535
IMAGE_MAX_PIXELS = 1_000_000_000     # 10亿像素限制

# 文件大小限制
FILE_SIZE_MAX = 100 * 1024 * 1024    # 100MB


# ============================================================================
# AI相关常量
# ============================================================================

@dataclass(frozen=True)
class AIThresholds:
    """AI阈值配置"""
    confidence_reject: float = 0.5     # 低于此值拒绝预测
    confidence_warn: float = 0.7       # 低于此值发出警告
    confidence_accept: float = 0.8     # 高于此值完全接受
    timeout_seconds: int = 30          # AI服务超时
    max_retries: int = 3               # 最大重试次数
    batch_size_max: int = 10           # 批量预测最大数量

AI_THRESHOLDS = AIThresholds()


# ============================================================================
# 支持的格式列表
# ============================================================================

class FormatCategory(Enum):
    """格式类别"""
    MODERN = "modern"        # 现代格式 (JXL, AVIF, WebP)
    LEGACY = "legacy"        # 传统格式 (JPEG, PNG, GIF)
    RAW = "raw"             # RAW格式 (TIFF, BMP)
    VIDEO = "video"         # 视频格式 (MP4, WebM)


# 输入格式支持
SUPPORTED_INPUT_FORMATS = {
    # 现代格式
    "jxl", "avif", "webp", "heic", "heif",
    # 传统格式  
    "jpg", "jpeg", "png", "gif", "bmp",
    # RAW格式
    "tiff", "tif", "psd", "raw", "dng"
}

# 输出格式支持  
SUPPORTED_OUTPUT_FORMATS = {
    "jxl", "avif", "webp", "heic",
    "jpg", "jpeg", "png", "gif"
}

# 无损格式
LOSSLESS_FORMATS = {
    "png", "webp", "avif", "jxl", "tiff"
}

# 现代格式（优先推荐）
MODERN_FORMATS = {
    "jxl", "avif", "webp"
}

# 格式优先级（用于自动选择）
FORMAT_PRIORITY = [
    "jxl",      # 最佳压缩效率
    "avif",     # 优秀的兼容性
    "webp",     # 广泛支持  
    "heic",     # Apple生态
    "png",      # 无损后备
    "jpeg"      # 通用后备
]


# ============================================================================
# 工具名称常量
# ============================================================================

class ConversionTool(Enum):
    """转换工具枚举"""
    CJXL = "cjxl"
    DJXL = "djxl" 
    AVIFENC = "avifenc"
    AVIFDEC = "avifdec"
    CWEBP = "cwebp"
    DWEBP = "dwebp"
    FFMPEG = "ffmpeg"
    MAGICK = "magick"

# 工具到格式的映射
TOOL_FORMAT_MAP = {
    ConversionTool.CJXL: "jxl",
    ConversionTool.AVIFENC: "avif", 
    ConversionTool.CWEBP: "webp",
    ConversionTool.FFMPEG: ["jpg", "png", "webm", "mp4"],
    ConversionTool.MAGICK: ["jpg", "png", "bmp", "tiff"]
}

# 格式到推荐工具的映射
FORMAT_TOOL_MAP = {
    "jxl": ConversionTool.CJXL,
    "avif": ConversionTool.AVIFENC,
    "webp": ConversionTool.CWEBP,
    "jpg": ConversionTool.FFMPEG,
    "jpeg": ConversionTool.FFMPEG,
    "png": ConversionTool.MAGICK
}


# ============================================================================
# 验证消息常量
# ============================================================================

class ValidationMessages:
    """验证错误消息"""
    # 参数验证
    QUALITY_OUT_OF_RANGE = "Quality参数超出范围 (1-100)"
    SPEED_OUT_OF_RANGE = "Speed参数超出范围 (0-10)" 
    DISTANCE_OUT_OF_RANGE = "Distance参数超出范围 (0.0-15.0)"
    
    # 文件验证
    FILE_NOT_FOUND = "文件不存在"
    FILE_TOO_LARGE = "文件大小超限 (最大100MB)"
    IMAGE_TOO_LARGE = "图像尺寸超限"
    IMAGE_TOO_MANY_PIXELS = "图像像素数超限 (最大10亿像素)"
    
    # 格式验证
    FORMAT_UNSUPPORTED = "格式不支持"
    FORMAT_INPUT_UNSUPPORTED = "输入格式不支持"
    FORMAT_OUTPUT_UNSUPPORTED = "输出格式不支持"
    
    # AI验证
    AI_CONFIDENCE_LOW = "AI置信度过低"
    AI_TIMEOUT = "AI预测超时"
    AI_MODEL_UNAVAILABLE = "AI模型不可用"
    
    # 工具验证
    TOOL_NOT_FOUND = "转换工具不存在"
    TOOL_EXECUTION_FAILED = "工具执行失败"


# ============================================================================
# 性能相关常量
# ============================================================================

@dataclass(frozen=True)
class PerformanceConfig:
    """性能配置"""
    default_concurrency: int = 4      # 默认并发数
    max_concurrency: int = 16         # 最大并发数
    processing_timeout: int = 300     # 处理超时（秒）
    memory_limit_mb: int = 2048       # 内存限制（MB）
    temp_dir_cleanup: bool = True     # 自动清理临时目录
    enable_simd: bool = True          # 启用SIMD优化
    enable_gpu: bool = False          # 启用GPU加速

PERFORMANCE_CONFIG = PerformanceConfig()


# ============================================================================
# 质量配置预设
# ============================================================================

@dataclass(frozen=True)
class QualityPreset:
    """质量预设"""
    quality: int
    distance: float
    effort: int
    description: str

# 质量预设定义
QUALITY_PRESETS = {
    "maximum": QualityPreset(
        quality=100, distance=0.0, effort=9,
        description="最高质量，接近无损"
    ),
    "high": QualityPreset(
        quality=95, distance=0.5, effort=8,
        description="高质量，适合专业用途"
    ),
    "balanced": QualityPreset(
        quality=85, distance=1.0, effort=6,
        description="平衡质量与体积"
    ),
    "web": QualityPreset(
        quality=80, distance=1.5, effort=4,
        description="Web优化，快速加载"
    ),
    "preview": QualityPreset(
        quality=70, distance=2.0, effort=2,
        description="预览质量，快速处理"
    )
}


# ============================================================================
# 验证函数
# ============================================================================

class ValidationError(Exception):
    """验证错误异常"""
    def __init__(self, message: str, error_code: str = "VALIDATION_ERROR"):
        super().__init__(message)
        self.error_code = error_code


def validate_quality(value: int) -> None:
    """验证质量参数"""
    if not QUALITY_RANGE.validate(value):
        raise ValidationError(
            f"{ValidationMessages.QUALITY_OUT_OF_RANGE}: {value}",
            "QUALITY_RANGE_ERROR"
        )


def validate_speed(value: int) -> None:
    """验证速度参数"""
    if not SPEED_RANGE.validate(value):
        raise ValidationError(
            f"{ValidationMessages.SPEED_OUT_OF_RANGE}: {value}",
            "SPEED_RANGE_ERROR"
        )


def validate_distance(value: float) -> None:
    """验证distance参数"""
    if not DISTANCE_RANGE.validate(value):
        raise ValidationError(
            f"{ValidationMessages.DISTANCE_OUT_OF_RANGE}: {value}",
            "DISTANCE_RANGE_ERROR"
        )


def validate_image_dimensions(width: int, height: int) -> None:
    """验证图像尺寸"""
    if (width < IMAGE_DIM_MIN or width > IMAGE_DIM_MAX or
        height < IMAGE_DIM_MIN or height > IMAGE_DIM_MAX):
        raise ValidationError(
            f"{ValidationMessages.IMAGE_TOO_LARGE}: {width}x{height}",
            "DIMENSION_ERROR"
        )
    
    total_pixels = width * height
    if total_pixels > IMAGE_MAX_PIXELS:
        raise ValidationError(
            f"{ValidationMessages.IMAGE_TOO_MANY_PIXELS}: {total_pixels:,}",
            "PIXEL_COUNT_ERROR"
        )


def validate_file_size(size_bytes: int) -> None:
    """验证文件大小"""
    if size_bytes > FILE_SIZE_MAX:
        size_mb = size_bytes / (1024 * 1024)
        max_mb = FILE_SIZE_MAX / (1024 * 1024)
        raise ValidationError(
            f"{ValidationMessages.FILE_TOO_LARGE}: {size_mb:.1f}MB (最大: {max_mb}MB)",
            "FILE_SIZE_ERROR"
        )


def is_input_format_supported(format_name: str) -> bool:
    """检查输入格式是否支持"""
    return format_name.lower() in SUPPORTED_INPUT_FORMATS


def is_output_format_supported(format_name: str) -> bool:
    """检查输出格式是否支持"""
    return format_name.lower() in SUPPORTED_OUTPUT_FORMATS


def is_lossless_format(format_name: str) -> bool:
    """检查格式是否支持无损"""
    return format_name.lower() in LOSSLESS_FORMATS


def is_modern_format(format_name: str) -> bool:
    """检查是否为现代格式"""
    return format_name.lower() in MODERN_FORMATS


def get_recommended_tool(format_name: str) -> ConversionTool:
    """获取格式推荐的转换工具"""
    return FORMAT_TOOL_MAP.get(format_name.lower(), ConversionTool.MAGICK)


def get_quality_preset(preset_name: str) -> QualityPreset:
    """获取质量预设"""
    if preset_name not in QUALITY_PRESETS:
        return QUALITY_PRESETS["balanced"]  # 默认平衡模式
    return QUALITY_PRESETS[preset_name]


def validate_all_params(quality: int, speed: int, distance: float,
                       width: int, height: int, file_size: int) -> List[str]:
    """验证所有参数，返回错误列表"""
    errors = []
    
    try:
        validate_quality(quality)
    except ValidationError as e:
        errors.append(str(e))
    
    try:
        validate_speed(speed)
    except ValidationError as e:
        errors.append(str(e))
    
    try:
        validate_distance(distance)
    except ValidationError as e:
        errors.append(str(e))
    
    try:
        validate_image_dimensions(width, height)
    except ValidationError as e:
        errors.append(str(e))
    
    try:
        validate_file_size(file_size)
    except ValidationError as e:
        errors.append(str(e))
    
    return errors


# ============================================================================
# 常量摘要信息
# ============================================================================

def get_constants_summary() -> Dict[str, Any]:
    """获取常量系统摘要"""
    return {
        "version": "3.1.0",
        "parameter_ranges": {
            "quality": f"{QUALITY_RANGE.min}-{QUALITY_RANGE.max}",
            "speed": f"{SPEED_RANGE.min}-{SPEED_RANGE.max}",
            "distance": f"{DISTANCE_RANGE.min}-{DISTANCE_RANGE.max}"
        },
        "supported_formats": {
            "input": len(SUPPORTED_INPUT_FORMATS),
            "output": len(SUPPORTED_OUTPUT_FORMATS),
            "lossless": len(LOSSLESS_FORMATS),
            "modern": len(MODERN_FORMATS)
        },
        "tools_available": len(ConversionTool),
        "quality_presets": len(QUALITY_PRESETS),
        "ai_thresholds": {
            "confidence_reject": AI_THRESHOLDS.confidence_reject,
            "timeout_seconds": AI_THRESHOLDS.timeout_seconds
        },
        "performance_limits": {
            "max_concurrency": PERFORMANCE_CONFIG.max_concurrency,
            "max_file_size_mb": FILE_SIZE_MAX // (1024 * 1024),
            "max_pixels": f"{IMAGE_MAX_PIXELS:,}"
        }
    }
