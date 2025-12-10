#!/usr/bin/env python3
"""
🤖 LightGBM模型训练脚本 V2 - Phase 3.2 正确实现

训练目标：
- 输入：128维特征 + 参数（quality, effort）
- 输出：预测输出质量（SSIM、文件大小、处理时间）

模型用途：
- 给定图像特征，预测最佳参数组合
- 在质量和大小之间找到最佳平衡点
"""

import json
import numpy as np
import argparse
from pathlib import Path
from datetime import datetime

def load_training_data(data_file):
    """加载训练数据V2"""
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

def prepare_dataset_v2(samples):
    """
    准备训练数据集V2
    
    输入特征：128维图像特征 + 2维参数（quality, effort）= 130维
    输出目标：
    - SSIM（质量指标）
    - 压缩比（compression_ratio）
    - 处理时间（processing_time）
    """
    X = []
    y_ssim = []
    y_compression = []
    y_time = []
    
    for sample in samples:
        features = sample['features']
        if len(features) != 128:
            continue
        
        # 输入：特征 + 参数
        input_features = features + [
            sample['quality'] / 100.0,  # 归一化到0-1
            sample['effort'] / 10.0     # 归一化到0-1
        ]
        
        X.append(input_features)
        y_ssim.append(sample['ssim'])
        y_compression.append(sample['compression_ratio'])
        y_time.append(sample['processing_time'])
    
    return (np.array(X), 
            np.array(y_ssim), 
            np.array(y_compression), 
            np.array(y_time))

def train_lightgbm_model(X, y, model_name, target_name):
    """训练LightGBM模型"""
    try:
        import lightgbm as lgb
        from sklearn.model_selection import train_test_split
        from sklearn.metrics import mean_squared_error, r2_score
        
        print(f"\n🤖 Training {model_name} model (predicting {target_name})...")
        
        # 分割数据
        X_train, X_test, y_train, y_test = train_test_split(
            X, y, test_size=0.2, random_state=42
        )
        
        print(f"   Train: {len(X_train)} samples")
        print(f"   Test: {len(X_test)} samples")
        print(f"   Target range: [{y.min():.4f}, {y.max():.4f}]")
        print(f"   Target mean: {y.mean():.4f}")
        
        # 训练模型
        model = lgb.LGBMRegressor(
            n_estimators=300,
            learning_rate=0.03,
            max_depth=8,
            num_leaves=31,
            min_child_samples=20,
            subsample=0.8,
            colsample_bytree=0.8,
            random_state=42,
            verbose=-1
        )
        
        model.fit(X_train, y_train)
        
        # 评估
        train_pred = model.predict(X_train)
        test_pred = model.predict(X_test)
        
        train_r2 = r2_score(y_train, train_pred)
        test_r2 = r2_score(y_test, test_pred)
        train_rmse = np.sqrt(mean_squared_error(y_train, train_pred))
        test_rmse = np.sqrt(mean_squared_error(y_test, test_pred))
        
        print(f"   Train R²: {train_r2:.4f}, RMSE: {train_rmse:.4f}")
        print(f"   Test R²: {test_r2:.4f}, RMSE: {test_rmse:.4f}")
        
        # 特征重要性
        feature_importance = model.feature_importances_
        top_features = np.argsort(feature_importance)[-10:][::-1]
        print(f"   Top 10 important features: {top_features.tolist()}")
        
        return model, test_r2, test_rmse
        
    except ImportError:
        print("❌ LightGBM not installed")
        print("   Install: pip install lightgbm scikit-learn")
        return None, 0.0, 0.0

def main():
    parser = argparse.ArgumentParser(description="Train LightGBM models V2")
    parser.add_argument('--data', default='models/training_data_v2.json', 
                       help='Training data file')
    parser.add_argument('--output-dir', default='models', 
                       help='Output directory')
    parser.add_argument('--format', help='Train for specific format (avif/webp/jxl)')
    
    args = parser.parse_args()
    
    # 检查数据文件
    if not Path(args.data).exists():
        print(f"❌ Training data not found: {args.data}")
        print(f"   Run: python3 scripts/collect_training_data_v2.py")
        return
    
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
        X, y_ssim, y_compression, y_time = prepare_dataset_v2(samples)
        
        if len(X) < 20:
            print(f"⚠️ Not enough samples ({len(X)}), need at least 20")
            continue
        
        # 训练SSIM预测模型（质量指标）
        ssim_model, ssim_r2, ssim_rmse = train_lightgbm_model(
            X, y_ssim, f"{fmt}_ssim", "SSIM quality"
        )
        
        if ssim_model:
            model_path = output_dir / f"lightgbm_{fmt}_ssim.txt"
            ssim_model.booster_.save_model(str(model_path))
            print(f"   💾 Saved: {model_path}")
        
        # 训练压缩比预测模型
        compression_model, comp_r2, comp_rmse = train_lightgbm_model(
            X, y_compression, f"{fmt}_compression", "compression ratio"
        )
        
        if compression_model:
            model_path = output_dir / f"lightgbm_{fmt}_compression.txt"
            compression_model.booster_.save_model(str(model_path))
            print(f"   💾 Saved: {model_path}")
        
        # 训练处理时间预测模型
        time_model, time_r2, time_rmse = train_lightgbm_model(
            X, y_time, f"{fmt}_time", "processing time"
        )
        
        if time_model:
            model_path = output_dir / f"lightgbm_{fmt}_time.txt"
            time_model.booster_.save_model(str(model_path))
            print(f"   💾 Saved: {model_path}")
        
        results[fmt] = {
            'ssim_r2': ssim_r2,
            'ssim_rmse': ssim_rmse,
            'compression_r2': comp_r2,
            'compression_rmse': comp_rmse,
            'time_r2': time_r2,
            'time_rmse': time_rmse,
            'samples': len(X)
        }
    
    # 保存训练结果
    results_file = output_dir / f"training_results_v2_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
    with open(results_file, 'w') as f:
        json.dump(results, f, indent=2)
    
    print(f"\n{'='*60}")
    print(f"✅ Training complete!")
    print(f"{'='*60}")
    print(f"\n📊 Results:")
    for fmt, result in results.items():
        print(f"   {fmt}:")
        print(f"      SSIM R²: {result['ssim_r2']:.4f}, RMSE: {result['ssim_rmse']:.4f}")
        print(f"      Compression R²: {result['compression_r2']:.4f}, RMSE: {result['compression_rmse']:.4f}")
        print(f"      Time R²: {result['time_r2']:.4f}, RMSE: {result['time_rmse']:.4f}")
        print(f"      Samples: {result['samples']}")
    
    print(f"\n💾 Results saved to: {results_file}")
    print(f"\n🎯 Next steps:")
    print(f"   1. Check R² scores (should be >0.5 to be meaningful)")
    print(f"   2. If R² is too low, collect more training data")
    print(f"   3. Integrate these models into ml_bridge.py")

if __name__ == '__main__':
    main()
