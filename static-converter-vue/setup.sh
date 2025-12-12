#!/opt/homebrew/bin/bash
# Setup script for JPEG to JXL Eagle Plugin
# This script helps download and setup cjxl binaries for all platforms

set -e

PLUGIN_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DIR="$PLUGIN_DIR/bin"

LIBJXL_VERSION="v0.10.2"  # Update this to the latest version
RELEASE_URL="https://github.com/libjxl/libjxl/releases/download"

echo "🚀 JPEG to JXL Plugin Setup"
echo "============================"
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Detect current platform
detect_platform() {
    case "$OSTYPE" in
        darwin*)  echo "darwin" ;;
        linux*)   echo "linux" ;;
        msys*|cygwin*) echo "win32" ;;
        *)        echo "unknown" ;;
    esac
}

PLATFORM=$(detect_platform)
echo -e "${BLUE}Detected platform: ${PLATFORM}${NC}"
echo ""

# Check existing binaries
check_binaries() {
    echo "📦 Checking existing binaries..."
    
    # Darwin
    if [ -f "$BIN_DIR/darwin/cjxl" ]; then
        echo -e "${GREEN}✓${NC} macOS (darwin): cjxl found"
    else
        echo -e "${RED}✗${NC} macOS (darwin): cjxl missing"
    fi
    
    # Windows
    if [ -f "$BIN_DIR/win32/cjxl.exe" ]; then
        echo -e "${GREEN}✓${NC} Windows (win32): cjxl.exe found"
    else
        echo -e "${RED}✗${NC} Windows (win32): cjxl.exe missing"
    fi
    
    # Linux
    if [ -f "$BIN_DIR/linux/cjxl" ]; then
        echo -e "${GREEN}✓${NC} Linux: cjxl found"
        LINUX_BINARY_EXISTS=true
    else
        echo -e "${RED}✗${NC} Linux: cjxl missing"
        LINUX_BINARY_EXISTS=false
    fi
    
    echo ""
}

# Download Linux binary
download_linux_binary() {
    echo -e "${YELLOW}📥 Downloading cjxl for Linux...${NC}"
    
    TEMP_DIR=$(mktemp -d)
    DOWNLOAD_URL="${RELEASE_URL}/${LIBJXL_VERSION}/jxl-linux-x86_64-static-${LIBJXL_VERSION}.tar.gz"
    
    echo "Download URL: $DOWNLOAD_URL"
    echo "Temp directory: $TEMP_DIR"
    
    # Download
    if command -v curl &> /dev/null; then
        curl -L -o "$TEMP_DIR/libjxl.tar.gz" "$DOWNLOAD_URL"
    elif command -v wget &> /dev/null; then
        wget -O "$TEMP_DIR/libjxl.tar.gz" "$DOWNLOAD_URL"
    else
        echo -e "${RED}Error: Neither curl nor wget found. Please install one of them.${NC}"
        exit 1
    fi
    
    # Extract
    echo "Extracting..."
    tar -xzf "$TEMP_DIR/libjxl.tar.gz" -C "$TEMP_DIR"
    
    # Find cjxl binary
    CJXL_BINARY=$(find "$TEMP_DIR" -name "cjxl" -type f | head -n 1)
    
    if [ -z "$CJXL_BINARY" ]; then
        echo -e "${RED}Error: cjxl binary not found in the downloaded archive${NC}"
        rm -rf "$TEMP_DIR"
        exit 1
    fi
    
    # Create linux bin directory if not exists
    mkdir -p "$BIN_DIR/linux"
    
    # Copy binary
    cp "$CJXL_BINARY" "$BIN_DIR/linux/cjxl"
    chmod +x "$BIN_DIR/linux/cjxl"
    
    # Cleanup
    rm -rf "$TEMP_DIR"
    
    echo -e "${GREEN}✓ Linux binary downloaded and installed${NC}"
    echo ""
    echo -e "${GREEN}✓ Linux binary downloaded and installed${NC}"
    echo ""
}

# Download macOS binary
download_macos_binary() {
    echo -e "${YELLOW}📥 Downloading cjxl for macOS...${NC}"
    
    TEMP_DIR=$(mktemp -d)
    # Note: Using static build if available, otherwise standard build
    DOWNLOAD_URL="${RELEASE_URL}/${LIBJXL_VERSION}/jxl-darwin-x86_64-${LIBJXL_VERSION}.tar.gz"
    
    echo "Download URL: $DOWNLOAD_URL"
    
    if command -v curl &> /dev/null; then
        curl -L -o "$TEMP_DIR/libjxl.tar.gz" "$DOWNLOAD_URL"
    else
        wget -O "$TEMP_DIR/libjxl.tar.gz" "$DOWNLOAD_URL"
    fi
    
    echo "Extracting..."
    tar -xzf "$TEMP_DIR/libjxl.tar.gz" -C "$TEMP_DIR"
    
    CJXL_BINARY=$(find "$TEMP_DIR" -name "cjxl" -type f | head -n 1)
    
    if [ -z "$CJXL_BINARY" ]; then
        echo -e "${RED}Error: cjxl binary not found${NC}"
        rm -rf "$TEMP_DIR"
        exit 1
    fi
    
    mkdir -p "$BIN_DIR/darwin"
    cp "$CJXL_BINARY" "$BIN_DIR/darwin/cjxl"
    chmod +x "$BIN_DIR/darwin/cjxl"
    
    # Also copy libs if present
    LIB_DIR=$(find "$TEMP_DIR" -name "lib" -type d | head -n 1)
    if [ -n "$LIB_DIR" ]; then
        mkdir -p "$BIN_DIR/lib"
        cp -r "$LIB_DIR/"* "$BIN_DIR/lib/"
        echo "Copied libraries to bin/lib"
    fi
    
    rm -rf "$TEMP_DIR"
    echo -e "${GREEN}✓ macOS binary installed${NC}"
    echo ""
}

# Copy from system installation
copy_from_system() {
    echo -e "${YELLOW}📥 Looking for system-installed cjxl...${NC}"
    
    if command -v cjxl &> /dev/null; then
        SYSTEM_CJXL=$(command -v cjxl)
        echo "Found cjxl at: $SYSTEM_CJXL"
        
        mkdir -p "$BIN_DIR/linux"
        cp "$SYSTEM_CJXL" "$BIN_DIR/linux/cjxl"
        chmod +x "$BIN_DIR/linux/cjxl"
        
        echo -e "${GREEN}✓ Copied from system installation${NC}"
        echo ""
        return 0
    else
        echo -e "${YELLOW}cjxl not found in system PATH${NC}"
        echo ""
        return 1
    fi
}

# Main setup
main() {
    check_binaries
    
    # Check if we need to setup binaries
    NEEDS_SETUP=false
    if [ "$PLATFORM" = "linux" ] && [ "$LINUX_BINARY_EXISTS" = false ]; then NEEDS_SETUP=true; fi
    if [ "$PLATFORM" = "darwin" ]; then
        # Always offer to reinstall on macOS to fix lib issues
        NEEDS_SETUP=true
    fi

    if [ "$NEEDS_SETUP" = true ]; then
        echo -e "${BLUE}Binary setup required or recommended. Would you like to:${NC}"
        echo "1) Download from GitHub releases (recommended)"
        echo "2) Copy from system installation"
        echo "3) Skip"
        echo ""
        
        read -p "Enter your choice (1-3): " choice
        
        case $choice in
            1)
                if [ "$PLATFORM" = "linux" ]; then download_linux_binary; fi
                if [ "$PLATFORM" = "darwin" ]; then download_macos_binary; fi
                ;;
            2)
                if ! copy_from_system; then
                    echo "System binary not found."
                fi
                ;;
            3)
                echo "Skipped."
                ;;
        esac
    else
        echo -e "${GREEN}All required binaries are present!${NC}"
    fi
    
    echo ""
    echo -e "${GREEN}✅ Setup complete!${NC}"
    echo ""
    echo "Binary locations:"
    echo "  macOS:   $BIN_DIR/darwin/cjxl"
    echo "  Windows: $BIN_DIR/win32/cjxl.exe"
    echo "  Linux:   $BIN_DIR/linux/cjxl"
    echo ""
    echo "Next steps:"
    echo "1. Import this plugin folder to Eagle"
    echo "2. Select some JPEG images in Eagle"
    echo "3. Open the 'JPEG to JXL Converter' plugin"
    echo "4. Click 'Convert to JXL' button"
    echo ""
}

main
