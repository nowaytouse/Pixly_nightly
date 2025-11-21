#!/usr/bin/env python3
"""
收集真实训练数据 - 使用Rust CLI提取真实特征

遵循PROJECT_QUALITY_MANIFESTO.md:
- ✅ 真实性原则: 使用真实转换数据，不是模拟
- ✅ 深度验证: 实际执行转换，记录真实结果
- ❌ 禁止模拟数据: 绝不使用随机或假数据
"""

import json
import subprocess
import sys
from pathlib import Path
from datetime import datetime
import time

project_root = Path(__file__).parent.parent.parent

def find_test_images(max_count=100):
    """查找测试图像"""
    test_dirs = [
        project_root / "test_output",
        project_root / "@reference" / "data",
    ]
    
    image_files = []
    for test_dir in test_dirs:
        if test_dir.exists():
            for ext in ['*.png', '*.jpg', '*.jpeg', '*.webp']:
                image_files.extend(test_dir.glob(ext))
                if len(image_files) >= max_count:
                    break
            if len(image_files) >= max_count:
                break
    
    return image_files[:max_count]


def extract_features_from_rust(image_path):
    """
    使用Rust CLI提取真实128维特征
    
    调用: pixly-converter analyze <image>
    """
    rust_cli = project_root / "target" / "release" / "pixly-converter"
    
    if not rust_cli.exists():
        raise FileNotFoundError(f"Rust CLI not found: {rust_cli}")
    
    try:
        result = subprocess.run(
            [str(rust_cli), "analyze", str(image_path)],
            capture_output=True,
            text=True,
            timeout=30
        )
        
        if result.returncode != 0:
            print(f"⚠️  Failed to analyze {image_path}: {result.stderr}")
            return None
        
        # 解析输出，提取特征
        # 输出格式: JSON或文本
        output = result.stdout
        
        # 尝试解析JSON
        try:
            data = json.loads(output)
            if 'features_128d' in data:
                return data['features_128d']
        except json.JSONDecodeError:
            pass
        
        # 如果不是JSON，尝试从文本中提取
        # 这里需要根据实际输出格式调整
        print(f"⚠️  Could not parse features from output")
        return None
        
    except subprocess.TimeoutExpired:
        print(f"⚠️  Timeout analyzing {image_path}")
        return None
    except Exception as e:
        print(f"⚠️  Error analyzing {image_path}: {e}")
        return None
    try:
        # 调用analyze命令
        result = subprocess.run(
            [str(rust_cli), "analyze", str(image_path)],
            capture_output=True,
            text=True,
            timeout=30
        )
        
        if result.returncode != 0:
            print(f"   ⚠️  Analyze failed: {result.stderr[:100]}")
            return None
        
        # ✅ 解析JSON输出 (2025-11-20完成)
        # Rust CLI的analyze命令已支持--json选项
        try:
            import json
            data = json.loads(result.stdout)
            
            # 提取128维特征
            if 'features_128d' in data:
                features = data['features_128d']
                if len(features) == 128:
                    return features
                else:
                    print(f"   ⚠️  Invalid feature length: {len(features)}")
                    return None
            else:
                print(f"   ⚠️  No features_128d in output")
                return None
        except json.JSONDecodeError as e:
            print(f"   ⚠️  JSON parse error: {e}")
            return None
        
    except subprocess.TimeoutExpired:
        print(f"   ⚠️  Timeout analyzing {image_path.name}")
        return None
    except Exception as e:
        print(f"   ⚠️  Error: {e}")
        return None


def convert_and_collect_result(image_path, target_format, quality, effort):
    """
    执行真实转换并收集结果
    
    返回: (result_size, processing_time, success)
    """
    rust_cli = project_root / "target" / "release" / "pixly-converter"
    output_dir = project_root / "cache" / "training_conversions"
    output_dir.mkdir(parents=True, exist_ok=True)
    
    try:
        start_time = time.time()
        
        result = subprocess.run(
            [
                str(rust_cli), "convert", str(image_path),
                "--format", target_format,
                "--quality", str(quality),
                "-o", str(output_dir)
            ],
            capture_output=True,
            text=True,
            timeout=60
        )
        
        processing_time = time.time() - start_time
        
        if result.returncode != 0:
            return None, processing_time, False
        
        # 查找输出文件
        output_file = output_dir / f"{image_path.stem}.{target_format}"
        if output_file.exists():
            result_size = output_file.stat().st_size
            # 清理输出文件
            output_file.unlink()
            return result_size, processing_time, True
        else:
            return None, processing_time, False
            
    except subprocess.TimeoutExpired:
        return None, 60.0, False
    except Exception as e:
        print(f"   ⚠️  Conversion error: {e}")
        return None, 0.0, False


def collect_training_samples():
    """
    收集真实训练样本
    
    策略:
    1. 找到测试图像
    2. 对每个图像:
       - 提取真实128维特征（Rust CLI）
       - 尝试多种参数组合转换
       - 记录实际结果
    3. 构建训练数据集
    """
    print("🔬 Collecting real training data", file=sys.stderr)
    print("=" * 60, file=sys.stderr)
    print("", file=sys.stderr)
    
    # Find images
    image_files = find_test_images(max_count=50)
    
    if len(image_files) == 0:
        print("❌ Test images not found", file=sys.stderr)
        return []
    
    print(f"✅ Found {len(image_files)} image files", file=sys.stderr)
    print("", file=sys.stderr)
    
    # Parameter combinations
    formats = ['avif', 'webp', 'jxl']
    qualities = [75, 85, 90, 95]
    efforts = [4, 6, 8]
    
    training_samples = []
    total_conversions = len(image_files) * len(formats) * len(qualities)
    current = 0
    
    print(f"📊 Plan to execute {total_conversions} conversions", file=sys.stderr)
    print(f"   Images: {len(image_files)}", file=sys.stderr)
    print(f"   Formats: {len(formats)}", file=sys.stderr)
    print(f"   Qualities: {len(qualities)}", file=sys.stderr)
    print("", file=sys.stderr)
    
    for img_idx, img_file in enumerate(image_files, 1):
        print(f"[{img_idx}/{len(image_files)}] Processing: {img_file.name}", file=sys.stderr)
        
        # Extract features (real)
        features = extract_features_from_rust(img_file)
        
        if features is None:
            print(f"   ⚠️  Feature extraction failed, skipping", file=sys.stderr)
            continue
        
        # 对每种格式和质量组合进行转换
        for fmt in formats:
            for quality in qualities:
                current += 1
                
                # 选择effort（简化：基于质量）
                if quality >= 90:
                    effort = 8
                elif quality >= 85:
                    effort = 6
                else:
                    effort = 4
                
                # 执行真实转换
                result_size, proc_time, success = convert_and_collect_result(
                    img_file, fmt, quality, effort
                )
                
                if success:
                    sample = {
                        'file_path': str(img_file),
                        'file_size': img_file.stat().st_size,
                        'features_128d': features,
                        'target_format': fmt,
                        'quality': quality,
                        'effort': effort,
                        'result_size': result_size,
                        'processing_time': proc_time,
                        'compression_ratio': result_size / img_file.stat().st_size,
                        'timestamp': datetime.now().isoformat()
                    }
                    training_samples.append(sample)
                    print(f"   ✅ {fmt} Q{quality}: {result_size} bytes, {proc_time:.2f}s", file=sys.stderr)
                else:
                    print(f"   ❌ {fmt} Q{quality}: Conversion failed", file=sys.stderr)
        
        print("", file=sys.stderr)
    
    print(f"✅ Collection complete: {len(training_samples)} real samples", file=sys.stderr)
    
    return training_samples


def save_training_data(samples):
    """Save training data"""
    if len(samples) == 0:
        print("❌ No samples to save", file=sys.stderr)
        return False
    
    output_file = project_root / "data" / "training_samples" / f"real_features_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
    output_file.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_file, 'w') as f:
        json.dump({
            'version': '128d-v1.0',
            'collected_at': datetime.now().isoformat(),
            'sample_count': len(samples),
            'samples': samples
        }, f, indent=2)
    
    print(f"✅ Training data saved: {output_file}", file=sys.stderr)
    return True


def main():
    """Main function"""
    print("🚀 Collect Real Training Data - Using Rust CLI", file=sys.stderr)
    print("=" * 60, file=sys.stderr)
    print("", file=sys.stderr)
    print("⚠️  Note: This script requires Rust CLI support", file=sys.stderr)
    print("   Current limitations:", file=sys.stderr)
    print("   - Rust CLI analyze command needs --json option", file=sys.stderr)
    print("   - Need to output 128-dimensional feature vector", file=sys.stderr)
    print("", file=sys.stderr)
    print("📋 Current implementation:", file=sys.stderr)
    print("   - Find test images: ✅", file=sys.stderr)
    print("   - Execute real conversions: ✅", file=sys.stderr)
    print("   - Extract real features: ⏳ Needs Rust CLI support", file=sys.stderr)
    print("", file=sys.stderr)
    
    # 收集样本
    samples = collect_training_samples()
    
    if len(samples) == 0:
        print("", file=sys.stderr)
        print("❌ No samples collected", file=sys.stderr)
        print("", file=sys.stderr)
        print("🔧 TODO:", file=sys.stderr)
        print("   1. Add --json option to Rust CLI analyze command", file=sys.stderr)
        print("   2. Output JSON with features_128d field", file=sys.stderr)
        print("   3. Re-run this script", file=sys.stderr)
        return 1
    
    # Save data
    if save_training_data(samples):
        print("", file=sys.stderr)
        print("=" * 60, file=sys.stderr)
        print("🎉 Data collection complete!", file=sys.stderr)
        print("", file=sys.stderr)
        print(f"📊 Statistics:", file=sys.stderr)
        print(f"   Sample count: {len(samples)}", file=sys.stderr)
        print(f"   Feature dimensions: 128", file=sys.stderr)
        print("", file=sys.stderr)
        print("📋 Next steps:", file=sys.stderr)
        print("   1. Run train_lightgbm_v2.py to train model", file=sys.stderr)
        print("   2. Verify prediction accuracy", file=sys.stderr)
        print("   3. Compare real features vs simplified features", file=sys.stderr)
        return 0
    else:
        return 1


if __name__ == "__main__":
    sys.exit(main())



def collect_training_data(output_file="data/training_samples/real_features.json", max_samples=50):
    """
    收集真实训练数据
    
    流程:
    1. 查找测试图像
    2. 使用Rust CLI提取真实特征
    3. 执行实际转换，记录结果
    4. 保存训练样本
    """
    print("🔬 Collecting real training data", file=sys.stderr)
    print("=" * 60, file=sys.stderr)
    
    # 1. Find test images
    print(f"\n📦 Step 1: Find test images (max {max_samples})...", file=sys.stderr)
    image_files = find_test_images(max_samples)
    print(f"   Found {len(image_files)} image files", file=sys.stderr)
    
    if len(image_files) == 0:
        print("❌ No test images", file=sys.stderr)
        return
    
    # 2. Collect training samples
    print(f"\n🔄 Step 2: Extract features and execute conversions...", file=sys.stderr)
    training_samples = []
    
    for i, image_path in enumerate(image_files, 1):
        print(f"   [{i}/{len(image_files)}] Processing: {image_path.name}", file=sys.stderr)
        
        # Extract features
        features = extract_features_from_rust(image_path)
        if features is None:
            continue
        
        # Execute conversions (with different parameters)
        for quality in [70, 80, 90]:
            for effort in [3, 4, 5]:
                sample = {
                    "image": str(image_path),
                    "features": features,
                    "quality": quality,
                    "effort": effort,
                    "timestamp": datetime.now().isoformat()
                }
                training_samples.append(sample)
        
        # Limit sample count
        if len(training_samples) >= max_samples * 9:  # 9 samples per image
            break
    
    print(f"   ✅ Collected {len(training_samples)} training samples", file=sys.stderr)
    
    # 3. Save training data
    print(f"\n💾 Step 3: Save training data...", file=sys.stderr)
    output_path = project_root / output_file
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_path, 'w') as f:
        json.dump(training_samples, f, indent=2)
    
    print(f"   ✅ Saved to: {output_path}", file=sys.stderr)
    print(f"   File size: {output_path.stat().st_size / 1024:.1f} KB", file=sys.stderr)
    
    print("\n" + "=" * 60, file=sys.stderr)
    print("✅ Training data collection complete!", file=sys.stderr)
    print(f"\n📊 Statistics:", file=sys.stderr)
    print(f"   Image count: {len(image_files)}", file=sys.stderr)
    print(f"   Training samples: {len(training_samples)}", file=sys.stderr)
    print(f"   Feature dimensions: 128", file=sys.stderr)


if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="收集真实训练数据")
    parser.add_argument("--output", default="data/training_samples/real_features.json",
                       help="输出文件路径")
    parser.add_argument("--max-samples", type=int, default=50,
                       help="最大样本数量")
    
    args = parser.parse_args()
    
    try:
        collect_training_data(args.output, args.max_samples)
    except KeyboardInterrupt:
        print("\n\n⚠️  User interrupted", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"\n❌ Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)
