#!/usr/bin/env python3
"""
PPO训练脚本 - 纯FFmpeg方案
使用FFmpeg处理所有媒体类型（图像、视频、音频）
"""

import os
import sys
import json
import subprocess
from pathlib import Path
from datetime import datetime
import random
import shutil

def check_ffmpeg():
    """检查FFmpeg是否可用"""
    if not shutil.which('ffmpeg'):
        print("❌ FFmpeg not installed")
        return False
    return True

def find_media_files(data_dir):
    """查找所有媒体文件"""
    media_files = {
        'images': [],
        'videos': [],
        'audio': []
    }
    
    image_exts = {'.jpg', '.jpeg', '.png', '.webp', '.gif', '.bmp'}
    video_exts = {'.mp4', '.webm', '.mkv', '.avi', '.mov', '.ts'}
    audio_exts = {'.mp3', '.wav', '.ogg', '.m4a', '.flac', '.aac', '.adts'}
    
    data_path = Path(data_dir)
    
    for file in data_path.rglob('*'):
        if not file.is_file():
            continue
            
        ext = file.suffix.lower()
        
        if ext in image_exts:
            media_files['images'].append(str(file))
        elif ext in video_exts:
            media_files['videos'].append(str(file))
        elif ext in audio_exts:
            media_files['audio'].append(str(file))
    
    return media_files

def convert_image_ffmpeg(input_file, output_format, quality):
    """使用FFmpeg转换图像"""
    output_file = f"/tmp/ppo_test_{Path(input_file).stem}.{output_format}"
    
    try:
        # 根据格式选择编码器
        if output_format == 'webp':
            cmd = [
                'ffmpeg', '-y', '-i', input_file,
                '-c:v', 'libwebp',
                '-quality', str(quality),
                output_file
            ]
        elif output_format == 'avif':
            cmd = [
                'ffmpeg', '-y', '-i', input_file,
                '-c:v', 'libaom-av1',
                '-crf', str(100 - quality),  # CRF: 0-63, 质量越高CRF越低
                output_file
            ]
        elif output_format == 'jxl':
            cmd = [
                'ffmpeg', '-y', '-i', input_file,
                '-c:v', 'libjxl',
                '-q:v', str(quality),
                output_file
            ]
        else:
            return {'success': False, 'error': f'Unsupported format: {output_format}'}
        
        result = subprocess.run(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=60
        )
        
        if result.returncode == 0 and Path(output_file).exists():
            input_size = Path(input_file).stat().st_size
            output_size = Path(output_file).stat().st_size
            Path(output_file).unlink()
            
            return {
                'success': True,
                'input_size': input_size,
                'output_size': output_size,
                'compression_ratio': output_size / input_size if input_size > 0 else 1.0
            }
        else:
            error_msg = result.stderr.split('\n')[-3:] if result.stderr else 'Unknown'
            return {'success': False, 'error': ' '.join(error_msg)[:100]}
            
    except subprocess.TimeoutExpired:
        return {'success': False, 'error': 'Timeout'}
    except Exception as e:
        return {'success': False, 'error': str(e)[:100]}

def convert_video_ffmpeg(input_file, output_format, bitrate):
    """使用FFmpeg转换视频"""
    output_file = f"/tmp/ppo_test_{Path(input_file).stem}.{output_format}"
    
    try:
        if output_format == 'webm':
            video_codec = 'libvpx-vp9'
            audio_codec = 'libopus'
        else:  # mp4
            video_codec = 'libx264'
            audio_codec = 'aac'
        
        cmd = [
            'ffmpeg', '-y', '-i', input_file,
            '-c:v', video_codec,
            '-b:v', f'{bitrate}k',
            '-c:a', audio_codec,
            '-b:a', '128k',
            output_file
        ]
        
        result = subprocess.run(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=120
        )
        
        if result.returncode == 0 and Path(output_file).exists():
            input_size = Path(input_file).stat().st_size
            output_size = Path(output_file).stat().st_size
            Path(output_file).unlink()
            
            return {
                'success': True,
                'input_size': input_size,
                'output_size': output_size,
                'compression_ratio': output_size / input_size if input_size > 0 else 1.0
            }
        else:
            return {'success': False, 'error': 'Conversion failed'}
            
    except subprocess.TimeoutExpired:
        return {'success': False, 'error': 'Timeout'}
    except Exception as e:
        return {'success': False, 'error': str(e)[:100]}

def convert_audio_ffmpeg(input_file, output_format, bitrate):
    """使用FFmpeg转换音频"""
    output_file = f"/tmp/ppo_test_{Path(input_file).stem}.{output_format}"
    
    try:
        codec_map = {
            'opus': 'libopus',
            'aac': 'aac',
            'mp3': 'libmp3lame'
        }
        
        codec = codec_map.get(output_format, 'libopus')
        
        cmd = [
            'ffmpeg', '-y', '-i', input_file,
            '-c:a', codec,
            '-b:a', f'{bitrate}k',
            output_file
        ]
        
        result = subprocess.run(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=60
        )
        
        if result.returncode == 0 and Path(output_file).exists():
            input_size = Path(input_file).stat().st_size
            output_size = Path(output_file).stat().st_size
            Path(output_file).unlink()
            
            return {
                'success': True,
                'input_size': input_size,
                'output_size': output_size,
                'compression_ratio': output_size / input_size if input_size > 0 else 1.0
            }
        else:
            return {'success': False, 'error': 'Conversion failed'}
            
    except subprocess.TimeoutExpired:
        return {'success': False, 'error': 'Timeout'}
    except Exception as e:
        return {'success': False, 'error': str(e)[:100]}

def generate_training_data(media_files, sample_size=None, media_types=['image', 'video', 'audio']):
    """生成训练数据"""
    training_data = []
    
    for media_type in media_types:
        if media_type == 'image':
            files = media_files['images']
            type_name = '图像'
            formats = ['webp']  # 先只测试webp，因为avif和jxl可能不支持
            qualities = [70, 85, 95]
            convert_func = convert_image_ffmpeg
        elif media_type == 'video':
            files = media_files['videos']
            type_name = '视频'
            formats = ['webm', 'mp4']
            qualities = [500, 1000, 2000]  # 视频比特率 (kbps)
            convert_func = convert_video_ffmpeg
        elif media_type == 'audio':
            files = media_files['audio']
            type_name = '音频'
            formats = ['opus', 'aac', 'mp3']
            qualities = [128, 192, 256]  # 音频比特率 (kbps)
            convert_func = convert_audio_ffmpeg
        else:
            continue
        
        if not files:
            print(f"\n⚠️  No{type_name}files found, skipping")
            continue
        
        if sample_size:
            sampled_files = random.sample(files, min(sample_size, len(files)))
        else:
            sampled_files = files
        
        print(f"\n{'='*60}")
        print(f"🎯 Processing {len(sampled_files)} {type_name}files...")
        print(f"{'='*60}")
        
        for idx, file_path in enumerate(sampled_files, 1):
            print(f"\n[{idx}/{len(sampled_files)}] {type_name}: {Path(file_path).name}")
            
            for quality in qualities:
                for target_format in formats:
                    result = convert_func(file_path, target_format, quality)
                    
                    if result['success']:
                        training_data.append({
                            'media_type': media_type,
                            'input_file': file_path,
                            'input_size': result['input_size'],
                            'target_format': target_format,
                            'quality': quality,
                            'output_size': result['output_size'],
                            'compression_ratio': result['compression_ratio'],
                            'reward': calculate_reward(result['compression_ratio'], quality, media_type)
                        })
                        print(f"  ✅ {target_format} Q{quality}: {result['compression_ratio']:.2%}")
                    else:
                        print(f"  ❌ {target_format} Q{quality}: {result.get('error', 'Unknown')}")
    
    return training_data

def calculate_reward(compression_ratio, quality, media_type):
    """计算奖励值"""
    compression_score = max(0, 1 - compression_ratio)
    
    if media_type == 'image':
        quality_score = quality / 100
        return compression_score * 0.6 + quality_score * 0.4
    elif media_type == 'video':
        quality_score = min(quality / 2000, 1.0)
        return compression_score * 0.5 + quality_score * 0.5
    else:  # audio
        quality_score = min(quality / 320, 1.0)
        return compression_score * 0.7 + quality_score * 0.3

def save_training_data(training_data, output_file):
    """保存训练数据"""
    output_path = Path(output_file)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    stats = {
        'image': len([d for d in training_data if d['media_type'] == 'image']),
        'video': len([d for d in training_data if d['media_type'] == 'video']),
        'audio': len([d for d in training_data if d['media_type'] == 'audio'])
    }
    
    with open(output_path, 'w') as f:
        json.dump({
            'timestamp': datetime.now().isoformat(),
            'total_samples': len(training_data),
            'stats': stats,
            'data': training_data
        }, f, indent=2)
    
    print(f"\n💾 Training data saved: {output_path}")
    print(f"   Total samples: {len(training_data)}")
    print(f"   Image samples: {stats['image']}")
    print(f"   Video samples: {stats['video']}")
    print(f"   Audio samples: {stats['audio']}")

def main():
    """主函数"""
    print("=" * 60)
    print("🤖 PPO Training - Pure FFmpeg Solution")
    print("=" * 60)
    
    if not check_ffmpeg():
        return 1
    
    print("✅ FFmpeg ready")
    
    data_dir = '/Users/nyamiiko/Documents/GIT/chromium-main/media/test/data'
    
    print("\n📁 Scanning media files...")
    media_files = find_media_files(data_dir)
    
    print(f"\n📊 File statistics:")
    print(f"   Images: {len(media_files['images'])} ")
    print(f"   Videos: {len(media_files['videos'])} ")
    print(f"   Audio: {len(media_files['audio'])} ")
    print(f"   Total: {len(media_files['images']) + len(media_files['videos']) + len(media_files['audio'])} ")
    
    # 步骤1: 小规模测试
    print("\n" + "=" * 60)
    print("🧪 Step1: Small-scale test (per type2files)")
    print("=" * 60)
    
    test_data = generate_training_data(media_files, sample_size=2, media_types=['image', 'video', 'audio'])
    
    if len(test_data) == 0:
        print("\n❌ Test failed: No training data generated")
        return 1
    
    print(f"\n✅ Small-scale test succeeded! Generated {len(test_data)} training samples")
    
    save_training_data(test_data, 'models/ppo_training_ffmpeg_test.json')
    
    # 步骤2: 全量训练
    print("\n" + "=" * 60)
    print("🚀 Step2: Full training")
    print("=" * 60)
    
    # 自动继续全量训练
    response = 'y'
    print("\nAuto-continuing full training...")
    
    if response == 'y':
        print("\nStarting full training...")
        full_data = generate_training_data(media_files, sample_size=None, media_types=['image', 'video', 'audio'])
        
        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
        save_training_data(full_data, f'models/ppo_training_all_media_{timestamp}.json')
        
        print("\n" + "=" * 60)
        print("🎉 Training completed！")
        print("=" * 60)
        
        # 分类统计
        image_data = [d for d in full_data if d['media_type'] == 'image']
        video_data = [d for d in full_data if d['media_type'] == 'video']
        audio_data = [d for d in full_data if d['media_type'] == 'audio']
        
        print(f"\n📈 Training statistics:")
        print(f"   Total samples: {len(full_data)}")
        
        if image_data:
            print(f"\n   Images:")
            print(f"     Samples: {len(image_data)}")
            print(f"     Average compression ratio: {sum(d['compression_ratio'] for d in image_data) / len(image_data):.2%}")
            print(f"     Average reward: {sum(d['reward'] for d in image_data) / len(image_data):.4f}")
        
        if video_data:
            print(f"\n   Videos:")
            print(f"     Samples: {len(video_data)}")
            print(f"     Average compression ratio: {sum(d['compression_ratio'] for d in video_data) / len(video_data):.2%}")
            print(f"     Average reward: {sum(d['reward'] for d in video_data) / len(video_data):.4f}")
        
        if audio_data:
            print(f"\n   Audio:")
            print(f"     Samples: {len(audio_data)}")
            print(f"     Average compression ratio: {sum(d['compression_ratio'] for d in audio_data) / len(audio_data):.2%}")
            print(f"     Average reward: {sum(d['reward'] for d in audio_data) / len(audio_data):.4f}")
        
        return 0
    else:
        print("\n⏸️  Full training cancelled")
        return 0

if __name__ == '__main__':
    sys.exit(main())
