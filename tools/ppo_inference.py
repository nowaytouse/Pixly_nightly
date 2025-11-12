#!/usr/bin/env python3
"""
PIXLY PPO推理服务
用于加载训练好的PPO模型并进行参数优化
"""

import json
import sys
import os
import torch
import torch.nn as nn
import numpy as np
from pathlib import Path

class ActorNetwork(nn.Module):
    """Actor网络 - 预测最优参数（必须与训练脚本完全一致）"""
    def __init__(self, state_dim=12, action_dim=4, hidden_dim=128):
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
            nn.init.orthogonal_(module.weight, gain=np.sqrt(2))
            nn.init.constant_(module.bias, 0)
    
    def forward(self, state):
        """推理时只返回均值（确定性策略）"""
        features = self.network(state)
        mean = self.mean_layer(features)
        return torch.tanh(mean)  # 推理时只用均值，训练时会用标准差


class PPOInference:
    """PPO推理器"""
    def __init__(self, model_dir="models/ppo"):
        self.model_dir = Path(model_dir)
        self.actor = ActorNetwork()
        self.device = torch.device("cpu")
        self._load_models()
    
    def _load_models(self):
        """加载训练好的模型"""
        actor_path = self.model_dir / "actor_network.pth"
        
        if not actor_path.exists():
            raise FileNotFoundError(f"Actor model not found: {actor_path}")
        
        self.actor.load_state_dict(torch.load(actor_path, map_location=self.device))
        self.actor.eval()
        print(f"✅ Loaded PPO model from {self.model_dir}", file=sys.stderr)
    
    def normalize_state(self, state_dict):
        """
        将输入状态归一化为模型需要的格式
        
        输入状态包含:
        - image_width, image_height, has_alpha, is_animation
        - input_size, optimize_mode (size/balanced/quality)
        - tool (jxl/avif/webp/heic)
        - current_params (quality, effort, speed, distance)
        """
        # 图像特征归一化
        width = state_dict.get('image_width', 1920) / 4096.0
        height = state_dict.get('image_height', 1080) / 4096.0
        has_alpha = 1.0 if state_dict.get('has_alpha', False) else 0.0
        is_animation = 1.0 if state_dict.get('is_animation', False) else 0.0
        input_size = state_dict.get('input_size', 1000000) / 10000000.0
        
        # 优化模式编码 (one-hot-like)
        mode_map = {'size': 0.0, 'balanced': 0.5, 'quality': 1.0}
        mode = mode_map.get(state_dict.get('optimize_mode', 'balanced'), 0.5)
        
        # 工具编码 (one-hot-like)
        tool_map = {'jxl': 0.0, 'avif': 0.33, 'webp': 0.67, 'heic': 1.0}
        tool = tool_map.get(state_dict.get('tool', 'jxl'), 0.0)
        
        # 当前参数归一化
        quality = state_dict.get('quality', 85) / 100.0
        effort = state_dict.get('effort', 7) / 10.0
        speed = state_dict.get('speed', 5) / 10.0
        distance = state_dict.get('distance', 1) / 10.0
        
        # 组合状态向量 (12维)
        state_vector = np.array([
            width, height, has_alpha, is_animation, input_size,
            mode, tool,
            quality, effort, speed, distance, 0.0  # 最后一维预留
        ], dtype=np.float32)
        
        return state_vector
    
    def denormalize_action(self, action_tensor, tool):
        """
        将模型输出的归一化动作转换为实际参数
        
        动作空间: [quality, effort, speed, distance] ∈ [-1, 1]
        """
        action = action_tensor.detach().cpu().numpy()[0]
        
        # 将[-1, 1]映射到合理范围
        quality = int((action[0] + 1) * 50)  # [0, 100]
        effort = int((action[1] + 1) * 5)    # [0, 10]
        speed = int((action[2] + 1) * 5)     # [0, 10]
        distance = max(0.0, (action[3] + 1) * 5)  # [0, 10]
        
        # 工具特定约束
        if tool == 'jxl':
            quality = min(100, max(50, quality))
            effort = min(10, max(3, effort))
            distance = min(10.0, max(0.0, distance))
        elif tool == 'avif':
            quality = min(100, max(40, quality))
            speed = min(10, max(0, speed))
        elif tool == 'webp':
            quality = min(100, max(50, quality))
        elif tool == 'heic':
            quality = min(100, max(40, quality))
        
        return {
            'quality': int(quality),
            'effort': int(effort),
            'speed': int(speed),
            'distance': float(round(distance, 1))
        }
    
    def predict(self, state_dict):
        """
        使用PPO模型预测最优参数
        
        Args:
            state_dict: 包含图像信息和当前参数的字典
        
        Returns:
            优化后的参数字典
        """
        # 归一化状态
        state_vector = self.normalize_state(state_dict)
        state_tensor = torch.FloatTensor(state_vector).unsqueeze(0).to(self.device)
        
        # 推理
        with torch.no_grad():
            action = self.actor(state_tensor)
        
        # 反归一化动作
        tool = state_dict.get('tool', 'jxl')
        params = self.denormalize_action(action, tool)
        
        return params


def main():
    """
    命令行接口
    
    用法:
        python tools/ppo_inference.py '{"image_width":1920,"image_height":1080,...}'
    
    输出:
        JSON格式的优化参数
    """
    if len(sys.argv) < 2:
        print(json.dumps({
            "error": "Missing state input",
            "usage": "python tools/ppo_inference.py '{...state_json...}'"
        }))
        sys.exit(1)
    
    try:
        # 解析输入状态
        state_json = sys.argv[1]
        state_dict = json.loads(state_json)
        
        # 创建推理器并预测
        inferencer = PPOInference()
        optimized_params = inferencer.predict(state_dict)
        
        # 输出结果
        result = {
            "success": True,
            "params": optimized_params,
            "model_version": "v1.0",
            "trained_episodes": 500
        }
        print(json.dumps(result))
        
    except FileNotFoundError as e:
        print(json.dumps({
            "success": False,
            "error": f"Model not found: {str(e)}",
            "hint": "Please train the PPO model first using: ./train_ppo_model.sh"
        }))
        sys.exit(1)
    
    except Exception as e:
        print(json.dumps({
            "success": False,
            "error": str(e)
        }))
        sys.exit(1)


if __name__ == "__main__":
    main()
