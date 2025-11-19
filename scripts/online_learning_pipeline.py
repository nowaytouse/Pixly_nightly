#!/usr/bin/env python3
"""在线学习完整流程"""
import sys
import json
import numpy as np
from pathlib import Path
import time

# 导入各个模块
import importlib.util

def load_module(name, path):
    """动态加载模块"""
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

def main():
    print("🚀 在线学习完整流程")
    print("=" * 60)
    
    # Step 1: 优先级经验回放
    print("\n📍 Step 1: 优先级经验回放")
    print("-" * 60)
    per_module = load_module("per", "scripts/priority_experience_replay.py")
    
    experiences = per_module.load_experiences()
    if experiences:
        experiences = per_module.calculate_priorities(experiences)
        buffer = per_module.PriorityBuffer(max_size=1000)
        for exp in experiences:
            buffer.add(exp)
        print(f"✅ 优先级缓冲区: {buffer.size()} 个经验")
    else:
        print("⚠️  跳过 - 没有经验数据")
    
    # Step 2: 增量学习
    print("\n📍 Step 2: 增量学习")
    print("-" * 60)
    inc_module = load_module("inc", "scripts/incremental_learning.py")
    
    if Path("models/lightgbm_all_quality_v2.txt").exists():
        quality_learner = inc_module.IncrementalLearner(
            'models/lightgbm_all_quality_v2.txt',
            'models/feature_scaler.pkl'
        )
        
        effort_learner = inc_module.IncrementalLearner(
            'models/lightgbm_all_effort_v2.txt',
            'models/feature_scaler.pkl'
        )
        
        # 加载新经验
        new_experiences = inc_module.load_new_experiences()
        
        if new_experiences and len(new_experiences) >= 4:
            # 分割训练/测试
            split_idx = len(new_experiences) // 2
            train_exp = new_experiences[:split_idx]
            test_exp = new_experiences[split_idx:]
            
            print(f"训练经验: {len(train_exp)}, 测试经验: {len(test_exp)}")
            
            # 评估更新前
            quality_before = quality_learner.evaluate_drift(test_exp)
            effort_before = effort_learner.evaluate_drift(test_exp)
            
            # 增量更新
            quality_learner.incremental_update(train_exp, learning_rate=0.005)
            effort_learner.incremental_update(train_exp, learning_rate=0.005)
            
            # 评估更新后
            quality_after = quality_learner.evaluate_drift(test_exp)
            effort_after = effort_learner.evaluate_drift(test_exp)
            
            print(f"✅ 增量学习完成")
            print(f"   Quality: {quality_before.get('mae', 0):.3f} → {quality_after.get('mae', 0):.3f}")
            print(f"   Effort: {effort_before.get('mae', 0):.3f} → {effort_after.get('mae', 0):.3f}")
        else:
            print("⚠️  跳过 - 经验数据不足")
    else:
        print("⚠️  跳过 - 模型文件不存在")
    
    # Step 3: 用户反馈
    print("\n📍 Step 3: 用户反馈系统")
    print("-" * 60)
    feedback_module = load_module("feedback", "scripts/user_feedback_system.py")
    
    collector = feedback_module.FeedbackCollector()
    
    # 模拟一些反馈
    for i in range(5):
        feedback = feedback_module.UserFeedback(
            user_id=f"user_{i % 2}",
            session_id=f"session_{int(time.time())}_{i}",
            feedback_type=feedback_module.FeedbackType.QUALITY_RATING,
            rating=np.random.uniform(3.5, 5.0),
            conversion_id=f"conv_{i}",
            input_features=[np.random.random() for _ in range(128)],
            predicted_quality=np.random.randint(75, 90),
            predicted_effort=np.random.randint(5, 8),
            actual_output_size=np.random.randint(200000, 800000),
            processing_time=np.random.uniform(1.0, 2.5),
            timestamp=time.time()
        )
        collector.collect_feedback(feedback)
    
    stats = collector.analyze_feedback()
    signals = collector.get_training_signals()
    
    print(f"✅ 收集 {len(collector.feedback_buffer)} 个反馈")
    print(f"   平均评分: {stats.get('quality_rating', {}).get('mean', 0):.2f}")
    
    # Step 4: 生成报告
    print("\n📍 Step 4: 生成报告")
    print("-" * 60)
    
    report = {
        'timestamp': time.strftime('%Y-%m-%d %H:%M:%S'),
        'priority_buffer_size': buffer.size() if experiences else 0,
        'incremental_updates': {
            'quality_learner_updates': quality_learner.update_count if 'quality_learner' in locals() else 0,
            'effort_learner_updates': effort_learner.update_count if 'effort_learner' in locals() else 0,
        },
        'user_feedback': {
            'total_feedback': len(collector.feedback_buffer),
            'stats': stats,
            'training_signals': len(signals)
        },
        'performance': {
            'quality_mae_before': quality_before.get('mae', 0) if 'quality_before' in locals() else 0,
            'quality_mae_after': quality_after.get('mae', 0) if 'quality_after' in locals() else 0,
            'effort_mae_before': effort_before.get('mae', 0) if 'effort_before' in locals() else 0,
            'effort_mae_after': effort_after.get('mae', 0) if 'effort_after' in locals() else 0,
        }
    }
    
    # 保存报告
    report_path = f"models/online_learning_report_{time.strftime('%Y%m%d_%H%M%S')}.json"
    with open(report_path, 'w') as f:
        json.dump(report, f, indent=2)
    
    print(f"💾 报告已保存: {report_path}")
    
    # 总结
    print("\n" + "=" * 60)
    print("📊 在线学习流程总结")
    print("=" * 60)
    print(f"✅ 优先级经验回放: {report['priority_buffer_size']} 个经验")
    print(f"✅ 增量学习更新: Quality={report['incremental_updates']['quality_learner_updates']}, Effort={report['incremental_updates']['effort_learner_updates']}")
    print(f"✅ 用户反馈: {report['user_feedback']['total_feedback']} 个反馈")
    print(f"✅ 训练信号: {report['user_feedback']['training_signals']} 个")
    
    if report['performance']['quality_mae_before'] > 0:
        quality_improvement = report['performance']['quality_mae_before'] - report['performance']['quality_mae_after']
        effort_improvement = report['performance']['effort_mae_before'] - report['performance']['effort_mae_after']
        print(f"\n📈 性能提升:")
        print(f"   Quality MAE: {quality_improvement:+.3f}")
        print(f"   Effort MAE: {effort_improvement:+.3f}")
    
    print("\n✅ Phase 5: 在线学习增强 - 完成!")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
