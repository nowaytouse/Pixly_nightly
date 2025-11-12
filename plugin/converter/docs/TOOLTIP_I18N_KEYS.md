# PIXLY Tooltip 国际化键值清单

## 📝 说明

所有tooltip现已使用 `data-i18n="[title]key"` 语法绑定到Eagle的i18n系统，支持多语言显示。

---

## 🔑 Tooltip i18n 键值列表

### 1. 转换类型

```json
{
  "conversion": {
    "imageTooltip": "转换图片格式（JPEG、PNG等）",
    "videoTooltip": "转换视频格式（MP4、MOV等）"
  }
}
```

**英文参考**:
```json
{
  "conversion": {
    "imageTooltip": "Convert image formats (JPEG, PNG, etc.)",
    "videoTooltip": "Convert video formats (MP4, MOV, etc.)"
  }
}
```

---

### 2. 智能模式预设

```json
{
  "smartMode": {
    "generalTooltip": "快速处理 - 基础AI预测，适合批量转换",
    "balancedTooltip": "平衡模式 - ML + SSIM校验，日常推荐",
    "qualityTooltip": "质量优先 - 深度AI + PPO优化 + 多项质量验证"
  }
}
```

**英文参考**:
```json
{
  "smartMode": {
    "generalTooltip": "Fast processing - Basic AI prediction, suitable for batch conversion",
    "balancedTooltip": "Balanced mode - ML + SSIM validation, recommended for daily use",
    "qualityTooltip": "Quality first - Deep AI + PPO optimization + multiple quality validations"
  }
}
```

---

### 3. 视频AI预设

```json
{
  "videoAI": {
    "fastTooltip": "快速模式 - 场景检测，适合快速编码",
    "balancedTooltip": "平衡模式 - 场景检测 + VMAF校验，日常推荐",
    "fullTooltip": "完整模式 - 场景检测 + 时间预测 + VMAF校验 + 深度分析"
  }
}
```

**英文参考**:
```json
{
  "videoAI": {
    "fastTooltip": "Fast mode - Scene detection, suitable for quick encoding",
    "balancedTooltip": "Balanced mode - Scene detection + VMAF validation, recommended for daily use",
    "fullTooltip": "Full mode - Scene detection + time prediction + VMAF validation + deep analysis"
  }
}
```

---

### 4. 视频AI高级选项

```json
{
  "videoAI": {
    "sceneDetectionTooltip": "场景检测：AI自动识别视频中的场景切换点，在关键帧处插入I帧，优化压缩率和随机访问性能。适合有明显场景变化的视频。",
    "forceTransformerTooltip": "Transformer精细处理：强制使用Transformer深度学习模型进行视频分析，提供更精准的质量预测和参数优化。适合对画质要求极高的场景，但会增加处理时间。",
    "vmafValidationTooltip": "VMAF质量验证：转换完成后，使用VMAF（Video Multimethod Assessment Fusion）算法对比原始视频和转换后的视频，确保视觉质量在可接受范围内。适合对画质要求严格的场景。"
  }
}
```

**英文参考**:
```json
{
  "videoAI": {
    "sceneDetectionTooltip": "Scene Detection: AI automatically identifies scene changes in the video, inserts I-frames at key frames to optimize compression ratio and random access performance. Suitable for videos with obvious scene changes.",
    "forceTransformerTooltip": "Transformer Processing: Force use of Transformer deep learning model for video analysis, providing more accurate quality prediction and parameter optimization. Suitable for scenarios with extremely high image quality requirements, but will increase processing time.",
    "vmafValidationTooltip": "VMAF Validation: After conversion, use VMAF (Video Multimethod Assessment Fusion) algorithm to compare the original and converted videos, ensuring visual quality is within acceptable range. Suitable for scenarios with strict quality requirements."
  }
}
```

---

### 5. 快捷工具

```json
{
  "tools": {
    "fileValidationTooltip": "使用Google Magika AI模型检测文件真实格式，防止伪造文件",
    "formatCorrectionTooltip": "当文件扩展名与实际格式不符时，自动修正为正确的扩展名"
  }
}
```

**英文参考**:
```json
{
  "tools": {
    "fileValidationTooltip": "Use Google Magika AI model to detect real file format and prevent fake files",
    "formatCorrectionTooltip": "Automatically correct to the correct extension when file extension does not match actual format"
  }
}
```

---

### 6. 格式标签（Header）

```json
{
  "format": {
    "jxl": {
      "tooltip": "JPEG XL - 新一代图像格式，支持无损压缩"
    },
    "avif": {
      "tooltip": "AVIF - 基于AV1的现代图像格式"
    },
    "webp": {
      "tooltip": "WebP - Google开发的高效图像格式"
    },
    "heic": {
      "tooltip": "HEIC - Apple设备广泛支持的格式"
    }
  }
}
```

**英文参考**:
```json
{
  "format": {
    "jxl": {
      "tooltip": "JPEG XL - Next-generation image format with lossless compression"
    },
    "avif": {
      "tooltip": "AVIF - Modern image format based on AV1"
    },
    "webp": {
      "tooltip": "WebP - Efficient image format developed by Google"
    },
    "heic": {
      "tooltip": "HEIC - Format widely supported by Apple devices"
    }
  }
}
```

---

### 7. 手动模式格式选择（详细版）

```json
{
  "format": {
    "jxl": {
      "tooltipFull": "JPEG XL - 新一代图像格式，支持无损压缩，比PNG小30-50%，比JPEG小10-20%，支持透明度和动画"
    },
    "avif": {
      "tooltipFull": "AVIF - 基于AV1视频编码的现代图像格式，压缩效率优秀，特别适合动图和Web应用"
    },
    "webp": {
      "tooltipFull": "WebP - Google开发的高效图像格式，浏览器兼容性好，支持无损和有损压缩"
    },
    "heic": {
      "tooltipFull": "HEIC - Apple设备广泛支持的格式，在iOS/macOS上完美兼容，适合iCloud照片库和Apple设备间共享"
    }
  }
}
```

**英文参考**:
```json
{
  "format": {
    "jxl": {
      "tooltipFull": "JPEG XL - Next-generation image format with lossless compression, 30-50% smaller than PNG, 10-20% smaller than JPEG, supports transparency and animation"
    },
    "avif": {
      "tooltipFull": "AVIF - Modern image format based on AV1 video codec, excellent compression efficiency, especially suitable for animations and web applications"
    },
    "webp": {
      "tooltipFull": "WebP - Efficient image format developed by Google, good browser compatibility, supports both lossless and lossy compression"
    },
    "heic": {
      "tooltipFull": "HEIC - Format widely supported by Apple devices, perfect compatibility on iOS/macOS, suitable for iCloud Photo Library and sharing between Apple devices"
    }
  }
}
```

---

### 8. 空状态步骤

```json
{
  "empty": {
    "selectFilesTooltip": "在Eagle中选择要转换的图片或视频文件",
    "selectModeTooltip": "选择图像或视频处理模式",
    "startConversionTooltip": "配置参数后点击转换按钮开始处理"
  }
}
```

**英文参考**:
```json
{
  "empty": {
    "selectFilesTooltip": "Select images or videos to convert in Eagle",
    "selectModeTooltip": "Choose image or video processing mode",
    "startConversionTooltip": "Click the convert button to start processing after configuring parameters"
  }
}
```

---

## 📊 统计

### 已实现的tooltip i18n键值
- **转换类型**: 2个
- **智能模式预设**: 3个
- **视频AI预设**: 3个
- **视频AI高级选项**: 3个
- **快捷工具**: 2个
- **格式标签（Header）**: 4个
- **手动格式选择**: 4个
- **空状态步骤**: 3个
- **视频编码器**: 4个
- **视频容器格式**: 4个
- **AI高级选项**: 4个 (智能质量预测 + 自动参数优化 + SSIM验证 + 动图转视频)
- **智能模式信息**: 2个
- **Header选择器**: 2个 (日志级别 + 语言切换)

**总计**: 46个tooltip i18n键值

---

## 9. 视频编码器

```json
{
  "encoder": {
    "h265TooltipFull": "H.265 (HEVC) - 最成熟的现代编码器，压缩率比H.264高30-50%，广泛支持硬件加速，兼容性优秀",
    "h266TooltipFull": "H.266 (VVC) - 最新编码标准，压缩率比H.265高30-50%，画质更优但编码速度较慢，适合追求极致压缩的场景",
    "av1TooltipFull": "AV1 - 开源次世代编码器，压缩率与H.266相当，适合Web和流媒体应用，硬件加速逐渐普及",
    "proresTooltipFull": "ProRes - Apple专业后期编码器，无损或接近无损质量，文件大但编辑性能极佳，适合专业视频制作"
  }
}
```

**英文参考**:
```json
{
  "encoder": {
    "h265TooltipFull": "H.265 (HEVC) - Most mature modern codec, 30-50% better compression than H.264, widely supported hardware acceleration, excellent compatibility",
    "h266TooltipFull": "H.266 (VVC) - Latest encoding standard, 30-50% better compression than H.265, superior quality but slower encoding, suitable for extreme compression scenarios",
    "av1TooltipFull": "AV1 - Open-source next-gen codec, compression comparable to H.266, suitable for web and streaming applications, hardware acceleration gradually becoming popular",
    "proresTooltipFull": "ProRes - Apple professional post-production codec, lossless or near-lossless quality, large files but excellent editing performance, suitable for professional video production"
  }
}
```

---

## 10. 视频容器格式

```json
{
  "video": {
    "mp4TooltipFull": "MP4 - 最广泛兼容的容器格式，支持所有主流播放器和设备，适合分享和网络传播",
    "movTooltipFull": "MOV - Apple QuickTime容器，在iOS/macOS上完美支持，适合Apple生态内使用",
    "webmTooltipFull": "WebM - Google开发的Web优化容器，适合网页嵌入和流媒体播放",
    "mkvTooltipFull": "MKV - 通用开源容器，支持几乎所有编码器和字幕格式，功能最全面"
  }
}
```

**英文参考**:
```json
{
  "video": {
    "mp4TooltipFull": "MP4 - Most widely compatible container format, supported by all mainstream players and devices, suitable for sharing and online distribution",
    "movTooltipFull": "MOV - Apple QuickTime container, perfect support on iOS/macOS, suitable for use within Apple ecosystem",
    "webmTooltipFull": "WebM - Google's web-optimized container, suitable for web embedding and streaming playback",
    "mkvTooltipFull": "MKV - Universal open-source container, supports almost all codecs and subtitle formats, most comprehensive features"
  }
}
```

---

## 11. AI高级选项

```json
{
  "ai": {
    "smartQualityTooltip": "智能质量预测：AI会分析图像的纹理、边缘、色彩复杂度等特征，自动预测最适合的质量参数（quality值）。避免过度压缩导致画质损失，也避免质量过高导致文件过大。适合不确定最佳质量参数的场景。"
  }
}
```

**英文参考**:
```json
{
  "ai": {
    "smartQualityTooltip": "Smart Quality Prediction: AI analyzes image texture, edges, color complexity and other features to automatically predict the most suitable quality parameter (quality value). Avoids image quality loss from over-compression and excessively large files from too high quality. Suitable for scenarios where optimal quality parameter is uncertain."
  }
}
```

---

## 12. 智能模式信息

```json
{
  "smartMode": {
    "generalInfo": "通用模式状态",
    "generalRuleBasedDesc": "🔧 规则路由 | 🚀 极速处理"
  }
}
```

**英文参考**:
```json
{
  "smartMode": {
    "generalInfo": "General Mode Status",
    "generalRuleBasedDesc": "🔧 Rule-based Routing | 🚀 Ultra-fast Processing"
  }
}
```

---

**总计**: 46个tooltip i18n键值

---

## 13. Header选择器

```json
{
  "log": {
    "levelSelector": "日志级别选择器 - 选择输出日志的详细程度"
  },
  "language": {
    "switchLanguage": "切换语言 - Switch Language"
  }
}
```

**英文参考**:
```json
{
  "log": {
    "levelSelector": "Log Level Selector - Choose output log verbosity"
  },
  "language": {
    "switchLanguage": "Switch Language"
  }
}
```

---

## 14. AI高级选项（补充）

```json
{
  "ai": {
    "autoOptimizeTooltip": "自动参数优化：AI会根据输入格式、目标格式、图像尺寸等信息，自动调整编码器的内部参数（如effort、speed等），在压缩率和质量之间找到最佳平衡点。推荐大多数场景下保持开启。包含贝叶斯优化（自动启用，样本数充足时生效）。",
    "ssimValidationTooltip": "SSIM质量验证：转换完成后，使用结构相似性指数（SSIM）算法对比原始图像和转换后的图像，确保画质损失在可接受范围内。如果相似度过低（如<0.95），会在日志中警告。适合对画质要求严格的场景。",
    "videoForAnimationTooltip": "动图转视频：当检测到大型动图（如高分辨率GIF或超长WebP动画）时，AI会智能推荐转换为视频格式（如MP4/WebM），可大幅减小文件体积，提升播放性能。关闭此选项则始终保持原格式转换。"
  }
}
```

**英文参考**:
```json
{
  "ai": {
    "autoOptimizeTooltip": "Auto Parameter Optimization: AI automatically adjusts encoder internal parameters (such as effort, speed) based on input format, target format, and image dimensions to find the best balance between compression ratio and quality. Recommended to keep enabled in most scenarios. Includes Bayesian optimization (automatically enabled when sufficient samples available).",
    "ssimValidationTooltip": "SSIM Quality Validation: After conversion, uses Structural Similarity Index (SSIM) algorithm to compare original and converted images, ensuring quality loss is within acceptable range. If similarity is too low (e.g., <0.95), a warning is logged. Suitable for scenarios with strict quality requirements.",
    "videoForAnimationTooltip": "Animation to Video: When detecting large animated images (such as high-resolution GIFs or long WebP animations), AI will intelligently recommend converting to video format (such as MP4/WebM), which can significantly reduce file size and improve playback performance. Disabling this option will always maintain original format conversion."
  }
}
```

---

**总计**: 46个tooltip i18n键值

---

## 🎯 实施方式

### HTML绑定语法

```html
<!-- 单个绑定 -->
<button data-i18n="conversion.image;[title]conversion.imageTooltip">
    📷 图像转换
</button>

<!-- 仅tooltip绑定 -->
<label data-i18n="[title]smartMode.generalTooltip">
    <span data-i18n="smartMode.general">🔧 通用</span>
</label>
```

### Eagle i18n系统自动处理

Eagle的i18n系统会自动：
1. 根据当前语言加载对应的翻译
2. 将翻译值设置到`title`属性
3. 当语言切换时自动更新tooltip文本

---

## ✅ 优势

### 1. 多语言支持
- ✅ 中文
- ✅ 英文
- ✅ 其他语言（待添加）

### 2. 自动切换
- ✅ 跟随Eagle语言设置
- ✅ 无需手动处理
- ✅ 实时更新

### 3. 统一管理
- ✅ 所有文本集中在语言包
- ✅ 易于维护和更新
- ✅ 避免硬编码

---

## 📝 添加新tooltip

### 步骤

1. **在HTML中添加绑定**
   ```html
   <button data-i18n="[title]newFeature.tooltip">
       新功能
   </button>
   ```

2. **在语言包中添加键值**
   ```json
   {
     "newFeature": {
       "tooltip": "这是新功能的说明"
     }
   }
   ```

3. **添加英文翻译**
   ```json
   {
     "newFeature": {
       "tooltip": "This is a description of the new feature"
     }
   }
   ```

---

**更新时间**: 2025-11-09  
**版本**: PIXLY v3.0.0  
**状态**: ✅ 所有tooltip已支持i18n
