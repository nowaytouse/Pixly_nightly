# 🎯 两个完全独立的插件

## ✅ 问题已修复

### 问题
Eagle 通过 `manifest.json` 中的 `id` 字段识别插件。如果两个插件的 ID 相同，Eagle 会认为是同一个插件！

### 解决方案
已经为两个插件设置了**完全不同的 ID**：

---

## 📦 插件 1: PIXLY AI - 智能多媒体处理

### 位置
```
plugin/ai/
```

### manifest.json
```json
{
    "id": "pixly-ai-smart-converter-ml",
    "name": "PIXLY AI - 智能多媒体处理",
    "logo": "logo.png"
}
```

### 核心特点
- 🤖 **AI 驱动** - 机器学习自动优化
- ✨ **自动化** - 从 Eagle 自动获取文件
- 🎯 **智能推荐** - AI 自动推荐格式和参数
- ⚡ **极简界面** - 3 步完成转换

### 功能
1. 自动从 Eagle 获取选中的文件
2. 自动检测媒体类型（图像/动图/视频/RAW）
3. AI 智能推荐最优格式
4. AI 智能预测质量参数
5. 自动 SSIM/VMAF 验证
6. 自动 GPU 加速

---

## 📦 插件 2: PIXLY Format - 专业格式转换

### 位置
```
plugin/format/
```

### manifest.json
```json
{
    "id": "pixly-format-professional-manual",
    "name": "PIXLY Format - 专业格式转换",
    "logo": "logo.png"
}
```

### 核心特点
- ⚙️ **手动控制** - 完全控制所有参数
- 📊 **参数丰富** - 30+ 高级参数
- 🎨 **格式全面** - JXL/AVIF/HEIC 完整参数
- 🎬 **视频专业** - H.265/H.266/AV1 完整支持

### 功能
1. 手动选择文件
2. 手动选择格式（JXL/AVIF/HEIC）
3. 手动调整质量（1-100）
4. 手动设置所有高级参数
5. 视频编码完整控制
6. 专业级参数调优

---

## 🔑 关键区别

| 特性 | AI 版本 | Format 版本 |
|------|---------|-------------|
| **插件 ID** | `pixly-ai-smart-converter-ml` | `pixly-format-professional-manual` |
| **插件名称** | PIXLY AI - 智能多媒体处理 | PIXLY Format - 专业格式转换 |
| **文件选择** | 自动从 Eagle 获取 | 手动选择 |
| **格式选择** | AI 自动推荐 | 手动选择 |
| **参数控制** | AI 自动优化 | 手动调整 |
| **界面** | 极简 | 专业 |
| **目标用户** | 普通用户 | 专业用户 |

---

## 📁 文件结构

```
plugin/
├── ai/                                    # 插件 1: AI 智能版本
│   ├── manifest.json                     # ID: pixly-ai-smart-converter-ml
│   ├── index.html
│   ├── logo.png                          # ✅
│   ├── css/
│   │   ├── ai-styles.css
│   │   └── tooltip.css
│   ├── js/
│   │   ├── ai-core.js
│   │   └── i18n.js
│   └── _locales/
│       ├── en/messages.json
│       └── zh_CN/messages.json
│
├── format/                                # 插件 2: Format 专业版本
│   ├── manifest.json                     # ID: pixly-format-professional-manual
│   ├── index.html
│   ├── logo.png                          # ✅
│   ├── css/
│   │   └── format-styles.css
│   └── js/
│       └── format-core.js
│
└── old/                                   # 旧版本（参考）
    └── converter/
```

---

## 🚀 在 Eagle 中安装

### 方法 1: 直接安装（推荐）

1. **安装 AI 版本**
   ```
   Eagle → 插件 → 安装本地插件 → 选择 plugin/ai 文件夹
   ```

2. **安装 Format 版本**
   ```
   Eagle → 插件 → 安装本地插件 → 选择 plugin/format 文件夹
   ```

### 方法 2: 开发模式

1. **启用开发者模式**
   ```
   Eagle → 设置 → 插件 → 开发者模式
   ```

2. **加载插件**
   ```
   加载 plugin/ai 文件夹
   加载 plugin/format 文件夹
   ```

---

## ✅ 验证清单

安装后，在 Eagle 插件列表中应该看到：

- [ ] ✅ **PIXLY AI - 智能多媒体处理** (ID: pixly-ai-smart-converter-ml)
- [ ] ✅ **PIXLY Format - 专业格式转换** (ID: pixly-format-professional-manual)

两个插件应该：
- [ ] ✅ 显示不同的名称
- [ ] ✅ 显示不同的图标
- [ ] ✅ 可以同时安装
- [ ] ✅ 可以独立运行
- [ ] ✅ 互不干扰

---

## 🎯 使用场景

### 使用 AI 版本
- ✅ 日常照片批量处理
- ✅ 快速转换和分享
- ✅ 不想调整参数
- ✅ 信任 AI 自动优化

### 使用 Format 版本
- ✅ 专业摄影作品
- ✅ 需要精确控制
- ✅ 特定参数要求
- ✅ 格式实验测试

---

## 🔧 技术细节

### AI 版本核心代码

```javascript
// 自动从 Eagle 获取文件
eagle.onPluginShow(() => {
    setTimeout(() => {
        autoSelectFromEagle();
    }, 100);
});

async function autoSelectFromEagle() {
    const items = await eagle.item.getSelected();
    // 自动处理...
}
```

### Format 版本核心代码

```javascript
// 手动选择文件
document.getElementById('selectFilesBtn').addEventListener('click', () => {
    // 手动选择...
});
```

---

## 📊 对比总结

### AI 版本
- **定位**: 智能自动化
- **操作**: 3 步完成
- **适合**: 普通用户
- **特点**: AI 驱动，自动化

### Format 版本
- **定位**: 专业手动控制
- **操作**: 多步精细调整
- **适合**: 专业用户
- **特点**: 参数丰富，完全控制

---

## ✅ 状态

- ✅ 两个插件 ID 完全不同
- ✅ 两个插件名称完全不同
- ✅ 两个插件功能完全独立
- ✅ 两个插件可以同时安装
- ✅ Logo 文件已复制到两个文件夹

**状态**: 🟢 就绪  
**版本**: V2.1 - Fixed  
**日期**: 2025-01-17
