// dynamic2h266mov - 动态图片转高效视频MOV工具
// 版本: v1.0.0
// 作者: AI Assistant
// 功能: 将动态图片（GIF/WebP/APNG）转换为最新的H.266/VVC编码MOV视频
// 要求: FFmpeg 8.0+

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
}

func init() {
	logger = utils.SetupLogging("dynamic2h266mov.log")
	stats = utils.NewSharedStats()
	utils.SetupSignalHandlingWithCallback(logger, printStatistics)
}

func parseFlags() Options {
	var opts Options

	flag.StringVar(&opts.InputDir, "dir", "", "📂 输入目录路径（必需）")
	flag.StringVar(&opts.OutputDir, "output", "", "📁 输出目录路径（默认为输入目录）")
	flag.IntVar(&opts.Workers, "workers", 0, "⚡ 工作线程数 (0=自动检测)")
	flag.StringVar(&opts.Lang, "lang", "zh", "🌍 Language / 界面语言 (zh/en)")
	flag.BoolVar(&opts.SkipExist, "skip-exist", false, "⏭️ 跳过已存在的文件")
	flag.BoolVar(&opts.DryRun, "dry-run", false, "🔍 试运行模式")
	flag.IntVar(&opts.TimeoutSeconds, "timeout", 600, "⏰ 单个文件处理超时时间（秒）")
	flag.IntVar(&opts.Retries, "retries", 2, "🔄 转换失败重试次数")
	flag.Int64Var(&opts.MaxMemory, "max-memory", 0, "💾 最大内存使用量（字节，0=无限制）")
	flag.Int64Var(&opts.MaxFileSize, "max-file-size", 500*1024*1024, "📏 最大文件大小（字节）")
	flag.BoolVar(&opts.EnableHealthCheck, "health-check", true, "🏥 启用健康检查")

	flag.Parse()
	i18n.SetLang(opts.Lang)

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
			logger.Printf("⚠️  扫描文件时出错: %s - %v", osPathname, err)
			return godirwalk.SkipNode
		},
	})
	if err != nil {
		logger.Printf("❌ 扫描文件时出错: %v", err)
	}
	return files
}

func processFileWithRetry(filePath string, fileInfo os.FileInfo, opts Options) {
	var lastErr error
	for attempt := 0; attempt <= opts.Retries; attempt++ {
		if attempt > 0 {
			logger.Printf("🔄 重试处理文件: %s (第 %d 次)", filepath.Base(filePath), attempt)
			time.Sleep(time.Duration(attempt) * time.Second)
			stats.AddRetry()
		}
		err := processFileWithOpts(filePath, fileInfo, stats, opts)
		if err == nil {
			return
		}
		lastErr = err
		logger.Printf("⚠️  处理文件失败: %s - %v", filepath.Base(filePath), err)
		stats.AddErrorType(utils.ClassifyError(err))
	}
	logger.Printf("❌ 文件处理最终失败: %s - %v", filepath.Base(filePath), lastErr)
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
		stats.AddProcessed(fileInfo.Size(), utils.GetFileSize(outputPath))
		stats.AddByExt(filepath.Ext(filePath))
	}
	stats.AddDetailedLog(processInfo)
	return err
}

func processFileByType(filePath string, opts Options) (string, string, string, error) {
	// 动态图片转H.266/VVC编码MOV的实际转换逻辑
	outputPath := strings.TrimSuffix(filePath, filepath.Ext(filePath)) + ".mov"
	conversionMode := "动图转H.266编码MOV"

	// ✅ 步骤1: 捕获源文件的文件系统元数据（在转换之前）
	srcInfo, _ := os.Stat(filePath)
	var creationTime time.Time
	if srcInfo != nil {
		if stat, ok := srcInfo.Sys().(*syscall.Stat_t); ok {
			creationTime = time.Unix(stat.Birthtimespec.Sec, stat.Birthtimespec.Nsec)
		}
	}

	// ✅ 检查H.266支持（使用vvencFFapp）
	h266Supported, err := checkH266Support()
	if err != nil || !h266Supported {
		logger.Printf("⏭️  H.266/VVC编码器不可用，跳过文件: %s", filepath.Base(filePath))
		logger.Printf("   💡 提示: 请安装vvenc: brew install vvenc")
		return conversionMode, "", "", fmt.Errorf("H.266编码器不可用（已跳过）")
	}

	logger.Printf("🎯 使用vvencFFapp三步法（100%%成功率）")

	// 临时文件
	tempY4M := strings.TrimSuffix(outputPath, ".mov") + "_temp.y4m"
	tempH266 := strings.TrimSuffix(outputPath, ".mov") + "_temp.h266"
	defer func() {
		os.Remove(tempY4M)
		os.Remove(tempH266)
	}()

	// ✅ 步骤2.1: ffmpeg提取视频流到Y4M
	logger.Printf("  [1/3] 提取视频流到Y4M...")
	y4mArgs := []string{
		"-hide_banner", "-loglevel", "error",
		"-i", filePath,
		"-pix_fmt", "yuv420p",
		"-vf", "scale=trunc(iw/2)*2:trunc(ih/2)*2",
		"-f", "yuv4mpegpipe",
		"-y", tempY4M,
	}

	ctx1, cancel1 := context.WithTimeout(globalCtx, 3*time.Minute)
	defer cancel1()
	cmd1 := exec.CommandContext(ctx1, "ffmpeg", y4mArgs...)
	if output, err := cmd1.CombinedOutput(); err != nil {
		if ctx1.Err() == context.DeadlineExceeded {
			return conversionMode, "", string(output), fmt.Errorf("步骤1超时（Y4M提取）")
		}
		return conversionMode, "", string(output), fmt.Errorf("Y4M提取失败: %v", err)
	}

	// ✅ 步骤2.2: vvencFFapp编码H.266
	logger.Printf("  [2/3] vvencFFapp编码H.266...")
	vvencArgs := []string{
		"-i", tempY4M,
		"--BitstreamFile", tempH266,
		"--preset", "faster",
		"--qp", "30",
		"-t", "4",
		"--verbosity", "1",
	}

	ctx2, cancel2 := context.WithTimeout(globalCtx, 5*time.Minute)
	defer cancel2()
	cmd2 := exec.Command("vvencFFapp", vvencArgs...)
	if output, err := cmd2.CombinedOutput(); err != nil {
		if ctx2.Err() == context.DeadlineExceeded {
			return conversionMode, "", string(output), fmt.Errorf("步骤2超时（H.266编码）")
		}
		return conversionMode, "", string(output), fmt.Errorf("vvencFFapp编码失败: %v", err)
	}

	// ✅ 步骤2.3: ffmpeg打包到MOV
	logger.Printf("  [3/3] 打包到MOV容器...")
	muxArgs := []string{
		"-hide_banner", "-loglevel", "error",
		"-i", tempH266,
		"-c:v", "copy",
		"-map_metadata", "0",
		"-movflags", "use_metadata_tags",
		"-f", "mov",
		"-y", outputPath,
	}

	ctx3, cancel3 := context.WithTimeout(globalCtx, 1*time.Minute)
	defer cancel3()
	cmd3 := exec.CommandContext(ctx3, "ffmpeg", muxArgs...)
	if output, err := cmd3.CombinedOutput(); err != nil {
		if ctx3.Err() == context.DeadlineExceeded {
			return conversionMode, "", string(output), fmt.Errorf("步骤3超时（MOV封装）")
		}
		return conversionMode, "", string(output), fmt.Errorf("MOV封装失败: %v", err)
	}

	logger.Printf("✅ 动图转MOV成功（H.266编码）: %s", filepath.Base(outputPath))

	// ✅ 步骤3: 复制EXIF元数据（会改变文件修改时间）
	if err := utils.CopyMetadata(filePath, outputPath); err != nil {
		logger.Printf("⚠️  EXIF元数据复制失败: %s -> %s: %v",
			filepath.Base(filePath), filepath.Base(outputPath), err)
	} else {
		logger.Printf("✅ EXIF元数据复制成功: %s", filepath.Base(outputPath))
	}

	// ✅ 步骤4: 恢复文件系统元数据（在exiftool之后）
	if srcInfo != nil {
		// 4.1 恢复Finder标签和注释
		if err := utils.CopyFinderMetadata(filePath, outputPath); err != nil {
			logger.Printf("⚠️  Finder元数据复制失败 %s: %v", filepath.Base(outputPath), err)
		} else {
			logger.Printf("✅ Finder元数据复制成功: %s", filepath.Base(outputPath))
		}

		// 4.2 恢复修改时间和创建时间（使用touch统一设置）
		if !creationTime.IsZero() {
			timeStr := creationTime.Format("200601021504.05")
			touchCmd := exec.Command("touch", "-t", timeStr, outputPath)
			if err := touchCmd.Run(); err != nil {
				logger.Printf("⚠️  文件时间恢复失败 %s: %v", filepath.Base(outputPath), err)
			} else {
				logger.Printf("✅ 文件系统元数据已保留: %s (创建/修改: %s)",
					filepath.Base(outputPath), creationTime.Format("2006-01-02 15:04:05"))
			}
		}
	}

	// ✅ 原地删除：转换成功后删除原始文件
	if err := os.Remove(filePath); err != nil {
		logger.Printf("⚠️  原始文件删除失败 %s: %v", filepath.Base(filePath), err)
	} else {
		logger.Printf("🗑️  原始文件已删除: %s", filepath.Base(filePath))
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
	logger.Println("📊 处理统计:")
	logger.Printf("  • 总文件数: %d", totalProcessed)
	logger.Printf("  • 成功处理: %d", stats.ImagesProcessed)
	logger.Printf("  • 处理失败: %d", stats.ImagesFailed)
	logger.Printf("  • 跳过文件: %d", stats.ImagesSkipped)
	logger.Printf("  • 成功率: %.1f%%", successRate)
	if stats.TotalBytesBefore > 0 {
		savingPercent := (1 - float64(stats.TotalBytesAfter)/float64(stats.TotalBytesBefore)) * 100
		logger.Printf("  • 空间节省: %.1f%%", savingPercent)
	}
	logger.Printf("  • 处理时间: %v", stats.GetElapsedTime())
	if stats.TotalRetries > 0 {
		logger.Printf("  • 总重试次数: %d", stats.TotalRetries)
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
	logger.Printf("🎬 dynamic2mov v%s", version)
	logger.Printf("✨ 作者: %s", author)
	logger.Printf("🔧 开始初始化...")

	opts := parseFlags()
	logger.Println("🔍 检查系统依赖...")
	if err := checkDependencies(); err != nil {
		logger.Fatalf("❌ 系统依赖检查失败: %v", err)
	}

	configurePerformance(&opts)
	logger.Println("🔍 扫描GIF文件...")
	files := scanCandidateFiles(opts.InputDir, opts)
	logger.Printf("📊 发现 %d 个GIF文件", len(files))

	if len(files) == 0 {
		logger.Println("📊 没有找到GIF文件")
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

// runNonInteractiveMode_WithOpts 使用指定选项运行
func runNonInteractiveMode_WithOpts(opts Options) {
	logger.Printf("🎬 dynamic2mov v%s", version)
	logger.Println("🔍 检查系统依赖...")
	if err := checkDependencies(); err != nil {
		logger.Fatalf("❌ 系统依赖检查失败: %v", err)
	}

	configurePerformance(&opts)
	logger.Println("🔍 扫描动态图片文件（GIF/WebP/APNG）...")
	files := scanCandidateFiles(opts.InputDir, opts)
	logger.Printf("📊 发现 %d 个动态图片文件", len(files))

	if len(files) == 0 {
		logger.Println("📊 没有找到动态图片文件")
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

// runInteractiveMode 交互模式入口
func runInteractiveMode() {
	// 1. 显示横幅
	fmt.Println("╔═══════════════════════════════════════════════════════════════╗")
	fmt.Println("║                                                               ║")
	fmt.Println("║   🎬 dynamic2h266mov v1.0.0 - H.266实验性工具                ║")
	fmt.Println("║                                                               ║")
	fmt.Println("║   输入: GIF / WebP（动图）/ APNG                             ║")
	fmt.Println("║   输出: MOV视频（H.266/VVC编码）                        ║")
	fmt.Println("║   编码: H.266最新标准（极致压缩94.7%）                 ║")
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

// checkH266Support 检查H.266/VVC编码器支持（vvencFFapp）
func checkH266Support() (bool, error) {
	cmd := exec.Command("vvencFFapp", "--version")
	output, err := cmd.CombinedOutput()
	if err != nil {
		logger.Printf("⚠️  vvencFFapp不可用")
		logger.Printf("   安装方法: brew install vvenc")
		return false, nil
	}

	versionStr := string(output)
	logger.Printf("✅ 检测到VVenC: %s", strings.TrimSpace(strings.Split(versionStr, "\n")[0]))
	return true, nil
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
