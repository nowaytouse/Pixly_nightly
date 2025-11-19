# Phase 3: 模型质量改进 - 正确方法论

**日期**: 2025-11-18  
**状态**: 规划中  
**预计时间**: 8-10小时

---

## 🔍 问题深度分析

### 当前状态（2025-11-18 20:00验证）

**✅ 系统已工作**:
```bash
./target/release/pixly-converter analyze "image.jpeg" --ai
# 输出: AVIF推荐，confidence 75%
# 预估大小: 37.1 KB (减少69.3%)
# 参数: quality=80, effort=6
```

**⚠️ 模型质量问题**:
- LightGBM模型R²分数: -0.02 ~ -0.8（负数）
- 说明：模型预测不如简单平均值
- 但系统仍能给出合理推荐（基于规则+模型混合）

### 根本原因（5 Whys分析）

1. **Why R²为负？**
   - 模型预测的方差比目标变量的方差还大
   - 模型在"瞎猜"

2. **Why模型瞎猜？**
   - 训练数据中quality/effort只有几个离散值
   - 目标变量方差极小（几乎是常数）

3. **Why目标变量是常数？**
   - 我们用固定参数组合收集数据
   - quality: [60, 70, 80, 90]
   - effort: [4, 6, 8]

4. **Why用固定参数？**
   - 误解了监督学习的目标
   - 以为是学习"特征+参数 → 输出质量"

5. **Why误解？**
   - 没有明确区分：
     - 回归问题：预测连续值（如文件大小）
     - 推荐问题：选择最佳参数组合

---

## 🎯 正确的问题定义

### 我们真正要解决的问题

**不是**：给定特征和参数，预测输出质量（回归）
**而是**：给定特征，推荐最佳参数（推荐/优化）

### 三种正确的方法

#### 方法1: 强化学习（推荐⭐⭐⭐⭐⭐）

**原理**:
- Agent: 参数选择策略
- State: 图像特征（128维）
- Action: 参数组合（quality, effort）
- Reward: 文件大小减少 + SSIM质量保持

**优势**:
- ✅ 不需要大量预先标注的数据
- ✅ 在线学习：每次转换后改进
- ✅ 自动探索最优参数空间
- ✅ 适合参数优化问题

**实施**:
```python
# PPO (Proximal Policy Optimization)
class ParameterPolicy(nn.Module):
    def __init__(self):
        self.actor = nn.Sequential(
            nn.Linear(128, 256),  # 输入：特征
            nn.ReLU(),
            nn.Linear(256, 128),
            nn.ReLU(),
            nn.Linear(128, 2)     # 输出：quality, effort
        )
    
    def forward(self, features):
        return self.actor(features)

# 奖励函数
def calculate_reward(original_size, output_size, ssim):
    compression_reward = (original_size - output_size) / original_size
    quality_penalty = max(0, 0.95 - ssim) * 2  # SSIM<0.95惩罚
    return compression_reward - quality_penalty

# 训练循环
for episode in range(1000):
    features = extract_features(image)
    action = policy(features)  # 预测参数
    quality, effort = action_to_params(action)
    
    # 执行转换
    result = convert(image, quality, effort)
    
    # 计算奖励
    reward = calculate_reward(
        original_size, result.size, result.ssim
    )
    
    # 更新策略
    policy.update(features, action, reward)
```

**预计时间**: 6-8小时

---

#### 方法2: 贝叶斯优化（推荐⭐⭐⭐⭐）

**原理**:
- 将参数选择视为黑盒优化问题
- 使用高斯过程建模目标函数
- 平衡探索（exploration）和利用（exploitation）

**优势**:
- ✅ 样本效率高（少量转换即可学习）
- ✅ 自动处理参数空间
- ✅ 提供不确定性估计

**实施**:
```python
from skopt import gp_minimize
from skopt.space import Integer

def objective(params, features):
    """目标函数：最小化（负奖励）"""
    quality, effort = params
    result = convert(image, quality, effort)
    reward = calculate_reward(...)
    return -reward  # 最小化负奖励 = 最大化奖励

# 对每个图像特征聚类
for cluster_id, images in feature_clusters.items():
    # 优化该聚类的最佳参数
    space = [
        Integer(60, 95, name='quality'),
        Integer(4, 9, name='effort')
    ]
    
    result = gp_minimize(
        lambda params: objective(params, cluster_features),
        space,
        n_calls=50,  # 只需50次转换
        random_state=42
    )
    
    # 保存该聚类的最佳参数
    best_params[cluster_id] = result.x
```

**预计时间**: 4-6小时

---

#### 方法3: 协同过滤（推荐⭐⭐⭐）

**原理**:
- 收集用户实际使用的参数选择
- 学习"相似图像 → 相似参数"
- 类似推荐系统

**优势**:
- ✅ 利用真实用户行为
- ✅ 简单直观

**劣势**:
- ❌ 需要大量用户数据
- ❌ 冷启动问题

**实施**:
```python
# 收集用户行为
user_choices = []
for conversion in user_history:
    user_choices.append({
        'features': extract_features(conversion.input),
        'quality': conversion.quality,
        'effort': conversion.effort,
        'satisfaction': conversion.user_rating
    })

# 训练协同过滤模型
from sklearn.neighbors import NearestNeighbors

# 找到相似图像
nn = NearestNeighbors(n_neighbors=10)
nn.fit([c['features'] for c in user_choices])

# 推荐参数
def recommend_params(features):
    neighbors = nn.kneighbors([features])[1][0]
    similar_choices = [user_choices[i] for i in neighbors]
    
    # 加权平均（按满意度）
    weights = [c['satisfaction'] for c in similar_choices]
    quality = np.average([c['quality'] for c in similar_choices], weights=weights)
    effort = np.average([c['effort'] for c in similar_choices], weights=weights)
    
    return quality, effort
```

**预计时间**: 3-4小时

---

## 📋 推荐实施方案

### Phase 3.1: PPO强化学习（优先）

**为什么选PPO**:
1. 不需要大量预先标注数据
2. 在线学习，持续改进
3. 自动探索参数空间
4. 已有PPO框架代码（`scripts/train_ppo.py`）

**实施步骤**:

#### Step 1: 实现奖励函数（1小时）
```rust
// src/reward_calculator.rs
pub struct RewardCalculator;

impl RewardCalculator {
    pub fn calculate(
        original_size: u64,
        output_size: u64,
        ssim: f64,
        processing_time: f64
    ) -> f64 {
        // 压缩奖励（0-1）
        let compression_reward = 
            (original_size - output_size) as f64 / original_size as f64;
        
        // 质量惩罚（SSIM < 0.95）
        let quality_penalty = if ssim < 0.95 {
            (0.95 - ssim) * 2.0
        } else {
            0.0
        };
        
        // 速度惩罚（>5秒）
        let speed_penalty = if processing_time > 5.0 {
            (processing_time - 5.0) / 10.0
        } else {
            0.0
        };
        
        compression_reward - quality_penalty - speed_penalty
    }
}
```

#### Step 2: 集成PPO训练（3小时）
```python
# scripts/train_ppo_v2.py
import torch
import torch.nn as nn
from torch.distributions import Normal

class ActorNetwork(nn.Module):
    """策略网络：特征 → 参数"""
    def __init__(self):
        super().__init__()
        self.fc = nn.Sequential(
            nn.Linear(128, 256),
            nn.ReLU(),
            nn.Dropout(0.2),
            nn.Linear(256, 128),
            nn.ReLU(),
            nn.Linear(128, 2)  # quality, effort
        )
    
    def forward(self, features):
        output = self.fc(features)
        # quality: 60-95, effort: 4-9
        quality = torch.sigmoid(output[:, 0]) * 35 + 60
        effort = torch.sigmoid(output[:, 1]) * 5 + 4
        return quality, effort

class CriticNetwork(nn.Module):
    """价值网络：特征 → 预期奖励"""
    def __init__(self):
        super().__init__()
        self.fc = nn.Sequential(
            nn.Linear(128, 256),
            nn.ReLU(),
            nn.Linear(256, 128),
            nn.ReLU(),
            nn.Linear(128, 1)
        )
    
    def forward(self, features):
        return self.fc(features)

def train_ppo(data_dir, epochs=100):
    actor = ActorNetwork()
    critic = CriticNetwork()
    
    for epoch in range(epochs):
        # 收集轨迹
        for image_file in scan_images(data_dir):
            features = extract_features(image_file)
            
            # 策略预测参数
            quality, effort = actor(features)
            
            # 执行转换
            result = convert(image_file, quality, effort)
            
            # 计算奖励
            reward = calculate_reward(result)
            
            # 更新策略
            update_policy(actor, critic, features, reward)
        
        if epoch % 10 == 0:
            print(f"Epoch {epoch}: avg_reward={avg_reward:.4f}")
```

#### Step 3: 在线学习集成（2小时）
```rust
// src/online_learning.rs
pub struct OnlineLearner {
    ppo_model: PPOModel,
    experience_buffer: Vec<Experience>,
}

impl OnlineLearner {
    pub fn record_conversion(&mut self, 
        features: Vec<f64>,
        quality: u32,
        effort: u32,
        result: ConversionResult
    ) {
        let reward = RewardCalculator::calculate(
            result.original_size,
            result.output_size,
            result.ssim,
            result.processing_time
        );
        
        self.experience_buffer.push(Experience {
            features,
            action: (quality, effort),
            reward
        });
        
        // 每100次转换更新一次模型
        if self.experience_buffer.len() >= 100 {
            self.update_model();
        }
    }
}
```

#### Step 4: 测试和验证（2小时）
- 在测试集上评估
- 对比原有模型
- A/B测试

**总计**: 8小时

---

## 📊 成功标准

### 定量指标

1. **平均奖励提升**
   - 基线：当前模型平均奖励
   - 目标：提升20%以上

2. **参数准确度**
   - 在测试集上，推荐参数的平均奖励
   - 目标：超过人工选择的参数

3. **在线学习效果**
   - 100次转换后，奖励提升10%
   - 1000次转换后，奖励提升30%

### 定性指标

1. **用户满意度**
   - 推荐参数的接受率
   - 用户手动调整频率

2. **系统稳定性**
   - 不会推荐极端参数
   - 奖励函数合理性

---

## 🚫 避免的陷阱

### 陷阱1: 重复之前的错误
❌ 不要再用固定参数收集"训练数据"
✅ 使用强化学习在线学习

### 陷阱2: 过度复杂化
❌ 不要一开始就实现完整的PPO
✅ 先实现简单版本，验证可行性

### 陷阱3: 忽视现有系统
❌ 不要推翻现有工作
✅ 在现有基础上增量改进

### 陷阱4: 缺少验证
❌ 不要训练完就认为成功
✅ 在真实数据上A/B测试

---

## 📝 实施检查清单

- [ ] 理解问题本质（推荐 vs 回归）
- [ ] 选择正确方法（PPO强化学习）
- [ ] 实现奖励函数
- [ ] 集成PPO训练
- [ ] 在线学习机制
- [ ] 测试和验证
- [ ] A/B对比
- [ ] 文档更新

---

**遵循原则**: `PROJECT_QUALITY_MANIFESTO.md`
- 批判性思维：深度分析问题
- 真实性原则：不自欺欺人
- 质疑一切：不接受表面解释

**最后更新**: 2025-11-18  
**状态**: 规划完成，待实施
