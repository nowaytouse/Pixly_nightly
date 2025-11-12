package ai

import (
	"fmt"
	"testing"
)

// TestBayesianOptimizer_BasicUsage 测试基础模式
func TestBayesianOptimizer_BasicUsage(t *testing.T) {
	// 创建贝叶斯优化器
	optimizer := NewBayesianOptimizer("jxl", [2]int{70, 100})
	
	// 模拟添加历史观测
	observations := []Observation{
		{Quality: 85, Distance: 0.5, Reward: 0.85, SSIM: 0.95, FileSize: 12000, TargetMode: "balanced"},
		{Quality: 90, Distance: 0.3, Reward: 0.90, SSIM: 0.97, FileSize: 13000, TargetMode: "balanced"},
		{Quality: 80, Distance: 0.8, Reward: 0.75, SSIM: 0.92, FileSize: 10000, TargetMode: "size"},
	}
	
	for _, obs := range observations {
		optimizer.AddObservation(obs)
	}
	
	// 测试平衡模式预测
	quality, distance, err := optimizer.PredictOptimal("balanced")
	if err != nil {
		t.Fatalf("预测失败: %v", err)
	}
	
	fmt.Printf("✅ 基础模式预测结果:\n")
	fmt.Printf("   工具: jxl\n")
	fmt.Printf("   模式: balanced\n")
	fmt.Printf("   质量: %d\n", quality)
	fmt.Printf("   Distance: %.1f\n", distance)
	fmt.Printf("   历史观测: %d条\n", len(observations))
	
	// 验证结果在合理范围内
	if quality < 70 || quality > 100 {
		t.Errorf("质量超出范围: %d", quality)
	}
	if distance < 0 || distance > 2.0 {
		t.Errorf("Distance超出范围: %.1f", distance)
	}
}

// TestAdvancedPipeline_BasicUsage 测试进阶模式
func TestAdvancedPipeline_BasicUsage(t *testing.T) {
	// 创建进阶特征
	features := &ImageFeaturesAdvanced{
		Width:             1920,
		Height:            1080,
		HasAlpha:          false,
		IsAnimated:        false,
		FrameCount:        1,
		EdgeStrength:      70.0,
		TextureComplexity: 85.0,
		NoiseLevel:        15.0,
		DetailLevel:       80.0,
		HighFreqEnergy:    60.0,
		MidFreqEnergy:     70.0,
		LowFreqEnergy:     65.0,
		OverallQuality:    82.0,
		CompressionScore:  0.75,
		ColorSpace:        "RGB",
		ColorRange:        250.0,
		Saturation:        0.8,
		Brightness:        0.6,
		Contrast:          0.7,
		IsPhoto:           true,
		IsDocument:        false,
		IsScreenshot:      false,
		HasText:           false,
	}
	
	// 创建进阶流水线
	pipeline := NewAdvancedPipeline("jxl", features)
	
	// 执行流水线
	result, err := pipeline.Execute("balanced")
	if err != nil {
		t.Fatalf("流水线执行失败: %v", err)
	}
	
	fmt.Printf("\n✅ 进阶模式流水线结果:\n")
	fmt.Printf("   工具: %s\n", result.Tool)
	fmt.Printf("   模式: %s\n", result.Mode)
	fmt.Printf("   基础质量: %d\n", result.BaseQuality)
	fmt.Printf("   基础Distance: %.1f\n", result.BaseDistance)
	fmt.Printf("   最终质量: %d (微调: %+d)\n", result.FinalQuality, result.FinalQuality-result.BaseQuality)
	fmt.Printf("   最终Distance: %.1f (微调: %+.1f)\n", result.FinalDistance, result.FinalDistance-result.BaseDistance)
	fmt.Printf("   执行阶段数: %d\n", len(result.Stages))
	
	// 打印各阶段
	fmt.Printf("\n   执行阶段:\n")
	for _, stage := range result.Stages {
		if stage.Executed {
			fmt.Printf("     ✓ [%d] %s\n", stage.Order, stage.Name)
		}
	}
	
	// 验证结果
	if result.FinalQuality < 70 || result.FinalQuality > 100 {
		t.Errorf("最终质量超出范围: %d", result.FinalQuality)
	}
	if len(result.Stages) == 0 {
		t.Error("没有执行任何阶段")
	}
}

// TestRewardComputation 测试奖励函数
func TestRewardComputation(t *testing.T) {
	testCases := []struct {
		mode             string
		ssim             float64
		compressionRatio float64
		expectedMin      float64
		expectedMax      float64
	}{
		{"size", 0.95, 0.6, 0.5, 0.7},       // 体积优先 (0.3*0.95 + 0.7*0.4 = 0.565)
		{"balanced", 0.95, 0.6, 0.6, 0.8},   // 平衡模式 (0.5*0.95 + 0.5*0.4 = 0.675)
		{"quality", 0.95, 0.6, 0.8, 1.0},    // 质量优先 (0.8*0.95 + 0.2*0.4 = 0.840)
	}
	
	fmt.Printf("\n✅ 奖励函数测试:\n")
	for _, tc := range testCases {
		reward := ComputeReward(tc.ssim, tc.compressionRatio, tc.mode)
		
		fmt.Printf("   %s模式: SSIM=%.2f, 压缩率=%.0f%% → 奖励=%.3f\n",
			tc.mode, tc.ssim, tc.compressionRatio*100, reward)
		
		if reward < tc.expectedMin || reward > tc.expectedMax {
			t.Errorf("%s模式奖励异常: %.3f (期望范围: %.3f-%.3f)",
				tc.mode, reward, tc.expectedMin, tc.expectedMax)
		}
	}
}

// TestPrecisionPredictor_ModeSwitching 测试模式切换
func TestPrecisionPredictor_ModeSwitching(t *testing.T) {
	features := &ImageFeaturesAdvanced{
		Width:            1920,
		Height:           1080,
		TextureComplexity: 70.0,
		NoiseLevel:       20.0,
		IsPhoto:          true,
	}
	
	fmt.Printf("\n✅ 模式切换测试:\n")
	
	// 测试基础模式
	basicPredictor := NewPrecisionPredictor(PrecisionBasic, "jxl", features)
	basicResult, err := basicPredictor.Predict("balanced")
	if err != nil {
		t.Fatalf("基础模式预测失败: %v", err)
	}
	
	fmt.Printf("   基础模式: %v\n", basicResult)
	
	// 测试进阶模式
	advancedPredictor := NewPrecisionPredictor(PrecisionAdvanced, "jxl", features)
	advancedResult, err := advancedPredictor.Predict("balanced")
	if err != nil {
		t.Fatalf("进阶模式预测失败: %v", err)
	}
	
	fmt.Printf("   进阶模式: 工具=%s, 最终质量=%d\n",
		advancedResult.(*AdvancedResult).Tool,
		advancedResult.(*AdvancedResult).FinalQuality)
}

// BenchmarkBayesianOptimizer 基础模式性能基准测试
func BenchmarkBayesianOptimizer(b *testing.B) {
	optimizer := NewBayesianOptimizer("jxl", [2]int{70, 100})
	
	// 添加一些观测
	for i := 0; i < 10; i++ {
		optimizer.AddObservation(Observation{
			Quality:    80 + i,
			Distance:   0.5,
			Reward:     0.8,
			SSIM:       0.95,
			FileSize:   10000,
			TargetMode: "balanced",
		})
	}
	
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, _, _ = optimizer.PredictOptimal("balanced")
	}
}

// BenchmarkAdvancedPipeline 进阶模式性能基准测试
func BenchmarkAdvancedPipeline(b *testing.B) {
	features := &ImageFeaturesAdvanced{
		Width:             1920,
		Height:            1080,
		TextureComplexity: 70.0,
		NoiseLevel:        20.0,
		IsPhoto:           true,
	}
	
	pipeline := NewAdvancedPipeline("jxl", features)
	
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, _ = pipeline.Execute("balanced")
	}
}
