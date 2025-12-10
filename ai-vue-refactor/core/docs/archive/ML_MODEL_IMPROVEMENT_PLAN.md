# ML模型质量改进计划

**日期**: 2025-11-19  
**状态**: 🔴 高优先级  
**预计时间**: 8-10小时

---

## 📊 当前问题分析

### 问题症状
- LightGBM Quality准确率: 24% (±5)
- PPO Quality准确率: 34% (±5)
- Effort准确率: 66% (±1) - 相对较好

### 根本原因分析 (5 Whys)

**Q1: 为什么Quality准确率这么低？**
A1: 模型预测值与真实值差距大（MAE ~10）

**Q2: 为什么预测值差距大？**
A2: 训练数据的Quality分布太稀疏（只有3个值：60/75/90）

**Q3: 为什么训练数据分布稀疏？**
A3: 使用固定的参数组合收集数据（硬编码规则）

**Q4: 为什么使用固定参数？**
A4: 误解了监督学习的目标 - 试图学习"特征+参数 → 输出质量"

**Q5: 为什么这个目标是错误的？**
A5: 这需要大量真实转换数据，而我们实际需要的是"特征 → 最佳参数"（推荐系统）

---

## 🎯 解决方案

### 方案A: 强化学习 (推荐) ⭐

**核心思想**: 
- 不需要预先标注的"最佳参数"
- 通过实际转换结果学习
- 在线学习：每次转换都改进模型

**实施步骤**:

#### 1. 奖励函数设计 (2小时)
```python
def calculate_reward(original_size, converted_size, ssim_score, conversion_time):
    """
    奖励函数：平衡文件大小、质量和速度
    """
    # 压缩率奖励 (0-1)
    compression_reward = 1.0 - (converted_size / original_size)
    
    # 质量奖励 (0-1)
    quality_reward = ssim_score  # SSIM已经是0-1
    
    # 速度惩罚 (0-1)
    speed_penalty = min(conversion_time / 10.0, 1.0)  # 10秒以上全惩罚
    
    # 加权组合
    reward = (
        0.4 * compression_reward +  # 40% 压缩率
        0.5 * quality_reward +       # 50% 质量
        0.1 * (1.0 - speed_penalty)  # 10% 速度
    )
    
    return reward
```

#### 2. PPO训练循环优化 (3小时)
- 集成到`src/online_learning.rs`
- 每次转换后计算奖励
- 累积经验到replay buffer
- 定期更新策略网络

#### 3. 在线学习机制 (2小时)
```rust
// src/online_learning.rs
pub fn record_conversion_experience(
    features: &[f32; 128],
    params: &ConversionParams,
    result: &ConversionResult,
) -> Result<()> {
    // 1. 计算奖励
    let reward = calculate_reward(result);
    
    // 2. 存储经验
    let experience = Experience {
        state: features.clone(),
        action: params_to_action(params),
        reward,
        next_state: None,  // 单步任务
    };
    
    // 3. 添加到replay buffer
    REPLAY_BUFFER.lock().unwrap().push(experience);
    
    // 4. 如果buffer足够大，触发训练
    if REPLAY_BUFFER.lock().unwrap().len() >= 32 {
        trigger_ppo_training()?;
    }
    
    Ok(())
}
```

#### 4. 模型持久化 (1小时)
- 自动保存改进的模型
- 版本管理
- 回滚机制

**优势**:
- ✅ 不需要大量预标注数据
- ✅ 持续改进（用户越用越准）
- ✅ 适应不同场景
- ✅ 真实反馈驱动

**挑战**:
- ⚠️ 需要SSIM计算（已有`scripts/calculate_ssim.py`）
- ⚠️ 冷启动问题（初期可能不准）
- ⚠️ 需要用户同意在线学习

---

### 方案B: 改进训练数据 (备选)

**核心思想**: 
- 收集更多样化的训练数据
- 使用真实用户选择的参数

**实施步骤**:

#### 1. 数据收集策略 (3小时)
```python
# 生成更细粒度的参数组合
qualities = range(60, 96, 5)  # 60, 65, 70, ..., 95
efforts = range(1, 11)         # 1-10

# 对每个测试图像，尝试所有组合
for quality in qualities:
    for effort in efforts:
        convert_and_measure(image, quality, effort)
```

#### 2. 真实转换数据收集 (2小时)
- 记录用户实际使用的参数
- 收集转换结果（大小、SSIM）
- 构建"特征 → 参数 → 结果"数据集

#### 3. 重新训练模型 (2小时)
- 使用新数据集
- 调整模型超参数
- 交叉验证

**优势**:
- ✅ 传统方法，风险低
- ✅ 可控性强

**劣势**:
- ❌ 需要大量计算资源（数千次转换）
- ❌ 数据收集耗时
- ❌ 静态模型，不会持续改进

---

## 📋 推荐实施计划

### Phase 1: 奖励函数和在线学习 (1周)
1. ✅ 实现奖励函数计算
2. ✅ 集成SSIM质量评估
3. ✅ 实现经验记录机制
4. ✅ 测试在线学习流程

### Phase 2: PPO训练优化 (1周)
1. ✅ 优化PPO超参数
2. ✅ 实现replay buffer
3. ✅ 异步训练机制
4. ✅ 模型版本管理

### Phase 3: 用户体验优化 (3天)
1. ✅ 在线学习开关（默认关闭）
2. ✅ 进度反馈
3. ✅ 隐私保护
4. ✅ 模型性能监控

---

## 🎯 成功指标

### 短期目标 (1个月)
- Quality准确率: 24% → 50%
- Effort准确率: 66% → 80%
- 用户满意度: 收集反馈

### 长期目标 (3个月)
- Quality准确率: 50% → 70%
- Effort准确率: 80% → 90%
- 在线学习用户: 100+

---

## 🚨 风险和缓解

### 风险1: 冷启动问题
**缓解**: 保留当前LightGBM作为fallback

### 风险2: 训练不稳定
**缓解**: 实现模型回滚机制

### 风险3: 用户隐私
**缓解**: 
- 默认关闭在线学习
- 明确告知用户
- 只记录特征和参数，不记录文件内容

---

## 📝 下一步行动

1. **立即**: 实现奖励函数 (`src/reward_calculator.rs`)
2. **本周**: 集成在线学习到CLI (`--online-learning`)
3. **下周**: 优化PPO训练循环
4. **持续**: 监控模型性能，收集用户反馈

---

**遵循质量宣言**:
- ✅ 深度调查原则 - 5 Whys分析
- ✅ 真实性原则 - 基于实际转换结果
- ✅ 批判性思维 - 质疑简单归因
- ✅ 不接受表面解决 - 从根本改进

