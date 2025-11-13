#!/usr/bin/env python3
"""
Pixly AI Model Training Script
Version: 4.2.0

完整AI训练流程：
1. SWT小波变换特征提取
2. LightGBM梯度提升树训练
3. 模型导出为JSON（供Go调用）
"""

import json
import os
import sys
from pathlib import Path
from typing import Dict, List, Tuple
import numpy as np
from PIL import Image
import pywt
from sklearn.model_selection import train_test_split, GridSearchCV
from sklearn.preprocessing import StandardScaler
import lightgbm as lgb
import argparse
from datetime import datetime

# ============================================================================
# SWT特征提取器
# ============================================================================

class SWTFeatureExtractor:
    """静止小波变换特征提取器"""
    
    def __init__(self, wavelet='db1', max_level=3):
        self.wavelet = wavelet
        self.max_level = max_level
    
    def _prepare_image(self, img_array: np.ndarray) -> Tuple[np.ndarray, int]:
        """预处理图像：padding到偶数尺寸，计算最大可用层级"""
        h, w = img_array.shape
        
        # Padding到偶数尺寸
        new_h = h if h % 2 == 0 else h + 1
        new_w = w if w % 2 == 0 else w + 1
        
        if new_h != h or new_w != w:
            padded = np.zeros((new_h, new_w), dtype=img_array.dtype)
            padded[:h, :w] = img_array
            img_array = padded
        
        # 计算最大可用层级（基于最小尺寸）
        min_dim = min(new_h, new_w)
        max_possible_level = int(np.floor(np.log2(min_dim)))
        actual_level = min(self.max_level, max_possible_level, 3)  # 最多3层
        
        return img_array, max(1, actual_level)  # 至少1层
    
    def extract(self, image_path: str) -> Dict[str, float]:
        """提取SWT特征（9维）"""
        img = Image.open(image_path).convert('L')  # 转灰度
        img_array = np.array(img, dtype=np.float32)
        
        # 预处理图像
        img_array, actual_level = self._prepare_image(img_array)
        
        # 静止小波变换
        try:
            coeffs = pywt.swt2(img_array, self.wavelet, level=actual_level)
        except Exception as e:
            # 降级到1层
            coeffs = pywt.swt2(img_array, self.wavelet, level=1)
        
        features = {}
        
        # 获取实际层数
        num_levels = len(coeffs)
        
        # 1. 边缘强度（高频水平+垂直分量）
        edge_strength = 0
        for (cA, (cH, cV, cD)) in coeffs:
            edge_strength += np.std(cH) + np.std(cV)
        features['edge_strength'] = float(edge_strength / num_levels)
        
        # 2. 纹理复杂度（对角分量标准差）
        texture_complexity = 0
        for (cA, (cH, cV, cD)) in coeffs:
            texture_complexity += np.std(cD)
        features['texture_complexity'] = float(texture_complexity / num_levels)
        
        # 3. 噪声水平（最高频细节的MAD）
        (cA, (cH, cV, cD)) = coeffs[0]
        noise_level = np.median(np.abs(cD - np.median(cD)))
        features['noise_level'] = float(noise_level)
        
        # 4. 细节层次（多尺度能量分布）
        detail_energies = []
        for (cA, (cH, cV, cD)) in coeffs:
            energy = np.sum(cH**2 + cV**2 + cD**2)
            detail_energies.append(energy)
        features['detail_level'] = float(np.std(detail_energies))
        
        # 5-7. 频率能量分布
        total_energy = sum(detail_energies)
        if total_energy > 0:
            features['high_freq_energy'] = float(detail_energies[0] / total_energy)
            # 中频能量：如果有3层取中间层，否则取平均
            if len(detail_energies) >= 3:
                features['mid_freq_energy'] = float(detail_energies[1] / total_energy)
            elif len(detail_energies) == 2:
                features['mid_freq_energy'] = float((detail_energies[0] + detail_energies[1]) / 2 / total_energy)
            else:
                features['mid_freq_energy'] = float(detail_energies[0] / 2 / total_energy)
            features['low_freq_energy'] = float(detail_energies[-1] / total_energy)
        else:
            features['high_freq_energy'] = 0
            features['mid_freq_energy'] = 0
            features['low_freq_energy'] = 0
        
        # 8. 整体质量评分（基于多特征加权）
        features['overall_quality'] = float(
            0.3 * min(features['edge_strength'] / 100, 1) +
            0.2 * min(features['texture_complexity'] / 100, 1) +
            0.3 * (1 - min(features['noise_level'] / 50, 1)) +
            0.2 * min(features['detail_level'] / 1000, 1)
        ) * 100
        
        # 9. 可压缩性评分
        features['compression_score'] = float(
            0.5 * (1 - features['texture_complexity'] / 100) +
            0.3 * (1 - features['noise_level'] / 50) +
            0.2 * features['low_freq_energy']
        )
        
        return features

# ============================================================================
# 基础特征提取器
# ============================================================================

class BasicFeatureExtractor:
    """基础图像特征提取器"""
    
    @staticmethod
    def extract(image_path: str) -> Dict[str, any]:
        """提取基础特征"""
        img = Image.open(image_path)
        
        features = {
            'width': img.width,
            'height': img.height,
            'pixels': img.width * img.height,
            'aspect_ratio': img.width / img.height,
            'has_alpha': 1 if img.mode in ('RGBA', 'LA') else 0,
            'file_size': os.path.getsize(image_path),
        }
        
        return features

# ============================================================================
# 数据加载器
# ============================================================================

class DatasetLoader:
    """Eagle数据集加载器"""
    
    def __init__(self, eagle_dir: str, seed_file: str = None):
        self.eagle_dir = Path(eagle_dir)
        self.seed_file = seed_file
        self.swt_extractor = SWTFeatureExtractor()
        self.basic_extractor = BasicFeatureExtractor()
    
    def load_eagle_images(self) -> List[Dict]:
        """加载Eagle图像库"""
        print("📂 扫描Eagle图像库...")
        
        image_files = []
        for ext in ['*.jpg', '*.jpeg', '*.png', '*.webp']:
            image_files.extend(self.eagle_dir.glob(f'**/{ext}'))
        
        print(f"✅ 找到 {len(image_files)} 张图像")
        
        dataset = []
        for i, img_path in enumerate(image_files):
            if i % 50 == 0:
                print(f"   处理进度: {i}/{len(image_files)}")
            
            try:
                # 提取特征
                basic = self.basic_extractor.extract(str(img_path))
                swt = self.swt_extractor.extract(str(img_path))
                
                # 合并特征
                sample = {
                    'image_path': str(img_path),
                    **basic,
                    **swt,
                    # 标签（需要从XMP或手动标注获取，这里先用启发式）
                    'optimal_quality_jxl': self._estimate_optimal_quality(swt, 'jxl'),
                    'optimal_quality_avif': self._estimate_optimal_quality(swt, 'avif'),
                    'optimal_quality_webp': self._estimate_optimal_quality(swt, 'webp'),
                }
                
                dataset.append(sample)
                
            except Exception as e:
                print(f"   ⚠️  跳过 {img_path.name}: {e}")
                continue
        
        print(f"✅ 成功加载 {len(dataset)} 个样本")
        return dataset
    
    def _estimate_optimal_quality(self, swt_features: Dict, tool: str) -> int:
        """启发式估算最优质量（真实训练时应使用实际转换结果）"""
        # 基于纹理复杂度和噪声水平估算
        texture = swt_features['texture_complexity']
        noise = swt_features['noise_level']
        
        if tool == 'jxl':
            base_q = 90
            if texture > 80:
                base_q = 95
            if noise > 30:
                base_q -= 5
        elif tool == 'avif':
            base_q = 85
            if texture > 80:
                base_q = 90
            if noise > 30:
                base_q -= 5
        else:  # webp
            base_q = 85
            if texture > 80:
                base_q = 90
            if noise > 30:
                base_q -= 5
        
        return base_q
    
    def load_seed_library(self) -> List[Dict]:
        """加载预设种子库"""
        if not self.seed_file or not Path(self.seed_file).exists():
            print("⚠️  种子库文件不存在，跳过")
            return []
        
        print(f"📚 加载种子库: {self.seed_file}")
        with open(self.seed_file, 'r', encoding='utf-8') as f:
            seeds = json.load(f)
        
        print(f"✅ 加载 {len(seeds)} 个种子样本")
        return seeds
    
    def merge_datasets(self, eagle_data: List[Dict], seed_data: List[Dict]) -> List[Dict]:
        """合并数据集"""
        print(f"🔀 合并数据集: Eagle({len(eagle_data)}) + Seeds({len(seed_data)})")
        merged = eagle_data + seed_data
        print(f"✅ 合并后总计: {len(merged)} 个样本")
        return merged

# ============================================================================
# LightGBM训练器
# ============================================================================

class LightGBMTrainer:
    """LightGBM模型训练器"""
    
    def __init__(self, tool: str):
        self.tool = tool
        self.model = None
        self.scaler = StandardScaler()
        self.feature_names = [
            'width', 'height', 'pixels', 'aspect_ratio', 'has_alpha',
            'edge_strength', 'texture_complexity', 'noise_level',
            'detail_level', 'compression_score', 'high_freq_energy',
            'low_freq_energy'
        ]
    
    def prepare_features(self, dataset: List[Dict]) -> Tuple[np.ndarray, np.ndarray]:
        """准备特征矩阵和标签"""
        X = []
        y = []
        
        label_key = f'optimal_quality_{self.tool}'
        
        for sample in dataset:
            # 检查是否有所需的所有特征
            if not all(k in sample for k in self.feature_names):
                continue
            if label_key not in sample:
                continue
            
            # 提取特征向量
            features = [sample[k] for k in self.feature_names]
            X.append(features)
            y.append(sample[label_key])
        
        X = np.array(X)
        y = np.array(y)
        
        # 标准化特征
        X = self.scaler.fit_transform(X)
        
        print(f"📊 特征矩阵: {X.shape}, 标签: {y.shape}")
        return X, y
    
    def train(self, X: np.ndarray, y: np.ndarray) -> Dict:
        """训练LightGBM模型"""
        print(f"\n🚀 开始训练 {self.tool} 模型...")
        
        # 划分训练集和验证集
        X_train, X_val, y_train, y_val = train_test_split(
            X, y, test_size=0.2, random_state=42
        )
        
        # LightGBM参数
        params = {
            'objective': 'regression',
            'metric': 'rmse',
            'boosting_type': 'gbdt',
            'num_leaves': 31,
            'learning_rate': 0.05,
            'feature_fraction': 0.9,
            'bagging_fraction': 0.8,
            'bagging_freq': 5,
            'verbose': -1
        }
        
        # 创建数据集
        train_data = lgb.Dataset(X_train, label=y_train)
        val_data = lgb.Dataset(X_val, label=y_val, reference=train_data)
        
        # 训练模型
        self.model = lgb.train(
            params,
            train_data,
            num_boost_round=200,
            valid_sets=[train_data, val_data],
            valid_names=['train', 'val'],
            callbacks=[
                lgb.early_stopping(stopping_rounds=20),
                lgb.log_evaluation(period=50)
            ]
        )
        
        # 评估
        y_pred = self.model.predict(X_val)
        rmse = np.sqrt(np.mean((y_pred - y_val) ** 2))
        mae = np.mean(np.abs(y_pred - y_val))
        
        print(f"✅ 训练完成！RMSE: {rmse:.2f}, MAE: {mae:.2f}")
        
        return {
            'rmse': float(rmse),
            'mae': float(mae),
            'n_estimators': self.model.num_trees(),
        }
    
    def export_model(self, output_path: str):
        """导出模型为文本格式"""
        print(f"💾 导出模型到: {output_path}")
        
        # 使用LightGBM原生文本格式
        txt_path = output_path.replace('.json', '.txt')
        self.model.save_model(txt_path)
        
        # 保存元数据为JSON
        metadata = {
            'tool': self.tool,
            'version': '4.2.0',
            'trained_at': datetime.now().isoformat(),
            'feature_names': self.feature_names,
            'scaler_mean': self.scaler.mean_.tolist(),
            'scaler_std': self.scaler.scale_.tolist(),
            'model_file': txt_path,
        }
        
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(metadata, f, indent=2)
        
        print(f"✅ 模型已导出: {txt_path} + {output_path}")

# ============================================================================
# 主训练流程
# ============================================================================

def main():
    parser = argparse.ArgumentParser(description='Pixly AI Model Training v4.2.0')
    parser.add_argument('--eagle-dir', required=True, help='Eagle图像库目录')
    parser.add_argument('--seed-file', default=None, help='预设种子库JSON文件')
    parser.add_argument('--output-dir', default='./models', help='模型输出目录')
    parser.add_argument('--tools', nargs='+', default=['jxl', 'avif', 'webp'], 
                        help='训练的工具列表')
    
    args = parser.parse_args()
    
    print("╔" + "="*68 + "╗")
    print("║" + " "*15 + "🤖 Pixly AI Model Training v4.2.0" + " "*18 + "║")
    print("╚" + "="*68 + "╝\n")
    
    # 1. 加载数据集
    print("📊 阶段1: 数据加载")
    print("-" * 70)
    
    loader = DatasetLoader(args.eagle_dir, args.seed_file)
    eagle_data = loader.load_eagle_images()
    seed_data = loader.load_seed_library()
    full_dataset = loader.merge_datasets(eagle_data, seed_data)
    
    if len(full_dataset) == 0:
        print("❌ 错误：数据集为空！")
        sys.exit(1)
    
    # 保存完整数据集
    dataset_path = Path(args.output_dir) / 'training_dataset.json'
    dataset_path.parent.mkdir(parents=True, exist_ok=True)
    with open(dataset_path, 'w', encoding='utf-8') as f:
        json.dump(full_dataset, f, indent=2, ensure_ascii=False)
    print(f"💾 数据集已保存: {dataset_path}\n")
    
    # 2. 训练每个工具的模型
    print("🚀 阶段2: 模型训练")
    print("-" * 70)
    
    results = {}
    
    for tool in args.tools:
        print(f"\n{'='*70}")
        print(f"训练 {tool.upper()} 模型")
        print(f"{'='*70}")
        
        trainer = LightGBMTrainer(tool)
        X, y = trainer.prepare_features(full_dataset)
        
        if len(X) == 0:
            print(f"⚠️  跳过 {tool}：无有效样本")
            continue
        
        metrics = trainer.train(X, y)
        
        # 导出模型
        model_path = Path(args.output_dir) / f'lightgbm_{tool}.json'
        trainer.export_model(str(model_path))
        
        results[tool] = {
            'model_path': str(model_path),
            'metrics': metrics,
            'n_samples': len(X),
        }
    
    # 3. 生成训练报告
    print("\n" + "="*70)
    print("📋 训练总结")
    print("="*70)
    
    for tool, result in results.items():
        print(f"\n{tool.upper()}:")
        print(f"  样本数: {result['n_samples']}")
        print(f"  RMSE: {result['metrics']['rmse']:.2f}")
        print(f"  MAE: {result['metrics']['mae']:.2f}")
        print(f"  树数量: {result['metrics']['n_estimators']}")
        print(f"  模型文件: {result['model_path']}")
    
    # 保存训练报告
    report_path = Path(args.output_dir) / 'training_report.json'
    report = {
        'version': '4.2.0',
        'trained_at': datetime.now().isoformat(),
        'dataset_size': len(full_dataset),
        'models': results,
    }
    with open(report_path, 'w', encoding='utf-8') as f:
        json.dump(report, f, indent=2)
    
    print(f"\n💾 训练报告已保存: {report_path}")
    print("\n🎉 所有模型训练完成！\n")

if __name__ == '__main__':
    main()
