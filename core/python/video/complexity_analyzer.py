"""
视频复杂度分析器 - 第二阶段
基于废弃Go代码扩展，实现高级视频复杂度分析

功能:
- 运动复杂度分析
- 纹理复杂度分析
- 场景切换检测
- 编码难度评估
- CRF范围推荐

EX-013实现: 第二阶段 - 复杂度分析
"""

import subprocess
import json
import tempfile
import os
import numpy as np
from typing import Tuple, List, Dict, Optional
from dataclasses import dataclass
import logging

from .enhanced_processor import VideoComplexity, VideoType

class VideoComplexityAnalyzer:
    """视频复杂度分析器"""
    
    def __init__(self, debug: bool = False):
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        if debug:
            logging.basicConfig(level=logging.DEBUG)
    
    def analyze_complexity(self, video_path: str, video_type: VideoType) -> VideoComplexity:
        """
        分析视频复杂度
        
        Args:
            video_path: 视频文件路径
            video_type: 视频类型信息
            
        Returns:
            VideoComplexity: 复杂度分析结果
        """
        try:
            if self.debug:
                self.logger.debug(f"开始分析视频复杂度: {video_path}")
            
            # 运动复杂度分析
            motion_score = self._analyze_motion_complexity(video_path)
            
            # 纹理复杂度分析  
            texture_score = self._analyze_texture_complexity(video_path)
            
            # 场景切换检测
            scene_changes = self._detect_scene_changes(video_path)
            
            # 细节等级分析
            detail_level = self._analyze_detail_level(video_path, video_type)
            
            # 编码难度评估
            encoding_difficulty = self._evaluate_encoding_difficulty(
                motion_score, texture_score, scene_changes, detail_level
            )
            
            # CRF范围推荐
            crf_range = self._recommend_crf_range(
                encoding_difficulty, motion_score, texture_score
            )
            
            complexity = VideoComplexity(
                motion_score=motion_score,
                texture_score=texture_score,
                scene_changes=scene_changes,
                detail_level=detail_level,
                encoding_difficulty=encoding_difficulty,
                recommended_crf_range=crf_range
            )
            
            if self.debug:
                self.logger.debug(f"复杂度分析完成: {encoding_difficulty}, "
                                f"运动={motion_score:.2f}, 纹理={texture_score:.2f}")
            
            return complexity
            
        except Exception as e:
            self.logger.error(f"复杂度分析失败: {e}")
            # 返回默认值
            return VideoComplexity()
    
    def _analyze_motion_complexity(self, video_path: str) -> float:
        """
        分析运动复杂度
        使用ffmpeg的motion vectors分析
        """
        try:
            # 使用ffmpeg分析前10秒的运动向量
            with tempfile.NamedTemporaryFile(suffix='.log', delete=False) as tmp_file:
                cmd = [
                    'ffmpeg', '-i', video_path,
                    '-t', '10',  # 只分析前10秒
                    '-vf', 'mestimate=me_mode=epzs,showinfo',
                    '-f', 'null', '-',
                    '-v', 'info'
                ]
                
                result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
                
                if result.returncode != 0:
                    self.logger.warning("运动分析失败，使用默认值")
                    return 0.5
                
                # 解析ffmpeg输出中的运动信息
                motion_info = self._parse_motion_info(result.stderr)
                
                # 计算运动复杂度分数 (0-1)
                motion_score = min(1.0, motion_info / 100.0)
                
                return motion_score
                
        except subprocess.TimeoutExpired:
            self.logger.warning("运动分析超时")
            return 0.5
        except Exception as e:
            self.logger.error(f"运动分析异常: {e}")
            return 0.5
    
    def _parse_motion_info(self, stderr_output: str) -> float:
        """解析ffmpeg运动信息输出"""
        try:
            # 查找包含运动信息的行
            motion_values = []
            for line in stderr_output.split('\n'):
                if 'showinfo' in line and 'mean_diff' in line:
                    # 提取mean_diff值（像素差值的平均值）
                    parts = line.split()
                    for part in parts:
                        if 'mean_diff:' in part:
                            try:
                                value = float(part.split(':')[1])
                                motion_values.append(value)
                            except:
                                continue
            
            if motion_values:
                # 返回平均运动强度
                return np.mean(motion_values)
            else:
                return 50.0  # 默认中等运动
                
        except Exception:
            return 50.0
    
    def _analyze_texture_complexity(self, video_path: str) -> float:
        """
        分析纹理复杂度
        使用ffmpeg的spatial info分析
        """
        try:
            # 使用ffmpeg分析空间复杂度
            cmd = [
                'ffmpeg', '-i', video_path,
                '-t', '10',  # 前10秒
                '-vf', 'siti=print_summary=1',
                '-f', 'null', '-',
                '-v', 'info'
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
            
            if result.returncode != 0:
                # 如果siti过滤器不可用，使用备用方法
                return self._analyze_texture_fallback(video_path)
            
            # 解析spatial information
            si_values = self._parse_spatial_info(result.stderr)
            
            # 转换为0-1范围的纹理复杂度
            texture_score = min(1.0, si_values / 200.0)  # SI通常在0-200范围
            
            return texture_score
            
        except subprocess.TimeoutExpired:
            self.logger.warning("纹理分析超时")
            return 0.5
        except Exception as e:
            self.logger.error(f"纹理分析异常: {e}")
            return 0.5
    
    def _parse_spatial_info(self, stderr_output: str) -> float:
        """解析空间信息输出"""
        try:
            si_values = []
            for line in stderr_output.split('\n'):
                if 'SI:' in line:
                    # 提取SI值
                    parts = line.split('SI:')
                    if len(parts) > 1:
                        try:
                            si_part = parts[1].split()[0]
                            si_value = float(si_part)
                            si_values.append(si_value)
                        except:
                            continue
            
            if si_values:
                return np.mean(si_values)
            else:
                return 100.0  # 默认中等纹理复杂度
                
        except Exception:
            return 100.0
    
    def _analyze_texture_fallback(self, video_path: str) -> float:
        """纹理分析备用方法：使用像素标准差"""
        try:
            # 提取几帧并分析像素标准差
            cmd = [
                'ffmpeg', '-i', video_path,
                '-vf', 'select=not(mod(n\\,30)),scale=320:240',
                '-frames:v', '5',
                '-f', 'rawvideo', '-pix_fmt', 'gray', '-'
            ]
            
            result = subprocess.run(cmd, capture_output=True, timeout=15)
            
            if result.returncode != 0:
                return 0.5
            
            # 分析原始像素数据的标准差
            frame_size = 320 * 240
            frames_data = result.stdout
            
            if len(frames_data) < frame_size:
                return 0.5
            
            # 计算第一帧的标准差
            frame_data = np.frombuffer(frames_data[:frame_size], dtype=np.uint8)
            std_dev = np.std(frame_data.astype(float))
            
            # 标准差越大，纹理越复杂（归一化到0-1）
            texture_score = min(1.0, std_dev / 64.0)
            
            return texture_score
            
        except Exception as e:
            self.logger.error(f"备用纹理分析失败: {e}")
            return 0.5
    
    def _detect_scene_changes(self, video_path: str) -> int:
        """
        检测场景切换次数
        使用ffmpeg的scene detection
        """
        try:
            # 使用ffmpeg的scene filter检测场景切换
            cmd = [
                'ffmpeg', '-i', video_path,
                '-vf', 'select=gt(scene\\,0.3)',
                '-f', 'null', '-',
                '-v', 'info'
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
            
            if result.returncode != 0:
                self.logger.warning("场景检测失败，使用备用方法")
                return self._detect_scene_changes_fallback(video_path)
            
            # 计算选中的帧数（即场景切换数）
            scene_count = result.stderr.count('Parsed_select')
            
            return max(0, scene_count)
            
        except subprocess.TimeoutExpired:
            self.logger.warning("场景检测超时")
            return 5  # 默认值
        except Exception as e:
            self.logger.error(f"场景检测异常: {e}")
            return 5
    
    def _detect_scene_changes_fallback(self, video_path: str) -> int:
        """场景检测备用方法：基于时长估算"""
        try:
            # 获取视频时长
            cmd = [
                'ffprobe', '-v', 'quiet',
                '-show_entries', 'format=duration',
                '-of', 'default=noprint_wrappers=1:nokey=1',
                video_path
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
            
            if result.returncode == 0 and result.stdout.strip():
                duration = float(result.stdout.strip())
                # 估算：平均每15秒一个场景切换
                estimated_scenes = max(1, int(duration / 15))
                return estimated_scenes
            
            return 5
            
        except Exception:
            return 5
    
    def _analyze_detail_level(self, video_path: str, video_type: VideoType) -> float:
        """
        分析细节等级
        基于分辨率和码率
        """
        try:
            # 基于分辨率的细节基础分数
            pixel_count = video_type.width * video_type.height
            resolution_score = min(1.0, pixel_count / (1920 * 1080))  # 以1080p为基准
            
            # 基于码率的细节分数
            if video_type.bitrate > 0:
                # 计算每像素的码率
                bitrate_per_pixel = video_type.bitrate / pixel_count if pixel_count > 0 else 0
                bitrate_score = min(1.0, bitrate_per_pixel * 1000000)  # 调整比例
            else:
                bitrate_score = 0.5
            
            # 综合细节等级
            detail_level = (resolution_score * 0.6 + bitrate_score * 0.4)
            
            return detail_level
            
        except Exception:
            return 0.5
    
    def _evaluate_encoding_difficulty(self, motion_score: float, texture_score: float, 
                                    scene_changes: int, detail_level: float) -> str:
        """
        评估编码难度
        
        Args:
            motion_score: 运动复杂度 (0-1)
            texture_score: 纹理复杂度 (0-1)  
            scene_changes: 场景切换次数
            detail_level: 细节等级 (0-1)
            
        Returns:
            str: 编码难度 (easy/medium/hard)
        """
        try:
            # 场景切换归一化分数
            scene_score = min(1.0, scene_changes / 20.0)  # 20次切换为满分
            
            # 综合复杂度分数
            complexity_score = (
                motion_score * 0.3 +
                texture_score * 0.3 +
                scene_score * 0.2 +
                detail_level * 0.2
            )
            
            if complexity_score >= 0.7:
                return "hard"
            elif complexity_score >= 0.4:
                return "medium"
            else:
                return "easy"
                
        except Exception:
            return "medium"
    
    def _recommend_crf_range(self, encoding_difficulty: str, motion_score: float, 
                           texture_score: float) -> Tuple[int, int]:
        """
        推荐CRF范围
        
        Args:
            encoding_difficulty: 编码难度
            motion_score: 运动复杂度
            texture_score: 纹理复杂度
            
        Returns:
            Tuple[int, int]: (最低CRF, 最高CRF)
        """
        try:
            base_ranges = {
                "easy": (25, 30),
                "medium": (22, 27), 
                "hard": (18, 24)
            }
            
            min_crf, max_crf = base_ranges.get(encoding_difficulty, (22, 27))
            
            # 根据运动和纹理复杂度调整
            if motion_score > 0.7 or texture_score > 0.7:
                # 高复杂度需要更低的CRF
                min_crf = max(15, min_crf - 3)
                max_crf = max(min_crf + 2, max_crf - 2)
            elif motion_score < 0.3 and texture_score < 0.3:
                # 低复杂度可以使用更高的CRF
                min_crf = min(28, min_crf + 2)
                max_crf = min(32, max_crf + 3)
            
            return (min_crf, max_crf)
            
        except Exception:
            return (22, 27)  # 默认范围


# 便捷函数
def analyze_video_complexity(video_path: str, video_type: VideoType, 
                            debug: bool = False) -> VideoComplexity:
    """
    便捷函数：分析视频复杂度
    
    Args:
        video_path: 视频文件路径
        video_type: 视频类型信息
        debug: 调试模式
        
    Returns:
        VideoComplexity: 复杂度分析结果
    """
    analyzer = VideoComplexityAnalyzer(debug=debug)
    return analyzer.analyze_complexity(video_path, video_type)


if __name__ == "__main__":
    # 测试代码
    import sys
    from .enhanced_processor import EnhancedVideoProcessor
    
    if len(sys.argv) > 1:
        video_path = sys.argv[1]
        
        print(f"=== 视频复杂度分析测试 ===")
        print(f"视频路径: {video_path}")
        
        # 首先获取视频类型信息
        processor = EnhancedVideoProcessor(debug=True)
        video_type = processor._analyze_video_type(video_path)
        
        if video_type:
            print(f"视频信息: {video_type.width}x{video_type.height}, {video_type.fps}fps")
            
            # 分析复杂度
            complexity = analyze_video_complexity(video_path, video_type, debug=True)
            
            print(f"\n=== 复杂度分析结果 ===")
            print(f"运动复杂度: {complexity.motion_score:.2f}")
            print(f"纹理复杂度: {complexity.texture_score:.2f}")
            print(f"场景切换: {complexity.scene_changes}")
            print(f"细节等级: {complexity.detail_level:.2f}")
            print(f"编码难度: {complexity.encoding_difficulty}")
            print(f"推荐CRF: {complexity.recommended_crf_range[0]}-{complexity.recommended_crf_range[1]}")
        else:
            print("❌ 无法获取视频类型信息")
    else:
        print("Usage: python complexity_analyzer.py <video_path>")
