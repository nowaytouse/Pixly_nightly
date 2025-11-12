# PIXLY i18n 国际化更新清单

## 🎯 需要添加的翻译键

### 1. 智能模式预设 (Smart Mode Presets)
```json
{
  "smartMode": {
    "title": "智能模式",
    "general": "通用",
    "balanced": "平衡",
    "quality": "质量",
    "generalDesc": "快速处理 - 基础AI预测",
    "balancedDesc": "平衡：ML + balanced (Q85) + SSIM 校验 - 日常推荐",
    "qualityDesc": "完整：深度AI + PPO优化 + 多项质量验证"
  }
}
```

### 2. 视频AI预设 (Video AI Presets)
```json
{
  "videoAI": {
    "preset": "快捷预设",
    "fast": "快速",
    "balanced": "平衡",
    "full": "完整",
    "fastDesc": "场景检测 - 快速编码",
    "balancedDesc": "平衡：场景检测 + VMAF校验 - 日常推荐",
    "fullDesc": "完整：场景检测 + 时间预测 + VMAF校验 + 深度分析"
  }
}
```

### 3. 提示文本 (Hints & Tooltips)
```json
{
  "hints": {
    "xmpAutoMerge": "XMP自动合并",
    "xmpAutoMergeTooltip": "检测到.xmp文件，将自动合并到对应图像并删除XMP文件",
    "fileNameNormalize": "文件名自动规范",
    "fileNameNormalizeTooltip": "处理中自动规范文件名（移除特殊字符），输出后还原原始名称"
  }
}
```

### 4. 空状态提示 (Empty State)
```json
{
  "empty": {
    "noFiles": "还没有选择文件",
    "selectFilesPrompt": "请在 Eagle 中选择要转换的图片或视频文件，然后返回此插件开始转换。",
    "selectFiles": "选择文件",
    "selectFilesDesc": "在 Eagle 中选择图片或视频",
    "selectMode": "选择模式",
    "selectModeDesc": "智能模式或手动模式",
    "startConversion": "开始转换",
    "startConversionDesc": "点击转换按钮即可"
  }
}
```

### 5. 文件面板 (File Panel)
```json
{
  "files": {
    "title": "已选择的文件",
    "skipLivePhotos": "跳过Live Photos",
    "skipLivePhotosTitle": "启用后将跳过.mov配对文件，仅处理静态图像",
    "clearAll": "清空列表",
    "clearAllTooltip": "清除所有已选择的文件"
  }
}
```

### 6. 转换锁定提示 (Conversion Lock)
```json
{
  "conversionLock": {
    "title": "本轮转换已完成",
    "message": "请在Eagle中重新选择<strong>不同的文件</strong>以开始下一轮转换",
    "button": "我知道了"
  }
}
```

### 7. 快捷工具 (Quick Tools)
```json
{
  "tools": {
    "title": "快捷工具",
    "aiValidation": "AI文件验证",
    "aiValidationDesc": "使用AI检测并验证文件格式",
    "formatCorrection": "格式校正",
    "formatCorrectionDesc": "自动修复格式错误"
  }
}
```

### 8. 按钮悬浮提示 (Button Tooltips)
需要为以下元素添加 `title` 或 `data-tooltip` 属性：

- 图像/视频模式切换按钮
- 智能模式预设按钮 (通用/平衡/质量)
- 视频AI预设按钮 (快速/平衡/完整)
- 格式选择按钮 (JXL/AVIF/WebP/HEIC)
- 转换按钮下拉菜单
- 清空文件列表按钮
- Skip Live Photos开关

---

## 📝 实施建议

### 方案1：使用Eagle i18next（推荐）
在Eagle的语言包中添加上述翻译键，通过 `data-i18n` 属性自动绑定。

### 方案2：添加内联tooltip
为重要按钮添加 `title` 属性或 `data-tooltip` + tooltip系统。

---

## ✅ 已完成的i18n
- 基础UI文本（按钮、标签）
- 转换状态提示
- 错误消息

## ❌ 缺失的i18n
- 新增的智能模式预设
- 视频AI预设
- 各种tooltip提示
- 空状态页面文本
- 转换锁定提示

---

## 🎯 优先级
1. **高优先级**: 按钮tooltip（用户体验直接相关）
2. **中优先级**: 预设模式文本
3. **低优先级**: 错误提示和调试信息
