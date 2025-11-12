package main

import (
	"fmt"
	"os"
	"runtime"
	"strings"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"

	"pixly/cmd/commands"
)

const version = "5.2.0"
const description = "Next-generation image conversion suite with AI-powered optimization"

// 样式定义
var (
	titleStyle = lipgloss.NewStyle().
			Bold(true).
			Foreground(lipgloss.Color("#7D56F4")).
			Background(lipgloss.Color("#282828")).
			Padding(0, 1)

	headerStyle = lipgloss.NewStyle().
			Bold(true).
			Foreground(lipgloss.Color("#FAFAFA")).
			BorderStyle(lipgloss.RoundedBorder()).
			BorderForeground(lipgloss.Color("#7D56F4")).
			Padding(0, 1)

	selectedStyle = lipgloss.NewStyle().
			Bold(true).
			Foreground(lipgloss.Color("#7D56F4")).
			Background(lipgloss.Color("#3C3C3C")).
			Padding(0, 1)

	normalStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#FAFAFA")).
			Padding(0, 1)

	infoStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#7D56F4"))

	successStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#04B575"))

	warningStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#FFB86C"))

	helpStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#626262"))
)

// 菜单项
type menuItem struct {
	title   string
	content func() string
}

// Model 定义
type model struct {
	menuItems    []menuItem
	selectedItem int
	width        int
	height       int
}

func initialModel() model {
	return model{
		menuItems: []menuItem{
			{T("menu.system_info"), getSystemInfo},
			{T("menu.usage_guide"), getUsageGuide},
			{T("menu.plugin_integration"), getPluginIntegration},
			{T("menu.ai_features"), getAIFeatures},
			{T("menu.quick_start"), getQuickStart},
			{T("menu.documentation"), getDocumentation},
			{T("menu.best_practices"), getBestPractices},
			{T("menu.troubleshooting"), getTroubleshooting},
		},
		selectedItem: 0,
	}
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height
		return m, nil

	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q", "esc":
			return m, tea.Quit

		case "up", "k":
			if m.selectedItem > 0 {
				m.selectedItem--
			}

		case "down", "j":
			if m.selectedItem < len(m.menuItems)-1 {
				m.selectedItem++
			}

		case "home":
			m.selectedItem = 0

		case "end":
			m.selectedItem = len(m.menuItems) - 1
		}
	}

	return m, nil
}

func (m model) View() string {
	if m.width == 0 {
		return "Loading..."
	}

	var b strings.Builder

	// 标题Banner
	banner := `
╔═══════════════════════════════════════════════════════════════╗
║   _____ _____  __   ____    __  __                           ║
║  |  __ \_   _| \ \ / /\ \  / / / /                           ║
║  | |__) || |    \ V /  \ \/ / / /                            ║
║  |  ___/ | |     > <    \  / / /                             ║
║  | |    _| |_   / . \   / / / /__                            ║
║  |_|   |_____| /_/ \_\ /_/ /_____|                           ║
║                                                               ║
║  Pixly - ` + T("banner.title") + `                            ║
║  ` + T("banner.version") + `: v` + version + ` | ` + T("banner.status") + `: Production Ready               ║
╚═══════════════════════════════════════════════════════════════╝
`
	b.WriteString(titleStyle.Render(banner))
	b.WriteString("\n\n")

	// 左侧菜单栏
	menuWidth := 35
	contentWidth := m.width - menuWidth - 4

	menuContent := headerStyle.Render(T("menu.navigation")) + "\n\n"
	for i, item := range m.menuItems {
		cursor := "  "
		if i == m.selectedItem {
			cursor = "▶ "
			menuContent += selectedStyle.Render(cursor+item.title) + "\n"
		} else {
			menuContent += normalStyle.Render(cursor+item.title) + "\n"
		}
	}

	menuContent += "\n" + helpStyle.Render(T("menu.help.nav"))

	// 右侧内容区
	contentHeader := headerStyle.Render(m.menuItems[m.selectedItem].title)
	contentBody := m.menuItems[m.selectedItem].content()

	// 组合左右布局
	menuBox := lipgloss.NewStyle().
		Width(menuWidth).
		Height(m.height - 10).
		BorderStyle(lipgloss.RoundedBorder()).
		BorderForeground(lipgloss.Color("#7D56F4")).
		Padding(1).
		Render(menuContent)

	contentBox := lipgloss.NewStyle().
		Width(contentWidth).
		Height(m.height - 10).
		BorderStyle(lipgloss.RoundedBorder()).
		BorderForeground(lipgloss.Color("#04B575")).
		Padding(1).
		Render(contentHeader + "\n\n" + contentBody)

	layout := lipgloss.JoinHorizontal(lipgloss.Top, menuBox, contentBox)
	b.WriteString(layout)

	return b.String()
}

// 内容生成函数
func getSystemInfo() string {
	var b strings.Builder
	b.WriteString(infoStyle.Render(T("system.version")+":      ") + version + "\n")
	b.WriteString(infoStyle.Render(T("system.platform")+":      ") + fmt.Sprintf("%s/%s", runtime.GOOS, runtime.GOARCH) + "\n")
	b.WriteString(infoStyle.Render(T("system.go_version")+":    ") + runtime.Version() + "\n")
	b.WriteString(infoStyle.Render(T("system.work_dir")+":  ") + getCurrentDir() + "\n\n")

	b.WriteString(successStyle.Render("✅ " + T("system.core_status") + "\n"))
	b.WriteString("  • " + T("system.ai_service") + ": Ready\n")
	b.WriteString("  • " + T("system.database") + ": SQLite\n")
	b.WriteString("  • " + T("system.python_bridge") + ": " + T("system.configured") + "\n")
	b.WriteString("  • " + T("system.model_files") + ": " + T("system.trained") + "\n")

	return b.String()
}

func getUsageGuide() string {
	var b strings.Builder

	b.WriteString(headerStyle.Render(T("usage.plugin.title")) + "\n\n")
	b.WriteString("  " + successStyle.Render("✓") + " " + T("usage.plugin.feature1") + "\n")
	b.WriteString("  " + successStyle.Render("✓") + " " + T("usage.plugin.feature2") + "\n")
	b.WriteString("  " + successStyle.Render("✓") + " " + T("usage.plugin.feature3") + "\n")
	b.WriteString("  " + successStyle.Render("✓") + " " + T("usage.plugin.feature4") + "\n\n")
	b.WriteString("  " + infoStyle.Render(T("usage.plugin.location")) + " plugin_v3/\n")
	b.WriteString("  " + infoStyle.Render(T("usage.plugin.entry")) + " plugin_v3/index.html\n\n")

	b.WriteString(headerStyle.Render(T("usage.aiService.title")) + "\n\n")
	b.WriteString("  " + successStyle.Render("✓") + " RESTful API\n")
	b.WriteString("  " + successStyle.Render("✓") + " " + T("usage.aiService.feature1") + "\n")
	b.WriteString("  " + successStyle.Render("✓") + " " + T("usage.aiService.feature2") + "\n")
	b.WriteString("  " + successStyle.Render("✓") + " " + T("usage.aiService.feature3") + "\n\n")
	b.WriteString("  " + infoStyle.Render(T("usage.aiService.start")) + " go run cmd/ai-service/main.go\n")
	b.WriteString("  " + infoStyle.Render(T("usage.aiService.port")) + " http://localhost:50052\n")

	return b.String()
}

func getPluginIntegration() string {
	var b strings.Builder

	b.WriteString(T("integ.intro") + "\n\n")

	b.WriteString(successStyle.Render("✅ Smart format recommendation\n"))
	b.WriteString("  " + T("integ.formatRec.feature1") + "\n")
	b.WriteString("  " + T("integ.formatRec.feature2") + "\n\n")

	b.WriteString(successStyle.Render(T("integ.paramPred.title") + "\n"))
	b.WriteString("  " + T("integ.paramPred.feature1") + "\n")
	b.WriteString("  " + T("integ.paramPred.feature2") + "\n")
	b.WriteString("  " + T("integ.paramPred.feature3") + "\n\n")

	b.WriteString(successStyle.Render(T("integ.ssimVal.title") + "\n"))
	b.WriteString("  " + T("integ.ssimVal.feature1") + "\n")
	b.WriteString("  " + T("integ.ssimVal.feature2") + "\n")
	b.WriteString("  " + T("integ.ssimVal.feature3") + "\n\n")

	b.WriteString(successStyle.Render(T("integ.qualityAnalysis.title") + "\n"))
	b.WriteString("  " + T("integ.qualityAnalysis.feature1") + "\n")
	b.WriteString("  " + T("integ.qualityAnalysis.feature2") + "\n")
	b.WriteString("  " + T("integ.qualityAnalysis.feature3") + "\n\n")

	b.WriteString(infoStyle.Render(T("integ.configLocation") + ": ") + "plugin_v3/index.html\n")

	return b.String()
}

func getAIFeatures() string {
	var b strings.Builder

	b.WriteString(T("ai.title") + "\n\n")

	b.WriteString(headerStyle.Render(T("ai.layer1.title")) + "\n")
	b.WriteString("  ├─ " + T("ai.layer1.basic") + "\n")
	b.WriteString("  └─ " + T("ai.layer1.advanced") + "\n\n")

	b.WriteString(headerStyle.Render(T("ai.layer2.title")) + "\n")
	b.WriteString("  ├─ " + T("ai.layer2.strategy") + "\n")
	b.WriteString("  ├─ " + T("ai.layer2.state") + "\n")
	b.WriteString("  ├─ " + T("ai.layer2.action") + "\n")
	b.WriteString("  └─ " + T("ai.layer2.learning") + "\n\n")

	b.WriteString(headerStyle.Render(T("ai.layer3.title")) + "\n")
	b.WriteString("  ├─ " + T("ai.layer3.memory") + "\n")
	b.WriteString("  └─ " + T("ai.layer3.sqlite") + "\n\n")

	b.WriteString(headerStyle.Render(T("ai.layer4.title")) + "\n")
	b.WriteString("  ├─ " + T("ai.layer4.config") + "\n")
	b.WriteString("  ├─ " + T("ai.layer4.checkpoint") + "\n")
	b.WriteString("  └─ " + T("ai.layer4.stats") + "\n\n")

	b.WriteString(headerStyle.Render(T("ai.layer5.title")) + "\n")
	b.WriteString("  ├─ " + T("ai.layer5.recommend") + "\n")
	b.WriteString("  ├─ " + T("ai.layer5.confidence") + "\n")
	b.WriteString("  └─ " + T("ai.layer5.fallback") + "\n")

	return b.String()
}

func getQuickStart() string {
	var b strings.Builder

	b.WriteString(headerStyle.Render(T("quick.step1.title")) + "\n")
	b.WriteString("  " + infoStyle.Render("$") + " cd cmd/ai-service\n")
	b.WriteString("  " + infoStyle.Render("$") + " go run main.go --port 50052\n\n")

	b.WriteString(headerStyle.Render(T("quick.step2.title")) + "\n")
	b.WriteString("  • " + T("quick.step2.open") + "\n")
	b.WriteString("  • " + T("quick.step2.menu") + "\n")
	b.WriteString("  • " + T("quick.step2.select") + " plugin_v3/\n\n")

	b.WriteString(headerStyle.Render(T("quick.step3.title")) + "\n")
	b.WriteString("  • " + T("quick.step3.select") + "\n")
	b.WriteString("  • " + T("quick.step3.rightClick") + "\n")
	b.WriteString("  • " + T("quick.step3.mode") + "\n")
	b.WriteString("  • " + T("quick.step3.ai") + "\n")
	b.WriteString("  • " + T("quick.step3.start") + "\n\n")

	b.WriteString(headerStyle.Render(T("quick.step4.title")) + "\n")
	b.WriteString("  " + infoStyle.Render("$") + " cd tools\n")
	b.WriteString("  " + infoStyle.Render("$") + " python3 train_ai_models.py\n")

	return b.String()
}

func getDocumentation() string {
	var b strings.Builder

	b.WriteString(T("docs.title") + "\n\n")

	docs := []struct {
		file string
		desc string
	}{
		{"docs/PRECISION_MODES_v4.2.0.md", T("docs.precision")},
		{"docs/PRECISION_MODES_INTEGRATION_GUIDE.md", T("docs.integration")},
		{"docs/PLUGIN_INTEGRATION_v4.2.0.md", T("docs.plugin")},
		{"docs/AI_FEATURES_ALIGNMENT.md", T("docs.alignment")},
		{"docs/NEW_FEATURES_v4.2.0_COMPLETE.md", T("docs.features")},
		{"QUICK_REFERENCE.md", T("docs.reference")},
	}

	for _, doc := range docs {
		b.WriteString(infoStyle.Render("  • ") + doc.file + "\n")
		b.WriteString("    → " + doc.desc + "\n\n")
	}

	b.WriteString("\n" + warningStyle.Render("💡 " + T("docs.hint.title") + "\n"))
	b.WriteString("  " + T("docs.hint.usage") + "\n")
	b.WriteString("  " + T("docs.hint.aiService") + "\n")
	b.WriteString("  " + T("docs.hint.config") + "\n")

	return b.String()
}

func getBestPractices() string {
	var b strings.Builder

	b.WriteString(successStyle.Render(T("best.title") + "\n\n"))

	b.WriteString(headerStyle.Render(T("best.aiMgmt.title")) + "\n")
	b.WriteString("  • " + T("best.aiMgmt.check") + "\n")
	b.WriteString("  • " + T("best.aiMgmt.graceful") + "\n")
	b.WriteString("  • " + T("best.aiMgmt.monitor") + "\n\n")

	b.WriteString(headerStyle.Render(T("best.precision.title")) + "\n")
	b.WriteString("  • " + T("best.precision.preview") + "\n")
	b.WriteString("  • " + T("best.precision.final") + "\n")
	b.WriteString("  • " + T("best.precision.batch") + "\n\n")

	b.WriteString(headerStyle.Render(T("best.balance.title")) + "\n")
	b.WriteString("  • " + T("best.balance.size") + "\n")
	b.WriteString("  • " + T("best.balance.balanced") + "\n")
	b.WriteString("  • " + T("best.balance.quality") + "\n\n")

	b.WriteString(headerStyle.Render(T("best.learning.title")) + "\n")
	b.WriteString("  • " + T("best.learning.feedback") + "\n")
	b.WriteString("  • " + T("best.learning.ppo") + "\n")
	b.WriteString("  • " + T("best.learning.track") + "\n")

	return b.String()
}

func getTroubleshooting() string {
	var b strings.Builder

	b.WriteString(warningStyle.Render("⚠️  " + T("trouble.common") + "\n\n"))

	b.WriteString(headerStyle.Render(T("trouble.issue1.title")) + "\n")
	b.WriteString("  " + T("trouble.issue1.symptom") + ": AI service not available\n")
	b.WriteString("  " + T("trouble.solution") + ":\n")
	b.WriteString("    " + infoStyle.Render("$") + " ps aux | grep ai-service\n")
	b.WriteString("    " + infoStyle.Render("$") + " go run cmd/ai-service/main.go\n\n")

	b.WriteString(headerStyle.Render(T("trouble.issue2.title")) + "\n")
	b.WriteString("  " + T("trouble.issue2.symptom") + ": ModuleNotFoundError\n")
	b.WriteString("  " + T("trouble.solution") + ":\n")
	b.WriteString("    " + infoStyle.Render("$") + " pip3 install -r requirements.txt\n\n")

	b.WriteString(headerStyle.Render(T("trouble.issue3.title")) + "\n")
	b.WriteString("  " + T("trouble.issue3.symptom") + ": Model file not found\n")
	b.WriteString("  " + T("trouble.solution") + ":\n")
	b.WriteString("    " + infoStyle.Render("$") + " cd tools\n")
	b.WriteString("    " + infoStyle.Render("$") + " python3 train_ai_models.py\n\n")

	b.WriteString(headerStyle.Render(T("trouble.issue4.title")) + "\n")
	b.WriteString("  " + T("trouble.issue4.symptom") + ": SSIM validation failed\n")
	b.WriteString("  " + T("trouble.solution") + ":\n")
	b.WriteString("    • " + T("trouble.issue4.check1") + "\n")
	b.WriteString("    • " + T("trouble.issue4.check2") + "\n")
	b.WriteString("    • " + T("trouble.issue4.check3") + "\n")

	return b.String()
}

func getCurrentDir() string {
	dir, err := os.Getwd()
	if err != nil {
		return T("sys.unknown")
	}
	return dir
}

func main() {
	// Initialize language based on environment
	currentLang = GetLanguage()

	// 如果有命令行参数，使用CLI模式
	if len(os.Args) > 1 {
		commands.Execute()
		return
	}

	// 否则启动TUI交互界面（双击启动）
	p := tea.NewProgram(initialModel(), tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		fmt.Printf("%s: %v\n", T("sys.startupFailed"), err)
		os.Exit(1)
	}
}
