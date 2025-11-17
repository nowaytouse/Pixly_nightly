#!/usr/bin/env python3
"""
PPO训练脚本 - Chromium测试数据
使用Chromium的测试媒体数据进行PPO强化学习训练
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
    video_exts = {'.mp4', '.webm', '.mkv', '.avi', '.mov'}
    audio_exts = {'.mp3', '.wav', '.ogg', '.m4a', '.flac', '.aac'}
    
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

def run_conversion(input_file, output_format, quality):
    """运行转换并返回结果"""
    output_file = f"/tmp/ppo_test_{Path(input_file).stem}.{output_format}"
    
    try:
        # 使用已编译的Rust CLI (绝对路径)
        pixly_bin = '/Users/nyamiiko/Documents/GIT/Pixly/Pixly_Nightly/plugin/converter/bin/pixly-rust'
        
        if not Path(pixly_bin).exists():
            return {'success': False, 'error': f'pixly-rust binary not found at {pixly_bin}'}
        
        cmd = [
            pixly_bin,
            'convert',
            input_file,
            output_file,
            '--format', output_format,
            '--quality', str(quality)
        ]
        
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=60
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
            return {'success': False, 'error': result.stderr}
            
    except Exception as e:
        return {'success': False, 'error': str(e)}

def generate_training_data(media_files, sample_size=None):
    """生成训练数据"""
    training_data = []
    
    # 🎯 目前只处理图像文件（视频和音频转换需要不同的命令）
    image_files = media_files['images']
    
    # 如果指定sample_size，随机采样
    if sample_size:
        sampled_files = random.sample(image_files, min(sample_size, len(image_files)))
    else:
        sampled_files = image_files
    
    print(f"\n🎯 处理 {len(sampled_files)} 个图像文件...")
    print(f"   (视频和音频转换将在后续版本支持)")
    
    for idx, file_path in enumerate(sampled_files, 1):
        file_ext = Path(file_path).suffix.lower()
        
        # 只处理图像
        if file_ext in {'.jpg', '.jpeg', '.png', '.webp', '.gif', '.bmp'}:
            target_formats = ['webp', 'avif', 'jxl']
        else:
            continue
        
        print(f"[{idx}/{len(sampled_files)}] 处理: {Path(file_path).name}")
        
        # 测试不同质量参数
        for quality in [70, 85, 95]:
            for target_format in target_formats:
                result = run_conversion(file_path, target_format, quality)
                
                if result['success']:
                    training_data.append({
                        'input_file': file_path,
                        'input_size': result['input_size'],
                        'target_format': target_format,
                        'quality': quality,
                        'output_size': result['output_size'],
                        'compression_ratio': result['compression_ratio'],
                        'reward': calculate_reward(result['compression_ratio'], quality)
                    })
                    print(f"  ✅ {target_format} Q{quality}: {result['compression_ratio']:.2%}")
                else:
                    print(f"  ❌ {target_format} Q{quality}: {result.get('error', 'Unknown error')[:50]}")
    
    return training_data

def calculate_reward(compression_ratio, quality):
    """计算奖励值"""
    # 奖励函数：平衡压缩率和质量
    # 压缩率越小越好，质量越高越好
    compression_score = max(0, 1 - compression_ratio)
    quality_score = quality / 100
    
    return compression_score * 0.6 + quality_score * 0.4

def save_training_data(training_data, output_file):
    """保存训练数据"""
    output_path = Path(output_file)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_path, 'w') as f:
        json.dump({
            'timestamp': datetime.now().isoformat(),
            'total_samples': len(training_data),
            'data': training_data
        }, f, indent=2)
    
    print(f"\n💾 训练数据已保存: {output_path}")
    print(f"   样本数: {len(training_data)}")

def main():
    """主函数"""
    print("=" * 60)
    print("🤖 PPO训练 - Chromium测试数据")
    print("=" * 60)
    
    data_dir = '/Users/nyamiiko/Documents/GIT/chromium-main/media/test/data'
    
    # 查找所有媒体文件
    print("\n📁 扫描媒体文件...")
    media_files = find_media_files(data_dir)
    
    print(f"\n📊 文件统计:")
    print(f"   图像: {len(media_files['images'])} 个")
    print(f"   视频: {len(media_files['videos'])} 个")
    print(f"   音频: {len(media_files['audio'])} 个")
    print(f"   总计: {len(media_files['images']) + len(media_files['videos']) + len(media_files['audio'])} 个")
    
    # 询问是否进行小规模测试
    print("\n" + "=" * 60)
    print("🧪 步骤1: 小规模测试 (5个文件)")
    print("=" * 60)
    
    test_data = generate_training_data(media_files, sample_size=5)
    
    if len(test_data) == 0:
        print("\n❌ 测试失败：没有生成训练数据")
        return 1
    
    print(f"\n✅ 小规模测试成功！生成了 {len(test_data)} 个训练样本")
    
    # 保存测试数据
    save_training_data(test_data, 'models/ppo_training_test.json')
    
    # 询问是否继续全量训练
    print("\n" + "=" * 60)
    print("🚀 步骤2: 全量训练")
    print("=" * 60)
    
    response = input("\n是否继续全量训练？(y/n): ").strip().lower()
    
    if response == 'y':
        print("\n开始全量训练...")
        full_data = generate_training_data(media_files, sample_size=None)
        
        # 保存完整训练数据
        save_training_data(full_data, 'models/ppo_training_chromium.json')
        
        print("\n" + "=" * 60)
        print("🎉 训练完成！")
        print("=" * 60)
        print(f"\n📈 训练统计:")
        print(f"   总样本数: {len(full_data)}")
        print(f"   平均压缩率: {sum(d['compression_ratio'] for d in full_data) / len(full_data):.2%}")
        print(f"   平均奖励: {sum(d['reward'] for d in full_data) / len(full_data):.4f}")
        
        return 0
    else:
        print("\n⏸️  全量训练已取消")
        return 0

if __name__ == '__main__':
    sys.exit(main())
