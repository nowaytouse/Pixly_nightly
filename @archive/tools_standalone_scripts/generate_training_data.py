#!/usr/bin/env python3
"""
生成PPO训练数据
从图像文件生成观测数据（状态-动作-奖励）
"""

import os
import sys
import json
import glob
from pathlib import Path
from PIL import Image
import argparse

# 添加tools到路径
sys.path.insert(0, os.path.dirname(__file__))

from predict_params import AIPredictor

def calculate_reward(prediction, input_size, output_size_estimate):
    """
    计算奖励函数
    
    奖励 = 压缩率 * 质量保持 * 效率
    - 压缩率：文件大小减少比例（正向奖励）
    - 质量保持：基于AI的质量预测（高质量参数给高奖励）
    - 效率：基于effort/speed参数（低effort给更高奖励）
    """
    params = prediction.get('params', {})
    
    # 估计压缩率（简化版，实际需要转换后才知道）
    # 这里基于AI推荐的参数和格式估算
    recommended_format = prediction.get('recommended_format', 'webp')
    quality = params.get('quality', 90)
    
    # 格式压缩效率估计
    format_efficiency = {
        'jxl': 0.5,    # JXL最高效
        'avif': 0.6,   # AVIF次之
        'webp': 0.7,   # WebP中等
        'heic': 0.65   # HEIC略好于WebP
    }
    
    compression_estimate = format_efficiency.get(recommended_format, 0.7)
    estimated_output = input_size * compression_estimate * (quality / 100.0)
    compression_ratio = (input_size - estimated_output) / input_size
    
    # 质量保持分数（质量参数越高，分数越高）
    quality_score = quality / 100.0
    
    # 效率分数（effort越低越好，speed越高越好）
    effort = params.get('effort', 7)
    speed = params.get('speed', 5)
    efficiency_score = (1 - (effort / 9.0)) * 0.5 + (speed / 10.0) * 0.5
    
    # 综合奖励
    reward = (
        compression_ratio * 0.6 +      # 压缩率权重60%
        quality_score * 0.3 +           # 质量权重30%
        efficiency_score * 0.1          # 效率权重10%
    )
    
    return max(0.0, reward)  # 确保非负

def process_image(image_path, predictor, output_dir):
    """处理单个图像生成观测数据"""
    try:
        # 1. 获取图像基本信息
        img = Image.open(image_path)
        width, height = img.size
        has_alpha = img.mode in ['RGBA', 'LA', 'PA']
        input_size = os.path.getsize(image_path)
        
        # 2. 使用AI预测器生成参数
        result = predictor.predict(
            image_path,
            tool='auto',  # 自动选择工具
            target_quality=90,
            optimize_mode='balanced'
        )
        
        if not result:
            return None
        
        # 3. 提取预测结果
        params = result.get('params', {})
        recommended_format = result.get('recommended_format', 'webp')
        
        # 4. 计算奖励
        reward = calculate_reward(result, input_size, input_size * 0.6)
        
        # 5. 构建观测数据
        observation = {
            'image_path': str(image_path),
            'image_type': Path(image_path).suffix,
            'width': width,
            'height': height,
            'pixel_count': width * height,
            'has_alpha': has_alpha,
            'input_size': input_size,
            'tool': recommended_format,
            'optimize_mode': 'balanced',
            'params': params,
            'reward': reward,
            'timestamp': str(Path(image_path).stat().st_mtime)
        }
        
        # 6. 保存观测数据
        obs_filename = Path(image_path).stem + '_obs.json'
        obs_path = os.path.join(output_dir, obs_filename)
        
        with open(obs_path, 'w', encoding='utf-8') as f:
            json.dump(observation, f, indent=2)
        
        return observation
        
    except Exception as e:
        print(f"❌ 处理失败 {image_path}: {e}")
        return None

def main():
    parser = argparse.ArgumentParser(description='生成PPO训练数据')
    parser.add_argument('--input_dir', type=str, required=True,
                        help='输入图像目录')
    parser.add_argument('--output_dir', type=str, default='data/observations',
                        help='观测数据输出目录')
    parser.add_argument('--max_images', type=int, default=500,
                        help='最大处理图像数量')
    parser.add_argument('--extensions', type=str, default='jpg,jpeg,png,webp',
                        help='图像扩展名（逗号分隔）')
    
    args = parser.parse_args()
    
    print("=" * 70)
    print("🤖 生成PPO训练数据")
    print("=" * 70)
    print()
    
    # 创建输出目录
    os.makedirs(args.output_dir, exist_ok=True)
    
    # 初始化AI预测器
    print("🔧 初始化AI预测器...")
    predictor = AIPredictor(model_dir='./models')
    print()
    
    # 查找所有图像
    extensions = args.extensions.split(',')
    image_files = []
    for ext in extensions:
        pattern = f"{args.input_dir}/**/*.{ext}"
        image_files.extend(glob.glob(pattern, recursive=True))
    
    print(f"📂 找到 {len(image_files)} 个图像文件")
    
    # 限制数量
    if len(image_files) > args.max_images:
        import random
        random.shuffle(image_files)
        image_files = image_files[:args.max_images]
        print(f"   随机选择 {args.max_images} 个进行处理")
    
    print()
    print("⚙️  开始处理...")
    print()
    
    # 处理图像
    success_count = 0
    fail_count = 0
    observations = []
    
    for i, image_path in enumerate(image_files, 1):
        if i % 50 == 0:
            print(f"进度: {i}/{len(image_files)} ({i*100//len(image_files)}%)")
        
        obs = process_image(image_path, predictor, args.output_dir)
        
        if obs:
            observations.append(obs)
            success_count += 1
        else:
            fail_count += 1
    
    print()
    print("=" * 70)
    print("✅ 数据生成完成")
    print()
    print(f"📊 统计:")
    print(f"   成功: {success_count}")
    print(f"   失败: {fail_count}")
    print(f"   输出目录: {args.output_dir}")
    print()
    
    # 保存汇总
    summary = {
        'total_images': len(image_files),
        'success': success_count,
        'failed': fail_count,
        'average_reward': sum(obs['reward'] for obs in observations) / len(observations) if observations else 0,
        'output_dir': args.output_dir
    }
    
    summary_path = os.path.join(args.output_dir, 'summary.json')
    with open(summary_path, 'w') as f:
        json.dump(summary, f, indent=2)
    
    print(f"📄 汇总已保存: {summary_path}")
    print()
    print("🚀 下一步: 运行PPO训练")
    print(f"   python3 tools/train_ppo.py --observations_dir {args.output_dir}")
    print()
    
    return 0

if __name__ == '__main__':
    sys.exit(main())
