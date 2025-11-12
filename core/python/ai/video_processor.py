"""
🧠 PIXLY v3.0 视频处理器

替代Go video_handlers.go的完整实现：
- 视频格式检测与分析
- 智能编码参数预测
- 质量评估与优化
- 批量视频处理支持

完全本地化，基于FFmpeg和OpenCV
"""

import subprocess
import json
import re
import time
from pathlib import Path
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass
from enum import Enum
import threading

from .quality_metrics import QualityMetrics, get_quality_calculator


class VideoCodec(Enum):
    """视频编码器枚举"""
    H264 = "libx264"
    H265 = "libx265" 
    VP9 = "libvpx-vp9"
    AV1 = "libaom-av1"
    AUTO = "auto"


@dataclass
class VideoInfo:
    """视频信息结构"""
    file_path: str = ""
    duration: float = 0.0
    width: int = 0
    height: int = 0
    fps: float = 0.0
    bitrate: int = 0
    codec: str = ""
    format: str = ""
    file_size: int = 0
    has_audio: bool = False
    audio_codec: str = ""
    
    def get_resolution_category(self) -> str:
        """获取分辨率分类"""
        if self.width >= 3840:
            return "4K"
        elif self.width >= 1920:
            return "1080P"
        elif self.width >= 1280:
            return "720P"
        else:
            return "SD"


@dataclass
class VideoProcessingParams:
    """视频处理参数"""
    target_codec: VideoCodec = VideoCodec.H265
    crf: int = 23  # Constant Rate Factor
    preset: str = "medium"  # 编码预设
    target_bitrate: Optional[int] = None
    max_width: Optional[int] = None
    max_height: Optional[int] = None
    fps_limit: Optional[float] = None
    audio_codec: str = "aac"
    audio_bitrate: int = 128
    two_pass: bool = False


@dataclass
class VideoProcessingResult:
    """视频处理结果"""
    success: bool = False
    input_file: str = ""
    output_file: str = ""
    original_size: int = 0
    output_size: int = 0
    compression_ratio: float = 0.0
    savings_percent: float = 0.0
    processing_time: float = 0.0
    quality_metrics: Optional[QualityMetrics] = None
    error_message: str = ""


class LocalVideoProcessor:
    """
    🧠 本地化视频处理器
    
    替代Go video_handlers的完整Python实现
    """
    
    def __init__(self):
        self._lock = threading.RLock()
        self.ffmpeg_path = self._find_ffmpeg()
        self.quality_calculator = get_quality_calculator()
        
        print(f"✅ 视频处理器初始化: FFmpeg={'可用' if self.ffmpeg_path else '不可用'}")
    
    def _find_ffmpeg(self) -> Optional[str]:
        """查找FFmpeg可执行文件"""
        possible_paths = [
            "ffmpeg",
            "/usr/local/bin/ffmpeg",
            "/opt/homebrew/bin/ffmpeg",
            "C:/ffmpeg/bin/ffmpeg.exe"
        ]
        
        for path in possible_paths:
            try:
                result = subprocess.run([path, "-version"], 
                                      capture_output=True, text=True, timeout=5)
                if result.returncode == 0:
                    return path
            except (subprocess.TimeoutExpired, FileNotFoundError):
                continue
        
        return None
    
    def analyze_video(self, video_path: str) -> VideoInfo:
        """
        分析视频文件信息
        
        Args:
            video_path: 视频文件路径
            
        Returns:
            VideoInfo: 视频信息结构
        """
        if not self.ffmpeg_path:
            return VideoInfo(file_path=video_path, file_size=Path(video_path).stat().st_size)
        
        try:
            # 使用ffprobe获取视频信息
            cmd = [
                self.ffmpeg_path.replace("ffmpeg", "ffprobe"),
                "-v", "quiet",
                "-print_format", "json",
                "-show_format",
                "-show_streams",
                video_path
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
            
            if result.returncode != 0:
                raise subprocess.CalledProcessError(result.returncode, cmd, result.stderr)
            
            data = json.loads(result.stdout)
            
            # 解析视频流信息
            video_stream = None
            audio_stream = None
            
            for stream in data.get("streams", []):
                if stream.get("codec_type") == "video" and video_stream is None:
                    video_stream = stream
                elif stream.get("codec_type") == "audio" and audio_stream is None:
                    audio_stream = stream
            
            # 构建VideoInfo
            info = VideoInfo()
            info.file_path = video_path
            info.file_size = Path(video_path).stat().st_size
            
            if video_stream:
                info.width = int(video_stream.get("width", 0))
                info.height = int(video_stream.get("height", 0))
                info.codec = video_stream.get("codec_name", "")
                
                # 解析FPS
                fps_str = video_stream.get("r_frame_rate", "0/1")
                if "/" in fps_str:
                    num, den = fps_str.split("/")
                    info.fps = float(num) / float(den) if float(den) != 0 else 0
                
                # 解析比特率
                info.bitrate = int(video_stream.get("bit_rate", 0))
            
            if audio_stream:
                info.has_audio = True
                info.audio_codec = audio_stream.get("codec_name", "")
            
            # 解析总时长
            format_info = data.get("format", {})
            info.duration = float(format_info.get("duration", 0))
            info.format = format_info.get("format_name", "")
            
            return info
            
        except Exception as e:
            print(f"❌ 视频分析失败: {e}")
            return VideoInfo(file_path=video_path, file_size=Path(video_path).stat().st_size)
    
    def predict_optimal_params(self, video_info: VideoInfo, target_quality: str = "balanced") -> VideoProcessingParams:
        """
        预测最优处理参数
        
        Args:
            video_info: 视频信息
            target_quality: 目标质量 ("high", "balanced", "size")
            
        Returns:
            VideoProcessingParams: 推荐的处理参数
        """
        params = VideoProcessingParams()
        
        # 根据分辨率选择编码器
        if video_info.width >= 1920:
            params.target_codec = VideoCodec.H265  # 高分辨率使用HEVC
        else:
            params.target_codec = VideoCodec.H264  # 标清使用H.264
        
        # 根据质量目标调整CRF
        if target_quality == "high":
            params.crf = 18
            params.preset = "slow"
        elif target_quality == "balanced":
            params.crf = 23
            params.preset = "medium"
        elif target_quality == "size":
            params.crf = 28
            params.preset = "fast"
        
        # 音频处理
        if video_info.has_audio:
            params.audio_codec = "aac"
            if target_quality == "high":
                params.audio_bitrate = 192
            else:
                params.audio_bitrate = 128
        
        # 分辨率限制（如果需要）
        if video_info.width > 1920 and target_quality != "high":
            params.max_width = 1920
            params.max_height = 1080
        
        return params
    
    def process_video(self, input_path: str, output_path: str, 
                     params: VideoProcessingParams) -> VideoProcessingResult:
        """
        处理视频文件
        
        Args:
            input_path: 输入文件路径
            output_path: 输出文件路径
            params: 处理参数
            
        Returns:
            VideoProcessingResult: 处理结果
        """
        if not self.ffmpeg_path:
            return VideoProcessingResult(
                success=False,
                error_message="FFmpeg不可用"
            )
        
        start_time = time.time()
        
        try:
            # 构建FFmpeg命令
            cmd = [self.ffmpeg_path, "-i", input_path]
            
            # 视频编码参数
            if params.target_codec != VideoCodec.AUTO:
                cmd.extend(["-c:v", params.target_codec.value])
            
            if params.crf:
                cmd.extend(["-crf", str(params.crf)])
            
            if params.preset:
                cmd.extend(["-preset", params.preset])
            
            # 分辨率调整
            if params.max_width and params.max_height:
                cmd.extend(["-vf", f"scale={params.max_width}:{params.max_height}:force_original_aspect_ratio=decrease"])
            
            # 音频编码参数
            if params.audio_codec:
                cmd.extend(["-c:a", params.audio_codec])
                if params.audio_bitrate:
                    cmd.extend(["-b:a", f"{params.audio_bitrate}k"])
            
            # 输出选项
            cmd.extend(["-y", output_path])  # -y 覆盖输出文件
            
            # 执行FFmpeg
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=3600)  # 1小时超时
            
            processing_time = time.time() - start_time
            
            if result.returncode != 0:
                return VideoProcessingResult(
                    success=False,
                    input_file=input_path,
                    error_message=result.stderr,
                    processing_time=processing_time
                )
            
            # 计算文件大小和压缩比
            original_size = Path(input_path).stat().st_size
            output_size = Path(output_path).stat().st_size if Path(output_path).exists() else 0
            
            compression_ratio = output_size / original_size if original_size > 0 else 0
            savings_percent = (1 - compression_ratio) * 100
            
            return VideoProcessingResult(
                success=True,
                input_file=input_path,
                output_file=output_path,
                original_size=original_size,
                output_size=output_size,
                compression_ratio=compression_ratio,
                savings_percent=savings_percent,
                processing_time=processing_time
            )
            
        except subprocess.TimeoutExpired:
            return VideoProcessingResult(
                success=False,
                input_file=input_path,
                error_message="处理超时",
                processing_time=time.time() - start_time
            )
        except Exception as e:
            return VideoProcessingResult(
                success=False,
                input_file=input_path,
                error_message=str(e),
                processing_time=time.time() - start_time
            )
    
    def batch_process_videos(self, video_files: List[str], output_dir: str,
                           params: VideoProcessingParams) -> List[VideoProcessingResult]:
        """
        批量处理视频文件
        
        Args:
            video_files: 视频文件路径列表
            output_dir: 输出目录
            params: 处理参数
            
        Returns:
            List[VideoProcessingResult]: 处理结果列表
        """
        results = []
        output_path = Path(output_dir)
        output_path.mkdir(parents=True, exist_ok=True)
        
        for i, video_file in enumerate(video_files):
            print(f"🎬 处理视频 {i+1}/{len(video_files)}: {Path(video_file).name}")
            
            # 生成输出文件名
            input_path = Path(video_file)
            output_file = output_path / f"{input_path.stem}_optimized{input_path.suffix}"
            
            # 处理单个视频
            result = self.process_video(str(input_path), str(output_file), params)
            results.append(result)
            
            if result.success:
                print(f"✅ 完成: 节省 {result.savings_percent:.1f}%")
            else:
                print(f"❌ 失败: {result.error_message}")
        
        return results
    
    def analyze_batch_results(self, results: List[VideoProcessingResult]) -> Dict[str, Any]:
        """分析批量处理结果"""
        successful = [r for r in results if r.success]
        failed = [r for r in results if not r.success]
        
        if not successful:
            return {
                "total": len(results),
                "successful": 0,
                "failed": len(failed),
                "success_rate": 0.0
            }
        
        total_original_size = sum(r.original_size for r in successful)
        total_output_size = sum(r.output_size for r in successful)
        total_processing_time = sum(r.processing_time for r in successful)
        
        return {
            "total": len(results),
            "successful": len(successful),
            "failed": len(failed),
            "success_rate": len(successful) / len(results) * 100,
            "total_savings_mb": (total_original_size - total_output_size) / (1024 * 1024),
            "avg_savings_percent": sum(r.savings_percent for r in successful) / len(successful),
            "total_processing_time": total_processing_time,
            "avg_processing_time": total_processing_time / len(successful),
            "compression_efficiency": total_output_size / total_original_size if total_original_size > 0 else 0
        }
    
    def get_supported_formats(self) -> List[str]:
        """获取支持的视频格式"""
        if not self.ffmpeg_path:
            return ["mp4", "mkv", "avi"]  # 默认格式
        
        try:
            result = subprocess.run([self.ffmpeg_path, "-formats"], 
                                  capture_output=True, text=True, timeout=10)
            
            formats = []
            for line in result.stdout.split('\n'):
                if line.strip().startswith('DE'):  # 支持解码和编码
                    parts = line.split()
                    if len(parts) >= 3:
                        format_name = parts[1]
                        if format_name in ['mp4', 'mkv', 'avi', 'webm', 'mov']:
                            formats.append(format_name)
            
            return formats if formats else ["mp4", "mkv", "avi"]
            
        except Exception:
            return ["mp4", "mkv", "avi"]
    
    def estimate_processing_time(self, video_info: VideoInfo, 
                               params: VideoProcessingParams) -> float:
        """估算处理时间（秒）"""
        # 基础时间 = 视频时长
        base_time = video_info.duration
        
        # 编码器复杂度系数
        codec_multiplier = {
            VideoCodec.H264: 1.0,
            VideoCodec.H265: 2.5,
            VideoCodec.VP9: 3.0,
            VideoCodec.AV1: 5.0
        }.get(params.target_codec, 1.0)
        
        # 预设复杂度系数
        preset_multiplier = {
            "ultrafast": 0.3,
            "fast": 0.7,
            "medium": 1.0,
            "slow": 2.0,
            "veryslow": 4.0
        }.get(params.preset, 1.0)
        
        # 分辨率系数
        resolution_multiplier = 1.0
        if video_info.width >= 3840:  # 4K
            resolution_multiplier = 4.0
        elif video_info.width >= 1920:  # 1080p
            resolution_multiplier = 2.0
        
        estimated_time = base_time * codec_multiplier * preset_multiplier * resolution_multiplier
        
        return max(estimated_time, 10.0)  # 最少10秒


# 全局视频处理器实例
_global_video_processor: Optional[LocalVideoProcessor] = None
_processor_lock = threading.Lock()

def get_video_processor() -> LocalVideoProcessor:
    """获取全局视频处理器实例（单例模式）"""
    global _global_video_processor
    
    with _processor_lock:
        if _global_video_processor is None:
            _global_video_processor = LocalVideoProcessor()
        return _global_video_processor
