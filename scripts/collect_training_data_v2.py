#!/usr/bin/env python3
"""
🤖 训练数据收集脚本 V2 - Phase 3.1 正确实现

收集真实转换数据：
- 输入：图像特征（128维）+ 转换参数（quality, effort）
- 输出：文件大小、SSIM质量、处理时间
- 目标：学习"特征+参数 → 输出质量"的映射关系

这样模型才能学习：
- 什么样的图像用高quality效果好
- 什么样的图像用低effort就够了
- 如何在质量和大小之间平衡
"""

import json
import subprocess
import sys
import os
import tempfile
import time
from pathlib import Path
from datetime import datetime

def extract_features(image_path):
    """提取128维特征"""
    try:
        result = subprocess.run(
            ['cargo', 'run', '--release', '--', 'analyze', str(image_path)],
            capture_output=True,
            text=True,
            timeout=30
        )
        
        if result.returncode != 0:
            return None
        
        output = result.stdout
        if "Features (128-dim):" in output:
            features_line = output.split("Features (128-dim):")[1].split("\n")[0].strip()
            features_str = features_line.strip('[]')
            features = [float(x.strip()) for x in features_str.split(',')]
            return features
        
        return None
        
    except Exception as e:
        print(f"❌ Error extracting features: {e}")
        return None

def convert_and_measure(input_path, output_path, target_format, quality, effort):
    """
    执行转换并测量结果
    
    返回:
    - output_size: 输出文件大小（字节）
    - ssim: 结构相似性（0-1，越高越好）
    - processing_time: 处理时间（秒）
    - compression_ratio: 压缩比（输入/输出）
    """
    input_size = os.path.getsize(input_path)
    
    # 构建转换命令
    cmd = [
        'cargo', 'run', '--release', '--',
        'convert',
        str(input_path),
        str(output_path),
        '--format', target_format,
        '--quality', str(quality),
        '--effort', str(effort)
    ]
    
    # 执行转换并计时
    start_time = time.time()
    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=60
        )
        processing_time = time.time() - start_time
        
        if result.returncode != 0:
            print(f"   ⚠️ Conversion failed: {result.stderr[:100]}")
            return None
        
        # 检查输出文件
        if not os.path.exists(output_path):
            print(f"   ⚠️ Output file not created")
            return None
        
        output_size = os.path.getsize(output_path)
        
        # 计算SSIM（如果可用）
        ssim = calculate_ssim(input_path, output_path)
        
        # 计算压缩比
        compression_ratio = input_size / output_size if output_size > 0 else 0
        
        return {
            'output_size': output_size,
            'ssim': ssim,
            'processing_time': processing_time,
            'compression_ratio': compression_ratio,
            'input_size': input_size
        }
        
    except subprocess.TimeoutExpired:
        print(f"   ⚠️ Conversion timeout")
        return None
    except Exception as e:
        print(f"   ⚠️ Error: {e}")
        return None

def calculate_ssim(original, converted):
    """
    计算SSIM（结构相似性指数）
    
    使用ImageMagick的compare命令
    返回0-1之间的值，1表示完全相同
    """
    try:
        result = subprocess.run(
            ['magick', 'compare', '-metric', 'SSIM', 
             str(original), str(converted), 'null:'],
            capture_output=True,
            text=True,
            timeout=30
        )
        
        # SSIM值在stderr中
        stderr = result.stderr.strip()
        if stderr:
            # 解析SSIM值（格式可能是 "0.95" 或 "0.95 (0.95, 0.95, 0.95)"）
            ssim_str = stderr.split()[0]
            ssim = float(ssim_str)
            return ssim
        
        return 0.95  # 默认值
        
    except Exception as e:
        print(f"   ⚠️ SSIM calculation failed: {e}")
        return 0.95  # 默认值

def collect_training_samples(data_dir, formats=['webp', 'avif', 'jxl'], 
                             qualities=[60, 70, 80, 90], 
                             efforts=[4, 6, 8]):
    """
    收集训练样本
    
    对每个图像：
    1. 提取特征
    2. 尝试所有参数组合
    3. 记录输入特征、参数、输出结果
    """
    data_path = Path(data_dir)
    if not data_path.exists():
        print(f"❌ Data directory not found: {data_dir}")
        return []
    
    # 支持的图像格式
    image_extensions = ['.jpg', '.jpeg', '.png', '.webp', '.gif']
    image_files = []
    for ext in image_extensions:
        image_files.extend(data_path.glob(f'**/*{ext}'))
    
    print(f"📊 Found {len(image_files)} images in {data_dir}")
    
    training_samples = []
    temp_dir = Path(tempfile.mkdtemp(prefix='pixly_training_'))
    
    try:
        for idx, image_file in enumerate(image_files, 1):
            print(f"\n[{idx}/{len(image_files)}] Processing: {image_file.name}")
            
            # 提取特征
            features = extract_features(image_file)
            if features is None:
                print(f"   ⚠️ Failed to extract features, skipping")
                continue
            
            if len(features) != 128:
                print(f"   ⚠️ Invalid feature dimension: {len(features)}, skipping")
                continue
            
            # 对每种格式和参数组合进行转换
            for target_format in formats:
                for quality in qualities:
                    for effort in efforts:
                        output_file = temp_dir / f"temp_{idx}_{target_format}_q{quality}_e{effort}.{target_format}"
                        
                        print(f"   🔄 {target_format} Q{quality} E{effort}...", end=' ')
                        
                        # 执行转换并测量
                        result = convert_and_measure(
                            image_file, output_file, 
                            target_format, quality, effort
                        )
                        
                        if result:
                            # 记录训练样本
                            sample = {
                                'input_path': str(image_file),
                                'input_size': result['input_size'],
                                'features': features,
                                'target_format': target_format,
                                'quality': quality,
                                'effort': effort,
                                'output_size': result['output_size'],
                                'ssim': result['ssim'],
                                'processing_time': result['processing_time'],
                                'compression_ratio': result['compression_ratio']
                            }
                            training_samples.append(sample)
                            print(f"✅ {result['output_size']/1024:.1f}KB SSIM:{result['ssim']:.3f}")
                        else:
                            print("❌")
                        
                        # 清理临时文件
                        if output_file.exists():
                            output_file.unlink()
            
            # 每10个文件保存一次
            if idx % 10 == 0:
                save_samples(training_samples, 'models/training_data_v2_partial.json')
                print(f"\n💾 Saved {len(training_samples)} samples (partial)")
    
    finally:
        # 清理临时目录
        import shutil
        shutil.rmtree(temp_dir, ignore_errors=True)
    
    return training_samples

def save_samples(samples, output_file):
    """保存训练样本"""
    output_path = Path(output_file)
    output_path.parent.mkdir(exist_ok=True)
    
    with open(output_path, 'w') as f:
        json.dump(samples, f, indent=2)
    
    print(f"\n✅ Saved {len(samples)} training samples to {output_file}")

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Collect training data V2")
    parser.add_argument('--data-dir', default='data/training_samples', 
                       help='Directory containing images')
    parser.add_argument('--output', default='models/training_data_v2.json',
                       help='Output file')
    parser.add_argument('--formats', nargs='+', default=['webp', 'avif', 'jxl'],
                       help='Target formats')
    parser.add_argument('--qualities', nargs='+', type=int, default=[60, 70, 80, 90],
                       help='Quality values to test')
    parser.add_argument('--efforts', nargs='+', type=int, default=[4, 6, 8],
                       help='Effort values to test')
    parser.add_argument('--limit', type=int, help='Limit number of images')
    
    args = parser.parse_args()
    
    print("="*60)
    print("🤖 Pixly Training Data Collection V2")
    print("="*60)
    print(f"Data directory: {args.data_dir}")
    print(f"Target formats: {args.formats}")
    print(f"Quality values: {args.qualities}")
    print(f"Effort values: {args.efforts}")
    print(f"Output file: {args.output}")
    print("="*60)
    
    # 收集样本
    samples = collect_training_samples(
        args.data_dir,
        formats=args.formats,
        qualities=args.qualities,
        efforts=args.efforts
    )
    
    if args.limit:
        samples = samples[:args.limit]
    
    # 保存结果
    save_samples(samples, args.output)
    
    # 统计信息
    print("\n📊 Collection Summary:")
    print(f"   Total samples: {len(samples)}")
    
    by_format = {}
    for sample in samples:
        fmt = sample['target_format']
        by_format[fmt] = by_format.get(fmt, 0) + 1
    
    for fmt, count in sorted(by_format.items()):
        print(f"   {fmt}: {count} samples")
    
    print(f"\n✅ Training data collection complete!")
    print(f"   Next step: python3 scripts/train_lightgbm_v2.py --data {args.output}")

if __name__ == '__main__':
    main()
