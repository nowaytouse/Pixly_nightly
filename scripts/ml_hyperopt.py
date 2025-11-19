#!/usr/bin/env python3
"""超参数优化 - 网格搜索"""
import sys
import json
import numpy as np
from pathlib import Path
import lightgbm as lgb
from sklearn.model_selection import train_test_split
import itertools

def load_data():
    with open("models/training_data_normalized.json", 'r') as f:
        data = json.load(f)
    X = np.array([s['features'] for s in data])
    y_quality = np.array([s['quality'] for s in data])
    y_effort = np.array([s['effort'] for s in data])
    return train_test_split(X, y_quality, y_effort, test_size=0.2, random_state=42)

def grid_search(X_train, y_train, X_test, y_test, target_name):
    print(f"\n🔍 网格搜索: {target_name}")
    
    # 参数网格
    param_grid = {
        'learning_rate': [0.01, 0.05, 0.1],
        'num_leaves': [15, 31, 63],
        'max_depth': [5, 7, 9],
        'min_data_in_leaf': [10, 20, 30],
    }
    
    best_mae = float('inf')
    best_params = None
    
    # 网格搜索
    total = np.prod([len(v) for v in param_grid.values()])
    count = 0
    
    for lr in param_grid['learning_rate']:
        for leaves in param_grid['num_leaves']:
            for depth in param_grid['max_depth']:
                for min_data in param_grid['min_data_in_leaf']:
                    count += 1
                    
                    params = {
                        'objective': 'regression',
                        'metric': 'mae',
                        'learning_rate': lr,
                        'num_leaves': leaves,
                        'max_depth': depth,
                        'min_data_in_leaf': min_data,
                        'verbose': -1,
                    }
                    
                    train_data = lgb.Dataset(X_train, label=y_train)
                    test_data = lgb.Dataset(X_test, label=y_test, reference=train_data)
                    
                    model = lgb.train(
                        params,
                        train_data,
                        num_boost_round=100,
                        valid_sets=[test_data],
                        callbacks=[lgb.early_stopping(20)]
                    )
                    
                    y_pred = model.predict(X_test)
                    mae = np.mean(np.abs(y_pred - y_test))
                    
                    if mae < best_mae:
                        best_mae = mae
                        best_params = params.copy()
                        print(f"  [{count}/{total}] 新最佳: MAE={mae:.2f}, lr={lr}, leaves={leaves}, depth={depth}")
    
    print(f"\n  ✅ 最佳参数: MAE={best_mae:.2f}")
    print(f"     {best_params}")
    
    return best_params, best_mae

def main():
    print("🔍 超参数优化")
    
    X_train, X_test, y_quality_train, y_quality_test, y_effort_train, y_effort_test = load_data()
    
    # Quality优化
    quality_params, quality_mae = grid_search(X_train, y_quality_train, X_test, y_quality_test, "quality")
    
    # Effort优化
    effort_params, effort_mae = grid_search(X_train, y_effort_train, X_test, y_effort_test, "effort")
    
    # 保存结果
    results = {
        'quality': {'params': quality_params, 'mae': quality_mae},
        'effort': {'params': effort_params, 'mae': effort_mae}
    }
    
    with open('models/hyperopt_results.json', 'w') as f:
        json.dump(results, f, indent=2)
    
    print(f"\n✅ 优化完成，结果已保存")
    return 0

if __name__ == "__main__":
    sys.exit(main())
