// cmd/pixly-cli/main.go - PIXLY Unified CLI/TUI
//
// 功能：
// - 统一的 CLI/TUI 实现
// - 替代所有独立工具
// - 支持智能模式和手动模式
//
// 版本: v3.0.0

package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	
	"pixly/cmd/pixly-cli/ui"
	"pixly/pkg/converter"
	"pixly/pkg/metadata"
	"pixly/pkg/system"

	"github.com/spf13/cobra"
)

var (
	// 全局变量
	rustConverter  *converter.RustConverter
	smartConverter *converter.SmartConverter
	verbose        bool
	quiet          bool
)

// rootCmd 根命令
var rootCmd = &cobra.Command{
	Use:   "pixly",
	Short: "PIXLY - Modern Image Converter (GO/Rust Architecture)",
	Long:  "", // Will be set dynamically based on language
	PersistentPreRun: func(cmd *cobra.Command, args []string) {
		// 处理语言参数
		if langFlag, _ := cmd.Flags().GetString("lang"); langFlag != "" {
			ui.SetLanguage(ui.Language(langFlag))
		}

		// 设置动态 Long 描述
		cmd.Long = ui.T("banner")

		// 初始化 Rust 转换器
		rustConverter = converter.NewRustConverter()
		if !rustConverter.IsAvailable() {
			log.Println("[WARNING] Rust converter not available")
			log.Println("[HINT] Ensure Rust service is running: ./start-rust-service.sh")
		} else {
			if verbose {
				log.Printf("[INFO] Rust converter initialized: %s\n", rustConverter.Version())
			}
		}

		// 初始化智能转换器
		smartConverter = converter.NewSmartConverter(rustConverter)
		if !smartConverter.IsAvailable() && verbose {
			log.Println("[WARNING] AI Service not available (Smart mode will be disabled)")
			log.Println("[HINT] Start GO AI Service: ./quick_start_go_core.sh")
		}
	},
}

func init() {
	// 初始化国际化
	ui.Init()

	// 全局标志
	rootCmd.PersistentFlags().BoolVarP(&verbose, "verbose", "v", false, "Verbose output")
	rootCmd.PersistentFlags().BoolVarP(&quiet, "quiet", "q", false, "Quiet mode (no progress)")
	rootCmd.PersistentFlags().StringP("lang", "l", "", "Language (en, zh_CN, zh_TW, ja_JP)")

	// 添加子命令
	rootCmd.AddCommand(convertCmd)
	rootCmd.AddCommand(versionCmd)
	rootCmd.AddCommand(healthCmd)
}

// convertCmd 转换命令
var convertCmd = &cobra.Command{
	Use:   "convert [input] [output]",
	Short: "Convert image to specified format",
	Long: `Convert image file to various formats

Supported formats: AVIF, JXL, WebP, PNG, JPEG/JPG, GIF, HEIC

Examples:
  # Smart mode (AI-driven parameters)
  pixly convert input.jpg output.avif --smart

  # Manual mode (explicit parameters)
  pixly convert input.jpg output.avif --quality 90 --speed 4

  # Same-format re-encoding (optimization)
  pixly convert input.jpg output.jpg --smart --preserve-metadata

  # Auto-detect format from filename
  pixly convert input.png output.avif --smart

  # Batch conversion
  pixly convert *.jpg --format avif --output-dir ./converted

  # Preserve metadata
  pixly convert input.jpg output.avif --preserve-metadata
`,
	Args: cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		// 获取参数
		inputPath := args[0]
		outputPath := ""
		if len(args) > 1 {
			outputPath = args[1]
		}

		// 获取标志
		format, _ := cmd.Flags().GetString("format")
		smartMode, _ := cmd.Flags().GetBool("smart")
		quality, _ := cmd.Flags().GetInt("quality")
		speed, _ := cmd.Flags().GetInt("speed")
		preserveMetadata, _ := cmd.Flags().GetBool("preserve-metadata")

		// 🔥 格式处理逻辑
		// 1. 如果指定了输出文件，从输出文件推断格式
		if outputPath != "" && format == "" {
			format = detectFormatFromFilename(outputPath)
			if verbose {
				log.Printf("[INFO] Format auto-detected from output filename: %s", format)
			}
		}

		// 2. 如果没有输出文件且没有指定格式，使用输入文件格式（同格式re-encoding）
		if outputPath == "" && format == "" {
			format = detectFormatFromFilename(inputPath)
			if verbose {
				log.Printf("[INFO] Same-format re-encoding: %s → %s", format, format)
			}
		}

		// 3. 如果还是没有格式，使用默认值 (avif)
		if format == "" {
			format = "avif"
			log.Println("[INFO] Using default format: avif")
		}

		// 4. 自动生成输出路径（如果需要）
		if outputPath == "" {
			outputPath = inferOutputPath(inputPath, format)
		}

		// 创建进度条
		progressMode := system.ProgressModeCLI
		if quiet {
			progressMode = system.ProgressModeSilent
		}

		progress := system.NewProgressBar(system.ProgressConfig{
			Mode:        progressMode,
			Total:       100,
			Description: fmt.Sprintf("Converting %s...", inputPath),
		})

		// 执行转换
		var result *converter.ConversionResult
		var err error

		if smartMode {
			// 智能模式
			progress.SetDescription("🧠 AI prediction...")
			progress.SetCurrent(10)

			// TODO: 获取图像尺寸和文件大小
			result, err = smartConverter.Convert(inputPath, outputPath, format, 0, 0, 0)
		} else {
			// 手动模式
			progress.SetDescription("🔧 Manual conversion...")
			progress.SetCurrent(10)

			req := converter.ConversionRequest{
				Input:   inputPath,
				Output:  outputPath,
				Format:  format,
				Quality: quality,
				Speed:   speed,
			}
			result, err = rustConverter.Convert(req)
		}

		if err != nil {
			log.Fatalf("[ERROR] Conversion failed: %v", err)
		}

		progress.SetCurrent(90)

		// 保留元数据
		if preserveMetadata {
			progress.SetDescription("💾 Preserving metadata...")
			if err := metadata.PreserveCompleteMetadata(inputPath, outputPath); err != nil {
				log.Printf("[WARNING] Failed to preserve metadata: %v", err)
			}
		}

		progress.Finish()

		// 显示结果
		fmt.Printf("\n✅ Conversion complete!\n")
		fmt.Printf("   Input:  %s\n", inputPath)
		fmt.Printf("   Output: %s (%s)\n", result.Output, formatSize(result.FileSize))
		fmt.Printf("   Time:   %dms\n", result.Duration)

		// 发送通知
		system.NotifySuccess("PIXLY", fmt.Sprintf("Converted %s successfully", inputPath))
	},
}

func init() {
	// convert 命令标志
	convertCmd.Flags().StringP("format", "f", "", "Output format (avif, jxl, webp, png, jpg, gif, heic). Empty = auto-detect from output filename")
	convertCmd.Flags().Bool("smart", false, "Use AI smart mode")
	convertCmd.Flags().Int("quality", 90, "Quality (1-100)")
	convertCmd.Flags().IntP("speed", "s", 4, "Speed (1-10, lower is slower but better)")
	convertCmd.Flags().Bool("preserve-metadata", true, "Preserve complete metadata")
	convertCmd.Flags().String("output-dir", "", "Output directory for batch conversion")
}

// versionCmd 版本命令
var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Show version information",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("PIXLY CLI v3.0.0")
		fmt.Println("GO + Rust Architecture")

		if rustConverter != nil && rustConverter.IsAvailable() {
			fmt.Printf("Rust Converter: %s\n", rustConverter.Version())
		}

		if smartConverter != nil && smartConverter.IsAvailable() {
			fmt.Println("AI Service: Available")
		}
	},
}

// healthCmd 健康检查命令
var healthCmd = &cobra.Command{
	Use:   "health",
	Short: "Check service health",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("╔══════════════════════════════════════════════════════════════╗")
		fmt.Printf("║  %s%-44s║\n", ui.T("health_check"), "")
		fmt.Println("╚══════════════════════════════════════════════════════════════╝")
		fmt.Println()

		// 检查 Rust Service
		if rustConverter.IsAvailable() {
			fmt.Printf("✅ %s:    %s (%s)\n", ui.T("rust_service"), ui.T("available"), rustConverter.Version())
		} else {
			fmt.Printf("❌ %s:    %s\n", ui.T("rust_service"), ui.T("not_available"))
			fmt.Printf("   → %s: ./start-rust-service.sh\n", ui.T("start_hint"))
		}

		// 检查 AI Service
		if smartConverter.IsAvailable() {
			fmt.Printf("✅ %s:   %s\n", ui.T("ai_service"), ui.T("available"))
		} else {
			fmt.Printf("❌ %s:   %s\n", ui.T("ai_service"), ui.T("not_available"))
			fmt.Printf("   → %s: ./quick_start_go_core.sh\n", ui.T("start_hint"))
		}

		fmt.Println()
	},
}

// 工具函数

// detectFormatFromFilename 从文件名检测格式
func detectFormatFromFilename(filename string) string {
	ext := strings.ToLower(filename[strings.LastIndex(filename, ".")+1:])
	
	// 标准化格式名称
	switch ext {
	case "jpg", "jpeg", "jpe", "jfif":
		return "jpg"
	case "jxl":
		return "jxl"
	case "avif":
		return "avif"
	case "webp":
		return "webp"
	case "png":
		return "png"
	case "gif":
		return "gif"
	case "heic", "heif":
		return "heic"
	default:
		return ""
	}
}

func inferOutputPath(input, format string) string {
	// 获取输入文件的扩展名位置
	lastDot := strings.LastIndex(input, ".")
	if lastDot == -1 {
		// 没有扩展名，直接添加
		return input + "." + format
	}
	
	// 替换扩展名
	return input[:lastDot] + "." + format
}

func formatSize(bytes int64) string {
	const unit = 1024
	if bytes < unit {
		return fmt.Sprintf("%d B", bytes)
	}
	div, exp := int64(unit), 0
	for n := bytes / unit; n >= unit; n /= unit {
		div *= unit
		exp++
	}
	return fmt.Sprintf("%.1f %cB", float64(bytes)/float64(div), "KMGTPE"[exp])
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		os.Exit(1)
	}
}
