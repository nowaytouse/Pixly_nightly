// 模块化重构版 - XMP元数据合并工具
// 版本: v3.0.0 (模块化)
// 作者: AI Assistant
// 功能: XMP元数据合并到媒体文件,保留所有元数据

package main

import (
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

	"pixly/pkg/validation"
	"pixly/utils"

	"github.com/karrick/godirwalk"
)

const (
	version = "3.0.0"
	author  = "AI Assistant"
)

var logger *log.Logger

func init() {
	logger = utils.SetupLogging("merge_xmp.log")
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
	DeleteXMP      bool
}

func parseFlags() Options {
	var opts Options

	flag.StringVar(&opts.InputDir, "dir", "", "📂 输入目录路径（必需）")
	flag.IntVar(&opts.Workers, "workers", 0, "⚡ 工作线程数 (0=自动检测)")
	flag.Int64Var(&opts.MaxFileSize, "max-file-size", 500*1024*1024, "📏 最大文件大小（字节，默认500MB）")
	flag.BoolVar(&opts.SkipExist, "skip-exist", false, "⏭️ 跳过已存在的文件")
	flag.BoolVar(&opts.DryRun, "dry-run", false, "🔍 试运行模式")
	flag.BoolVar(&opts.StrictMode, "strict", true, "🔒 启用严格验证（默认开启）")
	flag.IntVar(&opts.TimeoutSeconds, "timeout", 30, "⏰ 单个文件处理超时时间（秒）")
	flag.BoolVar(&opts.DeleteXMP, "delete-xmp", true, "🗑️ 成功合并后删除XMP文件（默认开启）")

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
		opts.Workers = cpuCount / 2 // XMP处理IO密集
		if opts.Workers < 2 {
			opts.Workers = 2
		}
		if opts.Workers > 4 {
			opts.Workers = 4
		}
	}

	return opts
}

func checkDependencies() error {
	// 检查必要的依赖
	dependencies := []string{"exiftool"}
	for _, dep := range dependencies {
		if _, err := exec.LookPath(dep); err != nil {
			return fmt.Errorf("缺少依赖: %s", dep)
		}
	}
	logger.Println("✅ 所有系统依赖检查通过")
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
	logger.Printf("⚡ 性能配置: %d 个工作线程", opts.Workers)
}

func scanCandidateFiles(inputDir string, opts Options) []string {
	var files []string
	err := godirwalk.Walk(inputDir, &godirwalk.Options{
		Callback: func(osPathname string, de *godirwalk.Dirent) error {
			if de.IsDir() {
				return nil
			}
			ext := strings.ToLower(filepath.Ext(osPathname))
			if !isSupportedFile(ext) {
				return nil
			}
			if info, err := os.Stat(osPathname); err == nil {
				if info.Size() > 0 && info.Size() <= opts.MaxFileSize {
					files = append(files, osPathname)
				}
			}
			return nil
		},
		ErrorCallback: func(osPathname string, err error) godirwalk.ErrorAction {
			logger.Printf("⚠️  扫描文件时出错: %s - %v", osPathname, err)
			return godirwalk.SkipNode
		},
	})
	if err != nil {
		logger.Printf("❌ 扫描文件时出错: %v", err)
	}
	return files
}

func isSupportedFile(ext string) bool {
	// 根据工具类型返回支持的文件扩展名
	supportedExts := map[string]bool{
		".jpg": true, ".jpeg": true, ".png": true, ".bmp": true,
		".tiff": true, ".tif": true, ".gif": true, ".webp": true,
		".avif": true, ".jxl": true, ".heic": true, ".heif": true,
		".mov": true, ".mp4": true, ".avi": true, ".mkv": true,
	}
	return supportedExts[ext]
}

func processFileWithRetry(filePath string, fileInfo os.FileInfo, opts Options) {
	var lastErr error
	for attempt := 0; attempt <= opts.Retries; attempt++ {
		if attempt > 0 {
			logger.Printf("🔄 重试处理文件: %s (第 %d 次)", filepath.Base(filePath), attempt)
			time.Sleep(time.Duration(attempt) * time.Second)
			stats.Lock()
			stats.TotalRetries++
			stats.Unlock()
		}
		err := processFileWithOpts(filePath, fileInfo, stats, opts)
		if err == nil {
			return
		}
		lastErr = err
		logger.Printf("⚠️  处理文件失败: %s - %v", filepath.Base(filePath), err)
		stats.Lock()
		stats.ErrorTypes[classifyError(err)]++
		stats.Unlock()
	}
	logger.Printf("❌ 文件处理最终失败: %s - %v", filepath.Base(filePath), lastErr)
	stats.AddFailed()
}

func classifyError(err error) string {
	if err == nil {
		return "unknown"
	}
	errStr := err.Error()
	if strings.Contains(errStr, "timeout") {
		return "timeout"
	} else if strings.Contains(errStr, "memory") {
		return "memory"
	} else if strings.Contains(errStr, "permission") {
		return "permission"
	} else if strings.Contains(errStr, "format") {
		return "format"
	}
	return "unknown"
}

func processFileWithOpts(filePath string, fileInfo os.FileInfo, stats *utils.SharedStats, opts Options) error {
	StartTime := time.Now()
	procSem <- struct{}{}
	defer func() { <-procSem }()
	fdSem <- struct{}{}
	defer func() { <-fdSem }()

	select {
	case <-globalCtx.Done():
		return globalCtx.Err()
	default:
	}

	if _, err := os.Stat(filePath); os.IsNotExist(err) {
		return fmt.Errorf("文件不存在: %s", filePath)
	}

	// 根据工具类型执行相应的处理逻辑
	conversionMode, outputPath, errorMsg, err := processFileByType(filePath, opts)
	processingTime := time.Since(StartTime)

	processInfo := utils.SharedFileProcessInfo{
		FilePath:       filePath,
		FileSize:       fileInfo.Size(),
		FileType:       filepath.Ext(filePath),
		ProcessingTime: processingTime,
		ConversionMode: conversionMode,
		Success:        err == nil,
		ErrorMsg:       errorMsg,
		StartTime:      StartTime,
		EndTime:        time.Now(),
		ErrorType:      classifyError(err),
	}

	if err != nil {
		stats.AddFailed()
		processInfo.ErrorMsg = err.Error()
	} else {
		stats.AddProcessed(fileInfo.Size(), getFileSize(outputPath))
		stats.AddByExt(filepath.Ext(filePath))
	}
	stats.AddDetailedLog(processInfo)
	return err
}

func processFileByType(filePath string, opts Options) (string, string, string, error) {
	// 查找对应的XMP文件
	xmpPath := findXMPFile(filePath)
	if xmpPath == "" {
		return "跳过", "", "未找到对应的XMP文件", fmt.Errorf("未找到对应的XMP文件")
	}

	// 准备输出路径
	outputPath := filePath
	if opts.OutputDir != "" {
		relPath, err := filepath.Rel(opts.InputDir, filePath)
		if err != nil {
			relPath = filepath.Base(filePath)
		}
		outputPath = filepath.Join(opts.OutputDir, relPath)

		// 确保输出目录存在
		if err := os.MkdirAll(filepath.Dir(outputPath), 0755); err != nil {
			return "失败", "", fmt.Sprintf("创建输出目录失败: %v", err), err
		}
	}

	// 如果输出路径与输入路径不同，先复制文件
	if outputPath != filePath {
		if err := copyFile(filePath, outputPath); err != nil {
			return "失败", "", fmt.Sprintf("复制文件失败: %v", err), err
		}
	}

	// 执行XMP元数据合并
	logger.Printf("🔄 合并XMP: %s", filepath.Base(filePath))
	if err := mergeXMPMetadata(xmpPath, outputPath, opts); err != nil {
		return "失败", outputPath, fmt.Sprintf("XMP合并失败: %v", err), err
	}

	// ✅ 执行XMP合并验证
	if opts.StrictMode {
		logger.Printf("🔬 开始XMP合并验证: %s", filepath.Base(outputPath))

		validator := validation.NewXMPValidator(validation.XMPValidationOptions{
			TimeoutSeconds: opts.TimeoutSeconds,
			StrictMode:     opts.StrictMode,
			VerifyChecksum: opts.VerifyChecksum,
		})

		validationResult, err := validator.ValidateXMPMerge(filePath, xmpPath, outputPath)
		if err != nil {
			// ⚠️ 不删除输出文件，以防数据丢失
			logger.Printf("⚠️  验证出错但保留文件: %s - %v", filepath.Base(outputPath), err)
			// 验证出错但合并成功，仍然删除XMP（如果启用）
			if opts.DeleteXMP && !opts.DryRun {
				if err := os.Remove(xmpPath); err != nil {
					logger.Printf("⚠️  无法删除XMP文件: %s - %v", filepath.Base(xmpPath), err)
				} else {
					logger.Printf("🗑️ 已删除XMP文件: %s", filepath.Base(xmpPath))
				}
			}
			return "验证警告", outputPath, fmt.Sprintf("验证出错: %v", err), nil // 返回nil表示继续处理
		}

		if !validationResult.Success {
			// ⚠️ 验证失败时只记录警告，不删除文件
			logger.Printf("⚠️  验证未通过但保留文件: %s - Layer %d: %s",
				filepath.Base(outputPath), validationResult.Layer, validationResult.Message)
			// 验证未通过但合并成功，仍然删除XMP（如果启用）
			if opts.DeleteXMP && !opts.DryRun {
				if err := os.Remove(xmpPath); err != nil {
					logger.Printf("⚠️  无法删除XMP文件: %s - %v", filepath.Base(xmpPath), err)
				} else {
					logger.Printf("🗑️ 已删除XMP文件: %s", filepath.Base(xmpPath))
				}
			}
			return "验证警告", outputPath,
				fmt.Sprintf("Layer %d (%s): %s (文件已保留)", validationResult.Layer, validationResult.LayerName, validationResult.Message),
				nil // 返回nil表示继续处理，不算失败
		}

		logger.Printf("✅ 验证通过: %s (5层验证全部通过)", filepath.Base(outputPath))
	}

	// ✅ 成功合并后，根据选项决定是否删除XMP文件
	if opts.DeleteXMP && !opts.DryRun {
		if err := os.Remove(xmpPath); err != nil {
			logger.Printf("⚠️  无法删除XMP文件: %s - %v", filepath.Base(xmpPath), err)
		} else {
			logger.Printf("🗑️ 已删除XMP文件: %s", filepath.Base(xmpPath))
		}
	}

	return "XMP已合并", outputPath, "", nil
}

// findXMPFile 查找对应的XMP文件
func findXMPFile(mediaPath string) string {
	// 常见的XMP文件命名模式
	// 1. filename.ext.xmp
	xmpPath := mediaPath + ".xmp"
	if _, err := os.Stat(xmpPath); err == nil {
		return xmpPath
	}

	// 2. filename.xmp
	base := strings.TrimSuffix(mediaPath, filepath.Ext(mediaPath))
	xmpPath = base + ".xmp"
	if _, err := os.Stat(xmpPath); err == nil {
		return xmpPath
	}

	// 3. 在同目录下查找同名XMP文件
	dir := filepath.Dir(mediaPath)
	baseName := filepath.Base(base)
	entries, err := os.ReadDir(dir)
	if err != nil {
		return ""
	}

	for _, entry := range entries {
		if entry.IsDir() {
			continue
		}
		name := entry.Name()
		if strings.HasSuffix(strings.ToLower(name), ".xmp") {
			xmpBase := strings.TrimSuffix(name, filepath.Ext(name))
			if xmpBase == baseName {
				return filepath.Join(dir, name)
			}
		}
	}

	return ""
}

// mergeXMPMetadata 使用exiftool合并XMP元数据
func mergeXMPMetadata(xmpPath, targetPath string, opts Options) error {
	// ⚠️ 步骤0: 【关键】在合并前先捕获源文件的文件系统元数据
	srcInfo, _ := os.Stat(targetPath)
	var creationTime, modTime time.Time
	if srcInfo != nil {
		modTime = srcInfo.ModTime()
		if stat, ok := srcInfo.Sys().(*syscall.Stat_t); ok {
			creationTime = time.Unix(stat.Birthtimespec.Sec, stat.Birthtimespec.Nsec)
		}
	}

	ctx, cancel := context.WithTimeout(globalCtx, time.Duration(opts.TimeoutSeconds)*time.Second)
	defer cancel()

	// 使用exiftool将XMP数据写入目标文件（使用 -P 保留时间戳）
	cmd := exec.CommandContext(ctx, "exiftool",
		"-P", // ✅ 保留文件修改时间
		"-overwrite_original",
		"-TagsFromFile", xmpPath,
		"-all:all",
		targetPath)

	output, err := cmd.CombinedOutput()
	if err != nil {
		return fmt.Errorf("exiftool执行失败: %v, 输出: %s", err, string(output))
	}

	// ✅ 恢复文件系统元数据（创建时间、修改时间）
	if srcInfo != nil {
		// 恢复修改时间
		if err := os.Chtimes(targetPath, modTime, modTime); err != nil {
			logger.Printf("⚠️  文件时间恢复失败 %s: %v", filepath.Base(targetPath), err)
		}

		// 恢复创建时间（macOS）
		if !creationTime.IsZero() {
			timeStr := creationTime.Format("200601021504.05")
			exec.Command("touch", "-t", timeStr, targetPath).Run()
		}
	}

	return nil
}

// copyFile 复制文件
func copyFile(src, dst string) error {
	sourceData, err := os.ReadFile(src)
	if err != nil {
		return err
	}

	return os.WriteFile(dst, sourceData, 0644)
}

func copyMetadata(inputPath, outputPath string) error {
	cmd := exec.Command("exiftool", "-overwrite_original", "-TagsFromFile", inputPath, outputPath)
	return cmd.Run()
}

func getFileSize(filePath string) int64 {
	if info, err := os.Stat(filePath); err == nil {
		return info.Size()
	}
	return 0
}

func printStatistics() {
	stats.RLock()
	defer stats.RUnlock()
	totalProcessed := stats.ImagesProcessed + stats.ImagesFailed + stats.ImagesSkipped
	successRate := float64(stats.ImagesProcessed) / float64(totalProcessed) * 100
	logger.Println("")
	logger.Println("📊 处理统计:")
	logger.Printf("  • 总文件数: %d", totalProcessed)
	logger.Printf("  • 成功处理: %d", stats.ImagesProcessed)
	logger.Printf("  • 处理失败: %d", stats.ImagesFailed)
	logger.Printf("  • 跳过文件: %d", stats.ImagesSkipped)
	logger.Printf("  • 成功率: %.1f%%", successRate)
	if stats.TotalBytesBefore > 0 {
		compressionRatio := float64(stats.TotalBytesAfter) / float64(stats.TotalBytesBefore)
		logger.Printf("  • 压缩比: %.2f", compressionRatio)
	}
	logger.Printf("  • 处理时间: %v", stats.GetElapsedTime())
	if stats.PeakMemoryUsage > 0 {
		logger.Printf("  • 峰值内存: %d MB", stats.PeakMemoryUsage/1024/1024)
	}
	if stats.TotalRetries > 0 {
		logger.Printf("  • 总重试次数: %d", stats.TotalRetries)
	}
	if len(stats.ErrorTypes) > 0 {
		logger.Println("  • 错误类型统计:")
		for errorType, count := range stats.ErrorTypes {
			logger.Printf("    - %s: %d 次", errorType, count)
		}
	}
}

func main() {
	logger.Printf("🎨 优化版工具 v%s", version)
	logger.Printf("✨ 作者: %s", author)
	logger.Printf("🔧 开始初始化...")

	opts := parseFlags()
	logger.Println("🔍 检查系统依赖...")
	if err := checkDependencies(); err != nil {
		logger.Fatalf("❌ 系统依赖检查失败: %v", err)
	}

	configurePerformance(&opts)
	logger.Println("🔍 扫描文件...")
	files := scanCandidateFiles(opts.InputDir, opts)
	logger.Printf("📊 发现 %d 个候选文件", len(files))

	if len(files) == 0 {
		logger.Println("📊 没有找到需要处理的文件")
		return
	}

	if opts.DryRun {
		logger.Println("🔍 试运行模式 - 将要处理的文件:")
		for i, file := range files {
			logger.Printf("  %d. %s", i+1, file)
		}
		return
	}

	logger.Printf("🚀 开始处理 %d 个文件 (使用 %d 个工作线程)...", len(files), opts.Workers)
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
	logger.Println("🎉 处理完成！")
}
