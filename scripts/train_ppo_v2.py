#!/usr/bin/env python3
"""
🤖 PPO强化学习训练脚本 V2 - Phase 3.1

使用Proximal Policy Optimization训练参数选择策略

架构：
- Actor网络：特征(128维) → 参数(quality, effort)
- Critic网络：特征(128维) → 预期奖励
- 奖励函数：压缩率 + 质量保持 + 速度

训练流程：
1. 从图像提取特征
2. Actor预测参数
3. 执行转换
4. 计算奖励
5. 更新策略
"""

import torch
import torch.nn as nn
import torch.optim as optim
from torch.distributions import Normal
import numpy as np
import json
import subprocess
from pathlib import Path
from datetime import datetime
import argparse

# 检查PyTorch是否可用
try:
    import torch
    TORCH_AVAILABLE = True
except ImportError:
    TORCH_AVAILABLE = False
    print("⚠️  PyTorch not installed. Install: pip install torch")

class ActorNetwork(nn.Module):
    """
    Actor网络：策略网络
    
    输入：128维图像特征
    输出：2维参数（quality, effort）
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
            nn.Linear(hidden_dim // 2, 2)  # quality, effort
        )
        
        # 初始化权重
        self._init_weights()
    
    def _init_weights(self):
        for m in self.modules():
            if isinstance(m, nn.Linear):
                nn.init.xavier_uniform_(m.weight)
                nn.init.constant_(m.bias, 0)

    
    def forward(self, state):
        """
        前向传播
        
        输出：
        - quality: 60-95 (sigmoid映射)
        - effort: 4-9 (sigmoid映射)
        """
        output = self.fc(state)
        
        # 使用sigmoid映射到目标范围
        quality = torch.sigmoid(output[:, 0]) * 35 + 60  # 60-95
        effort = torch.sigmoid(output[:, 1]) * 5 + 4     # 4-9
        
        return quality, effort
    
    def get_action(self, state, deterministic=False):
        """
        获取动作（带探索噪声）
        
        Args:
            state: 特征向量
            deterministic: 是否确定性输出（测试时用）
        
        Returns:
            quality, effort, log_prob
        """
        quality, effort = self.forward(state)
        
        if deterministic:
            return quality, effort, None
        
        # 添加探索噪声（正态分布）
        quality_noise = torch.randn_like(quality) * 2.0
        effort_noise = torch.randn_like(effort) * 0.5
        
        quality_noisy = torch.clamp(quality + quality_noise, 60, 95)
        effort_noisy = torch.clamp(effort + effort_noise, 4, 9)
        
        # 计算log概率（用于PPO更新）
        log_prob_quality = -0.5 * ((quality_noisy - quality) / 2.0) ** 2
        log_prob_effort = -0.5 * ((effort_noisy - effort) / 0.5) ** 2
        log_prob = log_prob_quality + log_prob_effort
        
        return quality_noisy, effort_noisy, log_prob

class CriticNetwork(nn.Module):
    """
    Critic网络：价值网络
    
    输入：128维图像特征
    输出：预期奖励（标量）
    """
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
        """预测状态价值"""
        return self.fc(state).squeeze(-1)


class PPOTrainer:
    """PPO训练器"""
    
    def __init__(self, state_dim=128, lr=3e-4, gamma=0.99, 
                 clip_epsilon=0.2, device='cpu'):
        self.device = device
        self.gamma = gamma
        self.clip_epsilon = clip_epsilon
        
        # 初始化网络
        self.actor = ActorNetwork(state_dim).to(device)
        self.critic = CriticNetwork(state_dim).to(device)
        
        # 优化器
        self.actor_optimizer = optim.Adam(self.actor.parameters(), lr=lr)
        self.critic_optimizer = optim.Adam(self.critic.parameters(), lr=lr)
        
        # 经验缓冲
        self.states = []
        self.actions = []
        self.rewards = []
        self.log_probs = []
        self.values = []
    
    def select_action(self, state, deterministic=False):
        """选择动作"""
        state_tensor = torch.FloatTensor(state).unsqueeze(0).to(self.device)
        
        with torch.no_grad():
            quality, effort, log_prob = self.actor.get_action(
                state_tensor, deterministic
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
        """计算回报（带折扣）"""
        returns = []
        R = 0
        for reward in reversed(self.rewards):
            R = reward + self.gamma * R
            returns.insert(0, R)
        return returns
    
    def update(self):
        """PPO更新"""
        if len(self.states) == 0:
            return 0.0, 0.0
        
        # 转换为tensor
        states = torch.FloatTensor(np.array(self.states)).to(self.device)
        actions = torch.FloatTensor(np.array(self.actions)).to(self.device)
        old_log_probs = torch.FloatTensor(self.log_probs).to(self.device)
        returns = torch.FloatTensor(self.compute_returns()).to(self.device)
        old_values = torch.FloatTensor(self.values).to(self.device)
        
        # 计算优势函数
        advantages = returns - old_values
        advantages = (advantages - advantages.mean()) / (advantages.std() + 1e-8)
        
        # PPO更新（多个epoch）
        actor_losses = []
        critic_losses = []
        
        for _ in range(10):  # PPO epochs
            # 重新计算log_prob和value
            quality, effort = self.actor(states)
            new_actions = torch.stack([quality, effort], dim=1)
            
            # 简化的log_prob计算
            log_prob_diff = -torch.sum((new_actions - actions) ** 2, dim=1)
            
            values = self.critic(states)
            
            # Actor loss (PPO clip)
            ratio = torch.exp(log_prob_diff - old_log_probs)
            surr1 = ratio * advantages
            surr2 = torch.clamp(ratio, 1 - self.clip_epsilon, 
                               1 + self.clip_epsilon) * advantages
            actor_loss = -torch.min(surr1, surr2).mean()
            
            # Critic loss
            critic_loss = nn.MSELoss()(values, returns)
            
            # 更新
            self.actor_optimizer.zero_grad()
            actor_loss.backward()
            self.actor_optimizer.step()
            
            self.critic_optimizer.zero_grad()
            critic_loss.backward()
            self.critic_optimizer.step()
            
            actor_losses.append(actor_loss.item())
            critic_losses.append(critic_loss.item())
        
        # 清空缓冲
        self.states = []
        self.actions = []
        self.rewards = []
        self.log_probs = []
        self.values = []
        
        return np.mean(actor_losses), np.mean(critic_losses)
    
    def save(self, path):
        """保存模型"""
        torch.save({
            'actor': self.actor.state_dict(),
            'critic': self.critic.state_dict(),
        }, path)
    
    def load(self, path):
        """加载模型"""
        checkpoint = torch.load(path)
        self.actor.load_state_dict(checkpoint['actor'])
        self.critic.load_state_dict(checkpoint['critic'])


def extract_features(image_path):
    """提取128维特征"""
    try:
        result = subprocess.run(
            ['cargo', 'run', '--release', '--bin', 'pixly-converter', 
             '--', 'analyze', str(image_path)],
            capture_output=True,
            text=True,
            timeout=30
        )
        
        if result.returncode != 0:
            return None
        
        output = result.stdout
        if "Features (128-dim):" in output:
            features_line = output.split("Features (128-dim):")[1].split("\n")[0].strip()
            features_str = features_line.strip('[]')
            features = [float(x.strip()) for x in features_str.split(',')]
            return features
        
        return None
    except Exception as e:
        print(f"❌ Error extracting features: {e}")
        return None

def convert_and_measure(input_path, quality, effort, target_format='webp'):
    """
    执行转换并测量结果
    
    返回：
    - reward: 奖励值
    - result: 转换结果详情
    """
    import tempfile
    import os
    import time
    
    # 创建临时输出文件
    with tempfile.NamedTemporaryFile(suffix=f'.{target_format}', delete=False) as tmp:
        output_path = tmp.name
    
    try:
        input_size = os.path.getsize(input_path)
        
        # 执行转换
        start_time = time.time()
        result = subprocess.run(
            ['cargo', 'run', '--release', '--bin', 'pixly-converter',
             '--', 'convert', str(input_path), output_path,
             '--format', target_format,
             '--quality', str(int(quality)),
             '--effort', str(int(effort))],
            capture_output=True,
            text=True,
            timeout=60
        )
        processing_time = time.time() - start_time
        
        if result.returncode != 0 or not os.path.exists(output_path):
            return -1.0, None
        
        output_size = os.path.getsize(output_path)
        
        # 计算SSIM（简化版，使用ImageMagick）
        ssim = calculate_ssim(input_path, output_path)
        
        # 计算奖励
        reward = calculate_reward(input_size, output_size, ssim, processing_time)
        
        result_dict = {
            'input_size': input_size,
            'output_size': output_size,
            'ssim': ssim,
            'processing_time': processing_time,
            'reward': reward
        }
        
        return reward, result_dict
        
    finally:
        # 清理临时文件
        if os.path.exists(output_path):
            os.remove(output_path)

def calculate_ssim(original, converted):
    """计算SSIM（使用ImageMagick）"""
    try:
        result = subprocess.run(
            ['magick', 'compare', '-metric', 'SSIM',
             str(original), str(converted), 'null:'],
            capture_output=True,
            text=True,
            timeout=30
        )
        stderr = result.stderr.strip()
        if stderr:
            ssim_str = stderr.split()[0]
            return float(ssim_str)
        return 0.95
    except:
        return 0.95

def calculate_reward(original_size, output_size, ssim, processing_time):
    """
    计算奖励
    
    对应Rust的reward_calculator.rs
    """
    # 压缩奖励
    if output_size >= original_size:
        compression_reward = 0.0
    else:
        reduction = (original_size - output_size) / original_size
        compression_reward = min(1.0, max(0.0, reduction))
    
    # 质量惩罚
    quality_threshold = 0.95
    if ssim >= quality_threshold:
        quality_penalty = 0.0
    else:
        quality_loss = quality_threshold - ssim
        quality_penalty = -quality_loss * 2.0
    
    # 速度惩罚
    speed_threshold = 5.0
    if processing_time <= speed_threshold:
        speed_penalty = 0.0
    else:
        time_excess = processing_time - speed_threshold
        speed_penalty = -time_excess * 0.1
    
    total_reward = compression_reward + quality_penalty + speed_penalty
    return total_reward


def train_ppo(data_dir, epochs=100, batch_size=10, target_format='webp'):
    """
    训练PPO模型
    
    Args:
        data_dir: 训练数据目录
        epochs: 训练轮数
        batch_size: 每轮处理的图像数
        target_format: 目标格式
    """
    if not TORCH_AVAILABLE:
        print("❌ PyTorch not available")
        return
    
    print("="*60)
    print("🤖 PPO Training V2")
    print("="*60)
    print(f"Data directory: {data_dir}")
    print(f"Epochs: {epochs}")
    print(f"Batch size: {batch_size}")
    print(f"Target format: {target_format}")
    print("="*60)
    
    # 初始化训练器
    device = 'cuda' if torch.cuda.is_available() else 'cpu'
    print(f"Device: {device}")
    
    trainer = PPOTrainer(device=device)
    
    # 扫描图像
    data_path = Path(data_dir)
    image_extensions = ['.jpg', '.jpeg', '.png', '.webp']
    image_files = []
    for ext in image_extensions:
        image_files.extend(data_path.glob(f'**/*{ext}'))
    
    print(f"Found {len(image_files)} images")
    
    if len(image_files) == 0:
        print("❌ No images found")
        return
    
    # 训练循环
    best_avg_reward = -float('inf')
    training_history = []
    
    for epoch in range(epochs):
        print(f"\n{'='*60}")
        print(f"Epoch {epoch + 1}/{epochs}")
        print(f"{'='*60}")
        
        # 随机选择batch_size个图像
        import random
        batch_images = random.sample(image_files, min(batch_size, len(image_files)))
        
        epoch_rewards = []
        
        for idx, image_file in enumerate(batch_images, 1):
            print(f"\n[{idx}/{len(batch_images)}] Processing: {image_file.name}")
            
            # 提取特征
            features = extract_features(image_file)
            if features is None:
                print("   ⚠️  Failed to extract features")
                continue
            
            # 选择动作
            quality, effort, log_prob, value = trainer.select_action(features)
            print(f"   🎯 Action: quality={quality:.1f}, effort={effort:.1f}")
            
            # 执行转换
            reward, result = convert_and_measure(
                image_file, quality, effort, target_format
            )
            
            if result:
                print(f"   📊 Result:")
                print(f"      Size: {result['input_size']/1024:.1f}KB → {result['output_size']/1024:.1f}KB")
                print(f"      SSIM: {result['ssim']:.4f}")
                print(f"      Time: {result['processing_time']:.2f}s")
                print(f"      Reward: {reward:.4f}")
                
                # 存储转换
                trainer.store_transition(
                    features,
                    [quality, effort],
                    reward,
                    log_prob,
                    value
                )
                
                epoch_rewards.append(reward)
            else:
                print(f"   ❌ Conversion failed")
        
        # 更新策略
        if len(epoch_rewards) > 0:
            actor_loss, critic_loss = trainer.update()
            avg_reward = np.mean(epoch_rewards)
            
            print(f"\n📊 Epoch {epoch + 1} Summary:")
            print(f"   Avg Reward: {avg_reward:.4f}")
            print(f"   Actor Loss: {actor_loss:.4f}")
            print(f"   Critic Loss: {critic_loss:.4f}")
            
            training_history.append({
                'epoch': epoch + 1,
                'avg_reward': avg_reward,
                'actor_loss': actor_loss,
                'critic_loss': critic_loss
            })
            
            # 保存最佳模型
            if avg_reward > best_avg_reward:
                best_avg_reward = avg_reward
                model_path = Path('models/ppo') / f'ppo_best_{target_format}.pth'
                model_path.parent.mkdir(exist_ok=True, parents=True)
                trainer.save(model_path)
                print(f"   💾 Saved best model: {model_path}")
        
        # 每10轮保存一次
        if (epoch + 1) % 10 == 0:
            model_path = Path('models/ppo') / f'ppo_epoch{epoch+1}_{target_format}.pth'
            trainer.save(model_path)
            print(f"   💾 Checkpoint saved: {model_path}")
    
    # 保存训练历史
    history_file = Path('models/ppo') / f'training_history_{target_format}_{datetime.now().strftime("%Y%m%d_%H%M%S")}.json'
    with open(history_file, 'w') as f:
        json.dump(training_history, f, indent=2)
    
    print(f"\n{'='*60}")
    print(f"✅ Training complete!")
    print(f"{'='*60}")
    print(f"Best avg reward: {best_avg_reward:.4f}")
    print(f"Training history saved: {history_file}")

def main():
    parser = argparse.ArgumentParser(description="Train PPO model V2")
    parser.add_argument('--data-dir', default='data/training_samples',
                       help='Training data directory')
    parser.add_argument('--epochs', type=int, default=100,
                       help='Number of training epochs')
    parser.add_argument('--batch-size', type=int, default=10,
                       help='Batch size per epoch')
    parser.add_argument('--format', default='webp',
                       help='Target format (webp/avif/jxl)')
    
    args = parser.parse_args()
    
    train_ppo(
        args.data_dir,
        epochs=args.epochs,
        batch_size=args.batch_size,
        target_format=args.format
    )

if __name__ == '__main__':
    main()
