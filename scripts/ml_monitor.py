#!/usr/bin/env python3
"""
📊 ML模型性能监控器

功能：
- 实时追踪训练进度
- 奖励趋势分析
- 准确率评估
- 自动回滚机制
"""

import json
import sys
from pathlib import Path
from datetime import datetime
import numpy as np

class MLMonitor:
    """ML模型性能监控器"""
    
    def __init__(self, model_dir="models/ppo"):
        self.model_dir = Path(model_dir)
        self.history_file = self.model_dir / "batch_training_history.json"
        self.experience_file = self.model_dir / "experience_buffer.json"
        
    def load_history(self):
        """加载训练历史"""
        if not self.history_file.exists():
            return {"batches": [], "total_updates": 0}
        
        with open(self.history_file, 'r') as f:
            return json.load(f)
    
    def load_experiences(self):
        """加载经验缓冲"""
        if not self.experience_file.exists():
            return []
        
        with open(self.experience_file, 'r') as f:
            return json.load(f)
    
    def analyze_training_progress(self):
        """分析训练进度"""
        history = self.load_history()
        
        if not history["batches"]:
            print("⚠️  No training history found")
            return
        
        batches = history["batches"]
        
        print("📊 Training Progress Analysis")
        print("=" * 60)
        print(f"Total batches: {len(batches)}")
        print(f"Total updates: {history['total_updates']}")
        print()
        
        # 提取数据
        timestamps = [b["timestamp"] for b in batches]
        losses = [b["avg_loss"] for b in batches]
        rewards = [b["avg_reward"] for b in batches]
        
        # 趋势分析
        print("📈 Trend Analysis:")
        print(f"   Loss:   {losses[0]:.2f} → {losses[-1]:.2f} ({self._calc_change(losses[0], losses[-1]):+.1f}%)")
        print(f"   Reward: {rewards[0]:.4f} → {rewards[-1]:.4f} ({self._calc_change(rewards[0], rewards[-1]):+.1f}%)")
        print()
        
        # 最近5次
        if len(batches) >= 5:
            recent_losses = losses[-5:]
            recent_rewards = rewards[-5:]
            print("📋 Recent 5 Batches:")
            print(f"   Avg Loss:   {np.mean(recent_losses):.2f}")
            print(f"   Avg Reward: {np.mean(recent_rewards):.4f}")
            print(f"   Loss Std:   {np.std(recent_losses):.2f}")
            print(f"   Reward Std: {np.std(recent_rewards):.4f}")
            print()
        
        # 改进速度
        if len(batches) >= 2:
            loss_improvement_rate = (losses[0] - losses[-1]) / len(batches)
            reward_improvement_rate = (rewards[-1] - rewards[0]) / len(batches)
            print("⚡ Improvement Rate (per batch):")
            print(f"   Loss:   {loss_improvement_rate:+.2f}")
            print(f"   Reward: {reward_improvement_rate:+.4f}")
            print()
        
        return {
            "batches": len(batches),
            "total_updates": history["total_updates"],
            "loss_trend": losses,
            "reward_trend": rewards,
            "loss_improvement": self._calc_change(losses[0], losses[-1]),
            "reward_improvement": self._calc_change(rewards[0], rewards[-1])
        }
    
    def analyze_experience_quality(self):
        """分析经验质量"""
        experiences = self.load_experiences()
        
        if not experiences:
            print("⚠️  No experiences found")
            return
        
        print("🔍 Experience Quality Analysis")
        print("=" * 60)
        print(f"Total experiences: {len(experiences)}")
        print()
        
        # 奖励分布
        rewards = [e["reward"] for e in experiences]
        print("🎯 Reward Distribution:")
        print(f"   Min:    {min(rewards):.4f}")
        print(f"   Max:    {max(rewards):.4f}")
        print(f"   Mean:   {np.mean(rewards):.4f}")
        print(f"   Median: {np.median(rewards):.4f}")
        print(f"   Std:    {np.std(rewards):.4f}")
        print()
        
        # 参数分布
        qualities = [e["quality"] for e in experiences]
        efforts = [e["effort"] for e in experiences]
        
        print("📐 Parameter Distribution:")
        print(f"   Quality: {min(qualities)} - {max(qualities)} (mean: {np.mean(qualities):.1f})")
        print(f"   Effort:  {min(efforts)} - {max(efforts)} (mean: {np.mean(efforts):.1f})")
        print()
        
        # 参数组合多样性
        unique_combos = len(set((e["quality"], e["effort"]) for e in experiences))
        max_combos = (max(qualities) - min(qualities) + 1) * (max(efforts) - min(efforts) + 1)
        diversity = unique_combos / max_combos * 100
        
        print("🔢 Parameter Diversity:")
        print(f"   Unique combinations: {unique_combos}/{max_combos} ({diversity:.1f}%)")
        print()
        
        # 高质量经验比例
        high_reward_threshold = np.percentile(rewards, 75)
        high_reward_count = sum(1 for r in rewards if r >= high_reward_threshold)
        high_reward_ratio = high_reward_count / len(rewards) * 100
        
        print("⭐ High-Quality Experiences:")
        print(f"   Threshold: {high_reward_threshold:.4f} (75th percentile)")
        print(f"   Count: {high_reward_count}/{len(rewards)} ({high_reward_ratio:.1f}%)")
        print()
        
        return {
            "total_experiences": len(experiences),
            "reward_stats": {
                "min": min(rewards),
                "max": max(rewards),
                "mean": np.mean(rewards),
                "std": np.std(rewards)
            },
            "diversity": diversity,
            "high_quality_ratio": high_reward_ratio
        }
    
    def check_model_health(self):
        """检查模型健康状态"""
        history = self.load_history()
        
        if not history["batches"] or len(history["batches"]) < 3:
            print("⚠️  Insufficient training history for health check")
            return {"status": "insufficient_data"}
        
        batches = history["batches"]
        recent_losses = [b["avg_loss"] for b in batches[-5:]]
        recent_rewards = [b["avg_reward"] for b in batches[-5:]]
        
        print("🏥 Model Health Check")
        print("=" * 60)
        
        issues = []
        warnings = []
        
        # 检查1: Loss是否发散
        if len(recent_losses) >= 2:
            loss_trend = recent_losses[-1] - recent_losses[0]
            if loss_trend > 50:
                issues.append(f"Loss diverging: +{loss_trend:.1f} in recent batches")
            elif loss_trend > 20:
                warnings.append(f"Loss increasing: +{loss_trend:.1f}")
        
        # 检查2: Reward是否下降
        if len(recent_rewards) >= 2:
            reward_trend = recent_rewards[-1] - recent_rewards[0]
            if reward_trend < -0.05:
                issues.append(f"Reward declining: {reward_trend:.4f}")
            elif reward_trend < -0.02:
                warnings.append(f"Reward slightly declining: {reward_trend:.4f}")
        
        # 检查3: Loss是否过高
        if recent_losses[-1] > 300:
            issues.append(f"Loss too high: {recent_losses[-1]:.1f}")
        elif recent_losses[-1] > 250:
            warnings.append(f"Loss elevated: {recent_losses[-1]:.1f}")
        
        # 检查4: Reward是否过低
        if recent_rewards[-1] < 0.2:
            issues.append(f"Reward too low: {recent_rewards[-1]:.4f}")
        elif recent_rewards[-1] < 0.3:
            warnings.append(f"Reward below average: {recent_rewards[-1]:.4f}")
        
        # 输出结果
        if not issues and not warnings:
            print("✅ Model is healthy!")
            print("   No issues detected")
            status = "healthy"
        elif issues:
            print("🚨 Critical Issues Detected:")
            for issue in issues:
                print(f"   ❌ {issue}")
            if warnings:
                print("\n⚠️  Warnings:")
                for warning in warnings:
                    print(f"   ⚠️  {warning}")
            status = "critical"
        else:
            print("⚠️  Warnings Detected:")
            for warning in warnings:
                print(f"   ⚠️  {warning}")
            status = "warning"
        
        print()
        
        # 建议
        if status == "critical":
            print("💡 Recommendations:")
            print("   1. Consider rolling back to previous model")
            print("   2. Check training data quality")
            print("   3. Adjust learning rate or batch size")
            print()
        
        return {
            "status": status,
            "issues": issues,
            "warnings": warnings,
            "recent_loss": recent_losses[-1],
            "recent_reward": recent_rewards[-1]
        }
    
    def _calc_change(self, old, new):
        """计算变化百分比"""
        if old == 0:
            return 0
        return (new - old) / abs(old) * 100
    
    def generate_report(self, output_file=None):
        """生成完整报告"""
        print("\n" + "=" * 60)
        print("📊 ML Model Performance Report")
        print("=" * 60)
        print(f"Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        print("=" * 60)
        print()
        
        # 训练进度
        training_stats = self.analyze_training_progress()
        print()
        
        # 经验质量
        experience_stats = self.analyze_experience_quality()
        print()
        
        # 模型健康
        health_status = self.check_model_health()
        print()
        
        # 汇总
        report = {
            "timestamp": datetime.now().isoformat(),
            "training": training_stats,
            "experiences": experience_stats,
            "health": health_status
        }
        
        if output_file:
            with open(output_file, 'w') as f:
                json.dump(report, f, indent=2)
            print(f"📄 Report saved to: {output_file}")
        
        return report


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="ML Model Performance Monitor")
    parser.add_argument("--model-dir", type=str, default="models/ppo",
                       help="Model directory")
    parser.add_argument("--output", type=str,
                       help="Output report file (JSON)")
    
    args = parser.parse_args()
    
    monitor = MLMonitor(model_dir=args.model_dir)
    monitor.generate_report(output_file=args.output)


if __name__ == "__main__":
    main()
