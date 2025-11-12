"""
贝叶斯参数优化器
基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/precision_modes.go 重新实现

功能:
- 基于历史观测的智能参数调优
- UCB（Upper Confidence Bound）探索策略
- 贝叶斯后验推理
- 针对不同目标模式的优化
- 质量-压缩率权衡自动优化

EX-012实现: 从Go废弃代码价值提取
"""

import numpy as np
import json
from typing import List, Dict, Tuple, Optional
from dataclasses import dataclass
from pathlib import Path
import sqlite3
import logging

@dataclass
class Observation:
    """观测记录"""
    quality: int
    distance: float
    reward: float
    file_size: int
    ssim: float
    target_mode: str
    tool: str = ""
    timestamp: int = 0

    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            'quality': self.quality,
            'distance': self.distance,
            'reward': self.reward,
            'file_size': self.file_size,
            'ssim': self.ssim,
            'target_mode': self.target_mode,
            'tool': self.tool,
            'timestamp': self.timestamp
        }

class BayesianOptimizer:
    """
    贝叶斯优化器（基础模式）
    特点：仅优化工具参数（quality, distance, effort等）
    """
    
    def __init__(self, tool: str, quality_range: Tuple[int, int] = (1, 100), 
                 exploration_factor: float = 2.0, debug: bool = False):
        """
        初始化贝叶斯优化器
        
        Args:
            tool: 工具名称 (jxl/avif/webp/aac/opus等)
            quality_range: 质量参数范围 [min, max]
            exploration_factor: UCB探索因子
            debug: 调试模式
        """
        self.tool = tool
        self.quality_range = quality_range
        self.observations: List[Observation] = []
        self.prior_mean = float(sum(quality_range)) / 2
        self.prior_std = float(quality_range[1] - quality_range[0]) / 4
        self.exploration_factor = exploration_factor
        self.debug = debug
        
        if self.debug:
            logging.basicConfig(level=logging.DEBUG)
            self.logger = logging.getLogger(__name__)

    def add_observation(self, obs: Observation) -> None:
        """添加观测"""
        obs.tool = self.tool
        obs.timestamp = int(np.datetime64('now').astype(int))
        self.observations.append(obs)
        
        if self.debug:
            self.logger.debug(f"Added observation: quality={obs.quality}, reward={obs.reward:.3f}")

    def predict_optimal(self, target_mode: str = "balanced") -> Tuple[int, float]:
        """
        预测最优参数（贝叶斯后验）
        
        Args:
            target_mode: 目标模式 (size/balanced/quality)
            
        Returns:
            Tuple[optimal_quality, optimal_distance]: 最优质量和距离参数
        """
        if not self.observations:
            # 无历史数据，返回先验均值
            quality = int(self.prior_mean)
            distance = self._quality_to_distance(quality)
            
            if self.debug:
                self.logger.debug(f"No observations, using prior: quality={quality}, distance={distance:.3f}")
            
            return quality, distance
        
        # 过滤相关观测
        relevant_obs = [obs for obs in self.observations if obs.target_mode == target_mode]
        
        if not relevant_obs:
            # 无相关模式数据，使用所有观测
            relevant_obs = self.observations
        
        # UCB-based 优化
        best_quality = self._ucb_optimization(relevant_obs)
        best_distance = self._quality_to_distance(best_quality)
        
        if self.debug:
            self.logger.debug(f"UCB optimization: quality={best_quality}, distance={best_distance:.3f}")
        
        return best_quality, best_distance

    def _ucb_optimization(self, observations: List[Observation]) -> int:
        """
        Upper Confidence Bound 优化
        """
        # 创建质量点网格
        quality_points = np.arange(self.quality_range[0], self.quality_range[1] + 1, 1)
        best_quality = int(self.prior_mean)
        best_ucb = float('-inf')
        
        for quality in quality_points:
            # 计算该质量点的均值和置信区间
            mean, std = self._posterior_estimation(quality, observations)
            
            # UCB = 均值 + 探索因子 × 标准差
            ucb = mean + self.exploration_factor * std
            
            if ucb > best_ucb:
                best_ucb = ucb
                best_quality = quality
        
        return best_quality

    def _posterior_estimation(self, quality: int, observations: List[Observation]) -> Tuple[float, float]:
        """
        贝叶斯后验估计
        
        Returns:
            Tuple[posterior_mean, posterior_std]: 后验均值和标准差
        """
        # 使用高斯权重，距离当前质量点越近的观测权重越大
        weights = []
        rewards = []
        
        for obs in observations:
            # 计算质量距离权重
            distance = abs(obs.quality - quality)
            weight = np.exp(-distance / 10.0)  # 高斯权重
            
            weights.append(weight)
            rewards.append(obs.reward)
        
        if not weights:
            return self.prior_mean, self.prior_std
        
        weights = np.array(weights)
        rewards = np.array(rewards)
        
        # 加权平均
        if weights.sum() > 0:
            posterior_mean = np.average(rewards, weights=weights)
            
            # 加权方差估计
            variance = np.average((rewards - posterior_mean) ** 2, weights=weights)
            posterior_std = np.sqrt(variance + 1e-6)  # 避免除零
        else:
            posterior_mean = self.prior_mean
            posterior_std = self.prior_std
        
        return posterior_mean, posterior_std

    def _quality_to_distance(self, quality: int) -> float:
        """
        质量参数转换为distance参数
        这是一个启发式映射，不同工具可能有不同的映射关系
        """
        if self.tool == "jxl":
            # JXL: quality 1-100 -> distance 15.0-0.1
            return 15.0 - (quality - 1) * 14.9 / 99.0
        elif self.tool == "avif":
            # AVIF: quality 1-100 -> distance 63-0
            return 63.0 - (quality - 1) * 63.0 / 99.0
        elif self.tool == "webp":
            # WebP: quality 1-100 -> distance 100-0
            return 100.0 - quality
        elif self.tool in ["aac", "opus", "mp3"]:
            # 音频：quality作为比特率，distance不适用
            return 0.0
        else:
            # 默认线性映射
            return 100.0 - quality

    def calculate_reward(self, original_size: int, converted_size: int, 
                        ssim: float, target_mode: str = "balanced") -> float:
        """
        计算奖励值
        
        Args:
            original_size: 原文件大小（字节）
            converted_size: 转换后文件大小（字节）
            ssim: 结构相似性指数 (0-1)
            target_mode: 目标模式
            
        Returns:
            float: 奖励值 (0-100)
        """
        if original_size <= 0 or converted_size <= 0:
            return 0.0
        
        # 计算压缩率
        compression_ratio = 1.0 - (converted_size / original_size)
        compression_ratio = max(0.0, min(1.0, compression_ratio))  # 限制在[0,1]
        
        # 质量分数 (SSIM归一化到0-100)
        quality_score = ssim * 100.0
        
        # 压缩分数
        compression_score = compression_ratio * 100.0
        
        # 根据目标模式调整权重
        if target_mode == "quality":
            # 质量优先：80%质量 + 20%压缩
            reward = 0.8 * quality_score + 0.2 * compression_score
        elif target_mode == "size":
            # 大小优先：20%质量 + 80%压缩
            reward = 0.2 * quality_score + 0.8 * compression_score
        else:  # balanced
            # 平衡模式：50%质量 + 50%压缩
            reward = 0.5 * quality_score + 0.5 * compression_score
        
        # 质量惩罚：SSIM < 0.9时额外惩罚
        if ssim < 0.9:
            penalty = (0.9 - ssim) * 50.0  # 最大惩罚50分
            reward -= penalty
        
        return max(0.0, min(100.0, reward))

    def get_statistics(self) -> Dict[str, any]:
        """获取优化器统计信息"""
        if not self.observations:
            return {
                'total_observations': 0,
                'avg_reward': 0.0,
                'best_reward': 0.0,
                'modes': {}
            }
        
        rewards = [obs.reward for obs in self.observations]
        
        # 按模式分组统计
        modes_stats = {}
        for obs in self.observations:
            mode = obs.target_mode
            if mode not in modes_stats:
                modes_stats[mode] = []
            modes_stats[mode].append(obs.reward)
        
        for mode, mode_rewards in modes_stats.items():
            modes_stats[mode] = {
                'count': len(mode_rewards),
                'avg_reward': np.mean(mode_rewards),
                'best_reward': np.max(mode_rewards)
            }
        
        return {
            'total_observations': len(self.observations),
            'avg_reward': np.mean(rewards),
            'best_reward': np.max(rewards),
            'worst_reward': np.min(rewards),
            'std_reward': np.std(rewards),
            'modes': modes_stats
        }

    def save_to_file(self, filepath: str) -> None:
        """保存观测数据到文件"""
        data = {
            'tool': self.tool,
            'quality_range': self.quality_range,
            'exploration_factor': self.exploration_factor,
            'observations': [obs.to_dict() for obs in self.observations]
        }
        
        with open(filepath, 'w') as f:
            json.dump(data, f, indent=2)

    def load_from_file(self, filepath: str) -> None:
        """从文件加载观测数据"""
        if not Path(filepath).exists():
            return
        
        with open(filepath, 'r') as f:
            data = json.load(f)
        
        self.tool = data.get('tool', self.tool)
        self.quality_range = tuple(data.get('quality_range', self.quality_range))
        self.exploration_factor = data.get('exploration_factor', self.exploration_factor)
        
        # 加载观测数据
        self.observations = []
        for obs_dict in data.get('observations', []):
            obs = Observation(
                quality=obs_dict['quality'],
                distance=obs_dict['distance'],
                reward=obs_dict['reward'],
                file_size=obs_dict['file_size'],
                ssim=obs_dict['ssim'],
                target_mode=obs_dict['target_mode'],
                tool=obs_dict.get('tool', self.tool),
                timestamp=obs_dict.get('timestamp', 0)
            )
            self.observations.append(obs)


class MultiToolOptimizer:
    """多工具贝叶斯优化器管理器"""
    
    def __init__(self, tools: List[str], storage_dir: str = "data/optimizer"):
        """
        初始化多工具优化器
        
        Args:
            tools: 工具列表
            storage_dir: 存储目录
        """
        self.tools = tools
        self.storage_dir = Path(storage_dir)
        self.storage_dir.mkdir(parents=True, exist_ok=True)
        
        # 初始化各工具的优化器
        self.optimizers: Dict[str, BayesianOptimizer] = {}
        for tool in tools:
            self.optimizers[tool] = BayesianOptimizer(tool)
            
            # 尝试加载历史数据
            storage_file = self.storage_dir / f"{tool}_optimizer.json"
            self.optimizers[tool].load_from_file(str(storage_file))

    def get_optimizer(self, tool: str) -> Optional[BayesianOptimizer]:
        """获取指定工具的优化器"""
        return self.optimizers.get(tool)

    def add_observation(self, tool: str, obs: Observation) -> None:
        """添加观测到指定工具"""
        if tool in self.optimizers:
            self.optimizers[tool].add_observation(obs)
            
            # 自动保存
            storage_file = self.storage_dir / f"{tool}_optimizer.json"
            self.optimizers[tool].save_to_file(str(storage_file))

    def predict_optimal_params(self, tool: str, target_mode: str = "balanced") -> Optional[Tuple[int, float]]:
        """预测最优参数"""
        if tool in self.optimizers:
            return self.optimizers[tool].predict_optimal(target_mode)
        return None

    def get_global_statistics(self) -> Dict[str, any]:
        """获取全局统计信息"""
        global_stats = {}
        
        for tool, optimizer in self.optimizers.items():
            global_stats[tool] = optimizer.get_statistics()
        
        # 总体统计
        total_observations = sum(stats['total_observations'] for stats in global_stats.values())
        if total_observations > 0:
            avg_rewards = []
            for stats in global_stats.values():
                if stats['total_observations'] > 0:
                    avg_rewards.append(stats['avg_reward'])
            
            global_stats['_global'] = {
                'total_observations': total_observations,
                'avg_reward_across_tools': np.mean(avg_rewards) if avg_rewards else 0.0,
                'active_tools': len([t for t, s in global_stats.items() if s['total_observations'] > 0])
            }
        
        return global_stats


# 便捷函数
def create_observation(quality: int, distance: float, original_size: int, 
                      converted_size: int, ssim: float, target_mode: str) -> Observation:
    """
    创建观测记录的便捷函数
    """
    optimizer = BayesianOptimizer("temp")  # 临时优化器用于计算奖励
    reward = optimizer.calculate_reward(original_size, converted_size, ssim, target_mode)
    
    return Observation(
        quality=quality,
        distance=distance,
        reward=reward,
        file_size=converted_size,
        ssim=ssim,
        target_mode=target_mode
    )


if __name__ == "__main__":
    # 测试代码
    print("=== 贝叶斯优化器测试 ===")
    
    # 创建优化器
    optimizer = BayesianOptimizer("avif", debug=True)
    
    # 添加一些模拟观测
    observations = [
        Observation(quality=85, distance=10.0, reward=75.0, file_size=50000, ssim=0.95, target_mode="balanced"),
        Observation(quality=90, distance=5.0, reward=80.0, file_size=45000, ssim=0.96, target_mode="balanced"),
        Observation(quality=80, distance=15.0, reward=70.0, file_size=55000, ssim=0.94, target_mode="balanced"),
        Observation(quality=95, distance=2.0, reward=85.0, file_size=40000, ssim=0.98, target_mode="quality"),
    ]
    
    for obs in observations:
        optimizer.add_observation(obs)
    
    # 预测最优参数
    quality, distance = optimizer.predict_optimal("balanced")
    print(f"预测最优参数（balanced）: quality={quality}, distance={distance:.2f}")
    
    quality, distance = optimizer.predict_optimal("quality")
    print(f"预测最优参数（quality）: quality={quality}, distance={distance:.2f}")
    
    # 统计信息
    stats = optimizer.get_statistics()
    print(f"\n统计信息: {stats}")
