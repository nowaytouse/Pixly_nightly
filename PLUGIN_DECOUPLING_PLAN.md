# Eagle UI 插件解耦方案

## 📋 目标

将现有Eagle插件拆分为两个独立版本，各司其职：

### 1️⃣ Pixly Converter（当前版本）
**定位**: 强大深入专业健全万能的现代格式手动转换器

**核心功能**:
- ✅ 手动格式转换（Image/Video/Audio）
- ✅ 精确参数控制（质量、速度、努力值）
- ✅ 批量处理
- ✅ 格式支持全面（JXL, AVIF, WebP, HEVC, AV1, Opus, etc.）
- ✅ 专业用户导向

**移除功能**:
- ❌ AI智能推荐
- ❌ 自动格式选择
- ❌ 智能参数优化
- ❌ 预处理建议

**UI特点**:
- 直观的参数控制面板
- 详细的技术参数选项
- 专业模式（高级选项展开）
- 适合摄影师、设计师、专业用户

---

### 2️⃣ Pixly AI Optimizer（新建版本）
**定位**: AI智能加持下，质量极其优先前提下，必然减小空间占用的智能优化器

**核心功能**:
- ✅ AI智能格式推荐
- ✅ 自动参数优化
- ✅ 质量保证（无损转码检测）
- ✅ 智能编码器升级（H.264→H.265, MP3→Opus）
- ✅ 一键优化（最少用户干预）
- ✅ Magika AI安全检测

**核心原则**:
- **质量优先**: 绝不牺牲质量
- **必然减小**: 空间占用必然减小（智能检测）
- **极简操作**: 一键完成
- **AI驱动**: 全程AI决策

**UI特点**:
- 极简界面（只有优化模式选择）
- 智能推荐展示（格式、原因）
- 优化结果预览
- 适合普通用户、快速优化场景

---

## 🏗️ 技术架构

### 当前版本（Converter）改造

```
plugin/converter/          → 保留，移除AI相关
├── manifest.json          → 更新定位和描述
├── js/
│   ├── main.js           → 移除AI调用代码
│   ├── ui-builder.js     → 保留手动参数控制
│   └── converter.js      → 纯转换逻辑
└── _locales/             → 更新本地化文案
```

**移除内容**:
- AI预测API调用
- 智能推荐UI组件
- 自动格式选择逻辑

**保留内容**:
- Rust转换器调用
- 参数控制界面
- 批量处理逻辑

---

### 新版本（AI Optimizer）结构

```
plugin/ai-optimizer/       → 新建
├── manifest.json          → AI优化器配置
├── icon.png               → 新图标（带AI标识）
├── js/
│   ├── main.js           → 极简主逻辑
│   ├── ai-client.js      → Python AI API调用
│   └── optimizer.js      → 优化流程控制
├── css/
│   └── minimal.css       → 极简样式
└── _locales/
    ├── en.json
    ├── zh_CN.json
    └── zh_TW.json
```

**核心流程**:
```
用户选择文件
  ↓
选择优化模式（size/balanced/quality）
  ↓
AI分析（format recommendation + params）
  ↓
展示推荐（格式、原因、预期效果）
  ↓
用户确认
  ↓
自动优化（Rust执行）
  ↓
显示结果（对比、统计）
```

---

## 📦 实施步骤

### Phase 1: 清理当前版本（Converter）
1. ✅ 移除Python AI调用代码
2. ✅ 移除智能推荐UI
3. ✅ 更新manifest.json定位
4. ✅ 更新_locales本地化
5. ✅ 测试手动转换功能

### Phase 2: 创建AI版本（AI Optimizer）
1. ⏳ 复制基础结构
2. ⏳ 实现AI客户端（调用Python AI HTTP API）
3. ⏳ 设计极简UI
4. ⏳ 实现优化流程
5. ⏳ 添加Magika安全检测
6. ⏳ 测试完整流程

### Phase 3: 文档和发布
1. ⏳ 更新用户文档
2. ⏳ 添加两个版本的使用指南
3. ⏳ 更新README.md
4. ⏳ 准备发布包

---

## 🎯 用户场景对比

### Converter（手动版）
**场景**: 摄影师需要将RAW转换为特定格式
- 打开Converter插件
- 选择目标格式（如JXL）
- 精确设置质量参数（95）
- 设置努力值（9，最高质量）
- 批量转换
- 完全掌控

### AI Optimizer（智能版）
**场景**: 用户想压缩相册减少存储
- 打开AI Optimizer
- 选择"balanced"模式
- AI自动分析并推荐（JPEG→JXL lossless）
- 显示"无损转码，体积减少40%"
- 点击确认
- 自动完成

---

## 💡 命名建议

### 当前版本
- **名称**: Pixly Converter Pro
- **副标题**: Professional Modern Format Converter
- **图标**: 工具/参数图标

### 新AI版本
- **名称**: Pixly AI Optimizer
- **副标题**: AI-Powered Smart Media Optimizer
- **图标**: AI/智能图标（带光效）

---

## ⚠️ 注意事项

1. **不要重复造轮子**: 两个插件共享Rust转换器
2. **保持独立**: 两个插件可以同时安装
3. **明确定位**: 避免功能重叠混淆用户
4. **文档清晰**: 明确说明两者区别和适用场景
5. **用户选择**: 让用户根据需求选择合适版本

---

## 📝 待办清单

- [ ] 清理Converter版本AI代码
- [ ] 更新Converter manifest和本地化
- [ ] 创建AI Optimizer目录结构
- [ ] 实现AI Optimizer核心逻辑
- [ ] 设计AI Optimizer UI
- [ ] 测试两个版本
- [ ] 更新文档
- [ ] 准备发布

---

**原则**: 功能专一 > 大而全，让用户选择 > 强制决策
