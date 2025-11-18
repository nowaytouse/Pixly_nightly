#!/usr/bin/env python3
"""
🔬 训练数据收集脚本

收集真实转换数据用于模型训练

使用方法:
    python scripts/collect_training_data.py --data-dir data/test_images --limit 10
"""

import os
import sys
import json
import subprocess
from pathlib import Path
from datetime import datetime
import argparse
import time

def find_test_images(data_dir):
    """查找测试图像"""
    image_exts = {'.jpg', '.jpeg', '.png', '.webp', '.gif', '.bmp'}
    images = []
    
    data_path = Path(data_dir)
    for file in data_path.rglob('*'):
        if file.is_file() and file.suffix.lower() in image_exts:
            images.append(str(file))
    
    return images

def extract_features_rust(image_path, pixly_bin):
    """使用Rust CLI提取128维特征"""
    try:
        result = subprocess.run(
            [pixly_bin, 'analyze', image_path, '--json'],
            capture_output=True,
            text=True,
            timeout=30
        )
        
        if result.returncode == 0:
            data = json.loads(result.stdout)
            # 确保有128维特征
            if 'features' in data and len(data['features']) == 128:
                return data
            else:
                print(f"⚠️ Invalid feature vector length: {len(data.get('features', []))}", file=sys.stderr)
                return None
        else:
            print(f"⚠️ Feature extraction failed: {result.stderr}", file=sys.stderr)
            return None
    except json.JSONDecodeError as e:
        print(f"⚠️ JSON decode error: {e}", file=sys.stderr)
        return None
    except Exception as e:
        print(f"⚠️ Error: {e}", file=sys.stderr)
        return None

def convert_with_params(image_path, output_format, quality, effort, pixly_bin):
    """执行转换并返回结果（包括SSIM质量评估）"""
    timestamp = int(time.time() * 1000)
    output_file = f"/tmp/training_{Path(image_path).stem}_{timestamp}_q{quality}_e{effort}.{output_format}"
    
    try:
        start_time = time.time()
        
        cmd = [
            pixly_bin,
            'convert',
            image_path,
            output_file,
            '--format', output_format,
            '--quality', str(quality),
            '--effort', str(effort),
            '--no-ai'  # 禁用AI预测，使用指定参数
        ]
        
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=120
        )
        
        processing_time = time.time() - start_time
        
        if result.returncode == 0 and Path(output_file).exists():
            output_size = Path(output_file).stat().st_size
            
            # 尝试提取SSIM（如果输出中包含）
            ssim = None
            for line in result.stdout.split('\n'):
                if 'SSIM' in line or 'ssim' in line:
                    try:
                        ssim = float(line.split(':')[-1].strip())
                    except:
                        pass
            
            # 清理临时文件
            Path(output_file).unlink()
            
            return {
                'success': True,
                'output_size': output_size,
                'processing_time': processing_time,
                'ssim': ssim,
                'stdout': result.stdout
            }
        else:
            return {
                'success': False,
                'error': result.stderr,
                'processing_time': processing_time
            }
    except subprocess.TimeoutExpired:
        return {
            'success': False,
            'error': 'Timeout (>120s)'
        }
    except Exception as e:
        return {
            'success': False,
            'error': str(e)
        }

def collect_training_samples(data_dir, output_file, pixly_bin, sample_limit=None):
    """收集训练样本"""
    print(f"🔍 Scanning images in: {data_dir}")
    images = find_test_images(data_dir)
    
    if sample_limit:
        images = images[:sample_limit]
    
    print(f"📊 Found {len(images)} images")
    
    if len(images) == 0:
        print("❌ No images found!")
        return
    
    training_samples = []
    formats = ['avif', 'webp', 'jxl']
    qualities = [60, 70, 80, 90]
    efforts = [4, 6, 8]
    
    total_conversions = len(images) * len(formats) * len(qualities) * len(efforts)
    current = 0
    failed = 0
    
    for image_path in images:
        print(f"\n📷 Processing: {Path(image_path).name}")
        
        # 提取特征
        features = extract_features_rust(image_path, pixly_bin)
        if not features:
            print(f"   ⚠️ Skipping (feature extraction failed)")
            continue
        
        input_size = Path(image_path).stat().st_size
        
        for fmt in formats:
            for quality in qualities:
                for effort in efforts:
                    current += 1
                    progress = (current / total_conversions) * 100
                    
                    print(f"   [{progress:5.1f}%] {fmt} Q{quality} E{effort}...", end=' ', flush=True)
                    
                    # 执行转换
                    result = convert_with_params(image_path, fmt, quality, effort, pixly_bin)
                    
                    if result['success']:
                        compression_ratio = result['output_size'] / input_size
                        
                        # 记录样本
                        sample = {
                            'input_path': image_path,
                            'input_size': input_size,
                            'features': features.get('features', [0.0] * 128),
                            'target_format': fmt,
                            'quality': quality,
                            'effort': effort,
                            'output_size': result['output_size'],
                            'compression_ratio': compression_ratio,
                            'processing_time': result.get('processing_time', 0),
                            'ssim': result.get('ssim'),
                            'timestamp': int(datetime.now().timestamp())
                        }
                        
                        training_samples.append(sample)
                        print(f"✅ {compression_ratio:.1%} ({result['processing_time']:.1f}s)")
                    else:
                        failed += 1
                        print(f"❌ {result.get('error', 'Unknown error')[:30]}")
    
    # 保存数据集
    print(f"\n💾 Saving {len(training_samples)} samples to: {output_file}")
    
    # 确保输出目录存在
    Path(output_file).parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_file, 'w') as f:
        json.dump(training_samples, f, indent=2)
    
    print(f"\n✅ Training data collection complete!")
    print(f"   Total samples: {len(training_samples)}")
    print(f"   Failed: {failed}")
    print(f"   Success rate: {len(training_samples) / total_conversions * 100:.1f}%")
    print(f"   Output: {output_file}")

def main():
    parser = argparse.ArgumentParser(description="Collect training data for ML models")
    parser.add_argument('--data-dir', default='data/test_images', help='Test images directory')
    parser.add_argument('--output', default='models/training_data.json', help='Output file')
    parser.add_argument('--pixly-bin', default='./target/release/pixly-converter', help='Pixly binary path')
    parser.add_argument('--limit', type=int, help='Limit number of images')
    
    args = parser.parse_args()
    
    # 检查pixly binary
    if not Path(args.pixly_bin).exists():
        print(f"❌ Pixly binary not found: {args.pixly_bin}")
        print(f"   Run: cargo build --release")
        sys.exit(1)
    
    # 检查数据目录
    if not Path(args.data_dir).exists():
        print(f"❌ Data directory not found: {args.data_dir}")
        sys.exit(1)
    
    collect_training_samples(args.data_dir, args.output, args.pixly_bin, args.limit)

if __name__ == '__main__':
    main()
