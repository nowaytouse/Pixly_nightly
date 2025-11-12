# 🎨 PIXLY - Next-Gen Image & Video Converter

> **双内核架构**: Python AI决策 + Rust执行 + JavaScript UI  
> **真实调用**: 禁止fallback、禁止模拟数据、禁止硬编码  
> **质量优先**: 永远不牺牲质量换速度

## 🚀 快速开始

```bash
# 1. (可选) 启动 Python AI 服务
python3 tools/pixly_http_server.py --port 50052

# 2. 编译 Rust 核心
cd core/rust
cargo build --release

# 3. 运行 CLI 或通过 Eagle 插件启动
./target/release/pixly-rust --help
```

## 📖 用户手册

### 1. 环境准备

1. **安装语言工具链**
   - Python 3.10+ (`python3 --version`)
   - Rust 1.70+ (`rustc --version`)
   - Node.js 18+（仅用于调试 Eagle 插件 `node -v`）
2. **安装系统依赖**（macOS 示例）

    ```bash
    brew install ffmpeg libjxl libavif gifsicle webp
    ```

3. **安装Python环境（可选 AI 功能）**

    ```bash
    python3 -m venv .venv
    source .venv/bin/activate
    pip install -r tools/requirements_server.txt
    ```

### 2. 双内核健康检查

> 所有命令均可直接在终端执行，确保 Python / Rust 两个内核随时可用。

#### Python AI 决策服务

```bash
# 启动服务（默认50052端口）
python3 tools/pixly_http_server.py --port 50052

# 新终端窗口执行健康检查
curl http://localhost:50052/api/v1/health
# 预期输出: {"status":"ok","service":"Pixly Python Service"}
```

#### Rust 执行内核

```bash
cd core/rust
cargo build --release

# 核心自检
./target/release/pixly-rust --check-deps
./target/release/pixly-rust info ./sample.jpg
```

#### Rust CLI + Python AI 联动验证

```bash
# Python 服务需保持运行
./target/release/pixly-rust analyze ./sample.jpg --ai
# 日志中应包含 "Using AI service" 提示
```

### 3. 常见工作流

#### 单文件转换（图像）

```bash
pixly-rust convert input.png output.avif --quality 90 --metadata
```

#### 批量目录转换（图像）

```bash
pixly-rust batch ./images ./output avif --threads 0
```

#### GIF 多阶段优化

```bash
pixly-rust gif-optimize input.gif output.gif --lossy 20 --colors 128
```

#### 音频统一转换

```bash
# 直接调用Python音频预测器（推荐）
python3 tools/audio_predict.py --input input.wav --mode balanced

# 通过HTTP API获取音频编码建议
curl -X POST http://localhost:50052/api/v1/predict/audio \
  -H "Content-Type: application/json" \
  -d '{
        "audio_path": "/path/to/input.mp3",
        "mode": "balanced"
      }'
```

> 音频AI提供编码器推荐（AAC/Opus/FLAC）、比特率建议、Magika安全检测，并自动跳过极小/极短音频以避免误压缩。Rust CLI 接入正在进行中，可先通过 Python 工具链或 HTTP API 调用。

#### Eagle 插件操作

1. 在 Eagle 中选择素材（图像/视频/音频）
2. 选择模式（智能/手动/视频/音频）
3. 若启用手动模式，点击“核心健康检查”按钮 → 显示 Python / Rust 状态
4. 点击“开始转换”并在日志面板观察 `PIXLY_MSG` 统一日志

### 4. 故障排查

| 场景 | 诊断命令 | 解决建议 |
|------|---------|----------|
| Python 服务无响应 | `curl http://localhost:50052/api/v1/health` | 检查端口占用，确认日志无 `PIXLY-PYTHON-*` 错误 |
| Rust CLI 报缺依赖 | `pixly-rust --check-deps` | 安装缺失工具（ffmpeg/libjxl/libavif/gifsicle） |
| 手动模式控件灰显 | 查看 Eagle 插件日志 (`Cmd+Option+I`) | 运行 `checkAndEnableManualMode()`，确认 Rust CLI 或 HTTP 服务可用 |
| AI 参数预测失败 | `tail -f logs/ai-service.log` | 确认 Python 服务依赖已安装并正在运行 |

更多使用案例见 `docs/ARCHITECTURE.md` 与 `docs/todolist/MASTER_TODO_LIST.md` 里的任务记录。

## 📁 项目结构

```text
Pixly_Nightly/
├── core/                    # 核心代码
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
├── tools/                  # Python AI 服务与工具脚本
├── scripts/                # 自动化脚本
├── @deprecated/            # 废弃代码（不提交到git）
└── @reference/             # 参考资料（不提交到git）
```

## 🎯 核心原则

1. **质量 > 速度**: 永远不牺牲质量换速度
2. **正面解决 > 绕过**: 禁止fallback、禁止模拟数据、禁止硬编码
3. **响亮报错 > 静默降级**: AI不可用应立即报错，不要fallback
4. **双内核架构**: Python AI决策 + Rust执行 + JS仅UI
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
✅ **现代格式**: JXL/AVIF/WebP/HEVC/AV1/Opus全面支持（含统一音频管线）

### Python AI服务
- **用途**: AI参数预测、格式决策、视频策略推荐
- **端口**: 50052 (Flask HTTP API)
- **主要库**: Flask, LightGBM, Torch (PPO), Magika, librosa/ffmpeg, FastAPI 风格路由
- **音频能力**:
  - Magika 音频安全检测
  - 编码器升级：MP3→Opus、AAC→Opus、WAV→FLAC
  - 自动跳过极小（<10KB）或极短（<1s）音频
  - 返回比特率/声道/采样率建议及转换原因

### Rust执行内核
- **用途**: 图像/视频转换、批量处理
- **接口**: CLI + HTTP API
- **主要库**: image-rs, tokio, actix-web, ffmpeg-cli-wrappers（跨音频/视频）

### JavaScript UI
- **用途**: Eagle插件界面
- **框架**: 原生JS（无框架）
- **功能**: 参数配置、进度显示、文件选择

## 🔧 开发

### 环境要求
- Python 3.10+
- Rust 1.70+
- Node.js 18+ (仅用于测试)

### 编译命令
```bash
# Rust 核心
cd core/rust
cargo build --release

# 运行测试
cargo test
```

## 📡 API 文档

**基础URL**: `http://localhost:50052`

### 图像AI预测

```http
POST /api/v1/predict
```

### 视频AI预测 (F-002)

```http
POST /api/v1/predict/video
```

### 音频AI预测 (F-003)

```http
POST /api/v1/predict/audio
```

### 健康检查

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
- Rust/Python/JS统一JSON格式（音频流程同样输出 PIXLY_MSG）
- 23个标准错误码

## 📄 许可证

见 [LICENSE](LICENSE) 文件。

---

**注意**: 本项目遵循严格的质量标准，禁止任何形式的fallback代码或模拟数据。
