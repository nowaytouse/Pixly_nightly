"""
增强视频处理器 - 第一阶段：核心数据结构
基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/video_handlers.go 重新实现

功能:
- 视频类型自动识别
- 复杂度评分系统
- VMAF质量验证集成
- H.264/H.265/AV1编码器智能选择
- 高级特征分析和Transformer使用决策

EX-013实现: 从Go废弃代码价值提取 - 第一阶段
"""

import json
import subprocess
import tempfile
import os
import time
from pathlib import Path
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, field, asdict
from datetime import datetime
import logging

# 核心数据结构

@dataclass
class VideoRequestOptions:
    """视频请求选项"""
    use_advanced_ai: bool = True        # 高级特征分析
    enable_transformer: bool = False    # 强制Transformer
    enable_vmaf: bool = False          # VMAF验证
    allow_hevc: bool = True            # 允许H.265
    prefer_speed: bool = True          # 速度优先
    target_encoder: Optional[str] = None  # 目标Encoder

@dataclass
class VideoPredictRequest:
    """视频预测请求"""
    video_path: str                     # 视频路径
    optimize_mode: str = "balanced"     # 优化模式：size/balanced/quality
    options: Optional[VideoRequestOptions] = None

    def __post_init__(self):
        if self.options is None:
            self.options = VideoRequestOptions()

@dataclass
class VideoParams:
    """视频编码参数"""
    encoder: str                        # h264, h265, av1, vp9
    crf: int                           # 恒定速率因子
    preset: str                        # 编码preset
    fps: Optional[int] = None          # 帧率（None保持原始）
    scale: Optional[str] = None        # 分辨率缩放
    channel: str = "lightgbm"         # 预测通道
    target_bitrate: int = 0           # 目标码率(kbps)
    max_bitrate: int = 0             # 最大码率(kbps)
    two_pass: bool = False           # 两遍编码
    confidence: float = 0.0          # 预测置信度

    def to_dict(self) -> Dict[str, Any]:
        """转换为字典"""
        return asdict(self)

@dataclass
class VideoType:
    """视频类型识别结果"""
    container: str = ""               # 容器格式
    video_codec: str = ""            # 视频编解码器
    audio_codec: str = ""            # 音频编解码器
    duration: float = 0.0            # 时长（秒）
    bitrate: int = 0                 # 总码率(kbps)
    fps: float = 0.0                 # 帧率
    width: int = 0                   # 宽度
    height: int = 0                  # 高度
    has_audio: bool = False          # 是否有音频
    pixel_format: str = ""           # 像素格式

@dataclass
class VideoComplexity:
    """视频复杂度分析"""
    motion_score: float = 0.0        # 运动复杂度
    texture_score: float = 0.0       # 纹理复杂度
    scene_changes: int = 0           # 场景切换次数
    detail_level: float = 0.0        # 细节等级
    encoding_difficulty: str = "medium"  # 编码难度：easy/medium/hard
    recommended_crf_range: Tuple[int, int] = (23, 28)  # 推荐CRF范围

@dataclass
class VideoPredictResponse:
    """视频预测响应"""
    success: bool = True
    params: Optional[VideoParams] = None
    features: Dict[str, Any] = field(default_factory=dict)
    video_type: Optional[VideoType] = None
    advanced_features: Dict[str, Any] = field(default_factory=dict)
    complexity_score: int = 50
    use_transformer: bool = False
    mode: str = "balanced"
    confidence: float = 0.0
    error: str = ""
    time_ms: int = 0

    def to_dict(self) -> Dict[str, Any]:
        """转换为字典"""
        data = asdict(self)
        if self.params:
            data['params'] = self.params.to_dict()
        return data


class EnhancedVideoProcessor:
    """
    增强视频处理器
    第一阶段：基础框架和数据结构
    """
    
    def __init__(self, debug: bool = False):
        """
        初始化视频处理器
        
        Args:
            debug: 调试模式
        """
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        if debug:
            logging.basicConfig(level=logging.DEBUG)
    
    def predict_video_params(self, request: VideoPredictRequest) -> VideoPredictResponse:
        """
        预测视频编码参数
        
        Args:
            request: 视频预测请求
            
        Returns:
            VideoPredictResponse: 预测响应
        """
        start_time = time.time()
        
        try:
            # 验证输入
            if not request.video_path or not Path(request.video_path).exists():
                return VideoPredictResponse(
                    success=False,
                    error=f"视频文件不存在: {request.video_path}",
                    time_ms=int((time.time() - start_time) * 1000)
                )
            
            if self.debug:
                self.logger.debug(f"开始处理视频: {request.video_path}")
                self.logger.debug(f"优化模式: {request.optimize_mode}")
            
            # 第一阶段：基础信息获取
            video_type = self._analyze_video_type(request.video_path)
            if not video_type:
                return VideoPredictResponse(
                    success=False,
                    error="无法分析视频类型信息",
                    time_ms=int((time.time() - start_time) * 1000)
                )
            
            # 基础参数生成（第一阶段简化版）
            params = self._generate_basic_params(video_type, request.optimize_mode)
            
            # 构建响应
            response = VideoPredictResponse(
                success=True,
                params=params,
                video_type=video_type,
                mode=request.optimize_mode,
                confidence=0.8,  # 基础置信度
                time_ms=int((time.time() - start_time) * 1000)
            )
            
            if self.debug:
                self.logger.debug(f"处理完成，耗时: {response.time_ms}ms")
                self.logger.debug(f"推荐编码器: {params.encoder}")
                self.logger.debug(f"推荐CRF: {params.crf}")
            
            return response
            
        except Exception as e:
            self.logger.error(f"视频处理失败: {e}")
            return VideoPredictResponse(
                success=False,
                error=str(e),
                time_ms=int((time.time() - start_time) * 1000)
            )
    
    def _analyze_video_type(self, video_path: str) -> Optional[VideoType]:
        """
        分析视频类型信息
        第一阶段：使用ffprobe获取基础信息
        """
        try:
            cmd = [
                'ffprobe',
                '-v', 'quiet',
                '-print_format', 'json',
                '-show_format',
                '-show_streams',
                video_path
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
            if result.returncode != 0:
                self.logger.error(f"ffprobe失败: {result.stderr}")
                return None
            
            data = json.loads(result.stdout)
            
            # 解析视频流信息
            video_stream = None
            audio_stream = None
            
            for stream in data.get('streams', []):
                if stream.get('codec_type') == 'video' and video_stream is None:
                    video_stream = stream
                elif stream.get('codec_type') == 'audio' and audio_stream is None:
                    audio_stream = stream
            
            if not video_stream:
                self.logger.error("未找到视频流")
                return None
            
            # 构建VideoType对象
            format_info = data.get('format', {})
            
            video_type = VideoType(
                container=format_info.get('format_name', '').split(',')[0],
                video_codec=video_stream.get('codec_name', ''),
                audio_codec=audio_stream.get('codec_name', '') if audio_stream else '',
                duration=float(format_info.get('duration', 0)),
                bitrate=int(format_info.get('bit_rate', 0)) // 1000,  # 转换为kbps
                fps=self._parse_fps(video_stream.get('r_frame_rate', '0/1')),
                width=int(video_stream.get('width', 0)),
                height=int(video_stream.get('height', 0)),
                has_audio=audio_stream is not None,
                pixel_format=video_stream.get('pix_fmt', '')
            )
            
            if self.debug:
                self.logger.debug(f"视频信息: {video_type.width}x{video_type.height}, "
                                f"{video_type.video_codec}, {video_type.fps}fps")
            
            return video_type
            
        except subprocess.TimeoutExpired:
            self.logger.error("ffprobe超时")
            return None
        except Exception as e:
            self.logger.error(f"视频分析失败: {e}")
            return None
    
    def _parse_fps(self, fps_str: str) -> float:
        """解析帧率字符串"""
        try:
            if '/' in fps_str:
                num, den = fps_str.split('/')
                return float(num) / float(den) if float(den) != 0 else 0.0
            return float(fps_str)
        except:
            return 0.0
    
    def _generate_basic_params(self, video_type: VideoType, optimize_mode: str) -> VideoParams:
        """
        生成基础编码参数
        第一阶段：简化的参数生成逻辑
        """
        # 基础编码器选择逻辑
        encoder = self._select_encoder(video_type)
        
        # 基础CRF选择
        crf = self._select_crf(video_type, optimize_mode)
        
        # 基础preset选择
        preset = self._select_preset(optimize_mode)
        
        return VideoParams(
            encoder=encoder,
            crf=crf,
            preset=preset,
            confidence=0.8
        )
    
    def _select_encoder(self, video_type: VideoType) -> str:
        """
        选择编码器
        第一阶段：基础选择逻辑
        """
        # 基于分辨率的简单选择逻辑
        if video_type.width >= 1920:  # 1080p及以上
            return "h265"  # HEVC更适合高分辨率
        elif video_type.width >= 1280:  # 720p
            return "h264"  # H.264平衡性能和兼容性
        else:  # 低分辨率
            return "h264"  # H.264兼容性最好
    
    def _select_crf(self, video_type: VideoType, optimize_mode: str) -> int:
        """
        选择CRF值
        第一阶段：基础选择逻辑
        """
        if optimize_mode == "quality":
            return 20  # 高质量
        elif optimize_mode == "size":
            return 28  # 小体积
        else:  # balanced
            return 23  # 平衡
    
    def _select_preset(self, optimize_mode: str) -> str:
        """
        选择编码preset
        第一阶段：基础选择逻辑
        """
        if optimize_mode == "quality":
            return "slow"    # 质量优先，速度较慢
        elif optimize_mode == "size":
            return "slower"  # 最佳压缩
        else:  # balanced
            return "medium"  # 平衡速度和质量
    
    def test_ffmpeg_availability(self) -> bool:
        """测试ffmpeg可用性"""
        try:
            result = subprocess.run(['ffmpeg', '-version'], 
                                  capture_output=True, timeout=5)
            return result.returncode == 0
        except:
            return False
    
    def test_ffprobe_availability(self) -> bool:
        """测试ffprobe可用性"""
        try:
            result = subprocess.run(['ffprobe', '-version'], 
                                  capture_output=True, timeout=5)
            return result.returncode == 0
        except:
            return False


# 便捷函数
def predict_video_params(video_path: str, optimize_mode: str = "balanced", 
                        options: Optional[VideoRequestOptions] = None,
                        debug: bool = False) -> VideoPredictResponse:
    """
    便捷函数：预测视频编码参数
    
    Args:
        video_path: 视频文件路径
        optimize_mode: 优化模式
        options: 视频请求选项
        debug: 调试模式
        
    Returns:
        VideoPredictResponse: 预测响应
    """
    processor = EnhancedVideoProcessor(debug=debug)
    request = VideoPredictRequest(
        video_path=video_path,
        optimize_mode=optimize_mode,
        options=options
    )
    return processor.predict_video_params(request)


if __name__ == "__main__":
    # 测试代码
    import sys
    
    if len(sys.argv) > 1:
        video_path = sys.argv[1]
        optimize_mode = sys.argv[2] if len(sys.argv) > 2 else "balanced"
        
        print(f"=== 增强视频处理器测试 ===")
        print(f"视频路径: {video_path}")
        print(f"优化模式: {optimize_mode}")
        
        # 测试环境
        processor = EnhancedVideoProcessor(debug=True)
        print(f"ffmpeg可用: {processor.test_ffmpeg_availability()}")
        print(f"ffprobe可用: {processor.test_ffprobe_availability()}")
        
        # 预测参数
        response = predict_video_params(video_path, optimize_mode, debug=True)
        
        print(f"\n=== 预测结果 ===")
        print(f"成功: {response.success}")
        if response.success and response.params:
            print(f"推荐编码器: {response.params.encoder}")
            print(f"推荐CRF: {response.params.crf}")
            print(f"推荐preset: {response.params.preset}")
            print(f"置信度: {response.confidence:.2f}")
        else:
            print(f"错误: {response.error}")
        print(f"处理时间: {response.time_ms}ms")
    else:
        print("Usage: python enhanced_processor.py <video_path> [optimize_mode]")
        print("optimize_mode: quality|balanced|size")
