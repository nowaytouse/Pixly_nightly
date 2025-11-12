// dynamic2mov - 动态图片转高效视频MOV工具
// 版本: v2.0.0 (GPU 加速版)
// 作者: AI Assistant
// 功能: 将动态图片（GIF/WebP/APNG）转换为高效的AV1或H.265编码MOV视频
// 新增: GPU 硬件加速支持 (VideoToolbox/NVENC/QuickSync)

package main

import (
	"bufio"
	"context"
	"flag"
	"fmt"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"sync"
	"syscall"
	"time"

	"pixly/pkg/i18n"
	"pixly/utils"

	"github.com/karrick/godirwalk"
)

const (
	version = "1.0.0"
	author  = "AI Assistant"
)

var (
	logger     *log.Logger
	globalCtx  context.Context
	cancelFunc context.CancelFunc
	stats      *utils.SharedStats
	procSem    chan struct{}
	fdSem      chan struct{}
)

type Options struct {
	Workers           int
	InputDir          string
	OutputDir         string
	SkipExist         bool
	DryRun            bool
	TimeoutSeconds    int
	Retries           int
	MaxMemory         int64
	MaxFileSize       int64
	EnableHealthCheck bool
	PreferredCodec    string // "av1" 或 "h265" 或 "auto"
	OutputFormat      string // "mov" 或 "mp4"
	StrictMode        bool
	AllowTolerance    float64
	// 🔥 新增：GPU 加速参数
	EnableGPU     bool   // 启用 GPU 加速
	GPUPreset     string // GPU 预设 (fast/medium/slow)
	ShowGPUInfo   bool   // 显示 GPU 信息并退出
	ForceSoftware bool   // 强制使用软件编码
	// 🌍 国际化参数
	Lang string // 语言 (zh/en, 默认: zh)
}

func init() {
	logger = utils.SetupLogging("dynamic2mov.log")
	stats = utils.NewSharedStats()
	utils.SetupSignalHandlingWithCallback(logger, printStatistics)
}

func parseFlags() Options {
	var opts Options

	flag.StringVar(&opts.InputDir, "dir", "", "📂 输入目录路径（必需）")
	flag.StringVar(&opts.OutputDir, "output", "", "📁 输出目录路径（默认为输入目录）")
	flag.IntVar(&opts.Workers, "workers", 0, "⚡ 工作线程数 (0=自动检测)")
	flag.BoolVar(&opts.SkipExist, "skip-exist", false, "⏭️ 跳过已存在的文件")
	flag.BoolVar(&opts.DryRun, "dry-run", false, "🔍 试运行模式")
	flag.IntVar(&opts.TimeoutSeconds, "timeout", 600, "⏰ 单个文件处理超时时间（秒）")
	flag.IntVar(&opts.Retries, "retries", 2, "🔄 转换失败重试次数")
	flag.Int64Var(&opts.MaxMemory, "max-memory", 0, "💾 最大内存使用量（字节，0=无限制）")
	flag.Int64Var(&opts.MaxFileSize, "max-file-size", 500*1024*1024, "📏 最大文件大小（字节）")
	flag.BoolVar(&opts.EnableHealthCheck, "health-check", true, "🏥 启用健康检查")
	flag.StringVar(&opts.PreferredCodec, "codec", "auto", "🎬 编码器选择 (av1/h265/auto)")
	flag.StringVar(&opts.OutputFormat, "format", "mov", "📦 输出格式 (mov/mp4)")
	flag.BoolVar(&opts.StrictMode, "strict", true, "🔒 启用严格的转换后验证 (默认开启)")
	flag.Float64Var(&opts.AllowTolerance, "tolerance", 0.1, "📏 验证时允许的像素差异百分比")

	// 🔥 新增：GPU 加速参数
	flag.BoolVar(&opts.EnableGPU, "enable-gpu", true, "⚡ 启用 GPU 硬件加速 (默认开启，适用于动画转视频)")
	flag.StringVar(&opts.GPUPreset, "gpu-preset", "medium", "🎬 GPU 预设 (fast/medium/slow)")
	flag.BoolVar(&opts.ShowGPUInfo, "gpu-info", false, "🔍 显示 GPU 信息并退出")
	flag.BoolVar(&opts.ForceSoftware, "force-software", false, "🖥️ 强制使用软件编码（禁用 GPU）")

	// 🌍 国际化参数
	flag.StringVar(&opts.Lang, "lang", "zh", i18n.T("help.lang"))

	flag.Parse()

	// 🌍 设置语言
	if opts.Lang != "" {
		i18n.SetLang(opts.Lang)
	}

	// 交互模式：如果没有提供目录，提示用户输入
	opts.InputDir = utils.PromptForDirectory(opts.InputDir)
	if opts.InputDir == "" {
		logger.Fatal("❌ 错误: 必须指定输入目录")
	}
	if opts.OutputDir == "" {
		opts.OutputDir = opts.InputDir
	}
	if _, err := os.Stat(opts.InputDir); os.IsNotExist(err) {
		logger.Fatalf("❌ 错误: 输入目录不存在: %s", opts.InputDir)
	}

	return opts
}

func checkDependencies() error {
	dependencies := []string{"ffmpeg", "exiftool"}
	for _, dep := range dependencies {
		if _, err := exec.LookPath(dep); err != nil {
			return fmt.Errorf("缺少依赖: %s", dep)
		}
	}
	logger.Println(i18n.T("dynamic2mov.deps_ok"))
	return nil
}

func configurePerformance(opts *Options) {
	cpuCount := runtime.NumCPU()
	if opts.Workers <= 0 {
		if cpuCount >= 16 {
			opts.Workers = cpuCount
		} else if cpuCount >= 8 {
			opts.Workers = cpuCount - 1
		} else if cpuCount >= 4 {
			opts.Workers = cpuCount
		} else {
			opts.Workers = 4
		}
	}
	if opts.Workers > 8 {
		opts.Workers = 8
	}
	procSem = make(chan struct{}, opts.Workers)
	fdSem = make(chan struct{}, 16)
	globalCtx, cancelFunc = context.WithCancel(context.Background())
	logger.Printf("%s: %d %s", i18n.T("dynamic2mov.perf_config"), opts.Workers, i18n.T("exec.workers"))
}

func scanCandidateFiles(inputDir string, opts Options) []string {
	var files []string
	err := godirwalk.Walk(inputDir, &godirwalk.Options{
		Callback: func(osPathname string, de *godirwalk.Dirent) error {
			if de.IsDir() {
				return nil
			}
			ext := strings.ToLower(filepath.Ext(osPathname))
			// 支持所有动态图片格式
			if ext != ".gif" && ext != ".webp" && ext != ".apng" && ext != ".png" {
				return nil
			}
			// 对于PNG，需要检查是否为APNG（动态PNG）
			if ext == ".png" {
				// 简化：假设所有PNG都可能是APNG，让ffmpeg自动处理
			}
			if info, err := os.Stat(osPathname); err == nil {
				if info.Size() > 0 && info.Size() <= opts.MaxFileSize {
					files = append(files, osPathname)
				}
			}
			return nil
		},
		ErrorCallback: func(osPathname string, err error) godirwalk.ErrorAction {
			logger.Printf("%s: %s - %v", i18n.T("dynamic2mov.scan_error"), osPathname, err)
			return godirwalk.SkipNode
		},
	})
	if err != nil {
		logger.Printf("❌ %s: %v", i18n.T("dynamic2mov.scan_error"), err)
	}
	return files
}

func processFileWithRetry(filePath string, fileInfo os.FileInfo, opts Options) {
	var lastErr error
	for attempt := 0; attempt <= opts.Retries; attempt++ {
		if attempt > 0 {
			logger.Printf("%s: %s (%s %d %s)", i18n.T("dynamic2mov.retry"), filepath.Base(filePath), i18n.T("dynamic2mov.retry_count"), attempt, i18n.T("dynamic2mov.retry_suffix"))
			time.Sleep(time.Duration(attempt) * time.Second)
			stats.AddRetry()
		}
		err := processFileWithOpts(filePath, fileInfo, stats, opts)
		if err == nil {
			return
		}
		lastErr = err
		logger.Printf("%s: %s - %v", i18n.T("dynamic2mov.process_failed"), filepath.Base(filePath), err)
		stats.AddErrorType(utils.ClassifyError(err))
	}
	logger.Printf("%s: %s - %v", i18n.T("dynamic2mov.final_failed"), filepath.Base(filePath), lastErr)
	stats.AddFailed()
}

func processFileWithOpts(filePath string, fileInfo os.FileInfo, stats *utils.SharedStats, opts Options) error {
	startTime := time.Now()

	procSem <- struct{}{}
	defer func() { <-procSem }()

	if _, err := os.Stat(filePath); os.IsNotExist(err) {
		return fmt.Errorf("文件不存在: %s", filePath)
	}

	// GIF转AV1编码MOV
	conversionMode, outputPath, errorMsg, err := processFileByType(filePath, opts)
	processingTime := time.Since(startTime)

	processInfo := utils.SharedFileProcessInfo{
		FilePath:       filePath,
		FileSize:       fileInfo.Size(),
		FileType:       filepath.Ext(filePath),
		ProcessingTime: processingTime,
		ConversionMode: conversionMode,
		Success:        err == nil,
		ErrorMsg:       errorMsg,
		StartTime:      startTime,
		EndTime:        time.Now(),
		ErrorType:      utils.ClassifyError(err),
	}

	if err != nil {
		stats.AddFailed()
		processInfo.ErrorMsg = err.Error()
	} else {
		// 获取输出文件大小
		outputSize := int64(0)
		if outInfo, err := os.Stat(outputPath); err == nil {
			outputSize = outInfo.Size()
		}
		stats.AddProcessed(fileInfo.Size(), outputSize)
		stats.AddByExt(filepath.Ext(filePath))
	}
	stats.AddDetailedLog(processInfo)
	return err
}

func processFileByType(filePath string, opts Options) (string, string, string, error) {
	// 动态图片转AV1/H.265编码视频的实际转换逻辑（v2.4.0 + 统一验证）
	outputExt := "." + opts.OutputFormat
	outputPath := strings.TrimSuffix(filePath, filepath.Ext(filePath)) + outputExt

	// ✅ 步骤1: 捕获源文件的文件系统元数据（在转换之前）
	srcInfo, _ := os.Stat(filePath)
	var creationTime time.Time
	if srcInfo != nil {
		if stat, ok := srcInfo.Sys().(*syscall.Stat_t); ok {
			creationTime = time.Unix(stat.Birthtimespec.Sec, stat.Birthtimespec.Nsec)
		}
	}

	// ✅ 步骤2: 智能选择编码器（根据输出格式）
	// 🔥 新增：优先使用 GPU 加速
	var conversionMode string
	var args []string
	var useGPU bool

	// 检查是否应该使用 GPU
	if opts.EnableGPU && !opts.ForceSoftware {
		gpu := detectGPU()
		if gpu.Available {
			// 使用 GPU 编码
			useGPU = true
			conversionMode = fmt.Sprintf("动图转视频(GPU-%s)", gpu.Type)
			args = buildGPUArgs(filePath, outputPath, gpu, opts)
			logger.Printf("%s: %s (%s %.1fx)", i18n.T("dynamic2mov.gpu_using"), gpu.Type, i18n.T("dynamic2mov.gpu_speedup"), gpu.SpeedBoost)
		}
	}

	// 如果 GPU 不可用或被禁用，使用软件编码
	if !useGPU {
		codec, codecName := selectBestCodec(opts.PreferredCodec, opts.OutputFormat)

		if codec == "av1" {
			conversionMode = fmt.Sprintf("动图转AV1编码%s", strings.ToUpper(opts.OutputFormat))

			if codecName == "libaom-av1" {
				args = []string{
					"-i", filePath,
					"-c:v", "libaom-av1",
					"-crf", "28",
					"-cpu-used", "4",
					"-row-mt", "1",
					"-tiles", "2x2",
					"-pix_fmt", "yuv420p",
					"-map_metadata", "0",
					"-f", opts.OutputFormat,
					"-y", outputPath,
				}
			} else {
				args = []string{
					"-i", filePath,
					"-c:v", "libsvtav1",
					"-crf", "28",
					"-preset", "6",
					"-pix_fmt", "yuv420p",
					"-map_metadata", "0",
					"-f", opts.OutputFormat,
					"-y", outputPath,
				}
			}
		} else {
			conversionMode = fmt.Sprintf("动图转H.265编码%s", strings.ToUpper(opts.OutputFormat))

			baseArgs := []string{
				"-hide_banner", "-loglevel", "error",
				"-i", filePath,
				"-c:v", "libx265",
				"-crf", "28",
				"-preset", "faster",
				"-pix_fmt", "yuv420p",
				"-vf", "scale=trunc(iw/2)*2:trunc(ih/2)*2",
				"-map_metadata", "0",
				"-max_muxing_queue_size", "1024",
			}

			if opts.OutputFormat == "mov" {
				args = append(baseArgs, "-movflags", "use_metadata_tags")
			}

			args = append(args,
				"-f", opts.OutputFormat,
				"-y", outputPath,
			)
		}
	} // 结束 if !useGPU

	ctx, cancel := context.WithTimeout(globalCtx, time.Duration(opts.TimeoutSeconds)*time.Second)
	defer cancel()

	cmd := exec.CommandContext(ctx, "ffmpeg", args...)
	if output, err := cmd.CombinedOutput(); err != nil {
		return conversionMode, "", "CONVERSION_FAILED", fmt.Errorf("ffmpeg编码失败: %v\n输出: %s", err, string(output))
	}

	// 根据是否使用 GPU 显示不同的成功消息
	if useGPU {
		logger.Printf("%s: %s", i18n.T("dynamic2mov.success_gpu"), filepath.Base(outputPath))
	} else {
		logger.Printf("%s: %s", i18n.T("dynamic2mov.success_software"), filepath.Base(outputPath))
	}

	// ✅ 步骤3: 复制EXIF元数据（会改变文件修改时间）
	if err := utils.CopyMetadata(filePath, outputPath); err != nil {
		logger.Printf("%s: %s -> %s: %v",
			i18n.T("dynamic2mov.exif_copy_failed"), filepath.Base(filePath), filepath.Base(outputPath), err)
	} else {
		logger.Printf("%s: %s", i18n.T("dynamic2mov.exif_copy_success"), filepath.Base(outputPath))
	}

	// ✅ 步骤4: 恢复文件系统元数据（在exiftool之后）
	if srcInfo != nil {
		// 4.1 恢复Finder标签和注释
		if err := utils.CopyFinderMetadata(filePath, outputPath); err != nil {
			logger.Printf("%s %s: %v", i18n.T("dynamic2mov.finder_copy_failed"), filepath.Base(outputPath), err)
		} else {
			logger.Printf("%s: %s", i18n.T("dynamic2mov.finder_copy_success"), filepath.Base(outputPath))
		}

		// 4.2 恢复修改时间和创建时间（使用touch统一设置）
		if !creationTime.IsZero() {
			timeStr := creationTime.Format("200601021504.05")
			touchCmd := exec.Command("touch", "-t", timeStr, outputPath)
			if err := touchCmd.Run(); err != nil {
				logger.Printf("%s %s: %v", i18n.T("dynamic2mov.time_restore_failed"), filepath.Base(outputPath), err)
			} else {
				logger.Printf("%s: %s (%s: %s)",
					i18n.T("dynamic2mov.fs_metadata_ok"), filepath.Base(outputPath), i18n.T("metadata.preserved"), creationTime.Format("2006-01-02 15:04:05"))
			}
		}
	}

	// ✅ 基本验证：检查输出文件是否生成
	if opts.StrictMode {
		if info, err := os.Stat(outputPath); err != nil {
			return conversionMode, "", "VALIDATION_ERROR", fmt.Errorf("输出文件未生成: %w", err)
		} else if info.Size() == 0 {
			os.Remove(outputPath)
			return conversionMode, "", "VALIDATION_ERROR", fmt.Errorf("输出文件为空")
		}
		logger.Printf("%s: %s (%s)", i18n.T("dynamic2mov.validation_ok"), filepath.Base(outputPath), i18n.T("dynamic2mov.mov_generated"))
	}

	// ✅ 原地删除：转换成功后删除原始文件
	if err := os.Remove(filePath); err != nil {
		logger.Printf("%s %s: %v", i18n.T("dynamic2mov.delete_original_failed"), filepath.Base(filePath), err)
	} else {
		logger.Printf("%s: %s", i18n.T("dynamic2mov.delete_original_ok"), filepath.Base(filePath))
	}

	return conversionMode, outputPath, "", nil
}

// copyFinderMetadata 复制Finder标签和注释
func copyFinderMetadata(src, dst string) error {
	// 复制Finder标签
	cmd := exec.Command("xattr", "-p", "com.apple.metadata:_kMDItemUserTags", src)
	if output, err := cmd.CombinedOutput(); err == nil && len(output) > 0 {
		exec.Command("xattr", "-w", "com.apple.metadata:_kMDItemUserTags", string(output), dst).Run()
	}

	// 复制Finder注释
	cmd = exec.Command("xattr", "-p", "com.apple.metadata:kMDItemFinderComment", src)
	if output, err := cmd.CombinedOutput(); err == nil && len(output) > 0 {
		exec.Command("xattr", "-w", "com.apple.metadata:kMDItemFinderComment", string(output), dst).Run()
	}

	// 复制其他扩展属性
	cmd = exec.Command("xattr", src)
	if output, err := cmd.CombinedOutput(); err == nil {
		attrs := strings.Split(strings.TrimSpace(string(output)), "\n")
		for _, attr := range attrs {
			if attr != "" && !strings.Contains(attr, "com.apple.metadata:_kMDItemUserTags") &&
				!strings.Contains(attr, "com.apple.metadata:kMDItemFinderComment") {
				cmd = exec.Command("xattr", "-p", attr, src)
				if value, err := cmd.CombinedOutput(); err == nil && len(value) > 0 {
					exec.Command("xattr", "-w", attr, string(value), dst).Run()
				}
			}
		}
	}

	return nil
}

func printStatistics() {
	stats.RLock()
	defer stats.RUnlock()
	totalProcessed := stats.ImagesProcessed + stats.ImagesFailed + stats.ImagesSkipped
	if totalProcessed == 0 {
		return
	}
	successRate := float64(stats.ImagesProcessed) / float64(totalProcessed) * 100
	logger.Println("")
	logger.Println(i18n.T("stats.title"))
	logger.Printf("  • %s: %d", i18n.T("stats.total"), totalProcessed)
	logger.Printf("  • %s: %d", i18n.T("stats.success"), stats.ImagesProcessed)
	logger.Printf("  • %s: %d", i18n.T("stats.failed"), stats.ImagesFailed)
	logger.Printf("  • %s: %d", i18n.T("stats.skipped"), stats.ImagesSkipped)
	logger.Printf("  • %s: %.1f%%", i18n.T("result.success_rate"), successRate)
	if stats.TotalBytesBefore > 0 {
		savingPercent := (1 - float64(stats.TotalBytesAfter)/float64(stats.TotalBytesBefore)) * 100
		logger.Printf("  • %s: %.1f%%", i18n.T("result.size_saved"), savingPercent)
	}
	logger.Printf("  • %s: %v", i18n.T("stats.time"), stats.GetElapsedTime())
	if stats.TotalRetries > 0 {
		logger.Printf("  • %s: %d", i18n.T("stats.total"), stats.TotalRetries)
	}
}

func main() {
	// 🎨 检测模式：无参数时启动交互模式
	if len(os.Args) == 1 {
		runInteractiveMode()
		return
	}

	// 📝 非交互模式：命令行参数
	runNonInteractiveMode()
}

// runNonInteractiveMode 非交互模式入口
func runNonInteractiveMode() {
	logger.Printf("%s v%s", i18n.T("dynamic2mov.title"), version)
	logger.Printf("%s: %s", i18n.T("app.author"), author)
	logger.Print(i18n.T("app.init"))

	opts := parseFlags()

	// 🔥 新增：GPU 信息显示模式
	if opts.ShowGPUInfo {
		showGPUInfo()
		return
	}

	logger.Println(i18n.T("dynamic2mov.check_deps"))
	if err := checkDependencies(); err != nil {
		logger.Fatalf("❌ %v", err)
	}

	// 🔥 新增：检测 GPU 并显示状态
	gpu := detectGPU()
	if opts.EnableGPU && !opts.ForceSoftware && gpu.Available {
		logger.Printf("%s (%s, %s %.1fx)", i18n.T("gpu.enabled"), gpu.Type, i18n.T("dynamic2mov.gpu_speedup"), gpu.SpeedBoost)
	} else if opts.ForceSoftware {
		logger.Println(i18n.T("gpu.disabled"))
	} else if !gpu.Available {
		logger.Println(i18n.T("gpu.not_available"))
	}

	configurePerformance(&opts)
	logger.Println(i18n.T("dynamic2mov.scan_gif"))
	files := scanCandidateFiles(opts.InputDir, opts)
	logger.Printf("%s %d %s", i18n.T("dynamic2mov.found_files"), len(files), i18n.T("file.total"))

	if len(files) == 0 {
		logger.Println(i18n.T("dynamic2mov.no_files"))
		return
	}

	if opts.DryRun {
		logger.Println(i18n.T("dynamic2mov.dry_run"))
		for i, file := range files {
			logger.Printf("  %d. %s", i+1, file)
		}
		return
	}

	logger.Printf("%s %d %s %d %s", i18n.T("dynamic2mov.start_processing"), len(files), i18n.T("dynamic2mov.files_with_workers"), opts.Workers, i18n.T("dynamic2mov.workers_suffix"))
	var wg sync.WaitGroup
	for _, file := range files {
		wg.Add(1)
		go func(filePath string) {
			defer wg.Done()
			if info, err := os.Stat(filePath); err == nil {
				processFileWithRetry(filePath, info, opts)
			}
		}(file)
	}
	wg.Wait()
	printStatistics()
	logger.Println(i18n.T("success.all_done"))
}

// runNonInteractiveMode_WithOpts 使用指定选项运行
func runNonInteractiveMode_WithOpts(opts Options) {
	logger.Printf("%s v%s", i18n.T("dynamic2mov.title"), version)
	logger.Println(i18n.T("dynamic2mov.check_deps"))
	if err := checkDependencies(); err != nil {
		logger.Fatalf("❌ %v", err)
	}

	configurePerformance(&opts)
	logger.Println(i18n.T("dynamic2mov.scan_dynamic"))
	files := scanCandidateFiles(opts.InputDir, opts)
	logger.Printf("%s %d %s", i18n.T("dynamic2mov.found_files"), len(files), i18n.T("file.total"))

	if len(files) == 0 {
		logger.Println(i18n.T("dynamic2mov.no_files"))
		return
	}

	logger.Printf("%s %d %s %d %s", i18n.T("dynamic2mov.start_processing"), len(files), i18n.T("dynamic2mov.files_with_workers"), opts.Workers, i18n.T("dynamic2mov.workers_suffix"))
	var wg sync.WaitGroup
	for _, file := range files {
		wg.Add(1)
		go func(filePath string) {
			defer wg.Done()
			if info, err := os.Stat(filePath); err == nil {
				processFileWithRetry(filePath, info, opts)
			}
		}(file)
	}
	wg.Wait()
	printStatistics()
	logger.Println(i18n.T("success.all_done"))
}

// runInteractiveMode 交互模式入口
func runInteractiveMode() {
	// 1. 显示横幅
	fmt.Println("╔═══════════════════════════════════════════════════════════════╗")
	fmt.Println("║                                                               ║")
	fmt.Println("║   🎬 dynamic2mov v1.0.0 - 动态图片转视频工具                ║")
	fmt.Println("║                                                               ║")
	fmt.Println("║   输入: GIF / WebP（动图）/ APNG                             ║")
	fmt.Println("║   输出: MOV/MP4视频（AV1或H.265编码）                        ║")
	fmt.Println("║   编码: AV1(MP4)最高压缩 / H.265(MOV)高兼容                 ║")
	fmt.Println("║   元数据: EXIF + 文件系统时间戳 + Finder标签 100%保留       ║")
	fmt.Println("║                                                               ║")
	fmt.Println("╚═══════════════════════════════════════════════════════════════╝")
	fmt.Println("")

	// 2. 提示输入目录
	targetDir, err := promptForDirectory()
	if err != nil {
		fmt.Printf("❌ 错误: %v\n", err)
		os.Exit(1)
	}

	// 3. 安全检查
	if err := performSafetyCheck(targetDir); err != nil {
		fmt.Printf("❌ 安全检查失败: %v\n", err)
		os.Exit(1)
	}

	// 4. 设置选项并开始处理
	opts := Options{
		Workers:           4,
		InputDir:          targetDir,
		OutputDir:         targetDir,
		SkipExist:         false,
		DryRun:            false,
		TimeoutSeconds:    600,
		Retries:           2,
		MaxMemory:         0,
		MaxFileSize:       500 * 1024 * 1024,
		EnableHealthCheck: true,
		PreferredCodec:    "auto", // 自动选择
		OutputFormat:      "mov",  // 默认MOV格式
	}

	fmt.Println("🔄 开始处理...")
	fmt.Println("")

	// 开始主处理流程
	runNonInteractiveMode_WithOpts(opts)
}

// promptForDirectory 提示用户输入目录
func promptForDirectory() (string, error) {
	fmt.Println("📁 请拖入要处理的文件夹，然后按回车键：")
	fmt.Println("   （或直接输入路径）")
	fmt.Print("\n路径: ")

	reader := bufio.NewReader(os.Stdin)
	input, err := reader.ReadString('\n')
	if err != nil {
		return "", fmt.Errorf("读取输入失败: %v", err)
	}

	// 清理并反转义路径
	path := strings.TrimSpace(input)
	path = unescapeShellPath(path)

	if path == "" {
		return "", fmt.Errorf("路径不能为空")
	}

	return path, nil
}

// performSafetyCheck 执行安全检查
func performSafetyCheck(targetPath string) error {
	fmt.Println("")
	fmt.Println("🔍 正在执行安全检查...")
	fmt.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")

	// 1. 检查路径是否存在
	absPath, err := filepath.Abs(targetPath)
	if err != nil {
		return fmt.Errorf("无法解析路径: %v", err)
	}

	info, err := os.Stat(absPath)
	if err != nil {
		if os.IsNotExist(err) {
			return fmt.Errorf("路径不存在: %s", absPath)
		}
		return fmt.Errorf("无法访问路径: %v", err)
	}

	if !info.IsDir() {
		return fmt.Errorf("路径不是文件夹: %s", absPath)
	}

	fmt.Printf("  ✅ 路径存在: %s\n", absPath)

	// 2. 检查是否为系统关键目录
	if isCriticalSystemPath(absPath) {
		return fmt.Errorf("禁止访问系统关键目录: %s\n建议使用: ~/Documents, ~/Desktop, ~/Downloads", absPath)
	}

	fmt.Printf("  ✅ 路径安全: 非系统目录\n")

	// 3. 检查读写权限
	testFile := filepath.Join(absPath, ".pixly_permission_test")
	if file, err := os.Create(testFile); err != nil {
		return fmt.Errorf("目录没有写入权限: %v", err)
	} else {
		file.Close()
		os.Remove(testFile)
		fmt.Printf("  ✅ 权限验证: 可读可写\n")
	}

	// 4. 检查磁盘空间
	if freeSpace, totalSpace, err := getDiskSpace(absPath); err == nil {
		freeGB := float64(freeSpace) / 1024 / 1024 / 1024
		totalGB := float64(totalSpace) / 1024 / 1024 / 1024
		ratio := float64(freeSpace) / float64(totalSpace) * 100

		fmt.Printf("  💾 磁盘空间: %.1fGB / %.1fGB (%.1f%% 可用)\n", freeGB, totalGB, ratio)

		if ratio < 10 {
			return fmt.Errorf("磁盘空间不足（剩余%.1f%%），建议至少保留10%%空间", ratio)
		} else if ratio < 20 {
			fmt.Printf("  ⚠️  磁盘空间较少（剩余%.1f%%），建议谨慎处理\n", ratio)
		}
	}

	// 5. 检查是否为敏感目录
	if isSensitiveDirectory(absPath) {
		fmt.Printf("  ⚠️  敏感目录警告: %s\n", absPath)
		fmt.Print("\n  是否继续处理此目录？(输入 yes 确认): ")

		reader := bufio.NewReader(os.Stdin)
		confirm, _ := reader.ReadString('\n')
		confirm = strings.TrimSpace(strings.ToLower(confirm))

		if confirm != "yes" && confirm != "y" {
			return fmt.Errorf("用户取消操作")
		}
	}

	fmt.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	fmt.Println("✅ 安全检查通过！")
	fmt.Println("")

	return nil
}

// isCriticalSystemPath 检查是否为系统关键目录
func isCriticalSystemPath(path string) bool {
	criticalPaths := []string{
		"/System",
		"/Library/System",
		"/private",
		"/usr/bin",
		"/usr/sbin",
		"/bin",
		"/sbin",
		"/var/root",
		"/etc",
		"/dev",
		"/proc",
		"/Applications/Utilities",
		"/System/Library",
	}

	for _, critical := range criticalPaths {
		if strings.HasPrefix(path, critical) {
			return true
		}
	}

	return false
}

// isSensitiveDirectory 检查是否为敏感目录
func isSensitiveDirectory(path string) bool {
	sensitivePaths := []string{
		"/Applications",
		"/Library",
		"/usr",
		"/var",
	}

	homeDir, _ := os.UserHomeDir()
	if homeDir != "" {
		sensitivePaths = append(sensitivePaths, homeDir)
	}

	for _, sensitive := range sensitivePaths {
		if path == sensitive {
			return true
		}
	}

	return false
}

// getDiskSpace 获取磁盘空间信息
func getDiskSpace(path string) (free, total uint64, err error) {
	var stat syscall.Statfs_t
	if err := syscall.Statfs(path, &stat); err != nil {
		return 0, 0, err
	}

	free = stat.Bavail * uint64(stat.Bsize)
	total = stat.Blocks * uint64(stat.Bsize)

	return free, total, nil
}

// unescapeShellPath 反转义Shell路径（macOS拖拽）
func unescapeShellPath(path string) string {
	path = strings.ReplaceAll(path, "\\ ", " ")
	path = strings.ReplaceAll(path, "\\!", "!")
	path = strings.ReplaceAll(path, "\\(", "(")
	path = strings.ReplaceAll(path, "\\)", ")")
	path = strings.ReplaceAll(path, "\\[", "[")
	path = strings.ReplaceAll(path, "\\]", "]")
	path = strings.ReplaceAll(path, "\\&", "&")
	path = strings.ReplaceAll(path, "\\$", "$")
	path = strings.Trim(path, "\"'")

	return path
}

// selectBestCodec 智能选择最佳编码器
func selectBestCodec(preferred, format string) (codec, codecName string) {
	// 如果输出格式是MP4，可以使用AV1
	if format == "mp4" {
		// MP4容器支持AV1编码
		if preferred == "av1" || preferred == "auto" {
			// 优先使用libaom-AV1（官方实现，质量最高，压缩比最好）
			if isCodecAvailable("libaom-av1") {
				logger.Printf("🎯 使用libaom-AV1编码器")
				logger.Printf("   ✨ 特性：官方AV1实现，最高质量，最佳压缩比（通常比H.265小30-50%%）")
				logger.Printf("   ⏱️  速度：编码较慢，适合追求最佳质量")
				logger.Printf("   📦 容器：MP4")
				return "av1", "libaom-av1"
			} else if isCodecAvailable("libsvtav1") {
				logger.Printf("🎯 使用SVT-AV1编码器")
				logger.Printf("   ✨ 特性：快速AV1实现，质量优秀，编码速度快")
				logger.Printf("   ⏱️  速度：比libaom快3-5倍")
				logger.Printf("   📦 容器：MP4")
				return "av1", "libsvtav1"
			} else if preferred == "av1" {
				// 用户明确要求AV1但系统不支持
				logger.Println("")
				logger.Println("❌ 错误：系统不支持AV1编码器")
				logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
				logger.Println("📋 检测结果：")
				logger.Println("   未找到 libaom-av1 或 libsvtav1 编码器")
				logger.Println("")
				logger.Println("💡 解决方案：")
				logger.Println("")
				logger.Println("   macOS (Homebrew):")
				logger.Println("      brew install ffmpeg")
				logger.Println("")
				logger.Println("   或者使用H.265编码（兼容性好）:")
				logger.Println("      -codec h265 -format mp4")
				logger.Println("      -codec auto -format mp4")
				logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
				logger.Println("")
				log.Fatal("⛔ 程序终止：缺少必要的编码器")
			}
		}

		// 如果AV1不可用，使用H.265
		if preferred == "h265" || preferred == "auto" {
			logger.Printf("🎯 使用H.265编码器")
			logger.Printf("   ✨ 特性：广泛支持，兼容性极好，质量优秀")
			logger.Printf("   ⏱️  速度：编码速度快，适合快速处理")
			logger.Printf("   📦 容器：MP4")
			return "h265", "libx265"
		}
	}

	// 如果输出格式是MOV，只能使用H.265
	if format == "mov" {
		if preferred == "av1" {
			// 用户明确指定AV1编码但使用MOV容器，这是不兼容的配置
			logger.Println("")
			logger.Println("❌ 错误：MOV容器不支持AV1编码")
			logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
			logger.Println("📋 原因：")
			logger.Println("   MOV是QuickTime容器格式，目前不支持AV1视频编码")
			logger.Println("")
			logger.Println("💡 解决方案（选择其中一个）：")
			logger.Println("")
			logger.Println("   方案1️⃣ : 使用MP4容器 + AV1编码（推荐）")
			logger.Println("   命令示例：")
			logger.Println("      -codec av1 -format mp4")
			logger.Println("")
			logger.Println("   方案2️⃣ : 使用MOV容器 + H.265编码")
			logger.Println("   命令示例：")
			logger.Println("      -codec h265 -format mov")
			logger.Println("      或使用 -codec auto -format mov")
			logger.Println("")
			logger.Println("   方案3️⃣ : 使用自动模式（推荐新手）")
			logger.Println("   命令示例：")
			logger.Println("      -codec auto -format mp4")
			logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
			logger.Println("")
			log.Fatal("⛔ 程序终止：配置不兼容")
		}
		logger.Printf("🎯 使用H.265编码器")
		logger.Printf("   ✨ 特性：MOV容器标准编码，macOS原生支持，兼容性最佳")
		logger.Printf("   ⏱️  速度：编码速度快")
		logger.Printf("   📦 容器：MOV (QuickTime)")
		logger.Printf("   💡 提示：如需更好的压缩比，考虑使用 -format mp4 -codec av1")
		return "h265", "libx265"
	}

	// 默认H.265
	logger.Printf("🎯 使用H.265编码器（默认）")
	logger.Printf("   ✨ 特性：通用编码器，兼容性好")
	return "h265", "libx265"
}

// isCodecAvailable 检查编码器是否可用
func isCodecAvailable(codecName string) bool {
	cmd := exec.Command("ffmpeg", "-codecs")
	output, err := cmd.CombinedOutput()
	if err != nil {
		return false
	}

	// 检查输出中是否包含编码器名称
	return strings.Contains(string(output), codecName)
}
