#!/usr/bin/env python3
"""
🤖 在线PPO训练器 - 每次转换后更新模型

架构：
- Rust CLI调用此脚本传递转换结果
- 计算奖励（文件大小减少 + SSIM质量）
- 更新PPO模型
- 持久化模型到磁盘
"""

import torch
import json
import sys
import argparse
from pathlib import Path
from datetime import datetime

# 导入PPO组件
from train_ppo_v3_optimized import OptimizedActorNetwork

class OnlinePPOTrainer:
    """在线PPO训练器"""
    
    def __init__(self, model_dir="models/ppo"):
        self.model_dir = Path(model_dir)
        self.model_dir.mkdir(parents=True, exist_ok=True)
        
        # 加载或创建Actor
        self.actor = OptimizedActorNetwork(state_dim=128, hidden_dim=256)
        self.optimizer = torch.optim.Adam(self.actor.parameters(), lr=1e-4)
        
        # 加载已有模型
        self.load_model()
        
        # 训练历史
        self.history_file = self.model_dir / "online_training_history.json"
        self.history = self.load_history()
    
    def load_model(self):
        """加载已有模型"""
        model_path = self.model_dir / "actor_online.pth"
        if model_path.exists():
            try:
                self.actor.load_state_dict(torch.load(model_path))
                print(f"✅ Loaded existing model from {model_path}")
            except Exception as e:
                print(f"⚠️  Failed to load model: {e}, using new model")
    
    def save_model(self):
        """保存模型"""
        model_path = self.model_dir / "actor_online.pth"
        torch.save(self.actor.state_dict(), model_path)
        print(f"💾 Model saved to {model_path}")
    
    def load_history(self):
        """加载训练历史"""
        if self.history_file.exists():
            with open(self.history_file, 'r') as f:
                return json.load(f)
        return {"updates": [], "total_updates": 0}
    
    def save_history(self):
        """保存训练历史"""
        with open(self.history_file, 'w') as f:
            json.dump(self.history, f, indent=2)
    
    def calculate_reward(self, conversion_result):
        """
        计算奖励
        
        奖励 = 文件大小减少比例 + SSIM质量保持
        
        Args:
            conversion_result: {
                "original_size": int,
                "converted_size": int,
                "ssim": float,
                "quality": int,
                "effort": int
            }
        """
        # 文件大小减少比例 (0-1)
        size_reduction = 1.0 - (conversion_result["converted_size"] / conversion_result["original_size"])
        size_reduction = max(0, min(1, size_reduction))  # 限制在[0,1]
        
        # SSIM质量 (0-1)
        ssim = conversion_result.get("ssim", 0.95)
        
        # 组合奖励
        # 权重: 50% 文件大小, 50% 质量
        reward = 0.5 * size_reduction + 0.5 * ssim
        
        # 惩罚: SSIM < 0.9 时额外惩罚
        if ssim < 0.9:
            reward -= 0.2 * (0.9 - ssim)
        
        return reward
    
    def update(self, features, action, reward):
        """
        更新模型（简化的策略梯度）
        
        Args:
            features: 128维特征向量
            action: (quality, effort)
            reward: 奖励值
        """
        # 转换为tensor
        state = torch.FloatTensor(features).unsqueeze(0)
        quality_target = torch.FloatTensor([action[0]])
        effort_target = torch.FloatTensor([action[1]])
        
        # 前向传播
        quality_pred, effort_pred = self.actor(state)
        
        # 计算损失（MSE + 奖励加权）
        quality_loss = ((quality_pred - quality_target) ** 2).mean()
        effort_loss = ((effort_pred - effort_target) ** 2).mean()
        
        # 奖励加权：高奖励的动作应该被强化
        loss = (quality_loss + effort_loss) * (1.0 - reward)
        
        # 反向传播
        self.optimizer.zero_grad()
        loss.backward()
        self.optimizer.step()
        
        return loss.item()
    
    def train_online(self, conversion_result):
        """
        在线训练
        
        Args:
            conversion_result: {
                "features": [128维特征],
                "action": {"quality": int, "effort": int},
                "original_size": int,
                "converted_size": int,
                "ssim": float
            }
        """
        # 计算奖励
        reward = self.calculate_reward(conversion_result)
        
        # 更新模型
        features = conversion_result["features"]
        action = (conversion_result["action"]["quality"], conversion_result["action"]["effort"])
        loss = self.update(features, action, reward)
        
        # 记录历史
        update_record = {
            "timestamp": datetime.now().isoformat(),
            "reward": reward,
            "loss": loss,
            "action": conversion_result["action"],
            "size_reduction": 1.0 - (conversion_result["converted_size"] / conversion_result["original_size"]),
            "ssim": conversion_result.get("ssim", 0.95)
        }
        
        self.history["updates"].append(update_record)
        self.history["total_updates"] += 1
        
        # 每10次更新保存一次
        if self.history["total_updates"] % 10 == 0:
            self.save_model()
            self.save_history()
        
        return {
            "success": True,
            "reward": reward,
            "loss": loss,
            "total_updates": self.history["total_updates"]
        }


def main():
    parser = argparse.ArgumentParser(description="Online PPO Trainer")
    parser.add_argument("--conversion-result", type=str, required=True,
                       help="JSON string of conversion result")
    parser.add_argument("--model-dir", type=str, default="models/ppo",
                       help="Model directory")
    
    args = parser.parse_args()
    
    # 解析转换结果
    try:
        conversion_result = json.loads(args.conversion_result)
    except json.JSONDecodeError as e:
        print(json.dumps({"success": False, "error": f"Invalid JSON: {e}"}))
        sys.exit(1)
    
    # 创建训练器
    trainer = OnlinePPOTrainer(model_dir=args.model_dir)
    
    # 在线训练
    result = trainer.train_online(conversion_result)
    
    # 输出结果
    print(json.dumps(result))


if __name__ == "__main__":
    main()
