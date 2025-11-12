# 🔧 PIXLY Core - 双内核架构

> **Go AI决策 + Rust执行内核**  
> **真实调用，拒绝Fallback**

## 📁 目录结构

```
core/
├── go/                 # Go AI 决策服务
│   ├── cmd/           # 命令行工具
│   │   └── ai-service/ # AI 参数预测服务 (gRPC)
│   ├── pkg/           # 核心包
│   │   ├── ai/        # AI 决策逻辑
│   │   ├── quality/   # 质量评估
│   │   ├── knowledge/ # 格式知识库
│   │   └── predictor/ # 参数预测器
│   ├── go.mod
│   └── go.sum
└── rust/              # Rust 执行内核
    ├── src/
    │   ├── cli/       # CLI 命令行接口
    │   │   └── commands/ # 模块化命令
    │   ├── converter/ # 转换核心（扁平化结构）
    │   │   ├── 🎯 核心引擎
    │   │   ├── 🦀 原生编码器
    │   │   ├── 📦 转换策略
    │   │   ├── 🎬 视频处理
    │   │   ├── 🤖 AI集成
    │   │   └── ... (详见 converter/mod.rs)
    │   ├── server/    # HTTP API 服务
    │   └── logging.rs # 统一日志
    ├── Cargo.toml
    └── Cargo.lock
```

## 🚀 Go AI 服务

### 用途
- **AI 参数预测**: 根据图像特征智能预测转换参数
- **格式决策**: 选择最优输出格式
- **质量评估**: 评估转换质量

### 启动服务
```bash
cd core/go
go run cmd/ai-service/main.go --port 50052
```

### 服务端口
- **HTTP API**: 8081 (Go AI Service)
- **协议**: JSON over HTTP

### 主要功能
1. **图像参数预测** (`PredictImageParams`)
   - 输入：图像路径、目标格式、偏好
   - 输出：quality、speed、lossless等参数

2. **视频参数预测** (`PredictVideoParams`)
   - 输入：视频路径、目标格式
   - 输出：codec、crf、preset等参数

3. **格式建议** (`SuggestFormat`)
   - 输入：图像特征
   - 输出：最优格式建议

## 🦀 Rust 执行内核

### 用途
- **图像转换**: 支持AVIF、JXL、WebP、PNG、JPEG等
- **视频转换**: H.264、H.265、AV1等编码
- **批量处理**: 并发批量转换
- **CLI工具**: 完整的命令行接口
- **HTTP API**: RESTful API接口

### 编译
```bash
cd core/rust
cargo build --release
```

### 运行
```bash
# CLI 模式
./target/release/pixly-rust convert input.jpg output.avif

# HTTP 服务模式
./target/release/pixly-rust serve --port 8080
```

### 核心模块

#### CLI 命令 (cli/commands/)
- **convert**: 图像转换
- **batch**: 批量处理
- **video**: 视频转换
- **gif**: GIF优化
- **detect**: 文件类型检测
- **info**: 媒体信息查询
- **analyze**: 参数分析与试运行
- **eagle**: Eagle图库集成

#### 转换器 (converter/)
**扁平化结构，所有模块在同一层级**

- **核心引擎**: `image_converter.rs`, `strategy.rs`
- **原生编码器**: `native_avif.rs`, `native_webp.rs`, 等
- **转换策略**: `*_strategy.rs` 文件
- **视频处理**: `video_processor.rs`, `video_strategy.rs`
- **AI集成**: `ai_client.rs`, `params.rs`, `param_optimizers.rs`
- **批量处理**: `batch.rs`, `batch_processor.rs`
- **元数据**: `metadata.rs`
- **验证**: `validator.rs`, `validation.rs`

## 🔗 双内核通信

### 通信方式
```
Rust Core ←→ HTTP API (8081) ←→ Go AI Service
```

### 工作流程
1. **Rust** 收到转换请求
2. **Rust** 分析图像特征
3. **Rust** → **Go**: 请求AI参数预测
4. **Go** 执行AI决策逻辑
5. **Go** → **Rust**: 返回优化参数
6. **Rust** 执行实际转换
7. **Rust** 返回转换结果

### 错误处理
- **响亮报错**: AI服务不可用时立即报错
- **禁止Fallback**: 不使用硬编码参数作为备选
- **真实调用**: 确保AI服务正常工作

## 🛠️ 开发

### 环境要求
- **Go**: 1.21+
- **Rust**: 1.70+
- **系统工具**: 
  - avifenc (libavif)
  - cjxl (libjxl)
  - cwebp (libwebp)
  - ffmpeg (视频处理)
  - gifsicle (GIF优化)

### 编译命令
```bash
# Go AI 服务
cd core/go
go build ./cmd/ai-service

# Rust 核心
cd core/rust
cargo build --release

# 运行测试
cd core/rust
cargo test

cd core/go
go test ./...
```

### 代码质量
- **Rust**: `cargo clippy` - 0 warnings
- **Go**: `go vet` - clean
- **测试覆盖**: 持续提升

## 📚 相关文档
- [项目架构](../docs/architecture/)
- [开发指南](../docs/guides/)
- [CLI命令文档](rust/src/cli/commands/)
- [转换器文档](rust/src/converter/)

---

**核心原则**: 质量优先、真实调用、响亮报错、禁止Fallback
