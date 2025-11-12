package main

import (
	"fmt"
	"strings"
)

// TUIManual 提供TUI交互式说明书
type TUIManual struct{}

// ShowManual 显示完整的TUI说明书
func (t *TUIManual) ShowManual() {
	manual := t.generateManual()
	fmt.Print(manual)
}

// ShowQuickHelp 显示快速帮助
func (t *TUIManual) ShowQuickHelp() {
	help := t.generateQuickHelp()
	fmt.Print(help)
}

// generateManual 生成完整说明书
func (t *TUIManual) generateManual() string {
	var sb strings.Builder
	
	sb.WriteString("\n")
	sb.WriteString("╔══════════════════════════════════════════════════════════════════════════════╗\n")
	sb.WriteString("║                      Pixly AI Service - TUI Manual                          ║\n")
	sb.WriteString("║                        智能参数预测服务                                      ║\n")
	sb.WriteString("╚══════════════════════════════════════════════════════════════════════════════╝\n")
	sb.WriteString("\n")
	
	// 1. 概述
	sb.WriteString("📖 概述\n")
	sb.WriteString("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
	sb.WriteString("  Pixly AI Service是基于Go+Python的机器学习服务，提供：\n")
	sb.WriteString("  • 图像转换参数智能预测 (LightGBM模型)\n")
	sb.WriteString("  • 视频编码参数优化建议\n")
	sb.WriteString("  • 反馈收集与持续学习\n")
	sb.WriteString("  • PPO强化学习训练\n")
	sb.WriteString("\n")
	
	// 2. 架构
	sb.WriteString("🏗️  架构\n")
	sb.WriteString("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
	sb.WriteString("  Go Service (HTTP/gRPC)\n")
	sb.WriteString("     ↓\n")
	sb.WriteString("  Python Bridge (CGo)\n")
	sb.WriteString("     ↓\n")
	sb.WriteString("  ML Models (LightGBM + PPO)\n")
	sb.WriteString("     ↓\n")
	sb.WriteString("  Training Queue (async)\n")
	sb.WriteString("\n")
	
	// 3. HTTP API
	sb.WriteString("🌐 HTTP API\n")
	sb.WriteString("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
	sb.WriteString("  默认端口: 50052\n")
	sb.WriteString("\n")
	sb.WriteString("  POST /api/v1/predict\n")
	sb.WriteString("    预测图像转换参数\n")
	sb.WriteString("    Body: {\n")
	sb.WriteString("      \"width\": 1920,\n")
	sb.WriteString("      \"height\": 1080,\n")
	sb.WriteString("      \"format\": \"jxl\",\n")
	sb.WriteString("      \"has_alpha\": false\n")
	sb.WriteString("    }\n")
	sb.WriteString("    Response: {\n")
	sb.WriteString("      \"quality\": 85,\n")
	sb.WriteString("      \"speed\": 4,\n")
	sb.WriteString("      \"lossless\": false\n")
	sb.WriteString("    }\n")
	sb.WriteString("\n")
	sb.WriteString("  POST /api/v1/video/predict\n")
	sb.WriteString("    预测视频编码参数\n")
	sb.WriteString("    Body: {\n")
	sb.WriteString("      \"width\": 1920,\n")
	sb.WriteString("      \"height\": 1080,\n")
	sb.WriteString("      \"fps\": 30,\n")
	sb.WriteString("      \"codec\": \"h264\"\n")
	sb.WriteString("    }\n")
	sb.WriteString("    Response: {\n")
	sb.WriteString("      \"crf\": 23,\n")
	sb.WriteString("      \"preset\": \"medium\"\n")
	sb.WriteString("    }\n")
	sb.WriteString("\n")
	sb.WriteString("  POST /api/v1/feedback\n")
	sb.WriteString("    提交反馈数据\n")
	sb.WriteString("    Body: {\n")
	sb.WriteString("      \"predicted\": {...},\n")
	sb.WriteString("      \"actual\": {...},\n")
	sb.WriteString("      \"ssim\": 0.95\n")
	sb.WriteString("    }\n")
	sb.WriteString("\n")
	sb.WriteString("  GET /health\n")
	sb.WriteString("    健康检查\n")
	sb.WriteString("\n")
	
	// 4. 命令行参数
	sb.WriteString("⚙️  命令行参数\n")
	sb.WriteString("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
	sb.WriteString("  --port          HTTP服务端口 (默认: 50052)\n")
	sb.WriteString("  --model-dir     模型目录路径\n")
	sb.WriteString("  --log-level     日志级别 (debug|info|warn|error)\n")
	sb.WriteString("  --manual        显示此说明书\n")
	sb.WriteString("  --help          显示快速帮助\n")
	sb.WriteString("  --version       显示版本信息\n")
	sb.WriteString("\n")
	
	// 5. 环境变量
	sb.WriteString("🌍 环境变量\n")
	sb.WriteString("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
	sb.WriteString("  PIXLY_AI_PORT         覆盖默认端口\n")
	sb.WriteString("  PIXLY_MODEL_DIR       模型目录\n")
	sb.WriteString("  PIXLY_PYTHON_PATH     Python解释器路径\n")
	sb.WriteString("  PIXLY_LOG_LEVEL       日志级别\n")
	sb.WriteString("  PIXLY_TRAINING_MODE   启用训练模式\n")
	sb.WriteString("\n")
	
	// 6. 机器学习模型
	sb.WriteString("🤖 机器学习模型\n")
	sb.WriteString("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
	sb.WriteString("  1. LightGBM参数预测器\n")
	sb.WriteString("     • 模型文件: models/lightgbm_params.txt\n")
	sb.WriteString("     • 输入特征: 分辨率、格式、Alpha通道、复杂度\n")
	sb.WriteString("     • 输出参数: quality, speed, lossless\n")
	sb.WriteString("\n")
	sb.WriteString("  2. PPO强化学习\n")
	sb.WriteString("     • 模型文件: models/ppo_model.zip\n")
	sb.WriteString("     • 奖励函数: SSIM质量 + 压缩率 + 速度\n")
	sb.WriteString("     • 训练触发: 收集100条反馈后自动训练\n")
	sb.WriteString("\n")
	
	// 7. 反馈系统
	sb.WriteString("💬 反馈系统\n")
	sb.WriteString("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
	sb.WriteString("  流程:\n")
	sb.WriteString("    1. AI预测参数\n")
	sb.WriteString("    2. 用户执行转换\n")
	sb.WriteString("    3. 系统计算SSIM质量\n")
	sb.WriteString("    4. 提交反馈到服务\n")
	sb.WriteString("    5. 累积到训练队列\n")
	sb.WriteString("    6. 定期触发模型重训练\n")
	sb.WriteString("\n")
	sb.WriteString("  数据存储:\n")
	sb.WriteString("    • 反馈文件: data/feedback/*.json\n")
	sb.WriteString("    • 训练数据: data/training/*.csv\n")
	sb.WriteString("\n")
	
	// 8. 性能指标
	sb.WriteString("📊 性能指标\n")
	sb.WriteString("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
	sb.WriteString("  • 预测延迟: <50ms (P99)\n")
	sb.WriteString("  • 并发请求: 100+ QPS\n")
	sb.WriteString("  • 模型大小: ~2MB (LightGBM)\n")
	sb.WriteString("  • 内存占用: ~100MB\n")
	sb.WriteString("  • 预测准确率: >85% (SSIM >0.90)\n")
	sb.WriteString("\n")
	
	// 9. 集成示例
	sb.WriteString("🔧 集成示例\n")
	sb.WriteString("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
	sb.WriteString("  Rust CLI调用:\n")
	sb.WriteString("    pixly-rust convert input.png output.jxl --ai\n")
	sb.WriteString("\n")
	sb.WriteString("  Go代码调用:\n")
	sb.WriteString("    params, err := client.PredictImageParams(ctx, features)\n")
	sb.WriteString("\n")
	sb.WriteString("  JavaScript调用:\n")
	sb.WriteString("    const params = await window.AIClient.predictFormat(features);\n")
	sb.WriteString("\n")
	
	// 10. 故障排查
	sb.WriteString("🔍 故障排查\n")
	sb.WriteString("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
	sb.WriteString("  Q: 服务启动失败\n")
	sb.WriteString("  A: 检查端口50052是否被占用，查看日志输出\n")
	sb.WriteString("\n")
	sb.WriteString("  Q: 预测失败\n")
	sb.WriteString("  A: 检查模型文件是否存在，Python环境是否正确\n")
	sb.WriteString("\n")
	sb.WriteString("  Q: 训练不触发\n")
	sb.WriteString("  A: 确认反馈数量>=100，检查training_queue.log\n")
	sb.WriteString("\n")
	
	// 11. 质量原则
	sb.WriteString("🎯 质量原则\n")
	sb.WriteString("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
	sb.WriteString("  • 真实性第一: 无mock数据，无fallback\n")
	sb.WriteString("  • 响亮报错: 模型缺失直接报错\n")
	sb.WriteString("  • 详细日志: 每次预测记录完整信息\n")
	sb.WriteString("  • 架构清晰: Go(服务) + Python(ML) 职责分明\n")
	sb.WriteString("\n")
	
	// 版权信息
	sb.WriteString("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
	sb.WriteString("  Pixly AI Service v4.2.0\n")
	sb.WriteString("  © 2025 Pixly Project\n")
	sb.WriteString("  遵循 @PROJECT_QUALITY_MANIFESTO.md\n")
	sb.WriteString("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
	sb.WriteString("\n")
	
	return sb.String()
}

// generateQuickHelp 生成快速帮助
func (t *TUIManual) generateQuickHelp() string {
	var sb strings.Builder
	
	sb.WriteString("\n")
	sb.WriteString("Pixly AI Service v4.2.0 - 智能参数预测服务\n")
	sb.WriteString("\n")
	sb.WriteString("使用方法:\n")
	sb.WriteString("  pixly-ai [选项]\n")
	sb.WriteString("\n")
	sb.WriteString("选项:\n")
	sb.WriteString("  --port <端口>       HTTP服务端口 (默认: 50052)\n")
	sb.WriteString("  --model-dir <路径>  模型目录路径\n")
	sb.WriteString("  --log-level <级别>  日志级别 (debug|info|warn|error)\n")
	sb.WriteString("  --manual            显示完整说明书\n")
	sb.WriteString("  --help              显示此帮助\n")
	sb.WriteString("  --version           显示版本信息\n")
	sb.WriteString("\n")
	sb.WriteString("HTTP API:\n")
	sb.WriteString("  POST /api/v1/predict         图像参数预测\n")
	sb.WriteString("  POST /api/v1/video/predict   视频参数预测\n")
	sb.WriteString("  POST /api/v1/feedback        提交反馈\n")
	sb.WriteString("  GET  /health                 健康检查\n")
	sb.WriteString("\n")
	sb.WriteString("更多信息:\n")
	sb.WriteString("  运行 'pixly-ai --manual' 查看完整TUI说明书\n")
	sb.WriteString("\n")
	
	return sb.String()
}
