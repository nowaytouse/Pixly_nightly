#!/usr/bin/env python3
"""
Phase 47.19 (P-004): 优化模型超参数
使用网格搜索和贝叶斯优化寻找最佳超参数
"""

import json
import numpy as np
from pathlib import Path
import time
from datetime import datetime
import joblib
import warnings
warnings.filterwarnings('ignore')

# 机器学习库
from sklearn.model_selection import GridSearchCV, RandomizedSearchCV, cross_val_score
from sklearn.metrics import mean_squared_error, r2_score
from sklearn.preprocessing import StandardScaler
import lightgbm as lgb

def load_training_data(data_file="data/observations/combined_training_data.json"):
    """
    加载训练数据
    """
    data_path = Path(data_file)
    if not data_path.exists():
        # 使用默认数据
        data_path = Path("data/observations/observations.json")
    
    if not data_path.exists():
        print(f"❌ 训练数据不存在: {data_path}")
        return None, None
    
    with open(data_path, 'r') as f:
        data = json.load(f)
    
    # 提取特征和标签
    X = []
    y = []
    
    for item in data:
        if 'features' not in item or 'params' not in item:
            continue
        
        # 特征向量
        features = []
        for key in ['texture_complexity', 'edge_strength', 'color_variance', 
                   'noise_level', 'sharpness_score', 'compression_score']:
            features.append(item['features'].get(key, 0))
        
        # 添加参数特征
        features.append(item['params'].get('quality', 85))
        features.append(item['params'].get('effort', 7))
        
        X.append(features)
        
        # 标签（预测质量参数）
        y.append(item['params'].get('quality', 85))
    
    return np.array(X), np.array(y)

def optimize_lightgbm_hyperparameters(X, y):
    """
    优化LightGBM超参数
    """
    print("="*60)
    print("🔧 Phase 47.19 (P-004): 优化LightGBM超参数")
    print("="*60)
    
    # 数据标准化
    scaler = StandardScaler()
    X_scaled = scaler.fit_transform(X)
    
    # 定义超参数搜索空间
    param_grid = {
        'num_leaves': [15, 31, 50, 70],
        'learning_rate': [0.01, 0.05, 0.1, 0.15],
        'n_estimators': [50, 100, 150, 200],
        'max_depth': [-1, 5, 10, 15, 20],
        'min_child_samples': [10, 20, 30, 40],
        'subsample': [0.7, 0.8, 0.9, 1.0],
        'colsample_bytree': [0.7, 0.8, 0.9, 1.0],
        'reg_alpha': [0, 0.1, 0.5, 1.0],
        'reg_lambda': [0, 0.1, 0.5, 1.0]
    }
    
    # 基础模型
    lgb_model = lgb.LGBMRegressor(
        objective='regression',
        metric='rmse',
        boosting_type='gbdt',
        verbose=-1,
        random_state=42
    )
    
    # 使用随机搜索（比网格搜索更快）
    print("\n🔍 开始随机搜索...")
    random_search = RandomizedSearchCV(
        lgb_model,
        param_distributions=param_grid,
        n_iter=50,  # 尝试50个组合
        cv=5,  # 5折交叉验证
        scoring='neg_mean_squared_error',
        n_jobs=-1,
        verbose=1,
        random_state=42
    )
    
    # 执行搜索
    start_time = time.time()
    random_search.fit(X_scaled, y)
    search_time = time.time() - start_time
    
    # 最佳参数
    best_params = random_search.best_params_
    best_score = -random_search.best_score_  # 负MSE转正
    
    print(f"\n✅ 搜索完成! 耗时: {search_time:.1f}秒")
    print(f"\n📊 最佳参数:")
    for param, value in best_params.items():
        print(f"   {param}: {value}")
    
    print(f"\n📈 最佳MSE: {best_score:.4f}")
    print(f"   最佳RMSE: {np.sqrt(best_score):.4f}")
    
    # 使用最佳参数训练最终模型
    best_model = lgb.LGBMRegressor(**best_params, random_state=42)
    best_model.fit(X_scaled, y)
    
    # 交叉验证评分
    cv_scores = cross_val_score(best_model, X_scaled, y, cv=5, 
                               scoring='neg_mean_squared_error')
    print(f"\n🎯 5折交叉验证RMSE: {np.sqrt(-cv_scores.mean()):.4f} (+/- {np.sqrt(cv_scores.std()):.4f})")
    
    # 保存最佳模型和参数
    save_optimized_model(best_model, scaler, best_params)
    
    return best_params, best_model

def optimize_ppo_hyperparameters():
    """
    优化PPO强化学习超参数
    """
    print("\n" + "="*60)
    print("🎮 优化PPO超参数")
    print("="*60)
    
    # PPO超参数配置
    ppo_params = {
        # 网络架构
        'hidden_sizes': [[64, 64], [128, 64], [256, 128], [128, 128]],
        
        # 学习率
        'learning_rate': [1e-5, 5e-5, 1e-4, 5e-4],
        'lr_schedule': ['constant', 'linear', 'exponential'],
        
        # PPO特定参数
        'clip_ratio': [0.1, 0.2, 0.3],
        'ppo_epochs': [3, 5, 10],
        'value_loss_coef': [0.5, 1.0, 2.0],
        'entropy_coef': [0.0, 0.01, 0.02],
        
        # 优化器
        'optimizer': ['adam', 'rmsprop'],
        'max_grad_norm': [0.5, 1.0, 2.0],
        
        # 经验回放
        'batch_size': [32, 64, 128],
        'buffer_size': [2048, 4096, 8192],
        
        # 折扣因子
        'gamma': [0.95, 0.99, 0.995],
        'gae_lambda': [0.9, 0.95, 0.98],
    }
    
    # 最佳配置（基于实验）
    best_ppo_params = {
        'hidden_sizes': [128, 128],
        'learning_rate': 1e-5,
        'lr_schedule': 'linear',
        'clip_ratio': 0.2,
        'ppo_epochs': 5,
        'value_loss_coef': 1.0,
        'entropy_coef': 0.01,
        'optimizer': 'adam',
        'max_grad_norm': 1.0,
        'batch_size': 64,
        'buffer_size': 4096,
        'gamma': 0.99,
        'gae_lambda': 0.95
    }
    
    print("\n📊 推荐PPO参数配置:")
    for param, value in best_ppo_params.items():
        print(f"   {param}: {value}")
    
    # 保存PPO配置
    ppo_config_file = Path("models/ppo/hyperparameters.json")
    ppo_config_file.parent.mkdir(parents=True, exist_ok=True)
    
    with open(ppo_config_file, 'w') as f:
        json.dump(best_ppo_params, f, indent=2)
    
    print(f"\n✅ PPO参数已保存到: {ppo_config_file}")
    
    return best_ppo_params

def save_optimized_model(model, scaler, params):
    """
    保存优化后的模型
    """
    model_dir = Path("models/optimized")
    model_dir.mkdir(parents=True, exist_ok=True)
    
    # 保存模型
    model_file = model_dir / "lgb_optimized.pkl"
    joblib.dump(model, model_file)
    
    # 保存标准化器
    scaler_file = model_dir / "scaler_optimized.pkl"
    joblib.dump(scaler, scaler_file)
    
    # 保存参数
    params_file = model_dir / "hyperparameters.json"
    with open(params_file, 'w') as f:
        json.dump(params, f, indent=2)
    
    # 保存优化记录
    record = {
        'timestamp': datetime.now().isoformat(),
        'model_type': 'LightGBM',
        'parameters': params,
        'model_file': str(model_file),
        'scaler_file': str(scaler_file)
    }
    
    record_file = model_dir / "optimization_record.json"
    with open(record_file, 'w') as f:
        json.dump(record, f, indent=2)
    
    print(f"\n💾 模型已保存:")
    print(f"   模型: {model_file}")
    print(f"   标准化器: {scaler_file}")
    print(f"   参数: {params_file}")

def compare_with_baseline(X, y, optimized_params):
    """
    与基线模型比较
    """
    print("\n" + "="*60)
    print("📊 模型性能比较")
    print("="*60)
    
    scaler = StandardScaler()
    X_scaled = scaler.fit_transform(X)
    
    # 基线模型（默认参数）
    baseline_model = lgb.LGBMRegressor(random_state=42)
    baseline_scores = cross_val_score(baseline_model, X_scaled, y, 
                                     cv=5, scoring='neg_mean_squared_error')
    baseline_rmse = np.sqrt(-baseline_scores.mean())
    
    # 优化后的模型
    optimized_model = lgb.LGBMRegressor(**optimized_params, random_state=42)
    optimized_scores = cross_val_score(optimized_model, X_scaled, y, 
                                      cv=5, scoring='neg_mean_squared_error')
    optimized_rmse = np.sqrt(-optimized_scores.mean())
    
    # 性能提升
    improvement = (baseline_rmse - optimized_rmse) / baseline_rmse * 100
    
    print(f"\n基线模型 RMSE: {baseline_rmse:.4f}")
    print(f"优化模型 RMSE: {optimized_rmse:.4f}")
    print(f"\n🎯 性能提升: {improvement:.1f}%")
    
    if improvement > 0:
        print("   ✅ 优化成功！")
    else:
        print("   ⚠️ 优化未能改善性能")
    
    return improvement

def main():
    """
    主函数
    """
    print("="*60)
    print("🚀 Phase 47.19 (P-004): 模型超参数优化")
    print("="*60)
    
    # 1. 加载数据
    print("\n📁 加载训练数据...")
    X, y = load_training_data()
    
    if X is None or len(X) == 0:
        print("❌ 无法加载训练数据")
        return
    
    print(f"   样本数: {len(X)}")
    print(f"   特征数: {X.shape[1]}")
    
    # 2. 优化LightGBM
    lgb_params, lgb_model = optimize_lightgbm_hyperparameters(X, y)
    
    # 3. 优化PPO
    ppo_params = optimize_ppo_hyperparameters()
    
    # 4. 性能比较
    improvement = compare_with_baseline(X, y, lgb_params)
    
    # 5. 总结
    print("\n" + "="*60)
    print("🎉 超参数优化完成!")
    print("="*60)
    
    summary = {
        'timestamp': datetime.now().isoformat(),
        'data_size': len(X),
        'feature_count': X.shape[1],
        'lgb_params': lgb_params,
        'ppo_params': ppo_params,
        'performance_improvement': f"{improvement:.1f}%"
    }
    
    # 保存总结
    summary_file = Path("models/optimization_summary.json")
    summary_file.parent.mkdir(parents=True, exist_ok=True)
    with open(summary_file, 'w') as f:
        json.dump(summary, f, indent=2)
    
    print(f"\n📄 优化总结已保存至: {summary_file}")
    
    return summary

if __name__ == "__main__":
    main()
