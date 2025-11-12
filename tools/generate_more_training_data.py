#!/usr/bin/env python3
"""
扩展PPO训练数据生成脚本
目标：从更多样化的图像生成训练数据，提升模型精度5-10%
"""

import os
import sys
import json
from pathlib import Path
from typing import List, Dict, Tuple
import numpy as np
from PIL import Image
import pywt  # PyWavelets
import hashlib
import time

# 添加tools目录到路径
sys.path.insert(0, str(Path(__file__).parent))

try:
    from predict_params import AIPredictor
except ImportError:
    print("❌ 无法导入AIPredictor，请确保predict_params.py存在")
    sys.exit(1)

class EnhancedTrainingDataGenerator:
    """增强的训练数据生成器"""
    
    def __init__(self):
        self.root_dir = Path(__file__).parent.parent
        self.data_dir = self.root_dir / 'data' / 'observations'
        self.data_dir.mkdir(parents=True, exist_ok=True)
        
        # 初始化AI预测器
        self.predictor = AIPredictor()
        
        # 扩展的搜索目录
        self.search_dirs = [
            self.root_dir / '@reference' / 'data',  # 测试数据
            Path.home() / 'Pictures',  # 用户图片目录
            Path.home() / 'Downloads',  # 下载目录
            Path('/System/Library/Desktop Pictures'),  # macOS系统壁纸
        ]
        
        # 支持的图像格式
        self.image_extensions = {'.jpg', '.jpeg', '.png', '.webp', '.bmp', '.gif', '.tiff'}
        
        # 质量策略映射
        self.quality_strategies = {
            'high_detail': {'quality': 95, 'reward': 0.7},  # 高细节图像
            'normal': {'quality': 85, 'reward': 0.8},       # 普通图像
            'smooth': {'quality': 75, 'reward': 0.9},       # 平滑图像
            'web_graphics': {'quality': 70, 'reward': 0.85}, # Web图形
        }
        
    def analyze_image_type(self, features: Dict) -> str:
        """根据特征判断图像类型"""
        edge_strength = features.get('edge_strength', 0)
        color_complexity = features.get('color_complexity', 0)
        
        if edge_strength > 8:
            return 'high_detail'
        elif edge_strength < 2:
            return 'smooth'
        elif color_complexity < 50:
            return 'web_graphics'
        else:
            return 'normal'
    
    def calculate_reward(self, features: Dict, action: Dict) -> float:
        """计算更精细的奖励"""
        # 基础奖励
        base_reward = 0.5
        
        # 根据图像类型调整
        image_type = self.analyze_image_type(features)
        strategy = self.quality_strategies[image_type]
        
        # 质量匹配度
        quality_diff = abs(action['quality'] - strategy['quality'])
        quality_reward = max(0, 1 - quality_diff / 100)
        
        # 格式选择奖励
        format_reward = 0
        if features.get('has_transparency') and action.get('format') in ['webp', 'avif']:
            format_reward = 0.2
        elif not features.get('has_transparency') and action.get('format') in ['jxl', 'avif']:
            format_reward = 0.2
            
        # 文件大小奖励（假设）
        size_reward = 0.1 if action.get('quality') < 90 else 0
        
        # 计算总奖励
        total_reward = base_reward + quality_reward * 0.3 + format_reward * 0.2 + size_reward * 0.1
        
        return min(1.0, max(0.0, total_reward))
    
    def collect_images(self, max_count: int = 500) -> List[Path]:
        """收集更多样化的图像"""
        images = []
        seen_hashes = set()
        
        for search_dir in self.search_dirs:
            if not search_dir.exists():
                continue
                
            print(f"🔍 搜索目录: {search_dir}")
            
            for ext in self.image_extensions:
                pattern = f"**/*{ext}"
                for img_path in search_dir.glob(pattern):
                    if len(images) >= max_count:
                        break
                        
                    # 跳过过小或过大的文件
                    try:
                        size = img_path.stat().st_size
                        if size < 1024 or size > 50 * 1024 * 1024:  # 1KB - 50MB
                            continue
                            
                        # 计算文件哈希避免重复
                        with open(img_path, 'rb') as f:
                            file_hash = hashlib.md5(f.read(8192)).hexdigest()
                            
                        if file_hash in seen_hashes:
                            continue
                            
                        seen_hashes.add(file_hash)
                        images.append(img_path)
                        
                    except Exception:
                        continue
                        
            if len(images) >= max_count:
                break
                
        print(f"✅ 收集到 {len(images)} 张图像")
        return images[:max_count]
    
    def generate_observations(self, images: List[Path]) -> List[Dict]:
        """生成观测数据"""
        observations = []
        
        for i, img_path in enumerate(images, 1):
            try:
                print(f"[{i}/{len(images)}] 处理: {img_path.name}")
                
                # 提取特征（通过预测获取）
                try:
                    # 使用predict方法获取特征
                    result = self.predictor.predict(str(img_path), 'webp', 85, 'balanced')
                    
                    # 构建特征字典
                    features = {
                        'width': result.get('width', 1920),
                        'height': result.get('height', 1080),
                        'has_transparency': result.get('has_alpha', False),
                        'edge_strength': result.get('preprocessing_steps', []).count('sharpen') * 5,
                        'color_complexity': 100 if 'colorful' in str(result).lower() else 50,
                        'file_size': img_path.stat().st_size,
                    }
                except Exception as e:
                    # 使用默认特征
                    features = {
                        'width': 1920,
                        'height': 1080,
                        'has_transparency': False,
                        'edge_strength': 5,
                        'color_complexity': 50,
                        'file_size': img_path.stat().st_size,
                    }
                
                # 生成多个动作变体（不同quality和format）
                formats = ['webp', 'avif', 'jxl']
                qualities = [70, 75, 80, 85, 90, 95]
                
                for fmt in formats:
                    for quality in qualities:
                        # 创建动作
                        action = {
                            'format': fmt,
                            'quality': quality,
                            'effort': 7,
                            'lossless': quality >= 95
                        }
                        
                        # 计算奖励
                        reward = self.calculate_reward(features, action)
                        
                        # 构建观测
                        observation = {
                            'state': list(features.values())[:20],  # 前20个特征
                            'action': [
                                formats.index(fmt) / len(formats),  # 归一化
                                quality / 100,
                                action['effort'] / 10,
                                1.0 if action['lossless'] else 0.0
                            ],
                            'reward': reward,
                            'next_state': list(features.values())[:20],  # 简化：使用相同状态
                            'done': True,
                            'metadata': {
                                'file': img_path.name,
                                'format': fmt,
                                'quality': quality,
                                'image_type': self.analyze_image_type(features)
                            }
                        }
                        
                        observations.append(observation)
                        
            except Exception as e:
                print(f"  ⚠️ 跳过 {img_path.name}: {str(e)[:50]}")
                continue
                
        return observations
    
    def save_observations(self, observations: List[Dict]):
        """保存观测数据"""
        timestamp = int(time.time())
        output_file = self.data_dir / f'enhanced_observations_{timestamp}.json'
        
        with open(output_file, 'w') as f:
            json.dump(observations, f, indent=2)
            
        print(f"💾 保存 {len(observations)} 个观测到: {output_file}")
        return output_file
    
    def run(self, max_images: int = 100):
        """运行数据生成流程"""
        print("="*70)
        print("🚀 PPO增强训练数据生成")
        print("="*70)
        print(f"目标: 生成更多样化的训练数据，提升5-10%模型精度")
        print()
        
        # 1. 收集图像
        print("📂 Step 1: 收集图像...")
        images = self.collect_images(max_images)
        
        if not images:
            print("❌ 没有找到图像文件")
            return None
            
        # 2. 生成观测
        print(f"\n🔬 Step 2: 生成观测数据...")
        observations = self.generate_observations(images)
        
        # 3. 保存数据
        print(f"\n💾 Step 3: 保存数据...")
        output_file = self.save_observations(observations)
        
        # 统计
        print("\n" + "="*70)
        print("📊 生成统计:")
        print(f"  图像数量: {len(images)}")
        print(f"  观测数量: {len(observations)}")
        print(f"  平均每图像: {len(observations)/len(images):.1f} 个观测")
        
        # 奖励分布
        rewards = [obs['reward'] for obs in observations]
        print(f"  平均奖励: {np.mean(rewards):.3f}")
        print(f"  奖励范围: [{min(rewards):.3f}, {max(rewards):.3f}]")
        
        # 图像类型分布
        types_count = {}
        for obs in observations:
            img_type = obs['metadata'].get('image_type', 'unknown')
            types_count[img_type] = types_count.get(img_type, 0) + 1
            
        print("  图像类型分布:")
        for img_type, count in sorted(types_count.items()):
            print(f"    - {img_type}: {count} ({count/len(observations)*100:.1f}%)")
            
        print("="*70)
        print("✅ 数据生成完成！")
        print(f"   下一步: python tools/train_ppo.py --data {output_file}")
        
        return output_file

def main():
    generator = EnhancedTrainingDataGenerator()
    
    # 支持命令行参数
    max_images = 50  # 默认50张图像（会生成更多观测）
    if len(sys.argv) > 1:
        try:
            max_images = int(sys.argv[1])
        except ValueError:
            print(f"⚠️ 无效的图像数量参数，使用默认值 {max_images}")
            
    generator.run(max_images)

if __name__ == '__main__':
    main()
