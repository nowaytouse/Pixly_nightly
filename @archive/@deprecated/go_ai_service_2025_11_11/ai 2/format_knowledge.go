package ai

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 格式特性知识库 - 预设种子库
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// FormatKnowledge 格式知识
type FormatKnowledge struct {
	Name           string              `json:"name"`
	FullName       string              `json:"full_name"`
	Year           int                 `json:"year"`
	Category       string              `json:"category"` // modern, legacy, video
	Strengths      []string            `json:"strengths"`
	Weaknesses     []string            `json:"weaknesses"`
	UseCases       []string            `json:"use_cases"`
	TechSpecs      map[string]string   `json:"tech_specs"`
	CompressionType string             `json:"compression_type"` // lossy, lossless, hybrid
	QualityRange   QualityRange        `json:"quality_range"`
	RecommendedFor []string            `json:"recommended_for"`
}

// QualityRange 质量范围
type QualityRange struct {
	Min            int     `json:"min"`
	Max            int     `json:"max"`
	Default        int     `json:"default"`
	Lossless       bool    `json:"lossless"`
	LosslessQValue int     `json:"lossless_q_value"` // 无损对应的quality值
}

// FormatKnowledgeBase 格式知识库
var FormatKnowledgeBase = map[string]*FormatKnowledge{
	"jxl": {
		Name:     "JXL",
		FullName: "JPEG XL",
		Year:     2021,
		Category: "modern",
		Strengths: []string{
			"最佳的压缩效率（比JPEG小30-60%）",
			"支持无损压缩",
			"支持渐进式解码",
			"支持动画和多层",
			"HDR和广色域支持",
			"优秀的有损质量",
			"快速编解码",
			"可从JPEG无损转换",
		},
		Weaknesses: []string{
			"浏览器支持有限（Safari 17+, Firefox需手动启用）",
			"生态系统还在发展中",
			"部分老旧设备不支持",
		},
		UseCases: []string{
			"网络图片分发（高质量+小体积）",
			"相册存档（无损压缩）",
			"HDR摄影作品",
			"需要渐进式加载的场景",
			"JPEG替代方案",
		},
		TechSpecs: map[string]string{
			"max_resolution":  "1,073,741,823 × 1,073,741,823",
			"color_depth":     "32-bit per channel",
			"alpha_support":   "yes",
			"animation":       "yes",
			"metadata":        "Exif, XMP, JUMBF",
		},
		CompressionType: "hybrid",
		QualityRange: QualityRange{
			Min:            60,
			Max:            100,
			Default:        90,
			Lossless:       true,
			LosslessQValue: 100,
		},
		RecommendedFor: []string{"photography", "web", "archival", "hdr"},
	},
	
	"avif": {
		Name:     "AVIF",
		FullName: "AV1 Image File Format",
		Year:     2019,
		Category: "modern",
		Strengths: []string{
			"卓越的压缩率（比JPEG小50%+）",
			"优秀的细节保留",
			"广泛的浏览器支持",
			"HDR和广色域",
			"10-bit色深支持",
			"渐进式解码",
		},
		Weaknesses: []string{
			"编码速度慢（CPU密集）",
			"解码性能一般",
			"工具链不够成熟",
			"在某些场景可能出现色带",
		},
		UseCases: []string{
			"现代网站（Chrome, Firefox, Safari支持）",
			"移动应用",
			"社交媒体图片",
			"需要极致压缩的场景",
		},
		TechSpecs: map[string]string{
			"max_resolution":  "65,536 × 65,536",
			"color_depth":     "12-bit per channel",
			"alpha_support":   "yes",
			"animation":       "yes",
			"metadata":        "Exif, XMP",
		},
		CompressionType: "lossy",
		QualityRange: QualityRange{
			Min:            50,
			Max:            100,
			Default:        80,
			Lossless:       false,
			LosslessQValue: -1,
		},
		RecommendedFor: []string{"web", "mobile", "social_media"},
	},
	
	"webp": {
		Name:     "WebP",
		FullName: "WebP",
		Year:     2010,
		Category: "modern",
		Strengths: []string{
			"良好的压缩效率",
			"支持无损和有损压缩",
			"支持透明度",
			"支持动画",
			"几乎全浏览器支持",
			"编解码速度快",
		},
		Weaknesses: []string{
			"压缩率不如JXL/AVIF",
			"有损模式细节损失较多",
			"无损压缩效率一般",
			"色彩处理不如现代格式",
		},
		UseCases: []string{
			"网站图标和UI元素",
			"需要透明度的场景",
			"GIF替代（动画WebP）",
			"兼容性优先的场景",
		},
		TechSpecs: map[string]string{
			"max_resolution":  "16,383 × 16,383",
			"color_depth":     "8-bit per channel",
			"alpha_support":   "yes",
			"animation":       "yes",
			"metadata":        "Exif, XMP, ICC",
		},
		CompressionType: "hybrid",
		QualityRange: QualityRange{
			Min:            60,
			Max:            100,
			Default:        85,
			Lossless:       true,
			LosslessQValue: 100,
		},
		RecommendedFor: []string{"web", "ui", "icons", "animation"},
	},
	
	"jpeg": {
		Name:     "JPEG",
		FullName: "Joint Photographic Experts Group",
		Year:     1992,
		Category: "legacy",
		Strengths: []string{
			"通用兼容性（所有设备支持）",
			"成熟的生态系统",
			"快速编解码",
			"可控的文件大小",
		},
		Weaknesses: []string{
			"有损压缩（无法恢复原始数据）",
			"不支持透明度",
			"不支持动画",
			"重复编辑会累积损失",
			"压缩效率低于现代格式",
		},
		UseCases: []string{
			"照相机输出",
			"需要最大兼容性的场景",
			"老旧设备和软件",
			"临时文件和预览",
		},
		TechSpecs: map[string]string{
			"max_resolution":  "65,535 × 65,535",
			"color_depth":     "8-bit per channel",
			"alpha_support":   "no",
			"animation":       "no",
			"metadata":        "Exif, IPTC, XMP",
		},
		CompressionType: "lossy",
		QualityRange: QualityRange{
			Min:            60,
			Max:            100,
			Default:        90,
			Lossless:       false,
			LosslessQValue: -1,
		},
		RecommendedFor: []string{"compatibility", "camera", "legacy"},
	},
	
	"png": {
		Name:     "PNG",
		FullName: "Portable Network Graphics",
		Year:     1996,
		Category: "legacy",
		Strengths: []string{
			"无损压缩",
			"支持透明度（8-bit alpha）",
			"通用兼容性",
			"适合图表和UI",
		},
		Weaknesses: []string{
			"文件体积大（照片场景）",
			"不支持动画",
			"压缩效率远低于现代格式",
		},
		UseCases: []string{
			"UI元素和图标",
			"需要透明度的图形",
			"截图和图表",
			"需要无损的简单图像",
		},
		TechSpecs: map[string]string{
			"max_resolution":  "2,147,483,647 × 2,147,483,647",
			"color_depth":     "16-bit per channel",
			"alpha_support":   "yes",
			"animation":       "no (use APNG)",
			"metadata":        "text chunks, Exif",
		},
		CompressionType: "lossless",
		QualityRange: QualityRange{
			Min:            0, // compression level
			Max:            9,
			Default:        6,
			Lossless:       true,
			LosslessQValue: -1,
		},
		RecommendedFor: []string{"ui", "graphics", "transparency", "screenshots"},
	},
	
	"heic": {
		Name:     "HEIC",
		FullName: "High Efficiency Image Container",
		Year:     2015,
		Category: "modern",
		Strengths: []string{
			"优秀的压缩效率（比JPEG小40-50%）",
			"Apple生态默认格式",
			"支持多图层和动画",
			"16-bit色深",
			"支持透明度",
		},
		Weaknesses: []string{
			"Windows/Android兼容性差",
			"需要专利授权",
			"浏览器不支持",
			"转换工具有限",
		},
		UseCases: []string{
			"iPhone/iPad照片",
			"Apple设备间分享",
			"需要高效存储的场景",
		},
		TechSpecs: map[string]string{
			"max_resolution":  "8,192 × 8,192 (typical)",
			"color_depth":     "16-bit per channel",
			"alpha_support":   "yes",
			"animation":       "yes",
			"metadata":        "Exif, XMP",
		},
		CompressionType: "lossy",
		QualityRange: QualityRange{
			Min:            50,
			Max:            100,
			Default:        85,
			Lossless:       false,
			LosslessQValue: -1,
		},
		RecommendedFor: []string{"apple_ecosystem", "mobile", "storage"},
	},
}

// GetFormatKnowledge 获取格式知识
func GetFormatKnowledge(format string) *FormatKnowledge {
	return FormatKnowledgeBase[format]
}

// CompareFormats 对比两个格式
func CompareFormats(format1, format2 string) map[string]interface{} {
	k1 := GetFormatKnowledge(format1)
	k2 := GetFormatKnowledge(format2)
	
	if k1 == nil || k2 == nil {
		return nil
	}
	
	comparison := map[string]interface{}{
		"format1": k1,
		"format2": k2,
		"winner": map[string]string{
			"compression":    determineBetter(k1, k2, "compression"),
			"quality":        determineBetter(k1, k2, "quality"),
			"compatibility":  determineBetter(k1, k2, "compatibility"),
			"speed":          determineBetter(k1, k2, "speed"),
		},
		"recommendation": generateRecommendation(k1, k2),
	}
	
	return comparison
}

// determineBetter 判断哪个格式在某方面更好
func determineBetter(k1, k2 *FormatKnowledge, aspect string) string {
	// 简化实现，实际应该有更复杂的评分系统
	scores := map[string]map[string]int{
		"compression": {
			"jxl":  95,
			"avif": 98,
			"webp": 80,
			"jpeg": 60,
			"png":  40,
			"heic": 85,
		},
		"quality": {
			"jxl":  95,
			"avif": 90,
			"webp": 75,
			"jpeg": 70,
			"png":  100,
			"heic": 85,
		},
		"compatibility": {
			"jxl":  40,
			"avif": 75,
			"webp": 90,
			"jpeg": 100,
			"png":  100,
			"heic": 50,
		},
		"speed": {
			"jxl":  85,
			"avif": 50,
			"webp": 90,
			"jpeg": 95,
			"png":  80,
			"heic": 70,
		},
	}
	
	score1 := scores[aspect][k1.Name]
	score2 := scores[aspect][k2.Name]
	
	if score1 > score2 {
		return k1.Name
	} else if score2 > score1 {
		return k2.Name
	}
	return "tie"
}

// generateRecommendation 生成推荐
func generateRecommendation(k1, k2 *FormatKnowledge) string {
	// 简化实现
	if k1.Category == "modern" && k2.Category == "legacy" {
		return k1.Name + " recommended (better compression and features)"
	} else if k2.Category == "modern" && k1.Category == "legacy" {
		return k2.Name + " recommended (better compression and features)"
	}
	return "Both are good choices, depends on your use case"
}

// GetRecommendedQuality 获取推荐质量值
func GetRecommendedQuality(format string, scenario string) int {
	knowledge := GetFormatKnowledge(format)
	if knowledge == nil {
		return 85 // 默认值
	}
	
	// 根据场景调整
	adjustments := map[string]int{
		"web":         -5,  // 网络传输，稍低质量
		"archival":    +10, // 存档，最高质量
		"social_media": -10, // 社交媒体，更激进压缩
		"print":       +5,  // 打印，高质量
		"preview":     -15, // 预览，低质量快速
	}
	
	baseQuality := knowledge.QualityRange.Default
	adjustment := adjustments[scenario]
	
	result := baseQuality + adjustment
	
	// 限制在有效范围内
	if result < knowledge.QualityRange.Min {
		result = knowledge.QualityRange.Min
	}
	if result > knowledge.QualityRange.Max {
		result = knowledge.QualityRange.Max
	}
	
	return result
}
