#!/usr/bin/env python3
"""
═══════════════════════════════════════════════════════════════
PIXLY AI Training - Data Preprocessing Script
═══════════════════════════════════════════════════════════════

从feedback数据库提取训练数据并进行特征工程。

功能:
- 从SQLite feedback数据库读取数据
- 特征提取和工程化
- 数据清洗和归一化
- 生成训练/验证/测试数据集
- 数据质量报告

作者: PIXLY AI Team
版本: 1.0.0
日期: 2025-11-06
"""

import sqlite3
import pandas as pd
import numpy as np
import json
import logging
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Tuple, Optional
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler, LabelEncoder

# 配置日志
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s',
    datefmt='%Y-%m-%d %H:%M:%S'
)
logger = logging.getLogger(__name__)


class DataPreprocessor:
    """数据预处理器"""
    
    def __init__(self, db_path: str, output_dir: str):
        """
        初始化预处理器
        
        Args:
            db_path: feedback数据库路径
            output_dir: 输出目录
        """
        self.db_path = Path(db_path)
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)
        
        self.scaler = StandardScaler()
        self.label_encoders = {}
        
        logger.info(f"数据预处理器初始化完成")
        logger.info(f"  数据库: {self.db_path}")
        logger.info(f"  输出目录: {self.output_dir}")
    
    def load_feedback_data(self) -> pd.DataFrame:
        """
        从数据库加载反馈数据
        
        Returns:
            包含反馈数据的DataFrame
        """
        logger.info("📊 加载feedback数据...")
        
        conn = sqlite3.connect(str(self.db_path))
        
        # 读取feedback记录
        query = """
        SELECT 
            request_id,
            input_format,
            target_format,
            width,
            height,
            file_size,
            has_alpha,
            has_animation,
            color_space,
            bit_depth,
            priority,
            preserve_quality,
            predicted_quality,
            predicted_speed,
            predicted_effort,
            predicted_distance,
            actual_quality,
            actual_size,
            actual_time,
            compression_ratio,
            success,
            created_at
        FROM feedback
        WHERE success = 1
        ORDER BY created_at DESC
        """
        
        df = pd.read_sql_query(query, conn)
        conn.close()
        
        logger.info(f"✅ 加载了 {len(df)} 条成功记录")
        logger.info(f"  日期范围: {df['created_at'].min()} ~ {df['created_at'].max()}")
        
        return df
    
    def extract_features(self, df: pd.DataFrame) -> pd.DataFrame:
        """
        特征提取和工程化
        
        Args:
            df: 原始数据
            
        Returns:
            包含特征的DataFrame
        """
        logger.info("🔧 开始特征工程...")
        
        features = df.copy()
        
        # 1. 图像尺寸相关特征
        features['aspect_ratio'] = features['width'] / features['height']
        features['total_pixels'] = features['width'] * features['height']
        features['megapixels'] = features['total_pixels'] / 1_000_000
        
        # 2. 文件大小相关特征
        features['file_size_mb'] = features['file_size'] / (1024 * 1024)
        features['bits_per_pixel'] = (features['file_size'] * 8) / features['total_pixels']
        
        # 3. 格式组合特征
        features['format_pair'] = features['input_format'] + '_to_' + features['target_format']
        
        # 4. 布尔特征转整数
        features['has_alpha_int'] = features['has_alpha'].astype(int)
        features['has_animation_int'] = features['has_animation'].astype(int)
        features['preserve_quality_int'] = features['preserve_quality'].astype(int)
        
        # 5. 尺寸分类
        features['size_category'] = pd.cut(
            features['megapixels'],
            bins=[0, 1, 4, 10, 50, np.inf],
            labels=['tiny', 'small', 'medium', 'large', 'huge']
        )
        
        # 6. 压缩率指标
        features['compression_efficiency'] = features['compression_ratio'] / features['actual_time']
        
        # 7. 质量相关
        if 'actual_quality' in features.columns:
            features['quality_efficiency'] = features['actual_quality'] / features['actual_time']
        
        logger.info(f"✅ 特征工程完成，特征数量: {len(features.columns)}")
        
        return features
    
    def encode_categorical(self, df: pd.DataFrame) -> pd.DataFrame:
        """
        编码分类特征
        
        Args:
            df: 包含分类特征的DataFrame
            
        Returns:
            编码后的DataFrame
        """
        logger.info("🏷️  编码分类特征...")
        
        encoded = df.copy()
        categorical_cols = [
            'input_format',
            'target_format',
            'format_pair',
            'priority',
            'color_space',
            'size_category'
        ]
        
        for col in categorical_cols:
            if col in encoded.columns:
                # 处理缺失值
                encoded[col] = encoded[col].fillna('unknown')
                
                # Label Encoding
                le = LabelEncoder()
                encoded[f'{col}_encoded'] = le.fit_transform(encoded[col])
                self.label_encoders[col] = le
                
                logger.info(f"  {col}: {len(le.classes_)} 个类别")
        
        return encoded
    
    def normalize_features(self, df: pd.DataFrame) -> pd.DataFrame:
        """
        归一化数值特征
        
        Args:
            df: 包含数值特征的DataFrame
            
        Returns:
            归一化后的DataFrame
        """
        logger.info("📏 归一化数值特征...")
        
        normalized = df.copy()
        
        numeric_cols = [
            'width', 'height', 'file_size', 'bit_depth',
            'aspect_ratio', 'total_pixels', 'megapixels',
            'file_size_mb', 'bits_per_pixel',
            'compression_ratio', 'actual_time'
        ]
        
        # 只归一化存在的列
        cols_to_normalize = [col for col in numeric_cols if col in normalized.columns]
        
        if cols_to_normalize:
            normalized[cols_to_normalize] = self.scaler.fit_transform(normalized[cols_to_normalize])
            logger.info(f"✅ 归一化了 {len(cols_to_normalize)} 个数值特征")
        
        return normalized
    
    def split_dataset(
        self,
        df: pd.DataFrame,
        test_size: float = 0.2,
        val_size: float = 0.1,
        random_state: int = 42
    ) -> Tuple[pd.DataFrame, pd.DataFrame, pd.DataFrame]:
        """
        划分数据集
        
        Args:
            df: 完整数据集
            test_size: 测试集比例
            val_size: 验证集比例
            random_state: 随机种子
            
        Returns:
            (训练集, 验证集, 测试集)
        """
        logger.info("📂 划分数据集...")
        
        # 先分出测试集
        train_val, test = train_test_split(
            df,
            test_size=test_size,
            random_state=random_state,
            stratify=df['target_format'] if 'target_format' in df.columns else None
        )
        
        # 再从训练集中分出验证集
        val_ratio = val_size / (1 - test_size)
        train, val = train_test_split(
            train_val,
            test_size=val_ratio,
            random_state=random_state,
            stratify=train_val['target_format'] if 'target_format' in train_val.columns else None
        )
        
        logger.info(f"✅ 数据集划分完成:")
        logger.info(f"  训练集: {len(train)} ({len(train)/len(df)*100:.1f}%)")
        logger.info(f"  验证集: {len(val)} ({len(val)/len(df)*100:.1f}%)")
        logger.info(f"  测试集: {len(test)} ({len(test)/len(df)*100:.1f}%)")
        
        return train, val, test
    
    def generate_report(self, df: pd.DataFrame) -> Dict:
        """
        生成数据质量报告
        
        Args:
            df: 数据集
            
        Returns:
            报告字典
        """
        report = {
            'total_records': len(df),
            'date_range': {
                'start': df['created_at'].min() if 'created_at' in df.columns else None,
                'end': df['created_at'].max() if 'created_at' in df.columns else None
            },
            'format_distribution': df['target_format'].value_counts().to_dict() if 'target_format' in df.columns else {},
            'priority_distribution': df['priority'].value_counts().to_dict() if 'priority' in df.columns else {},
            'statistics': {
                'avg_compression_ratio': float(df['compression_ratio'].mean()) if 'compression_ratio' in df.columns else None,
                'avg_processing_time': float(df['actual_time'].mean()) if 'actual_time' in df.columns else None,
                'avg_file_size': float(df['file_size'].mean()) if 'file_size' in df.columns else None
            },
            'missing_values': df.isnull().sum().to_dict(),
            'data_types': df.dtypes.astype(str).to_dict()
        }
        
        return report
    
    def save_datasets(
        self,
        train: pd.DataFrame,
        val: pd.DataFrame,
        test: pd.DataFrame
    ):
        """
        保存数据集到文件
        
        Args:
            train: 训练集
            val: 验证集
            test: 测试集
        """
        logger.info("💾 保存数据集...")
        
        train.to_csv(self.output_dir / 'train.csv', index=False)
        val.to_csv(self.output_dir / 'val.csv', index=False)
        test.to_csv(self.output_dir / 'test.csv', index=False)
        
        logger.info(f"✅ 数据集已保存到 {self.output_dir}")
    
    def save_artifacts(self, report: Dict):
        """
        保存预处理工件
        
        Args:
            report: 数据质量报告
        """
        logger.info("💾 保存预处理工件...")
        
        # 保存报告
        with open(self.output_dir / 'preprocessing_report.json', 'w') as f:
            json.dump(report, f, indent=2, default=str)
        
        # 保存label encoders
        encoders_info = {
            name: {
                'classes': encoder.classes_.tolist()
            }
            for name, encoder in self.label_encoders.items()
        }
        with open(self.output_dir / 'label_encoders.json', 'w') as f:
            json.dump(encoders_info, f, indent=2)
        
        logger.info("✅ 工件保存完成")
    
    def run(self):
        """执行完整的预处理pipeline"""
        logger.info("="*60)
        logger.info("🚀 开始数据预处理...")
        logger.info("="*60)
        
        # 1. 加载数据
        df = self.load_feedback_data()
        
        if len(df) == 0:
            logger.error("❌ 没有可用的feedback数据！")
            return
        
        # 2. 特征工程
        df = self.extract_features(df)
        
        # 3. 编码分类特征
        df = self.encode_categorical(df)
        
        # 4. 归一化
        df = self.normalize_features(df)
        
        # 5. 划分数据集
        train, val, test = self.split_dataset(df)
        
        # 6. 生成报告
        report = self.generate_report(df)
        
        # 7. 保存
        self.save_datasets(train, val, test)
        self.save_artifacts(report)
        
        logger.info("="*60)
        logger.info("✅ 数据预处理完成！")
        logger.info("="*60)
        logger.info(f"总样本数: {report['total_records']}")
        logger.info(f"训练集: {len(train)}")
        logger.info(f"验证集: {len(val)}")
        logger.info(f"测试集: {len(test)}")


def main():
    """主函数"""
    import argparse
    
    parser = argparse.ArgumentParser(description='PIXLY AI 数据预处理')
    parser.add_argument(
        '--db',
        type=str,
        default='../data/feedback.db',
        help='Feedback数据库路径'
    )
    parser.add_argument(
        '--output',
        type=str,
        default='../data/processed',
        help='输出目录'
    )
    
    args = parser.parse_args()
    
    preprocessor = DataPreprocessor(args.db, args.output)
    preprocessor.run()


if __name__ == '__main__':
    main()