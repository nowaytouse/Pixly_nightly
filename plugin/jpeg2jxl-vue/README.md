# Static to JXL Converter - Eagle Plugin

Convert static images to lossless JPEG XL (JXL) format with smart conversion logic. **Self-contained with bundled binaries - no system dependencies required.**

[English](#features) | [中文](#功能特性)

---

## Features

### ✅ Self-Contained (No System Dependencies)
This plugin includes pre-built `cjxl` binaries for all supported platforms. **No need to install libjxl or any other dependencies.**

### Supported Platforms
| Platform | Architecture | Status |
|----------|-------------|--------|
| macOS | ARM64 (Apple Silicon) | ✅ Supported |
| macOS | x64 (Intel) | ✅ Supported |
| Windows | x64 | ✅ Supported |
| Linux | x64 | ✅ Supported |
| Linux | ARM64 | 🔄 Planned |

### Supported Formats
| Format | Mode | Description |
|--------|------|-------------|
| JPEG | `--lossless_jpeg=1` | **Reversible transcode** - preserves DCT coefficients |
| PNG | `-d 0` | Mathematical lossless (≥1.25MB by default) |
| BMP | `-d 0` | Mathematical lossless (≥1.25MB by default) |
| TIFF | `-d 0` | Mathematical lossless (≥1.25MB by default) |
| TGA | `-d 0` | Mathematical lossless (≥1.25MB by default) |
| PPM/PBM/PGM | `-d 0` | Mathematical lossless (≥1.25MB by default) |

### Smart Conversion Logic
- **JPEG**: Always convert using reversible transcode (can be converted back to identical JPEG)
- **Lossless sources**: Only convert if file size ≥ threshold (default 1.25MB, customizable)
- **Smart rollback**: Skip if JXL output is larger than original
- **Metadata preservation**: EXIF/IPTC/XMP/ICC via exiftool (if bundled)
- **Timestamp preservation**: mtime/atime preserved

### Customizable Settings
- **Size threshold**: Default 1.25MB, adjustable in KB or MB
- **Disable filter**: Set threshold to 0 to convert all files
- **Multi-language**: English, 简体中文, 繁體中文, 日本語

## Usage

1. Select images in Eagle (JPEG/PNG/BMP/TIFF/TGA)
2. Open "Static to JXL Converter" plugin
3. Adjust size threshold if needed
4. Review files (skipped files shown with reason)
5. Click "Convert"
6. Wait for completion

## Installation

### From Eagle Plugin Center
1. Open Eagle → Plugins → Plugin Center
2. Search "Static to JXL"
3. Install

### Manual Installation
1. Download plugin folder
2. Eagle → Plugins → Developer → Import Local Plugin
3. Select `jpeg2jxl-vue` folder

## Binary Structure

```
bin/
├── darwin/           # macOS (Universal Binary: ARM64 + x64)
│   ├── cjxl
│   ├── exiftool      # Optional
│   └── lib/          # Dynamic libraries
│       └── libjxl*.dylib
├── win32/            # Windows x64
│   ├── cjxl.exe
│   └── *.dll         # Dynamic libraries
└── linux/            # Linux x64
    ├── cjxl
    └── lib/          # Dynamic libraries
        └── libjxl*.so
```

## Building Binaries

### macOS (Universal Binary)
```bash
# Build for both ARM64 and x64
brew install jpeg-xl
# Copy from /opt/homebrew/bin/cjxl (ARM64) or /usr/local/bin/cjxl (x64)
# Or build universal binary with lipo
```

### Windows
```bash
# Download from https://github.com/libjxl/libjxl/releases
# Extract cjxl.exe and required DLLs
```

### Linux
```bash
# Build from source or download from releases
# Include required .so files in lib/
```

---

## 功能特性

### ✅ 自包含（无需系统依赖）
此插件包含所有支持平台的预编译 `cjxl` 二进制文件。**无需安装 libjxl 或任何其他依赖。**

### 支持的平台
| 平台 | 架构 | 状态 |
|------|------|------|
| macOS | ARM64 (Apple Silicon) | ✅ 支持 |
| macOS | x64 (Intel) | ✅ 支持 |
| Windows | x64 | ✅ 支持 |
| Linux | x64 | ✅ 支持 |
| Linux | ARM64 | 🔄 计划中 |

### 支持的格式
| 格式 | 模式 | 说明 |
|------|------|------|
| JPEG | `--lossless_jpeg=1` | **可逆转码** - 保留 DCT 系数 |
| PNG | `-d 0` | 数学无损（默认 ≥1.25MB） |
| BMP | `-d 0` | 数学无损（默认 ≥1.25MB） |
| TIFF | `-d 0` | 数学无损（默认 ≥1.25MB） |
| TGA | `-d 0` | 数学无损（默认 ≥1.25MB） |
| PPM/PBM/PGM | `-d 0` | 数学无损（默认 ≥1.25MB） |

### 智能转换逻辑
- **JPEG**: 始终使用可逆转码（可完美还原为原始 JPEG）
- **无损源**: 仅当文件 ≥ 阈值时转换（默认 1.25MB，可自定义）
- **智能回退**: 如果 JXL 输出更大则跳过
- **元数据保留**: 通过 exiftool 保留 EXIF/IPTC/XMP/ICC（如已内置）
- **时间戳保留**: 保留 mtime/atime

### 可自定义设置
- **大小阈值**: 默认 1.25MB，可调整为 KB 或 MB
- **关闭过滤**: 设为 0 转换所有文件
- **多语言**: English, 简体中文, 繁體中文, 日本語

## 使用方法

1. 在 Eagle 中选择图像（JPEG/PNG/BMP/TIFF/TGA）
2. 打开 "Static to JXL Converter" 插件
3. 根据需要调整大小阈值
4. 查看文件（跳过的文件会显示原因）
5. 点击 "转换"
6. 等待完成

---

## Changelog

### v2.1.0 (2025-12-10)
- ✅ **Self-contained binaries** - no system dependencies
- ✅ Multi-platform support (macOS ARM64/x64, Windows x64, Linux x64)
- ✅ Customizable size threshold (default 1.25MB)
- ✅ Improved UI with larger fonts
- ✅ Help modal (click-to-close disabled)
- ✅ Resizable window

### v2.0.0 (2025-12-10)
- ✅ Enhanced with static2jxl kernel
- ✅ Added PNG/BMP/TIFF/TGA/PPM support
- ✅ Smart size threshold
- ✅ Smart rollback (skip if JXL larger)
- ✅ Metadata preservation via exiftool
- ✅ Timestamp preservation

### v1.0.0
- Initial release (JPEG only)

---

MIT License
