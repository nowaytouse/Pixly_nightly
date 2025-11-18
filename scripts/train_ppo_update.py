#!/usr/bin/env python3
"""
🎓 PPO在线更新脚本 - Phase 3.2

用于在线学习的增量模型更新

输入：
- 经验文件（JSON格式）
- 现有模型路径

输出：
- 更新后的模型
"""

import torch
import json
import argparse
from pathlib import Path
import sys

# 导入PPO训练器
sys.path.append(str(Path(__file__).parent))
from train_ppo_v2 import PPOTrainer, ActorNetwork, CriticNetwork

def load_experiences(experience_file):
    """加载经验数据"""
    with open(experience_file, 'r') as f:
        experiences = json.load(f)
    
    print(f"📊 Loaded {len(experiences)} experiences")
    return experiences

def update_model(model_path, experiences, epochs=5):
    """更新模型"""
    if not Path(model_path).exists():
        print(f"❌ Model not found: {model_path}")
        return False
    
    # 初始化训练器
    device = 'cuda' if torch.cuda.is_available() else 'cpu'
    trainer = PPOTrainer(device=device)
    
    # 加载现有模型
    trainer.load(model_path)
    print(f"✅ Loaded model from {model_path}")
    
    # 准备经验数据
    for exp in experiences:
        trainer.store_transition(
            exp['features'],
            [exp['quality'], exp['effort']],
            exp['reward'],
            0.0,  # log_prob（在线学习时不需要）
            0.0   # value（会重新计算）
        )
    
    print(f"🎓 Updating model with {len(experiences)} experiences...")

    
    # 执行多个epoch的更新
    total_actor_loss = 0
    total_critic_loss = 0
    
    for epoch in range(epochs):
        actor_loss, critic_loss = trainer.update()
        total_actor_loss += actor_loss
        total_critic_loss += critic_loss
        print(f"   Epoch {epoch+1}/{epochs}: actor_loss={actor_loss:.4f}, critic_loss={critic_loss:.4f}")
    
    avg_actor_loss = total_actor_loss / epochs
    avg_critic_loss = total_critic_loss / epochs
    
    print(f"\n📊 Update Summary:")
    print(f"   Avg Actor Loss: {avg_actor_loss:.4f}")
    print(f"   Avg Critic Loss: {avg_critic_loss:.4f}")
    
    # 保存更新后的模型
    trainer.save(model_path)
    print(f"💾 Saved updated model to {model_path}")
    
    return True

def main():
    parser = argparse.ArgumentParser(description="PPO Online Update")
    parser.add_argument('--experiences', required=True, help='Experience file (JSON)')
    parser.add_argument('--model', required=True, help='Model path')
    parser.add_argument('--epochs', type=int, default=5, help='Update epochs')
    
    args = parser.parse_args()
    
    print("="*60)
    print("🎓 PPO Online Update")
    print("="*60)
    print(f"Experiences: {args.experiences}")
    print(f"Model: {args.model}")
    print(f"Epochs: {args.epochs}")
    print("="*60)
    
    # 加载经验
    experiences = load_experiences(args.experiences)
    
    if len(experiences) == 0:
        print("❌ No experiences to update")
        return 1
    
    # 更新模型
    success = update_model(args.model, experiences, args.epochs)
    
    if success:
        print("\n✅ Online update complete!")
        return 0
    else:
        print("\n❌ Online update failed!")
        return 1

if __name__ == '__main__':
    sys.exit(main())
