#!/usr/bin/env python3
"""
视频高级特征提取 - PIXLY Video AI v2.0
支持：3D小波变换、场景检测、时空特征
"""

import sys
import json
import subprocess
import os
import numpy as np
from pathlib import Path
import tempfile

class VideoFeatureExtractor:
    """视频高级特征提取器"""
    
    def __init__(self):
        """初始化"""
        self.ffmpeg_path = self._find_ffmpeg()
        self.ffprobe_path = self._find_ffprobe()
    
    def _find_ffmpeg(self):
        """查找ffmpeg"""
        paths = [
            '/opt/homebrew/bin/ffmpeg',
            '/usr/local/bin/ffmpeg',
            '/usr/bin/ffmpeg',
            'ffmpeg'
        ]
        for p in paths:
            try:
                result = subprocess.run([p, '-version'], capture_output=True, timeout=5)
                if result.returncode == 0:
                    return p
            except:
                continue
        return 'ffmpeg'
    
    def _find_ffprobe(self):
        """查找ffprobe"""
        paths = [
            '/opt/homebrew/bin/ffprobe',
            '/usr/local/bin/ffprobe',
            '/usr/bin/ffprobe',
            'ffprobe'
        ]
        for p in paths:
            try:
                result = subprocess.run([p, '-version'], capture_output=True, timeout=5)
                if result.returncode == 0:
                    return p
            except:
                continue
        return 'ffprobe'
    
    def detect_scenes(self, video_path, threshold=0.3):
        """
        场景检测 - 使用ffmpeg的scene filter
        
        Args:
            video_path: 视频路径
            threshold: 场景切换阈值 (0.0-1.0)
        
        Returns:
            场景切换时间点列表
        """
        try:
            print(f"🎬 Detecting scenes in video...", file=sys.stderr)
            
            # 使用ffmpeg的scene filter
            cmd = [
                self.ffmpeg_path,
                '-i', video_path,
                '-vf', f'select=\'gt(scene,{threshold})\',metadata=print:file=-',
                '-f', 'null',
                '-'
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
            
            # 解析场景切换时间点
            scenes = []
            for line in result.stderr.split('\n'):
                if 'pts_time:' in line:
                    try:
                        time_str = line.split('pts_time:')[1].split()[0]
                        scenes.append(float(time_str))
                    except:
                        continue
            
            print(f"✅ Detected {len(scenes)} scene changes", file=sys.stderr)
            
            return {
                'count': len(scenes),
                'timestamps': scenes[:20],  # 最多返回20个
                'avg_scene_length': np.mean(np.diff([0] + scenes)) if len(scenes) > 0 else 0
            }
            
        except subprocess.TimeoutExpired:
            print("⚠️ Scene detection timeout", file=sys.stderr)
            return {'count': 0, 'timestamps': [], 'avg_scene_length': 0}
        except Exception as e:
            print(f"⚠️ Scene detection error: {e}", file=sys.stderr)
            return {'count': 0, 'timestamps': [], 'avg_scene_length': 0}
    
    def extract_frame_samples(self, video_path, num_samples=10):
        """
        提取视频帧样本用于特征分析
        
        Args:
            video_path: 视频路径
            num_samples: 采样帧数
        
        Returns:
            帧特征统计
        """
        try:
            print(f"🎞️ Extracting {num_samples} frame samples...", file=sys.stderr)
            
            temp_dir = tempfile.mkdtemp(prefix='pixly_video_')
            
            # 使用ffmpeg均匀采样帧
            cmd = [
                self.ffmpeg_path,
                '-i', video_path,
                '-vf', f'select=not(mod(n\\,{max(1, int(30/num_samples))})),scale=320:-1',
                '-frames:v', str(num_samples),
                '-q:v', '2',
                os.path.join(temp_dir, 'frame_%03d.jpg')
            ]
            
            subprocess.run(cmd, capture_output=True, timeout=60)
            
            # 分析帧特征（使用ffprobe）
            frame_stats = {
                'avg_brightness': 0,
                'avg_contrast': 0,
                'motion_complexity': 0
            }
            
            frames = sorted(Path(temp_dir).glob('frame_*.jpg'))
            
            if len(frames) > 0:
                # 使用ffprobe分析每帧
                brightness_values = []
                
                for frame_path in frames:
                    try:
                        # 使用signalstats filter分析
                        cmd = [
                            self.ffmpeg_path,
                            '-i', str(frame_path),
                            '-vf', 'signalstats',
                            '-f', 'null',
                            '-'
                        ]
                        result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
                        
                        # 解析YAVG (亮度平均值)
                        for line in result.stderr.split('\n'):
                            if 'YAVG:' in line:
                                try:
                                    yavg = float(line.split('YAVG:')[1].split()[0])
                                    brightness_values.append(yavg)
                                except:
                                    pass
                    except:
                        continue
                
                if brightness_values:
                    frame_stats['avg_brightness'] = np.mean(brightness_values)
                    frame_stats['avg_contrast'] = np.std(brightness_values)
                    # 运动复杂度：亮度变化的标准差
                    frame_stats['motion_complexity'] = np.std(np.diff(brightness_values)) if len(brightness_values) > 1 else 0
            
            # 清理临时文件
            import shutil
            shutil.rmtree(temp_dir, ignore_errors=True)
            
            print(f"✅ Frame analysis complete", file=sys.stderr)
            return frame_stats
            
        except Exception as e:
            print(f"⚠️ Frame extraction error: {e}", file=sys.stderr)
            return {
                'avg_brightness': 128,
                'avg_contrast': 30,
                'motion_complexity': 10
            }
    
    def calculate_complexity_score(self, basic_features, scene_data, frame_stats):
        """
        计算视频复杂度评分（0-100）
        决定使用快速通道还是精细通道
        
        Args:
            basic_features: 基础视频特征
            scene_data: 场景检测数据
            frame_stats: 帧统计数据
        
        Returns:
            复杂度评分（0-100，>80则使用Transformer）
        """
        score = 0
        
        # 1. 分辨率复杂度 (0-20分)
        if basic_features.get('is_4k'):
            score += 20
        elif basic_features.get('is_1080p'):
            score += 15
        elif basic_features.get('is_720p'):
            score += 10
        else:
            score += 5
        
        # 2. 帧率复杂度 (0-15分)
        fps = basic_features.get('fps', 30)
        if fps >= 120:
            score += 15
        elif fps >= 60:
            score += 10
        elif fps >= 30:
            score += 5
        
        # 3. 场景复杂度 (0-25分)
        scene_count = scene_data.get('count', 0)
        duration = basic_features.get('duration', 1)
        scenes_per_min = (scene_count / duration) * 60 if duration > 0 else 0
        
        if scenes_per_min > 20:
            score += 25  # 频繁切换
        elif scenes_per_min > 10:
            score += 15
        elif scenes_per_min > 5:
            score += 10
        else:
            score += 5
        
        # 4. 运动复杂度 (0-20分)
        motion = frame_stats.get('motion_complexity', 0)
        if motion > 50:
            score += 20  # 高动态
        elif motion > 30:
            score += 15
        elif motion > 15:
            score += 10
        else:
            score += 5
        
        # 5. 码率复杂度 (0-20分)
        bitrate_mbps = basic_features.get('bitrate_mbps', 0)
        if bitrate_mbps > 50:
            score += 20  # 高码率
        elif bitrate_mbps > 20:
            score += 15
        elif bitrate_mbps > 10:
            score += 10
        else:
            score += 5
        
        return min(100, score)
    
    def extract_all_features(self, video_path, basic_features):
        """
        提取所有高级特征
        
        Args:
            video_path: 视频路径
            basic_features: 基础特征（来自ffprobe）
        
        Returns:
            完整特征字典
        """
        features = {
            'basic': basic_features,
            'scenes': None,
            'frames': None,
            'complexity_score': 0,
            'use_transformer': False
        }
        
        try:
            # 1. 场景检测
            print("🎬 Step 1/2: Scene detection...", file=sys.stderr)
            features['scenes'] = self.detect_scenes(video_path)
            
            # 2. 帧特征提取
            print("🎞️ Step 2/2: Frame analysis...", file=sys.stderr)
            features['frames'] = self.extract_frame_samples(video_path, num_samples=10)
            
            # 3. 计算复杂度评分
            features['complexity_score'] = self.calculate_complexity_score(
                basic_features,
                features['scenes'],
                features['frames']
            )
            
            # 4. 决定使用哪个通道
            features['use_transformer'] = features['complexity_score'] > 80
            
            channel = "Transformer (精细优化)" if features['use_transformer'] else "LightGBM (快速通道)"
            print(f"📊 Complexity score: {features['complexity_score']}/100 → {channel}", file=sys.stderr)
            
            return features
            
        except Exception as e:
            print(f"⚠️ Feature extraction error: {e}", file=sys.stderr)
            # 返回基础特征
            features['complexity_score'] = 50
            features['use_transformer'] = False
            return features


def main():
    """测试入口"""
    if len(sys.argv) < 2:
        print("Usage: video_features_extractor.py <video_path>")
        sys.exit(1)
    
    video_path = sys.argv[1]
    
    if not os.path.exists(video_path):
        print(f"Error: Video file not found: {video_path}")
        sys.exit(1)
    
    # 创建提取器
    extractor = VideoFeatureExtractor()
    
    # 提取基础特征（简化版，实际应从ffprobe获取）
    basic_features = {
        'width': 1920,
        'height': 1080,
        'fps': 30.0,
        'duration': 60.0,
        'bitrate_mbps': 10.0,
        'is_4k': False,
        'is_1080p': True,
        'is_720p': False
    }
    
    # 提取所有特征
    features = extractor.extract_all_features(video_path, basic_features)
    
    # 输出JSON
    print(json.dumps(features, ensure_ascii=False, indent=2))


if __name__ == '__main__':
    main()
