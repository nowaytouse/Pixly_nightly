package main

import (
	"flag"
	"fmt"
	"os"
	
	"pixly/ai"
	"pixly/pkg/logging"
)

const Version = "4.2.0"

func main() {
	// 🔧 初始化统一日志系统
	logLevel := os.Getenv("PIXLY_LOG_LEVEL")
	if logLevel == "" {
		logLevel = "INFO"
	}
	jsonOutput := os.Getenv("PIXLY_LOG_JSON") != ""
	logging.InitLogging(logLevel, jsonOutput)
	
	// 解析命令行参数
	portStr := flag.String("port", "50052", "HTTP server port")
	modelDir := flag.String("model-dir", "./models", "AI model directory")
	flag.Parse()

	// 转换端口为int
	var port int
	if _, err := fmt.Sscanf(*portStr, "%d", &port); err != nil {
		port = 50052 // 默认端口
	}

	// 启动信息（使用统一日志）
	logging.Info("Pixly AI Service starting", map[string]interface{}{
		"version": Version,
		"port":    port,
	})
	logging.Info("🚀 Pixly AI Service v%s starting on http://localhost:%d", Version, port)
	logging.Info("📡 Endpoints:")
	logging.Info("   - Health Check:     http://localhost:%d/api/v1/health", port)
	logging.Info("   - Image Predict:    http://localhost:%d/api/v1/predict", port)
	logging.Info("   - Video Predict:    http://localhost:%d/api/v1/predict/video", port)
	logging.Info("   - Observations:     http://localhost:%d/api/v1/observations", port)
	logging.Info("")
	logging.Info("✅ AI Service is ready to serve requests")
	logging.Info("💡 Press Ctrl+C to stop")

	// 🔥 使用真实的HTTPGateway（集成Python Bridge）
	gateway := ai.NewHTTPGateway(*modelDir, port)
	
	// 启动服务器
	if err := gateway.Start(); err != nil {
		logging.Errorf("❌ Server failed to start: %v", err)
	}
}
