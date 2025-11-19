#!/usr/bin/env python3
"""
Phase 3.4: A/B测试 - PPO vs LightGBM

对比两种模型的实际转换效果：
1. LightGBM: 监督学习模型（当前生产模型）
2. PPO: 强化学习模型（Phase 3训练）

测试指标：
- 平均奖励（压缩率 + SSIM质量）
- 文件大小减少率
- SSIM质量保持
- 转换成功率
"""

import os
import sys
import json
import subprocess
from pathlib import Path
from typing import Dict, List, Tuple
import numpy as np
from PIL import Image

# 添加scripts目录到路径
sys.path.insert(0, str(Path(__file__).parent))
from calculate_ssim import calculate_ssim_simple as calculate_ssim

class ABTestRunner:
    def __init__(self, test_dir: str, output_dir: str):
        self.test_dir = Path(test_dir)
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)
        
        # 结果存储
        self.lightgbm_results = []
        self.ppo_results = []
        
    def load_ppo_model(self):
        """加载PPO模型"""
        import torch
        import torch.nn as nn
        
        class ActorNetwork(nn.Module):
            def __init__(self, state_dim=128, action_dim=2):
                super().__init__()
                self.fc = nn.Sequential(
                    nn.Linear(state_dim, 256),
                    nn.ReLU(),
                    nn.Dropout(0.2),
                    nn.Linear(256, 128),
                    nn.ReLU(),
                    nn.Dropout(0.2),
                    nn.Linear(128, action_dim)
                )
                
                # 初始化偏向合理参数
                with torch.no_grad():
                    self.fc[-1].bias[0] = -0.3  # quality偏向75
                    self.fc[-1].bias[1] = -0.4  # effort偏向6
            
            def forward(self, state):
                return self.fc(state)
        
        # 加载模型（优先使用v3最佳模型）
        model_candidates = [
            Path('models/ppo/ppo_v3_best_webp.pth'),
            Path('models/ppo/ppo_v3_epoch10_webp.pth'),
            Path('models/ppo/ppo_best_webp.pth'),
            Path('models/ppo/actor_network.pth'),
        ]
        
        model_path = None
        for candidate in model_candidates:
            if candidate.exists():
                model_path = candidate
                break
        
        if model_path is None:
            print(f"⚠️  PPO model not found, tried: {[str(c) for c in model_candidates]}")
            return None
        
        actor = ActorNetwork()
        
        # PyTorch 2.6+ requires weights_only=False to load old models
        checkpoint = torch.load(model_path, weights_only=False)
        
        # Check if it's a complete checkpoint or just state_dict
        if isinstance(checkpoint, dict) and 'actor' in checkpoint:
            # Complete checkpoint (actor+critic)
            actor.load_state_dict(checkpoint['actor'])
            print(f"✅ PPO model loaded successfully (full checkpoint): {model_path}")
        else:
            # Only state_dict
            actor.load_state_dict(checkpoint)
            print(f"✅ PPO model loaded successfully (state_dict): {model_path}")
        
        actor.eval()
        return actor
    
    def extract_features(self, image_path: Path) -> np.ndarray:
        """提取图像特征（简化版128维）"""
        img = Image.open(image_path).convert('RGB')
        
        # 基础特征
        width, height = img.size
        file_size = image_path.stat().st_size
        
        # 颜色统计
        pixels = np.array(img)
        r_mean = pixels[:,:,0].mean() / 255.0
        g_mean = pixels[:,:,1].mean() / 255.0
        b_mean = pixels[:,:,2].mean() / 255.0
        r_std = pixels[:,:,0].std() / 255.0
        g_std = pixels[:,:,1].std() / 255.0
        b_std = pixels[:,:,2].std() / 255.0
        
        # 构建128维特征向量（简化版）
        features = [
            width / 4000.0,
            height / 4000.0,
            file_size / (1024 * 1024),
            r_mean, g_mean, b_mean,
            r_std, g_std, b_std,
        ]
        
        # 补齐到128维
        while len(features) < 128:
            features.append(0.0)
        
        return np.array(features[:128], dtype=np.float32)
    
    def predict_with_lightgbm(self, features: np.ndarray) -> Tuple[int, int]:
        """使用LightGBM预测参数"""
        try:
            import lightgbm as lgb
            
            # 加载模型
            quality_model = lgb.Booster(model_file='models/lightgbm_webp_quality.txt')
            effort_model = lgb.Booster(model_file='models/lightgbm_webp_effort.txt')
            
            # 预测
            quality = int(quality_model.predict([features])[0])
            effort = int(effort_model.predict([features])[0])
            
            # 限制范围
            quality = max(60, min(95, quality))
            effort = max(4, min(9, effort))
            
            return quality, effort
        except Exception as e:
            print(f"⚠️  LightGBM prediction failed: {e}")
            return 80, 6  # Default values
    
    def predict_with_ppo(self, actor, features: np.ndarray) -> Tuple[int, int]:
        """使用PPO预测参数"""
        import torch
        
        with torch.no_grad():
            state = torch.FloatTensor(features).unsqueeze(0)
            action = actor(state).squeeze(0)
            
            # 转换为参数
            quality = torch.sigmoid(action[0]).item() * 35 + 60  # 60-95
            effort = torch.sigmoid(action[1]).item() * 5 + 4     # 4-9
            
            quality = int(quality)
            effort = int(effort)
            
            return quality, effort
    
    def convert_image(self, input_path: Path, output_path: Path, 
                     quality: int, effort: int) -> Dict:
        """执行图像转换"""
        try:
            # 使用cwebp转换
            cmd = [
                'cwebp',
                '-q', str(quality),
                '-m', str(effort),
                str(input_path),
                '-o', str(output_path)
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
            
            if result.returncode != 0:
                return {'success': False, 'error': result.stderr}
            
            # 计算结果
            original_size = input_path.stat().st_size
            output_size = output_path.stat().st_size
            compression_ratio = (original_size - output_size) / original_size
            
            # 计算SSIM
            ssim = calculate_ssim(str(input_path), str(output_path))
            
            # 计算奖励
            reward = compression_ratio + (ssim - 0.95) * 2
            
            return {
                'success': True,
                'original_size': original_size,
                'output_size': output_size,
                'compression_ratio': compression_ratio,
                'ssim': ssim,
                'reward': reward,
                'quality': quality,
                'effort': effort
            }
        except Exception as e:
            return {'success': False, 'error': str(e)}
    
    def test_single_image(self, image_path: Path, actor) -> Dict:
        """Test single image"""
        print(f"\n📸 Testing: {image_path.name}")
        
        # Extract features
        features = self.extract_features(image_path)
        
        # LightGBM prediction
        lgb_quality, lgb_effort = self.predict_with_lightgbm(features)
        print(f"   LightGBM: quality={lgb_quality}, effort={lgb_effort}")
        
        # PPO prediction
        ppo_quality, ppo_effort = self.predict_with_ppo(actor, features)
        print(f"   PPO:      quality={ppo_quality}, effort={ppo_effort}")
        
        # Convert (LightGBM)
        lgb_output = self.output_dir / f"lgb_{image_path.stem}.webp"
        lgb_result = self.convert_image(image_path, lgb_output, lgb_quality, lgb_effort)
        
        if lgb_result['success']:
            print(f"   LightGBM result: compression {lgb_result['compression_ratio']*100:.1f}%, "
                  f"SSIM={lgb_result['ssim']:.4f}, reward={lgb_result['reward']:.4f}")
            self.lightgbm_results.append(lgb_result)
        else:
            print(f"   ❌ LightGBM conversion failed: {lgb_result.get('error', 'Unknown')}")
        
        # Convert (PPO)
        ppo_output = self.output_dir / f"ppo_{image_path.stem}.webp"
        ppo_result = self.convert_image(image_path, ppo_output, ppo_quality, ppo_effort)
        
        if ppo_result['success']:
            print(f"   PPO result:      compression {ppo_result['compression_ratio']*100:.1f}%, "
                  f"SSIM={ppo_result['ssim']:.4f}, reward={ppo_result['reward']:.4f}")
            self.ppo_results.append(ppo_result)
        else:
            print(f"   ❌ PPO conversion failed: {ppo_result.get('error', 'Unknown')}")
        
        return {
            'image': image_path.name,
            'lightgbm': lgb_result,
            'ppo': ppo_result
        }
    
    def run_test(self, max_images: int = 50):
        """Run A/B test"""
        print("=" * 60)
        print("🧪 Phase 3.4: A/B Test - PPO vs LightGBM")
        print("=" * 60)
        
        # Load PPO model
        actor = self.load_ppo_model()
        if actor is None:
            print("❌ Unable to load PPO model, test terminated")
            return
        
        # Scan test images
        image_files = []
        for ext in ['*.jpg', '*.jpeg', '*.png']:
            image_files.extend(self.test_dir.glob(ext))
        
        if not image_files:
            print(f"❌ No test images found: {self.test_dir}")
            return
        
        print(f"\n📁 Found {len(image_files)} test images")
        print(f"   Testing: {min(max_images, len(image_files))} images")
        
        # Test each image
        test_results = []
        for i, image_path in enumerate(image_files[:max_images]):
            result = self.test_single_image(image_path, actor)
            test_results.append(result)
            
            if (i + 1) % 10 == 0:
                print(f"\nProgress: {i+1}/{min(max_images, len(image_files))}")
        
        # 生成报告
        self.generate_report(test_results)
    
    def generate_report(self, test_results: List[Dict]):
        """Generate test report"""
        print("\n" + "=" * 60)
        print("📊 A/B Test Results Report")
        print("=" * 60)
        
        # LightGBM statistics
        if self.lightgbm_results:
            lgb_rewards = [r['reward'] for r in self.lightgbm_results]
            lgb_compression = [r['compression_ratio'] for r in self.lightgbm_results]
            lgb_ssim = [r['ssim'] for r in self.lightgbm_results]
            
            print("\n🔵 LightGBM Model:")
            print(f"   Successful conversions: {len(self.lightgbm_results)}/{len(test_results)}")
            print(f"   Average reward: {np.mean(lgb_rewards):.4f} ± {np.std(lgb_rewards):.4f}")
            print(f"   Average compression ratio: {np.mean(lgb_compression)*100:.2f}%")
            print(f"   Average SSIM: {np.mean(lgb_ssim):.4f}")
            print(f"   Best reward: {np.max(lgb_rewards):.4f}")
            print(f"   Worst reward: {np.min(lgb_rewards):.4f}")
        
        # PPO statistics
        if self.ppo_results:
            ppo_rewards = [r['reward'] for r in self.ppo_results]
            ppo_compression = [r['compression_ratio'] for r in self.ppo_results]
            ppo_ssim = [r['ssim'] for r in self.ppo_results]
            
            print("\n🟢 PPO Model:")
            print(f"   Successful conversions: {len(self.ppo_results)}/{len(test_results)}")
            print(f"   Average reward: {np.mean(ppo_rewards):.4f} ± {np.std(ppo_rewards):.4f}")
            print(f"   Average compression ratio: {np.mean(ppo_compression)*100:.2f}%")
            print(f"   Average SSIM: {np.mean(ppo_ssim):.4f}")
            print(f"   Best reward: {np.max(ppo_rewards):.4f}")
            print(f"   Worst reward: {np.min(ppo_rewards):.4f}")
        
        # Comparative analysis
        if self.lightgbm_results and self.ppo_results:
            print("\n📈 Comparative Analysis:")
            
            reward_improvement = (np.mean(ppo_rewards) - np.mean(lgb_rewards)) / abs(np.mean(lgb_rewards)) * 100
            compression_improvement = (np.mean(ppo_compression) - np.mean(lgb_compression)) / np.mean(lgb_compression) * 100
            ssim_diff = np.mean(ppo_ssim) - np.mean(lgb_ssim)
            
            print(f"   Reward improvement: {reward_improvement:+.2f}%")
            print(f"   Compression improvement: {compression_improvement:+.2f}%")
            print(f"   SSIM difference: {ssim_diff:+.4f}")
            
            # Determine winner
            if reward_improvement > 5:
                print("\n🎉 Conclusion: PPO model significantly better than LightGBM!")
            elif reward_improvement > 0:
                print("\n✅ Conclusion: PPO model slightly better than LightGBM")
            elif reward_improvement > -5:
                print("\n⚖️  Conclusion: Both models perform similarly")
            else:
                print("\n⚠️  Conclusion: LightGBM model still better")
        
        # 保存详细结果（转换numpy类型为Python原生类型）
        def convert_to_native(obj):
            """递归转换numpy类型为Python原生类型"""
            if isinstance(obj, dict):
                return {k: convert_to_native(v) for k, v in obj.items()}
            elif isinstance(obj, list):
                return [convert_to_native(item) for item in obj]
            elif isinstance(obj, (np.integer, np.floating)):
                return float(obj)
            elif isinstance(obj, np.ndarray):
                return obj.tolist()
            else:
                return obj
        
        report_file = self.output_dir / 'ab_test_report.json'
        with open(report_file, 'w') as f:
            report_data = {
                'test_results': convert_to_native(test_results),
                'lightgbm_summary': {
                    'count': len(self.lightgbm_results),
                    'avg_reward': float(np.mean(lgb_rewards)) if self.lightgbm_results else 0,
                    'avg_compression': float(np.mean(lgb_compression)) if self.lightgbm_results else 0,
                    'avg_ssim': float(np.mean(lgb_ssim)) if self.lightgbm_results else 0,
                },
                'ppo_summary': {
                    'count': len(self.ppo_results),
                    'avg_reward': float(np.mean(ppo_rewards)) if self.ppo_results else 0,
                    'avg_compression': float(np.mean(ppo_compression)) if self.ppo_results else 0,
                    'avg_ssim': float(np.mean(ppo_ssim)) if self.ppo_results else 0,
                }
            }
            json.dump(report_data, f, indent=2)
        
        print(f"\n💾 Detailed report saved: {report_file}")

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description='A/B测试: PPO vs LightGBM')
    parser.add_argument('--test-dir', default='data/training_samples',
                       help='测试图像目录')
    parser.add_argument('--output-dir', default='test_output/ab_test',
                       help='输出目录')
    parser.add_argument('--max-images', type=int, default=50,
                       help='最大测试图像数量')
    
    args = parser.parse_args()
    
    # 运行测试
    runner = ABTestRunner(args.test_dir, args.output_dir)
    runner.run_test(args.max_images)

if __name__ == '__main__':
    main()
