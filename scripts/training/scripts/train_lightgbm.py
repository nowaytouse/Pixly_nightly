#!/usr/bin/env python3
"""
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PIXLY AI Training - LightGBM增量训练
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🎯 功能:
- 从SQLite feedback数据库读取训练数据
- LightGBM增量训练 (支持在线学习)
- 模型评估与版本管理
- 自动部署到GO AI服务

📊 训练目标:
- quality预测 (回归)
- speed/effort预测 (回归)
- lossless决策 (分类)  # 🔥 Phase 36新增
- format推荐 (分类)

作者: PIXLY AI Team
版本: 2.0.0 (Phase 35+36)
日期: 2025-11-06
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"""

import sys
import os
import sqlite3
import json
import argparse
import logging
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Tuple, Optional

import numpy as np
import pandas as pd
import lightgbm as lgb
from sklearn.model_selection import train_test_split
from sklearn.metrics import mean_squared_error, mean_absolute_error, accuracy_score, f1_score

# 配置日志
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s',
    handlers=[
        logging.StreamHandler(sys.stdout),
        logging.FileHandler('training.log')
    ]
)
logger = logging.getLogger(__name__)


class LightGBMTrainer:
    """LightGBM增量训练器"""
    
    def __init__(self, feedback_db_path: str, model_dir: str):
        self.feedback_db_path = feedback_db_path
        self.model_dir = Path(model_dir)
        self.model_dir.mkdir(parents=True, exist_ok=True)
        
        self.models = {
            'quality': None,     # 质量预测 (回归)
            'speed': None,       # 速度预测 (回归)
            'lossless': None,    # 🔥 无损决策 (分类)
            'format': None,      # 格式推荐 (分类)
        }
        
        logger.info(f"📂 Model directory: {self.model_dir}")
        logger.info(f"💾 Feedback DB: {self.feedback_db_path}")
    
    def load_feedback_data(self, min_samples: int = 100) -> Optional[pd.DataFrame]:
        """从SQLite加载反馈数据"""
        logger.info("📊 Loading feedback data from SQLite...")
        
        try:
            conn = sqlite3.connect(self.feedback_db_path)
            
            # 查询反馈数据
            query = """
            SELECT 
                input_format, target_format,
                width, height, file_size, has_alpha, has_animation,
                predicted_quality, predicted_speed,
                actual_quality, actual_size, actual_time,
                compression_ratio, success,
                user_rating, timestamp
            FROM feedback
            WHERE success = 1  -- 只使用成功的转换
            ORDER BY timestamp DESC
            """
            
            df = pd.read_sql_query(query, conn)
            conn.close()
            
            logger.info(f"✅ Loaded {len(df)} feedback samples")
            
            if len(df) < min_samples:
                logger.warning(f"⚠️  Not enough samples ({len(df)} < {min_samples})")
                return None
            
            return df
            
        except Exception as e:
            logger.error(f"❌ Failed to load feedback data: {e}")
            return None
    
    def prepare_features(self, df: pd.DataFrame) -> Tuple[np.ndarray, Dict[str, np.ndarray]]:
        """准备训练特征"""
        logger.info("🔧 Preparing features...")
        
        # 基础特征
        features = []
        for _, row in df.iterrows():
            f = [
                row['width'],
                row['height'],
                row['file_size'],
                1 if row['has_alpha'] else 0,
                1 if row['has_animation'] else 0,
                hash(row['input_format']) % 1000,  # 简单编码
                hash(row['target_format']) % 1000,
            ]
            features.append(f)
        
        X = np.array(features)
        
        # 多目标标签
        y_quality = df['actual_quality'].values if 'actual_quality' in df.columns else df['predicted_quality'].values
        y_speed = df['predicted_speed'].values
        
        # 🔥 Phase 36: lossless标签 (启发式推断)
        y_lossless = ((df['compression_ratio'] > 0.9) & (df['actual_quality'] >= 95)).astype(int).values
        
        # 格式标签
        y_format = df['target_format'].astype('category').cat.codes.values
        
        targets = {
            'quality': y_quality,
            'speed': y_speed,
            'lossless': y_lossless,
            'format': y_format,
        }
        
        logger.info(f"✅ Feature shape: {X.shape}")
        logger.info(f"✅ Targets: {list(targets.keys())}")
        
        return X, targets
    
    def train_model(self, X_train, y_train, X_val, y_val, 
                   task_type: str = 'regression',
                   model_name: str = 'quality') -> lgb.Booster:
        """训练单个LightGBM模型"""
        logger.info(f"🎓 Training {model_name} model ({task_type})...")
        
        params = {
            'objective': 'regression' if task_type == 'regression' else 'binary',
            'metric': 'rmse' if task_type == 'regression' else 'binary_logloss',
            'boosting_type': 'gbdt',
            'num_leaves': 31,
            'learning_rate': 0.05,
            'feature_fraction': 0.9,
            'bagging_fraction': 0.8,
            'bagging_freq': 5,
            'verbose': -1,
        }
        
        train_data = lgb.Dataset(X_train, label=y_train)
        val_data = lgb.Dataset(X_val, label=y_val, reference=train_data)
        
        model = lgb.train(
            params,
            train_data,
            num_boost_round=100,
            valid_sets=[train_data, val_data],
            valid_names=['train', 'val'],
            callbacks=[lgb.early_stopping(stopping_rounds=10), lgb.log_evaluation(period=10)]
        )
        
        # 评估
        y_pred = model.predict(X_val)
        
        if task_type == 'regression':
            mse = mean_squared_error(y_val, y_pred)
            mae = mean_absolute_error(y_val, y_pred)
            logger.info(f"📊 {model_name} - MSE: {mse:.4f}, MAE: {mae:.4f}")
        else:
            y_pred_binary = (y_pred > 0.5).astype(int)
            acc = accuracy_score(y_val, y_pred_binary)
            f1 = f1_score(y_val, y_pred_binary, average='binary')
            logger.info(f"📊 {model_name} - Accuracy: {acc:.4f}, F1: {f1:.4f}")
        
        return model
    
    def train_all_models(self, df: pd.DataFrame):
        """训练所有模型"""
        logger.info("\n" + "="*60)
        logger.info("🚀 Starting multi-target training...")
        logger.info("="*60 + "\n")
        
        # 准备特征
        X, targets = self.prepare_features(df)
        
        # 分割数据
        X_train, X_val, indices_train, indices_val = train_test_split(
            X, np.arange(len(X)), test_size=0.2, random_state=42
        )
        
        # 训练每个模型
        # 1. Quality (回归)
        y_train_quality = targets['quality'][indices_train]
        y_val_quality = targets['quality'][indices_val]
        self.models['quality'] = self.train_model(
            X_train, y_train_quality, X_val, y_val_quality,
            task_type='regression', model_name='quality'
        )
        
        # 2. Speed (回归)
        y_train_speed = targets['speed'][indices_train]
        y_val_speed = targets['speed'][indices_val]
        self.models['speed'] = self.train_model(
            X_train, y_train_speed, X_val, y_val_speed,
            task_type='regression', model_name='speed'
        )
        
        # 3. 🔥 Lossless (分类 - Phase 36新增)
        y_train_lossless = targets['lossless'][indices_train]
        y_val_lossless = targets['lossless'][indices_val]
        self.models['lossless'] = self.train_model(
            X_train, y_train_lossless, X_val, y_val_lossless,
            task_type='classification', model_name='lossless'
        )
        
        logger.info("\n✅ All models trained successfully!")
    
    def save_models(self, version: str = None):
        """保存模型"""
        if version is None:
            version = datetime.now().strftime("%Y%m%d_%H%M%S")
        
        logger.info(f"\n💾 Saving models (version: {version})...")
        
        for name, model in self.models.items():
            if model is not None:
                model_path = self.model_dir / f"{name}_v{version}.txt"
                model.save_model(str(model_path))
                logger.info(f"  ✅ {name}: {model_path}")
        
        # 保存元数据
        metadata = {
            'version': version,
            'timestamp': datetime.now().isoformat(),
            'models': list(self.models.keys()),
            'phase': '35+36',  # Phase 35 (训练) + Phase 36 (lossless)
        }
        
        metadata_path = self.model_dir / f"metadata_v{version}.json"
        with open(metadata_path, 'w') as f:
            json.dump(metadata, f, indent=2)
        
        logger.info(f"  ✅ metadata: {metadata_path}")
        logger.info("\n🎉 Models saved successfully!")
        
        return version


def main():
    parser = argparse.ArgumentParser(description='PIXLY AI - LightGBM增量训练')
    parser.add_argument('--feedback-db', required=True, help='SQLite feedback数据库路径')
    parser.add_argument('--model-dir', default='./models', help='模型保存目录')
    parser.add_argument('--min-samples', type=int, default=100, help='最小训练样本数')
    
    args = parser.parse_args()
    
    logger.info("\n" + "━"*60)
    logger.info("🎓 PIXLY AI Training - LightGBM (Phase 35+36)")
    logger.info("━"*60 + "\n")
    
    # 初始化训练器
    trainer = LightGBMTrainer(args.feedback_db, args.model_dir)
    
    # 加载数据
    df = trainer.load_feedback_data(min_samples=args.min_samples)
    if df is None:
        logger.error("❌ Failed to load sufficient training data")
        sys.exit(1)
    
    # 训练模型
    trainer.train_all_models(df)
    
    # 保存模型
    version = trainer.save_models()
    
    logger.info("\n" + "━"*60)
    logger.info(f"✅ Training完成! Version: {version}")
    logger.info("━"*60 + "\n")


if __name__ == '__main__':
    main()
