"""
增强视频处理模块
EX-013实现: 从Go废弃代码价值提取

模块组成:
- enhanced_processor.py: 核心视频处理器
- complexity_analyzer.py: 视频复杂度分析
- vmaf_validator.py: VMAF质量验证
"""

from .enhanced_processor import (
    EnhancedVideoProcessor,
    VideoPredictRequest,
    VideoPredictResponse,
    VideoParams,
    VideoType,
    VideoComplexity,
    VideoRequestOptions,
    predict_video_params
)

from .complexity_analyzer import (
    VideoComplexityAnalyzer,
    analyze_video_complexity
)

from .vmaf_validator import (
    VMAFValidator,
    VMAFRequest,
    VMAFResponse,
    VMAFDetails,
    validate_video_quality
)

__all__ = [
    # 核心处理器
    'EnhancedVideoProcessor',
    'predict_video_params',
    
    # 数据结构
    'VideoPredictRequest',
    'VideoPredictResponse', 
    'VideoParams',
    'VideoType',
    'VideoComplexity',
    'VideoRequestOptions',
    
    # 复杂度分析
    'VideoComplexityAnalyzer',
    'analyze_video_complexity',
    
    # VMAF验证
    'VMAFValidator',
    'VMAFRequest',
    'VMAFResponse', 
    'VMAFDetails',
    'validate_video_quality',
]
