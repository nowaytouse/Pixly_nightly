// media_tools - 媒体文件辅助工具集
//
// 功能说明：
// 1. XMP元数据合并 (merge命令)
// 2. 重复媒体文件检测和清理 (dedup命令)
//
// 作者：AI Assistant
// 版本：2.2.0
package main

import (
	"crypto/sha256"
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"sync"
	"sync/atomic"

	pkgutils "pixly/pkg/utils"
	"pixly/utils"
)

// 程序常量定义
const (
	toolName = "media_tools"  // 工具名称
	version  = "2.2.2"        // 程序版本号
	author   = "AI Assistant" // 作者信息
)

// 全局变量定义
var (
	logger *log.Logger // 全局日志记录器
)

// 命令类型
type Command string

const (
	MergeXMP  Command = "merge"     // 合并XMP元数据
	Dedup     Command = "dedup"     // 去重媒体文件
	Normalize Command = "normalize" // 规范化文件扩展名
	Auto      Command = "auto"      // 自动执行全部操作
)

func init() {
	// 设置日志记录器，带大小轮转
	rl, lf, err := utils.NewRotatingLogger("media_tools.log", 50*1024*1024)
	if err != nil {
		log.Fatalf("无法初始化轮转日志: %v", err)
	}
	logger = rl
	_ = lf
}

func main() {
	logger.Printf("🛠️  媒体工具集 v%s", version)
	logger.Printf("✨ 作者: %s", author)

	if len(os.Args) < 2 {
		printUsage()
		os.Exit(1)
	}

	command := Command(os.Args[1])
	switch command {
	case MergeXMP:
		runMergeXMP(os.Args[2:])
	case Dedup:
		runDedup(os.Args[2:])
	case Normalize:
		runNormalize(os.Args[2:])
	case Auto:
		runAuto(os.Args[2:])
	default:
		logger.Printf("❌ 未知命令: %s", command)
		printUsage()
		os.Exit(1)
	}
}

func printUsage() {
	fmt.Println(`
媒体工具集 v2.2.0

用法:
  media_tools <命令> [参数]

命令:
  merge      合并XMP侧边文件到媒体文件
  dedup      检测并清理重复媒体文件
  normalize  规范化文件名和扩展名
  auto       自动执行全部操作（推荐）

规范化功能:
  • 修复以'-'开头的文件名（会被工具识别为参数）
  • 移除开头/结尾的空格
  • 合并连续多个空格
  • 移除控制字符
  • 修复Windows保留文件名（CON, PRN等）
  • 截断过长的文件名
  • 规范化扩展名 (.jpeg→.jpg, .tiff→.tif)

示例:
  # 自动执行全部操作（推荐）
  media_tools auto -dir /path/to/media -trash /path/to/trash

  # 单独执行各项操作
  media_tools merge -dir /path/to/media
  media_tools normalize -dir /path/to/media
  media_tools normalize -dir /path/to/media -aggressive  # 激进模式
  media_tools dedup -dir /path/to/media -trash /path/to/trash

获取命令帮助:
  media_tools merge -h
  media_tools dedup -h
  media_tools normalize -h
  media_tools auto -h
`)
}

// ====== XMP合并功能 ======

func runMergeXMP(args []string) {
	fs := flag.NewFlagSet("merge", flag.ExitOnError)
	inputDir := fs.String("dir", "", "📂 输入目录路径（必需）")
	dryRun := fs.Bool("dry-run", false, "🔍 试运行模式，不实际执行")

	fs.Parse(args)

	if *inputDir == "" {
		logger.Println("❌ 错误: 必须指定输入目录 (-dir)")
		fs.PrintDefaults()
		os.Exit(1)
	}

	logger.Printf("🔧 开始XMP元数据合并...")
	logger.Printf("📂 输入目录: %s", *inputDir)
	logger.Printf("🔍 试运行: %v", *dryRun)

	// 扫描XMP文件
	xmpFiles, err := scanXMPFiles(*inputDir)
	if err != nil {
		logger.Fatalf("❌ 扫描XMP文件失败: %v", err)
	}

	logger.Printf("📊 找到 %d 个XMP文件", len(xmpFiles))

	merged := 0
	failed := 0

	for _, xmpFile := range xmpFiles {
		mediaFile := findMediaFile(xmpFile)
		if mediaFile == "" {
			logger.Printf("⚠️  未找到对应媒体文件: %s", xmpFile)
			failed++
			continue
		}

		if *dryRun {
			logger.Printf("🔍 [试运行] 将合并: %s -> %s", xmpFile, mediaFile)
			merged++
			continue
		}

		// 执行合并
		if err := mergeXMPToMedia(xmpFile, mediaFile); err != nil {
			logger.Printf("❌ 合并失败: %s -> %s: %v", xmpFile, mediaFile, err)
			failed++
			continue
		}

		logger.Printf("✅ 合并成功: %s -> %s", filepath.Base(xmpFile), filepath.Base(mediaFile))

		// 删除已合并的XMP文件
		if err := os.Remove(xmpFile); err != nil {
			logger.Printf("⚠️  删除XMP文件失败: %s: %v", filepath.Base(xmpFile), err)
		} else {
			logger.Printf("🗑️  已删除XMP文件: %s", filepath.Base(xmpFile))
		}

		merged++
	}

	logger.Printf("📊 合并完成: 成功 %d, 失败 %d", merged, failed)
}

// scanXMPFiles 扫描XMP文件
func scanXMPFiles(dir string) ([]string, error) {
	var xmpFiles []string
	err := filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && strings.HasSuffix(strings.ToLower(path), ".xmp") {
			xmpFiles = append(xmpFiles, path)
		}
		return nil
	})
	return xmpFiles, err
}

// findMediaFile 查找对应的媒体文件
// 支持包含空格和特殊字符的路径
func findMediaFile(xmpFile string) string {
	// 移除.xmp或.sidecar.xmp后缀
	basePath := strings.TrimSuffix(xmpFile, ".xmp")
	basePath = strings.TrimSuffix(basePath, ".sidecar")

	// 尝试查找对应的媒体文件
	mediaExts := []string{".jpg", ".jpeg", ".png", ".gif", ".mp4", ".mov", ".avi", ".jxl", ".avif", ".heic", ".heif", ".webp", ".bmp", ".tiff", ".tif", ".psd", ".psb", ".cr2", ".cr3", ".nef", ".arw", ".dng", ".raf", ".orf", ".rw2"}
	for _, ext := range mediaExts {
		mediaFile := basePath + ext
		if _, err := os.Stat(mediaFile); err == nil {
			return mediaFile
		}
		// 尝试大写
		mediaFile = basePath + strings.ToUpper(ext)
		if _, err := os.Stat(mediaFile); err == nil {
			return mediaFile
		}
	}

	return ""
}

// mergeXMPToMedia 合并XMP到媒体文件，并验证合并结果
func mergeXMPToMedia(xmpFile, mediaFile string) error {
	// 1. 验证文件路径安全性
	if !isValidFilePath(xmpFile) || !isValidFilePath(mediaFile) {
		return fmt.Errorf("文件路径包含不安全字符")
	}

	// 2. 验证媒体文件扩展名
	if !isMediaFile(filepath.Ext(mediaFile)) {
		return fmt.Errorf("不支持的媒体文件格式: %s", filepath.Ext(mediaFile))
	}

	// 3. 验证XMP文件内容
	if !isValidXMPFile(xmpFile) {
		return fmt.Errorf("XMP文件格式无效")
	}

	// 4. 执行合并
	cmd := exec.Command("exiftool", "-overwrite_original", "-TagsFromFile", xmpFile, "-all:all", mediaFile)
	output, err := cmd.CombinedOutput()
	if err != nil {
		return fmt.Errorf("exiftool执行失败: %v\n输出: %s", err, string(output))
	}

	// 5. 验证合并结果
	if !verifyMerge(mediaFile, xmpFile) {
		return fmt.Errorf("合并验证失败")
	}

	return nil
}

// isValidFilePath 验证文件路径是否安全
func isValidFilePath(filePath string) bool {
	// 检查路径是否包含非法字符
	if strings.ContainsAny(filePath, "\x00") {
		return false
	}

	// 检查路径长度
	if len(filePath) > 4096 {
		return false
	}

	return true
}

// isMediaFile 检查文件扩展名是否为支持的媒体格式
func isMediaFile(ext string) bool {
	switch strings.ToLower(ext) {
	case ".jpg", ".jpeg", ".png", ".tif", ".tiff", ".gif", ".mp4", ".mov", ".heic", ".heif", ".webp", ".avif", ".jxl", ".avi", ".mkv", ".bmp", ".psd", ".psb", ".cr2", ".cr3", ".nef", ".arw", ".dng", ".raf", ".orf", ".rw2":
		return true
	default:
		return false
	}
}

// isValidXMPFile 验证XMP文件格式是否有效
func isValidXMPFile(xmpPath string) bool {
	file, err := os.Open(xmpPath)
	if err != nil {
		return false
	}
	defer file.Close()

	// 读取文件头检查XMP格式
	header := make([]byte, 100)
	n, err := file.Read(header)
	if err != nil || n < 10 {
		return false
	}

	// 检查是否包含XMP标识
	content := string(header)
	if !strings.Contains(content, "xmpmeta") && !strings.Contains(content, "XMP") && !strings.Contains(content, "x:xmpmeta") {
		return false
	}

	// 检查文件大小是否合理（XMP文件通常不会太大）
	stat, err := file.Stat()
	if err != nil {
		return false
	}

	// XMP文件大小应该在100字节到10MB之间
	if stat.Size() < 100 || stat.Size() > 10*1024*1024 {
		return false
	}

	return true
}

// verifyMerge 验证XMP合并是否成功
// 通过比较XMP文件中的关键标签与媒体文件中的标签来验证
func verifyMerge(mediaPath, xmpPath string) bool {
	// 获取XMP文件中的所有标签
	xmpTagsCmd := exec.Command("exiftool", "-j", xmpPath)
	xmpTagsOutput, err := xmpTagsCmd.CombinedOutput()
	if err != nil {
		logger.Printf("❌ 获取XMP文件标签失败 %s: %v", filepath.Base(xmpPath), err)
		return false
	}

	var tags []map[string]interface{}
	if err := json.Unmarshal(xmpTagsOutput, &tags); err != nil {
		logger.Printf("❌ 解析XMP标签失败 %s: %v", filepath.Base(xmpPath), err)
		return false
	}

	if len(tags) == 0 || len(tags[0]) == 0 {
		logger.Printf("⚠️  XMP文件中没有找到任何可用标签 %s", filepath.Base(xmpPath))
		return false
	}

	// 找到一个有意义的标签进行验证，避免文件系统相关的标签
	var tagToVerify string
	for tag := range tags[0] {
		if !strings.HasPrefix(tag, "File:") && tag != "SourceFile" && tag != "ExifTool:ExifToolVersion" {
			tagToVerify = tag
			break
		}
	}

	if tagToVerify == "" {
		logger.Printf("⚠️  没有找到可验证的标签 %s", filepath.Base(xmpPath))
		return false
	}

	// 检查媒体文件中是否存在该标签
	mediaTagCmd := exec.Command("exiftool", "-"+tagToVerify, mediaPath)
	mediaTagOutput, err := mediaTagCmd.CombinedOutput()
	if err != nil {
		logger.Printf("❌ 获取媒体文件标签失败 %s: %v", filepath.Base(mediaPath), err)
		return false
	}

	if len(strings.TrimSpace(string(mediaTagOutput))) == 0 {
		logger.Printf("❌ 标签 %s 在媒体文件中未找到 %s", tagToVerify, filepath.Base(mediaPath))
		return false
	}

	logger.Printf("✅ 验证成功: 标签 '%s' 已正确合并", tagToVerify)
	return true
}

// ====== 去重功能 ======

func runDedup(args []string) {
	fs := flag.NewFlagSet("dedup", flag.ExitOnError)
	inputDir := fs.String("dir", "", "📂 输入目录路径（必需）")
	trashDir := fs.String("trash", "", "🗑️  垃圾箱目录（可选，默认为<dir>/.trash）")
	dryRun := fs.Bool("dry-run", false, "🔍 试运行模式，不实际执行")

	fs.Parse(args)

	if *inputDir == "" {
		logger.Println("❌ 错误: 必须指定输入目录 (-dir)")
		fs.PrintDefaults()
		os.Exit(1)
	}

	// 如果未指定trash目录，使用默认的.trash
	if *trashDir == "" {
		*trashDir = filepath.Join(*inputDir, ".trash")
		logger.Printf("📂 使用默认垃圾箱: %s", *trashDir)
	}

	logger.Printf("🔧 开始媒体文件去重...")
	logger.Printf("📂 输入目录: %s", *inputDir)
	logger.Printf("🗑️  垃圾箱: %s", *trashDir)
	logger.Printf("🔍 试运行: %v", *dryRun)

	// 创建垃圾箱目录
	if !*dryRun {
		if err := os.MkdirAll(*trashDir, 0755); err != nil {
			logger.Fatalf("❌ 创建垃圾箱目录失败: %v", err)
		}
	}

	// 扫描媒体文件
	mediaFiles, err := scanMediaFiles(*inputDir)
	if err != nil {
		logger.Fatalf("❌ 扫描媒体文件失败: %v", err)
	}

	logger.Printf("📊 找到 %d 个媒体文件", len(mediaFiles))

	// 计算哈希并检测重复
	hashMap := make(map[string]string) // hash -> first file path
	duplicates := 0
	moved := 0

	for _, file := range mediaFiles {
		hash, err := calculateHash(file)
		if err != nil {
			logger.Printf("⚠️  计算哈希失败: %s: %v", file, err)
			continue
		}

		if existingFile, exists := hashMap[hash]; exists {
			// 发现重复
			logger.Printf("🔍 发现重复: %s <-> %s", filepath.Base(file), filepath.Base(existingFile))
			duplicates++

			if !*dryRun {
				// 移动到垃圾箱
				trashPath := filepath.Join(*trashDir, filepath.Base(file))
				if err := os.Rename(file, trashPath); err != nil {
					logger.Printf("❌ 移动失败: %s: %v", file, err)
				} else {
					logger.Printf("✅ 已移动到垃圾箱: %s", filepath.Base(file))
					moved++
				}
			}
		} else {
			hashMap[hash] = file
		}
	}

	logger.Printf("📊 去重完成: 发现重复 %d, 已移动 %d", duplicates, moved)
}

// scanMediaFiles 扫描媒体文件
func scanMediaFiles(dir string) ([]string, error) {
	var mediaFiles []string
	mediaExts := map[string]bool{
		".jpg": true, ".jpeg": true, ".png": true, ".gif": true,
		".mp4": true, ".mov": true, ".avi": true, ".mkv": true,
		".jxl": true, ".avif": true, ".heic": true, ".heif": true,
		".webp": true, ".bmp": true, ".tiff": true, ".tif": true,
		".psd": true, ".psb": true,
		".cr2": true, ".cr3": true, ".nef": true, ".arw": true,
		".dng": true, ".raf": true, ".orf": true, ".rw2": true,
	}

	err := filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() {
			ext := strings.ToLower(filepath.Ext(path))
			if mediaExts[ext] {
				mediaFiles = append(mediaFiles, path)
			}
		}
		return nil
	})
	return mediaFiles, err
}

// calculateHash 计算文件SHA256哈希
// 支持包含空格和特殊字符的路径
func calculateHash(filePath string) (string, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return "", err
	}
	defer file.Close()

	hasher := sha256.New()
	if _, err := io.Copy(hasher, file); err != nil {
		return "", err
	}

	return fmt.Sprintf("%x", hasher.Sum(nil)), nil
}

// ====== 扩展名规范化功能 ======

func runNormalize(args []string) {
	fs := flag.NewFlagSet("normalize", flag.ExitOnError)
	inputDir := fs.String("dir", "", "📂 输入目录路径（必需）")
	dryRun := fs.Bool("dry-run", false, "🔍 试运行模式，不实际执行")
	fixFilenames := fs.Bool("fix-filenames", true, "🔧 修复问题文件名（默认启用）")
	aggressive := fs.Bool("aggressive", false, "⚡ 激进模式：更严格的规范化")

	fs.Parse(args)

	if *inputDir == "" {
		logger.Println("❌ 错误: 必须指定输入目录 (-dir)")
		fs.PrintDefaults()
		os.Exit(1)
	}

	logger.Printf("🔧 开始文件规范化...")
	logger.Printf("📂 输入目录: %s", *inputDir)
	logger.Printf("🔍 试运行: %v", *dryRun)
	logger.Printf("🔧 修复文件名: %v", *fixFilenames)
	logger.Printf("⚡ 激进模式: %v", *aggressive)

	totalNormalized := 0
	totalFailed := 0

	// 步骤1: 文件名规范化（新增）
	if *fixFilenames {
		logger.Printf("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
		logger.Printf("📋 步骤1: 文件名规范化")
		logger.Printf("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")

		filenameNormalizer := pkgutils.NewFilenameNormalizer()
		filenameNormalizer.Aggressive = *aggressive

		problemFiles, err := scanFilesForFilenameNormalization(*inputDir, filenameNormalizer)
		if err != nil {
			logger.Fatalf("❌ 扫描文件失败: %v", err)
		}

		logger.Printf("📊 找到 %d 个有问题的文件名", len(problemFiles))

		normalized := 0
		failed := 0

		for oldPath, result := range problemFiles {
			newPath := filepath.Join(filepath.Dir(oldPath), result.Normalized)

			if *dryRun {
				logger.Printf("🔍 [试运行] %s", filepath.Base(oldPath))
				logger.Printf("          → %s", filepath.Base(newPath))
				logger.Printf("          原因: %s", result.Details)
				normalized++
				continue
			}

			if err := os.Rename(oldPath, newPath); err != nil {
				logger.Printf("❌ 重命名失败: %s: %v", filepath.Base(oldPath), err)
				failed++
				continue
			}

			logger.Printf("✅ %s", filepath.Base(oldPath))
			logger.Printf("  → %s", filepath.Base(newPath))
			logger.Printf("  原因: %s", result.Details)
			normalized++
		}

		logger.Printf("\n📊 文件名规范化完成:")
		logger.Printf("  ✅ 成功: %d 个文件", normalized)
		logger.Printf("  ❌ 失败: %d 个文件", failed)

		totalNormalized += normalized
		totalFailed += failed
	}

	// 步骤2: 扩展名规范化（原有）
	logger.Printf("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	logger.Printf("📋 步骤2: 扩展名规范化")
	logger.Printf("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")

	// 扫描需要规范化的文件
	files, err := scanFilesForNormalization(*inputDir)
	if err != nil {
		logger.Fatalf("❌ 扫描文件失败: %v", err)
	}

	logger.Printf("📊 找到 %d 个需要规范化扩展名的文件", len(files))

	normalized := 0
	failed := 0

	for oldPath, newPath := range files {
		if *dryRun {
			logger.Printf("🔍 [试运行] 将重命名: %s -> %s", filepath.Base(oldPath), filepath.Base(newPath))
			normalized++
			continue
		}

		if err := os.Rename(oldPath, newPath); err != nil {
			logger.Printf("❌ 重命名失败: %s -> %s: %v", filepath.Base(oldPath), filepath.Base(newPath), err)
			failed++
			continue
		}

		logger.Printf("✅ 已规范化: %s -> %s", filepath.Base(oldPath), filepath.Base(newPath))
		normalized++
	}

	logger.Printf("\n📊 扩展名规范化完成:")
	logger.Printf("  ✅ 成功: %d 个文件", normalized)
	logger.Printf("  ❌ 失败: %d 个文件", failed)

	totalNormalized += normalized
	totalFailed += failed

	// 总结
	logger.Printf("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	logger.Printf("🎉 全部规范化完成:")
	logger.Printf("  ✅ 总成功: %d 个文件", totalNormalized)
	logger.Printf("  ❌ 总失败: %d 个文件", totalFailed)
	logger.Printf("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
}

// scanFilesForNormalization 扫描需要规范化的文件
// 返回 map[旧路径]新路径
func scanFilesForNormalization(dir string) (map[string]string, error) {
	needsNormalization := make(map[string]string)

	// 扩展名映射规则
	normalizationMap := map[string]string{
		".jpeg": ".jpg",
		".JPEG": ".jpg",
		".tiff": ".tif",
		".TIFF": ".tif",
	}

	err := filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			return nil
		}

		ext := filepath.Ext(path)
		if newExt, shouldNormalize := normalizationMap[ext]; shouldNormalize {
			newPath := strings.TrimSuffix(path, ext) + newExt
			needsNormalization[path] = newPath
		}

		return nil
	})

	return needsNormalization, err
}

// scanFilesForFilenameNormalization 扫描需要进行文件名规范化的文件
// 返回: map[原路径]*规范化结果
func scanFilesForFilenameNormalization(dir string, normalizer *pkgutils.FilenameNormalizer) (map[string]*pkgutils.NormalizationResult, error) {
	problemFiles := make(map[string]*pkgutils.NormalizationResult)

	err := filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			return nil
		}

		// 检查文件名是否有问题
		result := normalizer.NormalizeFilename(path)
		if result.Changed {
			problemFiles[path] = result
		}

		return nil
	})

	return problemFiles, err
}

// ====== 自动执行功能 ======

func runAuto(args []string) {
	fs := flag.NewFlagSet("auto", flag.ExitOnError)
	inputDir := fs.String("dir", "", "📂 输入目录路径（必需）")
	trashDir := fs.String("trash", "", "🗑️  垃圾箱目录（可选，默认为<dir>/.trash）")
	dryRun := fs.Bool("dry-run", false, "🔍 试运行模式，不实际执行")

	fs.Parse(args)

	if *inputDir == "" {
		logger.Println("❌ 错误: 必须指定输入目录 (-dir)")
		fs.PrintDefaults()
		os.Exit(1)
	}

	// 如果未指定trash目录，使用默认的.trash
	if *trashDir == "" {
		*trashDir = filepath.Join(*inputDir, ".trash")
		logger.Printf("📂 使用默认垃圾箱: %s", *trashDir)
	}

	logger.Printf("🚀 开始自动处理媒体文件...")
	logger.Printf("📂 输入目录: %s", *inputDir)
	logger.Printf("🗑️  垃圾箱: %s", *trashDir)
	logger.Printf("🔍 试运行: %v", *dryRun)
	logger.Println()

	// 步骤1: 扩展名规范化
	logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	logger.Println("📋 步骤 1/3: 扩展名规范化")
	logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	runNormalizeInternal(*inputDir, *dryRun)
	logger.Println()

	// 步骤2: XMP元数据合并
	logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	logger.Println("📋 步骤 2/3: XMP元数据合并")
	logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	runMergeXMPInternal(*inputDir, *dryRun)
	logger.Println()

	// 步骤3: 重复文件检测
	logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	logger.Println("📋 步骤 3/3: 重复文件检测和清理")
	logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	runDedupInternal(*inputDir, *trashDir, *dryRun)
	logger.Println()

	logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	logger.Println("🎉 自动处理完成！")
	logger.Println("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
}

// runNormalizeInternal 内部调用的规范化函数
func runNormalizeInternal(inputDir string, dryRun bool) {
	logger.Printf("🔍 扫描需要规范化的文件: %s", inputDir)

	files, err := scanFilesForNormalization(inputDir)
	if err != nil {
		logger.Printf("❌ 扫描文件失败: %v", err)
		return
	}

	logger.Printf("📊 找到 %d 个需要规范化的文件", len(files))

	if len(files) == 0 {
		logger.Printf("✅ 无需规范化,所有文件扩展名已标准化")
		return
	}

	normalized := 0
	failed := 0

	for oldPath, newPath := range files {
		if dryRun {
			logger.Printf("🔍 [试运行] 将重命名: %s -> %s", filepath.Base(oldPath), filepath.Base(newPath))
			normalized++
			continue
		}

		if err := os.Rename(oldPath, newPath); err != nil {
			logger.Printf("❌ 重命名失败: %s -> %s: %v", filepath.Base(oldPath), filepath.Base(newPath), err)
			failed++
			continue
		}

		logger.Printf("✅ 重命名成功: %s -> %s", filepath.Base(oldPath), filepath.Base(newPath))
		normalized++
	}

	logger.Printf("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	logger.Printf("📊 规范化完成: 成功 %d, 失败 %d", normalized, failed)
	logger.Printf("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
}

// runMergeXMPInternal 内部调用的XMP合并函数(并发版本)
func runMergeXMPInternal(inputDir string, dryRun bool) {
	logger.Printf("🔍 扫描XMP文件: %s", inputDir)

	xmpFiles, err := scanXMPFiles(inputDir)
	if err != nil {
		logger.Printf("❌ 扫描XMP文件失败: %v", err)
		return
	}

	logger.Printf("📊 找到 %d 个XMP文件", len(xmpFiles))

	if len(xmpFiles) == 0 {
		logger.Printf("✅ 无需合并,未发现XMP文件")
		return
	}

	// 智能计算并发数(参考universal_converter)
	workers := runtime.NumCPU()
	if workers > 8 {
		workers = 8 // 限制最大并发数
	}
	logger.Printf("⚡ 并发线程数: %d", workers)

	// 并发统计
	var merged, failed, deleted int32
	var wg sync.WaitGroup
	var mu sync.Mutex // 用于保护日志输出

	// 创建工作通道
	xmpChan := make(chan string, len(xmpFiles))

	// 进度计数
	processed := int32(0)
	total := int32(len(xmpFiles))

	// 启动工作协程
	for i := 0; i < workers; i++ {
		wg.Add(1)
		go func(workerID int) {
			defer wg.Done()

			for xmpFile := range xmpChan {
				mediaFile := findMediaFile(xmpFile)
				if mediaFile == "" {
					mu.Lock()
					logger.Printf("⚠️  未找到对应媒体文件: %s", filepath.Base(xmpFile))
					mu.Unlock()
					atomic.AddInt32(&failed, 1)
					atomic.AddInt32(&processed, 1)
					continue
				}

				if dryRun {
					mu.Lock()
					logger.Printf("🔍 [试运行] 将合并: %s -> %s", filepath.Base(xmpFile), filepath.Base(mediaFile))
					mu.Unlock()
					atomic.AddInt32(&merged, 1)
					atomic.AddInt32(&processed, 1)
					continue
				}

				if err := mergeXMPToMedia(xmpFile, mediaFile); err != nil {
					mu.Lock()
					logger.Printf("❌ 合并失败: %s -> %s: %v", filepath.Base(xmpFile), filepath.Base(mediaFile), err)
					mu.Unlock()
					atomic.AddInt32(&failed, 1)
					atomic.AddInt32(&processed, 1)
					continue
				}

				// 删除已合并的XMP文件
				if err := os.Remove(xmpFile); err != nil {
					mu.Lock()
					logger.Printf("⚠️  删除XMP文件失败: %s: %v", filepath.Base(xmpFile), err)
					mu.Unlock()
				} else {
					atomic.AddInt32(&deleted, 1)
				}

				atomic.AddInt32(&merged, 1)
				current := atomic.AddInt32(&processed, 1)

				// 每50个文件或最后一个文件显示进度
				if current%50 == 0 || current == total {
					mu.Lock()
					logger.Printf("⏳ XMP合并进度: %d/%d (%.1f%%)", current, total, float64(current)/float64(total)*100)
					mu.Unlock()
				}
			}
		}(i)
	}

	// 发送任务到通道
	for _, xmpFile := range xmpFiles {
		xmpChan <- xmpFile
	}
	close(xmpChan)

	// 等待所有工作完成
	wg.Wait()

	logger.Printf("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	logger.Printf("📊 合并完成: 成功 %d, 失败 %d", merged, failed)
	logger.Printf("🗑️  已删除XMP: %d 个", deleted)
	logger.Printf("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
}

// runDedupInternal 内部调用的去重函数
func runDedupInternal(inputDir string, trashDir string, dryRun bool) {
	logger.Printf("🔍 扫描媒体文件进行去重: %s", inputDir)
	logger.Printf("🗑️  垃圾箱目录: %s", trashDir)

	if !dryRun {
		if err := os.MkdirAll(trashDir, 0755); err != nil {
			logger.Printf("❌ 创建垃圾箱目录失败: %v", err)
			return
		}
	}

	mediaFiles, err := scanMediaFiles(inputDir)
	if err != nil {
		logger.Printf("❌ 扫描媒体文件失败: %v", err)
		return
	}

	logger.Printf("📊 找到 %d 个媒体文件,开始计算哈希值...", len(mediaFiles))

	hashMap := make(map[string]string)
	duplicates := 0
	moved := 0
	processed := 0

	for i, file := range mediaFiles {
		processed++
		if processed%50 == 0 || i == 0 || i == len(mediaFiles)-1 {
			logger.Printf("⏳ 去重进度: %d/%d (%.1f%%)", processed, len(mediaFiles), float64(processed)/float64(len(mediaFiles))*100)
		}

		hash, err := calculateHash(file)
		if err != nil {
			logger.Printf("⚠️  计算哈希失败: %s: %v", filepath.Base(file), err)
			continue
		}

		if existingFile, exists := hashMap[hash]; exists {
			logger.Printf("🔍 发现重复: %s <-> %s (哈希: %s...)", filepath.Base(file), filepath.Base(existingFile), hash[:8])
			duplicates++

			if dryRun {
				logger.Printf("🔍 [试运行] 将移动: %s", filepath.Base(file))
				moved++
			} else {
				trashPath := filepath.Join(trashDir, filepath.Base(file))
				if err := os.Rename(file, trashPath); err != nil {
					logger.Printf("❌ 移动失败: %s: %v", filepath.Base(file), err)
				} else {
					logger.Printf("✅ 已移动到垃圾箱: %s", filepath.Base(file))
					moved++
				}
			}
		} else {
			hashMap[hash] = file
		}
	}

	logger.Printf("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
	logger.Printf("📊 去重完成: 发现重复 %d, 已移动 %d", duplicates, moved)
	logger.Printf("✅ 唯一文件: %d 个", len(hashMap))
	logger.Printf("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
}
