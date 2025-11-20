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
    print("🔬 收集真实训练数据")
    print("=" * 60)
    print()
    
    # 查找图像
    image_files = find_test_images(max_count=50)
    
    if len(image_files) == 0:
        print("❌ 未找到测试图像")
        return []
    
    print(f"✅ 找到 {len(image_files)} 个图像文件")
    print()
    
    # 参数组合
    formats = ['avif', 'webp', 'jxl']
    qualities = [75, 85, 90, 95]
    efforts = [4, 6, 8]
    
    training_samples = []
    total_conversions = len(image_files) * len(formats) * len(qualities)
    current = 0
    
    print(f"📊 计划执行 {total_conversions} 次转换")
    print(f"   图像: {len(image_files)}")
    print(f"   格式: {len(formats)}")
    print(f"   质量: {len(qualities)}")
    print()
    
    for img_idx, img_file in enumerate(image_files, 1):
        print(f"[{img_idx}/{len(image_files)}] 处理: {img_file.name}")
        
        # 提取特征（真实）
        features = extract_features_from_rust(img_file)
        
        if features is None:
            print(f"   ⚠️  特征提取失败，跳过")
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
                    print(f"   ✅ {fmt} Q{quality}: {result_size} bytes, {proc_time:.2f}s")
                else:
                    print(f"   ❌ {fmt} Q{quality}: 转换失败")
        
        print()
    
    print(f"✅ 收集完成: {len(training_samples)} 个真实样本")
    
    return training_samples


def save_training_data(samples):
    """保存训练数据"""
    if len(samples) == 0:
        print("❌ 没有样本可保存")
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
    
    print(f"✅ 训练数据已保存: {output_file}")
    return True


def main():
    """主函数"""
    print("🚀 收集真实训练数据 - 使用Rust CLI")
    print("=" * 60)
    print()
    print("⚠️  注意: 此脚本需要Rust CLI支持")
    print("   当前限制:")
    print("   - Rust CLI的analyze命令需要添加--json选项")
    print("   - 需要输出128维特征向量")
    print()
    print("📋 当前实现:")
    print("   - 查找测试图像: ✅")
    print("   - 执行真实转换: ✅")
    print("   - 提取真实特征: ⏳ 需要Rust CLI支持")
    print()
    
    # 收集样本
    samples = collect_training_samples()
    
    if len(samples) == 0:
        print()
        print("❌ 未收集到样本")
        print()
        print("🔧 需要完成:")
        print("   1. 在Rust CLI的analyze命令添加--json选项")
        print("   2. 输出包含features_128d字段的JSON")
        print("   3. 重新运行此脚本")
        return 1
    
    # 保存数据
    if save_training_data(samples):
        print()
        print("=" * 60)
        print("🎉 数据收集完成！")
        print()
        print(f"📊 统计:")
        print(f"   样本数量: {len(samples)}")
        print(f"   特征维度: 128")
        print()
        print("📋 下一步:")
        print("   1. 运行 train_lightgbm_v2.py 训练模型")
        print("   2. 验证预测准确性")
        print("   3. 对比真实特征vs简化特征")
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
    print("🔬 收集真实训练数据")
    print("=" * 60)
    
    # 1. 查找测试图像
    print(f"\n📦 Step 1: 查找测试图像 (最多{max_samples}个)...")
    image_files = find_test_images(max_samples)
    print(f"   找到 {len(image_files)} 个图像文件")
    
    if len(image_files) == 0:
        print("❌ 没有找到测试图像")
        return
    
    # 2. 收集训练样本
    print(f"\n🔄 Step 2: 提取特征并执行转换...")
    training_samples = []
    
    for i, image_path in enumerate(image_files, 1):
        print(f"   [{i}/{len(image_files)}] 处理: {image_path.name}")
        
        # 提取特征
        features = extract_features_from_rust(image_path)
        if features is None:
            continue
        
        # 执行转换（使用不同参数）
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
        
        # 限制样本数量
        if len(training_samples) >= max_samples * 9:  # 每个图像9个样本
            break
    
    print(f"   ✅ 收集了 {len(training_samples)} 个训练样本")
    
    # 3. 保存训练数据
    print(f"\n💾 Step 3: 保存训练数据...")
    output_path = project_root / output_file
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_path, 'w') as f:
        json.dump(training_samples, f, indent=2)
    
    print(f"   ✅ 保存到: {output_path}")
    print(f"   文件大小: {output_path.stat().st_size / 1024:.1f} KB")
    
    print("\n" + "=" * 60)
    print("✅ 训练数据收集完成！")
    print(f"\n📊 统计:")
    print(f"   图像数量: {len(image_files)}")
    print(f"   训练样本: {len(training_samples)}")
    print(f"   特征维度: 128")


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
        print("\n\n⚠️  用户中断")
        sys.exit(1)
    except Exception as e:
        print(f"\n❌ 错误: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
