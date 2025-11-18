#!/usr/bin/env python3
"""
🤖 LightGBM模型训练脚本

从收集的训练数据训练LightGBM模型
"""

import json
import numpy as np
import argparse
from pathlib import Path
from datetime import datetime

def load_training_data(data_file):
    """加载训练数据"""
    with open(data_file, 'r') as f:
        samples = json.load(f)
    
    print(f"📊 Loaded {len(samples)} training samples")
    
    # 按格式分组
    by_format = {}
    for sample in samples:
        fmt = sample['target_format']
        if fmt not in by_format:
            by_format[fmt] = []
        by_format[fmt].append(sample)
    
    for fmt, samples_list in by_format.items():
        print(f"   {fmt}: {len(samples_list)} samples")
    
    return samples, by_format

def prepare_dataset(samples):
    """准备训练数据集"""
    X = []
    y_quality = []
    y_effort = []
    y_size = []
    
    for sample in samples:
        features = sample['features']
        if len(features) != 128:
            continue
        
        X.append(features)
        y_quality.append(sample['quality'])
        y_effort.append(sample['effort'])
        y_size.append(sample['output_size'])
    
    return np.array(X), np.array(y_quality), np.array(y_effort), np.array(y_size)

def train_lightgbm_model(X, y, model_name):
    """训练LightGBM模型"""
    try:
        import lightgbm as lgb
        from sklearn.model_selection import train_test_split
        
        print(f"\n🤖 Training {model_name} model...")
        
        # 分割数据
        X_train, X_test, y_train, y_test = train_test_split(
            X, y, test_size=0.2, random_state=42
        )
        
        print(f"   Train: {len(X_train)} samples")
        print(f"   Test: {len(X_test)} samples")
        
        # 训练模型
        model = lgb.LGBMRegressor(
            n_estimators=200,
            learning_rate=0.05,
            max_depth=7,
            num_leaves=31,
            min_child_samples=20,
            random_state=42,
            verbose=-1
        )
        
        model.fit(X_train, y_train)
        
        # 评估
        train_score = model.score(X_train, y_train)
        test_score = model.score(X_test, y_test)
        
        print(f"   Train R²: {train_score:.4f}")
        print(f"   Test R²: {test_score:.4f}")
        
        return model, test_score
        
    except ImportError:
        print("❌ LightGBM not installed")
        print("   Install: pip install lightgbm scikit-learn")
        return None, 0.0

def main():
    parser = argparse.ArgumentParser(description="Train LightGBM models")
    parser.add_argument('--data', default='models/training_data.json', help='Training data file')
    parser.add_argument('--output-dir', default='models', help='Output directory')
    parser.add_argument('--format', help='Train for specific format (avif/webp/jxl)')
    
    args = parser.parse_args()
    
    # 检查数据文件
    if not Path(args.data).exists():
        print(f"❌ Training data not found: {args.data}")
        print(f"   Run: python3 scripts/collect_training_data.py")
        sys.exit(1)
    
    # 加载数据
    all_samples, by_format = load_training_data(args.data)
    
    # 创建输出目录
    output_dir = Path(args.output_dir)
    output_dir.mkdir(exist_ok=True)
    
    # 训练模型
    formats_to_train = [args.format] if args.format else by_format.keys()
    
    results = {}
    
    for fmt in formats_to_train:
        if fmt not in by_format:
            print(f"⚠️ No training data for format: {fmt}")
            continue
        
        print(f"\n{'='*60}")
        print(f"📦 Training models for: {fmt.upper()}")
        print(f"{'='*60}")
        
        samples = by_format[fmt]
        X, y_quality, y_effort, y_size = prepare_dataset(samples)
        
        if len(X) < 10:
            print(f"⚠️ Not enough samples ({len(X)}), skipping")
            continue
        
        # 训练质量预测模型
        quality_model, quality_score = train_lightgbm_model(X, y_quality, f"{fmt}_quality")
        
        if quality_model:
            model_path = output_dir / f"lightgbm_{fmt}_quality.txt"
            quality_model.booster_.save_model(str(model_path))
            print(f"   💾 Saved: {model_path}")
        
        # 训练effort预测模型
        effort_model, effort_score = train_lightgbm_model(X, y_effort, f"{fmt}_effort")
        
        if effort_model:
            model_path = output_dir / f"lightgbm_{fmt}_effort.txt"
            effort_model.booster_.save_model(str(model_path))
            print(f"   💾 Saved: {model_path}")
        
        results[fmt] = {
            'quality_score': quality_score,
            'effort_score': effort_score,
            'samples': len(X)
        }
    
    # 保存训练结果
    results_file = output_dir / f"training_results_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
    with open(results_file, 'w') as f:
        json.dump(results, f, indent=2)
    
    print(f"\n{'='*60}")
    print(f"✅ Training complete!")
    print(f"{'='*60}")
    print(f"\n📊 Results:")
    for fmt, result in results.items():
        print(f"   {fmt}:")
        print(f"      Quality R²: {result['quality_score']:.4f}")
        print(f"      Effort R²: {result['effort_score']:.4f}")
        print(f"      Samples: {result['samples']}")
    
    print(f"\n💾 Results saved to: {results_file}")

if __name__ == '__main__':
    main()
