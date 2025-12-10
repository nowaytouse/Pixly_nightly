#!/usr/bin/env python3
"""
🚀 批量PPO更新器 - 一次性处理所有经验

优化：
- 一次加载所有经验
- 批量训练（batch_size=32）
- 避免重复的Python进程启动开销
"""

import sys
import torch
import json
import argparse
from pathlib import Path
from datetime import datetime

# 添加tools/training到Python路径
sys.path.insert(0, str(Path(__file__).parent.parent / "tools" / "training"))
from train_ppo_v3_optimized import OptimizedActorNetwork

class BatchPPOUpdater:
    """批量PPO更新器"""
    
    def __init__(self, model_dir="models/ppo", batch_size=32):
        self.model_dir = Path(model_dir)
        self.model_dir.mkdir(parents=True, exist_ok=True)
        self.batch_size = batch_size
        
        # 加载或创建Actor
        self.actor = OptimizedActorNetwork(state_dim=128, hidden_dim=256)
        self.optimizer = torch.optim.Adam(self.actor.parameters(), lr=1e-4)
        
        # 加载已有模型
        self.load_model()
        
        # 训练历史
        self.history_file = self.model_dir / "batch_training_history.json"
        self.history = self.load_history()
    
    def load_model(self):
        """加载已有模型"""
        # 优先加载在线模型
        model_path = self.model_dir / "actor_online.pth"
        if not model_path.exists():
            # 尝试加载其他模型
            model_path = self.model_dir / "actor_network.pth"
        
        if model_path.exists():
            try:
                self.actor.load_state_dict(torch.load(model_path))
                print(f"✅ Loaded existing model from {model_path}")
            except Exception as e:
                print(f"⚠️  Failed to load model: {e}, using new model")
        else:
            print("🆕 Creating new model")
    
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
        return {"batches": [], "total_updates": 0}
    
    def save_history(self):
        """保存训练历史"""
        with open(self.history_file, 'w') as f:
            json.dump(self.history, f, indent=2)
    
    def load_experiences(self, experience_file):
        """加载经验缓冲"""
        with open(experience_file, 'r') as f:
            experiences = json.load(f)
        print(f"📦 Loaded {len(experiences)} experiences")
        return experiences
    
    def batch_update(self, experiences):
        """
        批量更新模型
        
        Args:
            experiences: 经验列表
        """
        total_loss = 0.0
        num_batches = 0
        
        # 分批处理
        for i in range(0, len(experiences), self.batch_size):
            batch = experiences[i:i+self.batch_size]
            
            # 准备批量数据
            states = []
            quality_targets = []
            effort_targets = []
            rewards = []
            
            for exp in batch:
                states.append(exp['features'])
                quality_targets.append(exp['quality'])
                effort_targets.append(exp['effort'])
                rewards.append(exp['reward'])
            
            # 转换为tensor
            states = torch.FloatTensor(states)
            quality_targets = torch.FloatTensor(quality_targets)
            effort_targets = torch.FloatTensor(effort_targets)
            rewards = torch.FloatTensor(rewards)
            
            # 前向传播
            quality_pred, effort_pred = self.actor(states)
            
            # 计算损失
            quality_loss = ((quality_pred.squeeze() - quality_targets) ** 2).mean()
            effort_loss = ((effort_pred.squeeze() - effort_targets) ** 2).mean()
            
            # 奖励加权：高奖励的动作应该被强化
            # 使用 (1 - reward) 作为权重，低奖励的动作需要更多调整
            weights = 1.0 - rewards
            weighted_loss = (quality_loss + effort_loss) * weights.mean()
            
            # 反向传播
            self.optimizer.zero_grad()
            weighted_loss.backward()
            self.optimizer.step()
            
            total_loss += weighted_loss.item()
            num_batches += 1
            
            if (i // self.batch_size + 1) % 10 == 0:
                print(f"   Batch {i // self.batch_size + 1}/{(len(experiences) + self.batch_size - 1) // self.batch_size}: loss={weighted_loss.item():.4f}")
        
        avg_loss = total_loss / num_batches if num_batches > 0 else 0.0
        
        # 记录历史
        batch_record = {
            "timestamp": datetime.now().isoformat(),
            "num_experiences": len(experiences),
            "num_batches": num_batches,
            "avg_loss": avg_loss,
            "avg_reward": sum(e['reward'] for e in experiences) / len(experiences)
        }
        
        self.history["batches"].append(batch_record)
        self.history["total_updates"] += len(experiences)
        
        return {
            "success": True,
            "num_experiences": len(experiences),
            "num_batches": num_batches,
            "avg_loss": avg_loss,
            "total_updates": self.history["total_updates"]
        }


def main():
    parser = argparse.ArgumentParser(description="Batch PPO Updater")
    parser.add_argument("--experience-file", type=str, 
                       default="models/ppo/experience_buffer.json",
                       help="Experience buffer file")
    parser.add_argument("--model-dir", type=str, default="models/ppo",
                       help="Model directory")
    parser.add_argument("--batch-size", type=int, default=32,
                       help="Batch size for training")
    
    args = parser.parse_args()
    
    print("🚀 Batch PPO Model Update")
    print("=" * 60)
    
    # 创建更新器
    updater = BatchPPOUpdater(
        model_dir=args.model_dir,
        batch_size=args.batch_size
    )
    
    # 加载经验
    experiences = updater.load_experiences(args.experience_file)
    
    if len(experiences) == 0:
        print("⚠️  No experiences to update")
        return
    
    # 批量更新
    print(f"\n🎓 Training on {len(experiences)} experiences...")
    result = updater.batch_update(experiences)
    
    # 保存模型和历史
    updater.save_model()
    updater.save_history()
    
    # 输出结果
    print("\n" + "=" * 60)
    print("✅ Batch update complete!")
    print(f"   Experiences processed: {result['num_experiences']}")
    print(f"   Batches: {result['num_batches']}")
    print(f"   Average loss: {result['avg_loss']:.4f}")
    print(f"   Total updates: {result['total_updates']}")
    print("=" * 60)
    
    # 输出JSON结果（供Rust解析）
    print(json.dumps(result))


if __name__ == "__main__":
    main()
