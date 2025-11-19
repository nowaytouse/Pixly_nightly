#!/usr/bin/env python3
"""
PPO训练脚本 - 全媒体类型支持
支持图像、视频、音频的PPO强化学习训练
"""

import os
import sys
import json
import subprocess
from pathlib import Path
from datetime import datetime
import random

# 添加项目路径
sys.path.insert(0, str(Path(__file__).parent.parent))

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

def get_target_formats(media_type):
    """根据媒体类型返回目标格式"""
    formats = {
        'image': ['webp', 'avif', 'jxl'],
        'video': ['webm', 'mp4'],
        'audio': ['opus', 'aac', 'mp3']
    }
    return formats.get(media_type, [])

def run_conversion(input_file, output_format, quality, media_type):
    """运行转换并返回结果"""
    output_file = f"/tmp/ppo_test_{Path(input_file).stem}.{output_format}"
    
    try:
        # 使用已编译的Rust CLI
        pixly_bin = '/Users/nyamiiko/Documents/GIT/Pixly/Pixly_Nightly/plugin/converter/bin/pixly-rust'
        
        if not Path(pixly_bin).exists():
            return {'success': False, 'error': f'pixly-rust binary not found'}
        
        # 根据媒体类型构建命令
        cmd = [
            pixly_bin,
            'convert',
            input_file,
            output_file,
            '--format', output_format,
            '--quality', str(quality)
        ]
        
        # 视频特定参数
        if media_type == 'video':
            cmd.extend(['--video-codec', 'auto'])
        
        # 音频特定参数
        if media_type == 'audio':
            cmd.extend(['--audio-codec', 'auto'])
        
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=120  # 视频和音频需要更长时间
        )
        
        if result.returncode == 0 and Path(output_file).exists():
            input_size = Path(input_file).stat().st_size
            output_size = Path(output_file).stat().st_size
            
            # 清理临时文件
            Path(output_file).unlink()
            
            return {
                'success': True,
                'input_size': input_size,
                'output_size': output_size,
                'compression_ratio': output_size / input_size if input_size > 0 else 1.0
            }
        else:
            return {'success': False, 'error': result.stderr[:100] if result.stderr else 'Unknown error'}
            
    except subprocess.TimeoutExpired:
        return {'success': False, 'error': 'Timeout'}
    except Exception as e:
        return {'success': False, 'error': str(e)[:100]}

def generate_training_data(media_files, sample_size=None, media_types=['image', 'video', 'audio']):
    """生成训练数据 - 支持所有媒体类型"""
    training_data = []
    
    # 处理每种媒体类型
    for media_type in media_types:
        if media_type == 'image':
            files = media_files['images']
            type_name = '图像'
        elif media_type == 'video':
            files = media_files['videos']
            type_name = '视频'
        elif media_type == 'audio':
            files = media_files['audio']
            type_name = '音频'
        else:
            continue
        
        if not files:
            print(f"\n⚠️  No {type_name} files found, skipping")
            continue
        
        # Random sampling if sample_size specified
        if sample_size:
            sampled_files = random.sample(files, min(sample_size, len(files)))
        else:
            sampled_files = files
        
        print(f"\n{'='*60}")
        print(f"🎯 Processing {len(sampled_files)} {type_name} files...")
        print(f"{'='*60}")
        
        target_formats = get_target_formats(media_type)
        
        for idx, file_path in enumerate(sampled_files, 1):
            print(f"\n[{idx}/{len(sampled_files)}] {type_name}: {Path(file_path).name}")
            
            # 测试不同质量参数
            qualities = [70, 85, 95] if media_type == 'image' else [128, 192, 256]  # 音频/视频用比特率
            
            for quality in qualities:
                for target_format in target_formats:
                    result = run_conversion(file_path, target_format, quality, media_type)
                    
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
    """计算奖励值 - 根据媒体类型调整"""
    # 基础压缩分数
    compression_score = max(0, 1 - compression_ratio)
    
    # 质量分数（根据媒体类型归一化）
    if media_type == 'image':
        quality_score = quality / 100
    else:  # video/audio 使用比特率
        quality_score = min(quality / 320, 1.0)  # 归一化到0-1
    
    # 不同媒体类型的权重
    if media_type == 'image':
        return compression_score * 0.6 + quality_score * 0.4
    elif media_type == 'video':
        return compression_score * 0.5 + quality_score * 0.5  # 视频更注重质量
    else:  # audio
        return compression_score * 0.7 + quality_score * 0.3  # 音频更注重压缩

def save_training_data(training_data, output_file):
    """保存训练数据"""
    output_path = Path(output_file)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    # 统计各类型数据
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
    """Main function"""
    print("=" * 60)
    print("🤖 PPO Training - All Media Types Support")
    print("=" * 60)
    
    data_dir = '/Users/nyamiiko/Documents/GIT/chromium-main/media/test/data'
    
    # Find all media files
    print("\n📁 Scanning media files...")
    media_files = find_media_files(data_dir)
    
    print(f"\n📊 File statistics:")
    print(f"   Images: {len(media_files['images'])} files")
    print(f"   Videos: {len(media_files['videos'])} files")
    print(f"   Audio: {len(media_files['audio'])} files")
    print(f"   Total: {len(media_files['images']) + len(media_files['videos']) + len(media_files['audio'])} files")
    
    # Step 1: Small-scale test
    print("\n" + "=" * 60)
    print("🧪 Step 1: Small-scale test (2 files per type)")
    print("=" * 60)
    
    test_data = generate_training_data(media_files, sample_size=2, media_types=['image', 'video', 'audio'])
    
    if len(test_data) == 0:
        print("\n❌ Test failed: No training data generated")
        return 1
    
    print(f"\n✅ Small-scale test successful! Generated {len(test_data)} training samples")
    
    # 保存测试数据
    save_training_data(test_data, 'models/ppo_training_all_media_test.json')
    
    # 步骤2: 询问是否继续全量训练
    print("\n" + "=" * 60)
    print("🚀 Step 2: Full training")
    print("=" * 60)
    
    response = input("\nContinue with full training? (y/n): ").strip().lower()
    
    if response == 'y':
        print("\nStarting full training...")
        full_data = generate_training_data(media_files, sample_size=None, media_types=['image', 'video', 'audio'])
        
        # Save full training data
        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
        save_training_data(full_data, f'models/ppo_training_all_media_{timestamp}.json')
        
        print("\n" + "=" * 60)
        print("🎉 Training complete!")
        print("=" * 60)
        
        # Category statistics
        image_data = [d for d in full_data if d['media_type'] == 'image']
        video_data = [d for d in full_data if d['media_type'] == 'video']
        audio_data = [d for d in full_data if d['media_type'] == 'audio']
        
        print(f"\n📈 Training statistics:")
        print(f"   Total samples: {len(full_data)}")
        
        if image_data:
            print(f"\n   Images:")
            print(f"     Samples: {len(image_data)}")
            print(f"     Avg compression: {sum(d['compression_ratio'] for d in image_data) / len(image_data):.2%}")
            print(f"     Avg reward: {sum(d['reward'] for d in image_data) / len(image_data):.4f}")
        
        if video_data:
            print(f"\n   Videos:")
            print(f"     Samples: {len(video_data)}")
            print(f"     Avg compression: {sum(d['compression_ratio'] for d in video_data) / len(video_data):.2%}")
            print(f"     Avg reward: {sum(d['reward'] for d in video_data) / len(video_data):.4f}")
        
        if audio_data:
            print(f"\n   Audio:")
            print(f"     Samples: {len(audio_data)}")
            print(f"     Avg compression: {sum(d['compression_ratio'] for d in audio_data) / len(audio_data):.2%}")
            print(f"     Avg reward: {sum(d['reward'] for d in audio_data) / len(audio_data):.4f}")
        
        return 0
    else:
        print("\n⏸️  Full training cancelled")
        return 0

if __name__ == '__main__':
    sys.exit(main())
