// utils/parameters.go - 参数处理模块
//
// 功能说明：
// - 命令行参数解析和验证
// - 转换类型和处理模式定义
// - 系统配置和性能参数管理
// - 统一的参数接口和默认值设置
//
// 作者: AI Assistant
// 版本: v2.2.0
// 更新: 2025-10-24

package utils

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strconv"
	"strings"
)

// ConversionType 转换类型枚举
// 定义支持的输出格式类型
type ConversionType int

const (
	ConvertToAVIF ConversionType = iota // 转换为AVIF格式
	ConvertToJXL                        // 转换为JPEG XL格式
	ConvertToMOV                        // 转换为MOV格式
)

// ProcessingMode 处理模式枚举
// 定义文件处理的不同模式
type ProcessingMode int

const (
	ProcessAll       ProcessingMode = iota // 处理所有文件类型
	ProcessStatic                          // 仅处理静态图像
	ProcessDynamic                         // 仅处理动态图像
	ProcessVideo                           // 仅处理视频文件
	ProcessOptimized                       // 通用优化模式：JPEG使用无损，动态图片使用AVIF，视频使用MOV
)

// UniversalOptions 通用选项结构体
// 包含所有转换工具的统一配置参数，支持多种转换类型和处理模式
type UniversalOptions struct {
	// 基础参数 - 输入输出和基本设置
	InputDir       string // 输入目录路径
	OutputDir      string // 输出目录路径（可选，默认为输入目录）
	Workers        int    // 工作线程数（0表示自动检测）
	DryRun         bool   // 试运行模式，只显示将要处理的文件
	SkipExist      bool   // 跳过已存在的输出文件
	Retries        int    // 转换失败时的重试次数
	TimeoutSeconds int    // 单个文件处理的超时时间（秒）

	// 转换参数 - 转换类型和处理模式
	ConversionType ConversionType // 转换类型（AVIF/JXL/MOV）
	ProcessingMode ProcessingMode // 处理模式（全部/静态/动态/视频）

	// 质量参数 - 输出质量和性能设置
	Quality     int // 输出质量（1-100，100为最佳质量）
	Speed       int // 处理速度（1-10，10为最快速度）
	CJXLThreads int // CJXL编码器线程数

	// 验证参数 - 质量验证和容错设置
	StrictMode     bool // 严格模式，启用所有验证检查
	AllowTolerance float64
	Verify         bool

	// 元数据参数
	CopyMetadata  bool
	PreserveTimes bool

	// 日志参数
	LogLevel   string
	LogFile    string
	LogMaxSize int64

	// 报告参数
	ReportFormat string // json|csv|""
	ReportPath   string

	// 性能参数
	MaxMemoryUsage int64
	ProcessLimit   int
	FileLimit      int
}

// DefaultOptions 返回默认选项
func DefaultOptions() UniversalOptions {
	// 智能并发配置: 根据CPU核心数动态调整
	cpuCount := runtime.NumCPU()
	processLimit := getSmartProcessLimit(cpuCount)

	return UniversalOptions{
		Workers:        0, // 自动检测
		DryRun:         false,
		SkipExist:      false,
		Retries:        1,
		TimeoutSeconds: 30,
		ConversionType: ConvertToJXL,
		ProcessingMode: ProcessAll,
		Quality:        95,
		Speed:          4,
		CJXLThreads:    getSmartCJXLThreads(cpuCount),
		StrictMode:     true,
		AllowTolerance: 0.1,
		CopyMetadata:   true,
		PreserveTimes:  true,
		LogLevel:       "INFO",
		LogFile:        "",
		LogMaxSize:     100 * 1024 * 1024, // 100MB
		ReportFormat:   "",
		ReportPath:     "",
		MaxMemoryUsage: 0, // 无限制
		ProcessLimit:   processLimit,
		FileLimit:      processLimit * 2,
	}
}

// getSmartProcessLimit 根据CPU核心数智能计算并发进程数
// 避免过度并发导致内存耗尽
func getSmartProcessLimit(cpuCount int) int {
	// 保守策略: 预留一些核心给系统和其他任务
	// Mac Mini M4 (10核): 建议4-6个并发进程
	// 其他CPU: 核心数的40%-60%
	switch {
	case cpuCount >= 10: // M4/高端CPU
		return 4 // 保守策略,避免内存耗尽
	case cpuCount >= 8:
		return 3
	case cpuCount >= 4:
		return 2
	default:
		return 1
	}
}

// getSmartCJXLThreads 智能计算cjxl每个进程的线程数
// 单个进程不应使用全部CPU核心
func getSmartCJXLThreads(cpuCount int) int {
	// 每个cjxl进程使用的线程数
	// 避免单个进程占用过多资源
	switch {
	case cpuCount >= 10:
		return 4 // M4: 每个进程4线程
	case cpuCount >= 8:
		return 3
	case cpuCount >= 4:
		return 2
	default:
		return 1
	}
}

// ParseUniversalFlags 解析通用命令行参数
func ParseUniversalFlags() UniversalOptions {
	opts := DefaultOptions()

	// 基础参数
	// 统一主参：-input/-output/-workers/-timeout/-retries/-dry-run/-skip-exist
	var inputFlag string
	var dirAlias string
	flag.StringVar(&inputFlag, "input", "", "📂 输入目录路径")
	flag.StringVar(&dirAlias, "dir", "", "📂 输入目录路径（兼容，等价于 -input）")
	flag.StringVar(&opts.OutputDir, "output", "", "📁 输出目录（默认为输入目录）")
	flag.IntVar(&opts.Workers, "workers", opts.Workers, "⚡ 工作线程数 (0=自动检测)")
	flag.BoolVar(&opts.DryRun, "dry-run", opts.DryRun, "🔍 试运行模式，只显示将要处理的文件")
	flag.BoolVar(&opts.SkipExist, "skip-exist", opts.SkipExist, "⏭️ 跳过已存在的输出文件")
	flag.IntVar(&opts.Retries, "retries", opts.Retries, "🔄 转换失败时的重试次数")
	flag.IntVar(&opts.TimeoutSeconds, "timeout", opts.TimeoutSeconds, "⏰ 单个文件处理的超时时间（秒）")

	// 转换类型参数
	var conversionTypeStr string
	flag.StringVar(&conversionTypeStr, "type", "jxl", "🎨 转换类型: avif, jxl, mov")

	var processingModeStr string
	flag.StringVar(&processingModeStr, "mode", "all", "📋 处理模式: all, static, dynamic, video, optimized")

	// 质量参数
	flag.IntVar(&opts.Quality, "quality", opts.Quality, "🎯 输出质量 (1-100)")
	flag.IntVar(&opts.Speed, "speed", opts.Speed, "🚀 编码速度 (0-9)")
	flag.IntVar(&opts.CJXLThreads, "cjxl-threads", opts.CJXLThreads, "🧵 CJXL编码器线程数")

	// 验证参数
	flag.BoolVar(&opts.StrictMode, "strict", opts.StrictMode, "🔒 严格验证模式")
	flag.Float64Var(&opts.AllowTolerance, "tolerance", opts.AllowTolerance, "📏 允许的像素差异百分比")
	flag.BoolVar(&opts.Verify, "verify", true, "✅ 启用转换后验证 (默认开启)")

	// 元数据参数
	flag.BoolVar(&opts.CopyMetadata, "copy-metadata", opts.CopyMetadata, "📋 复制元数据")
	flag.BoolVar(&opts.PreserveTimes, "preserve-times", opts.PreserveTimes, "⏰ 保留文件时间戳")

	// 日志参数
	flag.StringVar(&opts.LogLevel, "log-level", opts.LogLevel, "📝 日志级别: DEBUG, INFO, WARN, ERROR")
	flag.StringVar(&opts.LogFile, "log-file", opts.LogFile, "📄 日志文件路径")
	flag.Int64Var(&opts.LogMaxSize, "log-max-size", opts.LogMaxSize, "📏 日志文件最大大小（字节）")

	// 报告参数
	flag.StringVar(&opts.ReportFormat, "report", opts.ReportFormat, "📊 报告格式: json|csv (留空不生成)")
	flag.StringVar(&opts.ReportPath, "report-path", opts.ReportPath, "🗂 报告输出路径（与 -report 搭配使用）")

	// 性能参数
	flag.Int64Var(&opts.MaxMemoryUsage, "max-memory", opts.MaxMemoryUsage, "💾 最大内存使用量（字节）")
	flag.IntVar(&opts.ProcessLimit, "process-limit", opts.ProcessLimit, "🔧 最大并发进程数")
	flag.IntVar(&opts.FileLimit, "file-limit", opts.FileLimit, "📁 最大并发文件数")

	flag.Parse()

	// 解析转换类型
	switch strings.ToLower(conversionTypeStr) {
	case "avif":
		opts.ConversionType = ConvertToAVIF
	case "jxl":
		opts.ConversionType = ConvertToJXL
	case "mov":
		opts.ConversionType = ConvertToMOV
	default:
		fmt.Printf("❌ 不支持的转换类型: %s\n", conversionTypeStr)
		os.Exit(1)
	}

	// 解析处理模式
	switch strings.ToLower(processingModeStr) {
	case "all":
		opts.ProcessingMode = ProcessAll
	case "static":
		opts.ProcessingMode = ProcessStatic
	case "dynamic":
		opts.ProcessingMode = ProcessDynamic
	case "video":
		opts.ProcessingMode = ProcessVideo
	case "optimized":
		opts.ProcessingMode = ProcessOptimized
	default:
		fmt.Printf("❌ 不支持的处理模式: %s\n", processingModeStr)
		os.Exit(1)
	}

	// 归一化输入目录（支持 -input 与 -dir）
	if inputFlag != "" {
		opts.InputDir = inputFlag
	} else {
		opts.InputDir = dirAlias
	}

	// 验证参数
	if err := opts.Validate(); err != nil {
		fmt.Printf("❌ 参数验证失败: %v\n", err)
		os.Exit(1)
	}

	// 设置默认输出目录
	if opts.OutputDir == "" {
		opts.OutputDir = opts.InputDir
	}

	return opts
}

// Validate 验证选项参数
func (opts *UniversalOptions) Validate() error {
	// 验证输入目录
	if opts.InputDir == "" {
		return fmt.Errorf("必须指定输入目录 (-input)")
	}

	if _, err := os.Stat(opts.InputDir); err != nil {
		return fmt.Errorf("输入目录不存在或不可访问: %v", err)
	}

	// 验证工作线程数
	if opts.Workers < 0 {
		return fmt.Errorf("工作线程数不能为负数: %d", opts.Workers)
	}

	if opts.Workers == 0 {
		opts.Workers = runtime.NumCPU()
	}

	// 验证重试次数
	if opts.Retries < 0 {
		return fmt.Errorf("重试次数不能为负数: %d", opts.Retries)
	}

	// 验证超时时间
	if opts.TimeoutSeconds <= 0 {
		return fmt.Errorf("超时时间必须大于0: %d", opts.TimeoutSeconds)
	}

	// 验证质量参数
	if opts.Quality < 1 || opts.Quality > 100 {
		return fmt.Errorf("质量参数必须在1-100之间: %d", opts.Quality)
	}

	// 验证速度参数
	if opts.Speed < 0 || opts.Speed > 9 {
		return fmt.Errorf("速度参数必须在0-9之间: %d", opts.Speed)
	}

	// 验证CJXL线程数
	if opts.CJXLThreads < 1 {
		return fmt.Errorf("CJXL线程数必须大于0: %d", opts.CJXLThreads)
	}

	// 验证容忍度
	if opts.AllowTolerance < 0 || opts.AllowTolerance > 100 {
		return fmt.Errorf("容忍度必须在0-100之间: %.2f", opts.AllowTolerance)
	}

	// 验证日志级别
	validLogLevels := map[string]bool{
		"DEBUG": true, "INFO": true, "WARN": true, "ERROR": true,
	}
	if !validLogLevels[strings.ToUpper(opts.LogLevel)] {
		return fmt.Errorf("无效的日志级别: %s", opts.LogLevel)
	}

	// 验证性能参数
	if opts.ProcessLimit < 1 {
		return fmt.Errorf("进程限制必须大于0: %d", opts.ProcessLimit)
	}

	if opts.FileLimit < 1 {
		return fmt.Errorf("文件限制必须大于0: %d", opts.FileLimit)
	}

	return nil
}

// GetOutputExtension 获取输出文件扩展名
func (opts *UniversalOptions) GetOutputExtension() string {
	switch opts.ConversionType {
	case ConvertToAVIF:
		return ".avif"
	case ConvertToJXL:
		return ".jxl"
	case ConvertToMOV:
		return ".mov"
	default:
		return ".unknown"
	}
}

// GetConversionCommand 获取转换命令
func (opts *UniversalOptions) GetConversionCommand(inputPath, outputPath string) (string, []string, error) {
	// 通用优化模式：根据文件类型智能选择转换方式
	if opts.ProcessingMode == ProcessOptimized {
		return opts.getOptimizedCommand(inputPath, outputPath)
	}

	switch opts.ConversionType {
	case ConvertToAVIF:
		return opts.getAVIFCommand(inputPath, outputPath)
	case ConvertToJXL:
		return opts.getJXLCommand(inputPath, outputPath)
	case ConvertToMOV:
		return opts.getMOVCommand(inputPath, outputPath)
	default:
		return "", nil, fmt.Errorf("不支持的转换类型: %d", opts.ConversionType)
	}
}

// getAVIFCommand 获取AVIF转换命令
func (opts *UniversalOptions) getAVIFCommand(inputPath, outputPath string) (string, []string, error) {
	// 检测文件类型以决定使用哪个工具
	fileType, _ := DetectFileType(inputPath)
	if fileType.IsAnimated {
		// 动态图片使用ffmpeg编码为动画AVIF
		args := []string{
			"-i", inputPath,
			"-c:v", "libaom-av1",
			"-crf", strconv.Itoa(63 - opts.Quality/2), // 质量映射: 100->0(最佳), 1->63(最差)
			"-cpu-used", strconv.Itoa(opts.Speed),
			"-an", // 不包含音频
			"-y",  // 覆盖已存在的文件
			outputPath,
		}
		return "ffmpeg", args, nil
	}
	// 静态图片使用官方avifenc编码为静态AVIF
	q := 63 - opts.Quality*63/100 // 质量映射: 100->0(最佳), 1->63(最差)
	args := []string{
		"--codec", "aom",
		"--min", strconv.Itoa(q),
		"--max", strconv.Itoa(q),
		"--speed", strconv.Itoa(opts.Speed),
		"-o", outputPath,
		inputPath,
	}
	return "avifenc", args, nil
}

// getJXLCommand 获取JXL转换命令
func (opts *UniversalOptions) getJXLCommand(inputPath, outputPath string) (string, []string, error) {
	// 根据文件类型选择转换策略
	ext := strings.ToLower(filepath.Ext(inputPath))

	// 智能effort选择: 根据文件大小动态调整
	// 大文件使用较低effort避免内存耗尽
	effort := opts.getSmartEffort(inputPath)

	// 检测文件类型（用于动画检测）
	_, err := DetectFileType(inputPath)
	if err != nil {
		return "", nil, fmt.Errorf("文件类型检测失败: %v", err)
	}

	switch ext {
	case ".jpg", ".jpeg":
		// JPEG专用无损模式
		args := []string{
			inputPath,
			"--lossless_jpeg=1",
			"-e", strconv.Itoa(effort),
			"--num_threads", strconv.Itoa(opts.CJXLThreads),
			outputPath,
		}
		return "cjxl", args, nil
	case ".gif":
		// GIF动画文件：JXL支持动画，使用cjxl转换
		args := []string{
			inputPath,
			"-d", "0", // 无损压缩
			"-e", strconv.Itoa(effort),
			"--num_threads", strconv.Itoa(opts.CJXLThreads),
			"--container=1", // 强制使用容器格式以支持动画
			outputPath,
		}
		return "cjxl", args, nil
	case ".png", ".bmp", ".tiff", ".tif", ".webp":
		// 非JPEG使用严格无损 (distance=0)
		args := []string{
			inputPath,
			"-d", "0",
			"-e", strconv.Itoa(effort),
			"--num_threads", strconv.Itoa(opts.CJXLThreads),
			outputPath,
		}
		return "cjxl", args, nil
	case ".avif", ".heic", ".heif":
		// 这些格式需要预处理
		return "", nil, fmt.Errorf("AVIF/HEIC/HEIF格式需要预处理，请使用专门的转换函数")
	default:
		return "", nil, fmt.Errorf("不支持的输入格式: %s", ext)
	}
}

// getSmartEffort 根据文件大小智能选择effort级别
// 避免大文件使用高effort导致内存耗尽
func (opts *UniversalOptions) getSmartEffort(inputPath string) int {
	stat, err := os.Stat(inputPath)
	if err != nil {
		return 7 // 默认值
	}

	fileSize := stat.Size()

	// 动态effort策略:
	// < 500KB:  effort 9 (最高压缩)
	// < 2MB:    effort 8
	// < 5MB:    effort 7
	// < 10MB:   effort 6
	// >= 10MB:  effort 5 (避免内存耗尽)
	switch {
	case fileSize < 500*1024:
		return 9
	case fileSize < 2*1024*1024:
		return 8
	case fileSize < 5*1024*1024:
		return 7
	case fileSize < 10*1024*1024:
		return 6
	default:
		return 5
	}
}

// getMOVCommand 获取MOV转换命令
func (opts *UniversalOptions) getMOVCommand(inputPath, outputPath string) (string, []string, error) {
	// 视频转换使用ffmpeg
	args := []string{
		"-i", inputPath,
		"-c", "copy", // 不重新编码，只重新封装
		"-movflags", "faststart", // 优化流媒体播放
		"-y", // 覆盖输出文件
		outputPath,
	}
	return "ffmpeg", args, nil
}

// IsSupportedInputFormat 检查是否为支持的输入格式
func (opts *UniversalOptions) IsSupportedInputFormat(filePath string) bool {
	ext := strings.ToLower(filepath.Ext(filePath))

	switch opts.ProcessingMode {
	case ProcessAll:
		return opts.isImageFormat(ext) || opts.isVideoFormat(ext)
	case ProcessStatic:
		return opts.isStaticImageFormat(ext)
	case ProcessDynamic:
		return opts.isDynamicImageFormat(ext)
	case ProcessVideo:
		return opts.isVideoFormat(ext)
	case ProcessOptimized:
		// 通用优化模式：支持JPEG、PNG、动态图片和视频格式
		return (ext == ".jpg" || ext == ".jpeg" || ext == ".png") || opts.isDynamicImageFormat(ext) || opts.isVideoFormat(ext)
	default:
		return false
	}
}

// isImageFormat 检查是否为图像格式
func (opts *UniversalOptions) isImageFormat(ext string) bool {
	imageFormats := map[string]bool{
		".jpg": true, ".jpeg": true, ".png": true, ".gif": true,
		".bmp": true, ".tiff": true, ".tif": true, ".webp": true,
		".avif": true, ".heic": true, ".heif": true,
	}
	return imageFormats[ext]
}

// isStaticImageFormat 检查是否为静态图像格式
func (opts *UniversalOptions) isStaticImageFormat(ext string) bool {
	staticFormats := map[string]bool{
		".jpg": true, ".jpeg": true, ".png": true,
		".bmp": true, ".tiff": true, ".tif": true,
		".avif": true, ".heic": true, ".heif": true,
	}
	return staticFormats[ext]
}

// isDynamicImageFormat 检查是否为动态图像格式
func (opts *UniversalOptions) isDynamicImageFormat(ext string) bool {
	dynamicFormats := map[string]bool{
		".gif": true, ".webp": true, ".avif": true, ".heic": true, ".heif": true,
	}
	return dynamicFormats[ext]
}

// isVideoFormat 检查是否为视频格式
func (opts *UniversalOptions) isVideoFormat(ext string) bool {
	videoFormats := map[string]bool{
		".mp4": true, ".mov": true, ".avi": true, ".mkv": true, ".webm": true,
		".m4v": true, ".3gp": true, ".flv": true, ".wmv": true,
	}
	return videoFormats[ext]
}

// GetDescription 获取选项描述
func (opts *UniversalOptions) GetDescription() string {
	var parts []string

	// 转换类型
	switch opts.ConversionType {
	case ConvertToAVIF:
		parts = append(parts, "AVIF转换")
	case ConvertToJXL:
		parts = append(parts, "JXL转换")
	case ConvertToMOV:
		parts = append(parts, "MOV转换")
	}

	// 处理模式
	switch opts.ProcessingMode {
	case ProcessAll:
		parts = append(parts, "全部文件")
	case ProcessStatic:
		parts = append(parts, "静态图像")
	case ProcessDynamic:
		parts = append(parts, "动态图像")
	case ProcessVideo:
		parts = append(parts, "视频文件")
	}

	// 质量设置
	parts = append(parts, fmt.Sprintf("质量%d", opts.Quality))

	// 线程设置
	parts = append(parts, fmt.Sprintf("%d线程", opts.Workers))

	return strings.Join(parts, " | ")
}

// getOptimizedCommand 获取通用优化模式的转换命令
// 根据文件类型智能选择转换方式：
// 1. JPEG文件 -> JXL无损模式 (jpeg_lossless=1)
// 2. PNG文件 -> JXL无损模式 (distance=0)
// 3. 动态图片 -> AVIF格式
// 4. 视频文件 -> MOV重新包装
func (opts *UniversalOptions) getOptimizedCommand(inputPath, outputPath string) (string, []string, error) {
	fileType, err := DetectFileType(inputPath)
	if err != nil {
		return "", nil, fmt.Errorf("文件类型检测失败: %v", err)
	}

	ext := strings.ToLower(filepath.Ext(inputPath))

	// 1. JPEG文件使用JXL无损模式
	if ext == ".jpg" || ext == ".jpeg" {
		effort := opts.getSmartEffort(inputPath)
		args := []string{
			inputPath,
			"--lossless_jpeg=1", // 使用JPEG无损模式
			"-e", strconv.Itoa(effort),
			"--num_threads", strconv.Itoa(opts.CJXLThreads),
			outputPath,
		}
		return "cjxl", args, nil
	}

	// 2. PNG文件使用JXL无损模式
	if ext == ".png" {
		effort := opts.getSmartEffort(inputPath)
		args := []string{
			inputPath,
			"-d", "0", // 使用无损模式（distance=0）
			"-e", strconv.Itoa(effort),
			"--num_threads", strconv.Itoa(opts.CJXLThreads),
			outputPath,
		}
		return "cjxl", args, nil
	}

	// 3. 动态图片使用AVIF格式
	if fileType.IsAnimated {
		args := []string{
			"-i", inputPath,
			"-c:v", "libaom-av1",
			"-crf", strconv.Itoa(63 - opts.Quality/2), // 质量映射: 100->0(最佳), 1->63(最差)
			"-cpu-used", strconv.Itoa(opts.Speed),
			"-an", // 不包含音频
			"-y",  // 覆盖已存在的文件
			outputPath,
		}
		return "ffmpeg", args, nil
	}

	// 4. 视频文件使用MOV重新包装
	if opts.isVideoFormat(ext) {
		args := []string{
			"-i", inputPath,
			"-c", "copy", // 不重新编码，只重新封装
			"-movflags", "faststart", // 优化流媒体播放
			"-y", // 覆盖输出文件
			outputPath,
		}
		return "ffmpeg", args, nil
	}

	// 其他格式不支持
	return "", nil, fmt.Errorf("通用优化模式不支持此文件格式: %s", ext)
}

// GetOutputExtensionForFile 根据文件路径获取输出扩展名（用于通用优化模式）
func (opts *UniversalOptions) GetOutputExtensionForFile(filePath string) string {
	// 通用优化模式：根据文件类型选择输出格式
	if opts.ProcessingMode == ProcessOptimized {
		fileType, err := DetectFileType(filePath)
		if err != nil {
			return ".unknown"
		}

		ext := strings.ToLower(filepath.Ext(filePath))

		// JPEG文件输出为JXL
		if ext == ".jpg" || ext == ".jpeg" {
			return ".jxl"
		}

		// PNG文件输出为JXL
		if ext == ".png" {
			return ".jxl"
		}

		// 动态图片输出为AVIF
		if fileType.IsAnimated {
			return ".avif"
		}

		// 视频文件输出为MOV
		if opts.isVideoFormat(ext) {
			return ".mov"
		}

		// 其他格式不应该到达这里
		return ".unknown"
	}

	// 非优化模式使用标准逻辑
	return opts.GetOutputExtension()
}
