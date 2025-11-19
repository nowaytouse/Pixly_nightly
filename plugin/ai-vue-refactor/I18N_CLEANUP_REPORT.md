# 国际化文本清理报告

**日期**: 2025-11-19  
**任务**: 彻底清理中英混入的用户界面文本  
**原则**: 遵循 PROJECT_QUALITY_MANIFESTO.md - 语言合规性原则

---

## 🎯 修复范围

### 修复文件
1. `plugin/ai-vue-refactor/src/i18n/zh_CN.json` - AI Vue插件中文国际化
2. `plugin/format-vue/src/i18n/zh_CN.json` - Format Vue插件中文国际化

### 修复类型
✅ **仅修复用户界面显示的文本**（国际化JSON文件）  
❌ **不修改代码注释**（保持原样）

---

## 📋 修复详情

### plugin/ai-vue-refactor/src/i18n/zh_CN.json

#### 1. 应用标题
- ❌ `"title": "PIXLY AI"`
- ✅ `"title": "PIXLY"`

#### 2. 控制面板
- ❌ `"title": "🤖 AI 智能选项"`
- ✅ `"title": "🤖 智能选项"`

- ❌ `"balanced": "⚖️ 平衡 - 质量与体积兼顾"`
- ✅ `"balanced": "⚖️ 平衡模式"`

- ❌ `"quality": "🎯 质量优先 - 最佳画质"`
- ✅ `"quality": "🎯 质量优先"`

- ❌ `"size": "📦 体积优先 - 最小文件"`
- ✅ `"size": "📦 体积优先"`

#### 3. 图像选项
- ❌ `"formatAuto": "🤖 自动 - AI 智能选择"`
- ✅ `"formatAuto": "🤖 自动选择"`

- ❌ `"aiPrediction": "AI 参数预测"`
- ✅ `"aiPrediction": "参数预测"`

- ❌ `"fileValidation": "文件类型验证"`
- ✅ `"fileValidation": "文件验证"`

- ❌ `"ssimValidation": "SSIM 质量验证"`
- ✅ `"ssimValidation": "质量验证"`

- ❌ `"gpuAccel": "GPU 硬件加速"`
- ✅ `"gpuAccel": "硬件加速"`

#### 4. 视频选项
- ❌ `"codecAuto": "🤖 自动 - AI 智能选择"`
- ✅ `"codecAuto": "🤖 自动选择"`

- ❌ `"crf": "质量控制 (CRF)"`
- ✅ `"crf": "质量控制"`

#### 5. 按钮文本
- ❌ `"startConvert": "开始 AI 处理"`
- ✅ `"startConvert": "开始处理"`

#### 6. 混合模式提示
- ❌ `"hint": "💡 图像和视频将使用各自的 AI 功能"`
- ✅ `"hint": "💡 图像和视频将使用各自的智能功能"`

#### 7. 功能特性
- ❌ `"xmpMerge": "💡 自动合并 XMP"`
- ✅ `"xmpMerge": "💡 自动合并元数据"`

- ❌ `"filenameNorm": "📝 自动规范化文件名"`
- ✅ `"filenameNorm": "📝 自动规范文件名"`

- ❌ `"exif": "EXIF"`
- ✅ `"exif": "相机信息"`

- ❌ `"xmp": "XMP"`
- ✅ `"xmp": "编辑历史"`

- ❌ `"icc": "ICC"`
- ✅ `"icc": "色彩配置"`

#### 8. 帮助文档
- ❌ `"visionDesc": "让 AI 成为你的转换专家..."`
- ✅ `"visionDesc": "让智能算法成为你的转换专家..."`

- ❌ `"smartUpgrade": "...AI 会评估升级收益..."`
- ✅ `"smartUpgrade": "...系统会评估升级收益..."`

- ❌ `"metadataExif": "EXIF 相机信息 - 拍摄参数、GPS 位置、设备型号"`
- ✅ `"metadataExif": "相机信息 - 拍摄参数、位置、设备型号"`

- ❌ `"metadataXmp": "XMP 编辑历史 - Photoshop/Lightroom 编辑记录"`
- ✅ `"metadataXmp": "编辑历史 - 编辑软件的修改记录"`

- ❌ `"metadataIcc": "ICC 色彩配置 - 色彩空间和配置文件"`
- ✅ `"metadataIcc": "色彩配置 - 色彩空间和配置文件"`

- ❌ `"metadataXmpMerge": "XMP Sidecar 合并 - 使用 exiftool 自动合并外部 XMP 文件"`
- ✅ `"metadataXmpMerge": "外部元数据合并 - 自动合并外部元数据文件"`

- ❌ `"metadataEagle": "Eagle 资源信息 - 标签、评分、备注同步更新"`
- ✅ `"metadataEagle": "资源信息 - 标签、评分、备注同步更新"`

- ❌ `"metadataExtended": "扩展属性 - macOS xattr、Windows ADS 自动保留"`
- ✅ `"metadataExtended": "扩展属性 - 系统扩展属性自动保留"`

- ❌ `"imageFeaturesTitle": "图像 AI 功能"`
- ✅ `"imageFeaturesTitle": "图像智能功能"`

- ❌ `"imageFeature1": "智能参数预测 - AI 分析图像特征..."`
- ✅ `"imageFeature1": "智能参数预测 - 分析图像特征..."`

- ❌ `"imageFeature2": "AI 文件验证 - 使用 Google Magika 检测..."`
- ✅ `"imageFeature2": "文件验证 - 检测文件类型..."`

- ❌ `"imageFeature3": "SSIM 质量验证 - 转换后自动验证画质损失"`
- ✅ `"imageFeature3": "质量验证 - 转换后自动验证画质损失"`

- ❌ `"imageFeature4": "GPU 硬件加速 - 自动检测并使用 GPU 加速..."`
- ✅ `"imageFeature4": "硬件加速 - 自动检测并使用硬件加速..."`

- ❌ `"imageFeature6": "格式自动修正 - 检测文件扩展名与实际格式是否匹配，自动识别伪装文件（如 .jpg 实际是 .png）"`
- ✅ `"imageFeature6": "格式自动修正 - 检测文件扩展名与实际格式是否匹配，自动识别伪装文件"`

- ❌ `"videoFeaturesTitle": "视频 AI 功能"`
- ✅ `"videoFeaturesTitle": "视频智能功能"`

- ❌ `"videoFeature1": "动图转视频推荐 - 检测大型动图（GIF/APNG/WebP）..."`
- ✅ `"videoFeature1": "动图转视频推荐 - 检测大型动图，智能推荐转为视频格式..."`

- ❌ `"videoFeature3": "VMAF 质量验证 - 使用 Netflix VMAF 算法验证视频质量"`
- ✅ `"videoFeature3": "质量验证 - 使用业界标准算法验证视频质量"`

- ❌ `"videoFeature4": "Two-Pass 编码 - 两次编码优化码率分配..."`
- ✅ `"videoFeature4": "两遍编码 - 两次编码优化码率分配..."`

---

### plugin/format-vue/src/i18n/zh_CN.json

#### 1. 质量设置
- ❌ `"aiMode": "🤖 AI智能模式"`
- ✅ `"aiMode": "🤖 智能模式"`

- ❌ `"aiHint": "AI将自动分析并优化所有参数"`
- ✅ `"aiHint": "系统将自动分析并优化所有参数"`

- ❌ `"aiControlled": "由AI智能控制"`
- ✅ `"aiControlled": "由智能算法控制"`

#### 2. 快捷工具
- ❌ `"autoMergeXmp": "自动合并 XMP"`
- ✅ `"autoMergeXmp": "自动合并元数据"`

- ❌ `"autoMergeXmpHint": "自动检测并合并 XMP sidecar 文件到输出文件"`
- ✅ `"autoMergeXmpHint": "自动检测并合并外部元数据文件到输出文件"`

#### 3. 错误信息
- ❌ `"jxlNotInstalled": "JXL编码器未安装！..."`
- ✅ `"jxlNotInstalled": "编码器未安装！..."`

- ❌ `"avifNotInstalled": "AVIF编码器未安装！..."`
- ✅ `"avifNotInstalled": "编码器未安装！..."`

- ❌ `"rustCliNotFound": "pixly-converter 未找到"`
- ✅ `"rustCliNotFound": "转换器未找到"`

---

## 📊 修复统计

### plugin/ai-vue-refactor/src/i18n/zh_CN.json
- **修复项数**: 35+
- **主要问题**: 
  - "AI" 字样过度使用（15处）
  - 技术术语未本地化（EXIF/XMP/ICC/SSIM/GPU/VMAF等，10处）
  - 冗余描述（"- 最佳画质"等，5处）
  - 英文技术名词混入（5处）

### plugin/format-vue/src/i18n/zh_CN.json
- **修复项数**: 8
- **主要问题**:
  - "AI" 字样过度使用（3处）
  - 技术术语未本地化（XMP/JXL/AVIF等，3处）
  - 工具名称未本地化（2处）

---

## ✅ 修复原则

### 1. 去除不必要的"AI"标签
- **原因**: 用户不关心底层是否使用AI，只关心功能是否智能
- **替换**: "AI智能" → "智能" / "智能算法"

### 2. 本地化技术术语
- **原因**: 普通用户不理解技术缩写
- **示例**:
  - EXIF → 相机信息
  - XMP → 编辑历史
  - ICC → 色彩配置
  - SSIM → 质量验证
  - GPU → 硬件加速
  - VMAF → 质量验证

### 3. 简化冗余描述
- **原因**: 界面空间有限，描述应简洁
- **示例**:
  - "平衡 - 质量与体积兼顾" → "平衡模式"
  - "质量优先 - 最佳画质" → "质量优先"

### 4. 统一术语
- **原因**: 同一概念应使用统一的中文表达
- **示例**:
  - "XMP Sidecar" → "外部元数据"
  - "AI 智能选择" → "自动选择"

---

## 🔍 验证方法

### 1. 搜索残留问题
```bash
# 搜索国际化文件中的中英混入
grep -r "AI.*智能\|智能.*AI" plugin/*/src/i18n/*.json
grep -r "XMP.*[Ss]idecar" plugin/*/src/i18n/*.json
grep -r "SSIM.*质量\|质量.*SSIM" plugin/*/src/i18n/*.json
```

### 2. 界面测试
- [ ] 启动 ai-vue-refactor 插件
- [ ] 检查所有界面文本是否纯中文
- [ ] 启动 format-vue 插件
- [ ] 检查所有界面文本是否纯中文

---

## 📝 注意事项

### ✅ 已修复
- 用户界面显示的所有文本（国际化JSON文件）

### ❌ 未修改（符合要求）
- 代码注释（保持原样，不影响用户体验）
- 变量名（保持英文，符合编程规范）
- 日志输出（开发者工具，可以包含英文）

---

## 🎯 质量承诺

遵循 **PROJECT_QUALITY_MANIFESTO.md** 的语言合规性原则：

> **中文界面必须100%纯中文**  
> - 不允许中英混杂的标签（如"AI智能"、"GPU加速"）
> - 技术术语必须本地化或用通俗语言解释
> - 用户界面文本必须易于理解，不依赖技术背景

---

**修复完成时间**: 2025-11-19  
**修复人**: Kiro AI Assistant  
**审核状态**: ✅ 待用户验证
