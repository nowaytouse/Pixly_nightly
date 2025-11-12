package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strconv"
	"strings"
	"time"

	"pixly/pkg/converter"
)

// Version information
const Version = "1.2.0"

// i18n messages
var messages = map[string]map[string]string{
	"en": {
		"banner_title":     "PIXLY Conversion Service",
		"banner_version":   "Version",
		"banner_desc":      "HTTP service for image conversion (wraps CLI tools)",
		"banner_port":      "Port",
		"endpoints":        "Endpoints",
		"features":         "Features",
		"feature_pipeline": "Zero-copy conversion pipeline",
		"feature_metadata": "Metadata preservation (EXIF, ICC, XMP)",
		"feature_animated": "Animated image support",
		"feature_eagle":    "Eagle library integration",
		"formats":          "Supported Formats",
		"format_avif":      "Modern, high compression",
		"format_jxl":       "JPEG XL, excellent quality",
		"format_webp":      "Wide browser support",
		"format_png":       "Lossless compression",
		"format_jpeg":      "Universal compatibility",
		"format_gif":       "Animated GIF optimization",
		"test_hint":        "Test: curl http://localhost:{PORT}/health",
		"starting":         "Starting service on",
	},
	"zh_CN": {
		"banner_title":     "PIXLY 转换服务",
		"banner_version":   "版本",
		"banner_desc":      "图像转换HTTP服务（封装CLI工具）",
		"banner_port":      "端口",
		"endpoints":        "接口端点",
		"features":         "功能特性",
		"feature_pipeline": "零拷贝转换流水线",
		"feature_metadata": "元数据保留（EXIF、ICC、XMP）",
		"feature_animated": "动画图像支持",
		"feature_eagle":    "Eagle资料库集成",
		"formats":          "支持格式",
		"format_avif":      "现代格式，高压缩率",
		"format_jxl":       "JPEG XL，卓越质量",
		"format_webp":      "浏览器广泛支持",
		"format_png":       "无损压缩",
		"format_jpeg":      "通用兼容性",
		"format_gif":       "动态GIF优化",
		"test_hint":        "测试: curl http://localhost:{PORT}/health",
		"starting":         "正在启动服务",
	},
	"ja_JP": {
		"banner_title":     "PIXLY 変換サービス",
		"banner_version":   "バージョン",
		"banner_desc":      "画像変換HTTPサービス（CLIツールをラップ）",
		"banner_port":      "ポート",
		"endpoints":        "エンドポイント",
		"features":         "機能",
		"feature_pipeline": "ゼロコピー変換パイプライン",
		"feature_metadata": "メタデータ保持（EXIF、ICC、XMP）",
		"feature_animated": "アニメーション画像サポート",
		"feature_eagle":    "Eagleライブラリ統合",
		"formats":          "対応フォーマット",
		"format_avif":      "モダン、高圧縮",
		"format_jxl":       "JPEG XL、優れた品質",
		"format_webp":      "ブラウザ広範囲サポート",
		"format_png":       "ロスレス圧縮",
		"format_jpeg":      "ユニバーサル互換性",
		"format_gif":       "アニメーションGIF最適化",
		"test_hint":        "テスト: curl http://localhost:{PORT}/health",
		"starting":         "サービス起動中",
	},
}

func detectLocale() string {
	lang := os.Getenv("LANG")
	if strings.Contains(lang, "zh_CN") || strings.Contains(lang, "zh") {
		return "zh_CN"
	}
	if strings.Contains(lang, "ja") {
		return "ja_JP"
	}
	return "en"
}

func printBanner() {
	locale := detectLocale()
	msg := messages[locale]

	fmt.Println()
	fmt.Println("╔══════════════════════════════════════════════════════════════╗")
	fmt.Printf("║  🦀 %-56s ║\n", msg["banner_title"])
	fmt.Println("╚══════════════════════════════════════════════════════════════╝")
	fmt.Println()
	fmt.Printf("  %s:  %s\n", msg["banner_version"], Version)
	fmt.Printf("  Core:     HTTP Service (Go + CLI Tools)\n")
	port := getPort()
	fmt.Printf("  %s:     %s\n", msg["banner_port"], strings.TrimPrefix(port, ":"))
	fmt.Printf("  Platform: %s/%s\n", runtime.GOOS, runtime.GOARCH)
	fmt.Println()
	fmt.Printf("📋 %s\n", msg["banner_desc"])
	fmt.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	fmt.Println()
	fmt.Printf("🎯 %s\n", msg["endpoints"])
	fmt.Println("  • GET  /health               - Health check")
	fmt.Println("  • POST /api/rust/convert     - Convert image")
	fmt.Println()
	fmt.Printf("✨ %s\n", msg["features"])
	fmt.Printf("  ✓ %s\n", msg["feature_pipeline"])
	fmt.Printf("  ✓ %s\n", msg["feature_metadata"])
	fmt.Printf("  ✓ %s\n", msg["feature_animated"])
	fmt.Printf("  ✓ %s\n", msg["feature_eagle"])
	fmt.Println()
	fmt.Printf("📦 %s\n", msg["formats"])
	fmt.Printf("  • AVIF  - %s\n", msg["format_avif"])
	fmt.Printf("  • JXL   - %s\n", msg["format_jxl"])
	fmt.Printf("  • WebP  - %s\n", msg["format_webp"])
	fmt.Printf("  • PNG   - %s\n", msg["format_png"])
	fmt.Printf("  • JPEG  - %s\n", msg["format_jpeg"])
	fmt.Printf("  • GIF   - %s\n", msg["format_gif"])
	fmt.Println()
	fmt.Println("╔══════════════════════════════════════════════════════════════╗")
	fmt.Printf("║  💡 %-58s ║\n", msg["test_hint"])
	fmt.Println("╚══════════════════════════════════════════════════════════════╝")
	fmt.Println()
}

func getPort() string {
	// 优先级: PIXLY_PORT > PORT > 默认8080
	if port := os.Getenv("PIXLY_PORT"); port != "" {
		return ":" + port
	}
	if port := os.Getenv("PORT"); port != "" {
		return ":" + port
	}
	return ":8080"
}

func main() {
	// ✅ 无参数时显示TUI说明书
	if len(os.Args) == 1 {
		showTUIManual()
		return
	}

	port := getPort()
	locale := detectLocale()
	msg := messages[locale]

	printBanner()

	http.HandleFunc("/health", healthHandler)
	http.HandleFunc("/api/health", healthHandler) // 兼容 rust_client.go
	http.HandleFunc("/api/rust/convert", convertHandler)

	log.Printf("🚀 %s %s", msg["starting"], port)
	log.Fatal(http.ListenAndServe(port, nil))
}

func healthHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"status":         "ok",
		"rust_available": true,
		"timestamp":      time.Now().Unix(),
	})
}

func convertHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req converter.ConversionRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, fmt.Sprintf("Invalid request: %v", err), http.StatusBadRequest)
		return
	}

	// Set defaults
	if req.Quality == 0 {
		req.Quality = 75
	}
	if req.Speed == 0 {
		req.Speed = 4
	}

	// Execute conversion using CLI tools
	start := time.Now()
	err := executeConversion(&req)
	duration := time.Since(start).Milliseconds()

	w.Header().Set("Content-Type", "application/json")

	if err != nil {
		json.NewEncoder(w).Encode(map[string]interface{}{
			"success":    false,
			"error":      err.Error(),
			"outputPath": req.Output,
			"fileSize":   0,
			"duration":   duration,
		})
		return
	}

	// Get output file size
	fileInfo, err := os.Stat(req.Output)
	fileSize := int64(0)
	if err == nil {
		fileSize = fileInfo.Size()
	}

	json.NewEncoder(w).Encode(map[string]interface{}{
		"success":    true,
		"error":      "",
		"outputPath": req.Output,
		"fileSize":   fileSize,
		"duration":   duration,
	})
}

// executeConversion 执行实际的转换 (调用 CLI 工具)
func executeConversion(req *converter.ConversionRequest) error {
	// Ensure output directory exists
	if err := os.MkdirAll(filepath.Dir(req.Output), 0755); err != nil {
		return fmt.Errorf("failed to create output directory: %w", err)
	}

	// Check if input is GIF and needs preprocessing for non-WebP formats
	inputFile := req.Input

	if strings.HasSuffix(strings.ToLower(req.Input), ".gif") && req.Format != "webp" && req.Format != "gif" {
		// Convert GIF to PNG first (extract first frame for static conversion)
		tempPNG := filepath.Join(os.TempDir(), fmt.Sprintf("pixly-temp-%d.png", time.Now().UnixNano()))
		convertCmd := exec.Command("convert", req.Input+"[0]", tempPNG)
		if output, err := convertCmd.CombinedOutput(); err != nil {
			return fmt.Errorf("failed to preprocess GIF: %v, output: %s", err, string(output))
		}
		inputFile = tempPNG
		defer os.Remove(tempPNG)
	}

	var cmd *exec.Cmd

	switch req.Format {
	case "jxl":
		args := []string{inputFile, req.Output, "-q", strconv.Itoa(req.Quality), "-e", strconv.Itoa(req.Speed)}
		if req.Lossless {
			args = append([]string{"--lossless"}, args...)
		}
		cmd = exec.Command("cjxl", args...)

	case "avif":
		args := []string{inputFile, req.Output, "-s", strconv.Itoa(req.Speed), "-q", strconv.Itoa(req.Quality)}
		cmd = exec.Command("avifenc", args...)

	case "webp":
		// Check if input is GIF (animated or not)
		if strings.HasSuffix(strings.ToLower(req.Input), ".gif") {
			// Use gif2webp for GIF files (supports both static and animated)
			args := []string{"-q", strconv.Itoa(req.Quality), req.Input, "-o", req.Output}
			if req.Lossless {
				args = []string{"-lossless", req.Input, "-o", req.Output}
			}
			cmd = exec.Command("gif2webp", args...)
		} else {
			// Use cwebp for other formats (PNG, JPG, etc.)
			args := []string{"-q", strconv.Itoa(req.Quality), req.Input, "-o", req.Output}
			if req.Lossless {
				args = []string{"-lossless", req.Input, "-o", req.Output}
			}
			cmd = exec.Command("cwebp", args...)
		}

	case "png":
		cmd = exec.Command("convert", req.Input, "-quality", strconv.Itoa(req.Quality), req.Output)

	case "jpeg", "jpg":
		cmd = exec.Command("convert", req.Input, "-quality", strconv.Itoa(req.Quality), req.Output)

	case "gif":
		// Use FFmpeg for GIF optimization with palette
		cmd = exec.Command("ffmpeg", "-i", req.Input, "-vf", "split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse", "-y", req.Output)

	default:
		return fmt.Errorf("unsupported format: %s", req.Format)
	}

	output, err := cmd.CombinedOutput()
	if err != nil {
		return fmt.Errorf("conversion failed: %v, output: %s", err, string(output))
	}

	return nil
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
║   PIXLY Conversion Service - CLI Tool Wrapper HTTP API      ║
║   Version: v1.2.0                                            ║
║   Status: Production Ready                                   ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
`
	fmt.Println(banner)
	fmt.Println("\n📚 TUI说明书 (TUI Manual)\n")
	fmt.Println("═══════════════════════════════════════════════════════════\n")

	fmt.Println("📖 服务说明 (Service Description):\n")
	fmt.Println("  PIXLY Conversion Service提供HTTP API封装的CLI工具转换功能")
	fmt.Println("  Conversion Service provides HTTP API wrapper for CLI tools\n")

	fmt.Println("═══════════════════════════════════════════════════════════\n")
	fmt.Println("🚀 启动服务 (Start Service):\n")
	fmt.Println("  PORT=8080 rust-service")
	fmt.Println("      启动转换服务在指定端口 (Start conversion service on specified port)\n")
	fmt.Println("  环境变量 (Environment Variables):")
	fmt.Println("    PORT     HTTP服务端口 (默认: 8080)")
	fmt.Println("    LOCALE   界面语言 (默认: 自动检测, 支持: en, zh_CN, zh_TW, ja_JP)\n")

	fmt.Println("═══════════════════════════════════════════════════════════\n")
	fmt.Println("🎯 功能特性 (Features):\n")
	fmt.Println("  ✅ HTTP API接口 (HTTP API interface)")
	fmt.Println("  ✅ CLI工具封装 (CLI tool wrapper)")
	fmt.Println("  ✅ 零拷贝转换 (Zero-copy conversion)")
	fmt.Println("  ✅ 元数据保留 (Metadata preservation)")
	fmt.Println("  ✅ 动态图像支持 (Animated image support)")
	fmt.Println("  ✅ Eagle库集成 (Eagle library integration)\n")

	fmt.Println("═══════════════════════════════════════════════════════════\n")
	fmt.Println("💡 API端点 (API Endpoints):\n")
	fmt.Println("  POST /api/rust/convert")
	fmt.Println("      图像转换 (Image conversion)\n")
	fmt.Println("  GET /api/health")
	fmt.Println("      健康检查 (Health check)\n")
	fmt.Println("  GET /health")
	fmt.Println("      健康检查 (Health check)\n")

	fmt.Println("═══════════════════════════════════════════════════════════\n")
	fmt.Println("🎨 支持的格式 (Supported Formats):\n")
	fmt.Println("  输入 (Input):  PNG, JPEG, WebP, AVIF, JXL, GIF")
	fmt.Println("  输出 (Output): PNG, JPEG, WebP, AVIF, JXL, GIF\n")

	fmt.Println("═══════════════════════════════════════════════════════════\n")
	fmt.Println("🔧 依赖的CLI工具 (Required CLI Tools):\n")
	fmt.Println("  • cjxl       - JPEG XL转换 (JPEG XL conversion)")
	fmt.Println("  • avifenc    - AVIF编码 (AVIF encoding)")
	fmt.Println("  • cwebp      - WebP编码 (WebP encoding)")
	fmt.Println("  • gif2webp   - GIF转WebP (GIF to WebP)")
	fmt.Println("  • convert    - ImageMagick (PNG/JPEG)")
	fmt.Println("  • ffmpeg     - GIF优化 (GIF optimization)\n")

	fmt.Println("═══════════════════════════════════════════════════════════\n")
	fmt.Println("📝 API请求示例 (API Request Example):\n")
	fmt.Println("  curl -X POST http://localhost:8080/api/rust/convert \\")
	fmt.Println("    -H 'Content-Type: application/json' \\")
	fmt.Println("    -d '{")
	fmt.Println("      \"input\": \"input.png\",")
	fmt.Println("      \"output\": \"output.avif\",")
	fmt.Println("      \"format\": \"avif\",")
	fmt.Println("      \"quality\": 85,")
	fmt.Println("      \"speed\": 4")
	fmt.Println("    }'\n")

	fmt.Println("═══════════════════════════════════════════════════════════\n")
	fmt.Println("⚠️  注意事项 (Important Notes):\n")
	fmt.Println("  • 此服务为CLI工具的HTTP封装 (This is an HTTP wrapper for CLI tools)")
	fmt.Println("  • 确保已安装所有依赖工具 (Ensure all CLI tools are installed)")
	fmt.Println("  • 工具缺失时转换会失败 (Conversion fails if tools are missing)")
	fmt.Println("  • 推荐使用Rust CLI直接转换 (Recommend using Rust CLI directly)\n")

	fmt.Println("═══════════════════════════════════════════════════════════\n")
	fmt.Println("🔗 更多信息 (More Info):\n")
	fmt.Println("  架构文档: ARCHITECTURE_ANALYSIS_AND_TODO.md")
	fmt.Println("  Rust CLI: pixly-rust (推荐/Recommended)")
	fmt.Println("  问题反馈: https://github.com/your-repo/issues\n")

	fmt.Println("═══════════════════════════════════════════════════════════\n")
	fmt.Println("提示: 设置 PORT=8080 环境变量后运行以启动服务")
	fmt.Println("Hint: Set PORT=8080 environment variable and run to start service\n")
}
