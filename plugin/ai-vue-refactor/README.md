# PIXLY AI Vue 重构版

## ✅ 阶段 1 完成

### 已实现
- ✅ 官方配色系统（CSS 变量）
- ✅ 默认暗色模式
- ✅ 无边框窗口（manifest.json）
- ✅ 下拉菜单（替代卡片）
- ✅ Rust CLI 集成
- ✅ Eagle API 集成
- ✅ 完整功能选项

### 功能列表
- 🤖 AI 智能选项
  - 优化目标（平衡/质量/体积）
  - 输出格式选择
  - 智能质量预测
  - SSIM 质量验证
  - GPU 硬件加速
  - 智能预处理
- ⚙️ 高级选项
  - 质量滑块
  - 速度选择
  - 数学无损
  - XMP 合并
- 📁 文件管理
  - 文件列表
  - 批量选择
  - 进度显示

## 🚀 使用方法

```bash
cd plugin/ai-vue-refactor
npm install
npm run dev
```

浏览器打开 http://localhost:5173

## 📦 构建

```bash
npm run build
```

将 `dist/` + `manifest.json` + `logo.png` + `bin/` 复制到 Eagle 插件目录。

## 🎨 设计特点

1. **官方配色**：使用 Eagle 官方插件的 CSS 变量
2. **暗色优先**：默认暗色模式
3. **紧凑布局**：下拉菜单替代卡片，节省空间
4. **无边框**：vibrancy + frame: false
5. **完整功能**：参考 old 版本的所有功能

## 📋 下一步

- ⏳ 添加更多格式选项
- ⏳ 实现日志查看
- ⏳ 添加结果统计
- ⏳ 国际化支持
