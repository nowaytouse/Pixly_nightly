# Pixly Standalone Tools

**Independent Command-Line Media Conversion Tools**

## 📦 Overview

Pixly Standalone Tools is a collection of independent, specialized command-line utilities for media format conversion. Each tool is optimized for a specific task and can be used without any dependencies on other Pixly components.

## 🛠️ Tools

### Image Converters

| Tool | Description | Use Case |
|------|-------------|----------|
| `all2jxl` | Convert any image to JXL | Universal JXL converter |
| `all2avif` | Convert any image to AVIF | Universal AVIF converter |
| `static2jxl` | Static images to JXL | Photos, illustrations |
| `static2avif` | Static images to AVIF | Web optimization |
| `dynamic2jxl` | Animated images to JXL | Animated graphics |
| `dynamic2avif` | Animated images to AVIF | Web animations |

### Video Converters

| Tool | Description | Use Case |
|------|-------------|----------|
| `dynamic2mov` | Animated images to MOV/MP4 | Create videos from GIFs |
| `dynamic2h266mov` | Animated images to H.266 | Modern video codec |
| `video2mov` | Video format conversion | Standardize video formats |

### Utilities

| Tool | Description | Use Case |
|------|-------------|----------|
| `deduplicate_media` | Remove duplicate files | Clean up media library |
| `merge_xmp` | XMP metadata operations | Preserve metadata |
| `PIXLY_media_tools` | Media analysis tools | File inspection |
| `PIXLY_universal_converter` | Universal converter | One-stop conversion |

## 📦 Installation

### From Source

```bash
# Navigate to standalone_tools
cd standalone_tools

# Build all tools
make all

# Or build specific tool
make all2jxl
make all2avif

# Install to system (optional)
sudo make install
```

### Dependencies

```bash
# macOS
brew install jpeg-xl libavif ffmpeg

# Ubuntu/Debian
apt-get install libjxl-dev libavif-dev ffmpeg

# Arch Linux
pacman -S libjxl libavif ffmpeg
```

## 🚀 Quick Start

### Basic Usage

```bash
# Convert single file to JXL
./bin/all2jxl image.png

# Convert multiple files to AVIF
./bin/all2avif *.jpg *.png

# Convert animated GIF to MOV
./bin/dynamic2mov animation.gif

# Remove duplicate media files
./bin/deduplicate_media ./photos
```

### Advanced Usage

```bash
# Convert with specific quality
./bin/all2jxl -q 90 image.png

# Convert with custom effort
./bin/all2avif -e 8 -q 85 photo.jpg

# Convert and preserve metadata
./bin/static2jxl --keep-metadata image.png

# Batch conversion with progress
./bin/all2jxl -v ./images/*.png
```

## 📖 Tool Details

### all2jxl

**Universal JXL Converter** - Automatically detects static/animated images and applies optimal encoder.

```bash
# Usage
all2jxl [OPTIONS] FILES...

# Options
-q, --quality     Quality (1-100, default: 90)
-e, --effort      Encoding effort (1-9, default: 7)
-l, --lossless    Lossless mode
-d, --distance    Psychovisual distance (0.0-15.0)
-v, --verbose     Verbose output
-o, --output      Output directory

# Examples
./bin/all2jxl image.png
./bin/all2jxl -q 95 -e 9 photo.jpg
./bin/all2jxl --lossless animation.gif
```

**Features:**
- Auto-detection of static/animated
- Metadata preservation
- Progress tracking
- Batch processing

### all2avif

**Universal AVIF Converter** - Optimized AVIF encoding for all image types.

```bash
# Usage
all2avif [OPTIONS] FILES...

# Options
-q, --quality     Quality (1-100, default: 85)
-s, --speed       Encoding speed (0-10, default: 6)
-c, --chroma      Chroma subsampling (444/422/420)
-v, --verbose     Verbose output

# Examples
./bin/all2avif image.png
./bin/all2avif -q 90 -s 4 photo.jpg
./bin/all2avif -c 444 high-quality.png
```

**Features:**
- Multiple quality presets
- Chroma subsampling control
- Alpha channel support
- Batch conversion

### dynamic2mov

**Animated Image to Video Converter** - Convert GIFs and animated images to video formats.

```bash
# Usage
dynamic2mov [OPTIONS] INPUT

# Options
-f, --format      Output format (mov/mp4, default: mov)
-c, --codec       Video codec (h264/h265/vp9, default: h264)
-q, --quality     Quality (crf 0-51, default: 23)
-r, --fps         Frame rate (default: auto)

# Examples
./bin/dynamic2mov animation.gif
./bin/dynamic2mov -f mp4 -c h265 animation.webp
./bin/dynamic2mov --fps 30 -q 18 animation.gif
```

**Features:**
- Multiple output formats
- Codec selection
- Frame rate control
- Quality presets

### deduplicate_media

**Duplicate Media Remover** - Find and remove duplicate media files based on content hash.

```bash
# Usage
deduplicate_media [OPTIONS] DIRECTORY

# Options
-d, --dry-run     Show duplicates without deleting
-r, --recursive   Scan subdirectories
-m, --min-size    Minimum file size (default: 1KB)
-v, --verbose     Verbose output

# Examples
./bin/deduplicate_media ./photos
./bin/deduplicate_media -d -r ./media
./bin/deduplicate_media --min-size 100KB ./images
```

**Features:**
- Content-based detection
- Safe dry-run mode
- Recursive scanning
- Size filtering

## 📊 Performance

### Conversion Speed

| Tool | Speed | Notes |
|------|-------|-------|
| all2jxl | ~15 images/sec | Depends on quality settings |
| all2avif | ~10 images/sec | Slower than JXL |
| dynamic2mov | ~30 fps | Depends on frame count |
| deduplicate | ~100 files/sec | Hash computation |

### Quality vs. Speed

```bash
# Fast (effort 3-5)
./bin/all2jxl -e 3 -q 85 image.png  # ~2x faster

# Balanced (effort 6-7) ⭐ Recommended
./bin/all2jxl -e 7 -q 90 image.png  # Default

# Quality (effort 8-9)
./bin/all2jxl -e 9 -q 95 image.png  # ~2x slower, +5% quality
```

## ⚙️ Configuration

### Environment Variables

```bash
# Set default quality
export PIXLY_QUALITY=90

# Set default effort
export PIXLY_EFFORT=7

# Set output directory
export PIXLY_OUTPUT_DIR=./converted

# Enable verbose mode
export PIXLY_VERBOSE=1
```

### Configuration File

Create `~/.pixlyrc`:

```ini
[defaults]
quality = 90
effort = 7
keep_metadata = true
output_dir = ./converted

[all2jxl]
quality = 95
distance = 0.5

[all2avif]
quality = 85
speed = 6
chroma = 444
```

## 🔧 Troubleshooting

### Tool Not Found

```bash
# Check if built
ls -la bin/

# Rebuild
make clean && make all

# Check PATH
export PATH=$PATH:$(pwd)/bin
```

### Conversion Failed

```bash
# Check dependencies
which cjxl avifenc ffmpeg

# Run with verbose mode
./bin/all2jxl -v image.png

# Check input file
file image.png
```

### Permission Denied

```bash
# Make executable
chmod +x bin/*

# Or use make
make install-permissions
```

## 🛠️ Development

### Build System

```makefile
# Build all tools
make all

# Build specific tool
make all2jxl

# Clean build artifacts
make clean

# Install to system
sudo make install

# Uninstall
sudo make uninstall
```

### Project Structure

```
standalone_tools/
├── Makefile              # Build configuration
├── go.mod                # Go module file
├── bin/                  # Compiled binaries
├── all2jxl/             # JXL converter
│   ├── main.go
│   └── encoder.go
├── all2avif/            # AVIF converter
│   ├── main.go
│   └── encoder.go
├── dynamic2mov/         # Video converter
│   ├── main.go
│   └── converter.go
└── utils/               # Shared utilities
    ├── progress.go
    └── metadata.go
```

### Adding New Tools

1. Create new directory: `mkdir mytool`
2. Add main.go with conversion logic
3. Update Makefile:
   ```makefile
   mytool:
       go build -o bin/mytool ./mytool
   ```
4. Build: `make mytool`

## 📚 Examples

### Batch Processing

```bash
# Convert all PNGs to JXL
find ./images -name "*.png" -exec ./bin/all2jxl {} \;

# Convert with custom naming
for f in *.jpg; do
  ./bin/all2avif -q 90 "$f" -o "${f%.jpg}.avif"
done

# Parallel processing
ls *.png | parallel -j4 ./bin/all2jxl {}
```

### Pipeline Operations

```bash
# Deduplicate then convert
./bin/deduplicate_media ./photos && \
./bin/all2jxl ./photos/*.png

# Convert and verify
./bin/all2jxl image.png && \
./bin/PIXLY_media_tools analyze image.jxl
```

### Script Integration

```bash
#!/bin/bash
# mass_convert.sh

QUALITY=90
EFFORT=7

for file in "$@"; do
  echo "Converting: $file"
  ./bin/all2jxl -q $QUALITY -e $EFFORT "$file"
  
  if [ $? -eq 0 ]; then
    echo "✓ Success: $file"
  else
    echo "✗ Failed: $file"
  fi
done
```

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](../CONTRIBUTING.md)

### Guidelines

1. Follow Go best practices
2. Add tests for new features
3. Update documentation
4. Test on multiple platforms

## 📄 License

MIT License - see [LICENSE](../LICENSE)

---

**Standalone tools for maximum flexibility** 🛠️
