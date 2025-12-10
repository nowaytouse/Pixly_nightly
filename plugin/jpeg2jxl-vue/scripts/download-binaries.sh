#!/bin/bash
# Download and prepare cjxl binaries for all platforms
# Usage: ./scripts/download-binaries.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PLUGIN_DIR="$(dirname "$SCRIPT_DIR")"
BIN_DIR="$PLUGIN_DIR/bin"

# libjxl version
LIBJXL_VERSION="0.11.1"
LIBJXL_RELEASE="https://github.com/libjxl/libjxl/releases/download/v${LIBJXL_VERSION}"

echo "📦 Downloading libjxl v${LIBJXL_VERSION} binaries..."
echo "   Target: $BIN_DIR"

# Create directories
mkdir -p "$BIN_DIR/darwin/lib"
mkdir -p "$BIN_DIR/win32"
mkdir -p "$BIN_DIR/linux/lib"

# ============================================
# macOS (Universal Binary from Homebrew)
# ============================================
echo ""
echo "🍎 macOS..."

if [[ "$(uname)" == "Darwin" ]]; then
    # Copy from Homebrew
    if [[ -f "/opt/homebrew/bin/cjxl" ]]; then
        echo "   Copying from Homebrew (ARM64)..."
        cp /opt/homebrew/bin/cjxl "$BIN_DIR/darwin/"
        
        # Copy dynamic libraries
        for lib in /opt/homebrew/lib/libjxl*.dylib /opt/homebrew/lib/libhwy*.dylib \
                   /opt/homebrew/lib/libbrotli*.dylib /opt/homebrew/lib/liblcms2*.dylib \
                   /opt/homebrew/lib/libpng*.dylib /opt/homebrew/lib/libjpeg*.dylib \
                   /opt/homebrew/lib/libgif*.dylib; do
            if [[ -f "$lib" ]]; then
                cp "$lib" "$BIN_DIR/darwin/lib/" 2>/dev/null || true
            fi
        done
        
        echo "   ✅ macOS ARM64 ready"
    elif [[ -f "/usr/local/bin/cjxl" ]]; then
        echo "   Copying from Homebrew (x64)..."
        cp /usr/local/bin/cjxl "$BIN_DIR/darwin/"
        
        # Copy dynamic libraries
        for lib in /usr/local/lib/libjxl*.dylib /usr/local/lib/libhwy*.dylib \
                   /usr/local/lib/libbrotli*.dylib /usr/local/lib/liblcms2*.dylib \
                   /usr/local/lib/libpng*.dylib /usr/local/lib/libjpeg*.dylib \
                   /usr/local/lib/libgif*.dylib; do
            if [[ -f "$lib" ]]; then
                cp "$lib" "$BIN_DIR/darwin/lib/" 2>/dev/null || true
            fi
        done
        
        echo "   ✅ macOS x64 ready"
    else
        echo "   ⚠️  cjxl not found. Install with: brew install jpeg-xl"
    fi
    
    # Fix library paths
    if [[ -f "$BIN_DIR/darwin/cjxl" ]]; then
        echo "   Fixing library paths..."
        install_name_tool -add_rpath @executable_path/lib "$BIN_DIR/darwin/cjxl" 2>/dev/null || true
        chmod +x "$BIN_DIR/darwin/cjxl"
    fi
else
    echo "   ⏭️  Skipping (not on macOS)"
fi

# ============================================
# Windows x64
# ============================================
echo ""
echo "🪟 Windows x64..."

WINDOWS_ZIP="jxl-x64-windows-static-${LIBJXL_VERSION}.zip"
WINDOWS_URL="${LIBJXL_RELEASE}/${WINDOWS_ZIP}"

if command -v curl &> /dev/null; then
    echo "   Downloading from GitHub releases..."
    curl -L -o "/tmp/${WINDOWS_ZIP}" "$WINDOWS_URL" 2>/dev/null || {
        echo "   ⚠️  Download failed. Manual download required:"
        echo "      $WINDOWS_URL"
    }
    
    if [[ -f "/tmp/${WINDOWS_ZIP}" ]]; then
        echo "   Extracting..."
        unzip -o "/tmp/${WINDOWS_ZIP}" -d "/tmp/jxl-win32" 2>/dev/null || true
        
        # Find and copy cjxl.exe
        find /tmp/jxl-win32 -name "cjxl.exe" -exec cp {} "$BIN_DIR/win32/" \; 2>/dev/null || true
        
        # Copy DLLs if any
        find /tmp/jxl-win32 -name "*.dll" -exec cp {} "$BIN_DIR/win32/" \; 2>/dev/null || true
        
        rm -rf "/tmp/jxl-win32" "/tmp/${WINDOWS_ZIP}"
        
        if [[ -f "$BIN_DIR/win32/cjxl.exe" ]]; then
            echo "   ✅ Windows x64 ready"
        else
            echo "   ⚠️  cjxl.exe not found in archive"
        fi
    fi
else
    echo "   ⚠️  curl not found. Manual download required."
fi

# ============================================
# Linux x64
# ============================================
echo ""
echo "🐧 Linux x64..."

LINUX_TAR="jxl-x86_64-linux-gnu-static-${LIBJXL_VERSION}.tar.gz"
LINUX_URL="${LIBJXL_RELEASE}/${LINUX_TAR}"

if command -v curl &> /dev/null; then
    echo "   Downloading from GitHub releases..."
    curl -L -o "/tmp/${LINUX_TAR}" "$LINUX_URL" 2>/dev/null || {
        echo "   ⚠️  Download failed. Manual download required:"
        echo "      $LINUX_URL"
    }
    
    if [[ -f "/tmp/${LINUX_TAR}" ]]; then
        echo "   Extracting..."
        mkdir -p /tmp/jxl-linux
        tar -xzf "/tmp/${LINUX_TAR}" -C /tmp/jxl-linux 2>/dev/null || true
        
        # Find and copy cjxl
        find /tmp/jxl-linux -name "cjxl" -type f -exec cp {} "$BIN_DIR/linux/" \; 2>/dev/null || true
        
        # Copy .so files if any
        find /tmp/jxl-linux -name "*.so*" -exec cp {} "$BIN_DIR/linux/lib/" \; 2>/dev/null || true
        
        rm -rf "/tmp/jxl-linux" "/tmp/${LINUX_TAR}"
        
        if [[ -f "$BIN_DIR/linux/cjxl" ]]; then
            chmod +x "$BIN_DIR/linux/cjxl"
            echo "   ✅ Linux x64 ready"
        else
            echo "   ⚠️  cjxl not found in archive"
        fi
    fi
else
    echo "   ⚠️  curl not found. Manual download required."
fi

# ============================================
# Summary
# ============================================
echo ""
echo "📊 Summary:"
echo "   darwin/cjxl:  $([ -f "$BIN_DIR/darwin/cjxl" ] && echo "✅" || echo "❌")"
echo "   win32/cjxl.exe: $([ -f "$BIN_DIR/win32/cjxl.exe" ] && echo "✅" || echo "❌")"
echo "   linux/cjxl:   $([ -f "$BIN_DIR/linux/cjxl" ] && echo "✅" || echo "❌")"
echo ""
echo "Done!"
