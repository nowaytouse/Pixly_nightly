#!/usr/bin/env python3
"""
🔧 特征标准化脚本
对训练数据进行标准化处理，提升模型性能
"""

import sys
import json
import numpy as np
from pathlib import Path
from sklearn.preprocessing import StandardScaler, MinMaxScaler
import pickle

def load_data(data_path: str):
    """加载训练数据"""
    print(f"📂 加载数据: {data_path}")
    
    with open(data_path, 'r') as f:
        data = json.load(f)
    
    print(f"  ✅ 加载 {len(data)} 个样本")
    return data

def normalize_features(data: list, method: str = 'standard'):
    """标准化特征"""
    print(f"\n🔧 标准化特征 (方法: {method})...")
    
    # 提取特征矩阵
    X = np.array([sample['features'] for sample in data])
    print(f"  原始特征范围: [{X.min():.2f}, {X.max():.2f}]")
    
    # 选择标准化方法
    if method == 'standard':
        scaler = StandardScaler()
        print(f"  使用StandardScaler (均值=0, 标准差=1)")
    elif method == 'minmax':
        scaler = MinMaxScaler()
        print(f"  使用MinMaxScaler (范围[0, 1])")
    else:
        raise ValueError(f"Unknown method: {method}")
    
    # 拟合并转换
    X_normalized = scaler.fit_transform(X)
    print(f"  标准化后范围: [{X_normalized.min():.2f}, {X_normalized.max():.2f}]")
    
    # 更新数据
    for i, sample in enumerate(data):
        sample['features'] = X_normalized[i].tolist()
    
    return data, scaler

def save_normalized_data(data: list, output_path: str):
    """保存标准化后的数据"""
    print(f"\n💾 保存标准化数据: {output_path}")
    
    with open(output_path, 'w') as f:
        json.dump(data, f)
    
    size_mb = Path(output_path).stat().st_size / (1024 * 1024)
    print(f"  ✅ 已保存 ({size_mb:.1f} MB)")

def save_scaler(scaler, output_path: str):
    """保存scaler对象"""
    print(f"\n💾 保存scaler: {output_path}")
    
    with open(output_path, 'wb') as f:
        pickle.dump(scaler, f)
    
    print(f"  ✅ 已保存")

def verify_normalization(data: list):
    """验证标准化效果"""
    print(f"\n✅ 验证标准化效果...")
    
    X = np.array([sample['features'] for sample in data])
    
    print(f"  特征统计:")
    print(f"    均值: {X.mean():.4f}")
    print(f"    标准差: {X.std():.4f}")
    print(f"    最小值: {X.min():.4f}")
    print(f"    最大值: {X.max():.4f}")
    
    # 检查每个特征维度
    feature_means = X.mean(axis=0)
    feature_stds = X.std(axis=0)
    
    print(f"\n  各维度统计:")
    print(f"    均值范围: [{feature_means.min():.4f}, {feature_means.max():.4f}]")
    print(f"    标准差范围: [{feature_stds.min():.4f}, {feature_stds.max():.4f}]")

def main():
    """主函数"""
    print("=" * 70)
    print("🔧 Pixly ML特征标准化")
    print("=" * 70)
    
    # 配置
    input_path = "models/training_data_final.json"
    output_path = "models/training_data_normalized.json"
    scaler_path = "models/feature_scaler.pkl"
    method = 'standard'  # 或 'minmax'
    
    # 加载数据
    data = load_data(input_path)
    
    # 标准化
    data_normalized, scaler = normalize_features(data, method=method)
    
    # 验证
    verify_normalization(data_normalized)
    
    # 保存
    save_normalized_data(data_normalized, output_path)
    save_scaler(scaler, scaler_path)
    
    print("\n" + "=" * 70)
    print("✅ 特征标准化完成")
    print("=" * 70)
    print(f"\n📊 生成的文件:")
    print(f"  - {output_path}")
    print(f"  - {scaler_path}")
    print(f"\n💡 下一步: 使用标准化数据重新训练模型")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
