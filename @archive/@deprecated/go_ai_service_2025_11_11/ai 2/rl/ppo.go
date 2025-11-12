package rl

import (
	"fmt"
	"math"
	"math/rand"
)

// PPOConfig PPO配置
type PPOConfig struct {
	Gamma        float64 // 折扣因子
	Epsilon      float64 // PPO clip参数
	LearningRate float64 // 学习率
	BatchSize    int     // 批量大小
	NEpochs      int     // 训练轮数
	Lambda       float64 // GAE lambda参数
}

// DefaultPPOConfig 默认PPO配置
func DefaultPPOConfig() *PPOConfig {
	return &PPOConfig{
		Gamma:        0.99,
		Epsilon:      0.2,
		LearningRate: 0.0003,
		BatchSize:    64,
		NEpochs:      10,
		Lambda:       0.95,
	}
}

// State 状态表示
type State struct {
	Features []float64 // 特征向量（SWT特征等）
}

// Action 动作表示（参数调整）
type Action struct {
	QualityDelta   float64 // 质量参数调整
	DistanceDelta  float64 // distance参数调整
	EffortDelta    float64 // effort参数调整（JXL）
	QuantizerDelta float64 // quantizer参数调整（AVIF）
}

// Experience 经验
type Experience struct {
	State      *State
	Action     *Action
	Reward     float64
	NextState  *State
	Done       bool
	Value      float64 // 状态价值
	LogProb    float64 // 动作对数概率
	Advantage  float64 // 优势函数
}

// ReplayBuffer 经验回放缓冲区
type ReplayBuffer struct {
	experiences []*Experience
	maxSize     int
}

// NewReplayBuffer 创建经验回放缓冲区
func NewReplayBuffer(maxSize int) *ReplayBuffer {
	return &ReplayBuffer{
		experiences: make([]*Experience, 0, maxSize),
		maxSize:     maxSize,
	}
}

// Add 添加经验
func (rb *ReplayBuffer) Add(exp *Experience) {
	if len(rb.experiences) >= rb.maxSize {
		rb.experiences = rb.experiences[1:]
	}
	rb.experiences = append(rb.experiences, exp)
}

// Sample 采样批量经验
func (rb *ReplayBuffer) Sample(batchSize int) []*Experience {
	if len(rb.experiences) <= batchSize {
		return rb.experiences
	}
	
	indices := rand.Perm(len(rb.experiences))[:batchSize]
	batch := make([]*Experience, batchSize)
	for i, idx := range indices {
		batch[i] = rb.experiences[idx]
	}
	return batch
}

// Size 获取缓冲区大小
func (rb *ReplayBuffer) Size() int {
	return len(rb.experiences)
}

// Clear 清空缓冲区
func (rb *ReplayBuffer) Clear() {
	rb.experiences = rb.experiences[:0]
}

// PolicyNetwork 策略网络
type PolicyNetwork struct {
	inputSize  int
	hiddenSize int
	outputSize int
	
	// 网络参数（简化版：单层MLP）
	w1 [][]float64 // 输入层 -> 隐藏层
	b1 []float64   // 隐藏层偏置
	w2 [][]float64 // 隐藏层 -> 输出层
	b2 []float64   // 输出层偏置
}

// NewPolicyNetwork 创建策略网络
func NewPolicyNetwork(inputSize, hiddenSize, outputSize int) *PolicyNetwork {
	pn := &PolicyNetwork{
		inputSize:  inputSize,
		hiddenSize: hiddenSize,
		outputSize: outputSize,
		w1:         make([][]float64, inputSize),
		b1:         make([]float64, hiddenSize),
		w2:         make([][]float64, hiddenSize),
		b2:         make([]float64, outputSize),
	}
	
	// Xavier初始化
	scale1 := math.Sqrt(2.0 / float64(inputSize+hiddenSize))
	for i := 0; i < inputSize; i++ {
		pn.w1[i] = make([]float64, hiddenSize)
		for j := 0; j < hiddenSize; j++ {
			pn.w1[i][j] = (rand.Float64()*2 - 1) * scale1
		}
	}
	
	scale2 := math.Sqrt(2.0 / float64(hiddenSize+outputSize))
	for i := 0; i < hiddenSize; i++ {
		pn.w2[i] = make([]float64, outputSize)
		for j := 0; j < outputSize; j++ {
			pn.w2[i][j] = (rand.Float64()*2 - 1) * scale2
		}
	}
	
	return pn
}

// Forward 前向传播
func (pn *PolicyNetwork) Forward(state *State) []float64 {
	// 隐藏层
	hidden := make([]float64, pn.hiddenSize)
	for j := 0; j < pn.hiddenSize; j++ {
		sum := pn.b1[j]
		for i := 0; i < pn.inputSize && i < len(state.Features); i++ {
			sum += state.Features[i] * pn.w1[i][j]
		}
		hidden[j] = relu(sum)
	}
	
	// 输出层
	output := make([]float64, pn.outputSize)
	for j := 0; j < pn.outputSize; j++ {
		sum := pn.b2[j]
		for i := 0; i < pn.hiddenSize; i++ {
			sum += hidden[i] * pn.w2[i][j]
		}
		output[j] = math.Tanh(sum) // 输出范围 [-1, 1]
	}
	
	return output
}

// Sample 从策略中采样动作
func (pn *PolicyNetwork) Sample(state *State) (*Action, float64) {
	output := pn.Forward(state)
	
	// 添加探索噪声
	noise := 0.1
	for i := range output {
		output[i] += (rand.Float64()*2 - 1) * noise
		output[i] = math.Max(-1, math.Min(1, output[i]))
	}
	
	action := &Action{
		QualityDelta:   output[0] * 10,  // 质量调整范围 [-10, 10]
		DistanceDelta:  output[1] * 0.5, // distance调整范围 [-0.5, 0.5]
		EffortDelta:    output[2] * 2,   // effort调整范围 [-2, 2]
		QuantizerDelta: output[3] * 10,  // quantizer调整范围 [-10, 10]
	}
	
	// 计算对数概率（简化版：高斯分布）
	logProb := 0.0
	for _, v := range output {
		logProb += -0.5 * v * v // log N(0, 1)
	}
	
	return action, logProb
}

// ValueNetwork 价值网络
type ValueNetwork struct {
	inputSize  int
	hiddenSize int
	
	// 网络参数
	w1 [][]float64
	b1 []float64
	w2 []float64
	b2 float64
}

// NewValueNetwork 创建价值网络
func NewValueNetwork(inputSize, hiddenSize int) *ValueNetwork {
	vn := &ValueNetwork{
		inputSize:  inputSize,
		hiddenSize: hiddenSize,
		w1:         make([][]float64, inputSize),
		b1:         make([]float64, hiddenSize),
		w2:         make([]float64, hiddenSize),
		b2:         0,
	}
	
	// Xavier初始化
	scale1 := math.Sqrt(2.0 / float64(inputSize+hiddenSize))
	for i := 0; i < inputSize; i++ {
		vn.w1[i] = make([]float64, hiddenSize)
		for j := 0; j < hiddenSize; j++ {
			vn.w1[i][j] = (rand.Float64()*2 - 1) * scale1
		}
	}
	
	scale2 := math.Sqrt(2.0 / float64(hiddenSize+1))
	for i := 0; i < hiddenSize; i++ {
		vn.w2[i] = (rand.Float64()*2 - 1) * scale2
	}
	
	return vn
}

// Forward 前向传播
func (vn *ValueNetwork) Forward(state *State) float64 {
	// 隐藏层
	hidden := make([]float64, vn.hiddenSize)
	for j := 0; j < vn.hiddenSize; j++ {
		sum := vn.b1[j]
		for i := 0; i < vn.inputSize && i < len(state.Features); i++ {
			sum += state.Features[i] * vn.w1[i][j]
		}
		hidden[j] = relu(sum)
	}
	
	// 输出层（标量）
	value := vn.b2
	for i := 0; i < vn.hiddenSize; i++ {
		value += hidden[i] * vn.w2[i]
	}
	
	return value
}

// PPOOptimizer PPO优化器
type PPOOptimizer struct {
	config       *PPOConfig
	policy       *PolicyNetwork
	valueNet     *ValueNetwork
	replayBuffer *ReplayBuffer
}

// NewPPOOptimizer 创建PPO优化器
func NewPPOOptimizer(config *PPOConfig, stateSize, actionSize int) *PPOOptimizer {
	if config == nil {
		config = DefaultPPOConfig()
	}
	
	return &PPOOptimizer{
		config:       config,
		policy:       NewPolicyNetwork(stateSize, 128, actionSize),
		valueNet:     NewValueNetwork(stateSize, 128),
		replayBuffer: NewReplayBuffer(10000),
	}
}

// SelectAction 选择动作
func (ppo *PPOOptimizer) SelectAction(state *State) (*Action, float64, float64) {
	action, logProb := ppo.policy.Sample(state)
	value := ppo.valueNet.Forward(state)
	return action, logProb, value
}

// ComputeAdvantages 计算优势函数（GAE）
func (ppo *PPOOptimizer) ComputeAdvantages(experiences []*Experience) {
	n := len(experiences)
	if n == 0 {
		return
	}
	
	// 计算TD误差
	for i := 0; i < n; i++ {
		exp := experiences[i]
		
		var nextValue float64
		if exp.Done {
			nextValue = 0
		} else if exp.NextState != nil {
			nextValue = ppo.valueNet.Forward(exp.NextState)
		}
		
		tdError := exp.Reward + ppo.config.Gamma*nextValue - exp.Value
		
		// GAE（简化版）
		exp.Advantage = tdError
	}
	
	// 标准化优势函数
	if n > 1 {
		mean := 0.0
		for _, exp := range experiences {
			mean += exp.Advantage
		}
		mean /= float64(n)
		
		std := 0.0
		for _, exp := range experiences {
			diff := exp.Advantage - mean
			std += diff * diff
		}
		std = math.Sqrt(std / float64(n))
		
		if std > 1e-8 {
			for _, exp := range experiences {
				exp.Advantage = (exp.Advantage - mean) / std
			}
		}
	}
}

// Update 更新网络参数
func (ppo *PPOOptimizer) Update() error {
	if ppo.replayBuffer.Size() < ppo.config.BatchSize {
		return fmt.Errorf("insufficient experiences: %d < %d", 
			ppo.replayBuffer.Size(), ppo.config.BatchSize)
	}
	
	// 采样批量
	batch := ppo.replayBuffer.Sample(ppo.config.BatchSize)
	
	// 计算优势函数
	ppo.ComputeAdvantages(batch)
	
	// PPO更新（简化版：这里只是框架，实际需要反向传播）
	// 在实际应用中，可以使用自动微分库或调用Python训练
	
	for epoch := 0; epoch < ppo.config.NEpochs; epoch++ {
		totalLoss := 0.0
		
		for _, exp := range batch {
			// 重新计算当前策略的logProb
			_, newLogProb := ppo.policy.Sample(exp.State)
			
			// PPO clip损失
			ratio := math.Exp(newLogProb - exp.LogProb)
			clippedRatio := math.Max(
				math.Min(ratio, 1+ppo.config.Epsilon),
				1-ppo.config.Epsilon,
			)
			
			policyLoss := -math.Min(ratio*exp.Advantage, clippedRatio*exp.Advantage)
			
			// 价值函数损失
			valueLoss := math.Pow(exp.Value-exp.Reward, 2)
			
			totalLoss += policyLoss + 0.5*valueLoss
		}
		
		// 这里需要梯度下降更新参数（简化版跳过）
		// 在实际应用中，调用训练函数或使用自动微分
		_ = totalLoss
	}
	
	return nil
}

// Optimize 优化参数（主接口）
func (ppo *PPOOptimizer) Optimize(
	initialState *State,
	rewardFunc func(*State, *Action) (float64, *State, bool),
	maxSteps int,
) (*Action, error) {
	
	state := initialState
	bestAction := &Action{}
	bestReward := math.Inf(-1)
	
	for step := 0; step < maxSteps; step++ {
		// 选择动作
		action, logProb, value := ppo.SelectAction(state)
		
		// 执行动作，获取奖励
		reward, nextState, done := rewardFunc(state, action)
		
		// 存储经验
		exp := &Experience{
			State:     state,
			Action:    action,
			Reward:    reward,
			NextState: nextState,
			Done:      done,
			Value:     value,
			LogProb:   logProb,
		}
		ppo.replayBuffer.Add(exp)
		
		// 记录最佳动作
		if reward > bestReward {
			bestReward = reward
			bestAction = action
		}
		
		// 更新网络
		if ppo.replayBuffer.Size() >= ppo.config.BatchSize {
			if err := ppo.Update(); err != nil {
				return nil, err
			}
		}
		
		if done {
			break
		}
		
		state = nextState
	}
	
	return bestAction, nil
}

// 辅助函数

// relu 激活函数
func relu(x float64) float64 {
	if x > 0 {
		return x
	}
	return 0
}

// ComputeReward 计算奖励（示例）
func ComputeReward(quality float64, size int64, targetSize int64, ssim float64) float64 {
	// 多目标奖励函数
	w1, w2, w3 := 0.4, 0.4, 0.2
	
	// 质量奖励（SSIM）
	qualityReward := ssim
	
	// 体积奖励（压缩率）
	sizeRatio := float64(size) / float64(targetSize)
	sizeReward := math.Max(0, 1.0-sizeRatio)
	
	// 质量参数奖励（接近目标质量）
	qualityPenalty := math.Abs(quality-90) / 90.0 // 假设目标是Q90
	
	totalReward := w1*qualityReward + w2*sizeReward - w3*qualityPenalty
	
	return totalReward
}
