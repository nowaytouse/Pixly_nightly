#!/usr/bin/env python3
"""
🤖 PPO强化学习训练脚本 V3 - Phase 3.3优化版

优化项：
1. Actor初始化偏向quality=75, effort=6
2. 减小探索噪声（2.0 → 1.0）
3. 调整Critic学习率
4. 添加奖励归一化
5. 改进训练循环
"""

import torch
import torch.nn as nn
import torch.optim as optim
import numpy as np
import json
import subprocess
import os
import sys
from pathlib import Path
from datetime import datetime
import argparse

# 🔥 Phase 修复: 实现v2的基础函数（v2文件丢失）
# 这些函数仅在训练时需要，batch_ppo_update只需要Actor网络

def extract_features(image_path):
    """提取图像特征（128维）- 仅训练时使用"""
    try:
        from PIL import Image
        import numpy as np
        
        img = Image.open(image_path)
        width, height = img.size
        pixels = width * height
        
        # 基础特征
        features = [
            width, height, pixels,
            width / height if height > 0 else 1.0,
            len(img.getbands()),
        ]
        
        # 填充到128维
        features.extend([0.0] * (128 - len(features)))
        return np.array(features[:128], dtype=np.float32)
    except Exception as e:
        print(f"❌ Feature extraction failed: {e}")
        return None

def convert_and_measure(image_path, quality, effort, target_format):
    """执行转换并测量结果- 仅训练时使用"""
    try:
        import subprocess
        
        output_path = image_path.parent / f"{image_path.stem}_converted.{target_format}"
        
        # 调用Rust CLI
        cmd = [
            "cargo", "run", "--release", "--bin", "pixly-converter", "--",
            "convert", str(image_path),
            "--format", target_format,
            "--quality", str(int(quality)),
            "--effort", str(int(effort)),
            "--output", str(image_path.parent)
        ]
        
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
        
        if result.returncode == 0 and output_path.exists():
            original_size = image_path.stat().st_size
            converted_size = output_path.stat().st_size
            compression_ratio = converted_size / original_size if original_size > 0 else 1.0
            
            # 计算SSIM
            ssim_score = calculate_ssim(image_path, output_path)
            
            # 计算奖励
            reward = calculate_reward(compression_ratio, ssim_score)
            
            return reward, {
                'compression_ratio': compression_ratio,
                'ssim': ssim_score,
                'original_size': original_size,
                'converted_size': converted_size
            }
        else:
            return -1.0, None
            
    except Exception as e:
        print(f"❌ Conversion failed: {e}")
        return -1.0, None

def calculate_ssim(img1_path, img2_path):
    """计算SSIM相似度 - 仅训练时使用"""
    try:
        from PIL import Image
        import numpy as np
        from skimage.metrics import structural_similarity as ssim
        
        img1 = np.array(Image.open(img1_path).convert('RGB'))
        img2 = np.array(Image.open(img2_path).convert('RGB'))
        
        # 确保尺寸相同
        if img1.shape != img2.shape:
            img2 = np.array(Image.fromarray(img2).resize((img1.shape[1], img1.shape[0])))
        
        return ssim(img1, img2, channel_axis=2, data_range=255)
    except Exception as e:
        print(f"❌ SSIM calculation failed: {e}")
        return 0.0

def calculate_reward(compression_ratio, ssim_score):
    """计算奖励值"""
    # 压缩率奖励（越小越好）
    compression_reward = (1.0 - compression_ratio) * 10.0
    
    # 质量奖励（SSIM越高越好）
    quality_reward = ssim_score * 10.0
    
    # 综合奖励
    return compression_reward + quality_reward

class OptimizedActorNetwork(nn.Module):
    """
    优化的Actor网络 - Phase 3.3
    
    改进：
    - 初始化偏向quality=75, effort=6
    - 减小探索噪声
    """
    def __init__(self, state_dim=128, hidden_dim=256):
        super().__init__()
        
        self.fc = nn.Sequential(
            nn.Linear(state_dim, hidden_dim),
            nn.ReLU(),
            nn.Dropout(0.2),
            nn.Linear(hidden_dim, hidden_dim // 2),
            nn.ReLU(),
            nn.Dropout(0.2),
            nn.Linear(hidden_dim // 2, 2)
        )
        
        self._init_weights()
    
    def _init_weights(self):
        """优化的权重初始化"""
        for m in self.modules():
            if isinstance(m, nn.Linear):
                nn.init.xavier_uniform_(m.weight)
                nn.init.constant_(m.bias, 0)
        
        # 🔥 Phase 3.3: 调整输出层bias
        # quality: sigmoid(bias) * 35 + 60 = 75 → sigmoid(bias) = 0.43 → bias ≈ -0.3
        # effort: sigmoid(bias) * 5 + 4 = 6 → sigmoid(bias) = 0.4 → bias ≈ -0.4
        with torch.no_grad():
            self.fc[-1].bias[0] = -0.3  # quality偏向75
            self.fc[-1].bias[1] = -0.4  # effort偏向6
    
    def forward(self, state):
        output = self.fc(state)
        quality = torch.sigmoid(output[:, 0]) * 35 + 60
        effort = torch.sigmoid(output[:, 1]) * 5 + 4
        return quality, effort
    
    def get_action(self, state, deterministic=False, noise_scale=1.0):
        """
        获取动作（优化的探索噪声）
        
        Phase 3.3: 减小噪声 (2.0 → 1.0)
        """
        quality, effort = self.forward(state)
        
        if deterministic:
            return quality, effort, None
        
        # 减小探索噪声
        quality_noise = torch.randn_like(quality) * noise_scale
        effort_noise = torch.randn_like(effort) * (noise_scale * 0.5)
        
        quality_noisy = torch.clamp(quality + quality_noise, 60, 95)
        effort_noisy = torch.clamp(effort + effort_noise, 4, 9)
        
        log_prob_quality = -0.5 * ((quality_noisy - quality) / noise_scale) ** 2
        log_prob_effort = -0.5 * ((effort_noisy - effort) / (noise_scale * 0.5)) ** 2
        log_prob = log_prob_quality + log_prob_effort
        
        return quality_noisy, effort_noisy, log_prob

class OptimizedCriticNetwork(nn.Module):
    """优化的Critic网络 - Phase 3.3"""
    def __init__(self, state_dim=128, hidden_dim=256):
        super().__init__()
        
        self.fc = nn.Sequential(
            nn.Linear(state_dim, hidden_dim),
            nn.ReLU(),
            nn.Dropout(0.2),
            nn.Linear(hidden_dim, hidden_dim // 2),
            nn.ReLU(),
            nn.Linear(hidden_dim // 2, 1)
        )
        
        self._init_weights()
    
    def _init_weights(self):
        for m in self.modules():
            if isinstance(m, nn.Linear):
                nn.init.xavier_uniform_(m.weight)
                nn.init.constant_(m.bias, 0)
    
    def forward(self, state):
        return self.fc(state).squeeze(-1)

class OptimizedPPOTrainer:
    """优化的PPO训练器 - Phase 3.3"""
    
    def __init__(self, state_dim=128, actor_lr=3e-4, critic_lr=1e-3,
                 gamma=0.99, clip_epsilon=0.2, device='cpu'):
        self.device = device
        self.gamma = gamma
        self.clip_epsilon = clip_epsilon
        
        # 使用优化的网络
        self.actor = OptimizedActorNetwork(state_dim).to(device)
        self.critic = OptimizedCriticNetwork(state_dim).to(device)
        
        # 🔥 Phase 3.3: 调整Critic学习率（更大）
        self.actor_optimizer = optim.Adam(self.actor.parameters(), lr=actor_lr)
        self.critic_optimizer = optim.Adam(self.critic.parameters(), lr=critic_lr)
        
        # 经验缓冲
        self.states = []
        self.actions = []
        self.rewards = []
        self.log_probs = []
        self.values = []
        
        # 🔥 Phase 3.3: 奖励归一化
        self.reward_mean = 0.0
        self.reward_std = 1.0
    
    def select_action(self, state, deterministic=False, noise_scale=1.0):
        """选择动作（支持调整噪声）"""
        state_tensor = torch.FloatTensor(state).unsqueeze(0).to(self.device)
        
        with torch.no_grad():
            quality, effort, log_prob = self.actor.get_action(
                state_tensor, deterministic, noise_scale
            )
            value = self.critic(state_tensor)
        
        return (
            quality.item(),
            effort.item(),
            log_prob.item() if log_prob is not None else None,
            value.item()
        )
    
    def store_transition(self, state, action, reward, log_prob, value):
        """存储转换"""
        self.states.append(state)
        self.actions.append(action)
        self.rewards.append(reward)
        self.log_probs.append(log_prob)
        self.values.append(value)
    
    def compute_returns(self):
        """计算回报"""
        returns = []
        R = 0
        for reward in reversed(self.rewards):
            R = reward + self.gamma * R
            returns.insert(0, R)
        return returns
    
    def normalize_rewards(self):
        """🔥 Phase 3.3: 奖励归一化"""
        if len(self.rewards) > 1:
            rewards_array = np.array(self.rewards)
            self.reward_mean = rewards_array.mean()
            self.reward_std = rewards_array.std() + 1e-8
            normalized = (rewards_array - self.reward_mean) / self.reward_std
            self.rewards = normalized.tolist()
    
    def update(self, normalize_rewards=True):
        """PPO更新（优化版）"""
        if len(self.states) == 0:
            return 0.0, 0.0
        
        # 🔥 Phase 3.3: 奖励归一化
        if normalize_rewards:
            self.normalize_rewards()
        
        states = torch.FloatTensor(np.array(self.states)).to(self.device)
        actions = torch.FloatTensor(np.array(self.actions)).to(self.device)
        old_log_probs = torch.FloatTensor(self.log_probs).to(self.device)
        returns = torch.FloatTensor(self.compute_returns()).to(self.device)
        old_values = torch.FloatTensor(self.values).to(self.device)
        
        advantages = returns - old_values
        advantages = (advantages - advantages.mean()) / (advantages.std() + 1e-8)
        
        actor_losses = []
        critic_losses = []
        
        for _ in range(10):
            quality, effort = self.actor(states)
            new_actions = torch.stack([quality, effort], dim=1)
            
            log_prob_diff = -torch.sum((new_actions - actions) ** 2, dim=1)
            values = self.critic(states)
            
            ratio = torch.exp(log_prob_diff - old_log_probs)
            surr1 = ratio * advantages
            surr2 = torch.clamp(ratio, 1 - self.clip_epsilon,
                               1 + self.clip_epsilon) * advantages
            actor_loss = -torch.min(surr1, surr2).mean()
            
            critic_loss = nn.MSELoss()(values, returns)
            
            self.actor_optimizer.zero_grad()
            actor_loss.backward()
            torch.nn.utils.clip_grad_norm_(self.actor.parameters(), 0.5)
            self.actor_optimizer.step()
            
            self.critic_optimizer.zero_grad()
            critic_loss.backward()
            torch.nn.utils.clip_grad_norm_(self.critic.parameters(), 0.5)
            self.critic_optimizer.step()
            
            actor_losses.append(actor_loss.item())
            critic_losses.append(critic_loss.item())
        
        self.states = []
        self.actions = []
        self.rewards = []
        self.log_probs = []
        self.values = []
        
        return np.mean(actor_losses), np.mean(critic_losses)
    
    def save(self, path):
        torch.save({
            'actor': self.actor.state_dict(),
            'critic': self.critic.state_dict(),
            'reward_mean': self.reward_mean,
            'reward_std': self.reward_std,
        }, path)
    
    def load(self, path):
        checkpoint = torch.load(path)
        self.actor.load_state_dict(checkpoint['actor'])
        self.critic.load_state_dict(checkpoint['critic'])
        if 'reward_mean' in checkpoint:
            self.reward_mean = checkpoint['reward_mean']
            self.reward_std = checkpoint['reward_std']

def train_ppo_optimized(data_dir, epochs=20, batch_size=10, target_format='webp',
                       noise_schedule='linear'):
    """
    优化的PPO训练
    
    Phase 3.3改进：
    - 更多epochs（20 vs 2）
    - 更大batch（10 vs 5）
    - 噪声衰减schedule
    - 奖励归一化
    """
    print("="*60)
    print("🤖 PPO Training V3 (Optimized)")
    print("="*60)
    print(f"Data directory: {data_dir}")
    print(f"Epochs: {epochs}")
    print(f"Batch size: {batch_size}")
    print(f"Target format: {target_format}")
    print(f"Noise schedule: {noise_schedule}")
    print("="*60)
    
    device = 'cuda' if torch.cuda.is_available() else 'cpu'
    print(f"Device: {device}")
    
    trainer = OptimizedPPOTrainer(device=device)
    
    data_path = Path(data_dir)
    image_extensions = ['.jpg', '.jpeg', '.png', '.webp']
    image_files = []
    for ext in image_extensions:
        image_files.extend(data_path.glob(f'**/*{ext}'))
    
    print(f"Found {len(image_files)} images")
    
    if len(image_files) == 0:
        print("❌ No images found")
        return
    
    best_avg_reward = -float('inf')
    training_history = []
    
    for epoch in range(epochs):
        # 🔥 Phase 3.3: 噪声衰减
        if noise_schedule == 'linear':
            noise_scale = 1.0 - (epoch / epochs) * 0.5  # 1.0 → 0.5
        else:
            noise_scale = 1.0
        
        print(f"\n{'='*60}")
        print(f"Epoch {epoch + 1}/{epochs} (noise_scale={noise_scale:.2f})")
        print(f"{'='*60}")
        
        import random
        batch_images = random.sample(image_files, min(batch_size, len(image_files)))
        
        epoch_rewards = []
        
        for idx, image_file in enumerate(batch_images, 1):
            print(f"\n[{idx}/{len(batch_images)}] Processing: {image_file.name}")
            
            features = extract_features(image_file)
            if features is None:
                print("   ⚠️  Failed to extract features")
                continue
            
            quality, effort, log_prob, value = trainer.select_action(
                features, noise_scale=noise_scale
            )
            print(f"   🎯 Action: quality={quality:.1f}, effort={effort:.1f}")
            
            reward, result = convert_and_measure(
                image_file, quality, effort, target_format
            )
            
            if result:
                print(f"   📊 Result:")
                print(f"      Size: {result['input_size']/1024:.1f}KB → {result['output_size']/1024:.1f}KB")
                print(f"      SSIM: {result['ssim']:.4f}")
                print(f"      Time: {result['processing_time']:.2f}s")
                print(f"      Reward: {reward:.4f}")
                
                trainer.store_transition(features, [quality, effort],
                                        reward, log_prob, value)
                epoch_rewards.append(reward)
            else:
                print(f"   ❌ Conversion failed")
        
        if len(epoch_rewards) > 0:
            actor_loss, critic_loss = trainer.update(normalize_rewards=True)
            avg_reward = np.mean(epoch_rewards)
            
            print(f"\n📊 Epoch {epoch + 1} Summary:")
            print(f"   Avg Reward: {avg_reward:.4f}")
            print(f"   Actor Loss: {actor_loss:.4f}")
            print(f"   Critic Loss: {critic_loss:.4f}")
            print(f"   Reward Mean: {trainer.reward_mean:.4f}")
            print(f"   Reward Std: {trainer.reward_std:.4f}")
            
            training_history.append({
                'epoch': epoch + 1,
                'avg_reward': avg_reward,
                'actor_loss': actor_loss,
                'critic_loss': critic_loss,
                'noise_scale': noise_scale
            })
            
            if avg_reward > best_avg_reward:
                best_avg_reward = avg_reward
                model_path = Path('models/ppo') / f'ppo_v3_best_{target_format}.pth'
                model_path.parent.mkdir(exist_ok=True, parents=True)
                trainer.save(model_path)
                print(f"   💾 Saved best model: {model_path}")
        
        if (epoch + 1) % 5 == 0:
            model_path = Path('models/ppo') / f'ppo_v3_epoch{epoch+1}_{target_format}.pth'
            trainer.save(model_path)
            print(f"   💾 Checkpoint saved: {model_path}")
    
    history_file = Path('models/ppo') / f'training_history_v3_{target_format}_{datetime.now().strftime("%Y%m%d_%H%M%S")}.json'
    with open(history_file, 'w') as f:
        json.dump(training_history, f, indent=2)
    
    print(f"\n{'='*60}")
    print(f"✅ Training complete!")
    print(f"{'='*60}")
    print(f"Best avg reward: {best_avg_reward:.4f}")
    print(f"Training history saved: {history_file}")

def main():
    parser = argparse.ArgumentParser(description="Train optimized PPO model V3")
    parser.add_argument('--data-dir', default='data/training_samples')
    parser.add_argument('--epochs', type=int, default=20)
    parser.add_argument('--batch-size', type=int, default=10)
    parser.add_argument('--format', default='webp')
    parser.add_argument('--noise-schedule', default='linear',
                       choices=['linear', 'constant'])
    
    args = parser.parse_args()
    
    train_ppo_optimized(
        args.data_dir,
        epochs=args.epochs,
        batch_size=args.batch_size,
        target_format=args.format,
        noise_schedule=args.noise_schedule
    )

if __name__ == '__main__':
    main()
