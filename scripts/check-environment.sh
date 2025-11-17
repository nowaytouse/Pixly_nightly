#!/bin/bash

# 🔍 PIXLY Environment Check Script
# Purpose: Verify all required tools and dependencies

set -e

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 项目路径
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${BLUE}╔══════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  🔍 PIXLY Environment Check & Dependency Installer  ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════╝${NC}"
echo ""

# 统计
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
MISSING_TOOLS=()

# 检查函数
check_command() {
    local cmd=$1
    local name=$2
    local install_hint=$3
    
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    
    if command -v $cmd &> /dev/null; then
        local version=$($cmd --version 2>&1 | head -1 || echo "unknown")
        echo -e "${GREEN}✅ $name${NC}"
        echo -e "   ${BLUE}→${NC} $version"
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
        return 0
    else
        echo -e "${RED}❌ $name${NC}"
        echo -e "   ${YELLOW}Install: $install_hint${NC}"
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
        MISSING_TOOLS+=("$name|$install_hint")
        return 1
    fi
}

check_rust_cli() {
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    local rust_cli="$PROJECT_ROOT/core/rust/target/release/pixly-rust"
    
    if [ -f "$rust_cli" ]; then
        local version=$($rust_cli --version 2>&1 || echo "unknown")
        echo -e "${GREEN}✅ PIXLY Rust CLI${NC}"
        echo -e "   ${BLUE}→${NC} $version"
        echo -e "   ${BLUE}→${NC} $rust_cli"
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
        return 0
    else
        echo -e "${RED}❌ PIXLY Rust CLI${NC}"
        echo -e "   ${YELLOW}Build: cd core/rust && cargo build --release${NC}"
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
        MISSING_TOOLS+=("PIXLY Rust CLI|cd core/rust && cargo build --release")
        return 1
    fi
}

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📦 Core Development Tools${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

check_command "go" "GO (AI Service)" "brew install go"
check_command "cargo" "Rust (CLI Tools)" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
check_command "node" "Node.js (Plugin Dev)" "brew install node"
check_rust_cli

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🖼️  Image Processing Tools${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

check_command "cjxl" "JPEG XL Encoder" "brew install jpeg-xl"
check_command "djxl" "JPEG XL Decoder" "brew install jpeg-xl"
check_command "avifenc" "AVIF Encoder" "brew install libavif"
check_command "avifdec" "AVIF Decoder" "brew install libavif"
check_command "cwebp" "WebP Encoder" "brew install webp"
check_command "dwebp" "WebP Decoder" "brew install webp"
check_command "exiftool" "ExifTool (Metadata)" "brew install exiftool"
check_command "heif-convert" "HEIF Converter" "brew install libheif"

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🎬 Video Processing Tools${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

check_command "ffmpeg" "FFmpeg (Video)" "brew install ffmpeg"
check_command "ffprobe" "FFprobe (Analysis)" "brew install ffmpeg"

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🔧 Optional Tools${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

check_command "python3" "Python 3" "brew install python3"
check_command "git" "Git" "brew install git"
check_command "curl" "cURL" "brew install curl"

echo ""
echo -e "${BLUE}╔══════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                  📊 Summary Report                   ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "Total Checks:  ${BLUE}$TOTAL_CHECKS${NC}"
echo -e "✅ Passed:     ${GREEN}$PASSED_CHECKS${NC}"
echo -e "❌ Failed:     ${RED}$FAILED_CHECKS${NC}"
echo ""

if [ $FAILED_CHECKS -eq 0 ]; then
    echo -e "${GREEN}╔══════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  🎉 All dependencies are installed! You're ready!  ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${BLUE}Next steps:${NC}"
    echo -e "  1. Start AI service:  ${YELLOW}./start-ai-service.sh${NC}"
    echo -e "  2. Open Eagle and use PIXLY plugin"
    echo ""
    exit 0
else
    echo -e "${RED}╔══════════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║   ⚠️  Some dependencies are missing                 ║${NC}"
    echo -e "${RED}╚══════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${YELLOW}Missing tools:${NC}"
    for tool in "${MISSING_TOOLS[@]}"; do
        IFS='|' read -r name install_cmd <<< "$tool"
        echo -e "  ${RED}•${NC} $name"
        echo -e "    ${BLUE}→${NC} $install_cmd"
    done
    echo ""
    echo -e "${YELLOW}Quick Install (Homebrew):${NC}"
    echo -e "  Run the following command to install all missing tools:"
    echo ""
    echo -e "  ${GREEN}brew install jpeg-xl libavif webp exiftool ffmpeg libheif${NC}"
    echo ""
    echo -e "${YELLOW}Or use the automatic installer:${NC}"
    echo -e "  ${GREEN}./install-dependencies.sh${NC}"
    echo ""
    exit 1
fi
