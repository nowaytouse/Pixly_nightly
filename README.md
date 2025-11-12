# 🎨 PIXLY - Next-Gen Image & Video Converter

> **双内核架构**: Go AI决策 + Rust执行 + JavaScript UI  
> **真实调用**: 禁止fallback、禁止模拟数据、禁止硬编码  
> **质量优先**: 永远不牺牲质量换速度

## 🚀 快速开始

```bash
# 1. 启动 Go AI 服务
cd core/go
go run cmd/ai-service/main.go --port 50052

# 2. 编译 Rust 核心
cd core/rust
cargo build --release

# 3. 启动应用
# (从 Eagle 插件启动)
```

## 📁 项目结构

```
Pixly_Nightly/
├── core/                    # 核心代码
│   ├── go/                  # Go AI 决策服务
│   │   ├── cmd/            # 命令行工具
│   │   ├── pkg/            # 核心包
│   │   └── ...
│   └── rust/               # Rust 执行内核
│       ├── src/
│       │   ├── cli/        # CLI 命令
│       │   ├── converter/  # 转换核心
│       │   └── server/     # HTTP 服务
│       └── ...
├── plugin/                 # Eagle 插件
│   ├── manifest.json
│   ├── main.js
│   └── ...
├── docs/                   # 项目文档
│   ├── architecture/       # 架构设计
│   ├── guides/             # 使用指南
│   ├── phases/             # 开发阶段
│   ├── reports/            # 报告和分析
│   └── sessions/           # 会话记录
├── tools/                  # 开发工具
├── scripts/                # 自动化脚本
├── @deprecated/            # 废弃代码（不提交到git）
└── @reference/             # 参考资料（不提交到git）
```

## 🎯 核心原则

1. **质量 > 速度**: 永远不牺牲质量换速度
2. **正面解决 > 绕过**: 禁止fallback、禁止模拟数据、禁止硬编码
3. **响亮报错 > 静默降级**: AI不可用应立即报错，不要fallback
4. **双内核架构**: Go AI决策 + Rust执行 + JS仅UI
5. **真实调用 > 演示代码**: 所有功能必须真实工作

## 📚 核心文档

- **[项目质量宣言](docs/architecture/PROJECT_QUALITY_MANIFESTO.md)** - 核心原则和架构规范
- **[任务清单](docs/todolist/MASTER_TODO_LIST.md)** - 开发任务和进度
- **[变更日志](CHANGELOG.md)** - 功能更新和修复记录
- **[README](README.md)** - 本文档
- **[架构文档](docs/ARCHITECTURE.md)** - (待创建)

## 🛠️ 技术栈

### Pixly - AI智能媒体优化器

**Pixly** 是一个AI驱动的现代媒体格式转换与优化工具，支持 JXL, AVIF, WebP, HEVC, AV1, Opus 等最新格式。

## 🎯 核心特性

✅ **质量优先**: 维持质量前提下必然减小空间占用  
✅ **AI智能**: PPO强化学习模型智能优化参数  
✅ **双端架构**: Rust高性能内核 + Python AI决策  
✅ **Eagle集成**: 专业版与AI版双插件支持  
✅ **现代格式**: JXL/AVIF/WebP/HEVC/AV1/Opus全面支持

### Go AI服务
- **用途**: AI参数预测、格式决策、视频策略推荐
- **端口**: 50052 (HTTP API)
- **主要库**: net/http, LightGBM (Python桥接)

### Rust执行内核
- **用途**: 图像/视频转换、批量处理
- **接口**: CLI + HTTP API
- **主要库**: image-rs, tokio, actix-web

### JavaScript UI
- **用途**: Eagle插件界面
- **框架**: 原生JS（无框架）
- **功能**: 参数配置、进度显示、文件选择

## 🔧 开发

### 环境要求
- Go 1.21+
- Rust 1.70+
- Node.js 18+ (仅用于测试)

### 编译命令
```bash
# Go AI服务
cd core/go && go build ./cmd/ai-service

# Rust核心
cd core/rust && cargo build --release

# 运行测试
cargo test
go test ./...
```

## 📡 API 文档

### Go AI服务 HTTP API

**基础URL**: `http://localhost:50052`

#### 1. 图像AI预测
```http
POST /api/v1/predict
```

#### 2. 视频AI预测 (F-002)
```http
POST /api/v1/predict/video
```

#### 3. 健康检查
```http
GET /api/v1/health
```

### Rust核心 CLI

```bash
# 图像转换（AI自动优化）
pixly convert input.png --format jxl --ai-optimize

# GIF多阶段优化 (F-001)
pixly optimize-gif input.gif --output output.gif --lossy

# 批量转换
pixly batch-convert ./images/*.png --format jxl --output ./output/
```

## 🎯 新功能 (Phase 46.14+)

### ✅ F-001: GIF多阶段优化
- 多轮gifsicle迭代（最多5轮，<1%停止）
- 预期减少15-30%额外体积

### ✅ F-002: 视频AI策略完善
- 智能编码器选择（h264/h265/av1/vp9）
- 视频类型识别（动画/真人/游戏/电影）
- 预期减少20-40%体积

### ✅ F-004: 批量处理队列
- 任务持久化（断点续传）
- 4级优先级支持
- 并发任务管理

### ✅ 三端统一日志系统
- Go/Rust/Python统一JSON格式
- 23个标准错误码

## 📄 许可证

见 [LICENSE](LICENSE) 文件。

---

**注意**: 本项目遵循严格的质量标准，禁止任何形式的fallback代码或模拟数据。
