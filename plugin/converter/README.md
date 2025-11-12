# 🎨 PIXLY Converter Plugin for Eagle

[![Version](https://img.shields.io/badge/version-4.3.0-blue.svg)](https://eagle.cool/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](../LICENSE)

AI-powered image conversion plugin for [Eagle App](https://eagle.cool/). Convert images to modern formats (AVIF, JXL, WebP) with intelligent optimization.

[中文文档](README.zh-CN.md) | [Main Project](../README.md)

---

## ✨ Features

### 🤖 Smart Mode (AI-Powered)
- **Automatic parameter prediction** using Light GBM
- **Format recommendation** based on image characteristics
- **SSIM quality validation** (≥95% similarity)
- **Bayesian optimization** for parameter tuning
- **PPO reinforcement learning** (experimental)

### ⚙️ Manual Mode
- **Full control** over all parameters
- **Format-specific settings** (JXL, AVIF, WebP, HEIC, PNG, JPEG)
- **Advanced options** (effort, speed, distance, lossless)
- **Real-time validation** of parameter combinations

### 🎬 Video Tools
- **Animated GIF → Video** conversion
- **Video format conversion** (H.264, H.265, AV1)
- **Batch GIF optimization**
- **Codec recommendations**

### 🛠️ Utility Tools
- **XMP Merge** - Combine XMP sidecar files
- **Metadata cleanup** - Remove or preserve metadata
- **Duplicate detection** - Find duplicate images
- **Batch repair** - Fix corrupted files

---

## 📦 Installation

### Option 1: Eagle Plugin Store (Recommended)
1. Open Eagle
2. Go to **Preferences** → **Plugins**
3. Search for "PIXLY Converter"
4. Click **Install**

### Option 2: Manual Installation
1. Download latest release from [Releases](https://github.com/yourusername/plxy-easy2jxlavif/releases)
2. Extract to Eagle plugins folder:
   - macOS: `~/Library/Application Support/Eagle/plugins/`
   - Windows: `%APPDATA%/Eagle/plugins/`
3. Restart Eagle

### Option 3: Development Install
```bash
# Clone repository
git clone https://github.com/yourusername/plxy-easy2jxlavif.git

# Link plugin folder
ln -s /path/to/plxy-easy2jxlavif/plugin_v3 ~/Library/Application\ Support/Eagle/plugins/pixly-converter

# Restart Eagle
```

---

## 🚀 Quick Start

### Basic Usage

1. **Select images** in Eagle library
2. **Right-click** → Plugins → **PIXLY Converter**
3. **Choose mode**:
   - **Smart Mode** - Let AI choose optimal settings
   - **Manual Mode** - Full control over parameters
4. **Configure settings** (quality, format, etc.)
5. Click **Start Conversion** ▶️

### Smart Mode Example
1. Select PNG images
2. Open PIXLY Converter
3. Click "Smart Mode" tab
4. Enable "AI Quality Prediction" ✓
5. Enable "Format Recommendation" ✓
6. Click "Start Conversion"
7. AI automatically:
   - Analyzes image features
   - Recommends JXL format
   - Predicts quality=92, effort=7
   - Validates with SSIM≥0.95

### Manual Mode Example
1. Select JPEG images
2. Open PIXLY Converter
3. Click "Manual Mode" tab
4. Select "AVIF" format
5. Set quality to 85
6. Set speed to 6
7. Click "Start Conversion"

---

## ⚙️ Configuration

### AI Options (Smart Mode)

#### Quality Prediction
- **Enable**: AI predicts optimal quality parameter
- **Range**: 1-100
- **Based on**: Image complexity, detail level, noise

#### Auto Optimization
- **Enable**: Auto-adjust parameters for best compression
- **Methods**: Bayesian optimization, PPO learning
- **Target**: Minimize size while maintaining quality

#### SSIM Validation
- **Enable**: Validate output quality ≥95% similarity
- **Threshold**: 0.90-0.99 (default: 0.95)
- **Fallback**: Auto-retry with higher quality if validation fails

#### Format Recommendation
- **Enable**: AI recommends best format
- **Factors**: Transparency, animation, complexity, size
- **Override**: Can specify preferred format (JXL, AVIF, WebP)

#### Video for Animation
- **Enable**: Convert animated GIFs to video (H.264/H.265)
- **Benefits**: Smaller size, better compatibility
- **Auto**: Only converts if video format is smaller

### Manual Options

#### Format Selection
- JXL, AVIF, WebP, HEIC, PNG, JPEG

#### Quality (1-100)
- **1-60**: Size priority
- **60-80**: Balanced
- **80-95**: Quality priority
- **95-100**: Lossless or near-lossless

#### Effort/Speed
- **Low (1-3)**: Fast conversion
- **Medium (4-6)**: Balanced
- **High (7-9)**: Best compression (slower)

#### Advanced
- **Lossless**: Enable lossless compression (JXL, WebP, PNG)
- **Distance**: JXL quality distance (0.0-15.0)
- **Workers**: Parallel conversion threads (1-16)

---

## 🎯 Use Cases

### Photography Workflow
```
1. Import RAW photos to Eagle
2. Select exported JPEGs
3. PIXLY → Smart Mode
4. Enable "Format Recommendation"
5. AI recommends JXL for high-quality images
6. Convert with SSIM validation
7. Original files preserved, new JXL files added
```

### Web Asset Optimization
```
1. Select PNG/JPEG website assets
2. PIXLY → Manual Mode
3. Choose AVIF (best browser support)
4. Quality: 80 (web-optimized)
5. Speed: 6 (fast encode)
6. Batch convert
7. Reduced bandwidth usage by 70%
```

### Animated GIF Modernization
```
1. Select old animated GIFs
2. PIXLY → Video Tools
3. Enable "Animated to Video"
4. Choose H.265 codec
5. Convert to MOV
6. File size reduced by 85%
```

---

## 🌍 Language Support

The plugin supports:
- **English** (Default)
- **简体中文** (Simplified Chinese)
- **日本語** (Japanese)

Language is auto-detected from Eagle's language setting.

---

## 🐛 Troubleshooting

### Plugin doesn't appear in Eagle
- Verify plugin folder location
- Check Eagle version (v3.0+ required)
- Restart Eagle completely

### "AI Service not available"
- Ensure AI service is running: `ai-service`
- Check port 3333 is accessible
- Try manual mode (works without AI service)

### "Rust service not available"
- Ensure Rust service is running: `rust-service`
- Check port 8080 is accessible
- Install required CLI tools

### Conversion fails
- **Check CLI tools**: `cjxl`, `avifenc`, `cwebp`
- **View logs**: Eagle → Preferences → Plugins → PIXLY → Logs
- **Debug mode**: Enable in plugin settings

### Poor quality results
- Increase quality setting (80+ recommended)
- Enable SSIM validation
- Try different format (JXL > AVIF > WebP for quality)

---

## 💡 Tips & Best Practices

### When to use Smart Mode
- ✅ Batch processing large collections
- ✅ Unfamiliar with conversion parameters
- ✅ Want AI to recommend format
- ✅ Need quality validation

### When to use Manual Mode
- ✅ Specific quality requirements
- ✅ Testing different settings
- ✅ AI service unavailable
- ✅ Prefer full control

### Format Selection Guide
- **JXL**: Best for high-quality photos, supports transparency
- **AVIF**: Great compression, wide browser support
- **WebP**: Good balance, universal compatibility
- **HEIC**: Apple ecosystem optimization
- **PNG**: Lossless required
- **JPEG**: Legacy compatibility

---

## 🔧 Advanced

### Custom Output Path
Configure output directory in plugin settings:
- **Same folder**: Output next to original
- **Custom folder**: Specify output directory
- **Subfolder**: Create format-specific subfolders

### Batch Processing
- **Smart queue**: Automatically manages conversion queue
- **Progress tracking**: Real-time progress updates
- **Error handling**: Failed conversions logged and retried
- **Resume support**: Can resume interrupted batches

### Metadata Options
- **Preserve EXIF**: Keep camera data
- **Preserve ICC Profile**: Keep color profile
- **Preserve XMP**: Keep Eagle metadata
- **Strip all**: Remove all metadata

---

## 📚 Related Documentation

- [Main Project README](../README.md)
- [AI Service README](../cmd/ai-service/README.md)
- [Rust Service README](../pixly-rust/README.md)
- [Preview Plugin](../plugin_preview/README.md)

---

## 📄 License

MIT License - see [LICENSE](../LICENSE) for details.

---

**Made for Eagle** | Part of [PIXLY Project](../README.md)
