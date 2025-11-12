"""
VMAF质量验证器 - 第三阶段
基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/video_handlers.go VMAF部分重新实现

功能:
- VMAF质量分数计算
- 质量等级评估 (优秀/良好/可接受/较差)
- 优化建议生成
- 多线程VMAF分析支持
- libvmaf环境检测

EX-013实现: 第三阶段 - VMAF质量验证
"""

import subprocess
import json
import tempfile
import os
import sys
import time
import platform
from pathlib import Path
from typing import Dict, Optional, Tuple
from dataclasses import dataclass, asdict
import logging
import contextlib

# Import configuration from enhanced_processor to avoid circular import
try:
    from .enhanced_processor import VideoProcessorConfig
except ImportError:
    # Fallback minimal config if import fails
    from typing import Optional
    @dataclass
    class VideoProcessorConfig:
        ffmpeg_path: Optional[str] = None
        temp_dir: Optional[str] = None
        max_analysis_timeout: int = 300

@dataclass
class VMAFRequest:
    """VMAF验证请求"""
    original_path: str                  # 原始视频路径
    converted_path: str                 # 转换后视频路径
    min_score: int = 85                # 最低可接受分数
    use_model: str = "vmaf_v0.6.1"     # VMAF模型版本
    n_threads: int = 4                 # 线程数

@dataclass
class VMAFDetails:
    """VMAF详细信息"""
    rating: str = ""                   # 评级：优秀/良好/可接受/较差
    recommendation: str = ""           # 优化建议

    def to_dict(self) -> Dict[str, str]:
        return asdict(self)

@dataclass
class VMAFResponse:
    """VMAF验证响应"""
    success: bool = True
    score: float = 0.0                 # VMAF分数
    passed: bool = False               # 是否通过最低分数要求
    min_score: int = 85               # 最低分数要求
    target_score: int = 92            # 目标分数
    details: Optional[VMAFDetails] = None
    error: str = ""                   # 错误信息
    time_ms: int = 0                  # 处理时间

    def __post_init__(self):
        if self.details is None:
            self.details = VMAFDetails()

    def to_dict(self) -> Dict[str, any]:
        data = asdict(self)
        if self.details:
            data['details'] = self.details.to_dict()
        return data

@dataclass 
class Resolution:
    """视频分辨率"""
    width: int
    height: int


class VMAFValidator:
    """
    VMAF质量验证器
    高稳定性跨平台实现
    """
    
    def __init__(self, config: Optional[VideoProcessorConfig] = None, debug: bool = False):
        self.config = config or VideoProcessorConfig()
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        if debug:
            logging.basicConfig(level=logging.DEBUG)
    
    def validate_quality(self, request: VMAFRequest) -> VMAFResponse:
        """
        验证视频质量
        
        Args:
            request: VMAF验证请求
            
        Returns:
            VMAFResponse: 验证响应
        """
        start_time = time.time()
        
        try:
            # 验证输入文件
            if not Path(request.original_path).exists():
                return VMAFResponse(
                    success=False,
                    error=f"原始视频文件不存在: {request.original_path}",
                    time_ms=int((time.time() - start_time) * 1000)
                )
            
            if not Path(request.converted_path).exists():
                return VMAFResponse(
                    success=False,
                    error=f"转换后视频文件不存在: {request.converted_path}",
                    time_ms=int((time.time() - start_time) * 1000)
                )
            
            if self.debug:
                self.logger.debug(f"开始VMAF分析: {Path(request.converted_path).name}")
                self.logger.debug(f"最低分数要求: {request.min_score}")
            
            # 检查ffmpeg libvmaf支持
            if not self.check_libvmaf_support():
                return VMAFResponse(
                    success=False,
                    error="ffmpeg不支持libvmaf，请重新编译ffmpeg并启用--enable-libvmaf",
                    time_ms=int((time.time() - start_time) * 1000)
                )
            
            # 执行VMAF分析
            vmaf_score = self._run_vmaf_analysis(request)
            
            if vmaf_score is None:
                return VMAFResponse(
                    success=False,
                    error="VMAF分析失败",
                    time_ms=int((time.time() - start_time) * 1000)
                )
            
            # 构建响应
            passed = vmaf_score >= request.min_score
            details = VMAFDetails(
                rating=self._get_rating(vmaf_score),
                recommendation=self._get_recommendation(vmaf_score, request.min_score)
            )
            
            response = VMAFResponse(
                success=True,
                score=vmaf_score,
                passed=passed,
                min_score=request.min_score,
                target_score=92,
                details=details,
                time_ms=int((time.time() - start_time) * 1000)
            )
            
            if self.debug:
                self.logger.debug(f"VMAF分析完成: 分数={vmaf_score:.2f}, "
                                f"通过={passed}, 耗时={response.time_ms}ms")
            
            return response
            
        except Exception as e:
            self.logger.error(f"VMAF验证异常: {e}")
            return VMAFResponse(
                success=False,
                error=str(e),
                time_ms=int((time.time() - start_time) * 1000)
            )
    
    def check_libvmaf_support(self) -> bool:
        """检查ffmpeg是否支持libvmaf"""
        try:
            # 检查ffmpeg filters
            result = subprocess.run(['ffmpeg', '-filters'], 
                                  capture_output=True, text=True, timeout=10)
            
            if result.returncode != 0:
                return False
            
            output = result.stdout.lower()
            
            # 检查是否包含libvmaf或vmaf
            return 'libvmaf' in output or 'vmaf' in output
            
        except Exception as e:
            self.logger.error(f"检查libvmaf支持失败: {e}")
            return False
    
    def _run_vmaf_analysis(self, request: VMAFRequest) -> Optional[float]:
        """
        执行VMAF分析
        
        Args:
            request: VMAF请求
            
        Returns:
            Optional[float]: VMAF分数，失败返回None
        """
        try:
            # 获取视频分辨率
            resolution = self._get_video_resolution(request.converted_path)
            if not resolution:
                self.logger.error("无法获取视频分辨率")
                return None
            
            # 创建临时JSON文件存储VMAF结果
            with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as tmp_file:
                temp_json_path = tmp_file.name
            
            try:
                # 构建VMAF filter
                vmaf_filter = self._build_vmaf_filter(resolution, temp_json_path, request.n_threads)
                
                # 执行ffmpeg
                cmd = [
                    'ffmpeg', '-y',
                    '-i', request.converted_path,
                    '-i', request.original_path,
                    '-lavfi', vmaf_filter,
                    '-f', 'null', '-'
                ]
                
                if self.debug:
                    self.logger.debug(f"VMAF命令: {' '.join(cmd)}")
                
                result = subprocess.run(cmd, capture_output=True, text=True, timeout=300)
                
                if result.returncode != 0:
                    self.logger.error(f"ffmpeg执行失败: {result.stderr}")
                    return None
                
                # 解析VMAF JSON结果
                vmaf_score = self._parse_vmaf_json(temp_json_path)
                
                return vmaf_score
                
            finally:
                # 清理临时文件
                try:
                    os.unlink(temp_json_path)
                except:
                    pass
                    
        except subprocess.TimeoutExpired:
            self.logger.error("VMAF分析超时")
            return None
        except Exception as e:
            self.logger.error(f"VMAF分析异常: {e}")
            return None
    
    def _get_video_resolution(self, video_path: str) -> Optional[Resolution]:
        """获取视频分辨率"""
        try:
            cmd = [
                'ffprobe', '-v', 'error',
                '-select_streams', 'v:0',
                '-show_entries', 'stream=width,height',
                '-of', 'json',
                video_path
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=15)
            
            if result.returncode != 0:
                return None
            
            data = json.loads(result.stdout)
            streams = data.get('streams', [])
            
            if not streams:
                return None
            
            stream = streams[0]
            width = stream.get('width', 0)
            height = stream.get('height', 0)
            
            if width > 0 and height > 0:
                return Resolution(width=width, height=height)
            
            return None
            
        except Exception as e:
            self.logger.error(f"获取分辨率失败: {e}")
            return None
    
    def _build_vmaf_filter(self, resolution: Resolution, json_path: str, n_threads: int) -> str:
        """构建VMAF filter字符串"""
        # 构建VMAF filter
        # 注意：libvmaf要求两个输入流的分辨率完全一致
        vmaf_filter = (
            f"[0:v]scale={resolution.width}:{resolution.height}:flags=bicubic[ref];"
            f"[1:v]scale={resolution.width}:{resolution.height}:flags=bicubic[dist];"
            f"[dist][ref]libvmaf=log_fmt=json:log_path={json_path}:n_threads={n_threads}"
        )
        
        return vmaf_filter
    
    def _parse_vmaf_json(self, json_path: str) -> Optional[float]:
        """解析VMAF JSON结果文件"""
        try:
            if not Path(json_path).exists():
                self.logger.error(f"VMAF结果文件不存在: {json_path}")
                return None
            
            with open(json_path, 'r') as f:
                data = json.load(f)
            
            # 解析pooled_metrics中的VMAF mean值
            pooled_metrics = data.get('pooled_metrics', {})
            vmaf_data = pooled_metrics.get('vmaf', {})
            mean_score = vmaf_data.get('mean')
            
            if mean_score is not None:
                return float(mean_score)
            
            # 尝试其他可能的结构
            # 有些版本的libvmaf输出格式可能不同
            frames = data.get('frames', [])
            if frames:
                # 计算所有帧的VMAF分数平均值
                vmaf_scores = []
                for frame in frames:
                    metrics = frame.get('metrics', {})
                    vmaf_score = metrics.get('vmaf')
                    if vmaf_score is not None:
                        vmaf_scores.append(float(vmaf_score))
                
                if vmaf_scores:
                    return sum(vmaf_scores) / len(vmaf_scores)
            
            self.logger.error("无法从VMAF结果中提取分数")
            return None
            
        except Exception as e:
            self.logger.error(f"解析VMAF JSON失败: {e}")
            return None
    
    def _get_rating(self, score: float) -> str:
        """获取质量评级"""
        if score >= 95:
            return "优秀"
        elif score >= 90:
            return "良好"
        elif score >= 85:
            return "可接受"
        elif score >= 75:
            return "较差"
        else:
            return "不可接受"
    
    def _get_recommendation(self, score: float, min_score: int) -> str:
        """获取优化建议"""
        target_score = 92.0
        
        if score >= target_score:
            return "质量优秀，无需调整"
        elif score >= min_score:
            return "质量可接受，可考虑稍微降低CRF以获得更好质量"
        else:
            return "质量不达标，建议降低CRF值或使用更慢的preset"


# 便捷函数
def validate_video_quality(original_path: str, converted_path: str, 
                          min_score: int = 85, n_threads: int = 4,
                          config: Optional[VideoProcessorConfig] = None,
                          debug: bool = False) -> VMAFResponse:
    """
    便捷函数：验证视频质量
    
    Args:
        original_path: 原始视频路径
        converted_path: 转换后视频路径
        min_score: 最低可接受分数
        n_threads: 线程数
        config: 处理器配置，None使用默认配置
        debug: 调试模式
        
    Returns:
        VMAFResponse: 验证响应
    """
    try:
        validator = VMAFValidator(config=config, debug=debug)
        request = VMAFRequest(
            original_path=original_path,
            converted_path=converted_path,
            min_score=min_score,
            n_threads=n_threads
        )
        return validator.validate_quality(request)
    except Exception as e:
        return VMAFResponse(
            success=False,
            error=f"VMAF验证器初始化失败: {str(e)}",
            time_ms=0
        )


if __name__ == "__main__":
    # 测试代码
    import sys
    
    if len(sys.argv) >= 3:
        original_path = sys.argv[1]
        converted_path = sys.argv[2]
        min_score = int(sys.argv[3]) if len(sys.argv) > 3 else 85
        
        print(f"=== VMAF质量验证测试 ===")
        print(f"原始视频: {original_path}")
        print(f"转换视频: {converted_path}")
        print(f"最低分数: {min_score}")
        
        # 检查libvmaf支持
        validator = VMAFValidator(debug=True)
        libvmaf_ok = validator.check_libvmaf_support()
        print(f"libvmaf支持: {'✅' if libvmaf_ok else '❌'}")
        
        if libvmaf_ok:
            # 执行VMAF验证
            response = validate_video_quality(
                original_path, converted_path, min_score, debug=True
            )
            
            print(f"\n=== VMAF验证结果 ===")
            print(f"成功: {response.success}")
            if response.success:
                print(f"VMAF分数: {response.score:.2f}")
                print(f"通过验证: {'✅' if response.passed else '❌'}")
                print(f"质量评级: {response.details.rating}")
                print(f"建议: {response.details.recommendation}")
            else:
                print(f"错误: {response.error}")
            print(f"处理时间: {response.time_ms}ms")
        else:
            print("\n❌ 无法进行VMAF验证，请安装支持libvmaf的ffmpeg")
    else:
        print("Usage: python vmaf_validator.py <original_video> <converted_video> [min_score]")
        print("Example: python vmaf_validator.py original.mp4 converted.mp4 85")
