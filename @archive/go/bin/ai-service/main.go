/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * PIXLY AI Service - Go机器学习内核
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 *
 * 🎯 架构要求 (CRITICAL - 反复强调，避免重复犯错)：
 *
 * 1. 【职责定位】
 *    🐹 Go AI服务: 仅负责AI预测，不做转换
 *       - HTTP API服务 (端口50052, 固定不变)
 *       - 机器学习预测 (quality, speed, lossless, format_options)
 *       - 模型管理 (LightGBM, PPO, Ensemble, A/B测试)
 *       - 训练队列 (SQLite feedback, 异步训练)
 *       - 反馈收集 (POST /api/v1/feedback/record)
 *
 * 2. 【禁止事项】
 *    ❌ 不要实现图像/视频转换功能 (转换全部由Rust核心完成)
 *    ❌ 不要调用外部CLI工具 (cjxl, avifenc等)
 *    ❌ 不要修改默认端口 (固定50052)
 *    ❌ 不要直接操作文件系统进行转换
 *
 * 3. 【API端点】
 *    - POST /api/ai/predict          - 单个预测
 *    - POST /api/ai/predict-batch    - 批量预测
 *    - POST /api/v1/feedback/record  - 反馈记录
 *    - GET  /api/v1/models/list      - 模型列表
 *    - POST /api/v1/training/trigger - 触发训练
 *    - GET  /health                  - 健康检查
 *
 * 4. 【AI预测字段 (Phase 36)】
 *    Response必须包含:
 *    - quality: int                  // 质量参数 (0-100)
 *    - speed: int                    // 速度参数
 *    - lossless: bool                // 🔥 智能预测无损模式
 *    - format_options: []KeyValue    // 🔥 格式特定参数
 *    - confidence: float64           // 置信度
 *    - reasoning: string             // 推理原因
 *
 * 5. 【双内核架构】
 *    ```
 *    🦀 Rust (转换执行) ←HTTP→ 🐹 Go (AI预测) ←SQLite→ 🐍 Python (训练)
 *         CLI工具                HTTP API              LightGBM
 *         Native编码器           多模型                增量训练
 *         策略系统               A/B测试               模型评估
 *    ```
 *
 * 6. 【测试方法】
 *    ```bash
 *    # 启动服务
 *    go run cmd/ai-service/main.go --port 50052
 *
 *    # 测试预测
 *    curl -X POST http://localhost:50052/api/v1/predict \
 *      -H 'Content-Type: application/json' \
 *      -d '{"image_path":"/path/to/image.jpg","tool":"jxl","target_quality":85,"optimize_mode":"balanced"}'
 *
 *    # 检查健康
 *    curl http://localhost:50052/health
 *    # 或
 *    curl http://localhost:50052/api/v1/health
 *    ```
 *
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 */
package main

import (
	"flag"
	"log"
	"os"
	"path/filepath"

	"pixly/ai"
)

func main() {
	// ✅ 无参数时显示TUI说明书
	if len(os.Args) == 1 {
		showTUIManual()
		return
	}

	// 命令行参数
	port := flag.Int("port", 50052, "HTTP服务端口")
	modelDir := flag.String("models", "./models", "模型文件目录")
	flag.Parse()

	// 打印横幅
	printBanner()

	// 检查Model directory
	absModelDir, err := filepath.Abs(*modelDir)
	if err != nil {
		log.Fatalf("❌ Failed to resolve model directory path: %v", err)
	}

	if _, err := os.Stat(absModelDir); os.IsNotExist(err) {
		log.Printf("⚠️  Model directory does not exist: %s", absModelDir)
		log.Printf("💡 Hint: Please run training script first to generate models")
		log.Printf("   python3 tools/train_ai_models.py --eagle-dir \"data/untitled folder\"")
	} else {
		log.Printf("📂 Model directory: %s", absModelDir)

		// 检查模型文件
		checkModelFiles(absModelDir)
	}

	// 创建并启动HTTP网关
	gateway := ai.NewHTTPGateway(absModelDir, *port)

	log.Printf("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	log.Printf("🚀 Pixly AI Service v4.2.0 Starting...")
	log.Printf("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")

	if err := gateway.Start(); err != nil {
		log.Fatalf("❌ Service failed to start: %v", err)
	}
}

func printBanner() {
	banner := `
╔════════════════════════════════════════════════════════════╗
║                                                            ║
║     ██████╗ ██╗██╗  ██╗██╗  ██╗   ██╗                     ║
║     ██╔══██╗██║╚██╗██╔╝██║  ╚██╗ ██╔╝                     ║
║     ██████╔╝██║ ╚███╔╝ ██║   ╚████╔╝                      ║
║     ██╔═══╝ ██║ ██╔██╗ ██║    ╚██╔╝                       ║
║     ██║     ██║██╔╝ ██╗███████╗██║                        ║
║     ╚═╝     ╚═╝╚═╝  ╚═╝╚══════╝╚═╝                        ║
║                                                            ║
║              AI Service v4.2.0                             ║
║         真实AI驱动的图像优化系统                           ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
`
	log.Println(banner)
}

func checkModelFiles(modelDir string) {
	tools := []string{"jxl", "avif", "webp"}
	foundCount := 0

	for _, tool := range tools {
		txtPath := filepath.Join(modelDir, "lightgbm_"+tool+".txt")
		jsonPath := filepath.Join(modelDir, "lightgbm_"+tool+".json")

		if _, err := os.Stat(txtPath); err == nil {
			if _, err := os.Stat(jsonPath); err == nil {
				log.Printf("   ✅ %s model loaded", tool)
				foundCount++
			}
		}
	}

	if foundCount == 0 {
		log.Printf("   ⚠️  No model files found")
	} else if foundCount < 3 {
		log.Printf("   ⚠️  Only found %d/3 models", foundCount)
	} else {
		log.Printf("   ✅ All models ready (%d/3)", foundCount)
	}
}

// ✅ TUI说明书 - 无参数时显示
func showTUIManual() {
	banner := `
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║   ██████╗ ██╗██╗  ██╗██╗  ██╗   ██╗                        ║
║   ██╔══██╗██║╚██╗██╔╝██║  ╚██╗ ██╔╝                        ║
║   ██████╔╝██║ ╚███╔╝ ██║   ╚████╔╝                         ║
║   ██╔═══╝ ██║ ██╔██╗ ██║    ╚██╔╝                          ║
║   ██║     ██║██╔╝ ██╗███████╗██║                           ║
║   ╚═╝     ╚═╝╚═╝  ╚═╝╚══════╝╚═╝                           ║
║                                                              ║
║   PIXLY AI Service - AI Prediction Service                  ║
║   Version: v4.2.0                                            ║
║   Status: Production Ready                                   ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
`
	log.Println(banner)
	log.Println("\n📚 TUI说明书 (TUI Manual)\n")
	log.Println("═══════════════════════════════════════════════════════════\n")

	log.Println("📖 服务说明 (Service Description):\n")
	log.Println("  PIXLY AI Service提供基于LightGBM模型的AI预测功能")
	log.Println("  AI Service provides LightGBM model-based predictions\n")

	log.Println("═══════════════════════════════════════════════════════════\n")
	log.Println("🚀 启动服务 (Start Service):\n")
	log.Println("  ai-service --port 50052 --models ./models")
	log.Println("      启动AI服务在指定端口 (Start AI service on specified port)\n")
	log.Println("  参数说明 (Parameters):")
	log.Println("    --port     HTTP服务端口 (默认: 50052)")
	log.Println("    --models   模型文件目录 (默认: ./models)\n")

	log.Println("═══════════════════════════════════════════════════════════\n")
	log.Println("🎯 功能特性 (Features):\n")
	log.Println("  ✅ 格式推荐 (Format recommendation)")
	log.Println("  ✅ 参数预测 (Parameter prediction)")
	log.Println("  ✅ 质量评估 (Quality assessment)")
	log.Println("  ✅ SSIM验证 (SSIM validation)")
	log.Println("  ✅ HTTP API接口 (HTTP API interface)\n")

	log.Println("═══════════════════════════════════════════════════════════\n")
	log.Println("💡 API端点 (API Endpoints):\n")
	log.Println("  POST /api/ai/predict")
	log.Println("      AI参数预测 (AI parameter prediction)\n")
	log.Println("  POST /api/ai/predict-batch")
	log.Println("      批量预测 (Batch prediction)\n")
	log.Println("  POST /api/ai/assess-quality")
	log.Println("      质量评估 (Quality assessment)\n")
	log.Println("  GET /api/ai/health")
	log.Println("      健康检查 (Health check)\n")

	log.Println("═══════════════════════════════════════════════════════════\n")
	log.Println("📝 模型训练 (Model Training):\n")
	log.Println("  python3 tools/train_ai_models.py --eagle-dir \"data/your-folder\"")
	log.Println("      训练AI模型 (Train AI models)\n")
	log.Println("  模型文件 (Model files):")
	log.Println("    models/lightgbm_format.txt")
	log.Println("    models/lightgbm_quality.txt")
	log.Println("    models/lightgbm_speed.txt\n")

	log.Println("═══════════════════════════════════════════════════════════\n")
	log.Println("🔗 更多信息 (More Info):\n")
	log.Println("  文档: docs/AI_INTEGRATION.md")
	log.Println("  问题: https://github.com/your-repo/issues\n")

	log.Println("═══════════════════════════════════════════════════════════\n")
	log.Println("提示: 运行 'ai-service --help' 查看完整参数列表")
	log.Println("Hint: Run 'ai-service --help' for full parameter list\n")
}
