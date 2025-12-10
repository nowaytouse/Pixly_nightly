# Pixly Format-Vue Plugin

**专业图像/视频格式转换工具 - Eagle 插件**

> ⚠️ **重要**: 此插件仅能在 [Eagle](https://eagle.cool) 环境中运行

## ✨ 特性

- 🖼️ **图像转换**: 支持 JXL、AVIF、WebP、HEIC、PNG、JPEG 等格式
- 🎬 **视频转换**: 支持 H.264、H.265、AV1 等编码器
- 🎨 **动图处理**: GIF、APNG、WebP 动画转换
- 📎 **XMP 合并**: 自动合并 XMP 元数据文件
- 🔒 **文件验证**: Magika AI 文件类型验证
- 📊 **质量检测**: SSIM 质量评估
- 📝 **文件名规范化**: 处理特殊字符

## 🚀 快速开始

### 用户使用

1. 在 Eagle 中安装此插件
2. 选择要转换的图像/视频文件
3. 打开插件，选择目标格式
4. 点击"转换"

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
- **环境**: 仅支持 Eagle，开发模式支持浏览器调试

## 📖 相关链接

- [项目主页](../../README.md)
- [开发者指南](../QUICKSTART.md)
- [实施计划](/.gemini/antigravity/brain/xxx/implementation_plan.md)

## 📄 许可证

MIT License - Copyright (c) 2025 Pixly Team
