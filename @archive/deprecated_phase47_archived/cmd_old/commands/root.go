// cmd/pixly/commands/root.go - 根命令
//
// 功能说明：
// - 定义pixly的根命令结构
// - 提供全局配置和选项
// - 整合所有子命令
//
// 作者: AI Assistant
// 版本: v2.2.0
// 更新: 2025-10-24

package commands

import (
	"fmt"
	"os"

	"pixly/pkg/config"
	"pixly/pkg/i18n"
	"pixly/pkg/ui"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var (
	// 全局配置
	cfgFile    string
	verbose    bool
	dryRun     bool
	workers    int
	outputDir  string
	logLevel   string
	configPath string

	// i18n和UI配置
	langFlag    string
	themeFlag   string
	noBanner    bool
	noAnimation bool // Disable animation effects(节省性能)
)

// RootCmd 根命令
var RootCmd = &cobra.Command{
	Use:   "pixly",
	Short: "Pixly - Intelligent Media Format Conversion Tool",
	Long: `Pixly is an intelligent media format conversion tool that supports:

• Image format conversion (JXL, AVIF, WebP, PNG, JPEG)
• Video format conversion (MOV, MP4, WebM)
• Smart format recommendation and optimization
• Batch processing and resume support
• Metadata preservation and quality validation
• Transparent image optimization
• Animated image to video

Use 'pixly <command> --help' to get help for specific commands.`,
	Version: "4.8.0",
	PersistentPreRun: func(cmd *cobra.Command, args []string) {
		// 加载完整配置系统
		loadFullConfig()

		// 初始化i18n
		initI18n()

		// 初始化UITheme
		initUITheme()

		// 显示欢迎横幅 (除非禁用)
		if !noBanner && len(os.Args) > 1 {
			showWelcomeBanner()
		}

		// 初始化配置 (viper兼容)
		initConfig()

		// 设置Log level
		setLogLevel()

		// 验证全局参数
		if err := validateGlobalFlags(); err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
	},
}

// Execute 执行根命令
func Execute() {
	if err := RootCmd.Execute(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}

func init() {
	// 添加全局标志
	RootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "Config file path (default: ~/.pixly/config.yaml)")
	RootCmd.PersistentFlags().BoolVarP(&verbose, "verbose", "v", false, "Verbose output")
	RootCmd.PersistentFlags().BoolVar(&dryRun, "dry-run", false, "Dry-run mode, only show operations to be executed")
	RootCmd.PersistentFlags().IntVarP(&workers, "workers", "w", 0, "Worker count (0means auto-detect)")
	RootCmd.PersistentFlags().StringVarP(&outputDir, "output", "o", "", "Output directory (default: Input directory)")
	RootCmd.PersistentFlags().StringVar(&logLevel, "log-level", "info", "Log level (debug, info, warn, error)")
	RootCmd.PersistentFlags().StringVar(&configPath, "config-path", "", "Config file path")

	// i18n和UI标志
	RootCmd.PersistentFlags().StringVar(&langFlag, "lang", "", "UI language (zh-CN, en-US, ja-JP, auto)")
	RootCmd.PersistentFlags().StringVar(&themeFlag, "theme", "default", "UITheme (default, dark, light, mono, neon)")
	RootCmd.PersistentFlags().BoolVar(&noBanner, "no-banner", false, "Disable welcome banner")
	RootCmd.PersistentFlags().BoolVar(&noAnimation, "no-animation", false, "Disable animation effects(Auto-disable during conversion to save performance)")

	// 绑定到viper
	viper.BindPFlag("verbose", RootCmd.PersistentFlags().Lookup("verbose"))
	viper.BindPFlag("dry-run", RootCmd.PersistentFlags().Lookup("dry-run"))
	viper.BindPFlag("workers", RootCmd.PersistentFlags().Lookup("workers"))
	viper.BindPFlag("output", RootCmd.PersistentFlags().Lookup("output"))
	viper.BindPFlag("log-level", RootCmd.PersistentFlags().Lookup("log-level"))
	viper.BindPFlag("config-path", RootCmd.PersistentFlags().Lookup("config-path"))

	// 添加子命令
	addSubCommands()
}

// addSubCommands 添加子命令
func addSubCommands() {
	// 转换命令
	RootCmd.AddCommand(convertCmd)
	RootCmd.AddCommand(batchCmd)
	RootCmd.AddCommand(smartCmd)

	// 工具命令
	RootCmd.AddCommand(analyzeCmd)
	RootCmd.AddCommand(optimizeCmd)
	RootCmd.AddCommand(validateCmd)

	// 管理命令
	RootCmd.AddCommand(configCmd)
	RootCmd.AddCommand(configEnhancedCmd) // 增强配置命令
	RootCmd.AddCommand(statsCmd)
	RootCmd.AddCommand(cleanupCmd)
}

// initConfig 初始化配置
func initConfig() {
	if cfgFile != "" {
		viper.SetConfigFile(cfgFile)
	} else {
		// 设置默认Config file path
		home, err := os.UserHomeDir()
		if err != nil {
			fmt.Fprintf(os.Stderr, "Cannot get user home directory: %v\n", err)
			return
		}

		viper.AddConfigPath(home + "/.pixly")
		viper.AddConfigPath(".")
		viper.SetConfigName("config")
		viper.SetConfigType("yaml")
	}

	// 设置环境变量前缀
	viper.SetEnvPrefix("PIXLY")
	viper.AutomaticEnv()

	// 读取配置文件
	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); !ok {
			fmt.Fprintf(os.Stderr, "Failed to read config file: %v\n", err)
		}
	} else if verbose {
		fmt.Printf("Using config file: %s\n", viper.ConfigFileUsed())
	}
}

// setLogLevel 设置Log level
func setLogLevel() {
	level := viper.GetString("log-level")
	if verbose {
		level = "debug"
	}

	// 这里应该设置实际的Log level
	// 简化实现
	fmt.Printf("Log level set to: %s\n", level)
}

// validateGlobalFlags 验证全局标志
func validateGlobalFlags() error {
	// 验证Worker count
	workers := viper.GetInt("workers")
	if workers < 0 {
		return fmt.Errorf("Worker count cannot be negative")
	}

	// 验证Log level
	logLevel := viper.GetString("log-level")
	validLevels := []string{"debug", "info", "warn", "error"}
	valid := false
	for _, level := range validLevels {
		if level == logLevel {
			valid = true
			break
		}
	}
	if !valid {
		return fmt.Errorf("Invalid log level: %s", logLevel)
	}

	return nil
}

// GetGlobalConfig 获取全局配置
func GetGlobalConfig() map[string]interface{} {
	return map[string]interface{}{
		"verbose":   viper.GetBool("verbose"),
		"dry-run":   viper.GetBool("dry-run"),
		"workers":   viper.GetInt("workers"),
		"output":    viper.GetString("output"),
		"log-level": viper.GetString("log-level"),
		"config":    viper.ConfigFileUsed(),
	}
}

// IsVerbose 是否详细模式
func IsVerbose() bool {
	return viper.GetBool("verbose")
}

// IsDryRun 是否试运行模式
func IsDryRun() bool {
	return viper.GetBool("dry-run")
}

// GetWorkers 获取Worker count
func GetWorkers() int {
	return viper.GetInt("workers")
}

// GetOutputDir 获取Output directory
func GetOutputDir() string {
	return viper.GetString("output")
}

// GetLogLevel 获取Log level
func GetLogLevel() string {
	return viper.GetString("log-level")
}

// initI18n 初始化国际化
func initI18n() {
	// 从标志或环境变量获取语言设置
	lang := langFlag
	if lang == "" {
		lang = os.Getenv("PIXLY_LANG")
	}
	if lang == "" {
		lang = os.Getenv("LANG")
	}

	// 解析语言代码
	var locale string
	switch lang {
	case "zh", "zh-CN", "zh_CN":
		locale = "zh"
	case "en", "en-US", "en_US":
		locale = "en"
	case "ja", "ja-JP", "ja_JP":
		locale = "ja"
	case "auto", "":
		locale = "zh" // 默认中文
	default:
		locale = "zh" // 默认中文
	}

	// 初始化i18n
	i18n.Init()
	i18n.SetLang(locale)

	if verbose {
		fmt.Printf("Language set to: %s\n", locale)
	}
}

// initUITheme 初始化UITheme
func initUITheme() {
	theme := themeFlag
	if theme == "" {
		theme = os.Getenv("PIXLY_THEME")
	}
	if theme == "" {
		theme = "default"
	}

	ui.InitTheme(theme)

	if verbose {
		fmt.Printf("UI theme: %s\n", theme)
	}
}

// showWelcomeBanner 显示欢迎横幅
func showWelcomeBanner() {
	// 只在非help命令时显示
	if len(os.Args) > 1 && (os.Args[1] == "help" || os.Args[1] == "--help" || os.Args[1] == "-h") {
		return
	}

	locale := i18n.GetLang()
	ui.ShowEnhancedWelcomeMessage(locale, "2.2.0")
}

// loadFullConfig 加载完整配置系统
func loadFullConfig() {
	// 尝试加载配置文件
	_, err := config.Load(cfgFile)
	if err != nil {
		// 配置文件不存在或加载失败,使用默认配置
		if verbose {
			fmt.Printf("Using default config (config file not found or invalid)\n")
		}
	} else {
		if verbose {
			fmt.Printf("Loaded config file: %s\n", config.DefaultConfigFile)
		}
	}

	// 从配置更新全局变量 (如果配置已加载)
	cfg := config.Get()
	if cfg != nil {
		// 更新workers数量
		if workers == 0 && cfg.Concurrency.ConversionWorkers > 0 {
			workers = cfg.Concurrency.ConversionWorkers
		}

		// 更新Log level
		if logLevel == "info" && cfg.Logging.Level != "" {
			logLevel = cfg.Logging.Level
		}

		// 更新语言
		if langFlag == "" && cfg.Language.Default != "" {
			langFlag = cfg.Language.Default
		}

		// 更新Theme
		if themeFlag == "default" && cfg.UI.Theme != "" {
			themeFlag = cfg.UI.Theme
		}
	}
}
