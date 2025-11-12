# 👁️ PIXLY Preview Plugin for Eagle

[![Version](https://img.shields.io/badge/version-2.1.0-green.svg)](https://eagle.cool/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](../LICENSE)

Native preview plugin for **JXL** and **AVIF** images in [Eagle App](https://eagle.cool/). Supports animated images with FFplay integration.

[中文文档](README.zh-CN.md) | [Main Project](../README.md)

---

## ✨ Features

### 🖼️ Format Support
- **JXL (JPEG XL)** - Next-gen image format
- **AVIF** - Modern compression format
- **Animated images** - Detect and play animations
- **Thumbnail generation** - Eagle library thumbnails

### 🎬 Animation Support
- **Auto-detection** - Automatically detect animated images
- **Frame info** - Show frame count, duration, FPS
- **FFplay integration** - External player for smooth playback
- **Fallback rendering** - Static preview if playback fails

### 🔧 Utilities
- **Image size** - Display resolution
- **File size** - Show compressed size
- **Format info** - Technical details
- **Quick preview** - Fast loading

---

## 📦 Installation

### Option 1: Eagle Plugin Store (Recommended)
1. Open Eagle
2. Go to **Preferences** → **Plugins**
3. Search for "PIXLY Preview"
4. Click **Install**

### Option 2: Manual Installation
1. Download from [Releases](https://github.com/yourusername/plxy-easy2jxlavif/releases)
2. Extract to Eagle plugins folder:
   - macOS: `~/Library/Application Support/Eagle/plugins/`
   - Windows: `%APPDATA%/Eagle/plugins/`
3. Restart Eagle

### Option 3: Development Install
```bash
# Clone repository
git clone https://github.com/yourusername/plxy-easy2jxlavif.git

# Link plugin folder
ln -s /path/to/plxy-easy2jxlavif/plugin_preview ~/Library/Application\ Support/Eagle/plugins/pixly-preview

# Restart Eagle
```

---

## 🚀 Quick Start

### Automatic Preview
1. Import JXL or AVIF images into Eagle
2. Click any image to preview
3. PIXLY Preview automatically handles rendering

### Play Animations
1. Open an animated JXL or AVIF
2. Plugin auto-detects animation
3. Click **▶️ Play with FFplay** button
4. External player opens with smooth playback

### Thumbnail Generation
- JXL and AVIF thumbnails are generated automatically
- Thumbnails appear in Eagle library grid
- Fast loading with caching

---

## 🛠️ Dependencies

### Required Tools

#### For JXL Preview
```bash
# Install djxl (JPEG XL decoder)
brew install jpeg-xl  # macOS
sudo apt install libjxl-tools  # Ubuntu

# Install ffprobe (for animation detection)
brew install ffmpeg  # macOS
sudo apt install ffmpeg  # Ubuntu
```

#### For AVIF Preview
```bash
# Install avifdec (AVIF decoder)
brew install libavif  # macOS
sudo apt install libavif-bin  # Ubuntu

# Install ffprobe (for animation detection)
brew install ffmpeg  # macOS
sudo apt install ffmpeg  # Ubuntu
```

#### For Animation Playback
```bash
# Install ffplay (video player)
brew install ffmpeg  # macOS (includes ffplay)
sudo apt install ffmpeg  # Ubuntu
```

---

## ⚙️ Configuration

### Viewer Settings

**JXL Viewer** (`viewer/jxl.html`):
- Auto-detects animations via `ffprobe`
- Decodes images using `djxl`
- Displays static preview
- Offers FFplay playback for animations

**AVIF Viewer** (`viewer/avif.html`):
- Auto-detects animations via `avifdec --info`
- Calculates dynamic FPS from timing
- Displays static preview
- Offers FFplay playback for animations

### Thumbnail Settings

**JXL Thumbnails** (`thumbnail/jxl.js`):
- Generates 400x400 thumbnails
- Caches in Eagle's thumbnail directory
- Fallback to default icon if decode fails

**AVIF Thumbnails** (`thumbnail/avif.js`):
- Generates 400x400 thumbnails
- Caches in Eagle's thumbnail directory
- Fallback to default icon if decode fails

---

## 🎯 Use Cases

### Photography Library
```
1. Import JXL photos into Eagle
2. Browse library with thumbnail preview
3. Click to view full-resolution preview
4. Animated JXLs show frame count and play button
```

### Web Asset Management
```
1. Download AVIF web assets
2. Import to Eagle for organization
3. Preview directly in Eagle (no conversion needed)
4. Share or export as needed
```

### Animation Collection
```
1. Collect animated AVIF/JXL files
2. View in Eagle with preview plugin
3. Click play button to see smooth playback
4. FFplay opens in windowed mode (800x600)
```

---

## 🐛 Troubleshooting

### Preview shows "Unable to load JXL/AVIF file"
- **Check tools**: Verify `djxl` or `avifdec` installed
- **Check PATH**: Ensure tools are in system PATH
- **File corruption**: Try opening file in other viewers
- **File extension**: Ensure file has correct extension (.jxl or .avif)

### Animation not detected
- **Check ffprobe**: `which ffprobe`
- **Check format**: Some older AVIF files may not report frames correctly
- **Manual check**: Use `avifdec --info file.avif` to verify

### FFplay button not appearing
- **Install ffplay**: `brew install ffmpeg` (includes ffplay)
- **Check PATH**: `which ffplay`
- **Restart Eagle**: Reload plugin after installing

### FFplay opens full-screen or in background
- **Current behavior**: Opens in 800x600 window, foreground mode
- **If issues persist**: Check ffplay version (`ffplay -version`)
- **Alternative**: Use system default video player

### Thumbnails not generating
- **Check permissions**: Eagle needs read/write to thumbnail directory
- **Check tools**: Verify decoders installed
- **Clear cache**: Delete Eagle thumbnail cache and reload

---

## 💡 Tips & Best Practices

### For Best Performance
- Keep CLI tools updated to latest versions
- Use SSD for Eagle library (faster decoding)
- Clear thumbnail cache periodically

### For Animated Images
- FFplay provides smoothest playback
- Static preview shows first frame
- Frame count and duration displayed in status bar

### For Large Collections
- Thumbnails are cached (generated once)
- Preview is fast after first load
- FFplay playback is immediate (no pre-processing)

---

## 🔧 Advanced

### Custom Viewer Configuration

Edit viewer HTML files to customize behavior:
- `viewer/jxl.html` - JXL viewer settings
- `viewer/avif.html` - AVIF viewer settings

### Custom Thumbnail Size

Edit thumbnail JavaScript files:
- `thumbnail/jxl.js` - JXL thumbnail generator
- `thumbnail/avif.js` - AVIF thumbnail generator

Change `targetSize` to desired resolution:
```javascript
const targetSize = 400; // Default: 400x400
```

### FFplay Parameters

Current FFplay command:
```bash
ffplay -x 800 -y 600 -alwaysontop <file>
```

To customize, edit `viewer/*.html` and modify `playWithFFplay()` function.

---

## 📚 Related Documentation

- [Main Project README](../README.md)
- [Converter Plugin](../plugin_v3/README.md)
- [AI Service](../cmd/ai-service/README.md)
- [Rust Service](../pixly-rust/README.md)

---

## 📄 License

MIT License - see [LICENSE](../LICENSE) for details.

---

**Made for Eagle** | Part of [PIXLY Project](../README.md)
