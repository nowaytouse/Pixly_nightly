# Static Converter - Eagle Plugin

🔒 JXL + 😂 AVIF 双模式静态图转换器

[English](#features) | [中文](#功能特性)

---

## 🎯 双模式设计

| 模式 | 图标 | 定位 | 策略 |
|------|------|------|------|
| **JXL** | 🔒 | 高质量图像优化 | 无损优先，质量第一 |
| **AVIF** | 😂 | 表情包/贴纸优化 | 有损压缩，体积优先 |

---

## Features

### ✅ Self-Contained (No System Dependencies)
Pre-built binaries for all platforms. No need to install libjxl, libavif, or any dependencies.

### Supported Platforms
| Platform | Architecture | Status |
|----------|-------------|--------|
| macOS | ARM64 (Apple Silicon) | ✅ |
| macOS | x64 (Intel) | ✅ |
| Windows | x64 | ✅ |
| Linux | x64 | ✅ |

### JXL Mode - High Quality
| Format | Mode | Description |
|--------|------|-------------|
| JPEG | `--lossless_jpeg=1` | **Reversible** - preserves DCT |
| PNG | `-d 0` | Mathematical lossless |
| BMP/TIFF/TGA | `-d 0` | Mathematical lossless |

### AVIF Mode - Sticker Optimization
| Format | Tool | Description |
|--------|------|-------------|
| JPEG/PNG/WebP/BMP | avifenc | Lossy compression |
| GIF (static) | avifenc | Lossy compression |
| GIF (animated) | FFmpeg | Animated AVIF |

### Smart Features
- **Auto rollback** - Skip if output is larger
- **Metadata preservation** - EXIF/IPTC/XMP/ICC
- **Timestamp preservation** - mtime/atime
- **Multi-language** - EN/简中/繁中/日本語

---

## 功能特性

### ✅ 自包含（无需系统依赖）
包含所有平台的预编译二进制文件，无需安装 libjxl、libavif 或任何依赖。

### JXL 模式 - 高质量
- **JPEG**: 可逆转码，保留 DCT 系数
- **PNG/BMP/TIFF**: 数学无损压缩
- **大小阈值**: 默认 ≥1.25MB 才转换（可调）

### AVIF 模式 - 表情包优化
- **所有格式**: 有损压缩，体积优先
- **质量可调**: 50-100（默认80）
- **动画支持**: GIF 动画用 FFmpeg 转换

### 智能特性
- **自动回退** - 输出更大时跳过
- **元数据保留** - EXIF/IPTC/XMP/ICC
- **时间戳保留** - mtime/atime

---

## Binary Structure

```
bin/
├── darwin/           # macOS
│   ├── cjxl          # JXL encoder
│   ├── avifenc       # AVIF encoder
│   ├── ffmpeg        # For animated AVIF
│   └── lib/          # Dynamic libraries
├── win32/            # Windows x64
│   ├── cjxl.exe
│   ├── avifenc.exe
│   └── *.dll
└── linux/            # Linux x64
    ├── cjxl
    ├── avifenc
    └── lib/
```

---

## Changelog

### v3.0.0 (2025-12-10)
- ✅ **Dual mode** - JXL + AVIF in one plugin
- ✅ **AVIF mode** - Sticker/meme optimization
- ✅ **Quality slider** - Adjustable AVIF quality
- ✅ **Animated GIF** - FFmpeg support
- ✅ **Renamed** - jpeg2jxl-vue → static-converter-vue

### v2.1.0
- Self-contained binaries
- Multi-platform support
- Customizable size threshold

---

MIT License
