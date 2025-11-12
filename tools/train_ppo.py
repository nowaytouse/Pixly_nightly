#!/usr/bin/env python3
"""
🤖 PIXLY PPO强化学习训练系统
版本: 1.0.0
日期: 2025-11-03

功能:
- 从观测数据训练PPO模型
- Actor-Critic网络架构
- 自适应参数优化
- 模型保存和评估
"""

import os
import sys
import json
import argparse
import glob
from datetime import datetime
from typing import List, Dict, Tuple
import numpy as np

# PyTorch导入（带错误处理）
try:
    import torch
    import torch.nn as nn
    import torch.optim as optim
    from torch.distributions import Normal
except ImportError:
    print("❌ 错误: PyTorch未安装")
    print("💡 安装命令: pip3 install torch")
    sys.exit(1)


class ActorNetwork(nn.Module):
    """Actor网络（策略网络）- 输出动作概率"""
    
    def __init__(self, state_dim: int, action_dim: int, hidden_dim: int = 128):
        super(ActorNetwork, self).__init__()
        
        self.network = nn.Sequential(
            nn.Linear(state_dim, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, hidden_dim // 2),
            nn.ReLU()
        )
        
        # 输出均值和标准差
        self.mean_layer = nn.Linear(hidden_dim // 2, action_dim)
        self.log_std_layer = nn.Linear(hidden_dim // 2, action_dim)
        
        # 初始化权重
        self.apply(self._init_weights)
    
    def _init_weights(self, module):
        if isinstance(module, nn.Linear):
            # 使用更保守的初始化
            nn.init.xavier_uniform_(module.weight, gain=0.5)  # 更小的增益
            nn.init.constant_(module.bias, 0)
    
    def forward(self, state):
        """前向传播"""
        x = self.network(state)
        mean = self.mean_layer(x)
        log_std = self.log_std_layer(x)
        log_std = torch.clamp(log_std, min=-20, max=2)  # 限制标准差范围
        
        # 限制mean范围防止数值溢出
        mean = torch.clamp(mean, min=-10, max=10)
        
        return mean, log_std
    
    def get_action(self, state, deterministic=False):
        """获取动作"""
        mean, log_std = self.forward(state)
        std = torch.exp(log_std)
        
        if deterministic:
            return mean
        
        # 使用正态分布采样
        dist = Normal(mean, std)
        action = dist.sample()
        return action
    
    def evaluate_action(self, state, action):
        """评估动作的对数概率和熵"""
        mean, log_std = self.forward(state)
        std = torch.exp(log_std)
        
        dist = Normal(mean, std)
        log_prob = dist.log_prob(action).sum(dim=-1)
        entropy = dist.entropy().sum(dim=-1)
        
        return log_prob, entropy


class CriticNetwork(nn.Module):
    """Critic网络（价值网络）- 估计状态价值"""
    
    def __init__(self, state_dim: int, hidden_dim: int = 128):
        super(CriticNetwork, self).__init__()
        
        self.network = nn.Sequential(
            nn.Linear(state_dim, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, hidden_dim // 2),
            nn.ReLU(),
            nn.Linear(hidden_dim // 2, 1)
        )
        
        # 初始化权重
        self.apply(self._init_weights)
    
    def _init_weights(self, module):
        if isinstance(module, nn.Linear):
            # 使用更保守的初始化
            nn.init.xavier_uniform_(module.weight, gain=0.5)  # 更小的增益
            nn.init.constant_(module.bias, 0)
    
    def forward(self, state):
        """前向传播"""
        return self.network(state).squeeze(-1)


class PPOTrainer:
    """PPO训练器"""
    
    def __init__(self, config: Dict):
        self.config = config
        
        # 设备
        self.device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')
        print(f"🖥️  使用设备: {self.device}")
        
        # 网络
        state_dim = config['state_dim']
        action_dim = config['action_dim']
        
        self.actor = ActorNetwork(state_dim, action_dim).to(self.device)
        self.critic = CriticNetwork(state_dim).to(self.device)
        
        # 优化器
        self.actor_optimizer = optim.Adam(
            self.actor.parameters(),
            lr=config['learning_rate']
        )
        self.critic_optimizer = optim.Adam(
            self.critic.parameters(),
            lr=config['learning_rate']
        )
        
        # 超参数
        self.gamma = config['gamma']
        self.clip_epsilon = config['clip_epsilon']
        self.value_coef = config['value_coefficient']
        self.entropy_coef = config['entropy_coefficient']
        self.max_grad_norm = config['max_grad_norm']
        self.ppo_epochs = config['ppo_epochs']
        
        # 统计
        self.training_stats = {
            'actor_losses': [],
            'critic_losses': [],
            'total_rewards': [],
            'episode_lengths': []
        }
    
    def train_step(self, states, actions, old_log_probs, returns, advantages):
        """执行一步PPO训练"""
        
        # 转换为tensor并检查NaN
        states = torch.FloatTensor(states).to(self.device)
        actions = torch.FloatTensor(actions).to(self.device)
        old_log_probs = torch.FloatTensor(old_log_probs).to(self.device)
        returns = torch.FloatTensor(returns).to(self.device)
        advantages = torch.FloatTensor(advantages).to(self.device)
        
        # 检查输入数据
        if torch.isnan(states).any() or torch.isnan(actions).any():
            print("⚠️  警告: 输入数据包含NaN，跳过此轮训练")
            return 0.0, 0.0
        
        # 标准化advantages（增强数值稳定性）
        advantages = (advantages - advantages.mean()) / (advantages.std() + 1e-8)
        advantages = torch.clamp(advantages, -10, 10)  # 限制范围防止梯度爆炸
        
        actor_losses = []
        critic_losses = []
        
        # PPO多轮更新
        for _ in range(self.ppo_epochs):
            # 评估当前策略
            log_probs, entropy = self.actor.evaluate_action(states, actions)
            values = self.critic(states)
            
            # 检查NaN
            if torch.isnan(log_probs).any() or torch.isnan(values).any():
                print("⚠️  警告: 前向传播出现NaN，跳过此轮更新")
                continue
            
            # Actor损失（PPO clip）
            ratio = torch.exp(log_probs - old_log_probs)
            ratio = torch.clamp(ratio, 0.1, 10)  # 防止比率过大
            surr1 = ratio * advantages
            surr2 = torch.clamp(ratio, 1 - self.clip_epsilon, 1 + self.clip_epsilon) * advantages
            actor_loss = -torch.min(surr1, surr2).mean()
            
            # 添加熵正则化（鼓励探索）
            entropy_loss = entropy.mean()
            if not torch.isnan(entropy_loss):
                actor_loss = actor_loss - self.entropy_coef * entropy_loss
            
            # Critic损失
            critic_loss = nn.MSELoss()(values, returns)
            
            # 检查损失是否有效
            if torch.isnan(actor_loss) or torch.isnan(critic_loss):
                print("⚠️  警告: 损失计算出现NaN，跳过此轮更新")
                continue
            
            # 更新Actor
            self.actor_optimizer.zero_grad()
            actor_loss.backward(retain_graph=True)
            torch.nn.utils.clip_grad_norm_(self.actor.parameters(), self.max_grad_norm)
            
            # 检查梯度
            has_nan_grad = False
            for param in self.actor.parameters():
                if param.grad is not None and torch.isnan(param.grad).any():
                    has_nan_grad = True
                    break
            
            if not has_nan_grad:
                self.actor_optimizer.step()
            else:
                print("⚠️  警告: Actor梯度包含NaN，跳过此次更新")
            
            # 更新Critic
            self.critic_optimizer.zero_grad()
            critic_loss.backward()
            torch.nn.utils.clip_grad_norm_(self.critic.parameters(), self.max_grad_norm)
            
            # 检查梯度
            has_nan_grad = False
            for param in self.critic.parameters():
                if param.grad is not None and torch.isnan(param.grad).any():
                    has_nan_grad = True
                    break
            
            if not has_nan_grad:
                self.critic_optimizer.step()
            else:
                print("⚠️  警告: Critic梯度包含NaN，跳过此次更新")
            
            actor_losses.append(actor_loss.item())
            critic_losses.append(critic_loss.item())
        
        return np.mean(actor_losses), np.mean(critic_losses)
    
    def save_model(self, output_dir: str, episode: int):
        """保存模型"""
        os.makedirs(output_dir, exist_ok=True)
        
        torch.save({
            'episode': episode,
            'actor_state_dict': self.actor.state_dict(),
            'critic_state_dict': self.critic.state_dict(),
            'actor_optimizer_state_dict': self.actor_optimizer.state_dict(),
            'critic_optimizer_state_dict': self.critic_optimizer.state_dict(),
            'config': self.config,
            'training_stats': self.training_stats
        }, f'{output_dir}/ppo_checkpoint_ep{episode}.pth')
        
        # 同时保存最新模型
        torch.save(self.actor.state_dict(), f'{output_dir}/actor_network.pth')
        torch.save(self.critic.state_dict(), f'{output_dir}/critic_network.pth')
        
        print(f"💾 模型已保存: {output_dir}/")


class ObservationProcessor:
    """观测数据处理器"""
    
    def __init__(self, seeds_file: str):
        """初始化"""
        self.seeds_file = seeds_file
        self.seeds = self._load_seeds()
        
        # 特征维度
        self.state_features = [
            'pixel_count', 'aspect_ratio', 'has_alpha', 'complexity_score',
            'tool_jxl', 'tool_avif', 'tool_webp', 'tool_heic',
            'mode_size', 'mode_balanced', 'mode_quality', 'mode_universal'
        ]
        self.state_dim = len(self.state_features)
        
        # 动作维度 (quality, distance, effort, speed)
        self.action_dim = 4
    
    def _load_seeds(self) -> Dict:
        """加载种子库"""
        try:
            with open(self.seeds_file, 'r', encoding='utf-8') as f:
                return json.load(f)
        except Exception as e:
            print(f"⚠️  无法加载种子库: {e}")
            return {}
    
    def load_observations(self, obs_dir: str) -> List[Dict]:
        """加载观测数据"""
        observations = []
        files = glob.glob(f'{obs_dir}/*.json')
        
        print(f"📂 找到 {len(files)} 个观测文件")
        
        for file in files:
            try:
                with open(file, 'r', encoding='utf-8') as f:
                    obs = json.load(f)
                    observations.append(obs)
            except Exception as e:
                print(f"⚠️  无法加载 {file}: {e}")
        
        return observations
    
    def extract_state(self, obs: Dict) -> np.ndarray:
        """从观测提取状态特征"""
        # 像素数
        pixel_count = obs.get('input_size', 1000000) / 1000000.0  # 标准化
        
        # 宽高比（假设正方形如果没有提供）
        aspect_ratio = 1.0
        
        # 透明度
        has_alpha = float(obs.get('has_alpha', False))
        
        # 复杂度（根据文件类型估计）
        image_type = obs.get('image_type', '.jpg').lower()
        complexity_score = 0.8 if image_type in ['.jpg', '.jpeg'] else 0.5
        
        # 工具one-hot编码
        tool = obs.get('tool', 'jxl')
        tool_jxl = 1.0 if tool == 'jxl' else 0.0
        tool_avif = 1.0 if tool == 'avif' else 0.0
        tool_webp = 1.0 if tool == 'webp' else 0.0
        tool_heic = 1.0 if tool == 'heic' else 0.0
        
        # 模式one-hot编码
        mode = obs.get('optimize_mode', 'balanced')
        mode_size = 1.0 if mode == 'size' else 0.0
        mode_balanced = 1.0 if mode == 'balanced' else 0.0
        mode_quality = 1.0 if mode == 'quality' else 0.0
        mode_universal = 1.0 if mode == 'universal' else 0.0
        
        state = np.array([
            pixel_count, aspect_ratio, has_alpha, complexity_score,
            tool_jxl, tool_avif, tool_webp, tool_heic,
            mode_size, mode_balanced, mode_quality, mode_universal
        ], dtype=np.float32)
        
        return state
    
    def extract_action(self, obs: Dict) -> np.ndarray:
        """从观测提取动作"""
        params = obs.get('params', {})
        
        # 标准化到[0, 1]范围，处理None值
        quality = (params.get('quality') or 90) / 100.0
        distance = (params.get('distance') if params.get('distance') is not None else 1.0) / 4.0
        effort = (params.get('effort') or 7) / 9.0
        speed = (params.get('speed') or 5) / 10.0
        
        # 确保值在有效范围内
        quality = np.clip(quality, 0.0, 1.0)
        distance = np.clip(distance, 0.0, 1.0)
        effort = np.clip(effort, 0.0, 1.0)
        speed = np.clip(speed, 0.0, 1.0)
        
        action = np.array([quality, distance, effort, speed], dtype=np.float32)
        return action
    
    def extract_reward(self, obs: Dict) -> float:
        """从观测提取奖励"""
        return obs.get('reward', 0.0)


def compute_advantages(rewards: List[float], values: List[float], gamma: float = 0.99, lam: float = 0.95):
    """计算优势函数（GAE）"""
    advantages = []
    returns = []
    
    gae = 0
    for i in reversed(range(len(rewards))):
        if i == len(rewards) - 1:
            next_value = 0
        else:
            next_value = values[i + 1]
        
        delta = rewards[i] + gamma * next_value - values[i]
        gae = delta + gamma * lam * gae
        advantages.insert(0, gae)
        returns.insert(0, gae + values[i])
    
    return advantages, returns


def main():
    """主函数"""
    parser = argparse.ArgumentParser(description='PIXLY PPO训练系统')
    parser.add_argument('--observations_dir', type=str, default='data/observations',
                        help='观测数据目录')
    parser.add_argument('--seeds_file', type=str, 
                        default='data/training_seeds/format_optimized_seeds.json',
                        help='种子库文件')
    parser.add_argument('--output_dir', type=str, default='models/ppo',
                        help='模型输出目录')
    parser.add_argument('--episodes', type=int, default=100,
                        help='训练轮数')
    parser.add_argument('--batch_size', type=int, default=32,
                        help='批次大小')
    parser.add_argument('--learning_rate', type=float, default=0.0003,
                        help='学习率')
    parser.add_argument('--log_dir', type=str, default='logs/training',
                        help='日志目录')
    
    args = parser.parse_args()
    
    print("=" * 70)
    print("🤖 PIXLY PPO强化学习训练系统")
    print("=" * 70)
    print()
    
    # 加载观测数据
    processor = ObservationProcessor(args.seeds_file)
    observations = processor.load_observations(args.observations_dir)
    
    if len(observations) < 50:
        print(f"❌ 错误: 观测数据不足")
        print(f"🎯 当前: {len(observations)} 个")
        print(f"🎯 最少需要: 50 个")
        print()
        print("💡 请先使用插件转换更多图像以收集观测数据")
        return 1
    
    print(f"✅ 加载 {len(observations)} 个观测数据")
    print()
    
    # 准备训练数据
    states = []
    actions = []
    rewards = []
    
    for obs in observations:
        state = processor.extract_state(obs)
        action = processor.extract_action(obs)
        reward = processor.extract_reward(obs)
        
        states.append(state)
        actions.append(action)
        rewards.append(reward)
    
    states = np.array(states)
    actions = np.array(actions)
    rewards = np.array(rewards)
    
    print(f"📊 数据统计:")
    print(f"   状态维度: {states.shape}")
    print(f"   动作维度: {actions.shape}")
    print(f"   平均奖励: {np.mean(rewards):.4f}")
    print(f"   奖励范围: [{np.min(rewards):.4f}, {np.max(rewards):.4f}]")
    print()
    
    # 训练配置
    config = {
        'state_dim': processor.state_dim,
        'action_dim': processor.action_dim,
        'learning_rate': args.learning_rate,
        'gamma': 0.99,
        'clip_epsilon': 0.2,
        'value_coefficient': 0.5,
        'entropy_coefficient': 0.01,
        'max_grad_norm': 0.5,
        'ppo_epochs': 10
    }
    
    # 创建训练器
    trainer = PPOTrainer(config)
    
    # 训练循环
    print("🎓 开始训练...")
    print("=" * 70)
    
    for episode in range(args.episodes):
        # 计算价值
        with torch.no_grad():
            states_tensor = torch.FloatTensor(states).to(trainer.device)
            values = trainer.critic(states_tensor).cpu().numpy()
        
        # 计算优势和回报
        advantages, returns = compute_advantages(rewards.tolist(), values.tolist())
        
        # 计算旧的log概率
        with torch.no_grad():
            states_tensor = torch.FloatTensor(states).to(trainer.device)
            actions_tensor = torch.FloatTensor(actions).to(trainer.device)
            old_log_probs, _ = trainer.actor.evaluate_action(states_tensor, actions_tensor)
            old_log_probs = old_log_probs.cpu().numpy()
        
        # 训练
        actor_loss, critic_loss = trainer.train_step(
            states, actions, old_log_probs, returns, advantages
        )
        
        # 记录统计
        trainer.training_stats['actor_losses'].append(actor_loss)
        trainer.training_stats['critic_losses'].append(critic_loss)
        trainer.training_stats['total_rewards'].append(np.sum(rewards))
        
        # 打印进度
        if (episode + 1) % 10 == 0:
            print(f"Episode {episode + 1}/{args.episodes} | "
                  f"Actor Loss: {actor_loss:.4f} | "
                  f"Critic Loss: {critic_loss:.4f} | "
                  f"Avg Reward: {np.mean(rewards):.4f}")
        
        # 保存检查点
        if (episode + 1) % 50 == 0:
            trainer.save_model(args.output_dir, episode + 1)
    
    print()
    print("=" * 70)
    print("✅ 训练完成！")
    print()
    
    # 保存最终模型
    trainer.save_model(args.output_dir, args.episodes)
    
    # 保存训练统计
    stats_file = f'{args.output_dir}/training_stats.json'
    with open(stats_file, 'w') as f:
        json.dump(trainer.training_stats, f, indent=2)
    
    print(f"📊 训练统计已保存: {stats_file}")
    print(f"💾 模型文件:")
    print(f"   • {args.output_dir}/actor_network.pth")
    print(f"   • {args.output_dir}/critic_network.pth")
    print()
    print("🚀 下一步: 重启GO核心以加载新模型")
    print("   ./start_ai_service.sh")
    print()
    
    return 0


if __name__ == '__main__':
    sys.exit(main())
