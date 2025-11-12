// cmd/pixly/commands/config_enhanced.go - 增强配置命令
package commands

import (
	"fmt"
	"os"
	"strings"

	"pixly/pkg/config"

	"github.com/spf13/cobra"
)

// configEnhancedCmd 增强配置命令
var configEnhancedCmd = &cobra.Command{
	Use:   "config-advanced",
	Short: "Advanced Config management",
	Long: `Advanced Config management tool:

• Create default config file
• 查看完整配置
• 编辑配置
• 验证配置
• 导入/导出配置`,
}

// configInitCmd 初始化配置
var configInitCmd = &cobra.Command{
	Use:   "init",
	Short: "Initialize config file",
	Long:  `Create default config file到 ~/.pixly/config.yaml`,
	Run:   runConfigInit,
}

// configShowEnhancedCmd 显示配置
var configShowEnhancedCmd = &cobra.Command{
	Use:   "show",
	Short: "显示当前配置",
	Long:  `显示当前加载的完整配置`,
	Run:   runConfigShowEnhanced,
}

// configValidateEnhancedCmd 验证配置
var configValidateEnhancedCmd = &cobra.Command{
	Use:   "validate",
	Short: "Validate config file",
	Long:  `Validate config file的正确性和完整性`,
	Run:   runConfigValidateEnhanced,
}

// configEditCmd 编辑配置
var configEditCmd = &cobra.Command{
	Use:   "edit",
	Short: "编辑配置文件",
	Long:  `使用默认编辑器打开配置文件`,
	Run:   runConfigEdit,
}

// configPathCmd 显示配置路径
var configPathCmd = &cobra.Command{
	Use:   "path",
	Short: "显示Config file path",
	Long:  `显示配置文件和目录的路径`,
	Run:   runConfigPath,
}

var (
	configForce   bool
	configVerbose bool
)

func init() {
	// 子命令
	configEnhancedCmd.AddCommand(configInitCmd)
	configEnhancedCmd.AddCommand(configShowEnhancedCmd)
	configEnhancedCmd.AddCommand(configValidateEnhancedCmd)
	configEnhancedCmd.AddCommand(configEditCmd)
	configEnhancedCmd.AddCommand(configPathCmd)

	// 标志
	configInitCmd.Flags().BoolVarP(&configForce, "force", "f", false, "强制覆盖已存在的配置文件")
	configShowEnhancedCmd.Flags().BoolVarP(&configVerbose, "verbose", "v", false, "显示详细信息")
}

// runConfigInit 执行初始化配置
func runConfigInit(cmd *cobra.Command, args []string) {
	fmt.Println("🔧 Initialize config file...")

	configPath := config.DefaultConfigFile

	// 检查文件是否存在
	if _, err := os.Stat(configPath); err == nil && !configForce {
		fmt.Printf("❌ 配置File already exists: %s\n", configPath)
		fmt.Println("使用 --force 标志强制覆盖")
		return
	}

	// 创建默认配置
	if err := config.CreateDefaultConfig(configPath); err != nil {
		fmt.Printf("❌ 创建配置文件失败: %v\n", err)
		os.Exit(1)
	}

	fmt.Printf("✅ 配置文件已创建: %s\n", configPath)
	fmt.Println("")
	fmt.Println("📝 配置文件包含:")
	fmt.Println("  • 并发控制")
	fmt.Println("  • 转换设置")
	fmt.Println("  • 预测引擎配置")
	fmt.Println("  • UI和Theme")
	fmt.Println("  • 日志设置")
	fmt.Println("  • 知识库配置")
	fmt.Println("")
	fmt.Printf("编辑配置: pixly config-advanced edit\n")
	fmt.Printf("查看配置: pixly config-advanced show\n")
}

// runConfigShowEnhanced 显示配置
func runConfigShowEnhanced(cmd *cobra.Command, args []string) {
	// 加载配置
	cfg, err := config.Load("")
	if err != nil {
		fmt.Printf("❌ 加载配置失败: %v\n", err)
		fmt.Println("Hint: 使用 'pixly config-advanced init' 创建配置文件")
		os.Exit(1)
	}

	fmt.Println("📋 当前配置:")
	fmt.Println(strings.Repeat("=", 60))

	// 项目信息
	fmt.Println("\n[项目信息]")
	fmt.Printf("  名称: %s\n", cfg.Project.Name)
	fmt.Printf("  版本: %s\n", cfg.Project.Version)
	fmt.Printf("  作者: %s\n", cfg.Project.Author)

	// 并发控制
	fmt.Println("\n[并发控制]")
	fmt.Printf("  自动调整: %v\n", cfg.Concurrency.AutoAdjust)
	fmt.Printf("  转换Workers: %d\n", cfg.Concurrency.ConversionWorkers)
	fmt.Printf("  扫描Workers: %d\n", cfg.Concurrency.ScanWorkers)
	fmt.Printf("  内存限制: %d MB\n", cfg.Concurrency.MemoryLimitMB)
	fmt.Printf("  性能监控: %v\n", cfg.Concurrency.EnableMonitoring)

	// 转换设置
	fmt.Println("\n[转换设置]")
	fmt.Printf("  默认模式: %s\n", cfg.Conversion.DefaultMode)

	// 预测引擎
	fmt.Println("\n[预测引擎]")
	fmt.Printf("  启用知识库: %v\n", cfg.Conversion.Predictor.EnableKnowledgeBase)
	fmt.Printf("  置信度阈值: %.2f\n", cfg.Conversion.Predictor.ConfidenceThreshold)
	fmt.Printf("  启用探索: %v\n", cfg.Conversion.Predictor.EnableExploration)
	fmt.Printf("  探索候选数: %d\n", cfg.Conversion.Predictor.ExplorationCandidates)

	// 格式配置
	if configVerbose {
		fmt.Println("\n[格式配置]")
		fmt.Printf("  PNG目标: %s (无损: %v, effort: %d)\n",
			cfg.Conversion.Formats.PNG.Target,
			cfg.Conversion.Formats.PNG.Lossless,
			cfg.Conversion.Formats.PNG.Effort)
		fmt.Printf("  JPEG目标: %s (无损JPEG: %v, effort: %d)\n",
			cfg.Conversion.Formats.JPEG.Target,
			cfg.Conversion.Formats.JPEG.LosslessJPEG,
			cfg.Conversion.Formats.JPEG.Effort)
		fmt.Printf("  GIF静态: %s, 动图: %s\n",
			cfg.Conversion.Formats.GIF.StaticTarget,
			cfg.Conversion.Formats.GIF.AnimatedTarget)
		fmt.Printf("  视频目标: %s (仅重封装: %v)\n",
			cfg.Conversion.Formats.Video.Target,
			cfg.Conversion.Formats.Video.RepackageOnly)
	}

	// 输出设置
	fmt.Println("\n[输出设置]")
	fmt.Printf("  保留原文件: %v\n", cfg.Output.KeepOriginal)
	fmt.Printf("  生成报告: %v\n", cfg.Output.GenerateReport)
	fmt.Printf("  报告格式: %s\n", cfg.Output.ReportFormat)

	// 安全设置
	if configVerbose {
		fmt.Println("\n[安全设置]")
		fmt.Printf("  路径检查: %v\n", cfg.Security.EnablePathCheck)
		fmt.Printf("  磁盘空间检查: %v\n", cfg.Security.CheckDiskSpace)
		fmt.Printf("  最小剩余空间: %d MB\n", cfg.Security.MinFreeSpaceMB)
		fmt.Printf("  最大文件大小: %d MB\n", cfg.Security.MaxFileSizeMB)
	}

	// 断点续传
	fmt.Println("\n[断点续传]")
	fmt.Printf("  启用: %v\n", cfg.Resume.Enable)
	fmt.Printf("  保存间隔: 每 %d 个文件\n", cfg.Resume.SaveInterval)
	fmt.Printf("  崩溃自动续传: %v\n", cfg.Resume.AutoResumeOnCrash)

	// UI设置
	fmt.Println("\n[UI设置]")
	fmt.Printf("  模式: %s\n", cfg.UI.Mode)
	fmt.Printf("  Theme: %s\n", cfg.UI.Theme)
	fmt.Printf("  Emoji: %v\n", cfg.UI.EnableEmoji)
	fmt.Printf("  ASCII艺术: %v\n", cfg.UI.EnableASCIIArt)
	fmt.Printf("  动画: %v\n", cfg.UI.EnableAnimations)

	// 日志设置
	fmt.Println("\n[日志设置]")
	fmt.Printf("  级别: %s\n", cfg.Logging.Level)
	fmt.Printf("  输出: %s\n", cfg.Logging.Output)
	fmt.Printf("  文件: %s\n", cfg.Logging.FilePath)
	if configVerbose {
		fmt.Printf("  最大大小: %d MB\n", cfg.Logging.MaxSizeMB)
		fmt.Printf("  保留备份: %d 个\n", cfg.Logging.MaxBackups)
		fmt.Printf("  保留天数: %d 天\n", cfg.Logging.MaxAgeDays)
		fmt.Printf("  压缩: %v\n", cfg.Logging.Compress)
	}

	// 知识库
	fmt.Println("\n[知识库]")
	fmt.Printf("  启用: %v\n", cfg.Knowledge.Enable)
	fmt.Printf("  数据库: %s\n", cfg.Knowledge.DBPath)
	fmt.Printf("  自动学习: %v\n", cfg.Knowledge.AutoLearn)
	fmt.Printf("  最小置信度: %.2f\n", cfg.Knowledge.MinConfidence)

	// 语言
	fmt.Println("\n[语言]")
	fmt.Printf("  default: %s\n", cfg.Language.Default)
	fmt.Printf("  自动检测: %v\n", cfg.Language.AutoDetect)

	fmt.Println(strings.Repeat("=", 60))

	if !configVerbose {
		fmt.Println("\nHint: 使用 --verbose 查看完整配置")
	}
}

// runConfigValidateEnhanced 验证配置
func runConfigValidateEnhanced(cmd *cobra.Command, args []string) {
	fmt.Println("✅ Validate config file...")

	// 加载配置
	cfg, err := config.Load("")
	if err != nil {
		fmt.Printf("❌ 加载配置失败: %v\n", err)
		os.Exit(1)
	}

	// 验证配置
	if err := cfg.Validate(); err != nil {
		fmt.Printf("❌ 配置验证失败: %v\n", err)
		os.Exit(1)
	}

	fmt.Println("✅ 配置验证通过!")
	fmt.Println("")
	fmt.Println("检查项:")
	fmt.Printf("  ✓ Workers数量: %d (有效)\n", cfg.Concurrency.ConversionWorkers)
	fmt.Printf("  ✓ 置信度阈值: %.2f (有效)\n", cfg.Conversion.Predictor.ConfidenceThreshold)
	fmt.Printf("  ✓ Log level: %s (有效)\n", cfg.Logging.Level)
	fmt.Printf("  ✓ Config file path: %s\n", config.DefaultConfigFile)
}

// runConfigEdit 编辑配置
func runConfigEdit(cmd *cobra.Command, args []string) {
	configPath := config.DefaultConfigFile

	// 检查配置文件是否存在
	if _, err := os.Stat(configPath); os.IsNotExist(err) {
		fmt.Printf("❌ 配置文件不存在: %s\n", configPath)
		fmt.Println("Hint: 使用 'pixly config-advanced init' 创建配置文件")
		os.Exit(1)
	}

	// 获取默认编辑器
	editor := os.Getenv("EDITOR")
	if editor == "" {
		editor = "vim" // 默认使用vim
		// 在macOS上尝试使用open
		if _, err := os.Stat("/usr/bin/open"); err == nil {
			fmt.Printf("📝 使用默认编辑器打开: %s\n", configPath)
			fmt.Printf("命令: open %s\n", configPath)
			return
		}
	}

	fmt.Printf("📝 使用 %s 打开配置文件...\n", editor)
	fmt.Printf("命令: %s %s\n", editor, configPath)
	fmt.Println("")
	fmt.Println("编辑后记得验证: pixly config-advanced validate")
}

// runConfigPath 显示配置路径
func runConfigPath(cmd *cobra.Command, args []string) {
	fmt.Println("📁 配置路径信息:")
	fmt.Println(strings.Repeat("=", 60))

	info := config.GetConfigInfo()

	fmt.Printf("配置目录: %s\n", info["config_dir"])
	fmt.Printf("Config file: %s\n", info["config_file"])

	// 检查是否存在
	configFile := info["config_file"].(string)
	if _, err := os.Stat(configFile); err == nil {
		stat, _ := os.Stat(configFile)
		fmt.Printf("状态: ✅ 存在\n")
		fmt.Printf("Size: %d bytes\n", stat.Size())
		fmt.Printf("修改时间: %s\n", stat.ModTime().Format("2006-01-02 15:04:05"))
	} else {
		fmt.Printf("状态: ❌ 不存在\n")
		fmt.Println("创建: pixly config-advanced init")
	}

	// 相关目录
	fmt.Println("\n相关目录:")
	home, _ := os.UserHomeDir()
	fmt.Printf("  日志: %s/.pixly/logs/\n", home)
	fmt.Printf("  知识库: %s/.pixly/knowledge.db\n", home)
	fmt.Printf("  断点: %s/.pixly/checkpoint.db\n", home)

	fmt.Println(strings.Repeat("=", 60))
}
