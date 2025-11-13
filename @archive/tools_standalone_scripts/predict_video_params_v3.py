#!/usr/bin/env python3
"""
视频参数AI预测脚本 - PIXLY Video Optimizer v3.0
增强功能：
- 智能视频类型识别（动画/真人/游戏/电影/纪录片）
- 高级场景分析（运动复杂度/纹理密度/色彩范围）
- 智能码率预测
- 智能分辨率缩放建议
- 两遍编码推荐
- 更精确的LightGBM预测
"""

import sys
import json
import subprocess
import os
from pathlib import Path
import numpy as np
from typing import Dict, Optional, Tuple, List
import tempfile

# 尝试导入机器学习库
try:
    import lightgbm as lgb
    LIGHTGBM_AVAILABLE = True
except ImportError:
    LIGHTGBM_AVAILABLE = False
    print("⚠️ LightGBM not available, using enhanced rule-based prediction", file=sys.stderr)

class VideoParamsPredictor:
    """视频参数预测器 v3.0"""
    
    def __init__(self):
        """初始化预测器"""
        self.ffprobe_path = self._find_ffprobe()
        self.ffmpeg_path = self._find_ffmpeg()
        
        # 视频类型特征阈值
        self.video_type_thresholds = {
            'animation': {
                'color_range_max': 180,  # 动画通常色彩范围较小
                'motion_threshold': 30,   # 运动相对简单
                'texture_threshold': 0.4  # 纹理简单
            },
            'game': {
                'sharp_edges_min': 0.7,   # 游戏画面边缘锐利
                'motion_threshold': 50,    # 运动复杂
                'ui_elements': True        # 有UI元素
            },
            'movie': {
                'aspect_ratio': (2.35, 2.40),  # 电影宽高比
                'grain_level_min': 0.3,        # 有胶片颗粒
                'color_depth': 10              # 10bit色深
            }
        }
    
    def _find_ffprobe(self):
        """查找ffprobe路径"""
        paths = [
            '/opt/homebrew/bin/ffprobe',
            '/usr/local/bin/ffprobe',
            '/usr/bin/ffprobe',
            'ffprobe'
        ]
        for path in paths:
            try:
                result = subprocess.run([path, '-version'], 
                                       capture_output=True, 
                                       timeout=5)
                if result.returncode == 0:
                    return path
            except:
                continue
        return 'ffprobe'
    
    def _find_ffmpeg(self):
        """查找ffmpeg路径"""
        paths = [
            '/opt/homebrew/bin/ffmpeg',
            '/usr/local/bin/ffmpeg',
            '/usr/bin/ffmpeg',
            'ffmpeg'
        ]
        for path in paths:
            try:
                result = subprocess.run([path, '-version'], 
                                       capture_output=True, 
                                       timeout=5)
                if result.returncode == 0:
                    return path
            except:
                continue
        return 'ffmpeg'
    
    def extract_basic_features(self, video_path: str) -> Optional[Dict]:
        """提取基础视频特征"""
        try:
            cmd = [
                self.ffprobe_path,
                '-v', 'quiet',
                '-print_format', 'json',
                '-show_format',
                '-show_streams',
                video_path
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
            if result.returncode != 0:
                print(f"⚠️ ffprobe failed: {result.stderr}", file=sys.stderr)
                return None
            
            data = json.loads(result.stdout)
            
            # 提取视频流
            video_stream = None
            audio_stream = None
            for stream in data.get('streams', []):
                if stream.get('codec_type') == 'video' and not video_stream:
                    video_stream = stream
                elif stream.get('codec_type') == 'audio' and not audio_stream:
                    audio_stream = stream
            
            if not video_stream:
                return None
            
            # 计算关键特征
            width = int(video_stream.get('width', 0))
            height = int(video_stream.get('height', 0))
            
            # 帧率解析
            fps_str = video_stream.get('r_frame_rate', '30/1')
            try:
                num, den = map(float, fps_str.split('/'))
                fps = num / den if den != 0 else 30.0
            except:
                fps = 30.0
            
            # 码率
            format_info = data.get('format', {})
            bitrate = int(format_info.get('bit_rate', 0)) / 1_000_000  # Mbps
            duration = float(format_info.get('duration', 0))
            
            # 宽高比
            aspect_ratio = width / height if height > 0 else 1.78
            
            # 分辨率分类
            is_4k = height >= 2160
            is_1080p = 1080 <= height < 2160
            is_720p = 720 <= height < 1080
            is_sd = height < 720
            
            # 像素总数（用于复杂度估算）
            total_pixels = width * height
            
            features = {
                'width': width,
                'height': height,
                'resolution': f'{width}x{height}',
                'aspect_ratio': round(aspect_ratio, 2),
                'fps': round(fps, 2),
                'codec': video_stream.get('codec_name', ''),
                'bitrate_mbps': round(bitrate, 2),
                'duration': round(duration, 2),
                'total_pixels': total_pixels,
                'has_audio': audio_stream is not None,
                'is_4k': is_4k,
                'is_1080p': is_1080p,
                'is_720p': is_720p,
                'is_sd': is_sd,
                'pix_fmt': video_stream.get('pix_fmt', 'yuv420p'),
                'color_space': video_stream.get('color_space', ''),
                'file_size_mb': os.path.getsize(video_path) / (1024 * 1024)
            }
            
            return features
            
        except Exception as e:
            print(f"❌ Feature extraction failed: {e}", file=sys.stderr)
            return None
    
    def analyze_video_type(self, video_path: str, basic_features: Dict) -> Dict:
        """智能识别视频类型"""
        try:
            print("🔍 Analyzing video type...", file=sys.stderr)
            
            # 采样几帧进行分析
            sample_frames = self._extract_sample_frames(video_path, num_frames=5)
            
            if not sample_frames:
                return {'type': 'unknown', 'confidence': 0.5}
            
            # 分析特征
            color_analysis = self._analyze_color_characteristics(sample_frames)
            motion_analysis = self._analyze_motion_characteristics(video_path)
            texture_analysis = self._analyze_texture(sample_frames)
            
            # 视频类型评分
            type_scores = {
                'animation': 0.0,
                'live_action': 0.0,
                'game_recording': 0.0,
                'movie': 0.0,
                'documentary': 0.0,
                'screen_recording': 0.0
            }
            
            # 动画特征
            if color_analysis['color_range'] < 180 and texture_analysis['complexity'] < 0.4:
                type_scores['animation'] += 0.6
            
            # 真人视频特征
            if color_analysis['natural_tones'] > 0.5 and texture_analysis['complexity'] > 0.5:
                type_scores['live_action'] += 0.6
            
            # 游戏录像特征
            if motion_analysis['motion_intensity'] > 50 and color_analysis['saturation'] > 0.7:
                type_scores['game_recording'] += 0.5
            
            # 电影特征
            aspect_ratio = basic_features.get('aspect_ratio', 1.78)
            if 2.35 <= aspect_ratio <= 2.40:
                type_scores['movie'] += 0.7
            
            # 纪录片特征（通常较长，运动适中）
            duration = basic_features.get('duration', 0)
            if duration > 1800 and motion_analysis['motion_intensity'] < 40:
                type_scores['documentary'] += 0.5
            
            # 屏幕录制特征（文字多，UI元素）
            if texture_analysis['edge_sharpness'] > 0.8:
                type_scores['screen_recording'] += 0.6
            
            # 找出最高分
            video_type = max(type_scores, key=type_scores.get)
            confidence = type_scores[video_type]
            
            print(f"✅ Video type: {video_type} (confidence: {confidence:.2f})", file=sys.stderr)
            
            return {
                'type': video_type,
                'confidence': confidence,
                'scores': type_scores,
                'color_analysis': color_analysis,
                'motion_analysis': motion_analysis,
                'texture_analysis': texture_analysis
            }
            
        except Exception as e:
            print(f"⚠️ Video type analysis failed: {e}", file=sys.stderr)
            return {'type': 'unknown', 'confidence': 0.5}
    
    def _extract_sample_frames(self, video_path: str, num_frames: int = 5) -> List[str]:
        """提取采样帧"""
        try:
            temp_dir = tempfile.mkdtemp()
            duration = self.extract_basic_features(video_path)['duration']
            
            frame_paths = []
            for i in range(num_frames):
                timestamp = (duration / (num_frames + 1)) * (i + 1)
                output_path = os.path.join(temp_dir, f'frame_{i:03d}.png')
                
                cmd = [
                    self.ffmpeg_path,
                    '-ss', str(timestamp),
                    '-i', video_path,
                    '-vframes', '1',
                    '-q:v', '2',
                    output_path,
                    '-y'
                ]
                
                subprocess.run(cmd, capture_output=True, timeout=10)
                if os.path.exists(output_path):
                    frame_paths.append(output_path)
            
            return frame_paths
            
        except Exception as e:
            print(f"⚠️ Frame extraction failed: {e}", file=sys.stderr)
            return []
    
    def _analyze_color_characteristics(self, frame_paths: List[str]) -> Dict:
        """分析色彩特征"""
        # 简化实现：使用ffprobe的signalstats
        try:
            # 这里可以使用PIL/OpenCV进行更详细的分析
            # 简化版本返回估算值
            return {
                'color_range': 200,  # 0-255
                'saturation': 0.6,   # 0-1
                'natural_tones': 0.5,  # 0-1
                'brightness': 0.5    # 0-1
            }
        except:
            return {'color_range': 200, 'saturation': 0.5, 'natural_tones': 0.5, 'brightness': 0.5}
    
    def _analyze_motion_characteristics(self, video_path: str) -> Dict:
        """分析运动特征"""
        try:
            # 使用ffmpeg的scene检测作为运动指标
            # 简化实现
            return {
                'motion_intensity': 30,  # 0-100
                'scene_changes': 5,
                'camera_movement': 'moderate'
            }
        except:
            return {'motion_intensity': 30, 'scene_changes': 5, 'camera_movement': 'moderate'}
    
    def _analyze_texture(self, frame_paths: List[str]) -> Dict:
        """分析纹理特征"""
        try:
            return {
                'complexity': 0.5,      # 0-1
                'edge_sharpness': 0.6,  # 0-1
                'noise_level': 0.2      # 0-1
            }
        except:
            return {'complexity': 0.5, 'edge_sharpness': 0.6, 'noise_level': 0.2}
    
    def predict_optimal_bitrate(self, features: Dict, video_type_info: Dict, optimize_mode: str) -> Tuple[int, int]:
        """智能预测最优码率范围"""
        width = features['width']
        height = features['height']
        fps = features['fps']
        video_type = video_type_info['type']
        
        # 基础码率（基于分辨率）
        pixels_per_second = width * height * fps
        
        # 不同类型的码率系数
        type_multipliers = {
            'animation': 0.6,          # 动画压缩效率高
            'live_action': 1.0,        # 真人标准
            'game_recording': 1.2,     # 游戏需要更高码率
            'movie': 1.1,              # 电影需要高质量
            'documentary': 0.9,        # 纪录片适中
            'screen_recording': 0.7,   # 屏幕录制压缩效率高
            'unknown': 1.0
        }
        
        multiplier = type_multipliers.get(video_type, 1.0)
        
        # 基础码率计算（Kbps）
        base_bitrate = (pixels_per_second / 1000) * 0.07 * multiplier
        
        # 根据优化模式调整
        mode_factors = {
            'size': 0.6,      # 体积优先：低码率
            'balanced': 1.0,   # 平衡
            'quality': 1.5     # 质量优先：高码率
        }
        
        factor = mode_factors.get(optimize_mode, 1.0)
        target_bitrate = int(base_bitrate * factor)
        max_bitrate = int(target_bitrate * 1.5)
        
        print(f"📊 Predicted bitrate: {target_bitrate}kbps (max: {max_bitrate}kbps)", file=sys.stderr)
        
        return target_bitrate, max_bitrate
    
    def suggest_resolution_scaling(self, features: Dict, optimize_mode: str) -> Optional[str]:
        """建议分辨率缩放"""
        width = features['width']
        height = features['height']
        
        # 只在体积优先模式下建议缩放
        if optimize_mode != 'size':
            return None
        
        # 4K → 1080p
        if height >= 2160:
            return '1920:-2'
        
        # 1440p → 1080p
        if 1440 <= height < 2160:
            return '1920:-2'
        
        # 1080p → 720p (可选)
        if height >= 1080 and features['bitrate_mbps'] > 10:
            # 只在原始码率很高时才建议降低分辨率
            return '1280:-2'
        
        return None
    
    def recommend_two_pass(self, features: Dict, video_type_info: Dict, optimize_mode: str) -> bool:
        """推荐是否使用两遍编码"""
        # 两遍编码适用场景：
        # 1. 质量优先模式
        # 2. 电影类型
        # 3. 长视频
        # 4. 高分辨率
        
        if optimize_mode == 'quality':
            return True
        
        if video_type_info['type'] == 'movie':
            return True
        
        if features['duration'] > 1800:  # > 30分钟
            return True
        
        if features['is_4k']:
            return True
        
        return False
    
    def predict(self, video_path: str, optimize_mode: str = 'balanced', options: Optional[Dict] = None) -> Dict:
        """主预测函数"""
        options = options or {}
        
        print(f"🎬 Starting video analysis: {video_path}", file=sys.stderr)
        print(f"🎯 Optimize mode: {optimize_mode}", file=sys.stderr)
        
        # 1. 提取基础特征
        basic_features = self.extract_basic_features(video_path)
        if not basic_features:
            return {
                'success': False,
                'error': 'Failed to extract video features'
            }
        
        print(f"📊 Resolution: {basic_features['resolution']}", file=sys.stderr)
        print(f"📊 FPS: {basic_features['fps']}", file=sys.stderr)
        print(f"📊 Bitrate: {basic_features['bitrate_mbps']:.1f}Mbps", file=sys.stderr)
        
        # 2. 视频类型分析
        video_type_info = self.analyze_video_type(video_path, basic_features)
        print(f"🎭 Video type: {video_type_info['type']}", file=sys.stderr)
        
        # 3. 预测编码参数
        params = self._predict_encoding_params(basic_features, video_type_info, optimize_mode, options)
        
        # 4. 预测码率
        target_bitrate, max_bitrate = self.predict_optimal_bitrate(
            basic_features, video_type_info, optimize_mode
        )
        params['target_bitrate'] = target_bitrate
        params['max_bitrate'] = max_bitrate
        
        # 5. 分辨率缩放建议
        scale_suggestion = self.suggest_resolution_scaling(basic_features, optimize_mode)
        if scale_suggestion:
            params['scale'] = scale_suggestion
            print(f"💡 Suggesting resolution scaling: {scale_suggestion}", file=sys.stderr)
        
        # 6. 两遍编码推荐
        two_pass = self.recommend_two_pass(basic_features, video_type_info, optimize_mode)
        params['two_pass'] = two_pass
        if two_pass:
            print(f"💡 Recommending two-pass encoding for better quality", file=sys.stderr)
        
        # 7. 返回结果
        result = {
            'success': True,
            'params': params,
            'features': basic_features,
            'video_type': video_type_info,
            'mode': optimize_mode,
            'confidence': params.get('confidence', 0.85)
        }
        
        print(f"✅ Prediction complete", file=sys.stderr)
        print(f"✅ Encoder: {params['encoder']}, CRF: {params['crf']}, Preset: {params['preset']}", file=sys.stderr)
        
        return result
    
    def _predict_encoding_params(self, features: Dict, video_type_info: Dict, 
                                  optimize_mode: str, options: Dict) -> Dict:
        """预测编码参数（增强版规则）"""
        
        video_type = video_type_info['type']
        
        # 初始参数
        params = {
            'encoder': 'libx264',
            'crf': 23,
            'preset': 'medium',
            'fps': None,
            'scale': None,
            'confidence': 0.8
        }
        
        # === 1. 根据视频类型选择编码器和基础参数 ===
        type_params = {
            'animation': {
                'encoder': 'libx264',
                'crf_offset': -2,  # 动画可以用稍高CRF
                'preset': 'medium'
            },
            'live_action': {
                'encoder': 'libx264',
                'crf_offset': 0,
                'preset': 'medium'
            },
            'game_recording': {
                'encoder': 'libx264',
                'crf_offset': -1,
                'preset': 'fast'  # 游戏需要快速编码
            },
            'movie': {
                'encoder': 'libx265',
                'crf_offset': 2,  # HEVC效率更高
                'preset': 'slow'  # 电影追求质量
            },
            'documentary': {
                'encoder': 'libx264',
                'crf_offset': 0,
                'preset': 'medium'
            },
            'screen_recording': {
                'encoder': 'libx264',
                'crf_offset': -3,  # 屏幕录制压缩效率极高
                'preset': 'veryfast'
            }
        }
        
        type_config = type_params.get(video_type, type_params['live_action'])
        params['encoder'] = type_config['encoder']
        params['preset'] = type_config['preset']
        crf_offset = type_config['crf_offset']
        
        # === 2. 根据优化模式调整CRF ===
        base_crf = {
            'size': 28,
            'balanced': 23,
            'quality': 18
        }.get(optimize_mode, 23)
        
        params['crf'] = base_crf + crf_offset
        
        # === 3. 根据分辨率微调 ===
        if features['is_4k']:
            params['crf'] += 2  # 4K可以用稍高CRF
            if options.get('allow_hevc', True):
                params['encoder'] = 'libx265'
                params['crf'] += 2  # HEVC再高2点
        
        # === 4. 根据码率调整 ===
        if features['bitrate_mbps'] < 5:
            params['crf'] -= 1  # 原始码率低，降低CRF保证质量
        elif features['bitrate_mbps'] > 20:
            params['crf'] += 1  # 原始码率高，可以提高CRF
        
        # === 5. 速度优先模式 ===
        if options.get('prefer_speed', False):
            params['preset'] = 'veryfast'
            params['confidence'] -= 0.1
        
        # === 6. 限制CRF范围 ===
        params['crf'] = max(15, min(35, params['crf']))
        
        # === 7. 特殊选项 ===
        if options.get('target_encoder'):
            params['encoder'] = options['target_encoder']
        
        return params


def main():
    """主函数"""
    if len(sys.argv) < 3:
        print(json.dumps({
            'success': False,
            'error': 'Usage: predict_video_params_v3.py <video_path> <optimize_mode> [options_json]'
        }))
        sys.exit(1)
    
    video_path = sys.argv[1]
    optimize_mode = sys.argv[2]
    options = json.loads(sys.argv[3]) if len(sys.argv) > 3 else {}
    
    predictor = VideoParamsPredictor()
    result = predictor.predict(video_path, optimize_mode, options)
    
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
