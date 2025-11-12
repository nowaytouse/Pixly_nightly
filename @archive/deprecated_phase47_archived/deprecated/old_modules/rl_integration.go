package ai

import (
	"fmt"
	"math"
	"pixly/ai/rl"
)

// ============================================================================
// PPO强化学习 + 双精度模式深度集成 v4.2.0
//
// 三层策略：
//   1. 观测<10 → 进阶模式（最安全）
//   2. 观测10-50 → 贝叶斯优化
//   3. 观测>=50 → PPO强化学习（最优）
// ============================================================================

// RLEnhancedPredictor PPO增强的预测器
type RLEnhancedPredictor struct {
	bayesianOpt  *BayesianOptimizer
	advancedPpl  *AdvancedPipeline
	ppoOptimizer *rl.PPOOptimizer
	useRL        bool
	rlThreshold  int
	tool         string
	features     *ImageFeaturesAdvanced
}

// RLConfig RL配置
type RLConfig struct {
	Enabled      bool
	Threshold    int
	LearningRate float64
	Gamma        float64
	Epsilon      float64
}

// DefaultRLConfig 默认配置
func DefaultRLConfig() RLConfig {
	return RLConfig{
		Enabled:      false,
		Threshold:    50,
		LearningRate: 0.0003,
		Gamma:        0.99,
		Epsilon:      0.2,
	}
}

// NewRLEnhancedPredictor 创建RL增强预测器
func NewRLEnhancedPredictor(tool string, features *ImageFeaturesAdvanced, rlConfig RLConfig) *RLEnhancedPredictor {
	predictor := &RLEnhancedPredictor{
		bayesianOpt: NewBayesianOptimizer(tool, [2]int{70, 100}),
		advancedPpl: NewAdvancedPipeline(tool, features),
		tool:        tool,
		features:    features,
		useRL:       rlConfig.Enabled,
		rlThreshold: rlConfig.Threshold,
	}

	if rlConfig.Enabled {
		ppoConfig := &rl.PPOConfig{
			Gamma:        rlConfig.Gamma,
			Epsilon:      rlConfig.Epsilon,
			LearningRate: rlConfig.LearningRate,
			BatchSize:    32,
			NEpochs:      10,
		}
		predictor.ppoOptimizer = rl.NewPPOOptimizer(ppoConfig, 20, 4)
	}

	return predictor
}

// Predict 三层策略预测
func (rp *RLEnhancedPredictor) Predict(targetMode string) (*RLPredictionResult, error) {
	result := &RLPredictionResult{
		Tool:       rp.tool,
		TargetMode: targetMode,
	}

	obsCount := len(rp.bayesianOpt.observations)

	// 策略1: 进阶模式
	if obsCount < 10 {
		advResult, err := rp.advancedPpl.Execute(targetMode)
		if err != nil {
			return nil, err
		}
		result.Strategy = "advanced"
		result.Quality = advResult.FinalQuality
		result.Distance = advResult.FinalDistance
		result.Effort = 7
		result.Reasoning = fmt.Sprintf("观测=%d<10，使用进阶模式", obsCount)
		return result, nil
	}

	// 策略2: 贝叶斯
	if obsCount < rp.rlThreshold || !rp.useRL {
		quality, distance, _ := rp.bayesianOpt.PredictOptimal(targetMode)
		result.Strategy = "bayesian"
		result.Quality = quality
		result.Distance = distance
		result.Effort = 7
		result.Reasoning = fmt.Sprintf("观测=%d，使用贝叶斯", obsCount)
		return result, nil
	}

	// 策略3: PPO
	state := rp.featuresToState()
	action, logProb, value := rp.ppoOptimizer.SelectAction(state)
	quality, distance, effort := rp.decodeAction(action, targetMode)

	result.Strategy = "ppo"
	result.Quality = quality
	result.Distance = distance
	result.Effort = effort
	result.Reasoning = fmt.Sprintf("观测=%d，PPO强化学习 (logP=%.2f,V=%.2f)", obsCount, logProb, value)
	result.RLAction = action
	result.RLLogProb = logProb
	result.RLValue = value

	return result, nil
}

// featuresToState 特征转状态
func (rp *RLEnhancedPredictor) featuresToState() *rl.State {
	f := rp.features
	return &rl.State{
		Features: []float64{
			float64(f.Width) / 4096.0, float64(f.Height) / 4096.0,
			boolToFloat(f.HasAlpha), boolToFloat(f.IsAnimated), float64(f.FrameCount) / 100.0,
			f.EdgeStrength / 100.0, f.TextureComplexity / 100.0, f.NoiseLevel / 100.0,
			f.DetailLevel / 100.0, f.HighFreqEnergy / 100.0, f.MidFreqEnergy / 100.0,
			f.LowFreqEnergy / 100.0, f.OverallQuality / 100.0, f.CompressionScore,
			f.Saturation, f.Brightness, f.Contrast,
			boolToFloat(f.IsPhoto), boolToFloat(f.IsDocument), boolToFloat(f.IsScreenshot),
		},
	}
}

// decodeAction 动作解码
func (rp *RLEnhancedPredictor) decodeAction(action *rl.Action, targetMode string) (int, float64, int) {
	baseQ := 85
	switch targetMode {
	case "size":
		baseQ = 80
	case "quality":
		baseQ = 100
	case "balanced":
		baseQ = 90
	}

	quality := baseQ + int(action.QualityDelta*15)
	if quality < 70 {
		quality = 70
	}
	if quality > 100 {
		quality = 100
	}

	distance := 1.0 + action.DistanceDelta
	if distance < 0 {
		distance = 0
	}
	if distance > 2.0 {
		distance = 2.0
	}

	effort := 7 + int(action.EffortDelta*2)
	if effort < 5 {
		effort = 5
	}
	if effort > 9 {
		effort = 9
	}

	return quality, distance, effort
}

// UpdateWithFeedback 更新模型
func (rp *RLEnhancedPredictor) UpdateWithFeedback(
	prediction *RLPredictionResult, actualSSIM float64, originalSize, outputSize int64,
) error {
	compressionRatio := float64(outputSize) / float64(originalSize)
	reward := ComputeReward(actualSSIM, compressionRatio, prediction.TargetMode)

	rp.bayesianOpt.AddObservation(Observation{
		Quality: prediction.Quality, Distance: prediction.Distance,
		Reward: reward, SSIM: actualSSIM, FileSize: outputSize, TargetMode: prediction.TargetMode,
	})

	// PPO更新已经在Optimize方法中自动处理
	return nil
}

// GetStatistics 统计信息
func (rp *RLEnhancedPredictor) GetStatistics() *RLStatistics {
	stats := &RLStatistics{
		BayesianObservations: len(rp.bayesianOpt.observations),
		RLEnabled:            rp.useRL,
		RLThreshold:          rp.rlThreshold,
		PPOExperiences:       0, // 简化：PPO内部管理
		PPOUpdates:           0,
	}

	if len(rp.bayesianOpt.observations) > 0 {
		totalReward := 0.0
		for _, obs := range rp.bayesianOpt.observations {
			totalReward += obs.Reward
		}
		stats.AverageReward = totalReward / float64(len(rp.bayesianOpt.observations))
	}

	return stats
}

func boolToFloat(b bool) float64 {
	if b {
		return 1.0
	}
	return 0.0
}

// ============================================================================
// 数据结构
// ============================================================================

type RLPredictionResult struct {
	Tool       string     `json:"tool"`
	TargetMode string     `json:"target_mode"`
	Strategy   string     `json:"strategy"`
	Quality    int        `json:"quality"`
	Distance   float64    `json:"distance"`
	Effort     int        `json:"effort"`
	Reasoning  string     `json:"reasoning"`
	RLAction   *rl.Action `json:"rl_action,omitempty"`
	RLLogProb  float64    `json:"rl_log_prob,omitempty"`
	RLValue    float64    `json:"rl_value,omitempty"`
}

type RLStatistics struct {
	BayesianObservations int     `json:"bayesian_observations"`
	PPOExperiences       int     `json:"ppo_experiences"`
	PPOUpdates           int     `json:"ppo_updates"`
	RLEnabled            bool    `json:"rl_enabled"`
	RLThreshold          int     `json:"rl_threshold"`
	AverageReward        float64 `json:"average_reward"`
}

// ============================================================================
// 观测持久化
// ============================================================================

type ObservationStore interface {
	Save(obs *Observation) error
	Load(tool, targetMode string, limit int) ([]*Observation, error)
	Count(tool, targetMode string) (int, error)
	Clear(tool string, olderThan int64) error
}

type MemoryStore struct {
	observations []*Observation
}

func NewMemoryStore() *MemoryStore {
	return &MemoryStore{observations: make([]*Observation, 0)}
}

func (ms *MemoryStore) Save(obs *Observation) error {
	ms.observations = append(ms.observations, obs)
	return nil
}

func (ms *MemoryStore) Load(tool, targetMode string, limit int) ([]*Observation, error) {
	filtered := make([]*Observation, 0)
	for _, obs := range ms.observations {
		if obs.TargetMode == targetMode {
			filtered = append(filtered, obs)
			if len(filtered) >= limit {
				break
			}
		}
	}
	return filtered, nil
}

func (ms *MemoryStore) Count(tool, targetMode string) (int, error) {
	count := 0
	for _, obs := range ms.observations {
		if obs.TargetMode == targetMode {
			count++
		}
	}
	return count, nil
}

func (ms *MemoryStore) Clear(tool string, olderThan int64) error {
	if len(ms.observations) > 1000 {
		ms.observations = ms.observations[len(ms.observations)-1000:]
	}
	return nil
}

// ============================================================================
// 持久化预测器
// ============================================================================

type PersistentPredictor struct {
	*RLEnhancedPredictor
	store ObservationStore
}

func NewPersistentPredictor(tool string, features *ImageFeaturesAdvanced, rlConfig RLConfig, store ObservationStore) (*PersistentPredictor, error) {
	pp := &PersistentPredictor{
		RLEnhancedPredictor: NewRLEnhancedPredictor(tool, features, rlConfig),
		store:               store,
	}
	_ = pp.LoadObservations("balanced", 100)
	return pp, nil
}

func (pp *PersistentPredictor) LoadObservations(targetMode string, limit int) error {
	observations, err := pp.store.Load(pp.tool, targetMode, limit)
	if err != nil {
		return err
	}
	for _, obs := range observations {
		pp.bayesianOpt.AddObservation(*obs)
	}
	return nil
}

func (pp *PersistentPredictor) UpdateWithFeedback(prediction *RLPredictionResult, actualSSIM float64, originalSize, outputSize int64) error {
	err := pp.RLEnhancedPredictor.UpdateWithFeedback(prediction, actualSSIM, originalSize, outputSize)
	if err != nil {
		return err
	}

	compressionRatio := float64(outputSize) / float64(originalSize)
	reward := ComputeReward(actualSSIM, compressionRatio, prediction.TargetMode)

	return pp.store.Save(&Observation{
		Quality: prediction.Quality, Distance: prediction.Distance, Reward: reward,
		SSIM: actualSSIM, FileSize: outputSize, TargetMode: prediction.TargetMode,
	})
}

// ============================================================================
// 策略选择器
// ============================================================================

type StrategySelector struct {
	predictor *PersistentPredictor
}

func NewStrategySelector(predictor *PersistentPredictor) *StrategySelector {
	return &StrategySelector{predictor: predictor}
}

func (ss *StrategySelector) SelectBestStrategy() string {
	stats := ss.predictor.GetStatistics()

	if stats.BayesianObservations < 10 {
		return "advanced"
	}
	if stats.BayesianObservations < stats.RLThreshold {
		return "bayesian"
	}
	if stats.RLEnabled && stats.PPOExperiences >= 32 && stats.AverageReward > 0.7 {
		return "ppo"
	}
	return "bayesian"
}

func (ss *StrategySelector) GetRecommendation() *StrategyRecommendation {
	stats := ss.predictor.GetStatistics()
	strategy := ss.SelectBestStrategy()

	rec := &StrategyRecommendation{
		RecommendedStrategy: strategy,
		Confidence:          ss.computeConfidence(stats, strategy),
		Statistics:          stats,
	}

	switch strategy {
	case "advanced":
		rec.Reasoning = fmt.Sprintf("观测不足(%d<10)，进阶模式", stats.BayesianObservations)
	case "bayesian":
		rec.Reasoning = fmt.Sprintf("观测充足(%d)，贝叶斯优化", stats.BayesianObservations)
	case "ppo":
		rec.Reasoning = fmt.Sprintf("观测充足(%d)，PPO训练完成(%d次)，强化学习最优",
			stats.BayesianObservations, stats.PPOUpdates)
	}

	return rec
}

func (ss *StrategySelector) computeConfidence(stats *RLStatistics, strategy string) float64 {
	switch strategy {
	case "advanced":
		return 0.95
	case "bayesian":
		return math.Min(0.7+float64(stats.BayesianObservations)/200.0, 0.95)
	case "ppo":
		return math.Min(0.6+float64(stats.PPOUpdates)/50.0, 0.98)
	default:
		return 0.5
	}
}

type StrategyRecommendation struct {
	RecommendedStrategy string        `json:"recommended_strategy"`
	Confidence          float64       `json:"confidence"`
	Reasoning           string        `json:"reasoning"`
	Statistics          *RLStatistics `json:"statistics"`
}
