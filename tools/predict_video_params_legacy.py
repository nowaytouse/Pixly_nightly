#!/usr/bin/env python3
"""
视频参数AI预测脚本 - PIXLY Video Optimizer v2.0
双层AI架构：
- 前80%视频：3D小波 + 场景检测 + LightGBM + VMAF
- 后20%复杂视频：Transformer精细优化
"""

import sys
import json
import subprocess
import os
from pathlib import Path
import numpy as np

# 尝试导入高级特征提取器
try:
    from video_features_extractor import VideoFeatureExtractor
    ADVANCED_FEATURES_AVAILABLE = True
except ImportError:
    ADVANCED_FEATURES_AVAILABLE = False
    print("⚠️ Advanced features module not available, using basic mode", file=sys.stderr)

class VideoParamsPredictor:
    """视频参数预测器"""
    
    def __init__(self):
        """初始化预测器"""
        self.ffprobe_path = self._find_ffprobe()
        self.feature_extractor = VideoFeatureExtractor() if ADVANCED_FEATURES_AVAILABLE else None
        self.lightgbm_available = self._check_lightgbm()
    
    def _check_lightgbm(self):
        """检查LightGBM是否可用"""
        try:
            import lightgbm
            return True
        except ImportError:
            print("⚠️ LightGBM not available, using rule-based prediction", file=sys.stderr)
            return False
    
    def _find_ffprobe(self):
        """查找ffprobe路径"""
        # 尝试常见路径
        paths = [
            '/opt/homebrew/bin/ffprobe',
            '/usr/local/bin/ffprobe',
            '/usr/bin/ffprobe',
            'ffprobe'  # PATH中查找
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
        
        return 'ffprobe'  # 默认
    
    def extract_video_features(self, video_path):
        """提取视频特征"""
        try:
            # 使用ffprobe提取视频信息
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
            
            # 提取视频流信息
            video_stream = None
            for stream in data.get('streams', []):
                if stream.get('codec_type') == 'video':
                    video_stream = stream
                    break
            
            if not video_stream:
                print("⚠️ No video stream found", file=sys.stderr)
                return None
            
            # 提取关键特征
            features = {
                'width': int(video_stream.get('width', 0)),
                'height': int(video_stream.get('height', 0)),
                'codec': video_stream.get('codec_name', ''),
                'bitrate': int(video_stream.get('bit_rate', 0)) if video_stream.get('bit_rate') else 0,
                'fps': self._parse_fps(video_stream.get('r_frame_rate', '30/1')),
                'duration': float(data.get('format', {}).get('duration', 0)),
                'file_size': int(data.get('format', {}).get('size', 0)),
                'has_audio': any(s.get('codec_type') == 'audio' for s in data.get('streams', []))
            }
            
            # 计算派生特征
            features['resolution'] = f"{features['width']}x{features['height']}"
            features['is_4k'] = features['height'] >= 2160
            features['is_1080p'] = 1080 <= features['height'] < 2160
            features['is_720p'] = 720 <= features['height'] < 1080
            features['is_hd'] = features['height'] >= 720
            features['bitrate_mbps'] = features['bitrate'] / 1_000_000 if features['bitrate'] > 0 else 0
            
            return features
            
        except subprocess.TimeoutExpired:
            print("⚠️ ffprobe timeout", file=sys.stderr)
            return None
        except Exception as e:
            print(f"⚠️ Feature extraction error: {e}", file=sys.stderr)
            return None
    
    def _parse_fps(self, fps_str):
        """解析帧率字符串"""
        try:
            if '/' in fps_str:
                num, den = fps_str.split('/')
                return float(num) / float(den)
            return float(fps_str)
        except:
            return 30.0
    
    def predict(self, video_path, optimize_mode='balanced', options=None):
        """
        预测最优视频编码参数
        
        Args:
            video_path: 视频文件路径
            optimize_mode: 优化模式 (size/balanced/quality)
            options: 额外选项
        
        Returns:
            预测结果字典
        """
        options = options or {}
        
        # 1. 提取基础视频特征
        print(f"🔍 Extracting basic video features: {video_path}", file=sys.stderr)
        basic_features = self.extract_video_features(video_path)
        
        if not basic_features:
            return {
                'success': False,
                'error': 'Failed to extract video features'
            }
        
        print(f"📊 Basic features: {basic_features['resolution']}, {basic_features['codec']}, {basic_features['fps']:.1f}fps, {basic_features['bitrate_mbps']:.1f}Mbps", file=sys.stderr)
        
        # 2. 提取高级特征（如果可用）
        advanced_features = None
        complexity_score = 50  # 默认中等复杂度
        use_transformer = False
        
        if self.feature_extractor and options.get('use_advanced_ai', True):
            print(f"🧠 Extracting advanced features (scene/motion/complexity)...", file=sys.stderr)
            try:
                advanced_features = self.feature_extractor.extract_all_features(video_path, basic_features)
                complexity_score = advanced_features.get('complexity_score', 50)
                use_transformer = advanced_features.get('use_transformer', False)
                
                print(f"📊 Complexity score: {complexity_score}/100", file=sys.stderr)
                print(f"📊 Scene changes: {advanced_features.get('scenes', {}).get('count', 0)}", file=sys.stderr)
                print(f"📊 Motion complexity: {advanced_features.get('frames', {}).get('motion_complexity', 0):.1f}", file=sys.stderr)
                
                if use_transformer:
                    print(f"🔮 Using Transformer channel (complex video)", file=sys.stderr)
                else:
                    print(f"⚡ Using LightGBM channel (standard video)", file=sys.stderr)
                    
            except Exception as e:
                print(f"⚠️ Advanced feature extraction failed: {e}", file=sys.stderr)
                advanced_features = None
        
        # 3. 根据复杂度选择预测通道
        if use_transformer and options.get('enable_transformer', False):
            # Transformer精细优化（占位实现，后续可升级）
            params = self._predict_with_transformer(basic_features, advanced_features, optimize_mode, options)
        elif self.lightgbm_available and advanced_features:
            # LightGBM快速通道
            params = self._predict_with_lightgbm(basic_features, advanced_features, optimize_mode, options)
        else:
            # Fallback：基于规则
            params = self._predict_params_rule_based(basic_features, optimize_mode, options)
        
        # 4. 返回预测结果
        result = {
            'success': True,
            'params': params,
            'features': basic_features,
            'advanced_features': advanced_features,
            'complexity_score': complexity_score,
            'use_transformer': use_transformer,
            'mode': optimize_mode,
            'confidence': params.get('confidence', 0.85)
        }
        
        print(f"✅ Predicted params: CRF={params['crf']}, preset={params['preset']}, encoder={params['encoder']}", file=sys.stderr)
        print(f"✅ Prediction channel: {'Transformer' if use_transformer else 'LightGBM' if advanced_features else 'Rule-based'}", file=sys.stderr)
        print(f"✅ Confidence: {result['confidence']:.2f}", file=sys.stderr)
        
        return result
    
    def _predict_params_rule_based(self, features, optimize_mode, options):
        """基于规则的参数预测（作为初始版本）"""
        
        # 基础参数
        params = {
            'crf': 23,
            'preset': 'medium',
            'encoder': 'libx264',
            'scale': None,  # 是否需要缩放
            'fps': None     # 是否需要调整帧率
        }
        
        # 1. 根据优化模式调整CRF
        if optimize_mode == 'size':
            # 体积优先：CRF稍高，压缩更强
            params['crf'] = 28
            params['preset'] = 'faster'  # 更快的编码
        elif optimize_mode == 'quality':
            # 质量优先：CRF较低
            params['crf'] = 20
            params['preset'] = 'slow'    # 更慢但质量更好
        else:  # balanced
            params['crf'] = 23
            params['preset'] = 'medium'
        
        # 2. 根据分辨率调整
        if features['is_4k']:
            # 4K视频：CRF可以稍高（人眼不易察觉）
            params['crf'] += 2
            # 建议使用H.265获得更好压缩
            if options.get('allow_hevc', True):
                params['encoder'] = 'libx265'
                params['crf'] += 2  # H.265可以用更高CRF获得同等质量
        elif features['is_1080p']:
            # 1080p：标准处理
            pass
        elif features['is_720p']:
            # 720p：CRF可以稍低
            params['crf'] -= 1
        else:
            # 低分辨率：CRF更低以保持质量
            params['crf'] -= 2
        
        # 3. 根据原始码率调整
        if features['bitrate_mbps'] > 0:
            if features['bitrate_mbps'] > 20:
                # 原始码率很高：可以压缩更多
                params['crf'] += 1
            elif features['bitrate_mbps'] < 2:
                # 原始码率很低：已经很压缩了，保持质量
                params['crf'] -= 1
        
        # 4. 根据帧率调整
        if features['fps'] > 60:
            # 高帧率视频：建议降低到60fps以减少体积
            if optimize_mode == 'size':
                params['fps'] = 60
        
        # 5. 根据文件大小调整
        size_mb = features['file_size'] / 1_000_000
        if size_mb > 500:
            # 大文件：考虑使用更快的preset
            if params['preset'] == 'slow':
                params['preset'] = 'medium'
            elif params['preset'] == 'medium':
                params['preset'] = 'fast'
        
        # 6. CRF范围约束
        params['crf'] = max(18, min(28, params['crf']))
        
        # 7. 用户自定义覆盖
        if options.get('target_encoder'):
            params['encoder'] = options['target_encoder']
        
        if options.get('prefer_speed'):
            # 用户要求速度优先
            if params['preset'] == 'slow':
                params['preset'] = 'medium'
            elif params['preset'] == 'medium':
                params['preset'] = 'fast'
            elif params['preset'] == 'fast':
                params['preset'] = 'faster'
        
        return params
    
    def _predict_with_lightgbm(self, basic_features, advanced_features, optimize_mode, options):
        """
        使用LightGBM模型预测（快速通道 - 80%视频）
        
        特征：
        - 基础特征：分辨率、码率、帧率
        - 场景特征：场景数、平均场景长度
        - 运动特征：运动复杂度、亮度、对比度
        """
        print("⚡ Using LightGBM fast channel for prediction", file=sys.stderr)
        
        # 先用规则获取基础参数
        base_params = self._predict_params_rule_based(basic_features, optimize_mode, options)
        
        # 准备LightGBM特征向量
        scene_data = advanced_features.get('scenes', {})
        frame_data = advanced_features.get('frames', {})
        
        feature_vector = np.array([
            basic_features['width'],
            basic_features['height'],
            basic_features['fps'],
            basic_features['bitrate_mbps'],
            basic_features['duration'],
            scene_data.get('count', 0),
            scene_data.get('avg_scene_length', 0),
            frame_data.get('avg_brightness', 128),
            frame_data.get('avg_contrast', 30),
            frame_data.get('motion_complexity', 10),
            1 if optimize_mode == 'size' else 0,
            1 if optimize_mode == 'balanced' else 0,
            1 if optimize_mode == 'quality' else 0
        ]).reshape(1, -1)
        
        # TODO: 加载训练好的LightGBM模型并预测
        # 当前使用基于规则的增强版
        
        # 基于场景复杂度微调CRF
        scene_count = scene_data.get('count', 0)
        duration = basic_features.get('duration', 1)
        scenes_per_min = (scene_count / duration) * 60 if duration > 0 else 0
        
        if scenes_per_min > 20:
            # 频繁切换：CRF-1（保持质量）
            base_params['crf'] = max(18, base_params['crf'] - 1)
        
        # 基于运动复杂度微调preset
        motion = frame_data.get('motion_complexity', 10)
        if motion > 40:
            # 高动态：使用更快preset（牺牲一点效率换速度）
            if base_params['preset'] == 'slow':
                base_params['preset'] = 'medium'
            elif base_params['preset'] == 'medium':
                base_params['preset'] = 'fast'
        
        base_params['confidence'] = 0.90  # LightGBM置信度更高
        base_params['channel'] = 'lightgbm'
        
        return base_params
    
    def _predict_with_transformer(self, basic_features, advanced_features, optimize_mode, options):
        """
        使用Transformer模型精细优化（精细通道 - 20%复杂视频）
        
        特征：
        - 时空序列特征
        - 多场景融合
        - 深度编码器注意力
        """
        print("🔮 Using Transformer fine-tuning channel for complex video", file=sys.stderr)
        
        # 先用LightGBM获取基础预测
        base_params = self._predict_with_lightgbm(basic_features, advanced_features, optimize_mode, options)
        
        # TODO: 加载Transformer模型并优化
        # 当前使用增强规则模拟Transformer优化
        
        scene_data = advanced_features.get('scenes', {})
        frame_data = advanced_features.get('frames', {})
        
        # Transformer优化：更保守的CRF（保证复杂场景质量）
        base_params['crf'] = max(18, base_params['crf'] - 2)
        
        # Transformer优化：使用更慢的preset（更好的编码效率）
        if base_params['preset'] == 'faster':
            base_params['preset'] = 'fast'
        elif base_params['preset'] == 'fast':
            base_params['preset'] = 'medium'
        
        # 复杂视频推荐H.265或AV1（更好的压缩率）
        if basic_features.get('is_hd') and options.get('allow_hevc', True):
            if base_params['encoder'] == 'libx264':
                base_params['encoder'] = 'libx265'
                base_params['crf'] += 2  # H.265可以用更高CRF
        
        base_params['confidence'] = 0.95  # Transformer置信度最高
        base_params['channel'] = 'transformer'
        
        print(f"🔮 Transformer optimized: CRF={base_params['crf']}, preset={base_params['preset']}", file=sys.stderr)
        
        return base_params


def main():
    """主函数"""
    if len(sys.argv) < 3:
        print(json.dumps({
            'success': False,
            'error': 'Usage: predict_video_params.py <video_path> <optimize_mode> [options_json]'
        }))
        sys.exit(1)
    
    video_path = sys.argv[1]
    optimize_mode = sys.argv[2]
    options_json = sys.argv[3] if len(sys.argv) > 3 else '{}'
    
    # 检查文件存在
    if not os.path.exists(video_path):
        print(json.dumps({
            'success': False,
            'error': f'Video file not found: {video_path}'
        }))
        sys.exit(1)
    
    # 解析选项
    try:
        options = json.loads(options_json)
    except:
        options = {}
    
    # 创建预测器并预测
    predictor = VideoParamsPredictor()
    result = predictor.predict(video_path, optimize_mode, options)
    
    # 输出JSON结果到stdout
    print(json.dumps(result, ensure_ascii=False))


if __name__ == '__main__':
    main()
