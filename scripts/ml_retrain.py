#!/usr/bin/env python3
"""
🚀 重新训练LightGBM模型
使用标准化后的特征数据
"""

import sys
import json
import numpy as np
from pathlib import Path
import time

def load_normalized_data(data_path: str = "models/training_data_normalized.json"):
    """加载标准化数据"""
    print(f"📂 加载标准化数据: {data_path}")
    
    with open(data_path, 'r') as f:
        data = json.load(f)
    
    # 分割训练集和测试集 (80/20)
    split_idx = int(len(data) * 0.8)
    train_data = data[:split_idx]
    test_data = data[split_idx:]
    
    print(f"  ✅ 训练集: {len(train_data)} 样本")
    print(f"  ✅ 测试集: {len(test_data)} 样本")
    
    return train_data, test_data

def prepare_dataset(data: list, target: str):
    """准备训练数据"""
    X = np.array([sample['features'] for sample in data])
    y = np.array([sample[target] for sample in data])
    return X, y

def train_lightgbm_model(X_train, y_train, X_test, y_test, 
                        target_name: str, format_name: str = "all"):
    """训练LightGBM模型"""
    print(f"\n🌳 训练LightGBM模型: {format_name}_{target_name}")
    
    try:
        import lightgbm as lgb
        
        # 创建数据集
        train_dataset = lgb.Dataset(X_train, label=y_train)
        test_dataset = lgb.Dataset(X_test, label=y_test, reference=train_dataset)
        
        # 超参数
        params = {
            'objective': 'regression',
            'metric': 'mae',
            'num_leaves': 31,
            'learning_rate': 0.05,
            'feature_fraction': 0.9,
            'bagging_fraction': 0.8,
            'bagging_freq': 5,
            'verbose': -1,
            'min_data_in_leaf': 20,
            'max_depth': 7,
        }
        
        # 训练
        print(f"  训练中...")
        start_time = time.time()
        
        model = lgb.train(
            params,
            train_dataset,
            num_boost_round=200,
            valid_sets=[train_dataset, test_dataset],
            valid_names=['train', 'test'],
            callbacks=[
                lgb.early_stopping(stopping_rounds=20),
                lgb.log_evaluation(period=50)
            ]
        )
        
        training_time = time.time() - start_time
        
        # 评估
        y_pred = model.predict(X_test)
        mae = np.mean(np.abs(y_pred - y_test))
        
        print(f"\n  📊 训练结果:")
        print(f"     训练时间: {training_time:.2f}s")
        print(f"     测试集MAE: {mae:.2f}")
        print(f"     最佳迭代: {model.best_iteration}")
        
        # 保存模型
        model_path = f"models/lightgbm_{format_name}_{target_name}_v2.txt"
        model.save_model(model_path)
        print(f"  ✅ 模型已保存: {model_path}")
        
        return {
            'mae': float(mae),
            'training_time': training_time,
            'best_iteration': model.best_iteration,
            'model_path': model_path
        }
        
    except Exception as e:
        print(f"  ❌ 训练失败: {e}")
        import traceback
        traceback.print_exc()
        return None

def main():
    """主函数"""
    print("=" * 70)
    print("🚀 Pixly ML模型重新训练")
    print("=" * 70)
    
    # 加载数据
    train_data, test_data = load_normalized_data()
    
    # 准备数据集
    X_train_quality, y_train_quality = prepare_dataset(train_data, 'quality')
    X_test_quality, y_test_quality = prepare_dataset(test_data, 'quality')
    
    X_train_effort, y_train_effort = prepare_dataset(train_data, 'effort')
    X_test_effort, y_test_effort = prepare_dataset(test_data, 'effort')
    
    # 训练模型
    results = {}
    
    # Quality模型
    quality_result = train_lightgbm_model(
        X_train_quality, y_train_quality,
        X_test_quality, y_test_quality,
        'quality', 'all'
    )
    if quality_result:
        results['quality'] = quality_result
    
    # Effort模型
    effort_result = train_lightgbm_model(
        X_train_effort, y_train_effort,
        X_test_effort, y_test_effort,
        'effort', 'all'
    )
    if effort_result:
        results['effort'] = effort_result
    
    # 保存训练结果
    timestamp = time.strftime('%Y%m%d_%H%M%S')
    results_path = f"models/training_results_v2_{timestamp}.json"
    
    with open(results_path, 'w') as f:
        json.dump({
            'timestamp': time.strftime('%Y-%m-%d %H:%M:%S'),
            'results': results
        }, f, indent=2)
    
    print("\n" + "=" * 70)
    print("✅ 模型训练完成")
    print("=" * 70)
    
    if results:
        print(f"\n📊 训练结果总结:")
        for target, result in results.items():
            print(f"\n  {target.upper()}:")
            print(f"    MAE: {result['mae']:.2f}")
            print(f"    训练时间: {result['training_time']:.2f}s")
            print(f"    模型: {result['model_path']}")
    
    print(f"\n💾 结果已保存: {results_path}")
    print(f"\n💡 下一步: 运行 ml_evaluate.py 重新评估性能")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
