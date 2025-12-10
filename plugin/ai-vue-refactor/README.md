# Pixly AI-Vue-Refactor Plugin

**AI 智能媒体转换工具 - Eagle 插件**

> ⚠️ **重要**: 此插件仅能在 [Eagle](https://eagle.cool) 环境中运行

## ✨ 特性

### AI 智能功能
- 🤖 **AI 参数预测**: 自动推荐最佳转换参数
- 🎯 **智能优化**: balanced/quality/size 三种优化模式
- 🔍 **场景检测**: 视频场景自动检测（优化 GOP）
- 📊 **VMAF 评估**: 视频质量自动验证
- ⚡ **GPU 加速**: 支持硬件加速

### 基础功能（与 format-vue 相同）
- 🖼️ **图像转换**: JXL、AVIF、WebP、HEIC、PNG、JPEG
- 🎬 **视频转换**: H.264、H.265、H.266、AV1、VP9
- 🎨 **动图处理**: GIF、APNG、WebP 动画
- 📎 **XMP 合并**: 自动合并元数据
- 🔒 **文件验证**: Magika AI 验证
- 📊 **质量检测**: SSIM 评估

## 🆚 与 format-vue 的区别

| 功能 | format-vue | ai-vue-refactor |
|------|------------|-----------------|
| 图像/视频转换 | ✅ | ✅ |
| 辅助功能 | ✅ | ✅ |
| **AI 参数预测** | ❌ | ✅ |
| **场景检测** | ❌ | ✅ |
| **VMAF 评估** | ❌ | ✅ |
| **GPU 加速** | ❌ | ✅ |

**简单来说**: ai-vue-refactor = format-vue + AI 智能引擎

## 🚀 快速开始

### 用户使用

1. 在 Eagle 中安装此插件
2. 选择要转换的文件
3. 启用"AI 智能参数预测"
4. 选择优化模式（balanced/quality/size）
5. 点击"转换"，AI 将自动推荐最佳参数

### 开发者设置

```bash
# 1. 设置开发环境
npm run setup

# 2. 安装依赖
npm install

# 3. 启动开发服务器
npm run dev

# 4. 打包分发版本
npm run release
```

详细开发文档请查看：[../QUICKSTART.md](../QUICKSTART.md)

## 🔧 技术架构

- **前端**: Vue 3 + Vite
- **后端**: Rust (`pixly-eagle-core` 共享二进制)
- **AI 引擎**: Python ML Bridge (LightGBM/PPO/Ensemble)
- **环境**: 仅支持 Eagle，开发模式支持浏览器调试

## 📖 相关链接

- [项目主页](../../README.md)
- [开发者指南](../QUICKSTART.md)
- [实施计划](/.gemini/antigravity/brain/xxx/implementation_plan.md)

## 📄 许可证

MIT License - Copyright (c) 2025 Pixly Team
