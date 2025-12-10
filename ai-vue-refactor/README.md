# Pixly AI - Eagle Plugin

**AI 智能媒体转换工具**

> ⚠️ **重要**: 此插件仅能在 [Eagle](https://eagle.cool) 环境中运行

## ✨ 特性

### AI 智能功能
- 🤖 **AI 参数预测**: 自动推荐最佳转换参数
- 🎯 **智能优化**: balanced/quality/size 三种优化模式
- 🔍 **场景检测**: 视频场景自动检测（优化 GOP）
- 📊 **VMAF 评估**: 视频质量自动验证
- ⚡ **GPU 加速**: 支持硬件加速

### 媒体转换
- 🖼️ **图像转换**: JXL、AVIF、WebP、HEIC、PNG、JPEG
- 🎬 **视频转换**: H.264、H.265、H.266、AV1、VP9
- 🎨 **动图处理**: GIF、APNG、WebP 动画
- 📎 **XMP 合并**: 自动合并元数据
- 🔒 **文件验证**: Magika AI 验证
- 📊 **质量检测**: SSIM 评估

## 🚀 快速开始

### 用户使用

1. 在 Eagle 中安装此插件
2. 选择要转换的文件
3. 启用"AI 智能参数预测"
4. 选择优化模式（balanced/quality/size）
5. 点击"转换"，AI 将自动推荐最佳参数

### 开发者设置

```bash
# 1. 安装依赖
npm install

# 2. 启动开发服务器
npm run dev

# 3. 打包分发版本
npm run release
```

## 🔧 技术架构

- **前端**: Vue 3 + Vite
- **后端**: Rust Core (`pixly-converter`)
- **AI 引擎**: Python ML Bridge (LightGBM/PPO/Ensemble)
- **环境**: 仅支持 Eagle

## 📄 许可证

MIT License - Copyright (c) 2025 Pixly Team
