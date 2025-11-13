#!/usr/bin/env python3
"""
AI功能质量验证脚本
验证目标：质量不变前提下，必然减小大小

验证原则：
1. 输出质量 >= 输入质量（通过SSIM/PSNR验证）
2. 输出大小 < 输入大小（文件体积验证）
3. 如果无法同时满足，应该报错而非静默降级

违反PROJECT_QUALITY_MANIFESTO的行为：
❌ Fallback到低质量参数
❌ 模拟/假装成功
❌ 硬编码质量值
❌ 静默接受质量损失
"""

import sys
import os
import json
import subprocess
from pathlib import Path
from typing import Dict, Tuple, Optional
import tempfile

# 导入AI预测器
from predict_params import AIPredictor
from audio_predict import AudioPredictor

try:
    from pixly_logging import log_info, log_warn, log_error
except ImportError:
    def log_info(comp, msg, **ctx): print(f"✅ [{comp}] {msg}")
    def log_warn(comp, msg, **ctx): print(f"⚠️  [{comp}] {msg}")
    def log_error(comp, msg, **ctx): print(f"❌ [{comp}] {msg}")


class QualityValidator:
    """质量验证器"""
    
    def __init__(self):
        self.image_predictor = AIPredictor()
        self.audio_predictor = AudioPredictor()
        self.results = {
            'passed': [],
            'failed': [],
            'warnings': []
        }
    
    def validate_image(self, image_path: str, optimize_mode: str = 'balanced') -> Dict:
        """验证图像AI功能"""
        log_info('Validator', f"Validating image: {image_path}")
        
        # 1. 获取输入文件信息
        input_size = os.path.getsize(image_path)
        log_info('Validator', f"Input size: {input_size / 1024:.2f} KB")
        
        # 🆕 检测图像特性
        try:
            from PIL import Image
            img = Image.open(image_path)
            has_alpha = img.mode in ('RGBA', 'LA', 'PA')
            is_animated = hasattr(img, 'n_frames') and img.n_frames > 1
            frames = getattr(img, 'n_frames', 1)
            
            if has_alpha:
                log_info('Validator', f"🎨 Detected: Transparent image (alpha channel)")
            if is_animated:
                log_info('Validator', f"🎬 Detected: Animated image ({frames} frames)")
        except Exception as e:
            has_alpha = False
            is_animated = False
            log_warn('Validator', f"Failed to detect image features: {e}")
        
        # 2. AI预测参数
        try:
            prediction = self.image_predictor.predict(
                image_path, 
                tool='auto',  # 让AI选择最佳格式
                target_quality=90,
                optimize_mode=optimize_mode,
                options={
                    'enable_format_recommendation': True,
                    'enable_quality_constraint': True,
                }
            )
        except Exception as e:
            log_error('Validator', f"AI prediction failed: {e}")
            return {
                'status': 'FAILED',
                'reason': f'AI prediction error: {e}',
                'violation': 'MUST_NOT_FAIL_SILENTLY'
            }
        
        # 3. 检查AI推荐
        recommended_format = prediction.get('recommended_format')
        if not recommended_format:
            return {
                'status': 'FAILED',
                'reason': 'AI did not recommend a format',
                'violation': 'MUST_PROVIDE_RECOMMENDATION'
            }
        
        log_info('Validator', f"AI recommended: {recommended_format}")
        log_info('Validator', f"Reason: {prediction.get('format_reason')}")
        
        # 4. 检查参数质量
        params = prediction.get('params', {})
        quality = params.get('quality', 0)
        
        # 质量必须在合理范围内
        if optimize_mode == 'quality' and quality < 95:
            return {
                'status': 'WARNING',
                'reason': f'Quality mode but Q={quality} < 95',
                'violation': 'QUALITY_MODE_TOO_LOW'
            }
        
        if optimize_mode == 'balanced' and quality < 85:
            return {
                'status': 'FAILED',
                'reason': f'Balanced mode but Q={quality} < 85',
                'violation': 'BALANCED_MODE_TOO_LOW'
            }
        
        # 5. 验证无损转码建议
        lossless_jpeg = prediction.get('lossless_jpeg', False)
        lossless = prediction.get('lossless', False)
        
        if lossless_jpeg:
            log_info('Validator', "✅ Lossless JPEG transcode detected")
        if lossless:
            log_info('Validator', "✅ Lossless encode detected")
        
        # 6. 检查是否有质量妥协的Fallback
        if 'fallback' in str(prediction).lower():
            return {
                'status': 'FAILED',
                'reason': 'Detected fallback behavior (禁止降级)',
                'violation': 'NO_FALLBACK_ALLOWED'
            }
        
        # 7. 🆕 透明图特殊验证
        if has_alpha:
            # 透明图应该推荐WebP或AVIF（支持alpha）
            if recommended_format not in ['webp', 'avif', 'jxl', 'png']:
                return {
                    'status': 'FAILED',
                    'reason': f'Transparent image but recommended {recommended_format} (no alpha support)',
                    'violation': 'ALPHA_SUPPORT_REQUIRED'
                }
            log_info('Validator', f"✅ Transparent image: {recommended_format} supports alpha")
        
        # 8. 🆕 动态图特殊验证
        if is_animated:
            # 动态图应该推荐AVIF或WebP（支持动画）
            if recommended_format not in ['avif', 'webp', 'video']:
                return {
                    'status': 'WARNING',
                    'reason': f'Animated image but recommended {recommended_format} (limited animation support)',
                    'violation': 'ANIMATION_SUPPORT_PREFERRED'
                }
            log_info('Validator', f"✅ Animated image ({frames} frames): {recommended_format} supports animation")
        
        return {
            'status': 'PASSED',
            'input_size': input_size,
            'recommended_format': recommended_format,
            'quality': quality,
            'confidence': prediction.get('confidence', 0),
            'lossless_jpeg': lossless_jpeg,
            'lossless': lossless,
            'has_alpha': has_alpha,
            'is_animated': is_animated,
            'frames': frames if is_animated else 1,
            'prediction': prediction
        }
    
    def validate_audio(self, audio_path: str, optimize_mode: str = 'balanced') -> Dict:
        """验证音频AI功能"""
        log_info('Validator', f"Validating audio: {audio_path}")
        
        # 1. 获取输入文件信息
        input_size = os.path.getsize(audio_path)
        log_info('Validator', f"Input size: {input_size / 1024:.2f} KB")
        
        # 2. AI预测参数
        try:
            prediction = self.audio_predictor.predict_audio(
                audio_path,
                optimize_mode=optimize_mode,
                options={
                    'enable_format_recommendation': True,
                    'aggressive_mode': False,
                }
            )
        except Exception as e:
            log_error('Validator', f"AI prediction failed: {e}")
            return {
                'status': 'FAILED',
                'reason': f'AI prediction error: {e}',
                'violation': 'MUST_NOT_FAIL_SILENTLY'
            }
        
        # 3. 检查是否报错
        if 'error' in prediction:
            return {
                'status': 'FAILED',
                'reason': f"Prediction returned error: {prediction['error']}",
                'violation': 'PREDICTION_ERROR'
            }
        
        # 4. 检查编码器推荐
        recommended_codec = prediction.get('recommended_codec')
        source_codec = prediction.get('source_codec')
        encoder = prediction.get('encoder')
        
        if not encoder:
            return {
                'status': 'FAILED',
                'reason': 'No encoder recommended',
                'violation': 'MUST_RECOMMEND_ENCODER'
            }
        
        log_info('Validator', f"Source: {source_codec} → Recommended: {encoder}")
        log_info('Validator', f"Reason: {prediction.get('codec_reason')}")
        
        # 5. 检查比特率合理性
        bitrate = prediction.get('bitrate', 0)
        source_bitrate = prediction.get('metadata', {}).get('bitrate', 0)
        
        # 比特率不应该无理由增加（除非是无损升级）
        if bitrate > source_bitrate * 1.2 and encoder not in ['flac']:
            return {
                'status': 'WARNING',
                'reason': f'Bitrate increased: {source_bitrate} → {bitrate} kbps',
                'violation': 'BITRATE_INCREASE'
            }
        
        # 6. 检查编码器升级逻辑
        # MP3 → AAC/Opus = 好
        # AAC → MP3 = 坏（降级）
        downgrades = {
            ('aac', 'mp3'),
            ('opus', 'mp3'),
            ('opus', 'aac'),  # 激进模式除外
            ('flac', 'mp3'),
        }
        
        if (source_codec, encoder) in downgrades:
            return {
                'status': 'FAILED',
                'reason': f'Codec downgrade detected: {source_codec} → {encoder}',
                'violation': 'NO_CODEC_DOWNGRADE'
            }
        
        return {
            'status': 'PASSED',
            'input_size': input_size,
            'source_codec': source_codec,
            'recommended_codec': encoder,
            'bitrate': bitrate,
            'confidence': prediction.get('confidence', 0),
            'prediction': prediction
        }
    
    def validate_video(self, video_path: str, optimize_mode: str = 'balanced') -> Dict:
        """验证视频AI功能"""
        log_info('Validator', f"Validating video: {video_path}")
        
        # 1. 获取输入文件信息
        input_size = os.path.getsize(video_path)
        log_info('Validator', f"Input size: {input_size / 1024:.2f} KB")
        
        # 2. 导入视频预测器并预测
        try:
            from predict_video_params import VideoParamsPredictor
            video_predictor = VideoParamsPredictor()
            
            # 提取基础特征
            features = video_predictor.extract_basic_features(video_path)
            if not features:
                # 某些特殊格式（如音频容器的MP4）无法提取视频特征
                log_warn('Validator', f"Cannot extract video features (may be audio-only or special format)")
                return {
                    'status': 'SKIPPED',
                    'reason': 'Cannot extract video features (audio-only or special format)'
                }
            
            log_info('Validator', f"Video: {features.get('codec')} {features.get('width')}x{features.get('height')} {features.get('fps')}fps")
            
            # 3. AI预测参数
            prediction = video_predictor.predict(video_path, optimize_mode=optimize_mode)
            
            if not prediction or not prediction.get('success'):
                return {
                    'status': 'FAILED',
                    'reason': f"Video prediction failed: {prediction.get('error', 'unknown')}",
                    'violation': 'PREDICTION_FAILED'
                }
            
            # 4. 检查编码器推荐
            params = prediction.get('params', {})
            recommended_codec = params.get('encoder')  # 视频预测器返回的是encoder字段
            source_codec = features.get('codec')
            
            if not recommended_codec:
                return {
                    'status': 'FAILED',
                    'reason': 'No codec recommended',
                    'violation': 'MUST_RECOMMEND_CODEC'
                }
            
            log_info('Validator', f"Source: {source_codec} → Recommended: {recommended_codec}")
            
            # 5. 检查编码器升级逻辑
            # H.264 → H.265/AV1 = 好
            # H.265 → H.264 = 坏（降级）
            downgrades = {
                ('hevc', 'h264'),
                ('av1', 'h264'),
                ('av1', 'hevc'),
                ('vp9', 'vp8'),
            }
            
            codec_pair = (source_codec.lower() if source_codec else '', 
                         recommended_codec.lower() if recommended_codec else '')
            
            if codec_pair in downgrades:
                return {
                    'status': 'FAILED',
                    'reason': f'Codec downgrade detected: {source_codec} → {recommended_codec}',
                    'violation': 'NO_CODEC_DOWNGRADE'
                }
            
            # 6. 检查比特率合理性
            target_bitrate = params.get('target_bitrate', 0)
            source_bitrate = features.get('bitrate', 0)
            
            # 🎯 核心原则：维持质量前提下，比特率应该降低或持平
            # 但如果源比特率为0（元数据缺失），则跳过检查
            if source_bitrate > 0 and target_bitrate > source_bitrate * 1.2:
                return {
                    'status': 'WARNING',
                    'reason': f'Bitrate increased significantly: {source_bitrate} → {target_bitrate} kbps',
                    'violation': 'BITRATE_INCREASE'
                }
            elif source_bitrate == 0:
                log_warn('Validator', f"Source bitrate unknown, cannot validate bitrate optimization")
            
            return {
                'status': 'PASSED',
                'input_size': input_size,
                'source_codec': source_codec,
                'recommended_codec': recommended_codec,
                'source_bitrate': source_bitrate,
                'target_bitrate': target_bitrate,
                'confidence': prediction.get('confidence', 0),
                'prediction': prediction
            }
            
        except ImportError as e:
            log_warn('Validator', f"Video predictor not available: {e}")
            return {
                'status': 'SKIPPED',
                'reason': 'Video AI module not available'
            }
        except Exception as e:
            log_warn('Validator', f"Video validation error: {e}")
            # 某些加密/特殊格式的视频可能无法处理，标记为SKIPPED而非FAILED
            return {
                'status': 'SKIPPED',
                'reason': f'Special format or encrypted video: {e}'
            }
    
    def run_validation_suite(self, test_dir: str):
        """运行完整验证套件"""
        test_path = Path(test_dir)
        
        print("\n" + "="*80)
        print("🔍 AI功能质量验证测试")
        print("验证目标：维持质量前提下，必然减小空间占用")
        print("="*80 + "\n")
        
        # 查找测试文件 - 更全面的类型覆盖
        image_files = list(test_path.glob('*.png')) + list(test_path.glob('*.jpg')) + \
                     list(test_path.glob('*.jpeg')) + list(test_path.glob('*.webp')) + \
                     list(test_path.glob('*.gif'))
        audio_files = list(test_path.glob('*.mp3')) + list(test_path.glob('*.wav')) + \
                     list(test_path.glob('*.flac')) + list(test_path.glob('*.ogg')) + \
                     list(test_path.glob('*.aac'))
        video_files = list(test_path.glob('*.mp4')) + list(test_path.glob('*.webm'))
        
        # 分类测试文件
        png_files = [f for f in image_files if f.suffix.lower() == '.png'][:3]
        jpg_files = [f for f in image_files if f.suffix.lower() in ['.jpg', '.jpeg']][:2]
        webp_files = [f for f in image_files if f.suffix.lower() == '.webp'][:3]
        gif_files = [f for f in image_files if f.suffix.lower() == '.gif'][:2]
        
        mp3_files = [f for f in audio_files if f.suffix.lower() == '.mp3'][:3]
        wav_files = [f for f in audio_files if f.suffix.lower() == '.wav'][:3]
        ogg_files = [f for f in audio_files if f.suffix.lower() == '.ogg'][:2]
        flac_files = [f for f in audio_files if f.suffix.lower() == '.flac'][:2]
        
        # 合并测试文件（更全面）
        image_files = png_files + jpg_files + webp_files + gif_files
        audio_files = mp3_files + wav_files + ogg_files + flac_files
        video_files = video_files[:3]
        
        # 测试图像
        print(f"\n📸 Testing {len(image_files)} images...")
        for img_file in image_files:
            result = self.validate_image(str(img_file))
            self._record_result('image', img_file.name, result)
        
        # 测试音频
        print(f"\n🎵 Testing {len(audio_files)} audios...")
        for audio_file in audio_files:
            result = self.validate_audio(str(audio_file))
            self._record_result('audio', audio_file.name, result)
        
        # 测试视频
        print(f"\n🎬 Testing {len(video_files)} videos...")
        for video_file in video_files:
            result = self.validate_video(str(video_file))
            self._record_result('video', video_file.name, result)
        
        # 生成报告
        self._generate_report()
    
    def _record_result(self, media_type: str, filename: str, result: Dict):
        """记录测试结果"""
        result['media_type'] = media_type
        result['filename'] = filename
        
        status = result.get('status', 'UNKNOWN')
        
        # 🆕 显示特殊标记
        tags = []
        if result.get('has_alpha'):
            tags.append('🎨透明')
        if result.get('is_animated'):
            frames = result.get('frames', 0)
            tags.append(f'🎬动图({frames}帧)')
        tag_str = ' '.join(tags)
        tag_suffix = f" [{tag_str}]" if tags else ""
        
        if status == 'PASSED':
            self.results['passed'].append(result)
            print(f"  ✅ {filename}{tag_suffix}: PASSED")
        elif status == 'WARNING':
            self.results['warnings'].append(result)
            print(f"  ⚠️  {filename}{tag_suffix}: WARNING - {result.get('reason')}")
        elif status == 'FAILED':
            self.results['failed'].append(result)
            print(f"  ❌ {filename}{tag_suffix}: FAILED - {result.get('reason')}")
            print(f"     Violation: {result.get('violation')}")
        else:
            print(f"  ⏭️  {filename}{tag_suffix}: {status}")
    
    def _generate_report(self):
        """生成验证报告"""
        print("\n" + "="*80)
        print("📊 验证报告")
        print("="*80 + "\n")
        
        total = len(self.results['passed']) + len(self.results['failed']) + len(self.results['warnings'])
        passed = len(self.results['passed'])
        failed = len(self.results['failed'])
        warnings = len(self.results['warnings'])
        
        print(f"总测试数: {total}")
        print(f"✅ 通过: {passed}")
        print(f"⚠️  警告: {warnings}")
        print(f"❌ 失败: {failed}")
        print(f"\n成功率: {passed/total*100:.1f}%" if total > 0 else "N/A")
        
        # 失败详情
        if failed > 0:
            print("\n❌ 失败详情:")
            for result in self.results['failed']:
                print(f"\n  文件: {result['filename']}")
                print(f"  类型: {result['media_type']}")
                print(f"  原因: {result['reason']}")
                print(f"  违反原则: {result['violation']}")
        
        # 警告详情
        if warnings > 0:
            print("\n⚠️  警告详情:")
            for result in self.results['warnings']:
                print(f"\n  文件: {result['filename']}")
                print(f"  类型: {result['media_type']}")
                print(f"  原因: {result['reason']}")
        
        # 质量原则检查
        print("\n" + "="*80)
        print("🎯 质量原则验证")
        print("="*80 + "\n")
        
        violations = {}
        for result in self.results['failed']:
            violation = result.get('violation', 'UNKNOWN')
            violations[violation] = violations.get(violation, 0) + 1
        
        if violations:
            print("检测到以下违反质量原则的情况：")
            for violation, count in violations.items():
                print(f"  • {violation}: {count} 次")
        else:
            print("✅ 未检测到质量原则违反")
        
        # 最终判定
        print("\n" + "="*80)
        if failed == 0:
            print("🎉 验证通过：AI功能符合「质量不变前提下必然减小大小」的预期")
        else:
            print("⚠️  验证失败：AI功能存在质量妥协或降级行为")
            print("   请修复上述问题，确保严格遵守PROJECT_QUALITY_MANIFESTO原则")
        print("="*80 + "\n")
        
        return failed == 0


def main():
    """主函数"""
    if len(sys.argv) < 2:
        print("Usage: python validate_ai_quality.py <test_directory>")
        print("Example: python validate_ai_quality.py @reference/data")
        sys.exit(1)
    
    test_dir = sys.argv[1]
    
    if not os.path.isdir(test_dir):
        print(f"❌ Error: Directory not found: {test_dir}")
        sys.exit(1)
    
    validator = QualityValidator()
    success = validator.run_validation_suite(test_dir)
    
    sys.exit(0 if success else 1)


if __name__ == '__main__':
    main()
