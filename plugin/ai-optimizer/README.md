# PIXLY AI 全媒体智能优化插件

## 项目状态

**当前阶段**: Phase 1 - 基础架构完成 ✅

### 已完成
- ✅ Vue3 + Element Plus + Vite 项目搭建
- ✅ Eagle API 集成 (useEagleAPI.js)
- ✅ Rust CLI 调用封装 (useRustCLI.js)
- ✅ 国际化支持 (en/zh_CN)
- ✅ AI 主题样式 (参考官方插件)
- ✅ 质量宣言合规性检查
  - 删除 fallback hell
  - 删除模拟数据
  - 响亮的错误处理

### 待完成 (见 docs/todolist/AI_OPTIMIZER_TODO.md)
- ⏳ AI-001: 集成 Python ML + Rust 推理系统
- ⏳ AI-002: 完善 analyze 命令的 CLI 集成
- ⏳ AI-003: UI 完善 (主题适配、Comet 动画)
- ⏳ AI-004: 媒体特征提取增强

## 真实架构

**Python ML + Rust 推理**:
```
Python 训练 (scripts/ml_bridge.py)
    ↓ LightGBM 模型
Rust 推理 (src/ml_bridge.rs)
    ↓ 128维特征提取
AI 预测结果
```

**不存在的架构** ❌:
- ❌ GO AI 服务 (过时架构)
- ❌ HTTP API (不需要)
- ❌ 网络调用 (本地推理)

## 快速开始

### 开发模式
```bash
cd plugin/ai-optimizer
npm run dev
```

### 构建生产版本
```bash
npm run build
# 输出: dist/
```

### 在 Eagle 中测试
1. 复制 `dist/` 内容到 Eagle 插件目录
2. 复制 `manifest.json` 和 `_locales/`
3. 在 Eagle 中重新加载插件

## 核心功能

### 1. 全媒体分析
- 图片: 静态/动态/透明
- 视频: MP4/MOV/WebM
- 音频: MP3/AAC/FLAC

### 2. AI 推荐
- 格式推荐 (AVIF/JXL/H.265)
- 参数推荐 (quality/effort/crf)
- 预估效果 (文件大小/质量评分)

### 3. 批量处理
- 多文件分析
- 分组展示
- 进度可见

## 质量标准

遵循 `PROJECT_QUALITY_MANIFESTO.md`:
- ✅ 真实的 AI 调用 (不模拟)
- ✅ 响亮的错误处理
- ✅ 完整的功能实现
- ✅ 明确标注临时实现
- ✅ 记录 TODO 任务

## 参考文档

- [架构设计](./ARCHITECTURE.md)
- [TODO 清单](../../docs/todolist/AI_OPTIMIZER_TODO.md)
- [质量宣言](../../PROJECT_QUALITY_MANIFESTO.md)
- [官方 AI 插件参考](../reference-ai-enlarger/)

---

**最后更新**: 2025-11-18  
**版本**: 1.0.0-alpha
