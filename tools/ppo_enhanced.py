"""
PPO强化学习架构增强版
基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/rl/ppo.go 重新实现

功能:
- 完整的PPO算法实现（Actor-Critic架构）
- 经验回放和GAE优势估计
- 多目标奖励函数设计
- 参数优化和超参数调优
- 模型保存和加载机制

EX-015实现: 从Go废弃代码价值提取
遵循四高原则：高规范化、高兼容性、高扩展性、高稳定性
"""

import numpy as np
import torch
import torch.nn as nn
import torch.optim as optim
import torch.nn.functional as F
from torch.distributions import Normal
import json
import logging
import os
import time
import platform
from pathlib import Path
from typing import Dict, List, Optional, Any, Tuple, Union, Callable
from dataclasses import dataclass, field, asdict
import contextlib
from collections import deque
import random


@dataclass
class PPOConfig:
    """PPO配置（与Go版本兼容）"""
    gamma: float = 0.99                    # 折扣因子
    epsilon: float = 0.2                   # PPO clip参数
    learning_rate: float = 3e-4            # 学习率
    batch_size: int = 64                   # 批量大小
    n_epochs: int = 10                     # 训练轮数
    lambda_gae: float = 0.95               # GAE lambda参数
    value_loss_coef: float = 0.5           # 价值函数损失系数
    entropy_coef: float = 0.01             # 熵正则化系数
    max_grad_norm: float = 0.5             # 梯度裁剪
    hidden_size: int = 128                 # 隐藏层大小
    buffer_size: int = 10000               # 经验缓冲区大小
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典"""
        return asdict(self)
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'PPOConfig':
        """从字典创建"""
        return cls(**data)


@dataclass
class State:
    """状态表示（与Go版本兼容）"""
    features: List[float]                  # 特征向量（SWT特征等）
    
    def to_tensor(self, device: torch.device = None) -> torch.Tensor:
        """转换为PyTorch张量"""
        tensor = torch.FloatTensor(self.features)
        if device:
            tensor = tensor.to(device)
        return tensor
    
    @classmethod
    def from_tensor(cls, tensor: torch.Tensor) -> 'State':
        """从张量创建状态"""
        return cls(features=tensor.cpu().numpy().tolist())


@dataclass
class Action:
    """动作表示（参数调整，与Go版本兼容）"""
    quality_delta: float = 0.0             # 质量参数调整
    distance_delta: float = 0.0            # distance参数调整
    effort_delta: float = 0.0              # effort参数调整（JXL）
    quantizer_delta: float = 0.0           # quantizer参数调整（AVIF）
    
    def to_tensor(self, device: torch.device = None) -> torch.Tensor:
        """转换为PyTorch张量"""
        values = [self.quality_delta, self.distance_delta, self.effort_delta, self.quantizer_delta]
        tensor = torch.FloatTensor(values)
        if device:
            tensor = tensor.to(device)
        return tensor
    
    @classmethod
    def from_tensor(cls, tensor: torch.Tensor) -> 'Action':
        """从张量创建动作"""
        values = tensor.cpu().numpy().tolist()
        return cls(
            quality_delta=values[0] if len(values) > 0 else 0.0,
            distance_delta=values[1] if len(values) > 1 else 0.0,
            effort_delta=values[2] if len(values) > 2 else 0.0,
            quantizer_delta=values[3] if len(values) > 3 else 0.0
        )
    
    def to_dict(self) -> Dict[str, float]:
        """转换为字典"""
        return asdict(self)


@dataclass
class Experience:
    """经验记录（与Go版本兼容）"""
    state: State
    action: Action
    reward: float
    next_state: Optional[State]
    done: bool
    value: float = 0.0                     # 状态价值
    log_prob: float = 0.0                  # 动作对数概率
    advantage: float = 0.0                 # 优势函数
    
    def to_dict(self) -> Dict[str, Any]:
        """转换为字典"""
        return {
            'state': self.state.features,
            'action': self.action.to_dict(),
            'reward': self.reward,
            'next_state': self.next_state.features if self.next_state else None,
            'done': self.done,
            'value': self.value,
            'log_prob': self.log_prob,
            'advantage': self.advantage
        }


class PolicyNetwork(nn.Module):
    """策略网络（Actor）"""
    
    def __init__(self, input_size: int, hidden_size: int, output_size: int):
        super(PolicyNetwork, self).__init__()
        
        self.fc1 = nn.Linear(input_size, hidden_size)
        self.fc2 = nn.Linear(hidden_size, hidden_size)
        self.mean_layer = nn.Linear(hidden_size, output_size)
        self.log_std_layer = nn.Linear(hidden_size, output_size)
        
        # Xavier初始化
        self._init_weights()
    
    def _init_weights(self):
        """初始化网络权重"""
        for m in self.modules():
            if isinstance(m, nn.Linear):
                nn.init.xavier_normal_(m.weight)
                nn.init.constant_(m.bias, 0.0)
    
    def forward(self, state: torch.Tensor) -> Tuple[torch.Tensor, torch.Tensor]:
        """前向传播"""
        x = F.relu(self.fc1(state))
        x = F.relu(self.fc2(x))
        
        mean = torch.tanh(self.mean_layer(x))  # 输出范围 [-1, 1]
        log_std = torch.clamp(self.log_std_layer(x), min=-20, max=2)
        std = torch.exp(log_std)
        
        return mean, std
    
    def get_action_and_value(self, state: torch.Tensor) -> Tuple[torch.Tensor, torch.Tensor, torch.Tensor]:
        """获取动作、对数概率和熵"""
        mean, std = self.forward(state)
        dist = Normal(mean, std)
        action = dist.sample()
        log_prob = dist.log_prob(action).sum(dim=-1)
        entropy = dist.entropy().sum(dim=-1)
        
        # 将动作限制在有效范围内
        action = torch.clamp(action, -1.0, 1.0)
        
        return action, log_prob, entropy


class ValueNetwork(nn.Module):
    """价值网络（Critic）"""
    
    def __init__(self, input_size: int, hidden_size: int):
        super(ValueNetwork, self).__init__()
        
        self.fc1 = nn.Linear(input_size, hidden_size)
        self.fc2 = nn.Linear(hidden_size, hidden_size)
        self.value_layer = nn.Linear(hidden_size, 1)
        
        # Xavier初始化
        self._init_weights()
    
    def _init_weights(self):
        """初始化网络权重"""
        for m in self.modules():
            if isinstance(m, nn.Linear):
                nn.init.xavier_normal_(m.weight)
                nn.init.constant_(m.bias, 0.0)
    
    def forward(self, state: torch.Tensor) -> torch.Tensor:
        """前向传播"""
        x = F.relu(self.fc1(state))
        x = F.relu(self.fc2(x))
        value = self.value_layer(x)
        
        return value.squeeze(-1)


class ReplayBuffer:
    """经验回放缓冲区（与Go版本兼容）"""
    
    def __init__(self, max_size: int):
        self.max_size = max_size
        self.experiences = deque(maxlen=max_size)
    
    def add(self, experience: Experience) -> None:
        """添加经验"""
        self.experiences.append(experience)
    
    def sample(self, batch_size: int) -> List[Experience]:
        """采样批量经验"""
        if len(self.experiences) <= batch_size:
            return list(self.experiences)
        
        return random.sample(self.experiences, batch_size)
    
    def get_all(self) -> List[Experience]:
        """获取所有经验"""
        return list(self.experiences)
    
    def size(self) -> int:
        """获取缓冲区大小"""
        return len(self.experiences)
    
    def clear(self) -> None:
        """清空缓冲区"""
        self.experiences.clear()


class PPOOptimizer:
    """
    PPO优化器
    高规范化、高兼容性、高扩展性、高稳定性实现
    """
    
    def __init__(self, 
                 config: PPOConfig,
                 state_size: int,
                 action_size: int = 4,
                 device: Optional[torch.device] = None,
                 debug: bool = False):
        """
        初始化PPO优化器
        
        Args:
            config: PPO配置
            state_size: 状态空间维度
            action_size: 动作空间维度
            device: 计算设备
            debug: 调试模式
        """
        self.config = config
        self.state_size = state_size
        self.action_size = action_size
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        # 设备选择（跨平台兼容）
        if device is None:
            if torch.cuda.is_available():
                self.device = torch.device('cuda')
            elif hasattr(torch.backends, 'mps') and torch.backends.mps.is_available():
                self.device = torch.device('mps')  # Apple Silicon
            else:
                self.device = torch.device('cpu')
        else:
            self.device = device
        
        if debug:
            self.logger.setLevel(logging.DEBUG)
            self.logger.debug(f"PPO优化器初始化，设备: {self.device}")
        
        # 初始化网络
        self.policy_network = PolicyNetwork(
            state_size, 
            config.hidden_size, 
            action_size
        ).to(self.device)
        
        self.value_network = ValueNetwork(
            state_size,
            config.hidden_size
        ).to(self.device)
        
        # 优化器
        self.policy_optimizer = optim.Adam(
            self.policy_network.parameters(),
            lr=config.learning_rate
        )
        self.value_optimizer = optim.Adam(
            self.value_network.parameters(),
            lr=config.learning_rate
        )
        
        # 经验缓冲区
        self.replay_buffer = ReplayBuffer(config.buffer_size)
        
        # 训练统计
        self.training_stats = {
            'total_updates': 0,
            'policy_losses': [],
            'value_losses': [],
            'entropies': []
        }
    
    def select_action(self, state: State) -> Tuple[Action, float, float]:
        """
        选择动作（与Go版本接口兼容）
        
        Args:
            state: 输入状态
            
        Returns:
            Tuple[Action, float, float]: (动作, 对数概率, 状态价值)
        """
        with torch.no_grad():
            state_tensor = state.to_tensor(self.device).unsqueeze(0)
            
            # 策略网络输出
            action_tensor, log_prob, _ = self.policy_network.get_action_and_value(state_tensor)
            
            # 价值网络输出
            value = self.value_network(state_tensor)
            
            # 转换动作格式（缩放到实际参数范围）
            action_scaled = self._scale_action(action_tensor.squeeze(0))
            action = Action.from_tensor(action_scaled)
            
            return action, log_prob.item(), value.item()
    
    def _scale_action(self, action_tensor: torch.Tensor) -> torch.Tensor:
        """将动作从[-1,1]缩放到实际参数范围"""
        # 动作缩放（与Go版本一致）
        scaled = action_tensor.clone()
        scaled[0] *= 10   # quality_delta: [-10, 10]
        scaled[1] *= 0.5  # distance_delta: [-0.5, 0.5]
        scaled[2] *= 2    # effort_delta: [-2, 2]
        scaled[3] *= 10   # quantizer_delta: [-10, 10]
        
        return scaled
    
    def compute_advantages(self, experiences: List[Experience]) -> None:
        """
        计算优势函数（GAE，与Go版本兼容）
        
        Args:
            experiences: 经验列表
        """
        n = len(experiences)
        if n == 0:
            return
        
        # 计算TD误差和优势函数
        advantages = []
        gae = 0
        
        for i in reversed(range(n)):
            exp = experiences[i]
            
            if exp.next_state is None or exp.done:
                next_value = 0.0
            else:
                with torch.no_grad():
                    next_state_tensor = exp.next_state.to_tensor(self.device).unsqueeze(0)
                    next_value = self.value_network(next_state_tensor).item()
            
            # TD误差
            td_error = exp.reward + self.config.gamma * next_value - exp.value
            
            # GAE
            gae = td_error + self.config.gamma * self.config.lambda_gae * gae
            advantages.insert(0, gae)
        
        # 标准化优势函数
        if len(advantages) > 1:
            advantages = np.array(advantages)
            mean = np.mean(advantages)
            std = np.std(advantages)
            
            if std > 1e-8:
                advantages = (advantages - mean) / std
            
            for i, adv in enumerate(advantages):
                experiences[i].advantage = float(adv)
        elif len(advantages) == 1:
            experiences[0].advantage = advantages[0]
    
    def update(self) -> Dict[str, float]:
        """
        更新网络参数（与Go版本接口兼容）
        
        Returns:
            Dict[str, float]: 训练统计信息
        """
        if self.replay_buffer.size() < self.config.batch_size:
            error_msg = f"insufficient experiences: {self.replay_buffer.size()} < {self.config.batch_size}"
            if self.debug:
                self.logger.warning(error_msg)
            return {'error': error_msg}
        
        # 获取所有经验
        experiences = self.replay_buffer.get_all()
        
        # 计算优势函数
        self.compute_advantages(experiences)
        
        # 准备训练数据
        states = torch.stack([exp.state.to_tensor(self.device) for exp in experiences])
        actions = torch.stack([exp.action.to_tensor(self.device) for exp in experiences])
        old_log_probs = torch.tensor([exp.log_prob for exp in experiences], 
                                   dtype=torch.float32, device=self.device)
        advantages = torch.tensor([exp.advantage for exp in experiences], 
                                dtype=torch.float32, device=self.device)
        values = torch.tensor([exp.value for exp in experiences], 
                            dtype=torch.float32, device=self.device)
        rewards = torch.tensor([exp.reward for exp in experiences], 
                             dtype=torch.float32, device=self.device)
        
        # 计算返回值
        returns = advantages + values
        
        # PPO更新
        policy_losses = []
        value_losses = []
        entropies = []
        
        for epoch in range(self.config.n_epochs):
            # 随机排列
            indices = torch.randperm(len(experiences))
            
            for start in range(0, len(experiences), self.config.batch_size):
                end = start + self.config.batch_size
                batch_indices = indices[start:end]
                
                # 批量数据
                batch_states = states[batch_indices]
                batch_actions = actions[batch_indices]
                batch_old_log_probs = old_log_probs[batch_indices]
                batch_advantages = advantages[batch_indices]
                batch_returns = returns[batch_indices]
                
                # 当前策略
                _, new_log_probs, entropy = self.policy_network.get_action_and_value(batch_states)
                current_values = self.value_network(batch_states)
                
                # 计算比率
                ratio = torch.exp(new_log_probs - batch_old_log_probs)
                
                # PPO clip损失
                surr1 = ratio * batch_advantages
                surr2 = torch.clamp(ratio, 1 - self.config.epsilon, 1 + self.config.epsilon) * batch_advantages
                policy_loss = -torch.min(surr1, surr2).mean()
                
                # 价值函数损失
                value_loss = F.mse_loss(current_values, batch_returns)
                
                # 熵损失
                entropy_loss = -entropy.mean()
                
                # 总损失
                total_loss = (policy_loss + 
                            self.config.value_loss_coef * value_loss + 
                            self.config.entropy_coef * entropy_loss)
                
                # 更新策略网络
                self.policy_optimizer.zero_grad()
                policy_loss.backward(retain_graph=True)
                torch.nn.utils.clip_grad_norm_(self.policy_network.parameters(), self.config.max_grad_norm)
                self.policy_optimizer.step()
                
                # 更新价值网络
                self.value_optimizer.zero_grad()
                value_loss.backward()
                torch.nn.utils.clip_grad_norm_(self.value_network.parameters(), self.config.max_grad_norm)
                self.value_optimizer.step()
                
                # 记录损失
                policy_losses.append(policy_loss.item())
                value_losses.append(value_loss.item())
                entropies.append(entropy.mean().item())
        
        # 清空缓冲区
        self.replay_buffer.clear()
        
        # 更新统计
        self.training_stats['total_updates'] += 1
        self.training_stats['policy_losses'].extend(policy_losses)
        self.training_stats['value_losses'].extend(value_losses)
        self.training_stats['entropies'].extend(entropies)
        
        # 计算平均损失
        avg_policy_loss = np.mean(policy_losses) if policy_losses else 0.0
        avg_value_loss = np.mean(value_losses) if value_losses else 0.0
        avg_entropy = np.mean(entropies) if entropies else 0.0
        
        if self.debug:
            self.logger.debug(f"PPO更新完成，策略损失: {avg_policy_loss:.4f}, "
                            f"价值损失: {avg_value_loss:.4f}, 熵: {avg_entropy:.4f}")
        
        return {
            'policy_loss': avg_policy_loss,
            'value_loss': avg_value_loss,
            'entropy': avg_entropy,
            'total_updates': self.training_stats['total_updates']
        }
    
    def optimize(self,
                 initial_state: State,
                 reward_function: Callable[[State, Action], Tuple[float, State, bool]],
                 max_steps: int) -> Tuple[Action, float]:
        """
        优化参数（与Go版本接口兼容）
        
        Args:
            initial_state: 初始状态
            reward_function: 奖励函数
            max_steps: 最大步数
            
        Returns:
            Tuple[Action, float]: (最佳动作, 最佳奖励)
        """
        state = initial_state
        best_action = Action()
        best_reward = float('-inf')
        
        for step in range(max_steps):
            # 选择动作
            action, log_prob, value = self.select_action(state)
            
            # 执行动作，获取奖励
            reward, next_state, done = reward_function(state, action)
            
            # 存储经验
            experience = Experience(
                state=state,
                action=action,
                reward=reward,
                next_state=next_state,
                done=done,
                value=value,
                log_prob=log_prob
            )
            self.replay_buffer.add(experience)
            
            # 记录最佳动作
            if reward > best_reward:
                best_reward = reward
                best_action = action
            
            # 更新网络
            if self.replay_buffer.size() >= self.config.batch_size:
                update_info = self.update()
                if 'error' not in update_info and self.debug:
                    self.logger.debug(f"步骤{step}: 奖励={reward:.4f}, 最佳奖励={best_reward:.4f}")
            
            if done:
                break
            
            state = next_state
        
        return best_action, best_reward
    
    def save_model(self, path: str) -> None:
        """保存模型"""
        save_path = Path(path)
        save_path.parent.mkdir(parents=True, exist_ok=True)
        
        checkpoint = {
            'config': self.config.to_dict(),
            'state_size': self.state_size,
            'action_size': self.action_size,
            'policy_network': self.policy_network.state_dict(),
            'value_network': self.value_network.state_dict(),
            'policy_optimizer': self.policy_optimizer.state_dict(),
            'value_optimizer': self.value_optimizer.state_dict(),
            'training_stats': self.training_stats
        }
        
        torch.save(checkpoint, save_path)
        
        if self.debug:
            self.logger.debug(f"模型保存到: {save_path}")
    
    def load_model(self, path: str) -> None:
        """加载模型"""
        checkpoint = torch.load(path, map_location=self.device)
        
        self.policy_network.load_state_dict(checkpoint['policy_network'])
        self.value_network.load_state_dict(checkpoint['value_network'])
        self.policy_optimizer.load_state_dict(checkpoint['policy_optimizer'])
        self.value_optimizer.load_state_dict(checkpoint['value_optimizer'])
        self.training_stats = checkpoint.get('training_stats', {})
        
        if self.debug:
            self.logger.debug(f"模型加载自: {path}")
    
    def get_training_stats(self) -> Dict[str, Any]:
        """获取训练统计信息"""
        return self.training_stats.copy()


# 奖励函数（与Go版本兼容）
def compute_reward(quality: float, size: int, target_size: int, ssim: float) -> float:
    """
    计算奖励（多目标奖励函数，与Go版本兼容）
    
    Args:
        quality: 质量参数
        size: 文件大小
        target_size: 目标文件大小
        ssim: SSIM指标
        
    Returns:
        float: 总奖励
    """
    # 权重系数（与Go版本一致）
    w1, w2, w3 = 0.4, 0.4, 0.2
    
    # 质量奖励（SSIM）
    quality_reward = ssim
    
    # 体积奖励（压缩率）
    size_ratio = size / max(target_size, 1)
    size_reward = max(0.0, 1.0 - size_ratio)
    
    # 质量参数奖励（接近目标质量）
    quality_penalty = abs(quality - 90) / 90.0  # 假设目标是Q90
    
    total_reward = w1 * quality_reward + w2 * size_reward - w3 * quality_penalty
    
    return max(0.0, total_reward)  # 确保奖励非负


# 便捷函数
def create_default_ppo_optimizer(state_size: int, 
                                action_size: int = 4,
                                debug: bool = False) -> PPOOptimizer:
    """
    创建默认PPO优化器
    
    Args:
        state_size: 状态空间维度
        action_size: 动作空间维度  
        debug: 调试模式
        
    Returns:
        PPOOptimizer: PPO优化器实例
    """
    config = PPOConfig()
    return PPOOptimizer(config, state_size, action_size, debug=debug)


if __name__ == "__main__":
    # 测试代码
    print("=== PPO强化学习架构测试 ===")
    
    # 设置随机种子
    torch.manual_seed(42)
    np.random.seed(42)
    random.seed(42)
    
    # 创建PPO优化器
    state_size = 10  # 假设状态特征维度
    ppo = create_default_ppo_optimizer(state_size, debug=True)
    
    print(f"✅ PPO优化器初始化成功，设备: {ppo.device}")
    print(f"   状态维度: {state_size}, 动作维度: {ppo.action_size}")
    
    # 测试状态和动作
    test_state = State(features=[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0])
    action, log_prob, value = ppo.select_action(test_state)
    
    print(f"✅ 动作选择测试:")
    print(f"   动作: {action.to_dict()}")
    print(f"   对数概率: {log_prob:.4f}")
    print(f"   状态价值: {value:.4f}")
    
    # 测试奖励函数
    reward = compute_reward(quality=80, size=500000, target_size=600000, ssim=0.92)
    print(f"✅ 奖励计算测试: {reward:.4f}")
    
    # 简单优化测试
    def simple_reward_function(state: State, action: Action) -> Tuple[float, State, bool]:
        # 简单的测试奖励函数
        reward = compute_reward(80 + action.quality_delta, 500000, 600000, 0.92)
        next_state = State(features=[f + 0.01 for f in state.features])
        done = False
        return reward, next_state, done
    
    print("🚀 开始简单优化测试...")
    best_action, best_reward = ppo.optimize(
        initial_state=test_state,
        reward_function=simple_reward_function,
        max_steps=50
    )
    
    print(f"✅ 优化完成:")
    print(f"   最佳动作: {best_action.to_dict()}")
    print(f"   最佳奖励: {best_reward:.4f}")
    
    # 训练统计
    stats = ppo.get_training_stats()
    print(f"   训练更新次数: {stats.get('total_updates', 0)}")
    
    # 测试模型保存和加载
    model_path = "test_ppo_model.pth"
    ppo.save_model(model_path)
    print(f"✅ 模型保存测试完成: {model_path}")
    
    # 创建新的优化器并加载模型
    ppo2 = create_default_ppo_optimizer(state_size)
    ppo2.load_model(model_path)
    print(f"✅ 模型加载测试完成")
    
    # 清理测试文件
    if os.path.exists(model_path):
        os.remove(model_path)
    
    print("🎯 所有测试完成！PPO强化学习架构实现成功！")
