#!/usr/bin/env python3
"""用户反馈系统"""
import sys
import json
import numpy as np
from dataclasses import dataclass
from typing import List, Dict, Optional
import time
from enum import Enum

class FeedbackType(Enum):
    QUALITY_RATING = "quality_rating"  # 质量评分 1-5
    SPEED_RATING = "speed_rating"      # 速度评分 1-5
    SIZE_RATING = "size_rating"        # 文件大小评分 1-5
    OVERALL_RATING = "overall_rating"  # 总体评分 1-5
    BINARY_LIKE = "binary_like"        # 喜欢/不喜欢
    COMPARISON = "comparison"           # A/B对比

@dataclass
class UserFeedback:
    """用户反馈"""
    user_id: str
    session_id: str
    feedback_type: FeedbackType
    rating: float  # 1-5 或 0-1
    conversion_id: str
    input_features: List[float]
    predicted_quality: int
    predicted_effort: int
    actual_output_size: int
    processing_time: float
    timestamp: float
    comment: Optional[str] = None

class FeedbackCollector:
    """反馈收集器"""
    
    def __init__(self, storage_path: str = "models/user_feedback.json"):
        self.storage_path = storage_path
        self.feedback_buffer = []
        self.load_existing_feedback()
    
    def load_existing_feedback(self):
        """加载已有反馈"""
        try:
            with open(self.storage_path, 'r') as f:
                data = json.load(f)
            
            self.feedback_buffer = []
            for item in data:
                feedback = UserFeedback(
                    user_id=item['user_id'],
                    session_id=item['session_id'],
                    feedback_type=FeedbackType(item['feedback_type']),
                    rating=item['rating'],
                    conversion_id=item['conversion_id'],
                    input_features=item['input_features'],
                    predicted_quality=item['predicted_quality'],
                    predicted_effort=item['predicted_effort'],
                    actual_output_size=item['actual_output_size'],
                    processing_time=item['processing_time'],
                    timestamp=item['timestamp'],
                    comment=item.get('comment')
                )
                self.feedback_buffer.append(feedback)
            
            print(f"✅ 加载 {len(self.feedback_buffer)} 个已有反馈")
            
        except FileNotFoundError:
            print(f"📝 创建新的反馈文件: {self.storage_path}")
        except Exception as e:
            print(f"⚠️  加载反馈失败: {e}")
    
    def collect_feedback(self, feedback: UserFeedback):
        """收集反馈"""
        self.feedback_buffer.append(feedback)
        print(f"📝 收集反馈: {feedback.feedback_type.value} = {feedback.rating}")
    
    def save_feedback(self):
        """保存反馈"""
        data = []
        for feedback in self.feedback_buffer:
            data.append({
                'user_id': feedback.user_id,
                'session_id': feedback.session_id,
                'feedback_type': feedback.feedback_type.value,
                'rating': feedback.rating,
                'conversion_id': feedback.conversion_id,
                'input_features': feedback.input_features,
                'predicted_quality': feedback.predicted_quality,
                'predicted_effort': feedback.predicted_effort,
                'actual_output_size': feedback.actual_output_size,
                'processing_time': feedback.processing_time,
                'timestamp': feedback.timestamp,
                'comment': feedback.comment
            })
        
        with open(self.storage_path, 'w') as f:
            json.dump(data, f, indent=2)
        
        print(f"💾 保存 {len(data)} 个反馈到 {self.storage_path}")
    
    def analyze_feedback(self) -> Dict:
        """分析反馈"""
        if not self.feedback_buffer:
            return {}
        
        print(f"\n📊 分析用户反馈...")
        
        # 按类型分组
        by_type = {}
        for feedback in self.feedback_buffer:
            ftype = feedback.feedback_type.value
            if ftype not in by_type:
                by_type[ftype] = []
            by_type[ftype].append(feedback.rating)
        
        # 统计
        stats = {}
        for ftype, ratings in by_type.items():
            stats[ftype] = {
                'count': len(ratings),
                'mean': float(np.mean(ratings)),
                'std': float(np.std(ratings)),
                'min': float(np.min(ratings)),
                'max': float(np.max(ratings))
            }
            
            print(f"  {ftype}:")
            print(f"    数量: {stats[ftype]['count']}")
            print(f"    平均: {stats[ftype]['mean']:.2f}")
            print(f"    标准差: {stats[ftype]['std']:.2f}")
        
        return stats
    
    def get_training_signals(self) -> List[Dict]:
        """从反馈中提取训练信号"""
        print(f"\n🎯 提取训练信号...")
        
        signals = []
        for feedback in self.feedback_buffer:
            # 将反馈转换为训练信号
            signal = {
                'features': feedback.input_features,
                'predicted_quality': feedback.predicted_quality,
                'predicted_effort': feedback.predicted_effort,
                'user_rating': feedback.rating,
                'feedback_type': feedback.feedback_type.value,
                'timestamp': feedback.timestamp
            }
            
            # 根据反馈调整目标值
            if feedback.feedback_type == FeedbackType.QUALITY_RATING:
                # 质量评分高 -> 可以降低quality参数
                # 质量评分低 -> 需要提高quality参数
                if feedback.rating >= 4.0:
                    signal['adjusted_quality'] = max(50, feedback.predicted_quality - 5)
                elif feedback.rating <= 2.0:
                    signal['adjusted_quality'] = min(100, feedback.predicted_quality + 10)
                else:
                    signal['adjusted_quality'] = feedback.predicted_quality
            
            signals.append(signal)
        
        print(f"  ✅ 提取 {len(signals)} 个训练信号")
        return signals

def simulate_user_feedback():
    """模拟用户反馈"""
    print(f"\n🎮 模拟用户反馈...")
    
    collector = FeedbackCollector()
    
    # 模拟10个用户反馈
    for i in range(10):
        feedback = UserFeedback(
            user_id=f"user_{i % 3}",  # 3个用户
            session_id=f"session_{int(time.time())}_{i}",
            feedback_type=FeedbackType.QUALITY_RATING,
            rating=np.random.uniform(3.0, 5.0),  # 3-5分
            conversion_id=f"conv_{i}",
            input_features=[np.random.random() for _ in range(128)],
            predicted_quality=np.random.randint(70, 95),
            predicted_effort=np.random.randint(4, 8),
            actual_output_size=np.random.randint(100000, 1000000),
            processing_time=np.random.uniform(0.5, 3.0),
            timestamp=time.time(),
            comment=f"Test feedback {i}"
        )
        
        collector.collect_feedback(feedback)
    
    # 保存反馈
    collector.save_feedback()
    
    # 分析反馈
    stats = collector.analyze_feedback()
    
    # 提取训练信号
    signals = collector.get_training_signals()
    
    return {
        'stats': stats,
        'signals': signals[:5]  # 只返回前5个示例
    }

def main():
    print("📝 用户反馈系统")
    
    # 模拟用户反馈
    results = simulate_user_feedback()
    
    # 保存结果
    with open('models/feedback_analysis.json', 'w') as f:
        json.dump(results, f, indent=2)
    
    print(f"\n💾 分析结果已保存: models/feedback_analysis.json")
    print(f"\n✅ 用户反馈系统测试完成")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
