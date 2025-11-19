#!/usr/bin/env python3
"""
🔍 特征重要性分析
使用SHAP和LightGBM内置方法分析特征贡献
"""

import sys
import json
import numpy as np
from pathlib import Path
from typing import Dict, List

# matplotlib是可选的
try:
    import matplotlib
    matplotlib.use('Agg')  # 无GUI后端
    import matplotlib.pyplot as plt
    HAS_MATPLOTLIB = True
except ImportError:
    HAS_MATPLOTLIB = False
    print("⚠️  matplotlib未安装，将跳过绘图功能")

# 添加scripts目录到路径
sys.path.insert(0, str(Path(__file__).parent))

from ml_bridge import StandardFeatures

def load_training_data(data_path: str = "models/training_data_final.json") -> tuple:
    """加载训练数据"""
    print(f"📂 加载训练数据: {data_path}")
    
    with open(data_path, 'r') as f:
        data = json.load(f)
    
    # 提取特征和标签
    X = np.array([sample['features'] for sample in data])
    y_quality = np.array([sample['quality'] for sample in data])
    y_effort = np.array([sample['effort'] for sample in data])
    
    print(f"  ✅ 加载 {len(X)} 个样本, 128维特征")
    return X, y_quality, y_effort

def analyze_lightgbm_importance(X: np.ndarray, y: np.ndarray, 
                                target_name: str = "quality") -> Dict:
    """使用LightGBM内置方法分析特征重要性"""
    print(f"\n🌳 分析LightGBM特征重要性 ({target_name})...")
    
    try:
        import lightgbm as lgb
        
        # 训练模型
        print("  训练模型...")
        train_data = lgb.Dataset(X, label=y)
        params = {
            'objective': 'regression',
            'metric': 'mae',
            'verbose': -1,
            'num_leaves': 31,
            'learning_rate': 0.05,
        }
        
        model = lgb.train(params, train_data, num_boost_round=100)
        
        # 获取特征重要性
        importance = model.feature_importance(importance_type='gain')
        
        # 特征名称
        feature_names = get_feature_names()
        
        # 排序
        indices = np.argsort(importance)[::-1]
        
        print(f"\n  📊 Top 20 重要特征:")
        for i in range(min(20, len(indices))):
            idx = indices[i]
            print(f"     {i+1:2d}. {feature_names[idx]:<40} {importance[idx]:>10.2f}")
        
        return {
            'importance': importance.tolist(),
            'feature_names': feature_names,
            'top_20_indices': indices[:20].tolist()
        }
        
    except Exception as e:
        print(f"  ❌ 分析失败: {e}")
        import traceback
        traceback.print_exc()
        return None

def get_feature_names() -> List[str]:
    """获取128维特征的名称"""
    names = []
    
    # Basic features (16)
    basic_names = ['width', 'height', 'aspect_ratio', 'resolution', 
                   'file_size', 'bit_depth', 'has_alpha', 'is_animated',
                   'frame_count', 'duration', 'format_id', 'compression',
                   'reserved1', 'reserved2', 'reserved3', 'reserved4']
    names.extend([f'basic_{n}' for n in basic_names])
    
    # Color features (16)
    color_names = ['mean_r', 'mean_g', 'mean_b', 'std_r', 'std_g', 'std_b',
                   'unique_colors', 'color_entropy', 'saturation', 'brightness',
                   'contrast', 'hue_variance', 'color_complexity',
                   'reserved1', 'reserved2', 'reserved3']
    names.extend([f'color_{n}' for n in color_names])
    
    # Texture features (16)
    texture_names = ['edge_density', 'edge_strength', 'gradient_mean', 'gradient_std',
                     'texture_entropy', 'smoothness', 'roughness', 'regularity',
                     'directionality', 'line_likeness', 'coarseness', 'contrast',
                     'reserved1', 'reserved2', 'reserved3', 'reserved4']
    names.extend([f'texture_{n}' for n in texture_names])
    
    # Shape features (16)
    shape_names = ['compactness', 'circularity', 'rectangularity', 'elongation',
                   'convexity', 'solidity', 'extent', 'perimeter',
                   'area_ratio', 'bbox_ratio', 'symmetry_h', 'symmetry_v',
                   'reserved1', 'reserved2', 'reserved3', 'reserved4']
    names.extend([f'shape_{n}' for n in shape_names])
    
    # Quality features (16)
    quality_names = ['noise_level', 'blur_level', 'sharpness', 'clarity',
                     'snr', 'psnr_estimate', 'ssim_estimate', 'artifacts',
                     'compression_artifacts', 'blocking', 'ringing', 'aliasing',
                     'reserved1', 'reserved2', 'reserved3', 'reserved4']
    names.extend([f'quality_{n}' for n in quality_names])
    
    # Metadata features (32)
    for i in range(32):
        names.append(f'metadata_{i}')
    
    # Context features (16)
    for i in range(16):
        names.append(f'context_{i}')
    
    return names

def analyze_feature_groups(importance: np.ndarray) -> Dict:
    """分析特征组的重要性"""
    print(f"\n📦 分析特征组重要性...")
    
    groups = {
        'basic': importance[0:16],
        'color': importance[16:32],
        'texture': importance[32:48],
        'shape': importance[48:64],
        'quality': importance[64:80],
        'metadata': importance[80:112],
        'context': importance[112:128]
    }
    
    group_importance = {name: np.sum(values) for name, values in groups.items()}
    total = sum(group_importance.values())
    
    print(f"\n  特征组贡献:")
    for name, value in sorted(group_importance.items(), key=lambda x: x[1], reverse=True):
        percentage = (value / total) * 100
        print(f"     {name:<12} {value:>10.2f} ({percentage:>5.1f}%)")
    
    return group_importance

def plot_feature_importance(importance: np.ndarray, feature_names: List[str],
                           output_path: str, top_n: int = 30):
    """绘制特征重要性图"""
    if not HAS_MATPLOTLIB:
        print(f"\n⏭️  跳过绘图 (matplotlib未安装)")
        return
    
    print(f"\n📊 绘制特征重要性图...")
    
    try:
        # 选择top N特征
        indices = np.argsort(importance)[-top_n:]
        
        plt.figure(figsize=(12, 10))
        plt.barh(range(top_n), importance[indices])
        plt.yticks(range(top_n), [feature_names[i] for i in indices])
        plt.xlabel('Feature Importance (Gain)')
        plt.title(f'Top {top_n} Most Important Features')
        plt.tight_layout()
        plt.savefig(output_path, dpi=150, bbox_inches='tight')
        plt.close()
        
        print(f"  ✅ 图表已保存: {output_path}")
        
    except Exception as e:
        print(f"  ⚠️  绘图失败: {e}")

def save_analysis_results(results: Dict, output_path: str):
    """保存分析结果"""
    print(f"\n💾 保存分析结果: {output_path}")
    
    with open(output_path, 'w') as f:
        json.dump(results, f, indent=2)
    
    print(f"  ✅ 结果已保存")

def main():
    """主函数"""
    print("=" * 70)
    print("🔍 Pixly ML特征重要性分析")
    print("=" * 70)
    
    # 加载数据
    X, y_quality, y_effort = load_training_data()
    
    # 分析Quality预测的特征重要性
    quality_results = analyze_lightgbm_importance(X, y_quality, "quality")
    
    if quality_results:
        # 分析特征组
        importance = np.array(quality_results['importance'])
        group_importance = analyze_feature_groups(importance)
        quality_results['group_importance'] = group_importance
        
        # 绘图
        plot_feature_importance(
            importance,
            quality_results['feature_names'],
            'models/feature_importance_quality.png'
        )
        
        # 保存结果
        save_analysis_results(
            quality_results,
            'models/feature_importance_quality.json'
        )
    
    # 分析Effort预测的特征重要性
    effort_results = analyze_lightgbm_importance(X, y_effort, "effort")
    
    if effort_results:
        # 分析特征组
        importance = np.array(effort_results['importance'])
        group_importance = analyze_feature_groups(importance)
        effort_results['group_importance'] = group_importance
        
        # 绘图
        plot_feature_importance(
            importance,
            effort_results['feature_names'],
            'models/feature_importance_effort.png'
        )
        
        # 保存结果
        save_analysis_results(
            effort_results,
            'models/feature_importance_effort.json'
        )
    
    print("\n" + "=" * 70)
    print("✅ 特征重要性分析完成")
    print("=" * 70)
    print("\n📊 生成的文件:")
    print("  - models/feature_importance_quality.json")
    print("  - models/feature_importance_quality.png")
    print("  - models/feature_importance_effort.json")
    print("  - models/feature_importance_effort.png")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
