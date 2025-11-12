# PIXLY i18n & Tooltip 改进总结

## ✅ 已完成的改进

### 1. 转换类型按钮 Tooltip
- ✅ **图像转换按钮**: "转换图片格式（JPEG、PNG等）"
- ✅ **视频处理按钮**: "转换视频格式（MP4、MOV等）"

### 2. 图像智能模式预设 Tooltip
- ✅ **通用模式**: "快速处理 - 基础AI预测，适合批量转换"
- ✅ **平衡模式**: "平衡模式 - ML + SSIM校验，日常推荐"
- ✅ **质量模式**: "质量优先 - 深度AI + PPO优化 + 多项质量验证"

### 3. 视频AI快捷预设 Tooltip
- ✅ **快速模式**: "快速模式 - 场景检测，适合快速编码"
- ✅ **平衡模式**: "平衡模式 - 场景检测 + VMAF校验，日常推荐"
- ✅ **完整模式**: "完整模式 - 场景检测 + 时间预测 + VMAF校验 + 深度分析"

### 4. 快捷工具 Tooltip
- ✅ **AI文件验证**: "使用Google Magika AI模型检测文件真实格式，防止伪造文件"
- ✅ **自动修正格式**: "当文件扩展名与实际格式不符时，自动修正为正确的扩展名"

### 5. 已有的提示文本
- ✅ XMP自动合并提示（含tooltip）
- ✅ 文件名自动规范提示（含tooltip）
- ✅ Skip Live Photos（含title绑定）
- ✅ 清空文件列表（含title绑定）

---

## 📋 建议后续添加的 Tooltip

### 优先级：高

#### 格式选择按钮
```html
<!-- 图像格式 -->
<button title="JPEG XL - 新一代图像格式，支持无损压缩">JXL</button>
<button title="AVIF - 基于AV1的现代图像格式">AVIF</button>
<button title="WebP - Google开发的高效图像格式">WebP</button>
<button title="HEIC - Apple设备广泛支持">HEIC</button>

<!-- 视频格式 -->
<button title="H.264 - 广泛兼容的视频编码">H.264</button>
<button title="H.265/HEVC - 新一代高效编码">H.265</button>
<button title="AV1 - 开源免专利的高效编码">AV1</button>
```

#### 转换按钮
```html
<button title="开始转换所选文件（快捷键：Ctrl+Enter）">开始转换</button>
<button title="切换到JPEG→JXL无损模式">JPEG→JXL</button>
```

### 优先级：中

#### 高级选项开关
- LightGBM质量预测
- PPO强化学习  
- SSIM/VMAF质量验证
- 场景检测
- 时间预测

#### 视频参数
- CRF滑块
- 编码速度
- 精度模式

### 优先级：低

#### 调试功能
- 日志级别选择器
- 性能监控开关

---

## 🌐 i18n 键值建议

### 需要Eagle国际化支持的键

```javascript
{
  // 转换类型
  "conversion.image": "图像转换",
  "conversion.video": "视频处理",
  "conversion.imageTooltip": "转换图片格式（JPEG、PNG等）",
  "conversion.videoTooltip": "转换视频格式（MP4、MOV等）",
  
  // 智能模式预设
  "smartMode.title": "智能模式",
  "smartMode.general": "通用",
  "smartMode.balanced": "平衡",
  "smartMode.quality": "质量",
  "smartMode.generalTooltip": "快速处理 - 基础AI预测，适合批量转换",
  "smartMode.balancedTooltip": "平衡模式 - ML + SSIM校验，日常推荐",
  "smartMode.qualityTooltip": "质量优先 - 深度AI + PPO优化 + 多项质量验证",
  
  // 视频AI预设
  "videoAI.preset": "快捷预设",
  "videoAI.fast": "快速",
  "videoAI.balanced": "平衡",
  "videoAI.full": "完整",
  "videoAI.fastTooltip": "快速模式 - 场景检测，适合快速编码",
  "videoAI.balancedTooltip": "平衡模式 - 场景检测 + VMAF校验，日常推荐",
  "videoAI.fullTooltip": "完整模式 - 场景检测 + 时间预测 + VMAF校验 + 深度分析",
  
  // 快捷工具
  "tools.title": "快捷工具",
  "tools.fileValidation": "AI文件验证",
  "tools.fileValidationDesc": "Magika AI 安全检测",
  "tools.fileValidationTooltip": "使用Google Magika AI模型检测文件真实格式，防止伪造文件",
  "tools.formatCorrection": "自动修正格式",
  "tools.formatCorrectionDesc": "扩展名不符时修复",
  "tools.formatCorrectionTooltip": "当文件扩展名与实际格式不符时，自动修正为正确的扩展名",
  
  // 提示文本
  "hints.xmpAutoMerge": "XMP自动合并",
  "hints.xmpTooltip": "检测到.xmp文件，将自动合并到对应图像并删除XMP文件",
  "hints.fileNameNormalize": "文件名自动规范",
  "hints.fileNameTooltip": "处理中自动规范文件名（移除特殊字符），输出后还原原始名称"
}
```

---

## 📊 统计

- ✅ **已添加tooltip**: 11个关键UI元素
- 📝 **建议添加**: ~20个额外元素
- 🌐 **i18n键建议**: ~30个翻译键
- 📄 **创建文档**: 2个（清单 + 总结）

---

## 🎯 实施建议

### 短期（立即）
1. ✅ 已完成：关键按钮tooltip
2. 建议：为格式选择按钮添加tooltip
3. 建议：为转换按钮添加tooltip

### 中期（可选）
1. 完整的i18n键值定义
2. 英文翻译支持
3. 更多高级选项tooltip

### 长期（优化）
1. 动态tooltip内容（根据上下文变化）
2. 多语言tooltip支持
3. 交互式帮助系统

---

## 📝 使用说明

### 查看tooltip
所有添加的tooltip都使用HTML原生 `title` 属性，鼠标悬停即可显示。

### 扩展tooltip
使用以下模式添加新tooltip：
```html
<!-- 基本tooltip -->
<button title="这是提示文本">按钮</button>

<!-- 带i18n的tooltip -->
<button title="tooltip text" data-i18n="[title]key.tooltip">按钮</button>

<!-- 使用data-tooltip（自定义系统） -->
<span class="tooltip-trigger" data-tooltip="提示内容">元素</span>
```

---

## ✨ 改进效果

### 用户体验提升
- ✅ 首次使用更友好
- ✅ 功能说明更清晰
- ✅ 降低学习成本
- ✅ 减少误操作

### 可访问性
- ✅ 屏幕阅读器支持（title属性）
- ✅ 键盘导航友好
- ✅ 多语言准备

---

生成时间: 2025-11-09
版本: PIXLY v3.0.0
