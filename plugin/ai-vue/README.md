# PIXLY AI Vue Plugin

**版本**: 3.0.0  
**状态**: 🚧 开发中  
**技术栈**: Vue3 + Vite + Element Plus

## 架构

参考官方 `ai-image-enlarger` 插件：
- ✅ Vue3 + Vite构建
- ✅ Element Plus UI组件
- ✅ Eagle API集成
- ⏳ Rust CLI调用（待实现）

## 当前功能

✅ **已实现**:
- Vue3 + Element Plus基础架构
- Eagle文件加载
- AI选项UI（预设、功能开关）
- 文件列表显示
- 进度条

⏳ **待实现**:
- Rust CLI调用（参考format-vue的useRustCLI）
- AI智能转换逻辑
- 格式推荐
- 批量优化

## 开发

```bash
# 安装依赖
npm install

# 开发模式
npm run dev

# 构建
npm run build
```

## 部署

构建后的文件在 `dist/` 目录，Eagle会加载 `dist/index.html`

## 下一步

1. 复用format-vue的useRustCLI.js
2. 实现AI转换逻辑
3. 集成GO AI Service
4. 完善UI交互
