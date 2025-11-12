// cmd/pixly-cli/ui/i18n.go - TUI Internationalization
//
// 功能：
// - TUI 界面国际化支持
// - 支持语言: 中文简体、英文、繁体中文、日文
//
// 版本: v3.0.0

package ui

import (
	"os"
	"strings"
)

// Language 语言类型
type Language string

const (
	LangEN   Language = "en"    // English
	LangZHCN Language = "zh_CN" // 中文简体
	LangZHTW Language = "zh_TW" // 繁体中文
	LangJA   Language = "ja_JP" // 日本語
)

// Messages 国际化消息
var Messages = map[Language]map[string]string{
	LangEN: {
		"banner": `
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║     PIXLY CLI v3.0.0 - Unified Image Converter              ║
║     GO + Rust Architecture                                  ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝

Powered by:
  • GO AI Service    - Smart parameter prediction
  • Rust Converter   - High-performance conversion
  • Eagle Integration - Seamless workflow

Replaces all standalone tools:
  ✅ all2avif, all2jxl, all2webp
  ✅ dynamic2avif, dynamic2jxl, dynamic2mov
  ✅ static2avif, static2jxl
  ✅ video2mov, deduplicate_media, merge_xmp`,
		"converting":        "Converting",
		"preparing":         "Preparing",
		"ai_prediction":     "AI prediction",
		"preserving_metadata": "Preserving metadata",
		"conversion_complete": "Conversion complete!",
		"input":             "Input",
		"output":            "Output",
		"time":              "Time",
		"health_check":      "PIXLY Service Health Check",
		"rust_service":      "Rust Service",
		"ai_service":        "GO AI Service",
		"available":         "Available",
		"not_available":     "Not available",
		"start_hint":        "Start",
	},
	LangZHCN: {
		"banner": `
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║     PIXLY CLI v3.0.0 - 统一图像转换器                        ║
║     GO + Rust 架构                                          ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝

驱动力:
  • GO AI 服务    - 智能参数预测
  • Rust 转换器   - 高性能转换
  • Eagle 集成    - 无缝工作流

替代所有独立工具:
  ✅ all2avif, all2jxl, all2webp
  ✅ dynamic2avif, dynamic2jxl, dynamic2mov
  ✅ static2avif, static2jxl
  ✅ video2mov, deduplicate_media, merge_xmp`,
		"converting":        "转换中",
		"preparing":         "准备中",
		"ai_prediction":     "AI 预测",
		"preserving_metadata": "保留元数据",
		"conversion_complete": "转换完成！",
		"input":             "输入",
		"output":            "输出",
		"time":              "时间",
		"health_check":      "PIXLY 服务健康检查",
		"rust_service":      "Rust 服务",
		"ai_service":        "GO AI 服务",
		"available":         "可用",
		"not_available":     "不可用",
		"start_hint":        "启动",
	},
	LangZHTW: {
		"banner": `
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║     PIXLY CLI v3.0.0 - 統一圖像轉換器                        ║
║     GO + Rust 架構                                          ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝

驅動力:
  • GO AI 服務    - 智能參數預測
  • Rust 轉換器   - 高性能轉換
  • Eagle 集成    - 無縫工作流

替代所有獨立工具:
  ✅ all2avif, all2jxl, all2webp
  ✅ dynamic2avif, dynamic2jxl, dynamic2mov
  ✅ static2avif, static2jxl
  ✅ video2mov, deduplicate_media, merge_xmp`,
		"converting":        "轉換中",
		"preparing":         "準備中",
		"ai_prediction":     "AI 預測",
		"preserving_metadata": "保留元數據",
		"conversion_complete": "轉換完成！",
		"input":             "輸入",
		"output":            "輸出",
		"time":              "時間",
		"health_check":      "PIXLY 服務健康檢查",
		"rust_service":      "Rust 服務",
		"ai_service":        "GO AI 服務",
		"available":         "可用",
		"not_available":     "不可用",
		"start_hint":        "啟動",
	},
	LangJA: {
		"banner": `
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║     PIXLY CLI v3.0.0 - 統合画像変換ツール                    ║
║     GO + Rust アーキテクチャ                                 ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝

機能:
  • GO AI サービス    - スマートパラメータ予測
  • Rust 変換器       - 高性能変換
  • Eagle 統合        - シームレスなワークフロー

すべてのスタンドアロンツールを置き換え:
  ✅ all2avif, all2jxl, all2webp
  ✅ dynamic2avif, dynamic2jxl, dynamic2mov
  ✅ static2avif, static2jxl
  ✅ video2mov, deduplicate_media, merge_xmp`,
		"converting":        "変換中",
		"preparing":         "準備中",
		"ai_prediction":     "AI 予測",
		"preserving_metadata": "メタデータを保持",
		"conversion_complete": "変換完了！",
		"input":             "入力",
		"output":            "出力",
		"time":              "時間",
		"health_check":      "PIXLY サービス健康チェック",
		"rust_service":      "Rust サービス",
		"ai_service":        "GO AI サービス",
		"available":         "利用可能",
		"not_available":     "利用不可",
		"start_hint":        "起動",
	},
}

// CurrentLang 当前语言
var CurrentLang Language = LangEN

// DetectLanguage 自动检测系统语言
func DetectLanguage() Language {
	// 从环境变量检测
	lang := os.Getenv("LANG")
	if lang == "" {
		lang = os.Getenv("LC_ALL")
	}
	
	lang = strings.ToLower(lang)
	
	if strings.Contains(lang, "zh_cn") || strings.Contains(lang, "zh-cn") {
		return LangZHCN
	}
	if strings.Contains(lang, "zh_tw") || strings.Contains(lang, "zh-tw") {
		return LangZHTW
	}
	if strings.Contains(lang, "ja") {
		return LangJA
	}
	
	return LangEN
}

// T 翻译函数
func T(key string) string {
	if msg, ok := Messages[CurrentLang][key]; ok {
		return msg
	}
	// Fallback to English
	if msg, ok := Messages[LangEN][key]; ok {
		return msg
	}
	return key
}

// SetLanguage 设置语言
func SetLanguage(lang Language) {
	CurrentLang = lang
}

// Init 初始化国际化
func Init() {
	CurrentLang = DetectLanguage()
}
