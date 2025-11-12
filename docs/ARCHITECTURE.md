# PIXLY 架构文档

> **最后更新**: 2025-11-11  
> **架构版本**: Phase 46.14+

## 📋 目录

1. [系统概览](#系统概览)
2. [三端架构](#三端架构)
3. [核心模块](#核心模块)
4. [通信协议](#通信协议)
5. [数据流](#数据流)
6. [技术栈](#技术栈)

---

## 系统概览

PIXLY是一个**AI驱动的图像/视频转换工具**，采用**三端分离架构**：

```
┌─────────────────────────────────────────────────────┐
│                    Eagle插件 (UI)                    │
│                   JavaScript/HTML                     │
└─────────────────────┬───────────────────────────────┘
                      │ HTTP API
┌─────────────────────▼───────────────────────────────┐
│                Go AI决策服务                         │
│           参数预测 + 策略推荐 + 错误处理             │
└─────────────────────┬───────────────────────────────┘
                      │ HTTP/CLI调用
┌─────────────────────▼───────────────────────────────┐
│                 Rust执行内核                         │
│        图像/视频转换 + 批量处理 + 质量验证           │
└─────────────────────────────────────────────────────┘
```

### 核心原则

1. **质量 > 速度** - 永远不牺牲质量换速度
2. **真实调用 > 演示代码** - 禁止fallback、模拟数据
3. **响亮报错 > 静默降级** - AI不可用应立即报错
4. **三端分离** - Go AI决策 + Rust执行 + JS仅UI
5. **YAGNI** - 不需要就不要（避免过度设计）

---

## 三端架构

### 1. Go AI决策服务 (`core/go/ai/`)

**职责**: AI参数预测、格式决策、视频策略推荐

**端口**: `50052` (HTTP API)

**核心模块**:
- `http_gateway.go` - HTTP API网关
- `messaging.go` - 统一消息传递
- `python_bridge.go` - Python ML桥接
- `error.go` - 错误码系统（23个标准错误码）
- `logging.go` - 统一日志（JSON格式）

**API端点**:
```
POST /api/v1/predict        - 图像AI预测
POST /api/v1/predict/video  - 视频AI预测 (F-002)
GET  /api/v1/health         - 健康检查
```

### 2. Rust执行内核 (`core/rust/`)

**职责**: 图像/视频转换、批量处理、质量验证

**核心模块**:

#### 转换器 (`src/converter/`)
- `image_converter.rs` - 图像转换核心
- `video_processor.rs` - 视频处理
- `gif_optimizer.rs` - GIF多阶段优化 (F-001)
- `task_queue.rs` - 批量处理队列 (F-004)
- `batch_processor.rs` - 高性能批量处理

#### 编码器 (`src/converter/`)
- `native_avif.rs` - AVIF编码（rav1e）
- `native_webp.rs` - WebP编码
- `native_png.rs` - PNG编码
- `native_jpeg.rs` - JPEG编码

#### 基础设施
- `messaging.rs` - 统一消息传递
- `logging.rs` - 统一日志（tracing）
- `error.rs` - 错误码系统

### 3. Python ML服务 (`tools/`)

**职责**: 机器学习预测、视频分析

**核心脚本**:
- `predict_params.py` (v4.4.0) - AI参数预测
  - `predict_image()` - 图像参数预测
  - `predict_video()` - 视频参数预测 (F-002)
- `pixly_messaging.py` - 统一消息传递

**ML模型**:
- LightGBM - 参数预测模型
- 视频类型识别（动画/真人/游戏/电影）

---

## 核心模块

### 统一日志系统 (A-002)

**三端统一JSON格式**:

```json
{
  "timestamp": "2025-11-11T12:00:00Z",
  "level": "INFO",
  "component": "Converter",
  "message": "Image converted successfully",
  "context": {
    "format": "jxl",
    "size_mb": 2.5
  }
}
```

**实现**:
- Go: `logging.go` - 结构化日志
- Rust: `logging.rs` - tracing框架
- Python: `predict_params.py` - JSON日志

### 统一错误码系统 (G-004, Q-002)

**23个标准错误码**:

| 类别 | 前缀 | 示例 |
|------|------|------|
| 系统错误 | SYS | PIXLY-GO-SYS-001 |
| 文件错误 | FILE | PIXLY-RUST-FILE-002 |
| 验证错误 | VAL | PIXLY-GO-VAL-003 |
| 业务错误 | BIZ | PIXLY-PYTHON-BIZ-001 |

**跨端一致**:
- Go: `error.go` - PixlyError结构
- Rust: `error.rs` - PixlyError枚举
- Python: 错误码字符串

### 统一消息传递系统

**协议**: `PIXLY_MSG:{json}` 输出到stdout

**消息格式**:
```json
{
  "id": "msg_abc123",
  "type": "progress",
  "level": 1,
  "source": "rust-core",
  "component": "Converter",
  "message": "Converting image...",
  "progress": 75,
  "timestamp": 1699699200
}
```

**6种消息类型**:
- Info, Warning, Error, Success, Progress, Status

**5级消息级别**:
- Debug(0), Info(1), Warning(2), Error(3), Critical(4)

### GIF多阶段优化 (F-001)

**算法**: 多轮gifsicle迭代优化

```
Pass 1: 无损优化 (O3)
Pass 2: 色彩优化 (动态色彩数)
Pass 3: 有损压缩 (可配置质量)
Pass 4-N: 迭代优化直到收益 < 1%
```

**预期效果**: 减少15-30%额外体积

### 视频AI策略 (F-002)

**智能决策**:
- 编码器选择: h264/h265/av1/vp9
- 视频类型识别: 动画/真人/游戏/电影
- CRF推荐: 基于分辨率和模式
- 两遍编码: 自动决策

**预期效果**: 减少20-40%视频体积

### 批量处理队列 (F-004)

**核心特性**:
- 任务持久化（JSON格式，断点续传）
- 4级优先级: Low/Normal/High/Urgent
- 6种状态: Pending/Running/Completed/Failed/Cancelled/Paused
- 并发管理: 自动调度

---

## 通信协议

### Go ↔ Python

**方式**: 子进程调用

```go
// Go调用Python
cmd := exec.Command("python3", "predict_params.py", imagePath, mode)
output, err := cmd.Output()
result := parseJSON(output)
```

### Go ↔ Rust

**方式**: HTTP API调用（gRPC已废弃）

```
Go调用Rust: 通过CLI或HTTP
Rust返回: JSON格式结果
```

### 三端消息同步

**方式**: stdout标准输出

```
任意端 → PIXLY_MSG:{json} → stdout → 其他端解析
```

---

## 数据流

### 图像转换流程

```
1. [Eagle插件] 用户选择图像
2. [Go AI] 调用Python预测参数
3. [Python] 返回推荐参数
4. [Go AI] 返回给插件展示
5. [Eagle插件] 用户确认参数
6. [Rust] 执行实际转换
7. [Rust] 返回转换结果
```

### 视频转换流程

```
1. [Go AI] 接收视频预测请求
2. [Python] ffprobe提取元数据
3. [Python] AI分析视频类型
4. [Python] 推荐编码参数
5. [Go AI] 返回推荐结果
6. [Rust] 执行视频转换
```

### 批量处理流程

```
1. [Rust] 创建TaskQueue
2. [Rust] 添加任务到队列
3. [Rust] 按优先级调度
4. [Rust] 并发执行转换
5. [Rust] 保存队列状态（持久化）
6. [Rust] 返回批量结果
```

---

## 技术栈

### Go AI服务
- **语言**: Go 1.21+
- **框架**: net/http (标准库)
- **日志**: 自定义JSON日志
- **桥接**: Python子进程调用

### Rust执行内核
- **语言**: Rust 1.70+
- **框架**: 
  - image-rs (图像处理)
  - rayon (并发)
  - serde (序列化)
  - tracing (日志)
- **编码器**:
  - rav1e (AVIF)
  - webp (WebP)
  - oxipng (PNG)

### Python ML服务
- **语言**: Python 3.8+
- **ML**: LightGBM
- **视频**: ffprobe/ffmpeg
- **格式**: JSON输入输出

### JavaScript UI (Eagle插件)
- **语言**: 原生JavaScript (无框架)
- **目标**: 最小化复杂度

---

## 已废弃技术

### ❌ gRPC通信
- **原因**: 过度复杂，HTTP足够
- **替代**: HTTP API + JSON
- **清理**: Phase 46 (A-003)

### ❌ Fallback代码
- **原因**: 违反"响亮报错"原则
- **替代**: 直接返回错误
- **清理**: Phase 46 (Q-003)

### ❌ MessageChannel/MessageReader
- **原因**: 过度设计，未被使用
- **替代**: 直接使用快捷函数
- **清理**: Phase 46.14+ (代码简化)

---

## 文件结构

```
Pixly_Nightly/
├── core/
│   ├── go/ai/              # Go AI决策服务
│   │   ├── http_gateway.go
│   │   ├── messaging.go
│   │   ├── python_bridge.go
│   │   ├── error.go
│   │   └── logging.go
│   └── rust/               # Rust执行内核
│       └── src/
│           ├── converter/  # 转换核心
│           ├── messaging.rs
│           ├── logging.rs
│           └── error.rs
├── tools/                  # Python ML服务
│   ├── predict_params.py   # v4.4.0
│   └── pixly_messaging.py
├── plugin/                 # Eagle插件
├── docs/                   # 核心文档
│   ├── architecture/
│   │   └── PROJECT_QUALITY_MANIFESTO.md
│   ├── todolist/
│   │   └── MASTER_TODO_LIST.md
│   └── ARCHITECTURE.md     # 本文档
├── CHANGELOG.md
└── README.md
```

---

## 性能指标

### 转换性能
- **单图转换**: < 5s (4K图像)
- **批量处理**: 并发度 = CPU核心数
- **内存占用**: < 500MB (单转换)

### AI预测性能
- **参数预测**: < 200ms
- **视频分析**: < 2s (10分钟视频)

### 质量指标
- **压缩率**: GIF多阶段优化 -15~30%
- **视频压缩**: AI策略优化 -20~40%
- **质量保持**: SSIM > 0.95

---

## 开发规范

### 代码质量
1. **禁止fallback代码** - AI不可用应报错
2. **禁止模拟数据** - 所有功能真实工作
3. **禁止硬编码** - 使用配置或AI决策
4. **禁止过度抽象** - YAGNI原则

### 日志规范
- 三端统一JSON格式
- 必须包含: timestamp, level, component, message
- 可选包含: context (key-value对)

### 错误处理
- 使用标准错误码（23个）
- 三端错误码格式: `PIXLY-{LANG}-{TYPE}-{NUM}`
- 错误必须包含: code, message, severity

### 测试规范
- 单元测试: 核心逻辑必须测试
- 集成测试: 三端通信必须验证
- 性能测试: 关键路径必须基准测试

---

## 部署架构

### 开发环境
```
Go AI服务: localhost:50052
Rust CLI: 本地命令行
Eagle插件: Eagle应用内
```

### 生产环境
```
Go AI服务: 独立进程（HTTP服务）
Rust CLI: 打包为可执行文件
Eagle插件: 通过插件市场分发
```

---

## 更新历史

| 日期 | 版本 | 主要变更 |
|------|------|----------|
| 2025-11-11 | Phase 47 | **架构简化：移除Go，双端架构（-11151行）** |
| 2025-11-11 | Phase 46.14+ | 统一消息系统、代码简化、架构文档 |
| 2025-11-11 | Phase 46.8-14 | GIF优化、视频AI、任务队列、API文档 |
| 2025-11-06 | Phase 46 | 三端统一日志、错误码系统、废弃gRPC |

---

## 架构简化方案（Phase 47 ✅ 已完成 - 2025-11-11）

### 当前架构分析

**三端架构现状**:
```
Go AI服务（11151行，33文件）
 ├─ HTTP API网关（端口50052）
 ├─ Python桥接层
 ├─ 错误处理和日志
 └─ 消息传递系统

Rust执行内核（主要）
 └─ 图像/视频转换

Python ML服务（辅助）
 └─ AI参数预测
```

**问题识别**:
1. **架构过度复杂**: Go仅作为HTTP网关和Python桥接，功能单一
2. **维护成本高**: 需要维护3种语言的代码库
3. **部署复杂**: 需要同时部署Go服务和Python环境
4. **重复功能**: 消息传递、错误处理在三端都有实现

### 简化方案：双端架构

**新架构**:
```
Python AI服务（可选）
 ├─ HTTP API网关（Flask/FastAPI）
 ├─ AI参数预测（LightGBM）
 ├─ 视频分析
 └─ 直接调用Rust CLI

Rust执行内核（核心）
 ├─ 图像/视频转换
 ├─ 批量处理
 └─ CLI接口
```

### 迁移计划（已完成）

#### Phase 1: Python HTTP服务 ✅ (2025-11-11 13:10)
- ✅ 创建`tools/pixly_http_server.py`（300行）
- ✅ 实现`/api/v1/predict`端点（图像）
- ✅ 实现`/api/v1/predict/video`端点（视频）
- ✅ 实现`/api/v1/health`端点
- ✅ 使用Flask框架

#### Phase 2: Rust适配 ✅ (2025-11-11 13:15)
- ✅ 更新`ai_client.rs`配置和注释
- ✅ 适配Python HTTP服务端点
- ✅ 保持API完全兼容

#### Phase 3: 移除Go ✅ (2025-11-11 13:20)
- ✅ 迁移`core/go/ai`到`@deprecated/go_ai_service_2025_11_11`
- ✅ 更新所有相关文档
- ✅ **-11151行代码，-66文件**

### 收益分析

**简化效果**:
- ✅ 代码量: -11151行Go代码
- ✅ 文件数: -33个Go文件
- ✅ 语言数: 3种 → 2种（Rust + Python）
- ✅ 部署: Go服务 + Python → 仅Python（可选）
- ✅ 维护: 减少33%语言相关工作

**保留功能**:
- ✅ HTTP API完全兼容
- ✅ AI预测功能不变
- ✅ Rust核心功能不变
- ✅ CLI独立使用不受影响

### 实施原则

1. **渐进式迁移**: 先建Python服务，再废弃Go
2. **向后兼容**: 保持API接口不变
3. **测试驱动**: 每步迁移都需要测试
4. **文档同步**: 实时更新架构文档

### 风险控制

**低风险**:
- Go代码功能单一，主要是HTTP网关
- Python已有AI预测能力
- Rust CLI独立，不受影响

**备份计划**:
- Go代码移到@deprecated，而非删除
- 保留1-2周观察期
- 发现问题可快速回滚

---

**文档维护**: 遵循"只更新不新建"原则，本文档为核心5文档之一
