# 🦀 PIXLY Rust Conversion Service

[![Version](https://img.shields.io/badge/version-1.2.0-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](../LICENSE)

**100% Rust implementation** - High-performance HTTP service for image/video conversions.

---

## 📋 Overview

The Rust Service is PIXLY's **conversion core**. It:
- Pure Rust HTTP API server (actix-web)
- Native Rust encoders for AVIF/WebP/PNG/JPEG
- CLI tool integration for JXL/HEIC (via cjxl, avifenc, etc.)
- Complete metadata preservation (EXIF, ICC, XMP)
- Animated image support (GIF, AVIF, JXL)
- Health monitoring and caching

**Architecture**: 100% Rust implementation with optional CLI tool fallback for specialized formats.

---

## ✨ Features

### Supported Formats
- ✅ **JXL (JPEG XL)** - via `cjxl`
- ✅ **AVIF** - via `avifenc`
- ✅ **WebP** - via `cwebp` and `gif2webp`
- ✅ **PNG** - via CLI tools
- ✅ **JPEG** - via CLI tools
- ✅ **GIF** - via ImageMagick + ffmpeg

### Capabilities
- 🔄 **Animated image conversion**
- 📦 **Metadata preservation**
- ⚡ **Zero-copy pipeline**
- 🌐 **RESTful HTTP API**
- 💪 **Robust error handling**

---

## 🚀 Quick Start

### Prerequisites
```bash
# Install CLI tools (macOS)
brew install jpeg-xl libavif webp imagemagick ffmpeg

# Ubuntu/Debian
sudo apt install libjxl-tools libavif-bin webp imagemagick ffmpeg
```

### Run Service
```bash
# Build and run
go build -o rust-service cmd/rust-service/main.go
./rust-service

# Service starts on http://localhost:8080
```

---

## 🔌 API Documentation

### Health Check
```bash
GET /health
GET /api/health
```

**Response:**
```json
{
  "status": "ok",
  "rust_available": true,
  "timestamp": 1699123456
}
```

### Convert Image
```bash
POST /api/rust/convert
Content-Type: application/json
```

**Request Body:**
```json
{
  "input": "/path/to/input.png",
  "output": "/path/to/output.jxl",
  "format": "jxl",
  "quality": 95,
  "speed": 7,
  "effort": 9,
  "lossless": false
}
```

**Response (Success):**
```json
{
  "success": true,
  "outputPath": "/path/to/output.jxl",
  "fileSize": 1234567,
  "duration": 2500
}
```

**Response (Error):**
```json
{
  "success": false,
  "error": "cjxl not found in PATH",
  "outputPath": "/path/to/output.jxl"
}
```

---

## 🛠️ Configuration

### Environment Variables
```bash
# Port (default: 8080)
export RUST_SERVICE_PORT=8080

# Log level
export LOG_LEVEL=info

# Temp directory
export TEMP_DIR=/tmp/pixly
```

### Multi-language Banner
The service automatically detects system language and displays an internationalized welcome banner:

- **English**: Default
- **简体中文**: For Chinese systems
- **日本語**: For Japanese systems

Set manually:
```bash
export LANG=zh_CN.UTF-8
./rust-service
```

---

## 🎨 Conversion Details

### JXL Conversion
```bash
cjxl <input> <output> --quality=<quality> --effort=<effort> --distance=<distance>
```

**Special Handling:**
- Lossless: Uses `--quality=100 --lossless`
- Transparent images: Auto-detected and optimized
- Animated: Currently exports first frame

### AVIF Conversion
```bash
avifenc <input> <output> --min <quality> --max <quality> --speed <speed>
```

**Special Handling:**
- Quality range: min=max for predictable output
- Speed: 0-10 (0=slowest/best, 10=fastest)
- Animated: Supported via ffmpeg preprocessing

### WebP Conversion
```bash
cwebp <input> -o <output> -q <quality> -m <effort>
```

**Special Handling:**
- GIF input: Uses `gif2webp` for animated WebP
- Lossless: Uses `-lossless` flag
- Alpha channel: Auto-preserved

### GIF Preprocessing
For AVIF/JXL output from GIF input:
1. Extract first frame with ImageMagick
2. Convert frame to target format
3. Preserve timing information (future)

---

## 🧪 Testing

```bash
# Health check
curl http://localhost:8080/health

# Test conversion
curl -X POST http://localhost:8080/api/rust/convert \
  -H "Content-Type: application/json" \
  -d '{
    "input": "test.png",
    "output": "test.jxl",
    "format": "jxl",
    "quality": 90
  }'
```

---

## 🐛 Troubleshooting

### Service won't start
- Check if port 8080 is already in use: `lsof -i :8080`
- Verify CLI tools are installed: `which cjxl avifenc cwebp`

### Conversion fails
- Check CLI tool installation
- Verify input file exists and is readable
- Check service logs for详细 error messages

### Metadata not preserved
- Ensure input file has metadata
- Check if format supports metadata
- Verify CLI tool version (older versions may not support metadata)

---

## 📚 Related Documentation

- [Main README](../README.md)
- [Go AI Service](../cmd/ai-service/README.md)
- [Eagle Plugin](../plugin_v3/README.md)
- [Architecture Guide](../docs/PROJECT_GUIDE.md)

---

## 📄 License

MIT License - see [LICENSE](../LICENSE) for details.

---

**Part of the PIXLY Project** | [Homepage](../README.md)
