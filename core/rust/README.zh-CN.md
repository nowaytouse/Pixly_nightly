# 🦀 PIXLY Rust 转换服务

[![版本](https://img.shields.io/badge/%E7%89%88%E6%9C%AC-1.2.0-orange.svg)](https://www.rust-lang.org/)
[![许可证](https://img.shields.io/badge/%E8%AE%B8%E5%8F%AF%E8%AF%81-MIT-green.svg)](../LICENSE)

**100% Rust实现** - 高性能的图像/视频转换HTTP服务。

---

## 📋 概述

Rust服务是PIXLY的**转换核心**。它负责：
- 纯Rust HTTP API服务器（actix-web）
- 原生Rust编码器：AVIF/WebP/PNG/JPEG
- CLI工具集成：JXL/HEIC（通过cjxl、avifenc等）
- 完整元数据保留（EXIF、ICC、XMP）
- 动画图像支持（GIF、AVIF、JXL）
- 健康监控和缓存系统

**架构**: 100% Rust实现，特殊格式可选CLI工具fallback。

---

## ✨ 功能特性

### 支持的格式
- ✅ **JXL (JPEG XL)** - 通过 `cjxl`
- ✅ **AVIF** - 通过 `avifenc`
- ✅ **WebP** - 通过 `cwebp` 和 `gif2webp`
- ✅ **PNG** - 通过 CLI 工具
- ✅ **JPEG** - 通过 CLI 工具
- ✅ **GIF** - 通过 ImageMagick + ffmpeg

### 功能
- 🔄 **动画图像转换**
- 📦 **元数据保留**
- ⚡ **零拷贝管道**
- 🌐 **RESTful HTTP API**
- 💪 **强大的错误处理**

---

## 🚀 快速开始

### 前置要求
```bash
# 安装 CLI 工具 (macOS)
brew install jpeg-xl libavif webp imagemagick ffmpeg

# Ubuntu/Debian
sudo apt install libjxl-tools libavif-bin webp imagemagick ffmpeg
```

### 运行服务
```bash
# 开发模式
go run main.go

# 生产构建
go build -o pixly-rust-service
./pixly-rust-service
```

### 默认配置
- **端口**: `3000`
- **API地址**: `http://localhost:3000`
- **健康检查**: `GET /health`

---

## 📡 API 端点

### 图像转换
```bash
POST /convert
Content-Type: application/json

{
  "inputPath": "/path/to/input.jpg",
  "outputPath": "/path/to/output.jxl",
  "format": "jxl",
  "quality": 85,
  "effort": 7
}
```

### 健康检查
```bash
GET /health
```

**响应**:
```json
{
  "status": "healthy",
  "tools": {
    "cjxl": "available",
    "avifenc": "available",
    "cwebp": "available"
  }
}
```

---

## 🏗️ 架构设计

```
┌─────────────────────────────────────────┐
│     Rust 服务 (端口 3000)                │
├─────────────────────────────────────────┤
│  ┌──────────────────────────────────┐   │
│  │   格式转换器                     │   │
│  │   - JXL (cjxl)                   │   │
│  │   - AVIF (avifenc)               │   │
│  │   - WebP (cwebp)                 │   │
│  └──────────────────────────────────┘   │
│  ┌──────────────────────────────────┐   │
│  │   元数据处理                     │   │
│  │   - EXIF 保留                    │   │
│  │   - ICC 配置文件                 │   │
│  └──────────────────────────────────┘   │
│  ┌──────────────────────────────────┐   │
│  │   动画支持                       │   │
│  │   - GIF → AVIF/JXL               │   │
│  │   - 帧提取与重组                 │   │
│  └──────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

---

## 📁 项目结构

```
core/rust/
├── main.go                 # 服务入口 (Go)
├── handlers/               # HTTP 处理器
│   ├── convert.go         # 转换端点
│   └── health.go          # 健康检查
├── converters/             # 格式转换器
│   ├── jxl.go             # JXL 转换
│   ├── avif.go            # AVIF 转换
│   ├── webp.go            # WebP 转换
│   └── animated.go        # 动画处理
├── metadata/               # 元数据处理
│   ├── exif.go            # EXIF 提取/注入
│   └── icc.go             # ICC 配置文件
└── pixly-rust/             # Rust 核心库
    └── src/
        └── lib.rs         # 转换逻辑
```

---

## 🔧 配置

### 环境变量
```bash
# 服务配置
RUST_SERVICE_PORT=3000
RUST_LOG_LEVEL=info

# CLI 工具路径
CJXL_PATH=/usr/local/bin/cjxl
AVIFENC_PATH=/usr/local/bin/avifenc
CWEBP_PATH=/usr/local/bin/cwebp
```

### 配置文件 (`config.yaml`)
```yaml
service:
  port: 3000
  timeout: 60s
  max_file_size: 104857600  # 100MB

tools:
  cjxl: "/usr/local/bin/cjxl"
  avifenc: "/usr/local/bin/avifenc"
  cwebp: "/usr/local/bin/cwebp"

quality:
  jxl_default: 85
  avif_default: 80
  webp_default: 85
```

---

## 🧪 测试

### 运行测试
```bash
# 单元测试
go test ./...

# 集成测试
go test -tags=integration ./tests/

# 测试特定格式
go test ./converters/jxl_test.go
```

### 手动测试
```bash
# 启动服务
go run main.go &

# 测试转换
curl -X POST http://localhost:3000/convert \
  -H "Content-Type: application/json" \
  -d '{
    "inputPath": "test.jpg",
    "outputPath": "test.jxl",
    "format": "jxl",
    "quality": 85
  }'
```

---

## 📊 支持的转换

### 图像格式
| 输入 | 输出 | 工具 | 状态 |
|------|------|------|------|
| JPEG/PNG | JXL | cjxl | ✅ |
| JPEG/PNG | AVIF | avifenc | ✅ |
| JPEG/PNG | WebP | cwebp | ✅ |
| GIF | AVIF | avifenc | ✅ |
| GIF | JXL | cjxl | ✅ |

### 元数据保留
- ✅ EXIF (相机信息、GPS等)
- ✅ ICC 配置文件 (色彩管理)
- ✅ XMP (Adobe元数据)
- ⚠️ IPTC (部分支持)

---

## 🐛 调试

### 启用调试日志
```bash
export RUST_LOG_LEVEL=debug
go run main.go
```

### 常见问题

**Q: cjxl未找到**
```bash
# macOS
brew install jpeg-xl

# Linux
sudo apt install libjxl-tools
```

**Q: 转换失败**
```bash
# 检查工具状态
curl http://localhost:3000/health

# 查看日志
tail -f logs/rust-service.log
```

---

## 🤝 集成

### 与Go AI服务
```go
// AI服务调用Rust服务
resp, err := http.Post("http://localhost:3000/convert", ...)
```

### 与插件(JS)
```javascript
// 插件通过Go服务间接调用
// Go服务 → Rust服务 → CLI工具
```

---

## 📚 资源

- [JPEG XL 参考编码器](https://github.com/libjxl/libjxl)
- [libavif](https://github.com/AOMediaCodec/libavif)
- [libwebp](https://developers.google.com/speed/webp)
- [ImageMagick](https://imagemagick.org/)

---

## 📄 许可证

MIT 许可证 - 详见 [LICENSE](../../LICENSE)

---

## 🔗 相关服务

- **[Go AI 服务](../go/)** - AI决策和质量预测
- **[插件 (JS)](../plugin/)** - Eagle中的用户界面

---

**版本**: 1.2.0  
**最后更新**: 2025-11-09  
**维护者**: PIXLY 团队
