#!/bin/bash
# Download and prepare cjxl + avifenc binaries for all platforms
# Usage: ./scripts/download-binaries.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PLUGIN_DIR="$(dirname "$SCRIPT_DIR")"
BIN_DIR="$PLUGIN_DIR/bin"

# Versions
LIBJXL_VERSION="0.11.1"
LIBAVIF_VERSION="1.1.1"

echo "📦 Static Converter - Binary Setup"
echo "   libjxl: v${LIBJXL_VERSION}"
echo "   libavif: v${LIBAVIF_VERSION}"
echo "   Target: $BIN_DIR"

mkdir -p "$BIN_DIR/darwin/lib"
mkdir -p "$BIN_DIR/win32"
mkdir -p "$BIN_DIR/linux/lib"

# ============================================
# macOS (from Homebrew)
# ============================================
echo ""
echo "🍎 macOS..."

if [[ "$(uname)" == "Darwin" ]]; then
    BREW_PREFIX=$([[ "$(uname -m)" == "arm64" ]] && echo "/opt/homebrew" || echo "/usr/local")
    
    # cjxl
    if [[ -f "$BREW_PREFIX/bin/cjxl" ]]; then
        echo "   Copying cjxl..."
        cp "$BREW_PREFIX/bin/cjxl" "$BIN_DIR/darwin/"
    else
        echo "   ⚠️  cjxl not found. Install: brew install jpeg-xl"
    fi
    
    # avifenc
    if [[ -f "$BREW_PREFIX/bin/avifenc" ]]; then
        echo "   Copying avifenc..."
        cp "$BREW_PREFIX/bin/avifenc" "$BIN_DIR/darwin/"
    else
        echo "   ⚠️  avifenc not found. Install: brew install libavif"
    fi
    
    # ffmpeg (optional)
    if [[ -f "$BREW_PREFIX/bin/ffmpeg" ]]; then
        echo "   Copying ffmpeg..."
        cp "$BREW_PREFIX/bin/ffmpeg" "$BIN_DIR/darwin/"
    fi
    
    # exiftool (optional)
    if [[ -f "$BREW_PREFIX/bin/exiftool" ]]; then
        echo "   Copying exiftool..."
        cp "$BREW_PREFIX/bin/exiftool" "$BIN_DIR/darwin/"
    fi
    
    # Copy dynamic libraries
    echo "   Copying libraries..."
    for lib in "$BREW_PREFIX"/lib/libjxl*.dylib "$BREW_PREFIX"/lib/libhwy*.dylib \
               "$BREW_PREFIX"/lib/libbrotli*.dylib "$BREW_PREFIX"/lib/liblcms2*.dylib \
               "$BREW_PREFIX"/lib/libpng*.dylib "$BREW_PREFIX"/lib/libjpeg*.dylib \
               "$BREW_PREFIX"/lib/libgif*.dylib "$BREW_PREFIX"/lib/libavif*.dylib \
               "$BREW_PREFIX"/lib/libaom*.dylib "$BREW_PREFIX"/lib/libdav1d*.dylib \
               "$BREW_PREFIX"/lib/librav1e*.dylib "$BREW_PREFIX"/lib/libSvtAv1*.dylib; do
        [[ -f "$lib" ]] && cp "$lib" "$BIN_DIR/darwin/lib/" 2>/dev/null || true
    done
    
    # Fix library paths
    for bin in cjxl avifenc ffmpeg; do
        if [[ -f "$BIN_DIR/darwin/$bin" ]]; then
            install_name_tool -add_rpath @executable_path/lib "$BIN_DIR/darwin/$bin" 2>/dev/null || true
            chmod +x "$BIN_DIR/darwin/$bin"
        fi
    done
    
    echo "   ✅ macOS ready"
else
    echo "   ⏭️  Skipping (not on macOS)"
fi

# ============================================
# Windows x64
# ============================================
echo ""
echo "🪟 Windows x64..."

# cjxl
WINDOWS_JXL="jxl-x64-windows-static-${LIBJXL_VERSION}.zip"
curl -L -o "/tmp/${WINDOWS_JXL}" "https://github.com/libjxl/libjxl/releases/download/v${LIBJXL_VERSION}/${WINDOWS_JXL}" 2>/dev/null && {
    unzip -o "/tmp/${WINDOWS_JXL}" -d "/tmp/jxl-win32" 2>/dev/null || true
    find /tmp/jxl-win32 -name "cjxl.exe" -exec cp {} "$BIN_DIR/win32/" \; 2>/dev/null || true
    find /tmp/jxl-win32 -name "*.dll" -exec cp {} "$BIN_DIR/win32/" \; 2>/dev/null || true
    rm -rf "/tmp/jxl-win32" "/tmp/${WINDOWS_JXL}"
    echo "   ✅ cjxl.exe ready"
} || echo "   ⚠️  cjxl download failed"

# avifenc - need to build or download from other source
echo "   ℹ️  avifenc.exe: Download from https://github.com/AOM-AV1/libavif/releases"

# ============================================
# Linux x64
# ============================================
echo ""
echo "🐧 Linux x64..."

# cjxl
LINUX_JXL="jxl-x86_64-linux-gnu-static-${LIBJXL_VERSION}.tar.gz"
curl -L -o "/tmp/${LINUX_JXL}" "https://github.com/libjxl/libjxl/releases/download/v${LIBJXL_VERSION}/${LINUX_JXL}" 2>/dev/null && {
    mkdir -p /tmp/jxl-linux
    tar -xzf "/tmp/${LINUX_JXL}" -C /tmp/jxl-linux 2>/dev/null || true
    find /tmp/jxl-linux -name "cjxl" -type f -exec cp {} "$BIN_DIR/linux/" \; 2>/dev/null || true
    chmod +x "$BIN_DIR/linux/cjxl" 2>/dev/null || true
    rm -rf "/tmp/jxl-linux" "/tmp/${LINUX_JXL}"
    echo "   ✅ cjxl ready"
} || echo "   ⚠️  cjxl download failed"

echo "   ℹ️  avifenc: Install via package manager or build from source"

# ============================================
# Summary
# ============================================
echo ""
echo "📊 Summary:"
echo "   darwin/cjxl:     $([ -f "$BIN_DIR/darwin/cjxl" ] && echo "✅" || echo "❌")"
echo "   darwin/avifenc:  $([ -f "$BIN_DIR/darwin/avifenc" ] && echo "✅" || echo "❌")"
echo "   darwin/ffmpeg:   $([ -f "$BIN_DIR/darwin/ffmpeg" ] && echo "✅" || echo "❌")"
echo "   win32/cjxl.exe:  $([ -f "$BIN_DIR/win32/cjxl.exe" ] && echo "✅" || echo "❌")"
echo "   win32/avifenc.exe: $([ -f "$BIN_DIR/win32/avifenc.exe" ] && echo "✅" || echo "❌")"
echo "   linux/cjxl:      $([ -f "$BIN_DIR/linux/cjxl" ] && echo "✅" || echo "❌")"
echo "   linux/avifenc:   $([ -f "$BIN_DIR/linux/avifenc" ] && echo "✅" || echo "❌")"
echo ""
echo "Done!"
