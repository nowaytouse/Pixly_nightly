// 模块化重构版 - Video2MOV 工具
// 版本: v3.0.0 (模块化)
// 作者: AI Assistant
// 功能: 重新包装视频为 MOV 容器，保留所有元数据

package main

import (
	"context"
	"flag"
	"fmt"
	"log"
	"os"
	"os/exec"
	"runtime"

	"pixly/pkg/converter"
	"pixly/pkg/pipeline"
	"pixly/pkg/protection"
	"pixly/pkg/scanner"
	"pixly/utils"
)

const (
	version = "3.0.0"
	author  = "AI Assistant"
)

var logger *log.Logger

func init() {
	logger = utils.SetupLogging("video2mov.log")
	utils.SetupSignalHandlingWithCallback(logger, func() {
		logger.Println("📊 程序被中断")
	})
}

type Options struct {
	InputDir       string
	Workers        int
	MaxFileSize    int64
	SkipExist      bool
	DryRun         bool
	StrictMode     bool
	TimeoutSeconds int
	DeleteOriginal bool
}

func parseFlags() Options {
	var opts Options

	flag.StringVar(&opts.InputDir, "dir", "", "📂 输入目录路径（必需）")
	flag.IntVar(&opts.Workers, "workers", 0, "⚡ 工作线程数 (0=自动检测)")
	flag.Int64Var(&opts.MaxFileSize, "max-file-size", 2*1024*1024*1024, "📏 最大文件大小（字节，默认2GB）")
	flag.BoolVar(&opts.SkipExist, "skip-exist", false, "⏭️ 跳过已存在的文件")
	flag.BoolVar(&opts.DryRun, "dry-run", false, "🔍 试运行模式")
	flag.BoolVar(&opts.StrictMode, "strict", true, "🔒 启用严格验证（默认开启）")
	flag.IntVar(&opts.TimeoutSeconds, "timeout", 120, "⏰ 单个文件处理超时时间（秒）")
	flag.BoolVar(&opts.DeleteOriginal, "delete-original", true, "🗑️ 转换成功后删除原文件（默认开启）")

	flag.Parse()

	// 交互模式：如果没有提供目录，提示用户输入
	opts.InputDir = utils.PromptForDirectory(opts.InputDir)
	if opts.InputDir == "" {
		logger.Fatal("❌ 错误: 必须指定输入目录")
	}

	if _, err := os.Stat(opts.InputDir); os.IsNotExist(err) {
		logger.Fatalf("❌ 错误: 输入目录不存在: %s", opts.InputDir)
	}

	// 自动检测工作线程数
	if opts.Workers <= 0 {
		cpuCount := runtime.NumCPU()
		opts.Workers = cpuCount / 2 // 视频处理IO密集
		if opts.Workers < 2 {
			opts.Workers = 2
		}
		if opts.Workers > 4 {
			opts.Workers = 4 // 视频处理限制线程数
		}
	}

	return opts
}

func checkDependencies() error {
	dependencies := []string{"ffmpeg", "ffprobe", "exiftool"}
	for _, dep := range dependencies {
		if _, err := exec.LookPath(dep); err != nil {
			return fmt.Errorf("缺少依赖: %s", dep)
		}
	}
	logger.Println("✅ 所有系统依赖检查通过")
	return nil
}

func main() {
	logger.Printf("🎬 Video2MOV v%s", version)
	logger.Printf("✨ 作者: %s", author)
	logger.Printf("🔧 开始初始化...")

	// 步骤1: 解析配置
	opts := parseFlags()

	// 步骤2: 检查依赖
	logger.Println("🔍 检查系统依赖...")
	if err := checkDependencies(); err != nil {
		logger.Fatalf("❌ 系统依赖检查失败: %v", err)
	}

	// 步骤3: 创建扫描器
	logger.Println("📂 初始化文件扫描器...")
	fileScanner := scanner.NewFileScanner(scanner.ScanConfig{
		RootDir:    opts.InputDir,
		Extensions: []string{".mp4", ".avi", ".mkv", ".webm", ".m4v", ".flv", ".wmv", ".mpg", ".mpeg"},
		SkipHidden: true,
		SkipDirs:   []string{".git", ".svn", "node_modules"},
	})

	// 步骤4: 创建保护检查器
	logger.Println("📏 初始化文件大小检查器...")
	protector := protection.NewFileSizeChecker(protection.FileSizeConfig{
		MaxFileSize:   opts.MaxFileSize,
		WarnThreshold: 500 * 1024 * 1024, // 500MB 警告阈值
		AllowZeroSize: false,
	})

	// 步骤5: 创建转换器
	logger.Println("🎬 初始化 MOV 转换器...")
	movConverter := NewMOVConverter(converter.ConvertOptions{
		StrictMode:       opts.StrictMode,
		TimeoutSeconds:   opts.TimeoutSeconds,
		PreserveMetadata: true, // 必须保留元数据
		DeleteOriginal:   opts.DeleteOriginal,
		Retries:          3,
	})

	// 步骤6: 创建流水线
	logger.Println("🔧 初始化处理流水线...")
	pipe := pipeline.NewPipeline(fileScanner, protector, movConverter, logger)
	pipe.SetWorkers(opts.Workers)

	// 步骤7: 执行流水线
	logger.Printf("🚀 开始执行（%d 个工作线程）", opts.Workers)
	logger.Println("📊 视频重新包装不改变编码，速度较快...")
	ctx := context.Background()

	result, err := pipe.Execute(ctx, pipeline.PipelineOptions{
		Workers:      opts.Workers,
		SkipExisting: opts.SkipExist,
		DryRun:       opts.DryRun,
		ConvertOptions: converter.ConvertOptions{
			StrictMode:       opts.StrictMode,
			TimeoutSeconds:   opts.TimeoutSeconds,
			PreserveMetadata: true,
			DeleteOriginal:   opts.DeleteOriginal,
		},
	})

	if err != nil {
		logger.Fatalf("❌ 流水线执行失败: %v", err)
	}

	// 步骤8: 输出结果
	logger.Println("")
	logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	logger.Println("🎉 Video2MOV 处理完成")
	logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	logger.Printf("版本:     %s", version)
	logger.Printf("成功率:   %.1f%% (%d/%d)",
		float64(result.SuccessCount)/float64(result.ProcessedFiles)*100,
		result.SuccessCount, result.ProcessedFiles)

	if result.FailedCount > 0 {
		logger.Printf("⚠️  失败文件: %d", result.FailedCount)
		logger.Println("\n失败详情:")
		for _, r := range result.Results {
			if !r.Success {
				logger.Printf("  • %s: %v", r.InputPath, r.Error)
			}
		}
	}

	logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	logger.Println("✅ 所有任务已完成")
}
